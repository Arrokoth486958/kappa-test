// 顶点着色器

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec4f,
    @location(2) uv: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // let flipped_y = 1.0 - model.position.y;
    // let x = (model.position.x + 1.0) * 0.5;
    // let y = (flipped_y + 1.0) * 0.5;
    out.color = model.color;
    // 不转换坐标的情况下
    // out.clip_position = vec4f(model.position, 1.0);
    // 坐标转换
    out.clip_position = vec4f(model.position.x * 2.0 - 1.0, model.position.y * -2.0 + 1.0, model.position.z, 1.0);
    return out;
}

// 片元着色器

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
    // return vec4f(pow(in.color.r, 2.2), pow(in.color.g, 2.2), pow(in.color.b, 2.2), pow(in.color.a, 2.2));
}