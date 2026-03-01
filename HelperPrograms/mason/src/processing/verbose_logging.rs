use std::sync::Mutex;
use crate::arguments::arguments;
use crate::arguments::arguments::Arguments;

#[derive(Debug)]
pub(super) struct State {
    is_verbose: bool,
}

impl State {
    const fn default() -> Self {
        Self { is_verbose: false }
    }
}

pub(super) static VERBOSE_LOGGER: Mutex<State> = Mutex::new(State::default());

pub(super) trait VerboseLogger<'a> {
    fn log(&'a self, message: &str);
    fn set_verbose(&'a self, arguments: &arguments::Arguments);
}

impl <'a> VerboseLogger<'a> for Mutex<State> {
    fn log(&'a self, message: &str) {
        if self.lock().unwrap().is_verbose {
            println!("INFO: {}", message);
        }
    }

    fn set_verbose(&'a self, arguments: &Arguments) {
        let mut state = self.lock().unwrap();
        state.is_verbose = arguments.verbose();
    }
}

pub(super) fn verbose_log(message: &str) {
    VERBOSE_LOGGER.log(message);
}