use application::Application;

pub mod application;
pub mod workspace;
pub mod buffer;
pub mod cursor;
pub mod jump_mode;

pub type Command = fn(&mut Application);
