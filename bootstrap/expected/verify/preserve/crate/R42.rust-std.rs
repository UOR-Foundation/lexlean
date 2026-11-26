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
    C0(Option<Bytes>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt9>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<Adt9>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(std::rc::Rc<Adt0>, List<(Bytes, Bytes)>, bool, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Fn0 {
    F12,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> Option<Adt4> {
        match self {
            Fn0::F12 => f12(p0),
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
    F67,
    F109,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn3::F67 => f67(p0, p1),
            Fn3::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F75,
    F108(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn4::F75 => f75(p0, p1),
            Fn4::F108(k0) => Ok(f108(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F86(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn5::F86(k0, k1) => f86(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F88,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt0, p1: Adt9) -> Adt0 {
        match self {
            Fn6::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F94,
    F95,
    F106,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn7::F94 => f94(p0, p1),
            Fn7::F95 => f95(p0, p1),
            Fn7::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F103(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn8::F103(k0) => f103(k0.clone(), p0, p1),
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

pub fn f0(v0: Adt0, v1: Option<Adt0>, v2: List<Adt2>, v3: u64, v4: u64) -> R<Result<Adt3, Bytes>> {
    let v5: Adt4 = f1(v4);
    let v6: Adt5 = f2(v5.clone(), f3(v2, v3), v0.clone(), List::<(Bytes, Adt0)>::nil())?;
    let m1 = v1;
    match m1 {
        None => {
            f4(v5, v6)
        }
        Some(v7) => {
            let m4 = match v6.clone() {
                Adt5::C0(_, _, h3) => {
                    (*h3).clone()
                }
            };
            if if match m4 {
                Adt6::C0(_, _, _, _, h2, _, _) => {
                    h2
                }
            } {
                true
            } else {
                let m6 = match v6.clone() {
                    Adt5::C0(_, h5, _) => {
                        h5
                    }
                };
                match m6 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            } {
                f4(v5, v6)
            } else {
                let v9: Adt7 = f5(v5.clone(), match v6.clone() {
                    Adt5::C0(_, _, h7) => {
                        (*h7).clone()
                    }
                }, v7.clone(), match v6.clone() {
                    Adt5::C0(h8, _, _) => {
                        (*h8).clone()
                    }
                })?;
                let m10 = match v9.clone() {
                    Adt7::C0(h9, _) => {
                        h9
                    }
                };
                match m10 {
                    None => {
                        f4(v5, f10(match v6 {
                            Adt5::C0(h12, _, _) => {
                                (*h12).clone()
                            }
                        }, match v9 {
                            Adt7::C0(_, h13) => {
                                (*h13).clone()
                            }
                        }))
                    }
                    Some(v10) => {
                        let v11: Bytes = f6(v0)?;
                        let v12: Bytes = f6(v7)?;
                        f4(v5, f7(match v9 {
                            Adt7::C0(_, h11) => {
                                (*h11).clone()
                            }
                        }, f8(f8(f8(f8(f8(f9(Str::lit("defined value ")), v11), f9(Str::lit(" does not match the signature "))), v12), f9(Str::lit(": "))), v10)))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> Adt4 {
    let (h14, _) = f11(Fn0::F12, v0, Adt4::C0);
    h14
}

pub fn f2(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m15 = v0;
    match m15 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r57) => {
            let v4 = (*r57).clone();
            let m16 = v2.clone();
            match m16 {
                Adt0::C0 => {
                    Ok(f10(Adt0::C1(std::rc::Rc::new(Adt1::C0(1u64))), v1))
                }
                Adt0::C1(r47) => {
                    let v5 = (*r47).clone();
                    Ok(f10(Adt0::C1(std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(v5)))), v1))
                }
                Adt0::C2(v6, v7, _) => {
                    let v9: (u64, bool) = f14(v1.clone(), v6.clone(), v7.clone());
                    let v10: u64 = {
                        let (h17, _) = v9;
                        h17
                    };
                    let a18 = v10;
                    let a19 = 0u64;
                    if nat_eq(a18, a19) {
                        Ok(f7(v1, f8(f8(f8(f8(f9(Str::lit("`")), v6), f9(Str::lit("::"))), v7), f9(Str::lit("` does not resolve")))))
                    } else {
                        let a20 = v10;
                        let a21 = 1u64;
                        if nat_eq(a20, a21) {
                            f15(v1)
                        } else {
                            let a22 = v10;
                            let a23 = 2u64;
                            if nat_eq(a22, a23) {
                                Ok(f7(v1, f8(f8(f8(f8(f9(Str::lit("`")), v6), f9(Str::lit("::"))), v7), f9(Str::lit("` has no signature and cannot appear in an expression")))))
                            } else {
                                Ok(f10(f16(v1.clone(), v6, v7), v1))
                            }
                        }
                    }
                }
                Adt0::C3(v11) => {
                    if f17(v11.clone())? {
                        f15(v1)
                    } else {
                        let m24 = f18(v3, v11.clone());
                        match m24 {
                            None => {
                                Ok(f7(v1, f8(f8(f9(Str::lit("local `")), v11), f9(Str::lit("` is not in scope")))))
                            }
                            Some(v12) => {
                                Ok(f10(v12, v1))
                            }
                        }
                    }
                }
                Adt0::C4(r48, r49) => {
                    let v15 = (*r48).clone();
                    let v16 = (*r49).clone();
                    let v17: u64 = f20(v15.clone());
                    let a27 = v17;
                    let a28 = 2u64;
                    if if nat_lt(a27, a28) {
                        true
                    } else {
                        let a29 = v17;
                        let a30 = 8u64;
                        nat_eq(a29, a30)
                    } {
                        let v18: Bytes = f6(v2)?;
                        let v19: Bytes = f6(v15)?;
                        Ok(f7(v1, f8(f8(f8(v18, f9(Str::lit(" applies "))), v19), f9(Str::lit(", which is not a function")))))
                    } else {
                        let v20: Adt5 = f2(v4.clone(), v1, v15.clone(), v3.clone())?;
                        let m32 = match v20.clone() {
                            Adt5::C0(_, h31, _) => {
                                h31
                            }
                        };
                        match m32 {
                            None => {
                                let v22: List<Adt0> = f21(v4.clone(), v16);
                                f22(v4, match v20.clone() {
                                    Adt5::C0(_, _, h33) => {
                                        (*h33).clone()
                                    }
                                }, v2, v15, v22.clone(), {
                                    let a34 = v22;
                                    length_list(a34)
                                }, match v20 {
                                    Adt5::C0(h35, _, _) => {
                                        (*h35).clone()
                                    }
                                }, 0u64, v3)
                            }
                            Some(_) => {
                                Ok(v20)
                            }
                        }
                    }
                }
                Adt0::C5(r50, r51) => {
                    let v23 = (*r50).clone();
                    let v24 = (*r51).clone();
                    f23(v4, v1, v23.clone(), v24, v3, false, v23)
                }
                Adt0::C6(r52, r53) => {
                    let v25 = (*r52).clone();
                    let v26 = (*r53).clone();
                    f23(v4, v1, v25.clone(), v26, v3, true, v25)
                }
                Adt0::C7(v27, r54, r55, r56) => {
                    let v28 = (*r54).clone();
                    let v29 = (*r55).clone();
                    let v30 = (*r56).clone();
                    let v31: Adt5 = f24(v4.clone(), v1, v28.clone(), v3.clone())?;
                    let m37 = match v31.clone() {
                        Adt5::C0(_, h36, _) => {
                            h36
                        }
                    };
                    match m37 {
                        None => {
                            let v33: Adt5 = f2(v4.clone(), match v31 {
                                Adt5::C0(_, _, h38) => {
                                    (*h38).clone()
                                }
                            }, v29, v3.clone())?;
                            let m40 = match v33.clone() {
                                Adt5::C0(_, h39, _) => {
                                    h39
                                }
                            };
                            match m40 {
                                None => {
                                    let v35: Adt7 = f5(v4.clone(), match v33.clone() {
                                        Adt5::C0(_, _, h41) => {
                                            (*h41).clone()
                                        }
                                    }, v28.clone(), match v33 {
                                        Adt5::C0(h42, _, _) => {
                                            (*h42).clone()
                                        }
                                    })?;
                                    let m44 = match v35.clone() {
                                        Adt7::C0(h43, _) => {
                                            h43
                                        }
                                    };
                                    match m44 {
                                        None => {
                                            f2(v4, match v35 {
                                                Adt7::C0(_, h46) => {
                                                    (*h46).clone()
                                                }
                                            }, v30, List::cons((v27, v28), v3))
                                        }
                                        Some(v36) => {
                                            Ok(f7(match v35 {
                                                Adt7::C0(_, h45) => {
                                                    (*h45).clone()
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
                    let v14: Adt8 = f19(v1)?;
                    Ok(f10(match v14.clone() {
                        Adt8::C0(h25, _) => {
                            (*h25).clone()
                        }
                    }, match v14 {
                        Adt8::C0(_, h26) => {
                            (*h26).clone()
                        }
                    }))
                }
                Adt0::C9 => {
                    Ok(f10(Adt0::C0, v1))
                }
                Adt0::C10(_, _) => {
                    Ok(f10(Adt0::C0, v1))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(f10(Adt0::C0, v1))
                }
            }
        }
    }
}

pub fn f3(v0: List<Adt2>, v1: u64) -> Adt6 {
    Adt6::C0(List::<(Bytes, Adt0)>::nil(), 0u64, List::<Bytes>::nil(), 0u64, false, v0, v1)
}

pub fn f4(v0: Adt4, v1: Adt5) -> R<Result<Adt3, Bytes>> {
    let m60 = match v1.clone() {
        Adt5::C0(_, _, h59) => {
            (*h59).clone()
        }
    };
    if match m60 {
        Adt6::C0(_, _, _, _, h58, _, _) => {
            h58
        }
    } {
        Ok(Err::<Adt3, Bytes>(f9(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        let m62 = match v1.clone() {
            Adt5::C0(_, h61, _) => {
                h61
            }
        };
        match m62 {
            None => {
                f25(v0, match v1.clone() {
                    Adt5::C0(_, _, h63) => {
                        (*h63).clone()
                    }
                }, match v1 {
                    Adt5::C0(h64, _, _) => {
                        (*h64).clone()
                    }
                })
            }
            Some(v2) => {
                Ok(Err::<Adt3, Bytes>(v2))
            }
        }
    }
}

pub fn f5(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m65 = v0;
    match m65 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r76) => {
            let v4 = (*r76).clone();
            let v5: Adt8 = f27(v4.clone(), v1, v2)?;
            let v6: Adt8 = f27(v4.clone(), match v5.clone() {
                Adt8::C0(_, h66) => {
                    (*h66).clone()
                }
            }, v3)?;
            let v7: Adt0 = match v5 {
                Adt8::C0(h67, _) => {
                    (*h67).clone()
                }
            };
            let v8: Adt0 = match v6.clone() {
                Adt8::C0(h68, _) => {
                    (*h68).clone()
                }
            };
            let v9: Adt6 = match v6 {
                Adt8::C0(_, h69) => {
                    (*h69).clone()
                }
            };
            let m70 = f28(v7.clone())?;
            match m70 {
                None => {
                    let m75 = f28(v8.clone())?;
                    match m75 {
                        None => {
                            f31(v4, v9, v7, v8)
                        }
                        Some(v11) => {
                            Ok(f26(f30(v9, v11, v7)))
                        }
                    }
                }
                Some(v10) => {
                    let a71 = f29(v8.clone());
                    let a72 = v10.clone();
                    if equal(a71, a72) {
                        let a73 = f20(v8.clone());
                        let a74 = 3u64;
                        if nat_eq(a73, a74) {
                            Ok(f26(v9))
                        } else {
                            Ok(f26(f30(v9, v10, v8)))
                        }
                    } else {
                        Ok(f26(f30(v9, v10, v8)))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Adt0) -> R<Bytes> {
    Ok(f8(f8(f9(Str::lit("`")), f32(v0, false)?), f9(Str::lit("`"))))
}

pub fn f7(v0: Adt6, v1: Bytes) -> Adt5 {
    Adt5::C0(std::rc::Rc::new(Adt0::C0), Some(v1), std::rc::Rc::new(v0))
}

pub fn f8(v0: Bytes, v1: Bytes) -> Bytes {
    let a77 = v0;
    let a78 = v1;
    append_bytes(a77, a78)
}

pub fn f9(v0: Str) -> Bytes {
    let a79 = v0;
    utf8_encode(a79)
}

pub fn f10(v0: Adt0, v1: Adt6) -> Adt5 {
    Adt5::C0(std::rc::Rc::new(v0), None::<Bytes>, std::rc::Rc::new(v1))
}

pub fn f11(v0: Fn0, p84: u64, p85: Adt4) -> (Adt4, bool) {
    let m83 = (p84, p85);
    {
        let mut m83 = m83;
        loop {
            let step = {
                let (v1, v2) = m83;
                let m80 = v1;
                if m80 == 0 {
                    Err::<(u64, Adt4), (Adt4, bool)>((v2, false))
                } else {
                    let v3 = m80 - 1;
                    let m82 = {
                        let c81: Fn0 = v0.clone();
                        c81.apply(v2.clone())
                    };
                    match m82 {
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
                Ok(next) => { m83 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt4) -> Option<Adt4> {
    Some(Adt4::C1(std::rc::Rc::new(v0)))
}

pub fn f13(v0: Adt6) -> Adt6 {
    f33(v0.clone(), match v0.clone() {
        Adt6::C0(h86, _, _, _, _, _, _) => {
            h86
        }
    }, match v0.clone() {
        Adt6::C0(_, h87, _, _, _, _, _) => {
            h87
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h88, _, _, _, _) => {
            h88
        }
    }, match v0 {
        Adt6::C0(_, _, _, h89, _, _, _) => {
            h89
        }
    }, true)
}

pub fn f14(v0: Adt6, v1: Bytes, v2: Bytes) -> (u64, bool) {
    let m91 = f34(Fn1::F35(v2, v1), None::<Adt2>, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h90, _) => {
            h90
        }
    });
    match m91 {
        None => {
            (match v0 {
                Adt6::C0(_, _, _, _, _, _, h94) => {
                    h94
                }
            }, false)
        }
        Some(v3) => {
            (match v3.clone() {
                Adt2::C0(_, _, h92, _, _) => {
                    h92
                }
            }, match v3 {
                Adt2::C0(_, _, _, _, h93) => {
                    h93
                }
            })
        }
    }
}

pub fn f15(v0: Adt6) -> R<Adt5> {
    let v1: Adt8 = f36(v0)?;
    Ok(f10(match v1.clone() {
        Adt8::C0(h95, _) => {
            (*h95).clone()
        }
    }, match v1 {
        Adt8::C0(_, h96) => {
            (*h96).clone()
        }
    }))
}

pub fn f16(v0: Adt6, v1: Bytes, v2: Bytes) -> Adt0 {
    let m98 = f34(Fn1::F37(v2, v1), None::<Adt2>, match v0 {
        Adt6::C0(_, _, _, _, _, h97, _) => {
            h97
        }
    });
    match m98 {
        None => {
            Adt0::C0
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, _, _, h99, _) => {
                    (*h99).clone()
                }
            }
        }
    }
}

pub fn f17(v0: Bytes) -> R<bool> {
    let a100 = f38(v0, 0u64)?;
    let a101 = 63u64;
    Ok(nat_eq(a100, a101))
}

pub fn f18(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F40(v1), None::<Adt0>, v0)
}

pub fn f19(v0: Adt6) -> R<Adt8> {
    let v1: Adt8 = f36(v0.clone())?;
    let v2: Adt6 = match v1.clone() {
        Adt8::C0(_, h102) => {
            (*h102).clone()
        }
    };
    Ok(Adt8::C0(std::rc::Rc::new(match v1 {
        Adt8::C0(h103, _) => {
            (*h103).clone()
        }
    }), std::rc::Rc::new(f33(v2.clone(), match v2.clone() {
        Adt6::C0(h104, _, _, _, _, _, _) => {
            h104
        }
    }, match v2.clone() {
        Adt6::C0(_, h105, _, _, _, _, _) => {
            h105
        }
    }, List::cons(f8(f9(Str::lit("?")), f41(match v0 {
        Adt6::C0(_, h106, _, _, _, _, _) => {
            h106
        }
    })), match v2.clone() {
        Adt6::C0(_, _, h107, _, _, _, _) => {
            h107
        }
    }), match v2.clone() {
        Adt6::C0(_, _, _, h108, _, _, _) => {
            h108
        }
    }, match v2 {
        Adt6::C0(_, _, _, _, h109, _, _) => {
            h109
        }
    }))))
}

pub fn f20(v0: Adt0) -> u64 {
    let m110 = v0;
    match m110 {
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

pub fn f21(v0: Adt4, v1: Adt0) -> List<Adt0> {
    let m111 = v0;
    match m111 {
        Adt4::C0 => {
            List::<Adt0>::nil()
        }
        Adt4::C1(r115) => {
            let v2 = (*r115).clone();
            let m112 = v1;
            match m112 {
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
                Adt0::C10(r113, r114) => {
                    let v3 = (*r113).clone();
                    let v4 = (*r114).clone();
                    List::cons(v3, f21(v2, v4))
                }
                Adt0::C11(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
            }
        }
    }
}

pub fn f22(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: Adt0, v7: u64, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m116 = v0;
    match m116 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r123) => {
            let v9 = (*r123).clone();
            let m117 = v4.clone();
            match m117.uncons() {
                None => {
                    Ok(f10(v6, v1))
                }
                Some((_, _)) => {
                    let v12: Adt8 = f27(v9.clone(), v1, v6)?;
                    let v13: Adt0 = match v12.clone() {
                        Adt8::C0(h118, _) => {
                            (*h118).clone()
                        }
                    };
                    let v14: Adt6 = match v12 {
                        Adt8::C0(_, h119) => {
                            (*h119).clone()
                        }
                    };
                    let m120 = v13.clone();
                    match m120 {
                        Adt0::C0 => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C1(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C2(_, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C3(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C4(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C5(r121, r122) => {
                            let v15 = (*r121).clone();
                            let v16 = (*r122).clone();
                            f42(v9.clone(), v14, v2, v3, f43(v9, v15), v16, v4, v5, v7, v8)
                        }
                        Adt0::C6(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C7(_, _, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C8(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C9 => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C10(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C11(_, _, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                    }
                }
            }
        }
    }
}

pub fn f23(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<(Bytes, Adt0)>, v5: bool, v6: Adt0) -> R<Adt5> {
    let m124 = v0;
    match m124 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r138) => {
            let v7 = (*r138).clone();
            let m125 = v2;
            match m125 {
                Adt0::C0 => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C1(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C2(_, _, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C3(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C4(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C5(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C6(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C7(_, _, _, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C8(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C9 => {
                    if v5 {
                        let v15: Adt5 = f2(v7, v1, v3, v4)?;
                        let m133 = match v15.clone() {
                            Adt5::C0(_, h132, _) => {
                                h132
                            }
                        };
                        match m133 {
                            None => {
                                Ok(f10(Adt0::C5(std::rc::Rc::new(v6), std::rc::Rc::new(match v15.clone() {
                                    Adt5::C0(h134, _, _) => {
                                        (*h134).clone()
                                    }
                                })), match v15 {
                                    Adt5::C0(_, _, h135) => {
                                        (*h135).clone()
                                    }
                                }))
                            }
                            Some(_) => {
                                Ok(v15)
                            }
                        }
                    } else {
                        f24(v7, v1, v3, v4)
                    }
                }
                Adt0::C10(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C11(_, v9, r136, r137) => {
                    let v10 = (*r136).clone();
                    let v11 = (*r137).clone();
                    let v12: Adt5 = f24(v7.clone(), v1, v10.clone(), v4.clone())?;
                    let m127 = match v12.clone() {
                        Adt5::C0(_, h126, _) => {
                            h126
                        }
                    };
                    match m127 {
                        None => {
                            let v14: Adt5 = f23(v7, match v12 {
                                Adt5::C0(_, _, h128) => {
                                    (*h128).clone()
                                }
                            }, v11, v3, List::cons((v9, v10), v4), v5, v6)?;
                            Ok(Adt5::C0(std::rc::Rc::new(match v14.clone() {
                                Adt5::C0(h129, _, _) => {
                                    (*h129).clone()
                                }
                            }), match v14.clone() {
                                Adt5::C0(_, h130, _) => {
                                    h130
                                }
                            }, std::rc::Rc::new(match v14 {
                                Adt5::C0(_, _, h131) => {
                                    (*h131).clone()
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

pub fn f24(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m139 = v0;
    match m139 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r144) => {
            let v4 = (*r144).clone();
            let v5: Adt5 = f2(v4.clone(), v1, v2.clone(), v3)?;
            let m141 = match v5.clone() {
                Adt5::C0(_, h140, _) => {
                    h140
                }
            };
            match m141 {
                None => {
                    f45(v4, match v5.clone() {
                        Adt5::C0(_, _, h142) => {
                            (*h142).clone()
                        }
                    }, v2, match v5 {
                        Adt5::C0(h143, _, _) => {
                            (*h143).clone()
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

pub fn f25(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Result<Adt3, Bytes>> {
    let v3: Adt8 = f27(v0.clone(), v1.clone(), v2)?;
    let v4: (List<Option<Adt0>>, Adt6) = f46(v0, match v3.clone() {
        Adt8::C0(_, h145) => {
            (*h145).clone()
        }
    }, f47(match v1 {
        Adt6::C0(_, _, h146, _, _, _, _) => {
            h146
        }
    }))?;
    let m148 = f48(v4.clone());
    if match m148 {
        Adt6::C0(_, _, _, _, h147, _, _) => {
            h147
        }
    } {
        Ok(Err::<Adt3, Bytes>(f9(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        Ok(Ok::<Adt3, Bytes>(Adt3::C0(std::rc::Rc::new(match v3 {
            Adt8::C0(h149, _) => {
                (*h149).clone()
            }
        }), {
            let (h150, _) = v4;
            h150
        })))
    }
}

pub fn f26(v0: Adt6) -> Adt7 {
    Adt7::C0(None::<Bytes>, std::rc::Rc::new(v0))
}

pub fn f27(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Adt8> {
    let m152 = f49(v0, match v1.clone() {
        Adt6::C0(h151, _, _, _, _, _, _) => {
            h151
        }
    }, v2.clone())?;
    match m152 {
        None => {
            Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f13(v1))))
        }
        Some(v3) => {
            Ok(Adt8::C0(std::rc::Rc::new(v3), std::rc::Rc::new(v1)))
        }
    }
}

pub fn f28(v0: Adt0) -> R<Option<Bytes>> {
    let m153 = v0;
    match m153 {
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
            if f17(v1.clone())? {
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

pub fn f29(v0: Adt0) -> Bytes {
    let m154 = v0;
    match m154 {
        Adt0::C0 => {
            Bytes::lit(&[])
        }
        Adt0::C1(_) => {
            Bytes::lit(&[])
        }
        Adt0::C2(v1, v2, _) => {
            f8(f8(v1, f9(Str::lit("::"))), v2)
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

pub fn f30(v0: Adt6, v1: Bytes, v2: Adt0) -> Adt6 {
    f33(v0.clone(), List::cons((v1, v2), match v0.clone() {
        Adt6::C0(h155, _, _, _, _, _, _) => {
            h155
        }
    }), match v0.clone() {
        Adt6::C0(_, h156, _, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h157, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h158, _, _, _) => {
            h158
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h159, _, _) => {
            h159
        }
    })
}

pub fn f31(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m160 = v0;
    match m160 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r178) => {
            let v4 = (*r178).clone();
            let v5: u64 = f50(v4.clone(), v2.clone(), v3.clone());
            let a161 = v5;
            let a162 = 0u64;
            if nat_eq(a161, a162) {
                Ok(f26(v1))
            } else {
                let a163 = v5;
                let a164 = 1u64;
                if nat_eq(a163, a164) {
                    if if f51(v4.clone(), v1.clone(), v2.clone()) {
                        true
                    } else {
                        f51(v4.clone(), v1.clone(), v3.clone())
                    } {
                        Ok(f26(v1))
                    } else {
                        let v6: (Adt0, Adt0) = f52(v2);
                        let v7: (Adt0, Adt0) = f52(v3);
                        let v8: Adt7 = f5(v4.clone(), v1, {
                            let (h165, _) = v6;
                            h165
                        }, {
                            let (h166, _) = v7;
                            h166
                        })?;
                        let m168 = match v8.clone() {
                            Adt7::C0(h167, _) => {
                                h167
                            }
                        };
                        match m168 {
                            None => {
                                f53(v4, match v8 {
                                    Adt7::C0(_, h169) => {
                                        (*h169).clone()
                                    }
                                }, {
                                    let (_, h170) = v6;
                                    h170
                                }, {
                                    let (_, h171) = v7;
                                    h171
                                })
                            }
                            Some(_) => {
                                Ok(v8)
                            }
                        }
                    }
                } else {
                    let a172 = v5;
                    let a173 = 2u64;
                    if nat_eq(a172, a173) {
                        f54(v4, v1, v2, v3)
                    } else {
                        if if f51(v4.clone(), v1.clone(), v2.clone()) {
                            true
                        } else {
                            f51(v4.clone(), v1.clone(), v3.clone())
                        } {
                            Ok(f26(v1))
                        } else {
                            let v10: (bool, Adt6) = f55(v4.clone(), v1, v2.clone())?;
                            if f56(v10.clone()) {
                                Ok(f26({
                                    let (_, h174) = v10;
                                    h174
                                }))
                            } else {
                                let v11: (bool, Adt6) = f55(v4, {
                                    let (_, h175) = v10;
                                    h175
                                }, v3.clone())?;
                                if f56(v11.clone()) {
                                    Ok(f26({
                                        let (_, h176) = v11;
                                        h176
                                    }))
                                } else {
                                    let v12: Bytes = f6(v2)?;
                                    let v13: Bytes = f6(v3)?;
                                    Ok(f57({
                                        let (_, h177) = v11;
                                        h177
                                    }, f8(f8(f8(f9(Str::lit("expected ")), v12), f9(Str::lit(", found "))), v13)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt0, v1: bool) -> R<Bytes> {
    let (h179, _) = f58(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h179)
}

pub fn f33(v0: Adt6, v1: List<(Bytes, Adt0)>, v2: u64, v3: List<Bytes>, v4: u64, v5: bool) -> Adt6 {
    Adt6::C0(v1, v2, v3, v4, v5, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h180, _) => {
            h180
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, h181) => {
            h181
        }
    })
}

pub fn f34(v0: Fn1, p185: Option<Adt2>, p186: List<Adt2>) -> Option<Adt2> {
    let m184 = (p185, p186);
    {
        let mut m184 = m184;
        loop {
            let step = {
                let (v1, v2) = m184;
                let m182 = v2;
                match m182.uncons() {
                    None => {
                        Err::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(({
                            let c183: Fn1 = v0.clone();
                            c183.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m187 = v2.clone();
    match m187 {
        None => {
            let a189 = match v3.clone() {
                Adt2::C0(h188, _, _, _, _) => {
                    h188
                }
            };
            let a190 = v1;
            if if equal(a189, a190) {
                let a192 = match v3.clone() {
                    Adt2::C0(_, h191, _, _, _) => {
                        h191
                    }
                };
                let a193 = v0;
                equal(a192, a193)
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

pub fn f36(v0: Adt6) -> R<Adt8> {
    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C3(f8(f9(Str::lit("?")), f41(match v0.clone() {
        Adt6::C0(_, h194, _, _, _, _, _) => {
            h194
        }
    })))), std::rc::Rc::new(f33(v0.clone(), match v0.clone() {
        Adt6::C0(h195, _, _, _, _, _, _) => {
            h195
        }
    }, {
        let a197 = match v0.clone() {
            Adt6::C0(_, h196, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = 1u64;
        nat_add(a197, a198)?
    }, match v0.clone() {
        Adt6::C0(_, _, h199, _, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h200, _, _, _) => {
            h200
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h201, _, _) => {
            h201
        }
    }))))
}

pub fn f37(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m202 = v2.clone();
    match m202 {
        None => {
            let a204 = match v3.clone() {
                Adt2::C0(h203, _, _, _, _) => {
                    h203
                }
            };
            let a205 = v1;
            if if equal(a204, a205) {
                let a207 = match v3.clone() {
                    Adt2::C0(_, h206, _, _, _) => {
                        h206
                    }
                };
                let a208 = v0;
                equal(a207, a208)
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
    let m209 = f59(v0, v1);
    match m209 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a254 = {
                let a230 = {
                    let a218 = {
                        let a212 = {
                            let a210 = v3;
                            let a211 = 128u8;
                            fixed_u8::bit_and(a210, a211)
                        };
                        let a213 = 128u8;
                        if equal(a212, a213) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a219 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 64u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 64u8;
                        if equal(a216, a217) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a218, a219)?
                };
                let a231 = {
                    let a228 = {
                        let a222 = {
                            let a220 = v3;
                            let a221 = 32u8;
                            fixed_u8::bit_and(a220, a221)
                        };
                        let a223 = 32u8;
                        if equal(a222, a223) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a229 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 16u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 16u8;
                        if equal(a226, a227) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a228, a229)?
                };
                nat_add(a230, a231)?
            };
            let a255 = {
                let a252 = {
                    let a240 = {
                        let a234 = {
                            let a232 = v3;
                            let a233 = 8u8;
                            fixed_u8::bit_and(a232, a233)
                        };
                        let a235 = 8u8;
                        if equal(a234, a235) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a241 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 4u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 4u8;
                        if equal(a238, a239) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a240, a241)?
                };
                let a253 = {
                    let a250 = {
                        let a244 = {
                            let a242 = v3;
                            let a243 = 2u8;
                            fixed_u8::bit_and(a242, a243)
                        };
                        let a245 = 2u8;
                        if equal(a244, a245) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a251 = {
                        let a248 = {
                            let a246 = v3;
                            let a247 = 1u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 1u8;
                        if equal(a248, a249) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a250, a251)?
                };
                nat_add(a252, a253)?
            };
            nat_add(a254, a255)
        }
    }
}

pub fn f39(v0: Fn2, p259: Option<Adt0>, p260: List<(Bytes, Adt0)>) -> Option<Adt0> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m256 = v2;
                match m256.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(({
                            let c257: Fn2 = v0.clone();
                            c257.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m261 = v1.clone();
    match m261 {
        None => {
            let a263 = {
                let (h262, _) = v2;
                h262
            };
            let a264 = v0;
            if equal(a263, a264) {
                Some({
                    let (_, h265) = v2;
                    h265
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
    f60(f61(), 10u64, v0)
}

pub fn f42(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt9>, v5: Adt0, v6: List<Adt0>, v7: u64, v8: u64, v9: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m266 = v0;
    match m266 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r285) => {
            let v10 = (*r285).clone();
            let m267 = v4.clone();
            match m267.uncons() {
                None => {
                    f22(v10, v1, v2, v3, v6, v7, v5, v8, v9)
                }
                Some((v11, v12)) => {
                    let a269 = match v11.clone() {
                        Adt9::C0(h268, _, _) => {
                            h268
                        }
                    };
                    let a270 = 0u64;
                    if nat_eq(a269, a270) {
                        let m271 = v6;
                        match m271.uncons() {
                            None => {
                                Ok(f10(Adt0::C5(std::rc::Rc::new(f62(v4)), std::rc::Rc::new(v5)), v1))
                            }
                            Some((v13, v14)) => {
                                let v15: Adt5 = f2(v10.clone(), v1, v13.clone(), v9.clone())?;
                                let m273 = match v15.clone() {
                                    Adt5::C0(_, h272, _) => {
                                        h272
                                    }
                                };
                                match m273 {
                                    None => {
                                        let v17: Adt7 = f5(v10.clone(), match v15.clone() {
                                            Adt5::C0(_, _, h274) => {
                                                (*h274).clone()
                                            }
                                        }, match v11.clone() {
                                            Adt9::C0(_, _, h275) => {
                                                (*h275).clone()
                                            }
                                        }, match v15 {
                                            Adt5::C0(h276, _, _) => {
                                                (*h276).clone()
                                            }
                                        })?;
                                        let m278 = match v17.clone() {
                                            Adt7::C0(h277, _) => {
                                                h277
                                            }
                                        };
                                        match m278 {
                                            None => {
                                                f63(v10, match v17 {
                                                    Adt7::C0(_, h280) => {
                                                        (*h280).clone()
                                                    }
                                                }, v2, v3, v11, v13, v12, v5, v14, v7, {
                                                    let a281 = v8;
                                                    let a282 = 1u64;
                                                    nat_add(a281, a282)?
                                                }, v9)
                                            }
                                            Some(v18) => {
                                                Ok(f7(match v17 {
                                                    Adt7::C0(_, h279) => {
                                                        (*h279).clone()
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
                        let v19: Adt8 = f36(v1)?;
                        f63(v10, match v19.clone() {
                            Adt8::C0(_, h283) => {
                                (*h283).clone()
                            }
                        }, v2, v3, v11, match v19 {
                            Adt8::C0(h284, _) => {
                                (*h284).clone()
                            }
                        }, v12, v5, v6, v7, v8, v9)
                    }
                }
            }
        }
    }
}

pub fn f43(v0: Adt4, v1: Adt0) -> List<Adt9> {
    let m286 = v0;
    match m286 {
        Adt4::C0 => {
            List::<Adt9>::nil()
        }
        Adt4::C1(r290) => {
            let v2 = (*r290).clone();
            let m287 = v1;
            match m287 {
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
                Adt0::C11(v3, v4, r288, r289) => {
                    let v5 = (*r288).clone();
                    let v6 = (*r289).clone();
                    List::cons(Adt9::C0(v3, v4, std::rc::Rc::new(v5)), f43(v2, v6))
                }
            }
        }
    }
}

pub fn f44(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: u64, v7: Adt0, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m291 = v0;
    match m291 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r293) => {
            let v9 = (*r293).clone();
            let m292 = f28(v7.clone())?;
            match m292 {
                None => {
                    if f51(v9.clone(), v1.clone(), v7.clone()) {
                        f64(v9, v1, v4, v8)
                    } else {
                        let v11: Bytes = f6(v2)?;
                        let v12: Bytes = f6(v3)?;
                        let v13: Bytes = f6(v7)?;
                        Ok(f7(v1, f8(f8(f8(f8(f8(f8(f8(f8(f8(v11, f9(Str::lit(" supplies "))), f41(v5)), f9(Str::lit(" explicit argument"))), f65(v5)), f9(Str::lit(" but "))), v12), f9(Str::lit(" accepts "))), f41(v6)), f8(f8(f9(Str::lit(": after them its type is ")), v13), f9(Str::lit(", not a function"))))))
                    }
                }
                Some(_) => {
                    f64(v9, v1, v4, v8)
                }
            }
        }
    }
}

pub fn f45(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt5> {
    let v4: Adt8 = f27(v0.clone(), v1, v3)?;
    let v5: Adt0 = match v4.clone() {
        Adt8::C0(h294, _) => {
            (*h294).clone()
        }
    };
    let v6: Adt6 = match v4 {
        Adt8::C0(_, h295) => {
            (*h295).clone()
        }
    };
    let v7: u64 = f20(v2.clone());
    let a296 = v7;
    let a297 = 8u64;
    if if nat_eq(a296, a297) {
        true
    } else {
        let a298 = v7;
        let a299 = 6u64;
        nat_eq(a298, a299)
    } {
        let v8: Bytes = f6(v2)?;
        Ok(f7(v6, f8(v8, f9(Str::lit(" is a term, not a type")))))
    } else {
        let a300 = f20(v5.clone());
        let a301 = 2u64;
        if nat_lt(a300, a301) {
            Ok(f10(v5, v6))
        } else {
            let m302 = f28(v5.clone())?;
            match m302 {
                None => {
                    if f51(v0, v6.clone(), v5.clone()) {
                        Ok(f10(v5, v6))
                    } else {
                        let v10: Bytes = f6(v2)?;
                        let v11: Bytes = f6(v5)?;
                        Ok(f7(v6, f8(f8(f8(v10, f9(Str::lit(" is not a type: its type is "))), v11), Bytes::lit(&[]))))
                    }
                }
                Some(_) => {
                    Ok(f10(v5, v6))
                }
            }
        }
    }
}

pub fn f46(v0: Adt4, v1: Adt6, v2: List<Bytes>) -> R<(List<Option<Adt0>>, Adt6)> {
    let m303 = v0;
    match m303 {
        Adt4::C0 => {
            Ok((List::<Option<Adt0>>::nil(), f13(v1)))
        }
        Adt4::C1(r311) => {
            let v3 = (*r311).clone();
            let m304 = v2;
            match m304.uncons() {
                None => {
                    Ok((List::<Option<Adt0>>::nil(), v1))
                }
                Some((v4, v5)) => {
                    let v6: Adt8 = f27(v3.clone(), v1, Adt0::C3(v4))?;
                    let v7: (bool, Adt6) = f55(v3.clone(), match v6.clone() {
                        Adt8::C0(_, h305) => {
                            (*h305).clone()
                        }
                    }, match v6.clone() {
                        Adt8::C0(h306, _) => {
                            (*h306).clone()
                        }
                    })?;
                    let v8: (List<Option<Adt0>>, Adt6) = f46(v3, {
                        let (_, h307) = v7.clone();
                        h307
                    }, v5)?;
                    Ok((List::cons(if f56(v7) {
                        None::<Adt0>
                    } else {
                        Some(match v6 {
                            Adt8::C0(h308, _) => {
                                (*h308).clone()
                            }
                        })
                    }, {
                        let (h309, _) = v8;
                        h309
                    }), {
                        let (_, h310) = v8;
                        h310
                    }))
                }
            }
        }
    }
}

pub fn f47(v0: List<Bytes>) -> List<Bytes> {
    f66(Fn3::F67, List::<Bytes>::nil(), v0)
}

pub fn f48(v0: (List<Option<Adt0>>, Adt6)) -> Adt6 {
    let (_, h312) = v0;
    h312
}

pub fn f49(v0: Adt4, v1: List<(Bytes, Adt0)>, v2: Adt0) -> R<Option<Adt0>> {
    let m313 = v0;
    match m313 {
        Adt4::C0 => {
            Ok(None::<Adt0>)
        }
        Adt4::C1(r350) => {
            let v3 = (*r350).clone();
            let m314 = v2;
            match m314 {
                Adt0::C0 => {
                    Ok(Some(Adt0::C0))
                }
                Adt0::C1(r335) => {
                    let v29 = (*r335).clone();
                    Ok(Some(Adt0::C1(std::rc::Rc::new(v29))))
                }
                Adt0::C2(v30, v31, r336) => {
                    let v32 = (*r336).clone();
                    Ok(Some(Adt0::C2(v30, v31, std::rc::Rc::new(v32))))
                }
                Adt0::C3(v4) => {
                    if f17(v4.clone())? {
                        let m315 = f68(v1.clone(), v4.clone());
                        match m315 {
                            None => {
                                Ok(Some(Adt0::C3(v4)))
                            }
                            Some(v5) => {
                                f49(v3, v1, v5)
                            }
                        }
                    } else {
                        Ok(Some(Adt0::C3(v4)))
                    }
                }
                Adt0::C4(r337, r338) => {
                    let v6 = (*r337).clone();
                    let v7 = (*r338).clone();
                    let m316 = f69(f49(v3.clone(), v1.clone(), v6)?, f49(v3, v1, v7)?);
                    match m316 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v8) => {
                            Ok(Some(Adt0::C4(std::rc::Rc::new({
                                let (h317, _) = v8;
                                h317
                            }), std::rc::Rc::new({
                                let (_, h318) = v8;
                                h318
                            }))))
                        }
                    }
                }
                Adt0::C5(r339, r340) => {
                    let v12 = (*r339).clone();
                    let v13 = (*r340).clone();
                    let m322 = f69(f49(v3.clone(), v1.clone(), v12)?, f49(v3, v1, v13)?);
                    match m322 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v14) => {
                            Ok(Some(Adt0::C5(std::rc::Rc::new({
                                let (h323, _) = v14;
                                h323
                            }), std::rc::Rc::new({
                                let (_, h324) = v14;
                                h324
                            }))))
                        }
                    }
                }
                Adt0::C6(r341, r342) => {
                    let v15 = (*r341).clone();
                    let v16 = (*r342).clone();
                    let m325 = f69(f49(v3.clone(), v1.clone(), v15)?, f49(v3, v1, v16)?);
                    match m325 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v17) => {
                            Ok(Some(Adt0::C6(std::rc::Rc::new({
                                let (h326, _) = v17;
                                h326
                            }), std::rc::Rc::new({
                                let (_, h327) = v17;
                                h327
                            }))))
                        }
                    }
                }
                Adt0::C7(v23, r343, r344, r345) => {
                    let v24 = (*r343).clone();
                    let v25 = (*r344).clone();
                    let v26 = (*r345).clone();
                    let m331 = f69(f49(v3.clone(), v1.clone(), v24)?, f49(v3.clone(), v1.clone(), v25)?);
                    match m331 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v27) => {
                            let m332 = f49(v3, v1, v26)?;
                            match m332 {
                                None => {
                                    Ok(None::<Adt0>)
                                }
                                Some(v28) => {
                                    Ok(Some(Adt0::C7(v23, std::rc::Rc::new({
                                        let (h333, _) = v27;
                                        h333
                                    }), std::rc::Rc::new({
                                        let (_, h334) = v27;
                                        h334
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
                Adt0::C10(r346, r347) => {
                    let v9 = (*r346).clone();
                    let v10 = (*r347).clone();
                    let m319 = f69(f49(v3.clone(), v1.clone(), v9)?, f49(v3, v1, v10)?);
                    match m319 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v11) => {
                            Ok(Some(Adt0::C10(std::rc::Rc::new({
                                let (h320, _) = v11;
                                h320
                            }), std::rc::Rc::new({
                                let (_, h321) = v11;
                                h321
                            }))))
                        }
                    }
                }
                Adt0::C11(v18, v19, r348, r349) => {
                    let v20 = (*r348).clone();
                    let v21 = (*r349).clone();
                    let m328 = f69(f49(v3.clone(), v1.clone(), v20)?, f49(v3, v1, v21)?);
                    match m328 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v22) => {
                            Ok(Some(Adt0::C11(v18, v19, std::rc::Rc::new({
                                let (h329, _) = v22;
                                h329
                            }), std::rc::Rc::new({
                                let (_, h330) = v22;
                                h330
                            }))))
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Adt4, v1: Adt0, v2: Adt0) -> u64 {
    let v3: u64 = f20(v1.clone());
    let a351 = v3;
    let a352 = f20(v2.clone());
    if nat_eq(a351, a352) {
        let a353 = v3;
        let a354 = 2u64;
        if nat_lt(a353, a354) {
            0u64
        } else {
            let a355 = v3;
            let a356 = 2u64;
            if if nat_eq(a355, a356) {
                true
            } else {
                let a357 = v3;
                let a358 = 3u64;
                if nat_eq(a357, a358) {
                    true
                } else {
                    let a359 = v3;
                    let a360 = 8u64;
                    nat_eq(a359, a360)
                }
            } {
                let a361 = f29(v1);
                let a362 = f29(v2);
                if equal(a361, a362) {
                    0u64
                } else {
                    3u64
                }
            } else {
                let a363 = v3;
                let a364 = 4u64;
                if nat_eq(a363, a364) {
                    let m365 = v1;
                    match m365 {
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
                        Adt0::C4(_, r372) => {
                            let v5 = (*r372).clone();
                            let m366 = v2;
                            match m366 {
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
                                Adt0::C4(_, r371) => {
                                    let v7 = (*r371).clone();
                                    let a369 = {
                                        let a367 = f21(v0.clone(), v5);
                                        length_list(a367)
                                    };
                                    let a370 = {
                                        let a368 = f21(v0, v7);
                                        length_list(a368)
                                    };
                                    if nat_eq(a369, a370) {
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
                    let a373 = v3;
                    let a374 = 5u64;
                    if if nat_eq(a373, a374) {
                        true
                    } else {
                        let a375 = v3;
                        let a376 = 6u64;
                        nat_eq(a375, a376)
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

pub fn f51(p382: Adt4, v1: Adt6, p383: Adt0) -> bool {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v0, v2) = m381;
                let m377 = v0;
                match m377 {
                    Adt4::C0 => {
                        Err::<(Adt4, Adt0), bool>(false)
                    }
                    Adt4::C1(r380) => {
                        let v3 = (*r380).clone();
                        let m378 = v2;
                        match m378 {
                            Adt0::C0 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C2(v4, v5, _) => {
                                Err::<(Adt4, Adt0), bool>(f70(v1.clone(), v4, v5))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C4(r379, _) => {
                                let v7 = (*r379).clone();
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
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Adt0) -> (Adt0, Adt0) {
    let m384 = v0;
    match m384 {
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
        Adt0::C4(r385, r386) => {
            let v1 = (*r385).clone();
            let v2 = (*r386).clone();
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

pub fn f53(p399: Adt4, p400: Adt6, p401: Adt0, p402: Adt0) -> R<Adt7> {
    let m398 = (p399, (p400, (p401, p402)));
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m398;
                let m387 = v0;
                match m387 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(f13(v1))))
                    }
                    Adt4::C1(r397) => {
                        let v4 = (*r397).clone();
                        let m388 = v2;
                        match m388 {
                            Adt0::C0 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C2(_, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C4(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C10(r395, r396) => {
                                let v5 = (*r395).clone();
                                let v6 = (*r396).clone();
                                let m389 = v3;
                                match m389 {
                                    Adt0::C0 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C1(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C2(_, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C3(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C4(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C5(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C6(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C7(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C8(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C9 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C10(r393, r394) => {
                                        let v7 = (*r393).clone();
                                        let v8 = (*r394).clone();
                                        let v9: Adt7 = f5(v4.clone(), v1, v5, v7)?;
                                        let m391 = match v9.clone() {
                                            Adt7::C0(h390, _) => {
                                                h390
                                            }
                                        };
                                        match m391 {
                                            None => {
                                                Ok::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>((v4, (match v9 {
                                                    Adt7::C0(_, h392) => {
                                                        (*h392).clone()
                                                    }
                                                }, (v6, v8))))
                                            }
                                            Some(_) => {
                                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(v9))
                                            }
                                        }
                                    }
                                    Adt0::C11(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                }
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m403 = v0;
    match m403 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r416) => {
            let v4 = (*r416).clone();
            let v5: bool = {
                let a404 = f20(v2.clone());
                let a405 = 5u64;
                nat_eq(a404, a405)
            };
            let v6: Adt10 = f71(v4.clone(), v2.clone(), v5);
            let v7: Adt10 = f71(v4.clone(), v3.clone(), v5);
            let v8: u64 = {
                let a407 = match v6.clone() {
                    Adt10::C0(h406, _) => {
                        h406
                    }
                };
                length_list(a407)
            };
            let v9: u64 = {
                let a409 = match v7.clone() {
                    Adt10::C0(h408, _) => {
                        h408
                    }
                };
                length_list(a409)
            };
            let a410 = v8;
            let a411 = v9;
            if nat_eq(a410, a411) {
                f72(v4, v1, match v6.clone() {
                    Adt10::C0(h412, _) => {
                        h412
                    }
                }, match v7.clone() {
                    Adt10::C0(h413, _) => {
                        h413
                    }
                }, match v7 {
                    Adt10::C0(_, h414) => {
                        (*h414).clone()
                    }
                }, match v6 {
                    Adt10::C0(_, h415) => {
                        (*h415).clone()
                    }
                }, v2, v3)
            } else {
                let v10: Bytes = f6(v2)?;
                let v11: Bytes = f6(v3)?;
                Ok(f57(v1, f8(f8(f8(f8(f8(f8(v10, f9(Str::lit(" binds "))), f41(v8)), f9(Str::lit(" arguments but "))), v11), f9(Str::lit(" binds "))), f41(v9))))
            }
        }
    }
}

pub fn f55(v0: Adt4, v1: Adt6, v2: Adt0) -> R<(bool, Adt6)> {
    let v3: Adt8 = f27(v0.clone(), v1, v2)?;
    let m418 = f73(v0, match v3.clone() {
        Adt8::C0(h417, _) => {
            (*h417).clone()
        }
    }, List::<Bytes>::nil())?;
    match m418 {
        None => {
            Ok((true, f13(match v3 {
                Adt8::C0(_, h420) => {
                    (*h420).clone()
                }
            })))
        }
        Some(v4) => {
            Ok((f74(Fn4::F75, false, v4)?, match v3 {
                Adt8::C0(_, h419) => {
                    (*h419).clone()
                }
            }))
        }
    }
}

pub fn f56(v0: (bool, Adt6)) -> bool {
    let (h421, _) = v0;
    h421
}

pub fn f57(v0: Adt6, v1: Bytes) -> Adt7 {
    Adt7::C0(Some(v1), std::rc::Rc::new(v0))
}

pub fn f58(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m422 = v0;
    match m422 {
        Adt0::C0 => {
            Ok((f76(Str::lit("(sort prop)")), v3))
        }
        Adt0::C1(r450) => {
            let v4 = (*r450).clone();
            Ok((f77(f77(f76(Str::lit("(sort (type ")), f78(v4)), f76(Str::lit("))"))), v3))
        }
        Adt0::C2(v5, v6, r451) => {
            let v7 = (*r451).clone();
            Ok((f79(v5, v6, v7), v3))
        }
        Adt0::C3(v8) => {
            Ok((f77(f77(f76(Str::lit("(local ")), if v1 {
                f80(v2, v8)
            } else {
                v8
            }), f76(Str::lit(")"))), v3))
        }
        Adt0::C4(r452, r453) => {
            let v9 = (*r452).clone();
            let v10 = (*r453).clone();
            let v11: (Bytes, u64) = f58(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f81(v10, v1, v2, {
                let (_, h423) = v11;
                h423
            })?;
            Ok((f77(f77(f77(f76(Str::lit("(app ")), {
                let (h424, _) = v11;
                h424
            }), {
                let (h425, _) = v12;
                h425
            }), f76(Str::lit(")"))), {
                let (_, h426) = v12;
                h426
            }))
        }
        Adt0::C5(r454, r455) => {
            let v13 = (*r454).clone();
            let v14 = (*r455).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f58(v14, v1, {
                let (_, h427) = {
                    let (h428, _) = v15.clone();
                    h428
                };
                h427
            }, {
                let (_, h429) = v15;
                h429
            })?;
            Ok((f77(f77(f77(f77(f76(Str::lit("(pi (")), {
                let (h430, _) = {
                    let (h431, _) = v15;
                    h431
                };
                h430
            }), f76(Str::lit(") "))), {
                let (h432, _) = v16;
                h432
            }), f76(Str::lit(")"))), {
                let (_, h433) = v16;
                h433
            }))
        }
        Adt0::C6(r456, r457) => {
            let v17 = (*r456).clone();
            let v18 = (*r457).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f58(v18, v1, {
                let (_, h434) = {
                    let (h435, _) = v19.clone();
                    h435
                };
                h434
            }, {
                let (_, h436) = v19;
                h436
            })?;
            Ok((f77(f77(f77(f77(f76(Str::lit("(lam (")), {
                let (h437, _) = {
                    let (h438, _) = v19;
                    h438
                };
                h437
            }), f76(Str::lit(") "))), {
                let (h439, _) = v20;
                h439
            }), f76(Str::lit(")"))), {
                let (_, h440) = v20;
                h440
            }))
        }
        Adt0::C7(v21, r458, r459, r460) => {
            let v22 = (*r458).clone();
            let v23 = (*r459).clone();
            let v24 = (*r460).clone();
            let v25: (Bytes, u64) = f83(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h441, _) = v25.clone();
                h441
            }), v2);
            let v27: (Bytes, u64) = f58(v22, v1, v26.clone(), {
                let (_, h442) = v25;
                h442
            })?;
            let v28: (Bytes, u64) = f58(v23, v1, v26.clone(), {
                let (_, h443) = v27;
                h443
            })?;
            let v29: (Bytes, u64) = f58(v24, v1, v26, {
                let (_, h444) = v28;
                h444
            })?;
            Ok((f77(f77(f77(f77(f77(f77(f77(f76(Str::lit("(let ")), {
                let (h445, _) = v25;
                h445
            }), f76(Str::lit(" "))), {
                let (h446, _) = v27;
                h446
            }), f76(Str::lit(" "))), {
                let (h447, _) = v28;
                h447
            }), f76(Str::lit(" "))), f77({
                let (h448, _) = v29;
                h448
            }, f76(Str::lit(")")))), {
                let (_, h449) = v29;
                h449
            }))
        }
        Adt0::C8(v30) => {
            Ok((f77(f77(f76(Str::lit("(nat ")), v30), f76(Str::lit(")"))), v3))
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

pub fn f59(v0: Bytes, v1: u64) -> Option<u8> {
    let a461 = v0;
    let a462 = v1;
    index_bytes(a461, a462)
}

pub fn f60(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a463 = v2;
    let a464 = 0u64;
    if nat_eq(a463, a464) {
        f84(v0, 0u64)
    } else {
        let (_, h465) = {
            let (h466, _) = f85(Fn5::F86(v1, v0), v2, (v2, Bytes::lit(&[])));
            h466
        };
        h465
    }
}

pub fn f61() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f62(v0: List<Adt9>) -> Adt0 {
    f87(Fn6::F88, Adt0::C9, f89(v0))
}

pub fn f63(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: Adt9, v5: Adt0, v6: List<Adt9>, v7: Adt0, v8: List<Adt0>, v9: u64, v10: u64, v11: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m467 = v0;
    match m467 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r474) => {
            let v12 = (*r474).clone();
            let v13: Adt11 = f90(v12.clone(), v1, v6, match v4.clone() {
                Adt9::C0(_, h468, _) => {
                    h468
                }
            }, v5.clone())?;
            let v14: Adt8 = f91(v12.clone(), match v13.clone() {
                Adt11::C0(_, h469) => {
                    (*h469).clone()
                }
            }, v7, match v4 {
                Adt9::C0(_, h470, _) => {
                    h470
                }
            }, v5)?;
            f42(v12, match v14.clone() {
                Adt8::C0(_, h471) => {
                    (*h471).clone()
                }
            }, v2, v3, match v13 {
                Adt11::C0(h472, _) => {
                    h472
                }
            }, match v14 {
                Adt8::C0(h473, _) => {
                    (*h473).clone()
                }
            }, v8, v9, v10, v11)
        }
    }
}

pub fn f64(v0: Adt4, v1: Adt6, v2: List<Adt0>, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m475 = v0;
    match m475 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r483) => {
            let v4 = (*r483).clone();
            let m476 = v2;
            match m476.uncons() {
                None => {
                    f15(v1)
                }
                Some((v5, v6)) => {
                    let v7: Adt5 = f2(v4.clone(), v1, v5, v3.clone())?;
                    let m478 = match v7.clone() {
                        Adt5::C0(_, h477, _) => {
                            h477
                        }
                    };
                    match m478 {
                        None => {
                            let v9: Adt5 = f64(v4, match v7 {
                                Adt5::C0(_, _, h479) => {
                                    (*h479).clone()
                                }
                            }, v6, v3)?;
                            Ok(Adt5::C0(std::rc::Rc::new(match v9.clone() {
                                Adt5::C0(h480, _, _) => {
                                    (*h480).clone()
                                }
                            }), match v9.clone() {
                                Adt5::C0(_, h481, _) => {
                                    h481
                                }
                            }, std::rc::Rc::new(match v9 {
                                Adt5::C0(_, _, h482) => {
                                    (*h482).clone()
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

pub fn f65(v0: u64) -> Bytes {
    let a484 = v0;
    let a485 = 1u64;
    if nat_eq(a484, a485) {
        Bytes::lit(&[])
    } else {
        f9(Str::lit("s"))
    }
}

pub fn f66(v0: Fn3, p489: List<Bytes>, p490: List<Bytes>) -> List<Bytes> {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v1, v2) = m488;
                let m486 = v2;
                match m486.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c487: Fn3 = v0.clone();
                            c487.apply(v1, v3)
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

pub fn f67(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f68(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F92(v1), None::<Adt0>, v0)
}

pub fn f69(v0: Option<Adt0>, v1: Option<Adt0>) -> Option<(Adt0, Adt0)> {
    let m491 = v0;
    match m491 {
        None => {
            None::<(Adt0, Adt0)>
        }
        Some(v2) => {
            let m492 = v1;
            match m492 {
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

pub fn f70(v0: Adt6, v1: Bytes, v2: Bytes) -> bool {
    let v3: (u64, bool) = f14(v0, v1, v2);
    let a494 = {
        let (h493, _) = v3;
        h493
    };
    let a495 = 1u64;
    if nat_eq(a494, a495) {
        true
    } else {
        let a497 = {
            let (h496, _) = v3;
            h496
        };
        let a498 = 3u64;
        if nat_eq(a497, a498) {
            let (_, h499) = v3;
            h499
        } else {
            false
        }
    }
}

pub fn f71(v0: Adt4, v1: Adt0, v2: bool) -> Adt10 {
    let m500 = v0;
    match m500 {
        Adt4::C0 => {
            Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(v1))
        }
        Adt4::C1(r521) => {
            let v3 = (*r521).clone();
            let m501 = v1;
            match m501 {
                Adt0::C0 => {
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C0))
                }
                Adt0::C1(r506) => {
                    let v10 = (*r506).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C1(std::rc::Rc::new(v10))))
                }
                Adt0::C2(v11, v12, r507) => {
                    let v13 = (*r507).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C2(v11, v12, std::rc::Rc::new(v13))))
                }
                Adt0::C3(v14) => {
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C3(v14)))
                }
                Adt0::C4(r508, r509) => {
                    let v15 = (*r508).clone();
                    let v16 = (*r509).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(v15), std::rc::Rc::new(v16))))
                }
                Adt0::C5(r510, r511) => {
                    let v4 = (*r510).clone();
                    let v5 = (*r511).clone();
                    if v2 {
                        let v6: Adt10 = f71(v3.clone(), v5, v2);
                        Adt10::C0(f93(Fn7::F94, match v6.clone() {
                            Adt10::C0(h502, _) => {
                                h502
                            }
                        }, f89(f43(v3, v4))), std::rc::Rc::new(match v6 {
                            Adt10::C0(_, h503) => {
                                (*h503).clone()
                            }
                        }))
                    } else {
                        Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C5(std::rc::Rc::new(v4), std::rc::Rc::new(v5))))
                    }
                }
                Adt0::C6(r512, r513) => {
                    let v7 = (*r512).clone();
                    let v8 = (*r513).clone();
                    if v2 {
                        Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new(v8))))
                    } else {
                        let v9: Adt10 = f71(v3.clone(), v8, v2);
                        Adt10::C0(f93(Fn7::F95, match v9.clone() {
                            Adt10::C0(h504, _) => {
                                h504
                            }
                        }, f89(f43(v3, v7))), std::rc::Rc::new(match v9 {
                            Adt10::C0(_, h505) => {
                                (*h505).clone()
                            }
                        }))
                    }
                }
                Adt0::C7(v17, r514, r515, r516) => {
                    let v18 = (*r514).clone();
                    let v19 = (*r515).clone();
                    let v20 = (*r516).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C7(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19), std::rc::Rc::new(v20))))
                }
                Adt0::C8(v21) => {
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C8(v21)))
                }
                Adt0::C9 => {
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C9))
                }
                Adt0::C10(r517, r518) => {
                    let v22 = (*r517).clone();
                    let v23 = (*r518).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(v22), std::rc::Rc::new(v23))))
                }
                Adt0::C11(v24, v25, r519, r520) => {
                    let v26 = (*r519).clone();
                    let v27 = (*r520).clone();
                    Adt10::C0(List::<Adt9>::nil(), std::rc::Rc::new(Adt0::C11(v24, v25, std::rc::Rc::new(v26), std::rc::Rc::new(v27))))
                }
            }
        }
    }
}

pub fn f72(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: List<Adt9>, v4: Adt0, v5: Adt0, v6: Adt0, v7: Adt0) -> R<Adt7> {
    let m522 = v0;
    match m522 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r547) => {
            let v8 = (*r547).clone();
            let m523 = v2;
            match m523.uncons() {
                None => {
                    f5(v8, v1, v5, v4)
                }
                Some((v9, v10)) => {
                    let m524 = v3;
                    match m524.uncons() {
                        None => {
                            f5(v8, v1, v5, v4)
                        }
                        Some((v11, v12)) => {
                            let a527 = match v9.clone() {
                                Adt9::C0(h525, _, _) => {
                                    h525
                                }
                            };
                            let a528 = match v11.clone() {
                                Adt9::C0(h526, _, _) => {
                                    h526
                                }
                            };
                            if nat_eq(a527, a528) {
                                let v13: Adt7 = f5(v8.clone(), v1, match v9.clone() {
                                    Adt9::C0(_, _, h529) => {
                                        (*h529).clone()
                                    }
                                }, match v11.clone() {
                                    Adt9::C0(_, _, h530) => {
                                        (*h530).clone()
                                    }
                                })?;
                                let m532 = match v13.clone() {
                                    Adt7::C0(h531, _) => {
                                        h531
                                    }
                                };
                                match m532 {
                                    None => {
                                        let v15: Adt0 = Adt0::C3(match v9 {
                                            Adt9::C0(_, h533, _) => {
                                                h533
                                            }
                                        });
                                        let v16: Adt11 = f90(v8.clone(), match v13 {
                                            Adt7::C0(_, h534) => {
                                                (*h534).clone()
                                            }
                                        }, v12, match v11.clone() {
                                            Adt9::C0(_, h535, _) => {
                                                h535
                                            }
                                        }, v15.clone())?;
                                        let v17: Adt8 = f91(v8.clone(), match v16.clone() {
                                            Adt11::C0(_, h536) => {
                                                (*h536).clone()
                                            }
                                        }, v4, match v11 {
                                            Adt9::C0(_, h537, _) => {
                                                h537
                                            }
                                        }, v15)?;
                                        let v18: Adt7 = f72(v8, match v17.clone() {
                                            Adt8::C0(_, h538) => {
                                                (*h538).clone()
                                            }
                                        }, v10, match v16 {
                                            Adt11::C0(h539, _) => {
                                                h539
                                            }
                                        }, match v17 {
                                            Adt8::C0(h540, _) => {
                                                (*h540).clone()
                                            }
                                        }, v5, v6, v7)?;
                                        Ok(Adt7::C0(match v18.clone() {
                                            Adt7::C0(h541, _) => {
                                                h541
                                            }
                                        }, std::rc::Rc::new(match v18 {
                                            Adt7::C0(_, h542) => {
                                                (*h542).clone()
                                            }
                                        })))
                                    }
                                    Some(_) => {
                                        Ok(v13)
                                    }
                                }
                            } else {
                                let v19: Bytes = f6(v6)?;
                                let v20: Bytes = f6(v7)?;
                                Ok(f57(v1, f8(f8(f8(f8(f8(f8(f8(f8(f8(f8(f8(f9(Str::lit("binder `")), match v9.clone() {
                                    Adt9::C0(_, h543, _) => {
                                        h543
                                    }
                                }), f9(Str::lit("` is "))), f96(match v9 {
                                    Adt9::C0(h544, _, _) => {
                                        h544
                                    }
                                })), f9(Str::lit(" in "))), v19), f9(Str::lit(" but `"))), match v11.clone() {
                                    Adt9::C0(_, h545, _) => {
                                        h545
                                    }
                                }), f9(Str::lit("` is "))), f96(match v11 {
                                    Adt9::C0(h546, _, _) => {
                                        h546
                                    }
                                })), f9(Str::lit(" in "))), v20)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f73(v0: Adt4, v1: Adt0, v2: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m548 = v0;
    match m548 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r561) => {
            let v3 = (*r561).clone();
            let m549 = v1;
            match m549 {
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
                    if f97(v2, v4.clone())? {
                        Ok(Some(List::<Bytes>::nil()))
                    } else {
                        Ok(Some(List::cons(v4, List::<Bytes>::nil())))
                    }
                }
                Adt0::C4(r550, r551) => {
                    let v5 = (*r550).clone();
                    let v6 = (*r551).clone();
                    Ok(f98(f73(v3.clone(), v5, v2.clone())?, f73(v3, v6, v2)?))
                }
                Adt0::C5(r552, r553) => {
                    let v9 = (*r552).clone();
                    let v10 = (*r553).clone();
                    f99(v3, v9, v2, v10)
                }
                Adt0::C6(r554, r555) => {
                    let v11 = (*r554).clone();
                    let v12 = (*r555).clone();
                    f99(v3, v11, v2, v12)
                }
                Adt0::C7(v13, r556, r557, r558) => {
                    let v14 = (*r556).clone();
                    let v15 = (*r557).clone();
                    let v16 = (*r558).clone();
                    Ok(f98(f98(f73(v3.clone(), v14, v2.clone())?, f73(v3.clone(), v15, v2.clone())?), f73(v3, v16, List::cons(v13, v2))?))
                }
                Adt0::C8(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C9 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C10(r559, r560) => {
                    let v7 = (*r559).clone();
                    let v8 = (*r560).clone();
                    Ok(f98(f73(v3.clone(), v7, v2.clone())?, f73(v3, v8, v2)?))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
            }
        }
    }
}

pub fn f74(v0: Fn4, p565: bool, p566: List<Bytes>) -> R<bool> {
    let m564 = (p565, p566);
    {
        let mut m564 = m564;
        loop {
            let step = {
                let (v1, v2) = m564;
                let m562 = v2;
                match m562.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c563: Fn4 = v0.clone();
                            c563.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        Ok(true)
    } else {
        f17(v1)
    }
}

pub fn f76(v0: Str) -> Bytes {
    let a567 = v0;
    utf8_encode(a567)
}

pub fn f77(v0: Bytes, v1: Bytes) -> Bytes {
    let a568 = v0;
    let a569 = v1;
    append_bytes(a568, a569)
}

pub fn f78(v0: Adt1) -> Bytes {
    let m570 = v0;
    match m570 {
        Adt1::C0(v1) => {
            f41(v1)
        }
        Adt1::C1(v2) => {
            v2
        }
        Adt1::C2(r571) => {
            let v3 = (*r571).clone();
            f77(f77(f76(Str::lit("(succ ")), f78(v3)), f76(Str::lit(")")))
        }
        Adt1::C3(r572) => {
            let v4 = (*r572).clone();
            f77(f77(f76(Str::lit("(max")), f100(v4)), f76(Str::lit(")")))
        }
        Adt1::C4(r573, r574) => {
            let v5 = (*r573).clone();
            let v6 = (*r574).clone();
            f77(f77(f77(f77(f76(Str::lit("(imax ")), f78(v5)), f76(Str::lit(" "))), f78(v6)), f76(Str::lit(")")))
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f79(v0: Bytes, v1: Bytes, v2: Adt1) -> Bytes {
    let v3: Bytes = f77(f77(f77(f76(Str::lit("(const ")), v0), f76(Str::lit("::"))), v1);
    let m575 = v2.clone();
    match m575 {
        Adt1::C0(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C1(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C2(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C3(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C4(_, _) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C5 => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C6(_, _) => {
            let v6: Bytes = f100(v2);
            f77(f77(f77(v3, f76(Str::lit(" ("))), f101(v6.clone(), 1u64, {
                let a577 = {
                    let a576 = v6;
                    length_bytes(a576)
                };
                let a578 = 1u64;
                nat_sub(a577, a578)
            })), f76(Str::lit("))")))
        }
    }
}

pub fn f80(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m579 = f102(Fn8::F103(v1.clone()), None::<Bytes>, v0);
    match m579 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f81(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m580 = v0;
    match m580 {
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
        Adt0::C10(r585, r586) => {
            let v4 = (*r585).clone();
            let v5 = (*r586).clone();
            let v6: (Bytes, u64) = f58(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f81(v5, v1, v2, {
                let (_, h581) = v6;
                h581
            })?;
            Ok((f77(f77(f76(Str::lit(" ")), {
                let (h582, _) = v6;
                h582
            }), {
                let (h583, _) = v7;
                h583
            }), {
                let (_, h584) = v7;
                h584
            }))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f82(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m587 = v0;
    match m587 {
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
        Adt0::C11(v4, v5, r610, r611) => {
            let v6 = (*r610).clone();
            let v7 = (*r611).clone();
            let v8: (Bytes, u64) = f83(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f58(v6, v1, v2.clone(), {
                let (_, h588) = v8.clone();
                h588
            })?;
            let v10: Bytes = f77(f77(f77(f77(f77(f77(f76(Str::lit("(")), f104(v4)), f76(Str::lit(" "))), {
                let (h589, _) = v8.clone();
                h589
            }), f76(Str::lit(" "))), {
                let (h590, _) = v9;
                h590
            }), f76(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h591, _) = v8;
                h591
            }), v2);
            let m592 = v7.clone();
            match m592 {
                Adt0::C0 => {
                    Ok(((v10, v11), {
                        let (_, h600) = v9;
                        h600
                    }))
                }
                Adt0::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h601) = v9;
                        h601
                    }))
                }
                Adt0::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h602) = v9;
                        h602
                    }))
                }
                Adt0::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h603) = v9;
                        h603
                    }))
                }
                Adt0::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h604) = v9;
                        h604
                    }))
                }
                Adt0::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h605) = v9;
                        h605
                    }))
                }
                Adt0::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h606) = v9;
                        h606
                    }))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h607) = v9;
                        h607
                    }))
                }
                Adt0::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h608) = v9;
                        h608
                    }))
                }
                Adt0::C9 => {
                    Ok(((v10, v11), {
                        let (_, h593) = v9;
                        h593
                    }))
                }
                Adt0::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h609) = v9;
                        h609
                    }))
                }
                Adt0::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v7, v1, v11, {
                        let (_, h594) = v9;
                        h594
                    })?;
                    Ok(((f77(f77(v10, f76(Str::lit(" "))), {
                        let (h595, _) = {
                            let (h596, _) = v16.clone();
                            h596
                        };
                        h595
                    }), {
                        let (_, h597) = {
                            let (h598, _) = v16;
                            h598
                        };
                        h597
                    }), {
                        let (_, h599) = v16;
                        h599
                    }))
                }
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f77(f76(Str::lit("x")), f41(v2)), {
            let a612 = v2;
            let a613 = 1u64;
            nat_add(a612, a613)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f84(v0: Bytes, v1: u64) -> Bytes {
    let m617 = {
        let a614 = v0;
        let a615 = v1;
        let a616 = 1u64;
        slice_bytes(a614, a615, a616)
    };
    match m617 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f85(v0: Fn5, p622: u64, p623: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m618 = v1;
                if m618 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m618 - 1;
                    let m620 = {
                        let c619: Fn5 = v0.clone();
                        c619.apply(v2.clone())
                    };
                    match m620 {
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
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f105(v1, v0, v2)
}

pub fn f87(v0: Fn6, p627: Adt0, p628: List<Adt9>) -> Adt0 {
    let m626 = (p627, p628);
    {
        let mut m626 = m626;
        loop {
            let step = {
                let (v1, v2) = m626;
                let m624 = v2;
                match m624.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt9>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt9>), Adt0>(({
                            let c625: Fn6 = v0.clone();
                            c625.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt0, v1: Adt9) -> Adt0 {
    Adt0::C11(match v1.clone() {
        Adt9::C0(h629, _, _) => {
            h629
        }
    }, match v1.clone() {
        Adt9::C0(_, h630, _) => {
            h630
        }
    }, std::rc::Rc::new(match v1 {
        Adt9::C0(_, _, h631) => {
            (*h631).clone()
        }
    }), std::rc::Rc::new(v0))
}

pub fn f89(v0: List<Adt9>) -> List<Adt9> {
    f93(Fn7::F106, List::<Adt9>::nil(), v0)
}

pub fn f90(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: Bytes, v4: Adt0) -> R<Adt11> {
    let m632 = v0;
    match m632 {
        Adt4::C0 => {
            Ok(Adt11::C0(v2, std::rc::Rc::new(f13(v1))))
        }
        Adt4::C1(r641) => {
            let v5 = (*r641).clone();
            let m633 = v2;
            match m633.uncons() {
                None => {
                    Ok(Adt11::C0(List::<Adt9>::nil(), std::rc::Rc::new(v1)))
                }
                Some((v6, v7)) => {
                    let v8: Adt8 = f91(v5.clone(), v1, match v6.clone() {
                        Adt9::C0(_, _, h634) => {
                            (*h634).clone()
                        }
                    }, v3.clone(), v4.clone())?;
                    let v9: Adt11 = f90(v5, match v8.clone() {
                        Adt8::C0(_, h635) => {
                            (*h635).clone()
                        }
                    }, v7, v3, v4)?;
                    Ok(Adt11::C0(List::cons(Adt9::C0(match v6.clone() {
                        Adt9::C0(h636, _, _) => {
                            h636
                        }
                    }, match v6 {
                        Adt9::C0(_, h637, _) => {
                            h637
                        }
                    }, std::rc::Rc::new(match v8 {
                        Adt8::C0(h638, _) => {
                            (*h638).clone()
                        }
                    })), match v9.clone() {
                        Adt11::C0(h639, _) => {
                            h639
                        }
                    }), std::rc::Rc::new(match v9 {
                        Adt11::C0(_, h640) => {
                            (*h640).clone()
                        }
                    })))
                }
            }
        }
    }
}

pub fn f91(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0) -> R<Adt8> {
    let m642 = f73(v0.clone(), v4.clone(), List::<Bytes>::nil())?;
    match m642 {
        None => {
            Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f13(v1))))
        }
        Some(v5) => {
            f107(v0, v1, v2, v3, v4, v5)
        }
    }
}

pub fn f92(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m643 = v1.clone();
    match m643 {
        None => {
            let a645 = {
                let (h644, _) = v2;
                h644
            };
            let a646 = v0;
            if equal(a645, a646) {
                Some({
                    let (_, h647) = v2;
                    h647
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

pub fn f93(v0: Fn7, p651: List<Adt9>, p652: List<Adt9>) -> List<Adt9> {
    let m650 = (p651, p652);
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v1, v2) = m650;
                let m648 = v2;
                match m648.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c649: Fn7 = v0.clone();
                            c649.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f96(v0: u64) -> Bytes {
    f104(v0)
}

pub fn f97(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f74(Fn4::F108(v1), false, v0)
}

pub fn f98(v0: Option<List<Bytes>>, v1: Option<List<Bytes>>) -> Option<List<Bytes>> {
    let m653 = v0;
    match m653 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m654 = v1;
            match m654 {
                None => {
                    None::<List<Bytes>>
                }
                Some(v3) => {
                    Some(f66(Fn3::F109, v3, v2))
                }
            }
        }
    }
}

pub fn f99(v0: Adt4, v1: Adt0, v2: List<Bytes>, v3: Adt0) -> R<Option<List<Bytes>>> {
    let m655 = v0;
    match m655 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r659) => {
            let v4 = (*r659).clone();
            let m656 = v1;
            match m656 {
                Adt0::C0 => {
                    f73(v4, v3, v2)
                }
                Adt0::C1(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C2(_, _, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C3(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C4(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C5(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C6(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C7(_, _, _, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C8(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C9 => {
                    f73(v4, v3, v2)
                }
                Adt0::C10(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C11(_, v6, r657, r658) => {
                    let v7 = (*r657).clone();
                    let v8 = (*r658).clone();
                    Ok(f98(f73(v4.clone(), v7, v2.clone())?, f99(v4, v8, List::cons(v6, v2), v3)?))
                }
            }
        }
    }
}

pub fn f100(v0: Adt1) -> Bytes {
    let m660 = v0;
    match m660 {
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
        Adt1::C6(r661, r662) => {
            let v7 = (*r661).clone();
            let v8 = (*r662).clone();
            f77(f77(f76(Str::lit(" ")), f78(v7)), f100(v8))
        }
    }
}

pub fn f101(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m666 = {
        let a663 = v0;
        let a664 = v1;
        let a665 = v2;
        slice_bytes(a663, a664, a665)
    };
    match m666 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f102(v0: Fn8, p670: Option<Bytes>, p671: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m669 = (p670, p671);
    {
        let mut m669 = m669;
        loop {
            let step = {
                let (v1, v2) = m669;
                let m667 = v2;
                match m667.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c668: Fn8 = v0.clone();
                            c668.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m672 = v1.clone();
    match m672 {
        None => {
            let a674 = {
                let (h673, _) = v2;
                h673
            };
            let a675 = v0;
            if equal(a674, a675) {
                Some({
                    let (_, h676) = v2;
                    h676
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

pub fn f104(v0: u64) -> Bytes {
    let a677 = v0;
    let a678 = 1u64;
    if nat_eq(a677, a678) {
        f76(Str::lit("implicit"))
    } else {
        let a679 = v0;
        let a680 = 2u64;
        if nat_eq(a679, a680) {
            f76(Str::lit("instance"))
        } else {
            f76(Str::lit("explicit"))
        }
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h681, _) = v2;
        h681
    };
    let a682 = v3;
    let a683 = 0u64;
    if nat_eq(a682, a683) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a684 = v3;
            let a685 = v1;
            let a686 = 0u64;
            nat_quot(a684, a685, a686)
        }, {
            let a691 = f84(v0, {
                let a687 = v3;
                let a688 = v1;
                let a689 = 0u64;
                nat_rem(a687, a688, a689)
            });
            let a692 = {
                let (_, h690) = v2;
                h690
            };
            append_bytes(a691, a692)
        }))
    }
}

pub fn f106(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f107(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt8> {
    let m693 = v0;
    match m693 {
        Adt4::C0 => {
            Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f13(v1))))
        }
        Adt4::C1(r747) => {
            let v6 = (*r747).clone();
            let m694 = v2;
            match m694 {
                Adt0::C0 => {
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C0), std::rc::Rc::new(v1)))
                }
                Adt0::C1(r732) => {
                    let v38 = (*r732).clone();
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C1(std::rc::Rc::new(v38))), std::rc::Rc::new(v1)))
                }
                Adt0::C2(v39, v40, r733) => {
                    let v41 = (*r733).clone();
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C2(v39, v40, std::rc::Rc::new(v41))), std::rc::Rc::new(v1)))
                }
                Adt0::C3(v7) => {
                    let a695 = v7.clone();
                    let a696 = v3;
                    if equal(a695, a696) {
                        Ok(Adt8::C0(std::rc::Rc::new(v4), std::rc::Rc::new(v1)))
                    } else {
                        Ok(Adt8::C0(std::rc::Rc::new(Adt0::C3(v7)), std::rc::Rc::new(v1)))
                    }
                }
                Adt0::C4(r734, r735) => {
                    let v8 = (*r734).clone();
                    let v9 = (*r735).clone();
                    let v10: Adt8 = f107(v6.clone(), v1, v8, v3.clone(), v4.clone(), v5.clone())?;
                    let v11: Adt8 = f107(v6, match v10.clone() {
                        Adt8::C0(_, h697) => {
                            (*h697).clone()
                        }
                    }, v9, v3, v4, v5)?;
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(match v10 {
                        Adt8::C0(h698, _) => {
                            (*h698).clone()
                        }
                    }), std::rc::Rc::new(match v11.clone() {
                        Adt8::C0(h699, _) => {
                            (*h699).clone()
                        }
                    }))), std::rc::Rc::new(match v11 {
                        Adt8::C0(_, h700) => {
                            (*h700).clone()
                        }
                    })))
                }
                Adt0::C5(r736, r737) => {
                    let v16 = (*r736).clone();
                    let v17 = (*r737).clone();
                    let v18: Adt12 = f110(v6.clone(), v1, v16, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v19: Adt8 = f111(v6, v18.clone(), v17, v3, v4, v5)?;
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C5(std::rc::Rc::new(match v18 {
                        Adt12::C0(h705, _, _, _) => {
                            (*h705).clone()
                        }
                    }), std::rc::Rc::new(match v19.clone() {
                        Adt8::C0(h706, _) => {
                            (*h706).clone()
                        }
                    }))), std::rc::Rc::new(match v19 {
                        Adt8::C0(_, h707) => {
                            (*h707).clone()
                        }
                    })))
                }
                Adt0::C6(r738, r739) => {
                    let v20 = (*r738).clone();
                    let v21 = (*r739).clone();
                    let v22: Adt12 = f110(v6.clone(), v1, v20, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v23: Adt8 = f111(v6, v22.clone(), v21, v3, v4, v5)?;
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C6(std::rc::Rc::new(match v22 {
                        Adt12::C0(h708, _, _, _) => {
                            (*h708).clone()
                        }
                    }), std::rc::Rc::new(match v23.clone() {
                        Adt8::C0(h709, _) => {
                            (*h709).clone()
                        }
                    }))), std::rc::Rc::new(match v23 {
                        Adt8::C0(_, h710) => {
                            (*h710).clone()
                        }
                    })))
                }
                Adt0::C7(v24, r740, r741, r742) => {
                    let v25 = (*r740).clone();
                    let v26 = (*r741).clone();
                    let v27 = (*r742).clone();
                    let v28: Adt8 = f107(v6.clone(), v1, v25, v3.clone(), v4.clone(), v5.clone())?;
                    let v29: Adt8 = f107(v6.clone(), match v28.clone() {
                        Adt8::C0(_, h711) => {
                            (*h711).clone()
                        }
                    }, v26, v3.clone(), v4.clone(), v5.clone())?;
                    let a712 = v24.clone();
                    let a713 = v3.clone();
                    if equal(a712, a713) {
                        Ok(Adt8::C0(std::rc::Rc::new(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                            Adt8::C0(h714, _) => {
                                (*h714).clone()
                            }
                        }), std::rc::Rc::new(match v29.clone() {
                            Adt8::C0(h715, _) => {
                                (*h715).clone()
                            }
                        }), std::rc::Rc::new(v27))), std::rc::Rc::new(match v29 {
                            Adt8::C0(_, h716) => {
                                (*h716).clone()
                            }
                        })))
                    } else {
                        if f97(v5.clone(), v24.clone())? {
                            let v30: (Bytes, Adt6) = f112(match v29.clone() {
                                Adt8::C0(_, h717) => {
                                    (*h717).clone()
                                }
                            }, v24.clone())?;
                            let v31: Adt8 = f107(v6.clone(), {
                                let (_, h718) = v30.clone();
                                h718
                            }, v27, v24, Adt0::C3({
                                let (h719, _) = v30.clone();
                                h719
                            }), List::<Bytes>::nil())?;
                            let v32: Adt8 = f107(v6, match v31.clone() {
                                Adt8::C0(_, h720) => {
                                    (*h720).clone()
                                }
                            }, match v31 {
                                Adt8::C0(h721, _) => {
                                    (*h721).clone()
                                }
                            }, v3, v4, v5)?;
                            Ok(Adt8::C0(std::rc::Rc::new(Adt0::C7({
                                let (h722, _) = v30;
                                h722
                            }, std::rc::Rc::new(match v28 {
                                Adt8::C0(h723, _) => {
                                    (*h723).clone()
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt8::C0(h724, _) => {
                                    (*h724).clone()
                                }
                            }), std::rc::Rc::new(match v32.clone() {
                                Adt8::C0(h725, _) => {
                                    (*h725).clone()
                                }
                            }))), std::rc::Rc::new(match v32 {
                                Adt8::C0(_, h726) => {
                                    (*h726).clone()
                                }
                            })))
                        } else {
                            let v33: Adt8 = f107(v6, match v29.clone() {
                                Adt8::C0(_, h727) => {
                                    (*h727).clone()
                                }
                            }, v27, v3, v4, v5)?;
                            Ok(Adt8::C0(std::rc::Rc::new(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                                Adt8::C0(h728, _) => {
                                    (*h728).clone()
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt8::C0(h729, _) => {
                                    (*h729).clone()
                                }
                            }), std::rc::Rc::new(match v33.clone() {
                                Adt8::C0(h730, _) => {
                                    (*h730).clone()
                                }
                            }))), std::rc::Rc::new(match v33 {
                                Adt8::C0(_, h731) => {
                                    (*h731).clone()
                                }
                            })))
                        }
                    }
                }
                Adt0::C8(v42) => {
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C8(v42)), std::rc::Rc::new(v1)))
                }
                Adt0::C9 => {
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C9), std::rc::Rc::new(v1)))
                }
                Adt0::C10(r743, r744) => {
                    let v12 = (*r743).clone();
                    let v13 = (*r744).clone();
                    let v14: Adt8 = f107(v6.clone(), v1, v12, v3.clone(), v4.clone(), v5.clone())?;
                    let v15: Adt8 = f107(v6, match v14.clone() {
                        Adt8::C0(_, h701) => {
                            (*h701).clone()
                        }
                    }, v13, v3, v4, v5)?;
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(match v14 {
                        Adt8::C0(h702, _) => {
                            (*h702).clone()
                        }
                    }), std::rc::Rc::new(match v15.clone() {
                        Adt8::C0(h703, _) => {
                            (*h703).clone()
                        }
                    }))), std::rc::Rc::new(match v15 {
                        Adt8::C0(_, h704) => {
                            (*h704).clone()
                        }
                    })))
                }
                Adt0::C11(v34, v35, r745, r746) => {
                    let v36 = (*r745).clone();
                    let v37 = (*r746).clone();
                    Ok(Adt8::C0(std::rc::Rc::new(Adt0::C11(v34, v35, std::rc::Rc::new(v36), std::rc::Rc::new(v37))), std::rc::Rc::new(v1)))
                }
            }
        }
    }
}

pub fn f108(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a748 = v2;
        let a749 = v0;
        equal(a748, a749)
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>, v6: List<(Bytes, Bytes)>, v7: bool) -> R<Adt12> {
    let m750 = v0;
    match m750 {
        Adt4::C0 => {
            Ok(Adt12::C0(std::rc::Rc::new(v2), v6, v7, std::rc::Rc::new(f13(v1))))
        }
        Adt4::C1(r777) => {
            let v8 = (*r777).clone();
            let m751 = v2.clone();
            match m751 {
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
                Adt0::C11(v9, v10, r775, r776) => {
                    let v11 = (*r775).clone();
                    let v12 = (*r776).clone();
                    let v13: Adt8 = f111(v8.clone(), Adt12::C0(std::rc::Rc::new(v2), v6.clone(), v7, std::rc::Rc::new(v1)), v11, v3.clone(), v4.clone(), v5.clone())?;
                    let v14: Adt8 = v13;
                    let a752 = v10.clone();
                    let a753 = v3.clone();
                    if equal(a752, a753) {
                        let v15: Adt12 = f110(v8, match v14.clone() {
                            Adt8::C0(_, h754) => {
                                (*h754).clone()
                            }
                        }, v12, v3, v4, v5, v6, true)?;
                        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                            Adt8::C0(h755, _) => {
                                (*h755).clone()
                            }
                        }), std::rc::Rc::new(match v15.clone() {
                            Adt12::C0(h756, _, _, _) => {
                                (*h756).clone()
                            }
                        }))), match v15.clone() {
                            Adt12::C0(_, h757, _, _) => {
                                h757
                            }
                        }, match v15.clone() {
                            Adt12::C0(_, _, h758, _) => {
                                h758
                            }
                        }, std::rc::Rc::new(match v15 {
                            Adt12::C0(_, _, _, h759) => {
                                (*h759).clone()
                            }
                        })))
                    } else {
                        if f97(v5.clone(), v10.clone())? {
                            let v16: (Bytes, Adt6) = f112(match v14.clone() {
                                Adt8::C0(_, h760) => {
                                    (*h760).clone()
                                }
                            }, v10.clone())?;
                            let v17: Adt12 = f110(v8, {
                                let (_, h761) = v16.clone();
                                h761
                            }, v12, v3, v4, v5, List::cons((v10, {
                                let (h762, _) = v16.clone();
                                h762
                            }), v6), v7)?;
                            Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, {
                                let (h763, _) = v16;
                                h763
                            }, std::rc::Rc::new(match v14 {
                                Adt8::C0(h764, _) => {
                                    (*h764).clone()
                                }
                            }), std::rc::Rc::new(match v17.clone() {
                                Adt12::C0(h765, _, _, _) => {
                                    (*h765).clone()
                                }
                            }))), match v17.clone() {
                                Adt12::C0(_, h766, _, _) => {
                                    h766
                                }
                            }, match v17.clone() {
                                Adt12::C0(_, _, h767, _) => {
                                    h767
                                }
                            }, std::rc::Rc::new(match v17 {
                                Adt12::C0(_, _, _, h768) => {
                                    (*h768).clone()
                                }
                            })))
                        } else {
                            let v18: Adt12 = f110(v8, match v14.clone() {
                                Adt8::C0(_, h769) => {
                                    (*h769).clone()
                                }
                            }, v12, v3, v4, v5, v6, v7)?;
                            Ok(Adt12::C0(std::rc::Rc::new(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                                Adt8::C0(h770, _) => {
                                    (*h770).clone()
                                }
                            }), std::rc::Rc::new(match v18.clone() {
                                Adt12::C0(h771, _, _, _) => {
                                    (*h771).clone()
                                }
                            }))), match v18.clone() {
                                Adt12::C0(_, h772, _, _) => {
                                    h772
                                }
                            }, match v18.clone() {
                                Adt12::C0(_, _, h773, _) => {
                                    h773
                                }
                            }, std::rc::Rc::new(match v18 {
                                Adt12::C0(_, _, _, h774) => {
                                    (*h774).clone()
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Adt4, v1: Adt12, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt8> {
    let m778 = v0;
    match m778 {
        Adt4::C0 => {
            Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f13(match v1 {
                Adt12::C0(_, _, _, h779) => {
                    (*h779).clone()
                }
            }))))
        }
        Adt4::C1(r785) => {
            let v6 = (*r785).clone();
            let v7: Adt8 = f113(v6.clone(), match v1.clone() {
                Adt12::C0(_, _, _, h780) => {
                    (*h780).clone()
                }
            }, v2, f114(match v1.clone() {
                Adt12::C0(_, h781, _, _) => {
                    h781
                }
            }))?;
            if match v1 {
                Adt12::C0(_, _, h782, _) => {
                    h782
                }
            } {
                Ok(v7)
            } else {
                f107(v6, match v7.clone() {
                    Adt8::C0(_, h783) => {
                        (*h783).clone()
                    }
                }, match v7 {
                    Adt8::C0(h784, _) => {
                        (*h784).clone()
                    }
                }, v3, v4, v5)
            }
        }
    }
}

pub fn f112(v0: Adt6, v1: Bytes) -> R<(Bytes, Adt6)> {
    let v2: u64 = {
        let a787 = match v0.clone() {
            Adt6::C0(_, _, _, h786, _, _, _) => {
                h786
            }
        };
        let a788 = 1u64;
        nat_add(a787, a788)?
    };
    Ok((f8(f8(v1, Bytes::lit(&[0x23])), f41(v2)), f33(v0.clone(), match v0.clone() {
        Adt6::C0(h789, _, _, _, _, _, _) => {
            h789
        }
    }, match v0.clone() {
        Adt6::C0(_, h790, _, _, _, _, _) => {
            h790
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h791, _, _, _, _) => {
            h791
        }
    }, v2, match v0 {
        Adt6::C0(_, _, _, _, h792, _, _) => {
            h792
        }
    })))
}

pub fn f113(p801: Adt4, p802: Adt6, p803: Adt0, p804: List<(Bytes, Bytes)>) -> R<Adt8> {
    let m800 = (p801, (p802, (p803, p804)));
    {
        let mut m800 = m800;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m800;
                let m793 = v0;
                match m793 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>(Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(f13(v1)))))
                    }
                    Adt4::C1(r799) => {
                        let v4 = (*r799).clone();
                        let m794 = v3;
                        match m794.uncons() {
                            None => {
                                Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>(Ok(Adt8::C0(std::rc::Rc::new(v2), std::rc::Rc::new(v1))))
                            }
                            Some((v5, v6)) => {
                                let v7: Adt8 = f107(v4.clone(), v1, v2, {
                                    let (h795, _) = v5;
                                    h795
                                }, Adt0::C3({
                                    let (_, h796) = v5;
                                    h796
                                }), List::<Bytes>::nil())?;
                                Ok::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>((v4, (match v7.clone() {
                                    Adt8::C0(_, h797) => {
                                        (*h797).clone()
                                    }
                                }, (match v7 {
                                    Adt8::C0(h798, _) => {
                                        (*h798).clone()
                                    }
                                }, v6))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f115(Fn9::F116, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f115(v0: Fn9, p808: List<(Bytes, Bytes)>, p809: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m807 = (p808, p809);
    {
        let mut m807 = m807;
        loop {
            let step = {
                let (v1, v2) = m807;
                let m805 = v2;
                match m805.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c806: Fn9 = v0.clone();
                            c806.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}
