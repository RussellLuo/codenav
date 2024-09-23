import * as path from 'path';
const EXAMPLES_DIR = path.join(path.dirname(path.dirname(import.meta.dirname)), 'examples', 'python');

import * as codenav from '../index.js';

const nav = new codenav.Navigator(codenav.Language.Python, 'test.sqlite', false);
nav.index([EXAMPLES_DIR], false);

const reference = {path: path.join(EXAMPLES_DIR, 'chef.py'), line: 2, column: 0, text: 'broil'};

const msg = `Resolving ${reference.path}:${reference.line}:${reference.column} "${reference.text}"`;
console.log('='.repeat(msg.length));
console.log(msg);

const definitions = nav.resolve(reference);
for (let d of definitions) {
  console.log(`${d.path}:${d.span.start.line}:${d.span.start.column}`);
  console.log(d.text(codenav.TextMode.Complete));
}

nav.clean(true);
