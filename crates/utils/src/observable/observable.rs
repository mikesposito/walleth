use std::sync::{Arc, Mutex};

use super::{ObservableError, Observer};

/// A store for state that can be subscribed to
#[derive(Debug, Clone)]
pub struct Observable<S> {
  state: S,
  observers: Vec<Observer<S>>,
}

impl<S> Observable<S>
where
  S: Clone,
{
  pub fn new(initial_state: S) -> Self {
    Observable {
      state: initial_state,
      observers: vec![],
    }
  }

  /// Get the current state
  pub fn get_state(&self) -> &S {
    &self.state
  }

  /// Set the current state
  /// This will call all event listeners with the new state
  pub fn set_state(&mut self, new_state: S) -> Result<(), ObservableError> {
    self.state = new_state;
    self.emit()
  }

  /// Update the current state
  /// This will call all event listeners with the new state
  /// The updater function will be called with a mutable reference to the current state
  /// allowing you to mutate the state
  pub fn update<F>(&mut self, updater: F) -> Result<(), ObservableError>
  where
    F: Fn(&mut S),
  {
    updater(&mut self.state);
    self.emit()
  }

  /// Subscribe to state changes
  /// Returns the id of the subscriber
  pub fn subscribe<F>(&mut self, subscriber: F) -> usize
  where
    F: 'static + FnMut(&S),
  {
    self.observers.push(Observer::new(
      self.observers.len(),
      Arc::new(Mutex::new(subscriber)),
    ));
    self.observers.len() - 1
  }

  /// Unsubscribe from state changes
  pub fn unsubscribe(&mut self, id: usize) {
    self.observers.retain(|observer| observer.id != id);
  }

  /// Emit the current state to all subscribers
  fn emit(&mut self) -> Result<(), ObservableError> {
    for observer in &mut self.observers {
      let mutex = Arc::clone(&observer.callback);

      let mut guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ObservableError::UnableToLockObserver),
      };

      (guard)(&self.state);
    }

    Ok(())
  }
}
