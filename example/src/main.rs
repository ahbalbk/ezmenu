use ezmenu::Menu;
use ezmenu::lib::MenuVec;

#[ezmenu::parsed]
enum Type {
    MIT,
    BSD,
    GPL,
}

#[derive(Menu)]
#[menu(
    title = "mkLicense program",
    chip = "--> Give the ",
    prefix = ">> ",
    new_line = true,
)]
struct License {
    #[menu(msg = "authors name separated by spaces")]
    authors: MenuVec<String>,

    #[menu(msg = "project name")]
    proj_name: String,

    #[menu(msg = "project year", default = 2022)]
    year: u16,

    #[menu(msg = "license type", default = "mit")]
    ty: Type,
}

fn main() {
    let _ = License::from_menu();
}
