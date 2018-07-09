extern crate gl;
extern crate std;

use ::utils::*;

use self::gl::types::*;
use std::ffi::CString;

#[repr(C, packed)]
pub struct ColoredVertex {
    pub pos: [f32; 2], // "a_Pos"
    pub color: [f32; 4], // "a_Color"
}

#[repr(C, packed)]
pub struct ColoredLocals {
    pub transform: [[f32; 4]; 4], // "u_Transform"
}

pub struct ColoredPipeline {
    program: Program,
    vbo: GLuint,
    vao: GLuint,
    transform_location: GLint,
}

impl ColoredPipeline {
    pub fn new() -> Self {
        let vertex_shader = Shader::from_vert_str(include_str!("shaders/colored.glslv")).unwrap();
		let pixel_shader = Shader::from_frag_str(include_str!("shaders/colored.glslf")).unwrap();
        let program = Program::from_shaders(&[vertex_shader, pixel_shader]).unwrap();

        let (vbo, vao) = ColoredPipeline::create_vbo_and_vao();
        ColoredPipeline::specify_layout(program.id(), vbo, vao);

        let transform_location = unsafe {
            gl::GetUniformLocation(program.id(), CString::new("u_Transform").unwrap().as_ptr())
        };

        ColoredPipeline {
            program, vbo, vao, transform_location,
        }
    }

    pub fn apply(&mut self) {
        self.program.set_used();
    }

    pub fn draw(&mut self, array: &[ColoredVertex], locals: &ColoredLocals) {
        self.apply_array(array);
        self.apply_locals(locals);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, array.len() as GLint);
        }
    }

    fn apply_array(&mut self, array: &[ColoredVertex]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (array.len() * std::mem::size_of::<ColoredVertex>()) as GLsizeiptr, // size of data in bytes
                array.as_ptr() as *const GLvoid, // pointer to data
                gl::STREAM_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn apply_locals(&mut self, locals: &ColoredLocals) {
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

            gl::BindFragDataLocation(program_id, 0, CString::new("v_Color").unwrap().as_ptr());

            let pos_attr = gl::GetAttribLocation(program_id, CString::new("a_Pos").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null()); // offset of the first component

            let pos_attr = gl::GetAttribLocation(program_id, CString::new("a_Color").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 4, gl::FLOAT, gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (2 * std::mem::size_of::<f32>()) as *const GLvoid); // offset of the first component

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for ColoredPipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}
