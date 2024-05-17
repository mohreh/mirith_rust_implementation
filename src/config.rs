use config::{File, FileFormat};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub const MIRITH_MODE: u8 = 0;

pub const CRYPTO_ALGNAME: &str = "MiRitH";

// Define an enum for the standards
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Set {
    Ia,
    Ib,
    IIIa,
    IIIb,
    Va,
    Vb,
}

// Define an enum for the hypercube versions
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Variant {
    Fast,
    Short,
    #[cfg(feature = "hypercube")]
    Shorter,
    #[cfg(feature = "hypercube")]
    Shortest,
}

impl FromStr for Set {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ia" => Ok(Set::Ia),
            "ib" => Ok(Set::Ib),
            "iiia" => Ok(Set::IIIa),
            "iiib" => Ok(Set::IIIb),
            "va" => Ok(Set::Va),
            "vb" => Ok(Set::Vb),
            _ => Err("not ok".to_string()),
        }
    }
}

impl FromStr for Variant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fast" => Ok(Self::Fast),
            "short" => Ok(Self::Short),
            #[cfg(feature = "hypercube")]
            "shorter" => Ok(Self::Shorter),
            #[cfg(feature = "hypercube")]
            "shortest" => Ok(Self::Shortest),
            other => {
                #[cfg(not(feature = "hypercube"))]
                return Err(format!(
                    "{} is not supported, Use either `fast` or `short`.",
                    other
                ));

                #[cfg(feature = "hypercube")]
                Err(format!(
                    "{} is not supported, Use either `fast`, `short` `shorter` or `shortest`.",
                    other
                ))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub set: Set,
    pub variant: Variant,
    pub deterministic: bool,
    pub supercop: bool,
    #[serde(deserialize_with = "deserialize_mode", default)]
    pub mode: u8,
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");

    let builder = config::Config::builder().add_source(
        File::from(base_path.join("config"))
            .format(FileFormat::Yaml)
            .required(true),
    );

    let mut config: Config = builder.build()?.try_deserialize()?;

    config.mode = {
        #[cfg(not(feature = "hypercube"))]
        {
            config.set as u8 * 2 + config.variant as u8
        }

        #[cfg(feature = "hypercube")]
        {
            config.set as u8 * 4 + config.variant as u8
        }
    };
    Ok(config)
}

fn deserialize_mode<'de, D>(_deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(0)
}
