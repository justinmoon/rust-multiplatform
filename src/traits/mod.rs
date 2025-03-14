//! Traits for application models to implement
//!
//! These traits define the interface between the user's app code and the framework.

use crossbeam::channel::Receiver;
use std::sync::Arc;

/// Trait for view models that can receive updates from the model
///
/// This trait is implemented by the framework's generated code and used as a callback
/// interface for sending updates from the model to the view.
pub trait RmpViewModel: Send + Sync + 'static {
    /// The type of updates that can be received from the model
    type UpdateType;

    /// Handle a model update
    ///
    /// This function is called when the model has an update to send to the view.
    fn model_update(&self, model_update: Self::UpdateType);
}

/// Trait for application models that can be managed by the framework
///
/// By implementing this trait, a model can be initialized and managed by the framework,
/// with automatic integration into the FFI layer.
pub trait RmpAppModel {
    /// The type of actions that can be dispatched to the model
    type ActionType: std::fmt::Debug;

    /// The type of updates that can be sent from the model to the view
    type UpdateType: std::fmt::Debug;

    /// Create a new instance of the model
    ///
    /// This function is called by the framework to initialize the model.
    /// The `data_dir` parameter provides a location for storing app data.
    fn create(data_dir: String) -> Self;

    /// Handle an action dispatched to the model
    ///
    /// This function is called when an action is dispatched from the frontend.
    fn action(&mut self, action: Self::ActionType);

    /// Get access to the model update receiver
    ///
    /// This is used by the framework to access the receiver for model updates.
    /// App developers should not need to implement this method manually as
    /// the app builder helper will implement it.
    fn get_update_receiver(&self) -> Arc<Receiver<Self::UpdateType>>;
}

/// Helper for model implementations
///
/// This struct helps app developers implement the RmpAppModel trait with
/// a default implementation for update receivers.
pub struct AppBuilder<T, U> {
    /// The model update receiver
    pub model_update_rx: Arc<Receiver<U>>,

    /// The data directory
    pub data_dir: String,

    /// Phantom data to tie the generic parameter to the model type
    _phantom: std::marker::PhantomData<T>,
}

impl<T, U> AppBuilder<T, U> {
    /// Create a new app builder with a receiver for model updates
    pub fn new(data_dir: String, model_update_rx: Receiver<U>) -> Self {
        Self {
            model_update_rx: Arc::new(model_update_rx),
            data_dir,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Trait for models that use the AppBuilder
///
/// This trait is implemented by models that contain an AppBuilder field,
/// providing access to the update receiver.
pub trait BuildableApp<U>: RmpAppModel<UpdateType = U> + Sized {
    /// Get the AppBuilder from the model
    fn builder(&self) -> &AppBuilder<Self, U>;
}

// Helper macro to implement BuildableApp for a model
#[macro_export]
macro_rules! impl_buildable_app {
    ($Model:ty, $UpdateType:ty, $builder_field:ident) => {
        impl $crate::traits::BuildableApp<$UpdateType> for $Model {
            fn builder(&self) -> &$crate::traits::AppBuilder<Self, $UpdateType> {
                &self.$builder_field
            }
        }
    };
}
