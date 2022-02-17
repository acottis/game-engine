//! This module handles all things user interfact and graphical
//! using our [crate::gfx] module to provide GPU support and [winit] 
//! to provide a window from the OS 
//! 

pub mod gfx;
use gfx::Instance;
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
/// 
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

pub fn init_gfx(window: &Window) -> Instance {
    // Block until we setup GPU
    pollster::block_on(gfx::Instance::new(&window))
        .expect("Could not init GPU/Onboard GPU")
}

/// We handle [Event::WindowEvent] here
/// 
fn handle_window_event(
    _window_id: &WindowId, 
    event: &WindowEvent, 
    ctrl_flow: &mut ControlFlow,
    gfx: &mut Instance,
){
    match event {
        // Handle user requesting close
        WindowEvent::CloseRequested => { *ctrl_flow = ControlFlow::Exit },
        WindowEvent::Resized(size) => {
            gfx.resize(size.height, size.width);
        }
        _ => { }
       // _ => { println!("{event:?}") }
    }
}

/// Handles [Event::RedrawRequested]
/// 
fn handle_redraw_request(gfx: &Instance){
    println!("Redraw");
    gfx.draw("triangle");
}

/// Entry point main event handler, main logic is here, it is called by 
/// [crate::main]
pub fn handle_events(
    event: &Event<()>, 
    ctrl_flow: &mut ControlFlow, 
    gfx: &mut Instance,
){
    // We send events to the appropriate handlers
    match event {
        // Emitted when new events arrive from the OS to be processed.
        Event::NewEvents(_start_cause) => {},
        // Emitted when the OS sends an event to a winit window
        Event::WindowEvent{
            window_id,
            event 
        } => { 
            handle_window_event(window_id, event, ctrl_flow, gfx) 
        },
        // Emitted when OS requests screen refresh
        Event::RedrawRequested(_) =>{
            handle_redraw_request(gfx);
        },
        // Emitted when the OS sends an event to a device.
        Event::DeviceEvent {
            device_id: _,
            event:     _,
        } => {},
        _ => {},
    }
}