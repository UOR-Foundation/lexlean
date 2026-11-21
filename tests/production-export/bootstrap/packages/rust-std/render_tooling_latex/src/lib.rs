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
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, List<Adt2>, List<Adt4>, bool, Option<Adt6>, std::rc::Rc<Adt8>),
    C1(Bytes, std::rc::Rc<Adt9>, Bytes, std::rc::Rc<Adt8>),
    C2(List<Bytes>),
    C3(Bytes, bool, bool, Option<Bytes>, List<Bytes>, std::rc::Rc<Adt5>, std::rc::Rc<Adt6>, std::rc::Rc<Adt11>),
    C4(Bytes, List<Adt12>, Bytes),
    C5(Bytes, List<Adt13>),
    C6(Bytes, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, std::rc::Rc<Adt5>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, std::rc::Rc<Adt6>),
    C1(Bytes, std::rc::Rc<Adt6>),
    C2(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C3(Bytes),
    C4,
    C5(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes),
    C1(Bytes),
    C2(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C3(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C4(Option<Bytes>, Adt10, std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C5(Option<Bytes>),
    C6(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C7,
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes),
    C1(Bytes),
    C2(Bytes),
    C3(u64),
    C4(Bytes),
    C5(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt11>),
    C6(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt11>),
    C7(std::rc::Rc<Adt11>, Bytes),
    C8(std::rc::Rc<Adt6>, std::rc::Rc<Adt11>),
    C9(std::rc::Rc<Adt6>, std::rc::Rc<Adt11>),
    C10(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C11(std::rc::Rc<Adt5>, std::rc::Rc<Adt11>),
    C12(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C13(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C14(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C15,
    C16(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C17,
    C18(Option<Bytes>, List<Bytes>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes),
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes),
    C1(std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>, List<Adt0>, List<Bytes>, List<Adt15>, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(std::rc::Rc<Adt14>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt16, p1: Adt1) -> R<Adt16> {
        match self {
            Fn0::F5(k0) => f5((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt1) -> List<Bytes> {
        match self {
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt15>, p1: Adt1) -> List<Adt15> {
        match self {
            Fn2::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F18,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn3::F18 => f18(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F20,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn4::F20 => f20(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F44,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> List<Bytes> {
        match self {
            Fn5::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F46,
}

impl Fn6 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn6::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F51,
}

impl Fn7 {
    pub fn apply(&self, p0: (bool, Adt16), p1: Adt2) -> R<(bool, Adt16)> {
        match self {
            Fn7::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F53(List<Bytes>),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt16, p1: Adt4) -> R<Adt16> {
        match self {
            Fn8::F53(k0) => f53(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F55(u64, bool, Bytes),
    F104(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
        match self {
            Fn9::F55(k0, k1, k2) => f55(*k0, *k1, k2.clone(), p0),
            Fn9::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F59(Str),
    F69(List<Bytes>),
}

impl Fn10 {
    pub fn apply(&self, p0: (bool, Adt16), p1: Bytes) -> R<(bool, Adt16)> {
        match self {
            Fn10::F59(k0) => f59(k0.clone(), p0, p1),
            Fn10::F69(k0) => f69(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66(std::rc::Rc<Adt14>),
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Adt13>, Adt16), p1: Adt12) -> R<(List<Adt13>, Adt16)> {
        match self {
            Fn11::F66(k0) => f66((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F78(List<Bytes>),
    F110(List<Bytes>),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt16, p1: Bytes) -> R<Adt16> {
        match self {
            Fn12::F78(k0) => f78(k0.clone(), p0, p1),
            Fn12::F110(k0) => f110(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F88,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt16, p1: Adt12) -> R<Adt16> {
        match self {
            Fn13::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F93(u8, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn14::F93(k0, k1) => f93(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F99(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn15::F99(k0) => f99(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F101(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<List<Bytes>>, p1: Adt15) -> Option<List<Bytes>> {
        match self {
            Fn16::F101(k0) => f101(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F103(Bytes),
    F118(Bytes),
    F121(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn17::F103(k0) => f103(k0.clone(), p0),
            Fn17::F118(k0) => f118(k0.clone(), p0),
            Fn17::F121(k0) => Ok(f121(k0.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F108(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt0) -> List<Bytes> {
        match self {
            Fn18::F108(k0) => f108(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F113(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Option<List<Adt12>>, p1: Adt1) -> Option<List<Adt12>> {
        match self {
            Fn19::F113(k0) => f113(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64, v1: List<Bytes>, v2: List<Adt0>, v3: List<Adt1>) -> R<Result<Bytes, Bytes>> {
    let v4: Adt14 = f1(v1, v2, v3.clone());
    Ok(f2(f3(f4(Fn0::F5(std::rc::Rc::new(v4)), f3(f6(v0), Str::lit("\\documentclass[11pt]{article}\u{a}\\usepackage[T1]{fontenc}\u{a}\\usepackage{amsmath,amssymb}\u{a}\\begin{document}\u{a}\\section*{Tooling declarations}\u{a}"))?, v3)?, Str::lit("\\end{document}\u{a}"))?))
}

pub fn f1(v0: List<Bytes>, v1: List<Adt0>, v2: List<Adt1>) -> Adt14 {
    Adt14::C0(v0, v1, f7(v2.clone()), f8(v2.clone()), v2)
}

pub fn f2(v0: Adt16) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt16::C0(_, _, _, _, h1) => {
            h1
        }
    } {
        Err::<Bytes, Bytes>({
            let a2 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a2)
        })
    } else {
        Ok::<Bytes, Bytes>(f9(f10(List::cons(match v0.clone() {
            Adt16::C0(_, h3, _, _, _) => {
                h3
            }
        }, match v0 {
            Adt16::C0(h4, _, _, _, _) => {
                h4
            }
        }))))
    }
}

pub fn f3(v0: Adt16, v1: Str) -> R<Adt16> {
    f11(v0, {
        let a5 = v1;
        utf8_encode(a5)
    })
}

pub fn f4(v0: Fn0, p9: Adt16, p10: List<Adt1>) -> R<Adt16> {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt1>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt1>), R<Adt16>>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Adt14, v1: Adt16, v2: Adt1) -> R<Adt16> {
    f12(v1, v0, v2)
}

pub fn f6(v0: u64) -> Adt16 {
    Adt16::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f7(v0: List<Adt1>) -> List<Bytes> {
    f13(Fn1::F14, List::<Bytes>::nil(), v0)
}

pub fn f8(v0: List<Adt1>) -> List<Adt15> {
    f15(Fn2::F16, List::<Adt15>::nil(), v0)
}

pub fn f9(v0: List<Bytes>) -> Bytes {
    let m13 = {
        let (h11, _) = f17(Fn3::F18, {
            let a12 = v0.clone();
            length_list(a12)
        }, v0);
        h11
    };
    match m13.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f10(v0: List<Bytes>) -> List<Bytes> {
    f19(Fn4::F20, List::<Bytes>::nil(), v0)
}

pub fn f11(v0: Adt16, v1: Bytes) -> R<Adt16> {
    if match v0.clone() {
        Adt16::C0(_, _, _, _, h14) => {
            h14
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a17 = match v0.clone() {
                Adt16::C0(_, _, h15, _, _) => {
                    h15
                }
            };
            let a18 = {
                let a16 = v1.clone();
                length_bytes(a16)
            };
            nat_add(a17, a18)?
        };
        let a20 = v2;
        let a21 = match v0.clone() {
            Adt16::C0(_, _, _, h19, _) => {
                h19
            }
        };
        if nat_le(a20, a21) {
            Ok(f21(v0.clone(), {
                let a23 = match v0 {
                    Adt16::C0(_, h22, _, _, _) => {
                        h22
                    }
                };
                let a24 = v1;
                append_bytes(a23, a24)
            }, v2))
        } else {
            Ok(f22(v0))
        }
    }
}

pub fn f12(v0: Adt16, v1: Adt14, v2: Adt1) -> R<Adt16> {
    let m25 = v2.clone();
    match m25 {
        Adt1::C0(v3, v4, v5, v6, v7, r27) => {
            let v8 = (*r27).clone();
            f23(f24(v0, v3, Str::lit("type"))?, v1, v4, v5, v6, v7, v8)
        }
        Adt1::C1(v9, r28, v11, r29) => {
            let v10 = (*r28).clone();
            let v12 = (*r29).clone();
            f25(f24(v0, v9, Str::lit("call"))?, v1, v10, v11, v12)
        }
        Adt1::C2(v13) => {
            f26(f24(v0, {
                let a26 = Str::lit("plumbing");
                utf8_encode(a26)
            }, Str::lit("plumbing"))?, v13)
        }
        Adt1::C3(v14, v15, v16, v17, _, _, _, _) => {
            f27(f28(f24(v0, v14, Str::lit("function"))?, v15, v16, v17)?, v1, v2, true)
        }
        Adt1::C4(v22, _, _) => {
            f27(f24(v0, v22, Str::lit("command"))?, v1, v2, true)
        }
        Adt1::C5(v25, _) => {
            f27(f24(v0, v25, Str::lit("invoke"))?, v1, v2, true)
        }
        Adt1::C6(v27, _) => {
            f27(f24(v0, v27, Str::lit("evaluate"))?, v1, v2, false)
        }
    }
}

pub fn f13(v0: Fn1, p33: List<Bytes>, p34: List<Adt1>) -> List<Bytes> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v2;
                match m30.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt1>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt1>), List<Bytes>>(({
                            let c31: Fn1 = v0.clone();
                            c31.apply(v1, v3)
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

pub fn f14(v0: List<Bytes>, v1: Adt1) -> List<Bytes> {
    let m35 = v1;
    match m35 {
        Adt1::C0(_, _, _, _, _, _) => {
            v0
        }
        Adt1::C1(v8, _, _, _) => {
            List::cons(v8, v0)
        }
        Adt1::C2(_) => {
            v0
        }
        Adt1::C3(_, _, _, _, _, _, _, _) => {
            v0
        }
        Adt1::C4(_, _, _) => {
            v0
        }
        Adt1::C5(_, _) => {
            v0
        }
        Adt1::C6(_, _) => {
            v0
        }
    }
}

pub fn f15(v0: Fn2, p39: List<Adt15>, p40: List<Adt1>) -> List<Adt15> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt1>), List<Adt15>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt15>, List<Adt1>), List<Adt15>>(({
                            let c37: Fn2 = v0.clone();
                            c37.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt15>, v1: Adt1) -> List<Adt15> {
    let m41 = v1;
    match m41 {
        Adt1::C0(v2, v3, _, _, _, _) => {
            List::cons(Adt15::C0(v2, f29(v3)), v0)
        }
        Adt1::C1(_, _, _, _) => {
            v0
        }
        Adt1::C2(_) => {
            v0
        }
        Adt1::C3(_, _, _, _, _, _, _, _) => {
            v0
        }
        Adt1::C4(_, _, _) => {
            v0
        }
        Adt1::C5(_, _) => {
            v0
        }
        Adt1::C6(_, _) => {
            v0
        }
    }
}

pub fn f17(v0: Fn3, p46: u64, p47: List<Bytes>) -> (List<Bytes>, bool) {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn3 = v0.clone();
                        c43.apply(v2.clone())
                    };
                    match m44 {
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
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<Bytes>) -> Option<List<Bytes>> {
    f30(v0)
}

pub fn f19(v0: Fn4, p51: List<Bytes>, p52: List<Bytes>) -> List<Bytes> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c49: Fn4 = v0.clone();
                            c49.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f21(v0: Adt16, v1: Bytes, v2: u64) -> Adt16 {
    let a54 = 256u64;
    let a55 = {
        let a53 = v1.clone();
        length_bytes(a53)
    };
    if nat_le(a54, a55) {
        Adt16::C0(List::cons(v1, match v0.clone() {
            Adt16::C0(h56, _, _, _, _) => {
                h56
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt16::C0(_, _, _, h57, _) => {
                h57
            }
        }, false)
    } else {
        Adt16::C0(match v0.clone() {
            Adt16::C0(h58, _, _, _, _) => {
                h58
            }
        }, v1, v2, match v0 {
            Adt16::C0(_, _, _, h59, _) => {
                h59
            }
        }, false)
    }
}

pub fn f22(v0: Adt16) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(h60, _, _, _, _) => {
            h60
        }
    }, match v0.clone() {
        Adt16::C0(_, h61, _, _, _) => {
            h61
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h62, _, _) => {
            h62
        }
    }, match v0 {
        Adt16::C0(_, _, _, h63, _) => {
            h63
        }
    }, true)
}

pub fn f23(v0: Adt16, v1: Adt14, v2: List<Adt2>, v3: List<Adt4>, v4: bool, v5: Option<Adt6>, v6: Adt8) -> R<Adt16> {
    let v7: List<Bytes> = match v1 {
        Adt14::C0(h64, _, _, _, _) => {
            h64
        }
    };
    let v8: Adt16 = f31(v0, v6)?;
    let v9: Adt16 = if v4 {
        f32(v8.clone(), f3(f33(v8), Str::lit("Constructors are not pinned (opaque)"))?)?
    } else {
        v8
    };
    let v11: Adt16 = {
        let m65 = v5;
        match m65 {
            None => {
                v9
            }
            Some(v10) => {
                f32(v9.clone(), f34(f3(f33(v9), Str::lit("Abbreviation of: "))?, v7.clone(), v10, 0u64, true)?)?
            }
        }
    };
    let v12: Adt16 = {
        let a67 = 0u64;
        let a68 = {
            let a66 = v2.clone();
            length_list(a66)
        };
        if nat_lt(a67, a68) {
            f32(v11.clone(), f35(f3(f33(v11), Str::lit("Parameters: "))?, v2)?)?
        } else {
            v11
        }
    };
    f36(v12, v7, v3)
}

pub fn f24(v0: Adt16, v1: Bytes, v2: Str) -> R<Adt16> {
    f3(f3(f3(f37(f3(v0, Str::lit("\\subsection*{\\texttt{"))?, v1.clone(), {
        let a69 = v1;
        length_bytes(a69)
    }, false)?, Str::lit("}}\u{a}\\noindent Kind: \\texttt{"))?, v2)?, Str::lit("}.\\par\u{a}"))
}

pub fn f25(v0: Adt16, v1: Adt14, v2: Adt9, v3: Bytes, v4: Adt8) -> R<Adt16> {
    f31(f32(f32(v0.clone(), f38(f3(f33(v0.clone()), Str::lit("Signature: "))?, v1, v2, true)?)?, f39(f3(f33(v0), Str::lit("Role: "))?, v3)?)?, v4)
}

pub fn f26(v0: Adt16, v1: List<Bytes>) -> R<Adt16> {
    f32(v0.clone(), f40(f3(f33(v0), Str::lit("Names: "))?, v1, Str::lit(", "))?)
}

pub fn f27(v0: Adt16, v1: Adt14, v2: Adt1, v3: bool) -> R<Adt16> {
    f41(v0.clone(), f42(f33(v0), v1, v2)?, true, v3)
}

pub fn f28(v0: Adt16, v1: bool, v2: bool, v3: Option<Bytes>) -> R<Adt16> {
    let v4: Adt16 = f3(f3(f3(f3(f33(v0.clone()), Str::lit("Visibility: "))?, if v1 {
        Str::lit("public")
    } else {
        Str::lit("private")
    })?, Str::lit("; recursion: "))?, if v2 {
        Str::lit("partial")
    } else {
        Str::lit("total")
    })?;
    f32(v0, {
        let m70 = v3;
        match m70 {
            None => {
                v4
            }
            Some(v5) => {
                f39(f3(v4, Str::lit("; mutual group "))?, v5)?
            }
        }
    })
}

pub fn f29(v0: List<Adt2>) -> List<Bytes> {
    f10(f43(Fn5::F44, List::<Bytes>::nil(), v0))
}

pub fn f30(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m71 = v0.clone();
    match m71.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m72 = v2;
            match m72.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f45(Fn6::F46, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m74 = {
                        let (h73, _) = v5;
                        h73
                    };
                    match m74 {
                        None => {
                            Some(f10({
                                let (_, h76) = v5;
                                h76
                            }))
                        }
                        Some(v6) => {
                            Some(f10(List::cons(v6, {
                                let (_, h75) = v5;
                                h75
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt16, v1: Adt8) -> R<Adt16> {
    f32(v0.clone(), f3(f39(f3(f39(f3(f33(v0), Str::lit("Source: "))?, match v1.clone() {
        Adt8::C0(h77, _) => {
            h77
        }
    })?, Str::lit(" ("))?, match v1 {
        Adt8::C0(_, h78) => {
            h78
        }
    })?, Str::lit(")"))?)
}

pub fn f32(v0: Adt16, v1: Adt16) -> R<Adt16> {
    f41(v0, v1, false, false)
}

pub fn f33(v0: Adt16) -> Adt16 {
    f6(match v0 {
        Adt16::C0(_, _, _, h79, _) => {
            h79
        }
    })
}

pub fn f34(v0: Adt16, v1: List<Bytes>, v2: Adt6, v3: u64, v4: bool) -> R<Adt16> {
    let m80 = v2;
    match m80 {
        Adt6::C0(v5, r83) => {
            let v6 = (*r83).clone();
            if f47(v6.clone()) {
                f48(v0, v1, v5)
            } else {
                f3(f34(f3(f48(f3(v0, Str::lit("("))?, v1.clone(), v5)?, Str::lit(" "))?, v1, v6, 1u64, true)?, Str::lit(")"))
            }
        }
        Adt6::C1(v7, r84) => {
            let v8 = (*r84).clone();
            if f47(v8.clone()) {
                f48(v0, v1, v7)
            } else {
                f3(f34(f3(f48(f3(v0, Str::lit("("))?, v1.clone(), v7)?, Str::lit(" "))?, v1, v8, 1u64, true)?, Str::lit(")"))
            }
        }
        Adt6::C2(r85, r86) => {
            let v9 = (*r85).clone();
            let v10 = (*r86).clone();
            f3(f34(f49(f34(f3(v0, Str::lit("("))?, v1.clone(), v9, 2u64, true)?)?, v1, v10, 0u64, true)?, Str::lit(")"))
        }
        Adt6::C3(v11) => {
            f48(v0, v1, v11)
        }
        Adt6::C4 => {
            Ok(v0)
        }
        Adt6::C5(r87, r88) => {
            let v12 = (*r87).clone();
            let v13 = (*r88).clone();
            f34(f34(if v4 {
                v0
            } else {
                let a81 = v3;
                let a82 = 2u64;
                if nat_eq(a81, a82) {
                    f49(v0)?
                } else {
                    f3(v0, Str::lit(" "))?
                }
            }, v1.clone(), v12, 0u64, true)?, v1, v13, v3, false)
        }
    }
}

pub fn f35(v0: Adt16, v1: List<Adt2>) -> R<Adt16> {
    let (_, h89) = f50(Fn7::F51, (true, v0), v1)?;
    Ok(h89)
}

pub fn f36(v0: Adt16, v1: List<Bytes>, v2: List<Adt4>) -> R<Adt16> {
    f52(Fn8::F53(v1), v0, v2)
}

pub fn f37(v0: Adt16, v1: Bytes, v2: u64, v3: bool) -> R<Adt16> {
    let (_, h90) = {
        let (h91, _) = f54(Fn9::F55(v2, v3, v1), v2, (0u64, v0))?;
        h91
    };
    Ok(h90)
}

pub fn f38(v0: Adt16, v1: Adt14, v2: Adt9, v3: bool) -> R<Adt16> {
    let m92 = v2;
    match m92 {
        Adt9::C0(v4) => {
            f48(v0, match v1 {
                Adt14::C0(h93, _, _, _, _) => {
                    h93
                }
            }, v4)
        }
        Adt9::C1(v5) => {
            f48(v0, match v1 {
                Adt14::C0(h94, _, _, _, _) => {
                    h94
                }
            }, v5)
        }
        Adt9::C2(r101, r102) => {
            let v6 = (*r101).clone();
            let v7 = (*r102).clone();
            let v8: Adt16 = if f56(v1.clone(), v6.clone(), f57(v7.clone())?) {
                f3(v0, Str::lit("(@"))?
            } else {
                f3(v0, Str::lit("("))?
            };
            f3(f38(f3(f38(v8, v1.clone(), v6, true)?, Str::lit(" "))?, v1, v7, true)?, Str::lit(")"))
        }
        Adt9::C3(r103, r104) => {
            let v9 = (*r103).clone();
            let v10 = (*r104).clone();
            f3(f38(f49(f38(f3(v0, Str::lit("("))?, v1.clone(), v9, true)?)?, v1, v10, true)?, Str::lit(")"))
        }
        Adt9::C4(v11, v12, r105, r106) => {
            let v13 = (*r105).clone();
            let v14 = (*r106).clone();
            let v15: Adt16 = f3(v0, Str::lit("("))?;
            f3(f38(f49({
                let m95 = v11;
                match m95 {
                    None => {
                        f3(f38(f3(v15, Str::lit("["))?, v1.clone(), v13, true)?, Str::lit("]"))?
                    }
                    Some(v16) => {
                        let m96 = v12;
                        match m96 {
                            Adt10::C0 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("("))?, match v1.clone() {
                                    Adt14::C0(h97, _, _, _, _) => {
                                        h97
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit(")"))?
                            }
                            Adt10::C1 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("{"))?, match v1.clone() {
                                    Adt14::C0(h98, _, _, _, _) => {
                                        h98
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit("}"))?
                            }
                            Adt10::C2 => {
                                f3(f38(f3(f48(f3(v15, Str::lit("["))?, match v1.clone() {
                                    Adt14::C0(h99, _, _, _, _) => {
                                        h99
                                    }
                                }, v16)?, Str::lit(" : "))?, v1.clone(), v13, true)?, Str::lit("]"))?
                            }
                        }
                    }
                }
            })?, v1, v14, true)?, Str::lit(")"))
        }
        Adt9::C5(v17) => {
            let m100 = v17;
            match m100 {
                None => {
                    f3(v0, Str::lit("Type"))
                }
                Some(v18) => {
                    f3(f39(f3(v0, Str::lit("(Type "))?, v18)?, Str::lit(")"))
                }
            }
        }
        Adt9::C6(r107, r108) => {
            let v19 = (*r107).clone();
            let v20 = (*r108).clone();
            f3(f38(f3(f38(f3(v0, Str::lit("(optParam "))?, v1.clone(), v19, true)?, Str::lit(" "))?, v1, v20, true)?, Str::lit(")"))
        }
        Adt9::C7 => {
            f3(v0, Str::lit("{ }"))
        }
        Adt9::C8(v21) => {
            f39(v0, v21)
        }
        Adt9::C9 => {
            Ok(v0)
        }
        Adt9::C10(r109, r110) => {
            let v22 = (*r109).clone();
            let v23 = (*r110).clone();
            f38(f38(if v3 {
                v0
            } else {
                f3(v0, Str::lit(" "))?
            }, v1.clone(), v22, true)?, v1, v23, false)
        }
    }
}

pub fn f39(v0: Adt16, v1: Bytes) -> R<Adt16> {
    f11(v0, v1)
}

pub fn f40(v0: Adt16, v1: List<Bytes>, v2: Str) -> R<Adt16> {
    let (_, h111) = f58(Fn10::F59(v2), (true, v0), v1)?;
    Ok(h111)
}

pub fn f41(v0: Adt16, v1: Adt16, v2: bool, v3: bool) -> R<Adt16> {
    let m112 = f2(v1);
    match m112 {
        Ok(v4) => {
            f60(v0, v4, v2, v3)
        }
        Err(_) => {
            Ok(f22(v0))
        }
    }
}

pub fn f42(v0: Adt16, v1: Adt14, v2: Adt1) -> R<Adt16> {
    let v3: List<Bytes> = match v1.clone() {
        Adt14::C0(h113, _, _, _, _) => {
            h113
        }
    };
    let m114 = v2;
    match m114 {
        Adt1::C0(_, _, _, _, _, _) => {
            Ok(v0)
        }
        Adt1::C1(_, _, _, _) => {
            Ok(v0)
        }
        Adt1::C2(_) => {
            Ok(v0)
        }
        Adt1::C3(v15, v16, v17, _, v19, r117, r118, r119) => {
            let v20 = (*r117).clone();
            let v21 = (*r118).clone();
            let v22 = (*r119).clone();
            let v23: Adt16 = if v16 {
                f3(v0, Str::lit("public "))?
            } else {
                v0
            };
            let v24: Adt16 = f3(v23, if v17 {
                Str::lit("meta partial def ")
            } else {
                Str::lit("meta def ")
            })?;
            let v25: Adt16 = f61(f48(v24, v3.clone(), v15)?, v3.clone(), v19)?;
            let v26: Adt16 = f3(f34(f3(f62(v25, v3.clone(), v20, true, true)?, Str::lit(" : "))?, v3, v21, 0u64, true)?, Str::lit(" :=\u{a}  "))?;
            f3(f63(v26, v1, v22, 0u64, true)?, Str::lit("\u{a}"))
        }
        Adt1::C4(v27, v28, v29) => {
            f3(f48(f3(f48(f3(f64(f3(f48(f3(v0, Str::lit("syntax (name := "))?, v3.clone(), v27.clone())?, Str::lit(") "))?, v28)?, Str::lit(": command\u{a}attribute [command_elab "))?, v3.clone(), v27)?, Str::lit("] "))?, v3, v29)?, Str::lit("\u{a}"))
        }
        Adt1::C5(v30, v31) => {
            f3({
                let (_, h115) = f65(Fn11::F66(std::rc::Rc::new(v1.clone())), (v31, v0), f67(match v1 {
                    Adt14::C0(_, _, _, _, h116) => {
                        h116
                    }
                }, v30))?;
                h115
            }, Str::lit("\u{a}"))
        }
        Adt1::C6(v32, r120) => {
            let v33 = (*r120).clone();
            f3(f63(f48(f3(v0, Str::lit("#eval "))?, v3, v32)?, v1, v33, 1u64, true)?, Str::lit("\u{a}"))
        }
    }
}

pub fn f43(v0: Fn5, p124: List<Bytes>, p125: List<Adt2>) -> List<Bytes> {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v1, v2) = m123;
                let m121 = v2;
                match m121.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), List<Bytes>>(({
                            let c122: Fn5 = v0.clone();
                            c122.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<Bytes>, v1: Adt2) -> List<Bytes> {
    List::cons(match v1 {
        Adt2::C0(h126, _) => {
            h126
        }
    }, v0)
}

pub fn f45(v0: Fn6, p130: (Option<Bytes>, List<Bytes>), p131: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v1, v2) = m129;
                let m127 = v2;
                match m127.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c128: Fn6 = v0.clone();
                            c128.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f47(v0: Adt6) -> bool {
    let m132 = v0;
    match m132 {
        Adt6::C0(_, _) => {
            false
        }
        Adt6::C1(_, _) => {
            false
        }
        Adt6::C2(_, _) => {
            false
        }
        Adt6::C3(_) => {
            false
        }
        Adt6::C4 => {
            true
        }
        Adt6::C5(_, _) => {
            false
        }
    }
}

pub fn f48(v0: Adt16, v1: List<Bytes>, v2: Bytes) -> R<Adt16> {
    let (_, h133) = f58(Fn10::F69(v1), (true, v0), f70(v2, 46u8)?)?;
    Ok(h133)
}

pub fn f49(v0: Adt16) -> R<Adt16> {
    f39(v0, Bytes::lit(&[0x20, 0xe2, 0x86, 0x92, 0x20]))
}

pub fn f50(v0: Fn7, p137: (bool, Adt16), p138: List<Adt2>) -> R<(bool, Adt16)> {
    let m136 = (p137, p138);
    {
        let mut m136 = m136;
        loop {
            let step = {
                let (v1, v2) = m136;
                let m134 = v2;
                match m134.uncons() {
                    None => {
                        Err::<((bool, Adt16), List<Adt2>), R<(bool, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt16), List<Adt2>), R<(bool, Adt16)>>(({
                            let c135: Fn7 = v0.clone();
                            c135.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: (bool, Adt16), v1: Adt2) -> R<(bool, Adt16)> {
    Ok((false, f71({
        let (h139, _) = v0;
        if h139 {
            let (_, h140) = v0;
            h140
        } else {
            f3({
                let (_, h141) = v0;
                h141
            }, Str::lit(", "))?
        }
    }, v1)?))
}

pub fn f52(v0: Fn8, p145: Adt16, p146: List<Adt4>) -> R<Adt16> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v1, v2) = m144;
                let m142 = v2;
                match m142.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt4>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt4>), R<Adt16>>(({
                            let c143: Fn8 = v0.clone();
                            c143.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<Bytes>, v1: Adt16, v2: Adt4) -> R<Adt16> {
    f32(v1.clone(), f72(f3(f33(v1), Str::lit("Constructor: "))?, v0, v2)?)
}

pub fn f54(v0: Fn9, p151: u64, p152: (u64, Adt16)) -> R<((u64, Adt16), bool)> {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v1, v2) = m150;
                let m147 = v1;
                if m147 == 0 {
                    Err::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m147 - 1;
                    let m149 = {
                        let c148: Fn9 = v0.clone();
                        c148.apply(v2.clone())?
                    };
                    match m149 {
                        None => {
                            Err::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt16)), R<((u64, Adt16), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u64, v1: bool, v2: Bytes, v3: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    f73(v2, v1, v0, v3)
}

pub fn f56(v0: Adt14, v1: Adt9, v2: u64) -> bool {
    let m153 = v1;
    match m153 {
        Adt9::C0(v3) => {
            if f74(match v0.clone() {
                Adt14::C0(_, _, h154, _, _) => {
                    h154
                }
            }, v3.clone()) {
                true
            } else {
                let m156 = f75(match v0 {
                    Adt14::C0(_, _, _, h155, _) => {
                        h155
                    }
                }, v3);
                match m156 {
                    None => {
                        false
                    }
                    Some(v4) => {
                        let a158 = {
                            let a157 = v4;
                            length_list(a157)
                        };
                        let a159 = v2;
                        nat_lt(a158, a159)
                    }
                }
            }
        }
        Adt9::C1(_) => {
            false
        }
        Adt9::C2(_, _) => {
            false
        }
        Adt9::C3(_, _) => {
            false
        }
        Adt9::C4(_, _, _, _) => {
            false
        }
        Adt9::C5(_) => {
            false
        }
        Adt9::C6(_, _) => {
            false
        }
        Adt9::C7 => {
            false
        }
        Adt9::C8(_) => {
            false
        }
        Adt9::C9 => {
            false
        }
        Adt9::C10(_, _) => {
            false
        }
    }
}

pub fn f57(v0: Adt9) -> R<u64> {
    let m160 = v0;
    match m160 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_) => {
            Ok(0u64)
        }
        Adt9::C2(_, _) => {
            Ok(0u64)
        }
        Adt9::C3(_, _) => {
            Ok(0u64)
        }
        Adt9::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt9::C5(_) => {
            Ok(0u64)
        }
        Adt9::C6(_, _) => {
            Ok(0u64)
        }
        Adt9::C7 => {
            Ok(0u64)
        }
        Adt9::C8(_) => {
            Ok(0u64)
        }
        Adt9::C9 => {
            Ok(0u64)
        }
        Adt9::C10(_, r163) => {
            let v16 = (*r163).clone();
            let a161 = f57(v16)?;
            let a162 = 1u64;
            nat_add(a161, a162)
        }
    }
}

pub fn f58(v0: Fn10, p167: (bool, Adt16), p168: List<Bytes>) -> R<(bool, Adt16)> {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v1, v2) = m166;
                let m164 = v2;
                match m164.uncons() {
                    None => {
                        Err::<((bool, Adt16), List<Bytes>), R<(bool, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt16), List<Bytes>), R<(bool, Adt16)>>(({
                            let c165: Fn10 = v0.clone();
                            c165.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Str, v1: (bool, Adt16), v2: Bytes) -> R<(bool, Adt16)> {
    Ok((false, f39({
        let (h169, _) = v1;
        if h169 {
            let (_, h170) = v1;
            h170
        } else {
            f3({
                let (_, h171) = v1;
                h171
            }, v0)?
        }
    }, v2)?))
}

pub fn f60(v0: Adt16, v1: Bytes, v2: bool, v3: bool) -> R<Adt16> {
    f3(f37(f3(v0, Str::lit("\\noindent \\texttt{"))?, v1.clone(), if v2 {
        f76(v1)?
    } else {
        let a172 = v1;
        length_bytes(a172)
    }, v3)?, Str::lit("}\\par\u{a}"))
}

pub fn f61(v0: Adt16, v1: List<Bytes>, v2: List<Bytes>) -> R<Adt16> {
    f77(Fn12::F78(v1), v0, v2)
}

pub fn f62(p177: Adt16, v1: List<Bytes>, p178: Adt5, v3: bool, p179: bool) -> R<Adt16> {
    let m176 = (p177, (p178, p179));
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v0, (v2, v4)) = m176;
                let m173 = v2;
                match m173 {
                    Adt5::C0 => {
                        Err::<(Adt16, (Adt5, bool)), R<Adt16>>(Ok(v0))
                    }
                    Adt5::C1(v5, r174, r175) => {
                        let v6 = (*r174).clone();
                        let v7 = (*r175).clone();
                        Ok::<(Adt16, (Adt5, bool)), R<Adt16>>((f3(f34(f3(f48(f3(if v4 {
                            if v3 {
                                f3(v0, Str::lit(" "))?
                            } else {
                                v0
                            }
                        } else {
                            f3(v0, Str::lit(" "))?
                        }, Str::lit("("))?, v1.clone(), v5)?, Str::lit(" : "))?, v1.clone(), v6, 0u64, true)?, Str::lit(")"))?, (v7, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt16, v1: Adt14, v2: Adt11, v3: u64, v4: bool) -> R<Adt16> {
    let m180 = v2;
    match m180 {
        Adt11::C0(v5) => {
            f48(v0, match v1 {
                Adt14::C0(h181, _, _, _, _) => {
                    h181
                }
            }, v5)
        }
        Adt11::C1(v6) => {
            f39(v0, v6)
        }
        Adt11::C2(v7) => {
            f79(v0, v7)
        }
        Adt11::C3(v8) => {
            f80(v0, v8)
        }
        Adt11::C4(v9) => {
            f48(f3(v0, Str::lit("``"))?, match v1 {
                Adt14::C0(h182, _, _, _, _) => {
                    h182
                }
            }, v9)
        }
        Adt11::C5(v10, r200, r201) => {
            let v11 = (*r200).clone();
            let v12 = (*r201).clone();
            f3(f63(f81(f48(f3(v0, Str::lit("("))?, match v1.clone() {
                Adt14::C0(h183, _, _, _, _) => {
                    h183
                }
            }, v10.clone())?, match v1.clone() {
                Adt14::C0(h184, _, _, _, _) => {
                    h184
                }
            }, f82(match v1.clone() {
                Adt14::C0(_, h185, _, _, _) => {
                    h185
                }
            }, v10), v11)?, v1, v12, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C6(v13, r202, r203) => {
            let v14 = (*r202).clone();
            let v15 = (*r203).clone();
            f3(f63(f81(f48(f3(v0, Str::lit("("))?, match v1.clone() {
                Adt14::C0(h186, _, _, _, _) => {
                    h186
                }
            }, v13.clone())?, match v1.clone() {
                Adt14::C0(h187, _, _, _, _) => {
                    h187
                }
            }, f83(match v1.clone() {
                Adt14::C0(_, _, _, h188, _) => {
                    h188
                }
            }, f84(v13)?), v14)?, v1, v15, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C7(r204, v17) => {
            let v16 = (*r204).clone();
            f48(f3(f63(f3(v0, Str::lit("("))?, v1.clone(), v16, 0u64, true)?, Str::lit(")."))?, match v1 {
                Adt14::C0(h189, _, _, _, _) => {
                    h189
                }
            }, v17)
        }
        Adt11::C8(r205, r206) => {
            let v18 = (*r205).clone();
            let v19 = (*r206).clone();
            f3(f34(f3(f63(f3(v0, Str::lit("((["))?, v1.clone(), v19, 2u64, true)?, Str::lit("] : List "))?, match v1 {
                Adt14::C0(h190, _, _, _, _) => {
                    h190
                }
            }, v18, 0u64, true)?, Str::lit("))"))
        }
        Adt11::C9(r207, r208) => {
            let v20 = (*r207).clone();
            let v21 = (*r208).clone();
            f3(f34(f3(f63(f3(v0, Str::lit("((#["))?, v1.clone(), v21, 2u64, true)?, Str::lit("] : Array "))?, match v1 {
                Adt14::C0(h191, _, _, _, _) => {
                    h191
                }
            }, v20, 0u64, true)?, Str::lit("))"))
        }
        Adt11::C10(v22, r209, r210, r211) => {
            let v23 = (*r209).clone();
            let v24 = (*r210).clone();
            let v25 = (*r211).clone();
            f3(f63(f3(f63(f3(f34(f3(f48(f3(v0, Str::lit("(let "))?, match v1.clone() {
                Adt14::C0(h192, _, _, _, _) => {
                    h192
                }
            }, v22)?, Str::lit(" : "))?, match v1.clone() {
                Adt14::C0(h193, _, _, _, _) => {
                    h193
                }
            }, v23, 0u64, true)?, Str::lit(" := "))?, v1.clone(), v24, 0u64, true)?, Str::lit("; "))?, v1, v25, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C11(r212, r213) => {
            let v26 = (*r212).clone();
            let v27 = (*r213).clone();
            f3(f63(f3(f62(f3(v0, Str::lit("(fun "))?, match v1.clone() {
                Adt14::C0(h194, _, _, _, _) => {
                    h194
                }
            }, v26, false, true)?, Str::lit(" => "))?, v1, v27, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C12(r214, r215) => {
            let v28 = (*r214).clone();
            let v29 = (*r215).clone();
            f3(f63(f63(f3(v0, Str::lit("("))?, v1.clone(), v28, 0u64, true)?, v1, v29, 1u64, true)?, Str::lit(")"))
        }
        Adt11::C13(r216, r217, r218) => {
            let v30 = (*r216).clone();
            let v31 = (*r217).clone();
            let v32 = (*r218).clone();
            f3(f63(f3(f63(f3(f63(f3(v0, Str::lit("(if "))?, v1.clone(), v30, 0u64, true)?, Str::lit(" then "))?, v1.clone(), v31, 0u64, true)?, Str::lit(" else "))?, v1, v32, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C14(r219, r220) => {
            let v33 = (*r219).clone();
            let v34 = (*r220).clone();
            f3(f63(f3(f63(f3(v0, Str::lit("(match "))?, v1.clone(), v33, 0u64, true)?, Str::lit(" with"))?, v1, v34, 0u64, true)?, Str::lit(")"))
        }
        Adt11::C15 => {
            Ok(v0)
        }
        Adt11::C16(r221, r222) => {
            let v35 = (*r221).clone();
            let v36 = (*r222).clone();
            f63(f63({
                let a195 = v3;
                let a196 = 1u64;
                if nat_eq(a195, a196) {
                    f3(v0, Str::lit(" "))?
                } else {
                    if v4 {
                        v0
                    } else {
                        f3(v0, Str::lit(", "))?
                    }
                }
            }, v1.clone(), v35, 0u64, true)?, v1, v36, v3, false)
        }
        Adt11::C17 => {
            Ok(v0)
        }
        Adt11::C18(v37, v38, r223, r224) => {
            let v39 = (*r223).clone();
            let v40 = (*r224).clone();
            f63(f63(f3({
                let m197 = v37;
                match m197 {
                    None => {
                        f3(v0, Str::lit(" | _"))?
                    }
                    Some(v41) => {
                        f85(f48(f3(f3(v0, Str::lit(" | "))?, Str::lit("."))?, match v1.clone() {
                            Adt14::C0(h198, _, _, _, _) => {
                                h198
                            }
                        }, f86(v41)?)?, match v1.clone() {
                            Adt14::C0(h199, _, _, _, _) => {
                                h199
                            }
                        }, v38)?
                    }
                }
            }, Str::lit(" => "))?, v1.clone(), v39, 0u64, true)?, v1, v40, 0u64, true)
        }
    }
}

pub fn f64(v0: Adt16, v1: List<Adt12>) -> R<Adt16> {
    f87(Fn13::F88, v0, v1)
}

pub fn f65(v0: Fn11, p228: (List<Adt13>, Adt16), p229: List<Adt12>) -> R<(List<Adt13>, Adt16)> {
    let m227 = (p228, p229);
    {
        let mut m227 = m227;
        loop {
            let step = {
                let (v1, v2) = m227;
                let m225 = v2;
                match m225.uncons() {
                    None => {
                        Err::<((List<Adt13>, Adt16), List<Adt12>), R<(List<Adt13>, Adt16)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt13>, Adt16), List<Adt12>), R<(List<Adt13>, Adt16)>>(({
                            let c226: Fn11 = v0.clone();
                            c226.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt14, v1: (List<Adt13>, Adt16), v2: Adt12) -> R<(List<Adt13>, Adt16)> {
    f89(v0, v1, v2)
}

pub fn f67(v0: List<Adt1>, v1: Bytes) -> List<Adt12> {
    let m230 = f90(v0, v1);
    match m230 {
        None => {
            List::<Adt12>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m232 = {
        let (h231, _) = v0;
        h231
    };
    match m232 {
        None => {
            (Some(v1), {
                let (_, h233) = v0;
                h233
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a234 = v2;
                let a235 = v1;
                append_bytes(a234, a235)
            }, {
                let (_, h236) = v0;
                h236
            }))
        }
    }
}

pub fn f69(v0: List<Bytes>, v1: (bool, Adt16), v2: Bytes) -> R<(bool, Adt16)> {
    Ok((false, f91({
        let (h237, _) = v1;
        if h237 {
            let (_, h238) = v1;
            h238
        } else {
            f3({
                let (_, h239) = v1;
                h239
            }, Str::lit("."))?
        }
    }, v0, v2)?))
}

pub fn f70(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a240 = v0.clone();
        length_bytes(a240)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h241, _) = f92(Fn14::F93(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h241
    };
    let v4: u64 = {
        let (h242, _) = {
            let (_, h243) = v3.clone();
            h243
        };
        h242
    };
    Ok(f10(List::cons(f94(v0, v4, {
        let a244 = v2;
        let a245 = v4;
        nat_sub(a244, a245)
    }), {
        let (_, h246) = {
            let (_, h247) = v3;
            h247
        };
        h246
    })))
}

pub fn f71(v0: Adt16, v1: Adt2) -> R<Adt16> {
    let m249 = match v1.clone() {
        Adt2::C0(_, h248) => {
            (*h248).clone()
        }
    };
    match m249 {
        Adt3::C0 => {
            f3(f39(v0, match v1 {
                Adt2::C0(h250, _) => {
                    h250
                }
            })?, Str::lit(" : Type"))
        }
        Adt3::C1 => {
            f3(f39(v0, match v1 {
                Adt2::C0(h251, _) => {
                    h251
                }
            })?, Str::lit(" : Type -> Type"))
        }
        Adt3::C2(v2) => {
            f39(f3(f39(v0, match v1 {
                Adt2::C0(h252, _) => {
                    h252
                }
            })?, Str::lit(" : "))?, v2)
        }
    }
}

pub fn f72(v0: Adt16, v1: List<Bytes>, v2: Adt4) -> R<Adt16> {
    let m254 = match v2.clone() {
        Adt4::C0(_, _, h253) => {
            h253
        }
    };
    match m254 {
        None => {
            f95(f3(f39(v0, match v2.clone() {
                Adt4::C0(h259, _, _) => {
                    h259
                }
            })?, Str::lit(" "))?, v1, match v2 {
                Adt4::C0(_, h260, _) => {
                    (*h260).clone()
                }
            }, true)
        }
        Some(v3) => {
            f3(f39(f3(f39(f3(f95(f3(f39(v0, match v2.clone() {
                Adt4::C0(h255, _, _) => {
                    h255
                }
            })?, Str::lit(" "))?, v1, match v2 {
                Adt4::C0(_, h256, _) => {
                    (*h256).clone()
                }
            }, true)?, Str::lit(" [only at "))?, match v3.clone() {
                Adt7::C0(h257, _) => {
                    h257
                }
            })?, Str::lit(" = "))?, match v3 {
                Adt7::C0(_, h258) => {
                    h258
                }
            })?, Str::lit("]"))
        }
    }
}

pub fn f73(v0: Bytes, v1: bool, v2: u64, v3: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    let a262 = {
        let (h261, _) = v3.clone();
        h261
    };
    let a263 = v2;
    if nat_lt(a262, a263) {
        let v4: (u64, u64) = f96(v0.clone(), {
            let (h264, _) = v3.clone();
            h264
        })?;
        let v5: u64 = {
            let (h265, _) = v4;
            h265
        };
        Ok(Some(({
            let a268 = {
                let (h266, _) = v3.clone();
                h266
            };
            let a269 = {
                let (_, h267) = v4;
                h267
            };
            nat_add(a268, a269)?
        }, {
            let a270 = v5;
            let a271 = 92u64;
            if nat_eq(a270, a271) {
                f3({
                    let (_, h272) = v3;
                    h272
                }, Str::lit("\\textbackslash{}"))?
            } else {
                let a273 = v5;
                let a274 = 123u64;
                if nat_eq(a273, a274) {
                    f3({
                        let (_, h275) = v3;
                        h275
                    }, Str::lit("\\{"))?
                } else {
                    let a276 = v5;
                    let a277 = 125u64;
                    if nat_eq(a276, a277) {
                        f3({
                            let (_, h278) = v3;
                            h278
                        }, Str::lit("\\}"))?
                    } else {
                        let a279 = v5;
                        let a280 = 35u64;
                        if nat_eq(a279, a280) {
                            f3({
                                let (_, h281) = v3;
                                h281
                            }, Str::lit("\\#"))?
                        } else {
                            let a282 = v5;
                            let a283 = 36u64;
                            if nat_eq(a282, a283) {
                                f3({
                                    let (_, h284) = v3;
                                    h284
                                }, Str::lit("\\$"))?
                            } else {
                                let a285 = v5;
                                let a286 = 37u64;
                                if nat_eq(a285, a286) {
                                    f39({
                                        let (_, h287) = v3;
                                        h287
                                    }, Bytes::lit(&[0x5c, 0x25]))?
                                } else {
                                    let a288 = v5;
                                    let a289 = 38u64;
                                    if nat_eq(a288, a289) {
                                        f3({
                                            let (_, h290) = v3;
                                            h290
                                        }, Str::lit("\\&"))?
                                    } else {
                                        let a291 = v5;
                                        let a292 = 95u64;
                                        if nat_eq(a291, a292) {
                                            f3({
                                                let (_, h293) = v3;
                                                h293
                                            }, Str::lit("\\_"))?
                                        } else {
                                            let a294 = v5;
                                            let a295 = 94u64;
                                            if nat_eq(a294, a295) {
                                                f3({
                                                    let (_, h296) = v3;
                                                    h296
                                                }, Str::lit("\\^{}"))?
                                            } else {
                                                let a297 = v5;
                                                let a298 = 126u64;
                                                if nat_eq(a297, a298) {
                                                    f3({
                                                        let (_, h299) = v3;
                                                        h299
                                                    }, Str::lit("\\~{}"))?
                                                } else {
                                                    let a300 = v5;
                                                    let a301 = 128u64;
                                                    if nat_lt(a300, a301) {
                                                        if if v1 {
                                                            let a302 = v5;
                                                            let a303 = 10u64;
                                                            nat_eq(a302, a303)
                                                        } else {
                                                            false
                                                        } {
                                                            f3({
                                                                let (_, h304) = v3;
                                                                h304
                                                            }, Str::lit(" "))?
                                                        } else {
                                                            f39({
                                                                let (_, h305) = v3;
                                                                h305
                                                            }, f94(v0, {
                                                                let (h306, _) = v3;
                                                                h306
                                                            }, 1u64))?
                                                        }
                                                    } else {
                                                        f3(f97(f3({
                                                            let (_, h307) = v3;
                                                            h307
                                                        }, Str::lit("<U+"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v5)?, Str::lit(">"))?
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
        })))
    } else {
        Ok(None::<(u64, Adt16)>)
    }
}

pub fn f74(v0: List<Bytes>, v1: Bytes) -> bool {
    f98(Fn15::F99(v1), false, v0)
}

pub fn f75(v0: List<Adt15>, v1: Bytes) -> Option<List<Bytes>> {
    f100(Fn16::F101(v1), None::<List<Bytes>>, v0)
}

pub fn f76(v0: Bytes) -> R<u64> {
    let (h308, _) = f102(Fn17::F103(v0.clone()), {
        let a309 = v0.clone();
        length_bytes(a309)
    }, {
        let a310 = v0;
        length_bytes(a310)
    })?;
    Ok(h308)
}

pub fn f77(v0: Fn12, p314: Adt16, p315: List<Bytes>) -> R<Adt16> {
    let m313 = (p314, p315);
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v1, v2) = m313;
                let m311 = v2;
                match m311.uncons() {
                    None => {
                        Err::<(Adt16, List<Bytes>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Bytes>), R<Adt16>>(({
                            let c312: Fn12 = v0.clone();
                            c312.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Bytes>, v1: Adt16, v2: Bytes) -> R<Adt16> {
    f3(f48(f3(v1, Str::lit(" {"))?, v0, v2)?, Str::lit(" : Type}"))
}

pub fn f79(v0: Adt16, v1: Bytes) -> R<Adt16> {
    f3({
        let (_, h316) = {
            let (h317, _) = f54(Fn9::F104(v1.clone()), {
                let a318 = v1;
                length_bytes(a318)
            }, (0u64, f3(v0, Str::lit("\""))?))?;
            h317
        };
        h316
    }, Str::lit("\""))
}

pub fn f80(v0: Adt16, v1: u64) -> R<Adt16> {
    let a319 = v1;
    let a320 = 39u64;
    if nat_eq(a319, a320) {
        f3(v0, Str::lit("'\\''"))
    } else {
        let a321 = v1;
        let a322 = 92u64;
        if nat_eq(a321, a322) {
            f3(v0, Str::lit("'\\\\'"))
        } else {
            let a323 = v1;
            let a324 = 10u64;
            if nat_eq(a323, a324) {
                f3(v0, Str::lit("'\\n'"))
            } else {
                let a325 = v1;
                let a326 = 13u64;
                if nat_eq(a325, a326) {
                    f3(v0, Str::lit("'\\r'"))
                } else {
                    let a327 = v1;
                    let a328 = 9u64;
                    if nat_eq(a327, a328) {
                        f3(v0, Str::lit("'\\t'"))
                    } else {
                        let a329 = 32u64;
                        let a330 = v1;
                        if if nat_le(a329, a330) {
                            let a331 = v1;
                            let a332 = 126u64;
                            nat_le(a331, a332)
                        } else {
                            false
                        } {
                            f3(f39(f3(v0, Str::lit("'"))?, f105(v1))?, Str::lit("'"))
                        } else {
                            f3(f97(f3(v0, Str::lit("'\\u"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), v1)?, Str::lit("'"))
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(p338: Adt16, v1: List<Bytes>, p339: List<Bytes>, p340: Adt6) -> R<Adt16> {
    let m337 = (p338, (p339, p340));
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v0, (v2, v3)) = m337;
                let m333 = v2;
                match m333.uncons() {
                    None => {
                        Err::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>(Ok(v0))
                    }
                    Some((v4, v5)) => {
                        let m334 = f106(v3);
                        match m334 {
                            None => {
                                Err::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>(Ok(v0))
                            }
                            Some(v6) => {
                                Ok::<(Adt16, (List<Bytes>, Adt6)), R<Adt16>>((f3(f34(f3(f48(f3(v0, Str::lit(" ("))?, v1.clone(), v4)?, Str::lit(" := "))?, v1.clone(), {
                                    let (h335, _) = v6;
                                    h335
                                }, 0u64, true)?, Str::lit(")"))?, (v5, {
                                    let (_, h336) = v6;
                                    h336
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt0>, v1: Bytes) -> List<Bytes> {
    f107(Fn18::F108(v1), List::<Bytes>::nil(), v0)
}

pub fn f83(v0: List<Adt15>, v1: Bytes) -> List<Bytes> {
    let m341 = f75(v0, v1);
    match m341 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f84(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f109(v0.clone())?;
    let a342 = 0u64;
    let a343 = v1;
    if nat_lt(a342, a343) {
        Ok(f94(v0, 0u64, {
            let a344 = v1;
            let a345 = 1u64;
            nat_sub(a344, a345)
        }))
    } else {
        Ok(Bytes::lit(&[]))
    }
}

pub fn f85(v0: Adt16, v1: List<Bytes>, v2: List<Bytes>) -> R<Adt16> {
    f77(Fn12::F110(v1), v0, v2)
}

pub fn f86(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f109(v0.clone())?;
    let a346 = 0u64;
    let a347 = v1;
    if nat_lt(a346, a347) {
        Ok(f94(v0.clone(), v1, {
            let a349 = {
                let a348 = v0;
                length_bytes(a348)
            };
            let a350 = v1;
            nat_sub(a349, a350)
        }))
    } else {
        Ok(v0)
    }
}

pub fn f87(v0: Fn13, p354: Adt16, p355: List<Adt12>) -> R<Adt16> {
    let m353 = (p354, p355);
    {
        let mut m353 = m353;
        loop {
            let step = {
                let (v1, v2) = m353;
                let m351 = v2;
                match m351.uncons() {
                    None => {
                        Err::<(Adt16, List<Adt12>), R<Adt16>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt16, List<Adt12>), R<Adt16>>(({
                            let c352: Fn13 = v0.clone();
                            c352.apply(v1, v3)?
                        }, v4))
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

pub fn f88(v0: Adt16, v1: Adt12) -> R<Adt16> {
    f3({
        let m356 = v1;
        match m356 {
            Adt12::C0(v2) => {
                f79(v0, v2)?
            }
            Adt12::C1 => {
                f3(v0, Str::lit("ident"))?
            }
            Adt12::C2 => {
                f3(v0, Str::lit("term"))?
            }
        }
    }, Str::lit(" "))
}

pub fn f89(v0: Adt14, v1: (List<Adt13>, Adt16), v2: Adt12) -> R<(List<Adt13>, Adt16)> {
    let m357 = v2;
    match m357 {
        Adt12::C0(v3) => {
            Ok(({
                let (h358, _) = v1;
                h358
            }, f39({
                let (_, h359) = v1;
                h359
            }, v3)?))
        }
        Adt12::C1 => {
            f111(v0, v1)
        }
        Adt12::C2 => {
            f111(v0, v1)
        }
    }
}

pub fn f90(v0: List<Adt1>, v1: Bytes) -> Option<List<Adt12>> {
    f112(Fn19::F113(v1), None::<List<Adt12>>, v0)
}

pub fn f91(v0: Adt16, v1: List<Bytes>, v2: Bytes) -> R<Adt16> {
    if f74(v1, v2.clone()) {
        f39(f39(f39(v0, Bytes::lit(&[0xc2, 0xab]))?, v2)?, Bytes::lit(&[0xc2, 0xbb]))
    } else {
        f39(v0, v2)
    }
}

pub fn f92(v0: Fn14, p364: u64, p365: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m363 = (p364, p365);
    {
        let mut m363 = m363;
        loop {
            let step = {
                let (v1, v2) = m363;
                let m360 = v1;
                if m360 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m360 - 1;
                    let m362 = {
                        let c361: Fn14 = v0.clone();
                        c361.apply(v2.clone())?
                    };
                    match m362 {
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
                Ok(next) => { m363 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f114(v1, v0, v2)
}

pub fn f94(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m369 = {
        let a366 = v0;
        let a367 = v1;
        let a368 = v2;
        slice_bytes(a366, a367, a368)
    };
    match m369 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f95(p374: Adt16, v1: List<Bytes>, p375: Adt5, p376: bool) -> R<Adt16> {
    let m373 = (p374, (p375, p376));
    {
        let mut m373 = m373;
        loop {
            let step = {
                let (v0, (v2, v3)) = m373;
                let m370 = v2;
                match m370 {
                    Adt5::C0 => {
                        Err::<(Adt16, (Adt5, bool)), R<Adt16>>(Ok(v0))
                    }
                    Adt5::C1(v4, r371, r372) => {
                        let v5 = (*r371).clone();
                        let v6 = (*r372).clone();
                        Ok::<(Adt16, (Adt5, bool)), R<Adt16>>((f3(f34(f3(f39(f3(if v3 {
                            v0
                        } else {
                            f3(v0, Str::lit(" "))?
                        }, Str::lit("("))?, v4)?, Str::lit(" : "))?, v1.clone(), v5, 0u64, true)?, Str::lit(")"))?, (v6, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m377 = f115(v0.clone(), v1);
    match m377 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a422 = {
                    let a398 = {
                        let a386 = {
                            let a380 = {
                                let a378 = v3;
                                let a379 = 128u8;
                                fixed_u8::bit_and(a378, a379)
                            };
                            let a381 = 128u8;
                            if equal(a380, a381) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a387 = {
                            let a384 = {
                                let a382 = v3;
                                let a383 = 64u8;
                                fixed_u8::bit_and(a382, a383)
                            };
                            let a385 = 64u8;
                            if equal(a384, a385) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a386, a387)?
                    };
                    let a399 = {
                        let a396 = {
                            let a390 = {
                                let a388 = v3;
                                let a389 = 32u8;
                                fixed_u8::bit_and(a388, a389)
                            };
                            let a391 = 32u8;
                            if equal(a390, a391) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a397 = {
                            let a394 = {
                                let a392 = v3;
                                let a393 = 16u8;
                                fixed_u8::bit_and(a392, a393)
                            };
                            let a395 = 16u8;
                            if equal(a394, a395) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a396, a397)?
                    };
                    nat_add(a398, a399)?
                };
                let a423 = {
                    let a420 = {
                        let a408 = {
                            let a402 = {
                                let a400 = v3;
                                let a401 = 8u8;
                                fixed_u8::bit_and(a400, a401)
                            };
                            let a403 = 8u8;
                            if equal(a402, a403) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a409 = {
                            let a406 = {
                                let a404 = v3;
                                let a405 = 4u8;
                                fixed_u8::bit_and(a404, a405)
                            };
                            let a407 = 4u8;
                            if equal(a406, a407) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a408, a409)?
                    };
                    let a421 = {
                        let a418 = {
                            let a412 = {
                                let a410 = v3;
                                let a411 = 2u8;
                                fixed_u8::bit_and(a410, a411)
                            };
                            let a413 = 2u8;
                            if equal(a412, a413) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a419 = {
                            let a416 = {
                                let a414 = v3;
                                let a415 = 1u8;
                                fixed_u8::bit_and(a414, a415)
                            };
                            let a417 = 1u8;
                            if equal(a416, a417) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a418, a419)?
                    };
                    nat_add(a420, a421)?
                };
                nat_add(a422, a423)?
            };
            let a424 = v4;
            let a425 = 128u64;
            if nat_lt(a424, a425) {
                Ok((v4, 1u64))
            } else {
                let a426 = v4;
                let a427 = 224u64;
                if nat_lt(a426, a427) {
                    Ok(({
                        let a435 = {
                            let a431 = {
                                let a428 = v4;
                                let a429 = 32u64;
                                let a430 = 0u64;
                                nat_rem(a428, a429, a430)
                            };
                            let a432 = 64u64;
                            nat_mul(a431, a432)?
                        };
                        let a436 = f116(v0, {
                            let a433 = v1;
                            let a434 = 1u64;
                            nat_add(a433, a434)?
                        })?;
                        nat_add(a435, a436)?
                    }, 2u64))
                } else {
                    let a437 = v4;
                    let a438 = 240u64;
                    if nat_lt(a437, a438) {
                        Ok(({
                            let a452 = {
                                let a442 = {
                                    let a439 = v4;
                                    let a440 = 16u64;
                                    let a441 = 0u64;
                                    nat_rem(a439, a440, a441)
                                };
                                let a443 = 4096u64;
                                nat_mul(a442, a443)?
                            };
                            let a453 = {
                                let a450 = {
                                    let a446 = f116(v0.clone(), {
                                        let a444 = v1;
                                        let a445 = 1u64;
                                        nat_add(a444, a445)?
                                    })?;
                                    let a447 = 64u64;
                                    nat_mul(a446, a447)?
                                };
                                let a451 = f116(v0, {
                                    let a448 = v1;
                                    let a449 = 2u64;
                                    nat_add(a448, a449)?
                                })?;
                                nat_add(a450, a451)?
                            };
                            nat_add(a452, a453)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a473 = {
                                let a457 = {
                                    let a454 = v4;
                                    let a455 = 8u64;
                                    let a456 = 0u64;
                                    nat_rem(a454, a455, a456)
                                };
                                let a458 = 262144u64;
                                nat_mul(a457, a458)?
                            };
                            let a474 = {
                                let a471 = {
                                    let a461 = f116(v0.clone(), {
                                        let a459 = v1;
                                        let a460 = 1u64;
                                        nat_add(a459, a460)?
                                    })?;
                                    let a462 = 4096u64;
                                    nat_mul(a461, a462)?
                                };
                                let a472 = {
                                    let a469 = {
                                        let a465 = f116(v0.clone(), {
                                            let a463 = v1;
                                            let a464 = 2u64;
                                            nat_add(a463, a464)?
                                        })?;
                                        let a466 = 64u64;
                                        nat_mul(a465, a466)?
                                    };
                                    let a470 = f116(v0, {
                                        let a467 = v1;
                                        let a468 = 3u64;
                                        nat_add(a467, a468)?
                                    })?;
                                    nat_add(a469, a470)?
                                };
                                nat_add(a471, a472)?
                            };
                            nat_add(a473, a474)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f97(v0: Adt16, v1: Bytes, v2: u64) -> R<Adt16> {
    let v3: Bytes = {
        let a487 = f117(v1.clone(), v2, 268435456u64);
        let a488 = {
            let a485 = f117(v1.clone(), v2, 16777216u64);
            let a486 = {
                let a483 = f117(v1.clone(), v2, 1048576u64);
                let a484 = {
                    let a481 = f117(v1.clone(), v2, 65536u64);
                    let a482 = {
                        let a479 = f117(v1.clone(), v2, 4096u64);
                        let a480 = {
                            let a477 = f117(v1.clone(), v2, 256u64);
                            let a478 = {
                                let a475 = f117(v1.clone(), v2, 16u64);
                                let a476 = f117(v1, v2, 1u64);
                                append_bytes(a475, a476)
                            };
                            append_bytes(a477, a478)
                        };
                        append_bytes(a479, a480)
                    };
                    append_bytes(a481, a482)
                };
                append_bytes(a483, a484)
            };
            append_bytes(a485, a486)
        };
        append_bytes(a487, a488)
    };
    let v4: u64 = {
        let (h489, _) = f102(Fn17::F118(v3.clone()), 4u64, 0u64)?;
        h489
    };
    f11(v0, f94(v3, v4, {
        let a490 = 8u64;
        let a491 = v4;
        nat_sub(a490, a491)
    }))
}

pub fn f98(v0: Fn15, p495: bool, p496: List<Bytes>) -> bool {
    let m494 = (p495, p496);
    {
        let mut m494 = m494;
        loop {
            let step = {
                let (v1, v2) = m494;
                let m492 = v2;
                match m492.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c493: Fn15 = v0.clone();
                            c493.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a497 = v2;
        let a498 = v0;
        equal(a497, a498)
    }
}

pub fn f100(v0: Fn16, p502: Option<List<Bytes>>, p503: List<Adt15>) -> Option<List<Bytes>> {
    let m501 = (p502, p503);
    {
        let mut m501 = m501;
        loop {
            let step = {
                let (v1, v2) = m501;
                let m499 = v2;
                match m499.uncons() {
                    None => {
                        Err::<(Option<List<Bytes>>, List<Adt15>), Option<List<Bytes>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Bytes>>, List<Adt15>), Option<List<Bytes>>>(({
                            let c500: Fn16 = v0.clone();
                            c500.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes, v1: Option<List<Bytes>>, v2: Adt15) -> Option<List<Bytes>> {
    let m504 = v1.clone();
    match m504 {
        None => {
            let a506 = match v2.clone() {
                Adt15::C0(h505, _) => {
                    h505
                }
            };
            let a507 = v0;
            if equal(a506, a507) {
                Some(match v2 {
                    Adt15::C0(_, h508) => {
                        h508
                    }
                })
            } else {
                v1
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f102(v0: Fn17, p513: u64, p514: u64) -> R<(u64, bool)> {
    let m512 = (p513, p514);
    {
        let mut m512 = m512;
        loop {
            let step = {
                let (v1, v2) = m512;
                let m509 = v1;
                if m509 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m509 - 1;
                    let m511 = {
                        let c510: Fn17 = v0.clone();
                        c510.apply(v2)?
                    };
                    match m511 {
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
                Ok(next) => { m512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f119(v0, v1)
}

pub fn f104(v0: Bytes, v1: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    f120(v0, v1)
}

pub fn f105(v0: u64) -> Bytes {
    f94(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f]), v0, 1u64)
}

pub fn f106(v0: Adt6) -> Option<(Adt6, Adt6)> {
    let m515 = v0;
    match m515 {
        Adt6::C0(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C2(_, _) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C4 => {
            None::<(Adt6, Adt6)>
        }
        Adt6::C5(r516, r517) => {
            let v8 = (*r516).clone();
            let v9 = (*r517).clone();
            Some((v8, v9))
        }
    }
}

pub fn f107(v0: Fn18, p521: List<Bytes>, p522: List<Adt0>) -> List<Bytes> {
    let m520 = (p521, p522);
    {
        let mut m520 = m520;
        loop {
            let step = {
                let (v1, v2) = m520;
                let m518 = v2;
                match m518.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt0>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt0>), List<Bytes>>(({
                            let c519: Fn18 = v0.clone();
                            c519.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Bytes, v1: List<Bytes>, v2: Adt0) -> List<Bytes> {
    let a524 = match v2.clone() {
        Adt0::C0(h523, _) => {
            h523
        }
    };
    let a525 = v0;
    if equal(a524, a525) {
        match v2 {
            Adt0::C0(_, h526) => {
                h526
            }
        }
    } else {
        v1
    }
}

pub fn f109(v0: Bytes) -> R<u64> {
    let (h527, _) = f102(Fn17::F121(v0.clone()), {
        let a528 = v0.clone();
        length_bytes(a528)
    }, {
        let a529 = v0;
        length_bytes(a529)
    })?;
    Ok(h527)
}

pub fn f110(v0: List<Bytes>, v1: Adt16, v2: Bytes) -> R<Adt16> {
    f48(f3(v1, Str::lit(" "))?, v0, v2)
}

pub fn f111(v0: Adt14, v1: (List<Adt13>, Adt16)) -> R<(List<Adt13>, Adt16)> {
    let m531 = {
        let (h530, _) = v1.clone();
        h530
    };
    match m531.uncons() {
        None => {
            Ok(v1)
        }
        Some((v2, v3)) => {
            Ok((v3, {
                let m532 = v2;
                match m532 {
                    Adt13::C0(v4) => {
                        f48({
                            let (_, h533) = v1;
                            h533
                        }, match v0 {
                            Adt14::C0(h534, _, _, _, _) => {
                                h534
                            }
                        }, v4)?
                    }
                    Adt13::C1(r536) => {
                        let v5 = (*r536).clone();
                        f38({
                            let (_, h535) = v1;
                            h535
                        }, v0, v5, true)?
                    }
                }
            }))
        }
    }
}

pub fn f112(v0: Fn19, p540: Option<List<Adt12>>, p541: List<Adt1>) -> Option<List<Adt12>> {
    let m539 = (p540, p541);
    {
        let mut m539 = m539;
        loop {
            let step = {
                let (v1, v2) = m539;
                let m537 = v2;
                match m537.uncons() {
                    None => {
                        Err::<(Option<List<Adt12>>, List<Adt1>), Option<List<Adt12>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt12>>, List<Adt1>), Option<List<Adt12>>>(({
                            let c538: Fn19 = v0.clone();
                            c538.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Bytes, v1: Option<List<Adt12>>, v2: Adt1) -> Option<List<Adt12>> {
    let m542 = v1.clone();
    match m542 {
        None => {
            let m543 = v2;
            match m543 {
                Adt1::C0(_, _, _, _, _, _) => {
                    v1
                }
                Adt1::C1(_, _, _, _) => {
                    v1
                }
                Adt1::C2(_) => {
                    v1
                }
                Adt1::C3(_, _, _, _, _, _, _, _) => {
                    v1
                }
                Adt1::C4(v23, v24, _) => {
                    let a544 = v23;
                    let a545 = v0;
                    if equal(a544, a545) {
                        Some(v24)
                    } else {
                        v1
                    }
                }
                Adt1::C5(_, _) => {
                    v1
                }
                Adt1::C6(_, _) => {
                    v1
                }
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f114(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h546, _) = v2.clone();
        h546
    };
    let v4: u64 = {
        let (h547, _) = {
            let (_, h548) = v2.clone();
            h548
        };
        h547
    };
    let v5: List<Bytes> = {
        let (_, h549) = {
            let (_, h550) = v2;
            h550
        };
        h549
    };
    let m551 = f115(v0.clone(), v3);
    match m551 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a552 = v6;
            let a553 = v1;
            if equal(a552, a553) {
                Ok(Some(({
                    let a554 = v3;
                    let a555 = 1u64;
                    nat_add(a554, a555)?
                }, ({
                    let a556 = v3;
                    let a557 = 1u64;
                    nat_add(a556, a557)?
                }, List::cons(f94(v0, v4, {
                    let a558 = v3;
                    let a559 = v4;
                    nat_sub(a558, a559)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a560 = v3;
                    let a561 = 1u64;
                    nat_add(a560, a561)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64) -> Option<u8> {
    let a562 = v0;
    let a563 = v1;
    index_bytes(a562, a563)
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m564 = f115(v0, v1);
    match m564 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a611 = {
                let v3: u8 = v2;
                let a609 = {
                    let a585 = {
                        let a573 = {
                            let a567 = {
                                let a565 = v3;
                                let a566 = 128u8;
                                fixed_u8::bit_and(a565, a566)
                            };
                            let a568 = 128u8;
                            if equal(a567, a568) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a574 = {
                            let a571 = {
                                let a569 = v3;
                                let a570 = 64u8;
                                fixed_u8::bit_and(a569, a570)
                            };
                            let a572 = 64u8;
                            if equal(a571, a572) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a573, a574)?
                    };
                    let a586 = {
                        let a583 = {
                            let a577 = {
                                let a575 = v3;
                                let a576 = 32u8;
                                fixed_u8::bit_and(a575, a576)
                            };
                            let a578 = 32u8;
                            if equal(a577, a578) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a584 = {
                            let a581 = {
                                let a579 = v3;
                                let a580 = 16u8;
                                fixed_u8::bit_and(a579, a580)
                            };
                            let a582 = 16u8;
                            if equal(a581, a582) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a583, a584)?
                    };
                    nat_add(a585, a586)?
                };
                let a610 = {
                    let a607 = {
                        let a595 = {
                            let a589 = {
                                let a587 = v3;
                                let a588 = 8u8;
                                fixed_u8::bit_and(a587, a588)
                            };
                            let a590 = 8u8;
                            if equal(a589, a590) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a596 = {
                            let a593 = {
                                let a591 = v3;
                                let a592 = 4u8;
                                fixed_u8::bit_and(a591, a592)
                            };
                            let a594 = 4u8;
                            if equal(a593, a594) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a595, a596)?
                    };
                    let a608 = {
                        let a605 = {
                            let a599 = {
                                let a597 = v3;
                                let a598 = 2u8;
                                fixed_u8::bit_and(a597, a598)
                            };
                            let a600 = 2u8;
                            if equal(a599, a600) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a606 = {
                            let a603 = {
                                let a601 = v3;
                                let a602 = 1u8;
                                fixed_u8::bit_and(a601, a602)
                            };
                            let a604 = 1u8;
                            if equal(a603, a604) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a605, a606)?
                    };
                    nat_add(a607, a608)?
                };
                nat_add(a609, a610)?
            };
            let a612 = 64u64;
            let a613 = 0u64;
            Ok(nat_rem(a611, a612, a613))
        }
    }
}

pub fn f117(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f94(v0, {
        let a617 = {
            let a614 = v1;
            let a615 = v2;
            let a616 = 0u64;
            nat_quot(a614, a615, a616)
        };
        let a618 = 16u64;
        let a619 = 0u64;
        nat_rem(a617, a618, a619)
    }, 1u64)
}

pub fn f118(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f122(v0, v1)
}

pub fn f119(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a620 = 0u64;
    let a621 = v1;
    if nat_lt(a620, a621) {
        let v2: u64 = f123(v0, v1)?;
        let a622 = 0u64;
        let a623 = v2;
        if nat_lt(a622, a623) {
            Ok(Some({
                let a624 = v1;
                let a625 = v2;
                nat_sub(a624, a625)
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f120(v0: Bytes, v1: (u64, Adt16)) -> R<Option<(u64, Adt16)>> {
    let a628 = {
        let (h626, _) = v1.clone();
        h626
    };
    let a629 = {
        let a627 = v0.clone();
        length_bytes(a627)
    };
    if nat_lt(a628, a629) {
        let v2: (u64, u64) = f96(v0.clone(), {
            let (h630, _) = v1.clone();
            h630
        })?;
        let v3: u64 = {
            let (h631, _) = v2;
            h631
        };
        Ok(Some(({
            let a634 = {
                let (h632, _) = v1.clone();
                h632
            };
            let a635 = {
                let (_, h633) = v2;
                h633
            };
            nat_add(a634, a635)?
        }, {
            let a636 = v3;
            let a637 = 92u64;
            if nat_eq(a636, a637) {
                f3({
                    let (_, h638) = v1;
                    h638
                }, Str::lit("\\\\"))?
            } else {
                let a639 = v3;
                let a640 = 34u64;
                if nat_eq(a639, a640) {
                    f3({
                        let (_, h641) = v1;
                        h641
                    }, Str::lit("\\\""))?
                } else {
                    let a642 = v3;
                    let a643 = 10u64;
                    if nat_eq(a642, a643) {
                        f3({
                            let (_, h644) = v1;
                            h644
                        }, Str::lit("\\n"))?
                    } else {
                        let a645 = v3;
                        let a646 = 13u64;
                        if nat_eq(a645, a646) {
                            f3({
                                let (_, h647) = v1;
                                h647
                            }, Str::lit("\\r"))?
                        } else {
                            let a648 = v3;
                            let a649 = 9u64;
                            if nat_eq(a648, a649) {
                                f3({
                                    let (_, h650) = v1;
                                    h650
                                }, Str::lit("\\t"))?
                            } else {
                                let a651 = v3;
                                let a652 = 32u64;
                                if if nat_lt(a651, a652) {
                                    true
                                } else {
                                    let a653 = 127u64;
                                    let a654 = v3;
                                    if nat_le(a653, a654) {
                                        let a655 = v3;
                                        let a656 = 159u64;
                                        nat_le(a655, a656)
                                    } else {
                                        false
                                    }
                                } {
                                    f97(f3({
                                        let (_, h657) = v1;
                                        h657
                                    }, Str::lit("\\u"))?, Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), v3)?
                                } else {
                                    f39({
                                        let (_, h658) = v1;
                                        h658
                                    }, f94(v0, {
                                        let (h659, _) = v1;
                                        h659
                                    }, {
                                        let (_, h660) = v2;
                                        h660
                                    }))?
                                }
                            }
                        }
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt16)>)
    }
}

pub fn f121(v0: Bytes, v1: u64) -> Option<u64> {
    f124(v0, v1)
}

pub fn f122(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m661 = f115(v0, v1);
    match m661 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a662 = v2;
            let a663 = 48u8;
            if equal(a662, a663) {
                Ok(Some({
                    let a664 = v1;
                    let a665 = 1u64;
                    nat_add(a664, a665)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f125(v0.clone(), {
        let a666 = v1;
        let a667 = 1u64;
        nat_sub(a666, a667)
    })?;
    let v3: u64 = f125(v0.clone(), {
        let a668 = v1;
        let a669 = 2u64;
        nat_sub(a668, a669)
    })?;
    let v4: u64 = f125(v0, {
        let a670 = v1;
        let a671 = 3u64;
        nat_sub(a670, a671)
    })?;
    let a672 = v2;
    let a673 = 32u64;
    if if nat_eq(a672, a673) {
        true
    } else {
        let a674 = 9u64;
        let a675 = v2;
        if nat_le(a674, a675) {
            let a676 = v2;
            let a677 = 13u64;
            nat_le(a676, a677)
        } else {
            false
        }
    } {
        Ok(1u64)
    } else {
        let a678 = 2u64;
        let a679 = v1;
        if if nat_le(a678, a679) {
            let a680 = v3;
            let a681 = 194u64;
            if nat_eq(a680, a681) {
                let a682 = v2;
                let a683 = 133u64;
                if nat_eq(a682, a683) {
                    true
                } else {
                    let a684 = v2;
                    let a685 = 160u64;
                    nat_eq(a684, a685)
                }
            } else {
                false
            }
        } else {
            false
        } {
            Ok(2u64)
        } else {
            let a686 = 3u64;
            let a687 = v1;
            if if nat_le(a686, a687) {
                let a688 = v4;
                let a689 = 225u64;
                if if nat_eq(a688, a689) {
                    let a690 = v3;
                    let a691 = 154u64;
                    if nat_eq(a690, a691) {
                        let a692 = v2;
                        let a693 = 128u64;
                        nat_eq(a692, a693)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    true
                } else {
                    let a694 = v4;
                    let a695 = 226u64;
                    if if nat_eq(a694, a695) {
                        let a696 = v3;
                        let a697 = 128u64;
                        if nat_eq(a696, a697) {
                            let a698 = 128u64;
                            let a699 = v2;
                            if if nat_le(a698, a699) {
                                let a700 = v2;
                                let a701 = 138u64;
                                nat_le(a700, a701)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a702 = v2;
                                let a703 = 168u64;
                                if nat_eq(a702, a703) {
                                    true
                                } else {
                                    let a704 = v2;
                                    let a705 = 169u64;
                                    if nat_eq(a704, a705) {
                                        true
                                    } else {
                                        let a706 = v2;
                                        let a707 = 175u64;
                                        nat_eq(a706, a707)
                                    }
                                }
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a708 = v4;
                        let a709 = 226u64;
                        if if nat_eq(a708, a709) {
                            let a710 = v3;
                            let a711 = 129u64;
                            if nat_eq(a710, a711) {
                                let a712 = v2;
                                let a713 = 159u64;
                                nat_eq(a712, a713)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a714 = v4;
                            let a715 = 227u64;
                            if nat_eq(a714, a715) {
                                let a716 = v3;
                                let a717 = 128u64;
                                if nat_eq(a716, a717) {
                                    let a718 = v2;
                                    let a719 = 128u64;
                                    nat_eq(a718, a719)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    }
                }
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

pub fn f124(v0: Bytes, v1: u64) -> Option<u64> {
    let a720 = 0u64;
    let a721 = v1;
    if nat_lt(a720, a721) {
        let m724 = f115(v0, {
            let a722 = v1;
            let a723 = 1u64;
            nat_sub(a722, a723)
        });
        match m724 {
            None => {
                None::<u64>
            }
            Some(v2) => {
                let a725 = v2;
                let a726 = 46u8;
                if equal(a725, a726) {
                    None::<u64>
                } else {
                    Some({
                        let a727 = v1;
                        let a728 = 1u64;
                        nat_sub(a727, a728)
                    })
                }
            }
        }
    } else {
        None::<u64>
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<u64> {
    let m729 = f115(v0, v1);
    match m729 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a774 = {
                let a750 = {
                    let a738 = {
                        let a732 = {
                            let a730 = v3;
                            let a731 = 128u8;
                            fixed_u8::bit_and(a730, a731)
                        };
                        let a733 = 128u8;
                        if equal(a732, a733) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a739 = {
                        let a736 = {
                            let a734 = v3;
                            let a735 = 64u8;
                            fixed_u8::bit_and(a734, a735)
                        };
                        let a737 = 64u8;
                        if equal(a736, a737) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a738, a739)?
                };
                let a751 = {
                    let a748 = {
                        let a742 = {
                            let a740 = v3;
                            let a741 = 32u8;
                            fixed_u8::bit_and(a740, a741)
                        };
                        let a743 = 32u8;
                        if equal(a742, a743) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a749 = {
                        let a746 = {
                            let a744 = v3;
                            let a745 = 16u8;
                            fixed_u8::bit_and(a744, a745)
                        };
                        let a747 = 16u8;
                        if equal(a746, a747) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a748, a749)?
                };
                nat_add(a750, a751)?
            };
            let a775 = {
                let a772 = {
                    let a760 = {
                        let a754 = {
                            let a752 = v3;
                            let a753 = 8u8;
                            fixed_u8::bit_and(a752, a753)
                        };
                        let a755 = 8u8;
                        if equal(a754, a755) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a761 = {
                        let a758 = {
                            let a756 = v3;
                            let a757 = 4u8;
                            fixed_u8::bit_and(a756, a757)
                        };
                        let a759 = 4u8;
                        if equal(a758, a759) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a760, a761)?
                };
                let a773 = {
                    let a770 = {
                        let a764 = {
                            let a762 = v3;
                            let a763 = 2u8;
                            fixed_u8::bit_and(a762, a763)
                        };
                        let a765 = 2u8;
                        if equal(a764, a765) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a771 = {
                        let a768 = {
                            let a766 = v3;
                            let a767 = 1u8;
                            fixed_u8::bit_and(a766, a767)
                        };
                        let a769 = 1u8;
                        if equal(a768, a769) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a770, a771)?
                };
                nat_add(a772, a773)?
            };
            nat_add(a774, a775)
        }
    }
}

pub fn run(p0: u64, p1: List<Bytes>, p2: List<Adt0>, p3: List<Adt1>) -> R<Result<Bytes, Bytes>> {
    f0(p0, p1, p2, p3)
}
