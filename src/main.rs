mod engine;
mod frontend;
mod rendering;
mod types;

use crate::{
    rendering::webgpu::{self, create_shader_resource},
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
    let mut shader_map: std::collections::HashMap<
        std::string::String,
        rendering::webgpu::WebGPUShaderResource,
    > = std::collections::HashMap::new();

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

    // Javascript controls
    let control_response_js: Shared<frontend::eventlistener::ControlResponseJs> = std::rc::Rc::new(
        std::cell::RefCell::new(frontend::eventlistener::ControlResponseJs::default()),
    );
    frontend::eventlistener::add_event_listener_control(&control_response_js);

    // Frontend GUI
    frontend::gui::start_gui(&scene);

    // Rendering loop
    let f: Shared<Option<_>> = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g: Shared<Option<_>> = f.clone();
    *g.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        engine::scene::update_control(&scene, &control_response_js);

        let shading_type: engine::scene::ShadingType =
            scene.borrow().scene_variables.scene_shading_type;

        match shading_type {
            engine::scene::ShadingType::Forward => {
                let shader_resource: &webgpu::WebGPUShaderResource = &shader_map
                    .entry("Phong".to_string())
                    .or_insert(create_shader_resource(&webgpu_interface));

                for scene_object in scene.borrow().objects.iter() {
                    if scene_object.rendering_resource.is_some() {
                        rendering::webgpu::update_phong_shading(
                            &webgpu_interface,
                            &shader_resource,
                            &scene.clone(),
                            &scene_object,
                        );
                    }
                }

                rendering::webgpu::render_forward_shading_main(
                    &webgpu_interface,
                    &shader_resource,
                    &scene,
                );
            }
            engine::scene::ShadingType::Differed => {
                let differed_resource: rendering::webgpu::WebGPUDifferedResource =
                    rendering::webgpu::init_differed_pipeline(&webgpu_interface);
                rendering::webgpu::init_differed_gbuffer_pipeline(&webgpu_interface, &scene);

                rendering::webgpu::update_differed_shading(
                    &webgpu_interface,
                    &scene,
                    &differed_resource,
                );
                rendering::webgpu::render_differed_shading_main(
                    &webgpu_interface,
                    &scene,
                    &differed_resource,
                );
            }

            _ => {}
        }

        if scene.borrow().scene_variables.is_first_update {
            scene.borrow_mut().scene_variables.is_first_update = false;
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
