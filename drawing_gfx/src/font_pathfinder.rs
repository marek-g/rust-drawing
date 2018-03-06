extern crate drawing;
extern crate app_units;
extern crate pathfinder_font_renderer;
extern crate pathfinder_partitioner;
extern crate pathfinder_path_utils;
extern crate lyon_path;

use self::drawing::font::*;
use self::drawing::backend::Font;

use self::pathfinder_font_renderer::{FontContext, FontInstance, FontKey, GlyphDimensions, GlyphKey, SubpixelOffset};
use self::pathfinder_partitioner::FillRule;
use self::pathfinder_partitioner::mesh_library::MeshLibrary;
use self::pathfinder_partitioner::partitioner::Partitioner;
use self::pathfinder_path_utils::transform::Transform2DPathIter;
use font_pathfinder::lyon_path::builder::{FlatPathBuilder, PathBuilder};

use self::app_units::Au;
use self::lyon_path::PathEvent;
use self::lyon_path::iterator::PathIter;

use std::sync::Arc;
use std::io::Cursor;

#[derive(Clone, Debug, PartialEq)]
pub struct PathfinderFont {
}

#[derive(Clone)]
struct PathDescriptor {
    path_index: usize,
    fill_rule: FillRule,
}

impl self::drawing::backend::Font for PathfinderFont {
    fn create(bytes: &[u8]) -> Self {
        // load font
        let mut font_context = FontContext::new().unwrap();
        let font_key = FontKey::new();
        font_context.add_font_from_memory(&font_key, Arc::new(bytes.to_vec()), 0).unwrap();

        // read glyph info
        let mut paths: Vec<Vec<PathEvent>> = vec![];
        let mut path_descriptors = vec![];
        let font_instance = FontInstance::new(&font_key, Au(60 * 16)); // TEST_FONT_SIZE
        let glyph_key = GlyphKey::new(68, SubpixelOffset(0)); // 'a'
        let glyph_outline = font_context.glyph_outline(&font_instance, &glyph_key).unwrap();
        //paths.push(Transform2DPathIter::new(glyph_outline.iter(), &[1, 0, 0, 1, 0, 0]))
        paths.push(glyph_outline.iter().collect());
        path_descriptors.push(PathDescriptor {
            path_index: 0,
            fill_rule: FillRule::Winding,
        });

        // partition glyph
        let mut library = MeshLibrary::new();
        for (stored_path_index, path_descriptor) in path_descriptors.iter().enumerate() {
            library.push_stencil_segments((path_descriptor.path_index + 1) as u16,
                PathIter::new(paths[stored_path_index].iter().cloned()));
            library.push_stencil_normals((path_descriptor.path_index + 1) as u16,
                paths[stored_path_index].iter().cloned());
        }
        let mut partitioner = Partitioner::new(library);
        for (path, path_descriptor) in paths.iter().zip(path_descriptors.iter()) {
            path.iter().for_each(|event| partitioner.builder_mut().path_event(*event));
            partitioner.partition((path_descriptor.path_index + 1) as u16,
                                  path_descriptor.fill_rule);
            partitioner.builder_mut().build_and_reset();
        }
        partitioner.library_mut().optimize();

        let mut data_buffer = Cursor::new(vec![]);
        drop(partitioner.library().serialize_into(&mut data_buffer));

        let i = 1;

        //info!("endpoints: {:#?}", glyph_outline_buffer.endpoints);
        //info!("control points: {:#?}", glyph_outline_buffer.control_points);

        PathfinderFont {}
    }

	fn get_dimensions(&mut self, font_params: FontParams, text: &str) -> (u16, u16) {
        (10, 10)
    }
}
