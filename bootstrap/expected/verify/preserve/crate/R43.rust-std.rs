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
    C0(bool, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(std::rc::Rc<Adt2>, std::rc::Rc<Adt8>, std::rc::Rc<Adt9>),
    C1(u64, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, bool, std::rc::Rc<Adt10>, std::rc::Rc<Adt9>),
    C2(std::rc::Rc<Adt12>, std::rc::Rc<Adt13>, std::rc::Rc<Adt2>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C3,
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Adt4),
    C2(Adt5, std::rc::Rc<Adt2>),
    C3(Adt6, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt7, u64),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt4 {
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
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(u64, std::rc::Rc<Adt2>, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(u64, List<Adt11>, bool, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1(std::rc::Rc<Adt14>, std::rc::Rc<Adt2>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt15),
    C1(std::rc::Rc<Adt12>),
    C2(std::rc::Rc<Adt14>),
    C3(Adt16, std::rc::Rc<Adt14>),
    C4(u64, u64, std::rc::Rc<Adt14>),
    C5,
    C6(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C7(Adt17),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt18 {
    C0(std::rc::Rc<Adt19>, std::rc::Rc<Adt9>),
    C1(Adt20, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C2(Adt21, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C3(std::rc::Rc<Adt22>, std::rc::Rc<Adt18>, bool, std::rc::Rc<Adt9>),
    C4(std::rc::Rc<Adt12>, std::rc::Rc<Adt18>, bool, std::rc::Rc<Adt9>),
    C5(std::rc::Rc<Adt24>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C6(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C7(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C8(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C9(std::rc::Rc<Adt18>),
    C10(Adt25, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C11(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C12,
    C13(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C14(std::rc::Rc<Adt14>, bool, std::rc::Rc<Adt2>, std::rc::Rc<Adt18>, std::rc::Rc<Adt9>, std::rc::Rc<Adt18>),
    C15(std::rc::Rc<Adt14>, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C16(std::rc::Rc<Adt12>, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt4, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt17),
    C8,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64),
    C1(std::rc::Rc<Adt23>),
    C2(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(std::rc::Rc<Adt2>),
    C1,
    C2(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F12,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F12 => f12(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F38,
    F39,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt26), p1: Adt11) -> R<(u64, Adt26)> {
        match self {
            Fn2::F38 => f38(p0, p1),
            Fn2::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F41(u64, u64),
    F81(u64),
    F90(Bytes),
    F91(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
        match self {
            Fn3::F41(k0, k1) => f41(*k0, *k1, p0),
            Fn3::F81(k0) => f81(*k0, p0),
            Fn3::F90(k0) => f90(k0.clone(), p0),
            Fn3::F91(k0) => f91(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F63(u64, std::rc::Rc<Adt13>, std::rc::Rc<Adt2>),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
        match self {
            Fn4::F63(k0, k1, k2) => f63(*k0, (**k1).clone(), (**k2).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F65,
}

impl Fn5 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn5::F65 => f65(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F88(Bytes),
    F104(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F88(k0) => f88(k0.clone(), p0),
            Fn6::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Bytes, v2: Adt0) -> R<Result<Bytes, Bytes>> {
    Ok(f1(f2(f3(f4(v0), v1)?, match v2 {
        Adt0::C0(_, h1) => {
            (*h1).clone()
        }
    })?))
}

pub fn f1(v0: Adt26) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt26::C0(_, _, _, _, h2) => {
            h2
        }
    } {
        Err::<Bytes, Bytes>({
            let a3 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a3)
        })
    } else {
        Ok::<Bytes, Bytes>(f5(f6(List::cons(match v0.clone() {
            Adt26::C0(_, h4, _, _, _) => {
                h4
            }
        }, match v0 {
            Adt26::C0(h5, _, _, _, _) => {
                h5
            }
        }))))
    }
}

pub fn f2(p10: Adt26, p11: Adt1) -> R<Adt26> {
    let m9 = (p10, p11);
    {
        let mut m9 = m9;
        loop {
            let step = {
                let (v0, v1) = m9;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C4(r7, r8) => {
                        let v2 = (*r7).clone();
                        let v3 = (*r8).clone();
                        Ok::<(Adt26, Adt1), R<Adt26>>((f7(f8(v0, Str::lit("\u{a}"))?, v2)?, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m9 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt26, v1: Bytes) -> R<Adt26> {
    if match v0.clone() {
        Adt26::C0(_, _, _, _, h12) => {
            h12
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a15 = match v0.clone() {
                Adt26::C0(_, _, h13, _, _) => {
                    h13
                }
            };
            let a16 = {
                let a14 = v1.clone();
                length_bytes(a14)
            };
            nat_add(a15, a16)?
        };
        let a18 = v2;
        let a19 = match v0.clone() {
            Adt26::C0(_, _, _, h17, _) => {
                h17
            }
        };
        if nat_le(a18, a19) {
            Ok(f9(v0.clone(), {
                let a21 = match v0 {
                    Adt26::C0(_, h20, _, _, _) => {
                        h20
                    }
                };
                let a22 = v1;
                append_bytes(a21, a22)
            }, v2))
        } else {
            Ok(f10(v0))
        }
    }
}

pub fn f4(v0: u64) -> Adt26 {
    Adt26::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f5(v0: List<Bytes>) -> Bytes {
    let m25 = {
        let (h23, _) = f11(Fn0::F12, {
            let a24 = v0.clone();
            length_list(a24)
        }, v0);
        h23
    };
    match m25.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f6(v0: List<Bytes>) -> List<Bytes> {
    f13(Fn1::F14, List::<Bytes>::nil(), v0)
}

pub fn f7(v0: Adt26, v1: Adt1) -> R<Adt26> {
    let m26 = v1;
    match m26 {
        Adt1::C0(r27, r28, _) => {
            let v2 = (*r27).clone();
            let v3 = (*r28).clone();
            f8(f15(f8(f16(f8(v0, Str::lit("#[derive(Clone)]\u{a}pub enum "))?, v2.clone())?, Str::lit(" {\u{a}"))?, v3, f17(v2))?, Str::lit("}\u{a}"))
        }
        Adt1::C1(v5, r29, r30, v8, r31, _) => {
            let v6 = (*r29).clone();
            let v7 = (*r30).clone();
            let v9 = (*r31).clone();
            let v11: bool = f18(v9.clone());
            let v12: Adt26 = f19(f8(v0, Str::lit("impl Fn"))?, v5)?;
            let v13: Adt26 = f8(f20(f8(f8(v12, Str::lit(" {\u{a}"))?, Str::lit("    pub fn apply(&self, "))?, v6.clone(), 0u64, v11)?, Str::lit(")"))?;
            let v14: Adt26 = if v8 {
                f8(f16(f8(v13, Str::lit(" -> R<"))?, v7.clone())?, Str::lit(">"))?
            } else {
                f21(v13, v7.clone())?
            };
            let v15: Adt26 = f8(v14, Str::lit(" {\u{a}"))?;
            let v16: Adt26 = if v11 {
                f8(v15, Str::lit("        match *self {}\u{a}"))?
            } else {
                f8(f22(f8(v15, Str::lit("        match self {\u{a}"))?, v9, v5, f23(v6)?, f24(v7), v8)?, Str::lit("        }\u{a}"))?
            };
            f8(f8(v16, Str::lit("    }\u{a}"))?, Str::lit("}\u{a}"))
        }
        Adt1::C2(r32, r33, r34, r35, _) => {
            let v17 = (*r32).clone();
            let v18 = (*r33).clone();
            let v19 = (*r34).clone();
            let v20 = (*r35).clone();
            f8(f25(f8(f21(f8(f26(f8(f27(f28(f8(v0, Str::lit("pub fn "))?, v17)?, v18.clone(), v19.clone())?, Str::lit("("))?, v18, true)?, Str::lit(")"))?, v19)?, Str::lit(" "))?, v20, 0u64, 100u64, 0u64)?, Str::lit("\u{a}"))
        }
        Adt1::C3 => {
            Ok(v0)
        }
        Adt1::C4(_, _) => {
            Ok(v0)
        }
    }
}

pub fn f8(v0: Adt26, v1: Str) -> R<Adt26> {
    f3(v0, {
        let a36 = v1;
        utf8_encode(a36)
    })
}

pub fn f9(v0: Adt26, v1: Bytes, v2: u64) -> Adt26 {
    let a38 = 256u64;
    let a39 = {
        let a37 = v1.clone();
        length_bytes(a37)
    };
    if nat_le(a38, a39) {
        Adt26::C0(List::cons(v1, match v0.clone() {
            Adt26::C0(h40, _, _, _, _) => {
                h40
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt26::C0(_, _, _, h41, _) => {
                h41
            }
        }, false)
    } else {
        Adt26::C0(match v0.clone() {
            Adt26::C0(h42, _, _, _, _) => {
                h42
            }
        }, v1, v2, match v0 {
            Adt26::C0(_, _, _, h43, _) => {
                h43
            }
        }, false)
    }
}

pub fn f10(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h44, _, _, _, _) => {
            h44
        }
    }, match v0.clone() {
        Adt26::C0(_, h45, _, _, _) => {
            h45
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h46, _, _) => {
            h46
        }
    }, match v0 {
        Adt26::C0(_, _, _, h47, _) => {
            h47
        }
    }, true)
}

pub fn f11(v0: Fn0, p52: u64, p53: List<Bytes>) -> (List<Bytes>, bool) {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v1, v2) = m51;
                let m48 = v1;
                if m48 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m48 - 1;
                    let m50 = {
                        let c49: Fn0 = v0.clone();
                        c49.apply(v2.clone())
                    };
                    match m50 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m51 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<Bytes>) -> Option<List<Bytes>> {
    f29(v0)
}

pub fn f13(v0: Fn1, p57: List<Bytes>, p58: List<Bytes>) -> List<Bytes> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m54 = v2;
                match m54.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c55: Fn1 = v0.clone();
                            c55.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f15(p63: Adt26, p64: Adt8, v2: bool) -> R<Adt26> {
    let m62 = (p63, p64);
    {
        let mut m62 = m62;
        loop {
            let step = {
                let (v0, v1) = m62;
                let m59 = v1;
                match m59 {
                    Adt8::C0 => {
                        Err::<(Adt26, Adt8), R<Adt26>>(Ok(v0))
                    }
                    Adt8::C1(v3, r60, r61) => {
                        let v4 = (*r60).clone();
                        let v5 = (*r61).clone();
                        let v6: Adt26 = f19(f8(f8(v0, Str::lit("    "))?, if v2 {
                            Str::lit("F")
                        } else {
                            Str::lit("C")
                        })?, v3)?;
                        let v7: Adt26 = if f30(v4.clone()) {
                            f8(f16(f8(v6, Str::lit("("))?, v4)?, Str::lit(")"))?
                        } else {
                            v6
                        };
                        Ok::<(Adt26, Adt8), R<Adt26>>((f8(v7, Str::lit(",\u{a}"))?, v5))
                    }
                }
            };
            match step {
                Ok(next) => { m62 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Adt26, v1: Adt2) -> R<Adt26> {
    let m65 = v1;
    match m65 {
        Adt2::C0(v2) => {
            f31(v0, v2)
        }
        Adt2::C1(v3) => {
            f32(v0, v3)
        }
        Adt2::C2(v4, r69) => {
            let v5 = (*r69).clone();
            f8(f16(f8(v0, f33(v4.clone()))?, v5)?, f34(v4))
        }
        Adt2::C3(v6, r70, r71) => {
            let v7 = (*r70).clone();
            let v8 = (*r71).clone();
            let m66 = v6;
            match m66 {
                Adt6::C0 => {
                    f8(f16(f8(f16(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt6::C1 => {
                    f8(f16(f8(f16(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt2::C4(v9, v10) => {
            let m67 = v9;
            match m67 {
                Adt7::C0 => {
                    f19(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt7::C1 => {
                    f19(f8(v0, Str::lit("Fn"))?, v10)
                }
                Adt7::C2 => {
                    f35(v0, v10)
                }
            }
        }
        Adt2::C5 => {
            Ok(v0)
        }
        Adt2::C6(r72, r73) => {
            let v11 = (*r72).clone();
            let v12 = (*r73).clone();
            let m68 = v12.clone();
            match m68 {
                Adt2::C0(_) => {
                    f16(v0, v11)
                }
                Adt2::C1(_) => {
                    f16(v0, v11)
                }
                Adt2::C2(_, _) => {
                    f16(v0, v11)
                }
                Adt2::C3(_, _, _) => {
                    f16(v0, v11)
                }
                Adt2::C4(_, _) => {
                    f16(v0, v11)
                }
                Adt2::C5 => {
                    f16(v0, v11)
                }
                Adt2::C6(_, _) => {
                    f16(f8(f16(v0, v11)?, Str::lit(", "))?, v12)
                }
            }
        }
    }
}

pub fn f17(v0: Adt2) -> bool {
    let m74 = v0;
    match m74 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(v1, _) => {
            let m75 = v1;
            match m75 {
                Adt7::C0 => {
                    false
                }
                Adt7::C1 => {
                    true
                }
                Adt7::C2 => {
                    false
                }
            }
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f18(v0: Adt10) -> bool {
    let m76 = v0;
    match m76 {
        Adt10::C0 => {
            true
        }
        Adt10::C1(_, _, _, _) => {
            false
        }
    }
}

pub fn f19(v0: Adt26, v1: u64) -> R<Adt26> {
    f36(v0, v1)
}

pub fn f20(p85: Adt26, p86: Adt2, p87: u64, v3: bool) -> R<Adt26> {
    let m84 = (p85, (p86, p87));
    {
        let mut m84 = m84;
        loop {
            let step = {
                let (v0, (v1, v2)) = m84;
                let m77 = v1;
                match m77 {
                    Adt2::C0(_) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C2(_, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C4(_, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C5 => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C6(r82, r83) => {
                        let v4 = (*r82).clone();
                        let v5 = (*r83).clone();
                        let v6: Adt26 = {
                            let a78 = v2;
                            let a79 = 0u64;
                            if nat_eq(a78, a79) {
                                v0
                            } else {
                                f8(v0, Str::lit(", "))?
                            }
                        };
                        let v7: Adt26 = f19(f8(if v3 {
                            f8(v6, Str::lit("_"))?
                        } else {
                            v6
                        }, Str::lit("p"))?, v2)?;
                        Ok::<(Adt26, (Adt2, u64)), R<Adt26>>((f16(f8(v7, Str::lit(": "))?, v4)?, (v5, {
                            let a80 = v2;
                            let a81 = 1u64;
                            nat_add(a80, a81)?
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt26, v1: Adt2) -> R<Adt26> {
    if f24(v1.clone()) {
        Ok(v0)
    } else {
        f16(f8(v0, Str::lit(" -> "))?, v1)
    }
}

pub fn f22(p99: Adt26, p100: Adt10, v2: u64, v3: u64, v4: bool, v5: bool) -> R<Adt26> {
    let m98 = (p99, p100);
    {
        let mut m98 = m98;
        loop {
            let step = {
                let (v0, v1) = m98;
                let m88 = v1;
                match m88 {
                    Adt10::C0 => {
                        Err::<(Adt26, Adt10), R<Adt26>>(Ok(v0))
                    }
                    Adt10::C1(v6, v7, v8, r97) => {
                        let v9 = (*r97).clone();
                        let v10: u64 = {
                            let a89 = v7.clone();
                            length_list(a89)
                        };
                        let v11: Adt26 = f19(f8(f19(f8(v0, Str::lit("            Fn"))?, v2)?, Str::lit("::F"))?, v6)?;
                        let v12: Adt26 = {
                            let a90 = v10;
                            let a91 = 0u64;
                            if nat_eq(a90, a91) {
                                v11
                            } else {
                                f8({
                                    let (_, h92) = f37(Fn2::F38, (0u64, f8(v11, Str::lit("("))?), v7.clone())?;
                                    h92
                                }, Str::lit(")"))?
                            }
                        };
                        let v13: bool = if v5 {
                            let a93 = v8;
                            bool_not(a93)
                        } else {
                            false
                        };
                        let v14: Adt26 = f8(v12, Str::lit(" => "))?;
                        let v15: Adt26 = if v13 {
                            if v4 {
                                f8(v14, Str::lit("{ let () = "))?
                            } else {
                                f8(v14, Str::lit("Ok("))?
                            }
                        } else {
                            v14
                        };
                        let v16: Adt26 = f8(f19(f8(v15, Str::lit("f"))?, v6)?, Str::lit("("))?;
                        let v17: Adt26 = {
                            let (_, h94) = f37(Fn2::F39, (0u64, v16), v7)?;
                            h94
                        };
                        let v18: Adt26 = {
                            let (_, h95) = {
                                let (h96, _) = f40(Fn3::F41(v10, v3), v3, (0u64, v17))?;
                                h96
                            };
                            h95
                        };
                        let v19: Adt26 = f8(v18, Str::lit(")"))?;
                        let v20: Adt26 = if v13 {
                            if v4 {
                                f8(v19, Str::lit("; Ok(()) }"))?
                            } else {
                                f8(v19, Str::lit(")"))?
                            }
                        } else {
                            v19
                        };
                        Ok::<(Adt26, Adt10), R<Adt26>>((f8(v20, Str::lit(",\u{a}"))?, v9))
                    }
                }
            };
            match step {
                Ok(next) => { m98 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt2) -> R<u64> {
    let m101 = v0;
    match m101 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _) => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_, r104) => {
            let v11 = (*r104).clone();
            let v12: u64 = f23(v11)?;
            let a102 = 1u64;
            let a103 = v12;
            nat_add(a102, a103)
        }
    }
}

pub fn f24(v0: Adt2) -> bool {
    let m105 = v0;
    match m105 {
        Adt2::C0(v1) => {
            let m106 = v1;
            match m106 {
                Adt3::C0 => {
                    true
                }
                Adt3::C1 => {
                    false
                }
                Adt3::C2 => {
                    false
                }
                Adt3::C3 => {
                    false
                }
                Adt3::C4 => {
                    false
                }
                Adt3::C5 => {
                    false
                }
                Adt3::C6 => {
                    false
                }
                Adt3::C7 => {
                    false
                }
                Adt3::C8 => {
                    false
                }
            }
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f25(v0: Adt26, v1: Adt18, v2: u64, v3: u64, v4: u64) -> R<Adt26> {
    let m107 = v1;
    match m107 {
        Adt18::C0(r166, _) => {
            let v5 = (*r166).clone();
            f42(v0, v5)
        }
        Adt18::C1(v7, r167, _) => {
            let v8 = (*r167).clone();
            let m108 = v7;
            match m108 {
                Adt20::C0 => {
                    f28(v0, v8)
                }
                Adt20::C1 => {
                    f8(f28(v0, v8)?, Str::lit(".clone()"))
                }
                Adt20::C2 => {
                    f28(v0, v8)
                }
                Adt20::C3 => {
                    f28(f8(v0, Str::lit("*"))?, v8)
                }
                Adt20::C4 => {
                    f8(f28(f8(v0, Str::lit("(*"))?, v8)?, Str::lit(").clone()"))
                }
            }
        }
        Adt18::C2(v10, r168, _) => {
            let v11 = (*r168).clone();
            let m109 = v10;
            match m109 {
                Adt21::C0 => {
                    let a110 = f43(v11.clone());
                    let a111 = 0u64;
                    if if nat_eq(a110, a111) {
                        true
                    } else {
                        let a112 = f43(v11.clone());
                        let a113 = 1u64;
                        if nat_eq(a112, a113) {
                            true
                        } else {
                            let a114 = f43(v11.clone());
                            let a115 = 3u64;
                            nat_eq(a114, a115)
                        }
                    } {
                        f8(f25(f8(v0, Str::lit("!("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        f25(f8(v0, Str::lit("!"))?, v11, v2, 100u64, 0u64)
                    }
                }
                Adt21::C1 => {
                    f8(f25(f8(v0, Str::lit("std::rc::Rc::new("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt21::C2 => {
                    f8(f25(f8(v0, Str::lit("i128::from("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt21::C3 => {
                    f8(f25(f8(v0, Str::lit("Ok("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
            }
        }
        Adt18::C3(r169, r170, v15, _) => {
            let v13 = (*r169).clone();
            let v14 = (*r170).clone();
            let v17: u64 = f44(v13.clone());
            let v22: Adt26 = {
                let m116 = v13.clone();
                match m116 {
                    Adt22::C0(v18) => {
                        f8(f19(f8(v0, Str::lit("f"))?, v18)?, Str::lit("("))?
                    }
                    Adt22::C1(r117) => {
                        let v19 = (*r117).clone();
                        f8(f45(v0, v19)?, Str::lit("("))?
                    }
                    Adt22::C2(v20, v21) => {
                        f46(v0, v20, v21)?
                    }
                }
            };
            let v23: Adt26 = f25(v22, v14, v2, v17, 0u64)?;
            let v28: Adt26 = {
                let m118 = v13;
                match m118 {
                    Adt22::C0(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt22::C1(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt22::C2(v26, _) => {
                        f47(v23, v26)?
                    }
                }
            };
            if v15 {
                f8(v28, Str::lit("?"))
            } else {
                Ok(v28)
            }
        }
        Adt18::C4(r171, r172, v31, _) => {
            let v29 = (*r171).clone();
            let v30 = (*r172).clone();
            let v33: Adt26 = f8(f25(f8(f28(v0, v29)?, Str::lit(".apply("))?, v30, v2, 100u64, 0u64)?, Str::lit(")"))?;
            if v31 {
                f8(v33, Str::lit("?"))
            } else {
                Ok(v33)
            }
        }
        Adt18::C5(r173, r174, _) => {
            let v34 = (*r173).clone();
            let v35 = (*r174).clone();
            let v37: Adt26 = f48(v0, v34.clone())?;
            let a119 = f49(v34.clone());
            let a120 = 1u64;
            if nat_eq(a119, a120) {
                f8(f25(v37, v35, v2, 100u64, 0u64)?, Str::lit(")"))
            } else {
                let a121 = f49(v34);
                let a122 = 2u64;
                if nat_eq(a121, a122) {
                    if f50(v35.clone()) {
                        f8(f25(f8(v37, Str::lit("("))?, v35, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        Ok(v37)
                    }
                } else {
                    Ok(v37)
                }
            }
        }
        Adt18::C6(r175, r176, _) => {
            let v38 = (*r175).clone();
            let v39 = (*r176).clone();
            f8(f25(f8(f25(f8(v0, Str::lit("("))?, v38, v2, 100u64, 0u64)?, Str::lit(", "))?, v39, v2, 100u64, 0u64)?, Str::lit(")"))
        }
        Adt18::C7(r177, r178, r179, _) => {
            let v41 = (*r177).clone();
            let v42 = (*r178).clone();
            let v43 = (*r179).clone();
            let v45: Adt26 = f25(f8(f25(f8(v0, Str::lit("if "))?, v41, v2, 100u64, 0u64)?, Str::lit(" "))?, v42, v2, 100u64, 0u64)?;
            if f51(v43.clone()) {
                Ok(v45)
            } else {
                f25(f8(v45, Str::lit(" else "))?, v43, v2, 100u64, 0u64)
            }
        }
        Adt18::C8(r180, r181, _) => {
            let v46 = (*r180).clone();
            let v47 = (*r181).clone();
            let a123 = f52(v47.clone());
            let a124 = 15u64;
            if nat_eq(a123, a124) {
                if f53(v46.clone()) {
                    let v49: Adt26 = f25(f8(f25(f8(v0, Str::lit("{ let value = "))?, v46, v2, 100u64, 0u64)?, Str::lit("; match value {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f54(v49, v2)?, Str::lit("} }"))
                } else {
                    let v50: Adt26 = f25(f8(f25(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f54(v50, v2)?, Str::lit("}"))
                }
            } else {
                f8(f25(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {}"))
            }
        }
        Adt18::C9(r182) => {
            let v51 = (*r182).clone();
            f25(v0, v51, v2, v3, v4)
        }
        Adt18::C10(v52, r183, _) => {
            let v53 = (*r183).clone();
            let m125 = v52;
            match m125 {
                Adt25::C0 => {
                    f8(f28(v0, v53)?, Str::lit(" == 0"))
                }
                Adt25::C1 => {
                    f8(f28(v0, v53)?, Str::lit(" != 0"))
                }
                Adt25::C2 => {
                    f8(f28(v0, v53)?, Str::lit(".uncons()"))
                }
                Adt25::C3 => {
                    f8(f28(v0, v53)?, Str::lit(" - 1"))
                }
            }
        }
        Adt18::C11(r184, r185) => {
            let v55 = (*r184).clone();
            let v56 = (*r185).clone();
            let a126 = f52(v55.clone());
            let a127 = 12u64;
            if if nat_eq(a126, a127) {
                f55(v56.clone())
            } else {
                false
            } {
                f8(v0, Str::lit("{}"))
            } else {
                let v57: Adt26 = f25(f8(v0, Str::lit("{\u{a}"))?, v55, v2, 100u64, 0u64)?;
                let v58: Adt26 = if f55(v56.clone()) {
                    v57
                } else {
                    f8(f25(f54(v57, {
                        let a128 = v2;
                        let a129 = 1u64;
                        nat_add(a128, a129)?
                    })?, v56, {
                        let a130 = v2;
                        let a131 = 1u64;
                        nat_add(a130, a131)?
                    }, 100u64, 0u64)?, Str::lit("\u{a}"))?
                };
                f8(f54(v58, v2)?, Str::lit("}"))
            }
        }
        Adt18::C12 => {
            Ok(v0)
        }
        Adt18::C13(r186, r187) => {
            let v59 = (*r186).clone();
            let v60 = (*r187).clone();
            let a132 = v4;
            let a133 = f56(v3);
            if nat_lt(a132, a133) {
                let v61: Adt26 = f25(v0, v59, v2, 100u64, 0u64)?;
                if f50(v60.clone()) {
                    let a136 = {
                        let a134 = v4;
                        let a135 = 1u64;
                        nat_add(a134, a135)?
                    };
                    let a137 = f56(v3);
                    if nat_lt(a136, a137) {
                        f25(f57(v61, v3, v4)?, v60, v2, v3, {
                            let a138 = v4;
                            let a139 = 1u64;
                            nat_add(a138, a139)?
                        })
                    } else {
                        Ok(v61)
                    }
                } else {
                    Ok(v61)
                }
            } else {
                Ok(v0)
            }
        }
        Adt18::C14(r188, v63, r189, r190, _, r191) => {
            let v62 = (*r188).clone();
            let v64 = (*r189).clone();
            let v65 = (*r190).clone();
            let v67 = (*r191).clone();
            let v68: Adt26 = f58(f8(f54(v0, {
                let a140 = v2;
                let a141 = 1u64;
                nat_add(a140, a141)?
            })?, Str::lit("let "))?, v62)?;
            let v69: Adt26 = if v63 {
                f16(f8(v68, Str::lit(": "))?, v64)?
            } else {
                v68
            };
            let v70: Adt26 = f8(f25(f8(v69, Str::lit(" = "))?, v65, {
                let a142 = v2;
                let a143 = 1u64;
                nat_add(a142, a143)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            f25(v70, v67, v2, 100u64, 0u64)
        }
        Adt18::C15(r192, r193, r194) => {
            let v71 = (*r192).clone();
            let v72 = (*r193).clone();
            let v73 = (*r194).clone();
            let v74: Adt26 = f8(f25(f8(f58(f54(v0, {
                let a144 = v2;
                let a145 = 1u64;
                nat_add(a144, a145)?
            })?, v71)?, Str::lit(" => "))?, v72, {
                let a146 = v2;
                let a147 = 1u64;
                nat_add(a146, a147)?
            }, 100u64, 0u64)?, Str::lit("\u{a}"))?;
            f25(v74, v73, v2, 100u64, 0u64)
        }
        Adt18::C16(r195, r196) => {
            let v75 = (*r195).clone();
            let v76 = (*r196).clone();
            let v77: Adt26 = f28(f8(f54(f8(v0, Str::lit("{\u{a}"))?, {
                let a148 = v2;
                let a149 = 1u64;
                nat_add(a148, a149)?
            })?, Str::lit("let mut "))?, v75.clone())?;
            let v78: Adt26 = f8(f28(f8(v77, Str::lit(" = "))?, v75.clone())?, Str::lit(";\u{a}"))?;
            let v79: Adt26 = f8(f54(v78, {
                let a150 = v2;
                let a151 = 1u64;
                nat_add(a150, a151)?
            })?, Str::lit("loop {\u{a}"))?;
            let v80: Adt26 = f8(f25(f8(f54(v79, {
                let a152 = v2;
                let a153 = 2u64;
                nat_add(a152, a153)?
            })?, Str::lit("let step = "))?, v76, {
                let a154 = v2;
                let a155 = 2u64;
                nat_add(a154, a155)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            let v81: Adt26 = f8(f54(v80, {
                let a156 = v2;
                let a157 = 2u64;
                nat_add(a156, a157)?
            })?, Str::lit("match step {\u{a}"))?;
            let v82: Adt26 = f8(f28(f8(f54(v81, {
                let a158 = v2;
                let a159 = 3u64;
                nat_add(a158, a159)?
            })?, Str::lit("Ok(next) => { "))?, v75)?, Str::lit(" = next; }\u{a}"))?;
            let v83: Adt26 = f8(f54(v82, {
                let a160 = v2;
                let a161 = 3u64;
                nat_add(a160, a161)?
            })?, Str::lit("Err(done) => break done,\u{a}"))?;
            f8(f54(f8(f54(f8(f54(v83, {
                let a162 = v2;
                let a163 = 2u64;
                nat_add(a162, a163)?
            })?, Str::lit("}\u{a}"))?, {
                let a164 = v2;
                let a165 = 1u64;
                nat_add(a164, a165)?
            })?, Str::lit("}\u{a}"))?, v2)?, Str::lit("}"))
        }
    }
}

pub fn f26(p202: Adt26, p203: Adt13, p204: bool) -> R<Adt26> {
    let m201 = (p202, (p203, p204));
    {
        let mut m201 = m201;
        loop {
            let step = {
                let (v0, (v1, v2)) = m201;
                let m197 = v1;
                match m197 {
                    Adt13::C0 => {
                        Err::<(Adt26, (Adt13, bool)), R<Adt26>>(Ok(v0))
                    }
                    Adt13::C1(r198, r199, r200) => {
                        let v3 = (*r198).clone();
                        let v4 = (*r199).clone();
                        let v5 = (*r200).clone();
                        Ok::<(Adt26, (Adt13, bool)), R<Adt26>>((f16(f8(f58(if v2 {
                            v0
                        } else {
                            f8(v0, Str::lit(", "))?
                        }, v3)?, Str::lit(": "))?, v4)?, (v5, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt26, v1: Adt13, v2: Adt2) -> R<Adt26> {
    let v3: u64 = f59(f60(v1.clone())?, f61(v2.clone())?);
    let v4: ((u64, (Adt26, bool)), bool) = f62(Fn4::F63(v3, std::rc::Rc::new(v1), std::rc::Rc::new(v2)), v3, (0u64, (v0, false)))?;
    let v5: (Adt26, bool) = {
        let (_, h205) = {
            let (h206, _) = v4;
            h206
        };
        h205
    };
    let (_, h207) = v5;
    if h207 {
        f8({
            let (h208, _) = v5;
            h208
        }, Str::lit(">"))
    } else {
        let (h209, _) = v5;
        Ok(h209)
    }
}

pub fn f28(v0: Adt26, v1: Adt12) -> R<Adt26> {
    let m210 = v1;
    match m210 {
        Adt12::C0(v2) => {
            f19(f8(v0, Str::lit("v"))?, v2)
        }
        Adt12::C1(v3) => {
            f19(f8(v0, Str::lit("m"))?, v3)
        }
        Adt12::C2(v4) => {
            f19(f8(v0, Str::lit("a"))?, v4)
        }
        Adt12::C3(v5) => {
            f19(f8(v0, Str::lit("c"))?, v5)
        }
        Adt12::C4(v6) => {
            f19(f8(v0, Str::lit("r"))?, v6)
        }
        Adt12::C5(v7) => {
            f19(f8(v0, Str::lit("h"))?, v7)
        }
        Adt12::C6(v8) => {
            f19(f8(v0, Str::lit("k"))?, v8)
        }
        Adt12::C7(v9) => {
            f19(f8(v0, Str::lit("p"))?, v9)
        }
        Adt12::C8(v10) => {
            f19(f8(v0, Str::lit("f"))?, v10)
        }
        Adt12::C9(v11) => {
            f3(v0, v11)
        }
    }
}

pub fn f29(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m211 = v0.clone();
    match m211.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m212 = v2;
            match m212.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f64(Fn5::F65, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m214 = {
                        let (h213, _) = v5;
                        h213
                    };
                    match m214 {
                        None => {
                            Some(f6({
                                let (_, h216) = v5;
                                h216
                            }))
                        }
                        Some(v6) => {
                            Some(f6(List::cons(v6, {
                                let (_, h215) = v5;
                                h215
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: Adt2) -> bool {
    let m217 = v0;
    match m217 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            true
        }
    }
}

pub fn f31(v0: Adt26, v1: Adt3) -> R<Adt26> {
    f8(v0, {
        let m218 = v1;
        match m218 {
            Adt3::C0 => {
                Str::lit("()")
            }
            Adt3::C1 => {
                Str::lit("bool")
            }
            Adt3::C2 => {
                Str::lit("u64")
            }
            Adt3::C3 => {
                Str::lit("i64")
            }
            Adt3::C4 => {
                Str::lit("Ordering")
            }
            Adt3::C5 => {
                Str::lit("Str")
            }
            Adt3::C6 => {
                Str::lit("Bytes")
            }
            Adt3::C7 => {
                Str::lit("Overflow")
            }
            Adt3::C8 => {
                Str::lit("i128")
            }
        }
    })
}

pub fn f32(v0: Adt26, v1: Adt4) -> R<Adt26> {
    f8(v0, f66(v1))
}

pub fn f33(v0: Adt5) -> Str {
    let m219 = v0;
    match m219 {
        Adt5::C0 => {
            Str::lit("Option<")
        }
        Adt5::C1 => {
            Str::lit("List<")
        }
        Adt5::C2 => {
            Str::lit("std::rc::Rc<")
        }
        Adt5::C3 => {
            Str::lit("R<")
        }
        Adt5::C4 => {
            Str::lit("&")
        }
    }
}

pub fn f34(v0: Adt5) -> Str {
    let m220 = v0;
    match m220 {
        Adt5::C0 => {
            Str::lit(">")
        }
        Adt5::C1 => {
            Str::lit(">")
        }
        Adt5::C2 => {
            Str::lit(">")
        }
        Adt5::C3 => {
            Str::lit(">")
        }
        Adt5::C4 => {
            Str::lit("")
        }
    }
}

pub fn f35(v0: Adt26, v1: u64) -> R<Adt26> {
    let a221 = v1;
    let a222 = 0u64;
    if nat_eq(a221, a222) {
        f8(v0, Str::lit("T"))
    } else {
        let a223 = v1;
        let a224 = 1u64;
        if nat_eq(a223, a224) {
            f8(v0, Str::lit("U"))
        } else {
            let a225 = v1;
            let a226 = 2u64;
            if nat_eq(a225, a226) {
                f8(v0, Str::lit("V"))
            } else {
                f19(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f36(v0: Adt26, v1: u64) -> R<Adt26> {
    f3(v0, f67(v1)?)
}

pub fn f37(v0: Fn2, p230: (u64, Adt26), p231: List<Adt11>) -> R<(u64, Adt26)> {
    let m229 = (p230, p231);
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v1, v2) = m229;
                let m227 = v2;
                match m227.uncons() {
                    None => {
                        Err::<((u64, Adt26), List<Adt11>), R<(u64, Adt26)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Adt26), List<Adt11>), R<(u64, Adt26)>>(({
                            let c228: Fn2 = v0.clone();
                            c228.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    f68(v0, v1)
}

pub fn f39(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    f69(v0, v1)
}

pub fn f40(v0: Fn3, p236: u64, p237: (u64, Adt26)) -> R<((u64, Adt26), bool)> {
    let m235 = (p236, p237);
    {
        let mut m235 = m235;
        loop {
            let step = {
                let (v1, v2) = m235;
                let m232 = v1;
                if m232 == 0 {
                    Err::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m232 - 1;
                    let m234 = {
                        let c233: Fn3 = v0.clone();
                        c233.apply(v2.clone())?
                    };
                    match m234 {
                        None => {
                            Err::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: u64, v1: u64, v2: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f70(v1, v0, v2)
}

pub fn f42(v0: Adt26, v1: Adt19) -> R<Adt26> {
    let m238 = v1;
    match m238 {
        Adt19::C0 => {
            f8(v0, Str::lit("()"))
        }
        Adt19::C1(v2) => {
            if v2 {
                f8(v0, Str::lit("true"))
            } else {
                f8(v0, Str::lit("false"))
            }
        }
        Adt19::C2(v3) => {
            f8(f19(v0, v3)?, Str::lit("u64"))
        }
        Adt19::C3(v4, v5) => {
            if if v4 {
                let a239 = v5;
                let a240 = 9223372036854775808u64;
                nat_eq(a239, a240)
            } else {
                false
            } {
                f8(v0, Str::lit("i64::MIN"))
            } else {
                f8(f19(if v4 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v5)?, Str::lit("i64"))
            }
        }
        Adt19::C4(v6, v7, v8) => {
            if if v7 {
                let a241 = v8;
                let a242 = f71(v6.clone());
                nat_eq(a241, a242)
            } else {
                false
            } {
                f8(f32(v0, v6)?, Str::lit("::MIN"))
            } else {
                f32(f19(if v7 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v8)?, v6)
            }
        }
        Adt19::C5(v9) => {
            f8(f72(f8(v0, Str::lit("Str::lit("))?, v9)?, Str::lit(")"))
        }
        Adt19::C6(v10) => {
            f73(v0, v10)
        }
        Adt19::C7(v11) => {
            f74(v0, v11)
        }
        Adt19::C8 => {
            f8(v0, Str::lit("Overflow"))
        }
    }
}

pub fn f43(v0: Adt18) -> u64 {
    let m243 = v0;
    match m243 {
        Adt18::C0(_, _) => {
            1000u64
        }
        Adt18::C1(_, _, _) => {
            1000u64
        }
        Adt18::C2(_, _, _) => {
            1000u64
        }
        Adt18::C3(_, _, _, _) => {
            1000u64
        }
        Adt18::C4(_, _, _, _) => {
            1000u64
        }
        Adt18::C5(_, _, _) => {
            1000u64
        }
        Adt18::C6(_, _, _) => {
            1000u64
        }
        Adt18::C7(_, _, _, _) => {
            1000u64
        }
        Adt18::C8(_, _, _) => {
            1000u64
        }
        Adt18::C9(_) => {
            1000u64
        }
        Adt18::C10(v31, _, _) => {
            let m244 = v31;
            match m244 {
                Adt25::C0 => {
                    0u64
                }
                Adt25::C1 => {
                    1u64
                }
                Adt25::C2 => {
                    2u64
                }
                Adt25::C3 => {
                    3u64
                }
            }
        }
        Adt18::C11(_, _) => {
            1000u64
        }
        Adt18::C12 => {
            1000u64
        }
        Adt18::C13(_, _) => {
            1000u64
        }
        Adt18::C14(_, _, _, _, _, _) => {
            1000u64
        }
        Adt18::C15(_, _, _) => {
            1000u64
        }
        Adt18::C16(_, _) => {
            1000u64
        }
    }
}

pub fn f44(v0: Adt22) -> u64 {
    let m245 = v0;
    match m245 {
        Adt22::C0(_) => {
            1000u64
        }
        Adt22::C1(_) => {
            1000u64
        }
        Adt22::C2(v3, _) => {
            v3
        }
    }
}

pub fn f45(v0: Adt26, v1: Adt23) -> R<Adt26> {
    let v2: u64 = match v1.clone() {
        Adt23::C0(h246, _) => {
            h246
        }
    };
    let m248 = match v1 {
        Adt23::C0(_, h247) => {
            h247
        }
    };
    match m248 {
        None => {
            f8(v0, {
                let a255 = v2;
                let a256 = 20u64;
                if nat_lt(a255, a256) {
                    f76(v2)
                } else {
                    f77(v2)
                }
            })
        }
        Some(v3) => {
            let a249 = v2;
            let a250 = 24u64;
            if nat_eq(a249, a250) {
                f32(f8(v0, Str::lit("checked_neg_"))?, v3)
            } else {
                let a251 = v2;
                let a252 = 47u64;
                if nat_eq(a251, a252) {
                    f32(f8(v0, Str::lit("format_"))?, v3)
                } else {
                    let a253 = v2;
                    let a254 = 49u64;
                    if nat_eq(a253, a254) {
                        f32(f8(v0, Str::lit("parse_"))?, v3)
                    } else {
                        f8(f8(f32(f8(v0, Str::lit("fixed_"))?, v3)?, Str::lit("::"))?, f75(v2))
                    }
                }
            }
        }
    }
}

pub fn f46(v0: Adt26, v1: u64, v2: Option<Adt4>) -> R<Adt26> {
    let a257 = v1;
    let a258 = 8u64;
    if nat_lt(a257, a258) {
        let m259 = v2;
        match m259 {
            None => {
                Ok(v0)
            }
            Some(v3) => {
                f8(f8(f8(f32(v0, v3)?, Str::lit("::"))?, {
                    let a260 = v1;
                    let a261 = 7u64;
                    if nat_eq(a260, a261) {
                        Str::lit("try_from")
                    } else {
                        f78(v1)
                    }
                })?, Str::lit("("))
            }
        }
    } else {
        let a262 = v1;
        let a263 = 40u64;
        if nat_eq(a262, a263) {
            let m264 = v2;
            match m264 {
                None => {
                    Ok(v0)
                }
                Some(v4) => {
                    f8(f32(f8(v0, Str::lit("Str::format_"))?, v4)?, Str::lit("("))
                }
            }
        } else {
            let a265 = v1;
            let a266 = 41u64;
            if nat_eq(a265, a266) {
                let m267 = v2;
                match m267 {
                    None => {
                        Ok(v0)
                    }
                    Some(v5) => {
                        f8(f32(f8(v0, Str::lit("Str::parse_"))?, v5)?, Str::lit("(&"))
                    }
                }
            } else {
                f8(v0, f79(v1))
            }
        }
    }
}

pub fn f47(v0: Adt26, v1: u64) -> R<Adt26> {
    let a268 = v1;
    let a269 = 8u64;
    if nat_lt(a268, a269) {
        f8(v0, {
            let a270 = v1;
            let a271 = 7u64;
            if nat_eq(a270, a271) {
                Str::lit(").ok()")
            } else {
                Str::lit(")")
            }
        })
    } else {
        f8(v0, f80(v1))
    }
}

pub fn f48(v0: Adt26, v1: Adt24) -> R<Adt26> {
    let m272 = v1;
    match m272 {
        Adt24::C0(r273) => {
            let v2 = (*r273).clone();
            f8(f16(f8(v0, Str::lit("None::<"))?, v2)?, Str::lit(">"))
        }
        Adt24::C1 => {
            f8(v0, Str::lit("Some("))
        }
        Adt24::C2(r274, r275) => {
            let v3 = (*r274).clone();
            let v4 = (*r275).clone();
            f8(f16(f8(f16(f8(v0, Str::lit("Ok::<"))?, v3)?, Str::lit(", "))?, v4)?, Str::lit(">("))
        }
        Adt24::C3(r276, r277) => {
            let v5 = (*r276).clone();
            let v6 = (*r277).clone();
            f8(f16(f8(f16(f8(v0, Str::lit("Err::<"))?, v5)?, Str::lit(", "))?, v6)?, Str::lit(">("))
        }
        Adt24::C4(v7, v8) => {
            f19(f8(f19(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)
        }
        Adt24::C5(v9, v10) => {
            f19(f8(f19(f8(v0, Str::lit("Fn"))?, v9)?, Str::lit("::F"))?, v10)
        }
        Adt24::C6 => {
            f8(v0, Str::lit("List::cons("))
        }
        Adt24::C7(r278) => {
            let v11 = (*r278).clone();
            f8(f16(f8(v0, Str::lit("List::<"))?, v11)?, Str::lit(">::nil()"))
        }
    }
}

pub fn f49(v0: Adt24) -> u64 {
    let m279 = v0;
    match m279 {
        Adt24::C0(_) => {
            0u64
        }
        Adt24::C1 => {
            1u64
        }
        Adt24::C2(_, _) => {
            1u64
        }
        Adt24::C3(_, _) => {
            1u64
        }
        Adt24::C4(_, _) => {
            2u64
        }
        Adt24::C5(_, _) => {
            2u64
        }
        Adt24::C6 => {
            1u64
        }
        Adt24::C7(_) => {
            0u64
        }
    }
}

pub fn f50(v0: Adt18) -> bool {
    let a280 = f52(v0);
    let a281 = 13u64;
    nat_eq(a280, a281)
}

pub fn f51(v0: Adt18) -> bool {
    let m282 = v0;
    match m282 {
        Adt18::C0(_, _) => {
            false
        }
        Adt18::C1(_, _, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_, _, _, _) => {
            false
        }
        Adt18::C4(_, _, _, _) => {
            false
        }
        Adt18::C5(_, _, _) => {
            false
        }
        Adt18::C6(_, _, _) => {
            false
        }
        Adt18::C7(_, _, _, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
        Adt18::C9(_) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
        Adt18::C11(r285, r286) => {
            let v34 = (*r285).clone();
            let v35 = (*r286).clone();
            let a283 = f52(v34);
            let a284 = 12u64;
            if nat_eq(a283, a284) {
                f55(v35)
            } else {
                false
            }
        }
        Adt18::C12 => {
            false
        }
        Adt18::C13(_, _) => {
            false
        }
        Adt18::C14(_, _, _, _, _, _) => {
            false
        }
        Adt18::C15(_, _, _) => {
            false
        }
        Adt18::C16(_, _) => {
            false
        }
    }
}

pub fn f52(v0: Adt18) -> u64 {
    let m287 = v0;
    match m287 {
        Adt18::C0(_, _) => {
            0u64
        }
        Adt18::C1(_, _, _) => {
            1u64
        }
        Adt18::C2(_, _, _) => {
            2u64
        }
        Adt18::C3(_, _, _, _) => {
            3u64
        }
        Adt18::C4(_, _, _, _) => {
            4u64
        }
        Adt18::C5(_, _, _) => {
            5u64
        }
        Adt18::C6(_, _, _) => {
            6u64
        }
        Adt18::C7(_, _, _, _) => {
            7u64
        }
        Adt18::C8(_, _, _) => {
            8u64
        }
        Adt18::C9(_) => {
            9u64
        }
        Adt18::C10(_, _, _) => {
            10u64
        }
        Adt18::C11(_, _) => {
            11u64
        }
        Adt18::C12 => {
            12u64
        }
        Adt18::C13(_, _) => {
            13u64
        }
        Adt18::C14(_, _, _, _, _, _) => {
            14u64
        }
        Adt18::C15(_, _, _) => {
            15u64
        }
        Adt18::C16(_, _) => {
            16u64
        }
    }
}

pub fn f53(p299: Adt18) -> bool {
    let m298 = p299;
    {
        let mut m298 = m298;
        loop {
            let step = {
                let v0 = m298;
                let m288 = v0;
                match m288 {
                    Adt18::C0(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C2(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C3(r295, _, _, _) => {
                        let v9 = (*r295).clone();
                        let a289 = f44(v9.clone());
                        let a290 = 16u64;
                        if nat_eq(a289, a290) {
                            Err::<Adt18, bool>(true)
                        } else {
                            let a291 = f44(v9);
                            let a292 = 25u64;
                            Err::<Adt18, bool>(nat_eq(a291, a292))
                        }
                    }
                    Adt18::C4(_, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C5(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C6(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C7(_, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C8(r296, r297, _) => {
                        let v27 = (*r296).clone();
                        let v28 = (*r297).clone();
                        let a293 = f52(v28);
                        let a294 = 15u64;
                        if nat_eq(a293, a294) {
                            Ok::<Adt18, bool>(v27)
                        } else {
                            Err::<Adt18, bool>(false)
                        }
                    }
                    Adt18::C9(_) => {
                        Err::<Adt18, bool>(true)
                    }
                    Adt18::C10(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C11(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C12 => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C13(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C14(_, _, _, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C15(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C16(_, _) => {
                        Err::<Adt18, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt26, v1: u64) -> R<Adt26> {
    let (_, h300) = {
        let (h301, _) = f40(Fn3::F81(v1), v1, (0u64, v0))?;
        h301
    };
    Ok(h300)
}

pub fn f55(v0: Adt18) -> bool {
    let m302 = v0;
    match m302 {
        Adt18::C0(r304, _) => {
            let v1 = (*r304).clone();
            let m303 = v1;
            match m303 {
                Adt19::C0 => {
                    true
                }
                Adt19::C1(_) => {
                    false
                }
                Adt19::C2(_) => {
                    false
                }
                Adt19::C3(_, _) => {
                    false
                }
                Adt19::C4(_, _, _) => {
                    false
                }
                Adt19::C5(_) => {
                    false
                }
                Adt19::C6(_) => {
                    false
                }
                Adt19::C7(_) => {
                    false
                }
                Adt19::C8 => {
                    false
                }
            }
        }
        Adt18::C1(_, _, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_, _, _, _) => {
            false
        }
        Adt18::C4(_, _, _, _) => {
            false
        }
        Adt18::C5(_, _, _) => {
            false
        }
        Adt18::C6(_, _, _) => {
            false
        }
        Adt18::C7(_, _, _, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
        Adt18::C9(_) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
        Adt18::C11(_, _) => {
            false
        }
        Adt18::C12 => {
            false
        }
        Adt18::C13(_, _) => {
            false
        }
        Adt18::C14(_, _, _, _, _, _) => {
            false
        }
        Adt18::C15(_, _, _) => {
            false
        }
        Adt18::C16(_, _) => {
            false
        }
    }
}

pub fn f56(v0: u64) -> u64 {
    let a305 = v0;
    let a306 = 8u64;
    if if nat_lt(a305, a306) {
        true
    } else {
        let a307 = 45u64;
        let a308 = v0;
        nat_le(a307, a308)
    } {
        100000u64
    } else {
        let a309 = v0;
        let a310 = 11u64;
        if nat_eq(a309, a310) {
            1u64
        } else {
            let a311 = v0;
            let a312 = 24u64;
            if if nat_eq(a311, a312) {
                true
            } else {
                let a313 = v0;
                let a314 = 26u64;
                if nat_eq(a313, a314) {
                    true
                } else {
                    let a315 = v0;
                    let a316 = 31u64;
                    if nat_eq(a315, a316) {
                        true
                    } else {
                        let a317 = v0;
                        let a318 = 32u64;
                        if nat_eq(a317, a318) {
                            true
                        } else {
                            let a319 = v0;
                            let a320 = 35u64;
                            if nat_eq(a319, a320) {
                                true
                            } else {
                                let a321 = v0;
                                let a322 = 36u64;
                                if nat_eq(a321, a322) {
                                    true
                                } else {
                                    let a323 = v0;
                                    let a324 = 40u64;
                                    if nat_eq(a323, a324) {
                                        true
                                    } else {
                                        let a325 = v0;
                                        let a326 = 41u64;
                                        if nat_eq(a325, a326) {
                                            true
                                        } else {
                                            let a327 = v0;
                                            let a328 = 42u64;
                                            if nat_eq(a327, a328) {
                                                true
                                            } else {
                                                let a329 = v0;
                                                let a330 = 43u64;
                                                nat_eq(a329, a330)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                1u64
            } else {
                let a331 = v0;
                let a332 = 15u64;
                if nat_eq(a331, a332) {
                    3u64
                } else {
                    let a333 = v0;
                    let a334 = 16u64;
                    if nat_eq(a333, a334) {
                        3u64
                    } else {
                        let a335 = v0;
                        let a336 = 20u64;
                        if nat_eq(a335, a336) {
                            3u64
                        } else {
                            let a337 = v0;
                            let a338 = 25u64;
                            if if nat_eq(a337, a338) {
                                true
                            } else {
                                let a339 = v0;
                                let a340 = 29u64;
                                if nat_eq(a339, a340) {
                                    true
                                } else {
                                    let a341 = v0;
                                    let a342 = 34u64;
                                    if nat_eq(a341, a342) {
                                        true
                                    } else {
                                        let a343 = v0;
                                        let a344 = 38u64;
                                        if nat_eq(a343, a344) {
                                            true
                                        } else {
                                            let a345 = v0;
                                            let a346 = 44u64;
                                            nat_eq(a345, a346)
                                        }
                                    }
                                }
                            } {
                                3u64
                            } else {
                                2u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: Adt26, v1: u64, v2: u64) -> R<Adt26> {
    let a347 = v1;
    let a348 = 8u64;
    if nat_lt(a347, a348) {
        f8(v0, Str::lit(", "))
    } else {
        let a349 = v1;
        let a350 = 100u64;
        if nat_eq(a349, a350) {
            f8(v0, Str::lit(", "))
        } else {
            let a351 = v2;
            let a352 = 0u64;
            if nat_eq(a351, a352) {
                f8(v0, f82(v1))
            } else {
                f8(v0, f83(v1))
            }
        }
    }
}

pub fn f58(v0: Adt26, v1: Adt14) -> R<Adt26> {
    let m353 = v1;
    match m353 {
        Adt14::C0(v2) => {
            f8(v0, {
                let m354 = v2;
                match m354 {
                    Adt15::C0 => {
                        Str::lit("_")
                    }
                    Adt15::C1 => {
                        Str::lit("()")
                    }
                    Adt15::C2 => {
                        Str::lit("None")
                    }
                }
            })
        }
        Adt14::C1(r358) => {
            let v3 = (*r358).clone();
            f28(v0, v3)
        }
        Adt14::C2(r359) => {
            let v4 = (*r359).clone();
            f8(f58(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt14::C3(v5, r360) => {
            let v6 = (*r360).clone();
            f8(f58(f8(v0, {
                let m355 = v5;
                match m355 {
                    Adt16::C0 => {
                        Str::lit("Some(")
                    }
                    Adt16::C1 => {
                        Str::lit("Ok(")
                    }
                    Adt16::C2 => {
                        Str::lit("Err(")
                    }
                }
            })?, v6)?, Str::lit(")"))
        }
        Adt14::C4(v7, v8, r361) => {
            let v9 = (*r361).clone();
            let v10: Adt26 = f19(f8(f19(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m356 = v9.clone();
            match m356 {
                Adt14::C0(_) => {
                    Ok(v10)
                }
                Adt14::C1(_) => {
                    Ok(v10)
                }
                Adt14::C2(_) => {
                    Ok(v10)
                }
                Adt14::C3(_, _) => {
                    Ok(v10)
                }
                Adt14::C4(_, _, _) => {
                    Ok(v10)
                }
                Adt14::C5 => {
                    Ok(v10)
                }
                Adt14::C6(_, _) => {
                    f8(f58(f8(v10, Str::lit("("))?, v9)?, Str::lit(")"))
                }
                Adt14::C7(_) => {
                    Ok(v10)
                }
            }
        }
        Adt14::C5 => {
            Ok(v0)
        }
        Adt14::C6(r362, r363) => {
            let v22 = (*r362).clone();
            let v23 = (*r363).clone();
            let m357 = v23.clone();
            match m357 {
                Adt14::C0(_) => {
                    f58(v0, v22)
                }
                Adt14::C1(_) => {
                    f58(v0, v22)
                }
                Adt14::C2(_) => {
                    f58(v0, v22)
                }
                Adt14::C3(_, _) => {
                    f58(v0, v22)
                }
                Adt14::C4(_, _, _) => {
                    f58(v0, v22)
                }
                Adt14::C5 => {
                    f58(v0, v22)
                }
                Adt14::C6(_, _) => {
                    f58(f8(f58(v0, v22)?, Str::lit(", "))?, v23)
                }
                Adt14::C7(_) => {
                    f58(v0, v22)
                }
            }
        }
        Adt14::C7(v35) => {
            f74(v0, v35)
        }
    }
}

pub fn f59(v0: u64, v1: u64) -> u64 {
    let a364 = v0;
    let a365 = v1;
    if nat_lt(a364, a365) {
        v1
    } else {
        v0
    }
}

pub fn f60(v0: Adt13) -> R<u64> {
    let m366 = v0;
    match m366 {
        Adt13::C0 => {
            Ok(0u64)
        }
        Adt13::C1(_, r367, r368) => {
            let v2 = (*r367).clone();
            let v3 = (*r368).clone();
            Ok(f59(f61(v2)?, f60(v3)?))
        }
    }
}

pub fn f61(v0: Adt2) -> R<u64> {
    let m369 = v0;
    match m369 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, r373) => {
            let v6 = (*r373).clone();
            f61(v6)
        }
        Adt2::C3(_, r374, r375) => {
            let v8 = (*r374).clone();
            let v9 = (*r375).clone();
            Ok(f59(f61(v8)?, f61(v9)?))
        }
        Adt2::C4(v1, v2) => {
            let m370 = v1;
            match m370 {
                Adt7::C0 => {
                    Ok(0u64)
                }
                Adt7::C1 => {
                    Ok(0u64)
                }
                Adt7::C2 => {
                    let a371 = v2;
                    let a372 = 1u64;
                    nat_add(a371, a372)
                }
            }
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(r376, r377) => {
            let v10 = (*r376).clone();
            let v11 = (*r377).clone();
            Ok(f59(f61(v10)?, f61(v11)?))
        }
    }
}

pub fn f62(v0: Fn4, p382: u64, p383: (u64, (Adt26, bool))) -> R<((u64, (Adt26, bool)), bool)> {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v1, v2) = m381;
                let m378 = v1;
                if m378 == 0 {
                    Err::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m378 - 1;
                    let m380 = {
                        let c379: Fn4 = v0.clone();
                        c379.apply(v2.clone())?
                    };
                    match m380 {
                        None => {
                            Err::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt13, v2: Adt2, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    f84(v1, v2, v0, v3)
}

pub fn f64(v0: Fn5, p387: (Option<Bytes>, List<Bytes>), p388: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m384 = v2;
                match m384.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c385: Fn5 = v0.clone();
                            c385.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f85(v0, v1)
}

pub fn f66(v0: Adt4) -> Str {
    let m389 = v0;
    match m389 {
        Adt4::C0 => {
            Str::lit("u8")
        }
        Adt4::C1 => {
            Str::lit("u16")
        }
        Adt4::C2 => {
            Str::lit("u32")
        }
        Adt4::C3 => {
            Str::lit("u64")
        }
        Adt4::C4 => {
            Str::lit("i8")
        }
        Adt4::C5 => {
            Str::lit("i16")
        }
        Adt4::C6 => {
            Str::lit("i32")
        }
        Adt4::C7 => {
            Str::lit("i64")
        }
    }
}

pub fn f67(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a426 = f86(v0, 10000000000000000000u64);
        let a427 = {
            let a424 = f86(v0, 1000000000000000000u64);
            let a425 = {
                let a422 = f86(v0, 100000000000000000u64);
                let a423 = {
                    let a420 = f86(v0, 10000000000000000u64);
                    let a421 = {
                        let a418 = f86(v0, 1000000000000000u64);
                        let a419 = {
                            let a416 = f86(v0, 100000000000000u64);
                            let a417 = {
                                let a414 = f86(v0, 10000000000000u64);
                                let a415 = {
                                    let a412 = f86(v0, 1000000000000u64);
                                    let a413 = {
                                        let a410 = f86(v0, 100000000000u64);
                                        let a411 = {
                                            let a408 = f86(v0, 10000000000u64);
                                            let a409 = {
                                                let a406 = f86(v0, 1000000000u64);
                                                let a407 = {
                                                    let a404 = f86(v0, 100000000u64);
                                                    let a405 = {
                                                        let a402 = f86(v0, 10000000u64);
                                                        let a403 = {
                                                            let a400 = f86(v0, 1000000u64);
                                                            let a401 = {
                                                                let a398 = f86(v0, 100000u64);
                                                                let a399 = {
                                                                    let a396 = f86(v0, 10000u64);
                                                                    let a397 = {
                                                                        let a394 = f86(v0, 1000u64);
                                                                        let a395 = {
                                                                            let a392 = f86(v0, 100u64);
                                                                            let a393 = {
                                                                                let a390 = f86(v0, 10u64);
                                                                                let a391 = f86(v0, 1u64);
                                                                                append_bytes(a390, a391)
                                                                            };
                                                                            append_bytes(a392, a393)
                                                                        };
                                                                        append_bytes(a394, a395)
                                                                    };
                                                                    append_bytes(a396, a397)
                                                                };
                                                                append_bytes(a398, a399)
                                                            };
                                                            append_bytes(a400, a401)
                                                        };
                                                        append_bytes(a402, a403)
                                                    };
                                                    append_bytes(a404, a405)
                                                };
                                                append_bytes(a406, a407)
                                            };
                                            append_bytes(a408, a409)
                                        };
                                        append_bytes(a410, a411)
                                    };
                                    append_bytes(a412, a413)
                                };
                                append_bytes(a414, a415)
                            };
                            append_bytes(a416, a417)
                        };
                        append_bytes(a418, a419)
                    };
                    append_bytes(a420, a421)
                };
                append_bytes(a422, a423)
            };
            append_bytes(a424, a425)
        };
        append_bytes(a426, a427)
    };
    let v2: u64 = {
        let (h428, _) = f87(Fn6::F88(v1.clone()), {
            let a429 = v1.clone();
            length_bytes(a429)
        }, 0u64)?;
        h428
    };
    let a430 = v0;
    let a431 = 0u64;
    if nat_eq(a430, a431) {
        let a432 = Str::lit("0");
        Ok(utf8_encode(a432))
    } else {
        Ok(f89(v1.clone(), v2, {
            let a434 = {
                let a433 = v1;
                length_bytes(a433)
            };
            let a435 = v2;
            nat_sub(a434, a435)
        }))
    }
}

pub fn f68(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    let v2: Adt26 = {
        let a437 = {
            let (h436, _) = v0.clone();
            h436
        };
        let a438 = 0u64;
        if nat_eq(a437, a438) {
            let (_, h439) = v0.clone();
            h439
        } else {
            f8({
                let (_, h440) = v0.clone();
                h440
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a442 = {
            let (h441, _) = v0.clone();
            h441
        };
        let a443 = 1u64;
        nat_add(a442, a443)?
    }, {
        let m444 = v1;
        match m444 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h445, _) = v0;
                    h445
                })?
            }
            Adt11::C1 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h446, _) = v0;
                    h446
                })?
            }
            Adt11::C2 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h447, _) = v0;
                    h447
                })?
            }
            Adt11::C3 => {
                f8(v2, Str::lit("_"))?
            }
        }
    }))
}

pub fn f69(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    let v2: Adt26 = {
        let a449 = {
            let (h448, _) = v0.clone();
            h448
        };
        let a450 = 0u64;
        if nat_eq(a449, a450) {
            let (_, h451) = v0.clone();
            h451
        } else {
            f8({
                let (_, h452) = v0.clone();
                h452
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a454 = {
            let (h453, _) = v0.clone();
            h453
        };
        let a455 = 1u64;
        nat_add(a454, a455)?
    }, {
        let m456 = v1;
        match m456 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("*k"))?, {
                    let (h457, _) = v0;
                    h457
                })?
            }
            Adt11::C1 => {
                f8(f19(f8(v2, Str::lit("k"))?, {
                    let (h458, _) = v0;
                    h458
                })?, Str::lit(".clone()"))?
            }
            Adt11::C2 => {
                f8(f19(f8(v2, Str::lit("(**k"))?, {
                    let (h459, _) = v0;
                    h459
                })?, Str::lit(").clone()"))?
            }
            Adt11::C3 => {
                f8(v2, Str::lit("()"))?
            }
        }
    }))
}

pub fn f70(v0: u64, v1: u64, v2: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a461 = {
        let (h460, _) = v2.clone();
        h460
    };
    let a462 = v0;
    if nat_lt(a461, a462) {
        Ok(Some(({
            let a464 = {
                let (h463, _) = v2.clone();
                h463
            };
            let a465 = 1u64;
            nat_add(a464, a465)?
        }, f19(f8({
            let a467 = {
                let (h466, _) = v2.clone();
                h466
            };
            let a468 = 0u64;
            if if nat_eq(a467, a468) {
                let a469 = v1;
                let a470 = 0u64;
                nat_eq(a469, a470)
            } else {
                false
            } {
                let (_, h471) = v2;
                h471
            } else {
                f8({
                    let (_, h472) = v2;
                    h472
                }, Str::lit(", "))?
            }
        }, Str::lit("p"))?, {
            let (h473, _) = v2;
            h473
        })?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f71(v0: Adt4) -> u64 {
    let m474 = v0;
    match m474 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            0u64
        }
        Adt4::C2 => {
            0u64
        }
        Adt4::C3 => {
            0u64
        }
        Adt4::C4 => {
            128u64
        }
        Adt4::C5 => {
            32768u64
        }
        Adt4::C6 => {
            2147483648u64
        }
        Adt4::C7 => {
            9223372036854775808u64
        }
    }
}

pub fn f72(v0: Adt26, v1: Bytes) -> R<Adt26> {
    f8({
        let (_, h475) = {
            let (h476, _) = f40(Fn3::F90(v1.clone()), {
                let a477 = v1;
                length_bytes(a477)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h476
        };
        h475
    }, Str::lit("\""))
}

pub fn f73(v0: Adt26, v1: Bytes) -> R<Adt26> {
    f8({
        let (_, h478) = {
            let (h479, _) = f40(Fn3::F91(v1.clone()), {
                let a480 = v1;
                length_bytes(a480)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h479
        };
        h478
    }, Str::lit("])"))
}

pub fn f74(v0: Adt26, v1: Adt17) -> R<Adt26> {
    f8(v0, {
        let m481 = v1;
        match m481 {
            Adt17::C0 => {
                Str::lit("Ordering::Less")
            }
            Adt17::C1 => {
                Str::lit("Ordering::Equal")
            }
            Adt17::C2 => {
                Str::lit("Ordering::Greater")
            }
        }
    })
}

pub fn f75(v0: u64) -> Str {
    let a482 = v0;
    let a483 = 20u64;
    if nat_eq(a482, a483) {
        Str::lit("checked_add")
    } else {
        let a484 = v0;
        let a485 = 21u64;
        if nat_eq(a484, a485) {
            Str::lit("checked_sub")
        } else {
            let a486 = v0;
            let a487 = 22u64;
            if nat_eq(a486, a487) {
                Str::lit("checked_mul")
            } else {
                let a488 = v0;
                let a489 = 23u64;
                if nat_eq(a488, a489) {
                    Str::lit("checked_quot")
                } else {
                    let a490 = v0;
                    let a491 = 25u64;
                    if nat_eq(a490, a491) {
                        Str::lit("bit_and")
                    } else {
                        let a492 = v0;
                        let a493 = 26u64;
                        if nat_eq(a492, a493) {
                            Str::lit("bit_or")
                        } else {
                            let a494 = v0;
                            let a495 = 27u64;
                            if nat_eq(a494, a495) {
                                Str::lit("bit_xor")
                            } else {
                                let a496 = v0;
                                let a497 = 28u64;
                                if nat_eq(a496, a497) {
                                    Str::lit("bit_not")
                                } else {
                                    let a498 = v0;
                                    let a499 = 29u64;
                                    if nat_eq(a498, a499) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a500 = v0;
                                        let a501 = 30u64;
                                        if nat_eq(a500, a501) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a502 = v0;
                                            let a503 = 31u64;
                                            if nat_eq(a502, a503) {
                                                Str::lit("convert")
                                            } else {
                                                Str::lit("")
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

pub fn f76(v0: u64) -> Str {
    let a504 = v0;
    let a505 = 0u64;
    if nat_eq(a504, a505) {
        Str::lit("nat_add")
    } else {
        let a506 = v0;
        let a507 = 1u64;
        if nat_eq(a506, a507) {
            Str::lit("nat_sub")
        } else {
            let a508 = v0;
            let a509 = 2u64;
            if nat_eq(a508, a509) {
                Str::lit("nat_mul")
            } else {
                let a510 = v0;
                let a511 = 3u64;
                if nat_eq(a510, a511) {
                    Str::lit("nat_quot")
                } else {
                    let a512 = v0;
                    let a513 = 4u64;
                    if nat_eq(a512, a513) {
                        Str::lit("nat_rem")
                    } else {
                        let a514 = v0;
                        let a515 = 5u64;
                        if nat_eq(a514, a515) {
                            Str::lit("nat_eq")
                        } else {
                            let a516 = v0;
                            let a517 = 6u64;
                            if nat_eq(a516, a517) {
                                Str::lit("nat_le")
                            } else {
                                let a518 = v0;
                                let a519 = 7u64;
                                if nat_eq(a518, a519) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a520 = v0;
                                    let a521 = 8u64;
                                    if nat_eq(a520, a521) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a522 = v0;
                                        let a523 = 9u64;
                                        if nat_eq(a522, a523) {
                                            Str::lit("int_add")
                                        } else {
                                            let a524 = v0;
                                            let a525 = 10u64;
                                            if nat_eq(a524, a525) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a526 = v0;
                                                let a527 = 11u64;
                                                if nat_eq(a526, a527) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a528 = v0;
                                                    let a529 = 12u64;
                                                    if nat_eq(a528, a529) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a530 = v0;
                                                        let a531 = 13u64;
                                                        if nat_eq(a530, a531) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a532 = v0;
                                                            let a533 = 14u64;
                                                            if nat_eq(a532, a533) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a534 = v0;
                                                                let a535 = 15u64;
                                                                if nat_eq(a534, a535) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a536 = v0;
                                                                    let a537 = 16u64;
                                                                    if nat_eq(a536, a537) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a538 = v0;
                                                                        let a539 = 17u64;
                                                                        if nat_eq(a538, a539) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a540 = v0;
                                                                            let a541 = 18u64;
                                                                            if nat_eq(a540, a541) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a542 = v0;
                                                                                let a543 = 19u64;
                                                                                if nat_eq(a542, a543) {
                                                                                    Str::lit("compare")
                                                                                } else {
                                                                                    Str::lit("")
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
    }
}

pub fn f77(v0: u64) -> Str {
    let a544 = v0;
    let a545 = 32u64;
    if nat_eq(a544, a545) {
        Str::lit("append_list")
    } else {
        let a546 = v0;
        let a547 = 33u64;
        if nat_eq(a546, a547) {
            Str::lit("append_bytes")
        } else {
            let a548 = v0;
            let a549 = 34u64;
            if nat_eq(a548, a549) {
                Str::lit("length_list")
            } else {
                let a550 = v0;
                let a551 = 35u64;
                if nat_eq(a550, a551) {
                    Str::lit("length_bytes")
                } else {
                    let a552 = v0;
                    let a553 = 36u64;
                    if nat_eq(a552, a553) {
                        Str::lit("length_string")
                    } else {
                        let a554 = v0;
                        let a555 = 37u64;
                        if nat_eq(a554, a555) {
                            Str::lit("index_list")
                        } else {
                            let a556 = v0;
                            let a557 = 38u64;
                            if nat_eq(a556, a557) {
                                Str::lit("index_bytes")
                            } else {
                                let a558 = v0;
                                let a559 = 39u64;
                                if nat_eq(a558, a559) {
                                    Str::lit("slice_list")
                                } else {
                                    let a560 = v0;
                                    let a561 = 40u64;
                                    if nat_eq(a560, a561) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a562 = v0;
                                        let a563 = 41u64;
                                        if nat_eq(a562, a563) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a564 = v0;
                                            let a565 = 42u64;
                                            if nat_eq(a564, a565) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a566 = v0;
                                                let a567 = 43u64;
                                                if nat_eq(a566, a567) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a568 = v0;
                                                    let a569 = 44u64;
                                                    if nat_eq(a568, a569) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a570 = v0;
                                                        let a571 = 45u64;
                                                        if nat_eq(a570, a571) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a572 = v0;
                                                            let a573 = 46u64;
                                                            if nat_eq(a572, a573) {
                                                                Str::lit("format_int")
                                                            } else {
                                                                let a574 = v0;
                                                                let a575 = 48u64;
                                                                if nat_eq(a574, a575) {
                                                                    Str::lit("parse_int")
                                                                } else {
                                                                    let a576 = v0;
                                                                    let a577 = 50u64;
                                                                    if nat_eq(a576, a577) {
                                                                        Str::lit("update_bytes")
                                                                    } else {
                                                                        Str::lit("")
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

pub fn f78(v0: u64) -> Str {
    let a578 = v0;
    let a579 = 0u64;
    if nat_eq(a578, a579) {
        Str::lit("checked_add")
    } else {
        let a580 = v0;
        let a581 = 1u64;
        if nat_eq(a580, a581) {
            Str::lit("checked_sub")
        } else {
            let a582 = v0;
            let a583 = 2u64;
            if nat_eq(a582, a583) {
                Str::lit("checked_mul")
            } else {
                let a584 = v0;
                let a585 = 3u64;
                if nat_eq(a584, a585) {
                    Str::lit("checked_div")
                } else {
                    let a586 = v0;
                    let a587 = 4u64;
                    if nat_eq(a586, a587) {
                        Str::lit("checked_neg")
                    } else {
                        let a588 = v0;
                        let a589 = 5u64;
                        if nat_eq(a588, a589) {
                            Str::lit("checked_shl")
                        } else {
                            let a590 = v0;
                            let a591 = 6u64;
                            if nat_eq(a590, a591) {
                                Str::lit("checked_shr")
                            } else {
                                Str::lit("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f79(v0: u64) -> Str {
    let a592 = v0;
    let a593 = 44u64;
    if nat_eq(a592, a593) {
        Str::lit("Bytes::update(")
    } else {
        let a594 = v0;
        let a595 = 42u64;
        if nat_eq(a594, a595) {
            Str::lit("Str::parse_int(&")
        } else {
            let a596 = v0;
            let a597 = 43u64;
            if nat_eq(a596, a597) {
                Str::lit("Str::format_i64(")
            } else {
                let a598 = v0;
                let a599 = 30u64;
                if nat_eq(a598, a599) {
                    Str::lit("Bytes::append(")
                } else {
                    let a600 = v0;
                    let a601 = 31u64;
                    if nat_eq(a600, a601) {
                        Str::lit("Bytes::length(&")
                    } else {
                        let a602 = v0;
                        let a603 = 32u64;
                        if nat_eq(a602, a603) {
                            Str::lit("Str::length(&")
                        } else {
                            let a604 = v0;
                            let a605 = 33u64;
                            if nat_eq(a604, a605) {
                                Str::lit("Bytes::index(&")
                            } else {
                                let a606 = v0;
                                let a607 = 34u64;
                                if nat_eq(a606, a607) {
                                    Str::lit("Bytes::slice(&")
                                } else {
                                    let a608 = v0;
                                    let a609 = 35u64;
                                    if nat_eq(a608, a609) {
                                        Str::lit("Str::utf8_encode(&")
                                    } else {
                                        let a610 = v0;
                                        let a611 = 36u64;
                                        if nat_eq(a610, a611) {
                                            Str::lit("Bytes::utf8_decode(&")
                                        } else {
                                            let a612 = v0;
                                            let a613 = 37u64;
                                            if nat_eq(a612, a613) {
                                                Str::lit("Bytes::compare(&")
                                            } else {
                                                let a614 = v0;
                                                let a615 = 38u64;
                                                if nat_eq(a614, a615) {
                                                    Str::lit("Str::split_exact(&")
                                                } else {
                                                    let a616 = v0;
                                                    let a617 = 39u64;
                                                    if nat_eq(a616, a617) {
                                                        Str::lit("Str::join(")
                                                    } else {
                                                        let a618 = v0;
                                                        let a619 = 8u64;
                                                        if nat_eq(a618, a619) {
                                                            Str::lit("core::ops::BitAnd::bitand(")
                                                        } else {
                                                            let a620 = v0;
                                                            let a621 = 9u64;
                                                            if nat_eq(a620, a621) {
                                                                Str::lit("core::ops::BitOr::bitor(")
                                                            } else {
                                                                let a622 = v0;
                                                                let a623 = 10u64;
                                                                if nat_eq(a622, a623) {
                                                                    Str::lit("core::ops::BitXor::bitxor(")
                                                                } else {
                                                                    let a624 = v0;
                                                                    let a625 = 11u64;
                                                                    if nat_eq(a624, a625) {
                                                                        Str::lit("core::ops::Not::not(")
                                                                    } else {
                                                                        let a626 = v0;
                                                                        let a627 = 12u64;
                                                                        if nat_eq(a626, a627) {
                                                                            Str::lit("core::cmp::PartialEq::eq(&")
                                                                        } else {
                                                                            let a628 = v0;
                                                                            let a629 = 13u64;
                                                                            if nat_eq(a628, a629) {
                                                                                Str::lit("core::cmp::PartialOrd::le(&")
                                                                            } else {
                                                                                let a630 = v0;
                                                                                let a631 = 14u64;
                                                                                if nat_eq(a630, a631) {
                                                                                    Str::lit("core::cmp::PartialOrd::lt(&")
                                                                                } else {
                                                                                    let a632 = v0;
                                                                                    let a633 = 15u64;
                                                                                    if nat_eq(a632, a633) {
                                                                                        Str::lit("u64::checked_rem(")
                                                                                    } else {
                                                                                        let a634 = v0;
                                                                                        let a635 = 16u64;
                                                                                        if nat_eq(a634, a635) {
                                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                        } else {
                                                                                            let a636 = v0;
                                                                                            let a637 = 17u64;
                                                                                            if nat_eq(a636, a637) {
                                                                                                Str::lit("u64::checked_add(")
                                                                                            } else {
                                                                                                let a638 = v0;
                                                                                                let a639 = 18u64;
                                                                                                if nat_eq(a638, a639) {
                                                                                                    Str::lit("u64::saturating_sub(")
                                                                                                } else {
                                                                                                    let a640 = v0;
                                                                                                    let a641 = 19u64;
                                                                                                    if nat_eq(a640, a641) {
                                                                                                        Str::lit("u64::checked_mul(")
                                                                                                    } else {
                                                                                                        let a642 = v0;
                                                                                                        let a643 = 20u64;
                                                                                                        if nat_eq(a642, a643) {
                                                                                                            Str::lit("u64::checked_div(")
                                                                                                        } else {
                                                                                                            let a644 = v0;
                                                                                                            let a645 = 21u64;
                                                                                                            if nat_eq(a644, a645) {
                                                                                                                Str::lit("i64::checked_add(")
                                                                                                            } else {
                                                                                                                let a646 = v0;
                                                                                                                let a647 = 22u64;
                                                                                                                if nat_eq(a646, a647) {
                                                                                                                    Str::lit("i64::checked_sub(")
                                                                                                                } else {
                                                                                                                    let a648 = v0;
                                                                                                                    let a649 = 23u64;
                                                                                                                    if nat_eq(a648, a649) {
                                                                                                                        Str::lit("i64::checked_mul(")
                                                                                                                    } else {
                                                                                                                        let a650 = v0;
                                                                                                                        let a651 = 24u64;
                                                                                                                        if nat_eq(a650, a651) {
                                                                                                                            Str::lit("i64::checked_neg(")
                                                                                                                        } else {
                                                                                                                            let a652 = v0;
                                                                                                                            let a653 = 25u64;
                                                                                                                            if nat_eq(a652, a653) {
                                                                                                                                Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                                                            } else {
                                                                                                                                let a654 = v0;
                                                                                                                                let a655 = 26u64;
                                                                                                                                if nat_eq(a654, a655) {
                                                                                                                                    Str::lit("List::length(&")
                                                                                                                                } else {
                                                                                                                                    let a656 = v0;
                                                                                                                                    let a657 = 27u64;
                                                                                                                                    if nat_eq(a656, a657) {
                                                                                                                                        Str::lit("List::append(")
                                                                                                                                    } else {
                                                                                                                                        let a658 = v0;
                                                                                                                                        let a659 = 28u64;
                                                                                                                                        if nat_eq(a658, a659) {
                                                                                                                                            Str::lit("List::index(&")
                                                                                                                                        } else {
                                                                                                                                            let a660 = v0;
                                                                                                                                            let a661 = 29u64;
                                                                                                                                            if nat_eq(a660, a661) {
                                                                                                                                                Str::lit("List::slice(&")
                                                                                                                                            } else {
                                                                                                                                                Str::lit("")
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

pub fn f80(v0: u64) -> Str {
    let a662 = v0;
    let a663 = 16u64;
    if nat_eq(a662, a663) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a664 = v0;
        let a665 = 25u64;
        if nat_eq(a664, a665) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a666 = v0;
            let a667 = 15u64;
            if nat_eq(a666, a667) {
                Str::lit(")")
            } else {
                let a668 = v0;
                let a669 = 20u64;
                if nat_eq(a668, a669) {
                    Str::lit(")")
                } else {
                    let a670 = v0;
                    let a671 = 8u64;
                    if nat_eq(a670, a671) {
                        Str::lit(")")
                    } else {
                        let a672 = v0;
                        let a673 = 9u64;
                        if nat_eq(a672, a673) {
                            Str::lit(")")
                        } else {
                            let a674 = v0;
                            let a675 = 10u64;
                            if nat_eq(a674, a675) {
                                Str::lit(")")
                            } else {
                                let a676 = v0;
                                let a677 = 11u64;
                                if nat_eq(a676, a677) {
                                    Str::lit(")")
                                } else {
                                    let a678 = v0;
                                    let a679 = 12u64;
                                    if nat_eq(a678, a679) {
                                        Str::lit(")")
                                    } else {
                                        let a680 = v0;
                                        let a681 = 13u64;
                                        if nat_eq(a680, a681) {
                                            Str::lit(")")
                                        } else {
                                            let a682 = v0;
                                            let a683 = 14u64;
                                            if nat_eq(a682, a683) {
                                                Str::lit(")")
                                            } else {
                                                let a684 = v0;
                                                let a685 = 17u64;
                                                if nat_eq(a684, a685) {
                                                    Str::lit(")")
                                                } else {
                                                    let a686 = v0;
                                                    let a687 = 18u64;
                                                    if nat_eq(a686, a687) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a688 = v0;
                                                        let a689 = 19u64;
                                                        if nat_eq(a688, a689) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a690 = v0;
                                                            let a691 = 21u64;
                                                            if nat_eq(a690, a691) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a692 = v0;
                                                                let a693 = 22u64;
                                                                if nat_eq(a692, a693) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a694 = v0;
                                                                    let a695 = 23u64;
                                                                    if nat_eq(a694, a695) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a696 = v0;
                                                                        let a697 = 24u64;
                                                                        let _ = nat_eq(a696, a697);
                                                                        Str::lit(")")
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

pub fn f81(v0: u64, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f92(v0, v1)
}

pub fn f82(v0: u64) -> Str {
    let a698 = v0;
    let a699 = 37u64;
    if nat_eq(a698, a699) {
        Str::lit(", &")
    } else {
        let a700 = v0;
        let a701 = 38u64;
        if nat_eq(a700, a701) {
            Str::lit(", &")
        } else {
            let a702 = v0;
            let a703 = 39u64;
            if nat_eq(a702, a703) {
                Str::lit(", &")
            } else {
                let a704 = v0;
                let a705 = 12u64;
                if nat_eq(a704, a705) {
                    Str::lit(", &")
                } else {
                    let a706 = v0;
                    let a707 = 13u64;
                    if nat_eq(a706, a707) {
                        Str::lit(", &")
                    } else {
                        let a708 = v0;
                        let a709 = 14u64;
                        if nat_eq(a708, a709) {
                            Str::lit(", &")
                        } else {
                            let a710 = v0;
                            let a711 = 8u64;
                            if nat_eq(a710, a711) {
                                Str::lit(", ")
                            } else {
                                let a712 = v0;
                                let a713 = 9u64;
                                if nat_eq(a712, a713) {
                                    Str::lit(", ")
                                } else {
                                    let a714 = v0;
                                    let a715 = 10u64;
                                    if nat_eq(a714, a715) {
                                        Str::lit(", ")
                                    } else {
                                        let a716 = v0;
                                        let a717 = 11u64;
                                        if nat_eq(a716, a717) {
                                            Str::lit(", ")
                                        } else {
                                            let a718 = v0;
                                            let a719 = 15u64;
                                            if nat_eq(a718, a719) {
                                                Str::lit(", ")
                                            } else {
                                                let a720 = v0;
                                                let a721 = 16u64;
                                                if nat_eq(a720, a721) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a722 = v0;
                                                    let a723 = 17u64;
                                                    if nat_eq(a722, a723) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a724 = v0;
                                                        let a725 = 18u64;
                                                        if nat_eq(a724, a725) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a726 = v0;
                                                            let a727 = 19u64;
                                                            if nat_eq(a726, a727) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a728 = v0;
                                                                let a729 = 20u64;
                                                                if nat_eq(a728, a729) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a730 = v0;
                                                                    let a731 = 21u64;
                                                                    if nat_eq(a730, a731) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a732 = v0;
                                                                        let a733 = 22u64;
                                                                        if nat_eq(a732, a733) {
                                                                            Str::lit(", ")
                                                                        } else {
                                                                            let a734 = v0;
                                                                            let a735 = 23u64;
                                                                            if nat_eq(a734, a735) {
                                                                                Str::lit(", ")
                                                                            } else {
                                                                                let a736 = v0;
                                                                                let a737 = 24u64;
                                                                                if nat_eq(a736, a737) {
                                                                                    Str::lit(", ")
                                                                                } else {
                                                                                    let a738 = v0;
                                                                                    let a739 = 25u64;
                                                                                    let _ = nat_eq(a738, a739);
                                                                                    Str::lit(", ")
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
    }
}

pub fn f83(v0: u64) -> Str {
    let a740 = v0;
    let a741 = 15u64;
    if nat_eq(a740, a741) {
        Str::lit(").unwrap_or(")
    } else {
        let a742 = v0;
        let a743 = 16u64;
        if nat_eq(a742, a743) {
            Str::lit(", ")
        } else {
            let a744 = v0;
            let a745 = 20u64;
            if nat_eq(a744, a745) {
                Str::lit(").unwrap_or(")
            } else {
                let a746 = v0;
                let a747 = 25u64;
                let _ = nat_eq(a746, a747);
                Str::lit(", ")
            }
        }
    }
}

pub fn f84(v0: Adt13, v1: Adt2, v2: u64, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    let a749 = {
        let (h748, _) = v3.clone();
        h748
    };
    let a750 = v2;
    if nat_lt(a749, a750) {
        Ok(Some(({
            let a752 = {
                let (h751, _) = v3.clone();
                h751
            };
            let a753 = 1u64;
            nat_add(a752, a753)?
        }, if if f93(v0, {
            let (h754, _) = v3.clone();
            h754
        }) {
            true
        } else {
            f94(v1, {
                let (h755, _) = v3.clone();
                h755
            })
        } {
            (f8(f35(f8({
                let (h756, _) = {
                    let (_, h757) = v3.clone();
                    h757
                };
                h756
            }, {
                let (_, h758) = {
                    let (_, h759) = v3;
                    h759
                };
                if h758 {
                    Str::lit(", ")
                } else {
                    Str::lit("<")
                }
            })?, {
                let (h760, _) = v3;
                h760
            })?, Str::lit(": Clone"))?, true)
        } else {
            let (_, h761) = v3;
            h761
        })))
    } else {
        Ok(None::<(u64, (Adt26, bool))>)
    }
}

pub fn f85(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m763 = {
        let (h762, _) = v0;
        h762
    };
    match m763 {
        None => {
            (Some(v1), {
                let (_, h764) = v0;
                h764
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a765 = v2;
                let a766 = v1;
                append_bytes(a765, a766)
            }, {
                let (_, h767) = v0;
                h767
            }))
        }
    }
}

pub fn f86(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a771 = {
            let a768 = v0;
            let a769 = v1;
            let a770 = 0u64;
            nat_quot(a768, a769, a770)
        };
        let a772 = 10u64;
        let a773 = 0u64;
        nat_rem(a771, a772, a773)
    }, 1u64)
}

pub fn f87(v0: Fn6, p778: u64, p779: u64) -> R<(u64, bool)> {
    let m777 = (p778, p779);
    {
        let mut m777 = m777;
        loop {
            let step = {
                let (v1, v2) = m777;
                let m774 = v1;
                if m774 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m774 - 1;
                    let m776 = {
                        let c775: Fn6 = v0.clone();
                        c775.apply(v2)?
                    };
                    match m776 {
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
                Ok(next) => { m777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f95(v0, v1)
}

pub fn f89(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m783 = {
        let a780 = v0;
        let a781 = v1;
        let a782 = v2;
        slice_bytes(a780, a781, a782)
    };
    match m783 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f90(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f96(v0, v1)
}

pub fn f91(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f97(v0, v1)
}

pub fn f92(v0: u64, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a785 = {
        let (h784, _) = v1.clone();
        h784
    };
    let a786 = v0;
    if nat_lt(a785, a786) {
        Ok(Some(({
            let a788 = {
                let (h787, _) = v1;
                h787
            };
            let a789 = 1u64;
            nat_add(a788, a789)?
        }, f8({
            let (_, h790) = v1;
            h790
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f93(p795: Adt13, v1: u64) -> bool {
    let m794 = p795;
    {
        let mut m794 = m794;
        loop {
            let step = {
                let v0 = m794;
                let m791 = v0;
                match m791 {
                    Adt13::C0 => {
                        Err::<Adt13, bool>(false)
                    }
                    Adt13::C1(_, r792, r793) => {
                        let v3 = (*r792).clone();
                        let v4 = (*r793).clone();
                        if f94(v3, v1) {
                            Err::<Adt13, bool>(true)
                        } else {
                            Ok::<Adt13, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt2, v1: u64) -> bool {
    let m796 = v0;
    match m796 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, r800) => {
            let v7 = (*r800).clone();
            f94(v7, v1)
        }
        Adt2::C3(_, r801, r802) => {
            let v9 = (*r801).clone();
            let v10 = (*r802).clone();
            if f94(v9, v1) {
                true
            } else {
                f94(v10, v1)
            }
        }
        Adt2::C4(v2, v3) => {
            let m797 = v2;
            match m797 {
                Adt7::C0 => {
                    false
                }
                Adt7::C1 => {
                    false
                }
                Adt7::C2 => {
                    let a798 = v3;
                    let a799 = v1;
                    nat_eq(a798, a799)
                }
            }
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(r803, r804) => {
            let v11 = (*r803).clone();
            let v12 = (*r804).clone();
            if f94(v11, v1) {
                true
            } else {
                f94(v12, v1)
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m805 = f98(v0, v1);
    match m805 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a806 = v2;
            let a807 = 48u8;
            if equal(a806, a807) {
                Ok(Some({
                    let a808 = v1;
                    let a809 = 1u64;
                    nat_add(a808, a809)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a812 = {
        let (h810, _) = v1.clone();
        h810
    };
    let a813 = {
        let a811 = v0.clone();
        length_bytes(a811)
    };
    if nat_lt(a812, a813) {
        let v2: (u64, u64) = f99(v0.clone(), {
            let (h814, _) = v1.clone();
            h814
        })?;
        let v3: u64 = {
            let (h815, _) = v2;
            h815
        };
        Ok(Some(({
            let a818 = {
                let (h816, _) = v1.clone();
                h816
            };
            let a819 = {
                let (_, h817) = v2;
                h817
            };
            nat_add(a818, a819)?
        }, {
            let a820 = v3;
            let a821 = 34u64;
            if nat_eq(a820, a821) {
                f8({
                    let (_, h822) = v1;
                    h822
                }, Str::lit("\\\""))?
            } else {
                let a823 = v3;
                let a824 = 92u64;
                if nat_eq(a823, a824) {
                    f8({
                        let (_, h825) = v1;
                        h825
                    }, Str::lit("\\\\"))?
                } else {
                    let a826 = 32u64;
                    let a827 = v3;
                    if if nat_le(a826, a827) {
                        let a828 = v3;
                        let a829 = 126u64;
                        nat_le(a828, a829)
                    } else {
                        false
                    } {
                        f3({
                            let (_, h830) = v1;
                            h830
                        }, f89(v0, {
                            let (h831, _) = v1;
                            h831
                        }, 1u64))?
                    } else {
                        f8(f100(f8({
                            let (_, h832) = v1;
                            h832
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f97(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let m834 = f98(v0, {
        let (h833, _) = v1.clone();
        h833
    });
    match m834 {
        None => {
            Ok(None::<(u64, Adt26)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a836 = {
                    let (h835, _) = v1.clone();
                    h835
                };
                let a837 = 1u64;
                nat_add(a836, a837)?
            }, f101(f8({
                let a839 = {
                    let (h838, _) = v1;
                    h838
                };
                let a840 = 0u64;
                if nat_eq(a839, a840) {
                    let (_, h841) = v1;
                    h841
                } else {
                    f8({
                        let (_, h842) = v1;
                        h842
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> Option<u8> {
    let a843 = v0;
    let a844 = v1;
    index_bytes(a843, a844)
}

pub fn f99(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m845 = f98(v0.clone(), v1);
    match m845 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a890 = {
                    let a866 = {
                        let a854 = {
                            let a848 = {
                                let a846 = v3;
                                let a847 = 128u8;
                                fixed_u8::bit_and(a846, a847)
                            };
                            let a849 = 128u8;
                            if equal(a848, a849) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a855 = {
                            let a852 = {
                                let a850 = v3;
                                let a851 = 64u8;
                                fixed_u8::bit_and(a850, a851)
                            };
                            let a853 = 64u8;
                            if equal(a852, a853) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a854, a855)?
                    };
                    let a867 = {
                        let a864 = {
                            let a858 = {
                                let a856 = v3;
                                let a857 = 32u8;
                                fixed_u8::bit_and(a856, a857)
                            };
                            let a859 = 32u8;
                            if equal(a858, a859) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a865 = {
                            let a862 = {
                                let a860 = v3;
                                let a861 = 16u8;
                                fixed_u8::bit_and(a860, a861)
                            };
                            let a863 = 16u8;
                            if equal(a862, a863) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a864, a865)?
                    };
                    nat_add(a866, a867)?
                };
                let a891 = {
                    let a888 = {
                        let a876 = {
                            let a870 = {
                                let a868 = v3;
                                let a869 = 8u8;
                                fixed_u8::bit_and(a868, a869)
                            };
                            let a871 = 8u8;
                            if equal(a870, a871) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a877 = {
                            let a874 = {
                                let a872 = v3;
                                let a873 = 4u8;
                                fixed_u8::bit_and(a872, a873)
                            };
                            let a875 = 4u8;
                            if equal(a874, a875) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a876, a877)?
                    };
                    let a889 = {
                        let a886 = {
                            let a880 = {
                                let a878 = v3;
                                let a879 = 2u8;
                                fixed_u8::bit_and(a878, a879)
                            };
                            let a881 = 2u8;
                            if equal(a880, a881) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a887 = {
                            let a884 = {
                                let a882 = v3;
                                let a883 = 1u8;
                                fixed_u8::bit_and(a882, a883)
                            };
                            let a885 = 1u8;
                            if equal(a884, a885) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a886, a887)?
                    };
                    nat_add(a888, a889)?
                };
                nat_add(a890, a891)?
            };
            let a892 = v4;
            let a893 = 128u64;
            if nat_lt(a892, a893) {
                Ok((v4, 1u64))
            } else {
                let a894 = v4;
                let a895 = 224u64;
                if nat_lt(a894, a895) {
                    Ok(({
                        let a903 = {
                            let a899 = {
                                let a896 = v4;
                                let a897 = 32u64;
                                let a898 = 0u64;
                                nat_rem(a896, a897, a898)
                            };
                            let a900 = 64u64;
                            nat_mul(a899, a900)?
                        };
                        let a904 = f102(v0, {
                            let a901 = v1;
                            let a902 = 1u64;
                            nat_add(a901, a902)?
                        })?;
                        nat_add(a903, a904)?
                    }, 2u64))
                } else {
                    let a905 = v4;
                    let a906 = 240u64;
                    if nat_lt(a905, a906) {
                        Ok(({
                            let a920 = {
                                let a910 = {
                                    let a907 = v4;
                                    let a908 = 16u64;
                                    let a909 = 0u64;
                                    nat_rem(a907, a908, a909)
                                };
                                let a911 = 4096u64;
                                nat_mul(a910, a911)?
                            };
                            let a921 = {
                                let a918 = {
                                    let a914 = f102(v0.clone(), {
                                        let a912 = v1;
                                        let a913 = 1u64;
                                        nat_add(a912, a913)?
                                    })?;
                                    let a915 = 64u64;
                                    nat_mul(a914, a915)?
                                };
                                let a919 = f102(v0, {
                                    let a916 = v1;
                                    let a917 = 2u64;
                                    nat_add(a916, a917)?
                                })?;
                                nat_add(a918, a919)?
                            };
                            nat_add(a920, a921)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a941 = {
                                let a925 = {
                                    let a922 = v4;
                                    let a923 = 8u64;
                                    let a924 = 0u64;
                                    nat_rem(a922, a923, a924)
                                };
                                let a926 = 262144u64;
                                nat_mul(a925, a926)?
                            };
                            let a942 = {
                                let a939 = {
                                    let a929 = f102(v0.clone(), {
                                        let a927 = v1;
                                        let a928 = 1u64;
                                        nat_add(a927, a928)?
                                    })?;
                                    let a930 = 4096u64;
                                    nat_mul(a929, a930)?
                                };
                                let a940 = {
                                    let a937 = {
                                        let a933 = f102(v0.clone(), {
                                            let a931 = v1;
                                            let a932 = 2u64;
                                            nat_add(a931, a932)?
                                        })?;
                                        let a934 = 64u64;
                                        nat_mul(a933, a934)?
                                    };
                                    let a938 = f102(v0, {
                                        let a935 = v1;
                                        let a936 = 3u64;
                                        nat_add(a935, a936)?
                                    })?;
                                    nat_add(a937, a938)?
                                };
                                nat_add(a939, a940)?
                            };
                            nat_add(a941, a942)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f100(v0: Adt26, v1: u64) -> R<Adt26> {
    let v2: Bytes = {
        let a955 = f103(v1, 268435456u64);
        let a956 = {
            let a953 = f103(v1, 16777216u64);
            let a954 = {
                let a951 = f103(v1, 1048576u64);
                let a952 = {
                    let a949 = f103(v1, 65536u64);
                    let a950 = {
                        let a947 = f103(v1, 4096u64);
                        let a948 = {
                            let a945 = f103(v1, 256u64);
                            let a946 = {
                                let a943 = f103(v1, 16u64);
                                let a944 = f103(v1, 1u64);
                                append_bytes(a943, a944)
                            };
                            append_bytes(a945, a946)
                        };
                        append_bytes(a947, a948)
                    };
                    append_bytes(a949, a950)
                };
                append_bytes(a951, a952)
            };
            append_bytes(a953, a954)
        };
        append_bytes(a955, a956)
    };
    let v3: u64 = {
        let (h957, _) = f87(Fn6::F104(v2.clone()), 7u64, 0u64)?;
        h957
    };
    f3(v0, f89(v2, v3, {
        let a958 = 8u64;
        let a959 = v3;
        nat_sub(a958, a959)
    }))
}

pub fn f101(v0: Adt26, v1: u8) -> R<Adt26> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a1004 = {
            let a980 = {
                let a968 = {
                    let a962 = {
                        let a960 = v2;
                        let a961 = 128u8;
                        fixed_u8::bit_and(a960, a961)
                    };
                    let a963 = 128u8;
                    if equal(a962, a963) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a969 = {
                    let a966 = {
                        let a964 = v2;
                        let a965 = 64u8;
                        fixed_u8::bit_and(a964, a965)
                    };
                    let a967 = 64u8;
                    if equal(a966, a967) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a968, a969)?
            };
            let a981 = {
                let a978 = {
                    let a972 = {
                        let a970 = v2;
                        let a971 = 32u8;
                        fixed_u8::bit_and(a970, a971)
                    };
                    let a973 = 32u8;
                    if equal(a972, a973) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a979 = {
                    let a976 = {
                        let a974 = v2;
                        let a975 = 16u8;
                        fixed_u8::bit_and(a974, a975)
                    };
                    let a977 = 16u8;
                    if equal(a976, a977) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a978, a979)?
            };
            nat_add(a980, a981)?
        };
        let a1005 = {
            let a1002 = {
                let a990 = {
                    let a984 = {
                        let a982 = v2;
                        let a983 = 8u8;
                        fixed_u8::bit_and(a982, a983)
                    };
                    let a985 = 8u8;
                    if equal(a984, a985) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a991 = {
                    let a988 = {
                        let a986 = v2;
                        let a987 = 4u8;
                        fixed_u8::bit_and(a986, a987)
                    };
                    let a989 = 4u8;
                    if equal(a988, a989) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a990, a991)?
            };
            let a1003 = {
                let a1000 = {
                    let a994 = {
                        let a992 = v2;
                        let a993 = 2u8;
                        fixed_u8::bit_and(a992, a993)
                    };
                    let a995 = 2u8;
                    if equal(a994, a995) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1001 = {
                    let a998 = {
                        let a996 = v2;
                        let a997 = 1u8;
                        fixed_u8::bit_and(a996, a997)
                    };
                    let a999 = 1u8;
                    if equal(a998, a999) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1000, a1001)?
            };
            nat_add(a1002, a1003)?
        };
        nat_add(a1004, a1005)?
    };
    f3(v0, {
        let a1006 = f103(v3, 16u64);
        let a1007 = f103(v3, 1u64);
        append_bytes(a1006, a1007)
    })
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let m1008 = f98(v0, v1);
    match m1008 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a1055 = {
                let v3: u8 = v2;
                let a1053 = {
                    let a1029 = {
                        let a1017 = {
                            let a1011 = {
                                let a1009 = v3;
                                let a1010 = 128u8;
                                fixed_u8::bit_and(a1009, a1010)
                            };
                            let a1012 = 128u8;
                            if equal(a1011, a1012) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1018 = {
                            let a1015 = {
                                let a1013 = v3;
                                let a1014 = 64u8;
                                fixed_u8::bit_and(a1013, a1014)
                            };
                            let a1016 = 64u8;
                            if equal(a1015, a1016) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1017, a1018)?
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1021 = {
                                let a1019 = v3;
                                let a1020 = 32u8;
                                fixed_u8::bit_and(a1019, a1020)
                            };
                            let a1022 = 32u8;
                            if equal(a1021, a1022) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1028 = {
                            let a1025 = {
                                let a1023 = v3;
                                let a1024 = 16u8;
                                fixed_u8::bit_and(a1023, a1024)
                            };
                            let a1026 = 16u8;
                            if equal(a1025, a1026) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1027, a1028)?
                    };
                    nat_add(a1029, a1030)?
                };
                let a1054 = {
                    let a1051 = {
                        let a1039 = {
                            let a1033 = {
                                let a1031 = v3;
                                let a1032 = 8u8;
                                fixed_u8::bit_and(a1031, a1032)
                            };
                            let a1034 = 8u8;
                            if equal(a1033, a1034) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1040 = {
                            let a1037 = {
                                let a1035 = v3;
                                let a1036 = 4u8;
                                fixed_u8::bit_and(a1035, a1036)
                            };
                            let a1038 = 4u8;
                            if equal(a1037, a1038) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1039, a1040)?
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1043 = {
                                let a1041 = v3;
                                let a1042 = 2u8;
                                fixed_u8::bit_and(a1041, a1042)
                            };
                            let a1044 = 2u8;
                            if equal(a1043, a1044) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1050 = {
                            let a1047 = {
                                let a1045 = v3;
                                let a1046 = 1u8;
                                fixed_u8::bit_and(a1045, a1046)
                            };
                            let a1048 = 1u8;
                            if equal(a1047, a1048) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1049, a1050)?
                    };
                    nat_add(a1051, a1052)?
                };
                nat_add(a1053, a1054)?
            };
            let a1056 = 64u64;
            let a1057 = 0u64;
            Ok(nat_rem(a1055, a1056, a1057))
        }
    }
}

pub fn f103(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1061 = {
            let a1058 = v0;
            let a1059 = v1;
            let a1060 = 0u64;
            nat_quot(a1058, a1059, a1060)
        };
        let a1062 = 16u64;
        let a1063 = 0u64;
        nat_rem(a1061, a1062, a1063)
    }, 1u64)
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f105(v0, v1)
}

pub fn f105(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1064 = f98(v0, v1);
    match m1064 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a1065 = v2;
            let a1066 = 48u8;
            if equal(a1065, a1066) {
                Ok(Some({
                    let a1067 = v1;
                    let a1068 = 1u64;
                    nat_add(a1067, a1068)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}
