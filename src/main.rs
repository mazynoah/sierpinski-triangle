use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use rand::prelude::*;
use std::{
    cmp::Ordering::*,
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
    width: u32,
    height: u32,
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

    fn init(width: u32, height: u32, iterations: u32) -> Self {
        let size = match width.cmp(&height) {
            Equal => width,
            Greater => height,
            Less => width,
        };

        Self {
            triangle: Triangle::new(size.into()),
            rng: rand::thread_rng(),
            iterations,
            width,
            height,
        }
    }

    fn gen_fractal(mut self) -> RgbImage {
        //Setup progress bar
        let pb = ProgressBar::new(self.iterations.into());
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-")
        );

        //Fractal generation
        let mut imgbuf = RgbImage::new(self.width, self.height);
        let mut point = self.get_triangle_random_point();

        for _ in (0..self.iterations).progress_with(pb.clone()) {
            let vertex = self.get_random_vertex();
            let (x, y) = ((point.x + vertex.x) / 2.0, (point.y + vertex.y) / 2.0);
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
            point = Point { x, y };
        }

        print!("Finished in {:?}", pb.elapsed());

        imgbuf
    }
}

fn main() {
    println!("Starting");

    let width = std::env::args()
        .nth(2)
        .expect("no width given")
        .parse()
        .expect("Not a valid number");

    let height = std::env::args()
        .nth(3)
        .expect("no height given")
        .parse()
        .expect("Not a valid number");

    let quality = std::env::args()
        .nth(4)
        .expect("no quality given (recommended 8 milion)")
        .parse()
        .expect("Not a valid number");

    let mut name = std::env::args().nth(1).expect("no name given");
    name = format!("{name}_{width}x{height}_{quality}");

    if !name.ends_with(".png") {
        name += ".png";
    }

    let sier = Sierpinski::init(width, height, quality);
    let image = sier.gen_fractal();
    image
        .save(name)
        .expect("An error occured while trying to save the file");
}
