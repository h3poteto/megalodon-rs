use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
