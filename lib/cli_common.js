// Copyright 2011 Joyent, Inc.  All rights reserved.

var assert = require('assert');
var fs = require('fs');
var path = require('path');
var url = require('url');

var nopt = require('nopt');
var restify = require('restify');

var CloudAPI = require('../lib/index').CloudAPI;


path.name = 'path';
url.name = 'url';



///--- Internal Functions

function usage(str, code, message) {
  assert.ok(str);

  var writer = console.log;
  if (code)
    writer = console.error;

  if (message)
    writer(message);
  writer(path.basename(process.argv[1]) + ' ' + str);
  process.exit(code || 0);
}


function buildUsageString(options) {
  assert.ok(options);

  var str = '';
  for (var k in options) {
    if (options.hasOwnProperty(k)) {
      var o = options[k].name ? options[k].name.toLowerCase() : '';
      str += '[--' + k + ' ' + o + '] ';
    }
  }
  return str;
}


function loadSigningKey(parsed) {
  assert.ok(parsed);

  try {
    parsed.signingKey = fs.readFileSync(parsed.identity, 'ascii');
    assert.ok(parsed.signingKey);
    return parsed;
  } catch (e) {
    console.error(e.message);
    process.exit(2);
  }
}



///--- Exported API

module.exports = {

  /**
   * Common callback for all CLI operations.
   *
   * @param {Error} err optional error object.
   * @param {Object} obj optional response object.
   */
  callback: function(err, obj) {
    if (err) {
      console.error(err.message);
      process.exit(3);
    }

    if (obj)
      console.log(JSON.stringify(obj, null, 2));
  },


  usage: usage,


  buildUsageString: buildUsageString,


  parseArguments: function(options, shortOptions, usageStr) {
    assert.ok(options);
    assert.ok(shortOptions);

    if (!usageStr)
      usageStr = buildUsageString(options);

    var parsed = nopt(options, shortOptions, process.argv, 2);
    if (parsed.help)
      usage(usageStr);

    if (parsed.debug)
      restify.log.level(restify.LogLevel.Trace);

    if (!parsed.keyId) {
      if (process.env.SDC_CLI_KEY_ID) {
        parsed.keyId = process.env.SDC_CLI_KEY_ID;
      } else {
        usage(usageStr, 1, 'keyId is required');
      }
    }

    if (!parsed.identity)
      parsed.identity = process.env.HOME + '/.ssh/id_rsa';

    if (!parsed.account)
      parsed.account = process.env.SDC_CLI_ACCOUNT || process.env.USER;

    if (!parsed.url)
      parsed.url = process.env.SDC_CLI_URL || 'https://10.99.99.15';

    return loadSigningKey(parsed);
  },


  newClient: function(parsed) {
    assert.ok(parsed);
    assert.ok(parsed.keyId);
    assert.ok(parsed.signingKey);

    try {
      return new CloudAPI({
        url: parsed.url,
        account: parsed.account,
        noCache: true,
        logLevel: restify.log.level(),
        key: parsed.signingKey,
        keyId: '/' + parsed.account + '/keys/' + parsed.keyId
      });
    } catch (e) {
      console.error(e.message);
      process.exit(1);
    }
  },


  loadKey: function(key) {
    try {
      return fs.readFileSync(key, 'ascii');
    } catch (e) {
      console.error('Unable to load key ' + key + ': ' + e);
      process.exit(2);
    }
  }

};
