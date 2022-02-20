//! Program entry point, sets up everything then runs a main game loop
//! 

mod interface;
mod engine;

const TICK_RATE: f32 = 1.0 / 144.0;

/// This function handles the main game loop with the multiple components
fn main() {
    // Setup the window, our window lives as long as _window lives
    let (event_loop, window) = interface::init_window();

    // Set up our GPU or onboard graphics
    let mut gfx_instance = interface::init_gfx(&window);

    // Set up our Game engine
    let mut game = engine::Game::new();

    println!("{:?}", game);
    // Listens for events in the windows and we handle our responses to those
    // events
    let mut last_time = std::time::Instant::now();
    event_loop.run(move | event, _, ctrl_flow | {

        let current_time = std::time::Instant::now();
        game.dt = (current_time - last_time).as_secs_f32();
        if game.dt < TICK_RATE { 
            game.dt = TICK_RATE;
        }
        //println!("{}", game.dt);
        last_time = current_time;

        interface::handle_events(
            &window,
            &event, 
            ctrl_flow,
            &mut gfx_instance, 
            &mut game
        );
    })
}
