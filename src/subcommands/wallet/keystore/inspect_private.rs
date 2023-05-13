use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use starknet::signers::SigningKey;

#[derive(Debug, Parser)]
pub struct InspectPrivate {
    #[clap(long, help = "Print the private key only")]
    raw: bool,
    #[clap(help = "Path to the JSON keystore")]
    file: PathBuf,
}

impl InspectPrivate {
    pub fn run(self) -> Result<()> {
        if !self.file.exists() {
            anyhow::bail!("keystore file not found");
        }

        let password = rpassword::prompt_password("Enter Password: ")?;

        let key = SigningKey::from_keystore(self.file, &password)?;

        if self.raw {
            println!("{:#064x}", key.secret_scalar());
        } else {
            println!("Private key: {:#064x}", key.secret_scalar());
        }

        Ok(())
    }
}
