use cgdraw_state::State;

pub struct AppHandler<'a> {
    state: &'a mut State,
}

impl<'a> AppHandler<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }
}

/**
 * Camera implementations
 */
impl<'a> AppHandler<'a> {
    /// Rotaciona a vertical da câmera em graus.
    pub fn rotate_camera_v(&mut self, deg: f32) {
        println!("Rotating camera vertical by {} degrees", deg);
        self.state.camera.vertical_rotate(cgmath::Deg(deg));
    }

    /// Rotaciona o horizontal da câmera em graus.
    pub fn rotate_camera_h(&mut self, deg: f32) {
        println!("Rotating camera horizontal by {} degrees", deg);
        self.state.camera.horizontal_rotate(cgmath::Deg(deg));
    }

    /// Pega a rotação atual da câmera
    pub fn get_rotation(&self) -> (f32, f32) {
        let (yaw, pitch) = self.state.camera.get_rotation();
        (
            <cgmath::Deg<f32>>::from(yaw).0,
            <cgmath::Deg<f32>>::from(pitch).0,
        )
    }

    /// Pega a rotação vertical atual da câmera
    pub fn get_vertical_rotation(&self) -> f32 {
        let (yaw, _) = self.get_rotation();
        yaw
    }

    /// Pega a rotação horizontal atual da câmera
    pub fn get_horizontal_rotation(&self) -> f32 {
        let (_, pitch) = self.get_rotation();
        pitch
    }
}
