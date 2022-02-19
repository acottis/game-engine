//! Here we will deal with all the game logic required
//!

use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use crate::entity::{Shape2D, Rectangle, Transform2D};

/// This will store our game state and pass it around
pub struct Game {
    pub entities: Vec<Shape2D>
}

impl Game {
    pub fn new() -> Self {
        let mut entities:Vec<Shape2D> = Vec::new();

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
        
        let player = Rectangle::default();
        println!("{:?}", player);
        //entities.push(Shape2D::Triangle(triangle));
        //entities.push(Shape2D::Triangle(triangle2));
        //entities.push(Shape2D::Rectangle(ground));
        entities.push(Shape2D::Rectangle(player));

        Self {
            entities,
        }
    }

    /// This is sent keyboard inputs from our event loop
    pub fn keyboard_input(&mut self, input: &KeyboardInput){
       
        // We only care when we press
        if input.state != ElementState::Pressed { return }
        match input.virtual_keycode {
            // Move right
            Some(VirtualKeyCode::D)  => {
                match self.entities[0] {
                    Shape2D::Rectangle(ref mut r) => {
                        if r.get_x() >= 1.0 { r.set_x(-1.0) }
                        r.shift_x(0.02);
                    }
                    _=> todo!()
                };  
            },
            Some(key) => {
                println!("We dont handle {key:?}");
            }
            None => todo!("WTF: {input:?}")
        }
    }  
}