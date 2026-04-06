use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use std::f64;
    use pyo3::prelude::*;

    #[pyclass(get_all, set_all)]
    struct MandelbrotSet {
        grid_size: usize,
        max_iter: i64
    }

    #[pymethods]
    impl MandelbrotSet {
        #[new]
        fn new(grid_size: usize, max_iter: i64) -> Self {
            MandelbrotSet { grid_size, max_iter}
        }
    
    fn make_grid(&self, re_min: f64, re_max: f64, im_min: f64, im_max: f64) -> Vec<Vec<bool>> {
        let mut grid = vec![vec![false; self.grid_size]; self.grid_size];

        for row in 0..self.grid_size {
            for col in 0..self.grid_size {
                let re = re_min + (col as f64 / self.grid_size as f64) * (re_max - re_min);
                let im = im_min + (row as f64 / self.grid_size as f64) * (im_max - im_min);

                grid[row][col] = _is_in_mandelbrot_set(re, im, self.max_iter);
            }
        }

        grid
        }
    }

    fn _is_in_mandelbrot_set(re: f64, im: f64, max_iter: i64) -> bool {
        let mut z_re = 0.0;
        let mut z_im = 0.0;

        for _ in 0..max_iter {
            let _z_re = z_re * z_re - z_im * z_im + re;
            let _z_im = 2.0 * z_re * z_im + im;

            z_re = _z_re;
            z_im = _z_im;

            if z_re * z_re + z_im * z_im > 4.0 {
                return false;
            }
        }

        true
    }

    #[pyfunction]
    fn is_in_mandelbrot_set(re: f64, im: f64, max_iter: i64) -> PyResult<bool> {
        Ok(_is_in_mandelbrot_set(re, im, max_iter))
    }
}
