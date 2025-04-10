#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    let uv = in.uv;
    let centre = vec2<f32>(0.5, 0.5);

    // Define the size of each pixel block
    let pixel_size = 0.05; // Keep this the same for consistent detail

    // Snap the UV coordinates to the nearest pixel block
    let pixel_uv = vec2<f32>(
        floor(uv.x / pixel_size) * pixel_size,
        floor(uv.y / pixel_size) * pixel_size
    );

    // Calculate distance from the centre
    let dist = length(pixel_uv - centre);

    // Define even smaller thresholds for a smaller pixelated circle
   let threshold1 = 0.04; // Further adjusted for smaller size
      let threshold2 = 0.08;
      let threshold3 = 0.12;
      let threshold4 = 0.16;

    // Use step functions to create a gradient effect
    let intensity1 = 1.0 - step(threshold1, dist);
    let intensity2 = step(threshold1, dist) * (1.0 - step(threshold2, dist));
    let intensity3 = step(threshold2, dist) * (1.0 - step(threshold3, dist));
    let intensity4 = step(threshold3, dist) * (1.0 - step(threshold4, dist));

    // Combine intensities to create a gradient
    let shadow_intensity = intensity1 * 0.8 + intensity2 * 0.6 + intensity3 * 0.4 + intensity4 * 0.2;

    // Return a color with pixelated shadow effect
    let shadow_color = vec4<f32>(0.0, 0.0, 0.0, shadow_intensity);
    return shadow_color;
}
