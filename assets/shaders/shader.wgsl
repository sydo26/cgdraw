struct Camera {
    view_proj: mat4x4<f32>,
    // Será usado quando for implementado texturas e sistemas de iluminação
    view_position: vec4<f32>
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_space: vec4<f32>,
    @location(0) color: vec3<f32>
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    let world_space = vec4<f32>(model.position, 1.0);

    // Transforma o vértice para o espaço de tela
    out.clip_space = camera.view_proj * world_space;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}