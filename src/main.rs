pub(crate) mod api;
pub(crate) mod consts;

use slint::include_modules;

include_modules!();
fn main() -> std::io::Result<()> {
    let _zxcvbn = api::Zxcvbn::new();
    let ui = ZxcvbnUI::new().expect("Raised a PlatformError");

    let _ = ui.run();
    Ok(())
}
