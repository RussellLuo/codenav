from pathlib import Path
EXAMPLES_DIR = f'{Path(__file__).parent.parent.parent}/examples'

import codenav
nav = codenav.Navigator("./test.sqlite")

# nav.clean()
nav.index([EXAMPLES_DIR])

snippet = codenav.Snippet(f'{EXAMPLES_DIR}/chef.py', 2, 2)

for reference in snippet.references():
    definitions = nav.resolve(reference)
    dependencies = [d for d in definitions if not snippet.contains(d)]
    if not dependencies:
        continue

    msg = f'Resolving {reference.path}:{reference.line}:{reference.column} "{reference.text}"'
    print('=' * len(msg))
    print(msg)

    for i, d in enumerate(dependencies):
        print(f'Found {i}:\n{d.path}:{d.span.start.line}:{d.span.start.column}')
        print(d.text())
