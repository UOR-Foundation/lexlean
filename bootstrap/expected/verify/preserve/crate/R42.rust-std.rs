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
    C0(Bytes, bool, Bytes, List<Adt1>, List<Adt1>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Adt2, bool, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt5, u64, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4, p1: Adt1) -> R<Adt4> {
        match self {
            Fn0::F8 => f8(p0, p1),
            Fn0::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt4, p1: Bytes) -> R<Adt4> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: (bool, Adt4), p1: Adt3) -> R<(bool, Adt4)> {
        match self {
            Fn2::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28,
}

impl Fn3 {
    pub fn apply(&self, p0: (bool, Adt4), p1: Bytes) -> R<(bool, Adt4)> {
        match self {
            Fn3::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F30(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
        match self {
            Fn4::F30(k0) => f30(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F38,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn5::F38 => f38(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F40,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F40 => f40(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F53,
}

impl Fn7 {
    pub fn apply(&self, p0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
        match self {
            Fn7::F53 => f53(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F57(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F57(k0) => f57(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F60,
}

impl Fn9 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn9::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F63(Bytes, u64),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
        match self {
            Fn10::F63(k0, k1) => f63(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F77,
}

impl Fn11 {
    pub fn apply(&self, p0: (Adt7, Adt6)) -> (Adt7, Adt6) {
        match self {
            Fn11::F77 => f77(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F79(u64, Bytes, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt5)) -> R<(u64, Adt5)> {
        match self {
            Fn12::F79(k0, k1, k2) => f79(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F83,
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn13::F83 => f83(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F86,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt8) -> Adt8 {
        match self {
            Fn14::F86 => f86(p0),
        }
    }
}

pub fn f0(v0: u64, v1: Adt0, v2: Bytes, v3: Bytes, v4: bool, v5: Bytes, v6: Bytes, v7: Bytes, v8: Bytes, v9: List<Bytes>, v10: List<Bytes>, v11: List<Adt3>, v12: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let m1 = f1(v0, v1.clone(), v2.clone(), v3.clone())?;
    match m1 {
        Ok(v14) => {
            let m3 = f2(v0, v2, v3, v4, v5, v6, if v4 {
                f3(v7, v8)
            } else {
                v7
            }, v9, v10, v11, match v1 {
                Adt0::C0(_, _, h2, _, _, _) => {
                    h2
                }
            }, v14.clone(), v12)?;
            match m3 {
                Ok(v16) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v14, v16)))
                }
                Err(v15) => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(v15))
                }
            }
        }
        Err(v13) => {
            Ok(Err::<(Bytes, Bytes), Bytes>(v13))
        }
    }
}

pub fn f1(v0: u64, v1: Adt0, v2: Bytes, v3: Bytes) -> R<Result<Bytes, Bytes>> {
    let v4: Adt4 = f4(f5(f4(if match v1.clone() {
        Adt0::C0(_, h4, _, _, _, _) => {
            h4
        }
    } {
        f4(f5(f4(f5(f4(f5(f4(f6(v0), Str::lit("[package]\u{a}name = \""))?, v2)?, Str::lit("\"\u{a}version = \""))?, v3)?, Str::lit("\"\u{a}edition = \""))?, match v1.clone() {
            Adt0::C0(h5, _, _, _, _, _) => {
                h5
            }
        })?, Str::lit("\"\u{a}publish = true"))?
    } else {
        f4(f5(f4(f5(f4(f5(f4(f6(v0), Str::lit("[package]\u{a}name = \""))?, v2)?, Str::lit("\"\u{a}version = \""))?, v3)?, Str::lit("\"\u{a}edition = \""))?, match v1.clone() {
            Adt0::C0(h6, _, _, _, _, _) => {
                h6
            }
        })?, Str::lit("\"\u{a}publish = false"))?
    }, Str::lit("\u{a}\u{a}[lib]\u{a}path = \""))?, match v1.clone() {
        Adt0::C0(_, _, h7, _, _, _) => {
            h7
        }
    })?, Str::lit("\"\u{a}\u{a}[lints.rust]\u{a}"))?;
    let v5: Adt4 = f7(Fn0::F8, v4, match v1.clone() {
        Adt0::C0(_, _, _, h8, _, _) => {
            h8
        }
    })?;
    let v6: Adt4 = f7(Fn0::F9, f4(v5, Str::lit("\u{a}[lints.clippy]\u{a}"))?, match v1.clone() {
        Adt0::C0(_, _, _, _, h9, _) => {
            h9
        }
    })?;
    Ok(f10(f11(Fn1::F12, v6, match v1 {
        Adt0::C0(_, _, _, _, _, h10) => {
            h10
        }
    })?))
}

pub fn f2(v0: u64, v1: Bytes, v2: Bytes, v3: bool, v4: Bytes, v5: Bytes, v6: Bytes, v7: List<Bytes>, v8: List<Bytes>, v9: List<Adt3>, v10: Bytes, v11: Bytes, v12: Bytes) -> R<Result<Bytes, Bytes>> {
    let v13: Adt4 = f13(f4(f14(f4(f15(f4(f6(v0), Str::lit("{\"compiler_semantics\":"))?, v5)?, Str::lit(",\"effects\":"))?, v8)?, Str::lit(",\"exports\":"))?, v9)?;
    let v14: Adt4 = f16(v13, v10, v11, v12)?;
    let v15: Adt4 = f15(f4(f15(f4(v14, Str::lit(",\"name\":"))?, v1)?, Str::lit(",\"profile\":"))?, if v3 {
        f17(Str::lit("rust-std"))
    } else {
        f17(Str::lit("rust-core"))
    })?;
    Ok(f10(f4(f15(f4(f14(f4(f15(f4(f15(f4(v15, Str::lit(",\"program\":"))?, v4)?, Str::lit(",\"runtime\":"))?, f18(v6)?)?, Str::lit(",\"sources\":"))?, v7)?, Str::lit(",\"spec\":\"lexlean/rust-provenance/2\",\"version\":"))?, v2)?, Str::lit("}\u{a}"))?))
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    let a11 = v0;
    let a12 = v1;
    append_bytes(a11, a12)
}

pub fn f4(v0: Adt4, v1: Str) -> R<Adt4> {
    f5(v0, {
        let a13 = v1;
        utf8_encode(a13)
    })
}

pub fn f5(v0: Adt4, v1: Bytes) -> R<Adt4> {
    if match v0.clone() {
        Adt4::C0(_, _, _, _, h14) => {
            h14
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a17 = match v0.clone() {
                Adt4::C0(_, _, h15, _, _) => {
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
            Adt4::C0(_, _, _, h19, _) => {
                h19
            }
        };
        if nat_le(a20, a21) {
            Ok(f19(v0.clone(), {
                let a23 = match v0 {
                    Adt4::C0(_, h22, _, _, _) => {
                        h22
                    }
                };
                let a24 = v1;
                append_bytes(a23, a24)
            }, v2))
        } else {
            Ok(f20(v0))
        }
    }
}

pub fn f6(v0: u64) -> Adt4 {
    Adt4::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f7(v0: Fn0, p28: Adt4, p29: List<Adt1>) -> R<Adt4> {
    let m27 = (p28, p29);
    {
        let mut m27 = m27;
        loop {
            let step = {
                let (v1, v2) = m27;
                let m25 = v2;
                match m25.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt1>), R<Adt4>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt1>), R<Adt4>>(({
                            let c26: Fn0 = v0.clone();
                            c26.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m27 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt4, v1: Adt1) -> R<Adt4> {
    f21(v0, v1)
}

pub fn f9(v0: Adt4, v1: Adt1) -> R<Adt4> {
    f21(v0, v1)
}

pub fn f10(v0: Adt4) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt4::C0(_, _, _, _, h30) => {
            h30
        }
    } {
        Err::<Bytes, Bytes>({
            let a31 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a31)
        })
    } else {
        Ok::<Bytes, Bytes>(f22(f23(List::cons(match v0.clone() {
            Adt4::C0(_, h32, _, _, _) => {
                h32
            }
        }, match v0 {
            Adt4::C0(h33, _, _, _, _) => {
                h33
            }
        }))))
    }
}

pub fn f11(v0: Fn1, p37: Adt4, p38: List<Bytes>) -> R<Adt4> {
    let m36 = (p37, p38);
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, v2) = m36;
                let m34 = v2;
                match m34.uncons() {
                    None => {
                        Err::<(Adt4, List<Bytes>), R<Adt4>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Bytes>), R<Adt4>>(({
                            let c35: Fn1 = v0.clone();
                            c35.apply(v1, v3)?
                        }, v4))
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

pub fn f12(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f24(v0, v1)
}

pub fn f13(v0: Adt4, v1: List<Adt3>) -> R<Adt4> {
    f4({
        let (_, h39) = f25(Fn2::F26, (true, f4(v0, Str::lit("["))?), v1)?;
        h39
    }, Str::lit("]"))
}

pub fn f14(v0: Adt4, v1: List<Bytes>) -> R<Adt4> {
    f4({
        let (_, h40) = f27(Fn3::F28, (true, f4(v0, Str::lit("["))?), v1)?;
        h40
    }, Str::lit("]"))
}

pub fn f15(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f4({
        let (_, h41) = {
            let (h42, _) = f29(Fn4::F30(v1.clone()), {
                let a43 = v1;
                length_bytes(a43)
            }, (0u64, f4(v0, Str::lit("\""))?))?;
            h42
        };
        h41
    }, Str::lit("\""))
}

pub fn f16(v0: Adt4, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Adt4> {
    let v4: Adt4 = f4(v0, Str::lit(",\"files\":{"))?;
    f4(if f31(v1.clone(), f17(Str::lit("Cargo.toml"))) {
        f32(v4, v1, f18(v3)?)?
    } else {
        if f33(v1.clone(), f17(Str::lit("Cargo.toml"))) {
            f32(f4(f32(v4, v1, f18(v3)?)?, Str::lit(","))?, f17(Str::lit("Cargo.toml")), f18(v2)?)?
        } else {
            f32(f4(f32(v4, f17(Str::lit("Cargo.toml")), f18(v2)?)?, Str::lit(","))?, v1, f18(v3)?)?
        }
    }, Str::lit("}"))
}

pub fn f17(v0: Str) -> Bytes {
    let a44 = v0;
    utf8_encode(a44)
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    let a46 = {
        let a45 = Str::lit("blake3:");
        utf8_encode(a45)
    };
    let a47 = f34(v0)?;
    Ok(append_bytes(a46, a47))
}

pub fn f19(v0: Adt4, v1: Bytes, v2: u64) -> Adt4 {
    let a49 = 256u64;
    let a50 = {
        let a48 = v1.clone();
        length_bytes(a48)
    };
    if nat_le(a49, a50) {
        Adt4::C0(List::cons(v1, match v0.clone() {
            Adt4::C0(h51, _, _, _, _) => {
                h51
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt4::C0(_, _, _, h52, _) => {
                h52
            }
        }, false)
    } else {
        Adt4::C0(match v0.clone() {
            Adt4::C0(h53, _, _, _, _) => {
                h53
            }
        }, v1, v2, match v0 {
            Adt4::C0(_, _, _, h54, _) => {
                h54
            }
        }, false)
    }
}

pub fn f20(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(h55, _, _, _, _) => {
            h55
        }
    }, match v0.clone() {
        Adt4::C0(_, h56, _, _, _) => {
            h56
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h57, _, _) => {
            h57
        }
    }, match v0 {
        Adt4::C0(_, _, _, h58, _) => {
            h58
        }
    }, true)
}

pub fn f21(v0: Adt4, v1: Adt1) -> R<Adt4> {
    let v2: Adt4 = f5(v0, match v1.clone() {
        Adt1::C0(h59, _, _, _) => {
            h59
        }
    })?;
    let a61 = match v1.clone() {
        Adt1::C0(_, _, _, h60) => {
            h60
        }
    };
    let a62 = 0u64;
    if nat_eq(a61, a62) {
        f4(f4(f4(v2, Str::lit(" = \""))?, f35(match v1 {
            Adt1::C0(_, h63, _, _) => {
                h63
            }
        }))?, Str::lit("\"\u{a}"))
    } else {
        f4(f36(if match v1.clone() {
            Adt1::C0(_, _, h64, _) => {
                h64
            }
        } {
            f4(f4(f4(v2, Str::lit(" = { level = \""))?, f35(match v1.clone() {
                Adt1::C0(_, h65, _, _) => {
                    h65
                }
            }))?, Str::lit("\", priority = -"))?
        } else {
            f4(f4(f4(v2, Str::lit(" = { level = \""))?, f35(match v1.clone() {
                Adt1::C0(_, h66, _, _) => {
                    h66
                }
            }))?, Str::lit("\", priority = "))?
        }, match v1 {
            Adt1::C0(_, _, _, h67) => {
                h67
            }
        })?, Str::lit(" }\u{a}"))
    }
}

pub fn f22(v0: List<Bytes>) -> Bytes {
    let m70 = {
        let (h68, _) = f37(Fn5::F38, {
            let a69 = v0.clone();
            length_list(a69)
        }, v0);
        h68
    };
    match m70.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f23(v0: List<Bytes>) -> List<Bytes> {
    f39(Fn6::F40, List::<Bytes>::nil(), v0)
}

pub fn f24(v0: Adt4, v1: Bytes) -> R<Adt4> {
    f4(f5(v0, v1)?, Str::lit(" = \"allow\"\u{a}"))
}

pub fn f25(v0: Fn2, p74: (bool, Adt4), p75: List<Adt3>) -> R<(bool, Adt4)> {
    let m73 = (p74, p75);
    {
        let mut m73 = m73;
        loop {
            let step = {
                let (v1, v2) = m73;
                let m71 = v2;
                match m71.uncons() {
                    None => {
                        Err::<((bool, Adt4), List<Adt3>), R<(bool, Adt4)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt4), List<Adt3>), R<(bool, Adt4)>>(({
                            let c72: Fn2 = v0.clone();
                            c72.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m73 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (bool, Adt4), v1: Adt3) -> R<(bool, Adt4)> {
    f41(v0, v1)
}

pub fn f27(v0: Fn3, p79: (bool, Adt4), p80: List<Bytes>) -> R<(bool, Adt4)> {
    let m78 = (p79, p80);
    {
        let mut m78 = m78;
        loop {
            let step = {
                let (v1, v2) = m78;
                let m76 = v2;
                match m76.uncons() {
                    None => {
                        Err::<((bool, Adt4), List<Bytes>), R<(bool, Adt4)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt4), List<Bytes>), R<(bool, Adt4)>>(({
                            let c77: Fn3 = v0.clone();
                            c77.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m78 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: (bool, Adt4), v1: Bytes) -> R<(bool, Adt4)> {
    f42(v0, v1)
}

pub fn f29(v0: Fn4, p85: u64, p86: (u64, Adt4)) -> R<((u64, Adt4), bool)> {
    let m84 = (p85, p86);
    {
        let mut m84 = m84;
        loop {
            let step = {
                let (v1, v2) = m84;
                let m81 = v1;
                if m81 == 0 {
                    Err::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m81 - 1;
                    let m83 = {
                        let c82: Fn4 = v0.clone();
                        c82.apply(v2.clone())?
                    };
                    match m83 {
                        None => {
                            Err::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt4)), R<((u64, Adt4), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
    f43(v0, v1)
}

pub fn f31(v0: Bytes, v1: Bytes) -> bool {
    let a91 = {
        let a87 = v0;
        let a88 = v1;
        compare_bytes(a87, a88)
    };
    let a92 = {
        let a89 = Bytes::lit(&[]);
        let a90 = Bytes::lit(&[]);
        compare_bytes(a89, a90)
    };
    equal(a91, a92)
}

pub fn f32(v0: Adt4, v1: Bytes, v2: Bytes) -> R<Adt4> {
    f15(f4(f15(v0, v1)?, Str::lit(":"))?, v2)
}

pub fn f33(v0: Bytes, v1: Bytes) -> bool {
    let a97 = {
        let a93 = v0;
        let a94 = v1;
        compare_bytes(a93, a94)
    };
    let a98 = {
        let a95 = Bytes::lit(&[]);
        let a96 = Bytes::lit(&[0x01]);
        compare_bytes(a95, a96)
    };
    equal(a97, a98)
}

pub fn f34(v0: Bytes) -> R<Bytes> {
    let v1: Adt5 = f44(v0)?;
    let a119 = f45(match v1.clone() {
        Adt5::C0(h99, _, _, _, _, _, _, _) => {
            h99
        }
    });
    let a120 = {
        let a117 = f45(match v1.clone() {
            Adt5::C0(_, h100, _, _, _, _, _, _) => {
                h100
            }
        });
        let a118 = {
            let a115 = f45(match v1.clone() {
                Adt5::C0(_, _, h101, _, _, _, _, _) => {
                    h101
                }
            });
            let a116 = {
                let a113 = f45(match v1.clone() {
                    Adt5::C0(_, _, _, h102, _, _, _, _) => {
                        h102
                    }
                });
                let a114 = {
                    let a111 = f45(match v1.clone() {
                        Adt5::C0(_, _, _, _, h103, _, _, _) => {
                            h103
                        }
                    });
                    let a112 = {
                        let a109 = f45(match v1.clone() {
                            Adt5::C0(_, _, _, _, _, h104, _, _) => {
                                h104
                            }
                        });
                        let a110 = {
                            let a107 = f45(match v1.clone() {
                                Adt5::C0(_, _, _, _, _, _, h105, _) => {
                                    h105
                                }
                            });
                            let a108 = f45(match v1 {
                                Adt5::C0(_, _, _, _, _, _, _, h106) => {
                                    h106
                                }
                            });
                            append_bytes(a107, a108)
                        };
                        append_bytes(a109, a110)
                    };
                    append_bytes(a111, a112)
                };
                append_bytes(a113, a114)
            };
            append_bytes(a115, a116)
        };
        append_bytes(a117, a118)
    };
    Ok(append_bytes(a119, a120))
}

pub fn f35(v0: Adt2) -> Str {
    let m121 = v0;
    match m121 {
        Adt2::C0 => {
            Str::lit("allow")
        }
        Adt2::C1 => {
            Str::lit("warn")
        }
        Adt2::C2 => {
            Str::lit("deny")
        }
        Adt2::C3 => {
            Str::lit("forbid")
        }
    }
}

pub fn f36(v0: Adt4, v1: u64) -> R<Adt4> {
    f5(v0, f46(v1)?)
}

pub fn f37(v0: Fn5, p126: u64, p127: List<Bytes>) -> (List<Bytes>, bool) {
    let m125 = (p126, p127);
    {
        let mut m125 = m125;
        loop {
            let step = {
                let (v1, v2) = m125;
                let m122 = v1;
                if m122 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m122 - 1;
                    let m124 = {
                        let c123: Fn5 = v0.clone();
                        c123.apply(v2.clone())
                    };
                    match m124 {
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
                Ok(next) => { m125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<Bytes>) -> Option<List<Bytes>> {
    f47(v0)
}

pub fn f39(v0: Fn6, p131: List<Bytes>, p132: List<Bytes>) -> List<Bytes> {
    let m130 = (p131, p132);
    {
        let mut m130 = m130;
        loop {
            let step = {
                let (v1, v2) = m130;
                let m128 = v2;
                match m128.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c129: Fn6 = v0.clone();
                            c129.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f41(v0: (bool, Adt4), v1: Adt3) -> R<(bool, Adt4)> {
    Ok((false, f4(f15(f4(f36(f4({
        let (h133, _) = v0;
        if h133 {
            let (_, h134) = v0;
            h134
        } else {
            f4({
                let (_, h135) = v0;
                h135
            }, Str::lit(","))?
        }
    }, Str::lit("{\"function\":"))?, match v1.clone() {
        Adt3::C0(_, h136) => {
            h136
        }
    })?, Str::lit(",\"name\":"))?, match v1 {
        Adt3::C0(h137, _) => {
            h137
        }
    })?, Str::lit("}"))?))
}

pub fn f42(v0: (bool, Adt4), v1: Bytes) -> R<(bool, Adt4)> {
    Ok((false, f15({
        let (h138, _) = v0;
        if h138 {
            let (_, h139) = v0;
            h139
        } else {
            f4({
                let (_, h140) = v0;
                h140
            }, Str::lit(","))?
        }
    }, v1)?))
}

pub fn f43(v0: Bytes, v1: (u64, Adt4)) -> R<Option<(u64, Adt4)>> {
    let m142 = f48(v0, {
        let (h141, _) = v1.clone();
        h141
    });
    match m142 {
        None => {
            Ok(None::<(u64, Adt4)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a144 = {
                    let (h143, _) = v1;
                    h143
                };
                let a145 = 1u64;
                nat_add(a144, a145)?
            }, f5({
                let (_, h146) = v1;
                h146
            }, f49(v2)?)?)))
        }
    }
}

pub fn f44(v0: Bytes) -> R<Adt5> {
    let v1: u64 = {
        let a147 = v0.clone();
        length_bytes(a147)
    };
    let v2: u64 = {
        let a148 = v1;
        let a149 = 0u64;
        if nat_eq(a148, a149) {
            1u64
        } else {
            let a152 = {
                let a150 = v1;
                let a151 = 1023u64;
                nat_add(a150, a151)?
            };
            let a153 = 1024u64;
            let a154 = 0u64;
            nat_quot(a152, a153, a154)
        }
    };
    let v3: u64 = {
        let a155 = v2;
        let a156 = 1u64;
        nat_sub(a155, a156)
    };
    let m158 = {
        let (_, h157) = f50(v0.clone(), v1, v3)?;
        h157
    };
    match m158.uncons() {
        None => {
            f51(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h159) = f52(Fn7::F53, 64u64, (List::cons(v4, v5), f51(v0, v1, v3, false)?));
            Ok(h159)
        }
    }
}

pub fn f45(v0: u32) -> Bytes {
    let a172 = {
        let a160 = f54(v0, 4u32);
        let a161 = f54(v0, 0u32);
        append_bytes(a160, a161)
    };
    let a173 = {
        let a170 = {
            let a162 = f54(v0, 12u32);
            let a163 = f54(v0, 8u32);
            append_bytes(a162, a163)
        };
        let a171 = {
            let a168 = {
                let a164 = f54(v0, 20u32);
                let a165 = f54(v0, 16u32);
                append_bytes(a164, a165)
            };
            let a169 = {
                let a166 = f54(v0, 28u32);
                let a167 = f54(v0, 24u32);
                append_bytes(a166, a167)
            };
            append_bytes(a168, a169)
        };
        append_bytes(a170, a171)
    };
    append_bytes(a172, a173)
}

pub fn f46(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a210 = f55(v0, 10000000000000000000u64);
        let a211 = {
            let a208 = f55(v0, 1000000000000000000u64);
            let a209 = {
                let a206 = f55(v0, 100000000000000000u64);
                let a207 = {
                    let a204 = f55(v0, 10000000000000000u64);
                    let a205 = {
                        let a202 = f55(v0, 1000000000000000u64);
                        let a203 = {
                            let a200 = f55(v0, 100000000000000u64);
                            let a201 = {
                                let a198 = f55(v0, 10000000000000u64);
                                let a199 = {
                                    let a196 = f55(v0, 1000000000000u64);
                                    let a197 = {
                                        let a194 = f55(v0, 100000000000u64);
                                        let a195 = {
                                            let a192 = f55(v0, 10000000000u64);
                                            let a193 = {
                                                let a190 = f55(v0, 1000000000u64);
                                                let a191 = {
                                                    let a188 = f55(v0, 100000000u64);
                                                    let a189 = {
                                                        let a186 = f55(v0, 10000000u64);
                                                        let a187 = {
                                                            let a184 = f55(v0, 1000000u64);
                                                            let a185 = {
                                                                let a182 = f55(v0, 100000u64);
                                                                let a183 = {
                                                                    let a180 = f55(v0, 10000u64);
                                                                    let a181 = {
                                                                        let a178 = f55(v0, 1000u64);
                                                                        let a179 = {
                                                                            let a176 = f55(v0, 100u64);
                                                                            let a177 = {
                                                                                let a174 = f55(v0, 10u64);
                                                                                let a175 = f55(v0, 1u64);
                                                                                append_bytes(a174, a175)
                                                                            };
                                                                            append_bytes(a176, a177)
                                                                        };
                                                                        append_bytes(a178, a179)
                                                                    };
                                                                    append_bytes(a180, a181)
                                                                };
                                                                append_bytes(a182, a183)
                                                            };
                                                            append_bytes(a184, a185)
                                                        };
                                                        append_bytes(a186, a187)
                                                    };
                                                    append_bytes(a188, a189)
                                                };
                                                append_bytes(a190, a191)
                                            };
                                            append_bytes(a192, a193)
                                        };
                                        append_bytes(a194, a195)
                                    };
                                    append_bytes(a196, a197)
                                };
                                append_bytes(a198, a199)
                            };
                            append_bytes(a200, a201)
                        };
                        append_bytes(a202, a203)
                    };
                    append_bytes(a204, a205)
                };
                append_bytes(a206, a207)
            };
            append_bytes(a208, a209)
        };
        append_bytes(a210, a211)
    };
    let v2: u64 = {
        let (h212, _) = f56(Fn8::F57(v1.clone()), {
            let a213 = v1.clone();
            length_bytes(a213)
        }, 0u64)?;
        h212
    };
    let a214 = v0;
    let a215 = 0u64;
    if nat_eq(a214, a215) {
        let a216 = Str::lit("0");
        Ok(utf8_encode(a216))
    } else {
        Ok(f58(v1.clone(), v2, {
            let a218 = {
                let a217 = v1;
                length_bytes(a217)
            };
            let a219 = v2;
            nat_sub(a218, a219)
        }))
    }
}

pub fn f47(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m220 = v0.clone();
    match m220.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m221 = v2;
            match m221.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f59(Fn9::F60, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m223 = {
                        let (h222, _) = v5;
                        h222
                    };
                    match m223 {
                        None => {
                            Some(f23({
                                let (_, h225) = v5;
                                h225
                            }))
                        }
                        Some(v6) => {
                            Some(f23(List::cons(v6, {
                                let (_, h224) = v5;
                                h224
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> Option<u8> {
    let a226 = v0;
    let a227 = v1;
    index_bytes(a226, a227)
}

pub fn f49(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a272 = {
            let a248 = {
                let a236 = {
                    let a230 = {
                        let a228 = v1;
                        let a229 = 128u8;
                        fixed_u8::bit_and(a228, a229)
                    };
                    let a231 = 128u8;
                    if equal(a230, a231) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a237 = {
                    let a234 = {
                        let a232 = v1;
                        let a233 = 64u8;
                        fixed_u8::bit_and(a232, a233)
                    };
                    let a235 = 64u8;
                    if equal(a234, a235) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a236, a237)?
            };
            let a249 = {
                let a246 = {
                    let a240 = {
                        let a238 = v1;
                        let a239 = 32u8;
                        fixed_u8::bit_and(a238, a239)
                    };
                    let a241 = 32u8;
                    if equal(a240, a241) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a247 = {
                    let a244 = {
                        let a242 = v1;
                        let a243 = 16u8;
                        fixed_u8::bit_and(a242, a243)
                    };
                    let a245 = 16u8;
                    if equal(a244, a245) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a246, a247)?
            };
            nat_add(a248, a249)?
        };
        let a273 = {
            let a270 = {
                let a258 = {
                    let a252 = {
                        let a250 = v1;
                        let a251 = 8u8;
                        fixed_u8::bit_and(a250, a251)
                    };
                    let a253 = 8u8;
                    if equal(a252, a253) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a259 = {
                    let a256 = {
                        let a254 = v1;
                        let a255 = 4u8;
                        fixed_u8::bit_and(a254, a255)
                    };
                    let a257 = 4u8;
                    if equal(a256, a257) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a258, a259)?
            };
            let a271 = {
                let a268 = {
                    let a262 = {
                        let a260 = v1;
                        let a261 = 2u8;
                        fixed_u8::bit_and(a260, a261)
                    };
                    let a263 = 2u8;
                    if equal(a262, a263) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a269 = {
                    let a266 = {
                        let a264 = v1;
                        let a265 = 1u8;
                        fixed_u8::bit_and(a264, a265)
                    };
                    let a267 = 1u8;
                    if equal(a266, a267) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a268, a269)?
            };
            nat_add(a270, a271)?
        };
        nat_add(a272, a273)?
    };
    let a274 = v2;
    let a275 = 34u64;
    if nat_eq(a274, a275) {
        Ok(f17(Str::lit("\\\"")))
    } else {
        let a276 = v2;
        let a277 = 92u64;
        if nat_eq(a276, a277) {
            Ok(f17(Str::lit("\\\\")))
        } else {
            let a278 = v2;
            let a279 = 8u64;
            if nat_eq(a278, a279) {
                Ok(f17(Str::lit("\\b")))
            } else {
                let a280 = v2;
                let a281 = 12u64;
                if nat_eq(a280, a281) {
                    Ok(f17(Str::lit("\\f")))
                } else {
                    let a282 = v2;
                    let a283 = 10u64;
                    if nat_eq(a282, a283) {
                        Ok(f17(Str::lit("\\n")))
                    } else {
                        let a284 = v2;
                        let a285 = 13u64;
                        if nat_eq(a284, a285) {
                            Ok(f17(Str::lit("\\r")))
                        } else {
                            let a286 = v2;
                            let a287 = 9u64;
                            if nat_eq(a286, a287) {
                                Ok(f17(Str::lit("\\t")))
                            } else {
                                let a288 = v2;
                                let a289 = 32u64;
                                if nat_lt(a288, a289) {
                                    Ok(f3(f17(Str::lit("\\u00")), f61(v2)))
                                } else {
                                    Ok(f58(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v2, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt5>)> {
    f62(Fn10::F63(v0, v1), v2, (0u64, List::<Adt5>::nil()))
}

pub fn f51(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt5> {
    let v4: u64 = {
        let a290 = 1024u64;
        let a291 = v2;
        nat_mul(a290, a291)?
    };
    let v5: u64 = f64(v1, v4);
    let v6: u64 = f65(v5)?;
    let v7: u64 = {
        let a292 = v6;
        let a293 = 1u64;
        nat_sub(a292, a293)
    };
    Ok(f66(f67(v0.clone(), v4, v2, v7)?, f68(v0, {
        let a296 = v4;
        let a297 = {
            let a294 = 64u64;
            let a295 = v7;
            nat_mul(a294, a295)?
        };
        nat_add(a296, a297)?
    })?, v2, f69({
        let a300 = v5;
        let a301 = {
            let a298 = 64u64;
            let a299 = v7;
            nat_mul(a298, a299)?
        };
        nat_sub(a300, a301)
    }), {
        let a304 = {
            let a302 = v7;
            let a303 = 0u64;
            if nat_eq(a302, a303) {
                3u32
            } else {
                2u32
            }
        };
        let a305 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a304, a305)
    }))
}

pub fn f52(v0: Fn7, p309: u64, p310: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m306 = v1;
                if m306 == 0 {
                    Err::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>(v2)
                } else {
                    let v3 = m306 - 1;
                    Ok::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>((v3, {
                        let c307: Fn7 = v0.clone();
                        c307.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    f70(v0)
}

pub fn f54(v0: u32, v1: u32) -> Bytes {
    f71(f72(v0, v1))
}

pub fn f55(v0: u64, v1: u64) -> Bytes {
    f58(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a314 = {
            let a311 = v0;
            let a312 = v1;
            let a313 = 0u64;
            nat_quot(a311, a312, a313)
        };
        let a315 = 10u64;
        let a316 = 0u64;
        nat_rem(a314, a315, a316)
    }, 1u64)
}

pub fn f56(v0: Fn8, p321: u64, p322: u64) -> R<(u64, bool)> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m317 = v1;
                if m317 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m317 - 1;
                    let m319 = {
                        let c318: Fn8 = v0.clone();
                        c318.apply(v2)?
                    };
                    match m319 {
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
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m326 = {
        let a323 = v0;
        let a324 = v1;
        let a325 = v2;
        slice_bytes(a323, a324, a325)
    };
    match m326 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f59(v0: Fn9, p330: (Option<Bytes>, List<Bytes>), p331: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m327 = v2;
                match m327.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c328: Fn9 = v0.clone();
                            c328.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f74(v0, v1)
}

pub fn f61(v0: u64) -> Bytes {
    f3(f58(f17(Str::lit("0123456789abcdef")), {
        let a332 = v0;
        let a333 = 16u64;
        let a334 = 0u64;
        nat_quot(a332, a333, a334)
    }, 1u64), f58(f17(Str::lit("0123456789abcdef")), {
        let a335 = v0;
        let a336 = 16u64;
        let a337 = 0u64;
        nat_rem(a335, a336, a337)
    }, 1u64))
}

pub fn f62(v0: Fn10, p341: u64, p342: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v1, v2) = m340;
                let m338 = v1;
                if m338 == 0 {
                    Err::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>(Ok(v2))
                } else {
                    let v3 = m338 - 1;
                    Ok::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>((v3, {
                        let c339: Fn10 = v0.clone();
                        c339.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Bytes, v1: u64, v2: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    f75(v2.clone(), f51(v0, v1, {
        let (h343, _) = v2;
        h343
    }, false)?)
}

pub fn f64(v0: u64, v1: u64) -> u64 {
    let a346 = 1024u64;
    let a347 = {
        let a344 = v0;
        let a345 = v1;
        nat_sub(a344, a345)
    };
    if nat_lt(a346, a347) {
        1024u64
    } else {
        let a348 = v0;
        let a349 = v1;
        nat_sub(a348, a349)
    }
}

pub fn f65(v0: u64) -> R<u64> {
    let a350 = v0;
    let a351 = 0u64;
    if nat_eq(a350, a351) {
        Ok(1u64)
    } else {
        let a354 = {
            let a352 = v0;
            let a353 = 63u64;
            nat_add(a352, a353)?
        };
        let a355 = 64u64;
        let a356 = 0u64;
        Ok(nat_quot(a354, a355, a356))
    }
}

pub fn f66(v0: Adt5, v1: Adt6, v2: u64, v3: u32, v4: u32) -> Adt5 {
    let v5: Adt7 = {
        let (h357, _) = f76(Fn11::F77, 7u64, (Adt7::C0(match v0.clone() {
            Adt5::C0(h358, _, _, _, _, _, _, _) => {
                h358
            }
        }, match v0.clone() {
            Adt5::C0(_, h359, _, _, _, _, _, _) => {
                h359
            }
        }, match v0.clone() {
            Adt5::C0(_, _, h360, _, _, _, _, _) => {
                h360
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, h361, _, _, _, _) => {
                h361
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, h362, _, _, _) => {
                h362
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, h363, _, _) => {
                h363
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, h364, _) => {
                h364
            }
        }, match v0 {
            Adt5::C0(_, _, _, _, _, _, _, h365) => {
                h365
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f69({
            let a366 = v2;
            let a367 = 4294967296u64;
            let a368 = 0u64;
            nat_rem(a366, a367, a368)
        }), f69({
            let a369 = v2;
            let a370 = 4294967296u64;
            let a371 = 0u64;
            nat_quot(a369, a370, a371)
        }), v3, v4), v1));
        h357
    };
    Adt5::C0({
        let a374 = match v5.clone() {
            Adt7::C0(h372, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        };
        let a375 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h373, _, _, _, _, _, _, _) => {
                h373
            }
        };
        fixed_u32::bit_xor(a374, a375)
    }, {
        let a378 = match v5.clone() {
            Adt7::C0(_, h376, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        let a379 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h377, _, _, _, _, _, _) => {
                h377
            }
        };
        fixed_u32::bit_xor(a378, a379)
    }, {
        let a382 = match v5.clone() {
            Adt7::C0(_, _, h380, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h380
            }
        };
        let a383 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h381, _, _, _, _, _) => {
                h381
            }
        };
        fixed_u32::bit_xor(a382, a383)
    }, {
        let a386 = match v5.clone() {
            Adt7::C0(_, _, _, h384, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h384
            }
        };
        let a387 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h385, _, _, _, _) => {
                h385
            }
        };
        fixed_u32::bit_xor(a386, a387)
    }, {
        let a390 = match v5.clone() {
            Adt7::C0(_, _, _, _, h388, _, _, _, _, _, _, _, _, _, _, _) => {
                h388
            }
        };
        let a391 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h389, _, _, _) => {
                h389
            }
        };
        fixed_u32::bit_xor(a390, a391)
    }, {
        let a394 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, h392, _, _, _, _, _, _, _, _, _, _) => {
                h392
            }
        };
        let a395 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h393, _, _) => {
                h393
            }
        };
        fixed_u32::bit_xor(a394, a395)
    }, {
        let a398 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, h396, _, _, _, _, _, _, _, _, _) => {
                h396
            }
        };
        let a399 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h397, _) => {
                h397
            }
        };
        fixed_u32::bit_xor(a398, a399)
    }, {
        let a402 = match v5.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h400, _, _, _, _, _, _, _, _) => {
                h400
            }
        };
        let a403 = match v5 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h401) => {
                h401
            }
        };
        fixed_u32::bit_xor(a402, a403)
    })
}

pub fn f67(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let (_, h404) = f78(Fn12::F79(v2, v0, v1), v3, (0u64, f80()))?;
    Ok(h404)
}

pub fn f68(v0: Bytes, v1: u64) -> R<Adt6> {
    Ok(Adt6::C0(f81(v0.clone(), {
        let a405 = v1;
        let a406 = 0u64;
        nat_add(a405, a406)?
    })?, f81(v0.clone(), {
        let a407 = v1;
        let a408 = 4u64;
        nat_add(a407, a408)?
    })?, f81(v0.clone(), {
        let a409 = v1;
        let a410 = 8u64;
        nat_add(a409, a410)?
    })?, f81(v0.clone(), {
        let a411 = v1;
        let a412 = 12u64;
        nat_add(a411, a412)?
    })?, f81(v0.clone(), {
        let a413 = v1;
        let a414 = 16u64;
        nat_add(a413, a414)?
    })?, f81(v0.clone(), {
        let a415 = v1;
        let a416 = 20u64;
        nat_add(a415, a416)?
    })?, f81(v0.clone(), {
        let a417 = v1;
        let a418 = 24u64;
        nat_add(a417, a418)?
    })?, f81(v0.clone(), {
        let a419 = v1;
        let a420 = 28u64;
        nat_add(a419, a420)?
    })?, f81(v0.clone(), {
        let a421 = v1;
        let a422 = 32u64;
        nat_add(a421, a422)?
    })?, f81(v0.clone(), {
        let a423 = v1;
        let a424 = 36u64;
        nat_add(a423, a424)?
    })?, f81(v0.clone(), {
        let a425 = v1;
        let a426 = 40u64;
        nat_add(a425, a426)?
    })?, f81(v0.clone(), {
        let a427 = v1;
        let a428 = 44u64;
        nat_add(a427, a428)?
    })?, f81(v0.clone(), {
        let a429 = v1;
        let a430 = 48u64;
        nat_add(a429, a430)?
    })?, f81(v0.clone(), {
        let a431 = v1;
        let a432 = 52u64;
        nat_add(a431, a432)?
    })?, f81(v0.clone(), {
        let a433 = v1;
        let a434 = 56u64;
        nat_add(a433, a434)?
    })?, f81(v0, {
        let a435 = v1;
        let a436 = 60u64;
        nat_add(a435, a436)?
    })?))
}

pub fn f69(v0: u64) -> u32 {
    let (_, h437) = {
        let (_, h438) = f82(Fn13::F83, 32u64, (v0, (1u32, 0u32)));
        h438
    };
    h437
}

pub fn f70(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m440 = {
        let (h439, _) = v0.clone();
        h439
    };
    match m440.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f84(v1, {
                let (_, h441) = v0;
                h441
            }, {
                let m442 = v2;
                match m442.uncons() {
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

pub fn f71(v0: u32) -> Bytes {
    let a445 = {
        let a443 = v0;
        let a444 = 8u32;
        fixed_u32::bit_and(a443, a444)
    };
    let a446 = 0u32;
    if equal(a445, a446) {
        let a449 = {
            let a447 = v0;
            let a448 = 4u32;
            fixed_u32::bit_and(a447, a448)
        };
        let a450 = 0u32;
        if equal(a449, a450) {
            let a453 = {
                let a451 = v0;
                let a452 = 2u32;
                fixed_u32::bit_and(a451, a452)
            };
            let a454 = 0u32;
            if equal(a453, a454) {
                let a457 = {
                    let a455 = v0;
                    let a456 = 1u32;
                    fixed_u32::bit_and(a455, a456)
                };
                let a458 = 0u32;
                if equal(a457, a458) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a461 = {
                    let a459 = v0;
                    let a460 = 1u32;
                    fixed_u32::bit_and(a459, a460)
                };
                let a462 = 0u32;
                if equal(a461, a462) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a465 = {
                let a463 = v0;
                let a464 = 2u32;
                fixed_u32::bit_and(a463, a464)
            };
            let a466 = 0u32;
            if equal(a465, a466) {
                let a469 = {
                    let a467 = v0;
                    let a468 = 1u32;
                    fixed_u32::bit_and(a467, a468)
                };
                let a470 = 0u32;
                if equal(a469, a470) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a473 = {
                    let a471 = v0;
                    let a472 = 1u32;
                    fixed_u32::bit_and(a471, a472)
                };
                let a474 = 0u32;
                if equal(a473, a474) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a477 = {
            let a475 = v0;
            let a476 = 4u32;
            fixed_u32::bit_and(a475, a476)
        };
        let a478 = 0u32;
        if equal(a477, a478) {
            let a481 = {
                let a479 = v0;
                let a480 = 2u32;
                fixed_u32::bit_and(a479, a480)
            };
            let a482 = 0u32;
            if equal(a481, a482) {
                let a485 = {
                    let a483 = v0;
                    let a484 = 1u32;
                    fixed_u32::bit_and(a483, a484)
                };
                let a486 = 0u32;
                if equal(a485, a486) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a489 = {
                    let a487 = v0;
                    let a488 = 1u32;
                    fixed_u32::bit_and(a487, a488)
                };
                let a490 = 0u32;
                if equal(a489, a490) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a493 = {
                let a491 = v0;
                let a492 = 2u32;
                fixed_u32::bit_and(a491, a492)
            };
            let a494 = 0u32;
            if equal(a493, a494) {
                let a497 = {
                    let a495 = v0;
                    let a496 = 1u32;
                    fixed_u32::bit_and(a495, a496)
                };
                let a498 = 0u32;
                if equal(a497, a498) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a501 = {
                    let a499 = v0;
                    let a500 = 1u32;
                    fixed_u32::bit_and(a499, a500)
                };
                let a502 = 0u32;
                if equal(a501, a502) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f72(v0: u32, v1: u32) -> u32 {
    let m505 = {
        let a503 = v0;
        let a504 = v1;
        fixed_u32::shift_right(a503, a504)
    };
    match m505 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m506 = f48(v0, v1);
    match m506 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a507 = v2;
            let a508 = 48u8;
            if equal(a507, a508) {
                Ok(Some({
                    let a509 = v1;
                    let a510 = 1u64;
                    nat_add(a509, a510)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f74(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m512 = {
        let (h511, _) = v0;
        h511
    };
    match m512 {
        None => {
            (Some(v1), {
                let (_, h513) = v0;
                h513
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a514 = v2;
                let a515 = v1;
                append_bytes(a514, a515)
            }, {
                let (_, h516) = v0;
                h516
            }))
        }
    }
}

pub fn f75(v0: (u64, List<Adt5>), v1: Adt5) -> R<(u64, List<Adt5>)> {
    let v2: Adt8 = f85(Fn14::F86, 40u64, Adt8::C0(v1, {
        let a518 = {
            let (h517, _) = v0.clone();
            h517
        };
        let a519 = 1u64;
        nat_add(a518, a519)?
    }, {
        let (_, h520) = v0;
        h520
    }));
    Ok(({
        let a522 = {
            let (h521, _) = v0;
            h521
        };
        let a523 = 1u64;
        nat_add(a522, a523)?
    }, List::cons(match v2.clone() {
        Adt8::C0(h524, _, _) => {
            h524
        }
    }, match v2 {
        Adt8::C0(_, _, h525) => {
            h525
        }
    })))
}

pub fn f76(v0: Fn11, p529: u64, p530: (Adt7, Adt6)) -> (Adt7, Adt6) {
    let m528 = (p529, p530);
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v1, v2) = m528;
                let m526 = v1;
                if m526 == 0 {
                    Err::<(u64, (Adt7, Adt6)), (Adt7, Adt6)>(v2)
                } else {
                    let v3 = m526 - 1;
                    Ok::<(u64, (Adt7, Adt6)), (Adt7, Adt6)>((v3, {
                        let c527: Fn11 = v0.clone();
                        c527.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: (Adt7, Adt6)) -> (Adt7, Adt6) {
    f87(v0)
}

pub fn f78(v0: Fn12, p534: u64, p535: (u64, Adt5)) -> R<(u64, Adt5)> {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v1, v2) = m533;
                let m531 = v1;
                if m531 == 0 {
                    Err::<(u64, (u64, Adt5)), R<(u64, Adt5)>>(Ok(v2))
                } else {
                    let v3 = m531 - 1;
                    Ok::<(u64, (u64, Adt5)), R<(u64, Adt5)>>((v3, {
                        let c532: Fn12 = v0.clone();
                        c532.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt5)) -> R<(u64, Adt5)> {
    Ok(({
        let a537 = {
            let (h536, _) = v3.clone();
            h536
        };
        let a538 = 1u64;
        nat_add(a537, a538)?
    }, f66({
        let (_, h539) = v3.clone();
        h539
    }, f68(v1, {
        let a543 = v2;
        let a544 = {
            let a541 = 64u64;
            let a542 = {
                let (h540, _) = v3.clone();
                h540
            };
            nat_mul(a541, a542)?
        };
        nat_add(a543, a544)?
    })?, v0, 64u32, {
        let a546 = {
            let (h545, _) = v3;
            h545
        };
        let a547 = 0u64;
        if nat_eq(a546, a547) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f80() -> Adt5 {
    Adt5::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f81(v0: Bytes, v1: u64) -> R<u32> {
    let a558 = f88(v0.clone(), v1);
    let a559 = {
        let a556 = f89(f88(v0.clone(), {
            let a548 = v1;
            let a549 = 1u64;
            nat_add(a548, a549)?
        }), 8u32);
        let a557 = {
            let a554 = f89(f88(v0.clone(), {
                let a550 = v1;
                let a551 = 2u64;
                nat_add(a550, a551)?
            }), 16u32);
            let a555 = f89(f88(v0, {
                let a552 = v1;
                let a553 = 3u64;
                nat_add(a552, a553)?
            }), 24u32);
            fixed_u32::bit_or(a554, a555)
        };
        fixed_u32::bit_or(a556, a557)
    };
    Ok(fixed_u32::bit_or(a558, a559))
}

pub fn f82(v0: Fn13, p563: u64, p564: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m562 = (p563, p564);
    {
        let mut m562 = m562;
        loop {
            let step = {
                let (v1, v2) = m562;
                let m560 = v1;
                if m560 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m560 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c561: Fn13 = v0.clone();
                        c561.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h565, _) = v0;
        h565
    };
    let v2: u32 = {
        let (h566, _) = {
            let (_, h567) = v0;
            h567
        };
        h566
    };
    let v3: u32 = {
        let (_, h568) = {
            let (_, h569) = v0;
            h569
        };
        h568
    };
    ({
        let a570 = v1;
        let a571 = 2u64;
        let a572 = 0u64;
        nat_quot(a570, a571, a572)
    }, (f89(v2, 1u32), {
        let a576 = {
            let a573 = v1;
            let a574 = 2u64;
            let a575 = 0u64;
            nat_rem(a573, a574, a575)
        };
        let a577 = 1u64;
        if nat_eq(a576, a577) {
            let a578 = v3;
            let a579 = v2;
            fixed_u32::bit_or(a578, a579)
        } else {
            v3
        }
    }))
}

pub fn f84(v0: Adt5, v1: Adt5, v2: bool) -> Adt5 {
    f66(f80(), f90(v0, v1), 0u64, 64u32, {
        let a580 = 4u32;
        let a581 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a580, a581)
    })
}

pub fn f85(v0: Fn14, p585: u64, p586: Adt8) -> Adt8 {
    let m584 = (p585, p586);
    {
        let mut m584 = m584;
        loop {
            let step = {
                let (v1, v2) = m584;
                let m582 = v1;
                if m582 == 0 {
                    Err::<(u64, Adt8), Adt8>(v2)
                } else {
                    let v3 = m582 - 1;
                    Ok::<(u64, Adt8), Adt8>((v3, {
                        let c583: Fn14 = v0.clone();
                        c583.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Adt8) -> Adt8 {
    f91(v0)
}

pub fn f87(v0: (Adt7, Adt6)) -> (Adt7, Adt6) {
    (f92(f93({
        let (h587, _) = v0.clone();
        h587
    }, {
        let (_, h588) = v0.clone();
        h588
    }), {
        let (_, h589) = v0.clone();
        h589
    }), f94({
        let (_, h590) = v0;
        h590
    }))
}

pub fn f88(v0: Bytes, v1: u64) -> u32 {
    let m593 = {
        let a591 = v0;
        let a592 = v1;
        index_bytes(a591, a592)
    };
    match m593 {
        None => {
            0u32
        }
        Some(v2) => {
            let m595 = {
                let a594 = v2;
                fixed_u32::convert(i128::from(a594))
            };
            match m595 {
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

pub fn f89(v0: u32, v1: u32) -> u32 {
    let m598 = {
        let a596 = v0;
        let a597 = v1;
        fixed_u32::shift_left(a596, a597)
    };
    match m598 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f90(v0: Adt5, v1: Adt5) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt5::C0(h599, _, _, _, _, _, _, _) => {
            h599
        }
    }, match v0.clone() {
        Adt5::C0(_, h600, _, _, _, _, _, _) => {
            h600
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h601, _, _, _, _, _) => {
            h601
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, h602, _, _, _, _) => {
            h602
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h603, _, _, _) => {
            h603
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h604, _, _) => {
            h604
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h605, _) => {
            h605
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, h606) => {
            h606
        }
    }, match v1.clone() {
        Adt5::C0(h607, _, _, _, _, _, _, _) => {
            h607
        }
    }, match v1.clone() {
        Adt5::C0(_, h608, _, _, _, _, _, _) => {
            h608
        }
    }, match v1.clone() {
        Adt5::C0(_, _, h609, _, _, _, _, _) => {
            h609
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, h610, _, _, _, _) => {
            h610
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, h611, _, _, _) => {
            h611
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, h612, _, _) => {
            h612
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, _, h613, _) => {
            h613
        }
    }, match v1 {
        Adt5::C0(_, _, _, _, _, _, _, h614) => {
            h614
        }
    })
}

pub fn f91(v0: Adt8) -> Adt8 {
    let a619 = {
        let a616 = match v0.clone() {
            Adt8::C0(_, h615, _) => {
                h615
            }
        };
        let a617 = 2u64;
        let a618 = 0u64;
        nat_rem(a616, a617, a618)
    };
    let a620 = 0u64;
    if nat_eq(a619, a620) {
        let m622 = match v0.clone() {
            Adt8::C0(_, _, h621) => {
                h621
            }
        };
        match m622.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt8::C0(f84(v1, match v0.clone() {
                    Adt8::C0(h623, _, _) => {
                        h623
                    }
                }, false), {
                    let a625 = match v0 {
                        Adt8::C0(_, h624, _) => {
                            h624
                        }
                    };
                    let a626 = 2u64;
                    let a627 = 0u64;
                    nat_quot(a625, a626, a627)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f92(v0: Adt7, v1: Adt6) -> Adt7 {
    let v2: Adt9 = f95(match v0.clone() {
        Adt7::C0(h628, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h628
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h629, _, _, _, _, _, _, _, _, _, _) => {
            h629
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h630, _, _, _, _, _) => {
            h630
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h631) => {
            h631
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h632, _, _, _, _, _, _, _) => {
            h632
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h633, _, _, _, _, _, _) => {
            h633
        }
    });
    let v3: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, h634, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h634
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h635, _, _, _, _, _, _, _, _, _) => {
            h635
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h636, _, _, _, _) => {
            h636
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h637, _, _, _) => {
            h637
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h638, _, _, _, _, _) => {
            h638
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h639, _, _, _, _) => {
            h639
        }
    });
    let v4: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, _, h640, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h640
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h641, _, _, _, _, _, _, _, _) => {
            h641
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h642, _, _, _, _, _, _, _) => {
            h642
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h643, _, _) => {
            h643
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h644, _, _, _) => {
            h644
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h645, _, _) => {
            h645
        }
    });
    let v5: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, _, _, h646, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h646
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h647, _, _, _, _, _, _, _, _, _, _, _) => {
            h647
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h648, _, _, _, _, _, _) => {
            h648
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h649, _) => {
            h649
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h650, _) => {
            h650
        }
    }, match v1 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h651) => {
            h651
        }
    });
    Adt7::C0(match v2.clone() {
        Adt9::C0(h652, _, _, _) => {
            h652
        }
    }, match v3.clone() {
        Adt9::C0(h653, _, _, _) => {
            h653
        }
    }, match v4.clone() {
        Adt9::C0(h654, _, _, _) => {
            h654
        }
    }, match v5.clone() {
        Adt9::C0(h655, _, _, _) => {
            h655
        }
    }, match v5.clone() {
        Adt9::C0(_, h656, _, _) => {
            h656
        }
    }, match v2.clone() {
        Adt9::C0(_, h657, _, _) => {
            h657
        }
    }, match v3.clone() {
        Adt9::C0(_, h658, _, _) => {
            h658
        }
    }, match v4.clone() {
        Adt9::C0(_, h659, _, _) => {
            h659
        }
    }, match v4.clone() {
        Adt9::C0(_, _, h660, _) => {
            h660
        }
    }, match v5.clone() {
        Adt9::C0(_, _, h661, _) => {
            h661
        }
    }, match v2.clone() {
        Adt9::C0(_, _, h662, _) => {
            h662
        }
    }, match v3.clone() {
        Adt9::C0(_, _, h663, _) => {
            h663
        }
    }, match v3 {
        Adt9::C0(_, _, _, h664) => {
            h664
        }
    }, match v4 {
        Adt9::C0(_, _, _, h665) => {
            h665
        }
    }, match v5 {
        Adt9::C0(_, _, _, h666) => {
            h666
        }
    }, match v2 {
        Adt9::C0(_, _, _, h667) => {
            h667
        }
    })
}

pub fn f93(v0: Adt7, v1: Adt6) -> Adt7 {
    let v2: Adt9 = f95(match v0.clone() {
        Adt7::C0(h668, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h668
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h669, _, _, _, _, _, _, _, _, _, _, _) => {
            h669
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h670, _, _, _, _, _, _, _) => {
            h670
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h671, _, _, _) => {
            h671
        }
    }, match v1.clone() {
        Adt6::C0(h672, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h672
        }
    }, match v1.clone() {
        Adt6::C0(_, h673, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h673
        }
    });
    let v3: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, h674, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h674
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h675, _, _, _, _, _, _, _, _, _, _) => {
            h675
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h676, _, _, _, _, _, _) => {
            h676
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h677, _, _) => {
            h677
        }
    }, match v1.clone() {
        Adt6::C0(_, _, h678, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h678
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, h679, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h679
        }
    });
    let v4: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, _, h680, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h680
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h681, _, _, _, _, _, _, _, _, _) => {
            h681
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h682, _, _, _, _, _) => {
            h682
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h683, _) => {
            h683
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, h684, _, _, _, _, _, _, _, _, _, _, _) => {
            h684
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, h685, _, _, _, _, _, _, _, _, _, _) => {
            h685
        }
    });
    let v5: Adt9 = f95(match v0.clone() {
        Adt7::C0(_, _, _, h686, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h686
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h687, _, _, _, _, _, _, _, _) => {
            h687
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h688, _, _, _, _) => {
            h688
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h689) => {
            h689
        }
    }, match v1.clone() {
        Adt6::C0(_, _, _, _, _, _, h690, _, _, _, _, _, _, _, _, _) => {
            h690
        }
    }, match v1 {
        Adt6::C0(_, _, _, _, _, _, _, h691, _, _, _, _, _, _, _, _) => {
            h691
        }
    });
    Adt7::C0(match v2.clone() {
        Adt9::C0(h692, _, _, _) => {
            h692
        }
    }, match v3.clone() {
        Adt9::C0(h693, _, _, _) => {
            h693
        }
    }, match v4.clone() {
        Adt9::C0(h694, _, _, _) => {
            h694
        }
    }, match v5.clone() {
        Adt9::C0(h695, _, _, _) => {
            h695
        }
    }, match v2.clone() {
        Adt9::C0(_, h696, _, _) => {
            h696
        }
    }, match v3.clone() {
        Adt9::C0(_, h697, _, _) => {
            h697
        }
    }, match v4.clone() {
        Adt9::C0(_, h698, _, _) => {
            h698
        }
    }, match v5.clone() {
        Adt9::C0(_, h699, _, _) => {
            h699
        }
    }, match v2.clone() {
        Adt9::C0(_, _, h700, _) => {
            h700
        }
    }, match v3.clone() {
        Adt9::C0(_, _, h701, _) => {
            h701
        }
    }, match v4.clone() {
        Adt9::C0(_, _, h702, _) => {
            h702
        }
    }, match v5.clone() {
        Adt9::C0(_, _, h703, _) => {
            h703
        }
    }, match v2 {
        Adt9::C0(_, _, _, h704) => {
            h704
        }
    }, match v3 {
        Adt9::C0(_, _, _, h705) => {
            h705
        }
    }, match v4 {
        Adt9::C0(_, _, _, h706) => {
            h706
        }
    }, match v5 {
        Adt9::C0(_, _, _, h707) => {
            h707
        }
    })
}

pub fn f94(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, _, h708, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h708
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h709, _, _, _, _, _, _, _, _, _) => {
            h709
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h710, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h710
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h711, _, _, _, _, _) => {
            h711
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h712, _, _, _, _, _, _, _, _) => {
            h712
        }
    }, match v0.clone() {
        Adt6::C0(h713, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h713
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h714, _, _, _, _, _, _, _, _, _, _, _) => {
            h714
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h715, _, _) => {
            h715
        }
    }, match v0.clone() {
        Adt6::C0(_, h716, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h716
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h717, _, _, _, _) => {
            h717
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h718, _, _, _) => {
            h718
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h719, _, _, _, _, _, _, _, _, _, _) => {
            h719
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h720, _, _, _, _, _, _) => {
            h720
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h721, _) => {
            h721
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h722) => {
            h722
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h723, _, _, _, _, _, _, _) => {
            h723
        }
    })
}

pub fn f95(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt9 {
    let v6: u32 = f96(f96(v0, v1), v4);
    let v7: u32 = f97({
        let a724 = v3;
        let a725 = v6;
        fixed_u32::bit_xor(a724, a725)
    }, 16u32, 16u32);
    let v8: u32 = f96(v2, v7);
    let v9: u32 = f97({
        let a726 = v1;
        let a727 = v8;
        fixed_u32::bit_xor(a726, a727)
    }, 12u32, 20u32);
    let v10: u32 = f96(f96(v6, v9), v5);
    let v11: u32 = f97({
        let a728 = v7;
        let a729 = v10;
        fixed_u32::bit_xor(a728, a729)
    }, 8u32, 24u32);
    let v12: u32 = f96(v8, v11);
    let v13: u32 = f97({
        let a730 = v9;
        let a731 = v12;
        fixed_u32::bit_xor(a730, a731)
    }, 7u32, 25u32);
    Adt9::C0(v10, v13, v12, v11)
}

pub fn f96(v0: u32, v1: u32) -> u32 {
    let m734 = {
        let a732 = f98(v0);
        let a733 = f98(v1);
        fixed_u64::checked_add(a732, a733)
    };
    match m734 {
        None => {
            0u32
        }
        Some(v2) => {
            f99(v2)
        }
    }
}

pub fn f97(v0: u32, v1: u32, v2: u32) -> u32 {
    let a735 = f72(v0, v1);
    let a736 = f89(v0, v2);
    fixed_u32::bit_or(a735, a736)
}

pub fn f98(v0: u32) -> u64 {
    let m738 = {
        let a737 = v0;
        fixed_u64::convert(i128::from(a737))
    };
    match m738 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f99(v0: u64) -> u32 {
    let m742 = {
        let a741 = {
            let a739 = v0;
            let a740 = 4294967295u64;
            fixed_u64::bit_and(a739, a740)
        };
        fixed_u32::convert(i128::from(a741))
    };
    match m742 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}
