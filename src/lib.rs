extern crate url;

use std::{
    env, error, fs,
    io::{Error, ErrorKind},
    process::{Command, Stdio},
    result::Result,
};

use url::Url;

pub fn open(spec: &str) -> Result<i32, Box<dyn error::Error>> {
    match Command::new(get_external_browser())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(get_url(spec)?)
        .status()
    {
        Ok(status) if status.success() => Ok(0),
        Ok(status) => Err(Box::new(Error::new(
            ErrorKind::Other,
            match status.code() {
                Some(c) => format!("process returned status code {}", c),
                None => "process was terminated by a signal".into(),
            },
        ))),
        Err(e) => Err(Box::new(e)),
    }
}

fn fixup_url(url: Url) -> Result<String, Box<dyn error::Error>> {
    Ok(if url.scheme() == "file" {
        let mut path = wslpath::wsl_to_windows(url.path())?;

        if path.starts_with("//wsl$/") {
            // firefox requires an extra / here for some reason
            path.insert(0, '/');
        }
        path.insert_str(0, "file://");

        path
    } else {
        url.into()
    })
}

fn force_url(spec: &str) -> Result<Url, Box<dyn error::Error>> {
    Ok(match Url::parse(spec) {
        Ok(u) => u,
        Err(_e) => Url::parse("file:///")?.join(&fs::canonicalize(spec)?.to_string_lossy())?,
    })
}

fn get_external_browser() -> String {
    match env::var("EXTERNAL_BROWSER") {
        Ok(v) => v,
        Err(_) => "external-browser".into(),
    }
}

fn get_url(spec: &str) -> Result<String, Box<dyn error::Error>> {
    fixup_url(force_url(spec)?)
}
