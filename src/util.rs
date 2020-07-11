/// Copy slice `src` to `dst`, return the number of elements copied.
#[inline]
pub fn copy_slice<T,U,V>(mut dst: U, src: V) -> usize
where T: Clone,
      U: AsMut<[T]>,
      V: AsRef<[T]>
{
    let mut i=0;
    for (d,s) in dst.as_mut().iter_mut().zip(src.as_ref().iter())
    {
	*d = s.clone();
	i+=1;
    }
    i
}

