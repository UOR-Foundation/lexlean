#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) {
    WORK.fetch_add(units, Memory::Relaxed);
}

pub fn work() -> u64 {
    WORK.load(Memory::Relaxed)
}

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) {
            Ordering::Equal => self.1.key(&other.1),
            decided => decided,
        }
    }
}

pub fn equal<T: Same>(a: T, b: T) -> bool {
    a.same(&b)
}

pub fn compare<T: Key>(a: T, b: T) -> Ordering {
    a.key(&b)
}

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => Ok(v0),
        None => Err::<u64, Overflow>(Overflow),
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => Ok(v0),
        None => Err::<u64, Overflow>(Overflow),
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => Ok(v0),
        None => Err::<u64, Overflow>(Overflow),
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => Ok(v0),
        None => Err::<i64, Overflow>(Overflow),
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => Ok(v0),
        None => Err::<i64, Overflow>(Overflow),
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => Ok(v0),
        None => Err::<i64, Overflow>(Overflow),
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => Ok(v0),
        None => Err::<i64, Overflow>(Overflow),
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    {
        let value = {
            let (a, b, z): (i64, i64, i64) = (p0, p1, p2);
            if b == 0 {
                Some(z)
            } else {
                a.checked_div(b)
            }
        };
        match value {
            Some(v0) => Ok(v0),
            None => Err::<i64, Overflow>(Overflow),
        }
    }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    {
        let (a, b, z): (i64, i64, i64) = (p0, p1, p2);
        if b == 0 {
            z
        } else {
            a.wrapping_rem(b)
        }
    }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}
use core::str;

use std::mem;

use std::rc::Rc;

#[derive(Clone)]
pub struct Str(Rc<str>);

impl Str {
    pub fn lit(text: &str) -> Str {
        Str(Rc::from(text))
    }
    pub fn text(&self) -> &str {
        &self.0
    }
    pub fn length(&self) -> u64 {
        chars(&self.0)
    }
    pub fn utf8_encode(&self) -> Bytes {
        tick(self.0.len() as u64);
        Bytes::lit(self.0.as_bytes())
    }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> {
        if delimiter.0.is_empty() {
            return None;
        };
        chars(&self.0);
        let mut fields: Vec<Str> = Vec::new();
        for piece in self.0.split(&*delimiter.0) {
            fields.push(Str::lit(piece));
        }
        if fields.len() as u128 > u128::from(maximum) {
            return None;
        };
        Some(List::onto(fields, List::nil()))
    }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str {
        let mut out = String::new();
        let mut first = true;
        let mut cursor = &texts;
        while let Repr::Cell(cell) = &cursor.0 {
            tick(1);
            if !first {
                chars(&delimiter.0);
                out.push_str(&delimiter.0);
            };
            first = false;
            chars(&cell.head.0);
            out.push_str(&cell.head.0);
            cursor = &cell.tail;
        }
        match &cursor.0 {
            Repr::Run { buffer, len } => {
                for text in buffer[..*len].iter().rev() {
                    tick(1);
                    if !first {
                        chars(&delimiter.0);
                        out.push_str(&delimiter.0);
                    };
                    first = false;
                    chars(&text.0);
                    out.push_str(&text.0);
                }
            }
            _ => {}
        };
        Str::lit(&out)
    }
    pub fn parse_int(text: &Str) -> Option<Option<i64>> {
        match canonical_decimal(&text.0) {
            None => Some(None),
            Some(Some(n)) => match i64::try_from(n) {
                Ok(value) => Some(Some(value)),
                Err(_) => None,
            },
            Some(None) => None,
        }
    }
    pub fn format_u8(a: u8) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_u8(text: &Str) -> Option<u8> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => u8::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_u16(a: u16) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_u16(text: &Str) -> Option<u16> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => u16::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_u32(a: u32) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_u32(text: &Str) -> Option<u32> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => u32::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_u64(a: u64) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_u64(text: &Str) -> Option<u64> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => u64::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_i8(a: i8) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_i8(text: &Str) -> Option<i8> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => i8::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_i16(a: i16) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_i16(text: &Str) -> Option<i16> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => i16::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_i32(a: i32) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_i32(text: &Str) -> Option<i32> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => i32::try_from(n).ok(),
            _ => None,
        }
    }
    pub fn format_i64(a: i64) -> Str {
        let text = a.to_string();
        tick(text.len() as u64);
        Str::lit(&text)
    }
    pub fn parse_i64(text: &Str) -> Option<i64> {
        match canonical_decimal(&text.0) {
            Some(Some(n)) => i64::try_from(n).ok(),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes {
        Bytes(Rc::from(octets))
    }
    pub fn octets(&self) -> &[u8] {
        &self.0
    }
    pub fn append(a: Bytes, b: Bytes) -> Bytes {
        tick((a.0.len() + b.0.len()) as u64);
        let mut out = Vec::with_capacity(a.0.len() + b.0.len());
        out.extend_from_slice(&a.0);
        out.extend_from_slice(&b.0);
        Bytes(Rc::from(out))
    }
    pub fn length(&self) -> u64 {
        self.0.len() as u64
    }
    pub fn index(&self, i: u64) -> Option<u8> {
        match usize::try_from(i) {
            Ok(at) => self.0.get(at).copied(),
            Err(_) => None,
        }
    }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> {
        let end = u128::from(start) + u128::from(count);
        if end > self.0.len() as u128 {
            return None;
        };
        tick(count);
        Some(Bytes::lit(&self.0[start as usize..end as usize]))
    }
    pub fn utf8_decode(&self) -> Option<Str> {
        tick(self.0.len() as u64);
        match str::from_utf8(&self.0) {
            Ok(text) => Some(Str::lit(text)),
            Err(_) => None,
        }
    }
    pub fn compare(&self, other: &Bytes) -> Ordering {
        tick(1 + self.0.len().min(other.0.len()) as u64);
        self.0.cmp(&other.0)
    }
}

pub struct Node<T> {
    head: T,
    tail: List<T>,
}

pub enum Repr<T> {
    Nil,
    Run { buffer: Rc<Vec<T>>, len: usize },
    Cell(Rc<Node<T>>),
}

pub struct List<T>(Repr<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(match &self.0 {
            Repr::Nil => Repr::Nil,
            Repr::Run { buffer, len } => Repr::Run {
                buffer: buffer.clone(),
                len: *len,
            },
            Repr::Cell(cell) => Repr::Cell(cell.clone()),
        })
    }
}

impl<T> List<T> {
    fn take(&mut self) -> Repr<T> {
        mem::replace(&mut self.0, Repr::Nil)
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = match self.tail.take() {
            Repr::Cell(cell) => Some(cell),
            _ => None,
        };
        while let Some(cell) = next {
            match Rc::try_unwrap(cell) {
                Ok(mut node) => {
                    next = match node.tail.take() {
                        Repr::Cell(cell) => Some(cell),
                        _ => None,
                    }
                }
                Err(_) => break,
            }
        }
    }
}

impl<T: Clone> List<T> {
    pub fn nil() -> Self {
        List(Repr::Nil)
    }
    pub fn cons(head: T, mut tail: List<T>) -> Self {
        List(match tail.take() {
            Repr::Nil => {
                let mut buffer = Vec::with_capacity(16);
                buffer.push(head);
                Repr::Run {
                    buffer: Rc::new(buffer),
                    len: 1,
                }
            }
            Repr::Run { mut buffer, len } => match Rc::get_mut(&mut buffer) {
                Some(cells) => {
                    cells.truncate(len);
                    cells.push(head);
                    Repr::Run {
                        buffer,
                        len: len + 1,
                    }
                }
                None => Repr::Cell(Rc::new(Node {
                    head,
                    tail: List(Repr::Run { buffer, len }),
                })),
            },
            rest => Repr::Cell(Rc::new(Node {
                head,
                tail: List(rest),
            })),
        })
    }
    pub fn uncons(&self) -> Option<(T, List<T>)> {
        match &self.0 {
            Repr::Nil => None,
            Repr::Run { buffer, len } => {
                let head = buffer[len - 1].clone();
                Some((
                    head,
                    List(if *len == 1 {
                        Repr::Nil
                    } else {
                        Repr::Run {
                            buffer: buffer.clone(),
                            len: len - 1,
                        }
                    }),
                ))
            }
            Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())),
        }
    }
    pub fn length(&self) -> u64 {
        let mut count = 0;
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 {
            tick(1);
            count += 1;
            cursor = &cell.tail;
        }
        match &cursor.0 {
            Repr::Run { buffer, len } => {
                for _ in buffer[..*len].iter().rev() {
                    tick(1);
                    count += 1;
                }
            }
            _ => {}
        };
        count
    }
    pub fn append(a: List<T>, b: List<T>) -> List<T> {
        List::onto(a.items(), b)
    }
    pub fn index(&self, i: u64) -> Option<T> {
        let mut position = 0;
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 {
            tick(1);
            if position == i {
                return Some(cell.head.clone());
            };
            position += 1;
            cursor = &cell.tail;
        }
        match &cursor.0 {
            Repr::Run { buffer, len } => {
                for item in buffer[..*len].iter().rev() {
                    tick(1);
                    if position == i {
                        return Some(item.clone());
                    };
                    position += 1;
                }
            }
            _ => {}
        };
        None
    }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> {
        let end = u128::from(start) + u128::from(count);
        let mut part = Vec::new();
        let mut position: u128 = 0;
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 {
            tick(1);
            if position >= end {
                return Some(List::onto(part, List::nil()));
            };
            if position >= u128::from(start) {
                part.push(cell.head.clone());
            };
            position += 1;
            cursor = &cell.tail;
        }
        match &cursor.0 {
            Repr::Run { buffer, len } => {
                for item in buffer[..*len].iter().rev() {
                    tick(1);
                    if position >= end {
                        return Some(List::onto(part, List::nil()));
                    };
                    if position >= u128::from(start) {
                        part.push(item.clone());
                    };
                    position += 1;
                }
            }
            _ => {}
        };
        if position < end {
            None
        } else {
            Some(List::onto(part, List::nil()))
        }
    }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 {
            tick(1);
            out.push(cell.head.clone());
            cursor = &cell.tail;
        }
        match &cursor.0 {
            Repr::Run { buffer, len } => {
                for item in buffer[..*len].iter().rev() {
                    tick(1);
                    out.push(item.clone());
                }
            }
            _ => {}
        };
        out
    }
    fn onto(mut items: Vec<T>, tail: List<T>) -> List<T> {
        let mut out = tail;
        while let Some(item) = items.pop() {
            tick(1);
            out = List::cons(item, out);
        }
        out
    }
}

pub fn chars(text: &str) -> u64 {
    let mut count = 0;
    for _ in text.chars() {
        tick(1);
        count += 1;
    }
    count
}

impl Same for Str {
    fn same(&self, other: &Self) -> bool {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) {
                (None, None) => return true,
                (a, b) if a != b => return false,
                _ => {}
            }
        }
    }
}

impl Same for Bytes {
    fn same(&self, other: &Self) -> bool {
        tick(1 + self.0.len().min(other.0.len()) as u64);
        self.0 == other.0
    }
}

impl Key for Str {
    fn key(&self, other: &Self) -> Ordering {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(&b) {
                    Ordering::Equal => {}
                    decided => return decided,
                },
            }
        }
    }
}

pub fn canonical_decimal(text: &str) -> Option<Option<i128>> {
    let digits = match text.strip_prefix('-') {
        Some(rest) => rest,
        None => text,
    };
    let mut canonical =
        !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0";
    for c in digits.chars() {
        tick(1);
        canonical = canonical && c.is_ascii_digit();
    }
    if !canonical {
        return None;
    };
    Some(text.parse::<i128>().ok())
}

pub fn append_list<T: Clone>(p0: List<T>, p1: List<T>) -> List<T> {
    List::append(p0, p1)
}

pub fn length_list<T: Clone>(p0: List<T>) -> u64 {
    List::length(&p0)
}

pub fn index_list<T: Clone>(p0: List<T>, p1: u64) -> Option<T> {
    List::index(&p0, p1)
}

pub fn slice_list<T: Clone>(p0: List<T>, p1: u64, p2: u64) -> Option<List<T>> {
    List::slice(&p0, p1, p2)
}

pub fn append_bytes(p0: Bytes, p1: Bytes) -> Bytes {
    Bytes::append(p0, p1)
}

pub fn length_bytes(p0: Bytes) -> u64 {
    Bytes::length(&p0)
}

pub fn length_string(p0: Str) -> u64 {
    Str::length(&p0)
}

pub fn index_bytes(p0: Bytes, p1: u64) -> Option<u8> {
    Bytes::index(&p0, p1)
}

pub fn slice_bytes(p0: Bytes, p1: u64, p2: u64) -> Option<Bytes> {
    Bytes::slice(&p0, p1, p2)
}

pub fn utf8_encode(p0: Str) -> Bytes {
    Str::utf8_encode(&p0)
}

pub fn utf8_decode(p0: Bytes) -> Option<Str> {
    Bytes::utf8_decode(&p0)
}

pub fn compare_bytes(p0: Bytes, p1: Bytes) -> Ordering {
    Bytes::compare(&p0, &p1)
}

pub fn split_exact(p0: Str, p1: Str, p2: u32) -> Option<List<Str>> {
    Str::split_exact(&p0, &p1, p2)
}

pub fn join(p0: List<Str>, p1: Str) -> Str {
    Str::join(p0, &p1)
}

pub fn format_u8(p0: u8) -> Str {
    Str::format_u8(p0)
}

pub fn parse_u8(p0: Str) -> Option<u8> {
    Str::parse_u8(&p0)
}

pub fn format_u16(p0: u16) -> Str {
    Str::format_u16(p0)
}

pub fn parse_u16(p0: Str) -> Option<u16> {
    Str::parse_u16(&p0)
}

pub fn format_u32(p0: u32) -> Str {
    Str::format_u32(p0)
}

pub fn parse_u32(p0: Str) -> Option<u32> {
    Str::parse_u32(&p0)
}

pub fn format_u64(p0: u64) -> Str {
    Str::format_u64(p0)
}

pub fn parse_u64(p0: Str) -> Option<u64> {
    Str::parse_u64(&p0)
}

pub fn format_i8(p0: i8) -> Str {
    Str::format_i8(p0)
}

pub fn parse_i8(p0: Str) -> Option<i8> {
    Str::parse_i8(&p0)
}

pub fn format_i16(p0: i16) -> Str {
    Str::format_i16(p0)
}

pub fn parse_i16(p0: Str) -> Option<i16> {
    Str::parse_i16(&p0)
}

pub fn format_i32(p0: i32) -> Str {
    Str::format_i32(p0)
}

pub fn parse_i32(p0: Str) -> Option<i32> {
    Str::parse_i32(&p0)
}

pub fn format_i64(p0: i64) -> Str {
    Str::format_i64(p0)
}

pub fn parse_i64(p0: Str) -> Option<i64> {
    Str::parse_i64(&p0)
}

pub fn parse_int(p0: Str) -> R<Option<i64>> {
    match Str::parse_int(&p0) {
        Some(v0) => Ok(v0),
        None => Err::<Option<i64>, Overflow>(Overflow),
    }
}

pub fn format_int(p0: i64) -> Str {
    Str::format_i64(p0)
}

#[derive(Clone)]
pub enum Adt0 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt0, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn0::F7 => f7(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F10(Bytes, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn1::F10(k0, k1) => f10(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn2::F22 => f22(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F24(u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn3::F24(k0, k1, k2) => f24(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F28,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F31,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn5::F31 => f31(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    let a2 = {
        let a1 = Str::lit("blake3:");
        utf8_encode(a1)
    };
    let a3 = f1(v0)?;
    Ok(append_bytes(a2, a3))
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f2(v0)?;
    let a24 = f3(match v1.clone() {
        Adt0::C0(h4, _, _, _, _, _, _, _) => h4,
    });
    let a25 = {
        let a22 = f3(match v1.clone() {
            Adt0::C0(_, h5, _, _, _, _, _, _) => h5,
        });
        let a23 = {
            let a20 = f3(match v1.clone() {
                Adt0::C0(_, _, h6, _, _, _, _, _) => h6,
            });
            let a21 = {
                let a18 = f3(match v1.clone() {
                    Adt0::C0(_, _, _, h7, _, _, _, _) => h7,
                });
                let a19 = {
                    let a16 = f3(match v1.clone() {
                        Adt0::C0(_, _, _, _, h8, _, _, _) => h8,
                    });
                    let a17 = {
                        let a14 = f3(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h9, _, _) => h9,
                        });
                        let a15 = {
                            let a12 = f3(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h10, _) => h10,
                            });
                            let a13 = f3(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h11) => h11,
                            });
                            append_bytes(a12, a13)
                        };
                        append_bytes(a14, a15)
                    };
                    append_bytes(a16, a17)
                };
                append_bytes(a18, a19)
            };
            append_bytes(a20, a21)
        };
        append_bytes(a22, a23)
    };
    Ok(append_bytes(a24, a25))
}

pub fn f2(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a26 = v0.clone();
        length_bytes(a26)
    };
    let v2: u64 = {
        let a27 = v1;
        let a28 = 0u64;
        if nat_eq(a27, a28) {
            1u64
        } else {
            let a31 = {
                let a29 = v1;
                let a30 = 1023u64;
                nat_add(a29, a30)?
            };
            let a32 = 1024u64;
            let a33 = 0u64;
            nat_quot(a31, a32, a33)
        }
    };
    let v3: u64 = {
        let a34 = v2;
        let a35 = 1u64;
        nat_sub(a34, a35)
    };
    let m37 = {
        let (_, h36) = f4(v0.clone(), v1, v3)?;
        h36
    };
    match m37.uncons() {
        None => f5(v0, v1, v3, true),
        Some((v4, v5)) => {
            let (_, h38) = f6(Fn0::F7, 64u64, (List::cons(v4, v5), f5(v0, v1, v3, false)?));
            Ok(h38)
        }
    }
}

pub fn f3(v0: u32) -> Bytes {
    let a51 = {
        let a39 = f8(v0, 4u32);
        let a40 = f8(v0, 0u32);
        append_bytes(a39, a40)
    };
    let a52 = {
        let a49 = {
            let a41 = f8(v0, 12u32);
            let a42 = f8(v0, 8u32);
            append_bytes(a41, a42)
        };
        let a50 = {
            let a47 = {
                let a43 = f8(v0, 20u32);
                let a44 = f8(v0, 16u32);
                append_bytes(a43, a44)
            };
            let a48 = {
                let a45 = f8(v0, 28u32);
                let a46 = f8(v0, 24u32);
                append_bytes(a45, a46)
            };
            append_bytes(a47, a48)
        };
        append_bytes(a49, a50)
    };
    append_bytes(a51, a52)
}

pub fn f4(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f9(Fn1::F10(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a53 = 1024u64;
        let a54 = v2;
        nat_mul(a53, a54)?
    };
    let v5: u64 = f11(v1, v4);
    let v6: u64 = f12(v5)?;
    let v7: u64 = {
        let a55 = v6;
        let a56 = 1u64;
        nat_sub(a55, a56)
    };
    Ok(f13(
        f14(v0.clone(), v4, v2, v7)?,
        f15(v0, {
            let a59 = v4;
            let a60 = {
                let a57 = 64u64;
                let a58 = v7;
                nat_mul(a57, a58)?
            };
            nat_add(a59, a60)?
        })?,
        v2,
        f16({
            let a63 = v5;
            let a64 = {
                let a61 = 64u64;
                let a62 = v7;
                nat_mul(a61, a62)?
            };
            nat_sub(a63, a64)
        }),
        {
            let a67 = {
                let a65 = v7;
                let a66 = 0u64;
                if nat_eq(a65, a66) {
                    3u32
                } else {
                    2u32
                }
            };
            let a68 = if v3 { 8u32 } else { 0u32 };
            fixed_u32::bit_or(a67, a68)
        },
    ))
}

pub fn f6(v0: Fn0, p72: u64, p73: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m71 = (p72, p73);
    {
        let mut m71 = m71;
        loop {
            let step = {
                let (v1, v2) = m71;
                let m69 = v1;
                if m69 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m69 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c70: Fn0 = v0.clone();
                        c70.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m71 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f17(v0)
}

pub fn f8(v0: u32, v1: u32) -> Bytes {
    f18(f19(v0, v1))
}

pub fn f9(v0: Fn1, p77: u64, p78: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v1;
                if m74 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m74 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c75: Fn1 = v0.clone();
                        c75.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m76 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f20(
        v2.clone(),
        f5(
            v0,
            v1,
            {
                let (h79, _) = v2;
                h79
            },
            false,
        )?,
    )
}

pub fn f11(v0: u64, v1: u64) -> u64 {
    let a82 = 1024u64;
    let a83 = {
        let a80 = v0;
        let a81 = v1;
        nat_sub(a80, a81)
    };
    if nat_lt(a82, a83) {
        1024u64
    } else {
        let a84 = v0;
        let a85 = v1;
        nat_sub(a84, a85)
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a86 = v0;
    let a87 = 0u64;
    if nat_eq(a86, a87) {
        Ok(1u64)
    } else {
        let a90 = {
            let a88 = v0;
            let a89 = 63u64;
            nat_add(a88, a89)?
        };
        let a91 = 64u64;
        let a92 = 0u64;
        Ok(nat_quot(a90, a91, a92))
    }
}

pub fn f13(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h93, _) = f21(
            Fn2::F22,
            7u64,
            (
                Adt2::C0(
                    match v0.clone() {
                        Adt0::C0(h94, _, _, _, _, _, _, _) => h94,
                    },
                    match v0.clone() {
                        Adt0::C0(_, h95, _, _, _, _, _, _) => h95,
                    },
                    match v0.clone() {
                        Adt0::C0(_, _, h96, _, _, _, _, _) => h96,
                    },
                    match v0.clone() {
                        Adt0::C0(_, _, _, h97, _, _, _, _) => h97,
                    },
                    match v0.clone() {
                        Adt0::C0(_, _, _, _, h98, _, _, _) => h98,
                    },
                    match v0.clone() {
                        Adt0::C0(_, _, _, _, _, h99, _, _) => h99,
                    },
                    match v0.clone() {
                        Adt0::C0(_, _, _, _, _, _, h100, _) => h100,
                    },
                    match v0 {
                        Adt0::C0(_, _, _, _, _, _, _, h101) => h101,
                    },
                    1779033703u32,
                    3144134277u32,
                    1013904242u32,
                    2773480762u32,
                    f16({
                        let a102 = v2;
                        let a103 = 4294967296u64;
                        let a104 = 0u64;
                        nat_rem(a102, a103, a104)
                    }),
                    f16({
                        let a105 = v2;
                        let a106 = 4294967296u64;
                        let a107 = 0u64;
                        nat_quot(a105, a106, a107)
                    }),
                    v3,
                    v4,
                ),
                v1,
            ),
        );
        h93
    };
    Adt0::C0(
        {
            let a110 = match v5.clone() {
                Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h108,
            };
            let a111 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h109, _, _, _, _, _, _, _) => h109,
            };
            fixed_u32::bit_xor(a110, a111)
        },
        {
            let a114 = match v5.clone() {
                Adt2::C0(_, h112, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h112,
            };
            let a115 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h113, _, _, _, _, _, _) => h113,
            };
            fixed_u32::bit_xor(a114, a115)
        },
        {
            let a118 = match v5.clone() {
                Adt2::C0(_, _, h116, _, _, _, _, _, _, _, _, _, _, _, _, _) => h116,
            };
            let a119 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h117, _, _, _, _, _) => h117,
            };
            fixed_u32::bit_xor(a118, a119)
        },
        {
            let a122 = match v5.clone() {
                Adt2::C0(_, _, _, h120, _, _, _, _, _, _, _, _, _, _, _, _) => h120,
            };
            let a123 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h121, _, _, _, _) => h121,
            };
            fixed_u32::bit_xor(a122, a123)
        },
        {
            let a126 = match v5.clone() {
                Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _, _, _, _, _, _) => h124,
            };
            let a127 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h125, _, _, _) => h125,
            };
            fixed_u32::bit_xor(a126, a127)
        },
        {
            let a130 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, h128, _, _, _, _, _, _, _, _, _, _) => h128,
            };
            let a131 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h129, _, _) => h129,
            };
            fixed_u32::bit_xor(a130, a131)
        },
        {
            let a134 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, h132, _, _, _, _, _, _, _, _, _) => h132,
            };
            let a135 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h133, _) => h133,
            };
            fixed_u32::bit_xor(a134, a135)
        },
        {
            let a138 = match v5.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h136, _, _, _, _, _, _, _, _) => h136,
            };
            let a139 = match v5 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h137) => h137,
            };
            fixed_u32::bit_xor(a138, a139)
        },
    )
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h140) = f23(Fn3::F24(v2, v0, v1), v3, (0u64, f25()))?;
    Ok(h140)
}

pub fn f15(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(
        f26(v0.clone(), {
            let a141 = v1;
            let a142 = 0u64;
            nat_add(a141, a142)?
        })?,
        f26(v0.clone(), {
            let a143 = v1;
            let a144 = 4u64;
            nat_add(a143, a144)?
        })?,
        f26(v0.clone(), {
            let a145 = v1;
            let a146 = 8u64;
            nat_add(a145, a146)?
        })?,
        f26(v0.clone(), {
            let a147 = v1;
            let a148 = 12u64;
            nat_add(a147, a148)?
        })?,
        f26(v0.clone(), {
            let a149 = v1;
            let a150 = 16u64;
            nat_add(a149, a150)?
        })?,
        f26(v0.clone(), {
            let a151 = v1;
            let a152 = 20u64;
            nat_add(a151, a152)?
        })?,
        f26(v0.clone(), {
            let a153 = v1;
            let a154 = 24u64;
            nat_add(a153, a154)?
        })?,
        f26(v0.clone(), {
            let a155 = v1;
            let a156 = 28u64;
            nat_add(a155, a156)?
        })?,
        f26(v0.clone(), {
            let a157 = v1;
            let a158 = 32u64;
            nat_add(a157, a158)?
        })?,
        f26(v0.clone(), {
            let a159 = v1;
            let a160 = 36u64;
            nat_add(a159, a160)?
        })?,
        f26(v0.clone(), {
            let a161 = v1;
            let a162 = 40u64;
            nat_add(a161, a162)?
        })?,
        f26(v0.clone(), {
            let a163 = v1;
            let a164 = 44u64;
            nat_add(a163, a164)?
        })?,
        f26(v0.clone(), {
            let a165 = v1;
            let a166 = 48u64;
            nat_add(a165, a166)?
        })?,
        f26(v0.clone(), {
            let a167 = v1;
            let a168 = 52u64;
            nat_add(a167, a168)?
        })?,
        f26(v0.clone(), {
            let a169 = v1;
            let a170 = 56u64;
            nat_add(a169, a170)?
        })?,
        f26(v0, {
            let a171 = v1;
            let a172 = 60u64;
            nat_add(a171, a172)?
        })?,
    ))
}

pub fn f16(v0: u64) -> u32 {
    let (_, h173) = {
        let (_, h174) = f27(Fn4::F28, 32u64, (v0, (1u32, 0u32)));
        h174
    };
    h173
}

pub fn f17(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m176 = {
        let (h175, _) = v0.clone();
        h175
    };
    match m176.uncons() {
        None => v0,
        Some((v1, v2)) => (
            v2.clone(),
            f29(
                v1,
                {
                    let (_, h177) = v0;
                    h177
                },
                {
                    let m178 = v2;
                    match m178.uncons() {
                        None => true,
                        Some((_, _)) => false,
                    }
                },
            ),
        ),
    }
}

pub fn f18(v0: u32) -> Bytes {
    let a181 = {
        let a179 = v0;
        let a180 = 8u32;
        fixed_u32::bit_and(a179, a180)
    };
    let a182 = 0u32;
    if equal(a181, a182) {
        let a185 = {
            let a183 = v0;
            let a184 = 4u32;
            fixed_u32::bit_and(a183, a184)
        };
        let a186 = 0u32;
        if equal(a185, a186) {
            let a189 = {
                let a187 = v0;
                let a188 = 2u32;
                fixed_u32::bit_and(a187, a188)
            };
            let a190 = 0u32;
            if equal(a189, a190) {
                let a193 = {
                    let a191 = v0;
                    let a192 = 1u32;
                    fixed_u32::bit_and(a191, a192)
                };
                let a194 = 0u32;
                if equal(a193, a194) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a197 = {
                    let a195 = v0;
                    let a196 = 1u32;
                    fixed_u32::bit_and(a195, a196)
                };
                let a198 = 0u32;
                if equal(a197, a198) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a201 = {
                let a199 = v0;
                let a200 = 2u32;
                fixed_u32::bit_and(a199, a200)
            };
            let a202 = 0u32;
            if equal(a201, a202) {
                let a205 = {
                    let a203 = v0;
                    let a204 = 1u32;
                    fixed_u32::bit_and(a203, a204)
                };
                let a206 = 0u32;
                if equal(a205, a206) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a209 = {
                    let a207 = v0;
                    let a208 = 1u32;
                    fixed_u32::bit_and(a207, a208)
                };
                let a210 = 0u32;
                if equal(a209, a210) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a213 = {
            let a211 = v0;
            let a212 = 4u32;
            fixed_u32::bit_and(a211, a212)
        };
        let a214 = 0u32;
        if equal(a213, a214) {
            let a217 = {
                let a215 = v0;
                let a216 = 2u32;
                fixed_u32::bit_and(a215, a216)
            };
            let a218 = 0u32;
            if equal(a217, a218) {
                let a221 = {
                    let a219 = v0;
                    let a220 = 1u32;
                    fixed_u32::bit_and(a219, a220)
                };
                let a222 = 0u32;
                if equal(a221, a222) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a225 = {
                    let a223 = v0;
                    let a224 = 1u32;
                    fixed_u32::bit_and(a223, a224)
                };
                let a226 = 0u32;
                if equal(a225, a226) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a229 = {
                let a227 = v0;
                let a228 = 2u32;
                fixed_u32::bit_and(a227, a228)
            };
            let a230 = 0u32;
            if equal(a229, a230) {
                let a233 = {
                    let a231 = v0;
                    let a232 = 1u32;
                    fixed_u32::bit_and(a231, a232)
                };
                let a234 = 0u32;
                if equal(a233, a234) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a237 = {
                    let a235 = v0;
                    let a236 = 1u32;
                    fixed_u32::bit_and(a235, a236)
                };
                let a238 = 0u32;
                if equal(a237, a238) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f19(v0: u32, v1: u32) -> u32 {
    let m241 = {
        let a239 = v0;
        let a240 = v1;
        fixed_u32::shift_right(a239, a240)
    };
    match m241 {
        None => 0u32,
        Some(v2) => v2,
    }
}

pub fn f20(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f30(
        Fn5::F31,
        40u64,
        Adt3::C0(
            v1,
            {
                let a243 = {
                    let (h242, _) = v0.clone();
                    h242
                };
                let a244 = 1u64;
                nat_add(a243, a244)?
            },
            {
                let (_, h245) = v0;
                h245
            },
        ),
    );
    Ok((
        {
            let a247 = {
                let (h246, _) = v0;
                h246
            };
            let a248 = 1u64;
            nat_add(a247, a248)?
        },
        List::cons(
            match v2.clone() {
                Adt3::C0(h249, _, _) => h249,
            },
            match v2 {
                Adt3::C0(_, _, h250) => h250,
            },
        ),
    ))
}

pub fn f21(v0: Fn2, p254: u64, p255: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m253 = (p254, p255);
    {
        let mut m253 = m253;
        loop {
            let step = {
                let (v1, v2) = m253;
                let m251 = v1;
                if m251 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m251 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c252: Fn2 = v0.clone();
                        c252.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m253 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f32(v0)
}

pub fn f23(v0: Fn3, p259: u64, p260: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m256 = v1;
                if m256 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m256 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c257: Fn3 = v0.clone();
                        c257.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m258 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok((
        {
            let a262 = {
                let (h261, _) = v3.clone();
                h261
            };
            let a263 = 1u64;
            nat_add(a262, a263)?
        },
        f13(
            {
                let (_, h264) = v3.clone();
                h264
            },
            f15(v1, {
                let a268 = v2;
                let a269 = {
                    let a266 = 64u64;
                    let a267 = {
                        let (h265, _) = v3.clone();
                        h265
                    };
                    nat_mul(a266, a267)?
                };
                nat_add(a268, a269)?
            })?,
            v0,
            64u32,
            {
                let a271 = {
                    let (h270, _) = v3;
                    h270
                };
                let a272 = 0u64;
                if nat_eq(a271, a272) {
                    1u32
                } else {
                    0u32
                }
            },
        ),
    ))
}

pub fn f25() -> Adt0 {
    Adt0::C0(
        1779033703u32,
        3144134277u32,
        1013904242u32,
        2773480762u32,
        1359893119u32,
        2600822924u32,
        528734635u32,
        1541459225u32,
    )
}

pub fn f26(v0: Bytes, v1: u64) -> R<u32> {
    let a283 = f33(v0.clone(), v1);
    let a284 = {
        let a281 = f34(
            f33(v0.clone(), {
                let a273 = v1;
                let a274 = 1u64;
                nat_add(a273, a274)?
            }),
            8u32,
        );
        let a282 = {
            let a279 = f34(
                f33(v0.clone(), {
                    let a275 = v1;
                    let a276 = 2u64;
                    nat_add(a275, a276)?
                }),
                16u32,
            );
            let a280 = f34(
                f33(v0, {
                    let a277 = v1;
                    let a278 = 3u64;
                    nat_add(a277, a278)?
                }),
                24u32,
            );
            fixed_u32::bit_or(a279, a280)
        };
        fixed_u32::bit_or(a281, a282)
    };
    Ok(fixed_u32::bit_or(a283, a284))
}

pub fn f27(v0: Fn4, p288: u64, p289: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m287 = (p288, p289);
    {
        let mut m287 = m287;
        loop {
            let step = {
                let (v1, v2) = m287;
                let m285 = v1;
                if m285 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m285 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c286: Fn4 = v0.clone();
                        c286.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m287 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h290, _) = v0;
        h290
    };
    let v2: u32 = {
        let (h291, _) = {
            let (_, h292) = v0;
            h292
        };
        h291
    };
    let v3: u32 = {
        let (_, h293) = {
            let (_, h294) = v0;
            h294
        };
        h293
    };
    (
        {
            let a295 = v1;
            let a296 = 2u64;
            let a297 = 0u64;
            nat_quot(a295, a296, a297)
        },
        (f34(v2, 1u32), {
            let a301 = {
                let a298 = v1;
                let a299 = 2u64;
                let a300 = 0u64;
                nat_rem(a298, a299, a300)
            };
            let a302 = 1u64;
            if nat_eq(a301, a302) {
                let a303 = v3;
                let a304 = v2;
                fixed_u32::bit_or(a303, a304)
            } else {
                v3
            }
        }),
    )
}

pub fn f29(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f13(f25(), f35(v0, v1), 0u64, 64u32, {
        let a305 = 4u32;
        let a306 = if v2 { 8u32 } else { 0u32 };
        fixed_u32::bit_or(a305, a306)
    })
}

pub fn f30(v0: Fn5, p310: u64, p311: Adt3) -> Adt3 {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v1, v2) = m309;
                let m307 = v1;
                if m307 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m307 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c308: Fn5 = v0.clone();
                        c308.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => {
                    m309 = next;
                }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt3) -> Adt3 {
    f36(v0)
}

pub fn f32(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (
        f37(
            f38(
                {
                    let (h312, _) = v0.clone();
                    h312
                },
                {
                    let (_, h313) = v0.clone();
                    h313
                },
            ),
            {
                let (_, h314) = v0.clone();
                h314
            },
        ),
        f39({
            let (_, h315) = v0;
            h315
        }),
    )
}

pub fn f33(v0: Bytes, v1: u64) -> u32 {
    let m318 = {
        let a316 = v0;
        let a317 = v1;
        index_bytes(a316, a317)
    };
    match m318 {
        None => 0u32,
        Some(v2) => {
            let m320 = {
                let a319 = v2;
                fixed_u32::convert(i128::from(a319))
            };
            match m320 {
                None => 0u32,
                Some(v3) => v3,
            }
        }
    }
}

pub fn f34(v0: u32, v1: u32) -> u32 {
    let m323 = {
        let a321 = v0;
        let a322 = v1;
        fixed_u32::shift_left(a321, a322)
    };
    match m323 {
        None => 0u32,
        Some(v2) => v2,
    }
}

pub fn f35(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(
        match v0.clone() {
            Adt0::C0(h324, _, _, _, _, _, _, _) => h324,
        },
        match v0.clone() {
            Adt0::C0(_, h325, _, _, _, _, _, _) => h325,
        },
        match v0.clone() {
            Adt0::C0(_, _, h326, _, _, _, _, _) => h326,
        },
        match v0.clone() {
            Adt0::C0(_, _, _, h327, _, _, _, _) => h327,
        },
        match v0.clone() {
            Adt0::C0(_, _, _, _, h328, _, _, _) => h328,
        },
        match v0.clone() {
            Adt0::C0(_, _, _, _, _, h329, _, _) => h329,
        },
        match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h330, _) => h330,
        },
        match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h331) => h331,
        },
        match v1.clone() {
            Adt0::C0(h332, _, _, _, _, _, _, _) => h332,
        },
        match v1.clone() {
            Adt0::C0(_, h333, _, _, _, _, _, _) => h333,
        },
        match v1.clone() {
            Adt0::C0(_, _, h334, _, _, _, _, _) => h334,
        },
        match v1.clone() {
            Adt0::C0(_, _, _, h335, _, _, _, _) => h335,
        },
        match v1.clone() {
            Adt0::C0(_, _, _, _, h336, _, _, _) => h336,
        },
        match v1.clone() {
            Adt0::C0(_, _, _, _, _, h337, _, _) => h337,
        },
        match v1.clone() {
            Adt0::C0(_, _, _, _, _, _, h338, _) => h338,
        },
        match v1 {
            Adt0::C0(_, _, _, _, _, _, _, h339) => h339,
        },
    )
}

pub fn f36(v0: Adt3) -> Adt3 {
    let a344 = {
        let a341 = match v0.clone() {
            Adt3::C0(_, h340, _) => h340,
        };
        let a342 = 2u64;
        let a343 = 0u64;
        nat_rem(a341, a342, a343)
    };
    let a345 = 0u64;
    if nat_eq(a344, a345) {
        let m347 = match v0.clone() {
            Adt3::C0(_, _, h346) => h346,
        };
        match m347.uncons() {
            None => v0,
            Some((v1, v2)) => Adt3::C0(
                f29(
                    v1,
                    match v0.clone() {
                        Adt3::C0(h348, _, _) => h348,
                    },
                    false,
                ),
                {
                    let a350 = match v0 {
                        Adt3::C0(_, h349, _) => h349,
                    };
                    let a351 = 2u64;
                    let a352 = 0u64;
                    nat_quot(a350, a351, a352)
                },
                v2,
            ),
        }
    } else {
        v0
    }
}

pub fn f37(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(h353, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h353,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, h354, _, _, _, _, _, _, _, _, _, _) => h354,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h355, _, _, _, _, _) => h355,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h356) => h356,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h357, _, _, _, _, _, _, _) => h357,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h358, _, _, _, _, _, _) => h358,
        },
    );
    let v3: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, h359, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h359,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h360, _, _, _, _, _, _, _, _, _) => h360,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h361, _, _, _, _) => h361,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h362, _, _, _) => h362,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h363, _, _, _, _, _) => h363,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h364, _, _, _, _) => h364,
        },
    );
    let v4: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, _, h365, _, _, _, _, _, _, _, _, _, _, _, _, _) => h365,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h366, _, _, _, _, _, _, _, _) => h366,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h367, _, _, _, _, _, _, _) => h367,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h368, _, _) => h368,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h369, _, _, _) => h369,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h370, _, _) => h370,
        },
    );
    let v5: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, _, _, h371, _, _, _, _, _, _, _, _, _, _, _, _) => h371,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, h372, _, _, _, _, _, _, _, _, _, _, _) => h372,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h373, _, _, _, _, _, _) => h373,
        },
        match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h374, _) => h374,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h375, _) => h375,
        },
        match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h376) => h376,
        },
    );
    Adt2::C0(
        match v2.clone() {
            Adt4::C0(h377, _, _, _) => h377,
        },
        match v3.clone() {
            Adt4::C0(h378, _, _, _) => h378,
        },
        match v4.clone() {
            Adt4::C0(h379, _, _, _) => h379,
        },
        match v5.clone() {
            Adt4::C0(h380, _, _, _) => h380,
        },
        match v5.clone() {
            Adt4::C0(_, h381, _, _) => h381,
        },
        match v2.clone() {
            Adt4::C0(_, h382, _, _) => h382,
        },
        match v3.clone() {
            Adt4::C0(_, h383, _, _) => h383,
        },
        match v4.clone() {
            Adt4::C0(_, h384, _, _) => h384,
        },
        match v4.clone() {
            Adt4::C0(_, _, h385, _) => h385,
        },
        match v5.clone() {
            Adt4::C0(_, _, h386, _) => h386,
        },
        match v2.clone() {
            Adt4::C0(_, _, h387, _) => h387,
        },
        match v3.clone() {
            Adt4::C0(_, _, h388, _) => h388,
        },
        match v3 {
            Adt4::C0(_, _, _, h389) => h389,
        },
        match v4 {
            Adt4::C0(_, _, _, h390) => h390,
        },
        match v5 {
            Adt4::C0(_, _, _, h391) => h391,
        },
        match v2 {
            Adt4::C0(_, _, _, h392) => h392,
        },
    )
}

pub fn f38(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(h393, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h393,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, h394, _, _, _, _, _, _, _, _, _, _, _) => h394,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h395, _, _, _, _, _, _, _) => h395,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h396, _, _, _) => h396,
        },
        match v1.clone() {
            Adt1::C0(h397, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h397,
        },
        match v1.clone() {
            Adt1::C0(_, h398, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h398,
        },
    );
    let v3: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, h399, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h399,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, h400, _, _, _, _, _, _, _, _, _, _) => h400,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h401, _, _, _, _, _, _) => h401,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h402, _, _) => h402,
        },
        match v1.clone() {
            Adt1::C0(_, _, h403, _, _, _, _, _, _, _, _, _, _, _, _, _) => h403,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, h404, _, _, _, _, _, _, _, _, _, _, _, _) => h404,
        },
    );
    let v4: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, _, h405, _, _, _, _, _, _, _, _, _, _, _, _, _) => h405,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h406, _, _, _, _, _, _, _, _, _) => h406,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h407, _, _, _, _, _) => h407,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h408, _) => h408,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, h409, _, _, _, _, _, _, _, _, _, _, _) => h409,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h410, _, _, _, _, _, _, _, _, _, _) => h410,
        },
    );
    let v5: Adt4 = f40(
        match v0.clone() {
            Adt2::C0(_, _, _, h411, _, _, _, _, _, _, _, _, _, _, _, _) => h411,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h412, _, _, _, _, _, _, _, _) => h412,
        },
        match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h413, _, _, _, _) => h413,
        },
        match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h414) => h414,
        },
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h415, _, _, _, _, _, _, _, _, _) => h415,
        },
        match v1 {
            Adt1::C0(_, _, _, _, _, _, _, h416, _, _, _, _, _, _, _, _) => h416,
        },
    );
    Adt2::C0(
        match v2.clone() {
            Adt4::C0(h417, _, _, _) => h417,
        },
        match v3.clone() {
            Adt4::C0(h418, _, _, _) => h418,
        },
        match v4.clone() {
            Adt4::C0(h419, _, _, _) => h419,
        },
        match v5.clone() {
            Adt4::C0(h420, _, _, _) => h420,
        },
        match v2.clone() {
            Adt4::C0(_, h421, _, _) => h421,
        },
        match v3.clone() {
            Adt4::C0(_, h422, _, _) => h422,
        },
        match v4.clone() {
            Adt4::C0(_, h423, _, _) => h423,
        },
        match v5.clone() {
            Adt4::C0(_, h424, _, _) => h424,
        },
        match v2.clone() {
            Adt4::C0(_, _, h425, _) => h425,
        },
        match v3.clone() {
            Adt4::C0(_, _, h426, _) => h426,
        },
        match v4.clone() {
            Adt4::C0(_, _, h427, _) => h427,
        },
        match v5.clone() {
            Adt4::C0(_, _, h428, _) => h428,
        },
        match v2 {
            Adt4::C0(_, _, _, h429) => h429,
        },
        match v3 {
            Adt4::C0(_, _, _, h430) => h430,
        },
        match v4 {
            Adt4::C0(_, _, _, h431) => h431,
        },
        match v5 {
            Adt4::C0(_, _, _, h432) => h432,
        },
    )
}

pub fn f39(v0: Adt1) -> Adt1 {
    Adt1::C0(
        match v0.clone() {
            Adt1::C0(_, _, h433, _, _, _, _, _, _, _, _, _, _, _, _, _) => h433,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h434, _, _, _, _, _, _, _, _, _) => h434,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, h435, _, _, _, _, _, _, _, _, _, _, _, _) => h435,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h436, _, _, _, _, _) => h436,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h437, _, _, _, _, _, _, _, _) => h437,
        },
        match v0.clone() {
            Adt1::C0(h438, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h438,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, h439, _, _, _, _, _, _, _, _, _, _, _) => h439,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h440, _, _) => h440,
        },
        match v0.clone() {
            Adt1::C0(_, h441, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => h441,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h442, _, _, _, _) => h442,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h443, _, _, _) => h443,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, h444, _, _, _, _, _, _, _, _, _, _) => h444,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h445, _, _, _, _, _, _) => h445,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h446, _) => h446,
        },
        match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h447) => h447,
        },
        match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h448, _, _, _, _, _, _, _) => h448,
        },
    )
}

pub fn f40(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f41(f41(v0, v1), v4);
    let v7: u32 = f42(
        {
            let a449 = v3;
            let a450 = v6;
            fixed_u32::bit_xor(a449, a450)
        },
        16u32,
        16u32,
    );
    let v8: u32 = f41(v2, v7);
    let v9: u32 = f42(
        {
            let a451 = v1;
            let a452 = v8;
            fixed_u32::bit_xor(a451, a452)
        },
        12u32,
        20u32,
    );
    let v10: u32 = f41(f41(v6, v9), v5);
    let v11: u32 = f42(
        {
            let a453 = v7;
            let a454 = v10;
            fixed_u32::bit_xor(a453, a454)
        },
        8u32,
        24u32,
    );
    let v12: u32 = f41(v8, v11);
    let v13: u32 = f42(
        {
            let a455 = v9;
            let a456 = v12;
            fixed_u32::bit_xor(a455, a456)
        },
        7u32,
        25u32,
    );
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f41(v0: u32, v1: u32) -> u32 {
    let m459 = {
        let a457 = f43(v0);
        let a458 = f43(v1);
        fixed_u64::checked_add(a457, a458)
    };
    match m459 {
        None => 0u32,
        Some(v2) => f44(v2),
    }
}

pub fn f42(v0: u32, v1: u32, v2: u32) -> u32 {
    let a460 = f19(v0, v1);
    let a461 = f34(v0, v2);
    fixed_u32::bit_or(a460, a461)
}

pub fn f43(v0: u32) -> u64 {
    let m463 = {
        let a462 = v0;
        fixed_u64::convert(i128::from(a462))
    };
    match m463 {
        None => 0u64,
        Some(v1) => v1,
    }
}

pub fn f44(v0: u64) -> u32 {
    let m467 = {
        let a466 = {
            let a464 = v0;
            let a465 = 4294967295u64;
            fixed_u64::bit_and(a464, a465)
        };
        fixed_u32::convert(i128::from(a466))
    };
    match m467 {
        None => 0u32,
        Some(v1) => v1,
    }
}

pub fn run(p0: Bytes) -> R<Bytes> {
    f0(p0)
}
