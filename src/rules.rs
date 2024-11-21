use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Packet {
    data: Vec<u8>
}

type TransformationMap = HashMap<Packet, Packet>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RuleHandler {
    rules: TransformationMap
}

impl RuleHandler {
    pub fn add_rule(&mut self, src: Packet, dst: Packet) {
        self.rules.insert(src, dst);
    }

    pub fn remove_rule(&mut self, src: Packet) {
        self.rules.remove(&src);
    }
}