#![cfg_attr(nightly, feature(never_type))] 

#![allow(dead_code)]

macro_rules! flush {
    () => {
	{
	    use std::io::Write;
	    let _ = std::io::stdout().flush();
	}
    }
}

#[cfg(feature="size")] 
use std::os::unix::io::*;

mod util;
mod inter;
pub use inter::*;

pub mod progress;
pub mod wheel;
pub mod spinner;
pub mod silent;

/// Returns true if `stdout` has a terminal output and can be used with terminal size responsiveness.
///
/// Requires `size` feature.
#[cfg(feature="size")] 
pub fn has_terminal_output_default() -> bool
{
    terminal_size::terminal_size().is_some()
}

/// Returns true if `f` has a terminal output and can be used with terminal size responsiveness.
///
/// Requires `size` feature.
#[cfg(feature="size")] 
pub fn has_terminal_output(f: &(impl AsRawFd + ?Sized)) -> bool
{
    terminal_size::terminal_size_using_fd(f.as_raw_fd()).is_some()
}

/// The prelude exposes the traits for spinners and progress bars, and the `spinner::Spin` and `progress::Bar` types for easy access and use.
pub mod prelude {
    pub use super::inter::*;
    pub use super::{
	spinner::Spin,
	progress::Bar,
	silent::Silent,
    };
}
