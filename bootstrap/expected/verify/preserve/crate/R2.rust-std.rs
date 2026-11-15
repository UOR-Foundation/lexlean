#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
}

pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }

pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
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
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
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
    pub fn lit(text: &str) -> Str { Str(Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
    pub fn length(&self) -> u64 { chars(&self.0) }
    pub fn utf8_encode(&self) -> Bytes { tick(self.0.len() as u64); Bytes::lit(self.0.as_bytes()) }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> { if delimiter.0.is_empty() { return None; }; chars(&self.0); let mut fields: Vec<Str> = Vec::new(); for piece in self.0.split(&*delimiter.0) { fields.push(Str::lit(piece)); }; if fields.len() as u128 > u128::from(maximum) { return None; }; Some(List::onto(fields, List::nil())) }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str { let mut out = String::new(); let mut first = true; let mut cursor = &texts; while let Repr::Cell(cell) = &cursor.0 { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0); out.push_str(&cell.head.0); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for text in buffer[..*len].iter().rev() { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&text.0); out.push_str(&text.0); } }, _ => { }, }; Str::lit(&out) }
    pub fn parse_int(text: &Str) -> Option<Option<i64>> { match canonical_decimal(&text.0) { None => Some(None), Some(Some(n)) => match i64::try_from(n) { Ok(value) => Some(Some(value)), Err(_) => None, }, Some(None) => None, } }
    pub fn format_u8(a: u8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u8(text: &Str) -> Option<u8> { match canonical_decimal(&text.0) { Some(Some(n)) => u8::try_from(n).ok(), _ => None, } }
    pub fn format_u16(a: u16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u16(text: &Str) -> Option<u16> { match canonical_decimal(&text.0) { Some(Some(n)) => u16::try_from(n).ok(), _ => None, } }
    pub fn format_u32(a: u32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u32(text: &Str) -> Option<u32> { match canonical_decimal(&text.0) { Some(Some(n)) => u32::try_from(n).ok(), _ => None, } }
    pub fn format_u64(a: u64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u64(text: &Str) -> Option<u64> { match canonical_decimal(&text.0) { Some(Some(n)) => u64::try_from(n).ok(), _ => None, } }
    pub fn format_i8(a: i8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i8(text: &Str) -> Option<i8> { match canonical_decimal(&text.0) { Some(Some(n)) => i8::try_from(n).ok(), _ => None, } }
    pub fn format_i16(a: i16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i16(text: &Str) -> Option<i16> { match canonical_decimal(&text.0) { Some(Some(n)) => i16::try_from(n).ok(), _ => None, } }
    pub fn format_i32(a: i32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i32(text: &Str) -> Option<i32> { match canonical_decimal(&text.0) { Some(Some(n)) => i32::try_from(n).ok(), _ => None, } }
    pub fn format_i64(a: i64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i64(text: &Str) -> Option<i64> { match canonical_decimal(&text.0) { Some(Some(n)) => i64::try_from(n).ok(), _ => None, } }
}

#[derive(Clone)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
    pub fn update(mut a: Bytes, i: u64, byte: u8) -> Option<Bytes> { let at = match usize::try_from(i) { Ok(at) => at, Err(_) => return None, }; if at >= a.0.len() { return None; }; tick(1); match Rc::get_mut(&mut a.0) { Some(cells) => { let slot: &mut [u8] = cells; slot[at] = byte; Some(a) }, None => { tick(a.0.len() as u64); let mut out = Vec::with_capacity(a.0.len()); out.extend_from_slice(&a.0); out[at] = byte; Some(Bytes(Rc::from(out))) }, } }
    pub fn append(a: Bytes, b: Bytes) -> Bytes { tick((a.0.len() + b.0.len()) as u64); let mut out = Vec::with_capacity(a.0.len() + b.0.len()); out.extend_from_slice(&a.0); out.extend_from_slice(&b.0); Bytes(Rc::from(out)) }
    pub fn length(&self) -> u64 { self.0.len() as u64 }
    pub fn index(&self, i: u64) -> Option<u8> { match usize::try_from(i) { Ok(at) => self.0.get(at).copied(), Err(_) => None, } }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> { let end = u128::from(start) + u128::from(count); if end > self.0.len() as u128 { return None; }; tick(count); Some(Bytes::lit(&self.0[start as usize..end as usize])) }
    pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64); match str::from_utf8(&self.0) { Ok(text) => Some(Str::lit(text)), Err(_) => None, } }
    pub fn compare(&self, other: &Bytes) -> Ordering { tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0) }
}

pub struct Node<T> { head: T, tail: List<T> }

pub enum Repr<T> { Nil, Run { buffer: Rc<Vec<T>>, len: usize }, Cell(Rc<Node<T>>) }

pub struct List<T>(Repr<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self { List(match &self.0 { Repr::Nil => Repr::Nil, Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len }, Repr::Cell(cell) => Repr::Cell(cell.clone()), }) }
}

impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { mem::replace(&mut self.0, Repr::Nil) }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) { let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }; while let Some(cell) = next { match Rc::try_unwrap(cell) { Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }, Err(_) => break, } } }
}

impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self { List(match tail.take() { Repr::Nil => { let mut buffer = Vec::with_capacity(16); buffer.push(head); Repr::Run { buffer: Rc::new(buffer), len: 1 } }, Repr::Run { mut buffer, len } => { match Rc::get_mut(&mut buffer) { Some(cells) => { cells.truncate(len); cells.push(head); Repr::Run { buffer, len: len + 1 } }, None => Repr::Cell(Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) })), } }, rest => Repr::Cell(Rc::new(Node { head, tail: List(rest) })), }) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { match &self.0 { Repr::Nil => None, Repr::Run { buffer, len } => { let head = buffer[len - 1].clone(); Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } }))) }, Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())), } }
    pub fn length(&self) -> u64 { let mut count = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); count += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for _ in buffer[..*len].iter().rev() { tick(1); count += 1; } }, _ => { }, }; count }
    pub fn append(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
    pub fn index(&self, i: u64) -> Option<T> { let mut position = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position == i { return Some(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position == i { return Some(item.clone()); }; position += 1; } }, _ => { }, }; None }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> { let end = u128::from(start) + u128::from(count); let mut part = Vec::new(); let mut position: u128 = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(item.clone()); }; position += 1; } }, _ => { }, }; if position < end { None } else { Some(List::onto(part, List::nil())) } }
    fn items(&self) -> Vec<T> { let mut out = Vec::new(); let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); out.push(cell.head.clone()); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); out.push(item.clone()); } }, _ => { }, }; out }
    fn onto(mut items: Vec<T>, tail: List<T>) -> List<T> { let mut out = tail; while let Some(item) = items.pop() { tick(1); out = List::cons(item, out); }; out }
}

pub fn chars(text: &str) -> u64 { let mut count = 0; for _ in text.chars() { tick(1); count += 1; }; count }

impl Same for Str {
    fn same(&self, other: &Self) -> bool { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => { }, } } }
}

impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}

impl Key for Str {
    fn key(&self, other: &Self) -> Ordering { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return Ordering::Equal, (None, Some(_)) => return Ordering::Less, (Some(_), None) => return Ordering::Greater, (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => { }, decided => return decided, }, } } }
}

pub fn canonical_decimal(text: &str) -> Option<Option<i128>> { let digits = match text.strip_prefix('-') { Some(rest) => rest, None => text, }; let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0"; for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }; if !canonical { return None; }; Some(text.parse::<i128>().ok()) }

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
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<Option<i64>, Overflow>(Overflow)
        }
    }
}

pub fn format_int(p0: i64) -> Str {
    Str::format_i64(p0)
}

pub fn update_bytes(p0: Bytes, p1: u64, p2: u8) -> Option<Bytes> {
    Bytes::update(p0, p1, p2)
}

#[derive(Clone)]
pub enum Adt0 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt3>),
    C1(u64),
    C2(u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt5>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(std::rc::Rc<Adt6>, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(std::rc::Rc<Adt5>, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(std::rc::Rc<Adt0>),
    C42,
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes),
    C1(std::rc::Rc<Adt0>),
    C2(List<Adt5>),
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F24(Bytes),
    F93(Bytes),
    F99(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F24(k0) => f24(k0.clone(), p0),
            Fn0::F93(k0) => f93(k0.clone(), p0),
            Fn0::F99(k0) => f99(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn1::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F89(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F89(k0) => f89(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2) -> R<Result<(), Bytes>> {
    let v6: Adt7 = Adt7::C0(v0, std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5));
    let a2 = match v6.clone() {
        Adt7::C0(h1, _, _, _, _, _) => {
            h1
        }
    };
    let a3 = f1(Str::lit("lexlean/target-program/1"));
    if equal(a2, a3) {
        let m5 = f2(v6.clone(), match v6.clone() {
            Adt7::C0(_, h4, _, _, _, _) => {
                (*h4).clone()
            }
        }, 0u64)?;
        match m5 {
            Ok(_) => {
                f3(v6.clone(), match v6 {
                    Adt7::C0(_, _, _, _, _, h6) => {
                        (*h6).clone()
                    }
                }, 0u64)
            }
            Err(v8) => {
                Ok(Err::<(), Bytes>(v8))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("target program has spec ")), f5(match v6 {
            Adt7::C0(h7, _, _, _, _, _) => {
                h7
            }
        }))))
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a8 = v0;
    utf8_encode(a8)
}

pub fn f2(v0: Adt7, p16: Adt0, p17: u64) -> R<Result<(), Bytes>> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m9 = v1;
                match m9 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C15(r13, r14) => {
                        let v13 = (*r13).clone();
                        let v14 = (*r14).clone();
                        if f6(v13.clone()) {
                            Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f1(Str::lit(" has no constructor")))))))
                        } else {
                            let m10 = f8(v0.clone(), v13)?;
                            match m10 {
                                Ok(_) => {
                                    Ok::<(Adt0, u64), R<Result<(), Bytes>>>((v14, {
                                        let a11 = v2;
                                        let a12 = 1u64;
                                        nat_add(a11, a12)?
                                    }))
                                }
                                Err(v16) => {
                                    Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v16))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt7, p25: Adt2, p26: u64) -> R<Result<(), Bytes>> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m18 = v1;
                match m18 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C13 => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C14(r22, r23) => {
                        let v3 = (*r22).clone();
                        let v4 = (*r23).clone();
                        let m19 = f9(v0.clone(), v3, v2)?;
                        match m19 {
                            Ok(_) => {
                                Ok::<(Adt2, u64), R<Result<(), Bytes>>>((v4, {
                                    let a20 = v2;
                                    let a21 = 1u64;
                                    nat_add(a20, a21)?
                                }))
                            }
                            Err(v6) => {
                                Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
                            }
                        }
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a27 = v0;
    let a28 = v1;
    append_bytes(a27, a28)
}

pub fn f5(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f6(v0: Adt0) -> bool {
    let a29 = f10(v0);
    let a30 = 14u64;
    nat_eq(a29, a30)
}

pub fn f7(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f11({
        let a67 = f12(v0, 10000000000000000000u64);
        let a68 = {
            let a65 = f12(v0, 1000000000000000000u64);
            let a66 = {
                let a63 = f12(v0, 100000000000000000u64);
                let a64 = {
                    let a61 = f12(v0, 10000000000000000u64);
                    let a62 = {
                        let a59 = f12(v0, 1000000000000000u64);
                        let a60 = {
                            let a57 = f12(v0, 100000000000000u64);
                            let a58 = {
                                let a55 = f12(v0, 10000000000000u64);
                                let a56 = {
                                    let a53 = f12(v0, 1000000000000u64);
                                    let a54 = {
                                        let a51 = f12(v0, 100000000000u64);
                                        let a52 = {
                                            let a49 = f12(v0, 10000000000u64);
                                            let a50 = {
                                                let a47 = f12(v0, 1000000000u64);
                                                let a48 = {
                                                    let a45 = f12(v0, 100000000u64);
                                                    let a46 = {
                                                        let a43 = f12(v0, 10000000u64);
                                                        let a44 = {
                                                            let a41 = f12(v0, 1000000u64);
                                                            let a42 = {
                                                                let a39 = f12(v0, 100000u64);
                                                                let a40 = {
                                                                    let a37 = f12(v0, 10000u64);
                                                                    let a38 = {
                                                                        let a35 = f12(v0, 1000u64);
                                                                        let a36 = {
                                                                            let a33 = f12(v0, 100u64);
                                                                            let a34 = {
                                                                                let a31 = f12(v0, 10u64);
                                                                                let a32 = f12(v0, 1u64);
                                                                                append_bytes(a31, a32)
                                                                            };
                                                                            append_bytes(a33, a34)
                                                                        };
                                                                        append_bytes(a35, a36)
                                                                    };
                                                                    append_bytes(a37, a38)
                                                                };
                                                                append_bytes(a39, a40)
                                                            };
                                                            append_bytes(a41, a42)
                                                        };
                                                        append_bytes(a43, a44)
                                                    };
                                                    append_bytes(a45, a46)
                                                };
                                                append_bytes(a47, a48)
                                            };
                                            append_bytes(a49, a50)
                                        };
                                        append_bytes(a51, a52)
                                    };
                                    append_bytes(a53, a54)
                                };
                                append_bytes(a55, a56)
                            };
                            append_bytes(a57, a58)
                        };
                        append_bytes(a59, a60)
                    };
                    append_bytes(a61, a62)
                };
                append_bytes(a63, a64)
            };
            append_bytes(a65, a66)
        };
        append_bytes(a67, a68)
    })?;
    let a69 = v0;
    let a70 = 0u64;
    if nat_eq(a69, a70) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f8(v0: Adt7, p76: Adt0) -> R<Result<(), Bytes>> {
    let m75 = p76;
    {
        let mut m75 = m75;
        loop {
            let step = {
                let v1 = m75;
                let m71 = v1;
                match m71 {
                    Adt0::C0 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C1 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C2 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C3 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C6 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C7 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C14 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C15(r73, r74) => {
                        let v12 = (*r73).clone();
                        let v13 = (*r74).clone();
                        let m72 = f13(v0.clone(), v12)?;
                        match m72 {
                            Ok(_) => {
                                Ok::<Adt0, R<Result<(), Bytes>>>(v13)
                            }
                            Err(v15) => {
                                Err::<Adt0, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v15)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Adt7, v1: Adt2, v2: u64) -> R<Result<(), Bytes>> {
    let v3: List<u64> = f14(v0.clone(), v2);
    let v4: Adt0 = f15(v0.clone(), v2);
    let v5: Adt0 = f16(v0.clone(), v2);
    let v6: u64 = f17(v3.clone())?;
    let v7: u64 = f18(v4.clone())?;
    let a77 = v6;
    let a78 = v7;
    if nat_eq(a77, a78) {
        if f19(v3.clone()) {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f1(Str::lit("a parameter name repeats")))))))
        } else {
            let m79 = f13(v0.clone(), v4.clone())?;
            match m79 {
                Ok(_) => {
                    let m80 = f13(v0.clone(), v5.clone())?;
                    match m80 {
                        Ok(_) => {
                            let m81 = f20(v0, v1, f21(v3, v4, List::<(u64, Adt0)>::nil()), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m81 {
                                Adt8::C0(v12) => {
                                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v12)))))
                                }
                                Adt8::C1(r83) => {
                                    let v14 = (*r83).clone();
                                    let m82 = f22(v14, v5, f1(Str::lit("the body")))?;
                                    match m82 {
                                        Ok(_) => {
                                            Ok(Ok::<(), Bytes>(()))
                                        }
                                        Err(v15) => {
                                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v15)))))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v10)))))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v8)))))
                }
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f4(f7(v6)?, f4(f1(Str::lit(" parameter names for ")), f4(f7(v7)?, f1(Str::lit(" parameter types"))))))))))
    }
}

pub fn f10(v0: Adt0) -> u64 {
    let m84 = v0;
    match m84 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4(_) => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h85, _) = f23(Fn0::F24(v0.clone()), {
            let a86 = v0.clone();
            length_bytes(a86)
        }, 0u64)?;
        h85
    };
    Ok(f25(v0.clone(), v1, {
        let a88 = {
            let a87 = v0;
            length_bytes(a87)
        };
        let a89 = v1;
        nat_sub(a88, a89)
    }))
}

pub fn f12(v0: u64, v1: u64) -> Bytes {
    f25(f1(Str::lit("0123456789")), {
        let a93 = {
            let a90 = v0;
            let a91 = v1;
            let a92 = 0u64;
            nat_quot(a90, a91, a92)
        };
        let a94 = 10u64;
        let a95 = 0u64;
        nat_rem(a93, a94, a95)
    }, 1u64)
}

pub fn f13(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m96 = v1;
    match m96 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(r102) => {
            let v3 = (*r102).clone();
            f13(v0, v3)
        }
        Adt0::C9(r103, r104) => {
            let v4 = (*r103).clone();
            let v5 = (*r104).clone();
            let m97 = f13(v0.clone(), v4)?;
            match m97 {
                Ok(_) => {
                    f13(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt0::C10(r105) => {
            let v8 = (*r105).clone();
            f13(v0, v8)
        }
        Adt0::C11(r106, r107) => {
            let v9 = (*r106).clone();
            let v10 = (*r107).clone();
            let m98 = f13(v0.clone(), v9)?;
            match m98 {
                Ok(_) => {
                    f13(v0, v10)
                }
                Err(v12) => {
                    Ok(Err::<(), Bytes>(v12))
                }
            }
        }
        Adt0::C12(v13) => {
            let m99 = f26(v0, v13);
            match m99 {
                None => {
                    Ok(Err::<(), Bytes>(f27(v13)?))
                }
                Some(_) => {
                    Ok(Ok::<(), Bytes>(()))
                }
            }
        }
        Adt0::C13(r108, r109) => {
            let v15 = (*r108).clone();
            let v16 = (*r109).clone();
            if f6(v15.clone()) {
                Ok(Err::<(), Bytes>(f1(Str::lit("a function type takes at least one parameter"))))
            } else {
                let m100 = f13(v0.clone(), v15)?;
                match m100 {
                    Ok(_) => {
                        f13(v0, v16)
                    }
                    Err(v18) => {
                        Ok(Err::<(), Bytes>(v18))
                    }
                }
            }
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r110, r111) => {
            let v19 = (*r110).clone();
            let v20 = (*r111).clone();
            let m101 = f13(v0.clone(), v19)?;
            match m101 {
                Ok(_) => {
                    f13(v0, v20)
                }
                Err(v22) => {
                    Ok(Err::<(), Bytes>(v22))
                }
            }
        }
    }
}

pub fn f14(v0: Adt7, v1: u64) -> List<u64> {
    let m113 = f28(match v0 {
        Adt7::C0(_, _, h112, _, _, _) => {
            h112
        }
    }, v1);
    match m113 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f15(v0: Adt7, v1: u64) -> Adt0 {
    let m114 = f29(v0, v1);
    match m114 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt7, v1: u64) -> Adt0 {
    let m116 = f30(match v0 {
        Adt7::C0(_, _, _, _, h115, _) => {
            (*h115).clone()
        }
    }, v1);
    match m116 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f17(v0: List<u64>) -> R<u64> {
    f31(Fn1::F32, 0u64, v0)
}

pub fn f18(v0: Adt0) -> R<u64> {
    let m117 = v0;
    match m117 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r120) => {
            let v12 = (*r120).clone();
            let v13: u64 = f18(v12)?;
            let a118 = 1u64;
            let a119 = v13;
            nat_add(a118, a119)
        }
    }
}

pub fn f19(v0: List<u64>) -> bool {
    let m121 = v0;
    match m121.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let v3: bool = f19(v2.clone());
            let v4: bool = f33(v2, v1);
            if v4 {
                true
            } else {
                v3
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: u64, v4: Adt0, v5: Adt0, v6: Bytes, v7: u64, v8: List<Adt5>) -> R<Adt8> {
    let m122 = v1;
    match m122 {
        Adt2::C0(r250, r251) => {
            let v9 = (*r250).clone();
            let v10 = (*r251).clone();
            let a123 = v3;
            let a124 = 0u64;
            if nat_eq(a123, a124) {
                let m125 = f13(v0.clone(), v9.clone())?;
                match m125 {
                    Ok(_) => {
                        let m126 = f34(v0, v10, v9.clone(), 0u64)?;
                        match m126 {
                            Ok(_) => {
                                Ok(Adt8::C1(std::rc::Rc::new(v9)))
                            }
                            Err(v13) => {
                                Ok(Adt8::C0(v13))
                            }
                        }
                    }
                    Err(v14) => {
                        Ok(Adt8::C0(v14))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C1(v15) => {
            let a127 = v3;
            let a128 = 0u64;
            if nat_eq(a127, a128) {
                let m129 = f35(v2, v15);
                match m129 {
                    None => {
                        Ok(Adt8::C0(f4(f1(Str::lit("local ")), f4(f7(v15)?, f1(Str::lit(" is unbound"))))))
                    }
                    Some(v16) => {
                        Ok(Adt8::C1(std::rc::Rc::new(v16)))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C2(v17, r252, r253, r254) => {
            let v18 = (*r252).clone();
            let v19 = (*r253).clone();
            let v20 = (*r254).clone();
            let a130 = v3;
            let a131 = 0u64;
            if nat_eq(a130, a131) {
                let m132 = f13(v0.clone(), v18.clone())?;
                match m132 {
                    Ok(_) => {
                        let m133 = f20(v0.clone(), v19, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m133 {
                            Adt8::C0(v25) => {
                                Ok(Adt8::C0(v25))
                            }
                            Adt8::C1(r135) => {
                                let v22 = (*r135).clone();
                                let m134 = f22(v22, v18.clone(), f4(f1(Str::lit("the value bound to ")), f7(v17)?))?;
                                match m134 {
                                    Ok(_) => {
                                        f20(v0, v20, f36(v17, v18, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())
                                    }
                                    Err(v24) => {
                                        Ok(Adt8::C0(v24))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v27) => {
                        Ok(Adt8::C0(v27))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C3(r255, r256, r257) => {
            let v28 = (*r255).clone();
            let v29 = (*r256).clone();
            let v30 = (*r257).clone();
            let a136 = v3;
            let a137 = 0u64;
            if nat_eq(a136, a137) {
                let m138 = f20(v0.clone(), v28, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m138 {
                    Adt8::C0(v42) => {
                        Ok(Adt8::C0(v42))
                    }
                    Adt8::C1(r145) => {
                        let v31 = (*r145).clone();
                        let m139 = f22(v31, Adt0::C1, f1(Str::lit("a condition")))?;
                        match m139 {
                            Ok(_) => {
                                let m140 = f20(v0.clone(), v29, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                match m140 {
                                    Adt8::C0(v39) => {
                                        Ok(Adt8::C0(v39))
                                    }
                                    Adt8::C1(r144) => {
                                        let v33 = (*r144).clone();
                                        let m141 = f20(v0, v30, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m141 {
                                            Adt8::C0(v37) => {
                                                Ok(Adt8::C0(v37))
                                            }
                                            Adt8::C1(r143) => {
                                                let v34 = (*r143).clone();
                                                let m142 = f22(v34, v33.clone(), f1(Str::lit("the else branch")))?;
                                                match m142 {
                                                    Ok(_) => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v33)))
                                                    }
                                                    Err(v36) => {
                                                        Ok(Adt8::C0(v36))
                                                    }
                                                }
                                            }
                                            Adt8::C2(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                }
                            }
                            Err(v41) => {
                                Ok(Adt8::C0(v41))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C4(r258, r259, r260) => {
            let v44 = (*r258).clone();
            let v45 = (*r259).clone();
            let v46 = (*r260).clone();
            let a146 = v3;
            let a147 = 0u64;
            if nat_eq(a146, a147) {
                let m148 = f13(v0.clone(), v44.clone())?;
                match m148 {
                    Ok(_) => {
                        let m149 = f20(v0.clone(), v45, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m149 {
                            Adt8::C0(v55) => {
                                Ok(Adt8::C0(v55))
                            }
                            Adt8::C1(r153) => {
                                let v48 = (*r153).clone();
                                let m150 = f37(v0.clone(), v48.clone())?;
                                match m150 {
                                    Ok(v49) => {
                                        let m151 = f20(v0, v46, v2, 4u64, v44.clone(), v48, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m151 {
                                            Adt8::C0(v52) => {
                                                Ok(Adt8::C0(v52))
                                            }
                                            Adt8::C1(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C2(v50) => {
                                                let m152 = f38(v49, v50);
                                                match m152 {
                                                    None => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v44)))
                                                    }
                                                    Some(v51) => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("the match is not exhaustive: ")), f4(f39(v51)?, f1(Str::lit(" is not covered"))))))
                                                    }
                                                }
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Err(v54) => {
                                        Ok(Adt8::C0(v54))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v57) => {
                        Ok(Adt8::C0(v57))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C5(r261, r262, r263) => {
            let v58 = (*r261).clone();
            let v59 = (*r262).clone();
            let v60 = (*r263).clone();
            let a154 = v3;
            let a155 = 0u64;
            if nat_eq(a154, a155) {
                let m156 = f13(v0.clone(), v59.clone())?;
                match m156 {
                    Ok(_) => {
                        let m157 = f40(v0.clone(), v58.clone(), v59.clone())?;
                        match m157 {
                            Ok(v62) => {
                                let v63: Bytes = f4(f1(Str::lit("constructor ")), f39(v58)?);
                                let a158 = f41(v60.clone())?;
                                let a159 = f18(v62.clone())?;
                                if nat_eq(a158, a159) {
                                    let m160 = f20(v0, v60, v2, 2u64, v62, Adt0::C14, v63, 0u64, List::<Adt5>::nil())?;
                                    match m160 {
                                        Adt8::C0(v64) => {
                                            Ok(Adt8::C0(v64))
                                        }
                                        Adt8::C1(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C1(std::rc::Rc::new(v59)))
                                        }
                                    }
                                } else {
                                    Ok(Adt8::C0(f4(v63, f4(f1(Str::lit(" takes ")), f4(f7(f18(v62)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v60)?)?))))))
                                }
                            }
                            Err(v67) => {
                                Ok(Adt8::C0(v67))
                            }
                        }
                    }
                    Err(v68) => {
                        Ok(Adt8::C0(v68))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C6(v69, r264) => {
            let v70 = (*r264).clone();
            let a161 = v3;
            let a162 = 0u64;
            if nat_eq(a161, a162) {
                let m163 = f29(v0.clone(), v69);
                match m163 {
                    None => {
                        Ok(Adt8::C0(f42(v69)?))
                    }
                    Some(v71) => {
                        let v72: Bytes = f4(f1(Str::lit("function ")), f7(v69)?);
                        let a164 = f41(v70.clone())?;
                        let a165 = f18(v71.clone())?;
                        if nat_eq(a164, a165) {
                            let m166 = f20(v0.clone(), v70, v2, 2u64, v71, Adt0::C14, v72, 0u64, List::<Adt5>::nil())?;
                            match m166 {
                                Adt8::C0(v73) => {
                                    Ok(Adt8::C0(v73))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C1(std::rc::Rc::new(f16(v0, v69))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(v72, f4(f1(Str::lit(" takes ")), f4(f7(f18(v71)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v70)?)?))))))
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C7(v76, r265) => {
            let v77 = (*r265).clone();
            let a167 = v3;
            let a168 = 0u64;
            if nat_eq(a167, a168) {
                let m169 = f29(v0.clone(), v76);
                match m169 {
                    None => {
                        Ok(Adt8::C0(f42(v76)?))
                    }
                    Some(v78) => {
                        let v79: u64 = f18(v78.clone())?;
                        let v80: u64 = f41(v77.clone())?;
                        let a170 = v79;
                        let a171 = v80;
                        if nat_le(a170, a171) {
                            Ok(Adt8::C0(f4(f1(Str::lit("closure of function ")), f4(f7(v76)?, f4(f1(Str::lit(" captures ")), f4(f7(v80)?, f4(f1(Str::lit(" of its ")), f4(f7(v79)?, f1(Str::lit(" parameters; at least one must remain"))))))))))
                        } else {
                            let m172 = f20(v0.clone(), v77, v2, 3u64, v78, Adt0::C14, f4(f1(Str::lit("closure of function ")), f7(v76)?), 0u64, List::<Adt5>::nil())?;
                            match m172 {
                                Adt8::C0(v82) => {
                                    Ok(Adt8::C0(v82))
                                }
                                Adt8::C1(r173) => {
                                    let v81 = (*r173).clone();
                                    Ok(Adt8::C1(std::rc::Rc::new(f43(v81, f16(v0, v76)))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C8(r266, r267) => {
            let v84 = (*r266).clone();
            let v85 = (*r267).clone();
            let a174 = v3;
            let a175 = 0u64;
            if nat_eq(a174, a175) {
                let m176 = f20(v0.clone(), v84, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m176 {
                    Adt8::C0(v91) => {
                        Ok(Adt8::C0(v91))
                    }
                    Adt8::C1(r182) => {
                        let v86 = (*r182).clone();
                        let a177 = f10(v86.clone());
                        let a178 = 13u64;
                        if nat_eq(a177, a178) {
                            let v87: Bytes = f1(Str::lit("an application"));
                            let a179 = f41(v85.clone())?;
                            let a180 = f18(f44(v86.clone()))?;
                            if nat_eq(a179, a180) {
                                let m181 = f20(v0, v85, v2, 2u64, f44(v86.clone()), Adt0::C14, v87, 0u64, List::<Adt5>::nil())?;
                                match m181 {
                                    Adt8::C0(v88) => {
                                        Ok(Adt8::C0(v88))
                                    }
                                    Adt8::C1(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C1(std::rc::Rc::new(f45(v86))))
                                    }
                                }
                            } else {
                                Ok(Adt8::C0(f4(v87, f4(f1(Str::lit(" takes ")), f4(f7(f18(f44(v86))?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v85)?)?))))))
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("only a function value can be applied"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C9(r268, r269) => {
            let v93 = (*r268).clone();
            let v94 = (*r269).clone();
            let a183 = v3;
            let a184 = 0u64;
            if nat_eq(a183, a184) {
                let m185 = f20(v0, v94, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m185 {
                    Adt8::C0(v98) => {
                        Ok(Adt8::C0(v98))
                    }
                    Adt8::C1(r187) => {
                        let v95 = (*r187).clone();
                        let m186 = f46(v93, v95)?;
                        match m186 {
                            Ok(v96) => {
                                Ok(Adt8::C1(std::rc::Rc::new(v96)))
                            }
                            Err(v97) => {
                                Ok(Adt8::C0(v97))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C10(r270) => {
            let v100 = (*r270).clone();
            let a188 = v3;
            let a189 = 0u64;
            if nat_eq(a188, a189) {
                let m190 = f20(v0, v100, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m190 {
                    Adt8::C0(v102) => {
                        Ok(Adt8::C0(v102))
                    }
                    Adt8::C1(r193) => {
                        let v101 = (*r193).clone();
                        let a191 = f10(v101.clone());
                        let a192 = 11u64;
                        if nat_eq(a191, a192) {
                            Ok(Adt8::C1(std::rc::Rc::new(f44(v101))))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`first` of ")), f47(v101)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C11(r271) => {
            let v104 = (*r271).clone();
            let a194 = v3;
            let a195 = 0u64;
            if nat_eq(a194, a195) {
                let m196 = f20(v0, v104, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m196 {
                    Adt8::C0(v106) => {
                        Ok(Adt8::C0(v106))
                    }
                    Adt8::C1(r199) => {
                        let v105 = (*r199).clone();
                        let a197 = f10(v105.clone());
                        let a198 = 11u64;
                        if nat_eq(a197, a198) {
                            Ok(Adt8::C1(std::rc::Rc::new(f45(v105))))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`second` of ")), f47(v105)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C12(r272, v109) => {
            let v108 = (*r272).clone();
            let a200 = v3;
            let a201 = 0u64;
            if nat_eq(a200, a201) {
                let m202 = f20(v0.clone(), v108, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m202 {
                    Adt8::C0(v115) => {
                        Ok(Adt8::C0(v115))
                    }
                    Adt8::C1(r210) => {
                        let v110 = (*r210).clone();
                        let a203 = f10(v110.clone());
                        let a204 = 12u64;
                        if nat_eq(a203, a204) {
                            let v111: u64 = f48(v110);
                            let m205 = f26(v0, v111);
                            match m205 {
                                None => {
                                    Ok(Adt8::C0(f27(v111)?))
                                }
                                Some(v112) => {
                                    let a206 = f18(v112.clone())?;
                                    let a207 = 1u64;
                                    if nat_eq(a206, a207) {
                                        let m208 = f30(v112, 0u64);
                                        match m208 {
                                            None => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Some(v113) => {
                                                let m209 = f30(v113, v109);
                                                match m209 {
                                                    None => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has no field ")), f7(v109)?)))))
                                                    }
                                                    Some(v114) => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v114)))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        Ok(Adt8::C0(f4(f1(Str::lit("`field` requires an ADT with exactly one constructor; ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has ")), f7(f18(v112)?)?)))))
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("`field` requires an ADT value"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C13 => {
            let a211 = v3;
            let a212 = 1u64;
            if nat_eq(a211, a212) {
                Ok(Adt8::C1(std::rc::Rc::new(Adt0::C14)))
            } else {
                let a213 = v3;
                let a214 = 2u64;
                if nat_eq(a213, a214) {
                    Ok(Adt8::C3)
                } else {
                    let a215 = v3;
                    let a216 = 3u64;
                    if nat_eq(a215, a216) {
                        Ok(Adt8::C1(std::rc::Rc::new(v4)))
                    } else {
                        let a217 = v3;
                        let a218 = 4u64;
                        if nat_eq(a217, a218) {
                            Ok(Adt8::C2(v8))
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C14(r273, r274) => {
            let v117 = (*r273).clone();
            let v118 = (*r274).clone();
            let a219 = v3;
            let a220 = 1u64;
            if nat_eq(a219, a220) {
                let m221 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m221 {
                    Adt8::C0(v123) => {
                        Ok(Adt8::C0(v123))
                    }
                    Adt8::C1(r224) => {
                        let v119 = (*r224).clone();
                        let m222 = f20(v0, v118, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m222 {
                            Adt8::C0(v121) => {
                                Ok(Adt8::C0(v121))
                            }
                            Adt8::C1(r223) => {
                                let v120 = (*r223).clone();
                                Ok(Adt8::C1(std::rc::Rc::new(f49(v119, v120))))
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                let a225 = v3;
                let a226 = 2u64;
                if nat_eq(a225, a226) {
                    if f6(v4.clone()) {
                        Ok(Adt8::C3)
                    } else {
                        let m227 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m227 {
                            Adt8::C0(v128) => {
                                Ok(Adt8::C0(v128))
                            }
                            Adt8::C1(r231) => {
                                let v125 = (*r231).clone();
                                let m228 = f22(v125, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                match m228 {
                                    Ok(_) => {
                                        f20(v0, v118, v2, 2u64, f45(v4), Adt0::C14, v6, {
                                            let a229 = v7;
                                            let a230 = 1u64;
                                            nat_add(a229, a230)?
                                        }, List::<Adt5>::nil())
                                    }
                                    Err(v127) => {
                                        Ok(Adt8::C0(v127))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                } else {
                    let a232 = v3;
                    let a233 = 3u64;
                    if nat_eq(a232, a233) {
                        if f6(v4.clone()) {
                            Ok(Adt8::C1(std::rc::Rc::new(Adt0::C14)))
                        } else {
                            let m234 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m234 {
                                Adt8::C0(v133) => {
                                    Ok(Adt8::C0(v133))
                                }
                                Adt8::C1(r238) => {
                                    let v130 = (*r238).clone();
                                    let m235 = f22(v130, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                    match m235 {
                                        Ok(_) => {
                                            f20(v0, v118, v2, 3u64, f45(v4), Adt0::C14, v6, {
                                                let a236 = v7;
                                                let a237 = 1u64;
                                                nat_add(a236, a237)?
                                            }, List::<Adt5>::nil())
                                        }
                                        Err(v132) => {
                                            Ok(Adt8::C0(v132))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    } else {
                        let a239 = v3;
                        let a240 = 4u64;
                        if nat_eq(a239, a240) {
                            let m241 = f20(v0.clone(), v117, v2.clone(), 5u64, v4.clone(), v5.clone(), f1(Str::lit("")), 0u64, v8)?;
                            match m241 {
                                Adt8::C0(v136) => {
                                    Ok(Adt8::C0(v136))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(v135) => {
                                    f20(v0, v118, v2, 4u64, v4, v5, f1(Str::lit("")), 0u64, v135)
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C15(r275, v139, r276) => {
            let v138 = (*r275).clone();
            let v140 = (*r276).clone();
            let a242 = v3;
            let a243 = 5u64;
            if nat_eq(a242, a243) {
                let m244 = f40(v0.clone(), v138.clone(), v5)?;
                match m244 {
                    Ok(v141) => {
                        let v142: u64 = f17(v139.clone())?;
                        let v143: u64 = f18(v141.clone())?;
                        let a245 = v142;
                        let a246 = v143;
                        if nat_eq(a245, a246) {
                            if f19(v139.clone()) {
                                Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f1(Str::lit(" binds the same name twice"))))))
                            } else {
                                if f50(v8.clone(), v138.clone()) {
                                    Ok(Adt8::C0(f4(f1(Str::lit("the arm for ")), f4(f39(v138)?, f1(Str::lit(" is unreachable"))))))
                                } else {
                                    let m247 = f20(v0, v140, f21(v139, v141, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                    match m247 {
                                        Adt8::C0(v147) => {
                                            Ok(Adt8::C0(v147))
                                        }
                                        Adt8::C1(r249) => {
                                            let v144 = (*r249).clone();
                                            let m248 = f22(v144, v4, f1(Str::lit("an arm")))?;
                                            match m248 {
                                                Ok(_) => {
                                                    Ok(Adt8::C2(f51(v138, v8)))
                                                }
                                                Err(v146) => {
                                                    Ok(Adt8::C0(v146))
                                                }
                                            }
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f4(f1(Str::lit(" binds ")), f4(f7(v143)?, f4(f1(Str::lit(" value(s), the arm names ")), f7(v142)?)))))))
                        }
                    }
                    Err(v149) => {
                        Ok(Adt8::C0(v149))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
    }
}

pub fn f21(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m277 = v0;
    match m277.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f21(v4, f45(v1.clone()), f36(v3, f44(v1.clone()), v2.clone()));
            if f6(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Result<(), Bytes>> {
    let v3: Result<(), Bytes> = Ok::<(), Bytes>(());
    let v4: Result<(), Bytes> = Err::<(), Bytes>(f4(v2, f4(f1(Str::lit(" has type ")), f4(f47(v0.clone())?, f4(f1(Str::lit(", expected ")), f47(v1.clone())?)))));
    if f52(v0, v1) {
        Ok(v3)
    } else {
        Ok(v4)
    }
}

pub fn f23(v0: Fn0, p282: u64, p283: u64) -> R<(u64, bool)> {
    let m281 = (p282, p283);
    {
        let mut m281 = m281;
        loop {
            let step = {
                let (v1, v2) = m281;
                let m278 = v1;
                if m278 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m278 - 1;
                    let m280 = {
                        let c279: Fn0 = v0.clone();
                        c279.apply(v2)?
                    };
                    match m280 {
                        None => {
                            Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), R<(u64, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f25(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m287 = {
        let a284 = v0;
        let a285 = v1;
        let a286 = v2;
        slice_bytes(a284, a285, a286)
    };
    match m287 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f26(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, h288, _, _, _, _) => {
            (*h288).clone()
        }
    }, v1)
}

pub fn f27(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("ADT ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f28(p292: List<List<u64>>, p293: u64) -> Option<List<u64>> {
    let m291 = (p292, p293);
    {
        let mut m291 = m291;
        loop {
            let step = {
                let (v0, v1) = m291;
                let m289 = v0;
                match m289.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m290 = v1;
                        if m290 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m290 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, _, _, h294, _, _) => {
            (*h294).clone()
        }
    }, v1)
}

pub fn f30(p300: Adt0, p301: u64) -> Option<Adt0> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v0, v1) = m299;
                let m295 = v0;
                match m295 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r297, r298) => {
                        let v12 = (*r297).clone();
                        let v13 = (*r298).clone();
                        let m296 = v1;
                        if m296 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m296 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Fn1, p305: u64, p306: List<u64>) -> R<u64> {
    let m304 = (p305, p306);
    {
        let mut m304 = m304;
        loop {
            let step = {
                let (v1, v2) = m304;
                let m302 = v2;
                match m302.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c303: Fn1 = v0.clone();
                            c303.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: u64, _: u64) -> R<u64> {
    let a307 = v0;
    let a308 = 1u64;
    nat_add(a307, a308)
}

pub fn f33(v0: List<u64>, v1: u64) -> bool {
    let m309 = v0;
    match m309.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f33(v3, v1);
            let a310 = v2;
            let a311 = v1;
            if nat_eq(a310, a311) {
                true
            } else {
                v4
            }
        }
    }
}

pub fn f34(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m312 = v1;
    match m312 {
        Adt3::C0 => {
            let v4: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v5: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C0)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a313 = f10(v2);
            let a314 = 0u64;
            if nat_eq(a313, a314) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt3::C1(v6) => {
            let v7: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v8: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C1(v6))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a315 = f10(v2);
            let a316 = 1u64;
            if nat_eq(a315, a316) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt3::C2(v9) => {
            let v10: Result<(), Bytes> = f55(v9.clone())?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C2(v9))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a317 = f10(v2);
            let a318 = 2u64;
            if nat_eq(a317, a318) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt3::C3(v12) => {
            let v13: Result<(), Bytes> = f56(v12.clone(), Adt1::C7)?;
            let v14: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C3(v12))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a319 = f10(v2);
            let a320 = 3u64;
            if nat_eq(a319, a320) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt3::C4(v15) => {
            let v16: Result<(), Bytes> = f56(v15.clone(), Adt1::C0)?;
            let v17: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C4(v15))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a321 = f10(v2.clone());
            let a322 = 4u64;
            if if nat_eq(a321, a322) {
                let a323 = f57(f58(v2));
                let a324 = 0u64;
                nat_eq(a323, a324)
            } else {
                false
            } {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt3::C5(v18) => {
            let v19: Result<(), Bytes> = f56(v18.clone(), Adt1::C1)?;
            let v20: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C5(v18))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a325 = f10(v2.clone());
            let a326 = 4u64;
            if if nat_eq(a325, a326) {
                let a327 = f57(f58(v2));
                let a328 = 1u64;
                nat_eq(a327, a328)
            } else {
                false
            } {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt3::C6(v21) => {
            let v22: Result<(), Bytes> = f56(v21.clone(), Adt1::C2)?;
            let v23: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C6(v21))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a329 = f10(v2.clone());
            let a330 = 4u64;
            if if nat_eq(a329, a330) {
                let a331 = f57(f58(v2));
                let a332 = 2u64;
                nat_eq(a331, a332)
            } else {
                false
            } {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt3::C7(v24) => {
            let v25: Result<(), Bytes> = f56(v24.clone(), Adt1::C3)?;
            let v26: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C7(v24))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a333 = f10(v2.clone());
            let a334 = 4u64;
            if if nat_eq(a333, a334) {
                let a335 = f57(f58(v2));
                let a336 = 3u64;
                nat_eq(a335, a336)
            } else {
                false
            } {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt3::C8(v27) => {
            let v28: Result<(), Bytes> = f56(v27.clone(), Adt1::C4)?;
            let v29: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C8(v27))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a337 = f10(v2.clone());
            let a338 = 4u64;
            if if nat_eq(a337, a338) {
                let a339 = f57(f58(v2));
                let a340 = 4u64;
                nat_eq(a339, a340)
            } else {
                false
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt3::C9(v30) => {
            let v31: Result<(), Bytes> = f56(v30.clone(), Adt1::C5)?;
            let v32: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C9(v30))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a341 = f10(v2.clone());
            let a342 = 4u64;
            if if nat_eq(a341, a342) {
                let a343 = f57(f58(v2));
                let a344 = 5u64;
                nat_eq(a343, a344)
            } else {
                false
            } {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt3::C10(v33) => {
            let v34: Result<(), Bytes> = f56(v33.clone(), Adt1::C6)?;
            let v35: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C10(v33))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a345 = f10(v2.clone());
            let a346 = 4u64;
            if if nat_eq(a345, a346) {
                let a347 = f57(f58(v2));
                let a348 = 6u64;
                nat_eq(a347, a348)
            } else {
                false
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt3::C11(v36) => {
            let v37: Result<(), Bytes> = f56(v36.clone(), Adt1::C7)?;
            let v38: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C11(v36))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a349 = f10(v2.clone());
            let a350 = 4u64;
            if if nat_eq(a349, a350) {
                let a351 = f57(f58(v2));
                let a352 = 7u64;
                nat_eq(a351, a352)
            } else {
                false
            } {
                Ok(v37)
            } else {
                Ok(v38)
            }
        }
        Adt3::C12(v39) => {
            let v40: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v41: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C12(v39))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a353 = f10(v2);
            let a354 = 5u64;
            if nat_eq(a353, a354) {
                Ok(v40)
            } else {
                Ok(v41)
            }
        }
        Adt3::C13(v42) => {
            let v43: Result<(), Bytes> = f59(v42.clone())?;
            let v44: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C13(v42))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a355 = f10(v2);
            let a356 = 6u64;
            if nat_eq(a355, a356) {
                Ok(v43)
            } else {
                Ok(v44)
            }
        }
        Adt3::C14(v45) => {
            let v46: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v47: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C14(v45))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a357 = f10(v2);
            let a358 = 7u64;
            if nat_eq(a357, a358) {
                Ok(v46)
            } else {
                Ok(v47)
            }
        }
        Adt3::C15 => {
            let v48: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v49: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C15)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a359 = f10(v2);
            let a360 = 8u64;
            if nat_eq(a359, a360) {
                Ok(v48)
            } else {
                Ok(v49)
            }
        }
        Adt3::C16(r386) => {
            let v50 = (*r386).clone();
            let v51: Result<(), Bytes> = f34(v0, v50.clone(), f44(v2.clone()), 0u64)?;
            let v52: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C16(std::rc::Rc::new(v50)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a361 = f10(v2);
            let a362 = 8u64;
            if nat_eq(a361, a362) {
                Ok(v51)
            } else {
                Ok(v52)
            }
        }
        Adt3::C17(r387) => {
            let v53 = (*r387).clone();
            let v54: Result<(), Bytes> = f34(v0, v53.clone(), f44(v2.clone()), 0u64)?;
            let v55: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C17(std::rc::Rc::new(v53)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a363 = f10(v2);
            let a364 = 9u64;
            if nat_eq(a363, a364) {
                Ok(v54)
            } else {
                Ok(v55)
            }
        }
        Adt3::C18(r388) => {
            let v56 = (*r388).clone();
            let v57: Result<(), Bytes> = f34(v0, v56.clone(), f45(v2.clone()), 0u64)?;
            let v58: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C18(std::rc::Rc::new(v56)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a365 = f10(v2);
            let a366 = 9u64;
            if nat_eq(a365, a366) {
                Ok(v57)
            } else {
                Ok(v58)
            }
        }
        Adt3::C19(r389) => {
            let v59 = (*r389).clone();
            let v60: Result<(), Bytes> = f34(v0, v59.clone(), f44(v2.clone()), 1u64)?;
            let v61: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C19(std::rc::Rc::new(v59)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a367 = f10(v2);
            let a368 = 10u64;
            if nat_eq(a367, a368) {
                Ok(v60)
            } else {
                Ok(v61)
            }
        }
        Adt3::C20(r390, r391) => {
            let v62 = (*r390).clone();
            let v63 = (*r391).clone();
            let v64: Result<(), Bytes> = f34(v0.clone(), v62.clone(), f44(v2.clone()), 0u64)?;
            let v65: Result<(), Bytes> = f34(v0, v63.clone(), f45(v2.clone()), 0u64)?;
            let v68: Result<(), Bytes> = {
                let m369 = v64;
                match m369 {
                    Ok(_) => {
                        v65
                    }
                    Err(v67) => {
                        Err::<(), Bytes>(v67)
                    }
                }
            };
            let v69: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C20(std::rc::Rc::new(v62), std::rc::Rc::new(v63)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a370 = f10(v2);
            let a371 = 11u64;
            if nat_eq(a370, a371) {
                Ok(v68)
            } else {
                Ok(v69)
            }
        }
        Adt3::C21(v70, r392) => {
            let v71 = (*r392).clone();
            let v77: Result<(), Bytes> = {
                let m372 = f60(v0.clone(), f48(v2.clone()), v70)?;
                match m372 {
                    Ok(v73) => {
                        let v74: Result<(), Bytes> = f34(v0, v71.clone(), v73.clone(), 2u64)?;
                        let v75: Result<(), Bytes> = v74;
                        let v76: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("constructor ")), f4(f7(v70)?, f4(f1(Str::lit(" of ADT ")), f4(f7(f48(v2.clone()))?, f4(f1(Str::lit(" takes ")), f4(f7(f18(v73.clone())?)?, f4(f1(Str::lit(" fields, the literal has ")), f7(f61(v71.clone())?)?))))))));
                        let a373 = f61(v71.clone())?;
                        let a374 = f18(v73)?;
                        if nat_eq(a373, a374) {
                            v75
                        } else {
                            v76
                        }
                    }
                    Err(v72) => {
                        Err::<(), Bytes>(v72)
                    }
                }
            };
            let v78: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C21(v70, std::rc::Rc::new(v71)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a375 = f10(v2);
            let a376 = 12u64;
            if nat_eq(a375, a376) {
                Ok(v77)
            } else {
                Ok(v78)
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f1(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r393, r394) => {
            let v81 = (*r393).clone();
            let v82 = (*r394).clone();
            let v83: Adt0 = {
                let a377 = v3;
                let a378 = 2u64;
                if nat_eq(a377, a378) {
                    f44(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v84: Adt0 = {
                let a379 = v3;
                let a380 = 2u64;
                if nat_eq(a379, a380) {
                    f45(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v85: Result<(), Bytes> = f34(v0.clone(), v81, v83, 0u64)?;
            let v86: Result<(), Bytes> = f34(v0, v82, v84, v3)?;
            let a381 = v3;
            let a382 = 0u64;
            if nat_eq(a381, a382) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a383 = v3;
                let a384 = 2u64;
                if if nat_eq(a383, a384) {
                    f6(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m385 = v85;
                    match m385 {
                        Ok(_) => {
                            Ok(v86)
                        }
                        Err(v88) => {
                            Ok(Err::<(), Bytes>(v88))
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: List<(u64, Adt0)>, v1: u64) -> Option<Adt0> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            None::<Adt0>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt0> = f35(v3, v1);
            let a397 = {
                let (h396, _) = v2;
                h396
            };
            let a398 = v1;
            if nat_eq(a397, a398) {
                Some({
                    let (_, h399) = v2;
                    h399
                })
            } else {
                v4
            }
        }
    }
}

pub fn f36(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f37(v0: Adt7, v1: Adt0) -> R<Result<List<Adt5>, Bytes>> {
    let m400 = v1.clone();
    match m400 {
        Adt0::C0 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C11, List::<Adt5>::nil())))
        }
        Adt0::C1 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C9, List::cons(Adt5::C10, List::<Adt5>::nil()))))
        }
        Adt0::C2 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C6, List::cons(Adt5::C7, List::<Adt5>::nil()))))
        }
        Adt0::C3 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C4(_) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C5 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C6 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C7 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C12, List::cons(Adt5::C13, List::cons(Adt5::C14, List::<Adt5>::nil())))))
        }
        Adt0::C8(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C0, List::cons(Adt5::C1, List::<Adt5>::nil()))))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C2, List::cons(Adt5::C3, List::<Adt5>::nil()))))
        }
        Adt0::C10(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C4, List::cons(Adt5::C5, List::<Adt5>::nil()))))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C8, List::<Adt5>::nil())))
        }
        Adt0::C12(v9) => {
            let m401 = f26(v0, v9);
            match m401 {
                None => {
                    Ok(Err::<List<Adt5>, Bytes>(f27(v9)?))
                }
                Some(v10) => {
                    Ok(Ok::<List<Adt5>, Bytes>(f62(v10, 0u64)?))
                }
            }
        }
        Adt0::C13(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C14 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C15(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
    }
}

pub fn f38(v0: List<Adt5>, v1: List<Adt5>) -> Option<Adt5> {
    let m402 = v0;
    match m402.uncons() {
        None => {
            None::<Adt5>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt5> = f38(v3, v1.clone());
            let v5: Option<Adt5> = v4;
            let v6: Option<Adt5> = Some(v2.clone());
            if f50(v1, v2) {
                v5
            } else {
                v6
            }
        }
    }
}

pub fn f39(v0: Adt5) -> R<Bytes> {
    let m403 = v0;
    match m403 {
        Adt5::C0 => {
            Ok(f1(Str::lit("None")))
        }
        Adt5::C1 => {
            Ok(f1(Str::lit("Some")))
        }
        Adt5::C2 => {
            Ok(f1(Str::lit("Ok")))
        }
        Adt5::C3 => {
            Ok(f1(Str::lit("Error")))
        }
        Adt5::C4 => {
            Ok(f1(Str::lit("Nil")))
        }
        Adt5::C5 => {
            Ok(f1(Str::lit("Cons")))
        }
        Adt5::C6 => {
            Ok(f1(Str::lit("Zero")))
        }
        Adt5::C7 => {
            Ok(f1(Str::lit("Succ")))
        }
        Adt5::C8 => {
            Ok(f1(Str::lit("Pair")))
        }
        Adt5::C9 => {
            Ok(f1(Str::lit("True")))
        }
        Adt5::C10 => {
            Ok(f1(Str::lit("False")))
        }
        Adt5::C11 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt5::C12 => {
            Ok(f1(Str::lit("Lt")))
        }
        Adt5::C13 => {
            Ok(f1(Str::lit("Eq")))
        }
        Adt5::C14 => {
            Ok(f1(Str::lit("Gt")))
        }
        Adt5::C15(v1) => {
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v1)?, f1(Str::lit(" }")))))
        }
    }
}

pub fn f40(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Result<Adt0, Bytes>> {
    let m404 = v1.clone();
    match m404 {
        Adt5::C0 => {
            let v3: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v4: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a405 = f10(v2);
            let a406 = 8u64;
            if nat_eq(a405, a406) {
                Ok(v3)
            } else {
                Ok(v4)
            }
        }
        Adt5::C1 => {
            let v5: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v6: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a407 = f10(v2);
            let a408 = 8u64;
            if nat_eq(a407, a408) {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
        Adt5::C2 => {
            let v7: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v8: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a409 = f10(v2);
            let a410 = 9u64;
            if nat_eq(a409, a410) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt5::C3 => {
            let v9: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v10: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a411 = f10(v2);
            let a412 = 9u64;
            if nat_eq(a411, a412) {
                Ok(v9)
            } else {
                Ok(v10)
            }
        }
        Adt5::C4 => {
            let v11: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v12: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a413 = f10(v2);
            let a414 = 10u64;
            if nat_eq(a413, a414) {
                Ok(v11)
            } else {
                Ok(v12)
            }
        }
        Adt5::C5 => {
            let v13: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v2.clone()), std::rc::Rc::new(Adt0::C14)))));
            let v14: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a415 = f10(v2);
            let a416 = 10u64;
            if nat_eq(a415, a416) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt5::C6 => {
            let v15: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v16: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a417 = f10(v2);
            let a418 = 2u64;
            if nat_eq(a417, a418) {
                Ok(v15)
            } else {
                Ok(v16)
            }
        }
        Adt5::C7 => {
            let v17: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)));
            let v18: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a419 = f10(v2);
            let a420 = 2u64;
            if nat_eq(a419, a420) {
                Ok(v17)
            } else {
                Ok(v18)
            }
        }
        Adt5::C8 => {
            let v19: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)))));
            let v20: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a421 = f10(v2);
            let a422 = 11u64;
            if nat_eq(a421, a422) {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt5::C9 => {
            let v21: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v22: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a423 = f10(v2);
            let a424 = 1u64;
            if nat_eq(a423, a424) {
                Ok(v21)
            } else {
                Ok(v22)
            }
        }
        Adt5::C10 => {
            let v23: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v24: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a425 = f10(v2);
            let a426 = 1u64;
            if nat_eq(a425, a426) {
                Ok(v23)
            } else {
                Ok(v24)
            }
        }
        Adt5::C11 => {
            let v25: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v26: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a427 = f10(v2);
            let a428 = 0u64;
            if nat_eq(a427, a428) {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt5::C12 => {
            let v27: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v28: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a429 = f10(v2);
            let a430 = 7u64;
            if nat_eq(a429, a430) {
                Ok(v27)
            } else {
                Ok(v28)
            }
        }
        Adt5::C13 => {
            let v29: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v30: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a431 = f10(v2);
            let a432 = 7u64;
            if nat_eq(a431, a432) {
                Ok(v29)
            } else {
                Ok(v30)
            }
        }
        Adt5::C14 => {
            let v31: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v32: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a433 = f10(v2);
            let a434 = 7u64;
            if nat_eq(a433, a434) {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt5::C15(v33) => {
            let v34: Result<Adt0, Bytes> = f60(v0, f48(v2.clone()), v33)?;
            let v35: Result<Adt0, Bytes> = v34;
            let v36: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a435 = f10(v2);
            let a436 = 12u64;
            if nat_eq(a435, a436) {
                Ok(v35)
            } else {
                Ok(v36)
            }
        }
    }
}

pub fn f41(v0: Adt2) -> R<u64> {
    let m437 = v0;
    match m437 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r440) => {
            let v33 = (*r440).clone();
            let v34: u64 = f41(v33)?;
            let a438 = 1u64;
            let a439 = v34;
            nat_add(a438, a439)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f42(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("function ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f43(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C13(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f44(v0: Adt0) -> Adt0 {
    let m441 = f63(v0);
    match m441 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f45(v0: Adt0) -> Adt0 {
    let m442 = f64(v0);
    match m442 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f46(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    let v2: Adt0 = f44(v1.clone());
    let v3: Adt0 = if f6(v1.clone()) {
        Adt0::C0
    } else {
        v2
    };
    let m443 = v0.clone();
    match m443 {
        Adt6::C0 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C1 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C2 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C3 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C4 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C5 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C6 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C7 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C8 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C9 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C10 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C11 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C3)
        }
        Adt6::C12 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C13 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C14 => {
            let v4: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v5: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a444 = f10(v3);
            let a445 = 4u64;
            if nat_eq(a444, a445) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt6::C15 => {
            let v6: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v7: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a446 = f10(v3);
            let a447 = 4u64;
            if nat_eq(a446, a447) {
                Ok(v6)
            } else {
                Ok(v7)
            }
        }
        Adt6::C16 => {
            let v8: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v9: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a448 = f10(v3);
            let a449 = 4u64;
            if nat_eq(a448, a449) {
                Ok(v8)
            } else {
                Ok(v9)
            }
        }
        Adt6::C17 => {
            let v12: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v13: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a452 = f10(v3.clone());
            let a453 = 4u64;
            if if nat_eq(a452, a453) {
                f67(f58(v3))
            } else {
                false
            } {
                Ok(v12)
            } else {
                Ok(v13)
            }
        }
        Adt6::C18 => {
            let v10: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v11: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a450 = f10(v3);
            let a451 = 4u64;
            if nat_eq(a450, a451) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt6::C19 => {
            let v14: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v15: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a454 = f10(v3);
            let a455 = 4u64;
            if nat_eq(a454, a455) {
                Ok(v14)
            } else {
                Ok(v15)
            }
        }
        Adt6::C20 => {
            let v16: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v17: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a456 = f10(v3);
            let a457 = 4u64;
            if nat_eq(a456, a457) {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt6::C21 => {
            let v18: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v19: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a458 = f10(v3);
            let a459 = 4u64;
            if nat_eq(a458, a459) {
                Ok(v18)
            } else {
                Ok(v19)
            }
        }
        Adt6::C22 => {
            let v20: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), v3.clone())?;
            let v21: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a460 = f10(v3);
            let a461 = 4u64;
            if nat_eq(a460, a461) {
                Ok(v20)
            } else {
                Ok(v21)
            }
        }
        Adt6::C23 => {
            let v22: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v23: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a462 = f10(v3);
            let a463 = 4u64;
            if nat_eq(a462, a463) {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt6::C24 => {
            let v24: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v25: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a464 = f10(v3);
            let a465 = 4u64;
            if nat_eq(a464, a465) {
                Ok(v24)
            } else {
                Ok(v25)
            }
        }
        Adt6::C25 => {
            let v26: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C1)?;
            let v27: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a466 = f10(v3.clone());
            let a467 = 2u64;
            if if nat_eq(a466, a467) {
                true
            } else {
                let a468 = f10(v3.clone());
                let a469 = 1u64;
                if nat_eq(a468, a469) {
                    true
                } else {
                    let a470 = f10(v3.clone());
                    let a471 = 4u64;
                    if nat_eq(a470, a471) {
                        true
                    } else {
                        let a472 = f10(v3.clone());
                        let a473 = 5u64;
                        if nat_eq(a472, a473) {
                            true
                        } else {
                            let a474 = f10(v3.clone());
                            let a475 = 6u64;
                            if nat_eq(a474, a475) {
                                true
                            } else {
                                let a476 = f10(v3);
                                let a477 = 7u64;
                                nat_eq(a476, a477)
                            }
                        }
                    }
                }
            } {
                Ok(v26)
            } else {
                Ok(v27)
            }
        }
        Adt6::C26 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C1)
        }
        Adt6::C27 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C28 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C29 => {
            let v28: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v29: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a478 = f10(v3.clone());
            let a479 = 10u64;
            if if nat_eq(a478, a479) {
                true
            } else {
                let a480 = f10(v3);
                let a481 = 6u64;
                nat_eq(a480, a481)
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt6::C30 => {
            let v30: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C2)?;
            let v31: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a482 = f10(v3.clone());
            let a483 = 10u64;
            if if nat_eq(a482, a483) {
                true
            } else {
                let a484 = f10(v3.clone());
                let a485 = 6u64;
                if nat_eq(a484, a485) {
                    true
                } else {
                    let a486 = f10(v3);
                    let a487 = 5u64;
                    nat_eq(a486, a487)
                }
            } {
                Ok(v30)
            } else {
                Ok(v31)
            }
        }
        Adt6::C31 => {
            let v32: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(f44(v3.clone()))))?;
            let v33: Result<Adt0, Bytes> = f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(Adt1::C0))))?;
            let a488 = f10(v3.clone());
            let a489 = 10u64;
            if nat_eq(a488, a489) {
                Ok(v32)
            } else {
                let a490 = f10(v3.clone());
                let a491 = 6u64;
                if nat_eq(a490, a491) {
                    Ok(v33)
                } else {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive Index of ")), f47(v3)?)))
                }
            }
        }
        Adt6::C32 => {
            let v34: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v35: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a492 = f10(v3.clone());
            let a493 = 10u64;
            if if nat_eq(a492, a493) {
                true
            } else {
                let a494 = f10(v3);
                let a495 = 6u64;
                nat_eq(a494, a495)
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt6::C33 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C6)
        }
        Adt6::C34 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C5)))
        }
        Adt6::C35 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
        }
        Adt6::C36 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5)))))
        }
        Adt6::C37 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5))), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C5)
        }
        Adt6::C38 => {
            let v36: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C5)?;
            let v37: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a496 = f10(v3.clone());
            let a497 = 3u64;
            if if nat_eq(a496, a497) {
                true
            } else {
                let a498 = f10(v3);
                let a499 = 4u64;
                nat_eq(a498, a499)
            } {
                Ok(v36)
            } else {
                Ok(v37)
            }
        }
        Adt6::C39 => {
            let v38: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C7)?;
            let v39: Result<Adt0, Bytes> = f66(v0, v1)?;
            if f68(v3) {
                Ok(v38)
            } else {
                Ok(v39)
            }
        }
        Adt6::C40(v40) => {
            let v41: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(Adt0::C4(v40))))?;
            let v42: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a500 = f10(v3.clone());
            let a501 = 3u64;
            if if nat_eq(a500, a501) {
                true
            } else {
                let a502 = f10(v3);
                let a503 = 4u64;
                nat_eq(a502, a503)
            } {
                Ok(v41)
            } else {
                Ok(v42)
            }
        }
        Adt6::C41(r508) => {
            let v43 = (*r508).clone();
            let v44: u64 = f10(v43.clone());
            let v45: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v43)))?;
            let v46: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a504 = v44;
            let a505 = 3u64;
            if if nat_eq(a504, a505) {
                true
            } else {
                let a506 = v44;
                let a507 = 4u64;
                nat_eq(a506, a507)
            } {
                Ok(v45)
            } else {
                Ok(v46)
            }
        }
        Adt6::C42 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C0)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C6)))
        }
    }
}

pub fn f47(v0: Adt0) -> R<Bytes> {
    let m509 = v0;
    match m509 {
        Adt0::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f1(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f1(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f1(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f4(f1(Str::lit("Fixed { width: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f1(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f1(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f1(Str::lit("Ordering")))
        }
        Adt0::C8(r510) => {
            let v2 = (*r510).clone();
            Ok(f4(f1(Str::lit("Option { value: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
        Adt0::C9(r511, r512) => {
            let v3 = (*r511).clone();
            let v4 = (*r512).clone();
            Ok(f4(f1(Str::lit("Result { ok: ")), f4(f47(v3)?, f4(f1(Str::lit(", error: ")), f4(f47(v4)?, f1(Str::lit(" }")))))))
        }
        Adt0::C10(r513) => {
            let v5 = (*r513).clone();
            Ok(f4(f1(Str::lit("List { element: ")), f4(f47(v5)?, f1(Str::lit(" }")))))
        }
        Adt0::C11(r514, r515) => {
            let v6 = (*r514).clone();
            let v7 = (*r515).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f47(v6)?, f4(f1(Str::lit(", right: ")), f4(f47(v7)?, f1(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f4(f1(Str::lit("Adt { index: ")), f4(f7(v8)?, f1(Str::lit(" }")))))
        }
        Adt0::C13(r516, r517) => {
            let v9 = (*r516).clone();
            let v10 = (*r517).clone();
            Ok(f4(f1(Str::lit("Fn { parameters: [")), f4(f47(v9)?, f4(f1(Str::lit("], result: ")), f4(f47(v10)?, f1(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f1(Str::lit("")))
        }
        Adt0::C15(r518, r519) => {
            let v11 = (*r518).clone();
            let v12 = (*r519).clone();
            let v13: Bytes = f47(v11)?;
            let v14: Bytes = f47(v12.clone())?;
            if f6(v12) {
                Ok(v13)
            } else {
                Ok(f4(v13, f4(f1(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f48(v0: Adt0) -> u64 {
    let m520 = v0;
    match m520 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f50(v0: List<Adt5>, v1: Adt5) -> bool {
    let m521 = v0;
    match m521.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f70(v2, v1.clone());
            let v5: bool = f50(v3, v1);
            if v4 {
                true
            } else {
                v5
            }
        }
    }
}

pub fn f51(v0: Adt5, v1: List<Adt5>) -> List<Adt5> {
    List::cons(v0, v1)
}

pub fn f52(v0: Adt0, v1: Adt0) -> bool {
    let m522 = v0;
    match m522 {
        Adt0::C0 => {
            let a523 = f10(v1);
            let a524 = 0u64;
            nat_eq(a523, a524)
        }
        Adt0::C1 => {
            let a525 = f10(v1);
            let a526 = 1u64;
            nat_eq(a525, a526)
        }
        Adt0::C2 => {
            let a527 = f10(v1);
            let a528 = 2u64;
            nat_eq(a527, a528)
        }
        Adt0::C3 => {
            let a529 = f10(v1);
            let a530 = 3u64;
            nat_eq(a529, a530)
        }
        Adt0::C4(v2) => {
            let a539 = f10(v1.clone());
            let a540 = 4u64;
            if nat_eq(a539, a540) {
                let a541 = f57(v2);
                let a542 = f57(f58(v1));
                nat_eq(a541, a542)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a531 = f10(v1);
            let a532 = 5u64;
            nat_eq(a531, a532)
        }
        Adt0::C6 => {
            let a533 = f10(v1);
            let a534 = 6u64;
            nat_eq(a533, a534)
        }
        Adt0::C7 => {
            let a535 = f10(v1);
            let a536 = 7u64;
            nat_eq(a535, a536)
        }
        Adt0::C8(r559) => {
            let v3 = (*r559).clone();
            let v4: bool = f52(v3, f44(v1.clone()));
            let a543 = f10(v1);
            let a544 = 8u64;
            if nat_eq(a543, a544) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r560, r561) => {
            let v5 = (*r560).clone();
            let v6 = (*r561).clone();
            let v7: bool = f52(v5, f44(v1.clone()));
            let v8: bool = f52(v6, f45(v1.clone()));
            let a545 = f10(v1);
            let a546 = 9u64;
            if nat_eq(a545, a546) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r562) => {
            let v9 = (*r562).clone();
            let v10: bool = f52(v9, f44(v1.clone()));
            let a547 = f10(v1);
            let a548 = 10u64;
            if nat_eq(a547, a548) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r563, r564) => {
            let v11 = (*r563).clone();
            let v12 = (*r564).clone();
            let v13: bool = f52(v11, f44(v1.clone()));
            let v14: bool = f52(v12, f45(v1.clone()));
            let a549 = f10(v1);
            let a550 = 11u64;
            if nat_eq(a549, a550) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a551 = f10(v1.clone());
            let a552 = 12u64;
            if nat_eq(a551, a552) {
                let a553 = v15;
                let a554 = f48(v1);
                nat_eq(a553, a554)
            } else {
                false
            }
        }
        Adt0::C13(r565, r566) => {
            let v16 = (*r565).clone();
            let v17 = (*r566).clone();
            let v18: bool = f52(v16, f44(v1.clone()));
            let v19: bool = f52(v17, f45(v1.clone()));
            let a555 = f10(v1);
            let a556 = 13u64;
            if nat_eq(a555, a556) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a537 = f10(v1);
            let a538 = 14u64;
            nat_eq(a537, a538)
        }
        Adt0::C15(r567, r568) => {
            let v20 = (*r567).clone();
            let v21 = (*r568).clone();
            let v22: bool = f52(v20, f44(v1.clone()));
            let v23: bool = f52(v21, f45(v1.clone()));
            let a557 = f10(v1);
            let a558 = 15u64;
            if nat_eq(a557, a558) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f71(v0, v1, 48u8) {
        Ok(Some({
            let a569 = v1;
            let a570 = 1u64;
            nat_add(a569, a570)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Adt3) -> R<Bytes> {
    let m571 = v0;
    match m571 {
        Adt3::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f4(f1(Str::lit("Bool { value: ")), f4(f72(v1), f1(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f4(f1(Str::lit("Nat { value: ")), f4(f73(v2)?, f1(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f4(f1(Str::lit("Int { value: ")), f4(f73(v3)?, f1(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f4(f1(Str::lit("U8 { value: ")), f4(f73(v4)?, f1(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f4(f1(Str::lit("U16 { value: ")), f4(f73(v5)?, f1(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f4(f1(Str::lit("U32 { value: ")), f4(f73(v6)?, f1(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f4(f1(Str::lit("U64 { value: ")), f4(f73(v7)?, f1(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f4(f1(Str::lit("I8 { value: ")), f4(f73(v8)?, f1(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f4(f1(Str::lit("I16 { value: ")), f4(f73(v9)?, f1(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f4(f1(Str::lit("I32 { value: ")), f4(f73(v10)?, f1(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f4(f1(Str::lit("I64 { value: ")), f4(f73(v11)?, f1(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f4(f1(Str::lit("String { value: ")), f4(f73(v12)?, f1(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f4(f1(Str::lit("Bytes { hex: ")), f4(f73(v13)?, f1(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f4(f1(Str::lit("Ordering { value: ")), f4(f74(v14), f1(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f1(Str::lit("None")))
        }
        Adt3::C16(r572) => {
            let v15 = (*r572).clone();
            Ok(f4(f1(Str::lit("Some { value: ")), f4(f54(v15)?, f1(Str::lit(" }")))))
        }
        Adt3::C17(r573) => {
            let v16 = (*r573).clone();
            Ok(f4(f1(Str::lit("Ok { value: ")), f4(f54(v16)?, f1(Str::lit(" }")))))
        }
        Adt3::C18(r574) => {
            let v17 = (*r574).clone();
            Ok(f4(f1(Str::lit("Error { value: ")), f4(f54(v17)?, f1(Str::lit(" }")))))
        }
        Adt3::C19(r575) => {
            let v18 = (*r575).clone();
            Ok(f4(f1(Str::lit("List { items: [")), f4(f54(v18)?, f1(Str::lit("] }")))))
        }
        Adt3::C20(r576, r577) => {
            let v19 = (*r576).clone();
            let v20 = (*r577).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f54(v19)?, f4(f1(Str::lit(", right: ")), f4(f54(v20)?, f1(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r578) => {
            let v22 = (*r578).clone();
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v21)?, f4(f1(Str::lit(", fields: [")), f4(f54(v22)?, f1(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r579) => {
            let v24 = (*r579).clone();
            Ok(f4(f1(Str::lit("Closure { function: ")), f4(f7(v23)?, f4(f1(Str::lit(", captures: [")), f4(f54(v24)?, f1(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f1(Str::lit("")))
        }
        Adt3::C24(r580, r581) => {
            let v25 = (*r580).clone();
            let v26 = (*r581).clone();
            let v27: Bytes = f54(v25)?;
            let v28: Bytes = f54(v26.clone())?;
            if f75(v26) {
                Ok(v27)
            } else {
                Ok(f4(v27, f4(f1(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f55(v0: Bytes) -> R<Result<(), Bytes>> {
    let m582 = f76(v0.clone())?;
    match m582 {
        Ok(v1) => {
            let v2: bool = f77(match v1.clone() {
                Adt9::C0(_, h583) => {
                    h583
                }
            }, f1(Str::lit("18446744073709551615")));
            if match v1 {
                Adt9::C0(h584, _) => {
                    h584
                }
            } {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m585 = f76(v0.clone())?;
    match m585 {
        Ok(v2) => {
            let v3: Adt9 = f78(v1.clone());
            let v4: Adt9 = f79(v1.clone());
            let v5: bool = f80(v3, v2.clone());
            let v6: bool = f80(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f57(v0: Adt1) -> u64 {
    let m586 = v0;
    match m586 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f58(v0: Adt0) -> Adt1 {
    let m587 = v0;
    match m587 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f59(v0: Bytes) -> R<Result<(), Bytes>> {
    let a592 = {
        let a589 = {
            let a588 = v0.clone();
            length_bytes(a588)
        };
        let a590 = 2u64;
        let a591 = 0u64;
        nat_rem(a589, a590, a591)
    };
    let a593 = 0u64;
    if nat_eq(a592, a593) {
        if f82(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f60(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m594 = f26(v0, v1);
    match m594 {
        None => {
            Ok(Err::<Adt0, Bytes>(f27(v1)?))
        }
        Some(v3) => {
            let m595 = f30(v3, v2);
            match m595 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v1)?, f4(f1(Str::lit(" has no constructor ")), f7(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f61(v0: Adt3) -> R<u64> {
    let m596 = v0;
    match m596 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_) => {
            Ok(0u64)
        }
        Adt3::C4(_) => {
            Ok(0u64)
        }
        Adt3::C5(_) => {
            Ok(0u64)
        }
        Adt3::C6(_) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_) => {
            Ok(0u64)
        }
        Adt3::C9(_) => {
            Ok(0u64)
        }
        Adt3::C10(_) => {
            Ok(0u64)
        }
        Adt3::C11(_) => {
            Ok(0u64)
        }
        Adt3::C12(_) => {
            Ok(0u64)
        }
        Adt3::C13(_) => {
            Ok(0u64)
        }
        Adt3::C14(_) => {
            Ok(0u64)
        }
        Adt3::C15 => {
            Ok(0u64)
        }
        Adt3::C16(_) => {
            Ok(0u64)
        }
        Adt3::C17(_) => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _) => {
            Ok(0u64)
        }
        Adt3::C22(_, _) => {
            Ok(0u64)
        }
        Adt3::C23 => {
            Ok(0u64)
        }
        Adt3::C24(_, r599) => {
            let v26 = (*r599).clone();
            let v27: u64 = f61(v26)?;
            let a597 = 1u64;
            let a598 = v27;
            nat_add(a597, a598)
        }
    }
}

pub fn f62(v0: Adt0, v1: u64) -> R<List<Adt5>> {
    let m600 = v0;
    match m600 {
        Adt0::C0 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C1 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C2 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C3 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C6 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C7 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C8(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C9(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C10(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C11(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C12(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C13(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C14 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C15(_, r603) => {
            let v13 = (*r603).clone();
            Ok(f51(Adt5::C15(v1), f62(v13, {
                let a601 = v1;
                let a602 = 1u64;
                nat_add(a601, a602)?
            })?))
        }
    }
}

pub fn f63(v0: Adt0) -> Option<Adt0> {
    let m604 = v0;
    match m604 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r605) => {
            let v2 = (*r605).clone();
            Some(v2)
        }
        Adt0::C9(r606, _) => {
            let v3 = (*r606).clone();
            Some(v3)
        }
        Adt0::C10(r607) => {
            let v5 = (*r607).clone();
            Some(v5)
        }
        Adt0::C11(r608, _) => {
            let v6 = (*r608).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r609, _) => {
            let v9 = (*r609).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r610, _) => {
            let v11 = (*r610).clone();
            Some(v11)
        }
    }
}

pub fn f64(v0: Adt0) -> Option<Adt0> {
    let m611 = v0;
    match m611 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r612) => {
            let v4 = (*r612).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r613) => {
            let v7 = (*r613).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r614) => {
            let v10 = (*r614).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r615) => {
            let v12 = (*r615).clone();
            Some(v12)
        }
    }
}

pub fn f65(v0: Adt6, v1: Adt0, v2: Adt0, v3: Adt0) -> R<Result<Adt0, Bytes>> {
    let v4: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(v3);
    let v5: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" takes ")), f4(f84(v1.clone())?, f4(f1(Str::lit(", received ")), f84(v2.clone())?))))));
    if f52(v2, v1) {
        Ok(v4)
    } else {
        Ok(v5)
    }
}

pub fn f66(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" does not apply to ")), f84(v1)?)))))
}

pub fn f67(v0: Adt1) -> bool {
    let m616 = v0;
    match m616 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            false
        }
        Adt1::C3 => {
            false
        }
        Adt1::C4 => {
            true
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6 => {
            true
        }
        Adt1::C7 => {
            true
        }
    }
}

pub fn f68(v0: Adt0) -> bool {
    let m617 = v0;
    match m617 {
        Adt0::C0 => {
            false
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            false
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r618, r619) => {
            let v2 = (*r618).clone();
            let v3 = (*r619).clone();
            let v4: bool = f68(v2);
            let v5: bool = f68(v3);
            if v4 {
                v5
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f69(v0: Adt1) -> Bytes {
    let m620 = v0;
    match m620 {
        Adt1::C0 => {
            f1(Str::lit("U8"))
        }
        Adt1::C1 => {
            f1(Str::lit("U16"))
        }
        Adt1::C2 => {
            f1(Str::lit("U32"))
        }
        Adt1::C3 => {
            f1(Str::lit("U64"))
        }
        Adt1::C4 => {
            f1(Str::lit("I8"))
        }
        Adt1::C5 => {
            f1(Str::lit("I16"))
        }
        Adt1::C6 => {
            f1(Str::lit("I32"))
        }
        Adt1::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f70(v0: Adt5, v1: Adt5) -> bool {
    let a621 = f85(v0.clone());
    let a622 = f85(v1.clone());
    if nat_eq(a621, a622) {
        let a623 = f86(v0);
        let a624 = f86(v1);
        nat_eq(a623, a624)
    } else {
        false
    }
}

pub fn f71(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m625 = f87(v0, v1);
    match m625 {
        None => {
            false
        }
        Some(v3) => {
            let a626 = v3;
            let a627 = v2;
            equal(a626, a627)
        }
    }
}

pub fn f72(v0: bool) -> Bytes {
    if v0 {
        f1(Str::lit("true"))
    } else {
        f1(Str::lit("false"))
    }
}

pub fn f73(v0: Bytes) -> R<Bytes> {
    Ok(f4(f1(Str::lit("\"")), f4({
        let (_, h628) = {
            let (h629, _) = f88(Fn2::F89(v0.clone()), {
                let a630 = v0;
                length_bytes(a630)
            }, (0u64, Bytes::lit(&[])))?;
            h629
        };
        h628
    }, f1(Str::lit("\"")))))
}

pub fn f74(v0: Adt4) -> Bytes {
    let m631 = v0;
    match m631 {
        Adt4::C0 => {
            f1(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f1(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f1(Str::lit("Gt"))
        }
    }
}

pub fn f75(v0: Adt3) -> bool {
    let m632 = v0;
    match m632 {
        Adt3::C0 => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_) => {
            false
        }
        Adt3::C5(_) => {
            false
        }
        Adt3::C6(_) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_) => {
            false
        }
        Adt3::C9(_) => {
            false
        }
        Adt3::C10(_) => {
            false
        }
        Adt3::C11(_) => {
            false
        }
        Adt3::C12(_) => {
            false
        }
        Adt3::C13(_) => {
            false
        }
        Adt3::C14(_) => {
            false
        }
        Adt3::C15 => {
            false
        }
        Adt3::C16(_) => {
            false
        }
        Adt3::C17(_) => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _) => {
            false
        }
        Adt3::C21(_, _) => {
            false
        }
        Adt3::C22(_, _) => {
            false
        }
        Adt3::C23 => {
            true
        }
        Adt3::C24(_, _) => {
            false
        }
    }
}

pub fn f76(v0: Bytes) -> R<Result<Adt9, Bytes>> {
    let v1: bool = f71(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f25(v0.clone(), 1u64, {
            let a634 = {
                let a633 = v0.clone();
                length_bytes(a633)
            };
            let a635 = 1u64;
            nat_sub(a634, a635)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f90(v3.clone(), v0.clone())?;
    let v5: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt9, Bytes>(Adt9::C0(v1, v3)))
        } else {
            Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a638 = {
            let a636 = v0.clone();
            length_bytes(a636)
        };
        let a639 = {
            let a637 = v1.clone();
            length_bytes(a637)
        };
        nat_lt(a638, a639)
    };
    let v3: bool = {
        let a642 = {
            let a640 = v0.clone();
            length_bytes(a640)
        };
        let a643 = {
            let a641 = v1.clone();
            length_bytes(a641)
        };
        nat_eq(a642, a643)
    };
    let v4: bool = f91(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a644 = v4;
            bool_not(a644)
        } else {
            false
        }
    }
}

pub fn f78(v0: Adt1) -> Adt9 {
    let m645 = v0;
    match m645 {
        Adt1::C0 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f92(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f92(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f92(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f92(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f79(v0: Adt1) -> Adt9 {
    let m646 = v0;
    match m646 {
        Adt1::C0 => {
            f92(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f92(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f92(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f92(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f92(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f80(v0: Adt9, v1: Adt9) -> bool {
    let v2: bool = f77(match v1.clone() {
        Adt9::C0(_, h647) => {
            h647
        }
    }, match v0.clone() {
        Adt9::C0(_, h648) => {
            h648
        }
    });
    let v3: bool = f77(match v0.clone() {
        Adt9::C0(_, h649) => {
            h649
        }
    }, match v1.clone() {
        Adt9::C0(_, h650) => {
            h650
        }
    });
    if match v0 {
        Adt9::C0(h651, _) => {
            h651
        }
    } {
        if match v1 {
            Adt9::C0(h652, _) => {
                h652
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt9::C0(h653, _) => {
                h653
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f81(v0: Adt1) -> Bytes {
    let m654 = v0;
    match m654 {
        Adt1::C0 => {
            f1(Str::lit("u8"))
        }
        Adt1::C1 => {
            f1(Str::lit("u16"))
        }
        Adt1::C2 => {
            f1(Str::lit("u32"))
        }
        Adt1::C3 => {
            f1(Str::lit("u64"))
        }
        Adt1::C4 => {
            f1(Str::lit("i8"))
        }
        Adt1::C5 => {
            f1(Str::lit("i16"))
        }
        Adt1::C6 => {
            f1(Str::lit("i32"))
        }
        Adt1::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f82(v0: Bytes) -> R<bool> {
    let a658 = {
        let (h655, _) = f23(Fn0::F93(v0.clone()), {
            let a656 = v0.clone();
            length_bytes(a656)
        }, 0u64)?;
        h655
    };
    let a659 = {
        let a657 = v0;
        length_bytes(a657)
    };
    Ok(nat_eq(a658, a659))
}

pub fn f83(v0: Adt6) -> R<Bytes> {
    let m660 = v0;
    match m660 {
        Adt6::C0 => {
            Ok(f1(Str::lit("NatAdd")))
        }
        Adt6::C1 => {
            Ok(f1(Str::lit("NatSub")))
        }
        Adt6::C2 => {
            Ok(f1(Str::lit("NatMul")))
        }
        Adt6::C3 => {
            Ok(f1(Str::lit("NatQuot")))
        }
        Adt6::C4 => {
            Ok(f1(Str::lit("NatRem")))
        }
        Adt6::C5 => {
            Ok(f1(Str::lit("NatEq")))
        }
        Adt6::C6 => {
            Ok(f1(Str::lit("NatLe")))
        }
        Adt6::C7 => {
            Ok(f1(Str::lit("NatLt")))
        }
        Adt6::C8 => {
            Ok(f1(Str::lit("IntAdd")))
        }
        Adt6::C9 => {
            Ok(f1(Str::lit("IntSub")))
        }
        Adt6::C10 => {
            Ok(f1(Str::lit("IntMul")))
        }
        Adt6::C11 => {
            Ok(f1(Str::lit("IntNeg")))
        }
        Adt6::C12 => {
            Ok(f1(Str::lit("IntQuot")))
        }
        Adt6::C13 => {
            Ok(f1(Str::lit("IntRem")))
        }
        Adt6::C14 => {
            Ok(f1(Str::lit("CheckedAdd")))
        }
        Adt6::C15 => {
            Ok(f1(Str::lit("CheckedSub")))
        }
        Adt6::C16 => {
            Ok(f1(Str::lit("CheckedMul")))
        }
        Adt6::C17 => {
            Ok(f1(Str::lit("CheckedNeg")))
        }
        Adt6::C18 => {
            Ok(f1(Str::lit("CheckedQuot")))
        }
        Adt6::C19 => {
            Ok(f1(Str::lit("BitAnd")))
        }
        Adt6::C20 => {
            Ok(f1(Str::lit("BitOr")))
        }
        Adt6::C21 => {
            Ok(f1(Str::lit("BitXor")))
        }
        Adt6::C22 => {
            Ok(f1(Str::lit("BitNot")))
        }
        Adt6::C23 => {
            Ok(f1(Str::lit("ShiftLeft")))
        }
        Adt6::C24 => {
            Ok(f1(Str::lit("ShiftRight")))
        }
        Adt6::C25 => {
            Ok(f1(Str::lit("Equal")))
        }
        Adt6::C26 => {
            Ok(f1(Str::lit("BoolNot")))
        }
        Adt6::C27 => {
            Ok(f1(Str::lit("BoolAnd")))
        }
        Adt6::C28 => {
            Ok(f1(Str::lit("BoolOr")))
        }
        Adt6::C29 => {
            Ok(f1(Str::lit("Append")))
        }
        Adt6::C30 => {
            Ok(f1(Str::lit("Length")))
        }
        Adt6::C31 => {
            Ok(f1(Str::lit("Index")))
        }
        Adt6::C32 => {
            Ok(f1(Str::lit("Slice")))
        }
        Adt6::C33 => {
            Ok(f1(Str::lit("Utf8Encode")))
        }
        Adt6::C34 => {
            Ok(f1(Str::lit("Utf8Decode")))
        }
        Adt6::C35 => {
            Ok(f1(Str::lit("CompareBytes")))
        }
        Adt6::C36 => {
            Ok(f1(Str::lit("SplitExact")))
        }
        Adt6::C37 => {
            Ok(f1(Str::lit("Join")))
        }
        Adt6::C38 => {
            Ok(f1(Str::lit("FormatDecimal")))
        }
        Adt6::C39 => {
            Ok(f1(Str::lit("Compare")))
        }
        Adt6::C40(v1) => {
            Ok(f4(f1(Str::lit("Convert { target: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt6::C41(r661) => {
            let v2 = (*r661).clone();
            Ok(f4(f1(Str::lit("ParseDecimal { target: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
        Adt6::C42 => {
            Ok(f1(Str::lit("Update")))
        }
    }
}

pub fn f84(v0: Adt0) -> R<Bytes> {
    Ok(f4(f1(Str::lit("[")), f4(f47(v0)?, f1(Str::lit("]")))))
}

pub fn f85(v0: Adt5) -> u64 {
    let m662 = v0;
    match m662 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            1u64
        }
        Adt5::C2 => {
            2u64
        }
        Adt5::C3 => {
            3u64
        }
        Adt5::C4 => {
            4u64
        }
        Adt5::C5 => {
            5u64
        }
        Adt5::C6 => {
            6u64
        }
        Adt5::C7 => {
            7u64
        }
        Adt5::C8 => {
            8u64
        }
        Adt5::C9 => {
            9u64
        }
        Adt5::C10 => {
            10u64
        }
        Adt5::C11 => {
            11u64
        }
        Adt5::C12 => {
            12u64
        }
        Adt5::C13 => {
            13u64
        }
        Adt5::C14 => {
            14u64
        }
        Adt5::C15(_) => {
            15u64
        }
    }
}

pub fn f86(v0: Adt5) -> u64 {
    let m663 = v0;
    match m663 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            0u64
        }
        Adt5::C2 => {
            0u64
        }
        Adt5::C3 => {
            0u64
        }
        Adt5::C4 => {
            0u64
        }
        Adt5::C5 => {
            0u64
        }
        Adt5::C6 => {
            0u64
        }
        Adt5::C7 => {
            0u64
        }
        Adt5::C8 => {
            0u64
        }
        Adt5::C9 => {
            0u64
        }
        Adt5::C10 => {
            0u64
        }
        Adt5::C11 => {
            0u64
        }
        Adt5::C12 => {
            0u64
        }
        Adt5::C13 => {
            0u64
        }
        Adt5::C14 => {
            0u64
        }
        Adt5::C15(v1) => {
            v1
        }
    }
}

pub fn f87(v0: Bytes, v1: u64) -> Option<u8> {
    let a664 = v0;
    let a665 = v1;
    index_bytes(a664, a665)
}

pub fn f88(v0: Fn2, p670: u64, p671: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m669 = (p670, p671);
    {
        let mut m669 = m669;
        loop {
            let step = {
                let (v1, v2) = m669;
                let m666 = v1;
                if m666 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m666 - 1;
                    let m668 = {
                        let c667: Fn2 = v0.clone();
                        c667.apply(v2.clone())?
                    };
                    match m668 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f94(v0, v1)
}

pub fn f90(v0: Bytes, v1: Bytes) -> R<bool> {
    let a673 = 0u64;
    let a674 = {
        let a672 = v0.clone();
        length_bytes(a672)
    };
    if nat_lt(a673, a674) {
        if f95(v0.clone())? {
            let a675 = v0.clone();
            let a676 = f1(Str::lit("0"));
            if equal(a675, a676) {
                let a679 = {
                    let a677 = v1;
                    let a678 = f1(Str::lit("-0"));
                    equal(a677, a678)
                };
                Ok(bool_not(a679))
            } else {
                let a680 = f71(v0, 0u64, 48u8);
                Ok(bool_not(a680))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Bytes, v1: Bytes) -> bool {
    let a685 = {
        let a681 = v0;
        let a682 = v1;
        compare_bytes(a681, a682)
    };
    let a686 = {
        let a683 = Bytes::lit(&[]);
        let a684 = Bytes::lit(&[0x01]);
        compare_bytes(a683, a684)
    };
    equal(a685, a686)
}

pub fn f92(v0: bool, v1: Str) -> Adt9 {
    Adt9::C0(v0, f1(v1))
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f96(v0, v1)
}

pub fn f94(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h687, _) = v1;
        h687
    };
    let v3: u64 = f97(v0.clone(), v2)?;
    let a688 = v3;
    let a689 = 256u64;
    if nat_eq(a688, a689) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f98(v0, v2, v3)?;
        Ok(Some(({
            let a691 = v2;
            let a692 = {
                let (h690, _) = v4;
                h690
            };
            nat_add(a691, a692)?
        }, f4({
            let (_, h693) = v1;
            h693
        }, {
            let (_, h694) = v4;
            h694
        }))))
    }
}

pub fn f95(v0: Bytes) -> R<bool> {
    let a698 = {
        let (h695, _) = f23(Fn0::F99(v0.clone()), {
            let a696 = v0.clone();
            length_bytes(a696)
        }, 0u64)?;
        h695
    };
    let a699 = {
        let a697 = v0;
        length_bytes(a697)
    };
    Ok(nat_eq(a698, a699))
}

pub fn f96(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f100(v0, v1)? {
        Ok(Some({
            let a700 = v1;
            let a701 = 1u64;
            nat_add(a700, a701)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f97(v0: Bytes, v1: u64) -> R<u64> {
    let m702 = f87(v0, v1);
    match m702 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a747 = {
                let a723 = {
                    let a711 = {
                        let a705 = {
                            let a703 = v3;
                            let a704 = 128u8;
                            fixed_u8::bit_and(a703, a704)
                        };
                        let a706 = 128u8;
                        if equal(a705, a706) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a712 = {
                        let a709 = {
                            let a707 = v3;
                            let a708 = 64u8;
                            fixed_u8::bit_and(a707, a708)
                        };
                        let a710 = 64u8;
                        if equal(a709, a710) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a711, a712)?
                };
                let a724 = {
                    let a721 = {
                        let a715 = {
                            let a713 = v3;
                            let a714 = 32u8;
                            fixed_u8::bit_and(a713, a714)
                        };
                        let a716 = 32u8;
                        if equal(a715, a716) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a722 = {
                        let a719 = {
                            let a717 = v3;
                            let a718 = 16u8;
                            fixed_u8::bit_and(a717, a718)
                        };
                        let a720 = 16u8;
                        if equal(a719, a720) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a721, a722)?
                };
                nat_add(a723, a724)?
            };
            let a748 = {
                let a745 = {
                    let a733 = {
                        let a727 = {
                            let a725 = v3;
                            let a726 = 8u8;
                            fixed_u8::bit_and(a725, a726)
                        };
                        let a728 = 8u8;
                        if equal(a727, a728) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a734 = {
                        let a731 = {
                            let a729 = v3;
                            let a730 = 4u8;
                            fixed_u8::bit_and(a729, a730)
                        };
                        let a732 = 4u8;
                        if equal(a731, a732) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a733, a734)?
                };
                let a746 = {
                    let a743 = {
                        let a737 = {
                            let a735 = v3;
                            let a736 = 2u8;
                            fixed_u8::bit_and(a735, a736)
                        };
                        let a738 = 2u8;
                        if equal(a737, a738) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a744 = {
                        let a741 = {
                            let a739 = v3;
                            let a740 = 1u8;
                            fixed_u8::bit_and(a739, a740)
                        };
                        let a742 = 1u8;
                        if equal(a741, a742) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a743, a744)?
                };
                nat_add(a745, a746)?
            };
            nat_add(a747, a748)
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f97(v0.clone(), {
        let a749 = v1;
        let a750 = 1u64;
        nat_add(a749, a750)?
    })?;
    let a751 = v2;
    let a752 = 34u64;
    if nat_eq(a751, a752) {
        Ok((1u64, f1(Str::lit("\\\""))))
    } else {
        let a753 = v2;
        let a754 = 92u64;
        if nat_eq(a753, a754) {
            Ok((1u64, f1(Str::lit("\\\\"))))
        } else {
            let a755 = v2;
            let a756 = 10u64;
            if nat_eq(a755, a756) {
                Ok((1u64, f1(Str::lit("\\n"))))
            } else {
                let a757 = v2;
                let a758 = 13u64;
                if nat_eq(a757, a758) {
                    Ok((1u64, f1(Str::lit("\\r"))))
                } else {
                    let a759 = v2;
                    let a760 = 9u64;
                    if nat_eq(a759, a760) {
                        Ok((1u64, f1(Str::lit("\\t"))))
                    } else {
                        let a761 = v2;
                        let a762 = 0u64;
                        if nat_eq(a761, a762) {
                            Ok((1u64, f1(Str::lit("\\0"))))
                        } else {
                            let a763 = v2;
                            let a764 = 32u64;
                            if nat_lt(a763, a764) {
                                Ok((1u64, f101(v2)))
                            } else {
                                let a765 = v2;
                                let a766 = 127u64;
                                if nat_eq(a765, a766) {
                                    Ok((1u64, f101(v2)))
                                } else {
                                    let a767 = v2;
                                    let a768 = 194u64;
                                    if nat_eq(a767, a768) {
                                        let a769 = 128u64;
                                        let a770 = v3;
                                        if nat_le(a769, a770) {
                                            let a771 = v3;
                                            let a772 = 159u64;
                                            if nat_le(a771, a772) {
                                                Ok((2u64, f101(v3)))
                                            } else {
                                                Ok((1u64, f25(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f25(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f25(v0, v1, 1u64)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f102(v0, v1)
}

pub fn f100(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a773 = 47u64;
    let a774 = v2;
    if nat_lt(a773, a774) {
        let a775 = v2;
        let a776 = 58u64;
        if nat_lt(a775, a776) {
            Ok(true)
        } else {
            let a777 = 96u64;
            let a778 = v2;
            if nat_lt(a777, a778) {
                let a779 = v2;
                let a780 = 103u64;
                Ok(nat_lt(a779, a780))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64) -> Bytes {
    f4(f1(Str::lit("\\u{")), f4(f103(v0), f1(Str::lit("}"))))
}

pub fn f102(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f104(v0, v1)? {
        Ok(Some({
            let a781 = v1;
            let a782 = 1u64;
            nat_add(a781, a782)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f103(v0: u64) -> Bytes {
    let v1: Bytes = f105(v0);
    let v2: Bytes = f4(f105({
        let a783 = v0;
        let a784 = 16u64;
        let a785 = 0u64;
        nat_quot(a783, a784, a785)
    }), f105({
        let a786 = v0;
        let a787 = 16u64;
        let a788 = 0u64;
        nat_rem(a786, a787, a788)
    }));
    let a789 = v0;
    let a790 = 16u64;
    if nat_lt(a789, a790) {
        v1
    } else {
        v2
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a791 = 47u64;
    let a792 = v2;
    if nat_lt(a791, a792) {
        let a793 = v2;
        let a794 = 58u64;
        Ok(nat_lt(a793, a794))
    } else {
        Ok(false)
    }
}

pub fn f105(v0: u64) -> Bytes {
    f25(f1(Str::lit("0123456789abcdef")), v0, 1u64)
}
