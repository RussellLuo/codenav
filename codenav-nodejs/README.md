# CodeNav Node.js Bindings

Node.js bindings for CodeNav.


## Installation

```bash
npm install @codenav/codenav
```


## Quick Start

```javascript
import * as codenav from '@codenav/codenav';
const nav = new codenav.Navigator(codenav.Language.Python, 'test.sqlite', false);
nav.index(['<YOUR_LOCAL_PATH>/codenav/examples/python'], false);
const reference = {path: '<YOUR_LOCAL_PATH>/codenav/examples/python/chef.py', line: 2, column: 0, text: 'broil'};
const definitions = nav.resolve(reference);
for (let d of definitions) {
  console.log(`${d.path}:${d.span.start.line}:${d.span.start.column}`);
  console.log(d.text(codenav.TextMode.Complete));
}
nav.clean(true);
```


## Examples

- [Resolve a Python reference](examples/resolve_python_reference.js)
- [Resolve a Python snippet](examples/resolve_python_snippet.js)
- [Resolve a JavaScript reference](examples/resolve_javascript_reference.js)
- [Resolve a JavaScript snippet](examples/resolve_javascript_snippet.js)
- [Resolve a TypeScript reference](examples/resolve_typescript_reference.js)
- [Resolve a TypeScript snippet](examples/resolve_typescript_snippet.js)


## Development

Install [napi-rs][1]:

```bash
npm install -g @napi-rs/cli
```

Build the native package:

```bash
napi build --platform --release
```

Run the examples:

```bash
node examples/resolve_python_reference.js
node examples/resolve_python_snippet.js
node examples/resolve_javascript_reference.js
node examples/resolve_javascript_snippet.js
node examples/resolve_typescript_reference.js
node examples/resolve_typescript_snippet.js
```


[1]: https://napi.rs/