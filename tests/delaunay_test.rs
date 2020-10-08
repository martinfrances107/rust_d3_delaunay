#![feature(generators, generator_trait)]

mod delaunay_test {
  #[cfg(test)]
  extern crate pretty_assertions;

  use delaunator::Point;
  use delaunator::EMPTY;
  use rust_d3_delaunay::delaunay::Delaunay;
  use std::f64::consts::PI;
  use std::ops::{Generator, GeneratorState};
  use std::pin::Pin;

  #[test]
  fn test_from_array() {
    println!("Delaunay.from(array)");
    let mut points = vec![
      Point { x: 0., y: 0. },
      Point { x: 1f64, y: 0f64 },
      Point { x: 0f64, y: 1f64 },
      Point { x: 1f64, y: 1f64 },
    ];
    let delaunay: Delaunay = Delaunay::new(&mut points);

    let mut neighbors = |i| {
      // degenerate case with several collinear points
      if !delaunay.colinear.is_empty() {
        let l = delaunay.colinear.iter().find(|&&x| x == i);
        match l {
          Some(l) => {
            if *l > 0 {
              yield delaunay.colinear[l - 1];
            }
            // allowed values of l here is zero !!!
            // but I am just copiying the javascript implementation.
            if l < &(delaunay.colinear.len() - 1) {
              yield delaunay.colinear[l + 1];
            }
            // return;
          }
          None => {
            return true;
          }
        }
      }

      let e0 = delaunay.inedges[i];
      if e0 == EMPTY {
        return true;
      } // coincident point
      let mut e = e0;
      let p0 = EMPTY;
      loop {
        let p0 = delaunay.triangles[e];
        yield p0;
        // e = e % 3 == 2 ? e - 2 : e + 1;
        if e % 3 == 2 {
          e = e - 2;
        } else {
          e = e + 1;
        }
        if delaunay.triangles[e] != i {
          return true;
        } // bad triangulation
        e = delaunay.half_edges[e];
        if e == EMPTY {
          let p = delaunay.hull[(delaunay.hull_index[i] + 1) % delaunay.hull.len()];
          if p != p0 {
            yield p;
          }
          return true;
        }
        if e == e0 {
          break true;
        }
        // }
      }
    };

    assert_eq!(
      delaunay.triangles,
      [0usize, 2usize, 1usize, 2usize, 3usize, 1usize]
    );
    assert_eq!(delaunay.half_edges, vec![EMPTY, 5, EMPTY, EMPTY, EMPTY, 1]);
    assert_eq!(delaunay.inedges, vec![2, 4, 0, 3]);
    // assert_eq!(neighbors(0), [1, 2]);
    match Pin::new(&mut neighbors).resume(0) {
      GeneratorState::Yielded(n) => assert_eq!(n, 1usize),
      _ => panic!("unexptected return"),
    }
    match Pin::new(&mut neighbors).resume(0) {
      GeneratorState::Yielded(n) => assert_eq!(n, 2usize),
      _ => panic!("unexptected return"),
    }
    match Pin::new(&mut neighbors).resume(0) {
      GeneratorState::Complete(n) => assert_eq!(n, true),
      _ => panic!("unexptected return"),
    }
    // assert_eq!(Array.from(delaunay.neighbors(1)), [3, 2, 0]);
    // assert_eq!(Array.from(delaunay.neighbors(2)), [0, 1, 3]);
    // assert_eq!(Array.from(delaunay.neighbors(3)), [2, 1]);
  }

  // tape("Delaunay.from(array) handles coincident points", test => {
  //   let delaunay = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 0]]);
  //   test.deepEqual(delaunay.inedges, Int32Array.of(2, 1, 0, -1));
  //   test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
  //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
  //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
  //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
  // });

  #[test]
  fn test_handles_coincident_points() {
    println!("Delaunay.from(array)");

    let mut points = vec![
      Point { x: 0f64, y: 0f64 },
      Point { x: 1f64, y: 0f64 },
      Point { x: 0f64, y: 1f64 },
      Point { x: 1f64, y: 0f64 },
    ];
    let delaunay: Delaunay = Delaunay::new(&mut points);
    assert_eq!(delaunay.inedges, vec![2, 1, 0, EMPTY]);

    println!(" TODO add mssing neigbors asserts");
    //   test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
    //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
    //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
    //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
  }

  // tape("Delaunay.from(iterable)", test => {
  //   let delaunay = Delaunay.from((function*() {
  //     yield [0, 0];
  //     yield [1, 0];
  //     yield [0, 1];
  //     yield [1, 1];
  //   })());
  //   test.deepEqual(delaunay.points, Float64Array.of(0, 0, 1, 0, 0, 1, 1, 1));
  //   test.deepEqual(delaunay.triangles, Uint32Array.of(0, 2, 1, 2, 3, 1));
  //   test.deepEqual(delaunay.halfedges, Int32Array.of(-1, 5, -1, -1, -1, 1));
  // });

  // tape("Delaunay.from(iterable, fx, fy)", test => {
  //   let delaunay = Delaunay.from((function*() {
  //     yield {x: 0, y: 0};
  //     yield {x: 1, y: 0};
  //     yield {x: 0, y: 1};
  //     yield {x: 1, y: 1};
  //   })(), d => d.x, d => d.y);
  //   test.deepEqual(delaunay.points, Float64Array.of(0, 0, 1, 0, 0, 1, 1, 1));
  //   test.deepEqual(delaunay.triangles, Uint32Array.of(0, 2, 1, 2, 3, 1));
  //   test.deepEqual(delaunay.halfedges, Int32Array.of(-1, 5, -1, -1, -1, 1));
  // });

  // tape("Delaunay.from({length}, fx, fy)", test => {
  //   let delaunay = Delaunay.from({length: 4}, (d, i) => i & 1, (d, i) => (i >> 1) & 1);
  //   test.deepEqual(delaunay.points, Float64Array.of(0, 0, 1, 0, 0, 1, 1, 1));
  //   test.deepEqual(delaunay.triangles, Uint32Array.of(0, 2, 1, 2, 3, 1));
  //   test.deepEqual(delaunay.halfedges, Int32Array.of(-1, 5, -1, -1, -1, 1));
  // });

  // tape("delaunay.voronoi() uses the default bounds", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 1]]).voronoi();
  //   test.equal(voronoi.xmin, 0);
  //   test.equal(voronoi.ymin, 0);
  //   test.equal(voronoi.xmax, 960);
  //   test.equal(voronoi.ymax, 500);
  // });

  fn test_voronoi_default_bounds() {
    let mut points = vec![
      Point { x: 0f64, y: 0f64 },
      Point { x: 1f64, y: 0f64 },
      Point { x: 0f64, y: 1f64 },
      Point { x: 1f64, y: 0f64 },
    ];
    // let voronoi: Delaunay<f64> = Delaunay::new(&mut points).voronoi();

    // let voronoi = Delaunay::new(points).voronoi();
    //  assert_eq!(voronoi.xmin, 0);
    //  assert_eq!(voronoi.ymin, 0);
    //   test.equal(voronoi.xmax, 960);
    //   test.equal(voronoi.ymax, 500);
  }
  // tape("delaunay.voronoi([xmin, ymin, xmax, ymax]) uses the specified bounds", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 1]]).voronoi([-1, -1, 2, 2]);
  //   test.equal(voronoi.xmin, -1);
  //   test.equal(voronoi.ymin, -1);
  //   test.equal(voronoi.xmax, 2);
  //   test.equal(voronoi.ymax, 2);
  // });

  // tape("delaunay.voronoi() returns the expected diagram", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 1]]).voronoi();
  //   test.deepEqual(voronoi.circumcenters, Float64Array.of(0.5, 0.5, 0.5, 0.5));
  //   test.deepEqual(voronoi.vectors, Float64Array.of(0, -1, -1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, 1, 0));
  // });

  // tape("delaunay.voronoi() skips cells for coincident points", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 0]]).voronoi([-1, -1, 2, 2]);
  //   test.deepEqual(voronoi.circumcenters, Float64Array.of(0.5, 0.5));
  //   test.deepEqual(voronoi.vectors, Float64Array.of(0, -1, -1, 0, 1, 1, 0, -1, -1, 0, 1, 1, 0, 0, 0, 0));
  // });

  // tape("delaunay.voronoi() for zero point returns expected values", test => {
  //   let voronoi = Delaunay.from([]).voronoi([-1, -1, 2, 2]);
  //   test.equal(voronoi.render(), null);
  // });

  // tape("delaunay.voronoi() for one point returns the bounding rectangle", test => {
  //   let voronoi = Delaunay.from([[0, 0]]).voronoi([-1, -1, 2, 2]);
  //   test.equal(voronoi.renderCell(0), "M2,-1L2,2L-1,2L-1,-1Z");
  //   test.equal(voronoi.render(), null);
  // });

  // tape("delaunay.voronoi() for two points", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [1, 0], [1, 0]]).voronoi([-1, -1, 2, 2]);
  //   test.equal(voronoi.renderCell(0), "M0.5,2L-1,2L-1,-1L0.5,-1Z");
  //   test.equal(voronoi.delaunay.find(-1,0), 0);
  //   test.equal(voronoi.delaunay.find(2,0), 1);
  // });

  // tape("delaunay.voronoi() for collinear points", test => {
  //   let voronoi = Delaunay.from([[0, 0], [1, 0], [-1, 0]]).voronoi([-1, -1, 2, 2]);
  //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(0)).sort(), [1, 2]);
  //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(1)), [0]);
  //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(2)), [0]);
  // });

  // tape("delaunay.find(x, y) returns the index of the cell that contains the specified point", test => {
  //   let delaunay = Delaunay.from([[0, 0], [300, 0], [0, 300], [300, 300], [100, 100]]);
  //   test.equal(delaunay.find(49, 49), 0);
  //   test.equal(delaunay.find(51, 51), 4);
  // });

  // tape("delaunay.find(x, y) works with one or two points", test => {
  //   const points = [[0, 1], [0, 2]];
  //   const delaunay = Delaunay.from(points);
  //   test.equal(points[delaunay.find(0, -1)][1], 1);
  //   test.equal(points[delaunay.find(0, 2.2)][1], 2);
  //   delaunay.points.fill(0);
  //   delaunay.update();
  //   test.equal(delaunay.find(0, -1), 0);
  //   test.equal(delaunay.find(0, 1.2), 0);
  // });

  // tape("delaunay.find(x, y) works with collinear points", test => {
  //   const points = [[0, 1], [0, 2], [0, 4], [0, 0], [0, 3], [0, 4], [0, 4]];
  //   const delaunay = Delaunay.from(points);
  //   test.equal(points[delaunay.find(0, -1)][1], 0);
  //   test.equal(points[delaunay.find(0, 1.2)][1], 1);
  //   test.equal(points[delaunay.find(1, 1.9)][1], 2);
  //   test.equal(points[delaunay.find(-1, 3.3)][1], 3);
  //   test.equal(points[delaunay.find(10, 10)][1], 4);
  //   test.equal(points[delaunay.find(10, 10, 0)][1], 4);
  // });

  // tape("delaunay.find(x, y) works with collinear points 2", test => {
  //   const points = Array.from({ length: 120 }, (_, i) => [i * 4, i / 3 + 100]);
  //   const delaunay = Delaunay.from(points);
  //   test.deepEqual([...delaunay.neighbors(2)], [ 1, 3 ]);
  // });

  // tape("delaunay.find(x, y) works with collinear points 3", test => {
  //   const points = Array.from({ length: 120 }, (_, i) => [i * 4, i / 3 + 100]);
  //   const delaunay = Delaunay.from(points);
  //   test.deepEqual([...delaunay.neighbors(2)], [ 1, 3 ]);
  // });

  // tape("delaunay.find(x, y) works with collinear points (large)", test => {
  //   const points = Array.from({length: 2000}, (_,i) => [i**2,i**2]);
  //   const delaunay = Delaunay.from(points);
  //   test.equal(points[delaunay.find(0, -1)][1], 0);
  //   test.equal(points[delaunay.find(0, 1.2)][1], 1);
  //   test.equal(points[delaunay.find(3.9, 3.9)][1], 4);
  //   test.equal(points[delaunay.find(10, 9.5)][1], 9);
  //   test.equal(points[delaunay.find(10, 9.5, 0)][1], 9);
  //   test.equal(points[delaunay.find(1e6, 1e6)][1], 1e6);
  // });

  // tape("delaunay.update() allows fast updates", test => {
  //   let delaunay = Delaunay.from([[0, 0], [300, 0], [0, 300], [300, 300], [100, 100]]);
  //   let circumcenters1 = delaunay.voronoi([-500, -500, 500, 500]).circumcenters;
  //   for (let i = 0; i < delaunay.points.length; i++) {
  //     delaunay.points[i] = -delaunay.points[i];
  //   }
  //   delaunay.update();
  //   let circumcenters2 = delaunay.voronoi([-500, -500, 500, 500]).circumcenters;
  //   test.deepEqual(circumcenters1, Float64Array.from([ 150, -50, -50, 150, 250, 150, 150, 250 ]));
  //   test.deepEqual(circumcenters2, Float64Array.from([ -150, 50, -250, -150, 50, -150, -150, -250 ]));
  // });

  // tape("delaunay.update() updates collinear points", test => {
  //   const delaunay = new Delaunay(Array.from({ length: 250 }).fill(0));
  //   test.equal(delaunay.collinear, undefined);
  //   for (let i = 0; i < delaunay.points.length; i++)
  //     delaunay.points[i] = (i % 2) ? i : 0;
  //   delaunay.update();
  //   test.equal(delaunay.collinear.length, 125);
  //   for (let i = 0; i < delaunay.points.length; i++)
  //     delaunay.points[i] = Math.sin(i);
  //   delaunay.update();
  //   test.equal(delaunay.collinear, undefined);
  //   for (let i = 0; i < delaunay.points.length; i++)
  //     delaunay.points[i] = (i % 2) ? i : 0;
  //   delaunay.update();
  //   test.equal(delaunay.collinear.length, 125);
  //   for (let i = 0; i < delaunay.points.length; i++)
  //     delaunay.points[i] = 0;
  //   delaunay.update();
  //   test.equal(delaunay.collinear, undefined);
  // });

  // tape("delaunay.find(x, y) with coincident point", test => {
  //   let delaunay = Delaunay.from([[0, 0], [0, 0], [10,10], [10, -10]]);
  //   test.equal(delaunay.find(100,100), 2);
  //   test.ok(delaunay.find(0,0,1) > -1);
  //   delaunay = Delaunay.from(Array.from({length:1000}, () => [0, 0]).concat([[10,10], [10, -10]]));
  //   test.ok(delaunay.find(0,0,1) > -1);
  // });

  // tape("delaunay.find(x, y, i) traverses the convex hull", test => {
  //   let delaunay = new Delaunay(Float64Array.of(509,253,426,240,426,292,567,272,355,356,413,392,319,408,374,285,327,303,381,215,475,319,301,352,247,426,532,334,234,366,479,375,251,302,340,170,160,377,626,317,177,296,322,243,195,422,241,232,585,358,666,406,689,343,172,198,527,401,766,350,444,432,117,316,267,170,580,412,754,425,117,231,725,300,700,222,438,165,703,168,558,221,475,211,491,125,216,166,240,108,783,266,640,258,184,77,387,90,162,125,621,162,296,78,532,154,763,199,132,165,422,343,312,128,125,77,450,95,635,106,803,415,714,63,529,87,388,152,575,126,573,64,726,381,773,143,787,67,690,117,813,203,811,319));
  //   test.equal(delaunay.find(49, 311), 31);
  //   test.equal(delaunay.find(49, 311, 22), 31);
  // });

  // tape("delaunay.renderHull(context) is closed", test => {
  //   let delaunay = Delaunay.from([[0, 0], [1, 0], [0, 1], [1, 1]]);
  //   let context = new Context;
  //   test.equal((delaunay.renderHull(context), context.toString()), `M0,1L1,1L1,0L0,0Z`);
  // });
}
