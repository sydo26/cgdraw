/// A primitiva que será renderizada pelo renderizador
pub enum Primitive {
    TriangleList,
    LineList,
    PointList,
}

impl Primitive {
    /// Converte a primitiva para o tipo de primitiva do wgpu
    pub fn to_wgpu_primitive(&self) -> wgpu::PrimitiveTopology {
        match self {
            Primitive::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            Primitive::LineList => wgpu::PrimitiveTopology::LineList,
            Primitive::PointList => wgpu::PrimitiveTopology::PointList,
        }
    }
}
