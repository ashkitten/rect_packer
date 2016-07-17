use texture::{
    Texture,
};

#[derive(Copy, Clone)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct MemoryRGBA8Texture {
    pixels: Vec<RGBA8>,
    width: u32,
    height: u32,
}

impl MemoryRGBA8Texture {
    pub fn from_memory(buf: &[u8], w: u32, h: u32) -> MemoryRGBA8Texture {
        let mut pixels = Vec::new();

        for pixel in buf.chunks(4) {
            pixels.push(RGBA8 {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }

        MemoryRGBA8Texture {
            pixels: pixels,
            width: w,
            height: h,
        }
    }

    #[inline(always)]
    fn index_for(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

impl Texture for MemoryRGBA8Texture {
    type Pixel = RGBA8;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get(&self, x: u32, y: u32) -> Option<RGBA8> {
        if let Some(p) = self.pixels.get(self.index_for(x, y)) {
            Some(*p)
        } else {
            None
        }
    }

    fn set(&mut self, x: u32, y: u32, val: RGBA8) {
        let index = self.index_for(x, y);
        self.pixels[index] = val;
    }
}
