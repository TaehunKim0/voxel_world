use std::f32::consts::PI;
use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

pub fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    (a1 - a0) * w + a0
}

pub fn random_gradient(ix: u32, iy: u32) -> Vector2 {

    let w : u32 = 8 * 4;
    let s : u32 = w / 2;
    let mut a = ix;
    let mut b = iy;
    a = a.wrapping_mul(3284157443);
    b ^= a << s | a >> w-s;
    b = b.wrapping_mul(1911520717); 
    a ^= b << s | b >> w-s;
    a = a.wrapping_mul(2048419325);

    let random = a as f32 * (PI / 2147483648.0); // in [0, 2*Pi]
    Vector2{x : random.cos(), y : random.sin()}
}

pub fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    let gradient = random_gradient(ix as u32, iy as u32);
    let dx = x - ix as f32;
    let dy = y - iy as f32;
    dx * gradient.x + dy * gradient.y
}

pub fn perlin(x: f32, y: f32) -> f32 {
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    let n0 = dot_grid_gradient(x0, y0, x, y);
    let n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);

    let n0 = dot_grid_gradient(x0, y1, x, y);
    let n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);

    let result = interpolate(ix0, ix1, sy);

    result
}

pub fn perlin_noise2d(x: f32, y: f32, num_octaves: i32) -> f32 {
    let mut result = 0.0;
    let mut amplitude = 0.1;
    let mut frequency = 0.05;
    let persistence = 0.5; // Persistence value adjustment

    for _ in 0..num_octaves {
        let n = amplitude * perlin(x * frequency, y * frequency);
        result += n;
        amplitude *= persistence; // Amplitude decay
        frequency *= 2.0;
    }

    result = result.max(-1.0).min(1.0);
    result
}
