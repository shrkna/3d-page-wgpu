const PI : f32 = radians(180.0);

struct Uniform
{
    directional_light  : vec4<f32>,
    ambient_light      : vec4<f32>,
    inverse_matrix     : mat4x4<f32>,
    buffer_type        : f32,
}

@group(0) @binding(0) var gbuffer_position : texture_2d<f32>;
@group(0) @binding(1) var gbuffer_normal   : texture_2d<f32>;
@group(0) @binding(2) var gbuffer_depth    : texture_depth_2d;
@group(0) @binding(3) var gbuffer_albedo   : texture_2d<f32>;
@group(0) @binding(4) var gbuffer_metallic : texture_2d<f32>;
@group(1) @binding(0) var<uniform> in_uniform: Uniform;

// pbr utility

fn distribution_GGX(normal : vec3<f32>, half : vec3<f32>, a : f32) -> f32
{
    let a2       : f32 = a * a;
    let n_dot_h  : f32 = max(dot(normal, half), 0.0);
    let n_dot_h2 : f32 = n_dot_h * n_dot_h;
	
    let nom   : f32    = a2;
    var denom : f32    = (n_dot_h2 * (a2 - 1.0) + 1.0);
    denom              = PI * denom * denom;
	
    return nom / denom;
}
fn geometry_schlick_GGX(n_dot_v : f32, k : f32) -> f32
{
    let nom   : f32 = n_dot_v;
    let denom : f32 = n_dot_v * (1.0 - k) + k;
	
    return nom / denom;
}
fn geometry_smith(n: vec3<f32>, v : vec3<f32>, l : vec3<f32>, k : f32) -> f32
{
    let n_dot_v : f32 = max(dot(n, v), 0.0);
    let n_dot_l : f32 = max(dot(n, l), 0.0);
    let ggx1 : f32 = geometry_schlick_GGX(n_dot_v, k);
    let ggx2 : f32 = geometry_schlick_GGX(n_dot_l, k);
	
    return ggx1 * ggx2;
}
fn fresnel_schlick(cos_theta :f32, f0 : vec3<f32>) -> vec3<f32>
{
    return f0 + (1.0 - f0) * pow(1.0 - cos_theta, 5.0);
}

// render funcions

@vertex
fn vs_main( @builtin(vertex_index) VertexIndex : u32 ) -> @builtin(position) vec4f 
{
  const pos = array(
    vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0),
    vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(1.0, 1.0),
  );

  return vec4f(pos[VertexIndex], 0.0, 1.0);
}

@fragment
fn fs_main( @builtin(position) coord : vec4f ) -> @location(0) vec4f
{
    let position : vec4f     = textureLoad( gbuffer_position, vec2i(floor(coord.xy)), 0 );
    var normal   : vec3<f32> = textureLoad( gbuffer_normal, vec2i(floor(coord.xy)), 0 ).xyz;
    var depth    : f32       = textureLoad( gbuffer_depth, vec2i(floor(coord.xy)), 0 );
    var albedo   : vec4<f32> = textureLoad( gbuffer_albedo, vec2i(floor(coord.xy)), 0 );

    if (depth >= 1.0) 
    {
      discard;
    }

    let directional_light : vec3<f32> = normalize(in_uniform.directional_light.xyz);
    let diffuse           : f32       = max(dot(-1.0 * directional_light, normal), 0.0);

    let view     : vec3<f32> = normalize((in_uniform.inverse_matrix * position).xyz);
    let halfway  : vec3<f32> = -normalize(directional_light.xyz + view);
    let specular : f32       = pow(max(dot(normal, halfway), 0.0), 100.0);

    let ambient_light     : vec4<f32> = in_uniform.ambient_light;

    let surface_color  : vec4<f32> = albedo;
    let specular_color : vec4<f32> = vec4(1.0, 1.0, 1.0, 1.0);

    var frag_color = diffuse * surface_color + specular * specular_color + ambient_light;
    return frag_color;
}

@fragment
fn fs_debug_main( @builtin(position) coord : vec4f ) -> @location(0) vec4f
{
    let position : vec4f     = textureLoad( gbuffer_position, vec2i(floor(coord.xy)), 0 );
    var normal   : vec3<f32> = textureLoad( gbuffer_normal, vec2i(floor(coord.xy)), 0 ).xyz;
    var depth    : f32       = textureLoad( gbuffer_depth, vec2i(floor(coord.xy)), 0 );
    let albedo   : vec4<f32> = textureLoad( gbuffer_albedo, vec2i(floor(coord.xy)), 0 );
    let metallic : vec4<f32> = textureLoad( gbuffer_metallic, vec2i(floor(coord.xy)), 0 );

    normal.x = (normal.x + 1.0) * 0.5;
    normal.y = (normal.y + 1.0) * 0.5;
    normal.z = (normal.z + 1.0) * 0.5;

    depth = (1.0 - depth) * 50.0;

    // ummm
    if(in_uniform.buffer_type == 1.0)
    {
      return vec4(normal, 1.0);
    }
    else if(in_uniform.buffer_type == 2.0)
    {
      return vec4(depth, 0.0, 0.0, 1.0);
    }
    else if(in_uniform.buffer_type == 3.0)
    {
      return albedo;
    }
    else if(in_uniform.buffer_type == 4.0)
    {
      return metallic;
    }

    return vec4(depth, 0.0, 0.0, 1.0);
}