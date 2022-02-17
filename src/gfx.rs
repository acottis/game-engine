//! Here we deal with all things Grpahics using [wgpu], we use [pollster] to 
//! handle the async parts 

/// This struct handles the basic state of our GPU after we initialise it, we 
/// can use these to then request models and shaders to be rendered
pub struct Instance {
    surface:         wgpu::Surface,
    surface_config:  wgpu::SurfaceConfiguration,
    device:          wgpu::Device,
    queue:           wgpu::Queue,
}

impl Instance{

    // Init our WGPU api
    pub async fn new(window: &winit::window::Window) -> Result<Self,()> {
        // Base type of the wgpu module
        let instance: wgpu::Instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        
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
        
        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
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

    // Testing
    pub fn draw(&self, label: &str) {

        // Set up labels for debugging
        let shader_label          = format!("'{label}' Shader");
        //let pipeline_layout_label = format!("'{label}' Pipeline Layout");
        let vertex_buffer_label     = format!("'{label}' Vertex Buffer");
        let render_pipeline_label = format!("'{label}' Render Pipeline");
        let render_pass_label     = format!("'{label}' Render Pass");

        // load shader from file
        let shader_desc = wgpu::ShaderModuleDescriptor {
            label: Some(&shader_label),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shader.wgsl").into()
            )
        };
        let shader = self.device.create_shader_module(&shader_desc);

        //// Dont think we need this yet
        // let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
        //     label:                Some(&pipeline_layout_label),
        //     bind_group_layouts:   &[],
        //     push_constant_ranges: &[],
        // };
        // let pipeline_layout 
        //     = self.device.create_pipeline_layout(&pipeline_layout_desc);

        // MY TESTING --------------------------

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
        struct Test{
            position: [f32; 3],
        }

        impl Test{
            fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Self>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }
            }
        }

        use wgpu::util::{DeviceExt, BufferInitDescriptor};
        let test_vertex_buffer = DeviceExt::create_buffer_init(
        &self.device, 
        &BufferInitDescriptor {
                label: Some(&vertex_buffer_label),
                contents: bytemuck::cast_slice(
                    &[
                        Test { position: [-1.0  ,-1.0,  0.0] }, //A
                        Test { position: [ 1.0  ,-0.5,  0.0] }, //B
                        Test { position: [ 1.0  ,-1.0,  0.0] }, //C
                        
                        Test { position: [-1.0  ,-1.0,  0.0] }, //A
                        Test { position: [ 1.0  ,-0.5,  0.0] }, //B
                        Test { position: [-1.0  ,-0.5,  0.0] }, //D
                    ]
                ),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        // MY TESTING --------------------------
        let render_pipeline = self.device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
                label: Some(&render_pipeline_label),
                //layout: Some(&pipeline_layout),
                layout: None,
                // This is for shape
                vertex: wgpu::VertexState { 
                    module: &shader, 
                    entry_point: "vs_main", 
                    //buffers: &[], 
                    buffers: &[Test::descriptor()], 
                },
                // This is for colour
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        wgpu::ColorTargetState::from(
                            self.surface_config.format)
                    ]
                }),
                primitive: wgpu::PrimitiveState::default(),
                multisample: wgpu::MultisampleState::default(),
                depth_stencil: None,
                multiview: None,
            }
        );

        
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { 
                label: None 
            });
    
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&render_pass_label),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // BackgroundColour
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE), 
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&render_pipeline);
        //rpass.draw(0..6, 0..1);
        rpass.set_vertex_buffer(
            0, 
            test_vertex_buffer.slice(..),
        );

        // Have to call this last
        rpass.draw(0..6, 0..1);

        println!("{rpass:?}");

        // We need to drop this as it owns encoder which we need to use in the 
        // nextline
        drop(rpass);

        // Send to the GPU
        self.queue.submit(Some(encoder.finish()));

        // Show the output on the surface
        frame.present();
    }
}