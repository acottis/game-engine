//! Here is our entity system, we describe our "Entities" that are objects
//! inside our game, we also do the maths on how to move them here
//! 
use super::physics::State;

#[derive(Debug, Clone, Copy)]
pub enum Shape2D{
    Triangle(Triangle),
    Rectangle(Rectangle),
}
/// Here are traits that must be implemented for each object
/// to move it in 2d space
pub trait Transform2D {
    /// Return the x value of the bottom left `C`
    fn x(&self) -> f32;
    /// Return the x,y of the bottom left `C`
    fn xy(&self) -> Point;
    /// Translate just the x coord
    fn shift_x(&mut self, x: f32);
    /// Translate just the y coord
    fn shift_y(&mut self, y: f32);
    /// Set the x coord to an arbitory value
    fn set_x(&mut self, x: f32);
    /// Set the x,y coords to arbitory values
    fn set_xy(&mut self, x: f32, y: f32);
}

/// Here are traits that are applied to entities such as
/// managing [super::physics::State]
pub trait Entity {
    // Get the state
    fn state(&self) -> State;
    // Set the state 
    fn set_state(&mut self, state: State);
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub colour: wgpu::Color,
    state: State,
}

/// See [Transform2D] for comments 
impl Transform2D for Rectangle{
    fn x(&self) -> f32 {
        self.c.x
    }
    fn xy(&self) -> Point {
        Point { x: self.c.x, y: self.c.y }
    }
    fn set_xy(&mut self, x: f32, y: f32) {
        let width = self.b.x - self.a.x;
        self.a.x = x;
        self.b.x = x + width;
        self.c.x = x;
        self.d.x = x + width;

        let height = self.a.y - self.c.y;
        self.a.y = y + height;
        self.b.y = y + height;
        self.c.y = y;
        self.d.y = y;
    }
    fn set_x(&mut self, x: f32) {
        let width = self.b.x - self.a.x;
        self.a.x = x;
        self.b.x = x + width;
        self.c.x = x;
        self.d.x = x + width;
    }
    fn shift_x(&mut self, x: f32) {
        self.a.x += x;
        self.b.x += x;
        self.c.x += x;
        self.d.x += x;
    }
    fn shift_y(&mut self, y: f32) {
        self.a.y += y;
        self.b.y += y;
        self.c.y += y;
        self.d.y += y;
    }
}

impl Entity for Rectangle {
    // Get the state
    fn state(&self) -> State {
        self.state
    }
    // Set the state 
    fn set_state(&mut self, state: State){
        self.state = state;
    }
}

impl Rectangle{
    pub fn new(a: Point, b: Point, c: Point, d: Point, colour: wgpu::Color) -> Self {
        Self { a, b, c, d, colour, state: State::None }
    }
}

/// C is the bottom Left of the screen
/// |
/// |
/// A-------B
/// | Black |
/// C-------D-------------
/// Its actually a square by default
impl Default for Rectangle{
    fn default() -> Self {
        Self { 
            a: Point::new(-1.0, -0.9), // A
            b: Point::new(-0.9, -0.9), // B
            c: Point::new(-1.0, -1.0), // C
            d: Point::new(-0.9, -1.0), // D
            colour: wgpu::Color::BLACK,
            state: State::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub colour: wgpu::Color,
}

impl Triangle{
    pub fn new(a: Point, b: Point, c: Point, colour: wgpu::Color) -> Self {
        Self { a, b, c, colour }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point{
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}