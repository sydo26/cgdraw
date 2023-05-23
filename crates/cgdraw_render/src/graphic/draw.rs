use crate::VertexBufferState;

pub trait Draw<'a> {
    /**
     * Desenha os vértices na tela.
     */
    fn draw_vertices(&mut self, buffer_state: &'a VertexBufferState);
}

impl<'a, 'b> Draw<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_vertices(&mut self, buffer_state: &'a VertexBufferState) {
        self.set_vertex_buffer(0, buffer_state.vertex_buffer.slice(..));
        self.set_index_buffer(
            buffer_state.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        self.draw_indexed(0..buffer_state.num_elements, 0, 0..1);
    }
}
