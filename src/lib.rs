mod canvas;
mod boid;

use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn start() {
    let mut spawner = boid::Spawner::new();
    let boids = Rc::new(RefCell::new(spawner.spawn(1000)));

    let prev_time = Rc::new(RefCell::new(now()));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let curr_time = now();
        let dt = curr_time - *prev_time.borrow();

        {
            let mut boids = boids.borrow_mut();
            boids.update(dt);
            boids.draw();
        }

        *prev_time.borrow_mut() = curr_time;

        // Schedule next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
