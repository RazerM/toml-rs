#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

fn main() -> Result<(), io::Error> {
    let config = Config {
        database: Database {
            ip: "192.168.1.1".to_string(),
            port: vec![8001, 8002, 8003],
            connection_max: 5000,
            enabled: false,
        },
    };

    let mut writer = io::Cursor::new(Vec::default());
    toml::to_writer(&mut writer, &config)?;

    println!(
        "Encode from writer: {:?}",
        std::str::from_utf8(&writer.into_inner()).unwrap()
    );

    Ok(())
}
