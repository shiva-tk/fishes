use wasm_bindgen::prelude::*;
use crate::boid;
use boid::Boid;

const CANVAS_ID: &str = "canvas";

pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn get() -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let canvas = document.get_element_by_id(CANVAS_ID)?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().ok()?;
        let context = canvas.get_context("2d").ok()??;
        let context: web_sys::CanvasRenderingContext2d = context.dyn_into().ok()?;
        Some(Canvas {
            canvas,
            context,
        })
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn draw(&self, boids: &Vec<Boid>) {
        self.context.reset();
        for boid in boids {
            self.draw_boid(boid);
        }
    }

    fn draw_boid(&self, boid: &Boid) {
        let context = &self.context;
        context.begin_path();

        let x = boid.x();
        let y = boid.y();
        let theta = boid.theta();

        context.translate(x, y).unwrap();
        context.rotate(theta).unwrap();

        context.move_to(0.0, - boid::SIZE);
        context.line_to(- boid::SIZE / 2.0, boid::SIZE / 2.0);
        context.line_to(boid::SIZE / 2.0, boid::SIZE / 2.0);
        context.close_path();

        context.stroke();
        // Reset the transformation matrix (so the next drawing is not affected)
        context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    }
}
