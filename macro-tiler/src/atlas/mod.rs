pub mod rect;
pub mod rect_handle;
pub mod draw_command;

use {
    crate::{
        utility::{with_context_mut},
        blitting::raw_image::{RawImageU32, RawImage},
        blitting::BlitBuilder, atlas::rect::AtlasRect
    },
    macroquad::prelude::*,
    macroquad::miniquad::{TextureParams, TextureFormat, TextureWrap},
    guillotiere::{AtlasAllocator, size2, Allocation},
    std::{collections::HashMap, hash::Hash},
    serde::Deserialize
};
use crate::atlas::rect_handle::AtlasRectHandle;

#[derive(Clone, Debug, Deserialize)]
pub struct AtlasDefinition<TKey: 'static + Copy + Clone + Hash + Eq + Send + Sync> {
    pub entries: HashMap<TKey, AtlasRect>
}

#[derive(Clone, Debug)]
pub struct Atlas<TKey: 'static + Copy + Clone + Hash + Eq + Send + Sync> {
    textures: Vec<Texture2D>,
    mapping: HashMap<TKey, (usize, AtlasRect)>
}
impl<TKey: 'static + Copy + Clone + Hash + Eq + Send + Sync> Atlas<TKey> {
    pub fn load(
        definition: &AtlasDefinition<TKey>,
        bytes: &[u8],
        border_size: usize,
        format: TextureFormat,
        filter: FilterMode,
        wrap: TextureWrap
    ) -> Self {
        const WIDTH_PER_TEXTURE: usize = 2048;
        const HEIGHT_PER_TEXTURE: usize = 2048;

        let img = image::load_from_memory(bytes)
            .unwrap_or_else(|e| panic!("{}", e))
            .to_rgba8();
        let (img_width, img_height) = (img.width() as usize, img.height() as usize);
        let raw_bytes = img.into_raw();
        let src = RawImageU32::from_byte_slice(img_width, img_height, &raw_bytes);
        let mut stage_buffer = RawImageU32::create_zeroed(WIDTH_PER_TEXTURE, HEIGHT_PER_TEXTURE);

        let mut atlas_allocator = AtlasAllocator::new(
            size2(
                WIDTH_PER_TEXTURE as i32,
                HEIGHT_PER_TEXTURE as i32
            )
        );

        let mut current_texture_no = 0;
        let mut textures = Vec::new();
        let mut mapping = HashMap::new();

        for (&key, &rect) in definition.entries.iter() {
            let alloc_w = (rect.w + border_size * 2) as i32;
            let alloc_h = (rect.h + border_size * 2) as i32;
            let next_rect = match atlas_allocator.allocate(size2(alloc_w, alloc_h)) {
                None => {
                    textures.push(<Atlas<TKey>>::gen_texture(format, filter, wrap, &mut stage_buffer));
                    current_texture_no += 1;
                    atlas_allocator = AtlasAllocator::new(
                        size2(
                            WIDTH_PER_TEXTURE as i32,
                            HEIGHT_PER_TEXTURE as i32
                        )
                    );
                    stage_buffer = RawImageU32::create_zeroed(WIDTH_PER_TEXTURE, HEIGHT_PER_TEXTURE);
                    let next_rect_maybe = atlas_allocator.allocate(size2(alloc_w, alloc_h));
                    match next_rect_maybe {
                        Some(rect) => rect,
                        None => panic!(
                            "failed to allocate sub rect for size ({}, {})",
                            alloc_w,
                            alloc_h
                        )
                    }
                }
                Some(rect) => rect
            };

            let dst_rect = <Atlas<TKey>>::blit_sub_rect(
                border_size,
                &src,
                &mut stage_buffer,
                rect,
                next_rect
            );
            <Atlas<TKey>>::draw_borders(border_size, &mut stage_buffer, dst_rect);
            mapping.insert(key, (current_texture_no, dst_rect));
        }
        textures.push(<Atlas<TKey>>::gen_texture(format, filter, wrap, &mut stage_buffer));
        Self { textures, mapping }
    }

    pub fn acquire_handle(&self, index: &TKey) -> Option<AtlasRectHandle> {
        self.mapping.get(index).map(|&entry| { AtlasRectHandle(self.textures[entry.0], entry.1) })
    }

    fn blit_sub_rect(
        border_size: usize,
        src: &RawImage<u32>,
        stage_buffer: &mut RawImage<u32>,
        rect: AtlasRect,
        next_rect: Allocation
    ) -> AtlasRect {
        let min_p = next_rect.rectangle.min;
        let max_p = next_rect.rectangle.max;
        let dst_rect = AtlasRect {
            x: min_p.x as usize + border_size,
            y: min_p.y as usize + border_size,
            w: (max_p.x - min_p.x) as usize - (border_size * 2),
            h: (max_p.y - min_p.y) as usize - (border_size * 2)
        };
        BlitBuilder::try_create(stage_buffer, src).unwrap()
            .with_source_subrect(rect.x, rect.y, rect.w, rect.h)
            .with_dest_subrect(dst_rect.x as i32, dst_rect.y as i32, dst_rect.w, dst_rect.h)
            .blit();
        dst_rect
    }

    fn draw_borders(border_size: usize, stage_buffer: &mut RawImage<u32>, rect: AtlasRect) {
        for i in 0..rect.w {
            for j in 1..=border_size {
                let top = stage_buffer.width * (rect.y - j) + rect.x + i;
                let bottom = stage_buffer.width * (rect.y + rect.h - 1 + j) + rect.x + i;
                stage_buffer.data[top] = stage_buffer.data[top + stage_buffer.width];
                stage_buffer.data[bottom] = stage_buffer.data[bottom - stage_buffer.width];
            }
        }

        for j in (rect.y - border_size)..(rect.y + rect.h + border_size) {
            for i in 1..=border_size {
                let offset_left = stage_buffer.width * j + rect.x - i;
                let offset_right = stage_buffer.width * j + rect.x + rect.w - 1 + i;
                stage_buffer.data[offset_left] = stage_buffer.data[offset_left + 1];
                stage_buffer.data[offset_right] = stage_buffer.data[offset_right - 1];
            }
        }
    }

    fn gen_texture(
        format: TextureFormat,
        filter: FilterMode,
        wrap: TextureWrap,
        stage_buffer: &mut RawImage<u32>
    ) -> Texture2D {
        with_context_mut(|ctx| {
            Texture2D::from_miniquad_texture(
                miniquad::Texture::from_data_and_format(
                    ctx,
                    bytemuck::cast_slice(&stage_buffer.data[..]),
                    TextureParams {
                        width: stage_buffer.width as u32,
                        height: stage_buffer.height as u32,
                        format,
                        filter,
                        wrap,
                    },
                )
            )
        })
    }
}