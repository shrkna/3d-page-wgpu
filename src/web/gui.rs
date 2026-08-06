use crate::engine;
use crate::types::Shared;
use wasm_bindgen::JsCast;

// Initialize frontend GUI

pub fn create_frontend_gui(scene: &Shared<engine::scene::Scene>) {
    create_debug_dialog(scene);
}

fn create_debug_dialog(scene: &Shared<engine::scene::Scene>) {
    let body: web_sys::HtmlElement = gloo::utils::body();

    let dialog_wrapper: web_sys::Element = gloo::utils::document().create_element("div").unwrap();
    dialog_wrapper.set_id("dialog-wrapper");

    // Environment dialog
    create_debug_dialog_environment(&dialog_wrapper, &scene);

    // Base pass dialog
    create_debug_dialog_base_pass(&dialog_wrapper, scene);

    // Sky box dialog
    create_debug_dialog_sky_box(&dialog_wrapper, scene);

    // Postprocess dialog
    create_debug_dialog_postprocess(&dialog_wrapper, &scene);

    // Overlay dialog
    create_debug_dialog_overlay(&dialog_wrapper, &scene);

    // Statistics dialog
    create_debug_dialog_statistics(&dialog_wrapper, &scene);

    body.append_child(&dialog_wrapper).unwrap();
}

// Create and append debug dialog

fn create_debug_dialog_environment(
    parent: &web_sys::Element,
    scene: &Shared<engine::scene::Scene>,
) {
    let scene_value = scene.borrow();

    let environment_dialog = gloo::utils::document().create_element("div").unwrap();
    environment_dialog.set_id("dialog-element-environment");
    environment_dialog.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-environment");
    //accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Environment"));
    accordion_label_element
        .set_attribute("for", "accordion-environment")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // directional light
    {
        let directional_accordion_input_element =
            gloo::utils::document().create_element("input").unwrap();
        let directional_accordion_input_element: web_sys::HtmlInputElement =
            directional_accordion_input_element.dyn_into().unwrap();
        directional_accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        directional_accordion_input_element.set_class_name("accordion-input");
        directional_accordion_input_element.set_id("accordion-directional");
        //directional_accordion_input_element.set_checked(true);

        let directional_accordion_label_element =
            gloo::utils::document().create_element("label").unwrap();
        directional_accordion_label_element.set_class_name("accordion-label inner-accordion-label");
        directional_accordion_label_element.set_text_content(Some("Directional Light"));
        directional_accordion_label_element
            .set_attribute("for", "accordion-directional")
            .unwrap();

        let directional_accordion_content_element =
            gloo::utils::document().create_element("div").unwrap();
        directional_accordion_content_element
            .set_class_name("accordion-content inner-accordion-content");

        // X
        {
            let directional_x_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_x_element.set_class_name("widget-row");

            let directional_x_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_x_label_element.set_class_name("widget-label");
            directional_x_label_element.set_text_content(Some("X"));

            let directional_x_content_element =
                gloo::utils::document().create_element("div").unwrap();
            directional_x_content_element.set_class_name("widget-value");

            {
                let directional_x_input_range: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let directional_x_input_range: web_sys::HtmlInputElement =
                    directional_x_input_range.dyn_into().unwrap();
                directional_x_input_range.set_id("directional-range-x");
                directional_x_input_range.set_class_name("range-element");
                directional_x_input_range
                    .set_attribute("type", "range")
                    .unwrap();
                directional_x_input_range
                    .set_attribute("min", "-1.0")
                    .unwrap();
                directional_x_input_range
                    .set_attribute("max", "1.0")
                    .unwrap();
                directional_x_input_range
                    .set_attribute("step", "0.01")
                    .unwrap();
                directional_x_input_range.set_value(
                    scene_value.parameters.light_parameters.directional_light_angle[0]
                        .to_string()
                        .as_str(),
                );

                let directional_x_input_range_text: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                directional_x_input_range_text.set_id("directional-range-x-text");
                directional_x_input_range_text.set_class_name("range-text-element");
                directional_x_input_range_text.set_text_content(Some(
                    scene_value.parameters.light_parameters.directional_light_angle[0]
                        .to_string()
                        .as_str(),
                ));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let directional_range_x_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let range_x_element: web_sys::Element = gloo::utils::document()
                                    .get_element_by_id("directional-range-x")
                                    .unwrap();
                                let range_x_element: web_sys::HtmlInputElement =
                                    range_x_element.dyn_into().unwrap();
                                let value: String = range_x_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.light_parameters.directional_light_angle[0] =
                                    value.parse::<f32>().unwrap();

                                let range_x_text_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("directional-range-x-text")
                                        .unwrap();
                                range_x_text_element.set_text_content(Some(&value));
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    directional_x_input_range
                        .add_event_listener_with_callback(
                            "input",
                            directional_range_x_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    directional_range_x_closure.forget();
                }

                directional_x_content_element
                    .append_child(&directional_x_input_range)
                    .unwrap();
                directional_x_content_element
                    .append_child(&directional_x_input_range_text)
                    .unwrap();
            }

            directional_x_element
                .append_child(&directional_x_label_element)
                .unwrap();
            directional_x_element
                .append_child(&directional_x_content_element)
                .unwrap();

            directional_accordion_content_element
                .append_child(&directional_x_element)
                .unwrap();
        }
        // Y
        {
            let directional_y_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_y_element.set_class_name("widget-row");

            let directional_y_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_y_label_element.set_class_name("widget-label");
            directional_y_label_element.set_text_content(Some("Y"));

            let directional_y_content_element =
                gloo::utils::document().create_element("div").unwrap();
            directional_y_content_element.set_class_name("widget-value");

            {
                let directional_y_input_range: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let directional_y_input_range: web_sys::HtmlInputElement =
                    directional_y_input_range.dyn_into().unwrap();
                directional_y_input_range.set_id("directional-range-y");
                directional_y_input_range.set_class_name("range-element");
                directional_y_input_range
                    .set_attribute("type", "range")
                    .unwrap();
                directional_y_input_range
                    .set_attribute("min", "-1.0")
                    .unwrap();
                directional_y_input_range
                    .set_attribute("max", "1.0")
                    .unwrap();
                directional_y_input_range
                    .set_attribute("step", "0.01")
                    .unwrap();
                directional_y_input_range.set_value(
                    scene_value.parameters.light_parameters.directional_light_angle[1]
                        .to_string()
                        .as_str(),
                );

                let directional_y_input_range_text: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                directional_y_input_range_text.set_id("directional-range-y-text");
                directional_y_input_range_text.set_class_name("range-text-element");
                directional_y_input_range_text.set_text_content(Some(
                    scene_value.parameters.light_parameters.directional_light_angle[1]
                        .to_string()
                        .as_str(),
                ));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let directional_range_y_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let range_x_element: web_sys::Element = gloo::utils::document()
                                    .get_element_by_id("directional-range-y")
                                    .unwrap();
                                let range_y_element: web_sys::HtmlInputElement =
                                    range_x_element.dyn_into().unwrap();
                                let value: String = range_y_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.light_parameters.directional_light_angle[1] =
                                    value.parse::<f32>().unwrap();

                                let range_y_text_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("directional-range-y-text")
                                        .unwrap();
                                range_y_text_element.set_text_content(Some(&value));
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    directional_y_input_range
                        .add_event_listener_with_callback(
                            "input",
                            directional_range_y_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    directional_range_y_closure.forget();
                }

                directional_y_content_element
                    .append_child(&directional_y_input_range)
                    .unwrap();
                directional_y_content_element
                    .append_child(&directional_y_input_range_text)
                    .unwrap();
            }
            directional_y_element
                .append_child(&directional_y_label_element)
                .unwrap();
            directional_y_element
                .append_child(&directional_y_content_element)
                .unwrap();

            directional_accordion_content_element
                .append_child(&directional_y_element)
                .unwrap();
        }
        // Z
        {
            let directional_z_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_z_element.set_class_name("widget-row");

            let directional_z_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_z_label_element.set_class_name("widget-label");
            directional_z_label_element.set_text_content(Some("Z"));

            let directional_z_content_element =
                gloo::utils::document().create_element("div").unwrap();
            directional_z_content_element.set_class_name("widget-value");

            {
                let directional_z_input_range: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let directional_z_input_range: web_sys::HtmlInputElement =
                    directional_z_input_range.dyn_into().unwrap();
                directional_z_input_range.set_id("directional-range-z");
                directional_z_input_range.set_class_name("range-element");
                directional_z_input_range
                    .set_attribute("type", "range")
                    .unwrap();
                directional_z_input_range
                    .set_attribute("min", "-1.0")
                    .unwrap();
                directional_z_input_range
                    .set_attribute("max", "1.0")
                    .unwrap();
                directional_z_input_range
                    .set_attribute("step", "0.01")
                    .unwrap();
                directional_z_input_range.set_value(
                    scene_value.parameters.light_parameters.directional_light_angle[2]
                        .to_string()
                        .as_str(),
                );

                let directional_z_input_range_text: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                directional_z_input_range_text.set_id("directional-range-z-text");
                directional_z_input_range_text.set_class_name("range-text-element");
                directional_z_input_range_text.set_text_content(Some(
                    scene_value.parameters.light_parameters.directional_light_angle[2]
                        .to_string()
                        .as_str(),
                ));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let directional_range_z_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let range_z_element: web_sys::Element = gloo::utils::document()
                                    .get_element_by_id("directional-range-z")
                                    .unwrap();
                                let range_z_element: web_sys::HtmlInputElement =
                                    range_z_element.dyn_into().unwrap();
                                let value: String = range_z_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.light_parameters.directional_light_angle[2] =
                                    value.parse::<f32>().unwrap();

                                let range_z_text_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("directional-range-z-text")
                                        .unwrap();
                                range_z_text_element.set_text_content(Some(&value));
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    directional_z_input_range
                        .add_event_listener_with_callback(
                            "input",
                            directional_range_z_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    directional_range_z_closure.forget();
                }

                directional_z_content_element
                    .append_child(&directional_z_input_range)
                    .unwrap();
                directional_z_content_element
                    .append_child(&directional_z_input_range_text)
                    .unwrap();
            }
            directional_z_element
                .append_child(&directional_z_label_element)
                .unwrap();
            directional_z_element
                .append_child(&directional_z_content_element)
                .unwrap();

            directional_accordion_content_element
                .append_child(&directional_z_element)
                .unwrap();
        }

        // Intensity
        {
            let directional_intensity_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_intensity_element.set_class_name("widget-row");

            let directional_intensity_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            directional_intensity_label_element.set_class_name("widget-label");
            directional_intensity_label_element.set_text_content(Some("Intensity"));

            let directional_intensity_content_element =
                gloo::utils::document().create_element("div").unwrap();
            directional_intensity_content_element.set_class_name("widget-value");

            {
                let directional_intensity_input_range: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let directional_intensity_input_range: web_sys::HtmlInputElement =
                    directional_intensity_input_range.dyn_into().unwrap();
                directional_intensity_input_range.set_id("directional-range-intensity");
                directional_intensity_input_range.set_class_name("range-element");
                directional_intensity_input_range
                    .set_attribute("type", "range")
                    .unwrap();
                directional_intensity_input_range
                    .set_attribute("min", "0.0")
                    .unwrap();
                directional_intensity_input_range
                    .set_attribute("max", "10.0")
                    .unwrap();
                directional_intensity_input_range
                    .set_attribute("step", "0.01")
                    .unwrap();
                directional_intensity_input_range.set_value(
                    scene_value.parameters.light_parameters.directional_light_intensity
                        .to_string()
                        .as_str(),
                );

                let directional_intensity_input_range_text: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                directional_intensity_input_range_text.set_id("directional-range-intensity-text");
                directional_intensity_input_range_text.set_class_name("range-text-element");
                directional_intensity_input_range_text.set_text_content(Some(
                    scene_value.parameters.light_parameters.directional_light_intensity
                        .to_string()
                        .as_str(),
                ));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let directional_range_intensity_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let range_intensity_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("directional-range-intensity")
                                        .unwrap();
                                let range_intensity_element: web_sys::HtmlInputElement =
                                    range_intensity_element.dyn_into().unwrap();
                                let value: String = range_intensity_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.light_parameters.directional_light_intensity =
                                    value.parse::<f32>().unwrap();

                                let range_intensity_text_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("directional-range-intensity-text")
                                        .unwrap();
                                range_intensity_text_element.set_text_content(Some(&value));
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    directional_intensity_input_range
                        .add_event_listener_with_callback(
                            "input",
                            directional_range_intensity_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    directional_range_intensity_closure.forget();
                }

                directional_intensity_content_element
                    .append_child(&directional_intensity_input_range)
                    .unwrap();
                directional_intensity_content_element
                    .append_child(&directional_intensity_input_range_text)
                    .unwrap();
            }
            directional_intensity_element
                .append_child(&directional_intensity_label_element)
                .unwrap();
            directional_intensity_element
                .append_child(&directional_intensity_content_element)
                .unwrap();

            directional_accordion_content_element
                .append_child(&directional_intensity_element)
                .unwrap();
        }

        accordion_content_element
            .append_child(&directional_accordion_input_element)
            .unwrap();
        accordion_content_element
            .append_child(&directional_accordion_label_element)
            .unwrap();
        accordion_content_element
            .append_child(&directional_accordion_content_element)
            .unwrap();
    }

    environment_dialog
        .append_child(&accordion_input_element)
        .unwrap();
    environment_dialog
        .append_child(&accordion_label_element)
        .unwrap();
    environment_dialog
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&environment_dialog).unwrap();
}

fn create_debug_dialog_base_pass(parent: &web_sys::Element, scene: &Shared<engine::scene::Scene>) {
    let scene_value = scene.borrow();

    let dialog_base_pass = gloo::utils::document().create_element("div").unwrap();
    dialog_base_pass.set_id("dialog-element-basepass");
    dialog_base_pass.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-basepass");
    //accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Base pass"));
    accordion_label_element
        .set_attribute("for", "accordion-basepass")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // rendering type
    {
        let render_type_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        render_type_element.set_class_name("widget-row");

        let render_type_label_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        render_type_label_element.set_class_name("widget-label");
        render_type_label_element.set_text_content(Some("Rendering type"));

        let render_type_select_element = gloo::utils::document().create_element("select").unwrap();
        render_type_select_element.set_class_name("select-element");
        render_type_select_element.set_id("render-type-select");

        let render_type_option_forward = gloo::utils::document().create_element("option").unwrap();
        render_type_option_forward.set_text_content(Some("forward"));
        let render_type_option_differed = gloo::utils::document().create_element("option").unwrap();
        render_type_option_differed.set_text_content(Some("differed"));

        match &scene_value.parameters.scene_shading_type {
            engine::scene::ShadingType::Forward => {
                render_type_option_forward.set_attribute("selected", "")
            }
            engine::scene::ShadingType::Differed => {
                render_type_option_differed.set_attribute("selected", "")
            }
        }
        .unwrap();

        {
            let scene_clone: Shared<engine::scene::Scene> = scene.clone();

            let render_type_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let render_type_element: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("render-type-select")
                        .unwrap();
                    let render_type_element: web_sys::HtmlSelectElement =
                        render_type_element.dyn_into().unwrap();
                    let value: String = render_type_element.value();

                    let forward_wrapper: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("forward-wrapper")
                        .unwrap();
                    let forward_wrapper: web_sys::HtmlElement = forward_wrapper.dyn_into().unwrap();

                    let differed_wrapper: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("differed-wrapper")
                        .unwrap();
                    let differed_wrapper: web_sys::HtmlElement =
                        differed_wrapper.dyn_into().unwrap();

                    let mut scene_value = scene_clone.borrow_mut();
                    match value.as_str() {
                        "forward" => {
                            scene_value.parameters.scene_shading_type =
                                engine::scene::ShadingType::Forward;

                            forward_wrapper.set_class_name("widget-wrapper");
                            differed_wrapper.set_class_name("widget-wrapper widget-wrapper-hidden");
                        }
                        "differed" => {
                            scene_value.parameters.scene_shading_type =
                                engine::scene::ShadingType::Differed;

                            forward_wrapper.set_class_name("widget-wrapper widget-wrapper-hidden");
                            differed_wrapper.set_class_name("widget-wrapper");
                        }
                        _ => {}
                    }
                }) as Box<dyn FnMut(_)>);

            render_type_select_element
                .add_event_listener_with_callback(
                    "change",
                    render_type_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            render_type_closure.forget();
        }

        render_type_select_element
            .append_child(&render_type_option_forward)
            .unwrap();
        render_type_select_element
            .append_child(&render_type_option_differed)
            .unwrap();

        render_type_element
            .append_child(&render_type_label_element)
            .unwrap();
        render_type_element
            .append_child(&render_type_select_element)
            .unwrap();

        accordion_content_element
            .append_child(&render_type_element)
            .unwrap();
    }

    // forward wrapper
    {
        let forward_wrapper = gloo::utils::document().create_element("div").unwrap();
        forward_wrapper.set_id("forward-wrapper");
        forward_wrapper.set_class_name("widget-wrapper");
        if scene.borrow().parameters.scene_shading_type != engine::scene::ShadingType::Forward {
            forward_wrapper.set_class_name("widget-wrapper-hidden");
        }

        // shader type
        {
            let shader_type_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            shader_type_element.set_class_name("widget-row");

            let shader_type_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            shader_type_label_element.set_class_name("widget-label");
            shader_type_label_element.set_text_content(Some("shading"));

            let shader_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            shader_type_select_element.set_class_name("select-element");
            shader_type_select_element.set_id("forward-type-select");

            let shader_type_option_phong =
                gloo::utils::document().create_element("option").unwrap();
            shader_type_option_phong.set_text_content(Some("phong"));

            /*
            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                let buffer_type_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let buffer_type_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("buffer-type-select")
                                .unwrap();
                            let buffer_type_element: web_sys::HtmlSelectElement =
                                buffer_type_element.dyn_into().unwrap();
                            let value: String = buffer_type_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "render" => scene_value.variables.differed_debug_type = 0,
                                "normal" => scene_value.variables.differed_debug_type = 1,
                                "depth" => scene_value.variables.differed_debug_type = 2,
                                "albedo" => scene_value.variables.differed_debug_type = 3,
                                "metallic" => scene_value.variables.differed_debug_type = 4,
                                _ => scene_value.variables.differed_debug_type = 0,
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>);

                shader_type_select_element
                    .add_event_listener_with_callback(
                        "change",
                        buffer_type_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                buffer_type_closure.forget();
            }*/

            shader_type_select_element
                .append_child(&shader_type_option_phong)
                .unwrap();

            shader_type_element
                .append_child(&shader_type_label_element)
                .unwrap();
            shader_type_element
                .append_child(&shader_type_select_element)
                .unwrap();

            forward_wrapper.append_child(&shader_type_element).unwrap();
        }

        // display
        {
            let forward_display_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            forward_display_element.set_class_name("widget-row");

            let forward_display_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            forward_display_label_element.set_class_name("widget-label");
            forward_display_label_element.set_text_content(Some("out"));

            let forward_display_select_element =
                gloo::utils::document().create_element("select").unwrap();
            forward_display_select_element.set_class_name("select-element");
            forward_display_select_element.set_id("forward-display-select");

            let forward_display_option_render =
                gloo::utils::document().create_element("option").unwrap();
            forward_display_option_render.set_text_content(Some("render"));
            let forward_display_option_normal =
                gloo::utils::document().create_element("option").unwrap();
            forward_display_option_normal.set_text_content(Some("normal"));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                let forward_display_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let forward_display_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("forward-display-select")
                                .unwrap();
                            let forward_display_element: web_sys::HtmlSelectElement =
                                forward_display_element.dyn_into().unwrap();
                            let value: String = forward_display_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "rendering" => scene_value.parameters.forward_debug_type = 0,
                                "normal" => scene_value.parameters.forward_debug_type = 1,
                                _ => scene_value.parameters.forward_debug_type = 0,
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>);

                forward_display_select_element
                    .add_event_listener_with_callback(
                        "change",
                        forward_display_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                forward_display_closure.forget();
            }

            forward_display_select_element
                .append_child(&forward_display_option_render)
                .unwrap();
            forward_display_select_element
                .append_child(&forward_display_option_normal)
                .unwrap();

            forward_display_element
                .append_child(&forward_display_label_element)
                .unwrap();
            forward_display_element
                .append_child(&forward_display_select_element)
                .unwrap();

            forward_wrapper
                .append_child(&forward_display_element)
                .unwrap();
        }

        accordion_content_element
            .append_child(&forward_wrapper)
            .unwrap();
    }

    // differed wrapper
    {
        let differed_wrapper = gloo::utils::document().create_element("div").unwrap();
        differed_wrapper.set_id("differed-wrapper");
        differed_wrapper.set_class_name("widget-wrapper");
        if scene.borrow().parameters.scene_shading_type != engine::scene::ShadingType::Differed {
            differed_wrapper.set_class_name("widget-wrapper-hidden");
        }

        // shader type
        {
            let shader_type_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            shader_type_element.set_class_name("widget-row");

            let shader_type_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            shader_type_label_element.set_class_name("widget-label");
            shader_type_label_element.set_text_content(Some("shading"));

            let shader_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            shader_type_select_element.set_class_name("select-element");
            shader_type_select_element.set_id("differed-type-select");

            let shader_type_option_pbr = gloo::utils::document().create_element("option").unwrap();
            shader_type_option_pbr.set_text_content(Some("pbr"));

            /*
            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                let buffer_type_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let buffer_type_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("buffer-type-select")
                                .unwrap();
                            let buffer_type_element: web_sys::HtmlSelectElement =
                                buffer_type_element.dyn_into().unwrap();
                            let value: String = buffer_type_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "render" => scene_value.variables.differed_debug_type = 0,
                                "normal" => scene_value.variables.differed_debug_type = 1,
                                "depth" => scene_value.variables.differed_debug_type = 2,
                                "albedo" => scene_value.variables.differed_debug_type = 3,
                                "metallic" => scene_value.variables.differed_debug_type = 4,
                                _ => scene_value.variables.differed_debug_type = 0,
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>);

                shader_type_select_element
                    .add_event_listener_with_callback(
                        "change",
                        buffer_type_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                buffer_type_closure.forget();
            }*/

            shader_type_select_element
                .append_child(&shader_type_option_pbr)
                .unwrap();

            shader_type_element
                .append_child(&shader_type_label_element)
                .unwrap();
            shader_type_element
                .append_child(&shader_type_select_element)
                .unwrap();

            differed_wrapper.append_child(&shader_type_element).unwrap();
        }

        // buffer
        {
            let buffer_type_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            buffer_type_element.set_class_name("widget-row");

            let buffer_type_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            buffer_type_label_element.set_class_name("widget-label");
            buffer_type_label_element.set_text_content(Some("out"));

            let buffer_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            buffer_type_select_element.set_class_name("select-element");
            buffer_type_select_element.set_id("buffer-type-select");

            let buffer_type_option_render =
                gloo::utils::document().create_element("option").unwrap();
            buffer_type_option_render.set_text_content(Some("render"));
            let buffer_type_option_normal =
                gloo::utils::document().create_element("option").unwrap();
            buffer_type_option_normal.set_text_content(Some("normal"));
            let buffer_type_option_depth =
                gloo::utils::document().create_element("option").unwrap();
            buffer_type_option_depth.set_text_content(Some("depth"));
            let buffer_type_option_albedo =
                gloo::utils::document().create_element("option").unwrap();
            buffer_type_option_albedo.set_text_content(Some("albedo"));
            let buffer_type_option_metallic =
                gloo::utils::document().create_element("option").unwrap();
            buffer_type_option_metallic.set_text_content(Some("metallic"));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                let buffer_type_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let buffer_type_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("buffer-type-select")
                                .unwrap();
                            let buffer_type_element: web_sys::HtmlSelectElement =
                                buffer_type_element.dyn_into().unwrap();
                            let value: String = buffer_type_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "render" => scene_value.parameters.differed_debug_type = 0,
                                "normal" => scene_value.parameters.differed_debug_type = 1,
                                "depth" => scene_value.parameters.differed_debug_type = 2,
                                "albedo" => scene_value.parameters.differed_debug_type = 3,
                                "metallic" => scene_value.parameters.differed_debug_type = 4,
                                _ => scene_value.parameters.differed_debug_type = 0,
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>);

                buffer_type_select_element
                    .add_event_listener_with_callback(
                        "change",
                        buffer_type_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                buffer_type_closure.forget();
            }

            buffer_type_select_element
                .append_child(&buffer_type_option_render)
                .unwrap();
            buffer_type_select_element
                .append_child(&buffer_type_option_normal)
                .unwrap();
            buffer_type_select_element
                .append_child(&buffer_type_option_depth)
                .unwrap();
            buffer_type_select_element
                .append_child(&buffer_type_option_albedo)
                .unwrap();
            buffer_type_select_element
                .append_child(&buffer_type_option_metallic)
                .unwrap();

            buffer_type_element
                .append_child(&buffer_type_label_element)
                .unwrap();
            buffer_type_element
                .append_child(&buffer_type_select_element)
                .unwrap();

            differed_wrapper.append_child(&buffer_type_element).unwrap();
        }

        accordion_content_element
            .append_child(&differed_wrapper)
            .unwrap();
    }

    // clear color
    {
        let clearcolor_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        clearcolor_element.set_class_name("widget-row");

        let clearcolor_label_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        clearcolor_label_element.set_class_name("widget-label");
        clearcolor_label_element.set_text_content(Some("Clear color"));

        let clearcolor_picker_element: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        clearcolor_picker_element.set_class_name("widget-value color-picker-element");
        clearcolor_picker_element.set_id("background-color-picker");
        clearcolor_picker_element
            .set_attribute("type", "color")
            .unwrap();
        {
            let bg_color: [f32; 4] = scene_value.parameters.background_color;
            let r_uint: u32 = (bg_color[0] * 255.0) as u32;
            let r_hex: String = format!("{r_uint:X}");
            let g_uint: u32 = (bg_color[1] * 255.0) as u32;
            let g_hex: String = format!("{g_uint:X}");
            let b_uint: u32 = (bg_color[2] * 255.0) as u32;
            let b_hex: String = format!("{b_uint:X}");

            let hex_string: String = "#".to_string() + &r_hex + &g_hex + &b_hex;
            clearcolor_picker_element
                .set_attribute("value", &hex_string)
                .unwrap();
        }

        {
            let scene_clone: Shared<engine::scene::Scene> = scene.clone();

            let bgcolor_picker_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let picker_element: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("background-color-picker")
                        .unwrap();
                    let picker_element: web_sys::HtmlInputElement =
                        picker_element.dyn_into().unwrap();
                    let value: String = picker_element.value();

                    let color_hex = value.trim_start_matches("#");
                    let color_u8: [u8; 4] =
                        u32::from_str_radix(&color_hex, 16).unwrap().to_be_bytes();

                    let mut scene_value = scene_clone.borrow_mut();
                    scene_value.parameters.background_color = [
                        color_u8[1] as f32 / 256 as f32,
                        color_u8[2] as f32 / 256 as f32,
                        color_u8[3] as f32 / 256 as f32,
                        1.0,
                    ];
                }) as Box<dyn FnMut(_)>);

            clearcolor_picker_element
                .add_event_listener_with_callback(
                    "input",
                    bgcolor_picker_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            bgcolor_picker_closure.forget();
        }

        clearcolor_element
            .append_child(&clearcolor_label_element)
            .unwrap();
        clearcolor_element
            .append_child(&clearcolor_picker_element)
            .unwrap();

        accordion_content_element
            .append_child(&clearcolor_element)
            .unwrap();
    }

    dialog_base_pass
        .append_child(&accordion_input_element)
        .unwrap();
    dialog_base_pass
        .append_child(&accordion_label_element)
        .unwrap();
    dialog_base_pass
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&dialog_base_pass).unwrap();
}

fn create_debug_dialog_sky_box(parent: &web_sys::Element, scene: &Shared<engine::scene::Scene>) {
    let view_statistics = gloo::utils::document().create_element("div").unwrap();
    view_statistics.set_id("dialog-element-skybox");
    view_statistics.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-skybox");
    // accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Skybox"));
    accordion_label_element
        .set_attribute("for", "accordion-skybox")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // active
    {
        let active_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        active_element.set_class_name("widget-row");

        let active_label_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        active_label_element.set_class_name("widget-label");
        active_label_element.set_text_content(Some("Active"));

        let active_content_element = gloo::utils::document().create_element("div").unwrap();
        active_content_element.set_class_name("widget-value");
        active_content_element.set_id("active-analytics-value");

        {
            let sky_box_active_input_checkbox: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let sky_box_active_input_checkbox: web_sys::HtmlInputElement =
                sky_box_active_input_checkbox.dyn_into().unwrap();
            sky_box_active_input_checkbox.set_id("skybox-active");
            sky_box_active_input_checkbox.set_class_name("checkbox-element");
            sky_box_active_input_checkbox
                .set_attribute("type", "checkbox")
                .unwrap();
            sky_box_active_input_checkbox.set_checked(scene.borrow().parameters.is_use_sky_box);

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                let sky_box_active_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let sky_box_active_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("skybox-active")
                                .unwrap();
                            let sky_box_active_element: web_sys::HtmlInputElement =
                                sky_box_active_element.dyn_into().unwrap();
                            let value: bool = sky_box_active_element.checked();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.is_use_sky_box = value;
                        },
                    )
                        as Box<dyn FnMut(_)>);

                sky_box_active_input_checkbox
                    .add_event_listener_with_callback(
                        "input",
                        sky_box_active_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                sky_box_active_closure.forget();
            }

            active_content_element
                .append_child(&sky_box_active_input_checkbox)
                .unwrap();
        }

        active_element.append_child(&active_label_element).unwrap();
        active_element
            .append_child(&active_content_element)
            .unwrap();

        accordion_content_element
            .append_child(&active_element)
            .unwrap();
    }

    view_statistics
        .append_child(&accordion_input_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_label_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&view_statistics).unwrap();
}

fn create_debug_dialog_postprocess(
    parent: &web_sys::Element,
    scene: &Shared<engine::scene::Scene>,
) {
    let scene_value = scene.borrow();

    let view_statistics = gloo::utils::document().create_element("div").unwrap();
    view_statistics.set_id("dialog-element-postprocess");
    view_statistics.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-postprocess");
    // accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Postprocess"));
    accordion_label_element
        .set_attribute("for", "accordion-postprocess")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // bloom
    {
        let bloom_accordion_input_element =
            gloo::utils::document().create_element("input").unwrap();
        let bloom_accordion_input_element: web_sys::HtmlInputElement =
            bloom_accordion_input_element.dyn_into().unwrap();
        bloom_accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        bloom_accordion_input_element.set_class_name("accordion-input");
        bloom_accordion_input_element.set_id("accordion-bloom");
        //bloom_accordion_input_element.set_checked(true);

        let bloom_accordion_label_element =
            gloo::utils::document().create_element("label").unwrap();
        bloom_accordion_label_element.set_class_name("accordion-label inner-accordion-label");
        bloom_accordion_label_element.set_text_content(Some("Bloom"));
        bloom_accordion_label_element
            .set_attribute("for", "accordion-bloom")
            .unwrap();

        let bloom_accordion_content_element =
            gloo::utils::document().create_element("div").unwrap();
        bloom_accordion_content_element.set_class_name("accordion-content inner-accordion-content");

        // active
        {
            let bloom_active_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            bloom_active_element.set_class_name("widget-row");

            let bloom_active_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            bloom_active_label_element.set_class_name("widget-label");
            bloom_active_label_element.set_text_content(Some("Active"));

            let bloom_active_content_element =
                gloo::utils::document().create_element("div").unwrap();
            bloom_active_content_element.set_class_name("widget-value");

            {
                let bloom_active_input_checkbox: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let bloom_active_input_checkbox: web_sys::HtmlInputElement =
                    bloom_active_input_checkbox.dyn_into().unwrap();
                bloom_active_input_checkbox.set_id("bloom-active");
                bloom_active_input_checkbox.set_class_name("checkbox-element");
                bloom_active_input_checkbox
                    .set_attribute("type", "checkbox")
                    .unwrap();
                bloom_active_input_checkbox.set_checked(scene_value.parameters.is_use_bloom);

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let bloom_active_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let bloom_active_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("bloom-active")
                                        .unwrap();
                                let bloom_active_element: web_sys::HtmlInputElement =
                                    bloom_active_element.dyn_into().unwrap();
                                let value: bool = bloom_active_element.checked();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.is_use_bloom = value;
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    bloom_active_input_checkbox
                        .add_event_listener_with_callback(
                            "input",
                            bloom_active_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    bloom_active_closure.forget();
                }

                bloom_active_content_element
                    .append_child(&bloom_active_input_checkbox)
                    .unwrap();
            }

            bloom_active_element
                .append_child(&bloom_active_label_element)
                .unwrap();
            bloom_active_element
                .append_child(&bloom_active_content_element)
                .unwrap();

            bloom_accordion_content_element
                .append_child(&bloom_active_element)
                .unwrap();
        }

        // threshold
        {
            let threshold_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            threshold_element.set_class_name("widget-row");

            let threshold_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            threshold_label_element.set_class_name("widget-label");
            threshold_label_element.set_text_content(Some("Threshold"));

            let threshold_content_element = gloo::utils::document().create_element("div").unwrap();
            threshold_content_element.set_class_name("widget-value");

            {
                let threshold_input_range: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let threshold_input_range: web_sys::HtmlInputElement =
                    threshold_input_range.dyn_into().unwrap();
                threshold_input_range.set_id("threshold-range");
                threshold_input_range.set_class_name("range-element");
                threshold_input_range
                    .set_attribute("type", "range")
                    .unwrap();
                threshold_input_range.set_attribute("min", "0.0").unwrap();
                threshold_input_range.set_attribute("max", "2.0").unwrap();
                threshold_input_range.set_attribute("step", "0.01").unwrap();
                threshold_input_range
                    .set_value(scene_value.parameters.bloom_threshold.to_string().as_str());

                let threshold_input_range_text: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                threshold_input_range_text.set_id("threshold-range-text");
                threshold_input_range_text.set_class_name("range-text-element");
                threshold_input_range_text.set_text_content(Some(
                    scene_value.parameters.bloom_threshold.to_string().as_str(),
                ));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let threshold_range_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let threshold_element: web_sys::Element = gloo::utils::document()
                                    .get_element_by_id("threshold-range")
                                    .unwrap();
                                let threshold_element: web_sys::HtmlInputElement =
                                    threshold_element.dyn_into().unwrap();
                                let value: String = threshold_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.bloom_threshold =
                                    value.parse::<f32>().unwrap();

                                let threshold_text_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("threshold-range-text")
                                        .unwrap();
                                threshold_text_element.set_text_content(Some(&value));
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    threshold_input_range
                        .add_event_listener_with_callback(
                            "input",
                            threshold_range_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    threshold_range_closure.forget();
                }

                threshold_content_element
                    .append_child(&threshold_input_range)
                    .unwrap();
                threshold_content_element
                    .append_child(&threshold_input_range_text)
                    .unwrap();
            }

            threshold_element
                .append_child(&threshold_label_element)
                .unwrap();
            threshold_element
                .append_child(&threshold_content_element)
                .unwrap();

            bloom_accordion_content_element
                .append_child(&threshold_element)
                .unwrap();
        }

        accordion_content_element
            .append_child(&bloom_accordion_input_element)
            .unwrap();
        accordion_content_element
            .append_child(&bloom_accordion_label_element)
            .unwrap();
        accordion_content_element
            .append_child(&bloom_accordion_content_element)
            .unwrap();
    }

    // composite
    {
        let composite_accordion_input_element =
            gloo::utils::document().create_element("input").unwrap();
        let composite_accordion_input_element: web_sys::HtmlInputElement =
            composite_accordion_input_element.dyn_into().unwrap();
        composite_accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        composite_accordion_input_element.set_class_name("accordion-input");
        composite_accordion_input_element.set_id("accordion-composite");
        //composite_accordion_input_element.set_checked(true);

        let composite_accordion_label_element =
            gloo::utils::document().create_element("label").unwrap();
        composite_accordion_label_element.set_class_name("accordion-label inner-accordion-label");
        composite_accordion_label_element.set_text_content(Some("Composite"));
        composite_accordion_label_element
            .set_attribute("for", "accordion-composite")
            .unwrap();

        let composite_accordion_content_element =
            gloo::utils::document().create_element("div").unwrap();
        composite_accordion_content_element
            .set_class_name("accordion-content inner-accordion-content");

        // active
        {
            let composite_active_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_active_element.set_class_name("widget-row");

            let composite_active_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_active_label_element.set_class_name("widget-label");
            composite_active_label_element.set_text_content(Some("Active"));

            let composite_active_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_active_content_element.set_class_name("widget-value");

            {
                let composite_active_input_checkbox: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let composite_active_input_checkbox: web_sys::HtmlInputElement =
                    composite_active_input_checkbox.dyn_into().unwrap();
                composite_active_input_checkbox.set_id("composite-active");
                composite_active_input_checkbox.set_class_name("checkbox-element");
                composite_active_input_checkbox
                    .set_attribute("type", "checkbox")
                    .unwrap();
                composite_active_input_checkbox
                    .set_checked(scene_value.parameters.is_use_composite);

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let composite_active_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let composite_active_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("composite-active")
                                        .unwrap();
                                let composite_active_element: web_sys::HtmlInputElement =
                                    composite_active_element.dyn_into().unwrap();
                                let value: bool = composite_active_element.checked();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.is_use_composite = value;
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    composite_active_input_checkbox
                        .add_event_listener_with_callback(
                            "input",
                            composite_active_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    composite_active_closure.forget();
                }

                composite_active_content_element
                    .append_child(&composite_active_input_checkbox)
                    .unwrap();
            }

            composite_active_element
                .append_child(&composite_active_label_element)
                .unwrap();
            composite_active_element
                .append_child(&composite_active_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_active_element)
                .unwrap();
        }

        // exposure
        {
            let composite_exposure_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_exposure_element.set_class_name("widget-row");

            let composite_exposure_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_exposure_label_element.set_class_name("widget-label");
            composite_exposure_label_element.set_text_content(Some("Exposure"));

            let composite_exposure_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_exposure_content_element.set_class_name("widget-value");

            let composite_exposure_input_range: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let composite_exposure_input_range: web_sys::HtmlInputElement =
                composite_exposure_input_range.dyn_into().unwrap();
            composite_exposure_input_range.set_id("composite-exposure-range");
            composite_exposure_input_range.set_class_name("range-element");
            composite_exposure_input_range
                .set_attribute("type", "range")
                .unwrap();
            composite_exposure_input_range
                .set_attribute("min", "-3.0")
                .unwrap();
            composite_exposure_input_range
                .set_attribute("max", "3.0")
                .unwrap();
            composite_exposure_input_range
                .set_attribute("step", "0.01")
                .unwrap();
            composite_exposure_input_range.set_value(
                scene_value.parameters.composite_exposure.to_string().as_str(),
            );

            let composite_exposure_input_range_text: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_exposure_input_range_text.set_id("composite-exposure-range-text");
            composite_exposure_input_range_text.set_class_name("range-text-element");
            composite_exposure_input_range_text.set_text_content(Some(
                scene_value.parameters.composite_exposure.to_string().as_str(),
            ));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_exposure_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let range_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-exposure-range").unwrap();
                            let range_element: web_sys::HtmlInputElement =
                                range_element.dyn_into().unwrap();
                            let value: String = range_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_exposure = value.parse::<f32>().unwrap();

                            let range_text_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-exposure-range-text").unwrap();
                            range_text_element.set_text_content(Some(&value));
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_exposure_input_range
                    .add_event_listener_with_callback(
                        "input",
                        composite_exposure_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_exposure_closure.forget();
            }

            composite_exposure_content_element
                .append_child(&composite_exposure_input_range)
                .unwrap();
            composite_exposure_content_element
                .append_child(&composite_exposure_input_range_text)
                .unwrap();
            composite_exposure_element
                .append_child(&composite_exposure_label_element)
                .unwrap();
            composite_exposure_element
                .append_child(&composite_exposure_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_exposure_element)
                .unwrap();
        }

        // saturation
        {
            let composite_saturation_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_saturation_element.set_class_name("widget-row");

            let composite_saturation_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_saturation_label_element.set_class_name("widget-label");
            composite_saturation_label_element.set_text_content(Some("Saturation"));

            let composite_saturation_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_saturation_content_element.set_class_name("widget-value");

            let composite_saturation_input_range: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let composite_saturation_input_range: web_sys::HtmlInputElement =
                composite_saturation_input_range.dyn_into().unwrap();
            composite_saturation_input_range.set_id("composite-saturation-range");
            composite_saturation_input_range.set_class_name("range-element");
            composite_saturation_input_range
                .set_attribute("type", "range")
                .unwrap();
            composite_saturation_input_range
                .set_attribute("min", "0.0")
                .unwrap();
            composite_saturation_input_range
                .set_attribute("max", "2.0")
                .unwrap();
            composite_saturation_input_range
                .set_attribute("step", "0.01")
                .unwrap();
            composite_saturation_input_range.set_value(
                scene_value.parameters.composite_saturation.to_string().as_str(),
            );

            let composite_saturation_input_range_text: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_saturation_input_range_text.set_id("composite-saturation-range-text");
            composite_saturation_input_range_text.set_class_name("range-text-element");
            composite_saturation_input_range_text.set_text_content(Some(
                scene_value.parameters.composite_saturation.to_string().as_str(),
            ));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_saturation_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let range_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-saturation-range").unwrap();
                            let range_element: web_sys::HtmlInputElement =
                                range_element.dyn_into().unwrap();
                            let value: String = range_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_saturation = value.parse::<f32>().unwrap();

                            let range_text_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-saturation-range-text").unwrap();
                            range_text_element.set_text_content(Some(&value));
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_saturation_input_range
                    .add_event_listener_with_callback(
                        "input",
                        composite_saturation_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_saturation_closure.forget();
            }

            composite_saturation_content_element
                .append_child(&composite_saturation_input_range)
                .unwrap();
            composite_saturation_content_element
                .append_child(&composite_saturation_input_range_text)
                .unwrap();
            composite_saturation_element
                .append_child(&composite_saturation_label_element)
                .unwrap();
            composite_saturation_element
                .append_child(&composite_saturation_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_saturation_element)
                .unwrap();
        }

        // tone mapping
        {
            let composite_tone_mapping_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_tone_mapping_element.set_class_name("widget-row");

            let composite_tone_mapping_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_tone_mapping_label_element.set_class_name("widget-label");
            composite_tone_mapping_label_element.set_text_content(Some("Tone Mapping"));

            let composite_tone_mapping_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_tone_mapping_content_element.set_class_name("widget-value");

            let composite_tone_mapping_select_element =
                gloo::utils::document().create_element("select").unwrap();
            composite_tone_mapping_select_element.set_class_name("select-element");
            composite_tone_mapping_select_element.set_id("composite-tone-mapping-select");

            let composite_tone_mapping_option_off =
                gloo::utils::document().create_element("option").unwrap();
            composite_tone_mapping_option_off.set_text_content(Some("off"));
            composite_tone_mapping_option_off.set_attribute("value", "off").unwrap();

            let composite_tone_mapping_option_aces =
                gloo::utils::document().create_element("option").unwrap();
            composite_tone_mapping_option_aces.set_text_content(Some("aces"));
            composite_tone_mapping_option_aces.set_attribute("value", "aces").unwrap();

            let composite_tone_mapping_option_filmic =
                gloo::utils::document().create_element("option").unwrap();
            composite_tone_mapping_option_filmic.set_text_content(Some("filmic"));
            composite_tone_mapping_option_filmic.set_attribute("value", "filmic").unwrap();

            let composite_tone_mapping_option_agx =
                gloo::utils::document().create_element("option").unwrap();
            composite_tone_mapping_option_agx.set_text_content(Some("agx"));
            composite_tone_mapping_option_agx.set_attribute("value", "agx").unwrap();

            match scene_value.parameters.tone_mapping_type {
                engine::scene::ToneMappingType::Off => {
                    composite_tone_mapping_option_off.set_attribute("selected", "")
                }
                engine::scene::ToneMappingType::Aces => {
                    composite_tone_mapping_option_aces.set_attribute("selected", "")
                }
                engine::scene::ToneMappingType::Filmic => {
                    composite_tone_mapping_option_filmic.set_attribute("selected", "")
                }
                engine::scene::ToneMappingType::Agx => {
                    composite_tone_mapping_option_agx.set_attribute("selected", "")
                }
            }
            .unwrap();

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_tone_mapping_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let select_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-tone-mapping-select").unwrap();
                            let select_element: web_sys::HtmlSelectElement =
                                select_element.dyn_into().unwrap();
                            let value: String = select_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.tone_mapping_type = match value.as_str() {
                                "off" => engine::scene::ToneMappingType::Off,
                                "aces" => engine::scene::ToneMappingType::Aces,
                                "filmic" => engine::scene::ToneMappingType::Filmic,
                                "agx" => engine::scene::ToneMappingType::Agx,
                                _ => engine::scene::ToneMappingType::Agx,
                            };
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_tone_mapping_select_element
                    .add_event_listener_with_callback(
                        "change",
                        composite_tone_mapping_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_tone_mapping_closure.forget();
            }

            composite_tone_mapping_select_element
                .append_child(&composite_tone_mapping_option_off)
                .unwrap();
            composite_tone_mapping_select_element
                .append_child(&composite_tone_mapping_option_aces)
                .unwrap();
            composite_tone_mapping_select_element
                .append_child(&composite_tone_mapping_option_filmic)
                .unwrap();
            composite_tone_mapping_select_element
                .append_child(&composite_tone_mapping_option_agx)
                .unwrap();

            composite_tone_mapping_content_element
                .append_child(&composite_tone_mapping_select_element)
                .unwrap();
            composite_tone_mapping_element
                .append_child(&composite_tone_mapping_label_element)
                .unwrap();
            composite_tone_mapping_element
                .append_child(&composite_tone_mapping_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_tone_mapping_element)
                .unwrap();
        }

        // highlight rolloff
        {
            let composite_highlight_rolloff_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_highlight_rolloff_element.set_class_name("widget-row");

            let composite_highlight_rolloff_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_highlight_rolloff_label_element.set_class_name("widget-label");
            composite_highlight_rolloff_label_element.set_text_content(Some("Soft"));

            let composite_highlight_rolloff_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_highlight_rolloff_content_element.set_class_name("widget-value");

            let composite_highlight_rolloff_input_range: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let composite_highlight_rolloff_input_range: web_sys::HtmlInputElement =
                composite_highlight_rolloff_input_range.dyn_into().unwrap();
            composite_highlight_rolloff_input_range.set_id("composite-highlight-rolloff-range");
            composite_highlight_rolloff_input_range.set_class_name("range-element");
            composite_highlight_rolloff_input_range
                .set_attribute("type", "range")
                .unwrap();
            composite_highlight_rolloff_input_range
                .set_attribute("min", "0.0")
                .unwrap();
            composite_highlight_rolloff_input_range
                .set_attribute("max", "1.0")
                .unwrap();
            composite_highlight_rolloff_input_range
                .set_attribute("step", "0.01")
                .unwrap();
            composite_highlight_rolloff_input_range.set_value(
                scene_value.parameters.composite_highlight_rolloff.to_string().as_str(),
            );

            let composite_highlight_rolloff_input_range_text: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_highlight_rolloff_input_range_text.set_id("composite-highlight-rolloff-range-text");
            composite_highlight_rolloff_input_range_text.set_class_name("range-text-element");
            composite_highlight_rolloff_input_range_text.set_text_content(Some(
                scene_value.parameters.composite_highlight_rolloff.to_string().as_str(),
            ));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_highlight_rolloff_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let range_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-highlight-rolloff-range").unwrap();
                            let range_element: web_sys::HtmlInputElement =
                                range_element.dyn_into().unwrap();
                            let value: String = range_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_highlight_rolloff = value.parse::<f32>().unwrap();

                            let range_text_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-highlight-rolloff-range-text").unwrap();
                            range_text_element.set_text_content(Some(&value));
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_highlight_rolloff_input_range
                    .add_event_listener_with_callback(
                        "input",
                        composite_highlight_rolloff_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_highlight_rolloff_closure.forget();
            }

            composite_highlight_rolloff_content_element
                .append_child(&composite_highlight_rolloff_input_range)
                .unwrap();
            composite_highlight_rolloff_content_element
                .append_child(&composite_highlight_rolloff_input_range_text)
                .unwrap();
            composite_highlight_rolloff_element
                .append_child(&composite_highlight_rolloff_label_element)
                .unwrap();
            composite_highlight_rolloff_element
                .append_child(&composite_highlight_rolloff_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_highlight_rolloff_element)
                .unwrap();
        }

        // white point
        {
            let composite_white_point_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_white_point_element.set_class_name("widget-row");

            let composite_white_point_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_white_point_label_element.set_class_name("widget-label");
            composite_white_point_label_element.set_text_content(Some("White"));

            let composite_white_point_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_white_point_content_element.set_class_name("widget-value");

            let composite_white_point_input_range: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let composite_white_point_input_range: web_sys::HtmlInputElement =
                composite_white_point_input_range.dyn_into().unwrap();
            composite_white_point_input_range.set_id("composite-white-point-range");
            composite_white_point_input_range.set_class_name("range-element");
            composite_white_point_input_range
                .set_attribute("type", "range")
                .unwrap();
            composite_white_point_input_range
                .set_attribute("min", "0.1")
                .unwrap();
            composite_white_point_input_range
                .set_attribute("max", "4.0")
                .unwrap();
            composite_white_point_input_range
                .set_attribute("step", "0.01")
                .unwrap();
            composite_white_point_input_range.set_value(
                scene_value.parameters.composite_white_point.to_string().as_str(),
            );

            let composite_white_point_input_range_text: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_white_point_input_range_text.set_id("composite-white-point-range-text");
            composite_white_point_input_range_text.set_class_name("range-text-element");
            composite_white_point_input_range_text.set_text_content(Some(
                scene_value.parameters.composite_white_point.to_string().as_str(),
            ));

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_white_point_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let range_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-white-point-range").unwrap();
                            let range_element: web_sys::HtmlInputElement =
                                range_element.dyn_into().unwrap();
                            let value: String = range_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_white_point = value.parse::<f32>().unwrap();

                            let range_text_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-white-point-range-text").unwrap();
                            range_text_element.set_text_content(Some(&value));
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_white_point_input_range
                    .add_event_listener_with_callback(
                        "input",
                        composite_white_point_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_white_point_closure.forget();
            }

            composite_white_point_content_element
                .append_child(&composite_white_point_input_range)
                .unwrap();
            composite_white_point_content_element
                .append_child(&composite_white_point_input_range_text)
                .unwrap();
            composite_white_point_element
                .append_child(&composite_white_point_label_element)
                .unwrap();
            composite_white_point_element
                .append_child(&composite_white_point_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_white_point_element)
                .unwrap();
        }

        // view transform
        {
            let composite_view_transform_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_view_transform_element.set_class_name("widget-row");

            let composite_view_transform_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_view_transform_label_element.set_class_name("widget-label");
            composite_view_transform_label_element.set_text_content(Some("View Trans"));

            let composite_view_transform_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_view_transform_content_element.set_class_name("widget-value");

            let composite_view_transform_select_element =
                gloo::utils::document().create_element("select").unwrap();
            composite_view_transform_select_element.set_class_name("select-element");
            composite_view_transform_select_element.set_id("composite-view-transform-select");

            let composite_view_transform_option_standard =
                gloo::utils::document().create_element("option").unwrap();
            composite_view_transform_option_standard.set_text_content(Some("standard"));
            composite_view_transform_option_standard
                .set_attribute("value", "standard")
                .unwrap();

            let composite_view_transform_option_agx =
                gloo::utils::document().create_element("option").unwrap();
            composite_view_transform_option_agx.set_text_content(Some("agx"));
            composite_view_transform_option_agx
                .set_attribute("value", "agx")
                .unwrap();

            let composite_view_transform_option_filmic =
                gloo::utils::document().create_element("option").unwrap();
            composite_view_transform_option_filmic.set_text_content(Some("filmic"));
            composite_view_transform_option_filmic
                .set_attribute("value", "filmic")
                .unwrap();

            match scene_value.parameters.composite_view_transform {
                engine::scene::CompositeViewTransform::Standard => {
                    composite_view_transform_option_standard.set_attribute("selected", "")
                }
                engine::scene::CompositeViewTransform::Agx => {
                    composite_view_transform_option_agx.set_attribute("selected", "")
                }
                engine::scene::CompositeViewTransform::Filmic => {
                    composite_view_transform_option_filmic.set_attribute("selected", "")
                }
            }
            .unwrap();

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_view_transform_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let select_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-view-transform-select").unwrap();
                            let select_element: web_sys::HtmlSelectElement =
                                select_element.dyn_into().unwrap();
                            let value: String = select_element.value();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_view_transform = match value.as_str() {
                                "standard" => engine::scene::CompositeViewTransform::Standard,
                                "agx" => engine::scene::CompositeViewTransform::Agx,
                                "filmic" => engine::scene::CompositeViewTransform::Filmic,
                                _ => engine::scene::CompositeViewTransform::Agx,
                            };
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_view_transform_select_element
                    .add_event_listener_with_callback(
                        "change",
                        composite_view_transform_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_view_transform_closure.forget();
            }

            composite_view_transform_select_element
                .append_child(&composite_view_transform_option_standard)
                .unwrap();
            composite_view_transform_select_element
                .append_child(&composite_view_transform_option_agx)
                .unwrap();
            composite_view_transform_select_element
                .append_child(&composite_view_transform_option_filmic)
                .unwrap();

            composite_view_transform_content_element
                .append_child(&composite_view_transform_select_element)
                .unwrap();
            composite_view_transform_element
                .append_child(&composite_view_transform_label_element)
                .unwrap();
            composite_view_transform_element
                .append_child(&composite_view_transform_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_view_transform_element)
                .unwrap();
        }

        // dither
        {
            let composite_dither_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_dither_element.set_class_name("widget-row");

            let composite_dither_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            composite_dither_label_element.set_class_name("widget-label");
            composite_dither_label_element.set_text_content(Some("Dithered"));

            let composite_dither_content_element =
                gloo::utils::document().create_element("div").unwrap();
            composite_dither_content_element.set_class_name("widget-value");

            let composite_dither_input_checkbox: web_sys::Element =
                gloo::utils::document().create_element("input").unwrap();
            let composite_dither_input_checkbox: web_sys::HtmlInputElement =
                composite_dither_input_checkbox.dyn_into().unwrap();
            composite_dither_input_checkbox.set_id("composite-dither-active");
            composite_dither_input_checkbox.set_class_name("checkbox-element");
            composite_dither_input_checkbox
                .set_attribute("type", "checkbox")
                .unwrap();
            composite_dither_input_checkbox.set_checked(scene_value.parameters.composite_use_dither);

            {
                let scene_clone: Shared<engine::scene::Scene> = scene.clone();
                let composite_dither_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let composite_dither_element: web_sys::Element =
                                gloo::utils::document().get_element_by_id("composite-dither-active").unwrap();
                            let composite_dither_element: web_sys::HtmlInputElement =
                                composite_dither_element.dyn_into().unwrap();
                            let value: bool = composite_dither_element.checked();

                            let mut scene_value = scene_clone.borrow_mut();
                            scene_value.parameters.composite_use_dither = value;
                        },
                    ) as Box<dyn FnMut(_)>);

                composite_dither_input_checkbox
                    .add_event_listener_with_callback(
                        "input",
                        composite_dither_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                composite_dither_closure.forget();
            }

            composite_dither_content_element
                .append_child(&composite_dither_input_checkbox)
                .unwrap();
            composite_dither_element
                .append_child(&composite_dither_label_element)
                .unwrap();
            composite_dither_element
                .append_child(&composite_dither_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&composite_dither_element)
                .unwrap();
        }

        // gamma correction
        {
            let gamma_correction_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            gamma_correction_element.set_class_name("widget-row");

            let gamma_correction_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            gamma_correction_label_element.set_class_name("widget-label");
            gamma_correction_label_element.set_text_content(Some("Gamma"));

            let gamma_correction_content_element =
                gloo::utils::document().create_element("div").unwrap();
            gamma_correction_content_element.set_class_name("widget-value");

            {
                let gamma_correction_input_checkbox: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let gamma_correction_input_checkbox: web_sys::HtmlInputElement =
                    gamma_correction_input_checkbox.dyn_into().unwrap();
                gamma_correction_input_checkbox.set_id("gamma-correction-active");
                gamma_correction_input_checkbox.set_class_name("checkbox-element");
                gamma_correction_input_checkbox
                    .set_attribute("type", "checkbox")
                    .unwrap();
                gamma_correction_input_checkbox
                    .set_checked(scene_value.parameters.is_use_gamma_correction);

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let gamma_correction_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let gamma_correction_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("gamma-correction-active")
                                        .unwrap();
                                let gamma_correction_element: web_sys::HtmlInputElement =
                                    gamma_correction_element.dyn_into().unwrap();
                                let value: bool = gamma_correction_element.checked();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.is_use_gamma_correction = value;
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    gamma_correction_input_checkbox
                        .add_event_listener_with_callback(
                            "input",
                            gamma_correction_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    gamma_correction_closure.forget();
                }

                gamma_correction_content_element
                    .append_child(&gamma_correction_input_checkbox)
                    .unwrap();
            }

            gamma_correction_element
                .append_child(&gamma_correction_label_element)
                .unwrap();
            gamma_correction_element
                .append_child(&gamma_correction_content_element)
                .unwrap();

            composite_accordion_content_element
                .append_child(&gamma_correction_element)
                .unwrap();
        }

        accordion_content_element
            .append_child(&composite_accordion_input_element)
            .unwrap();
        accordion_content_element
            .append_child(&composite_accordion_label_element)
            .unwrap();
        accordion_content_element
            .append_child(&composite_accordion_content_element)
            .unwrap();
    }

    view_statistics
        .append_child(&accordion_input_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_label_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&view_statistics).unwrap();
}

fn create_debug_dialog_overlay(parent: &web_sys::Element, scene: &Shared<engine::scene::Scene>) {
    let scene_value = scene.borrow();

    let dialog_overlay = gloo::utils::document().create_element("div").unwrap();
    dialog_overlay.set_id("dialog-element-overlay");
    dialog_overlay.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-overlay");
    // accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Overlay"));
    accordion_label_element
        .set_attribute("for", "accordion-overlay")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // grid
    {
        let grid_accordion_input_element = gloo::utils::document().create_element("input").unwrap();
        let grid_accordion_input_element: web_sys::HtmlInputElement =
            grid_accordion_input_element.dyn_into().unwrap();
        grid_accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        grid_accordion_input_element.set_class_name("accordion-input");
        grid_accordion_input_element.set_id("accordion-grid");
        //grid_accordion_input_element.set_checked(true);

        let grid_accordion_label_element = gloo::utils::document().create_element("label").unwrap();
        grid_accordion_label_element.set_class_name("accordion-label inner-accordion-label");
        grid_accordion_label_element.set_text_content(Some("Grid"));
        grid_accordion_label_element
            .set_attribute("for", "accordion-grid")
            .unwrap();

        let grid_accordion_content_element = gloo::utils::document().create_element("div").unwrap();
        grid_accordion_content_element.set_class_name("accordion-content inner-accordion-content");

        // active
        {
            let grid_active_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            grid_active_element.set_class_name("widget-row");

            let grid_active_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            grid_active_label_element.set_class_name("widget-label");
            grid_active_label_element.set_text_content(Some("Active"));

            let grid_active_content_element =
                gloo::utils::document().create_element("div").unwrap();
            grid_active_content_element.set_class_name("widget-value");

            {
                let grid_active_input_checkbox: web_sys::Element =
                    gloo::utils::document().create_element("input").unwrap();
                let grid_active_input_checkbox: web_sys::HtmlInputElement =
                    grid_active_input_checkbox.dyn_into().unwrap();
                grid_active_input_checkbox.set_id("grid-active");
                grid_active_input_checkbox.set_class_name("checkbox-element");
                grid_active_input_checkbox
                    .set_attribute("type", "checkbox")
                    .unwrap();
                grid_active_input_checkbox.set_checked(scene_value.parameters.is_use_grid);

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let grid_active_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let grid_active_element: web_sys::Element = gloo::utils::document()
                                    .get_element_by_id("grid-active")
                                    .unwrap();
                                let grid_active_element: web_sys::HtmlInputElement =
                                    grid_active_element.dyn_into().unwrap();
                                let value: bool = grid_active_element.checked();

                                let mut scene_value = scene_clone.borrow_mut();
                                scene_value.parameters.is_use_grid = value;
                            },
                        )
                            as Box<dyn FnMut(_)>);

                    grid_active_input_checkbox
                        .add_event_listener_with_callback(
                            "input",
                            grid_active_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    grid_active_closure.forget();
                }

                grid_active_content_element
                    .append_child(&grid_active_input_checkbox)
                    .unwrap();
            }

            grid_active_element
                .append_child(&grid_active_label_element)
                .unwrap();
            grid_active_element
                .append_child(&grid_active_content_element)
                .unwrap();

            grid_accordion_content_element
                .append_child(&grid_active_element)
                .unwrap();
        }

        accordion_content_element
            .append_child(&grid_accordion_input_element)
            .unwrap();
        accordion_content_element
            .append_child(&grid_accordion_label_element)
            .unwrap();
        accordion_content_element
            .append_child(&grid_accordion_content_element)
            .unwrap();
    }

    dialog_overlay
        .append_child(&accordion_input_element)
        .unwrap();
    dialog_overlay
        .append_child(&accordion_label_element)
        .unwrap();
    dialog_overlay
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&dialog_overlay).unwrap();
}

fn create_debug_dialog_statistics(parent: &web_sys::Element, scene: &Shared<engine::scene::Scene>) {
    let view_statistics = gloo::utils::document().create_element("div").unwrap();
    view_statistics.set_id("dialog-element-analytics");
    view_statistics.set_class_name("dialog-element dialog-element-display");

    let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
    let accordion_input_element: web_sys::HtmlInputElement =
        accordion_input_element.dyn_into().unwrap();
    accordion_input_element
        .set_attribute("type", "checkbox")
        .unwrap();
    accordion_input_element.set_class_name("accordion-input");
    accordion_input_element.set_id("accordion-analytics");
    // accordion_input_element.set_checked(true);

    let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
    accordion_label_element.set_class_name("accordion-label");
    accordion_label_element.set_text_content(Some("Statistics"));
    accordion_label_element
        .set_attribute("for", "accordion-analytics")
        .unwrap();

    let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
    accordion_content_element.set_class_name("accordion-content");

    // objects
    {
        let objects_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        objects_element.set_class_name("widget-row");

        let objects_label_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        objects_label_element.set_class_name("widget-label");
        objects_label_element.set_text_content(Some("Objects"));

        let objects_stats_content_element = gloo::utils::document().create_element("div").unwrap();
        objects_stats_content_element.set_class_name("widget-value");
        objects_stats_content_element.set_id("objects-analytics-value");
        objects_stats_content_element
            .set_text_content(Some(scene.borrow().objects.len().to_string().as_str()));

        objects_element
            .append_child(&objects_label_element)
            .unwrap();
        objects_element
            .append_child(&objects_stats_content_element)
            .unwrap();

        accordion_content_element
            .append_child(&objects_element)
            .unwrap();
    }

    // materials
    {
        let materials_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        materials_element.set_class_name("widget-row");

        let materials_label_element: web_sys::Element =
            gloo::utils::document().create_element("div").unwrap();
        materials_label_element.set_class_name("widget-label");
        materials_label_element.set_text_content(Some("Materials"));

        let objects_stats_content_element = gloo::utils::document().create_element("div").unwrap();
        objects_stats_content_element.set_class_name("widget-value");
        objects_stats_content_element.set_id("materials-analytics-value");
        objects_stats_content_element
            .set_text_content(Some(scene.borrow().materials.len().to_string().as_str()));

        materials_element
            .append_child(&materials_label_element)
            .unwrap();
        materials_element
            .append_child(&objects_stats_content_element)
            .unwrap();

        accordion_content_element
            .append_child(&materials_element)
            .unwrap();
    }

    view_statistics
        .append_child(&accordion_input_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_label_element)
        .unwrap();
    view_statistics
        .append_child(&accordion_content_element)
        .unwrap();

    parent.append_child(&view_statistics).unwrap();
}
