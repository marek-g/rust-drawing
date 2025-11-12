use anyhow::Result;
use drawing_api::ColorFormat;
use gl::types::*;

use crate::generic::device::Texture;

#[derive(Clone, Debug, PartialEq)]
pub struct GlTexture {
    pub(crate) id: GLuint,
    pub(crate) is_owned: bool,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) gl_format: GLuint,
    pub(crate) gl_type: GLuint,
    pub(crate) flipped_y: bool,
}

impl GlTexture {
    pub fn from_external(id: GLuint, width: u16, height: u16, format: ColorFormat) -> GlTexture {
        let (gl_type, gl_format) = match format {
            ColorFormat::RGBA => (gl::UNSIGNED_BYTE, gl::RGBA),
            ColorFormat::Y8 => (gl::UNSIGNED_BYTE, gl::RED),
        };
        GlTexture {
            id,
            is_owned: false,
            width,
            height,
            gl_format,
            gl_type,
            flipped_y: false,
        }
    }
}

impl Texture for GlTexture {
    fn update(
        &mut self,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                offset_x as GLint,
                offset_y as GLint,
                width as GLsizei,
                height as GLsizei,
                self.gl_format,
                self.gl_type,
                memory.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        if self.is_owned && self.id > 0 {
            unsafe {
                gl::DeleteTextures(1, &self.id);
            }
        }
    }
}
