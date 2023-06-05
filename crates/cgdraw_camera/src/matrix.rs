use cgmath::InnerSpace;

pub struct CameraMatrix4x4 {}

impl CameraMatrix4x4 {
    /// Calcula onde a câmera está posicionada e para onde ela está apontando.
    pub fn look_at_rh(
        eye: cgmath::Point3<f32>,
        direction: cgmath::Vector3<f32>,
        up: cgmath::Vector3<f32>,
    ) -> cgmath::Matrix4<f32> {
        // O vetor direção representa o eixo Z da câmera. Para sabermos qual eixo
        let dir = direction.normalize();

        // O vetor direito representa o eixo horizontal da câmera. Para sabermos
        // qual eixo é o horizontal, precisamos de um vetor indicando qual é o eixo
        // vertical da câmera. Nesse caso, o vetor up. Se o vetor up for o eixo Y,
        // o meu eixo horizontal será o eixo X.
        let right = up.cross(dir).normalize();

        // Aqui basicamente estamos capturando o vetor up da câmera. O vetor up
        // é o vetor que aponta para cima, ou seja, o eixo Y.
        let up = dir.cross(right);

        // Look-at right handed matrix
        //  rx   ry   rz  0
        //  ux   uy   uz  0
        //  dx   dy   dz  0
        //  0    0    0   1
        let look_at_matrix = cgmath::Matrix4::new(
            // Line 01
            right.x, up.x, dir.x, 0.0, //
            // Line 02
            right.y, up.y, dir.y, 0.0, //
            // Line 03
            right.z, up.z, dir.z, 0.0, //
            // Line 04
            0.0, 0.0, 0.0, 1.0,
        );

        // Translation matrix
        //  1  0  0  -x
        //  0  1  0  -y
        //  0  0  1  -z
        //  0  0  0   1
        let translation_matrix = cgmath::Matrix4::new(
            // Line 01
            1.0, 0.0, 0.0, 0.0, //
            // Line 02
            0.0, 1.0, 0.0, 0.0, //
            // Line 03
            0.0, 0.0, 1.0, 0.0, //
            // Line 04
            -eye.x, -eye.y, -eye.z, 1.0,
        );

        // Multiplicação da matriz de look-at com a matriz de translação
        look_at_matrix * translation_matrix
    }
}
