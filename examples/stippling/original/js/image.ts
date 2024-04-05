function image () {

  const context = context.context2d(width, height);
  const worker = new Worker(script);

  function messaged({data: points}) {
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

invalidation.then(() => worker.terminate());
worker.addEventListener("message", messaged);
worker.postMessage({data, width, height, n});
return context.canvas;
}

export image as default;