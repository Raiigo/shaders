struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

fn inv_lerp(a: f32, b: f32, value: f32) -> f32 {
    return ((value - a) / (b - a));
}

[[stage(vertex)]]
fn vertex([[builtin(vertex_index)]] in_vertex_index: u32, [[location(1)]] in_world_normal: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.world_normal = in_world_normal;
    return out;
}

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.world_normal[0], input.world_normal[1], input.world_normal[2], 1.0);
}