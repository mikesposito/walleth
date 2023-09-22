type Listener<T> = dyn FnMut(&T) -> ();

pub struct Observer<S> {
  pub id: usize,
  pub callback: Box<Listener<S>>,
}

impl<S> Observer<S> {
  pub fn new(id: usize, callback: Box<Listener<S>>) -> Self {
    Observer {
      id,
      callback: callback,
    }
  }
}
