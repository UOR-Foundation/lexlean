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
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<bool>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Fn0 {
    F10(std::rc::Rc<Adt2>, std::rc::Rc<Adt10>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt7>, std::rc::Rc<Adt0>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn0::F10(k0, k1, k2, k3, k4, k5) => f10((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), (**k5).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn1::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F51(std::rc::Rc<Adt9>, std::rc::Rc<Adt10>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt7>),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn2::F51(k0, k1, k2, k3, k4) => f51((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0) -> R<List<bool>> {
    let v7: Adt7 = Adt7::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3.clone()), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v5.clone()));
    let v8: u64 = f1(v5.clone())?;
    let v9: Adt0 = f2(f2(f2(Adt0::C14, v3), v4), v1.clone());
    let v10: Adt8 = f3(v7.clone(), Adt8::C0(std::rc::Rc::new(v9), std::rc::Rc::new(Adt9::C0)), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v11: Adt0 = match v10.clone() {
        Adt8::C0(h1, _) => {
            (*h1).clone()
        }
    };
    let v12: Adt0 = f4(f5(f6(v1)?), v11.clone());
    let v13: Adt10 = f7(v7.clone(), v11.clone(), match v10.clone() {
        Adt8::C0(_, h2) => {
            (*h2).clone()
        }
    }, v12.clone(), Adt10::C0);
    let v14: Adt10 = f8(v13, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f6(v12.clone())?;
            let a4 = f6(v12.clone())?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    let v15: Adt11 = {
        let (h7, _) = f9(Fn0::F10(std::rc::Rc::new(v5), std::rc::Rc::new(v14), std::rc::Rc::new(v11), std::rc::Rc::new(v12), std::rc::Rc::new(v7), std::rc::Rc::new(v6)), {
            let a8 = 1u64;
            let a9 = v8;
            nat_add(a8, a9)?
        }, Adt11::C0(f11(v8), std::rc::Rc::new(match v10 {
            Adt8::C0(_, h10) => {
                (*h10).clone()
            }
        })))?;
        h7
    };
    match v15 {
        Adt11::C0(h11, _) => {
            Ok(h11)
        }
    }
}

pub fn f1(v0: Adt2) -> R<u64> {
    let m12 = v0;
    match m12 {
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
        Adt2::C14(_, r15) => {
            let v33 = (*r15).clone();
            let v34: u64 = f1(v33)?;
            let a13 = 1u64;
            let a14 = v34;
            nat_add(a13, a14)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0) -> Adt0 {
    let m16 = v1;
    match m16 {
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
        Adt0::C8(r17) => {
            let v3 = (*r17).clone();
            f2(v0, v3)
        }
        Adt0::C9(r18, r19) => {
            let v4 = (*r18).clone();
            let v5 = (*r19).clone();
            f2(f2(v0, v4), v5)
        }
        Adt0::C10(r20) => {
            let v6 = (*r20).clone();
            f2(v0, v6)
        }
        Adt0::C11(r21, r22) => {
            let v7 = (*r21).clone();
            let v8 = (*r22).clone();
            f2(f2(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r23, r24) => {
            let v10 = (*r23).clone();
            let v11 = (*r24).clone();
            let v12: Adt0 = f12(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f2(f2(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r25, r26) => {
            let v13 = (*r25).clone();
            let v14 = (*r26).clone();
            f2(f2(v0, v13), v14)
        }
    }
}

pub fn f3(v0: Adt7, p37: Adt8, p38: Adt2, p39: u64, v4: List<bool>, p40: Adt0) -> R<Adt8> {
    let m36 = (p37, (p38, (p39, p40)));
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m36;
                let m27 = v2;
                match m27 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C14(r34, r35) => {
                        let v34 = (*r34).clone();
                        let v35 = (*r35).clone();
                        let v37: bool = {
                            let m28 = f13(v4.clone(), v3);
                            match m28 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a29 = v36;
                                    bool_not(a29)
                                }
                            }
                        };
                        let v38: Adt8 = Adt8::C0(std::rc::Rc::new(f14(v0.clone(), match v1.clone() {
                            Adt8::C0(h30, _) => {
                                (*h30).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt8::C0(_, h31) => {
                                (*h31).clone()
                            }
                        }));
                        let v39: Adt8 = if v37 {
                            f15(v0.clone(), v38, v34, f16(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>((v39, (v35, ({
                            let a32 = v3;
                            let a33 = 1u64;
                            nat_add(a32, a33)?
                        }, f17(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Adt0, v1: Adt0) -> Adt0 {
    let m41 = v0;
    match m41 {
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
        Adt0::C15(r42, r43) => {
            let v12 = (*r42).clone();
            let v13 = (*r43).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f4(v13, v1)))
        }
    }
}

pub fn f5(v0: u64) -> Adt0 {
    let m44 = v0;
    if m44 == 0 {
        Adt0::C14
    } else {
        let v1 = m44 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f5(v1)))
    }
}

pub fn f6(v0: Adt0) -> R<u64> {
    let m45 = v0;
    match m45 {
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
        Adt0::C15(_, r48) => {
            let v12 = (*r48).clone();
            let v13: u64 = f6(v12)?;
            let a46 = 1u64;
            let a47 = v13;
            nat_add(a46, a47)
        }
    }
}

pub fn f7(v0: Adt7, v1: Adt0, v2: Adt9, p53: Adt0, p54: Adt10) -> Adt10 {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v3, v4) = m52;
                let m49 = v3;
                match m49 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C15(r50, r51) => {
                        let v15 = (*r50).clone();
                        let v16 = (*r51).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v16, f18(v1.clone(), v15.clone(), f19(v0.clone(), v2.clone(), v15), v4)))
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

pub fn f8(v0: Adt10, v1: u64) -> R<Adt10> {
    let (h55, _) = f20(Fn1::F21, v1, v0)?;
    Ok(h55)
}

pub fn f9(v0: Fn0, p60: u64, p61: Adt11) -> R<(Adt11, bool)> {
    let m59 = (p60, p61);
    {
        let mut m59 = m59;
        loop {
            let step = {
                let (v1, v2) = m59;
                let m56 = v1;
                if m56 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m56 - 1;
                    let m58 = {
                        let c57: Fn0 = v0.clone();
                        c57.apply(v2.clone())?
                    };
                    match m58 {
                        None => {
                            Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt11), R<(Adt11, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m59 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt2, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    f22(v4, v0, v2, v1, v3, v5, v6)
}

pub fn f11(v0: u64) -> List<bool> {
    let m62 = v0;
    if m62 == 0 {
        List::<bool>::nil()
    } else {
        let v1 = m62 - 1;
        List::cons(false, f11(v1))
    }
}

pub fn f12(v0: Adt0, v1: Adt0) -> Adt0 {
    if f23(v0.clone(), v1.clone()) {
        v0
    } else {
        f4(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f13(p66: List<bool>, p67: u64) -> Option<bool> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v0, v1) = m65;
                let m63 = v0;
                match m63.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m64 = v1;
                        if m64 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m64 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
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

pub fn f14(v0: Adt7, v1: Adt0, v2: u64) -> Adt0 {
    f2(f24(v1, f25(v0.clone(), v2)), f26(v0, v2))
}

pub fn f15(v0: Adt7, v1: Adt8, v2: Adt2, v3: Adt0) -> R<Adt8> {
    let m68 = v2;
    match m68 {
        Adt2::C0(r79, _) => {
            let v4 = (*r79).clone();
            Ok(Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h69, _) => {
                    (*h69).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h70) => {
                    (*h70).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r80, r81, r82) => {
            let v8 = (*r80).clone();
            let v9 = (*r81).clone();
            let v10 = (*r82).clone();
            let v11: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h71, _) => {
                    (*h71).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h72) => {
                    (*h72).clone()
                }
            }));
            let v12: Adt8 = f15(v0.clone(), v11, v9, f16(v3.clone()))?;
            f15(v0, v12, v10, f17(v3))
        }
        Adt2::C3(r83, r84, r85) => {
            let v13 = (*r83).clone();
            let v14 = (*r84).clone();
            let v15 = (*r85).clone();
            let v16: Adt8 = f15(v0.clone(), v1, v13, f16(v3.clone()))?;
            let v17: Adt8 = f15(v0.clone(), v16, v14, f17(v3.clone()))?;
            f15(v0, v17, v15, f27(v3))
        }
        Adt2::C4(r86, r87, r88) => {
            let v18 = (*r86).clone();
            let v19 = (*r87).clone();
            let v20 = (*r88).clone();
            let v21: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h73, _) => {
                    (*h73).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h74) => {
                    (*h74).clone()
                }
            }));
            let v22: Adt8 = f15(v0.clone(), v21, v19, f16(v3.clone()))?;
            f15(v0, v22, v20, f17(v3))
        }
        Adt2::C5(_, r89, r90) => {
            let v24 = (*r89).clone();
            let v25 = (*r90).clone();
            let v26: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h75, _) => {
                    (*h75).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h76) => {
                    (*h76).clone()
                }
            }));
            f15(v0, v26, v25, f16(v3))
        }
        Adt2::C6(_, r91) => {
            let v28 = (*r91).clone();
            f15(v0, v1, v28, f16(v3))
        }
        Adt2::C7(v29, r92) => {
            let v30 = (*r92).clone();
            let v31: Adt0 = f25(v0.clone(), v29);
            let v32: u64 = f1(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f28(v31.clone(), v32)), std::rc::Rc::new(f26(v0.clone(), v29)));
            let v34: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h77, _) => {
                    (*h77).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f29(match v1 {
                Adt8::C0(_, h78) => {
                    (*h78).clone()
                }
            }, v33, v29, f30(v31, v32))));
            f15(v0, v34, v30, f16(v3))
        }
        Adt2::C8(r93, r94) => {
            let v35 = (*r93).clone();
            let v36 = (*r94).clone();
            let v37: Adt8 = f15(v0.clone(), v1, v35, f16(v3.clone()))?;
            f15(v0, v37, v36, f17(v3))
        }
        Adt2::C9(_, r95) => {
            let v39 = (*r95).clone();
            f15(v0, v1, v39, f16(v3))
        }
        Adt2::C10(r96) => {
            let v40 = (*r96).clone();
            f15(v0, v1, v40, f16(v3))
        }
        Adt2::C11(r97) => {
            let v41 = (*r97).clone();
            f15(v0, v1, v41, f16(v3))
        }
        Adt2::C12(r98, _) => {
            let v42 = (*r98).clone();
            f15(v0, v1, v42, f16(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r99, r100) => {
            let v44 = (*r99).clone();
            let v45 = (*r100).clone();
            let v46: Adt8 = f15(v0.clone(), v1, v44, f16(v3.clone()))?;
            f15(v0, v46, v45, f17(v3))
        }
        Adt2::C15(_, _, r101) => {
            let v49 = (*r101).clone();
            f15(v0, v1, v49, f16(v3))
        }
    }
}

pub fn f16(v0: Adt0) -> Adt0 {
    f31(f32(v0))
}

pub fn f17(v0: Adt0) -> Adt0 {
    f31(f32(f32(v0)))
}

pub fn f18(v0: Adt0, v1: Adt0, p106: Adt0, p107: Adt10) -> Adt10 {
    let m105 = (p106, p107);
    {
        let mut m105 = m105;
        loop {
            let step = {
                let (v2, v3) = m105;
                let m102 = v2;
                match m102 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r103, r104) => {
                        let v14 = (*r103).clone();
                        let v15 = (*r104).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f33(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt7, v1: Adt9, v2: Adt0) -> Adt0 {
    let m108 = v2;
    match m108 {
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
            let m109 = f34(v0, v10);
            match m109 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r110, r111) => {
            let v12 = (*r110).clone();
            let v13 = (*r111).clone();
            f35(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f20(v0: Fn1, p116: u64, p117: Adt10) -> R<(Adt10, bool)> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v1, v2) = m115;
                let m112 = v1;
                if m112 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m112 - 1;
                    let m114 = {
                        let c113: Fn1 = v0.clone();
                        c113.apply(v2.clone())?
                    };
                    match m114 {
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
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt10) -> R<Option<Adt10>> {
    f36(v0)
}

pub fn f22(v0: Adt7, v1: Adt2, v2: Adt0, v3: Adt10, v4: Adt0, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    let v7: Adt0 = f37(v0.clone(), v2.clone(), match v6.clone() {
        Adt11::C0(_, h118) => {
            (*h118).clone()
        }
    }, v3, v4)?;
    let v8: List<bool> = f38(match v0.clone() {
        Adt7::C0(_, _, _, h119, _, _) => {
            (*h119).clone()
        }
    }, v7);
    if f39(v8.clone(), match v6 {
        Adt11::C0(h120, _) => {
            h120
        }
    }) {
        Ok(None::<Adt11>)
    } else {
        Ok(Some(Adt11::C0(v8.clone(), std::rc::Rc::new({
            let m122 = f3(v0, Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(Adt9::C0)), v1, 0u64, v8, v5)?;
            match m122 {
                Adt8::C0(_, h121) => {
                    (*h121).clone()
                }
            }
        }))))
    }
}

pub fn f23(p127: Adt0, v1: Adt0) -> bool {
    let m126 = p127;
    {
        let mut m126 = m126;
        loop {
            let step = {
                let v0 = m126;
                let m123 = v0;
                match m123 {
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
                    Adt0::C15(r124, r125) => {
                        let v12 = (*r124).clone();
                        let v13 = (*r125).clone();
                        if f40(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(p132: Adt0, p133: Adt0) -> Adt0 {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v0, v1) = m131;
                let m128 = v1;
                match m128 {
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
                    Adt0::C15(r129, r130) => {
                        let v2 = (*r129).clone();
                        let v3 = (*r130).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f2(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt7, v1: u64) -> Adt0 {
    let m134 = f41(v0, v1);
    match m134 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f26(v0: Adt7, v1: u64) -> Adt0 {
    let m136 = f42(match v0 {
        Adt7::C0(_, _, _, _, h135, _) => {
            (*h135).clone()
        }
    }, v1);
    match m136 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f27(v0: Adt0) -> Adt0 {
    f31(f32(f32(f32(v0))))
}

pub fn f28(p142: Adt0, p143: u64) -> Adt0 {
    let m141 = (p142, p143);
    {
        let mut m141 = m141;
        loop {
            let step = {
                let (v0, v1) = m141;
                let m137 = v0;
                match m137 {
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
                    Adt0::C15(r139, r140) => {
                        let v12 = (*r139).clone();
                        let v13 = (*r140).clone();
                        let m138 = v1;
                        if m138 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m138 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
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

pub fn f29(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f43(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f30(v0: Adt0, v1: u64) -> Adt0 {
    let m144 = v0;
    match m144 {
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
        Adt0::C15(r146, r147) => {
            let v12 = (*r146).clone();
            let v13 = (*r147).clone();
            let m145 = v1;
            if m145 == 0 {
                Adt0::C14
            } else {
                let v14 = m145 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f30(v13, v14)))
            }
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    let m148 = f44(v0);
    match m148 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    let m149 = f45(v0);
    match m149 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0, p154: Adt0, p155: Adt10) -> Adt10 {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v2, v3) = m153;
                let m150 = v2;
                match m150 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r151, r152) => {
                        let v14 = (*r151).clone();
                        let v15 = (*r152).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f46(v1.clone(), f47(v0.clone(), Adt0::C14, v14), v3)))
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

pub fn f34(v0: Adt7, v1: u64) -> Option<Adt0> {
    f42(match v0 {
        Adt7::C0(_, h156, _, _, _, _) => {
            (*h156).clone()
        }
    }, v1)
}

pub fn f35(v0: Adt9, v1: Adt0) -> Adt0 {
    let m157 = v0;
    match m157 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(r158, _, r159, r160) => {
            let v2 = (*r158).clone();
            let v4 = (*r159).clone();
            let v5 = (*r160).clone();
            if f40(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f35(v5, v1)))
            } else {
                f35(v5, v1)
            }
        }
    }
}

pub fn f36(v0: Adt10) -> R<Option<Adt10>> {
    let v1: Adt10 = f48(v0.clone(), v0.clone(), v0.clone());
    let a161 = f49(v1.clone())?;
    let a162 = f49(v0)?;
    if nat_eq(a161, a162) {
        Ok(None::<Adt10>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f37(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0) -> R<Adt0> {
    let (h163, _) = f50(Fn2::F51(std::rc::Rc::new(v2), std::rc::Rc::new(v3), std::rc::Rc::new(v1), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v0)), {
        let a164 = 1u64;
        let a165 = f6(v4)?;
        nat_add(a164, a165)?
    }, Adt0::C14)?;
    Ok(h163)
}

pub fn f38(v0: Adt0, v1: Adt0) -> List<bool> {
    let m166 = v0;
    match m166 {
        Adt0::C0 => {
            List::<bool>::nil()
        }
        Adt0::C1 => {
            List::<bool>::nil()
        }
        Adt0::C2 => {
            List::<bool>::nil()
        }
        Adt0::C3 => {
            List::<bool>::nil()
        }
        Adt0::C4(_) => {
            List::<bool>::nil()
        }
        Adt0::C5 => {
            List::<bool>::nil()
        }
        Adt0::C6 => {
            List::<bool>::nil()
        }
        Adt0::C7 => {
            List::<bool>::nil()
        }
        Adt0::C8(_) => {
            List::<bool>::nil()
        }
        Adt0::C9(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C10(_) => {
            List::<bool>::nil()
        }
        Adt0::C11(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C12(_) => {
            List::<bool>::nil()
        }
        Adt0::C13(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C14 => {
            List::<bool>::nil()
        }
        Adt0::C15(r167, r168) => {
            let v12 = (*r167).clone();
            let v13 = (*r168).clone();
            List::cons(f52(v12, v1.clone()), f38(v13, v1))
        }
    }
}

pub fn f39(p175: List<bool>, p176: List<bool>) -> bool {
    let m174 = (p175, p176);
    {
        let mut m174 = m174;
        loop {
            let step = {
                let (v0, v1) = m174;
                let m169 = v0;
                match m169.uncons() {
                    None => {
                        let m170 = v1;
                        match m170.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m171 = v1;
                        match m171.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a172 = v4;
                                let a173 = v6;
                                if equal(a172, a173) {
                                    Ok::<(List<bool>, List<bool>), bool>((v5, v7))
                                } else {
                                    Err::<(List<bool>, List<bool>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt0, v1: Adt0) -> bool {
    let m177 = v0;
    match m177 {
        Adt0::C0 => {
            let a178 = f53(v1);
            let a179 = 0u64;
            nat_eq(a178, a179)
        }
        Adt0::C1 => {
            let a180 = f53(v1);
            let a181 = 1u64;
            nat_eq(a180, a181)
        }
        Adt0::C2 => {
            let a182 = f53(v1);
            let a183 = 2u64;
            nat_eq(a182, a183)
        }
        Adt0::C3 => {
            let a184 = f53(v1);
            let a185 = 3u64;
            nat_eq(a184, a185)
        }
        Adt0::C4(v2) => {
            let a194 = f53(v1.clone());
            let a195 = 4u64;
            if nat_eq(a194, a195) {
                let a196 = f54(v2);
                let a197 = f54(f55(v1));
                nat_eq(a196, a197)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a186 = f53(v1);
            let a187 = 5u64;
            nat_eq(a186, a187)
        }
        Adt0::C6 => {
            let a188 = f53(v1);
            let a189 = 6u64;
            nat_eq(a188, a189)
        }
        Adt0::C7 => {
            let a190 = f53(v1);
            let a191 = 7u64;
            nat_eq(a190, a191)
        }
        Adt0::C8(r214) => {
            let v3 = (*r214).clone();
            let v4: bool = f40(v3, f31(v1.clone()));
            let a198 = f53(v1);
            let a199 = 8u64;
            if nat_eq(a198, a199) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r215, r216) => {
            let v5 = (*r215).clone();
            let v6 = (*r216).clone();
            let v7: bool = f40(v5, f31(v1.clone()));
            let v8: bool = f40(v6, f32(v1.clone()));
            let a200 = f53(v1);
            let a201 = 9u64;
            if nat_eq(a200, a201) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r217) => {
            let v9 = (*r217).clone();
            let v10: bool = f40(v9, f31(v1.clone()));
            let a202 = f53(v1);
            let a203 = 10u64;
            if nat_eq(a202, a203) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r218, r219) => {
            let v11 = (*r218).clone();
            let v12 = (*r219).clone();
            let v13: bool = f40(v11, f31(v1.clone()));
            let v14: bool = f40(v12, f32(v1.clone()));
            let a204 = f53(v1);
            let a205 = 11u64;
            if nat_eq(a204, a205) {
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
            let a206 = f53(v1.clone());
            let a207 = 12u64;
            if nat_eq(a206, a207) {
                let a208 = v15;
                let a209 = f56(v1);
                nat_eq(a208, a209)
            } else {
                false
            }
        }
        Adt0::C13(r220, r221) => {
            let v16 = (*r220).clone();
            let v17 = (*r221).clone();
            let v18: bool = f40(v16, f31(v1.clone()));
            let v19: bool = f40(v17, f32(v1.clone()));
            let a210 = f53(v1);
            let a211 = 13u64;
            if nat_eq(a210, a211) {
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
            let a192 = f53(v1);
            let a193 = 14u64;
            nat_eq(a192, a193)
        }
        Adt0::C15(r222, r223) => {
            let v20 = (*r222).clone();
            let v21 = (*r223).clone();
            let v22: bool = f40(v20, f31(v1.clone()));
            let v23: bool = f40(v21, f32(v1.clone()));
            let a212 = f53(v1);
            let a213 = 15u64;
            if nat_eq(a212, a213) {
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

pub fn f41(v0: Adt7, v1: u64) -> Option<Adt0> {
    f42(match v0 {
        Adt7::C0(_, _, _, h224, _, _) => {
            (*h224).clone()
        }
    }, v1)
}

pub fn f42(p230: Adt0, p231: u64) -> Option<Adt0> {
    let m229 = (p230, p231);
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v0, v1) = m229;
                let m225 = v0;
                match m225 {
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
                    Adt0::C15(r227, r228) => {
                        let v12 = (*r227).clone();
                        let v13 = (*r228).clone();
                        let m226 = v1;
                        if m226 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m226 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p238: Adt9, v1: Adt0, v2: u64) -> bool {
    let m237 = p238;
    {
        let mut m237 = m237;
        loop {
            let step = {
                let v0 = m237;
                let m232 = v0;
                match m232 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r235, v4, _, r236) => {
                        let v3 = (*r235).clone();
                        let v6 = (*r236).clone();
                        let a233 = v4;
                        let a234 = v2;
                        if nat_eq(a233, a234) {
                            if f40(v3, v1.clone()) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt0) -> Option<Adt0> {
    let m239 = v0;
    match m239 {
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
        Adt0::C8(r240) => {
            let v2 = (*r240).clone();
            Some(v2)
        }
        Adt0::C9(r241, _) => {
            let v3 = (*r241).clone();
            Some(v3)
        }
        Adt0::C10(r242) => {
            let v5 = (*r242).clone();
            Some(v5)
        }
        Adt0::C11(r243, _) => {
            let v6 = (*r243).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r244, _) => {
            let v9 = (*r244).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r245, _) => {
            let v11 = (*r245).clone();
            Some(v11)
        }
    }
}

pub fn f45(v0: Adt0) -> Option<Adt0> {
    let m246 = v0;
    match m246 {
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
        Adt0::C9(_, r247) => {
            let v4 = (*r247).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r248) => {
            let v7 = (*r248).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r249) => {
            let v10 = (*r249).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r250) => {
            let v12 = (*r250).clone();
            Some(v12)
        }
    }
}

pub fn f46(v0: Adt0, p255: Adt0, p256: Adt10) -> Adt10 {
    let m254 = (p255, p256);
    {
        let mut m254 = m254;
        loop {
            let step = {
                let (v1, v2) = m254;
                let m251 = v1;
                match m251 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C15(r252, r253) => {
                        let v13 = (*r252).clone();
                        let v14 = (*r253).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v14, f57(v2, v0.clone(), v13)))
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

pub fn f47(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m257 = v2;
    match m257 {
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
        Adt0::C8(r258) => {
            let v4 = (*r258).clone();
            f47(v0, v1, v4)
        }
        Adt0::C9(r259, r260) => {
            let v5 = (*r259).clone();
            let v6 = (*r260).clone();
            f47(v0.clone(), f47(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r261, r262) => {
            let v8 = (*r261).clone();
            let v9 = (*r262).clone();
            f47(v0.clone(), f47(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f12(v1, Adt0::C12(v10))
        }
        Adt0::C13(r263, r264) => {
            let v11 = (*r263).clone();
            let v12 = (*r264).clone();
            if f23(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f12(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f48(v0: Adt10, p270: Adt10, p271: Adt10) -> Adt10 {
    let m269 = (p270, p271);
    {
        let mut m269 = m269;
        loop {
            let step = {
                let (v1, v2) = m269;
                let m265 = v1;
                match m265 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v2)
                    }
                    Adt10::C1(r266, r267, r268) => {
                        let v3 = (*r266).clone();
                        let v4 = (*r267).clone();
                        let v5 = (*r268).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v5, f58(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Adt10) -> R<u64> {
    let m272 = v0;
    match m272 {
        Adt10::C0 => {
            Ok(0u64)
        }
        Adt10::C1(_, _, r275) => {
            let v3 = (*r275).clone();
            let a273 = 1u64;
            let a274 = f49(v3)?;
            nat_add(a273, a274)
        }
    }
}

pub fn f50(v0: Fn2, p280: u64, p281: Adt0) -> R<(Adt0, bool)> {
    let m279 = (p280, p281);
    {
        let mut m279 = m279;
        loop {
            let step = {
                let (v1, v2) = m279;
                let m276 = v1;
                if m276 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m276 - 1;
                    let m278 = {
                        let c277: Fn2 = v0.clone();
                        c277.apply(v2.clone())?
                    };
                    match m278 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt9, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0) -> R<Option<Adt0>> {
    f59(v4, v2, v0, v1, v3, v5)
}

pub fn f52(p286: Adt0, v1: Adt0) -> bool {
    let m285 = p286;
    {
        let mut m285 = m285;
        loop {
            let step = {
                let v0 = m285;
                let m282 = v0;
                match m282 {
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
                    Adt0::C15(r283, r284) => {
                        let v12 = (*r283).clone();
                        let v13 = (*r284).clone();
                        if f60(v12, v1.clone()) {
                            Ok::<Adt0, bool>(v13)
                        } else {
                            Err::<Adt0, bool>(true)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Adt0) -> u64 {
    let m287 = v0;
    match m287 {
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

pub fn f54(v0: Adt1) -> u64 {
    let m288 = v0;
    match m288 {
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

pub fn f55(v0: Adt0) -> Adt1 {
    let m289 = v0;
    match m289 {
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

pub fn f56(v0: Adt0) -> u64 {
    let m290 = v0;
    match m290 {
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

pub fn f57(v0: Adt10, v1: Adt0, v2: Adt0) -> Adt10 {
    if f61(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt10::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v2), std::rc::Rc::new(v0))
    }
}

pub fn f58(p296: Adt10, v1: Adt0, v2: Adt0, p297: Adt10) -> Adt10 {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v0, v3) = m295;
                let m291 = v0;
                match m291 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v3)
                    }
                    Adt10::C1(r292, r293, r294) => {
                        let v4 = (*r292).clone();
                        let v5 = (*r293).clone();
                        let v6 = (*r294).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v6, if f40(v4, v1.clone()) {
                            f57(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f62(v0, v1, v2, v3, v4, v5.clone());
    let a298 = f6(v6.clone())?;
    let a299 = f6(v5)?;
    if nat_eq(a298, a299) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f60(v0: Adt0, v1: Adt0) -> bool {
    let m300 = v0;
    match m300 {
        Adt0::C0 => {
            true
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
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r301, r302) => {
            let v4 = (*r301).clone();
            let v5 = (*r302).clone();
            if f60(v4, v1.clone()) {
                true
            } else {
                f60(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r303, r304) => {
            let v7 = (*r303).clone();
            let v8 = (*r304).clone();
            if f60(v7, v1.clone()) {
                f60(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f23(v1, Adt0::C12(v9))
        }
        Adt0::C13(r305, r306) => {
            let v10 = (*r305).clone();
            let v11 = (*r306).clone();
            f23(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f61(p312: Adt10, v1: Adt0, v2: Adt0) -> bool {
    let m311 = p312;
    {
        let mut m311 = m311;
        loop {
            let step = {
                let v0 = m311;
                let m307 = v0;
                match m307 {
                    Adt10::C0 => {
                        Err::<Adt10, bool>(false)
                    }
                    Adt10::C1(r308, r309, r310) => {
                        let v3 = (*r308).clone();
                        let v4 = (*r309).clone();
                        let v5 = (*r310).clone();
                        if f40(v3, v1.clone()) {
                            if f40(v4, v2.clone()) {
                                Err::<Adt10, bool>(true)
                            } else {
                                Ok::<Adt10, bool>(v5)
                            }
                        } else {
                            Ok::<Adt10, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, p317: Adt0, p318: Adt0) -> Adt0 {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v4, v5) = m316;
                let m313 = v4;
                match m313 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r314, r315) => {
                        let v16 = (*r314).clone();
                        let v17 = (*r315).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f23(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f63(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f19(v0.clone(), v2.clone(), v16.clone())) {
                                f12(v5, v16)
                            } else {
                                v5
                            }
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p323: Adt0) -> bool {
    let m322 = p323;
    {
        let mut m322 = m322;
        loop {
            let step = {
                let v4 = m322;
                let m319 = v4;
                match m319 {
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
                    Adt0::C15(r320, r321) => {
                        let v15 = (*r320).clone();
                        let v16 = (*r321).clone();
                        if f64(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
                        }
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

pub fn f64(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p328: Adt0) -> bool {
    let m327 = p328;
    {
        let mut m327 = m327;
        loop {
            let step = {
                let v4 = m327;
                let m324 = v4;
                match m324 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r325, r326) => {
                        let v15 = (*r325).clone();
                        let v16 = (*r326).clone();
                        if if f65(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f60(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
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

pub fn f65(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0) -> bool {
    f66(v2, v1, f47(v0, Adt0::C14, v3))
}

pub fn f66(v0: Adt0, v1: Adt10, p333: Adt0) -> bool {
    let m332 = p333;
    {
        let mut m332 = m332;
        loop {
            let step = {
                let v2 = m332;
                let m329 = v2;
                match m329 {
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
                    Adt0::C15(r330, r331) => {
                        let v13 = (*r330).clone();
                        let v14 = (*r331).clone();
                        if f40(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f61(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
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
