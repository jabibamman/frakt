use crate::types::{
    color::{HSL, RGB},
    pixel_intensity::PixelIntensity,
};

///Generates a color based on the provided pixel intensity.
/// # Arguments
/// * `pixel_intensity`: A `PixelIntensity` containing the number of iterations and the norm of the complex point.
///
/// # Returns
/// Returns an array containing the RGB values of the color.
///
pub fn color(pixel_intensity: PixelIntensity) -> [u8; 3] {
    if pixel_intensity.count == 1.0 {
        return [0, 0, 0];
    }

    let hsl = HSL {
        h: pixel_intensity.count * 360.0,
        s: 0.5 + 0.5 * (pixel_intensity.zn * 3.14).cos(),
        l: 0.5,
    };

    let color = hsl_to_rgb(hsl);

    [color.r, color.g, color.b]
}

/// Convert a color from HSL to RGB
/// # Arguments
/// * `hsl`: A `HSL` containing the HSL values of the color (Hue, Saturation, Lightness)
///
/// # Returns
/// Returns a tuple containing the RGB values of the color
///
/// # Details
/// This function is based on the algorithm found at https://www.rapidtables.com/convert/color/hsl-to-rgb.html
///
pub fn hsl_to_rgb(hsl: HSL) -> RGB {
    let c = (1.0 - (2.0 * hsl.l - 1.0).abs()) * hsl.s;
    let h_prime = hsl.h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = hsl.l - c / 2.0;

    let (r_temp, g_temp, b_temp) = match h_prime.floor() as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    RGB {
        r: ((r_temp + m) * 255.0) as u8,
        g: ((g_temp + m) * 255.0) as u8,
        b: ((b_temp + m) * 255.0) as u8,
    }
}

#[test]
fn test_color() {
    let pixel_intensity = PixelIntensity {
        zn: 0.5,
        count: 0.5,
    };

    let result = color(pixel_intensity);

    let test0 = crate::utils::type_of::type_of(result[0]);
    let test1 = crate::utils::type_of::type_of(result[1]);
    let test2 = crate::utils::type_of::type_of(result[2]);

    assert!(test0.eq("u8"));
    assert!(test1.eq("u8"));
    assert!(test2.eq("u8"));

    assert_eq!(result, [63, 191, 191]);
}
