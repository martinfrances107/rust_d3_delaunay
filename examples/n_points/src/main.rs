#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! Generates a large number of random points.
//!
//! Then render the delaunay and voronoi meshes.

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of points generated a random.
    #[arg(short, long, default_value_t = 500)]
    n_points: u16,
}

extern crate clap;
extern crate geo;

use std::fs::File;
use std::io::LineWriter;
use std::io::Write;

use clap::arg;
use clap::command;

use d3_delaunay_rs::delaunay::Delaunay;
use geo_types::Coord;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

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
>\n
<rect x=\"0px\" y=\"0px\" width=\"100\" height=\"100\" fill=\"black\"/>\n",
    )?;

    let mut rng = rand::rng();

    // fill the unit square with points
    let points = (0..args.n_points)
        .map(|_| Coord {
            x: 96_f64.mul_add(rng.random::<f64>(), 2_f64),
            y: 96_f64.mul_add(rng.random::<f64>(), 2_f64),
        })
        .collect::<Vec<_>>();

    let delaunay = Delaunay::new(&points);
    let data_points = delaunay.render_points_to_string(Some(0.1_f64));

    // Output the delaunay mesh. ( green )
    let data_delaunay = delaunay.render_to_string();
    file.write_all(
        b"  <path stroke=\"blue\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_delaunay.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // The following produces an identical mesh to the one above ( green )
    // // Output triangles ( orange )
    // for polygon in delaunay.triangle_polygons_generator() {
    //     let mut l_iter = polygon.0.iter();
    //     if let Some(start) = l_iter.next() {
    //         file.write_all(b"  <path stroke=\"orange\" fill=\"none\" stroke-width=\"0.2px\" d=\"")?;
    //         write!(file, "M{},{}", start.x, start.y)?;

    //         // Loop over remaining points in the line.
    //         for p in l_iter {
    //             write!(file, "L{},{}", p.x, p.y)?;
    //         }

    //         // Close the line
    //         write!(file, "M{},{}", start.x, start.y)?;
    //     }
    //     writeln!(file, "\"/>")?;
    // }
    //
    //
    let voronoi = delaunay.voronoi(Some((0_f64, 0_f64, 100_f64, 100_f64)));
    // Output voronoi mesh. ( blue )
    let data_voronoi = voronoi.render_to_string();
    file.write_all(
        b"  <path stroke=\"green\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_voronoi.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // Output Render Bounds ( yellow )
    let data_bounds = voronoi.render_bounds_to_string();
    file.write_all(
        b"  <path stroke=\"yellow\" fill=\"none\" stroke-width=\"0.2px\" d=\"",
    )?;
    file.write_all(data_bounds.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // Output points used to generate the delaunay points. ( red )
    file.write_all(
        b"  <path fill=\"red\" stroke=\"red\" stroke-width=\"1px\" d=\"",
    )?;
    file.write_all(data_points.as_bytes())?;
    file.write_all(b"\"/>\n")?;

    // Close.
    file.write_all(b"</svg>")?;

    Ok(())
}
