use imgui::{InputText, Ui};

use crate::helpers::locale::locale_lang::LocaleLang;

pub fn show_cmd_editor(ui: &Ui, cmds: &mut String, label: &str, locale_lang: &LocaleLang) -> bool {
    // TODO
    InputText::new(ui, label, cmds)
        .hint(locale_lang.get_string_from_id("commands"))
        .build()
}
