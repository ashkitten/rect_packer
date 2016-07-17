use std::collections::HashMap;

use {
    TexturePackerConfig,
};

use rect::Rect;

use frame::Frame;
use packer::{
    Packer,
    SkylinePacker,
};

pub struct TexturePacker<P> {
    frames: HashMap<String, Frame>,
    packer: P,
    config: TexturePackerConfig,
}

impl TexturePacker<SkylinePacker> {
    pub fn new_skyline(config: TexturePackerConfig) -> TexturePacker<SkylinePacker> {
        TexturePacker {
            frames: HashMap::new(),
            packer: SkylinePacker::new(config),
            config: config,
        }
    }
}

impl<P: Packer> TexturePacker<P> {
    pub fn pack(&mut self, key: String, (width, height): (u32, u32)) {
        let source = Rect::new(0, 0, width, height);

        if let Some(mut frame) = self.packer.pack(key.clone(), (width, height)) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            frame.source = source;
            frame.source.w = width;
            frame.source.h = height;
            self.frames.insert(key.clone(), frame);
        }
    }

    pub fn get_frames(&self) -> &HashMap<String, Frame> {
        &self.frames
    }

    pub fn get_frame(&self, key: &str) -> Option<&Frame> {
        if let Some(frame) = self.frames.get(key) {
            Some(frame)
        } else {
            None
        }
    }
}
