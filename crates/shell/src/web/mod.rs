//! This module contains a platform specific implementation of the window shell.

use stdweb::js;

use crate::prelude::*;

pub use self::window::*;
pub use self::window_builder::*;

mod window;
mod window_builder;

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Initializes web stuff.
pub fn initialize() {
    set_panic_hook();
    stdweb::initialize();
}

/// Represents an application shell that could handle multiple windows. This implementation
/// is based on `minifb`.
pub struct Shell<A>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new() -> Self {
        Shell {
            window_shells: vec![],
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(
            self,
            adapter
        )
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(&mut self) {
       
    }
}

lazy_static! {
    pub static ref CONSOLE: Console = Console;
}

pub struct Console;

impl Console {
    pub fn time(&self, _name: impl Into<String>) {
        // js! {
        //     console.time(@{&name.into()})
        // }
    }

    pub fn time_end(&self, _name: impl Into<String>) {
        // js! {
        //     console.timeEnd(@{&name.into()})
        // }
    }

    pub fn log(&self, message: impl Into<String>) {
        js! {
            console.log(@{&message.into()});
        }
    }
}
