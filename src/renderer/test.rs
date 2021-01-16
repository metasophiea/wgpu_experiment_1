use rand::prelude::*;

use wgpu::util::DeviceExt;

use crate::library::data_type::{
    Colour,
    Point,
    Dimensions,
};

use super::Renderer;
use super::library;








#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexUniformData {
    xy: [f32; 2],
    scale: f32,
    angle: f32,
    dimensions: [f32; 2],
    width: f32, height: f32,
    anchor: [f32; 2],
}
impl VertexUniformData {
    pub fn new(
        x: f32,
        y: f32, 
        scale: f32, 
        angle: f32,
        dimensions: Dimensions<u32>,
        width: f32, 
        height: f32,
        anchor: Point,
    ) -> Self {
        Self {
            xy: [x, y],
            scale: scale,
            angle: angle,
            dimensions: [*dimensions.get_width() as f32, *dimensions.get_height() as f32],
            width: width,
            height: height,
            anchor: [anchor.get_x(), anchor.get_y()],
        }
    }
}
impl VertexUniformData {
    pub fn update(
        &mut self,
        x: Option<f32>,
        y: Option<f32>,
        scale: Option<f32>,
        angle: Option<f32>,
        dimensions: Option<(u32, u32)>,
        width: Option<f32>,
        height: Option<f32>,
        anchor: Option<Point>,
    ) {
        if let Some(x) = x { self.xy[0] = x; }
        if let Some(y) = y { self.xy[1] = y; }
        if let Some(angle) = angle { self.angle = angle; }
        if let Some(scale) = scale { self.scale = scale; }
        if let Some(width) = width { self.width = width; }
        if let Some(height) = height { self.height = height; }
        if let Some(anchor) = anchor { self.anchor = [anchor.get_x(), anchor.get_y()]; }
    }
    pub fn update_dimensions(&mut self, dimensions:&Dimensions<u32>) {
        self.dimensions = [*dimensions.get_width() as f32, *dimensions.get_height() as f32];
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FragmentUniformData {
    colour: [f32; 4],
}
impl FragmentUniformData {
    pub fn new(colour: Colour) -> FragmentUniformData {
        Self {
            colour: [colour.r(), colour.g(), colour.b(), colour.a()]
        }
    }
}
impl FragmentUniformData {
    pub fn update(
        &mut self,
        colour: Colour,
    ) {
        self.colour[0] = colour.premultiplied_r();
        self.colour[1] = colour.premultiplied_g();
        self.colour[2] = colour.premultiplied_b();
        self.colour[3] = colour.a();
    }
}




impl Renderer {
    pub fn test1(&mut self) {
        println!("- test 1 -");

        let rectangle_count = 1_000;

        //adapter
            let adapter = futures::executor::block_on(
                self.instance.request_adapter(
                    &wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::Default,
                        compatible_surface: Some(&self.surface),
                    }
                )
            ).unwrap();

        //device and queue
            let (device, queue) = futures::executor::block_on(
                adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        shader_validation: true,
                    },
                    None,
                )
            ).unwrap();

        //swap chain
            let swap_chain_descriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: self.size.width,
                height: self.size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            let mut swap_chain = device.create_swap_chain( &self.surface, &swap_chain_descriptor );

        //framebuffer
            let multisampled_texture_extent = wgpu::Extent3d {
                width: swap_chain_descriptor.width,
                height: swap_chain_descriptor.height,
                depth: 1,
            };
            let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
                size: multisampled_texture_extent,
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format: swap_chain_descriptor.format,
                usage: wgpu::TextureUsage::all(),
                label: Some("multisampled framebuffer"),
            };
            let framebuffer = device.create_texture(multisampled_frame_descriptor).create_view(&wgpu::TextureViewDescriptor::default());

        //create shader modules
            let vertex_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.vert.spv") );
            let fragment_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.frag.spv") );


        //uniforms
            let uniform_bind_group_layout = device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(&"Uniform Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::VERTEX,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                }
            );

            let mut rng = rand::thread_rng();

            let mut uniform_bind_group_vector:Vec<wgpu::BindGroup> = vec![];
            for index in 0..rectangle_count {
                //uniform buffers
                    //vertex
                        let vertex_data = VertexUniformData::new(
                            500.0*rng.gen::<f32>(), //x
                            400.0*rng.gen::<f32>(), //y
                            1.0, //scale
                            2.0 * std::f32::consts::PI * rng.gen::<f32>(), //angle
                            Dimensions::<u32>::new(500,400), //dimensions
                            30.0, //width
                            30.0, //height
                            Point::new(0.5,0.5), //anchor
                        );
                        let vertex_uniform_printing_buffer = device.create_buffer(
                            &wgpu::BufferDescriptor {
                                label: Some(&"Vertex Uniform Printing Buffer"),
                                size: std::mem::size_of::<VertexUniformData>() as wgpu::BufferAddress,
                                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                                mapped_at_creation: false,
                            }
                        );
                        queue.write_buffer(
                            &vertex_uniform_printing_buffer, 
                            0, 
                            bytemuck::cast_slice(&[vertex_data])
                        );
                    //fragment
                        let fragment_data = FragmentUniformData::new(
                            Colour::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0),
                        );
                        let fragment_uniform_printing_buffer = device.create_buffer(
                            &wgpu::BufferDescriptor {
                                label: Some(&"Fragment Uniform Printing Buffer"),
                                size: std::mem::size_of::<FragmentUniformData>() as wgpu::BufferAddress,
                                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                                mapped_at_creation: false,
                            }
                        );
                        queue.write_buffer(
                            &fragment_uniform_printing_buffer, 
                            0, 
                            bytemuck::cast_slice(&[fragment_data])
                        );

                //uniform bind group
                    let uniform_bind_group = device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(&"Uniform Bind Group"),
                            layout: &uniform_bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::Buffer(vertex_uniform_printing_buffer.slice(..))
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::Buffer(fragment_uniform_printing_buffer.slice(..))
                                }
                            ],
                        }
                    );

                //add to vector
                    uniform_bind_group_vector.push(
                        uniform_bind_group
                    );
            }
            
            println!("renderer >> uniforms created");

        //vertex buffer
            let rectangle_point: [f32; 8] = [
                0.0,0.0,
                1.0,0.0,
                1.0,1.0,
                0.0,1.0,
            ];
            let triangle_list_points = [
                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[1*2 + 0], rectangle_point[1*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],

                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],
                rectangle_point[3*2 + 0], rectangle_point[3*2 + 1],
            ];

            let mut vertex_point_data:Vec<library::Vertex> = vec![];
            for index in (0..triangle_list_points.len()).step_by(2) {
                vertex_point_data.push(
                    library::Vertex::new(
                        [
                            triangle_list_points[index + 0],
                            triangle_list_points[index + 1],
                        ]
                    ),
                );
            }
            
            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&"Vertex Buffer"),
                    contents: bytemuck::cast_slice(&*vertex_point_data),
                    usage: wgpu::BufferUsage::VERTEX,
                }
            );

        //render pipelines
            //layout
                let render_pipeline_layout = device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some(&"Render Pipeline Layout"),
                        bind_group_layouts: &[
                            &uniform_bind_group_layout
                        ],
                        push_constant_ranges: &[],
                    }
                );  
            //pipelines
                let render_pipeline = device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
                        label: Some(&"Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex_stage: wgpu::ProgrammableStageDescriptor {
                            module: &vertex_shader_module,
                            entry_point: "main",
                        },
                        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                            module: &fragment_shader_module,
                            entry_point: "main",
                        }),
                        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                            front_face: wgpu::FrontFace::Cw,
                            cull_mode: wgpu::CullMode::Back,
                            depth_bias: 0,
                            depth_bias_slope_scale: 0.0,
                            depth_bias_clamp: 0.0,
                            clamp_depth: false,
                        }),
                        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                        color_states: &[wgpu::ColorStateDescriptor {
                            format: swap_chain_descriptor.format,
                            color_blend: wgpu::BlendDescriptor {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                        depth_stencil_state: None,
                        vertex_state: wgpu::VertexStateDescriptor {
                            index_format: wgpu::IndexFormat::Uint16,
                            vertex_buffers: &[library::Vertex::desc()],
                        },
                        sample_count: 4,
                        sample_mask: !0,
                        alpha_to_coverage_enabled: false,
                    }
                );

        //setup frame
            let frame = match swap_chain.get_current_frame() {
                Err(e) => {
                    println!("ERROR - Renderer : swap_chain.get_current_frame failed! {}", e);
                    return;
                },
                Ok(frame) => frame.output,
            };
    
        //create encoder
            let mut encoder = device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Renderer : Command Encoder"),
                }
            );

        //clear frame
            {
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &framebuffer,
                                resolve_target: Some(&frame.view),
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 1.0,
                                            g: 1.0,
                                            b: 1.0,
                                            a: 1.0,
                                        }
                                    ),
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );
            }

        println!("renderer >> beginning main render pass");

        //begin main render pass
            let start_time = std::time::Instant::now();
            {
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &framebuffer,
                                resolve_target: Some(&frame.view),
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );

                //set pipeline
                    render_pass.set_pipeline(&render_pipeline);
                //set vertex buffer
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                // for round in 0..70 {
                    for index in 0..uniform_bind_group_vector.len() {
                        //set uniform buffer group
                            render_pass.set_bind_group(0, &uniform_bind_group_vector[index], &[]);
                        //draw
                            render_pass.draw(0..vertex_point_data.len() as u32, 0..1);
                    }
                // }
            }
            let end_time = std::time::Instant::now();
            println!("{:?}", end_time.checked_duration_since(start_time) );

        //submit encoder to queue, to be rendered
            queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn test2(&mut self) {
        //better buffer usage

        println!("- test 2 -");

        let rectangle_count:usize = 1_400_000;

        //adapter
            let adapter = futures::executor::block_on(
                self.instance.request_adapter(
                    &wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::Default,
                        compatible_surface: Some(&self.surface),
                    }
                )
            ).unwrap();

        //device and queue
            let (device, queue) = futures::executor::block_on(
                adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        shader_validation: true,
                    },
                    None,
                )
            ).unwrap();

        //swap chain
            let swap_chain_descriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: self.size.width,
                height: self.size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            let mut swap_chain = device.create_swap_chain( &self.surface, &swap_chain_descriptor );

        //framebuffer
            let multisampled_texture_extent = wgpu::Extent3d {
                width: swap_chain_descriptor.width,
                height: swap_chain_descriptor.height,
                depth: 1,
            };
            let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
                size: multisampled_texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: swap_chain_descriptor.format,
                usage: wgpu::TextureUsage::all(),
                label: Some("multisampled framebuffer"),
            };
            let framebuffer = device.create_texture(multisampled_frame_descriptor).create_view(&wgpu::TextureViewDescriptor::default());

        //create shader modules
            let vertex_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.vert.spv") );
            let fragment_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.frag.spv") );


        //uniforms
            //uniform buffers
                let mut rng = rand::thread_rng();

                //vertex
                    let vertex_uniform_buffer = device.create_buffer(
                        &wgpu::BufferDescriptor {
                            label: Some(&"Vertex Uniform Printing Buffer"),
                            size: rectangle_count as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT,
                            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            mapped_at_creation: false,
                        }
                    );
                    for index in 0..rectangle_count {
                        let vertex_data = VertexUniformData::new(
                            500.0*rng.gen::<f32>(), //x
                            400.0*rng.gen::<f32>(), //y
                            1.0, //scale
                            2.0 * std::f32::consts::PI * rng.gen::<f32>(), //angle
                            Dimensions::<u32>::new(500,400), //dimensions
                            30.0, //width
                            30.0, //height
                            Point::new(0.5,0.5), //anchor
                        );
                        queue.write_buffer(
                            &vertex_uniform_buffer, 
                            wgpu::BIND_BUFFER_ALIGNMENT * index as wgpu::BufferAddress,
                            bytemuck::cast_slice(&[vertex_data])
                        );
                    }

                //fragment
                    let fragment_uniform_buffer = device.create_buffer(
                        &wgpu::BufferDescriptor {
                            label: Some(&"Fragment Uniform Printing Buffer"),
                            size: rectangle_count as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT,
                            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            mapped_at_creation: false,
                        }
                    );
                    for index in 0..rectangle_count {
                        let fragment_data = FragmentUniformData::new(
                            Colour::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0),
                        );
                        queue.write_buffer(
                            &fragment_uniform_buffer, 
                            wgpu::BIND_BUFFER_ALIGNMENT * index as wgpu::BufferAddress,
                            bytemuck::cast_slice(&[fragment_data])
                        );
                    }

            //uniform bind group
                let uniform_bind_group_layout = device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: Some(&"Uniform Bind Group Layout"),
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStage::VERTEX,
                                ty: wgpu::BindingType::UniformBuffer {
                                    dynamic: true,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStage::FRAGMENT,
                                ty: wgpu::BindingType::UniformBuffer {
                                    dynamic: true,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                        ],
                    }
                );
                let uniform_bind_group = device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(&"Uniform Bind Group"),
                        layout: &uniform_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(vertex_uniform_buffer.slice(
                                    0..std::mem::size_of::<VertexUniformData>() as wgpu::BufferAddress
                                ))
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(fragment_uniform_buffer.slice(
                                    0..std::mem::size_of::<FragmentUniformData>() as wgpu::BufferAddress
                                ))
                            }
                        ],
                    }
                );

                println!("renderer >> uniforms created");

        //vertex buffer
            let rectangle_point: [f32; 8] = [
                0.0,0.0,
                1.0,0.0,
                1.0,1.0,
                0.0,1.0,
            ];
            let triangle_list_points = [
                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[1*2 + 0], rectangle_point[1*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],

                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],
                rectangle_point[3*2 + 0], rectangle_point[3*2 + 1],
            ];

            let mut vertex_point_data:Vec<library::Vertex> = vec![];
            for index in (0..triangle_list_points.len()).step_by(2) {
                vertex_point_data.push(
                    library::Vertex::new(
                        [
                            triangle_list_points[index + 0],
                            triangle_list_points[index + 1],
                        ]
                    ),
                );
            }
            
            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&"Vertex Buffer"),
                    contents: bytemuck::cast_slice(&*vertex_point_data),
                    usage: wgpu::BufferUsage::VERTEX,
                }
            );

        //render pipelines
            //layout
                let render_pipeline_layout = device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some(&"Render Pipeline Layout"),
                        bind_group_layouts: &[
                            &uniform_bind_group_layout
                        ],
                        push_constant_ranges: &[],
                    }
                );  
            //pipelines
                let render_pipeline = device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
                        label: Some(&"Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex_stage: wgpu::ProgrammableStageDescriptor {
                            module: &vertex_shader_module,
                            entry_point: "main",
                        },
                        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                            module: &fragment_shader_module,
                            entry_point: "main",
                        }),
                        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                            front_face: wgpu::FrontFace::Cw,
                            cull_mode: wgpu::CullMode::Back,
                            depth_bias: 0,
                            depth_bias_slope_scale: 0.0,
                            depth_bias_clamp: 0.0,
                            clamp_depth: false,
                        }),
                        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                        color_states: &[wgpu::ColorStateDescriptor {
                            format: swap_chain_descriptor.format,
                            color_blend: wgpu::BlendDescriptor {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                        depth_stencil_state: None,
                        vertex_state: wgpu::VertexStateDescriptor {
                            index_format: wgpu::IndexFormat::Uint16,
                            vertex_buffers: &[library::Vertex::desc()],
                        },
                        sample_count: 1,
                        sample_mask: !0,
                        alpha_to_coverage_enabled: false,
                    }
                );

        //setup frame
            let frame = match swap_chain.get_current_frame() {
                Err(e) => {
                    println!("ERROR - Renderer : swap_chain.get_current_frame failed! {}", e);
                    return;
                },
                Ok(frame) => frame.output,
            };
    
        //create encoder
            let mut encoder = device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Renderer : Command Encoder"),
                }
            );

        //clear frame
            {
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 1.0,
                                            g: 1.0,
                                            b: 1.0,
                                            a: 1.0,
                                        }
                                    ),
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );
            }

        println!("renderer >> beginning main render pass");

        //begin main render pass
            let start_time = std::time::Instant::now();
            {
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );

                //set pipeline
                    render_pass.set_pipeline(&render_pipeline);
                //set vertex buffer
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                for index in 0..rectangle_count {
                    //set uniform buffer group
                        render_pass.set_bind_group(0, &uniform_bind_group, &[
                            (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //vertex uniform
                            (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //fragment uniform
                        ]);
                    //draw
                        render_pass.draw(0..vertex_point_data.len() as u32, 0..1);
                }
            }
            let end_time = std::time::Instant::now();
            println!("{:?}", end_time.checked_duration_since(start_time) );

        //submit encoder to queue, to be rendered
            queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn test3(&mut self) {
        //better buffer usage, double render

        println!("- test 3 -");
        let mut start_time = std::time::Instant::now();

        let rectangle_count:usize = 2_000_000;

        //adapter
            let adapter = futures::executor::block_on(
                self.instance.request_adapter(
                    &wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::Default,
                        compatible_surface: Some(&self.surface),
                    }
                )
            ).unwrap();

        //device and queue
            let (device, queue) = futures::executor::block_on(
                adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        shader_validation: true,
                    },
                    None,
                )
            ).unwrap();

        //swap chain
            let swap_chain_descriptor = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: self.size.width,
                height: self.size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            let mut swap_chain = device.create_swap_chain( &self.surface, &swap_chain_descriptor );

        //framebuffer
            let multisampled_texture_extent = wgpu::Extent3d {
                width: swap_chain_descriptor.width,
                height: swap_chain_descriptor.height,
                depth: 1,
            };
            let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
                size: multisampled_texture_extent,
                mip_level_count: 1,
                sample_count: 4, //1,
                dimension: wgpu::TextureDimension::D2,
                format: swap_chain_descriptor.format,
                usage: wgpu::TextureUsage::all(),
                label: Some("multisampled framebuffer"),
            };
            let framebuffer = device.create_texture(multisampled_frame_descriptor).create_view(&wgpu::TextureViewDescriptor::default());

        //create shader modules
            let vertex_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.vert.spv") );
            let fragment_shader_module = device.create_shader_module( wgpu::include_spirv!("shader.frag.spv") );

        println!("renderer >> starting creation of uniform buffers {:?}", std::time::Instant::now().checked_duration_since(start_time) );
        start_time = std::time::Instant::now();

        //uniforms
            //uniform buffers
                let mut rng = rand::thread_rng();

                // println!("{} {}", std::mem::size_of::<VertexUniformData>(), wgpu::BIND_BUFFER_ALIGNMENT);

                //vertex
                    let vertex_uniform_buffer = device.create_buffer(
                        &wgpu::BufferDescriptor {
                            label: Some(&"Vertex Uniform Printing Buffer"),
                            size: rectangle_count as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT,
                            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            mapped_at_creation: false,
                        }
                    );
                    println!("renderer >> vertex uniform buffer created: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                    start_time = std::time::Instant::now();
                    for index in 0..rectangle_count {
                        let vertex_data = VertexUniformData::new(
                            500.0*rng.gen::<f32>(), //x
                            400.0*rng.gen::<f32>(), //y
                            1.0, //scale
                            2.0 * std::f32::consts::PI * rng.gen::<f32>(), //angle
                            Dimensions::<u32>::new(500,400), //dimensions
                            30.0, //width
                            30.0, //height
                            Point::new(0.5,0.5), //anchor
                        );
                        queue.write_buffer(
                            &vertex_uniform_buffer, 
                            wgpu::BIND_BUFFER_ALIGNMENT * index as wgpu::BufferAddress,
                            bytemuck::cast_slice(&[vertex_data])
                        );
                    }
                    println!("renderer >> vertex uniform buffer populated: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                    start_time = std::time::Instant::now();

                //fragment
                    let fragment_uniform_buffer = device.create_buffer(
                        &wgpu::BufferDescriptor {
                            label: Some(&"Fragment Uniform Printing Buffer"),
                            size: rectangle_count as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT,
                            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            mapped_at_creation: false,
                        }
                    );
                    println!("renderer >> fragment uniform buffer created: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                    start_time = std::time::Instant::now();
                    for index in 0..rectangle_count {
                        let fragment_data = FragmentUniformData::new(
                            Colour::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0),
                        );
                        queue.write_buffer(
                            &fragment_uniform_buffer, 
                            wgpu::BIND_BUFFER_ALIGNMENT * index as wgpu::BufferAddress,
                            bytemuck::cast_slice(&[fragment_data])
                        );
                    }
                    println!("renderer >> fragment uniform buffer populated: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                    start_time = std::time::Instant::now();

            //uniform bind group
                let uniform_bind_group_layout = device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: Some(&"Uniform Bind Group Layout"),
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStage::VERTEX,
                                ty: wgpu::BindingType::UniformBuffer {
                                    dynamic: true,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStage::FRAGMENT,
                                ty: wgpu::BindingType::UniformBuffer {
                                    dynamic: true,
                                    min_binding_size: None,
                                },
                                count: None,
                            },
                        ],
                    }
                );
                let uniform_bind_group = device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(&"Uniform Bind Group"),
                        layout: &uniform_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(vertex_uniform_buffer.slice(
                                    0..std::mem::size_of::<VertexUniformData>() as wgpu::BufferAddress
                                ))
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(fragment_uniform_buffer.slice(
                                    0..std::mem::size_of::<FragmentUniformData>() as wgpu::BufferAddress
                                ))
                            }
                        ],
                    }
                );

                println!("renderer >> uniforms created: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                start_time = std::time::Instant::now();

        //vertex buffer
            let rectangle_point: [f32; 8] = [
                0.0,0.0,
                1.0,0.0,
                1.0,1.0,
                0.0,1.0,
            ];
            let triangle_list_points = [
                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[1*2 + 0], rectangle_point[1*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],

                rectangle_point[0*2 + 0], rectangle_point[0*2 + 1],
                rectangle_point[2*2 + 0], rectangle_point[2*2 + 1],
                rectangle_point[3*2 + 0], rectangle_point[3*2 + 1],
            ];

            let mut vertex_point_data:Vec<library::Vertex> = vec![];
            for index in (0..triangle_list_points.len()).step_by(2) {
                vertex_point_data.push(
                    library::Vertex::new(
                        [
                            triangle_list_points[index + 0],
                            triangle_list_points[index + 1],
                        ]
                    ),
                );
            }
            
            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&"Vertex Buffer"),
                    contents: bytemuck::cast_slice(&*vertex_point_data),
                    usage: wgpu::BufferUsage::VERTEX,
                }
            );

        //render pipelines
            //layout
                let render_pipeline_layout = device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some(&"Render Pipeline Layout"),
                        bind_group_layouts: &[
                            &uniform_bind_group_layout
                        ],
                        push_constant_ranges: &[],
                    }
                );  
            //pipelines
                let render_pipeline = device.create_render_pipeline(
                    &wgpu::RenderPipelineDescriptor {
                        label: Some(&"Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex_stage: wgpu::ProgrammableStageDescriptor {
                            module: &vertex_shader_module,
                            entry_point: "main",
                        },
                        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                            module: &fragment_shader_module,
                            entry_point: "main",
                        }),
                        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                            front_face: wgpu::FrontFace::Cw,
                            cull_mode: wgpu::CullMode::Back,
                            depth_bias: 0,
                            depth_bias_slope_scale: 0.0,
                            depth_bias_clamp: 0.0,
                            clamp_depth: false,
                        }),
                        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                        color_states: &[wgpu::ColorStateDescriptor {
                            format: swap_chain_descriptor.format,
                            color_blend: wgpu::BlendDescriptor {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                        depth_stencil_state: None,
                        vertex_state: wgpu::VertexStateDescriptor {
                            index_format: wgpu::IndexFormat::Uint16,
                            vertex_buffers: &[library::Vertex::desc()],
                        },
                        sample_count: 4, //1,
                        sample_mask: !0,
                        alpha_to_coverage_enabled: false,
                    }
                );

        //setup frame
            let frame = match swap_chain.get_current_frame() {
                Err(e) => {
                    println!("ERROR - Renderer : swap_chain.get_current_frame failed! {}", e);
                    return;
                },
                Ok(frame) => frame.output,
            };
    
        //create encoder
            let mut encoder = device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Renderer : Command Encoder"),
                }
            );

        //clear frame
            {
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &framebuffer,
                                resolve_target: Some(&frame.view),
                                // attachment: &frame.view,
                                // resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 1.0,
                                            g: 1.0,
                                            b: 1.0,
                                            a: 1.0,
                                        }
                                    ),
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );
            }

        println!("renderer >> beginning main render pass: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
        start_time = std::time::Instant::now();

        //begin main render pass
            {
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &framebuffer,
                                resolve_target: Some(&frame.view),
                                // attachment: &frame.view,
                                // resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );

                //set pipeline
                    render_pass.set_pipeline(&render_pipeline);
                //set vertex buffer
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                println!("renderer >> (starting draw loop): {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                start_time = std::time::Instant::now();

                for index in 0..rectangle_count {
                    //set uniform buffer group
                        render_pass.set_bind_group(0, &uniform_bind_group, &[
                            (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //vertex uniform
                            (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //fragment uniform
                        ]);
                    //draw
                        render_pass.draw(0..vertex_point_data.len() as u32, 0..1);
                }
            }
            println!("renderer >> finished main render pass, now submitting: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
            start_time = std::time::Instant::now();

        //submit encoder to queue, to be rendered
            queue.submit(std::iter::once(encoder.finish()));

        println!("renderer >> submission complete: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
        start_time = std::time::Instant::now();





        std::thread::sleep( std::time::Duration::from_millis(1000) );
        println!("");





        //second render
            println!("renderer >> second render: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
            start_time = std::time::Instant::now();

            // //setup frame
            //     let frame = match swap_chain.get_current_frame() {
            //         Err(e) => {
            //             println!("ERROR - Renderer : swap_chain.get_current_frame failed! {}", e);
            //             return;
            //         },
            //         Ok(frame) => frame.output,
            //     };
        
            //create encoder
                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Renderer : Command Encoder"),
                    }
                );

            //clear frame
                {
                    encoder.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            color_attachments: &[
                                wgpu::RenderPassColorAttachmentDescriptor {
                                    attachment: &framebuffer,
                                    resolve_target: Some(&frame.view),
                                    // attachment: &frame.view,
                                    // resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(
                                            wgpu::Color {
                                                r: 1.0,
                                                g: 1.0,
                                                b: 1.0,
                                                a: 1.0,
                                            }
                                        ),
                                        store: true,
                                    },
                                }
                            ],
                            depth_stencil_attachment: None,
                        }
                    );
                }

            println!("renderer >> beginning main render pass: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
            start_time = std::time::Instant::now();

            //begin main render pass
                {
                    let mut render_pass = encoder.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            color_attachments: &[
                                wgpu::RenderPassColorAttachmentDescriptor {
                                    attachment: &framebuffer,
                                    resolve_target: Some(&frame.view),
                                    // attachment: &frame.view,
                                    // resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: true,
                                    },
                                }
                            ],
                            depth_stencil_attachment: None,
                        }
                    );

                    //set pipeline
                        render_pass.set_pipeline(&render_pipeline);
                    //set vertex buffer
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    println!("renderer >> (starting draw loop): {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                    start_time = std::time::Instant::now();

                    for index in 0..rectangle_count {
                        //set uniform buffer group
                            render_pass.set_bind_group(0, &uniform_bind_group, &[
                                (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //vertex uniform
                                (index as wgpu::BufferAddress * wgpu::BIND_BUFFER_ALIGNMENT) as wgpu::DynamicOffset, //fragment uniform
                            ]);
                        //draw
                            render_pass.draw(0..vertex_point_data.len() as u32, 0..1);
                    }
                }
                println!("renderer >> finished main render pass, now submitting: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
                start_time = std::time::Instant::now();

            //submit encoder to queue, to be rendered
                queue.submit(std::iter::once(encoder.finish()));

            println!("renderer >> submission complete: {:?}", std::time::Instant::now().checked_duration_since(start_time) );
            // start_time = std::time::Instant::now();
    }
}