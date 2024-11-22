#[derive(Debug, Default)]
struct Table {
    name: String,
    ptype: String,
    chains: Vec<Chain>,
}

impl Table {
    fn add_chain(&mut self, chain: Chain) {
        self.chains.push(chain);
    }

    fn remove_chain(&mut self, chain: &Chain) {
        self.chains.retain(|c| c != chain);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum ChainType {
    #[default]
    Filter,
    Nat,
    Mangle,
    Raw,
    Security,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Hook {
    #[default]
    Input,
    Forward,
    Output,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Chain {
    name: String,
    chain_type: ChainType,
    rules: Vec<Rule>,
}

impl Chain {
    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn remove_rule(&mut self, rule: Rule) {
        self.rules.retain(|r| r != &rule);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RuleCriteria {
    Saddr(String),
    Daddr(String),
    Sport(u16),
    Dport(u16),
}

impl Default for RuleCriteria {
    fn default() -> Self {
        RuleCriteria::Saddr("127.0.0.1".to_string())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Rule {
    criteria: RuleCriteria,
    queue: u16,
}
