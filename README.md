# rust d3 delaunay
2021 Edition.

This is a port of the [d3-delaunay](https://github.com/d3/d3-delaunay) library into a RUST library crate/package. It is in a very early development phase.

Current the code coverage as reported by Cargo tarpaulin is 79%.
## Phase 1

Early draft port - sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

## Phase 2

API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

## Other To-do's

This function used generators - which rust does not currently support.
  * maybe I could use next() see  https://doc.rust-lang.org/rust-by-example/trait/iter.html
  * maybe https://crates.io/crates/generator

Clippy report lots of documentation is missing.

