use chrono::Utc;
use clap::Parser;
use console::style;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use rand::prelude::*;
use std::path::Path;
use std::{
    fmt::Write,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    /// Generates a new isometric triangle
    fn new(length: f64) -> Self {
        let a = (0., 0.);
        let b = (length, 0.);
        let c = (length / 2.0, length * 3f64.sqrt() / 2.0);

        Triangle::from_tuples(a, b, c)
    }

    fn from_points(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }

    fn from_tuples(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> Self {
        Triangle {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

struct Sierpinski {
    triangle: Triangle,
    rng: ThreadRng,
    iterations: u32,
    size: u32,
}

impl Sierpinski {
    fn random_barycentric_coordinates(&mut self) -> (f64, f64) {
        let r1 = self.rng.gen_range(0.0..=1.0);
        let r2 = self.rng.gen_range(0.0..=1.0 - r1);

        (r1, r2)
    }

    fn get_triangle_random_point(&mut self) -> Point {
        let (u, v) = self.random_barycentric_coordinates();
        // P = A + u * (B - A) + v * (C - A)
        self.triangle.a
            + (self.triangle.b - self.triangle.a) * u
            + (self.triangle.c - self.triangle.a) * v
    }

    fn get_random_vertex(&mut self) -> Point {
        let arr = [self.triangle.a, self.triangle.b, self.triangle.c];

        arr[self.rng.gen_range(0..arr.len())]
    }

    fn init(size: u32, iterations: u32) -> Self {
        Self {
            triangle: Triangle::new(size.into()),
            rng: rand::thread_rng(),
            iterations,
            size,
        }
    }

    fn gen_fractal(mut self) -> RgbImage {
        println!(
            "{} {}Generating fractal...",
            style("[1/3]").bold().dim(),
            console::Emoji("🌀  ", "")
        );

        //Setup progress bar
        let pb = ProgressBar::new(self.iterations.into());
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-")
        );

        //Fractal generation
        let mut imgbuf = RgbImage::new(self.size, self.size);
        let mut point = self.get_triangle_random_point();

        for _ in (0..self.iterations).progress_with(pb.clone()) {
            let vertex = self.get_random_vertex();
            let (x, y) = ((point.x + vertex.x) / 2.0, (point.y + vertex.y) / 2.0);
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
            point = Point { x, y };
        }

        pb.finish_with_message(format!("Finished in {:?}.", pb.elapsed()));

        imgbuf
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    size: u32,

    #[arg(short, long, default_value_t = 4_000_000)]
    quality: u32,

    #[arg(short = 'd', long, default_value_t = {String::from("./")})]
    output_directory: String,
}

fn check_path(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = path.parent().ok_or("Invalid path")?;
    if !parent_dir.exists() {
        return Err(format!("Directory {:?} does not exist", parent_dir).into());
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let file_name = format!(
        "{0}_{1}x{1}_{2}.png",
        Utc::now().format("%d%H%M%S"),
        args.size,
        args.quality
    );

    let path = Path::new(&args.output_directory).join(file_name);

    match check_path(&path) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}: {err}", console::style("error").red(),);
            std::process::exit(1);
        }
    }

    let sier = Sierpinski::init(args.size, args.quality);
    let image = sier.gen_fractal();

    println!(
        "{} {}Saving file...",
        style("[2/3]").bold().dim(),
        console::Emoji("💾  ", "")
    );
    image
        .save(&path)
        .map(|()| {
            println!(
                "{} {}Saved to: {}",
                style("[3/3]").bold().dim(),
                console::Emoji("✅  ", ""),
                path.display()
            )
        })
        .expect("An error occured while trying to save the file");
}
