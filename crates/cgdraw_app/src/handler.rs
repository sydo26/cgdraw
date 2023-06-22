use cgdraw_state::State;

pub struct AppHandler<'a> {
    state: &'a mut State,
}

impl<'a> AppHandler<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }
}

// Camera Implementations
impl<'a> AppHandler<'a> {
    pub fn camera_view_proj(&mut self, view_proj: [[f32; 4]; 4]) {
        self.state.camera_uniform.view_proj = view_proj;
    }
}
