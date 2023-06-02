use cgdraw_math::translate::Translate;

use crate::Camera;

impl Translate<f32> for Camera {
    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position += cgmath::Vector3::new(x, y, z);
        self.target += cgmath::Vector3::new(x, y, z);
    }

    // /// Translada a posição da câmera em relação ao eixo global.
    // pub fn translate_global(&mut self, x: f32, y: f32, z: f32, move_target: bool) {
    //     if move_target {
    // self.position += cgmath::Vector3::new(x, y, z);
    // self.target += cgmath::Vector3::new(x, y, z);
    //     } else {
    //         let dir = (self.target - self.position).normalize();
    //         self.position += cgmath::Vector3::new(-x, -y, z);
    //         self.target += dir.cross(cgmath::Vector3::unit_y());
    //     }
    // }

    // /// Translada a posição da câmera em relação ao eixo vertical.
    // pub fn translate_vertical(&mut self, distance: f32) {
    //     let direction_vector = (self.target - self.position).normalize();

    //     self.position += direction_vector * distance;
    //     self.target += direction_vector * distance;
    // }

    // /// Translada a posição da câmera em relação ao eixo horizontal.
    // pub fn translate_horizontal(&mut self, distance: f32) {
    //     let direction_vector = (self.target - self.position).normalize();

    //     let side_vector = direction_vector
    //         .cross(cgmath::Vector3::unit_y())
    //         .normalize();

    //     self.position += side_vector * distance;
    //     self.target += side_vector * distance;
    // }
}
