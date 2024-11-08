use std::f32::consts::PI;

use nalgebra_glm::{Mat3, Mat4, Vec3, Vec4};
use crate::fragments::Fragment;
use crate::screen::color::Color;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;


pub fn vertex_shader(
    vertex: &Vertex,
    uniforms: &Uniforms
) -> Vertex{

    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Perspective division
    let w = transformed.w;
    let ndc_position  = Vec4::new(
        transformed.x/w,
        transformed.y/w,
        transformed.z/w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transform normal
  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;
  
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader_animated(fragment: &Fragment, uniforms: &Uniforms) ->Color {
    
    let colors = [
        Color::new(255, 0, 0),
        Color::new(0, 255, 0),
        Color::new(0, 0, 255),
        Color::new(255, 255, 0),
        Color::new(255, 0, 255),
        Color::new(0, 255, 255),
    ];
    let frames_per_color = 100;
    let color_index = (uniforms.time/frames_per_color) as usize % colors.len();

    let transition_progress = (uniforms.time/frames_per_color) as f32/ frames_per_color as f32;

    let current_color = colors[color_index];
    let next_color = colors[(color_index +1)%colors.len()];
    current_color.lerp(&next_color, transition_progress)*fragment.intensity
}

pub fn fragment_shader_stripes(fragment: &Fragment, uniforms: &Uniforms) ->Color {
    let color1 = Color::new(255, 0, 0);
    let color2 = Color::new(0, 0, 255);

    let stripe_width = 5.0;
    let speed = 0.002;

    let moving_y = fragment.position.y + uniforms.time as f32 * speed;
    let stripe_factor = ((moving_y/stripe_width)*PI).sin() * 0.5 + 0.5;
    
    color1.lerp(&color2, stripe_factor)*fragment.intensity
}

pub fn fragment_shader_spots(fragment: &Fragment, uniforms: &Uniforms) ->Color {
    let background_color = Color::new(250, 250, 250);
    let dot_color = Color::new(255, 0, 0);

    let dot_size = 0.1;
    let dot_spacing = 0.3;
    let speed = 0.01;

    let moving_x = fragment.position.x + uniforms.time as f32 * speed;
    let moving_y = fragment.position.y - uniforms.time as f32 * speed * 0.5;

    let pattern_x = ((moving_x/dot_spacing)*2.0 * PI).cos();
    let pattern_y = ((moving_y/dot_spacing)*2.0*PI).cos();

    let dot_pattern = (pattern_x*pattern_y).max(0.0);
    let dot_factor = (dot_pattern -(1.0 - dot_size)).max(0.0)/dot_size;
    background_color.lerp(&dot_color, dot_factor) * fragment.intensity
}


pub fn static_pattern_shader(fragment: &Fragment) ->Color {
    let x = fragment.position.x;
    let y = fragment.position.y;

    let pattern = ((x *10.0).sin() * (y*10.0).sin()).abs();
    let r = (pattern * 255.0) as u8;
    let g = ((1.0-pattern)*255.0) as u8;
    let b = 128;

    Color::new(r as i32, g as i32, b as i32)*fragment.intensity
}



fn purple_shader(_fragment: &Fragment) -> Color{
    Color::new(128, 0, 128)
}

fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.position.x;
    let y = fragment.position.y;
    let distance = ((x-400.0)*(x-400.0) + (y-300.0)*(y-300.0)).sqrt();

    if distance < 50.0 {
        Color::new(255, 255, 0)
    } else{
        Color::new(0, 0, 0)
    }
}

pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color  = purple_shader(fragment);
    let circle_color = circle_shader(fragment);

    let combined_color = match blend_mode {
        "normal" => base_color.blend_normal(&circle_color),
        "multiply" => base_color.blend_multiply(&circle_color),
        "add" => base_color.blend_add(&circle_color),
        "subtract" => base_color.blend_subtract(&circle_color),
        _ => base_color
    };

    combined_color * fragment.intensity
}

