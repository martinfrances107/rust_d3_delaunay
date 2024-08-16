#[macro_use]
extern crate criterion;
extern crate pretty_assertions;
extern crate rand;

mod static_points;

use core::time::Duration;
use criterion::Criterion;
use d3_delaunay_rs::delaunay::Delaunay;
use static_points::POINTS;

fn criterion_benchmark(c: &mut Criterion) {
    // Generates array stored in static_points.rs
    // static_points::output_points();

    let mut g = c.benchmark_group("n points");

    // g.measurement_time(Duration::from_secs(11));

    g.bench_function("n_points", |b| {
        b.iter(|| {
            let delaunay = Delaunay::new(&POINTS);
            assert!(delaunay.delaunator.triangles.len() > 500);
            let voronoi =
                delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));

            assert!(voronoi.circumcenters.len() > 50);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
