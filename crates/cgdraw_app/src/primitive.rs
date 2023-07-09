pub enum Primitive {
    TriangleList,
    LineList,
    PointList,
}

impl Primitive {
    pub fn to_wgpu_primitive(&self) -> wgpu::PrimitiveTopology {
        match self {
            Primitive::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            Primitive::LineList => wgpu::PrimitiveTopology::LineList,
            Primitive::PointList => wgpu::PrimitiveTopology::PointList,
        }
    }
}
