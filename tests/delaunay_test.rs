#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod delaunay_test {
    extern crate pretty_assertions;

    use delaunator::EMPTY;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_delaunay_rs::delaunay::Delaunay;
    use d3_delaunay_rs::path::Path;
    use d3_delaunay_rs::voronoi::Voronoi;
    use d3_geo_rs::clip::antimeridian::ClipAntimeridianC;
    use d3_geo_rs::clip::antimeridian::ClipAntimeridianU;
    use d3_geo_rs::projection::builder::template::NoPCNU;
    use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
    use d3_geo_rs::projection::builder::template::ResampleNoPCNU;
    use d3_geo_rs::projection::gnomic::Gnomic;
    use d3_geo_rs::stream::DrainStub;

    type DelaunayStub = Delaunay<
        ClipAntimeridianC<ResampleNoPCNC<DrainStub<f64>, Gnomic<f64>, f64>, f64>,
        ClipAntimeridianU<ResampleNoPCNC<DrainStub<f64>, Gnomic<f64>, f64>, f64>,
        DrainStub<f64>,
        NoPCNU,
        Gnomic<f64>,
        ResampleNoPCNU<Gnomic<f64>, f64>,
        f64,
    >;

    type VoronoiStub = Voronoi<
        ClipAntimeridianC<ResampleNoPCNC<DrainStub<f64>, Gnomic<f64>, f64>, f64>,
        ClipAntimeridianU<ResampleNoPCNC<DrainStub<f64>, Gnomic<f64>, f64>, f64>,
        DrainStub<f64>,
        NoPCNU,
        Gnomic<f64>,
        ResampleNoPCNU<Gnomic<f64>, f64>,
        f64,
    >;

    #[test]
    fn delaunay_from_array() {
        println!("Delaunay.from(array)");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
        ];
        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(
            delaunay.points,
            vec![
                Coord { x: 0f64, y: 0f64 },
                Coord { x: 1f64, y: 0f64 },
                Coord { x: 0f64, y: 1f64 },
                Coord { x: 1f64, y: 1f64 },
            ]
        );

        assert_eq!(delaunay.triangles, vec![0, 2, 1, 2, 3, 1]);
        assert_eq!(delaunay.half_edges, vec![EMPTY, 5, EMPTY, EMPTY, EMPTY, 1]);
        assert_eq!(delaunay.inedges, vec![2, 4, 0, 3]);

        // Cannot reproduce neighbors tests...neighbors is a generator function!

        // test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
        //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
        //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
        //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
    }

    #[test]
    fn handles_coincident_points() {
        println!("Delaunay.from(array) handles coincident points.");

        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 0f64 },
        ];

        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(delaunay.inedges, vec![2, 1, 0, EMPTY]);

        // Cannot reproduce neighbors tests...
        // neighbors is part of rust_d3_geo_voronoi.delaunay.

        //   test.deepEqual(Array.from(delaunay.neighbors(0)), [1, 2]);
        //   test.deepEqual(Array.from(delaunay.neighbors(1)), [2, 0]);
        //   test.deepEqual(Array.from(delaunay.neighbors(2)), [0, 1]);
        //   test.deepEqual(Array.from(delaunay.neighbors(3)), []);
    }

    // fn delaunay_from_iterable() {
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
    fn voronoi_default_bounds() {
        println!("delaunay.voronoi() uses the default bounds");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
        ];

        let voronoi: VoronoiStub = Delaunay::new(&points).voronoi(None);
        assert_eq!(voronoi.xmin, 0f64);
        assert_eq!(voronoi.ymin, 0f64);
        assert_eq!(voronoi.xmax, 960f64);
        assert_eq!(voronoi.ymax, 500f64);
    }

    #[test]
    fn voronoi_specific_bounds() {
        println!("delaunay.voronoi([xmin, ymin, xmax, ymax]) uses the specified bounds");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
        ];

        let voronoi: VoronoiStub = Delaunay::new(&points).voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(voronoi.xmin, -1f64);
        assert_eq!(voronoi.ymin, -1f64);
        assert_eq!(voronoi.xmax, 2f64);
        assert_eq!(voronoi.ymax, 2f64);
    }

    #[test]
    fn voronoi_returns_the_expected_diagram() {
        println!("delaunay.voronoi() returns the expected diagram");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
        ];

        let voronoi: VoronoiStub = Delaunay::new(&points).voronoi(None);
        assert_eq!(
            voronoi.circumcenters,
            vec![Coord { x: 0.5, y: 0.5 }, Coord { x: 0.5, y: 0.5 }]
        );
        assert_eq!(
            voronoi.vectors,
            vec![
                Coord {
                    x: 0.0f64,
                    y: -1.0f64
                },
                Coord {
                    x: -1f64,
                    y: 0.0f64
                },
                Coord {
                    x: 1.0f64,
                    y: 0.0f64
                },
                Coord {
                    x: 0f64,
                    y: -1.0f64
                },
                Coord {
                    x: -1f64,
                    y: 0.0f64
                },
                Coord { x: 0f64, y: 1.0f64 },
                Coord { x: 0f64, y: 1.0f64 },
                Coord {
                    x: 1.0f64,
                    y: 0.0f64
                }
            ]
        );
    }

    #[test]
    fn voronoi_skips_cells_for_coincident_points() {
        println!("delaunay.voronoi() skips cells for coincident points");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 0f64 },
        ];

        let voronoi: VoronoiStub = Delaunay::new(&points).voronoi(None);

        assert_eq!(voronoi.circumcenters, vec![Coord { x: 0.5, y: 0.5 }]);
        assert_eq!(
            voronoi.vectors,
            vec![
                Coord { x: 0.0f64, y: -1.0 },
                Coord { x: -1f64, y: 0.0 },
                Coord { x: 1.0, y: 1.0 },
                Coord { x: 0f64, y: -1.0 },
                Coord { x: -1f64, y: 0.0 },
                Coord { x: 1f64, y: 1.0 },
                Coord { x: 0f64, y: 0.0 },
                Coord { x: 0f64, y: 0.0 }
            ]
        );
    }

    #[test]
    fn delaunay_return_for_zero_points() {
        println!("delaunay.voronoi() for zero point returns expected values");
        let v: VoronoiStub = Delaunay::new(&[]).voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(v.render_to_string(), "");
    }

    #[test]
    fn delaunay_render_points_accepts_r() {
        println!("delaunay.voronoi() for zero point returns expected values");
        let points = vec![Coord {
            x: 0.0_f64,
            y: 0.0_f64,
        }];
        let d: DelaunayStub = Delaunay::new(&points);

        assert_eq!(
            d.render_points_to_string(None),
            "M2,0A2,2,0,1,1,-2,0A2,2,0,1,1,2,0"
        );
        assert_eq!(
            d.render_points_to_string(Some(5_f64)),
            "M5,0A5,5,0,1,1,-5,0A5,5,0,1,1,5,0"
        );

        let mut path = Path::default();
        d.render_points(&mut path, Some(3_f64));
        assert_eq!(path.to_string(), "M3,0A3,3,0,1,1,-3,0A3,3,0,1,1,3,0");
    }

    #[test]
    fn delaunay_return_for_one_point() {
        println!("delaunay.voronoi() for one point returns the bounding rectangle");
        let points = vec![Coord { x: 0., y: 0. }];
        let d: DelaunayStub = Delaunay::new(&points);
        let voronoi = d.voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(voronoi.render_cell_to_string(0), "M2,-1L2,2L-1,2L-1,-1Z");
        assert_eq!(voronoi.render_to_string(), "");
    }

    #[test]
    fn delaunay_return_for_two_point() {
        println!("delaunay.voronoi() for one point returns the bounding rectangle");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
        ];
        let d: DelaunayStub = Delaunay::new(&points);
        let voronoi = d.voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(
            voronoi.render_cell_to_string(0),
            "M-1,2L-1,-1L0.5,-1L0.5,2Z"
        );
        assert_eq!(
            voronoi.delaunay.find(
                &Coord {
                    x: -1_f64,
                    y: 0_f64
                },
                None
            ),
            0
        );
        let d: DelaunayStub = Delaunay::new(&points);
        let voronoi = d.voronoi(Some((-1f64, -1f64, 2f64, 2f64)));
        assert_eq!(
            voronoi.delaunay.find(&Coord { x: 2_f64, y: 0_f64 }, None),
            1
        );
    }

    // tape("delaunay.voronoi() for collinear points", test => {
    //   let voronoi = Delaunay.from([[0, 0], [1, 0], [-1, 0]]).voronoi([-1, -1, 2, 2]);
    //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(0)).sort(), [1, 2]);
    //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(1)), [0]);
    //   test.deepEqual(Array.from(voronoi.delaunay.neighbors(2)), [0]);
    // });

    #[test]
    fn find_x_y_returns_index_for_speficied_point() {
        println!(
            "delaunay.find(x, y) returns the index of the cell that contains the specified point"
        );

        let delaunay: DelaunayStub = Delaunay::new(&[
            Coord { x: 0., y: 0. },
            Coord { x: 300., y: 0. },
            Coord { x: 0., y: 300. },
            Coord { x: 300., y: 300. },
            Coord { x: 100., y: 100. },
        ]);
        assert_eq!(delaunay.find(&Coord { x: 49., y: 49. }, None), 0);
        let delaunay: DelaunayStub = Delaunay::new(&[
            Coord { x: 0., y: 0. },
            Coord { x: 300., y: 0. },
            Coord { x: 0., y: 300. },
            Coord { x: 300., y: 300. },
            Coord { x: 100., y: 100. },
        ]);
        assert_eq!(delaunay.find(&Coord { x: 51., y: 51. }, None), 4);
    }

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
    #[test]
    fn find_works_with_one_or_two_points() {
        println!("delaunay.find(x, y) works with one or two points");

        let points = vec![Coord { x: 0., y: 1. }, Coord { x: 0., y: 2. }];
        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: -1. }, None)].y, 1.);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: 2.2 }, None)].y, 2.);

        // TODO must complete this test when update() is implemented.
        // delaunay.points.fill(Coord { x: 0., y: 0. });
        // delaunay.update();
        // assert_eq(delaynay.find(&Coord { x: 0., y: -1 }, None), 0);
        // assert_eq(delaynay.find(&Coord { x: 0., y: 1.2 }, None), 0);
    }

    #[test]
    fn find_works_with_collinear_points() {
        println!("delaunay.find(x, y) works with collinear points");

        let points = vec![
            Coord { x: 0., y: 1. },
            Coord { x: 0., y: 2. },
            Coord { x: 0., y: 4. },
            Coord { x: 0., y: 0. },
            Coord { x: 0., y: 3. },
            Coord { x: 0., y: 4. },
            Coord { x: 0., y: 4. },
        ];
        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: -1. }, None)].y, 0.0);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: 1.2 }, None)].y, 1.);

        assert_eq!(points[delaunay.find(&Coord { x: 1., y: 1.9 }, None)].y, 2.);
        assert_eq!(points[delaunay.find(&Coord { x: -1., y: 3.3 }, None)].y, 3.);
        assert_eq!(points[delaunay.find(&Coord { x: 10., y: 10. }, None)].y, 4.);
        assert_eq!(
            points[delaunay.find(&Coord { x: 10., y: 10. }, Some(0))].y,
            4.
        );
    }

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

    #[test]
    fn find_works_with_collinear_points_large() {
        println!("delaunay.find(x, y) works with collinear points (large)");
        let points: Vec<Coord<f64>> = (0..2000_i32)
            .map(|i| Coord {
                x: i.pow(2u32) as f64,
                y: i.pow(2u32) as f64,
            })
            .collect();
        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: -1. }, None)].y, 0.);
        assert_eq!(points[delaunay.find(&Coord { x: 0., y: 1.2 }, None)].y, 1.);
        assert_eq!(points[delaunay.find(&Coord { x: 3.9, y: 3.9 }, None)].y, 4.);
        assert_eq!(
            points[delaunay.find(&Coord { x: 10.0, y: 9.5 }, Some(0))].y,
            9.
        );
        assert_eq!(
            points[delaunay.find(&Coord { x: 1e6, y: 1e6 }, None)].y,
            1e6
        );
    }

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

    #[test]
    fn delaunay_find_tranverses_the_convex_hull() {
        println!("delaunay.find(x, y, i) traverses the convex hull");
        let points = vec![
            Coord {
                x: 509_f64,
                y: 253_f64,
            },
            Coord {
                x: 426_f64,
                y: 240_f64,
            },
            Coord {
                x: 426_f64,
                y: 292_f64,
            },
            Coord {
                x: 567_f64,
                y: 272_f64,
            },
            Coord {
                x: 355_f64,
                y: 356_f64,
            },
            Coord {
                x: 413_f64,
                y: 392_f64,
            },
            Coord {
                x: 319_f64,
                y: 408_f64,
            },
            Coord {
                x: 374_f64,
                y: 285_f64,
            },
            Coord {
                x: 327_f64,
                y: 303_f64,
            },
            Coord {
                x: 381_f64,
                y: 215_f64,
            },
            Coord {
                x: 475_f64,
                y: 319_f64,
            },
            Coord {
                x: 301_f64,
                y: 352_f64,
            },
            Coord {
                x: 247_f64,
                y: 426_f64,
            },
            Coord {
                x: 532_f64,
                y: 334_f64,
            },
            Coord {
                x: 234_f64,
                y: 366_f64,
            },
            Coord {
                x: 479_f64,
                y: 375_f64,
            },
            Coord {
                x: 251_f64,
                y: 302_f64,
            },
            Coord {
                x: 340_f64,
                y: 170_f64,
            },
            Coord {
                x: 160_f64,
                y: 377_f64,
            },
            Coord {
                x: 626_f64,
                y: 317_f64,
            },
            Coord {
                x: 177_f64,
                y: 296_f64,
            },
            Coord {
                x: 322_f64,
                y: 243_f64,
            },
            Coord {
                x: 195_f64,
                y: 422_f64,
            },
            Coord {
                x: 241_f64,
                y: 232_f64,
            },
            Coord {
                x: 585_f64,
                y: 358_f64,
            },
            Coord {
                x: 666_f64,
                y: 406_f64,
            },
            Coord {
                x: 689_f64,
                y: 343_f64,
            },
            Coord {
                x: 172_f64,
                y: 198_f64,
            },
            Coord {
                x: 527_f64,
                y: 401_f64,
            },
            Coord {
                x: 766_f64,
                y: 350_f64,
            },
            Coord {
                x: 444_f64,
                y: 432_f64,
            },
            Coord {
                x: 117_f64,
                y: 316_f64,
            },
            Coord {
                x: 267_f64,
                y: 170_f64,
            },
            Coord {
                x: 580_f64,
                y: 412_f64,
            },
            Coord {
                x: 754_f64,
                y: 425_f64,
            },
            Coord {
                x: 117_f64,
                y: 231_f64,
            },
            Coord {
                x: 725_f64,
                y: 300_f64,
            },
            Coord {
                x: 700_f64,
                y: 222_f64,
            },
            Coord {
                x: 438_f64,
                y: 165_f64,
            },
            Coord {
                x: 703_f64,
                y: 168_f64,
            },
            Coord {
                x: 558_f64,
                y: 221_f64,
            },
            Coord {
                x: 475_f64,
                y: 211_f64,
            },
            Coord {
                x: 491_f64,
                y: 125_f64,
            },
            Coord {
                x: 216_f64,
                y: 166_f64,
            },
            Coord {
                x: 240_f64,
                y: 108_f64,
            },
            Coord {
                x: 783_f64,
                y: 266_f64,
            },
            Coord {
                x: 640_f64,
                y: 258_f64,
            },
            Coord {
                x: 184_f64,
                y: 77_f64,
            },
            Coord {
                x: 387_f64,
                y: 90_f64,
            },
            Coord {
                x: 162_f64,
                y: 125_f64,
            },
            Coord {
                x: 621_f64,
                y: 162_f64,
            },
            Coord {
                x: 296_f64,
                y: 78_f64,
            },
            Coord {
                x: 532_f64,
                y: 154_f64,
            },
            Coord {
                x: 763_f64,
                y: 199_f64,
            },
            Coord {
                x: 132_f64,
                y: 165_f64,
            },
            Coord {
                x: 422_f64,
                y: 343_f64,
            },
            Coord {
                x: 312_f64,
                y: 128_f64,
            },
            Coord {
                x: 125_f64,
                y: 77_f64,
            },
            Coord {
                x: 450_f64,
                y: 95_f64,
            },
            Coord {
                x: 635_f64,
                y: 106_f64,
            },
            Coord {
                x: 803_f64,
                y: 415_f64,
            },
            Coord {
                x: 714_f64,
                y: 63_f64,
            },
            Coord {
                x: 529_f64,
                y: 87_f64,
            },
            Coord {
                x: 388_f64,
                y: 152_f64,
            },
            Coord {
                x: 575_f64,
                y: 126_f64,
            },
            Coord {
                x: 573_f64,
                y: 64_f64,
            },
            Coord {
                x: 726_f64,
                y: 381_f64,
            },
            Coord {
                x: 773_f64,
                y: 143_f64,
            },
            Coord {
                x: 787_f64,
                y: 67_f64,
            },
            Coord {
                x: 690_f64,
                y: 117_f64,
            },
            Coord {
                x: 813_f64,
                y: 203_f64,
            },
            Coord {
                x: 811_f64,
                y: 319_f64,
            },
        ];

        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(
            delaunay.find(
                &Coord {
                    x: 49_f64,
                    y: 311_f64
                },
                None
            ),
            31
        );
        let delaunay2: DelaunayStub = Delaunay::new(&points);
        assert_eq!(
            delaunay2.find(
                &Coord {
                    x: 49_f64,
                    y: 311_f64
                },
                None
            ),
            31
        )
    }
    #[test]
    fn hull_context_is_closed() {
        println!("delaunay.renderHull(context) is closed");
        let points = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
        ];
        let delaunay: DelaunayStub = Delaunay::new(&points);
        assert_eq!(delaunay.render_hull_to_string(), "M0,1L1,1L1,0L0,0Z");
    }
}
