use cgdraw_core::graphic::Vertex;
use cgdraw_render::VertexBufferState;

use super::Primitive;

/// Representa um passo de renderização gráfica que será executado pelo renderizador
pub struct GraphicRenderStepBuffer {
    /// O tipo de primitiva que será renderizada
    pub primitive: Option<Primitive>,

    /// Os indices que serão renderizados
    pub indices: Vec<u16>,

    /// Os vertices que serão renderizados
    pub vertices: Vec<Vertex>,
}

/// Inicializa com os valores padrões
impl Default for GraphicRenderStepBuffer {
    fn default() -> Self {
        Self {
            primitive: None,
            indices: Vec::new(),
            vertices: Vec::new(),
        }
    }
}

/// Implementa métodos para gerenciar o passo de renderização gráfica
impl GraphicRenderStepBuffer {
    /// Define o tipo de primitiva que será renderizada
    pub fn primitive(&mut self, primitive: Primitive) {
        self.primitive = Some(primitive);
    }

    /// Define os indices que serão renderizados
    pub fn indices(&mut self, indices: Vec<u16>) {
        self.indices = indices;
    }

    /// Define os vertices que serão renderizados
    pub fn vertices(&mut self, vertices: Vec<Vertex>) {
        self.vertices = vertices;
    }

    /// Inicia o passo de renderização gráfica
    pub fn init(&mut self, primitive: Option<Primitive>) {
        self.primitive = if let Some(p) = primitive {
            Some(p)
        } else {
            Some(Primitive::TriangleList)
        };

        self.indices.clear();
        self.vertices.clear();
    }

    /// Limpa os dados do passo de renderização gráfica e retorna o buffer necessário para renderização
    pub fn end(&mut self, device: &wgpu::Device) -> VertexBufferState {
        // Verifica se os indices foram definidos
        if self.indices.is_empty() {
            // Gera os indices na ordem que os vértices foram adicionados
            self.indices = (0..self.vertices.len() as u16).collect();
        }

        // Cria o buffer de vértices
        let vertex_buffer = Vertex::create_buffer(device, &self.vertices.to_vec());

        // Cria o buffer de indices
        let index_buffer = Vertex::create_buffer_for_index(device, self.indices.as_slice());

        // Converte a primitiva para o tipo de primitiva do wgpu
        let primitive_topology = self.primitive.as_ref().unwrap().to_wgpu_primitive();

        // Cria o estado do buffer de vértices
        let vertex_buffer_state = VertexBufferState {
            vertex_buffer,
            index_buffer,
            num_elements: self.indices.len() as u32,
            primitive_topology,
        };

        // Limpa os dados do passo de renderização gráfica

        self.primitive = None;
        self.indices.clear();
        self.vertices.clear();

        vertex_buffer_state
    }
}
