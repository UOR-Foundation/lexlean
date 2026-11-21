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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, List<Adt2>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(std::rc::Rc<Adt0>),
    C1(Bytes, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F5(k0) => f5(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn1::F18(k0) => f18(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
        match self {
            Fn2::F26 => f26(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(Bytes),
    F39(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0),
            Fn3::F39(k0) => f39(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F31(k0) => f31(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F45(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn5::F45(k0) => f45(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Adt0>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let a4 = f1(v0.clone(), {
        let a2 = v1;
        let a3 = 1u64;
        nat_sub(a2, a3)
    })?;
    let a5 = 10u64;
    if nat_eq(a4, a5) {
        f2(f3(v0, 0u64, {
            let a6 = v1;
            let a7 = 1u64;
            nat_sub(a6, a7)
        }))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f1(v0: Bytes, v1: u64) -> R<u64> {
    let m10 = {
        let a8 = v0;
        let a9 = v1;
        index_bytes(a8, a9)
    };
    match m10 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a55 = {
                let a31 = {
                    let a19 = {
                        let a13 = {
                            let a11 = v3;
                            let a12 = 128u8;
                            fixed_u8::bit_and(a11, a12)
                        };
                        let a14 = 128u8;
                        if equal(a13, a14) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a20 = {
                        let a17 = {
                            let a15 = v3;
                            let a16 = 64u8;
                            fixed_u8::bit_and(a15, a16)
                        };
                        let a18 = 64u8;
                        if equal(a17, a18) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a19, a20)?
                };
                let a32 = {
                    let a29 = {
                        let a23 = {
                            let a21 = v3;
                            let a22 = 32u8;
                            fixed_u8::bit_and(a21, a22)
                        };
                        let a24 = 32u8;
                        if equal(a23, a24) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a30 = {
                        let a27 = {
                            let a25 = v3;
                            let a26 = 16u8;
                            fixed_u8::bit_and(a25, a26)
                        };
                        let a28 = 16u8;
                        if equal(a27, a28) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a29, a30)?
                };
                nat_add(a31, a32)?
            };
            let a56 = {
                let a53 = {
                    let a41 = {
                        let a35 = {
                            let a33 = v3;
                            let a34 = 8u8;
                            fixed_u8::bit_and(a33, a34)
                        };
                        let a36 = 8u8;
                        if equal(a35, a36) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a42 = {
                        let a39 = {
                            let a37 = v3;
                            let a38 = 4u8;
                            fixed_u8::bit_and(a37, a38)
                        };
                        let a40 = 4u8;
                        if equal(a39, a40) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a41, a42)?
                };
                let a54 = {
                    let a51 = {
                        let a45 = {
                            let a43 = v3;
                            let a44 = 2u8;
                            fixed_u8::bit_and(a43, a44)
                        };
                        let a46 = 2u8;
                        if equal(a45, a46) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a52 = {
                        let a49 = {
                            let a47 = v3;
                            let a48 = 1u8;
                            fixed_u8::bit_and(a47, a48)
                        };
                        let a50 = 1u8;
                        if equal(a49, a50) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a51, a52)?
                };
                nat_add(a53, a54)?
            };
            nat_add(a55, a56)
        }
    }
}

pub fn f2(v0: Bytes) -> R<Option<Adt0>> {
    let m61 = {
        let (h57, _) = f4(Fn0::F5(v0.clone()), {
            let a59 = {
                let a58 = v0;
                length_bytes(a58)
            };
            let a60 = 3u64;
            nat_add(a59, a60)?
        }, Adt1::C0(0u64, 0u64, List::<Adt2>::nil(), std::rc::Rc::new(Adt0::C5)))?;
        h57
    };
    match m61 {
        Adt1::C0(_, v2, _, r64) => {
            let v4 = (*r64).clone();
            let a62 = v2;
            let a63 = 8u64;
            if nat_eq(a62, a63) {
                Ok(Some(v4))
            } else {
                Ok(None::<Adt0>)
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m68 = {
        let a65 = v0;
        let a66 = v1;
        let a67 = v2;
        slice_bytes(a65, a66, a67)
    };
    match m68 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f4(v0: Fn0, p73: u64, p74: Adt1) -> R<(Adt1, bool)> {
    let m72 = (p73, p74);
    {
        let mut m72 = m72;
        loop {
            let step = {
                let (v1, v2) = m72;
                let m69 = v1;
                if m69 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m69 - 1;
                    let m71 = {
                        let c70: Fn0 = v0.clone();
                        c70.apply(v2.clone())?
                    };
                    match m71 {
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
                Ok(next) => { m72 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    f6(v0, v1)
}

pub fn f6(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    let m75 = v1;
    match m75 {
        Adt1::C0(v2, v3, v4, r78) => {
            let v5 = (*r78).clone();
            let a76 = 8u64;
            let a77 = v3;
            if nat_le(a76, a77) {
                Ok(None::<Adt1>)
            } else {
                Ok(Some(f7(v0, v2, v3, v4, v5)?))
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: Adt0) -> R<Adt1> {
    let v5: u64 = f1(v0.clone(), v1)?;
    let a79 = v2;
    let a80 = 7u64;
    if nat_eq(a79, a80) {
        Ok(Adt1::C0(v1, {
            let a81 = v5;
            let a82 = 256u64;
            if nat_eq(a81, a82) {
                8u64
            } else {
                9u64
            }
        }, v3, std::rc::Rc::new(v4)))
    } else {
        let a83 = v5;
        let a84 = 256u64;
        if nat_eq(a83, a84) {
            Ok(f8(v1))
        } else {
            let a85 = v2;
            let a86 = 0u64;
            if if nat_eq(a85, a86) {
                true
            } else {
                let a87 = v2;
                let a88 = 1u64;
                nat_eq(a87, a88)
            } {
                f9(v0, v1, v2, v3, v5)
            } else {
                let a89 = v2;
                let a90 = 2u64;
                if if nat_eq(a89, a90) {
                    true
                } else {
                    let a91 = v2;
                    let a92 = 3u64;
                    nat_eq(a91, a92)
                } {
                    f10(v0, v1, v2, v3, v5)
                } else {
                    let a93 = v2;
                    let a94 = 4u64;
                    if nat_eq(a93, a94) {
                        let a95 = v5;
                        let a96 = 58u64;
                        if nat_eq(a95, a96) {
                            Ok(Adt1::C0({
                                let a97 = v1;
                                let a98 = 1u64;
                                nat_add(a97, a98)?
                            }, 0u64, v3, std::rc::Rc::new(v4)))
                        } else {
                            Ok(f8(v1))
                        }
                    } else {
                        f11(v0, v1, v3, v5)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> Adt1 {
    Adt1::C0(v0, 9u64, List::<Adt2>::nil(), std::rc::Rc::new(Adt0::C5))
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a99 = v4;
    let a100 = 34u64;
    if nat_eq(a99, a100) {
        let m103 = f12(v0, {
            let a101 = v1;
            let a102 = 1u64;
            nat_add(a101, a102)?
        })?;
        match m103 {
            None => {
                Ok(f8(v1))
            }
            Some(v5) => {
                Ok(f13({
                    let (h104, _) = v5;
                    h104
                }, v3, Adt0::C2({
                    let (_, h105) = v5;
                    h105
                })))
            }
        }
    } else {
        let a106 = v4;
        let a107 = 116u64;
        if nat_eq(a106, a107) {
            let a108 = f3(v0, v1, 4u64);
            let a109 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
            if equal(a108, a109) {
                Ok(f13({
                    let a110 = v1;
                    let a111 = 4u64;
                    nat_add(a110, a111)?
                }, v3, Adt0::C0(true)))
            } else {
                Ok(f8(v1))
            }
        } else {
            let a112 = v4;
            let a113 = 102u64;
            if nat_eq(a112, a113) {
                let a114 = f3(v0, v1, 5u64);
                let a115 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
                if equal(a114, a115) {
                    Ok(f13({
                        let a116 = v1;
                        let a117 = 5u64;
                        nat_add(a116, a117)?
                    }, v3, Adt0::C0(false)))
                } else {
                    Ok(f8(v1))
                }
            } else {
                let a118 = v4;
                let a119 = 91u64;
                if nat_eq(a118, a119) {
                    Ok(Adt1::C0({
                        let a120 = v1;
                        let a121 = 1u64;
                        nat_add(a120, a121)?
                    }, 1u64, List::cons(Adt2::C0(std::rc::Rc::new(Adt0::C5)), v3), std::rc::Rc::new(Adt0::C5)))
                } else {
                    let a122 = v4;
                    let a123 = 123u64;
                    if nat_eq(a122, a123) {
                        Ok(Adt1::C0({
                            let a124 = v1;
                            let a125 = 1u64;
                            nat_add(a124, a125)?
                        }, 3u64, List::cons(Adt2::C1(Bytes::lit(&[]), std::rc::Rc::new(Adt0::C5)), v3), std::rc::Rc::new(Adt0::C5)))
                    } else {
                        let a126 = v4;
                        let a127 = 93u64;
                        if if nat_eq(a126, a127) {
                            let a128 = v2;
                            let a129 = 1u64;
                            nat_eq(a128, a129)
                        } else {
                            false
                        } {
                            f14(v0, v1, v3)
                        } else {
                            let a130 = v4;
                            let a131 = 45u64;
                            if if nat_eq(a130, a131) {
                                true
                            } else {
                                let a132 = 47u64;
                                let a133 = v4;
                                if nat_lt(a132, a133) {
                                    let a134 = v4;
                                    let a135 = 58u64;
                                    nat_lt(a134, a135)
                                } else {
                                    false
                                }
                            } {
                                let m136 = f15(v0, v1)?;
                                match m136 {
                                    None => {
                                        Ok(f8(v1))
                                    }
                                    Some(v6) => {
                                        Ok(f13({
                                            let (h137, _) = v6;
                                            h137
                                        }, v3, {
                                            let (_, h138) = v6;
                                            h138
                                        }))
                                    }
                                }
                            } else {
                                Ok(f8(v1))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a139 = v4;
    let a140 = 34u64;
    if nat_eq(a139, a140) {
        let m143 = f12(v0, {
            let a141 = v1;
            let a142 = 1u64;
            nat_add(a141, a142)?
        })?;
        match m143 {
            None => {
                Ok(f8(v1))
            }
            Some(v5) => {
                let m144 = v3;
                match m144.uncons() {
                    None => {
                        Ok(f8(v1))
                    }
                    Some((v6, v7)) => {
                        let m145 = v6;
                        match m145 {
                            Adt2::C0(_) => {
                                Ok(f8(v1))
                            }
                            Adt2::C1(_, r149) => {
                                let v10 = (*r149).clone();
                                if f16(v10.clone(), {
                                    let (_, h146) = v5.clone();
                                    h146
                                }) {
                                    Ok(Adt1::C0({
                                        let (h147, _) = v5;
                                        h147
                                    }, 4u64, List::cons(Adt2::C1({
                                        let (_, h148) = v5;
                                        h148
                                    }, std::rc::Rc::new(v10)), v7), std::rc::Rc::new(Adt0::C5)))
                                } else {
                                    Ok(f8(v1))
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a150 = v4;
        let a151 = 125u64;
        if if nat_eq(a150, a151) {
            let a152 = v2;
            let a153 = 3u64;
            nat_eq(a152, a153)
        } else {
            false
        } {
            f14(v0, v1, v3)
        } else {
            Ok(f8(v1))
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: List<Adt2>, v3: u64) -> R<Adt1> {
    let m154 = v2.clone();
    match m154.uncons() {
        None => {
            Ok(f8(v1))
        }
        Some((v4, _)) => {
            let m155 = v4;
            match m155 {
                Adt2::C0(_) => {
                    let a156 = v3;
                    let a157 = 44u64;
                    if nat_eq(a156, a157) {
                        Ok(Adt1::C0({
                            let a158 = v1;
                            let a159 = 1u64;
                            nat_add(a158, a159)?
                        }, 0u64, v2, std::rc::Rc::new(Adt0::C5)))
                    } else {
                        let a160 = v3;
                        let a161 = 93u64;
                        if nat_eq(a160, a161) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f8(v1))
                        }
                    }
                }
                Adt2::C1(_, _) => {
                    let a162 = v3;
                    let a163 = 44u64;
                    if nat_eq(a162, a163) {
                        Ok(Adt1::C0({
                            let a164 = v1;
                            let a165 = 1u64;
                            nat_add(a164, a165)?
                        }, 2u64, v2, std::rc::Rc::new(Adt0::C5)))
                    } else {
                        let a166 = v3;
                        let a167 = 125u64;
                        if nat_eq(a166, a167) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f8(v1))
                        }
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f17(Fn1::F18(v0.clone()), {
        let a169 = {
            let a168 = v0;
            length_bytes(a168)
        };
        let a170 = 1u64;
        nat_add(a169, a170)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a174 = {
        let (_, h171) = {
            let (_, h172) = {
                let (h173, _) = v2.clone();
                h173
            };
            h172
        };
        h171
    };
    let a175 = 1u64;
    if nat_eq(a174, a175) {
        Ok(Some(({
            let (h176, _) = {
                let (h177, _) = v2.clone();
                h177
            };
            h176
        }, {
            let (h178, _) = {
                let (_, h179) = {
                    let (h180, _) = v2;
                    h180
                };
                h179
            };
            h178
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f13(v0: u64, v1: List<Adt2>, v2: Adt0) -> Adt1 {
    let m181 = v1;
    match m181.uncons() {
        None => {
            Adt1::C0(v0, 7u64, List::<Adt2>::nil(), std::rc::Rc::new(v2))
        }
        Some((v3, v4)) => {
            let m182 = v3;
            match m182 {
                Adt2::C0(r183) => {
                    let v5 = (*r183).clone();
                    Adt1::C0(v0, 5u64, List::cons(Adt2::C0(std::rc::Rc::new(Adt0::C6(std::rc::Rc::new(v2), std::rc::Rc::new(v5)))), v4), std::rc::Rc::new(Adt0::C5))
                }
                Adt2::C1(v6, r184) => {
                    let v7 = (*r184).clone();
                    Adt1::C0(v0, 5u64, List::cons(Adt2::C1(Bytes::lit(&[]), std::rc::Rc::new(Adt0::C7(v6, std::rc::Rc::new(v2), std::rc::Rc::new(v7)))), v4), std::rc::Rc::new(Adt0::C5))
                }
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: List<Adt2>) -> R<Adt1> {
    let m185 = v2;
    match m185.uncons() {
        None => {
            Ok(f8(v1))
        }
        Some((v3, v4)) => {
            let m186 = v3;
            match m186 {
                Adt2::C0(r197) => {
                    let v5 = (*r197).clone();
                    Ok(f13({
                        let a187 = v1;
                        let a188 = 1u64;
                        nat_add(a187, a188)?
                    }, v4, Adt0::C3(std::rc::Rc::new(f19(v5, {
                        let a190 = {
                            let a189 = v0;
                            length_bytes(a189)
                        };
                        let a191 = 1u64;
                        nat_add(a190, a191)?
                    })))))
                }
                Adt2::C1(_, r198) => {
                    let v7 = (*r198).clone();
                    Ok(f13({
                        let a192 = v1;
                        let a193 = 1u64;
                        nat_add(a192, a193)?
                    }, v4, Adt0::C4(std::rc::Rc::new(f19(v7, {
                        let a195 = {
                            let a194 = v0;
                            length_bytes(a194)
                        };
                        let a196 = 1u64;
                        nat_add(a195, a196)?
                    })))))
                }
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<(u64, Adt0)>> {
    let v2: bool = {
        let a199 = f1(v0.clone(), v1)?;
        let a200 = 45u64;
        nat_eq(a199, a200)
    };
    let v3: u64 = if v2 {
        let a201 = v1;
        let a202 = 1u64;
        nat_add(a201, a202)?
    } else {
        v1
    };
    let v4: u64 = f20(v0.clone(), v3)?;
    let a205 = 19u64;
    let a206 = {
        let a203 = v4;
        let a204 = v3;
        nat_sub(a203, a204)
    };
    if nat_lt(a205, a206) {
        Ok(None::<(u64, Adt0)>)
    } else {
        let m209 = f21(f3(v0, v3, {
            let a207 = v4;
            let a208 = v3;
            nat_sub(a207, a208)
        }))?;
        match m209 {
            None => {
                Ok(None::<(u64, Adt0)>)
            }
            Some(v5) => {
                if f22(v2, v5) {
                    Ok(Some((v4, Adt0::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt0)>)
                }
            }
        }
    }
}

pub fn f16(v0: Adt0, v1: Bytes) -> bool {
    let m210 = v0;
    match m210 {
        Adt0::C0(_) => {
            true
        }
        Adt0::C1(_, _) => {
            true
        }
        Adt0::C2(_) => {
            true
        }
        Adt0::C3(_) => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6(_, _) => {
            true
        }
        Adt0::C7(v10, _, _) => {
            f23(v10, v1)
        }
    }
}

pub fn f17(v0: Fn1, p215: u64, p216: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m214 = (p215, p216);
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, v2) = m214;
                let m211 = v1;
                if m211 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m211 - 1;
                    let m213 = {
                        let c212: Fn1 = v0.clone();
                        c212.apply(v2.clone())?
                    };
                    match m213 {
                        None => {
                            Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f24(v0, v1)
}

pub fn f19(v0: Adt0, v1: u64) -> Adt0 {
    let (_, h217) = {
        let (h218, _) = f25(Fn2::F26, v1, (v0, Adt0::C5));
        h218
    };
    h217
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let (h219, _) = f27(Fn3::F28(v0.clone()), {
        let a221 = {
            let a220 = v0;
            length_bytes(a220)
        };
        let a222 = 1u64;
        nat_add(a221, a222)?
    }, v1)?;
    Ok(h219)
}

pub fn f21(v0: Bytes) -> R<Option<u64>> {
    if f29(v0.clone())? {
        Ok(Some({
            let (_, h223) = {
                let (h224, _) = f30(Fn4::F31(v0.clone()), {
                    let a225 = v0;
                    length_bytes(a225)
                }, (0u64, 0u64))?;
                h224
            };
            h223
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f22(v0: bool, v1: u64) -> bool {
    if v0 {
        let a226 = 0u64;
        let a227 = v1;
        if nat_lt(a226, a227) {
            let a228 = v1;
            let a229 = 9223372036854775808u64;
            nat_le(a228, a229)
        } else {
            false
        }
    } else {
        let a230 = v1;
        let a231 = 9223372036854775807u64;
        nat_le(a230, a231)
    }
}

pub fn f23(v0: Bytes, v1: Bytes) -> bool {
    let a236 = {
        let a232 = v0;
        let a233 = v1;
        compare_bytes(a232, a233)
    };
    let a237 = {
        let a234 = Bytes::lit(&[]);
        let a235 = Bytes::lit(&[0x01]);
        compare_bytes(a234, a235)
    };
    equal(a236, a237)
}

pub fn f24(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h238, _) = v1.clone();
        h238
    };
    let v3: Bytes = {
        let (h239, _) = {
            let (_, h240) = v1.clone();
            h240
        };
        h239
    };
    let a243 = {
        let (_, h241) = {
            let (_, h242) = v1;
            h242
        };
        h241
    };
    let a244 = 0u64;
    if nat_eq(a243, a244) {
        let v4: u64 = f1(v0.clone(), v2)?;
        let a245 = v4;
        let a246 = 34u64;
        if nat_eq(a245, a246) {
            Ok(Some(({
                let a247 = v2;
                let a248 = 1u64;
                nat_add(a247, a248)?
            }, (v3, 1u64))))
        } else {
            let a249 = v4;
            let a250 = 92u64;
            if nat_eq(a249, a250) {
                f32(v0, v2, v3)
            } else {
                let a251 = v4;
                let a252 = 32u64;
                if if nat_lt(a251, a252) {
                    true
                } else {
                    let a253 = v4;
                    let a254 = 256u64;
                    nat_eq(a253, a254)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a255 = v2;
                        let a256 = 1u64;
                        nat_add(a255, a256)?
                    }, ({
                        let a257 = v3;
                        let a258 = f3(v0, v2, 1u64);
                        append_bytes(a257, a258)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f25(v0: Fn2, p263: u64, p264: (Adt0, Adt0)) -> ((Adt0, Adt0), bool) {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v1, v2) = m262;
                let m259 = v1;
                if m259 == 0 {
                    Err::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v2, false))
                } else {
                    let v3 = m259 - 1;
                    let m261 = {
                        let c260: Fn2 = v0.clone();
                        c260.apply(v2.clone())
                    };
                    match m261 {
                        None => {
                            Err::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    f33(v0)
}

pub fn f27(v0: Fn3, p269: u64, p270: u64) -> R<(u64, bool)> {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m265 = v1;
                if m265 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m265 - 1;
                    let m267 = {
                        let c266: Fn3 = v0.clone();
                        c266.apply(v2)?
                    };
                    match m267 {
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
                Ok(next) => { m268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f34(v0, v1)
}

pub fn f29(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a271 = v0.clone();
        length_bytes(a271)
    };
    let a272 = 0u64;
    let a273 = v1;
    if nat_lt(a272, a273) {
        let a274 = f35(v0.clone())?;
        let a275 = v1;
        if nat_eq(a274, a275) {
            let a276 = 1u64;
            let a277 = v1;
            if nat_lt(a276, a277) {
                let m280 = {
                    let a278 = v0;
                    let a279 = 0u64;
                    index_bytes(a278, a279)
                };
                match m280 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a281 = v2;
                        let a282 = 48u8;
                        Ok(!equal(a281, a282))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Fn4, p287: u64, p288: (u64, u64)) -> R<((u64, u64), bool)> {
    let m286 = (p287, p288);
    {
        let mut m286 = m286;
        loop {
            let step = {
                let (v1, v2) = m286;
                let m283 = v1;
                if m283 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m283 - 1;
                    let m285 = {
                        let c284: Fn4 = v0.clone();
                        c284.apply(v2)?
                    };
                    match m285 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f36(v0, v1)
}

pub fn f32(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f1(v0.clone(), {
        let a289 = v1;
        let a290 = 1u64;
        nat_add(a289, a290)?
    })?;
    let v4: Bytes = {
        let a291 = v3;
        let a292 = 117u64;
        if nat_eq(a291, a292) {
            f37(v0, v1)?
        } else {
            f38(v3)
        }
    };
    let a293 = v4.clone();
    let a294 = Bytes::lit(&[]);
    if equal(a293, a294) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a295 = v3;
            let a296 = 117u64;
            if nat_eq(a295, a296) {
                let a297 = v1;
                let a298 = 6u64;
                nat_add(a297, a298)?
            } else {
                let a299 = v1;
                let a300 = 2u64;
                nat_add(a299, a300)?
            }
        }, ({
            let a301 = v2;
            let a302 = v4;
            append_bytes(a301, a302)
        }, 0u64))))
    }
}

pub fn f33(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    let m304 = {
        let (h303, _) = v0;
        h303
    };
    match m304 {
        Adt0::C0(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C1(_, _) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C2(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C3(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C4(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C5 => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C6(r307, r308) => {
            let v7 = (*r307).clone();
            let v8 = (*r308).clone();
            Some((v8, Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new({
                let (_, h305) = v0;
                h305
            }))))
        }
        Adt0::C7(v9, r309, r310) => {
            let v10 = (*r309).clone();
            let v11 = (*r310).clone();
            Some((v11, Adt0::C7(v9, std::rc::Rc::new(v10), std::rc::Rc::new({
                let (_, h306) = v0;
                h306
            }))))
        }
    }
}

pub fn f34(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f1(v0, v1)?;
    let a311 = 47u64;
    let a312 = v2;
    if if nat_lt(a311, a312) {
        let a313 = v2;
        let a314 = 58u64;
        nat_lt(a313, a314)
    } else {
        false
    } {
        Ok(Some({
            let a315 = v1;
            let a316 = 1u64;
            nat_add(a315, a316)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f35(v0: Bytes) -> R<u64> {
    let (h317, _) = f27(Fn3::F39(v0.clone()), {
        let a318 = v0;
        length_bytes(a318)
    }, 0u64)?;
    Ok(h317)
}

pub fn f36(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m322 = {
        let a320 = v0;
        let a321 = {
            let (h319, _) = v1;
            h319
        };
        index_bytes(a320, a321)
    };
    match m322 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a324 = {
                    let (h323, _) = v1;
                    h323
                };
                let a325 = 1u64;
                nat_add(a324, a325)?
            }, {
                let a329 = {
                    let a327 = {
                        let (_, h326) = v1;
                        h326
                    };
                    let a328 = 10u64;
                    nat_mul(a327, a328)?
                };
                let a330 = f40(v2)?;
                nat_add(a329, a330)?
            })))
        }
    }
}

pub fn f37(v0: Bytes, v1: u64) -> R<Bytes> {
    let a333 = f3(v0.clone(), {
        let a331 = v1;
        let a332 = 2u64;
        nat_add(a331, a332)?
    }, 2u64);
    let a334 = Bytes::lit(&[0x30, 0x30]);
    if equal(a333, a334) {
        let m337 = f41(f3(v0, {
            let a335 = v1;
            let a336 = 4u64;
            nat_add(a335, a336)?
        }, 2u64))?;
        match m337 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f1(v2.clone(), 0u64)?;
                let a338 = v3;
                let a339 = 32u64;
                if nat_lt(a338, a339) {
                    if f42(v3) {
                        Ok(Bytes::lit(&[]))
                    } else {
                        Ok(v2)
                    }
                } else {
                    Ok(Bytes::lit(&[]))
                }
            }
        }
    } else {
        Ok(Bytes::lit(&[]))
    }
}

pub fn f38(v0: u64) -> Bytes {
    let a340 = v0;
    let a341 = 34u64;
    if nat_eq(a340, a341) {
        Bytes::lit(&[0x22])
    } else {
        let a342 = v0;
        let a343 = 92u64;
        if nat_eq(a342, a343) {
            Bytes::lit(&[0x5c])
        } else {
            let a344 = v0;
            let a345 = 98u64;
            if nat_eq(a344, a345) {
                Bytes::lit(&[0x08])
            } else {
                let a346 = v0;
                let a347 = 102u64;
                if nat_eq(a346, a347) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a348 = v0;
                    let a349 = 110u64;
                    if nat_eq(a348, a349) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a350 = v0;
                        let a351 = 114u64;
                        if nat_eq(a350, a351) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a352 = v0;
                            let a353 = 116u64;
                            if nat_eq(a352, a353) {
                                Bytes::lit(&[0x09])
                            } else {
                                Bytes::lit(&[])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m356 = {
        let a354 = v0;
        let a355 = v1;
        index_bytes(a354, a355)
    };
    match m356 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f43(v2)? {
                Ok(Some({
                    let a357 = v1;
                    let a358 = 1u64;
                    nat_add(a357, a358)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f40(v0: u8) -> R<u64> {
    let a405 = {
        let v1: u8 = v0;
        let a403 = {
            let a379 = {
                let a367 = {
                    let a361 = {
                        let a359 = v1;
                        let a360 = 128u8;
                        fixed_u8::bit_and(a359, a360)
                    };
                    let a362 = 128u8;
                    if equal(a361, a362) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a368 = {
                    let a365 = {
                        let a363 = v1;
                        let a364 = 64u8;
                        fixed_u8::bit_and(a363, a364)
                    };
                    let a366 = 64u8;
                    if equal(a365, a366) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a367, a368)?
            };
            let a380 = {
                let a377 = {
                    let a371 = {
                        let a369 = v1;
                        let a370 = 32u8;
                        fixed_u8::bit_and(a369, a370)
                    };
                    let a372 = 32u8;
                    if equal(a371, a372) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a378 = {
                    let a375 = {
                        let a373 = v1;
                        let a374 = 16u8;
                        fixed_u8::bit_and(a373, a374)
                    };
                    let a376 = 16u8;
                    if equal(a375, a376) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a377, a378)?
            };
            nat_add(a379, a380)?
        };
        let a404 = {
            let a401 = {
                let a389 = {
                    let a383 = {
                        let a381 = v1;
                        let a382 = 8u8;
                        fixed_u8::bit_and(a381, a382)
                    };
                    let a384 = 8u8;
                    if equal(a383, a384) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a390 = {
                    let a387 = {
                        let a385 = v1;
                        let a386 = 4u8;
                        fixed_u8::bit_and(a385, a386)
                    };
                    let a388 = 4u8;
                    if equal(a387, a388) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a389, a390)?
            };
            let a402 = {
                let a399 = {
                    let a393 = {
                        let a391 = v1;
                        let a392 = 2u8;
                        fixed_u8::bit_and(a391, a392)
                    };
                    let a394 = 2u8;
                    if equal(a393, a394) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a400 = {
                    let a397 = {
                        let a395 = v1;
                        let a396 = 1u8;
                        fixed_u8::bit_and(a395, a396)
                    };
                    let a398 = 1u8;
                    if equal(a397, a398) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a399, a400)?
            };
            nat_add(a401, a402)?
        };
        nat_add(a403, a404)?
    };
    let a406 = 48u64;
    Ok(nat_sub(a405, a406))
}

pub fn f41(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a407 = v0.clone();
        length_bytes(a407)
    };
    let a411 = {
        let a408 = v1;
        let a409 = 2u64;
        let a410 = 0u64;
        nat_rem(a408, a409, a410)
    };
    let a412 = 0u64;
    if nat_eq(a411, a412) {
        let (_, h413) = {
            let (h414, _) = f44(Fn5::F45(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h414
        };
        Ok(h413)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f42(v0: u64) -> bool {
    let a415 = v0;
    let a416 = 8u64;
    if nat_eq(a415, a416) {
        true
    } else {
        let a417 = v0;
        let a418 = 9u64;
        if nat_eq(a417, a418) {
            true
        } else {
            let a419 = v0;
            let a420 = 10u64;
            if nat_eq(a419, a420) {
                true
            } else {
                let a421 = v0;
                let a422 = 12u64;
                if nat_eq(a421, a422) {
                    true
                } else {
                    let a423 = v0;
                    let a424 = 13u64;
                    nat_eq(a423, a424)
                }
            }
        }
    }
}

pub fn f43(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a469 = {
            let a445 = {
                let a433 = {
                    let a427 = {
                        let a425 = v1;
                        let a426 = 128u8;
                        fixed_u8::bit_and(a425, a426)
                    };
                    let a428 = 128u8;
                    if equal(a427, a428) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a434 = {
                    let a431 = {
                        let a429 = v1;
                        let a430 = 64u8;
                        fixed_u8::bit_and(a429, a430)
                    };
                    let a432 = 64u8;
                    if equal(a431, a432) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a433, a434)?
            };
            let a446 = {
                let a443 = {
                    let a437 = {
                        let a435 = v1;
                        let a436 = 32u8;
                        fixed_u8::bit_and(a435, a436)
                    };
                    let a438 = 32u8;
                    if equal(a437, a438) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a444 = {
                    let a441 = {
                        let a439 = v1;
                        let a440 = 16u8;
                        fixed_u8::bit_and(a439, a440)
                    };
                    let a442 = 16u8;
                    if equal(a441, a442) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a443, a444)?
            };
            nat_add(a445, a446)?
        };
        let a470 = {
            let a467 = {
                let a455 = {
                    let a449 = {
                        let a447 = v1;
                        let a448 = 8u8;
                        fixed_u8::bit_and(a447, a448)
                    };
                    let a450 = 8u8;
                    if equal(a449, a450) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a456 = {
                    let a453 = {
                        let a451 = v1;
                        let a452 = 4u8;
                        fixed_u8::bit_and(a451, a452)
                    };
                    let a454 = 4u8;
                    if equal(a453, a454) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a455, a456)?
            };
            let a468 = {
                let a465 = {
                    let a459 = {
                        let a457 = v1;
                        let a458 = 2u8;
                        fixed_u8::bit_and(a457, a458)
                    };
                    let a460 = 2u8;
                    if equal(a459, a460) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a466 = {
                    let a463 = {
                        let a461 = v1;
                        let a462 = 1u8;
                        fixed_u8::bit_and(a461, a462)
                    };
                    let a464 = 1u8;
                    if equal(a463, a464) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a465, a466)?
            };
            nat_add(a467, a468)?
        };
        nat_add(a469, a470)?
    };
    let a471 = 48u64;
    let a472 = v2;
    if nat_le(a471, a472) {
        let a473 = v2;
        let a474 = 57u64;
        Ok(nat_le(a473, a474))
    } else {
        Ok(false)
    }
}

pub fn f44(v0: Fn5, p479: u64, p480: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m478 = (p479, p480);
    {
        let mut m478 = m478;
        loop {
            let step = {
                let (v1, v2) = m478;
                let m475 = v1;
                if m475 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m475 - 1;
                    let m477 = {
                        let c476: Fn5 = v0.clone();
                        c476.apply(v2.clone())?
                    };
                    match m477 {
                        None => {
                            Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f46(v0, v1)
}

pub fn f46(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h481, _) = v1;
        h481
    };
    let m483 = {
        let (_, h482) = v1;
        h482
    };
    match m483 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m486 = {
                let a484 = v0.clone();
                let a485 = v2;
                index_bytes(a484, a485)
            };
            match m486 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m491 = {
                        let a489 = v0;
                        let a490 = {
                            let a487 = v2;
                            let a488 = 1u64;
                            nat_add(a487, a488)?
                        };
                        index_bytes(a489, a490)
                    };
                    match m491 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f47(v4)? {
                                f47(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a492 = v2;
                                    let a493 = 2u64;
                                    nat_add(a492, a493)?
                                }, Some({
                                    let a498 = v3;
                                    let a499 = f48(f49(), {
                                        let a496 = {
                                            let a494 = f50(v4)?;
                                            let a495 = 16u64;
                                            nat_mul(a494, a495)?
                                        };
                                        let a497 = f50(v5)?;
                                        nat_add(a496, a497)?
                                    });
                                    append_bytes(a498, a499)
                                }))))
                            } else {
                                Ok(Some((v2, None::<Bytes>)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f47(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a544 = {
            let a520 = {
                let a508 = {
                    let a502 = {
                        let a500 = v1;
                        let a501 = 128u8;
                        fixed_u8::bit_and(a500, a501)
                    };
                    let a503 = 128u8;
                    if equal(a502, a503) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a509 = {
                    let a506 = {
                        let a504 = v1;
                        let a505 = 64u8;
                        fixed_u8::bit_and(a504, a505)
                    };
                    let a507 = 64u8;
                    if equal(a506, a507) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a508, a509)?
            };
            let a521 = {
                let a518 = {
                    let a512 = {
                        let a510 = v1;
                        let a511 = 32u8;
                        fixed_u8::bit_and(a510, a511)
                    };
                    let a513 = 32u8;
                    if equal(a512, a513) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a519 = {
                    let a516 = {
                        let a514 = v1;
                        let a515 = 16u8;
                        fixed_u8::bit_and(a514, a515)
                    };
                    let a517 = 16u8;
                    if equal(a516, a517) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a518, a519)?
            };
            nat_add(a520, a521)?
        };
        let a545 = {
            let a542 = {
                let a530 = {
                    let a524 = {
                        let a522 = v1;
                        let a523 = 8u8;
                        fixed_u8::bit_and(a522, a523)
                    };
                    let a525 = 8u8;
                    if equal(a524, a525) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a531 = {
                    let a528 = {
                        let a526 = v1;
                        let a527 = 4u8;
                        fixed_u8::bit_and(a526, a527)
                    };
                    let a529 = 4u8;
                    if equal(a528, a529) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a530, a531)?
            };
            let a543 = {
                let a540 = {
                    let a534 = {
                        let a532 = v1;
                        let a533 = 2u8;
                        fixed_u8::bit_and(a532, a533)
                    };
                    let a535 = 2u8;
                    if equal(a534, a535) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a541 = {
                    let a538 = {
                        let a536 = v1;
                        let a537 = 1u8;
                        fixed_u8::bit_and(a536, a537)
                    };
                    let a539 = 1u8;
                    if equal(a538, a539) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a540, a541)?
            };
            nat_add(a542, a543)?
        };
        nat_add(a544, a545)?
    };
    let a546 = 48u64;
    let a547 = v2;
    if if nat_le(a546, a547) {
        let a548 = v2;
        let a549 = 57u64;
        nat_le(a548, a549)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a550 = 97u64;
        let a551 = v2;
        if nat_le(a550, a551) {
            let a552 = v2;
            let a553 = 102u64;
            Ok(nat_le(a552, a553))
        } else {
            Ok(false)
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> Bytes {
    let m557 = {
        let a554 = v0;
        let a555 = v1;
        let a556 = 1u64;
        slice_bytes(a554, a555, a556)
    };
    match m557 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f49() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f50(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a602 = {
            let a578 = {
                let a566 = {
                    let a560 = {
                        let a558 = v1;
                        let a559 = 128u8;
                        fixed_u8::bit_and(a558, a559)
                    };
                    let a561 = 128u8;
                    if equal(a560, a561) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a567 = {
                    let a564 = {
                        let a562 = v1;
                        let a563 = 64u8;
                        fixed_u8::bit_and(a562, a563)
                    };
                    let a565 = 64u8;
                    if equal(a564, a565) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a566, a567)?
            };
            let a579 = {
                let a576 = {
                    let a570 = {
                        let a568 = v1;
                        let a569 = 32u8;
                        fixed_u8::bit_and(a568, a569)
                    };
                    let a571 = 32u8;
                    if equal(a570, a571) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a577 = {
                    let a574 = {
                        let a572 = v1;
                        let a573 = 16u8;
                        fixed_u8::bit_and(a572, a573)
                    };
                    let a575 = 16u8;
                    if equal(a574, a575) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a576, a577)?
            };
            nat_add(a578, a579)?
        };
        let a603 = {
            let a600 = {
                let a588 = {
                    let a582 = {
                        let a580 = v1;
                        let a581 = 8u8;
                        fixed_u8::bit_and(a580, a581)
                    };
                    let a583 = 8u8;
                    if equal(a582, a583) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a589 = {
                    let a586 = {
                        let a584 = v1;
                        let a585 = 4u8;
                        fixed_u8::bit_and(a584, a585)
                    };
                    let a587 = 4u8;
                    if equal(a586, a587) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a588, a589)?
            };
            let a601 = {
                let a598 = {
                    let a592 = {
                        let a590 = v1;
                        let a591 = 2u8;
                        fixed_u8::bit_and(a590, a591)
                    };
                    let a593 = 2u8;
                    if equal(a592, a593) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a599 = {
                    let a596 = {
                        let a594 = v1;
                        let a595 = 1u8;
                        fixed_u8::bit_and(a594, a595)
                    };
                    let a597 = 1u8;
                    if equal(a596, a597) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a598, a599)?
            };
            nat_add(a600, a601)?
        };
        nat_add(a602, a603)?
    };
    let a604 = v2;
    let a605 = 57u64;
    if nat_le(a604, a605) {
        let a606 = v2;
        let a607 = 48u64;
        Ok(nat_sub(a606, a607))
    } else {
        let a608 = v2;
        let a609 = 87u64;
        Ok(nat_sub(a608, a609))
    }
}
