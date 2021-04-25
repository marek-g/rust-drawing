use gl::types::*;
use anyhow::Result;

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

impl drawing::backend::Texture for GlTexture {
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
