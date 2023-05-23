use cgdraw_core::{color::Color, graphic::Vertex};
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
        }
    }
}

impl Graphics<'_> {
    pub fn save_vertice(&mut self, x: f32, y: f32, z: f32) {
        let vertex = Vertex {
            position: [x, y, z],
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
