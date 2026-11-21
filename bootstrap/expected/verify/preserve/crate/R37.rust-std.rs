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
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<bool>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Fn0 {
    F11(std::rc::Rc<Adt2>, std::rc::Rc<Adt10>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt7>, std::rc::Rc<Adt0>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn0::F11(k0, k1, k2, k3, k4, k5) => f11((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), (**k5).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F23,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn1::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F57(std::rc::Rc<Adt9>, std::rc::Rc<Adt10>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt7>),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn2::F57(k0, k1, k2, k3, k4) => f57((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0) -> R<List<bool>> {
    let v7: Adt7 = Adt7::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3.clone()), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v5.clone()));
    let v8: u64 = f1(v5.clone())?;
    let v9: Adt0 = f2(f2(f2(Adt0::C14, v3), v4), v1.clone());
    let v10: Adt8 = f3(v7.clone(), Adt8::C0(std::rc::Rc::new(v9), std::rc::Rc::new(Adt9::C0)), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v11: Adt0 = match v10.clone() {
        Adt8::C0(h1, _) => {
            (*h1).clone()
        }
    };
    let v12: Adt0 = f4(f5(f6(v1)?), v11.clone());
    let v13: Adt10 = f7(v7.clone(), v11.clone(), match v10.clone() {
        Adt8::C0(_, h2) => {
            (*h2).clone()
        }
    }, v12.clone(), Adt10::C0);
    let v14: Adt10 = f8(v7.clone(), v12.clone(), f9(v13, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f6(v12.clone())?;
            let a4 = f6(v12.clone())?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?);
    let v15: Adt11 = {
        let (h7, _) = f10(Fn0::F11(std::rc::Rc::new(v5), std::rc::Rc::new(v14), std::rc::Rc::new(v11), std::rc::Rc::new(v12), std::rc::Rc::new(v7), std::rc::Rc::new(v6)), {
            let a8 = 1u64;
            let a9 = v8;
            nat_add(a8, a9)?
        }, Adt11::C0(f12(v8), std::rc::Rc::new(match v10 {
            Adt8::C0(_, h10) => {
                (*h10).clone()
            }
        })))?;
        h7
    };
    match v15 {
        Adt11::C0(h11, _) => {
            Ok(h11)
        }
    }
}

pub fn f1(v0: Adt2) -> R<u64> {
    let m12 = v0;
    match m12 {
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
        Adt2::C14(_, r15) => {
            let v33 = (*r15).clone();
            let v34: u64 = f1(v33)?;
            let a13 = 1u64;
            let a14 = v34;
            nat_add(a13, a14)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0) -> Adt0 {
    let m16 = v1;
    match m16 {
        Adt0::C0 => {
            v0
        }
        Adt0::C1 => {
            v0
        }
        Adt0::C2 => {
            v0
        }
        Adt0::C3 => {
            v0
        }
        Adt0::C4(_) => {
            v0
        }
        Adt0::C5 => {
            v0
        }
        Adt0::C6 => {
            v0
        }
        Adt0::C7 => {
            v0
        }
        Adt0::C8(r17) => {
            let v3 = (*r17).clone();
            f2(v0, v3)
        }
        Adt0::C9(r18, r19) => {
            let v4 = (*r18).clone();
            let v5 = (*r19).clone();
            f2(f2(v0, v4), v5)
        }
        Adt0::C10(r20) => {
            let v6 = (*r20).clone();
            f2(v0, v6)
        }
        Adt0::C11(r21, r22) => {
            let v7 = (*r21).clone();
            let v8 = (*r22).clone();
            f2(f2(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r23, r24) => {
            let v10 = (*r23).clone();
            let v11 = (*r24).clone();
            let v12: Adt0 = f13(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f2(f2(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r25, r26) => {
            let v13 = (*r25).clone();
            let v14 = (*r26).clone();
            f2(f2(v0, v13), v14)
        }
    }
}

pub fn f3(v0: Adt7, p37: Adt8, p38: Adt2, p39: u64, v4: List<bool>, p40: Adt0) -> R<Adt8> {
    let m36 = (p37, (p38, (p39, p40)));
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m36;
                let m27 = v2;
                match m27 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C14(r34, r35) => {
                        let v34 = (*r34).clone();
                        let v35 = (*r35).clone();
                        let v37: bool = {
                            let m28 = f14(v4.clone(), v3);
                            match m28 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a29 = v36;
                                    bool_not(a29)
                                }
                            }
                        };
                        let v38: Adt8 = Adt8::C0(std::rc::Rc::new(f15(v0.clone(), match v1.clone() {
                            Adt8::C0(h30, _) => {
                                (*h30).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt8::C0(_, h31) => {
                                (*h31).clone()
                            }
                        }));
                        let v39: Adt8 = if v37 {
                            f16(v0.clone(), v38, v34, f17(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>((v39, (v35, ({
                            let a32 = v3;
                            let a33 = 1u64;
                            nat_add(a32, a33)?
                        }, f18(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Adt0, v1: Adt0) -> Adt0 {
    let m41 = v0;
    match m41 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(_) => {
            v1
        }
        Adt0::C9(_, _) => {
            v1
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(_, _) => {
            v1
        }
        Adt0::C12(_) => {
            v1
        }
        Adt0::C13(_, _) => {
            v1
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(r42, r43) => {
            let v12 = (*r42).clone();
            let v13 = (*r43).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f4(v13, v1)))
        }
    }
}

pub fn f5(v0: u64) -> Adt0 {
    let m44 = v0;
    if m44 == 0 {
        Adt0::C14
    } else {
        let v1 = m44 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f5(v1)))
    }
}

pub fn f6(v0: Adt0) -> R<u64> {
    let m45 = v0;
    match m45 {
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
        Adt0::C15(_, r48) => {
            let v12 = (*r48).clone();
            let v13: u64 = f6(v12)?;
            let a46 = 1u64;
            let a47 = v13;
            nat_add(a46, a47)
        }
    }
}

pub fn f7(v0: Adt7, v1: Adt0, v2: Adt9, p53: Adt0, p54: Adt10) -> Adt10 {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v3, v4) = m52;
                let m49 = v3;
                match m49 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C15(r50, r51) => {
                        let v15 = (*r50).clone();
                        let v16 = (*r51).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v16, f19(v1.clone(), v15.clone(), f20(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt7, p59: Adt0, p60: Adt10) -> Adt10 {
    let m58 = (p59, p60);
    {
        let mut m58 = m58;
        loop {
            let step = {
                let (v1, v2) = m58;
                let m55 = v1;
                match m55 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C15(r56, r57) => {
                        let v13 = (*r56).clone();
                        let v14 = (*r57).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v14, if f21(f20(v0.clone(), Adt9::C0, v13.clone())) {
                            Adt10::C1(std::rc::Rc::new(v13), std::rc::Rc::new(Adt0::C0), std::rc::Rc::new(v2))
                        } else {
                            v2
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m58 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Adt10, v1: u64) -> R<Adt10> {
    let (h61, _) = f22(Fn1::F23, v1, v0)?;
    Ok(h61)
}

pub fn f10(v0: Fn0, p66: u64, p67: Adt11) -> R<(Adt11, bool)> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m62 = v1;
                if m62 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m62 - 1;
                    let m64 = {
                        let c63: Fn0 = v0.clone();
                        c63.apply(v2.clone())?
                    };
                    match m64 {
                        None => {
                            Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt11), R<(Adt11, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Adt2, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    f24(v4, v0, v2, v1, v3, v5, v6)
}

pub fn f12(v0: u64) -> List<bool> {
    let m68 = v0;
    if m68 == 0 {
        List::<bool>::nil()
    } else {
        let v1 = m68 - 1;
        List::cons(false, f12(v1))
    }
}

pub fn f13(v0: Adt0, v1: Adt0) -> Adt0 {
    if f25(v0.clone(), v1.clone()) {
        v0
    } else {
        f4(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f14(p72: List<bool>, p73: u64) -> Option<bool> {
    let m71 = (p72, p73);
    {
        let mut m71 = m71;
        loop {
            let step = {
                let (v0, v1) = m71;
                let m69 = v0;
                match m69.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m70 = v1;
                        if m70 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m70 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m71 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Adt7, v1: Adt0, v2: u64) -> Adt0 {
    f2(f26(v1, f27(v0.clone(), v2)), f28(v0, v2))
}

pub fn f16(v0: Adt7, v1: Adt8, v2: Adt2, v3: Adt0) -> R<Adt8> {
    let m74 = v2;
    match m74 {
        Adt2::C0(r85, _) => {
            let v4 = (*r85).clone();
            Ok(Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h75, _) => {
                    (*h75).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h76) => {
                    (*h76).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r86, r87, r88) => {
            let v8 = (*r86).clone();
            let v9 = (*r87).clone();
            let v10 = (*r88).clone();
            let v11: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h77, _) => {
                    (*h77).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h78) => {
                    (*h78).clone()
                }
            }));
            let v12: Adt8 = f16(v0.clone(), v11, v9, f17(v3.clone()))?;
            f16(v0, v12, v10, f18(v3))
        }
        Adt2::C3(r89, r90, r91) => {
            let v13 = (*r89).clone();
            let v14 = (*r90).clone();
            let v15 = (*r91).clone();
            let v16: Adt8 = f16(v0.clone(), v1, v13, f17(v3.clone()))?;
            let v17: Adt8 = f16(v0.clone(), v16, v14, f18(v3.clone()))?;
            f16(v0, v17, v15, f29(v3))
        }
        Adt2::C4(r92, r93, r94) => {
            let v18 = (*r92).clone();
            let v19 = (*r93).clone();
            let v20 = (*r94).clone();
            let v21: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h79, _) => {
                    (*h79).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h80) => {
                    (*h80).clone()
                }
            }));
            let v22: Adt8 = f16(v0.clone(), v21, v19, f17(v3.clone()))?;
            f16(v0, v22, v20, f18(v3))
        }
        Adt2::C5(_, r95, r96) => {
            let v24 = (*r95).clone();
            let v25 = (*r96).clone();
            let v26: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h81, _) => {
                    (*h81).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h82) => {
                    (*h82).clone()
                }
            }));
            f16(v0, v26, v25, f17(v3))
        }
        Adt2::C6(_, r97) => {
            let v28 = (*r97).clone();
            f16(v0, v1, v28, f17(v3))
        }
        Adt2::C7(v29, r98) => {
            let v30 = (*r98).clone();
            let v31: Adt0 = f27(v0.clone(), v29);
            let v32: u64 = f1(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f30(v31.clone(), v32)), std::rc::Rc::new(f28(v0.clone(), v29)));
            let v34: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h83, _) => {
                    (*h83).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f31(match v1 {
                Adt8::C0(_, h84) => {
                    (*h84).clone()
                }
            }, v33, v29, f32(v31, v32))));
            f16(v0, v34, v30, f17(v3))
        }
        Adt2::C8(r99, r100) => {
            let v35 = (*r99).clone();
            let v36 = (*r100).clone();
            let v37: Adt8 = f16(v0.clone(), v1, v35, f17(v3.clone()))?;
            f16(v0, v37, v36, f18(v3))
        }
        Adt2::C9(_, r101) => {
            let v39 = (*r101).clone();
            f16(v0, v1, v39, f17(v3))
        }
        Adt2::C10(r102) => {
            let v40 = (*r102).clone();
            f16(v0, v1, v40, f17(v3))
        }
        Adt2::C11(r103) => {
            let v41 = (*r103).clone();
            f16(v0, v1, v41, f17(v3))
        }
        Adt2::C12(r104, _) => {
            let v42 = (*r104).clone();
            f16(v0, v1, v42, f17(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r105, r106) => {
            let v44 = (*r105).clone();
            let v45 = (*r106).clone();
            let v46: Adt8 = f16(v0.clone(), v1, v44, f17(v3.clone()))?;
            f16(v0, v46, v45, f18(v3))
        }
        Adt2::C15(_, _, r107) => {
            let v49 = (*r107).clone();
            f16(v0, v1, v49, f17(v3))
        }
    }
}

pub fn f17(v0: Adt0) -> Adt0 {
    f33(f34(v0))
}

pub fn f18(v0: Adt0) -> Adt0 {
    f33(f34(f34(v0)))
}

pub fn f19(v0: Adt0, v1: Adt0, p112: Adt0, p113: Adt10) -> Adt10 {
    let m111 = (p112, p113);
    {
        let mut m111 = m111;
        loop {
            let step = {
                let (v2, v3) = m111;
                let m108 = v2;
                match m108 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r109, r110) => {
                        let v14 = (*r109).clone();
                        let v15 = (*r110).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f35(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt9, v2: Adt0) -> Adt0 {
    let m114 = v2;
    match m114 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(v10) => {
            let m115 = f36(v0, v10);
            match m115 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r116, r117) => {
            let v12 = (*r116).clone();
            let v13 = (*r117).clone();
            f37(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f21(p122: Adt0) -> bool {
    let m121 = p122;
    {
        let mut m121 = m121;
        loop {
            let step = {
                let v0 = m121;
                let m118 = v0;
                match m118 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r119, r120) => {
                        let v11 = (*r119).clone();
                        let v12 = (*r120).clone();
                        if f38(v11, Adt0::C14) {
                            Ok::<Adt0, bool>(v12)
                        } else {
                            Err::<Adt0, bool>(true)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Fn1, p127: u64, p128: Adt10) -> R<(Adt10, bool)> {
    let m126 = (p127, p128);
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v1, v2) = m126;
                let m123 = v1;
                if m123 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m123 - 1;
                    let m125 = {
                        let c124: Fn1 = v0.clone();
                        c124.apply(v2.clone())?
                    };
                    match m125 {
                        None => {
                            Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), R<(Adt10, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt10) -> R<Option<Adt10>> {
    f39(v0)
}

pub fn f24(v0: Adt7, v1: Adt2, v2: Adt0, v3: Adt10, v4: Adt0, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    let v7: Adt0 = f40(v0.clone(), v2.clone(), match v6.clone() {
        Adt11::C0(_, h129) => {
            (*h129).clone()
        }
    }, v3, v4)?;
    let v8: List<bool> = f41(match v0.clone() {
        Adt7::C0(_, _, _, h130, _, _) => {
            (*h130).clone()
        }
    }, v7);
    if f42(v8.clone(), match v6 {
        Adt11::C0(h131, _) => {
            h131
        }
    }) {
        Ok(None::<Adt11>)
    } else {
        Ok(Some(Adt11::C0(v8.clone(), std::rc::Rc::new({
            let m133 = f3(v0, Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(Adt9::C0)), v1, 0u64, v8, v5)?;
            match m133 {
                Adt8::C0(_, h132) => {
                    (*h132).clone()
                }
            }
        }))))
    }
}

pub fn f25(p138: Adt0, v1: Adt0) -> bool {
    let m137 = p138;
    {
        let mut m137 = m137;
        loop {
            let step = {
                let v0 = m137;
                let m134 = v0;
                match m134 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r135, r136) => {
                        let v12 = (*r135).clone();
                        let v13 = (*r136).clone();
                        if f38(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(p143: Adt0, p144: Adt0) -> Adt0 {
    let m142 = (p143, p144);
    {
        let mut m142 = m142;
        loop {
            let step = {
                let (v0, v1) = m142;
                let m139 = v1;
                match m139 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r140, r141) => {
                        let v2 = (*r140).clone();
                        let v3 = (*r141).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f2(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt7, v1: u64) -> Adt0 {
    let m145 = f43(v0, v1);
    match m145 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f28(v0: Adt7, v1: u64) -> Adt0 {
    let m147 = f44(match v0 {
        Adt7::C0(_, _, _, _, h146, _) => {
            (*h146).clone()
        }
    }, v1);
    match m147 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f29(v0: Adt0) -> Adt0 {
    f33(f34(f34(f34(v0))))
}

pub fn f30(p153: Adt0, p154: u64) -> Adt0 {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v0, v1) = m152;
                let m148 = v0;
                match m148 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C15(r150, r151) => {
                        let v12 = (*r150).clone();
                        let v13 = (*r151).clone();
                        let m149 = v1;
                        if m149 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m149 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f45(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f32(v0: Adt0, v1: u64) -> Adt0 {
    let m155 = v0;
    match m155 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(_) => {
            Adt0::C14
        }
        Adt0::C13(_, _) => {
            Adt0::C14
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(r157, r158) => {
            let v12 = (*r157).clone();
            let v13 = (*r158).clone();
            let m156 = v1;
            if m156 == 0 {
                Adt0::C14
            } else {
                let v14 = m156 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f32(v13, v14)))
            }
        }
    }
}

pub fn f33(v0: Adt0) -> Adt0 {
    let m159 = f46(v0);
    match m159 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f34(v0: Adt0) -> Adt0 {
    let m160 = f47(v0);
    match m160 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt0, p165: Adt0, p166: Adt10) -> Adt10 {
    let m164 = (p165, p166);
    {
        let mut m164 = m164;
        loop {
            let step = {
                let (v2, v3) = m164;
                let m161 = v2;
                match m161 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r162, r163) => {
                        let v14 = (*r162).clone();
                        let v15 = (*r163).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f48(v1.clone(), f49(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt7, v1: u64) -> Option<Adt0> {
    f44(match v0 {
        Adt7::C0(_, h167, _, _, _, _) => {
            (*h167).clone()
        }
    }, v1)
}

pub fn f37(v0: Adt9, v1: Adt0) -> Adt0 {
    let m168 = v0;
    match m168 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(r169, _, r170, r171) => {
            let v2 = (*r169).clone();
            let v4 = (*r170).clone();
            let v5 = (*r171).clone();
            if f38(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f37(v5, v1)))
            } else {
                f37(v5, v1)
            }
        }
    }
}

pub fn f38(v0: Adt0, v1: Adt0) -> bool {
    let m172 = v0;
    match m172 {
        Adt0::C0 => {
            let a173 = f50(v1);
            let a174 = 0u64;
            nat_eq(a173, a174)
        }
        Adt0::C1 => {
            let a175 = f50(v1);
            let a176 = 1u64;
            nat_eq(a175, a176)
        }
        Adt0::C2 => {
            let a177 = f50(v1);
            let a178 = 2u64;
            nat_eq(a177, a178)
        }
        Adt0::C3 => {
            let a179 = f50(v1);
            let a180 = 3u64;
            nat_eq(a179, a180)
        }
        Adt0::C4(v2) => {
            let a189 = f50(v1.clone());
            let a190 = 4u64;
            if nat_eq(a189, a190) {
                let a191 = f51(v2);
                let a192 = f51(f52(v1));
                nat_eq(a191, a192)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a181 = f50(v1);
            let a182 = 5u64;
            nat_eq(a181, a182)
        }
        Adt0::C6 => {
            let a183 = f50(v1);
            let a184 = 6u64;
            nat_eq(a183, a184)
        }
        Adt0::C7 => {
            let a185 = f50(v1);
            let a186 = 7u64;
            nat_eq(a185, a186)
        }
        Adt0::C8(r209) => {
            let v3 = (*r209).clone();
            let v4: bool = f38(v3, f33(v1.clone()));
            let a193 = f50(v1);
            let a194 = 8u64;
            if nat_eq(a193, a194) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r210, r211) => {
            let v5 = (*r210).clone();
            let v6 = (*r211).clone();
            let v7: bool = f38(v5, f33(v1.clone()));
            let v8: bool = f38(v6, f34(v1.clone()));
            let a195 = f50(v1);
            let a196 = 9u64;
            if nat_eq(a195, a196) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r212) => {
            let v9 = (*r212).clone();
            let v10: bool = f38(v9, f33(v1.clone()));
            let a197 = f50(v1);
            let a198 = 10u64;
            if nat_eq(a197, a198) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r213, r214) => {
            let v11 = (*r213).clone();
            let v12 = (*r214).clone();
            let v13: bool = f38(v11, f33(v1.clone()));
            let v14: bool = f38(v12, f34(v1.clone()));
            let a199 = f50(v1);
            let a200 = 11u64;
            if nat_eq(a199, a200) {
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
            let a201 = f50(v1.clone());
            let a202 = 12u64;
            if nat_eq(a201, a202) {
                let a203 = v15;
                let a204 = f53(v1);
                nat_eq(a203, a204)
            } else {
                false
            }
        }
        Adt0::C13(r215, r216) => {
            let v16 = (*r215).clone();
            let v17 = (*r216).clone();
            let v18: bool = f38(v16, f33(v1.clone()));
            let v19: bool = f38(v17, f34(v1.clone()));
            let a205 = f50(v1);
            let a206 = 13u64;
            if nat_eq(a205, a206) {
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
            let a187 = f50(v1);
            let a188 = 14u64;
            nat_eq(a187, a188)
        }
        Adt0::C15(r217, r218) => {
            let v20 = (*r217).clone();
            let v21 = (*r218).clone();
            let v22: bool = f38(v20, f33(v1.clone()));
            let v23: bool = f38(v21, f34(v1.clone()));
            let a207 = f50(v1);
            let a208 = 15u64;
            if nat_eq(a207, a208) {
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

pub fn f39(v0: Adt10) -> R<Option<Adt10>> {
    let v1: Adt10 = f54(v0.clone(), v0.clone(), v0.clone());
    let a219 = f55(v1.clone())?;
    let a220 = f55(v0)?;
    if nat_eq(a219, a220) {
        Ok(None::<Adt10>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f40(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0) -> R<Adt0> {
    let (h221, _) = f56(Fn2::F57(std::rc::Rc::new(v2), std::rc::Rc::new(v3), std::rc::Rc::new(v1), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v0)), {
        let a222 = 1u64;
        let a223 = f6(v4)?;
        nat_add(a222, a223)?
    }, Adt0::C14)?;
    Ok(h221)
}

pub fn f41(v0: Adt0, v1: Adt0) -> List<bool> {
    let m224 = v0;
    match m224 {
        Adt0::C0 => {
            List::<bool>::nil()
        }
        Adt0::C1 => {
            List::<bool>::nil()
        }
        Adt0::C2 => {
            List::<bool>::nil()
        }
        Adt0::C3 => {
            List::<bool>::nil()
        }
        Adt0::C4(_) => {
            List::<bool>::nil()
        }
        Adt0::C5 => {
            List::<bool>::nil()
        }
        Adt0::C6 => {
            List::<bool>::nil()
        }
        Adt0::C7 => {
            List::<bool>::nil()
        }
        Adt0::C8(_) => {
            List::<bool>::nil()
        }
        Adt0::C9(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C10(_) => {
            List::<bool>::nil()
        }
        Adt0::C11(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C12(_) => {
            List::<bool>::nil()
        }
        Adt0::C13(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C14 => {
            List::<bool>::nil()
        }
        Adt0::C15(r225, r226) => {
            let v12 = (*r225).clone();
            let v13 = (*r226).clone();
            List::cons(f58(v12, v1.clone()), f41(v13, v1))
        }
    }
}

pub fn f42(p233: List<bool>, p234: List<bool>) -> bool {
    let m232 = (p233, p234);
    {
        let mut m232 = m232;
        loop {
            let step = {
                let (v0, v1) = m232;
                let m227 = v0;
                match m227.uncons() {
                    None => {
                        let m228 = v1;
                        match m228.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m229 = v1;
                        match m229.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a230 = v4;
                                let a231 = v6;
                                if equal(a230, a231) {
                                    Ok::<(List<bool>, List<bool>), bool>((v5, v7))
                                } else {
                                    Err::<(List<bool>, List<bool>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt7, v1: u64) -> Option<Adt0> {
    f44(match v0 {
        Adt7::C0(_, _, _, h235, _, _) => {
            (*h235).clone()
        }
    }, v1)
}

pub fn f44(p241: Adt0, p242: u64) -> Option<Adt0> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v0, v1) = m240;
                let m236 = v0;
                match m236 {
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
                    Adt0::C15(r238, r239) => {
                        let v12 = (*r238).clone();
                        let v13 = (*r239).clone();
                        let m237 = v1;
                        if m237 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m237 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(p249: Adt9, v1: Adt0, v2: u64) -> bool {
    let m248 = p249;
    {
        let mut m248 = m248;
        loop {
            let step = {
                let v0 = m248;
                let m243 = v0;
                match m243 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r246, v4, _, r247) => {
                        let v3 = (*r246).clone();
                        let v6 = (*r247).clone();
                        let a244 = v4;
                        let a245 = v2;
                        if nat_eq(a244, a245) {
                            if f38(v3, v1.clone()) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Adt0) -> Option<Adt0> {
    let m250 = v0;
    match m250 {
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
        Adt0::C8(r251) => {
            let v2 = (*r251).clone();
            Some(v2)
        }
        Adt0::C9(r252, _) => {
            let v3 = (*r252).clone();
            Some(v3)
        }
        Adt0::C10(r253) => {
            let v5 = (*r253).clone();
            Some(v5)
        }
        Adt0::C11(r254, _) => {
            let v6 = (*r254).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r255, _) => {
            let v9 = (*r255).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r256, _) => {
            let v11 = (*r256).clone();
            Some(v11)
        }
    }
}

pub fn f47(v0: Adt0) -> Option<Adt0> {
    let m257 = v0;
    match m257 {
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
        Adt0::C9(_, r258) => {
            let v4 = (*r258).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r259) => {
            let v7 = (*r259).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r260) => {
            let v10 = (*r260).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r261) => {
            let v12 = (*r261).clone();
            Some(v12)
        }
    }
}

pub fn f48(v0: Adt0, p266: Adt0, p267: Adt10) -> Adt10 {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m262 = v1;
                match m262 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C15(r263, r264) => {
                        let v13 = (*r263).clone();
                        let v14 = (*r264).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v14, f59(v2, v0.clone(), v13)))
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m268 = v2;
    match m268 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(r269) => {
            let v4 = (*r269).clone();
            f49(v0, v1, v4)
        }
        Adt0::C9(r270, r271) => {
            let v5 = (*r270).clone();
            let v6 = (*r271).clone();
            f49(v0.clone(), f49(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r272, r273) => {
            let v8 = (*r272).clone();
            let v9 = (*r273).clone();
            f49(v0.clone(), f49(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f13(v1, Adt0::C12(v10))
        }
        Adt0::C13(r274, r275) => {
            let v11 = (*r274).clone();
            let v12 = (*r275).clone();
            if f25(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f13(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
            } else {
                v1
            }
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f50(v0: Adt0) -> u64 {
    let m276 = v0;
    match m276 {
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

pub fn f51(v0: Adt1) -> u64 {
    let m277 = v0;
    match m277 {
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

pub fn f52(v0: Adt0) -> Adt1 {
    let m278 = v0;
    match m278 {
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

pub fn f53(v0: Adt0) -> u64 {
    let m279 = v0;
    match m279 {
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

pub fn f54(v0: Adt10, p285: Adt10, p286: Adt10) -> Adt10 {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m280 = v1;
                match m280 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v2)
                    }
                    Adt10::C1(r281, r282, r283) => {
                        let v3 = (*r281).clone();
                        let v4 = (*r282).clone();
                        let v5 = (*r283).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v5, f60(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: Adt10) -> R<u64> {
    let m287 = v0;
    match m287 {
        Adt10::C0 => {
            Ok(0u64)
        }
        Adt10::C1(_, _, r290) => {
            let v3 = (*r290).clone();
            let a288 = 1u64;
            let a289 = f55(v3)?;
            nat_add(a288, a289)
        }
    }
}

pub fn f56(v0: Fn2, p295: u64, p296: Adt0) -> R<(Adt0, bool)> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v1, v2) = m294;
                let m291 = v1;
                if m291 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m291 - 1;
                    let m293 = {
                        let c292: Fn2 = v0.clone();
                        c292.apply(v2.clone())?
                    };
                    match m293 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt9, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0) -> R<Option<Adt0>> {
    f61(v4, v2, v0, v1, v3, v5)
}

pub fn f58(p301: Adt0, v1: Adt0) -> bool {
    let m300 = p301;
    {
        let mut m300 = m300;
        loop {
            let step = {
                let v0 = m300;
                let m297 = v0;
                match m297 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r298, r299) => {
                        let v12 = (*r298).clone();
                        let v13 = (*r299).clone();
                        if f62(v12, v1.clone()) {
                            Ok::<Adt0, bool>(v13)
                        } else {
                            Err::<Adt0, bool>(true)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt10, v1: Adt0, v2: Adt0) -> Adt10 {
    if f63(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt10::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v2), std::rc::Rc::new(v0))
    }
}

pub fn f60(p307: Adt10, v1: Adt0, v2: Adt0, p308: Adt10) -> Adt10 {
    let m306 = (p307, p308);
    {
        let mut m306 = m306;
        loop {
            let step = {
                let (v0, v3) = m306;
                let m302 = v0;
                match m302 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v3)
                    }
                    Adt10::C1(r303, r304, r305) => {
                        let v4 = (*r303).clone();
                        let v5 = (*r304).clone();
                        let v6 = (*r305).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v6, if f38(v4, v1.clone()) {
                            f59(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f64(v0, v1, v2, v3, v4, v5.clone());
    let a309 = f6(v6.clone())?;
    let a310 = f6(v5)?;
    if nat_eq(a309, a310) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f62(v0: Adt0, v1: Adt0) -> bool {
    let m311 = v0;
    match m311 {
        Adt0::C0 => {
            true
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
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r312, r313) => {
            let v4 = (*r312).clone();
            let v5 = (*r313).clone();
            if f62(v4, v1.clone()) {
                true
            } else {
                f62(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r314, r315) => {
            let v7 = (*r314).clone();
            let v8 = (*r315).clone();
            if f62(v7, v1.clone()) {
                f62(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f25(v1, Adt0::C12(v9))
        }
        Adt0::C13(r316, r317) => {
            let v10 = (*r316).clone();
            let v11 = (*r317).clone();
            f25(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f63(p323: Adt10, v1: Adt0, v2: Adt0) -> bool {
    let m322 = p323;
    {
        let mut m322 = m322;
        loop {
            let step = {
                let v0 = m322;
                let m318 = v0;
                match m318 {
                    Adt10::C0 => {
                        Err::<Adt10, bool>(false)
                    }
                    Adt10::C1(r319, r320, r321) => {
                        let v3 = (*r319).clone();
                        let v4 = (*r320).clone();
                        let v5 = (*r321).clone();
                        if f38(v3, v1.clone()) {
                            if f38(v4, v2.clone()) {
                                Err::<Adt10, bool>(true)
                            } else {
                                Ok::<Adt10, bool>(v5)
                            }
                        } else {
                            Ok::<Adt10, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, p328: Adt0, p329: Adt0) -> Adt0 {
    let m327 = (p328, p329);
    {
        let mut m327 = m327;
        loop {
            let step = {
                let (v4, v5) = m327;
                let m324 = v4;
                match m324 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r325, r326) => {
                        let v16 = (*r325).clone();
                        let v17 = (*r326).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f25(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f65(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f20(v0.clone(), v2.clone(), v16.clone())) {
                                f13(v5, v16)
                            } else {
                                v5
                            }
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p334: Adt0) -> bool {
    let m333 = p334;
    {
        let mut m333 = m333;
        loop {
            let step = {
                let v4 = m333;
                let m330 = v4;
                match m330 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r331, r332) => {
                        let v15 = (*r331).clone();
                        let v16 = (*r332).clone();
                        if f66(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p339: Adt0) -> bool {
    let m338 = p339;
    {
        let mut m338 = m338;
        loop {
            let step = {
                let v4 = m338;
                let m335 = v4;
                match m335 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r336, r337) => {
                        let v15 = (*r336).clone();
                        let v16 = (*r337).clone();
                        if if f67(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f62(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0) -> bool {
    f68(v2, v1, f49(v0, Adt0::C14, v3))
}

pub fn f68(v0: Adt0, v1: Adt10, p344: Adt0) -> bool {
    let m343 = p344;
    {
        let mut m343 = m343;
        loop {
            let step = {
                let v2 = m343;
                let m340 = v2;
                match m340 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r341, r342) => {
                        let v13 = (*r341).clone();
                        let v14 = (*r342).clone();
                        if f38(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f63(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}
