use mirith::config::{get_configuration, Config};

fn main() {
    let config: Config = get_configuration().expect("failed to read configuration");

    println!("{:?}", config.set as u8);
    println!("{:?}", config);
}
