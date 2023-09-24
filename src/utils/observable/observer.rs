use std::sync::{Arc, Mutex};

type Listener<T> = dyn FnMut(&T) -> ();

#[derive(Clone)]
pub struct Observer<S> {
  pub id: usize,
  pub callback: Arc<Mutex<Listener<S>>>,
}

impl<S> Observer<S> {
  pub fn new(id: usize, callback: Arc<Mutex<Listener<S>>>) -> Self {
    Observer {
      id,
      callback: callback,
    }
  }
}
