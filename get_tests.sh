#!/usr/bin/env bash

curl https://raw.githubusercontent.com/npm/npm/master/package.json -o tests/npm.json
curl https://raw.githubusercontent.com/yarnpkg/yarn/master/package.json -o tests/yarn.json
curl https://raw.githubusercontent.com/sinonjs/sinon/master/package.json -o tests/sinon.json
curl https://raw.githubusercontent.com/chaijs/chai/master/package.json -o tests/chai.json
curl https://raw.githubusercontent.com/expressjs/express/master/package.json -o tests/express.json
curl https://raw.githubusercontent.com/lodash/lodash/master/package.json -o tests/lodash.json
