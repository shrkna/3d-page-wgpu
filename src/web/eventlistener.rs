use crate::engine::define;
use crate::types::Shared;

use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Default)]
pub struct ControlResponseJs {
    pub movement_x: i32,
    pub movement_y: i32,
    pub on_click: bool,
    pub wheel_delta_y: f64,
    pub on_wheel: bool,
    pub on_shift: bool,
}

pub fn add_event_listener_control(event_response: &Shared<ControlResponseJs>) {
    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let response_clone_mouse: Shared<ControlResponseJs> = event_response.clone();

    let mouse_move_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
        wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut borrowed = response_clone_mouse.borrow_mut();

            borrowed.movement_x = event.movement_x();
            borrowed.movement_y = event.movement_y();
            borrowed.on_click = event.which() == 1;
        }) as Box<dyn FnMut(_)>);

    let response_clone_wheel: Shared<ControlResponseJs> = event_response.clone();

    let mouse_wheel_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
        wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            let mut borrowed = response_clone_wheel.borrow_mut();

            borrowed.on_wheel = true;
            borrowed.wheel_delta_y = event.delta_y();
        }) as Box<dyn FnMut(_)>);

    let response_clone_key_down: Shared<ControlResponseJs> = event_response.clone();

    let key_down_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
        wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut borrowed = response_clone_key_down.borrow_mut();

            borrowed.on_shift = event.shift_key();
        }) as Box<dyn FnMut(_)>);

    let response_clone_key_up: Shared<ControlResponseJs> = event_response.clone();

    let key_up_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
        wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::KeyboardEvent| {
            let mut borrowed = response_clone_key_up.borrow_mut();

            borrowed.on_shift = false;
        }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_closure.forget();

    canvas
        .add_event_listener_with_callback("wheel", mouse_wheel_closure.as_ref().unchecked_ref())
        .unwrap();
    mouse_wheel_closure.forget();

    canvas
        .add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref())
        .unwrap();
    key_down_closure.forget();

    canvas
        .add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
        .unwrap();
    key_up_closure.forget();
}
