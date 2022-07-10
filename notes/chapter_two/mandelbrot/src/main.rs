use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing bottom right");

    let mut pixles = vec![0; bounds.0 * bounds.1];

    render(&mut pixles, bounds, upper_left, lower_right);

    write_image(&args[1], &pixles, bounds).expect("error writting file");
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(c) => 255 - c as u8,
            }
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn escape_time(c: Complex<f64>, lim: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..lim {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = (z * z) + c
    }

    None
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,11", ','), Some((10, 11)));
}
