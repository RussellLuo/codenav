from pathlib import Path
EXAMPLES_DIR = f'{Path(__file__).parent.parent.parent}/examples/javascript'

import codenav

nav = codenav.Navigator(codenav.Language.JavaScript, "./test.sqlite")
nav.index([EXAMPLES_DIR])

snippet = codenav.Snippet(codenav.Language.JavaScript, f'{EXAMPLES_DIR}/kitchen.js', 2, 5)

for reference in snippet.references():
    definitions = nav.resolve(reference)
    dependencies = [d for d in definitions if not snippet.contains(d)]
    if not dependencies:
        continue

    msg = f'Resolving {reference.path}:{reference.line}:{reference.column} "{reference.text}"'
    print('=' * len(msg))
    print(msg)

    for d in dependencies:
        print(f'{d.path}:{d.span.start.line}:{d.span.start.column}')
        print(d.text())

nav.clean(True)
