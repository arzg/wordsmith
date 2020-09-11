use std::path::PathBuf;
use std::{fmt, fs, io};

fn main() -> io::Result<()> {
    fs::write(LIGHT.path(), LIGHT.to_string())
}

const LIGHT: Theme = Theme { name: "Light" };

struct Theme {
    name: &'static str,
}

impl Theme {
    fn path(&self) -> PathBuf {
        PathBuf::from(format!("themes/Wordsmith-{}-color-theme.json", self.name))
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\"name\": \"Wordsmith {}\"", self.name)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}
