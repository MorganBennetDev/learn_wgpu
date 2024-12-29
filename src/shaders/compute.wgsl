@group(0) @binding(0)
var input_texture: texture_2d<f32>;
@group(0) @binding(1)
var output_texture: texture_storage_2d<rgba8unorm, write>;
struct ComputeParameters {
    time: u32
}
@group(0) @binding(2)
var<uniform> parameters: ComputeParameters;

fn scan_filter(color: vec4<f32>, coords: vec2<u32>, tex_coords: vec2<f32>) -> vec4<f32> {
    let line_tex_coords = f32(parameters.time % 10000) / 10000.0;
    let line_tex_offset = (tex_coords.y - line_tex_coords) * 50;
    let line_mag = exp(-(line_tex_offset * line_tex_offset));

    return mix(color, vec4<f32>(1.0, 1.0, 1.0, 1.0), line_mag * line_mag);
}

const PI = radians(180.0);
const MASK_SIZE = 5.0;
const MASK_BORDER = vec2<f32>(1.0, 1.0);

// Based on https://www.shadertoy.com/view/DtscRf
fn crt_filter(color: vec4<f32>, coords: vec2<u32>, tex_coords: vec2<f32>) -> vec4<f32> {
    let cell_coords = vec2<f32>(f32(coords.x), f32(coords.y)) / MASK_SIZE;
    let subcell_coords = cell_coords * vec2<f32>(3.0, 1.0);

    let cell_offset = vec2<f32>(0, fract(floor(cell_coords.x) * 0.5));

    let ind = floor(subcell_coords.x) % 3;
    var mask_color = vec3<f32>(f32(ind == 0), f32(ind == 1), f32(ind == 2)) * 3.0;
    let cell_uv = fract(subcell_coords + cell_offset) * 2.0 - 1.0;
    let border = 1.0 - cell_uv * cell_uv * MASK_BORDER;
    mask_color *= border.x * border.y;

    let mask_coord = floor(cell_coords + cell_offset) * MASK_SIZE;

    return color * vec4<f32>(mask_color.rgb, 1.0);
}

const BLOOM_START = 0.6;
const BLOOM_END = 0.8;
const BLOOM_SAMPLES = 32.0;
const BLOOM_RADIUS = 16.0;
const BLOOM_BASE = 0.5;
const BLOOM_GLOW = 3.0;

fn bloom_filter(color: vec4<f32>, coords: vec2<u32>, tex_coords: vec2<f32>) -> vec4<f32> {
    // let luma = dot(color, vec4<f32>(0.299, 0.587, 0.114, 0.0));
    // let bloom = smoothstep(BLOOM_START, BLOOM_END, luma);
    let resolution = textureDimensions(input_texture);
    let texel = vec2<f32>(1 / f32(resolution.x), 1 / f32(resolution.y));

    var bloom = vec4<f32>(0);
    var point = vec2<f32>(BLOOM_RADIUS, 0) * inverseSqrt(BLOOM_SAMPLES);

    for (var i = 0.0; i < BLOOM_SAMPLES; i += 1.0) {
        point *= -mat2x2<f32>(0.7374, 0.6755, -0.6755, 0.7374);

        let bloom_tex_coords = (tex_coords + point * sqrt(i)) * texel;

        let bloom_coords = vec2<u32>(u32(bloom_tex_coords.x * f32(resolution.x)), u32(bloom_tex_coords.y * f32(resolution.y)));

        bloom += textureLoad(input_texture, bloom_coords.xy, 0) * (1.0 / BLOOM_SAMPLES);
    }

    bloom *= BLOOM_GLOW / BLOOM_SAMPLES;
    bloom += color * BLOOM_BASE;

    return bloom;
}

@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    let dimensions = textureDimensions(input_texture);
    let coords = global_id.xy;

    if (coords.x >= dimensions.x || coords.y >= dimensions.y) {
        return;
    }

    let tex_coords = vec2(f32(coords.x) / f32(dimensions.x), f32(coords.y) / f32(dimensions.y));

    let scan = scan_filter(textureLoad(input_texture, coords.xy, 0), coords, tex_coords);
    let crt = crt_filter(scan, coords, tex_coords);
    let bloom = bloom_filter(crt, coords, tex_coords);

    textureStore(output_texture, coords.xy, bloom);
}