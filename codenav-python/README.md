# CodeNav Python Bindings

Python bindings for CodeNav.


## Installation

```bash
pip install codenav-python
```


## Quick Start

```python
import codenav
nav = codenav.Navigator(codenav.Language.Python, './test.sqlite')
nav.index(['<YOUR_LOCAL_PATH>/codenav/examples/python'])
reference = codenav.Reference('<YOUR_LOCAL_PATH>/codenav/examples/python/chef.py', 2, 0, 'broil')
definitions = nav.resolve(reference)
for d in definitions:
    print(f'{d.path}:{d.span.start.line}:{d.span.start.column}')
    print(d.text())
nav.clean(True)
```


## Examples

- [Resolve a Python reference](examples/resolve_python_reference.py)
- [Resolve a Python snippet](examples/resolve_python_snippet.py)
- [Resolve a JavaScript reference](examples/resolve_javascript_reference.py)
- [Resolve a JavaScript snippet](examples/resolve_javascript_snippet.py)
- [Resolve a TypeScript reference](examples/resolve_typescript_reference.py)
- [Resolve a TypeScript snippet](examples/resolve_typescript_snippet.py)


## Development

Install [maturin][1]:

```bash
pip install maturin
```

Build and install the module:

```bash
maturin develop
```

Run the examples:

```bash
python examples/resolve_python_reference.py
python examples/resolve_python_snippet.py
python examples/resolve_javascript_reference.py
python examples/resolve_javascript_snippet.py
python examples/resolve_typescript_reference.py
python examples/resolve_typescript_snippet.py
```


[1]: https://www.maturin.rs/