use std::future::Future;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

thread_local! {
    pub static RUNTIME: Lazy<Mutex<Option<Runtime>>> = Lazy::new(Default::default);
}

/// Creates tokio runtime
#[inline]
fn create_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

/// Creates a thread-local tokio runtime
#[inline]
pub fn init_tokio() {
    RUNTIME.with(|rt| { *rt.lock().unwrap() = Some(create_runtime()) });
}

/// Block execution on specific future
#[inline]
pub fn tokio_block_on<F: Future>(f: F) -> F::Output {
    RUNTIME.with(|rt| { rt.try_lock().unwrap().as_mut().unwrap().block_on(f) })
}

/// Spawn a future without blocking on it
#[allow(dead_code)]
#[inline]
pub fn tokio_spawn<F: Future + std::marker::Send + 'static>(f: F)
    where
        <F as futures::Future>::Output: std::marker::Send,
{
    RUNTIME.with(|rt| {
        rt.try_lock().unwrap().as_mut().unwrap().spawn(f)
    });
}

