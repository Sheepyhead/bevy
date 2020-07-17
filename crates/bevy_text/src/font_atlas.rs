use bevy_asset::{Assets, Handle};
use bevy_math::Vec2;
use bevy_render::texture::Texture;
use bevy_sprite::{DynamicTextureAtlasBuilder, TextureAtlas};
use std::collections::HashMap;

pub struct FontAtlas {
    pub dynamic_texture_atlas_builder: DynamicTextureAtlasBuilder,
    pub glyph_to_index: HashMap<char, u32>,
    pub texture_atlas: Handle<TextureAtlas>,
}

impl FontAtlas {
    pub fn new(
        textures: &mut Assets<Texture>,
        texture_atlases: &mut Assets<TextureAtlas>,
        size: Vec2,
    ) -> FontAtlas {
        let atlas_texture = textures.add(Texture::new_fill(size, &[0, 0, 0, 0]));
        let texture_atlas = TextureAtlas::new_empty(atlas_texture, size);
        Self {
            texture_atlas: texture_atlases.add(texture_atlas),
            glyph_to_index: HashMap::new(),
            dynamic_texture_atlas_builder: DynamicTextureAtlasBuilder::new(size),
        }
    }

    pub fn get_char_index(&self, character: char) -> Option<u32> {
        self.glyph_to_index.get(&character).cloned()
    }

    pub fn add_char(
        &mut self,
        textures: &mut Assets<Texture>,
        texture_atlases: &mut Assets<TextureAtlas>,
        character: char,
        texture: &Texture,
    ) {
        let texture_atlas = texture_atlases.get_mut(&self.texture_atlas).unwrap();
        if let Some(index) =
            self.dynamic_texture_atlas_builder
                .add_texture(texture_atlas, textures, texture)
        {
            self.glyph_to_index.insert(character, index);
        } else {
            panic!("ran out of space in font atlas");
        }
    }
}
