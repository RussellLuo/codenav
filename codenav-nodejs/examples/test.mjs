import path from 'path';
const EXAMPLES_DIR = `${path.dirname(path.dirname(import.meta.dirname))}/examples`;

import { Navigator, Snippet, TextMode } from '../index.js';
let nav = new Navigator('./test.sqlite');

// nav.clean(false);
nav.index([EXAMPLES_DIR], false);

let snippet = new Snippet(`${EXAMPLES_DIR}/chef.py`, 2, 2);

for (let reference of snippet.references('')) {
    let definitions = nav.resolve(reference);
    let dependencies = definitions.filter(d => !snippet.contains(d));
    if (dependencies.length == 0) continue;

    const msg = `Resolving ${reference.path}:${reference.line}:${reference.column} "${reference.text}"`;
    console.log('='.repeat(msg.length));
    console.log(msg);

    for (let [i, d] of dependencies.entries()) {
        console.log(`Found ${i}:\n${d.path}:${d.span.start.line}:${d.span.start.column}`);
        console.log(d.text(TextMode.Complete));
    }
}
