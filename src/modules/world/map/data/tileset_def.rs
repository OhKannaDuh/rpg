prelude!();
use super::tile_flags::TileFlags;

#[derive(Debug, Clone)]
pub struct TilesetDef {
    flags_by_tile: Vec<TileFlags>,
    custom_data_by_tile: Vec<String>,
}

impl TilesetDef {
    pub fn from_instance(instance: &ldtk_rust::TilesetDefinition) -> Self {
        let mut flags_by_tile =
            vec![TileFlags::empty(); (instance.c_hei * instance.c_wid) as usize];

        for tag in &instance.enum_tags {
            let Some(flag) = TileFlags::from_ldtk_key(&tag.enum_value_id) else {
                continue;
            };

            for tile_id in tag.tile_ids.iter().copied() {
                if (tile_id as usize) < flags_by_tile.len() {
                    flags_by_tile[tile_id as usize] |= flag;
                }
            }
        }

        let mut custom_data_by_tile =
            vec![String::new(); (instance.c_hei * instance.c_wid) as usize];

        for data in &instance.custom_data {
            custom_data_by_tile[data.tile_id as usize] = data.data.clone();
        }

        TilesetDef {
            flags_by_tile,
            custom_data_by_tile,
        }
    }

    pub fn flags(&self, tile_id: i64) -> TileFlags {
        self.flags_by_tile
            .get(tile_id as usize)
            .copied()
            .unwrap_or_else(TileFlags::empty)
    }

    pub fn custom_data(&self, tile_id: i64) -> String {
        self.custom_data_by_tile
            .get(tile_id as usize)
            .cloned()
            .unwrap_or_default()
    }
}
