# Node.js clients for Smart DataCenter APIs

Where? <git@git.joyent.com:node-sdc-clients.git>
Who? Mark Cavage.
What APIs? Currently, most of the SDC APIs.


# Example?

    var clients = require('sdc-clients');

    var CAPI = new clients.CAPI({
        url: "http://10.99.99.11",
        username: "admin",
        password: "admin's password"
    });
    CAPI.authenticate(username, password, function(err, customer) {});

    var CA = new clients.CA({url: "..."});
    CA.listSchema(customer, function(err, schema) {});


# Docs?

See the inline jsdoc. We should generate HTML docs from those. Have any
suggestions for that?


# Versioning & Changelog

The version is the "version" field in package.json. Please follow these rules:

- bug fix: increment the patch level
- feature addition: increment the minor level
- major architectural change or backward incompat: increment the major level

A changelog is in CHANGES.md. Please add a note for each "interesting" change you make.


# Tests

    make test
