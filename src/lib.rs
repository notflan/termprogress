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

#[cfg(test)]
mod tests {
    
}
