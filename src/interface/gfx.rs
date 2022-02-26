//! Here we deal with all things Grpahics using [wgpu], we use [pollster] to 
//! handle the async parts 

use crate::engine::entity::Shape2D;

/// This struct repesents a position in 2d space, we use these in batches of 3
/// to build [crate::entity::Shape2D]
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex2D{
    coord: [f32; 2], // x, y
    colour: [f32; 4], // x, y
}

impl Vertex2D{
    /// Create new [Vertex2D]
    fn new(x: f32, y: f32, colour: wgpu::Color) -> Self{
        Self{
            coord: [x, y],
            colour: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32],
        }
    }
    /// This descriptor is passed to [wgpu::RenderPipelineDescriptor]
    fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;2]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// This struct handles the basic state of our GPU after we initialise it, we 
/// can use these to then request models and shaders to be rendered
pub struct Instance {
    surface:         wgpu::Surface,
    surface_config:  wgpu::SurfaceConfiguration,
    device:          wgpu::Device,
    queue:           wgpu::Queue,
    buffer:          wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl Instance{
    /// Init our WGPU api
    pub async fn new(window: &winit::window::Window) -> Result<Self,()> {
        // Base type of the wgpu module
        let instance: wgpu::Instance 
            = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        
        // Surface is on top of the window which we use to draw
        let surface = unsafe {
            instance.create_surface(window)
        };
        
        // Set up our adapter options to send to [request_adapter] with high
        // power and without forcing fallback as this would pick CPU
        let adaptor_options = wgpu::RequestAdapterOptions {
            power_preference:       wgpu::PowerPreference::HighPerformance,
            compatible_surface:     Some(&surface),
            force_fallback_adapter: false,
        };
        
        // Get the adapter with the specified options
        let adapter = instance.request_adapter(&adaptor_options).await;
        
        // Throw error if dont have an adapter, give us GPU and Backend chosen
        // Return a device and queue from our adapter
        let (device, queue) = match &adapter {
            Some(adapter) => {
                // Get basic adapter info
                let info = adapter.get_info();
                println!("GPU: {}, Backend: {:?}", &info.name, &info.backend);
                
                // Set up our device, the label is for debugging, we use the 
                // adapter to determine what the features to enable are and what 
                // are the best limits
                let device_desc = wgpu::DeviceDescriptor {
                    label:    Some("My Device Descriptor"),
                    limits:   adapter.limits(),
                    features: adapter.features(),
                };
                
                // Request the adapter with our device descriptor
                adapter.request_device(&device_desc, None)
                .await
                .expect("Could not get device")
            }
            // Not implemented handling if we do not get a single adapter yet
            None => todo!("No adaptor found")
        };
        
        let window_size = window.inner_size();
        // Surface Config
        let surface_config = wgpu::SurfaceConfiguration {
            usage:  wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(
                &adapter.unwrap()
            ).expect("No texture format supported"),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo, // Others not implemented
        };
        
        // Initize surface for presentation
        surface.configure(&device, &surface_config);

        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size: 1024,
                usage: wgpu::BufferUsages::all(),
                mapped_at_creation: false,
            }
        );
        
        // Initialize my shaders
        let shader_desc = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(
                // load shader from file
                include_str!("shaders/shader.wgsl").into())
        };
        let shader = device.create_shader_module(&shader_desc);

        //// Dont think we need this yet
        // let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
        //     label:                Some(&pipeline_layout_label),
        //     bind_group_layouts:   &[],
        //     push_constant_ranges: &[],
        // };
        // let pipeline_layout 
        //     = self.device.create_pipeline_layout(&pipeline_layout_desc);

        // Init render pipeline
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                    label: None,
                    //layout: Some(&pipeline_layout),
                    layout: None,
                    // This is for shape
                    vertex: wgpu::VertexState { 
                        module: &shader, 
                        entry_point: "vs_main", 
                        buffers: &[Vertex2D::descriptor()], 
                    },
                    // This is for colour
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[
                            wgpu::ColorTargetState::from(
                                surface_config.format)
                        ]
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    multisample: wgpu::MultisampleState::default(),
                    depth_stencil: None,
                    multiview: None,
                }
            );

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            buffer,
            render_pipeline,
        })
    }

    /// This resizing the window when the user adjusts the window size
    pub fn resize(&mut self, height: u32, width: u32){
        // if area == 0 it will panic
        if height * width == 0 { return }

        self.surface_config.height = height;
        self.surface_config.width = width;
        self.surface.configure(&self.device, &self.surface_config);
    }

    /// Main entry point for user to create a shape
    pub fn draw(&self, entities: &[Shape2D]) {

        // Puts all the entities into the vertex buffer
        let entity_buffer = self.create_buffer(entities);
        
        // This puts our entities into the GPU command queue to be sent
        // to the GPU when ready
        self.queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&entity_buffer)
        );
        
        // Get the next frame from the surface
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => { frame },
            // No idea why this panics so lets just handle it and not draw
            Err(wgpu::SurfaceError::Outdated) => { return }, 
            // I want to panic if anything else happens, not expected
            Err(e) => {
                panic!("{e:?}")
            },
        };

        let view = frame.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        // Init the Command Encoder
        let mut encoder = self.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor { 
            label: None 
        });

        // Init the Render Pass
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // BackgroundColour
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), 
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        // use our pipeline we init in our constructer
        rpass.set_pipeline(&self.render_pipeline);

        // Put the vertex buffer into slot 0 of the GPU
        rpass.set_vertex_buffer(
            0, 
            self.buffer.slice(..),
        );

        // Have to call this last after setting everything for the render_pass
        rpass.draw(0..(entity_buffer.len() as u32), 0..1);

        // We need to drop this as it owns encoder which we need to use in the 
        // nextline
        drop(rpass);

        // Send to the GPU
        self.queue.submit(Some(encoder.finish()));
        // Show the output on the surface
        frame.present();
    }

    /// Turn a shape into a buffer of its triangles we accept 
    /// [crate::entity::Shape] then turn it into a triangle or
    /// Rectangle on the GPU, we match on the shape and then create the
    /// shape from the coordinates
    fn create_buffer(&self, entities: &[Shape2D]) -> Vec<Vertex2D> {
        // Create an empty vec
        let mut vertex_buf: Vec<Vertex2D> = Vec::new();
        // Go through all entities we are given by engine and add them to 
        // buffer
        for entity in entities{
            match entity {
                // Turn the entity into 2D Vertexs
                Shape2D::Triangle(t) => {
                    vertex_buf.push(Vertex2D::new(t.a.x, t.a.y, t.colour)); //A
                    vertex_buf.push(Vertex2D::new(t.b.x, t.b.y, t.colour)); //B
                    vertex_buf.push(Vertex2D::new(t.c.x, t.c.y, t.colour)); //C
                },
                Shape2D::Rectangle(r) => {
                    vertex_buf.push(Vertex2D::new(r.a.x, r.a.y, r.colour)); //A
                    vertex_buf.push(Vertex2D::new(r.b.x, r.b.y, r.colour)); //B
                    vertex_buf.push(Vertex2D::new(r.c.x, r.c.y, r.colour)); //C
                    
                    vertex_buf.push(Vertex2D::new(r.b.x, r.b.y, r.colour)); //B
                    vertex_buf.push(Vertex2D::new(r.c.x, r.c.y, r.colour)); //C
                    vertex_buf.push(Vertex2D::new(r.d.x, r.d.y, r.colour)); //D
                },
                &Shape2D::Pentagon(p) => {
                    vertex_buf.push(Vertex2D::new(p.a.x, p.a.y, p.colour)); //A
                    vertex_buf.push(Vertex2D::new(p.b.x, p.b.y, p.colour)); //B
                    vertex_buf.push(Vertex2D::new(p.c.x, p.c.y, p.colour)); //C

                    vertex_buf.push(Vertex2D::new(p.b.x, p.b.y, p.colour)); //B
                    vertex_buf.push(Vertex2D::new(p.c.x, p.c.y, p.colour)); //C
                    vertex_buf.push(Vertex2D::new(p.d.x, p.d.y, p.colour)); //D

                    vertex_buf.push(Vertex2D::new(p.c.x, p.c.y, p.colour)); //C
                    vertex_buf.push(Vertex2D::new(p.d.x, p.d.y, p.colour)); //D
                    vertex_buf.push(Vertex2D::new(p.e.x, p.e.y, p.colour)); //E
                }
            }
        }
        vertex_buf
    }
}