@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    let dimensions = textureDimensions(input_texture);
    let tex_coords = global_id.xy;

    if (tex_coords.x >= dimensions.x || tex_coords.y >= dimensions.y) {
        return;
    }

    let in_color = textureLoad(input_texture, tex_coords.xy, 0);
    // let gray = dot(vec3<f32>(0.299, 0.587, 0.114), color.rgb);
    // let grayscale = vec4<f32>(gray, gray, gray, color.a);
    // let out_color = (color + grayscale) / 2.0;
    let out_color = vec4<f32>(in_color.r * (sin(f32(tex_coords.x)) + 1) / 2, in_color.g * (sin(f32(tex_coords.y)) + 1) / 2, in_color.b, in_color.a);

    textureStore(output_texture, tex_coords.xy, out_color);
}