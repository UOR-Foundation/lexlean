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
    C0(Bytes),
    C1(std::rc::Rc<Adt0>),
    C2(List<Adt5>),
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F65,
}

impl Fn0 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn0::F65 => f65(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F80(Bytes),
    F93(Bytes),
    F101(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F80(k0) => f80(k0.clone(), p0),
            Fn1::F93(k0) => f93(k0.clone(), p0),
            Fn1::F101(k0) => f101(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F87(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F87(k0) => f87(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2) -> R<Adt0> {
    f1(Adt7::C0(v0, std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5.clone())), v5, List::<(u64, Adt0)>::nil(), Adt0::C14, 1u64)
}

pub fn f1(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: Adt0, v4: u64) -> R<Adt0> {
    let m1 = v1;
    match m1 {
        Adt2::C0(_, _) => {
            Ok(Adt0::C14)
        }
        Adt2::C1(_) => {
            Ok(Adt0::C14)
        }
        Adt2::C2(v8, r5, r6, r7) => {
            let v9 = (*r5).clone();
            let v10 = (*r6).clone();
            let v11 = (*r7).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v10, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v11, f3(v8, v9, v2), Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C3(r8, r9, r10) => {
            let v12 = (*r8).clone();
            let v13 = (*r9).clone();
            let v14 = (*r10).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v12, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0.clone(), v13, v2.clone(), Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f1(v0, v14, v2, Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))))
        }
        Adt2::C4(_, r11, r12) => {
            let v16 = (*r11).clone();
            let v17 = (*r12).clone();
            let v18: Adt0 = f4(v0.clone(), v16.clone(), v2.clone())?;
            Ok(f2(Adt0::C14, f1(v0.clone(), v16, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v17, v2, v18, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C5(_, _, r13) => {
            let v21 = (*r13).clone();
            Ok(f5(f1(v0, v21, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C6(_, r14) => {
            let v23 = (*r14).clone();
            Ok(f5(f1(v0, v23, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C7(_, r15) => {
            let v25 = (*r15).clone();
            Ok(f5(f1(v0, v25, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C8(r16, r17) => {
            let v26 = (*r16).clone();
            let v27 = (*r17).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v26, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v27, v2, Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C9(_, r18) => {
            let v29 = (*r18).clone();
            Ok(f5(f1(v0, v29, v2, Adt0::C0, 0u64)?))
        }
        Adt2::C10(r19) => {
            let v30 = (*r19).clone();
            Ok(f5(f1(v0, v30, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C11(r20) => {
            let v31 = (*r20).clone();
            Ok(f5(f1(v0, v31, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C12(r21, _) => {
            let v32 = (*r21).clone();
            Ok(f5(f6(f4(v0.clone(), v32.clone(), v2.clone())?, f1(v0, v32, v2, Adt0::C14, 0u64)?)))
        }
        Adt2::C13 => {
            Ok(Adt0::C14)
        }
        Adt2::C14(r22, r23) => {
            let v34 = (*r22).clone();
            let v35 = (*r23).clone();
            let m2 = v4;
            if m2 == 0 {
                let v36: Adt0 = f1(v0.clone(), v34.clone(), v2.clone(), v3.clone(), 0u64)?;
                Ok(f2(Adt0::C14, if f7(v3.clone()) {
                    v36
                } else {
                    f6(f4(v0.clone(), v34, v2.clone())?, v36)
                }, Adt0::C15(std::rc::Rc::new(f1(v0, v35, v2, v3, 0u64)?), std::rc::Rc::new(Adt0::C14))))
            } else {
                let v37 = m2 - 1;
                Ok(f2(Adt0::C14, f1(v0.clone(), v34, f8(f9(v0.clone(), v37), f10(v0.clone(), v37), List::<(u64, Adt0)>::nil()), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v35, v2, v3, {
                    let a3 = v4;
                    let a4 = 1u64;
                    nat_add(a3, a4)?
                })?), std::rc::Rc::new(Adt0::C14))))
            }
        }
        Adt2::C15(r24, v39, r25) => {
            let v38 = (*r24).clone();
            let v40 = (*r25).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v40, f8(v39, f11(v0, v38, v3)?, v2), Adt0::C14, 0u64)?, Adt0::C14))
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(v0), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(v2))))
}

pub fn f3(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f4(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>) -> R<Adt0> {
    let m26 = f12(v0, v1, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
    match m26 {
        Adt8::C0(_) => {
            Ok(Adt0::C14)
        }
        Adt8::C1(r27) => {
            let v3 = (*r27).clone();
            Ok(v3)
        }
        Adt8::C2(_) => {
            Ok(Adt0::C14)
        }
        Adt8::C3 => {
            Ok(Adt0::C14)
        }
    }
}

pub fn f5(v0: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(Adt0::C14))))
}

pub fn f6(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(v0), std::rc::Rc::new(if f7(v1.clone()) {
        Adt0::C14
    } else {
        f14(v1)
    }))
}

pub fn f7(v0: Adt0) -> bool {
    let a28 = f15(v0);
    let a29 = 14u64;
    nat_eq(a28, a29)
}

pub fn f8(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m30 = v0;
    match m30.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f8(v4, f14(v1.clone()), f3(v3, f16(v1.clone()), v2.clone()));
            if f7(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f9(v0: Adt7, v1: u64) -> List<u64> {
    let m32 = f17(match v0 {
        Adt7::C0(_, _, h31, _, _, _) => {
            h31
        }
    }, v1);
    match m32 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f10(v0: Adt7, v1: u64) -> Adt0 {
    let m33 = f18(v0, v1);
    match m33 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f11(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Adt0> {
    let m34 = f19(v0, v1, v2)?;
    match m34 {
        Ok(v3) => {
            Ok(v3)
        }
        Err(_) => {
            Ok(Adt0::C14)
        }
    }
}

pub fn f12(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: u64, v4: Adt0, v5: Adt0, v6: Bytes, v7: u64, v8: List<Adt5>) -> R<Adt8> {
    let m35 = v1;
    match m35 {
        Adt2::C0(r163, r164) => {
            let v9 = (*r163).clone();
            let v10 = (*r164).clone();
            let a36 = v3;
            let a37 = 0u64;
            if nat_eq(a36, a37) {
                let m38 = f20(v0.clone(), v9.clone())?;
                match m38 {
                    Ok(_) => {
                        let m39 = f21(v0, v10, v9.clone(), 0u64)?;
                        match m39 {
                            Ok(_) => {
                                Ok(Adt8::C1(std::rc::Rc::new(v9)))
                            }
                            Err(v13) => {
                                Ok(Adt8::C0(v13))
                            }
                        }
                    }
                    Err(v14) => {
                        Ok(Adt8::C0(v14))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C1(v15) => {
            let a40 = v3;
            let a41 = 0u64;
            if nat_eq(a40, a41) {
                let m42 = f22(v2, v15);
                match m42 {
                    None => {
                        Ok(Adt8::C0(f23(f13(Str::lit("local ")), f23(f24(v15)?, f13(Str::lit(" is unbound"))))))
                    }
                    Some(v16) => {
                        Ok(Adt8::C1(std::rc::Rc::new(v16)))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C2(v17, r165, r166, r167) => {
            let v18 = (*r165).clone();
            let v19 = (*r166).clone();
            let v20 = (*r167).clone();
            let a43 = v3;
            let a44 = 0u64;
            if nat_eq(a43, a44) {
                let m45 = f20(v0.clone(), v18.clone())?;
                match m45 {
                    Ok(_) => {
                        let m46 = f12(v0.clone(), v19, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m46 {
                            Adt8::C0(v25) => {
                                Ok(Adt8::C0(v25))
                            }
                            Adt8::C1(r48) => {
                                let v22 = (*r48).clone();
                                let m47 = f25(v22, v18.clone(), f23(f13(Str::lit("the value bound to ")), f24(v17)?))?;
                                match m47 {
                                    Ok(_) => {
                                        f12(v0, v20, f3(v17, v18, v2), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())
                                    }
                                    Err(v24) => {
                                        Ok(Adt8::C0(v24))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v27) => {
                        Ok(Adt8::C0(v27))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C3(r168, r169, r170) => {
            let v28 = (*r168).clone();
            let v29 = (*r169).clone();
            let v30 = (*r170).clone();
            let a49 = v3;
            let a50 = 0u64;
            if nat_eq(a49, a50) {
                let m51 = f12(v0.clone(), v28, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m51 {
                    Adt8::C0(v42) => {
                        Ok(Adt8::C0(v42))
                    }
                    Adt8::C1(r58) => {
                        let v31 = (*r58).clone();
                        let m52 = f25(v31, Adt0::C1, f13(Str::lit("a condition")))?;
                        match m52 {
                            Ok(_) => {
                                let m53 = f12(v0.clone(), v29, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                match m53 {
                                    Adt8::C0(v39) => {
                                        Ok(Adt8::C0(v39))
                                    }
                                    Adt8::C1(r57) => {
                                        let v33 = (*r57).clone();
                                        let m54 = f12(v0, v30, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m54 {
                                            Adt8::C0(v37) => {
                                                Ok(Adt8::C0(v37))
                                            }
                                            Adt8::C1(r56) => {
                                                let v34 = (*r56).clone();
                                                let m55 = f25(v34, v33.clone(), f13(Str::lit("the else branch")))?;
                                                match m55 {
                                                    Ok(_) => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v33)))
                                                    }
                                                    Err(v36) => {
                                                        Ok(Adt8::C0(v36))
                                                    }
                                                }
                                            }
                                            Adt8::C2(_) => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                }
                            }
                            Err(v41) => {
                                Ok(Adt8::C0(v41))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C4(r171, r172, r173) => {
            let v44 = (*r171).clone();
            let v45 = (*r172).clone();
            let v46 = (*r173).clone();
            let a59 = v3;
            let a60 = 0u64;
            if nat_eq(a59, a60) {
                let m61 = f20(v0.clone(), v44.clone())?;
                match m61 {
                    Ok(_) => {
                        let m62 = f12(v0.clone(), v45, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m62 {
                            Adt8::C0(v55) => {
                                Ok(Adt8::C0(v55))
                            }
                            Adt8::C1(r66) => {
                                let v48 = (*r66).clone();
                                let m63 = f26(v0.clone(), v48.clone())?;
                                match m63 {
                                    Ok(v49) => {
                                        let m64 = f12(v0, v46, v2, 4u64, v44.clone(), v48, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m64 {
                                            Adt8::C0(v52) => {
                                                Ok(Adt8::C0(v52))
                                            }
                                            Adt8::C1(_) => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Adt8::C2(v50) => {
                                                let m65 = f27(v49, v50);
                                                match m65 {
                                                    None => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v44)))
                                                    }
                                                    Some(v51) => {
                                                        Ok(Adt8::C0(f23(f13(Str::lit("the match is not exhaustive: ")), f23(f28(v51)?, f13(Str::lit(" is not covered"))))))
                                                    }
                                                }
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Err(v54) => {
                                        Ok(Adt8::C0(v54))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v57) => {
                        Ok(Adt8::C0(v57))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C5(r174, r175, r176) => {
            let v58 = (*r174).clone();
            let v59 = (*r175).clone();
            let v60 = (*r176).clone();
            let a67 = v3;
            let a68 = 0u64;
            if nat_eq(a67, a68) {
                let m69 = f20(v0.clone(), v59.clone())?;
                match m69 {
                    Ok(_) => {
                        let m70 = f19(v0.clone(), v58.clone(), v59.clone())?;
                        match m70 {
                            Ok(v62) => {
                                let v63: Bytes = f23(f13(Str::lit("constructor ")), f28(v58)?);
                                let a71 = f29(v60.clone())?;
                                let a72 = f30(v62.clone())?;
                                if nat_eq(a71, a72) {
                                    let m73 = f12(v0, v60, v2, 2u64, v62, Adt0::C14, v63, 0u64, List::<Adt5>::nil())?;
                                    match m73 {
                                        Adt8::C0(v64) => {
                                            Ok(Adt8::C0(v64))
                                        }
                                        Adt8::C1(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C1(std::rc::Rc::new(v59)))
                                        }
                                    }
                                } else {
                                    Ok(Adt8::C0(f23(v63, f23(f13(Str::lit(" takes ")), f23(f24(f30(v62)?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v60)?)?))))))
                                }
                            }
                            Err(v67) => {
                                Ok(Adt8::C0(v67))
                            }
                        }
                    }
                    Err(v68) => {
                        Ok(Adt8::C0(v68))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C6(v69, r177) => {
            let v70 = (*r177).clone();
            let a74 = v3;
            let a75 = 0u64;
            if nat_eq(a74, a75) {
                let m76 = f18(v0.clone(), v69);
                match m76 {
                    None => {
                        Ok(Adt8::C0(f31(v69)?))
                    }
                    Some(v71) => {
                        let v72: Bytes = f23(f13(Str::lit("function ")), f24(v69)?);
                        let a77 = f29(v70.clone())?;
                        let a78 = f30(v71.clone())?;
                        if nat_eq(a77, a78) {
                            let m79 = f12(v0.clone(), v70, v2, 2u64, v71, Adt0::C14, v72, 0u64, List::<Adt5>::nil())?;
                            match m79 {
                                Adt8::C0(v73) => {
                                    Ok(Adt8::C0(v73))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C1(std::rc::Rc::new(f32(v0, v69))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f23(v72, f23(f13(Str::lit(" takes ")), f23(f24(f30(v71)?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v70)?)?))))))
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C7(v76, r178) => {
            let v77 = (*r178).clone();
            let a80 = v3;
            let a81 = 0u64;
            if nat_eq(a80, a81) {
                let m82 = f18(v0.clone(), v76);
                match m82 {
                    None => {
                        Ok(Adt8::C0(f31(v76)?))
                    }
                    Some(v78) => {
                        let v79: u64 = f30(v78.clone())?;
                        let v80: u64 = f29(v77.clone())?;
                        let a83 = v79;
                        let a84 = v80;
                        if nat_le(a83, a84) {
                            Ok(Adt8::C0(f23(f13(Str::lit("closure of function ")), f23(f24(v76)?, f23(f13(Str::lit(" captures ")), f23(f24(v80)?, f23(f13(Str::lit(" of its ")), f23(f24(v79)?, f13(Str::lit(" parameters; at least one must remain"))))))))))
                        } else {
                            let m85 = f12(v0.clone(), v77, v2, 3u64, v78, Adt0::C14, f23(f13(Str::lit("closure of function ")), f24(v76)?), 0u64, List::<Adt5>::nil())?;
                            match m85 {
                                Adt8::C0(v82) => {
                                    Ok(Adt8::C0(v82))
                                }
                                Adt8::C1(r86) => {
                                    let v81 = (*r86).clone();
                                    Ok(Adt8::C1(std::rc::Rc::new(f33(v81, f32(v0, v76)))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C8(r179, r180) => {
            let v84 = (*r179).clone();
            let v85 = (*r180).clone();
            let a87 = v3;
            let a88 = 0u64;
            if nat_eq(a87, a88) {
                let m89 = f12(v0.clone(), v84, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m89 {
                    Adt8::C0(v91) => {
                        Ok(Adt8::C0(v91))
                    }
                    Adt8::C1(r95) => {
                        let v86 = (*r95).clone();
                        let a90 = f15(v86.clone());
                        let a91 = 13u64;
                        if nat_eq(a90, a91) {
                            let v87: Bytes = f13(Str::lit("an application"));
                            let a92 = f29(v85.clone())?;
                            let a93 = f30(f16(v86.clone()))?;
                            if nat_eq(a92, a93) {
                                let m94 = f12(v0, v85, v2, 2u64, f16(v86.clone()), Adt0::C14, v87, 0u64, List::<Adt5>::nil())?;
                                match m94 {
                                    Adt8::C0(v88) => {
                                        Ok(Adt8::C0(v88))
                                    }
                                    Adt8::C1(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C1(std::rc::Rc::new(f14(v86))))
                                    }
                                }
                            } else {
                                Ok(Adt8::C0(f23(v87, f23(f13(Str::lit(" takes ")), f23(f24(f30(f16(v86))?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v85)?)?))))))
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("only a function value can be applied"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C9(r181, r182) => {
            let v93 = (*r181).clone();
            let v94 = (*r182).clone();
            let a96 = v3;
            let a97 = 0u64;
            if nat_eq(a96, a97) {
                let m98 = f12(v0, v94, v2, 1u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m98 {
                    Adt8::C0(v98) => {
                        Ok(Adt8::C0(v98))
                    }
                    Adt8::C1(r100) => {
                        let v95 = (*r100).clone();
                        let m99 = f34(v93, v95)?;
                        match m99 {
                            Ok(v96) => {
                                Ok(Adt8::C1(std::rc::Rc::new(v96)))
                            }
                            Err(v97) => {
                                Ok(Adt8::C0(v97))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C10(r183) => {
            let v100 = (*r183).clone();
            let a101 = v3;
            let a102 = 0u64;
            if nat_eq(a101, a102) {
                let m103 = f12(v0, v100, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m103 {
                    Adt8::C0(v102) => {
                        Ok(Adt8::C0(v102))
                    }
                    Adt8::C1(r106) => {
                        let v101 = (*r106).clone();
                        let a104 = f15(v101.clone());
                        let a105 = 11u64;
                        if nat_eq(a104, a105) {
                            Ok(Adt8::C1(std::rc::Rc::new(f16(v101))))
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("`first` of ")), f35(v101)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C11(r184) => {
            let v104 = (*r184).clone();
            let a107 = v3;
            let a108 = 0u64;
            if nat_eq(a107, a108) {
                let m109 = f12(v0, v104, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m109 {
                    Adt8::C0(v106) => {
                        Ok(Adt8::C0(v106))
                    }
                    Adt8::C1(r112) => {
                        let v105 = (*r112).clone();
                        let a110 = f15(v105.clone());
                        let a111 = 11u64;
                        if nat_eq(a110, a111) {
                            Ok(Adt8::C1(std::rc::Rc::new(f14(v105))))
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("`second` of ")), f35(v105)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C12(r185, v109) => {
            let v108 = (*r185).clone();
            let a113 = v3;
            let a114 = 0u64;
            if nat_eq(a113, a114) {
                let m115 = f12(v0.clone(), v108, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m115 {
                    Adt8::C0(v115) => {
                        Ok(Adt8::C0(v115))
                    }
                    Adt8::C1(r123) => {
                        let v110 = (*r123).clone();
                        let a116 = f15(v110.clone());
                        let a117 = 12u64;
                        if nat_eq(a116, a117) {
                            let v111: u64 = f36(v110);
                            let m118 = f37(v0, v111);
                            match m118 {
                                None => {
                                    Ok(Adt8::C0(f38(v111)?))
                                }
                                Some(v112) => {
                                    let a119 = f30(v112.clone())?;
                                    let a120 = 1u64;
                                    if nat_eq(a119, a120) {
                                        let m121 = f39(v112, 0u64);
                                        match m121 {
                                            None => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Some(v113) => {
                                                let m122 = f39(v113, v109);
                                                match m122 {
                                                    None => {
                                                        Ok(Adt8::C0(f23(f13(Str::lit("ADT ")), f23(f24(v111)?, f23(f13(Str::lit(" has no field ")), f24(v109)?)))))
                                                    }
                                                    Some(v114) => {
                                                        Ok(Adt8::C1(std::rc::Rc::new(v114)))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        Ok(Adt8::C0(f23(f13(Str::lit("`field` requires an ADT with exactly one constructor; ADT ")), f23(f24(v111)?, f23(f13(Str::lit(" has ")), f24(f30(v112)?)?)))))
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("`field` requires an ADT value"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C13 => {
            let a124 = v3;
            let a125 = 1u64;
            if nat_eq(a124, a125) {
                Ok(Adt8::C1(std::rc::Rc::new(Adt0::C14)))
            } else {
                let a126 = v3;
                let a127 = 2u64;
                if nat_eq(a126, a127) {
                    Ok(Adt8::C3)
                } else {
                    let a128 = v3;
                    let a129 = 3u64;
                    if nat_eq(a128, a129) {
                        Ok(Adt8::C1(std::rc::Rc::new(v4)))
                    } else {
                        let a130 = v3;
                        let a131 = 4u64;
                        if nat_eq(a130, a131) {
                            Ok(Adt8::C2(v8))
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C14(r186, r187) => {
            let v117 = (*r186).clone();
            let v118 = (*r187).clone();
            let a132 = v3;
            let a133 = 1u64;
            if nat_eq(a132, a133) {
                let m134 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m134 {
                    Adt8::C0(v123) => {
                        Ok(Adt8::C0(v123))
                    }
                    Adt8::C1(r137) => {
                        let v119 = (*r137).clone();
                        let m135 = f12(v0, v118, v2, 1u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m135 {
                            Adt8::C0(v121) => {
                                Ok(Adt8::C0(v121))
                            }
                            Adt8::C1(r136) => {
                                let v120 = (*r136).clone();
                                Ok(Adt8::C1(std::rc::Rc::new(f40(v119, v120))))
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                let a138 = v3;
                let a139 = 2u64;
                if nat_eq(a138, a139) {
                    if f7(v4.clone()) {
                        Ok(Adt8::C3)
                    } else {
                        let m140 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m140 {
                            Adt8::C0(v128) => {
                                Ok(Adt8::C0(v128))
                            }
                            Adt8::C1(r144) => {
                                let v125 = (*r144).clone();
                                let m141 = f25(v125, f16(v4.clone()), f23(v6.clone(), f23(f13(Str::lit(" operand ")), f24(v7)?)))?;
                                match m141 {
                                    Ok(_) => {
                                        f12(v0, v118, v2, 2u64, f14(v4), Adt0::C14, v6, {
                                            let a142 = v7;
                                            let a143 = 1u64;
                                            nat_add(a142, a143)?
                                        }, List::<Adt5>::nil())
                                    }
                                    Err(v127) => {
                                        Ok(Adt8::C0(v127))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                } else {
                    let a145 = v3;
                    let a146 = 3u64;
                    if nat_eq(a145, a146) {
                        if f7(v4.clone()) {
                            Ok(Adt8::C1(std::rc::Rc::new(Adt0::C14)))
                        } else {
                            let m147 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m147 {
                                Adt8::C0(v133) => {
                                    Ok(Adt8::C0(v133))
                                }
                                Adt8::C1(r151) => {
                                    let v130 = (*r151).clone();
                                    let m148 = f25(v130, f16(v4.clone()), f23(v6.clone(), f23(f13(Str::lit(" operand ")), f24(v7)?)))?;
                                    match m148 {
                                        Ok(_) => {
                                            f12(v0, v118, v2, 3u64, f14(v4), Adt0::C14, v6, {
                                                let a149 = v7;
                                                let a150 = 1u64;
                                                nat_add(a149, a150)?
                                            }, List::<Adt5>::nil())
                                        }
                                        Err(v132) => {
                                            Ok(Adt8::C0(v132))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        }
                    } else {
                        let a152 = v3;
                        let a153 = 4u64;
                        if nat_eq(a152, a153) {
                            let m154 = f12(v0.clone(), v117, v2.clone(), 5u64, v4.clone(), v5.clone(), f13(Str::lit("")), 0u64, v8)?;
                            match m154 {
                                Adt8::C0(v136) => {
                                    Ok(Adt8::C0(v136))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C2(v135) => {
                                    f12(v0, v118, v2, 4u64, v4, v5, f13(Str::lit("")), 0u64, v135)
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C15(r188, v139, r189) => {
            let v138 = (*r188).clone();
            let v140 = (*r189).clone();
            let a155 = v3;
            let a156 = 5u64;
            if nat_eq(a155, a156) {
                let m157 = f19(v0.clone(), v138.clone(), v5)?;
                match m157 {
                    Ok(v141) => {
                        let v142: u64 = f41(v139.clone())?;
                        let v143: u64 = f30(v141.clone())?;
                        let a158 = v142;
                        let a159 = v143;
                        if nat_eq(a158, a159) {
                            if f42(v139.clone()) {
                                Ok(Adt8::C0(f23(f13(Str::lit("shape ")), f23(f28(v138)?, f13(Str::lit(" binds the same name twice"))))))
                            } else {
                                if f43(v8.clone(), v138.clone()) {
                                    Ok(Adt8::C0(f23(f13(Str::lit("the arm for ")), f23(f28(v138)?, f13(Str::lit(" is unreachable"))))))
                                } else {
                                    let m160 = f12(v0, v140, f8(v139, v141, v2), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                    match m160 {
                                        Adt8::C0(v147) => {
                                            Ok(Adt8::C0(v147))
                                        }
                                        Adt8::C1(r162) => {
                                            let v144 = (*r162).clone();
                                            let m161 = f25(v144, v4, f13(Str::lit("an arm")))?;
                                            match m161 {
                                                Ok(_) => {
                                                    Ok(Adt8::C2(f44(v138, v8)))
                                                }
                                                Err(v146) => {
                                                    Ok(Adt8::C0(v146))
                                                }
                                            }
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("shape ")), f23(f28(v138)?, f23(f13(Str::lit(" binds ")), f23(f24(v143)?, f23(f13(Str::lit(" value(s), the arm names ")), f24(v142)?)))))))
                        }
                    }
                    Err(v149) => {
                        Ok(Adt8::C0(v149))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
    }
}

pub fn f13(v0: Str) -> Bytes {
    let a190 = v0;
    utf8_encode(a190)
}

pub fn f14(v0: Adt0) -> Adt0 {
    let m191 = f45(v0);
    match m191 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f15(v0: Adt0) -> u64 {
    let m192 = v0;
    match m192 {
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

pub fn f16(v0: Adt0) -> Adt0 {
    let m193 = f46(v0);
    match m193 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f17(p197: List<List<u64>>, p198: u64) -> Option<List<u64>> {
    let m196 = (p197, p198);
    {
        let mut m196 = m196;
        loop {
            let step = {
                let (v0, v1) = m196;
                let m194 = v0;
                match m194.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m195 = v1;
                        if m195 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m195 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt7, v1: u64) -> Option<Adt0> {
    f39(match v0 {
        Adt7::C0(_, _, _, h199, _, _) => {
            (*h199).clone()
        }
    }, v1)
}

pub fn f19(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Result<Adt0, Bytes>> {
    let m200 = v1.clone();
    match m200 {
        Adt5::C0 => {
            let a201 = f15(v2.clone());
            let a202 = 8u64;
            if nat_eq(a201, a202) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C1 => {
            let a203 = f15(v2.clone());
            let a204 = 8u64;
            if nat_eq(a203, a204) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2)), std::rc::Rc::new(Adt0::C14))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C2 => {
            let a205 = f15(v2.clone());
            let a206 = 9u64;
            if nat_eq(a205, a206) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2)), std::rc::Rc::new(Adt0::C14))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C3 => {
            let a207 = f15(v2.clone());
            let a208 = 9u64;
            if nat_eq(a207, a208) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f14(v2)), std::rc::Rc::new(Adt0::C14))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C4 => {
            let a209 = f15(v2.clone());
            let a210 = 10u64;
            if nat_eq(a209, a210) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C5 => {
            let a211 = f15(v2.clone());
            let a212 = 10u64;
            if nat_eq(a211, a212) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v2), std::rc::Rc::new(Adt0::C14))))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C6 => {
            let a213 = f15(v2.clone());
            let a214 = 2u64;
            if nat_eq(a213, a214) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C7 => {
            let a215 = f15(v2.clone());
            let a216 = 2u64;
            if nat_eq(a215, a216) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C8 => {
            let a217 = f15(v2.clone());
            let a218 = 11u64;
            if nat_eq(a217, a218) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f14(v2)), std::rc::Rc::new(Adt0::C14))))))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C9 => {
            let a219 = f15(v2.clone());
            let a220 = 1u64;
            if nat_eq(a219, a220) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C10 => {
            let a221 = f15(v2.clone());
            let a222 = 1u64;
            if nat_eq(a221, a222) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C11 => {
            let a223 = f15(v2.clone());
            let a224 = 0u64;
            if nat_eq(a223, a224) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C12 => {
            let a225 = f15(v2.clone());
            let a226 = 7u64;
            if nat_eq(a225, a226) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C13 => {
            let a227 = f15(v2.clone());
            let a228 = 7u64;
            if nat_eq(a227, a228) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C14 => {
            let a229 = f15(v2.clone());
            let a230 = 7u64;
            if nat_eq(a229, a230) {
                Ok(Ok::<Adt0, Bytes>(Adt0::C14))
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
        Adt5::C15(v3) => {
            let v4: Result<Adt0, Bytes> = f47(v0, f36(v2.clone()), v3)?;
            let a231 = f15(v2.clone());
            let a232 = 12u64;
            if nat_eq(a231, a232) {
                Ok(v4)
            } else {
                Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2)?)))))
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m233 = v1;
    match m233 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(r239) => {
            let v3 = (*r239).clone();
            f20(v0, v3)
        }
        Adt0::C9(r240, r241) => {
            let v4 = (*r240).clone();
            let v5 = (*r241).clone();
            let m234 = f20(v0.clone(), v4)?;
            match m234 {
                Ok(_) => {
                    f20(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt0::C10(r242) => {
            let v8 = (*r242).clone();
            f20(v0, v8)
        }
        Adt0::C11(r243, r244) => {
            let v9 = (*r243).clone();
            let v10 = (*r244).clone();
            let m235 = f20(v0.clone(), v9)?;
            match m235 {
                Ok(_) => {
                    f20(v0, v10)
                }
                Err(v12) => {
                    Ok(Err::<(), Bytes>(v12))
                }
            }
        }
        Adt0::C12(v13) => {
            let m236 = f37(v0, v13);
            match m236 {
                None => {
                    Ok(Err::<(), Bytes>(f38(v13)?))
                }
                Some(_) => {
                    Ok(Ok::<(), Bytes>(()))
                }
            }
        }
        Adt0::C13(r245, r246) => {
            let v15 = (*r245).clone();
            let v16 = (*r246).clone();
            if f7(v15.clone()) {
                Ok(Err::<(), Bytes>(f13(Str::lit("a function type takes at least one parameter"))))
            } else {
                let m237 = f20(v0.clone(), v15)?;
                match m237 {
                    Ok(_) => {
                        f20(v0, v16)
                    }
                    Err(v18) => {
                        Ok(Err::<(), Bytes>(v18))
                    }
                }
            }
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r247, r248) => {
            let v19 = (*r247).clone();
            let v20 = (*r248).clone();
            let m238 = f20(v0.clone(), v19)?;
            match m238 {
                Ok(_) => {
                    f20(v0, v20)
                }
                Err(v22) => {
                    Ok(Err::<(), Bytes>(v22))
                }
            }
        }
    }
}

pub fn f21(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m249 = v1;
    match m249 {
        Adt3::C0 => {
            let a250 = f15(v2.clone());
            let a251 = 0u64;
            if nat_eq(a250, a251) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C0)?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C1(v4) => {
            let a252 = f15(v2.clone());
            let a253 = 1u64;
            if nat_eq(a252, a253) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C1(v4))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C2(v5) => {
            let a254 = f15(v2.clone());
            let a255 = 2u64;
            if nat_eq(a254, a255) {
                f49(v5)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C2(v5))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C3(v6) => {
            let a256 = f15(v2.clone());
            let a257 = 3u64;
            if nat_eq(a256, a257) {
                f50(v6, Adt1::C7)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C3(v6))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C4(v7) => {
            let a258 = f15(v2.clone());
            let a259 = 4u64;
            if if nat_eq(a258, a259) {
                let a260 = f51(f52(v2.clone()));
                let a261 = 0u64;
                nat_eq(a260, a261)
            } else {
                false
            } {
                f50(v7, Adt1::C0)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C4(v7))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C5(v8) => {
            let a262 = f15(v2.clone());
            let a263 = 4u64;
            if if nat_eq(a262, a263) {
                let a264 = f51(f52(v2.clone()));
                let a265 = 1u64;
                nat_eq(a264, a265)
            } else {
                false
            } {
                f50(v8, Adt1::C1)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C5(v8))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C6(v9) => {
            let a266 = f15(v2.clone());
            let a267 = 4u64;
            if if nat_eq(a266, a267) {
                let a268 = f51(f52(v2.clone()));
                let a269 = 2u64;
                nat_eq(a268, a269)
            } else {
                false
            } {
                f50(v9, Adt1::C2)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C6(v9))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C7(v10) => {
            let a270 = f15(v2.clone());
            let a271 = 4u64;
            if if nat_eq(a270, a271) {
                let a272 = f51(f52(v2.clone()));
                let a273 = 3u64;
                nat_eq(a272, a273)
            } else {
                false
            } {
                f50(v10, Adt1::C3)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C7(v10))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C8(v11) => {
            let a274 = f15(v2.clone());
            let a275 = 4u64;
            if if nat_eq(a274, a275) {
                let a276 = f51(f52(v2.clone()));
                let a277 = 4u64;
                nat_eq(a276, a277)
            } else {
                false
            } {
                f50(v11, Adt1::C4)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C8(v11))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C9(v12) => {
            let a278 = f15(v2.clone());
            let a279 = 4u64;
            if if nat_eq(a278, a279) {
                let a280 = f51(f52(v2.clone()));
                let a281 = 5u64;
                nat_eq(a280, a281)
            } else {
                false
            } {
                f50(v12, Adt1::C5)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C9(v12))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C10(v13) => {
            let a282 = f15(v2.clone());
            let a283 = 4u64;
            if if nat_eq(a282, a283) {
                let a284 = f51(f52(v2.clone()));
                let a285 = 6u64;
                nat_eq(a284, a285)
            } else {
                false
            } {
                f50(v13, Adt1::C6)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C10(v13))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C11(v14) => {
            let a286 = f15(v2.clone());
            let a287 = 4u64;
            if if nat_eq(a286, a287) {
                let a288 = f51(f52(v2.clone()));
                let a289 = 7u64;
                nat_eq(a288, a289)
            } else {
                false
            } {
                f50(v14, Adt1::C7)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C11(v14))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C12(v15) => {
            let a290 = f15(v2.clone());
            let a291 = 5u64;
            if nat_eq(a290, a291) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C12(v15))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C13(v16) => {
            let a292 = f15(v2.clone());
            let a293 = 6u64;
            if nat_eq(a292, a293) {
                f53(v16)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C13(v16))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C14(v17) => {
            let a294 = f15(v2.clone());
            let a295 = 7u64;
            if nat_eq(a294, a295) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C14(v17))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C15 => {
            let a296 = f15(v2.clone());
            let a297 = 8u64;
            if nat_eq(a296, a297) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C15)?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C16(r323) => {
            let v18 = (*r323).clone();
            let a298 = f15(v2.clone());
            let a299 = 8u64;
            if nat_eq(a298, a299) {
                f21(v0, v18, f16(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C16(std::rc::Rc::new(v18)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C17(r324) => {
            let v19 = (*r324).clone();
            let a300 = f15(v2.clone());
            let a301 = 9u64;
            if nat_eq(a300, a301) {
                f21(v0, v19, f16(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C17(std::rc::Rc::new(v19)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C18(r325) => {
            let v20 = (*r325).clone();
            let a302 = f15(v2.clone());
            let a303 = 9u64;
            if nat_eq(a302, a303) {
                f21(v0, v20, f14(v2), 0u64)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C18(std::rc::Rc::new(v20)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C19(r326) => {
            let v21 = (*r326).clone();
            let a304 = f15(v2.clone());
            let a305 = 10u64;
            if nat_eq(a304, a305) {
                f21(v0, v21, f16(v2), 1u64)
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C19(std::rc::Rc::new(v21)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C20(r327, r328) => {
            let v22 = (*r327).clone();
            let v23 = (*r328).clone();
            let v24: Result<(), Bytes> = f21(v0.clone(), v22.clone(), f16(v2.clone()), 0u64)?;
            let v25: Result<(), Bytes> = f21(v0, v23.clone(), f14(v2.clone()), 0u64)?;
            let a306 = f15(v2.clone());
            let a307 = 11u64;
            if nat_eq(a306, a307) {
                let m308 = v24;
                match m308 {
                    Ok(_) => {
                        Ok(v25)
                    }
                    Err(v27) => {
                        Ok(Err::<(), Bytes>(v27))
                    }
                }
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C20(std::rc::Rc::new(v22), std::rc::Rc::new(v23)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C21(v28, r329) => {
            let v29 = (*r329).clone();
            let a309 = f15(v2.clone());
            let a310 = 12u64;
            if nat_eq(a309, a310) {
                let m311 = f47(v0.clone(), f36(v2.clone()), v28)?;
                match m311 {
                    Ok(v31) => {
                        let v32: Result<(), Bytes> = f21(v0, v29.clone(), v31.clone(), 2u64)?;
                        let a312 = f54(v29.clone())?;
                        let a313 = f30(v31.clone())?;
                        if nat_eq(a312, a313) {
                            Ok(v32)
                        } else {
                            Ok(Err::<(), Bytes>(f23(f13(Str::lit("constructor ")), f23(f24(v28)?, f23(f13(Str::lit(" of ADT ")), f23(f24(f36(v2))?, f23(f13(Str::lit(" takes ")), f23(f24(f30(v31)?)?, f23(f13(Str::lit(" fields, the literal has ")), f24(f54(v29)?)?)))))))))
                        }
                    }
                    Err(v30) => {
                        Ok(Err::<(), Bytes>(v30))
                    }
                }
            } else {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C21(v28, std::rc::Rc::new(v29)))?, f23(f13(Str::lit(" does not have type ")), f35(v2)?)))))
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f13(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r330, r331) => {
            let v35 = (*r330).clone();
            let v36 = (*r331).clone();
            let v37: Adt0 = {
                let a314 = v3;
                let a315 = 2u64;
                if nat_eq(a314, a315) {
                    f16(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v38: Adt0 = {
                let a316 = v3;
                let a317 = 2u64;
                if nat_eq(a316, a317) {
                    f14(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v39: Result<(), Bytes> = f21(v0.clone(), v35, v37, 0u64)?;
            let v40: Result<(), Bytes> = f21(v0, v36, v38, v3)?;
            let a318 = v3;
            let a319 = 0u64;
            if nat_eq(a318, a319) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a320 = v3;
                let a321 = 2u64;
                if if nat_eq(a320, a321) {
                    f7(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m322 = v39;
                    match m322 {
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

pub fn f22(v0: List<(u64, Adt0)>, v1: u64) -> Option<Adt0> {
    let m332 = v0;
    match m332.uncons() {
        None => {
            None::<Adt0>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt0> = f22(v3, v1);
            let a334 = {
                let (h333, _) = v2;
                h333
            };
            let a335 = v1;
            if nat_eq(a334, a335) {
                Some({
                    let (_, h336) = v2;
                    h336
                })
            } else {
                v4
            }
        }
    }
}

pub fn f23(v0: Bytes, v1: Bytes) -> Bytes {
    let a337 = v0;
    let a338 = v1;
    append_bytes(a337, a338)
}

pub fn f24(v0: u64) -> R<Bytes> {
    let v1: Bytes = f13(Str::lit("0"));
    let v2: Bytes = f55({
        let a375 = f56(v0, 10000000000000000000u64);
        let a376 = {
            let a373 = f56(v0, 1000000000000000000u64);
            let a374 = {
                let a371 = f56(v0, 100000000000000000u64);
                let a372 = {
                    let a369 = f56(v0, 10000000000000000u64);
                    let a370 = {
                        let a367 = f56(v0, 1000000000000000u64);
                        let a368 = {
                            let a365 = f56(v0, 100000000000000u64);
                            let a366 = {
                                let a363 = f56(v0, 10000000000000u64);
                                let a364 = {
                                    let a361 = f56(v0, 1000000000000u64);
                                    let a362 = {
                                        let a359 = f56(v0, 100000000000u64);
                                        let a360 = {
                                            let a357 = f56(v0, 10000000000u64);
                                            let a358 = {
                                                let a355 = f56(v0, 1000000000u64);
                                                let a356 = {
                                                    let a353 = f56(v0, 100000000u64);
                                                    let a354 = {
                                                        let a351 = f56(v0, 10000000u64);
                                                        let a352 = {
                                                            let a349 = f56(v0, 1000000u64);
                                                            let a350 = {
                                                                let a347 = f56(v0, 100000u64);
                                                                let a348 = {
                                                                    let a345 = f56(v0, 10000u64);
                                                                    let a346 = {
                                                                        let a343 = f56(v0, 1000u64);
                                                                        let a344 = {
                                                                            let a341 = f56(v0, 100u64);
                                                                            let a342 = {
                                                                                let a339 = f56(v0, 10u64);
                                                                                let a340 = f56(v0, 1u64);
                                                                                append_bytes(a339, a340)
                                                                            };
                                                                            append_bytes(a341, a342)
                                                                        };
                                                                        append_bytes(a343, a344)
                                                                    };
                                                                    append_bytes(a345, a346)
                                                                };
                                                                append_bytes(a347, a348)
                                                            };
                                                            append_bytes(a349, a350)
                                                        };
                                                        append_bytes(a351, a352)
                                                    };
                                                    append_bytes(a353, a354)
                                                };
                                                append_bytes(a355, a356)
                                            };
                                            append_bytes(a357, a358)
                                        };
                                        append_bytes(a359, a360)
                                    };
                                    append_bytes(a361, a362)
                                };
                                append_bytes(a363, a364)
                            };
                            append_bytes(a365, a366)
                        };
                        append_bytes(a367, a368)
                    };
                    append_bytes(a369, a370)
                };
                append_bytes(a371, a372)
            };
            append_bytes(a373, a374)
        };
        append_bytes(a375, a376)
    })?;
    let a377 = v0;
    let a378 = 0u64;
    if nat_eq(a377, a378) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f25(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Result<(), Bytes>> {
    if f57(v0.clone(), v1.clone()) {
        Ok(Ok::<(), Bytes>(()))
    } else {
        Ok(Err::<(), Bytes>(f23(v2, f23(f13(Str::lit(" has type ")), f23(f35(v0)?, f23(f13(Str::lit(", expected ")), f35(v1)?))))))
    }
}

pub fn f26(v0: Adt7, v1: Adt0) -> R<Result<List<Adt5>, Bytes>> {
    let m379 = v1.clone();
    match m379 {
        Adt0::C0 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C11, List::<Adt5>::nil())))
        }
        Adt0::C1 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C9, List::cons(Adt5::C10, List::<Adt5>::nil()))))
        }
        Adt0::C2 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C6, List::cons(Adt5::C7, List::<Adt5>::nil()))))
        }
        Adt0::C3 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C4(_) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C5 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C6 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C7 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C12, List::cons(Adt5::C13, List::cons(Adt5::C14, List::<Adt5>::nil())))))
        }
        Adt0::C8(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C0, List::cons(Adt5::C1, List::<Adt5>::nil()))))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C2, List::cons(Adt5::C3, List::<Adt5>::nil()))))
        }
        Adt0::C10(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C4, List::cons(Adt5::C5, List::<Adt5>::nil()))))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C8, List::<Adt5>::nil())))
        }
        Adt0::C12(v9) => {
            let m380 = f37(v0, v9);
            match m380 {
                None => {
                    Ok(Err::<List<Adt5>, Bytes>(f38(v9)?))
                }
                Some(v10) => {
                    Ok(Ok::<List<Adt5>, Bytes>(f58(v10, 0u64)?))
                }
            }
        }
        Adt0::C13(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C14 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C15(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
    }
}

pub fn f27(v0: List<Adt5>, v1: List<Adt5>) -> Option<Adt5> {
    let m381 = v0;
    match m381.uncons() {
        None => {
            None::<Adt5>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt5> = f27(v3, v1.clone());
            if f43(v1, v2.clone()) {
                v4
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f28(v0: Adt5) -> R<Bytes> {
    let m382 = v0;
    match m382 {
        Adt5::C0 => {
            Ok(f13(Str::lit("None")))
        }
        Adt5::C1 => {
            Ok(f13(Str::lit("Some")))
        }
        Adt5::C2 => {
            Ok(f13(Str::lit("Ok")))
        }
        Adt5::C3 => {
            Ok(f13(Str::lit("Error")))
        }
        Adt5::C4 => {
            Ok(f13(Str::lit("Nil")))
        }
        Adt5::C5 => {
            Ok(f13(Str::lit("Cons")))
        }
        Adt5::C6 => {
            Ok(f13(Str::lit("Zero")))
        }
        Adt5::C7 => {
            Ok(f13(Str::lit("Succ")))
        }
        Adt5::C8 => {
            Ok(f13(Str::lit("Pair")))
        }
        Adt5::C9 => {
            Ok(f13(Str::lit("True")))
        }
        Adt5::C10 => {
            Ok(f13(Str::lit("False")))
        }
        Adt5::C11 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt5::C12 => {
            Ok(f13(Str::lit("Lt")))
        }
        Adt5::C13 => {
            Ok(f13(Str::lit("Eq")))
        }
        Adt5::C14 => {
            Ok(f13(Str::lit("Gt")))
        }
        Adt5::C15(v1) => {
            Ok(f23(f13(Str::lit("Adt { constructor: ")), f23(f24(v1)?, f13(Str::lit(" }")))))
        }
    }
}

pub fn f29(v0: Adt2) -> R<u64> {
    let m383 = v0;
    match m383 {
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
        Adt2::C14(_, r386) => {
            let v33 = (*r386).clone();
            let v34: u64 = f29(v33)?;
            let a384 = 1u64;
            let a385 = v34;
            nat_add(a384, a385)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f30(v0: Adt0) -> R<u64> {
    let m387 = v0;
    match m387 {
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
        Adt0::C15(_, r390) => {
            let v12 = (*r390).clone();
            let v13: u64 = f30(v12)?;
            let a388 = 1u64;
            let a389 = v13;
            nat_add(a388, a389)
        }
    }
}

pub fn f31(v0: u64) -> R<Bytes> {
    Ok(f23(f13(Str::lit("function ")), f23(f24(v0)?, f13(Str::lit(" is not declared")))))
}

pub fn f32(v0: Adt7, v1: u64) -> Adt0 {
    let m392 = f39(match v0 {
        Adt7::C0(_, _, _, _, h391, _) => {
            (*h391).clone()
        }
    }, v1);
    match m392 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C13(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f34(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    let v2: Adt0 = f16(v1.clone());
    let v3: Adt0 = if f7(v1.clone()) {
        Adt0::C0
    } else {
        v2
    };
    let m393 = v0.clone();
    match m393 {
        Adt6::C0 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C1 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C2 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C3 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C4 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C5 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C6 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C7 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C8 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C9 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C10 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C11 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C3)
        }
        Adt6::C12 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C13 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C14 => {
            let a394 = f15(v3.clone());
            let a395 = 4u64;
            if nat_eq(a394, a395) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C15 => {
            let a396 = f15(v3.clone());
            let a397 = 4u64;
            if nat_eq(a396, a397) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C16 => {
            let a398 = f15(v3.clone());
            let a399 = 4u64;
            if nat_eq(a398, a399) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C17 => {
            let a402 = f15(v3.clone());
            let a403 = 4u64;
            if if nat_eq(a402, a403) {
                f61(f52(v3.clone()))
            } else {
                false
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C18 => {
            let a400 = f15(v3.clone());
            let a401 = 4u64;
            if nat_eq(a400, a401) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C19 => {
            let a404 = f15(v3.clone());
            let a405 = 4u64;
            if nat_eq(a404, a405) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, v3)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C20 => {
            let a406 = f15(v3.clone());
            let a407 = 4u64;
            if nat_eq(a406, a407) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, v3)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C21 => {
            let a408 = f15(v3.clone());
            let a409 = 4u64;
            if nat_eq(a408, a409) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, v3)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C22 => {
            let a410 = f15(v3.clone());
            let a411 = 4u64;
            if nat_eq(a410, a411) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1, v3)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C23 => {
            let a412 = f15(v3.clone());
            let a413 = 4u64;
            if nat_eq(a412, a413) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C24 => {
            let a414 = f15(v3.clone());
            let a415 = 4u64;
            if nat_eq(a414, a415) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C25 => {
            let a416 = f15(v3.clone());
            let a417 = 2u64;
            if if nat_eq(a416, a417) {
                true
            } else {
                let a418 = f15(v3.clone());
                let a419 = 1u64;
                if nat_eq(a418, a419) {
                    true
                } else {
                    let a420 = f15(v3.clone());
                    let a421 = 4u64;
                    if nat_eq(a420, a421) {
                        true
                    } else {
                        let a422 = f15(v3.clone());
                        let a423 = 5u64;
                        if nat_eq(a422, a423) {
                            true
                        } else {
                            let a424 = f15(v3.clone());
                            let a425 = 6u64;
                            if nat_eq(a424, a425) {
                                true
                            } else {
                                let a426 = f15(v3.clone());
                                let a427 = 7u64;
                                nat_eq(a426, a427)
                            }
                        }
                    }
                }
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C26 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C1)
        }
        Adt6::C27 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C28 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C29 => {
            let a428 = f15(v3.clone());
            let a429 = 10u64;
            if if nat_eq(a428, a429) {
                true
            } else {
                let a430 = f15(v3.clone());
                let a431 = 6u64;
                nat_eq(a430, a431)
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1, v3)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C30 => {
            let a432 = f15(v3.clone());
            let a433 = 10u64;
            if if nat_eq(a432, a433) {
                true
            } else {
                let a434 = f15(v3.clone());
                let a435 = 6u64;
                if nat_eq(a434, a435) {
                    true
                } else {
                    let a436 = f15(v3.clone());
                    let a437 = 5u64;
                    nat_eq(a436, a437)
                }
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C2)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C31 => {
            let v4: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(f16(v3.clone()))))?;
            let v5: Result<Adt0, Bytes> = f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(Adt1::C0))))?;
            let a438 = f15(v3.clone());
            let a439 = 10u64;
            if nat_eq(a438, a439) {
                Ok(v4)
            } else {
                let a440 = f15(v3.clone());
                let a441 = 6u64;
                if nat_eq(a440, a441) {
                    Ok(v5)
                } else {
                    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive Index of ")), f35(v3)?)))
                }
            }
        }
        Adt6::C32 => {
            let a442 = f15(v3.clone());
            let a443 = 10u64;
            if if nat_eq(a442, a443) {
                true
            } else {
                let a444 = f15(v3.clone());
                let a445 = 6u64;
                nat_eq(a444, a445)
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(v3)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C33 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C6)
        }
        Adt6::C34 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C5)))
        }
        Adt6::C35 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
        }
        Adt6::C36 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5)))))
        }
        Adt6::C37 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5))), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C5)
        }
        Adt6::C38 => {
            let a446 = f15(v3.clone());
            let a447 = 3u64;
            if if nat_eq(a446, a447) {
                true
            } else {
                let a448 = f15(v3.clone());
                let a449 = 4u64;
                nat_eq(a448, a449)
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C5)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C39 => {
            if f62(v3.clone()) {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C40(v6) => {
            let a450 = f15(v3.clone());
            let a451 = 3u64;
            if if nat_eq(a450, a451) {
                true
            } else {
                let a452 = f15(v3.clone());
                let a453 = 4u64;
                nat_eq(a452, a453)
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(v6))))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C41(r458) => {
            let v7 = (*r458).clone();
            let v8: u64 = f15(v7.clone());
            let a454 = v8;
            let a455 = 3u64;
            if if nat_eq(a454, a455) {
                true
            } else {
                let a456 = v8;
                let a457 = 4u64;
                nat_eq(a456, a457)
            } {
                f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(v7)))
            } else {
                f60(v0, v1)
            }
        }
        Adt6::C42 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C0)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C6)))
        }
    }
}

pub fn f35(v0: Adt0) -> R<Bytes> {
    let m459 = v0;
    match m459 {
        Adt0::C0 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f13(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f13(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f13(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f23(f13(Str::lit("Fixed { width: ")), f23(f63(v1), f13(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f13(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f13(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f13(Str::lit("Ordering")))
        }
        Adt0::C8(r460) => {
            let v2 = (*r460).clone();
            Ok(f23(f13(Str::lit("Option { value: ")), f23(f35(v2)?, f13(Str::lit(" }")))))
        }
        Adt0::C9(r461, r462) => {
            let v3 = (*r461).clone();
            let v4 = (*r462).clone();
            Ok(f23(f13(Str::lit("Result { ok: ")), f23(f35(v3)?, f23(f13(Str::lit(", error: ")), f23(f35(v4)?, f13(Str::lit(" }")))))))
        }
        Adt0::C10(r463) => {
            let v5 = (*r463).clone();
            Ok(f23(f13(Str::lit("List { element: ")), f23(f35(v5)?, f13(Str::lit(" }")))))
        }
        Adt0::C11(r464, r465) => {
            let v6 = (*r464).clone();
            let v7 = (*r465).clone();
            Ok(f23(f13(Str::lit("Pair { left: ")), f23(f35(v6)?, f23(f13(Str::lit(", right: ")), f23(f35(v7)?, f13(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f23(f13(Str::lit("Adt { index: ")), f23(f24(v8)?, f13(Str::lit(" }")))))
        }
        Adt0::C13(r466, r467) => {
            let v9 = (*r466).clone();
            let v10 = (*r467).clone();
            Ok(f23(f13(Str::lit("Fn { parameters: [")), f23(f35(v9)?, f23(f13(Str::lit("], result: ")), f23(f35(v10)?, f13(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f13(Str::lit("")))
        }
        Adt0::C15(r468, r469) => {
            let v11 = (*r468).clone();
            let v12 = (*r469).clone();
            let v13: Bytes = f35(v11)?;
            let v14: Bytes = f35(v12.clone())?;
            if f7(v12) {
                Ok(v13)
            } else {
                Ok(f23(v13, f23(f13(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f36(v0: Adt0) -> u64 {
    let m470 = v0;
    match m470 {
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

pub fn f37(v0: Adt7, v1: u64) -> Option<Adt0> {
    f39(match v0 {
        Adt7::C0(_, h471, _, _, _, _) => {
            (*h471).clone()
        }
    }, v1)
}

pub fn f38(v0: u64) -> R<Bytes> {
    Ok(f23(f13(Str::lit("ADT ")), f23(f24(v0)?, f13(Str::lit(" is not declared")))))
}

pub fn f39(p477: Adt0, p478: u64) -> Option<Adt0> {
    let m476 = (p477, p478);
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v0, v1) = m476;
                let m472 = v0;
                match m472 {
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
                    Adt0::C15(r474, r475) => {
                        let v12 = (*r474).clone();
                        let v13 = (*r475).clone();
                        let m473 = v1;
                        if m473 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m473 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f41(v0: List<u64>) -> R<u64> {
    f64(Fn0::F65, 0u64, v0)
}

pub fn f42(v0: List<u64>) -> bool {
    let m479 = v0;
    match m479.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let v3: bool = f42(v2.clone());
            let v4: bool = f66(v2, v1);
            if v4 {
                true
            } else {
                v3
            }
        }
    }
}

pub fn f43(v0: List<Adt5>, v1: Adt5) -> bool {
    let m480 = v0;
    match m480.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f67(v2, v1.clone());
            let v5: bool = f43(v3, v1);
            if v4 {
                true
            } else {
                v5
            }
        }
    }
}

pub fn f44(v0: Adt5, v1: List<Adt5>) -> List<Adt5> {
    List::cons(v0, v1)
}

pub fn f45(v0: Adt0) -> Option<Adt0> {
    let m481 = v0;
    match m481 {
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
        Adt0::C9(_, r482) => {
            let v4 = (*r482).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r483) => {
            let v7 = (*r483).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r484) => {
            let v10 = (*r484).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r485) => {
            let v12 = (*r485).clone();
            Some(v12)
        }
    }
}

pub fn f46(v0: Adt0) -> Option<Adt0> {
    let m486 = v0;
    match m486 {
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
        Adt0::C8(r487) => {
            let v2 = (*r487).clone();
            Some(v2)
        }
        Adt0::C9(r488, _) => {
            let v3 = (*r488).clone();
            Some(v3)
        }
        Adt0::C10(r489) => {
            let v5 = (*r489).clone();
            Some(v5)
        }
        Adt0::C11(r490, _) => {
            let v6 = (*r490).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r491, _) => {
            let v9 = (*r491).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r492, _) => {
            let v11 = (*r492).clone();
            Some(v11)
        }
    }
}

pub fn f47(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m493 = f37(v0, v1);
    match m493 {
        None => {
            Ok(Err::<Adt0, Bytes>(f38(v1)?))
        }
        Some(v3) => {
            let m494 = f39(v3, v2);
            match m494 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("ADT ")), f23(f24(v1)?, f23(f13(Str::lit(" has no constructor ")), f24(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f48(v0: Adt3) -> R<Bytes> {
    let m495 = v0;
    match m495 {
        Adt3::C0 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f23(f13(Str::lit("Bool { value: ")), f23(f68(v1), f13(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f23(f13(Str::lit("Nat { value: ")), f23(f69(v2)?, f13(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f23(f13(Str::lit("Int { value: ")), f23(f69(v3)?, f13(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f23(f13(Str::lit("U8 { value: ")), f23(f69(v4)?, f13(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f23(f13(Str::lit("U16 { value: ")), f23(f69(v5)?, f13(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f23(f13(Str::lit("U32 { value: ")), f23(f69(v6)?, f13(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f23(f13(Str::lit("U64 { value: ")), f23(f69(v7)?, f13(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f23(f13(Str::lit("I8 { value: ")), f23(f69(v8)?, f13(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f23(f13(Str::lit("I16 { value: ")), f23(f69(v9)?, f13(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f23(f13(Str::lit("I32 { value: ")), f23(f69(v10)?, f13(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f23(f13(Str::lit("I64 { value: ")), f23(f69(v11)?, f13(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f23(f13(Str::lit("String { value: ")), f23(f69(v12)?, f13(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f23(f13(Str::lit("Bytes { hex: ")), f23(f69(v13)?, f13(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f23(f13(Str::lit("Ordering { value: ")), f23(f70(v14), f13(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f13(Str::lit("None")))
        }
        Adt3::C16(r496) => {
            let v15 = (*r496).clone();
            Ok(f23(f13(Str::lit("Some { value: ")), f23(f48(v15)?, f13(Str::lit(" }")))))
        }
        Adt3::C17(r497) => {
            let v16 = (*r497).clone();
            Ok(f23(f13(Str::lit("Ok { value: ")), f23(f48(v16)?, f13(Str::lit(" }")))))
        }
        Adt3::C18(r498) => {
            let v17 = (*r498).clone();
            Ok(f23(f13(Str::lit("Error { value: ")), f23(f48(v17)?, f13(Str::lit(" }")))))
        }
        Adt3::C19(r499) => {
            let v18 = (*r499).clone();
            Ok(f23(f13(Str::lit("List { items: [")), f23(f48(v18)?, f13(Str::lit("] }")))))
        }
        Adt3::C20(r500, r501) => {
            let v19 = (*r500).clone();
            let v20 = (*r501).clone();
            Ok(f23(f13(Str::lit("Pair { left: ")), f23(f48(v19)?, f23(f13(Str::lit(", right: ")), f23(f48(v20)?, f13(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r502) => {
            let v22 = (*r502).clone();
            Ok(f23(f13(Str::lit("Adt { constructor: ")), f23(f24(v21)?, f23(f13(Str::lit(", fields: [")), f23(f48(v22)?, f13(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r503) => {
            let v24 = (*r503).clone();
            Ok(f23(f13(Str::lit("Closure { function: ")), f23(f24(v23)?, f23(f13(Str::lit(", captures: [")), f23(f48(v24)?, f13(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f13(Str::lit("")))
        }
        Adt3::C24(r504, r505) => {
            let v25 = (*r504).clone();
            let v26 = (*r505).clone();
            let v27: Bytes = f48(v25)?;
            let v28: Bytes = f48(v26.clone())?;
            if f71(v26) {
                Ok(v27)
            } else {
                Ok(f23(v27, f23(f13(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f49(v0: Bytes) -> R<Result<(), Bytes>> {
    let m506 = f72(v0.clone())?;
    match m506 {
        Ok(v1) => {
            let v2: bool = f73(match v1.clone() {
                Adt9::C0(_, h507) => {
                    h507
                }
            }, f13(Str::lit("18446744073709551615")));
            if match v1 {
                Adt9::C0(h508, _) => {
                    h508
                }
            } {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("nat literal ")), f23(v0, f13(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f23(f13(Str::lit("nat literal ")), f23(v0, f13(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f50(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m509 = f72(v0.clone())?;
    match m509 {
        Ok(v2) => {
            let v3: Adt9 = f74(v1.clone());
            let v4: Adt9 = f75(v1.clone());
            let v5: bool = f76(v3, v2.clone());
            let v6: bool = f76(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f23(v0, f23(f13(Str::lit(" is outside ")), f77(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f23(v0, f23(f13(Str::lit(" is outside ")), f77(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f51(v0: Adt1) -> u64 {
    let m510 = v0;
    match m510 {
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
    let m511 = v0;
    match m511 {
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

pub fn f53(v0: Bytes) -> R<Result<(), Bytes>> {
    let a516 = {
        let a513 = {
            let a512 = v0.clone();
            length_bytes(a512)
        };
        let a514 = 2u64;
        let a515 = 0u64;
        nat_rem(a513, a514, a515)
    };
    let a517 = 0u64;
    if nat_eq(a516, a517) {
        if f78(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f23(f13(Str::lit("byte literal `")), f23(v0, f13(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f23(f13(Str::lit("byte literal `")), f23(v0, f13(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f54(v0: Adt3) -> R<u64> {
    let m518 = v0;
    match m518 {
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
        Adt3::C24(_, r521) => {
            let v26 = (*r521).clone();
            let v27: u64 = f54(v26)?;
            let a519 = 1u64;
            let a520 = v27;
            nat_add(a519, a520)
        }
    }
}

pub fn f55(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h522, _) = f79(Fn1::F80(v0.clone()), {
            let a523 = v0.clone();
            length_bytes(a523)
        }, 0u64)?;
        h522
    };
    Ok(f81(v0.clone(), v1, {
        let a525 = {
            let a524 = v0;
            length_bytes(a524)
        };
        let a526 = v1;
        nat_sub(a525, a526)
    }))
}

pub fn f56(v0: u64, v1: u64) -> Bytes {
    f81(f13(Str::lit("0123456789")), {
        let a530 = {
            let a527 = v0;
            let a528 = v1;
            let a529 = 0u64;
            nat_quot(a527, a528, a529)
        };
        let a531 = 10u64;
        let a532 = 0u64;
        nat_rem(a530, a531, a532)
    }, 1u64)
}

pub fn f57(v0: Adt0, v1: Adt0) -> bool {
    let m533 = v0;
    match m533 {
        Adt0::C0 => {
            let a534 = f15(v1);
            let a535 = 0u64;
            nat_eq(a534, a535)
        }
        Adt0::C1 => {
            let a536 = f15(v1);
            let a537 = 1u64;
            nat_eq(a536, a537)
        }
        Adt0::C2 => {
            let a538 = f15(v1);
            let a539 = 2u64;
            nat_eq(a538, a539)
        }
        Adt0::C3 => {
            let a540 = f15(v1);
            let a541 = 3u64;
            nat_eq(a540, a541)
        }
        Adt0::C4(v2) => {
            let a550 = f15(v1.clone());
            let a551 = 4u64;
            if nat_eq(a550, a551) {
                let a552 = f51(v2);
                let a553 = f51(f52(v1));
                nat_eq(a552, a553)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a542 = f15(v1);
            let a543 = 5u64;
            nat_eq(a542, a543)
        }
        Adt0::C6 => {
            let a544 = f15(v1);
            let a545 = 6u64;
            nat_eq(a544, a545)
        }
        Adt0::C7 => {
            let a546 = f15(v1);
            let a547 = 7u64;
            nat_eq(a546, a547)
        }
        Adt0::C8(r570) => {
            let v3 = (*r570).clone();
            let v4: bool = f57(v3, f16(v1.clone()));
            let a554 = f15(v1);
            let a555 = 8u64;
            if nat_eq(a554, a555) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r571, r572) => {
            let v5 = (*r571).clone();
            let v6 = (*r572).clone();
            let v7: bool = f57(v5, f16(v1.clone()));
            let v8: bool = f57(v6, f14(v1.clone()));
            let a556 = f15(v1);
            let a557 = 9u64;
            if nat_eq(a556, a557) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r573) => {
            let v9 = (*r573).clone();
            let v10: bool = f57(v9, f16(v1.clone()));
            let a558 = f15(v1);
            let a559 = 10u64;
            if nat_eq(a558, a559) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r574, r575) => {
            let v11 = (*r574).clone();
            let v12 = (*r575).clone();
            let v13: bool = f57(v11, f16(v1.clone()));
            let v14: bool = f57(v12, f14(v1.clone()));
            let a560 = f15(v1);
            let a561 = 11u64;
            if nat_eq(a560, a561) {
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
            let a562 = f15(v1.clone());
            let a563 = 12u64;
            if nat_eq(a562, a563) {
                let a564 = v15;
                let a565 = f36(v1);
                nat_eq(a564, a565)
            } else {
                false
            }
        }
        Adt0::C13(r576, r577) => {
            let v16 = (*r576).clone();
            let v17 = (*r577).clone();
            let v18: bool = f57(v16, f16(v1.clone()));
            let v19: bool = f57(v17, f14(v1.clone()));
            let a566 = f15(v1);
            let a567 = 13u64;
            if nat_eq(a566, a567) {
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
            let a548 = f15(v1);
            let a549 = 14u64;
            nat_eq(a548, a549)
        }
        Adt0::C15(r578, r579) => {
            let v20 = (*r578).clone();
            let v21 = (*r579).clone();
            let v22: bool = f57(v20, f16(v1.clone()));
            let v23: bool = f57(v21, f14(v1.clone()));
            let a568 = f15(v1);
            let a569 = 15u64;
            if nat_eq(a568, a569) {
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

pub fn f58(v0: Adt0, v1: u64) -> R<List<Adt5>> {
    let m580 = v0;
    match m580 {
        Adt0::C0 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C1 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C2 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C3 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C6 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C7 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C8(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C9(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C10(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C11(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C12(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C13(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C14 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C15(_, r583) => {
            let v13 = (*r583).clone();
            Ok(f44(Adt5::C15(v1), f58(v13, {
                let a581 = v1;
                let a582 = 1u64;
                nat_add(a581, a582)?
            })?))
        }
    }
}

pub fn f59(v0: Adt6, v1: Adt0, v2: Adt0, v3: Adt0) -> R<Result<Adt0, Bytes>> {
    if f57(v2.clone(), v1.clone()) {
        Ok(Ok::<Adt0, Bytes>(v3))
    } else {
        Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive ")), f23(f82(v0)?, f23(f13(Str::lit(" takes ")), f23(f83(v1)?, f23(f13(Str::lit(", received ")), f83(v2)?)))))))
    }
}

pub fn f60(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive ")), f23(f82(v0)?, f23(f13(Str::lit(" does not apply to ")), f83(v1)?)))))
}

pub fn f61(v0: Adt1) -> bool {
    let m584 = v0;
    match m584 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            false
        }
        Adt1::C3 => {
            false
        }
        Adt1::C4 => {
            true
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6 => {
            true
        }
        Adt1::C7 => {
            true
        }
    }
}

pub fn f62(v0: Adt0) -> bool {
    let m585 = v0;
    match m585 {
        Adt0::C0 => {
            false
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
            false
        }
        Adt0::C7 => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            false
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r586, r587) => {
            let v2 = (*r586).clone();
            let v3 = (*r587).clone();
            let v4: bool = f62(v2);
            let v5: bool = f62(v3);
            if v4 {
                v5
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f63(v0: Adt1) -> Bytes {
    let m588 = v0;
    match m588 {
        Adt1::C0 => {
            f13(Str::lit("U8"))
        }
        Adt1::C1 => {
            f13(Str::lit("U16"))
        }
        Adt1::C2 => {
            f13(Str::lit("U32"))
        }
        Adt1::C3 => {
            f13(Str::lit("U64"))
        }
        Adt1::C4 => {
            f13(Str::lit("I8"))
        }
        Adt1::C5 => {
            f13(Str::lit("I16"))
        }
        Adt1::C6 => {
            f13(Str::lit("I32"))
        }
        Adt1::C7 => {
            f13(Str::lit("I64"))
        }
    }
}

pub fn f64(v0: Fn0, p592: u64, p593: List<u64>) -> R<u64> {
    let m591 = (p592, p593);
    {
        let mut m591 = m591;
        loop {
            let step = {
                let (v1, v2) = m591;
                let m589 = v2;
                match m589.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c590: Fn0 = v0.clone();
                            c590.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: u64, _: u64) -> R<u64> {
    let a594 = v0;
    let a595 = 1u64;
    nat_add(a594, a595)
}

pub fn f66(v0: List<u64>, v1: u64) -> bool {
    let m596 = v0;
    match m596.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f66(v3, v1);
            let a597 = v2;
            let a598 = v1;
            if nat_eq(a597, a598) {
                true
            } else {
                v4
            }
        }
    }
}

pub fn f67(v0: Adt5, v1: Adt5) -> bool {
    let a599 = f84(v0.clone());
    let a600 = f84(v1.clone());
    if nat_eq(a599, a600) {
        let a601 = f85(v0);
        let a602 = f85(v1);
        nat_eq(a601, a602)
    } else {
        false
    }
}

pub fn f68(v0: bool) -> Bytes {
    if v0 {
        f13(Str::lit("true"))
    } else {
        f13(Str::lit("false"))
    }
}

pub fn f69(v0: Bytes) -> R<Bytes> {
    Ok(f23(f13(Str::lit("\"")), f23({
        let (_, h603) = {
            let (h604, _) = f86(Fn2::F87(v0.clone()), {
                let a605 = v0;
                length_bytes(a605)
            }, (0u64, Bytes::lit(&[])))?;
            h604
        };
        h603
    }, f13(Str::lit("\"")))))
}

pub fn f70(v0: Adt4) -> Bytes {
    let m606 = v0;
    match m606 {
        Adt4::C0 => {
            f13(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f13(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f13(Str::lit("Gt"))
        }
    }
}

pub fn f71(v0: Adt3) -> bool {
    let m607 = v0;
    match m607 {
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

pub fn f72(v0: Bytes) -> R<Result<Adt9, Bytes>> {
    let v1: bool = f88(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f81(v0.clone(), 1u64, {
            let a609 = {
                let a608 = v0.clone();
                length_bytes(a608)
            };
            let a610 = 1u64;
            nat_sub(a609, a610)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f89(v3.clone(), v0.clone())?;
    let v5: bool = f73(v3.clone(), f13(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f73(v3.clone(), f13(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt9, Bytes>(Adt9::C0(v1, v3)))
        } else {
            Ok(Err::<Adt9, Bytes>(f23(f90(v0), f13(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt9, Bytes>(f23(f90(v0), f13(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f73(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a613 = {
            let a611 = v0.clone();
            length_bytes(a611)
        };
        let a614 = {
            let a612 = v1.clone();
            length_bytes(a612)
        };
        nat_lt(a613, a614)
    };
    let v3: bool = {
        let a617 = {
            let a615 = v0.clone();
            length_bytes(a615)
        };
        let a618 = {
            let a616 = v1.clone();
            length_bytes(a616)
        };
        nat_eq(a617, a618)
    };
    let v4: bool = f91(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a619 = v4;
            bool_not(a619)
        } else {
            false
        }
    }
}

pub fn f74(v0: Adt1) -> Adt9 {
    let m620 = v0;
    match m620 {
        Adt1::C0 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f92(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f92(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f92(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f92(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f75(v0: Adt1) -> Adt9 {
    let m621 = v0;
    match m621 {
        Adt1::C0 => {
            f92(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f92(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f92(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f92(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f92(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f76(v0: Adt9, v1: Adt9) -> bool {
    let v2: bool = f73(match v1.clone() {
        Adt9::C0(_, h622) => {
            h622
        }
    }, match v0.clone() {
        Adt9::C0(_, h623) => {
            h623
        }
    });
    let v3: bool = f73(match v0.clone() {
        Adt9::C0(_, h624) => {
            h624
        }
    }, match v1.clone() {
        Adt9::C0(_, h625) => {
            h625
        }
    });
    if match v0 {
        Adt9::C0(h626, _) => {
            h626
        }
    } {
        if match v1 {
            Adt9::C0(h627, _) => {
                h627
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt9::C0(h628, _) => {
                h628
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f77(v0: Adt1) -> Bytes {
    let m629 = v0;
    match m629 {
        Adt1::C0 => {
            f13(Str::lit("u8"))
        }
        Adt1::C1 => {
            f13(Str::lit("u16"))
        }
        Adt1::C2 => {
            f13(Str::lit("u32"))
        }
        Adt1::C3 => {
            f13(Str::lit("u64"))
        }
        Adt1::C4 => {
            f13(Str::lit("i8"))
        }
        Adt1::C5 => {
            f13(Str::lit("i16"))
        }
        Adt1::C6 => {
            f13(Str::lit("i32"))
        }
        Adt1::C7 => {
            f13(Str::lit("i64"))
        }
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    let a633 = {
        let (h630, _) = f79(Fn1::F93(v0.clone()), {
            let a631 = v0.clone();
            length_bytes(a631)
        }, 0u64)?;
        h630
    };
    let a634 = {
        let a632 = v0;
        length_bytes(a632)
    };
    Ok(nat_eq(a633, a634))
}

pub fn f79(v0: Fn1, p639: u64, p640: u64) -> R<(u64, bool)> {
    let m638 = (p639, p640);
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v1, v2) = m638;
                let m635 = v1;
                if m635 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m635 - 1;
                    let m637 = {
                        let c636: Fn1 = v0.clone();
                        c636.apply(v2)?
                    };
                    match m637 {
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
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f94(v0, v1)
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m644 = {
        let a641 = v0;
        let a642 = v1;
        let a643 = v2;
        slice_bytes(a641, a642, a643)
    };
    match m644 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f82(v0: Adt6) -> R<Bytes> {
    let m645 = v0;
    match m645 {
        Adt6::C0 => {
            Ok(f13(Str::lit("NatAdd")))
        }
        Adt6::C1 => {
            Ok(f13(Str::lit("NatSub")))
        }
        Adt6::C2 => {
            Ok(f13(Str::lit("NatMul")))
        }
        Adt6::C3 => {
            Ok(f13(Str::lit("NatQuot")))
        }
        Adt6::C4 => {
            Ok(f13(Str::lit("NatRem")))
        }
        Adt6::C5 => {
            Ok(f13(Str::lit("NatEq")))
        }
        Adt6::C6 => {
            Ok(f13(Str::lit("NatLe")))
        }
        Adt6::C7 => {
            Ok(f13(Str::lit("NatLt")))
        }
        Adt6::C8 => {
            Ok(f13(Str::lit("IntAdd")))
        }
        Adt6::C9 => {
            Ok(f13(Str::lit("IntSub")))
        }
        Adt6::C10 => {
            Ok(f13(Str::lit("IntMul")))
        }
        Adt6::C11 => {
            Ok(f13(Str::lit("IntNeg")))
        }
        Adt6::C12 => {
            Ok(f13(Str::lit("IntQuot")))
        }
        Adt6::C13 => {
            Ok(f13(Str::lit("IntRem")))
        }
        Adt6::C14 => {
            Ok(f13(Str::lit("CheckedAdd")))
        }
        Adt6::C15 => {
            Ok(f13(Str::lit("CheckedSub")))
        }
        Adt6::C16 => {
            Ok(f13(Str::lit("CheckedMul")))
        }
        Adt6::C17 => {
            Ok(f13(Str::lit("CheckedNeg")))
        }
        Adt6::C18 => {
            Ok(f13(Str::lit("CheckedQuot")))
        }
        Adt6::C19 => {
            Ok(f13(Str::lit("BitAnd")))
        }
        Adt6::C20 => {
            Ok(f13(Str::lit("BitOr")))
        }
        Adt6::C21 => {
            Ok(f13(Str::lit("BitXor")))
        }
        Adt6::C22 => {
            Ok(f13(Str::lit("BitNot")))
        }
        Adt6::C23 => {
            Ok(f13(Str::lit("ShiftLeft")))
        }
        Adt6::C24 => {
            Ok(f13(Str::lit("ShiftRight")))
        }
        Adt6::C25 => {
            Ok(f13(Str::lit("Equal")))
        }
        Adt6::C26 => {
            Ok(f13(Str::lit("BoolNot")))
        }
        Adt6::C27 => {
            Ok(f13(Str::lit("BoolAnd")))
        }
        Adt6::C28 => {
            Ok(f13(Str::lit("BoolOr")))
        }
        Adt6::C29 => {
            Ok(f13(Str::lit("Append")))
        }
        Adt6::C30 => {
            Ok(f13(Str::lit("Length")))
        }
        Adt6::C31 => {
            Ok(f13(Str::lit("Index")))
        }
        Adt6::C32 => {
            Ok(f13(Str::lit("Slice")))
        }
        Adt6::C33 => {
            Ok(f13(Str::lit("Utf8Encode")))
        }
        Adt6::C34 => {
            Ok(f13(Str::lit("Utf8Decode")))
        }
        Adt6::C35 => {
            Ok(f13(Str::lit("CompareBytes")))
        }
        Adt6::C36 => {
            Ok(f13(Str::lit("SplitExact")))
        }
        Adt6::C37 => {
            Ok(f13(Str::lit("Join")))
        }
        Adt6::C38 => {
            Ok(f13(Str::lit("FormatDecimal")))
        }
        Adt6::C39 => {
            Ok(f13(Str::lit("Compare")))
        }
        Adt6::C40(v1) => {
            Ok(f23(f13(Str::lit("Convert { target: ")), f23(f63(v1), f13(Str::lit(" }")))))
        }
        Adt6::C41(r646) => {
            let v2 = (*r646).clone();
            Ok(f23(f13(Str::lit("ParseDecimal { target: ")), f23(f35(v2)?, f13(Str::lit(" }")))))
        }
        Adt6::C42 => {
            Ok(f13(Str::lit("Update")))
        }
    }
}

pub fn f83(v0: Adt0) -> R<Bytes> {
    Ok(f23(f13(Str::lit("[")), f23(f35(v0)?, f13(Str::lit("]")))))
}

pub fn f84(v0: Adt5) -> u64 {
    let m647 = v0;
    match m647 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            1u64
        }
        Adt5::C2 => {
            2u64
        }
        Adt5::C3 => {
            3u64
        }
        Adt5::C4 => {
            4u64
        }
        Adt5::C5 => {
            5u64
        }
        Adt5::C6 => {
            6u64
        }
        Adt5::C7 => {
            7u64
        }
        Adt5::C8 => {
            8u64
        }
        Adt5::C9 => {
            9u64
        }
        Adt5::C10 => {
            10u64
        }
        Adt5::C11 => {
            11u64
        }
        Adt5::C12 => {
            12u64
        }
        Adt5::C13 => {
            13u64
        }
        Adt5::C14 => {
            14u64
        }
        Adt5::C15(_) => {
            15u64
        }
    }
}

pub fn f85(v0: Adt5) -> u64 {
    let m648 = v0;
    match m648 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            0u64
        }
        Adt5::C2 => {
            0u64
        }
        Adt5::C3 => {
            0u64
        }
        Adt5::C4 => {
            0u64
        }
        Adt5::C5 => {
            0u64
        }
        Adt5::C6 => {
            0u64
        }
        Adt5::C7 => {
            0u64
        }
        Adt5::C8 => {
            0u64
        }
        Adt5::C9 => {
            0u64
        }
        Adt5::C10 => {
            0u64
        }
        Adt5::C11 => {
            0u64
        }
        Adt5::C12 => {
            0u64
        }
        Adt5::C13 => {
            0u64
        }
        Adt5::C14 => {
            0u64
        }
        Adt5::C15(v1) => {
            v1
        }
    }
}

pub fn f86(v0: Fn2, p653: u64, p654: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m652 = (p653, p654);
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v1, v2) = m652;
                let m649 = v1;
                if m649 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m649 - 1;
                    let m651 = {
                        let c650: Fn2 = v0.clone();
                        c650.apply(v2.clone())?
                    };
                    match m651 {
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
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f95(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m655 = f96(v0, v1);
    match m655 {
        None => {
            false
        }
        Some(v3) => {
            let a656 = v3;
            let a657 = v2;
            equal(a656, a657)
        }
    }
}

pub fn f89(v0: Bytes, v1: Bytes) -> R<bool> {
    let a659 = 0u64;
    let a660 = {
        let a658 = v0.clone();
        length_bytes(a658)
    };
    if nat_lt(a659, a660) {
        if f97(v0.clone())? {
            let a661 = v0.clone();
            let a662 = f13(Str::lit("0"));
            if equal(a661, a662) {
                let a665 = {
                    let a663 = v1;
                    let a664 = f13(Str::lit("-0"));
                    equal(a663, a664)
                };
                Ok(bool_not(a665))
            } else {
                let a666 = f88(v0, 0u64, 48u8);
                Ok(bool_not(a666))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f90(v0: Bytes) -> Bytes {
    f23(f13(Str::lit("`")), f23(v0, f13(Str::lit("`"))))
}

pub fn f91(v0: Bytes, v1: Bytes) -> bool {
    let a671 = {
        let a667 = v0;
        let a668 = v1;
        compare_bytes(a667, a668)
    };
    let a672 = {
        let a669 = Bytes::lit(&[]);
        let a670 = Bytes::lit(&[0x01]);
        compare_bytes(a669, a670)
    };
    equal(a671, a672)
}

pub fn f92(v0: bool, v1: Str) -> Adt9 {
    Adt9::C0(v0, f13(v1))
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f94(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f88(v0, v1, 48u8) {
        Ok(Some({
            let a673 = v1;
            let a674 = 1u64;
            nat_add(a673, a674)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f95(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h675, _) = v1;
        h675
    };
    let v3: u64 = f99(v0.clone(), v2)?;
    let a676 = v3;
    let a677 = 256u64;
    if nat_eq(a676, a677) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f100(v0, v2, v3)?;
        Ok(Some(({
            let a679 = v2;
            let a680 = {
                let (h678, _) = v4;
                h678
            };
            nat_add(a679, a680)?
        }, f23({
            let (_, h681) = v1;
            h681
        }, {
            let (_, h682) = v4;
            h682
        }))))
    }
}

pub fn f96(v0: Bytes, v1: u64) -> Option<u8> {
    let a683 = v0;
    let a684 = v1;
    index_bytes(a683, a684)
}

pub fn f97(v0: Bytes) -> R<bool> {
    let a688 = {
        let (h685, _) = f79(Fn1::F101(v0.clone()), {
            let a686 = v0.clone();
            length_bytes(a686)
        }, 0u64)?;
        h685
    };
    let a689 = {
        let a687 = v0;
        length_bytes(a687)
    };
    Ok(nat_eq(a688, a689))
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f102(v0, v1)? {
        Ok(Some({
            let a690 = v1;
            let a691 = 1u64;
            nat_add(a690, a691)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<u64> {
    let m692 = f96(v0, v1);
    match m692 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a737 = {
                let a713 = {
                    let a701 = {
                        let a695 = {
                            let a693 = v3;
                            let a694 = 128u8;
                            fixed_u8::bit_and(a693, a694)
                        };
                        let a696 = 128u8;
                        if equal(a695, a696) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a702 = {
                        let a699 = {
                            let a697 = v3;
                            let a698 = 64u8;
                            fixed_u8::bit_and(a697, a698)
                        };
                        let a700 = 64u8;
                        if equal(a699, a700) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a701, a702)?
                };
                let a714 = {
                    let a711 = {
                        let a705 = {
                            let a703 = v3;
                            let a704 = 32u8;
                            fixed_u8::bit_and(a703, a704)
                        };
                        let a706 = 32u8;
                        if equal(a705, a706) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a712 = {
                        let a709 = {
                            let a707 = v3;
                            let a708 = 16u8;
                            fixed_u8::bit_and(a707, a708)
                        };
                        let a710 = 16u8;
                        if equal(a709, a710) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a711, a712)?
                };
                nat_add(a713, a714)?
            };
            let a738 = {
                let a735 = {
                    let a723 = {
                        let a717 = {
                            let a715 = v3;
                            let a716 = 8u8;
                            fixed_u8::bit_and(a715, a716)
                        };
                        let a718 = 8u8;
                        if equal(a717, a718) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a724 = {
                        let a721 = {
                            let a719 = v3;
                            let a720 = 4u8;
                            fixed_u8::bit_and(a719, a720)
                        };
                        let a722 = 4u8;
                        if equal(a721, a722) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a723, a724)?
                };
                let a736 = {
                    let a733 = {
                        let a727 = {
                            let a725 = v3;
                            let a726 = 2u8;
                            fixed_u8::bit_and(a725, a726)
                        };
                        let a728 = 2u8;
                        if equal(a727, a728) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a734 = {
                        let a731 = {
                            let a729 = v3;
                            let a730 = 1u8;
                            fixed_u8::bit_and(a729, a730)
                        };
                        let a732 = 1u8;
                        if equal(a731, a732) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a733, a734)?
                };
                nat_add(a735, a736)?
            };
            nat_add(a737, a738)
        }
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f99(v0.clone(), {
        let a739 = v1;
        let a740 = 1u64;
        nat_add(a739, a740)?
    })?;
    let a741 = v2;
    let a742 = 34u64;
    if nat_eq(a741, a742) {
        Ok((1u64, f13(Str::lit("\\\""))))
    } else {
        let a743 = v2;
        let a744 = 92u64;
        if nat_eq(a743, a744) {
            Ok((1u64, f13(Str::lit("\\\\"))))
        } else {
            let a745 = v2;
            let a746 = 10u64;
            if nat_eq(a745, a746) {
                Ok((1u64, f13(Str::lit("\\n"))))
            } else {
                let a747 = v2;
                let a748 = 13u64;
                if nat_eq(a747, a748) {
                    Ok((1u64, f13(Str::lit("\\r"))))
                } else {
                    let a749 = v2;
                    let a750 = 9u64;
                    if nat_eq(a749, a750) {
                        Ok((1u64, f13(Str::lit("\\t"))))
                    } else {
                        let a751 = v2;
                        let a752 = 0u64;
                        if nat_eq(a751, a752) {
                            Ok((1u64, f13(Str::lit("\\0"))))
                        } else {
                            let a753 = v2;
                            let a754 = 32u64;
                            if nat_lt(a753, a754) {
                                Ok((1u64, f103(v2)))
                            } else {
                                let a755 = v2;
                                let a756 = 127u64;
                                if nat_eq(a755, a756) {
                                    Ok((1u64, f103(v2)))
                                } else {
                                    let a757 = v2;
                                    let a758 = 194u64;
                                    if nat_eq(a757, a758) {
                                        let a759 = 128u64;
                                        let a760 = v3;
                                        if nat_le(a759, a760) {
                                            let a761 = v3;
                                            let a762 = 159u64;
                                            if nat_le(a761, a762) {
                                                Ok((2u64, f103(v3)))
                                            } else {
                                                Ok((1u64, f81(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f81(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f81(v0, v1, 1u64)))
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

pub fn f101(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f104(v0, v1)
}

pub fn f102(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f99(v0, v1)?;
    let a763 = 47u64;
    let a764 = v2;
    if nat_lt(a763, a764) {
        let a765 = v2;
        let a766 = 58u64;
        if nat_lt(a765, a766) {
            Ok(true)
        } else {
            let a767 = 96u64;
            let a768 = v2;
            if nat_lt(a767, a768) {
                let a769 = v2;
                let a770 = 103u64;
                Ok(nat_lt(a769, a770))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f103(v0: u64) -> Bytes {
    f23(f13(Str::lit("\\u{")), f23(f105(v0), f13(Str::lit("}"))))
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f106(v0, v1)? {
        Ok(Some({
            let a771 = v1;
            let a772 = 1u64;
            nat_add(a771, a772)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f105(v0: u64) -> Bytes {
    let v1: Bytes = f107(v0);
    let v2: Bytes = f23(f107({
        let a773 = v0;
        let a774 = 16u64;
        let a775 = 0u64;
        nat_quot(a773, a774, a775)
    }), f107({
        let a776 = v0;
        let a777 = 16u64;
        let a778 = 0u64;
        nat_rem(a776, a777, a778)
    }));
    let a779 = v0;
    let a780 = 16u64;
    if nat_lt(a779, a780) {
        v1
    } else {
        v2
    }
}

pub fn f106(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f99(v0, v1)?;
    let a781 = 47u64;
    let a782 = v2;
    if nat_lt(a781, a782) {
        let a783 = v2;
        let a784 = 58u64;
        Ok(nat_lt(a783, a784))
    } else {
        Ok(false)
    }
}

pub fn f107(v0: u64) -> Bytes {
    f81(f13(Str::lit("0123456789abcdef")), v0, 1u64)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5)
}
