use std::collections::HashSet;
use fancy_regex::Regex;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CgrcAttrib {
    CgrcNone = -1,
    CgrcReset = 0,
    CgrcBright = 1,
    CgrcDim = 2,
    CgrcItalic = 3,
    CgrcUnderline = 4,
    CgrcBlink = 5,
    CgrcRapidBlink = 6,
    CgrcReverse = 7,
    CgrcHidden = 8,
    CgrcStrikethrough = 9,
}

#[derive(Debug, Clone)]
pub enum CgrcResetAttrib {
    CgrcResetNone = 0,
    CgrcResetBright = 21,
    CgrcResetDim = 22,
    CgrcResetItalic = 23,
    CgrcResetUnderline = 24,
    CgrcResetBlink = 25,
    CgrcResetRapidBlink = 26,
    CgrcResetReverse = 27,
    CgrcResetHidden = 28,
    CgrcResetStrikethrough = 29,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CgrcCountMode {
    CgrcCountOnce,
    CgrcCountMore,
    CgrcCountStop,
    CgrcCountPrevious,
    CgrcCountBlock,
    CgrcCountUnblock,
}

#[derive(Debug, Copy, Clone)]
pub enum CgrcColor {
    LcBlack = 0,
    LcRed = 1,
    LcGreen = 2,
    LcYellow = 3,
    LcBlue = 4,
    LcMagenta = 5,
    LcCyan = 6,
    LcWhite = 7,
    LcDefault = 9,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LcBackColor {
    LcBackColBlack = 40 + CgrcColor::LcBlack as isize,
    LcBackColRed = 40 + CgrcColor::LcRed as isize,
    LcBackColGreen = 40 + CgrcColor::LcGreen as isize,
    LcBackColYellow = 40 + CgrcColor::LcYellow as isize,
    LcBackColBlue = 40 + CgrcColor::LcBlue as isize,
    LcBackColMagenta = 40 + CgrcColor::LcMagenta as isize,
    LcBackColCyan = 40 + CgrcColor::LcCyan as isize,
    LcBackColWhite = 40 + CgrcColor::LcWhite as isize,
    LcBackBrightColBlack = 100 + CgrcColor::LcBlack as isize,
    LcBackBrightColRed = 100 + CgrcColor::LcRed as isize,
    LcBackBrightColGreen = 100 + CgrcColor::LcGreen as isize,
    LcBackBrightColYellow = 100 + CgrcColor::LcYellow as isize,
    LcBackBrightColBlue = 100 + CgrcColor::LcBlue as isize,
    LcBackBrightColMagenta = 100 + CgrcColor::LcMagenta as isize,
    LcBackBrightColCyan = 100 + CgrcColor::LcCyan as isize,
    LcBackBrightColWhite = 100 + CgrcColor::LcWhite as isize,
    LcBackColDefault = 40 + CgrcColor::LcDefault as isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LcLogColor {
    LcForgColBlack = 30 + CgrcColor::LcBlack as isize,
    LcForgColRed = 30 + CgrcColor::LcRed as isize,
    LcForgColGreen = 30 + CgrcColor::LcGreen as isize,
    LcForgColYellow = 30 + CgrcColor::LcYellow as isize,
    LcForgColBlue = 30 + CgrcColor::LcBlue as isize,
    LcForgColMagenta = 30 + CgrcColor::LcMagenta as isize,
    LcForgColCyan = 30 + CgrcColor::LcCyan as isize,
    LcForgColWhite = 30 + CgrcColor::LcWhite as isize,
    LcForgBrightColBlack = 90 + CgrcColor::LcBlack as isize,
    LcForgBrightColRed = 90 + CgrcColor::LcRed as isize,
    LcForgBrightColGreen = 90 + CgrcColor::LcGreen as isize,
    LcForgBrightColYellow = 90 + CgrcColor::LcYellow as isize,
    LcForgBrightColBlue = 90 + CgrcColor::LcBlue as isize,
    LcForgBrightColMagenta = 90 + CgrcColor::LcMagenta as isize,
    LcForgBrightColCyan = 90 + CgrcColor::LcCyan as isize,
    LcForgBrightColWhite = 90 + CgrcColor::LcWhite as isize,
    LcForgColDefault = 30 + CgrcColor::LcDefault as isize,
}

pub static COLORS_ATTRS: phf::Map<&'static str, CgrcAttrib> = phf::phf_map! {
    "none" => CgrcAttrib::CgrcNone,
    "unchanged" => CgrcAttrib::CgrcNone,
    "default" => CgrcAttrib::CgrcReset,
    "bold" => CgrcAttrib::CgrcBright,
    "underline" => CgrcAttrib::CgrcUnderline,
    "blink" => CgrcAttrib::CgrcBlink,
    "reverse" => CgrcAttrib::CgrcReverse,
    "concealed" => CgrcAttrib::CgrcHidden,
    "dark" => CgrcAttrib::CgrcDim,
    "italic" => CgrcAttrib::CgrcItalic,
    "rapidblink" => CgrcAttrib::CgrcRapidBlink,
    "strikethrough" => CgrcAttrib::CgrcStrikethrough,
};

pub static COLORS_BACK: phf::Map<&'static str, LcBackColor> = phf::phf_map! {
    "on_black" => LcBackColor::LcBackColBlack,
    "on_red" => LcBackColor::LcBackColRed,
    "on_green" => LcBackColor::LcBackColGreen,
    "on_yellow" => LcBackColor::LcBackColYellow,
    "on_blue" => LcBackColor::LcBackColBlue,
    "on_magenta" => LcBackColor::LcBackColMagenta,
    "on_cyan" => LcBackColor::LcBackColCyan,
    "on_white" => LcBackColor::LcBackColWhite,
    "on_bright_black" => LcBackColor::LcBackBrightColBlack,
    "on_bright_red" => LcBackColor::LcBackBrightColRed,
    "on_bright_green" => LcBackColor::LcBackBrightColGreen,
    "on_bright_yellow" => LcBackColor::LcBackBrightColYellow,
    "on_bright_blue" => LcBackColor::LcBackBrightColBlue,
    "on_bright_magenta" => LcBackColor::LcBackBrightColMagenta,
    "on_bright_cyan" => LcBackColor::LcBackBrightColCyan,
    "on_bright_white" => LcBackColor::LcBackBrightColWhite,
};

pub static COLORS_FORG: phf::Map<&'static str, LcLogColor> = phf::phf_map! {
    "black" => LcLogColor::LcForgColBlack,
    "red" => LcLogColor::LcForgColRed,
    "green" => LcLogColor::LcForgColGreen,
    "yellow" => LcLogColor::LcForgColYellow,
    "blue" => LcLogColor::LcForgColBlue,
    "magenta" => LcLogColor::LcForgColMagenta,
    "cyan" => LcLogColor::LcForgColCyan,
    "white" => LcLogColor::LcForgColWhite,
    "bright_black" => LcLogColor::LcForgBrightColBlack,
    "bright_red" => LcLogColor::LcForgBrightColRed,
    "bright_green" => LcLogColor::LcForgBrightColGreen,
    "bright_yellow" => LcLogColor::LcForgBrightColYellow,
    "bright_blue" => LcLogColor::LcForgBrightColBlue,
    "bright_magenta" => LcLogColor::LcForgBrightColMagenta,
    "bright_cyan" => LcLogColor::LcForgBrightColCyan,
    "bright_white" => LcLogColor::LcForgBrightColWhite,
};

#[derive(Clone, Debug)]
pub struct CgrcColorItem {
    pub attrs: HashSet<CgrcAttrib>,
    pub forg: LcLogColor,
    pub back: LcBackColor,
    pub escape_seq: String,
    pub clear_seq: String,
}

impl CgrcColorItem {
    pub fn new(attrs: HashSet<CgrcAttrib>, forg: LcLogColor, back: LcBackColor) -> CgrcColorItem {
        let escape_seq = CgrcColorItem::build_escape_seq(&attrs, &forg, &back);
        let clear_seq = CgrcColorItem::build_clear_seq(&attrs);
        CgrcColorItem {
            attrs,
            forg,
            back,
            escape_seq,
            clear_seq,
        }
    }

    fn build_escape_seq(attrs: &HashSet<CgrcAttrib>, forg: &LcLogColor, back: &LcBackColor) -> String {
        let mut seq = format!("{}[{};{}", 0x1b as char, *forg as u8, *back as u8);
        for attr in attrs {
            seq += &format!(";{}", *attr as u8);
        }
        seq += "m";
        seq
    }

    fn build_clear_seq(attrs: &HashSet<CgrcAttrib>) -> String {
        let mut seq = format!(
            "{}[{};{}",
            0x1b as char,
            LcLogColor::LcForgColDefault as u8,
            LcBackColor::LcBackColDefault as u8
        );
        for attr in attrs {
            seq += &format!(";{}", colors_attr_clear(&attr) as u8);
        }
        seq += "m";
        seq
    }
}

#[derive(Clone, Debug)]
pub struct CgrcConfItem {
    pub regex: Option<Regex>,
    pub colors: Vec<CgrcColorItem>,
    pub skip: Option<bool>,
    pub count_mode: Option<CgrcCountMode>,
}

impl CgrcConfItem {
    pub fn new() -> CgrcConfItem {
        CgrcConfItem {
            regex: None,
            colors: vec![],
            skip: None,
            count_mode: None,
        }
    }
}

impl PartialEq for CgrcConfItem {
    fn eq(&self, other: &Self) -> bool {
        match &self.regex {
            None => match other.regex {
                None => true,
                Some(_) => false,
            },
            Some(r1) => match &other.regex {
                None => false,
                Some(r2) => r1.as_str() == r2.as_str(),
            },
        }
    }
}

pub struct CgrcConf {
    pub items: Vec<CgrcConfItem>,
    pub description: Option<String>,
}

impl CgrcConf {
    pub fn new() -> CgrcConf {
        CgrcConf {
            items: vec![],
            description: None,
        }
    }
}

fn colors_attr_clear(attr: &CgrcAttrib) -> CgrcResetAttrib {
    match attr {
        CgrcAttrib::CgrcBright => CgrcResetAttrib::CgrcResetBright,
        CgrcAttrib::CgrcDim => CgrcResetAttrib::CgrcResetDim,
        CgrcAttrib::CgrcItalic => CgrcResetAttrib::CgrcResetItalic,
        CgrcAttrib::CgrcUnderline => CgrcResetAttrib::CgrcResetUnderline,
        CgrcAttrib::CgrcBlink => CgrcResetAttrib::CgrcResetBlink,
        CgrcAttrib::CgrcRapidBlink => CgrcResetAttrib::CgrcResetRapidBlink,
        CgrcAttrib::CgrcReverse => CgrcResetAttrib::CgrcResetReverse,
        CgrcAttrib::CgrcHidden => CgrcResetAttrib::CgrcResetHidden,
        CgrcAttrib::CgrcStrikethrough => CgrcResetAttrib::CgrcResetStrikethrough,
        _ => CgrcResetAttrib::CgrcResetNone,
    }
}