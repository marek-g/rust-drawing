extern crate gl;
extern crate std;

use ::utils::*;

use self::gl::types::*;
use std::ffi::CString;

#[repr(C, packed)]
pub struct TexturedY8Vertex {
    pub pos: [f32; 2], // "in_position"
    pub tex_coords: [f32; 2], // "in_tex_coords"
    pub color: [f32; 4], // "in_color"
}

#[repr(C, packed)]
pub struct TexturedY8Locals {
    pub transform: [[f32; 4]; 4], // "transform"
}

pub struct TexturedY8Pipeline {
    program: Program,
    vbo: GLuint,
    vao: GLuint,
    transform_location: GLint,
}

impl TexturedY8Pipeline {
    pub fn new() -> Self {
        let vertex_shader = Shader::from_vert_str(include_str!("shaders/textured_y8.glslv")).unwrap();
		let pixel_shader = Shader::from_frag_str(include_str!("shaders/textured_y8.glslf")).unwrap();
        let program = Program::from_shaders(&[vertex_shader, pixel_shader]).unwrap();

        let (vbo, vao) = TexturedY8Pipeline::create_vbo_and_vao();
        TexturedY8Pipeline::specify_layout(program.id(), vbo, vao);

        let transform_location = unsafe {
            gl::GetUniformLocation(program.id(), CString::new("transform").unwrap().as_ptr())
        };

        TexturedY8Pipeline {
            program, vbo, vao, transform_location,
        }
    }

    pub fn apply(&mut self) {
        self.program.set_used();
    }

    pub fn draw(&mut self, array: &[TexturedY8Vertex], locals: &TexturedY8Locals) {
        self.apply_array(array);
        self.apply_locals(locals);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, array.len() as GLint);
        }
    }

    fn apply_array(&mut self, array: &[TexturedY8Vertex]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (array.len() * std::mem::size_of::<TexturedY8Vertex>()) as GLsizeiptr, // size of data in bytes
                array.as_ptr() as *const GLvoid, // pointer to data
                gl::STREAM_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn apply_locals(&mut self, locals: &TexturedY8Locals) {
        unsafe {
            let ptr: *const f32 = std::mem::transmute(&locals.transform);
            gl::UniformMatrix4fv(self.transform_location, 1, gl::FALSE, ptr);
        }
    }

    fn create_vbo_and_vao() -> (GLuint, GLuint) {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        (vbo, vao)
    }

    fn specify_layout(program_id: GLuint, vbo: GLuint, vao: GLuint) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let pos_attr = gl::GetAttribLocation(program_id, CString::new("in_position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null()); // offset of the first component

            let pos_attr = gl::GetAttribLocation(program_id, CString::new("in_tex_coords").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (2 * std::mem::size_of::<f32>()) as *const GLvoid); // offset of the first component

            let pos_attr = gl::GetAttribLocation(program_id, CString::new("in_color").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 4, gl::FLOAT, gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (4 * std::mem::size_of::<f32>()) as *const GLvoid); // offset of the first component

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for TexturedY8Pipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}
