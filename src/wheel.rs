//! Contains infinite `char` iterators and the type wrapping them, `Wheel`.

/// An infinite repeating series of `char`. Used for `spinner::Spin`.
///
/// It implements the `IntoIterator` trait, yielding an infinite iterator of a repeating series of `char`s.
#[derive(Clone,Debug)]
pub enum Wheel
{
    Static(&'static [char]),
    Dynamic(Box<[char]>),
}

impl Wheel
{
    /// Create a new spinner out of an interator of `char`.
    pub fn new<T>(iter: T) -> Self
    where T: IntoIterator<Item=char>
    {
	let col: Vec<char> = iter.into_iter().collect();
	Self::Dynamic(col.into_boxed_slice())
    }

    /// Get a reference to all the possible chars of this wheel.
    pub fn chars(&self) -> &[char]
    {
	match &self
	{
	    Wheel::Static(s) => s,
	    Wheel::Dynamic(b) => &b[..],
	}
    }
}

impl Default for Wheel
{
    fn default() -> Self
    {
	DEFAULT_WHEEL.clone()
    }
}

const DEFAULT_WHEEL: Wheel = Wheel::Static(&['/', '-', '\\', '|']);

/// An infinite iterator of a repeating series of `char`s, used for `spinner::Spin`
#[derive(Debug, Clone)]
pub struct WheelIntoIter
{
    source: Wheel,
    idx: usize,
}

impl Iterator for WheelIntoIter
{
    type Item = char;
    fn next(&mut self) -> Option<char>
    {
	let chars = self.source.chars();
	self.idx = (self.idx + 1) % chars.len();
	Some(chars[self.idx])
    }
}

impl IntoIterator for Wheel
{
    type IntoIter= WheelIntoIter;
    type Item = char;
    fn into_iter(self) -> Self::IntoIter
    {
	WheelIntoIter{
	    source: self,
	    idx: 0,
	}
    }
}
