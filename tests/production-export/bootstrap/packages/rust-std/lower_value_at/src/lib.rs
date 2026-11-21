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
    C0(std::rc::Rc<Adt8>, std::rc::Rc<Adt11>),
    C1(Adt12, std::rc::Rc<Adt13>, std::rc::Rc<Adt11>),
    C2(Adt14, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C3(std::rc::Rc<Adt15>, std::rc::Rc<Adt7>, bool, std::rc::Rc<Adt11>),
    C4(std::rc::Rc<Adt13>, std::rc::Rc<Adt7>, bool, std::rc::Rc<Adt11>),
    C5(std::rc::Rc<Adt17>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C6(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C7(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C8(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C9(std::rc::Rc<Adt7>),
    C10(Adt23, std::rc::Rc<Adt13>, std::rc::Rc<Adt11>),
    C11(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C12,
    C13(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C14(std::rc::Rc<Adt24>, bool, std::rc::Rc<Adt18>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>, std::rc::Rc<Adt7>),
    C15(std::rc::Rc<Adt24>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C16(std::rc::Rc<Adt13>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt9, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt10),
    C8,
}

#[derive(Clone)]
pub enum Adt9 {
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
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64),
    C1(std::rc::Rc<Adt16>),
    C2(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(std::rc::Rc<Adt18>),
    C1,
    C2(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19),
    C1(Adt9),
    C2(Adt20, std::rc::Rc<Adt18>),
    C3(Adt21, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4(Adt22, u64),
    C5,
    C6(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
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
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt25),
    C1(std::rc::Rc<Adt13>),
    C2(std::rc::Rc<Adt24>),
    C3(Adt26, std::rc::Rc<Adt24>),
    C4(u64, u64, std::rc::Rc<Adt24>),
    C5,
    C6(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C7(Adt10),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt29>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt29>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt30>),
}

#[derive(Clone)]
pub enum Fn0 {
    F23,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt30) -> R<Option<Adt30>> {
        match self {
            Fn0::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F56(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn1::F56(k0) => f56(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F82(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn2::F82(k0) => f82(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: u64) -> R<Option<Adt7>> {
    let v8: Adt27 = Adt27::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5.clone()));
    let v9: Adt28 = f1(v8.clone(), Adt28::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt29::C0)), v5.clone(), 0u64, List::<bool>::nil(), v6)?;
    let v10: Adt0 = f2(match v9.clone() {
        Adt28::C0(h1, _) => {
            (*h1).clone()
        }
    }, v1.clone());
    let v11: Adt0 = f3(f4(f5(v1.clone())?), v10.clone());
    let v12: Adt30 = f6(v8.clone(), v10.clone(), match v9 {
        Adt28::C0(_, h2) => {
            (*h2).clone()
        }
    }, v11.clone(), Adt30::C0);
    let v13: Adt30 = f7(v8, v11.clone(), f8(v12, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v11.clone())?;
            let a4 = f5(v11)?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?);
    let m7 = f9(v5, v7);
    match m7 {
        None => {
            Ok(None::<Adt7>)
        }
        Some(v14) => {
            let m8 = f10(v14);
            match m8 {
                None => {
                    Ok(None::<Adt7>)
                }
                Some(v15) => {
                    let v16: Adt7 = f11(v10, v13, v1, Adt0::C14, 0u64, {
                        let (_, h9) = v15;
                        h9
                    }, {
                        let (h10, _) = v15;
                        h10
                    })?;
                    if f12(v16.clone()) {
                        Ok(None::<Adt7>)
                    } else {
                        Ok(Some(v16))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Adt27, p21: Adt28, p22: Adt2, p23: u64, v4: List<bool>, p24: Adt0) -> R<Adt28> {
    let m20 = (p21, (p22, (p23, p24)));
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m20;
                let m11 = v2;
                match m11 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C14(r18, r19) => {
                        let v34 = (*r18).clone();
                        let v35 = (*r19).clone();
                        let v37: bool = {
                            let m12 = f13(v4.clone(), v3);
                            match m12 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a13 = v36;
                                    bool_not(a13)
                                }
                            }
                        };
                        let v38: Adt28 = Adt28::C0(std::rc::Rc::new(f14(v0.clone(), match v1.clone() {
                            Adt28::C0(h14, _) => {
                                (*h14).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt28::C0(_, h15) => {
                                (*h15).clone()
                            }
                        }));
                        let v39: Adt28 = if v37 {
                            f15(v0.clone(), v38, v34, f16(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>((v39, (v35, ({
                            let a16 = v3;
                            let a17 = 1u64;
                            nat_add(a16, a17)?
                        }, f17(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p29: Adt0, p30: Adt0) -> Adt0 {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v0, v1) = m28;
                let m25 = v1;
                match m25 {
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
                    Adt0::C15(r26, r27) => {
                        let v12 = (*r26).clone();
                        let v13 = (*r27).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f18(v0, v12), v13))
                    }
                }
            };
            match step {
                Ok(next) => { m28 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt0, v1: Adt0) -> Adt0 {
    let m31 = v0;
    match m31 {
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
        Adt0::C15(r32, r33) => {
            let v12 = (*r32).clone();
            let v13 = (*r33).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f3(v13, v1)))
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let m34 = v0;
    if m34 == 0 {
        Adt0::C14
    } else {
        let v1 = m34 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f4(v1)))
    }
}

pub fn f5(v0: Adt0) -> R<u64> {
    let m35 = v0;
    match m35 {
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
        Adt0::C15(_, r38) => {
            let v12 = (*r38).clone();
            let v13: u64 = f5(v12)?;
            let a36 = 1u64;
            let a37 = v13;
            nat_add(a36, a37)
        }
    }
}

pub fn f6(v0: Adt27, v1: Adt0, v2: Adt29, p43: Adt0, p44: Adt30) -> Adt30 {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v3, v4) = m42;
                let m39 = v3;
                match m39 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C15(r40, r41) => {
                        let v15 = (*r40).clone();
                        let v16 = (*r41).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v16, f19(v1.clone(), v15.clone(), f20(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt27, p49: Adt0, p50: Adt30) -> Adt30 {
    let m48 = (p49, p50);
    {
        let mut m48 = m48;
        loop {
            let step = {
                let (v1, v2) = m48;
                let m45 = v1;
                match m45 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C15(r46, r47) => {
                        let v13 = (*r46).clone();
                        let v14 = (*r47).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v14, if f21(f20(v0.clone(), Adt29::C0, v13.clone())) {
                            Adt30::C1(std::rc::Rc::new(v13), std::rc::Rc::new(Adt0::C0), std::rc::Rc::new(v2))
                        } else {
                            v2
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m48 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt30, v1: u64) -> R<Adt30> {
    let (h51, _) = f22(Fn0::F23, v1, v0)?;
    Ok(h51)
}

pub fn f9(p57: Adt2, p58: u64) -> Option<Adt2> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v0, v1) = m56;
                let m52 = v0;
                match m52 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C13 => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C14(r54, r55) => {
                        let v30 = (*r54).clone();
                        let v31 = (*r55).clone();
                        let m53 = v1;
                        if m53 == 0 {
                            Err::<(Adt2, u64), Option<Adt2>>(Some(v30))
                        } else {
                            let v32 = m53 - 1;
                            Ok::<(Adt2, u64), Option<Adt2>>((v31, v32))
                        }
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
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

pub fn f10(v0: Adt2) -> Option<(Adt0, Adt3)> {
    let m59 = v0;
    match m59 {
        Adt2::C0(r60, r61) => {
            let v1 = (*r60).clone();
            let v2 = (*r61).clone();
            Some((v1, v2))
        }
        Adt2::C1(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C2(_, _, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C3(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C4(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C5(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C6(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C7(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C8(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C9(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C10(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C11(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C12(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C13 => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C14(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C15(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
    }
}

pub fn f11(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0, v4: u64, v5: Adt3, v6: Adt0) -> R<Adt7> {
    let m62 = v5;
    match m62 {
        Adt3::C0 => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C0), std::rc::Rc::new(f24({
                let a63 = Str::lit("value:unit");
                utf8_encode(a63)
            }, v6))))
        }
        Adt3::C1(v7) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C1(v7)), std::rc::Rc::new(f24({
                let a64 = Str::lit("value:bool");
                utf8_encode(a64)
            }, v6))))
        }
        Adt3::C2(v8) => {
            let m65 = f25(v8)?;
            match m65 {
                None => {
                    Ok(f26(0u64))
                }
                Some(v9) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C2(v9)), std::rc::Rc::new(f24({
                        let a66 = Str::lit("value:nat");
                        utf8_encode(a66)
                    }, v6))))
                }
            }
        }
        Adt3::C3(v10) => {
            let m67 = f27(v10)?;
            match m67 {
                None => {
                    Ok(f26(0u64))
                }
                Some(v11) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C3({
                        let (h68, _) = v11;
                        h68
                    }, {
                        let (_, h69) = v11;
                        h69
                    })), std::rc::Rc::new(f24({
                        let a70 = Str::lit("value:int");
                        utf8_encode(a70)
                    }, v6))))
                }
            }
        }
        Adt3::C4(v12) => {
            f28(v12, v6, {
                let a71 = Str::lit("value:u8");
                utf8_encode(a71)
            })
        }
        Adt3::C5(v13) => {
            f28(v13, v6, {
                let a72 = Str::lit("value:u16");
                utf8_encode(a72)
            })
        }
        Adt3::C6(v14) => {
            f28(v14, v6, {
                let a73 = Str::lit("value:u32");
                utf8_encode(a73)
            })
        }
        Adt3::C7(v15) => {
            f28(v15, v6, {
                let a74 = Str::lit("value:u64");
                utf8_encode(a74)
            })
        }
        Adt3::C8(v16) => {
            f28(v16, v6, {
                let a75 = Str::lit("value:i8");
                utf8_encode(a75)
            })
        }
        Adt3::C9(v17) => {
            f28(v17, v6, {
                let a76 = Str::lit("value:i16");
                utf8_encode(a76)
            })
        }
        Adt3::C10(v18) => {
            f28(v18, v6, {
                let a77 = Str::lit("value:i32");
                utf8_encode(a77)
            })
        }
        Adt3::C11(v19) => {
            f28(v19, v6, {
                let a78 = Str::lit("value:i64");
                utf8_encode(a78)
            })
        }
        Adt3::C12(v20) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C5(v20)), std::rc::Rc::new(f24({
                let a79 = Str::lit("value:string");
                utf8_encode(a79)
            }, v6))))
        }
        Adt3::C13(v21) => {
            let m80 = f29(v21)?;
            match m80 {
                None => {
                    Ok(f26(0u64))
                }
                Some(v22) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C6(v22)), std::rc::Rc::new(f24({
                        let a81 = Str::lit("value:bytes");
                        utf8_encode(a81)
                    }, v6))))
                }
            }
        }
        Adt3::C14(v23) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C7({
                let m82 = v23;
                match m82 {
                    Adt4::C0 => {
                        Adt10::C0
                    }
                    Adt4::C1 => {
                        Adt10::C1
                    }
                    Adt4::C2 => {
                        Adt10::C2
                    }
                }
            })), std::rc::Rc::new(f24({
                let a83 = Str::lit("value:ordering");
                utf8_encode(a83)
            }, v6))))
        }
        Adt3::C15 => {
            let v24: Adt0 = f30(v6.clone());
            Ok(Adt7::C5(std::rc::Rc::new(Adt17::C0(std::rc::Rc::new(f31(v0, v24)?))), std::rc::Rc::new(Adt7::C12), std::rc::Rc::new(f24({
                let a84 = Str::lit("value:none");
                utf8_encode(a84)
            }, v6))))
        }
        Adt3::C16(r105) => {
            let v25 = (*r105).clone();
            let v26: Adt0 = f30(v6.clone());
            let v27: Adt7 = f11(v0, v1, v2, v3, 0u64, v25, v26)?;
            if f12(v27.clone()) {
                Ok(f26(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C1), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v27), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f24({
                    let a85 = Str::lit("value:some");
                    utf8_encode(a85)
                }, v6))))
            }
        }
        Adt3::C17(r106) => {
            let v28 = (*r106).clone();
            let v29: Adt7 = f11(v0.clone(), v1, v2, v3, 0u64, v28, f30(v6.clone()))?;
            if f12(v29.clone()) {
                Ok(f26(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C2(std::rc::Rc::new(f31(v0.clone(), f30(v6.clone()))?), std::rc::Rc::new(f31(v0, f32(v6.clone()))?))), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v29), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f24({
                    let a86 = Str::lit("value:ok");
                    utf8_encode(a86)
                }, v6))))
            }
        }
        Adt3::C18(r107) => {
            let v30 = (*r107).clone();
            let v31: Adt7 = f11(v0.clone(), v1, v2, v3, 0u64, v30, f32(v6.clone()))?;
            if f12(v31.clone()) {
                Ok(f26(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C3(std::rc::Rc::new(f31(v0.clone(), f30(v6.clone()))?), std::rc::Rc::new(f31(v0, f32(v6.clone()))?))), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v31), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f24({
                    let a87 = Str::lit("value:error");
                    utf8_encode(a87)
                }, v6))))
            }
        }
        Adt3::C19(r108) => {
            let v32 = (*r108).clone();
            f11(v0, v1, v2, v3, 1u64, v32, f30(v6))
        }
        Adt3::C20(r109, r110) => {
            let v33 = (*r109).clone();
            let v34 = (*r110).clone();
            let v35: Adt7 = f11(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v33, f30(v6.clone()))?;
            if f12(v35.clone()) {
                Ok(f26(0u64))
            } else {
                let v36: Adt7 = f11(v0, v1, v2, v3, 0u64, v34, f32(v6.clone()))?;
                if f12(v36.clone()) {
                    Ok(f26(0u64))
                } else {
                    Ok(Adt7::C6(std::rc::Rc::new(v35), std::rc::Rc::new(v36), std::rc::Rc::new(f24({
                        let a88 = Str::lit("value:pair");
                        utf8_encode(a88)
                    }, v6))))
                }
            }
        }
        Adt3::C21(v37, r111) => {
            let v38 = (*r111).clone();
            let v39: u64 = f33(v6.clone());
            let m89 = f34(v2.clone(), v39, v37);
            match m89 {
                None => {
                    Ok(f26(0u64))
                }
                Some(v40) => {
                    let v41: Adt7 = f11(v0, v1, v2, Adt0::C12(v39), 2u64, v38, v40)?;
                    if f12(v41.clone()) {
                        Ok(f26(0u64))
                    } else {
                        Ok(Adt7::C5(std::rc::Rc::new(Adt17::C4(v39, v37)), std::rc::Rc::new(v41), std::rc::Rc::new(f24({
                            let a90 = Str::lit("value:adt");
                            utf8_encode(a90)
                        }, v6))))
                    }
                }
            }
        }
        Adt3::C22(_, _) => {
            Ok(f26(0u64))
        }
        Adt3::C23 => {
            let a91 = v4;
            let a92 = 1u64;
            if nat_eq(a91, a92) {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C7(std::rc::Rc::new(f31(v0, v6)?))), std::rc::Rc::new(Adt7::C12), std::rc::Rc::new(Adt11::C0({
                    let a93 = Str::lit("value:list");
                    utf8_encode(a93)
                }, None::<Adt9>))))
            } else {
                let a94 = v4;
                let a95 = 2u64;
                if nat_eq(a94, a95) {
                    Ok(Adt7::C12)
                } else {
                    Ok(f26(0u64))
                }
            }
        }
        Adt3::C24(r112, r113) => {
            let v44 = (*r112).clone();
            let v45 = (*r113).clone();
            let a96 = v4;
            let a97 = 1u64;
            if nat_eq(a96, a97) {
                let v46: Adt7 = f11(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v6.clone())?;
                if f12(v46.clone()) {
                    Ok(f26(0u64))
                } else {
                    let v47: Adt7 = f11(v0, v1, v2, v3, 1u64, v45, v6)?;
                    if f12(v47.clone()) {
                        Ok(f26(0u64))
                    } else {
                        Ok(Adt7::C5(std::rc::Rc::new(Adt17::C6), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v46), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v47), std::rc::Rc::new(Adt7::C12))))), std::rc::Rc::new(Adt11::C0({
                            let a98 = Str::lit("value:list");
                            utf8_encode(a98)
                        }, None::<Adt9>))))
                    }
                }
            } else {
                let a99 = v4;
                let a100 = 2u64;
                if nat_eq(a99, a100) {
                    let m101 = v6;
                    match m101 {
                        Adt0::C0 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C1 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C2 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C3 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C5 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C6 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C7 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C14 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C15(r103, r104) => {
                            let v58 = (*r103).clone();
                            let v59 = (*r104).clone();
                            let v60: Adt7 = f11(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v58.clone())?;
                            if f12(v60.clone()) {
                                Ok(f26(0u64))
                            } else {
                                let v61: Adt7 = f11(v0.clone(), v1.clone(), v2, v3.clone(), 2u64, v45, v59)?;
                                if f12(v61.clone()) {
                                    Ok(f26(0u64))
                                } else {
                                    Ok(Adt7::C13(std::rc::Rc::new(if f35(v0, v1, v3, v58) {
                                        Adt7::C2(Adt14::C1, std::rc::Rc::new(v60), std::rc::Rc::new(Adt11::C0({
                                            let a102 = Str::lit("indirection");
                                            utf8_encode(a102)
                                        }, None::<Adt9>)))
                                    } else {
                                        v60
                                    }), std::rc::Rc::new(v61)))
                                }
                            }
                        }
                    }
                } else {
                    Ok(f26(0u64))
                }
            }
        }
    }
}

pub fn f12(v0: Adt7) -> bool {
    let m114 = v0;
    match m114 {
        Adt7::C0(r116, _) => {
            let v1 = (*r116).clone();
            let m115 = v1;
            match m115 {
                Adt8::C0 => {
                    false
                }
                Adt8::C1(_) => {
                    false
                }
                Adt8::C2(_) => {
                    false
                }
                Adt8::C3(_, _) => {
                    false
                }
                Adt8::C4(_, _, _) => {
                    false
                }
                Adt8::C5(_) => {
                    false
                }
                Adt8::C6(_) => {
                    false
                }
                Adt8::C7(_) => {
                    false
                }
                Adt8::C8 => {
                    true
                }
            }
        }
        Adt7::C1(_, _, _) => {
            false
        }
        Adt7::C2(_, _, _) => {
            false
        }
        Adt7::C3(_, _, _, _) => {
            false
        }
        Adt7::C4(_, _, _, _) => {
            false
        }
        Adt7::C5(_, _, _) => {
            false
        }
        Adt7::C6(_, _, _) => {
            false
        }
        Adt7::C7(_, _, _, _) => {
            false
        }
        Adt7::C8(_, _, _) => {
            false
        }
        Adt7::C9(_) => {
            false
        }
        Adt7::C10(_, _, _) => {
            false
        }
        Adt7::C11(_, _) => {
            false
        }
        Adt7::C12 => {
            false
        }
        Adt7::C13(_, _) => {
            false
        }
        Adt7::C14(_, _, _, _, _, _) => {
            false
        }
        Adt7::C15(_, _, _) => {
            false
        }
        Adt7::C16(_, _) => {
            false
        }
    }
}

pub fn f13(p120: List<bool>, p121: u64) -> Option<bool> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v0, v1) = m119;
                let m117 = v0;
                match m117.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m118 = v1;
                        if m118 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m118 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Adt27, v1: Adt0, v2: u64) -> Adt0 {
    f36(f37(v1, f38(v0.clone(), v2)), f39(v0, v2))
}

pub fn f15(v0: Adt27, v1: Adt28, v2: Adt2, v3: Adt0) -> R<Adt28> {
    let m122 = v2;
    match m122 {
        Adt2::C0(r133, _) => {
            let v4 = (*r133).clone();
            Ok(Adt28::C0(std::rc::Rc::new(f36(match v1.clone() {
                Adt28::C0(h123, _) => {
                    (*h123).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h124) => {
                    (*h124).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r134, r135, r136) => {
            let v8 = (*r134).clone();
            let v9 = (*r135).clone();
            let v10 = (*r136).clone();
            let v11: Adt28 = Adt28::C0(std::rc::Rc::new(f36(match v1.clone() {
                Adt28::C0(h125, _) => {
                    (*h125).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h126) => {
                    (*h126).clone()
                }
            }));
            let v12: Adt28 = f15(v0.clone(), v11, v9, f16(v3.clone()))?;
            f15(v0, v12, v10, f17(v3))
        }
        Adt2::C3(r137, r138, r139) => {
            let v13 = (*r137).clone();
            let v14 = (*r138).clone();
            let v15 = (*r139).clone();
            let v16: Adt28 = f15(v0.clone(), v1, v13, f16(v3.clone()))?;
            let v17: Adt28 = f15(v0.clone(), v16, v14, f17(v3.clone()))?;
            f15(v0, v17, v15, f40(v3))
        }
        Adt2::C4(r140, r141, r142) => {
            let v18 = (*r140).clone();
            let v19 = (*r141).clone();
            let v20 = (*r142).clone();
            let v21: Adt28 = Adt28::C0(std::rc::Rc::new(f36(match v1.clone() {
                Adt28::C0(h127, _) => {
                    (*h127).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h128) => {
                    (*h128).clone()
                }
            }));
            let v22: Adt28 = f15(v0.clone(), v21, v19, f16(v3.clone()))?;
            f15(v0, v22, v20, f17(v3))
        }
        Adt2::C5(_, r143, r144) => {
            let v24 = (*r143).clone();
            let v25 = (*r144).clone();
            let v26: Adt28 = Adt28::C0(std::rc::Rc::new(f36(match v1.clone() {
                Adt28::C0(h129, _) => {
                    (*h129).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h130) => {
                    (*h130).clone()
                }
            }));
            f15(v0, v26, v25, f16(v3))
        }
        Adt2::C6(_, r145) => {
            let v28 = (*r145).clone();
            f15(v0, v1, v28, f16(v3))
        }
        Adt2::C7(v29, r146) => {
            let v30 = (*r146).clone();
            let v31: Adt0 = f38(v0.clone(), v29);
            let v32: u64 = f41(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f42(v31.clone(), v32)), std::rc::Rc::new(f39(v0.clone(), v29)));
            let v34: Adt28 = Adt28::C0(std::rc::Rc::new(f36(match v1.clone() {
                Adt28::C0(h131, _) => {
                    (*h131).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f43(match v1 {
                Adt28::C0(_, h132) => {
                    (*h132).clone()
                }
            }, v33, v29, f44(v31, v32))));
            f15(v0, v34, v30, f16(v3))
        }
        Adt2::C8(r147, r148) => {
            let v35 = (*r147).clone();
            let v36 = (*r148).clone();
            let v37: Adt28 = f15(v0.clone(), v1, v35, f16(v3.clone()))?;
            f15(v0, v37, v36, f17(v3))
        }
        Adt2::C9(_, r149) => {
            let v39 = (*r149).clone();
            f15(v0, v1, v39, f16(v3))
        }
        Adt2::C10(r150) => {
            let v40 = (*r150).clone();
            f15(v0, v1, v40, f16(v3))
        }
        Adt2::C11(r151) => {
            let v41 = (*r151).clone();
            f15(v0, v1, v41, f16(v3))
        }
        Adt2::C12(r152, _) => {
            let v42 = (*r152).clone();
            f15(v0, v1, v42, f16(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r153, r154) => {
            let v44 = (*r153).clone();
            let v45 = (*r154).clone();
            let v46: Adt28 = f15(v0.clone(), v1, v44, f16(v3.clone()))?;
            f15(v0, v46, v45, f17(v3))
        }
        Adt2::C15(_, _, r155) => {
            let v49 = (*r155).clone();
            f15(v0, v1, v49, f16(v3))
        }
    }
}

pub fn f16(v0: Adt0) -> Adt0 {
    f30(f32(v0))
}

pub fn f17(v0: Adt0) -> Adt0 {
    f30(f32(f32(v0)))
}

pub fn f18(p160: Adt0, p161: Adt0) -> Adt0 {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v0, v1) = m159;
                let m156 = v1;
                match m156 {
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
                    Adt0::C15(r157, r158) => {
                        let v12 = (*r157).clone();
                        let v13 = (*r158).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f37(v0, v12), v13))
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt0, v1: Adt0, p166: Adt0, p167: Adt30) -> Adt30 {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v2, v3) = m165;
                let m162 = v2;
                match m162 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r163, r164) => {
                        let v14 = (*r163).clone();
                        let v15 = (*r164).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f45(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt27, v1: Adt29, v2: Adt0) -> Adt0 {
    let m168 = v2;
    match m168 {
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
            let m169 = f46(v0, v10);
            match m169 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r170, r171) => {
            let v12 = (*r170).clone();
            let v13 = (*r171).clone();
            f47(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f21(p176: Adt0) -> bool {
    let m175 = p176;
    {
        let mut m175 = m175;
        loop {
            let step = {
                let v0 = m175;
                let m172 = v0;
                match m172 {
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
                    Adt0::C15(r173, r174) => {
                        let v11 = (*r173).clone();
                        let v12 = (*r174).clone();
                        if f48(v11, Adt0::C14) {
                            Ok::<Adt0, bool>(v12)
                        } else {
                            Err::<Adt0, bool>(true)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Fn0, p181: u64, p182: Adt30) -> R<(Adt30, bool)> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v1, v2) = m180;
                let m177 = v1;
                if m177 == 0 {
                    Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m177 - 1;
                    let m179 = {
                        let c178: Fn0 = v0.clone();
                        c178.apply(v2.clone())?
                    };
                    match m179 {
                        None => {
                            Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt30), R<(Adt30, bool)>>((v3, v4))
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

pub fn f23(v0: Adt30) -> R<Option<Adt30>> {
    f49(v0)
}

pub fn f24(v0: Bytes, v1: Adt0) -> Adt11 {
    Adt11::C0(v0, f50(v1))
}

pub fn f25(v0: Bytes) -> R<Option<u64>> {
    let m183 = f51(v0, false)?;
    match m183 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            Ok(Some({
                let (_, h184) = v1;
                h184
            }))
        }
    }
}

pub fn f26(_: u64) -> Adt7 {
    Adt7::C0(std::rc::Rc::new(Adt8::C8), std::rc::Rc::new(Adt11::C0({
        let a185 = Str::lit("");
        utf8_encode(a185)
    }, None::<Adt9>)))
}

pub fn f27(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m186 = f51(v0, true)?;
    match m186 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            let (h187, _) = v1;
            if h187 {
                let a189 = {
                    let (_, h188) = v1;
                    h188
                };
                let a190 = 9223372036854775809u64;
                if nat_lt(a189, a190) {
                    Ok(Some(({
                        let a192 = 0u64;
                        let a193 = {
                            let (_, h191) = v1;
                            h191
                        };
                        nat_lt(a192, a193)
                    }, {
                        let (_, h194) = v1;
                        h194
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            } else {
                let a196 = {
                    let (_, h195) = v1;
                    h195
                };
                let a197 = 9223372036854775808u64;
                if nat_lt(a196, a197) {
                    Ok(Some((false, {
                        let (_, h198) = v1;
                        h198
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: Adt0, v2: Bytes) -> R<Adt7> {
    let m199 = f52(v1.clone());
    match m199 {
        None => {
            Ok(f26(0u64))
        }
        Some(v3) => {
            let m200 = f53(v0)?;
            match m200 {
                None => {
                    Ok(f26(0u64))
                }
                Some(v4) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C4(f54(v3), {
                        let (h201, _) = v4;
                        h201
                    }, {
                        let (_, h202) = v4;
                        h202
                    })), std::rc::Rc::new(f24(v2, v1))))
                }
            }
        }
    }
}

pub fn f29(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a203 = v0.clone();
        length_bytes(a203)
    };
    let v2: ((u64, Bytes), bool) = f55(Fn1::F56(v0), v1, (0u64, {
        let a204 = Str::lit("");
        utf8_encode(a204)
    }))?;
    let a207 = {
        let (h205, _) = {
            let (h206, _) = v2.clone();
            h206
        };
        h205
    };
    let a208 = v1;
    if nat_eq(a207, a208) {
        Ok(Some({
            let (_, h209) = {
                let (h210, _) = v2;
                h210
            };
            h209
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f30(v0: Adt0) -> Adt0 {
    let m211 = f57(v0);
    match m211 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f31(v0: Adt0, v1: Adt0) -> R<Adt18> {
    let m212 = v1;
    match m212 {
        Adt0::C0 => {
            Ok(Adt18::C0(Adt19::C0))
        }
        Adt0::C1 => {
            Ok(Adt18::C0(Adt19::C1))
        }
        Adt0::C2 => {
            Ok(Adt18::C0(Adt19::C2))
        }
        Adt0::C3 => {
            Ok(Adt18::C0(Adt19::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt18::C1(f54(v2)))
        }
        Adt0::C5 => {
            Ok(Adt18::C0(Adt19::C5))
        }
        Adt0::C6 => {
            Ok(Adt18::C0(Adt19::C6))
        }
        Adt0::C7 => {
            Ok(Adt18::C0(Adt19::C4))
        }
        Adt0::C8(r213) => {
            let v3 = (*r213).clone();
            Ok(Adt18::C2(Adt20::C0, std::rc::Rc::new(f31(v0, v3)?)))
        }
        Adt0::C9(r214, r215) => {
            let v4 = (*r214).clone();
            let v5 = (*r215).clone();
            Ok(Adt18::C3(Adt21::C0, std::rc::Rc::new(f31(v0.clone(), v4)?), std::rc::Rc::new(f31(v0, v5)?)))
        }
        Adt0::C10(r216) => {
            let v6 = (*r216).clone();
            Ok(Adt18::C2(Adt20::C1, std::rc::Rc::new(f31(v0, v6)?)))
        }
        Adt0::C11(r217, r218) => {
            let v7 = (*r217).clone();
            let v8 = (*r218).clone();
            Ok(Adt18::C3(Adt21::C1, std::rc::Rc::new(f31(v0.clone(), v7)?), std::rc::Rc::new(f31(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt18::C4(Adt22::C0, v9))
        }
        Adt0::C13(r219, r220) => {
            let v10 = (*r219).clone();
            let v11 = (*r220).clone();
            Ok(Adt18::C4(Adt22::C1, f58(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt18::C5)
        }
        Adt0::C15(r221, r222) => {
            let v12 = (*r221).clone();
            let v13 = (*r222).clone();
            Ok(Adt18::C6(std::rc::Rc::new(f31(v0.clone(), v12)?), std::rc::Rc::new(f31(v0, v13)?)))
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    let m223 = f59(v0);
    match m223 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: Adt0) -> u64 {
    let m224 = v0;
    match m224 {
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

pub fn f34(v0: Adt0, v1: u64, v2: u64) -> Option<Adt0> {
    let m225 = f60(v0, v1);
    match m225 {
        None => {
            None::<Adt0>
        }
        Some(v3) => {
            f60(v3, v2)
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0) -> bool {
    if f61(v0, v1.clone(), v2, v3.clone()) {
        true
    } else {
        f62(v1, v3, Adt0::C0)
    }
}

pub fn f36(v0: Adt0, v1: Adt0) -> Adt0 {
    let m226 = v1;
    match m226 {
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
        Adt0::C8(r227) => {
            let v3 = (*r227).clone();
            f36(v0, v3)
        }
        Adt0::C9(r228, r229) => {
            let v4 = (*r228).clone();
            let v5 = (*r229).clone();
            f36(f36(v0, v4), v5)
        }
        Adt0::C10(r230) => {
            let v6 = (*r230).clone();
            f36(v0, v6)
        }
        Adt0::C11(r231, r232) => {
            let v7 = (*r231).clone();
            let v8 = (*r232).clone();
            f36(f36(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r233, r234) => {
            let v10 = (*r233).clone();
            let v11 = (*r234).clone();
            let v12: Adt0 = f63(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f36(f36(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r235, r236) => {
            let v13 = (*r235).clone();
            let v14 = (*r236).clone();
            f36(f36(v0, v13), v14)
        }
    }
}

pub fn f37(p241: Adt0, p242: Adt0) -> Adt0 {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v0, v1) = m240;
                let m237 = v1;
                match m237 {
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
                    Adt0::C15(r238, r239) => {
                        let v2 = (*r238).clone();
                        let v3 = (*r239).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f36(v0, v2), v3))
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

pub fn f38(v0: Adt27, v1: u64) -> Adt0 {
    let m243 = f64(v0, v1);
    match m243 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f39(v0: Adt27, v1: u64) -> Adt0 {
    let m245 = f65(match v0 {
        Adt27::C0(_, _, _, _, h244, _) => {
            (*h244).clone()
        }
    }, v1);
    match m245 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: Adt0) -> Adt0 {
    f30(f32(f32(f32(v0))))
}

pub fn f41(v0: Adt2) -> R<u64> {
    let m246 = v0;
    match m246 {
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
        Adt2::C14(_, r249) => {
            let v33 = (*r249).clone();
            let v34: u64 = f41(v33)?;
            let a247 = 1u64;
            let a248 = v34;
            nat_add(a247, a248)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f42(p255: Adt0, p256: u64) -> Adt0 {
    let m254 = (p255, p256);
    {
        let mut m254 = m254;
        loop {
            let step = {
                let (v0, v1) = m254;
                let m250 = v0;
                match m250 {
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
                    Adt0::C15(r252, r253) => {
                        let v12 = (*r252).clone();
                        let v13 = (*r253).clone();
                        let m251 = v1;
                        if m251 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m251 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt29, v1: Adt0, v2: u64, v3: Adt0) -> Adt29 {
    if f66(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt29::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f44(v0: Adt0, v1: u64) -> Adt0 {
    let m257 = v0;
    match m257 {
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
        Adt0::C15(r259, r260) => {
            let v12 = (*r259).clone();
            let v13 = (*r260).clone();
            let m258 = v1;
            if m258 == 0 {
                Adt0::C14
            } else {
                let v14 = m258 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f44(v13, v14)))
            }
        }
    }
}

pub fn f45(v0: Adt0, v1: Adt0, p265: Adt0, p266: Adt30) -> Adt30 {
    let m264 = (p265, p266);
    {
        let mut m264 = m264;
        loop {
            let step = {
                let (v2, v3) = m264;
                let m261 = v2;
                match m261 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r262, r263) => {
                        let v14 = (*r262).clone();
                        let v15 = (*r263).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f67(v1.clone(), f68(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Adt27, v1: u64) -> Option<Adt0> {
    f65(match v0 {
        Adt27::C0(_, h267, _, _, _, _) => {
            (*h267).clone()
        }
    }, v1)
}

pub fn f47(v0: Adt29, v1: Adt0) -> Adt0 {
    let m268 = v0;
    match m268 {
        Adt29::C0 => {
            Adt0::C14
        }
        Adt29::C1(r269, _, r270, r271) => {
            let v2 = (*r269).clone();
            let v4 = (*r270).clone();
            let v5 = (*r271).clone();
            if f48(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f47(v5, v1)))
            } else {
                f47(v5, v1)
            }
        }
    }
}

pub fn f48(v0: Adt0, v1: Adt0) -> bool {
    let m272 = v0;
    match m272 {
        Adt0::C0 => {
            let a273 = f69(v1);
            let a274 = 0u64;
            nat_eq(a273, a274)
        }
        Adt0::C1 => {
            let a275 = f69(v1);
            let a276 = 1u64;
            nat_eq(a275, a276)
        }
        Adt0::C2 => {
            let a277 = f69(v1);
            let a278 = 2u64;
            nat_eq(a277, a278)
        }
        Adt0::C3 => {
            let a279 = f69(v1);
            let a280 = 3u64;
            nat_eq(a279, a280)
        }
        Adt0::C4(v2) => {
            let a289 = f69(v1.clone());
            let a290 = 4u64;
            if nat_eq(a289, a290) {
                let a291 = f70(v2);
                let a292 = f70(f71(v1));
                nat_eq(a291, a292)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a281 = f69(v1);
            let a282 = 5u64;
            nat_eq(a281, a282)
        }
        Adt0::C6 => {
            let a283 = f69(v1);
            let a284 = 6u64;
            nat_eq(a283, a284)
        }
        Adt0::C7 => {
            let a285 = f69(v1);
            let a286 = 7u64;
            nat_eq(a285, a286)
        }
        Adt0::C8(r309) => {
            let v3 = (*r309).clone();
            let v4: bool = f48(v3, f30(v1.clone()));
            let a293 = f69(v1);
            let a294 = 8u64;
            if nat_eq(a293, a294) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r310, r311) => {
            let v5 = (*r310).clone();
            let v6 = (*r311).clone();
            let v7: bool = f48(v5, f30(v1.clone()));
            let v8: bool = f48(v6, f32(v1.clone()));
            let a295 = f69(v1);
            let a296 = 9u64;
            if nat_eq(a295, a296) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r312) => {
            let v9 = (*r312).clone();
            let v10: bool = f48(v9, f30(v1.clone()));
            let a297 = f69(v1);
            let a298 = 10u64;
            if nat_eq(a297, a298) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r313, r314) => {
            let v11 = (*r313).clone();
            let v12 = (*r314).clone();
            let v13: bool = f48(v11, f30(v1.clone()));
            let v14: bool = f48(v12, f32(v1.clone()));
            let a299 = f69(v1);
            let a300 = 11u64;
            if nat_eq(a299, a300) {
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
            let a301 = f69(v1.clone());
            let a302 = 12u64;
            if nat_eq(a301, a302) {
                let a303 = v15;
                let a304 = f33(v1);
                nat_eq(a303, a304)
            } else {
                false
            }
        }
        Adt0::C13(r315, r316) => {
            let v16 = (*r315).clone();
            let v17 = (*r316).clone();
            let v18: bool = f48(v16, f30(v1.clone()));
            let v19: bool = f48(v17, f32(v1.clone()));
            let a305 = f69(v1);
            let a306 = 13u64;
            if nat_eq(a305, a306) {
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
            let a287 = f69(v1);
            let a288 = 14u64;
            nat_eq(a287, a288)
        }
        Adt0::C15(r317, r318) => {
            let v20 = (*r317).clone();
            let v21 = (*r318).clone();
            let v22: bool = f48(v20, f30(v1.clone()));
            let v23: bool = f48(v21, f32(v1.clone()));
            let a307 = f69(v1);
            let a308 = 15u64;
            if nat_eq(a307, a308) {
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

pub fn f49(v0: Adt30) -> R<Option<Adt30>> {
    let v1: Adt30 = f72(v0.clone(), v0.clone(), v0.clone());
    let a319 = f73(v1.clone())?;
    let a320 = f73(v0)?;
    if nat_eq(a319, a320) {
        Ok(None::<Adt30>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f50(v0: Adt0) -> Option<Adt9> {
    let m321 = v0;
    match m321 {
        Adt0::C0 => {
            None::<Adt9>
        }
        Adt0::C1 => {
            None::<Adt9>
        }
        Adt0::C2 => {
            None::<Adt9>
        }
        Adt0::C3 => {
            None::<Adt9>
        }
        Adt0::C4(v1) => {
            Some(f54(v1))
        }
        Adt0::C5 => {
            None::<Adt9>
        }
        Adt0::C6 => {
            None::<Adt9>
        }
        Adt0::C7 => {
            None::<Adt9>
        }
        Adt0::C8(_) => {
            None::<Adt9>
        }
        Adt0::C9(_, _) => {
            None::<Adt9>
        }
        Adt0::C10(_) => {
            None::<Adt9>
        }
        Adt0::C11(_, _) => {
            None::<Adt9>
        }
        Adt0::C12(_) => {
            None::<Adt9>
        }
        Adt0::C13(_, _) => {
            None::<Adt9>
        }
        Adt0::C14 => {
            None::<Adt9>
        }
        Adt0::C15(_, _) => {
            None::<Adt9>
        }
    }
}

pub fn f51(v0: Bytes, v1: bool) -> R<Option<(bool, u64)>> {
    let m322 = f74(v0.clone(), 0u64);
    match m322 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v2) => {
            if if v1 {
                let a323 = v2;
                let a324 = 45u8;
                equal(a323, a324)
            } else {
                false
            } {
                let m325 = f75(v0, 1u64)?;
                match m325 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v3) => {
                        Ok(Some((true, v3)))
                    }
                }
            } else {
                let m326 = f75(v0, 0u64)?;
                match m326 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v4) => {
                        Ok(Some((false, v4)))
                    }
                }
            }
        }
    }
}

pub fn f52(v0: Adt0) -> Option<Adt1> {
    let m327 = v0;
    match m327 {
        Adt0::C0 => {
            None::<Adt1>
        }
        Adt0::C1 => {
            None::<Adt1>
        }
        Adt0::C2 => {
            None::<Adt1>
        }
        Adt0::C3 => {
            None::<Adt1>
        }
        Adt0::C4(v1) => {
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt1>
        }
        Adt0::C6 => {
            None::<Adt1>
        }
        Adt0::C7 => {
            None::<Adt1>
        }
        Adt0::C8(_) => {
            None::<Adt1>
        }
        Adt0::C9(_, _) => {
            None::<Adt1>
        }
        Adt0::C10(_) => {
            None::<Adt1>
        }
        Adt0::C11(_, _) => {
            None::<Adt1>
        }
        Adt0::C12(_) => {
            None::<Adt1>
        }
        Adt0::C13(_, _) => {
            None::<Adt1>
        }
        Adt0::C14 => {
            None::<Adt1>
        }
        Adt0::C15(_, _) => {
            None::<Adt1>
        }
    }
}

pub fn f53(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m328 = f51(v0, true)?;
    match m328 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            Ok(Some(({
                let (h329, _) = v1;
                if h329 {
                    let a331 = 0u64;
                    let a332 = {
                        let (_, h330) = v1;
                        h330
                    };
                    nat_lt(a331, a332)
                } else {
                    false
                }
            }, {
                let (_, h333) = v1;
                h333
            })))
        }
    }
}

pub fn f54(v0: Adt1) -> Adt9 {
    let m334 = v0;
    match m334 {
        Adt1::C0 => {
            Adt9::C0
        }
        Adt1::C1 => {
            Adt9::C1
        }
        Adt1::C2 => {
            Adt9::C2
        }
        Adt1::C3 => {
            Adt9::C3
        }
        Adt1::C4 => {
            Adt9::C4
        }
        Adt1::C5 => {
            Adt9::C5
        }
        Adt1::C6 => {
            Adt9::C6
        }
        Adt1::C7 => {
            Adt9::C7
        }
    }
}

pub fn f55(v0: Fn1, p339: u64, p340: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v1, v2) = m338;
                let m335 = v1;
                if m335 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m335 - 1;
                    let m337 = {
                        let c336: Fn1 = v0.clone();
                        c336.apply(v2.clone())?
                    };
                    match m337 {
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
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f76(v0, v1)
}

pub fn f57(v0: Adt0) -> Option<Adt0> {
    let m341 = v0;
    match m341 {
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
        Adt0::C8(r342) => {
            let v2 = (*r342).clone();
            Some(v2)
        }
        Adt0::C9(r343, _) => {
            let v3 = (*r343).clone();
            Some(v3)
        }
        Adt0::C10(r344) => {
            let v5 = (*r344).clone();
            Some(v5)
        }
        Adt0::C11(r345, _) => {
            let v6 = (*r345).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r346, _) => {
            let v9 = (*r346).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r347, _) => {
            let v11 = (*r347).clone();
            Some(v11)
        }
    }
}

pub fn f58(p354: Adt0, v1: Adt0, p355: u64) -> R<u64> {
    let m353 = (p354, p355);
    {
        let mut m353 = m353;
        loop {
            let step = {
                let (v0, v2) = m353;
                let m348 = v0;
                match m348 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r351, r352) => {
                        let v13 = (*r351).clone();
                        let v14 = (*r352).clone();
                        if f48(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a349 = v2;
                                let a350 = 1u64;
                                nat_add(a349, a350)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt0) -> Option<Adt0> {
    let m356 = v0;
    match m356 {
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
        Adt0::C9(_, r357) => {
            let v4 = (*r357).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r358) => {
            let v7 = (*r358).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r359) => {
            let v10 = (*r359).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r360) => {
            let v12 = (*r360).clone();
            Some(v12)
        }
    }
}

pub fn f60(p366: Adt0, p367: u64) -> Option<Adt0> {
    let m365 = (p366, p367);
    {
        let mut m365 = m365;
        loop {
            let step = {
                let (v0, v1) = m365;
                let m361 = v0;
                match m361 {
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
                    Adt0::C15(r363, r364) => {
                        let v12 = (*r363).clone();
                        let v13 = (*r364).clone();
                        let m362 = v1;
                        if m362 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m362 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0) -> bool {
    f77(v2, v1, f68(v0, Adt0::C14, v3))
}

pub fn f62(p373: Adt30, v1: Adt0, v2: Adt0) -> bool {
    let m372 = p373;
    {
        let mut m372 = m372;
        loop {
            let step = {
                let v0 = m372;
                let m368 = v0;
                match m368 {
                    Adt30::C0 => {
                        Err::<Adt30, bool>(false)
                    }
                    Adt30::C1(r369, r370, r371) => {
                        let v3 = (*r369).clone();
                        let v4 = (*r370).clone();
                        let v5 = (*r371).clone();
                        if f48(v3, v1.clone()) {
                            if f48(v4, v2.clone()) {
                                Err::<Adt30, bool>(true)
                            } else {
                                Ok::<Adt30, bool>(v5)
                            }
                        } else {
                            Ok::<Adt30, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt0, v1: Adt0) -> Adt0 {
    if f78(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f64(v0: Adt27, v1: u64) -> Option<Adt0> {
    f65(match v0 {
        Adt27::C0(_, _, _, h374, _, _) => {
            (*h374).clone()
        }
    }, v1)
}

pub fn f65(p380: Adt0, p381: u64) -> Option<Adt0> {
    let m379 = (p380, p381);
    {
        let mut m379 = m379;
        loop {
            let step = {
                let (v0, v1) = m379;
                let m375 = v0;
                match m375 {
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
                    Adt0::C15(r377, r378) => {
                        let v12 = (*r377).clone();
                        let v13 = (*r378).clone();
                        let m376 = v1;
                        if m376 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m376 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(p388: Adt29, v1: Adt0, v2: u64) -> bool {
    let m387 = p388;
    {
        let mut m387 = m387;
        loop {
            let step = {
                let v0 = m387;
                let m382 = v0;
                match m382 {
                    Adt29::C0 => {
                        Err::<Adt29, bool>(false)
                    }
                    Adt29::C1(r385, v4, _, r386) => {
                        let v3 = (*r385).clone();
                        let v6 = (*r386).clone();
                        let a383 = v4;
                        let a384 = v2;
                        if nat_eq(a383, a384) {
                            if f48(v3, v1.clone()) {
                                Err::<Adt29, bool>(true)
                            } else {
                                Ok::<Adt29, bool>(v6)
                            }
                        } else {
                            Ok::<Adt29, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt0, p393: Adt0, p394: Adt30) -> Adt30 {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v1, v2) = m392;
                let m389 = v1;
                match m389 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C15(r390, r391) => {
                        let v13 = (*r390).clone();
                        let v14 = (*r391).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v14, f79(v2, v0.clone(), v13)))
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

pub fn f68(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m395 = v2;
    match m395 {
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
        Adt0::C8(r396) => {
            let v4 = (*r396).clone();
            f68(v0, v1, v4)
        }
        Adt0::C9(r397, r398) => {
            let v5 = (*r397).clone();
            let v6 = (*r398).clone();
            f68(v0.clone(), f68(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r399, r400) => {
            let v8 = (*r399).clone();
            let v9 = (*r400).clone();
            f68(v0.clone(), f68(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f63(v1, Adt0::C12(v10))
        }
        Adt0::C13(r401, r402) => {
            let v11 = (*r401).clone();
            let v12 = (*r402).clone();
            if f78(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f63(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f69(v0: Adt0) -> u64 {
    let m403 = v0;
    match m403 {
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

pub fn f70(v0: Adt1) -> u64 {
    let m404 = v0;
    match m404 {
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

pub fn f71(v0: Adt0) -> Adt1 {
    let m405 = v0;
    match m405 {
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

pub fn f72(v0: Adt30, p411: Adt30, p412: Adt30) -> Adt30 {
    let m410 = (p411, p412);
    {
        let mut m410 = m410;
        loop {
            let step = {
                let (v1, v2) = m410;
                let m406 = v1;
                match m406 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v2)
                    }
                    Adt30::C1(r407, r408, r409) => {
                        let v3 = (*r407).clone();
                        let v4 = (*r408).clone();
                        let v5 = (*r409).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v5, f80(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Adt30) -> R<u64> {
    let m413 = v0;
    match m413 {
        Adt30::C0 => {
            Ok(0u64)
        }
        Adt30::C1(_, _, r416) => {
            let v3 = (*r416).clone();
            let a414 = 1u64;
            let a415 = f73(v3)?;
            nat_add(a414, a415)
        }
    }
}

pub fn f74(v0: Bytes, v1: u64) -> Option<u8> {
    let a417 = v0;
    let a418 = v1;
    index_bytes(a417, a418)
}

pub fn f75(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a419 = v0.clone();
        length_bytes(a419)
    };
    let a420 = v1;
    let a421 = v2;
    if nat_lt(a420, a421) {
        let v3: ((u64, u64), bool) = f81(Fn2::F82(v0), {
            let a422 = v2;
            let a423 = v1;
            nat_sub(a422, a423)
        }, (v1, 0u64))?;
        let a426 = {
            let (h424, _) = {
                let (h425, _) = v3;
                h425
            };
            h424
        };
        let a427 = v2;
        if nat_eq(a426, a427) {
            Ok(Some({
                let (_, h428) = {
                    let (h429, _) = v3;
                    h429
                };
                h428
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f76(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m431 = f74(v0.clone(), {
        let (h430, _) = v1.clone();
        h430
    });
    match m431 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            let m435 = f74(v0, {
                let a433 = {
                    let (h432, _) = v1.clone();
                    h432
                };
                let a434 = 1u64;
                nat_add(a433, a434)?
            });
            match m435 {
                None => {
                    Ok(None::<(u64, Bytes)>)
                }
                Some(v3) => {
                    let m436 = f83(v2, v3)?;
                    match m436 {
                        None => {
                            Ok(None::<(u64, Bytes)>)
                        }
                        Some(v4) => {
                            Ok(Some(({
                                let a438 = {
                                    let (h437, _) = v1;
                                    h437
                                };
                                let a439 = 2u64;
                                nat_add(a438, a439)?
                            }, {
                                let a441 = {
                                    let (_, h440) = v1;
                                    h440
                                };
                                let a442 = f84(f85(0u64), v4, 1u64);
                                append_bytes(a441, a442)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f77(v0: Adt0, v1: Adt30, p447: Adt0) -> bool {
    let m446 = p447;
    {
        let mut m446 = m446;
        loop {
            let step = {
                let v2 = m446;
                let m443 = v2;
                match m443 {
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
                    Adt0::C15(r444, r445) => {
                        let v13 = (*r444).clone();
                        let v14 = (*r445).clone();
                        if f48(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f62(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(p452: Adt0, v1: Adt0) -> bool {
    let m451 = p452;
    {
        let mut m451 = m451;
        loop {
            let step = {
                let v0 = m451;
                let m448 = v0;
                match m448 {
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
                    Adt0::C15(r449, r450) => {
                        let v12 = (*r449).clone();
                        let v13 = (*r450).clone();
                        if f48(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt30, v1: Adt0, v2: Adt0) -> Adt30 {
    if f62(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt30::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v2), std::rc::Rc::new(v0))
    }
}

pub fn f80(p458: Adt30, v1: Adt0, v2: Adt0, p459: Adt30) -> Adt30 {
    let m457 = (p458, p459);
    {
        let mut m457 = m457;
        loop {
            let step = {
                let (v0, v3) = m457;
                let m453 = v0;
                match m453 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v3)
                    }
                    Adt30::C1(r454, r455, r456) => {
                        let v4 = (*r454).clone();
                        let v5 = (*r455).clone();
                        let v6 = (*r456).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v6, if f48(v4, v1.clone()) {
                            f79(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Fn2, p464: u64, p465: (u64, u64)) -> R<((u64, u64), bool)> {
    let m463 = (p464, p465);
    {
        let mut m463 = m463;
        loop {
            let step = {
                let (v1, v2) = m463;
                let m460 = v1;
                if m460 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m460 - 1;
                    let m462 = {
                        let c461: Fn2 = v0.clone();
                        c461.apply(v2)?
                    };
                    match m462 {
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
                Ok(next) => { m463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f86(v0, v1)
}

pub fn f83(v0: u8, v1: u8) -> R<Option<u64>> {
    let m466 = f87(v0)?;
    match m466 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m467 = f87(v1)?;
            match m467 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    Ok(Some({
                        let a470 = {
                            let a468 = v2;
                            let a469 = 16u64;
                            nat_mul(a468, a469)?
                        };
                        let a471 = v3;
                        nat_add(a470, a471)?
                    }))
                }
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m475 = {
        let a472 = v0;
        let a473 = v1;
        let a474 = v2;
        slice_bytes(a472, a473, a474)
    };
    match m475 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f85(_: u64) -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f86(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m477 = f74(v0, {
        let (h476, _) = v1;
        h476
    });
    match m477 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let m478 = f88(v2)?;
            match m478 {
                None => {
                    Ok(None::<(u64, u64)>)
                }
                Some(v3) => {
                    Ok(Some(({
                        let a480 = {
                            let (h479, _) = v1;
                            h479
                        };
                        let a481 = 1u64;
                        nat_add(a480, a481)?
                    }, {
                        let a485 = {
                            let a483 = {
                                let (_, h482) = v1;
                                h482
                            };
                            let a484 = 10u64;
                            nat_mul(a483, a484)?
                        };
                        let a486 = v3;
                        nat_add(a485, a486)?
                    })))
                }
            }
        }
    }
}

pub fn f87(v0: u8) -> R<Option<u64>> {
    let m487 = f88(v0)?;
    match m487 {
        None => {
            let v3: u64 = {
                let v2: u8 = v0;
                let a532 = {
                    let a508 = {
                        let a496 = {
                            let a490 = {
                                let a488 = v2;
                                let a489 = 128u8;
                                fixed_u8::bit_and(a488, a489)
                            };
                            let a491 = 128u8;
                            if equal(a490, a491) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a497 = {
                            let a494 = {
                                let a492 = v2;
                                let a493 = 64u8;
                                fixed_u8::bit_and(a492, a493)
                            };
                            let a495 = 64u8;
                            if equal(a494, a495) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a496, a497)?
                    };
                    let a509 = {
                        let a506 = {
                            let a500 = {
                                let a498 = v2;
                                let a499 = 32u8;
                                fixed_u8::bit_and(a498, a499)
                            };
                            let a501 = 32u8;
                            if equal(a500, a501) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a507 = {
                            let a504 = {
                                let a502 = v2;
                                let a503 = 16u8;
                                fixed_u8::bit_and(a502, a503)
                            };
                            let a505 = 16u8;
                            if equal(a504, a505) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a506, a507)?
                    };
                    nat_add(a508, a509)?
                };
                let a533 = {
                    let a530 = {
                        let a518 = {
                            let a512 = {
                                let a510 = v2;
                                let a511 = 8u8;
                                fixed_u8::bit_and(a510, a511)
                            };
                            let a513 = 8u8;
                            if equal(a512, a513) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a519 = {
                            let a516 = {
                                let a514 = v2;
                                let a515 = 4u8;
                                fixed_u8::bit_and(a514, a515)
                            };
                            let a517 = 4u8;
                            if equal(a516, a517) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a518, a519)?
                    };
                    let a531 = {
                        let a528 = {
                            let a522 = {
                                let a520 = v2;
                                let a521 = 2u8;
                                fixed_u8::bit_and(a520, a521)
                            };
                            let a523 = 2u8;
                            if equal(a522, a523) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a529 = {
                            let a526 = {
                                let a524 = v2;
                                let a525 = 1u8;
                                fixed_u8::bit_and(a524, a525)
                            };
                            let a527 = 1u8;
                            if equal(a526, a527) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a528, a529)?
                    };
                    nat_add(a530, a531)?
                };
                nat_add(a532, a533)?
            };
            let a534 = v3;
            let a535 = 97u64;
            if nat_lt(a534, a535) {
                Ok(None::<u64>)
            } else {
                let a536 = v3;
                let a537 = 103u64;
                if nat_lt(a536, a537) {
                    Ok(Some({
                        let a538 = v3;
                        let a539 = 87u64;
                        nat_sub(a538, a539)
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f88(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a584 = {
            let a560 = {
                let a548 = {
                    let a542 = {
                        let a540 = v1;
                        let a541 = 128u8;
                        fixed_u8::bit_and(a540, a541)
                    };
                    let a543 = 128u8;
                    if equal(a542, a543) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a549 = {
                    let a546 = {
                        let a544 = v1;
                        let a545 = 64u8;
                        fixed_u8::bit_and(a544, a545)
                    };
                    let a547 = 64u8;
                    if equal(a546, a547) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a548, a549)?
            };
            let a561 = {
                let a558 = {
                    let a552 = {
                        let a550 = v1;
                        let a551 = 32u8;
                        fixed_u8::bit_and(a550, a551)
                    };
                    let a553 = 32u8;
                    if equal(a552, a553) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a559 = {
                    let a556 = {
                        let a554 = v1;
                        let a555 = 16u8;
                        fixed_u8::bit_and(a554, a555)
                    };
                    let a557 = 16u8;
                    if equal(a556, a557) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a558, a559)?
            };
            nat_add(a560, a561)?
        };
        let a585 = {
            let a582 = {
                let a570 = {
                    let a564 = {
                        let a562 = v1;
                        let a563 = 8u8;
                        fixed_u8::bit_and(a562, a563)
                    };
                    let a565 = 8u8;
                    if equal(a564, a565) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a571 = {
                    let a568 = {
                        let a566 = v1;
                        let a567 = 4u8;
                        fixed_u8::bit_and(a566, a567)
                    };
                    let a569 = 4u8;
                    if equal(a568, a569) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a570, a571)?
            };
            let a583 = {
                let a580 = {
                    let a574 = {
                        let a572 = v1;
                        let a573 = 2u8;
                        fixed_u8::bit_and(a572, a573)
                    };
                    let a575 = 2u8;
                    if equal(a574, a575) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a581 = {
                    let a578 = {
                        let a576 = v1;
                        let a577 = 1u8;
                        fixed_u8::bit_and(a576, a577)
                    };
                    let a579 = 1u8;
                    if equal(a578, a579) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a580, a581)?
            };
            nat_add(a582, a583)?
        };
        nat_add(a584, a585)?
    };
    let a586 = v2;
    let a587 = 48u64;
    if nat_lt(a586, a587) {
        Ok(None::<u64>)
    } else {
        let a588 = v2;
        let a589 = 58u64;
        if nat_lt(a588, a589) {
            Ok(Some({
                let a590 = v2;
                let a591 = 48u64;
                nat_sub(a590, a591)
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: u64) -> R<Option<Adt7>> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7)
}
