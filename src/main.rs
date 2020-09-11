use std::path::PathBuf;
use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    fs::write(LIGHT.path(), LIGHT.to_string())
}

const LIGHT: Theme = Theme {
    name: "Light",
    background: Rgb(0xF7F7F7),
    foreground: Rgb(0x1A1A1A),
    borders: Rgb(0xDBDBDB),
    cursor: Rgb(0x16BDEC),
    editor_selection: Rgb(0xBFE8F4),
    ui_selection: Rgb(0xB3D7FF),
};

struct Theme {
    name: &'static str,
    background: Rgb,
    foreground: Rgb,
    borders: Rgb,
    cursor: Rgb,
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
        writeln!(f, "\"editor.background\": {},", self.background)?;
        writeln!(f, "\"editor.foreground\": {},", self.foreground)?;
        writeln!(f, "\"foreground\": {},", self.foreground)?;

        writeln!(f, "\"statusBar.foreground\": {},", self.foreground)?;
        writeln!(f, "\"statusBar.background\": {},", self.background)?;
        writeln!(f, "\"statusBar.debuggingBackground\": {},", self.background)?;
        writeln!(f, "\"statusBar.noFolderBackground\": {},", self.background)?;
        writeln!(f, "\"statusBar.border\": {},", self.borders)?;

        writeln!(f, "\"editorCursor.background\": {},", self.cursor)?;
        writeln!(f, "\"editorCursor.foreground\": {},", self.cursor)?;

        writeln!(f, "\"editor.selectionBackground\": {},", self.editor_selection)?;
        writeln!(f, "\"selection.background\": {},", self.ui_selection)?;
        writeln!(f, "}}")?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}\"", self.0)
    }
}
