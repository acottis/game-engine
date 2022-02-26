// Represents our Vertex2D struct in rust, this comes from buffer
struct VertexIn {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] colour: vec4<f32>;
};

// We store the location here and pass the colour to the fragment shader
// This comes from vs_main()
struct VertexOut{
    [[location(0)]] colour: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

// This draws vertexes, every 3 make a triangle
[[stage(vertex)]]
fn vs_main(v: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(v.position[0], v.position[1], 0.0, 1.0);
    out.colour = vec4<f32>(v.colour);
    return out;
}

// Colours in sets of 3 vertices
[[stage(fragment)]]
fn fs_main(v: VertexOut) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(v.colour); // This is the colour of the shape
}