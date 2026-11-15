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
    C0(Bytes, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>, List<Bytes>),
    C1(List<Adt2>, List<Bytes>, Bytes, Option<Bytes>),
    C2(std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1, p1: Bytes) -> R<Adt1> {
        match self {
            Fn0::F3 => f3(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Result<List<(Bytes, List<Bytes>)>, Adt0>, p1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16(Bytes),
    F92(u8, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn2::F16(k0) => f16(k0.clone(), p0),
            Fn2::F92(k0, k1) => f92(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
}

impl Fn3 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn3::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25,
}

impl Fn4 {
    pub fn apply(&self, p0: u64, p1: Adt2) -> R<u64> {
        match self {
            Fn4::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F39(Bytes),
    F63(Bytes),
    F64(u64, Bytes),
    F81(Bytes, Bytes),
    F86(Bytes, u64, Bytes, u64),
    F93(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F39(k0) => f39(k0.clone(), p0),
            Fn7::F63(k0) => f63(k0.clone(), p0),
            Fn7::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn7::F81(k0, k1) => Ok(f81(k0.clone(), k1.clone(), p0)),
            Fn7::F86(k0, k1, k2, k3) => f86(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn7::F93(k0) => f93(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn8::F72(k0) => f72(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F84,
}

impl Fn9 {
    pub fn apply(&self, p0: Result<List<Bytes>, Bytes>, p1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
        match self {
            Fn9::F84 => f84(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    f1(f2(Fn0::F3, Adt1::C0(List::<Adt2>::nil(), v1.clone()), f4(v0)?)?, v1)
}

pub fn f1(v0: Adt1, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    let m1 = v0;
    match m1 {
        Adt1::C0(v7, _) => {
            let v9: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected ")), f8(f9(v1.clone())?)?), f7(Str::lit(" audit records, found "))), f8(f10(v7.clone())?)?), None::<Bytes>);
            let a2 = f10(v7.clone())?;
            let a3 = f9(v1)?;
            if nat_eq(a2, a3) {
                Ok(f11(Fn1::F12, Ok::<List<(Bytes, List<Bytes>)>, Adt0>(List::<(Bytes, List<Bytes>)>::nil()), f13(v7)))
            } else {
                Ok(v9)
            }
        }
        Adt1::C1(_, _, v5, v6) => {
            Ok(f5(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
        }
        Adt1::C2(r4) => {
            let v2 = (*r4).clone();
            Ok(Err::<List<(Bytes, List<Bytes>)>, Adt0>(v2))
        }
    }
}

pub fn f2(v0: Fn0, p8: Adt1, p9: List<Bytes>) -> R<Adt1> {
    let m7 = (p8, p9);
    {
        let mut m7 = m7;
        loop {
            let step = {
                let (v1, v2) = m7;
                let m5 = v2;
                match m5.uncons() {
                    None => {
                        Err::<(Adt1, List<Bytes>), R<Adt1>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Bytes>), R<Adt1>>(({
                            let c6: Fn0 = v0.clone();
                            c6.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m7 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Bytes) -> R<Adt1> {
    f14(v0, v1)
}

pub fn f4(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a10 = v0.clone();
        length_bytes(a10)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h11, _) = f15(Fn2::F16(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h11
    };
    let v3: u64 = {
        let (h12, _) = {
            let (_, h13) = v2.clone();
            h13
        };
        h12
    };
    let v4: List<Bytes> = {
        let (_, h14) = {
            let (_, h15) = v2;
            h15
        };
        h14
    };
    let a16 = v3;
    let a17 = v1;
    if nat_lt(a16, a17) {
        Ok(f17(List::cons(f18(v0, v3, {
            let a18 = v1;
            let a19 = v3;
            nat_sub(a18, a19)
        }), v4)))
    } else {
        Ok(f17(v4))
    }
}

pub fn f5(v0: Bytes, v1: Option<Bytes>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    Err::<List<(Bytes, List<Bytes>)>, Adt0>(f19(v0, v1))
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    let a20 = v0;
    let a21 = v1;
    append_bytes(a20, a21)
}

pub fn f7(v0: Str) -> Bytes {
    let a22 = v0;
    utf8_encode(a22)
}

pub fn f8(v0: u64) -> R<Bytes> {
    let v1: Bytes = f7(Str::lit("0"));
    let v2: Bytes = f20({
        let a59 = f21(v0, 10000000000000000000u64);
        let a60 = {
            let a57 = f21(v0, 1000000000000000000u64);
            let a58 = {
                let a55 = f21(v0, 100000000000000000u64);
                let a56 = {
                    let a53 = f21(v0, 10000000000000000u64);
                    let a54 = {
                        let a51 = f21(v0, 1000000000000000u64);
                        let a52 = {
                            let a49 = f21(v0, 100000000000000u64);
                            let a50 = {
                                let a47 = f21(v0, 10000000000000u64);
                                let a48 = {
                                    let a45 = f21(v0, 1000000000000u64);
                                    let a46 = {
                                        let a43 = f21(v0, 100000000000u64);
                                        let a44 = {
                                            let a41 = f21(v0, 10000000000u64);
                                            let a42 = {
                                                let a39 = f21(v0, 1000000000u64);
                                                let a40 = {
                                                    let a37 = f21(v0, 100000000u64);
                                                    let a38 = {
                                                        let a35 = f21(v0, 10000000u64);
                                                        let a36 = {
                                                            let a33 = f21(v0, 1000000u64);
                                                            let a34 = {
                                                                let a31 = f21(v0, 100000u64);
                                                                let a32 = {
                                                                    let a29 = f21(v0, 10000u64);
                                                                    let a30 = {
                                                                        let a27 = f21(v0, 1000u64);
                                                                        let a28 = {
                                                                            let a25 = f21(v0, 100u64);
                                                                            let a26 = {
                                                                                let a23 = f21(v0, 10u64);
                                                                                let a24 = f21(v0, 1u64);
                                                                                append_bytes(a23, a24)
                                                                            };
                                                                            append_bytes(a25, a26)
                                                                        };
                                                                        append_bytes(a27, a28)
                                                                    };
                                                                    append_bytes(a29, a30)
                                                                };
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
    })?;
    let a61 = v0;
    let a62 = 0u64;
    if nat_eq(a61, a62) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f9(v0: List<Bytes>) -> R<u64> {
    f22(Fn3::F23, 0u64, v0)
}

pub fn f10(v0: List<Adt2>) -> R<u64> {
    f24(Fn4::F25, 0u64, v0)
}

pub fn f11(v0: Fn1, p66: Result<List<(Bytes, List<Bytes>)>, Adt0>, p67: List<Adt2>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v2;
                match m63.uncons() {
                    None => {
                        Err::<(Result<List<(Bytes, List<Bytes>)>, Adt0>, List<Adt2>), Result<List<(Bytes, List<Bytes>)>, Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<(Bytes, List<Bytes>)>, Adt0>, List<Adt2>), Result<List<(Bytes, List<Bytes>)>, Adt0>>(({
                            let c64: Fn1 = v0.clone();
                            c64.apply(v1, v3)
                        }, v4))
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

pub fn f12(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    f26(v0, v1)
}

pub fn f13(v0: List<Adt2>) -> List<Adt2> {
    f27(Fn5::F28, List::<Adt2>::nil(), v0)
}

pub fn f14(v0: Adt1, v1: Bytes) -> R<Adt1> {
    let m68 = v0.clone();
    match m68 {
        Adt1::C0(v8, v9) => {
            let a71 = 0u64;
            let a72 = {
                let a70 = f32(v1.clone())?;
                length_bytes(a70)
            };
            if nat_eq(a71, a72) {
                Ok(v0)
            } else {
                let m73 = f33(f32(v1.clone())?)?;
                match m73 {
                    None => {
                        Ok(f31(f6(f7(Str::lit("unrecognized audit line: ")), f32(v1)?), f34(v9)))
                    }
                    Some(v10) => {
                        f30(v8, v9.clone(), v10, f34(v9))
                    }
                }
            }
        }
        Adt1::C1(v3, v4, v5, v6) => {
            let m69 = f29(v1)?;
            match m69 {
                None => {
                    Ok(f31(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
                }
                Some(v7) => {
                    f30(v3, v4, f6(f6(v5, f7(Str::lit(" "))), v7), v6)
                }
            }
        }
        Adt1::C2(_) => {
            Ok(v0)
        }
    }
}

pub fn f15(v0: Fn2, p78: u64, p79: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m77 = (p78, p79);
    {
        let mut m77 = m77;
        loop {
            let step = {
                let (v1, v2) = m77;
                let m74 = v1;
                if m74 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m74 - 1;
                    let m76 = {
                        let c75: Fn2 = v0.clone();
                        c75.apply(v2.clone())?
                    };
                    match m76 {
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
                Ok(next) => { m77 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f35(v0, v1)
}

pub fn f17(v0: List<Bytes>) -> List<Bytes> {
    f36(Fn6::F37, List::<Bytes>::nil(), v0)
}

pub fn f18(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m83 = {
        let a80 = v0;
        let a81 = v1;
        let a82 = v2;
        slice_bytes(a80, a81, a82)
    };
    match m83 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f19(v0: Bytes, v1: Option<Bytes>) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h84, _) = f38(Fn7::F39(v0.clone()), {
            let a85 = v0.clone();
            length_bytes(a85)
        }, 0u64)?;
        h84
    };
    Ok(f18(v0.clone(), v1, {
        let a87 = {
            let a86 = v0;
            length_bytes(a86)
        };
        let a88 = v1;
        nat_sub(a87, a88)
    }))
}

pub fn f21(v0: u64, v1: u64) -> Bytes {
    f18(f7(Str::lit("0123456789")), {
        let a92 = {
            let a89 = v0;
            let a90 = v1;
            let a91 = 0u64;
            nat_quot(a89, a90, a91)
        };
        let a93 = 10u64;
        let a94 = 0u64;
        nat_rem(a92, a93, a94)
    }, 1u64)
}

pub fn f22(v0: Fn3, p98: u64, p99: List<Bytes>) -> R<u64> {
    let m97 = (p98, p99);
    {
        let mut m97 = m97;
        loop {
            let step = {
                let (v1, v2) = m97;
                let m95 = v2;
                match m95.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c96: Fn3 = v0.clone();
                            c96.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m97 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64, _: Bytes) -> R<u64> {
    let a100 = v0;
    let a101 = 1u64;
    nat_add(a100, a101)
}

pub fn f24(v0: Fn4, p105: u64, p106: List<Adt2>) -> R<u64> {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m102 = v2;
                match m102.uncons() {
                    None => {
                        Err::<(u64, List<Adt2>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt2>), R<u64>>(({
                            let c103: Fn4 = v0.clone();
                            c103.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: u64, _: Adt2) -> R<u64> {
    let a107 = v0;
    let a108 = 1u64;
    nat_add(a107, a108)
}

pub fn f26(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m109 = v0.clone();
    match m109 {
        Ok(v3) => {
            let v5: Bytes = {
                let m111 = match v1.clone() {
                    Adt2::C0(_, _, h110) => {
                        h110
                    }
                };
                match m111 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v4) => {
                        v4
                    }
                }
            };
            let v6: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f7(Str::lit("duplicate record for ")), f40(match v1.clone() {
                Adt2::C0(h112, _, _) => {
                    h112
                }
            })), Some(v5.clone()));
            let v7: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected a record for ")), f40(v5.clone())), f7(Str::lit(", found "))), f40(match v1.clone() {
                Adt2::C0(h113, _, _) => {
                    h113
                }
            })), Some(v5.clone()));
            let a115 = match v1.clone() {
                Adt2::C0(h114, _, _) => {
                    h114
                }
            };
            let a116 = v5;
            if equal(a115, a116) {
                let m119 = f41(match v1.clone() {
                    Adt2::C0(h117, _, _) => {
                        h117
                    }
                }, match v1 {
                    Adt2::C0(_, h118, _) => {
                        h118
                    }
                }, v3);
                match m119 {
                    None => {
                        v6
                    }
                    Some(v8) => {
                        Ok::<List<(Bytes, List<Bytes>)>, Adt0>(v8)
                    }
                }
            } else {
                v7
            }
        }
        Err(_) => {
            v0
        }
    }
}

pub fn f27(v0: Fn5, p123: List<Adt2>, p124: List<Adt2>) -> List<Adt2> {
    let m122 = (p123, p124);
    {
        let mut m122 = m122;
        loop {
            let step = {
                let (v1, v2) = m122;
                let m120 = v2;
                match m120.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c121: Fn5 = v0.clone();
                            c121.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f29(v0: Bytes) -> R<Option<Bytes>> {
    let v1: Bytes = f32(v0.clone())?;
    if f42(v0)? {
        let a126 = 0u64;
        let a127 = {
            let a125 = v1.clone();
            length_bytes(a125)
        };
        if nat_eq(a126, a127) {
            Ok(None::<Bytes>)
        } else {
            Ok(Some(v1))
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f30(v0: List<Adt2>, v1: List<Bytes>, v2: Bytes, v3: Option<Bytes>) -> R<Adt1> {
    if f43(v2.clone()) {
        let m128 = f44(v2)?;
        match m128 {
            Ok(v5) => {
                Ok(Adt1::C0(List::cons(Adt2::C0({
                    let (h129, _) = v5;
                    h129
                }, {
                    let (_, h130) = v5;
                    h130
                }, v3), v0), f45(v1)))
            }
            Err(v4) => {
                Ok(f31(v4, v3))
            }
        }
    } else {
        Ok(Adt1::C1(v0, v1, v2, v3))
    }
}

pub fn f31(v0: Bytes, v1: Option<Bytes>) -> Adt1 {
    Adt1::C2(std::rc::Rc::new(f19(v0, v1)))
}

pub fn f32(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f46(v0.clone())?;
    let v2: u64 = f47(v0.clone(), v1)?;
    Ok(f18(v0, v1, {
        let a131 = v2;
        let a132 = v1;
        nat_sub(a131, a132)
    }))
}

pub fn f33(v0: Bytes) -> R<Option<Bytes>> {
    if f48(v0.clone(), f7(Str::lit("'"))) {
        Ok(Some(v0))
    } else {
        let m133 = f49(v0.clone(), f7(Str::lit("info: ")));
        match m133 {
            None => {
                f50(v0)
            }
            Some(v1) => {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f34(v0: List<Bytes>) -> Option<Bytes> {
    let m134 = v0;
    match m134.uncons() {
        None => {
            None::<Bytes>
        }
        Some((v1, _)) => {
            Some(v1)
        }
    }
}

pub fn f35(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h135, _) = v1.clone();
        h135
    };
    let v3: u64 = {
        let (h136, _) = {
            let (_, h137) = v1.clone();
            h137
        };
        h136
    };
    let v4: List<Bytes> = {
        let (_, h138) = {
            let (_, h139) = v1;
            h139
        };
        h138
    };
    let m140 = f51(v0.clone(), v2);
    match m140 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a141 = v5;
            let a142 = 10u8;
            if equal(a141, a142) {
                Ok(Some(({
                    let a143 = v2;
                    let a144 = 1u64;
                    nat_add(a143, a144)?
                }, ({
                    let a145 = v2;
                    let a146 = 1u64;
                    nat_add(a145, a146)?
                }, List::cons(f52(f18(v0, v3, {
                    let a147 = v2;
                    let a148 = v3;
                    nat_sub(a147, a148)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a149 = v2;
                    let a150 = 1u64;
                    nat_add(a149, a150)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f36(v0: Fn6, p154: List<Bytes>, p155: List<Bytes>) -> List<Bytes> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v1, v2) = m153;
                let m151 = v2;
                match m151.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c152: Fn6 = v0.clone();
                            c152.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f38(v0: Fn7, p160: u64, p161: u64) -> R<(u64, bool)> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m156 = v1;
                if m156 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m156 - 1;
                    let m158 = {
                        let c157: Fn7 = v0.clone();
                        c157.apply(v2)?
                    };
                    match m158 {
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
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f40(v0: Bytes) -> Bytes {
    f6(f6(f7(Str::lit("`")), v0), f7(Str::lit("`")))
}

pub fn f41(v0: Bytes, v1: List<Bytes>, v2: List<(Bytes, List<Bytes>)>) -> Option<List<(Bytes, List<Bytes>)>> {
    let m162 = v2;
    match m162.uncons() {
        None => {
            Some(List::cons((v0, v1), List::<(Bytes, List<Bytes>)>::nil()))
        }
        Some((v3, v4)) => {
            let a164 = v0.clone();
            let a165 = {
                let (h163, _) = v3.clone();
                h163
            };
            if equal(a164, a165) {
                None::<List<(Bytes, List<Bytes>)>>
            } else {
                if f54(v0.clone(), {
                    let (h166, _) = v3.clone();
                    h166
                }) {
                    Some(List::cons((v0, v1), List::cons(v3, v4)))
                } else {
                    let m167 = f41(v0, v1, v4);
                    match m167 {
                        None => {
                            None::<List<(Bytes, List<Bytes>)>>
                        }
                        Some(v5) => {
                            Some(List::cons(v3, v5))
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Bytes) -> R<bool> {
    let a168 = 0u64;
    let a169 = f55(v0, 0u64)?;
    Ok(nat_lt(a168, a169))
}

pub fn f43(v0: Bytes) -> bool {
    let m170 = f56(v0.clone(), f7(Str::lit("]")));
    match m170 {
        None => {
            let m171 = f56(v0, f7(Str::lit(" does not depend on any axioms")));
            match m171 {
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
    }
}

pub fn f44(v0: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m172 = f49(v0.clone(), f7(Str::lit("'")));
    match m172 {
        None => {
            Ok(f57(v0))
        }
        Some(v1) => {
            let m173 = f56(v1.clone(), f7(Str::lit("' does not depend on any axioms")));
            match m173 {
                None => {
                    let m174 = f56(v1, f7(Str::lit("]")));
                    match m174 {
                        None => {
                            Ok(f57(v0))
                        }
                        Some(v5) => {
                            f62(v0, v5)
                        }
                    }
                }
                Some(v2) => {
                    let v3: Result<(Bytes, List<Bytes>), Bytes> = f58(v2.clone(), List::<Bytes>::nil());
                    let v4: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v2.clone()));
                    if f61(v2)? {
                        Ok(v3)
                    } else {
                        Ok(v4)
                    }
                }
            }
        }
    }
}

pub fn f45(v0: List<Bytes>) -> List<Bytes> {
    let m175 = v0;
    match m175.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f46(v0: Bytes) -> R<u64> {
    let (h176, _) = f38(Fn7::F63(v0.clone()), {
        let a177 = v0;
        length_bytes(a177)
    }, 0u64)?;
    Ok(h176)
}

pub fn f47(v0: Bytes, v1: u64) -> R<u64> {
    let (h178, _) = f38(Fn7::F64(v1, v0.clone()), {
        let a179 = v0.clone();
        length_bytes(a179)
    }, {
        let a180 = v0;
        length_bytes(a180)
    })?;
    Ok(h178)
}

pub fn f48(v0: Bytes, v1: Bytes) -> bool {
    let m185 = {
        let a182 = v0;
        let a183 = 0u64;
        let a184 = {
            let a181 = v1.clone();
            length_bytes(a181)
        };
        slice_bytes(a182, a183, a184)
    };
    match m185 {
        None => {
            false
        }
        Some(v2) => {
            let a186 = v2;
            let a187 = v1;
            equal(a186, a187)
        }
    }
}

pub fn f49(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f48(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), {
            let a188 = v1.clone();
            length_bytes(a188)
        }, {
            let a191 = {
                let a189 = v0;
                length_bytes(a189)
            };
            let a192 = {
                let a190 = v1;
                length_bytes(a190)
            };
            nat_sub(a191, a192)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f50(v0: Bytes) -> R<Option<Bytes>> {
    let m193 = f65(v0, f7(Str::lit(":")))?;
    match m193 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m195 = f65({
                let (_, h194) = v1;
                h194
            }, f7(Str::lit(":")))?;
            match m195 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let m197 = f65({
                        let (_, h196) = v2;
                        h196
                    }, f7(Str::lit(":")))?;
                    match m197 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v3) => {
                            let v4: Bytes = {
                                let (h198, _) = v2;
                                h198
                            };
                            let v5: Bytes = {
                                let (h199, _) = v3;
                                h199
                            };
                            let v6: Bytes = {
                                let (_, h200) = v3;
                                h200
                            };
                            let v7: Option<Bytes> = f66(v5, v6)?;
                            if f67(v4)? {
                                Ok(v7)
                            } else {
                                Ok(None::<Bytes>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> Option<u8> {
    let a201 = v0;
    let a202 = v1;
    index_bytes(a201, a202)
}

pub fn f52(v0: Bytes) -> Bytes {
    if f68(v0.clone(), Bytes::lit(&[0x0d])) {
        f18(v0.clone(), 0u64, {
            let a204 = {
                let a203 = v0;
                length_bytes(a203)
            };
            let a205 = 1u64;
            nat_sub(a204, a205)
        })
    } else {
        v0
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f69(v0, v1, 48u8) {
        Ok(Some({
            let a206 = v1;
            let a207 = 1u64;
            nat_add(a206, a207)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> bool {
    let a212 = {
        let a208 = v0;
        let a209 = v1;
        compare_bytes(a208, a209)
    };
    let a213 = {
        let a210 = Bytes::lit(&[]);
        let a211 = Bytes::lit(&[0x01]);
        compare_bytes(a210, a211)
    };
    equal(a212, a213)
}

pub fn f55(v0: Bytes, v1: u64) -> R<u64> {
    if if f70(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f69(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f69(v0.clone(), v1, 194u8) {
            if if f69(v0.clone(), {
                let a214 = v1;
                let a215 = 1u64;
                nat_add(a214, a215)?
            }, 133u8) {
                true
            } else {
                f69(v0, {
                    let a216 = v1;
                    let a217 = 1u64;
                    nat_add(a216, a217)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f69(v0.clone(), v1, 225u8) {
                if if f69(v0.clone(), {
                    let a218 = v1;
                    let a219 = 1u64;
                    nat_add(a218, a219)?
                }, 154u8) {
                    f69(v0, {
                        let a220 = v1;
                        let a221 = 2u64;
                        nat_add(a220, a221)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f69(v0.clone(), v1, 226u8) {
                    if f69(v0.clone(), {
                        let a222 = v1;
                        let a223 = 1u64;
                        nat_add(a222, a223)?
                    }, 128u8) {
                        if if f70(v0.clone(), {
                            let a224 = v1;
                            let a225 = 2u64;
                            nat_add(a224, a225)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f69(v0.clone(), {
                                let a226 = v1;
                                let a227 = 2u64;
                                nat_add(a226, a227)?
                            }, 168u8) {
                                true
                            } else {
                                if f69(v0.clone(), {
                                    let a228 = v1;
                                    let a229 = 2u64;
                                    nat_add(a228, a229)?
                                }, 169u8) {
                                    true
                                } else {
                                    f69(v0, {
                                        let a230 = v1;
                                        let a231 = 2u64;
                                        nat_add(a230, a231)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f69(v0.clone(), {
                            let a232 = v1;
                            let a233 = 1u64;
                            nat_add(a232, a233)?
                        }, 129u8) {
                            f69(v0, {
                                let a234 = v1;
                                let a235 = 2u64;
                                nat_add(a234, a235)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f69(v0.clone(), v1, 227u8) {
                        if if f69(v0.clone(), {
                            let a236 = v1;
                            let a237 = 1u64;
                            nat_add(a236, a237)?
                        }, 128u8) {
                            f69(v0, {
                                let a238 = v1;
                                let a239 = 2u64;
                                nat_add(a238, a239)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f68(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), 0u64, {
            let a242 = {
                let a240 = v0;
                length_bytes(a240)
            };
            let a243 = {
                let a241 = v1;
                length_bytes(a241)
            };
            nat_sub(a242, a243)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f57(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    f59(f6(f7(Str::lit("unrecognized axiom payload: ")), v0))
}

pub fn f58(v0: Bytes, v1: List<Bytes>) -> Result<(Bytes, List<Bytes>), Bytes> {
    Ok::<(Bytes, List<Bytes>), Bytes>((v0, v1))
}

pub fn f59(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    Err::<(Bytes, List<Bytes>), Bytes>(v0)
}

pub fn f60(v0: Bytes) -> Bytes {
    f6(f40(v0), f7(Str::lit(" is not a Lean name")))
}

pub fn f61(v0: Bytes) -> R<bool> {
    let a247 = {
        let (_, h244) = {
            let (h245, _) = f71(Fn8::F72(v0.clone()), {
                let a246 = v0;
                length_bytes(a246)
            }, (0u64, 0u64))?;
            h245
        };
        h244
    };
    let a248 = 1u64;
    Ok(nat_eq(a247, a248))
}

pub fn f62(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m249 = f73(v1, f7(Str::lit("' depends on axioms: [")))?;
    match m249 {
        None => {
            Ok(f57(v0))
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h250, _) = v2;
                h250
            };
            let v4: Bytes = {
                let (_, h251) = v2;
                h251
            };
            let v5: Result<(Bytes, List<Bytes>), Bytes> = f74(v3.clone(), v4)?;
            let v6: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v3.clone()));
            if f61(v3)? {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f63(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a252 = 0u64;
    let a253 = f55(v0.clone(), v1)?;
    if nat_lt(a252, a253) {
        Ok(Some({
            let a254 = v1;
            let a255 = f55(v0, v1)?;
            nat_add(a254, a255)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f64(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a256 = 0u64;
    let a257 = f75(v1.clone(), v2)?;
    if if nat_lt(a256, a257) {
        let a260 = {
            let a258 = v0;
            let a259 = f75(v1.clone(), v2)?;
            nat_add(a258, a259)?
        };
        let a261 = v2;
        nat_le(a260, a261)
    } else {
        false
    } {
        Ok(Some({
            let a262 = v2;
            let a263 = f75(v1, v2)?;
            nat_sub(a262, a263)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f65(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let m264 = f76(v0.clone(), v1.clone())?;
    match m264 {
        None => {
            Ok(None::<(Bytes, Bytes)>)
        }
        Some(v2) => {
            Ok(Some((f18(v0.clone(), 0u64, v2), f18(v0.clone(), {
                let a266 = v2;
                let a267 = {
                    let a265 = v1.clone();
                    length_bytes(a265)
                };
                nat_add(a266, a267)?
            }, {
                let a272 = {
                    let a268 = v0;
                    length_bytes(a268)
                };
                let a273 = {
                    let a270 = v2;
                    let a271 = {
                        let a269 = v1;
                        length_bytes(a269)
                    };
                    nat_add(a270, a271)?
                };
                nat_sub(a272, a273)
            }))))
        }
    }
}

pub fn f66(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m274 = f65(v0.clone(), f7(Str::lit("-")))?;
    match m274 {
        None => {
            let v9: Option<Bytes> = f77(v1);
            if f67(v0)? {
                Ok(v9)
            } else {
                Ok(None::<Bytes>)
            }
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h275, _) = v2;
                h275
            };
            let v4: Bytes = {
                let (_, h276) = v2;
                h276
            };
            let m277 = f65(v1, f7(Str::lit(":")))?;
            match m277 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v5) => {
                    let v6: Bytes = {
                        let (h278, _) = v5;
                        h278
                    };
                    let v7: Bytes = {
                        let (_, h279) = v5;
                        h279
                    };
                    let v8: Option<Bytes> = f77(v7);
                    if f67(v3)? {
                        if f67(v4)? {
                            if f67(v6)? {
                                Ok(v8)
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
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<bool> {
    f78(f32(v0)?)
}

pub fn f68(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a280 = v0.clone();
        length_bytes(a280)
    };
    let v3: u64 = {
        let a281 = v1.clone();
        length_bytes(a281)
    };
    let a282 = v3;
    let a283 = v2;
    if nat_le(a282, a283) {
        let m289 = {
            let a286 = v0;
            let a287 = {
                let a284 = v2;
                let a285 = v3;
                nat_sub(a284, a285)
            };
            let a288 = v3;
            slice_bytes(a286, a287, a288)
        };
        match m289 {
            None => {
                false
            }
            Some(v4) => {
                let a290 = v4;
                let a291 = v1;
                equal(a290, a291)
            }
        }
    } else {
        false
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m292 = f51(v0, v1);
    match m292 {
        None => {
            false
        }
        Some(v3) => {
            let a293 = v3;
            let a294 = v2;
            equal(a293, a294)
        }
    }
}

pub fn f70(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m295 = f51(v0, v1);
    match m295 {
        None => {
            false
        }
        Some(v4) => {
            f79(v4, v2, v3)
        }
    }
}

pub fn f71(v0: Fn8, p300: u64, p301: (u64, u64)) -> R<((u64, u64), bool)> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v1, v2) = m299;
                let m296 = v1;
                if m296 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m296 - 1;
                    let m298 = {
                        let c297: Fn8 = v0.clone();
                        c297.apply(v2)?
                    };
                    match m298 {
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
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f80(v0, v1)
}

pub fn f73(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a302 = v0.clone();
        length_bytes(a302)
    };
    let v3: u64 = {
        let a303 = v1.clone();
        length_bytes(a303)
    };
    let a304 = v2;
    let a305 = v3;
    if nat_lt(a304, a305) {
        Ok(None::<(Bytes, Bytes)>)
    } else {
        let v4: u64 = {
            let (h306, _) = f38(Fn7::F81(v1.clone(), v0.clone()), {
                let a307 = v2;
                let a308 = 1u64;
                nat_add(a307, a308)?
            }, {
                let a309 = v2;
                let a310 = v3;
                nat_sub(a309, a310)
            })?;
            h306
        };
        if f82(v0.clone(), v1, v4) {
            Ok(Some((f18(v0.clone(), 0u64, v4), f18(v0, {
                let a311 = v4;
                let a312 = v3;
                nat_add(a311, a312)?
            }, {
                let a315 = v2;
                let a316 = {
                    let a313 = v4;
                    let a314 = v3;
                    nat_add(a313, a314)?
                };
                nat_sub(a315, a316)
            }))))
        } else {
            Ok(None::<(Bytes, Bytes)>)
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m317 = f83(Fn9::F84, Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()), f85(v1, 44u8)?)?;
    match m317 {
        Ok(v3) => {
            let m318 = v3.clone();
            match m318.uncons() {
                None => {
                    Ok(f59(f7(Str::lit("an axiom list cannot be empty"))))
                }
                Some((_, _)) => {
                    Ok(f58(v0, v3))
                }
            }
        }
        Err(v2) => {
            Ok(f59(v2))
        }
    }
}

pub fn f75(v0: Bytes, v1: u64) -> R<u64> {
    let a319 = v1;
    let a320 = 0u64;
    if nat_eq(a319, a320) {
        Ok(0u64)
    } else {
        let a323 = f55(v0.clone(), {
            let a321 = v1;
            let a322 = 1u64;
            nat_sub(a321, a322)
        })?;
        let a324 = 1u64;
        if nat_eq(a323, a324) {
            Ok(1u64)
        } else {
            let a325 = 2u64;
            let a326 = v1;
            if if nat_le(a325, a326) {
                let a329 = f55(v0.clone(), {
                    let a327 = v1;
                    let a328 = 2u64;
                    nat_sub(a327, a328)
                })?;
                let a330 = 2u64;
                nat_eq(a329, a330)
            } else {
                false
            } {
                Ok(2u64)
            } else {
                let a331 = 3u64;
                let a332 = v1;
                if if nat_le(a331, a332) {
                    let a335 = f55(v0, {
                        let a333 = v1;
                        let a334 = 3u64;
                        nat_sub(a333, a334)
                    })?;
                    let a336 = 3u64;
                    nat_eq(a335, a336)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a337 = v0.clone();
        length_bytes(a337)
    };
    let v3: u64 = {
        let a338 = v1.clone();
        length_bytes(a338)
    };
    let v4: u64 = {
        let (h339, _) = f38(Fn7::F86(v1.clone(), v3, v0.clone(), v2), {
            let a340 = v2;
            let a341 = 1u64;
            nat_add(a340, a341)?
        }, 0u64)?;
        h339
    };
    if f82(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes) -> Option<Bytes> {
    let m342 = f49(v0, f7(Str::lit(" ")));
    match m342 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f49(v1, f7(Str::lit("info: ")))
        }
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    let v2: Bytes = {
        let m343 = f49(v0.clone(), f7(Str::lit("+")));
        match m343 {
            None => {
                v0
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: bool = f87(f20(v2.clone())?);
    let a345 = 0u64;
    let a346 = {
        let a344 = v2.clone();
        length_bytes(a344)
    };
    if nat_lt(a345, a346) {
        if f88(v2)? {
            Ok(v3)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u8, v1: u8, v2: u8) -> bool {
    let m349 = {
        let a347 = v0;
        let a348 = v1;
        fixed_u8::checked_sub(a347, a348)
    };
    match m349 {
        None => {
            false
        }
        Some(_) => {
            let m352 = {
                let a350 = v2;
                let a351 = v0;
                fixed_u8::checked_sub(a350, a351)
            };
            match m352 {
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

pub fn f80(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a354 = {
        let (_, h353) = v1;
        h353
    };
    let a355 = 2u64;
    if nat_eq(a354, a355) {
        Ok(None::<(u64, u64)>)
    } else {
        let m357 = f51(v0, {
            let (h356, _) = v1;
            h356
        });
        match m357 {
            None => {
                Ok(None::<(u64, u64)>)
            }
            Some(v2) => {
                Ok(Some(({
                    let a359 = {
                        let (h358, _) = v1;
                        h358
                    };
                    let a360 = 1u64;
                    nat_add(a359, a360)?
                }, f89({
                    let (_, h361) = v1;
                    h361
                }, v2))))
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    f90(v1, v0, v2)
}

pub fn f82(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m366 = {
        let a363 = v0;
        let a364 = v2;
        let a365 = {
            let a362 = v1.clone();
            length_bytes(a362)
        };
        slice_bytes(a363, a364, a365)
    };
    match m366 {
        None => {
            false
        }
        Some(v3) => {
            let a367 = v3;
            let a368 = v1;
            equal(a367, a368)
        }
    }
}

pub fn f83(v0: Fn9, p372: Result<List<Bytes>, Bytes>, p373: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m371 = (p372, p373);
    {
        let mut m371 = m371;
        loop {
            let step = {
                let (v1, v2) = m371;
                let m369 = v2;
                match m369.uncons() {
                    None => {
                        Err::<(Result<List<Bytes>, Bytes>, List<Bytes>), R<Result<List<Bytes>, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Bytes>, Bytes>, List<Bytes>), R<Result<List<Bytes>, Bytes>>>(({
                            let c370: Fn9 = v0.clone();
                            c370.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m371 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    f91(v0, v1)
}

pub fn f85(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a374 = v0.clone();
        length_bytes(a374)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h375, _) = f15(Fn2::F92(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h375
    };
    let v4: u64 = {
        let (h376, _) = {
            let (_, h377) = v3.clone();
            h377
        };
        h376
    };
    Ok(f17(List::cons(f18(v0, v4, {
        let a378 = v2;
        let a379 = v4;
        nat_sub(a378, a379)
    }), {
        let (_, h380) = {
            let (_, h381) = v3;
            h381
        };
        h380
    })))
}

pub fn f86(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f82(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a384 = {
            let a382 = v4;
            let a383 = v1;
            nat_add(a382, a383)?
        };
        let a385 = v3;
        if nat_lt(a384, a385) {
            Ok(Some({
                let a386 = v4;
                let a387 = 1u64;
                nat_add(a386, a387)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f87(v0: Bytes) -> bool {
    let a389 = {
        let a388 = v0.clone();
        length_bytes(a388)
    };
    let a390 = 20u64;
    if nat_lt(a389, a390) {
        true
    } else {
        let a392 = {
            let a391 = v0.clone();
            length_bytes(a391)
        };
        let a393 = 20u64;
        if nat_eq(a392, a393) {
            let a400 = {
                let a398 = {
                    let a394 = v0;
                    let a395 = f7(Str::lit("18446744073709551615"));
                    compare_bytes(a394, a395)
                };
                let a399 = {
                    let a396 = Bytes::lit(&[0x01]);
                    let a397 = Bytes::lit(&[0x00]);
                    compare_bytes(a396, a397)
                };
                equal(a398, a399)
            };
            bool_not(a400)
        } else {
            false
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    let a404 = {
        let (h401, _) = f38(Fn7::F93(v0.clone()), {
            let a402 = v0.clone();
            length_bytes(a402)
        }, 0u64)?;
        h401
    };
    let a405 = {
        let a403 = v0;
        length_bytes(a403)
    };
    Ok(nat_eq(a404, a405))
}

pub fn f89(v0: u64, v1: u8) -> u64 {
    let a406 = v0;
    let a407 = 2u64;
    if nat_eq(a406, a407) {
        2u64
    } else {
        let a408 = v1;
        let a409 = 46u8;
        if equal(a408, a409) {
            let a410 = v0;
            let a411 = 1u64;
            if nat_eq(a410, a411) {
                0u64
            } else {
                2u64
            }
        } else {
            if f94(v1) {
                1u64
            } else {
                let a412 = v0;
                let a413 = 1u64;
                if nat_eq(a412, a413) {
                    if f95(v1) {
                        1u64
                    } else {
                        2u64
                    }
                } else {
                    2u64
                }
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    if f82(v0, v1, v2) {
        None::<u64>
    } else {
        let a414 = 0u64;
        let a415 = v2;
        if nat_lt(a414, a415) {
            Some({
                let a416 = v2;
                let a417 = 1u64;
                nat_sub(a416, a417)
            })
        } else {
            None::<u64>
        }
    }
}

pub fn f91(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    let m418 = v0.clone();
    match m418 {
        Ok(v3) => {
            let v4: Bytes = f32(v1)?;
            if f61(v4.clone())? {
                let m419 = f96(v4.clone(), v3);
                match m419 {
                    None => {
                        Ok(Err::<List<Bytes>, Bytes>(f6(f7(Str::lit("duplicate axiom name ")), f40(v4))))
                    }
                    Some(v5) => {
                        Ok(Ok::<List<Bytes>, Bytes>(v5))
                    }
                }
            } else {
                Ok(Err::<List<Bytes>, Bytes>(f60(v4)))
            }
        }
        Err(_) => {
            Ok(v0)
        }
    }
}

pub fn f92(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f97(v1, v0, v2)
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f94(v0: u8) -> bool {
    if f79(v0, 65u8, 90u8) {
        true
    } else {
        if f79(v0, 97u8, 122u8) {
            true
        } else {
            let a420 = v0;
            let a421 = 95u8;
            equal(a420, a421)
        }
    }
}

pub fn f95(v0: u8) -> bool {
    if f94(v0) {
        true
    } else {
        if f79(v0, 48u8, 57u8) {
            true
        } else {
            let a422 = v0;
            let a423 = 63u8;
            if equal(a422, a423) {
                true
            } else {
                let a424 = v0;
                let a425 = 39u8;
                equal(a424, a425)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m426 = v1;
    match m426.uncons() {
        None => {
            Some(List::cons(v0, List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let a427 = v0.clone();
            let a428 = v2.clone();
            if equal(a427, a428) {
                None::<List<Bytes>>
            } else {
                if f54(v0.clone(), v2.clone()) {
                    Some(List::cons(v0, List::cons(v2, v3)))
                } else {
                    let m429 = f96(v0, v3);
                    match m429 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(List::cons(v2, v4))
                        }
                    }
                }
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h430, _) = v2.clone();
        h430
    };
    let v4: u64 = {
        let (h431, _) = {
            let (_, h432) = v2.clone();
            h432
        };
        h431
    };
    let v5: List<Bytes> = {
        let (_, h433) = {
            let (_, h434) = v2;
            h434
        };
        h433
    };
    let m435 = f51(v0.clone(), v3);
    match m435 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a436 = v6;
            let a437 = v1;
            if equal(a436, a437) {
                Ok(Some(({
                    let a438 = v3;
                    let a439 = 1u64;
                    nat_add(a438, a439)?
                }, ({
                    let a440 = v3;
                    let a441 = 1u64;
                    nat_add(a440, a441)?
                }, List::cons(f18(v0, v4, {
                    let a442 = v3;
                    let a443 = v4;
                    nat_sub(a442, a443)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a444 = v3;
                    let a445 = 1u64;
                    nat_add(a444, a445)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f70(v0, v1, 48u8, 57u8) {
        Ok(Some({
            let a446 = v1;
            let a447 = 1u64;
            nat_add(a446, a447)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn run(p0: Bytes, p1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    f0(p0, p1)
}
