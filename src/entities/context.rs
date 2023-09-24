use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
