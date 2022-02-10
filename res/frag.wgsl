struct FragmentInput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] frag_position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

struct BaseLight
{
    color: vec3<f32>;
    ambient_intensity: f32;
    diffuse_intensity: f32;
};

struct Attenuation
{
    constant: f32;
    linear: f32;
    exp: f32;
};

struct PointLight
{
    base: BaseLight;
    position: vec4<f32>;
    atten: Attenuation;
};

[[group(1), binding(0)]] var albedo_texture: texture_2d<f32>;
[[group(1), binding(1)]] var normal_texture: texture_2d<f32>;
[[group(1), binding(2)]] var texture_sampler: sampler;
[[group(1), binding(3)]] var normal_sampler: sampler;

[[group(2), binding(1)]]
var<uniform> point_light: PointLight;

[[stage(fragment)]]
fn fs(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let albedo = textureSample(albedo_texture, texture_sampler, in.uv).rgb;
    var normal = textureSample(normal_texture, normal_sampler, in.uv).rgb;
    normal.g = 1.0 - normal.g;
    normal = normalize(normal * 2.0 - 1.0);

    var direction = vec3<f32>(point_light.position.xy, 1.0) - in.frag_position.xyz;
    let distance = length(direction);
    direction  = normalize(direction);
    let max_dot = max(dot(normal, direction), 0.0);
    let diffuse = point_light.base.color * point_light.base.diffuse_intensity * max_dot;
    let attenuation = 1.0 / ((point_light.atten.constant) + (point_light.atten.linear * distance) + (point_light.atten.exp * distance * distance));

    return vec4<f32>(albedo * diffuse * attenuation, 1.0);
}