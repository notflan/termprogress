#![allow(dead_code)]

macro_rules! flush {
    () => {
	{
	    use std::io::Write;
	    let _ = std::io::stdout().flush();
	}
    }
}

mod util;
mod inter;
pub use inter::*;

pub mod progress;
pub mod wheel;
pub mod spinner;
pub mod silent;

/// The prelude exposes the traits for spinners and progress bars, and the `spinner::Spin` and `progress::Bar` types for easy access and use.
pub mod prelude {
    pub use super::inter::*;
    pub use super::{
	spinner::Spin,
	progress::Bar,
	silent::Silent,
    };
}
