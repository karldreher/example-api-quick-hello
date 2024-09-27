use serde_json::json;
use tokio::signal::unix::{signal, SignalKind};
use warp::Filter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = signal(SignalKind::terminate())?;

    let hello = warp::path("hello").map(|| {
        warp::reply::json(&json!({ "data": "world" }))
    });
    let (_, server) = warp::serve(hello)
    .bind_with_graceful_shutdown(([0, 0, 0, 0], 8000), async move {
        println!("waiting for signal");
        stream.recv().await;
        println!("done waiting for signal");
    });

    match tokio::join!(tokio::task::spawn(server)).0 {
    Ok(()) => println!("serving"),
    Err(e) => println!("ERROR: Thread join error {}", e)
    };

    println!("terminating");
    Ok(())

}