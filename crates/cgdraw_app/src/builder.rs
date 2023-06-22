use crate::App;

pub struct AppBuilder {}

impl Default for AppBuilder {
    /// Cria um novo construtor de aplicativos.
    #[inline]
    fn default() -> Self {
        Self {}
    }
}

impl AppBuilder {
    /// Constrói o aplicativo.
    #[inline]
    pub fn build(self) -> App {
        App {}
    }
}
