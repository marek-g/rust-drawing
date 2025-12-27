use crate::Path;

pub trait PathObject {}

impl<P: Path> PathObject for P {}
