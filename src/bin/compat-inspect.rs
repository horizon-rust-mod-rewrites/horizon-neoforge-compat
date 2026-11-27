//! Offline payload inspection only. Never connects to a server.
use horizon_neoforge_compat::{
    codec::*, extension::Extensions, registry::Snapshot, session::Session, Error, Result,
};
use std::{fs::File, io::Read};
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["common-demo"] {
        let mut s = Session::new("1.21.1", 767, "21.1.248", Extensions::new())?;
        for (channel, body) in [
            ("c:version", &[1, 1][..]),
            ("c:register", &[1, 4, b'p', b'l', b'a', b'y', 0][..]),
        ] {
            for reply in s.receive(channel, body)? {
                println!("reply {} {:02x?}", reply.channel, reply.body);
            }
        }
        println!(
            "state={:?}; NOT joined; finish_configuration is deliberately unsupported",
            s.state()
        );
        return Ok(());
    }
    if args.len() != 2 {
        return Err(Error::Invalid(
            "usage: compat-inspect common-demo | (version|register|dinnerbone|frozen) FILE",
        ));
    }
    let file = File::open(&args[1]).map_err(|e| Error::Handler(format!("open fixture: {e}")))?;
    let mut bytes = Vec::new();
    file.take((MAX_BODY + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| Error::Handler(format!("read fixture: {e}")))?;
    match args[0].as_str() {
        "version" => println!("{:?}", CommonVersion::decode(&bytes)?),
        "register" => println!("{:?}", CommonRegister::decode(&bytes)?),
        "dinnerbone" => println!("{:?}", decode_dinnerbone(&bytes)?),
        "frozen" => println!("{:?}", Snapshot::decode(&bytes)?),
        _ => return Err(Error::Invalid("unknown fixture kind")),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
