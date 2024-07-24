#![allow(clippy::pedantic)]
#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod voronoi {
    extern crate pretty_assertions;

    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_delaunay_rs::delaunay::Delaunay;
    use d3_delaunay_rs::path::Path;
    use d3_delaunay_rs::voronoi::Voronoi;

    type DelaunayStub = Delaunay<f64>;

    type VoronoiStub = Voronoi<f64>;

    #[test]
    fn simple() {
        let points = vec![
            Coord {
                x: -20.0_f64,
                y: 20.0_f64,
            },
            Coord { x: 20., y: 20. },
        ];
        let voronoi: VoronoiStub = Delaunay::new(&points).voronoi(None);

        // TODO is this test meaningful.
        assert_eq!(voronoi.render_cell_to_string(0), "");
        assert_eq!(
            voronoi.render_cell_to_string(1),
            "M960,0L960,500L0,500L0,0Z"
        );
        assert_eq!(voronoi.render_to_string(), "M0,20L0,0M0,20L0,500");
    }

    #[test]
    fn noop_for_coincident_points() {
        println!(
            "voronoi.renderCell(i, context) is a noop for coincident points"
        );
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 0f64 },
        ];
        let voronoi: VoronoiStub =
            Delaunay::new(&points).voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        let mut path = Path::default();
        voronoi.render_cell(3, &mut path);
        assert_eq!(path.to_string(), String::from(""));
    }

    #[test]
    fn render_cell_midpoint() {
        println!("voronoi.renderCell(i, context) handles midpoint coincident with circumcenter");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
        ];
        let voronoi: VoronoiStub =
            Delaunay::new(&points).voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        let mut context1 = Path::default();
        {
            voronoi.render_cell(0, &mut context1);
        }
        assert_eq!(
            context1.to_string(),
            "M-1,-1L0.5,-1L0.5,0.5L-1,0.5Z"
        );

        let mut context = Path::default();
        voronoi.render_cell(1, &mut context);
        assert_eq!(
            context.to_string(),
            "M2,-1L2,2L0.5,0.5L0.5,-1Z"
        );

        let mut context = Path::default();
        voronoi.render_cell(2, &mut context);
        assert_eq!(
            context.to_string(),
            "M-1,2L-1,0.5L0.5,0.5L2,2Z"
        );
    }

    #[test]
    fn contains_false_for_coincident() {
        println!("voronoi.contains(i, x, y) is false for coincident points");
        let points = vec![
            Coord { x: 0_f64, y: 0_f64 },
            Coord { x: 1_f64, y: 0_f64 },
            Coord { x: 0_f64, y: 1_f64 },
            Coord { x: 1_f64, y: 0_f64 },
        ];

        let voronoi: VoronoiStub = Delaunay::new(&points)
            .voronoi(Some((-1_f64, -1_f64, 2_f64, 2_f64)));
        assert_eq!(voronoi.contains(3, &Coord { x: 1_f64, y: 0_f64 }), false);
        assert_eq!(voronoi.contains(1, &Coord { x: 1_f64, y: 0_f64 }), true);
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

    #[test]
    fn zero_length_edges_are_removed() {
        println!("zero-length edges are removed");
        let voronoi1: VoronoiStub = Delaunay::new(&[
            Coord {
                x: 50.0f64,
                y: 10.0f64,
            },
            Coord {
                x: 10.0f64,
                y: 50.0f64,
            },
            Coord {
                x: 10.0f64,
                y: 10.0f64,
            },
            Coord {
                x: 200.0f64,
                y: 100.0f64,
            },
        ])
        .voronoi(Some((40f64, 40f64, 440f64, 180f64)));
        assert_eq!(voronoi1.cell_polygon(0).len(), 4);

        let voronoi2: VoronoiStub = Delaunay::new(&[
            Coord {
                x: 10.0f64,
                y: 10.0f64,
            },
            Coord {
                x: 20.0f64,
                y: 10.0f64,
            },
        ])
        .voronoi(Some((0f64, 0f64, 30f64, 20f64)));

        assert_eq!(
            voronoi2.cell_polygon(0),
            vec![
                Coord {
                    x: 0.0f64,
                    y: 20.0f64,
                },
                Coord {
                    x: 0.0f64,
                    y: 0.0f64,
                },
                Coord {
                    x: 15.0f64,
                    y: 0.0f64,
                },
                Coord {
                    x: 15.0f64,
                    y: 20.0f64,
                },
                Coord {
                    x: 0.0f64,
                    y: 20.0f64,
                }
            ]
        );
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
    //             Coord {
    //                 x: 289f64,
    //                 y: 25f64,
    //             },
    //             Coord { x: 3f64, y: 22f64 },
    //             Coord {
    //                 x: 93f64,
    //                 y: 165f64,
    //             },
    //             Coord {
    //                 x: 282f64,
    //                 y: 184f64,
    //             },
    //             Coord { x: 65f64, y: 89f64 },
    //         ],
    //         vec![6, 4, 6, 5, 6],
    //     )];

    //     for (coords, len_array) in pattern {
    //         let voronoi = Delaunay::new(coords).voronoi(Some((0., 0., 290., 190.)));

    //         for i in 0..len_array.len() {
    //             let cell_array = voronoi.cell_polygon(i);
    //             cell_array.map(|d| d.length).collect();
    //         }
    //     }
    // }

    // tape("a degenerate triangle is avoided", test => {
    //   const pts = [[424.75, 253.75],[424.75, 253.74999999999997],[407.17640687119285, 296.17640687119285],[364.75, 313.75],[322.32359312880715, 296.17640687119285],[304.75, 253.75],[322.32359312880715, 211.32359312880715],[364.75, 193.75],[407.17640687119285, 211.32359312880715],[624.75, 253.75],[607.1764068711929, 296.17640687119285],[564.75, 313.75],[522.3235931288071, 296.17640687119285],[504.75, 253.75],[564.75, 193.75]
    //   ]
    //   const voronoi = Delaunay.from(pts).voronoi([10, 10, 960, 500]);
    //   test.equal(voronoi.cellPolygon(0).length, 4);
    // });

    #[test]
    fn a_degenerate_triangle_is_avoided() {
        let pts = vec![
            Coord {
                x: 424.75,
                y: 253.75,
            },
            Coord {
                x: 424.75,
                y: 253.74999999999997,
            },
            Coord {
                x: 407.17640687119285,
                y: 296.17640687119285,
            },
            Coord {
                x: 364.75,
                y: 313.75,
            },
            Coord {
                x: 322.32359312880715,
                y: 296.17640687119285,
            },
            Coord {
                x: 304.75,
                y: 253.75,
            },
            Coord {
                x: 322.32359312880715,
                y: 211.32359312880715,
            },
            Coord {
                x: 364.75,
                y: 193.75,
            },
            Coord {
                x: 407.17640687119285,
                y: 211.32359312880715,
            },
            Coord {
                x: 624.75,
                y: 253.75,
            },
            Coord {
                x: 607.1764068711929,
                y: 296.17640687119285,
            },
            Coord {
                x: 564.75,
                y: 313.75,
            },
            Coord {
                x: 522.3235931288071,
                y: 296.17640687119285,
            },
            Coord {
                x: 504.75,
                y: 253.75,
            },
            Coord {
                x: 564.75,
                y: 193.75,
            },
        ];
        let d: DelaunayStub = Delaunay::new(&pts);
        let voronoi = d.voronoi(Some((10., 10., 960., 500.)));
        assert_eq!(voronoi.cell_polygon(0).len(), 4);
    }
    // tape("cellPolygons filter out empty cells and have the cell index as a property", test => {
    //   const pts = [[0, 0], [3, 3], [1, 1], [-3, -2]];
    //   const voronoi = Delaunay.from(pts).voronoi([0, 0, 2, 2]);
    //   test.deepEqual([...voronoi.cellPolygons()], [
    //     Object.assign([[0, 0], [1, 0], [0, 1], [0, 0]], {index:0, }),
    //     Object.assign([[0, 1], [1, 0], [2, 0], [2, 2], [0, 2], [0, 1]], { index: 2 })
    //   ]);
    // });

    // This test in not in the original
    //
    // it("pattern produces a cross", () => {
    //   const points = [
    //     [25, 25],
    //     [25, 75],
    //     [75, 75],
    //     [75, 25],
    //     [50, 50],
    //   ];

    //   const delaunay = Delaunay.from(points);
    //   const voronoi = delaunay.voronoi([0, 0, 100, 100]);
    //   // let context1 = new Context;
    //   // assert.strictEqual((delaunay.renderPoints(context1, 4), context1.toString()), `M-1,-1L0.5,-1L0.5,0.5L-1,0.5Z`);
    //   let context2 = new Context;
    //   assert.strictEqual((delaunay.render(context2), context2.toString()), `M50,50L25,25M25,75L50,50M75,75L50,50M75,25L50,50M75,75L75,25L25,25L25,75Z`);
    //   let context3 = new Context;
    //   assert.strictEqual((voronoi.render(context3), context3.toString()), `M25,50L50,25M25,50L50,75M50,75L75,50M75,50L50,25M50,75L50,100M75,50L100,50M50,25L50,0M25,50L0,50`);
    //   // let context4 = new Context;
    //   // assert.strictEqual((voronoi.renderBounds(context4), context4.toString()), `x`);
    // })

    #[test]
    fn generates_a_cross_pattern() {
        println!("pattern produces a cross");

        let points = [
            Coord {
                x: 25_f64,
                y: 25_f64,
            },
            Coord {
                x: 25_f64,
                y: 75_f64,
            },
            Coord {
                x: 75_f64,
                y: 75_f64,
            },
            Coord {
                x: 75_f64,
                y: 25f64,
            },
            Coord {
                x: 50_f64,
                y: 50_f64,
            },
        ];

        let delaunay = Delaunay::new(&points);
        // assert_eq!(
        //     delaunay.render_points_to_string(Some(4_f64)),
        //     String::from("M-1,2L-1,0.5L0.5,0.5L2,2Z")
        // );

        assert_eq!(
            delaunay.render_to_string(),
            String::from(
                "M25,75L50,50M50,50L25,25M75,75L50,50M50,50L75,25M75,75L75,25L25,25L25,75Z"
            )
        );

        let voronoi = delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));

        assert_eq!(
            voronoi.render_to_string(),
               String::from("M25,50L50,75M25,50L50,25M50,75L75,50M50,25L75,50M50,75L50,100M75,50L100,50M50,25L50,0M25,50L0,50")
        );

        // assert_eq!(voronoi.render_to_string(), String::from("fail"));
    }

    // it("almost colinear triangle", () => {
    // const points = [
    //   [90, 73],
    //   [7, 87],
    //   [33, 85]
    // ];

    // const delaunay = Delaunay.from(points);
    // const voronoi = delaunay.voronoi([0, 0, 100, 100]);
    // let context1 = new Context;
    // assert.strictEqual((delaunay.renderPoints()), `M92,73A2,2,0,1,1,88,73A2,2,0,1,1,92,73M9,87A2,2,0,1,1,5,87A2,2,0,1,1,9,87M35,85A2,2,0,1,1,31,85A2,2,0,1,1,35,85`);
    // let context2 = new Context;
    // assert.strictEqual((delaunay.render(context2), context2.toString()), `M33,85L90,73L7,87Z`);
    // let context3 = new Context;
    // assert.strictEqual((voronoi.render(context3), context3.toString()), `M13.384615384615387,0L21.07692307692308,100M44.8684210526316,0L65.92105263157896,100`);

    // })
    // }

    #[test]
    fn almost_colinear_triangle() {
        println!("almost colinear triangle");

        let points = [
            Coord {
                x: 90_f64,
                y: 73_f64,
            },
            Coord {
                x: 7_f64,
                y: 87_f64,
            },
            Coord {
                x: 33_f64,
                y: 85_f64,
            },
        ];

        let delaunay = Delaunay::new(&points);

        assert_eq!(
            delaunay.render_to_string(),
            String::from("M33,85L90,73L7,87Z")
        );

        let voronoi = delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));

        assert_eq!(
        voronoi.render_to_string(),
           String::from("M13.384615384615387,0L21.07692307692308,100M44.8684210526316,0L65.92105263157896,100")
    );
    }
}
