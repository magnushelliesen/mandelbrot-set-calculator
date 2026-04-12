use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;
    use rayon::prelude::*;
    use std::f64;

    #[pyclass(get_all, set_all)]
    struct MandelbrotSet {
        grid_size: usize,
    }

    #[pymethods]
    impl MandelbrotSet {
        #[new]
        fn new(grid_size: usize) -> Self {
            MandelbrotSet { grid_size }
        }

        fn make_grid(
            &self,
            re_min: f64,
            re_max: f64,
            im_min: f64,
            im_max: f64,
            max_iter: i32,
        ) -> Vec<Vec<i32>> {
            let mut grid = vec![vec![0; self.grid_size]; self.grid_size];

            #[allow(clippy::needless_range_loop)]
            for row in 0..self.grid_size {
                for col in 0..self.grid_size {
                    // Calculate the re and im part for point
                    let re =
                        re_min + (col as f64 / (self.grid_size - 1) as f64) * (re_max - re_min);
                    let im =
                        im_min + (row as f64 / (self.grid_size - 1) as f64) * (im_max - im_min);

                    // Return iteration at which point blows up
                    grid[self.grid_size - row - 1][col] =
                        _iteration_at_which_point_explodes(re, im, max_iter);
                }
            }

            grid
        }

        fn make_grid_parallell(
            &self,
            py: Python,
            re_min: f64,
            re_max: f64,
            im_min: f64,
            im_max: f64,
            max_iter: i32,
        ) -> Vec<Vec<i32>> {
            py.detach(|| {
                (0..self.grid_size)
                    .into_par_iter()
                    .map(|row| {
                        (0..self.grid_size)
                            .map(|col| {
                                // Calculate the re and im part for point
                                let re = re_min
                                    + (col as f64 / (self.grid_size - 1) as f64)
                                        * (re_max - re_min);
                                let im = im_min
                                    + (row as f64 / (self.grid_size - 1) as f64)
                                        * (im_max - im_min);

                                // Return iteration at which point blows up
                                _iteration_at_which_point_explodes(re, im, max_iter)
                            })
                            .collect::<Vec<i32>>()
                    })
                    .rev()
                    .collect()
            })
        }
    }

    fn _iteration_at_which_point_explodes(re: f64, im: f64, max_iter: i32) -> i32 {
        let mut z_re = 0.0;
        let mut z_im = 0.0;

        for i in 0..max_iter {
            let z_re_new = z_re * z_re - z_im * z_im + re;
            let z_im_new = 2.0 * z_re * z_im + im;

            z_re = z_re_new;
            z_im = z_im_new;

            // If |z| > 2 at any iteration, escape and return that iteration
            if z_re * z_re + z_im * z_im > 4.0 {
                return i;
            }
        }

        max_iter
    }

    #[pyfunction]
    fn is_in_mandelbrot_set(re: f64, im: f64, max_iter: i32) -> PyResult<bool> {
        match _iteration_at_which_point_explodes(re, im, max_iter) {
            i if i < max_iter => Ok(false),
            i if i == max_iter => Ok(true),
            _ => Err(PyRuntimeError::new_err("iteration failed")),
        }
    }
}
