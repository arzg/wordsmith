use std::path::PathBuf;
use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    fs::write(LIGHT.path(), LIGHT.to_string())
}

const LIGHT: Theme = Theme {
    name: "Light",
    background: Rgb(0xF7F7F7),
    ui_background: Rgb(0xFCFCFC),
    foreground: Rgb(0x1A1A1A),
    teal: Rgb(0x16BDEC),
    out_of_focus: Rgb(0xC7C4C2),
    borders: Rgb(0xDBDBDB),
    editor_selection: Rgb(0xBFE8F4),
    ui_selection: Rgb(0xB3D7FF),
};

struct Theme {
    name: &'static str,
    background: Rgb,
    ui_background: Rgb,
    foreground: Rgb,
    teal: Rgb,
    out_of_focus: Rgb,
    borders: Rgb,
    editor_selection: Rgb,
    ui_selection: Rgb,
}

impl Theme {
    fn path(&self) -> PathBuf {
        PathBuf::from(format!("themes/Wordsmith-{}-color-theme.json", self.name))
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\"name\": \"Wordsmith {}\",", self.name)?;

        writeln!(f, "\"colors\": {{")?;
        write_scope(f, "editor.background", self.background)?;
        write_scope(f, "editor.foreground", self.foreground)?;
        write_scope(f, "foreground", self.foreground)?;

        write_scope(f, "editorLineNumber.foreground", self.out_of_focus)?;
        write_scope(f, "editorLineNumber.activeForeground", self.foreground)?;

        write_scope(f, "editorIndentGuide.background", self.out_of_focus)?;
        write_scope(f, "editorIndentGuide.activeBackground", self.out_of_focus)?;
        write_scope(f, "tree.indentGuidesStroke", self.out_of_focus)?;

        write_scope(f, "sideBar.background", self.ui_background)?;
        write_scope(f, "sideBar.border", self.borders)?;

        write_scope(f, "activityBar.background", self.ui_background)?;
        write_scope(f, "activityBar.foreground", self.foreground)?;
        write_scope(f, "activityBar.inactiveForeground", self.out_of_focus)?;
        write_scope(f, "activityBar.activeBorder", self.teal)?;
        write_scope(f, "activityBar.border", self.borders)?;

        write_scope(f, "statusBar.foreground", self.foreground)?;
        write_scope(f, "statusBar.background", self.background)?;
        write_scope(f, "statusBar.debuggingBackground", self.background)?;
        write_scope(f, "statusBar.noFolderBackground", self.background)?;
        write_scope(f, "statusBar.border", self.borders)?;

        write_scope(f, "tab.activeBackground", self.ui_background)?;
        write_scope(f, "tab.border", self.borders)?;
        write_scope(f, "tab.inactiveBackground", self.background)?;
        write_scope(f, "breadcrumb.background", self.ui_background)?;
        write_scope(f, "editorGroupHeader.border", self.borders)?;
        write_scope(f, "editorGroupHeader.noTabsBackground", self.ui_background)?;
        write_scope(f, "editorGroupHeader.tabsBackground", self.background)?;
        write_scope(f, "editorGroup.border", self.borders)?;

        write_scope(f, "editorWidget.background", self.ui_background)?;
        write_scope(f, "editorWidget.border", self.borders)?;

        write_scope(f, "editorCursor.background", self.teal)?;
        write_scope(f, "editorCursor.foreground", self.teal)?;

        write_scope(f, "editor.selectionBackground", self.editor_selection)?;
        write_scope(f, "minimap.selectionHighlight", self.editor_selection)?;
        write_scope(f, "selection.background", self.ui_selection)?;

        write_scope(f, "textLink.foreground", self.teal)?;
        write_scope(f, "textLink.activeForeground", self.teal)?;
        write_scope(f, "editorLink.activeForeground", self.teal)?;

        write_scope(f, "rust_analyzer.inlayHints.foreground", self.out_of_focus)?;
        writeln!(f, "}}")?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

fn write_scope(f: &mut fmt::Formatter<'_>, scope_name: &str, value: Rgb) -> fmt::Result {
    writeln!(f, "\"{}\": {},", scope_name, value)
}

#[derive(Copy, Clone)]
struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}\"", self.0)
    }
}
