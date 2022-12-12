use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[wasm_bindgen]
pub struct Universe {
    val: u32
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        Universe {
            val: 0
        }
    }

    pub fn get_val(&self) -> u32 {
        self.val
    }

    pub fn set_val(&mut self, val: u32) {
        self.val = val;
    }

    pub fn inc_val(&mut self) {
        self.val += 1;
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
    canvas.set_width(500);
    canvas.set_height(500);
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
pub fn draw(ctx: &CanvasRenderingContext2d, x: f64) {
    ctx.clear_rect(0.0, 0.0, 500.0, 500.0);

    ctx.set_fill_style(&"cyan".into());
    ctx.fill_rect(0.0, 0.0, 500.0, 500.0);

    ctx.set_fill_style(&"black".into());
    ctx.set_line_width(10.0);
    ctx.stroke_rect(75.0 + x, 140.0, 150.0, 110.0);
    ctx.fill_rect(130.0 + x, 190.0, 40.0, 60.0);
    ctx.begin_path();
    ctx.move_to(50.0 + x, 140.0);
    ctx.line_to(150.0 + x, 60.0);
    ctx.line_to(250.0 + x, 140.0);
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
