# mandelbrot-set-calculator
Rust/Python code using Maturin, that allows the user to calculate the Mandelbrot set for some chosen input. Mostly for fun, in order to learn how to pass heavy tasks to Rust from Python.

The code is by no means perfect, and I'll try to make it better and more rusty over time.
But it seems to work as intended. Take this plot for example, it's of the Mandelbrot set on the domain $[-1, 1] \times [-1, 1]$:

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/c4dbe70a-89d8-413b-9985-47b6ca947060" />

And this is a plot where we graduallty zoom into a point (by a factor of 4 for iteration):

<img width="1019" height="1019" alt="image" src="https://github.com/user-attachments/assets/8cb71599-c465-49e7-a941-0edce4c8377e" />
