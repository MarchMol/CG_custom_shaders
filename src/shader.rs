use std::f32::consts::PI;
use std::ops::Add;
use std::os::unix::raw::gid_t;

use nalgebra_glm::{Mat3, Mat4, Vec3, Vec4};
use rand::{Rng, SeedableRng};
use crate::fragments::Fragment;
use crate::screen::color::{self, Color};
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;
use crate::rand::rngs::StdRng;

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

// SUN
pub fn sun_cellular_shader(fragment: &Fragment, uniforms: &Uniforms) ->Color{
    let zoom = 5.0;
    let ox = 50.0 + uniforms.time as f32;
    let oy = 50.0;
    let x = fragment.position.x;
    let y = fragment.position.y;

    let cell_noise_value = (uniforms.noise.get_noise_2d(
       x*zoom +ox, 
        y*zoom+oy
    )+1.0)/2.0;

    let dark = Color::from_hex(0xff2a00);
    let normal = Color::from_hex(0xff5100);
    let white =Color::new(255,255, 255);
    let final_color = if cell_noise_value< 0.1{
        dark*(cell_noise_value+0.5)
    } else if cell_noise_value< 0.55{
        normal*(cell_noise_value+0.4)
    } else{
        white*(cell_noise_value+0.2)
    };
    final_color
}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let spot_color = sun_cellular_shader(fragment, uniforms);
    let brighter_color = Color::from_hex(0xffe0ad);
    let final_color = spot_color.blend_multiply(&brighter_color);
    final_color
}

// EARTH
pub fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let map_color = create_map(fragment, uniforms);
    let cloud = cloud_shader(fragment, uniforms);
    let final_color = map_color.blend_add(&cloud);

    final_color*(fragment.intensity.min(2.0).max(0.05))
}

fn create_map(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 0.5;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise_big = ((uniforms.noise.get_noise_2d(
        (x+100.0)*zoom,(y+100.0)*zoom
     )+1.0)/2.0).max(0.0).min(1.0);

     let noise_small = ((uniforms.noise.get_noise_2d(
        (x+100.0)*8.0,(y+100.0)*8.0
     )+1.0)/2.0).max(0.0).min(1.0);

     let noise = noise_big*0.7+noise_small*0.3;
    let ocean = Color::from_hex(0x000d47);

    let final_color = if noise<0.4{
        biome_color(fragment, uniforms)
    } else {
        ocean
    };
    final_color
}

fn biome_color(fragment: &Fragment,uniforms: &Uniforms)-> Color{ 
    let zoom = 3.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise_small = ((uniforms.noise.get_noise_2d(
        (x+100.0)*zoom,(y+100.0)*zoom
     )+1.0)/2.0).max(0.0).min(1.0);
    let greenary = Color::from_hex(0x053300);
    let desert = Color::from_hex(0x7d6902);

    let final_color = if noise_small<0.2{
        desert
    } else if noise_small<0.8{
        let new_desert = desert*noise_small;
        greenary.blend_add(&new_desert)
    } else{
        greenary
    };
    final_color
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{

    let x = fragment.position.x;
    let y = fragment.position.y;
    let t = uniforms.time as f32*0.5;
    let noise_big = ((uniforms.noise.get_noise_2d(
        (x)*1.4+t*8.0,y*1.4
     )+1.0)/2.0).max(0.0).min(1.0);

     let noise_small = ((uniforms.noise.get_noise_2d(
        (x)*5.0+t*15.0,y*5.0
     )+1.0)/2.0).max(0.0).min(1.0);

     let noise = noise_small*0.3 + noise_big*0.7;
 
     let white = Color::new(255, 255, 255);
     let black  = Color::black();

    let final_color = if noise<0.4{
        white*(0.6-noise)
    } else{
        black
    };
    final_color
}
// MERCURY
pub fn mercury_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{
    let craters =mercury_craters(fragment, uniforms);
    let colors = mercury_colors(fragment, uniforms);
    let final_color = colors.blend_multiply(&craters);
    final_color
}

fn mercury_craters(fragment: &Fragment, uniforms: &Uniforms)-> Color{
    let zoom = 7.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise = (uniforms.noise.get_noise_2d(
        x*zoom,y*zoom
     )+1.0)/2.0;
    let noise = noise.max(0.0).min(1.0);

    let base_color = if noise<0.2{
        Color::new(255,255, 255)*(0.7-noise)
    } else if noise<0.25{
        Color::new(255,255, 255)*(1.0-noise)
    } else {
        Color::new(255,255, 255)
    };
    base_color
}

fn mercury_colors(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise_r = ((uniforms.noise.get_noise_2d(
        x*1.5,y*1.5
     )+1.0)/2.0).max(0.0).min(1.0);
     let noise_g = ((uniforms.noise.get_noise_2d(
        (x+10.0)*0.5,y*0.5
     )+1.0)/2.0).max(0.0).min(1.0);
     let noise_b = ((uniforms.noise.get_noise_2d(
        (x+20.0)*0.3,y*0.3
     )+1.0)/2.0).max(0.0).min(1.0);

    let red = Color::new(255, 0, 0)*noise_r;
    let green = Color::new(0, 255, 0)*noise_g;
    let blue = Color::new(0, 0, 255)*noise_b;
    let final_color = red.blend_add(&green).blend_add(&blue);
    final_color*(fragment.intensity.min(0.9))
}

// Venus 

pub fn venus_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let base_color = Color::from_hex(0xd9852b);
    let light_color = venus_lighter(fragment, uniforms);
    let darker_color = venus_darker(fragment, uniforms);
    let texture = venus_texture(fragment, uniforms);
    let final_color = base_color.blend_subtract(&darker_color).blend_add(&light_color).blend_multiply(&texture);

    final_color*(fragment.intensity.min(1.5).max(0.1))
}
fn venus_lighter(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 5.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise_small = (uniforms.noise.get_noise_2d(
        x*zoom,y*zoom
     )+1.0)/2.0;

     let noise_big = ((uniforms.noise.get_noise_2d(
        x*1.0,y*1.0
     )+1.0)/2.0);

     let noise = noise_big* 0.6+noise_small*0.4;

     let black = Color::black();
     let light_color = Color::from_hex(0xc28515);
     let final_color = if noise<0.5{
        light_color*(1.0-noise)
     } else{
        black
     };
     final_color
}

fn venus_darker(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 5.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise_small = (uniforms.noise.get_noise_2d(
        x*zoom,y*zoom
     )+1.0)/2.0;

     let noise_big = ((uniforms.noise.get_noise_2d(
        (x+40.0)*1.0,y*1.0
     )+1.0)/2.0);

     let noise = noise_big* 0.6+noise_small*0.4;
     let black = Color::black();
     let light_color = Color::from_hex(0xc28515);
     let final_color = if noise<0.5{
        light_color*(1.0-noise)
     } else{
        black
     };
     final_color
}

fn venus_texture(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 10.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise = (uniforms.noise.get_noise_2d(
        x*zoom,y*zoom
     )+1.0)/2.0;
     let noise_area = (uniforms.noise.get_noise_2d(
        x*2.0,y*2.0
     )+1.0)/2.0;
     let noise = noise_area.max(noise);
     let light_color = Color::from_hex(0xffffff);
     let final_color = if noise<0.4{
        light_color*(1.0-noise)
     } else{
        light_color
     };
     final_color
}

// Jupiter

pub fn jupiter_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{
    let stripes = jupiter_stripes(fragment, uniforms);
    let spots = jupiter_spot(fragment, stripes, uniforms);
    spots*(fragment.intensity.min(1.2).max(0.05))   
}

fn jupiter_spot(fragment: &Fragment,color: Color, uniforms: &Uniforms)-> Color{
    let x = fragment.position.x;
    let y = fragment.position.y;
    let distance = ((x-500.0)*(x-500.0) + (y-350.0)*(y-350.0)).sqrt();

    if distance < 20.0 {
        let t = uniforms.time as f32;
        let noise = (uniforms.noise.get_noise_2d(
            (x+t)*20.0,y*20.0)+1.0)/2.0;
        Color::from_hex(0xdb6f02)*(noise*0.5 +0.5)
    } else{
        color
    }
}
fn jupiter_stripes(fragment: &Fragment, uniforms: &Uniforms) ->Color {
    let color1 = light_stripes(fragment, uniforms);
    let color2 = other_stripes(fragment, uniforms);

    let stripe_width = 50.0;

    let stripe_factor = ((fragment.position.y/stripe_width)*PI).sin() * 0.5 + 0.5;
    
   let final_color = if stripe_factor<0.8{
        color1
    } else{
        color2
    };
    final_color
}

fn light_stripes(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 3.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let t = uniforms.time as f32 * 0.5;
    let noise = (uniforms.noise.get_noise_2d(
        (x+t)*zoom,y*10.0
     )+1.0)/2.0;
     let noise_area = (uniforms.noise.get_noise_2d(
        x+t,y
     )+1.0)/2.0;

     let noise = noise.min(noise_area*2.0);
    

     let light_color = Color::from_hex(0xffd896);
     let darker_color = Color::from_hex(0xffc86b);
     let final_color = if noise< 0.2{
        darker_color*(1.0-noise)
     } else{
        light_color*(0.5+noise).min(1.0)
     };
     
     final_color
}

fn other_stripes(fragment: &Fragment, uniforms: &Uniforms)->Color{
    let zoom = 3.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let t = uniforms.time as f32 * 0.5;
    let noise = (uniforms.noise.get_noise_2d(
        (x-t)*zoom +200.0,y*13.0+200.0
     )+1.0)/2.0;
     let noise_area = (uniforms.noise.get_noise_2d(
        x-t,y
     )+1.0)/2.0;

     let noise = noise.min(noise_area*2.0);
    

     let light_color = Color::from_hex(0xd9f6ff);
     let darker_color = Color::from_hex(0xabebff);
     let final_color = if noise< 0.2{
        darker_color*(1.0-noise)
     } else{
        light_color*(0.5+noise).min(1.0)
     };
     
     final_color
}

// Saturn

pub fn saturn_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{
    let saturn_lines =saturn_lines(fragment, uniforms);
    let ring_color = saturn_ring(fragment, uniforms, saturn_lines);
    let final_color = saturn_texture(fragment, uniforms, ring_color);
    final_color*(fragment.intensity.min(1.5).max(0.1))
}


fn saturn_ring(fragment: &Fragment, uniforms: &Uniforms, color:Color) -> Color{
    let color1 = Color::from_hex(0xff7e33);
    let min_y = 280.0;
    let max_y = 320.0;

    
   let final_color = if fragment.position.y < max_y && fragment.position.y > min_y{
        color1
    } else {
        color
    };
    final_color
}
fn saturn_lines(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let color1 = Color::from_hex(0xffd885);
    let color2 = Color::from_hex(0xff9238);

    let stripe_width = 5.0;

    let stripe_factor = ((fragment.position.y/stripe_width)*PI).sin() * 0.5 + 0.5;
    
   let final_color = if stripe_factor<0.8{
        color1
    } else{
        color2
    };
    final_color
}

fn saturn_texture(fragment: &Fragment, uniforms: &Uniforms, color: Color) -> Color {
    let zoom = 3.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise = (uniforms.noise.get_noise_2d(
        x*zoom,y*10.0
     )+1.0)/2.0;
    let final_color = if noise<0.6 {
        color
    } else{
        color*noise
    };
    final_color
}

//Neptune

pub fn neptune_shader(fragment: &Fragment, uniforms: &Uniforms)-> Color{
    let neptune_color = neptune_texture(fragment, uniforms);
    neptune_color*(fragment.intensity.min(2.0).max(0.05))
}

fn neptune_texture(fragment: &Fragment, uniforms: &Uniforms) -> Color{
    let zoom = 10.0;
    let x = fragment.position.x;
    let y = fragment.position.y;
    let noise = (uniforms.noise.get_noise_2d(
        x*zoom,y*zoom
     )+1.0)/2.0;
     let noise_area = (uniforms.noise.get_noise_2d(
        x*2.0,y*2.0
     )+1.0);
     let noise = noise_area.max(noise);
     let light_color = Color::from_hex(0x1350ba);
     let final_color = if noise<0.4{
        light_color*noise
     } else{
        light_color*noise
     };
     final_color
}
