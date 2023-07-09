mod primitive;
mod state;
mod step;
mod transform;

pub use primitive::*;
pub use state::*;
pub use step::*;
pub use transform::*;

use cgdraw_core::{color::Color, graphic::Vertex};
use cgdraw_math::vector::Vec4;
use cgdraw_render::RenderState;
use cgdraw_state::State;

/// Responsável por ser usado como interface para a renderização de gráficos na tela.
pub struct Graphics<'a> {
    /// A estrutura que armazena os buffers de renderização
    render_state: &'a mut RenderState,

    /// O estado da aplicação que armazena os recursos do wgpu
    state: &'a mut State,

    /// A estrutura que armazena o próximo passo de renderização que
    /// irá ser adicionado aos buffers de renderização
    buffer_step: Option<GraphicRenderStepBuffer>,

    /// O estado gráfico que armazena as configurações de renderização
    graphics_state: GraphicsState,

    /// Objeto que possui funções de transformações que você pode aplicar nos vértices antes
    /// de renderizá-los.
    pub t: GraphicsTransform,
}

/// Inicializa a instância da estrutura `Graphics`
impl<'a> Graphics<'a> {
    /// Cria uma nova instância da estrutura `Graphics`
    pub fn new(render_state: &'a mut RenderState, state: &'a mut State) -> Self {
        let graphics_state = GraphicsState {
            color: Color::WHITE,
        };

        Self {
            state,
            render_state,
            // Começa como `None` porque o primeiro passo de renderização tem que ser adicionado
            // pelo método `begin`
            buffer_step: None,
            graphics_state,

            t: GraphicsTransform::default(),
        }
    }
}

/// Métodos de desenho
impl Graphics<'_> {
    /// Adiciona um novo vértice 3D ao passo de renderização atual.
    pub fn v3d(&mut self, x: f32, y: f32, z: f32) {
        let v4 = Vec4::new(x, y, z, 1.0);

        let v3 = self.t.matrix * v4;
        let vertex = Vertex {
            position: [v3.x, v3.y, v3.z],
            color: self.graphics_state.color.to_vector(),
        };

        // Verifica se o passo de renderização atual já foi iniciado
        if let Some(step) = self.buffer_step.as_mut() {
            // Adiciona o vértice ao passo de renderização atual
            step.vertices.push(vertex);
        } else {
            panic!("O passo de renderização atual não foi iniciado! Use o método `begin` para iniciar o passo de renderização atual!");
        }
    }

    /// Define a lista de indices que serão renderizados. Caso não seja repassado,
    /// a lista será gerada na ordem que os vetores foram adicionados.
    pub fn indices(&mut self, indices: Vec<u16>) {
        // Verifica se o passo de renderização atual já foi iniciado
        if let Some(step) = self.buffer_step.as_mut() {
            // Adiciona o vértice ao passo de renderização atual
            step.indices(indices);
        } else {
            panic!("O passo de renderização atual não foi iniciado! Use o método `begin` para iniciar o passo de renderização atual!");
        }
    }
}

/// Métodos de configuração de renderização
impl Graphics<'_> {
    /// Define a cor que será usada para renderizar os gráficos
    pub fn color(&mut self, color: Color) {
        self.graphics_state.color = color;
    }

    /// Inicia o passo de renderização gráfica.
    pub fn begin(&mut self, primitive: Primitive) {
        // Verifica se o passo de renderização atual já foi iniciado
        if self.buffer_step.is_some() {
            panic!("O passo de renderização atual já foi iniciado! Use o método `end` para finalizar o passo de renderização atual!")
        }

        // Inicia o passo de renderização gráfica
        self.buffer_step = Some(GraphicRenderStepBuffer::default());

        // Define o tipo de primitiva que será renderizada
        self.buffer_step.as_mut().unwrap().init(Some(primitive));
    }

    /// Finaliza o passo de renderização gráfica
    pub fn end(&mut self) {
        // Verifica se o passo de renderização atual já foi iniciado
        if self.buffer_step.is_none() {
            panic!("O passo de renderização atual não foi iniciado! Use o método `begin` para iniciar o passo de renderização atual!")
        }

        // Finaliza o passo de renderização gráfica
        let mut step = self.buffer_step.take().unwrap();

        // Cria o objeto de buffers que será usado para renderizar os gráficos
        let vertex_buffer_state = step.end(&self.state.device);

        // Adiciona o estado do buffer de vértices ao estado de renderização
        self.render_state
            .add_vertex_buffer_state(vertex_buffer_state);

        self.buffer_step = None;
    }
}
