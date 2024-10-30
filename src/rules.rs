use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct Packet {
    data: Vec<u8>,
}

type TransformMap = HashMap<Packet, Packet>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct RulesHandler {
    rules: TransformMap,
}

impl RulesHandler {
    pub fn new() -> Self {
        Self {
            rules: TransformMap::new(),
        }
    }

    pub fn add_rule(&mut self, src: Packet, dst: Packet) {
        self.rules.insert(src, dst);
    }

    pub fn remove_rule(&mut self, src: Packet) {
        self.rules.remove(&src);
    }
}
