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
    C0(u64, bool, u64, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, bool, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, bool, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, std::rc::Rc<Adt9>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Adt0> {
        match self {
            Fn0::F4(k0) => f4(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Adt1> {
        match self {
            Fn1::F6(k0) => f6(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F9(u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn2::F9(k0, k1) => f9(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F14(u64, u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt3, p1: Adt2) -> R<Adt3> {
        match self {
            Fn3::F14(k0, k1, k2) => f14(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25,
    F46(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn4::F25 => Ok(f25(p0)),
            Fn4::F46(k0) => f46(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F27,
}

impl Fn5 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn5::F27 => f27(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F39(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn6::F39(k0) => f39(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F41,
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> Option<u64> {
        match self {
            Fn7::F41 => f41(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F43(Bytes, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Bytes)) -> Adt6 {
        match self {
            Fn8::F43(k0, k1) => f43(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F49(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
        match self {
            Fn9::F49(k0, k1) => f49(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F51(std::rc::Rc<Adt2>),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt7, p1: Adt2) -> Adt7 {
        match self {
            Fn10::F51(k0) => f51((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F56,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn11::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F61,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn12::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F66(Bytes, u64, u64),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Adt8)) -> R<(u64, Adt8)> {
        match self {
            Fn13::F66(k0, k1, k2) => f66(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F74,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt10) -> R<Adt10> {
        match self {
            Fn14::F74 => f74(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F90,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F90 => f90(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let a2 = 24u64;
    let a3 = v1;
    if if nat_le(a2, a3) {
        let a4 = f1(v0.clone(), 0u64, 8u64);
        let a5 = Bytes::lit(&[0x47, 0x47, 0x55, 0x46, 0x03, 0x00, 0x00, 0x00]);
        equal(a4, a5)
    } else {
        false
    } {
        let v2: u64 = f2(v0.clone(), 8u64, 8u64)?;
        let v3: u64 = f2(v0.clone(), 16u64, 8u64)?;
        let a6 = v3;
        let a7 = v1;
        if if nat_le(a6, a7) {
            let a8 = v2;
            let a9 = v1;
            nat_le(a8, a9)
        } else {
            false
        } {
            let v4: Adt0 = f3(Fn0::F4(v0.clone()), v3, Adt0::C0(24u64, true, 32u64, List::<(Bytes, Bytes)>::nil()))?;
            if match v4.clone() {
                Adt0::C0(_, h10, _, _) => {
                    h10
                }
            } {
                let v5: Adt1 = f5(Fn1::F6(v0.clone()), v2, Adt1::C0(match v4.clone() {
                    Adt0::C0(h11, _, _, _) => {
                        h11
                    }
                }, true, List::<Adt2>::nil()))?;
                if match v5.clone() {
                    Adt1::C0(_, h12, _) => {
                        h12
                    }
                } {
                    f7(v0, v4, v5)
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m16 = {
        let a13 = v0;
        let a14 = v1;
        let a15 = v2;
        slice_bytes(a13, a14, a15)
    };
    match m16 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h17) = f8(Fn2::F9(v1, v0), v2, (v2, 0u64))?;
    Ok(h17)
}

pub fn f3(v0: Fn0, p21: u64, p22: Adt0) -> R<Adt0> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m18 = v1;
                if m18 == 0 {
                    Err::<(u64, Adt0), R<Adt0>>(Ok(v2))
                } else {
                    let v3 = m18 - 1;
                    Ok::<(u64, Adt0), R<Adt0>>((v3, {
                        let c19: Fn0 = v0.clone();
                        c19.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Adt0) -> R<Adt0> {
    f10(v0, v1)
}

pub fn f5(v0: Fn1, p26: u64, p27: Adt1) -> R<Adt1> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v1, v2) = m25;
                let m23 = v1;
                if m23 == 0 {
                    Err::<(u64, Adt1), R<Adt1>>(Ok(v2))
                } else {
                    let v3 = m23 - 1;
                    Ok::<(u64, Adt1), R<Adt1>>((v3, {
                        let c24: Fn1 = v0.clone();
                        c24.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m25 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: Adt1) -> R<Adt1> {
    f11(v0, v1)
}

pub fn f7(v0: Bytes, v1: Adt0, v2: Adt1) -> R<Option<Bytes>> {
    let v3: u64 = match v1.clone() {
        Adt0::C0(_, _, h28, _) => {
            h28
        }
    };
    let v4: u64 = f12(match v2.clone() {
        Adt1::C0(h29, _, _) => {
            h29
        }
    }, v3)?;
    let v5: Adt3 = f13(Fn3::F14(v3, v4, v0.clone()), Adt3::C0(true, 0u64, Bytes::lit(&[])), match v2 {
        Adt1::C0(_, _, h30) => {
            h30
        }
    })?;
    if match v5.clone() {
        Adt3::C0(h31, _, _) => {
            h31
        }
    } {
        Ok(Some({
            let a38 = f1(v0, 0u64, 24u64);
            let a39 = {
                let a36 = f15(v3, 8u64)?;
                let a37 = {
                    let a34 = f16(match v1 {
                        Adt0::C0(_, _, _, h32) => {
                            h32
                        }
                    });
                    let a35 = match v5 {
                        Adt3::C0(_, _, h33) => {
                            h33
                        }
                    };
                    append_bytes(a34, a35)
                };
                append_bytes(a36, a37)
            };
            append_bytes(a38, a39)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f8(v0: Fn2, p43: u64, p44: (u64, u64)) -> R<(u64, u64)> {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m40 = v1;
                if m40 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m40 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c41: Fn2 = v0.clone();
                        c41.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a46 = {
            let (h45, _) = v2;
            h45
        };
        let a47 = 1u64;
        nat_sub(a46, a47)
    };
    Ok((v3, {
        let a53 = {
            let a49 = {
                let (_, h48) = v2;
                h48
            };
            let a50 = 256u64;
            nat_mul(a49, a50)?
        };
        let a54 = f17(v1, {
            let a51 = v0;
            let a52 = v3;
            nat_add(a51, a52)?
        })?;
        nat_add(a53, a54)?
    }))
}

pub fn f10(v0: Bytes, v1: Adt0) -> R<Adt0> {
    if match v1.clone() {
        Adt0::C0(_, h55, _, _) => {
            h55
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt0::C0(h56, _, _, _) => {
                h56
            }
        };
        if f18(v0.clone(), v2, 8u64) {
            let v3: u64 = f2(v0.clone(), v2, 8u64)?;
            if if f18(v0.clone(), {
                let a57 = v2;
                let a58 = 8u64;
                nat_add(a57, a58)?
            }, v3) {
                f18(v0.clone(), {
                    let a61 = {
                        let a59 = v2;
                        let a60 = 8u64;
                        nat_add(a59, a60)?
                    };
                    let a62 = v3;
                    nat_add(a61, a62)?
                }, 4u64)
            } else {
                false
            } {
                f19(v0.clone(), v1, f1(v0.clone(), {
                    let a63 = v2;
                    let a64 = 8u64;
                    nat_add(a63, a64)?
                }, v3), f2(v0, {
                    let a67 = {
                        let a65 = v2;
                        let a66 = 8u64;
                        nat_add(a65, a66)?
                    };
                    let a68 = v3;
                    nat_add(a67, a68)?
                }, 4u64)?, {
                    let a73 = {
                        let a71 = {
                            let a69 = v2;
                            let a70 = 8u64;
                            nat_add(a69, a70)?
                        };
                        let a72 = v3;
                        nat_add(a71, a72)?
                    };
                    let a74 = 4u64;
                    nat_add(a73, a74)?
                })
            } else {
                Ok(f20(v1))
            }
        } else {
            Ok(f20(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f11(v0: Bytes, v1: Adt1) -> R<Adt1> {
    if match v1.clone() {
        Adt1::C0(_, h75, _) => {
            h75
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt1::C0(h76, _, _) => {
                h76
            }
        };
        if f18(v0.clone(), v2, 8u64) {
            let v3: u64 = f2(v0.clone(), v2, 8u64)?;
            if if f18(v0.clone(), {
                let a77 = v2;
                let a78 = 8u64;
                nat_add(a77, a78)?
            }, v3) {
                f18(v0.clone(), {
                    let a81 = {
                        let a79 = v2;
                        let a80 = 8u64;
                        nat_add(a79, a80)?
                    };
                    let a82 = v3;
                    nat_add(a81, a82)?
                }, 4u64)
            } else {
                false
            } {
                f21(v0.clone(), v1, f1(v0, {
                    let a83 = v2;
                    let a84 = 8u64;
                    nat_add(a83, a84)?
                }, v3), {
                    let a87 = {
                        let a85 = v2;
                        let a86 = 8u64;
                        nat_add(a85, a86)?
                    };
                    let a88 = v3;
                    nat_add(a87, a88)?
                })
            } else {
                Ok(f22(v1))
            }
        } else {
            Ok(f22(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f12(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a89 = v0;
        let a90 = v1;
        let a91 = 0u64;
        nat_rem(a89, a90, a91)
    };
    let a92 = v2;
    let a93 = 0u64;
    if nat_eq(a92, a93) {
        Ok(v0)
    } else {
        let a96 = v0;
        let a97 = {
            let a94 = v1;
            let a95 = v2;
            nat_sub(a94, a95)
        };
        nat_add(a96, a97)
    }
}

pub fn f13(v0: Fn3, p101: Adt3, p102: List<Adt2>) -> R<Adt3> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v1, v2) = m100;
                let m98 = v2;
                match m98.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt2>), R<Adt3>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt2>), R<Adt3>>(({
                            let c99: Fn3 = v0.clone();
                            c99.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: Bytes, v3: Adt3, v4: Adt2) -> R<Adt3> {
    f23(v2, v1, v0, v3, v4)
}

pub fn f15(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h103) = f24(Fn4::F25, v1, (v0, Bytes::lit(&[])))?;
    Ok(h103)
}

pub fn f16(v0: List<(Bytes, Bytes)>) -> Bytes {
    f26(Fn5::F27, Bytes::lit(&[]), v0)
}

pub fn f17(v0: Bytes, v1: u64) -> R<u64> {
    let m106 = {
        let a104 = v0;
        let a105 = v1;
        index_bytes(a104, a105)
    };
    match m106 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a151 = {
                let a127 = {
                    let a115 = {
                        let a109 = {
                            let a107 = v3;
                            let a108 = 128u8;
                            fixed_u8::bit_and(a107, a108)
                        };
                        let a110 = 128u8;
                        if equal(a109, a110) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a116 = {
                        let a113 = {
                            let a111 = v3;
                            let a112 = 64u8;
                            fixed_u8::bit_and(a111, a112)
                        };
                        let a114 = 64u8;
                        if equal(a113, a114) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a115, a116)?
                };
                let a128 = {
                    let a125 = {
                        let a119 = {
                            let a117 = v3;
                            let a118 = 32u8;
                            fixed_u8::bit_and(a117, a118)
                        };
                        let a120 = 32u8;
                        if equal(a119, a120) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a126 = {
                        let a123 = {
                            let a121 = v3;
                            let a122 = 16u8;
                            fixed_u8::bit_and(a121, a122)
                        };
                        let a124 = 16u8;
                        if equal(a123, a124) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a125, a126)?
                };
                nat_add(a127, a128)?
            };
            let a152 = {
                let a149 = {
                    let a137 = {
                        let a131 = {
                            let a129 = v3;
                            let a130 = 8u8;
                            fixed_u8::bit_and(a129, a130)
                        };
                        let a132 = 8u8;
                        if equal(a131, a132) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a138 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 4u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 4u8;
                        if equal(a135, a136) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a137, a138)?
                };
                let a150 = {
                    let a147 = {
                        let a141 = {
                            let a139 = v3;
                            let a140 = 2u8;
                            fixed_u8::bit_and(a139, a140)
                        };
                        let a142 = 2u8;
                        if equal(a141, a142) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a148 = {
                        let a145 = {
                            let a143 = v3;
                            let a144 = 1u8;
                            fixed_u8::bit_and(a143, a144)
                        };
                        let a146 = 1u8;
                        if equal(a145, a146) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a147, a148)?
                };
                nat_add(a149, a150)?
            };
            nat_add(a151, a152)
        }
    }
}

pub fn f18(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a153 = v0;
        length_bytes(a153)
    };
    let a154 = v1;
    let a155 = v3;
    if nat_le(a154, a155) {
        let a158 = v2;
        let a159 = {
            let a156 = v3;
            let a157 = v1;
            nat_sub(a156, a157)
        };
        nat_le(a158, a159)
    } else {
        false
    }
}

pub fn f19(v0: Bytes, v1: Adt0, v2: Bytes, v3: u64, v4: u64) -> R<Adt0> {
    let v5: (bool, u64) = f28(v0.clone(), v4, v3)?;
    let (h160, _) = v5;
    if h160 {
        let v6: u64 = {
            let a161 = v2.clone();
            let a162 = f29(Str::lit("general.alignment"));
            if if equal(a161, a162) {
                let a163 = v3;
                let a164 = 4u64;
                nat_eq(a163, a164)
            } else {
                false
            } {
                f2(v0.clone(), v4, 4u64)?
            } else {
                match v1.clone() {
                    Adt0::C0(_, _, h165, _) => {
                        h165
                    }
                }
            }
        };
        let a166 = 8u64;
        let a167 = v6;
        if if nat_le(a166, a167) {
            f30(v6)
        } else {
            false
        } {
            Ok(Adt0::C0({
                let (_, h168) = v5;
                h168
            }, true, v6, f31(v2.clone(), {
                let a174 = f32(v2)?;
                let a175 = {
                    let a172 = f15(v3, 4u64)?;
                    let a173 = f33(v0, v3, v4, {
                        let a170 = {
                            let (_, h169) = v5;
                            h169
                        };
                        let a171 = v4;
                        nat_sub(a170, a171)
                    })?;
                    append_bytes(a172, a173)
                };
                append_bytes(a174, a175)
            }, match v1 {
                Adt0::C0(_, _, _, h176) => {
                    h176
                }
            })))
        } else {
            Ok(f20(v1))
        }
    } else {
        Ok(f20(v1))
    }
}

pub fn f20(v0: Adt0) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h177, _, _, _) => {
            h177
        }
    }, false, match v0 {
        Adt0::C0(_, _, h178, _) => {
            h178
        }
    }, List::<(Bytes, Bytes)>::nil())
}

pub fn f21(v0: Bytes, v1: Adt1, v2: Bytes, v3: u64) -> R<Adt1> {
    let v4: u64 = f2(v0.clone(), v3, 4u64)?;
    let a179 = v4;
    let a180 = 4u64;
    if if nat_le(a179, a180) {
        f18(v0.clone(), {
            let a181 = v3;
            let a182 = 4u64;
            nat_add(a181, a182)?
        }, {
            let a185 = {
                let a183 = v4;
                let a184 = 8u64;
                nat_mul(a183, a184)?
            };
            let a186 = 12u64;
            nat_add(a185, a186)?
        })
    } else {
        false
    } {
        let v5: (bool, u64) = f34(v0.clone(), {
            let a187 = v3;
            let a188 = 4u64;
            nat_add(a187, a188)?
        }, v4)?;
        let v6: u64 = {
            let a193 = {
                let a189 = v3;
                let a190 = 4u64;
                nat_add(a189, a190)?
            };
            let a194 = {
                let a191 = v4;
                let a192 = 8u64;
                nat_mul(a191, a192)?
            };
            nat_add(a193, a194)?
        };
        let v7: (u64, u64) = f35(f2(v0.clone(), v6, 4u64)?)?;
        let v8: u64 = {
            let (_, h195) = v7;
            h195
        };
        let (h196, _) = v5;
        if if h196 {
            let a197 = 0u64;
            let a198 = v8;
            if nat_lt(a197, a198) {
                let a203 = {
                    let a200 = {
                        let (_, h199) = v5;
                        h199
                    };
                    let a201 = v8;
                    let a202 = 0u64;
                    nat_rem(a200, a201, a202)
                };
                let a204 = 0u64;
                nat_eq(a203, a204)
            } else {
                false
            }
        } else {
            false
        } {
            let v9: u64 = {
                let a206 = {
                    let (_, h205) = v5;
                    h205
                };
                let a207 = v8;
                let a208 = 0u64;
                nat_quot(a206, a207, a208)
            };
            let a213 = v9;
            let a214 = {
                let a210 = 18446744073709551615u64;
                let a211 = {
                    let (h209, _) = v7;
                    h209
                };
                let a212 = 0u64;
                nat_quot(a210, a211, a212)
            };
            if nat_le(a213, a214) {
                Ok(Adt1::C0({
                    let a215 = v6;
                    let a216 = 12u64;
                    nat_add(a215, a216)?
                }, true, f36(Adt2::C0(v2, f1(v0.clone(), v3, {
                    let a219 = {
                        let a217 = v4;
                        let a218 = 8u64;
                        nat_mul(a217, a218)?
                    };
                    let a220 = 8u64;
                    nat_add(a219, a220)?
                }), f2(v0, {
                    let a221 = v6;
                    let a222 = 4u64;
                    nat_add(a221, a222)?
                }, 8u64)?, {
                    let a224 = v9;
                    let a225 = {
                        let (h223, _) = v7;
                        h223
                    };
                    nat_mul(a224, a225)?
                }), match v1 {
                    Adt1::C0(_, _, h226) => {
                        h226
                    }
                })))
            } else {
                Ok(f22(v1))
            }
        } else {
            Ok(f22(v1))
        }
    } else {
        Ok(f22(v1))
    }
}

pub fn f22(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0 {
        Adt1::C0(h227, _, _) => {
            h227
        }
    }, false, List::<Adt2>::nil())
}

pub fn f23(v0: Bytes, v1: u64, v2: u64, v3: Adt3, v4: Adt2) -> R<Adt3> {
    if if match v3.clone() {
        Adt3::C0(h228, _, _) => {
            h228
        }
    } {
        let a231 = match v4.clone() {
            Adt2::C0(_, _, h229, _) => {
                h229
            }
        };
        let a232 = {
            let a230 = v0.clone();
            length_bytes(a230)
        };
        if nat_le(a231, a232) {
            let a237 = match v3.clone() {
                Adt3::C0(_, h233, _) => {
                    h233
                }
            };
            let a238 = {
                let a235 = 18446744073709551615u64;
                let a236 = match v4.clone() {
                    Adt2::C0(_, _, _, h234) => {
                        h234
                    }
                };
                nat_sub(a235, a236)
            };
            nat_le(a237, a238)
        } else {
            false
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a240 = v1;
            let a241 = match v4.clone() {
                Adt2::C0(_, _, h239, _) => {
                    h239
                }
            };
            nat_add(a240, a241)?
        };
        if f18(v0.clone(), v5, match v4.clone() {
            Adt2::C0(_, _, _, h242) => {
                h242
            }
        }) {
            Ok(Adt3::C0(true, f12({
                let a245 = match v3.clone() {
                    Adt3::C0(_, h243, _) => {
                        h243
                    }
                };
                let a246 = match v4.clone() {
                    Adt2::C0(_, _, _, h244) => {
                        h244
                    }
                };
                nat_add(a245, a246)?
            }, v2)?, {
                let a258 = match v3.clone() {
                    Adt3::C0(_, _, h247) => {
                        h247
                    }
                };
                let a259 = {
                    let a256 = f32(match v4.clone() {
                        Adt2::C0(h248, _, _, _) => {
                            h248
                        }
                    })?;
                    let a257 = {
                        let a254 = match v4.clone() {
                            Adt2::C0(_, h249, _, _) => {
                                h249
                            }
                        };
                        let a255 = {
                            let a252 = f15(match v3 {
                                Adt3::C0(_, h250, _) => {
                                    h250
                                }
                            }, 8u64)?;
                            let a253 = f32(f1(v0, v5, match v4 {
                                Adt2::C0(_, _, _, h251) => {
                                    h251
                                }
                            }))?;
                            append_bytes(a252, a253)
                        };
                        append_bytes(a254, a255)
                    };
                    append_bytes(a256, a257)
                };
                append_bytes(a258, a259)
            }))
        } else {
            Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
        }
    } else {
        Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
    }
}

pub fn f24(v0: Fn4, p263: u64, p264: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v1, v2) = m262;
                let m260 = v1;
                if m260 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m260 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c261: Fn4 = v0.clone();
                        c261.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a266 = {
            let (h265, _) = v0.clone();
            h265
        };
        let a267 = 256u64;
        let a268 = 0u64;
        nat_quot(a266, a267, a268)
    }, {
        let a274 = {
            let (_, h269) = v0;
            h269
        };
        let a275 = f37({
            let a271 = {
                let (h270, _) = v0;
                h270
            };
            let a272 = 256u64;
            let a273 = 0u64;
            nat_rem(a271, a272, a273)
        });
        append_bytes(a274, a275)
    })
}

pub fn f26(v0: Fn5, p279: Bytes, p280: List<(Bytes, Bytes)>) -> Bytes {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m276 = v2;
                match m276.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c277: Fn5 = v0.clone();
                            c277.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    let a282 = v0;
    let a283 = {
        let (_, h281) = v1;
        h281
    };
    append_bytes(a282, a283)
}

pub fn f28(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let v3: Adt4 = {
        let (h284, _) = f38(Fn6::F39(v0.clone()), {
            let a288 = {
                let a286 = {
                    let a285 = v0;
                    length_bytes(a285)
                };
                let a287 = 2u64;
                nat_mul(a286, a287)?
            };
            let a289 = 4u64;
            nat_add(a288, a289)?
        }, Adt4::C0(v1, true, List::cons(Adt5::C0(v2, 1u64, 0u64), List::<Adt5>::nil())))?;
        h284
    };
    if match v3.clone() {
        Adt4::C0(_, h290, _) => {
            h290
        }
    } {
        let m292 = match v3.clone() {
            Adt4::C0(_, _, h291) => {
                h291
            }
        };
        match m292.uncons() {
            None => {
                Ok((true, match v3 {
                    Adt4::C0(h293, _, _) => {
                        h293
                    }
                }))
            }
            Some((_, _)) => {
                Ok((false, 0u64))
            }
        }
    } else {
        Ok((false, 0u64))
    }
}

pub fn f29(v0: Str) -> Bytes {
    let a294 = v0;
    utf8_encode(a294)
}

pub fn f30(v0: u64) -> bool {
    let a296 = {
        let (h295, _) = f40(Fn7::F41, 64u64, v0);
        h295
    };
    let a297 = 1u64;
    nat_eq(a296, a297)
}

pub fn f31(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt6 = f42(Fn8::F43(v1.clone(), v0.clone()), Adt6::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt6::C0(h298, _) => {
            h298
        }
    } {
        f44(match v3 {
            Adt6::C0(_, h299) => {
                h299
            }
        })
    } else {
        f44(List::cons((v0, v1), match v3 {
            Adt6::C0(_, h300) => {
                h300
            }
        }))
    }
}

pub fn f32(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f45(v0)?;
    let (_, h301) = f24(Fn4::F46(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h301)
}

pub fn f33(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Bytes> {
    let a302 = 0u64;
    let a303 = f47(v1);
    if nat_lt(a302, a303) {
        Ok(f1(v0, v2, v3))
    } else {
        let a304 = v1;
        let a305 = 8u64;
        if nat_eq(a304, a305) {
            let a310 = f1(v0.clone(), v2, 8u64);
            let a311 = f32(f1(v0, {
                let a306 = v2;
                let a307 = 8u64;
                nat_add(a306, a307)?
            }, {
                let a308 = v3;
                let a309 = 8u64;
                nat_sub(a308, a309)
            }))?;
            Ok(append_bytes(a310, a311))
        } else {
            let a316 = f1(v0.clone(), v2, 12u64);
            let a317 = f32(f1(v0, {
                let a312 = v2;
                let a313 = 12u64;
                nat_add(a312, a313)?
            }, {
                let a314 = v3;
                let a315 = 12u64;
                nat_sub(a314, a315)
            }))?;
            Ok(append_bytes(a316, a317))
        }
    }
}

pub fn f34(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let (_, h318) = f48(Fn9::F49(v1, v0), v2, (0u64, (true, 1u64)))?;
    Ok(h318)
}

pub fn f35(v0: u64) -> R<(u64, u64)> {
    let v1: Bytes = Bytes::lit(&[0x00, 0x04, 0x01, 0x00, 0x02, 0x01, 0x00, 0x12, 0x02, 0x00, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x02, 0x00, 0x18, 0x02, 0x00, 0x22, 0x02, 0x00, 0x24, 0x02, 0x00, 0x54, 0x03, 0x00, 0x6e, 0x03, 0x00, 0x90, 0x03, 0x00, 0xb0, 0x03, 0x00, 0xd2, 0x03, 0x01, 0x24, 0x03, 0x00, 0x42, 0x03, 0x00, 0x4a, 0x03, 0x00, 0x62, 0x03, 0x00, 0x32, 0x03, 0x00, 0x12, 0x02, 0x00, 0x6e, 0x03, 0x00, 0x52, 0x03, 0x00, 0x88, 0x03, 0x00, 0x01, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x01, 0x00, 0x08, 0x01, 0x00, 0x08, 0x01, 0x00, 0x38, 0x03, 0x00, 0x02, 0x01]);
    let v2: u64 = {
        let a319 = v0;
        let a320 = 3u64;
        nat_mul(a319, a320)?
    };
    let v3: u64 = {
        let a321 = v0;
        let a322 = 31u64;
        if nat_lt(a321, a322) {
            f17(v1.clone(), {
                let a323 = v2;
                let a324 = 2u64;
                nat_add(a323, a324)?
            })?
        } else {
            0u64
        }
    };
    Ok(({
        let a329 = {
            let a325 = f17(v1.clone(), v2)?;
            let a326 = 256u64;
            nat_mul(a325, a326)?
        };
        let a330 = f17(v1, {
            let a327 = v2;
            let a328 = 1u64;
            nat_add(a327, a328)?
        })?;
        nat_add(a329, a330)?
    }, {
        let a331 = v3;
        let a332 = 1u64;
        if nat_eq(a331, a332) {
            1u64
        } else {
            let a333 = v3;
            let a334 = 2u64;
            if nat_eq(a333, a334) {
                32u64
            } else {
                let a335 = v3;
                let a336 = 3u64;
                if nat_eq(a335, a336) {
                    256u64
                } else {
                    0u64
                }
            }
        }
    }))
}

pub fn f36(v0: Adt2, v1: List<Adt2>) -> List<Adt2> {
    let v2: Adt7 = f50(Fn10::F51(std::rc::Rc::new(v0.clone())), Adt7::C0(false, List::<Adt2>::nil()), v1);
    if match v2.clone() {
        Adt7::C0(h337, _) => {
            h337
        }
    } {
        f52(match v2 {
            Adt7::C0(_, h338) => {
                h338
            }
        })
    } else {
        f52(List::cons(v0, match v2 {
            Adt7::C0(_, h339) => {
                h339
            }
        }))
    }
}

pub fn f37(v0: u64) -> Bytes {
    f1(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f38(v0: Fn6, p344: u64, p345: Adt4) -> R<(Adt4, bool)> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v1, v2) = m343;
                let m340 = v1;
                if m340 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m340 - 1;
                    let m342 = {
                        let c341: Fn6 = v0.clone();
                        c341.apply(v2.clone())?
                    };
                    match m342 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
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

pub fn f39(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    f53(v0, v1)
}

pub fn f40(v0: Fn7, p350: u64, p351: u64) -> (u64, bool) {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v1, v2) = m349;
                let m346 = v1;
                if m346 == 0 {
                    Err::<(u64, u64), (u64, bool)>((v2, false))
                } else {
                    let v3 = m346 - 1;
                    let m348 = {
                        let c347: Fn7 = v0.clone();
                        c347.apply(v2)
                    };
                    match m348 {
                        None => {
                            Err::<(u64, u64), (u64, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), (u64, bool)>((v3, v4))
                        }
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

pub fn f41(v0: u64) -> Option<u64> {
    let a352 = 1u64;
    let a353 = v0;
    if if nat_lt(a352, a353) {
        let a357 = {
            let a354 = v0;
            let a355 = 2u64;
            let a356 = 0u64;
            nat_rem(a354, a355, a356)
        };
        let a358 = 0u64;
        nat_eq(a357, a358)
    } else {
        false
    } {
        Some({
            let a359 = v0;
            let a360 = 2u64;
            let a361 = 0u64;
            nat_quot(a359, a360, a361)
        })
    } else {
        None::<u64>
    }
}

pub fn f42(v0: Fn8, p365: Adt6, p366: List<(Bytes, Bytes)>) -> Adt6 {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v1, v2) = m364;
                let m362 = v2;
                match m362.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Bytes)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Bytes)>), Adt6>(({
                            let c363: Fn8 = v0.clone();
                            c363.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: Bytes, v2: Adt6, v3: (Bytes, Bytes)) -> Adt6 {
    if match v2.clone() {
        Adt6::C0(h367, _) => {
            h367
        }
    } {
        Adt6::C0(true, List::cons(v3, match v2 {
            Adt6::C0(_, h368) => {
                h368
            }
        }))
    } else {
        if f54(v1.clone(), {
            let (h369, _) = v3.clone();
            h369
        }) {
            Adt6::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt6::C0(_, h370) => {
                    h370
                }
            })))
        } else {
            Adt6::C0(false, List::cons(v3, match v2 {
                Adt6::C0(_, h371) => {
                    h371
                }
            }))
        }
    }
}

pub fn f44(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f55(Fn11::F56, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f45(v0: Bytes) -> R<Bytes> {
    Ok(f57(f58(v0)?))
}

pub fn f46(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a373 = {
            let (h372, _) = v1.clone();
            h372
        };
        let a374 = 2u64;
        nat_mul(a373, a374)?
    };
    Ok(({
        let a376 = {
            let (h375, _) = v1;
            h375
        };
        let a377 = 1u64;
        nat_add(a376, a377)?
    }, {
        let a385 = {
            let (_, h378) = v1;
            h378
        };
        let a386 = f37({
            let a383 = {
                let a379 = f59(f17(v0.clone(), v2)?);
                let a380 = 16u64;
                nat_mul(a379, a380)?
            };
            let a384 = f59(f17(v0, {
                let a381 = v2;
                let a382 = 1u64;
                nat_add(a381, a382)?
            })?);
            nat_add(a383, a384)?
        });
        append_bytes(a385, a386)
    }))
}

pub fn f47(v0: u64) -> u64 {
    let a387 = v0;
    let a388 = 0u64;
    if if nat_eq(a387, a388) {
        true
    } else {
        let a389 = v0;
        let a390 = 1u64;
        if nat_eq(a389, a390) {
            true
        } else {
            let a391 = v0;
            let a392 = 7u64;
            nat_eq(a391, a392)
        }
    } {
        1u64
    } else {
        let a393 = v0;
        let a394 = 2u64;
        if if nat_eq(a393, a394) {
            true
        } else {
            let a395 = v0;
            let a396 = 3u64;
            nat_eq(a395, a396)
        } {
            2u64
        } else {
            let a397 = v0;
            let a398 = 4u64;
            if if nat_eq(a397, a398) {
                true
            } else {
                let a399 = v0;
                let a400 = 5u64;
                if nat_eq(a399, a400) {
                    true
                } else {
                    let a401 = v0;
                    let a402 = 6u64;
                    nat_eq(a401, a402)
                }
            } {
                4u64
            } else {
                let a403 = v0;
                let a404 = 10u64;
                if if nat_eq(a403, a404) {
                    true
                } else {
                    let a405 = v0;
                    let a406 = 11u64;
                    if nat_eq(a405, a406) {
                        true
                    } else {
                        let a407 = v0;
                        let a408 = 12u64;
                        nat_eq(a407, a408)
                    }
                } {
                    8u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f48(v0: Fn9, p412: u64, p413: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, v2) = m411;
                let m409 = v1;
                if m409 == 0 {
                    Err::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>(Ok(v2))
                } else {
                    let v3 = m409 - 1;
                    Ok::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>((v3, {
                        let c410: Fn9 = v0.clone();
                        c410.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: u64, v1: Bytes, v2: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let v3: u64 = f2(v1, {
        let a417 = v0;
        let a418 = {
            let a415 = {
                let (h414, _) = v2;
                h414
            };
            let a416 = 8u64;
            nat_mul(a415, a416)?
        };
        nat_add(a417, a418)?
    }, 8u64)?;
    let v4: u64 = {
        let (_, h419) = {
            let (_, h420) = v2;
            h420
        };
        h419
    };
    let v5: bool = {
        let (h421, _) = {
            let (_, h422) = v2;
            h422
        };
        if h421 {
            let a423 = v3;
            let a424 = 0u64;
            if nat_eq(a423, a424) {
                true
            } else {
                let a428 = v4;
                let a429 = {
                    let a425 = 18446744073709551615u64;
                    let a426 = v3;
                    let a427 = 0u64;
                    nat_quot(a425, a426, a427)
                };
                nat_le(a428, a429)
            }
        } else {
            false
        }
    };
    Ok(({
        let a431 = {
            let (h430, _) = v2;
            h430
        };
        let a432 = 1u64;
        nat_add(a431, a432)?
    }, (v5, if v5 {
        let a433 = v4;
        let a434 = v3;
        nat_mul(a433, a434)?
    } else {
        0u64
    })))
}

pub fn f50(v0: Fn10, p438: Adt7, p439: List<Adt2>) -> Adt7 {
    let m437 = (p438, p439);
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v1, v2) = m437;
                let m435 = v2;
                match m435.uncons() {
                    None => {
                        Err::<(Adt7, List<Adt2>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<Adt2>), Adt7>(({
                            let c436: Fn10 = v0.clone();
                            c436.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt2, v1: Adt7, v2: Adt2) -> Adt7 {
    if match v1.clone() {
        Adt7::C0(h440, _) => {
            h440
        }
    } {
        Adt7::C0(true, List::cons(v2, match v1 {
            Adt7::C0(_, h441) => {
                h441
            }
        }))
    } else {
        if f54(match v0.clone() {
            Adt2::C0(h442, _, _, _) => {
                h442
            }
        }, match v2.clone() {
            Adt2::C0(h443, _, _, _) => {
                h443
            }
        }) {
            Adt7::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt7::C0(_, h444) => {
                    h444
                }
            })))
        } else {
            Adt7::C0(false, List::cons(v2, match v1 {
                Adt7::C0(_, h445) => {
                    h445
                }
            }))
        }
    }
}

pub fn f52(v0: List<Adt2>) -> List<Adt2> {
    f60(Fn12::F61, List::<Adt2>::nil(), v0)
}

pub fn f53(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    if match v1.clone() {
        Adt4::C0(_, h446, _) => {
            h446
        }
    } {
        let m448 = match v1.clone() {
            Adt4::C0(_, _, h447) => {
                h447
            }
        };
        match m448.uncons() {
            None => {
                Ok(None::<Adt4>)
            }
            Some((v2, v3)) => {
                let a450 = match v2.clone() {
                    Adt5::C0(_, h449, _) => {
                        h449
                    }
                };
                let a451 = 0u64;
                if nat_eq(a450, a451) {
                    Ok(f62(match v1 {
                        Adt4::C0(h452, _, _) => {
                            h452
                        }
                    }, v3))
                } else {
                    f63(v0, match v1 {
                        Adt4::C0(h453, _, _) => {
                            h453
                        }
                    }, v2.clone(), List::cons(Adt5::C0(match v2.clone() {
                        Adt5::C0(h454, _, _) => {
                            h454
                        }
                    }, {
                        let a456 = match v2.clone() {
                            Adt5::C0(_, h455, _) => {
                                h455
                            }
                        };
                        let a457 = 1u64;
                        nat_sub(a456, a457)
                    }, match v2 {
                        Adt5::C0(_, _, h458) => {
                            h458
                        }
                    }), v3))
                }
            }
        }
    } else {
        Ok(None::<Adt4>)
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> bool {
    let a463 = {
        let a459 = v0;
        let a460 = v1;
        compare_bytes(a459, a460)
    };
    let a464 = {
        let a461 = Bytes::lit(&[]);
        let a462 = Bytes::lit(&[0x01]);
        compare_bytes(a461, a462)
    };
    equal(a463, a464)
}

pub fn f55(v0: Fn11, p468: List<(Bytes, Bytes)>, p469: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m467 = (p468, p469);
    {
        let mut m467 = m467;
        loop {
            let step = {
                let (v1, v2) = m467;
                let m465 = v2;
                match m465.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c466: Fn11 = v0.clone();
                            c466.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f57(v0: Adt8) -> Bytes {
    let a490 = f64(match v0.clone() {
        Adt8::C0(h470, _, _, _, _, _, _, _) => {
            h470
        }
    });
    let a491 = {
        let a488 = f64(match v0.clone() {
            Adt8::C0(_, h471, _, _, _, _, _, _) => {
                h471
            }
        });
        let a489 = {
            let a486 = f64(match v0.clone() {
                Adt8::C0(_, _, h472, _, _, _, _, _) => {
                    h472
                }
            });
            let a487 = {
                let a484 = f64(match v0.clone() {
                    Adt8::C0(_, _, _, h473, _, _, _, _) => {
                        h473
                    }
                });
                let a485 = {
                    let a482 = f64(match v0.clone() {
                        Adt8::C0(_, _, _, _, h474, _, _, _) => {
                            h474
                        }
                    });
                    let a483 = {
                        let a480 = f64(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, h475, _, _) => {
                                h475
                            }
                        });
                        let a481 = {
                            let a478 = f64(match v0.clone() {
                                Adt8::C0(_, _, _, _, _, _, h476, _) => {
                                    h476
                                }
                            });
                            let a479 = f64(match v0 {
                                Adt8::C0(_, _, _, _, _, _, _, h477) => {
                                    h477
                                }
                            });
                            append_bytes(a478, a479)
                        };
                        append_bytes(a480, a481)
                    };
                    append_bytes(a482, a483)
                };
                append_bytes(a484, a485)
            };
            append_bytes(a486, a487)
        };
        append_bytes(a488, a489)
    };
    append_bytes(a490, a491)
}

pub fn f58(v0: Bytes) -> R<Adt8> {
    let v1: u64 = {
        let a492 = v0.clone();
        length_bytes(a492)
    };
    let v2: u64 = {
        let a498 = {
            let a495 = {
                let a493 = v1;
                let a494 = 72u64;
                nat_add(a493, a494)?
            };
            let a496 = 64u64;
            let a497 = 0u64;
            nat_quot(a495, a496, a497)
        };
        let a499 = 64u64;
        nat_mul(a498, a499)?
    };
    let (_, h500) = f65(Fn13::F66(v0, v1, v2), {
        let a501 = v2;
        let a502 = 64u64;
        let a503 = 0u64;
        nat_quot(a501, a502, a503)
    }, (0u64, f67()))?;
    Ok(h500)
}

pub fn f59(v0: u64) -> u64 {
    let a504 = 47u64;
    let a505 = v0;
    if nat_lt(a504, a505) {
        let a506 = v0;
        let a507 = 58u64;
        if nat_lt(a506, a507) {
            let a508 = v0;
            let a509 = 48u64;
            nat_sub(a508, a509)
        } else {
            let a510 = 64u64;
            let a511 = v0;
            if nat_lt(a510, a511) {
                let a512 = v0;
                let a513 = 71u64;
                if nat_lt(a512, a513) {
                    let a514 = v0;
                    let a515 = 55u64;
                    nat_sub(a514, a515)
                } else {
                    let a516 = 96u64;
                    let a517 = v0;
                    if nat_lt(a516, a517) {
                        let a518 = v0;
                        let a519 = 103u64;
                        if nat_lt(a518, a519) {
                            let a520 = v0;
                            let a521 = 87u64;
                            nat_sub(a520, a521)
                        } else {
                            16u64
                        }
                    } else {
                        16u64
                    }
                }
            } else {
                16u64
            }
        }
    } else {
        16u64
    }
}

pub fn f60(v0: Fn12, p525: List<Adt2>, p526: List<Adt2>) -> List<Adt2> {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v1, v2) = m524;
                let m522 = v2;
                match m522.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c523: Fn12 = v0.clone();
                            c523.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f62(v0: u64, v1: List<Adt5>) -> Option<Adt4> {
    Some(Adt4::C0(v0, true, v1))
}

pub fn f63(v0: Bytes, v1: u64, v2: Adt5, v3: List<Adt5>) -> R<Option<Adt4>> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h527, _, _) => {
            h527
        }
    };
    let v5: u64 = f47(v4);
    let a528 = 0u64;
    let a529 = v5;
    if nat_lt(a528, a529) {
        if f18(v0, v1, v5) {
            Ok(f62({
                let a530 = v1;
                let a531 = v5;
                nat_add(a530, a531)?
            }, v3))
        } else {
            Ok(f68(v1))
        }
    } else {
        let a532 = v4;
        let a533 = 8u64;
        if nat_eq(a532, a533) {
            if f18(v0.clone(), v1, 8u64) {
                let v6: u64 = f2(v0.clone(), v1, 8u64)?;
                if f18(v0, {
                    let a534 = v1;
                    let a535 = 8u64;
                    nat_add(a534, a535)?
                }, v6) {
                    Ok(f62({
                        let a538 = {
                            let a536 = v1;
                            let a537 = 8u64;
                            nat_add(a536, a537)?
                        };
                        let a539 = v6;
                        nat_add(a538, a539)?
                    }, v3))
                } else {
                    Ok(f68(v1))
                }
            } else {
                Ok(f68(v1))
            }
        } else {
            let a540 = v4;
            let a541 = 9u64;
            if nat_eq(a540, a541) {
                let a543 = 64u64;
                let a544 = match v2.clone() {
                    Adt5::C0(_, _, h542) => {
                        h542
                    }
                };
                if nat_le(a543, a544) {
                    Ok(f68(v1))
                } else {
                    if f18(v0.clone(), v1, 12u64) {
                        Ok(f62({
                            let a545 = v1;
                            let a546 = 12u64;
                            nat_add(a545, a546)?
                        }, List::cons(Adt5::C0(f2(v0.clone(), v1, 4u64)?, f2(v0, {
                            let a547 = v1;
                            let a548 = 4u64;
                            nat_add(a547, a548)?
                        }, 8u64)?, {
                            let a550 = match v2 {
                                Adt5::C0(_, _, h549) => {
                                    h549
                                }
                            };
                            let a551 = 1u64;
                            nat_add(a550, a551)?
                        }), v3)))
                    } else {
                        Ok(f68(v1))
                    }
                }
            } else {
                Ok(f68(v1))
            }
        }
    }
}

pub fn f64(v0: u32) -> Bytes {
    let a580 = f69({
        let a552 = f70(v0, 28u32);
        let a553 = 15u32;
        fixed_u32::bit_and(a552, a553)
    });
    let a581 = {
        let a578 = f69({
            let a554 = f70(v0, 24u32);
            let a555 = 15u32;
            fixed_u32::bit_and(a554, a555)
        });
        let a579 = {
            let a576 = f69({
                let a556 = f70(v0, 20u32);
                let a557 = 15u32;
                fixed_u32::bit_and(a556, a557)
            });
            let a577 = {
                let a574 = f69({
                    let a558 = f70(v0, 16u32);
                    let a559 = 15u32;
                    fixed_u32::bit_and(a558, a559)
                });
                let a575 = {
                    let a572 = f69({
                        let a560 = f70(v0, 12u32);
                        let a561 = 15u32;
                        fixed_u32::bit_and(a560, a561)
                    });
                    let a573 = {
                        let a570 = f69({
                            let a562 = f70(v0, 8u32);
                            let a563 = 15u32;
                            fixed_u32::bit_and(a562, a563)
                        });
                        let a571 = {
                            let a568 = f69({
                                let a564 = f70(v0, 4u32);
                                let a565 = 15u32;
                                fixed_u32::bit_and(a564, a565)
                            });
                            let a569 = f69({
                                let a566 = v0;
                                let a567 = 15u32;
                                fixed_u32::bit_and(a566, a567)
                            });
                            append_bytes(a568, a569)
                        };
                        append_bytes(a570, a571)
                    };
                    append_bytes(a572, a573)
                };
                append_bytes(a574, a575)
            };
            append_bytes(a576, a577)
        };
        append_bytes(a578, a579)
    };
    append_bytes(a580, a581)
}

pub fn f65(v0: Fn13, p585: u64, p586: (u64, Adt8)) -> R<(u64, Adt8)> {
    let m584 = (p585, p586);
    {
        let mut m584 = m584;
        loop {
            let step = {
                let (v1, v2) = m584;
                let m582 = v1;
                if m582 == 0 {
                    Err::<(u64, (u64, Adt8)), R<(u64, Adt8)>>(Ok(v2))
                } else {
                    let v3 = m582 - 1;
                    Ok::<(u64, (u64, Adt8)), R<(u64, Adt8)>>((v3, {
                        let c583: Fn13 = v0.clone();
                        c583.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt8)) -> R<(u64, Adt8)> {
    Ok(({
        let a588 = {
            let (h587, _) = v3.clone();
            h587
        };
        let a589 = 64u64;
        nat_add(a588, a589)?
    }, f71({
        let (_, h590) = v3;
        h590
    }, f72(v0, v1, v2, {
        let (h591, _) = v3;
        h591
    })?)?))
}

pub fn f67() -> Adt8 {
    Adt8::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f68(v0: u64) -> Option<Adt4> {
    Some(Adt4::C0(v0, false, List::<Adt5>::nil()))
}

pub fn f69(v0: u32) -> Bytes {
    let a594 = {
        let a592 = v0;
        let a593 = 8u32;
        fixed_u32::bit_and(a592, a593)
    };
    let a595 = 0u32;
    if equal(a594, a595) {
        let a598 = {
            let a596 = v0;
            let a597 = 4u32;
            fixed_u32::bit_and(a596, a597)
        };
        let a599 = 0u32;
        if equal(a598, a599) {
            let a602 = {
                let a600 = v0;
                let a601 = 2u32;
                fixed_u32::bit_and(a600, a601)
            };
            let a603 = 0u32;
            if equal(a602, a603) {
                let a606 = {
                    let a604 = v0;
                    let a605 = 1u32;
                    fixed_u32::bit_and(a604, a605)
                };
                let a607 = 0u32;
                if equal(a606, a607) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a610 = {
                    let a608 = v0;
                    let a609 = 1u32;
                    fixed_u32::bit_and(a608, a609)
                };
                let a611 = 0u32;
                if equal(a610, a611) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a614 = {
                let a612 = v0;
                let a613 = 2u32;
                fixed_u32::bit_and(a612, a613)
            };
            let a615 = 0u32;
            if equal(a614, a615) {
                let a618 = {
                    let a616 = v0;
                    let a617 = 1u32;
                    fixed_u32::bit_and(a616, a617)
                };
                let a619 = 0u32;
                if equal(a618, a619) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a622 = {
                    let a620 = v0;
                    let a621 = 1u32;
                    fixed_u32::bit_and(a620, a621)
                };
                let a623 = 0u32;
                if equal(a622, a623) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a626 = {
            let a624 = v0;
            let a625 = 4u32;
            fixed_u32::bit_and(a624, a625)
        };
        let a627 = 0u32;
        if equal(a626, a627) {
            let a630 = {
                let a628 = v0;
                let a629 = 2u32;
                fixed_u32::bit_and(a628, a629)
            };
            let a631 = 0u32;
            if equal(a630, a631) {
                let a634 = {
                    let a632 = v0;
                    let a633 = 1u32;
                    fixed_u32::bit_and(a632, a633)
                };
                let a635 = 0u32;
                if equal(a634, a635) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a638 = {
                    let a636 = v0;
                    let a637 = 1u32;
                    fixed_u32::bit_and(a636, a637)
                };
                let a639 = 0u32;
                if equal(a638, a639) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a642 = {
                let a640 = v0;
                let a641 = 2u32;
                fixed_u32::bit_and(a640, a641)
            };
            let a643 = 0u32;
            if equal(a642, a643) {
                let a646 = {
                    let a644 = v0;
                    let a645 = 1u32;
                    fixed_u32::bit_and(a644, a645)
                };
                let a647 = 0u32;
                if equal(a646, a647) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a650 = {
                    let a648 = v0;
                    let a649 = 1u32;
                    fixed_u32::bit_and(a648, a649)
                };
                let a651 = 0u32;
                if equal(a650, a651) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f70(v0: u32, v1: u32) -> u32 {
    let m654 = {
        let a652 = v0;
        let a653 = v1;
        fixed_u32::shift_right(a652, a653)
    };
    match m654 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f71(v0: Adt8, v1: Adt9) -> R<Adt8> {
    let v2: Adt10 = f73(Fn14::F74, 64u64, Adt10::C0(match v0.clone() {
        Adt8::C0(h655, _, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt8::C0(_, h656, _, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt8::C0(_, _, h657, _, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h658, _, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h659, _, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h660, _, _) => {
            h660
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h661, _) => {
            h661
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h662) => {
            h662
        }
    }, std::rc::Rc::new(v1), 0u64))?;
    Ok(Adt8::C0(f75(match v0.clone() {
        Adt8::C0(h663, _, _, _, _, _, _, _) => {
            h663
        }
    }, match v2.clone() {
        Adt10::C0(h664, _, _, _, _, _, _, _, _, _) => {
            h664
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, h665, _, _, _, _, _, _) => {
            h665
        }
    }, match v2.clone() {
        Adt10::C0(_, h666, _, _, _, _, _, _, _, _) => {
            h666
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, _, h667, _, _, _, _, _) => {
            h667
        }
    }, match v2.clone() {
        Adt10::C0(_, _, h668, _, _, _, _, _, _, _) => {
            h668
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, _, _, h669, _, _, _, _) => {
            h669
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, h670, _, _, _, _, _, _) => {
            h670
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, _, _, _, h671, _, _, _) => {
            h671
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, h672, _, _, _, _, _) => {
            h672
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, _, _, _, _, h673, _, _) => {
            h673
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, h674, _, _, _, _) => {
            h674
        }
    }), f75(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h675, _) => {
            h675
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, _, h676, _, _, _) => {
            h676
        }
    }), f75(match v0 {
        Adt8::C0(_, _, _, _, _, _, _, h677) => {
            h677
        }
    }, match v2 {
        Adt10::C0(_, _, _, _, _, _, _, h678, _, _) => {
            h678
        }
    })))
}

pub fn f72(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    Ok(Adt9::C0(f76(v0.clone(), v1, v2, v3)?, f76(v0.clone(), v1, v2, {
        let a679 = v3;
        let a680 = 4u64;
        nat_add(a679, a680)?
    })?, f76(v0.clone(), v1, v2, {
        let a681 = v3;
        let a682 = 8u64;
        nat_add(a681, a682)?
    })?, f76(v0.clone(), v1, v2, {
        let a683 = v3;
        let a684 = 12u64;
        nat_add(a683, a684)?
    })?, f76(v0.clone(), v1, v2, {
        let a685 = v3;
        let a686 = 16u64;
        nat_add(a685, a686)?
    })?, f76(v0.clone(), v1, v2, {
        let a687 = v3;
        let a688 = 20u64;
        nat_add(a687, a688)?
    })?, f76(v0.clone(), v1, v2, {
        let a689 = v3;
        let a690 = 24u64;
        nat_add(a689, a690)?
    })?, f76(v0.clone(), v1, v2, {
        let a691 = v3;
        let a692 = 28u64;
        nat_add(a691, a692)?
    })?, f76(v0.clone(), v1, v2, {
        let a693 = v3;
        let a694 = 32u64;
        nat_add(a693, a694)?
    })?, f76(v0.clone(), v1, v2, {
        let a695 = v3;
        let a696 = 36u64;
        nat_add(a695, a696)?
    })?, f76(v0.clone(), v1, v2, {
        let a697 = v3;
        let a698 = 40u64;
        nat_add(a697, a698)?
    })?, f76(v0.clone(), v1, v2, {
        let a699 = v3;
        let a700 = 44u64;
        nat_add(a699, a700)?
    })?, f76(v0.clone(), v1, v2, {
        let a701 = v3;
        let a702 = 48u64;
        nat_add(a701, a702)?
    })?, f76(v0.clone(), v1, v2, {
        let a703 = v3;
        let a704 = 52u64;
        nat_add(a703, a704)?
    })?, f76(v0.clone(), v1, v2, {
        let a705 = v3;
        let a706 = 56u64;
        nat_add(a705, a706)?
    })?, f76(v0, v1, v2, {
        let a707 = v3;
        let a708 = 60u64;
        nat_add(a707, a708)?
    })?))
}

pub fn f73(v0: Fn14, p712: u64, p713: Adt10) -> R<Adt10> {
    let m711 = (p712, p713);
    {
        let mut m711 = m711;
        loop {
            let step = {
                let (v1, v2) = m711;
                let m709 = v1;
                if m709 == 0 {
                    Err::<(u64, Adt10), R<Adt10>>(Ok(v2))
                } else {
                    let v3 = m709 - 1;
                    Ok::<(u64, Adt10), R<Adt10>>((v3, {
                        let c710: Fn14 = v0.clone();
                        c710.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Adt10) -> R<Adt10> {
    f77(v0)
}

pub fn f75(v0: u32, v1: u32) -> u32 {
    let m716 = {
        let a714 = f78(v0);
        let a715 = f78(v1);
        fixed_u64::checked_add(a714, a715)
    };
    match m716 {
        None => {
            0u32
        }
        Some(v2) => {
            f79(v2)
        }
    }
}

pub fn f76(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a719 = {
        let a717 = v3;
        let a718 = 8u64;
        nat_add(a717, a718)?
    };
    let a720 = v2;
    if nat_eq(a719, a720) {
        Ok(f80({
            let a723 = {
                let a721 = v1;
                let a722 = 8u64;
                nat_mul(a721, a722)?
            };
            let a724 = 4294967296u64;
            let a725 = 0u64;
            nat_quot(a723, a724, a725)
        }))
    } else {
        let a728 = {
            let a726 = v3;
            let a727 = 4u64;
            nat_add(a726, a727)?
        };
        let a729 = v2;
        if nat_eq(a728, a729) {
            Ok(f80({
                let a732 = {
                    let a730 = v1;
                    let a731 = 8u64;
                    nat_mul(a730, a731)?
                };
                let a733 = 4294967296u64;
                let a734 = 0u64;
                nat_rem(a732, a733, a734)
            }))
        } else {
            Ok(f81(f82(v0.clone(), v1, v3), f82(v0.clone(), v1, {
                let a735 = v3;
                let a736 = 1u64;
                nat_add(a735, a736)?
            }), f82(v0.clone(), v1, {
                let a737 = v3;
                let a738 = 2u64;
                nat_add(a737, a738)?
            }), f82(v0, v1, {
                let a739 = v3;
                let a740 = 3u64;
                nat_add(a739, a740)?
            })))
        }
    }
}

pub fn f77(v0: Adt10) -> R<Adt10> {
    let v1: u32 = f83(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, h741) => {
            h741
        }
    });
    let v2: u32 = f75(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, h742, _, _) => {
            h742
        }
    }, f75(f84(match v0.clone() {
        Adt10::C0(_, _, _, _, h743, _, _, _, _, _) => {
            h743
        }
    }), f75(f85(match v0.clone() {
        Adt10::C0(_, _, _, _, h744, _, _, _, _, _) => {
            h744
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h745, _, _, _, _) => {
            h745
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h746, _, _, _) => {
            h746
        }
    }), f75(v1, {
        let m749 = match v0.clone() {
            Adt10::C0(_, _, _, _, _, _, _, _, h748, _) => {
                (*h748).clone()
            }
        };
        match m749 {
            Adt9::C0(h747, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h747
            }
        }
    }))));
    let v3: u32 = f75(f86(match v0.clone() {
        Adt10::C0(h750, _, _, _, _, _, _, _, _, _) => {
            h750
        }
    }), f87(match v0.clone() {
        Adt10::C0(h751, _, _, _, _, _, _, _, _, _) => {
            h751
        }
    }, match v0.clone() {
        Adt10::C0(_, h752, _, _, _, _, _, _, _, _) => {
            h752
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h753, _, _, _, _, _, _, _) => {
            h753
        }
    }));
    Ok(Adt10::C0(f75(v2, v3), match v0.clone() {
        Adt10::C0(h754, _, _, _, _, _, _, _, _, _) => {
            h754
        }
    }, match v0.clone() {
        Adt10::C0(_, h755, _, _, _, _, _, _, _, _) => {
            h755
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h756, _, _, _, _, _, _, _) => {
            h756
        }
    }, f75(match v0.clone() {
        Adt10::C0(_, _, _, h757, _, _, _, _, _, _) => {
            h757
        }
    }, v2), match v0.clone() {
        Adt10::C0(_, _, _, _, h758, _, _, _, _, _) => {
            h758
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h759, _, _, _, _) => {
            h759
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h760, _, _, _) => {
            h760
        }
    }, std::rc::Rc::new(f88(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, h761, _) => {
            (*h761).clone()
        }
    })), {
        let a763 = match v0 {
            Adt10::C0(_, _, _, _, _, _, _, _, _, h762) => {
                h762
            }
        };
        let a764 = 1u64;
        nat_add(a763, a764)?
    }))
}

pub fn f78(v0: u32) -> u64 {
    let m766 = {
        let a765 = v0;
        fixed_u64::convert(i128::from(a765))
    };
    match m766 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f79(v0: u64) -> u32 {
    let m770 = {
        let a769 = {
            let a767 = v0;
            let a768 = 4294967295u64;
            fixed_u64::bit_and(a767, a768)
        };
        fixed_u32::convert(i128::from(a769))
    };
    match m770 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f80(v0: u64) -> u32 {
    let (_, h771) = {
        let (_, h772) = f89(Fn15::F90, 32u64, (v0, (1u32, 0u32)));
        h772
    };
    h771
}

pub fn f81(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a777 = f91(v0, 24u32);
    let a778 = {
        let a775 = f91(v1, 16u32);
        let a776 = {
            let a773 = f91(v2, 8u32);
            let a774 = v3;
            fixed_u32::bit_or(a773, a774)
        };
        fixed_u32::bit_or(a775, a776)
    };
    fixed_u32::bit_or(a777, a778)
}

pub fn f82(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a779 = v2;
    let a780 = v1;
    if nat_lt(a779, a780) {
        let m783 = {
            let a781 = v0;
            let a782 = v2;
            index_bytes(a781, a782)
        };
        match m783 {
            None => {
                0u32
            }
            Some(v3) => {
                let m785 = {
                    let a784 = v3;
                    fixed_u32::convert(i128::from(a784))
                };
                match m785 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a786 = v2;
        let a787 = v1;
        if nat_eq(a786, a787) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f83(v0: u64) -> u32 {
    let a788 = v0;
    let a789 = 32u64;
    if nat_lt(a788, a789) {
        let a790 = v0;
        let a791 = 16u64;
        if nat_lt(a790, a791) {
            let a792 = v0;
            let a793 = 8u64;
            if nat_lt(a792, a793) {
                let a794 = v0;
                let a795 = 4u64;
                if nat_lt(a794, a795) {
                    let a796 = v0;
                    let a797 = 2u64;
                    if nat_lt(a796, a797) {
                        let a798 = v0;
                        let a799 = 1u64;
                        if nat_lt(a798, a799) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a800 = v0;
                        let a801 = 3u64;
                        if nat_lt(a800, a801) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a802 = v0;
                    let a803 = 6u64;
                    if nat_lt(a802, a803) {
                        let a804 = v0;
                        let a805 = 5u64;
                        if nat_lt(a804, a805) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a806 = v0;
                        let a807 = 7u64;
                        if nat_lt(a806, a807) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a808 = v0;
                let a809 = 12u64;
                if nat_lt(a808, a809) {
                    let a810 = v0;
                    let a811 = 10u64;
                    if nat_lt(a810, a811) {
                        let a812 = v0;
                        let a813 = 9u64;
                        if nat_lt(a812, a813) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a814 = v0;
                        let a815 = 11u64;
                        if nat_lt(a814, a815) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a816 = v0;
                    let a817 = 14u64;
                    if nat_lt(a816, a817) {
                        let a818 = v0;
                        let a819 = 13u64;
                        if nat_lt(a818, a819) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a820 = v0;
                        let a821 = 15u64;
                        if nat_lt(a820, a821) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a822 = v0;
            let a823 = 24u64;
            if nat_lt(a822, a823) {
                let a824 = v0;
                let a825 = 20u64;
                if nat_lt(a824, a825) {
                    let a826 = v0;
                    let a827 = 18u64;
                    if nat_lt(a826, a827) {
                        let a828 = v0;
                        let a829 = 17u64;
                        if nat_lt(a828, a829) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a830 = v0;
                        let a831 = 19u64;
                        if nat_lt(a830, a831) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a832 = v0;
                    let a833 = 22u64;
                    if nat_lt(a832, a833) {
                        let a834 = v0;
                        let a835 = 21u64;
                        if nat_lt(a834, a835) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a836 = v0;
                        let a837 = 23u64;
                        if nat_lt(a836, a837) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a838 = v0;
                let a839 = 28u64;
                if nat_lt(a838, a839) {
                    let a840 = v0;
                    let a841 = 26u64;
                    if nat_lt(a840, a841) {
                        let a842 = v0;
                        let a843 = 25u64;
                        if nat_lt(a842, a843) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a844 = v0;
                        let a845 = 27u64;
                        if nat_lt(a844, a845) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a846 = v0;
                    let a847 = 30u64;
                    if nat_lt(a846, a847) {
                        let a848 = v0;
                        let a849 = 29u64;
                        if nat_lt(a848, a849) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a850 = v0;
                        let a851 = 31u64;
                        if nat_lt(a850, a851) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a852 = v0;
        let a853 = 48u64;
        if nat_lt(a852, a853) {
            let a854 = v0;
            let a855 = 40u64;
            if nat_lt(a854, a855) {
                let a856 = v0;
                let a857 = 36u64;
                if nat_lt(a856, a857) {
                    let a858 = v0;
                    let a859 = 34u64;
                    if nat_lt(a858, a859) {
                        let a860 = v0;
                        let a861 = 33u64;
                        if nat_lt(a860, a861) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a862 = v0;
                        let a863 = 35u64;
                        if nat_lt(a862, a863) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a864 = v0;
                    let a865 = 38u64;
                    if nat_lt(a864, a865) {
                        let a866 = v0;
                        let a867 = 37u64;
                        if nat_lt(a866, a867) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a868 = v0;
                        let a869 = 39u64;
                        if nat_lt(a868, a869) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a870 = v0;
                let a871 = 44u64;
                if nat_lt(a870, a871) {
                    let a872 = v0;
                    let a873 = 42u64;
                    if nat_lt(a872, a873) {
                        let a874 = v0;
                        let a875 = 41u64;
                        if nat_lt(a874, a875) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a876 = v0;
                        let a877 = 43u64;
                        if nat_lt(a876, a877) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a878 = v0;
                    let a879 = 46u64;
                    if nat_lt(a878, a879) {
                        let a880 = v0;
                        let a881 = 45u64;
                        if nat_lt(a880, a881) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a882 = v0;
                        let a883 = 47u64;
                        if nat_lt(a882, a883) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a884 = v0;
            let a885 = 56u64;
            if nat_lt(a884, a885) {
                let a886 = v0;
                let a887 = 52u64;
                if nat_lt(a886, a887) {
                    let a888 = v0;
                    let a889 = 50u64;
                    if nat_lt(a888, a889) {
                        let a890 = v0;
                        let a891 = 49u64;
                        if nat_lt(a890, a891) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a892 = v0;
                        let a893 = 51u64;
                        if nat_lt(a892, a893) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a894 = v0;
                    let a895 = 54u64;
                    if nat_lt(a894, a895) {
                        let a896 = v0;
                        let a897 = 53u64;
                        if nat_lt(a896, a897) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a898 = v0;
                        let a899 = 55u64;
                        if nat_lt(a898, a899) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a900 = v0;
                let a901 = 60u64;
                if nat_lt(a900, a901) {
                    let a902 = v0;
                    let a903 = 58u64;
                    if nat_lt(a902, a903) {
                        let a904 = v0;
                        let a905 = 57u64;
                        if nat_lt(a904, a905) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a906 = v0;
                        let a907 = 59u64;
                        if nat_lt(a906, a907) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a908 = v0;
                    let a909 = 62u64;
                    if nat_lt(a908, a909) {
                        let a910 = v0;
                        let a911 = 61u64;
                        if nat_lt(a910, a911) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a912 = v0;
                        let a913 = 63u64;
                        if nat_lt(a912, a913) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f84(v0: u32) -> u32 {
    let a916 = {
        let a914 = f92(v0, 6u32, 26u32);
        let a915 = f92(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a914, a915)
    };
    let a917 = f92(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a916, a917)
}

pub fn f85(v0: u32, v1: u32, v2: u32) -> u32 {
    let a923 = {
        let a918 = v0;
        let a919 = v1;
        fixed_u32::bit_and(a918, a919)
    };
    let a924 = {
        let a921 = {
            let a920 = v0;
            fixed_u32::bit_not(a920)
        };
        let a922 = v2;
        fixed_u32::bit_and(a921, a922)
    };
    fixed_u32::bit_xor(a923, a924)
}

pub fn f86(v0: u32) -> u32 {
    let a927 = {
        let a925 = f92(v0, 2u32, 30u32);
        let a926 = f92(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a925, a926)
    };
    let a928 = f92(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a927, a928)
}

pub fn f87(v0: u32, v1: u32, v2: u32) -> u32 {
    let a937 = {
        let a933 = {
            let a929 = v0;
            let a930 = v1;
            fixed_u32::bit_and(a929, a930)
        };
        let a934 = {
            let a931 = v0;
            let a932 = v2;
            fixed_u32::bit_and(a931, a932)
        };
        fixed_u32::bit_xor(a933, a934)
    };
    let a938 = {
        let a935 = v1;
        let a936 = v2;
        fixed_u32::bit_and(a935, a936)
    };
    fixed_u32::bit_xor(a937, a938)
}

pub fn f88(v0: Adt9) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(_, h939, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h940, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, h941, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h942, _, _, _, _, _, _, _, _, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h943, _, _, _, _, _, _, _, _, _, _) => {
            h943
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h944, _, _, _, _, _, _, _, _, _) => {
            h944
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h945, _, _, _, _, _, _, _, _) => {
            h945
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h946, _, _, _, _, _, _, _) => {
            h946
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h947, _, _, _, _, _, _) => {
            h947
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h948, _, _, _, _, _) => {
            h948
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h949, _, _, _, _) => {
            h949
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h950, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h951, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h952, _) => {
            h952
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h953) => {
            h953
        }
    }, f75(f93(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h954, _) => {
            h954
        }
    }), f75(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h955, _, _, _, _, _, _) => {
            h955
        }
    }, f75(f94(match v0.clone() {
        Adt9::C0(_, h956, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h956
        }
    }), match v0 {
        Adt9::C0(h957, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h957
        }
    }))))
}

pub fn f89(v0: Fn15, p961: u64, p962: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m960 = (p961, p962);
    {
        let mut m960 = m960;
        loop {
            let step = {
                let (v1, v2) = m960;
                let m958 = v1;
                if m958 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m958 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c959: Fn15 = v0.clone();
                        c959.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h963, _) = v0;
        h963
    };
    let v2: u32 = {
        let (h964, _) = {
            let (_, h965) = v0;
            h965
        };
        h964
    };
    let v3: u32 = {
        let (_, h966) = {
            let (_, h967) = v0;
            h967
        };
        h966
    };
    ({
        let a968 = v1;
        let a969 = 2u64;
        let a970 = 0u64;
        nat_quot(a968, a969, a970)
    }, (f91(v2, 1u32), {
        let a974 = {
            let a971 = v1;
            let a972 = 2u64;
            let a973 = 0u64;
            nat_rem(a971, a972, a973)
        };
        let a975 = 1u64;
        if nat_eq(a974, a975) {
            let a976 = v3;
            let a977 = v2;
            fixed_u32::bit_or(a976, a977)
        } else {
            v3
        }
    }))
}

pub fn f91(v0: u32, v1: u32) -> u32 {
    let m980 = {
        let a978 = v0;
        let a979 = v1;
        fixed_u32::shift_left(a978, a979)
    };
    match m980 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: u32, v1: u32, v2: u32) -> u32 {
    let a981 = f70(v0, v1);
    let a982 = f91(v0, v2);
    fixed_u32::bit_or(a981, a982)
}

pub fn f93(v0: u32) -> u32 {
    let a985 = {
        let a983 = f92(v0, 17u32, 15u32);
        let a984 = f92(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a983, a984)
    };
    let a986 = f70(v0, 10u32);
    fixed_u32::bit_xor(a985, a986)
}

pub fn f94(v0: u32) -> u32 {
    let a989 = {
        let a987 = f92(v0, 7u32, 25u32);
        let a988 = f92(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a987, a988)
    };
    let a990 = f70(v0, 3u32);
    fixed_u32::bit_xor(a989, a990)
}
