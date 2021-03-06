#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod delaunay_test {
    extern crate pretty_assertions;

    // use delaunator::Point;
    use delaunator::EMPTY;
    use geo::Coordinate;
    use rust_d3_delaunay::delaunay::Delaunay;
    use rust_d3_delaunay::voronoi::Voronoi;

    #[test]
    fn test_from_array() {
        println!("Delaunay.from(array)");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
        ];
        let delaunay = Delaunay::new(points);
        assert_eq!(
            delaunay.points,
            vec![
                Coordinate { x: 0f64, y: 0f64 },
                Coordinate { x: 1f64, y: 0f64 },
                Coordinate { x: 0f64, y: 1f64 },
                Coordinate { x: 1f64, y: 1f64 },
            ]
        );

        assert_eq!(delaunay.triangles, vec![0, 2, 1, 2, 3, 1]);
        assert_eq!(delaunay.half_edges, vec![EMPTY, 5, EMPTY, EMPTY, EMPTY, 1]);
        assert_eq!(delaunay.inedges, vec![2, 4, 0, 3]);

        // Cannot reproduce neighbors tests...neighbors is a genrator function!

        // test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
        //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
        //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
        //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
    }

    #[test]
    fn test_handles_coincident_points() {
        println!("Delaunay.from(array) handles coincident points.");

        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
        ];
        let delaunay: Delaunay<f64> = Delaunay::new(points);
        assert_eq!(delaunay.inedges, vec![2, 1, 0, EMPTY]);

        // Cannot reproduce neighbors tests...
        // neighbors is part of rust_d3_geo_voronoi.delaunay.

        //   test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
        //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
        //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
        //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
    }

    // fn test_delaunay_from_iterable() {
    //     println!("Delaunay.from(iterable)");
    //     // iterable not supported in initial rustlang version.
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

    #[test]
    fn test_voronoi_default_bounds() {
        println!("delaunay.voronoi() uses the default bounds");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
        ];

        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, None);
        assert_eq!(voronoi.xmin, 0f64);
        assert_eq!(voronoi.ymin, 0f64);
        assert_eq!(voronoi.xmax, 960f64);
        assert_eq!(voronoi.ymax, 500f64);
    }

    #[test]
    fn test_voronoi_specific_bounds() {
        println!("delaunay.voronoi([xmin, ymin, xmax, ymax]) uses the specified bounds");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
        ];

        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(voronoi.xmin, -1f64);
        assert_eq!(voronoi.ymin, -1f64);
        assert_eq!(voronoi.xmax, 2f64);
        assert_eq!(voronoi.ymax, 2f64);
    }

    #[test]
    fn test_voronoi_returns_the_expected_diagram() {
        println!("delaunay.voronoi() returns the expected diagram");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
        ];

        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, None);
        assert_eq!(
            voronoi.circumcenters,
            vec![Coordinate { x: 0.5, y: 0.5 }, Coordinate { x: 0.5, y: 0.5 }]
        );
        assert_eq!(
            voronoi.vectors,
            vec![
                Coordinate {
                    x: 0.0f64,
                    y: -1.0f64
                },
                Coordinate {
                    x: -1f64,
                    y: 0.0f64
                },
                Coordinate {
                    x: 1.0f64,
                    y: 0.0f64
                },
                Coordinate {
                    x: 0f64,
                    y: -1.0f64
                },
                Coordinate {
                    x: -1f64,
                    y: 0.0f64
                },
                Coordinate { x: 0f64, y: 1.0f64 },
                Coordinate { x: 0f64, y: 1.0f64 },
                Coordinate {
                    x: 1.0f64,
                    y: 0.0f64
                }
            ]
        );
    }

    #[test]
    fn test_voronoi_skips_cells_for_coincident_points() {
        println!("delaunay.voronoi() skips cells for coincident points");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
        ];

        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, None);

        assert_eq!(voronoi.circumcenters, vec![Coordinate { x: 0.5, y: 0.5 }]);
        assert_eq!(
            voronoi.vectors,
            vec![
                Coordinate { x: 0.0f64, y: -1.0 },
                Coordinate { x: -1f64, y: 0.0 },
                Coordinate { x: 1.0, y: 1.0 },
                Coordinate { x: 0f64, y: -1.0 },
                Coordinate { x: -1f64, y: 0.0 },
                Coordinate { x: 1f64, y: 1.0 },
                Coordinate { x: 0f64, y: 0.0 },
                Coordinate { x: 0f64, y: 0.0 }
            ]
        );
    }

    #[test]
    fn test_delaunay_return_for_zero_points() {
        // println!("delaunay.voronoi() for zero point returns expected values");
        // let delaunay = Delaunay::new(vec![]);
        // let voronoi = Voronoi::new(delaunay, Some((-1f64, -1f64, 2f64, 2f64)));
        // assert_eq!(voronoi.render(), None);
    }

    // tape("delaunay.voronoi() for one point returns the bounding rectangle", test => {
    //   let voronoi = Delaunay.from([[0, 0]]).voronoi([-1, -1, 2, 2]);
    //   test.equal(voronoi.renderCell(0), "M2,-1L2,2L-1,2L-1,-1Z");
    //   test.equal(voronoi.render(), null);
    // });

    #[test]
    fn test_delaunay_return_for_one_point() {
        println!("delaunay.voronoi() for one point returns the bounding rectangle");
        let points = vec![Coordinate { x: 0., y: 0. }];
        let voronoi = Delaunay::new(points).voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(voronoi.render_cell_to_path(0), "M2,-1L2,2L-1,2L-1,-1Z");
        // assert_eq!(voronoi.render(0), None);
    }

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
