use otpauth::TOTP;

use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();

    assert_eq!(args.len(), 2);
    let secret = args
        .nth(1)
        .expect("This definitely should not happen. You forgot to provide the base32 secret.");

    // the base32 secret could be stored in a stronghold instance
    let auth = TOTP::from_base32(secret).expect("Could not build TOTP from base32 secret");
    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let code = auth.generate(30, timestamp1);

    println!("{:06}", code);
    Ok(())
}
