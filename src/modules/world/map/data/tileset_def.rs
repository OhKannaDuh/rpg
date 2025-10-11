prelude!();

use super::tile_flags::TileFlags;
use crate::modules::world::map::{FromLdtkTile, LdtkComponentRegistry};

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

    fn custom_data(&self, tile_id: i64) -> String {
        self.custom_data_by_tile
            .get(tile_id as usize)
            .cloned()
            .unwrap_or_default()
    }

    fn parse_custom_data(s: &str) -> Option<(String, HashMap<String, String>)> {
        let s = s.trim();
        let (name, rest) = s.split_once('(')?;
        let name = name.trim().to_string();
        let rest = rest.strip_suffix(')')?.trim();

        let mut kv = HashMap::new();
        for pair in rest.split(',') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            let (k, v) = pair.split_once('=')?;
            kv.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
        }
        Some((name, kv))
    }

    fn parse_custom_data_iter(
        s: &str,
    ) -> impl Iterator<Item = (String, HashMap<String, String>)> + '_ {
        s.split(';')
            .map(str::trim)
            .filter(|seg| !seg.is_empty())
            .filter_map(TilesetDef::parse_custom_data)
    }

    pub fn create_components<'a>(
        &'a self,
        tile: &'a TileInstance,
        registry: &'a LdtkComponentRegistry,
    ) -> impl Iterator<Item = Box<dyn Reflect>> + 'a {
        let data = self.custom_data(tile.t);

        let components: Vec<Box<dyn Reflect>> = TilesetDef::parse_custom_data_iter(&data)
            .filter_map(|(name, kv)| registry.create(&name, tile, &kv))
            .collect();

        components.into_iter()
    }
}
