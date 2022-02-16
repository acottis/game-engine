mod interface;
mod gfx;

/// This function handles the main game loop with the multiple components
fn main() {
    // Setup the window, our window lives as long as _window lives
    let (event_loop, window) = interface::init_window();

    // Block until we setup GPU
    futures::executor::block_on(gfx::new(&window)).expect("Could not init GPU");

    // Listens for events in the windows and we handle our responses to those
    // events
    event_loop.run(| event, _, ctrl_flow | {
        interface::handle_events(&event, ctrl_flow);
    })
}
