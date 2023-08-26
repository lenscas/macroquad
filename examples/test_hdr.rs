use macroquad::prelude::*;

#[macroquad::main("Post processing")]
async fn main() {
    let render_target_t = render_target_with_format(320, 150,miniquad::TextureFormat::RGBA16);
    //let render_target_t = render_target(320,150);
    render_target_t.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource {
            glsl_vertex: Some(CRT_VERTEX_SHADER),
            glsl_fragment: Some(CRT_FRAGMENT_SHADER),
            metal_shader: None,
        },
        Default::default(),
    )
    .unwrap();
    loop {
        // drawing to the texture

        // 0..100, 0..100 camera
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target_t.clone()),
            ..Default::default()
        });

        //clear_background(Color::new(2., 2., 2., 1.));
        clear_background(WHITE);
        draw_line(-30.0, 45.0, 30.0, 45.0, 3.0, BLUE);
        draw_circle(-45.0, -35.0, 20.0, YELLOW);
        draw_circle(45.0, -35.0, 20.0, GREEN);

        // drawing to the screen

        set_default_camera();
        
        clear_background(WHITE);
        draw_texture(&render_target_t.texture, 0., 0., WHITE);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target_t.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await;
        
     }
}

const CRT_FRAGMENT_SHADER: &str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;
    
uniform sampler2D Texture;

void main() {
    vec3 res = texture2D(Texture, uv).rgb;
 	
    if (res.x < 1.01 && res.y < 1.01 && res.z < 1.01)
    {
        res = vec3(0.0, 0.0, 0.0);
    } 
    gl_FragColor = vec4(res, 1.0);

}
"#;

const CRT_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";