
      import {Delaunay} from "d3-delaunay";

      let image_element = document.querySelector('#obama')

      const context = image_element?.context2d(width, height);
      const worker = new Worker("/script.js");


      // See image{}
      //
      // Load image
      //
      // Use worker to draw initial rounds of points.
      function messaged({data: points})  {
        context.fillStyle = "#fff";
        context.fillRect(0, 0, width, height);
        context.beginPath();
        for (let i = 0, n = points.length; i < n; i += 2) {
          const x = points[i], y = points[i + 1];
          context.moveTo(x + 1.5, y);
          context.arc(x, y, 1.5, 0, 2 * Math.PI);
        }
        context.fillStyle = "#000";
        context.fill();
      }

      // /// terminate: Start worker from a know point.
      // // is this needed?
      // // worker.terminate();
      // worker.addEventListener("message", messaged);
      // worker.postMessage({data, width, height, n});

      // let image = context;

  // // See data{}
  // //
  // // takes output of image{}  as the variable image.
  // const height = Math.round(width * image.height / image.width);
  // const context = DOM.context2d(width, height, 1);
  // context.drawImage(image, 0, 0, image.width, image.height, 0, 0, width, height);
  // const {data: rgba} = context.getImageData(0, 0, width, height);
  // const data = new Float64Array(width * height);
  // for (let i = 0, n = rgba.length / 4; i < n; ++i) data[i] = Math.max(0, 1 - rgba[i * 4] / 254);
  // data.width = width;
  // data.height = height;
  // return data;
  // })