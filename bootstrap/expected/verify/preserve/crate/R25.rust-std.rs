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
    C0(Bytes, u64, u64, u64, Option<Adt1>, Option<Bytes>, Option<Bytes>, List<Bytes>, Option<u64>, Option<u64>, std::rc::Rc<Adt3>, List<Adt4>, Option<Adt7>, Option<Adt7>, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(std::rc::Rc<Adt2>),
    C2(Bytes, Bytes, std::rc::Rc<Adt2>),
    C3(Bytes),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C7(Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C11(u64, Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>),
    C4(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes),
    C1(Bytes, Bytes),
    C2(Bytes, Bytes),
    C3(std::rc::Rc<Adt1>, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, u64, Bytes, List<Adt5>, bool, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, u64, u64, u64, u64, u64, u64, Bytes),
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
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Bytes),
    C1(Bytes),
    C2(u64),
    C3(std::rc::Rc<Adt7>),
    C4,
    C5(Bytes),
    C6(std::rc::Rc<Adt7>),
    C7(std::rc::Rc<Adt7>),
    C8(std::rc::Rc<Adt7>),
    C9(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C10(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C11(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C12(Bytes),
    C13,
    C14(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Bytes, List<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(bool, Option<u64>, bool, bool, List<u64>, bool),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<(Bytes, Bytes)>, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(List<(Bytes, Bytes)>, List<Bytes>, Option<Adt1>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<(Bytes, Bytes)>, List<Adt4>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<(Bytes, Bytes)>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<(Bytes, Bytes)>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Bytes>, bool, List<List<(Bytes, Adt17)>>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes),
    C1(bool),
    C2(u64),
    C3(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Adt19>, List<Bytes>, bool, List<(Bytes, Adt17)>, Option<Bytes>, List<Bytes>, bool),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<Bytes>, bool, List<(Bytes, Adt17)>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Bytes, Bytes, Option<Bytes>, Option<List<Bytes>>, u64, Bytes, Option<u64>, Option<Bytes>, Bytes, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, List<Adt21>, bool, Option<Bytes>, Option<Bytes>, bool, Bytes, Option<Adt22>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, Bytes, Bytes, bool, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<(Bytes, Bytes)>, List<Adt4>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(List<(Bytes, Bytes)>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(Bytes, Bytes)>, List<Adt9>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(List<Adt30>, List<Bytes>, u64, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(std::rc::Rc<Adt1>),
    C1(Bytes),
    C2(u64),
    C3(std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(List<u64>, List<Bytes>, List<(Bytes, (Bytes, Bytes))>, List<Bytes>, List<Bytes>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Adt33 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(u64, u64, bool, List<Adt33>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Bytes),
    C1(std::rc::Rc<Adt35>),
    C2,
    C3(std::rc::Rc<Adt35>, std::rc::Rc<Adt35>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(u64, List<List<Adt35>>, Option<Adt35>, Option<Adt28>, bool),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(List<Adt38>, List<Adt1>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(std::rc::Rc<Adt35>),
    C1(std::rc::Rc<Adt35>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(List<Adt7>, std::rc::Rc<Adt31>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(u64, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(List<(Bytes, Bytes)>, Option<u64>, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(List<Adt44>, List<Adt7>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(std::rc::Rc<Adt35>),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(u64, Bytes, u64, bool),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Bytes, Bytes, Option<Adt49>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(std::rc::Rc<Adt51>, List<Adt5>, u64, Option<Adt48>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(std::rc::Rc<Adt39>, u64, List<Adt39>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt6, std::rc::Rc<Adt51>, Option<Adt48>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(u64, Bytes, std::rc::Rc<Adt35>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(List<Adt57>, List<Adt2>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(std::rc::Rc<Adt35>),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F20,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt18, p1: Bytes) -> R<Adt18> {
        match self {
            Fn0::F20 => f20(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, List<Adt16>), p1: Adt19) -> R<(bool, List<Adt16>)> {
        match self {
            Fn1::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F62(List<Bytes>, bool),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt24, p1: Adt21) -> R<Adt24> {
        match self {
            Fn2::F62(k0, k1) => f62(k0.clone(), *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F71,
}

impl Fn3 {
    pub fn apply(&self, p0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
        match self {
            Fn3::F71 => f71(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F75,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt26, p1: Adt23) -> R<Adt26> {
        match self {
            Fn4::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F78,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F81(Bytes),
    F308(u8, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn6::F81(k0) => f81(k0.clone(), p0),
            Fn6::F308(k0, k1) => f308(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F85,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt19) -> List<Adt19> {
        match self {
            Fn7::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F89(List<Bytes>),
    F283(Bytes),
    F389(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn8::F89(k0) => f89(k0.clone(), p0, p1),
            Fn8::F283(k0) => Ok(f283(k0.clone(), p0, p1)),
            Fn8::F389(k0) => Ok(f389(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F93(bool, List<Bytes>),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Adt16>, p1: Adt16) -> R<Option<Adt16>> {
        match self {
            Fn9::F93(k0, k1) => f93(*k0, k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F97,
}

impl Fn10 {
    pub fn apply(&self, p0: Option<List<Adt21>>, p1: List<(Bytes, Adt17)>) -> R<Option<List<Adt21>>> {
        match self {
            Fn10::F97 => f97(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F103(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Option<u64>), p1: Bytes) -> R<(u64, Option<u64>)> {
        match self {
            Fn11::F103(k0) => f103(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F116,
}

impl Fn12 {
    pub fn apply(&self, p0: (Option<Bytes>, bool), p1: Bytes) -> (Option<Bytes>, bool) {
        match self {
            Fn12::F116 => f116(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F118,
    F375(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn13::F118 => f118(p0, p1),
            Fn13::F375(k0) => Ok(f375(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F123(List<Bytes>),
}

impl Fn14 {
    pub fn apply(&self, p0: Adt29) -> R<Option<Adt29>> {
        match self {
            Fn14::F123(k0) => f123(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn15::F130 => f130(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F132,
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt4) -> bool {
        match self {
            Fn16::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F134(u64),
}

impl Fn17 {
    pub fn apply(&self, p0: u64, p1: Adt4) -> R<u64> {
        match self {
            Fn17::F134(k0) => f134(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F136(u64),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn18::F136(k0) => f136(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F146(u64, Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn19::F146(k0, k1) => f146(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F150,
}

impl Fn20 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn20::F150 => f150(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F157,
    F165,
    F236,
}

impl Fn21 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn21::F157 => f157(p0, p1),
            Fn21::F165 => f165(p0, p1),
            Fn21::F236 => f236(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F167,
}

impl Fn22 {
    pub fn apply(&self, p0: List<(Bytes, Adt17)>, p1: (Bytes, Adt17)) -> List<(Bytes, Adt17)> {
        match self {
            Fn22::F167 => f167(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F173,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Adt17)) -> List<Bytes> {
        match self {
            Fn23::F173 => f173(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F175(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: Option<Adt17>, p1: (Bytes, Adt17)) -> Option<Adt17> {
        match self {
            Fn24::F175(k0) => f175(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F180,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt21>, p1: Adt21) -> List<Adt21> {
        match self {
            Fn25::F180 => f180(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F182,
}

impl Fn26 {
    pub fn apply(&self, p0: Option<List<Adt23>>, p1: List<(Bytes, Adt17)>) -> R<Option<List<Adt23>>> {
        match self {
            Fn26::F182 => f182(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F186(u64),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64), p1: u64) -> R<(u64, u64)> {
        match self {
            Fn27::F186(k0) => f186(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F188(bool, u64, Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Adt32) -> R<Option<Adt32>> {
        match self {
            Fn28::F188(k0, k1, k2) => f188(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F191(u64, Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn29::F191(k0, k1) => f191(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F195(u64, Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: Adt34) -> R<Option<Adt34>> {
        match self {
            Fn30::F195(k0, k1) => f195(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F199(u64, List<Adt33>),
}

impl Fn31 {
    pub fn apply(&self, p0: Adt36) -> R<Option<Adt36>> {
        match self {
            Fn31::F199(k0, k1) => f199(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F203(u64),
}

impl Fn32 {
    pub fn apply(&self, p0: Adt37) -> R<Option<Adt37>> {
        match self {
            Fn32::F203(k0) => f203(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F219(u64),
}

impl Fn33 {
    pub fn apply(&self, p0: Adt40) -> R<Option<Adt40>> {
        match self {
            Fn33::F219(k0) => f219(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F223,
}

impl Fn34 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> Bytes {
        match self {
            Fn34::F223 => f223(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F225,
    F294,
}

impl Fn35 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn35::F225 => f225(p0, p1),
            Fn35::F294 => f294(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F227(u64),
}

impl Fn36 {
    pub fn apply(&self, p0: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
        match self {
            Fn36::F227(k0) => f227(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F229(Bytes, List<Adt4>, u64),
    F354(Bytes),
}

impl Fn37 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: Bytes) -> R<List<(Bytes, Bytes)>> {
        match self {
            Fn37::F229(k0, k1, k2) => f229(k0.clone(), k1.clone(), *k2, p0, p1),
            Fn37::F354(k0) => f354(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F238(Bytes, u64),
}

impl Fn38 {
    pub fn apply(&self, p0: Adt41) -> R<Option<Adt41>> {
        match self {
            Fn38::F238(k0, k1) => f238(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F241(Bytes, u64, Bytes, u64),
    F265(u64, Bytes),
    F305(Bytes),
    F306(u64, Bytes),
    F400(u64, Bytes),
    F418(Bytes),
    F517(u64, Bytes),
    F542(u64, Bytes),
    F549(u64, Bytes),
    F553(Bytes),
}

impl Fn39 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn39::F241(k0, k1, k2, k3) => f241(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn39::F265(k0, k1) => f265(*k0, k1.clone(), p0),
            Fn39::F305(k0) => Ok(f305(k0.clone(), p0)),
            Fn39::F306(k0, k1) => f306(*k0, k1.clone(), p0),
            Fn39::F400(k0, k1) => f400(*k0, k1.clone(), p0),
            Fn39::F418(k0) => f418(k0.clone(), p0),
            Fn39::F517(k0, k1) => f517(*k0, k1.clone(), p0),
            Fn39::F542(k0, k1) => f542(*k0, k1.clone(), p0),
            Fn39::F549(k0, k1) => f549(*k0, k1.clone(), p0),
            Fn39::F553(k0) => f553(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F244(Bytes),
}

impl Fn40 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt17)) -> bool {
        match self {
            Fn40::F244(k0) => f244(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F250(bool, List<Bytes>),
}

impl Fn41 {
    pub fn apply(&self, p0: bool, p1: Adt16) -> R<bool> {
        match self {
            Fn41::F250(k0, k1) => f250(*k0, k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F253(bool, List<Bytes>, List<(Bytes, Adt17)>),
    F314,
}

impl Fn42 {
    pub fn apply(&self, p0: List<Adt16>, p1: Adt16) -> R<List<Adt16>> {
        match self {
            Fn42::F253(k0, k1, k2) => f253(*k0, k1.clone(), k2.clone(), p0, p1),
            Fn42::F314 => Ok(f314(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F259,
}

impl Fn43 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt23) -> List<Adt23> {
        match self {
            Fn43::F259 => f259(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F268,
}

impl Fn44 {
    pub fn apply(&self, p0: List<Adt33>, p1: Adt33) -> List<Adt33> {
        match self {
            Fn44::F268 => f268(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F276(Bytes),
}

impl Fn45 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn45::F276(k0) => f276(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F281,
}

impl Fn46 {
    pub fn apply(&self, p0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
        match self {
            Fn46::F281 => f281(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F292(u64),
}

impl Fn47 {
    pub fn apply(&self, p0: Adt43) -> R<Option<Adt43>> {
        match self {
            Fn47::F292(k0) => f292(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn48 {
    F296,
}

impl Fn48 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn48::F296 => f296(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn49 {
    F304,
}

impl Fn49 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, List<Bytes>), p1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
        match self {
            Fn49::F304 => f304(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn50 {
    F310(Bytes, u64),
}

impl Fn50 {
    pub fn apply(&self, p0: Adt45) -> R<Option<Adt45>> {
        match self {
            Fn50::F310(k0, k1) => f310(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn51 {
    F313(Bytes),
}

impl Fn51 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn51::F313(k0) => f313(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn52 {
    F333,
}

impl Fn52 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn52::F333 => f333(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn53 {
    F335(Bytes, u64),
}

impl Fn53 {
    pub fn apply(&self, p0: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
        match self {
            Fn53::F335(k0, k1) => f335(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn54 {
    F349(List<Bytes>, Bytes),
}

impl Fn54 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: Adt5) -> R<List<(Bytes, Bytes)>> {
        match self {
            Fn54::F349(k0, k1) => f349(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn55 {
    F351,
}

impl Fn55 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn55::F351 => f351(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn56 {
    F353(u64),
}

impl Fn56 {
    pub fn apply(&self, p0: (u64, Option<Bytes>), p1: Adt5) -> R<(u64, Option<Bytes>)> {
        match self {
            Fn56::F353(k0) => f353(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn57 {
    F358,
    F409,
    F558,
}

impl Fn57 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn57::F358 => f358(p0, p1),
            Fn57::F409 => f409(p0, p1),
            Fn57::F558 => f558(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn58 {
    F364(u64),
}

impl Fn58 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: u64) -> (bool, List<u64>) {
        match self {
            Fn58::F364(k0) => f364(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn59 {
    F392,
}

impl Fn59 {
    pub fn apply(&self, p0: (Adt47, Adt46)) -> (Adt47, Adt46) {
        match self {
            Fn59::F392 => f392(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn60 {
    F394(u64, Bytes, u64),
}

impl Fn60 {
    pub fn apply(&self, p0: (u64, Adt39)) -> R<(u64, Adt39)> {
        match self {
            Fn60::F394(k0, k1, k2) => f394(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn61 {
    F398,
}

impl Fn61 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn61::F398 => f398(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn62 {
    F402(u64, u64, Bytes, Bytes),
}

impl Fn62 {
    pub fn apply(&self, p0: Adt50) -> R<Option<Adt50>> {
        match self {
            Fn62::F402(k0, k1, k2, k3) => f402(*k0, *k1, k2.clone(), k3.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn63 {
    F411,
}

impl Fn63 {
    pub fn apply(&self, p0: (Adt7, List<Adt7>)) -> Option<(Adt7, List<Adt7>)> {
        match self {
            Fn63::F411 => f411(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn64 {
    F415(Bytes),
}

impl Fn64 {
    pub fn apply(&self, p0: Option<Adt4>, p1: Adt4) -> Option<Adt4> {
        match self {
            Fn64::F415(k0) => f415(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn65 {
    F421,
}

impl Fn65 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt17)>>, p1: List<(Bytes, Adt17)>) -> List<List<(Bytes, Adt17)>> {
        match self {
            Fn65::F421 => f421(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn66 {
    F423,
}

impl Fn66 {
    pub fn apply(&self, p0: Adt35, p1: Adt35) -> Adt35 {
        match self {
            Fn66::F423 => f423(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn67 {
    F426,
}

impl Fn67 {
    pub fn apply(&self, p0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
        match self {
            Fn67::F426 => f426(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn68 {
    F433,
}

impl Fn68 {
    pub fn apply(&self, p0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
        match self {
            Fn68::F433 => f433(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn69 {
    F435,
}

impl Fn69 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn69::F435 => f435(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn70 {
    F437,
}

impl Fn70 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn70::F437 => f437(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn71 {
    F439(u64),
}

impl Fn71 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1), p1: Adt35) -> R<(List<Adt1>, Adt1)> {
        match self {
            Fn71::F439(k0) => f439(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn72 {
    F441,
}

impl Fn72 {
    pub fn apply(&self, p0: Adt52) -> Adt52 {
        match self {
            Fn72::F441 => f441(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn73 {
    F449,
}

impl Fn73 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn73::F449 => f449(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn74 {
    F466,
}

impl Fn74 {
    pub fn apply(&self, p0: List<Adt35>, p1: Adt35) -> List<Adt35> {
        match self {
            Fn74::F466 => f466(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn75 {
    F473,
}

impl Fn75 {
    pub fn apply(&self, p0: List<Adt38>, p1: Adt35) -> List<Adt38> {
        match self {
            Fn75::F473 => f473(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn76 {
    F492,
}

impl Fn76 {
    pub fn apply(&self, p0: (List<Adt7>, List<Adt7>)) -> Option<(List<Adt7>, List<Adt7>)> {
        match self {
            Fn76::F492 => f492(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn77 {
    F494,
}

impl Fn77 {
    pub fn apply(&self, p0: Adt7, p1: Adt7) -> Adt7 {
        match self {
            Fn77::F494 => f494(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn78 {
    F503(u64),
}

impl Fn78 {
    pub fn apply(&self, p0: Result<List<Adt38>, Adt28>, p1: Adt35) -> R<Result<List<Adt38>, Adt28>> {
        match self {
            Fn78::F503(k0) => f503(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn79 {
    F521,
}

impl Fn79 {
    pub fn apply(&self, p0: List<Adt44>, p1: Adt35) -> List<Adt44> {
        match self {
            Fn79::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn80 {
    F523(u64),
}

impl Fn80 {
    pub fn apply(&self, p0: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
        match self {
            Fn80::F523(k0) => f523(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn81 {
    F525(u64),
}

impl Fn81 {
    pub fn apply(&self, p0: Adt56) -> R<Option<Adt56>> {
        match self {
            Fn81::F525(k0) => f525(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn82 {
    F527(u64),
}

impl Fn82 {
    pub fn apply(&self, p0: Result<List<Adt2>, Adt28>, p1: Adt35) -> R<Result<List<Adt2>, Adt28>> {
        match self {
            Fn82::F527(k0) => f527(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn83 {
    F534(u64, u64, Bytes),
}

impl Fn83 {
    pub fn apply(&self, p0: Adt51) -> R<Option<Adt51>> {
        match self {
            Fn83::F534(k0, k1, k2) => f534(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn84 {
    F546,
}

impl Fn84 {
    pub fn apply(&self, p0: Adt2, p1: Adt2) -> Adt2 {
        match self {
            Fn84::F546 => f546(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn85 {
    F548,
}

impl Fn85 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn85::F548 => f548(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn86 {
    F570,
}

impl Fn86 {
    pub fn apply(&self, p0: List<Adt57>, p1: Adt35) -> List<Adt57> {
        match self {
            Fn86::F570 => f570(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn87 {
    F572,
}

impl Fn87 {
    pub fn apply(&self, p0: (List<Adt2>, List<Adt2>)) -> Option<(List<Adt2>, List<Adt2>)> {
        match self {
            Fn87::F572 => f572(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: bool, v2: List<Bytes>, v3: u64) -> R<Result<Adt0, List<(Bytes, Bytes)>>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        None => {
            Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f3(Str::lit("invalid entry file"))), List::<(Bytes, Bytes)>::nil())))
        }
        Some(v4) => {
            let m2 = f4(v4)?;
            match m2 {
                None => {
                    Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f3(Str::lit("invalid entry file"))), List::<(Bytes, Bytes)>::nil())))
                }
                Some(v5) => {
                    let a4 = match v5.clone() {
                        Adt20::C0(h3, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3
                        }
                    };
                    let a5 = f3(Str::lit("lexlean/entry/1"));
                    if !equal(a4, a5) {
                        Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons((f3(Str::lit("LLC0103")), f5(f5(f3(Str::lit("unsupported entry schema ")), f6(match v5 {
                            Adt20::C0(h6, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h6
                            }
                        })), Bytes::lit(&[]))), List::<(Bytes, Bytes)>::nil())))
                    } else {
                        let m8 = f7(0u64, match v5.clone() {
                            Adt20::C0(_, _, h7, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h7
                            }
                        })?;
                        match m8 {
                            None => {
                                Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f5(f6(match v5 {
                                    Adt20::C0(_, _, h9, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h9
                                    }
                                }), f3(Str::lit(" is not a category")))), List::<(Bytes, Bytes)>::nil())))
                            }
                            Some(v6) => {
                                let m11 = f7(1u64, match v5.clone() {
                                    Adt20::C0(_, _, _, _, _, _, h10, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h10
                                    }
                                })?;
                                match m11 {
                                    None => {
                                        Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f5(f6(match v5 {
                                            Adt20::C0(_, _, _, _, _, _, h12, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h12
                                            }
                                        }), f3(Str::lit(" is not a frame")))), List::<(Bytes, Bytes)>::nil())))
                                    }
                                    Some(v7) => {
                                        let v8: Adt10 = f8(v6, v7, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h13, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h13
                                            }
                                        })?;
                                        let v9: Adt11 = f9(match v5.clone() {
                                            Adt20::C0(_, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h14
                                            }
                                        }, v6, v7, match v5.clone() {
                                            Adt20::C0(_, _, h15, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h15
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, h16, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h16
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h17, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h17
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, h18, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h18
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, h19, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h19
                                            }
                                        }, v1)?;
                                        let v10: Adt12 = f10(match v5.clone() {
                                            Adt20::C0(_, h20, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h20
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, h21, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h21
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, h22, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h22
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h23
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, h24, _, _) => {
                                                h24
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h25, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h25
                                            }
                                        }, v3, match v9.clone() {
                                            Adt11::C0(h26, _, _) => {
                                                h26
                                            }
                                        })?;
                                        let v11: (List<(Bytes, Bytes)>, Adt3) = f11(v1, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, h27, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h27
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, h28, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h28
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, h29, _, _, _, _, _, _, _, _, _, _) => {
                                                h29
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, h30, _, _, _, _, _, _, _, _, _) => {
                                                h30
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h31, _, _, _, _, _, _, _, _) => {
                                                h31
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h32, _, _, _, _, _, _, _) => {
                                                h32
                                            }
                                        }, v3, match v10.clone() {
                                            Adt12::C0(_, h33, _, _, _) => {
                                                h33
                                            }
                                        }, match v10.clone() {
                                            Adt12::C0(h34, _, _, _, _) => {
                                                h34
                                            }
                                        })?;
                                        let v12: Adt13 = f12(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h35, _, _, _, _, _, _) => {
                                                h35
                                            }
                                        }, v1, v2, {
                                            let (h36, _) = v11;
                                            h36
                                        })?;
                                        let v13: List<(Bytes, Bytes)> = f13(match v5.clone() {
                                            Adt20::C0(_, _, h37, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h37
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, _, h38) => {
                                                h38
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, h39, _) => {
                                                h39
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(_, h40) => {
                                                h40
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(h41, _) => {
                                                h41
                                            }
                                        })?;
                                        let v14: Adt14 = f14(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h42, _, _, _, _, _) => {
                                                h42
                                            }
                                        }, v6, match v5.clone() {
                                            Adt20::C0(_, _, h43, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h43
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, _, h44) => {
                                                h44
                                            }
                                        }, match v8 {
                                            Adt10::C0(_, _, _, _, h45, _) => {
                                                h45
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h46, _, _, _, _) => {
                                                h46
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h47, _, _, _) => {
                                                h47
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(_, h48) => {
                                                h48
                                            }
                                        }, v3, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h49, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h49
                                            }
                                        }, v7, v13)?;
                                        let v15: (List<(Bytes, Bytes)>, bool) = f15(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h50, _, _) => {
                                                h50
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h51, _) => {
                                                h51
                                            }
                                        }, v6, match v14.clone() {
                                            Adt14::C0(h52, _, _) => {
                                                h52
                                            }
                                        });
                                        let v17: bool = {
                                            let m54 = match v10.clone() {
                                                Adt12::C0(_, _, h53, _, _) => {
                                                    h53
                                                }
                                            };
                                            match m54 {
                                                None => {
                                                    false
                                                }
                                                Some(v16) => {
                                                    let (_, h55) = f16(v16, {
                                                        let a56 = v0;
                                                        length_bytes(a56)
                                                    })?;
                                                    h55
                                                }
                                            }
                                        };
                                        let v18: Adt15 = f17(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h57) => {
                                                h57
                                            }
                                        }, v17, match v5.clone() {
                                            Adt20::C0(_, h58, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h58
                                            }
                                        }, {
                                            let (h59, _) = v15;
                                            h59
                                        })?;
                                        let m61 = match v18.clone() {
                                            Adt15::C0(h60, _) => {
                                                h60
                                            }
                                        };
                                        match m61.uncons() {
                                            None => {
                                                Ok(Ok::<Adt0, List<(Bytes, Bytes)>>(Adt0::C0(match v5.clone() {
                                                    Adt20::C0(_, h63, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        h63
                                                    }
                                                }, v6, v7, match v5 {
                                                    Adt20::C0(_, _, _, _, _, h64, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        h64
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, h65, _, _) => {
                                                        h65
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, _, h66, _) => {
                                                        h66
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, _, _, h67) => {
                                                        h67
                                                    }
                                                }, match v10 {
                                                    Adt12::C0(_, h68, _, _, _) => {
                                                        h68
                                                    }
                                                }, match v9.clone() {
                                                    Adt11::C0(_, h69, _) => {
                                                        h69
                                                    }
                                                }, match v9 {
                                                    Adt11::C0(_, _, h70) => {
                                                        h70
                                                    }
                                                }, std::rc::Rc::new({
                                                    let (_, h71) = v11;
                                                    h71
                                                }), match v12 {
                                                    Adt13::C0(_, h72) => {
                                                        h72
                                                    }
                                                }, match v14.clone() {
                                                    Adt14::C0(_, h73, _) => {
                                                        h73
                                                    }
                                                }, match v14 {
                                                    Adt14::C0(_, _, h74) => {
                                                        h74
                                                    }
                                                }, {
                                                    let (_, h75) = v15;
                                                    h75
                                                }, match v18 {
                                                    Adt15::C0(_, h76) => {
                                                        h76
                                                    }
                                                })))
                                            }
                                            Some((_, _)) => {
                                                Ok(Err::<Adt0, List<(Bytes, Bytes)>>(f18(match v18 {
                                                    Adt15::C0(h62, _) => {
                                                        h62
                                                    }
                                                })))
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

pub fn f1(v0: Bytes) -> R<Option<List<Adt16>>> {
    let v1: Adt18 = f19(Fn0::F20, Adt18::C0(List::<Adt19>::nil(), List::<Bytes>::nil(), false, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false), f21(v0)?)?;
    if match v1.clone() {
        Adt18::C0(_, _, _, _, _, _, h77) => {
            h77
        }
    } {
        Ok(None::<List<Adt16>>)
    } else {
        let m79 = match v1.clone() {
            Adt18::C0(_, _, _, _, h78, _, _) => {
                h78
            }
        };
        match m79 {
            None => {
                let v3: List<Adt19> = f22(f23(match v1.clone() {
                    Adt18::C0(h80, _, _, _, _, _, _) => {
                        h80
                    }
                }, match v1.clone() {
                    Adt18::C0(_, h81, _, _, _, _, _) => {
                        h81
                    }
                }, match v1.clone() {
                    Adt18::C0(_, _, h82, _, _, _, _) => {
                        h82
                    }
                }, match v1 {
                    Adt18::C0(_, _, _, h83, _, _, _) => {
                        h83
                    }
                }));
                let v4: (bool, List<Adt16>) = f24(Fn1::F25, (false, List::<Adt16>::nil()), v3)?;
                let (h84, _) = v4;
                if h84 {
                    Ok(None::<List<Adt16>>)
                } else {
                    Ok(Some({
                        let (_, h85) = v4;
                        h85
                    }))
                }
            }
            Some(_) => {
                Ok(None::<List<Adt16>>)
            }
        }
    }
}

pub fn f2(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLR3004")), v0)
}

pub fn f3(v0: Str) -> Bytes {
    let a86 = v0;
    utf8_encode(a86)
}

pub fn f4(v0: List<Adt16>) -> R<Option<Adt20>> {
    let v1: List<(Bytes, Adt17)> = f26(v0.clone())?;
    if f27(v1.clone(), f28())? {
        let m87 = f29(v1.clone(), f3(Str::lit("spec")));
        match m87 {
            None => {
                Ok(None::<Adt20>)
            }
            Some(v2) => {
                let m88 = f29(v1.clone(), f3(Str::lit("id")));
                match m88 {
                    None => {
                        Ok(None::<Adt20>)
                    }
                    Some(v3) => {
                        let m89 = f29(v1.clone(), f3(Str::lit("category")));
                        match m89 {
                            None => {
                                Ok(None::<Adt20>)
                            }
                            Some(v4) => {
                                let m90 = f30(v1.clone(), f3(Str::lit("signature")));
                                match m90 {
                                    None => {
                                        Ok(None::<Adt20>)
                                    }
                                    Some(v5) => {
                                        let m91 = f31(v1.clone(), f3(Str::lit("universes")));
                                        match m91 {
                                            None => {
                                                Ok(None::<Adt20>)
                                            }
                                            Some(v6) => {
                                                let m92 = f32(v1.clone(), f3(Str::lit("surface_arity")));
                                                match m92 {
                                                    None => {
                                                        Ok(None::<Adt20>)
                                                    }
                                                    Some(v7) => {
                                                        let m93 = f29(v1.clone(), f3(Str::lit("frame")));
                                                        match m93 {
                                                            None => {
                                                                Ok(None::<Adt20>)
                                                            }
                                                            Some(v8) => {
                                                                let m94 = f33(v1.clone(), f3(Str::lit("precedence")));
                                                                match m94 {
                                                                    None => {
                                                                        Ok(None::<Adt20>)
                                                                    }
                                                                    Some(v9) => {
                                                                        let m95 = f30(v1.clone(), f3(Str::lit("associativity")));
                                                                        match m95 {
                                                                            None => {
                                                                                Ok(None::<Adt20>)
                                                                            }
                                                                            Some(v10) => {
                                                                                let m96 = f34(v0.clone(), List::cons(f3(Str::lit("denotation")), List::<Bytes>::nil()), false)?;
                                                                                match m96 {
                                                                                    None => {
                                                                                        Ok(None::<Adt20>)
                                                                                    }
                                                                                    Some(v11) => {
                                                                                        let v12: List<(Bytes, Adt17)> = f35(v11);
                                                                                        let m97 = f36(v12)?;
                                                                                        match m97 {
                                                                                            None => {
                                                                                                Ok(None::<Adt20>)
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let v14: Bytes = match v13.clone() {
                                                                                                    Adt27::C0(h98, _, _, _, _, _) => {
                                                                                                        h98
                                                                                                    }
                                                                                                };
                                                                                                let v15: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, h99, _, _, _, _) => {
                                                                                                        h99
                                                                                                    }
                                                                                                };
                                                                                                let v16: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, h100, _, _, _) => {
                                                                                                        h100
                                                                                                    }
                                                                                                };
                                                                                                let v17: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, _, h101, _, _) => {
                                                                                                        h101
                                                                                                    }
                                                                                                };
                                                                                                let v18: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, _, _, h102, _) => {
                                                                                                        h102
                                                                                                    }
                                                                                                };
                                                                                                let v19: Option<Bytes> = match v13 {
                                                                                                    Adt27::C0(_, _, _, _, _, h103) => {
                                                                                                        h103
                                                                                                    }
                                                                                                };
                                                                                                let m104 = f37(v0.clone(), v1)?;
                                                                                                match m104 {
                                                                                                    None => {
                                                                                                        Ok(None::<Adt20>)
                                                                                                    }
                                                                                                    Some(v20) => {
                                                                                                        let m105 = f38(v20)?;
                                                                                                        match m105 {
                                                                                                            None => {
                                                                                                                Ok(None::<Adt20>)
                                                                                                            }
                                                                                                            Some(v21) => {
                                                                                                                let v23: bool = {
                                                                                                                    let m106 = f34(v0.clone(), List::cons(f3(Str::lit("render")), List::<Bytes>::nil()), false)?;
                                                                                                                    match m106 {
                                                                                                                        None => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        Some(_) => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                let v25: List<(Bytes, Adt17)> = {
                                                                                                                    let m107 = f34(v0.clone(), List::cons(f3(Str::lit("render")), List::<Bytes>::nil()), false)?;
                                                                                                                    match m107 {
                                                                                                                        None => {
                                                                                                                            List::<(Bytes, Adt17)>::nil()
                                                                                                                        }
                                                                                                                        Some(v24) => {
                                                                                                                            f35(v24)
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                let m108 = f30(v25.clone(), f3(Str::lit("math")));
                                                                                                                match m108 {
                                                                                                                    None => {
                                                                                                                        Ok(None::<Adt20>)
                                                                                                                    }
                                                                                                                    Some(v26) => {
                                                                                                                        let m109 = f30(v25, f3(Str::lit("text")));
                                                                                                                        match m109 {
                                                                                                                            None => {
                                                                                                                                Ok(None::<Adt20>)
                                                                                                                            }
                                                                                                                            Some(v27) => {
                                                                                                                                let v29: bool = {
                                                                                                                                    let m110 = f34(v0.clone(), List::cons(f3(Str::lit("calculation")), List::<Bytes>::nil()), false)?;
                                                                                                                                    match m110 {
                                                                                                                                        None => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        Some(_) => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let v31: List<(Bytes, Adt17)> = {
                                                                                                                                    let m111 = f34(v0.clone(), List::cons(f3(Str::lit("calculation")), List::<Bytes>::nil()), false)?;
                                                                                                                                    match m111 {
                                                                                                                                        None => {
                                                                                                                                            List::<(Bytes, Adt17)>::nil()
                                                                                                                                        }
                                                                                                                                        Some(v30) => {
                                                                                                                                            f35(v30)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let m112 = if v29 {
                                                                                                                                    f29(v31, f3(Str::lit("kind")))
                                                                                                                                } else {
                                                                                                                                    Some(Bytes::lit(&[]))
                                                                                                                                };
                                                                                                                                match m112 {
                                                                                                                                    None => {
                                                                                                                                        Ok(None::<Adt20>)
                                                                                                                                    }
                                                                                                                                    Some(v32) => {
                                                                                                                                        let m113 = f39(v0)?;
                                                                                                                                        match m113 {
                                                                                                                                            None => {
                                                                                                                                                Ok(None::<Adt20>)
                                                                                                                                            }
                                                                                                                                            Some(v33) => {
                                                                                                                                                Ok(Some(Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9, v10, v14, v15, v16, v17, v18, v19, v21, v23, v26, v27, v29, v32, v33)))
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
    } else {
        Ok(None::<Adt20>)
    }
}

pub fn f5(v0: Bytes, v1: Bytes) -> Bytes {
    let a114 = v0;
    let a115 = v1;
    append_bytes(a114, a115)
}

pub fn f6(v0: Bytes) -> Bytes {
    f40(f3(Str::lit("`")), v0, f3(Str::lit("`")))
}

pub fn f7(v0: u64, v1: Bytes) -> R<Option<u64>> {
    f41(f42(v0), v1)
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<Adt10> {
    Ok(Adt10::C0({
        let a116 = v1;
        let a117 = f43(v0, v2)?;
        nat_eq(a116, a117)
    }, f44(v1), {
        let a118 = v0;
        let a119 = 2u64;
        nat_lt(a118, a119)
    }, {
        let a120 = 2u64;
        let a121 = v0;
        nat_lt(a120, a121)
    }, f45(v0)?, {
        let a122 = v0;
        let a123 = 3u64;
        if nat_eq(a122, a123) {
            true
        } else {
            let a124 = v0;
            let a125 = 4u64;
            nat_eq(a124, a125)
        }
    }))
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: Bytes, v4: Bytes, v5: u64, v6: Option<u64>, v7: Option<Bytes>, v8: bool) -> R<Adt11> {
    let v9: Adt10 = f8(v1, v2, v5)?;
    let v10: List<(Bytes, Bytes)> = f46(List::<(Bytes, Bytes)>::nil(), if f47(v0.clone())? {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f6(v0), f3(Str::lit(" is not a valid entry ID")))))
    });
    let v11: List<(Bytes, Bytes)> = f46(v10, if if match v9.clone() {
        Adt10::C0(_, _, h126, _, _, _) => {
            h126
        }
    } {
        !v8
    } else {
        false
    } {
        Some(f2(f40(f3(Str::lit("a non-core entry cannot use category `")), v3.clone(), f3(Str::lit("`")))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v12: List<(Bytes, Bytes)> = f46(v11, if match v9.clone() {
        Adt10::C0(h127, _, _, _, _, _) => {
            h127
        }
    } {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f5(f5(f5(f3(Str::lit("category `")), v3), f3(Str::lit("` does not permit frame `"))), v4.clone()), f3(Str::lit("`")))))
    });
    let v14: List<(Bytes, Bytes)> = f46(v12, {
        let m129 = match v9 {
            Adt10::C0(_, h128, _, _, _, _) => {
                h128
            }
        };
        match m129 {
            None => {
                let a132 = v5;
                let a133 = 0u64;
                if nat_eq(a132, a133) {
                    Some(f2(f3(Str::lit("a call frame needs surface_arity >= 1"))))
                } else {
                    None::<(Bytes, Bytes)>
                }
            }
            Some(v13) => {
                let a130 = v5;
                let a131 = v13;
                if nat_eq(a130, a131) {
                    None::<(Bytes, Bytes)>
                } else {
                    Some(f2(f5(f5(f5(f5(f3(Str::lit("frame `")), v4), f3(Str::lit("` fixes surface_arity "))), f48(v13)), f5(f3(Str::lit(", found ")), f48(v5)))))
                }
            }
        }
    });
    let v15: bool = {
        let a134 = v2;
        let a135 = 2u64;
        if nat_eq(a134, a135) {
            true
        } else {
            let a136 = v2;
            let a137 = 3u64;
            if nat_eq(a136, a137) {
                true
            } else {
                let a138 = v2;
                let a139 = 4u64;
                nat_eq(a138, a139)
            }
        }
    };
    let v17: (List<(Bytes, Bytes)>, Option<u64>) = {
        let m140 = v6;
        match m140 {
            None => {
                if v15 {
                    (f46(v14, Some(f2(f3(Str::lit("this frame requires a precedence"))))), None::<u64>)
                } else {
                    (v14, None::<u64>)
                }
            }
            Some(v16) => {
                if v15 {
                    let a141 = v16;
                    let a142 = 255u64;
                    if nat_le(a141, a142) {
                        (v14, Some(v16))
                    } else {
                        (f46(v14, Some(f2(f3(Str::lit("precedence is 0..255"))))), None::<u64>)
                    }
                } else {
                    (f46(v14, Some(f2(f3(Str::lit("precedence does not apply to this frame"))))), None::<u64>)
                }
            }
        }
    };
    let v18: List<(Bytes, Bytes)> = {
        let (h143, _) = v17;
        h143
    };
    let v19: Option<u64> = {
        let (_, h144) = v17;
        h144
    };
    let v20: bool = {
        let a145 = v2;
        let a146 = 4u64;
        nat_eq(a145, a146)
    };
    let v22: (List<(Bytes, Bytes)>, Option<u64>) = {
        let m147 = v7;
        match m147 {
            None => {
                if v20 {
                    (f46(v18, Some(f2(f3(Str::lit("an infix frame requires an associativity"))))), None::<u64>)
                } else {
                    (v18, None::<u64>)
                }
            }
            Some(v21) => {
                if v20 {
                    let a148 = v21.clone();
                    let a149 = f3(Str::lit("left"));
                    if equal(a148, a149) {
                        (v18, Some(0u64))
                    } else {
                        let a150 = v21.clone();
                        let a151 = f3(Str::lit("right"));
                        if equal(a150, a151) {
                            (v18, Some(1u64))
                        } else {
                            let a152 = v21;
                            let a153 = f3(Str::lit("none"));
                            if equal(a152, a153) {
                                (v18, Some(2u64))
                            } else {
                                (f46(v18, Some(f2(f3(Str::lit("associativity is left, right, or none"))))), None::<u64>)
                            }
                        }
                    }
                } else {
                    (f46(v18, Some(f2(f3(Str::lit("associativity applies only to infix frames"))))), None::<u64>)
                }
            }
        }
    };
    Ok(Adt11::C0({
        let (h154, _) = v22;
        h154
    }, v19, {
        let (_, h155) = v22;
        h155
    }))
}

pub fn f10(v0: Bytes, v1: Bytes, v2: Option<List<Bytes>>, v3: Option<Bytes>, v4: bool, v5: u64, v6: u64, v7: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v9: List<Bytes> = {
        let m156 = v2.clone();
        match m156 {
            None => {
                List::<Bytes>::nil()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v10: List<(Bytes, Bytes)> = f46(v7, if f49(v9.clone()) {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f3(Str::lit("universes must be sorted and unique"))))
    });
    let v12: List<(Bytes, Bytes)> = {
        let m157 = f50(v9.clone())?;
        match m157 {
            None => {
                v10
            }
            Some(v11) => {
                f46(v10, Some(f2(f5(f6(v11), f3(Str::lit(" is not a universe name"))))))
            }
        }
    };
    let m158 = v3;
    match m158 {
        None => {
            if v4 {
                Ok(Adt12::C0(f46(v12, Some(f2(f3(Str::lit("every semantic entry has a complete LSE signature"))))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            } else {
                Ok(Adt12::C0(f46(v12, {
                    let m167 = v2;
                    match m167 {
                        None => {
                            None::<(Bytes, Bytes)>
                        }
                        Some(_) => {
                            Some(f2(f3(Str::lit("universes apply only with a signature"))))
                        }
                    }
                }), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            }
        }
        Some(v13) => {
            if v4 {
                let m159 = f51(v13.clone(), v6)?;
                match m159 {
                    Ok(v14) => {
                        let v15: u64 = {
                            let a160 = v13;
                            length_bytes(a160)
                        };
                        let v17: List<(Bytes, Bytes)> = {
                            let m161 = f52(v14.clone(), v9.clone(), v15)?;
                            match m161 {
                                None => {
                                    v12
                                }
                                Some(v16) => {
                                    f46(v12, Some(f2(f5(f3(Str::lit("invalid signature: ")), v16))))
                                }
                            }
                        };
                        let v18: (u64, bool) = f16(v14.clone(), v15)?;
                        let v19: u64 = {
                            let (h162, _) = v18;
                            h162
                        };
                        let v20: List<(Bytes, Bytes)> = {
                            let a163 = v5;
                            let a164 = v19;
                            if nat_eq(a163, a164) {
                                v17
                            } else {
                                f46(v17, Some(f2(f40(f40(f6(v0), f3(Str::lit(": surface_arity ")), f48(v5)), f3(Str::lit(" does not match the ")), f5(f48(v19), {
                                    let a165 = v19;
                                    let a166 = 1u64;
                                    if nat_eq(a165, a166) {
                                        f3(Str::lit(" explicit binder of the signature's outer pi"))
                                    } else {
                                        f3(Str::lit(" explicit binders of the signature's outer pi"))
                                    }
                                })))))
                            }
                        };
                        let v21: Bytes = f53(v14.clone(), true)?;
                        let v22: Bytes = f54(v21.clone())?;
                        Ok(Adt12::C0(v20, v9, Some(v14), Some(v21), Some(v22)))
                    }
                    Err(v23) => {
                        Ok(Adt12::C0(f46(v12, Some(f55(f3(Str::lit("signature")), v23))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
                    }
                }
            } else {
                Ok(Adt12::C0(f46(v12, Some(f2(f40(f3(Str::lit("category `")), v1, f3(Str::lit("` does not carry a signature")))))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            }
        }
    }
}

pub fn f11(v0: bool, v1: Bytes, v2: Option<Bytes>, v3: Option<Bytes>, v4: Option<Bytes>, v5: Option<Bytes>, v6: Option<Bytes>, v7: u64, v8: List<Bytes>, v9: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v10: u64 = f56(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())?;
    let a168 = v1.clone();
    let a169 = f3(Str::lit("core"));
    if equal(a168, a169) {
        f57(v0, v2, v10, v9)
    } else {
        let a170 = v1.clone();
        let a171 = f3(Str::lit("lean"));
        if equal(a170, a171) {
            f58(v3, v4, v10, v9)
        } else {
            let a172 = v1.clone();
            let a173 = f3(Str::lit("document"));
            if equal(a172, a173) {
                Ok(f59(v3, v5, v10, v9))
            } else {
                let a174 = v1.clone();
                let a175 = f3(Str::lit("defined"));
                if equal(a174, a175) {
                    f60(v6, v10, v7, v8, v9)
                } else {
                    Ok((f46(v9, Some(f2(f5(f6(v1), f3(Str::lit(" is not a denotation kind")))))), Adt3::C0(Bytes::lit(&[]))))
                }
            }
        }
    }
}

pub fn f12(v0: List<Adt21>, v1: bool, v2: List<Bytes>, v3: List<(Bytes, Bytes)>) -> R<Adt13> {
    let v6: List<(Bytes, Bytes)> = f46(v3, {
        let m176 = v0.clone();
        match m176.uncons() {
            None => {
                Some(f2(f3(Str::lit("an entry declares at least one form"))))
            }
            Some((_, _)) => {
                None::<(Bytes, Bytes)>
            }
        }
    });
    let v7: Adt24 = f61(Fn2::F62(v2, v1), Adt24::C0(v6, List::<Adt4>::nil(), List::<Bytes>::nil()), v0)?;
    Ok(Adt13::C0(match v7.clone() {
        Adt24::C0(h177, _, _) => {
            h177
        }
    }, f63(match v7 {
        Adt24::C0(_, h178, _) => {
            h178
        }
    })))
}

pub fn f13(v0: Bytes, v1: bool, v2: List<u64>, v3: List<Adt4>, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v5: List<(Bytes, Bytes)> = f46(v4, if if v1 {
        !f64(v3.clone())
    } else {
        false
    } {
        Some(f2(f5(f5(f3(Str::lit("category `")), v0.clone()), f3(Str::lit("` requires a canonical source form in the text or the math channel")))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v6: List<(Bytes, Bytes)> = f65(v0.clone(), v2.clone(), v3.clone(), 0u64, f3(Str::lit("text")), v5)?;
    f65(v0, v2, v3, 1u64, f3(Str::lit("math")), v6)
}

pub fn f14(v0: bool, v1: u64, v2: Bytes, v3: bool, v4: List<u64>, v5: Option<Bytes>, v6: Option<Bytes>, v7: List<Adt4>, v8: u64, v9: u64, v10: u64, v11: List<(Bytes, Bytes)>) -> R<Adt14> {
    let v12: List<(Bytes, Bytes)> = if v0 {
        f46(v11, {
            let a179 = v1;
            let a180 = 0u64;
            if if nat_eq(a179, a180) {
                true
            } else {
                let a181 = v1;
                let a182 = 1u64;
                nat_eq(a181, a182)
            } {
                Some(f2(f3(Str::lit("structural and grammar entries do not carry render templates"))))
            } else {
                None::<(Bytes, Bytes)>
            }
        })
    } else {
        v11
    };
    let v13: bool = if v3 {
        let a183 = 0u64;
        let a184 = f66(v7.clone(), 0u64)?;
        nat_lt(a183, a184)
    } else {
        false
    };
    let v14: List<(Bytes, Bytes)> = if v0 {
        f46(v12, if if f67(v6.clone()) {
            if !f68(v4, 0u64) {
                !v13
            } else {
                false
            }
        } else {
            false
        } {
            Some(f2(f5(f5(f3(Str::lit("category `")), v2), f3(Str::lit("` has no text canonical form, so a text render does not apply")))))
        } else {
            None::<(Bytes, Bytes)>
        })
    } else {
        v12
    };
    let v15: Adt25 = f69(f3(Str::lit("math")), 1u64, v5, v7.clone(), v8, v9, v14)?;
    let v16: Adt25 = f69(f3(Str::lit("text")), 0u64, v6, v7, v8, v9, match v15.clone() {
        Adt25::C0(h185, _) => {
            h185
        }
    })?;
    let v17: bool = {
        let a186 = v10;
        let a187 = 1u64;
        if nat_eq(a186, a187) {
            true
        } else {
            let a188 = v10;
            let a189 = 2u64;
            if nat_eq(a188, a189) {
                true
            } else {
                let a190 = v10;
                let a191 = 3u64;
                if nat_eq(a190, a191) {
                    true
                } else {
                    let a192 = v10;
                    let a193 = 4u64;
                    nat_eq(a192, a193)
                }
            }
        }
    };
    let v19: List<(Bytes, Bytes)> = f46(match v16.clone() {
        Adt25::C0(h194, _) => {
            h194
        }
    }, if if v17 {
        let m196 = match v15.clone() {
            Adt25::C0(_, h195) => {
                h195
            }
        };
        match m196 {
            None => {
                true
            }
            Some(_) => {
                false
            }
        }
    } else {
        false
    } {
        Some(f2(f3(Str::lit("a math frame with surface arguments requires a canonical math render"))))
    } else {
        None::<(Bytes, Bytes)>
    });
    Ok(Adt14::C0(v19, match v15 {
        Adt25::C0(_, h197) => {
            h197
        }
    }, match v16 {
        Adt25::C0(_, h198) => {
            h198
        }
    }))
}

pub fn f15(v0: bool, v1: Bytes, v2: u64, v3: List<(Bytes, Bytes)>) -> (List<(Bytes, Bytes)>, bool) {
    if !v0 {
        (v3, false)
    } else {
        let v4: List<(Bytes, Bytes)> = f46(v3, {
            let a199 = v1;
            let a200 = f3(Str::lit("equality"));
            if equal(a199, a200) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f3(Str::lit("language 1.0 ships exactly the equality calculation descriptor"))))
            }
        });
        let v5: List<(Bytes, Bytes)> = f46(v4, {
            let a201 = v2;
            let a202 = 15u64;
            if nat_eq(a201, a202) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f3(Str::lit("a calculation descriptor belongs to an infix relation"))))
            }
        });
        (v5, true)
    }
}

pub fn f16(v0: Adt1, v1: u64) -> R<(u64, bool)> {
    let m203 = v0;
    match m203 {
        Adt1::C0 => {
            Ok((0u64, true))
        }
        Adt1::C1(_) => {
            Ok((0u64, true))
        }
        Adt1::C2(_, _, _) => {
            Ok((0u64, false))
        }
        Adt1::C3(_) => {
            Ok((0u64, false))
        }
        Adt1::C4(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C5(r208, r209) => {
            let v2 = (*r208).clone();
            let v3 = (*r209).clone();
            Ok(({
                let (_, h204) = {
                    let (h205, _) = f70(Fn3::F71, {
                        let a206 = v1;
                        let a207 = 1u64;
                        nat_add(a206, a207)?
                    }, (v2, 0u64))?;
                    h205
                };
                h204
            }, f72(v3)))
        }
        Adt1::C6(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C7(_, _, _, _) => {
            Ok((0u64, false))
        }
        Adt1::C8(_) => {
            Ok((0u64, false))
        }
        Adt1::C9 => {
            Ok((0u64, false))
        }
        Adt1::C10(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((0u64, false))
        }
    }
}

pub fn f17(v0: Option<Adt22>, v1: bool, v2: Bytes, v3: List<(Bytes, Bytes)>) -> R<Adt15> {
    let m210 = v0;
    match m210 {
        None => {
            Ok(Adt15::C0(v3, None::<Adt8>))
        }
        Some(v4) => {
            let v5: List<(Bytes, Bytes)> = f46(v3, if v1 {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f6(v2), f3(Str::lit(": an eliminator descriptor belongs to a type entry, whose signature results in a sort"))), Bytes::lit(&[]))))
            });
            let v6: List<(Bytes, Bytes)> = f46(v5, if f73(match v4.clone() {
                Adt22::C0(h211, _, _) => {
                    h211
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f3(Str::lit("eliminator cases_lean_name ")), f6(match v4.clone() {
                    Adt22::C0(h212, _, _) => {
                        h212
                    }
                })), f3(Str::lit(" is not a Lean name")))))
            });
            let v7: List<(Bytes, Bytes)> = f46(v6, if f73(match v4.clone() {
                Adt22::C0(_, h213, _) => {
                    h213
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f3(Str::lit("eliminator induction_lean_name ")), f6(match v4.clone() {
                    Adt22::C0(_, h214, _) => {
                        h214
                    }
                })), f3(Str::lit(" is not a Lean name")))))
            });
            let v8: Adt26 = f74(Fn4::F75, Adt26::C0(v7, List::<Adt9>::nil(), List::<Bytes>::nil()), match v4.clone() {
                Adt22::C0(_, _, h215) => {
                    h215
                }
            })?;
            let v9: List<Adt9> = f76(match v8.clone() {
                Adt26::C0(_, h216, _) => {
                    h216
                }
            });
            let v10: List<(Bytes, Bytes)> = f46(match v8 {
                Adt26::C0(h217, _, _) => {
                    h217
                }
            }, {
                let a219 = {
                    let a218 = v9.clone();
                    length_list(a218)
                };
                let a220 = 0u64;
                if nat_eq(a219, a220) {
                    Some(f2(f3(Str::lit("an eliminator needs at least one constructor"))))
                } else {
                    None::<(Bytes, Bytes)>
                }
            });
            Ok(Adt15::C0(v10, Some(Adt8::C0(match v4.clone() {
                Adt22::C0(h221, _, _) => {
                    h221
                }
            }, match v4 {
                Adt22::C0(_, h222, _) => {
                    h222
                }
            }, v9))))
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f77(Fn5::F78, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f19(v0: Fn0, p226: Adt18, p227: List<Bytes>) -> R<Adt18> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(Adt18, List<Bytes>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Bytes>), R<Adt18>>(({
                            let c224: Fn0 = v0.clone();
                            c224.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt18, v1: Bytes) -> R<Adt18> {
    f79(v0, v1)
}

pub fn f21(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a228 = v0.clone();
        length_bytes(a228)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h229, _) = f80(Fn6::F81(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h229
    };
    let v3: u64 = {
        let (h230, _) = {
            let (_, h231) = v2.clone();
            h231
        };
        h230
    };
    let v4: List<Bytes> = {
        let (_, h232) = {
            let (_, h233) = v2;
            h233
        };
        h232
    };
    let a234 = v3;
    let a235 = v1;
    if nat_lt(a234, a235) {
        Ok(f82(List::cons(f83(v0, v3, {
            let a236 = v1;
            let a237 = v3;
            nat_sub(a236, a237)
        }), v4)))
    } else {
        Ok(f82(v4))
    }
}

pub fn f22(v0: List<Adt19>) -> List<Adt19> {
    f84(Fn7::F85, List::<Adt19>::nil(), v0)
}

pub fn f23(v0: List<Adt19>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> List<Adt19> {
    List::cons(Adt19::C0(v1, v2, f86(v3)), v0)
}

pub fn f24(v0: Fn1, p241: (bool, List<Adt16>), p242: List<Adt19>) -> R<(bool, List<Adt16>)> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v1, v2) = m240;
                let m238 = v2;
                match m238.uncons() {
                    None => {
                        Err::<((bool, List<Adt16>), List<Adt19>), R<(bool, List<Adt16>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt16>), List<Adt19>), R<(bool, List<Adt16>)>>(({
                            let c239: Fn1 = v0.clone();
                            c239.apply(v1, v3)?
                        }, v4))
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

pub fn f25(v0: (bool, List<Adt16>), v1: Adt19) -> R<(bool, List<Adt16>)> {
    f87(v0, v1)
}

pub fn f26(v0: List<Adt16>) -> R<List<(Bytes, Adt17)>> {
    let m243 = f34(v0, List::<Bytes>::nil(), false)?;
    match m243 {
        None => {
            Ok(List::<(Bytes, Adt17)>::nil())
        }
        Some(v1) => {
            Ok(f35(v1))
        }
    }
}

pub fn f27(v0: List<(Bytes, Adt17)>, v1: List<Bytes>) -> R<bool> {
    f88(Fn8::F89(v1), true, f90(v0))
}

pub fn f28() -> List<Bytes> {
    List::cons(f3(Str::lit("spec")), List::cons(f3(Str::lit("id")), List::cons(f3(Str::lit("category")), List::cons(f3(Str::lit("signature")), List::cons(f3(Str::lit("universes")), List::cons(f3(Str::lit("surface_arity")), List::cons(f3(Str::lit("frame")), List::cons(f3(Str::lit("precedence")), List::cons(f3(Str::lit("associativity")), List::cons(f3(Str::lit("form")), List::<Bytes>::nil()))))))))))
}

pub fn f29(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Bytes> {
    let m244 = f91(v0, v1);
    match m244 {
        None => {
            None::<Bytes>
        }
        Some(v2) => {
            let m245 = v2;
            match m245 {
                Adt17::C0(v3) => {
                    Some(v3)
                }
                Adt17::C1(_) => {
                    None::<Bytes>
                }
                Adt17::C2(_) => {
                    None::<Bytes>
                }
                Adt17::C3(_) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f30(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m246 = f91(v0, v1);
    match m246 {
        None => {
            Some(None::<Bytes>)
        }
        Some(v2) => {
            let m247 = v2;
            match m247 {
                Adt17::C0(v3) => {
                    Some(Some(v3))
                }
                Adt17::C1(_) => {
                    None::<Option<Bytes>>
                }
                Adt17::C2(_) => {
                    None::<Option<Bytes>>
                }
                Adt17::C3(_) => {
                    None::<Option<Bytes>>
                }
            }
        }
    }
}

pub fn f31(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<List<Bytes>>> {
    let m248 = f91(v0, v1);
    match m248 {
        None => {
            Some(None::<List<Bytes>>)
        }
        Some(v2) => {
            let m249 = v2;
            match m249 {
                Adt17::C0(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C1(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C2(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C3(v6) => {
                    Some(Some(v6))
                }
            }
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<u64> {
    let m250 = f91(v0, v1);
    match m250 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let m251 = v2;
            match m251 {
                Adt17::C0(_) => {
                    None::<u64>
                }
                Adt17::C1(_) => {
                    None::<u64>
                }
                Adt17::C2(v5) => {
                    Some(v5)
                }
                Adt17::C3(_) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<u64>> {
    let m252 = f91(v0, v1);
    match m252 {
        None => {
            Some(None::<u64>)
        }
        Some(v2) => {
            let m253 = v2;
            match m253 {
                Adt17::C0(_) => {
                    None::<Option<u64>>
                }
                Adt17::C1(_) => {
                    None::<Option<u64>>
                }
                Adt17::C2(v5) => {
                    Some(Some(v5))
                }
                Adt17::C3(_) => {
                    None::<Option<u64>>
                }
            }
        }
    }
}

pub fn f34(v0: List<Adt16>, v1: List<Bytes>, v2: bool) -> R<Option<Adt16>> {
    f92(Fn9::F93(v2, v1), None::<Adt16>, v0)
}

pub fn f35(v0: Adt16) -> List<(Bytes, Adt17)> {
    let m255 = match v0 {
        Adt16::C0(_, _, h254) => {
            h254
        }
    };
    match m255.uncons() {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f36(v0: List<(Bytes, Adt17)>) -> R<Option<Adt27>> {
    if f27(v0.clone(), f94())? {
        let m256 = f29(v0.clone(), f3(Str::lit("kind")));
        match m256 {
            None => {
                Ok(None::<Adt27>)
            }
            Some(v1) => {
                let m257 = f30(v0.clone(), f3(Str::lit("constructor")));
                match m257 {
                    None => {
                        Ok(None::<Adt27>)
                    }
                    Some(v2) => {
                        let m258 = f30(v0.clone(), f3(Str::lit("module")));
                        match m258 {
                            None => {
                                Ok(None::<Adt27>)
                            }
                            Some(v3) => {
                                let m259 = f30(v0.clone(), f3(Str::lit("name")));
                                match m259 {
                                    None => {
                                        Ok(None::<Adt27>)
                                    }
                                    Some(v4) => {
                                        let m260 = f30(v0.clone(), f3(Str::lit("component")));
                                        match m260 {
                                            None => {
                                                Ok(None::<Adt27>)
                                            }
                                            Some(v5) => {
                                                let m261 = f30(v0, f3(Str::lit("value")));
                                                match m261 {
                                                    None => {
                                                        Ok(None::<Adt27>)
                                                    }
                                                    Some(v6) => {
                                                        Ok(Some(Adt27::C0(v1, v2, v3, v4, v5, v6)))
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
    } else {
        Ok(None::<Adt27>)
    }
}

pub fn f37(v0: List<Adt16>, v1: List<(Bytes, Adt17)>) -> R<Option<List<List<(Bytes, Adt17)>>>> {
    let m262 = f34(v0, List::cons(f3(Str::lit("form")), List::<Bytes>::nil()), true)?;
    match m262 {
        None => {
            Ok(f95(v1, f3(Str::lit("form"))))
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt16::C0(_, _, h263) => {
                    h263
                }
            }))
        }
    }
}

pub fn f38(v0: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt21>>> {
    let m264 = f96(Fn10::F97, Some(List::<Adt21>::nil()), v0)?;
    match m264 {
        None => {
            Ok(None::<List<Adt21>>)
        }
        Some(v1) => {
            Ok(Some(f98(v1)))
        }
    }
}

pub fn f39(v0: List<Adt16>) -> R<Option<Option<Adt22>>> {
    let m265 = f34(v0.clone(), List::cons(f3(Str::lit("eliminator")), List::<Bytes>::nil()), false)?;
    match m265 {
        None => {
            Ok(Some(None::<Adt22>))
        }
        Some(v1) => {
            let v2: List<(Bytes, Adt17)> = f35(v1);
            if f27(v2.clone(), f99())? {
                let m266 = f29(v2.clone(), f3(Str::lit("cases_lean_name")));
                match m266 {
                    None => {
                        Ok(None::<Option<Adt22>>)
                    }
                    Some(v3) => {
                        let m267 = f29(v2.clone(), f3(Str::lit("induction_lean_name")));
                        match m267 {
                            None => {
                                Ok(None::<Option<Adt22>>)
                            }
                            Some(v4) => {
                                let m268 = f100(v0, v2)?;
                                match m268 {
                                    None => {
                                        Ok(None::<Option<Adt22>>)
                                    }
                                    Some(v5) => {
                                        let m269 = f101(v5)?;
                                        match m269 {
                                            None => {
                                                Ok(None::<Option<Adt22>>)
                                            }
                                            Some(v6) => {
                                                Ok(Some(Some(Adt22::C0(v3, v4, v6))))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Option<Adt22>>)
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f5(f5(v0, v1), v2)
}

pub fn f41(v0: List<Bytes>, v1: Bytes) -> R<Option<u64>> {
    let (_, h270) = f102(Fn11::F103(v1), (0u64, None::<u64>), v0)?;
    Ok(h270)
}

pub fn f42(v0: u64) -> List<Bytes> {
    let a271 = v0;
    let a272 = 0u64;
    if nat_eq(a271, a272) {
        f104()
    } else {
        let a273 = v0;
        let a274 = 1u64;
        if nat_eq(a273, a274) {
            f105()
        } else {
            let a275 = v0;
            let a276 = 2u64;
            if nat_eq(a275, a276) {
                f106()
            } else {
                let a277 = v0;
                let a278 = 3u64;
                if nat_eq(a277, a278) {
                    f107()
                } else {
                    f108()
                }
            }
        }
    }
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    let a279 = v0;
    let a280 = 16u64;
    if nat_eq(a279, a280) {
        let a281 = v1;
        let a282 = 0u64;
        if nat_eq(a281, a282) {
            Ok(0u64)
        } else {
            Ok(1u64)
        }
    } else {
        f109(f110(), v0)
    }
}

pub fn f44(v0: u64) -> Option<u64> {
    let a283 = v0;
    let a284 = 1u64;
    if nat_eq(a283, a284) {
        None::<u64>
    } else {
        let a285 = v0;
        let a286 = 0u64;
        if nat_eq(a285, a286) {
            Some(0u64)
        } else {
            let a287 = v0;
            let a288 = 4u64;
            if if nat_eq(a287, a288) {
                true
            } else {
                let a289 = v0;
                let a290 = 6u64;
                if nat_eq(a289, a290) {
                    true
                } else {
                    let a291 = v0;
                    let a292 = 9u64;
                    nat_eq(a291, a292)
                }
            } {
                Some(2u64)
            } else {
                Some(1u64)
            }
        }
    }
}

pub fn f45(v0: u64) -> R<List<u64>> {
    let v1: u64 = f109(f111(), v0)?;
    let a293 = v1;
    let a294 = 1u64;
    if nat_eq(a293, a294) {
        Ok(List::cons(0u64, List::<u64>::nil()))
    } else {
        let a295 = v1;
        let a296 = 2u64;
        if nat_eq(a295, a296) {
            Ok(List::cons(1u64, List::<u64>::nil()))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f46(v0: List<(Bytes, Bytes)>, v1: Option<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m297 = v1;
    match m297 {
        None => {
            v0
        }
        Some(v2) => {
            List::cons(v2, v0)
        }
    }
}

pub fn f47(v0: Bytes) -> R<bool> {
    f112(v0, true)
}

pub fn f48(v0: u64) -> Bytes {
    f113(f114(), 10u64, v0)
}

pub fn f49(v0: List<Bytes>) -> bool {
    let (_, h298) = f115(Fn12::F116, (None::<Bytes>, true), v0);
    h298
}

pub fn f50(v0: List<Bytes>) -> R<Option<Bytes>> {
    f117(Fn13::F118, None::<Bytes>, v0)
}

pub fn f51(v0: Bytes, v1: u64) -> R<Result<Adt1, Adt28>> {
    let m299 = f119(v0)?;
    match m299 {
        Ok(v3) => {
            let m300 = f120(v3.clone(), v1)?;
            match m300 {
                Ok(v5) => {
                    f121(v5, {
                        let a301 = v3;
                        length_list(a301)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt1, Adt28>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt1, Adt28>(v2))
        }
    }
}

pub fn f52(v0: Adt1, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    let m308 = {
        let (h303, _) = f122(Fn14::F123(v1), {
            let a306 = {
                let a304 = v2;
                let a305 = 4u64;
                nat_mul(a304, a305)?
            };
            let a307 = 4u64;
            nat_add(a306, a307)?
        }, Adt29::C0(List::cons(Adt30::C0(std::rc::Rc::new(v0)), List::<Adt30>::nil()), List::<Bytes>::nil(), 0u64, None::<Bytes>))?;
        h303
    };
    match m308 {
        Adt29::C0(_, _, _, h302) => {
            Ok(h302)
        }
    }
}

pub fn f53(v0: Adt1, v1: bool) -> R<Bytes> {
    let (h309, _) = f124(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h309)
}

pub fn f54(v0: Bytes) -> R<Bytes> {
    let a311 = {
        let a310 = Str::lit("blake3:");
        utf8_encode(a310)
    };
    let a312 = f125(v0)?;
    Ok(append_bytes(a311, a312))
}

pub fn f55(v0: Bytes, v1: Adt28) -> (Bytes, Bytes) {
    let a314 = match v1.clone() {
        Adt28::C0(h313, _) => {
            h313
        }
    };
    let a315 = 1u64;
    if nat_eq(a314, a315) {
        f126(f5(f5(v0, f3(Str::lit(": "))), match v1 {
            Adt28::C0(_, h316) => {
                h316
            }
        }))
    } else {
        f2(f5(f5(f5(f3(Str::lit("invalid ")), v0), f3(Str::lit(" LSE: "))), match v1 {
            Adt28::C0(_, h317) => {
                h317
            }
        }))
    }
}

pub fn f56(v0: Option<Bytes>, v1: Option<Bytes>, v2: Option<Bytes>, v3: Option<Bytes>, v4: Option<Bytes>) -> R<u64> {
    let a324 = {
        let a322 = {
            let a320 = {
                let a318 = f127(v0);
                let a319 = f127(v1);
                nat_add(a318, a319)?
            };
            let a321 = f127(v2);
            nat_add(a320, a321)?
        };
        let a323 = f127(v3);
        nat_add(a322, a323)?
    };
    let a325 = f127(v4);
    nat_add(a324, a325)
}

pub fn f57(v0: bool, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v4: List<(Bytes, Bytes)> = f46(v3, if v0 {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f3(Str::lit("only lexlean.core may use a core denotation"))))
    });
    let v6: bool = {
        let m326 = v1.clone();
        match m326 {
            None => {
                false
            }
            Some(v5) => {
                let a327 = v2;
                let a328 = 1u64;
                if nat_eq(a327, a328) {
                    f47(v5)?
                } else {
                    false
                }
            }
        }
    };
    if v6 {
        Ok((v4, Adt3::C0({
            let m329 = v1;
            match m329 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        })))
    } else {
        Ok((f46(v4, Some(f2(f3(Str::lit("a core denotation has exactly `constructor`"))))), Adt3::C0(Bytes::lit(&[]))))
    }
}

pub fn f58(v0: Option<Bytes>, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v6: bool = {
        let m330 = v0.clone();
        match m330 {
            None => {
                false
            }
            Some(v4) => {
                let m331 = v1.clone();
                match m331 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        let a332 = v2;
                        let a333 = 2u64;
                        if nat_eq(a332, a333) {
                            if f73(v4)? {
                                f73(v5)?
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                }
            }
        }
    };
    if v6 {
        Ok((v3, Adt3::C1({
            let m334 = v0;
            match m334 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        }, {
            let m335 = v1;
            match m335 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v8) => {
                    v8
                }
            }
        })))
    } else {
        Ok((f46(v3, Some(f2(f3(Str::lit("a lean denotation has exactly `module` and `name` in the conservative ASCII Lean-name grammar"))))), Adt3::C1(Bytes::lit(&[]), Bytes::lit(&[]))))
    }
}

pub fn f59(v0: Option<Bytes>, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> (List<(Bytes, Bytes)>, Adt3) {
    let v6: bool = {
        let m336 = v0.clone();
        match m336 {
            None => {
                false
            }
            Some(_) => {
                let m337 = v1.clone();
                match m337 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let a338 = v2;
                        let a339 = 2u64;
                        nat_eq(a338, a339)
                    }
                }
            }
        }
    };
    if v6 {
        (v3, Adt3::C2({
            let m340 = v0;
            match m340 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        }, {
            let m341 = v1;
            match m341 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v8) => {
                    v8
                }
            }
        }))
    } else {
        (f46(v3, Some(f2(f3(Str::lit("a document denotation has exactly `module` and `component`"))))), Adt3::C2(Bytes::lit(&[]), Bytes::lit(&[])))
    }
}

pub fn f60(v0: Option<Bytes>, v1: u64, v2: u64, v3: List<Bytes>, v4: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let m342 = v0;
    match m342 {
        None => {
            Ok((f46(v4, Some(f2(f3(Str::lit("a defined denotation has exactly `value`"))))), Adt3::C3(std::rc::Rc::new(Adt1::C0), Bytes::lit(&[]))))
        }
        Some(v5) => {
            let a343 = v1;
            let a344 = 1u64;
            if nat_eq(a343, a344) {
                let m345 = f51(v5.clone(), v2)?;
                match m345 {
                    Ok(v6) => {
                        let v7: u64 = {
                            let a346 = v5;
                            length_bytes(a346)
                        };
                        let v9: List<(Bytes, Bytes)> = {
                            let m347 = f52(v6.clone(), v3, v7)?;
                            match m347 {
                                None => {
                                    v4
                                }
                                Some(v8) => {
                                    f46(v4, Some(f2(f5(f3(Str::lit("invalid defined value: ")), v8))))
                                }
                            }
                        };
                        Ok((v9, Adt3::C3(std::rc::Rc::new(v6.clone()), f53(v6, true)?)))
                    }
                    Err(v10) => {
                        Ok((f46(v4, Some(f55(f3(Str::lit("defined value")), v10))), Adt3::C3(std::rc::Rc::new(Adt1::C0), Bytes::lit(&[]))))
                    }
                }
            } else {
                Ok((f46(v4, Some(f2(f3(Str::lit("a defined denotation has exactly `value`"))))), Adt3::C3(std::rc::Rc::new(Adt1::C0), Bytes::lit(&[]))))
            }
        }
    }
}

pub fn f61(v0: Fn2, p351: Adt24, p352: List<Adt21>) -> R<Adt24> {
    let m350 = (p351, p352);
    {
        let mut m350 = m350;
        loop {
            let step = {
                let (v1, v2) = m350;
                let m348 = v2;
                match m348.uncons() {
                    None => {
                        Err::<(Adt24, List<Adt21>), R<Adt24>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt24, List<Adt21>), R<Adt24>>(({
                            let c349: Fn2 = v0.clone();
                            c349.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<Bytes>, v1: bool, v2: Adt24, v3: Adt21) -> R<Adt24> {
    f128(v1, v0, v2, v3)
}

pub fn f63(v0: List<Adt4>) -> List<Adt4> {
    f129(Fn15::F130, List::<Adt4>::nil(), v0)
}

pub fn f64(v0: List<Adt4>) -> bool {
    f131(Fn16::F132, false, v0)
}

pub fn f65(v0: Bytes, v1: List<u64>, v2: List<Adt4>, v3: u64, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v6: u64 = f66(v2, v3)?;
    let v7: List<(Bytes, Bytes)> = f46(v5, {
        let a353 = 1u64;
        let a354 = v6;
        if nat_lt(a353, a354) {
            Some(f2(f5(f3(Str::lit("more than one canonical source form for the ")), f5(v4.clone(), f3(Str::lit(" channel"))))))
        } else {
            None::<(Bytes, Bytes)>
        }
    });
    Ok(f46(v7, if if f68(v1, v3) {
        let a355 = v6;
        let a356 = 0u64;
        nat_eq(a355, a356)
    } else {
        false
    } {
        Some(f2(f5(f5(f3(Str::lit("category `")), v0), f5(f3(Str::lit("` requires a canonical source form in the ")), f5(v4, f3(Str::lit(" channel")))))))
    } else {
        None::<(Bytes, Bytes)>
    }))
}

pub fn f66(v0: List<Adt4>, v1: u64) -> R<u64> {
    f133(Fn17::F134(v1), 0u64, v0)
}

pub fn f67(v0: Option<Bytes>) -> bool {
    let m357 = v0;
    match m357 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f68(v0: List<u64>, v1: u64) -> bool {
    f135(Fn18::F136(v1), false, v0)
}

pub fn f69(v0: Bytes, v1: u64, v2: Option<Bytes>, v3: List<Adt4>, v4: u64, v5: u64, v6: List<(Bytes, Bytes)>) -> R<Adt25> {
    let m358 = v2;
    match m358 {
        None => {
            Ok(Adt25::C0(v6, None::<Adt7>))
        }
        Some(v7) => {
            let m359 = f137(v7.clone(), v4)?;
            match m359 {
                Ok(v9) => {
                    let v10: u64 = {
                        let a361 = {
                            let a360 = v7;
                            length_bytes(a360)
                        };
                        let a362 = 2u64;
                        nat_add(a361, a362)?
                    };
                    let v11: Adt31 = f139(v9.clone(), v10)?;
                    let v12: List<(Bytes, Bytes)> = f46(v6, {
                        let a364 = f140(f141(match v11.clone() {
                            Adt31::C0(h363, _, _, _, _, _, _) => {
                                h363
                            }
                        }));
                        let a365 = f140(f142(v5)?);
                        if equal(a364, a365) {
                            None::<(Bytes, Bytes)>
                        } else {
                            Some(f2(f5(f5(v0.clone(), f3(Str::lit(" render must use every slot 0.."))), f5(f48(v5), f3(Str::lit(" exactly once"))))))
                        }
                    });
                    let v14: List<(Bytes, Bytes)> = {
                        let m367 = match v11.clone() {
                            Adt31::C0(_, _, _, _, _, _, h366) => {
                                h366
                            }
                        };
                        match m367 {
                            None => {
                                v12
                            }
                            Some(v13) => {
                                f46(v12, Some(f2(f5(f5(v0.clone(), f3(Str::lit(" render: "))), v13))))
                            }
                        }
                    };
                    let v16: List<(Bytes, Bytes)> = {
                        let a368 = v1;
                        let a369 = 0u64;
                        if nat_eq(a368, a369) {
                            let m371 = match v11.clone() {
                                Adt31::C0(_, _, _, _, _, h370, _) => {
                                    h370
                                }
                            };
                            match m371 {
                                None => {
                                    v14
                                }
                                Some(v15) => {
                                    f46(v14, Some(f2(f5(f5(f3(Str::lit("text render uses `")), v15), f3(Str::lit("`, a mathematical construct with no text rendering"))))))
                                }
                            }
                        } else {
                            v14
                        }
                    };
                    let v17: List<(Bytes, Bytes)> = f143(match v11 {
                        Adt31::C0(_, _, _, h372, _, _, _) => {
                            h372
                        }
                    }, v3, v1, v0, v16)?;
                    Ok(Adt25::C0(v17, Some(v9)))
                }
                Err(v8) => {
                    Ok(Adt25::C0(f46(v6, Some(f138(v0, v8))), None::<Adt7>))
                }
            }
        }
    }
}

pub fn f70(v0: Fn3, p377: u64, p378: (Adt1, u64)) -> R<((Adt1, u64), bool)> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v1, v2) = m376;
                let m373 = v1;
                if m373 == 0 {
                    Err::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m373 - 1;
                    let m375 = {
                        let c374: Fn3 = v0.clone();
                        c374.apply(v2.clone())?
                    };
                    match m375 {
                        None => {
                            Err::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
    f144(v0)
}

pub fn f72(v0: Adt1) -> bool {
    let m379 = v0;
    match m379 {
        Adt1::C0 => {
            true
        }
        Adt1::C1(_) => {
            true
        }
        Adt1::C2(_, _, _) => {
            false
        }
        Adt1::C3(_) => {
            false
        }
        Adt1::C4(_, _) => {
            false
        }
        Adt1::C5(_, _) => {
            false
        }
        Adt1::C6(_, _) => {
            false
        }
        Adt1::C7(_, _, _, _) => {
            false
        }
        Adt1::C8(_) => {
            false
        }
        Adt1::C9 => {
            false
        }
        Adt1::C10(_, _) => {
            false
        }
        Adt1::C11(_, _, _, _) => {
            false
        }
    }
}

pub fn f73(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a380 = v0.clone();
        length_bytes(a380)
    };
    let v2: (u64, bool) = {
        let (h381, _) = f145(Fn19::F146(v1, v0), {
            let a382 = v1;
            let a383 = 1u64;
            nat_add(a382, a383)?
        }, (0u64, true))?;
        h381
    };
    let a385 = {
        let (h384, _) = v2;
        h384
    };
    let a386 = v1;
    if nat_eq(a385, a386) {
        Ok(!f147(v2))
    } else {
        Ok(false)
    }
}

pub fn f74(v0: Fn4, p390: Adt26, p391: List<Adt23>) -> R<Adt26> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v1, v2) = m389;
                let m387 = v2;
                match m387.uncons() {
                    None => {
                        Err::<(Adt26, List<Adt23>), R<Adt26>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt26, List<Adt23>), R<Adt26>>(({
                            let c388: Fn4 = v0.clone();
                            c388.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Adt26, v1: Adt23) -> R<Adt26> {
    f148(v0, v1)
}

pub fn f76(v0: List<Adt9>) -> List<Adt9> {
    f149(Fn20::F150, List::<Adt9>::nil(), v0)
}

pub fn f77(v0: Fn5, p395: List<(Bytes, Bytes)>, p396: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m392 = v2;
                match m392.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c393: Fn5 = v0.clone();
                            c393.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f79(v0: Adt18, v1: Bytes) -> R<Adt18> {
    if match v0.clone() {
        Adt18::C0(_, _, _, _, _, _, h397) => {
            h397
        }
    } {
        Ok(v0)
    } else {
        let m399 = match v0.clone() {
            Adt18::C0(_, _, _, _, h398, _, _) => {
                h398
            }
        };
        match m399 {
            None => {
                if f158(v1.clone())? {
                    Ok(v0)
                } else {
                    let v8: Bytes = f151(v1)?;
                    if f159(v8.clone(), f152(Str::lit("["))) {
                        let v9: List<Adt19> = f23(match v0.clone() {
                            Adt18::C0(h426, _, _, _, _, _, _) => {
                                h426
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h427, _, _, _, _, _) => {
                                h427
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h428, _, _, _, _) => {
                                h428
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h429, _, _, _) => {
                                h429
                            }
                        });
                        let m430 = f160(v8)?;
                        match m430 {
                            None => {
                                Ok(Adt18::C0(v9, List::<Bytes>::nil(), false, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v10) => {
                                Ok(Adt18::C0(v9, {
                                    let (h431, _) = v10;
                                    h431
                                }, {
                                    let (_, h432) = v10;
                                    h432
                                }, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false))
                            }
                        }
                    } else {
                        let m433 = f161(v8.clone(), f152(Str::lit("=")))?;
                        match m433 {
                            None => {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h434, _, _, _, _, _, _) => {
                                        h434
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h435, _, _, _, _, _) => {
                                        h435
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h436, _, _, _, _) => {
                                        h436
                                    }
                                }, match v0 {
                                    Adt18::C0(_, _, _, h437, _, _, _) => {
                                        h437
                                    }
                                }, None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v11) => {
                                let v12: Bytes = f151(f83(v8.clone(), 0u64, v11))?;
                                let v13: Bytes = f151(f83(v8.clone(), {
                                    let a438 = v11;
                                    let a439 = 1u64;
                                    nat_add(a438, a439)?
                                }, {
                                    let a443 = {
                                        let a440 = v8;
                                        length_bytes(a440)
                                    };
                                    let a444 = {
                                        let a441 = v11;
                                        let a442 = 1u64;
                                        nat_add(a441, a442)?
                                    };
                                    nat_sub(a443, a444)
                                }))?;
                                let a446 = {
                                    let a445 = v12.clone();
                                    length_bytes(a445)
                                };
                                let a447 = 0u64;
                                if if nat_eq(a446, a447) {
                                    true
                                } else {
                                    let m448 = f161(v12.clone(), f152(Str::lit(".")))?;
                                    match m448 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                } {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h449, _, _, _, _, _, _) => {
                                            h449
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h450, _, _, _, _, _) => {
                                            h450
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h451, _, _, _, _) => {
                                            h451
                                        }
                                    }, match v0 {
                                        Adt18::C0(_, _, _, h452, _, _, _) => {
                                            h452
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                } else {
                                    if f162(match v0.clone() {
                                        Adt18::C0(_, _, _, h453, _, _, _) => {
                                            h453
                                        }
                                    }, v12.clone()) {
                                        Ok(Adt18::C0(match v0.clone() {
                                            Adt18::C0(h454, _, _, _, _, _, _) => {
                                                h454
                                            }
                                        }, match v0.clone() {
                                            Adt18::C0(_, h455, _, _, _, _, _) => {
                                                h455
                                            }
                                        }, match v0.clone() {
                                            Adt18::C0(_, _, h456, _, _, _, _) => {
                                                h456
                                            }
                                        }, match v0 {
                                            Adt18::C0(_, _, _, h457, _, _, _) => {
                                                h457
                                            }
                                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                                    } else {
                                        f163(v0, v12, v13)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(v2) => {
                let v3: Bytes = f151(v1)?;
                let a400 = v3.clone();
                let a401 = f152(Str::lit("]"));
                if equal(a400, a401) {
                    Ok(Adt18::C0(match v0.clone() {
                        Adt18::C0(h402, _, _, _, _, _, _) => {
                            h402
                        }
                    }, match v0.clone() {
                        Adt18::C0(_, h403, _, _, _, _, _) => {
                            h403
                        }
                    }, match v0.clone() {
                        Adt18::C0(_, _, h404, _, _, _, _) => {
                            h404
                        }
                    }, List::cons((v2, Adt17::C3(f153(match v0.clone() {
                        Adt18::C0(_, _, _, _, _, h405, _) => {
                            h405
                        }
                    }))), match v0 {
                        Adt18::C0(_, _, _, h406, _, _, _) => {
                            h406
                        }
                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                } else {
                    let v4: bool = f154(v3.clone(), f152(Str::lit("]")));
                    let v5: Bytes = if v4 {
                        f83(v3.clone(), 0u64, {
                            let a408 = {
                                let a407 = v3;
                                length_bytes(a407)
                            };
                            let a409 = 1u64;
                            nat_sub(a408, a409)
                        })
                    } else {
                        v3
                    };
                    let m410 = f155(v5)?;
                    match m410 {
                        None => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h420, _, _, _, _, _, _) => {
                                    h420
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h421, _, _, _, _, _) => {
                                    h421
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h422, _, _, _, _) => {
                                    h422
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, _, h423, _, _, _) => {
                                    h423
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, _, _, h424, _, _) => {
                                    h424
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, _, _, h425, _) => {
                                    h425
                                }
                            }, true))
                        }
                        Some(v6) => {
                            let v7: List<Bytes> = f156(Fn21::F157, match v0.clone() {
                                Adt18::C0(_, _, _, _, _, h411, _) => {
                                    h411
                                }
                            }, v6);
                            if v4 {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h412, _, _, _, _, _, _) => {
                                        h412
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h413, _, _, _, _, _) => {
                                        h413
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h414, _, _, _, _) => {
                                        h414
                                    }
                                }, List::cons((v2, Adt17::C3(f153(v7))), match v0 {
                                    Adt18::C0(_, _, _, h415, _, _, _) => {
                                        h415
                                    }
                                }), None::<Bytes>, List::<Bytes>::nil(), false))
                            } else {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h416, _, _, _, _, _, _) => {
                                        h416
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h417, _, _, _, _, _) => {
                                        h417
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h418, _, _, _, _) => {
                                        h418
                                    }
                                }, match v0 {
                                    Adt18::C0(_, _, _, h419, _, _, _) => {
                                        h419
                                    }
                                }, Some(v2), v7, false))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f80(v0: Fn6, p462: u64, p463: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m461 = (p462, p463);
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v1, v2) = m461;
                let m458 = v1;
                if m458 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m458 - 1;
                    let m460 = {
                        let c459: Fn6 = v0.clone();
                        c459.apply(v2.clone())?
                    };
                    match m460 {
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
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f164(v0, v1)
}

pub fn f82(v0: List<Bytes>) -> List<Bytes> {
    f156(Fn21::F165, List::<Bytes>::nil(), v0)
}

pub fn f83(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m467 = {
        let a464 = v0;
        let a465 = v1;
        let a466 = v2;
        slice_bytes(a464, a465, a466)
    };
    match m467 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f84(v0: Fn7, p471: List<Adt19>, p472: List<Adt19>) -> List<Adt19> {
    let m470 = (p471, p472);
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v1, v2) = m470;
                let m468 = v2;
                match m468.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>(({
                            let c469: Fn7 = v0.clone();
                            c469.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt19>, v1: Adt19) -> List<Adt19> {
    List::cons(v1, v0)
}

pub fn f86(v0: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    f166(Fn22::F167, List::<(Bytes, Adt17)>::nil(), v0)
}

pub fn f87(v0: (bool, List<Adt16>), v1: Adt19) -> R<(bool, List<Adt16>)> {
    let (h473, _) = v0.clone();
    if h473 {
        Ok(v0)
    } else {
        let v2: List<Bytes> = match v1.clone() {
            Adt19::C0(h474, _, _) => {
                h474
            }
        };
        let v3: bool = match v1.clone() {
            Adt19::C0(_, h475, _) => {
                h475
            }
        };
        let v4: List<(Bytes, Adt17)> = match v1 {
            Adt19::C0(_, _, h476) => {
                h476
            }
        };
        let v5: bool = f168({
            let (_, h477) = v0.clone();
            h477
        }, v2.clone(), v3)?;
        if v5 {
            if v3 {
                Ok((false, f169({
                    let (_, h478) = v0;
                    h478
                }, v2, v3, v4)?))
            } else {
                Ok((true, {
                    let (_, h479) = v0;
                    h479
                }))
            }
        } else {
            Ok((false, f170({
                let (_, h480) = v0;
                h480
            }, v2, v3, v4)?))
        }
    }
}

pub fn f88(v0: Fn8, p484: bool, p485: List<Bytes>) -> R<bool> {
    let m483 = (p484, p485);
    {
        let mut m483 = m483;
        loop {
            let step = {
                let (v1, v2) = m483;
                let m481 = v2;
                match m481.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c482: Fn8 = v0.clone();
                            c482.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Bytes>, v1: bool, v2: Bytes) -> R<bool> {
    if v1 {
        f171(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f90(v0: List<(Bytes, Adt17)>) -> List<Bytes> {
    f153(f172(Fn23::F173, List::<Bytes>::nil(), v0))
}

pub fn f91(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Adt17> {
    f174(Fn24::F175(v1), None::<Adt17>, v0)
}

pub fn f92(v0: Fn9, p489: Option<Adt16>, p490: List<Adt16>) -> R<Option<Adt16>> {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v1, v2) = m488;
                let m486 = v2;
                match m486.uncons() {
                    None => {
                        Err::<(Option<Adt16>, List<Adt16>), R<Option<Adt16>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt16>, List<Adt16>), R<Option<Adt16>>>(({
                            let c487: Fn9 = v0.clone();
                            c487.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: bool, v1: List<Bytes>, v2: Option<Adt16>, v3: Adt16) -> R<Option<Adt16>> {
    let m491 = v2.clone();
    match m491 {
        None => {
            if f176(v3.clone(), v1, v0)? {
                Ok(Some(v3))
            } else {
                Ok(None::<Adt16>)
            }
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f94() -> List<Bytes> {
    List::cons(f3(Str::lit("kind")), List::cons(f3(Str::lit("constructor")), List::cons(f3(Str::lit("module")), List::cons(f3(Str::lit("name")), List::cons(f3(Str::lit("component")), List::cons(f3(Str::lit("value")), List::<Bytes>::nil()))))))
}

pub fn f95(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<List<List<(Bytes, Adt17)>>> {
    let m492 = f177(v0, v1);
    match m492 {
        None => {
            None::<List<List<(Bytes, Adt17)>>>
        }
        Some(v2) => {
            let m493 = v2;
            match m493.uncons() {
                None => {
                    Some(List::<List<(Bytes, Adt17)>>::nil())
                }
                Some((_, _)) => {
                    None::<List<List<(Bytes, Adt17)>>>
                }
            }
        }
    }
}

pub fn f96(v0: Fn10, p497: Option<List<Adt21>>, p498: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt21>>> {
    let m496 = (p497, p498);
    {
        let mut m496 = m496;
        loop {
            let step = {
                let (v1, v2) = m496;
                let m494 = v2;
                match m494.uncons() {
                    None => {
                        Err::<(Option<List<Adt21>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt21>>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt21>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt21>>>>(({
                            let c495: Fn10 = v0.clone();
                            c495.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Option<List<Adt21>>, v1: List<(Bytes, Adt17)>) -> R<Option<List<Adt21>>> {
    let m499 = v0;
    match m499 {
        None => {
            Ok(None::<List<Adt21>>)
        }
        Some(v2) => {
            let m500 = f178(v1)?;
            match m500 {
                None => {
                    Ok(None::<List<Adt21>>)
                }
                Some(v3) => {
                    Ok(Some(List::cons(v3, v2)))
                }
            }
        }
    }
}

pub fn f98(v0: List<Adt21>) -> List<Adt21> {
    f179(Fn25::F180, List::<Adt21>::nil(), v0)
}

pub fn f99() -> List<Bytes> {
    List::cons(f3(Str::lit("cases_lean_name")), List::cons(f3(Str::lit("induction_lean_name")), List::cons(f3(Str::lit("constructor")), List::<Bytes>::nil())))
}

pub fn f100(v0: List<Adt16>, v1: List<(Bytes, Adt17)>) -> R<Option<List<List<(Bytes, Adt17)>>>> {
    let m501 = f34(v0, List::cons(f3(Str::lit("eliminator")), List::cons(f3(Str::lit("constructor")), List::<Bytes>::nil())), true)?;
    match m501 {
        None => {
            Ok(f95(v1, f3(Str::lit("constructor"))))
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt16::C0(_, _, h502) => {
                    h502
                }
            }))
        }
    }
}

pub fn f101(v0: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt23>>> {
    let m503 = f181(Fn26::F182, Some(List::<Adt23>::nil()), v0)?;
    match m503 {
        None => {
            Ok(None::<List<Adt23>>)
        }
        Some(v1) => {
            Ok(Some(f183(v1)))
        }
    }
}

pub fn f102(v0: Fn11, p507: (u64, Option<u64>), p508: List<Bytes>) -> R<(u64, Option<u64>)> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m504 = v2;
                match m504.uncons() {
                    None => {
                        Err::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(({
                            let c505: Fn11 = v0.clone();
                            c505.apply(v1, v3)?
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

pub fn f103(v0: Bytes, v1: (u64, Option<u64>), v2: Bytes) -> R<(u64, Option<u64>)> {
    let m510 = {
        let (_, h509) = v1;
        h509
    };
    match m510 {
        None => {
            let a511 = v2;
            let a512 = v0;
            if equal(a511, a512) {
                Ok(({
                    let (h513, _) = v1;
                    h513
                }, Some({
                    let (h514, _) = v1;
                    h514
                })))
            } else {
                Ok(({
                    let a516 = {
                        let (h515, _) = v1;
                        h515
                    };
                    let a517 = 1u64;
                    nat_add(a516, a517)?
                }, None::<u64>))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f104() -> List<Bytes> {
    List::cons(f184(Str::lit("structural")), List::cons(f184(Str::lit("grammar")), List::cons(f184(Str::lit("label-word")), List::cons(f184(Str::lit("type-noun")), List::cons(f184(Str::lit("term-constant")), List::cons(f184(Str::lit("function")), List::cons(f184(Str::lit("prefix-function")), List::cons(f184(Str::lit("postfix-function")), List::cons(f184(Str::lit("infix-function")), List::cons(f184(Str::lit("noun-function")), List::cons(f184(Str::lit("binary-noun-function")), List::cons(f184(Str::lit("predicate-constant")), List::cons(f184(Str::lit("adjective-predicate")), List::cons(f184(Str::lit("intransitive-predicate")), List::cons(f184(Str::lit("transitive-predicate")), List::cons(f184(Str::lit("infix-predicate")), List::cons(f184(Str::lit("proof-constant")), List::<Bytes>::nil())))))))))))))))))
}

pub fn f105() -> List<Bytes> {
    List::cons(f184(Str::lit("atom")), List::cons(f184(Str::lit("call")), List::cons(f184(Str::lit("prefix")), List::cons(f184(Str::lit("postfix")), List::cons(f184(Str::lit("infix")), List::cons(f184(Str::lit("noun-of")), List::cons(f184(Str::lit("binary-noun-of")), List::cons(f184(Str::lit("adjective")), List::cons(f184(Str::lit("intransitive")), List::cons(f184(Str::lit("transitive")), List::<Bytes>::nil()))))))))))
}

pub fn f106() -> List<Bytes> {
    List::cons(f184(Str::lit("text")), List::cons(f184(Str::lit("math")), List::cons(f184(Str::lit("both")), List::<Bytes>::nil())))
}

pub fn f107() -> List<Bytes> {
    List::cons(f184(Str::lit("left")), List::cons(f184(Str::lit("right")), List::cons(f184(Str::lit("none")), List::<Bytes>::nil())))
}

pub fn f108() -> List<Bytes> {
    List::cons(f184(Str::lit("singular")), List::cons(f184(Str::lit("plural")), List::cons(f184(Str::lit("sentence-case")), List::cons(f184(Str::lit("lower-case")), List::cons(f184(Str::lit("article-a")), List::cons(f184(Str::lit("article-an")), List::<Bytes>::nil()))))))
}

pub fn f109(v0: List<u64>, v1: u64) -> R<u64> {
    let (_, h518) = f185(Fn27::F186(v1), (0u64, 0u64), v0)?;
    Ok(h518)
}

pub fn f110() -> List<u64> {
    List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(1u64, List::cons(2u64, List::cons(3u64, List::cons(4u64, List::cons(5u64, List::cons(6u64, List::cons(0u64, List::cons(7u64, List::cons(8u64, List::cons(9u64, List::cons(4u64, List::cons(0u64, List::<u64>::nil())))))))))))))))))
}

pub fn f111() -> List<u64> {
    List::cons(0u64, List::cons(0u64, List::cons(1u64, List::cons(0u64, List::cons(0u64, List::cons(2u64, List::cons(2u64, List::cons(2u64, List::cons(2u64, List::cons(1u64, List::cons(1u64, List::cons(2u64, List::cons(1u64, List::cons(1u64, List::cons(1u64, List::cons(2u64, List::cons(2u64, List::<u64>::nil())))))))))))))))))
}

pub fn f112(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a519 = v0.clone();
        length_bytes(a519)
    };
    let v3: Adt32 = {
        let (h520, _) = f187(Fn28::F188(v1, v2, v0), {
            let a521 = v2;
            let a522 = 1u64;
            nat_add(a521, a522)?
        }, Adt32::C0(0u64, true, true, true))?;
        h520
    };
    if match v3.clone() {
        Adt32::C0(_, _, _, h523) => {
            h523
        }
    } {
        Ok(!match v3 {
            Adt32::C0(_, h524, _, _) => {
                h524
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f113(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a525 = v2;
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        f189(v0, 0u64)
    } else {
        let (_, h527) = {
            let (h528, _) = f190(Fn29::F191(v1, v0), v2, (v2, Bytes::lit(&[])));
            h528
        };
        h527
    }
}

pub fn f114() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f115(v0: Fn12, p532: (Option<Bytes>, bool), p533: List<Bytes>) -> (Option<Bytes>, bool) {
    let m531 = (p532, p533);
    {
        let mut m531 = m531;
        loop {
            let step = {
                let (v1, v2) = m531;
                let m529 = v2;
                match m529.uncons() {
                    None => {
                        Err::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(({
                            let c530: Fn12 = v0.clone();
                            c530.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    f192(v0, v1)
}

pub fn f117(v0: Fn13, p537: Option<Bytes>, p538: List<Bytes>) -> R<Option<Bytes>> {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v1, v2) = m536;
                let m534 = v2;
                match m534.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c535: Fn13 = v0.clone();
                            c535.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Option<Bytes>, v1: Bytes) -> R<Option<Bytes>> {
    let m539 = v0.clone();
    match m539 {
        None => {
            if f193(v1.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f119(v0: Bytes) -> R<Result<List<Adt33>, Adt28>> {
    let v1: u64 = {
        let a540 = v0.clone();
        length_bytes(a540)
    };
    let v2: Adt34 = {
        let (h541, _) = f194(Fn30::F195(v1, v0.clone()), {
            let a542 = v1;
            let a543 = 1u64;
            nat_add(a542, a543)?
        }, Adt34::C0(0u64, 0u64, false, List::<Adt33>::nil(), None::<Adt28>))?;
        h541
    };
    let m545 = match v2.clone() {
        Adt34::C0(_, _, _, _, h544) => {
            h544
        }
    };
    match m545 {
        None => {
            Ok(Ok::<List<Adt33>, Adt28>(f196(f197(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt33>, Adt28>(v3))
        }
    }
}

pub fn f120(v0: List<Adt33>, v1: u64) -> R<Result<Adt35, Adt28>> {
    let v2: Adt36 = {
        let (h546, _) = f198(Fn31::F199(v1, v0.clone()), {
            let a548 = {
                let a547 = v0;
                length_list(a547)
            };
            let a549 = 2u64;
            nat_add(a548, a549)?
        }, Adt36::C0(0u64, List::<List<Adt35>>::nil(), None::<Adt35>, None::<Adt28>, false))?;
        h546
    };
    let m551 = match v2.clone() {
        Adt36::C0(_, _, _, h550, _) => {
            h550
        }
    };
    match m551 {
        None => {
            let m553 = match v2 {
                Adt36::C0(_, _, h552, _, _) => {
                    h552
                }
            };
            match m553 {
                None => {
                    Ok(Err::<Adt35, Adt28>(f200(f201(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt35, Adt28>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt35, Adt28>(v3))
        }
    }
}

pub fn f121(v0: Adt35, v1: u64) -> R<Result<Adt1, Adt28>> {
    let v2: Adt37 = {
        let (h554, _) = f202(Fn32::F203(v1), {
            let a557 = {
                let a555 = v1;
                let a556 = 2u64;
                nat_mul(a555, a556)?
            };
            let a558 = 2u64;
            nat_add(a557, a558)?
        }, Adt37::C0(List::cons(Adt38::C0(std::rc::Rc::new(v0)), List::<Adt38>::nil()), List::<Adt1>::nil(), None::<Adt28>))?;
        h554
    };
    let m560 = match v2.clone() {
        Adt37::C0(_, _, h559) => {
            h559
        }
    };
    match m560 {
        None => {
            let m562 = match v2 {
                Adt37::C0(_, h561, _) => {
                    h561
                }
            };
            match m562.uncons() {
                None => {
                    Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("malformed expression")))))
                }
                Some((v4, v5)) => {
                    let m563 = v5;
                    match m563.uncons() {
                        None => {
                            Ok(Ok::<Adt1, Adt28>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("malformed expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt1, Adt28>(v3))
        }
    }
}

pub fn f122(v0: Fn14, p568: u64, p569: Adt29) -> R<(Adt29, bool)> {
    let m567 = (p568, p569);
    {
        let mut m567 = m567;
        loop {
            let step = {
                let (v1, v2) = m567;
                let m564 = v1;
                if m564 == 0 {
                    Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m564 - 1;
                    let m566 = {
                        let c565: Fn14 = v0.clone();
                        c565.apply(v2.clone())?
                    };
                    match m566 {
                        None => {
                            Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt29), R<(Adt29, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: List<Bytes>, v1: Adt29) -> R<Option<Adt29>> {
    f204(v0, v1)
}

pub fn f124(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m570 = v0;
    match m570 {
        Adt1::C0 => {
            Ok((f201(Str::lit("(sort prop)")), v3))
        }
        Adt1::C1(r598) => {
            let v4 = (*r598).clone();
            Ok((f205(f205(f201(Str::lit("(sort (type ")), f206(v4)), f201(Str::lit("))"))), v3))
        }
        Adt1::C2(v5, v6, r599) => {
            let v7 = (*r599).clone();
            Ok((f207(v5, v6, v7), v3))
        }
        Adt1::C3(v8) => {
            Ok((f205(f205(f201(Str::lit("(local ")), if v1 {
                f208(v2, v8)
            } else {
                v8
            }), f201(Str::lit(")"))), v3))
        }
        Adt1::C4(r600, r601) => {
            let v9 = (*r600).clone();
            let v10 = (*r601).clone();
            let v11: (Bytes, u64) = f124(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f209(v10, v1, v2, {
                let (_, h571) = v11;
                h571
            })?;
            Ok((f205(f205(f205(f201(Str::lit("(app ")), {
                let (h572, _) = v11;
                h572
            }), {
                let (h573, _) = v12;
                h573
            }), f201(Str::lit(")"))), {
                let (_, h574) = v12;
                h574
            }))
        }
        Adt1::C5(r602, r603) => {
            let v13 = (*r602).clone();
            let v14 = (*r603).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f124(v14, v1, {
                let (_, h575) = {
                    let (h576, _) = v15.clone();
                    h576
                };
                h575
            }, {
                let (_, h577) = v15;
                h577
            })?;
            Ok((f205(f205(f205(f205(f201(Str::lit("(pi (")), {
                let (h578, _) = {
                    let (h579, _) = v15;
                    h579
                };
                h578
            }), f201(Str::lit(") "))), {
                let (h580, _) = v16;
                h580
            }), f201(Str::lit(")"))), {
                let (_, h581) = v16;
                h581
            }))
        }
        Adt1::C6(r604, r605) => {
            let v17 = (*r604).clone();
            let v18 = (*r605).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f124(v18, v1, {
                let (_, h582) = {
                    let (h583, _) = v19.clone();
                    h583
                };
                h582
            }, {
                let (_, h584) = v19;
                h584
            })?;
            Ok((f205(f205(f205(f205(f201(Str::lit("(lam (")), {
                let (h585, _) = {
                    let (h586, _) = v19;
                    h586
                };
                h585
            }), f201(Str::lit(") "))), {
                let (h587, _) = v20;
                h587
            }), f201(Str::lit(")"))), {
                let (_, h588) = v20;
                h588
            }))
        }
        Adt1::C7(v21, r606, r607, r608) => {
            let v22 = (*r606).clone();
            let v23 = (*r607).clone();
            let v24 = (*r608).clone();
            let v25: (Bytes, u64) = f211(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h589, _) = v25.clone();
                h589
            }), v2);
            let v27: (Bytes, u64) = f124(v22, v1, v26.clone(), {
                let (_, h590) = v25;
                h590
            })?;
            let v28: (Bytes, u64) = f124(v23, v1, v26.clone(), {
                let (_, h591) = v27;
                h591
            })?;
            let v29: (Bytes, u64) = f124(v24, v1, v26, {
                let (_, h592) = v28;
                h592
            })?;
            Ok((f205(f205(f205(f205(f205(f205(f205(f201(Str::lit("(let ")), {
                let (h593, _) = v25;
                h593
            }), f201(Str::lit(" "))), {
                let (h594, _) = v27;
                h594
            }), f201(Str::lit(" "))), {
                let (h595, _) = v28;
                h595
            }), f201(Str::lit(" "))), f205({
                let (h596, _) = v29;
                h596
            }, f201(Str::lit(")")))), {
                let (_, h597) = v29;
                h597
            }))
        }
        Adt1::C8(v30) => {
            Ok((f205(f205(f201(Str::lit("(nat ")), v30), f201(Str::lit(")"))), v3))
        }
        Adt1::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C10(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f125(v0: Bytes) -> R<Bytes> {
    let v1: Adt39 = f212(v0)?;
    let a629 = f213(match v1.clone() {
        Adt39::C0(h609, _, _, _, _, _, _, _) => {
            h609
        }
    });
    let a630 = {
        let a627 = f213(match v1.clone() {
            Adt39::C0(_, h610, _, _, _, _, _, _) => {
                h610
            }
        });
        let a628 = {
            let a625 = f213(match v1.clone() {
                Adt39::C0(_, _, h611, _, _, _, _, _) => {
                    h611
                }
            });
            let a626 = {
                let a623 = f213(match v1.clone() {
                    Adt39::C0(_, _, _, h612, _, _, _, _) => {
                        h612
                    }
                });
                let a624 = {
                    let a621 = f213(match v1.clone() {
                        Adt39::C0(_, _, _, _, h613, _, _, _) => {
                            h613
                        }
                    });
                    let a622 = {
                        let a619 = f213(match v1.clone() {
                            Adt39::C0(_, _, _, _, _, h614, _, _) => {
                                h614
                            }
                        });
                        let a620 = {
                            let a617 = f213(match v1.clone() {
                                Adt39::C0(_, _, _, _, _, _, h615, _) => {
                                    h615
                                }
                            });
                            let a618 = f213(match v1 {
                                Adt39::C0(_, _, _, _, _, _, _, h616) => {
                                    h616
                                }
                            });
                            append_bytes(a617, a618)
                        };
                        append_bytes(a619, a620)
                    };
                    append_bytes(a621, a622)
                };
                append_bytes(a623, a624)
            };
            append_bytes(a625, a626)
        };
        append_bytes(a627, a628)
    };
    Ok(append_bytes(a629, a630))
}

pub fn f126(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLS8002")), v0)
}

pub fn f127(v0: Option<Bytes>) -> u64 {
    let m631 = v0;
    match m631 {
        None => {
            0u64
        }
        Some(_) => {
            1u64
        }
    }
}

pub fn f128(v0: bool, v1: List<Bytes>, v2: Adt24, v3: Adt21) -> R<Adt24> {
    let v4: bool = f214(match v2.clone() {
        Adt24::C0(_, _, h632) => {
            h632
        }
    }, match v3.clone() {
        Adt21::C0(h633, _, _, _, _) => {
            h633
        }
    })?;
    let v5: (List<(Bytes, Bytes)>, Option<Adt4>) = f215(v3.clone(), v4, v0, v1, match v2.clone() {
        Adt24::C0(h634, _, _) => {
            h634
        }
    })?;
    Ok(Adt24::C0({
        let (h635, _) = v5;
        h635
    }, {
        let m637 = {
            let (_, h636) = v5;
            h636
        };
        match m637 {
            None => {
                match v2.clone() {
                    Adt24::C0(_, h639, _) => {
                        h639
                    }
                }
            }
            Some(v6) => {
                List::cons(v6, match v2.clone() {
                    Adt24::C0(_, h638, _) => {
                        h638
                    }
                })
            }
        }
    }, List::cons(match v3 {
        Adt21::C0(h640, _, _, _, _) => {
            h640
        }
    }, match v2 {
        Adt24::C0(_, _, h641) => {
            h641
        }
    })))
}

pub fn f129(v0: Fn15, p645: List<Adt4>, p646: List<Adt4>) -> List<Adt4> {
    let m644 = (p645, p646);
    {
        let mut m644 = m644;
        loop {
            let step = {
                let (v1, v2) = m644;
                let m642 = v2;
                match m642.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c643: Fn15 = v0.clone();
                            c643.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f131(v0: Fn16, p650: bool, p651: List<Adt4>) -> bool {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v1, v2) = m649;
                let m647 = v2;
                match m647.uncons() {
                    None => {
                        Err::<(bool, List<Adt4>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt4>), bool>(({
                            let c648: Fn16 = v0.clone();
                            c648.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: bool, v1: Adt4) -> bool {
    if v0 {
        true
    } else {
        match v1 {
            Adt4::C0(_, _, _, _, h652, _) => {
                h652
            }
        }
    }
}

pub fn f133(v0: Fn17, p656: u64, p657: List<Adt4>) -> R<u64> {
    let m655 = (p656, p657);
    {
        let mut m655 = m655;
        loop {
            let step = {
                let (v1, v2) = m655;
                let m653 = v2;
                match m653.uncons() {
                    None => {
                        Err::<(u64, List<Adt4>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt4>), R<u64>>(({
                            let c654: Fn17 = v0.clone();
                            c654.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: u64, v1: u64, v2: Adt4) -> R<u64> {
    if if match v2.clone() {
        Adt4::C0(_, _, _, _, h658, _) => {
            h658
        }
    } {
        f216(match v2 {
            Adt4::C0(_, h659, _, _, _, _) => {
                h659
            }
        }, v0)
    } else {
        false
    } {
        let a660 = v1;
        let a661 = 1u64;
        nat_add(a660, a661)
    } else {
        Ok(v1)
    }
}

pub fn f135(v0: Fn18, p665: bool, p666: List<u64>) -> bool {
    let m664 = (p665, p666);
    {
        let mut m664 = m664;
        loop {
            let step = {
                let (v1, v2) = m664;
                let m662 = v2;
                match m662.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c663: Fn18 = v0.clone();
                            c663.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a667 = v2;
        let a668 = v0;
        nat_eq(a667, a668)
    }
}

pub fn f137(v0: Bytes, v1: u64) -> R<Result<Adt7, Adt28>> {
    let m669 = f119(v0)?;
    match m669 {
        Ok(v3) => {
            let m670 = f120(v3.clone(), v1)?;
            match m670 {
                Ok(v5) => {
                    f217(v5, {
                        let a671 = v3;
                        length_list(a671)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt7, Adt28>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt7, Adt28>(v2))
        }
    }
}

pub fn f138(v0: Bytes, v1: Adt28) -> (Bytes, Bytes) {
    let a673 = match v1.clone() {
        Adt28::C0(h672, _) => {
            h672
        }
    };
    let a674 = 1u64;
    if nat_eq(a673, a674) {
        f126(f5(f5(v0, f3(Str::lit(" render: "))), match v1 {
            Adt28::C0(_, h675) => {
                h675
            }
        }))
    } else {
        f2(f5(f5(f5(f3(Str::lit("invalid ")), v0), f3(Str::lit(" LRE: "))), match v1 {
            Adt28::C0(_, h676) => {
                h676
            }
        }))
    }
}

pub fn f139(v0: Adt7, v1: u64) -> R<Adt31> {
    let v2: Adt40 = {
        let (h677, _) = f218(Fn33::F219(v1), {
            let a678 = v1;
            let a679 = 1u64;
            nat_add(a678, a679)?
        }, Adt40::C0(List::cons(v0, List::<Adt7>::nil()), std::rc::Rc::new(Adt31::C0(List::<u64>::nil(), List::<Bytes>::nil(), List::<(Bytes, (Bytes, Bytes))>::nil(), List::<Bytes>::nil(), List::<Bytes>::nil(), None::<Bytes>, None::<Bytes>))))?;
        h677
    };
    let v3: Adt31 = match v2 {
        Adt40::C0(_, h680) => {
            (*h680).clone()
        }
    };
    Ok(Adt31::C0(f220(match v3.clone() {
        Adt31::C0(h681, _, _, _, _, _, _) => {
            h681
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, h682, _, _, _, _, _) => {
            h682
        }
    }), f221(match v3.clone() {
        Adt31::C0(_, _, h683, _, _, _, _) => {
            h683
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, _, _, h684, _, _, _) => {
            h684
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, _, _, _, h685, _, _) => {
            h685
        }
    }), match v3.clone() {
        Adt31::C0(_, _, _, _, _, h686, _) => {
            h686
        }
    }, match v3 {
        Adt31::C0(_, _, _, _, _, _, h687) => {
            h687
        }
    }))
}

pub fn f140(v0: List<u64>) -> Bytes {
    f222(Fn34::F223, Bytes::lit(&[]), v0)
}

pub fn f141(v0: List<u64>) -> List<u64> {
    f224(Fn35::F225, List::<u64>::nil(), v0)
}

pub fn f142(v0: u64) -> R<List<u64>> {
    Ok(f220({
        let (_, h688) = {
            let (h689, _) = f226(Fn36::F227(v0), {
                let a690 = v0;
                let a691 = 1u64;
                nat_add(a690, a691)?
            }, (0u64, List::<u64>::nil()))?;
            h689
        };
        h688
    }))
}

pub fn f143(v0: List<Bytes>, v1: List<Adt4>, v2: u64, v3: Bytes, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f228(Fn37::F229(v3, v1, v2), v4, v0)
}

pub fn f144(v0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
    let m693 = {
        let (h692, _) = v0;
        h692
    };
    match m693 {
        Adt1::C0 => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C1(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C2(_, _, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C3(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C4(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C5(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C6(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C7(_, _, _, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C8(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C9 => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C10(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C11(v1, _, _, r700) => {
            let v4 = (*r700).clone();
            Ok(Some((v4, {
                let a694 = v1;
                let a695 = 0u64;
                if nat_eq(a694, a695) {
                    let a697 = {
                        let (_, h696) = v0;
                        h696
                    };
                    let a698 = 1u64;
                    nat_add(a697, a698)?
                } else {
                    let (_, h699) = v0;
                    h699
                }
            })))
        }
    }
}

pub fn f145(v0: Fn19, p705: u64, p706: (u64, bool)) -> R<((u64, bool), bool)> {
    let m704 = (p705, p706);
    {
        let mut m704 = m704;
        loop {
            let step = {
                let (v1, v2) = m704;
                let m701 = v1;
                if m701 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m701 - 1;
                    let m703 = {
                        let c702: Fn19 = v0.clone();
                        c702.apply(v2)?
                    };
                    match m703 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f230(v1, v0, v2)
}

pub fn f147(v0: (u64, bool)) -> bool {
    let (_, h707) = v0;
    h707
}

pub fn f148(v0: Adt26, v1: Adt23) -> R<Adt26> {
    let m709 = f231(match v1.clone() {
        Adt23::C0(h708, _, _, _) => {
            h708
        }
    })?;
    match m709 {
        Ok(v3) => {
            let v4: Bytes = {
                let (h714, _) = v3;
                h714
            };
            let v5: Bytes = {
                let (_, h715) = v3;
                h715
            };
            let v6: Bytes = f232(v4.clone(), v5.clone());
            let v7: bool = f214(match v0.clone() {
                Adt26::C0(_, _, h716) => {
                    h716
                }
            }, v6.clone())?;
            let v8: List<(Bytes, Bytes)> = f46(match v0.clone() {
                Adt26::C0(h717, _, _) => {
                    h717
                }
            }, if v7 {
                Some(f2(f5(f3(Str::lit("duplicate eliminator constructor ")), f6(v6.clone()))))
            } else {
                None::<(Bytes, Bytes)>
            });
            let v9: List<(Bytes, Bytes)> = f46(v8, if f73(match v1.clone() {
                Adt23::C0(_, h718, _, _) => {
                    h718
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f6(match v1.clone() {
                    Adt23::C0(_, h719, _, _) => {
                        h719
                    }
                }), f3(Str::lit(" is not a Lean name")))))
            });
            let v10: List<(Bytes, Bytes)> = f233(match v1.clone() {
                Adt23::C0(_, _, h720, _) => {
                    h720
                }
            }, v9)?;
            let v11: List<(Bytes, Bytes)> = f233(match v1.clone() {
                Adt23::C0(_, _, _, h721) => {
                    h721
                }
            }, v10)?;
            Ok(Adt26::C0(v11, List::cons(Adt9::C0(v4, v5, match v1.clone() {
                Adt23::C0(_, h722, _, _) => {
                    h722
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h723, _) => {
                    h723
                }
            }, match v1 {
                Adt23::C0(_, _, _, h724) => {
                    h724
                }
            }), match v0.clone() {
                Adt26::C0(_, h725, _) => {
                    h725
                }
            }), List::cons(v6, match v0 {
                Adt26::C0(_, _, h726) => {
                    h726
                }
            })))
        }
        Err(v2) => {
            Ok(Adt26::C0(f46(match v0.clone() {
                Adt26::C0(h710, _, _) => {
                    h710
                }
            }, Some(f2(match v2 {
                Adt28::C0(_, h711) => {
                    h711
                }
            }))), match v0.clone() {
                Adt26::C0(_, h712, _) => {
                    h712
                }
            }, match v0 {
                Adt26::C0(_, _, h713) => {
                    h713
                }
            }))
        }
    }
}

pub fn f149(v0: Fn20, p730: List<Adt9>, p731: List<Adt9>) -> List<Adt9> {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v1, v2) = m729;
                let m727 = v2;
                match m727.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c728: Fn20 = v0.clone();
                            c728.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f151(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f234(v0.clone())?;
    let v2: u64 = f235(v0.clone(), v1)?;
    Ok(f83(v0, v2, {
        let a732 = v1;
        let a733 = v2;
        nat_sub(a732, a733)
    }))
}

pub fn f152(v0: Str) -> Bytes {
    let a734 = v0;
    utf8_encode(a734)
}

pub fn f153(v0: List<Bytes>) -> List<Bytes> {
    f156(Fn21::F236, List::<Bytes>::nil(), v0)
}

pub fn f154(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a735 = v0.clone();
        length_bytes(a735)
    };
    let v3: u64 = {
        let a736 = v1.clone();
        length_bytes(a736)
    };
    let a737 = v3;
    let a738 = v2;
    if nat_le(a737, a738) {
        let m744 = {
            let a741 = v0;
            let a742 = {
                let a739 = v2;
                let a740 = v3;
                nat_sub(a739, a740)
            };
            let a743 = v3;
            slice_bytes(a741, a742, a743)
        };
        match m744 {
            None => {
                false
            }
            Some(v4) => {
                let a745 = v4;
                let a746 = v1;
                equal(a745, a746)
            }
        }
    } else {
        false
    }
}

pub fn f155(v0: Bytes) -> R<Option<List<Bytes>>> {
    let v1: u64 = {
        let a747 = v0.clone();
        length_bytes(a747)
    };
    let v2: Adt41 = {
        let (h748, _) = f237(Fn38::F238(v0, v1), {
            let a749 = v1;
            let a750 = 2u64;
            nat_add(a749, a750)?
        }, Adt41::C0(0u64, List::<Bytes>::nil(), 0u64))?;
        h748
    };
    let a752 = match v2.clone() {
        Adt41::C0(_, _, h751) => {
            h751
        }
    };
    let a753 = 1u64;
    if equal(a752, a753) {
        Ok(Some(f153(match v2 {
            Adt41::C0(_, h754, _) => {
                h754
            }
        })))
    } else {
        Ok(None::<List<Bytes>>)
    }
}

pub fn f156(v0: Fn21, p758: List<Bytes>, p759: List<Bytes>) -> List<Bytes> {
    let m757 = (p758, p759);
    {
        let mut m757 = m757;
        loop {
            let step = {
                let (v1, v2) = m757;
                let m755 = v2;
                match m755.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c756: Fn21 = v0.clone();
                            c756.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f158(v0: Bytes) -> R<bool> {
    let v1: Bytes = f151(v0)?;
    let a761 = {
        let a760 = v1.clone();
        length_bytes(a760)
    };
    let a762 = 0u64;
    if nat_eq(a761, a762) {
        Ok(true)
    } else {
        Ok(f159(v1, f152(Str::lit("#"))))
    }
}

pub fn f159(v0: Bytes, v1: Bytes) -> bool {
    let m767 = {
        let a764 = v0;
        let a765 = 0u64;
        let a766 = {
            let a763 = v1.clone();
            length_bytes(a763)
        };
        slice_bytes(a764, a765, a766)
    };
    match m767 {
        None => {
            false
        }
        Some(v2) => {
            let a768 = v2;
            let a769 = v1;
            equal(a768, a769)
        }
    }
}

pub fn f160(v0: Bytes) -> R<Option<(List<Bytes>, bool)>> {
    let v1: bool = f159(v0.clone(), f152(Str::lit("[[")));
    let v2: u64 = {
        let a770 = v0.clone();
        length_bytes(a770)
    };
    let v3: Bytes = if v1 {
        f83(v0, 2u64, {
            let a771 = v2;
            let a772 = 2u64;
            nat_sub(a771, a772)
        })
    } else {
        f83(v0, 1u64, {
            let a773 = v2;
            let a774 = 1u64;
            nat_sub(a773, a774)
        })
    };
    let v4: bool = if v1 {
        f154(v3.clone(), f152(Str::lit("]]")))
    } else {
        f154(v3.clone(), f152(Str::lit("]")))
    };
    if v4 {
        let v5: u64 = {
            let a775 = v3.clone();
            length_bytes(a775)
        };
        let v6: u64 = if v1 {
            2u64
        } else {
            1u64
        };
        let v7: Bytes = f83(v3, 0u64, {
            let a776 = v5;
            let a777 = v6;
            nat_sub(a776, a777)
        });
        let a779 = {
            let a778 = v7.clone();
            length_bytes(a778)
        };
        let a780 = 0u64;
        if nat_eq(a779, a780) {
            Ok(None::<(List<Bytes>, bool)>)
        } else {
            Ok(Some((f239(v7, 46u8)?, v1)))
        }
    } else {
        Ok(None::<(List<Bytes>, bool)>)
    }
}

pub fn f161(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a781 = v0.clone();
        length_bytes(a781)
    };
    let v3: u64 = {
        let a782 = v1.clone();
        length_bytes(a782)
    };
    let v4: u64 = {
        let (h783, _) = f240(Fn39::F241(v1.clone(), v3, v0.clone(), v2), {
            let a784 = v2;
            let a785 = 1u64;
            nat_add(a784, a785)?
        }, 0u64)?;
        h783
    };
    if f242(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f162(v0: List<(Bytes, Adt17)>, v1: Bytes) -> bool {
    f243(Fn40::F244(v1), false, v0)
}

pub fn f163(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let a786 = v2.clone();
    let a787 = f152(Str::lit("true"));
    if equal(a786, a787) {
        Ok(Adt18::C0(match v0.clone() {
            Adt18::C0(h788, _, _, _, _, _, _) => {
                h788
            }
        }, match v0.clone() {
            Adt18::C0(_, h789, _, _, _, _, _) => {
                h789
            }
        }, match v0.clone() {
            Adt18::C0(_, _, h790, _, _, _, _) => {
                h790
            }
        }, List::cons((v1, Adt17::C1(true)), match v0 {
            Adt18::C0(_, _, _, h791, _, _, _) => {
                h791
            }
        }), None::<Bytes>, List::<Bytes>::nil(), false))
    } else {
        let a792 = v2.clone();
        let a793 = f152(Str::lit("false"));
        if equal(a792, a793) {
            Ok(Adt18::C0(match v0.clone() {
                Adt18::C0(h794, _, _, _, _, _, _) => {
                    h794
                }
            }, match v0.clone() {
                Adt18::C0(_, h795, _, _, _, _, _) => {
                    h795
                }
            }, match v0.clone() {
                Adt18::C0(_, _, h796, _, _, _, _) => {
                    h796
                }
            }, List::cons((v1, Adt17::C1(false)), match v0 {
                Adt18::C0(_, _, _, h797, _, _, _) => {
                    h797
                }
            }), None::<Bytes>, List::<Bytes>::nil(), false))
        } else {
            if f159(v2.clone(), f152(Str::lit("\""))) {
                let m798 = f245(v2.clone(), 0u64)?;
                match m798 {
                    None => {
                        Ok(Adt18::C0(match v0.clone() {
                            Adt18::C0(h812, _, _, _, _, _, _) => {
                                h812
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h813, _, _, _, _, _) => {
                                h813
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h814, _, _, _, _) => {
                                h814
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h815, _, _, _) => {
                                h815
                            }
                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                    }
                    Some(v3) => {
                        let a801 = {
                            let (_, h799) = v3;
                            h799
                        };
                        let a802 = {
                            let a800 = v2;
                            length_bytes(a800)
                        };
                        if nat_eq(a801, a802) {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h803, _, _, _, _, _, _) => {
                                    h803
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h804, _, _, _, _, _) => {
                                    h804
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h805, _, _, _, _) => {
                                    h805
                                }
                            }, List::cons((v1, Adt17::C0({
                                let (h806, _) = v3;
                                h806
                            })), match v0 {
                                Adt18::C0(_, _, _, h807, _, _, _) => {
                                    h807
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        } else {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h808, _, _, _, _, _, _) => {
                                    h808
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h809, _, _, _, _, _) => {
                                    h809
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h810, _, _, _, _) => {
                                    h810
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h811, _, _, _) => {
                                    h811
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                }
            } else {
                if f159(v2.clone(), f152(Str::lit("["))) {
                    let a816 = v2.clone();
                    let a817 = f152(Str::lit("["));
                    if equal(a816, a817) {
                        Ok(Adt18::C0(match v0.clone() {
                            Adt18::C0(h818, _, _, _, _, _, _) => {
                                h818
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h819, _, _, _, _, _) => {
                                h819
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h820, _, _, _, _) => {
                                h820
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h821, _, _, _) => {
                                h821
                            }
                        }, Some(v1), List::<Bytes>::nil(), false))
                    } else {
                        if f154(v2.clone(), f152(Str::lit("]"))) {
                            let m825 = f155(f83(v2.clone(), 1u64, {
                                let a823 = {
                                    let a822 = v2;
                                    length_bytes(a822)
                                };
                                let a824 = 2u64;
                                nat_sub(a823, a824)
                            }))?;
                            match m825 {
                                None => {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h830, _, _, _, _, _, _) => {
                                            h830
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h831, _, _, _, _, _) => {
                                            h831
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h832, _, _, _, _) => {
                                            h832
                                        }
                                    }, match v0 {
                                        Adt18::C0(_, _, _, h833, _, _, _) => {
                                            h833
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                }
                                Some(v4) => {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h826, _, _, _, _, _, _) => {
                                            h826
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h827, _, _, _, _, _) => {
                                            h827
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h828, _, _, _, _) => {
                                            h828
                                        }
                                    }, List::cons((v1, Adt17::C3(v4)), match v0 {
                                        Adt18::C0(_, _, _, h829, _, _, _) => {
                                            h829
                                        }
                                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                                }
                            }
                        } else {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h834, _, _, _, _, _, _) => {
                                    h834
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h835, _, _, _, _, _) => {
                                    h835
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h836, _, _, _, _) => {
                                    h836
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h837, _, _, _) => {
                                    h837
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                } else {
                    let m838 = f246(v2)?;
                    match m838 {
                        None => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h843, _, _, _, _, _, _) => {
                                    h843
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h844, _, _, _, _, _) => {
                                    h844
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h845, _, _, _, _) => {
                                    h845
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h846, _, _, _) => {
                                    h846
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                        Some(v5) => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h839, _, _, _, _, _, _) => {
                                    h839
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h840, _, _, _, _, _) => {
                                    h840
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h841, _, _, _, _) => {
                                    h841
                                }
                            }, List::cons((v1, Adt17::C2(v5)), match v0 {
                                Adt18::C0(_, _, _, h842, _, _, _) => {
                                    h842
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        }
                    }
                }
            }
        }
    }
}

pub fn f164(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h847, _) = v1.clone();
        h847
    };
    let v3: u64 = {
        let (h848, _) = {
            let (_, h849) = v1.clone();
            h849
        };
        h848
    };
    let v4: List<Bytes> = {
        let (_, h850) = {
            let (_, h851) = v1;
            h851
        };
        h850
    };
    let m852 = f247(v0.clone(), v2);
    match m852 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a853 = v5;
            let a854 = 10u8;
            if equal(a853, a854) {
                Ok(Some(({
                    let a855 = v2;
                    let a856 = 1u64;
                    nat_add(a855, a856)?
                }, ({
                    let a857 = v2;
                    let a858 = 1u64;
                    nat_add(a857, a858)?
                }, List::cons(f248(f83(v0, v3, {
                    let a859 = v2;
                    let a860 = v3;
                    nat_sub(a859, a860)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a861 = v2;
                    let a862 = 1u64;
                    nat_add(a861, a862)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f165(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f166(v0: Fn22, p866: List<(Bytes, Adt17)>, p867: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    let m865 = (p866, p867);
    {
        let mut m865 = m865;
        loop {
            let step = {
                let (v1, v2) = m865;
                let m863 = v2;
                match m863.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt17)>, List<(Bytes, Adt17)>), List<(Bytes, Adt17)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt17)>, List<(Bytes, Adt17)>), List<(Bytes, Adt17)>>(({
                            let c864: Fn22 = v0.clone();
                            c864.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: List<(Bytes, Adt17)>, v1: (Bytes, Adt17)) -> List<(Bytes, Adt17)> {
    List::cons(v1, v0)
}

pub fn f168(v0: List<Adt16>, v1: List<Bytes>, v2: bool) -> R<bool> {
    f249(Fn41::F250(v2, v1), false, v0)
}

pub fn f169(v0: List<Adt16>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<List<Adt16>> {
    f251(f252(Fn42::F253(v2, v1, v3), List::<Adt16>::nil(), v0)?)
}

pub fn f170(v0: List<Adt16>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<List<Adt16>> {
    f251(List::cons(Adt16::C0(v1, v2, List::cons(v3, List::<List<(Bytes, Adt17)>>::nil())), f251(v0)?))
}

pub fn f171(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m868 = f41(v0, v1)?;
    match m868 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f172(v0: Fn23, p872: List<Bytes>, p873: List<(Bytes, Adt17)>) -> List<Bytes> {
    let m871 = (p872, p873);
    {
        let mut m871 = m871;
        loop {
            let step = {
                let (v1, v2) = m871;
                let m869 = v2;
                match m869.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Adt17)>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Adt17)>), List<Bytes>>(({
                            let c870: Fn23 = v0.clone();
                            c870.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: List<Bytes>, v1: (Bytes, Adt17)) -> List<Bytes> {
    List::cons({
        let (h874, _) = v1;
        h874
    }, v0)
}

pub fn f174(v0: Fn24, p878: Option<Adt17>, p879: List<(Bytes, Adt17)>) -> Option<Adt17> {
    let m877 = (p878, p879);
    {
        let mut m877 = m877;
        loop {
            let step = {
                let (v1, v2) = m877;
                let m875 = v2;
                match m875.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<(Bytes, Adt17)>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<(Bytes, Adt17)>), Option<Adt17>>(({
                            let c876: Fn24 = v0.clone();
                            c876.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: Option<Adt17>, v2: (Bytes, Adt17)) -> Option<Adt17> {
    let m880 = v1.clone();
    match m880 {
        None => {
            let a882 = {
                let (h881, _) = v2;
                h881
            };
            let a883 = v0;
            if equal(a882, a883) {
                Some({
                    let (_, h884) = v2;
                    h884
                })
            } else {
                None::<Adt17>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f176(v0: Adt16, v1: List<Bytes>, v2: bool) -> R<bool> {
    if f254(match v0.clone() {
        Adt16::C0(h885, _, _) => {
            h885
        }
    }, v1)? {
        let a887 = match v0 {
            Adt16::C0(_, h886, _) => {
                h886
            }
        };
        let a888 = v2;
        Ok(equal(a887, a888))
    } else {
        Ok(false)
    }
}

pub fn f177(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<List<Bytes>> {
    let m889 = f91(v0, v1);
    match m889 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m890 = v2;
            match m890 {
                Adt17::C0(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C1(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C2(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C3(v6) => {
                    Some(v6)
                }
            }
        }
    }
}

pub fn f178(v0: List<(Bytes, Adt17)>) -> R<Option<Adt21>> {
    if f27(v0.clone(), f255())? {
        let m891 = f29(v0.clone(), f3(Str::lit("id")));
        match m891 {
            None => {
                Ok(None::<Adt21>)
            }
            Some(v1) => {
                let m892 = f29(v0.clone(), f3(Str::lit("channel")));
                match m892 {
                    None => {
                        Ok(None::<Adt21>)
                    }
                    Some(v2) => {
                        let m893 = f29(v0.clone(), f3(Str::lit("surface")));
                        match m893 {
                            None => {
                                Ok(None::<Adt21>)
                            }
                            Some(v3) => {
                                let m894 = f256(v0.clone(), f3(Str::lit("canonical_source")));
                                match m894 {
                                    None => {
                                        Ok(None::<Adt21>)
                                    }
                                    Some(v4) => {
                                        let m895 = f177(v0, f3(Str::lit("features")));
                                        match m895 {
                                            None => {
                                                Ok(None::<Adt21>)
                                            }
                                            Some(v5) => {
                                                Ok(Some(Adt21::C0(v1, v2, v3, v4, v5)))
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
    } else {
        Ok(None::<Adt21>)
    }
}

pub fn f179(v0: Fn25, p899: List<Adt21>, p900: List<Adt21>) -> List<Adt21> {
    let m898 = (p899, p900);
    {
        let mut m898 = m898;
        loop {
            let step = {
                let (v1, v2) = m898;
                let m896 = v2;
                match m896.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>(({
                            let c897: Fn25 = v0.clone();
                            c897.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<Adt21>, v1: Adt21) -> List<Adt21> {
    List::cons(v1, v0)
}

pub fn f181(v0: Fn26, p904: Option<List<Adt23>>, p905: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt23>>> {
    let m903 = (p904, p905);
    {
        let mut m903 = m903;
        loop {
            let step = {
                let (v1, v2) = m903;
                let m901 = v2;
                match m901.uncons() {
                    None => {
                        Err::<(Option<List<Adt23>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt23>>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt23>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt23>>>>(({
                            let c902: Fn26 = v0.clone();
                            c902.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Option<List<Adt23>>, v1: List<(Bytes, Adt17)>) -> R<Option<List<Adt23>>> {
    let m906 = v0;
    match m906 {
        None => {
            Ok(None::<List<Adt23>>)
        }
        Some(v2) => {
            let m907 = f257(v1)?;
            match m907 {
                None => {
                    Ok(None::<List<Adt23>>)
                }
                Some(v3) => {
                    Ok(Some(List::cons(v3, v2)))
                }
            }
        }
    }
}

pub fn f183(v0: List<Adt23>) -> List<Adt23> {
    f258(Fn43::F259, List::<Adt23>::nil(), v0)
}

pub fn f184(v0: Str) -> Bytes {
    let a908 = v0;
    utf8_encode(a908)
}

pub fn f185(v0: Fn27, p912: (u64, u64), p913: List<u64>) -> R<(u64, u64)> {
    let m911 = (p912, p913);
    {
        let mut m911 = m911;
        loop {
            let step = {
                let (v1, v2) = m911;
                let m909 = v2;
                match m909.uncons() {
                    None => {
                        Err::<((u64, u64), List<u64>), R<(u64, u64)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, u64), List<u64>), R<(u64, u64)>>(({
                            let c910: Fn27 = v0.clone();
                            c910.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: u64, v1: (u64, u64), v2: u64) -> R<(u64, u64)> {
    let a915 = {
        let (h914, _) = v1;
        h914
    };
    let a916 = v0;
    if nat_eq(a915, a916) {
        Ok(({
            let a918 = {
                let (h917, _) = v1;
                h917
            };
            let a919 = 1u64;
            nat_add(a918, a919)?
        }, v2))
    } else {
        Ok(({
            let a921 = {
                let (h920, _) = v1;
                h920
            };
            let a922 = 1u64;
            nat_add(a921, a922)?
        }, {
            let (_, h923) = v1;
            h923
        }))
    }
}

pub fn f187(v0: Fn28, p928: u64, p929: Adt32) -> R<(Adt32, bool)> {
    let m927 = (p928, p929);
    {
        let mut m927 = m927;
        loop {
            let step = {
                let (v1, v2) = m927;
                let m924 = v1;
                if m924 == 0 {
                    Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m924 - 1;
                    let m926 = {
                        let c925: Fn28 = v0.clone();
                        c925.apply(v2.clone())?
                    };
                    match m926 {
                        None => {
                            Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt32), R<(Adt32, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: bool, v1: u64, v2: Bytes, v3: Adt32) -> R<Option<Adt32>> {
    f260(v2, v1, v0, v3)
}

pub fn f189(v0: Bytes, v1: u64) -> Bytes {
    let m933 = {
        let a930 = v0;
        let a931 = v1;
        let a932 = 1u64;
        slice_bytes(a930, a931, a932)
    };
    match m933 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f190(v0: Fn29, p938: u64, p939: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m937 = (p938, p939);
    {
        let mut m937 = m937;
        loop {
            let step = {
                let (v1, v2) = m937;
                let m934 = v1;
                if m934 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m934 - 1;
                    let m936 = {
                        let c935: Fn29 = v0.clone();
                        c935.apply(v2.clone())
                    };
                    match m936 {
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
                Ok(next) => { m937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f261(v1, v0, v2)
}

pub fn f192(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    let m941 = {
        let (h940, _) = v0;
        h940
    };
    match m941 {
        None => {
            (Some(v1), {
                let (_, h942) = v0;
                h942
            })
        }
        Some(v2) => {
            (Some(v1.clone()), {
                let (_, h943) = v0;
                if h943 {
                    f262(v2, v1)
                } else {
                    false
                }
            })
        }
    }
}

pub fn f193(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a944 = v0.clone();
        length_bytes(a944)
    };
    let a945 = v1;
    let a946 = 0u64;
    if nat_eq(a945, a946) {
        Ok(false)
    } else {
        if f263(f264(v0.clone(), 0u64)?) {
            let a948 = {
                let (h947, _) = f240(Fn39::F265(v1, v0), v1, 1u64)?;
                h947
            };
            let a949 = v1;
            Ok(nat_eq(a948, a949))
        } else {
            Ok(false)
        }
    }
}

pub fn f194(v0: Fn30, p954: u64, p955: Adt34) -> R<(Adt34, bool)> {
    let m953 = (p954, p955);
    {
        let mut m953 = m953;
        loop {
            let step = {
                let (v1, v2) = m953;
                let m950 = v1;
                if m950 == 0 {
                    Err::<(u64, Adt34), R<(Adt34, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m950 - 1;
                    let m952 = {
                        let c951: Fn30 = v0.clone();
                        c951.apply(v2.clone())?
                    };
                    match m952 {
                        None => {
                            Err::<(u64, Adt34), R<(Adt34, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt34), R<(Adt34, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: u64, v1: Bytes, v2: Adt34) -> R<Option<Adt34>> {
    f266(v1, v0, v2)
}

pub fn f196(v0: List<Adt33>) -> List<Adt33> {
    f267(Fn44::F268, List::<Adt33>::nil(), v0)
}

pub fn f197(v0: Bytes, v1: Adt34) -> List<Adt33> {
    if match v1.clone() {
        Adt34::C0(_, _, h956, _, _) => {
            h956
        }
    } {
        List::cons(Adt33::C2(f83(v0, match v1.clone() {
            Adt34::C0(_, h957, _, _, _) => {
                h957
            }
        }, {
            let a960 = match v1.clone() {
                Adt34::C0(h958, _, _, _, _) => {
                    h958
                }
            };
            let a961 = match v1.clone() {
                Adt34::C0(_, h959, _, _, _) => {
                    h959
                }
            };
            nat_sub(a960, a961)
        })), match v1 {
            Adt34::C0(_, _, _, h962, _) => {
                h962
            }
        })
    } else {
        match v1 {
            Adt34::C0(_, _, _, h963, _) => {
                h963
            }
        }
    }
}

pub fn f198(v0: Fn31, p968: u64, p969: Adt36) -> R<(Adt36, bool)> {
    let m967 = (p968, p969);
    {
        let mut m967 = m967;
        loop {
            let step = {
                let (v1, v2) = m967;
                let m964 = v1;
                if m964 == 0 {
                    Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m964 - 1;
                    let m966 = {
                        let c965: Fn31 = v0.clone();
                        c965.apply(v2.clone())?
                    };
                    match m966 {
                        None => {
                            Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt36), R<(Adt36, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: u64, v1: List<Adt33>, v2: Adt36) -> R<Option<Adt36>> {
    f269(v1, v0, v2)
}

pub fn f200(v0: Bytes) -> Adt28 {
    Adt28::C0(0u64, v0)
}

pub fn f201(v0: Str) -> Bytes {
    let a970 = v0;
    utf8_encode(a970)
}

pub fn f202(v0: Fn32, p975: u64, p976: Adt37) -> R<(Adt37, bool)> {
    let m974 = (p975, p976);
    {
        let mut m974 = m974;
        loop {
            let step = {
                let (v1, v2) = m974;
                let m971 = v1;
                if m971 == 0 {
                    Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m971 - 1;
                    let m973 = {
                        let c972: Fn32 = v0.clone();
                        c972.apply(v2.clone())?
                    };
                    match m973 {
                        None => {
                            Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt37), R<(Adt37, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: u64, v1: Adt37) -> R<Option<Adt37>> {
    f270(v0, v1)
}

pub fn f204(v0: List<Bytes>, v1: Adt29) -> R<Option<Adt29>> {
    let m978 = match v1.clone() {
        Adt29::C0(_, _, _, h977) => {
            h977
        }
    };
    match m978 {
        None => {
            let m980 = match v1.clone() {
                Adt29::C0(h979, _, _, _) => {
                    h979
                }
            };
            match m980.uncons() {
                None => {
                    Ok(None::<Adt29>)
                }
                Some((v3, v4)) => {
                    let m981 = v3;
                    match m981 {
                        Adt30::C0(r990) => {
                            let v5 = (*r990).clone();
                            f271(v1, v4, v5)
                        }
                        Adt30::C1(v6) => {
                            Ok(Some(Adt29::C0(v4, List::cons(v6, match v1.clone() {
                                Adt29::C0(_, h982, _, _) => {
                                    h982
                                }
                            }), {
                                let a984 = match v1 {
                                    Adt29::C0(_, _, h983, _) => {
                                        h983
                                    }
                                };
                                let a985 = 1u64;
                                nat_add(a984, a985)?
                            }, None::<Bytes>)))
                        }
                        Adt30::C2(v7) => {
                            Ok(Some(Adt29::C0(v4, f272(match v1.clone() {
                                Adt29::C0(_, h986, _, _) => {
                                    h986
                                }
                            }, {
                                let a988 = match v1 {
                                    Adt29::C0(_, _, h987, _) => {
                                        h987
                                    }
                                };
                                let a989 = v7;
                                nat_sub(a988, a989)
                            }), v7, None::<Bytes>)))
                        }
                        Adt30::C3(r991) => {
                            let v8 = (*r991).clone();
                            f273(v1, v4, v0, v8)
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt29>)
        }
    }
}

pub fn f205(v0: Bytes, v1: Bytes) -> Bytes {
    let a992 = v0;
    let a993 = v1;
    append_bytes(a992, a993)
}

pub fn f206(v0: Adt2) -> Bytes {
    let m994 = v0;
    match m994 {
        Adt2::C0(v1) => {
            f48(v1)
        }
        Adt2::C1(v2) => {
            v2
        }
        Adt2::C2(r995) => {
            let v3 = (*r995).clone();
            f205(f205(f201(Str::lit("(succ ")), f206(v3)), f201(Str::lit(")")))
        }
        Adt2::C3(r996) => {
            let v4 = (*r996).clone();
            f205(f205(f201(Str::lit("(max")), f274(v4)), f201(Str::lit(")")))
        }
        Adt2::C4(r997, r998) => {
            let v5 = (*r997).clone();
            let v6 = (*r998).clone();
            f205(f205(f205(f205(f201(Str::lit("(imax ")), f206(v5)), f201(Str::lit(" "))), f206(v6)), f201(Str::lit(")")))
        }
        Adt2::C5 => {
            Bytes::lit(&[])
        }
        Adt2::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f207(v0: Bytes, v1: Bytes, v2: Adt2) -> Bytes {
    let v3: Bytes = f205(f205(f205(f201(Str::lit("(const ")), v0), f201(Str::lit("::"))), v1);
    let m999 = v2.clone();
    match m999 {
        Adt2::C0(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C1(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C2(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C3(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C4(_, _) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C5 => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C6(_, _) => {
            let v6: Bytes = f274(v2);
            f205(f205(f205(v3, f201(Str::lit(" ("))), f83(v6.clone(), 1u64, {
                let a1001 = {
                    let a1000 = v6;
                    length_bytes(a1000)
                };
                let a1002 = 1u64;
                nat_sub(a1001, a1002)
            })), f201(Str::lit("))")))
        }
    }
}

pub fn f208(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m1003 = f275(Fn45::F276(v1.clone()), None::<Bytes>, v0);
    match m1003 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f209(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m1004 = v0;
    match m1004 {
        Adt1::C0 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C1(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C2(_, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C3(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C4(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C5(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C6(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C7(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C8(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C10(r1009, r1010) => {
            let v4 = (*r1009).clone();
            let v5 = (*r1010).clone();
            let v6: (Bytes, u64) = f124(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f209(v5, v1, v2, {
                let (_, h1005) = v6;
                h1005
            })?;
            Ok((f205(f205(f201(Str::lit(" ")), {
                let (h1006, _) = v6;
                h1006
            }), {
                let (h1007, _) = v7;
                h1007
            }), {
                let (_, h1008) = v7;
                h1008
            }))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f210(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m1011 = v0;
    match m1011 {
        Adt1::C0 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C1(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C2(_, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C3(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C4(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C5(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C6(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C7(_, _, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C8(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C9 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C10(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C11(v4, v5, r1034, r1035) => {
            let v6 = (*r1034).clone();
            let v7 = (*r1035).clone();
            let v8: (Bytes, u64) = f211(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f124(v6, v1, v2.clone(), {
                let (_, h1012) = v8.clone();
                h1012
            })?;
            let v10: Bytes = f205(f205(f205(f205(f205(f205(f201(Str::lit("(")), f277(v4)), f201(Str::lit(" "))), {
                let (h1013, _) = v8.clone();
                h1013
            }), f201(Str::lit(" "))), {
                let (h1014, _) = v9;
                h1014
            }), f201(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h1015, _) = v8;
                h1015
            }), v2);
            let m1016 = v7.clone();
            match m1016 {
                Adt1::C0 => {
                    Ok(((v10, v11), {
                        let (_, h1024) = v9;
                        h1024
                    }))
                }
                Adt1::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h1025) = v9;
                        h1025
                    }))
                }
                Adt1::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h1026) = v9;
                        h1026
                    }))
                }
                Adt1::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h1027) = v9;
                        h1027
                    }))
                }
                Adt1::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1028) = v9;
                        h1028
                    }))
                }
                Adt1::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1029) = v9;
                        h1029
                    }))
                }
                Adt1::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1030) = v9;
                        h1030
                    }))
                }
                Adt1::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h1031) = v9;
                        h1031
                    }))
                }
                Adt1::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h1032) = v9;
                        h1032
                    }))
                }
                Adt1::C9 => {
                    Ok(((v10, v11), {
                        let (_, h1017) = v9;
                        h1017
                    }))
                }
                Adt1::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1033) = v9;
                        h1033
                    }))
                }
                Adt1::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v7, v1, v11, {
                        let (_, h1018) = v9;
                        h1018
                    })?;
                    Ok(((f205(f205(v10, f201(Str::lit(" "))), {
                        let (h1019, _) = {
                            let (h1020, _) = v16.clone();
                            h1020
                        };
                        h1019
                    }), {
                        let (_, h1021) = {
                            let (h1022, _) = v16;
                            h1022
                        };
                        h1021
                    }), {
                        let (_, h1023) = v16;
                        h1023
                    }))
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f205(f201(Str::lit("x")), f48(v2)), {
            let a1036 = v2;
            let a1037 = 1u64;
            nat_add(a1036, a1037)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f212(v0: Bytes) -> R<Adt39> {
    let v1: u64 = {
        let a1038 = v0.clone();
        length_bytes(a1038)
    };
    let v2: u64 = {
        let a1039 = v1;
        let a1040 = 0u64;
        if nat_eq(a1039, a1040) {
            1u64
        } else {
            let a1043 = {
                let a1041 = v1;
                let a1042 = 1023u64;
                nat_add(a1041, a1042)?
            };
            let a1044 = 1024u64;
            let a1045 = 0u64;
            nat_quot(a1043, a1044, a1045)
        }
    };
    let v3: u64 = {
        let a1046 = v2;
        let a1047 = 1u64;
        nat_sub(a1046, a1047)
    };
    let m1049 = {
        let (_, h1048) = f278(v0.clone(), v1, v3)?;
        h1048
    };
    match m1049.uncons() {
        None => {
            f279(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h1050) = f280(Fn46::F281, 64u64, (List::cons(v4, v5), f279(v0, v1, v3, false)?));
            Ok(h1050)
        }
    }
}

pub fn f213(v0: u32) -> Bytes {
    let a1063 = {
        let a1051 = f282(v0, 4u32);
        let a1052 = f282(v0, 0u32);
        append_bytes(a1051, a1052)
    };
    let a1064 = {
        let a1061 = {
            let a1053 = f282(v0, 12u32);
            let a1054 = f282(v0, 8u32);
            append_bytes(a1053, a1054)
        };
        let a1062 = {
            let a1059 = {
                let a1055 = f282(v0, 20u32);
                let a1056 = f282(v0, 16u32);
                append_bytes(a1055, a1056)
            };
            let a1060 = {
                let a1057 = f282(v0, 28u32);
                let a1058 = f282(v0, 24u32);
                append_bytes(a1057, a1058)
            };
            append_bytes(a1059, a1060)
        };
        append_bytes(a1061, a1062)
    };
    append_bytes(a1063, a1064)
}

pub fn f214(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f88(Fn8::F283(v1), false, v0)
}

pub fn f215(v0: Adt21, v1: bool, v2: bool, v3: List<Bytes>, v4: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Option<Adt4>)> {
    let v5: Bytes = match v0.clone() {
        Adt21::C0(h1065, _, _, _, _) => {
            h1065
        }
    };
    let v6: List<(Bytes, Bytes)> = f46(v4, if f284(v5.clone())? {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f6(v5.clone()), f3(Str::lit(" is not a form ID ([a-z][a-z0-9-]*)")))))
    });
    let v7: List<(Bytes, Bytes)> = f46(v6, if v1 {
        Some(f2(f5(f3(Str::lit("duplicate form ID ")), f6(v5.clone()))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v8: Adt42 = f285(v5.clone(), match v0.clone() {
        Adt21::C0(_, h1066, _, _, _) => {
            h1066
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1067, _, _) => {
            h1067
        }
    }, v7)?;
    let m1069 = match v8.clone() {
        Adt42::C0(_, h1068, _) => {
            h1068
        }
    };
    match m1069 {
        None => {
            Ok((match v8 {
                Adt42::C0(h1070, _, _) => {
                    h1070
                }
            }, None::<Adt4>))
        }
        Some(v9) => {
            let v10: List<Adt5> = match v8.clone() {
                Adt42::C0(_, _, h1071) => {
                    h1071
                }
            };
            let v11: List<(Bytes, Bytes)> = f286(v3, v5.clone(), v10.clone(), match v8 {
                Adt42::C0(h1072, _, _) => {
                    h1072
                }
            })?;
            let v12: bool = f287(v10.clone());
            let v13: List<(Bytes, Bytes)> = f46(v11, if if if v12 {
                match v0.clone() {
                    Adt21::C0(_, _, _, h1073, _) => {
                        h1073
                    }
                }
            } else {
                false
            } {
                !v2
            } else {
                false
            } {
                Some(f2(f5(f5(f3(Str::lit("form `")), v5.clone()), f3(Str::lit("`: control-sequence forms are aliases only for non-core entries")))))
            } else {
                None::<(Bytes, Bytes)>
            });
            let v14: bool = if if !match v0.clone() {
                Adt21::C0(_, _, _, h1074, _) => {
                    h1074
                }
            } {
                let a1076 = {
                    let a1075 = v10.clone();
                    length_list(a1075)
                };
                let a1077 = 1u64;
                nat_eq(a1076, a1077)
            } else {
                false
            } {
                f287(v10.clone())
            } else {
                false
            };
            let v16: List<(Bytes, Bytes)> = if if !v2 {
                !v14
            } else {
                false
            } {
                let m1078 = f288(v10.clone(), v9)?;
                match m1078 {
                    None => {
                        v13
                    }
                    Some(v15) => {
                        f46(v13, Some(f289(f5(f5(f5(f5(f5(f3(Str::lit("form `")), v5.clone()), f3(Str::lit("`: the "))), if match v0.clone() {
                            Adt21::C0(_, _, _, h1079, _) => {
                                h1079
                            }
                        } {
                            f3(Str::lit("canonical"))
                        } else {
                            f3(Str::lit("alias"))
                        }), f5(f3(Str::lit(" surface `")), match v0.clone() {
                            Adt21::C0(_, _, h1080, _, _) => {
                                h1080
                            }
                        })), f5(f3(Str::lit("` is not renderer-safe: ")), v15)))))
                    }
                }
            } else {
                v13
            };
            let v17: List<(Bytes, Bytes)> = f290(v5.clone(), match v0.clone() {
                Adt21::C0(_, _, _, _, h1081) => {
                    h1081
                }
            }, v16)?;
            Ok((v17, Some(Adt4::C0(v5, v9, match v0.clone() {
                Adt21::C0(_, _, h1082, _, _) => {
                    h1082
                }
            }, v10, match v0.clone() {
                Adt21::C0(_, _, _, h1083, _) => {
                    h1083
                }
            }, match v0 {
                Adt21::C0(_, _, _, _, h1084) => {
                    h1084
                }
            }))))
        }
    }
}

pub fn f216(v0: u64, v1: u64) -> bool {
    let a1085 = v0;
    let a1086 = 2u64;
    if nat_eq(a1085, a1086) {
        true
    } else {
        let a1087 = v0;
        let a1088 = v1;
        nat_eq(a1087, a1088)
    }
}

pub fn f217(v0: Adt35, v1: u64) -> R<Result<Adt7, Adt28>> {
    let v2: Adt43 = {
        let (h1089, _) = f291(Fn47::F292(v1), {
            let a1092 = {
                let a1090 = v1;
                let a1091 = 2u64;
                nat_mul(a1090, a1091)?
            };
            let a1093 = 2u64;
            nat_add(a1092, a1093)?
        }, Adt43::C0(List::cons(Adt44::C0(std::rc::Rc::new(v0)), List::<Adt44>::nil()), List::<Adt7>::nil(), None::<Adt28>))?;
        h1089
    };
    let m1095 = match v2.clone() {
        Adt43::C0(_, _, h1094) => {
            h1094
        }
    };
    match m1095 {
        None => {
            let m1097 = match v2 {
                Adt43::C0(_, h1096, _) => {
                    h1096
                }
            };
            match m1097.uncons() {
                None => {
                    Ok(Err::<Adt7, Adt28>(f200(f201(Str::lit("malformed render expression")))))
                }
                Some((v4, v5)) => {
                    let m1098 = v5;
                    match m1098.uncons() {
                        None => {
                            Ok(Ok::<Adt7, Adt28>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt7, Adt28>(f200(f201(Str::lit("malformed render expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt7, Adt28>(v3))
        }
    }
}

pub fn f218(v0: Fn33, p1103: u64, p1104: Adt40) -> R<(Adt40, bool)> {
    let m1102 = (p1103, p1104);
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let (v1, v2) = m1102;
                let m1099 = v1;
                if m1099 == 0 {
                    Err::<(u64, Adt40), R<(Adt40, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1099 - 1;
                    let m1101 = {
                        let c1100: Fn33 = v0.clone();
                        c1100.apply(v2.clone())?
                    };
                    match m1101 {
                        None => {
                            Err::<(u64, Adt40), R<(Adt40, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt40), R<(Adt40, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: u64, v1: Adt40) -> R<Option<Adt40>> {
    f293(v0, v1)
}

pub fn f220(v0: List<u64>) -> List<u64> {
    f224(Fn35::F294, List::<u64>::nil(), v0)
}

pub fn f221(v0: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    f295(Fn48::F296, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)
}

pub fn f222(v0: Fn34, p1108: Bytes, p1109: List<u64>) -> Bytes {
    let m1107 = (p1108, p1109);
    {
        let mut m1107 = m1107;
        loop {
            let step = {
                let (v1, v2) = m1107;
                let m1105 = v2;
                match m1105.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let c1106: Fn34 = v0.clone();
                            c1106.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Bytes, v1: u64) -> Bytes {
    f5(f5(v0, f3(Str::lit(","))), f48(v1))
}

pub fn f224(v0: Fn35, p1113: List<u64>, p1114: List<u64>) -> List<u64> {
    let m1112 = (p1113, p1114);
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, v2) = m1112;
                let m1110 = v2;
                match m1110.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1111: Fn35 = v0.clone();
                            c1111.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<u64>, v1: u64) -> List<u64> {
    f297(v0, v1)
}

pub fn f226(v0: Fn36, p1119: u64, p1120: (u64, List<u64>)) -> R<((u64, List<u64>), bool)> {
    let m1118 = (p1119, p1120);
    {
        let mut m1118 = m1118;
        loop {
            let step = {
                let (v1, v2) = m1118;
                let m1115 = v1;
                if m1115 == 0 {
                    Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1115 - 1;
                    let m1117 = {
                        let c1116: Fn36 = v0.clone();
                        c1116.apply(v2.clone())?
                    };
                    match m1117 {
                        None => {
                            Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: u64, v1: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
    let a1122 = {
        let (h1121, _) = v1.clone();
        h1121
    };
    let a1123 = v0;
    if nat_lt(a1122, a1123) {
        Ok(Some(({
            let a1125 = {
                let (h1124, _) = v1.clone();
                h1124
            };
            let a1126 = 1u64;
            nat_add(a1125, a1126)?
        }, List::cons({
            let (h1127, _) = v1;
            h1127
        }, {
            let (_, h1128) = v1;
            h1128
        }))))
    } else {
        Ok(None::<(u64, List<u64>)>)
    }
}

pub fn f228(v0: Fn37, p1132: List<(Bytes, Bytes)>, p1133: List<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1131 = (p1132, p1133);
    {
        let mut m1131 = m1131;
        loop {
            let step = {
                let (v1, v2) = m1131;
                let m1129 = v2;
                match m1129.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<Bytes>), R<List<(Bytes, Bytes)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<Bytes>), R<List<(Bytes, Bytes)>>>(({
                            let c1130: Fn37 = v0.clone();
                            c1130.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: List<Adt4>, v2: u64, v3: List<(Bytes, Bytes)>, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    f298(v1, v2, v0, v4, v3)
}

pub fn f230(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h1134, _) = v2;
        h1134
    };
    let a1135 = v3;
    let a1136 = v1;
    if nat_lt(a1135, a1136) {
        let v4: u64 = f264(v0, v3)?;
        let a1137 = v4;
        let a1138 = 46u64;
        if nat_eq(a1137, a1138) {
            if f147(v2) {
                Ok(None::<(u64, bool)>)
            } else {
                Ok(Some(({
                    let a1139 = v3;
                    let a1140 = 1u64;
                    nat_add(a1139, a1140)?
                }, true)))
            }
        } else {
            if f147(v2) {
                if f299(v4) {
                    Ok(Some(({
                        let a1141 = v3;
                        let a1142 = 1u64;
                        nat_add(a1141, a1142)?
                    }, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            } else {
                if f300(v4) {
                    Ok(Some(({
                        let a1143 = v3;
                        let a1144 = 1u64;
                        nat_add(a1143, a1144)?
                    }, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            }
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f231(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt28>> {
    let m1145 = f161(v0.clone(), f201(Str::lit("::")))?;
    match m1145 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v0), f201(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f83(v0.clone(), 0u64, v1);
            let v3: Bytes = f83(v0.clone(), {
                let a1146 = v1;
                let a1147 = 2u64;
                nat_add(a1146, a1147)?
            }, {
                let a1151 = {
                    let a1148 = v0;
                    length_bytes(a1148)
                };
                let a1152 = {
                    let a1149 = v1;
                    let a1150 = 2u64;
                    nat_add(a1149, a1150)?
                };
                nat_sub(a1151, a1152)
            });
            if f302(v2.clone())? {
                if f47(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt28>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v3), f201(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v2), f201(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: Bytes) -> Bytes {
    f5(f5(v0, f3(Str::lit("::"))), v1)
}

pub fn f233(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let (h1153, _) = f303(Fn49::F304, (v1, List::<Bytes>::nil()), v0)?;
    Ok(h1153)
}

pub fn f234(v0: Bytes) -> R<u64> {
    let (h1154, _) = f240(Fn39::F305(v0.clone()), {
        let a1155 = v0.clone();
        length_bytes(a1155)
    }, {
        let a1156 = v0;
        length_bytes(a1156)
    })?;
    Ok(h1154)
}

pub fn f235(v0: Bytes, v1: u64) -> R<u64> {
    let (h1157, _) = f240(Fn39::F306(v1, v0), v1, 0u64)?;
    Ok(h1157)
}

pub fn f236(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f237(v0: Fn38, p1162: u64, p1163: Adt41) -> R<(Adt41, bool)> {
    let m1161 = (p1162, p1163);
    {
        let mut m1161 = m1161;
        loop {
            let step = {
                let (v1, v2) = m1161;
                let m1158 = v1;
                if m1158 == 0 {
                    Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1158 - 1;
                    let m1160 = {
                        let c1159: Fn38 = v0.clone();
                        c1159.apply(v2.clone())?
                    };
                    match m1160 {
                        None => {
                            Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt41), R<(Adt41, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes, v1: u64, v2: Adt41) -> R<Option<Adt41>> {
    f307(v0, v1, v2)
}

pub fn f239(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a1164 = v0.clone();
        length_bytes(a1164)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h1165, _) = f80(Fn6::F308(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1165
    };
    let v4: u64 = {
        let (h1166, _) = {
            let (_, h1167) = v3.clone();
            h1167
        };
        h1166
    };
    Ok(f82(List::cons(f83(v0, v4, {
        let a1168 = v2;
        let a1169 = v4;
        nat_sub(a1168, a1169)
    }), {
        let (_, h1170) = {
            let (_, h1171) = v3;
            h1171
        };
        h1170
    })))
}

pub fn f240(v0: Fn39, p1176: u64, p1177: u64) -> R<(u64, bool)> {
    let m1175 = (p1176, p1177);
    {
        let mut m1175 = m1175;
        loop {
            let step = {
                let (v1, v2) = m1175;
                let m1172 = v1;
                if m1172 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1172 - 1;
                    let m1174 = {
                        let c1173: Fn39 = v0.clone();
                        c1173.apply(v2)?
                    };
                    match m1174 {
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
                Ok(next) => { m1175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f242(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1180 = {
            let a1178 = v4;
            let a1179 = v1;
            nat_add(a1178, a1179)?
        };
        let a1181 = v3;
        if nat_lt(a1180, a1181) {
            Ok(Some({
                let a1182 = v4;
                let a1183 = 1u64;
                nat_add(a1182, a1183)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f242(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1188 = {
        let a1185 = v0;
        let a1186 = v2;
        let a1187 = {
            let a1184 = v1.clone();
            length_bytes(a1184)
        };
        slice_bytes(a1185, a1186, a1187)
    };
    match m1188 {
        None => {
            false
        }
        Some(v3) => {
            let a1189 = v3;
            let a1190 = v1;
            equal(a1189, a1190)
        }
    }
}

pub fn f243(v0: Fn40, p1194: bool, p1195: List<(Bytes, Adt17)>) -> bool {
    let m1193 = (p1194, p1195);
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v1, v2) = m1193;
                let m1191 = v2;
                match m1191.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt17)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt17)>), bool>(({
                            let c1192: Fn40 = v0.clone();
                            c1192.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes, v1: bool, v2: (Bytes, Adt17)) -> bool {
    if v1 {
        true
    } else {
        let a1197 = {
            let (h1196, _) = v2;
            h1196
        };
        let a1198 = v0;
        equal(a1197, a1198)
    }
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<(Bytes, u64)>> {
    let m1199 = f247(v0.clone(), v1);
    match m1199 {
        None => {
            Ok(None::<(Bytes, u64)>)
        }
        Some(v2) => {
            let a1200 = v2;
            let a1201 = 34u8;
            if equal(a1200, a1201) {
                let v3: u64 = {
                    let a1202 = v0.clone();
                    length_bytes(a1202)
                };
                let v4: Adt45 = {
                    let (h1203, _) = f309(Fn50::F310(v0, v3), {
                        let a1204 = v3;
                        let a1205 = 2u64;
                        nat_add(a1204, a1205)?
                    }, Adt45::C0({
                        let a1206 = v1;
                        let a1207 = 1u64;
                        nat_add(a1206, a1207)?
                    }, Bytes::lit(&[]), 0u64, false))?;
                    h1203
                };
                if match v4.clone() {
                    Adt45::C0(_, _, _, h1208) => {
                        h1208
                    }
                } {
                    Ok(Some((match v4.clone() {
                        Adt45::C0(_, h1209, _, _) => {
                            h1209
                        }
                    }, match v4 {
                        Adt45::C0(h1210, _, _, _) => {
                            h1210
                        }
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f246(v0: Bytes) -> R<Option<u64>> {
    if f311(v0.clone())? {
        Ok(Some({
            let (_, h1211) = {
                let (h1212, _) = f312(Fn51::F313(v0.clone()), {
                    let a1213 = v0;
                    length_bytes(a1213)
                }, (0u64, 0u64))?;
                h1212
            };
            h1211
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f247(v0: Bytes, v1: u64) -> Option<u8> {
    let a1214 = v0;
    let a1215 = v1;
    index_bytes(a1214, a1215)
}

pub fn f248(v0: Bytes) -> Bytes {
    if f154(v0.clone(), Bytes::lit(&[0x0d])) {
        f83(v0.clone(), 0u64, {
            let a1217 = {
                let a1216 = v0;
                length_bytes(a1216)
            };
            let a1218 = 1u64;
            nat_sub(a1217, a1218)
        })
    } else {
        v0
    }
}

pub fn f249(v0: Fn41, p1222: bool, p1223: List<Adt16>) -> R<bool> {
    let m1221 = (p1222, p1223);
    {
        let mut m1221 = m1221;
        loop {
            let step = {
                let (v1, v2) = m1221;
                let m1219 = v2;
                match m1219.uncons() {
                    None => {
                        Err::<(bool, List<Adt16>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt16>), R<bool>>(({
                            let c1220: Fn41 = v0.clone();
                            c1220.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: bool, v1: List<Bytes>, v2: bool, v3: Adt16) -> R<bool> {
    if v2 {
        Ok(true)
    } else {
        f176(v3, v1, v0)
    }
}

pub fn f251(v0: List<Adt16>) -> R<List<Adt16>> {
    f252(Fn42::F314, List::<Adt16>::nil(), v0)
}

pub fn f252(v0: Fn42, p1227: List<Adt16>, p1228: List<Adt16>) -> R<List<Adt16>> {
    let m1226 = (p1227, p1228);
    {
        let mut m1226 = m1226;
        loop {
            let step = {
                let (v1, v2) = m1226;
                let m1224 = v2;
                match m1224.uncons() {
                    None => {
                        Err::<(List<Adt16>, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt16>, List<Adt16>), R<List<Adt16>>>(({
                            let c1225: Fn42 = v0.clone();
                            c1225.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: bool, v1: List<Bytes>, v2: List<(Bytes, Adt17)>, v3: List<Adt16>, v4: Adt16) -> R<List<Adt16>> {
    Ok(List::cons(f315(v4, v1, v0, v2)?, v3))
}

pub fn f254(v0: List<Bytes>, v1: List<Bytes>) -> R<bool> {
    let a1229 = f316(v0, Bytes::lit(&[0x2e]))?;
    let a1230 = f316(v1, Bytes::lit(&[0x2e]))?;
    Ok(equal(a1229, a1230))
}

pub fn f255() -> List<Bytes> {
    List::cons(f3(Str::lit("id")), List::cons(f3(Str::lit("channel")), List::cons(f3(Str::lit("surface")), List::cons(f3(Str::lit("canonical_source")), List::cons(f3(Str::lit("features")), List::<Bytes>::nil())))))
}

pub fn f256(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<bool> {
    let m1231 = f91(v0, v1);
    match m1231 {
        None => {
            None::<bool>
        }
        Some(v2) => {
            let m1232 = v2;
            match m1232 {
                Adt17::C0(_) => {
                    None::<bool>
                }
                Adt17::C1(v4) => {
                    Some(v4)
                }
                Adt17::C2(_) => {
                    None::<bool>
                }
                Adt17::C3(_) => {
                    None::<bool>
                }
            }
        }
    }
}

pub fn f257(v0: List<(Bytes, Adt17)>) -> R<Option<Adt23>> {
    if f27(v0.clone(), f317())? {
        let m1233 = f29(v0.clone(), f3(Str::lit("entry")));
        match m1233 {
            None => {
                Ok(None::<Adt23>)
            }
            Some(v1) => {
                let m1234 = f29(v0.clone(), f3(Str::lit("lean_name")));
                match m1234 {
                    None => {
                        Ok(None::<Adt23>)
                    }
                    Some(v2) => {
                        let m1235 = f177(v0.clone(), f3(Str::lit("fields")));
                        match m1235 {
                            None => {
                                Ok(None::<Adt23>)
                            }
                            Some(v3) => {
                                let m1236 = f177(v0, f3(Str::lit("induction_hypotheses")));
                                match m1236 {
                                    None => {
                                        Ok(None::<Adt23>)
                                    }
                                    Some(v4) => {
                                        Ok(Some(Adt23::C0(v1, v2, v3, v4)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt23>)
    }
}

pub fn f258(v0: Fn43, p1240: List<Adt23>, p1241: List<Adt23>) -> List<Adt23> {
    let m1239 = (p1240, p1241);
    {
        let mut m1239 = m1239;
        loop {
            let step = {
                let (v1, v2) = m1239;
                let m1237 = v2;
                match m1237.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>(({
                            let c1238: Fn43 = v0.clone();
                            c1238.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Adt23>, v1: Adt23) -> List<Adt23> {
    List::cons(v1, v0)
}

pub fn f260(v0: Bytes, v1: u64, v2: bool, v3: Adt32) -> R<Option<Adt32>> {
    if match v3.clone() {
        Adt32::C0(_, _, _, h1242) => {
            h1242
        }
    } {
        let a1244 = match v3.clone() {
            Adt32::C0(h1243, _, _, _) => {
                h1243
            }
        };
        let a1245 = v1;
        if nat_lt(a1244, a1245) {
            let v4: u64 = f264(v0, match v3.clone() {
                Adt32::C0(h1246, _, _, _) => {
                    h1246
                }
            })?;
            let a1247 = v4;
            let a1248 = 46u64;
            if nat_eq(a1247, a1248) {
                if match v3.clone() {
                    Adt32::C0(_, h1249, _, _) => {
                        h1249
                    }
                } {
                    Ok(f318(v3))
                } else {
                    f319(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt32::C0(_, h1250, _, _) => {
                        h1250
                    }
                } {
                    if f320(v4) {
                        f319(v3.clone(), false, match v3 {
                            Adt32::C0(_, _, h1251, _) => {
                                h1251
                            }
                        })
                    } else {
                        Ok(f318(v3))
                    }
                } else {
                    if if f320(v4) {
                        true
                    } else {
                        f321(v4)
                    } {
                        f319(v3.clone(), false, match v3 {
                            Adt32::C0(_, _, h1252, _) => {
                                h1252
                            }
                        })
                    } else {
                        let a1253 = v4;
                        let a1254 = 45u64;
                        if if nat_eq(a1253, a1254) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt32::C0(_, _, h1255, _) => {
                                        h1255
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f319(v3.clone(), false, match v3 {
                                Adt32::C0(_, _, h1256, _) => {
                                    h1256
                                }
                            })
                        } else {
                            Ok(f318(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt32>)
        }
    } else {
        Ok(None::<Adt32>)
    }
}

pub fn f261(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1257, _) = v2;
        h1257
    };
    let a1258 = v3;
    let a1259 = 0u64;
    if nat_eq(a1258, a1259) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1260 = v3;
            let a1261 = v1;
            let a1262 = 0u64;
            nat_quot(a1260, a1261, a1262)
        }, {
            let a1267 = f189(v0, {
                let a1263 = v3;
                let a1264 = v1;
                let a1265 = 0u64;
                nat_rem(a1263, a1264, a1265)
            });
            let a1268 = {
                let (_, h1266) = v2;
                h1266
            };
            append_bytes(a1267, a1268)
        }))
    }
}

pub fn f262(v0: Bytes, v1: Bytes) -> bool {
    let a1273 = {
        let a1269 = v0;
        let a1270 = v1;
        compare_bytes(a1269, a1270)
    };
    let a1274 = {
        let a1271 = Bytes::lit(&[]);
        let a1272 = Bytes::lit(&[0x01]);
        compare_bytes(a1271, a1272)
    };
    equal(a1273, a1274)
}

pub fn f263(v0: u64) -> bool {
    if f320(v0) {
        true
    } else {
        let a1275 = v0;
        let a1276 = 65u64;
        if nat_lt(a1275, a1276) {
            false
        } else {
            let a1277 = v0;
            let a1278 = 90u64;
            nat_le(a1277, a1278)
        }
    }
}

pub fn f264(v0: Bytes, v1: u64) -> R<u64> {
    let m1279 = f247(v0, v1);
    match m1279 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1324 = {
                let a1300 = {
                    let a1288 = {
                        let a1282 = {
                            let a1280 = v3;
                            let a1281 = 128u8;
                            fixed_u8::bit_and(a1280, a1281)
                        };
                        let a1283 = 128u8;
                        if equal(a1282, a1283) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1289 = {
                        let a1286 = {
                            let a1284 = v3;
                            let a1285 = 64u8;
                            fixed_u8::bit_and(a1284, a1285)
                        };
                        let a1287 = 64u8;
                        if equal(a1286, a1287) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1288, a1289)?
                };
                let a1301 = {
                    let a1298 = {
                        let a1292 = {
                            let a1290 = v3;
                            let a1291 = 32u8;
                            fixed_u8::bit_and(a1290, a1291)
                        };
                        let a1293 = 32u8;
                        if equal(a1292, a1293) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1299 = {
                        let a1296 = {
                            let a1294 = v3;
                            let a1295 = 16u8;
                            fixed_u8::bit_and(a1294, a1295)
                        };
                        let a1297 = 16u8;
                        if equal(a1296, a1297) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1298, a1299)?
                };
                nat_add(a1300, a1301)?
            };
            let a1325 = {
                let a1322 = {
                    let a1310 = {
                        let a1304 = {
                            let a1302 = v3;
                            let a1303 = 8u8;
                            fixed_u8::bit_and(a1302, a1303)
                        };
                        let a1305 = 8u8;
                        if equal(a1304, a1305) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1311 = {
                        let a1308 = {
                            let a1306 = v3;
                            let a1307 = 4u8;
                            fixed_u8::bit_and(a1306, a1307)
                        };
                        let a1309 = 4u8;
                        if equal(a1308, a1309) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1310, a1311)?
                };
                let a1323 = {
                    let a1320 = {
                        let a1314 = {
                            let a1312 = v3;
                            let a1313 = 2u8;
                            fixed_u8::bit_and(a1312, a1313)
                        };
                        let a1315 = 2u8;
                        if equal(a1314, a1315) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1321 = {
                        let a1318 = {
                            let a1316 = v3;
                            let a1317 = 1u8;
                            fixed_u8::bit_and(a1316, a1317)
                        };
                        let a1319 = 1u8;
                        if equal(a1318, a1319) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1320, a1321)?
                };
                nat_add(a1322, a1323)?
            };
            nat_add(a1324, a1325)
        }
    }
}

pub fn f265(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f322(v1, v0, v2)
}

pub fn f266(v0: Bytes, v1: u64, v2: Adt34) -> R<Option<Adt34>> {
    let m1327 = match v2.clone() {
        Adt34::C0(_, _, _, _, h1326) => {
            h1326
        }
    };
    match m1327 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt34::C0(h1328, _, _, _, _) => {
                    h1328
                }
            };
            let a1329 = v4;
            let a1330 = v1;
            if nat_lt(a1329, a1330) {
                let v5: u64 = f264(v0.clone(), v4)?;
                let a1331 = v5;
                let a1332 = 40u64;
                if nat_eq(a1331, a1332) {
                    Ok(Some(Adt34::C0({
                        let a1333 = v4;
                        let a1334 = 1u64;
                        nat_add(a1333, a1334)?
                    }, match v2.clone() {
                        Adt34::C0(_, h1335, _, _, _) => {
                            h1335
                        }
                    }, false, List::cons(Adt33::C0, f197(v0, v2)), None::<Adt28>)))
                } else {
                    let a1336 = v5;
                    let a1337 = 41u64;
                    if nat_eq(a1336, a1337) {
                        Ok(Some(Adt34::C0({
                            let a1338 = v4;
                            let a1339 = 1u64;
                            nat_add(a1338, a1339)?
                        }, match v2.clone() {
                            Adt34::C0(_, h1340, _, _, _) => {
                                h1340
                            }
                        }, false, List::cons(Adt33::C1, f197(v0, v2)), None::<Adt28>)))
                    } else {
                        let a1341 = v5;
                        let a1342 = 32u64;
                        if if nat_eq(a1341, a1342) {
                            true
                        } else {
                            let a1343 = v5;
                            let a1344 = 10u64;
                            nat_eq(a1343, a1344)
                        } {
                            Ok(Some(Adt34::C0({
                                let a1345 = v4;
                                let a1346 = 1u64;
                                nat_add(a1345, a1346)?
                            }, match v2.clone() {
                                Adt34::C0(_, h1347, _, _, _) => {
                                    h1347
                                }
                            }, false, f197(v0, v2), None::<Adt28>)))
                        } else {
                            let a1348 = v5;
                            let a1349 = 32u64;
                            if if nat_lt(a1348, a1349) {
                                true
                            } else {
                                let a1350 = v5;
                                let a1351 = 127u64;
                                nat_eq(a1350, a1351)
                            } {
                                Ok(Some(Adt34::C0(v4, match v2.clone() {
                                    Adt34::C0(_, h1352, _, _, _) => {
                                        h1352
                                    }
                                }, match v2.clone() {
                                    Adt34::C0(_, _, h1353, _, _) => {
                                        h1353
                                    }
                                }, match v2 {
                                    Adt34::C0(_, _, _, h1354, _) => {
                                        h1354
                                    }
                                }, Some(f200(f201(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt34::C0(_, _, h1355, _, _) => {
                                        h1355
                                    }
                                } {
                                    Ok(Some(Adt34::C0({
                                        let a1356 = v4;
                                        let a1357 = 1u64;
                                        nat_add(a1356, a1357)?
                                    }, match v2.clone() {
                                        Adt34::C0(_, h1358, _, _, _) => {
                                            h1358
                                        }
                                    }, true, match v2 {
                                        Adt34::C0(_, _, _, h1359, _) => {
                                            h1359
                                        }
                                    }, None::<Adt28>)))
                                } else {
                                    Ok(Some(Adt34::C0({
                                        let a1360 = v4;
                                        let a1361 = 1u64;
                                        nat_add(a1360, a1361)?
                                    }, v4, true, match v2 {
                                        Adt34::C0(_, _, _, h1362, _) => {
                                            h1362
                                        }
                                    }, None::<Adt28>)))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Adt34>)
            }
        }
        Some(_) => {
            Ok(None::<Adt34>)
        }
    }
}

pub fn f267(v0: Fn44, p1366: List<Adt33>, p1367: List<Adt33>) -> List<Adt33> {
    let m1365 = (p1366, p1367);
    {
        let mut m1365 = m1365;
        loop {
            let step = {
                let (v1, v2) = m1365;
                let m1363 = v2;
                match m1363.uncons() {
                    None => {
                        Err::<(List<Adt33>, List<Adt33>), List<Adt33>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt33>, List<Adt33>), List<Adt33>>(({
                            let c1364: Fn44 = v0.clone();
                            c1364.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: List<Adt33>, v1: Adt33) -> List<Adt33> {
    List::cons(v1, v0)
}

pub fn f269(v0: List<Adt33>, v1: u64, v2: Adt36) -> R<Option<Adt36>> {
    let m1369 = match v2.clone() {
        Adt36::C0(_, _, _, h1368, _) => {
            h1368
        }
    };
    match m1369 {
        None => {
            if match v2.clone() {
                Adt36::C0(_, _, _, _, h1370) => {
                    h1370
                }
            } {
                Ok(None::<Adt36>)
            } else {
                let m1374 = {
                    let a1372 = v0;
                    let a1373 = match v2.clone() {
                        Adt36::C0(h1371, _, _, _, _) => {
                            h1371
                        }
                    };
                    index_list(a1372, a1373)
                };
                match m1374 {
                    None => {
                        let m1376 = match v2.clone() {
                            Adt36::C0(_, h1375, _, _, _) => {
                                h1375
                            }
                        };
                        match m1376.uncons() {
                            None => {
                                let m1378 = match v2.clone() {
                                    Adt36::C0(_, _, h1377, _, _) => {
                                        h1377
                                    }
                                };
                                match m1378 {
                                    None => {
                                        Ok(Some(f323(v2, f200(f201(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt36::C0(match v2.clone() {
                                            Adt36::C0(h1379, _, _, _, _) => {
                                                h1379
                                            }
                                        }, List::<List<Adt35>>::nil(), match v2 {
                                            Adt36::C0(_, _, h1380, _, _) => {
                                                h1380
                                            }
                                        }, None::<Adt28>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f323(v2, f200(f201(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m1382 = match v2.clone() {
                            Adt36::C0(_, _, h1381, _, _) => {
                                h1381
                            }
                        };
                        match m1382 {
                            None => {
                                Ok(Some(f324(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f323(v2, f200(f201(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt36>)
        }
    }
}

pub fn f270(v0: u64, v1: Adt37) -> R<Option<Adt37>> {
    let m1384 = match v1.clone() {
        Adt37::C0(_, _, h1383) => {
            h1383
        }
    };
    match m1384 {
        None => {
            let m1386 = match v1.clone() {
                Adt37::C0(h1385, _, _) => {
                    h1385
                }
            };
            match m1386.uncons() {
                None => {
                    Ok(None::<Adt37>)
                }
                Some((v3, v4)) => {
                    let m1387 = v3;
                    match m1387 {
                        Adt38::C0(r1388) => {
                            let v5 = (*r1388).clone();
                            Ok(Some(f325(v1, v4, v5, v0)?))
                        }
                        Adt38::C1(r1389) => {
                            let v6 = (*r1389).clone();
                            Ok(Some(f326(v1, v4, v6, v0)?))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt37>)
        }
    }
}

pub fn f271(v0: Adt29, v1: List<Adt30>, v2: Adt1) -> R<Option<Adt29>> {
    let m1390 = v2;
    match m1390 {
        Adt1::C0 => {
            Ok(f327(v0, v1))
        }
        Adt1::C1(r1395) => {
            let v3 = (*r1395).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v3)), v1)))
        }
        Adt1::C2(_, _, r1396) => {
            let v6 = (*r1396).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v6)), v1)))
        }
        Adt1::C3(v7) => {
            if f328(match v0.clone() {
                Adt29::C0(_, h1391, _, _) => {
                    h1391
                }
            }, v7.clone())? {
                Ok(f327(v0, v1))
            } else {
                Ok(f329(v0, f330(f330(f331(Str::lit("local `")), v7), f331(Str::lit("` is not in scope")))))
            }
        }
        Adt1::C4(r1397, r1398) => {
            let v8 = (*r1397).clone();
            let v9 = (*r1398).clone();
            Ok(f327(v0, List::cons(Adt30::C0(std::rc::Rc::new(v8)), List::cons(Adt30::C0(std::rc::Rc::new(v9)), v1))))
        }
        Adt1::C5(r1399, r1400) => {
            let v10 = (*r1399).clone();
            let v11 = (*r1400).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(std::rc::Rc::new(v10)), List::cons(Adt30::C0(std::rc::Rc::new(v11)), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1392, _) => {
                    h1392
                }
            }), v1)))))
        }
        Adt1::C6(r1401, r1402) => {
            let v12 = (*r1401).clone();
            let v13 = (*r1402).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(std::rc::Rc::new(v12)), List::cons(Adt30::C0(std::rc::Rc::new(v13)), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1393, _) => {
                    h1393
                }
            }), v1)))))
        }
        Adt1::C7(v14, r1403, r1404, r1405) => {
            let v15 = (*r1403).clone();
            let v16 = (*r1404).clone();
            let v17 = (*r1405).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(std::rc::Rc::new(v15)), List::cons(Adt30::C0(std::rc::Rc::new(v16)), List::cons(Adt30::C1(v14), List::cons(Adt30::C0(std::rc::Rc::new(v17)), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1394, _) => {
                    h1394
                }
            }), v1)))))))
        }
        Adt1::C8(_) => {
            Ok(f327(v0, v1))
        }
        Adt1::C9 => {
            Ok(f327(v0, v1))
        }
        Adt1::C10(r1406, r1407) => {
            let v19 = (*r1406).clone();
            let v20 = (*r1407).clone();
            Ok(f327(v0, List::cons(Adt30::C0(std::rc::Rc::new(v19)), List::cons(Adt30::C0(std::rc::Rc::new(v20)), v1))))
        }
        Adt1::C11(_, v22, r1408, r1409) => {
            let v23 = (*r1408).clone();
            let v24 = (*r1409).clone();
            Ok(f327(v0, List::cons(Adt30::C0(std::rc::Rc::new(v23)), List::cons(Adt30::C1(v22), List::cons(Adt30::C0(std::rc::Rc::new(v24)), v1)))))
        }
    }
}

pub fn f272(v0: List<Bytes>, v1: u64) -> List<Bytes> {
    let (h1410, _) = f332(Fn52::F333, v1, v0);
    h1410
}

pub fn f273(v0: Adt29, v1: List<Adt30>, v2: List<Bytes>, v3: Adt2) -> R<Option<Adt29>> {
    let m1411 = v3;
    match m1411 {
        Adt2::C0(_) => {
            Ok(f327(v0, v1))
        }
        Adt2::C1(v5) => {
            if f328(v2, v5.clone())? {
                Ok(f327(v0, v1))
            } else {
                Ok(f329(v0, f330(f330(f331(Str::lit("universe variable `")), v5), f331(Str::lit("` is not declared")))))
            }
        }
        Adt2::C2(r1412) => {
            let v6 = (*r1412).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v6)), v1)))
        }
        Adt2::C3(r1413) => {
            let v7 = (*r1413).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v7)), v1)))
        }
        Adt2::C4(r1414, r1415) => {
            let v8 = (*r1414).clone();
            let v9 = (*r1415).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v8)), List::cons(Adt30::C3(std::rc::Rc::new(v9)), v1))))
        }
        Adt2::C5 => {
            Ok(f327(v0, v1))
        }
        Adt2::C6(r1416, r1417) => {
            let v10 = (*r1416).clone();
            let v11 = (*r1417).clone();
            Ok(f327(v0, List::cons(Adt30::C3(std::rc::Rc::new(v10)), List::cons(Adt30::C3(std::rc::Rc::new(v11)), v1))))
        }
    }
}

pub fn f274(v0: Adt2) -> Bytes {
    let m1418 = v0;
    match m1418 {
        Adt2::C0(_) => {
            Bytes::lit(&[])
        }
        Adt2::C1(_) => {
            Bytes::lit(&[])
        }
        Adt2::C2(_) => {
            Bytes::lit(&[])
        }
        Adt2::C3(_) => {
            Bytes::lit(&[])
        }
        Adt2::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt2::C5 => {
            Bytes::lit(&[])
        }
        Adt2::C6(r1419, r1420) => {
            let v7 = (*r1419).clone();
            let v8 = (*r1420).clone();
            f205(f205(f201(Str::lit(" ")), f206(v7)), f274(v8))
        }
    }
}

pub fn f275(v0: Fn45, p1424: Option<Bytes>, p1425: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m1423 = (p1424, p1425);
    {
        let mut m1423 = m1423;
        loop {
            let step = {
                let (v1, v2) = m1423;
                let m1421 = v2;
                match m1421.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c1422: Fn45 = v0.clone();
                            c1422.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m1426 = v1.clone();
    match m1426 {
        None => {
            let a1428 = {
                let (h1427, _) = v2;
                h1427
            };
            let a1429 = v0;
            if equal(a1428, a1429) {
                Some({
                    let (_, h1430) = v2;
                    h1430
                })
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f277(v0: u64) -> Bytes {
    let a1431 = v0;
    let a1432 = 1u64;
    if nat_eq(a1431, a1432) {
        f201(Str::lit("implicit"))
    } else {
        let a1433 = v0;
        let a1434 = 2u64;
        if nat_eq(a1433, a1434) {
            f201(Str::lit("instance"))
        } else {
            f201(Str::lit("explicit"))
        }
    }
}

pub fn f278(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt39>)> {
    f334(Fn53::F335(v0, v1), v2, (0u64, List::<Adt39>::nil()))
}

pub fn f279(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt39> {
    let v4: u64 = {
        let a1435 = 1024u64;
        let a1436 = v2;
        nat_mul(a1435, a1436)?
    };
    let v5: u64 = f336(v1, v4);
    let v6: u64 = f337(v5)?;
    let v7: u64 = {
        let a1437 = v6;
        let a1438 = 1u64;
        nat_sub(a1437, a1438)
    };
    Ok(f338(f339(v0.clone(), v4, v2, v7)?, f340(v0, {
        let a1441 = v4;
        let a1442 = {
            let a1439 = 64u64;
            let a1440 = v7;
            nat_mul(a1439, a1440)?
        };
        nat_add(a1441, a1442)?
    })?, v2, f341({
        let a1445 = v5;
        let a1446 = {
            let a1443 = 64u64;
            let a1444 = v7;
            nat_mul(a1443, a1444)?
        };
        nat_sub(a1445, a1446)
    }), {
        let a1449 = {
            let a1447 = v7;
            let a1448 = 0u64;
            if nat_eq(a1447, a1448) {
                3u32
            } else {
                2u32
            }
        };
        let a1450 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a1449, a1450)
    }))
}

pub fn f280(v0: Fn46, p1454: u64, p1455: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    let m1453 = (p1454, p1455);
    {
        let mut m1453 = m1453;
        loop {
            let step = {
                let (v1, v2) = m1453;
                let m1451 = v1;
                if m1451 == 0 {
                    Err::<(u64, (List<Adt39>, Adt39)), (List<Adt39>, Adt39)>(v2)
                } else {
                    let v3 = m1451 - 1;
                    Ok::<(u64, (List<Adt39>, Adt39)), (List<Adt39>, Adt39)>((v3, {
                        let c1452: Fn46 = v0.clone();
                        c1452.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    f342(v0)
}

pub fn f282(v0: u32, v1: u32) -> Bytes {
    f343(f344(v0, v1))
}

pub fn f283(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1456 = v2;
        let a1457 = v0;
        equal(a1456, a1457)
    }
}

pub fn f284(v0: Bytes) -> R<bool> {
    f345(v0)
}

pub fn f285(v0: Bytes, v1: Bytes, v2: Bytes, v3: List<(Bytes, Bytes)>) -> R<Adt42> {
    let m1458 = f7(2u64, v1)?;
    match m1458 {
        None => {
            Ok(Adt42::C0(f46(v3, Some(f2(f3(Str::lit("a form channel is text, math, or both"))))), None::<u64>, List::<Adt5>::nil()))
        }
        Some(v4) => {
            if f346(v2.clone())? {
                Ok(Adt42::C0(f46(v3, Some(f2(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: leading, trailing, or repeated whitespace in the surface is forbidden")))))), None::<u64>, List::<Adt5>::nil()))
            } else {
                let m1460 = f347(f3(Str::lit("<form>")), v2.clone(), {
                    let a1459 = v2;
                    length_bytes(a1459)
                })?;
                match m1460 {
                    Ok(v6) => {
                        Ok(Adt42::C0(v3, Some(v4), v6))
                    }
                    Err(v5) => {
                        Ok(Adt42::C0(f46(v3, Some(f2(f5(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: surface does not scan: "))), match v5 {
                            Adt48::C0(_, h1461, _, _) => {
                                h1461
                            }
                        })))), None::<u64>, List::<Adt5>::nil()))
                    }
                }
            }
        }
    }
}

pub fn f286(v0: List<Bytes>, v1: Bytes, v2: List<Adt5>, v3: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f348(Fn54::F349(v0, v1), v3, v2)
}

pub fn f287(v0: List<Adt5>) -> bool {
    f350(Fn55::F351, false, v0)
}

pub fn f288(v0: List<Adt5>, v1: u64) -> R<Option<Bytes>> {
    let (_, h1462) = f352(Fn56::F353(v1), (0u64, None::<Bytes>), v0)?;
    Ok(h1462)
}

pub fn f289(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLR3006")), v0)
}

pub fn f290(v0: Bytes, v1: List<Bytes>, v2: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v3: List<(Bytes, Bytes)> = if f49(v1.clone()) {
        v2
    } else {
        List::cons(f2(f5(f5(f3(Str::lit("form `")), v0.clone()), f3(Str::lit("`: features must be sorted and unique")))), v2)
    };
    f228(Fn37::F354(v0), v3, v1)
}

pub fn f291(v0: Fn47, p1467: u64, p1468: Adt43) -> R<(Adt43, bool)> {
    let m1466 = (p1467, p1468);
    {
        let mut m1466 = m1466;
        loop {
            let step = {
                let (v1, v2) = m1466;
                let m1463 = v1;
                if m1463 == 0 {
                    Err::<(u64, Adt43), R<(Adt43, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1463 - 1;
                    let m1465 = {
                        let c1464: Fn47 = v0.clone();
                        c1464.apply(v2.clone())?
                    };
                    match m1465 {
                        None => {
                            Err::<(u64, Adt43), R<(Adt43, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt43), R<(Adt43, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: u64, v1: Adt43) -> R<Option<Adt43>> {
    f355(v0, v1)
}

pub fn f293(v0: u64, v1: Adt40) -> R<Option<Adt40>> {
    let m1470 = match v1.clone() {
        Adt40::C0(h1469, _) => {
            h1469
        }
    };
    match m1470.uncons() {
        None => {
            Ok(None::<Adt40>)
        }
        Some((v2, v3)) => {
            let v4: Adt31 = match v1 {
                Adt40::C0(_, h1471) => {
                    (*h1471).clone()
                }
            };
            let m1472 = v2;
            match m1472 {
                Adt7::C0(v5, v6, v7) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1473, _, _, _, _, _, _) => {
                            h1473
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1474, _, _, _, _, _) => {
                            h1474
                        }
                    }, List::cons((v5, (v6, v7)), match v4.clone() {
                        Adt31::C0(_, _, h1475, _, _, _, _) => {
                            h1475
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, _, h1476, _, _, _) => {
                            h1476
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1477, _, _) => {
                            h1477
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1478, _) => {
                            h1478
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1479) => {
                            h1479
                        }
                    })))
                }
                Adt7::C1(v8) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1480, _, _, _, _, _, _) => {
                            h1480
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1481, _, _, _, _, _) => {
                            h1481
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1482, _, _, _, _) => {
                            h1482
                        }
                    }, List::cons(v8, match v4.clone() {
                        Adt31::C0(_, _, _, h1483, _, _, _) => {
                            h1483
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, _, _, h1484, _, _) => {
                            h1484
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1485, _) => {
                            h1485
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1486) => {
                            h1486
                        }
                    })))
                }
                Adt7::C2(v9) => {
                    Ok(f356(v3, Adt31::C0(List::cons(v9, match v4.clone() {
                        Adt31::C0(h1487, _, _, _, _, _, _) => {
                            h1487
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, h1488, _, _, _, _, _) => {
                            h1488
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1489, _, _, _, _) => {
                            h1489
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1490, _, _, _) => {
                            h1490
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1491, _, _) => {
                            h1491
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1492, _) => {
                            h1492
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1493) => {
                            h1493
                        }
                    })))
                }
                Adt7::C3(r1508) => {
                    let v10 = (*r1508).clone();
                    Ok(f356(f357(Fn57::F358, v3, f359(f360(v10, v0)?)), v4))
                }
                Adt7::C4 => {
                    Ok(f356(v3, v4))
                }
                Adt7::C5(v11) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1494, _, _, _, _, _, _) => {
                            h1494
                        }
                    }, List::cons(v11, match v4.clone() {
                        Adt31::C0(_, h1495, _, _, _, _, _) => {
                            h1495
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, h1496, _, _, _, _) => {
                            h1496
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1497, _, _, _) => {
                            h1497
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1498, _, _) => {
                            h1498
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1499, _) => {
                            h1499
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1500) => {
                            h1500
                        }
                    })))
                }
                Adt7::C6(r1509) => {
                    let v12 = (*r1509).clone();
                    Ok(f356(List::cons(v12, v3), v4))
                }
                Adt7::C7(r1510) => {
                    let v13 = (*r1510).clone();
                    Ok(f356(List::cons(v13, v3), v4))
                }
                Adt7::C8(r1511) => {
                    let v14 = (*r1511).clone();
                    Ok(f356(List::cons(v14, v3), v4))
                }
                Adt7::C9(r1512, r1513) => {
                    let v15 = (*r1512).clone();
                    let v16 = (*r1513).clone();
                    Ok(f356(List::cons(v15.clone(), List::cons(v16.clone(), v3)), f361(v4, f201(Str::lit("sub")), v15, v16, v0)?))
                }
                Adt7::C10(r1514, r1515) => {
                    let v17 = (*r1514).clone();
                    let v18 = (*r1515).clone();
                    Ok(f356(List::cons(v17.clone(), List::cons(v18.clone(), v3)), f361(v4, f201(Str::lit("sup")), v17, v18, v0)?))
                }
                Adt7::C11(r1516, r1517) => {
                    let v19 = (*r1516).clone();
                    let v20 = (*r1517).clone();
                    Ok(f356(List::cons(v19.clone(), List::cons(v20.clone(), v3)), f361(v4, f201(Str::lit("frac")), v19, v20, v0)?))
                }
                Adt7::C12(v21) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1501, _, _, _, _, _, _) => {
                            h1501
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1502, _, _, _, _, _) => {
                            h1502
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1503, _, _, _, _) => {
                            h1503
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1504, _, _, _) => {
                            h1504
                        }
                    }, List::cons(v21, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1505, _, _) => {
                            h1505
                        }
                    }), f362(match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1506, _) => {
                            h1506
                        }
                    }, f201(Str::lit("operator-name"))), match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1507) => {
                            h1507
                        }
                    })))
                }
                Adt7::C13 => {
                    Ok(f356(v3, v4))
                }
                Adt7::C14(_, _) => {
                    Ok(f356(v3, v4))
                }
            }
        }
    }
}

pub fn f294(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f295(v0: Fn48, p1521: List<(Bytes, (Bytes, Bytes))>, p1522: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1520 = (p1521, p1522);
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let (v1, v2) = m1520;
                let m1518 = v2;
                match m1518.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c1519: Fn48 = v0.clone();
                            c1519.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(v1, v0)
}

pub fn f297(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: (bool, List<u64>) = f363(Fn58::F364(v1), (false, List::<u64>::nil()), v0);
    let m1524 = {
        let (h1523, _) = v2;
        h1523
    };
    if m1524 {
        f220({
            let (_, h1525) = v2;
            h1525
        })
    } else {
        f220(List::cons(v1, {
            let (_, h1526) = v2;
            h1526
        }))
    }
}

pub fn f298(v0: List<Adt4>, v1: u64, v2: Bytes, v3: Bytes, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let m1527 = f365(v0, v3.clone());
    match m1527 {
        None => {
            Ok(List::cons(f2(f5(f5(f5(v2, f3(Str::lit(" render references unknown self form "))), f6(v3)), Bytes::lit(&[]))), v4))
        }
        Some(v5) => {
            let v6: List<(Bytes, Bytes)> = f46(v4, if f216(match v5.clone() {
                Adt4::C0(_, h1528, _, _, _, _) => {
                    h1528
                }
            }, v1) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f5(v2.clone(), f3(Str::lit(" render references self form "))), f6(v3.clone())), f5(f3(Str::lit(", which is not a ")), f5(v2.clone(), f3(Str::lit(" form")))))))
            });
            let m1531 = f288(match v5.clone() {
                Adt4::C0(_, _, _, h1529, _, _) => {
                    h1529
                }
            }, match v5.clone() {
                Adt4::C0(_, h1530, _, _, _, _) => {
                    h1530
                }
            })?;
            match m1531 {
                None => {
                    Ok(v6)
                }
                Some(v7) => {
                    Ok(f46(v6, Some(f289(f5(f5(f5(f5(f5(v2, f3(Str::lit(" render references self form "))), f6(v3)), f3(Str::lit(" whose surface `"))), match v5 {
                        Adt4::C0(_, _, h1532, _, _, _) => {
                            h1532
                        }
                    }), f5(f3(Str::lit("` is not renderer-safe: ")), v7))))))
                }
            }
        }
    }
}

pub fn f299(v0: u64) -> bool {
    if f263(v0) {
        true
    } else {
        let a1533 = v0;
        let a1534 = 95u64;
        nat_eq(a1533, a1534)
    }
}

pub fn f300(v0: u64) -> bool {
    if f366(v0) {
        true
    } else {
        let a1535 = v0;
        let a1536 = 95u64;
        nat_eq(a1535, a1536)
    }
}

pub fn f301(v0: Bytes) -> Bytes {
    f205(f205(f201(Str::lit("`")), v0), f201(Str::lit("`")))
}

pub fn f302(v0: Bytes) -> R<bool> {
    f112(v0, false)
}

pub fn f303(v0: Fn49, p1540: (List<(Bytes, Bytes)>, List<Bytes>), p1541: List<Bytes>) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    let m1539 = (p1540, p1541);
    {
        let mut m1539 = m1539;
        loop {
            let step = {
                let (v1, v2) = m1539;
                let m1537 = v2;
                match m1537.uncons() {
                    None => {
                        Err::<((List<(Bytes, Bytes)>, List<Bytes>), List<Bytes>), R<(List<(Bytes, Bytes)>, List<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<(Bytes, Bytes)>, List<Bytes>), List<Bytes>), R<(List<(Bytes, Bytes)>, List<Bytes>)>>(({
                            let c1538: Fn49 = v0.clone();
                            c1538.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: (List<(Bytes, Bytes)>, List<Bytes>), v1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    f367(v0, v1)
}

pub fn f305(v0: Bytes, v1: u64) -> Option<u64> {
    let a1542 = 0u64;
    let a1543 = v1;
    if nat_lt(a1542, a1543) {
        if f368(v0, {
            let a1544 = v1;
            let a1545 = 1u64;
            nat_sub(a1544, a1545)
        }) {
            Some({
                let a1546 = v1;
                let a1547 = 1u64;
                nat_sub(a1546, a1547)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f306(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1548 = v2;
    let a1549 = v0;
    if nat_lt(a1548, a1549) {
        if f368(v1, v2) {
            Ok(Some({
                let a1550 = v2;
                let a1551 = 1u64;
                nat_add(a1550, a1551)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f307(v0: Bytes, v1: u64, v2: Adt41) -> R<Option<Adt41>> {
    let a1553 = match v2.clone() {
        Adt41::C0(_, _, h1552) => {
            h1552
        }
    };
    let a1554 = 0u64;
    if equal(a1553, a1554) {
        let v3: u64 = f369(v0.clone(), match v2.clone() {
            Adt41::C0(h1555, _, _) => {
                h1555
            }
        })?;
        let a1556 = v3;
        let a1557 = v1;
        if nat_lt(a1556, a1557) {
            let m1558 = f247(v0.clone(), v3);
            match m1558 {
                None => {
                    Ok(None::<Adt41>)
                }
                Some(v4) => {
                    let a1559 = v4;
                    let a1560 = 34u8;
                    if equal(a1559, a1560) {
                        let m1561 = f245(v0.clone(), v3)?;
                        match m1561 {
                            None => {
                                Ok(Some(Adt41::C0(v3, match v2 {
                                    Adt41::C0(_, h1576, _) => {
                                        h1576
                                    }
                                }, 2u64)))
                            }
                            Some(v5) => {
                                let v6: u64 = f369(v0.clone(), {
                                    let (_, h1562) = v5;
                                    h1562
                                })?;
                                let a1563 = v6;
                                let a1564 = v1;
                                if nat_lt(a1563, a1564) {
                                    let m1565 = f247(v0.clone(), v6);
                                    match m1565 {
                                        None => {
                                            Ok(None::<Adt41>)
                                        }
                                        Some(v7) => {
                                            let a1566 = v7;
                                            let a1567 = 44u8;
                                            if equal(a1566, a1567) {
                                                Ok(Some(Adt41::C0(f369(v0, {
                                                    let a1568 = v6;
                                                    let a1569 = 1u64;
                                                    nat_add(a1568, a1569)?
                                                })?, List::cons({
                                                    let (h1570, _) = v5;
                                                    h1570
                                                }, match v2 {
                                                    Adt41::C0(_, h1571, _) => {
                                                        h1571
                                                    }
                                                }), 0u64)))
                                            } else {
                                                Ok(Some(Adt41::C0(v6, List::cons({
                                                    let (h1572, _) = v5;
                                                    h1572
                                                }, match v2 {
                                                    Adt41::C0(_, h1573, _) => {
                                                        h1573
                                                    }
                                                }), 2u64)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok(Some(Adt41::C0(v6, List::cons({
                                        let (h1574, _) = v5;
                                        h1574
                                    }, match v2 {
                                        Adt41::C0(_, h1575, _) => {
                                            h1575
                                        }
                                    }), 1u64)))
                                }
                            }
                        }
                    } else {
                        Ok(Some(Adt41::C0(v3, match v2 {
                            Adt41::C0(_, h1577, _) => {
                                h1577
                            }
                        }, 2u64)))
                    }
                }
            }
        } else {
            Ok(Some(Adt41::C0(v3, match v2 {
                Adt41::C0(_, h1578, _) => {
                    h1578
                }
            }, 1u64)))
        }
    } else {
        Ok(None::<Adt41>)
    }
}

pub fn f308(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f370(v1, v0, v2)
}

pub fn f309(v0: Fn50, p1583: u64, p1584: Adt45) -> R<(Adt45, bool)> {
    let m1582 = (p1583, p1584);
    {
        let mut m1582 = m1582;
        loop {
            let step = {
                let (v1, v2) = m1582;
                let m1579 = v1;
                if m1579 == 0 {
                    Err::<(u64, Adt45), R<(Adt45, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1579 - 1;
                    let m1581 = {
                        let c1580: Fn50 = v0.clone();
                        c1580.apply(v2.clone())?
                    };
                    match m1581 {
                        None => {
                            Err::<(u64, Adt45), R<(Adt45, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt45), R<(Adt45, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: u64, v2: Adt45) -> R<Option<Adt45>> {
    f371(v0, v1, v2)
}

pub fn f311(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1585 = v0.clone();
        length_bytes(a1585)
    };
    let a1586 = 0u64;
    let a1587 = v1;
    if nat_lt(a1586, a1587) {
        let a1588 = f372(v0.clone())?;
        let a1589 = v1;
        if nat_eq(a1588, a1589) {
            let a1590 = 1u64;
            let a1591 = v1;
            if nat_lt(a1590, a1591) {
                let m1594 = {
                    let a1592 = v0;
                    let a1593 = 0u64;
                    index_bytes(a1592, a1593)
                };
                match m1594 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1595 = v2;
                        let a1596 = 48u8;
                        Ok(!equal(a1595, a1596))
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

pub fn f312(v0: Fn51, p1601: u64, p1602: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1600 = (p1601, p1602);
    {
        let mut m1600 = m1600;
        loop {
            let step = {
                let (v1, v2) = m1600;
                let m1597 = v1;
                if m1597 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1597 - 1;
                    let m1599 = {
                        let c1598: Fn51 = v0.clone();
                        c1598.apply(v2)?
                    };
                    match m1599 {
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
                Ok(next) => { m1600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f373(v0, v1)
}

pub fn f314(v0: List<Adt16>, v1: Adt16) -> List<Adt16> {
    List::cons(v1, v0)
}

pub fn f315(v0: Adt16, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<Adt16> {
    if f176(v0.clone(), v1, v2)? {
        Ok(Adt16::C0(match v0.clone() {
            Adt16::C0(h1603, _, _) => {
                h1603
            }
        }, match v0.clone() {
            Adt16::C0(_, h1604, _) => {
                h1604
            }
        }, f374(List::cons(v3, f374(match v0 {
            Adt16::C0(_, _, h1605) => {
                h1605
            }
        })))))
    } else {
        Ok(v0)
    }
}

pub fn f316(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m1606 = f117(Fn13::F375(v1), None::<Bytes>, v0)?;
    match m1606 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f317() -> List<Bytes> {
    List::cons(f3(Str::lit("entry")), List::cons(f3(Str::lit("lean_name")), List::cons(f3(Str::lit("fields")), List::cons(f3(Str::lit("induction_hypotheses")), List::<Bytes>::nil()))))
}

pub fn f318(v0: Adt32) -> Option<Adt32> {
    Some(Adt32::C0(match v0.clone() {
        Adt32::C0(h1607, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt32::C0(_, h1608, _, _) => {
            h1608
        }
    }, match v0 {
        Adt32::C0(_, _, h1609, _) => {
            h1609
        }
    }, false))
}

pub fn f319(v0: Adt32, v1: bool, v2: bool) -> R<Option<Adt32>> {
    Ok(Some(Adt32::C0({
        let a1611 = match v0 {
            Adt32::C0(h1610, _, _, _) => {
                h1610
            }
        };
        let a1612 = 1u64;
        nat_add(a1611, a1612)?
    }, v1, v2, true)))
}

pub fn f320(v0: u64) -> bool {
    let a1613 = v0;
    let a1614 = 97u64;
    if nat_lt(a1613, a1614) {
        false
    } else {
        let a1615 = v0;
        let a1616 = 122u64;
        nat_le(a1615, a1616)
    }
}

pub fn f321(v0: u64) -> bool {
    let a1617 = v0;
    let a1618 = 48u64;
    if nat_lt(a1617, a1618) {
        false
    } else {
        let a1619 = v0;
        let a1620 = 57u64;
        nat_le(a1619, a1620)
    }
}

pub fn f322(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1621 = v2;
    let a1622 = v1;
    if nat_lt(a1621, a1622) {
        if f366(f264(v0, v2)?) {
            Ok(Some({
                let a1623 = v2;
                let a1624 = 1u64;
                nat_add(a1623, a1624)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f323(v0: Adt36, v1: Adt28) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h1625, _, _, _, _) => {
            h1625
        }
    }, match v0.clone() {
        Adt36::C0(_, h1626, _, _, _) => {
            h1626
        }
    }, match v0 {
        Adt36::C0(_, _, h1627, _, _) => {
            h1627
        }
    }, Some(v1), false)
}

pub fn f324(v0: u64, v1: Adt36, v2: Adt33) -> R<Adt36> {
    let m1628 = v2;
    match m1628 {
        Adt33::C0 => {
            let a1631 = {
                let a1630 = match v1.clone() {
                    Adt36::C0(_, h1629, _, _, _) => {
                        h1629
                    }
                };
                length_list(a1630)
            };
            let a1632 = v0;
            if nat_lt(a1631, a1632) {
                Ok(Adt36::C0({
                    let a1634 = match v1.clone() {
                        Adt36::C0(h1633, _, _, _, _) => {
                            h1633
                        }
                    };
                    let a1635 = 1u64;
                    nat_add(a1634, a1635)?
                }, List::cons(List::<Adt35>::nil(), match v1.clone() {
                    Adt36::C0(_, h1636, _, _, _) => {
                        h1636
                    }
                }), match v1 {
                    Adt36::C0(_, _, h1637, _, _) => {
                        h1637
                    }
                }, None::<Adt28>, false))
            } else {
                Ok(f323(v1, Adt28::C0(1u64, f205(f201(Str::lit("max_scope_depth exceeded: configured ")), f48(v0)))))
            }
        }
        Adt33::C1 => {
            let m1639 = match v1.clone() {
                Adt36::C0(_, h1638, _, _, _) => {
                    h1638
                }
            };
            match m1639.uncons() {
                None => {
                    Ok(f323(v1, f200(f201(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f376(v1, Adt35::C1(std::rc::Rc::new(f377(v3))), v4)
                }
            }
        }
        Adt33::C2(v5) => {
            f376(v1.clone(), Adt35::C0(v5), match v1 {
                Adt36::C0(_, h1640, _, _, _) => {
                    h1640
                }
            })
        }
    }
}

pub fn f325(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: u64) -> R<Adt37> {
    let m1641 = v2.clone();
    match m1641 {
        Adt35::C0(_) => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C1(r1644) => {
            let v4 = (*r1644).clone();
            let m1642 = f378(v4, v3)?;
            match m1642.uncons() {
                None => {
                    Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                }
                Some((v5, v6)) => {
                    let m1643 = v5;
                    match m1643 {
                        Adt35::C0(v7) => {
                            f380(v0, v1, v2, v7, v6, v3)
                        }
                        Adt35::C1(_) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                        Adt35::C2 => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C3(_, _) => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
    }
}

pub fn f326(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: u64) -> R<Adt37> {
    let m1645 = v2;
    match m1645 {
        Adt35::C0(_) => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
        Adt35::C1(r1673) => {
            let v4 = (*r1673).clone();
            let v5: List<Adt35> = f378(v4, v3)?;
            let m1646 = v5;
            match m1646.uncons() {
                None => {
                    Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
                }
                Some((v6, v7)) => {
                    let m1647 = v6;
                    match m1647 {
                        Adt35::C0(v8) => {
                            let a1648 = v8.clone();
                            let a1649 = f201(Str::lit("app"));
                            if equal(a1648, a1649) {
                                let v9: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                    Adt37::C0(_, h1650, _) => {
                                        h1650
                                    }
                                }, {
                                    let a1651 = v7;
                                    length_list(a1651)
                                });
                                let m1653 = {
                                    let (h1652, _) = v9;
                                    h1652
                                };
                                match m1653.uncons() {
                                    None => {
                                        Ok(f379(v0, f200(f201(Str::lit("missing function")))))
                                    }
                                    Some((v10, v11)) => {
                                        Ok(Adt37::C0(v1, List::cons(Adt1::C4(std::rc::Rc::new(v10), std::rc::Rc::new(f383(v11))), {
                                            let (_, h1654) = v9;
                                            h1654
                                        }), None::<Adt28>))
                                    }
                                }
                            } else {
                                if f384(v8.clone()) {
                                    let m1655 = f385(v7, 0u64);
                                    match m1655 {
                                        Adt35::C0(_) => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                        Adt35::C1(r1665) => {
                                            let v12 = (*r1665).clone();
                                            let v13: List<Adt35> = f378(v12, v3)?;
                                            let v14: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                                Adt37::C0(_, h1656, _) => {
                                                    h1656
                                                }
                                            }, {
                                                let a1658 = {
                                                    let a1657 = v13.clone();
                                                    length_list(a1657)
                                                };
                                                let a1659 = 1u64;
                                                nat_add(a1658, a1659)?
                                            });
                                            let m1661 = f386({
                                                let (h1660, _) = v14;
                                                h1660
                                            });
                                            match m1661.uncons() {
                                                None => {
                                                    Ok(f379(v0, f200(f201(Str::lit("missing body")))))
                                                }
                                                Some((v15, v16)) => {
                                                    let v17: Adt1 = f387(v13, f386(v16), v3)?;
                                                    Ok(Adt37::C0(v1, List::cons({
                                                        let a1662 = v8;
                                                        let a1663 = f201(Str::lit("pi"));
                                                        if equal(a1662, a1663) {
                                                            Adt1::C5(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        } else {
                                                            Adt1::C6(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        }
                                                    }, {
                                                        let (_, h1664) = v14;
                                                        h1664
                                                    }), None::<Adt28>))
                                                }
                                            }
                                        }
                                        Adt35::C2 => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                        Adt35::C3(_, _) => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                    }
                                } else {
                                    let v21: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                        Adt37::C0(_, h1666, _) => {
                                            h1666
                                        }
                                    }, 3u64);
                                    let m1668 = {
                                        let (h1667, _) = v21;
                                        h1667
                                    };
                                    match m1668.uncons() {
                                        None => {
                                            Ok(f379(v0, f200(f201(Str::lit("missing type")))))
                                        }
                                        Some((v22, v23)) => {
                                            let m1669 = v23;
                                            match m1669.uncons() {
                                                None => {
                                                    Ok(f379(v0, f200(f201(Str::lit("missing value")))))
                                                }
                                                Some((v24, v25)) => {
                                                    let m1670 = v25;
                                                    match m1670.uncons() {
                                                        None => {
                                                            Ok(f379(v0, f200(f201(Str::lit("missing body")))))
                                                        }
                                                        Some((v26, _)) => {
                                                            let m1671 = f388(f385(v7, 0u64), f201(Str::lit("a local ID")), v3);
                                                            match m1671 {
                                                                Ok(v28) => {
                                                                    Ok(Adt37::C0(v1, List::cons(Adt1::C7(v28, std::rc::Rc::new(v22), std::rc::Rc::new(v24), std::rc::Rc::new(v26)), {
                                                                        let (_, h1672) = v21;
                                                                        h1672
                                                                    }), None::<Adt28>))
                                                                }
                                                                Err(v29) => {
                                                                    Ok(f379(v0, v29))
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
                        Adt35::C1(_) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                        Adt35::C2 => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
        Adt35::C3(_, _) => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
    }
}

pub fn f327(v0: Adt29, v1: List<Adt30>) -> Option<Adt29> {
    Some(Adt29::C0(v1, match v0.clone() {
        Adt29::C0(_, h1674, _, _) => {
            h1674
        }
    }, match v0 {
        Adt29::C0(_, _, h1675, _) => {
            h1675
        }
    }, None::<Bytes>))
}

pub fn f328(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f88(Fn8::F389(v1), false, v0)
}

pub fn f329(v0: Adt29, v1: Bytes) -> Option<Adt29> {
    Some(Adt29::C0(match v0.clone() {
        Adt29::C0(h1676, _, _, _) => {
            h1676
        }
    }, match v0.clone() {
        Adt29::C0(_, h1677, _, _) => {
            h1677
        }
    }, match v0 {
        Adt29::C0(_, _, h1678, _) => {
            h1678
        }
    }, Some(v1)))
}

pub fn f330(v0: Bytes, v1: Bytes) -> Bytes {
    let a1679 = v0;
    let a1680 = v1;
    append_bytes(a1679, a1680)
}

pub fn f331(v0: Str) -> Bytes {
    let a1681 = v0;
    utf8_encode(a1681)
}

pub fn f332(v0: Fn52, p1686: u64, p1687: List<Bytes>) -> (List<Bytes>, bool) {
    let m1685 = (p1686, p1687);
    {
        let mut m1685 = m1685;
        loop {
            let step = {
                let (v1, v2) = m1685;
                let m1682 = v1;
                if m1682 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m1682 - 1;
                    let m1684 = {
                        let c1683: Fn52 = v0.clone();
                        c1683.apply(v2.clone())
                    };
                    match m1684 {
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
                Ok(next) => { m1685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1688 = v0;
    match m1688.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            Some(v2)
        }
    }
}

pub fn f334(v0: Fn53, p1692: u64, p1693: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
    let m1691 = (p1692, p1693);
    {
        let mut m1691 = m1691;
        loop {
            let step = {
                let (v1, v2) = m1691;
                let m1689 = v1;
                if m1689 == 0 {
                    Err::<(u64, (u64, List<Adt39>)), R<(u64, List<Adt39>)>>(Ok(v2))
                } else {
                    let v3 = m1689 - 1;
                    Ok::<(u64, (u64, List<Adt39>)), R<(u64, List<Adt39>)>>((v3, {
                        let c1690: Fn53 = v0.clone();
                        c1690.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes, v1: u64, v2: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
    f390(v2.clone(), f279(v0, v1, {
        let (h1694, _) = v2;
        h1694
    }, false)?)
}

pub fn f336(v0: u64, v1: u64) -> u64 {
    let a1697 = 1024u64;
    let a1698 = {
        let a1695 = v0;
        let a1696 = v1;
        nat_sub(a1695, a1696)
    };
    if nat_lt(a1697, a1698) {
        1024u64
    } else {
        let a1699 = v0;
        let a1700 = v1;
        nat_sub(a1699, a1700)
    }
}

pub fn f337(v0: u64) -> R<u64> {
    let a1701 = v0;
    let a1702 = 0u64;
    if nat_eq(a1701, a1702) {
        Ok(1u64)
    } else {
        let a1705 = {
            let a1703 = v0;
            let a1704 = 63u64;
            nat_add(a1703, a1704)?
        };
        let a1706 = 64u64;
        let a1707 = 0u64;
        Ok(nat_quot(a1705, a1706, a1707))
    }
}

pub fn f338(v0: Adt39, v1: Adt46, v2: u64, v3: u32, v4: u32) -> Adt39 {
    let v5: Adt47 = {
        let (h1708, _) = f391(Fn59::F392, 7u64, (Adt47::C0(match v0.clone() {
            Adt39::C0(h1709, _, _, _, _, _, _, _) => {
                h1709
            }
        }, match v0.clone() {
            Adt39::C0(_, h1710, _, _, _, _, _, _) => {
                h1710
            }
        }, match v0.clone() {
            Adt39::C0(_, _, h1711, _, _, _, _, _) => {
                h1711
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, h1712, _, _, _, _) => {
                h1712
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, h1713, _, _, _) => {
                h1713
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, _, h1714, _, _) => {
                h1714
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, _, _, h1715, _) => {
                h1715
            }
        }, match v0 {
            Adt39::C0(_, _, _, _, _, _, _, h1716) => {
                h1716
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f341({
            let a1717 = v2;
            let a1718 = 4294967296u64;
            let a1719 = 0u64;
            nat_rem(a1717, a1718, a1719)
        }), f341({
            let a1720 = v2;
            let a1721 = 4294967296u64;
            let a1722 = 0u64;
            nat_quot(a1720, a1721, a1722)
        }), v3, v4), v1));
        h1708
    };
    Adt39::C0({
        let a1725 = match v5.clone() {
            Adt47::C0(h1723, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1723
            }
        };
        let a1726 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, h1724, _, _, _, _, _, _, _) => {
                h1724
            }
        };
        fixed_u32::bit_xor(a1725, a1726)
    }, {
        let a1729 = match v5.clone() {
            Adt47::C0(_, h1727, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1727
            }
        };
        let a1730 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, h1728, _, _, _, _, _, _) => {
                h1728
            }
        };
        fixed_u32::bit_xor(a1729, a1730)
    }, {
        let a1733 = match v5.clone() {
            Adt47::C0(_, _, h1731, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1731
            }
        };
        let a1734 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, h1732, _, _, _, _, _) => {
                h1732
            }
        };
        fixed_u32::bit_xor(a1733, a1734)
    }, {
        let a1737 = match v5.clone() {
            Adt47::C0(_, _, _, h1735, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1735
            }
        };
        let a1738 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h1736, _, _, _, _) => {
                h1736
            }
        };
        fixed_u32::bit_xor(a1737, a1738)
    }, {
        let a1741 = match v5.clone() {
            Adt47::C0(_, _, _, _, h1739, _, _, _, _, _, _, _, _, _, _, _) => {
                h1739
            }
        };
        let a1742 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1740, _, _, _) => {
                h1740
            }
        };
        fixed_u32::bit_xor(a1741, a1742)
    }, {
        let a1745 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, h1743, _, _, _, _, _, _, _, _, _, _) => {
                h1743
            }
        };
        let a1746 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1744, _, _) => {
                h1744
            }
        };
        fixed_u32::bit_xor(a1745, a1746)
    }, {
        let a1749 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, h1747, _, _, _, _, _, _, _, _, _) => {
                h1747
            }
        };
        let a1750 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1748, _) => {
                h1748
            }
        };
        fixed_u32::bit_xor(a1749, a1750)
    }, {
        let a1753 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, h1751, _, _, _, _, _, _, _, _) => {
                h1751
            }
        };
        let a1754 = match v5 {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1752) => {
                h1752
            }
        };
        fixed_u32::bit_xor(a1753, a1754)
    })
}

pub fn f339(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt39> {
    let (_, h1755) = f393(Fn60::F394(v2, v0, v1), v3, (0u64, f395()))?;
    Ok(h1755)
}

pub fn f340(v0: Bytes, v1: u64) -> R<Adt46> {
    Ok(Adt46::C0(f396(v0.clone(), {
        let a1756 = v1;
        let a1757 = 0u64;
        nat_add(a1756, a1757)?
    })?, f396(v0.clone(), {
        let a1758 = v1;
        let a1759 = 4u64;
        nat_add(a1758, a1759)?
    })?, f396(v0.clone(), {
        let a1760 = v1;
        let a1761 = 8u64;
        nat_add(a1760, a1761)?
    })?, f396(v0.clone(), {
        let a1762 = v1;
        let a1763 = 12u64;
        nat_add(a1762, a1763)?
    })?, f396(v0.clone(), {
        let a1764 = v1;
        let a1765 = 16u64;
        nat_add(a1764, a1765)?
    })?, f396(v0.clone(), {
        let a1766 = v1;
        let a1767 = 20u64;
        nat_add(a1766, a1767)?
    })?, f396(v0.clone(), {
        let a1768 = v1;
        let a1769 = 24u64;
        nat_add(a1768, a1769)?
    })?, f396(v0.clone(), {
        let a1770 = v1;
        let a1771 = 28u64;
        nat_add(a1770, a1771)?
    })?, f396(v0.clone(), {
        let a1772 = v1;
        let a1773 = 32u64;
        nat_add(a1772, a1773)?
    })?, f396(v0.clone(), {
        let a1774 = v1;
        let a1775 = 36u64;
        nat_add(a1774, a1775)?
    })?, f396(v0.clone(), {
        let a1776 = v1;
        let a1777 = 40u64;
        nat_add(a1776, a1777)?
    })?, f396(v0.clone(), {
        let a1778 = v1;
        let a1779 = 44u64;
        nat_add(a1778, a1779)?
    })?, f396(v0.clone(), {
        let a1780 = v1;
        let a1781 = 48u64;
        nat_add(a1780, a1781)?
    })?, f396(v0.clone(), {
        let a1782 = v1;
        let a1783 = 52u64;
        nat_add(a1782, a1783)?
    })?, f396(v0.clone(), {
        let a1784 = v1;
        let a1785 = 56u64;
        nat_add(a1784, a1785)?
    })?, f396(v0, {
        let a1786 = v1;
        let a1787 = 60u64;
        nat_add(a1786, a1787)?
    })?))
}

pub fn f341(v0: u64) -> u32 {
    let (_, h1788) = {
        let (_, h1789) = f397(Fn61::F398, 32u64, (v0, (1u32, 0u32)));
        h1789
    };
    h1788
}

pub fn f342(v0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    let m1791 = {
        let (h1790, _) = v0.clone();
        h1790
    };
    match m1791.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f399(v1, {
                let (_, h1792) = v0;
                h1792
            }, {
                let m1793 = v2;
                match m1793.uncons() {
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

pub fn f343(v0: u32) -> Bytes {
    let a1796 = {
        let a1794 = v0;
        let a1795 = 8u32;
        fixed_u32::bit_and(a1794, a1795)
    };
    let a1797 = 0u32;
    if equal(a1796, a1797) {
        let a1800 = {
            let a1798 = v0;
            let a1799 = 4u32;
            fixed_u32::bit_and(a1798, a1799)
        };
        let a1801 = 0u32;
        if equal(a1800, a1801) {
            let a1804 = {
                let a1802 = v0;
                let a1803 = 2u32;
                fixed_u32::bit_and(a1802, a1803)
            };
            let a1805 = 0u32;
            if equal(a1804, a1805) {
                let a1808 = {
                    let a1806 = v0;
                    let a1807 = 1u32;
                    fixed_u32::bit_and(a1806, a1807)
                };
                let a1809 = 0u32;
                if equal(a1808, a1809) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1812 = {
                    let a1810 = v0;
                    let a1811 = 1u32;
                    fixed_u32::bit_and(a1810, a1811)
                };
                let a1813 = 0u32;
                if equal(a1812, a1813) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1816 = {
                let a1814 = v0;
                let a1815 = 2u32;
                fixed_u32::bit_and(a1814, a1815)
            };
            let a1817 = 0u32;
            if equal(a1816, a1817) {
                let a1820 = {
                    let a1818 = v0;
                    let a1819 = 1u32;
                    fixed_u32::bit_and(a1818, a1819)
                };
                let a1821 = 0u32;
                if equal(a1820, a1821) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1824 = {
                    let a1822 = v0;
                    let a1823 = 1u32;
                    fixed_u32::bit_and(a1822, a1823)
                };
                let a1825 = 0u32;
                if equal(a1824, a1825) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1828 = {
            let a1826 = v0;
            let a1827 = 4u32;
            fixed_u32::bit_and(a1826, a1827)
        };
        let a1829 = 0u32;
        if equal(a1828, a1829) {
            let a1832 = {
                let a1830 = v0;
                let a1831 = 2u32;
                fixed_u32::bit_and(a1830, a1831)
            };
            let a1833 = 0u32;
            if equal(a1832, a1833) {
                let a1836 = {
                    let a1834 = v0;
                    let a1835 = 1u32;
                    fixed_u32::bit_and(a1834, a1835)
                };
                let a1837 = 0u32;
                if equal(a1836, a1837) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1840 = {
                    let a1838 = v0;
                    let a1839 = 1u32;
                    fixed_u32::bit_and(a1838, a1839)
                };
                let a1841 = 0u32;
                if equal(a1840, a1841) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1844 = {
                let a1842 = v0;
                let a1843 = 2u32;
                fixed_u32::bit_and(a1842, a1843)
            };
            let a1845 = 0u32;
            if equal(a1844, a1845) {
                let a1848 = {
                    let a1846 = v0;
                    let a1847 = 1u32;
                    fixed_u32::bit_and(a1846, a1847)
                };
                let a1849 = 0u32;
                if equal(a1848, a1849) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1852 = {
                    let a1850 = v0;
                    let a1851 = 1u32;
                    fixed_u32::bit_and(a1850, a1851)
                };
                let a1853 = 0u32;
                if equal(a1852, a1853) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f344(v0: u32, v1: u32) -> u32 {
    let m1856 = {
        let a1854 = v0;
        let a1855 = v1;
        fixed_u32::shift_right(a1854, a1855)
    };
    match m1856 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f345(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1857 = v0.clone();
        length_bytes(a1857)
    };
    let a1858 = v1;
    let a1859 = 0u64;
    if nat_eq(a1858, a1859) {
        Ok(false)
    } else {
        if f320(f264(v0.clone(), 0u64)?) {
            let a1861 = {
                let (h1860, _) = f240(Fn39::F400(v1, v0), v1, 1u64)?;
                h1860
            };
            let a1862 = v1;
            Ok(nat_eq(a1861, a1862))
        } else {
            Ok(false)
        }
    }
}

pub fn f346(v0: Bytes) -> R<bool> {
    let a1864 = {
        let a1863 = v0.clone();
        length_bytes(a1863)
    };
    let a1865 = 0u64;
    if nat_eq(a1864, a1865) {
        Ok(true)
    } else {
        if f159(v0.clone(), f3(Str::lit(" "))) {
            Ok(true)
        } else {
            if f154(v0.clone(), f3(Str::lit(" "))) {
                Ok(true)
            } else {
                let m1866 = f161(v0.clone(), f5(f3(Str::lit(" ")), f3(Str::lit(" "))))?;
                if match m1866 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                } {
                    Ok(true)
                } else {
                    let m1867 = f161(v0, Bytes::lit(&[0x0a]))?;
                    match m1867 {
                        None => {
                            Ok(false)
                        }
                        Some(_) => {
                            Ok(true)
                        }
                    }
                }
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: Bytes, v2: u64) -> R<Result<List<Adt5>, Adt48>> {
    let v3: u64 = {
        let a1868 = v1.clone();
        length_bytes(a1868)
    };
    let v4: Adt50 = {
        let (h1869, _) = f401(Fn62::F402(v2, v3, v0, v1), {
            let a1870 = v3;
            let a1871 = 1u64;
            nat_add(a1870, a1871)?
        }, Adt50::C0(std::rc::Rc::new(Adt51::C0(0u64, 1u64, 1u64)), List::<Adt5>::nil(), 0u64, None::<Adt48>))?;
        h1869
    };
    let m1873 = match v4.clone() {
        Adt50::C0(_, _, _, h1872) => {
            h1872
        }
    };
    match m1873 {
        None => {
            Ok(Ok::<List<Adt5>, Adt48>(f403(match v4 {
                Adt50::C0(_, h1874, _, _) => {
                    h1874
                }
            })))
        }
        Some(v5) => {
            Ok(Err::<List<Adt5>, Adt48>(v5))
        }
    }
}

pub fn f348(v0: Fn54, p1878: List<(Bytes, Bytes)>, p1879: List<Adt5>) -> R<List<(Bytes, Bytes)>> {
    let m1877 = (p1878, p1879);
    {
        let mut m1877 = m1877;
        loop {
            let step = {
                let (v1, v2) = m1877;
                let m1875 = v2;
                match m1875.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<Adt5>), R<List<(Bytes, Bytes)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<Adt5>), R<List<(Bytes, Bytes)>>>(({
                            let c1876: Fn54 = v0.clone();
                            c1876.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: List<Bytes>, v1: Bytes, v2: List<(Bytes, Bytes)>, v3: Adt5) -> R<List<(Bytes, Bytes)>> {
    if f404(v3.clone()) {
        if f214(v0, match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, _, h1880) => {
                h1880
            }
        })? {
            Ok(List::cons(f2(f5(f5(f5(f3(Str::lit("form `")), v1), f3(Str::lit("` declares the always-forbidden control `"))), f5(match v3 {
                Adt5::C0(_, _, _, _, _, _, _, h1881) => {
                    h1881
                }
            }, f3(Str::lit("` (\u{a7}12.4)"))))), v2))
        } else {
            Ok(v2)
        }
    } else {
        Ok(v2)
    }
}

pub fn f350(v0: Fn55, p1885: bool, p1886: List<Adt5>) -> bool {
    let m1884 = (p1885, p1886);
    {
        let mut m1884 = m1884;
        loop {
            let step = {
                let (v1, v2) = m1884;
                let m1882 = v2;
                match m1882.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1883: Fn55 = v0.clone();
                            c1883.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: bool, v1: Adt5) -> bool {
    if v0 {
        true
    } else {
        f404(v1)
    }
}

pub fn f352(v0: Fn56, p1890: (u64, Option<Bytes>), p1891: List<Adt5>) -> R<(u64, Option<Bytes>)> {
    let m1889 = (p1890, p1891);
    {
        let mut m1889 = m1889;
        loop {
            let step = {
                let (v1, v2) = m1889;
                let m1887 = v2;
                match m1887.uncons() {
                    None => {
                        Err::<((u64, Option<Bytes>), List<Adt5>), R<(u64, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<Bytes>), List<Adt5>), R<(u64, Option<Bytes>)>>(({
                            let c1888: Fn56 = v0.clone();
                            c1888.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: u64, v1: (u64, Option<Bytes>), v2: Adt5) -> R<(u64, Option<Bytes>)> {
    let m1893 = {
        let (_, h1892) = v1.clone();
        h1892
    };
    match m1893 {
        None => {
            if f405(v2.clone(), {
                let (h1894, _) = v1.clone();
                h1894
            }, v0)? {
                Ok(({
                    let a1896 = {
                        let (h1895, _) = v1;
                        h1895
                    };
                    let a1897 = 1u64;
                    nat_add(a1896, a1897)?
                }, None::<Bytes>))
            } else {
                Ok(({
                    let a1899 = {
                        let (h1898, _) = v1;
                        h1898
                    };
                    let a1900 = 1u64;
                    nat_add(a1899, a1900)?
                }, Some(f406(v2, v0))))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f354(v0: Bytes, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<List<(Bytes, Bytes)>> {
    if f171(f108(), v2.clone())? {
        Ok(v1)
    } else {
        Ok(List::cons(f2(f5(f5(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: `"))), v2), f3(Str::lit("` is not a feature")))), v1))
    }
}

pub fn f355(v0: u64, v1: Adt43) -> R<Option<Adt43>> {
    let m1902 = match v1.clone() {
        Adt43::C0(_, _, h1901) => {
            h1901
        }
    };
    match m1902 {
        None => {
            let m1904 = match v1.clone() {
                Adt43::C0(h1903, _, _) => {
                    h1903
                }
            };
            match m1904.uncons() {
                None => {
                    Ok(None::<Adt43>)
                }
                Some((v3, v4)) => {
                    let m1905 = v3;
                    match m1905 {
                        Adt44::C0(r1906) => {
                            let v5 = (*r1906).clone();
                            Ok(Some(f407(v1, v4, v5, v0)?))
                        }
                        Adt44::C1(v6, v7) => {
                            Ok(Some(f408(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt43>)
        }
    }
}

pub fn f356(v0: List<Adt7>, v1: Adt31) -> Option<Adt40> {
    Some(Adt40::C0(v0, std::rc::Rc::new(v1)))
}

pub fn f357(v0: Fn57, p1910: List<Adt7>, p1911: List<Adt7>) -> List<Adt7> {
    let m1909 = (p1910, p1911);
    {
        let mut m1909 = m1909;
        loop {
            let step = {
                let (v1, v2) = m1909;
                let m1907 = v2;
                match m1907.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c1908: Fn57 = v0.clone();
                            c1908.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f359(v0: List<Adt7>) -> List<Adt7> {
    f357(Fn57::F409, List::<Adt7>::nil(), v0)
}

pub fn f360(v0: Adt7, v1: u64) -> R<List<Adt7>> {
    Ok(f359({
        let (_, h1912) = {
            let (h1913, _) = f410(Fn63::F411, {
                let a1914 = v1;
                let a1915 = 1u64;
                nat_add(a1914, a1915)?
            }, (v0, List::<Adt7>::nil()));
            h1913
        };
        h1912
    }))
}

pub fn f361(v0: Adt31, v1: Bytes, v2: Adt7, v3: Adt7, v4: u64) -> R<Adt31> {
    Ok(Adt31::C0(match v0.clone() {
        Adt31::C0(h1916, _, _, _, _, _, _) => {
            h1916
        }
    }, match v0.clone() {
        Adt31::C0(_, h1917, _, _, _, _, _) => {
            h1917
        }
    }, match v0.clone() {
        Adt31::C0(_, _, h1918, _, _, _, _) => {
            h1918
        }
    }, match v0.clone() {
        Adt31::C0(_, _, _, h1919, _, _, _) => {
            h1919
        }
    }, match v0.clone() {
        Adt31::C0(_, _, _, _, h1920, _, _) => {
            h1920
        }
    }, f362(match v0.clone() {
        Adt31::C0(_, _, _, _, _, h1921, _) => {
            h1921
        }
    }, v1.clone()), f412(match v0 {
        Adt31::C0(_, _, _, _, _, _, h1922) => {
            h1922
        }
    }, f413(v1, v2, v3, v4)?)))
}

pub fn f362(v0: Option<Bytes>, v1: Bytes) -> Option<Bytes> {
    let m1923 = v0.clone();
    match m1923 {
        None => {
            Some(v1)
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f363(v0: Fn58, p1927: (bool, List<u64>), p1928: List<u64>) -> (bool, List<u64>) {
    let m1926 = (p1927, p1928);
    {
        let mut m1926 = m1926;
        loop {
            let step = {
                let (v1, v2) = m1926;
                let m1924 = v2;
                match m1924.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(({
                            let c1925: Fn58 = v0.clone();
                            c1925.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: u64, v1: (bool, List<u64>), v2: u64) -> (bool, List<u64>) {
    let (h1929, _) = v1;
    if h1929 {
        (true, List::cons(v2, {
            let (_, h1930) = v1;
            h1930
        }))
    } else {
        let a1931 = v0;
        let a1932 = v2;
        if nat_le(a1931, a1932) {
            (true, List::cons(v2, List::cons(v0, {
                let (_, h1933) = v1;
                h1933
            })))
        } else {
            (false, List::cons(v2, {
                let (_, h1934) = v1;
                h1934
            }))
        }
    }
}

pub fn f365(v0: List<Adt4>, v1: Bytes) -> Option<Adt4> {
    f414(Fn64::F415(v1), None::<Adt4>, v0)
}

pub fn f366(v0: u64) -> bool {
    if f263(v0) {
        true
    } else {
        f321(v0)
    }
}

pub fn f367(v0: (List<(Bytes, Bytes)>, List<Bytes>), v1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    let v2: bool = f214({
        let (_, h1935) = v0.clone();
        h1935
    }, v1.clone())?;
    let v3: bool = if f193(v1.clone())? {
        v2
    } else {
        true
    };
    Ok((f46({
        let (h1936, _) = v0;
        h1936
    }, if v3 {
        Some(f2(f5(f5(f3(Str::lit("invalid or duplicate eliminator binder ")), f6(v1.clone())), Bytes::lit(&[]))))
    } else {
        None::<(Bytes, Bytes)>
    }), List::cons(v1, {
        let (_, h1937) = v0;
        h1937
    })))
}

pub fn f368(v0: Bytes, v1: u64) -> bool {
    let m1938 = f247(v0, v1);
    match m1938 {
        None => {
            false
        }
        Some(v2) => {
            f416(v2)
        }
    }
}

pub fn f369(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1939 = v0.clone();
        length_bytes(a1939)
    };
    let v3: u64 = {
        let a1940 = v2;
        let a1941 = v1;
        nat_sub(a1940, a1941)
    };
    let a1942 = v1;
    let a1943 = f235(f83(v0, v1, v3), v3)?;
    nat_add(a1942, a1943)
}

pub fn f370(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h1944, _) = v2.clone();
        h1944
    };
    let v4: u64 = {
        let (h1945, _) = {
            let (_, h1946) = v2.clone();
            h1946
        };
        h1945
    };
    let v5: List<Bytes> = {
        let (_, h1947) = {
            let (_, h1948) = v2;
            h1948
        };
        h1947
    };
    let m1949 = f247(v0.clone(), v3);
    match m1949 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a1950 = v6;
            let a1951 = v1;
            if equal(a1950, a1951) {
                Ok(Some(({
                    let a1952 = v3;
                    let a1953 = 1u64;
                    nat_add(a1952, a1953)?
                }, ({
                    let a1954 = v3;
                    let a1955 = 1u64;
                    nat_add(a1954, a1955)?
                }, List::cons(f83(v0, v4, {
                    let a1956 = v3;
                    let a1957 = v4;
                    nat_sub(a1956, a1957)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a1958 = v3;
                    let a1959 = 1u64;
                    nat_add(a1958, a1959)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: Adt45) -> R<Option<Adt45>> {
    if match v2.clone() {
        Adt45::C0(_, _, _, h1960) => {
            h1960
        }
    } {
        Ok(None::<Adt45>)
    } else {
        let v3: u64 = match v2.clone() {
            Adt45::C0(h1961, _, _, _) => {
                h1961
            }
        };
        let a1962 = v3;
        let a1963 = v1;
        if nat_lt(a1962, a1963) {
            let v4: u64 = match v2.clone() {
                Adt45::C0(_, _, h1964, _) => {
                    h1964
                }
            };
            let a1965 = v4;
            let a1966 = 1u64;
            if nat_eq(a1965, a1966) {
                let m1967 = f247(v0, v3);
                match m1967 {
                    None => {
                        Ok(None::<Adt45>)
                    }
                    Some(v5) => {
                        let a1968 = v5;
                        let a1969 = 34u8;
                        if if equal(a1968, a1969) {
                            true
                        } else {
                            let a1970 = v5;
                            let a1971 = 92u8;
                            if equal(a1970, a1971) {
                                true
                            } else {
                                let a1972 = v5;
                                let a1973 = 110u8;
                                if equal(a1972, a1973) {
                                    true
                                } else {
                                    let a1974 = v5;
                                    let a1975 = 116u8;
                                    if equal(a1974, a1975) {
                                        true
                                    } else {
                                        let a1976 = v5;
                                        let a1977 = 114u8;
                                        equal(a1976, a1977)
                                    }
                                }
                            }
                        } {
                            Ok(Some(Adt45::C0({
                                let a1978 = v3;
                                let a1979 = 1u64;
                                nat_add(a1978, a1979)?
                            }, f417(match v2 {
                                Adt45::C0(_, h1980, _, _) => {
                                    h1980
                                }
                            }, {
                                let a1981 = v5;
                                let a1982 = 34u8;
                                if equal(a1981, a1982) {
                                    Bytes::lit(&[0x22])
                                } else {
                                    let a1983 = v5;
                                    let a1984 = 92u8;
                                    if equal(a1983, a1984) {
                                        Bytes::lit(&[0x5c])
                                    } else {
                                        let a1985 = v5;
                                        let a1986 = 110u8;
                                        if equal(a1985, a1986) {
                                            Bytes::lit(&[0x0a])
                                        } else {
                                            let a1987 = v5;
                                            let a1988 = 116u8;
                                            if equal(a1987, a1988) {
                                                Bytes::lit(&[0x09])
                                            } else {
                                                Bytes::lit(&[0x0d])
                                            }
                                        }
                                    }
                                }
                            }), 0u64, false)))
                        } else {
                            Ok(None::<Adt45>)
                        }
                    }
                }
            } else {
                let m1989 = f247(v0.clone(), v3);
                match m1989 {
                    None => {
                        Ok(None::<Adt45>)
                    }
                    Some(v6) => {
                        let a1990 = v6;
                        let a1991 = 34u8;
                        if equal(a1990, a1991) {
                            Ok(Some(Adt45::C0({
                                let a1992 = v3;
                                let a1993 = 1u64;
                                nat_add(a1992, a1993)?
                            }, match v2 {
                                Adt45::C0(_, h1994, _, _) => {
                                    h1994
                                }
                            }, 0u64, true)))
                        } else {
                            let a1995 = v6;
                            let a1996 = 92u8;
                            if equal(a1995, a1996) {
                                Ok(Some(Adt45::C0({
                                    let a1997 = v3;
                                    let a1998 = 1u64;
                                    nat_add(a1997, a1998)?
                                }, match v2 {
                                    Adt45::C0(_, h1999, _, _) => {
                                        h1999
                                    }
                                }, 1u64, false)))
                            } else {
                                Ok(Some(Adt45::C0({
                                    let a2000 = v3;
                                    let a2001 = 1u64;
                                    nat_add(a2000, a2001)?
                                }, f417(match v2 {
                                    Adt45::C0(_, h2002, _, _) => {
                                        h2002
                                    }
                                }, f83(v0, v3, 1u64)), 0u64, false)))
                            }
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt45>)
        }
    }
}

pub fn f372(v0: Bytes) -> R<u64> {
    let (h2003, _) = f240(Fn39::F418(v0.clone()), {
        let a2004 = v0;
        length_bytes(a2004)
    }, 0u64)?;
    Ok(h2003)
}

pub fn f373(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2008 = {
        let a2006 = v0;
        let a2007 = {
            let (h2005, _) = v1;
            h2005
        };
        index_bytes(a2006, a2007)
    };
    match m2008 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2010 = {
                    let (h2009, _) = v1;
                    h2009
                };
                let a2011 = 1u64;
                nat_add(a2010, a2011)?
            }, {
                let a2015 = {
                    let a2013 = {
                        let (_, h2012) = v1;
                        h2012
                    };
                    let a2014 = 10u64;
                    nat_mul(a2013, a2014)?
                };
                let a2016 = f419(v2)?;
                nat_add(a2015, a2016)?
            })))
        }
    }
}

pub fn f374(v0: List<List<(Bytes, Adt17)>>) -> List<List<(Bytes, Adt17)>> {
    f420(Fn65::F421, List::<List<(Bytes, Adt17)>>::nil(), v0)
}

pub fn f375(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m2017 = v1;
    match m2017 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a2020 = {
                    let a2018 = v3;
                    let a2019 = v0;
                    append_bytes(a2018, a2019)
                };
                let a2021 = v2;
                append_bytes(a2020, a2021)
            })
        }
    }
}

pub fn f376(v0: Adt36, v1: Adt35, v2: List<List<Adt35>>) -> R<Adt36> {
    let m2022 = v2;
    match m2022.uncons() {
        None => {
            Ok(Adt36::C0({
                let a2024 = match v0 {
                    Adt36::C0(h2023, _, _, _, _) => {
                        h2023
                    }
                };
                let a2025 = 1u64;
                nat_add(a2024, a2025)?
            }, List::<List<Adt35>>::nil(), Some(v1), None::<Adt28>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt36::C0({
                let a2027 = match v0.clone() {
                    Adt36::C0(h2026, _, _, _, _) => {
                        h2026
                    }
                };
                let a2028 = 1u64;
                nat_add(a2027, a2028)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt36::C0(_, _, h2029, _, _) => {
                    h2029
                }
            }, None::<Adt28>, false))
        }
    }
}

pub fn f377(v0: List<Adt35>) -> Adt35 {
    f422(Fn66::F423, Adt35::C2, v0)
}

pub fn f378(v0: Adt35, v1: u64) -> R<List<Adt35>> {
    Ok(f424({
        let (_, h2030) = {
            let (h2031, _) = f425(Fn67::F426, {
                let a2032 = v1;
                let a2033 = 1u64;
                nat_add(a2032, a2033)?
            }, (v0, List::<Adt35>::nil()));
            h2031
        };
        h2030
    }))
}

pub fn f379(v0: Adt37, v1: Adt28) -> Adt37 {
    Adt37::C0(match v0.clone() {
        Adt37::C0(h2034, _, _) => {
            h2034
        }
    }, match v0 {
        Adt37::C0(_, h2035, _) => {
            h2035
        }
    }, Some(v1))
}

pub fn f380(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: Bytes, v4: List<Adt35>, v5: u64) -> R<Adt37> {
    if f427(v3.clone()) {
        let m2036 = f428(v3, v4, v5)?;
        match m2036 {
            Ok(v6) => {
                Ok(Adt37::C0(v1, List::cons(v6, match v0 {
                    Adt37::C0(_, h2037, _) => {
                        h2037
                    }
                }), None::<Adt28>))
            }
            Err(v7) => {
                Ok(f379(v0, v7))
            }
        }
    } else {
        let a2038 = v3.clone();
        let a2039 = f201(Str::lit("app"));
        if equal(a2038, a2039) {
            let a2041 = {
                let a2040 = v4.clone();
                length_list(a2040)
            };
            let a2042 = 2u64;
            if nat_lt(a2041, a2042) {
                Ok(f379(v0, f200(f201(Str::lit("`app` has at least a function and one argument")))))
            } else {
                Ok(Adt37::C0(f429(v4, List::cons(Adt38::C1(std::rc::Rc::new(v2)), v1)), match v0 {
                    Adt37::C0(_, h2043, _) => {
                        h2043
                    }
                }, None::<Adt28>))
            }
        } else {
            if f384(v3.clone()) {
                f430(v0, v1, v2, v3, v4, v5)
            } else {
                let a2044 = v3.clone();
                let a2045 = f201(Str::lit("let"));
                if equal(a2044, a2045) {
                    f431(v0, v1, v2, v4, v5)
                } else {
                    Ok(f379(v0, f200(f205(f301(v3), f201(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f381(v0: Adt35) -> Bytes {
    let m2046 = v0;
    match m2046 {
        Adt35::C0(v1) => {
            f301(v1)
        }
        Adt35::C1(r2050) => {
            let v2 = (*r2050).clone();
            let m2047 = v2;
            match m2047 {
                Adt35::C0(_) => {
                    f201(Str::lit("a list"))
                }
                Adt35::C1(_) => {
                    f201(Str::lit("a list"))
                }
                Adt35::C2 => {
                    f201(Str::lit("a list"))
                }
                Adt35::C3(r2049, _) => {
                    let v3 = (*r2049).clone();
                    let m2048 = v3;
                    match m2048 {
                        Adt35::C0(v5) => {
                            f205(f205(f201(Str::lit("`(")), v5), f201(Str::lit(" ...)`")))
                        }
                        Adt35::C1(_) => {
                            f201(Str::lit("a list"))
                        }
                        Adt35::C2 => {
                            f201(Str::lit("a list"))
                        }
                        Adt35::C3(_, _) => {
                            f201(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            f201(Str::lit("a list"))
        }
        Adt35::C3(_, _) => {
            f201(Str::lit("a list"))
        }
    }
}

pub fn f382(v0: List<Adt1>, v1: u64) -> (List<Adt1>, List<Adt1>) {
    let (h2051, _) = f432(Fn68::F433, v1, (List::<Adt1>::nil(), v0));
    h2051
}

pub fn f383(v0: List<Adt1>) -> Adt1 {
    f434(Fn69::F435, Adt1::C9, f386(v0))
}

pub fn f384(v0: Bytes) -> bool {
    let a2052 = v0.clone();
    let a2053 = f201(Str::lit("pi"));
    if equal(a2052, a2053) {
        true
    } else {
        let a2054 = v0;
        let a2055 = f201(Str::lit("lam"));
        equal(a2054, a2055)
    }
}

pub fn f385(v0: List<Adt35>, v1: u64) -> Adt35 {
    let m2058 = {
        let a2056 = v0;
        let a2057 = v1;
        index_list(a2056, a2057)
    };
    match m2058 {
        None => {
            Adt35::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f386(v0: List<Adt1>) -> List<Adt1> {
    f436(Fn70::F437, List::<Adt1>::nil(), v0)
}

pub fn f387(v0: List<Adt35>, v1: List<Adt1>, v2: u64) -> R<Adt1> {
    let (_, h2059) = f438(Fn71::F439(v2), (f386(v1), Adt1::C9), f424(v0))?;
    Ok(h2059)
}

pub fn f388(v0: Adt35, v1: Bytes, _: u64) -> Result<Bytes, Adt28> {
    let m2060 = v0.clone();
    match m2060 {
        Adt35::C0(v3) => {
            Ok::<Bytes, Adt28>(v3)
        }
        Adt35::C1(_) => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
        Adt35::C2 => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
        Adt35::C3(_, _) => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
    }
}

pub fn f389(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a2061 = v2;
        let a2062 = v0;
        equal(a2061, a2062)
    }
}

pub fn f390(v0: (u64, List<Adt39>), v1: Adt39) -> R<(u64, List<Adt39>)> {
    let v2: Adt52 = f440(Fn72::F441, 40u64, Adt52::C0(std::rc::Rc::new(v1), {
        let a2064 = {
            let (h2063, _) = v0.clone();
            h2063
        };
        let a2065 = 1u64;
        nat_add(a2064, a2065)?
    }, {
        let (_, h2066) = v0;
        h2066
    }));
    Ok(({
        let a2068 = {
            let (h2067, _) = v0;
            h2067
        };
        let a2069 = 1u64;
        nat_add(a2068, a2069)?
    }, List::cons(match v2.clone() {
        Adt52::C0(h2070, _, _) => {
            (*h2070).clone()
        }
    }, match v2 {
        Adt52::C0(_, _, h2071) => {
            h2071
        }
    })))
}

pub fn f391(v0: Fn59, p2075: u64, p2076: (Adt47, Adt46)) -> (Adt47, Adt46) {
    let m2074 = (p2075, p2076);
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let (v1, v2) = m2074;
                let m2072 = v1;
                if m2072 == 0 {
                    Err::<(u64, (Adt47, Adt46)), (Adt47, Adt46)>(v2)
                } else {
                    let v3 = m2072 - 1;
                    Ok::<(u64, (Adt47, Adt46)), (Adt47, Adt46)>((v3, {
                        let c2073: Fn59 = v0.clone();
                        c2073.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: (Adt47, Adt46)) -> (Adt47, Adt46) {
    f442(v0)
}

pub fn f393(v0: Fn60, p2080: u64, p2081: (u64, Adt39)) -> R<(u64, Adt39)> {
    let m2079 = (p2080, p2081);
    {
        let mut m2079 = m2079;
        loop {
            let step = {
                let (v1, v2) = m2079;
                let m2077 = v1;
                if m2077 == 0 {
                    Err::<(u64, (u64, Adt39)), R<(u64, Adt39)>>(Ok(v2))
                } else {
                    let v3 = m2077 - 1;
                    Ok::<(u64, (u64, Adt39)), R<(u64, Adt39)>>((v3, {
                        let c2078: Fn60 = v0.clone();
                        c2078.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt39)) -> R<(u64, Adt39)> {
    Ok(({
        let a2083 = {
            let (h2082, _) = v3.clone();
            h2082
        };
        let a2084 = 1u64;
        nat_add(a2083, a2084)?
    }, f338({
        let (_, h2085) = v3.clone();
        h2085
    }, f340(v1, {
        let a2089 = v2;
        let a2090 = {
            let a2087 = 64u64;
            let a2088 = {
                let (h2086, _) = v3.clone();
                h2086
            };
            nat_mul(a2087, a2088)?
        };
        nat_add(a2089, a2090)?
    })?, v0, 64u32, {
        let a2092 = {
            let (h2091, _) = v3;
            h2091
        };
        let a2093 = 0u64;
        if nat_eq(a2092, a2093) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f395() -> Adt39 {
    Adt39::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f396(v0: Bytes, v1: u64) -> R<u32> {
    let a2104 = f443(v0.clone(), v1);
    let a2105 = {
        let a2102 = f444(f443(v0.clone(), {
            let a2094 = v1;
            let a2095 = 1u64;
            nat_add(a2094, a2095)?
        }), 8u32);
        let a2103 = {
            let a2100 = f444(f443(v0.clone(), {
                let a2096 = v1;
                let a2097 = 2u64;
                nat_add(a2096, a2097)?
            }), 16u32);
            let a2101 = f444(f443(v0, {
                let a2098 = v1;
                let a2099 = 3u64;
                nat_add(a2098, a2099)?
            }), 24u32);
            fixed_u32::bit_or(a2100, a2101)
        };
        fixed_u32::bit_or(a2102, a2103)
    };
    Ok(fixed_u32::bit_or(a2104, a2105))
}

pub fn f397(v0: Fn61, p2109: u64, p2110: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2108 = (p2109, p2110);
    {
        let mut m2108 = m2108;
        loop {
            let step = {
                let (v1, v2) = m2108;
                let m2106 = v1;
                if m2106 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2106 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2107: Fn61 = v0.clone();
                        c2107.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2111, _) = v0;
        h2111
    };
    let v2: u32 = {
        let (h2112, _) = {
            let (_, h2113) = v0;
            h2113
        };
        h2112
    };
    let v3: u32 = {
        let (_, h2114) = {
            let (_, h2115) = v0;
            h2115
        };
        h2114
    };
    ({
        let a2116 = v1;
        let a2117 = 2u64;
        let a2118 = 0u64;
        nat_quot(a2116, a2117, a2118)
    }, (f444(v2, 1u32), {
        let a2122 = {
            let a2119 = v1;
            let a2120 = 2u64;
            let a2121 = 0u64;
            nat_rem(a2119, a2120, a2121)
        };
        let a2123 = 1u64;
        if nat_eq(a2122, a2123) {
            let a2124 = v3;
            let a2125 = v2;
            fixed_u32::bit_or(a2124, a2125)
        } else {
            v3
        }
    }))
}

pub fn f399(v0: Adt39, v1: Adt39, v2: bool) -> Adt39 {
    f338(f395(), f445(v0, v1), 0u64, 64u32, {
        let a2126 = 4u32;
        let a2127 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a2126, a2127)
    })
}

pub fn f400(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f446(v1, v0, v2)
}

pub fn f401(v0: Fn62, p2132: u64, p2133: Adt50) -> R<(Adt50, bool)> {
    let m2131 = (p2132, p2133);
    {
        let mut m2131 = m2131;
        loop {
            let step = {
                let (v1, v2) = m2131;
                let m2128 = v1;
                if m2128 == 0 {
                    Err::<(u64, Adt50), R<(Adt50, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2128 - 1;
                    let m2130 = {
                        let c2129: Fn62 = v0.clone();
                        c2129.apply(v2.clone())?
                    };
                    match m2130 {
                        None => {
                            Err::<(u64, Adt50), R<(Adt50, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt50), R<(Adt50, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: u64, v1: u64, v2: Bytes, v3: Bytes, v4: Adt50) -> R<Option<Adt50>> {
    f447(v2, v3, v1, v0, v4)
}

pub fn f403(v0: List<Adt5>) -> List<Adt5> {
    f448(Fn73::F449, List::<Adt5>::nil(), v0)
}

pub fn f404(v0: Adt5) -> bool {
    let m2135 = match v0 {
        Adt5::C0(h2134, _, _, _, _, _, _, _) => {
            h2134
        }
    };
    match m2135 {
        Adt6::C0 => {
            true
        }
        Adt6::C1 => {
            false
        }
        Adt6::C2 => {
            false
        }
        Adt6::C3 => {
            false
        }
        Adt6::C4 => {
            false
        }
        Adt6::C5 => {
            false
        }
        Adt6::C6 => {
            false
        }
    }
}

pub fn f405(v0: Adt5, v1: u64, v2: u64) -> R<bool> {
    let a2136 = v2;
    let a2137 = 0u64;
    if nat_eq(a2136, a2137) {
        f450(v0)
    } else {
        let a2138 = v2;
        let a2139 = 1u64;
        if nat_eq(a2138, a2139) {
            f451(v0, v1)
        } else {
            if f450(v0.clone())? {
                f451(v0, v1)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f406(v0: Adt5, v1: u64) -> Bytes {
    f452(f452(f452(f452(f452(f452(f184(Str::lit("`")), match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2140) => {
            h2140
        }
    }), f184(Str::lit("` ("))), f453(v0)), f184(Str::lit(") is not renderer-safe in the "))), f454(v1)), f184(Str::lit(" channel")))
}

pub fn f407(v0: Adt43, v1: List<Adt44>, v2: Adt35, v3: u64) -> R<Adt43> {
    let m2141 = v2.clone();
    match m2141 {
        Adt35::C0(_) => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C1(r2144) => {
            let v4 = (*r2144).clone();
            let m2142 = f378(v4, v3)?;
            match m2142.uncons() {
                None => {
                    Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                }
                Some((v5, v6)) => {
                    let m2143 = v5;
                    match m2143 {
                        Adt35::C0(v7) => {
                            f456(v0, v1, v7, v6, v3)
                        }
                        Adt35::C1(_) => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                        Adt35::C2 => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C3(_, _) => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
    }
}

pub fn f408(v0: Adt43, v1: List<Adt44>, v2: Bytes, v3: u64) -> Adt43 {
    let v4: (List<Adt7>, List<Adt7>) = f457(match v0.clone() {
        Adt43::C0(_, h2145, _) => {
            h2145
        }
    }, v3);
    let v5: List<Adt7> = {
        let (h2146, _) = v4;
        h2146
    };
    let a2147 = v2.clone();
    let a2148 = f201(Str::lit("seq"));
    if equal(a2147, a2148) {
        f458(Adt43::C0(v1.clone(), {
            let (_, h2149) = v4;
            h2149
        }, None::<Adt28>), v1, Adt7::C3(std::rc::Rc::new(f459(v5))))
    } else {
        if f460(v2.clone()) {
            let m2150 = v5;
            match m2150.uncons() {
                None => {
                    f455(v0, f200(f201(Str::lit("missing operand"))))
                }
                Some((v6, _)) => {
                    f458(Adt43::C0(v1.clone(), {
                        let (_, h2151) = v4;
                        h2151
                    }, None::<Adt28>), v1, f461(v2, v6))
                }
            }
        } else {
            let m2152 = v5;
            match m2152.uncons() {
                None => {
                    f455(v0, f200(f201(Str::lit("missing operand"))))
                }
                Some((v8, v9)) => {
                    let m2153 = v9;
                    match m2153.uncons() {
                        None => {
                            f455(v0, f200(f201(Str::lit("missing operand"))))
                        }
                        Some((v10, _)) => {
                            f458(Adt43::C0(v1.clone(), {
                                let (_, h2154) = v4;
                                h2154
                            }, None::<Adt28>), v1, f462(v2, v8, v10))
                        }
                    }
                }
            }
        }
    }
}

pub fn f409(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f410(v0: Fn63, p2159: u64, p2160: (Adt7, List<Adt7>)) -> ((Adt7, List<Adt7>), bool) {
    let m2158 = (p2159, p2160);
    {
        let mut m2158 = m2158;
        loop {
            let step = {
                let (v1, v2) = m2158;
                let m2155 = v1;
                if m2155 == 0 {
                    Err::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v2, false))
                } else {
                    let v3 = m2155 - 1;
                    let m2157 = {
                        let c2156: Fn63 = v0.clone();
                        c2156.apply(v2.clone())
                    };
                    match m2157 {
                        None => {
                            Err::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: (Adt7, List<Adt7>)) -> Option<(Adt7, List<Adt7>)> {
    let m2162 = {
        let (h2161, _) = v0;
        h2161
    };
    match m2162 {
        Adt7::C0(_, _, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C1(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C2(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C3(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C4 => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C5(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C6(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C7(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C8(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C9(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C10(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C11(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C12(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C13 => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C14(r2164, r2165) => {
            let v1 = (*r2164).clone();
            let v2 = (*r2165).clone();
            Some((v2, List::cons(v1, {
                let (_, h2163) = v0;
                h2163
            })))
        }
    }
}

pub fn f412(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2166 = v0.clone();
    match m2166 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f413(v0: Bytes, v1: Adt7, v2: Adt7, v3: u64) -> R<Option<Bytes>> {
    let m2167 = f463(v0.clone(), v1, v3)?;
    match m2167 {
        None => {
            f463(v0, v2, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f414(v0: Fn64, p2171: Option<Adt4>, p2172: List<Adt4>) -> Option<Adt4> {
    let m2170 = (p2171, p2172);
    {
        let mut m2170 = m2170;
        loop {
            let step = {
                let (v1, v2) = m2170;
                let m2168 = v2;
                match m2168.uncons() {
                    None => {
                        Err::<(Option<Adt4>, List<Adt4>), Option<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt4>, List<Adt4>), Option<Adt4>>(({
                            let c2169: Fn64 = v0.clone();
                            c2169.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: Bytes, v1: Option<Adt4>, v2: Adt4) -> Option<Adt4> {
    let m2173 = v1.clone();
    match m2173 {
        None => {
            let a2175 = match v2.clone() {
                Adt4::C0(h2174, _, _, _, _, _) => {
                    h2174
                }
            };
            let a2176 = v0;
            if equal(a2175, a2176) {
                Some(v2)
            } else {
                None::<Adt4>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f416(v0: u8) -> bool {
    let a2177 = v0;
    let a2178 = 32u8;
    if if if equal(a2177, a2178) {
        true
    } else {
        let a2179 = v0;
        let a2180 = 9u8;
        equal(a2179, a2180)
    } {
        true
    } else {
        let a2181 = v0;
        let a2182 = 10u8;
        equal(a2181, a2182)
    } {
        true
    } else {
        let a2183 = v0;
        let a2184 = 12u8;
        if equal(a2183, a2184) {
            true
        } else {
            let a2185 = v0;
            let a2186 = 13u8;
            equal(a2185, a2186)
        }
    }
}

pub fn f417(v0: Bytes, v1: Bytes) -> Bytes {
    let a2187 = v0;
    let a2188 = v1;
    append_bytes(a2187, a2188)
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2191 = {
        let a2189 = v0;
        let a2190 = v1;
        index_bytes(a2189, a2190)
    };
    match m2191 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f464(v2)? {
                Ok(Some({
                    let a2192 = v1;
                    let a2193 = 1u64;
                    nat_add(a2192, a2193)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f419(v0: u8) -> R<u64> {
    let a2240 = {
        let v1: u8 = v0;
        let a2238 = {
            let a2214 = {
                let a2202 = {
                    let a2196 = {
                        let a2194 = v1;
                        let a2195 = 128u8;
                        fixed_u8::bit_and(a2194, a2195)
                    };
                    let a2197 = 128u8;
                    if equal(a2196, a2197) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2203 = {
                    let a2200 = {
                        let a2198 = v1;
                        let a2199 = 64u8;
                        fixed_u8::bit_and(a2198, a2199)
                    };
                    let a2201 = 64u8;
                    if equal(a2200, a2201) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2202, a2203)?
            };
            let a2215 = {
                let a2212 = {
                    let a2206 = {
                        let a2204 = v1;
                        let a2205 = 32u8;
                        fixed_u8::bit_and(a2204, a2205)
                    };
                    let a2207 = 32u8;
                    if equal(a2206, a2207) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2213 = {
                    let a2210 = {
                        let a2208 = v1;
                        let a2209 = 16u8;
                        fixed_u8::bit_and(a2208, a2209)
                    };
                    let a2211 = 16u8;
                    if equal(a2210, a2211) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2212, a2213)?
            };
            nat_add(a2214, a2215)?
        };
        let a2239 = {
            let a2236 = {
                let a2224 = {
                    let a2218 = {
                        let a2216 = v1;
                        let a2217 = 8u8;
                        fixed_u8::bit_and(a2216, a2217)
                    };
                    let a2219 = 8u8;
                    if equal(a2218, a2219) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2225 = {
                    let a2222 = {
                        let a2220 = v1;
                        let a2221 = 4u8;
                        fixed_u8::bit_and(a2220, a2221)
                    };
                    let a2223 = 4u8;
                    if equal(a2222, a2223) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2224, a2225)?
            };
            let a2237 = {
                let a2234 = {
                    let a2228 = {
                        let a2226 = v1;
                        let a2227 = 2u8;
                        fixed_u8::bit_and(a2226, a2227)
                    };
                    let a2229 = 2u8;
                    if equal(a2228, a2229) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2235 = {
                    let a2232 = {
                        let a2230 = v1;
                        let a2231 = 1u8;
                        fixed_u8::bit_and(a2230, a2231)
                    };
                    let a2233 = 1u8;
                    if equal(a2232, a2233) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2234, a2235)?
            };
            nat_add(a2236, a2237)?
        };
        nat_add(a2238, a2239)?
    };
    let a2241 = 48u64;
    Ok(nat_sub(a2240, a2241))
}

pub fn f420(v0: Fn65, p2245: List<List<(Bytes, Adt17)>>, p2246: List<List<(Bytes, Adt17)>>) -> List<List<(Bytes, Adt17)>> {
    let m2244 = (p2245, p2246);
    {
        let mut m2244 = m2244;
        loop {
            let step = {
                let (v1, v2) = m2244;
                let m2242 = v2;
                match m2242.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt17)>>, List<List<(Bytes, Adt17)>>), List<List<(Bytes, Adt17)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt17)>>, List<List<(Bytes, Adt17)>>), List<List<(Bytes, Adt17)>>>(({
                            let c2243: Fn65 = v0.clone();
                            c2243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: List<List<(Bytes, Adt17)>>, v1: List<(Bytes, Adt17)>) -> List<List<(Bytes, Adt17)>> {
    List::cons(v1, v0)
}

pub fn f422(v0: Fn66, p2250: Adt35, p2251: List<Adt35>) -> Adt35 {
    let m2249 = (p2250, p2251);
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let (v1, v2) = m2249;
                let m2247 = v2;
                match m2247.uncons() {
                    None => {
                        Err::<(Adt35, List<Adt35>), Adt35>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt35, List<Adt35>), Adt35>(({
                            let c2248: Fn66 = v0.clone();
                            c2248.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: Adt35, v1: Adt35) -> Adt35 {
    Adt35::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f424(v0: List<Adt35>) -> List<Adt35> {
    f465(Fn74::F466, List::<Adt35>::nil(), v0)
}

pub fn f425(v0: Fn67, p2256: u64, p2257: (Adt35, List<Adt35>)) -> ((Adt35, List<Adt35>), bool) {
    let m2255 = (p2256, p2257);
    {
        let mut m2255 = m2255;
        loop {
            let step = {
                let (v1, v2) = m2255;
                let m2252 = v1;
                if m2252 == 0 {
                    Err::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v2, false))
                } else {
                    let v3 = m2252 - 1;
                    let m2254 = {
                        let c2253: Fn67 = v0.clone();
                        c2253.apply(v2.clone())
                    };
                    match m2254 {
                        None => {
                            Err::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
    f467(v0)
}

pub fn f427(v0: Bytes) -> bool {
    let a2258 = v0.clone();
    let a2259 = f201(Str::lit("sort"));
    if equal(a2258, a2259) {
        true
    } else {
        let a2260 = v0.clone();
        let a2261 = f201(Str::lit("const"));
        if equal(a2260, a2261) {
            true
        } else {
            let a2262 = v0.clone();
            let a2263 = f201(Str::lit("local"));
            if equal(a2262, a2263) {
                true
            } else {
                let a2264 = v0;
                let a2265 = f201(Str::lit("nat"));
                equal(a2264, a2265)
            }
        }
    }
}

pub fn f428(v0: Bytes, v1: List<Adt35>, v2: u64) -> R<Result<Adt1, Adt28>> {
    let a2266 = v0.clone();
    let a2267 = f201(Str::lit("sort"));
    if equal(a2266, a2267) {
        f468(v1, v2)
    } else {
        let a2268 = v0.clone();
        let a2269 = f201(Str::lit("const"));
        if equal(a2268, a2269) {
            f469(v1, v2)
        } else {
            let a2270 = v0.clone();
            let a2271 = f201(Str::lit("local"));
            if equal(a2270, a2271) {
                f470(v1)
            } else {
                let a2272 = v0.clone();
                let a2273 = f201(Str::lit("nat"));
                if equal(a2272, a2273) {
                    f471(v1)
                } else {
                    Ok(Err::<Adt1, Adt28>(f200(f205(f301(v0), f201(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f429(v0: List<Adt35>, v1: List<Adt38>) -> List<Adt38> {
    f472(Fn75::F473, v1, f424(v0))
}

pub fn f430(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: Bytes, v4: List<Adt35>, v5: u64) -> R<Adt37> {
    let a2275 = {
        let a2274 = v4.clone();
        length_list(a2274)
    };
    let a2276 = 2u64;
    if nat_eq(a2275, a2276) {
        let m2277 = f385(v4.clone(), 0u64);
        match m2277 {
            Adt35::C0(_) => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
            Adt35::C1(r2281) => {
                let v6 = (*r2281).clone();
                let v7: List<Adt35> = f378(v6, v5)?;
                let m2278 = v7.clone();
                match m2278.uncons() {
                    None => {
                        Ok(f379(v0, f200(f205(f301(v3), f201(Str::lit(" has at least one binder"))))))
                    }
                    Some((_, _)) => {
                        let m2279 = f474(v7, List::cons(Adt38::C0(std::rc::Rc::new(f385(v4, 1u64))), List::cons(Adt38::C1(std::rc::Rc::new(v2)), v1)), v5)?;
                        match m2279 {
                            Ok(v10) => {
                                Ok(Adt37::C0(v10, match v0 {
                                    Adt37::C0(_, h2280, _) => {
                                        h2280
                                    }
                                }, None::<Adt28>))
                            }
                            Err(v11) => {
                                Ok(f379(v0, v11))
                            }
                        }
                    }
                }
            }
            Adt35::C2 => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
            Adt35::C3(_, _) => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
        }
    } else {
        Ok(f379(v0, f200(f205(f205(f205(f205(f201(Str::lit("`")), v3.clone()), f201(Str::lit("` is `("))), v3), f201(Str::lit(" (binders...) body)`"))))))
    }
}

pub fn f431(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: List<Adt35>, v4: u64) -> R<Adt37> {
    let a2283 = {
        let a2282 = v3.clone();
        length_list(a2282)
    };
    let a2284 = 4u64;
    if nat_eq(a2283, a2284) {
        let m2285 = f388(f385(v3.clone(), 0u64), f201(Str::lit("a local ID")), v4);
        match m2285 {
            Ok(v5) => {
                if f193(v5.clone())? {
                    Ok(Adt37::C0(List::cons(Adt38::C0(std::rc::Rc::new(f385(v3.clone(), 1u64))), List::cons(Adt38::C0(std::rc::Rc::new(f385(v3.clone(), 2u64))), List::cons(Adt38::C0(std::rc::Rc::new(f385(v3, 3u64))), List::cons(Adt38::C1(std::rc::Rc::new(v2)), v1)))), match v0 {
                        Adt37::C0(_, h2286, _) => {
                            h2286
                        }
                    }, None::<Adt28>))
                } else {
                    Ok(f379(v0, f200(f205(f301(v5), f201(Str::lit(" is not a valid local ID"))))))
                }
            }
            Err(v6) => {
                Ok(f379(v0, v6))
            }
        }
    } else {
        Ok(f379(v0, f200(f201(Str::lit("`let` is `(let name type value body)`")))))
    }
}

pub fn f432(v0: Fn68, p2291: u64, p2292: (List<Adt1>, List<Adt1>)) -> ((List<Adt1>, List<Adt1>), bool) {
    let m2290 = (p2291, p2292);
    {
        let mut m2290 = m2290;
        loop {
            let step = {
                let (v1, v2) = m2290;
                let m2287 = v1;
                if m2287 == 0 {
                    Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, false))
                } else {
                    let v3 = m2287 - 1;
                    let m2289 = {
                        let c2288: Fn68 = v0.clone();
                        c2288.apply(v2.clone())
                    };
                    match m2289 {
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
                Ok(next) => { m2290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
    let m2294 = {
        let (_, h2293) = v0;
        h2293
    };
    match m2294.uncons() {
        None => {
            None::<(List<Adt1>, List<Adt1>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h2295, _) = v0;
                h2295
            }), v2))
        }
    }
}

pub fn f434(v0: Fn69, p2299: Adt1, p2300: List<Adt1>) -> Adt1 {
    let m2298 = (p2299, p2300);
    {
        let mut m2298 = m2298;
        loop {
            let step = {
                let (v1, v2) = m2298;
                let m2296 = v2;
                match m2296.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c2297: Fn69 = v0.clone();
                            c2297.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f436(v0: Fn70, p2304: List<Adt1>, p2305: List<Adt1>) -> List<Adt1> {
    let m2303 = (p2304, p2305);
    {
        let mut m2303 = m2303;
        loop {
            let step = {
                let (v1, v2) = m2303;
                let m2301 = v2;
                match m2301.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c2302: Fn70 = v0.clone();
                            c2302.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f438(v0: Fn71, p2309: (List<Adt1>, Adt1), p2310: List<Adt35>) -> R<(List<Adt1>, Adt1)> {
    let m2308 = (p2309, p2310);
    {
        let mut m2308 = m2308;
        loop {
            let step = {
                let (v1, v2) = m2308;
                let m2306 = v2;
                match m2306.uncons() {
                    None => {
                        Err::<((List<Adt1>, Adt1), List<Adt35>), R<(List<Adt1>, Adt1)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt1>, Adt1), List<Adt35>), R<(List<Adt1>, Adt1)>>(({
                            let c2307: Fn71 = v0.clone();
                            c2307.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: u64, v1: (List<Adt1>, Adt1), v2: Adt35) -> R<(List<Adt1>, Adt1)> {
    let m2312 = {
        let (h2311, _) = v1.clone();
        h2311
    };
    match m2312.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            let m2313 = f475(v2, v0)?;
            match m2313 {
                Ok(v5) => {
                    Ok((v4, Adt1::C11(match v5.clone() {
                        Adt54::C0(h2314, _, _) => {
                            h2314
                        }
                    }, match v5 {
                        Adt54::C0(_, h2315, _) => {
                            h2315
                        }
                    }, std::rc::Rc::new(v3), std::rc::Rc::new({
                        let (_, h2316) = v1;
                        h2316
                    }))))
                }
                Err(_) => {
                    Ok((v4, {
                        let (_, h2317) = v1;
                        h2317
                    }))
                }
            }
        }
    }
}

pub fn f440(v0: Fn72, p2321: u64, p2322: Adt52) -> Adt52 {
    let m2320 = (p2321, p2322);
    {
        let mut m2320 = m2320;
        loop {
            let step = {
                let (v1, v2) = m2320;
                let m2318 = v1;
                if m2318 == 0 {
                    Err::<(u64, Adt52), Adt52>(v2)
                } else {
                    let v3 = m2318 - 1;
                    Ok::<(u64, Adt52), Adt52>((v3, {
                        let c2319: Fn72 = v0.clone();
                        c2319.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Adt52) -> Adt52 {
    f476(v0)
}

pub fn f442(v0: (Adt47, Adt46)) -> (Adt47, Adt46) {
    (f477(f478({
        let (h2323, _) = v0.clone();
        h2323
    }, {
        let (_, h2324) = v0.clone();
        h2324
    }), {
        let (_, h2325) = v0.clone();
        h2325
    }), f479({
        let (_, h2326) = v0;
        h2326
    }))
}

pub fn f443(v0: Bytes, v1: u64) -> u32 {
    let m2329 = {
        let a2327 = v0;
        let a2328 = v1;
        index_bytes(a2327, a2328)
    };
    match m2329 {
        None => {
            0u32
        }
        Some(v2) => {
            let m2331 = {
                let a2330 = v2;
                fixed_u32::convert(i128::from(a2330))
            };
            match m2331 {
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

pub fn f444(v0: u32, v1: u32) -> u32 {
    let m2334 = {
        let a2332 = v0;
        let a2333 = v1;
        fixed_u32::shift_left(a2332, a2333)
    };
    match m2334 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f445(v0: Adt39, v1: Adt39) -> Adt46 {
    Adt46::C0(match v0.clone() {
        Adt39::C0(h2335, _, _, _, _, _, _, _) => {
            h2335
        }
    }, match v0.clone() {
        Adt39::C0(_, h2336, _, _, _, _, _, _) => {
            h2336
        }
    }, match v0.clone() {
        Adt39::C0(_, _, h2337, _, _, _, _, _) => {
            h2337
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, h2338, _, _, _, _) => {
            h2338
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, h2339, _, _, _) => {
            h2339
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, _, h2340, _, _) => {
            h2340
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, _, _, h2341, _) => {
            h2341
        }
    }, match v0 {
        Adt39::C0(_, _, _, _, _, _, _, h2342) => {
            h2342
        }
    }, match v1.clone() {
        Adt39::C0(h2343, _, _, _, _, _, _, _) => {
            h2343
        }
    }, match v1.clone() {
        Adt39::C0(_, h2344, _, _, _, _, _, _) => {
            h2344
        }
    }, match v1.clone() {
        Adt39::C0(_, _, h2345, _, _, _, _, _) => {
            h2345
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, h2346, _, _, _, _) => {
            h2346
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, h2347, _, _, _) => {
            h2347
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, _, h2348, _, _) => {
            h2348
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, _, _, h2349, _) => {
            h2349
        }
    }, match v1 {
        Adt39::C0(_, _, _, _, _, _, _, h2350) => {
            h2350
        }
    })
}

pub fn f446(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2351 = v2;
    let a2352 = v1;
    if nat_lt(a2351, a2352) {
        let v3: u64 = f264(v0, v2)?;
        if if f320(v3) {
            true
        } else {
            if f321(v3) {
                true
            } else {
                let a2353 = v3;
                let a2354 = 45u64;
                nat_eq(a2353, a2354)
            }
        } {
            Ok(Some({
                let a2355 = v2;
                let a2356 = 1u64;
                nat_add(a2355, a2356)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f447(v0: Bytes, v1: Bytes, v2: u64, v3: u64, v4: Adt50) -> R<Option<Adt50>> {
    let v5: Adt51 = match v4.clone() {
        Adt50::C0(h2357, _, _, _) => {
            (*h2357).clone()
        }
    };
    let v6: u64 = match v5.clone() {
        Adt51::C0(h2358, _, _) => {
            h2358
        }
    };
    let m2360 = match v4.clone() {
        Adt50::C0(_, _, _, h2359) => {
            h2359
        }
    };
    match m2360 {
        None => {
            let a2361 = v6;
            let a2362 = v2;
            if nat_lt(a2361, a2362) {
                let v8: Adt53 = f480(v0.clone(), v1.clone(), v5.clone())?;
                let m2364 = match v8.clone() {
                    Adt53::C0(_, _, h2363) => {
                        h2363
                    }
                };
                match m2364 {
                    None => {
                        let v10: Adt51 = match v8.clone() {
                            Adt53::C0(_, h2367, _) => {
                                (*h2367).clone()
                            }
                        };
                        let v11: Adt5 = Adt5::C0(match v8 {
                            Adt53::C0(h2368, _, _) => {
                                h2368
                            }
                        }, v6, match v10.clone() {
                            Adt51::C0(h2369, _, _) => {
                                h2369
                            }
                        }, match v5.clone() {
                            Adt51::C0(_, h2370, _) => {
                                h2370
                            }
                        }, match v5 {
                            Adt51::C0(_, _, h2371) => {
                                h2371
                            }
                        }, match v10.clone() {
                            Adt51::C0(_, h2372, _) => {
                                h2372
                            }
                        }, match v10.clone() {
                            Adt51::C0(_, _, h2373) => {
                                h2373
                            }
                        }, f83(v1, v6, {
                            let a2375 = match v10.clone() {
                                Adt51::C0(h2374, _, _) => {
                                    h2374
                                }
                            };
                            let a2376 = v6;
                            nat_sub(a2375, a2376)
                        }));
                        let v12: u64 = {
                            let a2378 = match v4.clone() {
                                Adt50::C0(_, _, h2377, _) => {
                                    h2377
                                }
                            };
                            let a2379 = 1u64;
                            nat_add(a2378, a2379)?
                        };
                        let a2380 = v3;
                        let a2381 = v12;
                        if nat_lt(a2380, a2381) {
                            let v13: Adt48 = f481(v0, v3)?;
                            Ok(Some(Adt50::C0(std::rc::Rc::new(v10), match v4 {
                                Adt50::C0(_, h2382, _, _) => {
                                    h2382
                                }
                            }, v12, Some(v13))))
                        } else {
                            Ok(Some(Adt50::C0(std::rc::Rc::new(v10), List::cons(v11, match v4 {
                                Adt50::C0(_, h2383, _, _) => {
                                    h2383
                                }
                            }), v12, None::<Adt48>)))
                        }
                    }
                    Some(v9) => {
                        Ok(Some(Adt50::C0(std::rc::Rc::new(v5), match v4.clone() {
                            Adt50::C0(_, h2365, _, _) => {
                                h2365
                            }
                        }, match v4 {
                            Adt50::C0(_, _, h2366, _) => {
                                h2366
                            }
                        }, Some(v9))))
                    }
                }
            } else {
                Ok(None::<Adt50>)
            }
        }
        Some(_) => {
            Ok(None::<Adt50>)
        }
    }
}

pub fn f448(v0: Fn73, p2387: List<Adt5>, p2388: List<Adt5>) -> List<Adt5> {
    let m2386 = (p2387, p2388);
    {
        let mut m2386 = m2386;
        loop {
            let step = {
                let (v1, v2) = m2386;
                let m2384 = v2;
                match m2384.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c2385: Fn73 = v0.clone();
                            c2385.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f450(v0: Adt5) -> R<bool> {
    let m2390 = match v0.clone() {
        Adt5::C0(h2389, _, _, _, _, _, _, _) => {
            h2389
        }
    };
    match m2390 {
        Adt6::C0 => {
            Ok(false)
        }
        Adt6::C1 => {
            Ok(true)
        }
        Adt6::C2 => {
            Ok(false)
        }
        Adt6::C3 => {
            Ok(false)
        }
        Adt6::C4 => {
            f171(f482(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2391) => {
                    h2391
                }
            })
        }
        Adt6::C5 => {
            Ok(false)
        }
        Adt6::C6 => {
            Ok(true)
        }
    }
}

pub fn f451(v0: Adt5, v1: u64) -> R<bool> {
    let m2393 = match v0.clone() {
        Adt5::C0(h2392, _, _, _, _, _, _, _) => {
            h2392
        }
    };
    match m2393 {
        Adt6::C0 => {
            Ok(false)
        }
        Adt6::C1 => {
            Ok(true)
        }
        Adt6::C2 => {
            let a2394 = 0u64;
            let a2395 = v1;
            Ok(nat_lt(a2394, a2395))
        }
        Adt6::C3 => {
            f171(f483(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2396) => {
                    h2396
                }
            })
        }
        Adt6::C4 => {
            Ok(!f171(f484(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2397) => {
                    h2397
                }
            })?)
        }
        Adt6::C5 => {
            f485(match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2398) => {
                    h2398
                }
            })
        }
        Adt6::C6 => {
            Ok(true)
        }
    }
}

pub fn f452(v0: Bytes, v1: Bytes) -> Bytes {
    let a2399 = v0;
    let a2400 = v1;
    append_bytes(a2399, a2400)
}

pub fn f453(v0: Adt5) -> Bytes {
    let m2402 = match v0 {
        Adt5::C0(h2401, _, _, _, _, _, _, _) => {
            h2401
        }
    };
    match m2402 {
        Adt6::C0 => {
            f184(Str::lit("control"))
        }
        Adt6::C1 => {
            f184(Str::lit("word"))
        }
        Adt6::C2 => {
            f184(Str::lit("numeral"))
        }
        Adt6::C3 => {
            f184(Str::lit("delimiter"))
        }
        Adt6::C4 => {
            f184(Str::lit("ascii-symbol"))
        }
        Adt6::C5 => {
            f184(Str::lit("unicode-symbol"))
        }
        Adt6::C6 => {
            f184(Str::lit("whitespace"))
        }
    }
}

pub fn f454(v0: u64) -> Bytes {
    let a2403 = v0;
    let a2404 = 0u64;
    if nat_eq(a2403, a2404) {
        f184(Str::lit("text"))
    } else {
        let a2405 = v0;
        let a2406 = 1u64;
        if nat_eq(a2405, a2406) {
            f184(Str::lit("math"))
        } else {
            f184(Str::lit("text and math"))
        }
    }
}

pub fn f455(v0: Adt43, v1: Adt28) -> Adt43 {
    Adt43::C0(match v0.clone() {
        Adt43::C0(h2407, _, _) => {
            h2407
        }
    }, match v0 {
        Adt43::C0(_, h2408, _) => {
            h2408
        }
    }, Some(v1))
}

pub fn f456(v0: Adt43, v1: List<Adt44>, v2: Bytes, v3: List<Adt35>, v4: u64) -> R<Adt43> {
    let v5: u64 = {
        let a2409 = v3.clone();
        length_list(a2409)
    };
    let a2410 = v2.clone();
    let a2411 = f201(Str::lit("form"));
    if equal(a2410, a2411) {
        f486(v0, v1, v3, v4)
    } else {
        let a2412 = v2.clone();
        let a2413 = f201(Str::lit("self-form"));
        if equal(a2412, a2413) {
            f487(v0, v1, v3, v4, f201(Str::lit("a form ID")), f201(Str::lit("a self form is `(self-form form-id)`")), 0u64)
        } else {
            let a2414 = v2.clone();
            let a2415 = f201(Str::lit("slot"));
            if equal(a2414, a2415) {
                f488(v0, v1, v3, v4)
            } else {
                let a2416 = v2.clone();
                let a2417 = f201(Str::lit("space"));
                if equal(a2416, a2417) {
                    let a2418 = v5;
                    let a2419 = 0u64;
                    if nat_eq(a2418, a2419) {
                        Ok(f458(v0, v1, Adt7::C4))
                    } else {
                        Ok(f455(v0, f200(f201(Str::lit("`(space)` takes no operand")))))
                    }
                } else {
                    let a2420 = v2.clone();
                    let a2421 = f201(Str::lit("token"));
                    if equal(a2420, a2421) {
                        f487(v0, v1, v3, v4, f201(Str::lit("a renderer-token ID")), f201(Str::lit("a token is `(token renderer-token-id)`")), 1u64)
                    } else {
                        let a2422 = v2.clone();
                        let a2423 = f201(Str::lit("operator-name"));
                        if equal(a2422, a2423) {
                            f487(v0, v1, v3, v4, f201(Str::lit("an operator name")), f201(Str::lit("an operator name is `(operator-name identifier)`")), 2u64)
                        } else {
                            let a2424 = v2.clone();
                            let a2425 = f201(Str::lit("seq"));
                            if equal(a2424, a2425) {
                                let a2426 = v5;
                                let a2427 = 0u64;
                                if nat_eq(a2426, a2427) {
                                    Ok(f455(v0, f200(f201(Str::lit("`seq` has at least one item")))))
                                } else {
                                    Ok(Adt43::C0(f489(v3, List::cons(Adt44::C1(v2, v5), v1)), match v0 {
                                        Adt43::C0(_, h2428, _) => {
                                            h2428
                                        }
                                    }, None::<Adt28>))
                                }
                            } else {
                                if f460(v2.clone()) {
                                    let a2429 = v5;
                                    let a2430 = 1u64;
                                    if nat_eq(a2429, a2430) {
                                        Ok(Adt43::C0(List::cons(Adt44::C0(std::rc::Rc::new(f385(v3, 0u64))), List::cons(Adt44::C1(v2, 1u64), v1)), match v0 {
                                            Adt43::C0(_, h2431, _) => {
                                                h2431
                                            }
                                        }, None::<Adt28>))
                                    } else {
                                        Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" takes exactly one operand"))))))
                                    }
                                } else {
                                    if f490(v2.clone()) {
                                        let a2432 = v5;
                                        let a2433 = 2u64;
                                        if nat_eq(a2432, a2433) {
                                            Ok(Adt43::C0(List::cons(Adt44::C0(std::rc::Rc::new(f385(v3.clone(), 0u64))), List::cons(Adt44::C0(std::rc::Rc::new(f385(v3, 1u64))), List::cons(Adt44::C1(v2, 2u64), v1))), match v0 {
                                                Adt43::C0(_, h2434, _) => {
                                                    h2434
                                                }
                                            }, None::<Adt28>))
                                        } else {
                                            Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" takes exactly two operands"))))))
                                        }
                                    } else {
                                        Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" is not an LRE form"))))))
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

pub fn f457(v0: List<Adt7>, v1: u64) -> (List<Adt7>, List<Adt7>) {
    let (h2435, _) = f491(Fn76::F492, v1, (List::<Adt7>::nil(), v0));
    h2435
}

pub fn f458(v0: Adt43, v1: List<Adt44>, v2: Adt7) -> Adt43 {
    Adt43::C0(v1, List::cons(v2, match v0 {
        Adt43::C0(_, h2436, _) => {
            h2436
        }
    }), None::<Adt28>)
}

pub fn f459(v0: List<Adt7>) -> Adt7 {
    f493(Fn77::F494, Adt7::C13, f359(v0))
}

pub fn f460(v0: Bytes) -> bool {
    let a2437 = v0.clone();
    let a2438 = f201(Str::lit("group"));
    if equal(a2437, a2438) {
        true
    } else {
        let a2439 = v0.clone();
        let a2440 = f201(Str::lit("paren"));
        if equal(a2439, a2440) {
            true
        } else {
            let a2441 = v0;
            let a2442 = f201(Str::lit("bracket"));
            equal(a2441, a2442)
        }
    }
}

pub fn f461(v0: Bytes, v1: Adt7) -> Adt7 {
    let a2443 = v0.clone();
    let a2444 = f201(Str::lit("group"));
    if equal(a2443, a2444) {
        Adt7::C6(std::rc::Rc::new(v1))
    } else {
        let a2445 = v0;
        let a2446 = f201(Str::lit("paren"));
        if equal(a2445, a2446) {
            Adt7::C7(std::rc::Rc::new(v1))
        } else {
            Adt7::C8(std::rc::Rc::new(v1))
        }
    }
}

pub fn f462(v0: Bytes, v1: Adt7, v2: Adt7) -> Adt7 {
    let a2447 = v0.clone();
    let a2448 = f201(Str::lit("sub"));
    if equal(a2447, a2448) {
        Adt7::C9(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
    } else {
        let a2449 = v0;
        let a2450 = f201(Str::lit("sup"));
        if equal(a2449, a2450) {
            Adt7::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        } else {
            Adt7::C11(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
    }
}

pub fn f463(v0: Bytes, v1: Adt7, v2: u64) -> R<Option<Bytes>> {
    if f495(v1.clone(), v2)? {
        Ok(Some(f205(f301(v0), f201(Str::lit(" operand renders nothing")))))
    } else {
        let a2451 = v0.clone();
        let a2452 = f201(Str::lit("frac"));
        if if equal(a2451, a2452) {
            false
        } else {
            f496(v1)
        } {
            Ok(Some(f205(f301(v0), f201(Str::lit(" operand is a bare script; wrap nested scripts in `(group ...)`")))))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f464(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2497 = {
            let a2473 = {
                let a2461 = {
                    let a2455 = {
                        let a2453 = v1;
                        let a2454 = 128u8;
                        fixed_u8::bit_and(a2453, a2454)
                    };
                    let a2456 = 128u8;
                    if equal(a2455, a2456) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2462 = {
                    let a2459 = {
                        let a2457 = v1;
                        let a2458 = 64u8;
                        fixed_u8::bit_and(a2457, a2458)
                    };
                    let a2460 = 64u8;
                    if equal(a2459, a2460) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2461, a2462)?
            };
            let a2474 = {
                let a2471 = {
                    let a2465 = {
                        let a2463 = v1;
                        let a2464 = 32u8;
                        fixed_u8::bit_and(a2463, a2464)
                    };
                    let a2466 = 32u8;
                    if equal(a2465, a2466) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2472 = {
                    let a2469 = {
                        let a2467 = v1;
                        let a2468 = 16u8;
                        fixed_u8::bit_and(a2467, a2468)
                    };
                    let a2470 = 16u8;
                    if equal(a2469, a2470) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2471, a2472)?
            };
            nat_add(a2473, a2474)?
        };
        let a2498 = {
            let a2495 = {
                let a2483 = {
                    let a2477 = {
                        let a2475 = v1;
                        let a2476 = 8u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 8u8;
                    if equal(a2477, a2478) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2484 = {
                    let a2481 = {
                        let a2479 = v1;
                        let a2480 = 4u8;
                        fixed_u8::bit_and(a2479, a2480)
                    };
                    let a2482 = 4u8;
                    if equal(a2481, a2482) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2483, a2484)?
            };
            let a2496 = {
                let a2493 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 2u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 2u8;
                    if equal(a2487, a2488) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2494 = {
                    let a2491 = {
                        let a2489 = v1;
                        let a2490 = 1u8;
                        fixed_u8::bit_and(a2489, a2490)
                    };
                    let a2492 = 1u8;
                    if equal(a2491, a2492) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2493, a2494)?
            };
            nat_add(a2495, a2496)?
        };
        nat_add(a2497, a2498)?
    };
    let a2499 = 48u64;
    let a2500 = v2;
    if nat_le(a2499, a2500) {
        let a2501 = v2;
        let a2502 = 57u64;
        Ok(nat_le(a2501, a2502))
    } else {
        Ok(false)
    }
}

pub fn f465(v0: Fn74, p2506: List<Adt35>, p2507: List<Adt35>) -> List<Adt35> {
    let m2505 = (p2506, p2507);
    {
        let mut m2505 = m2505;
        loop {
            let step = {
                let (v1, v2) = m2505;
                let m2503 = v2;
                match m2503.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt35>), List<Adt35>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt35>, List<Adt35>), List<Adt35>>(({
                            let c2504: Fn74 = v0.clone();
                            c2504.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: List<Adt35>, v1: Adt35) -> List<Adt35> {
    List::cons(v1, v0)
}

pub fn f467(v0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
    let m2509 = {
        let (h2508, _) = v0;
        h2508
    };
    match m2509 {
        Adt35::C0(_) => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C1(_) => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C2 => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C3(r2511, r2512) => {
            let v3 = (*r2511).clone();
            let v4 = (*r2512).clone();
            Some((v4, List::cons(v3, {
                let (_, h2510) = v0;
                h2510
            })))
        }
    }
}

pub fn f468(v0: List<Adt35>, v1: u64) -> R<Result<Adt1, Adt28>> {
    let a2514 = {
        let a2513 = v0.clone();
        length_list(a2513)
    };
    let a2515 = 1u64;
    if nat_eq(a2514, a2515) {
        let m2516 = f385(v0, 0u64);
        match m2516 {
            Adt35::C0(v2) => {
                let a2517 = v2;
                let a2518 = f201(Str::lit("prop"));
                if equal(a2517, a2518) {
                    Ok(Ok::<Adt1, Adt28>(Adt1::C0))
                } else {
                    Ok(Err::<Adt1, Adt28>(f497()))
                }
            }
            Adt35::C1(r2526) => {
                let v3 = (*r2526).clone();
                let v4: List<Adt35> = f378(v3, v1)?;
                let a2520 = {
                    let a2519 = v4.clone();
                    length_list(a2519)
                };
                let a2521 = 2u64;
                if nat_eq(a2520, a2521) {
                    let m2522 = f385(v4.clone(), 0u64);
                    match m2522 {
                        Adt35::C0(v5) => {
                            let a2523 = v5;
                            let a2524 = f201(Str::lit("type"));
                            if equal(a2523, a2524) {
                                let m2525 = f498(f385(v4, 1u64), v1)?;
                                match m2525 {
                                    Ok(v6) => {
                                        Ok(Ok::<Adt1, Adt28>(Adt1::C1(std::rc::Rc::new(v6))))
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt1, Adt28>(v7))
                                    }
                                }
                            } else {
                                Ok(Err::<Adt1, Adt28>(f497()))
                            }
                        }
                        Adt35::C1(_) => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                        Adt35::C2 => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                        Adt35::C3(_, _) => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                    }
                } else {
                    Ok(Err::<Adt1, Adt28>(f497()))
                }
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f497()))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f497()))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f497()))
    }
}

pub fn f469(v0: List<Adt35>, v1: u64) -> R<Result<Adt1, Adt28>> {
    let v2: u64 = {
        let a2527 = v0.clone();
        length_list(a2527)
    };
    let m2528 = f385(v0.clone(), 0u64);
    match m2528 {
        Adt35::C0(v3) => {
            let a2529 = v2;
            let a2530 = 1u64;
            if nat_eq(a2529, a2530) {
                let m2531 = f231(v3)?;
                match m2531 {
                    Ok(v4) => {
                        Ok(Ok::<Adt1, Adt28>(Adt1::C2({
                            let (h2532, _) = v4;
                            h2532
                        }, {
                            let (_, h2533) = v4;
                            h2533
                        }, std::rc::Rc::new(Adt2::C5))))
                    }
                    Err(v5) => {
                        Ok(Err::<Adt1, Adt28>(v5))
                    }
                }
            } else {
                let a2534 = v2;
                let a2535 = 2u64;
                if nat_eq(a2534, a2535) {
                    let m2536 = f385(v0, 1u64);
                    match m2536 {
                        Adt35::C0(_) => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                        Adt35::C1(r2542) => {
                            let v6 = (*r2542).clone();
                            let v7: List<Adt35> = f378(v6, v1)?;
                            let m2537 = v7.clone();
                            match m2537.uncons() {
                                None => {
                                    Ok(Err::<Adt1, Adt28>(f499()))
                                }
                                Some((_, _)) => {
                                    let m2538 = f231(v3)?;
                                    match m2538 {
                                        Ok(v10) => {
                                            let m2539 = f500(v7, v1)?;
                                            match m2539 {
                                                Ok(v11) => {
                                                    Ok(Ok::<Adt1, Adt28>(Adt1::C2({
                                                        let (h2540, _) = v10;
                                                        h2540
                                                    }, {
                                                        let (_, h2541) = v10;
                                                        h2541
                                                    }, std::rc::Rc::new(v11))))
                                                }
                                                Err(v12) => {
                                                    Ok(Err::<Adt1, Adt28>(v12))
                                                }
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<Adt1, Adt28>(v13))
                                        }
                                    }
                                }
                            }
                        }
                        Adt35::C2 => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                        Adt35::C3(_, _) => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                    }
                } else {
                    Ok(Err::<Adt1, Adt28>(f499()))
                }
            }
        }
        Adt35::C1(_) => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
        Adt35::C2 => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
        Adt35::C3(_, _) => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
    }
}

pub fn f470(v0: List<Adt35>) -> R<Result<Adt1, Adt28>> {
    let a2544 = {
        let a2543 = v0.clone();
        length_list(a2543)
    };
    let a2545 = 1u64;
    if nat_eq(a2544, a2545) {
        let m2546 = f385(v0, 0u64);
        match m2546 {
            Adt35::C0(v1) => {
                if f193(v1.clone())? {
                    Ok(Ok::<Adt1, Adt28>(Adt1::C3(v1)))
                } else {
                    Ok(Err::<Adt1, Adt28>(f200(f205(f301(v1), f201(Str::lit(" is not a valid local ID"))))))
                }
            }
            Adt35::C1(_) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
    }
}

pub fn f471(v0: List<Adt35>) -> R<Result<Adt1, Adt28>> {
    let a2548 = {
        let a2547 = v0.clone();
        length_list(a2547)
    };
    let a2549 = 1u64;
    if nat_eq(a2548, a2549) {
        let m2550 = f385(v0, 0u64);
        match m2550 {
            Adt35::C0(v1) => {
                let m2551 = f501(v1.clone())?;
                match m2551 {
                    None => {
                        Ok(Ok::<Adt1, Adt28>(Adt1::C8(v1)))
                    }
                    Some(v2) => {
                        Ok(Err::<Adt1, Adt28>(v2))
                    }
                }
            }
            Adt35::C1(_) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
    }
}

pub fn f472(v0: Fn75, p2555: List<Adt38>, p2556: List<Adt35>) -> List<Adt38> {
    let m2554 = (p2555, p2556);
    {
        let mut m2554 = m2554;
        loop {
            let step = {
                let (v1, v2) = m2554;
                let m2552 = v2;
                match m2552.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt35>), List<Adt38>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt38>, List<Adt35>), List<Adt38>>(({
                            let c2553: Fn75 = v0.clone();
                            c2553.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: List<Adt38>, v1: Adt35) -> List<Adt38> {
    List::cons(Adt38::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f474(v0: List<Adt35>, v1: List<Adt38>, v2: u64) -> R<Result<List<Adt38>, Adt28>> {
    f502(Fn78::F503(v2), Ok::<List<Adt38>, Adt28>(v1), f424(v0))
}

pub fn f475(v0: Adt35, v1: u64) -> R<Result<Adt54, Adt28>> {
    let m2557 = v0.clone();
    match m2557 {
        Adt35::C0(_) => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
        Adt35::C1(r2564) => {
            let v2 = (*r2564).clone();
            let v3: List<Adt35> = f378(v2, v1)?;
            let a2559 = {
                let a2558 = v3.clone();
                length_list(a2558)
            };
            let a2560 = 3u64;
            if nat_eq(a2559, a2560) {
                let m2561 = f388(f385(v3.clone(), 0u64), f201(Str::lit("a binder mode")), v1);
                match m2561 {
                    Ok(v4) => {
                        let m2562 = f504(v4.clone());
                        match m2562 {
                            None => {
                                Ok(Err::<Adt54, Adt28>(f200(f205(f301(v4), f201(Str::lit(" is not a binder mode"))))))
                            }
                            Some(v5) => {
                                let m2563 = f388(f385(v3.clone(), 1u64), f201(Str::lit("a local ID")), v1);
                                match m2563 {
                                    Ok(v6) => {
                                        if f193(v6.clone())? {
                                            Ok(Ok::<Adt54, Adt28>(Adt54::C0(v5, v6, std::rc::Rc::new(f385(v3, 2u64)))))
                                        } else {
                                            Ok(Err::<Adt54, Adt28>(f200(f205(f301(v6), f201(Str::lit(" is not a valid local ID"))))))
                                        }
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt54, Adt28>(v7))
                                    }
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<Adt54, Adt28>(v8))
                    }
                }
            } else {
                Ok(Err::<Adt54, Adt28>(f200(f201(Str::lit("a binder is `(mode name type)`")))))
            }
        }
        Adt35::C2 => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
        Adt35::C3(_, _) => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
    }
}

pub fn f476(v0: Adt52) -> Adt52 {
    let a2569 = {
        let a2566 = match v0.clone() {
            Adt52::C0(_, h2565, _) => {
                h2565
            }
        };
        let a2567 = 2u64;
        let a2568 = 0u64;
        nat_rem(a2566, a2567, a2568)
    };
    let a2570 = 0u64;
    if nat_eq(a2569, a2570) {
        let m2572 = match v0.clone() {
            Adt52::C0(_, _, h2571) => {
                h2571
            }
        };
        match m2572.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt52::C0(std::rc::Rc::new(f399(v1, match v0.clone() {
                    Adt52::C0(h2573, _, _) => {
                        (*h2573).clone()
                    }
                }, false)), {
                    let a2575 = match v0 {
                        Adt52::C0(_, h2574, _) => {
                            h2574
                        }
                    };
                    let a2576 = 2u64;
                    let a2577 = 0u64;
                    nat_quot(a2575, a2576, a2577)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f477(v0: Adt47, v1: Adt46) -> Adt47 {
    let v2: Adt55 = f505(match v0.clone() {
        Adt47::C0(h2578, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2578
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, h2579, _, _, _, _, _, _, _, _, _, _) => {
            h2579
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, h2580, _, _, _, _, _) => {
            h2580
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2581) => {
            h2581
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, h2582, _, _, _, _, _, _, _) => {
            h2582
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, h2583, _, _, _, _, _, _) => {
            h2583
        }
    });
    let v3: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, h2584, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2584
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, h2585, _, _, _, _, _, _, _, _, _) => {
            h2585
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h2586, _, _, _, _) => {
            h2586
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2587, _, _, _) => {
            h2587
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, h2588, _, _, _, _, _) => {
            h2588
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, h2589, _, _, _, _) => {
            h2589
        }
    });
    let v4: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, h2590, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2590
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, h2591, _, _, _, _, _, _, _, _) => {
            h2591
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, h2592, _, _, _, _, _, _, _) => {
            h2592
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2593, _, _) => {
            h2593
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2594, _, _, _) => {
            h2594
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2595, _, _) => {
            h2595
        }
    });
    let v5: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, _, h2596, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2596
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, h2597, _, _, _, _, _, _, _, _, _, _, _) => {
            h2597
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, h2598, _, _, _, _, _, _) => {
            h2598
        }
    }, match v0 {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2599, _) => {
            h2599
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2600, _) => {
            h2600
        }
    }, match v1 {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2601) => {
            h2601
        }
    });
    Adt47::C0(match v2.clone() {
        Adt55::C0(h2602, _, _, _) => {
            h2602
        }
    }, match v3.clone() {
        Adt55::C0(h2603, _, _, _) => {
            h2603
        }
    }, match v4.clone() {
        Adt55::C0(h2604, _, _, _) => {
            h2604
        }
    }, match v5.clone() {
        Adt55::C0(h2605, _, _, _) => {
            h2605
        }
    }, match v5.clone() {
        Adt55::C0(_, h2606, _, _) => {
            h2606
        }
    }, match v2.clone() {
        Adt55::C0(_, h2607, _, _) => {
            h2607
        }
    }, match v3.clone() {
        Adt55::C0(_, h2608, _, _) => {
            h2608
        }
    }, match v4.clone() {
        Adt55::C0(_, h2609, _, _) => {
            h2609
        }
    }, match v4.clone() {
        Adt55::C0(_, _, h2610, _) => {
            h2610
        }
    }, match v5.clone() {
        Adt55::C0(_, _, h2611, _) => {
            h2611
        }
    }, match v2.clone() {
        Adt55::C0(_, _, h2612, _) => {
            h2612
        }
    }, match v3.clone() {
        Adt55::C0(_, _, h2613, _) => {
            h2613
        }
    }, match v3 {
        Adt55::C0(_, _, _, h2614) => {
            h2614
        }
    }, match v4 {
        Adt55::C0(_, _, _, h2615) => {
            h2615
        }
    }, match v5 {
        Adt55::C0(_, _, _, h2616) => {
            h2616
        }
    }, match v2 {
        Adt55::C0(_, _, _, h2617) => {
            h2617
        }
    })
}

pub fn f478(v0: Adt47, v1: Adt46) -> Adt47 {
    let v2: Adt55 = f505(match v0.clone() {
        Adt47::C0(h2618, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2618
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, h2619, _, _, _, _, _, _, _, _, _, _, _) => {
            h2619
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, h2620, _, _, _, _, _, _, _) => {
            h2620
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2621, _, _, _) => {
            h2621
        }
    }, match v1.clone() {
        Adt46::C0(h2622, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2622
        }
    }, match v1.clone() {
        Adt46::C0(_, h2623, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2623
        }
    });
    let v3: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, h2624, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2624
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, h2625, _, _, _, _, _, _, _, _, _, _) => {
            h2625
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, h2626, _, _, _, _, _, _) => {
            h2626
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2627, _, _) => {
            h2627
        }
    }, match v1.clone() {
        Adt46::C0(_, _, h2628, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2628
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, h2629, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2629
        }
    });
    let v4: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, h2630, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2630
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, h2631, _, _, _, _, _, _, _, _, _) => {
            h2631
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, h2632, _, _, _, _, _) => {
            h2632
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2633, _) => {
            h2633
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, h2634, _, _, _, _, _, _, _, _, _, _, _) => {
            h2634
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, h2635, _, _, _, _, _, _, _, _, _, _) => {
            h2635
        }
    });
    let v5: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, _, h2636, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2636
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, h2637, _, _, _, _, _, _, _, _) => {
            h2637
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h2638, _, _, _, _) => {
            h2638
        }
    }, match v0 {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2639) => {
            h2639
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, h2640, _, _, _, _, _, _, _, _, _) => {
            h2640
        }
    }, match v1 {
        Adt46::C0(_, _, _, _, _, _, _, h2641, _, _, _, _, _, _, _, _) => {
            h2641
        }
    });
    Adt47::C0(match v2.clone() {
        Adt55::C0(h2642, _, _, _) => {
            h2642
        }
    }, match v3.clone() {
        Adt55::C0(h2643, _, _, _) => {
            h2643
        }
    }, match v4.clone() {
        Adt55::C0(h2644, _, _, _) => {
            h2644
        }
    }, match v5.clone() {
        Adt55::C0(h2645, _, _, _) => {
            h2645
        }
    }, match v2.clone() {
        Adt55::C0(_, h2646, _, _) => {
            h2646
        }
    }, match v3.clone() {
        Adt55::C0(_, h2647, _, _) => {
            h2647
        }
    }, match v4.clone() {
        Adt55::C0(_, h2648, _, _) => {
            h2648
        }
    }, match v5.clone() {
        Adt55::C0(_, h2649, _, _) => {
            h2649
        }
    }, match v2.clone() {
        Adt55::C0(_, _, h2650, _) => {
            h2650
        }
    }, match v3.clone() {
        Adt55::C0(_, _, h2651, _) => {
            h2651
        }
    }, match v4.clone() {
        Adt55::C0(_, _, h2652, _) => {
            h2652
        }
    }, match v5.clone() {
        Adt55::C0(_, _, h2653, _) => {
            h2653
        }
    }, match v2 {
        Adt55::C0(_, _, _, h2654) => {
            h2654
        }
    }, match v3 {
        Adt55::C0(_, _, _, h2655) => {
            h2655
        }
    }, match v4 {
        Adt55::C0(_, _, _, h2656) => {
            h2656
        }
    }, match v5 {
        Adt55::C0(_, _, _, h2657) => {
            h2657
        }
    })
}

pub fn f479(v0: Adt46) -> Adt46 {
    Adt46::C0(match v0.clone() {
        Adt46::C0(_, _, h2658, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2658
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, h2659, _, _, _, _, _, _, _, _, _) => {
            h2659
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, h2660, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2660
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, h2661, _, _, _, _, _) => {
            h2661
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, h2662, _, _, _, _, _, _, _, _) => {
            h2662
        }
    }, match v0.clone() {
        Adt46::C0(h2663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2663
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, h2664, _, _, _, _, _, _, _, _, _, _, _) => {
            h2664
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2665, _, _) => {
            h2665
        }
    }, match v0.clone() {
        Adt46::C0(_, h2666, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2666
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, h2667, _, _, _, _) => {
            h2667
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2668, _, _, _) => {
            h2668
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, h2669, _, _, _, _, _, _, _, _, _, _) => {
            h2669
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, h2670, _, _, _, _, _, _) => {
            h2670
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2671, _) => {
            h2671
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2672) => {
            h2672
        }
    }, match v0 {
        Adt46::C0(_, _, _, _, _, _, _, _, h2673, _, _, _, _, _, _, _) => {
            h2673
        }
    })
}

pub fn f480(v0: Bytes, v1: Bytes, v2: Adt51) -> R<Adt53> {
    let v3: u64 = {
        let a2674 = v1.clone();
        length_bytes(a2674)
    };
    let v4: u64 = match v2.clone() {
        Adt51::C0(h2675, _, _) => {
            h2675
        }
    };
    let v5: u8 = f506(v1.clone(), v4);
    let v6: Adt51 = f507(v1.clone(), 0u64, v2.clone())?;
    let v7: Adt51 = f507(v1.clone(), 1u64, v2.clone())?;
    let v8: Adt51 = f507(v1.clone(), 2u64, v2.clone())?;
    let v9: Adt51 = f508(v1.clone(), v2.clone())?;
    let v10: u64 = match v9.clone() {
        Adt51::C0(h2676, _, _) => {
            h2676
        }
    };
    let v11: u8 = f506(v1.clone(), v10);
    let v12: bool = {
        let a2677 = v10;
        let a2678 = v3;
        nat_lt(a2677, a2678)
    };
    let v13: bool = f509(v11);
    let v14: bool = f510(v11, 0u8, 127u8);
    let v15: Adt51 = f507(v1.clone(), 1u64, v9.clone())?;
    let v16: Adt51 = f508(v1.clone(), v9.clone())?;
    let v17: bool = f509(v5);
    let v18: bool = f511(v5);
    let v19: bool = f510(v5, 32u8, 126u8);
    let v20: bool = f510(v5, 0u8, 127u8);
    let v21: bool = {
        let a2679 = v5;
        let a2680 = 123u8;
        if equal(a2679, a2680) {
            true
        } else {
            let a2681 = v5;
            let a2682 = 125u8;
            if equal(a2681, a2682) {
                true
            } else {
                let a2683 = v5;
                let a2684 = 40u8;
                if equal(a2683, a2684) {
                    true
                } else {
                    let a2685 = v5;
                    let a2686 = 41u8;
                    if equal(a2685, a2686) {
                        true
                    } else {
                        let a2687 = v5;
                        let a2688 = 91u8;
                        if equal(a2687, a2688) {
                            true
                        } else {
                            let a2689 = v5;
                            let a2690 = 93u8;
                            equal(a2689, a2690)
                        }
                    }
                }
            }
        }
    };
    let a2691 = v5;
    let a2692 = 32u8;
    if if equal(a2691, a2692) {
        true
    } else {
        let a2693 = v5;
        let a2694 = 10u8;
        equal(a2693, a2694)
    } {
        Ok(Adt53::C0(Adt6::C6, std::rc::Rc::new(v6), None::<Adt48>))
    } else {
        let a2695 = v5;
        let a2696 = 92u8;
        if equal(a2695, a2696) {
            if if v12 {
                v13
            } else {
                false
            } {
                Ok(Adt53::C0(Adt6::C0, std::rc::Rc::new(v15), None::<Adt48>))
            } else {
                if if v12 {
                    v14
                } else {
                    false
                } {
                    Ok(Adt53::C0(Adt6::C0, std::rc::Rc::new(v16), None::<Adt48>))
                } else {
                    let v22: Adt48 = f512(v0, v2, v9.clone());
                    Ok(Adt53::C0(Adt6::C0, std::rc::Rc::new(v9), Some(v22)))
                }
            }
        } else {
            if v17 {
                Ok(Adt53::C0(Adt6::C1, std::rc::Rc::new(v7), None::<Adt48>))
            } else {
                if v18 {
                    Ok(Adt53::C0(Adt6::C2, std::rc::Rc::new(v8), None::<Adt48>))
                } else {
                    if v21 {
                        Ok(Adt53::C0(Adt6::C3, std::rc::Rc::new(v9), None::<Adt48>))
                    } else {
                        if v19 {
                            Ok(Adt53::C0(Adt6::C4, std::rc::Rc::new(v9), None::<Adt48>))
                        } else {
                            if v20 {
                                let v23: Adt48 = f513(v0, v1, v2.clone())?;
                                Ok(Adt53::C0(Adt6::C0, std::rc::Rc::new(v2), Some(v23)))
                            } else {
                                Ok(Adt53::C0(Adt6::C5, std::rc::Rc::new(v9), None::<Adt48>))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f481(v0: Bytes, v1: u64) -> R<Adt48> {
    Ok(Adt48::C0(f514(Str::lit("LLS8002")), f515(f515(f515(f514(Str::lit("max_primitive_atoms exceeded: configured ")), f516(v1)?), f514(Str::lit(", scanning "))), v0), None::<Adt49>, None::<Bytes>))
}

pub fn f482() -> List<Bytes> {
    List::cons(f184(Str::lit(":")), List::cons(f184(Str::lit("-")), List::cons(f184(Str::lit(",")), List::cons(f184(Str::lit(".")), List::cons(f184(Str::lit(";")), List::cons(f184(Str::lit("'")), List::<Bytes>::nil()))))))
}

pub fn f483() -> List<Bytes> {
    List::cons(f184(Str::lit("(")), List::cons(f184(Str::lit(")")), List::cons(f184(Str::lit("[")), List::cons(f184(Str::lit("]")), List::<Bytes>::nil()))))
}

pub fn f484() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x60]), List::<Bytes>::nil()))))))))))))
}

pub fn f485(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2697 = v0.clone();
        length_bytes(a2697)
    };
    let a2701 = v1;
    let a2702 = {
        let (h2698, _) = f240(Fn39::F517(v1, v0), {
            let a2699 = v1;
            let a2700 = 1u64;
            nat_add(a2699, a2700)?
        }, 0u64)?;
        h2698
    };
    Ok(nat_le(a2701, a2702))
}

pub fn f486(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64) -> R<Adt43> {
    let a2704 = {
        let a2703 = v2.clone();
        length_list(a2703)
    };
    let a2705 = 2u64;
    if nat_eq(a2704, a2705) {
        let m2706 = f388(f385(v2.clone(), 0u64), f201(Str::lit("a qualified ID")), v3);
        match m2706 {
            Ok(v4) => {
                let m2707 = f231(v4)?;
                match m2707 {
                    Ok(v5) => {
                        let m2708 = f388(f385(v2, 1u64), f201(Str::lit("a form ID")), v3);
                        match m2708 {
                            Ok(v6) => {
                                if f284(v6.clone())? {
                                    Ok(f458(v0, v1, Adt7::C0({
                                        let (h2709, _) = v5;
                                        h2709
                                    }, {
                                        let (_, h2710) = v5;
                                        h2710
                                    }, v6)))
                                } else {
                                    Ok(f455(v0, f200(f205(f301(v6), f201(Str::lit(" is not a form ID"))))))
                                }
                            }
                            Err(v7) => {
                                Ok(f455(v0, v7))
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(f455(v0, v8))
                    }
                }
            }
            Err(v9) => {
                Ok(f455(v0, v9))
            }
        }
    } else {
        Ok(f455(v0, f200(f201(Str::lit("a form reference is `(form qualified-id form-id)`")))))
    }
}

pub fn f487(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64, v4: Bytes, v5: Bytes, v6: u64) -> R<Adt43> {
    let a2712 = {
        let a2711 = v2.clone();
        length_list(a2711)
    };
    let a2713 = 1u64;
    if nat_eq(a2712, a2713) {
        let m2714 = f388(f385(v2, 0u64), v4, v3);
        match m2714 {
            Ok(v7) => {
                let a2715 = v6;
                let a2716 = 0u64;
                if nat_eq(a2715, a2716) {
                    if f284(v7.clone())? {
                        Ok(f458(v0, v1, Adt7::C1(v7)))
                    } else {
                        Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not a form ID"))))))
                    }
                } else {
                    let a2717 = v6;
                    let a2718 = 1u64;
                    if nat_eq(a2717, a2718) {
                        if f345(v7.clone())? {
                            Ok(f458(v0, v1, Adt7::C5(v7)))
                        } else {
                            Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not a renderer-token ID"))))))
                        }
                    } else {
                        if f518(v7.clone())? {
                            Ok(f458(v0, v1, Adt7::C12(v7)))
                        } else {
                            Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not an operator name"))))))
                        }
                    }
                }
            }
            Err(v8) => {
                Ok(f455(v0, v8))
            }
        }
    } else {
        Ok(f455(v0, f200(v5)))
    }
}

pub fn f488(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64) -> R<Adt43> {
    let a2720 = {
        let a2719 = v2.clone();
        length_list(a2719)
    };
    let a2721 = 1u64;
    if nat_eq(a2720, a2721) {
        let m2722 = f388(f385(v2, 0u64), f201(Str::lit("a slot index")), v3);
        match m2722 {
            Ok(v4) => {
                let m2723 = f501(v4.clone())?;
                match m2723 {
                    None => {
                        let v6: u64 = f519(v4.clone())?;
                        let a2724 = 4294967295u64;
                        let a2725 = v6;
                        if nat_lt(a2724, a2725) {
                            Ok(f455(v0, f200(f205(f301(v4), f201(Str::lit(" is not a slot index"))))))
                        } else {
                            Ok(f458(v0, v1, Adt7::C2(v6)))
                        }
                    }
                    Some(_) => {
                        Ok(f455(v0, f200(f205(f301(v4), f201(Str::lit(" is not a slot index"))))))
                    }
                }
            }
            Err(v7) => {
                Ok(f455(v0, v7))
            }
        }
    } else {
        Ok(f455(v0, f200(f201(Str::lit("a slot is `(slot decimal)`")))))
    }
}

pub fn f489(v0: List<Adt35>, v1: List<Adt44>) -> List<Adt44> {
    f520(Fn79::F521, v1, f424(v0))
}

pub fn f490(v0: Bytes) -> bool {
    let a2726 = v0.clone();
    let a2727 = f201(Str::lit("sub"));
    if equal(a2726, a2727) {
        true
    } else {
        let a2728 = v0.clone();
        let a2729 = f201(Str::lit("sup"));
        if equal(a2728, a2729) {
            true
        } else {
            let a2730 = v0;
            let a2731 = f201(Str::lit("frac"));
            equal(a2730, a2731)
        }
    }
}

pub fn f491(v0: Fn76, p2736: u64, p2737: (List<Adt7>, List<Adt7>)) -> ((List<Adt7>, List<Adt7>), bool) {
    let m2735 = (p2736, p2737);
    {
        let mut m2735 = m2735;
        loop {
            let step = {
                let (v1, v2) = m2735;
                let m2732 = v1;
                if m2732 == 0 {
                    Err::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v2, false))
                } else {
                    let v3 = m2732 - 1;
                    let m2734 = {
                        let c2733: Fn76 = v0.clone();
                        c2733.apply(v2.clone())
                    };
                    match m2734 {
                        None => {
                            Err::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: (List<Adt7>, List<Adt7>)) -> Option<(List<Adt7>, List<Adt7>)> {
    let m2739 = {
        let (_, h2738) = v0;
        h2738
    };
    match m2739.uncons() {
        None => {
            None::<(List<Adt7>, List<Adt7>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h2740, _) = v0;
                h2740
            }), v2))
        }
    }
}

pub fn f493(v0: Fn77, p2744: Adt7, p2745: List<Adt7>) -> Adt7 {
    let m2743 = (p2744, p2745);
    {
        let mut m2743 = m2743;
        loop {
            let step = {
                let (v1, v2) = m2743;
                let m2741 = v2;
                match m2741.uncons() {
                    None => {
                        Err::<(Adt7, List<Adt7>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<Adt7>), Adt7>(({
                            let c2742: Fn77 = v0.clone();
                            c2742.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Adt7, v1: Adt7) -> Adt7 {
    Adt7::C14(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f495(v0: Adt7, v1: u64) -> R<bool> {
    let a2750 = {
        let (_, h2746) = {
            let (h2747, _) = f522(Fn80::F523(v1), {
                let a2748 = v1;
                let a2749 = 1u64;
                nat_add(a2748, a2749)?
            }, (List::cons(v0, List::<Adt7>::nil()), 0u64))?;
            h2747
        };
        h2746
    };
    let a2751 = 0u64;
    Ok(nat_eq(a2750, a2751))
}

pub fn f496(v0: Adt7) -> bool {
    let m2752 = v0;
    match m2752 {
        Adt7::C0(_, _, _) => {
            false
        }
        Adt7::C1(_) => {
            false
        }
        Adt7::C2(_) => {
            false
        }
        Adt7::C3(_) => {
            false
        }
        Adt7::C4 => {
            false
        }
        Adt7::C5(_) => {
            false
        }
        Adt7::C6(_) => {
            false
        }
        Adt7::C7(_) => {
            false
        }
        Adt7::C8(_) => {
            false
        }
        Adt7::C9(_, _) => {
            true
        }
        Adt7::C10(_, _) => {
            true
        }
        Adt7::C11(_, _) => {
            false
        }
        Adt7::C12(_) => {
            false
        }
        Adt7::C13 => {
            false
        }
        Adt7::C14(_, _) => {
            false
        }
    }
}

pub fn f497() -> Adt28 {
    f200(f201(Str::lit("a sort is `(sort prop)` or `(sort (type u))`")))
}

pub fn f498(v0: Adt35, v1: u64) -> R<Result<Adt2, Adt28>> {
    let v2: Adt56 = {
        let (h2753, _) = f524(Fn81::F525(v1), {
            let a2756 = {
                let a2754 = v1;
                let a2755 = 2u64;
                nat_mul(a2754, a2755)?
            };
            let a2757 = 2u64;
            nat_add(a2756, a2757)?
        }, Adt56::C0(List::cons(Adt57::C0(std::rc::Rc::new(v0)), List::<Adt57>::nil()), List::<Adt2>::nil(), None::<Adt28>))?;
        h2753
    };
    let m2759 = match v2.clone() {
        Adt56::C0(_, _, h2758) => {
            h2758
        }
    };
    match m2759 {
        None => {
            let m2761 = match v2 {
                Adt56::C0(_, h2760, _) => {
                    h2760
                }
            };
            match m2761.uncons() {
                None => {
                    Ok(Err::<Adt2, Adt28>(f200(f201(Str::lit("expected a universe")))))
                }
                Some((v4, _)) => {
                    Ok(Ok::<Adt2, Adt28>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt2, Adt28>(v3))
        }
    }
}

pub fn f499() -> Adt28 {
    f200(f201(Str::lit("a const is `(const qualified-id [universes])`")))
}

pub fn f500(v0: List<Adt35>, v1: u64) -> R<Result<Adt2, Adt28>> {
    let m2762 = f526(Fn82::F527(v1), Ok::<List<Adt2>, Adt28>(List::<Adt2>::nil()), v0)?;
    match m2762 {
        Ok(v2) => {
            Ok(Ok::<Adt2, Adt28>(f528(f529(v2))))
        }
        Err(v3) => {
            Ok(Err::<Adt2, Adt28>(v3))
        }
    }
}

pub fn f501(v0: Bytes) -> R<Option<Adt28>> {
    let v1: u64 = {
        let a2763 = v0.clone();
        length_bytes(a2763)
    };
    let a2764 = v1;
    let a2765 = 0u64;
    if if nat_eq(a2764, a2765) {
        true
    } else {
        !f530(v0.clone())?
    } {
        Ok(Some(f200(f205(f301(v0), f201(Str::lit(" is not a decimal"))))))
    } else {
        let a2766 = 1u64;
        let a2767 = v1;
        if if nat_lt(a2766, a2767) {
            let a2768 = f264(v0.clone(), 0u64)?;
            let a2769 = 48u64;
            nat_eq(a2768, a2769)
        } else {
            false
        } {
            Ok(Some(f200(f205(f301(v0), f201(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a2770 = 20u64;
            let a2771 = v1;
            if if nat_lt(a2770, a2771) {
                true
            } else {
                let a2772 = v1;
                let a2773 = 20u64;
                if nat_eq(a2772, a2773) {
                    let a2778 = {
                        let a2774 = v0.clone();
                        let a2775 = f201(Str::lit("18446744073709551615"));
                        compare_bytes(a2774, a2775)
                    };
                    let a2779 = {
                        let a2776 = Bytes::lit(&[0x01]);
                        let a2777 = Bytes::lit(&[]);
                        compare_bytes(a2776, a2777)
                    };
                    equal(a2778, a2779)
                } else {
                    false
                }
            } {
                Ok(Some(f200(f205(f301(v0), f201(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt28>)
            }
        }
    }
}

pub fn f502(v0: Fn78, p2783: Result<List<Adt38>, Adt28>, p2784: List<Adt35>) -> R<Result<List<Adt38>, Adt28>> {
    let m2782 = (p2783, p2784);
    {
        let mut m2782 = m2782;
        loop {
            let step = {
                let (v1, v2) = m2782;
                let m2780 = v2;
                match m2780.uncons() {
                    None => {
                        Err::<(Result<List<Adt38>, Adt28>, List<Adt35>), R<Result<List<Adt38>, Adt28>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt38>, Adt28>, List<Adt35>), R<Result<List<Adt38>, Adt28>>>(({
                            let c2781: Fn78 = v0.clone();
                            c2781.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: u64, v1: Result<List<Adt38>, Adt28>, v2: Adt35) -> R<Result<List<Adt38>, Adt28>> {
    let m2785 = v1;
    match m2785 {
        Ok(v3) => {
            let m2786 = f475(v2, v0)?;
            match m2786 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt38>, Adt28>(List::cons(Adt38::C0(std::rc::Rc::new(match v4 {
                        Adt54::C0(_, _, h2787) => {
                            (*h2787).clone()
                        }
                    })), v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt38>, Adt28>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt38>, Adt28>(v6))
        }
    }
}

pub fn f504(v0: Bytes) -> Option<u64> {
    let a2788 = v0.clone();
    let a2789 = f201(Str::lit("explicit"));
    if equal(a2788, a2789) {
        Some(0u64)
    } else {
        let a2790 = v0.clone();
        let a2791 = f201(Str::lit("implicit"));
        if equal(a2790, a2791) {
            Some(1u64)
        } else {
            let a2792 = v0;
            let a2793 = f201(Str::lit("instance"));
            if equal(a2792, a2793) {
                Some(2u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f505(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt55 {
    let v6: u32 = f531(f531(v0, v1), v4);
    let v7: u32 = f532({
        let a2794 = v3;
        let a2795 = v6;
        fixed_u32::bit_xor(a2794, a2795)
    }, 16u32, 16u32);
    let v8: u32 = f531(v2, v7);
    let v9: u32 = f532({
        let a2796 = v1;
        let a2797 = v8;
        fixed_u32::bit_xor(a2796, a2797)
    }, 12u32, 20u32);
    let v10: u32 = f531(f531(v6, v9), v5);
    let v11: u32 = f532({
        let a2798 = v7;
        let a2799 = v10;
        fixed_u32::bit_xor(a2798, a2799)
    }, 8u32, 24u32);
    let v12: u32 = f531(v8, v11);
    let v13: u32 = f532({
        let a2800 = v9;
        let a2801 = v12;
        fixed_u32::bit_xor(a2800, a2801)
    }, 7u32, 25u32);
    Adt55::C0(v10, v13, v12, v11)
}

pub fn f506(v0: Bytes, v1: u64) -> u8 {
    let m2802 = f247(v0, v1);
    match m2802 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f507(v0: Bytes, v1: u64, v2: Adt51) -> R<Adt51> {
    let v3: u64 = {
        let a2803 = v0.clone();
        length_bytes(a2803)
    };
    let (h2804, _) = f533(Fn83::F534(v1, v3, v0), {
        let a2806 = v3;
        let a2807 = match v2.clone() {
            Adt51::C0(h2805, _, _) => {
                h2805
            }
        };
        nat_sub(a2806, a2807)
    }, v2)?;
    Ok(h2804)
}

pub fn f508(v0: Bytes, v1: Adt51) -> R<Adt51> {
    let v2: u64 = match v1.clone() {
        Adt51::C0(h2808, _, _) => {
            h2808
        }
    };
    let v3: u64 = f535(v0.clone(), v2);
    let v4: bool = f536(v0, v2, 10u8);
    if v4 {
        Ok(Adt51::C0({
            let a2809 = v2;
            let a2810 = 1u64;
            nat_add(a2809, a2810)?
        }, {
            let a2812 = match v1 {
                Adt51::C0(_, h2811, _) => {
                    h2811
                }
            };
            let a2813 = 1u64;
            nat_add(a2812, a2813)?
        }, 1u64))
    } else {
        Ok(Adt51::C0({
            let a2814 = v2;
            let a2815 = v3;
            nat_add(a2814, a2815)?
        }, match v1.clone() {
            Adt51::C0(_, h2816, _) => {
                h2816
            }
        }, {
            let a2818 = match v1 {
                Adt51::C0(_, _, h2817) => {
                    h2817
                }
            };
            let a2819 = 1u64;
            nat_add(a2818, a2819)?
        }))
    }
}

pub fn f509(v0: u8) -> bool {
    let v1: bool = f510(v0, 97u8, 122u8);
    let v2: bool = f510(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f510(v0: u8, v1: u8, v2: u8) -> bool {
    let m2822 = {
        let a2820 = v0;
        let a2821 = v1;
        fixed_u8::checked_sub(a2820, a2821)
    };
    match m2822 {
        None => {
            false
        }
        Some(_) => {
            let m2825 = {
                let a2823 = v2;
                let a2824 = v0;
                fixed_u8::checked_sub(a2823, a2824)
            };
            match m2825 {
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

pub fn f511(v0: u8) -> bool {
    f510(v0, 48u8, 57u8)
}

pub fn f512(v0: Bytes, v1: Adt51, v2: Adt51) -> Adt48 {
    let v3: Adt49 = f537(v0, v1, v2);
    Adt48::C0(f514(Str::lit("LLL1004")), f514(Str::lit("a control sequence needs ASCII letters or one ASCII nonletter after the backslash")), Some(v3), None::<Bytes>)
}

pub fn f513(v0: Bytes, v1: Bytes, v2: Adt51) -> R<Adt48> {
    let v3: u8 = f506(v1, match v2.clone() {
        Adt51::C0(h2826, _, _) => {
            h2826
        }
    });
    let v5: u8 = {
        let m2829 = {
            let a2827 = v3;
            let a2828 = 4u32;
            fixed_u8::shift_right(a2827, a2828)
        };
        match m2829 {
            None => {
                0u8
            }
            Some(v4) => {
                v4
            }
        }
    };
    let v6: u8 = {
        let a2830 = v3;
        let a2831 = 15u8;
        fixed_u8::bit_and(a2830, a2831)
    };
    let v7: Bytes = f538(v5);
    let v8: Bytes = f538(v6);
    let v9: Adt51 = Adt51::C0({
        let a2833 = match v2.clone() {
            Adt51::C0(h2832, _, _) => {
                h2832
            }
        };
        let a2834 = 1u64;
        nat_add(a2833, a2834)?
    }, match v2.clone() {
        Adt51::C0(_, h2835, _) => {
            h2835
        }
    }, {
        let a2837 = match v2.clone() {
            Adt51::C0(_, _, h2836) => {
                h2836
            }
        };
        let a2838 = 1u64;
        nat_add(a2837, a2838)?
    });
    let v10: Adt49 = f537(v0, v2, v9);
    Ok(Adt48::C0(f514(Str::lit("LLL1001")), f515(f515(f514(Str::lit("forbidden scalar U+00")), v7), v8), Some(v10), None::<Bytes>))
}

pub fn f514(v0: Str) -> Bytes {
    let a2839 = v0;
    utf8_encode(a2839)
}

pub fn f515(v0: Bytes, v1: Bytes) -> Bytes {
    let a2840 = v0;
    let a2841 = v1;
    append_bytes(a2840, a2841)
}

pub fn f516(v0: u64) -> R<Bytes> {
    let v1: Bytes = f514(Str::lit("0"));
    let v2: Bytes = f539({
        let a2878 = f540(v0, 10000000000000000000u64);
        let a2879 = {
            let a2876 = f540(v0, 1000000000000000000u64);
            let a2877 = {
                let a2874 = f540(v0, 100000000000000000u64);
                let a2875 = {
                    let a2872 = f540(v0, 10000000000000000u64);
                    let a2873 = {
                        let a2870 = f540(v0, 1000000000000000u64);
                        let a2871 = {
                            let a2868 = f540(v0, 100000000000000u64);
                            let a2869 = {
                                let a2866 = f540(v0, 10000000000000u64);
                                let a2867 = {
                                    let a2864 = f540(v0, 1000000000000u64);
                                    let a2865 = {
                                        let a2862 = f540(v0, 100000000000u64);
                                        let a2863 = {
                                            let a2860 = f540(v0, 10000000000u64);
                                            let a2861 = {
                                                let a2858 = f540(v0, 1000000000u64);
                                                let a2859 = {
                                                    let a2856 = f540(v0, 100000000u64);
                                                    let a2857 = {
                                                        let a2854 = f540(v0, 10000000u64);
                                                        let a2855 = {
                                                            let a2852 = f540(v0, 1000000u64);
                                                            let a2853 = {
                                                                let a2850 = f540(v0, 100000u64);
                                                                let a2851 = {
                                                                    let a2848 = f540(v0, 10000u64);
                                                                    let a2849 = {
                                                                        let a2846 = f540(v0, 1000u64);
                                                                        let a2847 = {
                                                                            let a2844 = f540(v0, 100u64);
                                                                            let a2845 = {
                                                                                let a2842 = f540(v0, 10u64);
                                                                                let a2843 = f540(v0, 1u64);
                                                                                append_bytes(a2842, a2843)
                                                                            };
                                                                            append_bytes(a2844, a2845)
                                                                        };
                                                                        append_bytes(a2846, a2847)
                                                                    };
                                                                    append_bytes(a2848, a2849)
                                                                };
                                                                append_bytes(a2850, a2851)
                                                            };
                                                            append_bytes(a2852, a2853)
                                                        };
                                                        append_bytes(a2854, a2855)
                                                    };
                                                    append_bytes(a2856, a2857)
                                                };
                                                append_bytes(a2858, a2859)
                                            };
                                            append_bytes(a2860, a2861)
                                        };
                                        append_bytes(a2862, a2863)
                                    };
                                    append_bytes(a2864, a2865)
                                };
                                append_bytes(a2866, a2867)
                            };
                            append_bytes(a2868, a2869)
                        };
                        append_bytes(a2870, a2871)
                    };
                    append_bytes(a2872, a2873)
                };
                append_bytes(a2874, a2875)
            };
            append_bytes(a2876, a2877)
        };
        append_bytes(a2878, a2879)
    })?;
    let a2880 = v0;
    let a2881 = 0u64;
    if nat_eq(a2880, a2881) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f517(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f541(v1, v0, v2)
}

pub fn f518(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2882 = v0.clone();
        length_bytes(a2882)
    };
    let a2883 = v1;
    let a2884 = 0u64;
    if nat_eq(a2883, a2884) {
        Ok(false)
    } else {
        if f263(f264(v0.clone(), 0u64)?) {
            let a2886 = {
                let (h2885, _) = f240(Fn39::F542(v1, v0), v1, 1u64)?;
                h2885
            };
            let a2887 = v1;
            Ok(nat_eq(a2886, a2887))
        } else {
            Ok(false)
        }
    }
}

pub fn f519(v0: Bytes) -> R<u64> {
    let m2888 = f246(v0)?;
    match m2888 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f520(v0: Fn79, p2892: List<Adt44>, p2893: List<Adt35>) -> List<Adt44> {
    let m2891 = (p2892, p2893);
    {
        let mut m2891 = m2891;
        loop {
            let step = {
                let (v1, v2) = m2891;
                let m2889 = v2;
                match m2889.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt35>), List<Adt44>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt44>, List<Adt35>), List<Adt44>>(({
                            let c2890: Fn79 = v0.clone();
                            c2890.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<Adt44>, v1: Adt35) -> List<Adt44> {
    List::cons(Adt44::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f522(v0: Fn80, p2898: u64, p2899: (List<Adt7>, u64)) -> R<((List<Adt7>, u64), bool)> {
    let m2897 = (p2898, p2899);
    {
        let mut m2897 = m2897;
        loop {
            let step = {
                let (v1, v2) = m2897;
                let m2894 = v1;
                if m2894 == 0 {
                    Err::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2894 - 1;
                    let m2896 = {
                        let c2895: Fn80 = v0.clone();
                        c2895.apply(v2.clone())?
                    };
                    match m2896 {
                        None => {
                            Err::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: u64, v1: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
    f543(v0, v1)
}

pub fn f524(v0: Fn81, p2904: u64, p2905: Adt56) -> R<(Adt56, bool)> {
    let m2903 = (p2904, p2905);
    {
        let mut m2903 = m2903;
        loop {
            let step = {
                let (v1, v2) = m2903;
                let m2900 = v1;
                if m2900 == 0 {
                    Err::<(u64, Adt56), R<(Adt56, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2900 - 1;
                    let m2902 = {
                        let c2901: Fn81 = v0.clone();
                        c2901.apply(v2.clone())?
                    };
                    match m2902 {
                        None => {
                            Err::<(u64, Adt56), R<(Adt56, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt56), R<(Adt56, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: u64, v1: Adt56) -> R<Option<Adt56>> {
    f544(v0, v1)
}

pub fn f526(v0: Fn82, p2909: Result<List<Adt2>, Adt28>, p2910: List<Adt35>) -> R<Result<List<Adt2>, Adt28>> {
    let m2908 = (p2909, p2910);
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let (v1, v2) = m2908;
                let m2906 = v2;
                match m2906.uncons() {
                    None => {
                        Err::<(Result<List<Adt2>, Adt28>, List<Adt35>), R<Result<List<Adt2>, Adt28>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt2>, Adt28>, List<Adt35>), R<Result<List<Adt2>, Adt28>>>(({
                            let c2907: Fn82 = v0.clone();
                            c2907.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: u64, v1: Result<List<Adt2>, Adt28>, v2: Adt35) -> R<Result<List<Adt2>, Adt28>> {
    let m2911 = v1;
    match m2911 {
        Ok(v3) => {
            let m2912 = f498(v2, v0)?;
            match m2912 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt2>, Adt28>(List::cons(v4, v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt2>, Adt28>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt2>, Adt28>(v6))
        }
    }
}

pub fn f528(v0: List<Adt2>) -> Adt2 {
    f545(Fn84::F546, Adt2::C5, f529(v0))
}

pub fn f529(v0: List<Adt2>) -> List<Adt2> {
    f547(Fn85::F548, List::<Adt2>::nil(), v0)
}

pub fn f530(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2913 = v0.clone();
        length_bytes(a2913)
    };
    let a2917 = {
        let (h2914, _) = f240(Fn39::F549(v1, v0), {
            let a2915 = v1;
            let a2916 = 1u64;
            nat_add(a2915, a2916)?
        }, 0u64)?;
        h2914
    };
    let a2918 = v1;
    Ok(nat_eq(a2917, a2918))
}

pub fn f531(v0: u32, v1: u32) -> u32 {
    let m2921 = {
        let a2919 = f550(v0);
        let a2920 = f550(v1);
        fixed_u64::checked_add(a2919, a2920)
    };
    match m2921 {
        None => {
            0u32
        }
        Some(v2) => {
            f551(v2)
        }
    }
}

pub fn f532(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2922 = f344(v0, v1);
    let a2923 = f444(v0, v2);
    fixed_u32::bit_or(a2922, a2923)
}

pub fn f533(v0: Fn83, p2928: u64, p2929: Adt51) -> R<(Adt51, bool)> {
    let m2927 = (p2928, p2929);
    {
        let mut m2927 = m2927;
        loop {
            let step = {
                let (v1, v2) = m2927;
                let m2924 = v1;
                if m2924 == 0 {
                    Err::<(u64, Adt51), R<(Adt51, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2924 - 1;
                    let m2926 = {
                        let c2925: Fn83 = v0.clone();
                        c2925.apply(v2.clone())?
                    };
                    match m2926 {
                        None => {
                            Err::<(u64, Adt51), R<(Adt51, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt51), R<(Adt51, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: u64, v1: u64, v2: Bytes, v3: Adt51) -> R<Option<Adt51>> {
    f552(v2, v1, v0, v3)
}

pub fn f535(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f506(v0, v1);
    let v3: bool = f510(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f510(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f510(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m2930 = f247(v0, v1);
    match m2930 {
        None => {
            false
        }
        Some(v3) => {
            let a2931 = v3;
            let a2932 = v2;
            equal(a2931, a2932)
        }
    }
}

pub fn f537(v0: Bytes, v1: Adt51, v2: Adt51) -> Adt49 {
    Adt49::C0(v0, match v1.clone() {
        Adt51::C0(h2933, _, _) => {
            h2933
        }
    }, match v2.clone() {
        Adt51::C0(h2934, _, _) => {
            h2934
        }
    }, match v1.clone() {
        Adt51::C0(_, h2935, _) => {
            h2935
        }
    }, match v1 {
        Adt51::C0(_, _, h2936) => {
            h2936
        }
    }, match v2.clone() {
        Adt51::C0(_, h2937, _) => {
            h2937
        }
    }, match v2 {
        Adt51::C0(_, _, h2938) => {
            h2938
        }
    })
}

pub fn f538(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a2939 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2939)
    };
    let v2: u64 = {
        let a2940 = v0;
        let a2941 = 0u8;
        if equal(a2940, a2941) {
            0u64
        } else {
            let a2942 = v0;
            let a2943 = 1u8;
            if equal(a2942, a2943) {
                1u64
            } else {
                let a2944 = v0;
                let a2945 = 2u8;
                if equal(a2944, a2945) {
                    2u64
                } else {
                    let a2946 = v0;
                    let a2947 = 3u8;
                    if equal(a2946, a2947) {
                        3u64
                    } else {
                        let a2948 = v0;
                        let a2949 = 4u8;
                        if equal(a2948, a2949) {
                            4u64
                        } else {
                            let a2950 = v0;
                            let a2951 = 5u8;
                            if equal(a2950, a2951) {
                                5u64
                            } else {
                                let a2952 = v0;
                                let a2953 = 6u8;
                                if equal(a2952, a2953) {
                                    6u64
                                } else {
                                    let a2954 = v0;
                                    let a2955 = 7u8;
                                    if equal(a2954, a2955) {
                                        7u64
                                    } else {
                                        let a2956 = v0;
                                        let a2957 = 8u8;
                                        if equal(a2956, a2957) {
                                            8u64
                                        } else {
                                            let a2958 = v0;
                                            let a2959 = 9u8;
                                            if equal(a2958, a2959) {
                                                9u64
                                            } else {
                                                let a2960 = v0;
                                                let a2961 = 10u8;
                                                if equal(a2960, a2961) {
                                                    10u64
                                                } else {
                                                    let a2962 = v0;
                                                    let a2963 = 11u8;
                                                    if equal(a2962, a2963) {
                                                        11u64
                                                    } else {
                                                        let a2964 = v0;
                                                        let a2965 = 12u8;
                                                        if equal(a2964, a2965) {
                                                            12u64
                                                        } else {
                                                            let a2966 = v0;
                                                            let a2967 = 13u8;
                                                            if equal(a2966, a2967) {
                                                                13u64
                                                            } else {
                                                                let a2968 = v0;
                                                                let a2969 = 14u8;
                                                                if equal(a2968, a2969) {
                                                                    14u64
                                                                } else {
                                                                    15u64
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
    };
    f83(v1, v2, 1u64)
}

pub fn f539(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h2970, _) = f240(Fn39::F553(v0.clone()), {
            let a2971 = v0.clone();
            length_bytes(a2971)
        }, 0u64)?;
        h2970
    };
    Ok(f83(v0.clone(), v1, {
        let a2973 = {
            let a2972 = v0;
            length_bytes(a2972)
        };
        let a2974 = v1;
        nat_sub(a2973, a2974)
    }))
}

pub fn f540(v0: u64, v1: u64) -> Bytes {
    f83(f514(Str::lit("0123456789")), {
        let a2978 = {
            let a2975 = v0;
            let a2976 = v1;
            let a2977 = 0u64;
            nat_quot(a2975, a2976, a2977)
        };
        let a2979 = 10u64;
        let a2980 = 0u64;
        nat_rem(a2978, a2979, a2980)
    }, 1u64)
}

pub fn f541(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2981 = v2;
    let a2982 = v1;
    if nat_lt(a2981, a2982) {
        if f554(f555(v0.clone(), v2)?) {
            Ok(Some({
                let a2983 = v2;
                let a2984 = f556(f264(v0, v2)?);
                nat_add(a2983, a2984)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f542(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f557(v1, v0, v2)
}

pub fn f543(v0: u64, v1: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
    let a2986 = {
        let (_, h2985) = v1;
        h2985
    };
    let a2987 = 1u64;
    if nat_eq(a2986, a2987) {
        Ok(None::<(List<Adt7>, u64)>)
    } else {
        let m2989 = {
            let (h2988, _) = v1;
            h2988
        };
        match m2989.uncons() {
            None => {
                Ok(None::<(List<Adt7>, u64)>)
            }
            Some((v2, v3)) => {
                let m2990 = v2;
                match m2990 {
                    Adt7::C0(_, _, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C1(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C2(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C3(r2991) => {
                        let v4 = (*r2991).clone();
                        Ok(Some((f357(Fn57::F558, v3, f360(v4, v0)?), 0u64)))
                    }
                    Adt7::C4 => {
                        Ok(Some((v3, 0u64)))
                    }
                    Adt7::C5(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C6(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C7(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C8(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C9(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C10(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C11(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C12(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C13 => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C14(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                }
            }
        }
    }
}

pub fn f544(v0: u64, v1: Adt56) -> R<Option<Adt56>> {
    let m2993 = match v1.clone() {
        Adt56::C0(_, _, h2992) => {
            h2992
        }
    };
    match m2993 {
        None => {
            let m2995 = match v1.clone() {
                Adt56::C0(h2994, _, _) => {
                    h2994
                }
            };
            match m2995.uncons() {
                None => {
                    Ok(None::<Adt56>)
                }
                Some((v3, v4)) => {
                    let m2996 = v3;
                    match m2996 {
                        Adt57::C0(r2997) => {
                            let v5 = (*r2997).clone();
                            Ok(Some(f559(v0, v1, v4, v5)?))
                        }
                        Adt57::C1(v6, v7) => {
                            Ok(Some(f560(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt56>)
        }
    }
}

pub fn f545(v0: Fn84, p3001: Adt2, p3002: List<Adt2>) -> Adt2 {
    let m3000 = (p3001, p3002);
    {
        let mut m3000 = m3000;
        loop {
            let step = {
                let (v1, v2) = m3000;
                let m2998 = v2;
                match m2998.uncons() {
                    None => {
                        Err::<(Adt2, List<Adt2>), Adt2>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt2, List<Adt2>), Adt2>(({
                            let c2999: Fn84 = v0.clone();
                            c2999.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: Adt2, v1: Adt2) -> Adt2 {
    Adt2::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f547(v0: Fn85, p3006: List<Adt2>, p3007: List<Adt2>) -> List<Adt2> {
    let m3005 = (p3006, p3007);
    {
        let mut m3005 = m3005;
        loop {
            let step = {
                let (v1, v2) = m3005;
                let m3003 = v2;
                match m3003.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c3004: Fn85 = v0.clone();
                            c3004.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f549(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f561(v1, v0, v2)
}

pub fn f550(v0: u32) -> u64 {
    let m3009 = {
        let a3008 = v0;
        fixed_u64::convert(i128::from(a3008))
    };
    match m3009 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f551(v0: u64) -> u32 {
    let m3013 = {
        let a3012 = {
            let a3010 = v0;
            let a3011 = 4294967295u64;
            fixed_u64::bit_and(a3010, a3011)
        };
        fixed_u32::convert(i128::from(a3012))
    };
    match m3013 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f552(v0: Bytes, v1: u64, v2: u64, v3: Adt51) -> R<Option<Adt51>> {
    let v4: u64 = match v3.clone() {
        Adt51::C0(h3014, _, _) => {
            h3014
        }
    };
    let a3015 = v4;
    let a3016 = v1;
    if nat_lt(a3015, a3016) {
        let v5: u8 = f506(v0, v4);
        let v6: bool = f562(v2, v5);
        if v6 {
            let a3017 = v5;
            let a3018 = 10u8;
            if equal(a3017, a3018) {
                Ok(Some(Adt51::C0({
                    let a3019 = v4;
                    let a3020 = 1u64;
                    nat_add(a3019, a3020)?
                }, {
                    let a3022 = match v3 {
                        Adt51::C0(_, h3021, _) => {
                            h3021
                        }
                    };
                    let a3023 = 1u64;
                    nat_add(a3022, a3023)?
                }, 1u64)))
            } else {
                Ok(Some(Adt51::C0({
                    let a3024 = v4;
                    let a3025 = 1u64;
                    nat_add(a3024, a3025)?
                }, match v3.clone() {
                    Adt51::C0(_, h3026, _) => {
                        h3026
                    }
                }, {
                    let a3028 = match v3 {
                        Adt51::C0(_, _, h3027) => {
                            h3027
                        }
                    };
                    let a3029 = 1u64;
                    nat_add(a3028, a3029)?
                })))
            }
        } else {
            Ok(None::<Adt51>)
        }
    } else {
        Ok(None::<Adt51>)
    }
}

pub fn f553(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f563(v0, v1)
}

pub fn f554(v0: u64) -> bool {
    let a3030 = v0;
    let a3031 = 172u64;
    if if if if if nat_eq(a3030, a3031) {
        true
    } else {
        let a3032 = v0;
        let a3033 = 177u64;
        nat_eq(a3032, a3033)
    } {
        true
    } else {
        let a3034 = v0;
        let a3035 = 183u64;
        if nat_eq(a3034, a3035) {
            true
        } else {
            let a3036 = v0;
            let a3037 = 215u64;
            nat_eq(a3036, a3037)
        }
    } {
        true
    } else {
        let a3038 = v0;
        let a3039 = 247u64;
        if if nat_eq(a3038, a3039) {
            true
        } else {
            if f564(v0, 913u64, 937u64) {
                let a3040 = v0;
                let a3041 = 930u64;
                !nat_eq(a3040, a3041)
            } else {
                false
            }
        } {
            true
        } else {
            if f564(v0, 945u64, 969u64) {
                true
            } else {
                let a3042 = v0;
                let a3043 = 8230u64;
                nat_eq(a3042, a3043)
            }
        }
    } {
        true
    } else {
        if if if f564(v0, 8242u64, 8244u64) {
            true
        } else {
            f564(v0, 8448u64, 8527u64)
        } {
            true
        } else {
            if f564(v0, 8592u64, 8703u64) {
                true
            } else {
                f564(v0, 8704u64, 8959u64)
            }
        } {
            true
        } else {
            if if f564(v0, 8968u64, 8971u64) {
                true
            } else {
                f564(v0, 10176u64, 10223u64)
            } {
                true
            } else {
                if f564(v0, 10624u64, 10751u64) {
                    true
                } else {
                    f564(v0, 10752u64, 11007u64)
                }
            }
        }
    } {
        true
    } else {
        f564(v0, 119808u64, 120831u64)
    }
}

pub fn f555(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f264(v0.clone(), v1)?;
    let v3: u64 = f556(v2);
    let a3044 = v3;
    let a3045 = 1u64;
    if nat_eq(a3044, a3045) {
        Ok(v2)
    } else {
        let a3046 = v3;
        let a3047 = 2u64;
        if nat_eq(a3046, a3047) {
            let a3055 = {
                let a3051 = {
                    let a3048 = v2;
                    let a3049 = 32u64;
                    let a3050 = 0u64;
                    nat_rem(a3048, a3049, a3050)
                };
                let a3052 = 64u64;
                nat_mul(a3051, a3052)?
            };
            let a3056 = f565(v0, {
                let a3053 = v1;
                let a3054 = 1u64;
                nat_add(a3053, a3054)?
            })?;
            nat_add(a3055, a3056)
        } else {
            let a3057 = v3;
            let a3058 = 3u64;
            if nat_eq(a3057, a3058) {
                let a3072 = {
                    let a3068 = {
                        let a3062 = {
                            let a3059 = v2;
                            let a3060 = 16u64;
                            let a3061 = 0u64;
                            nat_rem(a3059, a3060, a3061)
                        };
                        let a3063 = 4096u64;
                        nat_mul(a3062, a3063)?
                    };
                    let a3069 = {
                        let a3066 = f565(v0.clone(), {
                            let a3064 = v1;
                            let a3065 = 1u64;
                            nat_add(a3064, a3065)?
                        })?;
                        let a3067 = 64u64;
                        nat_mul(a3066, a3067)?
                    };
                    nat_add(a3068, a3069)?
                };
                let a3073 = f565(v0, {
                    let a3070 = v1;
                    let a3071 = 2u64;
                    nat_add(a3070, a3071)?
                })?;
                nat_add(a3072, a3073)
            } else {
                let a3093 = {
                    let a3089 = {
                        let a3083 = {
                            let a3077 = {
                                let a3074 = v2;
                                let a3075 = 8u64;
                                let a3076 = 0u64;
                                nat_rem(a3074, a3075, a3076)
                            };
                            let a3078 = 262144u64;
                            nat_mul(a3077, a3078)?
                        };
                        let a3084 = {
                            let a3081 = f565(v0.clone(), {
                                let a3079 = v1;
                                let a3080 = 1u64;
                                nat_add(a3079, a3080)?
                            })?;
                            let a3082 = 4096u64;
                            nat_mul(a3081, a3082)?
                        };
                        nat_add(a3083, a3084)?
                    };
                    let a3090 = {
                        let a3087 = f565(v0.clone(), {
                            let a3085 = v1;
                            let a3086 = 2u64;
                            nat_add(a3085, a3086)?
                        })?;
                        let a3088 = 64u64;
                        nat_mul(a3087, a3088)?
                    };
                    nat_add(a3089, a3090)?
                };
                let a3094 = f565(v0, {
                    let a3091 = v1;
                    let a3092 = 3u64;
                    nat_add(a3091, a3092)?
                })?;
                nat_add(a3093, a3094)
            }
        }
    }
}

pub fn f556(v0: u64) -> u64 {
    let a3095 = v0;
    let a3096 = 128u64;
    if nat_lt(a3095, a3096) {
        1u64
    } else {
        let a3097 = v0;
        let a3098 = 224u64;
        if nat_lt(a3097, a3098) {
            2u64
        } else {
            let a3099 = v0;
            let a3100 = 240u64;
            if nat_lt(a3099, a3100) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f557(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3101 = v2;
    let a3102 = v1;
    if nat_lt(a3101, a3102) {
        let v3: u64 = f264(v0, v2)?;
        if if f366(v3) {
            true
        } else {
            let a3103 = v3;
            let a3104 = 95u64;
            nat_eq(a3103, a3104)
        } {
            Ok(Some({
                let a3105 = v2;
                let a3106 = 1u64;
                nat_add(a3105, a3106)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f558(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f559(v0: u64, v1: Adt56, v2: List<Adt57>, v3: Adt35) -> R<Adt56> {
    let m3107 = v3;
    match m3107 {
        Adt35::C0(v4) => {
            if f530(v4.clone())? {
                let m3108 = f501(v4.clone())?;
                match m3108 {
                    None => {
                        Ok(Adt56::C0(v2, List::cons(Adt2::C0(f519(v4)?), match v1 {
                            Adt56::C0(_, h3109, _) => {
                                h3109
                            }
                        }), None::<Adt28>))
                    }
                    Some(v5) => {
                        Ok(f566(v1, v5))
                    }
                }
            } else {
                if f193(v4.clone())? {
                    Ok(Adt56::C0(v2, List::cons(Adt2::C1(v4), match v1 {
                        Adt56::C0(_, h3110, _) => {
                            h3110
                        }
                    }), None::<Adt28>))
                } else {
                    Ok(f566(v1, f200(f205(f301(v4), f201(Str::lit(" is not a universe"))))))
                }
            }
        }
        Adt35::C1(r3133) => {
            let v6 = (*r3133).clone();
            let v7: List<Adt35> = f378(v6, v0)?;
            let m3111 = v7;
            match m3111.uncons() {
                None => {
                    Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                }
                Some((v8, v9)) => {
                    let m3112 = v8;
                    match m3112 {
                        Adt35::C0(v10) => {
                            let v11: u64 = {
                                let a3113 = v9.clone();
                                length_list(a3113)
                            };
                            let v12: bool = {
                                let a3114 = v10.clone();
                                let a3115 = f201(Str::lit("succ"));
                                if equal(a3114, a3115) {
                                    true
                                } else {
                                    let a3116 = v10.clone();
                                    let a3117 = f201(Str::lit("max"));
                                    if equal(a3116, a3117) {
                                        true
                                    } else {
                                        let a3118 = v10.clone();
                                        let a3119 = f201(Str::lit("imax"));
                                        equal(a3118, a3119)
                                    }
                                }
                            };
                            let v13: bool = {
                                let a3120 = v10.clone();
                                let a3121 = f201(Str::lit("succ"));
                                if equal(a3120, a3121) {
                                    let a3122 = v11;
                                    let a3123 = 1u64;
                                    nat_eq(a3122, a3123)
                                } else {
                                    let a3124 = v10.clone();
                                    let a3125 = f201(Str::lit("max"));
                                    if equal(a3124, a3125) {
                                        let a3126 = 2u64;
                                        let a3127 = v11;
                                        nat_le(a3126, a3127)
                                    } else {
                                        let a3128 = v10.clone();
                                        let a3129 = f201(Str::lit("imax"));
                                        if equal(a3128, a3129) {
                                            let a3130 = v11;
                                            let a3131 = 2u64;
                                            nat_eq(a3130, a3131)
                                        } else {
                                            false
                                        }
                                    }
                                }
                            };
                            if v13 {
                                Ok(Adt56::C0(f567(v9, List::cons(Adt57::C1(v10, v11), v2)), match v1 {
                                    Adt56::C0(_, h3132, _) => {
                                        h3132
                                    }
                                }, None::<Adt28>))
                            } else {
                                if v12 {
                                    Ok(f566(v1, f200(f205(f205(f201(Str::lit("`(")), v10), f201(Str::lit(" ...)` has the wrong arity"))))))
                                } else {
                                    Ok(f566(v1, f200(f205(f301(v10), f201(Str::lit(" is not a universe form"))))))
                                }
                            }
                        }
                        Adt35::C1(_) => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                        Adt35::C2 => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
        }
        Adt35::C3(_, _) => {
            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
        }
    }
}

pub fn f560(v0: Adt56, v1: List<Adt57>, v2: Bytes, v3: u64) -> Adt56 {
    let v4: (List<Adt2>, List<Adt2>) = f568(match v0 {
        Adt56::C0(_, h3134, _) => {
            h3134
        }
    }, v3);
    let v5: List<Adt2> = {
        let (h3135, _) = v4;
        h3135
    };
    let v12: Adt2 = {
        let a3136 = v2.clone();
        let a3137 = f201(Str::lit("succ"));
        if equal(a3136, a3137) {
            let m3138 = v5;
            match m3138.uncons() {
                None => {
                    Adt2::C5
                }
                Some((v6, _)) => {
                    Adt2::C2(std::rc::Rc::new(v6))
                }
            }
        } else {
            let a3139 = v2;
            let a3140 = f201(Str::lit("imax"));
            if equal(a3139, a3140) {
                let m3141 = v5;
                match m3141.uncons() {
                    None => {
                        Adt2::C5
                    }
                    Some((v8, v9)) => {
                        let m3142 = v9;
                        match m3142.uncons() {
                            None => {
                                Adt2::C5
                            }
                            Some((v10, _)) => {
                                Adt2::C4(std::rc::Rc::new(v8), std::rc::Rc::new(v10))
                            }
                        }
                    }
                }
            } else {
                Adt2::C3(std::rc::Rc::new(f528(v5)))
            }
        }
    };
    Adt56::C0(v1, List::cons(v12, {
        let (_, h3143) = v4;
        h3143
    }), None::<Adt28>)
}

pub fn f561(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3144 = v2;
    let a3145 = v1;
    if nat_lt(a3144, a3145) {
        if f321(f264(v0, v2)?) {
            Ok(Some({
                let a3146 = v2;
                let a3147 = 1u64;
                nat_add(a3146, a3147)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f562(v0: u64, v1: u8) -> bool {
    let v2: bool = {
        let a3148 = v1;
        let a3149 = 32u8;
        equal(a3148, a3149)
    };
    let v3: bool = {
        let a3150 = v1;
        let a3151 = 10u8;
        equal(a3150, a3151)
    };
    let v4: bool = f509(v1);
    let v5: bool = f511(v1);
    let a3152 = v0;
    let a3153 = 0u64;
    if nat_eq(a3152, a3153) {
        if v2 {
            true
        } else {
            v3
        }
    } else {
        let a3154 = v0;
        let a3155 = 1u64;
        if nat_eq(a3154, a3155) {
            v4
        } else {
            v5
        }
    }
}

pub fn f563(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f536(v0, v1, 48u8) {
        Ok(Some({
            let a3156 = v1;
            let a3157 = 1u64;
            nat_add(a3156, a3157)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f564(v0: u64, v1: u64, v2: u64) -> bool {
    let a3158 = v0;
    let a3159 = v1;
    if nat_lt(a3158, a3159) {
        false
    } else {
        let a3160 = v0;
        let a3161 = v2;
        nat_le(a3160, a3161)
    }
}

pub fn f565(v0: Bytes, v1: u64) -> R<u64> {
    let a3162 = f264(v0, v1)?;
    let a3163 = 64u64;
    let a3164 = 0u64;
    Ok(nat_rem(a3162, a3163, a3164))
}

pub fn f566(v0: Adt56, v1: Adt28) -> Adt56 {
    Adt56::C0(match v0.clone() {
        Adt56::C0(h3165, _, _) => {
            h3165
        }
    }, match v0 {
        Adt56::C0(_, h3166, _) => {
            h3166
        }
    }, Some(v1))
}

pub fn f567(v0: List<Adt35>, v1: List<Adt57>) -> List<Adt57> {
    f569(Fn86::F570, v1, f424(v0))
}

pub fn f568(v0: List<Adt2>, v1: u64) -> (List<Adt2>, List<Adt2>) {
    let (h3167, _) = f571(Fn87::F572, v1, (List::<Adt2>::nil(), v0));
    h3167
}

pub fn f569(v0: Fn86, p3171: List<Adt57>, p3172: List<Adt35>) -> List<Adt57> {
    let m3170 = (p3171, p3172);
    {
        let mut m3170 = m3170;
        loop {
            let step = {
                let (v1, v2) = m3170;
                let m3168 = v2;
                match m3168.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt35>), List<Adt57>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt57>, List<Adt35>), List<Adt57>>(({
                            let c3169: Fn86 = v0.clone();
                            c3169.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: List<Adt57>, v1: Adt35) -> List<Adt57> {
    List::cons(Adt57::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f571(v0: Fn87, p3177: u64, p3178: (List<Adt2>, List<Adt2>)) -> ((List<Adt2>, List<Adt2>), bool) {
    let m3176 = (p3177, p3178);
    {
        let mut m3176 = m3176;
        loop {
            let step = {
                let (v1, v2) = m3176;
                let m3173 = v1;
                if m3173 == 0 {
                    Err::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v2, false))
                } else {
                    let v3 = m3173 - 1;
                    let m3175 = {
                        let c3174: Fn87 = v0.clone();
                        c3174.apply(v2.clone())
                    };
                    match m3175 {
                        None => {
                            Err::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: (List<Adt2>, List<Adt2>)) -> Option<(List<Adt2>, List<Adt2>)> {
    let m3180 = {
        let (_, h3179) = v0;
        h3179
    };
    match m3180.uncons() {
        None => {
            None::<(List<Adt2>, List<Adt2>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h3181, _) = v0;
                h3181
            }), v2))
        }
    }
}
