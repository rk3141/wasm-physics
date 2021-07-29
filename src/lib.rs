extern crate rand;

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d};

mod body;
mod color;
mod vector2;

use body::Body;
use vector2::Vector2;

const HEIGHT: f64 = 400.0;
const WIDTH: f64 = 400.0;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn clear(ctx: &CanvasRenderingContext2d) {
    ctx.set_fill_style(&JsValue::from_str("#fff"));
    ctx.fill_rect(0.0, 0.0, 400.0, 400.0);
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut bodies = vec![];

    // for _ in 0..100 {
    //     let x: f64 = rand::random::<f64>() * WIDTH - 10.0;
    //     let y: f64 = rand::random::<f64>() * HEIGHT - 10.0;
    //     bodies.push(Body::new(Vector2 { x, y }));
    // }

    // let bodies = Arc::new(Mutex::new(bodies));

    let body1 = Arc::new(Mutex::new(Body::new(Vector2 {
        x: (rand::random::<f64>() * WIDTH - 10.0) / 2.0,
        y: (rand::random::<f64>() * HEIGHT - 10.0) / 2.0,
    })));

    let body2 = Arc::new(Mutex::new(Body::new(Vector2 {
        x: (rand::random::<f64>() * WIDTH - 10.0) / 2.0,
        y: (rand::random::<f64>() * HEIGHT - 10.0) / 2.0,
    })));

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        clear(&context);

        // let mut bodyvec = bodies.lock().unwrap();

        // for body in (*bodyvec).iter_mut() {
        //     body.apply_force(Vector2 { x: 0.0, y: 0.05 }); // gravity
        //     body.update();
        //     body.draw(&context);
        // }

        let mut body1 = body1.lock().unwrap();
        let mut body2 = body2.lock().unwrap();

        context.set_stroke_style(&JsValue::from_str("#fff"));
        context.move_to(body1.get_state().pos.x + 5.0, body1.get_state().pos.y + 5.0);
        context.line_to(body2.get_state().pos.x + 5.0, body2.get_state().pos.y + 5.0);
        context.stroke();

        let mut sp = (*body1).spring_force(&body2);

        (*body1).apply_force(sp);
        sp.mult_f64(-1.0);
        (*body2).apply_force(sp);

        (*body1).update();
        (*body2).update();

        (*body1).draw(&context);
        (*body2).draw(&context);

        // context.set_stroke_style(&JsValue::from_str("rgba(0,0,0, 0.1)"));
        // context.move_to(body1.get_state().pos.x + 5.0, body1.get_state().pos.y + 5.0);
        // context.line_to(body2.get_state().pos.x + 5.0, body2.get_state().pos.y + 5.0);
        // context.stroke();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
