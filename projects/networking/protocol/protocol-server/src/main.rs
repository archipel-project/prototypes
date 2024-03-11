use std::{thread::sleep, time::Duration};

use internal::server::NotchianServer;

mod internal;

fn main() -> anyhow::Result<()> {
    let mut server = NotchianServer::new()?;

    loop {
        server.tick(Duration::from_millis(50));
        sleep(Duration::from_millis(50));
    }
}
