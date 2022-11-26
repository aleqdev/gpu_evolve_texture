
// --------------------------------------------------------
// PROCESSOR_SHADER_VARS
// --------------------------------------------------------

@group(0) @binding(0)
var display_texture: texture_storage_2d<rgba8unorm, read_write>;



// ----------
// flammability
// ----------
@group(0) @binding(1) var<storage, read_write> flammability : array<f32>;
fn process_flammability() {

}

// ----------
// heat
// ----------
@group(0) @binding(2) var<storage, read_write> heat : array<f32>;
fn process_heat() {

}

// ----------
// presence
// ----------
@group(0) @binding(3) var<storage, read_write> presence : array<f32>;
fn process_presence() {

}


// --------------------------------------------------------
// PROCESSOR_SHADER_INIT_BEGIN
// --------------------------------------------------------

@compute @workgroup_size(8, 8, 1)
fn init(
    @builtin(global_invocation_id) invocation_id: vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>
) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let color = vec4<f32>(0.);

    // --------------------------------------------------------
    // PROCESSOR_SHADER_INIT_END
    // --------------------------------------------------------

}

// --------------------------------------------------------
// PROCESSOR_SHADER_PROCESS_BEGIN
// --------------------------------------------------------

@compute @workgroup_size(8, 8, 1)
fn process(
    @builtin(global_invocation_id) invocation_id: vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>
) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let color = vec4<f32>(0., 0., 0., 255.);
    let color = color + process_flammability(location, color);
    let color = color + process_heat(location, color);
    let color = color + process_presence(location, color);

    // --------------------------------------------------------
    // PROCESSOR_SHADER_PROCESS_END
    // --------------------------------------------------------

    storageBarrier();

    textureStore(display_texture, location, color)
}
