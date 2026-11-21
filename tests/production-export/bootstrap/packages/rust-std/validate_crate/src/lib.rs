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
    C2(std::rc::Rc<Adt12>, std::rc::Rc<Adt13>, std::rc::Rc<Adt2>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
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
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C1(Adt19, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C2(Adt20, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C3(std::rc::Rc<Adt21>, std::rc::Rc<Adt17>, bool, std::rc::Rc<Adt9>),
    C4(std::rc::Rc<Adt12>, std::rc::Rc<Adt17>, bool, std::rc::Rc<Adt9>),
    C5(std::rc::Rc<Adt23>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(std::rc::Rc<Adt14>, bool, std::rc::Rc<Adt2>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>, std::rc::Rc<Adt17>),
    C15(std::rc::Rc<Adt14>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(std::rc::Rc<Adt12>, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt4),
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(std::rc::Rc<Adt22>),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
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
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, Bytes, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(u64, List<(u64, u64)>)>, List<(u64, List<(u64, u64)>)>, List<(u64, bool)>, List<(u64, bool)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(bool, std::rc::Rc<Adt26>, bool, List<Adt25>, bool),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Adt12>, List<Adt12>, List<Adt12>, List<(Adt12, List<u64>)>, bool),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, Bytes, Option<Adt4>, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(List<Adt29>, List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Fn0 {
    F35(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<List<(u64, u64)>>, p1: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
        match self {
            Fn0::F35(k0) => f35(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F37(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<bool>, p1: (u64, bool)) -> Option<bool> {
        match self {
            Fn1::F37(k0) => f37(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39(Bytes),
    F184(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F184(k0, k1, k2, k3) => f184(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F56,
}

impl Fn3 {
    pub fn apply(&self, p0: bool, p1: Adt11) -> bool {
        match self {
            Fn3::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F92(std::rc::Rc<Adt12>),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F92(k0) => f92((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F113,
    F117(std::rc::Rc<Adt12>),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F113 => f113(p0, p1),
            Fn5::F117(k0) => f117((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F115,
    F165(std::rc::Rc<Adt12>),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F115 => f115(p0, p1),
            Fn6::F165(k0) => f165((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F119(List<Adt12>, List<Adt12>, std::rc::Rc<Adt12>),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F119(k0, k1, k2) => f119(k0.clone(), k1.clone(), (**k2).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F121(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F121(k0, k1) => f121(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F123(std::rc::Rc<Adt12>),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F123(k0) => f123((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F127(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F127(k0) => f127(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F130(std::rc::Rc<Adt12>),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F130(k0) => f130((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F134,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F134 => f134(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F154(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F154(k0) => f154(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F164(u64),
    F189(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F164(k0) => f164(*k0, p0, p1),
            Fn14::F189(k0) => f189(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F178,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F178 => f178(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F183(std::rc::Rc<Adt29>),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F183(k0) => f183((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F193(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F193(k0) => f193(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F197(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F197(k0) => f197(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F200,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F200 => f200(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let v1: List<Adt25> = List::cons(Adt25::C0(0u64, f1(Str::lit("nat_add")), 0u64, true, false), List::cons(Adt25::C0(1u64, f1(Str::lit("nat_sub")), 0u64, false, false), List::cons(Adt25::C0(2u64, f1(Str::lit("nat_mul")), 0u64, true, false), List::cons(Adt25::C0(3u64, f1(Str::lit("nat_quot")), 0u64, false, false), List::cons(Adt25::C0(4u64, f1(Str::lit("nat_rem")), 0u64, false, false), List::cons(Adt25::C0(5u64, f1(Str::lit("nat_eq")), 0u64, false, false), List::cons(Adt25::C0(6u64, f1(Str::lit("nat_le")), 0u64, false, false), List::cons(Adt25::C0(7u64, f1(Str::lit("nat_lt")), 0u64, false, false), List::cons(Adt25::C0(8u64, f1(Str::lit("nat_succ")), 0u64, true, false), List::cons(Adt25::C0(9u64, f1(Str::lit("int_add")), 0u64, true, false), List::<Adt25>::nil()))))))))));
    let v2: List<Adt25> = List::cons(Adt25::C0(10u64, f1(Str::lit("int_sub")), 0u64, true, false), List::cons(Adt25::C0(11u64, f1(Str::lit("int_mul")), 0u64, true, false), List::cons(Adt25::C0(12u64, f1(Str::lit("int_neg")), 0u64, true, false), List::cons(Adt25::C0(13u64, f1(Str::lit("int_quot")), 0u64, true, false), List::cons(Adt25::C0(14u64, f1(Str::lit("int_rem")), 0u64, false, false), List::cons(Adt25::C0(15u64, f1(Str::lit("bool_not")), 0u64, false, false), List::cons(Adt25::C0(16u64, f1(Str::lit("bool_and")), 0u64, false, false), List::cons(Adt25::C0(17u64, f1(Str::lit("bool_or")), 0u64, false, false), List::cons(Adt25::C0(18u64, f1(Str::lit("equal")), 0u64, false, false), List::cons(Adt25::C0(19u64, f1(Str::lit("compare")), 0u64, false, false), v1))))))))));
    let v3: List<Adt25> = List::cons(Adt25::C0(20u64, f1(Str::lit("checked_add")), 1u64, false, false), List::cons(Adt25::C0(21u64, f1(Str::lit("checked_sub")), 1u64, false, false), List::cons(Adt25::C0(22u64, f1(Str::lit("checked_mul")), 1u64, false, false), List::cons(Adt25::C0(23u64, f1(Str::lit("checked_quot")), 1u64, false, false), List::cons(Adt25::C0(24u64, f1(Str::lit("checked_neg_")), 2u64, false, false), List::cons(Adt25::C0(25u64, f1(Str::lit("bit_and")), 1u64, false, false), List::cons(Adt25::C0(26u64, f1(Str::lit("bit_or")), 1u64, false, false), List::cons(Adt25::C0(27u64, f1(Str::lit("bit_xor")), 1u64, false, false), List::cons(Adt25::C0(28u64, f1(Str::lit("bit_not")), 1u64, false, false), List::cons(Adt25::C0(29u64, f1(Str::lit("shift_left")), 1u64, false, false), v2))))))))));
    let v4: List<Adt25> = List::cons(Adt25::C0(30u64, f1(Str::lit("shift_right")), 1u64, false, false), List::cons(Adt25::C0(31u64, f1(Str::lit("convert")), 1u64, false, false), List::cons(Adt25::C0(32u64, f1(Str::lit("append_list")), 0u64, false, true), List::cons(Adt25::C0(33u64, f1(Str::lit("append_bytes")), 0u64, false, true), List::cons(Adt25::C0(34u64, f1(Str::lit("length_list")), 0u64, false, true), List::cons(Adt25::C0(35u64, f1(Str::lit("length_bytes")), 0u64, false, true), List::cons(Adt25::C0(36u64, f1(Str::lit("length_string")), 0u64, false, true), List::cons(Adt25::C0(37u64, f1(Str::lit("index_list")), 0u64, false, true), List::cons(Adt25::C0(38u64, f1(Str::lit("index_bytes")), 0u64, false, true), List::cons(Adt25::C0(39u64, f1(Str::lit("slice_list")), 0u64, false, true), v3))))))))));
    let v5: List<Adt25> = List::cons(Adt25::C0(40u64, f1(Str::lit("slice_bytes")), 0u64, false, true), List::cons(Adt25::C0(41u64, f1(Str::lit("utf8_encode")), 0u64, false, true), List::cons(Adt25::C0(42u64, f1(Str::lit("utf8_decode")), 0u64, false, true), List::cons(Adt25::C0(43u64, f1(Str::lit("compare_bytes")), 0u64, false, true), List::cons(Adt25::C0(44u64, f1(Str::lit("split_exact")), 0u64, false, true), List::cons(Adt25::C0(45u64, f1(Str::lit("join")), 0u64, false, true), List::cons(Adt25::C0(46u64, f1(Str::lit("format_int")), 0u64, false, true), List::cons(Adt25::C0(47u64, f1(Str::lit("format_")), 2u64, false, true), List::cons(Adt25::C0(48u64, f1(Str::lit("parse_int")), 0u64, true, true), List::cons(Adt25::C0(49u64, f1(Str::lit("parse_")), 2u64, false, true), List::cons(Adt25::C0(50u64, f1(Str::lit("update_bytes")), 0u64, false, true), v4)))))))))));
    let m2 = f2(Adt26::C0(List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, bool)>::nil(), List::<(u64, bool)>::nil()), match v0.clone() {
        Adt0::C0(_, h1) => {
            (*h1).clone()
        }
    })?;
    match m2 {
        Ok(v6) => {
            f3(match v0.clone() {
                Adt0::C0(h3, _) => {
                    h3
                }
            }, v6, v5, match v0 {
                Adt0::C0(_, h4) => {
                    (*h4).clone()
                }
            })
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a5 = v0;
    utf8_encode(a5)
}

pub fn f2(p42: Adt26, p43: Adt1) -> R<Result<Adt26, Bytes>> {
    let m41 = (p42, p43);
    {
        let mut m41 = m41;
        loop {
            let step = {
                let (v0, v1) = m41;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C4(r39, r40) => {
                        let v16 = (*r39).clone();
                        let v17 = (*r40).clone();
                        let m7 = v16;
                        match m7 {
                            Adt1::C0(r35, r36, _) => {
                                let v18 = (*r35).clone();
                                let v19 = (*r36).clone();
                                let m8 = v18.clone();
                                match m8 {
                                    Adt2::C0(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C1(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C2(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C3(_, _, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C4(v28, v29) => {
                                        let m9 = f6(v28);
                                        if m9 {
                                            let m11 = f7(match v0.clone() {
                                                Adt26::C0(_, h10, _, _) => {
                                                    h10
                                                }
                                            }, v29);
                                            if m11 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                    Adt26::C0(h12, _, _, _) => {
                                                        h12
                                                    }
                                                }, List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(_, h13, _, _) => {
                                                        h13
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, _, h14, _) => {
                                                        h14
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h15) => {
                                                        h15
                                                    }
                                                }), v17))
                                            }
                                        } else {
                                            let m17 = f7(match v0.clone() {
                                                Adt26::C0(h16, _, _, _) => {
                                                    h16
                                                }
                                            }, v29);
                                            if m17 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(h18, _, _, _) => {
                                                        h18
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, h19, _, _) => {
                                                        h19
                                                    }
                                                }, match v0.clone() {
                                                    Adt26::C0(_, _, h20, _) => {
                                                        h20
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h21) => {
                                                        h21
                                                    }
                                                }), v17))
                                            }
                                        }
                                    }
                                    Adt2::C5 => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C6(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                }
                            }
                            Adt1::C1(v32, _, _, v35, _, _) => {
                                let m23 = f9(match v0.clone() {
                                    Adt26::C0(_, _, h22, _) => {
                                        h22
                                    }
                                }, v32);
                                if m23 {
                                    Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: Fn")), f4(f10(v32)?, f1(Str::lit("::apply is declared twice")))))))
                                } else {
                                    Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                        Adt26::C0(h24, _, _, _) => {
                                            h24
                                        }
                                    }, match v0.clone() {
                                        Adt26::C0(_, h25, _, _) => {
                                            h25
                                        }
                                    }, List::cons((v32, v35), match v0.clone() {
                                        Adt26::C0(_, _, h26, _) => {
                                            h26
                                        }
                                    }), match v0 {
                                        Adt26::C0(_, _, _, h27) => {
                                            h27
                                        }
                                    }), v17))
                                }
                            }
                            Adt1::C2(r37, _, r38, _, _) => {
                                let v38 = (*r37).clone();
                                let v40 = (*r38).clone();
                                let m28 = v38;
                                match m28 {
                                    Adt12::C0(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C1(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C2(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C3(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C4(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C5(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C6(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C7(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C8(v51) => {
                                        let m30 = f9(match v0.clone() {
                                            Adt26::C0(_, _, _, h29) => {
                                                h29
                                            }
                                        }, v51);
                                        if m30 {
                                            Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: f")), f4(f10(v51)?, f1(Str::lit(" is declared twice")))))))
                                        } else {
                                            Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                Adt26::C0(h31, _, _, _) => {
                                                    h31
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, h32, _, _) => {
                                                    h32
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, _, h33, _) => {
                                                    h33
                                                }
                                            }, List::cons((v51, f11(v40)), match v0 {
                                                Adt26::C0(_, _, _, h34) => {
                                                    h34
                                                }
                                            })), v17))
                                        }
                                    }
                                    Adt12::C9(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                }
                            }
                            Adt1::C3 => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                            Adt1::C4(_, _) => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m41 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt26, v2: List<Adt25>, p49: Adt1) -> R<Result<(), Bytes>> {
    let m48 = p49;
    {
        let mut m48 = m48;
        loop {
            let step = {
                let v3 = m48;
                let m44 = v3;
                match m44 {
                    Adt1::C0(_, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C3 => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C4(r46, r47) => {
                        let v18 = (*r46).clone();
                        let v19 = (*r47).clone();
                        let m45 = f12(v0, v1.clone(), v2.clone(), v18)?;
                        match m45 {
                            Ok(_) => {
                                Ok::<Adt1, R<Result<(), Bytes>>>(v19)
                            }
                            Err(v21) => {
                                Err::<Adt1, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v21)))
                            }
                        }
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

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a50 = v0;
    let a51 = v1;
    append_bytes(a50, a51)
}

pub fn f5(v0: Adt2) -> R<Bytes> {
    let m52 = v0;
    match m52 {
        Adt2::C0(v1) => {
            Ok(f13(v1))
        }
        Adt2::C1(v2) => {
            Ok(f4(f1(Str::lit("Fixed(")), f4(f14(v2), f1(Str::lit(")")))))
        }
        Adt2::C2(v3, r53) => {
            let v4 = (*r53).clone();
            Ok(f4(f15(v3), f4(f1(Str::lit("(")), f4(f5(v4)?, f1(Str::lit(")"))))))
        }
        Adt2::C3(v5, r54, r55) => {
            let v6 = (*r54).clone();
            let v7 = (*r55).clone();
            Ok(f4(f16(v5), f4(f1(Str::lit("(")), f4(f5(v6)?, f4(f1(Str::lit(", ")), f4(f5(v7)?, f1(Str::lit(")"))))))))
        }
        Adt2::C4(v8, v9) => {
            Ok(f4(f17(v8), f4(f1(Str::lit("(")), f4(f10(v9)?, f1(Str::lit(")"))))))
        }
        Adt2::C5 => {
            Ok(f1(Str::lit("unreachable")))
        }
        Adt2::C6(_, _) => {
            Ok(f1(Str::lit("unreachable")))
        }
    }
}

pub fn f6(v0: Adt7) -> bool {
    let m56 = v0;
    match m56 {
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

pub fn f7(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> bool {
    let m57 = f18(v0, v1);
    match m57 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(p62: List<(u64, u64)>, p63: Adt8) -> R<List<(u64, u64)>> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v0, v1) = m61;
                let m58 = v1;
                match m58 {
                    Adt8::C0 => {
                        Err::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>(Ok(v0))
                    }
                    Adt8::C1(v2, r59, r60) => {
                        let v3 = (*r59).clone();
                        let v4 = (*r60).clone();
                        Ok::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>((List::cons((v2, f19(v3)?), v0), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<(u64, bool)>, v1: u64) -> bool {
    let m64 = f20(v0, v1);
    match m64 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f10(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f21({
        let a101 = f22(v0, 10000000000000000000u64);
        let a102 = {
            let a99 = f22(v0, 1000000000000000000u64);
            let a100 = {
                let a97 = f22(v0, 100000000000000000u64);
                let a98 = {
                    let a95 = f22(v0, 10000000000000000u64);
                    let a96 = {
                        let a93 = f22(v0, 1000000000000000u64);
                        let a94 = {
                            let a91 = f22(v0, 100000000000000u64);
                            let a92 = {
                                let a89 = f22(v0, 10000000000000u64);
                                let a90 = {
                                    let a87 = f22(v0, 1000000000000u64);
                                    let a88 = {
                                        let a85 = f22(v0, 100000000000u64);
                                        let a86 = {
                                            let a83 = f22(v0, 10000000000u64);
                                            let a84 = {
                                                let a81 = f22(v0, 1000000000u64);
                                                let a82 = {
                                                    let a79 = f22(v0, 100000000u64);
                                                    let a80 = {
                                                        let a77 = f22(v0, 10000000u64);
                                                        let a78 = {
                                                            let a75 = f22(v0, 1000000u64);
                                                            let a76 = {
                                                                let a73 = f22(v0, 100000u64);
                                                                let a74 = {
                                                                    let a71 = f22(v0, 10000u64);
                                                                    let a72 = {
                                                                        let a69 = f22(v0, 1000u64);
                                                                        let a70 = {
                                                                            let a67 = f22(v0, 100u64);
                                                                            let a68 = {
                                                                                let a65 = f22(v0, 10u64);
                                                                                let a66 = f22(v0, 1u64);
                                                                                append_bytes(a65, a66)
                                                                            };
                                                                            append_bytes(a67, a68)
                                                                        };
                                                                        append_bytes(a69, a70)
                                                                    };
                                                                    append_bytes(a71, a72)
                                                                };
                                                                append_bytes(a73, a74)
                                                            };
                                                            append_bytes(a75, a76)
                                                        };
                                                        append_bytes(a77, a78)
                                                    };
                                                    append_bytes(a79, a80)
                                                };
                                                append_bytes(a81, a82)
                                            };
                                            append_bytes(a83, a84)
                                        };
                                        append_bytes(a85, a86)
                                    };
                                    append_bytes(a87, a88)
                                };
                                append_bytes(a89, a90)
                            };
                            append_bytes(a91, a92)
                        };
                        append_bytes(a93, a94)
                    };
                    append_bytes(a95, a96)
                };
                append_bytes(a97, a98)
            };
            append_bytes(a99, a100)
        };
        append_bytes(a101, a102)
    })?;
    let a103 = v0;
    let a104 = 0u64;
    if nat_eq(a103, a104) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f11(v0: Adt2) -> bool {
    let m105 = v0;
    match m105 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, _) => {
            f23(v3)
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

pub fn f12(v0: bool, v1: Adt26, v2: List<Adt25>, v3: Adt1) -> R<Result<(), Bytes>> {
    let m106 = v3;
    match m106 {
        Adt1::C0(_, r116, _) => {
            let v5 = (*r116).clone();
            f24(Adt27::C0(v0, std::rc::Rc::new(v1), false, v2, false), v5)
        }
        Adt1::C1(v7, r117, r118, v10, r119, _) => {
            let v8 = (*r117).clone();
            let v9 = (*r118).clone();
            let v11 = (*r119).clone();
            let v13: Adt27 = Adt27::C0(v0, std::rc::Rc::new(v1), false, v2, false);
            let m107 = f25(v13.clone(), v8)?;
            match m107 {
                Ok(_) => {
                    let m108 = f26(v13.clone(), v9)?;
                    match m108 {
                        Ok(_) => {
                            f27(v13, v7, v10, v11)
                        }
                        Err(v16) => {
                            Ok(Err::<(), Bytes>(v16))
                        }
                    }
                }
                Err(v17) => {
                    Ok(Err::<(), Bytes>(v17))
                }
            }
        }
        Adt1::C2(r120, r121, r122, r123, _) => {
            let v18 = (*r120).clone();
            let v19 = (*r121).clone();
            let v20 = (*r122).clone();
            let v21 = (*r123).clone();
            let v23: bool = f11(v20.clone());
            let v24: Adt27 = Adt27::C0(v0, std::rc::Rc::new(v1.clone()), v23, v2, false);
            let m109 = f26(v24.clone(), v20)?;
            match m109 {
                Ok(_) => {
                    let m110 = f28(v24.clone(), Adt28::C0(List::<Adt12>::nil(), List::<Adt12>::nil(), List::<Adt12>::nil(), List::<(Adt12, List<u64>)>::nil(), false), v19.clone())?;
                    match m110 {
                        Ok(v26) => {
                            let m111 = f29(v26, v24, v21.clone(), true)?;
                            match m111 {
                                Ok(v27) => {
                                    let m114 = if match v27 {
                                        Adt28::C0(_, _, _, _, h112) => {
                                            h112
                                        }
                                    } {
                                        let a113 = v23;
                                        bool_not(a113)
                                    } else {
                                        false
                                    };
                                    if m114 {
                                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: ")), f4(f30(f31(v18)?), f1(Str::lit(" propagates a failure its result type does not carry"))))))
                                    } else {
                                        let m115 = f32(f33(List::<(Adt12, u64)>::nil(), v19), v1, v21, v23)?;
                                        match m115 {
                                            Ok(_) => {
                                                Ok(Ok::<(), Bytes>(()))
                                            }
                                            Err(v29) => {
                                                Ok(Err::<(), Bytes>(v29))
                                            }
                                        }
                                    }
                                }
                                Err(v30) => {
                                    Ok(Err::<(), Bytes>(v30))
                                }
                            }
                        }
                        Err(v31) => {
                            Ok(Err::<(), Bytes>(v31))
                        }
                    }
                }
                Err(v32) => {
                    Ok(Err::<(), Bytes>(v32))
                }
            }
        }
        Adt1::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt1::C4(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f13(v0: Adt3) -> Bytes {
    let m124 = v0;
    match m124 {
        Adt3::C0 => {
            f1(Str::lit("Unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("Bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("Nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("Int"))
        }
        Adt3::C4 => {
            f1(Str::lit("Ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("Str"))
        }
        Adt3::C6 => {
            f1(Str::lit("Bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("Overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("I128"))
        }
    }
}

pub fn f14(v0: Adt4) -> Bytes {
    let m125 = v0;
    match m125 {
        Adt4::C0 => {
            f1(Str::lit("U8"))
        }
        Adt4::C1 => {
            f1(Str::lit("U16"))
        }
        Adt4::C2 => {
            f1(Str::lit("U32"))
        }
        Adt4::C3 => {
            f1(Str::lit("U64"))
        }
        Adt4::C4 => {
            f1(Str::lit("I8"))
        }
        Adt4::C5 => {
            f1(Str::lit("I16"))
        }
        Adt4::C6 => {
            f1(Str::lit("I32"))
        }
        Adt4::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f15(v0: Adt5) -> Bytes {
    let m126 = v0;
    match m126 {
        Adt5::C0 => {
            f1(Str::lit("Option"))
        }
        Adt5::C1 => {
            f1(Str::lit("List"))
        }
        Adt5::C2 => {
            f1(Str::lit("Rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("Fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("Ref"))
        }
    }
}

pub fn f16(v0: Adt6) -> Bytes {
    let m127 = v0;
    match m127 {
        Adt6::C0 => {
            f1(Str::lit("Result"))
        }
        Adt6::C1 => {
            f1(Str::lit("Pair"))
        }
    }
}

pub fn f17(v0: Adt7) -> Bytes {
    let m128 = v0;
    match m128 {
        Adt7::C0 => {
            f1(Str::lit("Adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("Fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("Param"))
        }
    }
}

pub fn f18(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> Option<List<(u64, u64)>> {
    f34(Fn0::F35(v1), None::<List<(u64, u64)>>, v0)
}

pub fn f19(v0: Adt2) -> R<u64> {
    let m129 = v0;
    match m129 {
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
        Adt2::C6(_, r132) => {
            let v11 = (*r132).clone();
            let v12: u64 = f19(v11)?;
            let a130 = 1u64;
            let a131 = v12;
            nat_add(a130, a131)
        }
    }
}

pub fn f20(v0: List<(u64, bool)>, v1: u64) -> Option<bool> {
    f36(Fn1::F37(v1), None::<bool>, v0)
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h133, _) = f38(Fn2::F39(v0.clone()), {
            let a134 = v0.clone();
            length_bytes(a134)
        }, 0u64)?;
        h133
    };
    Ok(f40(v0.clone(), v1, {
        let a136 = {
            let a135 = v0;
            length_bytes(a135)
        };
        let a137 = v1;
        nat_sub(a136, a137)
    }))
}

pub fn f22(v0: u64, v1: u64) -> Bytes {
    f40(f1(Str::lit("0123456789")), {
        let a141 = {
            let a138 = v0;
            let a139 = v1;
            let a140 = 0u64;
            nat_quot(a138, a139, a140)
        };
        let a142 = 10u64;
        let a143 = 0u64;
        nat_rem(a141, a142, a143)
    }, 1u64)
}

pub fn f23(v0: Adt5) -> bool {
    let m144 = v0;
    match m144 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            true
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f24(v0: Adt27, p150: Adt8) -> R<Result<(), Bytes>> {
    let m149 = p150;
    {
        let mut m149 = m149;
        loop {
            let step = {
                let v1 = m149;
                let m145 = v1;
                match m145 {
                    Adt8::C0 => {
                        Err::<Adt8, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt8::C1(_, r147, r148) => {
                        let v3 = (*r147).clone();
                        let v4 = (*r148).clone();
                        let m146 = f25(v0.clone(), v3)?;
                        match m146 {
                            Ok(_) => {
                                Ok::<Adt8, R<Result<(), Bytes>>>(v4)
                            }
                            Err(v6) => {
                                Err::<Adt8, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt27, p156: Adt2) -> R<Result<(), Bytes>> {
    let m155 = p156;
    {
        let mut m155 = m155;
        loop {
            let step = {
                let v1 = m155;
                let m151 = v1;
                match m151 {
                    Adt2::C0(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5 => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(r153, r154) => {
                        let v11 = (*r153).clone();
                        let v12 = (*r154).clone();
                        let m152 = f26(v0.clone(), v11)?;
                        match m152 {
                            Ok(_) => {
                                Ok::<Adt2, R<Result<(), Bytes>>>(v12)
                            }
                            Err(v14) => {
                                Err::<Adt2, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v14)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let v2: bool = f41(v1.clone());
    let v3: bool = if match v0.clone() {
        Adt27::C0(h157, _, _, _, _) => {
            h157
        }
    } {
        v2
    } else {
        false
    };
    let m158 = v3;
    if m158 {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders the heap type ")), f4(f5(v1)?, f1(Str::lit(", which rust-core does not provide"))))))
    } else {
        f42(v0, v1)
    }
}

pub fn f27(v0: Adt27, v1: u64, v2: bool, p170: Adt10) -> R<Result<(), Bytes>> {
    let m169 = p170;
    {
        let mut m169 = m169;
        loop {
            let step = {
                let v3 = m169;
                let m159 = v3;
                match m159 {
                    Adt10::C0 => {
                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt10::C1(v4, v5, v6, r168) => {
                        let v7 = (*r168).clone();
                        let m163 = f20({
                            let m162 = match v0.clone() {
                                Adt27::C0(_, h161, _, _, _) => {
                                    (*h161).clone()
                                }
                            };
                            match m162 {
                                Adt26::C0(_, _, _, h160) => {
                                    h160
                                }
                            }
                        }, v4);
                        match m163 {
                            None => {
                                Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v1)?, f4(f1(Str::lit(" dispatches to undeclared f")), f10(v4)?))))))
                            }
                            Some(v8) => {
                                let v9: bool = if v8 {
                                    !v6
                                } else {
                                    v6
                                };
                                let v10: bool = if v6 {
                                    !v2
                                } else {
                                    false
                                };
                                let m164 = v9;
                                if m164 {
                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                } else {
                                    let m165 = v10;
                                    if m165 {
                                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                    } else {
                                        let m166 = f43(v5);
                                        if m166 {
                                            let m167 = f44(v0.clone(), f1(Str::lit("a boxed capture")));
                                            match m167 {
                                                Ok(_) => {
                                                    Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                                }
                                                Err(v12) => {
                                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v12)))
                                                }
                                            }
                                        } else {
                                            Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt27, p179: Adt28, p180: Adt13) -> R<Result<Adt28, Bytes>> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v1, v2) = m178;
                let m171 = v2;
                match m171 {
                    Adt13::C0 => {
                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v1)))
                    }
                    Adt13::C1(r175, r176, r177) => {
                        let v3 = (*r175).clone();
                        let v4 = (*r176).clone();
                        let v5 = (*r177).clone();
                        let m172 = f26(v0.clone(), v4)?;
                        match m172 {
                            Ok(_) => {
                                let m174 = f45(v1, match v0.clone() {
                                    Adt27::C0(_, h173, _, _, _) => {
                                        (*h173).clone()
                                    }
                                }, v3)?;
                                match m174 {
                                    Ok(v7) => {
                                        Ok::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>((v7, v5))
                                    }
                                    Err(v8) => {
                                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v8)))
                                    }
                                }
                            }
                            Err(v9) => {
                                Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v9)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m181 = v2;
    match m181 {
        Adt17::C0(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C11(r189, r190) => {
            let v37 = (*r189).clone();
            let v38 = (*r190).clone();
            let v39: List<Adt12> = match v0.clone() {
                Adt28::C0(_, h182, _, _, _) => {
                    h182
                }
            };
            let m183 = f46(v0, v1.clone(), v37)?;
            match m183 {
                Ok(v40) => {
                    let m184 = f47(v40, v1, v38, v3)?;
                    match m184 {
                        Ok(v41) => {
                            Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v41.clone() {
                                Adt28::C0(h185, _, _, _, _) => {
                                    h185
                                }
                            }, v39, match v41.clone() {
                                Adt28::C0(_, _, h186, _, _) => {
                                    h186
                                }
                            }, match v41.clone() {
                                Adt28::C0(_, _, _, h187, _) => {
                                    h187
                                }
                            }, match v41 {
                                Adt28::C0(_, _, _, _, h188) => {
                                    h188
                                }
                            })))
                        }
                        Err(v42) => {
                            Ok(Err::<Adt28, Bytes>(v42))
                        }
                    }
                }
                Err(v43) => {
                    Ok(Err::<Adt28, Bytes>(v43))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
    }
}

pub fn f30(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f31(v0: Adt12) -> R<Bytes> {
    let m191 = v0;
    match m191 {
        Adt12::C0(v1) => {
            Ok(f4(f1(Str::lit("v")), f10(v1)?))
        }
        Adt12::C1(v2) => {
            Ok(f4(f1(Str::lit("m")), f10(v2)?))
        }
        Adt12::C2(v3) => {
            Ok(f4(f1(Str::lit("a")), f10(v3)?))
        }
        Adt12::C3(v4) => {
            Ok(f4(f1(Str::lit("c")), f10(v4)?))
        }
        Adt12::C4(v5) => {
            Ok(f4(f1(Str::lit("r")), f10(v5)?))
        }
        Adt12::C5(v6) => {
            Ok(f4(f1(Str::lit("h")), f10(v6)?))
        }
        Adt12::C6(v7) => {
            Ok(f4(f1(Str::lit("k")), f10(v7)?))
        }
        Adt12::C7(v8) => {
            Ok(f4(f1(Str::lit("p")), f10(v8)?))
        }
        Adt12::C8(v9) => {
            Ok(f4(f1(Str::lit("f")), f10(v9)?))
        }
        Adt12::C9(v10) => {
            Ok(v10)
        }
    }
}

pub fn f32(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m192 = v2;
    match m192 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(r194, r195) => {
            let v37 = (*r194).clone();
            let v38 = (*r195).clone();
            let m193 = f48(v0, v1.clone(), v37)?;
            match m193 {
                Ok(v39) => {
                    f49(v39, v1, v38, v3)
                }
                Err(v40) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v40))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
    }
}

pub fn f33(p201: List<(Adt12, u64)>, p202: Adt13) -> List<(Adt12, u64)> {
    let m200 = (p201, p202);
    {
        let mut m200 = m200;
        loop {
            let step = {
                let (v0, v1) = m200;
                let m196 = v1;
                match m196 {
                    Adt13::C0 => {
                        Err::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>(v0)
                    }
                    Adt13::C1(r197, r198, r199) => {
                        let v2 = (*r197).clone();
                        let v3 = (*r198).clone();
                        let v4 = (*r199).clone();
                        Ok::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>((f50(v0, v2, v3), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Fn0, p206: Option<List<(u64, u64)>>, p207: List<(u64, List<(u64, u64)>)>) -> Option<List<(u64, u64)>> {
    let m205 = (p206, p207);
    {
        let mut m205 = m205;
        loop {
            let step = {
                let (v1, v2) = m205;
                let m203 = v2;
                match m203.uncons() {
                    None => {
                        Err::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(({
                            let c204: Fn0 = v0.clone();
                            c204.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Option<List<(u64, u64)>>, v2: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
    let m208 = v1;
    match m208 {
        None => {
            let a210 = {
                let (h209, _) = v2;
                h209
            };
            let a211 = v0;
            if nat_eq(a210, a211) {
                Some({
                    let (_, h212) = v2;
                    h212
                })
            } else {
                None::<List<(u64, u64)>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f36(v0: Fn1, p216: Option<bool>, p217: List<(u64, bool)>) -> Option<bool> {
    let m215 = (p216, p217);
    {
        let mut m215 = m215;
        loop {
            let step = {
                let (v1, v2) = m215;
                let m213 = v2;
                match m213.uncons() {
                    None => {
                        Err::<(Option<bool>, List<(u64, bool)>), Option<bool>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<bool>, List<(u64, bool)>), Option<bool>>(({
                            let c214: Fn1 = v0.clone();
                            c214.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: u64, v1: Option<bool>, v2: (u64, bool)) -> Option<bool> {
    let m218 = v1;
    match m218 {
        None => {
            let a220 = {
                let (h219, _) = v2;
                h219
            };
            let a221 = v0;
            if nat_eq(a220, a221) {
                Some({
                    let (_, h222) = v2;
                    h222
                })
            } else {
                None::<bool>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f38(v0: Fn2, p227: u64, p228: u64) -> R<(u64, bool)> {
    let m226 = (p227, p228);
    {
        let mut m226 = m226;
        loop {
            let step = {
                let (v1, v2) = m226;
                let m223 = v1;
                if m223 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m223 - 1;
                    let m225 = {
                        let c224: Fn2 = v0.clone();
                        c224.apply(v2)?
                    };
                    match m225 {
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
                Ok(next) => { m226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f51(v0, v1)
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m232 = {
        let a229 = v0;
        let a230 = v1;
        let a231 = v2;
        slice_bytes(a229, a230, a231)
    };
    match m232 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt2) -> bool {
    let m233 = v0;
    match m233 {
        Adt2::C0(v1) => {
            f52(v1)
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, r235) => {
            let v4 = (*r235).clone();
            let m234 = f53(v3);
            if m234 {
                true
            } else {
                f41(v4)
            }
        }
        Adt2::C3(_, r236, r237) => {
            let v6 = (*r236).clone();
            let v7 = (*r237).clone();
            let v8: bool = f41(v6);
            let v9: bool = f41(v7);
            if v8 {
                true
            } else {
                v9
            }
        }
        Adt2::C4(v10, _) => {
            f54(v10)
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f42(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let m238 = v1;
    match m238 {
        Adt2::C0(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C1(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C2(_, r250) => {
            let v5 = (*r250).clone();
            f42(v0, v5)
        }
        Adt2::C3(_, r251, r252) => {
            let v7 = (*r251).clone();
            let v8 = (*r252).clone();
            let m239 = f42(v0.clone(), v7)?;
            match m239 {
                Ok(_) => {
                    f42(v0, v8)
                }
                Err(v10) => {
                    Ok(Err::<(), Bytes>(v10))
                }
            }
        }
        Adt2::C4(v11, v12) => {
            let m240 = f54(v11.clone());
            if m240 {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let m241 = f6(v11);
                if m241 {
                    let m245 = f7({
                        let m244 = match v0 {
                            Adt27::C0(_, h243, _, _, _) => {
                                (*h243).clone()
                            }
                        };
                        match m244 {
                            Adt26::C0(_, h242, _, _) => {
                                h242
                            }
                        }
                    }, v12);
                    if m245 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Fn")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                } else {
                    let m249 = f7({
                        let m248 = match v0 {
                            Adt27::C0(_, h247, _, _, _) => {
                                (*h247).clone()
                            }
                        };
                        match m248 {
                            Adt26::C0(h246, _, _, _) => {
                                h246
                            }
                        }
                    }, v12);
                    if m249 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Adt")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C6(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f43(v0: List<Adt11>) -> bool {
    f55(Fn3::F56, false, v0)
}

pub fn f44(v0: Adt27, v1: Bytes) -> Result<(), Bytes> {
    let m254 = match v0 {
        Adt27::C0(h253, _, _, _, _) => {
            h253
        }
    };
    if m254 {
        Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v1, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<(), Bytes>(())
    }
}

pub fn f45(v0: Adt28, v1: Adt26, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m255 = v2;
    match m255 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C1(r268) => {
            let v4 = (*r268).clone();
            let m257 = f57(match v0.clone() {
                Adt28::C0(h256, _, _, _, _) => {
                    h256
                }
            }, v4.clone());
            if m257 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f30(f31(v4)?), f1(Str::lit(" is bound twice in one function"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(v4.clone(), match v0.clone() {
                    Adt28::C0(h258, _, _, _, _) => {
                        h258
                    }
                }), List::cons(v4, match v0.clone() {
                    Adt28::C0(_, h259, _, _, _) => {
                        h259
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, h260, _, _) => {
                        h260
                    }
                }, match v0.clone() {
                    Adt28::C0(_, _, _, h261, _) => {
                        h261
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h262) => {
                        h262
                    }
                })))
            }
        }
        Adt14::C2(r269) => {
            let v5 = (*r269).clone();
            f45(v0, v1, v5)
        }
        Adt14::C3(_, r270) => {
            let v7 = (*r270).clone();
            f45(v0, v1, v7)
        }
        Adt14::C4(v8, v9, r271) => {
            let v10 = (*r271).clone();
            let m263 = f58(v1.clone(), v8, v9);
            match m263 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v11) => {
                    let m266 = {
                        let a264 = v11;
                        let a265 = f59(v10.clone())?;
                        nat_eq(a264, a265)
                    };
                    if m266 {
                        f45(v0, v1, v10)
                    } else {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f4(f1(Str::lit(" binds ")), f4(f10(f59(v10)?)?, f4(f1(Str::lit(" fields of ")), f10(v11)?)))))))))
                    }
                }
            }
        }
        Adt14::C5 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C6(r272, r273) => {
            let v12 = (*r272).clone();
            let v13 = (*r273).clone();
            let m267 = f45(v0, v1.clone(), v12)?;
            match m267 {
                Ok(v14) => {
                    f45(v14, v1, v13)
                }
                Err(v15) => {
                    Ok(Err::<Adt28, Bytes>(v15))
                }
            }
        }
    }
}

pub fn f46(p288: Adt28, v1: Adt27, p289: Adt17) -> R<Result<Adt28, Bytes>> {
    let m287 = (p288, p289);
    {
        let mut m287 = m287;
        loop {
            let step = {
                let (v0, v2) = m287;
                let m274 = v2;
                match m274 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C14(r283, v41, r284, r285, _, r286) => {
                        let v40 = (*r283).clone();
                        let v42 = (*r284).clone();
                        let v43 = (*r285).clone();
                        let v45 = (*r286).clone();
                        let m276 = {
                            let m275 = v41;
                            if m275 {
                                f26(v1.clone(), v42)?
                            } else {
                                Ok::<(), Bytes>(())
                            }
                        };
                        match m276 {
                            Ok(_) => {
                                let m280 = {
                                    let m277 = f60(v40.clone(), v43.clone());
                                    match m277 {
                                        None => {
                                            f47(v0, v1.clone(), v43, false)?
                                        }
                                        Some(v47) => {
                                            let m278 = v40.clone();
                                            match m278 {
                                                Adt14::C0(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C1(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C2(r279) => {
                                                    let v50 = (*r279).clone();
                                                    f61(v0, v47, v50)?
                                                }
                                                Adt14::C3(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C4(_, _, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C5 => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C6(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                            }
                                        }
                                    }
                                };
                                match m280 {
                                    Ok(v58) => {
                                        let m282 = f45(v58, match v1.clone() {
                                            Adt27::C0(_, h281, _, _, _) => {
                                                (*h281).clone()
                                            }
                                        }, v40)?;
                                        match m282 {
                                            Ok(v59) => {
                                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v59, v45))
                                            }
                                            Err(v60) => {
                                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v60)))
                                            }
                                        }
                                    }
                                    Err(v61) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v61)))
                                    }
                                }
                            }
                            Err(v62) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v62)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m290 = v2;
    match m290 {
        Adt17::C0(r467, r468) => {
            let v4 = (*r467).clone();
            let v5 = (*r468).clone();
            let v6: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h291) => {
                        h291
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h292, _, _) => {
                            h292
                        }
                    }
                }
            } else {
                false
            };
            let m293 = v6;
            if m293 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h294, _) => {
                        h294
                    }
                }, Adt17::C0(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m295 = v4;
                match m295 {
                    Adt18::C0 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C1 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C2 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C3 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C4(_) => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C5 => {
                        Ok(f63(v1, v0, f1(Str::lit("a string literal"))))
                    }
                    Adt18::C6 => {
                        Ok(f63(v1, v0, f1(Str::lit("a byte string literal"))))
                    }
                    Adt18::C7 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C8 => {
                        Ok(Err::<Adt28, Bytes>(f1(Str::lit("overflow: the overflow value stands outside the payload of an `Err`"))))
                    }
                }
            }
        }
        Adt17::C1(v8, r469, r470) => {
            let v9 = (*r469).clone();
            let v10 = (*r470).clone();
            let v11: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h296) => {
                        h296
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h297, _, _) => {
                            h297
                        }
                    }
                }
            } else {
                false
            };
            let m298 = v11;
            if m298 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h299, _) => {
                        h299
                    }
                }, Adt17::C1(v8, std::rc::Rc::new(v9), std::rc::Rc::new(v10)))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m300 = v8;
                match m300 {
                    Adt19::C0 => {
                        f64(v0, v9)
                    }
                    Adt19::C1 => {
                        let m301 = f65(v0.clone(), v9)?;
                        match m301 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                    Adt19::C2 => {
                        let m302 = f65(v0.clone(), v9)?;
                        match m302 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v15) => {
                                Ok(Err::<Adt28, Bytes>(v15))
                            }
                        }
                    }
                    Adt19::C3 => {
                        let m303 = f65(v0.clone(), v9)?;
                        match m303 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v17) => {
                                Ok(Err::<Adt28, Bytes>(v17))
                            }
                        }
                    }
                    Adt19::C4 => {
                        let m304 = f44(v1, f1(Str::lit("a boxed read")));
                        match m304 {
                            Ok(_) => {
                                let m305 = f65(v0.clone(), v9)?;
                                match m305 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt28, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Err::<Adt28, Bytes>(v21))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C2(v22, r471, r472) => {
            let v23 = (*r471).clone();
            let v24 = (*r472).clone();
            let m306 = v22.clone();
            match m306 {
                Adt20::C0 => {
                    let v25: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h307) => {
                                h307
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h308, _, _) => {
                                    h308
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m309 = v25;
                    if m309 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h310, _) => {
                                h310
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), std::rc::Rc::new(v24)))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m311 = f66(v23.clone());
                        if m311 {
                            Ok(Err::<Adt28, Bytes>(f1(Str::lit("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean"))))
                        } else {
                            f47(v0, v1, v23, false)
                        }
                    }
                }
                Adt20::C1 => {
                    let v26: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h312) => {
                                h312
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h313, _, _) => {
                                    h313
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m314 = v26;
                    if m314 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h315, _) => {
                                h315
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), std::rc::Rc::new(v24)))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m316 = f44(v1.clone(), f1(Str::lit("a box")));
                        match m316 {
                            Ok(_) => {
                                f47(v0, v1, v23, false)
                            }
                            Err(v28) => {
                                Ok(Err::<Adt28, Bytes>(v28))
                            }
                        }
                    }
                }
                Adt20::C2 => {
                    let v29: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h317) => {
                                h317
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h318, _, _) => {
                                    h318
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m319 = v29;
                    if m319 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h320, _) => {
                                h320
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), std::rc::Rc::new(v24)))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        f47(v0, v1, v23, false)
                    }
                }
                Adt20::C3 => {
                    f47(v0, v1, v23, false)
                }
            }
        }
        Adt17::C3(r473, r474, v32, r475) => {
            let v30 = (*r473).clone();
            let v31 = (*r474).clone();
            let v33 = (*r475).clone();
            let m321 = v32;
            if m321 {
                let v34: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h322) => {
                            h322
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h323, _, _) => {
                                h323
                            }
                        }
                    }
                } else {
                    false
                };
                let m324 = v34;
                if m324 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h325, _) => {
                            h325
                        }
                    }, Adt17::C3(std::rc::Rc::new(v30), std::rc::Rc::new(v31), v32, std::rc::Rc::new(v33)))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m326 = f67(v0, v1.clone(), false, v31.clone())?;
                    match m326 {
                        Ok(v35) => {
                            let m327 = v30;
                            match m327 {
                                Adt21::C0(v36) => {
                                    let m331 = f20({
                                        let m330 = match v1.clone() {
                                            Adt27::C0(_, h329, _, _, _) => {
                                                (*h329).clone()
                                            }
                                        };
                                        match m330 {
                                            Adt26::C0(_, _, _, h328) => {
                                                h328
                                            }
                                        }
                                    }, v36);
                                    match m331 {
                                        None => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v36)?, f1(Str::lit(" is not declared"))))))
                                        }
                                        Some(v37) => {
                                            Ok(f68(v35, v1.clone(), v37, v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h332) => {
                                                    h332
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f4(f1(Str::lit("f")), f10(v36)?)))
                                        }
                                    }
                                }
                                Adt21::C1(r340) => {
                                    let v39 = (*r340).clone();
                                    let v40: Bytes = f70(match v1.clone() {
                                        Adt27::C0(_, _, _, h334, _) => {
                                            h334
                                        }
                                    }, v39.clone());
                                    let m337 = {
                                        let m336 = f71(match v1.clone() {
                                            Adt27::C0(_, _, _, h335, _) => {
                                                h335
                                            }
                                        }, v39.clone());
                                        if m336 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v40.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m337 {
                                        Ok(_) => {
                                            Ok(f68(v35, v1.clone(), f72(match v1.clone() {
                                                Adt27::C0(_, _, _, h338, _) => {
                                                    h338
                                                }
                                            }, v39), v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h339) => {
                                                    h339
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f30(v40)))
                                        }
                                        Err(v42) => {
                                            Ok(Err::<Adt28, Bytes>(v42))
                                        }
                                    }
                                }
                                Adt21::C2(v38) => {
                                    f69(v35, v1.clone(), v38, v31, v32, if match v1 {
                                        Adt27::C0(_, _, _, _, h333) => {
                                            h333
                                        }
                                    } {
                                        false
                                    } else {
                                        v3
                                    })
                                }
                            }
                        }
                        Err(v43) => {
                            Ok(Err::<Adt28, Bytes>(v43))
                        }
                    }
                }
            } else {
                let m341 = f67(v0, v1.clone(), false, v31.clone())?;
                match m341 {
                    Ok(v44) => {
                        let m342 = v30;
                        match m342 {
                            Adt21::C0(v45) => {
                                let m346 = f20({
                                    let m345 = match v1.clone() {
                                        Adt27::C0(_, h344, _, _, _) => {
                                            (*h344).clone()
                                        }
                                    };
                                    match m345 {
                                        Adt26::C0(_, _, _, h343) => {
                                            h343
                                        }
                                    }
                                }, v45);
                                match m346 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v45)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v46) => {
                                        Ok(f68(v44, v1.clone(), v46, v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h347) => {
                                                h347
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f4(f1(Str::lit("f")), f10(v45)?)))
                                    }
                                }
                            }
                            Adt21::C1(r355) => {
                                let v48 = (*r355).clone();
                                let v49: Bytes = f70(match v1.clone() {
                                    Adt27::C0(_, _, _, h349, _) => {
                                        h349
                                    }
                                }, v48.clone());
                                let m352 = {
                                    let m351 = f71(match v1.clone() {
                                        Adt27::C0(_, _, _, h350, _) => {
                                            h350
                                        }
                                    }, v48.clone());
                                    if m351 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v49.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m352 {
                                    Ok(_) => {
                                        Ok(f68(v44, v1.clone(), f72(match v1.clone() {
                                            Adt27::C0(_, _, _, h353, _) => {
                                                h353
                                            }
                                        }, v48), v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h354) => {
                                                h354
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f30(v49)))
                                    }
                                    Err(v51) => {
                                        Ok(Err::<Adt28, Bytes>(v51))
                                    }
                                }
                            }
                            Adt21::C2(v47) => {
                                f69(v44, v1.clone(), v47, v31, v32, if match v1 {
                                    Adt27::C0(_, _, _, _, h348) => {
                                        h348
                                    }
                                } {
                                    false
                                } else {
                                    v3
                                })
                            }
                        }
                    }
                    Err(v52) => {
                        Ok(Err::<Adt28, Bytes>(v52))
                    }
                }
            }
        }
        Adt17::C4(r476, r477, v55, r478) => {
            let v53 = (*r476).clone();
            let v54 = (*r477).clone();
            let v56 = (*r478).clone();
            let m356 = v55;
            if m356 {
                let v57: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h357) => {
                            h357
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h358, _, _) => {
                                h358
                            }
                        }
                    }
                } else {
                    false
                };
                let m359 = v57;
                if m359 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h360, _) => {
                            h360
                        }
                    }, Adt17::C4(std::rc::Rc::new(v53), std::rc::Rc::new(v54), v55, std::rc::Rc::new(v56)))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m361 = f65(v0.clone(), v53)?;
                    match m361 {
                        Ok(_) => {
                            let m362 = f67(v0, v1, false, v54)?;
                            match m362 {
                                Ok(v59) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v59.clone() {
                                        Adt28::C0(h363, _, _, _, _) => {
                                            h363
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, h364, _, _, _) => {
                                            h364
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, h365, _, _) => {
                                            h365
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, _, h366, _) => {
                                            h366
                                        }
                                    }, if v55 {
                                        true
                                    } else {
                                        match v59 {
                                            Adt28::C0(_, _, _, _, h367) => {
                                                h367
                                            }
                                        }
                                    })))
                                }
                                Err(v60) => {
                                    Ok(Err::<Adt28, Bytes>(v60))
                                }
                            }
                        }
                        Err(v61) => {
                            Ok(Err::<Adt28, Bytes>(v61))
                        }
                    }
                }
            } else {
                let m368 = f65(v0.clone(), v53)?;
                match m368 {
                    Ok(_) => {
                        let m369 = f67(v0, v1, false, v54)?;
                        match m369 {
                            Ok(v63) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v63.clone() {
                                    Adt28::C0(h370, _, _, _, _) => {
                                        h370
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, h371, _, _, _) => {
                                        h371
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, h372, _, _) => {
                                        h372
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, _, h373, _) => {
                                        h373
                                    }
                                }, if v55 {
                                    true
                                } else {
                                    match v63 {
                                        Adt28::C0(_, _, _, _, h374) => {
                                            h374
                                        }
                                    }
                                })))
                            }
                            Err(v64) => {
                                Ok(Err::<Adt28, Bytes>(v64))
                            }
                        }
                    }
                    Err(v65) => {
                        Ok(Err::<Adt28, Bytes>(v65))
                    }
                }
            }
        }
        Adt17::C5(r479, r480, r481) => {
            let v66 = (*r479).clone();
            let v67 = (*r480).clone();
            let v68 = (*r481).clone();
            let v69: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h375) => {
                        h375
                    }
                } {
                    f73(v66.clone())
                } else {
                    false
                }
            } else {
                false
            };
            let m376 = v69;
            if m376 {
                let m381 = f74(v0, Adt27::C0(match v1.clone() {
                    Adt27::C0(h377, _, _, _, _) => {
                        h377
                    }
                }, std::rc::Rc::new(match v1.clone() {
                    Adt27::C0(_, h378, _, _, _) => {
                        (*h378).clone()
                    }
                }), match v1.clone() {
                    Adt27::C0(_, _, h379, _, _) => {
                        h379
                    }
                }, match v1.clone() {
                    Adt27::C0(_, _, _, h380, _) => {
                        h380
                    }
                }, false), v67.clone())?;
                match m381 {
                    Ok(v70) => {
                        let m382 = f75(v1, v66, f76(v67)?)?;
                        match m382 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v70))
                            }
                            Err(v72) => {
                                Ok(Err::<Adt28, Bytes>(v72))
                            }
                        }
                    }
                    Err(v73) => {
                        Ok(Err::<Adt28, Bytes>(v73))
                    }
                }
            } else {
                let v74: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h383) => {
                            h383
                        }
                    } {
                        false
                    } else {
                        if match v1.clone() {
                            Adt27::C0(_, _, h384, _, _) => {
                                h384
                            }
                        } {
                            let a385 = f77(v66.clone(), v67.clone())?;
                            bool_not(a385)
                        } else {
                            false
                        }
                    }
                } else {
                    false
                };
                let m386 = v74;
                if m386 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h387, _) => {
                            h387
                        }
                    }, Adt17::C5(std::rc::Rc::new(v66), std::rc::Rc::new(v67), std::rc::Rc::new(v68)))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m388 = f67(v0, v1.clone(), f73(v66.clone()), v67.clone())?;
                    match m388 {
                        Ok(v75) => {
                            let m389 = f75(v1, v66, f76(v67)?)?;
                            match m389 {
                                Ok(_) => {
                                    Ok(Ok::<Adt28, Bytes>(v75))
                                }
                                Err(v77) => {
                                    Ok(Err::<Adt28, Bytes>(v77))
                                }
                            }
                        }
                        Err(v78) => {
                            Ok(Err::<Adt28, Bytes>(v78))
                        }
                    }
                }
            }
        }
        Adt17::C6(r482, r483, r484) => {
            let v79 = (*r482).clone();
            let v80 = (*r483).clone();
            let v81 = (*r484).clone();
            let v82: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h390) => {
                        h390
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h391, _, _) => {
                            h391
                        }
                    }
                }
            } else {
                false
            };
            let m392 = v82;
            if m392 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h393, _) => {
                        h393
                    }
                }, Adt17::C6(std::rc::Rc::new(v79), std::rc::Rc::new(v80), std::rc::Rc::new(v81)))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m394 = f47(v0, v1.clone(), v79, false)?;
                match m394 {
                    Ok(v83) => {
                        f47(v83, v1, v80, false)
                    }
                    Err(v84) => {
                        Ok(Err::<Adt28, Bytes>(v84))
                    }
                }
            }
        }
        Adt17::C7(r485, r486, r487, _) => {
            let v85 = (*r485).clone();
            let v86 = (*r486).clone();
            let v87 = (*r487).clone();
            let m395 = f47(v0, v1.clone(), v85, false)?;
            match m395 {
                Ok(v89) => {
                    let v90: Adt28 = Adt28::C0(match v89.clone() {
                        Adt28::C0(h396, _, _, _, _) => {
                            h396
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, h397, _, _, _) => {
                            h397
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, h398, _, _) => {
                            h398
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, _, h399, _) => {
                            h399
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, _, _, h400) => {
                            h400
                        }
                    });
                    let m402 = f45(v90, match v1.clone() {
                        Adt27::C0(_, h401, _, _, _) => {
                            (*h401).clone()
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m402 {
                        Ok(v91) => {
                            let m403 = f29(v91, v1.clone(), v86, v3)?;
                            match m403 {
                                Ok(v92) => {
                                    let v93: Adt28 = Adt28::C0(match v92.clone() {
                                        Adt28::C0(h404, _, _, _, _) => {
                                            h404
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, h405, _, _, _) => {
                                            h405
                                        }
                                    }, f78(match v89.clone() {
                                        Adt28::C0(_, _, h406, _, _) => {
                                            h406
                                        }
                                    }, match v92.clone() {
                                        Adt28::C0(_, _, h407, _, _) => {
                                            h407
                                        }
                                    }), f79(match v89.clone() {
                                        Adt28::C0(_, _, _, h408, _) => {
                                            h408
                                        }
                                    }, match v92.clone() {
                                        Adt28::C0(_, _, _, h409, _) => {
                                            h409
                                        }
                                    }), match v92 {
                                        Adt28::C0(_, _, _, _, h410) => {
                                            h410
                                        }
                                    });
                                    let v94: Adt28 = Adt28::C0(match v93.clone() {
                                        Adt28::C0(h411, _, _, _, _) => {
                                            h411
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, h412, _, _, _) => {
                                            h412
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, _, h413, _, _) => {
                                            h413
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, _, _, h414, _) => {
                                            h414
                                        }
                                    }, match v93.clone() {
                                        Adt28::C0(_, _, _, _, h415) => {
                                            h415
                                        }
                                    });
                                    let m417 = f45(v94, match v1.clone() {
                                        Adt27::C0(_, h416, _, _, _) => {
                                            (*h416).clone()
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m417 {
                                        Ok(v95) => {
                                            let m418 = f29(v95, v1, v87, v3)?;
                                            match m418 {
                                                Ok(v96) => {
                                                    let v97: Adt28 = Adt28::C0(match v96.clone() {
                                                        Adt28::C0(h419, _, _, _, _) => {
                                                            h419
                                                        }
                                                    }, match v89 {
                                                        Adt28::C0(_, h420, _, _, _) => {
                                                            h420
                                                        }
                                                    }, f78(match v93.clone() {
                                                        Adt28::C0(_, _, h421, _, _) => {
                                                            h421
                                                        }
                                                    }, match v96.clone() {
                                                        Adt28::C0(_, _, h422, _, _) => {
                                                            h422
                                                        }
                                                    }), f79(match v93 {
                                                        Adt28::C0(_, _, _, h423, _) => {
                                                            h423
                                                        }
                                                    }, match v96.clone() {
                                                        Adt28::C0(_, _, _, h424, _) => {
                                                            h424
                                                        }
                                                    }), match v96 {
                                                        Adt28::C0(_, _, _, _, h425) => {
                                                            h425
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v97))
                                                }
                                                Err(v98) => {
                                                    Ok(Err::<Adt28, Bytes>(v98))
                                                }
                                            }
                                        }
                                        Err(v99) => {
                                            Ok(Err::<Adt28, Bytes>(v99))
                                        }
                                    }
                                }
                                Err(v100) => {
                                    Ok(Err::<Adt28, Bytes>(v100))
                                }
                            }
                        }
                        Err(v101) => {
                            Ok(Err::<Adt28, Bytes>(v101))
                        }
                    }
                }
                Err(v102) => {
                    Ok(Err::<Adt28, Bytes>(v102))
                }
            }
        }
        Adt17::C8(r488, r489, _) => {
            let v103 = (*r488).clone();
            let v104 = (*r489).clone();
            let m426 = f47(v0, v1.clone(), v103, false)?;
            match m426 {
                Ok(v106) => {
                    f80(v106.clone(), v1, v106.clone(), match v106 {
                        Adt28::C0(_, h427, _, _, _) => {
                            h427
                        }
                    }, v104, v3)
                }
                Err(v107) => {
                    Ok(Err::<Adt28, Bytes>(v107))
                }
            }
        }
        Adt17::C9(r490) => {
            let v108 = (*r490).clone();
            f29(v0, v1, v108, v3)
        }
        Adt17::C10(v109, r491, r492) => {
            let v110 = (*r491).clone();
            let v111 = (*r492).clone();
            let v112: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h428) => {
                        h428
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h429, _, _) => {
                            h429
                        }
                    }
                }
            } else {
                false
            };
            let m430 = v112;
            if m430 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h431, _) => {
                        h431
                    }
                }, Adt17::C10(v109, std::rc::Rc::new(v110), std::rc::Rc::new(v111)))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m432 = v109;
                match m432 {
                    Adt24::C0 => {
                        let m433 = f65(v0.clone(), v110)?;
                        match m433 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v114) => {
                                Ok(Err::<Adt28, Bytes>(v114))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m434 = f65(v0.clone(), v110)?;
                        match m434 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v116) => {
                                Ok(Err::<Adt28, Bytes>(v116))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m435 = f44(v1, f1(Str::lit("a list match")));
                        match m435 {
                            Ok(_) => {
                                let m436 = f65(v0.clone(), v110)?;
                                match m436 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v119) => {
                                        Ok(Err::<Adt28, Bytes>(v119))
                                    }
                                }
                            }
                            Err(v120) => {
                                Ok(Err::<Adt28, Bytes>(v120))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m437 = f65(v0.clone(), v110)?;
                        match m437 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v122) => {
                                Ok(Err::<Adt28, Bytes>(v122))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(r493, r494) => {
            let v136 = (*r493).clone();
            let v137 = (*r494).clone();
            let v138: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h438) => {
                        h438
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h439, _, _) => {
                            h439
                        }
                    }
                }
            } else {
                false
            };
            let v139: Result<(), Bytes> = {
                let m440 = v138;
                if m440 {
                    f81(match v1.clone() {
                        Adt27::C0(_, _, _, h441, _) => {
                            h441
                        }
                    }, v137.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m442 = v139;
            match m442 {
                Ok(_) => {
                    let m443 = f64(v0, v136.clone())?;
                    match m443 {
                        Ok(v141) => {
                            let v142: Adt28 = Adt28::C0(match v141.clone() {
                                Adt28::C0(h444, _, _, _, _) => {
                                    h444
                                }
                            }, match v141.clone() {
                                Adt28::C0(_, h445, _, _, _) => {
                                    h445
                                }
                            }, f82(match v141.clone() {
                                Adt28::C0(_, _, h446, _, _) => {
                                    h446
                                }
                            }, v136.clone()), match v141.clone() {
                                Adt28::C0(_, _, _, h447, _) => {
                                    h447
                                }
                            }, match v141.clone() {
                                Adt28::C0(_, _, _, _, h448) => {
                                    h448
                                }
                            });
                            let m453 = f29(v142.clone(), Adt27::C0(match v1.clone() {
                                Adt27::C0(h449, _, _, _, _) => {
                                    h449
                                }
                            }, std::rc::Rc::new(match v1.clone() {
                                Adt27::C0(_, h450, _, _, _) => {
                                    (*h450).clone()
                                }
                            }), match v1.clone() {
                                Adt27::C0(_, _, h451, _, _) => {
                                    h451
                                }
                            }, match v1 {
                                Adt27::C0(_, _, _, h452, _) => {
                                    h452
                                }
                            }, v138), v137, v138)?;
                            match m453 {
                                Ok(v143) => {
                                    let m457 = f83(match v143.clone() {
                                        Adt28::C0(_, _, h454, _, _) => {
                                            h454
                                        }
                                    }, v136, match v142.clone() {
                                        Adt28::C0(_, _, h455, _, _) => {
                                            h455
                                        }
                                    }, match v141.clone() {
                                        Adt28::C0(_, h456, _, _, _) => {
                                            h456
                                        }
                                    });
                                    match m457 {
                                        None => {
                                            let m461 = f84(match v143.clone() {
                                                Adt28::C0(_, _, _, h458, _) => {
                                                    h458
                                                }
                                            }, match v142 {
                                                Adt28::C0(_, _, _, h459, _) => {
                                                    h459
                                                }
                                            }, match v141.clone() {
                                                Adt28::C0(_, h460, _, _, _) => {
                                                    h460
                                                }
                                            });
                                            match m461 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v143.clone() {
                                                        Adt28::C0(h462, _, _, _, _) => {
                                                            h462
                                                        }
                                                    }, match v143.clone() {
                                                        Adt28::C0(_, h463, _, _, _) => {
                                                            h463
                                                        }
                                                    }, match v141.clone() {
                                                        Adt28::C0(_, _, h464, _, _) => {
                                                            h464
                                                        }
                                                    }, match v141 {
                                                        Adt28::C0(_, _, _, h465, _) => {
                                                            h465
                                                        }
                                                    }, match v143 {
                                                        Adt28::C0(_, _, _, _, h466) => {
                                                            h466
                                                        }
                                                    })))
                                                }
                                                Some(v145) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v145)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v144) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v144)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v146) => {
                                    Ok(Err::<Adt28, Bytes>(v146))
                                }
                            }
                        }
                        Err(v147) => {
                            Ok(Err::<Adt28, Bytes>(v147))
                        }
                    }
                }
                Err(v148) => {
                    Ok(Err::<Adt28, Bytes>(v148))
                }
            }
        }
    }
}

pub fn f48(p503: List<(Adt12, u64)>, v1: Adt26, p504: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m502 = (p503, p504);
    {
        let mut m502 = m502;
        loop {
            let step = {
                let (v0, v2) = m502;
                let m495 = v2;
                match m495 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(r498, v41, r499, r500, _, r501) => {
                        let v40 = (*r498).clone();
                        let v42 = (*r499).clone();
                        let v43 = (*r500).clone();
                        let v45 = (*r501).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m496 = v41;
                            if m496 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m497 = f49(v46, v1.clone(), v43, false)?;
                        match m497 {
                            Ok(v47) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v47, v45))
                            }
                            Err(v48) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v48)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m505 = v2;
    match m505 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r515, _) => {
            let v10 = (*r515).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r516, _, _) => {
            let v13 = (*r516).clone();
            f85(v0, v1, v13)
        }
        Adt17::C4(r517, r518, v18, _) => {
            let v16 = (*r517).clone();
            let v17 = (*r518).clone();
            let m506 = f86(v0.clone(), v16.clone());
            match m506 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m508 = f20(match v1.clone() {
                        Adt26::C0(_, _, h507, _) => {
                            h507
                        }
                    }, v20);
                    match m508 {
                        None => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v20)?, f1(Str::lit("::apply is not declared"))))))
                        }
                        Some(v21) => {
                            let v22: bool = if v21 {
                                !v18
                            } else {
                                v18
                            };
                            let v23: bool = if v3 {
                                v21
                            } else {
                                false
                            };
                            let m509 = v22;
                            if m509 {
                                let m510 = v23;
                                if m510 {
                                    f85(v0, v1, v17)
                                } else {
                                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("arithmetic: an application of Fn")), f4(f10(v20)?, f4(f1(Str::lit(" ")), f4(if v18 {
                                        f1(Str::lit("propagates"))
                                    } else {
                                        f1(Str::lit("drops"))
                                    }, f4(f1(Str::lit(" a failure its apply ")), if v21 {
                                        f1(Str::lit("can have"))
                                    } else {
                                        f1(Str::lit("cannot have"))
                                    })))))))
                                }
                            } else {
                                f85(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r519, _) => {
            let v25 = (*r519).clone();
            f85(v0, v1, v25)
        }
        Adt17::C6(r520, r521, _) => {
            let v27 = (*r520).clone();
            let v28 = (*r521).clone();
            let m511 = f49(v0, v1.clone(), v27, false)?;
            match m511 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r522, r523, r524, _) => {
            let v32 = (*r522).clone();
            let v33 = (*r523).clone();
            let v34 = (*r524).clone();
            let m512 = f49(v0, v1.clone(), v32, false)?;
            match m512 {
                Ok(v36) => {
                    let m513 = f32(v36, v1.clone(), v33, v3)?;
                    match m513 {
                        Ok(v37) => {
                            f32(v37, v1, v34, v3)
                        }
                        Err(v38) => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(v38))
                        }
                    }
                }
                Err(v39) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v39))
                }
            }
        }
        Adt17::C8(r525, r526, _) => {
            let v40 = (*r525).clone();
            let v41 = (*r526).clone();
            let m514 = f49(v0, v1.clone(), v40, false)?;
            match m514 {
                Ok(v43) => {
                    f87(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r527) => {
            let v45 = (*r527).clone();
            f32(v0, v1, v45, v3)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, r528) => {
            let v63 = (*r528).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m529 = f88(v1);
    match m529 {
        None => {
            v0
        }
        Some(v3) => {
            let m530 = f89(v2);
            match m530 {
                None => {
                    v0
                }
                Some(v4) => {
                    List::cons((v3, v4), v0)
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f90(v0, v1, 48u8) {
        Ok(Some({
            let a531 = v1;
            let a532 = 1u64;
            nat_add(a531, a532)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m533 = v0;
    match m533 {
        Adt3::C0 => {
            false
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
            true
        }
        Adt3::C6 => {
            true
        }
        Adt3::C7 => {
            false
        }
        Adt3::C8 => {
            false
        }
    }
}

pub fn f53(v0: Adt5) -> bool {
    let m534 = v0;
    match m534 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            true
        }
        Adt5::C2 => {
            true
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f54(v0: Adt7) -> bool {
    let m535 = v0;
    match m535 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            false
        }
        Adt7::C2 => {
            true
        }
    }
}

pub fn f55(v0: Fn3, p539: bool, p540: List<Adt11>) -> bool {
    let m538 = (p539, p540);
    {
        let mut m538 = m538;
        loop {
            let step = {
                let (v1, v2) = m538;
                let m536 = v2;
                match m536.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c537: Fn3 = v0.clone();
                            c537.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: bool, v1: Adt11) -> bool {
    let m541 = v1;
    match m541 {
        Adt11::C0 => {
            v0
        }
        Adt11::C1 => {
            v0
        }
        Adt11::C2 => {
            true
        }
        Adt11::C3 => {
            v0
        }
    }
}

pub fn f57(v0: List<Adt12>, v1: Adt12) -> bool {
    f91(Fn4::F92(std::rc::Rc::new(v1)), false, v0)
}

pub fn f58(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m543 = f18(match v0 {
        Adt26::C0(h542, _, _, _) => {
            h542
        }
    }, v1);
    match m543 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f93(v3, v2)
        }
    }
}

pub fn f59(v0: Adt14) -> R<u64> {
    let m544 = v0;
    match m544 {
        Adt14::C0(_) => {
            Ok(0u64)
        }
        Adt14::C1(_) => {
            Ok(0u64)
        }
        Adt14::C2(_) => {
            Ok(0u64)
        }
        Adt14::C3(_, _) => {
            Ok(0u64)
        }
        Adt14::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt14::C5 => {
            Ok(0u64)
        }
        Adt14::C6(_, r547) => {
            let v10 = (*r547).clone();
            let v11: u64 = f59(v10)?;
            let a545 = 1u64;
            let a546 = v11;
            nat_add(a545, a546)
        }
    }
}

pub fn f60(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m548 = v0;
    match m548 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r553) => {
            let v4 = (*r553).clone();
            let m549 = v1;
            match m549 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, r552, _) => {
                    let v8 = (*r552).clone();
                    let m550 = f94(v7);
                    if m550 {
                        let m551 = f95(v4);
                        if m551 {
                            Some(v8)
                        } else {
                            None::<Adt12>
                        }
                    } else {
                        None::<Adt12>
                    }
                }
                Adt17::C2(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C3(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C4(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C5(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C6(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C7(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C8(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C9(_) => {
                    None::<Adt12>
                }
                Adt17::C10(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C11(_, _) => {
                    None::<Adt12>
                }
                Adt17::C12 => {
                    None::<Adt12>
                }
                Adt17::C13(_, _) => {
                    None::<Adt12>
                }
                Adt17::C14(_, _, _, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C15(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C16(_, _) => {
                    None::<Adt12>
                }
            }
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f61(v0: Adt28, v1: Adt12, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m555 = f57(match v0.clone() {
        Adt28::C0(_, h554, _, _, _) => {
            h554
        }
    }, v1.clone());
    if m555 {
        let m557 = f57(match v0.clone() {
            Adt28::C0(_, _, h556, _, _) => {
                h556
            }
        }, v1.clone());
        if m557 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m559 = f96(match v0.clone() {
                    Adt28::C0(_, _, _, h558, _) => {
                        h558
                    }
                }, v1.clone());
                match m559 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m560 = f97(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m560 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h561, _, _, _, _) => {
                            h561
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h562, _, _, _) => {
                            h562
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h563, _, _) => {
                            h563
                        }
                    }, f98(match v0.clone() {
                        Adt28::C0(_, _, _, h564, _) => {
                            h564
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h565) => {
                            h565
                        }
                    })))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(v6))
                }
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f62(v0: List<Adt25>, v1: Adt17) -> R<Bytes> {
    Ok(f99({
        let m567 = f100(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m567 {
            Adt30::C0(h566, _, _) => {
                h566
            }
        }
    }))
}

pub fn f63(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m569 = match v0 {
        Adt27::C0(h568, _, _, _, _) => {
            h568
        }
    };
    if m569 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m571 = f57(match v0.clone() {
        Adt28::C0(_, h570, _, _, _) => {
            h570
        }
    }, v1.clone());
    if m571 {
        let m573 = f101(match v0.clone() {
            Adt28::C0(_, _, _, h572, _) => {
                h572
            }
        }, v1.clone());
        if m573 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m575 = f57(match v0.clone() {
                Adt28::C0(_, _, h574, _, _) => {
                    h574
                }
            }, v1.clone());
            if m575 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h576, _, _, _, _) => {
                        h576
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h577, _, _, _) => {
                        h577
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h578, _, _) => {
                        h578
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h579, _) => {
                        h579
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h580) => {
                        h580
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f65(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
    let m582 = f57(match v0.clone() {
        Adt28::C0(_, h581, _, _, _) => {
            h581
        }
    }, v1.clone());
    if m582 {
        let m584 = f57(match v0.clone() {
            Adt28::C0(_, _, h583, _, _) => {
                h583
            }
        }, v1.clone());
        if m584 {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m586 = f101(match v0 {
                Adt28::C0(_, _, _, h585, _) => {
                    h585
                }
            }, v1.clone());
            if m586 {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after a component was moved out of it"))))))
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f66(v0: Adt17) -> bool {
    let m587 = v0;
    match m587 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f102(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f67(p594: Adt28, v1: Adt27, v2: bool, p595: Adt17) -> R<Result<Adt28, Bytes>> {
    let m593 = (p594, p595);
    {
        let mut m593 = m593;
        loop {
            let step = {
                let (v0, v3) = m593;
                let m588 = v3;
                match m588 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r591, r592) => {
                        let v39 = (*r591).clone();
                        let v40 = (*r592).clone();
                        let v41: bool = if v2 {
                            f103(v39.clone())
                        } else {
                            false
                        };
                        let m589 = v41;
                        if m589 {
                            Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v0, v40))
                        } else {
                            let m590 = f47(v0, v1.clone(), v39, false)?;
                            match m590 {
                                Ok(v42) => {
                                    Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v42, v40))
                                }
                                Err(v43) => {
                                    Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v43)))
                                }
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
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

pub fn f68(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h596, _, _) => {
                h596
            }
        }
    } else {
        false
    };
    let m597 = v2;
    if m597 {
        let m598 = v3;
        if m598 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h599, _, _, _, _) => {
                    h599
                }
            }, match v0.clone() {
                Adt28::C0(_, h600, _, _, _) => {
                    h600
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h601, _, _) => {
                    h601
                }
            }, match v0 {
                Adt28::C0(_, _, _, h602, _) => {
                    h602
                }
            }, true))
        } else {
            let m603 = v6;
            if m603 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m604 = v3;
        if m604 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f69(v0: Adt28, v1: Adt27, v2: u64, v3: Adt17, v4: bool, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m607 = {
        let a605 = f76(v3.clone())?;
        let a606 = f104(v2);
        nat_eq(a605, a606)
    };
    if m607 {
        Ok(f68(v0, v1, false, v4, v5, f30(f105(v2))))
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("machine: ")), f4(f30(f105(v2)), f4(f1(Str::lit(" receives ")), f4(f10(f76(v3)?)?, f4(f1(Str::lit(" operands of ")), f10(f104(v2))?)))))))
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m609 = f106(v0, match v1.clone() {
        Adt22::C0(h608, _) => {
            h608
        }
    });
    match m609 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a611 = match v2.clone() {
                Adt25::C0(_, _, h610, _, _) => {
                    h610
                }
            };
            let a612 = 0u64;
            if nat_eq(a611, a612) {
                match v2 {
                    Adt25::C0(_, h613, _, _, _) => {
                        h613
                    }
                }
            } else {
                let a615 = match v2.clone() {
                    Adt25::C0(_, _, h614, _, _) => {
                        h614
                    }
                };
                let a616 = 1u64;
                if nat_eq(a615, a616) {
                    f4(f1(Str::lit("fixed_")), f4(f107(f108(match v1 {
                        Adt22::C0(_, h617) => {
                            h617
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h618, _, _, _) => {
                            h618
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h619, _, _, _) => {
                            h619
                        }
                    }, f107(f108(match v1 {
                        Adt22::C0(_, h620) => {
                            h620
                        }
                    })))
                }
            }
        }
    }
}

pub fn f71(v0: List<Adt25>, v1: Adt22) -> bool {
    let m622 = f106(v0, match v1 {
        Adt22::C0(h621, _) => {
            h621
        }
    });
    match m622 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h623) => {
                    h623
                }
            }
        }
    }
}

pub fn f72(v0: List<Adt25>, v1: Adt22) -> bool {
    let m625 = f106(v0, match v1 {
        Adt22::C0(h624, _) => {
            h624
        }
    });
    match m625 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h626, _) => {
                    h626
                }
            }
        }
    }
}

pub fn f73(v0: Adt23) -> bool {
    let m627 = v0;
    match m627 {
        Adt23::C0(_) => {
            false
        }
        Adt23::C1 => {
            false
        }
        Adt23::C2(_, _) => {
            false
        }
        Adt23::C3(_, _) => {
            true
        }
        Adt23::C4(_, _) => {
            false
        }
        Adt23::C5(_, _) => {
            false
        }
        Adt23::C6 => {
            false
        }
        Adt23::C7(_) => {
            false
        }
    }
}

pub fn f74(p633: Adt28, v1: Adt27, p634: Adt17) -> R<Result<Adt28, Bytes>> {
    let m632 = (p633, p634);
    {
        let mut m632 = m632;
        loop {
            let step = {
                let (v0, v2) = m632;
                let m628 = v2;
                match m628 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r630, r631) => {
                        let v38 = (*r630).clone();
                        let v39 = (*r631).clone();
                        let m629 = f47(v0, v1.clone(), v38, true)?;
                        match m629 {
                            Ok(v40) => {
                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m635 = v1;
    match m635 {
        Adt23::C0(r649) => {
            let v3 = (*r649).clone();
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(r650, r651) => {
            let v4 = (*r650).clone();
            let v5 = (*r651).clone();
            let m636 = f26(v0.clone(), v4)?;
            match m636 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(r652, r653) => {
            let v8 = (*r652).clone();
            let v9 = (*r653).clone();
            let m637 = f26(v0.clone(), v8)?;
            match m637 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m639 = f58(match v0 {
                Adt27::C0(_, h638, _, _, _) => {
                    (*h638).clone()
                }
            }, v12, v13);
            match m639 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m642 = {
                        let a640 = v14;
                        let a641 = v2;
                        nat_eq(a640, a641)
                    };
                    if m642 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m644 = f109(match v0 {
                Adt27::C0(_, h643, _, _, _) => {
                    (*h643).clone()
                }
            }, v15, v16);
            match m644 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m647 = {
                        let a645 = v17;
                        let a646 = v2;
                        nat_eq(a645, a646)
                    };
                    if m647 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f4(f1(Str::lit(" captures ")), f4(f10(v2)?, f4(f1(Str::lit(" values of ")), f10(v17)?)))))))))
                    }
                }
            }
        }
        Adt23::C6 => {
            Ok(f44(v0, f1(Str::lit("a list cell"))))
        }
        Adt23::C7(r654) => {
            let v18 = (*r654).clone();
            let m648 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m648 {
                Ok(_) => {
                    f26(v0, v18)
                }
                Err(v20) => {
                    Ok(Err::<(), Bytes>(v20))
                }
            }
        }
    }
}

pub fn f76(v0: Adt17) -> R<u64> {
    let m655 = v0;
    match m655 {
        Adt17::C0(_, _) => {
            Ok(0u64)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C8(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C9(_) => {
            Ok(0u64)
        }
        Adt17::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C11(_, _) => {
            Ok(0u64)
        }
        Adt17::C12 => {
            Ok(0u64)
        }
        Adt17::C13(_, r658) => {
            let v37 = (*r658).clone();
            let v38: u64 = f76(v37)?;
            let a656 = 1u64;
            let a657 = v38;
            nat_add(a656, a657)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C15(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C16(_, _) => {
            Ok(0u64)
        }
    }
}

pub fn f77(v0: Adt23, v1: Adt17) -> R<bool> {
    let m659 = v0;
    match m659 {
        Adt23::C0(_) => {
            Ok(false)
        }
        Adt23::C1 => {
            Ok(false)
        }
        Adt23::C2(_, _) => {
            Ok(false)
        }
        Adt23::C3(_, r660) => {
            let v6 = (*r660).clone();
            let v7: bool = f110(v6);
            let v8: bool = f111(v1)?;
            if v7 {
                Ok(v8)
            } else {
                Ok(false)
            }
        }
        Adt23::C4(_, _) => {
            Ok(false)
        }
        Adt23::C5(_, _) => {
            Ok(false)
        }
        Adt23::C6 => {
            Ok(false)
        }
        Adt23::C7(_) => {
            Ok(false)
        }
    }
}

pub fn f78(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f112(Fn5::F113, v0, v1)
}

pub fn f79(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f114(Fn6::F115, v0, v1)
}

pub fn f80(v0: Adt28, v1: Adt27, p679: Adt28, v3: List<Adt12>, p680: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v2, v4) = m678;
                let m661 = v4;
                match m661 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C15(r675, r676, r677) => {
                        let v49 = (*r675).clone();
                        let v50 = (*r676).clone();
                        let v51 = (*r677).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h662, _, _, _, _) => {
                                h662
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h663, _, _) => {
                                h663
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h664, _) => {
                                h664
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h665) => {
                                h665
                            }
                        });
                        let m667 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h666, _, _, _) => {
                                (*h666).clone()
                            }
                        }, v49)?;
                        match m667 {
                            Ok(v53) => {
                                let m668 = f29(v53, v1.clone(), v50, v5)?;
                                match m668 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h669, _, _, _, _) => {
                                                h669
                                            }
                                        }, v3.clone(), f78(match v2.clone() {
                                            Adt28::C0(_, _, h670, _, _) => {
                                                h670
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h671, _, _) => {
                                                h671
                                            }
                                        }), f79(match v2 {
                                            Adt28::C0(_, _, _, h672, _) => {
                                                h672
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h673, _) => {
                                                h673
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h674) => {
                                                h674
                                            }
                                        });
                                        Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v55, v51))
                                    }
                                    Err(v56) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v56)))
                                    }
                                }
                            }
                            Err(v57) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v57)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m681 = v1;
    match m681 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C9(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, r682) => {
            let v36 = (*r682).clone();
            f116(v0, v36)
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f82(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f112(Fn5::F117(std::rc::Rc::new(v1)), List::<Adt12>::nil(), v0)
}

pub fn f83(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f118(Fn7::F119(v2, v3, std::rc::Rc::new(v1)), None::<Adt12>, v0)
}

pub fn f84(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f120(Fn8::F121(v1, v2), None::<Adt12>, v0)
}

pub fn f85(p688: List<(Adt12, u64)>, v1: Adt26, p689: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m687 = (p688, p689);
    {
        let mut m687 = m687;
        loop {
            let step = {
                let (v0, v2) = m687;
                let m683 = v2;
                match m683 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(r685, r686) => {
                        let v38 = (*r685).clone();
                        let v39 = (*r686).clone();
                        let m684 = f49(v0, v1.clone(), v38, false)?;
                        match m684 {
                            Ok(v40) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f122(Fn9::F123(std::rc::Rc::new(v1)), None::<u64>, v0)
}

pub fn f87(p695: List<(Adt12, u64)>, v1: Adt26, p696: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v0, v2) = m694;
                let m690 = v2;
                match m690 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, r692, r693) => {
                        let v48 = (*r692).clone();
                        let v49 = (*r693).clone();
                        let m691 = f32(v0, v1.clone(), v48, v3)?;
                        match m691 {
                            Ok(v50) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v50, v49))
                            }
                            Err(v51) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v51)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt14) -> Option<Adt12> {
    let m697 = v0;
    match m697 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(r698) => {
            let v2 = (*r698).clone();
            Some(v2)
        }
        Adt14::C2(_) => {
            None::<Adt12>
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f89(v0: Adt2) -> Option<u64> {
    let m699 = v0;
    match m699 {
        Adt2::C0(_) => {
            None::<u64>
        }
        Adt2::C1(_) => {
            None::<u64>
        }
        Adt2::C2(_, _) => {
            None::<u64>
        }
        Adt2::C3(_, _, _) => {
            None::<u64>
        }
        Adt2::C4(v8, v9) => {
            let m700 = f6(v8);
            if m700 {
                Some(v9)
            } else {
                None::<u64>
            }
        }
        Adt2::C5 => {
            None::<u64>
        }
        Adt2::C6(_, _) => {
            None::<u64>
        }
    }
}

pub fn f90(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m701 = f124(v0, v1);
    match m701 {
        None => {
            false
        }
        Some(v3) => {
            let a702 = v3;
            let a703 = v2;
            equal(a702, a703)
        }
    }
}

pub fn f91(v0: Fn4, p707: bool, p708: List<Adt12>) -> bool {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v1, v2) = m706;
                let m704 = v2;
                match m704.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c705: Fn4 = v0.clone();
                            c705.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f125(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f93(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f126(Fn10::F127(v1), None::<u64>, v0)
}

pub fn f94(v0: Adt19) -> bool {
    let m709 = v0;
    match m709 {
        Adt19::C0 => {
            true
        }
        Adt19::C1 => {
            false
        }
        Adt19::C2 => {
            false
        }
        Adt19::C3 => {
            false
        }
        Adt19::C4 => {
            false
        }
    }
}

pub fn f95(v0: Adt14) -> bool {
    let m710 = v0;
    match m710 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(r711, r712) => {
            let v9 = (*r711).clone();
            let v10 = (*r712).clone();
            let v11: bool = f128(v9);
            let v12: bool = f95(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f96(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f129(Fn11::F130(std::rc::Rc::new(v1)), None::<List<u64>>, v0)
}

pub fn f97(p723: List<u64>, p724: u64, p725: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m722 = (p723, (p724, p725));
    {
        let mut m722 = m722;
        loop {
            let step = {
                let (v0, (v1, v2)) = m722;
                let m713 = v2;
                match m713 {
                    Adt14::C0(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C1(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C2(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C3(_, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C4(_, _, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C5 => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C6(r720, r721) => {
                        let v12 = (*r720).clone();
                        let v13 = (*r721).clone();
                        let v14: bool = f128(v12);
                        let m714 = v14;
                        if m714 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a715 = v1;
                                let a716 = 1u64;
                                nat_add(a715, a716)?
                            }, v13)))
                        } else {
                            let m717 = f131(v0.clone(), v1);
                            if m717 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a718 = v1;
                                    let a719 = 1u64;
                                    nat_add(a718, a719)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f132(v0, v1))
}

pub fn f99(v0: List<Adt29>) -> Bytes {
    let m726 = f133(Fn12::F134, None::<Bytes>, v0);
    match m726 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f100(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m727 = v1;
    match m727 {
        Adt17::C0(r738, r739) => {
            let v2 = (*r738).clone();
            let v3 = (*r739).clone();
            let m728 = f135(v2.clone());
            if m728 {
                Ok(v0)
            } else {
                Ok(f136(v0, f137(v2.clone()), v3, f138(v2)))
            }
        }
        Adt17::C1(v4, _, r740) => {
            let v6 = (*r740).clone();
            Ok(f139(v0, f140(v4), v6))
        }
        Adt17::C2(v7, r741, r742) => {
            let v8 = (*r741).clone();
            let v9 = (*r742).clone();
            let v10: Adt30 = f139(v0, f141(v7), v9);
            f100(v10, v8)
        }
        Adt17::C3(r743, r744, _, r745) => {
            let v11 = (*r743).clone();
            let v12 = (*r744).clone();
            let v14 = (*r745).clone();
            let v18: Adt30 = {
                let m729 = v11;
                match m729 {
                    Adt21::C0(_) => {
                        f139(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(r732) => {
                        let v17 = (*r732).clone();
                        f136(v0.clone(), f4(f1(Str::lit("call:runtime:")), f142(f143(f70(match v0.clone() {
                            Adt30::C0(_, _, h730) => {
                                h730
                            }
                        }, v17.clone()))?)), v14, f144(match v0 {
                            Adt30::C0(_, _, h731) => {
                                h731
                            }
                        }, v17))
                    }
                    Adt21::C2(_) => {
                        f139(v0, f1(Str::lit("call:machine")), v14)
                    }
                }
            };
            f100(v18, v12)
        }
        Adt17::C4(_, r746, _, r747) => {
            let v20 = (*r746).clone();
            let v22 = (*r747).clone();
            let v23: Adt30 = f139(v0, f1(Str::lit("apply")), v22);
            f100(v23, v20)
        }
        Adt17::C5(r748, r749, r750) => {
            let v24 = (*r748).clone();
            let v25 = (*r749).clone();
            let v26 = (*r750).clone();
            let v27: Adt30 = f139(f145(v0, v24.clone()), f4(f1(Str::lit("construct:")), f146(v24)), v26);
            f100(v27, v25)
        }
        Adt17::C6(r751, r752, r753) => {
            let v28 = (*r751).clone();
            let v29 = (*r752).clone();
            let v30 = (*r753).clone();
            let v31: Adt30 = f139(v0, f1(Str::lit("construct:pair")), v30);
            let v32: Adt30 = f100(v31, v28)?;
            f100(v32, v29)
        }
        Adt17::C7(r754, r755, r756, r757) => {
            let v33 = (*r754).clone();
            let v34 = (*r755).clone();
            let v35 = (*r756).clone();
            let v36 = (*r757).clone();
            let v37: Adt30 = f139(v0, f1(Str::lit("if")), v36);
            let v38: Adt30 = f100(v37, v33)?;
            let v39: Adt30 = f100(v38, v34)?;
            f100(v39, v35)
        }
        Adt17::C8(r758, r759, r760) => {
            let v40 = (*r758).clone();
            let v41 = (*r759).clone();
            let v42 = (*r760).clone();
            let v43: Adt30 = {
                let m733 = f147(v41.clone());
                if m733 {
                    f139(v0, f1(Str::lit("match:empty")), v42)
                } else {
                    let m734 = f148(v40.clone());
                    if m734 {
                        f139(v0, f1(Str::lit("match:list")), v42)
                    } else {
                        f149(v0, v41.clone(), v42)
                    }
                }
            };
            let v44: Adt30 = f100(v43, v40)?;
            f100(v44, v41)
        }
        Adt17::C9(r761) => {
            let v45 = (*r761).clone();
            f100(v0, v45)
        }
        Adt17::C10(v46, _, r762) => {
            let v48 = (*r762).clone();
            let m735 = f150(v46);
            if m735 {
                Ok(f139(v0, f1(Str::lit("match:nat")), v48))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r763, r764) => {
            let v49 = (*r763).clone();
            let v50 = (*r764).clone();
            let v51: Adt30 = f100(v0, v49)?;
            f100(v51, v50)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r765, r766) => {
            let v52 = (*r765).clone();
            let v53 = (*r766).clone();
            let v54: Adt30 = f100(v0, v52)?;
            f100(v54, v53)
        }
        Adt17::C14(r767, v56, r768, r769, r770, r771) => {
            let v55 = (*r767).clone();
            let v57 = (*r768).clone();
            let v58 = (*r769).clone();
            let v59 = (*r770).clone();
            let v60 = (*r771).clone();
            let v61: Bytes = {
                let m736 = f151(v55);
                if m736 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v62: Adt30 = {
                let m737 = v56;
                if m737 {
                    f152(v0, v57)
                } else {
                    v0
                }
            };
            let v63: Adt30 = f139(v62, v61, v59);
            let v64: Adt30 = f100(v63, v58)?;
            f100(v64, v60)
        }
        Adt17::C15(_, r772, r773) => {
            let v66 = (*r772).clone();
            let v67 = (*r773).clone();
            let v68: Adt30 = f100(v0, v66)?;
            f100(v68, v67)
        }
        Adt17::C16(_, r774) => {
            let v70 = (*r774).clone();
            f100(v0, v70)
        }
    }
}

pub fn f101(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m775 = f96(v0, v1);
    match m775 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f102(v0: Adt24) -> bool {
    let m776 = v0;
    match m776 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            true
        }
    }
}

pub fn f103(v0: Adt17) -> bool {
    let m777 = v0;
    match m777 {
        Adt17::C0(r778, _) => {
            let v1 = (*r778).clone();
            f135(v1)
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f104(v0: u64) -> u64 {
    let a779 = v0;
    let a780 = 0u64;
    if nat_eq(a779, a780) {
        2u64
    } else {
        let a781 = v0;
        let a782 = 1u64;
        if nat_eq(a781, a782) {
            2u64
        } else {
            let a783 = v0;
            let a784 = 2u64;
            if nat_eq(a783, a784) {
                2u64
            } else {
                let a785 = v0;
                let a786 = 3u64;
                if nat_eq(a785, a786) {
                    2u64
                } else {
                    let a787 = v0;
                    let a788 = 4u64;
                    if nat_eq(a787, a788) {
                        1u64
                    } else {
                        let a789 = v0;
                        let a790 = 5u64;
                        if nat_eq(a789, a790) {
                            2u64
                        } else {
                            let a791 = v0;
                            let a792 = 6u64;
                            if nat_eq(a791, a792) {
                                2u64
                            } else {
                                let a793 = v0;
                                let a794 = 7u64;
                                if nat_eq(a793, a794) {
                                    1u64
                                } else {
                                    let a795 = v0;
                                    let a796 = 8u64;
                                    if nat_eq(a795, a796) {
                                        2u64
                                    } else {
                                        let a797 = v0;
                                        let a798 = 9u64;
                                        if nat_eq(a797, a798) {
                                            2u64
                                        } else {
                                            let a799 = v0;
                                            let a800 = 10u64;
                                            if nat_eq(a799, a800) {
                                                2u64
                                            } else {
                                                let a801 = v0;
                                                let a802 = 11u64;
                                                if nat_eq(a801, a802) {
                                                    1u64
                                                } else {
                                                    let a803 = v0;
                                                    let a804 = 12u64;
                                                    if nat_eq(a803, a804) {
                                                        2u64
                                                    } else {
                                                        let a805 = v0;
                                                        let a806 = 13u64;
                                                        if nat_eq(a805, a806) {
                                                            2u64
                                                        } else {
                                                            let a807 = v0;
                                                            let a808 = 14u64;
                                                            if nat_eq(a807, a808) {
                                                                2u64
                                                            } else {
                                                                let a809 = v0;
                                                                let a810 = 15u64;
                                                                if nat_eq(a809, a810) {
                                                                    3u64
                                                                } else {
                                                                    let a811 = v0;
                                                                    let a812 = 16u64;
                                                                    if nat_eq(a811, a812) {
                                                                        3u64
                                                                    } else {
                                                                        let a813 = v0;
                                                                        let a814 = 17u64;
                                                                        if nat_eq(a813, a814) {
                                                                            2u64
                                                                        } else {
                                                                            let a815 = v0;
                                                                            let a816 = 18u64;
                                                                            if nat_eq(a815, a816) {
                                                                                2u64
                                                                            } else {
                                                                                let a817 = v0;
                                                                                let a818 = 19u64;
                                                                                if nat_eq(a817, a818) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a819 = v0;
                                                                                    let a820 = 20u64;
                                                                                    if nat_eq(a819, a820) {
                                                                                        3u64
                                                                                    } else {
                                                                                        let a821 = v0;
                                                                                        let a822 = 21u64;
                                                                                        if nat_eq(a821, a822) {
                                                                                            2u64
                                                                                        } else {
                                                                                            let a823 = v0;
                                                                                            let a824 = 22u64;
                                                                                            if nat_eq(a823, a824) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a825 = v0;
                                                                                                let a826 = 23u64;
                                                                                                if nat_eq(a825, a826) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a827 = v0;
                                                                                                    let a828 = 24u64;
                                                                                                    if nat_eq(a827, a828) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        let a829 = v0;
                                                                                                        let a830 = 25u64;
                                                                                                        if nat_eq(a829, a830) {
                                                                                                            3u64
                                                                                                        } else {
                                                                                                            let a831 = v0;
                                                                                                            let a832 = 26u64;
                                                                                                            if nat_eq(a831, a832) {
                                                                                                                1u64
                                                                                                            } else {
                                                                                                                let a833 = v0;
                                                                                                                let a834 = 27u64;
                                                                                                                if nat_eq(a833, a834) {
                                                                                                                    2u64
                                                                                                                } else {
                                                                                                                    let a835 = v0;
                                                                                                                    let a836 = 28u64;
                                                                                                                    if nat_eq(a835, a836) {
                                                                                                                        2u64
                                                                                                                    } else {
                                                                                                                        let a837 = v0;
                                                                                                                        let a838 = 29u64;
                                                                                                                        if nat_eq(a837, a838) {
                                                                                                                            3u64
                                                                                                                        } else {
                                                                                                                            let a839 = v0;
                                                                                                                            let a840 = 30u64;
                                                                                                                            if nat_eq(a839, a840) {
                                                                                                                                2u64
                                                                                                                            } else {
                                                                                                                                let a841 = v0;
                                                                                                                                let a842 = 31u64;
                                                                                                                                if nat_eq(a841, a842) {
                                                                                                                                    1u64
                                                                                                                                } else {
                                                                                                                                    let a843 = v0;
                                                                                                                                    let a844 = 32u64;
                                                                                                                                    if nat_eq(a843, a844) {
                                                                                                                                        1u64
                                                                                                                                    } else {
                                                                                                                                        let a845 = v0;
                                                                                                                                        let a846 = 33u64;
                                                                                                                                        if nat_eq(a845, a846) {
                                                                                                                                            2u64
                                                                                                                                        } else {
                                                                                                                                            let a847 = v0;
                                                                                                                                            let a848 = 34u64;
                                                                                                                                            if nat_eq(a847, a848) {
                                                                                                                                                3u64
                                                                                                                                            } else {
                                                                                                                                                let a849 = v0;
                                                                                                                                                let a850 = 35u64;
                                                                                                                                                if nat_eq(a849, a850) {
                                                                                                                                                    1u64
                                                                                                                                                } else {
                                                                                                                                                    let a851 = v0;
                                                                                                                                                    let a852 = 36u64;
                                                                                                                                                    if nat_eq(a851, a852) {
                                                                                                                                                        1u64
                                                                                                                                                    } else {
                                                                                                                                                        let a853 = v0;
                                                                                                                                                        let a854 = 37u64;
                                                                                                                                                        if nat_eq(a853, a854) {
                                                                                                                                                            2u64
                                                                                                                                                        } else {
                                                                                                                                                            let a855 = v0;
                                                                                                                                                            let a856 = 38u64;
                                                                                                                                                            if nat_eq(a855, a856) {
                                                                                                                                                                3u64
                                                                                                                                                            } else {
                                                                                                                                                                let a857 = v0;
                                                                                                                                                                let a858 = 39u64;
                                                                                                                                                                if nat_eq(a857, a858) {
                                                                                                                                                                    2u64
                                                                                                                                                                } else {
                                                                                                                                                                    let a859 = v0;
                                                                                                                                                                    let a860 = 40u64;
                                                                                                                                                                    if nat_eq(a859, a860) {
                                                                                                                                                                        1u64
                                                                                                                                                                    } else {
                                                                                                                                                                        let a861 = v0;
                                                                                                                                                                        let a862 = 41u64;
                                                                                                                                                                        if nat_eq(a861, a862) {
                                                                                                                                                                            1u64
                                                                                                                                                                        } else {
                                                                                                                                                                            let a863 = v0;
                                                                                                                                                                            let a864 = 42u64;
                                                                                                                                                                            if nat_eq(a863, a864) {
                                                                                                                                                                                1u64
                                                                                                                                                                            } else {
                                                                                                                                                                                let a865 = v0;
                                                                                                                                                                                let a866 = 43u64;
                                                                                                                                                                                if nat_eq(a865, a866) {
                                                                                                                                                                                    1u64
                                                                                                                                                                                } else {
                                                                                                                                                                                    let a867 = v0;
                                                                                                                                                                                    let a868 = 44u64;
                                                                                                                                                                                    if nat_eq(a867, a868) {
                                                                                                                                                                                        3u64
                                                                                                                                                                                    } else {
                                                                                                                                                                                        0u64
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

pub fn f105(v0: u64) -> Bytes {
    let a869 = v0;
    let a870 = 0u64;
    if nat_eq(a869, a870) {
        f1(Str::lit("checked_add"))
    } else {
        let a871 = v0;
        let a872 = 1u64;
        if nat_eq(a871, a872) {
            f1(Str::lit("checked_sub"))
        } else {
            let a873 = v0;
            let a874 = 2u64;
            if nat_eq(a873, a874) {
                f1(Str::lit("checked_mul"))
            } else {
                let a875 = v0;
                let a876 = 3u64;
                if nat_eq(a875, a876) {
                    f1(Str::lit("checked_div"))
                } else {
                    let a877 = v0;
                    let a878 = 4u64;
                    if nat_eq(a877, a878) {
                        f1(Str::lit("checked_neg"))
                    } else {
                        let a879 = v0;
                        let a880 = 5u64;
                        if nat_eq(a879, a880) {
                            f1(Str::lit("checked_shl"))
                        } else {
                            let a881 = v0;
                            let a882 = 6u64;
                            if nat_eq(a881, a882) {
                                f1(Str::lit("checked_shr"))
                            } else {
                                let a883 = v0;
                                let a884 = 7u64;
                                if nat_eq(a883, a884) {
                                    f1(Str::lit("convert"))
                                } else {
                                    let a885 = v0;
                                    let a886 = 8u64;
                                    if nat_eq(a885, a886) {
                                        f1(Str::lit("bit_and"))
                                    } else {
                                        let a887 = v0;
                                        let a888 = 9u64;
                                        if nat_eq(a887, a888) {
                                            f1(Str::lit("bit_or"))
                                        } else {
                                            let a889 = v0;
                                            let a890 = 10u64;
                                            if nat_eq(a889, a890) {
                                                f1(Str::lit("bit_xor"))
                                            } else {
                                                let a891 = v0;
                                                let a892 = 11u64;
                                                if nat_eq(a891, a892) {
                                                    f1(Str::lit("bit_not"))
                                                } else {
                                                    let a893 = v0;
                                                    let a894 = 12u64;
                                                    if nat_eq(a893, a894) {
                                                        f1(Str::lit("nat_eq"))
                                                    } else {
                                                        let a895 = v0;
                                                        let a896 = 13u64;
                                                        if nat_eq(a895, a896) {
                                                            f1(Str::lit("nat_le"))
                                                        } else {
                                                            let a897 = v0;
                                                            let a898 = 14u64;
                                                            if nat_eq(a897, a898) {
                                                                f1(Str::lit("nat_lt"))
                                                            } else {
                                                                let a899 = v0;
                                                                let a900 = 15u64;
                                                                if nat_eq(a899, a900) {
                                                                    f1(Str::lit("nat_rem"))
                                                                } else {
                                                                    let a901 = v0;
                                                                    let a902 = 16u64;
                                                                    if nat_eq(a901, a902) {
                                                                        f1(Str::lit("int_rem"))
                                                                    } else {
                                                                        let a903 = v0;
                                                                        let a904 = 17u64;
                                                                        if nat_eq(a903, a904) {
                                                                            f1(Str::lit("nat_add"))
                                                                        } else {
                                                                            let a905 = v0;
                                                                            let a906 = 18u64;
                                                                            if nat_eq(a905, a906) {
                                                                                f1(Str::lit("nat_sub"))
                                                                            } else {
                                                                                let a907 = v0;
                                                                                let a908 = 19u64;
                                                                                if nat_eq(a907, a908) {
                                                                                    f1(Str::lit("nat_mul"))
                                                                                } else {
                                                                                    let a909 = v0;
                                                                                    let a910 = 20u64;
                                                                                    if nat_eq(a909, a910) {
                                                                                        f1(Str::lit("nat_quot"))
                                                                                    } else {
                                                                                        let a911 = v0;
                                                                                        let a912 = 21u64;
                                                                                        if nat_eq(a911, a912) {
                                                                                            f1(Str::lit("int_add"))
                                                                                        } else {
                                                                                            let a913 = v0;
                                                                                            let a914 = 22u64;
                                                                                            if nat_eq(a913, a914) {
                                                                                                f1(Str::lit("int_sub"))
                                                                                            } else {
                                                                                                let a915 = v0;
                                                                                                let a916 = 23u64;
                                                                                                if nat_eq(a915, a916) {
                                                                                                    f1(Str::lit("int_mul"))
                                                                                                } else {
                                                                                                    let a917 = v0;
                                                                                                    let a918 = 24u64;
                                                                                                    if nat_eq(a917, a918) {
                                                                                                        f1(Str::lit("int_neg"))
                                                                                                    } else {
                                                                                                        let a919 = v0;
                                                                                                        let a920 = 25u64;
                                                                                                        if nat_eq(a919, a920) {
                                                                                                            f1(Str::lit("int_quot"))
                                                                                                        } else {
                                                                                                            let a921 = v0;
                                                                                                            let a922 = 26u64;
                                                                                                            if nat_eq(a921, a922) {
                                                                                                                f1(Str::lit("list_length"))
                                                                                                            } else {
                                                                                                                let a923 = v0;
                                                                                                                let a924 = 27u64;
                                                                                                                if nat_eq(a923, a924) {
                                                                                                                    f1(Str::lit("list_append"))
                                                                                                                } else {
                                                                                                                    let a925 = v0;
                                                                                                                    let a926 = 28u64;
                                                                                                                    if nat_eq(a925, a926) {
                                                                                                                        f1(Str::lit("list_index"))
                                                                                                                    } else {
                                                                                                                        let a927 = v0;
                                                                                                                        let a928 = 29u64;
                                                                                                                        if nat_eq(a927, a928) {
                                                                                                                            f1(Str::lit("list_slice"))
                                                                                                                        } else {
                                                                                                                            let a929 = v0;
                                                                                                                            let a930 = 30u64;
                                                                                                                            if nat_eq(a929, a930) {
                                                                                                                                f1(Str::lit("bytes_append"))
                                                                                                                            } else {
                                                                                                                                let a931 = v0;
                                                                                                                                let a932 = 31u64;
                                                                                                                                if nat_eq(a931, a932) {
                                                                                                                                    f1(Str::lit("bytes_length"))
                                                                                                                                } else {
                                                                                                                                    let a933 = v0;
                                                                                                                                    let a934 = 32u64;
                                                                                                                                    if nat_eq(a933, a934) {
                                                                                                                                        f1(Str::lit("string_length"))
                                                                                                                                    } else {
                                                                                                                                        let a935 = v0;
                                                                                                                                        let a936 = 33u64;
                                                                                                                                        if nat_eq(a935, a936) {
                                                                                                                                            f1(Str::lit("bytes_index"))
                                                                                                                                        } else {
                                                                                                                                            let a937 = v0;
                                                                                                                                            let a938 = 34u64;
                                                                                                                                            if nat_eq(a937, a938) {
                                                                                                                                                f1(Str::lit("bytes_slice"))
                                                                                                                                            } else {
                                                                                                                                                let a939 = v0;
                                                                                                                                                let a940 = 35u64;
                                                                                                                                                if nat_eq(a939, a940) {
                                                                                                                                                    f1(Str::lit("utf8_encode"))
                                                                                                                                                } else {
                                                                                                                                                    let a941 = v0;
                                                                                                                                                    let a942 = 36u64;
                                                                                                                                                    if nat_eq(a941, a942) {
                                                                                                                                                        f1(Str::lit("utf8_decode"))
                                                                                                                                                    } else {
                                                                                                                                                        let a943 = v0;
                                                                                                                                                        let a944 = 37u64;
                                                                                                                                                        if nat_eq(a943, a944) {
                                                                                                                                                            f1(Str::lit("bytes_compare"))
                                                                                                                                                        } else {
                                                                                                                                                            let a945 = v0;
                                                                                                                                                            let a946 = 38u64;
                                                                                                                                                            if nat_eq(a945, a946) {
                                                                                                                                                                f1(Str::lit("string_split"))
                                                                                                                                                            } else {
                                                                                                                                                                let a947 = v0;
                                                                                                                                                                let a948 = 39u64;
                                                                                                                                                                if nat_eq(a947, a948) {
                                                                                                                                                                    f1(Str::lit("string_join"))
                                                                                                                                                                } else {
                                                                                                                                                                    let a949 = v0;
                                                                                                                                                                    let a950 = 40u64;
                                                                                                                                                                    if nat_eq(a949, a950) {
                                                                                                                                                                        f1(Str::lit("format_decimal"))
                                                                                                                                                                    } else {
                                                                                                                                                                        let a951 = v0;
                                                                                                                                                                        let a952 = 41u64;
                                                                                                                                                                        if nat_eq(a951, a952) {
                                                                                                                                                                            f1(Str::lit("parse_decimal"))
                                                                                                                                                                        } else {
                                                                                                                                                                            let a953 = v0;
                                                                                                                                                                            let a954 = 42u64;
                                                                                                                                                                            if nat_eq(a953, a954) {
                                                                                                                                                                                f1(Str::lit("parse_decimal_int"))
                                                                                                                                                                            } else {
                                                                                                                                                                                let a955 = v0;
                                                                                                                                                                                let a956 = 43u64;
                                                                                                                                                                                if nat_eq(a955, a956) {
                                                                                                                                                                                    f1(Str::lit("format_decimal_int"))
                                                                                                                                                                                } else {
                                                                                                                                                                                    let a957 = v0;
                                                                                                                                                                                    let a958 = 44u64;
                                                                                                                                                                                    if nat_eq(a957, a958) {
                                                                                                                                                                                        f1(Str::lit("bytes_update"))
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
    }
}

pub fn f106(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f153(Fn13::F154(v1), None::<Adt25>, v0)
}

pub fn f107(v0: Adt4) -> Bytes {
    let m959 = v0;
    match m959 {
        Adt4::C0 => {
            f1(Str::lit("u8"))
        }
        Adt4::C1 => {
            f1(Str::lit("u16"))
        }
        Adt4::C2 => {
            f1(Str::lit("u32"))
        }
        Adt4::C3 => {
            f1(Str::lit("u64"))
        }
        Adt4::C4 => {
            f1(Str::lit("i8"))
        }
        Adt4::C5 => {
            f1(Str::lit("i16"))
        }
        Adt4::C6 => {
            f1(Str::lit("i32"))
        }
        Adt4::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f108(v0: Option<Adt4>) -> Adt4 {
    let m960 = v0;
    match m960 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f109(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m962 = f18(match v0 {
        Adt26::C0(_, h961, _, _) => {
            h961
        }
    }, v1);
    match m962 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f93(v3, v2)
        }
    }
}

pub fn f110(v0: Adt2) -> bool {
    let m963 = v0;
    match m963 {
        Adt2::C0(v1) => {
            let m964 = v1;
            match m964 {
                Adt3::C0 => {
                    false
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
                    true
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

pub fn f111(v0: Adt17) -> R<bool> {
    let m965 = v0;
    match m965 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(_, _, _) => {
            Ok(false)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C5(_, _, _) => {
            Ok(false)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C8(_, _, _) => {
            Ok(false)
        }
        Adt17::C9(_) => {
            Ok(false)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(r968, r969) => {
            let v36 = (*r968).clone();
            let v37 = (*r969).clone();
            let v38: bool = {
                let a966 = f76(v37)?;
                let a967 = 0u64;
                nat_eq(a966, a967)
            };
            let v39: bool = f103(v36);
            if v38 {
                Ok(v39)
            } else {
                Ok(false)
            }
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(false)
        }
    }
}

pub fn f112(v0: Fn5, p973: List<Adt12>, p974: List<Adt12>) -> List<Adt12> {
    let m972 = (p973, p974);
    {
        let mut m972 = m972;
        loop {
            let step = {
                let (v1, v2) = m972;
                let m970 = v2;
                match m970.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c971: Fn5 = v0.clone();
                            c971.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m972 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f57(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f114(v0: Fn6, p978: List<(Adt12, List<u64>)>, p979: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m977 = (p978, p979);
    {
        let mut m977 = m977;
        loop {
            let step = {
                let (v1, v2) = m977;
                let m975 = v2;
                match m975.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c976: Fn6 = v0.clone();
                            c976.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m981 = f96(v0.clone(), {
        let (h980, _) = v1.clone();
        h980
    });
    match m981 {
        None => {
            f98(v0, {
                let (h984, _) = v1;
                h984
            }, {
                let (_, h985) = v1;
                h985
            })
        }
        Some(v2) => {
            f98(v0, {
                let (h982, _) = v1;
                h982
            }, f155(v2, {
                let (_, h983) = v1;
                h983
            }))
        }
    }
}

pub fn f116(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m986 = v1;
    match m986 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(r989, r990, _) => {
            let v18 = (*r989).clone();
            let v19 = (*r990).clone();
            let m987 = f73(v18);
            if m987 {
                f156(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r991, r992, _) => {
            let v25 = (*r991).clone();
            let v26 = (*r992).clone();
            let m988 = f81(v0.clone(), v25)?;
            match m988 {
                Ok(_) => {
                    f81(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r993, _) => {
            let v31 = (*r993).clone();
            f157(v0, v31)
        }
        Adt17::C9(r994) => {
            let v33 = (*r994).clone();
            f81(v0, v33)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f117(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f125(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f118(v0: Fn7, p998: Option<Adt12>, p999: List<Adt12>) -> Option<Adt12> {
    let m997 = (p998, p999);
    {
        let mut m997 = m997;
        loop {
            let step = {
                let (v1, v2) = m997;
                let m995 = v2;
                match m995.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c996: Fn7 = v0.clone();
                            c996.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f125(v4.clone(), v2);
    let v6: bool = f57(v0, v4.clone());
    let v7: bool = f57(v1, v4.clone());
    let v8: bool = if v5 {
        false
    } else {
        if v6 {
            false
        } else {
            v7
        }
    };
    if v8 {
        f158(v3, v4)
    } else {
        v3
    }
}

pub fn f120(v0: Fn8, p1003: Option<Adt12>, p1004: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m1002 = (p1003, p1004);
    {
        let mut m1002 = m1002;
        loop {
            let step = {
                let (v1, v2) = m1002;
                let m1000 = v2;
                match m1000.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c1001: Fn8 = v0.clone();
                            c1001.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f57(v1, {
        let (h1005, _) = v3.clone();
        h1005
    });
    let v5: bool = f159(v0, {
        let (h1006, _) = v3.clone();
        h1006
    }, {
        let (_, h1007) = v3;
        h1007
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f158(v2, {
            let (h1008, _) = v3;
            h1008
        })
    } else {
        v2
    }
}

pub fn f122(v0: Fn9, p1012: Option<u64>, p1013: List<(Adt12, u64)>) -> Option<u64> {
    let m1011 = (p1012, p1013);
    {
        let mut m1011 = m1011;
        loop {
            let step = {
                let (v1, v2) = m1011;
                let m1009 = v2;
                match m1009.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c1010: Fn9 = v0.clone();
                            c1010.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1011 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m1014 = v1;
    match m1014 {
        None => {
            let v4: bool = f125({
                let (h1015, _) = v2;
                h1015
            }, v0);
            if v4 {
                Some({
                    let (_, h1016) = v2;
                    h1016
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Option<u8> {
    let a1017 = v0;
    let a1018 = v1;
    index_bytes(a1017, a1018)
}

pub fn f125(v0: Adt12, v1: Adt12) -> bool {
    let a1019 = f160(v0.clone());
    let a1020 = f160(v1.clone());
    if nat_eq(a1019, a1020) {
        let a1021 = f161(v0.clone());
        let a1022 = f161(v1.clone());
        if nat_eq(a1021, a1022) {
            let a1023 = f162(v0);
            let a1024 = f162(v1);
            equal(a1023, a1024)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f126(v0: Fn10, p1028: Option<u64>, p1029: List<(u64, u64)>) -> Option<u64> {
    let m1027 = (p1028, p1029);
    {
        let mut m1027 = m1027;
        loop {
            let step = {
                let (v1, v2) = m1027;
                let m1025 = v2;
                match m1025.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c1026: Fn10 = v0.clone();
                            c1026.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m1030 = v1;
    match m1030 {
        None => {
            let a1032 = {
                let (h1031, _) = v2;
                h1031
            };
            let a1033 = v0;
            if nat_eq(a1032, a1033) {
                Some({
                    let (_, h1034) = v2;
                    h1034
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f128(v0: Adt14) -> bool {
    let m1035 = v0;
    match m1035 {
        Adt14::C0(v1) => {
            let m1036 = v1;
            match m1036 {
                Adt15::C0 => {
                    true
                }
                Adt15::C1 => {
                    false
                }
                Adt15::C2 => {
                    false
                }
                Adt15::C3 => {
                    false
                }
            }
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f129(v0: Fn11, p1040: Option<List<u64>>, p1041: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m1039 = (p1040, p1041);
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v1, v2) = m1039;
                let m1037 = v2;
                match m1037.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c1038: Fn11 = v0.clone();
                            c1038.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m1042 = v1;
    match m1042 {
        None => {
            let v4: bool = f125({
                let (h1043, _) = v2;
                h1043
            }, v0);
            if v4 {
                Some({
                    let (_, h1044) = v2;
                    h1044
                })
            } else {
                None::<List<u64>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f131(v0: List<u64>, v1: u64) -> bool {
    f163(Fn14::F164(v1), false, v0)
}

pub fn f132(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f114(Fn6::F165(std::rc::Rc::new(v1)), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f133(v0: Fn12, p1048: Option<Bytes>, p1049: List<Adt29>) -> Option<Bytes> {
    let m1047 = (p1048, p1049);
    {
        let mut m1047 = m1047;
        loop {
            let step = {
                let (v1, v2) = m1047;
                let m1045 = v2;
                match m1045.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c1046: Fn12 = v0.clone();
                            c1046.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m1050 = v0;
    match m1050 {
        None => {
            Some(match v1 {
                Adt29::C0(h1051, _, _, _) => {
                    h1051
                }
            })
        }
        Some(v2) => {
            let m1053 = f166(match v1.clone() {
                Adt29::C0(h1052, _, _, _) => {
                    h1052
                }
            }, v2.clone());
            if m1053 {
                Some(match v1 {
                    Adt29::C0(h1054, _, _, _) => {
                        h1054
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f135(v0: Adt18) -> bool {
    let m1055 = v0;
    match m1055 {
        Adt18::C0 => {
            false
        }
        Adt18::C1 => {
            false
        }
        Adt18::C2 => {
            false
        }
        Adt18::C3 => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5 => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7 => {
            false
        }
        Adt18::C8 => {
            true
        }
    }
}

pub fn f136(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h1056, _) => {
            h1056
        }
    }, match v2 {
        Adt9::C0(_, h1057) => {
            h1057
        }
    }, v3);
    let m1059 = f167(match v0.clone() {
        Adt30::C0(h1058, _, _) => {
            h1058
        }
    }, v4.clone());
    if m1059 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h1060, _, _) => {
                h1060
            }
        }), match v0.clone() {
            Adt30::C0(_, h1061, _) => {
                h1061
            }
        }, match v0 {
            Adt30::C0(_, _, h1062) => {
                h1062
            }
        })
    }
}

pub fn f137(v0: Adt18) -> Bytes {
    let m1063 = v0;
    match m1063 {
        Adt18::C0 => {
            f1(Str::lit("lit:unit"))
        }
        Adt18::C1 => {
            f1(Str::lit("lit:bool"))
        }
        Adt18::C2 => {
            f1(Str::lit("lit:nat"))
        }
        Adt18::C3 => {
            f1(Str::lit("lit:int"))
        }
        Adt18::C4(_) => {
            f1(Str::lit("lit:fixed"))
        }
        Adt18::C5 => {
            f1(Str::lit("lit:string"))
        }
        Adt18::C6 => {
            f1(Str::lit("lit:bytes"))
        }
        Adt18::C7 => {
            f1(Str::lit("lit:ordering"))
        }
        Adt18::C8 => {
            f1(Str::lit("lit:overflow"))
        }
    }
}

pub fn f138(v0: Adt18) -> Option<Adt4> {
    let m1064 = v0;
    match m1064 {
        Adt18::C0 => {
            None::<Adt4>
        }
        Adt18::C1 => {
            None::<Adt4>
        }
        Adt18::C2 => {
            None::<Adt4>
        }
        Adt18::C3 => {
            None::<Adt4>
        }
        Adt18::C4(v1) => {
            Some(v1)
        }
        Adt18::C5 => {
            None::<Adt4>
        }
        Adt18::C6 => {
            None::<Adt4>
        }
        Adt18::C7 => {
            None::<Adt4>
        }
        Adt18::C8 => {
            None::<Adt4>
        }
    }
}

pub fn f139(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f136(v0, v1, v2, None::<Adt4>)
}

pub fn f140(v0: Adt19) -> Bytes {
    let m1065 = v0;
    match m1065 {
        Adt19::C0 => {
            f1(Str::lit("read"))
        }
        Adt19::C1 => {
            f1(Str::lit("read"))
        }
        Adt19::C2 => {
            f1(Str::lit("read"))
        }
        Adt19::C3 => {
            f1(Str::lit("read"))
        }
        Adt19::C4 => {
            f1(Str::lit("box"))
        }
    }
}

pub fn f141(v0: Adt20) -> Bytes {
    let m1066 = v0;
    match m1066 {
        Adt20::C0 => {
            f1(Str::lit("if"))
        }
        Adt20::C1 => {
            f1(Str::lit("box"))
        }
        Adt20::C2 => {
            f1(Str::lit("widen"))
        }
        Adt20::C3 => {
            f1(Str::lit("succeed"))
        }
    }
}

pub fn f142(v0: Bytes) -> Bytes {
    let m1067 = f168(v0.clone(), f1(Str::lit("checked_neg_")));
    if m1067 {
        f1(Str::lit("checked_neg"))
    } else {
        let m1068 = f168(v0.clone(), f1(Str::lit("format_")));
        if m1068 {
            f1(Str::lit("format_decimal"))
        } else {
            let m1069 = f168(v0.clone(), f1(Str::lit("parse_")));
            if m1069 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m1070 = f168(v0.clone(), f1(Str::lit("append_")));
                if m1070 {
                    f1(Str::lit("append"))
                } else {
                    let m1071 = f168(v0.clone(), f1(Str::lit("length_")));
                    if m1071 {
                        f1(Str::lit("length"))
                    } else {
                        let m1072 = f168(v0.clone(), f1(Str::lit("index_")));
                        if m1072 {
                            f1(Str::lit("index"))
                        } else {
                            let m1073 = f168(v0.clone(), f1(Str::lit("slice_")));
                            if m1073 {
                                f1(Str::lit("slice"))
                            } else {
                                let m1074 = f168(v0.clone(), f1(Str::lit("update_")));
                                if m1074 {
                                    f1(Str::lit("update"))
                                } else {
                                    let m1077 = {
                                        let a1075 = v0.clone();
                                        let a1076 = f1(Str::lit("nat_succ"));
                                        equal(a1075, a1076)
                                    };
                                    if m1077 {
                                        f1(Str::lit("succ"))
                                    } else {
                                        v0
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

pub fn f143(v0: Bytes) -> R<Bytes> {
    let m1078 = f169(v0.clone(), f1(Str::lit("::")))?;
    match m1078 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a1079 = v1;
                let a1080 = 2u64;
                nat_add(a1079, a1080)?
            }, {
                let a1084 = {
                    let a1081 = v0;
                    length_bytes(a1081)
                };
                let a1085 = {
                    let a1082 = v1;
                    let a1083 = 2u64;
                    nat_add(a1082, a1083)?
                };
                nat_sub(a1084, a1085)
            }))
        }
    }
}

pub fn f144(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m1087 = f106(v0, match v1.clone() {
        Adt22::C0(h1086, _) => {
            h1086
        }
    });
    match m1087 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a1089 = match v2 {
                Adt25::C0(_, _, h1088, _, _) => {
                    h1088
                }
            };
            let a1090 = 0u64;
            if nat_eq(a1089, a1090) {
                None::<Adt4>
            } else {
                Some(f108(match v1 {
                    Adt22::C0(_, h1091) => {
                        h1091
                    }
                }))
            }
        }
    }
}

pub fn f145(v0: Adt30, v1: Adt23) -> Adt30 {
    let m1092 = v1;
    match m1092 {
        Adt23::C0(r1093) => {
            let v2 = (*r1093).clone();
            f152(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(r1094, r1095) => {
            let v3 = (*r1094).clone();
            let v4 = (*r1095).clone();
            f152(f152(v0, v3), v4)
        }
        Adt23::C3(r1096, r1097) => {
            let v5 = (*r1096).clone();
            let v6 = (*r1097).clone();
            f152(f152(v0, v5), v6)
        }
        Adt23::C4(_, _) => {
            v0
        }
        Adt23::C5(_, _) => {
            v0
        }
        Adt23::C6 => {
            v0
        }
        Adt23::C7(r1098) => {
            let v11 = (*r1098).clone();
            f152(v0, v11)
        }
    }
}

pub fn f146(v0: Adt23) -> Bytes {
    let m1099 = v0;
    match m1099 {
        Adt23::C0(_) => {
            f1(Str::lit("none"))
        }
        Adt23::C1 => {
            f1(Str::lit("some"))
        }
        Adt23::C2(_, _) => {
            f1(Str::lit("ok"))
        }
        Adt23::C3(_, _) => {
            f1(Str::lit("err"))
        }
        Adt23::C4(_, _) => {
            f1(Str::lit("adt"))
        }
        Adt23::C5(_, _) => {
            f1(Str::lit("closure"))
        }
        Adt23::C6 => {
            f1(Str::lit("cons"))
        }
        Adt23::C7(_) => {
            f1(Str::lit("nil"))
        }
    }
}

pub fn f147(v0: Adt17) -> bool {
    let m1100 = v0;
    match m1100 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            true
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f148(v0: Adt17) -> bool {
    let m1101 = v0;
    match m1101 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f170(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f149(p1107: Adt30, p1108: Adt17, v2: Adt9) -> Adt30 {
    let m1106 = (p1107, p1108);
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let (v0, v1) = m1106;
                let m1102 = v1;
                match m1102 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C12 => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C15(r1104, _, r1105) => {
                        let v46 = (*r1104).clone();
                        let v48 = (*r1105).clone();
                        let m1103 = f171(v46);
                        match m1103 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f139(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Adt24) -> bool {
    let m1109 = v0;
    match m1109 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f151(v0: Adt14) -> bool {
    let m1110 = v0;
    match m1110 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            true
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f152(v0: Adt30, v1: Adt2) -> Adt30 {
    let m1111 = v1;
    match m1111 {
        Adt2::C0(v2) => {
            f172(v0, f4(f1(Str::lit("type:")), f173(v2)))
        }
        Adt2::C1(_) => {
            f172(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r1112) => {
            let v5 = (*r1112).clone();
            let v6: Adt30 = f172(v0, f4(f1(Str::lit("type:")), f174(v4)));
            f152(v6, v5)
        }
        Adt2::C3(v7, r1113, r1114) => {
            let v8 = (*r1113).clone();
            let v9 = (*r1114).clone();
            let v10: Adt30 = f172(v0, f4(f1(Str::lit("type:")), f175(v7)));
            let v11: Adt30 = f152(v10, v8);
            f152(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f172(v0, f4(f1(Str::lit("type:")), f176(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r1115, r1116) => {
            let v14 = (*r1115).clone();
            let v15 = (*r1116).clone();
            let v16: Adt30 = f152(v0, v14);
            f152(v16, v15)
        }
    }
}

pub fn f153(v0: Fn13, p1120: Option<Adt25>, p1121: List<Adt25>) -> Option<Adt25> {
    let m1119 = (p1120, p1121);
    {
        let mut m1119 = m1119;
        loop {
            let step = {
                let (v1, v2) = m1119;
                let m1117 = v2;
                match m1117.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c1118: Fn13 = v0.clone();
                            c1118.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m1122 = v1;
    match m1122 {
        None => {
            let a1124 = match v2.clone() {
                Adt25::C0(h1123, _, _, _, _) => {
                    h1123
                }
            };
            let a1125 = v0;
            if nat_eq(a1124, a1125) {
                Some(v2)
            } else {
                None::<Adt25>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f155(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f177(Fn15::F178, v0, v1)
}

pub fn f156(v0: List<Adt25>, p1131: Adt17) -> R<Result<(), Bytes>> {
    let m1130 = p1131;
    {
        let mut m1130 = m1130;
        loop {
            let step = {
                let v1 = m1130;
                let m1126 = v1;
                match m1126 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(r1128, r1129) => {
                        let v37 = (*r1128).clone();
                        let v38 = (*r1129).clone();
                        let m1127 = f179(v37.clone())?;
                        if m1127 {
                            Ok::<Adt17, R<Result<(), Bytes>>>(v38)
                        } else {
                            Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible loop ends with, ")), f4(f62(v0.clone(), v37)?, f1(Str::lit(", is not a fallible value")))))))
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Adt25>, p1137: Adt17) -> R<Result<(), Bytes>> {
    let m1136 = p1137;
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let v1 = m1136;
                let m1132 = v1;
                match m1132 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, r1134, r1135) => {
                        let v46 = (*r1134).clone();
                        let v47 = (*r1135).clone();
                        let m1133 = f81(v0.clone(), v46)?;
                        match m1133 {
                            Ok(_) => {
                                Ok::<Adt17, R<Result<(), Bytes>>>(v47)
                            }
                            Err(v49) => {
                                Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v49)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m1138 = v0;
    match m1138 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m1139 = f180(v1.clone(), v2.clone());
            if m1139 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f159(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m1140 = f96(v0, v1);
    match m1140 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f181(v2.clone(), v3.clone());
            let v5: bool = f181(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f160(v0: Adt12) -> u64 {
    let m1141 = v0;
    match m1141 {
        Adt12::C0(_) => {
            0u64
        }
        Adt12::C1(_) => {
            1u64
        }
        Adt12::C2(_) => {
            2u64
        }
        Adt12::C3(_) => {
            3u64
        }
        Adt12::C4(_) => {
            4u64
        }
        Adt12::C5(_) => {
            5u64
        }
        Adt12::C6(_) => {
            6u64
        }
        Adt12::C7(_) => {
            7u64
        }
        Adt12::C8(_) => {
            8u64
        }
        Adt12::C9(_) => {
            9u64
        }
    }
}

pub fn f161(v0: Adt12) -> u64 {
    let m1142 = v0;
    match m1142 {
        Adt12::C0(v1) => {
            v1
        }
        Adt12::C1(v2) => {
            v2
        }
        Adt12::C2(v3) => {
            v3
        }
        Adt12::C3(v4) => {
            v4
        }
        Adt12::C4(v5) => {
            v5
        }
        Adt12::C5(v6) => {
            v6
        }
        Adt12::C6(v7) => {
            v7
        }
        Adt12::C7(v8) => {
            v8
        }
        Adt12::C8(v9) => {
            v9
        }
        Adt12::C9(_) => {
            0u64
        }
    }
}

pub fn f162(v0: Adt12) -> Bytes {
    let m1143 = v0;
    match m1143 {
        Adt12::C0(_) => {
            Bytes::lit(&[])
        }
        Adt12::C1(_) => {
            Bytes::lit(&[])
        }
        Adt12::C2(_) => {
            Bytes::lit(&[])
        }
        Adt12::C3(_) => {
            Bytes::lit(&[])
        }
        Adt12::C4(_) => {
            Bytes::lit(&[])
        }
        Adt12::C5(_) => {
            Bytes::lit(&[])
        }
        Adt12::C6(_) => {
            Bytes::lit(&[])
        }
        Adt12::C7(_) => {
            Bytes::lit(&[])
        }
        Adt12::C8(_) => {
            Bytes::lit(&[])
        }
        Adt12::C9(v10) => {
            v10
        }
    }
}

pub fn f163(v0: Fn14, p1147: bool, p1148: List<u64>) -> bool {
    let m1146 = (p1147, p1148);
    {
        let mut m1146 = m1146;
        loop {
            let step = {
                let (v1, v2) = m1146;
                let m1144 = v2;
                match m1144.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1145: Fn14 = v0.clone();
                            c1145.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1149 = v2;
        let a1150 = v0;
        nat_eq(a1149, a1150)
    }
}

pub fn f165(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f125({
        let (h1151, _) = v2.clone();
        h1151
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f166(v0: Bytes, v1: Bytes) -> bool {
    let a1156 = {
        let a1152 = v0;
        let a1153 = v1;
        compare_bytes(a1152, a1153)
    };
    let a1157 = {
        let a1154 = Bytes::lit(&[]);
        let a1155 = Bytes::lit(&[0x01]);
        compare_bytes(a1154, a1155)
    };
    equal(a1156, a1157)
}

pub fn f167(v0: List<Adt29>, v1: Adt29) -> bool {
    f182(Fn16::F183(std::rc::Rc::new(v1)), false, v0)
}

pub fn f168(v0: Bytes, v1: Bytes) -> bool {
    let m1162 = {
        let a1159 = v0;
        let a1160 = 0u64;
        let a1161 = {
            let a1158 = v1.clone();
            length_bytes(a1158)
        };
        slice_bytes(a1159, a1160, a1161)
    };
    match m1162 {
        None => {
            false
        }
        Some(v2) => {
            let a1163 = v2;
            let a1164 = v1;
            equal(a1163, a1164)
        }
    }
}

pub fn f169(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1165 = v0.clone();
        length_bytes(a1165)
    };
    let v3: u64 = {
        let a1166 = v1.clone();
        length_bytes(a1166)
    };
    let v4: u64 = {
        let (h1167, _) = f38(Fn2::F184(v1.clone(), v3, v0.clone(), v2), {
            let a1168 = v2;
            let a1169 = 1u64;
            nat_add(a1168, a1169)?
        }, 0u64)?;
        h1167
    };
    if f185(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f170(v0: Adt24) -> bool {
    let m1170 = v0;
    match m1170 {
        Adt24::C0 => {
            false
        }
        Adt24::C1 => {
            false
        }
        Adt24::C2 => {
            true
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f171(v0: Adt14) -> Option<Bytes> {
    let m1171 = v0;
    match m1171 {
        Adt14::C0(v1) => {
            f186(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f187(v4))
        }
        Adt14::C4(_, _, _) => {
            Some(f1(Str::lit("match:adt")))
        }
        Adt14::C5 => {
            None::<Bytes>
        }
        Adt14::C6(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f172(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h1172, _, _) => {
            h1172
        }
    }, f188(match v0.clone() {
        Adt30::C0(_, h1173, _) => {
            h1173
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h1174) => {
            h1174
        }
    })
}

pub fn f173(v0: Adt3) -> Bytes {
    let m1175 = v0;
    match m1175 {
        Adt3::C0 => {
            f1(Str::lit("unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("int"))
        }
        Adt3::C4 => {
            f1(Str::lit("ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("string"))
        }
        Adt3::C6 => {
            f1(Str::lit("bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("i128"))
        }
    }
}

pub fn f174(v0: Adt5) -> Bytes {
    let m1176 = v0;
    match m1176 {
        Adt5::C0 => {
            f1(Str::lit("option"))
        }
        Adt5::C1 => {
            f1(Str::lit("list"))
        }
        Adt5::C2 => {
            f1(Str::lit("rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("ref"))
        }
    }
}

pub fn f175(v0: Adt6) -> Bytes {
    let m1177 = v0;
    match m1177 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f176(v0: Adt7) -> Bytes {
    let m1178 = v0;
    match m1178 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("param"))
        }
    }
}

pub fn f177(v0: Fn15, p1182: List<u64>, p1183: List<u64>) -> List<u64> {
    let m1181 = (p1182, p1183);
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let (v1, v2) = m1181;
                let m1179 = v2;
                match m1179.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1180: Fn15 = v0.clone();
                            c1180.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f131(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f179(v0: Adt17) -> R<bool> {
    let m1184 = v0;
    match m1184 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(v6, _, _) => {
            let m1185 = v6;
            match m1185 {
                Adt20::C0 => {
                    Ok(false)
                }
                Adt20::C1 => {
                    Ok(false)
                }
                Adt20::C2 => {
                    Ok(false)
                }
                Adt20::C3 => {
                    Ok(true)
                }
            }
        }
        Adt17::C3(_, _, v11, _) => {
            Ok(!v11)
        }
        Adt17::C4(_, _, v15, _) => {
            Ok(!v15)
        }
        Adt17::C5(r1186, r1187, _) => {
            let v17 = (*r1186).clone();
            let v18 = (*r1187).clone();
            f77(v17, v18)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(true)
        }
        Adt17::C8(_, _, _) => {
            Ok(true)
        }
        Adt17::C9(_) => {
            Ok(true)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(_, _) => {
            Ok(false)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(true)
        }
    }
}

pub fn f180(v0: Adt12, v1: Adt12) -> bool {
    let a1188 = f160(v0.clone());
    let a1189 = f160(v1.clone());
    if nat_eq(a1188, a1189) {
        let a1190 = f160(v0.clone());
        let a1191 = 9u64;
        if nat_eq(a1190, a1191) {
            f166(f162(v0), f162(v1))
        } else {
            let a1192 = f161(v0);
            let a1193 = f161(v1);
            nat_lt(a1192, a1193)
        }
    } else {
        let a1194 = f160(v0);
        let a1195 = f160(v1);
        nat_lt(a1194, a1195)
    }
}

pub fn f181(v0: List<u64>, v1: List<u64>) -> bool {
    f163(Fn14::F189(v1), true, v0)
}

pub fn f182(v0: Fn16, p1199: bool, p1200: List<Adt29>) -> bool {
    let m1198 = (p1199, p1200);
    {
        let mut m1198 = m1198;
        loop {
            let step = {
                let (v1, v2) = m1198;
                let m1196 = v2;
                match m1196.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c1197: Fn16 = v0.clone();
                            c1197.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f190(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f184(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f185(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1203 = {
            let a1201 = v4;
            let a1202 = v1;
            nat_add(a1201, a1202)?
        };
        let a1204 = v3;
        if nat_lt(a1203, a1204) {
            Ok(Some({
                let a1205 = v4;
                let a1206 = 1u64;
                nat_add(a1205, a1206)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f185(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1211 = {
        let a1208 = v0;
        let a1209 = v2;
        let a1210 = {
            let a1207 = v1.clone();
            length_bytes(a1207)
        };
        slice_bytes(a1208, a1209, a1210)
    };
    match m1211 {
        None => {
            false
        }
        Some(v3) => {
            let a1212 = v3;
            let a1213 = v1;
            equal(a1212, a1213)
        }
    }
}

pub fn f186(v0: Adt15) -> Option<Bytes> {
    let m1214 = v0;
    match m1214 {
        Adt15::C0 => {
            None::<Bytes>
        }
        Adt15::C1 => {
            None::<Bytes>
        }
        Adt15::C2 => {
            Some(f1(Str::lit("match:option")))
        }
        Adt15::C3 => {
            Some(f1(Str::lit("match:ordering")))
        }
    }
}

pub fn f187(v0: Adt16) -> Bytes {
    let m1215 = v0;
    match m1215 {
        Adt16::C0 => {
            f1(Str::lit("match:option"))
        }
        Adt16::C1 => {
            f1(Str::lit("match:result"))
        }
        Adt16::C2 => {
            f1(Str::lit("match:result"))
        }
    }
}

pub fn f188(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1216 = f191(v0.clone(), v1.clone());
    if m1216 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f192(Fn17::F193(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m1218 = {
            let (h1217, _) = v2;
            h1217
        };
        if m1218 {
            f194({
                let (_, h1219) = v2;
                h1219
            })
        } else {
            f194(List::cons(v1, {
                let (_, h1220) = v2;
                h1220
            }))
        }
    }
}

pub fn f189(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f131(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f190(v0: Adt29, v1: Adt29) -> bool {
    let a1223 = match v0.clone() {
        Adt29::C0(h1221, _, _, _) => {
            h1221
        }
    };
    let a1224 = match v1.clone() {
        Adt29::C0(h1222, _, _, _) => {
            h1222
        }
    };
    if equal(a1223, a1224) {
        let a1227 = match v0.clone() {
            Adt29::C0(_, h1225, _, _) => {
                h1225
            }
        };
        let a1228 = match v1.clone() {
            Adt29::C0(_, h1226, _, _) => {
                h1226
            }
        };
        if equal(a1227, a1228) {
            if f195(match v0.clone() {
                Adt29::C0(_, _, h1229, _) => {
                    h1229
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h1230, _) => {
                    h1230
                }
            }) {
                f195(match v0 {
                    Adt29::C0(_, _, _, h1231) => {
                        h1231
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h1232) => {
                        h1232
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f191(v0: List<Bytes>, v1: Bytes) -> bool {
    f196(Fn18::F197(v1), false, v0)
}

pub fn f192(v0: Fn17, p1236: (bool, List<Bytes>), p1237: List<Bytes>) -> (bool, List<Bytes>) {
    let m1235 = (p1236, p1237);
    {
        let mut m1235 = m1235;
        loop {
            let step = {
                let (v1, v2) = m1235;
                let m1233 = v2;
                match m1233.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c1234: Fn17 = v0.clone();
                            c1234.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f198(v0, v1, v2)
}

pub fn f194(v0: List<Bytes>) -> List<Bytes> {
    f199(Fn19::F200, List::<Bytes>::nil(), v0)
}

pub fn f195(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a1238 = f201(v0);
    let a1239 = f201(v1);
    equal(a1238, a1239)
}

pub fn f196(v0: Fn18, p1243: bool, p1244: List<Bytes>) -> bool {
    let m1242 = (p1243, p1244);
    {
        let mut m1242 = m1242;
        loop {
            let step = {
                let (v1, v2) = m1242;
                let m1240 = v2;
                match m1240.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1241: Fn18 = v0.clone();
                            c1241.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1245 = v2;
        let a1246 = v0;
        equal(a1245, a1246)
    }
}

pub fn f198(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h1247, _) = v1;
        h1247
    };
    let v4: List<Bytes> = {
        let (_, h1248) = v1;
        h1248
    };
    let m1249 = v3;
    if m1249 {
        (true, List::cons(v2, v4))
    } else {
        let m1250 = f166(v0.clone(), v2.clone());
        if m1250 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f199(v0: Fn19, p1254: List<Bytes>, p1255: List<Bytes>) -> List<Bytes> {
    let m1253 = (p1254, p1255);
    {
        let mut m1253 = m1253;
        loop {
            let step = {
                let (v1, v2) = m1253;
                let m1251 = v2;
                match m1251.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1252: Fn19 = v0.clone();
                            c1252.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f201(v0: Option<Adt4>) -> Bytes {
    let m1256 = v0;
    match m1256 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
