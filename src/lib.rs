use std::convert::AsRef;
use std::process::{Command, ExitStatus};

/// This function either uses the `override_editor` argument as an editor
/// or tries to get this information from the environment variables.
/// A file to edit can be provided via `extra_args`
///
/// Example usage:
/// ```no_run
/// spawn_editor::spawn_editor(Some("nano"), &["src/lib.rs"]);
/// ```
pub fn spawn_editor(
    override_editor: Option<&str>,
    extra_args: &[&str],
) -> Result<ExitStatus, failure::Error> {
    let editor: std::borrow::Cow<str> = match override_editor {
        Some(z) => z.into(),
        None => default_editor::get()?.into(),
    };

    let joined_args = {
        let mut all_args = vec![&*editor];
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
        .spawn()?
        .wait()?)
}

/// This function is a convenient wrapper around [`spawn_editor`],
/// in case that the arguments aren't simple string slices
pub fn spawn_editor_generic<Ta, Tb>(
    override_editor: Option<Ta>,
    extra_args: &[Tb],
) -> Result<ExitStatus, failure::Error>
where
    Ta: AsRef<str>,
    Tb: AsRef<str>,
{
    let xar: Vec<_> = extra_args.iter().map(|x| x.as_ref()).collect();
    let real_oore = override_editor.as_ref().map(|x| x.as_ref());
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
pub fn spawn_editor_with_args<Tb>(extra_args: &[Tb]) -> Result<ExitStatus, failure::Error>
where
    Tb: AsRef<str>,
{
    spawn_editor_generic::<&str, Tb>(None, extra_args)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testit() {
        spawn_editor_with_args(&["src/lib.rs"]);
    }
}
*/
