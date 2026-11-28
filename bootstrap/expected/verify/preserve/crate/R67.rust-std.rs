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
    C0(std::rc::Rc<Adt1>, std::rc::Rc<Adt4>),
    C1(Adt5, std::rc::Rc<Adt6>, std::rc::Rc<Adt4>),
    C2(Adt7, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>),
    C3(std::rc::Rc<Adt8>, std::rc::Rc<Adt0>, bool, std::rc::Rc<Adt4>),
    C4(std::rc::Rc<Adt6>, std::rc::Rc<Adt0>, bool, std::rc::Rc<Adt4>),
    C5(std::rc::Rc<Adt10>, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>),
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>),
    C7(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>),
    C8(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>),
    C9(std::rc::Rc<Adt0>),
    C10(Adt16, std::rc::Rc<Adt6>, std::rc::Rc<Adt4>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12,
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14(std::rc::Rc<Adt17>, bool, std::rc::Rc<Adt11>, std::rc::Rc<Adt0>, std::rc::Rc<Adt4>, std::rc::Rc<Adt0>),
    C15(std::rc::Rc<Adt17>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C16(std::rc::Rc<Adt6>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt2, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt3),
    C8,
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Option<Adt2>),
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
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64),
    C1(std::rc::Rc<Adt9>),
    C2(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(std::rc::Rc<Adt11>),
    C1,
    C2(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C3(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12),
    C1(Adt2),
    C2(Adt13, std::rc::Rc<Adt11>),
    C3(Adt14, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C4(Adt15, u64),
    C5,
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
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
pub enum Adt13 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
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
    C0(Adt18),
    C1(std::rc::Rc<Adt6>),
    C2(std::rc::Rc<Adt17>),
    C3(Adt19, std::rc::Rc<Adt17>),
    C4(u64, u64, std::rc::Rc<Adt17>),
    C5,
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C7(Adt3),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt20 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F28,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F30,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F45(u64),
    F54(Bytes),
    F55(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
        match self {
            Fn2::F45(k0) => f45(*k0, p0),
            Fn2::F54(k0) => f54(k0.clone(), p0),
            Fn2::F55(k0) => f55(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F61,
}

impl Fn3 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn3::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F66(Bytes),
    F76(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F66(k0) => f66(k0.clone(), p0),
            Fn4::F76(k0) => f76(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Adt0) -> R<Result<Bytes, Bytes>> {
    Ok(f1(f2(f3(v0), v1, 0u64, 100u64, 0u64)?))
}

pub fn f1(v0: Adt20) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h1) => {
            h1
        }
    } {
        Err::<Bytes, Bytes>({
            let a2 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a2)
        })
    } else {
        Ok::<Bytes, Bytes>(f4(f5(List::cons(match v0.clone() {
            Adt20::C0(_, h3, _, _, _) => {
                h3
            }
        }, match v0 {
            Adt20::C0(h4, _, _, _, _) => {
                h4
            }
        }))))
    }
}

pub fn f2(v0: Adt20, v1: Adt0, v2: u64, v3: u64, v4: u64) -> R<Adt20> {
    let m5 = v1;
    match m5 {
        Adt0::C0(r64, _) => {
            let v5 = (*r64).clone();
            f6(v0, v5)
        }
        Adt0::C1(v7, r65, _) => {
            let v8 = (*r65).clone();
            let m6 = v7;
            match m6 {
                Adt5::C0 => {
                    f7(v0, v8)
                }
                Adt5::C1 => {
                    f8(f7(v0, v8)?, Str::lit(".clone()"))
                }
                Adt5::C2 => {
                    f7(v0, v8)
                }
                Adt5::C3 => {
                    f7(f8(v0, Str::lit("*"))?, v8)
                }
                Adt5::C4 => {
                    f8(f7(f8(v0, Str::lit("(*"))?, v8)?, Str::lit(").clone()"))
                }
            }
        }
        Adt0::C2(v10, r66, _) => {
            let v11 = (*r66).clone();
            let m7 = v10;
            match m7 {
                Adt7::C0 => {
                    let a8 = f9(v11.clone());
                    let a9 = 0u64;
                    if if nat_eq(a8, a9) {
                        true
                    } else {
                        let a10 = f9(v11.clone());
                        let a11 = 1u64;
                        if nat_eq(a10, a11) {
                            true
                        } else {
                            let a12 = f9(v11.clone());
                            let a13 = 3u64;
                            nat_eq(a12, a13)
                        }
                    } {
                        f8(f2(f8(v0, Str::lit("!("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        f2(f8(v0, Str::lit("!"))?, v11, v2, 100u64, 0u64)
                    }
                }
                Adt7::C1 => {
                    f8(f2(f8(v0, Str::lit("std::rc::Rc::new("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C2 => {
                    f8(f2(f8(v0, Str::lit("i128::from("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C3 => {
                    f8(f2(f8(v0, Str::lit("Ok("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
            }
        }
        Adt0::C3(r67, r68, v15, _) => {
            let v13 = (*r67).clone();
            let v14 = (*r68).clone();
            let v17: u64 = f10(v13.clone());
            let v22: Adt20 = {
                let m14 = v13.clone();
                match m14 {
                    Adt8::C0(v18) => {
                        f8(f11(f8(v0, Str::lit("f"))?, v18)?, Str::lit("("))?
                    }
                    Adt8::C1(r15) => {
                        let v19 = (*r15).clone();
                        f8(f12(v0, v19)?, Str::lit("("))?
                    }
                    Adt8::C2(v20, v21) => {
                        f13(v0, v20, v21)?
                    }
                }
            };
            let v23: Adt20 = f2(v22, v14, v2, v17, 0u64)?;
            let v28: Adt20 = {
                let m16 = v13;
                match m16 {
                    Adt8::C0(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C1(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C2(v26, _) => {
                        f14(v23, v26)?
                    }
                }
            };
            if v15 {
                f8(v28, Str::lit("?"))
            } else {
                Ok(v28)
            }
        }
        Adt0::C4(r69, r70, v31, _) => {
            let v29 = (*r69).clone();
            let v30 = (*r70).clone();
            let v33: Adt20 = f8(f2(f8(f7(v0, v29)?, Str::lit(".apply("))?, v30, v2, 100u64, 0u64)?, Str::lit(")"))?;
            if v31 {
                f8(v33, Str::lit("?"))
            } else {
                Ok(v33)
            }
        }
        Adt0::C5(r71, r72, _) => {
            let v34 = (*r71).clone();
            let v35 = (*r72).clone();
            let v37: Adt20 = f15(v0, v34.clone())?;
            let a17 = f16(v34.clone());
            let a18 = 1u64;
            if nat_eq(a17, a18) {
                f8(f2(v37, v35, v2, 100u64, 0u64)?, Str::lit(")"))
            } else {
                let a19 = f16(v34);
                let a20 = 2u64;
                if nat_eq(a19, a20) {
                    if f17(v35.clone()) {
                        f8(f2(f8(v37, Str::lit("("))?, v35, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        Ok(v37)
                    }
                } else {
                    Ok(v37)
                }
            }
        }
        Adt0::C6(r73, r74, _) => {
            let v38 = (*r73).clone();
            let v39 = (*r74).clone();
            f8(f2(f8(f2(f8(v0, Str::lit("("))?, v38, v2, 100u64, 0u64)?, Str::lit(", "))?, v39, v2, 100u64, 0u64)?, Str::lit(")"))
        }
        Adt0::C7(r75, r76, r77, _) => {
            let v41 = (*r75).clone();
            let v42 = (*r76).clone();
            let v43 = (*r77).clone();
            let v45: Adt20 = f2(f8(f2(f8(v0, Str::lit("if "))?, v41, v2, 100u64, 0u64)?, Str::lit(" "))?, v42, v2, 100u64, 0u64)?;
            if f18(v43.clone()) {
                Ok(v45)
            } else {
                f2(f8(v45, Str::lit(" else "))?, v43, v2, 100u64, 0u64)
            }
        }
        Adt0::C8(r78, r79, _) => {
            let v46 = (*r78).clone();
            let v47 = (*r79).clone();
            let a21 = f19(v47.clone());
            let a22 = 15u64;
            if nat_eq(a21, a22) {
                if f20(v46.clone()) {
                    let v49: Adt20 = f2(f8(f2(f8(v0, Str::lit("{ let value = "))?, v46, v2, 100u64, 0u64)?, Str::lit("; match value {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v49, v2)?, Str::lit("} }"))
                } else {
                    let v50: Adt20 = f2(f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v50, v2)?, Str::lit("}"))
                }
            } else {
                f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {}"))
            }
        }
        Adt0::C9(r80) => {
            let v51 = (*r80).clone();
            f2(v0, v51, v2, v3, v4)
        }
        Adt0::C10(v52, r81, _) => {
            let v53 = (*r81).clone();
            let m23 = v52;
            match m23 {
                Adt16::C0 => {
                    f8(f7(v0, v53)?, Str::lit(" == 0"))
                }
                Adt16::C1 => {
                    f8(f7(v0, v53)?, Str::lit(" != 0"))
                }
                Adt16::C2 => {
                    f8(f7(v0, v53)?, Str::lit(".uncons()"))
                }
                Adt16::C3 => {
                    f8(f7(v0, v53)?, Str::lit(" - 1"))
                }
            }
        }
        Adt0::C11(r82, r83) => {
            let v55 = (*r82).clone();
            let v56 = (*r83).clone();
            let a24 = f19(v55.clone());
            let a25 = 12u64;
            if if nat_eq(a24, a25) {
                f22(v56.clone())
            } else {
                false
            } {
                f8(v0, Str::lit("{}"))
            } else {
                let v57: Adt20 = f2(f8(v0, Str::lit("{\u{a}"))?, v55, v2, 100u64, 0u64)?;
                let v58: Adt20 = if f22(v56.clone()) {
                    v57
                } else {
                    f8(f2(f21(v57, {
                        let a26 = v2;
                        let a27 = 1u64;
                        nat_add(a26, a27)?
                    })?, v56, {
                        let a28 = v2;
                        let a29 = 1u64;
                        nat_add(a28, a29)?
                    }, 100u64, 0u64)?, Str::lit("\u{a}"))?
                };
                f8(f21(v58, v2)?, Str::lit("}"))
            }
        }
        Adt0::C12 => {
            Ok(v0)
        }
        Adt0::C13(r84, r85) => {
            let v59 = (*r84).clone();
            let v60 = (*r85).clone();
            let a30 = v4;
            let a31 = f23(v3);
            if nat_lt(a30, a31) {
                let v61: Adt20 = f2(v0, v59, v2, 100u64, 0u64)?;
                if f17(v60.clone()) {
                    let a34 = {
                        let a32 = v4;
                        let a33 = 1u64;
                        nat_add(a32, a33)?
                    };
                    let a35 = f23(v3);
                    if nat_lt(a34, a35) {
                        f2(f24(v61, v3, v4)?, v60, v2, v3, {
                            let a36 = v4;
                            let a37 = 1u64;
                            nat_add(a36, a37)?
                        })
                    } else {
                        Ok(v61)
                    }
                } else {
                    Ok(v61)
                }
            } else {
                Ok(v0)
            }
        }
        Adt0::C14(r86, v63, r87, r88, _, r89) => {
            let v62 = (*r86).clone();
            let v64 = (*r87).clone();
            let v65 = (*r88).clone();
            let v67 = (*r89).clone();
            let v68: Adt20 = f25(f8(f21(v0, {
                let a38 = v2;
                let a39 = 1u64;
                nat_add(a38, a39)?
            })?, Str::lit("let "))?, v62)?;
            let v69: Adt20 = if v63 {
                f26(f8(v68, Str::lit(": "))?, v64)?
            } else {
                v68
            };
            let v70: Adt20 = f8(f2(f8(v69, Str::lit(" = "))?, v65, {
                let a40 = v2;
                let a41 = 1u64;
                nat_add(a40, a41)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            f2(v70, v67, v2, 100u64, 0u64)
        }
        Adt0::C15(r90, r91, r92) => {
            let v71 = (*r90).clone();
            let v72 = (*r91).clone();
            let v73 = (*r92).clone();
            let v74: Adt20 = f8(f2(f8(f25(f21(v0, {
                let a42 = v2;
                let a43 = 1u64;
                nat_add(a42, a43)?
            })?, v71)?, Str::lit(" => "))?, v72, {
                let a44 = v2;
                let a45 = 1u64;
                nat_add(a44, a45)?
            }, 100u64, 0u64)?, Str::lit("\u{a}"))?;
            f2(v74, v73, v2, 100u64, 0u64)
        }
        Adt0::C16(r93, r94) => {
            let v75 = (*r93).clone();
            let v76 = (*r94).clone();
            let v77: Adt20 = f7(f8(f21(f8(v0, Str::lit("{\u{a}"))?, {
                let a46 = v2;
                let a47 = 1u64;
                nat_add(a46, a47)?
            })?, Str::lit("let mut "))?, v75.clone())?;
            let v78: Adt20 = f8(f7(f8(v77, Str::lit(" = "))?, v75.clone())?, Str::lit(";\u{a}"))?;
            let v79: Adt20 = f8(f21(v78, {
                let a48 = v2;
                let a49 = 1u64;
                nat_add(a48, a49)?
            })?, Str::lit("loop {\u{a}"))?;
            let v80: Adt20 = f8(f2(f8(f21(v79, {
                let a50 = v2;
                let a51 = 2u64;
                nat_add(a50, a51)?
            })?, Str::lit("let step = "))?, v76, {
                let a52 = v2;
                let a53 = 2u64;
                nat_add(a52, a53)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            let v81: Adt20 = f8(f21(v80, {
                let a54 = v2;
                let a55 = 2u64;
                nat_add(a54, a55)?
            })?, Str::lit("match step {\u{a}"))?;
            let v82: Adt20 = f8(f7(f8(f21(v81, {
                let a56 = v2;
                let a57 = 3u64;
                nat_add(a56, a57)?
            })?, Str::lit("Ok(next) => { "))?, v75)?, Str::lit(" = next; }\u{a}"))?;
            let v83: Adt20 = f8(f21(v82, {
                let a58 = v2;
                let a59 = 3u64;
                nat_add(a58, a59)?
            })?, Str::lit("Err(done) => break done,\u{a}"))?;
            f8(f21(f8(f21(f8(f21(v83, {
                let a60 = v2;
                let a61 = 2u64;
                nat_add(a60, a61)?
            })?, Str::lit("}\u{a}"))?, {
                let a62 = v2;
                let a63 = 1u64;
                nat_add(a62, a63)?
            })?, Str::lit("}\u{a}"))?, v2)?, Str::lit("}"))
        }
    }
}

pub fn f3(v0: u64) -> Adt20 {
    Adt20::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f4(v0: List<Bytes>) -> Bytes {
    let m97 = {
        let (h95, _) = f27(Fn0::F28, {
            let a96 = v0.clone();
            length_list(a96)
        }, v0);
        h95
    };
    match m97.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f5(v0: List<Bytes>) -> List<Bytes> {
    f29(Fn1::F30, List::<Bytes>::nil(), v0)
}

pub fn f6(v0: Adt20, v1: Adt1) -> R<Adt20> {
    let m98 = v1;
    match m98 {
        Adt1::C0 => {
            f8(v0, Str::lit("()"))
        }
        Adt1::C1(v2) => {
            if v2 {
                f8(v0, Str::lit("true"))
            } else {
                f8(v0, Str::lit("false"))
            }
        }
        Adt1::C2(v3) => {
            f8(f11(v0, v3)?, Str::lit("u64"))
        }
        Adt1::C3(v4, v5) => {
            if if v4 {
                let a99 = v5;
                let a100 = 9223372036854775808u64;
                nat_eq(a99, a100)
            } else {
                false
            } {
                f8(v0, Str::lit("i64::MIN"))
            } else {
                f8(f11(if v4 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v5)?, Str::lit("i64"))
            }
        }
        Adt1::C4(v6, v7, v8) => {
            if if v7 {
                let a101 = v8;
                let a102 = f31(v6.clone());
                nat_eq(a101, a102)
            } else {
                false
            } {
                f8(f32(v0, v6)?, Str::lit("::MIN"))
            } else {
                f32(f11(if v7 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v8)?, v6)
            }
        }
        Adt1::C5(v9) => {
            f8(f33(f8(v0, Str::lit("Str::lit("))?, v9)?, Str::lit(")"))
        }
        Adt1::C6(v10) => {
            f34(v0, v10)
        }
        Adt1::C7(v11) => {
            f35(v0, v11)
        }
        Adt1::C8 => {
            f8(v0, Str::lit("Overflow"))
        }
    }
}

pub fn f7(v0: Adt20, v1: Adt6) -> R<Adt20> {
    let m103 = v1;
    match m103 {
        Adt6::C0(v2) => {
            f11(f8(v0, Str::lit("v"))?, v2)
        }
        Adt6::C1(v3) => {
            f11(f8(v0, Str::lit("m"))?, v3)
        }
        Adt6::C2(v4) => {
            f11(f8(v0, Str::lit("a"))?, v4)
        }
        Adt6::C3(v5) => {
            f11(f8(v0, Str::lit("c"))?, v5)
        }
        Adt6::C4(v6) => {
            f11(f8(v0, Str::lit("r"))?, v6)
        }
        Adt6::C5(v7) => {
            f11(f8(v0, Str::lit("h"))?, v7)
        }
        Adt6::C6(v8) => {
            f11(f8(v0, Str::lit("k"))?, v8)
        }
        Adt6::C7(v9) => {
            f11(f8(v0, Str::lit("p"))?, v9)
        }
        Adt6::C8(v10) => {
            f11(f8(v0, Str::lit("f"))?, v10)
        }
        Adt6::C9(v11) => {
            f36(v0, v11)
        }
    }
}

pub fn f8(v0: Adt20, v1: Str) -> R<Adt20> {
    f36(v0, {
        let a104 = v1;
        utf8_encode(a104)
    })
}

pub fn f9(v0: Adt0) -> u64 {
    let m105 = v0;
    match m105 {
        Adt0::C0(_, _) => {
            1000u64
        }
        Adt0::C1(_, _, _) => {
            1000u64
        }
        Adt0::C2(_, _, _) => {
            1000u64
        }
        Adt0::C3(_, _, _, _) => {
            1000u64
        }
        Adt0::C4(_, _, _, _) => {
            1000u64
        }
        Adt0::C5(_, _, _) => {
            1000u64
        }
        Adt0::C6(_, _, _) => {
            1000u64
        }
        Adt0::C7(_, _, _, _) => {
            1000u64
        }
        Adt0::C8(_, _, _) => {
            1000u64
        }
        Adt0::C9(_) => {
            1000u64
        }
        Adt0::C10(v31, _, _) => {
            let m106 = v31;
            match m106 {
                Adt16::C0 => {
                    0u64
                }
                Adt16::C1 => {
                    1u64
                }
                Adt16::C2 => {
                    2u64
                }
                Adt16::C3 => {
                    3u64
                }
            }
        }
        Adt0::C11(_, _) => {
            1000u64
        }
        Adt0::C12 => {
            1000u64
        }
        Adt0::C13(_, _) => {
            1000u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            1000u64
        }
        Adt0::C15(_, _, _) => {
            1000u64
        }
        Adt0::C16(_, _) => {
            1000u64
        }
    }
}

pub fn f10(v0: Adt8) -> u64 {
    let m107 = v0;
    match m107 {
        Adt8::C0(_) => {
            1000u64
        }
        Adt8::C1(_) => {
            1000u64
        }
        Adt8::C2(v3, _) => {
            v3
        }
    }
}

pub fn f11(v0: Adt20, v1: u64) -> R<Adt20> {
    f37(v0, v1)
}

pub fn f12(v0: Adt20, v1: Adt9) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h108, _) => {
            h108
        }
    };
    let m110 = match v1 {
        Adt9::C0(_, h109) => {
            h109
        }
    };
    match m110 {
        None => {
            f8(v0, {
                let a117 = v2;
                let a118 = 20u64;
                if nat_lt(a117, a118) {
                    f39(v2)
                } else {
                    f40(v2)
                }
            })
        }
        Some(v3) => {
            let a111 = v2;
            let a112 = 24u64;
            if nat_eq(a111, a112) {
                f32(f8(v0, Str::lit("checked_neg_"))?, v3)
            } else {
                let a113 = v2;
                let a114 = 47u64;
                if nat_eq(a113, a114) {
                    f32(f8(v0, Str::lit("format_"))?, v3)
                } else {
                    let a115 = v2;
                    let a116 = 49u64;
                    if nat_eq(a115, a116) {
                        f32(f8(v0, Str::lit("parse_"))?, v3)
                    } else {
                        f8(f8(f32(f8(v0, Str::lit("fixed_"))?, v3)?, Str::lit("::"))?, f38(v2))
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Adt20, v1: u64, v2: Option<Adt2>) -> R<Adt20> {
    let a119 = v1;
    let a120 = 8u64;
    if nat_lt(a119, a120) {
        let m121 = v2;
        match m121 {
            None => {
                Ok(v0)
            }
            Some(v3) => {
                f8(f8(f8(f32(v0, v3)?, Str::lit("::"))?, {
                    let a122 = v1;
                    let a123 = 7u64;
                    if nat_eq(a122, a123) {
                        Str::lit("try_from")
                    } else {
                        f41(v1)
                    }
                })?, Str::lit("("))
            }
        }
    } else {
        let a124 = v1;
        let a125 = 40u64;
        if nat_eq(a124, a125) {
            let m126 = v2;
            match m126 {
                None => {
                    Ok(v0)
                }
                Some(v4) => {
                    f8(f32(f8(v0, Str::lit("Str::format_"))?, v4)?, Str::lit("("))
                }
            }
        } else {
            let a127 = v1;
            let a128 = 41u64;
            if nat_eq(a127, a128) {
                let m129 = v2;
                match m129 {
                    None => {
                        Ok(v0)
                    }
                    Some(v5) => {
                        f8(f32(f8(v0, Str::lit("Str::parse_"))?, v5)?, Str::lit("(&"))
                    }
                }
            } else {
                f8(v0, f42(v1))
            }
        }
    }
}

pub fn f14(v0: Adt20, v1: u64) -> R<Adt20> {
    let a130 = v1;
    let a131 = 8u64;
    if nat_lt(a130, a131) {
        f8(v0, {
            let a132 = v1;
            let a133 = 7u64;
            if nat_eq(a132, a133) {
                Str::lit(").ok()")
            } else {
                Str::lit(")")
            }
        })
    } else {
        f8(v0, f43(v1))
    }
}

pub fn f15(v0: Adt20, v1: Adt10) -> R<Adt20> {
    let m134 = v1;
    match m134 {
        Adt10::C0(r135) => {
            let v2 = (*r135).clone();
            f8(f26(f8(v0, Str::lit("None::<"))?, v2)?, Str::lit(">"))
        }
        Adt10::C1 => {
            f8(v0, Str::lit("Some("))
        }
        Adt10::C2(r136, r137) => {
            let v3 = (*r136).clone();
            let v4 = (*r137).clone();
            f8(f26(f8(f26(f8(v0, Str::lit("Ok::<"))?, v3)?, Str::lit(", "))?, v4)?, Str::lit(">("))
        }
        Adt10::C3(r138, r139) => {
            let v5 = (*r138).clone();
            let v6 = (*r139).clone();
            f8(f26(f8(f26(f8(v0, Str::lit("Err::<"))?, v5)?, Str::lit(", "))?, v6)?, Str::lit(">("))
        }
        Adt10::C4(v7, v8) => {
            f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)
        }
        Adt10::C5(v9, v10) => {
            f11(f8(f11(f8(v0, Str::lit("Fn"))?, v9)?, Str::lit("::F"))?, v10)
        }
        Adt10::C6 => {
            f8(v0, Str::lit("List::cons("))
        }
        Adt10::C7(r140) => {
            let v11 = (*r140).clone();
            f8(f26(f8(v0, Str::lit("List::<"))?, v11)?, Str::lit(">::nil()"))
        }
    }
}

pub fn f16(v0: Adt10) -> u64 {
    let m141 = v0;
    match m141 {
        Adt10::C0(_) => {
            0u64
        }
        Adt10::C1 => {
            1u64
        }
        Adt10::C2(_, _) => {
            1u64
        }
        Adt10::C3(_, _) => {
            1u64
        }
        Adt10::C4(_, _) => {
            2u64
        }
        Adt10::C5(_, _) => {
            2u64
        }
        Adt10::C6 => {
            1u64
        }
        Adt10::C7(_) => {
            0u64
        }
    }
}

pub fn f17(v0: Adt0) -> bool {
    let a142 = f19(v0);
    let a143 = 13u64;
    nat_eq(a142, a143)
}

pub fn f18(v0: Adt0) -> bool {
    let m144 = v0;
    match m144 {
        Adt0::C0(_, _) => {
            false
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(r147, r148) => {
            let v34 = (*r147).clone();
            let v35 = (*r148).clone();
            let a145 = f19(v34);
            let a146 = 12u64;
            if nat_eq(a145, a146) {
                f22(v35)
            } else {
                false
            }
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f19(v0: Adt0) -> u64 {
    let m149 = v0;
    match m149 {
        Adt0::C0(_, _) => {
            0u64
        }
        Adt0::C1(_, _, _) => {
            1u64
        }
        Adt0::C2(_, _, _) => {
            2u64
        }
        Adt0::C3(_, _, _, _) => {
            3u64
        }
        Adt0::C4(_, _, _, _) => {
            4u64
        }
        Adt0::C5(_, _, _) => {
            5u64
        }
        Adt0::C6(_, _, _) => {
            6u64
        }
        Adt0::C7(_, _, _, _) => {
            7u64
        }
        Adt0::C8(_, _, _) => {
            8u64
        }
        Adt0::C9(_) => {
            9u64
        }
        Adt0::C10(_, _, _) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12 => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            14u64
        }
        Adt0::C15(_, _, _) => {
            15u64
        }
        Adt0::C16(_, _) => {
            16u64
        }
    }
}

pub fn f20(p161: Adt0) -> bool {
    let m160 = p161;
    {
        let mut m160 = m160;
        loop {
            let step = {
                let v0 = m160;
                let m150 = v0;
                match m150 {
                    Adt0::C0(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3(r157, _, _, _) => {
                        let v9 = (*r157).clone();
                        let a151 = f10(v9.clone());
                        let a152 = 16u64;
                        if nat_eq(a151, a152) {
                            Err::<Adt0, bool>(true)
                        } else {
                            let a153 = f10(v9);
                            let a154 = 25u64;
                            Err::<Adt0, bool>(nat_eq(a153, a154))
                        }
                    }
                    Adt0::C4(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(r158, r159, _) => {
                        let v27 = (*r158).clone();
                        let v28 = (*r159).clone();
                        let a155 = f19(v28);
                        let a156 = 15u64;
                        if nat_eq(a155, a156) {
                            Ok::<Adt0, bool>(v27)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                    Adt0::C9(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14(_, _, _, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C16(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt20, v1: u64) -> R<Adt20> {
    let (_, h162) = {
        let (h163, _) = f44(Fn2::F45(v1), v1, (0u64, v0))?;
        h163
    };
    Ok(h162)
}

pub fn f22(v0: Adt0) -> bool {
    let m164 = v0;
    match m164 {
        Adt0::C0(r166, _) => {
            let v1 = (*r166).clone();
            let m165 = v1;
            match m165 {
                Adt1::C0 => {
                    true
                }
                Adt1::C1(_) => {
                    false
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
                Adt1::C5(_) => {
                    false
                }
                Adt1::C6(_) => {
                    false
                }
                Adt1::C7(_) => {
                    false
                }
                Adt1::C8 => {
                    false
                }
            }
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(_, _) => {
            false
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    let a167 = v0;
    let a168 = 8u64;
    if if nat_lt(a167, a168) {
        true
    } else {
        let a169 = 44u64;
        let a170 = v0;
        nat_le(a169, a170)
    } {
        100000u64
    } else {
        let a171 = v0;
        let a172 = 11u64;
        if nat_eq(a171, a172) {
            1u64
        } else {
            let a173 = v0;
            let a174 = 24u64;
            if if nat_eq(a173, a174) {
                true
            } else {
                let a175 = v0;
                let a176 = 26u64;
                if nat_eq(a175, a176) {
                    true
                } else {
                    let a177 = v0;
                    let a178 = 31u64;
                    if nat_eq(a177, a178) {
                        true
                    } else {
                        let a179 = v0;
                        let a180 = 32u64;
                        if nat_eq(a179, a180) {
                            true
                        } else {
                            let a181 = v0;
                            let a182 = 35u64;
                            if nat_eq(a181, a182) {
                                true
                            } else {
                                let a183 = v0;
                                let a184 = 36u64;
                                if nat_eq(a183, a184) {
                                    true
                                } else {
                                    let a185 = v0;
                                    let a186 = 40u64;
                                    if nat_eq(a185, a186) {
                                        true
                                    } else {
                                        let a187 = v0;
                                        let a188 = 41u64;
                                        if nat_eq(a187, a188) {
                                            true
                                        } else {
                                            let a189 = v0;
                                            let a190 = 42u64;
                                            if nat_eq(a189, a190) {
                                                true
                                            } else {
                                                let a191 = v0;
                                                let a192 = 43u64;
                                                nat_eq(a191, a192)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                1u64
            } else {
                let a193 = v0;
                let a194 = 15u64;
                if nat_eq(a193, a194) {
                    3u64
                } else {
                    let a195 = v0;
                    let a196 = 16u64;
                    if nat_eq(a195, a196) {
                        3u64
                    } else {
                        let a197 = v0;
                        let a198 = 20u64;
                        if nat_eq(a197, a198) {
                            3u64
                        } else {
                            let a199 = v0;
                            let a200 = 25u64;
                            if if nat_eq(a199, a200) {
                                true
                            } else {
                                let a201 = v0;
                                let a202 = 29u64;
                                if nat_eq(a201, a202) {
                                    true
                                } else {
                                    let a203 = v0;
                                    let a204 = 34u64;
                                    if nat_eq(a203, a204) {
                                        true
                                    } else {
                                        let a205 = v0;
                                        let a206 = 38u64;
                                        nat_eq(a205, a206)
                                    }
                                }
                            } {
                                3u64
                            } else {
                                2u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt20, v1: u64, v2: u64) -> R<Adt20> {
    let a207 = v1;
    let a208 = 8u64;
    if nat_lt(a207, a208) {
        f8(v0, Str::lit(", "))
    } else {
        let a209 = v1;
        let a210 = 100u64;
        if nat_eq(a209, a210) {
            f8(v0, Str::lit(", "))
        } else {
            let a211 = v2;
            let a212 = 0u64;
            if nat_eq(a211, a212) {
                f8(v0, f46(v1))
            } else {
                f8(v0, f47(v1))
            }
        }
    }
}

pub fn f25(v0: Adt20, v1: Adt17) -> R<Adt20> {
    let m213 = v1;
    match m213 {
        Adt17::C0(v2) => {
            f8(v0, {
                let m214 = v2;
                match m214 {
                    Adt18::C0 => {
                        Str::lit("_")
                    }
                    Adt18::C1 => {
                        Str::lit("()")
                    }
                    Adt18::C2 => {
                        Str::lit("None")
                    }
                }
            })
        }
        Adt17::C1(r218) => {
            let v3 = (*r218).clone();
            f7(v0, v3)
        }
        Adt17::C2(r219) => {
            let v4 = (*r219).clone();
            f8(f25(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt17::C3(v5, r220) => {
            let v6 = (*r220).clone();
            f8(f25(f8(v0, {
                let m215 = v5;
                match m215 {
                    Adt19::C0 => {
                        Str::lit("Some(")
                    }
                    Adt19::C1 => {
                        Str::lit("Ok(")
                    }
                    Adt19::C2 => {
                        Str::lit("Err(")
                    }
                }
            })?, v6)?, Str::lit(")"))
        }
        Adt17::C4(v7, v8, r221) => {
            let v9 = (*r221).clone();
            let v10: Adt20 = f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m216 = v9.clone();
            match m216 {
                Adt17::C0(_) => {
                    Ok(v10)
                }
                Adt17::C1(_) => {
                    Ok(v10)
                }
                Adt17::C2(_) => {
                    Ok(v10)
                }
                Adt17::C3(_, _) => {
                    Ok(v10)
                }
                Adt17::C4(_, _, _) => {
                    Ok(v10)
                }
                Adt17::C5 => {
                    Ok(v10)
                }
                Adt17::C6(_, _) => {
                    f8(f25(f8(v10, Str::lit("("))?, v9)?, Str::lit(")"))
                }
                Adt17::C7(_) => {
                    Ok(v10)
                }
            }
        }
        Adt17::C5 => {
            Ok(v0)
        }
        Adt17::C6(r222, r223) => {
            let v22 = (*r222).clone();
            let v23 = (*r223).clone();
            let m217 = v23.clone();
            match m217 {
                Adt17::C0(_) => {
                    f25(v0, v22)
                }
                Adt17::C1(_) => {
                    f25(v0, v22)
                }
                Adt17::C2(_) => {
                    f25(v0, v22)
                }
                Adt17::C3(_, _) => {
                    f25(v0, v22)
                }
                Adt17::C4(_, _, _) => {
                    f25(v0, v22)
                }
                Adt17::C5 => {
                    f25(v0, v22)
                }
                Adt17::C6(_, _) => {
                    f25(f8(f25(v0, v22)?, Str::lit(", "))?, v23)
                }
                Adt17::C7(_) => {
                    f25(v0, v22)
                }
            }
        }
        Adt17::C7(v35) => {
            f35(v0, v35)
        }
    }
}

pub fn f26(v0: Adt20, v1: Adt11) -> R<Adt20> {
    let m224 = v1;
    match m224 {
        Adt11::C0(v2) => {
            f48(v0, v2)
        }
        Adt11::C1(v3) => {
            f32(v0, v3)
        }
        Adt11::C2(v4, r228) => {
            let v5 = (*r228).clone();
            f8(f26(f8(v0, f49(v4.clone()))?, v5)?, f50(v4))
        }
        Adt11::C3(v6, r229, r230) => {
            let v7 = (*r229).clone();
            let v8 = (*r230).clone();
            let m225 = v6;
            match m225 {
                Adt14::C0 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt14::C1 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt11::C4(v9, v10) => {
            let m226 = v9;
            match m226 {
                Adt15::C0 => {
                    f11(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt15::C1 => {
                    f11(f8(v0, Str::lit("Fn"))?, v10)
                }
                Adt15::C2 => {
                    f51(v0, v10)
                }
            }
        }
        Adt11::C5 => {
            Ok(v0)
        }
        Adt11::C6(r231, r232) => {
            let v11 = (*r231).clone();
            let v12 = (*r232).clone();
            let m227 = v12.clone();
            match m227 {
                Adt11::C0(_) => {
                    f26(v0, v11)
                }
                Adt11::C1(_) => {
                    f26(v0, v11)
                }
                Adt11::C2(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C3(_, _, _) => {
                    f26(v0, v11)
                }
                Adt11::C4(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C5 => {
                    f26(v0, v11)
                }
                Adt11::C6(_, _) => {
                    f26(f8(f26(v0, v11)?, Str::lit(", "))?, v12)
                }
            }
        }
    }
}

pub fn f27(v0: Fn0, p237: u64, p238: List<Bytes>) -> (List<Bytes>, bool) {
    let m236 = (p237, p238);
    {
        let mut m236 = m236;
        loop {
            let step = {
                let (v1, v2) = m236;
                let m233 = v1;
                if m233 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m233 - 1;
                    let m235 = {
                        let c234: Fn0 = v0.clone();
                        c234.apply(v2.clone())
                    };
                    match m235 {
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
                Ok(next) => { m236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>) -> Option<List<Bytes>> {
    f52(v0)
}

pub fn f29(v0: Fn1, p242: List<Bytes>, p243: List<Bytes>) -> List<Bytes> {
    let m241 = (p242, p243);
    {
        let mut m241 = m241;
        loop {
            let step = {
                let (v1, v2) = m241;
                let m239 = v2;
                match m239.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c240: Fn1 = v0.clone();
                            c240.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt2) -> u64 {
    let m244 = v0;
    match m244 {
        Adt2::C0 => {
            0u64
        }
        Adt2::C1 => {
            0u64
        }
        Adt2::C2 => {
            0u64
        }
        Adt2::C3 => {
            0u64
        }
        Adt2::C4 => {
            128u64
        }
        Adt2::C5 => {
            32768u64
        }
        Adt2::C6 => {
            2147483648u64
        }
        Adt2::C7 => {
            9223372036854775808u64
        }
    }
}

pub fn f32(v0: Adt20, v1: Adt2) -> R<Adt20> {
    f8(v0, f53(v1))
}

pub fn f33(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h245) = {
            let (h246, _) = f44(Fn2::F54(v1.clone()), {
                let a247 = v1;
                length_bytes(a247)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h246
        };
        h245
    }, Str::lit("\""))
}

pub fn f34(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h248) = {
            let (h249, _) = f44(Fn2::F55(v1.clone()), {
                let a250 = v1;
                length_bytes(a250)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h249
        };
        h248
    }, Str::lit("])"))
}

pub fn f35(v0: Adt20, v1: Adt3) -> R<Adt20> {
    f8(v0, {
        let m251 = v1;
        match m251 {
            Adt3::C0 => {
                Str::lit("Ordering::Less")
            }
            Adt3::C1 => {
                Str::lit("Ordering::Equal")
            }
            Adt3::C2 => {
                Str::lit("Ordering::Greater")
            }
        }
    })
}

pub fn f36(v0: Adt20, v1: Bytes) -> R<Adt20> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h252) => {
            h252
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a255 = match v0.clone() {
                Adt20::C0(_, _, h253, _, _) => {
                    h253
                }
            };
            let a256 = {
                let a254 = v1.clone();
                length_bytes(a254)
            };
            nat_add(a255, a256)?
        };
        let a258 = v2;
        let a259 = match v0.clone() {
            Adt20::C0(_, _, _, h257, _) => {
                h257
            }
        };
        if nat_le(a258, a259) {
            Ok(f56(v0.clone(), {
                let a261 = match v0 {
                    Adt20::C0(_, h260, _, _, _) => {
                        h260
                    }
                };
                let a262 = v1;
                append_bytes(a261, a262)
            }, v2))
        } else {
            Ok(f57(v0))
        }
    }
}

pub fn f37(v0: Adt20, v1: u64) -> R<Adt20> {
    f36(v0, f58(v1)?)
}

pub fn f38(v0: u64) -> Str {
    let a263 = v0;
    let a264 = 20u64;
    if nat_eq(a263, a264) {
        Str::lit("checked_add")
    } else {
        let a265 = v0;
        let a266 = 21u64;
        if nat_eq(a265, a266) {
            Str::lit("checked_sub")
        } else {
            let a267 = v0;
            let a268 = 22u64;
            if nat_eq(a267, a268) {
                Str::lit("checked_mul")
            } else {
                let a269 = v0;
                let a270 = 23u64;
                if nat_eq(a269, a270) {
                    Str::lit("checked_quot")
                } else {
                    let a271 = v0;
                    let a272 = 25u64;
                    if nat_eq(a271, a272) {
                        Str::lit("bit_and")
                    } else {
                        let a273 = v0;
                        let a274 = 26u64;
                        if nat_eq(a273, a274) {
                            Str::lit("bit_or")
                        } else {
                            let a275 = v0;
                            let a276 = 27u64;
                            if nat_eq(a275, a276) {
                                Str::lit("bit_xor")
                            } else {
                                let a277 = v0;
                                let a278 = 28u64;
                                if nat_eq(a277, a278) {
                                    Str::lit("bit_not")
                                } else {
                                    let a279 = v0;
                                    let a280 = 29u64;
                                    if nat_eq(a279, a280) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a281 = v0;
                                        let a282 = 30u64;
                                        if nat_eq(a281, a282) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a283 = v0;
                                            let a284 = 31u64;
                                            if nat_eq(a283, a284) {
                                                Str::lit("convert")
                                            } else {
                                                Str::lit("")
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

pub fn f39(v0: u64) -> Str {
    let a285 = v0;
    let a286 = 0u64;
    if nat_eq(a285, a286) {
        Str::lit("nat_add")
    } else {
        let a287 = v0;
        let a288 = 1u64;
        if nat_eq(a287, a288) {
            Str::lit("nat_sub")
        } else {
            let a289 = v0;
            let a290 = 2u64;
            if nat_eq(a289, a290) {
                Str::lit("nat_mul")
            } else {
                let a291 = v0;
                let a292 = 3u64;
                if nat_eq(a291, a292) {
                    Str::lit("nat_quot")
                } else {
                    let a293 = v0;
                    let a294 = 4u64;
                    if nat_eq(a293, a294) {
                        Str::lit("nat_rem")
                    } else {
                        let a295 = v0;
                        let a296 = 5u64;
                        if nat_eq(a295, a296) {
                            Str::lit("nat_eq")
                        } else {
                            let a297 = v0;
                            let a298 = 6u64;
                            if nat_eq(a297, a298) {
                                Str::lit("nat_le")
                            } else {
                                let a299 = v0;
                                let a300 = 7u64;
                                if nat_eq(a299, a300) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a301 = v0;
                                    let a302 = 8u64;
                                    if nat_eq(a301, a302) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a303 = v0;
                                        let a304 = 9u64;
                                        if nat_eq(a303, a304) {
                                            Str::lit("int_add")
                                        } else {
                                            let a305 = v0;
                                            let a306 = 10u64;
                                            if nat_eq(a305, a306) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a307 = v0;
                                                let a308 = 11u64;
                                                if nat_eq(a307, a308) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a309 = v0;
                                                    let a310 = 12u64;
                                                    if nat_eq(a309, a310) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a311 = v0;
                                                        let a312 = 13u64;
                                                        if nat_eq(a311, a312) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a313 = v0;
                                                            let a314 = 14u64;
                                                            if nat_eq(a313, a314) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a315 = v0;
                                                                let a316 = 15u64;
                                                                if nat_eq(a315, a316) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a317 = v0;
                                                                    let a318 = 16u64;
                                                                    if nat_eq(a317, a318) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a319 = v0;
                                                                        let a320 = 17u64;
                                                                        if nat_eq(a319, a320) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a321 = v0;
                                                                            let a322 = 18u64;
                                                                            if nat_eq(a321, a322) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a323 = v0;
                                                                                let a324 = 19u64;
                                                                                if nat_eq(a323, a324) {
                                                                                    Str::lit("compare")
                                                                                } else {
                                                                                    Str::lit("")
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

pub fn f40(v0: u64) -> Str {
    let a325 = v0;
    let a326 = 32u64;
    if nat_eq(a325, a326) {
        Str::lit("append_list")
    } else {
        let a327 = v0;
        let a328 = 33u64;
        if nat_eq(a327, a328) {
            Str::lit("append_bytes")
        } else {
            let a329 = v0;
            let a330 = 34u64;
            if nat_eq(a329, a330) {
                Str::lit("length_list")
            } else {
                let a331 = v0;
                let a332 = 35u64;
                if nat_eq(a331, a332) {
                    Str::lit("length_bytes")
                } else {
                    let a333 = v0;
                    let a334 = 36u64;
                    if nat_eq(a333, a334) {
                        Str::lit("length_string")
                    } else {
                        let a335 = v0;
                        let a336 = 37u64;
                        if nat_eq(a335, a336) {
                            Str::lit("index_list")
                        } else {
                            let a337 = v0;
                            let a338 = 38u64;
                            if nat_eq(a337, a338) {
                                Str::lit("index_bytes")
                            } else {
                                let a339 = v0;
                                let a340 = 39u64;
                                if nat_eq(a339, a340) {
                                    Str::lit("slice_list")
                                } else {
                                    let a341 = v0;
                                    let a342 = 40u64;
                                    if nat_eq(a341, a342) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a343 = v0;
                                        let a344 = 41u64;
                                        if nat_eq(a343, a344) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a345 = v0;
                                            let a346 = 42u64;
                                            if nat_eq(a345, a346) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a347 = v0;
                                                let a348 = 43u64;
                                                if nat_eq(a347, a348) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a349 = v0;
                                                    let a350 = 44u64;
                                                    if nat_eq(a349, a350) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a351 = v0;
                                                        let a352 = 45u64;
                                                        if nat_eq(a351, a352) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a353 = v0;
                                                            let a354 = 46u64;
                                                            if nat_eq(a353, a354) {
                                                                Str::lit("format_int")
                                                            } else {
                                                                let a355 = v0;
                                                                let a356 = 48u64;
                                                                if nat_eq(a355, a356) {
                                                                    Str::lit("parse_int")
                                                                } else {
                                                                    Str::lit("")
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

pub fn f41(v0: u64) -> Str {
    let a357 = v0;
    let a358 = 0u64;
    if nat_eq(a357, a358) {
        Str::lit("checked_add")
    } else {
        let a359 = v0;
        let a360 = 1u64;
        if nat_eq(a359, a360) {
            Str::lit("checked_sub")
        } else {
            let a361 = v0;
            let a362 = 2u64;
            if nat_eq(a361, a362) {
                Str::lit("checked_mul")
            } else {
                let a363 = v0;
                let a364 = 3u64;
                if nat_eq(a363, a364) {
                    Str::lit("checked_div")
                } else {
                    let a365 = v0;
                    let a366 = 4u64;
                    if nat_eq(a365, a366) {
                        Str::lit("checked_neg")
                    } else {
                        let a367 = v0;
                        let a368 = 5u64;
                        if nat_eq(a367, a368) {
                            Str::lit("checked_shl")
                        } else {
                            let a369 = v0;
                            let a370 = 6u64;
                            if nat_eq(a369, a370) {
                                Str::lit("checked_shr")
                            } else {
                                Str::lit("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: u64) -> Str {
    let a371 = v0;
    let a372 = 42u64;
    if nat_eq(a371, a372) {
        Str::lit("Str::parse_int(&")
    } else {
        let a373 = v0;
        let a374 = 43u64;
        if nat_eq(a373, a374) {
            Str::lit("Str::format_i64(")
        } else {
            let a375 = v0;
            let a376 = 30u64;
            if nat_eq(a375, a376) {
                Str::lit("Bytes::append(")
            } else {
                let a377 = v0;
                let a378 = 31u64;
                if nat_eq(a377, a378) {
                    Str::lit("Bytes::length(&")
                } else {
                    let a379 = v0;
                    let a380 = 32u64;
                    if nat_eq(a379, a380) {
                        Str::lit("Str::length(&")
                    } else {
                        let a381 = v0;
                        let a382 = 33u64;
                        if nat_eq(a381, a382) {
                            Str::lit("Bytes::index(&")
                        } else {
                            let a383 = v0;
                            let a384 = 34u64;
                            if nat_eq(a383, a384) {
                                Str::lit("Bytes::slice(&")
                            } else {
                                let a385 = v0;
                                let a386 = 35u64;
                                if nat_eq(a385, a386) {
                                    Str::lit("Str::utf8_encode(&")
                                } else {
                                    let a387 = v0;
                                    let a388 = 36u64;
                                    if nat_eq(a387, a388) {
                                        Str::lit("Bytes::utf8_decode(&")
                                    } else {
                                        let a389 = v0;
                                        let a390 = 37u64;
                                        if nat_eq(a389, a390) {
                                            Str::lit("Bytes::compare(&")
                                        } else {
                                            let a391 = v0;
                                            let a392 = 38u64;
                                            if nat_eq(a391, a392) {
                                                Str::lit("Str::split_exact(&")
                                            } else {
                                                let a393 = v0;
                                                let a394 = 39u64;
                                                if nat_eq(a393, a394) {
                                                    Str::lit("Str::join(")
                                                } else {
                                                    let a395 = v0;
                                                    let a396 = 8u64;
                                                    if nat_eq(a395, a396) {
                                                        Str::lit("core::ops::BitAnd::bitand(")
                                                    } else {
                                                        let a397 = v0;
                                                        let a398 = 9u64;
                                                        if nat_eq(a397, a398) {
                                                            Str::lit("core::ops::BitOr::bitor(")
                                                        } else {
                                                            let a399 = v0;
                                                            let a400 = 10u64;
                                                            if nat_eq(a399, a400) {
                                                                Str::lit("core::ops::BitXor::bitxor(")
                                                            } else {
                                                                let a401 = v0;
                                                                let a402 = 11u64;
                                                                if nat_eq(a401, a402) {
                                                                    Str::lit("core::ops::Not::not(")
                                                                } else {
                                                                    let a403 = v0;
                                                                    let a404 = 12u64;
                                                                    if nat_eq(a403, a404) {
                                                                        Str::lit("core::cmp::PartialEq::eq(&")
                                                                    } else {
                                                                        let a405 = v0;
                                                                        let a406 = 13u64;
                                                                        if nat_eq(a405, a406) {
                                                                            Str::lit("core::cmp::PartialOrd::le(&")
                                                                        } else {
                                                                            let a407 = v0;
                                                                            let a408 = 14u64;
                                                                            if nat_eq(a407, a408) {
                                                                                Str::lit("core::cmp::PartialOrd::lt(&")
                                                                            } else {
                                                                                let a409 = v0;
                                                                                let a410 = 15u64;
                                                                                if nat_eq(a409, a410) {
                                                                                    Str::lit("u64::checked_rem(")
                                                                                } else {
                                                                                    let a411 = v0;
                                                                                    let a412 = 16u64;
                                                                                    if nat_eq(a411, a412) {
                                                                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                    } else {
                                                                                        let a413 = v0;
                                                                                        let a414 = 17u64;
                                                                                        if nat_eq(a413, a414) {
                                                                                            Str::lit("u64::checked_add(")
                                                                                        } else {
                                                                                            let a415 = v0;
                                                                                            let a416 = 18u64;
                                                                                            if nat_eq(a415, a416) {
                                                                                                Str::lit("u64::saturating_sub(")
                                                                                            } else {
                                                                                                let a417 = v0;
                                                                                                let a418 = 19u64;
                                                                                                if nat_eq(a417, a418) {
                                                                                                    Str::lit("u64::checked_mul(")
                                                                                                } else {
                                                                                                    let a419 = v0;
                                                                                                    let a420 = 20u64;
                                                                                                    if nat_eq(a419, a420) {
                                                                                                        Str::lit("u64::checked_div(")
                                                                                                    } else {
                                                                                                        let a421 = v0;
                                                                                                        let a422 = 21u64;
                                                                                                        if nat_eq(a421, a422) {
                                                                                                            Str::lit("i64::checked_add(")
                                                                                                        } else {
                                                                                                            let a423 = v0;
                                                                                                            let a424 = 22u64;
                                                                                                            if nat_eq(a423, a424) {
                                                                                                                Str::lit("i64::checked_sub(")
                                                                                                            } else {
                                                                                                                let a425 = v0;
                                                                                                                let a426 = 23u64;
                                                                                                                if nat_eq(a425, a426) {
                                                                                                                    Str::lit("i64::checked_mul(")
                                                                                                                } else {
                                                                                                                    let a427 = v0;
                                                                                                                    let a428 = 24u64;
                                                                                                                    if nat_eq(a427, a428) {
                                                                                                                        Str::lit("i64::checked_neg(")
                                                                                                                    } else {
                                                                                                                        let a429 = v0;
                                                                                                                        let a430 = 25u64;
                                                                                                                        if nat_eq(a429, a430) {
                                                                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                                                        } else {
                                                                                                                            let a431 = v0;
                                                                                                                            let a432 = 26u64;
                                                                                                                            if nat_eq(a431, a432) {
                                                                                                                                Str::lit("List::length(&")
                                                                                                                            } else {
                                                                                                                                let a433 = v0;
                                                                                                                                let a434 = 27u64;
                                                                                                                                if nat_eq(a433, a434) {
                                                                                                                                    Str::lit("List::append(")
                                                                                                                                } else {
                                                                                                                                    let a435 = v0;
                                                                                                                                    let a436 = 28u64;
                                                                                                                                    if nat_eq(a435, a436) {
                                                                                                                                        Str::lit("List::index(&")
                                                                                                                                    } else {
                                                                                                                                        let a437 = v0;
                                                                                                                                        let a438 = 29u64;
                                                                                                                                        if nat_eq(a437, a438) {
                                                                                                                                            Str::lit("List::slice(&")
                                                                                                                                        } else {
                                                                                                                                            Str::lit("")
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

pub fn f43(v0: u64) -> Str {
    let a439 = v0;
    let a440 = 16u64;
    if nat_eq(a439, a440) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a441 = v0;
        let a442 = 25u64;
        if nat_eq(a441, a442) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a443 = v0;
            let a444 = 15u64;
            if nat_eq(a443, a444) {
                Str::lit(")")
            } else {
                let a445 = v0;
                let a446 = 20u64;
                if nat_eq(a445, a446) {
                    Str::lit(")")
                } else {
                    let a447 = v0;
                    let a448 = 8u64;
                    if nat_eq(a447, a448) {
                        Str::lit(")")
                    } else {
                        let a449 = v0;
                        let a450 = 9u64;
                        if nat_eq(a449, a450) {
                            Str::lit(")")
                        } else {
                            let a451 = v0;
                            let a452 = 10u64;
                            if nat_eq(a451, a452) {
                                Str::lit(")")
                            } else {
                                let a453 = v0;
                                let a454 = 11u64;
                                if nat_eq(a453, a454) {
                                    Str::lit(")")
                                } else {
                                    let a455 = v0;
                                    let a456 = 12u64;
                                    if nat_eq(a455, a456) {
                                        Str::lit(")")
                                    } else {
                                        let a457 = v0;
                                        let a458 = 13u64;
                                        if nat_eq(a457, a458) {
                                            Str::lit(")")
                                        } else {
                                            let a459 = v0;
                                            let a460 = 14u64;
                                            if nat_eq(a459, a460) {
                                                Str::lit(")")
                                            } else {
                                                let a461 = v0;
                                                let a462 = 17u64;
                                                if nat_eq(a461, a462) {
                                                    Str::lit(")")
                                                } else {
                                                    let a463 = v0;
                                                    let a464 = 18u64;
                                                    if nat_eq(a463, a464) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a465 = v0;
                                                        let a466 = 19u64;
                                                        if nat_eq(a465, a466) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a467 = v0;
                                                            let a468 = 21u64;
                                                            if nat_eq(a467, a468) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a469 = v0;
                                                                let a470 = 22u64;
                                                                if nat_eq(a469, a470) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a471 = v0;
                                                                    let a472 = 23u64;
                                                                    if nat_eq(a471, a472) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a473 = v0;
                                                                        let a474 = 24u64;
                                                                        let _ = nat_eq(a473, a474);
                                                                        Str::lit(")")
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

pub fn f44(v0: Fn2, p479: u64, p480: (u64, Adt20)) -> R<((u64, Adt20), bool)> {
    let m478 = (p479, p480);
    {
        let mut m478 = m478;
        loop {
            let step = {
                let (v1, v2) = m478;
                let m475 = v1;
                if m475 == 0 {
                    Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m475 - 1;
                    let m477 = {
                        let c476: Fn2 = v0.clone();
                        c476.apply(v2.clone())?
                    };
                    match m477 {
                        None => {
                            Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f59(v0, v1)
}

pub fn f46(v0: u64) -> Str {
    let a481 = v0;
    let a482 = 37u64;
    if nat_eq(a481, a482) {
        Str::lit(", &")
    } else {
        let a483 = v0;
        let a484 = 38u64;
        if nat_eq(a483, a484) {
            Str::lit(", &")
        } else {
            let a485 = v0;
            let a486 = 39u64;
            if nat_eq(a485, a486) {
                Str::lit(", &")
            } else {
                let a487 = v0;
                let a488 = 12u64;
                if nat_eq(a487, a488) {
                    Str::lit(", &")
                } else {
                    let a489 = v0;
                    let a490 = 13u64;
                    if nat_eq(a489, a490) {
                        Str::lit(", &")
                    } else {
                        let a491 = v0;
                        let a492 = 14u64;
                        if nat_eq(a491, a492) {
                            Str::lit(", &")
                        } else {
                            let a493 = v0;
                            let a494 = 8u64;
                            if nat_eq(a493, a494) {
                                Str::lit(", ")
                            } else {
                                let a495 = v0;
                                let a496 = 9u64;
                                if nat_eq(a495, a496) {
                                    Str::lit(", ")
                                } else {
                                    let a497 = v0;
                                    let a498 = 10u64;
                                    if nat_eq(a497, a498) {
                                        Str::lit(", ")
                                    } else {
                                        let a499 = v0;
                                        let a500 = 11u64;
                                        if nat_eq(a499, a500) {
                                            Str::lit(", ")
                                        } else {
                                            let a501 = v0;
                                            let a502 = 15u64;
                                            if nat_eq(a501, a502) {
                                                Str::lit(", ")
                                            } else {
                                                let a503 = v0;
                                                let a504 = 16u64;
                                                if nat_eq(a503, a504) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a505 = v0;
                                                    let a506 = 17u64;
                                                    if nat_eq(a505, a506) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a507 = v0;
                                                        let a508 = 18u64;
                                                        if nat_eq(a507, a508) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a509 = v0;
                                                            let a510 = 19u64;
                                                            if nat_eq(a509, a510) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a511 = v0;
                                                                let a512 = 20u64;
                                                                if nat_eq(a511, a512) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a513 = v0;
                                                                    let a514 = 21u64;
                                                                    if nat_eq(a513, a514) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a515 = v0;
                                                                        let a516 = 22u64;
                                                                        if nat_eq(a515, a516) {
                                                                            Str::lit(", ")
                                                                        } else {
                                                                            let a517 = v0;
                                                                            let a518 = 23u64;
                                                                            if nat_eq(a517, a518) {
                                                                                Str::lit(", ")
                                                                            } else {
                                                                                let a519 = v0;
                                                                                let a520 = 24u64;
                                                                                if nat_eq(a519, a520) {
                                                                                    Str::lit(", ")
                                                                                } else {
                                                                                    let a521 = v0;
                                                                                    let a522 = 25u64;
                                                                                    let _ = nat_eq(a521, a522);
                                                                                    Str::lit(", ")
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

pub fn f47(v0: u64) -> Str {
    let a523 = v0;
    let a524 = 15u64;
    if nat_eq(a523, a524) {
        Str::lit(").unwrap_or(")
    } else {
        let a525 = v0;
        let a526 = 16u64;
        if nat_eq(a525, a526) {
            Str::lit(", ")
        } else {
            let a527 = v0;
            let a528 = 20u64;
            if nat_eq(a527, a528) {
                Str::lit(").unwrap_or(")
            } else {
                let a529 = v0;
                let a530 = 25u64;
                let _ = nat_eq(a529, a530);
                Str::lit(", ")
            }
        }
    }
}

pub fn f48(v0: Adt20, v1: Adt12) -> R<Adt20> {
    f8(v0, {
        let m531 = v1;
        match m531 {
            Adt12::C0 => {
                Str::lit("()")
            }
            Adt12::C1 => {
                Str::lit("bool")
            }
            Adt12::C2 => {
                Str::lit("u64")
            }
            Adt12::C3 => {
                Str::lit("i64")
            }
            Adt12::C4 => {
                Str::lit("Ordering")
            }
            Adt12::C5 => {
                Str::lit("Str")
            }
            Adt12::C6 => {
                Str::lit("Bytes")
            }
            Adt12::C7 => {
                Str::lit("Overflow")
            }
            Adt12::C8 => {
                Str::lit("i128")
            }
        }
    })
}

pub fn f49(v0: Adt13) -> Str {
    let m532 = v0;
    match m532 {
        Adt13::C0 => {
            Str::lit("Option<")
        }
        Adt13::C1 => {
            Str::lit("List<")
        }
        Adt13::C2 => {
            Str::lit("std::rc::Rc<")
        }
        Adt13::C3 => {
            Str::lit("R<")
        }
        Adt13::C4 => {
            Str::lit("&")
        }
    }
}

pub fn f50(v0: Adt13) -> Str {
    let m533 = v0;
    match m533 {
        Adt13::C0 => {
            Str::lit(">")
        }
        Adt13::C1 => {
            Str::lit(">")
        }
        Adt13::C2 => {
            Str::lit(">")
        }
        Adt13::C3 => {
            Str::lit(">")
        }
        Adt13::C4 => {
            Str::lit("")
        }
    }
}

pub fn f51(v0: Adt20, v1: u64) -> R<Adt20> {
    let a534 = v1;
    let a535 = 0u64;
    if nat_eq(a534, a535) {
        f8(v0, Str::lit("T"))
    } else {
        let a536 = v1;
        let a537 = 1u64;
        if nat_eq(a536, a537) {
            f8(v0, Str::lit("U"))
        } else {
            let a538 = v1;
            let a539 = 2u64;
            if nat_eq(a538, a539) {
                f8(v0, Str::lit("V"))
            } else {
                f11(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f52(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m540 = v0.clone();
    match m540.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m541 = v2;
            match m541.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f60(Fn3::F61, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m543 = {
                        let (h542, _) = v5;
                        h542
                    };
                    match m543 {
                        None => {
                            Some(f5({
                                let (_, h545) = v5;
                                h545
                            }))
                        }
                        Some(v6) => {
                            Some(f5(List::cons(v6, {
                                let (_, h544) = v5;
                                h544
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f53(v0: Adt2) -> Str {
    let m546 = v0;
    match m546 {
        Adt2::C0 => {
            Str::lit("u8")
        }
        Adt2::C1 => {
            Str::lit("u16")
        }
        Adt2::C2 => {
            Str::lit("u32")
        }
        Adt2::C3 => {
            Str::lit("u64")
        }
        Adt2::C4 => {
            Str::lit("i8")
        }
        Adt2::C5 => {
            Str::lit("i16")
        }
        Adt2::C6 => {
            Str::lit("i32")
        }
        Adt2::C7 => {
            Str::lit("i64")
        }
    }
}

pub fn f54(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f62(v0, v1)
}

pub fn f55(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f63(v0, v1)
}

pub fn f56(v0: Adt20, v1: Bytes, v2: u64) -> Adt20 {
    let a548 = 256u64;
    let a549 = {
        let a547 = v1.clone();
        length_bytes(a547)
    };
    if nat_le(a548, a549) {
        Adt20::C0(List::cons(v1, match v0.clone() {
            Adt20::C0(h550, _, _, _, _) => {
                h550
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt20::C0(_, _, _, h551, _) => {
                h551
            }
        }, false)
    } else {
        Adt20::C0(match v0.clone() {
            Adt20::C0(h552, _, _, _, _) => {
                h552
            }
        }, v1, v2, match v0 {
            Adt20::C0(_, _, _, h553, _) => {
                h553
            }
        }, false)
    }
}

pub fn f57(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h554, _, _, _, _) => {
            h554
        }
    }, match v0.clone() {
        Adt20::C0(_, h555, _, _, _) => {
            h555
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h556, _, _) => {
            h556
        }
    }, match v0 {
        Adt20::C0(_, _, _, h557, _) => {
            h557
        }
    }, true)
}

pub fn f58(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a594 = f64(v0, 10000000000000000000u64);
        let a595 = {
            let a592 = f64(v0, 1000000000000000000u64);
            let a593 = {
                let a590 = f64(v0, 100000000000000000u64);
                let a591 = {
                    let a588 = f64(v0, 10000000000000000u64);
                    let a589 = {
                        let a586 = f64(v0, 1000000000000000u64);
                        let a587 = {
                            let a584 = f64(v0, 100000000000000u64);
                            let a585 = {
                                let a582 = f64(v0, 10000000000000u64);
                                let a583 = {
                                    let a580 = f64(v0, 1000000000000u64);
                                    let a581 = {
                                        let a578 = f64(v0, 100000000000u64);
                                        let a579 = {
                                            let a576 = f64(v0, 10000000000u64);
                                            let a577 = {
                                                let a574 = f64(v0, 1000000000u64);
                                                let a575 = {
                                                    let a572 = f64(v0, 100000000u64);
                                                    let a573 = {
                                                        let a570 = f64(v0, 10000000u64);
                                                        let a571 = {
                                                            let a568 = f64(v0, 1000000u64);
                                                            let a569 = {
                                                                let a566 = f64(v0, 100000u64);
                                                                let a567 = {
                                                                    let a564 = f64(v0, 10000u64);
                                                                    let a565 = {
                                                                        let a562 = f64(v0, 1000u64);
                                                                        let a563 = {
                                                                            let a560 = f64(v0, 100u64);
                                                                            let a561 = {
                                                                                let a558 = f64(v0, 10u64);
                                                                                let a559 = f64(v0, 1u64);
                                                                                append_bytes(a558, a559)
                                                                            };
                                                                            append_bytes(a560, a561)
                                                                        };
                                                                        append_bytes(a562, a563)
                                                                    };
                                                                    append_bytes(a564, a565)
                                                                };
                                                                append_bytes(a566, a567)
                                                            };
                                                            append_bytes(a568, a569)
                                                        };
                                                        append_bytes(a570, a571)
                                                    };
                                                    append_bytes(a572, a573)
                                                };
                                                append_bytes(a574, a575)
                                            };
                                            append_bytes(a576, a577)
                                        };
                                        append_bytes(a578, a579)
                                    };
                                    append_bytes(a580, a581)
                                };
                                append_bytes(a582, a583)
                            };
                            append_bytes(a584, a585)
                        };
                        append_bytes(a586, a587)
                    };
                    append_bytes(a588, a589)
                };
                append_bytes(a590, a591)
            };
            append_bytes(a592, a593)
        };
        append_bytes(a594, a595)
    };
    let v2: u64 = {
        let (h596, _) = f65(Fn4::F66(v1.clone()), {
            let a597 = v1.clone();
            length_bytes(a597)
        }, 0u64)?;
        h596
    };
    let a598 = v0;
    let a599 = 0u64;
    if nat_eq(a598, a599) {
        let a600 = Str::lit("0");
        Ok(utf8_encode(a600))
    } else {
        Ok(f67(v1.clone(), v2, {
            let a602 = {
                let a601 = v1;
                length_bytes(a601)
            };
            let a603 = v2;
            nat_sub(a602, a603)
        }))
    }
}

pub fn f59(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a605 = {
        let (h604, _) = v1.clone();
        h604
    };
    let a606 = v0;
    if nat_lt(a605, a606) {
        Ok(Some(({
            let a608 = {
                let (h607, _) = v1;
                h607
            };
            let a609 = 1u64;
            nat_add(a608, a609)?
        }, f8({
            let (_, h610) = v1;
            h610
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f60(v0: Fn3, p614: (Option<Bytes>, List<Bytes>), p615: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m611 = v2;
                match m611.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c612: Fn3 = v0.clone();
                            c612.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f62(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a618 = {
        let (h616, _) = v1.clone();
        h616
    };
    let a619 = {
        let a617 = v0.clone();
        length_bytes(a617)
    };
    if nat_lt(a618, a619) {
        let v2: (u64, u64) = f69(v0.clone(), {
            let (h620, _) = v1.clone();
            h620
        })?;
        let v3: u64 = {
            let (h621, _) = v2;
            h621
        };
        Ok(Some(({
            let a624 = {
                let (h622, _) = v1.clone();
                h622
            };
            let a625 = {
                let (_, h623) = v2;
                h623
            };
            nat_add(a624, a625)?
        }, {
            let a626 = v3;
            let a627 = 34u64;
            if nat_eq(a626, a627) {
                f8({
                    let (_, h628) = v1;
                    h628
                }, Str::lit("\\\""))?
            } else {
                let a629 = v3;
                let a630 = 92u64;
                if nat_eq(a629, a630) {
                    f8({
                        let (_, h631) = v1;
                        h631
                    }, Str::lit("\\\\"))?
                } else {
                    let a632 = 32u64;
                    let a633 = v3;
                    if if nat_le(a632, a633) {
                        let a634 = v3;
                        let a635 = 126u64;
                        nat_le(a634, a635)
                    } else {
                        false
                    } {
                        f36({
                            let (_, h636) = v1;
                            h636
                        }, f67(v0, {
                            let (h637, _) = v1;
                            h637
                        }, 1u64))?
                    } else {
                        f8(f70(f8({
                            let (_, h638) = v1;
                            h638
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f63(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let m640 = f71(v0, {
        let (h639, _) = v1.clone();
        h639
    });
    match m640 {
        None => {
            Ok(None::<(u64, Adt20)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a642 = {
                    let (h641, _) = v1.clone();
                    h641
                };
                let a643 = 1u64;
                nat_add(a642, a643)?
            }, f72(f8({
                let a645 = {
                    let (h644, _) = v1;
                    h644
                };
                let a646 = 0u64;
                if nat_eq(a645, a646) {
                    let (_, h647) = v1;
                    h647
                } else {
                    f8({
                        let (_, h648) = v1;
                        h648
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a652 = {
            let a649 = v0;
            let a650 = v1;
            let a651 = 0u64;
            nat_quot(a649, a650, a651)
        };
        let a653 = 10u64;
        let a654 = 0u64;
        nat_rem(a652, a653, a654)
    }, 1u64)
}

pub fn f65(v0: Fn4, p659: u64, p660: u64) -> R<(u64, bool)> {
    let m658 = (p659, p660);
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v1, v2) = m658;
                let m655 = v1;
                if m655 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m655 - 1;
                    let m657 = {
                        let c656: Fn4 = v0.clone();
                        c656.apply(v2)?
                    };
                    match m657 {
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m664 = {
        let a661 = v0;
        let a662 = v1;
        let a663 = v2;
        slice_bytes(a661, a662, a663)
    };
    match m664 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m666 = {
        let (h665, _) = v0;
        h665
    };
    match m666 {
        None => {
            (Some(v1), {
                let (_, h667) = v0;
                h667
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a668 = v2;
                let a669 = v1;
                append_bytes(a668, a669)
            }, {
                let (_, h670) = v0;
                h670
            }))
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m671 = f71(v0.clone(), v1);
    match m671 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a716 = {
                    let a692 = {
                        let a680 = {
                            let a674 = {
                                let a672 = v3;
                                let a673 = 128u8;
                                fixed_u8::bit_and(a672, a673)
                            };
                            let a675 = 128u8;
                            if equal(a674, a675) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a681 = {
                            let a678 = {
                                let a676 = v3;
                                let a677 = 64u8;
                                fixed_u8::bit_and(a676, a677)
                            };
                            let a679 = 64u8;
                            if equal(a678, a679) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a680, a681)?
                    };
                    let a693 = {
                        let a690 = {
                            let a684 = {
                                let a682 = v3;
                                let a683 = 32u8;
                                fixed_u8::bit_and(a682, a683)
                            };
                            let a685 = 32u8;
                            if equal(a684, a685) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a691 = {
                            let a688 = {
                                let a686 = v3;
                                let a687 = 16u8;
                                fixed_u8::bit_and(a686, a687)
                            };
                            let a689 = 16u8;
                            if equal(a688, a689) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a690, a691)?
                    };
                    nat_add(a692, a693)?
                };
                let a717 = {
                    let a714 = {
                        let a702 = {
                            let a696 = {
                                let a694 = v3;
                                let a695 = 8u8;
                                fixed_u8::bit_and(a694, a695)
                            };
                            let a697 = 8u8;
                            if equal(a696, a697) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a703 = {
                            let a700 = {
                                let a698 = v3;
                                let a699 = 4u8;
                                fixed_u8::bit_and(a698, a699)
                            };
                            let a701 = 4u8;
                            if equal(a700, a701) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a702, a703)?
                    };
                    let a715 = {
                        let a712 = {
                            let a706 = {
                                let a704 = v3;
                                let a705 = 2u8;
                                fixed_u8::bit_and(a704, a705)
                            };
                            let a707 = 2u8;
                            if equal(a706, a707) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a713 = {
                            let a710 = {
                                let a708 = v3;
                                let a709 = 1u8;
                                fixed_u8::bit_and(a708, a709)
                            };
                            let a711 = 1u8;
                            if equal(a710, a711) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a712, a713)?
                    };
                    nat_add(a714, a715)?
                };
                nat_add(a716, a717)?
            };
            let a718 = v4;
            let a719 = 128u64;
            if nat_lt(a718, a719) {
                Ok((v4, 1u64))
            } else {
                let a720 = v4;
                let a721 = 224u64;
                if nat_lt(a720, a721) {
                    Ok(({
                        let a729 = {
                            let a725 = {
                                let a722 = v4;
                                let a723 = 32u64;
                                let a724 = 0u64;
                                nat_rem(a722, a723, a724)
                            };
                            let a726 = 64u64;
                            nat_mul(a725, a726)?
                        };
                        let a730 = f74(v0, {
                            let a727 = v1;
                            let a728 = 1u64;
                            nat_add(a727, a728)?
                        })?;
                        nat_add(a729, a730)?
                    }, 2u64))
                } else {
                    let a731 = v4;
                    let a732 = 240u64;
                    if nat_lt(a731, a732) {
                        Ok(({
                            let a746 = {
                                let a736 = {
                                    let a733 = v4;
                                    let a734 = 16u64;
                                    let a735 = 0u64;
                                    nat_rem(a733, a734, a735)
                                };
                                let a737 = 4096u64;
                                nat_mul(a736, a737)?
                            };
                            let a747 = {
                                let a744 = {
                                    let a740 = f74(v0.clone(), {
                                        let a738 = v1;
                                        let a739 = 1u64;
                                        nat_add(a738, a739)?
                                    })?;
                                    let a741 = 64u64;
                                    nat_mul(a740, a741)?
                                };
                                let a745 = f74(v0, {
                                    let a742 = v1;
                                    let a743 = 2u64;
                                    nat_add(a742, a743)?
                                })?;
                                nat_add(a744, a745)?
                            };
                            nat_add(a746, a747)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a767 = {
                                let a751 = {
                                    let a748 = v4;
                                    let a749 = 8u64;
                                    let a750 = 0u64;
                                    nat_rem(a748, a749, a750)
                                };
                                let a752 = 262144u64;
                                nat_mul(a751, a752)?
                            };
                            let a768 = {
                                let a765 = {
                                    let a755 = f74(v0.clone(), {
                                        let a753 = v1;
                                        let a754 = 1u64;
                                        nat_add(a753, a754)?
                                    })?;
                                    let a756 = 4096u64;
                                    nat_mul(a755, a756)?
                                };
                                let a766 = {
                                    let a763 = {
                                        let a759 = f74(v0.clone(), {
                                            let a757 = v1;
                                            let a758 = 2u64;
                                            nat_add(a757, a758)?
                                        })?;
                                        let a760 = 64u64;
                                        nat_mul(a759, a760)?
                                    };
                                    let a764 = f74(v0, {
                                        let a761 = v1;
                                        let a762 = 3u64;
                                        nat_add(a761, a762)?
                                    })?;
                                    nat_add(a763, a764)?
                                };
                                nat_add(a765, a766)?
                            };
                            nat_add(a767, a768)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: Adt20, v1: u64) -> R<Adt20> {
    let v2: Bytes = {
        let a781 = f75(v1, 268435456u64);
        let a782 = {
            let a779 = f75(v1, 16777216u64);
            let a780 = {
                let a777 = f75(v1, 1048576u64);
                let a778 = {
                    let a775 = f75(v1, 65536u64);
                    let a776 = {
                        let a773 = f75(v1, 4096u64);
                        let a774 = {
                            let a771 = f75(v1, 256u64);
                            let a772 = {
                                let a769 = f75(v1, 16u64);
                                let a770 = f75(v1, 1u64);
                                append_bytes(a769, a770)
                            };
                            append_bytes(a771, a772)
                        };
                        append_bytes(a773, a774)
                    };
                    append_bytes(a775, a776)
                };
                append_bytes(a777, a778)
            };
            append_bytes(a779, a780)
        };
        append_bytes(a781, a782)
    };
    let v3: u64 = {
        let (h783, _) = f65(Fn4::F76(v2.clone()), 7u64, 0u64)?;
        h783
    };
    f36(v0, f67(v2, v3, {
        let a784 = 8u64;
        let a785 = v3;
        nat_sub(a784, a785)
    }))
}

pub fn f71(v0: Bytes, v1: u64) -> Option<u8> {
    let a786 = v0;
    let a787 = v1;
    index_bytes(a786, a787)
}

pub fn f72(v0: Adt20, v1: u8) -> R<Adt20> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a832 = {
            let a808 = {
                let a796 = {
                    let a790 = {
                        let a788 = v2;
                        let a789 = 128u8;
                        fixed_u8::bit_and(a788, a789)
                    };
                    let a791 = 128u8;
                    if equal(a790, a791) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a797 = {
                    let a794 = {
                        let a792 = v2;
                        let a793 = 64u8;
                        fixed_u8::bit_and(a792, a793)
                    };
                    let a795 = 64u8;
                    if equal(a794, a795) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a796, a797)?
            };
            let a809 = {
                let a806 = {
                    let a800 = {
                        let a798 = v2;
                        let a799 = 32u8;
                        fixed_u8::bit_and(a798, a799)
                    };
                    let a801 = 32u8;
                    if equal(a800, a801) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a807 = {
                    let a804 = {
                        let a802 = v2;
                        let a803 = 16u8;
                        fixed_u8::bit_and(a802, a803)
                    };
                    let a805 = 16u8;
                    if equal(a804, a805) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a806, a807)?
            };
            nat_add(a808, a809)?
        };
        let a833 = {
            let a830 = {
                let a818 = {
                    let a812 = {
                        let a810 = v2;
                        let a811 = 8u8;
                        fixed_u8::bit_and(a810, a811)
                    };
                    let a813 = 8u8;
                    if equal(a812, a813) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a819 = {
                    let a816 = {
                        let a814 = v2;
                        let a815 = 4u8;
                        fixed_u8::bit_and(a814, a815)
                    };
                    let a817 = 4u8;
                    if equal(a816, a817) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a818, a819)?
            };
            let a831 = {
                let a828 = {
                    let a822 = {
                        let a820 = v2;
                        let a821 = 2u8;
                        fixed_u8::bit_and(a820, a821)
                    };
                    let a823 = 2u8;
                    if equal(a822, a823) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a829 = {
                    let a826 = {
                        let a824 = v2;
                        let a825 = 1u8;
                        fixed_u8::bit_and(a824, a825)
                    };
                    let a827 = 1u8;
                    if equal(a826, a827) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a828, a829)?
            };
            nat_add(a830, a831)?
        };
        nat_add(a832, a833)?
    };
    f36(v0, {
        let a834 = f75(v3, 16u64);
        let a835 = f75(v3, 1u64);
        append_bytes(a834, a835)
    })
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m836 = f71(v0, v1);
    match m836 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a837 = v2;
            let a838 = 48u8;
            if equal(a837, a838) {
                Ok(Some({
                    let a839 = v1;
                    let a840 = 1u64;
                    nat_add(a839, a840)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64) -> R<u64> {
    let m841 = f71(v0, v1);
    match m841 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a888 = {
                let v3: u8 = v2;
                let a886 = {
                    let a862 = {
                        let a850 = {
                            let a844 = {
                                let a842 = v3;
                                let a843 = 128u8;
                                fixed_u8::bit_and(a842, a843)
                            };
                            let a845 = 128u8;
                            if equal(a844, a845) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a851 = {
                            let a848 = {
                                let a846 = v3;
                                let a847 = 64u8;
                                fixed_u8::bit_and(a846, a847)
                            };
                            let a849 = 64u8;
                            if equal(a848, a849) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a850, a851)?
                    };
                    let a863 = {
                        let a860 = {
                            let a854 = {
                                let a852 = v3;
                                let a853 = 32u8;
                                fixed_u8::bit_and(a852, a853)
                            };
                            let a855 = 32u8;
                            if equal(a854, a855) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a861 = {
                            let a858 = {
                                let a856 = v3;
                                let a857 = 16u8;
                                fixed_u8::bit_and(a856, a857)
                            };
                            let a859 = 16u8;
                            if equal(a858, a859) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a860, a861)?
                    };
                    nat_add(a862, a863)?
                };
                let a887 = {
                    let a884 = {
                        let a872 = {
                            let a866 = {
                                let a864 = v3;
                                let a865 = 8u8;
                                fixed_u8::bit_and(a864, a865)
                            };
                            let a867 = 8u8;
                            if equal(a866, a867) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a873 = {
                            let a870 = {
                                let a868 = v3;
                                let a869 = 4u8;
                                fixed_u8::bit_and(a868, a869)
                            };
                            let a871 = 4u8;
                            if equal(a870, a871) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a872, a873)?
                    };
                    let a885 = {
                        let a882 = {
                            let a876 = {
                                let a874 = v3;
                                let a875 = 2u8;
                                fixed_u8::bit_and(a874, a875)
                            };
                            let a877 = 2u8;
                            if equal(a876, a877) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a883 = {
                            let a880 = {
                                let a878 = v3;
                                let a879 = 1u8;
                                fixed_u8::bit_and(a878, a879)
                            };
                            let a881 = 1u8;
                            if equal(a880, a881) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a882, a883)?
                    };
                    nat_add(a884, a885)?
                };
                nat_add(a886, a887)?
            };
            let a889 = 64u64;
            let a890 = 0u64;
            Ok(nat_rem(a888, a889, a890))
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a894 = {
            let a891 = v0;
            let a892 = v1;
            let a893 = 0u64;
            nat_quot(a891, a892, a893)
        };
        let a895 = 16u64;
        let a896 = 0u64;
        nat_rem(a894, a895, a896)
    }, 1u64)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f77(v0, v1)
}

pub fn f77(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m897 = f71(v0, v1);
    match m897 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a898 = v2;
            let a899 = 48u8;
            if equal(a898, a899) {
                Ok(Some({
                    let a900 = v1;
                    let a901 = 1u64;
                    nat_add(a900, a901)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}
