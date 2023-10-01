use std::{
  fmt::{Debug, Formatter, Result},
  sync::{Arc, Mutex},
};

type Listener<T> = dyn FnMut(&T);

#[derive(Clone)]
pub struct Observer<S> {
  pub id: usize,
  pub callback: Arc<Mutex<Listener<S>>>,
}

impl<S> Observer<S> {
  pub fn new(id: usize, callback: Arc<Mutex<Listener<S>>>) -> Self {
    Observer { id, callback }
  }
}

impl<S> Debug for Observer<S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Observer {{ id: {} }}", self.id)
  }
}
