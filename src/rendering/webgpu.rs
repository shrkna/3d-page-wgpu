use crate::engine::{self, define};
use crate::rendering::common;
use crate::types::Shared;
use wasm_bindgen::JsCast;
use wgpu::util::DeviceExt;
use wgpu::TextureViewDescriptor;

const WEBGPU_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const WEBGPU_FRONT_FACE: wgpu::FrontFace = wgpu::FrontFace::Ccw;
const WEBGPU_CULL_MODE: wgpu::Face = wgpu::Face::Back;

// --------------------------------------------------------------------------------------------

pub struct WebGPUInterface<'a> {
    pub surface: wgpu::Surface<'a>,
    pub _adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swapchain_format: wgpu::TextureFormat,
    pub depth_texture: wgpu::Texture,
}

pub struct WebGPUShaderResource {
    pub _shader_name: std::string::String,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub _bind_group_layout_2: Option<wgpu::BindGroupLayout>,
    pub uniform_size: u64,
}

pub struct WebGPURenderingResource {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub bind_group_2: Option<wgpu::BindGroup>,
    pub _base_color_texture: Option<wgpu::Texture>,
    pub base_color_texture_view: Option<wgpu::TextureView>,
    pub base_color_texture_sampler: Option<wgpu::Sampler>,
    pub _normal_texture: Option<wgpu::Texture>,
    pub normal_texture_view: Option<wgpu::TextureView>,
    pub normal_texture_sampler: Option<wgpu::Sampler>,
}

// Deprecated ---------------------------------------------------------------------------------

pub struct WebGPURenderingResourceDeprecated {
    pub _shader: wgpu::ShaderModule,
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub index_count: u32,
    pub bind_group: wgpu::BindGroup,
    pub _bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group_2: Option<wgpu::BindGroup>,
    pub uniform_buf: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

pub struct WebGPUDifferedResource {
    pub _shader: wgpu::ShaderModule,
    gbuffer_position_texture: wgpu::Texture,
    gbuffer_normal_texture: wgpu::Texture,
    gbuffer_albedo_texture: wgpu::Texture,
    gbuffer_metallic_roughness_texture: wgpu::Texture,
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub uniform_buf: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub debug_pipeline: wgpu::RenderPipeline,
}

// Webgpu contexts -----------------------------------------------------------------------------

pub async fn init_interface<'a>() -> WebGPUInterface<'a> {
    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .expect("Failed to get canvas element");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into()
        .expect("Failed to dynamic cast canvas element");

    let width: u32 = canvas.client_width() as u32;
    let height: u32 = canvas.client_height() as u32;

    // Initialize webgpu

    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface_target = wgpu::SurfaceTarget::Canvas(canvas);

    let surface: wgpu::Surface = instance
        .create_surface(surface_target)
        .expect("Failed to create surface from canvas");

    let adapter: wgpu::Adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to request adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
        .expect("Failed to request device");

    let swapchain_capabilities: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
    let swapchain_format: wgpu::TextureFormat = swapchain_capabilities.formats[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: width,
        height: height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    let depth_texture: wgpu::Texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size: wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: WEBGPU_DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    // Return webgpu resource

    let interface: WebGPUInterface<'_> = WebGPUInterface {
        surface,
        _adapter: adapter,
        device,
        queue,
        swapchain_format,
        depth_texture,
    };

    return interface;
}

pub fn create_shader_resource(interface: &WebGPUInterface) -> WebGPUShaderResource {
    struct PhongUniform {
        _transform_matrix: [f32; 16],
        _rotation_matrix: [f32; 16],
        _directional_light: [f32; 4],
        _ambient_light: [f32; 4],
        _inverse_matrix: [f32; 16],
        _buffer_type: [f32; 4],
    }

    let shader: wgpu::ShaderModule =
        interface
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../shader/phong.wgsl"
                ))),
            });

    let vertex_size: usize = std::mem::size_of::<common::Vertex>();
    let vertex_buffer_layout: [wgpu::VertexBufferLayout<'_>; 1] = [wgpu::VertexBufferLayout {
        array_stride: vertex_size as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 9]>() as u64,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 7]>() as u64,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 12]>() as u64,
                shader_location: 3,
            },
        ],
    }];

    let uniform_size: u64 = std::mem::size_of::<PhongUniform>() as u64;
    let bind_group_layout: wgpu::BindGroupLayout =
        interface
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(uniform_size),
                    },
                    count: None,
                }],
            });

    let bind_group_layout_2 =
        interface
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("bind_group_layout_2"),
            });

    let pipeline_layout: wgpu::PipelineLayout =
        interface
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout, &bind_group_layout_2],
                push_constant_ranges: &[],
            });

    let render_pipeline: wgpu::RenderPipeline =
        interface
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(define::VS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    buffers: &vertex_buffer_layout,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(define::FS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    targets: &[Some(interface.swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: WEBGPU_FRONT_FACE,
                    cull_mode: Some(WEBGPU_CULL_MODE),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: WEBGPU_DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

    return WebGPUShaderResource {
        _shader_name: "Phong".to_string(),
        bind_group_layout: bind_group_layout,
        _bind_group_layout_2: Some(bind_group_layout_2),
        render_pipeline: render_pipeline,
        uniform_size: uniform_size,
    };
}

pub fn create_rendering_resource(
    interface: &WebGPUInterface,
    mesh: &common::Mesh,
    materials: &Vec<engine::scene::SceneMaterial>,
) -> WebGPURenderingResource {
    let vertex_data: &Vec<common::Vertex> = &mesh.vertices;
    let index_data: &Vec<u32> = &mesh.indices;

    let vertex_buf: wgpu::Buffer =
        interface
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

    let index_buf: wgpu::Buffer =
        interface
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX,
            });

    let index_count: u32 = index_data.len() as u32;

    let base_color_texture_raw = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .base_color_texture;
    let base_color_texture_size = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .base_color_texture_size;
    let base_color_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("base color texture"),
            size: wgpu::Extent3d {
                width: base_color_texture_size[0],
                height: base_color_texture_size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

    interface.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &base_color_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &base_color_texture_raw,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * base_color_texture_size[0]),
            rows_per_image: Some(base_color_texture_size[1]),
        },
        wgpu::Extent3d {
            width: base_color_texture_size[0],
            height: base_color_texture_size[1],
            depth_or_array_layers: 1,
        },
    );

    let base_color_texture_view: wgpu::TextureView =
        base_color_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let base_color_texture_sampler: wgpu::Sampler =
        interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

    let normal_texture_raw = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .normal_texture;
    let normal_texture_size = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .normal_texture_size;
    let normal_texture: wgpu::Texture = interface.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("normal texture"),
        size: wgpu::Extent3d {
            width: normal_texture_size[0],
            height: normal_texture_size[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    interface.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &normal_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &normal_texture_raw,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * normal_texture_size[0]),
            rows_per_image: Some(normal_texture_size[1]),
        },
        wgpu::Extent3d {
            width: normal_texture_size[0],
            height: normal_texture_size[1],
            depth_or_array_layers: 1,
        },
    );

    let normal_texture_view: wgpu::TextureView =
        normal_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let normal_texture_sampler: wgpu::Sampler =
        interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

    return WebGPURenderingResource {
        vertex_buffer: vertex_buf,
        index_buffer: index_buf,
        index_count: index_count,
        uniform_buffer: None,
        bind_group: None,
        bind_group_2: None,
        _base_color_texture: Some(base_color_texture),
        base_color_texture_view: Some(base_color_texture_view),
        base_color_texture_sampler: Some(base_color_texture_sampler),
        _normal_texture: Some(normal_texture),
        normal_texture_view: Some(normal_texture_view),
        normal_texture_sampler: Some(normal_texture_sampler),
    };
}

// Forward rendering ---------------------------------------------------------------------------

pub fn update_phong_shading(
    interface: &WebGPUInterface,
    shader_resource: &WebGPUShaderResource,
    scene: &Shared<engine::scene::Scene>,
    object: &engine::scene::SceneObject,
) {
    if object.rendering_resource.is_none() {
        log::debug!("[Update] Rendering resource is empty");
        return;
    }

    if object
        .rendering_resource
        .as_ref()
        .unwrap()
        .borrow()
        .uniform_buffer
        .is_none()
    {
        let uniform_buf: wgpu::Buffer = interface.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: shader_resource.uniform_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        object
            .rendering_resource
            .as_ref()
            .unwrap()
            .borrow_mut()
            .uniform_buffer = Some(uniform_buf);
    }

    if object
        .rendering_resource
        .as_ref()
        .unwrap()
        .borrow()
        .bind_group
        .is_none()
    {
        let bind_group: wgpu::BindGroup =
            interface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &shader_resource.bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: object
                            .rendering_resource
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .uniform_buffer
                            .as_ref()
                            .unwrap()
                            .as_entire_binding(),
                    }],
                    label: Some("Bind group 0"),
                });
        object
            .rendering_resource
            .as_ref()
            .unwrap()
            .borrow_mut()
            .bind_group = Some(bind_group);
    }

    if object
        .rendering_resource
        .as_ref()
        .unwrap()
        .borrow()
        .bind_group_2
        .is_none()
    {
        let bind_group_2 = interface
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &shader_resource
                    ._bind_group_layout_2
                    .as_ref()
                    .expect("[Update] Bind group layout 2"),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &object
                                .rendering_resource
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .base_color_texture_view
                                .as_ref()
                                .unwrap(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &object
                                .rendering_resource
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .base_color_texture_sampler
                                .as_ref()
                                .unwrap(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &object
                                .rendering_resource
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .normal_texture_view
                                .as_ref()
                                .unwrap(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(
                            &object
                                .rendering_resource
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .normal_texture_sampler
                                .as_ref()
                                .unwrap(),
                        ),
                    },
                ],
                label: Some("texture_bind_group"),
            });

        object
            .rendering_resource
            .as_ref()
            .unwrap()
            .borrow_mut()
            .bind_group_2 = Some(bind_group_2);
    }

    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let width: u32 = canvas.client_width() as u32;
    let height: u32 = canvas.client_height() as u32;
    let aspect_ratio: f32 = width as f32 / height as f32;

    let scene_value = scene.borrow();

    let eye: glam::Vec3 = scene_value.variables.eye_location;
    let direction: glam::Vec3 = scene_value.variables.eye_direction;

    let mut model_matrix = glam::Mat4::from_cols_array_2d(&object.world_transform);

    // Force Y-up to Z-up
    if scene_value.variables.convert_y_to_z {
        let y_to_z_mat: glam::Mat4 =
            glam::Mat4::from_axis_angle(glam::Vec3::new(1.0, 0.0, 0.0), std::f32::consts::PI / 2.0);
        model_matrix = y_to_z_mat * model_matrix;
    }

    let view_matrix = glam::Mat4::look_to_rh(eye, direction, glam::Vec3::Z);
    let projection_matrix: glam::Mat4 =
        glam::Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, aspect_ratio, 0.01, 100.0);
    let transform_matrix: glam::Mat4 = projection_matrix * view_matrix * model_matrix;

    let directional: [f32; 3] = scene_value.variables.directional_light_angle;
    let ambient: [f32; 4] = scene_value.variables.ambient_light_color;
    let inverse_projection: glam::Mat4 = transform_matrix.inverse();

    let rotaton_matrix: glam::Mat4 =
        glam::Mat4::from_quat(model_matrix.to_scale_rotation_translation().1);

    let mut uniform_total: Vec<f32> = transform_matrix.to_cols_array().to_vec();
    uniform_total.extend_from_slice(&rotaton_matrix.to_cols_array().to_vec());
    uniform_total.extend_from_slice(&directional);
    uniform_total.extend_from_slice(&[0.0]); // Padding!
    uniform_total.extend_from_slice(&ambient);
    uniform_total.extend_from_slice(&inverse_projection.to_cols_array().to_vec());
    uniform_total.extend_from_slice(&[
        scene_value.variables.differed_debug_type as f32,
        0.0,
        0.0,
        0.0,
    ]);

    let uniform_ref: &[f32] = uniform_total.as_ref();
    interface.queue.write_buffer(
        &object
            .rendering_resource
            .as_ref()
            .unwrap()
            .borrow()
            .uniform_buffer
            .as_ref()
            .unwrap(),
        0,
        bytemuck::cast_slice(uniform_ref),
    );
}

pub fn render_forward_shading_main(
    interface: &WebGPUInterface,
    shader_resource: &WebGPUShaderResource,
    scene: &Shared<engine::scene::Scene>,
) {
    let frame: wgpu::SurfaceTexture = interface
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    let view: wgpu::TextureView = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let depth_texture_view: wgpu::TextureView =
        interface
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("depth texture view"),
                format: Some(WEBGPU_DEPTH_FORMAT),
                aspect: wgpu::TextureAspect::All,
                base_array_layer: 0,
                array_layer_count: Some(1),
                base_mip_level: 0,
                mip_level_count: Some(1),
                dimension: Some(wgpu::TextureViewDimension::D2),
            });

    let mut encoder: wgpu::CommandEncoder =
        interface
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Forward render encoder"),
            });

    {
        let scene_value = scene.borrow();

        let mut rpass: wgpu::RenderPass<'_> =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Forward render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: scene_value.variables.background_color[0] as f64,
                            g: scene_value.variables.background_color[1] as f64,
                            b: scene_value.variables.background_color[2] as f64,
                            a: scene_value.variables.background_color[3] as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        for object in scene.borrow().objects.iter() {
            if object.rendering_resource.is_none() {
                continue;
            }

            rpass.set_pipeline(&shader_resource.render_pipeline);
            rpass.set_bind_group(
                0,
                &object
                    .rendering_resource
                    .as_ref()
                    .expect("[draw] Rendering resource is empty")
                    .borrow()
                    .bind_group,
                &[],
            );
            rpass.set_bind_group(
                1,
                &object
                    .rendering_resource
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .bind_group_2,
                &[],
            );
            rpass.set_index_buffer(
                object
                    .rendering_resource
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .index_buffer
                    .slice(..),
                wgpu::IndexFormat::Uint32,
            );
            rpass.set_vertex_buffer(
                0,
                object
                    .rendering_resource
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .vertex_buffer
                    .slice(..),
            );
            rpass.draw_indexed(
                0..object
                    .rendering_resource
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .index_count,
                0,
                0..1,
            );
        }
    }

    interface.queue.submit(Some(encoder.finish()));
    frame.present();
}

// Differed shading -----------------------------------------------------------------------------

pub fn init_differed_gbuffer_pipeline(
    interface: &WebGPUInterface,
    scene: &Shared<engine::scene::Scene>,
) {
    struct InitMap {
        index: usize,
        resource: WebGPURenderingResourceDeprecated,
    }
    // Create render resources
    let mut init_list: Vec<InitMap> = Vec::new();
    {
        let scene_borrow = scene.borrow();
        let scene_mterials = &scene_borrow.materials;

        let render_objects: Vec<engine::scene::SceneObject> = if scene_borrow.variables.use_batched
        {
            scene_borrow.batched_objects.clone()
        } else {
            scene_borrow.objects.clone()
        };

        for i in 0..render_objects.len() {
            let object_borrow = render_objects.get(i).unwrap();
            if object_borrow.shading_type != 0 && object_borrow.source_mesh.is_some() {
                init_list.push(InitMap {
                    index: i,
                    resource: init_differed_gbuffers_shading(
                        &interface,
                        &object_borrow.source_mesh.as_ref().unwrap().borrow(),
                        &scene_mterials,
                    ),
                });
            }
        }
    }

    // Initialize render pipeline
    for init_elem in init_list {
        let mut scene_borrow = scene.borrow_mut();

        let render_objects: &mut Vec<engine::scene::SceneObject> =
            if scene_borrow.variables.use_batched {
                &mut scene_borrow.batched_objects
            } else {
                &mut scene_borrow.objects
            };

        let object_borrow = render_objects.get_mut(init_elem.index).unwrap();
        object_borrow.shading_type = 0;
        object_borrow.render_resource_deprecated = Some(std::rc::Rc::new(std::cell::RefCell::new(
            init_elem.resource,
        )));
    }
}

fn init_differed_gbuffers_shading(
    interface: &WebGPUInterface,
    mesh: &common::Mesh,
    materials: &Vec<engine::scene::SceneMaterial>,
) -> WebGPURenderingResourceDeprecated {
    struct WriteGBuffersUniform {
        _model_matrix: [f32; 16],
        _view_matrix: [f32; 16],
        _projection_matrix: [f32; 16],
        _rotation_matrix: [f32; 16],
    }
    let shader: wgpu::ShaderModule =
        interface
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../shader/differed_write_gbuffers.wgsl"
                ))),
            });

    let vertex_size: usize = std::mem::size_of::<common::Vertex>();
    let vertex_data: &Vec<common::Vertex> = &mesh.vertices;
    let index_data: &Vec<u32> = &mesh.indices;

    let vertex_buf: wgpu::Buffer =
        interface
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

    let index_buf: wgpu::Buffer =
        interface
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX,
            });

    // Textures : warning write texture is slow

    let base_color_texture_raw = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .base_color_texture;
    let base_color_texture_size = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .base_color_texture_size;
    let base_color_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("base color texture"),
            size: wgpu::Extent3d {
                width: base_color_texture_size[0],
                height: base_color_texture_size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

    interface.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &base_color_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &base_color_texture_raw,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * base_color_texture_size[0]),
            rows_per_image: Some(base_color_texture_size[1]),
        },
        wgpu::Extent3d {
            width: base_color_texture_size[0],
            height: base_color_texture_size[1],
            depth_or_array_layers: 1,
        },
    );

    let base_color_texture_view: wgpu::TextureView =
        base_color_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let base_color_texture_sampler: wgpu::Sampler =
        interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

    let normal_texture_raw = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .normal_texture;
    let normal_texture_size = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .normal_texture_size;
    let normal_texture: wgpu::Texture = interface.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("normal texture"),
        size: wgpu::Extent3d {
            width: normal_texture_size[0],
            height: normal_texture_size[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    interface.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &normal_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &normal_texture_raw,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * normal_texture_size[0]),
            rows_per_image: Some(normal_texture_size[1]),
        },
        wgpu::Extent3d {
            width: normal_texture_size[0],
            height: normal_texture_size[1],
            depth_or_array_layers: 1,
        },
    );

    let normal_texture_view: wgpu::TextureView =
        normal_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let normal_texture_sampler: wgpu::Sampler =
        interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

    let metallic_texture_raw = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .metallic_roughness_texture;
    let metallic_texture_size = &materials
        .get(mesh.material.unwrap() as usize)
        .unwrap()
        .metallic_roughness_texture_size;
    let metallic_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("metallic roughness texture"),
            size: wgpu::Extent3d {
                width: metallic_texture_size[0],
                height: metallic_texture_size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

    interface.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &metallic_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &metallic_texture_raw,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * metallic_texture_size[0]),
            rows_per_image: Some(metallic_texture_size[1]),
        },
        wgpu::Extent3d {
            width: metallic_texture_size[0],
            height: metallic_texture_size[1],
            depth_or_array_layers: 1,
        },
    );

    let metallic_texture_view: wgpu::TextureView =
        metallic_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let metallic_texture_sampler: wgpu::Sampler =
        interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

    // bindings

    let uniform_size: u64 = std::mem::size_of::<WriteGBuffersUniform>() as u64;
    let uniform_buf: wgpu::Buffer = interface.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Uniform Buffer"),
        size: uniform_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let uniform_bind_group_layout: wgpu::BindGroupLayout = interface
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let uniform_bind_group: wgpu::BindGroup =
        interface
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }],
                label: Some("Bind group 0"),
            });

    let texture_bind_group_layout =
        interface
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

    let texture_bind_group = interface
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&base_color_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&base_color_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&metallic_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&metallic_texture_sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

    // pipeline

    let pipeline_layout: wgpu::PipelineLayout =
        interface
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

    let vertex_buffers: [wgpu::VertexBufferLayout<'_>; 1] = [wgpu::VertexBufferLayout {
        array_stride: vertex_size as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 9]>() as u64,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 7]>() as u64,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 12]>() as u64,
                shader_location: 3,
            },
        ],
    }];

    let render_pipeline: wgpu::RenderPipeline =
        interface
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(define::VS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    buffers: &vertex_buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(define::FS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::all(),
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::all(),
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::all(),
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::all(),
                        }),
                    ],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(WEBGPU_CULL_MODE),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: WEBGPU_DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });
    let index_count: u32 = index_data.len() as u32;

    let render_resource: WebGPURenderingResourceDeprecated = WebGPURenderingResourceDeprecated {
        _shader: shader,
        vertex_buf,
        index_buf,
        index_count,
        bind_group: uniform_bind_group,
        _bind_group_layout: uniform_bind_group_layout,
        bind_group_2: Some(texture_bind_group),
        uniform_buf,
        render_pipeline,
    };

    return render_resource;
}

fn update_differed_gbuffers_shading(
    scene: &Shared<engine::scene::Scene>,
    interface: &WebGPUInterface,
    object: &engine::scene::SceneObject,
) {
    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let width: u32 = canvas.client_width() as u32;
    let height: u32 = canvas.client_height() as u32;
    let aspect_ratio: f32 = width as f32 / height as f32;

    let scene_value = scene.borrow();
    let eye: glam::Vec3 = scene_value.variables.eye_location;
    let direction: glam::Vec3 = scene_value.variables.eye_direction;

    let mut model_matrix = glam::Mat4::from_cols_array_2d(&object.world_transform);

    // Force Y-up to Z-up
    if scene_value.variables.convert_y_to_z {
        let y_to_z_mat: glam::Mat4 =
            glam::Mat4::from_axis_angle(glam::Vec3::new(1.0, 0.0, 0.0), std::f32::consts::PI / 2.0);
        model_matrix = y_to_z_mat * model_matrix;
    }

    // Create matrices and write buffer
    let view_matrix = glam::Mat4::look_to_rh(eye, direction, glam::Vec3::Z);
    let projection_matrix: glam::Mat4 =
        glam::Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, aspect_ratio, 0.01, 100.0);

    let rotaton_matrix: glam::Mat4 =
        glam::Mat4::from_quat(model_matrix.to_scale_rotation_translation().1);

    let mut uniform_total = model_matrix.to_cols_array().to_vec();
    uniform_total.extend_from_slice(&view_matrix.to_cols_array());
    uniform_total.extend_from_slice(&projection_matrix.to_cols_array());
    uniform_total.extend_from_slice(&rotaton_matrix.to_cols_array());
    let uniform_ref: &[f32] = uniform_total.as_ref();
    interface.queue.write_buffer(
        &object
            .render_resource_deprecated
            .as_ref()
            .unwrap()
            .borrow()
            .uniform_buf,
        0,
        bytemuck::cast_slice(uniform_ref),
    );
}

pub fn init_differed_pipeline(interface: &WebGPUInterface) -> WebGPUDifferedResource {
    struct DifferedUniform {
        _directional_light: [f32; 4],
        _ambient_light: [f32; 4],
        _inverse_matrix: [f32; 16],
        _debug: DifferedDebugUniform,
    }

    struct DifferedDebugUniform {
        _buffer_type: f32,
        _padding: [f32; 3],
    }

    let shader: wgpu::ShaderModule =
        interface
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../shader/differed.wgsl"
                ))),
            });

    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .expect("Failed to get canvas element");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into()
        .expect("Failed to dynamic cast canvas element");

    let width: u32 = canvas.client_width() as u32;
    let height: u32 = canvas.client_height() as u32;

    // Gbuffers

    let gbuffer_position_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("position texture"),
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

    let gbuffer_normal_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("normal texture"),
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

    let gbuffer_albedo_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("albedo texture"),
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

    let gbuffer_metallic_roughness_texture: wgpu::Texture =
        interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("metallic roughness texture"),
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

    // bindings

    let gbuffer_bind_group_layout: wgpu::BindGroupLayout = interface
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

    let gbuffer_bind_group: wgpu::BindGroup =
        interface
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &gbuffer_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &gbuffer_position_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &gbuffer_normal_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &interface
                                .depth_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(
                            &gbuffer_albedo_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(
                            &gbuffer_metallic_roughness_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                ],
                label: Some("Bind group 0"),
            });

    let uniform_size: u64 = std::mem::size_of::<DifferedUniform>() as u64;
    let uniform_buf: wgpu::Buffer = interface.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Differed uniform buffer"),
        size: uniform_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let uniform_bind_group_layout: wgpu::BindGroupLayout = interface
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let uniform_bind_group: wgpu::BindGroup =
        interface
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }],
                label: Some("Bind group 1"),
            });

    // pipeline

    let pipeline_layout: wgpu::PipelineLayout =
        interface
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&gbuffer_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

    let render_pipeline: wgpu::RenderPipeline =
        interface
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(define::VS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(engine::define::FS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    targets: &[Some(interface.swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(WEBGPU_CULL_MODE),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

    let debug_pipeline: wgpu::RenderPipeline =
        interface
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(define::VS_ENTRY_POINT),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_debug_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(interface.swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(WEBGPU_CULL_MODE),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

    let mut bind_groups: Vec<wgpu::BindGroup> = Vec::new();
    bind_groups.push(gbuffer_bind_group);
    bind_groups.push(uniform_bind_group);

    let resource: WebGPUDifferedResource = WebGPUDifferedResource {
        _shader: shader,
        gbuffer_position_texture,
        gbuffer_normal_texture,
        gbuffer_albedo_texture,
        gbuffer_metallic_roughness_texture,
        bind_groups,
        uniform_buf,
        render_pipeline,
        debug_pipeline,
    };

    return resource;
}

pub fn update_differed_buffer(
    scene: &Shared<engine::scene::Scene>,
    interface: &WebGPUInterface,
    resource: &WebGPUDifferedResource,
) {
    let canvas: web_sys::Element = gloo::utils::document()
        .get_element_by_id(define::CANVAS_ELEMENT_ID)
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let width: u32 = canvas.client_width() as u32;
    let height: u32 = canvas.client_height() as u32;
    let aspect_ratio: f32 = width as f32 / height as f32;

    let scene_value = scene.borrow();

    let eye: glam::Vec3 = scene_value.variables.eye_location;
    let direction: glam::Vec3 = scene_value.variables.eye_direction;

    // Create matrices and write buffer
    let view_matrix = glam::Mat4::look_to_rh(eye, direction, glam::Vec3::Z);
    let projection_matrix: glam::Mat4 =
        glam::Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, aspect_ratio, 0.01, 100.0);
    let transform_matrix: glam::Mat4 = projection_matrix * view_matrix;

    let directional: [f32; 3] = scene_value.variables.directional_light_angle;
    let ambient: [f32; 4] = scene_value.variables.ambient_light_color;
    let inverse_projection: glam::Mat4 = transform_matrix.inverse();

    let mut uniform_total: Vec<f32> = Vec::new();
    uniform_total.extend_from_slice(&directional);
    uniform_total.extend_from_slice(&[0.0]); // Padding!
    uniform_total.extend_from_slice(&ambient);
    uniform_total.extend_from_slice(&inverse_projection.to_cols_array().to_vec());
    uniform_total.extend_from_slice(&[
        scene_value.variables.differed_debug_type as f32,
        0.0,
        0.0,
        0.0,
    ]);

    let uniform_ref: &[f32] = uniform_total.as_ref();
    interface
        .queue
        .write_buffer(&resource.uniform_buf, 0, bytemuck::cast_slice(uniform_ref));
}

pub fn update_differed_shading(
    interface: &WebGPUInterface,
    scene: &Shared<engine::scene::Scene>,
    differed_resource: &WebGPUDifferedResource,
) {
    // Update gbuffer
    if scene.borrow().variables.use_batched == false {
        for scene_object in &scene.as_ref().borrow().objects {
            if scene_object.shading_type == 0 {
                update_differed_gbuffers_shading(&scene, &interface, &scene_object);
            }
        }
    } else {
        for batched in &scene.as_ref().borrow().batched_objects {
            if batched.shading_type == 0 {
                update_differed_gbuffers_shading(&scene, &interface, &batched);
            }
        }
    }
    // Update differed
    update_differed_buffer(&scene, &interface, &differed_resource);
}

pub fn render_differed_shading_main(
    interface: &WebGPUInterface,
    scene: &Shared<engine::scene::Scene>,
    differed_resource: &WebGPUDifferedResource,
) {
    let frame: wgpu::SurfaceTexture = interface
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    let view: wgpu::TextureView = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: wgpu::CommandEncoder =
        interface
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Differed render encoder"),
            });

    // gbuffer pass
    {
        let mut gbuffer_pass: wgpu::RenderPass<'_> =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Differed gbuffer pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &differed_resource
                            .gbuffer_position_texture
                            .create_view(&TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &differed_resource
                            .gbuffer_normal_texture
                            .create_view(&TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 1.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &differed_resource
                            .gbuffer_albedo_texture
                            .create_view(&TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &differed_resource
                            .gbuffer_metallic_roughness_texture
                            .create_view(&TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &interface
                        .depth_texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        if scene.borrow().variables.use_batched == false {
            for object in scene.borrow().objects.iter() {
                if object.shading_type == 0 {
                    gbuffer_pass.set_pipeline(
                        &object
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .render_pipeline,
                    );
                    gbuffer_pass.set_bind_group(
                        0,
                        &object
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .bind_group,
                        &[],
                    );
                    if object
                        .render_resource_deprecated
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .bind_group_2
                        .is_some()
                    {
                        gbuffer_pass.set_bind_group(
                            1,
                            &object
                                .render_resource_deprecated
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .bind_group_2,
                            &[],
                        );
                    }
                    gbuffer_pass.set_index_buffer(
                        object
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .index_buf
                            .slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    gbuffer_pass.set_vertex_buffer(
                        0,
                        object
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .vertex_buf
                            .slice(..),
                    );
                    gbuffer_pass.draw_indexed(
                        0..object
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .index_count,
                        0,
                        0..1,
                    );
                }
            }
        } else {
            for batched in scene.borrow().batched_objects.iter() {
                if batched.shading_type == 0 {
                    gbuffer_pass.set_pipeline(
                        &batched
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .render_pipeline,
                    );
                    gbuffer_pass.set_bind_group(
                        0,
                        &batched
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .bind_group,
                        &[],
                    );
                    if batched
                        .render_resource_deprecated
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .bind_group_2
                        .is_some()
                    {
                        gbuffer_pass.set_bind_group(
                            1,
                            &batched
                                .render_resource_deprecated
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .bind_group_2,
                            &[],
                        );
                    }
                    gbuffer_pass.set_index_buffer(
                        batched
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .index_buf
                            .slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    gbuffer_pass.set_vertex_buffer(
                        0,
                        batched
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .vertex_buf
                            .slice(..),
                    );
                    gbuffer_pass.draw_indexed(
                        0..batched
                            .render_resource_deprecated
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .index_count,
                        0,
                        0..1,
                    );
                }
            }
        }
    }

    // differed pass
    {
        let scene_value = scene.borrow();

        let mut differed_pass: wgpu::RenderPass<'_> =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Differed render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: scene_value.variables.background_color[0] as f64,
                            g: scene_value.variables.background_color[1] as f64,
                            b: scene_value.variables.background_color[2] as f64,
                            a: scene_value.variables.background_color[3] as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        if scene_value.variables.differed_debug_type == 0 {
            differed_pass.set_pipeline(&differed_resource.render_pipeline);
        } else {
            differed_pass.set_pipeline(&differed_resource.debug_pipeline);
        }

        differed_pass.set_bind_group(0, &differed_resource.bind_groups[0], &[]);
        differed_pass.set_bind_group(1, &differed_resource.bind_groups[1], &[]);
        differed_pass.draw(0..6, 0..1);
    }

    interface.queue.submit(Some(encoder.finish()));
    frame.present();
}
