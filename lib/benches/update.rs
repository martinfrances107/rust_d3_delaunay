#[macro_use]
extern crate criterion;
extern crate pretty_assertions;
extern crate rand;

mod static_points;

use core::time::Duration;
use criterion::Criterion;
use d3_delaunay_rs::delaunay::Delaunay;
use geo_types::Coord;

use static_points::POINTS;

// 1000 points, updated 1000 times.
fn criterion_benchmark(c: &mut Criterion) {
    // Generates array stored in static_points.rs
    // static_points::output_points();

    let mut g = c.benchmark_group("update");

    g.measurement_time(Duration::from_secs(70));

    g.bench_function("update", |b| {
        b.iter(|| {
            let mut delaunay = Delaunay::new(&POINTS);
            let mut voronoi =
                delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));

            // 1000 call to update
            for _i in 0..1000 {
                for pair in voronoi.delaunay.points.chunks_exact_mut(4) {
                    // One walks up, one walk right...
                    pair[0].x += 1_f64;
                    pair[1].y += 1_f64;
                    pair[2].x -= 1_f64;
                    pair[3].y -= 1_f64;
                }

                // Before update is implement voronoi must be
                // recomputed from scratch.
                delaunay = Delaunay::new(&voronoi.delaunay.points);
                voronoi =
                    delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));

                assert!(voronoi.delaunay.delaunator.triangles.len() > 500);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
