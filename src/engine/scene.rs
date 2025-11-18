use crate::types::Shared;
use crate::{frontend, rendering};
use glam::Vec4Swizzles;

// Scene definision

#[derive(Clone, Default)]
pub struct Scene {
    // own
    pub objects: Vec<SceneObject>,
    pub materials: Vec<SceneMaterial>,
    pub batched_objects: Vec<SceneObject>,
    pub scene_variables: SceneVariables,
}

#[derive(Clone, Default)]
pub struct SceneObject {
    pub _name: Option<std::string::String>,
    pub index: u32,
    pub parent_index: Option<u32>,
    pub child_index: Vec<u32>,
    pub world_transform: [[f32; 4]; 4],
    pub source_mesh: Option<Shared<rendering::common::Mesh>>,
    pub shading_type: u8,
    pub rendering_resource: Option<Shared<rendering::webgpu::WebGPURenderingResource>>,
    pub render_resource_deprecated:
        Option<Shared<rendering::webgpu::WebGPURenderingResourceDeprecated>>,
}

#[derive(Clone, Default)]
pub struct SceneMaterial {
    pub _name: Option<std::string::String>,
    pub base_color_texture: Vec<u8>,
    pub base_color_texture_size: [u32; 2],
    pub normal_texture: Vec<u8>,
    pub normal_texture_size: [u32; 2],
    pub metallic_roughness_texture: Vec<u8>,
    pub metallic_roughness_texture_size: [u32; 2],
}

#[derive(Clone, Default)]
pub struct SceneVariables {
    // rendering variables
    pub eye_location: glam::Vec3,
    pub eye_direction: glam::Vec3,
    pub directional_light_angle: [f32; 3],
    pub ambient_light_color: [f32; 4],
    pub background_color: [f32; 4],
    pub scene_shading_type: ShadingType,
    pub differed_debug_type: u8,
    // config
    pub is_first_update: bool,
    pub convert_y_to_z: bool,
    pub use_batched: bool,
}

#[derive(Clone, Copy, Default)]
pub enum ShadingType {
    #[default]
    None,
    Differed,
    Forward,
}

// Initialize builder

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: [].to_vec(),
            batched_objects: [].to_vec(),
            materials: [].to_vec(),
            scene_variables: SceneVariables::new(),
        }
    }
}

impl SceneVariables {
    pub fn new() -> Self {
        Self {
            eye_location: glam::Vec3 {
                x: 0.0,
                y: -2.0,
                z: 0.0,
            },
            eye_direction: glam::Vec3::Y,
            directional_light_angle: [1.0, 1.0, -1.0],
            ambient_light_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.7, 0.7, 0.7, 1.0],
            scene_shading_type: ShadingType::Forward,
            differed_debug_type: 0,
            is_first_update: false,
            convert_y_to_z: true,
            use_batched: false,
        }
    }
}

// Utilities
#[allow(dead_code)]
pub fn batch_objects(scene: &Shared<Scene>) {
    scene.borrow_mut().batched_objects = [].to_vec();

    let mut batch_map: std::collections::HashMap<u32, rendering::common::Mesh> =
        std::collections::HashMap::with_capacity(scene.borrow().objects.len());

    for object in scene.borrow().objects.iter() {
        if object.source_mesh.is_some() {
            let source_mesh: &Shared<rendering::common::Mesh> =
                object.source_mesh.as_ref().unwrap();
            let material_option: Option<u32> = source_mesh.borrow().material;
            if material_option.is_some() {
                let material: &u32 = material_option.as_ref().unwrap();
                // init
                if batch_map.get(material).is_none() {
                    batch_map.insert(*material, rendering::common::Mesh::default());
                }
                // batch
                let batched_mesh = batch_map.get_mut(material).unwrap();
                let mut source_vertices = source_mesh.borrow().vertices.clone();
                let indices_offset = batched_mesh.vertices.len() as u32;
                let trans_matrix = glam::Mat4::from_cols_array_2d(&object.world_transform);
                let rotation_matrix =
                    glam::Mat4::from_quat(trans_matrix.to_scale_rotation_translation().1);
                for i in 0..source_vertices.len() {
                    let vert = glam::Vec4::from_array(source_vertices[i]._pos);
                    let transed_vert = trans_matrix.mul_vec4(vert);
                    source_vertices[i]._pos = transed_vert.to_array();
                    let norm = glam::Vec4::new(
                        source_vertices[i]._normal[0],
                        source_vertices[i]._normal[1],
                        source_vertices[i]._normal[2],
                        1.0,
                    );
                    let transed_norm = rotation_matrix.mul_vec4(norm);
                    source_vertices[i]._normal = [transed_norm.x, transed_norm.y, transed_norm.z];
                    let tangent = glam::Vec4::new(
                        source_vertices[i]._tangent[0],
                        source_vertices[i]._tangent[1],
                        source_vertices[i]._tangent[2],
                        1.0,
                    );
                    let transed_tangent = rotation_matrix.mul_vec4(tangent);
                    source_vertices[i]._tangent =
                        [transed_tangent.x, transed_tangent.y, transed_tangent.z];
                }
                batched_mesh.vertices.append(&mut source_vertices);

                let mut source_indices = source_mesh.borrow().indices.clone();
                for index in source_indices.iter_mut() {
                    *index += indices_offset;
                }
                batched_mesh.indices.append(&mut source_indices);
                batched_mesh.material = Some(*material);
            } else {
                // no material not create batched mesh
            }
        }
    }

    for batch_pair in batch_map {
        let batched_object = SceneObject {
            _name: Some("batched".to_string()),
            shading_type: 44,
            world_transform: glam::Mat4::IDENTITY.to_cols_array_2d(),
            source_mesh: Some(std::rc::Rc::new(std::cell::RefCell::new(batch_pair.1))),
            render_resource_deprecated: None,
            index: batch_pair.0,
            ..Default::default()
        };
        scene.borrow_mut().batched_objects.push(batched_object);
    }
}

pub fn update_control(
    scene: &Shared<Scene>,
    in_control_event: &Shared<frontend::eventlistener::ControlResponseJs>,
) {
    let mut scene_value = scene.borrow_mut();
    let mut eye: glam::Vec3 = scene_value.scene_variables.eye_location;
    let mut direction: glam::Vec3 = scene_value.scene_variables.eye_direction;

    let mut control_event_js = in_control_event.borrow_mut();

    // Calculate eye direction (rotation)
    let on_click: bool = control_event_js.on_click;
    let on_shift: bool = control_event_js.on_shift;
    if on_click && !on_shift {
        let rotate_x_mat =
            glam::Mat3::from_rotation_z(-1.0 * control_event_js.movement_x as f32 * 0.005);
        direction = rotate_x_mat.mul_vec3(direction);

        let y_axis = glam::vec3(direction.x, direction.y, direction.z)
            .cross(glam::vec3(0.0, 0.0, 1.0))
            .normalize();
        let rotate_y_quat =
            glam::Quat::from_axis_angle(y_axis, -1.0 * control_event_js.movement_y as f32 * 0.005);
        direction = rotate_y_quat.mul_vec3(direction);
    } else if on_click && on_shift {
        let direction_mat: glam::Mat4 = glam::Mat4::from_translation(direction);
        let up_move_vec: glam::Vec4 = direction_mat.mul_vec4(glam::Vec4::Z).normalize();
        let right_move_vec: glam::Vec4 = direction_mat.mul_vec4(glam::Vec4::Y).normalize();
        eye += -1.0 * up_move_vec.xyz() * control_event_js.movement_y as f32 * 0.01;
        eye += 1.0 * right_move_vec.xyz() * control_event_js.movement_x as f32 * 0.01;
    }

    // Calculate eye location
    let on_wheel = control_event_js.on_wheel;
    if on_wheel {
        eye += -1.0 * direction.normalize() * control_event_js.wheel_delta_y as f32 * 0.005;
    }

    // Update
    scene_value.scene_variables.eye_location = eye;
    scene_value.scene_variables.eye_direction = direction;

    // Event context override
    control_event_js.on_click = false;
    control_event_js.on_wheel = false;
}
