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
    C0(Bytes, Bytes, Bytes, List<Bytes>, List<Bytes>, Bytes, Bytes, Bytes, List<Adt1>, List<Adt2>, List<Adt3>, Adt4, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes),
    C1(Bytes, Bytes),
    C2(Bytes, Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Bytes, List<Bytes>, Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Adt6, u64, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F10,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: Adt2) -> R<Bytes> {
        match self {
            Fn0::F10 => f10(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes, p1: Adt3) -> R<Bytes> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F27,
}

impl Fn2 {
    pub fn apply(&self, p0: Bytes, p1: Adt1) -> R<Bytes> {
        match self {
            Fn2::F27 => f27(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F33,
}

impl Fn3 {
    pub fn apply(&self, p0: (List<Adt6>, Adt6)) -> (List<Adt6>, Adt6) {
        match self {
            Fn3::F33 => f33(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F40,
    F66,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn4::F40 => f40(p0, p1),
            Fn4::F66 => Ok(f66(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F45,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn5::F45 => f45(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F49,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: Adt1) -> List<(Bytes, Adt1)> {
        match self {
            Fn6::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F52(Bytes, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, List<Adt6>)) -> R<(u64, List<Adt6>)> {
        match self {
            Fn7::F52(k0, k1) => f52(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F63(Bytes),
    F92(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn8::F63(k0) => f63(k0.clone(), p0),
            Fn8::F92(k0, k1) => Ok(f92(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F65(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn9::F65(k0) => f65(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F68,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn10::F68 => f68(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F71,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn11::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F73,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn12::F73 => f73(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F79,
}

impl Fn13 {
    pub fn apply(&self, p0: (Adt8, Adt7)) -> (Adt8, Adt7) {
        match self {
            Fn13::F79 => f79(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F81(u64, Bytes, u64),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Adt6)) -> R<(u64, Adt6)> {
        match self {
            Fn14::F81(k0, k1, k2) => f81(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F85,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F85 => f85(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F89,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn16::F89 => f89(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F94,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt9) -> Adt9 {
        match self {
            Fn17::F94 => f94(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F110,
}

impl Fn18 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn18::F110 => f110(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F119,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn19::F119 => f119(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let a2 = {
        let a1 = Str::lit("blake3:");
        utf8_encode(a1)
    };
    let a3 = f3(v0)?;
    Ok(append_bytes(a2, a3))
}

pub fn f2(v0: Adt0) -> R<Bytes> {
    let v1: Bytes = f4(f4(f4(f4(f4(f4(f4(f4(f4(f5(Str::lit("spec"), f6(Str::lit("lexlean/project/2")))?, f5(Str::lit("name"), match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4
        }
    })?), f5(Str::lit("language"), match v0.clone() {
        Adt0::C0(_, h5, _, _, _, _, _, _, _, _, _, _, _) => {
            h5
        }
    })?), f5(Str::lit("module_prefix"), match v0.clone() {
        Adt0::C0(_, _, h6, _, _, _, _, _, _, _, _, _, _) => {
            h6
        }
    })?), f7(Str::lit("source_roots"), match v0.clone() {
        Adt0::C0(_, _, _, h7, _, _, _, _, _, _, _, _, _) => {
            h7
        }
    })?), f7(Str::lit("entrypoints"), match v0.clone() {
        Adt0::C0(_, _, _, _, h8, _, _, _, _, _, _, _, _) => {
            h8
        }
    })?), f5(Str::lit("build_root"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, h9, _, _, _, _, _, _, _) => {
            h9
        }
    })?), f5(Str::lit("lockfile"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h10, _, _, _, _, _, _) => {
            h10
        }
    })?), f5(Str::lit("lean_workspace"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h11, _, _, _, _, _) => {
            h11
        }
    })?), f5(Str::lit("lean_toolchain"), f8())?);
    let v2: Bytes = f9(Fn0::F10, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h12, _, _, _) => {
            h12
        }
    })?;
    let v3: Bytes = f11(Fn1::F12, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h13, _, _) => {
            h13
        }
    })?;
    let v5: Bytes = {
        let m15 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h14) => {
                h14
            }
        };
        match m15 {
            None => {
                Bytes::lit(&[])
            }
            Some(v4) => {
                f13(v4)?
            }
        }
    };
    Ok(f4(f4(f4(f4(f4(v1, f14(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h16, _, _, _, _) => {
            h16
        }
    })?), v2), v3), f15(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h17, _) => {
            h17
        }
    })?), v5))
}

pub fn f3(v0: Bytes) -> R<Bytes> {
    let v1: Adt6 = f16(v0)?;
    let a38 = f17(match v1.clone() {
        Adt6::C0(h18, _, _, _, _, _, _, _) => {
            h18
        }
    });
    let a39 = {
        let a36 = f17(match v1.clone() {
            Adt6::C0(_, h19, _, _, _, _, _, _) => {
                h19
            }
        });
        let a37 = {
            let a34 = f17(match v1.clone() {
                Adt6::C0(_, _, h20, _, _, _, _, _) => {
                    h20
                }
            });
            let a35 = {
                let a32 = f17(match v1.clone() {
                    Adt6::C0(_, _, _, h21, _, _, _, _) => {
                        h21
                    }
                });
                let a33 = {
                    let a30 = f17(match v1.clone() {
                        Adt6::C0(_, _, _, _, h22, _, _, _) => {
                            h22
                        }
                    });
                    let a31 = {
                        let a28 = f17(match v1.clone() {
                            Adt6::C0(_, _, _, _, _, h23, _, _) => {
                                h23
                            }
                        });
                        let a29 = {
                            let a26 = f17(match v1.clone() {
                                Adt6::C0(_, _, _, _, _, _, h24, _) => {
                                    h24
                                }
                            });
                            let a27 = f17(match v1 {
                                Adt6::C0(_, _, _, _, _, _, _, h25) => {
                                    h25
                                }
                            });
                            append_bytes(a26, a27)
                        };
                        append_bytes(a28, a29)
                    };
                    append_bytes(a30, a31)
                };
                append_bytes(a32, a33)
            };
            append_bytes(a34, a35)
        };
        append_bytes(a36, a37)
    };
    Ok(append_bytes(a38, a39))
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a40 = v0;
    let a41 = v1;
    append_bytes(a40, a41)
}

pub fn f5(v0: Str, v1: Bytes) -> R<Bytes> {
    f18(f6(v0), v1)
}

pub fn f6(v0: Str) -> Bytes {
    let a42 = v0;
    utf8_encode(a42)
}

pub fn f7(v0: Str, v1: List<Bytes>) -> R<Bytes> {
    Ok(f19(f6(v0), f20(v1)?))
}

pub fn f8() -> Bytes {
    f21(Str::lit("leanprover/lean4:v4.32.2"))
}

pub fn f9(v0: Fn0, p46: Bytes, p47: List<Adt2>) -> R<Bytes> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m43 = v2;
                match m43.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt2>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt2>), R<Bytes>>(({
                            let c44: Fn0 = v0.clone();
                            c44.apply(v1, v3)?
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

pub fn f10(v0: Bytes, v1: Adt2) -> R<Bytes> {
    Ok(f4(v0, f22(v1)?))
}

pub fn f11(v0: Fn1, p51: Bytes, p52: List<Adt3>) -> R<Bytes> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt3>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt3>), R<Bytes>>(({
                            let c49: Fn1 = v0.clone();
                            c49.apply(v1, v3)?
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

pub fn f12(v0: Bytes, v1: Adt3) -> R<Bytes> {
    Ok(f4(v0, f23(v1)?))
}

pub fn f13(v0: Adt5) -> R<Bytes> {
    Ok(f4(f4(f4(f4(f4(f4(f4(f4(f24(Str::lit("[pdf]")), f5(Str::lit("mode"), f6(Str::lit("external")))?), f5(Str::lit("program"), match v0.clone() {
        Adt5::C0(h53, _, _, _, _, _, _) => {
            h53
        }
    })?), f25(Str::lit("program_kappa"), match v0.clone() {
        Adt5::C0(_, h54, _, _, _, _, _) => {
            h54
        }
    })?), f7(Str::lit("version_argv"), match v0.clone() {
        Adt5::C0(_, _, h55, _, _, _, _) => {
            h55
        }
    })?), f25(Str::lit("version_stdout_kappa"), match v0.clone() {
        Adt5::C0(_, _, _, h56, _, _, _) => {
            h56
        }
    })?), f7(Str::lit("compile_argv"), match v0.clone() {
        Adt5::C0(_, _, _, _, h57, _, _) => {
            h57
        }
    })?), f5(Str::lit("output"), match v0.clone() {
        Adt5::C0(_, _, _, _, _, h58, _) => {
            h58
        }
    })?), f7(Str::lit("resources"), match v0 {
        Adt5::C0(_, _, _, _, _, _, h59) => {
            h59
        }
    })?))
}

pub fn f14(v0: List<Adt1>) -> R<Bytes> {
    let m60 = v0.clone();
    match m60.uncons() {
        None => {
            Ok(f4(f6(Str::lit("lexicon_source = []")), Bytes::lit(&[0x0a])))
        }
        Some((_, _)) => {
            f26(Fn2::F27, Bytes::lit(&[]), f28(v0)?)
        }
    }
}

pub fn f15(v0: Adt4) -> R<Bytes> {
    Ok(f4(f4(f4(f4(f4(f4(f4(f4(f4(f4(f4(f24(Str::lit("[limits]")), f29(Str::lit("max_file_bytes"), match v0.clone() {
        Adt4::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
            h61
        }
    })?), f29(Str::lit("max_total_source_bytes"), match v0.clone() {
        Adt4::C0(_, h62, _, _, _, _, _, _, _, _, _) => {
            h62
        }
    })?), f29(Str::lit("max_primitive_atoms"), match v0.clone() {
        Adt4::C0(_, _, h63, _, _, _, _, _, _, _, _) => {
            h63
        }
    })?), f29(Str::lit("max_token_lattice_edges"), match v0.clone() {
        Adt4::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
            h64
        }
    })?), f29(Str::lit("max_parse_states"), match v0.clone() {
        Adt4::C0(_, _, _, _, h65, _, _, _, _, _, _) => {
            h65
        }
    })?), f29(Str::lit("max_ir_nodes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, h66, _, _, _, _, _) => {
            h66
        }
    })?), f29(Str::lit("max_scope_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h67, _, _, _, _) => {
            h67
        }
    })?), f29(Str::lit("max_import_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h68, _, _, _) => {
            h68
        }
    })?), f29(Str::lit("max_diagnostics"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h69, _, _) => {
            h69
        }
    })?), f29(Str::lit("max_child_output_bytes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h70, _) => {
            h70
        }
    })?), f29(Str::lit("child_timeout_ms"), match v0 {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h71) => {
            h71
        }
    })?))
}

pub fn f16(v0: Bytes) -> R<Adt6> {
    let v1: u64 = {
        let a72 = v0.clone();
        length_bytes(a72)
    };
    let v2: u64 = {
        let a73 = v1;
        let a74 = 0u64;
        if nat_eq(a73, a74) {
            1u64
        } else {
            let a77 = {
                let a75 = v1;
                let a76 = 1023u64;
                nat_add(a75, a76)?
            };
            let a78 = 1024u64;
            let a79 = 0u64;
            nat_quot(a77, a78, a79)
        }
    };
    let v3: u64 = {
        let a80 = v2;
        let a81 = 1u64;
        nat_sub(a80, a81)
    };
    let m83 = {
        let (_, h82) = f30(v0.clone(), v1, v3)?;
        h82
    };
    match m83.uncons() {
        None => {
            f31(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h84) = f32(Fn3::F33, 64u64, (List::cons(v4, v5), f31(v0, v1, v3, false)?));
            Ok(h84)
        }
    }
}

pub fn f17(v0: u32) -> Bytes {
    let a97 = {
        let a85 = f34(v0, 4u32);
        let a86 = f34(v0, 0u32);
        append_bytes(a85, a86)
    };
    let a98 = {
        let a95 = {
            let a87 = f34(v0, 12u32);
            let a88 = f34(v0, 8u32);
            append_bytes(a87, a88)
        };
        let a96 = {
            let a93 = {
                let a89 = f34(v0, 20u32);
                let a90 = f34(v0, 16u32);
                append_bytes(a89, a90)
            };
            let a94 = {
                let a91 = f34(v0, 28u32);
                let a92 = f34(v0, 24u32);
                append_bytes(a91, a92)
            };
            append_bytes(a93, a94)
        };
        append_bytes(a95, a96)
    };
    append_bytes(a97, a98)
}

pub fn f18(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f19(v0, f35(v1)?))
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    f36(f36(f36(v0, f21(Str::lit(" = "))), v1), Bytes::lit(&[0x0a]))
}

pub fn f20(v0: List<Bytes>) -> R<Bytes> {
    Ok(f36(f36(f21(Str::lit("[")), f37(f38(f39(Fn4::F40, List::<Bytes>::nil(), v0)?)?, f21(Str::lit(", ")))), f21(Str::lit("]"))))
}

pub fn f21(v0: Str) -> Bytes {
    let a99 = v0;
    utf8_encode(a99)
}

pub fn f22(v0: Adt2) -> R<Bytes> {
    Ok(f4(f4(f24(Str::lit("[[artifact_source]]")), f25(Str::lit("kappa"), match v0.clone() {
        Adt2::C0(h100, _) => {
            h100
        }
    })?), f5(Str::lit("path"), match v0 {
        Adt2::C0(_, h101) => {
            h101
        }
    })?))
}

pub fn f23(v0: Adt3) -> R<Bytes> {
    Ok(f4(f4(f4(f4(f24(Str::lit("[[source_package]]")), f5(Str::lit("package"), match v0.clone() {
        Adt3::C0(h102, _, _) => {
            h102
        }
    })?), f5(Str::lit("version"), match v0.clone() {
        Adt3::C0(_, h103, _) => {
            h103
        }
    })?), f5(Str::lit("kind"), f6(Str::lit("path")))?), f5(Str::lit("path"), match v0 {
        Adt3::C0(_, _, h104) => {
            h104
        }
    })?))
}

pub fn f24(v0: Str) -> Bytes {
    f41(f6(v0))
}

pub fn f25(v0: Str, v1: Bytes) -> R<Bytes> {
    f42(f6(v0), v1)
}

pub fn f26(v0: Fn2, p108: Bytes, p109: List<Adt1>) -> R<Bytes> {
    let m107 = (p108, p109);
    {
        let mut m107 = m107;
        loop {
            let step = {
                let (v1, v2) = m107;
                let m105 = v2;
                match m105.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt1>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt1>), R<Bytes>>(({
                            let c106: Fn2 = v0.clone();
                            c106.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Bytes, v1: Adt1) -> R<Bytes> {
    Ok(f4(v0, f43(v1)?))
}

pub fn f28(v0: List<Adt1>) -> R<List<Adt1>> {
    Ok(f44(Fn5::F45, List::<Adt1>::nil(), f46(f47(f46(f48(Fn6::F49, List::<(Bytes, Adt1)>::nil(), v0)))?)))
}

pub fn f29(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f19(f6(v0), f50(v1)?))
}

pub fn f30(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt6>)> {
    f51(Fn7::F52(v0, v1), v2, (0u64, List::<Adt6>::nil()))
}

pub fn f31(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt6> {
    let v4: u64 = {
        let a110 = 1024u64;
        let a111 = v2;
        nat_mul(a110, a111)?
    };
    let v5: u64 = f53(v1, v4);
    let v6: u64 = f54(v5)?;
    let v7: u64 = {
        let a112 = v6;
        let a113 = 1u64;
        nat_sub(a112, a113)
    };
    Ok(f55(f56(v0.clone(), v4, v2, v7)?, f57(v0, {
        let a116 = v4;
        let a117 = {
            let a114 = 64u64;
            let a115 = v7;
            nat_mul(a114, a115)?
        };
        nat_add(a116, a117)?
    })?, v2, f58({
        let a120 = v5;
        let a121 = {
            let a118 = 64u64;
            let a119 = v7;
            nat_mul(a118, a119)?
        };
        nat_sub(a120, a121)
    }), {
        let a124 = {
            let a122 = v7;
            let a123 = 0u64;
            if nat_eq(a122, a123) {
                3u32
            } else {
                2u32
            }
        };
        let a125 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a124, a125)
    }))
}

pub fn f32(v0: Fn3, p129: u64, p130: (List<Adt6>, Adt6)) -> (List<Adt6>, Adt6) {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v1, v2) = m128;
                let m126 = v1;
                if m126 == 0 {
                    Err::<(u64, (List<Adt6>, Adt6)), (List<Adt6>, Adt6)>(v2)
                } else {
                    let v3 = m126 - 1;
                    Ok::<(u64, (List<Adt6>, Adt6)), (List<Adt6>, Adt6)>((v3, {
                        let c127: Fn3 = v0.clone();
                        c127.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: (List<Adt6>, Adt6)) -> (List<Adt6>, Adt6) {
    f59(v0)
}

pub fn f34(v0: u32, v1: u32) -> Bytes {
    f60(f61(v0, v1))
}

pub fn f35(v0: Bytes) -> R<Bytes> {
    Ok(f36(f36(Bytes::lit(&[0x22]), {
        let (_, h131) = {
            let (h132, _) = f62(Fn8::F63(v0.clone()), {
                let a134 = {
                    let a133 = v0;
                    length_bytes(a133)
                };
                let a135 = 1u64;
                nat_add(a134, a135)?
            }, (0u64, Bytes::lit(&[])))?;
            h132
        };
        h131
    }), Bytes::lit(&[0x22])))
}

pub fn f36(v0: Bytes, v1: Bytes) -> Bytes {
    let a136 = v0;
    let a137 = v1;
    append_bytes(a136, a137)
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m138 = f64(Fn9::F65(v1), None::<Bytes>, v0);
    match m138 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: List<Bytes>) -> R<List<Bytes>> {
    f39(Fn4::F66, List::<Bytes>::nil(), v0)
}

pub fn f39(v0: Fn4, p142: List<Bytes>, p143: List<Bytes>) -> R<List<Bytes>> {
    let m141 = (p142, p143);
    {
        let mut m141 = m141;
        loop {
            let step = {
                let (v1, v2) = m141;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c140: Fn4 = v0.clone();
                            c140.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f35(v1)?, v0))
}

pub fn f41(v0: Bytes) -> Bytes {
    f36(f36(Bytes::lit(&[0x0a]), v0), Bytes::lit(&[0x0a]))
}

pub fn f42(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f19(v0, f35(v1)?))
}

pub fn f43(v0: Adt1) -> R<Bytes> {
    Ok(f4(f24(Str::lit("[[lexicon_source]]")), {
        let m144 = v0;
        match m144 {
            Adt1::C0(v1) => {
                f4(f5(Str::lit("package"), v1)?, f5(Str::lit("kind"), f6(Str::lit("builtin")))?)
            }
            Adt1::C1(v2, v3) => {
                f4(f4(f5(Str::lit("package"), v2)?, f5(Str::lit("kind"), f6(Str::lit("path")))?), f5(Str::lit("path"), v3)?)
            }
            Adt1::C2(v4, v5, v6, v7) => {
                f4(f4(f4(f4(f5(Str::lit("package"), v4)?, f5(Str::lit("kind"), f6(Str::lit("git")))?), f5(Str::lit("url"), v5)?), f5(Str::lit("revision"), v6)?), f5(Str::lit("subdirectory"), v7)?)
            }
        }
    }))
}

pub fn f44(v0: Fn5, p148: List<Adt1>, p149: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v1, v2) = m147;
                let m145 = v2;
                match m145.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c146: Fn5 = v0.clone();
                            c146.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h150) = v1;
        h150
    }, v0)
}

pub fn f46(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f67(Fn10::F68, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f47(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f69(f70(Fn11::F71, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m155 = {
        let (h151, _) = f72(Fn12::F73, {
            let a153 = {
                let a152 = v0;
                length_list(a152)
            };
            let a154 = 1u64;
            nat_add(a153, a154)?
        }, v1)?;
        h151
    };
    match m155.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f48(v0: Fn6, p159: List<(Bytes, Adt1)>, p160: List<Adt1>) -> List<(Bytes, Adt1)> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, v2) = m158;
                let m156 = v2;
                match m156.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(({
                            let c157: Fn6 = v0.clone();
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

pub fn f49(v0: List<(Bytes, Adt1)>, v1: Adt1) -> List<(Bytes, Adt1)> {
    List::cons((f74(v1.clone()), v1), v0)
}

pub fn f50(v0: u64) -> R<Bytes> {
    f75(f76(), 10u64, v0)
}

pub fn f51(v0: Fn7, p164: u64, p165: (u64, List<Adt6>)) -> R<(u64, List<Adt6>)> {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v1, v2) = m163;
                let m161 = v1;
                if m161 == 0 {
                    Err::<(u64, (u64, List<Adt6>)), R<(u64, List<Adt6>)>>(Ok(v2))
                } else {
                    let v3 = m161 - 1;
                    Ok::<(u64, (u64, List<Adt6>)), R<(u64, List<Adt6>)>>((v3, {
                        let c162: Fn7 = v0.clone();
                        c162.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: u64, v2: (u64, List<Adt6>)) -> R<(u64, List<Adt6>)> {
    f77(v2.clone(), f31(v0, v1, {
        let (h166, _) = v2;
        h166
    }, false)?)
}

pub fn f53(v0: u64, v1: u64) -> u64 {
    let a169 = 1024u64;
    let a170 = {
        let a167 = v0;
        let a168 = v1;
        nat_sub(a167, a168)
    };
    if nat_lt(a169, a170) {
        1024u64
    } else {
        let a171 = v0;
        let a172 = v1;
        nat_sub(a171, a172)
    }
}

pub fn f54(v0: u64) -> R<u64> {
    let a173 = v0;
    let a174 = 0u64;
    if nat_eq(a173, a174) {
        Ok(1u64)
    } else {
        let a177 = {
            let a175 = v0;
            let a176 = 63u64;
            nat_add(a175, a176)?
        };
        let a178 = 64u64;
        let a179 = 0u64;
        Ok(nat_quot(a177, a178, a179))
    }
}

pub fn f55(v0: Adt6, v1: Adt7, v2: u64, v3: u32, v4: u32) -> Adt6 {
    let v5: Adt8 = {
        let (h180, _) = f78(Fn13::F79, 7u64, (Adt8::C0(match v0.clone() {
            Adt6::C0(h181, _, _, _, _, _, _, _) => {
                h181
            }
        }, match v0.clone() {
            Adt6::C0(_, h182, _, _, _, _, _, _) => {
                h182
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h183, _, _, _, _, _) => {
                h183
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h184, _, _, _, _) => {
                h184
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, _, h185, _, _, _) => {
                h185
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, _, _, h186, _, _) => {
                h186
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, h187, _) => {
                h187
            }
        }, match v0 {
            Adt6::C0(_, _, _, _, _, _, _, h188) => {
                h188
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f58({
            let a189 = v2;
            let a190 = 4294967296u64;
            let a191 = 0u64;
            nat_rem(a189, a190, a191)
        }), f58({
            let a192 = v2;
            let a193 = 4294967296u64;
            let a194 = 0u64;
            nat_quot(a192, a193, a194)
        }), v3, v4), v1));
        h180
    };
    Adt6::C0({
        let a197 = match v5.clone() {
            Adt8::C0(h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h195
            }
        };
        let a198 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, h196, _, _, _, _, _, _, _) => {
                h196
            }
        };
        fixed_u32::bit_xor(a197, a198)
    }, {
        let a201 = match v5.clone() {
            Adt8::C0(_, h199, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h199
            }
        };
        let a202 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, h200, _, _, _, _, _, _) => {
                h200
            }
        };
        fixed_u32::bit_xor(a201, a202)
    }, {
        let a205 = match v5.clone() {
            Adt8::C0(_, _, h203, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h203
            }
        };
        let a206 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h204, _, _, _, _, _) => {
                h204
            }
        };
        fixed_u32::bit_xor(a205, a206)
    }, {
        let a209 = match v5.clone() {
            Adt8::C0(_, _, _, h207, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        };
        let a210 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h208, _, _, _, _) => {
                h208
            }
        };
        fixed_u32::bit_xor(a209, a210)
    }, {
        let a213 = match v5.clone() {
            Adt8::C0(_, _, _, _, h211, _, _, _, _, _, _, _, _, _, _, _) => {
                h211
            }
        };
        let a214 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h212, _, _, _) => {
                h212
            }
        };
        fixed_u32::bit_xor(a213, a214)
    }, {
        let a217 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, h215, _, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        };
        let a218 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h216, _, _) => {
                h216
            }
        };
        fixed_u32::bit_xor(a217, a218)
    }, {
        let a221 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, h219, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        };
        let a222 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h220, _) => {
                h220
            }
        };
        fixed_u32::bit_xor(a221, a222)
    }, {
        let a225 = match v5.clone() {
            Adt8::C0(_, _, _, _, _, _, _, h223, _, _, _, _, _, _, _, _) => {
                h223
            }
        };
        let a226 = match v5 {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h224) => {
                h224
            }
        };
        fixed_u32::bit_xor(a225, a226)
    })
}

pub fn f56(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let (_, h227) = f80(Fn14::F81(v2, v0, v1), v3, (0u64, f82()))?;
    Ok(h227)
}

pub fn f57(v0: Bytes, v1: u64) -> R<Adt7> {
    Ok(Adt7::C0(f83(v0.clone(), {
        let a228 = v1;
        let a229 = 0u64;
        nat_add(a228, a229)?
    })?, f83(v0.clone(), {
        let a230 = v1;
        let a231 = 4u64;
        nat_add(a230, a231)?
    })?, f83(v0.clone(), {
        let a232 = v1;
        let a233 = 8u64;
        nat_add(a232, a233)?
    })?, f83(v0.clone(), {
        let a234 = v1;
        let a235 = 12u64;
        nat_add(a234, a235)?
    })?, f83(v0.clone(), {
        let a236 = v1;
        let a237 = 16u64;
        nat_add(a236, a237)?
    })?, f83(v0.clone(), {
        let a238 = v1;
        let a239 = 20u64;
        nat_add(a238, a239)?
    })?, f83(v0.clone(), {
        let a240 = v1;
        let a241 = 24u64;
        nat_add(a240, a241)?
    })?, f83(v0.clone(), {
        let a242 = v1;
        let a243 = 28u64;
        nat_add(a242, a243)?
    })?, f83(v0.clone(), {
        let a244 = v1;
        let a245 = 32u64;
        nat_add(a244, a245)?
    })?, f83(v0.clone(), {
        let a246 = v1;
        let a247 = 36u64;
        nat_add(a246, a247)?
    })?, f83(v0.clone(), {
        let a248 = v1;
        let a249 = 40u64;
        nat_add(a248, a249)?
    })?, f83(v0.clone(), {
        let a250 = v1;
        let a251 = 44u64;
        nat_add(a250, a251)?
    })?, f83(v0.clone(), {
        let a252 = v1;
        let a253 = 48u64;
        nat_add(a252, a253)?
    })?, f83(v0.clone(), {
        let a254 = v1;
        let a255 = 52u64;
        nat_add(a254, a255)?
    })?, f83(v0.clone(), {
        let a256 = v1;
        let a257 = 56u64;
        nat_add(a256, a257)?
    })?, f83(v0, {
        let a258 = v1;
        let a259 = 60u64;
        nat_add(a258, a259)?
    })?))
}

pub fn f58(v0: u64) -> u32 {
    let (_, h260) = {
        let (_, h261) = f84(Fn15::F85, 32u64, (v0, (1u32, 0u32)));
        h261
    };
    h260
}

pub fn f59(v0: (List<Adt6>, Adt6)) -> (List<Adt6>, Adt6) {
    let m263 = {
        let (h262, _) = v0.clone();
        h262
    };
    match m263.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f86(v1, {
                let (_, h264) = v0;
                h264
            }, {
                let m265 = v2;
                match m265.uncons() {
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

pub fn f60(v0: u32) -> Bytes {
    let a268 = {
        let a266 = v0;
        let a267 = 8u32;
        fixed_u32::bit_and(a266, a267)
    };
    let a269 = 0u32;
    if equal(a268, a269) {
        let a272 = {
            let a270 = v0;
            let a271 = 4u32;
            fixed_u32::bit_and(a270, a271)
        };
        let a273 = 0u32;
        if equal(a272, a273) {
            let a276 = {
                let a274 = v0;
                let a275 = 2u32;
                fixed_u32::bit_and(a274, a275)
            };
            let a277 = 0u32;
            if equal(a276, a277) {
                let a280 = {
                    let a278 = v0;
                    let a279 = 1u32;
                    fixed_u32::bit_and(a278, a279)
                };
                let a281 = 0u32;
                if equal(a280, a281) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a284 = {
                    let a282 = v0;
                    let a283 = 1u32;
                    fixed_u32::bit_and(a282, a283)
                };
                let a285 = 0u32;
                if equal(a284, a285) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a288 = {
                let a286 = v0;
                let a287 = 2u32;
                fixed_u32::bit_and(a286, a287)
            };
            let a289 = 0u32;
            if equal(a288, a289) {
                let a292 = {
                    let a290 = v0;
                    let a291 = 1u32;
                    fixed_u32::bit_and(a290, a291)
                };
                let a293 = 0u32;
                if equal(a292, a293) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a296 = {
                    let a294 = v0;
                    let a295 = 1u32;
                    fixed_u32::bit_and(a294, a295)
                };
                let a297 = 0u32;
                if equal(a296, a297) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a300 = {
            let a298 = v0;
            let a299 = 4u32;
            fixed_u32::bit_and(a298, a299)
        };
        let a301 = 0u32;
        if equal(a300, a301) {
            let a304 = {
                let a302 = v0;
                let a303 = 2u32;
                fixed_u32::bit_and(a302, a303)
            };
            let a305 = 0u32;
            if equal(a304, a305) {
                let a308 = {
                    let a306 = v0;
                    let a307 = 1u32;
                    fixed_u32::bit_and(a306, a307)
                };
                let a309 = 0u32;
                if equal(a308, a309) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a312 = {
                    let a310 = v0;
                    let a311 = 1u32;
                    fixed_u32::bit_and(a310, a311)
                };
                let a313 = 0u32;
                if equal(a312, a313) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a316 = {
                let a314 = v0;
                let a315 = 2u32;
                fixed_u32::bit_and(a314, a315)
            };
            let a317 = 0u32;
            if equal(a316, a317) {
                let a320 = {
                    let a318 = v0;
                    let a319 = 1u32;
                    fixed_u32::bit_and(a318, a319)
                };
                let a321 = 0u32;
                if equal(a320, a321) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a324 = {
                    let a322 = v0;
                    let a323 = 1u32;
                    fixed_u32::bit_and(a322, a323)
                };
                let a325 = 0u32;
                if equal(a324, a325) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f61(v0: u32, v1: u32) -> u32 {
    let m328 = {
        let a326 = v0;
        let a327 = v1;
        fixed_u32::shift_right(a326, a327)
    };
    match m328 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f62(v0: Fn8, p333: u64, p334: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m332 = (p333, p334);
    {
        let mut m332 = m332;
        loop {
            let step = {
                let (v1, v2) = m332;
                let m329 = v1;
                if m329 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m329 - 1;
                    let m331 = {
                        let c330: Fn8 = v0.clone();
                        c330.apply(v2.clone())?
                    };
                    match m331 {
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
                Ok(next) => { m332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f87(v0, v1)
}

pub fn f64(v0: Fn9, p338: Option<Bytes>, p339: List<Bytes>) -> Option<Bytes> {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c336: Fn9 = v0.clone();
                            c336.apply(v1, v3)
                        }, v4))
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

pub fn f65(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m340 = v1;
    match m340 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a343 = {
                    let a341 = v3;
                    let a342 = v0;
                    append_bytes(a341, a342)
                };
                let a344 = v2;
                append_bytes(a343, a344)
            })
        }
    }
}

pub fn f66(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f67(v0: Fn10, p348: List<(Bytes, Adt1)>, p349: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m347 = (p348, p349);
    {
        let mut m347 = m347;
        loop {
            let step = {
                let (v1, v2) = m347;
                let m345 = v2;
                match m345.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c346: Fn10 = v0.clone();
                            c346.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f69(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f88(Fn16::F89, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f70(v0: Fn11, p353: List<List<(Bytes, Adt1)>>, p354: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m352 = (p353, p354);
    {
        let mut m352 = m352;
        loop {
            let step = {
                let (v1, v2) = m352;
                let m350 = v2;
                match m350.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c351: Fn11 = v0.clone();
                            c351.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f72(v0: Fn12, p359: u64, p360: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m358 = (p359, p360);
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v1, v2) = m358;
                let m355 = v1;
                if m355 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m355 - 1;
                    let m357 = {
                        let c356: Fn12 = v0.clone();
                        c356.apply(v2.clone())?
                    };
                    match m357 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f90(v0)
}

pub fn f74(v0: Adt1) -> Bytes {
    let m361 = v0;
    match m361 {
        Adt1::C0(v1) => {
            v1
        }
        Adt1::C1(v2, _) => {
            v2
        }
        Adt1::C2(v4, _, _, _) => {
            v4
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a362 = v2;
    let a363 = 0u64;
    if nat_eq(a362, a363) {
        Ok(f91(v0, 0u64))
    } else {
        let (_, h364) = {
            let (h365, _) = f62(Fn8::F92(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h365
        };
        Ok(h364)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: (u64, List<Adt6>), v1: Adt6) -> R<(u64, List<Adt6>)> {
    let v2: Adt9 = f93(Fn17::F94, 40u64, Adt9::C0(v1, {
        let a367 = {
            let (h366, _) = v0.clone();
            h366
        };
        let a368 = 1u64;
        nat_add(a367, a368)?
    }, {
        let (_, h369) = v0;
        h369
    }));
    Ok(({
        let a371 = {
            let (h370, _) = v0;
            h370
        };
        let a372 = 1u64;
        nat_add(a371, a372)?
    }, List::cons(match v2.clone() {
        Adt9::C0(h373, _, _) => {
            h373
        }
    }, match v2 {
        Adt9::C0(_, _, h374) => {
            h374
        }
    })))
}

pub fn f78(v0: Fn13, p378: u64, p379: (Adt8, Adt7)) -> (Adt8, Adt7) {
    let m377 = (p378, p379);
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v1, v2) = m377;
                let m375 = v1;
                if m375 == 0 {
                    Err::<(u64, (Adt8, Adt7)), (Adt8, Adt7)>(v2)
                } else {
                    let v3 = m375 - 1;
                    Ok::<(u64, (Adt8, Adt7)), (Adt8, Adt7)>((v3, {
                        let c376: Fn13 = v0.clone();
                        c376.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: (Adt8, Adt7)) -> (Adt8, Adt7) {
    f95(v0)
}

pub fn f80(v0: Fn14, p383: u64, p384: (u64, Adt6)) -> R<(u64, Adt6)> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v1, v2) = m382;
                let m380 = v1;
                if m380 == 0 {
                    Err::<(u64, (u64, Adt6)), R<(u64, Adt6)>>(Ok(v2))
                } else {
                    let v3 = m380 - 1;
                    Ok::<(u64, (u64, Adt6)), R<(u64, Adt6)>>((v3, {
                        let c381: Fn14 = v0.clone();
                        c381.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a386 = {
            let (h385, _) = v3.clone();
            h385
        };
        let a387 = 1u64;
        nat_add(a386, a387)?
    }, f55({
        let (_, h388) = v3.clone();
        h388
    }, f57(v1, {
        let a392 = v2;
        let a393 = {
            let a390 = 64u64;
            let a391 = {
                let (h389, _) = v3.clone();
                h389
            };
            nat_mul(a390, a391)?
        };
        nat_add(a392, a393)?
    })?, v0, 64u32, {
        let a395 = {
            let (h394, _) = v3;
            h394
        };
        let a396 = 0u64;
        if nat_eq(a395, a396) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f82() -> Adt6 {
    Adt6::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f83(v0: Bytes, v1: u64) -> R<u32> {
    let a407 = f96(v0.clone(), v1);
    let a408 = {
        let a405 = f97(f96(v0.clone(), {
            let a397 = v1;
            let a398 = 1u64;
            nat_add(a397, a398)?
        }), 8u32);
        let a406 = {
            let a403 = f97(f96(v0.clone(), {
                let a399 = v1;
                let a400 = 2u64;
                nat_add(a399, a400)?
            }), 16u32);
            let a404 = f97(f96(v0, {
                let a401 = v1;
                let a402 = 3u64;
                nat_add(a401, a402)?
            }), 24u32);
            fixed_u32::bit_or(a403, a404)
        };
        fixed_u32::bit_or(a405, a406)
    };
    Ok(fixed_u32::bit_or(a407, a408))
}

pub fn f84(v0: Fn15, p412: u64, p413: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, v2) = m411;
                let m409 = v1;
                if m409 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m409 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c410: Fn15 = v0.clone();
                        c410.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h414, _) = v0;
        h414
    };
    let v2: u32 = {
        let (h415, _) = {
            let (_, h416) = v0;
            h416
        };
        h415
    };
    let v3: u32 = {
        let (_, h417) = {
            let (_, h418) = v0;
            h418
        };
        h417
    };
    ({
        let a419 = v1;
        let a420 = 2u64;
        let a421 = 0u64;
        nat_quot(a419, a420, a421)
    }, (f97(v2, 1u32), {
        let a425 = {
            let a422 = v1;
            let a423 = 2u64;
            let a424 = 0u64;
            nat_rem(a422, a423, a424)
        };
        let a426 = 1u64;
        if nat_eq(a425, a426) {
            let a427 = v3;
            let a428 = v2;
            fixed_u32::bit_or(a427, a428)
        } else {
            v3
        }
    }))
}

pub fn f86(v0: Adt6, v1: Adt6, v2: bool) -> Adt6 {
    f55(f82(), f98(v0, v1), 0u64, 64u32, {
        let a429 = 4u32;
        let a430 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a429, a430)
    })
}

pub fn f87(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m432 = f99(v0, {
        let (h431, _) = v1.clone();
        h431
    });
    match m432 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a434 = {
                    let (h433, _) = v1;
                    h433
                };
                let a435 = 1u64;
                nat_add(a434, a435)?
            }, f36({
                let (_, h436) = v1;
                h436
            }, f100({
                let v3: u8 = v2;
                let a481 = {
                    let a457 = {
                        let a445 = {
                            let a439 = {
                                let a437 = v3;
                                let a438 = 128u8;
                                fixed_u8::bit_and(a437, a438)
                            };
                            let a440 = 128u8;
                            if equal(a439, a440) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a446 = {
                            let a443 = {
                                let a441 = v3;
                                let a442 = 64u8;
                                fixed_u8::bit_and(a441, a442)
                            };
                            let a444 = 64u8;
                            if equal(a443, a444) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a445, a446)?
                    };
                    let a458 = {
                        let a455 = {
                            let a449 = {
                                let a447 = v3;
                                let a448 = 32u8;
                                fixed_u8::bit_and(a447, a448)
                            };
                            let a450 = 32u8;
                            if equal(a449, a450) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a456 = {
                            let a453 = {
                                let a451 = v3;
                                let a452 = 16u8;
                                fixed_u8::bit_and(a451, a452)
                            };
                            let a454 = 16u8;
                            if equal(a453, a454) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a455, a456)?
                    };
                    nat_add(a457, a458)?
                };
                let a482 = {
                    let a479 = {
                        let a467 = {
                            let a461 = {
                                let a459 = v3;
                                let a460 = 8u8;
                                fixed_u8::bit_and(a459, a460)
                            };
                            let a462 = 8u8;
                            if equal(a461, a462) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a468 = {
                            let a465 = {
                                let a463 = v3;
                                let a464 = 4u8;
                                fixed_u8::bit_and(a463, a464)
                            };
                            let a466 = 4u8;
                            if equal(a465, a466) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a467, a468)?
                    };
                    let a480 = {
                        let a477 = {
                            let a471 = {
                                let a469 = v3;
                                let a470 = 2u8;
                                fixed_u8::bit_and(a469, a470)
                            };
                            let a472 = 2u8;
                            if equal(a471, a472) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a478 = {
                            let a475 = {
                                let a473 = v3;
                                let a474 = 1u8;
                                fixed_u8::bit_and(a473, a474)
                            };
                            let a476 = 1u8;
                            if equal(a475, a476) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a477, a478)?
                    };
                    nat_add(a479, a480)?
                };
                nat_add(a481, a482)?
            })))))
        }
    }
}

pub fn f88(v0: Fn16, p486: List<List<(Bytes, Adt1)>>, p487: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v1, v2) = m485;
                let m483 = v2;
                match m483.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c484: Fn16 = v0.clone();
                            c484.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f90(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a489 = {
        let a488 = v0.clone();
        length_list(a488)
    };
    let a490 = 1u64;
    if nat_le(a489, a490) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f101(v0)?))
    }
}

pub fn f91(v0: Bytes, v1: u64) -> Bytes {
    let m494 = {
        let a491 = v0;
        let a492 = v1;
        let a493 = 1u64;
        slice_bytes(a491, a492, a493)
    };
    match m494 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f102(v1, v0, v2)
}

pub fn f93(v0: Fn17, p498: u64, p499: Adt9) -> Adt9 {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m495 = v1;
                if m495 == 0 {
                    Err::<(u64, Adt9), Adt9>(v2)
                } else {
                    let v3 = m495 - 1;
                    Ok::<(u64, Adt9), Adt9>((v3, {
                        let c496: Fn17 = v0.clone();
                        c496.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt9) -> Adt9 {
    f103(v0)
}

pub fn f95(v0: (Adt8, Adt7)) -> (Adt8, Adt7) {
    (f104(f105({
        let (h500, _) = v0.clone();
        h500
    }, {
        let (_, h501) = v0.clone();
        h501
    }), {
        let (_, h502) = v0.clone();
        h502
    }), f106({
        let (_, h503) = v0;
        h503
    }))
}

pub fn f96(v0: Bytes, v1: u64) -> u32 {
    let m506 = {
        let a504 = v0;
        let a505 = v1;
        index_bytes(a504, a505)
    };
    match m506 {
        None => {
            0u32
        }
        Some(v2) => {
            let m508 = {
                let a507 = v2;
                fixed_u32::convert(i128::from(a507))
            };
            match m508 {
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

pub fn f97(v0: u32, v1: u32) -> u32 {
    let m511 = {
        let a509 = v0;
        let a510 = v1;
        fixed_u32::shift_left(a509, a510)
    };
    match m511 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f98(v0: Adt6, v1: Adt6) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt6::C0(h512, _, _, _, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt6::C0(_, h513, _, _, _, _, _, _) => {
            h513
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h514, _, _, _, _, _) => {
            h514
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h515, _, _, _, _) => {
            h515
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h516, _, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h517, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h518, _) => {
            h518
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, h519) => {
            h519
        }
    }, match v1.clone() {
        Adt6::C0(h520, _, _, _, _, _, _, _) => {
            h520
        }
    }, match v1.clone() {
        Adt6::C0(_, h521, _, _, _, _, _, _) => {
            h521
        }
    }, match v1.clone() {
        Adt6::C0(_, _, h522, _, _, _, _, _) => {
            h522
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, h523, _, _, _, _) => {
            h523
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, h524, _, _, _) => {
            h524
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, h525, _, _) => {
            h525
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, h526, _) => {
            h526
        }
    }, match v1 {
        Adt6::C0(_, _, _, _, _, _, _, h527) => {
            h527
        }
    })
}

pub fn f99(v0: Bytes, v1: u64) -> Option<u8> {
    let a528 = v0;
    let a529 = v1;
    index_bytes(a528, a529)
}

pub fn f100(v0: u64) -> Bytes {
    let a530 = v0;
    let a531 = 34u64;
    if nat_eq(a530, a531) {
        Bytes::lit(&[0x5c, 0x22])
    } else {
        let a532 = v0;
        let a533 = 92u64;
        if nat_eq(a532, a533) {
            Bytes::lit(&[0x5c, 0x5c])
        } else {
            let a534 = v0;
            let a535 = 32u64;
            if if nat_lt(a534, a535) {
                true
            } else {
                let a536 = v0;
                let a537 = 127u64;
                nat_eq(a536, a537)
            } {
                f36(f36(Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]), f107({
                    let a538 = v0;
                    let a539 = 16u64;
                    let a540 = 0u64;
                    nat_quot(a538, a539, a540)
                })), f107({
                    let a541 = v0;
                    let a542 = 16u64;
                    let a543 = 0u64;
                    nat_rem(a541, a542, a543)
                }))
            } else {
                f91(f108(), v0)
            }
        }
    }
}

pub fn f101(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f69({
        let (_, h544) = {
            let (h545, _) = f109(Fn18::F110, {
                let a547 = {
                    let a546 = v0.clone();
                    length_list(a546)
                };
                let a548 = 1u64;
                nat_add(a547, a548)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h545
        };
        h544
    }))
}

pub fn f102(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h549, _) = v2;
        h549
    };
    let a550 = v3;
    let a551 = 0u64;
    if nat_eq(a550, a551) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a552 = v3;
            let a553 = v1;
            let a554 = 0u64;
            nat_quot(a552, a553, a554)
        }, {
            let a559 = f91(v0, {
                let a555 = v3;
                let a556 = v1;
                let a557 = 0u64;
                nat_rem(a555, a556, a557)
            });
            let a560 = {
                let (_, h558) = v2;
                h558
            };
            append_bytes(a559, a560)
        }))
    }
}

pub fn f103(v0: Adt9) -> Adt9 {
    let a565 = {
        let a562 = match v0.clone() {
            Adt9::C0(_, h561, _) => {
                h561
            }
        };
        let a563 = 2u64;
        let a564 = 0u64;
        nat_rem(a562, a563, a564)
    };
    let a566 = 0u64;
    if nat_eq(a565, a566) {
        let m568 = match v0.clone() {
            Adt9::C0(_, _, h567) => {
                h567
            }
        };
        match m568.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt9::C0(f86(v1, match v0.clone() {
                    Adt9::C0(h569, _, _) => {
                        h569
                    }
                }, false), {
                    let a571 = match v0 {
                        Adt9::C0(_, h570, _) => {
                            h570
                        }
                    };
                    let a572 = 2u64;
                    let a573 = 0u64;
                    nat_quot(a571, a572, a573)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f104(v0: Adt8, v1: Adt7) -> Adt8 {
    let v2: Adt10 = f111(match v0.clone() {
        Adt8::C0(h574, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h574
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h575, _, _, _, _, _, _, _, _, _, _) => {
            h575
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h576, _, _, _, _, _) => {
            h576
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h577) => {
            h577
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h578, _, _, _, _, _, _, _) => {
            h578
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h579, _, _, _, _, _, _) => {
            h579
        }
    });
    let v3: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, h580, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h580
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h581, _, _, _, _, _, _, _, _, _) => {
            h581
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h582, _, _, _, _) => {
            h582
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h583, _, _, _) => {
            h583
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h584, _, _, _, _, _) => {
            h584
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h585, _, _, _, _) => {
            h585
        }
    });
    let v4: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, _, h586, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h586
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h587, _, _, _, _, _, _, _, _) => {
            h587
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h588, _, _, _, _, _, _, _) => {
            h588
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h589, _, _) => {
            h589
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h590, _, _, _) => {
            h590
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h591, _, _) => {
            h591
        }
    });
    let v5: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, _, _, h592, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h592
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h593, _, _, _, _, _, _, _, _, _, _, _) => {
            h593
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h594, _, _, _, _, _, _) => {
            h594
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h595, _) => {
            h595
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h596, _) => {
            h596
        }
    }, match v1 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h597) => {
            h597
        }
    });
    Adt8::C0(match v2.clone() {
        Adt10::C0(h598, _, _, _) => {
            h598
        }
    }, match v3.clone() {
        Adt10::C0(h599, _, _, _) => {
            h599
        }
    }, match v4.clone() {
        Adt10::C0(h600, _, _, _) => {
            h600
        }
    }, match v5.clone() {
        Adt10::C0(h601, _, _, _) => {
            h601
        }
    }, match v5.clone() {
        Adt10::C0(_, h602, _, _) => {
            h602
        }
    }, match v2.clone() {
        Adt10::C0(_, h603, _, _) => {
            h603
        }
    }, match v3.clone() {
        Adt10::C0(_, h604, _, _) => {
            h604
        }
    }, match v4.clone() {
        Adt10::C0(_, h605, _, _) => {
            h605
        }
    }, match v4.clone() {
        Adt10::C0(_, _, h606, _) => {
            h606
        }
    }, match v5.clone() {
        Adt10::C0(_, _, h607, _) => {
            h607
        }
    }, match v2.clone() {
        Adt10::C0(_, _, h608, _) => {
            h608
        }
    }, match v3.clone() {
        Adt10::C0(_, _, h609, _) => {
            h609
        }
    }, match v3 {
        Adt10::C0(_, _, _, h610) => {
            h610
        }
    }, match v4 {
        Adt10::C0(_, _, _, h611) => {
            h611
        }
    }, match v5 {
        Adt10::C0(_, _, _, h612) => {
            h612
        }
    }, match v2 {
        Adt10::C0(_, _, _, h613) => {
            h613
        }
    })
}

pub fn f105(v0: Adt8, v1: Adt7) -> Adt8 {
    let v2: Adt10 = f111(match v0.clone() {
        Adt8::C0(h614, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h614
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h615, _, _, _, _, _, _, _, _, _, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h616, _, _, _, _, _, _, _) => {
            h616
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h617, _, _, _) => {
            h617
        }
    }, match v1.clone() {
        Adt7::C0(h618, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h618
        }
    }, match v1.clone() {
        Adt7::C0(_, h619, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h619
        }
    });
    let v3: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, h620, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h620
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h621, _, _, _, _, _, _, _, _, _, _) => {
            h621
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h622, _, _, _, _, _, _) => {
            h622
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h623, _, _) => {
            h623
        }
    }, match v1.clone() {
        Adt7::C0(_, _, h624, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h624
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, h625, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h625
        }
    });
    let v4: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, _, h626, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h626
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h627, _, _, _, _, _, _, _, _, _) => {
            h627
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h628, _, _, _, _, _) => {
            h628
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h629, _) => {
            h629
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, h630, _, _, _, _, _, _, _, _, _, _, _) => {
            h630
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, h631, _, _, _, _, _, _, _, _, _, _) => {
            h631
        }
    });
    let v5: Adt10 = f111(match v0.clone() {
        Adt8::C0(_, _, _, h632, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h632
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h633, _, _, _, _, _, _, _, _) => {
            h633
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h634, _, _, _, _) => {
            h634
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h635) => {
            h635
        }
    }, match v1.clone() {
        Adt7::C0(_, _, _, _, _, _, h636, _, _, _, _, _, _, _, _, _) => {
            h636
        }
    }, match v1 {
        Adt7::C0(_, _, _, _, _, _, _, h637, _, _, _, _, _, _, _, _) => {
            h637
        }
    });
    Adt8::C0(match v2.clone() {
        Adt10::C0(h638, _, _, _) => {
            h638
        }
    }, match v3.clone() {
        Adt10::C0(h639, _, _, _) => {
            h639
        }
    }, match v4.clone() {
        Adt10::C0(h640, _, _, _) => {
            h640
        }
    }, match v5.clone() {
        Adt10::C0(h641, _, _, _) => {
            h641
        }
    }, match v2.clone() {
        Adt10::C0(_, h642, _, _) => {
            h642
        }
    }, match v3.clone() {
        Adt10::C0(_, h643, _, _) => {
            h643
        }
    }, match v4.clone() {
        Adt10::C0(_, h644, _, _) => {
            h644
        }
    }, match v5.clone() {
        Adt10::C0(_, h645, _, _) => {
            h645
        }
    }, match v2.clone() {
        Adt10::C0(_, _, h646, _) => {
            h646
        }
    }, match v3.clone() {
        Adt10::C0(_, _, h647, _) => {
            h647
        }
    }, match v4.clone() {
        Adt10::C0(_, _, h648, _) => {
            h648
        }
    }, match v5.clone() {
        Adt10::C0(_, _, h649, _) => {
            h649
        }
    }, match v2 {
        Adt10::C0(_, _, _, h650) => {
            h650
        }
    }, match v3 {
        Adt10::C0(_, _, _, h651) => {
            h651
        }
    }, match v4 {
        Adt10::C0(_, _, _, h652) => {
            h652
        }
    }, match v5 {
        Adt10::C0(_, _, _, h653) => {
            h653
        }
    })
}

pub fn f106(v0: Adt7) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(_, _, h654, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h654
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h655, _, _, _, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h656, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h657, _, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h658, _, _, _, _, _, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt7::C0(h659, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h660, _, _, _, _, _, _, _, _, _, _, _) => {
            h660
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h661, _, _) => {
            h661
        }
    }, match v0.clone() {
        Adt7::C0(_, h662, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h662
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h663, _, _, _, _) => {
            h663
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h664, _, _, _) => {
            h664
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h665, _, _, _, _, _, _, _, _, _, _) => {
            h665
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h666, _, _, _, _, _, _) => {
            h666
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h667, _) => {
            h667
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h668) => {
            h668
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, h669, _, _, _, _, _, _, _) => {
            h669
        }
    })
}

pub fn f107(v0: u64) -> Bytes {
    f91(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v0)
}

pub fn f108() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f109(v0: Fn18, p674: u64, p675: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m673 = (p674, p675);
    {
        let mut m673 = m673;
        loop {
            let step = {
                let (v1, v2) = m673;
                let m670 = v1;
                if m670 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m670 - 1;
                    let m672 = {
                        let c671: Fn18 = v0.clone();
                        c671.apply(v2.clone())?
                    };
                    match m672 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f112(v0)
}

pub fn f111(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt10 {
    let v6: u32 = f113(f113(v0, v1), v4);
    let v7: u32 = f114({
        let a676 = v3;
        let a677 = v6;
        fixed_u32::bit_xor(a676, a677)
    }, 16u32, 16u32);
    let v8: u32 = f113(v2, v7);
    let v9: u32 = f114({
        let a678 = v1;
        let a679 = v8;
        fixed_u32::bit_xor(a678, a679)
    }, 12u32, 20u32);
    let v10: u32 = f113(f113(v6, v9), v5);
    let v11: u32 = f114({
        let a680 = v7;
        let a681 = v10;
        fixed_u32::bit_xor(a680, a681)
    }, 8u32, 24u32);
    let v12: u32 = f113(v8, v11);
    let v13: u32 = f114({
        let a682 = v9;
        let a683 = v12;
        fixed_u32::bit_xor(a682, a683)
    }, 7u32, 25u32);
    Adt10::C0(v10, v13, v12, v11)
}

pub fn f112(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h684, _) = v0;
        h684
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h685) = v0;
        h685
    };
    let m686 = v1;
    match m686.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m687 = v4;
            match m687.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f115(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f113(v0: u32, v1: u32) -> u32 {
    let m690 = {
        let a688 = f116(v0);
        let a689 = f116(v1);
        fixed_u64::checked_add(a688, a689)
    };
    match m690 {
        None => {
            0u32
        }
        Some(v2) => {
            f117(v2)
        }
    }
}

pub fn f114(v0: u32, v1: u32, v2: u32) -> u32 {
    let a691 = f61(v0, v1);
    let a692 = f97(v0, v2);
    fixed_u32::bit_or(a691, a692)
}

pub fn f115(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a697 = {
            let a695 = {
                let a693 = v0.clone();
                length_list(a693)
            };
            let a696 = {
                let a694 = v1.clone();
                length_list(a694)
            };
            nat_add(a695, a696)?
        };
        let a698 = 1u64;
        nat_add(a697, a698)?
    };
    Ok(f46({
        let (_, h699) = {
            let (_, h700) = {
                let (h701, _) = f118(Fn19::F119, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h701
            };
            h700
        };
        h699
    }))
}

pub fn f116(v0: u32) -> u64 {
    let m703 = {
        let a702 = v0;
        fixed_u64::convert(i128::from(a702))
    };
    match m703 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f117(v0: u64) -> u32 {
    let m707 = {
        let a706 = {
            let a704 = v0;
            let a705 = 4294967295u64;
            fixed_u64::bit_and(a704, a705)
        };
        fixed_u32::convert(i128::from(a706))
    };
    match m707 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f118(v0: Fn19, p712: u64, p713: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m711 = (p712, p713);
    {
        let mut m711 = m711;
        loop {
            let step = {
                let (v1, v2) = m711;
                let m708 = v1;
                if m708 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m708 - 1;
                    let m710 = {
                        let c709: Fn19 = v0.clone();
                        c709.apply(v2.clone())
                    };
                    match m710 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f120(v0)
}

pub fn f120(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h714, _) = v0.clone();
        h714
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h715, _) = {
            let (_, h716) = v0.clone();
            h716
        };
        h715
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h717) = {
            let (_, h718) = v0;
            h718
        };
        h717
    };
    let m719 = v1.clone();
    match m719.uncons() {
        None => {
            let m720 = v2;
            match m720.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m721 = v2.clone();
            match m721.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f121({
                        let (h722, _) = v8.clone();
                        h722
                    }, {
                        let (h723, _) = v6.clone();
                        h723
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

pub fn f121(v0: Bytes, v1: Bytes) -> bool {
    let a728 = {
        let a724 = v0;
        let a725 = v1;
        compare_bytes(a724, a725)
    };
    let a729 = {
        let a726 = Bytes::lit(&[]);
        let a727 = Bytes::lit(&[0x01]);
        compare_bytes(a726, a727)
    };
    equal(a728, a729)
}
