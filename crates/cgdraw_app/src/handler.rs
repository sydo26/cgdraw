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
impl<'a> AppHandler<'a> {}
