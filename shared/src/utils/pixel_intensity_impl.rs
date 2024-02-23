use crate::types::pixel_intensity::PixelIntensity;

impl PixelIntensity {
    /// Creates a new `PixelIntensity` with the specified resolution and pixel data.
    ///
    /// # Arguments
    /// - `zn`: A `f32` representing the magnitude at the end of the iteration process.
    ///   This is typically used to determine the color or intensity of a pixel in fractal images.
    /// - `count`: A `f32` reflecting the number of iterations performed, normalized by the maximum number of iterations.
    ///   This value is often used to apply color gradients based on the iteration depth.
    /// A new `PixelIntensity` with the specified resolution and pixel data.
    pub fn new(zn: f32, count: f32) -> PixelIntensity {
        PixelIntensity { zn, count }
    }

    pub fn vec_data_to_pixel_intensity_matrix(vec_data: Vec<u8>) -> Vec<PixelIntensity> {
        let mut pixel_intensity_matrix = Vec::new();
        let mut i = 0;

        // Assurez-vous que i + la taille de zn + la taille de count n'est pas hors limite
        while i + 8 + 4 <= vec_data.len() {
            // Extraire zn
            let zn_bytes =
                <[u8; 4]>::try_from(&vec_data[i..i + 4]).expect("slice with incorrect length");
            let zn = f32::from_be_bytes(zn_bytes);
            i += 4;

            // Extraire count
            let count_bytes =
                <[u8; 4]>::try_from(&vec_data[i..i + 4]).expect("slice with incorrect length");
            let count = f32::from_be_bytes(count_bytes);
            i += 4;

            // Ajouter à la matrice
            pixel_intensity_matrix.push(PixelIntensity { zn, count });
        }

        pixel_intensity_matrix
    }
}

#[cfg(test)]
mod pixel_intensity_tests {
    use super::*;

    #[test]
    fn should_convert_vector_to_pixel_intensity() {
        let vec_data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = PixelIntensity::vec_data_to_pixel_intensity_matrix(vec_data);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].zn, 0.0);
        assert_eq!(result[0].count, 0.0);
    }

    #[test]
    fn should_handle_vector_with_incomplete_zn_bytes() {
        let vec_data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0];

        let result = PixelIntensity::vec_data_to_pixel_intensity_matrix(vec_data);
        assert_eq!(result.len(), 0);
    }
}
