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
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>, List<Adt1>, List<Adt2>, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes, Bytes, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>),
    C5,
    C6(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C7(Bytes, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Fn0 {
    F18,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3, p1: Bytes) -> Adt3 {
        match self {
            Fn0::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt3, p1: Adt1) -> Adt3 {
        match self {
            Fn1::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F24,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt3, p1: Adt2) -> Adt3 {
        match self {
            Fn2::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F29(Bytes),
    F67(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F29(k0) => f29(k0.clone(), p0),
            Fn3::F67(k0, k1) => Ok(f67(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F32,
    F113,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F32 => f32(p0, p1),
            Fn4::F113 => f113(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F35,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
    F102,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F37 => f37(p0, p1),
            Fn6::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F40,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn7::F40 => f40(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F43,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn8::F43 => f43(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F45,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: (Bytes, Adt2)) -> List<Adt2> {
        match self {
            Fn9::F45 => f45(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F49,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: Adt2) -> R<List<(Bytes, Adt2)>> {
        match self {
            Fn10::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F54(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn11::F54(k0) => f54(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F59,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn12::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F62,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn13::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F64,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
        match self {
            Fn14::F64 => f64(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F71,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F73,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F73 => f73(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F75,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn17::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F84,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn18::F84 => f84(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F89,
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn19::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F93,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
        match self {
            Fn20::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F95(u8, Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn21::F95(k0, k1) => f95(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F99,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F99 => f99(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F108,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
        match self {
            Fn23::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F111,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F111 => f111(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Adt3) -> R<Bytes> {
    let a1 = f3(v0)?;
    let a2 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1, a2))
}

pub fn f2(v0: Adt0) -> R<Adt3> {
    Ok(Adt3::C4(std::rc::Rc::new(f4(Str::lit("spec"), f5(f6(Str::lit("lexlean/build-manifest/2"))), f4(Str::lit("compiler"), Adt3::C4(std::rc::Rc::new(f4(Str::lit("version"), f5(match v0.clone() {
        Adt0::C0(_, h3, _, _, _, _, _, _, _, _, _) => {
            h3
        }
    }), f4(Str::lit("semantics_id"), f7(match v0.clone() {
        Adt0::C0(_, _, h4, _, _, _, _, _, _, _, _) => {
            h4
        }
    }), Adt3::C5)))), f4(Str::lit("language"), f5(match v0.clone() {
        Adt0::C0(h5, _, _, _, _, _, _, _, _, _, _) => {
            h5
        }
    }), f4(Str::lit("project"), f5(match v0.clone() {
        Adt0::C0(_, _, _, h6, _, _, _, _, _, _, _) => {
            h6
        }
    }), f4(Str::lit("source_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, h7, _, _, _, _, _, _) => {
            h7
        }
    }), f4(Str::lit("semantic_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h8, _, _, _, _, _) => {
            h8
        }
    }), f4(Str::lit("build_id"), f7(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h9, _, _, _, _) => {
            h9
        }
    }), f4(Str::lit("lean_toolchain"), f5(f6(Str::lit("leanprover/lean4:v4.32.2"))), f4(Str::lit("selection"), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h10, _, _, _) => {
            h10
        }
    }), f4(Str::lit("modules"), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h11, _, _) => {
            h11
        }
    }), f4(Str::lit("inputs"), f10(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h12, _) => {
            h12
        }
    })?, f4(Str::lit("outputs"), f10(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h13) => {
            h13
        }
    })?, Adt3::C5)))))))))))))))
}

pub fn f3(v0: Adt3) -> R<Bytes> {
    let m14 = v0;
    match m14 {
        Adt3::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt3::C1(v2, v3) => {
            f11(v2, v3)
        }
        Adt3::C2(v4) => {
            f12(v4)
        }
        Adt3::C3(r15) => {
            let v5 = (*r15).clone();
            Ok(f13(f14(v5)?))
        }
        Adt3::C4(r16) => {
            let v6 = (*r16).clone();
            f15(f16(v6)?)
        }
        Adt3::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt3::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt3::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt3, v2: Adt3) -> Adt3 {
    Adt3::C7(f6(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f5(v0: Bytes) -> Adt3 {
    Adt3::C2(v0)
}

pub fn f6(v0: Str) -> Bytes {
    let a17 = v0;
    utf8_encode(a17)
}

pub fn f7(v0: Bytes) -> Adt3 {
    f5(v0)
}

pub fn f8(v0: List<Bytes>) -> Adt3 {
    Adt3::C3(std::rc::Rc::new(f17(Fn0::F18, Adt3::C5, f19(v0))))
}

pub fn f9(v0: List<Adt1>) -> Adt3 {
    Adt3::C3(std::rc::Rc::new(f20(Fn1::F21, Adt3::C5, f22(v0))))
}

pub fn f10(v0: List<Adt2>) -> R<Adt3> {
    Ok(Adt3::C3(std::rc::Rc::new(f23(Fn2::F24, Adt3::C5, f25(f26(v0)?)))))
}

pub fn f11(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a18 = 0u64;
        let a19 = v1;
        nat_lt(a18, a19)
    } else {
        false
    } {
        let a20 = Bytes::lit(&[0x2d]);
        let a21 = f27(v1)?;
        Ok(append_bytes(a20, a21))
    } else {
        f27(v1)
    }
}

pub fn f12(v0: Bytes) -> R<Bytes> {
    let a27 = {
        let a25 = Bytes::lit(&[0x22]);
        let a26 = {
            let (_, h22) = {
                let (h23, _) = f28(Fn3::F29(v0.clone()), {
                    let a24 = v0;
                    length_bytes(a24)
                }, (0u64, Bytes::lit(&[])))?;
                h23
            };
            h22
        };
        append_bytes(a25, a26)
    };
    let a28 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a27, a28))
}

pub fn f13(v0: List<Bytes>) -> Bytes {
    let a31 = {
        let a29 = Bytes::lit(&[0x5b]);
        let a30 = f30(v0, Bytes::lit(&[0x2c]));
        append_bytes(a29, a30)
    };
    let a32 = Bytes::lit(&[0x5d]);
    append_bytes(a31, a32)
}

pub fn f14(v0: Adt3) -> R<List<Bytes>> {
    let m33 = v0;
    match m33 {
        Adt3::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt3::C6(r34, r35) => {
            let v7 = (*r34).clone();
            let v8 = (*r35).clone();
            Ok(List::cons(f3(v7)?, f14(v8)?))
        }
        Adt3::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f15(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f31(Fn4::F32, List::<(Bytes, Bytes)>::nil(), f33(v0)?);
    let v2: List<Bytes> = f34(Fn5::F35, List::<Bytes>::nil(), v1)?;
    let a38 = {
        let a36 = Bytes::lit(&[0x7b]);
        let a37 = f30(v2, Bytes::lit(&[0x2c]));
        append_bytes(a36, a37)
    };
    let a39 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a38, a39))
}

pub fn f16(v0: Adt3) -> R<List<(Bytes, Bytes)>> {
    let m40 = v0;
    match m40 {
        Adt3::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt3::C7(v9, r41, r42) => {
            let v10 = (*r41).clone();
            let v11 = (*r42).clone();
            Ok(List::cons((v9, f3(v10)?), f16(v11)?))
        }
    }
}

pub fn f17(v0: Fn0, p46: Adt3, p47: List<Bytes>) -> Adt3 {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m43 = v2;
                match m43.uncons() {
                    None => {
                        Err::<(Adt3, List<Bytes>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Bytes>), Adt3>(({
                            let c44: Fn0 = v0.clone();
                            c44.apply(v1, v3)
                        }, v4))
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

pub fn f18(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C6(std::rc::Rc::new(f5(v1)), std::rc::Rc::new(v0))
}

pub fn f19(v0: List<Bytes>) -> List<Bytes> {
    f36(Fn6::F37, List::<Bytes>::nil(), v0)
}

pub fn f20(v0: Fn1, p51: Adt3, p52: List<Adt1>) -> Adt3 {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt1>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt1>), Adt3>(({
                            let c49: Fn1 = v0.clone();
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

pub fn f21(v0: Adt3, v1: Adt1) -> Adt3 {
    Adt3::C6(std::rc::Rc::new(f38(v1)), std::rc::Rc::new(v0))
}

pub fn f22(v0: List<Adt1>) -> List<Adt1> {
    f39(Fn7::F40, List::<Adt1>::nil(), v0)
}

pub fn f23(v0: Fn2, p56: Adt3, p57: List<Adt2>) -> Adt3 {
    let m55 = (p56, p57);
    {
        let mut m55 = m55;
        loop {
            let step = {
                let (v1, v2) = m55;
                let m53 = v2;
                match m53.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt2>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt2>), Adt3>(({
                            let c54: Fn2 = v0.clone();
                            c54.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m55 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Adt3, v1: Adt2) -> Adt3 {
    Adt3::C6(std::rc::Rc::new(f41(v1)), std::rc::Rc::new(v0))
}

pub fn f25(v0: List<Adt2>) -> List<Adt2> {
    f42(Fn8::F43, List::<Adt2>::nil(), v0)
}

pub fn f26(v0: List<Adt2>) -> R<List<Adt2>> {
    Ok(f44(Fn9::F45, List::<Adt2>::nil(), f46(f47(f46(f48(Fn10::F49, List::<(Bytes, Adt2)>::nil(), v0)?))?)))
}

pub fn f27(v0: u64) -> R<Bytes> {
    f50(f51(), 10u64, v0)
}

pub fn f28(v0: Fn3, p62: u64, p63: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v1, v2) = m61;
                let m58 = v1;
                if m58 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m58 - 1;
                    let m60 = {
                        let c59: Fn3 = v0.clone();
                        c59.apply(v2.clone())?
                    };
                    match m60 {
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
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f52(v0, v1)
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m64 = f53(Fn11::F54(v1), None::<Bytes>, v0);
    match m64 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f31(v0: Fn4, p68: List<(Bytes, Bytes)>, p69: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m67 = (p68, p69);
    {
        let mut m67 = m67;
        loop {
            let step = {
                let (v1, v2) = m67;
                let m65 = v2;
                match m65.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c66: Fn4 = v0.clone();
                            c66.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m67 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f55(v0, v1)
}

pub fn f33(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f56(v0)
}

pub fn f34(v0: Fn5, p73: List<Bytes>, p74: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m72 = (p73, p74);
    {
        let mut m72 = m72;
        loop {
            let step = {
                let (v1, v2) = m72;
                let m70 = v2;
                match m70.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c71: Fn5 = v0.clone();
                            c71.apply(v1, v3)?
                        }, v4))
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

pub fn f35(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f57(v1)?, v0))
}

pub fn f36(v0: Fn6, p78: List<Bytes>, p79: List<Bytes>) -> List<Bytes> {
    let m77 = (p78, p79);
    {
        let mut m77 = m77;
        loop {
            let step = {
                let (v1, v2) = m77;
                let m75 = v2;
                match m75.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c76: Fn6 = v0.clone();
                            c76.apply(v1, v3)
                        }, v4))
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

pub fn f37(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f38(v0: Adt1) -> Adt3 {
    let v2: Adt3 = {
        let m81 = match v0.clone() {
            Adt1::C0(_, _, _, h80) => {
                h80
            }
        };
        match m81 {
            None => {
                Adt3::C5
            }
            Some(v1) => {
                f4(Str::lit("origin"), f5(v1), Adt3::C5)
            }
        }
    };
    Adt3::C4(std::rc::Rc::new(f4(Str::lit("module"), f5(match v0.clone() {
        Adt1::C0(h82, _, _, _) => {
            h82
        }
    }), f4(Str::lit("lean_module"), f5(match v0.clone() {
        Adt1::C0(_, h83, _, _) => {
            h83
        }
    }), f4(Str::lit("source_path"), f5(match v0 {
        Adt1::C0(_, _, h84, _) => {
            h84
        }
    }), v2)))))
}

pub fn f39(v0: Fn7, p88: List<Adt1>, p89: List<Adt1>) -> List<Adt1> {
    let m87 = (p88, p89);
    {
        let mut m87 = m87;
        loop {
            let step = {
                let (v1, v2) = m87;
                let m85 = v2;
                match m85.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c86: Fn7 = v0.clone();
                            c86.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m87 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f41(v0: Adt2) -> Adt3 {
    Adt3::C4(std::rc::Rc::new(f4(Str::lit("kind"), f5(match v0.clone() {
        Adt2::C0(h90, _, _, _) => {
            h90
        }
    }), f4(Str::lit("path"), f5(match v0.clone() {
        Adt2::C0(_, h91, _, _) => {
            h91
        }
    }), f4(Str::lit("byte_length"), Adt3::C1(false, match v0.clone() {
        Adt2::C0(_, _, h92, _) => {
            h92
        }
    }), f4(Str::lit("kappa"), f7(match v0 {
        Adt2::C0(_, _, _, h93) => {
            h93
        }
    }), Adt3::C5))))))
}

pub fn f42(v0: Fn8, p97: List<Adt2>, p98: List<Adt2>) -> List<Adt2> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m94 = v2;
                match m94.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c95: Fn8 = v0.clone();
                            c95.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f44(v0: Fn9, p102: List<Adt2>, p103: List<(Bytes, Adt2)>) -> List<Adt2> {
    let m101 = (p102, p103);
    {
        let mut m101 = m101;
        loop {
            let step = {
                let (v1, v2) = m101;
                let m99 = v2;
                match m99.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(({
                            let c100: Fn9 = v0.clone();
                            c100.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: List<Adt2>, v1: (Bytes, Adt2)) -> List<Adt2> {
    List::cons({
        let (_, h104) = v1;
        h104
    }, v0)
}

pub fn f46(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f58(Fn12::F59, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f47(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v1: List<List<(Bytes, Adt2)>> = f60(f61(Fn13::F62, List::<List<(Bytes, Adt2)>>::nil(), v0.clone()));
    let m109 = {
        let (h105, _) = f63(Fn14::F64, {
            let a107 = {
                let a106 = v0;
                length_list(a106)
            };
            let a108 = 1u64;
            nat_add(a107, a108)?
        }, v1)?;
        h105
    };
    match m109.uncons() {
        None => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f48(v0: Fn10, p113: List<(Bytes, Adt2)>, p114: List<Adt2>) -> R<List<(Bytes, Adt2)>> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m110 = v2;
                match m110.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<Adt2>), R<List<(Bytes, Adt2)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<Adt2>), R<List<(Bytes, Adt2)>>>(({
                            let c111: Fn10 = v0.clone();
                            c111.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Bytes, Adt2)>, v1: Adt2) -> R<List<(Bytes, Adt2)>> {
    Ok(List::cons((f65(v1.clone())?, v1), v0))
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a115 = v2;
    let a116 = 0u64;
    if nat_eq(a115, a116) {
        Ok(f66(v0, 0u64))
    } else {
        let (_, h117) = {
            let (h118, _) = f28(Fn3::F67(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h118
        };
        Ok(h117)
    }
}

pub fn f51() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f52(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m122 = {
        let a120 = v0.clone();
        let a121 = {
            let (h119, _) = v1.clone();
            h119
        };
        index_bytes(a120, a121)
    };
    match m122 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a124 = {
                    let (h123, _) = v1.clone();
                    h123
                };
                let a125 = 1u64;
                nat_add(a124, a125)?
            }, {
                let a128 = {
                    let (_, h126) = v1;
                    h126
                };
                let a129 = f68(v0, {
                    let (h127, _) = v1;
                    h127
                }, v2)?;
                append_bytes(a128, a129)
            })))
        }
    }
}

pub fn f53(v0: Fn11, p133: Option<Bytes>, p134: List<Bytes>) -> Option<Bytes> {
    let m132 = (p133, p134);
    {
        let mut m132 = m132;
        loop {
            let step = {
                let (v1, v2) = m132;
                let m130 = v2;
                match m130.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c131: Fn11 = v0.clone();
                            c131.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m135 = v1;
    match m135 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a138 = {
                    let a136 = v3;
                    let a137 = v0;
                    append_bytes(a136, a137)
                };
                let a139 = v2;
                append_bytes(a138, a139)
            })
        }
    }
}

pub fn f55(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m140 = v0.clone();
    match m140.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a143 = {
                let (h141, _) = v2;
                h141
            };
            let a144 = {
                let (h142, _) = v1.clone();
                h142
            };
            if equal(a143, a144) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f56(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f69(f70(Fn15::F71, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m149 = {
        let (h145, _) = f72(Fn16::F73, {
            let a147 = {
                let a146 = v0;
                length_list(a146)
            };
            let a148 = 1u64;
            nat_add(a147, a148)?
        }, v1)?;
        h145
    };
    match m149.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f57(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a154 = {
        let a151 = f12({
            let (h150, _) = v0;
            h150
        })?;
        let a152 = Bytes::lit(&[0x3a]);
        append_bytes(a151, a152)
    };
    let a155 = {
        let (_, h153) = v0;
        h153
    };
    Ok(append_bytes(a154, a155))
}

pub fn f58(v0: Fn12, p159: List<(Bytes, Adt2)>, p160: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, v2) = m158;
                let m156 = v2;
                match m156.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c157: Fn12 = v0.clone();
                            c157.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f60(v0: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    f74(Fn17::F75, List::<List<(Bytes, Adt2)>>::nil(), v0)
}

pub fn f61(v0: Fn13, p164: List<List<(Bytes, Adt2)>>, p165: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v1, v2) = m163;
                let m161 = v2;
                match m161.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(({
                            let c162: Fn13 = v0.clone();
                            c162.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<List<(Bytes, Adt2)>>, v1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt2)>::nil()), v0)
}

pub fn f63(v0: Fn14, p170: u64, p171: List<List<(Bytes, Adt2)>>) -> R<(List<List<(Bytes, Adt2)>>, bool)> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v1, v2) = m169;
                let m166 = v1;
                if m166 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m166 - 1;
                    let m168 = {
                        let c167: Fn14 = v0.clone();
                        c167.apply(v2.clone())?
                    };
                    match m168 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>((v3, v4))
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

pub fn f64(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    f76(v0)
}

pub fn f65(v0: Adt2) -> R<Bytes> {
    Ok(f77(f77(f77(f78(match v0.clone() {
        Adt2::C0(h172, _, _, _) => {
            h172
        }
    })?, f78(match v0.clone() {
        Adt2::C0(_, h173, _, _) => {
            h173
        }
    })?), f79(match v0.clone() {
        Adt2::C0(_, _, h174, _) => {
            h174
        }
    }, 8u64)), match v0 {
        Adt2::C0(_, _, _, h175) => {
            h175
        }
    }))
}

pub fn f66(v0: Bytes, v1: u64) -> Bytes {
    let m179 = {
        let a176 = v0;
        let a177 = v1;
        let a178 = 1u64;
        slice_bytes(a176, a177, a178)
    };
    match m179 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f67(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f80(v1, v0, v2)
}

pub fn f68(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a224 = {
            let a200 = {
                let a188 = {
                    let a182 = {
                        let a180 = v3;
                        let a181 = 128u8;
                        fixed_u8::bit_and(a180, a181)
                    };
                    let a183 = 128u8;
                    if equal(a182, a183) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a189 = {
                    let a186 = {
                        let a184 = v3;
                        let a185 = 64u8;
                        fixed_u8::bit_and(a184, a185)
                    };
                    let a187 = 64u8;
                    if equal(a186, a187) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a188, a189)?
            };
            let a201 = {
                let a198 = {
                    let a192 = {
                        let a190 = v3;
                        let a191 = 32u8;
                        fixed_u8::bit_and(a190, a191)
                    };
                    let a193 = 32u8;
                    if equal(a192, a193) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a199 = {
                    let a196 = {
                        let a194 = v3;
                        let a195 = 16u8;
                        fixed_u8::bit_and(a194, a195)
                    };
                    let a197 = 16u8;
                    if equal(a196, a197) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a198, a199)?
            };
            nat_add(a200, a201)?
        };
        let a225 = {
            let a222 = {
                let a210 = {
                    let a204 = {
                        let a202 = v3;
                        let a203 = 8u8;
                        fixed_u8::bit_and(a202, a203)
                    };
                    let a205 = 8u8;
                    if equal(a204, a205) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a211 = {
                    let a208 = {
                        let a206 = v3;
                        let a207 = 4u8;
                        fixed_u8::bit_and(a206, a207)
                    };
                    let a209 = 4u8;
                    if equal(a208, a209) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a210, a211)?
            };
            let a223 = {
                let a220 = {
                    let a214 = {
                        let a212 = v3;
                        let a213 = 2u8;
                        fixed_u8::bit_and(a212, a213)
                    };
                    let a215 = 2u8;
                    if equal(a214, a215) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a221 = {
                    let a218 = {
                        let a216 = v3;
                        let a217 = 1u8;
                        fixed_u8::bit_and(a216, a217)
                    };
                    let a219 = 1u8;
                    if equal(a218, a219) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a220, a221)?
            };
            nat_add(a222, a223)?
        };
        nat_add(a224, a225)?
    };
    let a226 = v4;
    let a227 = 34u64;
    if nat_eq(a226, a227) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a228 = v4;
        let a229 = 92u64;
        if nat_eq(a228, a229) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a230 = v4;
            let a231 = 8u64;
            if nat_eq(a230, a231) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a232 = v4;
                let a233 = 12u64;
                if nat_eq(a232, a233) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a234 = v4;
                    let a235 = 10u64;
                    if nat_eq(a234, a235) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a236 = v4;
                        let a237 = 13u64;
                        if nat_eq(a236, a237) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a238 = v4;
                            let a239 = 9u64;
                            if nat_eq(a238, a239) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a240 = v4;
                                let a241 = 32u64;
                                if nat_lt(a240, a241) {
                                    let a242 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a243 = f81(v2)?;
                                    Ok(append_bytes(a242, a243))
                                } else {
                                    Ok(f82(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f69(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f83(Fn18::F84, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f70(v0: Fn15, p247: List<List<(Bytes, Bytes)>>, p248: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m246 = (p247, p248);
    {
        let mut m246 = m246;
        loop {
            let step = {
                let (v1, v2) = m246;
                let m244 = v2;
                match m244.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c245: Fn15 = v0.clone();
                            c245.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f72(v0: Fn16, p253: u64, p254: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m249 = v1;
                if m249 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m249 - 1;
                    let m251 = {
                        let c250: Fn16 = v0.clone();
                        c250.apply(v2.clone())?
                    };
                    match m251 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f85(v0)
}

pub fn f74(v0: Fn17, p258: List<List<(Bytes, Adt2)>>, p259: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m255 = v2;
                match m255.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(({
                            let c256: Fn17 = v0.clone();
                            c256.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<List<(Bytes, Adt2)>>, v1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    List::cons(v1, v0)
}

pub fn f76(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    let a261 = {
        let a260 = v0.clone();
        length_list(a260)
    };
    let a262 = 1u64;
    if nat_le(a261, a262) {
        Ok(None::<List<List<(Bytes, Adt2)>>>)
    } else {
        Ok(Some(f86(v0)?))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> Bytes {
    let a263 = v0;
    let a264 = v1;
    append_bytes(a263, a264)
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    Ok(f77(f30(f87(v0, 0u8)?, Bytes::lit(&[0x00, 0x01])), Bytes::lit(&[0x00, 0x00])))
}

pub fn f79(v0: u64, v1: u64) -> Bytes {
    let (_, h265) = f88(Fn19::F89, v1, (v0, Bytes::lit(&[])));
    h265
}

pub fn f80(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h266, _) = v2;
        h266
    };
    let a267 = v3;
    let a268 = 0u64;
    if nat_eq(a267, a268) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a269 = v3;
            let a270 = v1;
            let a271 = 0u64;
            nat_quot(a269, a270, a271)
        }, {
            let a276 = f66(v0, {
                let a272 = v3;
                let a273 = v1;
                let a274 = 0u64;
                nat_rem(a272, a273, a274)
            });
            let a277 = {
                let (_, h275) = v2;
                h275
            };
            append_bytes(a276, a277)
        }))
    }
}

pub fn f81(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a322 = {
            let a298 = {
                let a286 = {
                    let a280 = {
                        let a278 = v1;
                        let a279 = 128u8;
                        fixed_u8::bit_and(a278, a279)
                    };
                    let a281 = 128u8;
                    if equal(a280, a281) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a287 = {
                    let a284 = {
                        let a282 = v1;
                        let a283 = 64u8;
                        fixed_u8::bit_and(a282, a283)
                    };
                    let a285 = 64u8;
                    if equal(a284, a285) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a286, a287)?
            };
            let a299 = {
                let a296 = {
                    let a290 = {
                        let a288 = v1;
                        let a289 = 32u8;
                        fixed_u8::bit_and(a288, a289)
                    };
                    let a291 = 32u8;
                    if equal(a290, a291) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a297 = {
                    let a294 = {
                        let a292 = v1;
                        let a293 = 16u8;
                        fixed_u8::bit_and(a292, a293)
                    };
                    let a295 = 16u8;
                    if equal(a294, a295) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a296, a297)?
            };
            nat_add(a298, a299)?
        };
        let a323 = {
            let a320 = {
                let a308 = {
                    let a302 = {
                        let a300 = v1;
                        let a301 = 8u8;
                        fixed_u8::bit_and(a300, a301)
                    };
                    let a303 = 8u8;
                    if equal(a302, a303) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a309 = {
                    let a306 = {
                        let a304 = v1;
                        let a305 = 4u8;
                        fixed_u8::bit_and(a304, a305)
                    };
                    let a307 = 4u8;
                    if equal(a306, a307) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a308, a309)?
            };
            let a321 = {
                let a318 = {
                    let a312 = {
                        let a310 = v1;
                        let a311 = 2u8;
                        fixed_u8::bit_and(a310, a311)
                    };
                    let a313 = 2u8;
                    if equal(a312, a313) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a319 = {
                    let a316 = {
                        let a314 = v1;
                        let a315 = 1u8;
                        fixed_u8::bit_and(a314, a315)
                    };
                    let a317 = 1u8;
                    if equal(a316, a317) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a318, a319)?
            };
            nat_add(a320, a321)?
        };
        nat_add(a322, a323)?
    };
    let a330 = f66(f90(), {
        let a324 = v2;
        let a325 = 16u64;
        let a326 = 0u64;
        nat_quot(a324, a325, a326)
    });
    let a331 = f66(f90(), {
        let a327 = v2;
        let a328 = 16u64;
        let a329 = 0u64;
        nat_rem(a327, a328, a329)
    });
    Ok(append_bytes(a330, a331))
}

pub fn f82(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m335 = {
        let a332 = v0;
        let a333 = v1;
        let a334 = v2;
        slice_bytes(a332, a333, a334)
    };
    match m335 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f83(v0: Fn18, p339: List<List<(Bytes, Bytes)>>, p340: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v1, v2) = m338;
                let m336 = v2;
                match m336.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c337: Fn18 = v0.clone();
                            c337.apply(v1, v3)
                        }, v4))
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

pub fn f84(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f85(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a342 = {
        let a341 = v0.clone();
        length_list(a341)
    };
    let a343 = 1u64;
    if nat_le(a342, a343) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f91(v0)?))
    }
}

pub fn f86(v0: List<List<(Bytes, Adt2)>>) -> R<List<List<(Bytes, Adt2)>>> {
    Ok(f60({
        let (_, h344) = {
            let (h345, _) = f92(Fn20::F93, {
                let a347 = {
                    let a346 = v0.clone();
                    length_list(a346)
                };
                let a348 = 1u64;
                nat_add(a347, a348)?
            }, (v0, List::<List<(Bytes, Adt2)>>::nil()))?;
            h345
        };
        h344
    }))
}

pub fn f87(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a349 = v0.clone();
        length_bytes(a349)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h350, _) = f94(Fn21::F95(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h350
    };
    let v4: u64 = {
        let (h351, _) = {
            let (_, h352) = v3.clone();
            h352
        };
        h351
    };
    Ok(f96(List::cons(f82(v0, v4, {
        let a353 = v2;
        let a354 = v4;
        nat_sub(a353, a354)
    }), {
        let (_, h355) = {
            let (_, h356) = v3;
            h356
        };
        h355
    })))
}

pub fn f88(v0: Fn19, p360: u64, p361: (u64, Bytes)) -> (u64, Bytes) {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v1, v2) = m359;
                let m357 = v1;
                if m357 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m357 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c358: Fn19 = v0.clone();
                        c358.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a363 = {
            let (h362, _) = v0.clone();
            h362
        };
        let a364 = 256u64;
        let a365 = 0u64;
        nat_quot(a363, a364, a365)
    }, f77(f66(f97(), {
        let a367 = {
            let (h366, _) = v0;
            h366
        };
        let a368 = 256u64;
        let a369 = 0u64;
        nat_rem(a367, a368, a369)
    }), {
        let (_, h370) = v0;
        h370
    }))
}

pub fn f90() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f91(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f69({
        let (_, h371) = {
            let (h372, _) = f98(Fn22::F99, {
                let a374 = {
                    let a373 = v0.clone();
                    length_list(a373)
                };
                let a375 = 1u64;
                nat_add(a374, a375)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h372
        };
        h371
    }))
}

pub fn f92(v0: Fn20, p380: u64, p381: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)> {
    let m379 = (p380, p381);
    {
        let mut m379 = m379;
        loop {
            let step = {
                let (v1, v2) = m379;
                let m376 = v1;
                if m376 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m376 - 1;
                    let m378 = {
                        let c377: Fn20 = v0.clone();
                        c377.apply(v2.clone())?
                    };
                    match m378 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>((v3, v4))
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

pub fn f93(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    f100(v0)
}

pub fn f94(v0: Fn21, p386: u64, p387: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m382 = v1;
                if m382 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m382 - 1;
                    let m384 = {
                        let c383: Fn21 = v0.clone();
                        c383.apply(v2.clone())?
                    };
                    match m384 {
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
                Ok(next) => { m385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f101(v1, v0, v2)
}

pub fn f96(v0: List<Bytes>) -> List<Bytes> {
    f36(Fn6::F102, List::<Bytes>::nil(), v0)
}

pub fn f97() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f98(v0: Fn22, p392: u64, p393: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m391 = (p392, p393);
    {
        let mut m391 = m391;
        loop {
            let step = {
                let (v1, v2) = m391;
                let m388 = v1;
                if m388 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m388 - 1;
                    let m390 = {
                        let c389: Fn22 = v0.clone();
                        c389.apply(v2.clone())?
                    };
                    match m390 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f103(v0)
}

pub fn f100(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    let v1: List<List<(Bytes, Adt2)>> = {
        let (h394, _) = v0;
        h394
    };
    let v2: List<List<(Bytes, Adt2)>> = {
        let (_, h395) = v0;
        h395
    };
    let m396 = v1;
    match m396.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>)
        }
        Some((v3, v4)) => {
            let m397 = v4;
            match m397.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt2)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f104(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f101(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h398, _) = v2.clone();
        h398
    };
    let v4: u64 = {
        let (h399, _) = {
            let (_, h400) = v2.clone();
            h400
        };
        h399
    };
    let v5: List<Bytes> = {
        let (_, h401) = {
            let (_, h402) = v2;
            h402
        };
        h401
    };
    let m403 = f105(v0.clone(), v3);
    match m403 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a404 = v6;
            let a405 = v1;
            if equal(a404, a405) {
                Ok(Some(({
                    let a406 = v3;
                    let a407 = 1u64;
                    nat_add(a406, a407)?
                }, ({
                    let a408 = v3;
                    let a409 = 1u64;
                    nat_add(a408, a409)?
                }, List::cons(f82(v0, v4, {
                    let a410 = v3;
                    let a411 = v4;
                    nat_sub(a410, a411)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a412 = v3;
                    let a413 = 1u64;
                    nat_add(a412, a413)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f102(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f103(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h414, _) = v0;
        h414
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h415) = v0;
        h415
    };
    let m416 = v1;
    match m416.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m417 = v4;
            match m417.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f106(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f104(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v2: u64 = {
        let a422 = {
            let a420 = {
                let a418 = v0.clone();
                length_list(a418)
            };
            let a421 = {
                let a419 = v1.clone();
                length_list(a419)
            };
            nat_add(a420, a421)?
        };
        let a423 = 1u64;
        nat_add(a422, a423)?
    };
    Ok(f46({
        let (_, h424) = {
            let (_, h425) = {
                let (h426, _) = f107(Fn23::F108, v2, (v0, (v1, List::<(Bytes, Adt2)>::nil())));
                h426
            };
            h425
        };
        h424
    }))
}

pub fn f105(v0: Bytes, v1: u64) -> Option<u8> {
    let a427 = v0;
    let a428 = v1;
    index_bytes(a427, a428)
}

pub fn f106(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a433 = {
            let a431 = {
                let a429 = v0.clone();
                length_list(a429)
            };
            let a432 = {
                let a430 = v1.clone();
                length_list(a430)
            };
            nat_add(a431, a432)?
        };
        let a434 = 1u64;
        nat_add(a433, a434)?
    };
    Ok(f109({
        let (_, h435) = {
            let (_, h436) = {
                let (h437, _) = f110(Fn24::F111, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h437
            };
            h436
        };
        h435
    }))
}

pub fn f107(v0: Fn23, p442: u64, p443: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool) {
    let m441 = (p442, p443);
    {
        let mut m441 = m441;
        loop {
            let step = {
                let (v1, v2) = m441;
                let m438 = v1;
                if m438 == 0 {
                    Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, false))
                } else {
                    let v3 = m438 - 1;
                    let m440 = {
                        let c439: Fn23 = v0.clone();
                        c439.apply(v2.clone())
                    };
                    match m440 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    f112(v0)
}

pub fn f109(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f31(Fn4::F113, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f110(v0: Fn24, p448: u64, p449: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m447 = (p448, p449);
    {
        let mut m447 = m447;
        loop {
            let step = {
                let (v1, v2) = m447;
                let m444 = v1;
                if m444 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m444 - 1;
                    let m446 = {
                        let c445: Fn24 = v0.clone();
                        c445.apply(v2.clone())
                    };
                    match m446 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f114(v0)
}

pub fn f112(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    let v1: List<(Bytes, Adt2)> = {
        let (h450, _) = v0.clone();
        h450
    };
    let v2: List<(Bytes, Adt2)> = {
        let (h451, _) = {
            let (_, h452) = v0.clone();
            h452
        };
        h451
    };
    let v3: List<(Bytes, Adt2)> = {
        let (_, h453) = {
            let (_, h454) = v0;
            h454
        };
        h453
    };
    let m455 = v1.clone();
    match m455.uncons() {
        None => {
            let m456 = v2;
            match m456.uncons() {
                None => {
                    None::<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt2)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m457 = v2.clone();
            match m457.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt2)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f115({
                        let (h458, _) = v8.clone();
                        h458
                    }, {
                        let (h459, _) = v6.clone();
                        h459
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f113(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f114(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h460, _) = v0.clone();
        h460
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h461, _) = {
            let (_, h462) = v0.clone();
            h462
        };
        h461
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h463) = {
            let (_, h464) = v0;
            h464
        };
        h463
    };
    let m465 = v1.clone();
    match m465.uncons() {
        None => {
            let m466 = v2;
            match m466.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m467 = v2.clone();
            match m467.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f115({
                        let (h468, _) = v8.clone();
                        h468
                    }, {
                        let (h469, _) = v6.clone();
                        h469
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: Bytes) -> bool {
    let a474 = {
        let a470 = v0;
        let a471 = v1;
        compare_bytes(a470, a471)
    };
    let a475 = {
        let a472 = Bytes::lit(&[]);
        let a473 = Bytes::lit(&[0x01]);
        compare_bytes(a472, a473)
    };
    equal(a474, a475)
}
