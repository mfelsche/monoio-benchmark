use config::{ServerConfig, PACKET_SIZE};
use async_global_executor::GlobalExecutorConfig;
use async_std::io::{WriteExt, ReadExt};
use async_std::net::TcpListener;


fn main() {
    let cfg = ServerConfig::parse();
    let cores = cfg.cores.len();
    println!(
        "Running ping pong server with Async-Std.\nPacket size: {}\nListen {}\nCPU count: {}",
        PACKET_SIZE, cfg.bind, cores
    );
    // TODO: use async_executor::Executor vs. async_executor::LocalExecutor when cores = 1
    async_global_executor::init_with_config(GlobalExecutorConfig::default().with_min_threads(cores).with_max_threads(cores));
    async_global_executor::block_on(serve(&cfg));
}

async fn serve(cfg: &ServerConfig) {
    let listener = TcpListener::bind(&cfg.bind).await.unwrap();
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        async_global_executor::spawn(async move {
            let mut buf = vec![0; PACKET_SIZE];
            loop {
                match stream.read_exact(&mut buf).await {
                    Ok(_) => {}
                    Err(_) => {
                        return;
                    }
                }
                match stream.write_all(&buf).await {
                    Ok(_) => {}
                    Err(_) => {
                        return;
                    }
                }
            }
        }).detach();
    }
}