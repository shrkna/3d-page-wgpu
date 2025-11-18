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

    // Weather panel
    {
        let panel_weather_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_weather_radio: web_sys::HtmlInputElement =
            panel_weather_radio.dyn_into().unwrap();
        panel_weather_radio.set_id("panel-weather-checkbox");
        panel_weather_radio.set_class_name("panel-checkbox");
        panel_weather_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_weather_radio.set_attribute("name", "panel").unwrap();

        let panel_weather_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_weather_label.set_class_name("panel-label");
        panel_weather_label
            .set_attribute("for", "panel-weather-checkbox")
            .unwrap();

        let panel_weather_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_weather_icon.set_class_name("material-symbols-outlined");
        panel_weather_icon.set_text_content(Some("partly_cloudy_day"));

        panel_weather_label
            .append_child(&panel_weather_icon)
            .unwrap();

        panel_wrapper.append_child(&panel_weather_radio).unwrap();
        panel_wrapper.append_child(&panel_weather_label).unwrap();

        {
            let panel_weather_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let weather_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-weather-checkbox")
                        .unwrap();
                    let weather_checkbox: web_sys::HtmlInputElement =
                        weather_checkbox.dyn_into().unwrap();
                    let checked: bool = weather_checkbox.checked();

                    reset_state();
                    weather_checkbox.set_checked(checked);

                    let view_weather: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-weather")
                        .unwrap();
                    let view_weather: web_sys::HtmlElement = view_weather.dyn_into().unwrap();

                    if checked {
                        view_weather.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_weather.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_weather_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_weather_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_weather_closure.forget();
        }
    }

    // Graphics panel
    {
        let panel_graphics_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_graphics_radio: web_sys::HtmlInputElement =
            panel_graphics_radio.dyn_into().unwrap();
        panel_graphics_radio.set_id("panel-graphics-checkbox");
        panel_graphics_radio.set_class_name("panel-checkbox");
        panel_graphics_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_graphics_radio.set_attribute("name", "panel").unwrap();
        panel_graphics_radio.set_checked(true);

        let panel_graphics_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_graphics_label.set_class_name("panel-label");
        panel_graphics_label
            .set_attribute("for", "panel-graphics-checkbox")
            .unwrap();

        let panel_graphics_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_graphics_icon.set_class_name("material-symbols-outlined");
        panel_graphics_icon.set_text_content(Some("photo_camera"));

        panel_graphics_label
            .append_child(&panel_graphics_icon)
            .unwrap();

        panel_wrapper.append_child(&panel_graphics_radio).unwrap();
        panel_wrapper.append_child(&panel_graphics_label).unwrap();

        {
            let panel_grahics_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let graphic_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-graphics-checkbox")
                        .unwrap();
                    let graphic_checkbox: web_sys::HtmlInputElement =
                        graphic_checkbox.dyn_into().unwrap();
                    let checked: bool = graphic_checkbox.checked();

                    reset_state();
                    graphic_checkbox.set_checked(checked);

                    let view_graphics: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-graphics")
                        .unwrap();
                    let view_graphics: web_sys::HtmlElement = view_graphics.dyn_into().unwrap();

                    if checked {
                        view_graphics.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_graphics.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_graphics_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_grahics_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_grahics_closure.forget();
        }
    }

    // Analytics panel
    {
        let panel_analytics_radio: web_sys::Element =
            gloo::utils::document().create_element("input").unwrap();
        let panel_analytics_radio: web_sys::HtmlInputElement =
            panel_analytics_radio.dyn_into().unwrap();
        panel_analytics_radio.set_id("panel-analytics-checkbox");
        panel_analytics_radio.set_class_name("panel-checkbox");
        panel_analytics_radio
            .set_attribute("type", "checkbox")
            .unwrap();
        panel_analytics_radio
            .set_attribute("name", "panel")
            .unwrap();

        let panel_analytics_label: web_sys::Element =
            gloo::utils::document().create_element("label").unwrap();
        panel_analytics_label.set_class_name("panel-label");
        panel_analytics_label
            .set_attribute("for", "panel-analytics-checkbox")
            .unwrap();

        let panel_analytics_icon: web_sys::Element =
            gloo::utils::document().create_element("span").unwrap();
        panel_analytics_icon.set_class_name("material-symbols-outlined");
        panel_analytics_icon.set_text_content(Some("analytics"));

        panel_analytics_label
            .append_child(&panel_analytics_icon)
            .unwrap();

        panel_wrapper.append_child(&panel_analytics_radio).unwrap();
        panel_wrapper.append_child(&panel_analytics_label).unwrap();

        {
            let panel_analytics_closure: wasm_bindgen::prelude::Closure<dyn FnMut(_)> =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                    let analytics_checkbox: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("panel-analytics-checkbox")
                        .unwrap();
                    let analytics_checkbox: web_sys::HtmlInputElement =
                        analytics_checkbox.dyn_into().unwrap();
                    let checked: bool = analytics_checkbox.checked();

                    reset_state();
                    analytics_checkbox.set_checked(checked);

                    let view_analytics: web_sys::Element = gloo::utils::document()
                        .get_element_by_id("view-dialog-analytics")
                        .unwrap();
                    let view_analytics: web_sys::HtmlElement = view_analytics.dyn_into().unwrap();

                    if checked {
                        view_analytics.set_class_name("view-dialog view-dialog-display");
                    } else {
                        view_analytics.set_class_name("view-dialog view-dialog-hidden");
                    }
                }) as Box<dyn FnMut(_)>);

            panel_analytics_radio
                .add_event_listener_with_callback(
                    "change",
                    panel_analytics_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            panel_analytics_closure.forget();
        }
    }

    body.append_child(&panel_wrapper).unwrap();
}

fn create_view_dialog(scene: &Shared<engine::scene::Scene>) {
    let body: web_sys::HtmlElement = gloo::utils::body();

    let view_wrapper: web_sys::Element = gloo::utils::document().create_element("div").unwrap();
    view_wrapper.set_id("view-wrapper");

    let scene_value: std::cell::Ref<'_, engine::scene::Scene> = scene.borrow();

    // weather view
    {
        let view_weather = gloo::utils::document().create_element("div").unwrap();
        view_weather.set_id("view-dialog-weather");
        view_weather.set_class_name("view-dialog view-dialog-hidden");

        let accordion_input_element = gloo::utils::document().create_element("input").unwrap();
        let accordion_input_element: web_sys::HtmlInputElement =
            accordion_input_element.dyn_into().unwrap();
        accordion_input_element
            .set_attribute("type", "checkbox")
            .unwrap();
        accordion_input_element.set_class_name("accordion-input");
        accordion_input_element.set_id("accordion-weather");
        accordion_input_element.set_checked(true);

        let accordion_label_element = gloo::utils::document().create_element("label").unwrap();
        accordion_label_element.set_class_name("accordion-label");
        accordion_label_element.set_text_content(Some("Weather"));
        accordion_label_element
            .set_attribute("for", "accordion-weather")
            .unwrap();

        let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
        accordion_content_element.set_class_name("accordion-content");

        // sun light
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
            sun_accordion_label_element.set_text_content(Some("Sun Light"));
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
                        scene_value.scene_variables.directional_light_angle[0]
                            .to_string()
                            .as_str(),
                    );

                    let sun_x_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_x_input_range_text.set_id("sun-range-x-text");
                    sun_x_input_range_text.set_class_name("range-text-element");
                    sun_x_input_range_text.set_text_content(Some(
                        scene_value.scene_variables.directional_light_angle[0]
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
                                    scene_value.scene_variables.directional_light_angle[0] =
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
                        scene_value.scene_variables.directional_light_angle[1]
                            .to_string()
                            .as_str(),
                    );

                    let sun_y_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_y_input_range_text.set_id("sun-range-y-text");
                    sun_y_input_range_text.set_class_name("range-text-element");
                    sun_y_input_range_text.set_text_content(Some(
                        scene_value.scene_variables.directional_light_angle[1]
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
                                    scene_value.scene_variables.directional_light_angle[1] =
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
                        scene_value.scene_variables.directional_light_angle[2]
                            .to_string()
                            .as_str(),
                    );

                    let sun_z_input_range_text: web_sys::Element =
                        gloo::utils::document().create_element("div").unwrap();
                    sun_z_input_range_text.set_id("sun-range-z-text");
                    sun_z_input_range_text.set_class_name("range-text-element");
                    sun_z_input_range_text.set_text_content(Some(
                        scene_value.scene_variables.directional_light_angle[2]
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
                                    scene_value.scene_variables.directional_light_angle[2] =
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

        view_weather.append_child(&accordion_input_element).unwrap();
        view_weather.append_child(&accordion_label_element).unwrap();
        view_weather
            .append_child(&accordion_content_element)
            .unwrap();

        view_wrapper.append_child(&view_weather).unwrap();
    }

    // graphics view
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
        accordion_label_element.set_text_content(Some("Graphics"));
        accordion_label_element
            .set_attribute("for", "accordion-graphics")
            .unwrap();

        let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
        accordion_content_element.set_class_name("accordion-content");

        // render type
        {
            let render_type_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            render_type_element.set_class_name("widget-row");

            let render_type_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            render_type_label_element.set_class_name("widget-label");
            render_type_label_element.set_text_content(Some("Render type"));

            let render_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            render_type_select_element.set_class_name("widget-value select-element");
            render_type_select_element.set_id("render-type-select");

            let render_type_option_differed =
                gloo::utils::document().create_element("option").unwrap();
            render_type_option_differed.set_text_content(Some("differed"));
            let render_type_option_forward =
                gloo::utils::document().create_element("option").unwrap();
            render_type_option_forward.set_text_content(Some("forward"));

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

                            let mut scene_value = scene_clone.borrow_mut();
                            match value.as_str() {
                                "differed" => {
                                    scene_value.scene_variables.scene_shading_type =
                                        engine::scene::ShadingType::Differed;
                                }
                                "forward" => {
                                    scene_value.scene_variables.scene_shading_type =
                                        engine::scene::ShadingType::Forward
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
                .append_child(&render_type_option_differed)
                .unwrap();
            render_type_select_element
                .append_child(&render_type_option_forward)
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
                let bg_color: [f32; 4] = scene_value.scene_variables.background_color;
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
                            scene_value.scene_variables.background_color = [
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

        // buffer
        {
            let buffer_type_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            buffer_type_element.set_class_name("widget-row");

            let buffer_type_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            buffer_type_label_element.set_class_name("widget-label");
            buffer_type_label_element.set_text_content(Some("Buffer"));

            let buffer_type_select_element =
                gloo::utils::document().create_element("select").unwrap();
            buffer_type_select_element.set_class_name("widget-value select-element");
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
                                "render" => scene_value.scene_variables.differed_debug_type = 0,
                                "normal" => scene_value.scene_variables.differed_debug_type = 1,
                                "depth" => scene_value.scene_variables.differed_debug_type = 2,
                                "albedo" => scene_value.scene_variables.differed_debug_type = 3,
                                "metallic" => scene_value.scene_variables.differed_debug_type = 4,
                                _ => scene_value.scene_variables.differed_debug_type = 0,
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

            accordion_content_element
                .append_child(&buffer_type_element)
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

    // analytics view
    {
        let view_analytics = gloo::utils::document().create_element("div").unwrap();
        view_analytics.set_id("view-dialog-analytics");
        view_analytics.set_class_name("view-dialog view-dialog-hidden");

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
        accordion_label_element.set_text_content(Some("Analytics"));
        accordion_label_element
            .set_attribute("for", "accordion-analytics")
            .unwrap();

        let accordion_content_element = gloo::utils::document().create_element("div").unwrap();
        accordion_content_element.set_class_name("accordion-content");

        // device
        {
            let device_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            device_element.set_class_name("widget-row");

            let device_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            device_label_element.set_class_name("widget-label");
            device_label_element.set_text_content(Some("Device"));

            let divce_stats_content_element =
                gloo::utils::document().create_element("div").unwrap();
            divce_stats_content_element.set_class_name("widget-value");
            divce_stats_content_element.set_id("device-analytics-value");
            divce_stats_content_element.set_text_content(Some("None"));

            device_element.append_child(&device_label_element).unwrap();
            device_element
                .append_child(&divce_stats_content_element)
                .unwrap();

            accordion_content_element
                .append_child(&device_element)
                .unwrap();
        }

        // fps
        {
            let fps_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            fps_element.set_class_name("widget-row");

            let fps_label_element: web_sys::Element =
                gloo::utils::document().create_element("div").unwrap();
            fps_label_element.set_class_name("widget-label");
            fps_label_element.set_text_content(Some("Frame"));

            let fps_stats_content_element = gloo::utils::document().create_element("div").unwrap();
            fps_stats_content_element.set_class_name("widget-value");
            fps_stats_content_element.set_id("fps-analytics-value");
            fps_stats_content_element.set_text_content(Some("0"));

            fps_element.append_child(&fps_label_element).unwrap();
            fps_element
                .append_child(&fps_stats_content_element)
                .unwrap();

            accordion_content_element
                .append_child(&fps_element)
                .unwrap();
        }

        view_analytics
            .append_child(&accordion_input_element)
            .unwrap();
        view_analytics
            .append_child(&accordion_label_element)
            .unwrap();
        view_analytics
            .append_child(&accordion_content_element)
            .unwrap();

        view_wrapper.append_child(&view_analytics).unwrap();
    }

    body.append_child(&view_wrapper).unwrap();
}

fn reset_state() {
    let all_panel: web_sys::HtmlCollection =
        gloo::utils::document().get_elements_by_class_name("panel-checkbox");
    for i in 0..all_panel.length() {
        let panel: web_sys::Element = all_panel.item(i).unwrap();
        let panel: web_sys::HtmlInputElement = panel.dyn_into().unwrap();
        panel.set_checked(false);
    }

    let all_dialog: web_sys::HtmlCollection =
        gloo::utils::document().get_elements_by_class_name("view-dialog");
    for i in 0..all_dialog.length() {
        let dialog = all_dialog.item(i).unwrap();
        dialog.set_class_name("view-dialog view-dialog-hidden");
    }
}
