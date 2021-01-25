#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod voronoi_test {
    // extern crate pretty_assertions;
    use geo::Coordinate;

    use rust_d3_delaunay::delaunay::Delaunay;
    use rust_d3_delaunay::path::Path;
    use rust_d3_delaunay::voronoi::Voronoi;
    use rust_d3_delaunay::RenderingContext2d;

    #[test]
    fn test_noop_for_coincident_points() {
        println!("voronoi.renderCell(i, context) is a noop for coincident points");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
        ];
        let delaunay = Delaunay::new(points);

        let voronoi = Voronoi::new(delaunay, Some((-1f64, -1f64, 2f64, 2f64)));
        let mut path = Path::default();
        voronoi.render_cell::<Path<f64>>(3, &mut path);
        assert_eq!(path.value_str(), String::from(""));
    }

    #[test]
    fn test_render_cell_midpoint() {
        println!("voronoi.renderCell(i, context) handles midpoint coincident with circumcenter");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
        ];
        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, Some((-1f64, -1f64, 2f64, 2f64)));
        let mut context1 = Path::default();
        {
            voronoi.render_cell::<Path<f64>>(0, &mut context1);
        }
        assert_eq!(
            context1.value_str(),
            String::from("M-1,-1L0.5,-1L0.5,0.5L-1,0.5Z")
        );

        let mut context = Path::default();
        voronoi.render_cell(1, &mut context);
        assert_eq!(
            context.value_str(),
            String::from("M2,-1L2,2L0.5,0.5L0.5,-1Z")
        );

        let mut context = Path::default();
        voronoi.render_cell(2, &mut context);
        assert_eq!(
            context.value_str(),
            String::from("M-1,2L-1,0.5L0.5,0.5L2,2Z")
        );
    }

    #[test]
    fn test_contains_false_for_coincident() {
        println!("voronoi.contains(i, x, y) is false for coincident points");
        let points = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
        ];
        let delaunay = Delaunay::new(points);
        let voronoi = Voronoi::new(delaunay, Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(voronoi.contains(3, 1f64, 0f64), false);
        assert_eq!(voronoi.contains(1, 1f64, 0f64), true);
    }

    // tape("voronoi.update() updates the voronoi", test => {
    //   let delaunay = Delaunay.from([[0, 0], [300, 0], [0, 300], [300, 300], [100, 100]]);
    //   let voronoi = delaunay.voronoi([-500, -500, 500, 500]);
    //   for (let i = 0; i < delaunay.points.length; i++) {
    //     delaunay.points[i] = 10 - delaunay.points[i];
    //   }
    //   const p = voronoi.update().cellPolygon(1); // correct after voronoi.update
    //   test.deepEqual(p, [[-500, 500], [-500, -140], [-240, -140], [-140, 60], [-140, 500], [-500, 500]]);
    // });

    // tape("voronoi.update() updates a degenerate voronoi", test => {
    //   const pts = [10, 10, -290, 10, 10, -290, -290, -290, -90, -90];
    //   let delaunay = new Delaunay(Array.from({length: pts.length}).fill(0));
    //   let voronoi = delaunay.voronoi([-500, -500, 500, 500]);
    //   test.deepEqual(voronoi.cellPolygon(0), [ [ 500, -500 ], [ 500, 500 ], [ -500, 500 ], [ -500, -500 ], [ 500, -500 ] ]);
    //   test.equal(voronoi.cellPolygon(1), null);
    //   for (let i = 0; i < delaunay.points.length; i++) {
    //     delaunay.points[i] = pts[i];
    //   }
    //   const p = voronoi.update().cellPolygon(1);
    //   test.deepEqual(p, [[-500, 500], [-500, -140], [-240, -140], [-140, 60], [-140, 500], [-500, 500]]);
    // });

    // tape("zero-length edges are removed", test => {
    //    const voronoi1 = Delaunay.from([[50, 10], [10, 50], [10, 10], [200, 100]]).voronoi([40, 40, 440, 180]);
    //    test.equal(voronoi1.cellPolygon(0).length, 4);
    //    const voronoi2 = Delaunay.from([[10, 10], [20, 10]]).voronoi([0, 0, 30, 20]);
    //    test.deepEqual(voronoi2.cellPolygon(0), [[15, 20], [0, 20], [0, 0], [15, 0], [15, 20]]);
    // });

    #[test]
    fn zero_length_edges_are_removed() {
        let d1 = Delaunay::new(vec![
            Coordinate {
                x: 50.0f64,
                y: 10.0f64,
            },
            Coordinate {
                x: 10.0f64,
                y: 50.0f64,
            },
            Coordinate {
                x: 10.0f64,
                y: 10.0f64,
            },
            Coordinate {
                x: 200.0f64,
                y: 100.0f64,
            },
        ]);
        let voronoi1 = Voronoi::new(d1, Some((40f64, 40f64, 440f64, 180f64)));
        assert_eq!(voronoi1.cell_polygon(0).len(), 4);
    }

    // tape("voronoi neighbors are clipped", test => {
    //    const voronoi = Delaunay.from([[300, 10], [200, 100], [300, 100], [10, 10], [350, 200], [350, 400]]).voronoi([0, 0, 500, 150]);
    //    test.deepEqual([...voronoi.neighbors(0)].sort(), [1, 2]);
    //    test.deepEqual([...voronoi.neighbors(1)].sort(), [0, 2]);
    //    test.deepEqual([...voronoi.neighbors(2)].sort(), [0, 1, 4]);
    //    test.deepEqual([...voronoi.neighbors(3)].sort(), []);
    //    test.deepEqual([...voronoi.neighbors(4)].sort(), [2]);
    //    test.deepEqual([...voronoi.neighbors(5)].sort(), []);
    // });

    // tape("unnecessary points on the corners are avoided (#88)", test => {
    //   for (const [points, lengths] of [
    //     [ [[289,25],[3,22],[93,165],[282,184],[65,89]], [ 6, 4, 6, 5, 6 ] ],
    //     [ [[189,13],[197,26],[47,133],[125,77],[288,15]], [ 4, 6, 5, 6, 5 ] ],
    //     [ [[44,42],[210,193],[113,103],[185,43],[184,37]], [ 5, 5, 7, 5, 6 ]]
    //   ]) {
    //     const voronoi = Delaunay.from(points).voronoi([0, 0, 290, 190]);
    //     test.deepEqual([...voronoi.cellPolygons()].map(d => d.length), lengths);
    //   }
    // });

    // fn unnecessary_point_on_the_corner() {
    //     println!("unnecessary points on the corners are avoided (#88)");
    //     let pattern = vec![(
    //         vec![
    //             Coordinate { x: 289f64, y: 25f64 },
    //             Coordinate { x: 3f64, y: 22f64 },
    //             Coordinate { x: 93f64, y: 165f64 },
    //             Coordinate { x: 282f64, y: 184f64 },
    //             Coordinate { x: 65f64, y: 89f64 },
    //         ],
    //         vec![6, 4, 6, 5, 6],
    //     )];

    //     for (coords, len_array) in pattern {
    //         let delaunay = Delaunay::new(coords);
    //         let voronoi = Voronoi::new(delaunay, Some((0., 0., 290., 190.)));

    //         for i in 0..len_array.len() {
    //             let cell_array = voronoi.cell_polygon(i);
    //             cell_array.map(|d|  d.length).collect();
    //         }
    //     }
    // }

    // tape("a degenerate triangle is avoided", test => {
    //   const pts = [[424.75, 253.75],[424.75, 253.74999999999997],[407.17640687119285, 296.17640687119285],[364.75, 313.75],[322.32359312880715, 296.17640687119285],[304.75, 253.75],[322.32359312880715, 211.32359312880715],[364.75, 193.75],[407.17640687119285, 211.32359312880715],[624.75, 253.75],[607.1764068711929, 296.17640687119285],[564.75, 313.75],[522.3235931288071, 296.17640687119285],[504.75, 253.75],[564.75, 193.75]
    //   ]
    //   const voronoi = Delaunay.from(pts).voronoi([10, 10, 960, 500]);
    //   test.equal(voronoi.cellPolygon(0).length, 4);
    // });

    // tape("cellPolygons filter out empty cells and have the cell index as a property", test => {
    //   const pts = [[0, 0], [3, 3], [1, 1], [-3, -2]];
    //   const voronoi = Delaunay.from(pts).voronoi([0, 0, 2, 2]);
    //   test.deepEqual([...voronoi.cellPolygons()], [
    //     Object.assign([[0, 0], [1, 0], [0, 1], [0, 0]], {index:0, }),
    //     Object.assign([[0, 1], [1, 0], [2, 0], [2, 2], [0, 2], [0, 1]], { index: 2 })
    //   ]);
    // });
}
