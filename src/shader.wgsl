struct Test {
    [[location(0)]] position: vec3<f32>;
};

// [[stage(vertex)]]
// fn vs_main( 
//     [[builtin(vertex_index)]] in_vertex_index: u32,
//     [[location(0)]] test: Test,        
// ) -> [[builtin(position)]] vec4<f32> {
//     if (i32(in_vertex_index) == 2) {
//         return vec4<f32>(0.5,0.5,0.0,1.0);
//     }
//     if (i32(in_vertex_index) == 3) {
//         return vec4<f32>(-0.5,0.5,0.0,1.0);
//     }
//     let x = f32(i32(in_vertex_index) - 1);
//     let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
//     return vec4<f32>(x, y, 0., 1.);
// }

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
    test: Test
) -> [[builtin(position)]] vec4<f32> {
    // let x = f32(i32(in_vertex_index) - 1);
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    // return vec4<f32>(x, y, 0., 1.);
    return vec4<f32>(test.position[0], test.position[1], 0., 1.);
}


[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0); // This is the colour of the shape
}