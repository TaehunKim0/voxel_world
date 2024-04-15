use rand::prelude::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Copy, Clone, Debug)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}


fn shuffle<T>(mut array: Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();
    array.shuffle(&mut rng);
    array
}


lazy_static! {
    static ref PERMUTATION: Vec<i32> = {
        let mut permutation: Vec<i32> = (0..256).collect();
        permutation = shuffle(permutation);
        let mut result = permutation.clone();
        result.extend(permutation);
        result
    };
}

fn get_gradient_vector2d(v: i32) -> Vector2 {
    let h = v & 3;
    match h {
        0 => Vector2::new(1.0, 1.0),
        1 => Vector2::new(-1.0, 1.0),
        2 => Vector2::new(-1.0, -1.0),
        _ => Vector2::new(1.0, -1.0),
    }
}

fn get_gradient_vector3d(v: i32) -> Vector3 {
    let h = v & 15;
    match h {
        0 | 1 | 4 | 11 => Vector3::new(1.0, 1.0, 0.0),
        2 | 3 | 5 | 10 => Vector3::new(-1.0, 1.0, 0.0),
        6 | 7 | 8 | 9 => Vector3::new(-1.0, -1.0, 0.0),
        _ => Vector3::new(1.0, -1.0, 0.0),
    }
}

fn fade(t: f32) -> f32 {
    ((6.0 * t - 15.0) * t + 10.0) * t * t * t
}

fn lerp(t: f32, a1: f32, a2: f32) -> f32 {
    a1 + t * (a2 - a1)
}


fn noise2d(x: f32, y: f32) -> f32 {
    let origin_x = x;
    let origin_y = y;
    let x = (x.floor() as i32) & 255;
    let y = (y.floor() as i32) & 255;

    let xf = origin_x as f32 - origin_x.floor();
    let yf = origin_y as f32 - origin_y.floor();

    let value_top_right = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + (y + 1) & 255) as usize];
    let value_top_left = PERMUTATION[(PERMUTATION[x as usize] + (y + 1) & 255) as usize];
    let value_bottom_right = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + y) as usize];
    let value_bottom_left = PERMUTATION[(PERMUTATION[x as usize] + y) as usize];

    let top_right = Vector2::new(xf - 1.0, yf - 1.0);
    let top_left = Vector2::new(xf, yf - 1.0);
    let bottom_right = Vector2::new(xf - 1.0, yf);
    let bottom_left = Vector2::new(xf, yf);

    let dot_top_right = top_right.dot(&get_gradient_vector2d(value_top_right));
    let dot_top_left = top_left.dot(&get_gradient_vector2d(value_top_left));
    let dot_bottom_right = bottom_right.dot(&get_gradient_vector2d(value_bottom_right));
    let dot_bottom_left = bottom_left.dot(&get_gradient_vector2d(value_bottom_left));

    let u = fade(xf);
    let v = fade(yf);

    lerp(
        v,
        lerp(u, dot_bottom_left, dot_bottom_right),
        lerp(u, dot_top_left, dot_top_right),
    )
}
fn noise3d(x: f32, y: f32, z: f32) -> f32 {
    let origin_x = x;
    let origin_y = y;
    let origin_z = z;
    
    let x = (x.floor() as i32) & 255;
    let y = (y.floor() as i32) & 255;
    let z = (z.floor() as i32) & 255;

    let xf = origin_x - origin_x.floor();
    let yf = origin_y - origin_y.floor();
    let zf = origin_z - origin_z.floor();

    let value_bottom_front_left = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + (y + 1) & 255) as usize];
    let value_bottom_front_right = PERMUTATION[(PERMUTATION[x as usize] + (y + 1) & 255) as usize];
    let value_bottom_back_left = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + y) as usize];
    let value_bottom_back_right = PERMUTATION[(PERMUTATION[x as usize] + y) as usize];
    let value_top_front_left = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + (y + 1) & 255) as usize];
    let value_top_front_right = PERMUTATION[(PERMUTATION[x as usize] + (y + 1) & 255) as usize];
    let value_top_back_left = PERMUTATION[(PERMUTATION[((x + 1) & 255) as usize] + y) as usize];
    let value_top_back_right = PERMUTATION[(PERMUTATION[x as usize] + y) as usize];

    let bottom_front_left = Vector3::new(xf - 1.0, yf - 1.0, zf - 1.0);
    let bottom_front_right = Vector3::new(xf, yf - 1.0, zf - 1.0);
    let bottom_back_left = Vector3::new(xf - 1.0, yf, zf - 1.0);
    let bottom_back_right = Vector3::new(xf, yf, zf - 1.0);
    let top_front_left = Vector3::new(xf - 1.0, yf - 1.0, zf);
    let top_front_right = Vector3::new(xf, yf - 1.0, zf);
    let top_back_left = Vector3::new(xf - 1.0, yf, zf);
    let top_back_right = Vector3::new(xf, yf, zf);

    let dot_bottom_front_left = bottom_front_left.dot(&get_gradient_vector3d(value_bottom_front_left));
    let dot_bottom_front_right = bottom_front_right.dot(&get_gradient_vector3d(value_bottom_front_right));
    let dot_bottom_back_left = bottom_back_left.dot(&get_gradient_vector3d(value_bottom_back_left));
    let dot_bottom_back_right = bottom_back_right.dot(&get_gradient_vector3d(value_bottom_back_right));
    let dot_top_front_left = top_front_left.dot(&get_gradient_vector3d(value_top_front_left));
    let dot_top_front_right = top_front_right.dot(&get_gradient_vector3d(value_top_front_right));
    let dot_top_back_left = top_back_left.dot(&get_gradient_vector3d(value_top_back_left));
    let dot_top_back_right = top_back_right.dot(&get_gradient_vector3d(value_top_back_right));

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    lerp(
        w,
        lerp(
            v,
            lerp(u, dot_bottom_back_left, dot_bottom_back_right),
            lerp(u, dot_bottom_front_left, dot_bottom_front_right),
        ),
        lerp(
            v,
            lerp(u, dot_top_back_left, dot_top_back_right),
            lerp(u, dot_top_front_left, dot_top_front_right),
        ),
    )
}

pub fn perlin_noise2d(x: f32, y: f32, num_octaves: i32) -> f32 {
    let mut result = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 0.005;
    let persistence = 0.5;

    for _ in 0..num_octaves {
        result += amplitude * noise2d(x * frequency, y * frequency);
        amplitude *= persistence;
        frequency *= 2.0;
    }

    result.clamp(-1.0, 1.0)
}

pub fn perlin_noise3d(x: f32, y: f32, z: f32, num_octaves: i32) -> f32 {
    let mut result = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 0.005;
    let persistence = 0.5;

    for _ in 0..num_octaves {
        result += amplitude * noise3d(x * frequency, y * frequency, z * frequency);
        amplitude *= persistence;
        frequency *= 2.0;
    }

    result.clamp(-1.0, 1.0)
}
