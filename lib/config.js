/* Copyright 2012 Joyent, Inc. All rights reserved. */

var EventEmitter = require('events').EventEmitter;
var ldap = require('ldapjs');
var url = require('url');
var path = require('path');
var util = require('util');
var fs = require('fs');
/**
 * Constructor
 *
 * @Param {Object} options the config options object:
 *                         - svc {String} service name
 *                         - ufdsargs {Object} UFDS config options object
 *                         - dc {String} the datacenter name, if used for scoping
 *                         - host {String} the host name, if used for scoping
 *                         - nofollowref {Boolean} do not follow references
 */

function Config(options) {
  var svc = options.svc;
  var nofollowref = options.nofollowref || false; 

  var self = this;

  EventEmitter.call(this);
  
  self._dn = "svc=" + svc + ",ou=config,o=smartdc";

  self._ldapcfg = {canonical: {}, file: {}};

  self._backingfile = options.backingfile;
  if (self._backingfile) {
    var file = JSON.parse(fs.readFileSync(self._backingfile));
    self._makeconfig(file, "file");
  }
    
  if (options.ufds && !options.ufds.maxConnections)
    options.ufds.maxConnections = 5;

  if ( options.ufds )
    self._ufdsargs = options.ufds;
  
  if(options.pollinterval)
    self._pollinterval = options.pollinterval;
  else
    self._pollinterval = 5000;

  self._scopedc = options.dc;
  self._scopehost = options.host;
  self._lastchange = 0;

  this._client = ldap.createClient(self._ufdsargs);
  this._client.bind(self._ufdsargs.bindDN, self._ufdsargs.bindPassword, function(err) {

  self._client.search(self._dn, {scope: "sub"}, function (err, res) {

      res.on("error", function(err) { console.log(err.message); });

      res.on("searchEntry", function(entry) {
        var cfgresult = self._getscope(entry.objectName);

        if (entry.object.objectclass == "config")
          self._makeconfig(entry, cfgresult);
        else if ( entry.object.objectclass == "referral" && ! nofollowref )
          self._makeref(entry,cfgresult);
      }); // on.("searchentry")
    }); // self._client.search()


    var makechanges = false;

    setInterval(function() {
      self._client.search('cn=changelog',
        {scope: "sub", filter: "changenumber>=" + self._lastchange.toString() },
        function (err, res) {
      res.on("error", function(err) { console.log( " error " + err); });
      res.on("searchEntry", function(entry) {

        var targetdn = ldap.parseDN(entry.object.targetdn);
        // need to deal with the first return
        if (targetdn.childOf(self._dn) || targetdn.equals(self._dn) && makechanges) {
          var targetscope = self._getscope(entry.object.targetdn);
          var changes = JSON.parse(entry.object.changes);
          if (changes.length)
            changes.forEach(function(change) {
              var obj = {};
              obj[change.modification.type] = change.modification.vals[0];
              self._makeconfig(obj, targetscope);
            });
        }
        self._lastchange = Number(entry.object.changenumber) + 1;
      });
      makechanges = true;
    });}, self._pollinterval);

  }); // self._client.bind()
}
util.inherits(Config, EventEmitter);
module.exports = Config;

/*
 * Return the scope of an LDAP dn
 */
Config.prototype._getscope = function(objectname) {
  var self = this;
  var cfgresult = "canonical";
  var dn = ldap.parseDN(objectname);
  var dndc = undefined;
  var dnhost = undefined;

  // determine the scope of the search result
  for (var i = 0 ; i < dn.length; i++) {
    dndc = dn.rdns[i].dc || dndc;
    dnhost = dn.rdns[i].host || dnhost;
  }
  if (dndc) {
    if(!self._ldapcfg[dndc]) self._ldapcfg[dndc] = {};
   cfgresult = dndc;
  } else if (dnhost) {
    if (!self._ldapcfg[dnhost]) self._ldapcfg[dnhost] = {};
    cfgresult = dnhost;
  }
  return cfgresult;
};

/*
 * Add a config pointer to another service
 */
Config.prototype._makeref = function(entry, cfgresult) {
  var self = this;
  var refsvc = "";
  var ref = url.parse(entry.object.ref);
  var refdnElem = ldap.parseDN(path.basename(ref.path));
  var refcfgargs;

  // we are only concerned with referrals that point at svc=...
  for (var i = 0; i < refdnElem.length; ++i) {
    if (refdnElem[i]['svc'])
      refsvc = refdnElem[i]['svc'];
  }
  if (ref.hostname == undefined) {
    if (refsvc) {
      refcfgargs = {
        svc: refsvc,
        ufdsargs: self._ufdsargs,
        dc: self._scopedc,
        host: self._scopehost,
        nofollowref: true
      };
      cfgresult[refsvc] = new Config(refcfgargs);
      cfgresult[refsvc].on('update', function() { self.emit('update'); });
    }
  } else {
    var connectargs = { url: ref.protocol + "//" + ref.host };

    if ( entry.object.bindDN != undefined )
      connectargs.bindDN = entry.object.bindDN;
    else
      connectargs.bindDN = self._ufdsargs.bindDN;
        
    if ( entry.object.bindPassword != undefined )
      connectargs.bindPassword = entry.object.bindPassword;
    else
      connectargs.bindPassword = self._ufdsargs.bindPassword;

    if (refsvc) {
      try {
        refcfgargs = {
          svc: refsvc,
          ufdsargs: connectargs,
          dc: self._scopedc,
          host: self._scopehost,
          nofollowref: true
        };
        cfgresult[refsvc] = new Config(refcfgargs);
        cfgresult[refsvc].on('update', function() { self.emit('update'); });
      } catch (err) {
        console.log(JSON.stringify(err));
      }
    } // if (refsvc)
  }
  self._updatekeys();
};

/*
 * Mutate self based on an ldap config entry
 */

Config.prototype._makeconfig = function(entry, cfgresult) {
  var self = this;
  var obj = entry.object || entry;
  var entries = Object.keys(obj);
  var i = 0;

  var dst = self._ldapcfg[cfgresult];

  if(entry.objectName) {
    var dn = ldap.parseDN(entry.objectName);
    for(i = dn.length - 1; i >= 0; i--) {
      var sub = dn.rdns[i].cfg;
      if (sub && !dst[sub])
        dst[sub] = {};
      if (sub)
        dst = dst[sub];
    }
  }

  for (i = 0; i < entries.length; ++i ) {
    // remove the internal/private variables ( starting with '_' )
    if (!/^_/.test(entries[i])) {
      //self._ldapcfg[cfgresult][entries[i]] = obj[entries[i]];
      dst[entries[i]] = obj[entries[i]];
    }
  }
  self._updatekeys();
};

/*
 * update the Config object's getter keys
 */
Config.prototype._updatekeys = function() {
  
  var self = this;
  //  list of keys to update
  var keys = Object.keys(self._ldapcfg.canonical);

  if(self._ldapcfg[self._scopedc])
    keys  = keys.concat(Object.keys(self._ldapcfg[self._scopedc]));

  if(self._ldapcfg[self._scopehost])
    keys = keys.concat(Object.keys(self._ldapcfg[self._scopehost]));
  if(self._ldapcfg.file)
    keys = keys.concat(Object.keys(self._ldapcfg.file));

  var len = keys.length;

  // make config.var work
  keys.forEach( function(val) {
    self.__defineGetter__(val, function() {
      // using host scope
      if (self._ldapcfg[self._scopehost] && self._ldapcfg[self._scopehost][val])
        return (self._ldapcfg[self._scopehost][val]);
      // using DC scope
      if (self._ldapcfg[self._scopedc] && self._ldapcfg[self._scopedc][val])
        return (self._ldapcfg[self._scopedc][val]);
      // fall through to canonical scope          
      if (self._ldapcfg.canonical[val])
        return self._ldapcfg.canonical[val];          
      // finally fall through to file scope          
      return self._ldapcfg.file[val];          
    });

    // after updates, fire the 'update' emitter
    if ( --len <= 0 )
      self.emit('update');
  });
};

/**
 * Insert a config variable to the configuration pointed at by this object,
 * as well as LDAP
 *
 * @Param {Object} cfg The key:value pairs to add to the config
 */
Config.prototype.insert = function(cfg) {
  if (typeof(cfg) != 'object') {
    throw new TypeError('Can only insert objects');
  }
  var self = this;

  var changes = [];
  var len = Object.keys(cfg).length;

  Object.keys(cfg).forEach( function(key) {
    var ldapconfig = {};
    var mod = {};
    mod[key] = cfg[key];

    if (typeof(cfg[key]) != 'string')
      throw new TypeError('Can only insert string:string pairs');

    if (self._ldapcfg[key] == 'undefined')
      ldapconfig.operation = 'add';
    else
      ldapconfig.operation = 'replace';

    ldapconfig.modification = mod;

    var change = new ldap.Change(ldapconfig);
    changes.push(change);

    len--;

    if ( len <= 0 )
      self._client.modify(self.dn, changes, function(err) {
        // not really sure what the "right" thing to do is here
        if (err && err != 'null')
          throw new Error("Error inserting changes: " + err); 
      });
  }); // keys(cfg).forEach
}; // insert