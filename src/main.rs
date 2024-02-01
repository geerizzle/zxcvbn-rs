pub(crate) mod api;
pub(crate) mod consts;

use slint::include_modules;
use std::fs::File;

include_modules!();
fn main() -> std::io::Result<()> {
    let subs_dict = File::open("tests/leet.tsv")?;
    let mut zxcvbn = api::Zxcvbn::new();
    zxcvbn.set_substitution(&subs_dict);
    let ui = ZxcvbnUI::new().expect("Raised a PlatformError");

    let _ = ui.run();
    Ok(())
}
