use crate::http::Method::POST;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Http {
    //Research this field is necessary or not
    id: String,
    // remove pub when it's unnecessary
    pub name: String,
    url: String,
    method: Method,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            name: "New http request".to_string(),
            url: "".to_string(),
            method: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
enum Method {
    POST,
}

impl Default for Method {
    fn default() -> Self {
        POST
    }
}
