struct VertexOutput {
    @builtin(position) position   : vec4<f32>,
	@location(0)       normal     : vec3<f32>,
    @location(1)       uv         : vec2<f32>,
    @location(2)       tangent    : vec3<f32>,
};

struct Uniform 
{
    transform_matrix   : mat4x4<f32>,
    rotation_matrix    : mat4x4<f32>,
    directional_light  : vec4<f32>,
    ambient_light      : vec4<f32>,
    inverse_matrix     : mat4x4<f32>,
    buffer_type        : f32,
}

@group(0) @binding(0) var<uniform> inUniform          : Uniform;
@group(1) @binding(0) var          base_color_texture : texture_2d<f32>;
@group(1) @binding(1) var          base_color_sampler : sampler;
@group(1) @binding(2) var          normal_texture     : texture_2d<f32>;
@group(1) @binding(3) var          normal_sampler     : sampler;

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) normal  : vec3<f32>,
    @location(2) uv       : vec2<f32>,
    @location(3) tangent  : vec3<f32>,
) -> VertexOutput 
{
    let normal_world  : vec3<f32> = normalize(inUniform.rotation_matrix * vec4<f32>(normal, 1.0)).xyz;
	let tangent_world : vec3<f32> = normalize(inUniform.rotation_matrix * vec4<f32>(tangent, 1.0)).xyz;

    var output: VertexOutput;
    output.position = inUniform.transform_matrix * position;
    output.normal   = normal_world;
    output.uv       = uv;
    output.tangent  = tangent_world;
    return output;
}

@fragment
fn fs_main(
    vertex: VertexOutput
) -> @location(0) vec4<f32> 
{
    let binormal_world : vec3<f32>   = normalize(cross(vertex.normal, vertex.tangent));
	let tbn_matrix     : mat3x3<f32> = mat3x3<f32>(vertex.tangent, binormal_world, vertex.normal);

    let encoded_normal : vec3<f32> = textureSample(normal_texture, normal_sampler, vertex.uv).rgb;
    let surface_normal : vec3<f32> = normalize(encoded_normal - 0.5);

    let directional_light : vec3<f32> = normalize(inUniform.directional_light.xyz);
    let normal            : vec3<f32> = normalize(tbn_matrix * surface_normal);
    let diffuse           : f32       = max(dot(-1.0 * directional_light, normal), 0.0);

    let view     : vec3<f32> = normalize((inUniform.inverse_matrix * vertex.position).xyz);
    let halfway  : vec3<f32> = -normalize(directional_light.xyz + view);
    let specular : f32       = pow(max(dot(normal, halfway), 0.0), 300.0);

    let ambient_light     : vec4<f32> = inUniform.ambient_light;

    let surface_color  : vec4<f32> = textureSample(base_color_texture, base_color_sampler, vertex.uv);;
    let specular_color : vec4<f32> = vec4(1.0, 1.0, 1.0, 1.0);

    var frag_color = diffuse * surface_color + specular * surface_color + ambient_light;

    // ummm
    if(inUniform.buffer_type == 1.0)
    {
      return vec4((normal / 2.0 + 0.5), 1.0);
    }

    return frag_color;
}