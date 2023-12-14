use std::process;
use youdao_rs::{opt::get_opt, run};

#[tokio::main]
async fn main() {
    let opt = get_opt();

    // println!("{:#?}", opt);

    if let Err(e) = run(opt).await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
