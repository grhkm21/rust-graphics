mod consts;

use consts::*;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[wasm_bindgen]
pub struct House {
    pos_x: f64,
    pos_y: f64,
    dir_x: f64,
    dir_y: f64,
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl House {
    pub fn new(speed: f64) -> House {
        // TODO: abstract width and height, don't hardcode
        House {
            pos_x: 0.0,
            pos_y: 0.0,
            dir_x: speed,
            dir_y: speed,
            width: 200.0,
            height: 190.0,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.pos_x
    }

    pub fn get_y(&self) -> f64 {
        self.pos_y
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn tick(&mut self) {
        self.pos_x += self.dir_x;
        self.pos_y += self.dir_y;

        if self.pos_x < 0.0 || self.pos_x + self.width > WIDTH.into() {
            self.dir_x *= -1.0;
        }

        if self.pos_y < 0.0 || self.pos_y + self.height > HEIGHT.into() {
            self.dir_y *= -1.0;
        }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<CanvasRenderingContext2d, JsValue> {
    // reference https://github.com/tjcdeveloper/evolution-sim/blob/main/src/manager.rs
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let body = document.body().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas.set_attribute(&"id", &"canvas").unwrap();
    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    body.append_child(&canvas)?;

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Setup mouse down listener
    let handle_mousedown_closure =
        Closure::wrap(
            Box::new(move |evt: MouseEvent| handle_mousedown(evt.into())) as Box<dyn FnMut(_)>,
        );
    canvas.add_event_listener_with_callback(
        "mousedown",
        handle_mousedown_closure.as_ref().unchecked_ref(),
    )?;
    handle_mousedown_closure.forget();

    console::log_1(&JsValue::from_str("start()"));

    Ok(ctx)
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, pos_x: f64, pos_y: f64) {
    ctx.clear_rect(0.0, 0.0, WIDTH.into(), HEIGHT.into());

    ctx.set_fill_style(&"cyan".into());
    ctx.fill_rect(0.0, 0.0, WIDTH.into(), HEIGHT.into());

    ctx.set_fill_style(&"black".into());
    ctx.set_line_width(3.0);
    ctx.stroke_rect(25.0 + pos_x, 80.0 + pos_y, 150.0, 110.0);
    ctx.fill_rect(80.0 + pos_x, 130.0 + pos_y, 40.0, 60.0);
    ctx.begin_path();
    ctx.move_to(0.0 + pos_x, 80.0 + pos_y);
    ctx.line_to(100.0 + pos_x, 0.0 + pos_y);
    ctx.line_to(200.0 + pos_x, 80.0 + pos_y);
    ctx.close_path();
    ctx.stroke();

    // console::log_1(&JsValue::from_str(&format!("draw({})", x)));
}

fn handle_mousedown(event: MouseEvent) {
    let msg = format!(
        "Mouse down at position ({}, {})",
        event.client_x(),
        event.client_y()
    );
    console::log_1(&JsValue::from_str(&msg));
}
