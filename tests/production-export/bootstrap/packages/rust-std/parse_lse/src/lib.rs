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
    C1(std::rc::Rc<Adt1>),
    C2(Bytes, Bytes, std::rc::Rc<Adt1>),
    C3(Bytes),
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C5(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C11(u64, Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt1>),
    C3(std::rc::Rc<Adt1>),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, bool, List<Adt3>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes),
    C1(std::rc::Rc<Adt5>),
    C2,
    C3(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<List<Adt5>>, Option<Adt5>, Option<Adt2>, bool),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(List<Adt8>, List<Adt0>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt5>),
    C1(std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt11>, List<Adt1>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(std::rc::Rc<Adt5>),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F5(k0, k1) => f5(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9(u64, List<Adt3>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn1::F9(k0, k1) => f9(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn2::F13(k0) => f13(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn3::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F44,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt5, p1: Adt5) -> Adt5 {
        match self {
            Fn4::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
        match self {
            Fn5::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F55,
}

impl Fn6 {
    pub fn apply(&self, p0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
        match self {
            Fn6::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F57,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn7::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F59,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn8::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F61(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0), p1: Adt5) -> R<(List<Adt0>, Adt0)> {
        match self {
            Fn9::F61(k0) => f61(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F64(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn10::F64(k0, k1) => f64(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn11::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F73,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt8>, p1: Adt5) -> List<Adt8> {
        match self {
            Fn12::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F85(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Result<List<Adt8>, Adt2>, p1: Adt5) -> R<Result<List<Adt8>, Adt2>> {
        match self {
            Fn13::F85(k0) => f85(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F88(u64, Bytes),
    F103(Bytes, u64, Bytes, u64),
    F110(u64, Bytes),
    F135(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn14::F88(k0, k1) => f88(*k0, k1.clone(), p0),
            Fn14::F103(k0, k1, k2, k3) => f103(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn14::F110(k0, k1) => f110(*k0, k1.clone(), p0),
            Fn14::F135(k0) => f135(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F91(u64),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn15::F91(k0) => f91(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F96(u64),
}

impl Fn16 {
    pub fn apply(&self, p0: Result<List<Adt1>, Adt2>, p1: Adt5) -> R<Result<List<Adt1>, Adt2>> {
        match self {
            Fn16::F96(k0) => f96(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F107,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn17::F107 => f107(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F109,
}

impl Fn18 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn18::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F115(bool, u64, Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn19::F115(k0, k1, k2) => f115(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F125,
}

impl Fn20 {
    pub fn apply(&self, p0: List<Adt11>, p1: Adt5) -> List<Adt11> {
        match self {
            Fn20::F125 => f125(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F127,
}

impl Fn21 {
    pub fn apply(&self, p0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
        match self {
            Fn21::F127 => f127(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F132(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn22::F132(k0) => f132(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: u64) -> R<Result<Adt0, Adt2>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let m2 = f2(v3.clone(), v1)?;
            match m2 {
                Ok(v5) => {
                    f3(v5, {
                        let a3 = v3;
                        length_list(a3)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt0, Adt2>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt2>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt3>, Adt2>> {
    let v1: u64 = {
        let a4 = v0.clone();
        length_bytes(a4)
    };
    let v2: Adt4 = {
        let (h5, _) = f4(Fn0::F5(v1, v0.clone()), {
            let a6 = v1;
            let a7 = 1u64;
            nat_add(a6, a7)?
        }, Adt4::C0(0u64, 0u64, false, List::<Adt3>::nil(), None::<Adt2>))?;
        h5
    };
    let m9 = match v2.clone() {
        Adt4::C0(_, _, _, _, h8) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt3>, Adt2>(f6(f7(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt3>, Adt2>(v3))
        }
    }
}

pub fn f2(v0: List<Adt3>, v1: u64) -> R<Result<Adt5, Adt2>> {
    let v2: Adt6 = {
        let (h10, _) = f8(Fn1::F9(v1, v0.clone()), {
            let a12 = {
                let a11 = v0;
                length_list(a11)
            };
            let a13 = 2u64;
            nat_add(a12, a13)?
        }, Adt6::C0(0u64, List::<List<Adt5>>::nil(), None::<Adt5>, None::<Adt2>, false))?;
        h10
    };
    let m15 = match v2.clone() {
        Adt6::C0(_, _, _, h14, _) => {
            h14
        }
    };
    match m15 {
        None => {
            let m17 = match v2 {
                Adt6::C0(_, _, h16, _, _) => {
                    h16
                }
            };
            match m17 {
                None => {
                    Ok(Err::<Adt5, Adt2>(f10(f11(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt5, Adt2>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt5, Adt2>(v3))
        }
    }
}

pub fn f3(v0: Adt5, v1: u64) -> R<Result<Adt0, Adt2>> {
    let v2: Adt7 = {
        let (h18, _) = f12(Fn2::F13(v1), {
            let a21 = {
                let a19 = v1;
                let a20 = 2u64;
                nat_mul(a19, a20)?
            };
            let a22 = 2u64;
            nat_add(a21, a22)?
        }, Adt7::C0(List::cons(Adt8::C0(std::rc::Rc::new(v0)), List::<Adt8>::nil()), List::<Adt0>::nil(), None::<Adt2>))?;
        h18
    };
    let m24 = match v2.clone() {
        Adt7::C0(_, _, h23) => {
            h23
        }
    };
    match m24 {
        None => {
            let m26 = match v2 {
                Adt7::C0(_, h25, _) => {
                    h25
                }
            };
            match m26.uncons() {
                None => {
                    Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("malformed expression")))))
                }
                Some((v4, v5)) => {
                    let m27 = v5;
                    match m27.uncons() {
                        None => {
                            Ok(Ok::<Adt0, Adt2>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("malformed expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt2>(v3))
        }
    }
}

pub fn f4(v0: Fn0, p32: u64, p33: Adt4) -> R<(Adt4, bool)> {
    let m31 = (p32, p33);
    {
        let mut m31 = m31;
        loop {
            let step = {
                let (v1, v2) = m31;
                let m28 = v1;
                if m28 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m28 - 1;
                    let m30 = {
                        let c29: Fn0 = v0.clone();
                        c29.apply(v2.clone())?
                    };
                    match m30 {
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
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f14(v1, v0, v2)
}

pub fn f6(v0: List<Adt3>) -> List<Adt3> {
    f15(Fn3::F16, List::<Adt3>::nil(), v0)
}

pub fn f7(v0: Bytes, v1: Adt4) -> List<Adt3> {
    if match v1.clone() {
        Adt4::C0(_, _, h34, _, _) => {
            h34
        }
    } {
        List::cons(Adt3::C2(f17(v0, match v1.clone() {
            Adt4::C0(_, h35, _, _, _) => {
                h35
            }
        }, {
            let a38 = match v1.clone() {
                Adt4::C0(h36, _, _, _, _) => {
                    h36
                }
            };
            let a39 = match v1.clone() {
                Adt4::C0(_, h37, _, _, _) => {
                    h37
                }
            };
            nat_sub(a38, a39)
        })), match v1 {
            Adt4::C0(_, _, _, h40, _) => {
                h40
            }
        })
    } else {
        match v1 {
            Adt4::C0(_, _, _, h41, _) => {
                h41
            }
        }
    }
}

pub fn f8(v0: Fn1, p46: u64, p47: Adt6) -> R<(Adt6, bool)> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn1 = v0.clone();
                        c43.apply(v2.clone())?
                    };
                    match m44 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64, v1: List<Adt3>, v2: Adt6) -> R<Option<Adt6>> {
    f18(v1, v0, v2)
}

pub fn f10(v0: Bytes) -> Adt2 {
    Adt2::C0(0u64, v0)
}

pub fn f11(v0: Str) -> Bytes {
    let a48 = v0;
    utf8_encode(a48)
}

pub fn f12(v0: Fn2, p53: u64, p54: Adt7) -> R<(Adt7, bool)> {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v1, v2) = m52;
                let m49 = v1;
                if m49 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m49 - 1;
                    let m51 = {
                        let c50: Fn2 = v0.clone();
                        c50.apply(v2.clone())?
                    };
                    match m51 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
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

pub fn f13(v0: u64, v1: Adt7) -> R<Option<Adt7>> {
    f19(v0, v1)
}

pub fn f14(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m56 = match v2.clone() {
        Adt4::C0(_, _, _, _, h55) => {
            h55
        }
    };
    match m56 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h57, _, _, _, _) => {
                    h57
                }
            };
            let a58 = v4;
            let a59 = v1;
            if nat_lt(a58, a59) {
                let v5: u64 = f20(v0.clone(), v4)?;
                let a60 = v5;
                let a61 = 40u64;
                if nat_eq(a60, a61) {
                    Ok(Some(Adt4::C0({
                        let a62 = v4;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    }, match v2.clone() {
                        Adt4::C0(_, h64, _, _, _) => {
                            h64
                        }
                    }, false, List::cons(Adt3::C0, f7(v0, v2)), None::<Adt2>)))
                } else {
                    let a65 = v5;
                    let a66 = 41u64;
                    if nat_eq(a65, a66) {
                        Ok(Some(Adt4::C0({
                            let a67 = v4;
                            let a68 = 1u64;
                            nat_add(a67, a68)?
                        }, match v2.clone() {
                            Adt4::C0(_, h69, _, _, _) => {
                                h69
                            }
                        }, false, List::cons(Adt3::C1, f7(v0, v2)), None::<Adt2>)))
                    } else {
                        let a70 = v5;
                        let a71 = 32u64;
                        if if nat_eq(a70, a71) {
                            true
                        } else {
                            let a72 = v5;
                            let a73 = 10u64;
                            nat_eq(a72, a73)
                        } {
                            Ok(Some(Adt4::C0({
                                let a74 = v4;
                                let a75 = 1u64;
                                nat_add(a74, a75)?
                            }, match v2.clone() {
                                Adt4::C0(_, h76, _, _, _) => {
                                    h76
                                }
                            }, false, f7(v0, v2), None::<Adt2>)))
                        } else {
                            let a77 = v5;
                            let a78 = 32u64;
                            if if nat_lt(a77, a78) {
                                true
                            } else {
                                let a79 = v5;
                                let a80 = 127u64;
                                nat_eq(a79, a80)
                            } {
                                Ok(Some(Adt4::C0(v4, match v2.clone() {
                                    Adt4::C0(_, h81, _, _, _) => {
                                        h81
                                    }
                                }, match v2.clone() {
                                    Adt4::C0(_, _, h82, _, _) => {
                                        h82
                                    }
                                }, match v2 {
                                    Adt4::C0(_, _, _, h83, _) => {
                                        h83
                                    }
                                }, Some(f10(f11(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt4::C0(_, _, h84, _, _) => {
                                        h84
                                    }
                                } {
                                    Ok(Some(Adt4::C0({
                                        let a85 = v4;
                                        let a86 = 1u64;
                                        nat_add(a85, a86)?
                                    }, match v2.clone() {
                                        Adt4::C0(_, h87, _, _, _) => {
                                            h87
                                        }
                                    }, true, match v2 {
                                        Adt4::C0(_, _, _, h88, _) => {
                                            h88
                                        }
                                    }, None::<Adt2>)))
                                } else {
                                    Ok(Some(Adt4::C0({
                                        let a89 = v4;
                                        let a90 = 1u64;
                                        nat_add(a89, a90)?
                                    }, v4, true, match v2 {
                                        Adt4::C0(_, _, _, h91, _) => {
                                            h91
                                        }
                                    }, None::<Adt2>)))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f15(v0: Fn3, p95: List<Adt3>, p96: List<Adt3>) -> List<Adt3> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m92 = v2;
                match m92.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c93: Fn3 = v0.clone();
                            c93.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m100 = {
        let a97 = v0;
        let a98 = v1;
        let a99 = v2;
        slice_bytes(a97, a98, a99)
    };
    match m100 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f18(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Option<Adt6>> {
    let m102 = match v2.clone() {
        Adt6::C0(_, _, _, h101, _) => {
            h101
        }
    };
    match m102 {
        None => {
            if match v2.clone() {
                Adt6::C0(_, _, _, _, h103) => {
                    h103
                }
            } {
                Ok(None::<Adt6>)
            } else {
                let m107 = {
                    let a105 = v0;
                    let a106 = match v2.clone() {
                        Adt6::C0(h104, _, _, _, _) => {
                            h104
                        }
                    };
                    index_list(a105, a106)
                };
                match m107 {
                    None => {
                        let m109 = match v2.clone() {
                            Adt6::C0(_, h108, _, _, _) => {
                                h108
                            }
                        };
                        match m109.uncons() {
                            None => {
                                let m111 = match v2.clone() {
                                    Adt6::C0(_, _, h110, _, _) => {
                                        h110
                                    }
                                };
                                match m111 {
                                    None => {
                                        Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt6::C0(match v2.clone() {
                                            Adt6::C0(h112, _, _, _, _) => {
                                                h112
                                            }
                                        }, List::<List<Adt5>>::nil(), match v2 {
                                            Adt6::C0(_, _, h113, _, _) => {
                                                h113
                                            }
                                        }, None::<Adt2>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m115 = match v2.clone() {
                            Adt6::C0(_, _, h114, _, _) => {
                                h114
                            }
                        };
                        match m115 {
                            None => {
                                Ok(Some(f22(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f19(v0: u64, v1: Adt7) -> R<Option<Adt7>> {
    let m117 = match v1.clone() {
        Adt7::C0(_, _, h116) => {
            h116
        }
    };
    match m117 {
        None => {
            let m119 = match v1.clone() {
                Adt7::C0(h118, _, _) => {
                    h118
                }
            };
            match m119.uncons() {
                None => {
                    Ok(None::<Adt7>)
                }
                Some((v3, v4)) => {
                    let m120 = v3;
                    match m120 {
                        Adt8::C0(r121) => {
                            let v5 = (*r121).clone();
                            Ok(Some(f23(v1, v4, v5, v0)?))
                        }
                        Adt8::C1(r122) => {
                            let v6 = (*r122).clone();
                            Ok(Some(f24(v1, v4, v6, v0)?))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt7>)
        }
    }
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let m123 = f25(v0, v1);
    match m123 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a168 = {
                let a144 = {
                    let a132 = {
                        let a126 = {
                            let a124 = v3;
                            let a125 = 128u8;
                            fixed_u8::bit_and(a124, a125)
                        };
                        let a127 = 128u8;
                        if equal(a126, a127) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a133 = {
                        let a130 = {
                            let a128 = v3;
                            let a129 = 64u8;
                            fixed_u8::bit_and(a128, a129)
                        };
                        let a131 = 64u8;
                        if equal(a130, a131) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a132, a133)?
                };
                let a145 = {
                    let a142 = {
                        let a136 = {
                            let a134 = v3;
                            let a135 = 32u8;
                            fixed_u8::bit_and(a134, a135)
                        };
                        let a137 = 32u8;
                        if equal(a136, a137) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a143 = {
                        let a140 = {
                            let a138 = v3;
                            let a139 = 16u8;
                            fixed_u8::bit_and(a138, a139)
                        };
                        let a141 = 16u8;
                        if equal(a140, a141) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a142, a143)?
                };
                nat_add(a144, a145)?
            };
            let a169 = {
                let a166 = {
                    let a154 = {
                        let a148 = {
                            let a146 = v3;
                            let a147 = 8u8;
                            fixed_u8::bit_and(a146, a147)
                        };
                        let a149 = 8u8;
                        if equal(a148, a149) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a155 = {
                        let a152 = {
                            let a150 = v3;
                            let a151 = 4u8;
                            fixed_u8::bit_and(a150, a151)
                        };
                        let a153 = 4u8;
                        if equal(a152, a153) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a154, a155)?
                };
                let a167 = {
                    let a164 = {
                        let a158 = {
                            let a156 = v3;
                            let a157 = 2u8;
                            fixed_u8::bit_and(a156, a157)
                        };
                        let a159 = 2u8;
                        if equal(a158, a159) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a165 = {
                        let a162 = {
                            let a160 = v3;
                            let a161 = 1u8;
                            fixed_u8::bit_and(a160, a161)
                        };
                        let a163 = 1u8;
                        if equal(a162, a163) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a164, a165)?
                };
                nat_add(a166, a167)?
            };
            nat_add(a168, a169)
        }
    }
}

pub fn f21(v0: Adt6, v1: Adt2) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h170, _, _, _, _) => {
            h170
        }
    }, match v0.clone() {
        Adt6::C0(_, h171, _, _, _) => {
            h171
        }
    }, match v0 {
        Adt6::C0(_, _, h172, _, _) => {
            h172
        }
    }, Some(v1), false)
}

pub fn f22(v0: u64, v1: Adt6, v2: Adt3) -> R<Adt6> {
    let m173 = v2;
    match m173 {
        Adt3::C0 => {
            let a176 = {
                let a175 = match v1.clone() {
                    Adt6::C0(_, h174, _, _, _) => {
                        h174
                    }
                };
                length_list(a175)
            };
            let a177 = v0;
            if nat_lt(a176, a177) {
                Ok(Adt6::C0({
                    let a179 = match v1.clone() {
                        Adt6::C0(h178, _, _, _, _) => {
                            h178
                        }
                    };
                    let a180 = 1u64;
                    nat_add(a179, a180)?
                }, List::cons(List::<Adt5>::nil(), match v1.clone() {
                    Adt6::C0(_, h181, _, _, _) => {
                        h181
                    }
                }), match v1 {
                    Adt6::C0(_, _, h182, _, _) => {
                        h182
                    }
                }, None::<Adt2>, false))
            } else {
                Ok(f21(v1, Adt2::C0(1u64, f26(f11(Str::lit("max_scope_depth exceeded: configured ")), f27(v0)))))
            }
        }
        Adt3::C1 => {
            let m184 = match v1.clone() {
                Adt6::C0(_, h183, _, _, _) => {
                    h183
                }
            };
            match m184.uncons() {
                None => {
                    Ok(f21(v1, f10(f11(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f28(v1, Adt5::C1(std::rc::Rc::new(f29(v3))), v4)
                }
            }
        }
        Adt3::C2(v5) => {
            f28(v1.clone(), Adt5::C0(v5), match v1 {
                Adt6::C0(_, h185, _, _, _) => {
                    h185
                }
            })
        }
    }
}

pub fn f23(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: u64) -> R<Adt7> {
    let m186 = v2.clone();
    match m186 {
        Adt5::C0(_) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt5::C1(r189) => {
            let v4 = (*r189).clone();
            let m187 = f30(v4, v3)?;
            match m187.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                }
                Some((v5, v6)) => {
                    let m188 = v5;
                    match m188 {
                        Adt5::C0(v7) => {
                            f32(v0, v1, v2, v7, v6, v3)
                        }
                        Adt5::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                        Adt5::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt5::C3(_, _) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
    }
}

pub fn f24(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: u64) -> R<Adt7> {
    let m190 = v2;
    match m190 {
        Adt5::C0(_) => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
        Adt5::C1(r218) => {
            let v4 = (*r218).clone();
            let v5: List<Adt5> = f30(v4, v3)?;
            let m191 = v5;
            match m191.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
                }
                Some((v6, v7)) => {
                    let m192 = v6;
                    match m192 {
                        Adt5::C0(v8) => {
                            let a193 = v8.clone();
                            let a194 = f11(Str::lit("app"));
                            if equal(a193, a194) {
                                let v9: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                    Adt7::C0(_, h195, _) => {
                                        h195
                                    }
                                }, {
                                    let a196 = v7;
                                    length_list(a196)
                                });
                                let m198 = {
                                    let (h197, _) = v9;
                                    h197
                                };
                                match m198.uncons() {
                                    None => {
                                        Ok(f31(v0, f10(f11(Str::lit("missing function")))))
                                    }
                                    Some((v10, v11)) => {
                                        Ok(Adt7::C0(v1, List::cons(Adt0::C4(std::rc::Rc::new(v10), std::rc::Rc::new(f35(v11))), {
                                            let (_, h199) = v9;
                                            h199
                                        }), None::<Adt2>))
                                    }
                                }
                            } else {
                                if f36(v8.clone()) {
                                    let m200 = f37(v7, 0u64);
                                    match m200 {
                                        Adt5::C0(_) => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                        Adt5::C1(r210) => {
                                            let v12 = (*r210).clone();
                                            let v13: List<Adt5> = f30(v12, v3)?;
                                            let v14: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                                Adt7::C0(_, h201, _) => {
                                                    h201
                                                }
                                            }, {
                                                let a203 = {
                                                    let a202 = v13.clone();
                                                    length_list(a202)
                                                };
                                                let a204 = 1u64;
                                                nat_add(a203, a204)?
                                            });
                                            let m206 = f38({
                                                let (h205, _) = v14;
                                                h205
                                            });
                                            match m206.uncons() {
                                                None => {
                                                    Ok(f31(v0, f10(f11(Str::lit("missing body")))))
                                                }
                                                Some((v15, v16)) => {
                                                    let v17: Adt0 = f39(v13, f38(v16), v3)?;
                                                    Ok(Adt7::C0(v1, List::cons({
                                                        let a207 = v8;
                                                        let a208 = f11(Str::lit("pi"));
                                                        if equal(a207, a208) {
                                                            Adt0::C5(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        } else {
                                                            Adt0::C6(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        }
                                                    }, {
                                                        let (_, h209) = v14;
                                                        h209
                                                    }), None::<Adt2>))
                                                }
                                            }
                                        }
                                        Adt5::C2 => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                        Adt5::C3(_, _) => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                    }
                                } else {
                                    let v21: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                        Adt7::C0(_, h211, _) => {
                                            h211
                                        }
                                    }, 3u64);
                                    let m213 = {
                                        let (h212, _) = v21;
                                        h212
                                    };
                                    match m213.uncons() {
                                        None => {
                                            Ok(f31(v0, f10(f11(Str::lit("missing type")))))
                                        }
                                        Some((v22, v23)) => {
                                            let m214 = v23;
                                            match m214.uncons() {
                                                None => {
                                                    Ok(f31(v0, f10(f11(Str::lit("missing value")))))
                                                }
                                                Some((v24, v25)) => {
                                                    let m215 = v25;
                                                    match m215.uncons() {
                                                        None => {
                                                            Ok(f31(v0, f10(f11(Str::lit("missing body")))))
                                                        }
                                                        Some((v26, _)) => {
                                                            let m216 = f40(f37(v7, 0u64), f11(Str::lit("a local ID")), v3);
                                                            match m216 {
                                                                Ok(v28) => {
                                                                    Ok(Adt7::C0(v1, List::cons(Adt0::C7(v28, std::rc::Rc::new(v22), std::rc::Rc::new(v24), std::rc::Rc::new(v26)), {
                                                                        let (_, h217) = v21;
                                                                        h217
                                                                    }), None::<Adt2>))
                                                                }
                                                                Err(v29) => {
                                                                    Ok(f31(v0, v29))
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
                        Adt5::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                        Adt5::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
        Adt5::C3(_, _) => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a219 = v0;
    let a220 = v1;
    index_bytes(a219, a220)
}

pub fn f26(v0: Bytes, v1: Bytes) -> Bytes {
    let a221 = v0;
    let a222 = v1;
    append_bytes(a221, a222)
}

pub fn f27(v0: u64) -> Bytes {
    f41(f42(), 10u64, v0)
}

pub fn f28(v0: Adt6, v1: Adt5, v2: List<List<Adt5>>) -> R<Adt6> {
    let m223 = v2;
    match m223.uncons() {
        None => {
            Ok(Adt6::C0({
                let a225 = match v0 {
                    Adt6::C0(h224, _, _, _, _) => {
                        h224
                    }
                };
                let a226 = 1u64;
                nat_add(a225, a226)?
            }, List::<List<Adt5>>::nil(), Some(v1), None::<Adt2>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt6::C0({
                let a228 = match v0.clone() {
                    Adt6::C0(h227, _, _, _, _) => {
                        h227
                    }
                };
                let a229 = 1u64;
                nat_add(a228, a229)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt6::C0(_, _, h230, _, _) => {
                    h230
                }
            }, None::<Adt2>, false))
        }
    }
}

pub fn f29(v0: List<Adt5>) -> Adt5 {
    f43(Fn4::F44, Adt5::C2, v0)
}

pub fn f30(v0: Adt5, v1: u64) -> R<List<Adt5>> {
    Ok(f45({
        let (_, h231) = {
            let (h232, _) = f46(Fn5::F47, {
                let a233 = v1;
                let a234 = 1u64;
                nat_add(a233, a234)?
            }, (v0, List::<Adt5>::nil()));
            h232
        };
        h231
    }))
}

pub fn f31(v0: Adt7, v1: Adt2) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h235, _, _) => {
            h235
        }
    }, match v0 {
        Adt7::C0(_, h236, _) => {
            h236
        }
    }, Some(v1))
}

pub fn f32(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: Bytes, v4: List<Adt5>, v5: u64) -> R<Adt7> {
    if f48(v3.clone()) {
        let m237 = f49(v3, v4, v5)?;
        match m237 {
            Ok(v6) => {
                Ok(Adt7::C0(v1, List::cons(v6, match v0 {
                    Adt7::C0(_, h238, _) => {
                        h238
                    }
                }), None::<Adt2>))
            }
            Err(v7) => {
                Ok(f31(v0, v7))
            }
        }
    } else {
        let a239 = v3.clone();
        let a240 = f11(Str::lit("app"));
        if equal(a239, a240) {
            let a242 = {
                let a241 = v4.clone();
                length_list(a241)
            };
            let a243 = 2u64;
            if nat_lt(a242, a243) {
                Ok(f31(v0, f10(f11(Str::lit("`app` has at least a function and one argument")))))
            } else {
                Ok(Adt7::C0(f50(v4, List::cons(Adt8::C1(std::rc::Rc::new(v2)), v1)), match v0 {
                    Adt7::C0(_, h244, _) => {
                        h244
                    }
                }, None::<Adt2>))
            }
        } else {
            if f36(v3.clone()) {
                f51(v0, v1, v2, v3, v4, v5)
            } else {
                let a245 = v3.clone();
                let a246 = f11(Str::lit("let"));
                if equal(a245, a246) {
                    f52(v0, v1, v2, v4, v5)
                } else {
                    Ok(f31(v0, f10(f26(f53(v3), f11(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f33(v0: Adt5) -> Bytes {
    let m247 = v0;
    match m247 {
        Adt5::C0(v1) => {
            f53(v1)
        }
        Adt5::C1(r251) => {
            let v2 = (*r251).clone();
            let m248 = v2;
            match m248 {
                Adt5::C0(_) => {
                    f11(Str::lit("a list"))
                }
                Adt5::C1(_) => {
                    f11(Str::lit("a list"))
                }
                Adt5::C2 => {
                    f11(Str::lit("a list"))
                }
                Adt5::C3(r250, _) => {
                    let v3 = (*r250).clone();
                    let m249 = v3;
                    match m249 {
                        Adt5::C0(v5) => {
                            f26(f26(f11(Str::lit("`(")), v5), f11(Str::lit(" ...)`")))
                        }
                        Adt5::C1(_) => {
                            f11(Str::lit("a list"))
                        }
                        Adt5::C2 => {
                            f11(Str::lit("a list"))
                        }
                        Adt5::C3(_, _) => {
                            f11(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            f11(Str::lit("a list"))
        }
        Adt5::C3(_, _) => {
            f11(Str::lit("a list"))
        }
    }
}

pub fn f34(v0: List<Adt0>, v1: u64) -> (List<Adt0>, List<Adt0>) {
    let (h252, _) = f54(Fn6::F55, v1, (List::<Adt0>::nil(), v0));
    h252
}

pub fn f35(v0: List<Adt0>) -> Adt0 {
    f56(Fn7::F57, Adt0::C9, f38(v0))
}

pub fn f36(v0: Bytes) -> bool {
    let a253 = v0.clone();
    let a254 = f11(Str::lit("pi"));
    if equal(a253, a254) {
        true
    } else {
        let a255 = v0;
        let a256 = f11(Str::lit("lam"));
        equal(a255, a256)
    }
}

pub fn f37(v0: List<Adt5>, v1: u64) -> Adt5 {
    let m259 = {
        let a257 = v0;
        let a258 = v1;
        index_list(a257, a258)
    };
    match m259 {
        None => {
            Adt5::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: List<Adt0>) -> List<Adt0> {
    f58(Fn8::F59, List::<Adt0>::nil(), v0)
}

pub fn f39(v0: List<Adt5>, v1: List<Adt0>, v2: u64) -> R<Adt0> {
    let (_, h260) = f60(Fn9::F61(v2), (f38(v1), Adt0::C9), f45(v0))?;
    Ok(h260)
}

pub fn f40(v0: Adt5, v1: Bytes, _: u64) -> Result<Bytes, Adt2> {
    let m261 = v0.clone();
    match m261 {
        Adt5::C0(v3) => {
            Ok::<Bytes, Adt2>(v3)
        }
        Adt5::C1(_) => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt5::C2 => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt5::C3(_, _) => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
    }
}

pub fn f41(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a262 = v2;
    let a263 = 0u64;
    if nat_eq(a262, a263) {
        f62(v0, 0u64)
    } else {
        let (_, h264) = {
            let (h265, _) = f63(Fn10::F64(v1, v0), v2, (v2, Bytes::lit(&[])));
            h265
        };
        h264
    }
}

pub fn f42() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f43(v0: Fn4, p269: Adt5, p270: List<Adt5>) -> Adt5 {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m266 = v2;
                match m266.uncons() {
                    None => {
                        Err::<(Adt5, List<Adt5>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<Adt5>), Adt5>(({
                            let c267: Fn4 = v0.clone();
                            c267.apply(v1, v3)
                        }, v4))
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

pub fn f44(v0: Adt5, v1: Adt5) -> Adt5 {
    Adt5::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f45(v0: List<Adt5>) -> List<Adt5> {
    f65(Fn11::F66, List::<Adt5>::nil(), v0)
}

pub fn f46(v0: Fn5, p275: u64, p276: (Adt5, List<Adt5>)) -> ((Adt5, List<Adt5>), bool) {
    let m274 = (p275, p276);
    {
        let mut m274 = m274;
        loop {
            let step = {
                let (v1, v2) = m274;
                let m271 = v1;
                if m271 == 0 {
                    Err::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v2, false))
                } else {
                    let v3 = m271 - 1;
                    let m273 = {
                        let c272: Fn5 = v0.clone();
                        c272.apply(v2.clone())
                    };
                    match m273 {
                        None => {
                            Err::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
    f67(v0)
}

pub fn f48(v0: Bytes) -> bool {
    let a277 = v0.clone();
    let a278 = f11(Str::lit("sort"));
    if equal(a277, a278) {
        true
    } else {
        let a279 = v0.clone();
        let a280 = f11(Str::lit("const"));
        if equal(a279, a280) {
            true
        } else {
            let a281 = v0.clone();
            let a282 = f11(Str::lit("local"));
            if equal(a281, a282) {
                true
            } else {
                let a283 = v0;
                let a284 = f11(Str::lit("nat"));
                equal(a283, a284)
            }
        }
    }
}

pub fn f49(v0: Bytes, v1: List<Adt5>, v2: u64) -> R<Result<Adt0, Adt2>> {
    let a285 = v0.clone();
    let a286 = f11(Str::lit("sort"));
    if equal(a285, a286) {
        f68(v1, v2)
    } else {
        let a287 = v0.clone();
        let a288 = f11(Str::lit("const"));
        if equal(a287, a288) {
            f69(v1, v2)
        } else {
            let a289 = v0.clone();
            let a290 = f11(Str::lit("local"));
            if equal(a289, a290) {
                f70(v1)
            } else {
                let a291 = v0.clone();
                let a292 = f11(Str::lit("nat"));
                if equal(a291, a292) {
                    f71(v1)
                } else {
                    Ok(Err::<Adt0, Adt2>(f10(f26(f53(v0), f11(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f50(v0: List<Adt5>, v1: List<Adt8>) -> List<Adt8> {
    f72(Fn12::F73, v1, f45(v0))
}

pub fn f51(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: Bytes, v4: List<Adt5>, v5: u64) -> R<Adt7> {
    let a294 = {
        let a293 = v4.clone();
        length_list(a293)
    };
    let a295 = 2u64;
    if nat_eq(a294, a295) {
        let m296 = f37(v4.clone(), 0u64);
        match m296 {
            Adt5::C0(_) => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
            Adt5::C1(r300) => {
                let v6 = (*r300).clone();
                let v7: List<Adt5> = f30(v6, v5)?;
                let m297 = v7.clone();
                match m297.uncons() {
                    None => {
                        Ok(f31(v0, f10(f26(f53(v3), f11(Str::lit(" has at least one binder"))))))
                    }
                    Some((_, _)) => {
                        let m298 = f74(v7, List::cons(Adt8::C0(std::rc::Rc::new(f37(v4, 1u64))), List::cons(Adt8::C1(std::rc::Rc::new(v2)), v1)), v5)?;
                        match m298 {
                            Ok(v10) => {
                                Ok(Adt7::C0(v10, match v0 {
                                    Adt7::C0(_, h299, _) => {
                                        h299
                                    }
                                }, None::<Adt2>))
                            }
                            Err(v11) => {
                                Ok(f31(v0, v11))
                            }
                        }
                    }
                }
            }
            Adt5::C2 => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
            Adt5::C3(_, _) => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
        }
    } else {
        Ok(f31(v0, f10(f26(f26(f26(f26(f11(Str::lit("`")), v3.clone()), f11(Str::lit("` is `("))), v3), f11(Str::lit(" (binders...) body)`"))))))
    }
}

pub fn f52(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: List<Adt5>, v4: u64) -> R<Adt7> {
    let a302 = {
        let a301 = v3.clone();
        length_list(a301)
    };
    let a303 = 4u64;
    if nat_eq(a302, a303) {
        let m304 = f40(f37(v3.clone(), 0u64), f11(Str::lit("a local ID")), v4);
        match m304 {
            Ok(v5) => {
                if f75(v5.clone())? {
                    Ok(Adt7::C0(List::cons(Adt8::C0(std::rc::Rc::new(f37(v3.clone(), 1u64))), List::cons(Adt8::C0(std::rc::Rc::new(f37(v3.clone(), 2u64))), List::cons(Adt8::C0(std::rc::Rc::new(f37(v3, 3u64))), List::cons(Adt8::C1(std::rc::Rc::new(v2)), v1)))), match v0 {
                        Adt7::C0(_, h305, _) => {
                            h305
                        }
                    }, None::<Adt2>))
                } else {
                    Ok(f31(v0, f10(f26(f53(v5), f11(Str::lit(" is not a valid local ID"))))))
                }
            }
            Err(v6) => {
                Ok(f31(v0, v6))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("`let` is `(let name type value body)`")))))
    }
}

pub fn f53(v0: Bytes) -> Bytes {
    f26(f26(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f54(v0: Fn6, p310: u64, p311: (List<Adt0>, List<Adt0>)) -> ((List<Adt0>, List<Adt0>), bool) {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v1, v2) = m309;
                let m306 = v1;
                if m306 == 0 {
                    Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, false))
                } else {
                    let v3 = m306 - 1;
                    let m308 = {
                        let c307: Fn6 = v0.clone();
                        c307.apply(v2.clone())
                    };
                    match m308 {
                        None => {
                            Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
    let m313 = {
        let (_, h312) = v0;
        h312
    };
    match m313.uncons() {
        None => {
            None::<(List<Adt0>, List<Adt0>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h314, _) = v0;
                h314
            }), v2))
        }
    }
}

pub fn f56(v0: Fn7, p318: Adt0, p319: List<Adt0>) -> Adt0 {
    let m317 = (p318, p319);
    {
        let mut m317 = m317;
        loop {
            let step = {
                let (v1, v2) = m317;
                let m315 = v2;
                match m315.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c316: Fn7 = v0.clone();
                            c316.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f58(v0: Fn8, p323: List<Adt0>, p324: List<Adt0>) -> List<Adt0> {
    let m322 = (p323, p324);
    {
        let mut m322 = m322;
        loop {
            let step = {
                let (v1, v2) = m322;
                let m320 = v2;
                match m320.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c321: Fn8 = v0.clone();
                            c321.apply(v1, v3)
                        }, v4))
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

pub fn f59(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f60(v0: Fn9, p328: (List<Adt0>, Adt0), p329: List<Adt5>) -> R<(List<Adt0>, Adt0)> {
    let m327 = (p328, p329);
    {
        let mut m327 = m327;
        loop {
            let step = {
                let (v1, v2) = m327;
                let m325 = v2;
                match m325.uncons() {
                    None => {
                        Err::<((List<Adt0>, Adt0), List<Adt5>), R<(List<Adt0>, Adt0)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt0>, Adt0), List<Adt5>), R<(List<Adt0>, Adt0)>>(({
                            let c326: Fn9 = v0.clone();
                            c326.apply(v1, v3)?
                        }, v4))
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

pub fn f61(v0: u64, v1: (List<Adt0>, Adt0), v2: Adt5) -> R<(List<Adt0>, Adt0)> {
    let m331 = {
        let (h330, _) = v1.clone();
        h330
    };
    match m331.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            let m332 = f76(v2, v0)?;
            match m332 {
                Ok(v5) => {
                    Ok((v4, Adt0::C11(match v5.clone() {
                        Adt9::C0(h333, _, _) => {
                            h333
                        }
                    }, match v5 {
                        Adt9::C0(_, h334, _) => {
                            h334
                        }
                    }, std::rc::Rc::new(v3), std::rc::Rc::new({
                        let (_, h335) = v1;
                        h335
                    }))))
                }
                Err(_) => {
                    Ok((v4, {
                        let (_, h336) = v1;
                        h336
                    }))
                }
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> Bytes {
    let m340 = {
        let a337 = v0;
        let a338 = v1;
        let a339 = 1u64;
        slice_bytes(a337, a338, a339)
    };
    match m340 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f63(v0: Fn10, p345: u64, p346: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m344 = (p345, p346);
    {
        let mut m344 = m344;
        loop {
            let step = {
                let (v1, v2) = m344;
                let m341 = v1;
                if m341 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m341 - 1;
                    let m343 = {
                        let c342: Fn10 = v0.clone();
                        c342.apply(v2.clone())
                    };
                    match m343 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f77(v1, v0, v2)
}

pub fn f65(v0: Fn11, p350: List<Adt5>, p351: List<Adt5>) -> List<Adt5> {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v1, v2) = m349;
                let m347 = v2;
                match m347.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c348: Fn11 = v0.clone();
                            c348.apply(v1, v3)
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

pub fn f66(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f67(v0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
    let m353 = {
        let (h352, _) = v0;
        h352
    };
    match m353 {
        Adt5::C0(_) => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C1(_) => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C2 => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C3(r355, r356) => {
            let v3 = (*r355).clone();
            let v4 = (*r356).clone();
            Some((v4, List::cons(v3, {
                let (_, h354) = v0;
                h354
            })))
        }
    }
}

pub fn f68(v0: List<Adt5>, v1: u64) -> R<Result<Adt0, Adt2>> {
    let a358 = {
        let a357 = v0.clone();
        length_list(a357)
    };
    let a359 = 1u64;
    if nat_eq(a358, a359) {
        let m360 = f37(v0, 0u64);
        match m360 {
            Adt5::C0(v2) => {
                let a361 = v2;
                let a362 = f11(Str::lit("prop"));
                if equal(a361, a362) {
                    Ok(Ok::<Adt0, Adt2>(Adt0::C0))
                } else {
                    Ok(Err::<Adt0, Adt2>(f78()))
                }
            }
            Adt5::C1(r370) => {
                let v3 = (*r370).clone();
                let v4: List<Adt5> = f30(v3, v1)?;
                let a364 = {
                    let a363 = v4.clone();
                    length_list(a363)
                };
                let a365 = 2u64;
                if nat_eq(a364, a365) {
                    let m366 = f37(v4.clone(), 0u64);
                    match m366 {
                        Adt5::C0(v5) => {
                            let a367 = v5;
                            let a368 = f11(Str::lit("type"));
                            if equal(a367, a368) {
                                let m369 = f79(f37(v4, 1u64), v1)?;
                                match m369 {
                                    Ok(v6) => {
                                        Ok(Ok::<Adt0, Adt2>(Adt0::C1(std::rc::Rc::new(v6))))
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt0, Adt2>(v7))
                                    }
                                }
                            } else {
                                Ok(Err::<Adt0, Adt2>(f78()))
                            }
                        }
                        Adt5::C1(_) => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                        Adt5::C2 => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                        Adt5::C3(_, _) => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                    }
                } else {
                    Ok(Err::<Adt0, Adt2>(f78()))
                }
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f78()))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f78()))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f78()))
    }
}

pub fn f69(v0: List<Adt5>, v1: u64) -> R<Result<Adt0, Adt2>> {
    let v2: u64 = {
        let a371 = v0.clone();
        length_list(a371)
    };
    let m372 = f37(v0.clone(), 0u64);
    match m372 {
        Adt5::C0(v3) => {
            let a373 = v2;
            let a374 = 1u64;
            if nat_eq(a373, a374) {
                let m375 = f80(v3)?;
                match m375 {
                    Ok(v4) => {
                        Ok(Ok::<Adt0, Adt2>(Adt0::C2({
                            let (h376, _) = v4;
                            h376
                        }, {
                            let (_, h377) = v4;
                            h377
                        }, std::rc::Rc::new(Adt1::C5))))
                    }
                    Err(v5) => {
                        Ok(Err::<Adt0, Adt2>(v5))
                    }
                }
            } else {
                let a378 = v2;
                let a379 = 2u64;
                if nat_eq(a378, a379) {
                    let m380 = f37(v0, 1u64);
                    match m380 {
                        Adt5::C0(_) => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                        Adt5::C1(r386) => {
                            let v6 = (*r386).clone();
                            let v7: List<Adt5> = f30(v6, v1)?;
                            let m381 = v7.clone();
                            match m381.uncons() {
                                None => {
                                    Ok(Err::<Adt0, Adt2>(f81()))
                                }
                                Some((_, _)) => {
                                    let m382 = f80(v3)?;
                                    match m382 {
                                        Ok(v10) => {
                                            let m383 = f82(v7, v1)?;
                                            match m383 {
                                                Ok(v11) => {
                                                    Ok(Ok::<Adt0, Adt2>(Adt0::C2({
                                                        let (h384, _) = v10;
                                                        h384
                                                    }, {
                                                        let (_, h385) = v10;
                                                        h385
                                                    }, std::rc::Rc::new(v11))))
                                                }
                                                Err(v12) => {
                                                    Ok(Err::<Adt0, Adt2>(v12))
                                                }
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<Adt0, Adt2>(v13))
                                        }
                                    }
                                }
                            }
                        }
                        Adt5::C2 => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                        Adt5::C3(_, _) => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                    }
                } else {
                    Ok(Err::<Adt0, Adt2>(f81()))
                }
            }
        }
        Adt5::C1(_) => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
        Adt5::C2 => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
        Adt5::C3(_, _) => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
    }
}

pub fn f70(v0: List<Adt5>) -> R<Result<Adt0, Adt2>> {
    let a388 = {
        let a387 = v0.clone();
        length_list(a387)
    };
    let a389 = 1u64;
    if nat_eq(a388, a389) {
        let m390 = f37(v0, 0u64);
        match m390 {
            Adt5::C0(v1) => {
                if f75(v1.clone())? {
                    Ok(Ok::<Adt0, Adt2>(Adt0::C3(v1)))
                } else {
                    Ok(Err::<Adt0, Adt2>(f10(f26(f53(v1), f11(Str::lit(" is not a valid local ID"))))))
                }
            }
            Adt5::C1(_) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
    }
}

pub fn f71(v0: List<Adt5>) -> R<Result<Adt0, Adt2>> {
    let a392 = {
        let a391 = v0.clone();
        length_list(a391)
    };
    let a393 = 1u64;
    if nat_eq(a392, a393) {
        let m394 = f37(v0, 0u64);
        match m394 {
            Adt5::C0(v1) => {
                let m395 = f83(v1.clone())?;
                match m395 {
                    None => {
                        Ok(Ok::<Adt0, Adt2>(Adt0::C8(v1)))
                    }
                    Some(v2) => {
                        Ok(Err::<Adt0, Adt2>(v2))
                    }
                }
            }
            Adt5::C1(_) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
    }
}

pub fn f72(v0: Fn12, p399: List<Adt8>, p400: List<Adt5>) -> List<Adt8> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v1, v2) = m398;
                let m396 = v2;
                match m396.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Adt5>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Adt5>), List<Adt8>>(({
                            let c397: Fn12 = v0.clone();
                            c397.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt8>, v1: Adt5) -> List<Adt8> {
    List::cons(Adt8::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f74(v0: List<Adt5>, v1: List<Adt8>, v2: u64) -> R<Result<List<Adt8>, Adt2>> {
    f84(Fn13::F85(v2), Ok::<List<Adt8>, Adt2>(v1), f45(v0))
}

pub fn f75(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a401 = v0.clone();
        length_bytes(a401)
    };
    let a402 = v1;
    let a403 = 0u64;
    if nat_eq(a402, a403) {
        Ok(false)
    } else {
        if f86(f20(v0.clone(), 0u64)?) {
            let a405 = {
                let (h404, _) = f87(Fn14::F88(v1, v0), v1, 1u64)?;
                h404
            };
            let a406 = v1;
            Ok(nat_eq(a405, a406))
        } else {
            Ok(false)
        }
    }
}

pub fn f76(v0: Adt5, v1: u64) -> R<Result<Adt9, Adt2>> {
    let m407 = v0.clone();
    match m407 {
        Adt5::C0(_) => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
        Adt5::C1(r414) => {
            let v2 = (*r414).clone();
            let v3: List<Adt5> = f30(v2, v1)?;
            let a409 = {
                let a408 = v3.clone();
                length_list(a408)
            };
            let a410 = 3u64;
            if nat_eq(a409, a410) {
                let m411 = f40(f37(v3.clone(), 0u64), f11(Str::lit("a binder mode")), v1);
                match m411 {
                    Ok(v4) => {
                        let m412 = f89(v4.clone());
                        match m412 {
                            None => {
                                Ok(Err::<Adt9, Adt2>(f10(f26(f53(v4), f11(Str::lit(" is not a binder mode"))))))
                            }
                            Some(v5) => {
                                let m413 = f40(f37(v3.clone(), 1u64), f11(Str::lit("a local ID")), v1);
                                match m413 {
                                    Ok(v6) => {
                                        if f75(v6.clone())? {
                                            Ok(Ok::<Adt9, Adt2>(Adt9::C0(v5, v6, std::rc::Rc::new(f37(v3, 2u64)))))
                                        } else {
                                            Ok(Err::<Adt9, Adt2>(f10(f26(f53(v6), f11(Str::lit(" is not a valid local ID"))))))
                                        }
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt9, Adt2>(v7))
                                    }
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<Adt9, Adt2>(v8))
                    }
                }
            } else {
                Ok(Err::<Adt9, Adt2>(f10(f11(Str::lit("a binder is `(mode name type)`")))))
            }
        }
        Adt5::C2 => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
        Adt5::C3(_, _) => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h415, _) = v2;
        h415
    };
    let a416 = v3;
    let a417 = 0u64;
    if nat_eq(a416, a417) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a418 = v3;
            let a419 = v1;
            let a420 = 0u64;
            nat_quot(a418, a419, a420)
        }, {
            let a425 = f62(v0, {
                let a421 = v3;
                let a422 = v1;
                let a423 = 0u64;
                nat_rem(a421, a422, a423)
            });
            let a426 = {
                let (_, h424) = v2;
                h424
            };
            append_bytes(a425, a426)
        }))
    }
}

pub fn f78() -> Adt2 {
    f10(f11(Str::lit("a sort is `(sort prop)` or `(sort (type u))`")))
}

pub fn f79(v0: Adt5, v1: u64) -> R<Result<Adt1, Adt2>> {
    let v2: Adt10 = {
        let (h427, _) = f90(Fn15::F91(v1), {
            let a430 = {
                let a428 = v1;
                let a429 = 2u64;
                nat_mul(a428, a429)?
            };
            let a431 = 2u64;
            nat_add(a430, a431)?
        }, Adt10::C0(List::cons(Adt11::C0(std::rc::Rc::new(v0)), List::<Adt11>::nil()), List::<Adt1>::nil(), None::<Adt2>))?;
        h427
    };
    let m433 = match v2.clone() {
        Adt10::C0(_, _, h432) => {
            h432
        }
    };
    match m433 {
        None => {
            let m435 = match v2 {
                Adt10::C0(_, h434, _) => {
                    h434
                }
            };
            match m435.uncons() {
                None => {
                    Ok(Err::<Adt1, Adt2>(f10(f11(Str::lit("expected a universe")))))
                }
                Some((v4, _)) => {
                    Ok(Ok::<Adt1, Adt2>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt1, Adt2>(v3))
        }
    }
}

pub fn f80(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt2>> {
    let m436 = f92(v0.clone(), f11(Str::lit("::")))?;
    match m436 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v0), f11(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f17(v0.clone(), 0u64, v1);
            let v3: Bytes = f17(v0.clone(), {
                let a437 = v1;
                let a438 = 2u64;
                nat_add(a437, a438)?
            }, {
                let a442 = {
                    let a439 = v0;
                    length_bytes(a439)
                };
                let a443 = {
                    let a440 = v1;
                    let a441 = 2u64;
                    nat_add(a440, a441)?
                };
                nat_sub(a442, a443)
            });
            if f93(v2.clone())? {
                if f94(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt2>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v3), f11(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v2), f11(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f81() -> Adt2 {
    f10(f11(Str::lit("a const is `(const qualified-id [universes])`")))
}

pub fn f82(v0: List<Adt5>, v1: u64) -> R<Result<Adt1, Adt2>> {
    let m444 = f95(Fn16::F96(v1), Ok::<List<Adt1>, Adt2>(List::<Adt1>::nil()), v0)?;
    match m444 {
        Ok(v2) => {
            Ok(Ok::<Adt1, Adt2>(f97(f98(v2))))
        }
        Err(v3) => {
            Ok(Err::<Adt1, Adt2>(v3))
        }
    }
}

pub fn f83(v0: Bytes) -> R<Option<Adt2>> {
    let v1: u64 = {
        let a445 = v0.clone();
        length_bytes(a445)
    };
    let a446 = v1;
    let a447 = 0u64;
    if if nat_eq(a446, a447) {
        true
    } else {
        !f99(v0.clone())?
    } {
        Ok(Some(f10(f26(f53(v0), f11(Str::lit(" is not a decimal"))))))
    } else {
        let a448 = 1u64;
        let a449 = v1;
        if if nat_lt(a448, a449) {
            let a450 = f20(v0.clone(), 0u64)?;
            let a451 = 48u64;
            nat_eq(a450, a451)
        } else {
            false
        } {
            Ok(Some(f10(f26(f53(v0), f11(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a452 = 20u64;
            let a453 = v1;
            if if nat_lt(a452, a453) {
                true
            } else {
                let a454 = v1;
                let a455 = 20u64;
                if nat_eq(a454, a455) {
                    let a460 = {
                        let a456 = v0.clone();
                        let a457 = f11(Str::lit("18446744073709551615"));
                        compare_bytes(a456, a457)
                    };
                    let a461 = {
                        let a458 = Bytes::lit(&[0x01]);
                        let a459 = Bytes::lit(&[]);
                        compare_bytes(a458, a459)
                    };
                    equal(a460, a461)
                } else {
                    false
                }
            } {
                Ok(Some(f10(f26(f53(v0), f11(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f84(v0: Fn13, p465: Result<List<Adt8>, Adt2>, p466: List<Adt5>) -> R<Result<List<Adt8>, Adt2>> {
    let m464 = (p465, p466);
    {
        let mut m464 = m464;
        loop {
            let step = {
                let (v1, v2) = m464;
                let m462 = v2;
                match m462.uncons() {
                    None => {
                        Err::<(Result<List<Adt8>, Adt2>, List<Adt5>), R<Result<List<Adt8>, Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt8>, Adt2>, List<Adt5>), R<Result<List<Adt8>, Adt2>>>(({
                            let c463: Fn13 = v0.clone();
                            c463.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: Result<List<Adt8>, Adt2>, v2: Adt5) -> R<Result<List<Adt8>, Adt2>> {
    let m467 = v1;
    match m467 {
        Ok(v3) => {
            let m468 = f76(v2, v0)?;
            match m468 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt8>, Adt2>(List::cons(Adt8::C0(std::rc::Rc::new(match v4 {
                        Adt9::C0(_, _, h469) => {
                            (*h469).clone()
                        }
                    })), v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt8>, Adt2>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt8>, Adt2>(v6))
        }
    }
}

pub fn f86(v0: u64) -> bool {
    if f100(v0) {
        true
    } else {
        let a470 = v0;
        let a471 = 65u64;
        if nat_lt(a470, a471) {
            false
        } else {
            let a472 = v0;
            let a473 = 90u64;
            nat_le(a472, a473)
        }
    }
}

pub fn f87(v0: Fn14, p478: u64, p479: u64) -> R<(u64, bool)> {
    let m477 = (p478, p479);
    {
        let mut m477 = m477;
        loop {
            let step = {
                let (v1, v2) = m477;
                let m474 = v1;
                if m474 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m474 - 1;
                    let m476 = {
                        let c475: Fn14 = v0.clone();
                        c475.apply(v2)?
                    };
                    match m476 {
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
                Ok(next) => { m477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f101(v1, v0, v2)
}

pub fn f89(v0: Bytes) -> Option<u64> {
    let a480 = v0.clone();
    let a481 = f11(Str::lit("explicit"));
    if equal(a480, a481) {
        Some(0u64)
    } else {
        let a482 = v0.clone();
        let a483 = f11(Str::lit("implicit"));
        if equal(a482, a483) {
            Some(1u64)
        } else {
            let a484 = v0;
            let a485 = f11(Str::lit("instance"));
            if equal(a484, a485) {
                Some(2u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f90(v0: Fn15, p490: u64, p491: Adt10) -> R<(Adt10, bool)> {
    let m489 = (p490, p491);
    {
        let mut m489 = m489;
        loop {
            let step = {
                let (v1, v2) = m489;
                let m486 = v1;
                if m486 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m486 - 1;
                    let m488 = {
                        let c487: Fn15 = v0.clone();
                        c487.apply(v2.clone())?
                    };
                    match m488 {
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
                Ok(next) => { m489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: u64, v1: Adt10) -> R<Option<Adt10>> {
    f102(v0, v1)
}

pub fn f92(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a492 = v0.clone();
        length_bytes(a492)
    };
    let v3: u64 = {
        let a493 = v1.clone();
        length_bytes(a493)
    };
    let v4: u64 = {
        let (h494, _) = f87(Fn14::F103(v1.clone(), v3, v0.clone(), v2), {
            let a495 = v2;
            let a496 = 1u64;
            nat_add(a495, a496)?
        }, 0u64)?;
        h494
    };
    if f104(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f93(v0: Bytes) -> R<bool> {
    f105(v0, false)
}

pub fn f94(v0: Bytes) -> R<bool> {
    f105(v0, true)
}

pub fn f95(v0: Fn16, p500: Result<List<Adt1>, Adt2>, p501: List<Adt5>) -> R<Result<List<Adt1>, Adt2>> {
    let m499 = (p500, p501);
    {
        let mut m499 = m499;
        loop {
            let step = {
                let (v1, v2) = m499;
                let m497 = v2;
                match m497.uncons() {
                    None => {
                        Err::<(Result<List<Adt1>, Adt2>, List<Adt5>), R<Result<List<Adt1>, Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt1>, Adt2>, List<Adt5>), R<Result<List<Adt1>, Adt2>>>(({
                            let c498: Fn16 = v0.clone();
                            c498.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: u64, v1: Result<List<Adt1>, Adt2>, v2: Adt5) -> R<Result<List<Adt1>, Adt2>> {
    let m502 = v1;
    match m502 {
        Ok(v3) => {
            let m503 = f79(v2, v0)?;
            match m503 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt1>, Adt2>(List::cons(v4, v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt1>, Adt2>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt1>, Adt2>(v6))
        }
    }
}

pub fn f97(v0: List<Adt1>) -> Adt1 {
    f106(Fn17::F107, Adt1::C5, f98(v0))
}

pub fn f98(v0: List<Adt1>) -> List<Adt1> {
    f108(Fn18::F109, List::<Adt1>::nil(), v0)
}

pub fn f99(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a504 = v0.clone();
        length_bytes(a504)
    };
    let a508 = {
        let (h505, _) = f87(Fn14::F110(v1, v0), {
            let a506 = v1;
            let a507 = 1u64;
            nat_add(a506, a507)?
        }, 0u64)?;
        h505
    };
    let a509 = v1;
    Ok(nat_eq(a508, a509))
}

pub fn f100(v0: u64) -> bool {
    let a510 = v0;
    let a511 = 97u64;
    if nat_lt(a510, a511) {
        false
    } else {
        let a512 = v0;
        let a513 = 122u64;
        nat_le(a512, a513)
    }
}

pub fn f101(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a514 = v2;
    let a515 = v1;
    if nat_lt(a514, a515) {
        if f111(f20(v0, v2)?) {
            Ok(Some({
                let a516 = v2;
                let a517 = 1u64;
                nat_add(a516, a517)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f102(v0: u64, v1: Adt10) -> R<Option<Adt10>> {
    let m519 = match v1.clone() {
        Adt10::C0(_, _, h518) => {
            h518
        }
    };
    match m519 {
        None => {
            let m521 = match v1.clone() {
                Adt10::C0(h520, _, _) => {
                    h520
                }
            };
            match m521.uncons() {
                None => {
                    Ok(None::<Adt10>)
                }
                Some((v3, v4)) => {
                    let m522 = v3;
                    match m522 {
                        Adt11::C0(r523) => {
                            let v5 = (*r523).clone();
                            Ok(Some(f112(v0, v1, v4, v5)?))
                        }
                        Adt11::C1(v6, v7) => {
                            Ok(Some(f113(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt10>)
        }
    }
}

pub fn f103(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f104(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a526 = {
            let a524 = v4;
            let a525 = v1;
            nat_add(a524, a525)?
        };
        let a527 = v3;
        if nat_lt(a526, a527) {
            Ok(Some({
                let a528 = v4;
                let a529 = 1u64;
                nat_add(a528, a529)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f104(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m534 = {
        let a531 = v0;
        let a532 = v2;
        let a533 = {
            let a530 = v1.clone();
            length_bytes(a530)
        };
        slice_bytes(a531, a532, a533)
    };
    match m534 {
        None => {
            false
        }
        Some(v3) => {
            let a535 = v3;
            let a536 = v1;
            equal(a535, a536)
        }
    }
}

pub fn f105(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a537 = v0.clone();
        length_bytes(a537)
    };
    let v3: Adt12 = {
        let (h538, _) = f114(Fn19::F115(v1, v2, v0), {
            let a539 = v2;
            let a540 = 1u64;
            nat_add(a539, a540)?
        }, Adt12::C0(0u64, true, true, true))?;
        h538
    };
    if match v3.clone() {
        Adt12::C0(_, _, _, h541) => {
            h541
        }
    } {
        Ok(!match v3 {
            Adt12::C0(_, h542, _, _) => {
                h542
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f106(v0: Fn17, p546: Adt1, p547: List<Adt1>) -> Adt1 {
    let m545 = (p546, p547);
    {
        let mut m545 = m545;
        loop {
            let step = {
                let (v1, v2) = m545;
                let m543 = v2;
                match m543.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c544: Fn17 = v0.clone();
                            c544.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f108(v0: Fn18, p551: List<Adt1>, p552: List<Adt1>) -> List<Adt1> {
    let m550 = (p551, p552);
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v1, v2) = m550;
                let m548 = v2;
                match m548.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c549: Fn18 = v0.clone();
                            c549.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f110(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f116(v1, v0, v2)
}

pub fn f111(v0: u64) -> bool {
    if f86(v0) {
        true
    } else {
        f117(v0)
    }
}

pub fn f112(v0: u64, v1: Adt10, v2: List<Adt11>, v3: Adt5) -> R<Adt10> {
    let m553 = v3;
    match m553 {
        Adt5::C0(v4) => {
            if f99(v4.clone())? {
                let m554 = f83(v4.clone())?;
                match m554 {
                    None => {
                        Ok(Adt10::C0(v2, List::cons(Adt1::C0(f119(v4)?), match v1 {
                            Adt10::C0(_, h555, _) => {
                                h555
                            }
                        }), None::<Adt2>))
                    }
                    Some(v5) => {
                        Ok(f118(v1, v5))
                    }
                }
            } else {
                if f75(v4.clone())? {
                    Ok(Adt10::C0(v2, List::cons(Adt1::C1(v4), match v1 {
                        Adt10::C0(_, h556, _) => {
                            h556
                        }
                    }), None::<Adt2>))
                } else {
                    Ok(f118(v1, f10(f26(f53(v4), f11(Str::lit(" is not a universe"))))))
                }
            }
        }
        Adt5::C1(r579) => {
            let v6 = (*r579).clone();
            let v7: List<Adt5> = f30(v6, v0)?;
            let m557 = v7;
            match m557.uncons() {
                None => {
                    Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                }
                Some((v8, v9)) => {
                    let m558 = v8;
                    match m558 {
                        Adt5::C0(v10) => {
                            let v11: u64 = {
                                let a559 = v9.clone();
                                length_list(a559)
                            };
                            let v12: bool = {
                                let a560 = v10.clone();
                                let a561 = f11(Str::lit("succ"));
                                if equal(a560, a561) {
                                    true
                                } else {
                                    let a562 = v10.clone();
                                    let a563 = f11(Str::lit("max"));
                                    if equal(a562, a563) {
                                        true
                                    } else {
                                        let a564 = v10.clone();
                                        let a565 = f11(Str::lit("imax"));
                                        equal(a564, a565)
                                    }
                                }
                            };
                            let v13: bool = {
                                let a566 = v10.clone();
                                let a567 = f11(Str::lit("succ"));
                                if equal(a566, a567) {
                                    let a568 = v11;
                                    let a569 = 1u64;
                                    nat_eq(a568, a569)
                                } else {
                                    let a570 = v10.clone();
                                    let a571 = f11(Str::lit("max"));
                                    if equal(a570, a571) {
                                        let a572 = 2u64;
                                        let a573 = v11;
                                        nat_le(a572, a573)
                                    } else {
                                        let a574 = v10.clone();
                                        let a575 = f11(Str::lit("imax"));
                                        if equal(a574, a575) {
                                            let a576 = v11;
                                            let a577 = 2u64;
                                            nat_eq(a576, a577)
                                        } else {
                                            false
                                        }
                                    }
                                }
                            };
                            if v13 {
                                Ok(Adt10::C0(f120(v9, List::cons(Adt11::C1(v10, v11), v2)), match v1 {
                                    Adt10::C0(_, h578, _) => {
                                        h578
                                    }
                                }, None::<Adt2>))
                            } else {
                                if v12 {
                                    Ok(f118(v1, f10(f26(f26(f11(Str::lit("`(")), v10), f11(Str::lit(" ...)` has the wrong arity"))))))
                                } else {
                                    Ok(f118(v1, f10(f26(f53(v10), f11(Str::lit(" is not a universe form"))))))
                                }
                            }
                        }
                        Adt5::C1(_) => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                        Adt5::C2 => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
        }
        Adt5::C3(_, _) => {
            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
        }
    }
}

pub fn f113(v0: Adt10, v1: List<Adt11>, v2: Bytes, v3: u64) -> Adt10 {
    let v4: (List<Adt1>, List<Adt1>) = f121(match v0 {
        Adt10::C0(_, h580, _) => {
            h580
        }
    }, v3);
    let v5: List<Adt1> = {
        let (h581, _) = v4;
        h581
    };
    let v12: Adt1 = {
        let a582 = v2.clone();
        let a583 = f11(Str::lit("succ"));
        if equal(a582, a583) {
            let m584 = v5;
            match m584.uncons() {
                None => {
                    Adt1::C5
                }
                Some((v6, _)) => {
                    Adt1::C2(std::rc::Rc::new(v6))
                }
            }
        } else {
            let a585 = v2;
            let a586 = f11(Str::lit("imax"));
            if equal(a585, a586) {
                let m587 = v5;
                match m587.uncons() {
                    None => {
                        Adt1::C5
                    }
                    Some((v8, v9)) => {
                        let m588 = v9;
                        match m588.uncons() {
                            None => {
                                Adt1::C5
                            }
                            Some((v10, _)) => {
                                Adt1::C4(std::rc::Rc::new(v8), std::rc::Rc::new(v10))
                            }
                        }
                    }
                }
            } else {
                Adt1::C3(std::rc::Rc::new(f97(v5)))
            }
        }
    };
    Adt10::C0(v1, List::cons(v12, {
        let (_, h589) = v4;
        h589
    }), None::<Adt2>)
}

pub fn f114(v0: Fn19, p594: u64, p595: Adt12) -> R<(Adt12, bool)> {
    let m593 = (p594, p595);
    {
        let mut m593 = m593;
        loop {
            let step = {
                let (v1, v2) = m593;
                let m590 = v1;
                if m590 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m590 - 1;
                    let m592 = {
                        let c591: Fn19 = v0.clone();
                        c591.apply(v2.clone())?
                    };
                    match m592 {
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
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: bool, v1: u64, v2: Bytes, v3: Adt12) -> R<Option<Adt12>> {
    f122(v2, v1, v0, v3)
}

pub fn f116(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a596 = v2;
    let a597 = v1;
    if nat_lt(a596, a597) {
        if f117(f20(v0, v2)?) {
            Ok(Some({
                let a598 = v2;
                let a599 = 1u64;
                nat_add(a598, a599)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f117(v0: u64) -> bool {
    let a600 = v0;
    let a601 = 48u64;
    if nat_lt(a600, a601) {
        false
    } else {
        let a602 = v0;
        let a603 = 57u64;
        nat_le(a602, a603)
    }
}

pub fn f118(v0: Adt10, v1: Adt2) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt10::C0(h604, _, _) => {
            h604
        }
    }, match v0 {
        Adt10::C0(_, h605, _) => {
            h605
        }
    }, Some(v1))
}

pub fn f119(v0: Bytes) -> R<u64> {
    let m606 = f123(v0)?;
    match m606 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f120(v0: List<Adt5>, v1: List<Adt11>) -> List<Adt11> {
    f124(Fn20::F125, v1, f45(v0))
}

pub fn f121(v0: List<Adt1>, v1: u64) -> (List<Adt1>, List<Adt1>) {
    let (h607, _) = f126(Fn21::F127, v1, (List::<Adt1>::nil(), v0));
    h607
}

pub fn f122(v0: Bytes, v1: u64, v2: bool, v3: Adt12) -> R<Option<Adt12>> {
    if match v3.clone() {
        Adt12::C0(_, _, _, h608) => {
            h608
        }
    } {
        let a610 = match v3.clone() {
            Adt12::C0(h609, _, _, _) => {
                h609
            }
        };
        let a611 = v1;
        if nat_lt(a610, a611) {
            let v4: u64 = f20(v0, match v3.clone() {
                Adt12::C0(h612, _, _, _) => {
                    h612
                }
            })?;
            let a613 = v4;
            let a614 = 46u64;
            if nat_eq(a613, a614) {
                if match v3.clone() {
                    Adt12::C0(_, h615, _, _) => {
                        h615
                    }
                } {
                    Ok(f128(v3))
                } else {
                    f129(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt12::C0(_, h616, _, _) => {
                        h616
                    }
                } {
                    if f100(v4) {
                        f129(v3.clone(), false, match v3 {
                            Adt12::C0(_, _, h617, _) => {
                                h617
                            }
                        })
                    } else {
                        Ok(f128(v3))
                    }
                } else {
                    if if f100(v4) {
                        true
                    } else {
                        f117(v4)
                    } {
                        f129(v3.clone(), false, match v3 {
                            Adt12::C0(_, _, h618, _) => {
                                h618
                            }
                        })
                    } else {
                        let a619 = v4;
                        let a620 = 45u64;
                        if if nat_eq(a619, a620) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt12::C0(_, _, h621, _) => {
                                        h621
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f129(v3.clone(), false, match v3 {
                                Adt12::C0(_, _, h622, _) => {
                                    h622
                                }
                            })
                        } else {
                            Ok(f128(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt12>)
        }
    } else {
        Ok(None::<Adt12>)
    }
}

pub fn f123(v0: Bytes) -> R<Option<u64>> {
    if f130(v0.clone())? {
        Ok(Some({
            let (_, h623) = {
                let (h624, _) = f131(Fn22::F132(v0.clone()), {
                    let a625 = v0;
                    length_bytes(a625)
                }, (0u64, 0u64))?;
                h624
            };
            h623
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f124(v0: Fn20, p629: List<Adt11>, p630: List<Adt5>) -> List<Adt11> {
    let m628 = (p629, p630);
    {
        let mut m628 = m628;
        loop {
            let step = {
                let (v1, v2) = m628;
                let m626 = v2;
                match m626.uncons() {
                    None => {
                        Err::<(List<Adt11>, List<Adt5>), List<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt11>, List<Adt5>), List<Adt11>>(({
                            let c627: Fn20 = v0.clone();
                            c627.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<Adt11>, v1: Adt5) -> List<Adt11> {
    List::cons(Adt11::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f126(v0: Fn21, p635: u64, p636: (List<Adt1>, List<Adt1>)) -> ((List<Adt1>, List<Adt1>), bool) {
    let m634 = (p635, p636);
    {
        let mut m634 = m634;
        loop {
            let step = {
                let (v1, v2) = m634;
                let m631 = v1;
                if m631 == 0 {
                    Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, false))
                } else {
                    let v3 = m631 - 1;
                    let m633 = {
                        let c632: Fn21 = v0.clone();
                        c632.apply(v2.clone())
                    };
                    match m633 {
                        None => {
                            Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
    let m638 = {
        let (_, h637) = v0;
        h637
    };
    match m638.uncons() {
        None => {
            None::<(List<Adt1>, List<Adt1>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h639, _) = v0;
                h639
            }), v2))
        }
    }
}

pub fn f128(v0: Adt12) -> Option<Adt12> {
    Some(Adt12::C0(match v0.clone() {
        Adt12::C0(h640, _, _, _) => {
            h640
        }
    }, match v0.clone() {
        Adt12::C0(_, h641, _, _) => {
            h641
        }
    }, match v0 {
        Adt12::C0(_, _, h642, _) => {
            h642
        }
    }, false))
}

pub fn f129(v0: Adt12, v1: bool, v2: bool) -> R<Option<Adt12>> {
    Ok(Some(Adt12::C0({
        let a644 = match v0 {
            Adt12::C0(h643, _, _, _) => {
                h643
            }
        };
        let a645 = 1u64;
        nat_add(a644, a645)?
    }, v1, v2, true)))
}

pub fn f130(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a646 = v0.clone();
        length_bytes(a646)
    };
    let a647 = 0u64;
    let a648 = v1;
    if nat_lt(a647, a648) {
        let a649 = f133(v0.clone())?;
        let a650 = v1;
        if nat_eq(a649, a650) {
            let a651 = 1u64;
            let a652 = v1;
            if nat_lt(a651, a652) {
                let m655 = {
                    let a653 = v0;
                    let a654 = 0u64;
                    index_bytes(a653, a654)
                };
                match m655 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a656 = v2;
                        let a657 = 48u8;
                        Ok(!equal(a656, a657))
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

pub fn f131(v0: Fn22, p662: u64, p663: (u64, u64)) -> R<((u64, u64), bool)> {
    let m661 = (p662, p663);
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v1, v2) = m661;
                let m658 = v1;
                if m658 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m658 - 1;
                    let m660 = {
                        let c659: Fn22 = v0.clone();
                        c659.apply(v2)?
                    };
                    match m660 {
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
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f134(v0, v1)
}

pub fn f133(v0: Bytes) -> R<u64> {
    let (h664, _) = f87(Fn14::F135(v0.clone()), {
        let a665 = v0;
        length_bytes(a665)
    }, 0u64)?;
    Ok(h664)
}

pub fn f134(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m669 = {
        let a667 = v0;
        let a668 = {
            let (h666, _) = v1;
            h666
        };
        index_bytes(a667, a668)
    };
    match m669 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a671 = {
                    let (h670, _) = v1;
                    h670
                };
                let a672 = 1u64;
                nat_add(a671, a672)?
            }, {
                let a676 = {
                    let a674 = {
                        let (_, h673) = v1;
                        h673
                    };
                    let a675 = 10u64;
                    nat_mul(a674, a675)?
                };
                let a677 = f136(v2)?;
                nat_add(a676, a677)?
            })))
        }
    }
}

pub fn f135(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m680 = {
        let a678 = v0;
        let a679 = v1;
        index_bytes(a678, a679)
    };
    match m680 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f137(v2)? {
                Ok(Some({
                    let a681 = v1;
                    let a682 = 1u64;
                    nat_add(a681, a682)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f136(v0: u8) -> R<u64> {
    let a729 = {
        let v1: u8 = v0;
        let a727 = {
            let a703 = {
                let a691 = {
                    let a685 = {
                        let a683 = v1;
                        let a684 = 128u8;
                        fixed_u8::bit_and(a683, a684)
                    };
                    let a686 = 128u8;
                    if equal(a685, a686) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a692 = {
                    let a689 = {
                        let a687 = v1;
                        let a688 = 64u8;
                        fixed_u8::bit_and(a687, a688)
                    };
                    let a690 = 64u8;
                    if equal(a689, a690) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a691, a692)?
            };
            let a704 = {
                let a701 = {
                    let a695 = {
                        let a693 = v1;
                        let a694 = 32u8;
                        fixed_u8::bit_and(a693, a694)
                    };
                    let a696 = 32u8;
                    if equal(a695, a696) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a702 = {
                    let a699 = {
                        let a697 = v1;
                        let a698 = 16u8;
                        fixed_u8::bit_and(a697, a698)
                    };
                    let a700 = 16u8;
                    if equal(a699, a700) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a701, a702)?
            };
            nat_add(a703, a704)?
        };
        let a728 = {
            let a725 = {
                let a713 = {
                    let a707 = {
                        let a705 = v1;
                        let a706 = 8u8;
                        fixed_u8::bit_and(a705, a706)
                    };
                    let a708 = 8u8;
                    if equal(a707, a708) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a714 = {
                    let a711 = {
                        let a709 = v1;
                        let a710 = 4u8;
                        fixed_u8::bit_and(a709, a710)
                    };
                    let a712 = 4u8;
                    if equal(a711, a712) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a713, a714)?
            };
            let a726 = {
                let a723 = {
                    let a717 = {
                        let a715 = v1;
                        let a716 = 2u8;
                        fixed_u8::bit_and(a715, a716)
                    };
                    let a718 = 2u8;
                    if equal(a717, a718) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a724 = {
                    let a721 = {
                        let a719 = v1;
                        let a720 = 1u8;
                        fixed_u8::bit_and(a719, a720)
                    };
                    let a722 = 1u8;
                    if equal(a721, a722) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a723, a724)?
            };
            nat_add(a725, a726)?
        };
        nat_add(a727, a728)?
    };
    let a730 = 48u64;
    Ok(nat_sub(a729, a730))
}

pub fn f137(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a775 = {
            let a751 = {
                let a739 = {
                    let a733 = {
                        let a731 = v1;
                        let a732 = 128u8;
                        fixed_u8::bit_and(a731, a732)
                    };
                    let a734 = 128u8;
                    if equal(a733, a734) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a740 = {
                    let a737 = {
                        let a735 = v1;
                        let a736 = 64u8;
                        fixed_u8::bit_and(a735, a736)
                    };
                    let a738 = 64u8;
                    if equal(a737, a738) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a739, a740)?
            };
            let a752 = {
                let a749 = {
                    let a743 = {
                        let a741 = v1;
                        let a742 = 32u8;
                        fixed_u8::bit_and(a741, a742)
                    };
                    let a744 = 32u8;
                    if equal(a743, a744) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a750 = {
                    let a747 = {
                        let a745 = v1;
                        let a746 = 16u8;
                        fixed_u8::bit_and(a745, a746)
                    };
                    let a748 = 16u8;
                    if equal(a747, a748) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a749, a750)?
            };
            nat_add(a751, a752)?
        };
        let a776 = {
            let a773 = {
                let a761 = {
                    let a755 = {
                        let a753 = v1;
                        let a754 = 8u8;
                        fixed_u8::bit_and(a753, a754)
                    };
                    let a756 = 8u8;
                    if equal(a755, a756) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a762 = {
                    let a759 = {
                        let a757 = v1;
                        let a758 = 4u8;
                        fixed_u8::bit_and(a757, a758)
                    };
                    let a760 = 4u8;
                    if equal(a759, a760) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a761, a762)?
            };
            let a774 = {
                let a771 = {
                    let a765 = {
                        let a763 = v1;
                        let a764 = 2u8;
                        fixed_u8::bit_and(a763, a764)
                    };
                    let a766 = 2u8;
                    if equal(a765, a766) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a772 = {
                    let a769 = {
                        let a767 = v1;
                        let a768 = 1u8;
                        fixed_u8::bit_and(a767, a768)
                    };
                    let a770 = 1u8;
                    if equal(a769, a770) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a771, a772)?
            };
            nat_add(a773, a774)?
        };
        nat_add(a775, a776)?
    };
    let a777 = 48u64;
    let a778 = v2;
    if nat_le(a777, a778) {
        let a779 = v2;
        let a780 = 57u64;
        Ok(nat_le(a779, a780))
    } else {
        Ok(false)
    }
}

pub fn run(p0: Bytes, p1: u64) -> R<Result<Adt0, Adt2>> {
    f0(p0, p1)
}
