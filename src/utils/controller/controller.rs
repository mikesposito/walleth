/// A controller is a struct that holds a state and allows for updates to that state.
pub trait Controller<State> {
	/// Get the current state
	fn get_state(&self) -> &State;

	/// Update the current state
	/// The updater function will be called with a mutable reference to the current state
	fn update<F>(&mut self, updater: F) -> ()
	where
		F: Fn(&mut State) -> ();

	/// Subscribe to state changes
	fn subscribe<F>(&mut self, subscriber: F) -> usize
	where
		F: 'static + FnMut(&State);

	/// Unsubscribe from state changes
	fn unsubscribe(&mut self, id: usize) -> ();
}
