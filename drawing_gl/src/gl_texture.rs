use std::sync::Arc;

use drawing_api::ColorFormat;
use drawing_api::Texture;
use drawing_api::TextureDescriptor;
use gl::types::*;

use crate::units::PixelToUvTransform;

#[derive(Debug)]
pub(crate) struct GlTextureData {
    pub id: GLuint,
    pub is_owned: bool,
    pub width: u16,
    pub height: u16,
    pub gl_format: GLuint,
    pub gl_type: GLuint,
    pub flipped_y: bool,
}

impl Drop for GlTextureData {
    fn drop(&mut self) {
        if self.is_owned && self.id > 0 {
            unsafe {
                gl::DeleteTextures(1, &self.id);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlTexture {
    pub(crate) data: Arc<GlTextureData>,
}

impl GlTexture {
    pub fn from_external(id: GLuint, width: u16, height: u16, format: ColorFormat) -> GlTexture {
        let (gl_type, gl_format) = match format {
            ColorFormat::RGBA => (gl::UNSIGNED_BYTE, gl::RGBA),
            ColorFormat::Y8 => (gl::UNSIGNED_BYTE, gl::RED),
        };
        GlTexture {
            data: Arc::new(GlTextureData {
                id,
                is_owned: false,
                width,
                height,
                gl_format,
                gl_type,
                flipped_y: false,
            }),
        }
    }
}

impl Texture for GlTexture {
    /*fn update(
        &mut self,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        let gl_format = self.data.lock().unwrap().gl_format;
        let gl_type = self.data.lock().unwrap().gl_type;
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                offset_x as GLint,
                offset_y as GLint,
                width as GLsizei,
                height as GLsizei,
                gl_format,
                gl_type,
                memory.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }*/

    fn get_descriptor(&self) -> drawing_api::TextureDescriptor {
        TextureDescriptor {
            width: self.data.width as u32,
            height: self.data.height as u32,
            color_format: if self.data.gl_format == gl::RGBA {
                ColorFormat::RGBA
            } else {
                ColorFormat::Y8
            },
            mip_count: 0,
        }
    }

    fn get_gl_handle(&self) -> usize {
        self.data.id as usize
    }
}
