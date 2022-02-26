use std::collections::HashMap;
use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use super::entity::{Shape2D, Triangle, Rectangle, Point};
use crate::globals::TICK_RATE;

/// This will store our game state and pass it around
#[derive(Debug)]
pub struct Game {
    // Stores all objects, we send this to the GPU for rendering
    pub entities: Vec<Shape2D>,
    // This will keep track of the player entities index
    pub players: Vec<usize>,
    // Keeps track of keys down
    pub keys_down: HashMap<Option<VirtualKeyCode>, u8>,
    // Last time to calculate the delta
    last_time: std::time::Instant,
    // Delta time to fix physics
    pub dt: f32,
}

impl Game {
    /// Create a new game
    pub fn new() -> Self {
        // Init our arrays
        let mut players: Vec<usize> = Vec::new();
        let mut entities: Vec<Shape2D> = Vec::new();

        // We push the index of our player into our players vec
        players.push(entities.len());
        // PLAYER
        let player = Shape2D::Triangle(Triangle::new(
            Point::new(-0.95, -0.9), // A
            Point::new(-0.9,  -1.0), // B
            Point::new(-1.0,  -1.0), // C
            wgpu::Color::BLACK,
            super::physics::Physics { state: super::physics::State::None, collides: false }
        ));

        entities.push(player);
        //entities.push(Shape2D::Rectangle(Rectangle::default()));
        //entities.push(Shape2D::Pentagon(Pentagon::default()));

        let floor = Shape2D::Rectangle(
            Rectangle::new(
            Point::new(-1.1, -0.95),
            Point::new(1.1, -0.95),
            Point::new(-1.1, -1.05),
            Point::new(1.1, -1.05),
            wgpu::Color::GREEN,
            super::physics::State::Static,
        ));

        let platform = Shape2D::Rectangle(
            Rectangle::new(
            Point::new(-0.7, -0.77),
            Point::new(-0.5, -0.77),
            Point::new(-0.7, -0.75),
            Point::new(-0.5, -0.75),
            wgpu::Color::GREEN,
            super::physics::State::Static,
        ));
        
        entities.push(floor);
        entities.push(platform);

        Self {
            entities,
            players,
            keys_down: HashMap::new(),
            last_time: std::time::Instant::now(),
            dt: 0.0,
        }
    }
    /// Update the delta to fix the rate at which the game is played
    /// 
    pub fn update_dt(&mut self){
        // Get current time
        let current_time = std::time::Instant::now();
        // Get the difference between the last frame and this one
        self.dt = (current_time - self.last_time).as_secs_f32();
        // If the game is running too fast we cap to 144 ticks per frame
        if self.dt < TICK_RATE { 
            self.dt = TICK_RATE;
        }
        self.last_time = current_time;
    }
    /// This is sent keyboard inputs from our event loop
    /// 
    pub fn keyboard_input(&mut self, input: &KeyboardInput){
        // This prevents a bug where we no longer get key events when we 
        // Press multiple at once, we add them to a dictionary that we trust
        // as the truth of user inputs
        match input.state {
            ElementState::Pressed => {
                self.keys_down.insert(input.virtual_keycode, 0u8);
            }
            ElementState::Released => { 
                self.keys_down.remove(&input.virtual_keycode);
            },
        }
    }
    /// Runs game logic in a tick, also calls physics and handles
    /// user input logic
    /// 
    pub fn update(&mut self){
        // Handle any user inputs
        super::controls::update(self);
        // Run the phsyics against our game
        super::physics::update(self);
        // Run the camera
        super::camera::update(self);
    }
}