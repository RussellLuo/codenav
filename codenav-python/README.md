# CodeNav Python Bindings

Python bindings for CodeNav.


## Installation

```bash
pip install codenav-python
```


## Quick Start

```python
import codenav
# Create a navigator with a test database.
nav = codenav.Navigator('./test.sqlite')
# Index the examples folder.
nav.index(['/<YOUR_LOCAL_PATH>/codenav/examples'])
# Construct a reference "broil".
reference = codenav.Reference('/<YOUR_LOCAL_PATH>/codenav/examples/chef.py', 2, 4, 'broil')
# Find the definition(s) for "broil".
definitions = nav.resolve(reference)
for d in definitions:
    print(f'{d.path}:{d.span.start.line}:{d.span.start.column}')
    print(d.text())
# Clean the test database.
nav.clean()
```


## Development

Install [maturin][1]:

```bash
pip install maturin
```

Build and install the module:

```bash
maturin develop
```

Run the test script:

```bash
python examples/test.py
```


[1]: https://www.maturin.rs/