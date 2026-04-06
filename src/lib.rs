use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use std::f64;
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn is_in_mandelbrot_set(c_re: f64, c_im: f64, n: i64) -> PyResult<bool> {
        let z_re_1: f64 = 0.0;
        let z_im_1: f64 = 0.0;
        let _z_re_0: f64 = 0.0;
        let _z_im_0: f64 = 0.0;
        for _ in 0..n {
            let z_re_1 = _z_re_0.powf(2.0)-_z_im_0.powf(2.0) + c_re;
            let z_im_1 = 2.0 * _z_re_0 * _z_im_0 + c_im;
            let _z_re_0 = z_re_1;
            let _z_im_0 = z_im_1;

            print!("{} {}\n", z_re_1, z_im_1);
        }

        Ok(
            if (z_re_1.powf(2.0) + z_im_1.powf(2.0)).sqrt() < 2.0 {
                true
            } else {
                false
            }
        )
    }
}
