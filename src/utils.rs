//! Utility functions for the rust-multiplatform framework

use crate::traits::{AppBuilder, RmpAppModel, RmpViewModel};
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::thread;

/// Set up a listener for model updates
///
/// This function creates a thread that listens for model updates and forwards them to the view model.
/// It's used by the framework's generated code to handle the boilerplate of setting up the listener.
pub fn listen_for_model_updates<M, V>(model: &M, view_model: Box<V>)
where
    M: RmpAppModel,
    V: RmpViewModel<UpdateType = M::UpdateType> + ?Sized + 'static,
    M::UpdateType: Send + 'static,
{
    // Get the receiver from the model
    let model_update_rx = model.get_update_receiver().clone();

    // Spawn a thread to listen for updates
    thread::spawn(move || {
        while let Ok(update) = model_update_rx.recv() {
            view_model.model_update(update);
        }
    });
}

/// Create a new channel for model updates
///
/// This function creates a channel for model updates and returns the sender and receiver.
/// The sender should be passed to the ViewModel and the receiver should be used to create the AppBuilder.
pub fn create_model_update_channel<T>() -> (Sender<T>, Receiver<T>) {
    unbounded()
}

/// Create a new app builder with a receiver for model updates
///
/// This is a convenience function to create an AppBuilder with a new receiver.
pub fn create_app_builder<M, U>(data_dir: String, receiver: Receiver<U>) -> AppBuilder<M, U>
where
    M: RmpAppModel<UpdateType = U>,
{
    AppBuilder::new(data_dir, receiver)
}
