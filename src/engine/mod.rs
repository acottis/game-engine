//! Here we will deal with all the game logic required
//! 

use crate::entity::{Shape2D, Point, Triangle, Rectangle, Transform2D};

/// This will store our game state and pass it around
pub struct Game {
    pub entities: Vec<Shape2D>
}

impl Game {
    pub fn new() -> Self {
        let mut entities:Vec<Shape2D> = Vec::new();

        let triangle = Triangle::new(
            Point::new(-1.0, -0.4),
            Point::new( 0.0,  1.0),
            Point::new( 1.0, -0.4),
            wgpu::Color::RED
        );
    
        let triangle2 = Triangle::new(
            Point::new(-1.0, -0.2),
            Point::new( 0.0,  1.0),
            Point::new( 1.0, -0.2),
            wgpu::Color::BLACK
        );
    
        let ground = Rectangle::new(
            Point::new(-1.0, -1.0),
            Point::new( 1.0, -0.4),
            Point::new( 1.0, -1.0),
            Point::new(-1.0, -0.4),
            wgpu::Color::GREEN,
        );
        
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

    pub fn update(&mut self){
        match self.entities[0]{
            Shape2D::Rectangle(ref mut r) => {
                // r.set_xy(0.0,0.0);
                // r.set_x(1.0);
                println!("{:?}", r.get_xy());
                if r.get_x() >= 1.0 { r.set_x(-1.0) }
                r.shift_x(0.02);
            }
            _=> todo!()
        };
    }
}