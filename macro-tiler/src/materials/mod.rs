use std::str::from_utf8;
use macroquad::prelude::{Material, MaterialParams, load_material};
use naga::valid::Capabilities;
use macroquad::models::draw_cube;

#[derive(Debug, thiserror::Error)]
pub enum CreateShaderModuleError {
    #[error("Source code it not utf-8")]
    SourceNotUtf8 {
        #[from]
        source: std::str::Utf8Error,
    },

    #[error("Failed to parse WGSL shader")]
    NagaWgslParseError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Failed to validate shader module")]
    NagaValidationError {
        #[from]
        source: naga::valid::ValidationError,
    },

    #[error("Error during translation to glsl")]
    NagaGlslBackendError {
        #[from]
        source: naga::back::glsl::Error
    },

    #[error("Miniquad shader compilation failed")]
    MiniquadShaderCompilationError {
        #[from]
        source: macroquad::miniquad::graphics::ShaderError
    },
}

pub enum ShaderStage { Vertex, Fragment }

pub fn load_wgsl_material(
    vertex_shader: &str,
    fragment_shader: &str,
    params: MaterialParams
) -> Result<Material, CreateShaderModuleError> {
    let vertex_module_src = load_shader_and_translate(
        vertex_shader,
        ShaderStage::Vertex,
        "vert".to_string()
    )?;
    let fragment_module_src = load_shader_and_translate(
        fragment_shader,
        ShaderStage::Fragment,
        "frag".to_string()
    )?;
    Ok(load_material(&vertex_module_src, &fragment_module_src, params)?)
}

pub fn load_wgsl_material_from_bytes(
    vertex_shader: &[u8],
    fragment_shader: &[u8],
    params: MaterialParams
) -> Result<Material, CreateShaderModuleError> {
    let vertex_module_src = load_raw_shader_bytes_and_translate(
        vertex_shader,
        ShaderStage::Vertex,
        "main".to_string()
    )?;
    let fragment_module_src = load_raw_shader_bytes_and_translate(
        fragment_shader,
        ShaderStage::Fragment,
        "main".to_string()
    )?;
    Ok(load_material(&vertex_module_src, &fragment_module_src, params)?)
}

fn load_raw_shader_bytes_and_translate(
    bytes: &[u8],
    shader_stage: ShaderStage,
    entry_point: String
) -> Result<String, CreateShaderModuleError> {
    let code = from_utf8(bytes)?;
    load_shader_and_translate(code, shader_stage, entry_point)
}

fn load_shader_and_translate(
    code: &str,
    shader_stage: ShaderStage,
    entry_point: String
) -> Result<String, CreateShaderModuleError> {
    let module = naga::front::wgsl::parse_str(code).map_err(|err| {
        CreateShaderModuleError::NagaWgslParseError {
            source: Box::from(err.emit_to_string(".")),
        }
    })?;
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        Capabilities::all(),
    ).validate(&module)?;

    let mut translated = String::new();
    let opts = naga::back::glsl::Options {
        version: naga::back::glsl::Version::Desktop(330),
        ..Default::default()
    };
    let pipeline_opts = naga::back::glsl::PipelineOptions {
        shader_stage: match shader_stage {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Fragment => naga::ShaderStage::Fragment,
        },
        entry_point
    };

    let mut translator_writer = naga::back::glsl::Writer::new(
        &mut translated,
        &module,
        &info,
        &opts,
        &pipeline_opts
    )?;

    let _reflection_info = translator_writer.write()?;

    Ok(translated)
}

#[cfg(test)]
mod tests {
    use crate::materials::{load_shader_and_translate, ShaderStage, CreateShaderModuleError};

    #[test]
    fn test_vertex() {
        let source = include_str!("vertex_shader_test.wgsl");
        let translated = load_shader_and_translate(source, ShaderStage::Vertex, "main".to_string());
        match translated {
            Ok(code) => {
                println!("{}", code);
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    #[test]
    fn test_fragment() {
        let source = include_str!("fragment_shader_test.wgsl");
        let translated = load_shader_and_translate(source, ShaderStage::Fragment, "main".to_string());
        match translated {
            Ok(code) => {
                println!("{}", code);
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}