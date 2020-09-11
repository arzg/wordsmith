use std::path::PathBuf;
use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    fs::write(LIGHT.path(), LIGHT.to_string())
}

const LIGHT: Theme = Theme {
    name: "Light",
    background: Rgb(0xF7F7F7),
    ui_background: Rgb(0xFCFCFC),
    ui_background_focused: Rgb(0xF0F0F0),
    foreground: Rgb(0x1A1A1A),
    teal: Rgb(0x16BDEC),
    yellow: Rgb(0xA66501),
    red: Rgb(0xCB4819),
    purple: Rgb(0xB24FA3),
    blue: Rgb(0x3476B9),
    green: Rgb(0x3F831E),
    out_of_focus: Rgb(0xC7C4C2),
    less_out_of_focus: Rgb(0x999999),
    borders: Rgb(0xDBDBDB),
    editor_selection: Rgb(0xBFE8F4),
    ui_selection: Rgb(0xB3D7FF),
};

struct Theme {
    name: &'static str,
    background: Rgb,
    ui_background: Rgb,
    ui_background_focused: Rgb,
    foreground: Rgb,
    teal: Rgb,
    yellow: Rgb,
    red: Rgb,
    purple: Rgb,
    blue: Rgb,
    green: Rgb,
    out_of_focus: Rgb,
    less_out_of_focus: Rgb,
    borders: Rgb,
    editor_selection: Rgb,
    ui_selection: Rgb,
}

impl Theme {
    fn path(&self) -> PathBuf {
        PathBuf::from(format!("themes/Wordsmith-{}-color-theme.json", self.name))
    }

    fn workbench_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        write_scope(f, "sideBarSectionHeader.foreground", self.less_out_of_focus)?;
        write_scope(f, "sideBarSectionHeader.background", self.ui_background)?;

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

        write_scope(f, "list.hoverBackground", self.ui_background)?;
        write_scope(f, "list.focusBackground", self.ui_background_focused)?;
        write_scope(f, "list.activeSelectionBackground", self.ui_background_focused)?;
        write_scope(f, "list.activeSelectionForeground", self.foreground)?;
        write_scope(f, "list.inactiveSelectionBackground", self.ui_background)?;
        write_scope(f, "list.highlightForeground", self.teal)?;

        write_scope(f, "panel.background", self.ui_background)?;
        write_scope(f, "panel.border", self.borders)?;

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

        write_scope(f, "editor.hoverHighlightBackground", self.editor_selection)?;
        write_scope(f, "editor.wordHighlightBackground", (self.editor_selection, 0x55))?;
        write_scope(f, "editor.selectionHighlightBackground", (self.editor_selection, 0x55))?;

        write_scope(f, "editorCursor.background", self.teal)?;
        write_scope(f, "editorCursor.foreground", self.teal)?;

        write_scope(f, "editor.selectionBackground", self.editor_selection)?;
        write_scope(f, "minimap.selectionHighlight", self.editor_selection)?;
        write_scope(f, "selection.background", self.ui_selection)?;

        write_scope(f, "textLink.foreground", self.teal)?;
        write_scope(f, "textLink.activeForeground", self.teal)?;
        write_scope(f, "editorLink.activeForeground", self.teal)?;

        write_scope(f, "terminal.foreground", self.foreground)?;
        write_scope(f, "terminal.ansiBlack", self.background)?;
        write_scope(f, "terminal.ansiBrightBlack", self.out_of_focus)?;
        write_scope(f, "terminal.ansiRed", self.red)?;
        write_scope(f, "terminal.ansiBrightRed", self.red)?;
        write_scope(f, "terminal.ansiGreen", self.green)?;
        write_scope(f, "terminal.ansiBrightGreen", self.green)?;
        write_scope(f, "terminal.ansiYellow", self.yellow)?;
        write_scope(f, "terminal.ansiBrightYellow", self.yellow)?;
        write_scope(f, "terminal.ansiBlue", self.blue)?;
        write_scope(f, "terminal.ansiBrightBlue", self.blue)?;
        write_scope(f, "terminal.ansiMagenta", self.purple)?;
        write_scope(f, "terminal.ansiBrightMagenta", self.purple)?;
        write_scope(f, "terminal.ansiCyan", self.teal)?;
        write_scope(f, "terminal.ansiBrightCyan", self.teal)?;
        write_scope(f, "terminal.ansiWhite", self.foreground)?;
        write_scope(f, "terminal.ansiBrightWhite", self.foreground)?;

        write_scope(f, "rust_analyzer.inlayHints.foreground", self.out_of_focus)?;

        writeln!(f, "}},")?;

        Ok(())
    }

    fn semantic_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\"semanticHighlighting\": true,")?;
        writeln!(f, "\"semanticTokenColors\": {{")?;

        write_scope(f, "type", self.yellow)?;
        write_scope(f, "struct", self.yellow)?;
        write_scope(f, "enum", self.yellow)?;
        write_scope(f, "property", self.red)?;
        write_scope(f, "number", self.red)?;
        write_scope(f, "string", self.red)?;
        write_scope(f, "keyword", self.purple)?;
        write_scope(f, "function", self.blue)?;
        write_scope(f, "interface", self.green)?;
        write_scope(f, "enumMember", self.green)?;

        write_scope(f, "comment", self.out_of_focus)?;

        writeln!(f, "}},")?;

        Ok(())
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\"name\": \"Wordsmith {}\",", self.name)?;
        self.workbench_colors(f)?;
        self.semantic_colors(f)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

fn write_scope(f: &mut fmt::Formatter<'_>, scope_name: &str, value: impl Into<Rgba>) -> fmt::Result {
    writeln!(f, "\"{}\": {},", scope_name, value.into())
}

struct Rgba {
    rgb: Rgb,
    a: u8,
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Self { rgb, a: 0xFF }
    }
}

impl From<(Rgb, u8)> for Rgba {
    fn from((rgb, a): (Rgb, u8)) -> Self {
        Self { rgb, a }
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 0xFF {
            write!(f, "\"#{:06X}\"", self.rgb.0)
        } else {
            write!(f, "\"#{:06X}{:02X}\"", self.rgb.0, self.a)
        }
    }
}

#[derive(Copy, Clone)]
struct Rgb(u32);
