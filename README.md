# mandelbrot-calculator
[Rust](https://github.com/magnushelliesen/mandelbrot-calculator/blob/main/src/lib.rs)/[Python](https://github.com/magnushelliesen/mandelbrot-calculator/blob/main/notebooks/run_mandelbrot_calculator.ipynb) code, intergated using [Maturin](https://www.maturin.rs/), that allows the user to calculate the Mandelbrot set for some chosen input. Mostly for fun, in order to learn how to pass heavy tasks to Rust from Python.

## The Mandelbrot set
The Mandelbrot set consists of points $c$ on the complex plane for which

$$
  z_{n+1} = z_n^2 + c\ \text{with}\ z_0 = 0
$$

does not explode as $n\to\infty$. Deteremining if a point is in the set is done by checking that

$$
  |z_n|^2 = \mathrm{Re}^2(z_n) + \mathrm{Im}^2(z_n) \leq 4\ \text{for}\ n\leq N,
$$

where $N$ is some maximum number of iterations. Points at which $z_n$ does not explode are given the value $N$ (and are seemingly in the set), and points where $z_n$ *does* explode are given the value $n$ at which $|z_n|^2 > 4$ (and are not in the set).

The code is by no means perfect, and I'll try to make it better and more rusty over time.
But it seems to work as intended. Take this plot for example, it's of the Mandelbrot set on the domain $[-1, 1] \times [-1, 1]$:

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/b942eb73-e9c4-4991-b812-6e17f6c342d4" />

And this is a plot where we graduallty zoom into a point (by a factor of 4 for each iteration):

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/5bea7877-4f96-489a-80ab-d799aa00d264" />

Since each iteration zooms in 4 times, the last plot is zoomed in $4^8=65536$ times (relative to the first), which is equivalent to the first plot being of the whole earth, and the last one being of a $100\times 100$ meter square.

## Rust vs. Python performance
I wrote a [Python class](https://github.com/magnushelliesen/mandelbrot-calculator/blob/main/notebooks/mandelbrot_calculator_py/mandelbrot_calculator.py), doing the same calculations as performed in Rust. The table below shows the speedup relative to the baseline, which is sequential Python code:

 | Language | Sequential  | Parallel    |
 |----------|------------:|------------:|
 | Python   | 1.0         | 2.4         |
 | Rust     | 26.6        | 106.8       |

The table is from this [notebook](https://github.com/magnushelliesen/mandelbrot-calculator/blob/main/notebooks/compare_runtimes.ipynb).
