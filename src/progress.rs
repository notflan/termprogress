///! Progress bar that has a size and also a max size.

use super::*;
use std::{
    fmt::Write,
    io,
};
use atomic_refcell::AtomicRefCell;

/// A progress bar with a size and optionally title. It implements the `ProgressBar` trait, and is the default progress bar.
///
/// The bar has a max size, that is usually derived from the size of your terminal (if it can be detected) or can be set yourself, to stop the title line going over the side.
/// It also has a configurable width, which is defaulted to 50.
///
/// # Usage
///
/// To create a new `progress::Bar` with a max size tuned to your terminal (or `Width+20`, if it cannot be detected), and of the default size, `Bar` implements the `Default` trait:
/// ```rust
/// # use termprogress::prelude::*;
/// let mut bar = Bar::default(); //Creates a bar of width 50 by default.
/// ```
///
/// You can configure sizes and initial title with `new()`, `with_title()`, and `with_max()` functions.
/// # How it looks
/// It renders in the terminal like:
/// `[=========================                         ]: 50% this is a title that may get cut if it reaches max le...`
///
/// # Thread `Sync`safety
/// This type is safely `Sync` (where `T` is), the behaviour is defined to prevent overlapping writes to `T`.
/// Though it is *advised* to not render a `Bar` from more than a single thread, you still safely can.
///
/// Rendering functions should not be called on multiple threads at the same time, though it is safe to do so.
/// A thread-sync'd rendering operation will safetly (and silently) give up before writing if another thread is already engaging in one.
///
/// A display operation on one thread will cause any other threads attempting on to silently and safely abort their display attempt before anything is written to output.
#[derive(Debug)]
pub struct Bar<T: ?Sized = DefaultOutputDevice> 
{
    width: usize,
    max_width: usize,
    progress: f64,
    buffer: String,
    title: String,
    #[cfg(feature="size")]
    fit_to_term: bool,
    
    // Allowing `Bar` to manage the sync will ensure that the bar is not interrupted by another bar-related write, and so any accidental inter-thread corrupting writes will not be drawn (unlike if we relied on `T`'s sync, since we have multiple `write()` calls when rendering and blanking.) *NOTE*: using `AtomicRefCell` i think is actually still be preferable for those reasons. If `T` can be shared and written to with internal sync (like stdout/err,) then non-`Bar` writes are not affected, but `Bar` writes are better contained.
    output: AtomicRefCell<T>
}

/// The default size of the terminal bar when the programmer does not provide her own.
/// Or if `size` is not used.
pub const DEFAULT_SIZE: usize = 50;

/// The default size of the max render bar when the programmer does not provide her own.
/// Or if `size` is not used.
pub const DEFAULT_MAX_BORDER_SIZE: usize = 20;

/*
impl<T: Default + io::Write> Default for Bar<T>
{
#[inline] 
fn default() -> Self
{
Self::new(T::default(), DEFAULT_SIZE)
    }
}
 */

impl Default for Bar
{
    #[inline]
    fn default() -> Self
    {
	Self::new_default(DEFAULT_SIZE)
    }
}


impl Bar {
    /// Create a new bar `width` long with a title using the default output descriptor `stdout`.
    #[inline] 
    pub fn with_title_default(width: usize, title: impl AsRef<str>) -> Self
    {
	Self::with_title(create_default_output_device(), width, title)
    }

    /// Attempt to create a new bar with max display width of our terminal and a title.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    #[inline] 
    pub fn try_new_with_title_default(width: usize, title: impl AsRef<str>) -> Option<Self>
    {
	Self::try_new_with_title(create_default_output_device(), width, title)
    }
    
    /// Create a new bar with max display width of our terminal
    ///
    /// # Notes
    /// Without feature `size`, will be the same as `Self::with_max(width, width +20)`
    ///
    /// To try to create one that always adheres to `size`, use the `try_new()` family of functions.
    #[inline]
    pub fn new_default(width: usize) -> Self
    {
	Self::new(create_default_output_device(), width)
    }
    
    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    #[inline] 
    pub fn try_new_default(width: usize) -> Option<Self>
    {
	Self::try_new(create_default_output_device(), width)
    }
    
    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    #[inline] 
    pub fn try_new_default_size_default() -> Option<Self>
    {
	Self::try_new_default_size(create_default_output_device())
    }
    
    /// Create a bar with a max display width
    ///
    /// # Panics
    /// If `width` is larger than or equal to `max_width`.
    #[inline] 
    pub fn with_max_default(width: usize, max_width: usize) -> Self
    {
	Self::with_max(create_default_output_device(), width, max_width)
    }
}

impl<T: io::Write + AsRawFd> Bar<T>
{
    

    /// Create a new bar `width` long with a title.
    pub fn with_title(output: impl Into<T> + AsRawFd, width: usize, title: impl AsRef<str>) -> Self
    {
	let mut this = Self::new(output, width);
	this.add_title(title.as_ref());
	this
    }

    #[inline(always)] fn add_title(&mut self, title: &str)
    {
	self.set_title(title);
	self.update()
    }
    
    /// Attempt to create a new bar with max display width of our terminal and a title.
    ///
    /// If `output` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    pub fn try_new_with_title(output: impl Into<T> + AsRawFd, width: usize, title: impl AsRef<str>) -> Option<Self>
    {
	let (terminal_size::Width(tw), _) = terminal_size_of(&output)?;
	let tw = usize::from(tw);
	let mut o = Self::with_max(output.into(), if width < tw {width} else {tw}, tw);
	o.set_title(title.as_ref());
	o.fit_to_term = true;
	o.update();
	Some(o)
    }

    #[inline]
    fn autofit(&mut self)
    {
	#[cfg(feature="size")]
	self.fit();
    }
    
    /// Create a new bar with max display width of our terminal
    ///
    /// # Notes
    /// Without feature `size`, will be the same as `Self::with_max(width, width +20)`
    ///
    /// To try to create one that always adheres to `size`, use the `try_new()` family of functions.
    #[cfg_attr(not(feature="size"), inline)]
    pub fn new(output: impl Into<T> + AsRawFd, width: usize) -> Self
    {
	#[cfg(feature="size")]
	return {
	    if let Some((terminal_size::Width(tw), _)) = terminal_size_of(&output) {
		let tw = usize::from(tw);
		let mut o = Self::with_max(output.into(), if width < tw {width} else {tw}, tw);
		o.fit_to_term = true;
		o
	    } else {
		let mut o = Self::with_max(output.into(), width, width + DEFAULT_MAX_BORDER_SIZE);
		o.fit_to_term = true;
		o
	    }
	};
	#[cfg(not(feature="size"))]
	return {
	    Self::with_max(output.into(), width, width +DEFAULT_MAX_BORDER_SIZE)
	};
    }

    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `output` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    pub fn try_new(output: impl Into<T> + AsRawFd, width: usize) -> Option<Self>
    {
	let (terminal_size::Width(tw), _) = terminal_size_of(&output)?;
	let tw = usize::from(tw);
	let mut o = Self::with_max(output.into(), if width < tw {width} else {tw}, tw);
	o.fit_to_term = true;
	Some(o)
    }
    
    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `output` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    #[inline] 
    pub fn try_new_default_size(to: impl Into<T> + AsRawFd) -> Option<Self>
    {
	Self::try_new(to, DEFAULT_SIZE)
    }
    
    /// Create a bar with a max display width
    ///
    /// # Panics
    /// If `width` is larger than or equal to `max_width`.
    pub fn with_max(output: impl Into<T>, width: usize, max_width: usize) -> Self
    {
	let mut this = Self {
	    width,
	    max_width,
	    progress: 0.0,
	    buffer: String::with_capacity(width),
	    title: String::with_capacity(max_width - width),
	    #[cfg(feature="size")] 
	    fit_to_term: false,
	    output: AtomicRefCell::new(output.into())
	};
	this.update();
	this
    }

}

impl<T: ?Sized + io::Write + AsRawFd> Bar<T> {
    #[inline(always)]
    #[cfg(feature="size")]
    fn try_get_size(&self) -> Option<(terminal_size::Width, terminal_size::Height)>
    {
	let b = self.output.try_borrow().ok()?;
	terminal_size::terminal_size_using_fd(b.as_raw_fd())
    }
    /// Fit to terminal's width if possible.
    ///
    /// # Notes
    /// Available with feature `size` only.
    ///
    /// # Returns
    /// If the re-fit succeeded.
    /// A `fit()` will also fail if another thread is already engaging in a display operation.
    pub fn fit(&mut self) -> bool
    {
	#[cfg(feature="size")] {
	    if let Some((terminal_size::Width(tw), _)) = self.try_get_size() {
		let tw = usize::from(tw);
		self.width = if self.width < tw {self.width} else {tw};
		self.update_dimensions(tw);
		return true;
	    }
	}
	false
    }

    #[inline] fn widths(&self) -> (usize, usize)
    {
	#[cfg(feature="size")] 
	if self.fit_to_term {
	    if let Some((terminal_size::Width(tw), _)) = self.try_get_size() {
		let tw = usize::from(tw);
		let width = if self.width < tw {self.width} else {tw};
		return (width, tw);
	    }
	};
	(self.width, self.max_width)
    }
    
    /// Update the buffer.
    pub fn update(&mut self)
    {
	self.buffer.clear();

	let pct = (self.progress * (self.width as f64)) as usize;
	for i in 0..self.width
	{
	    if i >= pct {
		write!(self.buffer, " ").unwrap();
	    } else {
		write!(self.buffer, "=").unwrap();
	    }
	}
    }

}
impl<T: io::Write> Bar<T> {
    /// Consume the bar and complete it, regardless of progress.
    pub fn complete(self) -> io::Result<()>
    {
	writeln!(&mut self.output.into_inner(), "")
    }
}

fn ensure_eq(input: String, to: usize) -> String
{
    let  chars = input.chars();
    if chars.count() != to {
	let mut chars = input.chars();
	let mut output = String::with_capacity(to);
	for _ in 0..to
	{
	    if let Some(c) = chars.next() {
		write!(output, "{}", c).unwrap();
	    } else {
		write!(output, " ").unwrap();
	    }
	}
	output
    } else {
	input
    }
}


fn ensure_lower(input: String, to: usize) -> String
{
    let chars = input.chars();
    if chars.count() > to
    {
	let chars = input.chars();
	let mut output = String::with_capacity(to);
	for (i,c) in (0..).zip(chars)
	{
	    write!(output, "{}", c).unwrap();
	    if to>3 && i == to-3 {
		write!(output, "...").unwrap();
		break;
	    } else if i==to {
		break;
	    }
	}

	output
    } else {
	input
    }
}

impl<T: ?Sized + io::Write + AsRawFd> Display for Bar<T>
{
    fn refresh(&self)
    {
	let (_, max_width) = self.widths();
	
	let temp = format!("[{}]: {:.2}%", self.buffer, self.progress * 100.00);
	let title = ensure_lower(format!(" {}", self.title), max_width - temp.chars().count());

	let temp = ensure_eq(format!("{}{}", temp, title), max_width);
	
	// If another thread is writing, just abort (XXX: Is this the best way to handle it?)
	//
	// We acquire the lock after work allocation and computation to keep it for the shortest amount of time, this is an acceptible tradeoff since multiple threads shouldn't be calling this at once anyway.
	let Ok(mut out) = self.output.try_borrow_mut() else { return };
	
	//TODO: What to do about I/O errors?
	let _ = write!(out, "\x1B[0m\x1B[K{}", temp) // XXX: For now, just abort if one fails.
	    .and_then(|_| write!(out, "\n\x1B[1A"))
	    .and_then(move |_| flush!(? out)); 
    }

    fn blank(&self)
    {
	let (_, max_width) = self.widths();

	// If another thread is writing, just abort (XXX: Is this the best way to handle it?)
	let Ok(mut out) = self.output.try_borrow_mut() else { return };
	
	//TODO: What to do about I/O errors?
	let _ = out.write_all(b"\r")
	    .and_then(|_| stackalloc::stackalloc(max_width, b' ',|spaces| out.write_all(spaces))) // Write `max_width` spaces (TODO: Is there a better way to do this? With no allocation? With a repeating iterator maybe?)
	    .and_then(|_| out.write_all(b"\r"))
	    .and_then(move |_| flush!(? out));
    }

    fn get_title(&self) -> &str
    {
	&self.title
    }

    fn set_title(&mut self, from: &str)
    {
	self.title = from.to_string();
	self.refresh();
    }

    fn update_dimensions(&mut self, to: usize)
    {
	self.max_width = to;
	self.refresh();
    }
}

impl<T: ?Sized + io::Write + AsRawFd> ProgressBar for Bar<T>
{
    fn get_progress(&self) -> f64
    {
	self.progress
    }
    fn set_progress(&mut self, value: f64)
    {
	if self.progress != value {
	    self.progress = value;
	    self.update();
	}
	self.refresh();
    }
}

impl<T: io::Write + AsRawFd> WithTitle for Bar<T>
{
    fn add_title(&mut self, string: impl AsRef<str>)
    {
	(*self).add_title(string.as_ref())
    }
    fn update(&mut self)
    {
	self.update();
    }
    fn complete(self)
    {
	//TODO: What to do about I/O errors?
	let _ = self.complete();
    }
}

const _:() = {
    const fn declval<T>() -> Bar<T> {
	unreachable!()
    }
    fn take_title(_: &(impl WithTitle + ?Sized)) {}
    fn take_progress(_: &(impl ProgressBar + ?Sized)) {}
    fn take_display(_: &(impl Display + ?Sized)) {}
    fn test()
    {
	#[macro_export] macro_rules! assert_is_bar {
	    ($ty:path) => {
		take_title(&declval::<$ty>());
		take_progress(&declval::<$ty>());
		take_display(&declval::<$ty>());
	    }
	}

	assert_is_bar!(io::Stdout);
	assert_is_bar!(std::fs::File);
    }
};

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    
    fn rendering_blanking()
    {
	let mut bar = {
	    #[cfg(feature="size")] 
	    let Some(bar)= Bar::try_new_default_size_default() else { return };
	    #[cfg(not(feature="size"))] 
	    let bar= Bar::new_default(50);
	    bar
	};
	bar.set_progress(0.5);
	bar.blank();
	bar.set_progress(0.7);
	bar.set_title("70 percent.");
	bar.refresh();
	//println!();
	bar.set_progress(0.2);
	bar.set_title("...");
	bar.blank();
	bar.complete().unwrap();
    }
}
