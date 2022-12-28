use num::Complex;
use std::str::FromStr;

fn main() {
    let c: Complex<f64> = Complex { re: 1.0, im: -1.0 };
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
}

/// Given the bounds of an image, and the upper left and bottom right
/// coordinates in the complex plane, convert a given pixel location in
/// the image to the corresponding point in the complex plane.
///
/// Return the complex point if conversion is successfull, None otherwise.
fn pixel_to_complex(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Option<Complex<f64>> {
    if pixel.0 > bounds.0 || pixel.1 > bounds.1 {
        return None;
    }

    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Some(Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    })
}

#[test]
fn test_pixel_to_complex() {
    assert_eq!(
        pixel_to_complex(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Some(Complex {
            re: -0.5,
            im: -0.75
        })
    );
}

/// Parse a pair of floating point numbers separated by a comma
/// as a Complex number.
fn parse_complex(pair: &str) -> Option<Complex<f64>> {
    match parse_pair(pair, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Parses a string of the form XdelimY, where X and Y are of type T,
/// and returns Some((X, Y)) if successfull.
///
/// Returns None if unable to parse pair.
fn parse_pair<T: FromStr>(pair: &str, delim: char) -> Option<(T, T)> {
    match pair.find(delim) {
        Some(index) => {
            match (T::from_str(&pair[..index]), T::from_str(&pair[index + 1..])) {
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None,
            }
        },
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(
        parse_pair::<f64>("1080.0x920.0", 'x'),
        Some((1080.0, 920.0))
    );
    assert_eq!(parse_pair::<i32>("200,500", ','), Some((200, 500)));
    assert_eq!(parse_pair::<f64>("1080.0x920.0df", 'x'), None);
}

/// Determine if a given complex number c is part of the mandelbrot
/// set or not. If c is not a part of the mandelbrot set, return
/// the number of iterations it took to determine it, i, in the
/// form Some(i).
///
/// Return None if c is in the mandelbrot set.
fn escape_time(c: Complex<f64>, iterations: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..iterations {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
