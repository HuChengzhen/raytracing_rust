use minifb::{Key, Window, WindowOptions};
use nalgebra::{Vector2, Vector3};
use sphere::Sphere;

mod sphere;

type Color = Vector3<u8>;

const VIEWPORT_SIZE: f64 = 1.0;
const PROJECTION_PLANE_Z: f64 = 1.0;
const CAMERA_POSITION: Vector3<f64> = Vector3::new(0f64, 0f64, 0f64);

const BACKGROUND_COLOR: Vector3<u8> = Vector3::new(255, 255, 255);

const SPHERES: [Sphere; 3] = [Sphere {
    center: Vector3::new(0.0, -1.0, 3.0),
    radius: 1.0,
    color: Vector3::new(255, 0, 0),
}, Sphere {
    center: Vector3::new(2.0, 0.0, 4.0),
    radius: 1.0,
    color: Vector3::new(0, 0, 255),
}, Sphere {
    center: Vector3::new(-2.0, 0.0, 4.0),
    radius: 1.0,
    color: Vector3::new(0, 255, 0),
}];


fn canvas_to_viewport(point: Vector2<f64>) -> Vector3<f64> {
    return Vector3::new(
        point.x * VIEWPORT_SIZE / WIDTH as f64,
        point.y * VIEWPORT_SIZE / HEIGHT as f64,
        PROJECTION_PLANE_Z,
    );
}

fn intersect_ray_sphere(origin: Vector3<f64>, direction: Vector3<f64>, sphere: &Sphere) -> [f64; 2] {
    let oc = origin - sphere.center;

    let k1 = direction.dot(&direction);
    let k2 = 2.0 * oc.dot(&direction);
    let k3 = oc.dot(&oc) - sphere.radius * sphere.radius;

    let discriminant = k2 * k2 - 4.0 * k1 * k3;
    if discriminant < 0.0 {
        return [f64::INFINITY, f64::INFINITY];
    }

    let t1 = (-k2 + discriminant.sqrt()) / (2.0 * k1);
    let t2 = (-k2 - discriminant.sqrt()) / (2.0 * k1);
    [t1, t2]
}

fn trace_ray(origin: Vector3<f64>, direction: Vector3<f64>, min_t: f64, max_t: f64) -> Color {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in &SPHERES {
        let ts = intersect_ray_sphere(origin, direction, sphere);
        if ts[0] < closest_t && min_t < ts[0] && ts[0] < max_t {
            closest_t = ts[0];
            closest_sphere = Some(sphere);
        }
        if ts[1] < closest_t && min_t < ts[1] && ts[1] < max_t {
            closest_t = ts[1];
            closest_sphere = Some(sphere);
        }
    }

    if let Some(closest_sphere) = closest_sphere {
        return closest_sphere.color;
    }

    return BACKGROUND_COLOR;
}

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn put_pixel(x: i32, y: i32, color: Color, buffer: &mut Vec<u32>) {
    let x = (WIDTH / 2) as i32 + x;
    let y = (HEIGHT / 2) as i32 - y - 1;
    if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
        return;
    }

    buffer[y as usize * WIDTH + x as usize] = from_u8_rgb(color.x, color.y, color.z);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for i in -(WIDTH as i32) / 2..(WIDTH as i32) / 2 {
        for j in -(HEIGHT as i32) / 2..(HEIGHT as i32) / 2 {
            let direction = canvas_to_viewport(Vector2::new(i as f64, j as f64));
            let color = trace_ray(CAMERA_POSITION, direction, 1.0, f64::INFINITY);
            put_pixel(i, j, color, &mut buffer);
        }
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
