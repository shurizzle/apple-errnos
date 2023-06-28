use std::{
    collections::{BTreeSet, HashMap},
    fmt,
    io::{BufRead, BufReader, Lines},
};

use atoi::FromRadix10Signed;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};

#[derive(Debug)]
pub enum ParsedErrno {
    Define(Box<str>, i32, Option<Box<str>>),
    Alias(Box<str>, Box<str>, Option<Box<str>>),
    Undef(Box<str>),
}

pub struct Id<'a>(pub &'a str);

impl<'a> Id<'a> {
    const KEYWORDS: &'static [&'static str] = [
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "async",
        "await",
        "dyn",
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "override",
        "priv",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        "try",
        "macro_rules",
        "union",
    ]
    .as_slice();
}

impl<'a> fmt::Display for Id<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Self::KEYWORDS.contains(&self.0) {
            write!(f, "r#{}", self.0)
        } else {
            fmt::Display::fmt(self.0, f)
        }
    }
}

fn parse_line(line: &str) -> Option<ParsedErrno> {
    fn space0(b: &str) -> &str {
        b.trim_start()
    }

    fn space1(b: &str) -> Option<&str> {
        if b.chars().next()?.is_whitespace() {
            Some(b.trim_start())
        } else {
            None
        }
    }

    fn int<T: FromRadix10Signed>(b: &str) -> Option<(T, &str)> {
        let (b, needs_paren) = if *b.as_bytes().first()? == b'(' {
            (space0(unsafe { b.get_unchecked(1..) }), true)
        } else {
            (b, false)
        };

        let (res, size) = T::from_radix_10_signed(b.as_bytes());
        let (res, b) = if size == 0 {
            return None;
        } else {
            (res, unsafe { b.get_unchecked(size..) })
        };

        let b = if needs_paren {
            let b = space0(b);

            if *b.as_bytes().first()? == b')' {
                unsafe { b.get_unchecked(1..) }
            } else {
                return None;
            }
        } else {
            b
        };

        Some((res, b))
    }

    fn id(b: &str) -> Option<(Box<str>, &str)> {
        b.char_indices()
            .find(|&(i, c)| {
                if i == 0 {
                    !c.is_ascii_alphabetic() && c != '_'
                } else {
                    !c.is_ascii_alphanumeric() && c != '_'
                }
            })
            .and_then(|(i, _)| {
                let name = unsafe { b.get_unchecked(..i) };
                if !name.is_empty() && name.starts_with('E') && name != "ELAST" {
                    Some((name.to_string().into_boxed_str(), unsafe {
                        b.get_unchecked(i..)
                    }))
                } else {
                    None
                }
            })
            .or_else(|| {
                if b.is_empty() {
                    None
                } else {
                    Some((b.to_string().into_boxed_str(), unsafe {
                        b.get_unchecked(b.len()..)
                    }))
                }
            })
    }

    fn comment(b: &str) -> Option<Box<str>> {
        let b = b.trim();
        if let Some(b) = b.strip_prefix("/*") {
            let b = b.strip_suffix("*/")?.trim();
            if b.is_empty() {
                None
            } else {
                Some(b.to_string().into_boxed_str())
            }
        } else if let Some(b) = b.strip_prefix("//") {
            let b = b.trim();
            if b.is_empty() {
                None
            } else {
                Some(b.to_string().into_boxed_str())
            }
        } else {
            None
        }
    }

    let line = space0(line);
    let line = line.strip_prefix('#')?;
    let line = space0(line);
    if let Some(line) = line.strip_prefix("define") {
        let line = space1(line)?;
        let (name, line) = id(line)?;
        let line = space1(line)?;

        if let Some((value, line)) = int::<i32>(line) {
            Some(ParsedErrno::Define(name, value, comment(line)))
        } else if line.starts_with('E') {
            let (alias, line) = id(line)?;
            Some(ParsedErrno::Alias(
                name,
                alias.to_string().into_boxed_str(),
                comment(line),
            ))
        } else {
            None
        }
    } else if let Some(line) = line.strip_prefix("undef") {
        let line = space1(line)?;
        let (name, _) = id(line)?;
        Some(ParsedErrno::Undef(name))
    } else {
        None
    }
}

pub struct FileParser<R: BufRead>(Lines<R>);

impl<R: BufRead> Iterator for FileParser<R> {
    type Item = std::io::Result<ParsedErrno>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.0.next()? {
                Ok(line) => line,
                Err(err) => return Some(Err(err)),
            };

            if let Some(inst) = parse_line(&line) {
                return Some(Ok(inst));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bindings {
    pub defines: Vec<(Box<str>, i32, Box<str>)>,
    pub aliases: Vec<(Box<str>, Box<str>)>,
}

impl PartialEq for Bindings {
    fn eq(&self, other: &Self) -> bool {
        if self.defines.len() != other.defines.len() || self.aliases.len() != other.aliases.len() {
            return false;
        }

        for a in &self.defines {
            if let Some(b) = other.defines.iter().find(|(n, _, _)| *n == a.0) {
                if a.1 != b.1 {
                    return false;
                }
            } else {
                return false;
            }
        }

        for a in &self.aliases {
            if let Some(b) = other.aliases.iter().find(|(n, _)| *n == a.0) {
                if a.1 != b.1 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Bindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#![allow(dead_code)]\n")?;
        writeln!(f, "// This file automatically generate. Do not edit.\n")?;
        writeln!(f, "use crate::Errno;")?;

        writeln!(f, "impl Errno {{")?;
        let mut min: Option<i32> = None;
        let mut max: Option<i32> = None;
        let mut errs = Vec::<i32>::new();
        for (name, no, desc) in &self.defines {
            max = max.map_or(Some(*no), |max| Some(max.max(*no)));
            min = min.map_or(Some(*no), |min| Some(min.min(*no)));
            errs.push(*no);
            writeln!(f, "    /// {}", desc)?;
            writeln!(f, "    pub const {}: Self = Self({});", Id(name), no)?;
        }
        for (alias, name) in &self.aliases {
            writeln!(f, "    /// Alias for [Self::{}]", Id(name))?;
            writeln!(f, "    pub const {}: Self = Self::{};", Id(alias), name)?;
        }
        if let Some(min) = min.take() {
            writeln!(f, "\n    pub const MIN: i32 = {};", min)?;
        }
        if let Some(max) = max.take() {
            writeln!(f, "    pub const MAX: i32 = {};", max)?;
        }
        writeln!(
            f,
            "\n#[cfg(feature = \"iter\")]\n    const ALL: [i32; {}] = {:?};",
            errs.len(),
            errs
        )?;
        writeln!(f,
        "\n    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {{"
    )?;
        writeln!(f, "        match self.0 {{")?;
        for (name, no, desc) in &self.defines {
            writeln!(f, "            {} => Some(({:?}, {:?})),", no, name, desc)?;
        }
        writeln!(f, "            _ => None,")?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")
    }
}

pub fn generate_bindings() -> Result<Bindings> {
    let parser = ureq::get(
        "https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/sys/errno.h",
    )
    .call()
    .wrap_err("Failed to fetch kernel")
    .map(|b| FileParser(BufReader::new(b.into_reader()).lines()))?;

    let mut defs = HashMap::new();
    let mut aliases = HashMap::new();
    {
        let mut nos = BTreeSet::new();

        for errno in parser {
            match errno.wrap_err("Failed to parse xnu headers")? {
                ParsedErrno::Alias(name, alias, _doc) => {
                    if name.as_ref() != "EOPNOTSUPP" {
                        if defs.contains_key(&name) {
                            bail!("{} defined multiple times", name);
                        }
                        if let Some(other) = aliases.insert(name, alias) {
                            bail!("{} defined multiple times", other);
                        }
                    }
                }
                ParsedErrno::Define(name, value, doc) => {
                    if aliases.contains_key(&name) {
                        bail!("{} defined multiple times", name);
                    }
                    let desc = if let Some(desc) = doc {
                        desc
                    } else {
                        match name.as_ref() {
                            "EREDRIVEOPEN" => "kernel internal error",
                            "EKEEPLOOKING" => "kernel internal error",
                            "EDATALESS" => "kernel internal error",
                            "ECVCERORR" => "kernel internal error",
                            "ECVPERORR" => "kernel internal error",
                            _ => bail!("No description for {name}"),
                        }
                        .to_string()
                        .into_boxed_str()
                    };

                    if nos.contains(&value) {
                        bail!("Errno {value} ({name}) defined multiple times");
                    } else {
                        nos.insert(value);
                    }

                    if let Some((other, _)) = defs.insert(name, (value, desc)) {
                        bail!("{} defined multiple times", other)
                    }
                }
                ParsedErrno::Undef(name) => {
                    if aliases.remove(&name).is_none() {
                        if let Some((no, _)) = defs.remove(&name) {
                            nos.remove(&no);
                        }
                    }
                }
            }
        }
    }

    let mut aliases = {
        let mut t: Vec<(Box<str>, Box<str>)> = Vec::with_capacity(aliases.len());
        for (a, b) in aliases {
            if !defs.contains_key(&b) {
                bail!("Alias to unknown error {}", b);
            }
            t.push((a, b));
        }
        t
    };
    aliases.sort_by(|a, b| Ord::cmp(a.0.as_ref(), b.0.as_ref()));
    let mut defines: Vec<(Box<str>, i32, Box<str>)> =
        defs.into_iter().map(|(a, (b, c))| (a, b, c)).collect();
    defines.sort_by_key(|t| t.1);

    Ok(Bindings { defines, aliases })
}
