use std::path::PathBuf;

pub enum FractalConfigPath {
    Julia,
    IteratedSinZ,
    Mandelbrot,
    NewtonRaphsonZ3,
    NewtonRaphsonZ4,
}

impl FractalConfigPath {
    pub fn to_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push("config");

        match self {
            FractalConfigPath::Julia => path.push("julia.toml"),
            FractalConfigPath::IteratedSinZ => path.push("iterated_sinz.toml"),
            FractalConfigPath::Mandelbrot => path.push("mandelbrot.toml"),
            FractalConfigPath::NewtonRaphsonZ3 => path.push("newton_raphson_z3.toml"),
            FractalConfigPath::NewtonRaphsonZ4 => path.push("newton_raphson_z4.toml"),
        }

        path
    }

    pub fn new(arg: &str) -> Result<Self, String> {
        match PathBuf::from(arg).file_stem().and_then(|s| s.to_str()) {
            Some("julia") => Ok(FractalConfigPath::Julia),
            Some("iterated_sinz") => Ok(FractalConfigPath::IteratedSinZ),
            Some("mandelbrot") => Ok(FractalConfigPath::Mandelbrot),
            Some("newton_raphson_z3") => Ok(FractalConfigPath::NewtonRaphsonZ3),
            Some("newton_raphson_z4") => Ok(FractalConfigPath::NewtonRaphsonZ4),
            _ => Err("Invalid fractal type".to_string()),
        }
    }
}
