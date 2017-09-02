include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
pub struct Glw {
    inner: Rc<Gl>,
}

impl Glw {
    pub fn new(gl: Gl) -> Glw {
        Glw {
            inner: Rc::new(gl),
        }
    }
}

impl Deref for Glw {
    type Target = Gl;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}