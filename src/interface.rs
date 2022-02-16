use winit::{
    event::{WindowEvent, Event},
    event_loop::{EventLoop, ControlFlow}, 
    window::{
        Window,
        WindowBuilder, 
        WindowId,
    },
};

/// Set up the window and return an [EventLoop] and [Window] Object
pub fn init_window() -> (EventLoop<()>, Window) {
    // Creates an event listener that we can pass into our window
    let event_loop = EventLoop::new();

    // Creates a window using the WindowBuilder to choose the properties
    let window = WindowBuilder::new()
    .with_resizable(true)
    .with_title("Azph Engine")
    .build(&event_loop).expect("Cant Create Window");

    (event_loop, window)
}

/// We handle [Event::WindowEvent] here
fn handle_window_event(
    _window_id: &WindowId, 
    event: &WindowEvent, 
    ctrl_flow: &mut ControlFlow
){
    match event {
        // Handle user requesting close
        WindowEvent::CloseRequested => { *ctrl_flow = ControlFlow::Exit }
        _ => { }
       // _ => { println!("{event:?}") }
    }
}

pub fn handle_events(event: &Event<()>, ctrl_flow: &mut ControlFlow){
        // We send events to the appropriate handlers
        match event {
            // Emitted when new events arrive from the OS to be processed.
            Event::NewEvents(_start_cause) => {},
            // Emitted when the OS sends an event to a winit window
            Event::WindowEvent{
                window_id,
                event 
            } => { 
                handle_window_event(window_id, event, ctrl_flow) 
            },
            // Emitted when the OS sends an event to a device.
            Event::DeviceEvent {
                device_id: _,
                event: _,
            } => {},
            _ => {},
        }
}