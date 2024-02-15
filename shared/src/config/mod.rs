use std::{fs, io};

use log::info;

use crate::{types::{fractal_config_path::FractalConfigPath, fractal_descriptor::FractalDescriptor}, utils::filesystem::project_root};

pub fn save_configuration(config: &FractalDescriptor, file_path: &str) -> io::Result<()> {
    let serialized = toml::to_string(config).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(file_path, serialized)?;
    Ok(())
}

pub fn load_fractal_configuration(config_path: FractalConfigPath) -> Result<FractalDescriptor, io::Error> {
    let file_path = project_root()?.join(config_path.to_path());
    info!("Loading fractal configuration from {}", file_path.display());
    let contents = fs::read_to_string(file_path)?;
    let fractal_descriptor = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(fractal_descriptor)
}

#[cfg(test)]
mod config_test {
    use std::path::PathBuf;

    use crate::{config::load_fractal_configuration, types::{complex::Complex, fractal_config_path::FractalConfigPath, fractal_descriptor::{FractalDescriptor, JuliaDescriptor, NewtonRaphsonZ4Descriptor}}};
    use crate::types::fractal_descriptor::FractalType::{Julia, NewtonRaphsonZ4};

    #[test]
    fn test_load_valid_fractal_configuration() -> Result<(), String> {
        let config_path: FractalConfigPath = FractalConfigPath::Julia;
        assert_eq!(config_path.to_path(), PathBuf::from("config/julia.toml"));

        let loaded_descriptor = load_fractal_configuration(config_path)
            .map_err(|e| e.to_string())?;

        let expected_descriptor = FractalDescriptor {
            fractal_type: Julia(JuliaDescriptor {
                c: Complex { re: 0.285, im: 0.013 },
                divergence_threshold_square: 4.0,
            }),
        };

        assert_eq!(loaded_descriptor, expected_descriptor);

        Ok(())
    }

    #[test]
    fn test_load_valid_fractal_configuration_with_empty_values() {
        let config_path: FractalConfigPath = FractalConfigPath::NewtonRaphsonZ4;
        assert_eq!(config_path.to_path(), PathBuf::from("config/newton_raphson_z4.toml"));

        let loaded_descriptor = load_fractal_configuration(config_path)
            .expect("Failed to load fractal configuration");

        let expected_descriptor = FractalDescriptor {
            fractal_type: NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor { }),
        };

        assert_eq!(loaded_descriptor, expected_descriptor);
    }

    #[test]
    fn test_return_error_invalid_file_path() {
        let config_path = FractalConfigPath::new("config/invalid_file_path.toml");
        assert!(config_path.is_err());
    }

}