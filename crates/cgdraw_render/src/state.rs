pub struct VertexBufferState {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub primitive_topology: wgpu::PrimitiveTopology,
}

pub struct BuffersState {
    /**
     * Os buffers dos vertices que serão renderizados
     */
    pub vertices: Vec<VertexBufferState>,
}

pub struct RenderState {
    /**
     * Os buffers que serão renderizados
     */
    pub buffers: BuffersState,
}

impl Default for RenderState {
    fn default() -> Self {
        let buffers = BuffersState {
            vertices: Vec::new(),
        };

        Self { buffers }
    }
}

impl RenderState {
    pub fn add_vertex_buffer_state(&mut self, vertex_buffer_state: VertexBufferState) {
        self.buffers.vertices.push(vertex_buffer_state);
    }
}
