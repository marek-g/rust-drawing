use gl::types::*;

/// Represents data that needs to be created separately for each context,
/// for example: Vertex Array Objects, Vertex Buffer Objects.
pub struct GlContextData {
    pub(crate) colored_pipeline_buffers: (GLuint, GLuint),
    pub(crate) textured_pipeline_buffers: (GLuint, GLuint),
    pub(crate) textured_y8_pipeline_buffers: (GLuint, GLuint),
    pub(crate) universal_pipeline_buffers: (GLuint, GLuint),
}

impl Drop for GlContextData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.colored_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.colored_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.textured_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.textured_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.textured_y8_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.textured_y8_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.universal_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.universal_pipeline_buffers.0);
        }
    }
}
