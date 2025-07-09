
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Generic observer trait that can observe any type of subject
#[async_trait]
pub trait Observer<T>: Send + Sync {
    /// Called when the subject notifies observers of a change
    async fn notify(&self, data: &T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Optional method to get observer identifier
    fn id(&self) -> String {
        format!("{:p}", self)
    }
}

/// Subject trait that can be observed
#[async_trait]
pub trait Subject<T>: Send + Sync {
    /// Add an observer to the subject
    async fn attach(&self, observer: Arc<dyn Observer<T>>);

    /// Remove an observer from the subject
    async fn detach(&self, observer_id: &str);

    /// Notify all observers of a change
    async fn notify_observers(&self, data: &T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Concrete implementation of a subject that can be observed
pub struct ObservableSubject<T> {
    observers: Arc<RwLock<HashMap<String, Arc<dyn Observer<T>>>>>,
}

impl<T> ObservableSubject<T> {
    pub fn new() -> Self {
        Self {
            observers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn observer_count(&self) -> usize {
        self.observers.read().await.len()
    }
}

impl<T> Default for ObservableSubject<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T> Subject<T> for ObservableSubject<T>
where
    T: Send + Sync,
{
    async fn attach(&self, observer: Arc<dyn Observer<T>>) {
        let id = observer.id();
        self.observers.write().await.insert(id, observer);
    }

    async fn detach(&self, observer_id: &str) {
        self.observers.write().await.remove(observer_id);
    }

    async fn notify_observers(&self, data: &T) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let observers = self.observers.read().await;

        // Collect all notification results
        let mut results = Vec::new();
        for observer in observers.values() {
            results.push(observer.notify(data).await);
        }

        // Check if any observer failed
        for result in results {
            if let Err(e) = result {
                return Err(e);
            }
        }

        Ok(())
    }
}

/// Convenience macro for creating observers
#[macro_export]
macro_rules! create_observer {
    ($name:ident, $data_type:ty, $handler:expr) => {
        pub struct $name;
        
        #[async_trait::async_trait]
        impl Observer<$data_type> for $name {
            async fn notify(&self, data: &$data_type) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                $handler(data).await
            }
            
            fn id(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug, Clone)]
    struct TestData {
        value: i32,
    }

    struct TestObserver {
        id: String,
        call_count: Arc<AtomicUsize>,
    }

    impl TestObserver {
        fn new(id: String) -> Self {
            Self {
                id,
                call_count: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn get_call_count(&self) -> usize {
            self.call_count.load(Ordering::Relaxed)
        }
    }

    #[async_trait]
    impl Observer<TestData> for TestObserver {
        async fn notify(&self, _data: &TestData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.call_count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    #[tokio::test]
    async fn test_observer_pattern() {
        let subject = ObservableSubject::new();
        let observer1 = Arc::new(TestObserver::new("observer1".to_string()));
        let observer2 = Arc::new(TestObserver::new("observer2".to_string()));

        // Attach observers
        subject.attach(observer1.clone()).await;
        subject.attach(observer2.clone()).await;

        assert_eq!(subject.observer_count().await, 2);

        // Notify observers
        let test_data = TestData { value: 42 };
        subject.notify_observers(&test_data).await.unwrap();

        assert_eq!(observer1.get_call_count(), 1);
        assert_eq!(observer2.get_call_count(), 1);

        // Detach one observer
        subject.detach("observer1").await;
        assert_eq!(subject.observer_count().await, 1);

        // Notify again
        subject.notify_observers(&test_data).await.unwrap();

        assert_eq!(observer1.get_call_count(), 1); // Should not increase
        assert_eq!(observer2.get_call_count(), 2); // Should increase
    }
}