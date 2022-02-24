//! Here is our entity system, we describe our "Entities" that are objects
//! inside our game, we also do the maths on how to move them here. We define
//! how we want our objects to behave here
use super::physics::State;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Shape2D{
    Triangle(Triangle),
    Rectangle(Rectangle),
    Pentagon(Pentagon),
}
/// Here are traits that must be implemented for each object
/// to move it in 2d space
pub trait Transform2D {
    /// Return the x value of the bottom left `C`
    fn x(&self) -> f32;
    /// Return the y value of the bottom left `C`
    fn y(&self) -> f32;
    /// Return the x,y of the bottom left `C`
    fn xy(&self) -> Point;
    /// Translate just the x coord
    fn shift_x(&mut self, x: f32);
    /// Translate just the y coord
    fn shift_y(&mut self, y: f32);
    /// Shift the x,y coords to arbitory values
    fn shift_xy(&mut self, x: f32, y: f32) {
        self.shift_x(x);
        self.shift_y(y);
    }
    /// Set the x coord to an arbitory value
    fn set_x(&mut self, x: f32);
    /// Set the y coord to an arbitory value
    fn set_y(&mut self, y: f32);
    /// Set the x,y coords to arbitory values
    fn set_xy(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
    // Find the y value with the highest value
    fn max_y(&self) -> f32;    
    // Find the x value with the highest value
    fn max_x(&self) -> f32;
}

/// Here are traits that are applied to entities such as
/// managing [super::physics::State]
pub trait Entity {
    // Get the state
    fn state(&self) -> State;
    // Set the state 
    fn set_state(&mut self, state: State);
    // Get the collision status
    fn collides(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub colour: wgpu::Color,
    state: State,
    collides: bool,
}

impl Rectangle{
    pub fn new(a: Point, b: Point, c: Point, d: Point, colour: wgpu::Color) -> Self {
        Self { a, b, c, d, colour, state: State::None, collides: true }
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
            a:          Point::new(-1.0, -0.9), // A
            b:          Point::new(-0.9, -0.9), // B
            c:          Point::new(-1.0, -1.0), // C
            d:          Point::new(-0.9, -1.0), // D
            colour:     wgpu::Color::BLACK,
            state:      State::None,
            collides:  true,
        }
    }
}
// Our respresentation of a Pentagon entity
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pentagon{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub e: Point,
    pub colour: wgpu::Color,
    state: State,
    collides: bool,
}
/// reference: https://mathworld.wolfram.com/RegularPentagon.html
/// C is the bottom Left of the screen
/// |    B
/// |  /   \
/// |A/     \D
/// | \Black/
/// |  \___/
/// ---C---E--------------
/// ^^default^^
impl Default for Pentagon{
    fn default() -> Self {
        // Probably should precompute this, or calculate before game starts
        const B_X: f32 = 0.0 / 10.0;
        const B_Y: f32 = 1.0 / 10.0;
        let c1 = f32::cos(2.0 * PI / 5.0) / 10.0;
        let c2 = f32::cos(      PI / 5.0) / 10.0;
        let s1 = f32::sin(2.0 * PI / 5.0) / 10.0;
        let s2 = f32::sin(4.0 * PI / 5.0) / 10.0;

        Self {
            a: Point::new(-s1,  c1), // A
            b: Point::new(   B_X,  B_Y), // B
            c: Point::new(-s2,  -c2), // C
            d: Point::new( s1,  c1), // D
            e: Point::new( s2,  -c2), // E
            colour: wgpu::Color::BLACK,
            state: State::None,
            collides: false,
        }
    }
}
// Our respresentation of a triangle entity
#[derive(Debug, Clone, Copy)]
pub struct Triangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub colour: wgpu::Color,
    state: State,
    collides: bool,
}
/// C is the bottom Left of the screen
/// |
/// |   A
/// | /   \
/// |/Black\
/// C-------B-------------
/// ^^default^^
impl Default for Triangle{
    fn default() -> Self {
        Self { 
            a: Point::new(-0.95, -0.9), // A
            b: Point::new(-0.9,  -1.0), // B
            c: Point::new(-1.0,  -1.0), // C
            colour: wgpu::Color::BLACK,
            state: State::None,
            collides: false,
        }
    }
}


// impl Triangle{
//     pub fn new(a: Point, b: Point, c: Point, colour: wgpu::Color) -> Self {
//         Self { a, b, c, colour, state: State::None }
//     }
// }

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

/// See [Transform2D] for comments 
impl Transform2D for Shape2D {
    fn x(&self) -> f32 {
        match &self {
            Shape2D::Triangle(t) => { t.c.x },
            Shape2D::Rectangle(r) => { r.c.x },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn y(&self) -> f32 {
        match &self {
            Shape2D::Triangle(t) => { t.c.y },
            Shape2D::Rectangle(r) => { r.c.y },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn xy(&self) -> Point {
        match &self {
            Shape2D::Triangle(t) => { Point { x: t.c.x, y: t.c.y } },
            Shape2D::Rectangle(r) => { Point { x: r.c.x, y: r.c.y } },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn set_x(&mut self, x: f32) {
        match self {
            Shape2D::Triangle(ref mut t) => {
                let ca_side_y = t.a.x - t.c.x;
                let cb_side_x = t.b.x - t.c.x;
                t.a.x = x + ca_side_y;
                t.b.x = x + cb_side_x;
                t.c.x = x;
            },
            Shape2D::Rectangle(ref mut r) => { 
                let width = r.b.x - r.a.x;
                r.a.x = x;
                r.b.x = x + width;
                r.c.x = x;
                r.d.x = x + width;
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn set_y(&mut self, y: f32) {
        match self {
            Shape2D::Triangle(ref mut t) => {
                let ca_side_y = t.a.y - t.c.y;
                let cb_side_y = t.b.y - t.c.y;
                t.a.y = y + ca_side_y;
                t.b.y = y + cb_side_y;
                t.c.y = y;
            },
            Shape2D::Rectangle(ref mut r) => {
                let height = r.a.y - r.c.y;
                r.a.y = y + height;
                r.b.y = y + height;
                r.c.y = y;
                r.d.y = y;
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn shift_x(&mut self, x: f32) {
        match self {
            Shape2D::Triangle(ref mut t) => {
                t.a.x += x;
                t.b.x += x;
                t.c.x += x;
            },
            Shape2D::Rectangle(ref mut r) => {
                r.a.x += x;
                r.b.x += x;
                r.c.x += x;
                r.d.x += x;
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn shift_y(&mut self, y: f32) {
        match self {
            Shape2D::Triangle(ref mut t) => {
                t.a.y += y;
                t.b.y += y;
                t.c.y += y;
            },
            Shape2D::Rectangle(ref mut r) => {
                r.a.y += y;
                r.b.y += y;
                r.c.y += y;
                r.d.y += y;
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
            _ => {}
        }
    }
    fn max_y(&self) -> f32 {
        match &self {
            Shape2D::Triangle(t) => { unimplemented!() },
            Shape2D::Rectangle(r) => { 
                let mut max = r.a.y;
                if r.b.y > max { max = r.b.y }
                if r.c.y > max { max = r.c.y }
                if r.d.y > max { max = r.d.y }
                max
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
    fn max_x(&self) -> f32 {
        match &self {
            Shape2D::Triangle(t) => { 
                let mut max = t.a.x;
                if t.b.x > max { max = t.b.x }
                if t.c.x > max { max = t.c.x }
                max
             },
            Shape2D::Rectangle(r) => { 
                let mut max = r.a.x;
                if r.b.x > max { max = r.b.x }
                if r.c.x > max { max = r.c.x }
                if r.d.x > max { max = r.d.x }
                max
            },
            Shape2D::Pentagon(_) => { unimplemented!() }
        }
    }
}

impl Entity for Shape2D {
    // Get the state
    fn state(&self) -> State {
        match self {
            Shape2D::Triangle(t) => {
                t.state
            },
            Shape2D::Rectangle(r) => {
                r.state
            },
            Shape2D::Pentagon(p) => {
                p.state
            },
        }
    }
    // Set the state 
    fn set_state(&mut self, state: State){
        match self {
            Shape2D::Triangle(ref mut t) => {
                t.state = state
            },
            Shape2D::Rectangle(ref mut r) => {
                r.state = state
            },
            Shape2D::Pentagon(ref mut p) => {
                p.state = state
            },
        }
    }
    // Get if the object collides
    fn collides(&self) -> bool {
        match self {
            Shape2D::Triangle(t) => {
                t.collides
            },
            Shape2D::Rectangle(r) => {
                r.collides
            },
            Shape2D::Pentagon(p) => {
                p.collides
            },
        }
    }
}