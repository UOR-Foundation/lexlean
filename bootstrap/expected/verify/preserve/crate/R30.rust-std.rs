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
    C0(bool, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Bytes>, List<Bytes>),
    C1(Bytes, List<Bytes>),
    C2(Bytes, List<Bytes>, std::rc::Rc<Adt2>),
    C3(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt3>),
    C4(Bytes, List<Adt5>),
    C5(List<Adt7>, List<Bytes>, std::rc::Rc<Adt2>, List<Adt8>),
    C6(std::rc::Rc<Adt8>),
    C7(Bytes, List<Bytes>, List<Bytes>, std::rc::Rc<Adt2>),
    C8(Bytes, List<Bytes>, List<Bytes>, List<Bytes>, std::rc::Rc<Adt2>),
    C9(Bytes, List<Bytes>, List<Bytes>, List<Adt9>),
    C10(Bytes),
    C11(Bytes, List<Adt1>),
    C12(Bytes, List<Bytes>),
    C13(Bytes, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(u64),
    C7(std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>),
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>),
    C11,
    C12,
    C13,
    C14,
    C15(std::rc::Rc<Adt2>),
    C16,
    C17,
    C18,
    C19(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes),
    C1(u64),
    C2(List<Bytes>),
    C3(List<Bytes>, std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>, Bytes, std::rc::Rc<Adt3>),
    C5(std::rc::Rc<Adt3>, u64),
    C6(std::rc::Rc<Adt3>, Bytes),
    C7(std::rc::Rc<Adt3>),
    C8(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C9(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C10(std::rc::Rc<Adt4>, std::rc::Rc<Adt2>, std::rc::Rc<Adt3>),
    C11(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C12(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C13(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C14(std::rc::Rc<Adt4>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C15(std::rc::Rc<Adt3>),
    C16(std::rc::Rc<Adt4>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C17,
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(List<Bytes>, List<Bytes>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C22(std::rc::Rc<Adt3>),
    C23(std::rc::Rc<Adt3>, std::rc::Rc<Adt2>),
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C25(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C26(std::rc::Rc<Adt3>),
    C27(std::rc::Rc<Adt3>),
    C28(std::rc::Rc<Adt3>, Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt3>),
    C29(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C30,
    C31,
    C32(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C33(std::rc::Rc<Adt4>, std::rc::Rc<Adt2>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C34(std::rc::Rc<Adt4>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Bytes),
    C2(Bytes),
    C3(List<Bytes>),
    C4(std::rc::Rc<Adt4>),
    C5(List<Bytes>, std::rc::Rc<Adt4>),
    C6(List<Bytes>, List<Bytes>, std::rc::Rc<Adt4>),
    C7(u64),
    C8,
    C9(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, bool, bool, List<Adt6>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, bool, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt5>, List<Adt7>, bool, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes),
    C1(Bytes, std::rc::Rc<Adt2>),
    C2(Bytes, List<Bytes>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, List<Adt9>, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool, List<Bytes>, List<Adt10>, List<Adt11>, List<Adt12>, List<Adt13>, List<Bytes>, List<Adt13>, List<Adt13>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Adt7>, List<Adt17>, std::rc::Rc<Adt2>, bool),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, bool, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(std::rc::Rc<Adt14>),
    F8,
    F36(Bytes, std::rc::Rc<Adt14>),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt1) -> R<Option<Bytes>> {
        match self {
            Fn0::F5(k0) => f5((**k0).clone(), p0, p1),
            Fn0::F8 => Ok(f8(p0, p1)),
            Fn0::F36(k0, k1) => f36(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt15, p1: Adt1) -> R<Adt15> {
        match self {
            Fn1::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F11,
}

impl Fn2 {
    pub fn apply(&self, p0: Result<Adt14, Bytes>, p1: Adt1) -> R<Result<Adt14, Bytes>> {
        match self {
            Fn2::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F18(List<Adt13>, List<Adt13>),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt13) -> R<Option<Bytes>> {
        match self {
            Fn3::F18(k0, k1) => f18(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F27(std::rc::Rc<Adt14>),
    F96(List<Adt8>, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt5) -> R<Option<Bytes>> {
        match self {
            Fn4::F27(k0) => f27((**k0).clone(), p0, p1),
            Fn4::F96(k0, k1) => f96(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F31(List<Adt7>, std::rc::Rc<Adt14>, std::rc::Rc<Adt2>),
    F97(List<Adt7>, std::rc::Rc<Adt14>, List<Adt5>, Bytes, std::rc::Rc<Adt2>),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt8) -> R<Option<Bytes>> {
        match self {
            Fn5::F31(k0, k1, k2) => f31(k0.clone(), (**k1).clone(), (**k2).clone(), p0, p1),
            Fn5::F97(k0, k1, k2, k3, k4) => f97(k0.clone(), (**k1).clone(), k2.clone(), k3.clone(), (**k4).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F34(List<Bytes>, std::rc::Rc<Adt14>),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt9) -> R<Option<Bytes>> {
        match self {
            Fn6::F34(k0, k1) => f34(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F43(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt1) -> List<Adt13> {
        match self {
            Fn7::F43(k0) => f43(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F91,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt18, p1: Adt5) -> R<Adt18> {
        match self {
            Fn8::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F94(std::rc::Rc<Adt14>),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt7) -> R<Option<Bytes>> {
        match self {
            Fn9::F94(k0) => f94((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F102,
    F131(Bytes),
    F150(List<Bytes>, List<Bytes>),
    F241(Bytes, std::rc::Rc<Adt14>),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn10::F102 => Ok(f102(p0, p1)),
            Fn10::F131(k0) => Ok(f131(k0.clone(), p0, p1)),
            Fn10::F150(k0, k1) => f150(k0.clone(), k1.clone(), p0, p1),
            Fn10::F241(k0, k1) => Ok(f241(k0.clone(), (**k1).clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F111(Str),
    F112(Str),
    F152(std::rc::Rc<Adt7>, std::rc::Rc<Adt14>),
    F171(Str),
    F209(List<Bytes>, List<Bytes>),
    F210(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn11::F111(k0) => Ok(f111(k0.clone(), p0, p1)),
            Fn11::F112(k0) => f112(k0.clone(), p0, p1),
            Fn11::F152(k0, k1) => Ok(f152((**k0).clone(), (**k1).clone(), p0, p1)),
            Fn11::F171(k0) => f171(k0.clone(), p0, p1),
            Fn11::F209(k0, k1) => f209(k0.clone(), k1.clone(), p0, p1),
            Fn11::F210(k0) => Ok(f210(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F114,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn12::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F118,
    F174(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn13::F118 => f118(p0, p1),
            Fn13::F174(k0) => f174(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F121(Bytes),
    F122,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt18, p1: Adt9) -> R<Adt18> {
        match self {
            Fn14::F121(k0) => f121(k0.clone(), p0, p1),
            Fn14::F122 => f122(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Option<Adt17>, p1: Adt17) -> Option<Adt17> {
        match self {
            Fn15::F130(k0) => f130(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F162(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn16::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F166(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64, p1: Adt13) -> R<u64> {
        match self {
            Fn17::F166(k0) => f166(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F168(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<u64>, p1: Adt13) -> Option<u64> {
        match self {
            Fn18::F168(k0) => f168(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F170,
}

impl Fn19 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn19::F170 => f170(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F173(Bytes),
    F231(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn20::F173(k0) => f173(k0.clone(), p0),
            Fn20::F231(k0) => f231(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F180(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn21::F180(k0) => f180(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F182(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn22::F182(k0) => f182(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F184(Bytes),
    F234(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn23::F184(k0) => f184(k0.clone(), p0, p1),
            Fn23::F234(k0) => f234(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F196(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn24::F196(k0) => f196(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(u64),
}

impl Fn25 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn25::F198(k0) => f198(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F204,
}

impl Fn26 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn26::F204 => f204(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F213(bool, u64, std::rc::Rc<Adt14>),
}

impl Fn27 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn27::F213(k0, k1, k2) => f213(*k0, *k1, (**k2).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F215(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn28::F215(k0) => f215(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F217(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn29::F217(k0) => f217(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F219(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn30::F219(k0) => f219(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F221,
}

impl Fn31 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn31::F221 => f221(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F224(Bytes),
    F248(Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn32::F224(k0) => f224(k0.clone(), p0, p1),
            Fn32::F248(k0) => f248(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F226(Bytes),
}

impl Fn33 {
    pub fn apply(&self, p0: Result<List<Adt17>, Bytes>, p1: Adt6) -> Result<List<Adt17>, Bytes> {
        match self {
            Fn33::F226(k0) => f226(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F229,
}

impl Fn34 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn34::F229 => f229(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F233(Bytes),
}

impl Fn35 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn35::F233(k0) => f233(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F244(Bytes),
}

impl Fn36 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn36::F244(k0) => f244(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let m2 = f1(match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        None => {
            let m4 = f2(match v0.clone() {
                Adt0::C0(_, h3) => {
                    h3
                }
            })?;
            match m4 {
                None => {
                    let m7 = f3(Adt14::C0(match v0.clone() {
                        Adt0::C0(h5, _) => {
                            h5
                        }
                    }, List::<Bytes>::nil(), List::<Adt10>::nil(), List::<Adt11>::nil(), List::<Adt12>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil(), List::<Adt13>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil()), match v0.clone() {
                        Adt0::C0(_, h6) => {
                            h6
                        }
                    })?;
                    match m7 {
                        Ok(v4) => {
                            let m9 = f4(Fn0::F5(std::rc::Rc::new(v4)), None::<Bytes>, match v0 {
                                Adt0::C0(_, h8) => {
                                    h8
                                }
                            })?;
                            match m9 {
                                None => {
                                    Ok(Ok::<(), Bytes>(()))
                                }
                                Some(v5) => {
                                    Ok(Err::<(), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(), Bytes>(v3))
                        }
                    }
                }
                Some(v2) => {
                    Ok(Err::<(), Bytes>(v2))
                }
            }
        }
        Some(v1) => {
            Ok(Err::<(), Bytes>(v1))
        }
    }
}

pub fn f1(v0: List<Adt1>) -> R<Option<Bytes>> {
    let m11 = f6(Fn1::F7, Adt15::C0(false, None::<Bytes>), v0)?;
    match m11 {
        Adt15::C0(_, h10) => {
            Ok(h10)
        }
    }
}

pub fn f2(v0: List<Adt1>) -> R<Option<Bytes>> {
    let m12 = f4(Fn0::F8, None::<Bytes>, v0.clone())?;
    match m12 {
        None => {
            f9(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f3(v0: Adt14, v1: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    f10(Fn2::F11, Ok::<Adt14, Bytes>(v0), v1)
}

pub fn f4(v0: Fn0, p16: Option<Bytes>, p17: List<Adt1>) -> R<Option<Bytes>> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m13 = v2;
                match m13.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(({
                            let c14: Fn0 = v0.clone();
                            c14.apply(v1, v3)?
                        }, v4))
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

pub fn f5(v0: Adt14, v1: Option<Bytes>, v2: Adt1) -> R<Option<Bytes>> {
    let m18 = v1;
    match m18 {
        None => {
            f12(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f6(v0: Fn1, p22: Adt15, p23: List<Adt1>) -> R<Adt15> {
    let m21 = (p22, p23);
    {
        let mut m21 = m21;
        loop {
            let step = {
                let (v1, v2) = m21;
                let m19 = v2;
                match m19.uncons() {
                    None => {
                        Err::<(Adt15, List<Adt1>), R<Adt15>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt15, List<Adt1>), R<Adt15>>(({
                            let c20: Fn1 = v0.clone();
                            c20.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m21 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m25 = match v0.clone() {
        Adt15::C0(_, h24) => {
            h24
        }
    };
    match m25 {
        None => {
            f13(v0, v1)
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f8(v0: Option<Bytes>, v1: Adt1) -> Option<Bytes> {
    let m26 = v0;
    match m26 {
        None => {
            f14(v1)
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f9(v0: List<Adt1>) -> R<Option<Bytes>> {
    let v1: List<Adt13> = f15(v0.clone(), f16(Str::lit("unix")));
    let v2: List<Adt13> = f15(v0, f16(Str::lit("not_unix")));
    let m29 = f17(Fn3::F18(v2.clone(), v1.clone()), None::<Bytes>, {
        let a27 = v1;
        let a28 = v2;
        append_list(a27, a28)
    })?;
    match m29 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            Ok(Some(f19(f16(Str::lit("runtime support: cfg-pair: ")), f19(f20(v3), f16(Str::lit(" is not stated once under `unix` and once under `not_unix` with one arity"))))))
        }
    }
}

pub fn f10(v0: Fn2, p33: Result<Adt14, Bytes>, p34: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v2;
                match m30.uncons() {
                    None => {
                        Err::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(({
                            let c31: Fn2 = v0.clone();
                            c31.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Result<Adt14, Bytes>, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m35 = v0;
    match m35 {
        Ok(v3) => {
            f21(v3, v1)
        }
        Err(v2) => {
            Ok(Err::<Adt14, Bytes>(v2))
        }
    }
}

pub fn f12(v0: Adt14, v1: Adt1) -> R<Option<Bytes>> {
    let m36 = v1;
    match m36 {
        Adt1::C0(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C2(_, v7, r48) => {
            let v8 = (*r48).clone();
            f22(v0, v8, {
                let a37 = v7;
                length_list(a37)
            }, false)
        }
        Adt1::C3(_, r49, r50) => {
            let v10 = (*r49).clone();
            let v11 = (*r50).clone();
            let m38 = f22(v0.clone(), v10, 0u64, false)?;
            match m38 {
                None => {
                    f23(v0, f24(()), v11)
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt1::C4(v13, v14) => {
            let m40 = {
                let m39 = f25(v14.clone())?;
                match m39 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v15) => {
                        Some(f19(f16(Str::lit("runtime support: trait-method: ")), f19(f20(v13), f19(f16(Str::lit(" declares ")), f19(f20(v15), f16(Str::lit(" twice")))))))
                    }
                }
            };
            match m40 {
                None => {
                    f26(Fn4::F27(std::rc::Rc::new(v0)), None::<Bytes>, v14)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
        Adt1::C5(v17, v18, r51, v20) => {
            let v19 = (*r51).clone();
            let m41 = f28(v0.clone(), v17.clone())?;
            match m41 {
                None => {
                    let m43 = f22(v0.clone(), v19.clone(), {
                        let a42 = v17.clone();
                        length_list(a42)
                    }, false)?;
                    match m43 {
                        None => {
                            let m44 = v18;
                            match m44.uncons() {
                                None => {
                                    f30(Fn5::F31(v17, std::rc::Rc::new(v0), std::rc::Rc::new(v19)), None::<Bytes>, v20)
                                }
                                Some((v23, _)) => {
                                    f29(v0, v23, v17, v19, v20)
                                }
                            }
                        }
                        Some(v22) => {
                            Ok(Some(v22))
                        }
                    }
                }
                Some(v21) => {
                    Ok(Some(v21))
                }
            }
        }
        Adt1::C6(r52) => {
            let v25 = (*r52).clone();
            f32(v0, v25)
        }
        Adt1::C7(_, v27, _, r53) => {
            let v29 = (*r53).clone();
            f22(v0, v29, {
                let a45 = v27;
                length_list(a45)
            }, false)
        }
        Adt1::C8(_, v31, _, _, r54) => {
            let v34 = (*r54).clone();
            f22(v0, v34, {
                let a46 = v31;
                length_list(a46)
            }, false)
        }
        Adt1::C9(_, v36, _, v38) => {
            f33(Fn6::F34(v36, std::rc::Rc::new(v0)), None::<Bytes>, v38)
        }
        Adt1::C10(v39) => {
            f35(v39)
        }
        Adt1::C11(v40, v41) => {
            f4(Fn0::F36(v40, std::rc::Rc::new(v0)), None::<Bytes>, v41)
        }
        Adt1::C12(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C13(_, r55) => {
            let v45 = (*r55).clone();
            let m47 = f37(v45);
            match m47 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v46) => {
                    f32(v0, v46)
                }
            }
        }
    }
}

pub fn f13(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m56 = v1;
    match m56 {
        Adt1::C0(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C1(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C2(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C3(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C4(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C6(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C10(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C11(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C12(v37, v38) => {
            if match v0 {
                Adt15::C0(h57, _) => {
                    h57
                }
            } {
                Ok(Adt15::C0(true, Some(f19(f16(Str::lit("runtime support: attribute-position: `#![")), f19(v37, f16(Str::lit("]` follows an item")))))))
            } else {
                Ok(Adt15::C0(false, f38(v37, v38)?))
            }
        }
        Adt1::C13(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
    }
}

pub fn f14(v0: Adt1) -> Option<Bytes> {
    let m58 = f39(v0);
    match m58 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            if if f40({
                let (h59, _) = v1.clone();
                h59
            }, f16(Str::lit("unix"))) {
                true
            } else {
                f40({
                    let (h60, _) = v1.clone();
                    h60
                }, f16(Str::lit("not_unix")))
            } {
                let m62 = f37({
                    let (_, h61) = v1.clone();
                    h61
                });
                match m62 {
                    None => {
                        let m64 = f41({
                            let (_, h63) = v1;
                            h63
                        });
                        match m64 {
                            None => {
                                Some(f16(Str::lit("runtime support: cfg-item: a `cfg` item that is neither a `use` nor a function")))
                            }
                            Some(_) => {
                                None::<Bytes>
                            }
                        }
                    }
                    Some(_) => {
                        None::<Bytes>
                    }
                }
            } else {
                Some(f19(f16(Str::lit("runtime support: cfg-condition: ")), f19(f20({
                    let (h65, _) = v1;
                    h65
                }), f16(Str::lit(" is neither `unix` nor `not_unix`")))))
            }
        }
    }
}

pub fn f15(v0: List<Adt1>, v1: Bytes) -> List<Adt13> {
    f42(Fn7::F43(v1), List::<Adt13>::nil(), v0)
}

pub fn f16(v0: Str) -> Bytes {
    let a66 = v0;
    utf8_encode(a66)
}

pub fn f17(v0: Fn3, p70: Option<Bytes>, p71: List<Adt13>) -> R<Option<Bytes>> {
    let m69 = (p70, p71);
    {
        let mut m69 = m69;
        loop {
            let step = {
                let (v1, v2) = m69;
                let m67 = v2;
                match m67.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt13>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt13>), R<Option<Bytes>>>(({
                            let c68: Fn3 = v0.clone();
                            c68.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m69 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Adt13>, v1: List<Adt13>, v2: Option<Bytes>, v3: Adt13) -> R<Option<Bytes>> {
    if f44(v1, v0, match v3.clone() {
        Adt13::C0(h72, _) => {
            h72
        }
    })? {
        Ok(v2)
    } else {
        let m73 = v2;
        match m73 {
            None => {
                Ok(Some(match v3 {
                    Adt13::C0(h76, _) => {
                        h76
                    }
                }))
            }
            Some(v4) => {
                if f45(match v3.clone() {
                    Adt13::C0(h74, _) => {
                        h74
                    }
                }, v4.clone()) {
                    Ok(Some(match v3 {
                        Adt13::C0(h75, _) => {
                            h75
                        }
                    }))
                } else {
                    Ok(Some(v4))
                }
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    let a77 = v0;
    let a78 = v1;
    append_bytes(a77, a78)
}

pub fn f20(v0: Bytes) -> Bytes {
    f19(f16(Str::lit("`")), f19(v0, f16(Str::lit("`"))))
}

pub fn f21(v0: Adt14, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m79 = v1;
    match m79 {
        Adt1::C0(v2, v3) => {
            f46(v0, v2, v3)
        }
        Adt1::C1(v4, v5) => {
            Ok(f47(f48(Str::lit("a unit struct name"), v4.clone())?, f49({
                let m80 = f50(v0, v4.clone())?;
                match m80 {
                    Ok(v7) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v7.clone() {
                            Adt14::C0(h81, _, _, _, _, _, _, _, _, _) => {
                                h81
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, h82, _, _, _, _, _, _, _, _) => {
                                h82
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, h83, _, _, _, _, _, _, _) => {
                                h83
                            }
                        }, {
                            let a85 = match v7.clone() {
                                Adt14::C0(_, _, _, h84, _, _, _, _, _, _) => {
                                    h84
                                }
                            };
                            let a86 = List::cons(Adt11::C0(v4, List::<Bytes>::nil(), 0u64), List::<Adt11>::nil());
                            append_list(a85, a86)
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, h87, _, _, _, _, _) => {
                                h87
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, h88, _, _, _, _) => {
                                h88
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, h89, _, _, _) => {
                                h89
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h90, _, _) => {
                                h90
                            }
                        }, match v7.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h91, _) => {
                                h91
                            }
                        }, match v7 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h92) => {
                                h92
                            }
                        }))
                    }
                    Err(v6) => {
                        Err::<Adt14, Bytes>(v6)
                    }
                }
            }, f51(Str::lit("a unit struct"), v5)?)))
        }
        Adt1::C2(v8, v9, _) => {
            Ok(f47(f48(Str::lit("an alias name"), v8.clone())?, f49({
                let m93 = f50(v0, v8.clone())?;
                match m93 {
                    Ok(v12) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v12.clone() {
                            Adt14::C0(h94, _, _, _, _, _, _, _, _, _) => {
                                h94
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, h95, _, _, _, _, _, _, _, _) => {
                                h95
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, h96, _, _, _, _, _, _, _) => {
                                h96
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, h97, _, _, _, _, _, _) => {
                                h97
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, h98, _, _, _, _, _) => {
                                h98
                            }
                        }, {
                            let a101 = match v12.clone() {
                                Adt14::C0(_, _, _, _, _, h99, _, _, _, _) => {
                                    h99
                                }
                            };
                            let a102 = List::cons(Adt13::C0(v8, {
                                let a100 = v9.clone();
                                length_list(a100)
                            }), List::<Adt13>::nil());
                            append_list(a101, a102)
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, h103, _, _, _) => {
                                h103
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h104, _, _) => {
                                h104
                            }
                        }, match v12.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h105, _) => {
                                h105
                            }
                        }, match v12 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h106) => {
                                h106
                            }
                        }))
                    }
                    Err(v11) => {
                        Err::<Adt14, Bytes>(v11)
                    }
                }
            }, f52(Str::lit("an alias parameter"), v9)?)))
        }
        Adt1::C3(v13, _, _) => {
            Ok(f47(f48(Str::lit("a static name"), v13.clone())?, {
                let m107 = f50(v0, v13.clone())?;
                match m107 {
                    Ok(v17) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v17.clone() {
                            Adt14::C0(h108, _, _, _, _, _, _, _, _, _) => {
                                h108
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, h109, _, _, _, _, _, _, _, _) => {
                                h109
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, h110, _, _, _, _, _, _, _) => {
                                h110
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, h111, _, _, _, _, _, _) => {
                                h111
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, h112, _, _, _, _, _) => {
                                h112
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, h113, _, _, _, _) => {
                                h113
                            }
                        }, {
                            let a115 = match v17.clone() {
                                Adt14::C0(_, _, _, _, _, _, h114, _, _, _) => {
                                    h114
                                }
                            };
                            let a116 = List::cons(v13, List::<Bytes>::nil());
                            append_list(a115, a116)
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h117, _, _) => {
                                h117
                            }
                        }, match v17.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h118, _) => {
                                h118
                            }
                        }, match v17 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h119) => {
                                h119
                            }
                        }))
                    }
                    Err(v16) => {
                        Err::<Adt14, Bytes>(v16)
                    }
                }
            }))
        }
        Adt1::C4(v18, v19) => {
            Ok(f47(f48(Str::lit("a trait name"), v18.clone())?, {
                let m120 = f50(v0, v18.clone())?;
                match m120 {
                    Ok(v21) => {
                        Ok::<Adt14, Bytes>(f53(Adt14::C0(match v21.clone() {
                            Adt14::C0(h121, _, _, _, _, _, _, _, _, _) => {
                                h121
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, h122, _, _, _, _, _, _, _, _) => {
                                h122
                            }
                        }, {
                            let a124 = match v21.clone() {
                                Adt14::C0(_, _, h123, _, _, _, _, _, _, _) => {
                                    h123
                                }
                            };
                            let a125 = List::cons(Adt10::C0(v18, v19.clone()), List::<Adt10>::nil());
                            append_list(a124, a125)
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, h126, _, _, _, _, _, _) => {
                                h126
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, h127, _, _, _, _, _) => {
                                h127
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, h128, _, _, _, _) => {
                                h128
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, h129, _, _, _) => {
                                h129
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h130, _, _) => {
                                h130
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h131, _) => {
                                h131
                            }
                        }, match v21 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h132) => {
                                h132
                            }
                        }), v19))
                    }
                    Err(v20) => {
                        Err::<Adt14, Bytes>(v20)
                    }
                }
            }))
        }
        Adt1::C5(_, _, r230, v25) => {
            let v24 = (*r230).clone();
            Ok(Ok::<Adt14, Bytes>(f54(f55(v0, v25.clone()), v24, v25)))
        }
        Adt1::C6(r231) => {
            let v26 = (*r231).clone();
            Ok(f47(f48(Str::lit("a function name"), {
                let m135 = match v26.clone() {
                    Adt8::C0(h134, _, _, _) => {
                        (*h134).clone()
                    }
                };
                match m135 {
                    Adt5::C0(h133, _, _, _, _) => {
                        h133
                    }
                }
            })?, {
                let m139 = f50(v0, {
                    let m138 = match v26.clone() {
                        Adt8::C0(h137, _, _, _) => {
                            (*h137).clone()
                        }
                    };
                    match m138 {
                        Adt5::C0(h136, _, _, _, _) => {
                            h136
                        }
                    }
                })?;
                match m139 {
                    Ok(v28) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v28.clone() {
                            Adt14::C0(h140, _, _, _, _, _, _, _, _, _) => {
                                h140
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, h141, _, _, _, _, _, _, _, _) => {
                                h141
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, h142, _, _, _, _, _, _, _) => {
                                h142
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, h143, _, _, _, _, _, _) => {
                                h143
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, h144, _, _, _, _, _) => {
                                h144
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, _, h145, _, _, _, _) => {
                                h145
                            }
                        }, match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, h146, _, _, _) => {
                                h146
                            }
                        }, List::cons(Adt13::C0({
                            let m149 = match v26.clone() {
                                Adt8::C0(h148, _, _, _) => {
                                    (*h148).clone()
                                }
                            };
                            match m149 {
                                Adt5::C0(h147, _, _, _, _) => {
                                    h147
                                }
                            }
                        }, {
                            let a153 = {
                                let m152 = match v26 {
                                    Adt8::C0(h151, _, _, _) => {
                                        (*h151).clone()
                                    }
                                };
                                match m152 {
                                    Adt5::C0(_, _, _, h150, _) => {
                                        h150
                                    }
                                }
                            };
                            length_list(a153)
                        }), match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h154, _, _) => {
                                h154
                            }
                        }), match v28.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h155, _) => {
                                h155
                            }
                        }, match v28 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h156) => {
                                h156
                            }
                        }))
                    }
                    Err(v27) => {
                        Err::<Adt14, Bytes>(v27)
                    }
                }
            }))
        }
        Adt1::C7(v29, v30, v31, r232) => {
            let v32 = (*r232).clone();
            Ok(f47(f48(Str::lit("a tuple struct name"), v29.clone())?, f49({
                let m157 = f50(v0, v29.clone())?;
                match m157 {
                    Ok(v34) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v34.clone() {
                            Adt14::C0(h158, _, _, _, _, _, _, _, _, _) => {
                                h158
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, h159, _, _, _, _, _, _, _, _) => {
                                h159
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, h160, _, _, _, _, _, _, _) => {
                                h160
                            }
                        }, {
                            let a163 = match v34.clone() {
                                Adt14::C0(_, _, _, h161, _, _, _, _, _, _) => {
                                    h161
                                }
                            };
                            let a164 = List::cons(Adt11::C0(v29, f56(v32, 0u64)?, {
                                let a162 = v30.clone();
                                length_list(a162)
                            }), List::<Adt11>::nil());
                            append_list(a163, a164)
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, h165, _, _, _, _, _) => {
                                h165
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, h166, _, _, _, _) => {
                                h166
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, h167, _, _, _) => {
                                h167
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h168, _, _) => {
                                h168
                            }
                        }, match v34.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h169, _) => {
                                h169
                            }
                        }, match v34 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h170) => {
                                h170
                            }
                        }))
                    }
                    Err(v33) => {
                        Err::<Adt14, Bytes>(v33)
                    }
                }
            }, f57(f52(Str::lit("a tuple struct generic"), v30)?, f51(Str::lit("a tuple struct"), v31)?))))
        }
        Adt1::C8(v35, v36, v37, v38, r233) => {
            let v39 = (*r233).clone();
            Ok(f47(f48(Str::lit("a record struct name"), v35.clone())?, {
                let m171 = f50(v0, v35.clone())?;
                match m171 {
                    Ok(v41) => {
                        let m172 = f57(f52(Str::lit("a record struct generic"), v36.clone())?, f57(f51(Str::lit("a record struct"), v37)?, f52(Str::lit("a record field name"), v38.clone())?));
                        match m172 {
                            None => {
                                let a174 = {
                                    let a173 = v38.clone();
                                    length_list(a173)
                                };
                                let a175 = f58(v39.clone())?;
                                if nat_eq(a174, a175) {
                                    Ok::<Adt14, Bytes>(Adt14::C0(match v41.clone() {
                                        Adt14::C0(h176, _, _, _, _, _, _, _, _, _) => {
                                            h176
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, h177, _, _, _, _, _, _, _, _) => {
                                            h177
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, h178, _, _, _, _, _, _, _) => {
                                            h178
                                        }
                                    }, {
                                        let a181 = match v41.clone() {
                                            Adt14::C0(_, _, _, h179, _, _, _, _, _, _) => {
                                                h179
                                            }
                                        };
                                        let a182 = List::cons(Adt11::C0(v35, v38, {
                                            let a180 = v36;
                                            length_list(a180)
                                        }), List::<Adt11>::nil());
                                        append_list(a181, a182)
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, h183, _, _, _, _, _) => {
                                            h183
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, h184, _, _, _, _) => {
                                            h184
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, h185, _, _, _) => {
                                            h185
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, h186, _, _) => {
                                            h186
                                        }
                                    }, match v41.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, _, h187, _) => {
                                            h187
                                        }
                                    }, match v41 {
                                        Adt14::C0(_, _, _, _, _, _, _, _, _, h188) => {
                                            h188
                                        }
                                    }))
                                } else {
                                    Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(v35), f19(f16(Str::lit(" has ")), f19(f59({
                                        let a189 = v38;
                                        length_list(a189)
                                    })?, f19(f16(Str::lit(" field names and ")), f19(f59(f58(v39)?)?, f16(Str::lit(" types")))))))))
                                }
                            }
                            Some(v42) => {
                                Err::<Adt14, Bytes>(v42)
                            }
                        }
                    }
                    Err(v40) => {
                        Err::<Adt14, Bytes>(v40)
                    }
                }
            }))
        }
        Adt1::C9(v52, v53, v54, v55) => {
            Ok(f47(f48(Str::lit("an enum name"), v52.clone())?, {
                let m214 = f50(v0, v52.clone())?;
                match m214 {
                    Ok(v57) => {
                        let m215 = f57(f52(Str::lit("an enum generic"), v53.clone())?, f57(f51(Str::lit("an enum"), v54)?, f60(v52.clone(), v55.clone())?));
                        match m215 {
                            None => {
                                let m216 = f61(v55.clone())?;
                                match m216 {
                                    None => {
                                        Ok::<Adt14, Bytes>(Adt14::C0(match v57.clone() {
                                            Adt14::C0(h217, _, _, _, _, _, _, _, _, _) => {
                                                h217
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, h218, _, _, _, _, _, _, _, _) => {
                                                h218
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, h219, _, _, _, _, _, _, _) => {
                                                h219
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, h220, _, _, _, _, _, _) => {
                                                h220
                                            }
                                        }, {
                                            let a223 = match v57.clone() {
                                                Adt14::C0(_, _, _, _, h221, _, _, _, _, _) => {
                                                    h221
                                                }
                                            };
                                            let a224 = List::cons(Adt12::C0(v52, v55, {
                                                let a222 = v53;
                                                length_list(a222)
                                            }), List::<Adt12>::nil());
                                            append_list(a223, a224)
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, h225, _, _, _, _) => {
                                                h225
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, h226, _, _, _) => {
                                                h226
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, h227, _, _) => {
                                                h227
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, _, h228, _) => {
                                                h228
                                            }
                                        }, match v57 {
                                            Adt14::C0(_, _, _, _, _, _, _, _, _, h229) => {
                                                h229
                                            }
                                        }))
                                    }
                                    Some(v59) => {
                                        Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(v52), f19(f16(Str::lit(" declares ")), f19(f20(v59), f16(Str::lit(" twice")))))))
                                    }
                                }
                            }
                            Some(v58) => {
                                Err::<Adt14, Bytes>(v58)
                            }
                        }
                    }
                    Err(v56) => {
                        Err::<Adt14, Bytes>(v56)
                    }
                }
            }))
        }
        Adt1::C10(_) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
        Adt1::C11(v44, _) => {
            Ok(f47(f48(Str::lit("a module name"), v44.clone())?, f50(v0, v44)?))
        }
        Adt1::C12(_, _) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
        Adt1::C13(_, r234) => {
            let v49 = (*r234).clone();
            let m190 = f37(v49.clone());
            match m190 {
                None => {
                    let m211 = f41(v49);
                    match m211 {
                        None => {
                            Ok(Ok::<Adt14, Bytes>(v0))
                        }
                        Some(v51) => {
                            f46(v0, {
                                let (h212, _) = v51;
                                h212
                            }, {
                                let (_, h213) = v51;
                                h213
                            })
                        }
                    }
                }
                Some(v50) => {
                    Ok(f47(f48(Str::lit("a function name"), {
                        let m193 = match v50.clone() {
                            Adt8::C0(h192, _, _, _) => {
                                (*h192).clone()
                            }
                        };
                        match m193 {
                            Adt5::C0(h191, _, _, _, _) => {
                                h191
                            }
                        }
                    })?, Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                        Adt14::C0(h194, _, _, _, _, _, _, _, _, _) => {
                            h194
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, h195, _, _, _, _, _, _, _, _) => {
                            h195
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, h196, _, _, _, _, _, _, _) => {
                            h196
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, h197, _, _, _, _, _, _) => {
                            h197
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, h198, _, _, _, _, _) => {
                            h198
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h199, _, _, _, _) => {
                            h199
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, h200, _, _, _) => {
                            h200
                        }
                    }, List::cons(Adt13::C0({
                        let m203 = match v50.clone() {
                            Adt8::C0(h202, _, _, _) => {
                                (*h202).clone()
                            }
                        };
                        match m203 {
                            Adt5::C0(h201, _, _, _, _) => {
                                h201
                            }
                        }
                    }, {
                        let a207 = {
                            let m206 = match v50 {
                                Adt8::C0(h205, _, _, _) => {
                                    (*h205).clone()
                                }
                            };
                            match m206 {
                                Adt5::C0(_, _, _, h204, _) => {
                                    h204
                                }
                            }
                        };
                        length_list(a207)
                    }), match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, h208, _, _) => {
                            h208
                        }
                    }), match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, h209, _) => {
                            h209
                        }
                    }, match v0 {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, h210) => {
                            h210
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f22(v0: Adt14, v1: Adt2, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m235 = v1;
    match m235 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            if v3 {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: self-type: ")), f16(Str::lit("`Self` outside a trait or an impl")))))
            }
        }
        Adt2::C6(v4) => {
            let a236 = v4;
            let a237 = v2;
            if nat_lt(a236, a237) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: type-parameter: ")), f19(f16(Str::lit("parameter ")), f19(f59(v4)?, f19(f16(Str::lit(" of ")), f19(f59(v2)?, f16(Str::lit(" in scope")))))))))
            }
        }
        Adt2::C7(r245) => {
            let v5 = (*r245).clone();
            f22(v0, v5, v2, v3)
        }
        Adt2::C8(r246) => {
            let v6 = (*r246).clone();
            f22(v0, v6, v2, v3)
        }
        Adt2::C9(r247, r248) => {
            let v7 = (*r247).clone();
            let v8 = (*r248).clone();
            let m238 = f22(v0.clone(), v7, v2, v3)?;
            match m238 {
                None => {
                    f22(v0, v8, v2, v3)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt2::C10(v10, r249) => {
            let v11 = (*r249).clone();
            let m239 = f62(v10.clone())?;
            match m239 {
                None => {
                    let m240 = f63(v0.clone(), v10.clone())?;
                    match m240 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: type-name: ")), f19(f20(v10), f16(Str::lit(" is neither declared nor in the closed table"))))))
                        }
                        Some(v13) => {
                            if if match v0.clone() {
                                Adt14::C0(h241, _, _, _, _, _, _, _, _, _) => {
                                    h241
                                }
                            } {
                                f64(v0.clone(), v10.clone())
                            } else {
                                false
                            } {
                                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v10), f16(Str::lit(", which requires heap allocation")))))))
                            } else {
                                let a242 = v13;
                                let a243 = f58(v11.clone())?;
                                if nat_eq(a242, a243) {
                                    f22(v0, v11, v2, v3)
                                } else {
                                    Ok(Some(f19(f16(Str::lit("runtime support: type-arity: ")), f19(f20(v10), f19(f16(Str::lit(" takes ")), f19(f59(v13)?, f19(f16(Str::lit(" arguments, given ")), f59(f58(v11)?)?)))))))
                                }
                            }
                        }
                    }
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(r250) => {
            let v14 = (*r250).clone();
            f22(v0, v14, v2, v3)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(r251, r252) => {
            let v15 = (*r251).clone();
            let v16 = (*r252).clone();
            let m244 = f22(v0.clone(), v15, v2, v3)?;
            match m244 {
                None => {
                    f22(v0, v16, v2, v3)
                }
                Some(v17) => {
                    Ok(Some(v17))
                }
            }
        }
    }
}

pub fn f23(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m253 = v2;
    match m253 {
        Adt3::C0(v3) => {
            let m255 = f65(match v1 {
                Adt16::C0(_, h254, _, _) => {
                    h254
                }
            }, v3.clone());
            match m255 {
                None => {
                    if f66(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h256, _, _, _) => {
                            h256
                        }
                    }, v3.clone())? {
                        Ok(None::<Bytes>)
                    } else {
                        Ok(Some(f19(f16(Str::lit("runtime support: variable-scope: ")), f19(f20(v3), f16(Str::lit(" is not in scope"))))))
                    }
                }
                Some(_) => {
                    Ok(None::<Bytes>)
                }
            }
        }
        Adt3::C1(v5) => {
            f67(v5)
        }
        Adt3::C2(v6) => {
            f68(v0, v6)
        }
        Adt3::C3(v7, r301) => {
            let v8 = (*r301).clone();
            let m257 = f69(v0.clone(), v7, f70(v8.clone())?)?;
            match m257 {
                None => {
                    f71(v0, v1, v8)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt3::C4(r302, v11, r303) => {
            let v10 = (*r302).clone();
            let v12 = (*r303).clone();
            let m258 = f23(v0.clone(), v1.clone(), v10.clone())?;
            match m258 {
                None => {
                    let m259 = f72(v0.clone(), v1.clone(), v10, v11, f70(v12.clone())?)?;
                    match m259 {
                        None => {
                            f71(v0, v1, v12)
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt3::C5(r304, _) => {
            let v15 = (*r304).clone();
            f23(v0, v1, v15)
        }
        Adt3::C6(r305, v18) => {
            let v17 = (*r305).clone();
            let m260 = f48(Str::lit("a field access"), v18)?;
            match m260 {
                None => {
                    f23(v0, v1, v17)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
        Adt3::C7(r306) => {
            let v20 = (*r306).clone();
            f23(v0, v1, v20)
        }
        Adt3::C8(r307, r308) => {
            let v21 = (*r307).clone();
            let v22 = (*r308).clone();
            let m261 = f23(v0.clone(), v1.clone(), v21)?;
            match m261 {
                None => {
                    f23(v0, v1, v22)
                }
                Some(v23) => {
                    Ok(Some(v23))
                }
            }
        }
        Adt3::C9(r309, r310) => {
            let v24 = (*r309).clone();
            let v25 = (*r310).clone();
            let m262 = f57(f73(v24.clone())?, f23(v0.clone(), v1.clone(), v24)?);
            match m262 {
                None => {
                    if f74(v25.clone()) {
                        Ok(Some(f19(f16(Str::lit("runtime support: match-arms: ")), f16(Str::lit("a match with no arm")))))
                    } else {
                        f75(v0, v1, v25)
                    }
                }
                Some(v26) => {
                    Ok(Some(v26))
                }
            }
        }
        Adt3::C10(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: let-position: ")), f16(Str::lit("a `let` that is not a statement of a block")))))
        }
        Adt3::C11(r311, r312) => {
            let v30 = (*r311).clone();
            let v31 = (*r312).clone();
            let m263 = f76(v1.clone(), v30.clone());
            match m263 {
                None => {
                    let m264 = f23(v0.clone(), v1.clone(), v30)?;
                    match m264 {
                        None => {
                            f23(v0, v1, v31)
                        }
                        Some(v33) => {
                            Ok(Some(v33))
                        }
                    }
                }
                Some(v32) => {
                    Ok(Some(v32))
                }
            }
        }
        Adt3::C12(r313, r314, r315) => {
            let v34 = (*r313).clone();
            let v35 = (*r314).clone();
            let v36 = (*r315).clone();
            let m265 = f57(f73(v34.clone())?, f23(v0.clone(), v1.clone(), v34)?);
            match m265 {
                None => {
                    let m266 = f77(v0.clone(), v1.clone(), v35)?;
                    match m266 {
                        None => {
                            f77(v0, v1, v36)
                        }
                        Some(v38) => {
                            Ok(Some(v38))
                        }
                    }
                }
                Some(v37) => {
                    Ok(Some(v37))
                }
            }
        }
        Adt3::C13(r316, r317) => {
            let v39 = (*r316).clone();
            let v40 = (*r317).clone();
            let m267 = f57(f73(v39.clone())?, f23(v0.clone(), v1.clone(), v39)?);
            match m267 {
                None => {
                    f77(v0, f78(v1, List::<Adt17>::nil(), true), v40)
                }
                Some(v41) => {
                    Ok(Some(v41))
                }
            }
        }
        Adt3::C14(r318, r319, r320) => {
            let v42 = (*r318).clone();
            let v43 = (*r319).clone();
            let v44 = (*r320).clone();
            let m268 = f57(f73(v43.clone())?, f23(v0.clone(), v1.clone(), v43)?);
            match m268 {
                None => {
                    let m269 = f79(v0.clone(), v42.clone())?;
                    match m269 {
                        None => {
                            let m270 = f80(List::<Adt17>::nil(), v42)?;
                            match m270 {
                                Ok(v48) => {
                                    f77(v0, f78(v1, v48, true), v44)
                                }
                                Err(v47) => {
                                    Ok(Some(v47))
                                }
                            }
                        }
                        Some(v46) => {
                            Ok(Some(v46))
                        }
                    }
                }
                Some(v45) => {
                    Ok(Some(v45))
                }
            }
        }
        Adt3::C15(r321) => {
            let v49 = (*r321).clone();
            f77(v0, f78(v1, List::<Adt17>::nil(), true), v49)
        }
        Adt3::C16(r322, r323, r324) => {
            let v50 = (*r322).clone();
            let v51 = (*r323).clone();
            let v52 = (*r324).clone();
            let m271 = f57(f73(v51.clone())?, f23(v0.clone(), v1.clone(), v51)?);
            match m271 {
                None => {
                    let m272 = f79(v0.clone(), v50.clone())?;
                    match m272 {
                        None => {
                            let m273 = f80(List::<Adt17>::nil(), v50)?;
                            match m273 {
                                Ok(v56) => {
                                    f77(v0, f78(v1, v56, true), v52)
                                }
                                Err(v55) => {
                                    Ok(Some(v55))
                                }
                            }
                        }
                        Some(v54) => {
                            Ok(Some(v54))
                        }
                    }
                }
                Some(v53) => {
                    Ok(Some(v53))
                }
            }
        }
        Adt3::C17 => {
            if match v1 {
                Adt16::C0(_, _, _, h274) => {
                    h274
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: break-position: ")), f16(Str::lit("a `break` outside a loop")))))
            }
        }
        Adt3::C18(r325) => {
            let v57 = (*r325).clone();
            if f81(v57.clone()) {
                Ok(None::<Bytes>)
            } else {
                f23(v0, v1, v57)
            }
        }
        Adt3::C19(r326) => {
            let v58 = (*r326).clone();
            let a275 = f70(v58.clone())?;
            let a276 = 2u64;
            if nat_lt(a275, a276) {
                Ok(Some(f19(f16(Str::lit("runtime support: tuple-arity: ")), f19(f16(Str::lit("a tuple of ")), f59(f70(v58)?)?))))
            } else {
                f71(v0, v1, v58)
            }
        }
        Adt3::C20(v59, v60, r327) => {
            let v61 = (*r327).clone();
            let m277 = f68(v0.clone(), v59.clone())?;
            match m277 {
                None => {
                    let m278 = f82(v0.clone(), v59.clone());
                    match m278 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(f83(v59, Str::lit("::"))?), f16(Str::lit(" is no record struct or record variant"))))))
                        }
                        Some(v63) => {
                            if f84(v60.clone(), v63.clone(), f70(v61.clone())?)? {
                                f71(v0, v1, v61)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: struct-fields: ")), f19(f20(f83(v59, Str::lit("::"))?), f19(f16(Str::lit(" is given ")), f19(f83(v60, Str::lit(", "))?, f19(f16(Str::lit(", not ")), f83(v63, Str::lit(", "))?)))))))
                            }
                        }
                    }
                }
                Some(v62) => {
                    Ok(Some(v62))
                }
            }
        }
        Adt3::C21(_, r328, r329) => {
            let v65 = (*r328).clone();
            let v66 = (*r329).clone();
            let m279 = f23(v0.clone(), v1.clone(), v65)?;
            match m279 {
                None => {
                    f23(v0, v1, v66)
                }
                Some(v67) => {
                    Ok(Some(v67))
                }
            }
        }
        Adt3::C22(r330) => {
            let v68 = (*r330).clone();
            f23(v0, v1, v68)
        }
        Adt3::C23(r331, r332) => {
            let v69 = (*r331).clone();
            let v70 = (*r332).clone();
            let m284 = f57(f22(v0.clone(), v70.clone(), {
                let a281 = match v1.clone() {
                    Adt16::C0(h280, _, _, _) => {
                        h280
                    }
                };
                length_list(a281)
            }, {
                let a283 = f85(match v1.clone() {
                    Adt16::C0(_, _, h282, _) => {
                        (*h282).clone()
                    }
                });
                bool_not(a283)
            })?, f86(Str::lit("a cast type"), v70));
            match m284 {
                None => {
                    f23(v0, v1, v69)
                }
                Some(v71) => {
                    Ok(Some(v71))
                }
            }
        }
        Adt3::C24(r333, r334) => {
            let v72 = (*r333).clone();
            let v73 = (*r334).clone();
            let m285 = f23(v0.clone(), v1.clone(), v72)?;
            match m285 {
                None => {
                    f23(v0, v1, v73)
                }
                Some(v74) => {
                    Ok(Some(v74))
                }
            }
        }
        Adt3::C25(r335, r336, r337) => {
            let v75 = (*r335).clone();
            let v76 = (*r336).clone();
            let v77 = (*r337).clone();
            let m286 = f23(v0.clone(), v1.clone(), v75)?;
            match m286 {
                None => {
                    let m287 = if f81(v76.clone()) {
                        None::<Bytes>
                    } else {
                        f23(v0.clone(), v1.clone(), v76)?
                    };
                    match m287 {
                        None => {
                            if f81(v77.clone()) {
                                Ok(None::<Bytes>)
                            } else {
                                f23(v0, v1, v77)
                            }
                        }
                        Some(v79) => {
                            Ok(Some(v79))
                        }
                    }
                }
                Some(v78) => {
                    Ok(Some(v78))
                }
            }
        }
        Adt3::C26(r338) => {
            let v80 = (*r338).clone();
            f23(v0, v1, v80)
        }
        Adt3::C27(r339) => {
            let v81 = (*r339).clone();
            f23(v0, v1, v81)
        }
        Adt3::C28(r340, v83, r341, r342) => {
            let v82 = (*r340).clone();
            let v84 = (*r341).clone();
            let v85 = (*r342).clone();
            let m288 = f23(v0.clone(), v1.clone(), v82)?;
            match m288 {
                None => {
                    let m289 = f87(v83.clone());
                    match m289 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: method-name: ")), f19(f20(v83), f16(Str::lit(" takes no type arguments in the closed table"))))))
                        }
                        Some(v87) => {
                            let a290 = v87;
                            let a291 = f58(v84.clone())?;
                            if if nat_eq(a290, a291) {
                                let a292 = f88(v83.clone());
                                let a293 = f70(v85.clone())?;
                                nat_eq(a292, a293)
                            } else {
                                false
                            } {
                                if if match v0.clone() {
                                    Adt14::C0(h294, _, _, _, _, _, _, _, _, _) => {
                                        h294
                                    }
                                } {
                                    f40(v83.clone(), f16(Str::lit("collect")))
                                } else {
                                    false
                                } {
                                    Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v83), f16(Str::lit(", which requires heap allocation")))))))
                                } else {
                                    let m299 = f57(f22(v0.clone(), v84.clone(), {
                                        let a296 = match v1.clone() {
                                            Adt16::C0(h295, _, _, _) => {
                                                h295
                                            }
                                        };
                                        length_list(a296)
                                    }, {
                                        let a298 = f85(match v1.clone() {
                                            Adt16::C0(_, _, h297, _) => {
                                                (*h297).clone()
                                            }
                                        });
                                        bool_not(a298)
                                    })?, f86(Str::lit("a type argument"), v84));
                                    match m299 {
                                        None => {
                                            f71(v0, v1, v85)
                                        }
                                        Some(v88) => {
                                            Ok(Some(v88))
                                        }
                                    }
                                }
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v83.clone()), f19(f16(Str::lit(" takes ")), f19(f59(v87)?, f19(f16(Str::lit(" type arguments and ")), f19(f59(f88(v83))?, f16(Str::lit(" arguments"))))))))))
                            }
                        }
                    }
                }
                Some(v86) => {
                    Ok(Some(v86))
                }
            }
        }
        Adt3::C29(r343, r344) => {
            let v89 = (*r343).clone();
            let v90 = (*r344).clone();
            let m300 = f89(v0.clone(), v1, v89)?;
            match m300 {
                Ok(v92) => {
                    f23(v0, v92, v90)
                }
                Err(v91) => {
                    Ok(Some(v91))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f24(_: ()) -> Adt16 {
    Adt16::C0(List::<Adt7>::nil(), List::<Adt17>::nil(), std::rc::Rc::new(Adt2::C17), false)
}

pub fn f25(v0: List<Adt5>) -> R<Option<Bytes>> {
    let m346 = f90(Fn8::F91, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m346 {
        Adt18::C0(_, h345) => {
            Ok(h345)
        }
    }
}

pub fn f26(v0: Fn4, p350: Option<Bytes>, p351: List<Adt5>) -> R<Option<Bytes>> {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v1, v2) = m349;
                let m347 = v2;
                match m347.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(({
                            let c348: Fn4 = v0.clone();
                            c348.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt14, v1: Option<Bytes>, v2: Adt5) -> R<Option<Bytes>> {
    let m352 = v1;
    match m352 {
        None => {
            f92(v0, v2, 0u64, true)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f28(v0: Adt14, v1: List<Adt7>) -> R<Option<Bytes>> {
    f93(Fn9::F94(std::rc::Rc::new(v0)), None::<Bytes>, v1)
}

pub fn f29(v0: Adt14, v1: Bytes, v2: List<Adt7>, v3: Adt2, v4: List<Adt8>) -> R<Option<Bytes>> {
    let m353 = f28(v0.clone(), v2.clone())?;
    match m353 {
        None => {
            let m354 = f95(v0.clone(), v1.clone());
            match m354 {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-trait: ")), f19(f16(Str::lit("`impl ")), f19(v1, f16(Str::lit("` names no declared trait")))))))
                }
                Some(v6) => {
                    let m355 = f26(Fn4::F96(v4.clone(), v1.clone()), None::<Bytes>, v6.clone())?;
                    match m355 {
                        None => {
                            f30(Fn5::F97(v2, std::rc::Rc::new(v0), v6, v1, std::rc::Rc::new(v3)), None::<Bytes>, v4)
                        }
                        Some(v7) => {
                            Ok(Some(v7))
                        }
                    }
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f30(v0: Fn5, p359: Option<Bytes>, p360: List<Adt8>) -> R<Option<Bytes>> {
    let m358 = (p359, p360);
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v1, v2) = m358;
                let m356 = v2;
                match m356.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(({
                            let c357: Fn5 = v0.clone();
                            c357.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: List<Adt7>, v1: Adt14, v2: Adt2, v3: Option<Bytes>, v4: Adt8) -> R<Option<Bytes>> {
    let m361 = v3;
    match m361 {
        None => {
            f98(v1, v0, v2, v4)
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f32(v0: Adt14, v1: Adt8) -> R<Option<Bytes>> {
    let a363 = match v1.clone() {
        Adt8::C0(_, _, h362, _) => {
            h362
        }
    };
    if bool_not(a363) {
        Ok(Some(f19(f16(Str::lit("runtime support: function-visibility: ")), f19(f20({
            let m366 = match v1 {
                Adt8::C0(h365, _, _, _) => {
                    (*h365).clone()
                }
            };
            match m366 {
                Adt5::C0(h364, _, _, _, _) => {
                    h364
                }
            }
        }), f16(Str::lit(" is a free function that is not public"))))))
    } else {
        let m369 = match v1.clone() {
            Adt8::C0(h368, _, _, _) => {
                (*h368).clone()
            }
        };
        if match m369 {
            Adt5::C0(_, h367, _, _, _) => {
                h367
            }
        } {
            Ok(Some(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f16(Str::lit("free function ")), f19(f20({
                let m372 = match v1 {
                    Adt8::C0(h371, _, _, _) => {
                        (*h371).clone()
                    }
                };
                match m372 {
                    Adt5::C0(h370, _, _, _, _) => {
                        h370
                    }
                }
            }), f16(Str::lit(" takes `&self`")))))))
        } else {
            f98(v0, List::<Adt7>::nil(), Adt2::C17, v1)
        }
    }
}

pub fn f33(v0: Fn6, p376: Option<Bytes>, p377: List<Adt9>) -> R<Option<Bytes>> {
    let m375 = (p376, p377);
    {
        let mut m375 = m375;
        loop {
            let step = {
                let (v1, v2) = m375;
                let m373 = v2;
                match m373.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(({
                            let c374: Fn6 = v0.clone();
                            c374.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<Bytes>, v1: Adt14, v2: Option<Bytes>, v3: Adt9) -> R<Option<Bytes>> {
    let m378 = v2;
    match m378 {
        None => {
            let m379 = v3;
            match m379 {
                Adt9::C0(_) => {
                    Ok(None::<Bytes>)
                }
                Adt9::C1(_, r382) => {
                    let v7 = (*r382).clone();
                    f22(v1, v7, {
                        let a380 = v0;
                        length_list(a380)
                    }, false)
                }
                Adt9::C2(_, _, r383) => {
                    let v10 = (*r383).clone();
                    f22(v1, v10, {
                        let a381 = v0;
                        length_list(a381)
                    }, false)
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f35(v0: Bytes) -> R<Option<Bytes>> {
    let a385 = {
        let a384 = v0.clone();
        length_bytes(a384)
    };
    let a386 = 0u64;
    if nat_eq(a385, a386) {
        Ok(Some(f16(Str::lit("runtime support: defined-path: empty"))))
    } else {
        f52(Str::lit("a defined path segment"), f99(v0)?)
    }
}

pub fn f36(v0: Bytes, v1: Adt14, v2: Option<Bytes>, v3: Adt1) -> R<Option<Bytes>> {
    let m387 = v2;
    match m387 {
        None => {
            f100(v1, v0, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f37(v0: Adt1) -> Option<Adt8> {
    let m388 = v0;
    match m388 {
        Adt1::C0(_, _) => {
            None::<Adt8>
        }
        Adt1::C1(_, _) => {
            None::<Adt8>
        }
        Adt1::C2(_, _, _) => {
            None::<Adt8>
        }
        Adt1::C3(_, _, _) => {
            None::<Adt8>
        }
        Adt1::C4(_, _) => {
            None::<Adt8>
        }
        Adt1::C5(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C6(r389) => {
            let v17 = (*r389).clone();
            Some(v17)
        }
        Adt1::C7(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C9(_, _, _, _) => {
            None::<Adt8>
        }
        Adt1::C10(_) => {
            None::<Adt8>
        }
        Adt1::C11(_, _) => {
            None::<Adt8>
        }
        Adt1::C12(_, _) => {
            None::<Adt8>
        }
        Adt1::C13(_, _) => {
            None::<Adt8>
        }
    }
}

pub fn f38(v0: Bytes, v1: List<Bytes>) -> R<Option<Bytes>> {
    if f40(v0.clone(), f16(Str::lit("no_std"))) {
        let m390 = v1;
        match m390.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, _)) => {
                Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes no arguments"))))))
            }
        }
    } else {
        if f40(v0.clone(), f16(Str::lit("forbid"))) {
            let m391 = v1.clone();
            match m391.uncons() {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes a lint"))))))
                }
                Some((_, _)) => {
                    if f101(Fn10::F102, false, v1.clone())? {
                        Ok(Some(f19(f16(Str::lit("runtime support: attribute-args: ")), f19(f20(v0), f16(Str::lit(" takes a lint with a name"))))))
                    } else {
                        f52(Str::lit("a lint name"), v1)
                    }
                }
            }
        } else {
            Ok(Some(f19(f16(Str::lit("runtime support: attribute-name: ")), f19(f20(v0), f16(Str::lit(" is not an inner attribute of the closed set"))))))
        }
    }
}

pub fn f39(v0: Adt1) -> Option<(Bytes, Adt1)> {
    let m392 = v0;
    match m392 {
        Adt1::C0(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C1(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C2(_, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C3(_, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C4(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C5(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C6(_) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C7(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C9(_, _, _, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C10(_) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C11(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C12(_, _) => {
            None::<(Bytes, Adt1)>
        }
        Adt1::C13(v36, r393) => {
            let v37 = (*r393).clone();
            Some((v36, v37))
        }
    }
}

pub fn f40(v0: Bytes, v1: Bytes) -> bool {
    let a394 = v0;
    let a395 = v1;
    equal(a394, a395)
}

pub fn f41(v0: Adt1) -> Option<(List<Bytes>, List<Bytes>)> {
    let m396 = v0;
    match m396 {
        Adt1::C0(v1, v2) => {
            Some((v1, v2))
        }
        Adt1::C1(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C2(_, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C3(_, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C4(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C5(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C6(_) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C7(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C8(_, _, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C9(_, _, _, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C10(_) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C11(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C12(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
        Adt1::C13(_, _) => {
            None::<(List<Bytes>, List<Bytes>)>
        }
    }
}

pub fn f42(v0: Fn7, p400: List<Adt13>, p401: List<Adt1>) -> List<Adt13> {
    let m399 = (p400, p401);
    {
        let mut m399 = m399;
        loop {
            let step = {
                let (v1, v2) = m399;
                let m397 = v2;
                match m397.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt1>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt1>), List<Adt13>>(({
                            let c398: Fn7 = v0.clone();
                            c398.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: List<Adt13>, v2: Adt1) -> List<Adt13> {
    let m402 = f103(v2, v0);
    match m402 {
        None => {
            v1
        }
        Some(v3) => {
            let a403 = v1;
            let a404 = List::cons(v3, List::<Adt13>::nil());
            append_list(a403, a404)
        }
    }
}

pub fn f44(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> R<bool> {
    let a405 = f104(v0.clone(), v2.clone())?;
    let a406 = 1u64;
    if nat_eq(a405, a406) {
        let a407 = f104(v1.clone(), v2.clone())?;
        let a408 = 1u64;
        if nat_eq(a407, a408) {
            let m409 = f105(v0, v2.clone());
            match m409 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m410 = f105(v1, v2);
                    match m410 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            let a411 = v3;
                            let a412 = v4;
                            Ok(nat_eq(a411, a412))
                        }
                    }
                }
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f45(v0: Bytes, v1: Bytes) -> bool {
    let a417 = {
        let a413 = v0;
        let a414 = v1;
        compare_bytes(a413, a414)
    };
    let a418 = {
        let a415 = Bytes::lit(&[]);
        let a416 = Bytes::lit(&[0x01]);
        compare_bytes(a415, a416)
    };
    equal(a417, a418)
}

pub fn f46(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<Adt14, Bytes>> {
    Ok(f49({
        let m419 = v1.clone();
        match m419.uncons() {
            None => {
                Err::<Adt14, Bytes>(f16(Str::lit("runtime support: use-path: empty")))
            }
            Some((v3, _)) => {
                if if f40(v3.clone(), f16(Str::lit("core"))) {
                    true
                } else {
                    if match v0.clone() {
                        Adt14::C0(h420, _, _, _, _, _, _, _, _, _) => {
                            h420
                        }
                    } {
                        false
                    } else {
                        f40(v3.clone(), f16(Str::lit("std")))
                    }
                } {
                    Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                        Adt14::C0(h421, _, _, _, _, _, _, _, _, _) => {
                            h421
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, h422, _, _, _, _, _, _, _, _) => {
                            h422
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, h423, _, _, _, _, _, _, _) => {
                            h423
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, h424, _, _, _, _, _, _) => {
                            h424
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, h425, _, _, _, _, _) => {
                            h425
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h426, _, _, _, _) => {
                            h426
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, h427, _, _, _) => {
                            h427
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, h428, _, _) => {
                            h428
                        }
                    }, match v0.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, h429, _) => {
                            h429
                        }
                    }, {
                        let a432 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h430) => {
                                h430
                            }
                        };
                        let a433 = List::cons({
                            let m431 = v2.clone();
                            match m431.uncons() {
                                None => {
                                    f106(v1.clone())
                                }
                                Some((v5, _)) => {
                                    v5
                                }
                            }
                        }, List::<Bytes>::nil());
                        append_list(a432, a433)
                    }))
                } else {
                    Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: profile-root: ")), f19(f20(f19(f16(Str::lit("use ")), f83(v1.clone(), Str::lit("::"))?)), f19(f16(Str::lit(" starts at ")), f19(f20(v3), f16(Str::lit(", which the profile does not carry")))))))
                }
            }
        }
    }, f57(f107(Str::lit("a `use` segment"), v1.clone())?, f57(f52(Str::lit("a `use` alias"), v2)?, f62(f106(v1))?))))
}

pub fn f47(v0: Option<Bytes>, v1: Result<Adt14, Bytes>) -> Result<Adt14, Bytes> {
    let m434 = v0;
    match m434 {
        None => {
            v1
        }
        Some(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f48(v0: Str, v1: Bytes) -> R<Option<Bytes>> {
    if if f108(v1.clone())? {
        let a435 = f66(f109(), v1.clone())?;
        bool_not(a435)
    } else {
        false
    } {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f19(f16(Str::lit("runtime support: identifier: ")), f19(f16(v0), f19(f16(Str::lit(" ")), f19(f20(v1), f16(Str::lit(" is not an identifier"))))))))
    }
}

pub fn f49(v0: Result<Adt14, Bytes>, v1: Option<Bytes>) -> Result<Adt14, Bytes> {
    let m436 = v0.clone();
    match m436 {
        Ok(_) => {
            let m437 = v1;
            match m437 {
                None => {
                    v0
                }
                Some(v4) => {
                    Err::<Adt14, Bytes>(v4)
                }
            }
        }
        Err(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f50(v0: Adt14, v1: Bytes) -> R<Result<Adt14, Bytes>> {
    if f66(match v0.clone() {
        Adt14::C0(_, h438, _, _, _, _, _, _, _, _) => {
            h438
        }
    }, v1.clone())? {
        Ok(Err::<Adt14, Bytes>(f19(f16(Str::lit("runtime support: duplicate-name: ")), f19(f20(v1), f16(Str::lit(" is declared twice"))))))
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
            Adt14::C0(h439, _, _, _, _, _, _, _, _, _) => {
                h439
            }
        }, {
            let a441 = match v0.clone() {
                Adt14::C0(_, h440, _, _, _, _, _, _, _, _) => {
                    h440
                }
            };
            let a442 = List::cons(v1, List::<Bytes>::nil());
            append_list(a441, a442)
        }, match v0.clone() {
            Adt14::C0(_, _, h443, _, _, _, _, _, _, _) => {
                h443
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h444, _, _, _, _, _, _) => {
                h444
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h445, _, _, _, _, _) => {
                h445
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h446, _, _, _, _) => {
                h446
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h447, _, _, _) => {
                h447
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h448, _, _) => {
                h448
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h449, _) => {
                h449
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h450) => {
                h450
            }
        })))
    }
}

pub fn f51(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F111(v0), None::<Bytes>, v1)
}

pub fn f52(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F112(v0), None::<Bytes>, v1)
}

pub fn f53(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f113(Fn12::F114, v0, v1)
}

pub fn f54(v0: Adt14, v1: Adt2, v2: List<Adt8>) -> Adt14 {
    let m451 = f115(v1);
    match m451 {
        None => {
            v0
        }
        Some(v3) => {
            f116(v0, v3, v2)
        }
    }
}

pub fn f55(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f117(Fn13::F118, v0, v1)
}

pub fn f56(v0: Adt2, v1: u64) -> R<List<Bytes>> {
    let m452 = v0;
    match m452 {
        Adt2::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C10(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C11 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C12 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C13 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C14 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C15(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C16 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C17 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C18 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C19(_, r455) => {
            let v11 = (*r455).clone();
            Ok(List::cons(f59(v1)?, f56(v11, {
                let a453 = v1;
                let a454 = 1u64;
                nat_add(a453, a454)?
            })?))
        }
    }
}

pub fn f57(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m456 = v0;
    match m456 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f58(v0: Adt2) -> R<u64> {
    let m457 = v0;
    match m457 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1 => {
            Ok(0u64)
        }
        Adt2::C2 => {
            Ok(0u64)
        }
        Adt2::C3 => {
            Ok(0u64)
        }
        Adt2::C4 => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_, _) => {
            Ok(0u64)
        }
        Adt2::C11 => {
            Ok(0u64)
        }
        Adt2::C12 => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14 => {
            Ok(0u64)
        }
        Adt2::C15(_) => {
            Ok(0u64)
        }
        Adt2::C16 => {
            Ok(0u64)
        }
        Adt2::C17 => {
            Ok(0u64)
        }
        Adt2::C18 => {
            Ok(0u64)
        }
        Adt2::C19(_, r460) => {
            let v10 = (*r460).clone();
            let a458 = f58(v10)?;
            let a459 = 1u64;
            nat_add(a458, a459)
        }
    }
}

pub fn f59(v0: u64) -> R<Bytes> {
    f119(v0)
}

pub fn f60(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m462 = f120(Fn14::F121(v0), Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m462 {
        Adt18::C0(_, h461) => {
            Ok(h461)
        }
    }
}

pub fn f61(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m464 = f120(Fn14::F122, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m464 {
        Adt18::C0(_, h463) => {
            Ok(h463)
        }
    }
}

pub fn f62(v0: Bytes) -> R<Option<Bytes>> {
    let m465 = f123(v0.clone())?;
    match m465 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f19(f16(Str::lit("runtime support: hidden-effect: ")), f19(f20(v0), f19(f16(Str::lit(" realizes `")), f19(v1, f16(Str::lit("`, which the root does not admit"))))))))
        }
    }
}

pub fn f63(v0: Adt14, v1: Bytes) -> R<Option<u64>> {
    let m466 = f124(v0.clone(), v1.clone());
    match m466 {
        None => {
            let m468 = f125(v0.clone(), v1.clone());
            match m468 {
                None => {
                    let m470 = f126(v0, v1.clone());
                    match m470 {
                        None => {
                            f127(v1)
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h471) => {
                                    h471
                                }
                            }))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Some(match v3 {
                        Adt12::C0(_, _, h469) => {
                            h469
                        }
                    }))
                }
            }
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt11::C0(_, _, h467) => {
                    h467
                }
            }))
        }
    }
}

pub fn f64(v0: Adt14, v1: Bytes) -> bool {
    let m472 = f124(v0.clone(), v1.clone());
    match m472 {
        None => {
            let m473 = f125(v0.clone(), v1.clone());
            match m473 {
                None => {
                    let m474 = f126(v0, v1.clone());
                    match m474 {
                        None => {
                            f128(v1)
                        }
                        Some(_) => {
                            false
                        }
                    }
                }
                Some(_) => {
                    false
                }
            }
        }
        Some(_) => {
            false
        }
    }
}

pub fn f65(v0: List<Adt17>, v1: Bytes) -> Option<Adt17> {
    f129(Fn15::F130(v1), None::<Adt17>, v0)
}

pub fn f66(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f101(Fn10::F131(v1), false, v0)
}

pub fn f67(v0: u64) -> R<Option<Bytes>> {
    let a475 = v0;
    let a476 = 1u64;
    if nat_eq(a475, a476) {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f19(f16(Str::lit("runtime support: char-literal: ")), f19(f16(Str::lit("a character literal of ")), f19(f59(v0)?, f16(Str::lit(" scalar values")))))))
    }
}

pub fn f68(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let m477 = f107(Str::lit("a path segment"), v1.clone())?;
    match m477 {
        None => {
            let m478 = f62(f132(v1.clone()))?;
            match m478 {
                None => {
                    f133(v0, v1)
                }
                Some(v3) => {
                    Ok(Some(v3))
                }
            }
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f69(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    let m479 = f62(f132(v1.clone()))?;
    match m479 {
        None => {
            if f134(v0.clone(), v1.clone())? {
                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(f83(v1, Str::lit("::"))?), f16(Str::lit(", which requires heap allocation")))))))
            } else {
                let m480 = f135(v0, v1.clone())?;
                match m480 {
                    None => {
                        Ok(Some(f19(f16(Str::lit("runtime support: call-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f16(Str::lit(" is neither declared nor in the closed table"))))))
                    }
                    Some(v4) => {
                        let a481 = v4;
                        let a482 = v2;
                        if nat_eq(a481, a482) {
                            Ok(None::<Bytes>)
                        } else {
                            Ok(Some(f19(f16(Str::lit("runtime support: call-arity: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" takes ")), f19(f59(v4)?, f19(f16(Str::lit(" arguments, given ")), f59(v2)?)))))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f70(v0: Adt3) -> R<u64> {
    let m483 = v0;
    match m483 {
        Adt3::C0(_) => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_, _) => {
            Ok(0u64)
        }
        Adt3::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C5(_, _) => {
            Ok(0u64)
        }
        Adt3::C6(_, _) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_, _) => {
            Ok(0u64)
        }
        Adt3::C9(_, _) => {
            Ok(0u64)
        }
        Adt3::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C11(_, _) => {
            Ok(0u64)
        }
        Adt3::C12(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C13(_, _) => {
            Ok(0u64)
        }
        Adt3::C14(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C15(_) => {
            Ok(0u64)
        }
        Adt3::C16(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C17 => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C22(_) => {
            Ok(0u64)
        }
        Adt3::C23(_, _) => {
            Ok(0u64)
        }
        Adt3::C24(_, _) => {
            Ok(0u64)
        }
        Adt3::C25(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C26(_) => {
            Ok(0u64)
        }
        Adt3::C27(_) => {
            Ok(0u64)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C29(_, _) => {
            Ok(0u64)
        }
        Adt3::C30 => {
            Ok(0u64)
        }
        Adt3::C31 => {
            Ok(0u64)
        }
        Adt3::C32(_, r486) => {
            let v60 = (*r486).clone();
            let a484 = f70(v60)?;
            let a485 = 1u64;
            nat_add(a484, a485)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f71(v0: Adt14, v1: Adt16, p492: Adt3) -> R<Option<Bytes>> {
    let m491 = p492;
    {
        let mut m491 = m491;
        loop {
            let step = {
                let v2 = m491;
                let m487 = v2;
                match m487 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(r489, r490) => {
                        let v61 = (*r489).clone();
                        let v62 = (*r490).clone();
                        let m488 = f23(v0.clone(), v1.clone(), v61)?;
                        match m488 {
                            None => {
                                Ok::<Adt3, R<Option<Bytes>>>(v62)
                            }
                            Some(v63) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v63)))
                            }
                        }
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                }
            };
            match step {
                Ok(next) => { m491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m493 = f136(v3.clone())?;
    match m493 {
        None => {
            let v6: List<u64> = f138(v0.clone(), v3.clone());
            let m497 = v6.clone();
            match m497.uncons() {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: method-name: ")), f19(f20(v3), f16(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f139(v6.clone(), v4) {
                        f140(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v3), f19(f16(Str::lit(" takes ")), f19(f141(v6)?, f19(f16(Str::lit(" arguments, given ")), f59(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f137(v3.clone())? {
                match v0 {
                    Adt14::C0(h494, _, _, _, _, _, _, _, _, _) => {
                        h494
                    }
                }
            } else {
                false
            } {
                Ok(Some(f19(f16(Str::lit("runtime support: hidden-allocation: ")), f19(f16(Str::lit("rust-core renders ")), f19(f20(v3), f16(Str::lit(", which requires heap allocation")))))))
            } else {
                let a495 = v5;
                let a496 = v4;
                if nat_eq(a495, a496) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f19(f16(Str::lit("runtime support: method-arity: ")), f19(f20(v3), f19(f16(Str::lit(" takes ")), f19(f59(v5)?, f19(f16(Str::lit(" arguments, given ")), f59(v4)?)))))))
                }
            }
        }
    }
}

pub fn f73(v0: Adt3) -> R<Option<Bytes>> {
    if f142(v0)? {
        Ok(Some(f19(f16(Str::lit("runtime support: struct-head: ")), f16(Str::lit("a struct literal outside parentheses in the head of a condition, a loop, or a match")))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f74(v0: Adt3) -> bool {
    let m498 = v0;
    match m498 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_, _) => {
            false
        }
        Adt3::C4(_, _, _) => {
            false
        }
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            false
        }
        Adt3::C31 => {
            true
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f75(v0: Adt14, v1: Adt16, p509: Adt3) -> R<Option<Bytes>> {
    let m508 = p509;
    {
        let mut m508 = m508;
        loop {
            let step = {
                let v2 = m508;
                let m499 = v2;
                match m499 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(r504, r505, r506, r507) => {
                        let v67 = (*r504).clone();
                        let v68 = (*r505).clone();
                        let v69 = (*r506).clone();
                        let v70 = (*r507).clone();
                        let m500 = f79(v0.clone(), v67.clone())?;
                        match m500 {
                            None => {
                                let m501 = f80(List::<Adt17>::nil(), v67)?;
                                match m501 {
                                    Ok(v73) => {
                                        let v74: Adt16 = f78(v1.clone(), v73, false);
                                        let m502 = if f81(v68.clone()) {
                                            None::<Bytes>
                                        } else {
                                            f23(v0.clone(), v74.clone(), v68)?
                                        };
                                        match m502 {
                                            None => {
                                                let m503 = f23(v0.clone(), v74, v69)?;
                                                match m503 {
                                                    None => {
                                                        Ok::<Adt3, R<Option<Bytes>>>(v70)
                                                    }
                                                    Some(v76) => {
                                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v76)))
                                                    }
                                                }
                                            }
                                            Some(v75) => {
                                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v75)))
                                            }
                                        }
                                    }
                                    Err(v72) => {
                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v72)))
                                    }
                                }
                            }
                            Some(v71) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v71)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Adt16, p520: Adt3) -> Option<Bytes> {
    let m519 = p520;
    {
        let mut m519 = m519;
        loop {
            let step = {
                let v1 = m519;
                let m510 = v1;
                match m510 {
                    Adt3::C0(v2) => {
                        let m512 = f65(match v0.clone() {
                            Adt16::C0(_, h511, _, _) => {
                                h511
                            }
                        }, v2.clone());
                        match m512 {
                            None => {
                                Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: variable-scope: ")), f19(f20(v2), f16(Str::lit(" is not in scope"))))))
                            }
                            Some(v3) => {
                                if match v3.clone() {
                                    Adt17::C0(_, h513, _) => {
                                        h513
                                    }
                                } {
                                    Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                } else {
                                    if f143(match v3 {
                                        Adt17::C0(_, _, h514) => {
                                            (*h514).clone()
                                        }
                                    }) {
                                        Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                    } else {
                                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-immutable: ")), f19(f20(v2), f16(Str::lit(" is assigned and is not `mut` or `&mut`"))))))
                                    }
                                }
                            }
                        }
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C5(r515, _) => {
                        let v11 = (*r515).clone();
                        Ok::<Adt3, Option<Bytes>>(v11)
                    }
                    Adt3::C6(r516, _) => {
                        let v13 = (*r516).clone();
                        Ok::<Adt3, Option<Bytes>>(v13)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C24(r517, _) => {
                        let v48 = (*r517).clone();
                        Ok::<Adt3, Option<Bytes>>(v48)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C26(r518) => {
                        let v53 = (*r518).clone();
                        Ok::<Adt3, Option<Bytes>>(v53)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f19(f16(Str::lit("runtime support: assign-place: ")), f16(Str::lit("an assignment to a value that is no place")))))
                    }
                }
            };
            match step {
                Ok(next) => { m519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m521 = v2;
    match m521 {
        Adt3::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C3(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C4(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C8(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C11(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C12(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C13(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C14(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C16(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C17 => {
            Ok(None::<Bytes>)
        }
        Adt3::C18(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C19(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C20(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C21(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C22(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C23(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C24(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C25(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C26(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C27(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C29(r523, r524) => {
            let v59 = (*r523).clone();
            let v60 = (*r524).clone();
            let m522 = f89(v0.clone(), v1, v59)?;
            match m522 {
                Ok(v62) => {
                    f23(v0, v62, v60)
                }
                Err(v61) => {
                    Ok(Some(v61))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f78(v0: Adt16, v1: List<Adt17>, v2: bool) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(h525, _, _, _) => {
            h525
        }
    }, {
        let a527 = v1;
        let a528 = match v0.clone() {
            Adt16::C0(_, h526, _, _) => {
                h526
            }
        };
        append_list(a527, a528)
    }, std::rc::Rc::new(match v0.clone() {
        Adt16::C0(_, _, h529, _) => {
            (*h529).clone()
        }
    }), if match v0 {
        Adt16::C0(_, _, _, h530) => {
            h530
        }
    } {
        true
    } else {
        v2
    })
}

pub fn f79(v0: Adt14, v1: Adt4) -> R<Option<Bytes>> {
    let m531 = v1;
    match m531 {
        Adt4::C0 => {
            Ok(None::<Bytes>)
        }
        Adt4::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C3(v4) => {
            f68(v0, v4)
        }
        Adt4::C4(r541) => {
            let v5 = (*r541).clone();
            f79(v0, v5)
        }
        Adt4::C5(v6, r542) => {
            let v7 = (*r542).clone();
            if f144(v6.clone()) {
                f79(v0, v7)
            } else {
                let m532 = f68(v0.clone(), v6.clone())?;
                match m532 {
                    None => {
                        let m533 = f145(v0.clone(), v6, v7.clone())?;
                        match m533 {
                            None => {
                                f79(v0, v7)
                            }
                            Some(v9) => {
                                Ok(Some(v9))
                            }
                        }
                    }
                    Some(v8) => {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt4::C6(v10, v11, r543) => {
            let v12 = (*r543).clone();
            let m534 = f68(v0.clone(), v10.clone())?;
            match m534 {
                None => {
                    let m535 = f52(Str::lit("a pattern field name"), v11.clone())?;
                    match m535 {
                        None => {
                            let a537 = {
                                let a536 = v11.clone();
                                length_list(a536)
                            };
                            let a538 = f146(v12.clone())?;
                            if nat_eq(a537, a538) {
                                let m539 = f147(v0.clone(), v10, v11)?;
                                match m539 {
                                    None => {
                                        f79(v0, v12)
                                    }
                                    Some(v15) => {
                                        Ok(Some(v15))
                                    }
                                }
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f16(Str::lit("a struct pattern with unequal names and patterns")))))
                            }
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt4::C7(v16) => {
            f67(v16)
        }
        Adt4::C8 => {
            Ok(None::<Bytes>)
        }
        Adt4::C9(r544, r545) => {
            let v17 = (*r544).clone();
            let v18 = (*r545).clone();
            let m540 = f79(v0.clone(), v17)?;
            match m540 {
                None => {
                    f79(v0, v18)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
    }
}

pub fn f80(v0: List<Adt17>, v1: Adt4) -> R<Result<List<Adt17>, Bytes>> {
    let m546 = v1;
    match m546 {
        Adt4::C0 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C1(v2) => {
            let m547 = f48(Str::lit("a pattern binding"), v2.clone())?;
            match m547 {
                None => {
                    let m548 = f65(v0.clone(), v2.clone());
                    match m548 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v2, false, std::rc::Rc::new(Adt2::C16)), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: pattern-duplicate: ")), f19(f20(v2), f16(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Err::<List<Adt17>, Bytes>(v3))
                }
            }
        }
        Adt4::C2(v5) => {
            let m549 = f48(Str::lit("a pattern binding"), v5.clone())?;
            match m549 {
                None => {
                    let m550 = f65(v0.clone(), v5.clone());
                    match m550 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v5, true, std::rc::Rc::new(Adt2::C16)), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: pattern-duplicate: ")), f19(f20(v5), f16(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Err::<List<Adt17>, Bytes>(v6))
                }
            }
        }
        Adt4::C3(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C4(r552) => {
            let v9 = (*r552).clone();
            f80(v0, v9)
        }
        Adt4::C5(_, r553) => {
            let v11 = (*r553).clone();
            f80(v0, v11)
        }
        Adt4::C6(_, _, r554) => {
            let v14 = (*r554).clone();
            f80(v0, v14)
        }
        Adt4::C7(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C8 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C9(r555, r556) => {
            let v16 = (*r555).clone();
            let v17 = (*r556).clone();
            let m551 = f80(v0, v16)?;
            match m551 {
                Ok(v18) => {
                    f80(v18, v17)
                }
                Err(v19) => {
                    Ok(Err::<List<Adt17>, Bytes>(v19))
                }
            }
        }
    }
}

pub fn f81(v0: Adt3) -> bool {
    let m557 = v0;
    match m557 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_, _) => {
            false
        }
        Adt3::C4(_, _, _) => {
            false
        }
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            true
        }
        Adt3::C31 => {
            false
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f82(v0: Adt14, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m558 = v1;
    match m558.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((v2, v3)) => {
            let m559 = v3;
            match m559.uncons() {
                None => {
                    let m560 = f124(v0, v2);
                    match m560 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt11::C0(_, h561, _) => {
                                    h561
                                }
                            })
                        }
                    }
                }
                Some((v5, v6)) => {
                    let m562 = v6;
                    match m562.uncons() {
                        None => {
                            let m563 = f125(v0, v2);
                            match m563 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f148(match v7 {
                                        Adt12::C0(_, h564, _) => {
                                            h564
                                        }
                                    }, v5)
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<List<Bytes>>
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f149(v0, f16(v1))
}

pub fn f84(v0: List<Bytes>, v1: List<Bytes>, v2: u64) -> R<bool> {
    let a567 = {
        let a565 = v0.clone();
        length_list(a565)
    };
    let a568 = {
        let a566 = v1.clone();
        length_list(a566)
    };
    if nat_eq(a567, a568) {
        let a570 = {
            let a569 = v0.clone();
            length_list(a569)
        };
        let a571 = v2;
        if nat_eq(a570, a571) {
            f101(Fn10::F150(v1.clone(), v0), true, v1)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f85(v0: Adt2) -> bool {
    let m572 = v0;
    match m572 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            true
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f86(v0: Str, v1: Adt2) -> Option<Bytes> {
    let m573 = v1;
    match m573 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            Some(f19(f16(Str::lit("runtime support: type-scope: ")), f19(f16(v0), f16(Str::lit(" names a generic parameter, which the printer writes without the generics in scope")))))
        }
        Adt2::C7(r576) => {
            let v3 = (*r576).clone();
            f86(v0, v3)
        }
        Adt2::C8(r577) => {
            let v4 = (*r577).clone();
            f86(v0, v4)
        }
        Adt2::C9(r578, r579) => {
            let v5 = (*r578).clone();
            let v6 = (*r579).clone();
            let m574 = f86(v0.clone(), v5);
            match m574 {
                None => {
                    f86(v0, v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r580) => {
            let v9 = (*r580).clone();
            f86(v0, v9)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(r581) => {
            let v10 = (*r581).clone();
            f86(v0, v10)
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(r582, r583) => {
            let v11 = (*r582).clone();
            let v12 = (*r583).clone();
            let m575 = f86(v0.clone(), v11);
            match m575 {
                None => {
                    f86(v0, v12)
                }
                Some(v13) => {
                    Some(v13)
                }
            }
        }
    }
}

pub fn f87(v0: Bytes) -> Option<u64> {
    let a584 = v0.clone();
    let a585 = f16(Str::lit("parse"));
    if equal(a584, a585) {
        Some(1u64)
    } else {
        let a586 = v0;
        let a587 = f16(Str::lit("collect"));
        if equal(a586, a587) {
            Some(1u64)
        } else {
            None::<u64>
        }
    }
}

pub fn f88(v0: Bytes) -> u64 {
    let a588 = v0.clone();
    let a589 = f16(Str::lit("parse"));
    if equal(a588, a589) {
        0u64
    } else {
        let a590 = v0;
        let a591 = f16(Str::lit("collect"));
        let _ = equal(a590, a591);
        0u64
    }
}

pub fn f89(v0: Adt14, p609: Adt16, p610: Adt3) -> R<Result<Adt16, Bytes>> {
    let m608 = (p609, p610);
    {
        let mut m608 = m608;
        loop {
            let step = {
                let (v1, v2) = m608;
                let m592 = v2;
                match m592 {
                    Adt3::C0(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C1(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C2(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C3(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C5(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C6(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C7(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C8(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C9(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C11(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C13(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C15(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C17 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C18(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C19(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C22(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C23(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C24(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C26(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C27(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C29(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C30 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C31 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C32(r602, r603) => {
                        let v61 = (*r602).clone();
                        let v62 = (*r603).clone();
                        let m593 = f23(v0.clone(), v1.clone(), v61)?;
                        match m593 {
                            None => {
                                Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((v1, v62))
                            }
                            Some(v63) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v63)))
                            }
                        }
                    }
                    Adt3::C33(r604, r605, r606, r607) => {
                        let v64 = (*r604).clone();
                        let v65 = (*r605).clone();
                        let v66 = (*r606).clone();
                        let v67 = (*r607).clone();
                        let m598 = if f85(v65.clone()) {
                            None::<Bytes>
                        } else {
                            f57(f22(v0.clone(), v65.clone(), {
                                let a595 = match v1.clone() {
                                    Adt16::C0(h594, _, _, _) => {
                                        h594
                                    }
                                };
                                length_list(a595)
                            }, {
                                let a597 = f85(match v1.clone() {
                                    Adt16::C0(_, _, h596, _) => {
                                        (*h596).clone()
                                    }
                                });
                                bool_not(a597)
                            })?, f86(Str::lit("a `let` type"), v65))
                        };
                        match m598 {
                            None => {
                                let m599 = f23(v0.clone(), v1.clone(), v66)?;
                                match m599 {
                                    None => {
                                        let m600 = f80(List::<Adt17>::nil(), v64.clone())?;
                                        match m600 {
                                            Ok(v71) => {
                                                let m601 = f79(v0.clone(), v64)?;
                                                match m601 {
                                                    None => {
                                                        Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((f78(v1, v71, false), v67))
                                                    }
                                                    Some(v72) => {
                                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v72)))
                                                    }
                                                }
                                            }
                                            Err(v70) => {
                                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v70)))
                                            }
                                        }
                                    }
                                    Some(v69) => {
                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v69)))
                                    }
                                }
                            }
                            Some(v68) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v68)))
                            }
                        }
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Fn8, p614: Adt18, p615: List<Adt5>) -> R<Adt18> {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m611 = v2;
                match m611.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt5>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt5>), R<Adt18>>(({
                            let c612: Fn8 = v0.clone();
                            c612.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt18, v1: Adt5) -> R<Adt18> {
    let m617 = match v0.clone() {
        Adt18::C0(_, h616) => {
            h616
        }
    };
    match m617 {
        None => {
            if f66(match v0.clone() {
                Adt18::C0(h618, _) => {
                    h618
                }
            }, match v1.clone() {
                Adt5::C0(h619, _, _, _, _) => {
                    h619
                }
            })? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h620, _) => {
                        h620
                    }
                }, Some(match v1 {
                    Adt5::C0(h621, _, _, _, _) => {
                        h621
                    }
                })))
            } else {
                Ok(Adt18::C0(List::cons(match v1 {
                    Adt5::C0(h622, _, _, _, _) => {
                        h622
                    }
                }, match v0 {
                    Adt18::C0(h623, _) => {
                        h623
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f92(v0: Adt14, v1: Adt5, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m625 = f48(Str::lit("a method name"), match v1.clone() {
        Adt5::C0(h624, _, _, _, _) => {
            h624
        }
    })?;
    match m625 {
        None => {
            if if match v1.clone() {
                Adt5::C0(_, _, h626, _, _) => {
                    h626
                }
            } {
                let a628 = match v1.clone() {
                    Adt5::C0(_, h627, _, _, _) => {
                        h627
                    }
                };
                bool_not(a628)
            } else {
                false
            } {
                Ok(Some(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f20(match v1 {
                    Adt5::C0(h629, _, _, _, _) => {
                        h629
                    }
                }), f16(Str::lit(" is exclusive without a receiver"))))))
            } else {
                let m631 = f151(v0.clone(), match v1.clone() {
                    Adt5::C0(_, _, _, h630, _) => {
                        h630
                    }
                }, v2, v3)?;
                match m631 {
                    None => {
                        f22(v0, match v1 {
                            Adt5::C0(_, _, _, _, h632) => {
                                (*h632).clone()
                            }
                        }, v2, v3)
                    }
                    Some(v5) => {
                        Ok(Some(v5))
                    }
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f93(v0: Fn9, p636: Option<Bytes>, p637: List<Adt7>) -> R<Option<Bytes>> {
    let m635 = (p636, p637);
    {
        let mut m635 = m635;
        loop {
            let step = {
                let (v1, v2) = m635;
                let m633 = v2;
                match m633.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(({
                            let c634: Fn9 = v0.clone();
                            c634.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt14, v1: Option<Bytes>, v2: Adt7) -> R<Option<Bytes>> {
    let m638 = v1;
    match m638 {
        None => {
            let m640 = f48(Str::lit("a generic parameter"), match v2.clone() {
                Adt7::C0(h639, _) => {
                    h639
                }
            })?;
            match m640 {
                None => {
                    f110(Fn11::F152(std::rc::Rc::new(v2.clone()), std::rc::Rc::new(v0)), None::<Bytes>, match v2 {
                        Adt7::C0(_, h641) => {
                            h641
                        }
                    })
                }
                Some(v4) => {
                    Ok(Some(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f95(v0: Adt14, v1: Bytes) -> Option<List<Adt5>> {
    let m642 = f153(v0, v1.clone());
    match m642 {
        None => {
            let m644 = f154(v1.clone());
            match m644 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f155(v1.clone()), List::<Adt6>::nil(), std::rc::Rc::new(if f155(v1) {
                        Adt2::C0
                    } else {
                        Adt2::C5
                    })), List::<Adt5>::nil()))
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt10::C0(_, h643) => {
                    h643
                }
            })
        }
    }
}

pub fn f96(v0: List<Adt8>, v1: Bytes, v2: Option<Bytes>, v3: Adt5) -> R<Option<Bytes>> {
    let m645 = v2;
    match m645 {
        None => {
            let v5: u64 = f156(v0.clone(), match v3.clone() {
                Adt5::C0(h646, _, _, _, _) => {
                    h646
                }
            })?;
            let a647 = v5;
            let a648 = 0u64;
            if nat_eq(a647, a648) {
                Ok(Some(f19(f16(Str::lit("runtime support: impl-complete: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` does not state ")), f20(match v3 {
                    Adt5::C0(h649, _, _, _, _) => {
                        h649
                    }
                })))))))
            } else {
                let a650 = v5;
                let a651 = 1u64;
                if nat_eq(a650, a651) {
                    let m653 = f157(v0, match v3.clone() {
                        Adt5::C0(h652, _, _, _, _) => {
                            h652
                        }
                    });
                    match m653 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a655 = f158(match v6 {
                                Adt8::C0(h654, _, _, _) => {
                                    (*h654).clone()
                                }
                            })?;
                            let a656 = f158(v3.clone())?;
                            if equal(a655, a656) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: impl-signature: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` states ")), f19(f20(match v3 {
                                    Adt5::C0(h657, _, _, _, _) => {
                                        h657
                                    }
                                }), f16(Str::lit(" with a signature other than the trait's")))))))))
                            }
                        }
                    }
                } else {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-complete: ")), f19(f16(Str::lit("`impl ")), f19(v1, f19(f16(Str::lit("` states ")), f19(f20(match v3 {
                        Adt5::C0(h658, _, _, _, _) => {
                            h658
                        }
                    }), f16(Str::lit(" twice")))))))))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f97(v0: List<Adt7>, v1: Adt14, v2: List<Adt5>, v3: Bytes, v4: Adt2, v5: Option<Bytes>, v6: Adt8) -> R<Option<Bytes>> {
    let m659 = v5;
    match m659 {
        None => {
            let a663 = f159(v2, {
                let m662 = match v6.clone() {
                    Adt8::C0(h661, _, _, _) => {
                        (*h661).clone()
                    }
                };
                match m662 {
                    Adt5::C0(h660, _, _, _, _) => {
                        h660
                    }
                }
            });
            if bool_not(a663) {
                Ok(Some(f19(f16(Str::lit("runtime support: impl-extra: ")), f19(f16(Str::lit("`impl ")), f19(v3, f19(f16(Str::lit("` states ")), f19(f20({
                    let m666 = match v6 {
                        Adt8::C0(h665, _, _, _) => {
                            (*h665).clone()
                        }
                    };
                    match m666 {
                        Adt5::C0(h664, _, _, _, _) => {
                            h664
                        }
                    }
                }), f16(Str::lit(", which the trait does not")))))))))
            } else {
                if match v6.clone() {
                    Adt8::C0(_, _, h667, _) => {
                        h667
                    }
                } {
                    Ok(Some(f19(f16(Str::lit("runtime support: impl-visibility: ")), f19(f20({
                        let m670 = match v6 {
                            Adt8::C0(h669, _, _, _) => {
                                (*h669).clone()
                            }
                        };
                        match m670 {
                            Adt5::C0(h668, _, _, _, _) => {
                                h668
                            }
                        }
                    }), f19(f16(Str::lit(" of `impl ")), f19(v3, f16(Str::lit("` is public; a trait method has the trait's visibility"))))))))
                } else {
                    f98(v1, v0, v4, v6)
                }
            }
        }
        Some(v7) => {
            Ok(Some(v7))
        }
    }
}

pub fn f98(v0: Adt14, v1: List<Adt7>, v2: Adt2, v3: Adt8) -> R<Option<Bytes>> {
    let m672 = f28(v0.clone(), match v3.clone() {
        Adt8::C0(_, h671, _, _) => {
            h671
        }
    })?;
    match m672 {
        None => {
            let v5: List<Adt7> = {
                let a674 = v1;
                let a675 = match v3.clone() {
                    Adt8::C0(_, h673, _, _) => {
                        h673
                    }
                };
                append_list(a674, a675)
            };
            let v6: u64 = {
                let a676 = v5.clone();
                length_list(a676)
            };
            let v7: Adt5 = match v3.clone() {
                Adt8::C0(h677, _, _, _) => {
                    (*h677).clone()
                }
            };
            let m679 = f92(v0.clone(), v7.clone(), v6, {
                let a678 = f85(v2.clone());
                bool_not(a678)
            })?;
            match m679 {
                None => {
                    let m684 = if match v7.clone() {
                        Adt5::C0(_, h680, _, _, _) => {
                            h680
                        }
                    } {
                        if f85(v2.clone()) {
                            Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: function-receiver: ")), f19(f20(match v7.clone() {
                                Adt5::C0(h681, _, _, _, _) => {
                                    h681
                                }
                            }), f16(Str::lit(" takes `&self` outside an impl")))))
                        } else {
                            Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(f16(Str::lit("self")), match v7.clone() {
                                Adt5::C0(_, _, h682, _, _) => {
                                    h682
                                }
                            }, std::rc::Rc::new(if match v7.clone() {
                                Adt5::C0(_, _, h683, _, _) => {
                                    h683
                                }
                            } {
                                Adt2::C8(std::rc::Rc::new(v2.clone()))
                            } else {
                                Adt2::C7(std::rc::Rc::new(v2.clone()))
                            })), List::<Adt17>::nil()))
                        }
                    } else {
                        Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil())
                    };
                    match m684 {
                        Ok(v10) => {
                            let m687 = f160(Ok::<List<Adt17>, Bytes>(v10), match v7.clone() {
                                Adt5::C0(_, _, _, h685, _) => {
                                    h685
                                }
                            }, match v7 {
                                Adt5::C0(h686, _, _, _, _) => {
                                    h686
                                }
                            });
                            match m687 {
                                Ok(v12) => {
                                    f77(v0, Adt16::C0(v5, v12, std::rc::Rc::new(v2), false), match v3 {
                                        Adt8::C0(_, _, _, h688) => {
                                            (*h688).clone()
                                        }
                                    })
                                }
                                Err(v11) => {
                                    Ok(Some(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Some(v9))
                        }
                    }
                }
                Some(v8) => {
                    Ok(Some(v8))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a689 = v0.clone();
        length_bytes(a689)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h690, _) = f161(Fn16::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h690
    };
    let v3: u64 = {
        let (h691, _) = {
            let (_, h692) = v2.clone();
            h692
        };
        h691
    };
    Ok(f163(List::cons(f164(v0, v3, {
        let a693 = v1;
        let a694 = v3;
        nat_sub(a693, a694)
    }), {
        let (_, h695) = {
            let (_, h696) = v2;
            h696
        };
        h695
    })))
}

pub fn f100(v0: Adt14, v1: Bytes, v2: Adt1) -> R<Option<Bytes>> {
    let m697 = v2;
    match m697 {
        Adt1::C0(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C1(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C2(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C3(_, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C4(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C6(r698) => {
            let v4 = (*r698).clone();
            f32(v0, v4)
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C10(v3) => {
            f35(v3)
        }
        Adt1::C11(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C12(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C13(_, _) => {
            Ok(Some(f19(f16(Str::lit("runtime support: module-member: ")), f19(f20(v1), f16(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
    }
}

pub fn f101(v0: Fn10, p702: bool, p703: List<Bytes>) -> R<bool> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m699 = v2;
                match m699.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c700: Fn10 = v0.clone();
                            c700.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: bool, v1: Bytes) -> bool {
    if v0 {
        true
    } else {
        let a705 = {
            let a704 = v1;
            length_bytes(a704)
        };
        let a706 = 0u64;
        nat_eq(a705, a706)
    }
}

pub fn f103(v0: Adt1, v1: Bytes) -> Option<Adt13> {
    let m707 = f39(v0);
    match m707 {
        None => {
            None::<Adt13>
        }
        Some(v2) => {
            if f40({
                let (h708, _) = v2;
                h708
            }, v1) {
                let m710 = f37({
                    let (_, h709) = v2;
                    h709
                });
                match m710 {
                    None => {
                        None::<Adt13>
                    }
                    Some(v3) => {
                        Some(Adt13::C0({
                            let m713 = match v3.clone() {
                                Adt8::C0(h712, _, _, _) => {
                                    (*h712).clone()
                                }
                            };
                            match m713 {
                                Adt5::C0(h711, _, _, _, _) => {
                                    h711
                                }
                            }
                        }, {
                            let a717 = {
                                let m716 = match v3 {
                                    Adt8::C0(h715, _, _, _) => {
                                        (*h715).clone()
                                    }
                                };
                                match m716 {
                                    Adt5::C0(_, _, _, h714, _) => {
                                        h714
                                    }
                                }
                            };
                            length_list(a717)
                        }))
                    }
                }
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f104(v0: List<Adt13>, v1: Bytes) -> R<u64> {
    f165(Fn17::F166(v1), 0u64, v0)
}

pub fn f105(v0: List<Adt13>, v1: Bytes) -> Option<u64> {
    f167(Fn18::F168(v1), None::<u64>, v0)
}

pub fn f106(v0: List<Bytes>) -> Bytes {
    f169(Fn19::F170, f16(Str::lit("")), v0)
}

pub fn f107(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f110(Fn11::F171(v0), None::<Bytes>, v1)
}

pub fn f108(v0: Bytes) -> R<bool> {
    let a719 = 0u64;
    let a720 = {
        let a718 = v0.clone();
        length_bytes(a718)
    };
    if nat_lt(a719, a720) {
        let a724 = {
            let (h721, _) = f172(Fn20::F173(v0.clone()), {
                let a722 = v0.clone();
                length_bytes(a722)
            }, 0u64)?;
            h721
        };
        let a725 = {
            let a723 = v0;
            length_bytes(a723)
        };
        Ok(nat_eq(a724, a725))
    } else {
        Ok(false)
    }
}

pub fn f109() -> List<Bytes> {
    List::cons(f16(Str::lit("abstract")), List::cons(f16(Str::lit("as")), List::cons(f16(Str::lit("async")), List::cons(f16(Str::lit("await")), List::cons(f16(Str::lit("become")), List::cons(f16(Str::lit("box")), List::cons(f16(Str::lit("break")), List::cons(f16(Str::lit("const")), List::cons(f16(Str::lit("continue")), List::cons(f16(Str::lit("crate")), List::cons(f16(Str::lit("do")), List::cons(f16(Str::lit("dyn")), List::cons(f16(Str::lit("else")), List::cons(f16(Str::lit("enum")), List::cons(f16(Str::lit("extern")), List::cons(f16(Str::lit("false")), List::cons(f16(Str::lit("final")), List::cons(f16(Str::lit("fn")), List::cons(f16(Str::lit("for")), List::cons(f16(Str::lit("if")), List::cons(f16(Str::lit("impl")), List::cons(f16(Str::lit("in")), List::cons(f16(Str::lit("let")), List::cons(f16(Str::lit("loop")), List::cons(f16(Str::lit("macro")), List::cons(f16(Str::lit("match")), List::cons(f16(Str::lit("mod")), List::cons(f16(Str::lit("move")), List::cons(f16(Str::lit("mut")), List::cons(f16(Str::lit("override")), List::cons(f16(Str::lit("priv")), List::cons(f16(Str::lit("pub")), List::cons(f16(Str::lit("ref")), List::cons(f16(Str::lit("return")), List::cons(f16(Str::lit("self")), List::cons(f16(Str::lit("Self")), List::cons(f16(Str::lit("static")), List::cons(f16(Str::lit("struct")), List::cons(f16(Str::lit("super")), List::cons(f16(Str::lit("trait")), List::cons(f16(Str::lit("true")), List::cons(f16(Str::lit("try")), List::cons(f16(Str::lit("type")), List::cons(f16(Str::lit("typeof")), List::cons(f16(Str::lit("unsafe")), List::cons(f16(Str::lit("unsized")), List::cons(f16(Str::lit("use")), List::cons(f16(Str::lit("virtual")), List::cons(f16(Str::lit("where")), List::cons(f16(Str::lit("while")), List::cons(f16(Str::lit("yield")), List::cons(f16(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f110(v0: Fn11, p729: Option<Bytes>, p730: List<Bytes>) -> R<Option<Bytes>> {
    let m728 = (p729, p730);
    {
        let mut m728 = m728;
        loop {
            let step = {
                let (v1, v2) = m728;
                let m726 = v2;
                match m726.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c727: Fn11 = v0.clone();
                            c727.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m731 = v1;
    match m731 {
        None => {
            if if f40(v2.clone(), f16(Str::lit("Clone"))) {
                true
            } else {
                if f40(v2.clone(), f16(Str::lit("Copy"))) {
                    true
                } else {
                    if f40(v2.clone(), f16(Str::lit("Debug"))) {
                        true
                    } else {
                        if f40(v2.clone(), f16(Str::lit("Eq"))) {
                            true
                        } else {
                            f40(v2.clone(), f16(Str::lit("PartialEq")))
                        }
                    }
                }
            } {
                None::<Bytes>
            } else {
                Some(f19(f16(Str::lit("runtime support: derive: ")), f19(f16(v0), f19(f16(Str::lit(" derives ")), f19(f20(v2), f16(Str::lit(", which is not in the closed table")))))))
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f112(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m732 = v1;
    match m732 {
        None => {
            f48(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f113(v0: Fn12, p736: Adt14, p737: List<Adt5>) -> Adt14 {
    let m735 = (p736, p737);
    {
        let mut m735 = m735;
        loop {
            let step = {
                let (v1, v2) = m735;
                let m733 = v2;
                match m733.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c734: Fn12 = v0.clone();
                            c734.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h738, _, _, _, _, _, _, _, _, _) => {
            h738
        }
    }, match v0.clone() {
        Adt14::C0(_, h739, _, _, _, _, _, _, _, _) => {
            h739
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h740, _, _, _, _, _, _, _) => {
            h740
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h741, _, _, _, _, _, _) => {
            h741
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h742, _, _, _, _, _) => {
            h742
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h743, _, _, _, _) => {
            h743
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h744, _, _, _) => {
            h744
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h745, _, _) => {
            h745
        }
    }, {
        let a750 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h746, _) => {
                h746
            }
        };
        let a751 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h747, _, _, _, _) => {
                h747
            }
        }, {
            let a749 = match v1 {
                Adt5::C0(_, _, _, h748, _) => {
                    h748
                }
            };
            length_list(a749)
        }), List::<Adt13>::nil());
        append_list(a750, a751)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h752) => {
            h752
        }
    })
}

pub fn f115(v0: Adt2) -> Option<Bytes> {
    let m753 = v0;
    match m753 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            None::<Bytes>
        }
        Adt2::C7(_) => {
            None::<Bytes>
        }
        Adt2::C8(_) => {
            None::<Bytes>
        }
        Adt2::C9(_, _) => {
            None::<Bytes>
        }
        Adt2::C10(v6, _) => {
            Some(v6)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(_) => {
            None::<Bytes>
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f116(v0: Adt14, v1: Bytes, v2: List<Adt8>) -> Adt14 {
    f117(Fn13::F174(v1), v0, v2)
}

pub fn f117(v0: Fn13, p757: Adt14, p758: List<Adt8>) -> Adt14 {
    let m756 = (p757, p758);
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v1, v2) = m756;
                let m754 = v2;
                match m754.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c755: Fn13 = v0.clone();
                            c755.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h759, _, _, _, _, _, _, _, _, _) => {
            h759
        }
    }, match v0.clone() {
        Adt14::C0(_, h760, _, _, _, _, _, _, _, _) => {
            h760
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h761, _, _, _, _, _, _, _) => {
            h761
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h762, _, _, _, _, _, _) => {
            h762
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h763, _, _, _, _, _) => {
            h763
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h764, _, _, _, _) => {
            h764
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h765, _, _, _) => {
            h765
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h766, _, _) => {
            h766
        }
    }, {
        let a775 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h767, _) => {
                h767
            }
        };
        let a776 = List::cons(Adt13::C0({
            let m770 = match v1.clone() {
                Adt8::C0(h769, _, _, _) => {
                    (*h769).clone()
                }
            };
            match m770 {
                Adt5::C0(h768, _, _, _, _) => {
                    h768
                }
            }
        }, {
            let a774 = {
                let m773 = match v1 {
                    Adt8::C0(h772, _, _, _) => {
                        (*h772).clone()
                    }
                };
                match m773 {
                    Adt5::C0(_, _, _, h771, _) => {
                        h771
                    }
                }
            };
            length_list(a774)
        }), List::<Adt13>::nil());
        append_list(a775, a776)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h777) => {
            h777
        }
    })
}

pub fn f119(v0: u64) -> R<Bytes> {
    let v1: Bytes = f175(Str::lit("0"));
    let v2: Bytes = f176({
        let a814 = f177(v0, 10000000000000000000u64);
        let a815 = {
            let a812 = f177(v0, 1000000000000000000u64);
            let a813 = {
                let a810 = f177(v0, 100000000000000000u64);
                let a811 = {
                    let a808 = f177(v0, 10000000000000000u64);
                    let a809 = {
                        let a806 = f177(v0, 1000000000000000u64);
                        let a807 = {
                            let a804 = f177(v0, 100000000000000u64);
                            let a805 = {
                                let a802 = f177(v0, 10000000000000u64);
                                let a803 = {
                                    let a800 = f177(v0, 1000000000000u64);
                                    let a801 = {
                                        let a798 = f177(v0, 100000000000u64);
                                        let a799 = {
                                            let a796 = f177(v0, 10000000000u64);
                                            let a797 = {
                                                let a794 = f177(v0, 1000000000u64);
                                                let a795 = {
                                                    let a792 = f177(v0, 100000000u64);
                                                    let a793 = {
                                                        let a790 = f177(v0, 10000000u64);
                                                        let a791 = {
                                                            let a788 = f177(v0, 1000000u64);
                                                            let a789 = {
                                                                let a786 = f177(v0, 100000u64);
                                                                let a787 = {
                                                                    let a784 = f177(v0, 10000u64);
                                                                    let a785 = {
                                                                        let a782 = f177(v0, 1000u64);
                                                                        let a783 = {
                                                                            let a780 = f177(v0, 100u64);
                                                                            let a781 = {
                                                                                let a778 = f177(v0, 10u64);
                                                                                let a779 = f177(v0, 1u64);
                                                                                append_bytes(a778, a779)
                                                                            };
                                                                            append_bytes(a780, a781)
                                                                        };
                                                                        append_bytes(a782, a783)
                                                                    };
                                                                    append_bytes(a784, a785)
                                                                };
                                                                append_bytes(a786, a787)
                                                            };
                                                            append_bytes(a788, a789)
                                                        };
                                                        append_bytes(a790, a791)
                                                    };
                                                    append_bytes(a792, a793)
                                                };
                                                append_bytes(a794, a795)
                                            };
                                            append_bytes(a796, a797)
                                        };
                                        append_bytes(a798, a799)
                                    };
                                    append_bytes(a800, a801)
                                };
                                append_bytes(a802, a803)
                            };
                            append_bytes(a804, a805)
                        };
                        append_bytes(a806, a807)
                    };
                    append_bytes(a808, a809)
                };
                append_bytes(a810, a811)
            };
            append_bytes(a812, a813)
        };
        append_bytes(a814, a815)
    })?;
    let a816 = v0;
    let a817 = 0u64;
    if nat_eq(a816, a817) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f120(v0: Fn14, p821: Adt18, p822: List<Adt9>) -> R<Adt18> {
    let m820 = (p821, p822);
    {
        let mut m820 = m820;
        loop {
            let step = {
                let (v1, v2) = m820;
                let m818 = v2;
                match m818.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt9>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt9>), R<Adt18>>(({
                            let c819: Fn14 = v0.clone();
                            c819.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, v1: Adt18, v2: Adt9) -> R<Adt18> {
    let m824 = match v1.clone() {
        Adt18::C0(_, h823) => {
            h823
        }
    };
    match m824 {
        None => {
            let m825 = f48(Str::lit("a variant name"), f178(v2.clone()))?;
            match m825 {
                None => {
                    let m828 = {
                        let m827 = v2.clone();
                        match m827 {
                            Adt9::C0(_) => {
                                None::<Bytes>
                            }
                            Adt9::C1(_, _) => {
                                None::<Bytes>
                            }
                            Adt9::C2(_, v9, _) => {
                                f52(Str::lit("a record variant field name"), v9)?
                            }
                        }
                    };
                    match m828 {
                        None => {
                            if f66(match v1.clone() {
                                Adt18::C0(h830, _) => {
                                    h830
                                }
                            }, f178(v2.clone()))? {
                                Ok(Adt18::C0(match v1 {
                                    Adt18::C0(h831, _) => {
                                        h831
                                    }
                                }, Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(v0), f19(f16(Str::lit(" declares ")), f19(f20(f178(v2)), f16(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt18::C0(List::cons(f178(v2), match v1 {
                                    Adt18::C0(h832, _) => {
                                        h832
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt18::C0(match v1 {
                                Adt18::C0(h829, _) => {
                                    h829
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C0(match v1 {
                        Adt18::C0(h826, _) => {
                            h826
                        }
                    }, Some(v4)))
                }
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f122(v0: Adt18, v1: Adt9) -> R<Adt18> {
    let m834 = match v0.clone() {
        Adt18::C0(_, h833) => {
            h833
        }
    };
    match m834 {
        None => {
            if f66(match v0.clone() {
                Adt18::C0(h835, _) => {
                    h835
                }
            }, f178(v1.clone()))? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h836, _) => {
                        h836
                    }
                }, Some(f178(v1))))
            } else {
                Ok(Adt18::C0(List::cons(f178(v1), match v0 {
                    Adt18::C0(h837, _) => {
                        h837
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f123(v0: Bytes) -> R<Option<Bytes>> {
    if f66(List::cons(f16(Str::lit("fs")), List::cons(f16(Str::lit("Metadata")), List::cons(f16(Str::lit("MetadataExt")), List::<Bytes>::nil()))), v0.clone())? {
        Ok(Some(f16(Str::lit("files"))))
    } else {
        let a838 = v0.clone();
        let a839 = f16(Str::lit("File"));
        if equal(a838, a839) {
            Ok(Some(f16(Str::lit("files` or `process"))))
        } else {
            if f66(List::cons(f16(Str::lit("Command")), List::cons(f16(Str::lit("CommandExt")), List::cons(f16(Str::lit("Stdio")), List::cons(f16(Str::lit("Child")), List::cons(f16(Str::lit("Instant")), List::cons(f16(Str::lit("Duration")), List::cons(f16(Str::lit("thread")), List::<Bytes>::nil()))))))), v0)? {
                Ok(Some(f16(Str::lit("process"))))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f124(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f179(Fn21::F180(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h840, _, _, _, _, _, _) => {
            h840
        }
    })
}

pub fn f125(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f181(Fn22::F182(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h841, _, _, _, _, _) => {
            h841
        }
    })
}

pub fn f126(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f183(Fn23::F184(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h842, _, _, _, _) => {
            h842
        }
    })
}

pub fn f127(v0: Bytes) -> R<Option<u64>> {
    let a843 = v0.clone();
    let a844 = f16(Str::lit("Result"));
    if equal(a843, a844) {
        Ok(Some(2u64))
    } else {
        if f66(List::cons(f16(Str::lit("Option")), List::cons(f16(Str::lit("Rc")), List::cons(f16(Str::lit("Vec")), List::<Bytes>::nil()))), v0.clone())? {
            Ok(Some(1u64))
        } else {
            if f66(List::cons(f16(Str::lit("AtomicU64")), List::cons(f16(Str::lit("File")), List::cons(f16(Str::lit("Metadata")), List::cons(f16(Str::lit("Command")), List::cons(f16(Str::lit("Child")), List::cons(f16(Str::lit("Instant")), List::cons(f16(Str::lit("Error")), List::cons(f16(Str::lit("String")), List::<Bytes>::nil())))))))), v0)? {
                Ok(Some(0u64))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f128(v0: Bytes) -> bool {
    let a845 = v0.clone();
    let a846 = f16(Str::lit("Rc"));
    if equal(a845, a846) {
        true
    } else {
        let a847 = v0.clone();
        let a848 = f16(Str::lit("Vec"));
        if equal(a847, a848) {
            true
        } else {
            let a849 = v0;
            let a850 = f16(Str::lit("String"));
            equal(a849, a850)
        }
    }
}

pub fn f129(v0: Fn15, p854: Option<Adt17>, p855: List<Adt17>) -> Option<Adt17> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v1, v2) = m853;
                let m851 = v2;
                match m851.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(({
                            let c852: Fn15 = v0.clone();
                            c852.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Bytes, v1: Option<Adt17>, v2: Adt17) -> Option<Adt17> {
    let m856 = v1;
    match m856 {
        None => {
            if f40(match v2.clone() {
                Adt17::C0(h857, _, _) => {
                    h857
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt17>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f131(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f40(v2, v0)
    }
}

pub fn f132(v0: List<Bytes>) -> Bytes {
    let m858 = v0;
    match m858.uncons() {
        None => {
            f16(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f133(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f132(v1.clone());
    let m859 = f125(v0.clone(), v2.clone());
    match m859 {
        None => {
            if if f66(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h863) => {
                    h863
                }
            }, v2.clone())? {
                true
            } else {
                let a865 = {
                    let a864 = v1.clone();
                    length_list(a864)
                };
                let a866 = 1u64;
                if if nat_eq(a865, a866) {
                    let m867 = f187(v0.clone(), v2.clone());
                    match m867 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                } else {
                    false
                } {
                    true
                } else {
                    let m868 = f124(v0.clone(), v2.clone());
                    if match m868 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    } {
                        true
                    } else {
                        if f66(match v0 {
                            Adt14::C0(_, _, _, _, _, _, h869, _, _, _) => {
                                h869
                            }
                        }, v2.clone())? {
                            true
                        } else {
                            if f188(v2.clone()) {
                                true
                            } else {
                                if f40(v2.clone(), f16(Str::lit("None"))) {
                                    true
                                } else {
                                    if f40(v2.clone(), f16(Str::lit("Self"))) {
                                        true
                                    } else {
                                        f40(v2.clone(), f16(Str::lit("usize")))
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: path-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" starts at ")), f19(f20(v2), f16(Str::lit(", which no `use` or item declares"))))))))
            }
        }
        Some(v3) => {
            let m860 = f185(v1.clone(), 1u64);
            match m860 {
                None => {
                    Ok(Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is not a variant of ")), f20(v2))))))
                }
                Some(v4) => {
                    let m862 = f186(match v3 {
                        Adt12::C0(_, h861, _) => {
                            h861
                        }
                    }, v4);
                    match m862 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: variant-name: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is not a variant of ")), f20(v2))))))
                        }
                        Some(_) => {
                            Ok(None::<Bytes>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f134(v0: Adt14, v1: List<Bytes>) -> R<bool> {
    if match v0 {
        Adt14::C0(h870, _, _, _, _, _, _, _, _, _) => {
            h870
        }
    } {
        let m871 = v1;
        match m871.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, v3)) => {
                let m872 = v3;
                match m872.uncons() {
                    None => {
                        Ok(false)
                    }
                    Some((v4, v5)) => {
                        let m873 = v5;
                        match m873.uncons() {
                            None => {
                                f189(v2, v4)
                            }
                            Some((_, _)) => {
                                Ok(false)
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f135(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m874 = v1;
    match m874.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m875 = v3;
            match m875.uncons() {
                None => {
                    let m876 = f187(v0.clone(), v2.clone());
                    match m876 {
                        None => {
                            let m878 = f190(v2.clone());
                            match m878 {
                                None => {
                                    Ok(f191(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h877) => {
                                    h877
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m879 = v7;
                    match m879.uncons() {
                        None => {
                            let m880 = f192(v2.clone(), v6.clone())?;
                            match m880 {
                                None => {
                                    let m881 = f193(v0.clone(), v2.clone(), v6.clone())?;
                                    match m881 {
                                        None => {
                                            Ok(f194(v0, v2, v6))
                                        }
                                        Some(v9) => {
                                            Ok(Some(v9))
                                        }
                                    }
                                }
                                Some(v8) => {
                                    Ok(Some(v8))
                                }
                            }
                        }
                        Some((_, _)) => {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f136(v0: Bytes) -> R<Option<u64>> {
    if f66(List::cons(f16(Str::lit("fetch_add")), List::cons(f16(Str::lit("env")), List::<Bytes>::nil())), v0.clone())? {
        Ok(Some(2u64))
    } else {
        if f66(List::cons(f16(Str::lit("load")), List::cons(f16(Str::lit("cmp")), List::cons(f16(Str::lit("min")), List::cons(f16(Str::lit("max")), List::cons(f16(Str::lit("push")), List::cons(f16(Str::lit("truncate")), List::cons(f16(Str::lit("push_str")), List::cons(f16(Str::lit("extend_from_slice")), List::cons(f16(Str::lit("get")), List::cons(f16(Str::lit("strip_prefix")), List::cons(f16(Str::lit("starts_with")), List::cons(f16(Str::lit("split")), List::cons(f16(Str::lit("contains")), List::cons(f16(Str::lit("skip")), List::cons(f16(Str::lit("saturating_add")), List::cons(f16(Str::lit("arg")), List::cons(f16(Str::lit("current_dir")), List::cons(f16(Str::lit("stdin")), List::cons(f16(Str::lit("stdout")), List::cons(f16(Str::lit("stderr")), List::cons(f16(Str::lit("process_group")), List::cons(f16(Str::lit("stack_size")), List::cons(f16(Str::lit("spawn")), List::cons(f16(Str::lit("write_all")), List::<Bytes>::nil())))))))))))))))))))))))), v0.clone())? {
            Ok(Some(1u64))
        } else {
            if f66(List::cons(f16(Str::lit("len")), List::cons(f16(Str::lit("is_empty")), List::cons(f16(Str::lit("chars")), List::cons(f16(Str::lit("next")), List::cons(f16(Str::lit("pop")), List::cons(f16(Str::lit("iter")), List::cons(f16(Str::lit("rev")), List::cons(f16(Str::lit("copied")), List::cons(f16(Str::lit("ok")), List::cons(f16(Str::lit("clone")), List::cons(f16(Str::lit("to_string")), List::cons(f16(Str::lit("as_bytes")), List::cons(f16(Str::lit("collect")), List::cons(f16(Str::lit("is_ascii_digit")), List::cons(f16(Str::lit("kind")), List::cons(f16(Str::lit("file_name")), List::cons(f16(Str::lit("into_encoded_bytes")), List::cons(f16(Str::lit("into_os_string")), List::cons(f16(Str::lit("is_symlink")), List::cons(f16(Str::lit("is_file")), List::cons(f16(Str::lit("is_dir")), List::cons(f16(Str::lit("sort")), List::cons(f16(Str::lit("elapsed")), List::cons(f16(Str::lit("as_millis")), List::cons(f16(Str::lit("try_wait")), List::cons(f16(Str::lit("kill")), List::cons(f16(Str::lit("wait")), List::cons(f16(Str::lit("id")), List::cons(f16(Str::lit("code")), List::cons(f16(Str::lit("env_clear")), List::cons(f16(Str::lit("status")), List::cons(f16(Str::lit("join")), List::cons(f16(Str::lit("flush")), List::cons(f16(Str::lit("is_terminal")), List::cons(f16(Str::lit("lock")), List::cons(f16(Str::lit("dev")), List::cons(f16(Str::lit("ino")), List::cons(f16(Str::lit("text")), List::cons(f16(Str::lit("octets")), List::cons(f16(Str::lit("uncons")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))), v0)? {
                Ok(Some(0u64))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f137(v0: Bytes) -> R<bool> {
    f66(List::cons(f16(Str::lit("push")), List::cons(f16(Str::lit("pop")), List::cons(f16(Str::lit("truncate")), List::cons(f16(Str::lit("push_str")), List::cons(f16(Str::lit("extend_from_slice")), List::cons(f16(Str::lit("to_string")), List::cons(f16(Str::lit("collect")), List::cons(f16(Str::lit("sort")), List::<Bytes>::nil())))))))), v0)
}

pub fn f138(v0: Adt14, v1: Bytes) -> List<u64> {
    f195(Fn24::F196(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h882, _) => {
            h882
        }
    })
}

pub fn f139(v0: List<u64>, v1: u64) -> bool {
    f197(Fn25::F198(v1), false, v0)
}

pub fn f140(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m883 = f199(f200(v1.clone(), v2));
    match m883 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            Ok(None::<Bytes>)
        }
        Adt2::C6(v4) => {
            if f201(v0, f202(match v1 {
                Adt16::C0(h884, _, _, _) => {
                    h884
                }
            }, v4), v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: method-bound: ")), f19(f20(v3), f16(Str::lit(" is called on a parameter whose bounds do not provide it"))))))
            }
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(_, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f141(v0: List<u64>) -> R<Bytes> {
    f203(Fn26::F204, f16(Str::lit("")), v0)
}

pub fn f142(v0: Adt3) -> R<bool> {
    let m885 = v0;
    match m885 {
        Adt3::C0(_) => {
            Ok(false)
        }
        Adt3::C1(_) => {
            Ok(false)
        }
        Adt3::C2(_) => {
            Ok(false)
        }
        Adt3::C3(_, _) => {
            Ok(false)
        }
        Adt3::C4(r916, _, _) => {
            let v6 = (*r916).clone();
            let a886 = 13u64;
            let a887 = f205(v6.clone());
            if nat_le(a886, a887) {
                f142(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r917, _) => {
            let v9 = (*r917).clone();
            let a888 = 13u64;
            let a889 = f205(v9.clone());
            if nat_le(a888, a889) {
                f142(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r918, _) => {
            let v11 = (*r918).clone();
            let a890 = 13u64;
            let a891 = f205(v11.clone());
            if nat_le(a890, a891) {
                f142(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r919) => {
            let v13 = (*r919).clone();
            let a892 = 12u64;
            let a893 = f205(v13.clone());
            if nat_le(a892, a893) {
                f142(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r920, r921) => {
            let v14 = (*r920).clone();
            let v15 = (*r921).clone();
            let a894 = 6u64;
            let a895 = f205(v14.clone());
            if if nat_le(a894, a895) {
                f142(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a896 = 6u64;
                let a897 = f205(v15.clone());
                if nat_le(a896, a897) {
                    f142(v15)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C9(_, _) => {
            Ok(false)
        }
        Adt3::C10(_, _, _) => {
            Ok(false)
        }
        Adt3::C11(r922, r923) => {
            let v21 = (*r922).clone();
            let v22 = (*r923).clone();
            if f142(v21)? {
                Ok(true)
            } else {
                f142(v22)
            }
        }
        Adt3::C12(_, _, _) => {
            Ok(false)
        }
        Adt3::C13(_, _) => {
            Ok(false)
        }
        Adt3::C14(_, _, _) => {
            Ok(false)
        }
        Adt3::C15(_) => {
            Ok(false)
        }
        Adt3::C16(_, _, _) => {
            Ok(false)
        }
        Adt3::C17 => {
            Ok(false)
        }
        Adt3::C18(r924) => {
            let v35 = (*r924).clone();
            f142(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r925, r926) => {
            let v41 = (*r925).clone();
            let v42 = (*r926).clone();
            if if f206(v40, v41.clone()) {
                false
            } else {
                f142(v41)?
            } {
                Ok(true)
            } else {
                let a900 = {
                    let a898 = f207(v40);
                    let a899 = 1u64;
                    nat_add(a898, a899)?
                };
                let a901 = f205(v42.clone());
                if nat_le(a900, a901) {
                    f142(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r927) => {
            let v43 = (*r927).clone();
            let a902 = 12u64;
            let a903 = f205(v43.clone());
            if nat_le(a902, a903) {
                f142(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r928, _) => {
            let v44 = (*r928).clone();
            let a904 = 11u64;
            let a905 = f205(v44.clone());
            if nat_le(a904, a905) {
                f142(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r929, _) => {
            let v46 = (*r929).clone();
            let a906 = 13u64;
            let a907 = f205(v46.clone());
            if nat_le(a906, a907) {
                f142(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r930, _, _) => {
            let v48 = (*r930).clone();
            let a908 = 13u64;
            let a909 = f205(v48.clone());
            if nat_le(a908, a909) {
                f142(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r931) => {
            let v51 = (*r931).clone();
            let a910 = 12u64;
            let a911 = f205(v51.clone());
            if nat_le(a910, a911) {
                f142(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r932) => {
            let v52 = (*r932).clone();
            let a912 = 12u64;
            let a913 = f205(v52.clone());
            if nat_le(a912, a913) {
                f142(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r933, _, _, _) => {
            let v53 = (*r933).clone();
            let a914 = 13u64;
            let a915 = f205(v53.clone());
            if nat_le(a914, a915) {
                f142(v53)
            } else {
                Ok(false)
            }
        }
        Adt3::C29(_, _) => {
            Ok(false)
        }
        Adt3::C30 => {
            Ok(false)
        }
        Adt3::C31 => {
            Ok(false)
        }
        Adt3::C32(_, _) => {
            Ok(false)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(false)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(false)
        }
    }
}

pub fn f143(v0: Adt2) -> bool {
    let m934 = v0;
    match m934 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            true
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f144(v0: List<Bytes>) -> bool {
    let m935 = v0;
    match m935.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m936 = v2;
            match m936.uncons() {
                None => {
                    let m937 = f190(v1);
                    match m937 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some((_, _)) => {
                    false
                }
            }
        }
    }
}

pub fn f145(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m938 = f185(v1.clone(), 1u64);
    match m938 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m939 = f125(v0, f132(v1.clone()));
            match m939 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m941 = f186(match v4 {
                        Adt12::C0(_, h940, _) => {
                            h940
                        }
                    }, v3);
                    match m941 {
                        None => {
                            Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is matched with ")), f19(f59(f146(v2)?)?, f16(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a942 = f208(v5)?;
                            let a943 = f146(v2.clone())?;
                            if nat_eq(a942, a943) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" is matched with ")), f19(f59(f146(v2)?)?, f16(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f146(v0: Adt4) -> R<u64> {
    let m944 = v0;
    match m944 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(_) => {
            Ok(0u64)
        }
        Adt4::C2(_) => {
            Ok(0u64)
        }
        Adt4::C3(_) => {
            Ok(0u64)
        }
        Adt4::C4(_) => {
            Ok(0u64)
        }
        Adt4::C5(_, _) => {
            Ok(0u64)
        }
        Adt4::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt4::C7(_) => {
            Ok(0u64)
        }
        Adt4::C8 => {
            Ok(0u64)
        }
        Adt4::C9(_, r947) => {
            let v12 = (*r947).clone();
            let a945 = f146(v12)?;
            let a946 = 1u64;
            nat_add(a945, a946)
        }
    }
}

pub fn f147(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m948 = f82(v0, v1.clone());
    match m948 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f110(Fn11::F209(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f148(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m949 = f186(v0, v1);
    match m949 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m950 = v2;
            match m950 {
                Adt9::C0(_) => {
                    None::<List<Bytes>>
                }
                Adt9::C1(_, _) => {
                    None::<List<Bytes>>
                }
                Adt9::C2(_, v7, _) => {
                    Some(v7)
                }
            }
        }
    }
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m951 = f110(Fn11::F210(v1), None::<Bytes>, v0)?;
    match m951 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f150(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a952 = f211(v1, v3.clone())?;
        let a953 = f211(v0, v3)?;
        Ok(nat_eq(a952, a953))
    } else {
        Ok(false)
    }
}

pub fn f151(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f212(Fn27::F213(v3, v2, std::rc::Rc::new(v0)), None::<Bytes>, v1)
}

pub fn f152(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m954 = v2;
    match m954 {
        None => {
            let m955 = f153(v1, v3.clone());
            if match m955 {
                None => {
                    let m956 = f154(v3.clone());
                    match m956 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some(_) => {
                    true
                }
            } {
                None::<Bytes>
            } else {
                Some(f19(f16(Str::lit("runtime support: bound-trait: ")), f19(f20(f19(match v0 {
                    Adt7::C0(h957, _) => {
                        h957
                    }
                }, f19(f16(Str::lit(": ")), v3))), f16(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f153(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f214(Fn28::F215(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h958, _, _, _, _, _, _, _) => {
            h958
        }
    })
}

pub fn f154(v0: Bytes) -> Option<Bytes> {
    let a959 = v0.clone();
    let a960 = f16(Str::lit("Clone"));
    if equal(a959, a960) {
        Some(f16(Str::lit("clone")))
    } else {
        let a961 = v0;
        let a962 = f16(Str::lit("Drop"));
        if equal(a961, a962) {
            Some(f16(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f155(v0: Bytes) -> bool {
    let a963 = v0.clone();
    let a964 = f16(Str::lit("Clone"));
    if equal(a963, a964) {
        false
    } else {
        let a965 = v0;
        let a966 = f16(Str::lit("Drop"));
        equal(a965, a966)
    }
}

pub fn f156(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f216(Fn29::F217(v1), 0u64, v0)
}

pub fn f157(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f218(Fn30::F219(v1), None::<Adt8>, v0)
}

pub fn f158(v0: Adt5) -> R<Bytes> {
    Ok(f19(match v0.clone() {
        Adt5::C0(h967, _, _, _, _) => {
            h967
        }
    }, f19(f16(Str::lit("|")), f19(if match v0.clone() {
        Adt5::C0(_, h968, _, _, _) => {
            h968
        }
    } {
        f16(Str::lit("r"))
    } else {
        f16(Str::lit("-"))
    }, f19(if match v0.clone() {
        Adt5::C0(_, _, h969, _, _) => {
            h969
        }
    } {
        f16(Str::lit("x"))
    } else {
        f16(Str::lit("-"))
    }, f19(f16(Str::lit("|")), f19(f220(Fn31::F221, f16(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h970, _) => {
            h970
        }
    })?, f19(f16(Str::lit("|")), f222(match v0 {
        Adt5::C0(_, _, _, _, h971) => {
            (*h971).clone()
        }
    })?))))))))
}

pub fn f159(v0: List<Adt5>, v1: Bytes) -> bool {
    f223(Fn32::F224(v1), false, v0)
}

pub fn f160(v0: Result<List<Adt17>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt17>, Bytes> {
    f225(Fn33::F226(v2), v0, v1)
}

pub fn f161(v0: Fn16, p976: u64, p977: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m975 = (p976, p977);
    {
        let mut m975 = m975;
        loop {
            let step = {
                let (v1, v2) = m975;
                let m972 = v1;
                if m972 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m972 - 1;
                    let m974 = {
                        let c973: Fn16 = v0.clone();
                        c973.apply(v2.clone())?
                    };
                    match m974 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f227(v0, v1)
}

pub fn f163(v0: List<Bytes>) -> List<Bytes> {
    f228(Fn34::F229, List::<Bytes>::nil(), v0)
}

pub fn f164(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m981 = {
        let a978 = v0;
        let a979 = v1;
        let a980 = v2;
        slice_bytes(a978, a979, a980)
    };
    match m981 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f165(v0: Fn17, p985: u64, p986: List<Adt13>) -> R<u64> {
    let m984 = (p985, p986);
    {
        let mut m984 = m984;
        loop {
            let step = {
                let (v1, v2) = m984;
                let m982 = v2;
                match m982.uncons() {
                    None => {
                        Err::<(u64, List<Adt13>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt13>), R<u64>>(({
                            let c983: Fn17 = v0.clone();
                            c983.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Bytes, v1: u64, v2: Adt13) -> R<u64> {
    if f40(match v2 {
        Adt13::C0(h987, _) => {
            h987
        }
    }, v0) {
        let a988 = v1;
        let a989 = 1u64;
        nat_add(a988, a989)
    } else {
        Ok(v1)
    }
}

pub fn f167(v0: Fn18, p993: Option<u64>, p994: List<Adt13>) -> Option<u64> {
    let m992 = (p993, p994);
    {
        let mut m992 = m992;
        loop {
            let step = {
                let (v1, v2) = m992;
                let m990 = v2;
                match m990.uncons() {
                    None => {
                        Err::<(Option<u64>, List<Adt13>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<Adt13>), Option<u64>>(({
                            let c991: Fn18 = v0.clone();
                            c991.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: Option<u64>, v2: Adt13) -> Option<u64> {
    let m995 = v1;
    match m995 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h996, _) => {
                    h996
                }
            }, v0) {
                Some(match v2 {
                    Adt13::C0(_, h997) => {
                        h997
                    }
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f169(v0: Fn19, p1001: Bytes, p1002: List<Bytes>) -> Bytes {
    let m1000 = (p1001, p1002);
    {
        let mut m1000 = m1000;
        loop {
            let step = {
                let (v1, v2) = m1000;
                let m998 = v2;
                match m998.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c999: Fn19 = v0.clone();
                            c999.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f171(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m1003 = v1;
    match m1003 {
        None => {
            if if f40(v2.clone(), f16(Str::lit("Self"))) {
                true
            } else {
                if f40(v2.clone(), f16(Str::lit("self"))) {
                    true
                } else {
                    if f40(v2.clone(), f16(Str::lit("crate"))) {
                        true
                    } else {
                        f40(v2.clone(), f16(Str::lit("super")))
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                f48(v0, v2)
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f172(v0: Fn20, p1008: u64, p1009: u64) -> R<(u64, bool)> {
    let m1007 = (p1008, p1009);
    {
        let mut m1007 = m1007;
        loop {
            let step = {
                let (v1, v2) = m1007;
                let m1004 = v1;
                if m1004 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1004 - 1;
                    let m1006 = {
                        let c1005: Fn20 = v0.clone();
                        c1005.apply(v2)?
                    };
                    match m1006 {
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
                Ok(next) => { m1007 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(v0, v1) {
        Ok(Some({
            let a1010 = v1;
            let a1011 = 1u64;
            nat_add(a1010, a1011)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f174(v0: Bytes, v1: Adt14, v2: Adt8) -> Adt14 {
    let m1014 = match v2.clone() {
        Adt8::C0(h1013, _, _, _) => {
            (*h1013).clone()
        }
    };
    if match m1014 {
        Adt5::C0(_, h1012, _, _, _) => {
            h1012
        }
    } {
        v1
    } else {
        Adt14::C0(match v1.clone() {
            Adt14::C0(h1015, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        }, match v1.clone() {
            Adt14::C0(_, h1016, _, _, _, _, _, _, _, _) => {
                h1016
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h1017, _, _, _, _, _, _, _) => {
                h1017
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h1018, _, _, _, _, _, _) => {
                h1018
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h1019, _, _, _, _, _) => {
                h1019
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, h1020, _, _, _, _) => {
                h1020
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h1021, _, _, _) => {
                h1021
            }
        }, {
            let a1030 = match v1.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h1022, _, _) => {
                    h1022
                }
            };
            let a1031 = List::cons(Adt13::C0(f19(v0, f19(f16(Str::lit("::")), {
                let m1025 = match v2.clone() {
                    Adt8::C0(h1024, _, _, _) => {
                        (*h1024).clone()
                    }
                };
                match m1025 {
                    Adt5::C0(h1023, _, _, _, _) => {
                        h1023
                    }
                }
            })), {
                let a1029 = {
                    let m1028 = match v2 {
                        Adt8::C0(h1027, _, _, _) => {
                            (*h1027).clone()
                        }
                    };
                    match m1028 {
                        Adt5::C0(_, _, _, h1026, _) => {
                            h1026
                        }
                    }
                };
                length_list(a1029)
            }), List::<Adt13>::nil());
            append_list(a1030, a1031)
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h1032, _) => {
                h1032
            }
        }, match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h1033) => {
                h1033
            }
        })
    }
}

pub fn f175(v0: Str) -> Bytes {
    let a1034 = v0;
    utf8_encode(a1034)
}

pub fn f176(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h1035, _) = f172(Fn20::F231(v0.clone()), {
            let a1036 = v0.clone();
            length_bytes(a1036)
        }, 0u64)?;
        h1035
    };
    Ok(f164(v0.clone(), v1, {
        let a1038 = {
            let a1037 = v0;
            length_bytes(a1037)
        };
        let a1039 = v1;
        nat_sub(a1038, a1039)
    }))
}

pub fn f177(v0: u64, v1: u64) -> Bytes {
    f164(f175(Str::lit("0123456789")), {
        let a1043 = {
            let a1040 = v0;
            let a1041 = v1;
            let a1042 = 0u64;
            nat_quot(a1040, a1041, a1042)
        };
        let a1044 = 10u64;
        let a1045 = 0u64;
        nat_rem(a1043, a1044, a1045)
    }, 1u64)
}

pub fn f178(v0: Adt9) -> Bytes {
    let m1046 = v0;
    match m1046 {
        Adt9::C0(v1) => {
            v1
        }
        Adt9::C1(v2, _) => {
            v2
        }
        Adt9::C2(v4, _, _) => {
            v4
        }
    }
}

pub fn f179(v0: Fn21, p1050: Option<Adt11>, p1051: List<Adt11>) -> Option<Adt11> {
    let m1049 = (p1050, p1051);
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let (v1, v2) = m1049;
                let m1047 = v2;
                match m1047.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c1048: Fn21 = v0.clone();
                            c1048.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m1052 = v1;
    match m1052 {
        None => {
            if f40(match v2.clone() {
                Adt11::C0(h1053, _, _) => {
                    h1053
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt11>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f181(v0: Fn22, p1057: Option<Adt12>, p1058: List<Adt12>) -> Option<Adt12> {
    let m1056 = (p1057, p1058);
    {
        let mut m1056 = m1056;
        loop {
            let step = {
                let (v1, v2) = m1056;
                let m1054 = v2;
                match m1054.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c1055: Fn22 = v0.clone();
                            c1055.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m1059 = v1;
    match m1059 {
        None => {
            if f40(match v2.clone() {
                Adt12::C0(h1060, _, _) => {
                    h1060
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt12>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f183(v0: Fn23, p1064: Option<Adt13>, p1065: List<Adt13>) -> Option<Adt13> {
    let m1063 = (p1064, p1065);
    {
        let mut m1063 = m1063;
        loop {
            let step = {
                let (v1, v2) = m1063;
                let m1061 = v2;
                match m1061.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c1062: Fn23 = v0.clone();
                            c1062.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1066 = v1;
    match m1066 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h1067, _) => {
                    h1067
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f185(p1071: List<Bytes>, p1072: u64) -> Option<Bytes> {
    let m1070 = (p1071, p1072);
    {
        let mut m1070 = m1070;
        loop {
            let step = {
                let (v0, v1) = m1070;
                let m1068 = v0;
                match m1068.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1069 = v1;
                        if m1069 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m1069 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f232(Fn35::F233(v1), None::<Adt9>, v0)
}

pub fn f187(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f183(Fn23::F234(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h1073, _, _) => {
            h1073
        }
    })
}

pub fn f188(v0: Bytes) -> bool {
    let a1074 = v0.clone();
    let a1075 = f16(Str::lit("u8"));
    if equal(a1074, a1075) {
        true
    } else {
        let a1076 = v0.clone();
        let a1077 = f16(Str::lit("u16"));
        if equal(a1076, a1077) {
            true
        } else {
            let a1078 = v0.clone();
            let a1079 = f16(Str::lit("u32"));
            if equal(a1078, a1079) {
                true
            } else {
                let a1080 = v0.clone();
                let a1081 = f16(Str::lit("u64"));
                if equal(a1080, a1081) {
                    true
                } else {
                    let a1082 = v0.clone();
                    let a1083 = f16(Str::lit("u128"));
                    if equal(a1082, a1083) {
                        true
                    } else {
                        let a1084 = v0.clone();
                        let a1085 = f16(Str::lit("i8"));
                        if equal(a1084, a1085) {
                            true
                        } else {
                            let a1086 = v0.clone();
                            let a1087 = f16(Str::lit("i16"));
                            if equal(a1086, a1087) {
                                true
                            } else {
                                let a1088 = v0.clone();
                                let a1089 = f16(Str::lit("i32"));
                                if equal(a1088, a1089) {
                                    true
                                } else {
                                    let a1090 = v0.clone();
                                    let a1091 = f16(Str::lit("i64"));
                                    if equal(a1090, a1091) {
                                        true
                                    } else {
                                        let a1092 = v0;
                                        let a1093 = f16(Str::lit("i128"));
                                        equal(a1092, a1093)
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

pub fn f189(v0: Bytes, v1: Bytes) -> R<bool> {
    let m1094 = f235(v0.clone(), v1.clone())?;
    match m1094 {
        None => {
            if f188(v0) {
                f236(f16(Str::lit("usize")), v1)
            } else {
                Ok(false)
            }
        }
        Some(_) => {
            f236(v0, v1)
        }
    }
}

pub fn f190(v0: Bytes) -> Option<u64> {
    let a1095 = v0.clone();
    let a1096 = f16(Str::lit("Some"));
    if equal(a1095, a1096) {
        Some(1u64)
    } else {
        let a1097 = v0.clone();
        let a1098 = f16(Str::lit("Ok"));
        if equal(a1097, a1098) {
            Some(1u64)
        } else {
            let a1099 = v0;
            let a1100 = f16(Str::lit("Err"));
            if equal(a1099, a1100) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f191(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m1101 = f124(v0, v1);
    match m1101 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a1103 = match v2 {
                    Adt11::C0(_, h1102, _) => {
                        h1102
                    }
                };
                length_list(a1103)
            })
        }
    }
}

pub fn f192(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let m1104 = f235(v0.clone(), v1.clone())?;
    match m1104 {
        None => {
            if f188(v0) {
                f235(f16(Str::lit("usize")), v1)
            } else {
                Ok(None::<u64>)
            }
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f193(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m1105 = f125(v0, v1);
    match m1105 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f237(match v3 {
                Adt12::C0(_, h1106, _) => {
                    h1106
                }
            }, v2)
        }
    }
}

pub fn f194(v0: Adt14, v1: Bytes, v2: Bytes) -> Option<u64> {
    let m1107 = f187(v0, f19(v1, f19(f16(Str::lit("::")), v2)));
    match m1107 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            Some(match v3 {
                Adt13::C0(_, h1108) => {
                    h1108
                }
            })
        }
    }
}

pub fn f195(v0: Fn24, p1112: List<u64>, p1113: List<Adt13>) -> List<u64> {
    let m1111 = (p1112, p1113);
    {
        let mut m1111 = m1111;
        loop {
            let step = {
                let (v1, v2) = m1111;
                let m1109 = v2;
                match m1109.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1110: Fn24 = v0.clone();
                            c1110.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f40(match v2.clone() {
        Adt13::C0(h1114, _) => {
            h1114
        }
    }, v0) {
        let a1116 = v1;
        let a1117 = List::cons(match v2 {
            Adt13::C0(_, h1115) => {
                h1115
            }
        }, List::<u64>::nil());
        append_list(a1116, a1117)
    } else {
        v1
    }
}

pub fn f197(v0: Fn25, p1121: bool, p1122: List<u64>) -> bool {
    let m1120 = (p1121, p1122);
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let (v1, v2) = m1120;
                let m1118 = v2;
                match m1118.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1119: Fn25 = v0.clone();
                            c1119.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1123 = v2;
        let a1124 = v0;
        nat_eq(a1123, a1124)
    }
}

pub fn f199(p1135: Adt2) -> Adt2 {
    let m1134 = p1135;
    {
        let mut m1134 = m1134;
        loop {
            let step = {
                let v0 = m1134;
                let m1125 = v0;
                match m1125 {
                    Adt2::C0 => {
                        Err::<Adt2, Adt2>(Adt2::C0)
                    }
                    Adt2::C1 => {
                        Err::<Adt2, Adt2>(Adt2::C1)
                    }
                    Adt2::C2 => {
                        Err::<Adt2, Adt2>(Adt2::C2)
                    }
                    Adt2::C3 => {
                        Err::<Adt2, Adt2>(Adt2::C3)
                    }
                    Adt2::C4 => {
                        Err::<Adt2, Adt2>(Adt2::C4)
                    }
                    Adt2::C5 => {
                        Err::<Adt2, Adt2>(Adt2::C5)
                    }
                    Adt2::C6(v1) => {
                        Err::<Adt2, Adt2>(Adt2::C6(v1))
                    }
                    Adt2::C7(r1126) => {
                        let v2 = (*r1126).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1127) => {
                        let v3 = (*r1127).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1128, r1129) => {
                        let v4 = (*r1128).clone();
                        let v5 = (*r1129).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1130) => {
                        let v7 = (*r1130).clone();
                        Err::<Adt2, Adt2>(Adt2::C10(v6, std::rc::Rc::new(v7)))
                    }
                    Adt2::C11 => {
                        Err::<Adt2, Adt2>(Adt2::C11)
                    }
                    Adt2::C12 => {
                        Err::<Adt2, Adt2>(Adt2::C12)
                    }
                    Adt2::C13 => {
                        Err::<Adt2, Adt2>(Adt2::C13)
                    }
                    Adt2::C14 => {
                        Err::<Adt2, Adt2>(Adt2::C14)
                    }
                    Adt2::C15(r1131) => {
                        let v8 = (*r1131).clone();
                        Err::<Adt2, Adt2>(Adt2::C15(std::rc::Rc::new(v8)))
                    }
                    Adt2::C16 => {
                        Err::<Adt2, Adt2>(Adt2::C16)
                    }
                    Adt2::C17 => {
                        Err::<Adt2, Adt2>(Adt2::C17)
                    }
                    Adt2::C18 => {
                        Err::<Adt2, Adt2>(Adt2::C18)
                    }
                    Adt2::C19(r1132, r1133) => {
                        let v9 = (*r1132).clone();
                        let v10 = (*r1133).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
                    }
                }
            };
            match step {
                Ok(next) => { m1134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Adt16, v1: Adt3) -> Adt2 {
    let m1136 = v1;
    match m1136 {
        Adt3::C0(v2) => {
            let m1138 = f65(match v0 {
                Adt16::C0(_, h1137, _, _) => {
                    h1137
                }
            }, v2);
            match m1138 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt17::C0(_, _, h1139) => {
                            (*h1139).clone()
                        }
                    }
                }
            }
        }
        Adt3::C1(_) => {
            Adt2::C16
        }
        Adt3::C2(_) => {
            Adt2::C16
        }
        Adt3::C3(_, _) => {
            Adt2::C16
        }
        Adt3::C4(_, _, _) => {
            Adt2::C16
        }
        Adt3::C5(r1140, v12) => {
            let v11 = (*r1140).clone();
            f238(f239(v0.clone(), f199(f200(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1141) => {
            let v15 = (*r1141).clone();
            let v16: Adt2 = f200(v0, v15);
            if f240(v16.clone()) {
                Adt2::C16
            } else {
                Adt2::C7(std::rc::Rc::new(v16))
            }
        }
        Adt3::C8(_, _) => {
            Adt2::C16
        }
        Adt3::C9(_, _) => {
            Adt2::C16
        }
        Adt3::C10(_, _, _) => {
            Adt2::C16
        }
        Adt3::C11(_, _) => {
            Adt2::C16
        }
        Adt3::C12(_, _, _) => {
            Adt2::C16
        }
        Adt3::C13(_, _) => {
            Adt2::C16
        }
        Adt3::C14(_, _, _) => {
            Adt2::C16
        }
        Adt3::C15(_) => {
            Adt2::C16
        }
        Adt3::C16(_, _, _) => {
            Adt2::C16
        }
        Adt3::C17 => {
            Adt2::C16
        }
        Adt3::C18(_) => {
            Adt2::C16
        }
        Adt3::C19(_) => {
            Adt2::C16
        }
        Adt3::C20(_, _, _) => {
            Adt2::C16
        }
        Adt3::C21(_, _, _) => {
            Adt2::C16
        }
        Adt3::C22(_) => {
            Adt2::C16
        }
        Adt3::C23(_, _) => {
            Adt2::C16
        }
        Adt3::C24(_, _) => {
            Adt2::C16
        }
        Adt3::C25(_, _, _) => {
            Adt2::C16
        }
        Adt3::C26(_) => {
            Adt2::C16
        }
        Adt3::C27(_) => {
            Adt2::C16
        }
        Adt3::C28(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C29(_, _) => {
            Adt2::C16
        }
        Adt3::C30 => {
            Adt2::C16
        }
        Adt3::C31 => {
            Adt2::C16
        }
        Adt3::C32(_, _) => {
            Adt2::C16
        }
        Adt3::C33(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C34(_, _, _, _) => {
            Adt2::C16
        }
    }
}

pub fn f201(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f101(Fn10::F241(v2, std::rc::Rc::new(v0)), false, v1)
}

pub fn f202(p1146: List<Adt7>, p1147: u64) -> List<Bytes> {
    let m1145 = (p1146, p1147);
    {
        let mut m1145 = m1145;
        loop {
            let step = {
                let (v0, v1) = m1145;
                let m1142 = v0;
                match m1142.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1143 = v1;
                        if m1143 == 0 {
                            match v2 {
                                Adt7::C0(_, h1144) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1144)
                                }
                            }
                        } else {
                            let v4 = m1143 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Fn26, p1151: Bytes, p1152: List<u64>) -> R<Bytes> {
    let m1150 = (p1151, p1152);
    {
        let mut m1150 = m1150;
        loop {
            let step = {
                let (v1, v2) = m1150;
                let m1148 = v2;
                match m1148.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1149: Fn26 = v0.clone();
                            c1149.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1153 = v0.clone();
    let a1154 = f16(Str::lit(""));
    if equal(a1153, a1154) {
        f59(v1)
    } else {
        Ok(f19(v0, f19(f16(Str::lit("/")), f59(v1)?)))
    }
}

pub fn f205(v0: Adt3) -> u64 {
    let m1155 = v0;
    match m1155 {
        Adt3::C0(_) => {
            14u64
        }
        Adt3::C1(_) => {
            14u64
        }
        Adt3::C2(_) => {
            14u64
        }
        Adt3::C3(_, _) => {
            13u64
        }
        Adt3::C4(_, _, _) => {
            13u64
        }
        Adt3::C5(_, _) => {
            13u64
        }
        Adt3::C6(_, _) => {
            13u64
        }
        Adt3::C7(_) => {
            12u64
        }
        Adt3::C8(_, _) => {
            5u64
        }
        Adt3::C9(_, _) => {
            14u64
        }
        Adt3::C10(_, _, _) => {
            1u64
        }
        Adt3::C11(_, _) => {
            1u64
        }
        Adt3::C12(_, _, _) => {
            14u64
        }
        Adt3::C13(_, _) => {
            14u64
        }
        Adt3::C14(_, _, _) => {
            14u64
        }
        Adt3::C15(_) => {
            14u64
        }
        Adt3::C16(_, _, _) => {
            14u64
        }
        Adt3::C17 => {
            1u64
        }
        Adt3::C18(_) => {
            1u64
        }
        Adt3::C19(_) => {
            14u64
        }
        Adt3::C20(_, _, _) => {
            14u64
        }
        Adt3::C21(v40, _, _) => {
            f207(v40)
        }
        Adt3::C22(_) => {
            12u64
        }
        Adt3::C23(_, _) => {
            11u64
        }
        Adt3::C24(_, _) => {
            13u64
        }
        Adt3::C25(_, _, _) => {
            14u64
        }
        Adt3::C26(_) => {
            12u64
        }
        Adt3::C27(_) => {
            12u64
        }
        Adt3::C28(_, _, _, _) => {
            13u64
        }
        Adt3::C29(_, _) => {
            14u64
        }
        Adt3::C30 => {
            14u64
        }
        Adt3::C31 => {
            14u64
        }
        Adt3::C32(_, _) => {
            14u64
        }
        Adt3::C33(_, _, _, _) => {
            14u64
        }
        Adt3::C34(_, _, _, _) => {
            14u64
        }
    }
}

pub fn f206(v0: u64, v1: Adt3) -> bool {
    let a1156 = v0;
    let a1157 = 8u64;
    if if nat_eq(a1156, a1157) {
        f242(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1160 = f205(v1);
        let a1161 = {
            let a1158 = f207(v0);
            let a1159 = 5u64;
            if nat_eq(a1158, a1159) {
                6u64
            } else {
                f207(v0)
            }
        };
        nat_lt(a1160, a1161)
    }
}

pub fn f207(v0: u64) -> u64 {
    let a1162 = v0;
    let a1163 = 6u64;
    if nat_eq(a1162, a1163) {
        3u64
    } else {
        let a1164 = v0;
        let a1165 = 5u64;
        if nat_eq(a1164, a1165) {
            4u64
        } else {
            let a1166 = v0;
            let a1167 = 7u64;
            if nat_eq(a1166, a1167) {
                5u64
            } else {
                let a1168 = v0;
                let a1169 = 8u64;
                if nat_eq(a1168, a1169) {
                    5u64
                } else {
                    let a1170 = v0;
                    let a1171 = 9u64;
                    if nat_eq(a1170, a1171) {
                        5u64
                    } else {
                        let a1172 = v0;
                        let a1173 = 10u64;
                        if nat_eq(a1172, a1173) {
                            5u64
                        } else {
                            let a1174 = v0;
                            let a1175 = 11u64;
                            if nat_eq(a1174, a1175) {
                                5u64
                            } else {
                                let a1176 = v0;
                                let a1177 = 0u64;
                                if nat_eq(a1176, a1177) {
                                    9u64
                                } else {
                                    let a1178 = v0;
                                    let a1179 = 1u64;
                                    if nat_eq(a1178, a1179) {
                                        9u64
                                    } else {
                                        10u64
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

pub fn f208(v0: Adt9) -> R<u64> {
    let m1180 = v0;
    match m1180 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_, r1181) => {
            let v3 = (*r1181).clone();
            f58(v3)
        }
        Adt9::C2(_, _, r1182) => {
            let v6 = (*r1182).clone();
            f58(v6)
        }
    }
}

pub fn f209(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1183 = v2;
    match m1183 {
        None => {
            if f66(v0, v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f19(f16(Str::lit("runtime support: pattern-fields: ")), f19(f20(f83(v1, Str::lit("::"))?), f19(f16(Str::lit(" has no field ")), f20(v3))))))
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f210(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1184 = v1;
    match m1184 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1187 = {
                    let a1185 = v3;
                    let a1186 = v0;
                    append_bytes(a1185, a1186)
                };
                let a1188 = v2;
                append_bytes(a1187, a1188)
            })
        }
    }
}

pub fn f211(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f243(Fn36::F244(v1), 0u64, v0)
}

pub fn f212(v0: Fn27, p1192: Option<Bytes>, p1193: List<Adt6>) -> R<Option<Bytes>> {
    let m1191 = (p1192, p1193);
    {
        let mut m1191 = m1191;
        loop {
            let step = {
                let (v1, v2) = m1191;
                let m1189 = v2;
                match m1189.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1190: Fn27 = v0.clone();
                            c1190.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1194 = v3;
    match m1194 {
        None => {
            let m1196 = f48(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1195, _, _) => {
                    h1195
                }
            })?;
            match m1196 {
                None => {
                    f22(v2, match v4 {
                        Adt6::C0(_, _, h1197) => {
                            (*h1197).clone()
                        }
                    }, v1, v0)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f214(v0: Fn28, p1201: Option<Adt10>, p1202: List<Adt10>) -> Option<Adt10> {
    let m1200 = (p1201, p1202);
    {
        let mut m1200 = m1200;
        loop {
            let step = {
                let (v1, v2) = m1200;
                let m1198 = v2;
                match m1198.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1199: Fn28 = v0.clone();
                            c1199.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1203 = v1;
    match m1203 {
        None => {
            if f40(match v2.clone() {
                Adt10::C0(h1204, _) => {
                    h1204
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt10>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f216(v0: Fn29, p1208: u64, p1209: List<Adt8>) -> R<u64> {
    let m1207 = (p1208, p1209);
    {
        let mut m1207 = m1207;
        loop {
            let step = {
                let (v1, v2) = m1207;
                let m1205 = v2;
                match m1205.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1206: Fn29 = v0.clone();
                            c1206.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f40({
        let m1212 = match v2 {
            Adt8::C0(h1211, _, _, _) => {
                (*h1211).clone()
            }
        };
        match m1212 {
            Adt5::C0(h1210, _, _, _, _) => {
                h1210
            }
        }
    }, v0) {
        let a1213 = v1;
        let a1214 = 1u64;
        nat_add(a1213, a1214)
    } else {
        Ok(v1)
    }
}

pub fn f218(v0: Fn30, p1218: Option<Adt8>, p1219: List<Adt8>) -> Option<Adt8> {
    let m1217 = (p1218, p1219);
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let (v1, v2) = m1217;
                let m1215 = v2;
                match m1215.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1216: Fn30 = v0.clone();
                            c1216.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1220 = v1;
    match m1220 {
        None => {
            if f40({
                let m1223 = match v2.clone() {
                    Adt8::C0(h1222, _, _, _) => {
                        (*h1222).clone()
                    }
                };
                match m1223 {
                    Adt5::C0(h1221, _, _, _, _) => {
                        h1221
                    }
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt8>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f220(v0: Fn31, p1227: Bytes, p1228: List<Adt6>) -> R<Bytes> {
    let m1226 = (p1227, p1228);
    {
        let mut m1226 = m1226;
        loop {
            let step = {
                let (v1, v2) = m1226;
                let m1224 = v2;
                match m1224.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1225: Fn31 = v0.clone();
                            c1225.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f19(v0, f19(match v1.clone() {
        Adt6::C0(h1229, _, _) => {
            h1229
        }
    }, f19(if match v1.clone() {
        Adt6::C0(_, h1230, _) => {
            h1230
        }
    } {
        f16(Str::lit(":m:"))
    } else {
        f16(Str::lit(":-:"))
    }, f19(f222(match v1 {
        Adt6::C0(_, _, h1231) => {
            (*h1231).clone()
        }
    })?, f16(Str::lit(",")))))))
}

pub fn f222(v0: Adt2) -> R<Bytes> {
    let m1232 = v0;
    match m1232 {
        Adt2::C0 => {
            Ok(f16(Str::lit("U")))
        }
        Adt2::C1 => {
            Ok(f16(Str::lit("B")))
        }
        Adt2::C2 => {
            Ok(f16(Str::lit("N")))
        }
        Adt2::C3 => {
            Ok(f16(Str::lit("F")))
        }
        Adt2::C4 => {
            Ok(f16(Str::lit("O")))
        }
        Adt2::C5 => {
            Ok(f16(Str::lit("S")))
        }
        Adt2::C6(v1) => {
            Ok(f19(f16(Str::lit("P")), f19(f59(v1)?, f16(Str::lit(";")))))
        }
        Adt2::C7(r1233) => {
            let v2 = (*r1233).clone();
            Ok(f19(f16(Str::lit("R(")), f19(f222(v2)?, f16(Str::lit(")")))))
        }
        Adt2::C8(r1234) => {
            let v3 = (*r1234).clone();
            Ok(f19(f16(Str::lit("M(")), f19(f222(v3)?, f16(Str::lit(")")))))
        }
        Adt2::C9(r1235, r1236) => {
            let v4 = (*r1235).clone();
            let v5 = (*r1236).clone();
            Ok(f19(f16(Str::lit("T(")), f19(f222(v4)?, f19(f16(Str::lit(",")), f19(f222(v5)?, f16(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1237) => {
            let v7 = (*r1237).clone();
            Ok(f19(f16(Str::lit("D")), f19(f20(v6), f19(f16(Str::lit("(")), f19(f222(v7)?, f16(Str::lit(")")))))))
        }
        Adt2::C11 => {
            Ok(f16(Str::lit("W")))
        }
        Adt2::C12 => {
            Ok(f16(Str::lit("Z")))
        }
        Adt2::C13 => {
            Ok(f16(Str::lit("C")))
        }
        Adt2::C14 => {
            Ok(f16(Str::lit("Q")))
        }
        Adt2::C15(r1238) => {
            let v8 = (*r1238).clone();
            Ok(f19(f16(Str::lit("L(")), f19(f222(v8)?, f16(Str::lit(")")))))
        }
        Adt2::C16 => {
            Ok(f16(Str::lit("?")))
        }
        Adt2::C17 => {
            Ok(f16(Str::lit("-")))
        }
        Adt2::C18 => {
            Ok(f16(Str::lit("")))
        }
        Adt2::C19(r1239, r1240) => {
            let v9 = (*r1239).clone();
            let v10 = (*r1240).clone();
            Ok(f19(f222(v9)?, f19(f16(Str::lit(";")), f222(v10)?)))
        }
    }
}

pub fn f223(v0: Fn32, p1244: bool, p1245: List<Adt5>) -> bool {
    let m1243 = (p1244, p1245);
    {
        let mut m1243 = m1243;
        loop {
            let step = {
                let (v1, v2) = m1243;
                let m1241 = v2;
                match m1241.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1242: Fn32 = v0.clone();
                            c1242.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f40(match v2 {
            Adt5::C0(h1246, _, _, _, _) => {
                h1246
            }
        }, v0)
    }
}

pub fn f225(v0: Fn33, p1250: Result<List<Adt17>, Bytes>, p1251: List<Adt6>) -> Result<List<Adt17>, Bytes> {
    let m1249 = (p1250, p1251);
    {
        let mut m1249 = m1249;
        loop {
            let step = {
                let (v1, v2) = m1249;
                let m1247 = v2;
                match m1247.uncons() {
                    None => {
                        Err::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(({
                            let c1248: Fn33 = v0.clone();
                            c1248.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: Result<List<Adt17>, Bytes>, v2: Adt6) -> Result<List<Adt17>, Bytes> {
    let m1252 = v1;
    match m1252 {
        Ok(v4) => {
            let m1254 = f65(v4.clone(), match v2.clone() {
                Adt6::C0(h1253, _, _) => {
                    h1253
                }
            });
            match m1254 {
                None => {
                    Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(match v2.clone() {
                        Adt6::C0(h1256, _, _) => {
                            h1256
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h1257, _) => {
                            h1257
                        }
                    }, std::rc::Rc::new(match v2 {
                        Adt6::C0(_, _, h1258) => {
                            (*h1258).clone()
                        }
                    })), v4))
                }
                Some(_) => {
                    Err::<List<Adt17>, Bytes>(f19(f16(Str::lit("runtime support: param-duplicate: ")), f19(f20(match v2 {
                        Adt6::C0(h1255, _, _) => {
                            h1255
                        }
                    }), f19(f16(Str::lit(" names two parameters of ")), f20(v0)))))
                }
            }
        }
        Err(v3) => {
            Err::<List<Adt17>, Bytes>(v3)
        }
    }
}

pub fn f227(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1259, _) = v1.clone();
        h1259
    };
    let v3: u64 = {
        let (h1260, _) = {
            let (_, h1261) = v1.clone();
            h1261
        };
        h1260
    };
    let v4: List<Bytes> = {
        let (_, h1262) = {
            let (_, h1263) = v1;
            h1263
        };
        h1262
    };
    let m1264 = f245(v0.clone(), v2);
    match m1264 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1265 = v5;
            let a1266 = 58u8;
            if equal(a1265, a1266) {
                let m1269 = f245(v0.clone(), {
                    let a1267 = v2;
                    let a1268 = 1u64;
                    nat_add(a1267, a1268)?
                });
                match m1269 {
                    None => {
                        Ok(Some(({
                            let a1280 = v2;
                            let a1281 = 1u64;
                            nat_add(a1280, a1281)?
                        }, (v3, v4))))
                    }
                    Some(v6) => {
                        let a1270 = v6;
                        let a1271 = 58u8;
                        if equal(a1270, a1271) {
                            Ok(Some(({
                                let a1272 = v2;
                                let a1273 = 2u64;
                                nat_add(a1272, a1273)?
                            }, ({
                                let a1274 = v2;
                                let a1275 = 2u64;
                                nat_add(a1274, a1275)?
                            }, List::cons(f164(v0, v3, {
                                let a1276 = v2;
                                let a1277 = v3;
                                nat_sub(a1276, a1277)
                            }), v4)))))
                        } else {
                            Ok(Some(({
                                let a1278 = v2;
                                let a1279 = 1u64;
                                nat_add(a1278, a1279)?
                            }, (v3, v4))))
                        }
                    }
                }
            } else {
                Ok(Some(({
                    let a1282 = v2;
                    let a1283 = 1u64;
                    nat_add(a1282, a1283)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f228(v0: Fn34, p1287: List<Bytes>, p1288: List<Bytes>) -> List<Bytes> {
    let m1286 = (p1287, p1288);
    {
        let mut m1286 = m1286;
        loop {
            let step = {
                let (v1, v2) = m1286;
                let m1284 = v2;
                match m1284.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1285: Fn34 = v0.clone();
                            c1285.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f230(v0: Bytes, v1: u64) -> bool {
    let m1289 = f245(v0, v1);
    match m1289 {
        None => {
            false
        }
        Some(v2) => {
            if f246(v2, 97u8, 122u8) {
                true
            } else {
                if f246(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1290 = v2;
                    let a1291 = 95u8;
                    if equal(a1290, a1291) {
                        true
                    } else {
                        let a1292 = 0u64;
                        let a1293 = v1;
                        if nat_lt(a1292, a1293) {
                            f246(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f247(v0, v1)
}

pub fn f232(v0: Fn35, p1297: Option<Adt9>, p1298: List<Adt9>) -> Option<Adt9> {
    let m1296 = (p1297, p1298);
    {
        let mut m1296 = m1296;
        loop {
            let step = {
                let (v1, v2) = m1296;
                let m1294 = v2;
                match m1294.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1295: Fn35 = v0.clone();
                            c1295.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1299 = v1;
    match m1299 {
        None => {
            if f40(f178(v2.clone()), v0) {
                Some(v2)
            } else {
                None::<Adt9>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f234(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1300 = v1;
    match m1300 {
        None => {
            if f40(match v2.clone() {
                Adt13::C0(h1301, _) => {
                    h1301
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f235(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let a1302 = v0.clone();
    let a1303 = f16(Str::lit("AtomicU64"));
    if equal(a1302, a1303) {
        let a1304 = v1;
        let a1305 = f16(Str::lit("new"));
        if equal(a1304, a1305) {
            Ok(Some(1u64))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a1306 = v0.clone();
        let a1307 = f16(Str::lit("Rc"));
        if equal(a1306, a1307) {
            if f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_unwrap")), List::cons(f16(Str::lit("get_mut")), List::<Bytes>::nil())))), v1)? {
                Ok(Some(1u64))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let a1308 = v0.clone();
            let a1309 = f16(Str::lit("Vec"));
            if equal(a1308, a1309) {
                let a1310 = v1.clone();
                let a1311 = f16(Str::lit("new"));
                if equal(a1310, a1311) {
                    Ok(Some(0u64))
                } else {
                    let a1312 = v1;
                    let a1313 = f16(Str::lit("with_capacity"));
                    if equal(a1312, a1313) {
                        Ok(Some(1u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            } else {
                let a1314 = v0.clone();
                let a1315 = f16(Str::lit("String"));
                if equal(a1314, a1315) {
                    let a1316 = v1;
                    let a1317 = f16(Str::lit("new"));
                    if equal(a1316, a1317) {
                        Ok(Some(0u64))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a1318 = v0.clone();
                    let a1319 = f16(Str::lit("mem"));
                    if equal(a1318, a1319) {
                        let a1320 = v1.clone();
                        let a1321 = f16(Str::lit("replace"));
                        if equal(a1320, a1321) {
                            Ok(Some(2u64))
                        } else {
                            let a1322 = v1;
                            let a1323 = f16(Str::lit("take"));
                            if equal(a1322, a1323) {
                                Ok(Some(1u64))
                            } else {
                                Ok(None::<u64>)
                            }
                        }
                    } else {
                        let a1324 = v0.clone();
                        let a1325 = f16(Str::lit("str"));
                        if equal(a1324, a1325) {
                            let a1326 = v1;
                            let a1327 = f16(Str::lit("from_utf8"));
                            if equal(a1326, a1327) {
                                Ok(Some(1u64))
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            let a1328 = v0.clone();
                            let a1329 = f16(Str::lit("usize"));
                            if equal(a1328, a1329) {
                                if f66(List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_from")), List::<Bytes>::nil())), v1)? {
                                    Ok(Some(1u64))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a1330 = v0.clone();
                                let a1331 = f16(Str::lit("fs"));
                                if equal(a1330, a1331) {
                                    if f66(List::cons(f16(Str::lit("read")), List::cons(f16(Str::lit("read_dir")), List::cons(f16(Str::lit("symlink_metadata")), List::cons(f16(Str::lit("create_dir_all")), List::cons(f16(Str::lit("remove_file")), List::cons(f16(Str::lit("remove_dir_all")), List::cons(f16(Str::lit("canonicalize")), List::<Bytes>::nil()))))))), v1.clone())? {
                                        Ok(Some(1u64))
                                    } else {
                                        if f66(List::cons(f16(Str::lit("write")), List::cons(f16(Str::lit("rename")), List::<Bytes>::nil())), v1)? {
                                            Ok(Some(2u64))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    }
                                } else {
                                    let a1332 = v0.clone();
                                    let a1333 = f16(Str::lit("File"));
                                    if equal(a1332, a1333) {
                                        let a1334 = v1;
                                        let a1335 = f16(Str::lit("create"));
                                        if equal(a1334, a1335) {
                                            Ok(Some(1u64))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    } else {
                                        let a1336 = v0.clone();
                                        let a1337 = f16(Str::lit("env"));
                                        if equal(a1336, a1337) {
                                            if f66(List::cons(f16(Str::lit("args_os")), List::cons(f16(Str::lit("current_dir")), List::cons(f16(Str::lit("current_exe")), List::cons(f16(Str::lit("vars_os")), List::<Bytes>::nil())))), v1)? {
                                                Ok(Some(0u64))
                                            } else {
                                                Ok(None::<u64>)
                                            }
                                        } else {
                                            let a1338 = v0.clone();
                                            let a1339 = f16(Str::lit("io"));
                                            if equal(a1338, a1339) {
                                                if f66(List::cons(f16(Str::lit("stdout")), List::cons(f16(Str::lit("stderr")), List::<Bytes>::nil())), v1)? {
                                                    Ok(Some(0u64))
                                                } else {
                                                    Ok(None::<u64>)
                                                }
                                            } else {
                                                let a1340 = v0.clone();
                                                let a1341 = f16(Str::lit("process"));
                                                if equal(a1340, a1341) {
                                                    let a1342 = v1;
                                                    let a1343 = f16(Str::lit("exit"));
                                                    if equal(a1342, a1343) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        Ok(None::<u64>)
                                                    }
                                                } else {
                                                    let a1344 = v0.clone();
                                                    let a1345 = f16(Str::lit("Command"));
                                                    if equal(a1344, a1345) {
                                                        if f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("spawn")), List::<Bytes>::nil())), v1)? {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            Ok(None::<u64>)
                                                        }
                                                    } else {
                                                        let a1346 = v0.clone();
                                                        let a1347 = f16(Str::lit("Stdio"));
                                                        if equal(a1346, a1347) {
                                                            let a1348 = v1;
                                                            let a1349 = f16(Str::lit("null"));
                                                            if equal(a1348, a1349) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                Ok(None::<u64>)
                                                            }
                                                        } else {
                                                            let a1350 = v0.clone();
                                                            let a1351 = f16(Str::lit("Instant"));
                                                            if equal(a1350, a1351) {
                                                                let a1352 = v1;
                                                                let a1353 = f16(Str::lit("now"));
                                                                if equal(a1352, a1353) {
                                                                    Ok(Some(0u64))
                                                                } else {
                                                                    Ok(None::<u64>)
                                                                }
                                                            } else {
                                                                let a1354 = v0.clone();
                                                                let a1355 = f16(Str::lit("Duration"));
                                                                if equal(a1354, a1355) {
                                                                    let a1356 = v1;
                                                                    let a1357 = f16(Str::lit("from_millis"));
                                                                    if equal(a1356, a1357) {
                                                                        Ok(Some(1u64))
                                                                    } else {
                                                                        Ok(None::<u64>)
                                                                    }
                                                                } else {
                                                                    let a1358 = v0.clone();
                                                                    let a1359 = f16(Str::lit("thread"));
                                                                    if equal(a1358, a1359) {
                                                                        let a1360 = v1;
                                                                        let a1361 = f16(Str::lit("sleep"));
                                                                        if equal(a1360, a1361) {
                                                                            Ok(Some(1u64))
                                                                        } else {
                                                                            Ok(None::<u64>)
                                                                        }
                                                                    } else {
                                                                        let a1362 = v0;
                                                                        let a1363 = f16(Str::lit("Builder"));
                                                                        if equal(a1362, a1363) {
                                                                            let a1364 = v1;
                                                                            let a1365 = f16(Str::lit("new"));
                                                                            if equal(a1364, a1365) {
                                                                                Ok(Some(0u64))
                                                                            } else {
                                                                                Ok(None::<u64>)
                                                                            }
                                                                        } else {
                                                                            Ok(None::<u64>)
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1366 = v0.clone();
    let a1367 = f16(Str::lit("Rc"));
    if equal(a1366, a1367) {
        f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("from")), List::cons(f16(Str::lit("try_unwrap")), List::cons(f16(Str::lit("get_mut")), List::<Bytes>::nil())))), v1)
    } else {
        let a1368 = v0.clone();
        let a1369 = f16(Str::lit("Vec"));
        if equal(a1368, a1369) {
            f66(List::cons(f16(Str::lit("new")), List::cons(f16(Str::lit("with_capacity")), List::<Bytes>::nil())), v1)
        } else {
            let a1370 = v0;
            let a1371 = f16(Str::lit("String"));
            if equal(a1370, a1371) {
                let a1372 = v1;
                let a1373 = f16(Str::lit("new"));
                Ok(equal(a1372, a1373))
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f237(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1374 = f186(v0, v1);
    match m1374 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1375 = v2;
            match m1375 {
                Adt9::C0(_) => {
                    Ok(None::<u64>)
                }
                Adt9::C1(_, r1376) => {
                    let v5 = (*r1376).clone();
                    Ok(Some(f58(v5)?))
                }
                Adt9::C2(_, _, _) => {
                    Ok(None::<u64>)
                }
            }
        }
    }
}

pub fn f238(v0: Adt2, v1: u64) -> Adt2 {
    let m1377 = v0;
    match m1377 {
        Adt2::C0 => {
            Adt2::C16
        }
        Adt2::C1 => {
            Adt2::C16
        }
        Adt2::C2 => {
            Adt2::C16
        }
        Adt2::C3 => {
            Adt2::C16
        }
        Adt2::C4 => {
            Adt2::C16
        }
        Adt2::C5 => {
            Adt2::C16
        }
        Adt2::C6(_) => {
            Adt2::C16
        }
        Adt2::C7(_) => {
            Adt2::C16
        }
        Adt2::C8(_) => {
            Adt2::C16
        }
        Adt2::C9(r1382, r1383) => {
            let v5 = (*r1382).clone();
            let v6 = (*r1383).clone();
            let a1378 = v1;
            let a1379 = 0u64;
            if nat_eq(a1378, a1379) {
                v5
            } else {
                let a1380 = v1;
                let a1381 = 1u64;
                if nat_eq(a1380, a1381) {
                    v6
                } else {
                    Adt2::C16
                }
            }
        }
        Adt2::C10(_, _) => {
            Adt2::C16
        }
        Adt2::C11 => {
            Adt2::C16
        }
        Adt2::C12 => {
            Adt2::C16
        }
        Adt2::C13 => {
            Adt2::C16
        }
        Adt2::C14 => {
            Adt2::C16
        }
        Adt2::C15(_) => {
            Adt2::C16
        }
        Adt2::C16 => {
            Adt2::C16
        }
        Adt2::C17 => {
            Adt2::C16
        }
        Adt2::C18 => {
            Adt2::C16
        }
        Adt2::C19(_, _) => {
            Adt2::C16
        }
    }
}

pub fn f239(v0: Adt16, v1: Adt2) -> Adt2 {
    let m1384 = v1.clone();
    match m1384 {
        Adt2::C0 => {
            v1
        }
        Adt2::C1 => {
            v1
        }
        Adt2::C2 => {
            v1
        }
        Adt2::C3 => {
            v1
        }
        Adt2::C4 => {
            v1
        }
        Adt2::C5 => {
            match v0 {
                Adt16::C0(_, _, h1385, _) => {
                    (*h1385).clone()
                }
            }
        }
        Adt2::C6(_) => {
            v1
        }
        Adt2::C7(_) => {
            v1
        }
        Adt2::C8(_) => {
            v1
        }
        Adt2::C9(_, _) => {
            v1
        }
        Adt2::C10(_, _) => {
            v1
        }
        Adt2::C11 => {
            v1
        }
        Adt2::C12 => {
            v1
        }
        Adt2::C13 => {
            v1
        }
        Adt2::C14 => {
            v1
        }
        Adt2::C15(_) => {
            v1
        }
        Adt2::C16 => {
            v1
        }
        Adt2::C17 => {
            v1
        }
        Adt2::C18 => {
            v1
        }
        Adt2::C19(_, _) => {
            v1
        }
    }
}

pub fn f240(v0: Adt2) -> bool {
    let m1386 = v0;
    match m1386 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            true
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f241(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1387 = f153(v1, v3.clone());
        match m1387 {
            None => {
                let m1389 = f154(v3);
                match m1389 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f40(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f223(Fn32::F248(v0), false, match v4 {
                    Adt10::C0(_, h1388) => {
                        h1388
                    }
                })
            }
        }
    }
}

pub fn f242(p1395: Adt3) -> bool {
    let m1394 = p1395;
    {
        let mut m1394 = m1394;
        loop {
            let step = {
                let v0 = m1394;
                let m1390 = v0;
                match m1390 {
                    Adt3::C0(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C17 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C21(v40, _, r1393) => {
                        let v42 = (*r1393).clone();
                        let a1391 = f207(v40);
                        let a1392 = f205(v42.clone());
                        if nat_lt(a1391, a1392) {
                            Ok::<Adt3, bool>(v42)
                        } else {
                            Err::<Adt3, bool>(false)
                        }
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, bool>(true)
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C30 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C31 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m1394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Fn36, p1399: u64, p1400: List<Bytes>) -> R<u64> {
    let m1398 = (p1399, p1400);
    {
        let mut m1398 = m1398;
        loop {
            let step = {
                let (v1, v2) = m1398;
                let m1396 = v2;
                match m1396.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1397: Fn36 = v0.clone();
                            c1397.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f40(v2, v0) {
        let a1401 = v1;
        let a1402 = 1u64;
        nat_add(a1401, a1402)
    } else {
        Ok(v1)
    }
}

pub fn f245(v0: Bytes, v1: u64) -> Option<u8> {
    let a1403 = v0;
    let a1404 = v1;
    index_bytes(a1403, a1404)
}

pub fn f246(v0: u8, v1: u8, v2: u8) -> bool {
    let m1407 = {
        let a1405 = v0;
        let a1406 = v1;
        fixed_u8::checked_sub(a1405, a1406)
    };
    match m1407 {
        None => {
            false
        }
        Some(_) => {
            let m1410 = {
                let a1408 = v2;
                let a1409 = v0;
                fixed_u8::checked_sub(a1408, a1409)
            };
            match m1410 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f247(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f249(v0, v1, 48u8) {
        Ok(Some({
            let a1411 = v1;
            let a1412 = 1u64;
            nat_add(a1411, a1412)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f40(match v2 {
            Adt5::C0(h1413, _, _, _, _) => {
                h1413
            }
        }, v0)
    }
}

pub fn f249(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1414 = f245(v0, v1);
    match m1414 {
        None => {
            false
        }
        Some(v3) => {
            let a1415 = v3;
            let a1416 = v2;
            equal(a1415, a1416)
        }
    }
}
