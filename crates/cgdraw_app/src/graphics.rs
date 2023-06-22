use cgdraw_core::{color::Color, graphic::Vertex};
use cgdraw_math::{
    angle::Rad,
    matrix::Matrix4x4,
    vector::{Vec3, Vec4},
};
use cgdraw_render::{RenderState, VertexBufferState};
use cgdraw_state::State;

pub struct GraphicState {
    pub color: [f32; 3],

    pub vertices: Vec<Vertex>,
}

pub struct Graphics<'a> {
    render_state: &'a mut RenderState,
    state: &'a State,
    pub graphic_state: GraphicState,

    transform_matrix: Matrix4x4<f32>,
}

impl<'a> Graphics<'a> {
    pub fn new(render_state: &'a mut RenderState, state: &'a State) -> Self {
        let graphic_state = GraphicState {
            color: Color::WHITE.to_vector(),
            vertices: Vec::new(),
        };

        Self {
            render_state,
            state,
            graphic_state,
            transform_matrix: Matrix4x4::identity(),
        }
    }
}

impl Graphics<'_> {
    pub fn save_vertice(&mut self, x: f32, y: f32, z: f32) {
        let vector = self.transform_matrix * Vec4::new(x, y, z, 1.0);

        let vertex = Vertex {
            position: [vector.x, vector.y, vector.z],
            color: self.graphic_state.color,
        };

        self.graphic_state.vertices.push(vertex);
    }

    pub fn color(&mut self, color: Color) {
        self.graphic_state.color = color.to_vector();
    }

    pub fn draw(&mut self, indices: Vec<u16>) {
        let vertex_buffer = Vertex::create_buffer(&self.state.device, &self.graphic_state.vertices);
        let index_buffer = Vertex::create_buffer_for_index(&self.state.device, indices.as_slice());

        let vertex_buffer_state = VertexBufferState {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
        };

        self.render_state
            .add_vertex_buffer_state(vertex_buffer_state);

        self.graphic_state.vertices.clear();
    }
}

impl Graphics<'_> {
    pub fn load_identity(&mut self) {
        self.transform_matrix = Matrix4x4::identity();
    }

    pub fn scale_x(&mut self, scale: f32) {
        let scale_matrix = Matrix4x4::from_scale(Vec3::new(scale, 1.0, 1.0));

        self.transform_matrix = self.transform_matrix * scale_matrix;
    }

    pub fn scale_y(&mut self, scale: f32) {
        let scale_matrix = Matrix4x4::from_scale(Vec3::new(1.0, scale, 1.0));

        self.transform_matrix = self.transform_matrix * scale_matrix;
    }

    pub fn scale_z(&mut self, scale: f32) {
        let scale_matrix = Matrix4x4::from_scale(Vec3::new(1.0, 1.0, scale));

        self.transform_matrix = self.transform_matrix * scale_matrix;
    }

    pub fn translate_x(&mut self, translate: f32) {
        let translate_matrix = Matrix4x4::from_translate(Vec3::new(translate, 0.0, 0.0));

        self.transform_matrix = self.transform_matrix * translate_matrix;
    }

    pub fn translate_y(&mut self, translate: f32) {
        let translate_matrix = Matrix4x4::from_translate(Vec3::new(0.0, translate, 0.0));

        self.transform_matrix = self.transform_matrix * translate_matrix;
    }

    pub fn translate_z(&mut self, translate: f32) {
        let translate_matrix = Matrix4x4::from_translate(Vec3::new(0.0, 0.0, translate));

        self.transform_matrix = self.transform_matrix * translate_matrix;
    }

    pub fn rotate_x(&mut self, angle: Rad<f32>) {
        let rotate_matrix = Matrix4x4::from_rotate_x(angle);

        self.transform_matrix = self.transform_matrix * rotate_matrix;
    }

    pub fn rotate_y(&mut self, angle: Rad<f32>) {
        let rotate_matrix = Matrix4x4::from_rotate_y(angle);

        self.transform_matrix = self.transform_matrix * rotate_matrix;
    }

    pub fn rotate_z(&mut self, angle: Rad<f32>) {
        let rotate_matrix = Matrix4x4::from_rotate_z(angle);

        self.transform_matrix = self.transform_matrix * rotate_matrix;
    }
}
