from mandelbrot_calculator import (
    is_in_mandelbrot_set,
    MandelbrotSet
)

def test_mandelbrot_calculator():
    # Testing function
    assert is_in_mandelbrot_set(0, 0, 100) is True, "is_in_mandelbrot_set failed"

    # Testing class
    ms = MandelbrotSet(10)

    expected = [
        [2, 2, 3, 4, 9, 3, 2, 1, 1, 1],
        [2, 3, 4, 6, 100, 5, 4, 2, 1, 1],
        [4, 5, 100, 100, 100, 100, 67, 3, 2, 1],
        [10, 8, 100, 100, 100, 100, 100, 3, 2, 1],
        [100, 100, 100, 100, 100, 100, 100, 3, 2, 1],
        [100, 100, 100, 100, 100, 100, 100, 3, 2, 1],
        [10, 8, 100, 100, 100, 100, 100, 3, 2, 1],
        [4, 5, 100, 100, 100, 100, 67, 3, 2, 1],
        [2, 3, 4, 6, 100, 5, 4, 2, 1, 1],
        [2, 2, 3, 4, 9, 3, 2, 1, 1, 1]
    ]

    observed = ms.make_grid(-1, 1, -1, 1, 100)

    assert expected == observed, "make_grid failed"

    observed = ms.make_grid_parallell(-1, 1, -1, 1, 100)

    assert expected == observed, "make_grid_parallell failed"