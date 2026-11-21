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
    C1(Adt1, Adt6, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2),
    C1(Adt3),
    C2(std::rc::Rc<Adt1>),
    C3(Adt4, std::rc::Rc<Adt1>),
    C4(u64, u64, std::rc::Rc<Adt1>),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C7(Adt5),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt3 {
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
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt7),
    C1(Adt8),
    C2(Adt9, std::rc::Rc<Adt6>),
    C3(Adt10, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C4(Adt11, u64),
    C5,
    C6(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
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
pub enum Adt8 {
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
pub enum Adt9 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt13, Adt14),
    C1(Adt15, Adt3, Adt14),
    C2(Adt16, std::rc::Rc<Adt12>, Adt14),
    C3(Adt17, std::rc::Rc<Adt12>, bool, Adt14),
    C4(Adt3, std::rc::Rc<Adt12>, bool, Adt14),
    C5(Adt19, std::rc::Rc<Adt12>, Adt14),
    C6(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>, Adt14),
    C7(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>, Adt14),
    C8(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>, Adt14),
    C9(std::rc::Rc<Adt12>),
    C10(Adt20, Adt3, Adt14),
    C11(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C12,
    C13(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C14(Adt1, bool, Adt6, std::rc::Rc<Adt12>, Adt14, std::rc::Rc<Adt12>),
    C15(Adt1, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C16(Adt3, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt8, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt5),
    C8,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Bytes, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64),
    C1(Adt18),
    C2(u64, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Adt6),
    C1,
    C2(Adt6, Adt6),
    C3(Adt6, Adt6),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt6),
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
    C0(bool, Adt0, Adt12, u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(bool, Adt12),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(bool, u64, Adt6, Adt1, Adt12, Adt0),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u64, Adt0, Adt12, Adt6, Adt6),
}

pub fn f0(v0: u64, v1: Adt0, v2: Adt6, v3: Adt12, v4: u64) -> R<Adt21> {
    let v5: Adt22 = f1(v3.clone(), v3.clone(), v0, 1u64);
    let a2 = match v5.clone() {
        Adt22::C0(h1, _) => {
            h1
        }
    };
    if if bool_not(a2) {
        true
    } else {
        f2(match v5.clone() {
            Adt22::C0(_, h3) => {
                h3
            }
        })
    } {
        Ok(Adt21::C0(false, v1, v3, v4))
    } else {
        let v6: Adt23 = f3(v1.clone(), match v5.clone() {
            Adt22::C0(_, h4) => {
                h4
            }
        }, 0u64, v4, 0u64)?;
        let a6 = match v6.clone() {
            Adt23::C0(h5, _, _, _, _, _) => {
                h5
            }
        };
        if if bool_not(a6) {
            true
        } else {
            let a8 = match v6.clone() {
                Adt23::C0(_, h7, _, _, _, _) => {
                    h7
                }
            };
            let a9 = 0u64;
            nat_eq(a8, a9)
        } {
            Ok(Adt21::C0(false, v1, v3, v4))
        } else {
            let v7: Adt12 = f4(v3.clone(), v3, Adt24::C0(v0, v1, match v5 {
                Adt22::C0(_, h10) => {
                    h10
                }
            }, match v6.clone() {
                Adt23::C0(_, _, h11, _, _, _) => {
                    h11
                }
            }, v2), 1u64)?;
            Ok(Adt21::C0(true, match v6.clone() {
                Adt23::C0(_, _, _, _, _, h12) => {
                    h12
                }
            }, Adt12::C11(std::rc::Rc::new(Adt12::C14(Adt1::C1(Adt3::C1({
                let a13 = v4;
                let a14 = 1u64;
                nat_add(a13, a14)?
            })), false, f5(0u64), std::rc::Rc::new(match v6.clone() {
                Adt23::C0(_, _, _, _, h15, _) => {
                    h15
                }
            }), f6(0u64), std::rc::Rc::new(Adt12::C12))), std::rc::Rc::new(Adt12::C16(Adt3::C1({
                let a16 = v4;
                let a17 = 1u64;
                nat_add(a16, a17)?
            }), std::rc::Rc::new(Adt12::C11(std::rc::Rc::new(Adt12::C14(match v6.clone() {
                Adt23::C0(_, _, _, h18, _, _) => {
                    h18
                }
            }, false, f5(0u64), std::rc::Rc::new(Adt12::C1(Adt15::C0, Adt3::C1({
                let a19 = v4;
                let a20 = 1u64;
                nat_add(a19, a20)?
            }), f6(0u64))), f6(0u64), std::rc::Rc::new(f7(v7.clone())))), std::rc::Rc::new(f8(v7))))))), {
                let a24 = {
                    let a21 = v4;
                    let a22 = 1u64;
                    nat_add(a21, a22)?
                };
                let a25 = match v6 {
                    Adt23::C0(_, h23, _, _, _, _) => {
                        h23
                    }
                };
                nat_add(a24, a25)?
            }))
        }
    }
}

pub fn f1(v0: Adt12, v1: Adt12, v2: u64, v3: u64) -> Adt22 {
    let m26 = v0;
    match m26 {
        Adt12::C0(_, _) => {
            let a27 = v3;
            let a28 = 0u64;
            if nat_eq(a27, a28) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C1(_, _, _) => {
            let a29 = v3;
            let a30 = 0u64;
            if nat_eq(a29, a30) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C2(_, _, _) => {
            let a31 = v3;
            let a32 = 0u64;
            if nat_eq(a31, a32) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C3(v12, r66, _, _) => {
            let v13 = (*r66).clone();
            let m33 = v12;
            match m33 {
                Adt17::C0(v16) => {
                    let a34 = v16;
                    let a35 = v2;
                    if nat_eq(a34, a35) {
                        if f9(v13.clone(), v2) {
                            Adt22::C0(false, Adt12::C12)
                        } else {
                            Adt22::C0(true, Adt12::C13(std::rc::Rc::new(v13), std::rc::Rc::new(Adt12::C12)))
                        }
                    } else {
                        if f9(v1, v2) {
                            Adt22::C0(false, Adt12::C12)
                        } else {
                            Adt22::C0(true, Adt12::C12)
                        }
                    }
                }
                Adt17::C1(_) => {
                    if f9(v1, v2) {
                        Adt22::C0(false, Adt12::C12)
                    } else {
                        Adt22::C0(true, Adt12::C12)
                    }
                }
                Adt17::C2(_, _) => {
                    if f9(v1, v2) {
                        Adt22::C0(false, Adt12::C12)
                    } else {
                        Adt22::C0(true, Adt12::C12)
                    }
                }
            }
        }
        Adt12::C4(_, _, _, _) => {
            let a36 = v3;
            let a37 = 0u64;
            if nat_eq(a36, a37) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C5(_, _, _) => {
            let a38 = v3;
            let a39 = 0u64;
            if nat_eq(a38, a39) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C6(_, _, _) => {
            let a40 = v3;
            let a41 = 0u64;
            if nat_eq(a40, a41) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C7(r67, r68, r69, _) => {
            let v30 = (*r67).clone();
            let v31 = (*r68).clone();
            let v32 = (*r69).clone();
            let v34: Adt22 = f1(v31.clone(), v31, v2, 1u64);
            let v35: Adt22 = f1(v32.clone(), v32, v2, 1u64);
            Adt22::C0({
                let a42 = f9(v30, v2);
                if bool_not(a42) {
                    if match v34.clone() {
                        Adt22::C0(h43, _) => {
                            h43
                        }
                    } {
                        match v35.clone() {
                            Adt22::C0(h44, _) => {
                                h44
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }, f10(match v34 {
                Adt22::C0(_, h45) => {
                    h45
                }
            }, match v35 {
                Adt22::C0(_, h46) => {
                    h46
                }
            }))
        }
        Adt12::C8(_, r70, _) => {
            let v37 = (*r70).clone();
            let v39: Adt22 = f1(v37.clone(), v37, v2, 2u64);
            Adt22::C0(match v39.clone() {
                Adt22::C0(h47, _) => {
                    h47
                }
            }, match v39 {
                Adt22::C0(_, h48) => {
                    h48
                }
            })
        }
        Adt12::C9(_) => {
            let a49 = v3;
            let a50 = 0u64;
            if nat_eq(a49, a50) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C10(_, _, _) => {
            let a51 = v3;
            let a52 = 0u64;
            if nat_eq(a51, a52) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C11(r71, r72) => {
            let v44 = (*r71).clone();
            let v45 = (*r72).clone();
            let v46: Adt22 = f1(v45.clone(), v45, v2, 0u64);
            Adt22::C0({
                let a53 = f9(v44, v2);
                if bool_not(a53) {
                    match v46.clone() {
                        Adt22::C0(h54, _) => {
                            h54
                        }
                    }
                } else {
                    false
                }
            }, match v46 {
                Adt22::C0(_, h55) => {
                    h55
                }
            })
        }
        Adt12::C12 => {
            Adt22::C0(true, Adt12::C12)
        }
        Adt12::C13(_, _) => {
            let a56 = v3;
            let a57 = 0u64;
            if nat_eq(a56, a57) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C14(_, _, _, _, _, _) => {
            let a58 = v3;
            let a59 = 0u64;
            if nat_eq(a58, a59) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
        Adt12::C15(_, r73, r74) => {
            let v56 = (*r73).clone();
            let v57 = (*r74).clone();
            let v58: Adt22 = f1(v56.clone(), v56, v2, 1u64);
            let v59: Adt22 = f1(v57.clone(), v57, v2, 2u64);
            Adt22::C0(if match v58.clone() {
                Adt22::C0(h60, _) => {
                    h60
                }
            } {
                match v59.clone() {
                    Adt22::C0(h61, _) => {
                        h61
                    }
                }
            } else {
                false
            }, f10(match v58 {
                Adt22::C0(_, h62) => {
                    h62
                }
            }, match v59 {
                Adt22::C0(_, h63) => {
                    h63
                }
            }))
        }
        Adt12::C16(_, _) => {
            let a64 = v3;
            let a65 = 0u64;
            if nat_eq(a64, a65) {
                if f9(v1, v2) {
                    Adt22::C0(false, Adt12::C12)
                } else {
                    Adt22::C0(true, Adt12::C12)
                }
            } else {
                Adt22::C0(false, Adt12::C12)
            }
        }
    }
}

pub fn f2(v0: Adt12) -> bool {
    let m75 = v0;
    match m75 {
        Adt12::C0(_, _) => {
            false
        }
        Adt12::C1(_, _, _) => {
            false
        }
        Adt12::C2(_, _, _) => {
            false
        }
        Adt12::C3(_, _, _, _) => {
            false
        }
        Adt12::C4(_, _, _, _) => {
            false
        }
        Adt12::C5(_, _, _) => {
            false
        }
        Adt12::C6(_, _, _) => {
            false
        }
        Adt12::C7(_, _, _, _) => {
            false
        }
        Adt12::C8(_, _, _) => {
            false
        }
        Adt12::C9(_) => {
            false
        }
        Adt12::C10(_, _, _) => {
            false
        }
        Adt12::C11(_, _) => {
            false
        }
        Adt12::C12 => {
            true
        }
        Adt12::C13(_, _) => {
            false
        }
        Adt12::C14(_, _, _, _, _, _) => {
            false
        }
        Adt12::C15(_, _, _) => {
            false
        }
        Adt12::C16(_, _) => {
            false
        }
    }
}

pub fn f3(v0: Adt0, v1: Adt12, v2: u64, v3: u64, v4: u64) -> R<Adt23> {
    let m76 = v0;
    match m76 {
        Adt0::C0 => {
            Ok(Adt23::C0(true, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt0::C1(v5, v6, r85) => {
            let v7 = (*r85).clone();
            let v8: bool = f11(v5.clone(), v1.clone(), v2);
            f12(v5.clone(), v5, v6, f3(v7, v1, {
                let a77 = v2;
                let a78 = 1u64;
                nat_add(a77, a78)?
            }, v3, if v8 {
                let a79 = v4;
                let a80 = 1u64;
                nat_add(a79, a80)?
            } else {
                v4
            })?, v8, {
                let a83 = v3;
                let a84 = {
                    let a81 = 2u64;
                    let a82 = v4;
                    nat_add(a81, a82)?
                };
                nat_add(a83, a84)?
            })
        }
    }
}

pub fn f4(v0: Adt12, v1: Adt12, v2: Adt24, v3: u64) -> R<Adt12> {
    let m86 = v0;
    match m86 {
        Adt12::C0(_, _) => {
            let a87 = v3;
            let a88 = 0u64;
            if nat_eq(a87, a88) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h89, _) => {
                        h89
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h90) => {
                        h90
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C1(_, _, _) => {
            let a91 = v3;
            let a92 = 0u64;
            if nat_eq(a91, a92) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h93, _) => {
                        h93
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h94) => {
                        h94
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C2(_, _, _) => {
            let a95 = v3;
            let a96 = 0u64;
            if nat_eq(a95, a96) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h97, _) => {
                        h97
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h98) => {
                        h98
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C3(v12, r155, _, v15) => {
            let v13 = (*r155).clone();
            let m99 = v12;
            match m99 {
                Adt17::C0(v16) => {
                    let a101 = v16;
                    let a102 = match v2.clone() {
                        Adt24::C0(h100, _, _, _, _) => {
                            h100
                        }
                    };
                    if nat_eq(a101, a102) {
                        Ok(Adt12::C5(Adt19::C2(match v2.clone() {
                            Adt24::C0(_, _, _, h103, _) => {
                                h103
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, _, h104) => {
                                h104
                            }
                        }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(f13(match v2.clone() {
                            Adt24::C0(_, h105, _, _, _) => {
                                h105
                            }
                        }, match v2 {
                            Adt24::C0(_, _, h106, _, _) => {
                                h106
                            }
                        }, v13, 0u64, v15.clone())?), std::rc::Rc::new(Adt12::C12))), v15))
                    } else {
                        Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                            Adt24::C0(_, _, _, h107, _) => {
                                h107
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, _, h108) => {
                                h108
                            }
                        }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
                    }
                }
                Adt17::C1(_) => {
                    Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                        Adt24::C0(_, _, _, h109, _) => {
                            h109
                        }
                    }, match v2 {
                        Adt24::C0(_, _, _, _, h110) => {
                            h110
                        }
                    }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
                }
                Adt17::C2(_, _) => {
                    Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                        Adt24::C0(_, _, _, h111, _) => {
                            h111
                        }
                    }, match v2 {
                        Adt24::C0(_, _, _, _, h112) => {
                            h112
                        }
                    }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
                }
            }
        }
        Adt12::C4(_, _, _, _) => {
            let a113 = v3;
            let a114 = 0u64;
            if nat_eq(a113, a114) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h115, _) => {
                        h115
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h116) => {
                        h116
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C5(_, _, _) => {
            let a117 = v3;
            let a118 = 0u64;
            if nat_eq(a117, a118) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h119, _) => {
                        h119
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h120) => {
                        h120
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C6(_, _, _) => {
            let a121 = v3;
            let a122 = 0u64;
            if nat_eq(a121, a122) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h123, _) => {
                        h123
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h124) => {
                        h124
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C7(r156, r157, r158, v33) => {
            let v30 = (*r156).clone();
            let v31 = (*r157).clone();
            let v32 = (*r158).clone();
            Ok(Adt12::C7(std::rc::Rc::new(v30), std::rc::Rc::new(f4(v31.clone(), v31, v2.clone(), 1u64)?), std::rc::Rc::new(f4(v32.clone(), v32, v2, 1u64)?), v33))
        }
        Adt12::C8(r159, r160, v36) => {
            let v34 = (*r159).clone();
            let v35 = (*r160).clone();
            Ok(Adt12::C8(std::rc::Rc::new(v34), std::rc::Rc::new(f4(v35.clone(), v35, v2, 2u64)?), v36))
        }
        Adt12::C9(_) => {
            let a125 = v3;
            let a126 = 0u64;
            if nat_eq(a125, a126) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h127, _) => {
                        h127
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h128) => {
                        h128
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C10(_, _, _) => {
            let a129 = v3;
            let a130 = 0u64;
            if nat_eq(a129, a130) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h131, _) => {
                        h131
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h132) => {
                        h132
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C11(r161, r162) => {
            let v41 = (*r161).clone();
            let v42 = (*r162).clone();
            let a133 = v3;
            let a134 = 1u64;
            if nat_eq(a133, a134) {
                Ok(Adt12::C11(std::rc::Rc::new(v41), std::rc::Rc::new(f4(v42.clone(), v42, v2, 0u64)?)))
            } else {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h135, _) => {
                        h135
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h136) => {
                        h136
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            }
        }
        Adt12::C12 => {
            let a137 = v3;
            let a138 = 0u64;
            if nat_eq(a137, a138) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h139, _) => {
                        h139
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h140) => {
                        h140
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C13(_, _) => {
            let a141 = v3;
            let a142 = 0u64;
            if nat_eq(a141, a142) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h143, _) => {
                        h143
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h144) => {
                        h144
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C14(_, _, _, _, _, _) => {
            let a145 = v3;
            let a146 = 0u64;
            if nat_eq(a145, a146) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h147, _) => {
                        h147
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h148) => {
                        h148
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C15(v51, r163, r164) => {
            let v52 = (*r163).clone();
            let v53 = (*r164).clone();
            let a149 = v3;
            let a150 = 2u64;
            if nat_eq(a149, a150) {
                Ok(Adt12::C15(v51, std::rc::Rc::new(f4(v52.clone(), v52, v2.clone(), 1u64)?), std::rc::Rc::new(f4(v53.clone(), v53, v2, 2u64)?)))
            } else {
                Ok(v1)
            }
        }
        Adt12::C16(_, _) => {
            let a151 = v3;
            let a152 = 0u64;
            if nat_eq(a151, a152) {
                Ok(Adt12::C5(Adt19::C3(match v2.clone() {
                    Adt24::C0(_, _, _, h153, _) => {
                        h153
                    }
                }, match v2 {
                    Adt24::C0(_, _, _, _, h154) => {
                        h154
                    }
                }), std::rc::Rc::new(Adt12::C13(std::rc::Rc::new(v1), std::rc::Rc::new(Adt12::C12))), f6(0u64)))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f5(_: u64) -> Adt6 {
    Adt6::C0(Adt7::C0)
}

pub fn f6(_: u64) -> Adt14 {
    Adt14::C0({
        let a165 = Str::lit("expr:call");
        utf8_encode(a165)
    }, None::<Adt8>)
}

pub fn f7(v0: Adt12) -> Adt12 {
    let m166 = v0;
    match m166 {
        Adt12::C0(_, _) => {
            Adt12::C12
        }
        Adt12::C1(_, _, _) => {
            Adt12::C12
        }
        Adt12::C2(_, _, _) => {
            Adt12::C12
        }
        Adt12::C3(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C4(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C5(_, _, _) => {
            Adt12::C12
        }
        Adt12::C6(_, _, _) => {
            Adt12::C12
        }
        Adt12::C7(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C8(_, _, _) => {
            Adt12::C12
        }
        Adt12::C9(_) => {
            Adt12::C12
        }
        Adt12::C10(_, _, _) => {
            Adt12::C12
        }
        Adt12::C11(r167, _) => {
            (*r167).clone()
        }
        Adt12::C12 => {
            Adt12::C12
        }
        Adt12::C13(_, _) => {
            Adt12::C12
        }
        Adt12::C14(_, _, _, _, _, _) => {
            Adt12::C12
        }
        Adt12::C15(_, _, _) => {
            Adt12::C12
        }
        Adt12::C16(_, _) => {
            Adt12::C12
        }
    }
}

pub fn f8(v0: Adt12) -> Adt12 {
    let m168 = v0;
    match m168 {
        Adt12::C0(_, _) => {
            Adt12::C12
        }
        Adt12::C1(_, _, _) => {
            Adt12::C12
        }
        Adt12::C2(_, _, _) => {
            Adt12::C12
        }
        Adt12::C3(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C4(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C5(_, _, _) => {
            Adt12::C12
        }
        Adt12::C6(_, _, _) => {
            Adt12::C12
        }
        Adt12::C7(_, _, _, _) => {
            Adt12::C12
        }
        Adt12::C8(_, _, _) => {
            Adt12::C12
        }
        Adt12::C9(_) => {
            Adt12::C12
        }
        Adt12::C10(_, _, _) => {
            Adt12::C12
        }
        Adt12::C11(_, r169) => {
            (*r169).clone()
        }
        Adt12::C12 => {
            Adt12::C12
        }
        Adt12::C13(_, _) => {
            Adt12::C12
        }
        Adt12::C14(_, _, _, _, _, _) => {
            Adt12::C12
        }
        Adt12::C15(_, _, _) => {
            Adt12::C12
        }
        Adt12::C16(_, _) => {
            Adt12::C12
        }
    }
}

pub fn f9(v0: Adt12, v1: u64) -> bool {
    let m170 = v0;
    match m170 {
        Adt12::C0(_, _) => {
            false
        }
        Adt12::C1(_, _, _) => {
            false
        }
        Adt12::C2(_, r174, _) => {
            let v8 = (*r174).clone();
            f9(v8, v1)
        }
        Adt12::C3(v10, r175, _, _) => {
            let v11 = (*r175).clone();
            let m171 = v10;
            match m171 {
                Adt17::C0(v14) => {
                    let a172 = v14;
                    let a173 = v1;
                    if nat_eq(a172, a173) {
                        true
                    } else {
                        f9(v11, v1)
                    }
                }
                Adt17::C1(_) => {
                    f9(v11, v1)
                }
                Adt17::C2(_, _) => {
                    f9(v11, v1)
                }
            }
        }
        Adt12::C4(_, r176, _, _) => {
            let v19 = (*r176).clone();
            f9(v19, v1)
        }
        Adt12::C5(_, r177, _) => {
            let v23 = (*r177).clone();
            f9(v23, v1)
        }
        Adt12::C6(r178, r179, _) => {
            let v25 = (*r178).clone();
            let v26 = (*r179).clone();
            if f9(v25, v1) {
                true
            } else {
                f9(v26, v1)
            }
        }
        Adt12::C7(r180, r181, r182, _) => {
            let v28 = (*r180).clone();
            let v29 = (*r181).clone();
            let v30 = (*r182).clone();
            if f9(v28, v1) {
                true
            } else {
                if f9(v29, v1) {
                    true
                } else {
                    f9(v30, v1)
                }
            }
        }
        Adt12::C8(r183, r184, _) => {
            let v32 = (*r183).clone();
            let v33 = (*r184).clone();
            if f9(v32, v1) {
                true
            } else {
                f9(v33, v1)
            }
        }
        Adt12::C9(r185) => {
            let v35 = (*r185).clone();
            f9(v35, v1)
        }
        Adt12::C10(_, _, _) => {
            false
        }
        Adt12::C11(r186, r187) => {
            let v39 = (*r186).clone();
            let v40 = (*r187).clone();
            if f9(v39, v1) {
                true
            } else {
                f9(v40, v1)
            }
        }
        Adt12::C12 => {
            false
        }
        Adt12::C13(r188, r189) => {
            let v41 = (*r188).clone();
            let v42 = (*r189).clone();
            if f9(v41, v1) {
                true
            } else {
                f9(v42, v1)
            }
        }
        Adt12::C14(_, _, _, r190, _, r191) => {
            let v46 = (*r190).clone();
            let v48 = (*r191).clone();
            if f9(v46, v1) {
                true
            } else {
                f9(v48, v1)
            }
        }
        Adt12::C15(_, r192, r193) => {
            let v50 = (*r192).clone();
            let v51 = (*r193).clone();
            if f9(v50, v1) {
                true
            } else {
                f9(v51, v1)
            }
        }
        Adt12::C16(_, r194) => {
            let v53 = (*r194).clone();
            f9(v53, v1)
        }
    }
}

pub fn f10(v0: Adt12, v1: Adt12) -> Adt12 {
    let m195 = v0;
    match m195 {
        Adt12::C0(_, _) => {
            v1
        }
        Adt12::C1(_, _, _) => {
            v1
        }
        Adt12::C2(_, _, _) => {
            v1
        }
        Adt12::C3(_, _, _, _) => {
            v1
        }
        Adt12::C4(_, _, _, _) => {
            v1
        }
        Adt12::C5(_, _, _) => {
            v1
        }
        Adt12::C6(_, _, _) => {
            v1
        }
        Adt12::C7(_, _, _, _) => {
            v1
        }
        Adt12::C8(_, _, _) => {
            v1
        }
        Adt12::C9(_) => {
            v1
        }
        Adt12::C10(_, _, _) => {
            v1
        }
        Adt12::C11(_, _) => {
            v1
        }
        Adt12::C12 => {
            v1
        }
        Adt12::C13(r196, r197) => {
            let v37 = (*r196).clone();
            let v38 = (*r197).clone();
            Adt12::C13(std::rc::Rc::new(v37), std::rc::Rc::new(f10(v38, v1)))
        }
        Adt12::C14(_, _, _, _, _, _) => {
            v1
        }
        Adt12::C15(_, _, _) => {
            v1
        }
        Adt12::C16(_, _) => {
            v1
        }
    }
}

pub fn f11(v0: Adt1, v1: Adt12, v2: u64) -> bool {
    let m198 = v0;
    match m198 {
        Adt1::C0(_) => {
            false
        }
        Adt1::C1(v4) => {
            let a199 = f14(v1, v2, v4);
            bool_not(a199)
        }
        Adt1::C2(_) => {
            false
        }
        Adt1::C3(_, _) => {
            false
        }
        Adt1::C4(_, _, _) => {
            false
        }
        Adt1::C5 => {
            false
        }
        Adt1::C6(_, _) => {
            false
        }
        Adt1::C7(_) => {
            false
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt1, v2: Adt6, v3: Adt23, v4: bool, v5: u64) -> R<Adt23> {
    let m200 = v0;
    match m200 {
        Adt1::C0(_) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C1(_) => {
            if v4 {
                Ok(Adt23::C0(match v3.clone() {
                    Adt23::C0(h201, _, _, _, _, _) => {
                        h201
                    }
                }, {
                    let a203 = match v3.clone() {
                        Adt23::C0(_, h202, _, _, _, _) => {
                            h202
                        }
                    };
                    let a204 = 1u64;
                    nat_add(a203, a204)?
                }, {
                    let a206 = match v3.clone() {
                        Adt23::C0(_, h205, _, _, _, _) => {
                            h205
                        }
                    };
                    let a207 = 0u64;
                    if nat_eq(a206, a207) {
                        v2.clone()
                    } else {
                        Adt6::C3(Adt10::C1, std::rc::Rc::new(v2.clone()), std::rc::Rc::new(match v3.clone() {
                            Adt23::C0(_, _, h208, _, _, _) => {
                                h208
                            }
                        }))
                    }
                }, {
                    let a210 = match v3.clone() {
                        Adt23::C0(_, h209, _, _, _, _) => {
                            h209
                        }
                    };
                    let a211 = 0u64;
                    if nat_eq(a210, a211) {
                        v1
                    } else {
                        Adt1::C2(std::rc::Rc::new(Adt1::C6(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C6(std::rc::Rc::new(match v3.clone() {
                            Adt23::C0(_, _, _, h212, _, _) => {
                                h212
                            }
                        }), std::rc::Rc::new(Adt1::C5))))))
                    }
                }, {
                    let a214 = match v3.clone() {
                        Adt23::C0(_, h213, _, _, _, _) => {
                            h213
                        }
                    };
                    let a215 = 0u64;
                    if nat_eq(a214, a215) {
                        Adt12::C1(Adt15::C0, Adt3::C7(v5), f6(0u64))
                    } else {
                        Adt12::C6(std::rc::Rc::new(Adt12::C1(Adt15::C0, Adt3::C7(v5), f6(0u64))), std::rc::Rc::new(match v3.clone() {
                            Adt23::C0(_, _, _, _, h216, _) => {
                                h216
                            }
                        }), f6(0u64))
                    }
                }, Adt0::C1(Adt1::C1(Adt3::C7(v5)), v2, std::rc::Rc::new(match v3 {
                    Adt23::C0(_, _, _, _, _, h217) => {
                        h217
                    }
                }))))
            } else {
                Ok(Adt23::C0(match v3.clone() {
                    Adt23::C0(h218, _, _, _, _, _) => {
                        h218
                    }
                }, match v3.clone() {
                    Adt23::C0(_, h219, _, _, _, _) => {
                        h219
                    }
                }, match v3.clone() {
                    Adt23::C0(_, _, h220, _, _, _) => {
                        h220
                    }
                }, match v3.clone() {
                    Adt23::C0(_, _, _, h221, _, _) => {
                        h221
                    }
                }, match v3.clone() {
                    Adt23::C0(_, _, _, _, h222, _) => {
                        h222
                    }
                }, Adt0::C1(v1, v2, std::rc::Rc::new(match v3 {
                    Adt23::C0(_, _, _, _, _, h223) => {
                        h223
                    }
                }))))
            }
        }
        Adt1::C2(_) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C3(_, _) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C4(_, _, _) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C5 => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C6(_, _) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
        Adt1::C7(_) => {
            Ok(Adt23::C0(false, 0u64, Adt6::C5, Adt1::C5, Adt12::C12, Adt0::C0))
        }
    }
}

pub fn f13(v0: Adt0, v1: Adt12, v2: Adt12, v3: u64, v4: Adt14) -> R<Adt12> {
    let m224 = v0;
    match m224 {
        Adt0::C0 => {
            Ok(Adt12::C12)
        }
        Adt0::C1(v5, _, r227) => {
            let v7 = (*r227).clone();
            Ok(f15(f11(v5, v1.clone(), v3), v2.clone(), v3, f13(v7, v1, v2, {
                let a225 = v3;
                let a226 = 1u64;
                nat_add(a225, a226)?
            }, v4.clone())?, v4))
        }
    }
}

pub fn f14(p232: Adt12, v1: u64, v2: Adt3) -> bool {
    let m231 = p232;
    {
        let mut m231 = m231;
        loop {
            let step = {
                let v0 = m231;
                let m228 = v0;
                match m228 {
                    Adt12::C0(_, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C1(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C2(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C3(_, _, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C4(_, _, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C5(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C6(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C7(_, _, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C8(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C9(_) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C10(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C11(_, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C12 => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C13(r229, r230) => {
                        let v38 = (*r229).clone();
                        let v39 = (*r230).clone();
                        if f16(f17(v38, v1), v2.clone()) {
                            Ok::<Adt12, bool>(v39)
                        } else {
                            Err::<Adt12, bool>(false)
                        }
                    }
                    Adt12::C14(_, _, _, _, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C15(_, _, _) => {
                        Err::<Adt12, bool>(true)
                    }
                    Adt12::C16(_, _) => {
                        Err::<Adt12, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: bool, v1: Adt12, v2: u64, v3: Adt12, v4: Adt14) -> Adt12 {
    if v0 {
        if f2(v3.clone()) {
            f17(v1, v2)
        } else {
            Adt12::C6(std::rc::Rc::new(f17(v1, v2)), std::rc::Rc::new(v3), v4)
        }
    } else {
        v3
    }
}

pub fn f16(v0: Adt12, v1: Adt3) -> bool {
    let m233 = v0;
    match m233 {
        Adt12::C0(_, _) => {
            false
        }
        Adt12::C1(v4, v5, _) => {
            let m234 = v4;
            match m234 {
                Adt15::C0 => {
                    f18(v5, v1)
                }
                Adt15::C1 => {
                    f18(v5, v1)
                }
                Adt15::C2 => {
                    f18(v5, v1)
                }
                Adt15::C3 => {
                    f18(v5, v1)
                }
                Adt15::C4 => {
                    false
                }
            }
        }
        Adt12::C2(_, _, _) => {
            false
        }
        Adt12::C3(_, _, _, _) => {
            false
        }
        Adt12::C4(_, _, _, _) => {
            false
        }
        Adt12::C5(_, _, _) => {
            false
        }
        Adt12::C6(_, _, _) => {
            false
        }
        Adt12::C7(_, _, _, _) => {
            false
        }
        Adt12::C8(_, _, _) => {
            false
        }
        Adt12::C9(_) => {
            false
        }
        Adt12::C10(_, _, _) => {
            false
        }
        Adt12::C11(_, _) => {
            false
        }
        Adt12::C12 => {
            false
        }
        Adt12::C13(_, _) => {
            false
        }
        Adt12::C14(_, _, _, _, _, _) => {
            false
        }
        Adt12::C15(_, _, _) => {
            false
        }
        Adt12::C16(_, _) => {
            false
        }
    }
}

pub fn f17(p240: Adt12, p241: u64) -> Adt12 {
    let m239 = (p240, p241);
    {
        let mut m239 = m239;
        loop {
            let step = {
                let (v0, v1) = m239;
                let m235 = v0;
                match m235 {
                    Adt12::C0(_, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C1(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C2(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C3(_, _, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C4(_, _, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C5(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C6(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C7(_, _, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C8(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C9(_) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C10(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C11(_, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C12 => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C13(r237, r238) => {
                        let v37 = (*r237).clone();
                        let v38 = (*r238).clone();
                        let m236 = v1;
                        if m236 == 0 {
                            Err::<(Adt12, u64), Adt12>(v37)
                        } else {
                            let v39 = m236 - 1;
                            Ok::<(Adt12, u64), Adt12>((v38, v39))
                        }
                    }
                    Adt12::C14(_, _, _, _, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C15(_, _, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                    Adt12::C16(_, _) => {
                        Err::<(Adt12, u64), Adt12>(Adt12::C12)
                    }
                }
            };
            match step {
                Ok(next) => { m239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt3, v1: Adt3) -> bool {
    let a242 = f19(v0.clone());
    let a243 = f19(v1.clone());
    if nat_eq(a242, a243) {
        let a244 = f20(v0);
        let a245 = f20(v1);
        nat_eq(a244, a245)
    } else {
        false
    }
}

pub fn f19(v0: Adt3) -> u64 {
    let m246 = v0;
    match m246 {
        Adt3::C0(_) => {
            0u64
        }
        Adt3::C1(_) => {
            1u64
        }
        Adt3::C2(_) => {
            2u64
        }
        Adt3::C3(_) => {
            3u64
        }
        Adt3::C4(_) => {
            4u64
        }
        Adt3::C5(_) => {
            5u64
        }
        Adt3::C6(_) => {
            6u64
        }
        Adt3::C7(_) => {
            7u64
        }
        Adt3::C8(_) => {
            8u64
        }
        Adt3::C9(_) => {
            9u64
        }
    }
}

pub fn f20(v0: Adt3) -> u64 {
    let m247 = v0;
    match m247 {
        Adt3::C0(v1) => {
            v1
        }
        Adt3::C1(v2) => {
            v2
        }
        Adt3::C2(v3) => {
            v3
        }
        Adt3::C3(v4) => {
            v4
        }
        Adt3::C4(v5) => {
            v5
        }
        Adt3::C5(v6) => {
            v6
        }
        Adt3::C6(v7) => {
            v7
        }
        Adt3::C7(v8) => {
            v8
        }
        Adt3::C8(v9) => {
            v9
        }
        Adt3::C9(_) => {
            0u64
        }
    }
}
