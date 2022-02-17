struct Vertex {
    [[location(0)]] position: vec3<f32>;
};


// This draws points
[[stage(vertex)]]
fn vs_main(v: Vertex) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(v.position[0], v.position[1], 0., 1.);
}


[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0); // This is the colour of the shape
}