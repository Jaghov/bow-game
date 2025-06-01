

struct DitheredMaterial {
    color: vec4<f32>
}


@group(2) @binding(0)
var<uniform> material: DitheredMaterial ;


@fragment
fn fragment() -> @location(0) vec4<f32> {
    let out = material.color;

    return out;
}
