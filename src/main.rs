use core::f64;
use rand::Rng;
use std::time::Instant;
const H: usize = 500;
const W: usize = 500;
const D_A: f64 = 1.0;
const D_B: f64 = 0.5;
const F_MIN: f64 = 0.02;
const F_MAX: f64 = 0.025;
const K_MIN: f64 = 0.045;
const K_MAX: f64 = 0.06;
const D_T: f64 = 1.0;
const FRAMES: usize = 30000;
fn main() {
    let now = Instant::now();
    let mut a_concen: [f64; H * W] = [1.0; H * W];
    let mut b_concen: [f64; H * W] = [0.0; H * W];
    let mut a_buffer: [f64; H * W] = [0.0; H * W];
    let mut b_buffer: [f64; H * W] = [0.0; H * W];

    random_seed_b(&mut b_concen);

    println!("Starting.");
    for t in 0..FRAMES {
        for j in 1..W - 1 {
            for i in 1..H - 1 {
                a_buffer[i + j * H] = (a_concen[i + j * H]
                    + (D_A * three_by_three_laplacian(i, j, &a_concen)
                        - a_concen[i + j * H] * b_concen[i + j * H] * b_concen[i + j * H]
                        + lerp(F_MIN, F_MAX, i, H) * (1.0 - a_concen[i + j * H]) * D_T))
                    .max(0.0)
                    .min(1.0);

                b_buffer[i + j * H] = (b_concen[i + j * H]
                    + (D_B * three_by_three_laplacian(i, j, &b_concen)
                        + a_concen[i + j * H] * b_concen[i + j * H] * b_concen[i + j * H]
                        - (lerp(
                            lerp(K_MIN, (5.0 * K_MAX + 5.0 * K_MIN) * 0.1, i, H),
                            K_MAX,
                            j,
                            W,
                        ) + lerp(F_MIN, F_MAX, i, H))
                            * b_concen[i + j * H])
                        * D_T)
                    .max(0.0)
                    .min(1.0);
            }
        }

        a_concen = a_buffer;
        b_concen = b_buffer;

        let mut imgbuf = image::ImageBuffer::new(H as u32, W as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let h = b_concen[x as usize + y as usize * H];
            let k = a_concen[x as usize + y as usize * H];

            let l = ((h - k) * (h - k) < 0.01) as u8;
            let l_d = (((h - k) + 0.1) * 5.0 * 255.0) as u8;

            /*
            Here's the regular branched logic equivalent of the above two lines:
            if |h-k| <0.1 {
            l = ((h-k) + 0.1) * 5 * 255.0
                ^^^^^^^^^^^^^^^^^
                rescaled to betw.
                0.0 and 1.0
            } else if h > k {
             l = 255
            } else {
             l = 0
            }
            This cranks down the dynamic range to give that nice blown-out look.
            */

            let l = 255 * (1 - l) * (h > k) as u8 + (l * l_d);
            *pixel = image::Luma([l]);
        }

        match imgbuf.save(format!("imgs/Frame{:0>6}.png", t)) {
            Ok(_) => {}
            Err(_) => std::fs::create_dir("./imgs").unwrap(),
        }
    }
    println!("{}", now.elapsed().as_millis());
}

fn lerp(min: f64, max: f64, step: usize, steps: usize) -> f64 {
    min + (max - min) * (step as f64 / steps as f64)
}

fn random_seed_b(grid: &mut [f64; H * W]) {
    let mut rng = rand::thread_rng();

    for i in 1..W - 1 {
        for j in 1..H - 1 {
            grid[i + j * H] = rng.gen_range(0.0..0.145);
        }
    }
}

fn three_by_three_laplacian(x: usize, y: usize, grid: &[f64; H * W]) -> f64 {
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
