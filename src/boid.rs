use crate::canvas::Canvas;
use js_sys::Math::sqrt;
use rand::{rngs::ThreadRng, Rng};
use std::f64;

pub const SIZE: f64 = 10.0;
const STEP_SIZE: f64 = 0.2;
const TURN_RATE: f64 = 0.01; // Between 0 and 1
const VISION_RADIUS: f64 = 50.0;
const ALIGNMENT_FACTOR: f64 = 1.0;
const COHESION_FACTOR: f64 = 1.0;
const SEPARATION_FACTOR: f64 = 1.0;

pub struct Spawner {
    canvas: Canvas,
    rng: ThreadRng
}

impl Spawner {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let canvas = Canvas::get().unwrap();
        Spawner {
            canvas,
            rng
        }
    }

    fn rand_x(&mut self) -> f64 {
        let width = self.canvas.width() as f64;
        self.rng.gen_range(0.0..width)
    }

    fn rand_y(&mut self) -> f64 {
        let height = self.canvas.height() as f64;
        self.rng.gen_range(0.0..height)
    }

    fn rand_theta(&mut self) -> f64 {
        self.rng.gen_range(0.0..2.0 * f64::consts::PI)
    }

    pub fn spawn(&mut self, n: usize) -> Boids {
        let mut boids = Vec::with_capacity(n);

        for _ in 0..n {
            let x = self.rand_x();
            let y = self.rand_y();
            let theta = self.rand_theta();

            boids.push(Boid {
                x,
                y,
                theta,
            })
        }

        Boids::new(boids)
    }
}

pub struct Boid {
    x: f64,
    y: f64,
    theta: f64,
}

impl Boid {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    fn distance_to(&self, other: &Boid) -> f64 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();

        sqrt(dx * dx + dy * dy)
    }

    fn move_forward(&mut self, step_size: f64, canvas_width: f64, canvas_height: f64) {
        let dx = self.theta.sin() * step_size;
        let dy = - self.theta.cos() * step_size;

        self.x += dx;
        self.y += dy;

        self.x = self.x.rem_euclid(canvas_width);
        self.y = self.y.rem_euclid(canvas_height);
    }

    fn steer_toward(&mut self, theta_target: f64) {
        let d_theta = theta_target - self.theta;
        let d_theta = d_theta.rem_euclid(2.0 * f64::consts::PI);
        let d_theta = if d_theta > f64::consts::PI {
            - d_theta
        } else {
            d_theta
        };
        self.theta += d_theta * TURN_RATE;
        self.theta = self.theta.rem_euclid(2.0 * f64::consts::PI);
    }
}

pub struct Boids {
    boids: Vec<Boid>,
    canvas: Canvas
}

impl Boids {
    pub fn new(boids: Vec<Boid>) -> Self {
        let canvas = Canvas::get().unwrap();
        Boids {
            boids,
            canvas
        }
    }

    pub fn update(&mut self, dt: f64) {
        let step_size = STEP_SIZE * dt;
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        let theta_targets: Vec<f64> = self.boids.iter().map(|boid| {
            let others: Vec<&Boid> = self.boids.iter().filter(|other| boid.distance_to(other) < VISION_RADIUS).collect();
            let n = others.len() as f64;

            let mut theta_mean: f64 = 0.0;
            let mut x_mean: f64 = 0.0;
            let mut y_mean: f64 = 0.0;
            let mut separation_dx = 0.0;
            let mut separation_dy = 0.0;
            for other in others {
                theta_mean += other.theta;
                x_mean += other.x();
                y_mean += other.y();

                let distance = boid.distance_to(other);
                if distance > 0.0 {
                    separation_dx += (other.x() - boid.x()) / (distance * distance);
                    separation_dy += (other.y() - boid.y()) / (distance * distance)
                }
            }
            theta_mean /= n;
            x_mean /= n;
            y_mean /= n;

            let alignment_dir = theta_mean;
            let cohesion_dx = x_mean - boid.x;
            let cohesion_dy = y_mean - boid.y;
            let cohesion_dir = cohesion_dx.atan2(- cohesion_dy);
            let separation_dir = separation_dx.atan2(- cohesion_dy);

            (
                ALIGNMENT_FACTOR * alignment_dir +
                    COHESION_FACTOR * cohesion_dir +
                    SEPARATION_FACTOR * separation_dir
            ) / (
                ALIGNMENT_FACTOR +
                    COHESION_FACTOR +
                    SEPARATION_FACTOR
            )
        }).collect();

        for (boid, theta_target) in self.boids.iter_mut().zip(theta_targets) {
            boid.steer_toward(theta_target);
            boid.move_forward(step_size, width, height);
        }
    }

    pub fn draw(&self) {
        self.canvas.draw(&self.boids);
    }
}
