use std::sync::{Arc, Mutex};

use walleth::Observable;

#[test]
fn it_creates_emitter_store() {
	let store = Observable::new(0);
	assert_eq!(store.get_state(), &0);
}

#[test]
fn it_sets_the_state() {
	let mut store = Observable::new(0);
	store.set_state(1);
	assert_eq!(store.get_state(), &1);
}

#[test]
fn it_calls_subscriber_callback_when_setting_state() {
	let mut store = Observable::new(0);
	let spy = Arc::new(Mutex::<Vec<i32>>::new(vec![]));

	let r_spy = spy.clone();
	store.subscribe(move |state| {
		r_spy.clone().lock().unwrap().push(state.clone());
	});
	store.set_state(1);

	assert_eq!(spy.lock().unwrap()[0], 1);
}

#[test]
fn it_calls_subscriber_callback_everytime_when_setting_state() {
	let mut store = Observable::new(0);
	let history = Arc::new(Mutex::<Vec<i32>>::new(vec![]));

	let r_history = history.clone();
	store.subscribe(move |state| {
		r_history.lock().unwrap().push(state.clone());
	});
	store.set_state(1);
	store.set_state(2);
	store.set_state(3);

	let locked_history = history.lock().unwrap();
	assert_eq!(locked_history.len(), 3);
	assert_eq!(locked_history[0], 1);
	assert_eq!(locked_history[1], 2);
	assert_eq!(locked_history[2], 3);
}

#[test]
fn it_stops_calling_callback_after_unsubscribe() {
	let mut store = Observable::new(0);
	let history = Arc::new(Mutex::<Vec<i32>>::new(vec![]));
	let r_history = history.clone();
	let id = store.subscribe(move |state| {
		r_history.lock().unwrap().push(state.clone());
	});
	store.set_state(1);

	store.unsubscribe(id);
	store.set_state(2);

	assert_eq!(history.lock().unwrap().len(), 1);
}
