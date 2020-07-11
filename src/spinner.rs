use super::*;

pub struct Spin
{
    title: String,
    current: char,
    chars: wheel::WheelIntoIter,
}

impl Spin
{
    /// Create a new spinner with title
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
    /// Create a new blank spinner
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

    /// Consume the spinner and complete it
    pub fn complete(self) {
	println!("{} ", (8u8 as char));
    }
    
    /// Consume the spinner and complete it with a message
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

/*
#[cfg(test)]
mod tests
{
    #[test]
    fn test()
    {
	const MAX: usize = 1000000;
	let mut spin = Spin::with_title("Working maybe...", Default::default());
	for i in 0..=MAX
	{
	    spin.bump();
	    if i == MAX / 2 {
		spin.set_title("Working more...");
	    }
	}
	spin.complete_with("OK");
	println!("Oke");
    }
}*/

impl Display for Spin
{
    fn refresh(&self)
    {
	print!("\r{} {}", self.title, self.current);
    }
    fn blank(&self)
    {
	print!("\r{}  \r", " ".repeat(self.title.chars().count()));
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
}

impl Spinner for Spin
{
    fn bump(&mut self)
    {
	self.current = self.chars.next().unwrap();
	self.refresh();
    }
}
