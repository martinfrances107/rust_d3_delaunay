# Stippling

Based on this code snippet.

<https://observablehq.com/@mbostock/voronoi-stippling>

This javascript example could benefit from a performance boost.

Image Source has been downsampled to 800x600
<https://commons.wikimedia.org/wiki/File:Close_up_of_eye.jpg>

Complications:
  The original <IMAGE> element is copied to a hidden <CANVAS> element
  in order to make the pixels values accessible.

javascript has a update() function ... the RUST port does not.

I am in the process of benchmarking and decide wheather to implement
update().
