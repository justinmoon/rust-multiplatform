//! Tests for the rust-multiplatform framework

#[cfg(test)]
mod tests {
    use crossbeam::channel::{Sender, unbounded};
    // No need for Arc import
    
    // Define a test model update type
    #[derive(Debug, PartialEq, Clone)]
    pub enum TestModelUpdate {
        TestUpdate { value: i32 },
    }

    // Define a test action type
    #[derive(Debug, PartialEq)]
    pub enum TestAction {
        TestAction,
    }

    // Define a test model with an app builder field for the update receiver
    #[derive(Debug)]
    pub struct TestModel {
        pub count: i32,
        pub data_dir: String,
    }

    // Implement RmpAppModel for the test model
    impl crate::traits::RmpAppModel for TestModel {
        type ActionType = TestAction;
        type UpdateType = TestModelUpdate;

        fn create(data_dir: String) -> Self {
            TestModel {
                count: 0,
                data_dir,
            }
        }

        fn action(&mut self, action: Self::ActionType) {
            match action {
                TestAction::TestAction => self.count += 1,
            }
        }
    }

    // Define a test view model
    #[derive(Clone)]
    struct TestViewModel(pub Sender<TestModelUpdate>);

    // Use the register_app macro to generate the FFI code
    // This is what we're testing - that the macro expands properly
    crate::register_app!(TestModel, TestViewModel, TestAction, TestModelUpdate);

    #[test]
    fn test_model_creation() {
        // Create an RmpModel instance
        let model = RmpModel::new("test_dir".to_string());
        
        // Verify it has the right data_dir
        assert_eq!(model.data_dir, "test_dir");
    }

    #[test]
    fn test_action_handling() {
        // Create an RmpModel instance
        let model = RmpModel::new("test_dir".to_string());
        
        // Call the action method
        model.action(TestAction::TestAction);
        
        // Get the global model
        let global_model = model.get_or_set_global_model().read().unwrap();
        
        // Verify the action was handled
        assert_eq!(global_model.count, 1);
    }

    #[test]
    fn test_view_model() {
        // Create a channel for the view model
        let (sender, receiver) = unbounded();
        
        // Initialize the view model
        TestViewModel::init(sender);
        
        // Send a model update
        TestViewModel::model_update(TestModelUpdate::TestUpdate { value: 42 });
        
        // Verify the update was sent
        if let Ok(update) = receiver.try_recv() {
            match update {
                TestModelUpdate::TestUpdate { value } => assert_eq!(value, 42),
            }
        } else {
            panic!("No update received");
        }
    }
}