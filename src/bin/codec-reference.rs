//! Offline differential-test driver; no networking, task acknowledgments or finish path.
//! Text envelope documented in docs/java-reference.md. Not a public stable CLI.
use horizon_neoforge_compat::codec::Id;
use horizon_neoforge_compat::modded::{
    ConnectionProtocol, Flow, ModdedQuery, ModdedSetup, NetworkChannel, QueryComponent,
};
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 || !text.is_ascii() {
        return Err("hex".into());
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|_| "hex".into()))
        .collect()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn string(text: &str) -> Result<String, String> {
    String::from_utf8(unhex(text)?).map_err(|_| "UTF-8".into())
}
fn parse_model(kind: &str, text: &str) -> Result<(ModdedQuery, ModdedSetup), String> {
    let mut query = ModdedQuery::default();
    let mut setup = ModdedSetup::default();
    let mut phase = None;
    if !text.is_empty() {
        for line in text.split(';') {
            let fields: Vec<_> = line.split(':').collect();
            match fields.as_slice() {
                ["P", ordinal] => {
                    let p = ConnectionProtocol::from_ordinal(ordinal.parse().map_err(|_| "phase")?)
                        .map_err(|e| e.to_string())?;
                    if query.phases.insert(p, BTreeMap::new()).is_some() {
                        return Err("duplicate phase".into());
                    }
                    setup.phases.insert(p, BTreeMap::new());
                    phase = Some(p);
                }
                ["Q", id, version, flow, optional] if kind == "query" => {
                    let id = Id::parse(&string(id)?).map_err(|e| e.to_string())?;
                    let flow = match *flow {
                        "-1" => None,
                        "0" => Some(Flow::Serverbound),
                        "1" => Some(Flow::Clientbound),
                        _ => return Err("flow".into()),
                    };
                    let optional = match *optional {
                        "0" => false,
                        "1" => true,
                        _ => return Err("optional".into()),
                    };
                    let p = phase.ok_or("missing phase")?;
                    let record = QueryComponent {
                        id: id.clone(),
                        version: string(version)?,
                        flow,
                        optional,
                    };
                    if query
                        .phases
                        .get_mut(&p)
                        .unwrap()
                        .insert(id, record)
                        .is_some()
                    {
                        return Err("duplicate ID".into());
                    }
                }
                ["S", id, version] if kind == "setup" => {
                    let id = Id::parse(&string(id)?).map_err(|e| e.to_string())?;
                    let p = phase.ok_or("missing phase")?;
                    let record = NetworkChannel {
                        id: id.clone(),
                        chosen_version: string(version)?,
                    };
                    if setup
                        .phases
                        .get_mut(&p)
                        .unwrap()
                        .insert(id, record)
                        .is_some()
                    {
                        return Err("duplicate ID".into());
                    }
                }
                _ => return Err("model".into()),
            }
        }
    }
    Ok((query, setup))
}
fn request(line: &str) -> Result<String, String> {
    if line.len() > 200_000 {
        return Err("request size".into());
    }
    let fields: Vec<_> = line.split('\t').collect();
    let [action, kind, data] = fields.as_slice() else {
        return Err("request".into());
    };
    if !["query", "setup"].contains(kind) {
        return Err("kind".into());
    }
    if *action == "encode" {
        let (query, setup) = parse_model(kind, data)?;
        let bytes = if *kind == "query" {
            query.encode()
        } else {
            setup.encode()
        }
        .map_err(|e| e.to_string())?;
        return Ok(hex(&bytes));
    }
    if *action != "decode" || data.len() > 65_534 {
        return Err("bounded decode request".into());
    }
    let bytes = unhex(data)?;
    let mut lines = Vec::new();
    if *kind == "query" {
        let query = ModdedQuery::decode(&bytes).map_err(|e| e.to_string())?;
        for (phase, records) in query.phases {
            lines.push(format!("P:{}", phase.ordinal()));
            for record in records.into_values() {
                lines.push(format!(
                    "Q:{}:{}:{}:{}",
                    hex(record.id.as_str().as_bytes()),
                    hex(record.version.as_bytes()),
                    record.flow.map_or(-1, Flow::ordinal),
                    u8::from(record.optional)
                ));
            }
        }
    } else {
        let setup = ModdedSetup::decode(&bytes).map_err(|e| e.to_string())?;
        for (phase, records) in setup.phases {
            lines.push(format!("P:{}", phase.ordinal()));
            for record in records.into_values() {
                lines.push(format!(
                    "S:{}:{}",
                    hex(record.id.as_str().as_bytes()),
                    hex(record.chosen_version.as_bytes())
                ));
            }
        }
    }
    Ok(lines.join(";"))
}
fn main() {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    for line in io::stdin().lock().lines() {
        let result = line.map_err(|e| e.to_string()).and_then(|s| request(&s));
        let response = match result {
            Ok(value) => format!("REF\tOK\t{value}"),
            Err(error) => format!("REF\tERR\t{error}"),
        };
        if writeln!(output, "{response}")
            .and_then(|()| output.flush())
            .is_err()
        {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_empty_maps() {
        for kind in ["query", "setup"] {
            assert_eq!(request(&format!("encode\t{kind}\t")).unwrap(), "00");
            assert_eq!(request(&format!("decode\t{kind}\t00")).unwrap(), "");
        }
    }

    #[test]
    fn envelope_preserves_empty_phases_and_unicode() {
        for text in ["P:4", "P:4;Q:613a78:f09fa6ab:-1:1"] {
            let bytes = request(&format!("encode\tquery\t{text}")).unwrap();
            assert_eq!(request(&format!("decode\tquery\t{bytes}")).unwrap(), text);
        }
    }

    #[test]
    fn bad_envelopes_and_models_refuse() {
        for text in [
            "",
            "decode\tquery",
            "decode\tother\t00",
            "decode\tquery\t0",
            "decode\tquery\tλ",
            "decode\tquery\tzz",
            "encode\tquery\tP:5",
            "encode\tquery\tP:4;P:4",
            "encode\tsetup\tS:613a78:31",
            "encode\tquery\tP:4;Q:613a78:31:2:0",
            "encode\tquery\tP:4;Q:613a78:31:0:2",
        ] {
            assert!(request(text).is_err(), "{text}");
        }
    }
}
