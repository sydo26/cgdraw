pub struct CameraInitialAttributes {
    /// Posição inicial da câmera. (x, y, z);
    pub position: (f32, f32, f32),

    /// Ponto para onde a câmera deve apontar inicialmente. (x, y, z);
    pub target: Option<(f32, f32, f32)>,

    /// Rotação vertical aplicada inicialmente após a câmera estar apontando para o ponto target informado.
    pub v_rotation: Option<f32>,

    /// Rotação horizontal aplicada inicialmente após a câmera estar apontando para o ponto target informado.
    pub h_rotation: Option<f32>,
}
