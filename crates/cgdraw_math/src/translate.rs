pub trait Translate<T> {
    /// Translada o objeto em relação aos globais.
    fn translate(&mut self, x: T, y: T, z: T);
}
