impl PixelData {
    /// Creates a new `PixelData` with the specified resolution and pixel data.
    ///
    /// # Arguments
    ///
    /// * `resolution` - A `Resolution` representing the resolution of the pixel data.
    /// * `data` - A `Vec<u8>` representing the pixel data.
    ///
    /// # Returns
    ///
    /// A new `PixelData` with the specified resolution and pixel data.
    pub fn new(resolution: Resolution, data: Vec<u8>) -> PixelData {
        PixelData { resolution, data }
    }
}
