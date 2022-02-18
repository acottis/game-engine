//! Program entry point, sets up everything then runs a main game loop
//! 
mod interface;
mod entity;

/// This function handles the main game loop with the multiple components
fn main() {
    // Setup the window, our window lives as long as _window lives
    let (event_loop, window) = interface::init_window();

    // Set up our GPU or onboard graphics
    let mut gfx_instance = interface::init_gfx(&window);

    // Listens for events in the windows and we handle our responses to those
    // events
    event_loop.run(move | event, _, ctrl_flow | {
        interface::handle_events(&event, ctrl_flow, &mut gfx_instance);
    })
}
