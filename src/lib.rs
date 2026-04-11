use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod mandelbrot_calculator {
    use std::f64;
    use pyo3::prelude::*;
    use rayon::prelude::*;

    #[pyclass(get_all, set_all)]
    struct MandelbrotSet {
        grid_size: usize
    }

    #[pymethods]
    impl MandelbrotSet {
        #[new]
        fn new(grid_size: usize) -> Self {
            MandelbrotSet {grid_size}
        }

        fn make_grid(
            &self,
            re_min: f64,
            re_max: f64,
            im_min: f64,
            im_max: f64,
            max_iter: i64
        ) -> Vec<Vec<i64>> {
            let mut grid = vec![vec![0; self.grid_size]; self.grid_size];

            for row in 0..self.grid_size {
                for col in 0..self.grid_size {
                    let re = re_min + (col as f64 / (self.grid_size - 1) as f64) * (re_max - re_min);
                    let im = im_min + (row as f64 / (self.grid_size - 1) as f64) * (im_max - im_min);

                    grid[self.grid_size - row - 1][col] = _is_in_mandelbrot_set(re, im, max_iter);
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
            max_iter: i64
        ) -> Vec<Vec<i64>> {
            py.detach(|| {
                let mut grid = vec![vec![0; self.grid_size]; self.grid_size];

                for row in 0..self.grid_size {
                    let cols: Vec<_> = (0..self.grid_size)
                        .into_par_iter()
                        .map(|col: usize| {
                            let re = re_min
                                + (col as f64 / (self.grid_size - 1) as f64) * (re_max - re_min);

                            let im = im_min
                                + (row as f64 / (self.grid_size - 1) as f64) * (im_max - im_min);

                            _is_in_mandelbrot_set(re, im, max_iter)
                        })
                        .collect();

                    grid[self.grid_size - row - 1] = cols;
                }

                grid
            })
        }
    }

    fn _is_in_mandelbrot_set(re: f64, im: f64, max_iter: i64) -> i64 {
        let mut z_re = 0.0;
        let mut z_im = 0.0;

        for i in 0..max_iter {
            let _z_re = z_re * z_re - z_im * z_im + re;
            let _z_im = 2.0 * z_re * z_im + im;

            z_re = _z_re;
            z_im = _z_im;

            if z_re * z_re + z_im * z_im > 4.0 {
                return i;
            }
        }

        max_iter
    }

    #[pyfunction]
    fn is_in_mandelbrot_set(re: f64, im: f64, max_iter: i64) -> PyResult<i64> {
        Ok(_is_in_mandelbrot_set(re, im, max_iter))
    }
}