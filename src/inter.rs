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
    fn bump(&mut self);
}
