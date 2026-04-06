use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use std::f64;
    use pyo3::prelude::*;

    #[pyclass(get_all, set_all)]
    struct MandelbrotSet {
        grid_size: i64
    }

    #[pymethods]
    impl MandelbrotSet {
        #[new]
        fn new(grid_size: i64) -> Self {
            MandelbrotSet { grid_size }
        }
    
        fn make_grid(&self, re_min: f64, re_max: f64, im_min: f64, im_max: f64) -> Vec<f64> {
            vec![1.0, 2.0, 3.0, 4.0, 5.0]
        }
    }

    #[pyfunction]
    fn is_in_mandelbrot_set(re: f64, im: f64, n: i64) -> PyResult<bool> {
        // Setup variabels
        let mut z_re: f64 = 0.0;
        let mut z_im: f64 = 0.0;
        let _z_re: f64 = 0.0;
        let _z_im: f64 = 0.0;

        // Do n iterations of sequence
        for _ in 0..n {
            z_re =_z_re.powf(2.0)-_z_im.powf(2.0) + re + z_re;
            z_im = 2.0 * _z_re * _z_im + im;
            let _z_re = z_re;
            let _z_im = z_im;
        }

        // Return true if absolute value of z <= 2
        Ok(
            if (z_re.powf(2.0) + z_im.powf(2.0)).sqrt() <= 2.0 {
                true
            } else {
                false
            }
        )
    }
}
