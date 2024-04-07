const sizeRange = document.getElementById("size-range");
const sizeLabel = document.getElementById("size-label");
const perf = document.getElementById("perf");

import("../pkg").then((pkg) => {
  console.log("wasm is imported");

  const stippledCanvas = document
    .querySelector("#stippled_canvas")
    .transferControlToOffscreen();

  let stippler = pkg.main(stippledCanvas);
  stippler.draw();

  let k = 0;
  const renderLoop = () => {
    console.log("k={}", k);
    stippler.next(k);

    if (k < 80) {
      requestAnimationFrame(renderLoop);
      k++;
    }
  };

  renderLoop();
});
