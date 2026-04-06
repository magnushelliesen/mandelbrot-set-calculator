use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use std::f64;
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn is_in_mandelbrot_set(re: f64, im: f64, n: i64) -> PyResult<bool> {
        let mut z_re: f64 = 0.0;
        let mut z_im: f64 = 0.0;
        let _z_re: f64 = 0.0;
        let _z_im: f64 = 0.0;

        for _ in 0..n {
            z_re =_z_re.powf(2.0)-_z_im.powf(2.0) + re + z_re;
            z_im = 2.0 * _z_re * _z_im + im;
            let _z_re = z_re;
            let _z_im = z_im;
        }

        Ok(
            if (z_re.powf(2.0) + z_im.powf(2.0)).sqrt() <= 2.0 {
                true
            } else {
                false
            }
        )
    }
}
