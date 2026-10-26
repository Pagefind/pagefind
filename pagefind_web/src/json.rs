use std::borrow::Cow;

use hifijson::escape::Lex as _;
use hifijson::num::LexWrite as _;
use hifijson::str::{self, LexWrite as _};
use hifijson::token::Lex as _;
use hifijson::{Error, Expect, Read, SliceLexer};

const MAX_DEPTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Num(f32),
    Str(Cow<'a, str>),
    Arr(Vec<Value<'a>>),
    Obj(Vec<(Cow<'a, str>, Value<'a>)>),
}

impl<'a> Value<'a> {
    pub fn parse(src: &'a str) -> Option<Self> {
        let mut lx = SliceLexer::new(src.as_bytes());
        lx.exactly_one(SliceLexer::ws_peek, |next, lx| value(MAX_DEPTH, next, lx))
            .ok()
    }

    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        self.as_obj()?
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_obj(&self) -> Option<&[(Cow<'a, str>, Value<'a>)]> {
        match self {
            Value::Obj(o) => Some(o),
            _ => None,
        }
    }
}

fn value<'a>(depth: usize, next: u8, lx: &mut SliceLexer<'a>) -> Result<Value<'a>, Error> {
    let depth = depth.checked_sub(1).ok_or(Error::Depth)?;
    Ok(match next {
        b'a'..=b'z' => match lx.null_or_bool().ok_or(Expect::Value)? {
            None => Value::Null,
            Some(b) => Value::Bool(b),
        },
        b'0'..=b'9' | b'-' => {
            let (num, _) = lx.num_bytes().validated()?;
            Value::Num(to_f32(num))
        }
        b'"' => Value::Str(string(lx.discarded())?),
        b'[' => {
            let mut arr = Vec::new();
            lx.discarded().seq(b']', SliceLexer::ws_peek, |next, lx| {
                arr.push(value(depth, next, lx)?);
                Ok::<_, Error>(())
            })?;
            Value::Arr(arr)
        }
        b'{' => {
            let mut obj = Vec::new();
            lx.discarded().seq(b'}', SliceLexer::ws_peek, |next, lx| {
                lx.expect(|_| Some(next), b'"').ok_or(Expect::String)?;
                let key = string(lx)?;
                lx.expect(SliceLexer::ws_peek, b':').ok_or(Expect::Colon)?;
                let next = lx.ws_peek().ok_or(Expect::Value)?;
                obj.push((key, value(depth, next, lx)?));
                Ok::<_, Error>(())
            })?;
            Value::Obj(obj)
        }
        _ => Err(Expect::Value)?,
    })
}

// Like hifijson's `str_string`, but lone surrogates become U+FFFD rather than
// failing the whole document, matching how JS strings cross into wasm.
fn string<'a>(lx: &mut SliceLexer<'a>) -> Result<Cow<'a, str>, str::Error> {
    let on_string = |bytes: &mut &'a [u8], out: &mut Cow<'a, str>| {
        match std::str::from_utf8(bytes).map_err(str::Error::Utf8)? {
            "" => {}
            s if out.is_empty() => *out = Cow::Borrowed(s),
            s => out.to_mut().push_str(s),
        }
        Ok(())
    };
    lx.str_fold(Cow::Borrowed(""), on_string, |lx, out| {
        let out = out.to_mut();
        match lx.take_next().ok_or(str::Error::Eof)? {
            b'u' => {
                let mut unit = lx.hex::<u16>().map_err(str::Error::Escape)?;
                while (0xD800..0xDC00).contains(&unit) && lx.strip_prefix(b"\\u") {
                    let low = lx.hex::<u16>().map_err(str::Error::Escape)?;
                    if (0xDC00..0xE000).contains(&low) {
                        let c = 0x10000 + ((unit as u32 - 0xD800) << 10) + (low as u32 - 0xDC00);
                        out.extend(char::from_u32(c));
                        return Ok(());
                    }
                    out.push('\u{FFFD}');
                    unit = low;
                }
                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            }
            c => out.push(lx.escape(c).map_err(str::Error::Escape)?),
        }
        Ok(())
    })
}

// Using `str::parse::<f32>` here pulls `core::num::dec2flt` into the wasm,
// which costs ~14kb gzipped. Ranking weights don't need perfect rounding.
fn to_f32(num: &[u8]) -> f32 {
    let (mut mantissa, mut exp, mut e, mut e_neg) = (0u64, 0i32, 0i32, false);
    let (mut in_frac, mut in_exp) = (false, false);
    for &b in num {
        match b {
            b'.' => in_frac = true,
            b'e' | b'E' => in_exp = true,
            b'-' if in_exp => e_neg = true,
            b'0'..=b'9' if in_exp => e = (e * 10 + (b - b'0') as i32).min(9999),
            b'0'..=b'9' if mantissa < 1_000_000_000_000_000_000 => {
                mantissa = mantissa * 10 + (b - b'0') as u64;
                exp -= in_frac as i32;
            }
            b'0'..=b'9' => exp += !in_frac as i32,
            _ => {}
        }
    }
    if mantissa == 0 {
        return if num[0] == b'-' { -0.0 } else { 0.0 };
    }
    exp += if e_neg { -e } else { e };
    let mut scale = 1.0f64;
    for _ in 0..exp.unsigned_abs().min(400) {
        scale *= 10.0;
    }
    let v = if exp < 0 {
        mantissa as f64 / scale
    } else {
        mantissa as f64 * scale
    };
    if num[0] == b'-' {
        -v as f32
    } else {
        v as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numbers() {
        for num in [
            "0", "-0", "2.5", "0.1", "-7.5", "1e1", "5E-1", "1.5e+2", "0e400", "1e39",
        ] {
            assert_eq!(
                Value::parse(num),
                Some(Value::Num(num.parse().unwrap())),
                "{num}"
            );
        }
    }

    #[test]
    fn decodes_escapes() {
        assert_eq!(
            Value::parse(r#""d\u006fcs \"\ud83d\ude00\"""#),
            Some(Value::Str("docs \"😀\"".into()))
        );
        assert_eq!(
            Value::parse(r#""\ud800 \ud800\u0041 \udc00""#),
            Some(Value::Str("\u{FFFD} \u{FFFD}A \u{FFFD}".into()))
        );
    }

    #[test]
    fn rejects_malformed_input() {
        let deep = "[".repeat(100_000);
        for input in [
            "",
            "{",
            "[1,]",
            "{\"a\":1}x",
            "{\"category\":\"docs\"",
            "01",
            &deep,
        ] {
            assert_eq!(Value::parse(input), None, "{input}");
        }
    }
}
