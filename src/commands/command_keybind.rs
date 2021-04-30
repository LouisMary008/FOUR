use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use super::KeyCommand;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(KeyCommand),
    CompositeKeybind(AppKeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}

pub trait JoshutoRunnable {
    fn execute(&self, context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()>;
}

pub trait JoshutoCommand: JoshutoRunnable + std::fmt::Display + std::fmt::Debug {}
