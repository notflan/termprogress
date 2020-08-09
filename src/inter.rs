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
pub trait WithTitle: Sized + ProgressBar + Display
{
    fn with_title(len: usize, string: impl AsRef<str>) -> Self;
    fn update(&mut self);
    fn complete(self);
}

