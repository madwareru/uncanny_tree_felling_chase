use crate::atlas::rect::AtlasRect;

pub type RawImageU32 = RawImage<u32>;
pub type RawImageU8 = RawImage<u8>;

pub struct RawImage<TElement: bytemuck::Pod> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<TElement>
}

impl<TElement: bytemuck::Pod> RawImage<TElement> {
    pub fn create_zeroed(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![bytemuck::Zeroable::zeroed(); width * height]
        }
    }

    pub fn from_byte_slice(width: usize, height: usize, slice: &[u8]) -> Self {
        let casted = bytemuck::cast_slice(slice);
        assert_eq!(width * height, casted.len());
        Self {
            width,
            height,
            data: Vec::from(casted)
        }
    }
}

impl<TElement: bytemuck::Pod> super::Blittable<TElement> for RawImage<TElement> {
    fn blit_impl(&self, buffer: &mut [TElement], buffer_width: usize, self_rect: AtlasRect, dst_rect: AtlasRect) {
        let span_length = (self_rect.w).min(dst_rect.w);
        let span_count = self_rect.h.min(dst_rect.h);
        let colors = &self.data[..];

        let mut src_stride = self_rect.y * self.width + self_rect.x;
        let mut dst_stride = dst_rect.y * buffer_width + dst_rect.x;
        for _ in 0..span_count {
            let zipped = (&mut buffer[dst_stride..dst_stride+span_length])
                .iter_mut()
                .zip(&colors[src_stride..src_stride+span_length]);
            for (dest, src) in zipped {
                *dest = *src;
            }
            src_stride += self.width;
            dst_stride += buffer_width;
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl<'a, TElement, TBlittable> super::BlitDestination<'a, TElement, TBlittable>
for RawImage<TElement>
where
    TElement: bytemuck::Pod,
    TBlittable: super::Blittable<TElement>
{
    fn try_initiate_blit_on_self(
        &'a mut self, source_blittable: &'a TBlittable
    ) -> Option<super::BlitBuilder<'a, TElement, TBlittable>> {
        Some(super::BlitBuilder::create(&mut self.data[..], self.width, source_blittable))
    }
}