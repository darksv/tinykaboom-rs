mod geometry;

use geometry::Vec3f;
use rayon::prelude::*;
use std::io::Write;
use std::ops::{Add, Mul, Sub};

const SPHERE_RADIUS: f32 = 1.5;
const NOISE_AMPLITUDE: f32 = 1.0;

fn lerp<T>(v0: T, v1: T, t: f32) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy,
{
    v0 + (v1 - v0) * t.min(1.).max(0.)
}

fn hash(n: f32) -> f32 {
    let x = n.sin() * 43758.5453;
    x - x.floor()
}

fn noise(x: Vec3f) -> f32 {
    let p = Vec3f::new(x.x.floor(), x.y.floor(), x.z.floor());
    let f = Vec3f::new(x.x - p.x, x.y - p.y, x.z - p.z);
    let f = f * (f * (Vec3f::new(3., 3., 3.) - f * 2.));
    let n = p * Vec3f::new(1., 57., 113.);

    lerp(
        lerp(
            lerp(hash(n + 0.), hash(n + 1.), f.x),
            lerp(hash(n + 57.), hash(n + 58.), f.x),
            f.y,
        ),
        lerp(
            lerp(hash(n + 113.), hash(n + 114.), f.x),
            lerp(hash(n + 170.), hash(n + 171.), f.x),
            f.y,
        ),
        f.z,
    )
}

fn rotate(v: Vec3f) -> Vec3f {
    Vec3f::new(
        Vec3f::new(0.00, 0.80, 0.60) * v,
        Vec3f::new(-0.80, 0.36, -0.48) * v,
        Vec3f::new(-0.60, -0.48, 0.64) * v,
    )
}

fn fractal_brownian_motion(v: Vec3f) -> f32 {
    let mut p = rotate(v);
    let mut f = 0.0;
    f += 0.5000 * noise(p);
    p = p * 2.32;
    f += 0.2500 * noise(p);
    p = p * 3.03;
    f += 0.1250 * noise(p);
    p = p * 2.61;
    f += 0.0625 * noise(p);

    f / 0.9375
}

fn palette_fire(d: f32) -> Vec3f {
    let yellow = Vec3f::new(1.7, 1.3, 1.0);
    let orange = Vec3f::new(1.0, 0.6, 0.0);
    let red = Vec3f::new(1.0, 0.0, 0.0);
    let darkgray = Vec3f::new(0.2, 0.2, 0.2);
    let gray = Vec3f::new(0.4, 0.4, 0.4);

    let d = d.min(1.).max(0.);
    if d < 0.25 {
        lerp(gray, darkgray, d * 4.)
    } else if d < 0.5 {
        lerp(darkgray, red, d * 4. - 1.)
    } else if d < 0.75 {
        lerp(red, orange, d * 4. - 2.)
    } else {
        lerp(orange, yellow, d * 4. - 3.)
    }
}

fn signed_distance(p: Vec3f) -> f32 {
    let displacement = -fractal_brownian_motion(p * 3.4) * NOISE_AMPLITUDE;
    return p.norm() - (SPHERE_RADIUS + displacement);
}

fn sphere_trace(origin: Vec3f, dir: Vec3f) -> Option<Vec3f> {
    if origin * origin - (origin * dir).powf(2.) > SPHERE_RADIUS.powf(2.) {
        return None;
    }

    let mut pos = origin;
    for _ in 0..128 {
        let d = signed_distance(pos);
        if d < 0. {
            return Some(pos);
        }

        pos = pos + dir * (d * 0.1).max(0.01);
    }
    None
}

fn distance_field_normal(pos: Vec3f) -> Vec3f {
    let eps = 0.1;
    let d = signed_distance(pos);
    let nx = signed_distance(pos + Vec3f::new(eps, 0., 0.)) - d;
    let ny = signed_distance(pos + Vec3f::new(0., eps, 0.)) - d;
    let nz = signed_distance(pos + Vec3f::new(0., 0., eps)) - d;
    Vec3f::new(nx, ny, nz).normalize()
}

fn main() {
    let width = 640;
    let height = 480;
    let fov = std::f32::consts::PI / 3.;

    let mut framebuffer = vec![Vec3f::new(0.0, 0.0, 0.0); width * height];
    framebuffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(j, line)| {
            for (i, pixel) in line.iter_mut().enumerate() {
                let light_pos = Vec3f::new(10., 10., 10.);
                let camera_pos = Vec3f::new(0., 0., 3.);
                let background_color = Vec3f::new(0.2, 0.7, 0.8);

                let dir_x = (i as f32 + 0.5) - width as f32 / 2.;
                let dir_y = -(j as f32 + 0.5) + height as f32 / 2.;
                let dir_z = -(height as f32) / (2. * (fov / 2.).tan());

                if let Some(hit) =
                    sphere_trace(camera_pos, Vec3f::new(dir_x, dir_y, dir_z).normalize())
                {
                    let noise_level = (SPHERE_RADIUS - hit.norm()) / NOISE_AMPLITUDE;
                    let light_dir = (light_pos - hit).normalize();
                    let light_intensity = (light_dir * distance_field_normal(hit)).max(0.4);

                    *pixel = palette_fire((-0.2 + noise_level) * 2.) * light_intensity;
                } else {
                    *pixel = background_color;
                }
            }
        });

    let file = std::fs::File::create("./out.ppm").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    write!(writer, "P6\n{} {}\n255\n", width, height).unwrap();
    for pixel in framebuffer {
        let p = 255. * pixel;
        let x = (p.x as i32).min(255).max(0) as u8;
        let y = (p.y as i32).min(255).max(0) as u8;
        let z = (p.z as i32).min(255).max(0) as u8;
        writer.write_all(&[x, y, z]).unwrap();
    }
}
