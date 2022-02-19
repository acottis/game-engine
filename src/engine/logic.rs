use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use super::entity::{Shape2D, Rectangle, Transform2D, Entity};
use super::physics::{State, JUMP_TIME};
use std::collections::HashMap;

/// This will store our game state and pass it around
#[derive(Debug)]
pub struct Game {
    // Stores all objects, we send this to the GPU for rendering
    pub entities: Vec<Shape2D>,
    // This will keep track of the player entities index
    pub players: Vec<usize>,
    // Keeps track of keys down
    pub keys_down: HashMap<Option<VirtualKeyCode>, u8>
}

impl Game {

    const PLAYER_ONE: usize = 0;

    pub fn new() -> Self {
        let mut players: Vec<usize> = Vec::new();
        let mut entities: Vec<Shape2D> = Vec::new();

        // let triangle = Triangle::new(
        //     Point::new(-1.0, -0.4),
        //     Point::new( 0.0,  1.0),
        //     Point::new( 1.0, -0.4),
        //     wgpu::Color::RED
        // );
    
        // let triangle2 = Triangle::new(
        //     Point::new(-1.0, -0.2),
        //     Point::new( 0.0,  1.0),
        //     Point::new( 1.0, -0.2),
        //     wgpu::Color::BLACK
        // );
    
        // let ground = Rectangle::new(
        //     Point::new(-1.0, -1.0),
        //     Point::new( 1.0, -0.4),
        //     Point::new( 1.0, -1.0),
        //     Point::new(-1.0, -0.4),
        //     wgpu::Color::GREEN,
        // );
        //entities.push(Shape2D::Triangle(triangle));
        //entities.push(Shape2D::Triangle(triangle2));
        //entities.push(Shape2D::Rectangle(ground));
        
        // Our player is a rectangle
        let player = Rectangle::default();
        // We push the index of our player into our players vec
        players.push(entities.len());
        // Push the player into the entities array
        entities.push(Shape2D::Rectangle(player));

        Self {
            entities,
            players,
            keys_down: HashMap::new(),
        }
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
    /// Run logic on the inputs in [keys_down] HasMap
    /// 
    pub fn handle_user_inputs(&mut self){
        // This is temporary
        let player = match self.entities[self.players[Self::PLAYER_ONE]] {
            Shape2D::Rectangle(ref mut r) => {
                r
            }
            _=> todo!()
        };   
        for (key, _) in &self.keys_down {
            match key {
                // Move right
                Some(VirtualKeyCode::D) | Some(VirtualKeyCode::Right)  => { 
                    if player.x() >= 1.0 { player.set_x(-1.0) }
                    else { player.shift_x(0.02); }  
                },
                // Move Left
                Some(VirtualKeyCode::A) | Some(VirtualKeyCode::Left)  => { 
                    if player.x() <= -1.0 { player.set_x(1.0) }
                    else { player.shift_x(-0.02); }  
                },
                // Jump
                Some(VirtualKeyCode::W) | Some(VirtualKeyCode::Space)  => {
                    match player.state() {
                        State::None => {
                            player.set_state(State::Jumping(JUMP_TIME))
                        }
                        _=> {}
                    };
                },
                Some(key) => {
                    println!("We dont handle {key:?}");
                }
                None => todo!("WTF: {key:?}")
            };
        }
    }
    /// Runs game logic in a tick, also calls phsyics
    /// 
    pub fn update(&mut self){
        // Handle any user inputs
        self.handle_user_inputs();
        // Run the phsyics against our game
        super::physics::update(self);
    }
}