use crate::prelude::*;
pub trait Template: for<'a> Deserialize<'a> + Serialize + Default + ToNote {}
impl<T> Template for T where T: for<'a> Deserialize<'a> + Serialize + Default + ToNote {}
