#![cfg_attr(nightly, feature(never_type))] 

#![allow(dead_code)]

//TODO: XXX: Change default output to `stderr`, **NOT** stdout, ffs... Also add allow custom stream output.  Change behaviour that if `not isatty(S)` with `terminal_size` feature enabled an error is returned instead of *guessing* default sizes when it's not (caller can force by `unwrap_or*(50)`.)

macro_rules! flush {
    ($stream:expr) => {
	{
	    #![allow(unused_imports)]
	    use std::io::Write;
	    let _ = $stream.flush();
	}
    };
    () => {
	    use std::io::Write;
	    let _ = ::std::io::stdout().flush();
    };
    (? $stream:expr) => {
	{
	    #[allow(unused_imports)]
	    use std::io::Write;
	    $stream.flush()
	}
    };
    (?) => {
	    use std::io::Write;
	    ::std::io::stdout().flush()
    };
}

/// The default place to write bars to if an output is not user-specified.
pub(crate) type DefaultOutputDevice = std::io::Stdout;
/// A function that creates the default output device object for constructing a progress bar.
///
/// This must return multiple handles, since multiple bars can exist throughout the program at overlapping lifetimes.
/// `DefaultOutputDevice` should internally manage this state.
pub(crate) const CREATE_DEFAULT_OUTPUT_DEVICE_FUNC: fn () -> DefaultOutputDevice = std::io::stdout;

/// Create an object for the default output device.
#[inline] 
pub(crate) fn create_default_output_device() -> DefaultOutputDevice
{
    CREATE_DEFAULT_OUTPUT_DEVICE_FUNC()
}

#[cfg(feature="size")]
#[inline(always)] 
fn terminal_size_of(f: &(impl AsFd + ?Sized)) -> Option<(terminal_size::Width, terminal_size::Height)>
{
    terminal_size::terminal_size_of(f)
}

use atomic_refcell::AtomicRefCell;

//#[cfg(feature="size")] TODO: How to add `AsRawFd` bound to `Bar` *only* when `size` feature is enabled?
//use std::os::unix::io::*; // Not currently needed right now, platform-agnostic `AsFd` is used instead.
use std::os::fd::AsFd;

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
pub fn has_terminal_output(f: &(impl AsFd + ?Sized)) -> bool
{
    terminal_size::terminal_size_of(f).is_some()
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
