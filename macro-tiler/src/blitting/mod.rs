pub mod raw_image;

use super::atlas::rect::AtlasRect;

pub trait Blittable<T> {
    fn blit_impl(
        &self,
        buffer: &mut [T],
        buffer_width: usize,
        self_rect: AtlasRect,
        dst_rect: AtlasRect
    );
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

fn blit_ext<T, TBlittable: Blittable<T>>(
    drawable: &TBlittable, buffer: &mut [T], buffer_width: usize,
    src_x: usize, src_y: usize,
    src_width: usize, src_height: usize,
    dst_x: i32, dst_y: i32,
    dst_width: usize, dst_height: usize
) {
    let src_width_max = (src_width + src_x).min(drawable.get_width());
    let src_height_max = (src_height + src_y).min(drawable.get_height());

    let dst_width_max = ((dst_width as i32 + dst_x) as usize).min(buffer_width);
    let dst_height_max = ((dst_height as i32 + dst_y) as usize).min(buffer.len() / buffer_width);

    let mut x_range_src = src_x.min(src_width_max)..src_width_max;
    let mut y_range_src = src_y.min(src_height_max)..src_height_max;

    let mut x_range_dst = 0..dst_width_max;
    let mut y_range_dst = 0..dst_height_max;

    if dst_x < 0 {
        x_range_src.start = (x_range_src.start + (-dst_x) as usize)
            .min(x_range_src.end);
    } else {
        x_range_dst.start = ((x_range_dst.start as i32 + dst_x) as usize)
            .min(x_range_dst.end);
    }
    if dst_y < 0 {
        y_range_src.start = (y_range_src.start + (-dst_y) as usize)
            .min(y_range_src.end);
    } else {
        y_range_dst.start = ((y_range_dst.start as i32 + dst_y) as usize)
            .min(y_range_dst.end);
    }

    let src_rect = AtlasRect {
        x: x_range_src.start,
        y: y_range_src.start,
        w: x_range_src.end - x_range_src.start,
        h: y_range_src.end - y_range_src.start
    };

    let dst_rect = AtlasRect {
        x: x_range_dst.start,
        y: y_range_dst.start,
        w: x_range_dst.end - x_range_dst.start,
        h: y_range_dst.end - y_range_dst.start
    };

    drawable.blit_impl(
        buffer,
        buffer_width,
        src_rect,
        dst_rect
    )
}

pub trait BlitDestination<'a, T, TBlittable: Blittable<T>> {
    fn try_initiate_blit_on_self(
        &'a mut self, source_blittable: &'a TBlittable
    ) -> Option<BlitBuilder<'a, T, TBlittable>>;
}

pub struct BlitBuilder<'a, T, TBlittable: Blittable<T>> {
    drawable: &'a TBlittable,
    buffer: &'a mut [T],
    buffer_width: usize,
    src_x: usize,
    src_y: usize,
    src_width: usize,
    src_height: usize,
    dst_x: i32,
    dst_y: i32,
    dst_width: usize,
    dst_height: usize
}
impl<'a, T, TBlittable: Blittable<T>> BlitBuilder<'a, T, TBlittable> {
    pub fn create(buffer: &'a mut [T], buffer_width: usize, drawable: &'a TBlittable) -> Self {
        let dst_height = buffer.len() / buffer_width;
        Self {
            drawable,
            buffer,
            buffer_width,
            src_x: 0,
            src_y: 0,
            src_width: drawable.get_width(),
            src_height: drawable.get_height(),
            dst_x: 0,
            dst_y: 0,
            dst_width: buffer_width,
            dst_height
        }
    }
    pub fn try_create(
        dest: & 'a mut impl BlitDestination<'a, T, TBlittable>,
        src: &'a TBlittable
    ) -> Option<Self> {
        dest.try_initiate_blit_on_self(src)
    }
    pub fn with_dest_pos(self, dst_x: i32, dst_y: i32) -> Self {
        Self {
            dst_x,
            dst_y,
            ..self
        }
    }
    pub fn with_source_subrect(self, src_x: usize, src_y: usize, src_width: usize, src_height: usize) -> Self {
        Self {
            src_x,
            src_y,
            src_width,
            src_height,
            ..self
        }
    }
    pub fn with_dest_subrect(self, dst_x: i32, dst_y: i32, dst_width: usize, dst_height: usize) -> Self {
        Self {
            dst_x,
            dst_y,
            dst_width,
            dst_height,
            ..self
        }
    }
    pub fn blit(&mut self) {
        blit_ext(
            self.drawable,
            self.buffer,
            self.buffer_width,
            self.src_x,
            self.src_y,
            self.src_width,
            self.src_height,
            self.dst_x,
            self.dst_y,
            self.dst_width,
            self.dst_height
        )
    }
}