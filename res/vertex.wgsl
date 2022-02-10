struct View {
    view_proj: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> view: View;

struct Mesh {
    mesh: mat4x4<f32>;
};
[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct VertexInput {
    [[location(0)]] position : vec3<f32>;
    [[location(1)]] uv : vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] frag_position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position =  view.view_proj * mesh.mesh * vec4<f32>(in.position, 1.0);
    out.frag_position = vec3<f32>(mesh.mesh * vec4<f32>(in.position, 1.0)).xyz;
    out.uv = in.uv;
    return out;
}