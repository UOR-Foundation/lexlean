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
    C0(Bytes, Bytes, u64, std::rc::Rc<Adt0>, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(std::rc::Rc<Adt0>, List<Option<Adt0>>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(std::rc::Rc<Adt0>, Option<Bytes>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<(Bytes, Adt0)>, u64, List<Bytes>, u64, bool, List<Adt2>, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Option<Bytes>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt9>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<Adt9>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(std::rc::Rc<Adt0>, List<(Bytes, Bytes)>, bool, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> Option<Adt4> {
        match self {
            Fn0::F6 => f6(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F35(Bytes, Bytes),
    F37(Bytes, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Adt2>, p1: Adt2) -> Option<Adt2> {
        match self {
            Fn1::F35(k0, k1) => f35(k0.clone(), k1.clone(), p0, p1),
            Fn1::F37(k0, k1) => f37(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F40(Bytes),
    F92(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Adt0>, p1: (Bytes, Adt0)) -> Option<Adt0> {
        match self {
            Fn2::F40(k0) => f40(k0.clone(), p0, p1),
            Fn2::F92(k0) => f92(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F55,
    F109,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn3::F55 => f55(p0, p1),
            Fn3::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F73,
    F108(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn4::F73 => f73(p0, p1),
            Fn4::F108(k0) => Ok(f108(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F76(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn5::F76(k0, k1) => f76(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F86,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt0, p1: Adt9) -> Adt0 {
        match self {
            Fn6::F86 => f86(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F100(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn7::F100(k0) => f100(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F103,
    F105,
    F106,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn8::F103 => f103(p0, p1),
            Fn8::F105 => f105(p0, p1),
            Fn8::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F116,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn9::F116 => f116(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0, v1: List<Adt2>, v2: u64, v3: u64) -> R<Result<Adt3, Bytes>> {
    let v4: Adt4 = f1(v3);
    let v5: Adt5 = f2(v4.clone(), f3(v1, v2), v0, List::<(Bytes, Adt0)>::nil())?;
    f4(v4, v5)
}

pub fn f1(v0: u64) -> Adt4 {
    let (h1, _) = f5(Fn0::F6, v0, Adt4::C0);
    h1
}

pub fn f2(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m2 = v0;
    match m2 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r7) => {
            let v4 = (*r7).clone();
            let v5: Adt5 = f9(v4.clone(), v1, v2.clone(), v3)?;
            let m4 = match v5.clone() {
                Adt5::C0(_, h3, _) => {
                    h3
                }
            };
            match m4 {
                None => {
                    f10(v4, match v5.clone() {
                        Adt5::C0(_, _, h5) => {
                            (*h5).clone()
                        }
                    }, v2, match v5 {
                        Adt5::C0(h6, _, _) => {
                            (*h6).clone()
                        }
                    })
                }
                Some(_) => {
                    Ok(v5)
                }
            }
        }
    }
}

pub fn f3(v0: List<Adt2>, v1: u64) -> Adt6 {
    Adt6::C0(List::<(Bytes, Adt0)>::nil(), 0u64, List::<Bytes>::nil(), 0u64, false, v0, v1)
}

pub fn f4(v0: Adt4, v1: Adt5) -> R<Result<Adt3, Bytes>> {
    let m10 = match v1.clone() {
        Adt5::C0(_, _, h9) => {
            (*h9).clone()
        }
    };
    if match m10 {
        Adt6::C0(_, _, _, _, h8, _, _) => {
            h8
        }
    } {
        Ok(Err::<Adt3, Bytes>(f11(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        let m12 = match v1.clone() {
            Adt5::C0(_, h11, _) => {
                h11
            }
        };
        match m12 {
            None => {
                f12(v0, match v1.clone() {
                    Adt5::C0(_, _, h13) => {
                        (*h13).clone()
                    }
                }, match v1 {
                    Adt5::C0(h14, _, _) => {
                        (*h14).clone()
                    }
                })
            }
            Some(v2) => {
                Ok(Err::<Adt3, Bytes>(v2))
            }
        }
    }
}

pub fn f5(v0: Fn0, p19: u64, p20: Adt4) -> (Adt4, bool) {
    let m18 = (p19, p20);
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, v2) = m18;
                let m15 = v1;
                if m15 == 0 {
                    Err::<(u64, Adt4), (Adt4, bool)>((v2, false))
                } else {
                    let v3 = m15 - 1;
                    let m17 = {
                        let c16: Fn0 = v0.clone();
                        c16.apply(v2.clone())
                    };
                    match m17 {
                        None => {
                            Err::<(u64, Adt4), (Adt4, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), (Adt4, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt4) -> Option<Adt4> {
    Some(Adt4::C1(std::rc::Rc::new(v0)))
}

pub fn f7(v0: Adt6, v1: Bytes) -> Adt5 {
    Adt5::C0(std::rc::Rc::new(Adt0::C0), Some(v1), std::rc::Rc::new(v0))
}

pub fn f8(v0: Adt6) -> Adt6 {
    f13(v0.clone(), match v0.clone() {
        Adt6::C0(h21, _, _, _, _, _, _) => {
            h21
        }
    }, match v0.clone() {
        Adt6::C0(_, h22, _, _, _, _, _) => {
            h22
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h23, _, _, _, _) => {
            h23
        }
    }, match v0 {
        Adt6::C0(_, _, _, h24, _, _, _) => {
            h24
        }
    }, true)
}

pub fn f9(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m25 = v0;
    match m25 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r67) => {
            let v4 = (*r67).clone();
            let m26 = v2.clone();
            match m26 {
                Adt0::C0 => {
                    Ok(f14(Adt0::C1(std::rc::Rc::new(Adt1::C0(1u64))), v1))
                }
                Adt0::C1(r57) => {
                    let v5 = (*r57).clone();
                    Ok(f14(Adt0::C1(std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(v5)))), v1))
                }
                Adt0::C2(v6, v7, _) => {
                    let v9: (u64, bool) = f15(v1.clone(), v6.clone(), v7.clone());
                    let v10: u64 = {
                        let (h27, _) = v9;
                        h27
                    };
                    let a28 = v10;
                    let a29 = 0u64;
                    if nat_eq(a28, a29) {
                        Ok(f7(v1, f16(f16(f16(f16(f11(Str::lit("`")), v6), f11(Str::lit("::"))), v7), f11(Str::lit("` does not resolve")))))
                    } else {
                        let a30 = v10;
                        let a31 = 1u64;
                        if nat_eq(a30, a31) {
                            f17(v1)
                        } else {
                            let a32 = v10;
                            let a33 = 2u64;
                            if nat_eq(a32, a33) {
                                Ok(f7(v1, f16(f16(f16(f16(f11(Str::lit("`")), v6), f11(Str::lit("::"))), v7), f11(Str::lit("` has no signature and cannot appear in an expression")))))
                            } else {
                                Ok(f14(f18(v1.clone(), v6, v7), v1))
                            }
                        }
                    }
                }
                Adt0::C3(v11) => {
                    if f19(v11.clone())? {
                        f17(v1)
                    } else {
                        let m34 = f20(v3, v11.clone());
                        match m34 {
                            None => {
                                Ok(f7(v1, f16(f16(f11(Str::lit("local `")), v11), f11(Str::lit("` is not in scope")))))
                            }
                            Some(v12) => {
                                Ok(f14(v12, v1))
                            }
                        }
                    }
                }
                Adt0::C4(r58, r59) => {
                    let v15 = (*r58).clone();
                    let v16 = (*r59).clone();
                    let v17: u64 = f22(v15.clone());
                    let a37 = v17;
                    let a38 = 2u64;
                    if if nat_lt(a37, a38) {
                        true
                    } else {
                        let a39 = v17;
                        let a40 = 8u64;
                        nat_eq(a39, a40)
                    } {
                        let v18: Bytes = f23(v2)?;
                        let v19: Bytes = f23(v15)?;
                        Ok(f7(v1, f16(f16(f16(v18, f11(Str::lit(" applies "))), v19), f11(Str::lit(", which is not a function")))))
                    } else {
                        let v20: Adt5 = f9(v4.clone(), v1, v15.clone(), v3.clone())?;
                        let m42 = match v20.clone() {
                            Adt5::C0(_, h41, _) => {
                                h41
                            }
                        };
                        match m42 {
                            None => {
                                let v22: List<Adt0> = f24(v4.clone(), v16);
                                f25(v4, match v20.clone() {
                                    Adt5::C0(_, _, h43) => {
                                        (*h43).clone()
                                    }
                                }, v2, v15, v22.clone(), {
                                    let a44 = v22;
                                    length_list(a44)
                                }, match v20 {
                                    Adt5::C0(h45, _, _) => {
                                        (*h45).clone()
                                    }
                                }, 0u64, v3)
                            }
                            Some(_) => {
                                Ok(v20)
                            }
                        }
                    }
                }
                Adt0::C5(r60, r61) => {
                    let v23 = (*r60).clone();
                    let v24 = (*r61).clone();
                    f26(v4, v1, v23.clone(), v24, v3, false, v23)
                }
                Adt0::C6(r62, r63) => {
                    let v25 = (*r62).clone();
                    let v26 = (*r63).clone();
                    f26(v4, v1, v25.clone(), v26, v3, true, v25)
                }
                Adt0::C7(v27, r64, r65, r66) => {
                    let v28 = (*r64).clone();
                    let v29 = (*r65).clone();
                    let v30 = (*r66).clone();
                    let v31: Adt5 = f2(v4.clone(), v1, v28.clone(), v3.clone())?;
                    let m47 = match v31.clone() {
                        Adt5::C0(_, h46, _) => {
                            h46
                        }
                    };
                    match m47 {
                        None => {
                            let v33: Adt5 = f9(v4.clone(), match v31 {
                                Adt5::C0(_, _, h48) => {
                                    (*h48).clone()
                                }
                            }, v29, v3.clone())?;
                            let m50 = match v33.clone() {
                                Adt5::C0(_, h49, _) => {
                                    h49
                                }
                            };
                            match m50 {
                                None => {
                                    let v35: Adt8 = f27(v4.clone(), match v33.clone() {
                                        Adt5::C0(_, _, h51) => {
                                            (*h51).clone()
                                        }
                                    }, v28.clone(), match v33 {
                                        Adt5::C0(h52, _, _) => {
                                            (*h52).clone()
                                        }
                                    })?;
                                    let m54 = match v35.clone() {
                                        Adt8::C0(h53, _) => {
                                            h53
                                        }
                                    };
                                    match m54 {
                                        None => {
                                            f9(v4, match v35 {
                                                Adt8::C0(_, h56) => {
                                                    (*h56).clone()
                                                }
                                            }, v30, List::cons((v27, v28), v3))
                                        }
                                        Some(v36) => {
                                            Ok(f7(match v35 {
                                                Adt8::C0(_, h55) => {
                                                    (*h55).clone()
                                                }
                                            }, v36))
                                        }
                                    }
                                }
                                Some(_) => {
                                    Ok(v33)
                                }
                            }
                        }
                        Some(_) => {
                            Ok(v31)
                        }
                    }
                }
                Adt0::C8(_) => {
                    let v14: Adt7 = f21(v1)?;
                    Ok(f14(match v14.clone() {
                        Adt7::C0(h35, _) => {
                            (*h35).clone()
                        }
                    }, match v14 {
                        Adt7::C0(_, h36) => {
                            (*h36).clone()
                        }
                    }))
                }
                Adt0::C9 => {
                    Ok(f14(Adt0::C0, v1))
                }
                Adt0::C10(_, _) => {
                    Ok(f14(Adt0::C0, v1))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(f14(Adt0::C0, v1))
                }
            }
        }
    }
}

pub fn f10(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt5> {
    let v4: Adt7 = f28(v0.clone(), v1, v3)?;
    let v5: Adt0 = match v4.clone() {
        Adt7::C0(h68, _) => {
            (*h68).clone()
        }
    };
    let v6: Adt6 = match v4 {
        Adt7::C0(_, h69) => {
            (*h69).clone()
        }
    };
    let v7: u64 = f22(v2.clone());
    let a70 = v7;
    let a71 = 8u64;
    if if nat_eq(a70, a71) {
        true
    } else {
        let a72 = v7;
        let a73 = 6u64;
        nat_eq(a72, a73)
    } {
        let v8: Bytes = f23(v2)?;
        Ok(f7(v6, f16(v8, f11(Str::lit(" is a term, not a type")))))
    } else {
        let a74 = f22(v5.clone());
        let a75 = 2u64;
        if nat_lt(a74, a75) {
            Ok(f14(v5, v6))
        } else {
            let m76 = f29(v5.clone())?;
            match m76 {
                None => {
                    if f30(v0, v6.clone(), v5.clone()) {
                        Ok(f14(v5, v6))
                    } else {
                        let v10: Bytes = f23(v2)?;
                        let v11: Bytes = f23(v5)?;
                        Ok(f7(v6, f16(f16(f16(v10, f11(Str::lit(" is not a type: its type is "))), v11), Bytes::lit(&[]))))
                    }
                }
                Some(_) => {
                    Ok(f14(v5, v6))
                }
            }
        }
    }
}

pub fn f11(v0: Str) -> Bytes {
    let a77 = v0;
    utf8_encode(a77)
}

pub fn f12(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Result<Adt3, Bytes>> {
    let v3: Adt7 = f28(v0.clone(), v1.clone(), v2)?;
    let v4: (List<Option<Adt0>>, Adt6) = f31(v0, match v3.clone() {
        Adt7::C0(_, h78) => {
            (*h78).clone()
        }
    }, f32(match v1 {
        Adt6::C0(_, _, h79, _, _, _, _) => {
            h79
        }
    }))?;
    let m81 = f33(v4.clone());
    if match m81 {
        Adt6::C0(_, _, _, _, h80, _, _) => {
            h80
        }
    } {
        Ok(Err::<Adt3, Bytes>(f11(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        Ok(Ok::<Adt3, Bytes>(Adt3::C0(std::rc::Rc::new(match v3 {
            Adt7::C0(h82, _) => {
                (*h82).clone()
            }
        }), {
            let (h83, _) = v4;
            h83
        })))
    }
}

pub fn f13(v0: Adt6, v1: List<(Bytes, Adt0)>, v2: u64, v3: List<Bytes>, v4: u64, v5: bool) -> Adt6 {
    Adt6::C0(v1, v2, v3, v4, v5, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h84, _) => {
            h84
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, h85) => {
            h85
        }
    })
}

pub fn f14(v0: Adt0, v1: Adt6) -> Adt5 {
    Adt5::C0(std::rc::Rc::new(v0), None::<Bytes>, std::rc::Rc::new(v1))
}

pub fn f15(v0: Adt6, v1: Bytes, v2: Bytes) -> (u64, bool) {
    let m87 = f34(Fn1::F35(v2, v1), None::<Adt2>, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h86, _) => {
            h86
        }
    });
    match m87 {
        None => {
            (match v0 {
                Adt6::C0(_, _, _, _, _, _, h90) => {
                    h90
                }
            }, false)
        }
        Some(v3) => {
            (match v3.clone() {
                Adt2::C0(_, _, h88, _, _) => {
                    h88
                }
            }, match v3 {
                Adt2::C0(_, _, _, _, h89) => {
                    h89
                }
            })
        }
    }
}

pub fn f16(v0: Bytes, v1: Bytes) -> Bytes {
    let a91 = v0;
    let a92 = v1;
    append_bytes(a91, a92)
}

pub fn f17(v0: Adt6) -> R<Adt5> {
    let v1: Adt7 = f36(v0)?;
    Ok(f14(match v1.clone() {
        Adt7::C0(h93, _) => {
            (*h93).clone()
        }
    }, match v1 {
        Adt7::C0(_, h94) => {
            (*h94).clone()
        }
    }))
}

pub fn f18(v0: Adt6, v1: Bytes, v2: Bytes) -> Adt0 {
    let m96 = f34(Fn1::F37(v2, v1), None::<Adt2>, match v0 {
        Adt6::C0(_, _, _, _, _, h95, _) => {
            h95
        }
    });
    match m96 {
        None => {
            Adt0::C0
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, _, _, h97, _) => {
                    (*h97).clone()
                }
            }
        }
    }
}

pub fn f19(v0: Bytes) -> R<bool> {
    let a98 = f38(v0, 0u64)?;
    let a99 = 63u64;
    Ok(nat_eq(a98, a99))
}

pub fn f20(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F40(v1), None::<Adt0>, v0)
}

pub fn f21(v0: Adt6) -> R<Adt7> {
    let v1: Adt7 = f36(v0.clone())?;
    let v2: Adt6 = match v1.clone() {
        Adt7::C0(_, h100) => {
            (*h100).clone()
        }
    };
    Ok(Adt7::C0(std::rc::Rc::new(match v1 {
        Adt7::C0(h101, _) => {
            (*h101).clone()
        }
    }), std::rc::Rc::new(f13(v2.clone(), match v2.clone() {
        Adt6::C0(h102, _, _, _, _, _, _) => {
            h102
        }
    }, match v2.clone() {
        Adt6::C0(_, h103, _, _, _, _, _) => {
            h103
        }
    }, List::cons(f16(f11(Str::lit("?")), f41(match v0 {
        Adt6::C0(_, h104, _, _, _, _, _) => {
            h104
        }
    })), match v2.clone() {
        Adt6::C0(_, _, h105, _, _, _, _) => {
            h105
        }
    }), match v2.clone() {
        Adt6::C0(_, _, _, h106, _, _, _) => {
            h106
        }
    }, match v2 {
        Adt6::C0(_, _, _, _, h107, _, _) => {
            h107
        }
    }))))
}

pub fn f22(v0: Adt0) -> u64 {
    let m108 = v0;
    match m108 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1(_) => {
            1u64
        }
        Adt0::C2(_, _, _) => {
            2u64
        }
        Adt0::C3(_) => {
            3u64
        }
        Adt0::C4(_, _) => {
            4u64
        }
        Adt0::C5(_, _) => {
            5u64
        }
        Adt0::C6(_, _) => {
            6u64
        }
        Adt0::C7(_, _, _, _) => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9 => {
            9u64
        }
        Adt0::C10(_, _) => {
            9u64
        }
        Adt0::C11(_, _, _, _) => {
            9u64
        }
    }
}

pub fn f23(v0: Adt0) -> R<Bytes> {
    Ok(f16(f16(f11(Str::lit("`")), f42(v0, false)?), f11(Str::lit("`"))))
}

pub fn f24(v0: Adt4, v1: Adt0) -> List<Adt0> {
    let m109 = v0;
    match m109 {
        Adt4::C0 => {
            List::<Adt0>::nil()
        }
        Adt4::C1(r113) => {
            let v2 = (*r113).clone();
            let m110 = v1;
            match m110 {
                Adt0::C0 => {
                    List::<Adt0>::nil()
                }
                Adt0::C1(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C2(_, _, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C3(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C4(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C5(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C6(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C7(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C8(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C9 => {
                    List::<Adt0>::nil()
                }
                Adt0::C10(r111, r112) => {
                    let v3 = (*r111).clone();
                    let v4 = (*r112).clone();
                    List::cons(v3, f24(v2, v4))
                }
                Adt0::C11(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
            }
        }
    }
}

pub fn f25(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: Adt0, v7: u64, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m114 = v0;
    match m114 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r121) => {
            let v9 = (*r121).clone();
            let m115 = v4.clone();
            match m115.uncons() {
                None => {
                    Ok(f14(v6, v1))
                }
                Some((_, _)) => {
                    let v12: Adt7 = f28(v9.clone(), v1, v6)?;
                    let v13: Adt0 = match v12.clone() {
                        Adt7::C0(h116, _) => {
                            (*h116).clone()
                        }
                    };
                    let v14: Adt6 = match v12 {
                        Adt7::C0(_, h117) => {
                            (*h117).clone()
                        }
                    };
                    let m118 = v13.clone();
                    match m118 {
                        Adt0::C0 => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C1(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C2(_, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C3(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C4(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C5(r119, r120) => {
                            let v15 = (*r119).clone();
                            let v16 = (*r120).clone();
                            f43(v9.clone(), v14, v2, v3, f44(v9, v15), v16, v4, v5, v7, v8)
                        }
                        Adt0::C6(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C7(_, _, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C8(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C9 => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C10(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C11(_, _, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                    }
                }
            }
        }
    }
}

pub fn f26(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<(Bytes, Adt0)>, v5: bool, v6: Adt0) -> R<Adt5> {
    let m122 = v0;
    match m122 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r136) => {
            let v7 = (*r136).clone();
            let m123 = v2;
            match m123 {
                Adt0::C0 => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C1(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C2(_, _, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C3(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C4(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C5(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C6(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C7(_, _, _, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C8(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C9 => {
                    if v5 {
                        let v15: Adt5 = f9(v7, v1, v3, v4)?;
                        let m131 = match v15.clone() {
                            Adt5::C0(_, h130, _) => {
                                h130
                            }
                        };
                        match m131 {
                            None => {
                                Ok(f14(Adt0::C5(std::rc::Rc::new(v6), std::rc::Rc::new(match v15.clone() {
                                    Adt5::C0(h132, _, _) => {
                                        (*h132).clone()
                                    }
                                })), match v15 {
                                    Adt5::C0(_, _, h133) => {
                                        (*h133).clone()
                                    }
                                }))
                            }
                            Some(_) => {
                                Ok(v15)
                            }
                        }
                    } else {
                        f2(v7, v1, v3, v4)
                    }
                }
                Adt0::C10(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C11(_, v9, r134, r135) => {
                    let v10 = (*r134).clone();
                    let v11 = (*r135).clone();
                    let v12: Adt5 = f2(v7.clone(), v1, v10.clone(), v4.clone())?;
                    let m125 = match v12.clone() {
                        Adt5::C0(_, h124, _) => {
                            h124
                        }
                    };
                    match m125 {
                        None => {
                            let v14: Adt5 = f26(v7, match v12 {
                                Adt5::C0(_, _, h126) => {
                                    (*h126).clone()
                                }
                            }, v11, v3, List::cons((v9, v10), v4), v5, v6)?;
                            Ok(Adt5::C0(std::rc::Rc::new(match v14.clone() {
                                Adt5::C0(h127, _, _) => {
                                    (*h127).clone()
                                }
                            }), match v14.clone() {
                                Adt5::C0(_, h128, _) => {
                                    h128
                                }
                            }, std::rc::Rc::new(match v14 {
                                Adt5::C0(_, _, h129) => {
                                    (*h129).clone()
                                }
                            })))
                        }
                        Some(_) => {
                            Ok(v12)
                        }
                    }
                }
            }
        }
    }
}

pub fn f27(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m137 = v0;
    match m137 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r148) => {
            let v4 = (*r148).clone();
            let v5: Adt7 = f28(v4.clone(), v1, v2)?;
            let v6: Adt7 = f28(v4.clone(), match v5.clone() {
                Adt7::C0(_, h138) => {
                    (*h138).clone()
                }
            }, v3)?;
            let v7: Adt0 = match v5 {
                Adt7::C0(h139, _) => {
                    (*h139).clone()
                }
            };
            let v8: Adt0 = match v6.clone() {
                Adt7::C0(h140, _) => {
                    (*h140).clone()
                }
            };
            let v9: Adt6 = match v6 {
                Adt7::C0(_, h141) => {
                    (*h141).clone()
                }
            };
            let m142 = f29(v7.clone())?;
            match m142 {
                None => {
                    let m147 = f29(v8.clone())?;
                    match m147 {
                        None => {
                            f49(v4, v9, v7, v8)
                        }
                        Some(v11) => {
                            Ok(f46(f48(v9, v11, v7)))
                        }
                    }
                }
                Some(v10) => {
                    let a143 = f47(v8.clone());
                    let a144 = v10.clone();
                    if equal(a143, a144) {
                        let a145 = f22(v8.clone());
                        let a146 = 3u64;
                        if nat_eq(a145, a146) {
                            Ok(f46(v9))
                        } else {
                            Ok(f46(f48(v9, v10, v8)))
                        }
                    } else {
                        Ok(f46(f48(v9, v10, v8)))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Adt7> {
    let m150 = f50(v0, match v1.clone() {
        Adt6::C0(h149, _, _, _, _, _, _) => {
            h149
        }
    }, v2.clone())?;
    match m150 {
        None => {
            Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f8(v1))))
        }
        Some(v3) => {
            Ok(Adt7::C0(std::rc::Rc::new(v3), std::rc::Rc::new(v1)))
        }
    }
}

pub fn f29(v0: Adt0) -> R<Option<Bytes>> {
    let m151 = v0;
    match m151 {
        Adt0::C0 => {
            Ok(None::<Bytes>)
        }
        Adt0::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt0::C2(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C3(v1) => {
            if f19(v1.clone())? {
                Ok(Some(v1))
            } else {
                Ok(None::<Bytes>)
            }
        }
        Adt0::C4(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C7(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt0::C9 => {
            Ok(None::<Bytes>)
        }
        Adt0::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C11(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f30(p157: Adt4, v1: Adt6, p158: Adt0) -> bool {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v0, v2) = m156;
                let m152 = v0;
                match m152 {
                    Adt4::C0 => {
                        Err::<(Adt4, Adt0), bool>(false)
                    }
                    Adt4::C1(r155) => {
                        let v3 = (*r155).clone();
                        let m153 = v2;
                        match m153 {
                            Adt0::C0 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C2(v4, v5, _) => {
                                Err::<(Adt4, Adt0), bool>(f51(v1.clone(), v4, v5))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C4(r154, _) => {
                                let v7 = (*r154).clone();
                                Ok::<(Adt4, Adt0), bool>((v3, v7))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C10(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt4, v1: Adt6, v2: List<Bytes>) -> R<(List<Option<Adt0>>, Adt6)> {
    let m159 = v0;
    match m159 {
        Adt4::C0 => {
            Ok((List::<Option<Adt0>>::nil(), f8(v1)))
        }
        Adt4::C1(r167) => {
            let v3 = (*r167).clone();
            let m160 = v2;
            match m160.uncons() {
                None => {
                    Ok((List::<Option<Adt0>>::nil(), v1))
                }
                Some((v4, v5)) => {
                    let v6: Adt7 = f28(v3.clone(), v1, Adt0::C3(v4))?;
                    let v7: (bool, Adt6) = f52(v3.clone(), match v6.clone() {
                        Adt7::C0(_, h161) => {
                            (*h161).clone()
                        }
                    }, match v6.clone() {
                        Adt7::C0(h162, _) => {
                            (*h162).clone()
                        }
                    })?;
                    let v8: (List<Option<Adt0>>, Adt6) = f31(v3, {
                        let (_, h163) = v7.clone();
                        h163
                    }, v5)?;
                    Ok((List::cons(if f53(v7) {
                        None::<Adt0>
                    } else {
                        Some(match v6 {
                            Adt7::C0(h164, _) => {
                                (*h164).clone()
                            }
                        })
                    }, {
                        let (h165, _) = v8;
                        h165
                    }), {
                        let (_, h166) = v8;
                        h166
                    }))
                }
            }
        }
    }
}

pub fn f32(v0: List<Bytes>) -> List<Bytes> {
    f54(Fn3::F55, List::<Bytes>::nil(), v0)
}

pub fn f33(v0: (List<Option<Adt0>>, Adt6)) -> Adt6 {
    let (_, h168) = v0;
    h168
}

pub fn f34(v0: Fn1, p172: Option<Adt2>, p173: List<Adt2>) -> Option<Adt2> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m169 = v2;
                match m169.uncons() {
                    None => {
                        Err::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(({
                            let c170: Fn1 = v0.clone();
                            c170.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m174 = v2.clone();
    match m174 {
        None => {
            let a176 = match v3.clone() {
                Adt2::C0(h175, _, _, _, _) => {
                    h175
                }
            };
            let a177 = v1;
            if if equal(a176, a177) {
                let a179 = match v3.clone() {
                    Adt2::C0(_, h178, _, _, _) => {
                        h178
                    }
                };
                let a180 = v0;
                equal(a179, a180)
            } else {
                false
            } {
                Some(v3)
            } else {
                None::<Adt2>
            }
        }
        Some(_) => {
            v2
        }
    }
}

pub fn f36(v0: Adt6) -> R<Adt7> {
    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C3(f16(f11(Str::lit("?")), f41(match v0.clone() {
        Adt6::C0(_, h181, _, _, _, _, _) => {
            h181
        }
    })))), std::rc::Rc::new(f13(v0.clone(), match v0.clone() {
        Adt6::C0(h182, _, _, _, _, _, _) => {
            h182
        }
    }, {
        let a184 = match v0.clone() {
            Adt6::C0(_, h183, _, _, _, _, _) => {
                h183
            }
        };
        let a185 = 1u64;
        nat_add(a184, a185)?
    }, match v0.clone() {
        Adt6::C0(_, _, h186, _, _, _, _) => {
            h186
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h187, _, _, _) => {
            h187
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h188, _, _) => {
            h188
        }
    }))))
}

pub fn f37(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m189 = v2.clone();
    match m189 {
        None => {
            let a191 = match v3.clone() {
                Adt2::C0(h190, _, _, _, _) => {
                    h190
                }
            };
            let a192 = v1;
            if if equal(a191, a192) {
                let a194 = match v3.clone() {
                    Adt2::C0(_, h193, _, _, _) => {
                        h193
                    }
                };
                let a195 = v0;
                equal(a194, a195)
            } else {
                false
            } {
                Some(v3)
            } else {
                None::<Adt2>
            }
        }
        Some(_) => {
            v2
        }
    }
}

pub fn f38(v0: Bytes, v1: u64) -> R<u64> {
    let m196 = f56(v0, v1);
    match m196 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a241 = {
                let a217 = {
                    let a205 = {
                        let a199 = {
                            let a197 = v3;
                            let a198 = 128u8;
                            fixed_u8::bit_and(a197, a198)
                        };
                        let a200 = 128u8;
                        if equal(a199, a200) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a206 = {
                        let a203 = {
                            let a201 = v3;
                            let a202 = 64u8;
                            fixed_u8::bit_and(a201, a202)
                        };
                        let a204 = 64u8;
                        if equal(a203, a204) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a205, a206)?
                };
                let a218 = {
                    let a215 = {
                        let a209 = {
                            let a207 = v3;
                            let a208 = 32u8;
                            fixed_u8::bit_and(a207, a208)
                        };
                        let a210 = 32u8;
                        if equal(a209, a210) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a216 = {
                        let a213 = {
                            let a211 = v3;
                            let a212 = 16u8;
                            fixed_u8::bit_and(a211, a212)
                        };
                        let a214 = 16u8;
                        if equal(a213, a214) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a215, a216)?
                };
                nat_add(a217, a218)?
            };
            let a242 = {
                let a239 = {
                    let a227 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 8u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 8u8;
                        if equal(a221, a222) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a228 = {
                        let a225 = {
                            let a223 = v3;
                            let a224 = 4u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 4u8;
                        if equal(a225, a226) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a227, a228)?
                };
                let a240 = {
                    let a237 = {
                        let a231 = {
                            let a229 = v3;
                            let a230 = 2u8;
                            fixed_u8::bit_and(a229, a230)
                        };
                        let a232 = 2u8;
                        if equal(a231, a232) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a238 = {
                        let a235 = {
                            let a233 = v3;
                            let a234 = 1u8;
                            fixed_u8::bit_and(a233, a234)
                        };
                        let a236 = 1u8;
                        if equal(a235, a236) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a237, a238)?
                };
                nat_add(a239, a240)?
            };
            nat_add(a241, a242)
        }
    }
}

pub fn f39(v0: Fn2, p246: Option<Adt0>, p247: List<(Bytes, Adt0)>) -> Option<Adt0> {
    let m245 = (p246, p247);
    {
        let mut m245 = m245;
        loop {
            let step = {
                let (v1, v2) = m245;
                let m243 = v2;
                match m243.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(({
                            let c244: Fn2 = v0.clone();
                            c244.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m248 = v1.clone();
    match m248 {
        None => {
            let a250 = {
                let (h249, _) = v2;
                h249
            };
            let a251 = v0;
            if equal(a250, a251) {
                Some({
                    let (_, h252) = v2;
                    h252
                })
            } else {
                None::<Adt0>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f41(v0: u64) -> Bytes {
    f57(f58(), 10u64, v0)
}

pub fn f42(v0: Adt0, v1: bool) -> R<Bytes> {
    let (h253, _) = f59(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h253)
}

pub fn f43(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt9>, v5: Adt0, v6: List<Adt0>, v7: u64, v8: u64, v9: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m254 = v0;
    match m254 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r273) => {
            let v10 = (*r273).clone();
            let m255 = v4.clone();
            match m255.uncons() {
                None => {
                    f25(v10, v1, v2, v3, v6, v7, v5, v8, v9)
                }
                Some((v11, v12)) => {
                    let a257 = match v11.clone() {
                        Adt9::C0(h256, _, _) => {
                            h256
                        }
                    };
                    let a258 = 0u64;
                    if nat_eq(a257, a258) {
                        let m259 = v6;
                        match m259.uncons() {
                            None => {
                                Ok(f14(Adt0::C5(std::rc::Rc::new(f60(v4)), std::rc::Rc::new(v5)), v1))
                            }
                            Some((v13, v14)) => {
                                let v15: Adt5 = f9(v10.clone(), v1, v13.clone(), v9.clone())?;
                                let m261 = match v15.clone() {
                                    Adt5::C0(_, h260, _) => {
                                        h260
                                    }
                                };
                                match m261 {
                                    None => {
                                        let v17: Adt8 = f27(v10.clone(), match v15.clone() {
                                            Adt5::C0(_, _, h262) => {
                                                (*h262).clone()
                                            }
                                        }, match v11.clone() {
                                            Adt9::C0(_, _, h263) => {
                                                (*h263).clone()
                                            }
                                        }, match v15 {
                                            Adt5::C0(h264, _, _) => {
                                                (*h264).clone()
                                            }
                                        })?;
                                        let m266 = match v17.clone() {
                                            Adt8::C0(h265, _) => {
                                                h265
                                            }
                                        };
                                        match m266 {
                                            None => {
                                                f61(v10, match v17 {
                                                    Adt8::C0(_, h268) => {
                                                        (*h268).clone()
                                                    }
                                                }, v2, v3, v11, v13, v12, v5, v14, v7, {
                                                    let a269 = v8;
                                                    let a270 = 1u64;
                                                    nat_add(a269, a270)?
                                                }, v9)
                                            }
                                            Some(v18) => {
                                                Ok(f7(match v17 {
                                                    Adt8::C0(_, h267) => {
                                                        (*h267).clone()
                                                    }
                                                }, v18))
                                            }
                                        }
                                    }
                                    Some(_) => {
                                        Ok(v15)
                                    }
                                }
                            }
                        }
                    } else {
                        let v19: Adt7 = f36(v1)?;
                        f61(v10, match v19.clone() {
                            Adt7::C0(_, h271) => {
                                (*h271).clone()
                            }
                        }, v2, v3, v11, match v19 {
                            Adt7::C0(h272, _) => {
                                (*h272).clone()
                            }
                        }, v12, v5, v6, v7, v8, v9)
                    }
                }
            }
        }
    }
}

pub fn f44(v0: Adt4, v1: Adt0) -> List<Adt9> {
    let m274 = v0;
    match m274 {
        Adt4::C0 => {
            List::<Adt9>::nil()
        }
        Adt4::C1(r278) => {
            let v2 = (*r278).clone();
            let m275 = v1;
            match m275 {
                Adt0::C0 => {
                    List::<Adt9>::nil()
                }
                Adt0::C1(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C2(_, _, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C3(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C4(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C5(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C6(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C7(_, _, _, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C8(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C9 => {
                    List::<Adt9>::nil()
                }
                Adt0::C10(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C11(v3, v4, r276, r277) => {
                    let v5 = (*r276).clone();
                    let v6 = (*r277).clone();
                    List::cons(Adt9::C0(v3, v4, std::rc::Rc::new(v5)), f44(v2, v6))
                }
            }
        }
    }
}

pub fn f45(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: u64, v7: Adt0, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m279 = v0;
    match m279 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r281) => {
            let v9 = (*r281).clone();
            let m280 = f29(v7.clone())?;
            match m280 {
                None => {
                    if f30(v9.clone(), v1.clone(), v7.clone()) {
                        f62(v9, v1, v4, v8)
                    } else {
                        let v11: Bytes = f23(v2)?;
                        let v12: Bytes = f23(v3)?;
                        let v13: Bytes = f23(v7)?;
                        Ok(f7(v1, f16(f16(f16(f16(f16(f16(f16(f16(f16(v11, f11(Str::lit(" supplies "))), f41(v5)), f11(Str::lit(" explicit argument"))), f63(v5)), f11(Str::lit(" but "))), v12), f11(Str::lit(" accepts "))), f41(v6)), f16(f16(f11(Str::lit(": after them its type is ")), v13), f11(Str::lit(", not a function"))))))
                    }
                }
                Some(_) => {
                    f62(v9, v1, v4, v8)
                }
            }
        }
    }
}

pub fn f46(v0: Adt6) -> Adt8 {
    Adt8::C0(None::<Bytes>, std::rc::Rc::new(v0))
}

pub fn f47(v0: Adt0) -> Bytes {
    let m282 = v0;
    match m282 {
        Adt0::C0 => {
            Bytes::lit(&[])
        }
        Adt0::C1(_) => {
            Bytes::lit(&[])
        }
        Adt0::C2(v1, v2, _) => {
            f16(f16(v1, f11(Str::lit("::"))), v2)
        }
        Adt0::C3(v4) => {
            v4
        }
        Adt0::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C5(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C6(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C7(_, _, _, _) => {
            Bytes::lit(&[])
        }
        Adt0::C8(v5) => {
            v5
        }
        Adt0::C9 => {
            Bytes::lit(&[])
        }
        Adt0::C10(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C11(_, _, _, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f48(v0: Adt6, v1: Bytes, v2: Adt0) -> Adt6 {
    f13(v0.clone(), List::cons((v1, v2), match v0.clone() {
        Adt6::C0(h283, _, _, _, _, _, _) => {
            h283
        }
    }), match v0.clone() {
        Adt6::C0(_, h284, _, _, _, _, _) => {
            h284
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h285, _, _, _, _) => {
            h285
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h286, _, _, _) => {
            h286
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h287, _, _) => {
            h287
        }
    })
}

pub fn f49(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m288 = v0;
    match m288 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r306) => {
            let v4 = (*r306).clone();
            let v5: u64 = f64(v4.clone(), v2.clone(), v3.clone());
            let a289 = v5;
            let a290 = 0u64;
            if nat_eq(a289, a290) {
                Ok(f46(v1))
            } else {
                let a291 = v5;
                let a292 = 1u64;
                if nat_eq(a291, a292) {
                    if if f30(v4.clone(), v1.clone(), v2.clone()) {
                        true
                    } else {
                        f30(v4.clone(), v1.clone(), v3.clone())
                    } {
                        Ok(f46(v1))
                    } else {
                        let v6: (Adt0, Adt0) = f65(v2);
                        let v7: (Adt0, Adt0) = f65(v3);
                        let v8: Adt8 = f27(v4.clone(), v1, {
                            let (h293, _) = v6;
                            h293
                        }, {
                            let (h294, _) = v7;
                            h294
                        })?;
                        let m296 = match v8.clone() {
                            Adt8::C0(h295, _) => {
                                h295
                            }
                        };
                        match m296 {
                            None => {
                                f66(v4, match v8 {
                                    Adt8::C0(_, h297) => {
                                        (*h297).clone()
                                    }
                                }, {
                                    let (_, h298) = v6;
                                    h298
                                }, {
                                    let (_, h299) = v7;
                                    h299
                                })
                            }
                            Some(_) => {
                                Ok(v8)
                            }
                        }
                    }
                } else {
                    let a300 = v5;
                    let a301 = 2u64;
                    if nat_eq(a300, a301) {
                        f67(v4, v1, v2, v3)
                    } else {
                        if if f30(v4.clone(), v1.clone(), v2.clone()) {
                            true
                        } else {
                            f30(v4.clone(), v1.clone(), v3.clone())
                        } {
                            Ok(f46(v1))
                        } else {
                            let v10: (bool, Adt6) = f52(v4.clone(), v1, v2.clone())?;
                            if f53(v10.clone()) {
                                Ok(f46({
                                    let (_, h302) = v10;
                                    h302
                                }))
                            } else {
                                let v11: (bool, Adt6) = f52(v4, {
                                    let (_, h303) = v10;
                                    h303
                                }, v3.clone())?;
                                if f53(v11.clone()) {
                                    Ok(f46({
                                        let (_, h304) = v11;
                                        h304
                                    }))
                                } else {
                                    let v12: Bytes = f23(v2)?;
                                    let v13: Bytes = f23(v3)?;
                                    Ok(f68({
                                        let (_, h305) = v11;
                                        h305
                                    }, f16(f16(f16(f11(Str::lit("expected ")), v12), f11(Str::lit(", found "))), v13)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Adt4, v1: List<(Bytes, Adt0)>, v2: Adt0) -> R<Option<Adt0>> {
    let m307 = v0;
    match m307 {
        Adt4::C0 => {
            Ok(None::<Adt0>)
        }
        Adt4::C1(r344) => {
            let v3 = (*r344).clone();
            let m308 = v2;
            match m308 {
                Adt0::C0 => {
                    Ok(Some(Adt0::C0))
                }
                Adt0::C1(r329) => {
                    let v29 = (*r329).clone();
                    Ok(Some(Adt0::C1(std::rc::Rc::new(v29))))
                }
                Adt0::C2(v30, v31, r330) => {
                    let v32 = (*r330).clone();
                    Ok(Some(Adt0::C2(v30, v31, std::rc::Rc::new(v32))))
                }
                Adt0::C3(v4) => {
                    if f19(v4.clone())? {
                        let m309 = f69(v1.clone(), v4.clone());
                        match m309 {
                            None => {
                                Ok(Some(Adt0::C3(v4)))
                            }
                            Some(v5) => {
                                f50(v3, v1, v5)
                            }
                        }
                    } else {
                        Ok(Some(Adt0::C3(v4)))
                    }
                }
                Adt0::C4(r331, r332) => {
                    let v6 = (*r331).clone();
                    let v7 = (*r332).clone();
                    let m310 = f70(f50(v3.clone(), v1.clone(), v6)?, f50(v3, v1, v7)?);
                    match m310 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v8) => {
                            Ok(Some(Adt0::C4(std::rc::Rc::new({
                                let (h311, _) = v8;
                                h311
                            }), std::rc::Rc::new({
                                let (_, h312) = v8;
                                h312
                            }))))
                        }
                    }
                }
                Adt0::C5(r333, r334) => {
                    let v12 = (*r333).clone();
                    let v13 = (*r334).clone();
                    let m316 = f70(f50(v3.clone(), v1.clone(), v12)?, f50(v3, v1, v13)?);
                    match m316 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v14) => {
                            Ok(Some(Adt0::C5(std::rc::Rc::new({
                                let (h317, _) = v14;
                                h317
                            }), std::rc::Rc::new({
                                let (_, h318) = v14;
                                h318
                            }))))
                        }
                    }
                }
                Adt0::C6(r335, r336) => {
                    let v15 = (*r335).clone();
                    let v16 = (*r336).clone();
                    let m319 = f70(f50(v3.clone(), v1.clone(), v15)?, f50(v3, v1, v16)?);
                    match m319 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v17) => {
                            Ok(Some(Adt0::C6(std::rc::Rc::new({
                                let (h320, _) = v17;
                                h320
                            }), std::rc::Rc::new({
                                let (_, h321) = v17;
                                h321
                            }))))
                        }
                    }
                }
                Adt0::C7(v23, r337, r338, r339) => {
                    let v24 = (*r337).clone();
                    let v25 = (*r338).clone();
                    let v26 = (*r339).clone();
                    let m325 = f70(f50(v3.clone(), v1.clone(), v24)?, f50(v3.clone(), v1.clone(), v25)?);
                    match m325 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v27) => {
                            let m326 = f50(v3, v1, v26)?;
                            match m326 {
                                None => {
                                    Ok(None::<Adt0>)
                                }
                                Some(v28) => {
                                    Ok(Some(Adt0::C7(v23, std::rc::Rc::new({
                                        let (h327, _) = v27;
                                        h327
                                    }), std::rc::Rc::new({
                                        let (_, h328) = v27;
                                        h328
                                    }), std::rc::Rc::new(v28))))
                                }
                            }
                        }
                    }
                }
                Adt0::C8(v33) => {
                    Ok(Some(Adt0::C8(v33)))
                }
                Adt0::C9 => {
                    Ok(Some(Adt0::C9))
                }
                Adt0::C10(r340, r341) => {
                    let v9 = (*r340).clone();
                    let v10 = (*r341).clone();
                    let m313 = f70(f50(v3.clone(), v1.clone(), v9)?, f50(v3, v1, v10)?);
                    match m313 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v11) => {
                            Ok(Some(Adt0::C10(std::rc::Rc::new({
                                let (h314, _) = v11;
                                h314
                            }), std::rc::Rc::new({
                                let (_, h315) = v11;
                                h315
                            }))))
                        }
                    }
                }
                Adt0::C11(v18, v19, r342, r343) => {
                    let v20 = (*r342).clone();
                    let v21 = (*r343).clone();
                    let m322 = f70(f50(v3.clone(), v1.clone(), v20)?, f50(v3, v1, v21)?);
                    match m322 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v22) => {
                            Ok(Some(Adt0::C11(v18, v19, std::rc::Rc::new({
                                let (h323, _) = v22;
                                h323
                            }), std::rc::Rc::new({
                                let (_, h324) = v22;
                                h324
                            }))))
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Adt6, v1: Bytes, v2: Bytes) -> bool {
    let v3: (u64, bool) = f15(v0, v1, v2);
    let a346 = {
        let (h345, _) = v3;
        h345
    };
    let a347 = 1u64;
    if nat_eq(a346, a347) {
        true
    } else {
        let a349 = {
            let (h348, _) = v3;
            h348
        };
        let a350 = 3u64;
        if nat_eq(a349, a350) {
            let (_, h351) = v3;
            h351
        } else {
            false
        }
    }
}

pub fn f52(v0: Adt4, v1: Adt6, v2: Adt0) -> R<(bool, Adt6)> {
    let v3: Adt7 = f28(v0.clone(), v1, v2)?;
    let m353 = f71(v0, match v3.clone() {
        Adt7::C0(h352, _) => {
            (*h352).clone()
        }
    }, List::<Bytes>::nil())?;
    match m353 {
        None => {
            Ok((true, f8(match v3 {
                Adt7::C0(_, h355) => {
                    (*h355).clone()
                }
            })))
        }
        Some(v4) => {
            Ok((f72(Fn4::F73, false, v4)?, match v3 {
                Adt7::C0(_, h354) => {
                    (*h354).clone()
                }
            }))
        }
    }
}

pub fn f53(v0: (bool, Adt6)) -> bool {
    let (h356, _) = v0;
    h356
}

pub fn f54(v0: Fn3, p360: List<Bytes>, p361: List<Bytes>) -> List<Bytes> {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v1, v2) = m359;
                let m357 = v2;
                match m357.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c358: Fn3 = v0.clone();
                            c358.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f56(v0: Bytes, v1: u64) -> Option<u8> {
    let a362 = v0;
    let a363 = v1;
    index_bytes(a362, a363)
}

pub fn f57(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a364 = v2;
    let a365 = 0u64;
    if nat_eq(a364, a365) {
        f74(v0, 0u64)
    } else {
        let (_, h366) = {
            let (h367, _) = f75(Fn5::F76(v1, v0), v2, (v2, Bytes::lit(&[])));
            h367
        };
        h366
    }
}

pub fn f58() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f59(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m368 = v0;
    match m368 {
        Adt0::C0 => {
            Ok((f77(Str::lit("(sort prop)")), v3))
        }
        Adt0::C1(r396) => {
            let v4 = (*r396).clone();
            Ok((f78(f78(f77(Str::lit("(sort (type ")), f79(v4)), f77(Str::lit("))"))), v3))
        }
        Adt0::C2(v5, v6, r397) => {
            let v7 = (*r397).clone();
            Ok((f80(v5, v6, v7), v3))
        }
        Adt0::C3(v8) => {
            Ok((f78(f78(f77(Str::lit("(local ")), if v1 {
                f81(v2, v8)
            } else {
                v8
            }), f77(Str::lit(")"))), v3))
        }
        Adt0::C4(r398, r399) => {
            let v9 = (*r398).clone();
            let v10 = (*r399).clone();
            let v11: (Bytes, u64) = f59(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f82(v10, v1, v2, {
                let (_, h369) = v11;
                h369
            })?;
            Ok((f78(f78(f78(f77(Str::lit("(app ")), {
                let (h370, _) = v11;
                h370
            }), {
                let (h371, _) = v12;
                h371
            }), f77(Str::lit(")"))), {
                let (_, h372) = v12;
                h372
            }))
        }
        Adt0::C5(r400, r401) => {
            let v13 = (*r400).clone();
            let v14 = (*r401).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f59(v14, v1, {
                let (_, h373) = {
                    let (h374, _) = v15.clone();
                    h374
                };
                h373
            }, {
                let (_, h375) = v15;
                h375
            })?;
            Ok((f78(f78(f78(f78(f77(Str::lit("(pi (")), {
                let (h376, _) = {
                    let (h377, _) = v15;
                    h377
                };
                h376
            }), f77(Str::lit(") "))), {
                let (h378, _) = v16;
                h378
            }), f77(Str::lit(")"))), {
                let (_, h379) = v16;
                h379
            }))
        }
        Adt0::C6(r402, r403) => {
            let v17 = (*r402).clone();
            let v18 = (*r403).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f59(v18, v1, {
                let (_, h380) = {
                    let (h381, _) = v19.clone();
                    h381
                };
                h380
            }, {
                let (_, h382) = v19;
                h382
            })?;
            Ok((f78(f78(f78(f78(f77(Str::lit("(lam (")), {
                let (h383, _) = {
                    let (h384, _) = v19;
                    h384
                };
                h383
            }), f77(Str::lit(") "))), {
                let (h385, _) = v20;
                h385
            }), f77(Str::lit(")"))), {
                let (_, h386) = v20;
                h386
            }))
        }
        Adt0::C7(v21, r404, r405, r406) => {
            let v22 = (*r404).clone();
            let v23 = (*r405).clone();
            let v24 = (*r406).clone();
            let v25: (Bytes, u64) = f84(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h387, _) = v25.clone();
                h387
            }), v2);
            let v27: (Bytes, u64) = f59(v22, v1, v26.clone(), {
                let (_, h388) = v25;
                h388
            })?;
            let v28: (Bytes, u64) = f59(v23, v1, v26.clone(), {
                let (_, h389) = v27;
                h389
            })?;
            let v29: (Bytes, u64) = f59(v24, v1, v26, {
                let (_, h390) = v28;
                h390
            })?;
            Ok((f78(f78(f78(f78(f78(f78(f78(f77(Str::lit("(let ")), {
                let (h391, _) = v25;
                h391
            }), f77(Str::lit(" "))), {
                let (h392, _) = v27;
                h392
            }), f77(Str::lit(" "))), {
                let (h393, _) = v28;
                h393
            }), f77(Str::lit(" "))), f78({
                let (h394, _) = v29;
                h394
            }, f77(Str::lit(")")))), {
                let (_, h395) = v29;
                h395
            }))
        }
        Adt0::C8(v30) => {
            Ok((f78(f78(f77(Str::lit("(nat ")), v30), f77(Str::lit(")"))), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f60(v0: List<Adt9>) -> Adt0 {
    f85(Fn6::F86, Adt0::C9, f87(v0))
}

pub fn f61(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: Adt9, v5: Adt0, v6: List<Adt9>, v7: Adt0, v8: List<Adt0>, v9: u64, v10: u64, v11: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m407 = v0;
    match m407 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r414) => {
            let v12 = (*r414).clone();
            let v13: Adt10 = f88(v12.clone(), v1, v6, match v4.clone() {
                Adt9::C0(_, h408, _) => {
                    h408
                }
            }, v5.clone())?;
            let v14: Adt7 = f89(v12.clone(), match v13.clone() {
                Adt10::C0(_, h409) => {
                    (*h409).clone()
                }
            }, v7, match v4 {
                Adt9::C0(_, h410, _) => {
                    h410
                }
            }, v5)?;
            f43(v12, match v14.clone() {
                Adt7::C0(_, h411) => {
                    (*h411).clone()
                }
            }, v2, v3, match v13 {
                Adt10::C0(h412, _) => {
                    h412
                }
            }, match v14 {
                Adt7::C0(h413, _) => {
                    (*h413).clone()
                }
            }, v8, v9, v10, v11)
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt6, v2: List<Adt0>, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m415 = v0;
    match m415 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r423) => {
            let v4 = (*r423).clone();
            let m416 = v2;
            match m416.uncons() {
                None => {
                    f17(v1)
                }
                Some((v5, v6)) => {
                    let v7: Adt5 = f9(v4.clone(), v1, v5, v3.clone())?;
                    let m418 = match v7.clone() {
                        Adt5::C0(_, h417, _) => {
                            h417
                        }
                    };
                    match m418 {
                        None => {
                            let v9: Adt5 = f62(v4, match v7 {
                                Adt5::C0(_, _, h419) => {
                                    (*h419).clone()
                                }
                            }, v6, v3)?;
                            Ok(Adt5::C0(std::rc::Rc::new(match v9.clone() {
                                Adt5::C0(h420, _, _) => {
                                    (*h420).clone()
                                }
                            }), match v9.clone() {
                                Adt5::C0(_, h421, _) => {
                                    h421
                                }
                            }, std::rc::Rc::new(match v9 {
                                Adt5::C0(_, _, h422) => {
                                    (*h422).clone()
                                }
                            })))
                        }
                        Some(_) => {
                            Ok(v7)
                        }
                    }
                }
            }
        }
    }
}

pub fn f63(v0: u64) -> Bytes {
    let a424 = v0;
    let a425 = 1u64;
    if nat_eq(a424, a425) {
        Bytes::lit(&[])
    } else {
        f11(Str::lit("s"))
    }
}

pub fn f64(v0: Adt4, v1: Adt0, v2: Adt0) -> u64 {
    let v3: u64 = f22(v1.clone());
    let a426 = v3;
    let a427 = f22(v2.clone());
    if nat_eq(a426, a427) {
        let a428 = v3;
        let a429 = 2u64;
        if nat_lt(a428, a429) {
            0u64
        } else {
            let a430 = v3;
            let a431 = 2u64;
            if if nat_eq(a430, a431) {
                true
            } else {
                let a432 = v3;
                let a433 = 3u64;
                if nat_eq(a432, a433) {
                    true
                } else {
                    let a434 = v3;
                    let a435 = 8u64;
                    nat_eq(a434, a435)
                }
            } {
                let a436 = f47(v1);
                let a437 = f47(v2);
                if equal(a436, a437) {
                    0u64
                } else {
                    3u64
                }
            } else {
                let a438 = v3;
                let a439 = 4u64;
                if nat_eq(a438, a439) {
                    let m440 = v1;
                    match m440 {
                        Adt0::C0 => {
                            3u64
                        }
                        Adt0::C1(_) => {
                            3u64
                        }
                        Adt0::C2(_, _, _) => {
                            3u64
                        }
                        Adt0::C3(_) => {
                            3u64
                        }
                        Adt0::C4(_, r447) => {
                            let v5 = (*r447).clone();
                            let m441 = v2;
                            match m441 {
                                Adt0::C0 => {
                                    3u64
                                }
                                Adt0::C1(_) => {
                                    3u64
                                }
                                Adt0::C2(_, _, _) => {
                                    3u64
                                }
                                Adt0::C3(_) => {
                                    3u64
                                }
                                Adt0::C4(_, r446) => {
                                    let v7 = (*r446).clone();
                                    let a444 = {
                                        let a442 = f24(v0.clone(), v5);
                                        length_list(a442)
                                    };
                                    let a445 = {
                                        let a443 = f24(v0, v7);
                                        length_list(a443)
                                    };
                                    if nat_eq(a444, a445) {
                                        1u64
                                    } else {
                                        3u64
                                    }
                                }
                                Adt0::C5(_, _) => {
                                    3u64
                                }
                                Adt0::C6(_, _) => {
                                    3u64
                                }
                                Adt0::C7(_, _, _, _) => {
                                    3u64
                                }
                                Adt0::C8(_) => {
                                    3u64
                                }
                                Adt0::C9 => {
                                    3u64
                                }
                                Adt0::C10(_, _) => {
                                    3u64
                                }
                                Adt0::C11(_, _, _, _) => {
                                    3u64
                                }
                            }
                        }
                        Adt0::C5(_, _) => {
                            3u64
                        }
                        Adt0::C6(_, _) => {
                            3u64
                        }
                        Adt0::C7(_, _, _, _) => {
                            3u64
                        }
                        Adt0::C8(_) => {
                            3u64
                        }
                        Adt0::C9 => {
                            3u64
                        }
                        Adt0::C10(_, _) => {
                            3u64
                        }
                        Adt0::C11(_, _, _, _) => {
                            3u64
                        }
                    }
                } else {
                    let a448 = v3;
                    let a449 = 5u64;
                    if if nat_eq(a448, a449) {
                        true
                    } else {
                        let a450 = v3;
                        let a451 = 6u64;
                        nat_eq(a450, a451)
                    } {
                        2u64
                    } else {
                        3u64
                    }
                }
            }
        }
    } else {
        3u64
    }
}

pub fn f65(v0: Adt0) -> (Adt0, Adt0) {
    let m452 = v0;
    match m452 {
        Adt0::C0 => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C1(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C2(_, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C3(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C4(r453, r454) => {
            let v1 = (*r453).clone();
            let v2 = (*r454).clone();
            (v1, v2)
        }
        Adt0::C5(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C6(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C7(_, _, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C8(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C9 => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C10(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C11(_, _, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
    }
}

pub fn f66(p467: Adt4, p468: Adt6, p469: Adt0, p470: Adt0) -> R<Adt8> {
    let m466 = (p467, (p468, (p469, p470)));
    {
        let mut m466 = m466;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m466;
                let m455 = v0;
                match m455 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(f8(v1))))
                    }
                    Adt4::C1(r465) => {
                        let v4 = (*r465).clone();
                        let m456 = v2;
                        match m456 {
                            Adt0::C0 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C2(_, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C4(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C10(r463, r464) => {
                                let v5 = (*r463).clone();
                                let v6 = (*r464).clone();
                                let m457 = v3;
                                match m457 {
                                    Adt0::C0 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C1(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C2(_, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C3(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C4(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C5(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C6(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C7(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C8(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C9 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C10(r461, r462) => {
                                        let v7 = (*r461).clone();
                                        let v8 = (*r462).clone();
                                        let v9: Adt8 = f27(v4.clone(), v1, v5, v7)?;
                                        let m459 = match v9.clone() {
                                            Adt8::C0(h458, _) => {
                                                h458
                                            }
                                        };
                                        match m459 {
                                            None => {
                                                Ok::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>((v4, (match v9 {
                                                    Adt8::C0(_, h460) => {
                                                        (*h460).clone()
                                                    }
                                                }, (v6, v8))))
                                            }
                                            Some(_) => {
                                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(v9))
                                            }
                                        }
                                    }
                                    Adt0::C11(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                }
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m471 = v0;
    match m471 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r484) => {
            let v4 = (*r484).clone();
            let v5: bool = {
                let a472 = f22(v2.clone());
                let a473 = 5u64;
                nat_eq(a472, a473)
            };
            let v6: Adt11 = f90(v4.clone(), v2.clone(), v5);
            let v7: Adt11 = f90(v4.clone(), v3.clone(), v5);
            let v8: u64 = {
                let a475 = match v6.clone() {
                    Adt11::C0(h474, _) => {
                        h474
                    }
                };
                length_list(a475)
            };
            let v9: u64 = {
                let a477 = match v7.clone() {
                    Adt11::C0(h476, _) => {
                        h476
                    }
                };
                length_list(a477)
            };
            let a478 = v8;
            let a479 = v9;
            if nat_eq(a478, a479) {
                f91(v4, v1, match v6.clone() {
                    Adt11::C0(h480, _) => {
                        h480
                    }
                }, match v7.clone() {
                    Adt11::C0(h481, _) => {
                        h481
                    }
                }, match v7 {
                    Adt11::C0(_, h482) => {
                        (*h482).clone()
                    }
                }, match v6 {
                    Adt11::C0(_, h483) => {
                        (*h483).clone()
                    }
                }, v2, v3)
            } else {
                let v10: Bytes = f23(v2)?;
                let v11: Bytes = f23(v3)?;
                Ok(f68(v1, f16(f16(f16(f16(f16(f16(v10, f11(Str::lit(" binds "))), f41(v8)), f11(Str::lit(" arguments but "))), v11), f11(Str::lit(" binds "))), f41(v9))))
            }
        }
    }
}

pub fn f68(v0: Adt6, v1: Bytes) -> Adt8 {
    Adt8::C0(Some(v1), std::rc::Rc::new(v0))
}

pub fn f69(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F92(v1), None::<Adt0>, v0)
}

pub fn f70(v0: Option<Adt0>, v1: Option<Adt0>) -> Option<(Adt0, Adt0)> {
    let m485 = v0;
    match m485 {
        None => {
            None::<(Adt0, Adt0)>
        }
        Some(v2) => {
            let m486 = v1;
            match m486 {
                None => {
                    None::<(Adt0, Adt0)>
                }
                Some(v3) => {
                    Some((v2, v3))
                }
            }
        }
    }
}

pub fn f71(v0: Adt4, v1: Adt0, v2: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m487 = v0;
    match m487 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r500) => {
            let v3 = (*r500).clone();
            let m488 = v1;
            match m488 {
                Adt0::C0 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C1(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C2(_, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C3(v4) => {
                    if f93(v2, v4.clone())? {
                        Ok(Some(List::<Bytes>::nil()))
                    } else {
                        Ok(Some(List::cons(v4, List::<Bytes>::nil())))
                    }
                }
                Adt0::C4(r489, r490) => {
                    let v5 = (*r489).clone();
                    let v6 = (*r490).clone();
                    Ok(f94(f71(v3.clone(), v5, v2.clone())?, f71(v3, v6, v2)?))
                }
                Adt0::C5(r491, r492) => {
                    let v9 = (*r491).clone();
                    let v10 = (*r492).clone();
                    f95(v3, v9, v2, v10)
                }
                Adt0::C6(r493, r494) => {
                    let v11 = (*r493).clone();
                    let v12 = (*r494).clone();
                    f95(v3, v11, v2, v12)
                }
                Adt0::C7(v13, r495, r496, r497) => {
                    let v14 = (*r495).clone();
                    let v15 = (*r496).clone();
                    let v16 = (*r497).clone();
                    Ok(f94(f94(f71(v3.clone(), v14, v2.clone())?, f71(v3.clone(), v15, v2.clone())?), f71(v3, v16, List::cons(v13, v2))?))
                }
                Adt0::C8(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C9 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C10(r498, r499) => {
                    let v7 = (*r498).clone();
                    let v8 = (*r499).clone();
                    Ok(f94(f71(v3.clone(), v7, v2.clone())?, f71(v3, v8, v2)?))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
            }
        }
    }
}

pub fn f72(v0: Fn4, p504: bool, p505: List<Bytes>) -> R<bool> {
    let m503 = (p504, p505);
    {
        let mut m503 = m503;
        loop {
            let step = {
                let (v1, v2) = m503;
                let m501 = v2;
                match m501.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c502: Fn4 = v0.clone();
                            c502.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        Ok(true)
    } else {
        f19(v1)
    }
}

pub fn f74(v0: Bytes, v1: u64) -> Bytes {
    let m509 = {
        let a506 = v0;
        let a507 = v1;
        let a508 = 1u64;
        slice_bytes(a506, a507, a508)
    };
    match m509 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f75(v0: Fn5, p514: u64, p515: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m513 = (p514, p515);
    {
        let mut m513 = m513;
        loop {
            let step = {
                let (v1, v2) = m513;
                let m510 = v1;
                if m510 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m510 - 1;
                    let m512 = {
                        let c511: Fn5 = v0.clone();
                        c511.apply(v2.clone())
                    };
                    match m512 {
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
                Ok(next) => { m513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f96(v1, v0, v2)
}

pub fn f77(v0: Str) -> Bytes {
    let a516 = v0;
    utf8_encode(a516)
}

pub fn f78(v0: Bytes, v1: Bytes) -> Bytes {
    let a517 = v0;
    let a518 = v1;
    append_bytes(a517, a518)
}

pub fn f79(v0: Adt1) -> Bytes {
    let m519 = v0;
    match m519 {
        Adt1::C0(v1) => {
            f41(v1)
        }
        Adt1::C1(v2) => {
            v2
        }
        Adt1::C2(r520) => {
            let v3 = (*r520).clone();
            f78(f78(f77(Str::lit("(succ ")), f79(v3)), f77(Str::lit(")")))
        }
        Adt1::C3(r521) => {
            let v4 = (*r521).clone();
            f78(f78(f77(Str::lit("(max")), f97(v4)), f77(Str::lit(")")))
        }
        Adt1::C4(r522, r523) => {
            let v5 = (*r522).clone();
            let v6 = (*r523).clone();
            f78(f78(f78(f78(f77(Str::lit("(imax ")), f79(v5)), f77(Str::lit(" "))), f79(v6)), f77(Str::lit(")")))
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f80(v0: Bytes, v1: Bytes, v2: Adt1) -> Bytes {
    let v3: Bytes = f78(f78(f78(f77(Str::lit("(const ")), v0), f77(Str::lit("::"))), v1);
    let m524 = v2.clone();
    match m524 {
        Adt1::C0(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C1(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C2(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C3(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C4(_, _) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C5 => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C6(_, _) => {
            let v6: Bytes = f97(v2);
            f78(f78(f78(v3, f77(Str::lit(" ("))), f98(v6.clone(), 1u64, {
                let a526 = {
                    let a525 = v6;
                    length_bytes(a525)
                };
                let a527 = 1u64;
                nat_sub(a526, a527)
            })), f77(Str::lit("))")))
        }
    }
}

pub fn f81(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m528 = f99(Fn7::F100(v1.clone()), None::<Bytes>, v0);
    match m528 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f82(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m529 = v0;
    match m529 {
        Adt0::C0 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C1(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C3(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C4(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C5(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C6(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C8(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(r534, r535) => {
            let v4 = (*r534).clone();
            let v5 = (*r535).clone();
            let v6: (Bytes, u64) = f59(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f82(v5, v1, v2, {
                let (_, h530) = v6;
                h530
            })?;
            Ok((f78(f78(f77(Str::lit(" ")), {
                let (h531, _) = v6;
                h531
            }), {
                let (h532, _) = v7;
                h532
            }), {
                let (_, h533) = v7;
                h533
            }))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f83(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m536 = v0;
    match m536 {
        Adt0::C0 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C1(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C3(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C4(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C5(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C6(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C8(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C9 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C10(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C11(v4, v5, r559, r560) => {
            let v6 = (*r559).clone();
            let v7 = (*r560).clone();
            let v8: (Bytes, u64) = f84(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f59(v6, v1, v2.clone(), {
                let (_, h537) = v8.clone();
                h537
            })?;
            let v10: Bytes = f78(f78(f78(f78(f78(f78(f77(Str::lit("(")), f101(v4)), f77(Str::lit(" "))), {
                let (h538, _) = v8.clone();
                h538
            }), f77(Str::lit(" "))), {
                let (h539, _) = v9;
                h539
            }), f77(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h540, _) = v8;
                h540
            }), v2);
            let m541 = v7.clone();
            match m541 {
                Adt0::C0 => {
                    Ok(((v10, v11), {
                        let (_, h549) = v9;
                        h549
                    }))
                }
                Adt0::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h550) = v9;
                        h550
                    }))
                }
                Adt0::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h551) = v9;
                        h551
                    }))
                }
                Adt0::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h552) = v9;
                        h552
                    }))
                }
                Adt0::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h553) = v9;
                        h553
                    }))
                }
                Adt0::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h554) = v9;
                        h554
                    }))
                }
                Adt0::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h555) = v9;
                        h555
                    }))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h556) = v9;
                        h556
                    }))
                }
                Adt0::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h557) = v9;
                        h557
                    }))
                }
                Adt0::C9 => {
                    Ok(((v10, v11), {
                        let (_, h542) = v9;
                        h542
                    }))
                }
                Adt0::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h558) = v9;
                        h558
                    }))
                }
                Adt0::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v7, v1, v11, {
                        let (_, h543) = v9;
                        h543
                    })?;
                    Ok(((f78(f78(v10, f77(Str::lit(" "))), {
                        let (h544, _) = {
                            let (h545, _) = v16.clone();
                            h545
                        };
                        h544
                    }), {
                        let (_, h546) = {
                            let (h547, _) = v16;
                            h547
                        };
                        h546
                    }), {
                        let (_, h548) = v16;
                        h548
                    }))
                }
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f78(f77(Str::lit("x")), f41(v2)), {
            let a561 = v2;
            let a562 = 1u64;
            nat_add(a561, a562)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f85(v0: Fn6, p566: Adt0, p567: List<Adt9>) -> Adt0 {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m563 = v2;
                match m563.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt9>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt9>), Adt0>(({
                            let c564: Fn6 = v0.clone();
                            c564.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Adt0, v1: Adt9) -> Adt0 {
    Adt0::C11(match v1.clone() {
        Adt9::C0(h568, _, _) => {
            h568
        }
    }, match v1.clone() {
        Adt9::C0(_, h569, _) => {
            h569
        }
    }, std::rc::Rc::new(match v1 {
        Adt9::C0(_, _, h570) => {
            (*h570).clone()
        }
    }), std::rc::Rc::new(v0))
}

pub fn f87(v0: List<Adt9>) -> List<Adt9> {
    f102(Fn8::F103, List::<Adt9>::nil(), v0)
}

pub fn f88(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: Bytes, v4: Adt0) -> R<Adt10> {
    let m571 = v0;
    match m571 {
        Adt4::C0 => {
            Ok(Adt10::C0(v2, std::rc::Rc::new(f8(v1))))
        }
        Adt4::C1(r580) => {
            let v5 = (*r580).clone();
            let m572 = v2;
            match m572.uncons() {
                None => {
                    Ok(Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(v1)))
                }
                Some((v6, v7)) => {
                    let v8: Adt7 = f89(v5.clone(), v1, match v6.clone() {
                        Adt9::C0(_, _, h573) => {
                            (*h573).clone()
                        }
                    }, v3.clone(), v4.clone())?;
                    let v9: Adt10 = f88(v5, match v8.clone() {
                        Adt7::C0(_, h574) => {
                            (*h574).clone()
                        }
                    }, v7, v3, v4)?;
                    Ok(Adt10::C0(List::cons(Adt9::C0(match v6.clone() {
                        Adt9::C0(h575, _, _) => {
                            h575
                        }
                    }, match v6 {
                        Adt9::C0(_, h576, _) => {
                            h576
                        }
                    }, std::rc::Rc::new(match v8 {
                        Adt7::C0(h577, _) => {
                            (*h577).clone()
                        }
                    })), match v9.clone() {
                        Adt10::C0(h578, _) => {
                            h578
                        }
                    }), std::rc::Rc::new(match v9 {
                        Adt10::C0(_, h579) => {
                            (*h579).clone()
                        }
                    })))
                }
            }
        }
    }
}

pub fn f89(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0) -> R<Adt7> {
    let m581 = f71(v0.clone(), v4.clone(), List::<Bytes>::nil())?;
    match m581 {
        None => {
            Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f8(v1))))
        }
        Some(v5) => {
            f104(v0, v1, v2, v3, v4, v5)
        }
    }
}

pub fn f90(v0: Adt4, v1: Adt0, v2: bool) -> Adt11 {
    let m582 = v0;
    match m582 {
        Adt4::C0 => {
            Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(v1))
        }
        Adt4::C1(r603) => {
            let v3 = (*r603).clone();
            let m583 = v1;
            match m583 {
                Adt0::C0 => {
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C0))
                }
                Adt0::C1(r588) => {
                    let v10 = (*r588).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C1(std::rc::Rc::new(v10))))
                }
                Adt0::C2(v11, v12, r589) => {
                    let v13 = (*r589).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C2(v11, v12, std::rc::Rc::new(v13))))
                }
                Adt0::C3(v14) => {
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C3(v14)))
                }
                Adt0::C4(r590, r591) => {
                    let v15 = (*r590).clone();
                    let v16 = (*r591).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(v15), std::rc::Rc::new(v16))))
                }
                Adt0::C5(r592, r593) => {
                    let v4 = (*r592).clone();
                    let v5 = (*r593).clone();
                    if v2 {
                        let v6: Adt11 = f90(v3.clone(), v5, v2);
                        Adt11::C0(f102(Fn8::F105, match v6.clone() {
                            Adt11::C0(h584, _) => {
                                h584
                            }
                        }, f87(f44(v3, v4))), std::rc::Rc::new(match v6 {
                            Adt11::C0(_, h585) => {
                                (*h585).clone()
                            }
                        }))
                    } else {
                        Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C5(std::rc::Rc::new(v4), std::rc::Rc::new(v5))))
                    }
                }
                Adt0::C6(r594, r595) => {
                    let v7 = (*r594).clone();
                    let v8 = (*r595).clone();
                    if v2 {
                        Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new(v8))))
                    } else {
                        let v9: Adt11 = f90(v3.clone(), v8, v2);
                        Adt11::C0(f102(Fn8::F106, match v9.clone() {
                            Adt11::C0(h586, _) => {
                                h586
                            }
                        }, f87(f44(v3, v7))), std::rc::Rc::new(match v9 {
                            Adt11::C0(_, h587) => {
                                (*h587).clone()
                            }
                        }))
                    }
                }
                Adt0::C7(v17, r596, r597, r598) => {
                    let v18 = (*r596).clone();
                    let v19 = (*r597).clone();
                    let v20 = (*r598).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C7(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19), std::rc::Rc::new(v20))))
                }
                Adt0::C8(v21) => {
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C8(v21)))
                }
                Adt0::C9 => {
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C9))
                }
                Adt0::C10(r599, r600) => {
                    let v22 = (*r599).clone();
                    let v23 = (*r600).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(v22), std::rc::Rc::new(v23))))
                }
                Adt0::C11(v24, v25, r601, r602) => {
                    let v26 = (*r601).clone();
                    let v27 = (*r602).clone();
                    Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C11(v24, v25, std::rc::Rc::new(v26), std::rc::Rc::new(v27))))
                }
            }
        }
    }
}

pub fn f91(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: List<Adt9>, v4: Adt0, v5: Adt0, v6: Adt0, v7: Adt0) -> R<Adt8> {
    let m604 = v0;
    match m604 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r629) => {
            let v8 = (*r629).clone();
            let m605 = v2;
            match m605.uncons() {
                None => {
                    f27(v8, v1, v5, v4)
                }
                Some((v9, v10)) => {
                    let m606 = v3;
                    match m606.uncons() {
                        None => {
                            f27(v8, v1, v5, v4)
                        }
                        Some((v11, v12)) => {
                            let a609 = match v9.clone() {
                                Adt9::C0(h607, _, _) => {
                                    h607
                                }
                            };
                            let a610 = match v11.clone() {
                                Adt9::C0(h608, _, _) => {
                                    h608
                                }
                            };
                            if nat_eq(a609, a610) {
                                let v13: Adt8 = f27(v8.clone(), v1, match v9.clone() {
                                    Adt9::C0(_, _, h611) => {
                                        (*h611).clone()
                                    }
                                }, match v11.clone() {
                                    Adt9::C0(_, _, h612) => {
                                        (*h612).clone()
                                    }
                                })?;
                                let m614 = match v13.clone() {
                                    Adt8::C0(h613, _) => {
                                        h613
                                    }
                                };
                                match m614 {
                                    None => {
                                        let v15: Adt0 = Adt0::C3(match v9 {
                                            Adt9::C0(_, h615, _) => {
                                                h615
                                            }
                                        });
                                        let v16: Adt10 = f88(v8.clone(), match v13 {
                                            Adt8::C0(_, h616) => {
                                                (*h616).clone()
                                            }
                                        }, v12, match v11.clone() {
                                            Adt9::C0(_, h617, _) => {
                                                h617
                                            }
                                        }, v15.clone())?;
                                        let v17: Adt7 = f89(v8.clone(), match v16.clone() {
                                            Adt10::C0(_, h618) => {
                                                (*h618).clone()
                                            }
                                        }, v4, match v11 {
                                            Adt9::C0(_, h619, _) => {
                                                h619
                                            }
                                        }, v15)?;
                                        let v18: Adt8 = f91(v8, match v17.clone() {
                                            Adt7::C0(_, h620) => {
                                                (*h620).clone()
                                            }
                                        }, v10, match v16 {
                                            Adt10::C0(h621, _) => {
                                                h621
                                            }
                                        }, match v17 {
                                            Adt7::C0(h622, _) => {
                                                (*h622).clone()
                                            }
                                        }, v5, v6, v7)?;
                                        Ok(Adt8::C0(match v18.clone() {
                                            Adt8::C0(h623, _) => {
                                                h623
                                            }
                                        }, std::rc::Rc::new(match v18 {
                                            Adt8::C0(_, h624) => {
                                                (*h624).clone()
                                            }
                                        })))
                                    }
                                    Some(_) => {
                                        Ok(v13)
                                    }
                                }
                            } else {
                                let v19: Bytes = f23(v6)?;
                                let v20: Bytes = f23(v7)?;
                                Ok(f68(v1, f16(f16(f16(f16(f16(f16(f16(f16(f16(f16(f16(f11(Str::lit("binder `")), match v9.clone() {
                                    Adt9::C0(_, h625, _) => {
                                        h625
                                    }
                                }), f11(Str::lit("` is "))), f107(match v9 {
                                    Adt9::C0(h626, _, _) => {
                                        h626
                                    }
                                })), f11(Str::lit(" in "))), v19), f11(Str::lit(" but `"))), match v11.clone() {
                                    Adt9::C0(_, h627, _) => {
                                        h627
                                    }
                                }), f11(Str::lit("` is "))), f107(match v11 {
                                    Adt9::C0(h628, _, _) => {
                                        h628
                                    }
                                })), f11(Str::lit(" in "))), v20)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m630 = v1.clone();
    match m630 {
        None => {
            let a632 = {
                let (h631, _) = v2;
                h631
            };
            let a633 = v0;
            if equal(a632, a633) {
                Some({
                    let (_, h634) = v2;
                    h634
                })
            } else {
                None::<Adt0>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f93(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f72(Fn4::F108(v1), false, v0)
}

pub fn f94(v0: Option<List<Bytes>>, v1: Option<List<Bytes>>) -> Option<List<Bytes>> {
    let m635 = v0;
    match m635 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m636 = v1;
            match m636 {
                None => {
                    None::<List<Bytes>>
                }
                Some(v3) => {
                    Some(f54(Fn3::F109, v3, v2))
                }
            }
        }
    }
}

pub fn f95(v0: Adt4, v1: Adt0, v2: List<Bytes>, v3: Adt0) -> R<Option<List<Bytes>>> {
    let m637 = v0;
    match m637 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r641) => {
            let v4 = (*r641).clone();
            let m638 = v1;
            match m638 {
                Adt0::C0 => {
                    f71(v4, v3, v2)
                }
                Adt0::C1(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C2(_, _, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C3(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C4(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C5(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C6(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C7(_, _, _, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C8(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C9 => {
                    f71(v4, v3, v2)
                }
                Adt0::C10(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C11(_, v6, r639, r640) => {
                    let v7 = (*r639).clone();
                    let v8 = (*r640).clone();
                    Ok(f94(f71(v4.clone(), v7, v2.clone())?, f95(v4, v8, List::cons(v6, v2), v3)?))
                }
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h642, _) = v2;
        h642
    };
    let a643 = v3;
    let a644 = 0u64;
    if nat_eq(a643, a644) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a645 = v3;
            let a646 = v1;
            let a647 = 0u64;
            nat_quot(a645, a646, a647)
        }, {
            let a652 = f74(v0, {
                let a648 = v3;
                let a649 = v1;
                let a650 = 0u64;
                nat_rem(a648, a649, a650)
            });
            let a653 = {
                let (_, h651) = v2;
                h651
            };
            append_bytes(a652, a653)
        }))
    }
}

pub fn f97(v0: Adt1) -> Bytes {
    let m654 = v0;
    match m654 {
        Adt1::C0(_) => {
            Bytes::lit(&[])
        }
        Adt1::C1(_) => {
            Bytes::lit(&[])
        }
        Adt1::C2(_) => {
            Bytes::lit(&[])
        }
        Adt1::C3(_) => {
            Bytes::lit(&[])
        }
        Adt1::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(r655, r656) => {
            let v7 = (*r655).clone();
            let v8 = (*r656).clone();
            f78(f78(f77(Str::lit(" ")), f79(v7)), f97(v8))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m660 = {
        let a657 = v0;
        let a658 = v1;
        let a659 = v2;
        slice_bytes(a657, a658, a659)
    };
    match m660 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f99(v0: Fn7, p664: Option<Bytes>, p665: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m663 = (p664, p665);
    {
        let mut m663 = m663;
        loop {
            let step = {
                let (v1, v2) = m663;
                let m661 = v2;
                match m661.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c662: Fn7 = v0.clone();
                            c662.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m663 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m666 = v1.clone();
    match m666 {
        None => {
            let a668 = {
                let (h667, _) = v2;
                h667
            };
            let a669 = v0;
            if equal(a668, a669) {
                Some({
                    let (_, h670) = v2;
                    h670
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

pub fn f101(v0: u64) -> Bytes {
    let a671 = v0;
    let a672 = 1u64;
    if nat_eq(a671, a672) {
        f77(Str::lit("implicit"))
    } else {
        let a673 = v0;
        let a674 = 2u64;
        if nat_eq(a673, a674) {
            f77(Str::lit("instance"))
        } else {
            f77(Str::lit("explicit"))
        }
    }
}

pub fn f102(v0: Fn8, p678: List<Adt9>, p679: List<Adt9>) -> List<Adt9> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v1, v2) = m677;
                let m675 = v2;
                match m675.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c676: Fn8 = v0.clone();
                            c676.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f104(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt7> {
    let m680 = v0;
    match m680 {
        Adt4::C0 => {
            Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f8(v1))))
        }
        Adt4::C1(r734) => {
            let v6 = (*r734).clone();
            let m681 = v2;
            match m681 {
                Adt0::C0 => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C0), std::rc::Rc::new(v1)))
                }
                Adt0::C1(r719) => {
                    let v38 = (*r719).clone();
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C1(std::rc::Rc::new(v38))), std::rc::Rc::new(v1)))
                }
                Adt0::C2(v39, v40, r720) => {
                    let v41 = (*r720).clone();
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C2(v39, v40, std::rc::Rc::new(v41))), std::rc::Rc::new(v1)))
                }
                Adt0::C3(v7) => {
                    let a682 = v7.clone();
                    let a683 = v3;
                    if equal(a682, a683) {
                        Ok(Adt7::C0(std::rc::Rc::new(v4), std::rc::Rc::new(v1)))
                    } else {
                        Ok(Adt7::C0(std::rc::Rc::new(Adt0::C3(v7)), std::rc::Rc::new(v1)))
                    }
                }
                Adt0::C4(r721, r722) => {
                    let v8 = (*r721).clone();
                    let v9 = (*r722).clone();
                    let v10: Adt7 = f104(v6.clone(), v1, v8, v3.clone(), v4.clone(), v5.clone())?;
                    let v11: Adt7 = f104(v6, match v10.clone() {
                        Adt7::C0(_, h684) => {
                            (*h684).clone()
                        }
                    }, v9, v3, v4, v5)?;
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(match v10 {
                        Adt7::C0(h685, _) => {
                            (*h685).clone()
                        }
                    }), std::rc::Rc::new(match v11.clone() {
                        Adt7::C0(h686, _) => {
                            (*h686).clone()
                        }
                    }))), std::rc::Rc::new(match v11 {
                        Adt7::C0(_, h687) => {
                            (*h687).clone()
                        }
                    })))
                }
                Adt0::C5(r723, r724) => {
                    let v16 = (*r723).clone();
                    let v17 = (*r724).clone();
                    let v18: Adt12 = f110(v6.clone(), v1, v16, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v19: Adt7 = f111(v6, v18.clone(), v17, v3, v4, v5)?;
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C5(std::rc::Rc::new(match v18 {
                        Adt12::C0(h692, _, _, _) => {
                            (*h692).clone()
                        }
                    }), std::rc::Rc::new(match v19.clone() {
                        Adt7::C0(h693, _) => {
                            (*h693).clone()
                        }
                    }))), std::rc::Rc::new(match v19 {
                        Adt7::C0(_, h694) => {
                            (*h694).clone()
                        }
                    })))
                }
                Adt0::C6(r725, r726) => {
                    let v20 = (*r725).clone();
                    let v21 = (*r726).clone();
                    let v22: Adt12 = f110(v6.clone(), v1, v20, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v23: Adt7 = f111(v6, v22.clone(), v21, v3, v4, v5)?;
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C6(std::rc::Rc::new(match v22 {
                        Adt12::C0(h695, _, _, _) => {
                            (*h695).clone()
                        }
                    }), std::rc::Rc::new(match v23.clone() {
                        Adt7::C0(h696, _) => {
                            (*h696).clone()
                        }
                    }))), std::rc::Rc::new(match v23 {
                        Adt7::C0(_, h697) => {
                            (*h697).clone()
                        }
                    })))
                }
                Adt0::C7(v24, r727, r728, r729) => {
                    let v25 = (*r727).clone();
                    let v26 = (*r728).clone();
                    let v27 = (*r729).clone();
                    let v28: Adt7 = f104(v6.clone(), v1, v25, v3.clone(), v4.clone(), v5.clone())?;
                    let v29: Adt7 = f104(v6.clone(), match v28.clone() {
                        Adt7::C0(_, h698) => {
                            (*h698).clone()
                        }
                    }, v26, v3.clone(), v4.clone(), v5.clone())?;
                    let a699 = v24.clone();
                    let a700 = v3.clone();
                    if equal(a699, a700) {
                        Ok(Adt7::C0(std::rc::Rc::new(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                            Adt7::C0(h701, _) => {
                                (*h701).clone()
                            }
                        }), std::rc::Rc::new(match v29.clone() {
                            Adt7::C0(h702, _) => {
                                (*h702).clone()
                            }
                        }), std::rc::Rc::new(v27))), std::rc::Rc::new(match v29 {
                            Adt7::C0(_, h703) => {
                                (*h703).clone()
                            }
                        })))
                    } else {
                        if f93(v5.clone(), v24.clone())? {
                            let v30: (Bytes, Adt6) = f112(match v29.clone() {
                                Adt7::C0(_, h704) => {
                                    (*h704).clone()
                                }
                            }, v24.clone())?;
                            let v31: Adt7 = f104(v6.clone(), {
                                let (_, h705) = v30.clone();
                                h705
                            }, v27, v24, Adt0::C3({
                                let (h706, _) = v30.clone();
                                h706
                            }), List::<Bytes>::nil())?;
                            let v32: Adt7 = f104(v6, match v31.clone() {
                                Adt7::C0(_, h707) => {
                                    (*h707).clone()
                                }
                            }, match v31 {
                                Adt7::C0(h708, _) => {
                                    (*h708).clone()
                                }
                            }, v3, v4, v5)?;
                            Ok(Adt7::C0(std::rc::Rc::new(Adt0::C7({
                                let (h709, _) = v30;
                                h709
                            }, std::rc::Rc::new(match v28 {
                                Adt7::C0(h710, _) => {
                                    (*h710).clone()
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt7::C0(h711, _) => {
                                    (*h711).clone()
                                }
                            }), std::rc::Rc::new(match v32.clone() {
                                Adt7::C0(h712, _) => {
                                    (*h712).clone()
                                }
                            }))), std::rc::Rc::new(match v32 {
                                Adt7::C0(_, h713) => {
                                    (*h713).clone()
                                }
                            })))
                        } else {
                            let v33: Adt7 = f104(v6, match v29.clone() {
                                Adt7::C0(_, h714) => {
                                    (*h714).clone()
                                }
                            }, v27, v3, v4, v5)?;
                            Ok(Adt7::C0(std::rc::Rc::new(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                                Adt7::C0(h715, _) => {
                                    (*h715).clone()
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt7::C0(h716, _) => {
                                    (*h716).clone()
                                }
                            }), std::rc::Rc::new(match v33.clone() {
                                Adt7::C0(h717, _) => {
                                    (*h717).clone()
                                }
                            }))), std::rc::Rc::new(match v33 {
                                Adt7::C0(_, h718) => {
                                    (*h718).clone()
                                }
                            })))
                        }
                    }
                }
                Adt0::C8(v42) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C8(v42)), std::rc::Rc::new(v1)))
                }
                Adt0::C9 => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C9), std::rc::Rc::new(v1)))
                }
                Adt0::C10(r730, r731) => {
                    let v12 = (*r730).clone();
                    let v13 = (*r731).clone();
                    let v14: Adt7 = f104(v6.clone(), v1, v12, v3.clone(), v4.clone(), v5.clone())?;
                    let v15: Adt7 = f104(v6, match v14.clone() {
                        Adt7::C0(_, h688) => {
                            (*h688).clone()
                        }
                    }, v13, v3, v4, v5)?;
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(match v14 {
                        Adt7::C0(h689, _) => {
                            (*h689).clone()
                        }
                    }), std::rc::Rc::new(match v15.clone() {
                        Adt7::C0(h690, _) => {
                            (*h690).clone()
                        }
                    }))), std::rc::Rc::new(match v15 {
                        Adt7::C0(_, h691) => {
                            (*h691).clone()
                        }
                    })))
                }
                Adt0::C11(v34, v35, r732, r733) => {
                    let v36 = (*r732).clone();
                    let v37 = (*r733).clone();
                    Ok(Adt7::C0(std::rc::Rc::new(Adt0::C11(v34, v35, std::rc::Rc::new(v36), std::rc::Rc::new(v37))), std::rc::Rc::new(v1)))
                }
            }
        }
    }
}

pub fn f105(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f106(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f107(v0: u64) -> Bytes {
    f101(v0)
}

pub fn f108(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a735 = v2;
        let a736 = v0;
        equal(a735, a736)
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>, v6: List<(Bytes, Bytes)>, v7: bool) -> R<Adt12> {
    let m737 = v0;
    match m737 {
        Adt4::C0 => {
            Ok(Adt12::C0(std::rc::Rc::new(v2), v6, v7, std::rc::Rc::new(f8(v1))))
        }
        Adt4::C1(r764) => {
            let v8 = (*r764).clone();
            let m738 = v2.clone();
            match m738 {
                Adt0::C0 => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C1(_) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C2(_, _, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C3(_) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C4(_, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C5(_, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C6(_, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C8(_) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C9 => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C10(_, _) => {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C9), v6, v7, std::rc::Rc::new(v1)))
                }
                Adt0::C11(v9, v10, r762, r763) => {
                    let v11 = (*r762).clone();
                    let v12 = (*r763).clone();
                    let v13: Adt7 = f111(v8.clone(), Adt12::C0(std::rc::Rc::new(v2), v6.clone(), v7, std::rc::Rc::new(v1)), v11, v3.clone(), v4.clone(), v5.clone())?;
                    let v14: Adt7 = v13;
                    let a739 = v10.clone();
                    let a740 = v3.clone();
                    if equal(a739, a740) {
                        let v15: Adt12 = f110(v8, match v14.clone() {
                            Adt7::C0(_, h741) => {
                                (*h741).clone()
                            }
                        }, v12, v3, v4, v5, v6, true)?;
                        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                            Adt7::C0(h742, _) => {
                                (*h742).clone()
                            }
                        }), std::rc::Rc::new(match v15.clone() {
                            Adt12::C0(h743, _, _, _) => {
                                (*h743).clone()
                            }
                        }))), match v15.clone() {
                            Adt12::C0(_, h744, _, _) => {
                                h744
                            }
                        }, match v15.clone() {
                            Adt12::C0(_, _, h745, _) => {
                                h745
                            }
                        }, std::rc::Rc::new(match v15 {
                            Adt12::C0(_, _, _, h746) => {
                                (*h746).clone()
                            }
                        })))
                    } else {
                        if f93(v5.clone(), v10.clone())? {
                            let v16: (Bytes, Adt6) = f112(match v14.clone() {
                                Adt7::C0(_, h747) => {
                                    (*h747).clone()
                                }
                            }, v10.clone())?;
                            let v17: Adt12 = f110(v8, {
                                let (_, h748) = v16.clone();
                                h748
                            }, v12, v3, v4, v5, List::cons((v10, {
                                let (h749, _) = v16.clone();
                                h749
                            }), v6), v7)?;
                            Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, {
                                let (h750, _) = v16;
                                h750
                            }, std::rc::Rc::new(match v14 {
                                Adt7::C0(h751, _) => {
                                    (*h751).clone()
                                }
                            }), std::rc::Rc::new(match v17.clone() {
                                Adt12::C0(h752, _, _, _) => {
                                    (*h752).clone()
                                }
                            }))), match v17.clone() {
                                Adt12::C0(_, h753, _, _) => {
                                    h753
                                }
                            }, match v17.clone() {
                                Adt12::C0(_, _, h754, _) => {
                                    h754
                                }
                            }, std::rc::Rc::new(match v17 {
                                Adt12::C0(_, _, _, h755) => {
                                    (*h755).clone()
                                }
                            })))
                        } else {
                            let v18: Adt12 = f110(v8, match v14.clone() {
                                Adt7::C0(_, h756) => {
                                    (*h756).clone()
                                }
                            }, v12, v3, v4, v5, v6, v7)?;
                            Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                                Adt7::C0(h757, _) => {
                                    (*h757).clone()
                                }
                            }), std::rc::Rc::new(match v18.clone() {
                                Adt12::C0(h758, _, _, _) => {
                                    (*h758).clone()
                                }
                            }))), match v18.clone() {
                                Adt12::C0(_, h759, _, _) => {
                                    h759
                                }
                            }, match v18.clone() {
                                Adt12::C0(_, _, h760, _) => {
                                    h760
                                }
                            }, std::rc::Rc::new(match v18 {
                                Adt12::C0(_, _, _, h761) => {
                                    (*h761).clone()
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Adt4, v1: Adt12, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt7> {
    let m765 = v0;
    match m765 {
        Adt4::C0 => {
            Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f8(match v1 {
                Adt12::C0(_, _, _, h766) => {
                    (*h766).clone()
                }
            }))))
        }
        Adt4::C1(r772) => {
            let v6 = (*r772).clone();
            let v7: Adt7 = f113(v6.clone(), match v1.clone() {
                Adt12::C0(_, _, _, h767) => {
                    (*h767).clone()
                }
            }, v2, f114(match v1.clone() {
                Adt12::C0(_, h768, _, _) => {
                    h768
                }
            }))?;
            if match v1 {
                Adt12::C0(_, _, h769, _) => {
                    h769
                }
            } {
                Ok(v7)
            } else {
                f104(v6, match v7.clone() {
                    Adt7::C0(_, h770) => {
                        (*h770).clone()
                    }
                }, match v7 {
                    Adt7::C0(h771, _) => {
                        (*h771).clone()
                    }
                }, v3, v4, v5)
            }
        }
    }
}

pub fn f112(v0: Adt6, v1: Bytes) -> R<(Bytes, Adt6)> {
    let v2: u64 = {
        let a774 = match v0.clone() {
            Adt6::C0(_, _, _, h773, _, _, _) => {
                h773
            }
        };
        let a775 = 1u64;
        nat_add(a774, a775)?
    };
    Ok((f16(f16(v1, Bytes::lit(&[0x23])), f41(v2)), f13(v0.clone(), match v0.clone() {
        Adt6::C0(h776, _, _, _, _, _, _) => {
            h776
        }
    }, match v0.clone() {
        Adt6::C0(_, h777, _, _, _, _, _) => {
            h777
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h778, _, _, _, _) => {
            h778
        }
    }, v2, match v0 {
        Adt6::C0(_, _, _, _, h779, _, _) => {
            h779
        }
    })))
}

pub fn f113(p788: Adt4, p789: Adt6, p790: Adt0, p791: List<(Bytes, Bytes)>) -> R<Adt7> {
    let m787 = (p788, (p789, (p790, p791)));
    {
        let mut m787 = m787;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m787;
                let m780 = v0;
                match m780 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>(Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f8(v1)))))
                    }
                    Adt4::C1(r786) => {
                        let v4 = (*r786).clone();
                        let m781 = v3;
                        match m781.uncons() {
                            None => {
                                Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>(Ok(Adt7::C0(std::rc::Rc::new(v2), std::rc::Rc::new(v1))))
                            }
                            Some((v5, v6)) => {
                                let v7: Adt7 = f104(v4.clone(), v1, v2, {
                                    let (h782, _) = v5;
                                    h782
                                }, Adt0::C3({
                                    let (_, h783) = v5;
                                    h783
                                }), List::<Bytes>::nil())?;
                                Ok::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>((v4, (match v7.clone() {
                                    Adt7::C0(_, h784) => {
                                        (*h784).clone()
                                    }
                                }, (match v7 {
                                    Adt7::C0(h785, _) => {
                                        (*h785).clone()
                                    }
                                }, v6))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f115(Fn9::F116, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f115(v0: Fn9, p795: List<(Bytes, Bytes)>, p796: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m794 = (p795, p796);
    {
        let mut m794 = m794;
        loop {
            let step = {
                let (v1, v2) = m794;
                let m792 = v2;
                match m792.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c793: Fn9 = v0.clone();
                            c793.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}
