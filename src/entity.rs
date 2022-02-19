#[derive(Debug, Clone, Copy)]
pub enum Shape2D{
    Triangle(Triangle),
    Rectangle(Rectangle)
}

pub trait Transform2D {
    /// Return the x value of the bottom left `C`
    fn get_x(&self) -> f32;
    /// Return the x,y of the bottom left `C`
    fn get_xy(&self) -> Point;
    /// Translate just the x coord
    fn shift_x(&mut self, x: f32);
    /// Set the x coord to an arbitory value
    fn set_x(&mut self, x: f32);
    /// Set the x,y coords to arbitory values
    fn set_xy(&mut self, x: f32, y: f32);
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub colour: wgpu::Color,
}

/// See [Transform2D] for comments 
impl Transform2D for Rectangle{
    fn get_x(&self) -> f32 {
        self.c.x
    }
    fn set_xy(&mut self, x: f32, y: f32) {
        todo!();
    }
    fn set_x(&mut self, x: f32) {
        let width = self.b.x - self.a.x;
        self.a.x = x;
        self.b.x = x + width;
        self.c.x = x;
        self.d.x = x + width;
    }
    fn get_xy(&self) -> Point {
        Point { x: self.c.x, y: self.c.y }
    }
    fn shift_x(&mut self, x: f32) {
        self.a.x += x;
        self.b.x += x;
        self.c.x += x;
        self.d.x += x;
    }
}

impl Rectangle{
    pub fn new(a: Point, b: Point, c: Point, d: Point, colour: wgpu::Color) -> Self {
        Self { a, b, c, d, colour }
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
            colour: wgpu::Color::BLACK }
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