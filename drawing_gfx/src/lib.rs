#[macro_use]
extern crate gfx;

#[cfg(target_os = "windows")]
#[path="dx11/backend_dx11.rs"]
pub mod backend;

#[cfg(not(target_os = "windows"))]
#[path="opengl/backend_opengl.rs"]
pub mod backend;

pub mod font_pathfinder;
