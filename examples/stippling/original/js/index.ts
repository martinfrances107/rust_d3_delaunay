import {Delaunay} from "d3-delaunay";

let image_element = document.querySelector('img#input_image');
console.log(image_element);
if (image_element instanceof HTMLImageElement){

  console.log(image_element);
let width = image_element?.width;
let height = image_element?.height;
let stippled_canvas = document.querySelector('canvas#stippled_canvas');
console.log(stippled_canvas);
if (typeof width === "number" && typeof height === "number" && stippled_canvas instanceof HTMLCanvasElement) {
  let stippled_context = stippled_canvas.getContext('2d');

  if (stippled_context instanceof CanvasRenderingContext2D){

    const worker = new Worker("/script.js");

const n = Math.round(width * height / 40);

// See data{}
//
// takes output of image{}  as the variable image.
// const height = Math.round(width * image.height / image.width);
stippled_context.drawImage(image_element, 0, 0, width, height, 0, 0, width, height);
const {data: rgba} = stippled_context.getImageData(0, 0, width, height);
const data = new Float64Array(width * height);
for (let i = 0, n = rgba.length / 4; i < n; ++i) data[i] = Math.max(0, 1 - rgba[i * 4] / 254);

// See image{}
//
// Load image
//
// Use worker to draw initial rounds of points.
function messaged({data: points})  {
  console.log("inside draw function.");
  stippled_context.fillStyle = "#fff";
  stippled_context.fillRect(0, 0, width, height);
  stippled_context.beginPath();
  for (let i = 0, n = points.length; i < n; i += 2) {
    const x = points[i], y = points[i + 1];
    stippled_context.moveTo(x + 1.5, y);
    stippled_context.arc(x, y, 1.5, 0, 2 * Math.PI);
  }
  stippled_context.fillStyle = "#000";
  stippled_context.fill();
}

// /// terminate: Start worker from a know point.
// // is this needed?
// worker.terminate();
worker.addEventListener("message", messaged);
worker.postMessage({data, width, height, n});


} else {
  console.log("bailing1");
}
} else {
  console.log("bailing2");
}
} else {
  console.log("bailing3");
}
