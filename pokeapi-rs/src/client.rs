use ureq::Agent;

pub(super) const BASE_URL: &str = "https://pokeapi.co/api/v2/";

#[derive(Clone, Debug)]
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

    // pub fn get<T>(&self, endpoint: &str) -> Result<T, ureq::Error>
    // where
    //     T: DeserializeOwned,
    // {
    //     self.agent.get(BASE_URL).call()?.into_json::<T>()
    // }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Agent> for Client {
    #[inline]
    fn from(agent: Agent) -> Self {
        Self { agent }
    }
}
