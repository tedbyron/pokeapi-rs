use ureq::Agent;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/";

#[derive(Debug, Clone)]
pub struct Client {
    agent: Agent,
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
