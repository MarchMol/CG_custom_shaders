use crate::fragments::{triangle_fill, Fragment};
use crate::shader::{earth_shader, jupiter_shader, mercury_shader, neptune_shader, saturn_shader, sun_shader, venus_shader};
use crate::vertex::Vertex;
use crate::{screen::framebuffer::Framebuffer, shader::{vertex_shader}};
use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Mat4, Vec3};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub light_dir: Vec3,
    pub time: u32,
    pub noise: FastNoiseLite,
    pub planet: u8
}
pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // 1. Vertex shader stage
    let mut shaded_vertices: Vec<Vertex> = Vec::new();
    for vertex in vertex_array {
        shaded_vertices.push(vertex_shader(vertex, uniforms))
    }

    // 2. Primitive Assembly stage (only triangles)
    let len = shaded_vertices.len();
    let mut triangles= Vec::new();
    
    for i in (0..shaded_vertices.len()).step_by(3) {
        if i + 2 < shaded_vertices.len() {
            triangles.push([
                shaded_vertices[i].clone(),
                shaded_vertices[i + 1].clone(),
                shaded_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage

    let mut fragments: Vec<Fragment> = Vec::new();
    for tri in triangles {
        fragments.extend(triangle_fill(&tri[0], &tri[1], &tri[2], uniforms));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        let shaded_color = match uniforms.planet {
            1=> sun_shader(&fragment, uniforms),
            2=> mercury_shader(&fragment, uniforms),
            3=> venus_shader(&fragment, uniforms),
            4=> earth_shader(&fragment, uniforms),
            5=> jupiter_shader(&fragment, uniforms),
            6=> saturn_shader(&fragment, uniforms),
            7=> neptune_shader(&fragment, uniforms),
            _=> sun_shader(&fragment, uniforms)
        };
        let color = shaded_color.to_hex();
        framebuffer.set_current_color(color);
        framebuffer.point(x, y, fragment.depth);
    }
}