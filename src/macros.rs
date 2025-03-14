//! Macros for automatic generation of FFI and cross-platform code

/// Registers an application's core components with the framework
///
/// This macro generates all the necessary boilerplate code for FFI integration,
/// including global statics, FFI objects with uniffi annotations, and callback interfaces.
///
/// # Example
///
/// ```ignore
/// use rust_multiplatform::register_app;
///
/// // First define your model, view model, action, and update types
/// struct Model { /* ... */ }
/// struct ViewModel(/* ... */);
/// enum Action { /* ... */ }
/// enum ModelUpdate { /* ... */ }
///
/// // Then register your app
/// register_app!(Model, ViewModel, Action, ModelUpdate);
/// ```
#[macro_export]
macro_rules! register_app {
    ($Model:ident, $ViewModel:ident, $Action:ident, $ModelUpdate:ident) => {
        // 1. Global static definitions for model and view model
        static GLOBAL_MODEL: $crate::once_cell::sync::OnceCell<std::sync::RwLock<$Model>> =
            $crate::once_cell::sync::OnceCell::new();

        static GLOBAL_VIEW_MODEL: $crate::once_cell::sync::OnceCell<$ViewModel> =
            $crate::once_cell::sync::OnceCell::new();

        // 2. Define the RmpViewModel trait first
        // In regular usage, mark with uniffi callback_interface
        // #[cfg(not(test))]
        #[::uniffi::export(callback_interface)]
        pub trait RmpViewModel: Send + Sync + 'static {
            fn model_update(&self, model_update: $ModelUpdate);
        }

        // In test mode, just define the trait without the uniffi attributes
        // #[cfg(test)]
        // pub trait RmpViewModel: Send + Sync + 'static {
        //     fn model_update(&self, model_update: $ModelUpdate);
        // }

        // 3. Define a wrapper struct for FFI
        // #[cfg(not(test))]
        #[derive(uniffi::Object)]
        pub struct RmpModel {
            pub data_dir: String,
        }

        // #[cfg(test)]
        // pub struct RmpModel {
        //     pub data_dir: String,
        // }

        // 4. Implement the FFI interface
        // #[cfg(not(test))]
        #[::uniffi::export]
        impl RmpModel {
            #[::uniffi::constructor]
            pub fn new(data_dir: String) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self { data_dir })
            }

            pub fn action(&self, action: $Action) {
                // Get the global model and call its action method
                let mut model = self
                    .get_or_set_global_model()
                    .write()
                    .expect("Failed to acquire write lock on model");

                // Call the action method from the RmpAppModel trait
                use $crate::traits::RmpAppModel;
                model.action(action);
            }

            pub fn listen_for_model_updates(&self, view_model: Box<dyn RmpViewModel>) {
                // Set up the listener
                let model = self
                    .get_or_set_global_model()
                    .read()
                    .expect("Failed to acquire read lock on model");

                // Just pass the updater as is
                $crate::listen_for_model_updates(&*model, view_model);
            }
        }

        // #[cfg(test)]
        // impl RmpModel {
        //     pub fn new(data_dir: String) -> std::sync::Arc<Self> {
        //         std::sync::Arc::new(Self { data_dir })
        //     }

        //     pub fn action(&self, action: $Action) {
        //         // Get the global model and call its action method
        //         let mut model = self
        //             .get_or_set_global_model()
        //             .write()
        //             .expect("Failed to acquire write lock on model");

        //         // Call the action method from the RmpAppModel trait
        //         use $crate::traits::RmpAppModel;
        //         model.action(action);
        //     }

        //     pub fn listen_for_model_updates(&self, _updater: Box<dyn RmpViewModel>) {
        //         // For tests, we won't actually call listen_for_model_updates to avoid
        //         // thread spawning and other side effects
        //     }
        // }

        // 5. Helper methods for the FFI object
        impl RmpModel {
            fn get_or_set_global_model(&self) -> &std::sync::RwLock<$Model> {
                GLOBAL_MODEL.get_or_init(|| {
                    // Create a new model
                    let model =
                        <$Model as $crate::traits::RmpAppModel>::create(self.data_dir.clone());
                    std::sync::RwLock::new(model)
                })
            }
        }

        // 6. Implement the framework's RmpViewModel trait for our local RmpViewModel
        impl $crate::traits::RmpViewModel for dyn RmpViewModel {
            type UpdateType = $ModelUpdate;

            fn model_update(&self, model_update: Self::UpdateType) {
                RmpViewModel::model_update(self, model_update);
            }
        }

        // 7. Extend the ViewModel to integrate with the framework
        impl $ViewModel {
            pub fn init(sender: $crate::crossbeam::channel::Sender<$ModelUpdate>) {
                GLOBAL_VIEW_MODEL.get_or_init(|| $ViewModel(sender));
            }

            pub fn model_update(model_update: $ModelUpdate) {
                if let Some(view_model) = GLOBAL_VIEW_MODEL.get() {
                    view_model
                        .0
                        .send(model_update)
                        .expect("Failed to send model update");
                }
            }
        }

        // 8. Set up the uniffi scaffolding
        // #[cfg(not(test))]
        ::uniffi::setup_scaffolding!();
    };
}
