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

An example app is provided in the git repository associated with crate

### examples/500_points

This demo renders the meshes associated with a set of 500 points. The points are created at random.

to run the application

```console
cd example/500_points
cargo run
```

This produces a file "mesh.svg"

![500 points](https://raw.githubusercontent.com/martinfrances107/rust_d3_delaunay/main/images/500_points.svg)

* The delaunay mesh is in blue.

* The voronoi mesh is in green.

## Phase 1

Early draft port - sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarantees.

## Phase 2

API finalization. There maybe optimization in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

## Unimplemented generators

Functions that use the generator crate are now availble only when the
"generator" feature is enabled.

The following functions are under going rapid development.

The following generators functions are missing.

| delaunay    |   | voronoi         |
| ------------|---| --------------  |
| neighbors() |   |  cellPolygons() |
|             |   |  update()       |
