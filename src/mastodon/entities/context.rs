use super::Status;

pub struct Context {
    ancestors: Vec<Status>,
    descendants: Vec<Status>,
}
