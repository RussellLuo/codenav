import test from 'ava';

import * as path from 'path';
const EXAMPLES_DIR = path.join(path.dirname(path.dirname(import.meta.dirname)), 'examples');

import { Navigator, Snippet } from '../index';
let nav = new Navigator('test.sqlite');

test('resloving references', (t) => {
  nav.index([EXAMPLES_DIR], true);
  
  let snippet = new Snippet(path.join(EXAMPLES_DIR, 'chef.py'), 2, 2);
  
  let defs: string[] = [];
  for (let reference of snippet.references('')) {
      let definitions = nav.resolve(reference);
      let dependencies = definitions.filter(d => !snippet.contains(d));
      if (dependencies.length == 0) continue;
  
      for (let [_, d] of dependencies.entries()) {
          const filename = path.basename(d.path);
          defs.push(`${filename}:${d.span.start.line}:${d.span.start.column}`);
      }
  }
  
  nav.clean(true);

  t.deepEqual(defs, ['chef.py:0:20', 'kitchen.py:2:4', 'stove.py:3:4'], 'unexpected definitions');
})
