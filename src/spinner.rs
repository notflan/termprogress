//! A simple character spinner for bars with no known size

use super::*;

/// A single character spinner with optional title that can be told to spin whenever it wants. It implements `Spinner` trait, and is the default spinner.
///
/// The spinner takes up a line, and renders it's spinner on the end of its title. Calling the `Spinner::bump()` function will cause the character sequence to advance.
///
/// # Usage
/// To use the spinner you can provide it a `Wheel` (see [[wheel]] module), or it implements the `Default` trait, creating a traditional looking spinner (`|/-\`)
///
/// ```rust
/// let mut spin = Spin::default(); //Default new spinner without a title.
/// ```
///
/// # How it looks
/// It renders in the terminal like:
/// `This is a spinner /`
pub struct Spin
{
    title: String,
    current: char,
    chars: wheel::WheelIntoIter,
}

impl Spin
{
    /// Create a new spinner with title and wheel.
    ///
    /// To give it the default wheel, you can pass `whl` `Default::default()` to use the default one.
    pub fn with_title(title: &str, whl: wheel::Wheel) -> Self
    {
	let mut chars = whl.into_iter();
	let current = chars.next().unwrap();
	Self {
	    title: title.to_string(),
	    current,
	    chars,
	}
    }
    /// Create a new blank spinner with a wheel
    ///
    /// # Example
    /// ```rust
    ///  Spin::new(Default::default()); // Create a spinner with the default wheel ('|/-\')
    /// ```
    pub fn new(whl: wheel::Wheel) -> Self
    {
	let mut chars = whl.into_iter();
	let current = chars.next().unwrap();
	Self {
	    title: String::new(),
	    current,
	    chars,
	}
    }

    /// Consume the spinner and complete it. Removes the spin character.
    pub fn complete(self) {
	println!("{} ", (8u8 as char));
    }
    
    /// Consume the spinner and complete it with a message. Removes the spin character and then prints the message.
    pub fn complete_with(self, msg: &str)
    {
	println!("{}{}", (8u8 as char), msg);
    }
}

impl Default for Spin
{
    fn default() -> Self
    {
	Self {
	    title: String::new(),
	    chars: wheel::Wheel::default().into_iter(),
	    current: '|',
	}
    }
}

impl Display for Spin
{
    fn refresh(&self)
    {
	print!("\r{} {}", self.title, self.current);
	flush!();
    }
    fn blank(&self)
    {
	print!("\r{}  \r", " ".repeat(self.title.chars().count()));
	flush!();
    }
    fn get_title(&self) -> &str
    {
	&self.title[..]
    }
    fn set_title(&mut self, from: &str)
    {
	self.blank();
	self.title = from.to_string();
	self.refresh();
    }
    fn update_dimensions(&mut self, _to:usize){}

    fn println(&self, string: &str)
    {
	self.blank();
	println!("{}", string);
	self.refresh();
    }
}

impl Spinner for Spin
{
    fn bump(&mut self)
    {
	self.current = self.chars.next().unwrap();
	self.refresh();
    }
}


impl WithTitle for Spin
{
    fn with_title(_: usize, t: impl AsRef<str>) -> Self
    {
	Self::with_title(t.as_ref(), Default::default())
    }
    #[inline] fn update(&mut self){}
    #[inline] fn complete(self)
    {
	Spin::complete(self)
    }
}
