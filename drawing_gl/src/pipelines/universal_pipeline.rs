extern crate drawing;
extern crate gl;
extern crate std;

use crate::utils::*;
use drawing::color::Color;
use std::os::raw::c_void;

use self::drawing::backend::TexturedVertex;
use self::gl::types::*;
use std::ffi::CString;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct FragUniforms {
    pub scissor_mat: [f32; 12],
    pub paint_mat: [f32; 12],
    pub inner_color: Color,
    pub outer_color: Color,
    pub scissor_ext: [f32; 2],
    pub scissor_scale: [f32; 2],
    pub extent: [f32; 2],
    pub radius: f32,
    pub feather: f32,
    pub stroke_mult: f32,
    pub stroke_thr: f32,
    pub tex_type: i32,
    pub type_: i32,
    // warning! always add padding to multiply of 32 bytes (std140 layout rules)
    //pub _padding: [u8; 16],
}

pub enum ShaderType {
    FillGradient,
    FillImage,
    Simple,
    Image,
}

pub struct UniversalPipeline {
    program: Program,
    vbo: GLuint,
    vao: GLuint,
    transform_location: GLint,
    flipped_y_location: GLint,
    frag_uniform_buf: GLuint,
}

impl UniversalPipeline {
    pub fn new() -> Self {
        let vertex_shader = Shader::from_vert_str(include_str!("shaders/universal.glslv")).unwrap();
        let pixel_shader = Shader::from_frag_str(include_str!("shaders/universal.glslf")).unwrap();
        let program = Program::from_shaders(&[vertex_shader, pixel_shader]).unwrap();

        let transform_location = unsafe {
            gl::GetUniformLocation(program.id(), CString::new("transform").unwrap().as_ptr())
        };
        let flipped_y_location = unsafe {
            gl::GetUniformLocation(program.id(), CString::new("flipped_y").unwrap().as_ptr())
        };

        let frag_uniform_buf = unsafe {
            let loc_frag =
                gl::GetUniformBlockIndex(program.id(), CString::new("frag").unwrap().as_ptr());
            gl::UniformBlockBinding(program.id(), loc_frag, 0);

            let mut frag_uniform_buf: gl::types::GLuint = std::mem::zeroed();
            gl::GenBuffers(1, &mut frag_uniform_buf);

            //let mut align = std::mem::zeroed();
            //gl::GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, &mut align);

            frag_uniform_buf
        };

        UniversalPipeline {
            program,
            vbo: 0,
            vao: 0,
            transform_location,
            flipped_y_location,
            frag_uniform_buf,
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

    pub fn apply_frag_uniforms(&self, uniforms: &FragUniforms) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.frag_uniform_buf);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (1 * std::mem::size_of::<FragUniforms>()) as GLsizeiptr,
                //1 as isize, // TODO: is 1 or sizeof() correct?
                uniforms as *const FragUniforms as *const c_void,
                //gl::STREAM_DRAW,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

            gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, self.frag_uniform_buf);

            /*gl::BindBufferRange(
                gl::UNIFORM_BUFFER,
                0,
                self.frag_uniform_buf,
                (0/* offset * self.frag_size*/) as isize,
                11 * 16 as isize,
                //std::mem::size_of::<FragUniforms>() as isize,
            );*/
        }
    }

    /*pub fn apply_frag_uniforms_buffer(&self, uniforms: &[FragUniforms]) {
        gl::BindBuffer(gl::UNIFORM_BUFFER, self.frag_uniform_buf);
        gl::BufferData(
            gl::UNIFORM_BUFFER,
            uniforms.len() as isize,
            uniforms.as_ptr() as *const c_void,
            gl::STREAM_DRAW,
        );
    }

    pub fn set_frag_uniforms(&self, uniforms: &FragUniforms) {
        unsafe {
            gl::BindBufferRange(
                gl::UNIFORM_BUFFER,
                0,
                self.frag_uniform_buf,
                (0 /* offset */ * self.frag_size) as isize,
                std::mem::size_of::<FragUniforms>() as isize,
            );

            if let Some(img) = img {
                if let Some(texture) = self.textures.get(img) {
                    gl::BindTexture(gl::TEXTURE_2D, texture.tex);
                }
            } else {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }*/

    pub fn draw(&mut self, array: &[TexturedVertex], mode: GLenum) {
        self.apply_array(array);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(mode, 0, array.len() as GLint);
        }
    }

    fn apply_array(&mut self, array: &[TexturedVertex]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                    // target
                (array.len() * std::mem::size_of::<TexturedVertex>()) as GLsizeiptr, // size of data in bytes
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

        UniversalPipeline::specify_layout(self.program.id(), vbo, vao);

        (vbo, vao)
    }

    fn specify_layout(program_id: GLuint, vbo: GLuint, vao: GLuint) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let pos_attr =
                gl::GetAttribLocation(program_id, CString::new("in_position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),
            ); // offset of the first component

            let pos_attr =
                gl::GetAttribLocation(program_id, CString::new("in_tex_coords").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (8 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                (2 * std::mem::size_of::<f32>()) as *const GLvoid,
            ); // offset of the first component

            let pos_attr =
                gl::GetAttribLocation(program_id, CString::new("in_color").unwrap().as_ptr());
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

impl Drop for UniversalPipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.frag_uniform_buf);
        }
    }
}
