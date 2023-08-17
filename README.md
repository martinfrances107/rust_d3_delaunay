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


# examples/500_points


to run the application

```console
cd example/500_points
cargo run
```

This produces a file "mesh.svg"

The delaunay mesh is in blue.
The voronoi mesh is in green.

![alt](https://raw.githubusercontent.com/martinfrances107/rust_d3_delaunay/main/images/500_points.svg)


## Phase 1

Early draft port - sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarantees.

## Phase 2

API finalization. There maybe optimization in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

## Other To-do's

This function used generators - which rust does not currently support.

* Maybe I could use next() see  [iter](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
* Maybe [generator](https://crates.io/crates/generator) This routes would be a major breaking change
  as it would require T to become Sync and Send.

Clippy report lots of documentation is missing.
