use log::error;
use pinns::Pinns;
use std::process::exit;

fn main() {
    if let Err(e) = Pinns::default().run() {
        error!(
            "{}",
            &e.chain()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(": ")
        );
        exit(1);
    }
}
