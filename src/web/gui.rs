use crate::engine;
use crate::types::Shared;
use wasm_bindgen::JsCast;

pub fn start_gui(scene: &Shared<engine::scene::Scene>) {
    create_panels();
    create_view_dialog(scene);
}

fn create_panels() {
    let body: web_sys::HtmlElement = gloo::utils::body();

    let panel_wrapper: web_sys::Element = gloo::utils::document().create_element("div").unwrap();
    panel_wrapper.set_id("panel-wrapper");

    // Environment panel
    {
        let panel_environment_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_environment_radio: web_sys::HtmlInputElement =
            panel_environment_radio.dyn_into().unwrap();
        panel_environment_radio.set_id("panel-environment-checkbox");
        panel_environment_radio.set_class_name("panel-checkbox");
        panel_environment_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_environment_radio
            .set_attribute("name", "panel")
            .unwrap();
        panel_environment_radio.set_checked(true);

        let panel_environment_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_environment_label.set_class_name("panel-label");
        panel_environment_label
            .set_attribute("for", "panel-environment-checkbox")
            .unwrap();

        let panel_environment_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_environment_icon.set_class_name("material-symbols-outlined");
        panel_environment_icon.set_text_content(Some("globe_asia"));

        panel_environment_label
            .append_child(&panel_environment_icon)
            .unwrap();

        panel_wrapper
            .append_child(&panel_environment_radio)
            .unwrap();
        panel_wrapper
            .append_child(&panel_environment_label)
            .unwrap();

        {
            let panel_environment_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let environment_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-environment-checkbox")
                        .unwrap();
                    let environment_checkbox: web_sys::HtmlInputElement =
                        environment_checkbox.dyn_into().unwrap();
                    let checked: bool = environment_checkbox.checked();
                    environment_checkbox.set_checked(checked);

                    let view_environment: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-environment")
                        .unwrap();
                    let view_environment: web_sys::HtmlElement =
                        view_environment.dyn_into().unwrap();

                    if checked {
                        view_environment.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_environment.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_environment_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_environment_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_environment_closure.forget();
        }
    }

    // Rendering panel
    {
        let panel_rendering_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_rendering_radio: web_sys::HtmlInputElement =
            panel_rendering_radio.dyn_into().unwrap();
        panel_rendering_radio.set_id("panel-graphics-checkbox");
        panel_rendering_radio.set_class_name("panel-checkbox");
        panel_rendering_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_rendering_radio
            .set_attribute("name", "panel")
            .unwrap();
        panel_rendering_radio.set_checked(true);

        let panel_rendering_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_rendering_label.set_class_name("panel-label");
        panel_rendering_label
            .set_attribute("for", "panel-graphics-checkbox")
            .unwrap();

        let panel_rendering_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_rendering_icon.set_class_name("material-symbols-outlined");
        panel_rendering_icon.set_text_content(Some("3d"));

        panel_rendering_label
            .append_child(&panel_rendering_icon)
            .unwrap();

        panel_wrapper.append_child(&panel_rendering_radio).unwrap();
        panel_wrapper.append_child(&panel_rendering_label).unwrap();

        {
            let panel_rendering_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let rendering_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-graphics-checkbox")
                        .unwrap();
                    let rendering_checkbox: web_sys::HtmlInputElement =
                        rendering_checkbox.dyn_into().unwrap();
                    let checked: bool = rendering_checkbox.checked();
                    rendering_checkbox.set_checked(checked);

                    let view_rendering: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-graphics")
                        .unwrap();
                    let view_rendering: web_sys::HtmlElement = view_rendering.dyn_into().unwrap();

                    if checked {
                        view_rendering.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_rendering.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_rendering_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_rendering_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_rendering_closure.forget();
        }
    }

    // Statistics panel
    {
        let panel_statistics_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_statistics_radio: web_sys::HtmlInputElement =
            panel_statistics_radio.dyn_into().unwrap();
        panel_statistics_radio.set_id("panel-analytics-checkbox");
        panel_statistics_radio.set_class_name("panel-checkbox");
        panel_statistics_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_statistics_radio
            .set_attribute("name", "panel")
            .unwrap();
        panel_statistics_radio.set_checked(true);

        let panel_statistics_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_statistics_label.set_class_name("panel-label");
        panel_statistics_label
            .set_attribute("for", "panel-analytics-checkbox")
            .unwrap();

        let panel_statistics_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_statistics_icon.set_class_name("material-symbols-outlined");
        panel_statistics_icon.set_text_content(Some("bar_chart"));

        panel_statistics_label
            .append_child(&panel_statistics_icon)
            .unwrap();

        panel_wrapper.append_child(&panel_statistics_radio).unwrap();
        panel_wrapper.append_child(&panel_statistics_label).unwrap();

        {
            let panel_statistics_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let statistics_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-analytics-checkbox")
                        .unwrap();
                    let statistics_checkbox: web_sys::HtmlInputElement =
                        statistics_checkbox.dyn_into().unwrap();
                    let checked: bool = statistics_checkbox.checked();
                    statistics_checkbox.set_checked(checked);

                    let view_statistics: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-analytics")
                        .unwrap();
                    let view_statistics: web_sys::HtmlElement = view_statistics.dyn_into().unwrap();

                    if checked {
                        view_statistics.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_statistics.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_statistics_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_statistics_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_statistics_closure.forget();
        }
    }

    body.append_child(&panel_wrapper).unwrap();
}

fn create_view_dialog(scene: &Shared<engine::scene::Scene>) {
    let body: web_sys::HtmlElement = gloo::utils::body();

    let view_wrapper: web_sys::Element = gloo::utils::document().create_element("div").unwrap();
    view_wrapper.set_id("view-wrapper");

    let scene_value: std::cell::Ref<'_, engine::scene::Scene> = scene.borrow();

    // Environment view
    {
        let view_environment = gloo::utils::document().create_element("div").unwrap();
        view_environment.set_id("view-dialog-environment");
        view_environment.set_class_name("view-dialog view-dialog-display");

        let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
        let accordion_input_element: web_sys::HtmlInputElement =
            accordion_input_element.dyn_into().unwrap();
        accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        accordion_input_element.set_class_name("accordion-input");
        accordion_input_element.set_id("accordion-environment");
        accordion_input_element.set_checked(true);

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
            let sun_accordion_input_element =
                gloo::utils::document().create_element("input").unwrap();
            let sun_accordion_input_element: web_sys::HtmlInputElement =
                sun_accordion_input_element.dyn_into().unwrap();
            sun_accordion_input_element
                .set_attribute("type", "checkbox")
                .unwrap();
            sun_accordion_input_element.set_class_name("accordion-input");
            sun_accordion_input_element.set_id("accordion-sun");
            sun_accordion_input_element.set_checked(true);

            let sun_accordion_label_element =
                gloo::utils::document().create_element("label").unwrap();
            sun_accordion_label_element.set_class_name("accordion-label inner-accordion-label");
            sun_accordion_label_element.set_text_content(Some("Directional Light"));
            sun_accordion_label_element
                .set_attribute("for", "accordion-sun")
                .unwrap();

            let sun_accordion_content_element =
                gloo::utils::document().create_element("div").unwrap();
            sun_accordion_content_element
                .set_class_name("accordion-content inner-accordion-content");

            // X
            {
                let sun_x_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_x_element.set_class_name("widget-row");

                let sun_x_label_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_x_label_element.set_class_name("widget-label");
                sun_x_label_element.set_text_content(Some("X"));

                let sun_x_content_element = gloo::utils::document().create_element("div").unwrap();
                sun_x_content_element.set_class_name("widget-value");

                {
                    let sun_x_input_range: web_sys::Element =
                        gloo::utils::document().create_element("input").unwrap();
                    let sun_x_input_range: web_sys::HtmlInputElement =
                        sun_x_input_range.dyn_into().unwrap();
                    sun_x_input_range.set_id("sun-range-x");
                    sun_x_input_range.set_class_name("range-element");
                    sun_x_input_range.set_attribute("type", "range").unwrap();
                    sun_x_input_range.set_attribute("min", "-1.0").unwrap();
                    sun_x_input_range.set_attribute("max", "1.0").unwrap();
                    sun_x_input_range.set_attribute("step", "0.01").unwrap();
                    sun_x_input_range.set_value(
                        scene_value.variables.directional_light_angle[0]
                            .to_string()
                            .as_str(),
                    );

                    let sun_x_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_x_input_range_text.set_id("sun-range-x-text");
                    sun_x_input_range_text.set_class_name("range-text-element");
                    sun_x_input_range_text.set_text_content(Some(
                        scene_value.variables.directional_light_angle[0]
                            .to_string()
                            .as_str(),
                    ));

                    {
                        let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                        let sun_range_x_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                            wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_event: web_sys::InputEvent| {
                                    let range_x_element: web_sys::Element = gloo::utils::document()
                                        .get_element_by_id("sun-range-x")
                                        .unwrap();
                                    let range_x_element: web_sys::HtmlInputElement =
                                        range_x_element.dyn_into().unwrap();
                                    let value: String = range_x_element.value();

                                    let mut scene_value = scene_clone.borrow_mut();
                                    scene_value.variables.directional_light_angle[0] =
                                        value.parse::<f32>().unwrap();

                                    let range_x_text_element: web_sys::Element =
                                        gloo::utils::document()
                                            .get_element_by_id("sun-range-x-text")
                                            .unwrap();
                                    range_x_text_element.set_text_content(Some(&value));
                                },
                            )
                                as Box<dyn FnMut(_)>);

                        sun_x_input_range
                            .add_event_listener_with_callback(
                                "input",
                                sun_range_x_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        sun_range_x_closure.forget();
                    }

                    sun_x_content_element
                        .append_child(&sun_x_input_range)
                        .unwrap();
                    sun_x_content_element
                        .append_child(&sun_x_input_range_text)
                        .unwrap();
                }

                sun_x_element.append_child(&sun_x_label_element).unwrap();
                sun_x_element.append_child(&sun_x_content_element).unwrap();

                sun_accordion_content_element
                    .append_child(&sun_x_element)
                    .unwrap();
            }
            // Y
            {
                let sun_y_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_y_element.set_class_name("widget-row");

                let sun_y_label_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_y_label_element.set_class_name("widget-label");
                sun_y_label_element.set_text_content(Some("Y"));

                let sun_y_content_element = gloo::utils::document().create_element("div").unwrap();
                sun_y_content_element.set_class_name("widget-value");

                {
                    let sun_y_input_range: web_sys::Element =
                        gloo::utils::document().create_element("input").unwrap();
                    let sun_y_input_range: web_sys::HtmlInputElement =
                        sun_y_input_range.dyn_into().unwrap();
                    sun_y_input_range.set_id("sun-range-y");
                    sun_y_input_range.set_class_name("range-element");
                    sun_y_input_range.set_attribute("type", "range").unwrap();
                    sun_y_input_range.set_attribute("min", "-1.0").unwrap();
                    sun_y_input_range.set_attribute("max", "1.0").unwrap();
                    sun_y_input_range.set_attribute("step", "0.01").unwrap();
                    sun_y_input_range.set_value(
                        scene_value.variables.directional_light_angle[1]
                            .to_string()
                            .as_str(),
                    );

                    let sun_y_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_y_input_range_text.set_id("sun-range-y-text");
                    sun_y_input_range_text.set_class_name("range-text-element");
                    sun_y_input_range_text.set_text_content(Some(
                        scene_value.variables.directional_light_angle[1]
                            .to_string()
                            .as_str(),
                    ));

                    {
                        let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                        let sun_range_y_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                            wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_event: web_sys::InputEvent| {
                                    let range_x_element: web_sys::Element = gloo::utils::document()
                                        .get_element_by_id("sun-range-y")
                                        .unwrap();
                                    let range_y_element: web_sys::HtmlInputElement =
                                        range_x_element.dyn_into().unwrap();
                                    let value: String = range_y_element.value();

                                    let mut scene_value = scene_clone.borrow_mut();
                                    scene_value.variables.directional_light_angle[1] =
                                        value.parse::<f32>().unwrap();

                                    let range_y_text_element: web_sys::Element =
                                        gloo::utils::document()
                                            .get_element_by_id("sun-range-y-text")
                                            .unwrap();
                                    range_y_text_element.set_text_content(Some(&value));
                                },
                            )
                                as Box<dyn FnMut(_)>);

                        sun_y_input_range
                            .add_event_listener_with_callback(
                                "input",
                                sun_range_y_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        sun_range_y_closure.forget();
                    }

                    sun_y_content_element
                        .append_child(&sun_y_input_range)
                        .unwrap();
                    sun_y_content_element
                        .append_child(&sun_y_input_range_text)
                        .unwrap();
                }
                sun_y_element.append_child(&sun_y_label_element).unwrap();
                sun_y_element.append_child(&sun_y_content_element).unwrap();

                sun_accordion_content_element
                    .append_child(&sun_y_element)
                    .unwrap();
            }
            // Z
            {
                let sun_z_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_z_element.set_class_name("widget-row");

                let sun_z_label_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                sun_z_label_element.set_class_name("widget-label");
                sun_z_label_element.set_text_content(Some("Z"));

                let sun_z_content_element = gloo::utils::document().create_element("div").unwrap();
                sun_z_content_element.set_class_name("widget-value");

                {
                    let sun_z_input_range: web_sys::Element =
                        gloo::utils::document().create_element("input").unwrap();
                    let sun_z_input_range: web_sys::HtmlInputElement =
                        sun_z_input_range.dyn_into().unwrap();
                    sun_z_input_range.set_id("sun-range-z");
                    sun_z_input_range.set_class_name("range-element");
                    sun_z_input_range.set_attribute("type", "range").unwrap();
                    sun_z_input_range.set_attribute("min", "-1.0").unwrap();
                    sun_z_input_range.set_attribute("max", "1.0").unwrap();
                    sun_z_input_range.set_attribute("step", "0.01").unwrap();
                    sun_z_input_range.set_value(
                        scene_value.variables.directional_light_angle[2]
                            .to_string()
                            .as_str(),
                    );

                    let sun_z_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_z_input_range_text.set_id("sun-range-z-text");
                    sun_z_input_range_text.set_class_name("range-text-element");
                    sun_z_input_range_text.set_text_content(Some(
                        scene_value.variables.directional_light_angle[2]
                            .to_string()
                            .as_str(),
                    ));

                    {
                        let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                        let sun_range_z_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                            wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_event: web_sys::InputEvent| {
                                    let range_z_element: web_sys::Element = gloo::utils::document()
                                        .get_element_by_id("sun-range-z")
                                        .unwrap();
                                    let range_z_element: web_sys::HtmlInputElement =
                                        range_z_element.dyn_into().unwrap();
                                    let value: String = range_z_element.value();

                                    let mut scene_value = scene_clone.borrow_mut();
                                    scene_value.variables.directional_light_angle[2] =
                                        value.parse::<f32>().unwrap();

                                    let range_z_text_element: web_sys::Element =
                                        gloo::utils::document()
                                            .get_element_by_id("sun-range-z-text")
                                            .unwrap();
                                    range_z_text_element.set_text_content(Some(&value));
                                },
                            )
                                as Box<dyn FnMut(_)>);

                        sun_z_input_range
                            .add_event_listener_with_callback(
                                "input",
                                sun_range_z_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        sun_range_z_closure.forget();
                    }

                    sun_z_content_element
                        .append_child(&sun_z_input_range)
                        .unwrap();
                    sun_z_content_element
                        .append_child(&sun_z_input_range_text)
                        .unwrap();
                }
                sun_z_element.append_child(&sun_z_label_element).unwrap();
                sun_z_element.append_child(&sun_z_content_element).unwrap();

                sun_accordion_content_element
                    .append_child(&sun_z_element)
                    .unwrap();
            }

            accordion_content_element
                .append_child(&sun_accordion_input_element)
                .unwrap();
            accordion_content_element
                .append_child(&sun_accordion_label_element)
                .unwrap();
            accordion_content_element
                .append_child(&sun_accordion_content_element)
                .unwrap();
        }

        view_environment
            .append_child(&accordion_input_element)
            .unwrap();
        view_environment
            .append_child(&accordion_label_element)
            .unwrap();
        view_environment
            .append_child(&accordion_content_element)
            .unwrap();

        view_wrapper.append_child(&view_environment).unwrap();
    }

    // Rendering view
    {
        let view_graphics = gloo::utils::document().create_element("div").unwrap();
        view_graphics.set_id("view-dialog-graphics");
        view_graphics.set_class_name("view-dialog view-dialog-display");

        let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
        let accordion_input_element: web_sys::HtmlInputElement =
            accordion_input_element.dyn_into().unwrap();
        accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        accordion_input_element.set_class_name("accordion-input");
        accordion_input_element.set_id("accordion-graphics");
        accordion_input_element.set_checked(true);

        let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
        accordion_label_element.set_class_name("accordion-label");
        accordion_label_element.set_text_content(Some("Rendering"));
        accordion_label_element
            .set_attribute("for", "accordion-graphics")
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

            let render_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            render_type_select_element.set_class_name("select-element");
            render_type_select_element.set_id("render-type-select");

            let render_type_option_forward =
                gloo::utils::document().create_element("option").unwrap();
            render_type_option_forward.set_text_content(Some("forward"));
            let render_type_option_differed =
                gloo::utils::document().create_element("option").unwrap();
            render_type_option_differed.set_text_content(Some("differed"));

            match &scene_value.variables.scene_shading_type {
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
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
                            let render_type_element: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("render-type-select")
                                .unwrap();
                            let render_type_element: web_sys::HtmlSelectElement =
                                render_type_element.dyn_into().unwrap();
                            let value: String = render_type_element.value();

                            let forward_wrapper: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("forward-wrapper")
                                .unwrap();
                            let forward_wrapper: web_sys::HtmlElement =
                                forward_wrapper.dyn_into().unwrap();

                            let differed_wrapper: web_sys::Element = gloo::utils::document()
                                .get_element_by_id("differed-wrapper")
                                .unwrap();
                            let differed_wrapper: web_sys::HtmlElement =
                                differed_wrapper.dyn_into().unwrap();

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "forward" => {
                                    scene_value.variables.scene_shading_type =
                                        engine::scene::ShadingType::Forward;

                                    forward_wrapper.set_class_name("widget-wrapper");
                                    differed_wrapper
                                        .set_class_name("widget-wrapper widget-wrapper-hidden");
                                }
                                "differed" => {
                                    scene_value.variables.scene_shading_type =
                                        engine::scene::ShadingType::Differed;

                                    forward_wrapper
                                        .set_class_name("widget-wrapper widget-wrapper-hidden");
                                    differed_wrapper.set_class_name("widget-wrapper");
                                }
                                _ => {}
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>);

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

            // shader type
            {
                let shader_type_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                shader_type_element.set_class_name("widget-row");

                let shader_type_label_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                shader_type_label_element.set_class_name("widget-label");
                shader_type_label_element.set_text_content(Some("shader"));

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
                forward_display_label_element.set_text_content(Some("display"));

                let forward_display_select_element =
                    gloo::utils::document().create_element("select").unwrap();
                forward_display_select_element.set_class_name("select-element");
                forward_display_select_element.set_id("forward-display-select");

                let forward_display_option_render =
                    gloo::utils::document().create_element("option").unwrap();
                forward_display_option_render.set_text_content(Some("rendering"));
                let forward_display_option_normal =
                    gloo::utils::document().create_element("option").unwrap();
                forward_display_option_normal.set_text_content(Some("normal"));

                {
                    let scene_clone: Shared<engine::scene::Scene> = scene.clone();

                    let forward_display_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                        wasm_bindgen::closure::Closure::wrap(Box::new(
                            move |_event: web_sys::InputEvent| {
                                let forward_display_element: web_sys::Element =
                                    gloo::utils::document()
                                        .get_element_by_id("forward-display-select")
                                        .unwrap();
                                let forward_display_element: web_sys::HtmlSelectElement =
                                    forward_display_element.dyn_into().unwrap();
                                let value: String = forward_display_element.value();

                                let mut scene_value = scene_clone.borrow_mut();
                                match value.as_str() {
                                    "rendering" => scene_value.variables.forward_debug_type = 0,
                                    "normal" => scene_value.variables.forward_debug_type = 1,
                                    _ => scene_value.variables.forward_debug_type = 0,
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
            differed_wrapper.set_class_name("widget-wrapper widget-wrapper-hidden");

            // shader type
            {
                let shader_type_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                shader_type_element.set_class_name("widget-row");

                let shader_type_label_element: web_sys::Element =
                    gloo::utils::document().create_element("div").unwrap();
                shader_type_label_element.set_class_name("widget-label");
                shader_type_label_element.set_text_content(Some("shader"));

                let shader_type_select_element =
                    gloo::utils::document().create_element("select").unwrap();
                shader_type_select_element.set_class_name("select-element");
                shader_type_select_element.set_id("differed-type-select");

                let shader_type_option_pbr =
                    gloo::utils::document().create_element("option").unwrap();
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
                buffer_type_label_element.set_text_content(Some("display"));

                let buffer_type_select_element =
                    gloo::utils::document().create_element("select").unwrap();
                buffer_type_select_element.set_class_name("select-element");
                buffer_type_select_element.set_id("buffer-type-select");

                let buffer_type_option_render =
                    gloo::utils::document().create_element("option").unwrap();
                buffer_type_option_render.set_text_content(Some("rendering"));
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
                let bg_color: [f32; 4] = scene_value.variables.background_color;
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
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |_event: web_sys::InputEvent| {
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
                            scene_value.variables.background_color = [
                                color_u8[1] as f32 / 256 as f32,
                                color_u8[2] as f32 / 256 as f32,
                                color_u8[3] as f32 / 256 as f32,
                                1.0,
                            ];
                        },
                    )
                        as Box<dyn FnMut(_)>);

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

        view_graphics
            .append_child(&accordion_input_element)
            .unwrap();
        view_graphics
            .append_child(&accordion_label_element)
            .unwrap();
        view_graphics
            .append_child(&accordion_content_element)
            .unwrap();

        view_wrapper.append_child(&view_graphics).unwrap();
    }

    // Statistics view
    {
        let view_statistics = gloo::utils::document().create_element("div").unwrap();
        view_statistics.set_id("view-dialog-analytics");
        view_statistics.set_class_name("view-dialog view-dialog-display");

        let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
        let accordion_input_element: web_sys::HtmlInputElement =
            accordion_input_element.dyn_into().unwrap();
        accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        accordion_input_element.set_class_name("accordion-input");
        accordion_input_element.set_id("accordion-analytics");
        accordion_input_element.set_checked(true);

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

            let objects_stats_content_element =
                gloo::utils::document().create_element("div").unwrap();
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

            let objects_stats_content_element =
                gloo::utils::document().create_element("div").unwrap();
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

        view_wrapper.append_child(&view_statistics).unwrap();
    }

    body.append_child(&view_wrapper).unwrap();
}
