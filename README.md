# rust d3 delaunay

2021 Edition.
<div align="center">

<a href="https://crates.io/crates/d3_delaunay_rs"><img alt="crates.io" src="https://img.shields.io/crates/v/d3_delaunay_rs.svg"/></a>
<a href="https://docs.rs/d3_delaunay_rs" rel="nofollow noopener noreferrer"><img src="https://docs.rs/d3_geo_rs/badge.svg" alt="Documentation"></a>
<a href="https://crates.io/crates/d3_geo_rs"><img src="https://img.shields.io/crates/d/d3_delaunay_rs.svg" alt="Download" /></a>
</div>

## About

This is a library for computing the Voronoi diagram of a set of two-dimensional points.

This is a port of [d3-delaunay](https://github.com/d3/d3-delaunay). It is in a very early development phase.

It is part of a collection d3 modules ported into RUST

* [d3_geo_rs](https://crates.io/crates/d3_geo_rs)
* d3_delaunay_rs
* [d3_geo_voronoi_rs](https://crates.io/crates/d3_geo_voronoi_rs)

Currently the code coverage, as reported by Cargo tarpaulin is 80%.

## Examples

Three example web pages are provided in the git repository associated with crate

### examples/500_points

This demo renders the meshes associated with a set of 500 points. The points are created at random.

to run the application

```console
cd examples/500_points
cargo run
```

This produces a file "mesh.svg"

![500 points](https://raw.githubusercontent.com/martinfrances107/rust_d3_delaunay/main/images/500_points.svg)

* The delaunay mesh is in blue.

* The voronoi mesh is in green.

### examples/stippling

Using this library - I have port this example into RUST

<https://observablehq.com/@mbostock/voronoi-stippling>

![eye](https://raw.githubusercontent.com/martinfrances107/rust_d3_delaunay/main/images/stippling.png)

to run the example

```bash
cd examples/stippling
npm install
npm run build
npm run serve
```

Currently the RUST port of this example runs in javascrtipt's main event loop. This needs to be refactored so that the main computation run in parallel, runs in a web worker.

### examples/cross_pattern

This is a confidence building exercise
With only 5 points in a symmetric pattern the meshes can be predicted.

## Next steps

API finalization. There maybe optimization in the area of generics.

We need a profile taget based on the stippling example.
To profile and identify bottlenecks.

## Unimplemented generators

Functions that use the generator crate are now availble only when the
"generator" feature is enabled.

The following functions are under going rapid development.

The following generators functions are missing.

| delaunay    |   | voronoi         |
| ------------|---| --------------  |
| neighbors() |   |  cellPolygons() |

### update()

d3-geo-delaunay has a dependency on this npm package [delauantor](https://github.com/mapbox/delaunator)
the function update() allow for rapid retriangulation - in a memory efficient mannor.

This module has a parallel depenecy on [delaunator-rs](https://github.com/mourner/delaunator-rs/issues/30)
unfortunatly this is missing from the rust port.

There is an open issue to add a update function https://github.com/mourner/delaunator-rs/issues/30