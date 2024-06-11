struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

@vertex
fn main(@location(0) position: vec2<f32>) -> VertexOutput {
    var result: VertexOutput;
    result.coord = position;
    result.position = vec4<f32>(position, 0.0, 1.0);
    return result;
}
