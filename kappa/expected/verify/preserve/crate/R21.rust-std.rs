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
    C0(u64, u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, bool, bool, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(bool, Bytes, List<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool, List<u64>, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64, u64, u64, bool, bool),
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
pub enum Adt11 {
    C0(bool, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(List<Adt13>, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<Adt0>, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, bool, List<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F14(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn0::F14(k0) => f14(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, bool), p1: Bytes) -> R<(bool, bool)> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F20,
    F21,
    F52(u64),
    F53,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn2::F20 => f20(p0, p1),
            Fn2::F21 => Ok(f21(p0, p1)),
            Fn2::F52(k0) => f52(*k0, p0, p1),
            Fn2::F53 => Ok(f53(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F24,
    F137,
}

impl Fn3 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> R<Bytes> {
        match self {
            Fn3::F24 => f24(p0, p1),
            Fn3::F137 => f137(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F27,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn4::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F29,
    F105,
    F117(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> R<(bool, Bytes)> {
        match self {
            Fn5::F29 => f29(p0, p1),
            Fn5::F105 => f105(p0, p1),
            Fn5::F117(k0) => f117(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F32,
    F34(Bytes),
    F60(Bytes),
    F61,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn6::F32 => Ok(f32(p0)),
            Fn6::F34(k0) => f34(k0.clone(), p0),
            Fn6::F60(k0) => f60(k0.clone(), p0),
            Fn6::F61 => Ok(f61(p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F36(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn7::F36(k0, k1) => f36(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F38,
    F158,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn8::F38 => f38(p0, p1),
            Fn8::F158 => f158(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F40(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt0) -> List<Bytes> {
        match self {
            Fn9::F40(k0) => f40(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F45,
}

impl Fn10 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn10::F45 => f45(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F47,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Bytes>, List<(Bytes, Bytes)>), p1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
        match self {
            Fn11::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F49,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> List<Bytes> {
        match self {
            Fn12::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F64,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn13::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F67,
    F89,
    F107,
    F139,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn14::F67 => f67(p0, p1),
            Fn14::F89 => f89(p0, p1),
            Fn14::F107 => f107(p0, p1),
            Fn14::F139 => f139(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F72(Bytes, u64, u64),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<(u64, Adt4)> {
        match self {
            Fn15::F72(k0, k1, k2) => f72(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F78(Bytes, Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Adt5, p1: (Bytes, Bytes)) -> Adt5 {
        match self {
            Fn16::F78(k0, k1) => f78(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F80,
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, (bool, List<Adt6>)), p1: Bytes) -> R<(u64, (bool, List<Adt6>))> {
        match self {
            Fn17::F80 => f80(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F83(List<Adt6>),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt7) -> Adt7 {
        match self {
            Fn18::F83(k0) => f83(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F88(u64),
}

impl Fn19 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Adt6) -> R<(bool, List<Adt3>)> {
        match self {
            Fn19::F88(k0) => f88(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F95(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn20::F95(k0) => f95(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F102,
}

impl Fn21 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn21::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F109,
}

impl Fn22 {
    pub fn apply(&self, p0: Adt10) -> R<Adt10> {
        match self {
            Fn22::F109 => f109(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F114(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn23::F114(k0, k1) => f114(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F116(List<Adt6>, std::rc::Rc<Adt7>),
}

impl Fn24 {
    pub fn apply(&self, p0: Adt11, p1: Adt6) -> Adt11 {
        match self {
            Fn24::F116(k0, k1) => f116(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F122(u64),
}

impl Fn25 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Bytes) -> R<(bool, List<Adt3>)> {
        match self {
            Fn25::F122(k0) => f122(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F133,
}

impl Fn26 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn26::F133 => f133(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F136(u64),
}

impl Fn27 {
    pub fn apply(&self, p0: u64, p1: Adt0) -> R<u64> {
        match self {
            Fn27::F136(k0) => f136(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F147,
}

impl Fn28 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn28::F147 => f147(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F150(u64),
}

impl Fn29 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn29::F150(k0) => f150(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F152(List<u64>, List<Adt6>),
    F191(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn30::F152(k0, k1) => f152(k0.clone(), k1.clone(), p0, p1),
            Fn30::F191(k0) => f191(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F160(u64),
}

impl Fn31 {
    pub fn apply(&self, p0: List<Adt3>, p1: Bytes) -> R<List<Adt3>> {
        match self {
            Fn31::F160(k0) => f160(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F166(List<u64>, Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: bool, p1: Adt6) -> bool {
        match self {
            Fn32::F166(k0, k1) => f166(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F168(u64),
}

impl Fn33 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: Adt0) -> R<(bool, List<u64>)> {
        match self {
            Fn33::F168(k0) => f168(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F170,
    F184,
    F185,
}

impl Fn34 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn34::F170 => f170(p0, p1),
            Fn34::F184 => f184(p0, p1),
            Fn34::F185 => f185(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F175,
}

impl Fn35 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn35::F175 => f175(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F177(u64),
}

impl Fn36 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn36::F177(k0) => f177(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F187(u64),
    F190,
}

impl Fn37 {
    pub fn apply(&self, p0: Bytes, p1: Adt0) -> R<Bytes> {
        match self {
            Fn37::F187(k0) => f187(*k0, p0, p1),
            Fn37::F190 => f190(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F189(std::rc::Rc<Adt0>),
}

impl Fn38 {
    pub fn apply(&self, p0: Adt14, p1: Adt0) -> Adt14 {
        match self {
            Fn38::F189(k0) => f189((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F193(Bytes),
}

impl Fn39 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn39::F193(k0) => f193(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    let v1: (bool, List<Adt0>) = f1(v0.clone())?;
    let v2: List<Adt0> = {
        let (_, h1) = v1;
        h1
    };
    let v3: Adt0 = f2(v2.clone(), 1u64);
    let v4: (bool, Bytes) = f3(v2.clone())?;
    let v5: Bytes = f4(f2(v2.clone(), 7u64));
    let v6: (bool, Bytes) = f5(v5.clone())?;
    let v7: (bool, Bytes) = f6(v0, 14u64)?;
    let (h2, _) = v1;
    if if if if h2 {
        f7(v3.clone())
    } else {
        false
    } {
        let a4 = 0u64;
        let a5 = match v3.clone() {
            Adt0::C0(_, _, h3, _) => {
                h3
            }
        };
        if if nat_lt(a4, a5) {
            let a7 = match v3.clone() {
                Adt0::C0(_, _, h6, _) => {
                    h6
                }
            };
            let a8 = 13u64;
            nat_le(a7, a8)
        } else {
            false
        } {
            let (h9, _) = v4;
            h9
        } else {
            false
        }
    } else {
        false
    } {
        let a11 = 0u64;
        let a12 = {
            let a10 = v5;
            length_bytes(a10)
        };
        if if nat_lt(a11, a12) {
            let (h13, _) = v6;
            h13
        } else {
            false
        } {
            let (h14, _) = v7;
            h14
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a31 = f8(match v3 {
                Adt0::C0(_, _, h15, _) => {
                    h15
                }
            }, 8u64)?;
            let a32 = {
                let a29 = {
                    let (_, h16) = v4;
                    h16
                };
                let a30 = {
                    let a27 = {
                        let (_, h17) = v6;
                        h17
                    };
                    let a28 = {
                        let a25 = f9(f4(f2(v2.clone(), 2u64)))?;
                        let a26 = {
                            let a23 = f9(f4(f2(v2.clone(), 3u64)))?;
                            let a24 = {
                                let a21 = f9(f4(f2(v2.clone(), 4u64)))?;
                                let a22 = {
                                    let a19 = f8(f10(f2(v2, 5u64)), 8u64)?;
                                    let a20 = {
                                        let (_, h18) = v7;
                                        h18
                                    };
                                    append_bytes(a19, a20)
                                };
                                append_bytes(a21, a22)
                            };
                            append_bytes(a23, a24)
                        };
                        append_bytes(a25, a26)
                    };
                    append_bytes(a27, a28)
                };
                append_bytes(a29, a30)
            };
            append_bytes(a31, a32)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes) -> R<(bool, List<Adt0>)> {
    let v1: Adt1 = f11(v0, 0u64)?;
    Ok((match v1.clone() {
        Adt1::C0(_, h33, _, _) => {
            h33
        }
    }, f12(match v1 {
        Adt1::C0(_, _, _, h34) => {
            h34
        }
    })))
}

pub fn f2(v0: List<Adt0>, v1: u64) -> Adt0 {
    f13(Fn0::F14(v1), f15(), v0)
}

pub fn f3(v0: List<Adt0>) -> R<(bool, Bytes)> {
    let v1: List<Bytes> = f16(v0, 8u64);
    let v2: (bool, bool) = f17(Fn1::F18, (true, false), v1.clone())?;
    let v3: List<Bytes> = f19(Fn2::F20, List::<Bytes>::nil(), f19(Fn2::F21, List::<Bytes>::nil(), v1.clone())?)?;
    Ok(({
        let (h35, _) = v2;
        if h35 {
            let (_, h36) = v2;
            if h36 {
                true
            } else {
                let a37 = f22(v1.clone())?;
                let a38 = 0u64;
                nat_eq(a37, a38)
            }
        } else {
            false
        }
    }, f23(Fn3::F24, Bytes::lit(&[]), f25(v1, v3))?))
}

pub fn f4(v0: Adt0) -> Bytes {
    let a40 = match v0.clone() {
        Adt0::C0(_, h39, _, _) => {
            h39
        }
    };
    let a41 = 2u64;
    if nat_eq(a40, a41) {
        match v0 {
            Adt0::C0(_, _, _, h42) => {
                h42
            }
        }
    } else {
        Bytes::lit(&[])
    }
}

pub fn f5(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: Adt2 = {
        let (h43, _) = f26(Fn4::F27, {
            let a47 = {
                let a45 = {
                    let a44 = v0.clone();
                    length_bytes(a44)
                };
                let a46 = 8u64;
                nat_mul(a45, a46)?
            };
            let a48 = 16u64;
            nat_add(a47, a48)?
        }, Adt2::C0(true, Bytes::lit(&[]), List::cons(Adt3::C0(true, v0, 0u64), List::<Adt3>::nil())))?;
        h43
    };
    let v4: bool = {
        let m50 = match v1.clone() {
            Adt2::C0(_, _, h49) => {
                h49
            }
        };
        match m50.uncons() {
            None => {
                true
            }
            Some((_, _)) => {
                false
            }
        }
    };
    Ok((if match v1.clone() {
        Adt2::C0(h51, _, _) => {
            h51
        }
    } {
        v4
    } else {
        false
    }, match v1 {
        Adt2::C0(_, h52, _) => {
            h52
        }
    }))
}

pub fn f6(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f1(v0)?;
    let v3: List<Bytes> = f16({
        let (_, h53) = v2;
        h53
    }, v1);
    f28(Fn5::F29, ({
        let (h54, _) = v2;
        h54
    }, f8(f22(v3.clone())?, 4u64)?), f25(v3.clone(), f30(v3, 1u64)?))
}

pub fn f7(v0: Adt0) -> bool {
    let a56 = 0u64;
    let a57 = match v0.clone() {
        Adt0::C0(h55, _, _, _) => {
            h55
        }
    };
    if nat_lt(a56, a57) {
        let a59 = match v0 {
            Adt0::C0(_, h58, _, _) => {
                h58
            }
        };
        let a60 = 0u64;
        nat_eq(a59, a60)
    } else {
        false
    }
}

pub fn f8(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h61) = f31(Fn6::F32, v1, (v0, Bytes::lit(&[])))?;
    Ok(h61)
}

pub fn f9(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f33(v0)?;
    let (_, h62) = f31(Fn6::F34(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h62)
}

pub fn f10(v0: Adt0) -> u64 {
    let a64 = 0u64;
    let a65 = match v0.clone() {
        Adt0::C0(h63, _, _, _) => {
            h63
        }
    };
    if if nat_lt(a64, a65) {
        let a67 = match v0.clone() {
            Adt0::C0(_, h66, _, _) => {
                h66
            }
        };
        let a68 = 0u64;
        nat_eq(a67, a68)
    } else {
        false
    } {
        match v0 {
            Adt0::C0(_, _, h69, _) => {
                h69
            }
        }
    } else {
        0u64
    }
}

pub fn f11(v0: Bytes, v1: u64) -> R<Adt1> {
    let (h70, _) = f35(Fn7::F36(v1, v0.clone()), {
        let a72 = {
            let a71 = v0;
            length_bytes(a71)
        };
        let a73 = 2u64;
        nat_add(a72, a73)?
    }, Adt1::C0(0u64, true, false, List::<Adt0>::nil()))?;
    Ok(h70)
}

pub fn f12(v0: List<Adt0>) -> List<Adt0> {
    f37(Fn8::F38, List::<Adt0>::nil(), v0)
}

pub fn f13(v0: Fn0, p77: Adt0, p78: List<Adt0>) -> Adt0 {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v2;
                match m74.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c75: Fn0 = v0.clone();
                            c75.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64, v1: Adt0, v2: Adt0) -> Adt0 {
    let a80 = 0u64;
    let a81 = match v1.clone() {
        Adt0::C0(h79, _, _, _) => {
            h79
        }
    };
    if nat_lt(a80, a81) {
        v1
    } else {
        let a83 = match v2.clone() {
            Adt0::C0(h82, _, _, _) => {
                h82
            }
        };
        let a84 = v0;
        if nat_eq(a83, a84) {
            v2
        } else {
            v1
        }
    }
}

pub fn f15() -> Adt0 {
    Adt0::C0(0u64, 0u64, 0u64, Bytes::lit(&[]))
}

pub fn f16(v0: List<Adt0>, v1: u64) -> List<Bytes> {
    f39(Fn9::F40(v1), List::<Bytes>::nil(), f12(v0))
}

pub fn f17(v0: Fn1, p88: (bool, bool), p89: List<Bytes>) -> R<(bool, bool)> {
    let m87 = (p88, p89);
    {
        let mut m87 = m87;
        loop {
            let step = {
                let (v1, v2) = m87;
                let m85 = v2;
                match m85.uncons() {
                    None => {
                        Err::<((bool, bool), List<Bytes>), R<(bool, bool)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, bool), List<Bytes>), R<(bool, bool)>>(({
                            let c86: Fn1 = v0.clone();
                            c86.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m87 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: (bool, bool), v1: Bytes) -> R<(bool, bool)> {
    let v2: (bool, Adt0) = f41(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f41(v1, 2u64)?;
    Ok(({
        let (h90, _) = v0;
        if h90 {
            let (h91, _) = v2;
            if h91 {
                let (h92, _) = v3.clone();
                h92
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let (_, h93) = v0;
        if h93 {
            true
        } else {
            let a96 = {
                let a95 = f4({
                    let (_, h94) = v2;
                    h94
                });
                length_bytes(a95)
            };
            let a97 = 0u64;
            if nat_eq(a96, a97) {
                let a99 = 0u64;
                let a100 = f10({
                    let (_, h98) = v3.clone();
                    h98
                });
                if nat_lt(a99, a100) {
                    let a102 = f10({
                        let (_, h101) = v3;
                        h101
                    });
                    let a103 = 9223372036854775807u64;
                    nat_le(a102, a103)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }))
}

pub fn f19(v0: Fn2, p107: List<Bytes>, p108: List<Bytes>) -> R<List<Bytes>> {
    let m106 = (p107, p108);
    {
        let mut m106 = m106;
        loop {
            let step = {
                let (v1, v2) = m106;
                let m104 = v2;
                match m104.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c105: Fn2 = v0.clone();
                            c105.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons({
        let a111 = f42(f4({
            let (_, h109) = f41(v1.clone(), 1u64)?;
            h109
        }))?;
        let a112 = f43(f10({
            let (_, h110) = f41(v1, 2u64)?;
            h110
        }))?;
        append_bytes(a111, a112)
    }, v0))
}

pub fn f21(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f22(v0: List<Bytes>) -> R<u64> {
    f44(Fn10::F45, 0u64, v0)
}

pub fn f23(v0: Fn3, p116: Bytes, p117: List<Bytes>) -> R<Bytes> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v1, v2) = m115;
                let m113 = v2;
                match m113.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), R<Bytes>>(({
                            let c114: Fn3 = v0.clone();
                            c114.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a122 = v0;
    let a123 = {
        let a120 = f9(f4({
            let (_, h118) = f41(v1.clone(), 1u64)?;
            h118
        }))?;
        let a121 = f8(f10({
            let (_, h119) = f41(v1, 2u64)?;
            h119
        }), 8u64)?;
        append_bytes(a120, a121)
    };
    Ok(append_bytes(a122, a123))
}

pub fn f25(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let v2: List<(Bytes, Bytes)> = {
        let (_, h124) = f46(Fn11::F47, (v1, List::<(Bytes, Bytes)>::nil()), v0);
        h124
    };
    f48(Fn12::F49, List::<Bytes>::nil(), f50(v2))
}

pub fn f26(v0: Fn4, p129: u64, p130: Adt2) -> R<(Adt2, bool)> {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v1, v2) = m128;
                let m125 = v1;
                if m125 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m125 - 1;
                    let m127 = {
                        let c126: Fn4 = v0.clone();
                        c126.apply(v2.clone())?
                    };
                    match m127 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt2) -> R<Option<Adt2>> {
    f51(v0)
}

pub fn f28(v0: Fn5, p134: (bool, Bytes), p135: List<Bytes>) -> R<(bool, Bytes)> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(({
                            let c132: Fn5 = v0.clone();
                            c132.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f41(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f41(v1, 2u64)?;
    Ok(({
        let (h136, _) = v0;
        if h136 {
            let (h137, _) = v2;
            if h137 {
                let (h138, _) = v3;
                h138
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a144 = {
            let (_, h139) = v0;
            h139
        };
        let a145 = {
            let a142 = f9(f4({
                let (_, h140) = v2;
                h140
            }))?;
            let a143 = f9(f4({
                let (_, h141) = v3;
                h141
            }))?;
            append_bytes(a142, a143)
        };
        append_bytes(a144, a145)
    }))
}

pub fn f30(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    f19(Fn2::F52(v1), List::<Bytes>::nil(), f19(Fn2::F53, List::<Bytes>::nil(), v0)?)
}

pub fn f31(v0: Fn6, p149: u64, p150: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v1, v2) = m148;
                let m146 = v1;
                if m146 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m146 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c147: Fn6 = v0.clone();
                        c147.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a152 = {
            let (h151, _) = v0.clone();
            h151
        };
        let a153 = 256u64;
        let a154 = 0u64;
        nat_quot(a152, a153, a154)
    }, {
        let a160 = {
            let (_, h155) = v0;
            h155
        };
        let a161 = f54({
            let a157 = {
                let (h156, _) = v0;
                h156
            };
            let a158 = 256u64;
            let a159 = 0u64;
            nat_rem(a157, a158, a159)
        });
        append_bytes(a160, a161)
    })
}

pub fn f33(v0: Bytes) -> R<Bytes> {
    Ok(f55(f56(v0)?))
}

pub fn f34(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a163 = {
            let (h162, _) = v1.clone();
            h162
        };
        let a164 = 2u64;
        nat_mul(a163, a164)?
    };
    Ok(({
        let a166 = {
            let (h165, _) = v1;
            h165
        };
        let a167 = 1u64;
        nat_add(a166, a167)?
    }, {
        let a175 = {
            let (_, h168) = v1;
            h168
        };
        let a176 = f54({
            let a173 = {
                let a169 = f57(f58(v0.clone(), v2)?);
                let a170 = 16u64;
                nat_mul(a169, a170)?
            };
            let a174 = f57(f58(v0, {
                let a171 = v2;
                let a172 = 1u64;
                nat_add(a171, a172)?
            })?);
            nat_add(a173, a174)?
        });
        append_bytes(a175, a176)
    }))
}

pub fn f35(v0: Fn7, p181: u64, p182: Adt1) -> R<(Adt1, bool)> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v1, v2) = m180;
                let m177 = v1;
                if m177 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m177 - 1;
                    let m179 = {
                        let c178: Fn7 = v0.clone();
                        c178.apply(v2.clone())?
                    };
                    match m179 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: u64, v1: Bytes, v2: Adt1) -> R<Option<Adt1>> {
    f59(v1, v0, v2)
}

pub fn f37(v0: Fn8, p186: List<Adt0>, p187: List<Adt0>) -> List<Adt0> {
    let m185 = (p186, p187);
    {
        let mut m185 = m185;
        loop {
            let step = {
                let (v1, v2) = m185;
                let m183 = v2;
                match m183.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c184: Fn8 = v0.clone();
                            c184.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f39(v0: Fn9, p191: List<Bytes>, p192: List<Adt0>) -> List<Bytes> {
    let m190 = (p191, p192);
    {
        let mut m190 = m190;
        loop {
            let step = {
                let (v1, v2) = m190;
                let m188 = v2;
                match m188.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt0>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt0>), List<Bytes>>(({
                            let c189: Fn9 = v0.clone();
                            c189.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: u64, v1: List<Bytes>, v2: Adt0) -> List<Bytes> {
    let a194 = match v2.clone() {
        Adt0::C0(h193, _, _, _) => {
            h193
        }
    };
    let a195 = v0;
    if if nat_eq(a194, a195) {
        let a197 = match v2.clone() {
            Adt0::C0(_, h196, _, _) => {
                h196
            }
        };
        let a198 = 2u64;
        nat_eq(a197, a198)
    } else {
        false
    } {
        List::cons(match v2 {
            Adt0::C0(_, _, _, h199) => {
                h199
            }
        }, v1)
    } else {
        v1
    }
}

pub fn f41(v0: Bytes, v1: u64) -> R<(bool, Adt0)> {
    let v2: Adt1 = f11(v0, v1)?;
    if match v2.clone() {
        Adt1::C0(_, h200, _, _) => {
            h200
        }
    } {
        let m202 = match v2 {
            Adt1::C0(_, _, _, h201) => {
                h201
            }
        };
        match m202.uncons() {
            None => {
                Ok((true, f15()))
            }
            Some((v3, _)) => {
                Ok((true, {
                    let a204 = match v3.clone() {
                        Adt0::C0(h203, _, _, _) => {
                            h203
                        }
                    };
                    let a205 = v1;
                    if nat_eq(a204, a205) {
                        v3
                    } else {
                        f15()
                    }
                }))
            }
        }
    } else {
        Ok((false, f15()))
    }
}

pub fn f42(v0: Bytes) -> R<Bytes> {
    let a208 = {
        let (_, h206) = f31(Fn6::F60(v0.clone()), {
            let a207 = v0;
            length_bytes(a207)
        }, (0u64, Bytes::lit(&[])))?;
        h206
    };
    let a209 = Bytes::lit(&[0x00]);
    Ok(append_bytes(a208, a209))
}

pub fn f43(v0: u64) -> R<Bytes> {
    let (_, h210) = f31(Fn6::F61, 8u64, (v0, Bytes::lit(&[])))?;
    Ok(h210)
}

pub fn f44(v0: Fn10, p214: u64, p215: List<Bytes>) -> R<u64> {
    let m213 = (p214, p215);
    {
        let mut m213 = m213;
        loop {
            let step = {
                let (v1, v2) = m213;
                let m211 = v2;
                match m211.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c212: Fn10 = v0.clone();
                            c212.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, _: Bytes) -> R<u64> {
    let a216 = v0;
    let a217 = 1u64;
    nat_add(a216, a217)
}

pub fn f46(v0: Fn11, p221: (List<Bytes>, List<(Bytes, Bytes)>), p222: List<Bytes>) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v1, v2) = m220;
                let m218 = v2;
                match m218.uncons() {
                    None => {
                        Err::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(({
                            let c219: Fn11 = v0.clone();
                            c219.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (List<Bytes>, List<(Bytes, Bytes)>), v1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m224 = {
        let (h223, _) = v0;
        h223
    };
    match m224.uncons() {
        None => {
            (List::<Bytes>::nil(), f62(Bytes::lit(&[]), v1, {
                let (_, h226) = v0;
                h226
            }))
        }
        Some((v2, v3)) => {
            (v3, f62(v2, v1, {
                let (_, h225) = v0;
                h225
            }))
        }
    }
}

pub fn f48(v0: Fn12, p230: List<Bytes>, p231: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m229 = (p230, p231);
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v1, v2) = m229;
                let m227 = v2;
                match m227.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(({
                            let c228: Fn12 = v0.clone();
                            c228.apply(v1, v3)
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

pub fn f49(v0: List<Bytes>, v1: (Bytes, Bytes)) -> List<Bytes> {
    List::cons({
        let (_, h232) = v1;
        h232
    }, v0)
}

pub fn f50(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f63(Fn13::F64, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f51(v0: Adt2) -> R<Option<Adt2>> {
    if match v0.clone() {
        Adt2::C0(h233, _, _) => {
            h233
        }
    } {
        let m235 = match v0.clone() {
            Adt2::C0(_, _, h234) => {
                h234
            }
        };
        match m235.uncons() {
            None => {
                Ok(None::<Adt2>)
            }
            Some((v1, v2)) => {
                if match v1.clone() {
                    Adt3::C0(h236, _, _) => {
                        h236
                    }
                } {
                    let v3: (bool, List<Adt3>) = f65(match v1.clone() {
                        Adt3::C0(_, h237, _) => {
                            h237
                        }
                    }, match v1 {
                        Adt3::C0(_, _, h238) => {
                            h238
                        }
                    })?;
                    let (h239, _) = v3;
                    if h239 {
                        Ok(Some(Adt2::C0(true, match v0 {
                            Adt2::C0(_, h240, _) => {
                                h240
                            }
                        }, f66(Fn14::F67, v2, f68({
                            let (_, h241) = v3;
                            h241
                        })))))
                    } else {
                        Ok(Some(Adt2::C0(false, Bytes::lit(&[]), List::<Adt3>::nil())))
                    }
                } else {
                    Ok(Some(Adt2::C0(true, {
                        let a244 = match v0 {
                            Adt2::C0(_, h242, _) => {
                                h242
                            }
                        };
                        let a245 = match v1 {
                            Adt3::C0(_, h243, _) => {
                                h243
                            }
                        };
                        append_bytes(a244, a245)
                    }, v2)))
                }
            }
        }
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f52(v0: u64, v1: List<Bytes>, v2: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f4({
        let (_, h246) = f41(v2, v0)?;
        h246
    }), v1))
}

pub fn f53(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f54(v0: u64) -> Bytes {
    f69(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f55(v0: Adt4) -> Bytes {
    let a267 = f70(match v0.clone() {
        Adt4::C0(h247, _, _, _, _, _, _, _) => {
            h247
        }
    });
    let a268 = {
        let a265 = f70(match v0.clone() {
            Adt4::C0(_, h248, _, _, _, _, _, _) => {
                h248
            }
        });
        let a266 = {
            let a263 = f70(match v0.clone() {
                Adt4::C0(_, _, h249, _, _, _, _, _) => {
                    h249
                }
            });
            let a264 = {
                let a261 = f70(match v0.clone() {
                    Adt4::C0(_, _, _, h250, _, _, _, _) => {
                        h250
                    }
                });
                let a262 = {
                    let a259 = f70(match v0.clone() {
                        Adt4::C0(_, _, _, _, h251, _, _, _) => {
                            h251
                        }
                    });
                    let a260 = {
                        let a257 = f70(match v0.clone() {
                            Adt4::C0(_, _, _, _, _, h252, _, _) => {
                                h252
                            }
                        });
                        let a258 = {
                            let a255 = f70(match v0.clone() {
                                Adt4::C0(_, _, _, _, _, _, h253, _) => {
                                    h253
                                }
                            });
                            let a256 = f70(match v0 {
                                Adt4::C0(_, _, _, _, _, _, _, h254) => {
                                    h254
                                }
                            });
                            append_bytes(a255, a256)
                        };
                        append_bytes(a257, a258)
                    };
                    append_bytes(a259, a260)
                };
                append_bytes(a261, a262)
            };
            append_bytes(a263, a264)
        };
        append_bytes(a265, a266)
    };
    append_bytes(a267, a268)
}

pub fn f56(v0: Bytes) -> R<Adt4> {
    let v1: u64 = {
        let a269 = v0.clone();
        length_bytes(a269)
    };
    let v2: u64 = {
        let a275 = {
            let a272 = {
                let a270 = v1;
                let a271 = 72u64;
                nat_add(a270, a271)?
            };
            let a273 = 64u64;
            let a274 = 0u64;
            nat_quot(a272, a273, a274)
        };
        let a276 = 64u64;
        nat_mul(a275, a276)?
    };
    let (_, h277) = f71(Fn15::F72(v0, v1, v2), {
        let a278 = v2;
        let a279 = 64u64;
        let a280 = 0u64;
        nat_quot(a278, a279, a280)
    }, (0u64, f73()))?;
    Ok(h277)
}

pub fn f57(v0: u64) -> u64 {
    let a281 = 47u64;
    let a282 = v0;
    if nat_lt(a281, a282) {
        let a283 = v0;
        let a284 = 58u64;
        if nat_lt(a283, a284) {
            let a285 = v0;
            let a286 = 48u64;
            nat_sub(a285, a286)
        } else {
            let a287 = 64u64;
            let a288 = v0;
            if nat_lt(a287, a288) {
                let a289 = v0;
                let a290 = 71u64;
                if nat_lt(a289, a290) {
                    let a291 = v0;
                    let a292 = 55u64;
                    nat_sub(a291, a292)
                } else {
                    let a293 = 96u64;
                    let a294 = v0;
                    if nat_lt(a293, a294) {
                        let a295 = v0;
                        let a296 = 103u64;
                        if nat_lt(a295, a296) {
                            let a297 = v0;
                            let a298 = 87u64;
                            nat_sub(a297, a298)
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

pub fn f58(v0: Bytes, v1: u64) -> R<u64> {
    let m301 = {
        let a299 = v0;
        let a300 = v1;
        index_bytes(a299, a300)
    };
    match m301 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a346 = {
                let a322 = {
                    let a310 = {
                        let a304 = {
                            let a302 = v3;
                            let a303 = 128u8;
                            fixed_u8::bit_and(a302, a303)
                        };
                        let a305 = 128u8;
                        if equal(a304, a305) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a311 = {
                        let a308 = {
                            let a306 = v3;
                            let a307 = 64u8;
                            fixed_u8::bit_and(a306, a307)
                        };
                        let a309 = 64u8;
                        if equal(a308, a309) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a310, a311)?
                };
                let a323 = {
                    let a320 = {
                        let a314 = {
                            let a312 = v3;
                            let a313 = 32u8;
                            fixed_u8::bit_and(a312, a313)
                        };
                        let a315 = 32u8;
                        if equal(a314, a315) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a321 = {
                        let a318 = {
                            let a316 = v3;
                            let a317 = 16u8;
                            fixed_u8::bit_and(a316, a317)
                        };
                        let a319 = 16u8;
                        if equal(a318, a319) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a320, a321)?
                };
                nat_add(a322, a323)?
            };
            let a347 = {
                let a344 = {
                    let a332 = {
                        let a326 = {
                            let a324 = v3;
                            let a325 = 8u8;
                            fixed_u8::bit_and(a324, a325)
                        };
                        let a327 = 8u8;
                        if equal(a326, a327) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a333 = {
                        let a330 = {
                            let a328 = v3;
                            let a329 = 4u8;
                            fixed_u8::bit_and(a328, a329)
                        };
                        let a331 = 4u8;
                        if equal(a330, a331) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a332, a333)?
                };
                let a345 = {
                    let a342 = {
                        let a336 = {
                            let a334 = v3;
                            let a335 = 2u8;
                            fixed_u8::bit_and(a334, a335)
                        };
                        let a337 = 2u8;
                        if equal(a336, a337) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a343 = {
                        let a340 = {
                            let a338 = v3;
                            let a339 = 1u8;
                            fixed_u8::bit_and(a338, a339)
                        };
                        let a341 = 1u8;
                        if equal(a340, a341) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a342, a343)?
                };
                nat_add(a344, a345)?
            };
            nat_add(a346, a347)
        }
    }
}

pub fn f59(v0: Bytes, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    if if match v2.clone() {
        Adt1::C0(_, _, h348, _) => {
            h348
        }
    } {
        true
    } else {
        let a351 = {
            let a349 = v0.clone();
            length_bytes(a349)
        };
        let a352 = match v2.clone() {
            Adt1::C0(h350, _, _, _) => {
                h350
            }
        };
        nat_le(a351, a352)
    } {
        Ok(None::<Adt1>)
    } else {
        let v3: (bool, (u64, u64)) = f74(v0.clone(), match v2.clone() {
            Adt1::C0(h353, _, _, _) => {
                h353
            }
        })?;
        let v4: u64 = {
            let (h354, _) = {
                let (_, h355) = v3;
                h355
            };
            h354
        };
        let v5: u64 = {
            let a356 = v4;
            let a357 = 8u64;
            let a358 = 0u64;
            nat_quot(a356, a357, a358)
        };
        let (h359, _) = v3;
        if if h359 {
            let a360 = 0u64;
            let a361 = v5;
            nat_lt(a360, a361)
        } else {
            false
        } {
            f75(v0, v1, v2, v5, {
                let a362 = v4;
                let a363 = 8u64;
                let a364 = 0u64;
                nat_rem(a362, a363, a364)
            }, {
                let (_, h365) = {
                    let (_, h366) = v3;
                    h366
                };
                h365
            })
        } else {
            Ok(f76(v2))
        }
    }
}

pub fn f60(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a368 = {
            let (h367, _) = v1.clone();
            h367
        };
        let a369 = 1u64;
        nat_add(a368, a369)?
    }, {
        let a374 = {
            let (_, h370) = v1;
            h370
        };
        let a375 = {
            let a372 = Bytes::lit(&[0x01]);
            let a373 = f54(f58(v0, {
                let (h371, _) = v1;
                h371
            })?);
            append_bytes(a372, a373)
        };
        append_bytes(a374, a375)
    }))
}

pub fn f61(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a377 = {
            let (h376, _) = v0.clone();
            h376
        };
        let a378 = 256u64;
        let a379 = 0u64;
        nat_quot(a377, a378, a379)
    }, {
        let a385 = f54({
            let a381 = {
                let (h380, _) = v0;
                h380
            };
            let a382 = 256u64;
            let a383 = 0u64;
            nat_rem(a381, a382, a383)
        });
        let a386 = {
            let (_, h384) = v0;
            h384
        };
        append_bytes(a385, a386)
    })
}

pub fn f62(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt5 = f77(Fn16::F78(v1.clone(), v0.clone()), Adt5::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt5::C0(h387, _) => {
            h387
        }
    } {
        f50(match v3 {
            Adt5::C0(_, h388) => {
                h388
            }
        })
    } else {
        f50(List::cons((v0, v1), match v3 {
            Adt5::C0(_, h389) => {
                h389
            }
        }))
    }
}

pub fn f63(v0: Fn13, p393: List<(Bytes, Bytes)>, p394: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v1, v2) = m392;
                let m390 = v2;
                match m390.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c391: Fn13 = v0.clone();
                            c391.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f65(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, List<Adt0>) = f1(v0)?;
    let v3: List<Adt0> = {
        let (_, h395) = v2;
        h395
    };
    let v4: List<Bytes> = f16(v3.clone(), 1u64);
    let v5: (u64, (bool, List<Adt6>)) = f79(Fn17::F80, (0u64, (true, List::<Adt6>::nil())), v4)?;
    let v6: List<Adt6> = f81({
        let (_, h396) = {
            let (_, h397) = v5.clone();
            h397
        };
        h396
    });
    let v7: Adt7 = f82(Fn18::F83(v6), {
        let (h398, _) = v5.clone();
        h398
    }, Adt7::C0(true, List::<u64>::nil(), List::<Adt6>::nil()));
    let v8: (bool, Bytes) = f84(v3.clone())?;
    let v9: (bool, Bytes) = f85(v3.clone(), 11u64)?;
    let v10: (bool, Bytes) = f85(v3.clone(), 12u64)?;
    let v11: (bool, Bytes) = f85(v3.clone(), 13u64)?;
    let v12: Adt3 = f86({
        let a407 = {
            let (_, h399) = v8;
            h399
        };
        let a408 = {
            let a405 = {
                let (_, h400) = v9;
                h400
            };
            let a406 = {
                let a403 = {
                    let (_, h401) = v10;
                    h401
                };
                let a404 = {
                    let (_, h402) = v11;
                    h402
                };
                append_bytes(a403, a404)
            };
            append_bytes(a405, a406)
        };
        append_bytes(a407, a408)
    });
    let v13: (bool, List<Adt3>) = f87(Fn19::F88(v1), (true, List::cons(f86({
        let a410 = f9(f4(f2(v3, 2u64)))?;
        let a411 = f8({
            let (h409, _) = v5;
            h409
        }, 4u64)?;
        append_bytes(a410, a411)
    }), List::<Adt3>::nil())), f81(match v7.clone() {
        Adt7::C0(_, _, h412) => {
            h412
        }
    }))?;
    Ok(({
        let a413 = v1;
        let a414 = 64u64;
        if if nat_le(a413, a414) {
            let (h415, _) = v2;
            h415
        } else {
            false
        } {
            let (h416, _) = {
                let (_, h417) = v5;
                h417
            };
            if if h416 {
                match v7 {
                    Adt7::C0(h418, _, _) => {
                        h418
                    }
                }
            } else {
                false
            } {
                let (h419, _) = v13;
                if h419 {
                    let (h420, _) = v8;
                    if h420 {
                        let (h421, _) = v9;
                        if h421 {
                            let (h422, _) = v10;
                            if h422 {
                                let (h423, _) = v11;
                                h423
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }, f68(List::cons(v12, {
        let (_, h424) = v13;
        h424
    }))))
}

pub fn f66(v0: Fn14, p428: List<Adt3>, p429: List<Adt3>) -> List<Adt3> {
    let m427 = (p428, p429);
    {
        let mut m427 = m427;
        loop {
            let step = {
                let (v1, v2) = m427;
                let m425 = v2;
                match m425.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c426: Fn14 = v0.clone();
                            c426.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f68(v0: List<Adt3>) -> List<Adt3> {
    f66(Fn14::F89, List::<Adt3>::nil(), v0)
}

pub fn f69(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m433 = {
        let a430 = v0;
        let a431 = v1;
        let a432 = v2;
        slice_bytes(a430, a431, a432)
    };
    match m433 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f70(v0: u32) -> Bytes {
    let a462 = f90({
        let a434 = f91(v0, 28u32);
        let a435 = 15u32;
        fixed_u32::bit_and(a434, a435)
    });
    let a463 = {
        let a460 = f90({
            let a436 = f91(v0, 24u32);
            let a437 = 15u32;
            fixed_u32::bit_and(a436, a437)
        });
        let a461 = {
            let a458 = f90({
                let a438 = f91(v0, 20u32);
                let a439 = 15u32;
                fixed_u32::bit_and(a438, a439)
            });
            let a459 = {
                let a456 = f90({
                    let a440 = f91(v0, 16u32);
                    let a441 = 15u32;
                    fixed_u32::bit_and(a440, a441)
                });
                let a457 = {
                    let a454 = f90({
                        let a442 = f91(v0, 12u32);
                        let a443 = 15u32;
                        fixed_u32::bit_and(a442, a443)
                    });
                    let a455 = {
                        let a452 = f90({
                            let a444 = f91(v0, 8u32);
                            let a445 = 15u32;
                            fixed_u32::bit_and(a444, a445)
                        });
                        let a453 = {
                            let a450 = f90({
                                let a446 = f91(v0, 4u32);
                                let a447 = 15u32;
                                fixed_u32::bit_and(a446, a447)
                            });
                            let a451 = f90({
                                let a448 = v0;
                                let a449 = 15u32;
                                fixed_u32::bit_and(a448, a449)
                            });
                            append_bytes(a450, a451)
                        };
                        append_bytes(a452, a453)
                    };
                    append_bytes(a454, a455)
                };
                append_bytes(a456, a457)
            };
            append_bytes(a458, a459)
        };
        append_bytes(a460, a461)
    };
    append_bytes(a462, a463)
}

pub fn f71(v0: Fn15, p467: u64, p468: (u64, Adt4)) -> R<(u64, Adt4)> {
    let m466 = (p467, p468);
    {
        let mut m466 = m466;
        loop {
            let step = {
                let (v1, v2) = m466;
                let m464 = v1;
                if m464 == 0 {
                    Err::<(u64, (u64, Adt4)), R<(u64, Adt4)>>(Ok(v2))
                } else {
                    let v3 = m464 - 1;
                    Ok::<(u64, (u64, Adt4)), R<(u64, Adt4)>>((v3, {
                        let c465: Fn15 = v0.clone();
                        c465.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt4)) -> R<(u64, Adt4)> {
    Ok(({
        let a470 = {
            let (h469, _) = v3.clone();
            h469
        };
        let a471 = 64u64;
        nat_add(a470, a471)?
    }, f92({
        let (_, h472) = v3;
        h472
    }, f93(v0, v1, v2, {
        let (h473, _) = v3;
        h473
    })?)?))
}

pub fn f73() -> Adt4 {
    Adt4::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f74(v0: Bytes, v1: u64) -> R<(bool, (u64, u64))> {
    let v2: Adt8 = {
        let (h474, _) = f94(Fn20::F95(v0), 12u64, Adt8::C0(v1, 0u64, 0u64, 1u64, true, false))?;
        h474
    };
    Ok((if match v2.clone() {
        Adt8::C0(_, _, _, _, h475, _) => {
            h475
        }
    } {
        match v2.clone() {
            Adt8::C0(_, _, _, _, _, h476) => {
                h476
            }
        }
    } else {
        false
    }, (match v2.clone() {
        Adt8::C0(_, h477, _, _, _, _) => {
            h477
        }
    }, match v2 {
        Adt8::C0(h478, _, _, _, _, _) => {
            h478
        }
    })))
}

pub fn f75(v0: Bytes, v1: u64, v2: Adt1, v3: u64, v4: u64, v5: u64) -> R<Option<Adt1>> {
    let a479 = v4;
    let a480 = 0u64;
    if nat_eq(a479, a480) {
        let v6: (bool, (u64, u64)) = f74(v0, v5)?;
        let (h481, _) = v6;
        if h481 {
            Ok(f96(v2, v1, v3, 0u64, {
                let (h482, _) = {
                    let (_, h483) = v6;
                    h483
                };
                h482
            }, Bytes::lit(&[]), {
                let (_, h484) = {
                    let (_, h485) = v6;
                    h485
                };
                h484
            }))
        } else {
            Ok(f76(v2))
        }
    } else {
        let a486 = v4;
        let a487 = 1u64;
        if nat_eq(a486, a487) {
            if f97(v0.clone(), v5, 8u64) {
                Ok(f96(v2, v1, v3, 1u64, f98(v0, v5, 8u64)?, Bytes::lit(&[]), {
                    let a488 = v5;
                    let a489 = 8u64;
                    nat_add(a488, a489)?
                }))
            } else {
                Ok(f76(v2))
            }
        } else {
            let a490 = v4;
            let a491 = 5u64;
            if nat_eq(a490, a491) {
                if f97(v0.clone(), v5, 4u64) {
                    Ok(f96(v2, v1, v3, 5u64, f98(v0, v5, 4u64)?, Bytes::lit(&[]), {
                        let a492 = v5;
                        let a493 = 4u64;
                        nat_add(a492, a493)?
                    }))
                } else {
                    Ok(f76(v2))
                }
            } else {
                let a494 = v4;
                let a495 = 2u64;
                if nat_eq(a494, a495) {
                    let v7: (bool, (u64, u64)) = f74(v0.clone(), v5)?;
                    let v8: u64 = {
                        let (h496, _) = {
                            let (_, h497) = v7;
                            h497
                        };
                        h496
                    };
                    let v9: u64 = {
                        let (_, h498) = {
                            let (_, h499) = v7;
                            h499
                        };
                        h498
                    };
                    let (h500, _) = v7;
                    if if h500 {
                        f97(v0.clone(), v9, v8)
                    } else {
                        false
                    } {
                        Ok(f96(v2, v1, v3, 2u64, 0u64, f69(v0, v9, v8), {
                            let a501 = v9;
                            let a502 = v8;
                            nat_add(a501, a502)?
                        }))
                    } else {
                        Ok(f76(v2))
                    }
                } else {
                    Ok(f76(v2))
                }
            }
        }
    }
}

pub fn f76(v0: Adt1) -> Option<Adt1> {
    Some(Adt1::C0(match v0 {
        Adt1::C0(h503, _, _, _) => {
            h503
        }
    }, false, true, List::<Adt0>::nil()))
}

pub fn f77(v0: Fn16, p507: Adt5, p508: List<(Bytes, Bytes)>) -> Adt5 {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m504 = v2;
                match m504.uncons() {
                    None => {
                        Err::<(Adt5, List<(Bytes, Bytes)>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<(Bytes, Bytes)>), Adt5>(({
                            let c505: Fn16 = v0.clone();
                            c505.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Bytes, v1: Bytes, v2: Adt5, v3: (Bytes, Bytes)) -> Adt5 {
    if match v2.clone() {
        Adt5::C0(h509, _) => {
            h509
        }
    } {
        Adt5::C0(true, List::cons(v3, match v2 {
            Adt5::C0(_, h510) => {
                h510
            }
        }))
    } else {
        if f99(v1.clone(), {
            let (h511, _) = v3.clone();
            h511
        }) {
            Adt5::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt5::C0(_, h512) => {
                    h512
                }
            })))
        } else {
            Adt5::C0(false, List::cons(v3, match v2 {
                Adt5::C0(_, h513) => {
                    h513
                }
            }))
        }
    }
}

pub fn f79(v0: Fn17, p517: (u64, (bool, List<Adt6>)), p518: List<Bytes>) -> R<(u64, (bool, List<Adt6>))> {
    let m516 = (p517, p518);
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, v2) = m516;
                let m514 = v2;
                match m514.uncons() {
                    None => {
                        Err::<((u64, (bool, List<Adt6>)), List<Bytes>), R<(u64, (bool, List<Adt6>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (bool, List<Adt6>)), List<Bytes>), R<(u64, (bool, List<Adt6>))>>(({
                            let c515: Fn17 = v0.clone();
                            c515.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: (u64, (bool, List<Adt6>)), v1: Bytes) -> R<(u64, (bool, List<Adt6>))> {
    let v2: (bool, Adt6) = f100({
        let (h519, _) = v0.clone();
        h519
    }, v1)?;
    Ok(({
        let a521 = {
            let (h520, _) = v0.clone();
            h520
        };
        let a522 = 1u64;
        nat_add(a521, a522)?
    }, ({
        let (h523, _) = {
            let (_, h524) = v0.clone();
            h524
        };
        if h523 {
            let (h525, _) = v2;
            h525
        } else {
            false
        }
    }, List::cons({
        let (_, h526) = v2;
        h526
    }, {
        let (_, h527) = {
            let (_, h528) = v0;
            h528
        };
        h527
    }))))
}

pub fn f81(v0: List<Adt6>) -> List<Adt6> {
    f101(Fn21::F102, List::<Adt6>::nil(), v0)
}

pub fn f82(v0: Fn18, p532: u64, p533: Adt7) -> Adt7 {
    let m531 = (p532, p533);
    {
        let mut m531 = m531;
        loop {
            let step = {
                let (v1, v2) = m531;
                let m529 = v1;
                if m529 == 0 {
                    Err::<(u64, Adt7), Adt7>(v2)
                } else {
                    let v3 = m529 - 1;
                    Ok::<(u64, Adt7), Adt7>((v3, {
                        let c530: Fn18 = v0.clone();
                        c530.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<Adt6>, v1: Adt7) -> Adt7 {
    f103(v0, v1)
}

pub fn f84(v0: List<Adt0>) -> R<(bool, Bytes)> {
    let v1: List<Bytes> = f16(v0, 5u64);
    let v2: (bool, Bytes) = f104(f25(v1.clone(), f30(v1.clone(), 8u64)?), 0u64)?;
    Ok(({
        let (h534, _) = v2;
        h534
    }, {
        let a536 = f8(f22(v1)?, 4u64)?;
        let a537 = {
            let (_, h535) = v2;
            h535
        };
        append_bytes(a536, a537)
    }))
}

pub fn f85(v0: List<Adt0>, v1: u64) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f16(v0, v1);
    f28(Fn5::F105, (true, f8(f22(v2.clone())?, 4u64)?), f25(v2.clone(), f30(v2, 1u64)?))
}

pub fn f86(v0: Bytes) -> Adt3 {
    Adt3::C0(false, v0, 0u64)
}

pub fn f87(v0: Fn19, p541: (bool, List<Adt3>), p542: List<Adt6>) -> R<(bool, List<Adt3>)> {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v1, v2) = m540;
                let m538 = v2;
                match m538.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Adt6>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Adt6>), R<(bool, List<Adt3>)>>(({
                            let c539: Fn19 = v0.clone();
                            c539.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: u64, v1: (bool, List<Adt3>), v2: Adt6) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f106(v2, v0)?;
    Ok(({
        let (h543, _) = v1;
        if h543 {
            let (h544, _) = v3;
            h544
        } else {
            false
        }
    }, f66(Fn14::F107, {
        let (_, h545) = v1;
        h545
    }, f68({
        let (_, h546) = v3;
        h546
    }))))
}

pub fn f89(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f90(v0: u32) -> Bytes {
    let a549 = {
        let a547 = v0;
        let a548 = 8u32;
        fixed_u32::bit_and(a547, a548)
    };
    let a550 = 0u32;
    if equal(a549, a550) {
        let a553 = {
            let a551 = v0;
            let a552 = 4u32;
            fixed_u32::bit_and(a551, a552)
        };
        let a554 = 0u32;
        if equal(a553, a554) {
            let a557 = {
                let a555 = v0;
                let a556 = 2u32;
                fixed_u32::bit_and(a555, a556)
            };
            let a558 = 0u32;
            if equal(a557, a558) {
                let a561 = {
                    let a559 = v0;
                    let a560 = 1u32;
                    fixed_u32::bit_and(a559, a560)
                };
                let a562 = 0u32;
                if equal(a561, a562) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a565 = {
                    let a563 = v0;
                    let a564 = 1u32;
                    fixed_u32::bit_and(a563, a564)
                };
                let a566 = 0u32;
                if equal(a565, a566) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a569 = {
                let a567 = v0;
                let a568 = 2u32;
                fixed_u32::bit_and(a567, a568)
            };
            let a570 = 0u32;
            if equal(a569, a570) {
                let a573 = {
                    let a571 = v0;
                    let a572 = 1u32;
                    fixed_u32::bit_and(a571, a572)
                };
                let a574 = 0u32;
                if equal(a573, a574) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a577 = {
                    let a575 = v0;
                    let a576 = 1u32;
                    fixed_u32::bit_and(a575, a576)
                };
                let a578 = 0u32;
                if equal(a577, a578) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a581 = {
            let a579 = v0;
            let a580 = 4u32;
            fixed_u32::bit_and(a579, a580)
        };
        let a582 = 0u32;
        if equal(a581, a582) {
            let a585 = {
                let a583 = v0;
                let a584 = 2u32;
                fixed_u32::bit_and(a583, a584)
            };
            let a586 = 0u32;
            if equal(a585, a586) {
                let a589 = {
                    let a587 = v0;
                    let a588 = 1u32;
                    fixed_u32::bit_and(a587, a588)
                };
                let a590 = 0u32;
                if equal(a589, a590) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a593 = {
                    let a591 = v0;
                    let a592 = 1u32;
                    fixed_u32::bit_and(a591, a592)
                };
                let a594 = 0u32;
                if equal(a593, a594) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a597 = {
                let a595 = v0;
                let a596 = 2u32;
                fixed_u32::bit_and(a595, a596)
            };
            let a598 = 0u32;
            if equal(a597, a598) {
                let a601 = {
                    let a599 = v0;
                    let a600 = 1u32;
                    fixed_u32::bit_and(a599, a600)
                };
                let a602 = 0u32;
                if equal(a601, a602) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a605 = {
                    let a603 = v0;
                    let a604 = 1u32;
                    fixed_u32::bit_and(a603, a604)
                };
                let a606 = 0u32;
                if equal(a605, a606) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f91(v0: u32, v1: u32) -> u32 {
    let m609 = {
        let a607 = v0;
        let a608 = v1;
        fixed_u32::shift_right(a607, a608)
    };
    match m609 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: Adt4, v1: Adt9) -> R<Adt4> {
    let v2: Adt10 = f108(Fn22::F109, 64u64, Adt10::C0(match v0.clone() {
        Adt4::C0(h610, _, _, _, _, _, _, _) => {
            h610
        }
    }, match v0.clone() {
        Adt4::C0(_, h611, _, _, _, _, _, _) => {
            h611
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h612, _, _, _, _, _) => {
            h612
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h613, _, _, _, _) => {
            h613
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h614, _, _, _) => {
            h614
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h615, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h616, _) => {
            h616
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h617) => {
            h617
        }
    }, std::rc::Rc::new(v1), 0u64))?;
    Ok(Adt4::C0(f110(match v0.clone() {
        Adt4::C0(h618, _, _, _, _, _, _, _) => {
            h618
        }
    }, match v2.clone() {
        Adt10::C0(h619, _, _, _, _, _, _, _, _, _) => {
            h619
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, h620, _, _, _, _, _, _) => {
            h620
        }
    }, match v2.clone() {
        Adt10::C0(_, h621, _, _, _, _, _, _, _, _) => {
            h621
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, _, h622, _, _, _, _, _) => {
            h622
        }
    }, match v2.clone() {
        Adt10::C0(_, _, h623, _, _, _, _, _, _, _) => {
            h623
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, _, _, h624, _, _, _, _) => {
            h624
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, h625, _, _, _, _, _, _) => {
            h625
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, _, _, _, h626, _, _, _) => {
            h626
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, h627, _, _, _, _, _) => {
            h627
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, _, _, _, _, h628, _, _) => {
            h628
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, h629, _, _, _, _) => {
            h629
        }
    }), f110(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h630, _) => {
            h630
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, _, h631, _, _, _) => {
            h631
        }
    }), f110(match v0 {
        Adt4::C0(_, _, _, _, _, _, _, h632) => {
            h632
        }
    }, match v2 {
        Adt10::C0(_, _, _, _, _, _, _, h633, _, _) => {
            h633
        }
    })))
}

pub fn f93(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    Ok(Adt9::C0(f111(v0.clone(), v1, v2, v3)?, f111(v0.clone(), v1, v2, {
        let a634 = v3;
        let a635 = 4u64;
        nat_add(a634, a635)?
    })?, f111(v0.clone(), v1, v2, {
        let a636 = v3;
        let a637 = 8u64;
        nat_add(a636, a637)?
    })?, f111(v0.clone(), v1, v2, {
        let a638 = v3;
        let a639 = 12u64;
        nat_add(a638, a639)?
    })?, f111(v0.clone(), v1, v2, {
        let a640 = v3;
        let a641 = 16u64;
        nat_add(a640, a641)?
    })?, f111(v0.clone(), v1, v2, {
        let a642 = v3;
        let a643 = 20u64;
        nat_add(a642, a643)?
    })?, f111(v0.clone(), v1, v2, {
        let a644 = v3;
        let a645 = 24u64;
        nat_add(a644, a645)?
    })?, f111(v0.clone(), v1, v2, {
        let a646 = v3;
        let a647 = 28u64;
        nat_add(a646, a647)?
    })?, f111(v0.clone(), v1, v2, {
        let a648 = v3;
        let a649 = 32u64;
        nat_add(a648, a649)?
    })?, f111(v0.clone(), v1, v2, {
        let a650 = v3;
        let a651 = 36u64;
        nat_add(a650, a651)?
    })?, f111(v0.clone(), v1, v2, {
        let a652 = v3;
        let a653 = 40u64;
        nat_add(a652, a653)?
    })?, f111(v0.clone(), v1, v2, {
        let a654 = v3;
        let a655 = 44u64;
        nat_add(a654, a655)?
    })?, f111(v0.clone(), v1, v2, {
        let a656 = v3;
        let a657 = 48u64;
        nat_add(a656, a657)?
    })?, f111(v0.clone(), v1, v2, {
        let a658 = v3;
        let a659 = 52u64;
        nat_add(a658, a659)?
    })?, f111(v0.clone(), v1, v2, {
        let a660 = v3;
        let a661 = 56u64;
        nat_add(a660, a661)?
    })?, f111(v0, v1, v2, {
        let a662 = v3;
        let a663 = 60u64;
        nat_add(a662, a663)?
    })?))
}

pub fn f94(v0: Fn20, p668: u64, p669: Adt8) -> R<(Adt8, bool)> {
    let m667 = (p668, p669);
    {
        let mut m667 = m667;
        loop {
            let step = {
                let (v1, v2) = m667;
                let m664 = v1;
                if m664 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m664 - 1;
                    let m666 = {
                        let c665: Fn20 = v0.clone();
                        c665.apply(v2.clone())?
                    };
                    match m666 {
                        None => {
                            Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), R<(Adt8, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    f112(v0, v1)
}

pub fn f96(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64, v5: Bytes, v6: u64) -> Option<Adt1> {
    Some(Adt1::C0(v6, true, {
        let a670 = v2;
        let a671 = v1;
        nat_eq(a670, a671)
    }, List::cons(Adt0::C0(v2, v3, v4, v5), match v0 {
        Adt1::C0(_, _, _, h672) => {
            h672
        }
    })))
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a673 = v0;
        length_bytes(a673)
    };
    let a674 = v1;
    let a675 = v3;
    if nat_le(a674, a675) {
        let a678 = v2;
        let a679 = {
            let a676 = v3;
            let a677 = v1;
            nat_sub(a676, a677)
        };
        nat_le(a678, a679)
    } else {
        false
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h680) = f113(Fn23::F114(v1, v0), v2, (v2, 0u64))?;
    Ok(h680)
}

pub fn f99(v0: Bytes, v1: Bytes) -> bool {
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

pub fn f100(v0: u64, v1: Bytes) -> R<(bool, Adt6)> {
    let v2: (bool, List<Adt0>) = f1(v1.clone())?;
    let v3: List<Adt0> = {
        let (_, h687) = v2;
        h687
    };
    Ok(({
        let (h688, _) = v2;
        h688
    }, Adt6::C0(v0, v1, {
        let a691 = f42(f4(f2(v3.clone(), 3u64)))?;
        let a692 = {
            let a689 = f42(f4(f2(v3.clone(), 4u64)))?;
            let a690 = f42(f4(f2(v3.clone(), 7u64)))?;
            append_bytes(a689, a690)
        };
        append_bytes(a691, a692)
    }, f16(v3.clone(), 1u64), f16(v3, 2u64))))
}

pub fn f101(v0: Fn21, p696: List<Adt6>, p697: List<Adt6>) -> List<Adt6> {
    let m695 = (p696, p697);
    {
        let mut m695 = m695;
        loop {
            let step = {
                let (v1, v2) = m695;
                let m693 = v2;
                match m693.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c694: Fn21 = v0.clone();
                            c694.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f103(v0: List<Adt6>, v1: Adt7) -> Adt7 {
    if match v1.clone() {
        Adt7::C0(h698, _, _) => {
            h698
        }
    } {
        let v2: Adt11 = f115(Fn24::F116(v0.clone(), std::rc::Rc::new(v1.clone())), Adt11::C0(false, std::rc::Rc::new(Adt6::C0(0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), List::<Bytes>::nil()))), v0);
        if match v2.clone() {
            Adt11::C0(h699, _) => {
                h699
            }
        } {
            Adt7::C0(true, List::cons({
                let m702 = match v2.clone() {
                    Adt11::C0(_, h701) => {
                        (*h701).clone()
                    }
                };
                match m702 {
                    Adt6::C0(h700, _, _, _, _) => {
                        h700
                    }
                }
            }, match v1.clone() {
                Adt7::C0(_, h703, _) => {
                    h703
                }
            }), List::cons(match v2 {
                Adt11::C0(_, h704) => {
                    (*h704).clone()
                }
            }, match v1 {
                Adt7::C0(_, _, h705) => {
                    h705
                }
            }))
        } else {
            Adt7::C0(false, List::<u64>::nil(), List::<Adt6>::nil())
        }
    } else {
        v1
    }
}

pub fn f104(v0: List<Bytes>, v1: u64) -> R<(bool, Bytes)> {
    f28(Fn5::F117(v1), (true, Bytes::lit(&[])), v0)
}

pub fn f105(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f41(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f41(v1, 2u64)?;
    let v4: (bool, Bytes) = f118(f4({
        let (_, h706) = v3;
        h706
    }))?;
    Ok(({
        let (h707, _) = v0;
        if h707 {
            let (h708, _) = v2;
            if h708 {
                let (h709, _) = v3;
                if h709 {
                    let (h710, _) = v4;
                    h710
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a716 = {
            let (_, h711) = v0;
            h711
        };
        let a717 = {
            let a714 = f9(f4({
                let (_, h712) = v2;
                h712
            }))?;
            let a715 = {
                let (_, h713) = v4;
                h713
            };
            append_bytes(a714, a715)
        };
        append_bytes(a716, a717)
    }))
}

pub fn f106(v0: Adt6, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: List<Adt0> = {
        let (_, h718) = f1(match v0 {
            Adt6::C0(_, h719, _, _, _) => {
                h719
            }
        })?;
        h718
    };
    let v3: List<Bytes> = f16(v2.clone(), 5u64);
    let v4: Bytes = {
        let a734 = f9(f4(f2(v2.clone(), 3u64)))?;
        let a735 = {
            let a732 = f9(f4(f2(v2.clone(), 4u64)))?;
            let a733 = {
                let a730 = f9(f4(f2(v2.clone(), 7u64)))?;
                let a731 = {
                    let a728 = f9(f4(f2(v2.clone(), 8u64)))?;
                    let a729 = {
                        let a726 = f8(f119(v2.clone(), 1u64)?, 4u64)?;
                        let a727 = {
                            let a724 = f120(f16(v2.clone(), 1u64))?;
                            let a725 = {
                                let a722 = f8(f119(v2.clone(), 2u64)?, 4u64)?;
                                let a723 = {
                                    let a720 = f120(f16(v2, 2u64))?;
                                    let a721 = f8(f22(v3.clone())?, 4u64)?;
                                    append_bytes(a720, a721)
                                };
                                append_bytes(a722, a723)
                            };
                            append_bytes(a724, a725)
                        };
                        append_bytes(a726, a727)
                    };
                    append_bytes(a728, a729)
                };
                append_bytes(a730, a731)
            };
            append_bytes(a732, a733)
        };
        append_bytes(a734, a735)
    };
    f121(Fn25::F122(v1), (true, List::cons(f86(v4), List::<Adt3>::nil())), f25(v3.clone(), f30(v3, 1u64)?))
}

pub fn f107(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f108(v0: Fn22, p739: u64, p740: Adt10) -> R<Adt10> {
    let m738 = (p739, p740);
    {
        let mut m738 = m738;
        loop {
            let step = {
                let (v1, v2) = m738;
                let m736 = v1;
                if m736 == 0 {
                    Err::<(u64, Adt10), R<Adt10>>(Ok(v2))
                } else {
                    let v3 = m736 - 1;
                    Ok::<(u64, Adt10), R<Adt10>>((v3, {
                        let c737: Fn22 = v0.clone();
                        c737.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Adt10) -> R<Adt10> {
    f123(v0)
}

pub fn f110(v0: u32, v1: u32) -> u32 {
    let m743 = {
        let a741 = f124(v0);
        let a742 = f124(v1);
        fixed_u64::checked_add(a741, a742)
    };
    match m743 {
        None => {
            0u32
        }
        Some(v2) => {
            f125(v2)
        }
    }
}

pub fn f111(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a746 = {
        let a744 = v3;
        let a745 = 8u64;
        nat_add(a744, a745)?
    };
    let a747 = v2;
    if nat_eq(a746, a747) {
        Ok(f126({
            let a750 = {
                let a748 = v1;
                let a749 = 8u64;
                nat_mul(a748, a749)?
            };
            let a751 = 4294967296u64;
            let a752 = 0u64;
            nat_quot(a750, a751, a752)
        }))
    } else {
        let a755 = {
            let a753 = v3;
            let a754 = 4u64;
            nat_add(a753, a754)?
        };
        let a756 = v2;
        if nat_eq(a755, a756) {
            Ok(f126({
                let a759 = {
                    let a757 = v1;
                    let a758 = 8u64;
                    nat_mul(a757, a758)?
                };
                let a760 = 4294967296u64;
                let a761 = 0u64;
                nat_rem(a759, a760, a761)
            }))
        } else {
            Ok(f127(f128(v0.clone(), v1, v3), f128(v0.clone(), v1, {
                let a762 = v3;
                let a763 = 1u64;
                nat_add(a762, a763)?
            }), f128(v0.clone(), v1, {
                let a764 = v3;
                let a765 = 2u64;
                nat_add(a764, a765)?
            }), f128(v0, v1, {
                let a766 = v3;
                let a767 = 3u64;
                nat_add(a766, a767)?
            })))
        }
    }
}

pub fn f112(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    if match v1.clone() {
        Adt8::C0(_, _, _, _, _, h768) => {
            h768
        }
    } {
        Ok(None::<Adt8>)
    } else {
        let v2: u64 = match v1.clone() {
            Adt8::C0(h769, _, _, _, _, _) => {
                h769
            }
        };
        let a771 = {
            let a770 = v0.clone();
            length_bytes(a770)
        };
        let a772 = v2;
        if if nat_le(a771, a772) {
            true
        } else {
            let a774 = 70u64;
            let a775 = match v1.clone() {
                Adt8::C0(_, _, h773, _, _, _) => {
                    h773
                }
            };
            nat_le(a774, a775)
        } {
            Ok(Some(Adt8::C0(v2, 0u64, 0u64, 1u64, false, true)))
        } else {
            let v3: u64 = f58(v0, v2)?;
            let v4: u64 = {
                let a776 = v3;
                let a777 = 128u64;
                let a778 = 0u64;
                nat_rem(a776, a777, a778)
            };
            let v5: u64 = {
                let a780 = match v1.clone() {
                    Adt8::C0(_, _, h779, _, _, _) => {
                        h779
                    }
                };
                let a781 = 63u64;
                if nat_eq(a780, a781) {
                    let a786 = {
                        let a782 = v4;
                        let a783 = 2u64;
                        let a784 = 0u64;
                        nat_rem(a782, a783, a784)
                    };
                    let a787 = match v1.clone() {
                        Adt8::C0(_, _, _, h785, _, _) => {
                            h785
                        }
                    };
                    nat_mul(a786, a787)?
                } else {
                    let a789 = v4;
                    let a790 = match v1.clone() {
                        Adt8::C0(_, _, _, h788, _, _) => {
                            h788
                        }
                    };
                    nat_mul(a789, a790)?
                }
            };
            Ok(Some(Adt8::C0({
                let a791 = v2;
                let a792 = 1u64;
                nat_add(a791, a792)?
            }, {
                let a794 = match v1.clone() {
                    Adt8::C0(_, h793, _, _, _, _) => {
                        h793
                    }
                };
                let a795 = v5;
                nat_add(a794, a795)?
            }, {
                let a797 = match v1.clone() {
                    Adt8::C0(_, _, h796, _, _, _) => {
                        h796
                    }
                };
                let a798 = 7u64;
                nat_add(a797, a798)?
            }, {
                let a800 = match v1.clone() {
                    Adt8::C0(_, _, h799, _, _, _) => {
                        h799
                    }
                };
                let a801 = 63u64;
                if nat_lt(a800, a801) {
                    let a803 = match v1 {
                        Adt8::C0(_, _, _, h802, _, _) => {
                            h802
                        }
                    };
                    let a804 = 128u64;
                    nat_mul(a803, a804)?
                } else {
                    match v1 {
                        Adt8::C0(_, _, _, h805, _, _) => {
                            h805
                        }
                    }
                }
            }, true, {
                let a806 = v3;
                let a807 = 128u64;
                nat_lt(a806, a807)
            })))
        }
    }
}

pub fn f113(v0: Fn23, p811: u64, p812: (u64, u64)) -> R<(u64, u64)> {
    let m810 = (p811, p812);
    {
        let mut m810 = m810;
        loop {
            let step = {
                let (v1, v2) = m810;
                let m808 = v1;
                if m808 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m808 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c809: Fn23 = v0.clone();
                        c809.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a814 = {
            let (h813, _) = v2;
            h813
        };
        let a815 = 1u64;
        nat_sub(a814, a815)
    };
    Ok((v3, {
        let a821 = {
            let a817 = {
                let (_, h816) = v2;
                h816
            };
            let a818 = 256u64;
            nat_mul(a817, a818)?
        };
        let a822 = f58(v1, {
            let a819 = v0;
            let a820 = v3;
            nat_add(a819, a820)?
        })?;
        nat_add(a821, a822)?
    }))
}

pub fn f115(v0: Fn24, p826: Adt11, p827: List<Adt6>) -> Adt11 {
    let m825 = (p826, p827);
    {
        let mut m825 = m825;
        loop {
            let step = {
                let (v1, v2) = m825;
                let m823 = v2;
                match m823.uncons() {
                    None => {
                        Err::<(Adt11, List<Adt6>), Adt11>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt11, List<Adt6>), Adt11>(({
                            let c824: Fn24 = v0.clone();
                            c824.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m825 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<Adt6>, v1: Adt7, v2: Adt11, v3: Adt6) -> Adt11 {
    if if f129(match v1.clone() {
        Adt7::C0(_, h828, _) => {
            h828
        }
    }, match v3.clone() {
        Adt6::C0(h829, _, _, _, _) => {
            h829
        }
    }) {
        true
    } else {
        !f130(v0, match v1 {
            Adt7::C0(_, h830, _) => {
                h830
            }
        }, v3.clone())
    } {
        v2
    } else {
        if match v2.clone() {
            Adt11::C0(h831, _) => {
                h831
            }
        } {
            if f99({
                let m834 = match v2.clone() {
                    Adt11::C0(_, h833) => {
                        (*h833).clone()
                    }
                };
                match m834 {
                    Adt6::C0(_, _, h832, _, _) => {
                        h832
                    }
                }
            }, match v3.clone() {
                Adt6::C0(_, _, h835, _, _) => {
                    h835
                }
            }) {
                v2
            } else {
                Adt11::C0(true, std::rc::Rc::new(v3))
            }
        } else {
            Adt11::C0(true, std::rc::Rc::new(v3))
        }
    }
}

pub fn f117(v0: u64, v1: (bool, Bytes), v2: Bytes) -> R<(bool, Bytes)> {
    let v3: (bool, Bytes) = {
        let a836 = v0;
        let a837 = 0u64;
        if nat_eq(a836, a837) {
            f131(v2)?
        } else {
            f118(v2)?
        }
    };
    Ok(({
        let (h838, _) = v1;
        if h838 {
            let (h839, _) = v3;
            h839
        } else {
            false
        }
    }, {
        let a842 = {
            let (_, h840) = v1;
            h840
        };
        let a843 = {
            let (_, h841) = v3;
            h841
        };
        append_bytes(a842, a843)
    }))
}

pub fn f118(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f1(v0.clone())?;
    let v2: Adt12 = {
        let (h844, _) = f132(Fn26::F133, {
            let a848 = {
                let a846 = {
                    let a845 = v0;
                    length_bytes(a845)
                };
                let a847 = 40u64;
                nat_mul(a846, a847)?
            };
            let a849 = 8u64;
            nat_add(a848, a849)?
        }, Adt12::C0(List::cons(Adt13::C0(f134({
            let (_, h850) = v1;
            h850
        }), Bytes::lit(&[]), 0u64), List::<Adt13>::nil()), Bytes::lit(&[]), false))?;
        h844
    };
    Ok(({
        let (h851, _) = v1;
        if h851 {
            match v2.clone() {
                Adt12::C0(_, _, h852) => {
                    h852
                }
            }
        } else {
            false
        }
    }, match v2 {
        Adt12::C0(_, h853, _) => {
            h853
        }
    }))
}

pub fn f119(v0: List<Adt0>, v1: u64) -> R<u64> {
    f135(Fn27::F136(v1), 0u64, v0)
}

pub fn f120(v0: List<Bytes>) -> R<Bytes> {
    f23(Fn3::F137, Bytes::lit(&[]), v0)
}

pub fn f121(v0: Fn25, p857: (bool, List<Adt3>), p858: List<Bytes>) -> R<(bool, List<Adt3>)> {
    let m856 = (p857, p858);
    {
        let mut m856 = m856;
        loop {
            let step = {
                let (v1, v2) = m856;
                let m854 = v2;
                match m854.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(({
                            let c855: Fn25 = v0.clone();
                            c855.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64, v1: (bool, List<Adt3>), v2: Bytes) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f138(v2, v0)?;
    Ok(({
        let (h859, _) = v1;
        if h859 {
            let (h860, _) = v3;
            h860
        } else {
            false
        }
    }, f66(Fn14::F139, {
        let (_, h861) = v1;
        h861
    }, f68({
        let (_, h862) = v3;
        h862
    }))))
}

pub fn f123(v0: Adt10) -> R<Adt10> {
    let v1: u32 = f140(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, h863) => {
            h863
        }
    });
    let v2: u32 = f110(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, h864, _, _) => {
            h864
        }
    }, f110(f141(match v0.clone() {
        Adt10::C0(_, _, _, _, h865, _, _, _, _, _) => {
            h865
        }
    }), f110(f142(match v0.clone() {
        Adt10::C0(_, _, _, _, h866, _, _, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h867, _, _, _, _) => {
            h867
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h868, _, _, _) => {
            h868
        }
    }), f110(v1, {
        let m871 = match v0.clone() {
            Adt10::C0(_, _, _, _, _, _, _, _, h870, _) => {
                (*h870).clone()
            }
        };
        match m871 {
            Adt9::C0(h869, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h869
            }
        }
    }))));
    let v3: u32 = f110(f143(match v0.clone() {
        Adt10::C0(h872, _, _, _, _, _, _, _, _, _) => {
            h872
        }
    }), f144(match v0.clone() {
        Adt10::C0(h873, _, _, _, _, _, _, _, _, _) => {
            h873
        }
    }, match v0.clone() {
        Adt10::C0(_, h874, _, _, _, _, _, _, _, _) => {
            h874
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h875, _, _, _, _, _, _, _) => {
            h875
        }
    }));
    Ok(Adt10::C0(f110(v2, v3), match v0.clone() {
        Adt10::C0(h876, _, _, _, _, _, _, _, _, _) => {
            h876
        }
    }, match v0.clone() {
        Adt10::C0(_, h877, _, _, _, _, _, _, _, _) => {
            h877
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h878, _, _, _, _, _, _, _) => {
            h878
        }
    }, f110(match v0.clone() {
        Adt10::C0(_, _, _, h879, _, _, _, _, _, _) => {
            h879
        }
    }, v2), match v0.clone() {
        Adt10::C0(_, _, _, _, h880, _, _, _, _, _) => {
            h880
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h881, _, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h882, _, _, _) => {
            h882
        }
    }, std::rc::Rc::new(f145(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, h883, _) => {
            (*h883).clone()
        }
    })), {
        let a885 = match v0 {
            Adt10::C0(_, _, _, _, _, _, _, _, _, h884) => {
                h884
            }
        };
        let a886 = 1u64;
        nat_add(a885, a886)?
    }))
}

pub fn f124(v0: u32) -> u64 {
    let m888 = {
        let a887 = v0;
        fixed_u64::convert(i128::from(a887))
    };
    match m888 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f125(v0: u64) -> u32 {
    let m892 = {
        let a891 = {
            let a889 = v0;
            let a890 = 4294967295u64;
            fixed_u64::bit_and(a889, a890)
        };
        fixed_u32::convert(i128::from(a891))
    };
    match m892 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f126(v0: u64) -> u32 {
    let (_, h893) = {
        let (_, h894) = f146(Fn28::F147, 32u64, (v0, (1u32, 0u32)));
        h894
    };
    h893
}

pub fn f127(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a899 = f148(v0, 24u32);
    let a900 = {
        let a897 = f148(v1, 16u32);
        let a898 = {
            let a895 = f148(v2, 8u32);
            let a896 = v3;
            fixed_u32::bit_or(a895, a896)
        };
        fixed_u32::bit_or(a897, a898)
    };
    fixed_u32::bit_or(a899, a900)
}

pub fn f128(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a901 = v2;
    let a902 = v1;
    if nat_lt(a901, a902) {
        let m905 = {
            let a903 = v0;
            let a904 = v2;
            index_bytes(a903, a904)
        };
        match m905 {
            None => {
                0u32
            }
            Some(v3) => {
                let m907 = {
                    let a906 = v3;
                    fixed_u32::convert(i128::from(a906))
                };
                match m907 {
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
        let a908 = v2;
        let a909 = v1;
        if nat_eq(a908, a909) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f129(v0: List<u64>, v1: u64) -> bool {
    f149(Fn29::F150(v1), false, v0)
}

pub fn f130(v0: List<Adt6>, v1: List<u64>, v2: Adt6) -> bool {
    f151(Fn30::F152(v1, v0), true, match v2 {
        Adt6::C0(_, _, _, h910, _) => {
            h910
        }
    })
}

pub fn f131(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f1(v0.clone())?;
    let v2: List<Adt0> = {
        let (_, h911) = v1;
        h911
    };
    let v3: u64 = {
        let a912 = f10(f2(v2.clone(), 2u64));
        let a913 = 4294967296u64;
        let a914 = 0u64;
        nat_rem(a912, a913, a914)
    };
    let v4: (bool, List<u64>) = f153(v2.clone(), 1u64)?;
    let v5: (bool, Bytes) = f154(v0, v2.clone(), v3)?;
    Ok(({
        let (h915, _) = v1;
        if if h915 {
            let a916 = 0u64;
            let a917 = v3;
            if nat_lt(a916, a917) {
                let a918 = v3;
                let a919 = 24u64;
                nat_lt(a918, a919)
            } else {
                false
            }
        } else {
            false
        } {
            let (h920, _) = v4.clone();
            if h920 {
                let (h921, _) = v5;
                h921
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a931 = f9(f4(f2(v2, 8u64)))?;
        let a932 = {
            let a929 = f8(v3, 4u64)?;
            let a930 = {
                let a927 = f8(f155({
                    let (_, h922) = v4.clone();
                    h922
                })?, 4u64)?;
                let a928 = {
                    let a925 = f156({
                        let (_, h923) = v4;
                        h923
                    }, 8u64)?;
                    let a926 = {
                        let (_, h924) = v5;
                        h924
                    };
                    append_bytes(a925, a926)
                };
                append_bytes(a927, a928)
            };
            append_bytes(a929, a930)
        };
        append_bytes(a931, a932)
    }))
}

pub fn f132(v0: Fn26, p937: u64, p938: Adt12) -> R<(Adt12, bool)> {
    let m936 = (p937, p938);
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, v2) = m936;
                let m933 = v1;
                if m933 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m933 - 1;
                    let m935 = {
                        let c934: Fn26 = v0.clone();
                        c934.apply(v2.clone())?
                    };
                    match m935 {
                        None => {
                            Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt12), R<(Adt12, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Adt12) -> R<Option<Adt12>> {
    f157(v0)
}

pub fn f134(v0: List<Adt0>) -> List<Adt0> {
    f37(Fn8::F158, List::<Adt0>::nil(), v0)
}

pub fn f135(v0: Fn27, p942: u64, p943: List<Adt0>) -> R<u64> {
    let m941 = (p942, p943);
    {
        let mut m941 = m941;
        loop {
            let step = {
                let (v1, v2) = m941;
                let m939 = v2;
                match m939.uncons() {
                    None => {
                        Err::<(u64, List<Adt0>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt0>), R<u64>>(({
                            let c940: Fn27 = v0.clone();
                            c940.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: u64, v1: u64, v2: Adt0) -> R<u64> {
    let a945 = match v2 {
        Adt0::C0(h944, _, _, _) => {
            h944
        }
    };
    let a946 = v0;
    if nat_eq(a945, a946) {
        let a947 = v1;
        let a948 = 1u64;
        nat_add(a947, a948)
    } else {
        Ok(v1)
    }
}

pub fn f137(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a949 = v0;
    let a950 = f9(v1)?;
    Ok(append_bytes(a949, a950))
}

pub fn f138(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, Adt0) = f41(v0.clone(), 1u64)?;
    let v3: (bool, Adt0) = f41(v0.clone(), 20u64)?;
    let v4: u64 = {
        let a952 = f10({
            let (_, h951) = v3;
            h951
        });
        let a953 = 4294967296u64;
        let a954 = 0u64;
        nat_rem(a952, a953, a954)
    };
    let v5: Adt3 = f86({
        let a956 = f9(f4({
            let (_, h955) = v2;
            h955
        }))?;
        let a957 = f8(v4, 4u64)?;
        append_bytes(a956, a957)
    });
    let v6: bool = {
        let (h958, _) = v2;
        if h958 {
            let (h959, _) = v3;
            h959
        } else {
            false
        }
    };
    let a960 = v4;
    let a961 = 5u64;
    if nat_eq(a960, a961) {
        let v7: (bool, Adt0) = f41(v0, 6u64)?;
        Ok((if v6 {
            let (h962, _) = v7;
            h962
        } else {
            false
        }, List::cons(Adt3::C0(true, f4({
            let (_, h963) = v7;
            h963
        }), {
            let a964 = v1;
            let a965 = 1u64;
            nat_add(a964, a965)?
        }), List::cons(v5, List::<Adt3>::nil()))))
    } else {
        let a966 = v4;
        let a967 = 10u64;
        if nat_eq(a966, a967) {
            let v8: (bool, List<Adt0>) = f1(v0)?;
            Ok((if v6 {
                let (h968, _) = v8;
                h968
            } else {
                false
            }, f159(Fn31::F160(v1), List::cons(v5, List::<Adt3>::nil()), f16({
                let (_, h969) = v8;
                h969
            }, 11u64))?))
        } else {
            let v9: (bool, Bytes) = f161(v0, v4)?;
            Ok((if v6 {
                let (h970, _) = v9;
                h970
            } else {
                false
            }, List::cons(f86({
                let (_, h971) = v9;
                h971
            }), List::cons(v5, List::<Adt3>::nil()))))
        }
    }
}

pub fn f139(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f140(v0: u64) -> u32 {
    let a972 = v0;
    let a973 = 32u64;
    if nat_lt(a972, a973) {
        let a974 = v0;
        let a975 = 16u64;
        if nat_lt(a974, a975) {
            let a976 = v0;
            let a977 = 8u64;
            if nat_lt(a976, a977) {
                let a978 = v0;
                let a979 = 4u64;
                if nat_lt(a978, a979) {
                    let a980 = v0;
                    let a981 = 2u64;
                    if nat_lt(a980, a981) {
                        let a982 = v0;
                        let a983 = 1u64;
                        if nat_lt(a982, a983) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a984 = v0;
                        let a985 = 3u64;
                        if nat_lt(a984, a985) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a986 = v0;
                    let a987 = 6u64;
                    if nat_lt(a986, a987) {
                        let a988 = v0;
                        let a989 = 5u64;
                        if nat_lt(a988, a989) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a990 = v0;
                        let a991 = 7u64;
                        if nat_lt(a990, a991) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a992 = v0;
                let a993 = 12u64;
                if nat_lt(a992, a993) {
                    let a994 = v0;
                    let a995 = 10u64;
                    if nat_lt(a994, a995) {
                        let a996 = v0;
                        let a997 = 9u64;
                        if nat_lt(a996, a997) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a998 = v0;
                        let a999 = 11u64;
                        if nat_lt(a998, a999) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1000 = v0;
                    let a1001 = 14u64;
                    if nat_lt(a1000, a1001) {
                        let a1002 = v0;
                        let a1003 = 13u64;
                        if nat_lt(a1002, a1003) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1004 = v0;
                        let a1005 = 15u64;
                        if nat_lt(a1004, a1005) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1006 = v0;
            let a1007 = 24u64;
            if nat_lt(a1006, a1007) {
                let a1008 = v0;
                let a1009 = 20u64;
                if nat_lt(a1008, a1009) {
                    let a1010 = v0;
                    let a1011 = 18u64;
                    if nat_lt(a1010, a1011) {
                        let a1012 = v0;
                        let a1013 = 17u64;
                        if nat_lt(a1012, a1013) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1014 = v0;
                        let a1015 = 19u64;
                        if nat_lt(a1014, a1015) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1016 = v0;
                    let a1017 = 22u64;
                    if nat_lt(a1016, a1017) {
                        let a1018 = v0;
                        let a1019 = 21u64;
                        if nat_lt(a1018, a1019) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1020 = v0;
                        let a1021 = 23u64;
                        if nat_lt(a1020, a1021) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1022 = v0;
                let a1023 = 28u64;
                if nat_lt(a1022, a1023) {
                    let a1024 = v0;
                    let a1025 = 26u64;
                    if nat_lt(a1024, a1025) {
                        let a1026 = v0;
                        let a1027 = 25u64;
                        if nat_lt(a1026, a1027) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1028 = v0;
                        let a1029 = 27u64;
                        if nat_lt(a1028, a1029) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1030 = v0;
                    let a1031 = 30u64;
                    if nat_lt(a1030, a1031) {
                        let a1032 = v0;
                        let a1033 = 29u64;
                        if nat_lt(a1032, a1033) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1034 = v0;
                        let a1035 = 31u64;
                        if nat_lt(a1034, a1035) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1036 = v0;
        let a1037 = 48u64;
        if nat_lt(a1036, a1037) {
            let a1038 = v0;
            let a1039 = 40u64;
            if nat_lt(a1038, a1039) {
                let a1040 = v0;
                let a1041 = 36u64;
                if nat_lt(a1040, a1041) {
                    let a1042 = v0;
                    let a1043 = 34u64;
                    if nat_lt(a1042, a1043) {
                        let a1044 = v0;
                        let a1045 = 33u64;
                        if nat_lt(a1044, a1045) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1046 = v0;
                        let a1047 = 35u64;
                        if nat_lt(a1046, a1047) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1048 = v0;
                    let a1049 = 38u64;
                    if nat_lt(a1048, a1049) {
                        let a1050 = v0;
                        let a1051 = 37u64;
                        if nat_lt(a1050, a1051) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1052 = v0;
                        let a1053 = 39u64;
                        if nat_lt(a1052, a1053) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1054 = v0;
                let a1055 = 44u64;
                if nat_lt(a1054, a1055) {
                    let a1056 = v0;
                    let a1057 = 42u64;
                    if nat_lt(a1056, a1057) {
                        let a1058 = v0;
                        let a1059 = 41u64;
                        if nat_lt(a1058, a1059) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1060 = v0;
                        let a1061 = 43u64;
                        if nat_lt(a1060, a1061) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1062 = v0;
                    let a1063 = 46u64;
                    if nat_lt(a1062, a1063) {
                        let a1064 = v0;
                        let a1065 = 45u64;
                        if nat_lt(a1064, a1065) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1066 = v0;
                        let a1067 = 47u64;
                        if nat_lt(a1066, a1067) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1068 = v0;
            let a1069 = 56u64;
            if nat_lt(a1068, a1069) {
                let a1070 = v0;
                let a1071 = 52u64;
                if nat_lt(a1070, a1071) {
                    let a1072 = v0;
                    let a1073 = 50u64;
                    if nat_lt(a1072, a1073) {
                        let a1074 = v0;
                        let a1075 = 49u64;
                        if nat_lt(a1074, a1075) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1076 = v0;
                        let a1077 = 51u64;
                        if nat_lt(a1076, a1077) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1078 = v0;
                    let a1079 = 54u64;
                    if nat_lt(a1078, a1079) {
                        let a1080 = v0;
                        let a1081 = 53u64;
                        if nat_lt(a1080, a1081) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1082 = v0;
                        let a1083 = 55u64;
                        if nat_lt(a1082, a1083) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1084 = v0;
                let a1085 = 60u64;
                if nat_lt(a1084, a1085) {
                    let a1086 = v0;
                    let a1087 = 58u64;
                    if nat_lt(a1086, a1087) {
                        let a1088 = v0;
                        let a1089 = 57u64;
                        if nat_lt(a1088, a1089) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1090 = v0;
                        let a1091 = 59u64;
                        if nat_lt(a1090, a1091) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1092 = v0;
                    let a1093 = 62u64;
                    if nat_lt(a1092, a1093) {
                        let a1094 = v0;
                        let a1095 = 61u64;
                        if nat_lt(a1094, a1095) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1096 = v0;
                        let a1097 = 63u64;
                        if nat_lt(a1096, a1097) {
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

pub fn f141(v0: u32) -> u32 {
    let a1100 = {
        let a1098 = f162(v0, 6u32, 26u32);
        let a1099 = f162(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1098, a1099)
    };
    let a1101 = f162(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1100, a1101)
}

pub fn f142(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1107 = {
        let a1102 = v0;
        let a1103 = v1;
        fixed_u32::bit_and(a1102, a1103)
    };
    let a1108 = {
        let a1105 = {
            let a1104 = v0;
            fixed_u32::bit_not(a1104)
        };
        let a1106 = v2;
        fixed_u32::bit_and(a1105, a1106)
    };
    fixed_u32::bit_xor(a1107, a1108)
}

pub fn f143(v0: u32) -> u32 {
    let a1111 = {
        let a1109 = f162(v0, 2u32, 30u32);
        let a1110 = f162(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1109, a1110)
    };
    let a1112 = f162(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1111, a1112)
}

pub fn f144(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1121 = {
        let a1117 = {
            let a1113 = v0;
            let a1114 = v1;
            fixed_u32::bit_and(a1113, a1114)
        };
        let a1118 = {
            let a1115 = v0;
            let a1116 = v2;
            fixed_u32::bit_and(a1115, a1116)
        };
        fixed_u32::bit_xor(a1117, a1118)
    };
    let a1122 = {
        let a1119 = v1;
        let a1120 = v2;
        fixed_u32::bit_and(a1119, a1120)
    };
    fixed_u32::bit_xor(a1121, a1122)
}

pub fn f145(v0: Adt9) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(_, h1123, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1123
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h1124, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1124
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, h1125, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1125
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1126, _, _, _, _, _, _, _, _, _, _, _) => {
            h1126
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1127, _, _, _, _, _, _, _, _, _, _) => {
            h1127
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1128, _, _, _, _, _, _, _, _, _) => {
            h1128
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1129, _, _, _, _, _, _, _, _) => {
            h1129
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1130, _, _, _, _, _, _, _) => {
            h1130
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1131, _, _, _, _, _, _) => {
            h1131
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1132, _, _, _, _, _) => {
            h1132
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1133, _, _, _, _) => {
            h1133
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1134, _, _, _) => {
            h1134
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1135, _, _) => {
            h1135
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1136, _) => {
            h1136
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1137) => {
            h1137
        }
    }, f110(f163(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1138, _) => {
            h1138
        }
    }), f110(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1139, _, _, _, _, _, _) => {
            h1139
        }
    }, f110(f164(match v0.clone() {
        Adt9::C0(_, h1140, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1140
        }
    }), match v0 {
        Adt9::C0(h1141, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1141
        }
    }))))
}

pub fn f146(v0: Fn28, p1145: u64, p1146: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1144 = (p1145, p1146);
    {
        let mut m1144 = m1144;
        loop {
            let step = {
                let (v1, v2) = m1144;
                let m1142 = v1;
                if m1142 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1142 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1143: Fn28 = v0.clone();
                        c1143.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1147, _) = v0;
        h1147
    };
    let v2: u32 = {
        let (h1148, _) = {
            let (_, h1149) = v0;
            h1149
        };
        h1148
    };
    let v3: u32 = {
        let (_, h1150) = {
            let (_, h1151) = v0;
            h1151
        };
        h1150
    };
    ({
        let a1152 = v1;
        let a1153 = 2u64;
        let a1154 = 0u64;
        nat_quot(a1152, a1153, a1154)
    }, (f148(v2, 1u32), {
        let a1158 = {
            let a1155 = v1;
            let a1156 = 2u64;
            let a1157 = 0u64;
            nat_rem(a1155, a1156, a1157)
        };
        let a1159 = 1u64;
        if nat_eq(a1158, a1159) {
            let a1160 = v3;
            let a1161 = v2;
            fixed_u32::bit_or(a1160, a1161)
        } else {
            v3
        }
    }))
}

pub fn f148(v0: u32, v1: u32) -> u32 {
    let m1164 = {
        let a1162 = v0;
        let a1163 = v1;
        fixed_u32::shift_left(a1162, a1163)
    };
    match m1164 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f149(v0: Fn29, p1168: bool, p1169: List<u64>) -> bool {
    let m1167 = (p1168, p1169);
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let (v1, v2) = m1167;
                let m1165 = v2;
                match m1165.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1166: Fn29 = v0.clone();
                            c1166.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1170 = v2;
        let a1171 = v0;
        nat_eq(a1170, a1171)
    }
}

pub fn f151(v0: Fn30, p1175: bool, p1176: List<Bytes>) -> bool {
    let m1174 = (p1175, p1176);
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let (v1, v2) = m1174;
                let m1172 = v2;
                match m1172.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1173: Fn30 = v0.clone();
                            c1173.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<u64>, v1: List<Adt6>, v2: bool, v3: Bytes) -> bool {
    let a1178 = 0u64;
    let a1179 = {
        let a1177 = v3.clone();
        length_bytes(a1177)
    };
    if nat_lt(a1178, a1179) {
        f165(Fn32::F166(v0, v3), v2, v1)
    } else {
        v2
    }
}

pub fn f153(v0: List<Adt0>, v1: u64) -> R<(bool, List<u64>)> {
    let v2: (bool, List<u64>) = f167(Fn33::F168(v1), (true, List::<u64>::nil()), v0)?;
    Ok(({
        let (h1180, _) = v2;
        h1180
    }, f169(Fn34::F170, List::<u64>::nil(), {
        let (_, h1181) = v2;
        h1181
    })))
}

pub fn f154(v0: Bytes, v1: List<Adt0>, v2: u64) -> R<(bool, Bytes)> {
    let a1182 = f10(f2(v1.clone(), 14u64));
    let a1183 = 1u64;
    if nat_eq(a1182, a1183) {
        let v3: (bool, Bytes) = f6(v0, 13u64)?;
        Ok(({
            let (h1184, _) = v3;
            h1184
        }, f9({
            let a1186 = f171(Str::lit("onnx:external-data:v1"));
            let a1187 = {
                let (_, h1185) = v3;
                h1185
            };
            append_bytes(a1186, a1187)
        })?))
    } else {
        let v4: Adt0 = f2(v1.clone(), 9u64);
        let a1189 = match v4.clone() {
            Adt0::C0(_, h1188, _, _) => {
                h1188
            }
        };
        let a1190 = 2u64;
        if if nat_eq(a1189, a1190) {
            let a1193 = 0u64;
            let a1194 = {
                let a1192 = match v4.clone() {
                    Adt0::C0(_, _, _, h1191) => {
                        h1191
                    }
                };
                length_bytes(a1192)
            };
            nat_lt(a1193, a1194)
        } else {
            false
        } {
            Ok((true, f9(match v4 {
                Adt0::C0(_, _, _, h1195) => {
                    h1195
                }
            })?))
        } else {
            let v5: (u64, u64) = f172(v2);
            let a1197 = {
                let (_, h1196) = v5;
                h1196
            };
            let a1198 = 0u64;
            if nat_eq(a1197, a1198) {
                let a1199 = v2;
                let a1200 = 8u64;
                if nat_eq(a1199, a1200) {
                    Ok((true, f9(f120(f16(v1, 6u64))?)?))
                } else {
                    Ok((true, f9(f173(v1, {
                        let (h1201, _) = v5;
                        h1201
                    })?)?))
                }
            } else {
                let v6: (bool, List<u64>) = f153(v1, {
                    let (h1202, _) = v5;
                    h1202
                })?;
                Ok(({
                    let (h1203, _) = v6;
                    h1203
                }, f9(f156({
                    let (_, h1204) = v6;
                    h1204
                }, {
                    let (_, h1205) = v5;
                    h1205
                })?)?))
            }
        }
    }
}

pub fn f155(v0: List<u64>) -> R<u64> {
    f174(Fn35::F175, 0u64, v0)
}

pub fn f156(v0: List<u64>, v1: u64) -> R<Bytes> {
    f176(Fn36::F177(v1), Bytes::lit(&[]), v0)
}

pub fn f157(v0: Adt12) -> R<Option<Adt12>> {
    if match v0.clone() {
        Adt12::C0(_, _, h1206) => {
            h1206
        }
    } {
        Ok(None::<Adt12>)
    } else {
        let m1208 = match v0 {
            Adt12::C0(h1207, _, _) => {
                h1207
            }
        };
        match m1208.uncons() {
            None => {
                Ok(None::<Adt12>)
            }
            Some((v1, v2)) => {
                let m1210 = match v1.clone() {
                    Adt13::C0(h1209, _, _) => {
                        h1209
                    }
                };
                match m1210.uncons() {
                    None => {
                        Ok(Some(f178(v2, f9(match v1 {
                            Adt13::C0(_, h1211, _) => {
                                h1211
                            }
                        })?)))
                    }
                    Some((v3, v4)) => {
                        let v5: Bytes = {
                            let a1213 = match v1.clone() {
                                Adt13::C0(_, h1212, _) => {
                                    h1212
                                }
                            };
                            let a1214 = f179(v3.clone())?;
                            append_bytes(a1213, a1214)
                        };
                        let v6: u64 = match v1 {
                            Adt13::C0(_, _, h1215) => {
                                h1215
                            }
                        };
                        let v7: Bytes = match v3.clone() {
                            Adt0::C0(_, _, _, h1216) => {
                                h1216
                            }
                        };
                        let v8: (bool, List<Adt0>) = f1(v7.clone())?;
                        let v9: bool = {
                            let a1218 = match v3.clone() {
                                Adt0::C0(_, h1217, _, _) => {
                                    h1217
                                }
                            };
                            let a1219 = 2u64;
                            if if nat_eq(a1218, a1219) {
                                let a1220 = v6;
                                let a1221 = 32u64;
                                nat_lt(a1220, a1221)
                            } else {
                                false
                            } {
                                let a1223 = 0u64;
                                let a1224 = {
                                    let a1222 = v7.clone();
                                    length_bytes(a1222)
                                };
                                if nat_lt(a1223, a1224) {
                                    let (h1225, _) = v8;
                                    h1225
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        if v9 {
                            Ok(Some(Adt12::C0(List::cons(Adt13::C0(f134({
                                let (_, h1226) = v8;
                                h1226
                            }), Bytes::lit(&[]), {
                                let a1227 = v6;
                                let a1228 = 1u64;
                                nat_add(a1227, a1228)?
                            }), List::cons(Adt13::C0(v4, v5, v6), v2)), Bytes::lit(&[]), false)))
                        } else {
                            Ok(Some(Adt12::C0(List::cons(Adt13::C0(v4, {
                                let a1230 = match v3 {
                                    Adt0::C0(_, h1229, _, _) => {
                                        h1229
                                    }
                                };
                                let a1231 = 2u64;
                                if nat_eq(a1230, a1231) {
                                    let a1232 = v5;
                                    let a1233 = f9(v7)?;
                                    append_bytes(a1232, a1233)
                                } else {
                                    v5
                                }
                            }, v6), v2), Bytes::lit(&[]), false)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f158(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    f180(v1, v0)
}

pub fn f159(v0: Fn31, p1237: List<Adt3>, p1238: List<Bytes>) -> R<List<Adt3>> {
    let m1236 = (p1237, p1238);
    {
        let mut m1236 = m1236;
        loop {
            let step = {
                let (v1, v2) = m1236;
                let m1234 = v2;
                match m1234.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(({
                            let c1235: Fn31 = v0.clone();
                            c1235.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: u64, v1: List<Adt3>, v2: Bytes) -> R<List<Adt3>> {
    Ok(List::cons(Adt3::C0(true, v2, {
        let a1239 = v0;
        let a1240 = 1u64;
        nat_add(a1239, a1240)?
    }), v1))
}

pub fn f161(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f1(v0.clone())?;
    let v3: List<Adt0> = {
        let (_, h1241) = v2;
        h1241
    };
    let a1242 = v1;
    let a1243 = 1u64;
    if nat_eq(a1242, a1243) {
        let v4: (bool, Adt0) = f41(v0, 2u64)?;
        Ok(({
            let (h1244, _) = v4.clone();
            h1244
        }, {
            let a1248 = {
                let m1247 = {
                    let (_, h1246) = v4.clone();
                    h1246
                };
                match m1247 {
                    Adt0::C0(_, h1245, _, _) => {
                        h1245
                    }
                }
            };
            let a1249 = 5u64;
            if nat_eq(a1248, a1249) {
                f8({
                    let m1252 = {
                        let (_, h1251) = v4;
                        h1251
                    };
                    match m1252 {
                        Adt0::C0(_, _, h1250, _) => {
                            h1250
                        }
                    }
                }, 4u64)?
            } else {
                Bytes::lit(&[])
            }
        }))
    } else {
        let a1253 = v1;
        let a1254 = 2u64;
        if nat_eq(a1253, a1254) {
            let v5: (bool, Adt0) = f41(v0, 3u64)?;
            Ok(({
                let (h1255, _) = v5;
                h1255
            }, f8(f10({
                let (_, h1256) = v5;
                h1256
            }), 8u64)?))
        } else {
            let a1257 = v1;
            let a1258 = 3u64;
            if nat_eq(a1257, a1258) {
                let v6: (bool, Adt0) = f41(v0, 4u64)?;
                Ok(({
                    let (h1259, _) = v6;
                    h1259
                }, f9(f4({
                    let (_, h1260) = v6;
                    h1260
                }))?))
            } else {
                let a1261 = v1;
                let a1262 = 4u64;
                if nat_eq(a1261, a1262) {
                    let v7: (bool, Adt0) = f41(v0, 5u64)?;
                    let v8: (bool, Bytes) = f131(f4({
                        let (_, h1263) = v7;
                        h1263
                    }))?;
                    Ok(({
                        let (h1264, _) = v7;
                        if h1264 {
                            let (h1265, _) = v8;
                            h1265
                        } else {
                            false
                        }
                    }, {
                        let (_, h1266) = v8;
                        h1266
                    }))
                } else {
                    let a1267 = v1;
                    let a1268 = 6u64;
                    if nat_eq(a1267, a1268) {
                        Ok(({
                            let (h1269, _) = v2;
                            h1269
                        }, f181(v3)?))
                    } else {
                        let a1270 = v1;
                        let a1271 = 7u64;
                        if nat_eq(a1270, a1271) {
                            let v9: (bool, List<u64>) = f153(v3, 8u64)?;
                            Ok(({
                                let (h1272, _) = v2;
                                if h1272 {
                                    let (h1273, _) = v9;
                                    h1273
                                } else {
                                    false
                                }
                            }, f156({
                                let (_, h1274) = v9;
                                h1274
                            }, 8u64)?))
                        } else {
                            let a1275 = v1;
                            let a1276 = 8u64;
                            if nat_eq(a1275, a1276) {
                                Ok(({
                                    let (h1277, _) = v2;
                                    h1277
                                }, f120(f16(v3, 9u64))?))
                            } else {
                                let a1278 = v1;
                                let a1279 = 9u64;
                                if nat_eq(a1278, a1279) {
                                    let v10: (bool, Bytes) = f104(f16(v3, 10u64), 0u64)?;
                                    Ok(({
                                        let (h1280, _) = v2;
                                        if h1280 {
                                            let (h1281, _) = v10;
                                            h1281
                                        } else {
                                            false
                                        }
                                    }, {
                                        let (_, h1282) = v10;
                                        h1282
                                    }))
                                } else {
                                    let a1283 = v1;
                                    let a1284 = 11u64;
                                    if nat_eq(a1283, a1284) {
                                        let v11: (bool, Adt0) = f41(v0, 22u64)?;
                                        let v12: (bool, Bytes) = f118(f4({
                                            let (_, h1285) = v11;
                                            h1285
                                        }))?;
                                        Ok(({
                                            let (h1286, _) = v11;
                                            if h1286 {
                                                let (h1287, _) = v12;
                                                h1287
                                            } else {
                                                false
                                            }
                                        }, {
                                            let (_, h1288) = v12;
                                            h1288
                                        }))
                                    } else {
                                        let a1289 = v1;
                                        let a1290 = 12u64;
                                        if nat_eq(a1289, a1290) {
                                            let v13: (bool, Bytes) = f104(f16(v3, 23u64), 1u64)?;
                                            Ok(({
                                                let (h1291, _) = v2;
                                                if h1291 {
                                                    let (h1292, _) = v13;
                                                    h1292
                                                } else {
                                                    false
                                                }
                                            }, {
                                                let (_, h1293) = v13;
                                                h1293
                                            }))
                                        } else {
                                            let a1294 = v1;
                                            let a1295 = 13u64;
                                            if nat_eq(a1294, a1295) {
                                                let v14: (bool, Adt0) = f41(v0, 14u64)?;
                                                let v15: (bool, Bytes) = f118(f4({
                                                    let (_, h1296) = v14;
                                                    h1296
                                                }))?;
                                                Ok(({
                                                    let (h1297, _) = v14;
                                                    if h1297 {
                                                        let (h1298, _) = v15;
                                                        h1298
                                                    } else {
                                                        false
                                                    }
                                                }, {
                                                    let (_, h1299) = v15;
                                                    h1299
                                                }))
                                            } else {
                                                let a1300 = v1;
                                                let a1301 = 14u64;
                                                if nat_eq(a1300, a1301) {
                                                    let v16: (bool, Bytes) = f104(f16(v3, 15u64), 1u64)?;
                                                    Ok(({
                                                        let (h1302, _) = v2;
                                                        if h1302 {
                                                            let (h1303, _) = v16;
                                                            h1303
                                                        } else {
                                                            false
                                                        }
                                                    }, {
                                                        let (_, h1304) = v16;
                                                        h1304
                                                    }))
                                                } else {
                                                    Ok((true, Bytes::lit(&[])))
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

pub fn f162(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1305 = f91(v0, v1);
    let a1306 = f148(v0, v2);
    fixed_u32::bit_or(a1305, a1306)
}

pub fn f163(v0: u32) -> u32 {
    let a1309 = {
        let a1307 = f162(v0, 17u32, 15u32);
        let a1308 = f162(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1307, a1308)
    };
    let a1310 = f91(v0, 10u32);
    fixed_u32::bit_xor(a1309, a1310)
}

pub fn f164(v0: u32) -> u32 {
    let a1313 = {
        let a1311 = f162(v0, 7u32, 25u32);
        let a1312 = f162(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1311, a1312)
    };
    let a1314 = f91(v0, 3u32);
    fixed_u32::bit_xor(a1313, a1314)
}

pub fn f165(v0: Fn32, p1318: bool, p1319: List<Adt6>) -> bool {
    let m1317 = (p1318, p1319);
    {
        let mut m1317 = m1317;
        loop {
            let step = {
                let (v1, v2) = m1317;
                let m1315 = v2;
                match m1315.uncons() {
                    None => {
                        Err::<(bool, List<Adt6>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt6>), bool>(({
                            let c1316: Fn32 = v0.clone();
                            c1316.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: List<u64>, v1: Bytes, v2: bool, v3: Adt6) -> bool {
    if if f182(match v3.clone() {
        Adt6::C0(_, _, _, _, h1320) => {
            h1320
        }
    }, v1) {
        !f129(v0, match v3 {
            Adt6::C0(h1321, _, _, _, _) => {
                h1321
            }
        })
    } else {
        false
    } {
        false
    } else {
        v2
    }
}

pub fn f167(v0: Fn33, p1325: (bool, List<u64>), p1326: List<Adt0>) -> R<(bool, List<u64>)> {
    let m1324 = (p1325, p1326);
    {
        let mut m1324 = m1324;
        loop {
            let step = {
                let (v1, v2) = m1324;
                let m1322 = v2;
                match m1322.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(({
                            let c1323: Fn33 = v0.clone();
                            c1323.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: u64, v1: (bool, List<u64>), v2: Adt0) -> R<(bool, List<u64>)> {
    let a1328 = match v2.clone() {
        Adt0::C0(h1327, _, _, _) => {
            h1327
        }
    };
    let a1329 = v0;
    if nat_eq(a1328, a1329) {
        let a1331 = match v2.clone() {
            Adt0::C0(_, h1330, _, _) => {
                h1330
            }
        };
        let a1332 = 2u64;
        if nat_eq(a1331, a1332) {
            let v3: (bool, List<u64>) = f183(match v2 {
                Adt0::C0(_, _, _, h1333) => {
                    h1333
                }
            })?;
            Ok(({
                let (h1334, _) = v1;
                if h1334 {
                    let (h1335, _) = v3;
                    h1335
                } else {
                    false
                }
            }, f169(Fn34::F184, {
                let (_, h1336) = v1;
                h1336
            }, f169(Fn34::F185, List::<u64>::nil(), {
                let (_, h1337) = v3;
                h1337
            }))))
        } else {
            let a1339 = match v2.clone() {
                Adt0::C0(_, h1338, _, _) => {
                    h1338
                }
            };
            let a1340 = 0u64;
            if nat_eq(a1339, a1340) {
                Ok(({
                    let (h1341, _) = v1;
                    h1341
                }, List::cons(match v2 {
                    Adt0::C0(_, _, h1342, _) => {
                        h1342
                    }
                }, {
                    let (_, h1343) = v1;
                    h1343
                })))
            } else {
                Ok(v1)
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f169(v0: Fn34, p1347: List<u64>, p1348: List<u64>) -> List<u64> {
    let m1346 = (p1347, p1348);
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let (v1, v2) = m1346;
                let m1344 = v2;
                match m1344.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1345: Fn34 = v0.clone();
                            c1345.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f171(v0: Str) -> Bytes {
    let a1349 = v0;
    utf8_encode(a1349)
}

pub fn f172(v0: u64) -> (u64, u64) {
    let a1350 = v0;
    let a1351 = 1u64;
    if if nat_eq(a1350, a1351) {
        true
    } else {
        let a1352 = v0;
        let a1353 = 14u64;
        nat_eq(a1352, a1353)
    } {
        (4u64, 0u64)
    } else {
        let a1354 = v0;
        let a1355 = 11u64;
        if if nat_eq(a1354, a1355) {
            true
        } else {
            let a1356 = v0;
            let a1357 = 15u64;
            nat_eq(a1356, a1357)
        } {
            (10u64, 0u64)
        } else {
            let a1358 = v0;
            let a1359 = 7u64;
            if nat_eq(a1358, a1359) {
                (7u64, 8u64)
            } else {
                let a1360 = v0;
                let a1361 = 13u64;
                if nat_eq(a1360, a1361) {
                    (11u64, 8u64)
                } else {
                    let a1362 = v0;
                    let a1363 = 12u64;
                    if nat_eq(a1362, a1363) {
                        (11u64, 4u64)
                    } else {
                        let a1364 = v0;
                        let a1365 = 6u64;
                        if nat_eq(a1364, a1365) {
                            (5u64, 4u64)
                        } else {
                            let a1366 = v0;
                            let a1367 = 4u64;
                            if if nat_eq(a1366, a1367) {
                                true
                            } else {
                                let a1368 = v0;
                                let a1369 = 5u64;
                                if nat_eq(a1368, a1369) {
                                    true
                                } else {
                                    let a1370 = v0;
                                    let a1371 = 10u64;
                                    if nat_eq(a1370, a1371) {
                                        true
                                    } else {
                                        let a1372 = v0;
                                        let a1373 = 16u64;
                                        nat_eq(a1372, a1373)
                                    }
                                }
                            } {
                                (5u64, 2u64)
                            } else {
                                let a1374 = v0;
                                let a1375 = 8u64;
                                if nat_eq(a1374, a1375) {
                                    (6u64, 0u64)
                                } else {
                                    (5u64, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f173(v0: List<Adt0>, v1: u64) -> R<Bytes> {
    f186(Fn37::F187(v1), Bytes::lit(&[]), v0)
}

pub fn f174(v0: Fn35, p1379: u64, p1380: List<u64>) -> R<u64> {
    let m1378 = (p1379, p1380);
    {
        let mut m1378 = m1378;
        loop {
            let step = {
                let (v1, v2) = m1378;
                let m1376 = v2;
                match m1376.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c1377: Fn35 = v0.clone();
                            c1377.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: u64, _: u64) -> R<u64> {
    let a1381 = v0;
    let a1382 = 1u64;
    nat_add(a1381, a1382)
}

pub fn f176(v0: Fn36, p1386: Bytes, p1387: List<u64>) -> R<Bytes> {
    let m1385 = (p1386, p1387);
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let (v1, v2) = m1385;
                let m1383 = v2;
                match m1383.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1384: Fn36 = v0.clone();
                            c1384.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: u64, v1: Bytes, v2: u64) -> R<Bytes> {
    let a1388 = v1;
    let a1389 = f8(v2, v0)?;
    Ok(append_bytes(a1388, a1389))
}

pub fn f178(v0: List<Adt13>, v1: Bytes) -> Adt12 {
    let m1390 = v0;
    match m1390.uncons() {
        None => {
            Adt12::C0(List::<Adt13>::nil(), v1, true)
        }
        Some((v2, v3)) => {
            Adt12::C0(List::cons(Adt13::C0(match v2.clone() {
                Adt13::C0(h1391, _, _) => {
                    h1391
                }
            }, {
                let a1393 = match v2.clone() {
                    Adt13::C0(_, h1392, _) => {
                        h1392
                    }
                };
                let a1394 = v1;
                append_bytes(a1393, a1394)
            }, match v2 {
                Adt13::C0(_, _, h1395) => {
                    h1395
                }
            }), v3), Bytes::lit(&[]), false)
        }
    }
}

pub fn f179(v0: Adt0) -> R<Bytes> {
    let v1: u64 = match v0.clone() {
        Adt0::C0(_, h1396, _, _) => {
            h1396
        }
    };
    let a1408 = f8(match v0.clone() {
        Adt0::C0(h1397, _, _, _) => {
            h1397
        }
    }, 8u64)?;
    let a1409 = {
        let a1406 = f54(v1);
        let a1407 = {
            let a1398 = v1;
            let a1399 = 0u64;
            if if nat_eq(a1398, a1399) {
                true
            } else {
                let a1400 = v1;
                let a1401 = 1u64;
                nat_eq(a1400, a1401)
            } {
                f8(match v0 {
                    Adt0::C0(_, _, h1402, _) => {
                        h1402
                    }
                }, 8u64)?
            } else {
                let a1403 = v1;
                let a1404 = 5u64;
                if nat_eq(a1403, a1404) {
                    f8(match v0 {
                        Adt0::C0(_, _, h1405, _) => {
                            h1405
                        }
                    }, 4u64)?
                } else {
                    Bytes::lit(&[])
                }
            }
        };
        append_bytes(a1406, a1407)
    };
    Ok(append_bytes(a1408, a1409))
}

pub fn f180(v0: Adt0, v1: List<Adt0>) -> List<Adt0> {
    let v2: Adt14 = f188(Fn38::F189(std::rc::Rc::new(v0.clone())), Adt14::C0(false, List::<Adt0>::nil()), v1);
    if match v2.clone() {
        Adt14::C0(h1410, _) => {
            h1410
        }
    } {
        f12(match v2 {
            Adt14::C0(_, h1411) => {
                h1411
            }
        })
    } else {
        f12(List::cons(v0, match v2 {
            Adt14::C0(_, h1412) => {
                h1412
            }
        }))
    }
}

pub fn f181(v0: List<Adt0>) -> R<Bytes> {
    f186(Fn37::F190, Bytes::lit(&[]), v0)
}

pub fn f182(v0: List<Bytes>, v1: Bytes) -> bool {
    f151(Fn30::F191(v1), false, v0)
}

pub fn f183(v0: Bytes) -> R<(bool, List<u64>)> {
    let v1: Adt15 = {
        let (h1413, _) = f192(Fn39::F193(v0.clone()), {
            let a1415 = {
                let a1414 = v0;
                length_bytes(a1414)
            };
            let a1416 = 1u64;
            nat_add(a1415, a1416)?
        }, Adt15::C0(0u64, true, List::<u64>::nil()))?;
        h1413
    };
    Ok((match v1.clone() {
        Adt15::C0(_, h1417, _) => {
            h1417
        }
    }, match v1 {
        Adt15::C0(_, _, h1418) => {
            h1418
        }
    }))
}

pub fn f184(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f185(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f186(v0: Fn37, p1422: Bytes, p1423: List<Adt0>) -> R<Bytes> {
    let m1421 = (p1422, p1423);
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let (v1, v2) = m1421;
                let m1419 = v2;
                match m1419.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt0>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt0>), R<Bytes>>(({
                            let c1420: Fn37 = v0.clone();
                            c1420.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: u64, v1: Bytes, v2: Adt0) -> R<Bytes> {
    let a1425 = match v2.clone() {
        Adt0::C0(h1424, _, _, _) => {
            h1424
        }
    };
    let a1426 = v0;
    if nat_eq(a1425, a1426) {
        let a1428 = match v2.clone() {
            Adt0::C0(_, h1427, _, _) => {
                h1427
            }
        };
        let a1429 = 2u64;
        if nat_eq(a1428, a1429) {
            let a1431 = v1;
            let a1432 = match v2 {
                Adt0::C0(_, _, _, h1430) => {
                    h1430
                }
            };
            Ok(append_bytes(a1431, a1432))
        } else {
            let a1434 = match v2.clone() {
                Adt0::C0(_, h1433, _, _) => {
                    h1433
                }
            };
            let a1435 = 5u64;
            if nat_eq(a1434, a1435) {
                let a1437 = v1;
                let a1438 = f8(match v2 {
                    Adt0::C0(_, _, h1436, _) => {
                        h1436
                    }
                }, 4u64)?;
                Ok(append_bytes(a1437, a1438))
            } else {
                let a1440 = match v2.clone() {
                    Adt0::C0(_, h1439, _, _) => {
                        h1439
                    }
                };
                let a1441 = 1u64;
                if nat_eq(a1440, a1441) {
                    let a1443 = v1;
                    let a1444 = f8(match v2 {
                        Adt0::C0(_, _, h1442, _) => {
                            h1442
                        }
                    }, 8u64)?;
                    Ok(append_bytes(a1443, a1444))
                } else {
                    Ok(v1)
                }
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f188(v0: Fn38, p1448: Adt14, p1449: List<Adt0>) -> Adt14 {
    let m1447 = (p1448, p1449);
    {
        let mut m1447 = m1447;
        loop {
            let step = {
                let (v1, v2) = m1447;
                let m1445 = v2;
                match m1445.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt0>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt0>), Adt14>(({
                            let c1446: Fn38 = v0.clone();
                            c1446.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt0, v1: Adt14, v2: Adt0) -> Adt14 {
    if match v1.clone() {
        Adt14::C0(h1450, _) => {
            h1450
        }
    } {
        Adt14::C0(true, List::cons(v2, match v1 {
            Adt14::C0(_, h1451) => {
                h1451
            }
        }))
    } else {
        let a1454 = match v0.clone() {
            Adt0::C0(h1452, _, _, _) => {
                h1452
            }
        };
        let a1455 = match v2.clone() {
            Adt0::C0(h1453, _, _, _) => {
                h1453
            }
        };
        if nat_lt(a1454, a1455) {
            Adt14::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt14::C0(_, h1456) => {
                    h1456
                }
            })))
        } else {
            Adt14::C0(false, List::cons(v2, match v1 {
                Adt14::C0(_, h1457) => {
                    h1457
                }
            }))
        }
    }
}

pub fn f190(v0: Bytes, v1: Adt0) -> R<Bytes> {
    let a1459 = match v1.clone() {
        Adt0::C0(h1458, _, _, _) => {
            h1458
        }
    };
    let a1460 = 7u64;
    if nat_eq(a1459, a1460) {
        let a1462 = match v1.clone() {
            Adt0::C0(_, h1461, _, _) => {
                h1461
            }
        };
        let a1463 = 2u64;
        if nat_eq(a1462, a1463) {
            let a1465 = v0;
            let a1466 = f9(match v1 {
                Adt0::C0(_, _, _, h1464) => {
                    h1464
                }
            })?;
            Ok(append_bytes(a1465, a1466))
        } else {
            let a1468 = match v1.clone() {
                Adt0::C0(_, h1467, _, _) => {
                    h1467
                }
            };
            let a1469 = 5u64;
            if nat_eq(a1468, a1469) {
                let a1471 = v0;
                let a1472 = f8(match v1 {
                    Adt0::C0(_, _, h1470, _) => {
                        h1470
                    }
                }, 4u64)?;
                Ok(append_bytes(a1471, a1472))
            } else {
                Ok(v0)
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f191(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1473 = v2;
        let a1474 = v0;
        equal(a1473, a1474)
    }
}

pub fn f192(v0: Fn39, p1479: u64, p1480: Adt15) -> R<(Adt15, bool)> {
    let m1478 = (p1479, p1480);
    {
        let mut m1478 = m1478;
        loop {
            let step = {
                let (v1, v2) = m1478;
                let m1475 = v1;
                if m1475 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1475 - 1;
                    let m1477 = {
                        let c1476: Fn39 = v0.clone();
                        c1476.apply(v2.clone())?
                    };
                    match m1477 {
                        None => {
                            Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt15), R<(Adt15, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: Adt15) -> R<Option<Adt15>> {
    f194(v0, v1)
}

pub fn f194(v0: Bytes, v1: Adt15) -> R<Option<Adt15>> {
    if if match v1.clone() {
        Adt15::C0(_, h1481, _) => {
            h1481
        }
    } {
        let a1484 = match v1.clone() {
            Adt15::C0(h1482, _, _) => {
                h1482
            }
        };
        let a1485 = {
            let a1483 = v0.clone();
            length_bytes(a1483)
        };
        nat_lt(a1484, a1485)
    } else {
        false
    } {
        let v2: (bool, (u64, u64)) = f74(v0, match v1.clone() {
            Adt15::C0(h1486, _, _) => {
                h1486
            }
        })?;
        let (h1487, _) = v2;
        if h1487 {
            Ok(Some(Adt15::C0({
                let (_, h1488) = {
                    let (_, h1489) = v2;
                    h1489
                };
                h1488
            }, true, List::cons({
                let (h1490, _) = {
                    let (_, h1491) = v2;
                    h1491
                };
                h1490
            }, match v1 {
                Adt15::C0(_, _, h1492) => {
                    h1492
                }
            }))))
        } else {
            Ok(Some(Adt15::C0(0u64, false, List::<u64>::nil())))
        }
    } else {
        Ok(None::<Adt15>)
    }
}
