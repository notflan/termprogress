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

/// An enum wrapper for a progress bar or spinner that might be silent.
#[derive(Debug)]
pub enum MaybeSilent<T>
{
    /// There is no progress
    Silent,
    /// There is progress
    Loud(T),
}

impl<T> Display for MaybeSilent<T>
where T: Display
{
    
    fn refresh(&self)
    {
	if let Self::Loud(this) = self {
	    this.refresh();
	}
    }
    fn blank(&self)
    {
	
	if let Self::Loud(this) = self {
	    this.blank();
	}
    }
    fn println(&self, string: &str)
    {	
	if let Self::Loud(this) = self {
	    this.println(string);
	}
    }
    fn eprintln(&self, string: &str)
    {
	if let Self::Loud(this) = self {
	    this.eprintln(string)
	}
    }

    fn get_title(&self) -> &str
    {
	if let Self::Loud(this) = self {
	    this.get_title()
	} else {
	    ""
	}
    }
    fn set_title(&mut self, from: &str)
    {
	if let Self::Loud(this) = self {
	    this.set_title(from);
	}
    }

    fn update_dimensions(&mut self, to: usize)
    {
	if let Self::Loud(this) = self {
	    this.update_dimensions(to)
	}
    }
}


impl<T> ProgressBar for MaybeSilent<T>
    where T: ProgressBar
{
    fn set_progress(&mut self, value: f64)
    {
	if let Self::Loud(this) = self {
	    this.set_progress(value)
	}
    }
    fn get_progress(&self) -> f64
    {
	if let Self::Loud(this) = self {
	    this.get_progress()
	} else {
	    0.0
	}
    }
}

impl<T> Spinner for MaybeSilent<T>
    where T: Spinner
{
    fn bump(&mut self)
    {
	if let Self::Loud(this) = self {
	    this.bump()
	}
    }
}

/// A trait for creating a progress bar or spinner with a title.
impl<T> WithTitle for MaybeSilent<T>
    where T: WithTitle
{
    fn with_title(len: usize, string: impl AsRef<str>) -> Self
    {
	Self::Loud(T::with_title(len, string))
    }
    fn update(&mut self)
    {
	if let Self::Loud(this) = self {
	    this.update()
	}
    }
    fn complete(self)
    {
	if let Self::Loud(this) = self {
	    this.complete()
	}
    }
}

impl<T> From<Option<T>> for MaybeSilent<T>
{
    #[inline] fn from(from: Option<T>) -> Self
    {
	match from {
	    Some(from) => Self::Loud(from),
	    None => Self::Silent,
	}
    }
}

impl<T> From<MaybeSilent<T>> for Option<T>
{
    fn from(from: MaybeSilent<T>) -> Self
    {
	match from {
	    MaybeSilent::Loud(loud) => Some(loud),
	    _ => None,
	}
    }
}

/// Return a `MaybeSilent` that is always silent
#[cfg(nightly)]
pub const fn always() -> MaybeSilent<!>
{
    MaybeSilent::Silent
}

/// Return a `MaybeSilent` that is always silent
#[cfg(not(nightly))]
pub const fn always() -> MaybeSilent<Silent>
{
    MaybeSilent::Silent
}

impl<T> MaybeSilent<T>
{
    /// Is this the not silent variant?
    #[inline] pub fn is_loud(&self) -> bool
    {
	!self.is_silent()
    }
    /// Is this the silent variant?
    #[inline] pub fn is_silent(&self) -> bool
    {
	if let Self::Silent = self {
	    true
	} else {
	    false
	}
    }
    /// Create a new `MaybeSilent` with a value.
    pub const fn new_some(value: T) -> Self
    {
	Self::Loud(value)
    }

    /// Create a new `MaybeSilent` with a potential value
    #[inline] pub fn new<U>(from: U) -> Self
    where U: Into<Option<T>>
    {
	match from.into() {
	    Some(x) => Self::Loud(x),
	    _ => Self::Silent,
	}
    }
    
    /// Get a reference to the inner type if possible
    pub fn as_ref(&self) -> Option<&T>
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }
    /// Get a mutable reference to the inner type if possible
    pub fn as_mut(&mut self) -> Option<&mut T>
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }

    
    /// Get a dynamic mutable reference to the internal value if it is `Display`.
    pub fn as_display_mut(&mut self) -> Option<&mut (dyn Display + 'static)>
    where T: Display + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }

    /// Consume this instance and return the inner value if possible
    #[inline] pub fn into_inner(self) -> Option<T>
    {
	self.into()
    }

    /// Consume this instance and return silent if it had no value
    #[inline] pub fn into_silent(self) -> Option<Silent>
    {
	match self {
	    Self::Silent => Some(Silent),
	    _ => None
	}
    }
    
    /// Get a dynamic mutable reference to the internal value if it is `ProgressBar`
    pub fn as_bar_mut(&mut self) -> Option<&mut (dyn ProgressBar + 'static)>
    where T: ProgressBar + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }
    
    /// Get a dynamic mutable reference to the internal value if it is `Spinner`.
    pub fn as_spinner_mut(&mut self) -> Option<&mut (dyn Spinner + 'static)>
    where T: Spinner + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }

    /// Get a dynamic reference to the internal value if it is `Display`.
    pub fn as_display(&self) -> Option<&(dyn Display + 'static)>
    where T: Display + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }
    
    /// Get a dynamic reference to the internal value if it is `ProgressBar`
    pub fn as_bar(&self) -> Option<&(dyn ProgressBar + 'static)>
    where T: ProgressBar + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }
    
    /// Get a dynamic reference to the internal value if it is `Spinner`.
    pub fn as_spinner(&self) -> Option<&(dyn Spinner + 'static)>
    where T: Spinner + 'static
    {
	match self {
	    Self::Loud(loud) => Some(loud),
	    _ => None
	}
    }
}
