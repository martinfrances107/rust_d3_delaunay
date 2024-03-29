// import { Renderer } from 'benchmark';

const sizeRange = document.getElementById('size-range');
const sizeLabel = document.getElementById('size-label');
const perf = document.getElementById('perf');

import('../pkg')
  .then(pkg => {
    console.log('wasm is imported');

    const originalCanvas = document.querySelector('#eye_canvas').transferControlToOffscreen();
    const stippledCanvas = document.querySelector('#stippled_canvas').transferControlToOffscreen();


    pkg.main(originalCanvas, stippledCanvas);
  })