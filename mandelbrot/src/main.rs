use num::Complex;

fn main() {
    let c: Complex<f64> = Complex { re: 1.0, im: -1.0 };
}

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
