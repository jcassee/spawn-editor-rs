#![forbid(unsafe_code)]

use std::convert::AsRef;
use std::process::{Command, ExitStatus};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SEError {
    #[error("editor spawning/waiting failed")]
    Process(#[source] std::io::Error),
    #[error("got invalid environment variable")]
    Var(#[source] std::env::VarError),
}

type SEResult = Result<ExitStatus, SEError>;

/// This function either uses the `override_editor` argument as an editor
/// or tries to get this information from the environment variables.
/// A file to edit can be provided via `extra_args`
///
/// Example usage:
/// ```no_run
/// spawn_editor::spawn_editor(Some("nano"), &["src/lib.rs"]);
/// ```
pub fn spawn_editor(override_editor: Option<&str>, extra_args: &[&str]) -> SEResult {
    let editor: std::borrow::Cow<str> = match override_editor {
        Some(z) => z.into(),
        None => default_editor::get().map_err(SEError::Var)?.into(),
    };

    let joined_args = {
        let mut all_args = Vec::with_capacity(1 + extra_args.len());
        all_args.push(&*editor);
        all_args.extend(extra_args.iter());
        all_args.join(" ")
    };

    let (sh_x, sh_c) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Ok(Command::new(sh_x)
        .args(&[sh_c, &joined_args[..]])
        .spawn()
        .and_then(|mut c| c.wait())
        .map_err(SEError::Process)?)
}

/// This function is a convenient wrapper around [`spawn_editor`],
/// in case that the arguments aren't simple string slices
pub fn spawn_editor_generic<Ta, Tb>(override_editor: Option<Ta>, extra_args: &[Tb]) -> SEResult
where
    Ta: AsRef<str>,
    Tb: AsRef<str>,
{
    let real_oore = override_editor.as_ref().map(|x| x.as_ref());
    let xar: Vec<_> = extra_args.iter().map(|x| x.as_ref()).collect();
    spawn_editor(real_oore, &xar[..])
}

/// This function is a convenient wrapper around [`spawn_editor_generic`],
/// in case that `override_editor == None`
///
/// Example usage:
/// ```no_run
/// spawn_editor::spawn_editor_with_args(&["src/lib.rs"]);
/// ```
#[inline]
pub fn spawn_editor_with_args<Tb: AsRef<str>>(extra_args: &[Tb]) -> SEResult {
    spawn_editor_generic::<&str, Tb>(None, extra_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn testit() {
        spawn_editor_with_args(&["src/lib.rs"]);
    }
}
