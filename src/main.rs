mod engine;
mod rendering;
mod types;
mod web;

use crate::{
    rendering::webgpu::{self},
    types::Shared,
};
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen::prelude::wasm_bindgen(main)]
pub async fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    debug_log_with_time("Main");

    // Scene initialize
    let scene: engine::scene::Scene = engine::scene::Scene::new();
    let scene: Shared<engine::scene::Scene> = std::rc::Rc::new(std::cell::RefCell::new(scene));

    // Load .gltf data
    (scene.borrow_mut().objects, scene.borrow_mut().materials) =
        engine::load::load_gltf_scene(engine::define::GLTF_LOGO_PATH).await;

    // Batch objects
    //engine::scene::batch_objects(&scene);

    // Initialize rendering context
    let webgpu_interface: rendering::webgpu::WebGPUInterface =
        rendering::webgpu::init_interface().await;

    // Initialize rendering resources
    let object_num: usize = scene.borrow().objects.len();
    for i in 0..object_num {
        let is_mesh: bool = scene.borrow().objects.get(i).unwrap().source_mesh.is_some();
        if is_mesh {
            let rendering_resource: std::rc::Rc<
                std::cell::RefCell<webgpu::WebGPURenderingResource>,
            > = std::rc::Rc::new(std::cell::RefCell::new(
                rendering::webgpu::create_rendering_resource(
                    &webgpu_interface,
                    &scene
                        .borrow()
                        .objects
                        .get(i)
                        .unwrap()
                        .source_mesh
                        .as_ref()
                        .unwrap()
                        .borrow(),
                    &scene.borrow().materials,
                ),
            ));
            scene
                .borrow_mut()
                .objects
                .get_mut(i)
                .unwrap()
                .rendering_resource = Some(rendering_resource);
        }
    }

    // Shader resources
    let mut shader_map: std::collections::HashMap<
        std::string::String,
        rendering::webgpu::WebGPUShaderResource,
    > = std::collections::HashMap::new();

    // Global resource
    let mut global_resource_map: std::collections::HashMap<
        std::string::String,
        rendering::webgpu::WebGPURenderingResource,
    > = std::collections::HashMap::new();

    // Javascript controls
    let control_response_js: Shared<web::eventlistener::ControlResponseJs> = std::rc::Rc::new(
        std::cell::RefCell::new(web::eventlistener::ControlResponseJs::default()),
    );
    web::eventlistener::add_event_listener_control(&control_response_js);

    // Frontend GUI
    web::gui::start_gui(&scene);

    // Rendering loop
    let f: Shared<Option<_>> = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g: Shared<Option<_>> = f.clone();
    *g.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        engine::scene::update_control(&scene, &control_response_js);

        rendering::webgpu::update_rendering_main(
            &webgpu_interface,
            &scene,
            &mut shader_map,
            &mut global_resource_map,
        );

        if scene.borrow().variables.is_first_update {
            scene.borrow_mut().variables.is_first_update = false;
            debug_log_with_time("Render OK");
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    })
        as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    debug_log_with_time("Main end");
}

fn request_animation_frame(f: &wasm_bindgen::closure::Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn debug_log_with_time(message: &str) {
    let enable_log = true;

    if enable_log {
        log::debug!(
            "{:.2} ms : {}",
            web_sys::window()
                .expect("should have a Window")
                .performance()
                .expect("should have a Performance")
                .now(),
            message
        );
    }
}
