use crate::Plugin;

pub type Plugins<T, U> = Vec<Box<dyn Plugin<T, U>>>;
