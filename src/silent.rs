//! A silent progress bar and spinner that does nothing.
//!
//! Useful for when progress bars are optional.

use super::*;


/// An implementor for the `Display`, `ProgressBar`, `Spinner`, and `WithTitle` that does nothing.
///
/// It also implements `Display::println()` and `Display::eprintln()` to do nothing as well.
#[derive(Debug)]
pub struct Silent;

impl Display for Silent
{
    #[inline] fn println(&self, _: &str){}
    #[inline] fn eprintln(&self, _: &str){}
    #[inline] fn refresh(&self){}
    #[inline] fn blank(&self){}
    #[inline] fn get_title(&self) -> &str{""}
    #[inline] fn set_title(&mut self, _: &str){}
    #[inline] fn update_dimensions(&mut self, _:usize){}
}

impl ProgressBar for Silent
{
    #[inline] fn set_progress(&mut self, _:f64){}
    #[inline] fn get_progress(&self) -> f64{0.0}
}

impl Spinner for Silent
{
    #[inline] fn bump(&mut self){}
}

impl WithTitle for Silent
{
    #[inline] fn with_title(_: usize, _: impl AsRef<str>) -> Self{Self}
    #[inline] fn update(&mut self) {}
    #[inline] fn complete(self) {}
}

