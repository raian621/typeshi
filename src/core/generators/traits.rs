use std::fmt::Debug;

pub trait Generator: Debug {
    fn get_token(&self) -> String;
}
