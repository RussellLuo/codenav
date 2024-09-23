import * as path from 'path';
const EXAMPLES_DIR = path.join(path.dirname(path.dirname(import.meta.dirname)), 'examples', 'javascript');

import * as codenav from '../index.js';

const nav = new codenav.Navigator(codenav.Language.JavaScript, 'test.sqlite', false);
nav.index([EXAMPLES_DIR], false);

const snippet = new codenav.Snippet(codenav.Language.JavaScript, path.join(EXAMPLES_DIR, 'kitchen.js'), 2, 5);

for (let reference of snippet.references('')) {
  const definitions = nav.resolve(reference);
  const dependencies = definitions.filter(d => !snippet.contains(d));
  if (dependencies.length == 0) continue;

  const msg = `Resolving ${reference.path}:${reference.line}:${reference.column} "${reference.text}"`;
  console.log('='.repeat(msg.length));
  console.log(msg);

  for (let d of dependencies) {
    console.log(`${d.path}:${d.span.start.line}:${d.span.start.column}`);
    console.log(d.text(codenav.TextMode.Complete));
  }
}

nav.clean(true);
