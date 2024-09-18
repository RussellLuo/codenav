# CodeNav Node.js Bindings

Node.js bindings for CodeNav.


## Installation

```bash
npm install @codenav/codenav
```


## Quick Start

```javascript
import * as codenav from '@codenav/codenav';
// Create a navigator with a test database.
const nav = new codenav.Navigator('./test.sqlite');
// Index the examples folder (force a re-index).
nav.index(['/<YOUR_LOCAL_PATH>/codenav/examples'], true);
// Construct a reference "broil".
const reference = {path: '/<YOUR_LOCAL_PATH>/codenav/examples/chef.py', line: 2, column: 4, text: 'broil'};
// Find the definition(s) for "broil".
const definitions = nav.resolve(reference);
for (let d of definitions) {
    console.log(`${d.path}:${d.span.start.line}:${d.span.start.column}`);
    console.log(d.text(codenav.TextMode.Complete));
}
// Clean the test database (also delete the database).
nav.clean(true);
```


## Development

Install [napi-rs][1]:

```bash
npm install -g @napi-rs/cli
```

Build the native package:

```bash
napi build --platform --release
```

Run the test script:

```bash
node examples/test.mjs
```


[1]: https://napi.rs/