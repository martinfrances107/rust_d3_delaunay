// import { Renderer } from 'benchmark';

const sizeRange = document.getElementById('size-range');
const sizeLabel = document.getElementById('size-label');
const perf = document.getElementById('perf');
const canvas = document.getElementById('c');

import('../pkg')
  .then(pkg => {
    console.log('wasm is imported');

    pkg.main();
  })