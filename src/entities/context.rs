use super::Status;

#[derive(Debug, Clone)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
