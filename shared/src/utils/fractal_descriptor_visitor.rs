use core::fmt;

use serde::{de::{self, MapAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use crate::types::fractal_descriptor::{FractalDescriptor, FractalType};

struct FractalDescriptorVisitor;

impl<'de> Visitor<'de> for FractalDescriptorVisitor {
    type Value = FractalDescriptor;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct FractalDescriptor")
    }

    fn visit_map<V>(self, mut map: V) -> Result<FractalDescriptor, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut fractal_type = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "Julia" => {
                    if fractal_type.is_some() {
                        return Err(de::Error::duplicate_field("Julia"));
                    }
                    fractal_type = Some(FractalType::Julia(map.next_value()?));
                },
                "IteratedSinZ" => {
                    if fractal_type.is_some() {
                        return Err(de::Error::duplicate_field("IteratedSinZ"));
                    }
                    fractal_type = Some(FractalType::IteratedSinZ(map.next_value()?));
                },
                "NewtonRaphsonZ3" => {
                    if fractal_type.is_some() {
                        return Err(de::Error::duplicate_field("NewtonRaphsonZ3"));
                    }
                    fractal_type = Some(FractalType::NewtonRaphsonZ3(map.next_value()?));
                },
                "NewtonRaphsonZ4" => {
                    if fractal_type.is_some() {
                        return Err(de::Error::duplicate_field("NewtonRaphsonZ4"));
                    }
                    fractal_type = Some(FractalType::NewtonRaphsonZ4(map.next_value()?));
                },
                _ => return Err(de::Error::unknown_field(&key, &["Julia", "IteratedSinZ", "NewtonRaphson"])),
            }
        }
        let fractal_type = fractal_type.ok_or_else(|| de::Error::missing_field("FractalType"))?;
        Ok(FractalDescriptor { fractal_type })
    }
}

/// A struct representing a Fractal Descriptor without the fractal type.
impl<'de> Deserialize<'de> for FractalDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(FractalDescriptorVisitor)
    }
}

/// A struct representing a Fractal Descriptor.
/// 
/// # Details
/// 
/// This struct is used to describe the type of fractal to generate.
impl Serialize for FractalDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FractalDescriptor", 1)?;
        match self.fractal_type {
            FractalType::Julia(ref descriptor) => {
                state.serialize_field("Julia", descriptor)?;
            },
            FractalType::IteratedSinZ(ref descriptor) => {
                state.serialize_field("IteratedSinZ", descriptor)?;
            },
            FractalType::NewtonRaphsonZ3(ref descriptor) => {
                state.serialize_field("NewtonRaphsonZ3", descriptor)?;
            },
            FractalType::NewtonRaphsonZ4(ref descriptor) => {
                state.serialize_field("NewtonRaphsonZ4", descriptor)?;
            },
            FractalType::Mandelbrot(ref descriptor) => {
                state.serialize_field("Mandelbrot", descriptor)?;
            },
        }
        state.end()
    }
}