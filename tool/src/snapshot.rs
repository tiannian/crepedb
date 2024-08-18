use clap::Args;

#[derive(Debug, Clone)]
pub enum Version {
    Latest,
    Root,
    Number(u64),
}

fn parse_version(s: &str) -> Result<Version, String> {
    match s {
        "latest" => Ok(Version::Latest),
        "root" => Ok(Version::Root),
        _ => {
            let data: u64 = s.parse().map_err(|_| String::from("unsupport value"))?;

            Ok(Version::Number(data))
        }
    }
}

#[derive(Debug, Args)]
pub struct Snapshot {
    #[arg(value_parser = parse_version)]
    version: Option<Version>,

    #[arg(long)]
    id: Option<u64>,
}
