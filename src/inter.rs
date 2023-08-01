/// A trait for all bars' displaying
pub trait Display
{
    /// Refresh the display
    fn refresh(&self);
    /// Blank the display
    fn blank(&self);
    /// Blank then print a line, and redisplay.
    fn println(&self, string: &str)
    {
	self.blank();
	println!("{}", string);
	self.refresh();
    }
    /// Blank then print a line std stderr, and redisplay.
    fn eprintln(&self, string: &str)
    {
	self.blank();
	eprintln!("{}", string);
	self.refresh();
    }

    /// Get the title for this display
    fn get_title(&self) -> &str;
    /// Set the title for this display
    fn set_title(&mut self, from: &str);

    /// Update the max size if needed
    fn update_dimensions(&mut self, to: usize);
}

/// A trait for any bar with progress. You can implemnent your own styles through this trait.
pub trait ProgressBar: Display
{
    fn set_progress(&mut self, value: f64);
    fn get_progress(&self) -> f64;
}

/// A trait for any bar without progress. You can implemnent your own styles through this trait.
pub trait Spinner: Display
{
    /// Cause the spinner to increment once.
    fn bump(&mut self);
}

/// A trait for creating a progress bar or spinner with a title.
pub trait WithTitle: Sized + Display
{
    /// Add a title to this indicator.
    #[inline] 
    fn with_title(mut self, string: impl AsRef<str>) -> Self
    {
	self.add_title(string);
	self
    }
    
    /// Add a title to this indicator.
    fn add_title(&mut self, string: impl AsRef<str>);
    
    fn update(&mut self);
    fn complete(self);
}

impl<T> WithTitle for Box<T>
where T: WithTitle + ?Sized
{
    /*fn with_title(len: usize, string: impl AsRef<str>) -> Self
    {
    Box::new(T::with_title(len, string))
}*/
    
    fn add_title(&mut self, string: impl AsRef<str>) {
	self.as_mut().add_title(string.as_ref())
    }
    fn update(&mut self)
    {
	self.as_mut().update()
    }
    fn complete(self)
    {
	(*self).complete()
    }
}

impl<T> Display for Box<T>
where T: Display + ?Sized
{
    #[inline] fn refresh(&self)
    {
	self.as_ref().refresh();
    }
    #[inline] fn blank(&self)
    {
	self.as_ref().blank();
    }
    #[inline] fn println(&self, string: &str)
    {
	self.as_ref().println(string);
    }
    #[inline] fn eprintln(&self, string: &str)
    {
	self.as_ref().eprintln(string);
    }
    #[inline] fn get_title(&self) -> &str
    {
	self.as_ref().get_title()
    }
    #[inline] fn set_title(&mut self, from: &str)
    {
	self.as_mut().set_title(from);
    }
    #[inline] fn update_dimensions(&mut self, to: usize)
    {
	self.as_mut().update_dimensions(to);
    }
}


impl<T> ProgressBar for Box<T>
where T: ProgressBar + ?Sized
{
    #[inline] fn set_progress(&mut self, value: f64)
    {
	self.as_mut().set_progress(value)
    }
    #[inline] fn get_progress(&self) -> f64
    {
	self.as_ref().get_progress()
    }
}

impl<T> Spinner for Box<T>
where T: Spinner + ?Sized
{
    #[inline] fn bump(&mut self)
    {
	self.as_mut().bump()
    }
}

#[cfg(nightly)] mod never
{
    use super::*;

    impl Display for !
    {
	#[inline] fn refresh(&self)
	{
	    
	}
	#[inline] fn blank(&self)
	{

	}
	#[inline] fn println(&self, _: &str)
	{
	    
	}
	#[inline] fn eprintln(&self, _: &str)
	{
	    
	}
	#[inline] fn get_title(&self) -> &str
	{
	    *self
	}
	#[inline] fn set_title(&mut self, _: &str)
	{
	}

	#[inline] fn update_dimensions(&mut self, _: usize)
	{
	    
	}
    }

    impl ProgressBar for !
    {
	
	#[inline] fn set_progress(&mut self, _: f64)
	{
	    
	}
	#[inline] fn get_progress(&self) -> f64
	{
	    *self
	}
    }

    impl Spinner for !
    {
	#[inline] fn bump(&mut self){}
    }

    impl WithTitle for !
    {
	#[inline] fn with_title(self, _: impl AsRef<str>) -> Self
	{
	    unreachable!()
	}
	#[inline] fn add_title(&mut self, _: impl AsRef<str>)
	{
	    
	}
	#[inline] fn update(&mut self)
	{
	    
	}
	#[inline] fn complete(self)
	{
	    
	}
    }
}
#[cfg(nightly)] pub use never::*;
