use cgdraw_core::color::Color;

/// Responsável por armazenar o estado global das configurações de renderização
pub struct GraphicsState {
    /// A cor que será usada para renderizar os gráficos
    pub color: Color,
}
