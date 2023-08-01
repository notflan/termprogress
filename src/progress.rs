///! Progress bar that has a size and also a max size.

use super::*;
use std::{
    fmt::Write,
    io,
};

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
#[derive(Debug)]
pub struct Bar/*<T: ?Sized = io::Stdout> //TODO: Implement this after try_new(), WINCH, etc*/
{
    width: usize,
    max_width: usize,
    progress: f64,
    buffer: String,
    title: String,
    #[cfg(feature="size")]
    fit_to_term: bool,
    //output_to: T
}

/// The default size of the terminal bar when the programmer does not provide her own.
/// Or if `size` is not used.
pub const DEFAULT_SIZE: usize = 50;

/// The default size of the max render bar when the programmer does not provide her own.
/// Or if `size` is not used.
pub const DEFAULT_MAX_BORDER_SIZE: usize = 20;

impl/*<T: Default>*/ Default for Bar/*<T>*/
{
    #[inline] 
    fn default() -> Self
    {
	Self::new(DEFAULT_SIZE)
    }
}

impl/*<T: io::Write>*/ Bar/*<T>*/
{
    /// Create a new bar `width` long with a title.
    pub fn with_title(width: usize, title: impl AsRef<str>) -> Self
    {
	let mut this = Self::new(width);
	this.set_title(title.as_ref());
	this.update();
	this
    }
    
    /// Attempt to create a new bar with max display width of our terminal and a title.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    pub fn try_new_with_title(width: usize, title: impl AsRef<str>) -> Option<Self>
    {
	let (terminal_size::Width(tw), _) = terminal_size::terminal_size()?;
	let tw = usize::from(tw);
	let mut o = Self::with_max(if width < tw {width} else {tw}, tw);
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
    pub fn new(width: usize) -> Self
    {
	#[cfg(feature="size")]
	return {
	    if let Some((terminal_size::Width(tw), _)) = terminal_size::terminal_size() {
		let tw = usize::from(tw);
		let mut o = Self::with_max(if width < tw {width} else {tw}, tw);
		o.fit_to_term = true;
		o
	    } else {
		let mut o = Self::with_max(width, width + DEFAULT_MAX_BORDER_SIZE);
		o.fit_to_term = true;
		o
	    }
	};
	#[cfg(not(feature="size"))]
	return {
	    Self::with_max(width, width +DEFAULT_MAX_BORDER_SIZE)
	};
    }

    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    pub fn try_new(width: usize) -> Option<Self>
    {
	let (terminal_size::Width(tw), _) = terminal_size::terminal_size()?;
	let tw = usize::from(tw);
	let mut o = Self::with_max(if width < tw {width} else {tw}, tw);
	o.fit_to_term = true;
	Some(o)
    }
    
    /// Attempt to create a new bar with max display width of our terminal.
    ///
    /// If `stdout` is not a terminal, then `None` is returned.
    #[cfg(feature="size")]
    #[inline] 
    pub fn try_new_default_size() -> Option<Self>
    {
	Self::try_new(DEFAULT_SIZE)
    }
    
    /// Create a bar with a max display width
    ///
    /// # Panics
    /// If `width` is larger than or equal to `max_width`.
    pub fn with_max(width: usize, max_width: usize) -> Self
    {
	let mut this = Self {
	    width,
	    max_width,
	    progress: 0.0,
	    buffer: String::with_capacity(width),
	    title: String::with_capacity(max_width - width),
	    #[cfg(feature="size")] 
	    fit_to_term: false,
	    /*output_to: io::stdout(),*/
	};
	this.update();
	this
    }

    /// Fit to terminal's width if possible.
    ///
    /// # Notes
    /// Available with feature `size` only.
    pub fn fit(&mut self) -> bool
    {
	#[cfg(feature="size")]
	if let Some((terminal_size::Width(tw), _)) = terminal_size::terminal_size() {
	    let tw = usize::from(tw);
	    self.width = if self.width < tw {self.width} else {tw};
	    self.update_dimensions(tw);
	    return true;
	}
	false
    }

    #[inline] fn widths(&self) -> (usize, usize)
    {
	#[cfg(feature="size")] 
	if self.fit_to_term {
	    if let Some((terminal_size::Width(tw), _)) = terminal_size::terminal_size() {
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

    /// Consume the bar and complete it, regardless of progress.
    pub fn complete(self)
    {
	println!("");
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

impl Display for Bar
{
    fn refresh(&self)
    {
	let (_, max_width) = self.widths();
	let temp = format!("[{}]: {:.2}%", self.buffer, self.progress * 100.00);
	let title = ensure_lower(format!(" {}", self.title), max_width - temp.chars().count());

	let temp = ensure_eq(format!("{}{}", temp, title), max_width);
	print!("\x1B[0m\x1B[K{}", temp);
	print!("\n\x1B[1A");
	flush!();
    }

    fn blank(&self)
    {
	let (_, max_width) = self.widths();
	print!("\r{}\r", " ".repeat(max_width));
	flush!();
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

impl ProgressBar for Bar
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

impl WithTitle for Bar
{
    fn with_title(len: usize, string: impl AsRef<str>) -> Self
    {
	Self::with_title(len, string)
    }
    fn update(&mut self)
    {
	self.update();
    }
    fn complete(self)
    {
	self.complete();
    }
}
