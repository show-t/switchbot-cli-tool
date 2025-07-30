use std::collections::HashMap;

#[derive(Debug)]
pub struct AliasResolver {
    aliases: HashMap<String, String>,
}

impl AliasResolver {
    pub fn new(aliases: HashMap<String, String>) -> Self {
        Self { 
            aliases
        }
    }
    pub fn resolve<'a>(&'a self, input: &'a str) -> &'a str {
        self.aliases.get(input).map(|s| s.as_str()).unwrap_or_else(|| input)
    }
}