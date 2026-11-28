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

#[derive(Clone)]
pub enum Adt0 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(std::rc::Rc<Adt0>, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn0::F9 => f9(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F16,
}

impl Fn1 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn1::F16 => f16(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21(Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn2::F21(k0, k1) => f21(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F33,
}

impl Fn3 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn3::F33 => f33(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F35(u64, Bytes, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn4::F35(k0, k1, k2) => f35(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F39,
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn5::F39 => f39(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F42,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn6::F42 => f42(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    f1(f2(Str::lit("lexlean-attestation-v2")), f3(f2(Str::lit("attestation-body")), v0))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f4(f5(f5(v0, Bytes::lit(&[0x00])), v1))
}

pub fn f2(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    f5(f5(f5(f6({
        let a2 = v0.clone();
        length_bytes(a2)
    }, 4u64), v0), f6({
        let a3 = v1.clone();
        length_bytes(a3)
    }, 8u64)), v1)
}

pub fn f4(v0: Bytes) -> R<Bytes> {
    let a5 = {
        let a4 = Str::lit("blake3:");
        utf8_encode(a4)
    };
    let a6 = f7(v0)?;
    Ok(append_bytes(a5, a6))
}

pub fn f5(v0: Bytes, v1: Bytes) -> Bytes {
    let a7 = v0;
    let a8 = v1;
    append_bytes(a7, a8)
}

pub fn f6(v0: u64, v1: u64) -> Bytes {
    let (_, h9) = f8(Fn0::F9, v1, (v0, Bytes::lit(&[])));
    h9
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f10(v0)?;
    let a30 = f11(match v1.clone() {
        Adt0::C0(h10, _, _, _, _, _, _, _) => {
            h10
        }
    });
    let a31 = {
        let a28 = f11(match v1.clone() {
            Adt0::C0(_, h11, _, _, _, _, _, _) => {
                h11
            }
        });
        let a29 = {
            let a26 = f11(match v1.clone() {
                Adt0::C0(_, _, h12, _, _, _, _, _) => {
                    h12
                }
            });
            let a27 = {
                let a24 = f11(match v1.clone() {
                    Adt0::C0(_, _, _, h13, _, _, _, _) => {
                        h13
                    }
                });
                let a25 = {
                    let a22 = f11(match v1.clone() {
                        Adt0::C0(_, _, _, _, h14, _, _, _) => {
                            h14
                        }
                    });
                    let a23 = {
                        let a20 = f11(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h15, _, _) => {
                                h15
                            }
                        });
                        let a21 = {
                            let a18 = f11(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h16, _) => {
                                    h16
                                }
                            });
                            let a19 = f11(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h17) => {
                                    h17
                                }
                            });
                            append_bytes(a18, a19)
                        };
                        append_bytes(a20, a21)
                    };
                    append_bytes(a22, a23)
                };
                append_bytes(a24, a25)
            };
            append_bytes(a26, a27)
        };
        append_bytes(a28, a29)
    };
    Ok(append_bytes(a30, a31))
}

pub fn f8(v0: Fn0, p35: u64, p36: (u64, Bytes)) -> (u64, Bytes) {
    let m34 = (p35, p36);
    {
        let mut m34 = m34;
        loop {
            let step = {
                let (v1, v2) = m34;
                let m32 = v1;
                if m32 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m32 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c33: Fn0 = v0.clone();
                        c33.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m34 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a38 = {
            let (h37, _) = v0.clone();
            h37
        };
        let a39 = 256u64;
        let a40 = 0u64;
        nat_quot(a38, a39, a40)
    }, f5(f12({
        let a42 = {
            let (h41, _) = v0;
            h41
        };
        let a43 = 256u64;
        let a44 = 0u64;
        nat_rem(a42, a43, a44)
    }), {
        let (_, h45) = v0;
        h45
    }))
}

pub fn f10(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a46 = v0.clone();
        length_bytes(a46)
    };
    let v2: u64 = {
        let a47 = v1;
        let a48 = 0u64;
        if nat_eq(a47, a48) {
            1u64
        } else {
            let a51 = {
                let a49 = v1;
                let a50 = 1023u64;
                nat_add(a49, a50)?
            };
            let a52 = 1024u64;
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        }
    };
    let v3: u64 = {
        let a54 = v2;
        let a55 = 1u64;
        nat_sub(a54, a55)
    };
    let m57 = {
        let (_, h56) = f13(v0.clone(), v1, v3)?;
        h56
    };
    match m57.uncons() {
        None => {
            f14(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h58) = f15(Fn1::F16, 64u64, (List::cons(v4, v5), f14(v0, v1, v3, false)?));
            Ok(h58)
        }
    }
}

pub fn f11(v0: u32) -> Bytes {
    let a71 = {
        let a59 = f17(v0, 4u32);
        let a60 = f17(v0, 0u32);
        append_bytes(a59, a60)
    };
    let a72 = {
        let a69 = {
            let a61 = f17(v0, 12u32);
            let a62 = f17(v0, 8u32);
            append_bytes(a61, a62)
        };
        let a70 = {
            let a67 = {
                let a63 = f17(v0, 20u32);
                let a64 = f17(v0, 16u32);
                append_bytes(a63, a64)
            };
            let a68 = {
                let a65 = f17(v0, 28u32);
                let a66 = f17(v0, 24u32);
                append_bytes(a65, a66)
            };
            append_bytes(a67, a68)
        };
        append_bytes(a69, a70)
    };
    append_bytes(a71, a72)
}

pub fn f12(v0: u64) -> Bytes {
    f18(f19(), v0)
}

pub fn f13(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f20(Fn2::F21(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a73 = 1024u64;
        let a74 = v2;
        nat_mul(a73, a74)?
    };
    let v5: u64 = f22(v1, v4);
    let v6: u64 = f23(v5)?;
    let v7: u64 = {
        let a75 = v6;
        let a76 = 1u64;
        nat_sub(a75, a76)
    };
    Ok(f24(f25(v0.clone(), v4, v2, v7)?, f26(v0, {
        let a79 = v4;
        let a80 = {
            let a77 = 64u64;
            let a78 = v7;
            nat_mul(a77, a78)?
        };
        nat_add(a79, a80)?
    })?, v2, f27({
        let a83 = v5;
        let a84 = {
            let a81 = 64u64;
            let a82 = v7;
            nat_mul(a81, a82)?
        };
        nat_sub(a83, a84)
    }), {
        let a87 = {
            let a85 = v7;
            let a86 = 0u64;
            if nat_eq(a85, a86) {
                3u32
            } else {
                2u32
            }
        };
        let a88 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a87, a88)
    }))
}

pub fn f15(v0: Fn1, p92: u64, p93: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m91 = (p92, p93);
    {
        let mut m91 = m91;
        loop {
            let step = {
                let (v1, v2) = m91;
                let m89 = v1;
                if m89 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m89 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c90: Fn1 = v0.clone();
                        c90.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m91 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f28(v0)
}

pub fn f17(v0: u32, v1: u32) -> Bytes {
    f29(f30(v0, v1))
}

pub fn f18(v0: Bytes, v1: u64) -> Bytes {
    let m97 = {
        let a94 = v0;
        let a95 = v1;
        let a96 = 1u64;
        slice_bytes(a94, a95, a96)
    };
    match m97 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f20(v0: Fn2, p101: u64, p102: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v1, v2) = m100;
                let m98 = v1;
                if m98 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m98 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c99: Fn2 = v0.clone();
                        c99.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f31(v2.clone(), f14(v0, v1, {
        let (h103, _) = v2;
        h103
    }, false)?)
}

pub fn f22(v0: u64, v1: u64) -> u64 {
    let a106 = 1024u64;
    let a107 = {
        let a104 = v0;
        let a105 = v1;
        nat_sub(a104, a105)
    };
    if nat_lt(a106, a107) {
        1024u64
    } else {
        let a108 = v0;
        let a109 = v1;
        nat_sub(a108, a109)
    }
}

pub fn f23(v0: u64) -> R<u64> {
    let a110 = v0;
    let a111 = 0u64;
    if nat_eq(a110, a111) {
        Ok(1u64)
    } else {
        let a114 = {
            let a112 = v0;
            let a113 = 63u64;
            nat_add(a112, a113)?
        };
        let a115 = 64u64;
        let a116 = 0u64;
        Ok(nat_quot(a114, a115, a116))
    }
}

pub fn f24(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h117, _) = f32(Fn3::F33, 7u64, (Adt2::C0(match v0.clone() {
            Adt0::C0(h118, _, _, _, _, _, _, _) => {
                h118
            }
        }, match v0.clone() {
            Adt0::C0(_, h119, _, _, _, _, _, _) => {
                h119
            }
        }, match v0.clone() {
            Adt0::C0(_, _, h120, _, _, _, _, _) => {
                h120
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, h121, _, _, _, _) => {
                h121
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, h122, _, _, _) => {
                h122
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, h123, _, _) => {
                h123
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h124, _) => {
                h124
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h125) => {
                h125
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f27({
            let a126 = v2;
            let a127 = 4294967296u64;
            let a128 = 0u64;
            nat_rem(a126, a127, a128)
        }), f27({
            let a129 = v2;
            let a130 = 4294967296u64;
            let a131 = 0u64;
            nat_quot(a129, a130, a131)
        }), v3, v4), v1));
        h117
    };
    Adt0::C0({
        let a134 = match v5.clone() {
            Adt2::C0(h132, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h132
            }
        };
        let a135 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h133, _, _, _, _, _, _, _) => {
                h133
            }
        };
        fixed_u32::bit_xor(a134, a135)
    }, {
        let a138 = match v5.clone() {
            Adt2::C0(_, h136, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        };
        let a139 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h137, _, _, _, _, _, _) => {
                h137
            }
        };
        fixed_u32::bit_xor(a138, a139)
    }, {
        let a142 = match v5.clone() {
            Adt2::C0(_, _, h140, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h140
            }
        };
        let a143 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h141, _, _, _, _, _) => {
                h141
            }
        };
        fixed_u32::bit_xor(a142, a143)
    }, {
        let a146 = match v5.clone() {
            Adt2::C0(_, _, _, h144, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h144
            }
        };
        let a147 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h145, _, _, _, _) => {
                h145
            }
        };
        fixed_u32::bit_xor(a146, a147)
    }, {
        let a150 = match v5.clone() {
            Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h149, _, _, _) => {
                h149
            }
        };
        fixed_u32::bit_xor(a150, a151)
    }, {
        let a154 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a155 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h153, _, _) => {
                h153
            }
        };
        fixed_u32::bit_xor(a154, a155)
    }, {
        let a158 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, h156, _, _, _, _, _, _, _, _, _) => {
                h156
            }
        };
        let a159 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h157, _) => {
                h157
            }
        };
        fixed_u32::bit_xor(a158, a159)
    }, {
        let a162 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h160, _, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v5 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h161) => {
                h161
            }
        };
        fixed_u32::bit_xor(a162, a163)
    })
}

pub fn f25(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h164) = f34(Fn4::F35(v2, v0, v1), v3, (0u64, f36()))?;
    Ok(h164)
}

pub fn f26(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(f37(v0.clone(), {
        let a165 = v1;
        let a166 = 0u64;
        nat_add(a165, a166)?
    })?, f37(v0.clone(), {
        let a167 = v1;
        let a168 = 4u64;
        nat_add(a167, a168)?
    })?, f37(v0.clone(), {
        let a169 = v1;
        let a170 = 8u64;
        nat_add(a169, a170)?
    })?, f37(v0.clone(), {
        let a171 = v1;
        let a172 = 12u64;
        nat_add(a171, a172)?
    })?, f37(v0.clone(), {
        let a173 = v1;
        let a174 = 16u64;
        nat_add(a173, a174)?
    })?, f37(v0.clone(), {
        let a175 = v1;
        let a176 = 20u64;
        nat_add(a175, a176)?
    })?, f37(v0.clone(), {
        let a177 = v1;
        let a178 = 24u64;
        nat_add(a177, a178)?
    })?, f37(v0.clone(), {
        let a179 = v1;
        let a180 = 28u64;
        nat_add(a179, a180)?
    })?, f37(v0.clone(), {
        let a181 = v1;
        let a182 = 32u64;
        nat_add(a181, a182)?
    })?, f37(v0.clone(), {
        let a183 = v1;
        let a184 = 36u64;
        nat_add(a183, a184)?
    })?, f37(v0.clone(), {
        let a185 = v1;
        let a186 = 40u64;
        nat_add(a185, a186)?
    })?, f37(v0.clone(), {
        let a187 = v1;
        let a188 = 44u64;
        nat_add(a187, a188)?
    })?, f37(v0.clone(), {
        let a189 = v1;
        let a190 = 48u64;
        nat_add(a189, a190)?
    })?, f37(v0.clone(), {
        let a191 = v1;
        let a192 = 52u64;
        nat_add(a191, a192)?
    })?, f37(v0.clone(), {
        let a193 = v1;
        let a194 = 56u64;
        nat_add(a193, a194)?
    })?, f37(v0, {
        let a195 = v1;
        let a196 = 60u64;
        nat_add(a195, a196)?
    })?))
}

pub fn f27(v0: u64) -> u32 {
    let (_, h197) = {
        let (_, h198) = f38(Fn5::F39, 32u64, (v0, (1u32, 0u32)));
        h198
    };
    h197
}

pub fn f28(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m200 = {
        let (h199, _) = v0.clone();
        h199
    };
    match m200.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f40(v1, {
                let (_, h201) = v0;
                h201
            }, {
                let m202 = v2;
                match m202.uncons() {
                    None => {
                        true
                    }
                    Some((_, _)) => {
                        false
                    }
                }
            }))
        }
    }
}

pub fn f29(v0: u32) -> Bytes {
    let a205 = {
        let a203 = v0;
        let a204 = 8u32;
        fixed_u32::bit_and(a203, a204)
    };
    let a206 = 0u32;
    if equal(a205, a206) {
        let a209 = {
            let a207 = v0;
            let a208 = 4u32;
            fixed_u32::bit_and(a207, a208)
        };
        let a210 = 0u32;
        if equal(a209, a210) {
            let a213 = {
                let a211 = v0;
                let a212 = 2u32;
                fixed_u32::bit_and(a211, a212)
            };
            let a214 = 0u32;
            if equal(a213, a214) {
                let a217 = {
                    let a215 = v0;
                    let a216 = 1u32;
                    fixed_u32::bit_and(a215, a216)
                };
                let a218 = 0u32;
                if equal(a217, a218) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a221 = {
                    let a219 = v0;
                    let a220 = 1u32;
                    fixed_u32::bit_and(a219, a220)
                };
                let a222 = 0u32;
                if equal(a221, a222) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a225 = {
                let a223 = v0;
                let a224 = 2u32;
                fixed_u32::bit_and(a223, a224)
            };
            let a226 = 0u32;
            if equal(a225, a226) {
                let a229 = {
                    let a227 = v0;
                    let a228 = 1u32;
                    fixed_u32::bit_and(a227, a228)
                };
                let a230 = 0u32;
                if equal(a229, a230) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a233 = {
                    let a231 = v0;
                    let a232 = 1u32;
                    fixed_u32::bit_and(a231, a232)
                };
                let a234 = 0u32;
                if equal(a233, a234) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a237 = {
            let a235 = v0;
            let a236 = 4u32;
            fixed_u32::bit_and(a235, a236)
        };
        let a238 = 0u32;
        if equal(a237, a238) {
            let a241 = {
                let a239 = v0;
                let a240 = 2u32;
                fixed_u32::bit_and(a239, a240)
            };
            let a242 = 0u32;
            if equal(a241, a242) {
                let a245 = {
                    let a243 = v0;
                    let a244 = 1u32;
                    fixed_u32::bit_and(a243, a244)
                };
                let a246 = 0u32;
                if equal(a245, a246) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a249 = {
                    let a247 = v0;
                    let a248 = 1u32;
                    fixed_u32::bit_and(a247, a248)
                };
                let a250 = 0u32;
                if equal(a249, a250) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a253 = {
                let a251 = v0;
                let a252 = 2u32;
                fixed_u32::bit_and(a251, a252)
            };
            let a254 = 0u32;
            if equal(a253, a254) {
                let a257 = {
                    let a255 = v0;
                    let a256 = 1u32;
                    fixed_u32::bit_and(a255, a256)
                };
                let a258 = 0u32;
                if equal(a257, a258) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a261 = {
                    let a259 = v0;
                    let a260 = 1u32;
                    fixed_u32::bit_and(a259, a260)
                };
                let a262 = 0u32;
                if equal(a261, a262) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f30(v0: u32, v1: u32) -> u32 {
    let m265 = {
        let a263 = v0;
        let a264 = v1;
        fixed_u32::shift_right(a263, a264)
    };
    match m265 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f31(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f41(Fn6::F42, 40u64, Adt3::C0(std::rc::Rc::new(v1), {
        let a267 = {
            let (h266, _) = v0.clone();
            h266
        };
        let a268 = 1u64;
        nat_add(a267, a268)?
    }, {
        let (_, h269) = v0;
        h269
    }));
    Ok(({
        let a271 = {
            let (h270, _) = v0;
            h270
        };
        let a272 = 1u64;
        nat_add(a271, a272)?
    }, List::cons(match v2.clone() {
        Adt3::C0(h273, _, _) => {
            (*h273).clone()
        }
    }, match v2 {
        Adt3::C0(_, _, h274) => {
            h274
        }
    })))
}

pub fn f32(v0: Fn3, p278: u64, p279: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m277 = (p278, p279);
    {
        let mut m277 = m277;
        loop {
            let step = {
                let (v1, v2) = m277;
                let m275 = v1;
                if m275 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m275 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c276: Fn3 = v0.clone();
                        c276.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f43(v0)
}

pub fn f34(v0: Fn4, p283: u64, p284: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v1;
                if m280 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m280 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c281: Fn4 = v0.clone();
                        c281.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a286 = {
            let (h285, _) = v3.clone();
            h285
        };
        let a287 = 1u64;
        nat_add(a286, a287)?
    }, f24({
        let (_, h288) = v3.clone();
        h288
    }, f26(v1, {
        let a292 = v2;
        let a293 = {
            let a290 = 64u64;
            let a291 = {
                let (h289, _) = v3.clone();
                h289
            };
            nat_mul(a290, a291)?
        };
        nat_add(a292, a293)?
    })?, v0, 64u32, {
        let a295 = {
            let (h294, _) = v3;
            h294
        };
        let a296 = 0u64;
        if nat_eq(a295, a296) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f36() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f37(v0: Bytes, v1: u64) -> R<u32> {
    let a307 = f44(v0.clone(), v1);
    let a308 = {
        let a305 = f45(f44(v0.clone(), {
            let a297 = v1;
            let a298 = 1u64;
            nat_add(a297, a298)?
        }), 8u32);
        let a306 = {
            let a303 = f45(f44(v0.clone(), {
                let a299 = v1;
                let a300 = 2u64;
                nat_add(a299, a300)?
            }), 16u32);
            let a304 = f45(f44(v0, {
                let a301 = v1;
                let a302 = 3u64;
                nat_add(a301, a302)?
            }), 24u32);
            fixed_u32::bit_or(a303, a304)
        };
        fixed_u32::bit_or(a305, a306)
    };
    Ok(fixed_u32::bit_or(a307, a308))
}

pub fn f38(v0: Fn5, p312: u64, p313: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m311 = (p312, p313);
    {
        let mut m311 = m311;
        loop {
            let step = {
                let (v1, v2) = m311;
                let m309 = v1;
                if m309 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m309 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c310: Fn5 = v0.clone();
                        c310.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h314, _) = v0;
        h314
    };
    let v2: u32 = {
        let (h315, _) = {
            let (_, h316) = v0;
            h316
        };
        h315
    };
    let v3: u32 = {
        let (_, h317) = {
            let (_, h318) = v0;
            h318
        };
        h317
    };
    ({
        let a319 = v1;
        let a320 = 2u64;
        let a321 = 0u64;
        nat_quot(a319, a320, a321)
    }, (f45(v2, 1u32), {
        let a325 = {
            let a322 = v1;
            let a323 = 2u64;
            let a324 = 0u64;
            nat_rem(a322, a323, a324)
        };
        let a326 = 1u64;
        if nat_eq(a325, a326) {
            let a327 = v3;
            let a328 = v2;
            fixed_u32::bit_or(a327, a328)
        } else {
            v3
        }
    }))
}

pub fn f40(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f24(f36(), f46(v0, v1), 0u64, 64u32, {
        let a329 = 4u32;
        let a330 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a329, a330)
    })
}

pub fn f41(v0: Fn6, p334: u64, p335: Adt3) -> Adt3 {
    let m333 = (p334, p335);
    {
        let mut m333 = m333;
        loop {
            let step = {
                let (v1, v2) = m333;
                let m331 = v1;
                if m331 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m331 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c332: Fn6 = v0.clone();
                        c332.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Adt3) -> Adt3 {
    f47(v0)
}

pub fn f43(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (f48(f49({
        let (h336, _) = v0.clone();
        h336
    }, {
        let (_, h337) = v0.clone();
        h337
    }), {
        let (_, h338) = v0.clone();
        h338
    }), f50({
        let (_, h339) = v0;
        h339
    }))
}

pub fn f44(v0: Bytes, v1: u64) -> u32 {
    let m342 = {
        let a340 = v0;
        let a341 = v1;
        index_bytes(a340, a341)
    };
    match m342 {
        None => {
            0u32
        }
        Some(v2) => {
            let m344 = {
                let a343 = v2;
                fixed_u32::convert(i128::from(a343))
            };
            match m344 {
                None => {
                    0u32
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f45(v0: u32, v1: u32) -> u32 {
    let m347 = {
        let a345 = v0;
        let a346 = v1;
        fixed_u32::shift_left(a345, a346)
    };
    match m347 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f46(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt0::C0(h348, _, _, _, _, _, _, _) => {
            h348
        }
    }, match v0.clone() {
        Adt0::C0(_, h349, _, _, _, _, _, _) => {
            h349
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h350, _, _, _, _, _) => {
            h350
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h351, _, _, _, _) => {
            h351
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h352, _, _, _) => {
            h352
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h353, _, _) => {
            h353
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h354, _) => {
            h354
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h355) => {
            h355
        }
    }, match v1.clone() {
        Adt0::C0(h356, _, _, _, _, _, _, _) => {
            h356
        }
    }, match v1.clone() {
        Adt0::C0(_, h357, _, _, _, _, _, _) => {
            h357
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h358, _, _, _, _, _) => {
            h358
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h359, _, _, _, _) => {
            h359
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h360, _, _, _) => {
            h360
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h361, _, _) => {
            h361
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h362, _) => {
            h362
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h363) => {
            h363
        }
    })
}

pub fn f47(v0: Adt3) -> Adt3 {
    let a368 = {
        let a365 = match v0.clone() {
            Adt3::C0(_, h364, _) => {
                h364
            }
        };
        let a366 = 2u64;
        let a367 = 0u64;
        nat_rem(a365, a366, a367)
    };
    let a369 = 0u64;
    if nat_eq(a368, a369) {
        let m371 = match v0.clone() {
            Adt3::C0(_, _, h370) => {
                h370
            }
        };
        match m371.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt3::C0(std::rc::Rc::new(f40(v1, match v0.clone() {
                    Adt3::C0(h372, _, _) => {
                        (*h372).clone()
                    }
                }, false)), {
                    let a374 = match v0 {
                        Adt3::C0(_, h373, _) => {
                            h373
                        }
                    };
                    let a375 = 2u64;
                    let a376 = 0u64;
                    nat_quot(a374, a375, a376)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f48(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f51(match v0.clone() {
        Adt2::C0(h377, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h377
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h378, _, _, _, _, _, _, _, _, _, _) => {
            h378
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h379, _, _, _, _, _) => {
            h379
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h380) => {
            h380
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h381, _, _, _, _, _, _, _) => {
            h381
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h382, _, _, _, _, _, _) => {
            h382
        }
    });
    let v3: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, h383, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h383
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h384, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h385, _, _, _, _) => {
            h385
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h386, _, _, _) => {
            h386
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h387, _, _, _, _, _) => {
            h387
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h388, _, _, _, _) => {
            h388
        }
    });
    let v4: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, _, h389, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h389
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h390, _, _, _, _, _, _, _, _) => {
            h390
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h391, _, _, _, _, _, _, _) => {
            h391
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h392, _, _) => {
            h392
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h394, _, _) => {
            h394
        }
    });
    let v5: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, _, _, h395, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h395
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h396, _, _, _, _, _, _, _, _, _, _, _) => {
            h396
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h397, _, _, _, _, _, _) => {
            h397
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h398, _) => {
            h398
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h399, _) => {
            h399
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h400) => {
            h400
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h401, _, _, _) => {
            h401
        }
    }, match v3.clone() {
        Adt4::C0(h402, _, _, _) => {
            h402
        }
    }, match v4.clone() {
        Adt4::C0(h403, _, _, _) => {
            h403
        }
    }, match v5.clone() {
        Adt4::C0(h404, _, _, _) => {
            h404
        }
    }, match v5.clone() {
        Adt4::C0(_, h405, _, _) => {
            h405
        }
    }, match v2.clone() {
        Adt4::C0(_, h406, _, _) => {
            h406
        }
    }, match v3.clone() {
        Adt4::C0(_, h407, _, _) => {
            h407
        }
    }, match v4.clone() {
        Adt4::C0(_, h408, _, _) => {
            h408
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h409, _) => {
            h409
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h410, _) => {
            h410
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h411, _) => {
            h411
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h412, _) => {
            h412
        }
    }, match v3 {
        Adt4::C0(_, _, _, h413) => {
            h413
        }
    }, match v4 {
        Adt4::C0(_, _, _, h414) => {
            h414
        }
    }, match v5 {
        Adt4::C0(_, _, _, h415) => {
            h415
        }
    }, match v2 {
        Adt4::C0(_, _, _, h416) => {
            h416
        }
    })
}

pub fn f49(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f51(match v0.clone() {
        Adt2::C0(h417, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h417
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h418, _, _, _, _, _, _, _, _, _, _, _) => {
            h418
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h419, _, _, _, _, _, _, _) => {
            h419
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h420, _, _, _) => {
            h420
        }
    }, match v1.clone() {
        Adt1::C0(h421, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h421
        }
    }, match v1.clone() {
        Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h422
        }
    });
    let v3: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, h423, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h423
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h424, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h425, _, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h426, _, _) => {
            h426
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h427, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h427
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h428, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    });
    let v4: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, _, h429, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h430, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h431, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h432, _) => {
            h432
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h433, _, _, _, _, _, _, _, _, _, _, _) => {
            h433
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h434, _, _, _, _, _, _, _, _, _, _) => {
            h434
        }
    });
    let v5: Adt4 = f51(match v0.clone() {
        Adt2::C0(_, _, _, h435, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h435
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h436, _, _, _, _, _, _, _, _) => {
            h436
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h437, _, _, _, _) => {
            h437
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h438) => {
            h438
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h439, _, _, _, _, _, _, _, _, _) => {
            h439
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h440, _, _, _, _, _, _, _, _) => {
            h440
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h441, _, _, _) => {
            h441
        }
    }, match v3.clone() {
        Adt4::C0(h442, _, _, _) => {
            h442
        }
    }, match v4.clone() {
        Adt4::C0(h443, _, _, _) => {
            h443
        }
    }, match v5.clone() {
        Adt4::C0(h444, _, _, _) => {
            h444
        }
    }, match v2.clone() {
        Adt4::C0(_, h445, _, _) => {
            h445
        }
    }, match v3.clone() {
        Adt4::C0(_, h446, _, _) => {
            h446
        }
    }, match v4.clone() {
        Adt4::C0(_, h447, _, _) => {
            h447
        }
    }, match v5.clone() {
        Adt4::C0(_, h448, _, _) => {
            h448
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h449, _) => {
            h449
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h450, _) => {
            h450
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h451, _) => {
            h451
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h452, _) => {
            h452
        }
    }, match v2 {
        Adt4::C0(_, _, _, h453) => {
            h453
        }
    }, match v3 {
        Adt4::C0(_, _, _, h454) => {
            h454
        }
    }, match v4 {
        Adt4::C0(_, _, _, h455) => {
            h455
        }
    }, match v5 {
        Adt4::C0(_, _, _, h456) => {
            h456
        }
    })
}

pub fn f50(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h457, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h457
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h458, _, _, _, _, _, _, _, _, _) => {
            h458
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h459, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h459
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h460, _, _, _, _, _) => {
            h460
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h461, _, _, _, _, _, _, _, _) => {
            h461
        }
    }, match v0.clone() {
        Adt1::C0(h462, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h462
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h463, _, _, _, _, _, _, _, _, _, _, _) => {
            h463
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h464, _, _) => {
            h464
        }
    }, match v0.clone() {
        Adt1::C0(_, h465, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h465
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h466, _, _, _, _) => {
            h466
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h467, _, _, _) => {
            h467
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h468, _, _, _, _, _, _, _, _, _, _) => {
            h468
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h469, _, _, _, _, _, _) => {
            h469
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h470, _) => {
            h470
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h471) => {
            h471
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h472, _, _, _, _, _, _, _) => {
            h472
        }
    })
}

pub fn f51(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f52(f52(v0, v1), v4);
    let v7: u32 = f53({
        let a473 = v3;
        let a474 = v6;
        fixed_u32::bit_xor(a473, a474)
    }, 16u32, 16u32);
    let v8: u32 = f52(v2, v7);
    let v9: u32 = f53({
        let a475 = v1;
        let a476 = v8;
        fixed_u32::bit_xor(a475, a476)
    }, 12u32, 20u32);
    let v10: u32 = f52(f52(v6, v9), v5);
    let v11: u32 = f53({
        let a477 = v7;
        let a478 = v10;
        fixed_u32::bit_xor(a477, a478)
    }, 8u32, 24u32);
    let v12: u32 = f52(v8, v11);
    let v13: u32 = f53({
        let a479 = v9;
        let a480 = v12;
        fixed_u32::bit_xor(a479, a480)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f52(v0: u32, v1: u32) -> u32 {
    let m483 = {
        let a481 = f54(v0);
        let a482 = f54(v1);
        fixed_u64::checked_add(a481, a482)
    };
    match m483 {
        None => {
            0u32
        }
        Some(v2) => {
            f55(v2)
        }
    }
}

pub fn f53(v0: u32, v1: u32, v2: u32) -> u32 {
    let a484 = f30(v0, v1);
    let a485 = f45(v0, v2);
    fixed_u32::bit_or(a484, a485)
}

pub fn f54(v0: u32) -> u64 {
    let m487 = {
        let a486 = v0;
        fixed_u64::convert(i128::from(a486))
    };
    match m487 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f55(v0: u64) -> u32 {
    let m491 = {
        let a490 = {
            let a488 = v0;
            let a489 = 4294967295u64;
            fixed_u64::bit_and(a488, a489)
        };
        fixed_u32::convert(i128::from(a490))
    };
    match m491 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}
