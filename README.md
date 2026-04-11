# mandelbrot-calculator
Rust/Python code, intergated using Maturin, that allows the user to calculate the Mandelbrot set for some chosen input. Mostly for fun, in order to learn how to pass heavy tasks to Rust from Python.

The Mandelbrot set consists of points $c$ on the complex plane for which

$$
  z_{n+1} = z_n^2 + c\ \mbox{with}\ z_0 = 0
$$

does not explode as $n\to\infty$. Deteremining if a point is in the set is done by checking that

$$
  |z_n|^2 = \mathrm{Re}^2(z_n) + \mathrm{Im}^2(z_n) \leq 4\ \mbox{for}\ n\leq N,
$$

where $N$ is some maximum number of iterations. Points at which $z_n$ does not explode are given the value $N$ (and are seemingly in the set), and points where $z_n$ *does* explode are given the value $n$ at which $|z_n|^2 > 4$ (and are not in the set).

The code is by no means perfect, and I'll try to make it better and more rusty over time.
But it seems to work as intended. Take this plot for example, it's of the Mandelbrot set on the domain $[-1, 1] \times [-1, 1]$:

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/5ed26469-2e94-468c-8299-2a0dbb0143c1" />

And this is a plot where we graduallty zoom into a point (by a factor of 4 for each iteration):

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/e5907a2e-41de-43f3-a745-ab8fc71d30bc" />
