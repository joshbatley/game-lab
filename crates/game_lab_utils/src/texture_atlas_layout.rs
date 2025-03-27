use bevy::math::UVec2;
use bevy::sprite::TextureAtlasLayout;

pub fn texture_atlas_layout_with_padding(size: UVec2, columns: u32, rows: u32, padding: u32) -> TextureAtlasLayout {
    let mut x = TextureAtlasLayout::from_grid(
        size,
        columns,
        rows,
        Some(UVec2::splat(padding)),
        Some(UVec2::splat(padding / 2)));
    x.size = (size + UVec2::splat(padding)) * (UVec2::new(columns, rows));
    x
}