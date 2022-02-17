//! Here we deal with all things Grpahics using [wgpu], we use [futures] to 
//! handle the async parts 

pub struct Instance {
    pub surface:         wgpu::Surface,
    pub surface_config:  wgpu::SurfaceConfiguration,
    pub device:          wgpu::Device,
    pub queue:           wgpu::Queue,
    pub render_pipeline: wgpu::RenderPipeline,
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
        
        
        println!("{surface:?}, \n{device:?}, \n{queue:?}");
        
        let render_pipeline = triangle(&device, &surface_config.format);
        
        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
        })
    }

    pub fn resize(&mut self, height: u32, width: u32){
     
        self.surface_config.height = height;
        self.surface_config.width = width;

        self.surface.configure(&self.device, &self.surface_config);
    }
}


fn triangle(
    device: &wgpu::Device, 
    swapchain_format: &wgpu::TextureFormat
) -> wgpu::RenderPipeline {
    // load shader from file
    let shader_desc = wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(
            std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))
        )
    };
    let shader = device.create_shader_module(&shader_desc);

    let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
        label:                Some("Triangle Pipeline Layout"),
        bind_group_layouts:   &[],
        push_constant_ranges: &[],
    };
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let render_pipeline = device.create_render_pipeline(
    &wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Render Pipline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: "vs_main", 
                buffers: &[], 
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState::from(*swapchain_format)]
                // targets: &[ wgpu::ColorTargetState {
                //     format: *swapchain_format,
                //     blend: None,
                //     write_mask: wgpu::ColorWrites::empty(),
                // }],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        }
    );

    println!("{shader:?}, \n{pipeline_layout:?}, \n{pipeline_layout:?}, \n{render_pipeline:?}");

    render_pipeline

}