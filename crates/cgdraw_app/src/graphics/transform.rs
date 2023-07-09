use cgdraw_math::{angle::Rad, matrix::Matrix4x4, vector::Vec3};

pub struct GraphicsTransform {
    pub matrix: Matrix4x4<f32>,
}

impl Default for GraphicsTransform {
    fn default() -> Self {
        Self {
            matrix: Matrix4x4::identity(),
        }
    }
}

impl GraphicsTransform {
    /// Realiza a transformação de translação na matriz atual.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix = self.matrix * Matrix4x4::from_translate(Vec3::new(x, y, z));
    }

    /// Realiza a transformação de rotação na matriz atual.
    pub fn rotate(&mut self, angle_x: Rad<f32>, angle_y: Rad<f32>, angle_z: Rad<f32>) {
        self.matrix = self.matrix * Matrix4x4::from_rotate(angle_x, angle_y, angle_z);
    }

    /// Realiza a transformação de rotação em X na matriz atual.
    pub fn rotate_x(&mut self, angle: Rad<f32>) {
        self.matrix = self.matrix * Matrix4x4::from_rotate_x(angle);
    }

    /// Realiza a transformação de rotação em Y na matriz atual.
    pub fn rotate_y(&mut self, angle: Rad<f32>) {
        self.matrix = self.matrix * Matrix4x4::from_rotate_y(angle);
    }

    /// Realiza a transformação de escala na matriz atual.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix = self.matrix * Matrix4x4::from_scale(Vec3::new(x, y, z));
    }

    /// Realiza a transformação de escala em X na matriz atual.
    // pub fn scale_x(&mut self, x: f32) {
    //     self.matrix = self.matrix * Matrix4x4::from_scale_x(x);
    // }

    // /// Realiza a transformação de escala em Y na matriz atual.
    // pub fn scale_y(&mut self, y: f32) {
    //     self.matrix = self.matrix * Matrix4x4::from_scale_y(y);
    // }

    // /// Realiza a transformação de escala em Z na matriz atual.
    // pub fn scale_z(&mut self, z: f32) {
    //     self.matrix = self.matrix * Matrix4x4::from_scale_z(z);
    // }

    /// Carrega a matriz identidade na matriz atual.
    pub fn load_identity(&mut self) {
        self.matrix = Matrix4x4::identity();
    }

    /// Carrega uma matriz na matriz atual.
    pub fn load(&mut self, matrix: Matrix4x4<f32>) {
        self.matrix = matrix;
    }
}
