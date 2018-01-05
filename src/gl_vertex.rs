/* Satisfy trait for glium. */

#[derive(Copy, Clone)]
pub struct GlVertex {
    pub position: [f32; 3]
}

implement_vertex!(GlVertex, position);

#[derive(Copy, Clone)]
pub struct GlNormal {
    pub normal: [f32; 3]
}

implement_vertex!(GlNormal, normal);