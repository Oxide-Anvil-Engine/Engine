
pub struct Tokio {
    pub(crate) runtime: Option<tokio::runtime::Runtime>,
}

impl Clone for Tokio {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Tokio {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Falha ao criar o runtime Tokio");
        Self { runtime: Some(runtime) }
    }

    pub fn handle(&self) -> tokio::runtime::Handle {
        self.runtime.as_ref().unwrap().handle().clone()
    }

    pub fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.runtime.as_ref().unwrap().block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.as_ref().unwrap().spawn(future)
    }
}