#[derive(Debug)]
pub enum Shape2D{
    Triangle(Triangle),
    Rectangle(Rectangle)
}

#[derive(Debug)]
pub struct Rectangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Rectangle{
    pub fn new(a: Point, b: Point, c: Point, d: Point) -> Self {
        Self { a, b, c, d }
    }
}

#[derive(Debug)]
pub struct Triangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle{
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point{
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}