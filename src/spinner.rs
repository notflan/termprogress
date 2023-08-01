//! A simple character spinner for bars with no known size

use super::*;
use std::io;

/// A single character spinner with optional title that can be told to spin whenever it wants. It implements `Spinner` trait, and is the default spinner.
///
/// The spinner takes up a line, and renders it's spinner on the end of its title. Calling the `Spinner::bump()` function will cause the character sequence to advance.
///
/// # Usage
/// To use the spinner you can provide it a `Wheel` (see [[wheel]] module), or it implements the `Default` trait, creating a traditional looking spinner (`|/-\`)
///
/// ```rust
/// # use termprogress::prelude::*;
/// let mut spin = Spin::default(); //Default new spinner without a title.
/// ```
///
/// # How it looks
/// It renders in the terminal like:
/// `This is a spinner /`
///
/// # Thread `Sync`safety
/// This type is safely `Sync` (where `T` is), the behaviour is defined to prevent overlapping writes to `T`.
/// Though it is *advised* to not render a `Spin` from more than a single thread, you still safely can.
///
/// A display operation on one thread will cause any other threads attempting on to silently and safely abort their display attempt before anything is written to output.
pub struct Spin<T: ?Sized = DefaultOutputDevice>/*<T: ?Sized = DefaultOutputDevice>*/ //TODO: <- implement same as `Bar
{
    title: String,
    current: char,
    chars: wheel::WheelIntoIter,
    output: AtomicRefCell<T>,
}

impl Spin
{
    /// Create a new spinner with title and wheel writing to `stdout`.
    ///
    /// To give it the default wheel, you can pass `whl` `Default::default()` to use the default one.
    #[inline] 
    pub fn with_title_default(title: &str, whl: wheel::Wheel) -> Self
    {
	Self::with_title(create_default_output_device(), title, whl)
    }
    
    /// Create a new blank spinner with a wheel writing to `stdout`.
    ///
    /// # Example
    /// ```rust
    /// # use termprogress::prelude::*;
    ///  Spin::new_default(Default::default()); // Create a spinner with the default wheel ('|/-\') that writes to stdout.
    /// ```
    #[inline] 
    pub fn new_default(whl: wheel::Wheel) -> Self
    {
	Self::new(create_default_output_device(), whl)
    }
}

impl<T> Spin<T>
{
    /// Return the backing write object
    #[inline] 
    pub fn into_inner(self) -> T
    {
	self.output.into_inner()
    }
}

impl<T: ?Sized> Spin<T>
{
    /// Get a mutable reference to the inner object
    #[inline] 
    pub fn inner_mut(&mut self) -> &mut T
    {
	self.output.get_mut()
    }
    /// Get a shared reference to the inner object
    ///
    /// # Returns
    /// `None` is returned if a display operation is currently in progress on another thread.
    ///
    /// ## Operation suspension
    /// As long as the returned reference lives, **all** display operations will fail silently, on this thread and any other. This method should be avoided in favour of `&*inner_mut()` when exclusive ownership is avaliable.
    #[inline] 
    pub fn inner(&self) -> Option<atomic_refcell::AtomicRef<'_, T>>
    {
	self.output.try_borrow().ok()
    }
}

impl<T: io::Write> Spin<T>
{
    /// Create a new spinner with title and wheel writing to `output`.
    ///
    /// To give it the default wheel, you can pass `whl` `Default::default()` to use the default one.
    pub fn with_title(output: T, title: &str, whl: wheel::Wheel) -> Self
    {
	let mut chars = whl.into_iter();
	let current = chars.next().unwrap();
	Self {
	    title: title.to_string(),
	    current,
	    chars,
	    output: AtomicRefCell::new(output)
	}
    }
    
    /// Create a new blank spinner with a wheel writing to `output`.
    ///
    /// # Example
    /// ```rust
    /// # use termprogress::prelude::*;
    ///  Spin::new(std::io::stdout(), Default::default()); // Create a spinner with the default wheel ('|/-\') that writes to stdout.
    /// ```
    pub fn new(output: T, whl: wheel::Wheel) -> Self
    {
	let mut chars = whl.into_iter();
	let current = chars.next().unwrap();
	Self {
	    title: String::new(),
	    current,
	    chars,
	    output: output.into()
	}
    }

    /// Consume the spinner and complete it. Removes the spin character.
    pub fn complete(self) -> io::Result<()> {
	let mut output = self.output.into_inner();
	writeln!(&mut output, "{} ", (8u8 as char))
    }
    
    /// Consume the spinner and complete it with a message. Removes the spin character and then prints the message.
    pub fn complete_with(self, msg: &str) -> io::Result<()>
    {
	let mut output = self.output.into_inner();
	writeln!(&mut output, "{}{}", (8u8 as char), msg)
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
	    output: AtomicRefCell::new(create_default_output_device())
	}
    }
}

impl<T: ?Sized + io::Write> Display for Spin<T>
{
    fn refresh(&self)
    {
	let Ok(mut output) = self.output.try_borrow_mut() else { return };
	
	//TODO: What to do about I/O errors?
	let _ = write!(&mut output, "\r{} {}", self.title, self.current)
	    .and_then(move |_| flush!(? output));
    }
    fn blank(&self)
    {
	let Ok(mut output) = self.output.try_borrow_mut() else { return };
	
	//TODO: What to do about I/O errors?
	let _ = output.write_all(b"\r")
	    .and_then(|_|
		      stackalloc::stackalloc(self.title.chars().count(), b' ',
					     |spaces| output.write_all(spaces)))
	    .and_then(|_| write!(&mut output, "  \r"))
	    .and_then(move |_| flush!(? output));
    }
    fn get_title(&self) -> &str
    {
	&self.title[..]
    }
    fn set_title(&mut self, from: &str)
    {
	//self.blank(), with exclusive access
	let mut output = self.output.get_mut();

	let size = self.title.chars().count();
	let _ = output.write_all(b"\r")
	    .and_then(|_|
		      stackalloc::stackalloc(size, b' ',
					     |spaces| output.write_all(spaces)))
	    .and_then(|_| write!(&mut output, "  \r"))
	    .and_then(|_| flush!(? output));
	
	self.title = from.to_string();
	
	//self.refresh(), with exclusive access
	let _ = write!(&mut output, "\r{} {}", self.title, self.current)
	    .and_then(move |_| flush!(? output));
    }
    fn update_dimensions(&mut self, _:usize){}

    fn println(&self, string: &str)
    {
	self.blank();
	if let Ok(mut output) = self.output.try_borrow_mut() {
	    
	    //TODO: What to do about I/O errors?
	    let _ = writeln!(&mut output, "{}", string);
	    drop(output)
	} else {
	    return;
	}
	self.refresh();
    }
}

impl<T: ?Sized + io::Write> Spinner for Spin<T>
{
    fn bump(&mut self)
    {
	self.current = self.chars.next().unwrap();
	let mut output = self.output.get_mut();
	
	let _ = write!(&mut output, "\r{} {}", self.title, self.current)
	    .and_then(move |_| flush!(? output));
    }
}


impl<T: io::Write> WithTitle for Spin<T>
{
    #[inline] 
    fn with_title(self, t: impl AsRef<str>) -> Self
    {
	Self {
	    title: t.as_ref().to_owned(),
	    ..self
	}
    }
    #[inline] 
    fn add_title(&mut self, t: impl AsRef<str>)
    {
	self.title = t.as_ref().to_owned();
	//	Self::with_title(t.as_ref(), Default::default())
    }
    #[inline] fn update(&mut self){}
    #[inline] fn complete(self)
    {
	//TODO: What to do about I/O errors?
	let _ = Spin::complete(self);
    }
}
