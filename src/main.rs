use core::f64;
use std::time::Instant;
const H: usize = 300;
const W: usize = 300;
const D_A: f64 = 1.0;
const D_B: f64 = 0.7;
const F: f64 = 0.005;
const K: f64 = 0.042983;
fn main() {
    let now = Instant::now();
    let mut a_concen: [f64; H * W] = [1.0; H * W];
    let mut b_concen: [f64; H * W] = [0.0; H * W];
    let mut a_buffer: [f64; H * W] = [0.0; H * W];
    let mut b_buffer: [f64; H * W] = [0.0; H * W];

    put_some_b_in(&mut b_concen);

    println!("Starting.");
    for t in 0..10000 {
        for j in 1..W - 1 {
            for i in 1..H - 1 {
                a_buffer[i + j * H] = (a_concen[i + j * H]
                    + (D_A * y3_laplace(i, j, &a_concen)
                        - a_concen[i + j * H] * b_concen[i + j * H] * b_concen[i + j * H]
                        + F * (1.0 - a_concen[i + j * H])))
                    .max(0.0)
                    .min(1.0);

                b_buffer[i + j * H] = (b_concen[i + j * H]
                    + (D_B * y3_laplace(i, j, &b_concen)
                        + a_concen[i + j * H] * b_concen[i + j * H] * b_concen[i + j * H]
                        - (K + F) * b_concen[i + j * H]))
                    .max(0.0)
                    .min(1.0);
            }
        }

        a_concen = a_buffer;
        b_concen = b_buffer;

        let mut imgbuf = image::ImageBuffer::new(H as u32, W as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let h = (256.0 * b_concen[x as usize + y as usize * H]) as u8;
            let k = (64.0 * a_concen[x as usize + y as usize * H]) as u8;

            *pixel = image::Rgb([h, 0, k]);
        }

        match imgbuf.save(format!("imgs/Frame{:0>6}.png", t)) {
            Ok(_) => {}
            Err(_) => std::fs::create_dir("./imgs").unwrap(),
        }
    }
    println!("{}", now.elapsed().as_millis());
}

fn put_some_b_in(grid: &mut [f64; H * W]) {
    for i in 0..W {
        for j in 0..H {
            if (i - 10).pow(2) + (j - 3).pow(2) < 100 {
                grid[i + j * H] = 1.0;
            }
        }
    }
    // for i in 0..W {
    //     for j in 0..H {
    //         if (i - 200).pow(2) + (j - 10).pow(2) < 100 {
    //             grid[i + j * H] = 1.0;
    //         }
    //     }
    // }
}

fn reflecting_laplace(x: usize, y: usize, grid: &[f64; H * W]) -> f64 {
    let x_lower = (x > 0) as u8; //this is a terrible hack
    let x_upper = (x < H - 1) as u8;
    let y_lower = (y > 0) as u8;
    let y_upper = (y < W - 1) as u8;
    let mut lap = 0.0;

    let deficit = 4 - x_lower - x_upper - y_lower - y_upper;
    if x > 0 {
        lap += grid[(x - 1) + H * y] * 0.25;
    }
    if x < W - 1 {
        lap += grid[(x + 1) + H * y] * 0.25;
    }
    if y > 0 {
        lap += grid[x + H * (y - 1)] * 0.25;
    }
    if y < H - 1 {
        lap += grid[x + H * (y + 1)] * 0.25;
    }
    lap += deficit as f64 * grid[x + H * y];

    lap -= grid[x + H * y];

    lap
}

fn y3_laplace(x: usize, y: usize, grid: &[f64; H * W]) -> f64 {
    let mut lap = 0.0;

    lap += grid[(x - 1) + H * y] * 0.2;
    lap += grid[(x - 1) + H * (y + 1)] * 0.05;
    lap += grid[(x - 1) + H * (y - 1)] * 0.05;

    lap += grid[(x + 1) + H * y] * 0.2;
    lap += grid[(x + 1) + H * (y - 1)] * 0.05;
    lap += grid[(x + 1) + H * (y + 1)] * 0.05;

    lap += grid[x + H * (y - 1)] * 0.2;

    lap += grid[x + H * (y + 1)] * 0.2;

    lap -= grid[x + H * y];

    lap
}
