use super::Status;

pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
