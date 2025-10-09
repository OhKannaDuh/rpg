prelude!();
module!(data);

#[derive(Resource, Clone, Default, Debug)]
pub struct LayerDefs(pub HashMap<i64, LayerDef>);
