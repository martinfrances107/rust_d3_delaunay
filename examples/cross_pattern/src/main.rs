#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A known pattern of points
//!
//! Where the voronoi and delaunay are very predictable.
extern crate geo;

use std::fs::File;
use std::io::LineWriter;
use std::io::Write;

use d3_delaunay_rs::delaunay::Delaunay;
use geo_types::Coord;

fn main() -> std::io::Result<()> {
    let file = File::create("mesh.svg")?;
    let mut file = LineWriter::new(file);

    // Drawing area is a 100 100 rectangle with a 10% margin all round.

    file.write_all(
        b"
<svg version=\"1.1\"
  width=\"1280\"
  height=\"1280\"
  viewBox=\"-10 -10 120 120\"
  xmlns=\"http://www.w3.org/2000/svg\"
>\n",
    )?;

    // Fill the unit square with points.
    let points = vec![
        Coord { x: 25f64, y: 25f64 },
        Coord { x: 25f64, y: 75f64 },
        Coord { x: 75f64, y: 75f64 },
        Coord { x: 75f64, y: 25f64 },
        Coord { x: 50f64, y: 50f64 },
    ];
    let delaunay = Delaunay::new(&points);
    let data_points = delaunay.render_points_to_string(Some(0.1_f64));

    // Output points used to generate the delaunay points. ( red )
    file.write_all(b"  <path stroke=\"red\" stroke-width=\"1px\" d=\"")?;
    file.write_all(data_points.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // Output the delaunay mesh. ( green )
    let data_delaunay = delaunay.render_to_string();
    file.write_all(
        b"  <path stroke=\"green\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_delaunay.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    let voronoi = delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));
    // Output voronoi mesh. ( blue )
    let data_voronoi = voronoi.render_to_string();
    file.write_all(
        b"  <path stroke=\"blue\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_voronoi.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // Output Render Bounds ( black )
    let data_bounds = voronoi.render_bounds_to_string();
    file.write_all(
        b"  <path stroke=\"black\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_bounds.as_bytes())?;
    file.write_all(b"\"/>\n")?;
    // Close.
    file.write_all(b"</svg>")?;

    Ok(())
}
