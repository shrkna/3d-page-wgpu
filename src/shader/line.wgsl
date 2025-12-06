struct VertexOutput {
    @builtin(position) position   : vec4<f32>,
};

struct Uniform 
{
    transform_matrix   : mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> inUniform          : Uniform;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
) -> VertexOutput 
{
    var output: VertexOutput;

    output.position  = inUniform.transform_matrix * vec4<f32>(position, 1.0);
    return output;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> 
{
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}