use clap::Parser;
use num::complex::Complex;

#[derive(Parser, Debug)]
#[command(name = "MendelBrot CLI")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "Mendelbrot cli drawing", long_about = None)]
struct Cli {
    #[arg(
        short = 'm',
        long = "iters",
        help = "max number of iterations",
        default_value = "1000"
    )]
    max_iters: usize,
    #[arg(short = 'x', long = "xmin", help = "x minimum", default_value = "-2.0")]
    x_min: f64,
    #[arg(short = 'z', long = "xmax", help = "x maximum", default_value = "2.0")]
    x_max: f64,
    #[arg(short = 'y', long = "ymin", help = "y minimum", default_value = "-1.0")]
    y_min: f64,
    #[arg(short = 't', long = "ymax", help = "y maximum", default_value = "1.0")]
    y_max: f64,
    #[arg(
        short = 'w',
        long = "width",
        help = "width of graph",
        default_value = "100"
    )]
    width: usize,
    #[arg(
        short = 'i',
        long = "height",
        help = "height of graph",
        default_value = "24"
    )]
    height: usize,
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent: f64 = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let args = Cli::parse();
    let mandelbrot = calculate_mandelbrot(
        args.max_iters,
        args.x_min,
        args.x_max,
        args.y_min,
        args.y_max,
        args.width,
        args.height,
    );
    render_mandelbrot(mandelbrot);
}
