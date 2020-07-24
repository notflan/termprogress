//! Progress bar that has a size and also a max size.

use super::*;
use std::{
    fmt::Write,
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
/// let mut bar = Bar::default(); //Creates a bar of width 50 by default.
/// ```
///
/// You can configure sizes and initial title with `new()`, `with_title()`, and `with_max()` functions.
/// # How it looks
/// It renders in the terminal like:
/// `[=========================                         ]: 50% this is a title that may get cut if it reaches max le...`
#[derive(Debug)]
pub struct Bar
{
    width: usize,
    max_width: usize,
    progress: f64,
    buffer: String,
    title: String,
}

impl Default for Bar
{
    fn default() -> Self
    {
	Self::new(50)
    }
}

impl Bar
{
    /// Create a new bar `width` long with a title.
    pub fn with_title(width: usize, title: impl AsRef<str>) -> Self
    {
	let mut this = Self::new(width);
	this.set_title(title.as_ref());
	this.update();
	this
    }
    /// Create a new bar with max display width of our terminal
    pub fn new(width: usize) -> Self
    {
	if let Some((terminal_size::Width(tw), _)) = terminal_size::terminal_size() {
	    let tw = usize::from(tw);
	    Self::with_max(if width < tw {width} else {tw}, tw)
	} else {
	    Self::with_max(width, width +20)
	}
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
	};
	this.update();
	this
    }

    /// Fit to terminal's width if possible.
    pub fn fit(&mut self) -> bool
    {
	
	if let Some((terminal_size::Width(tw), _)) = terminal_size::terminal_size() {
	    let tw = usize::from(tw);
	    self.width = if self.width < tw {self.width} else {tw};
	    self.update_dimensions(tw);
	    return true;
	}
	false
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
	let temp = format!("[{}]: {:.2}%", self.buffer, self.progress * 100.00);
	let title = ensure_lower(format!(" {}", self.title), self.max_width - temp.chars().count());

	let temp = ensure_eq(format!("{}{}", temp, title), self.max_width);
	print!("\x1B[0m\x1B[K{}", temp);
	print!("\n\x1B[1A");
	flush!();
    }

    fn blank(&self)
    {
	print!("\r{}\r", " ".repeat(self.max_width));
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
