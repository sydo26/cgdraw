pub trait Draw<'a> {
    /**
     * Desenha os vértices na tela.
     */
    fn draw_vertices(
        &mut self,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        num_elements: u32,
    );
}

impl<'a, 'b> Draw<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_vertices(
        &mut self,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        num_elements: u32,
    ) {
        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..num_elements, 0, 0..1);
    }
}
