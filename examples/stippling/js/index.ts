// import { Renderer } from 'benchmark';

const sizeRange = document.getElementById('size-range');
const sizeLabel = document.getElementById('size-label');
const perf = document.getElementById('perf');

import('../pkg')
  .then(pkg => {
    console.log('wasm is imported');

    const stippledCanvas = document.querySelector('#stippled_canvas').transferControlToOffscreen();


    let stippler = pkg.main(stippledCanvas);
    // for (let i = 0; i < 8; i++) {
      // stippler.next();
      stippler.draw();
    // }


    let k = 0;
    const renderLoop = () => {

      stippler.next(k);

      if (k < 80) {
        requestAnimationFrame(renderLoop);
        k++;
      }
    }

    renderLoop();
  })