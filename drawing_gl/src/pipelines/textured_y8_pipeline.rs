use crate::utils::*;

use drawing_api::backend::TexturedY8Vertex;
use gl::types::*;
use std::ffi::CString;

pub struct TexturedY8Pipeline {
    program: Program,
    vbo: GLuint,
    vao: GLuint,
    transform_location: GLint,
    flipped_y_location: GLint,
}

impl TexturedY8Pipeline {
    pub fn new() -> Self {
        let vertex_shader =
            Shader::from_vert_str(include_str!("shaders/textured_y8.glslv")).unwrap();
        let pixel_shader =
            Shader::from_frag_str(include_str!("shaders/textured_y8.glslf")).unwrap();
        let program = Program::from_shaders(&[vertex_shader, pixel_shader]).unwrap();

        let transform_location = unsafe {
            let str = CString::new("transform").unwrap();
            gl::GetUniformLocation(program.id(), str.as_ptr())
        };
        let flipped_y_location = unsafe {
            let str = CString::new("flipped_y").unwrap();
            gl::GetUniformLocation(program.id(), str.as_ptr())
        };

        TexturedY8Pipeline {
            program,
            vbo: 0,
            vao: 0,
            transform_location,
            flipped_y_location,
        }
    }

    pub fn set_buffers(&mut self, buffers_vbo_vba: (GLuint, GLuint)) {
        self.vbo = buffers_vbo_vba.0;
        self.vao = buffers_vbo_vba.1;
    }

    pub fn apply(&mut self) {
        self.program.set_used();
    }

    pub fn set_transform(&mut self, transform: &[[f32; 4]; 4]) {
        unsafe {
            let ptr: *const f32 = std::mem::transmute(transform);
            gl::UniformMatrix4fv(self.transform_location, 1, gl::FALSE, ptr);
        }
    }

    pub fn set_flipped_y(&mut self, flipped_y: bool) {
        unsafe {
            gl::Uniform1i(self.flipped_y_location, if flipped_y { 1 } else { 0 });
        }
    }

    pub fn draw(&mut self, array: &[TexturedY8Vertex]) {
        self.apply_array(array);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, array.len() as GLint);
        }
    }

    fn apply_array(&mut self, array: &[TexturedY8Vertex]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                      // target
                (array.len() * std::mem::size_of::<TexturedY8Vertex>()) as GLsizeiptr, // size of data in bytes
                array.as_ptr() as *const GLvoid, // pointer to data
                gl::STREAM_DRAW,                 // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn create_vbo_and_vao(&self) -> (GLuint, GLuint) {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        TexturedY8Pipeline::specify_layout(self.program.id(), vbo, vao);

        (vbo, vao)
    }

    fn specify_layout(program_id: GLuint, vbo: GLuint, vao: GLuint) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let str = CString::new("in_position").unwrap();
            let pos_attr = gl::GetAttribLocation(program_id, str.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),
            ); // offset of the first component

            let str = CString::new("in_tex_coords").unwrap();
            let pos_attr = gl::GetAttribLocation(program_id, str.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (2 * std::mem::size_of::<f32>()) as *const GLvoid,
            ); // offset of the first component

            let str = CString::new("in_color").unwrap();
            let pos_attr = gl::GetAttribLocation(program_id, str.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                4,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (4 * std::mem::size_of::<f32>()) as *const GLvoid,
            ); // offset of the first component

            gl::BindVertexArray(0);
        }
    }
}
