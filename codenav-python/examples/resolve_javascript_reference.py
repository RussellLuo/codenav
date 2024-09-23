from pathlib import Path
EXAMPLES_DIR = f'{Path(__file__).parent.parent.parent}/examples/javascript'

import codenav

nav = codenav.Navigator(codenav.Language.JavaScript, './test.sqlite')
nav.index([EXAMPLES_DIR])

reference = codenav.Reference(f'{EXAMPLES_DIR}/chef.js', 2, 0, 'broil')

msg = f'Resolving {reference.path}:{reference.line}:{reference.column} "{reference.text}"'
print('=' * len(msg))
print(msg)

definitions = nav.resolve(reference)
for d in definitions:
    print(f'{d.path}:{d.span.start.line}:{d.span.start.column}')
    print(d.text())

nav.clean(True)
