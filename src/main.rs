pub(crate) mod api;

use slint::include_modules;

include_modules!();
fn main() -> std::io::Result<()> {
    let zxcvbn = api::Zxcvbn::new();
    let ui = ZxcvbnUI::new()
        .expect("Raised a PlatformError");

    let _ = ui.run();
    Ok(())
}
