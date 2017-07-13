extern crate num;
extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;

use num::complex::Complex64;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;
//use serde::de::{Deserialize, Deserializer, Error, Visitor};


type Rgba = image::Rgba<u8>;


// Write the Docopt usage string
const USAGE: &'static str = "
Usage: mandelbrot [options]

Options:
    -h                  Display this usage statement
    -i ITER             Maximum iterations [default: 30]
    --step STEP         A pixel is drawn for each step [default: 0.003]
    --x0 FROMX          the left hand x coordinate to start from [default: -2.0]
    --x1 TOX            the right hand x coordinate to go to [default: 1.0]
    --y0 FROMY          the top y coordinate (-ve is UP) [default: -1.2]
    --y1 TOY            the bottom y coordiante (+ve is DOWN) [default: 1.2]
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_h: bool,
    flag_i: u32,
    flag_step: f64,
    flag_x0: f64,
    flag_x1: f64,
    flag_y0: f64,
    flag_y1: f64,
}


fn mandelbrot(x: f64, y: f64, imax: u32) -> u32 {
    let a = Complex64::new(x, y);
    let mut i: u32 = 0;
    let mut z = a.clone();
    //while abs(z) < 2.0 && i < imax {
    while z.norm_sqr() < 4.0 && i < imax {
        i += 1;
        z = z * z + a;
    }
    i
}


fn transpose_coordinate(i: u32, min: f64, step: f64) -> f64 {
    min + (step * (i as f64))
}


fn get_range(min: f64, max: f64, step: f64) -> u32 {
    (((max - min) / step).floor()) as u32
}


fn main() {
    // command line options .. todo
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_h {
        println!("{}", USAGE);
        return;
    }

    let step: f64 = args.flag_step;
    let max_iterations: u32 = args.flag_i;
    let y_m_min: f64 = args.flag_y0;
    let y_m_max: f64 = args.flag_y1;
    let x_m_min: f64 = args.flag_x0;
    let x_m_max: f64 = args.flag_x1;

    let h = get_range(y_m_min, y_m_max, step);
    let w = get_range(x_m_min, x_m_max, step);
    println!("max iterations: {}", max_iterations);
    println!("Generating mandelbrot W:{} H:{}", w, h);

    //make an image
    let mut img = image::RgbaImage::new(w, h);
    let black = Rgba { data:[0, 0, 0, 255] };
    let colours = rainbow(max_iterations);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let depth = mandelbrot(
            transpose_coordinate(x, x_m_min, step),
            transpose_coordinate(y, y_m_min, step),
            max_iterations);
        //set the image colour according to depth.
        *pixel = if depth < max_iterations {
            colours[depth as usize]
        } else {
            black
        };
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
    println!("All done!");
}


fn rainbow(c: u32) -> Vec<Rgba> {
    (0..c).map(|i| Rgba { data:[
        sin_to_dec(c, i, 0.0 * std::f64::consts::PI * 2.0/3.0),
        sin_to_dec(c, i, 2.0 * std::f64::consts::PI * 2.0/3.0),
        sin_to_dec(c, i, 1.0 * std::f64::consts::PI * 2.0/3.0),
        255] })
        .collect()
}


fn sin_to_dec(c: u32, i: u32, phase: f64) -> u8 {
    let s = (std::f64::consts::PI / (c as f64) * 2.0 * (i as f64) + phase).sin();
    (((s * 127.0) + 128.0).floor()) as u8
}
