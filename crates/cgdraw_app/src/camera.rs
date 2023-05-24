pub struct CameraInitialAttributes {
    // Posição inicial da câmera. (x, y, z);
    pub position: (f32, f32, f32),

    // Rotação vertical inicial da câmera em graus.
    pub v_rotation: f32,

    // Rotação horizontal inicial da câmera em graus.
    pub h_rotation: f32,
}
