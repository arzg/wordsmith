use std::path::PathBuf;
use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    fs::write(LIGHT.path(), LIGHT.to_string())
}

const LIGHT: Theme = Theme {
    name: "Light",
    background: Rgb(0xF7F7F7),
};

struct Theme {
    name: &'static str,
    background: Rgb,
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
        writeln!(f, "\"editor.background\": {}", self.background)?;
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
