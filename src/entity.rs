#[derive(Debug, Clone, Copy)]
pub enum Shape2D{
    Triangle(Triangle),
    Rectangle(Rectangle)
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub colour: wgpu::Color,
}

impl Rectangle{
    pub fn new(a: Point, b: Point, c: Point, d: Point, colour: wgpu::Color) -> Self {
        Self { a, b, c, d, colour }
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