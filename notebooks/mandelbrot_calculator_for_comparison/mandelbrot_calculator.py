import os
from concurrent.futures import ProcessPoolExecutor

from typing import List, Tuple


class MandelbrotSet:
    def __init__(self, grid_size: int, max_workers: int | None = None) -> None:
        self.grid_size = grid_size
        self.max_workers = max_workers

    #######################
    ## Sequential method ##
    #######################

    def make_grid(
        self,
        re_min: float,
        re_max: float,
        im_min: float,
        im_max: float,
        max_iter: int,
    ) -> List[List[int]]:
        # Initialize a 2D grid to hold the iteration counts for each point
        grid = [[0 for _ in range(self.grid_size)] for _ in range(self.grid_size)]

        # Calculate the step size for real and imaginary parts based on the grid size
        for row in range(self.grid_size):
            for col in range(self.grid_size):
                re = re_min + (col / (self.grid_size - 1)) * (re_max - re_min)
                im = im_min + (row / (self.grid_size - 1)) * (im_max - im_min)

                grid[self.grid_size - row - 1][col] = (
                    self._iteration_at_which_point_explodes(re, im, max_iter)
                )

        return grid

    #####################
    ## Parallel method ##
    #####################

    def make_grid_parallel(
        self,
        re_min: float,
        re_max: float,
        im_min: float,
        im_max: float,
        max_iter: int,
    ) -> List[List[int]]:
        grid = [[0 for _ in range(self.grid_size)] for _ in range(self.grid_size)]

        row_args = [
            (row, self.grid_size, re_min, re_max, im_min, im_max, max_iter)
            for row in range(self.grid_size)
        ]

        workers = self.max_workers or os.cpu_count() or 1
        with ProcessPoolExecutor(max_workers=workers) as executor:
            completed_rows = executor.map(self._compute_row, row_args)
            for row, values in completed_rows:
                grid[self.grid_size - row - 1] = values

        return grid

    @staticmethod
    def _compute_row(
        args: Tuple[int, int, float, float, float, float, int]
    ) -> Tuple[int, List[int]]:
        row, grid_size, re_min, re_max, im_min, im_max, max_iter = args

        im = im_min + (row / (grid_size - 1)) * (im_max - im_min)
        values: List[int] = []

        for col in range(grid_size):
            re = re_min + (col / (grid_size - 1)) * (re_max - re_min)
            values.append(
                MandelbrotSet._iteration_at_which_point_explodes(re, im, max_iter)
            )

        return row, values

    ###################
    ## Helper method ##
    ###################

    @staticmethod
    def _iteration_at_which_point_explodes(re: float, im: float, max_iter: int) -> int:
        z_re = 0.0
        z_im = 0.0

        for i in range(max_iter):
            z_re_new = z_re * z_re - z_im * z_im + re
            z_im_new = 2.0 * z_re * z_im + im

            z_re = z_re_new
            z_im = z_im_new

            # If |z| > 2 at any iteration, escape and return that iteration
            if z_re * z_re + z_im * z_im > 4.0:
                return i

        return max_iter
