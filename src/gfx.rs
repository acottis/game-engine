//! Here we deal with all things Grpahics using [wgpu], we use [futures] to 
//! handle the async parts 

pub struct Gpu {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

// Init our WGPU api
pub async fn new(window: &winit::window::Window) -> Result<(),()> {
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


    Ok(())
}