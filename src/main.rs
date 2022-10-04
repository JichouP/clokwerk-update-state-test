use clokwerk::{AsyncScheduler, TimeUnits};
use futures;
use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Default)]
struct MyState {
    pub x: String,
}

#[tokio::main]
async fn main() {
    let state = MyState::default();
    let state = Arc::new(Mutex::new(state));

    let _state = Arc::clone(&state);
    let handle1 = schedule(move || print_state(Arc::clone(&_state)));

    let _state = Arc::clone(&state);
    let handle2 = schedule(move || update_state(Arc::clone(&_state)));

    futures::future::join(handle1, handle2).await;
}

async fn print_state(state: Arc<Mutex<MyState>>) {
    let state = state.lock().unwrap();
    println!("current state is: {:?}", state.x);
}

async fn update_state(state: Arc<Mutex<MyState>>) {
    let mut state = state.lock().unwrap();
    state.x += "x";
}

async fn schedule<F, T>(f: F) -> JoinHandle<()>
where
    F: 'static + FnMut() -> T + Send,
    T: 'static + Future<Output = ()> + Send,
{
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.seconds()).run(f);

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
