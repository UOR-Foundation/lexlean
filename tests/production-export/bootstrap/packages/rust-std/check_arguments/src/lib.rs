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
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F42(Bytes),
    F51(Bytes),
    F59(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F42(k0) => f42(k0.clone(), p0),
            Fn0::F51(k0) => f51(k0.clone(), p0),
            Fn0::F59(k0) => f59(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F45(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn1::F45(k0) => f45(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: u64, v7: Adt3) -> R<Result<(), Bytes>> {
    let v8: Adt7 = Adt7::C0(v0, std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5));
    let m1 = f1(v8.clone(), v6);
    match m1 {
        None => {
            Ok(Err::<(), Bytes>(f2(v6)?))
        }
        Some(v9) => {
            let a2 = f3(v7.clone())?;
            let a3 = f4(v9.clone())?;
            if nat_eq(a2, a3) {
                f5(v8, v7, v9, 2u64)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("function ")), f6(f8(v6)?, f6(f7(Str::lit(" takes ")), f6(f8(f4(v9)?)?, f6(f7(Str::lit(" argument(s), received ")), f8(f3(v7)?)?)))))))
            }
        }
    }
}

pub fn f1(v0: Adt7, v1: u64) -> Option<Adt0> {
    f9(match v0 {
        Adt7::C0(_, _, _, h4, _, _) => {
            (*h4).clone()
        }
    }, v1)
}

pub fn f2(v0: u64) -> R<Bytes> {
    Ok(f6(f7(Str::lit("function ")), f6(f8(v0)?, f7(Str::lit(" is not declared")))))
}

pub fn f3(v0: Adt3) -> R<u64> {
    let m5 = v0;
    match m5 {
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
        Adt3::C24(_, r8) => {
            let v26 = (*r8).clone();
            let v27: u64 = f3(v26)?;
            let a6 = 1u64;
            let a7 = v27;
            nat_add(a6, a7)
        }
    }
}

pub fn f4(v0: Adt0) -> R<u64> {
    let m9 = v0;
    match m9 {
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
        Adt0::C15(_, r12) => {
            let v12 = (*r12).clone();
            let v13: u64 = f4(v12)?;
            let a10 = 1u64;
            let a11 = v13;
            nat_add(a10, a11)
        }
    }
}

pub fn f5(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m13 = v1;
    match m13 {
        Adt3::C0 => {
            let a14 = f10(v2.clone());
            let a15 = 0u64;
            if nat_eq(a14, a15) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C0)?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C1(v4) => {
            let a16 = f10(v2.clone());
            let a17 = 1u64;
            if nat_eq(a16, a17) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C1(v4))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C2(v5) => {
            let a18 = f10(v2.clone());
            let a19 = 2u64;
            if nat_eq(a18, a19) {
                f13(v5)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C2(v5))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C3(v6) => {
            let a20 = f10(v2.clone());
            let a21 = 3u64;
            if nat_eq(a20, a21) {
                f14(v6, Adt1::C7)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C3(v6))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C4(v7) => {
            let a22 = f10(v2.clone());
            let a23 = 4u64;
            if if nat_eq(a22, a23) {
                let a24 = f15(f16(v2.clone()));
                let a25 = 0u64;
                nat_eq(a24, a25)
            } else {
                false
            } {
                f14(v7, Adt1::C0)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C4(v7))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C5(v8) => {
            let a26 = f10(v2.clone());
            let a27 = 4u64;
            if if nat_eq(a26, a27) {
                let a28 = f15(f16(v2.clone()));
                let a29 = 1u64;
                nat_eq(a28, a29)
            } else {
                false
            } {
                f14(v8, Adt1::C1)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C5(v8))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C6(v9) => {
            let a30 = f10(v2.clone());
            let a31 = 4u64;
            if if nat_eq(a30, a31) {
                let a32 = f15(f16(v2.clone()));
                let a33 = 2u64;
                nat_eq(a32, a33)
            } else {
                false
            } {
                f14(v9, Adt1::C2)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C6(v9))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C7(v10) => {
            let a34 = f10(v2.clone());
            let a35 = 4u64;
            if if nat_eq(a34, a35) {
                let a36 = f15(f16(v2.clone()));
                let a37 = 3u64;
                nat_eq(a36, a37)
            } else {
                false
            } {
                f14(v10, Adt1::C3)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C7(v10))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C8(v11) => {
            let a38 = f10(v2.clone());
            let a39 = 4u64;
            if if nat_eq(a38, a39) {
                let a40 = f15(f16(v2.clone()));
                let a41 = 4u64;
                nat_eq(a40, a41)
            } else {
                false
            } {
                f14(v11, Adt1::C4)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C8(v11))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C9(v12) => {
            let a42 = f10(v2.clone());
            let a43 = 4u64;
            if if nat_eq(a42, a43) {
                let a44 = f15(f16(v2.clone()));
                let a45 = 5u64;
                nat_eq(a44, a45)
            } else {
                false
            } {
                f14(v12, Adt1::C5)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C9(v12))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C10(v13) => {
            let a46 = f10(v2.clone());
            let a47 = 4u64;
            if if nat_eq(a46, a47) {
                let a48 = f15(f16(v2.clone()));
                let a49 = 6u64;
                nat_eq(a48, a49)
            } else {
                false
            } {
                f14(v13, Adt1::C6)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C10(v13))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C11(v14) => {
            let a50 = f10(v2.clone());
            let a51 = 4u64;
            if if nat_eq(a50, a51) {
                let a52 = f15(f16(v2.clone()));
                let a53 = 7u64;
                nat_eq(a52, a53)
            } else {
                false
            } {
                f14(v14, Adt1::C7)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C11(v14))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C12(v15) => {
            let a54 = f10(v2.clone());
            let a55 = 5u64;
            if nat_eq(a54, a55) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C12(v15))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C13(v16) => {
            let a56 = f10(v2.clone());
            let a57 = 6u64;
            if nat_eq(a56, a57) {
                f17(v16)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C13(v16))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C14(v17) => {
            let a58 = f10(v2.clone());
            let a59 = 7u64;
            if nat_eq(a58, a59) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C14(v17))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C15 => {
            let a60 = f10(v2.clone());
            let a61 = 8u64;
            if nat_eq(a60, a61) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C15)?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C16(r87) => {
            let v18 = (*r87).clone();
            let a62 = f10(v2.clone());
            let a63 = 8u64;
            if nat_eq(a62, a63) {
                f5(v0, v18, f18(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C16(std::rc::Rc::new(v18)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C17(r88) => {
            let v19 = (*r88).clone();
            let a64 = f10(v2.clone());
            let a65 = 9u64;
            if nat_eq(a64, a65) {
                f5(v0, v19, f18(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C17(std::rc::Rc::new(v19)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C18(r89) => {
            let v20 = (*r89).clone();
            let a66 = f10(v2.clone());
            let a67 = 9u64;
            if nat_eq(a66, a67) {
                f5(v0, v20, f19(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C18(std::rc::Rc::new(v20)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C19(r90) => {
            let v21 = (*r90).clone();
            let a68 = f10(v2.clone());
            let a69 = 10u64;
            if nat_eq(a68, a69) {
                f5(v0, v21, f18(v2), 1u64)
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C19(std::rc::Rc::new(v21)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C20(r91, r92) => {
            let v22 = (*r91).clone();
            let v23 = (*r92).clone();
            let v24: Result<(), Bytes> = f5(v0.clone(), v22.clone(), f18(v2.clone()), 0u64)?;
            let v25: Result<(), Bytes> = f5(v0, v23.clone(), f19(v2.clone()), 0u64)?;
            let a70 = f10(v2.clone());
            let a71 = 11u64;
            if nat_eq(a70, a71) {
                let m72 = v24;
                match m72 {
                    Ok(_) => {
                        Ok(v25)
                    }
                    Err(v27) => {
                        Ok(Err::<(), Bytes>(v27))
                    }
                }
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C20(std::rc::Rc::new(v22), std::rc::Rc::new(v23)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C21(v28, r93) => {
            let v29 = (*r93).clone();
            let a73 = f10(v2.clone());
            let a74 = 12u64;
            if nat_eq(a73, a74) {
                let m75 = f20(v0.clone(), f21(v2.clone()), v28)?;
                match m75 {
                    Ok(v31) => {
                        let v32: Result<(), Bytes> = f5(v0, v29.clone(), v31.clone(), 2u64)?;
                        let a76 = f3(v29.clone())?;
                        let a77 = f4(v31.clone())?;
                        if nat_eq(a76, a77) {
                            Ok(v32)
                        } else {
                            Ok(Err::<(), Bytes>(f6(f7(Str::lit("constructor ")), f6(f8(v28)?, f6(f7(Str::lit(" of ADT ")), f6(f8(f21(v2))?, f6(f7(Str::lit(" takes ")), f6(f8(f4(v31)?)?, f6(f7(Str::lit(" fields, the literal has ")), f8(f3(v29)?)?)))))))))
                        }
                    }
                    Err(v30) => {
                        Ok(Err::<(), Bytes>(v30))
                    }
                }
            } else {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("the literal ")), f6(f11(Adt3::C21(v28, std::rc::Rc::new(v29)))?, f6(f7(Str::lit(" does not have type ")), f12(v2)?)))))
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f7(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r94, r95) => {
            let v35 = (*r94).clone();
            let v36 = (*r95).clone();
            let v37: Adt0 = {
                let a78 = v3;
                let a79 = 2u64;
                if nat_eq(a78, a79) {
                    f18(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v38: Adt0 = {
                let a80 = v3;
                let a81 = 2u64;
                if nat_eq(a80, a81) {
                    f19(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v39: Result<(), Bytes> = f5(v0.clone(), v35, v37, 0u64)?;
            let v40: Result<(), Bytes> = f5(v0, v36, v38, v3)?;
            let a82 = v3;
            let a83 = 0u64;
            if nat_eq(a82, a83) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a84 = v3;
                let a85 = 2u64;
                if if nat_eq(a84, a85) {
                    f22(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m86 = v39;
                    match m86 {
                        Ok(_) => {
                            Ok(v40)
                        }
                        Err(v42) => {
                            Ok(Err::<(), Bytes>(v42))
                        }
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    let a96 = v0;
    let a97 = v1;
    append_bytes(a96, a97)
}

pub fn f7(v0: Str) -> Bytes {
    let a98 = v0;
    utf8_encode(a98)
}

pub fn f8(v0: u64) -> R<Bytes> {
    let v1: Bytes = f7(Str::lit("0"));
    let v2: Bytes = f23({
        let a135 = f24(v0, 10000000000000000000u64);
        let a136 = {
            let a133 = f24(v0, 1000000000000000000u64);
            let a134 = {
                let a131 = f24(v0, 100000000000000000u64);
                let a132 = {
                    let a129 = f24(v0, 10000000000000000u64);
                    let a130 = {
                        let a127 = f24(v0, 1000000000000000u64);
                        let a128 = {
                            let a125 = f24(v0, 100000000000000u64);
                            let a126 = {
                                let a123 = f24(v0, 10000000000000u64);
                                let a124 = {
                                    let a121 = f24(v0, 1000000000000u64);
                                    let a122 = {
                                        let a119 = f24(v0, 100000000000u64);
                                        let a120 = {
                                            let a117 = f24(v0, 10000000000u64);
                                            let a118 = {
                                                let a115 = f24(v0, 1000000000u64);
                                                let a116 = {
                                                    let a113 = f24(v0, 100000000u64);
                                                    let a114 = {
                                                        let a111 = f24(v0, 10000000u64);
                                                        let a112 = {
                                                            let a109 = f24(v0, 1000000u64);
                                                            let a110 = {
                                                                let a107 = f24(v0, 100000u64);
                                                                let a108 = {
                                                                    let a105 = f24(v0, 10000u64);
                                                                    let a106 = {
                                                                        let a103 = f24(v0, 1000u64);
                                                                        let a104 = {
                                                                            let a101 = f24(v0, 100u64);
                                                                            let a102 = {
                                                                                let a99 = f24(v0, 10u64);
                                                                                let a100 = f24(v0, 1u64);
                                                                                append_bytes(a99, a100)
                                                                            };
                                                                            append_bytes(a101, a102)
                                                                        };
                                                                        append_bytes(a103, a104)
                                                                    };
                                                                    append_bytes(a105, a106)
                                                                };
                                                                append_bytes(a107, a108)
                                                            };
                                                            append_bytes(a109, a110)
                                                        };
                                                        append_bytes(a111, a112)
                                                    };
                                                    append_bytes(a113, a114)
                                                };
                                                append_bytes(a115, a116)
                                            };
                                            append_bytes(a117, a118)
                                        };
                                        append_bytes(a119, a120)
                                    };
                                    append_bytes(a121, a122)
                                };
                                append_bytes(a123, a124)
                            };
                            append_bytes(a125, a126)
                        };
                        append_bytes(a127, a128)
                    };
                    append_bytes(a129, a130)
                };
                append_bytes(a131, a132)
            };
            append_bytes(a133, a134)
        };
        append_bytes(a135, a136)
    })?;
    let a137 = v0;
    let a138 = 0u64;
    if nat_eq(a137, a138) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f9(p144: Adt0, p145: u64) -> Option<Adt0> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m139 = v0;
                match m139 {
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
                    Adt0::C15(r141, r142) => {
                        let v12 = (*r141).clone();
                        let v13 = (*r142).clone();
                        let m140 = v1;
                        if m140 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m140 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt0) -> u64 {
    let m146 = v0;
    match m146 {
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

pub fn f11(v0: Adt3) -> R<Bytes> {
    let m147 = v0;
    match m147 {
        Adt3::C0 => {
            Ok(f7(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f6(f7(Str::lit("Bool { value: ")), f6(f25(v1), f7(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f6(f7(Str::lit("Nat { value: ")), f6(f26(v2)?, f7(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f6(f7(Str::lit("Int { value: ")), f6(f26(v3)?, f7(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f6(f7(Str::lit("U8 { value: ")), f6(f26(v4)?, f7(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f6(f7(Str::lit("U16 { value: ")), f6(f26(v5)?, f7(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f6(f7(Str::lit("U32 { value: ")), f6(f26(v6)?, f7(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f6(f7(Str::lit("U64 { value: ")), f6(f26(v7)?, f7(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f6(f7(Str::lit("I8 { value: ")), f6(f26(v8)?, f7(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f6(f7(Str::lit("I16 { value: ")), f6(f26(v9)?, f7(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f6(f7(Str::lit("I32 { value: ")), f6(f26(v10)?, f7(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f6(f7(Str::lit("I64 { value: ")), f6(f26(v11)?, f7(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f6(f7(Str::lit("String { value: ")), f6(f26(v12)?, f7(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f6(f7(Str::lit("Bytes { hex: ")), f6(f26(v13)?, f7(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f6(f7(Str::lit("Ordering { value: ")), f6(f27(v14), f7(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f7(Str::lit("None")))
        }
        Adt3::C16(r148) => {
            let v15 = (*r148).clone();
            Ok(f6(f7(Str::lit("Some { value: ")), f6(f11(v15)?, f7(Str::lit(" }")))))
        }
        Adt3::C17(r149) => {
            let v16 = (*r149).clone();
            Ok(f6(f7(Str::lit("Ok { value: ")), f6(f11(v16)?, f7(Str::lit(" }")))))
        }
        Adt3::C18(r150) => {
            let v17 = (*r150).clone();
            Ok(f6(f7(Str::lit("Error { value: ")), f6(f11(v17)?, f7(Str::lit(" }")))))
        }
        Adt3::C19(r151) => {
            let v18 = (*r151).clone();
            Ok(f6(f7(Str::lit("List { items: [")), f6(f11(v18)?, f7(Str::lit("] }")))))
        }
        Adt3::C20(r152, r153) => {
            let v19 = (*r152).clone();
            let v20 = (*r153).clone();
            Ok(f6(f7(Str::lit("Pair { left: ")), f6(f11(v19)?, f6(f7(Str::lit(", right: ")), f6(f11(v20)?, f7(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r154) => {
            let v22 = (*r154).clone();
            Ok(f6(f7(Str::lit("Adt { constructor: ")), f6(f8(v21)?, f6(f7(Str::lit(", fields: [")), f6(f11(v22)?, f7(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r155) => {
            let v24 = (*r155).clone();
            Ok(f6(f7(Str::lit("Closure { function: ")), f6(f8(v23)?, f6(f7(Str::lit(", captures: [")), f6(f11(v24)?, f7(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f7(Str::lit("")))
        }
        Adt3::C24(r156, r157) => {
            let v25 = (*r156).clone();
            let v26 = (*r157).clone();
            let v27: Bytes = f11(v25)?;
            let v28: Bytes = f11(v26.clone())?;
            if f28(v26) {
                Ok(v27)
            } else {
                Ok(f6(v27, f6(f7(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f12(v0: Adt0) -> R<Bytes> {
    let m158 = v0;
    match m158 {
        Adt0::C0 => {
            Ok(f7(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f7(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f7(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f7(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f6(f7(Str::lit("Fixed { width: ")), f6(f29(v1), f7(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f7(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f7(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f7(Str::lit("Ordering")))
        }
        Adt0::C8(r159) => {
            let v2 = (*r159).clone();
            Ok(f6(f7(Str::lit("Option { value: ")), f6(f12(v2)?, f7(Str::lit(" }")))))
        }
        Adt0::C9(r160, r161) => {
            let v3 = (*r160).clone();
            let v4 = (*r161).clone();
            Ok(f6(f7(Str::lit("Result { ok: ")), f6(f12(v3)?, f6(f7(Str::lit(", error: ")), f6(f12(v4)?, f7(Str::lit(" }")))))))
        }
        Adt0::C10(r162) => {
            let v5 = (*r162).clone();
            Ok(f6(f7(Str::lit("List { element: ")), f6(f12(v5)?, f7(Str::lit(" }")))))
        }
        Adt0::C11(r163, r164) => {
            let v6 = (*r163).clone();
            let v7 = (*r164).clone();
            Ok(f6(f7(Str::lit("Pair { left: ")), f6(f12(v6)?, f6(f7(Str::lit(", right: ")), f6(f12(v7)?, f7(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f6(f7(Str::lit("Adt { index: ")), f6(f8(v8)?, f7(Str::lit(" }")))))
        }
        Adt0::C13(r165, r166) => {
            let v9 = (*r165).clone();
            let v10 = (*r166).clone();
            Ok(f6(f7(Str::lit("Fn { parameters: [")), f6(f12(v9)?, f6(f7(Str::lit("], result: ")), f6(f12(v10)?, f7(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f7(Str::lit("")))
        }
        Adt0::C15(r167, r168) => {
            let v11 = (*r167).clone();
            let v12 = (*r168).clone();
            let v13: Bytes = f12(v11)?;
            let v14: Bytes = f12(v12.clone())?;
            if f22(v12) {
                Ok(v13)
            } else {
                Ok(f6(v13, f6(f7(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f13(v0: Bytes) -> R<Result<(), Bytes>> {
    let m169 = f30(v0.clone())?;
    match m169 {
        Ok(v1) => {
            let v2: bool = f31(match v1.clone() {
                Adt8::C0(_, h170) => {
                    h170
                }
            }, f7(Str::lit("18446744073709551615")));
            if match v1 {
                Adt8::C0(h171, _) => {
                    h171
                }
            } {
                Ok(Err::<(), Bytes>(f6(f7(Str::lit("nat literal ")), f6(v0, f7(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f6(f7(Str::lit("nat literal ")), f6(v0, f7(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f14(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m172 = f30(v0.clone())?;
    match m172 {
        Ok(v2) => {
            let v3: Adt8 = f32(v1.clone());
            let v4: Adt8 = f33(v1.clone());
            let v5: bool = f34(v3, v2.clone());
            let v6: bool = f34(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f6(v0, f6(f7(Str::lit(" is outside ")), f35(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f6(v0, f6(f7(Str::lit(" is outside ")), f35(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f15(v0: Adt1) -> u64 {
    let m173 = v0;
    match m173 {
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

pub fn f16(v0: Adt0) -> Adt1 {
    let m174 = v0;
    match m174 {
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

pub fn f17(v0: Bytes) -> R<Result<(), Bytes>> {
    let a179 = {
        let a176 = {
            let a175 = v0.clone();
            length_bytes(a175)
        };
        let a177 = 2u64;
        let a178 = 0u64;
        nat_rem(a176, a177, a178)
    };
    let a180 = 0u64;
    if nat_eq(a179, a180) {
        if f36(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f6(f7(Str::lit("byte literal `")), f6(v0, f7(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f6(f7(Str::lit("byte literal `")), f6(v0, f7(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f18(v0: Adt0) -> Adt0 {
    let m181 = f37(v0);
    match m181 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f19(v0: Adt0) -> Adt0 {
    let m182 = f38(v0);
    match m182 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f20(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m183 = f39(v0, v1);
    match m183 {
        None => {
            Ok(Err::<Adt0, Bytes>(f40(v1)?))
        }
        Some(v3) => {
            let m184 = f9(v3, v2);
            match m184 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f6(f7(Str::lit("ADT ")), f6(f8(v1)?, f6(f7(Str::lit(" has no constructor ")), f8(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f21(v0: Adt0) -> u64 {
    let m185 = v0;
    match m185 {
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

pub fn f22(v0: Adt0) -> bool {
    let a186 = f10(v0);
    let a187 = 14u64;
    nat_eq(a186, a187)
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h188, _) = f41(Fn0::F42(v0.clone()), {
            let a189 = v0.clone();
            length_bytes(a189)
        }, 0u64)?;
        h188
    };
    Ok(f43(v0.clone(), v1, {
        let a191 = {
            let a190 = v0;
            length_bytes(a190)
        };
        let a192 = v1;
        nat_sub(a191, a192)
    }))
}

pub fn f24(v0: u64, v1: u64) -> Bytes {
    f43(f7(Str::lit("0123456789")), {
        let a196 = {
            let a193 = v0;
            let a194 = v1;
            let a195 = 0u64;
            nat_quot(a193, a194, a195)
        };
        let a197 = 10u64;
        let a198 = 0u64;
        nat_rem(a196, a197, a198)
    }, 1u64)
}

pub fn f25(v0: bool) -> Bytes {
    if v0 {
        f7(Str::lit("true"))
    } else {
        f7(Str::lit("false"))
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    Ok(f6(f7(Str::lit("\"")), f6({
        let (_, h199) = {
            let (h200, _) = f44(Fn1::F45(v0.clone()), {
                let a201 = v0;
                length_bytes(a201)
            }, (0u64, Bytes::lit(&[])))?;
            h200
        };
        h199
    }, f7(Str::lit("\"")))))
}

pub fn f27(v0: Adt4) -> Bytes {
    let m202 = v0;
    match m202 {
        Adt4::C0 => {
            f7(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f7(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f7(Str::lit("Gt"))
        }
    }
}

pub fn f28(v0: Adt3) -> bool {
    let m203 = v0;
    match m203 {
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

pub fn f29(v0: Adt1) -> Bytes {
    let m204 = v0;
    match m204 {
        Adt1::C0 => {
            f7(Str::lit("U8"))
        }
        Adt1::C1 => {
            f7(Str::lit("U16"))
        }
        Adt1::C2 => {
            f7(Str::lit("U32"))
        }
        Adt1::C3 => {
            f7(Str::lit("U64"))
        }
        Adt1::C4 => {
            f7(Str::lit("I8"))
        }
        Adt1::C5 => {
            f7(Str::lit("I16"))
        }
        Adt1::C6 => {
            f7(Str::lit("I32"))
        }
        Adt1::C7 => {
            f7(Str::lit("I64"))
        }
    }
}

pub fn f30(v0: Bytes) -> R<Result<Adt8, Bytes>> {
    let v1: bool = f46(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f43(v0.clone(), 1u64, {
            let a206 = {
                let a205 = v0.clone();
                length_bytes(a205)
            };
            let a207 = 1u64;
            nat_sub(a206, a207)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f47(v3.clone(), v0.clone())?;
    let v5: bool = f31(v3.clone(), f7(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f31(v3.clone(), f7(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt8, Bytes>(Adt8::C0(v1, v3)))
        } else {
            Ok(Err::<Adt8, Bytes>(f6(f48(v0), f7(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt8, Bytes>(f6(f48(v0), f7(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f31(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a210 = {
            let a208 = v0.clone();
            length_bytes(a208)
        };
        let a211 = {
            let a209 = v1.clone();
            length_bytes(a209)
        };
        nat_lt(a210, a211)
    };
    let v3: bool = {
        let a214 = {
            let a212 = v0.clone();
            length_bytes(a212)
        };
        let a215 = {
            let a213 = v1.clone();
            length_bytes(a213)
        };
        nat_eq(a214, a215)
    };
    let v4: bool = f49(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a216 = v4;
            bool_not(a216)
        } else {
            false
        }
    }
}

pub fn f32(v0: Adt1) -> Adt8 {
    let m217 = v0;
    match m217 {
        Adt1::C0 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f50(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f50(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f50(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f50(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f33(v0: Adt1) -> Adt8 {
    let m218 = v0;
    match m218 {
        Adt1::C0 => {
            f50(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f50(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f50(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f50(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f50(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f50(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f50(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f50(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f34(v0: Adt8, v1: Adt8) -> bool {
    let v2: bool = f31(match v1.clone() {
        Adt8::C0(_, h219) => {
            h219
        }
    }, match v0.clone() {
        Adt8::C0(_, h220) => {
            h220
        }
    });
    let v3: bool = f31(match v0.clone() {
        Adt8::C0(_, h221) => {
            h221
        }
    }, match v1.clone() {
        Adt8::C0(_, h222) => {
            h222
        }
    });
    if match v0 {
        Adt8::C0(h223, _) => {
            h223
        }
    } {
        if match v1 {
            Adt8::C0(h224, _) => {
                h224
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt8::C0(h225, _) => {
                h225
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f35(v0: Adt1) -> Bytes {
    let m226 = v0;
    match m226 {
        Adt1::C0 => {
            f7(Str::lit("u8"))
        }
        Adt1::C1 => {
            f7(Str::lit("u16"))
        }
        Adt1::C2 => {
            f7(Str::lit("u32"))
        }
        Adt1::C3 => {
            f7(Str::lit("u64"))
        }
        Adt1::C4 => {
            f7(Str::lit("i8"))
        }
        Adt1::C5 => {
            f7(Str::lit("i16"))
        }
        Adt1::C6 => {
            f7(Str::lit("i32"))
        }
        Adt1::C7 => {
            f7(Str::lit("i64"))
        }
    }
}

pub fn f36(v0: Bytes) -> R<bool> {
    let a230 = {
        let (h227, _) = f41(Fn0::F51(v0.clone()), {
            let a228 = v0.clone();
            length_bytes(a228)
        }, 0u64)?;
        h227
    };
    let a231 = {
        let a229 = v0;
        length_bytes(a229)
    };
    Ok(nat_eq(a230, a231))
}

pub fn f37(v0: Adt0) -> Option<Adt0> {
    let m232 = v0;
    match m232 {
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
        Adt0::C8(r233) => {
            let v2 = (*r233).clone();
            Some(v2)
        }
        Adt0::C9(r234, _) => {
            let v3 = (*r234).clone();
            Some(v3)
        }
        Adt0::C10(r235) => {
            let v5 = (*r235).clone();
            Some(v5)
        }
        Adt0::C11(r236, _) => {
            let v6 = (*r236).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r237, _) => {
            let v9 = (*r237).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r238, _) => {
            let v11 = (*r238).clone();
            Some(v11)
        }
    }
}

pub fn f38(v0: Adt0) -> Option<Adt0> {
    let m239 = v0;
    match m239 {
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
        Adt0::C9(_, r240) => {
            let v4 = (*r240).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r241) => {
            let v7 = (*r241).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r242) => {
            let v10 = (*r242).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r243) => {
            let v12 = (*r243).clone();
            Some(v12)
        }
    }
}

pub fn f39(v0: Adt7, v1: u64) -> Option<Adt0> {
    f9(match v0 {
        Adt7::C0(_, h244, _, _, _, _) => {
            (*h244).clone()
        }
    }, v1)
}

pub fn f40(v0: u64) -> R<Bytes> {
    Ok(f6(f7(Str::lit("ADT ")), f6(f8(v0)?, f7(Str::lit(" is not declared")))))
}

pub fn f41(v0: Fn0, p249: u64, p250: u64) -> R<(u64, bool)> {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m245 = v1;
                if m245 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m245 - 1;
                    let m247 = {
                        let c246: Fn0 = v0.clone();
                        c246.apply(v2)?
                    };
                    match m247 {
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
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f52(v0, v1)
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m254 = {
        let a251 = v0;
        let a252 = v1;
        let a253 = v2;
        slice_bytes(a251, a252, a253)
    };
    match m254 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f44(v0: Fn1, p259: u64, p260: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m255 = v1;
                if m255 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m255 - 1;
                    let m257 = {
                        let c256: Fn1 = v0.clone();
                        c256.apply(v2.clone())?
                    };
                    match m257 {
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
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f53(v0, v1)
}

pub fn f46(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m261 = f54(v0, v1);
    match m261 {
        None => {
            false
        }
        Some(v3) => {
            let a262 = v3;
            let a263 = v2;
            equal(a262, a263)
        }
    }
}

pub fn f47(v0: Bytes, v1: Bytes) -> R<bool> {
    let a265 = 0u64;
    let a266 = {
        let a264 = v0.clone();
        length_bytes(a264)
    };
    if nat_lt(a265, a266) {
        if f55(v0.clone())? {
            let a267 = v0.clone();
            let a268 = f7(Str::lit("0"));
            if equal(a267, a268) {
                let a271 = {
                    let a269 = v1;
                    let a270 = f7(Str::lit("-0"));
                    equal(a269, a270)
                };
                Ok(bool_not(a271))
            } else {
                let a272 = f46(v0, 0u64, 48u8);
                Ok(bool_not(a272))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f48(v0: Bytes) -> Bytes {
    f6(f7(Str::lit("`")), f6(v0, f7(Str::lit("`"))))
}

pub fn f49(v0: Bytes, v1: Bytes) -> bool {
    let a277 = {
        let a273 = v0;
        let a274 = v1;
        compare_bytes(a273, a274)
    };
    let a278 = {
        let a275 = Bytes::lit(&[]);
        let a276 = Bytes::lit(&[0x01]);
        compare_bytes(a275, a276)
    };
    equal(a277, a278)
}

pub fn f50(v0: bool, v1: Str) -> Adt8 {
    Adt8::C0(v0, f7(v1))
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f56(v0, v1)
}

pub fn f52(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f46(v0, v1, 48u8) {
        Ok(Some({
            let a279 = v1;
            let a280 = 1u64;
            nat_add(a279, a280)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f53(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h281, _) = v1;
        h281
    };
    let v3: u64 = f57(v0.clone(), v2)?;
    let a282 = v3;
    let a283 = 256u64;
    if nat_eq(a282, a283) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f58(v0, v2, v3)?;
        Ok(Some(({
            let a285 = v2;
            let a286 = {
                let (h284, _) = v4;
                h284
            };
            nat_add(a285, a286)?
        }, f6({
            let (_, h287) = v1;
            h287
        }, {
            let (_, h288) = v4;
            h288
        }))))
    }
}

pub fn f54(v0: Bytes, v1: u64) -> Option<u8> {
    let a289 = v0;
    let a290 = v1;
    index_bytes(a289, a290)
}

pub fn f55(v0: Bytes) -> R<bool> {
    let a294 = {
        let (h291, _) = f41(Fn0::F59(v0.clone()), {
            let a292 = v0.clone();
            length_bytes(a292)
        }, 0u64)?;
        h291
    };
    let a295 = {
        let a293 = v0;
        length_bytes(a293)
    };
    Ok(nat_eq(a294, a295))
}

pub fn f56(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f60(v0, v1)? {
        Ok(Some({
            let a296 = v1;
            let a297 = 1u64;
            nat_add(a296, a297)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f57(v0: Bytes, v1: u64) -> R<u64> {
    let m298 = f54(v0, v1);
    match m298 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a343 = {
                let a319 = {
                    let a307 = {
                        let a301 = {
                            let a299 = v3;
                            let a300 = 128u8;
                            fixed_u8::bit_and(a299, a300)
                        };
                        let a302 = 128u8;
                        if equal(a301, a302) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a308 = {
                        let a305 = {
                            let a303 = v3;
                            let a304 = 64u8;
                            fixed_u8::bit_and(a303, a304)
                        };
                        let a306 = 64u8;
                        if equal(a305, a306) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a307, a308)?
                };
                let a320 = {
                    let a317 = {
                        let a311 = {
                            let a309 = v3;
                            let a310 = 32u8;
                            fixed_u8::bit_and(a309, a310)
                        };
                        let a312 = 32u8;
                        if equal(a311, a312) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a318 = {
                        let a315 = {
                            let a313 = v3;
                            let a314 = 16u8;
                            fixed_u8::bit_and(a313, a314)
                        };
                        let a316 = 16u8;
                        if equal(a315, a316) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a317, a318)?
                };
                nat_add(a319, a320)?
            };
            let a344 = {
                let a341 = {
                    let a329 = {
                        let a323 = {
                            let a321 = v3;
                            let a322 = 8u8;
                            fixed_u8::bit_and(a321, a322)
                        };
                        let a324 = 8u8;
                        if equal(a323, a324) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a330 = {
                        let a327 = {
                            let a325 = v3;
                            let a326 = 4u8;
                            fixed_u8::bit_and(a325, a326)
                        };
                        let a328 = 4u8;
                        if equal(a327, a328) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a329, a330)?
                };
                let a342 = {
                    let a339 = {
                        let a333 = {
                            let a331 = v3;
                            let a332 = 2u8;
                            fixed_u8::bit_and(a331, a332)
                        };
                        let a334 = 2u8;
                        if equal(a333, a334) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a340 = {
                        let a337 = {
                            let a335 = v3;
                            let a336 = 1u8;
                            fixed_u8::bit_and(a335, a336)
                        };
                        let a338 = 1u8;
                        if equal(a337, a338) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a339, a340)?
                };
                nat_add(a341, a342)?
            };
            nat_add(a343, a344)
        }
    }
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f57(v0.clone(), {
        let a345 = v1;
        let a346 = 1u64;
        nat_add(a345, a346)?
    })?;
    let a347 = v2;
    let a348 = 34u64;
    if nat_eq(a347, a348) {
        Ok((1u64, f7(Str::lit("\\\""))))
    } else {
        let a349 = v2;
        let a350 = 92u64;
        if nat_eq(a349, a350) {
            Ok((1u64, f7(Str::lit("\\\\"))))
        } else {
            let a351 = v2;
            let a352 = 10u64;
            if nat_eq(a351, a352) {
                Ok((1u64, f7(Str::lit("\\n"))))
            } else {
                let a353 = v2;
                let a354 = 13u64;
                if nat_eq(a353, a354) {
                    Ok((1u64, f7(Str::lit("\\r"))))
                } else {
                    let a355 = v2;
                    let a356 = 9u64;
                    if nat_eq(a355, a356) {
                        Ok((1u64, f7(Str::lit("\\t"))))
                    } else {
                        let a357 = v2;
                        let a358 = 0u64;
                        if nat_eq(a357, a358) {
                            Ok((1u64, f7(Str::lit("\\0"))))
                        } else {
                            let a359 = v2;
                            let a360 = 32u64;
                            if nat_lt(a359, a360) {
                                Ok((1u64, f61(v2)))
                            } else {
                                let a361 = v2;
                                let a362 = 127u64;
                                if nat_eq(a361, a362) {
                                    Ok((1u64, f61(v2)))
                                } else {
                                    let a363 = v2;
                                    let a364 = 194u64;
                                    if nat_eq(a363, a364) {
                                        let a365 = 128u64;
                                        let a366 = v3;
                                        if nat_le(a365, a366) {
                                            let a367 = v3;
                                            let a368 = 159u64;
                                            if nat_le(a367, a368) {
                                                Ok((2u64, f61(v3)))
                                            } else {
                                                Ok((1u64, f43(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f43(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f43(v0, v1, 1u64)))
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

pub fn f59(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f60(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f57(v0, v1)?;
    let a369 = 47u64;
    let a370 = v2;
    if nat_lt(a369, a370) {
        let a371 = v2;
        let a372 = 58u64;
        if nat_lt(a371, a372) {
            Ok(true)
        } else {
            let a373 = 96u64;
            let a374 = v2;
            if nat_lt(a373, a374) {
                let a375 = v2;
                let a376 = 103u64;
                Ok(nat_lt(a375, a376))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f61(v0: u64) -> Bytes {
    f6(f7(Str::lit("\\u{")), f6(f63(v0), f7(Str::lit("}"))))
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f64(v0, v1)? {
        Ok(Some({
            let a377 = v1;
            let a378 = 1u64;
            nat_add(a377, a378)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: u64) -> Bytes {
    let v1: Bytes = f65(v0);
    let v2: Bytes = f6(f65({
        let a379 = v0;
        let a380 = 16u64;
        let a381 = 0u64;
        nat_quot(a379, a380, a381)
    }), f65({
        let a382 = v0;
        let a383 = 16u64;
        let a384 = 0u64;
        nat_rem(a382, a383, a384)
    }));
    let a385 = v0;
    let a386 = 16u64;
    if nat_lt(a385, a386) {
        v1
    } else {
        v2
    }
}

pub fn f64(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f57(v0, v1)?;
    let a387 = 47u64;
    let a388 = v2;
    if nat_lt(a387, a388) {
        let a389 = v2;
        let a390 = 58u64;
        Ok(nat_lt(a389, a390))
    } else {
        Ok(false)
    }
}

pub fn f65(v0: u64) -> Bytes {
    f43(f7(Str::lit("0123456789abcdef")), v0, 1u64)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: u64, p7: Adt3) -> R<Result<(), Bytes>> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7)
}
