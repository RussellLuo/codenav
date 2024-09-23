import test from 'ava';

import * as path from 'path';
const EXAMPLES_DIR = path.join(path.dirname(path.dirname(import.meta.dirname)), 'examples', 'python');

import * as codenav from '../index.js';
const nav = new codenav.Navigator(codenav.Language.Python, 'test.sqlite', false);

test('resloving references', (t) => {
  nav.index([EXAMPLES_DIR], false);

  const reference = {path: path.join(EXAMPLES_DIR, 'chef.py'), line: 2, column: 0, text: 'broil'};
  const definitions = nav.resolve(reference);

  let defs: string[] = [];
  for (let d of definitions) {
    const filename = path.basename(d.path);
    defs.push(`${filename}:${d.span.start.line}:${d.span.start.column}`);
  }

  nav.clean(true);

  t.deepEqual(defs, ['chef.py:0:20', 'kitchen.py:2:4'], 'unexpected definitions');
})
