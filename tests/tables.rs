//! Tests of table rendering

use termimad::MadSkin;

/// see https://github.com/Canop/termimad/issues/77
#[test]
fn test_fix_issue_77() {
    let skin = MadSkin::default();
    // Header has 2 columns; the data row has 6.
    let md = "| Key | Value |\n\
              | --- | --- |\n\
              | alpha | beta | gamma | delta | epsilon | zeta |\n";
    let _ = skin.text(md, Some(20)).to_string(); // Panics with Termimad 0.35.0
}
