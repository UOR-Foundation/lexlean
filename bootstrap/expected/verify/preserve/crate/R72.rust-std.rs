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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C3(u64, std::rc::Rc<Adt1>),
    C4,
    C5,
    C6(std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4(std::rc::Rc<Adt0>),
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
}

#[derive(Clone)]
pub enum Adt3 {
    C0(List<Bytes>, u64, List<Adt4>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, bool, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Option<Adt0>),
    C1(Bytes),
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt6 {
    C0(std::rc::Rc<Adt0>, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(std::rc::Rc<Adt0>, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, bool, List<Adt4>, List<Adt4>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F18(u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F18(k0, k1) => f18(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F40,
    F88(Bytes, Bytes),
    F94,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn1::F40 => f40(p0, p1),
            Fn1::F88(k0, k1) => f88(k0.clone(), k1.clone(), p0, p1),
            Fn1::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F79,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn2::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F81(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt1, p1: (Bytes, Adt1)) -> Adt1 {
        match self {
            Fn3::F81(k0) => f81(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F90(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, List<Adt1>), p1: Adt1) -> (bool, List<Adt1>) {
        match self {
            Fn4::F90(k0) => f90(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F93,
}

impl Fn5 {
    pub fn apply(&self, p0: (bool, Adt1), p1: Adt1) -> (bool, Adt1) {
        match self {
            Fn5::F93 => f93(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F97(Option<Bytes>),
    F241,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
        match self {
            Fn6::F97(k0) => f97(k0.clone(), p0, p1),
            Fn6::F241 => Ok(f241(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F99,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn7::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F102(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6) -> Option<Adt6> {
        match self {
            Fn8::F102(k0) => f102(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F104(std::rc::Rc<Adt0>),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn9::F104(k0) => f104((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F108,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn10::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F110,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn11::F110 => f110(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F121,
    F122,
    F127,
    F128,
    F129,
    F208,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt0) -> R<bool> {
        match self {
            Fn12::F121 => f121(p0),
            Fn12::F122 => f122(p0),
            Fn12::F127 => f127(p0),
            Fn12::F128 => f128(p0),
            Fn12::F129 => f129(p0),
            Fn12::F208 => f208(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F131(u64, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn13::F131(k0, k1) => f131(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F136,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt0) -> List<Adt1> {
        match self {
            Fn14::F136 => f136(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F141,
}

impl Fn15 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn15::F141 => f141(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F147(Bytes, std::rc::Rc<Adt0>),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn16::F147(k0, k1) => f147(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F149(std::rc::Rc<Adt0>),
    F174(Fn12),
    F217(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn17::F149(k0) => f149((**k0).clone(), p0, p1),
            Fn17::F174(k0) => f174(k0.clone(), p0, p1),
            Fn17::F217(k0) => Ok(f217(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F157,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn18::F157 => f157(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F159,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn19::F159 => f159(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F163(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn20::F163(k0) => f163(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F181,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt10) -> Option<Adt10> {
        match self {
            Fn21::F181 => f181(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F185(Bytes),
    F207(bool, Bytes),
    F226(u64, Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn22::F185(k0) => f185(k0.clone(), p0),
            Fn22::F207(k0, k1) => f207(*k0, k1.clone(), p0),
            Fn22::F226(k0, k1) => f226(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F189(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn23::F189(k0, k1) => f189(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F193,
}

impl Fn24 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn24::F193 => f193(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn25::F198(k0) => f198(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F201,
}

impl Fn26 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn26::F201 => f201(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F203,
}

impl Fn27 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn27::F203 => f203(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F205(std::rc::Rc<Adt0>),
    F206(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn28::F205(k0) => f205((**k0).clone(), p0, p1),
            Fn28::F206(k0) => f206(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F213,
}

impl Fn29 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn29::F213 => f213(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F222(u64),
}

impl Fn30 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn30::F222(k0) => f222(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F229,
}

impl Fn31 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn31::F229 => f229(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F240,
}

impl Fn32 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn32::F240 => f240(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Adt0, v2: u64) -> R<Bytes> {
    f1(f2(v0.clone(), v1, f3(v0, v2)?)?)
}

pub fn f1(v0: Adt1) -> R<Bytes> {
    f4(v0.clone(), 100u64, f5(v0)?)
}

pub fn f2(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt1> {
    let m1 = v1;
    match m1 {
        Adt0::C0(v5) => {
            f12(Adt0::C0(v5))
        }
        Adt0::C1(v6, v7) => {
            f12(Adt0::C1(v6, v7))
        }
        Adt0::C2(v8) => {
            f12(Adt0::C2(v8))
        }
        Adt0::C3(r2) => {
            let v4 = (*r2).clone();
            Ok(f9(v2.clone(), f10(v0.clone(), v4, f11(v0, v2)?)?))
        }
        Adt0::C4(r3) => {
            let v3 = (*r3).clone();
            f6(v3.clone(), v2.clone(), f7(v0.clone(), v3.clone(), f8(v0, v3, v2)?)?)
        }
        Adt0::C5 => {
            Ok(Adt1::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt1::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f3(v0: Adt0, v1: u64) -> R<Adt2> {
    let a4 = v1;
    let a5 = 0u64;
    if nat_eq(a4, a5) {
        let m6 = f13(v0.clone(), f14(Str::lit("properties")));
        match m6 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m7 = f13(v2, f14(Str::lit("declarations")));
                match m7 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f15(v0, v3)
                    }
                }
            }
        }
    } else {
        let a8 = v1;
        let a9 = 1u64;
        if nat_eq(a8, a9) {
            Ok(Adt2::C0)
        } else {
            let a10 = v1;
            let a11 = 2u64;
            if nat_eq(a10, a11) {
                Ok(Adt2::C1)
            } else {
                let m12 = f16(v0, f14(Str::lit("proof")));
                match m12 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v4) => {
                        Ok(Adt2::C4(std::rc::Rc::new(v4)))
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt3 = {
        let (h13, _) = f17(Fn0::F18(v2, v1), {
            let a14 = v2;
            let a15 = 1u64;
            nat_add(a14, a15)?
        }, Adt3::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt4::C0(0u64, false, std::rc::Rc::new(v0)), List::<Adt4>::nil())))?;
        h13
    };
    Ok(f19(match v3 {
        Adt3::C0(h16, _, _) => {
            h16
        }
    }))
}

pub fn f5(v0: Adt1) -> R<u64> {
    let m17 = v0;
    match m17 {
        Adt1::C0 => {
            Ok(1u64)
        }
        Adt1::C1(_) => {
            Ok(1u64)
        }
        Adt1::C2(r26, r27) => {
            let v2 = (*r26).clone();
            let v3 = (*r27).clone();
            let v4: u64 = f5(v2)?;
            let v5: u64 = f5(v3)?;
            let a20 = {
                let a18 = 1u64;
                let a19 = v4;
                nat_add(a18, a19)?
            };
            let a21 = v5;
            nat_add(a20, a21)
        }
        Adt1::C3(_, r28) => {
            let v7 = (*r28).clone();
            let v8: u64 = f5(v7)?;
            let a22 = 1u64;
            let a23 = v8;
            nat_add(a22, a23)
        }
        Adt1::C4 => {
            Ok(1u64)
        }
        Adt1::C5 => {
            Ok(1u64)
        }
        Adt1::C6(r29) => {
            let v9 = (*r29).clone();
            let v10: u64 = f5(v9)?;
            let a24 = 1u64;
            let a25 = v10;
            nat_add(a24, a25)
        }
    }
}

pub fn f6(v0: Adt0, v1: Adt2, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m30 = v1;
    match m30 {
        Adt2::C0 => {
            if f20(v0.clone())? {
                f21(v0.clone(), {
                    let m31 = f22(v0);
                    match m31 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }, v2)
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C1 => {
            if f24(v0.clone())? {
                f25(v0.clone(), {
                    let m32 = f22(v0);
                    match m32 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v4) => {
                            v4
                        }
                    }
                }, v2)
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C2 => {
            let m33 = f26(Adt0::C4(std::rc::Rc::new(v0.clone())))?;
            match m33 {
                None => {
                    f12(Adt0::C4(std::rc::Rc::new(v0)))
                }
                Some(v5) => {
                    Ok(f27(v5))
                }
            }
        }
        Adt2::C3 => {
            if f28(Adt0::C4(std::rc::Rc::new(v0.clone())))? {
                Ok(f29(List::cons(f27(f30(f31(v0, f14(Str::lit("name"))), f14(Str::lit(" : ")))), List::cons(f32(v2, f14(Str::lit("type"))), List::<Adt1>::nil()))))
            } else {
                f23(v0, v2)
            }
        }
        Adt2::C4(_) => {
            f23(v0, v2)
        }
        Adt2::C5 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C6 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C7 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C8 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C9 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C10 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C11 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C12 => {
            Ok(f29(List::cons(f27(f30(f31(v0, f14(Str::lit("field"))), f14(Str::lit(" := ")))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
        }
        Adt2::C13 => {
            f12(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C14 => {
            f33(v0, v2)
        }
    }
}

pub fn f7(v0: Adt0, v1: Adt0, v2: Adt5) -> R<List<(Bytes, Adt1)>> {
    let m34 = v1;
    match m34 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C7(v3, r35, r36) => {
            let v4 = (*r35).clone();
            let v5 = (*r36).clone();
            if f34(v2.clone()) {
                Ok(List::<(Bytes, Adt1)>::nil())
            } else {
                Ok(List::cons((v3.clone(), f2(v0.clone(), v4, f35(v0.clone(), v2.clone(), v3)?)?), f7(v0, v5, v2)?))
            }
        }
    }
}

pub fn f8(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt5> {
    let m37 = v2.clone();
    match m37 {
        Adt2::C0 => {
            if f20(v1.clone())? {
                Ok(Adt5::C1({
                    let m38 = f22(v1);
                    match m38 {
                        None => {
                            f14(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }))
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C1 => {
            if f24(v1.clone())? {
                Ok(Adt5::C2)
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C2 => {
            Ok(Adt5::C6)
        }
        Adt2::C3 => {
            if f28(Adt0::C4(std::rc::Rc::new(v1.clone())))? {
                Ok(Adt5::C3)
            } else {
                Ok(Adt5::C0(f36(v0, v1, v2)?))
            }
        }
        Adt2::C4(_) => {
            Ok(Adt5::C0(f36(v0, v1, v2)?))
        }
        Adt2::C5 => {
            Ok(Adt5::C6)
        }
        Adt2::C6 => {
            Ok(Adt5::C6)
        }
        Adt2::C7 => {
            Ok(Adt5::C6)
        }
        Adt2::C8 => {
            Ok(Adt5::C6)
        }
        Adt2::C9 => {
            Ok(Adt5::C6)
        }
        Adt2::C10 => {
            Ok(Adt5::C6)
        }
        Adt2::C11 => {
            Ok(Adt5::C6)
        }
        Adt2::C12 => {
            Ok(Adt5::C4)
        }
        Adt2::C13 => {
            Ok(Adt5::C6)
        }
        Adt2::C14 => {
            Ok(Adt5::C5)
        }
    }
}

pub fn f9(v0: Adt2, v1: List<Adt1>) -> Adt1 {
    let m39 = v0;
    match m39 {
        Adt2::C0 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C1 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C2 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C3 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C4(_) => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C5 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C6 => {
            f29(f37(v1, f14(Str::lit(" ")), f14(Str::lit(""))))
        }
        Adt2::C7 => {
            f29(List::cons(f27(f14(Str::lit("["))), {
                let a40 = f38(v1, f14(Str::lit(", ")));
                let a41 = List::cons(f27(f14(Str::lit("]"))), List::<Adt1>::nil());
                append_list(a40, a41)
            }))
        }
        Adt2::C8 => {
            f29(f37(v1, f14(Str::lit("(")), f14(Str::lit(") "))))
        }
        Adt2::C9 => {
            f29(List::cons(f27(f14(Str::lit("("))), {
                let a42 = f38(v1, f14(Str::lit(" ")));
                let a43 = List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil());
                append_list(a42, a43)
            }))
        }
        Adt2::C10 => {
            f29(f37(v1, f14(Str::lit(" ")), f14(Str::lit(""))))
        }
        Adt2::C11 => {
            f29(List::cons(f27(f14(Str::lit("{"))), {
                let a44 = f38(v1, f14(Str::lit(", ")));
                let a45 = List::cons(f27(f14(Str::lit("}"))), List::<Adt1>::nil());
                append_list(a44, a45)
            }))
        }
        Adt2::C12 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
        Adt2::C13 => {
            f29(f39(Fn1::F40, List::<Adt1>::nil(), v1))
        }
        Adt2::C14 => {
            f41(f14(Str::lit("[")), f14(Str::lit("]")), f42(v1))
        }
    }
}

pub fn f10(v0: Adt0, v1: Adt0, v2: Adt2) -> R<List<Adt1>> {
    let m46 = v1;
    match m46 {
        Adt0::C0(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C6(r47, r48) => {
            let v3 = (*r47).clone();
            let v4 = (*r48).clone();
            Ok(List::cons(f2(v0.clone(), v3, v2.clone())?, f10(v0, v4, v2)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt1>::nil())
        }
    }
}

pub fn f11(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m49 = v1;
    match m49 {
        Adt2::C0 => {
            Ok(Adt2::C5)
        }
        Adt2::C1 => {
            Ok(Adt2::C5)
        }
        Adt2::C2 => {
            Ok(Adt2::C5)
        }
        Adt2::C3 => {
            Ok(Adt2::C5)
        }
        Adt2::C4(r50) => {
            let v2 = (*r50).clone();
            f15(v0, v2)
        }
        Adt2::C5 => {
            Ok(Adt2::C5)
        }
        Adt2::C6 => {
            Ok(Adt2::C0)
        }
        Adt2::C7 => {
            Ok(Adt2::C1)
        }
        Adt2::C8 => {
            Ok(Adt2::C3)
        }
        Adt2::C9 => {
            Ok(Adt2::C1)
        }
        Adt2::C10 => {
            Ok(Adt2::C1)
        }
        Adt2::C11 => {
            Ok(Adt2::C12)
        }
        Adt2::C12 => {
            Ok(Adt2::C5)
        }
        Adt2::C13 => {
            Ok(Adt2::C14)
        }
        Adt2::C14 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f12(v0: Adt0) -> R<Adt1> {
    let m51 = v0;
    match m51 {
        Adt0::C0(v1) => {
            Ok(Adt1::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt1::C1(f43(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt1::C1(f44(v4)?))
        }
        Adt0::C3(r52) => {
            let v5 = (*r52).clone();
            Ok(f41(f45(Str::lit("[")), f45(Str::lit("]")), f42(f46(v5)?)))
        }
        Adt0::C4(r53) => {
            let v6 = (*r53).clone();
            f47(f48(v6.clone())?, f49(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt1::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt1::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f13(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m54 = f50(v0);
    match m54 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f51(v2, v1)
        }
    }
}

pub fn f14(v0: Str) -> Bytes {
    let a55 = v0;
    utf8_encode(a55)
}

pub fn f15(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m56 = f13(f52(v0.clone(), v1)?, f14(Str::lit("items")));
    match m56 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f53(v0, v2)
        }
    }
}

pub fn f16(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m57 = f13(v0, f14(Str::lit("$defs")));
    match m57 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f13(v2, v1)
        }
    }
}

pub fn f17(v0: Fn0, p62: u64, p63: Adt3) -> R<(Adt3, bool)> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v1, v2) = m61;
                let m58 = v1;
                if m58 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m58 - 1;
                    let m60 = {
                        let c59: Fn0 = v0.clone();
                        c59.apply(v2.clone())?
                    };
                    match m60 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
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

pub fn f18(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    f54(v1, v0, v2)
}

pub fn f19(v0: List<Bytes>) -> Bytes {
    f55(f56(v0))
}

pub fn f20(v0: Adt0) -> R<bool> {
    let m64 = f22(v0.clone());
    match m64 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f57(v0, v1)
        }
    }
}

pub fn f21(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m65 = f58(v1.clone());
    match m65 {
        None => {
            if f61(v1.clone(), f14(Str::lit("var"))) {
                Ok(f27(f31(v0, f14(Str::lit("name")))))
            } else {
                if f61(v1.clone(), f14(Str::lit("nat"))) {
                    Ok(f27(f31(v0, f14(Str::lit("value")))))
                } else {
                    if f61(v1.clone(), f14(Str::lit("bool"))) {
                        Ok(f27({
                            let m66 = f51(v0, f14(Str::lit("value")));
                            match m66 {
                                None => {
                                    f14(Str::lit(""))
                                }
                                Some(v4) => {
                                    let m67 = f62(v4);
                                    match m67 {
                                        None => {
                                            f14(Str::lit(""))
                                        }
                                        Some(v5) => {
                                            if v5 {
                                                f14(Str::lit("true"))
                                            } else {
                                                f14(Str::lit("false"))
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("string"))) {
                            Ok(f27(f63(f31(v0, f14(Str::lit("value"))))?))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("bytes"))) {
                                Ok(f27(f30(f14(Str::lit("0x")), f31(v0, f14(Str::lit("hex"))))))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("unit"))) {
                                    Ok(f27(f14(Str::lit("()"))))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("integer"))) {
                                        Ok(f27(f30(f30(f30(f30(f14(Str::lit("(")), f31(v0.clone(), f14(Str::lit("value")))), f14(Str::lit(" : "))), {
                                            let m68 = f64(f31(v0, f14(Str::lit("representation"))));
                                            match m68 {
                                                None => {
                                                    f14(Str::lit(""))
                                                }
                                                Some(v6) => {
                                                    v6
                                                }
                                            }
                                        }), f14(Str::lit(")")))))
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("pair"))) {
                                            Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f14(Str::lit("left"))), List::cons(f27(f14(Str::lit(", "))), List::cons(f32(v2, f14(Str::lit("right"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                        } else {
                                            if if f61(v1.clone(), f14(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f61(v1.clone(), f14(Str::lit("second")))
                                                }
                                            } {
                                                Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("cons"))) {
                                                    Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("head"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("tail"))), List::<Adt1>::nil()))))))
                                                } else {
                                                    if f61(v1.clone(), f14(Str::lit("nil"))) {
                                                        Ok(f65(v1, List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                                                    } else {
                                                        if f61(v1.clone(), f14(Str::lit("if"))) {
                                                            Ok(f29(List::cons(f27(f14(Str::lit("(if "))), List::cons(f32(v2.clone(), f14(Str::lit("condition"))), List::cons(f27(f14(Str::lit(" then "))), List::cons(f32(v2.clone(), f14(Str::lit("then_value"))), List::cons(f27(f14(Str::lit(" else "))), List::cons(f32(v2, f14(Str::lit("else_value"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))))
                                                        } else {
                                                            if f61(v1.clone(), f14(Str::lit("let"))) {
                                                                Ok(f29(List::cons(f27(f14(Str::lit("(let "))), List::cons(f32(v2.clone(), f14(Str::lit("binder"))), List::cons(f27(f14(Str::lit(" := "))), List::cons(f32(v2.clone(), f14(Str::lit("value"))), List::cons(f27(f14(Str::lit("; "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))))
                                                            } else {
                                                                if f61(v1.clone(), f14(Str::lit("forall"))) {
                                                                    Ok(f29(List::cons(f27(f14(Str::lit("(forall ("))), List::cons(f32(v2.clone(), f14(Str::lit("binder"))), List::cons(f27(f14(Str::lit("), "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                                                } else {
                                                                    if f61(v1.clone(), f14(Str::lit("lambda"))) {
                                                                        let v7: List<Adt1> = f66(v0, f14(Str::lit("captures")));
                                                                        Ok(f29(List::cons(f27(f14(Str::lit("(fun "))), {
                                                                            let a74 = {
                                                                                let a70 = 0u64;
                                                                                let a71 = {
                                                                                    let a69 = v7.clone();
                                                                                    length_list(a69)
                                                                                };
                                                                                if nat_lt(a70, a71) {
                                                                                    List::cons(f27(f14(Str::lit("["))), {
                                                                                        let a72 = f38(v7, f14(Str::lit(", ")));
                                                                                        let a73 = List::cons(f27(f14(Str::lit("]"))), List::cons(f27(f14(Str::lit(" "))), List::<Adt1>::nil()));
                                                                                        append_list(a72, a73)
                                                                                    })
                                                                                } else {
                                                                                    List::<Adt1>::nil()
                                                                                }
                                                                            };
                                                                            let a75 = List::cons(f32(v2.clone(), f14(Str::lit("parameters"))), List::cons(f27(f14(Str::lit("=> "))), List::cons(f32(v2, f14(Str::lit("body"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()))));
                                                                            append_list(a74, a75)
                                                                        })))
                                                                    } else {
                                                                        if f61(v1.clone(), f14(Str::lit("apply"))) {
                                                                            Ok(f65(f14(Str::lit("apply")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("function"))), List::cons(f32(v2, f14(Str::lit("arguments"))), List::<Adt1>::nil())))))
                                                                        } else {
                                                                            if f61(v1.clone(), f14(Str::lit("project"))) {
                                                                                Ok(f65(f14(Str::lit("project")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::cons(f27(f30(f14(Str::lit(" ")), f31(v0, f14(Str::lit("field"))))), List::<Adt1>::nil())))))
                                                                            } else {
                                                                                if f61(v1.clone(), f14(Str::lit("call"))) {
                                                                                    Ok(f29(List::cons(f27(f30(f14(Str::lit("(")), f67(v0.clone(), f14(Str::lit("function")))?)), {
                                                                                        let a76 = f68(v0, v2.clone());
                                                                                        let a77 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()));
                                                                                        append_list(a76, a77)
                                                                                    })))
                                                                                } else {
                                                                                    if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                                                                        Ok(f65(f14(Str::lit("ctor")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("constructor")))?)), {
                                                                                            let a78 = f68(v0, v2.clone());
                                                                                            let a79 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::<Adt1>::nil());
                                                                                            append_list(a78, a79)
                                                                                        })))
                                                                                    } else {
                                                                                        if f61(v1.clone(), f14(Str::lit("function_ref"))) {
                                                                                            Ok(f65(f14(Str::lit("ref")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("function")))?)), f68(v0, v2))))
                                                                                        } else {
                                                                                            if f61(v1.clone(), f14(Str::lit("record"))) {
                                                                                                Ok(f65(f14(Str::lit("record")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("type")))?)), {
                                                                                                    let a80 = f68(v0, v2.clone());
                                                                                                    let a81 = List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("fields"))), List::<Adt1>::nil()));
                                                                                                    append_list(a80, a81)
                                                                                                })))
                                                                                            } else {
                                                                                                if f61(v1.clone(), f14(Str::lit("primitive"))) {
                                                                                                    Ok(f29(List::cons(f27(f30(f14(Str::lit("(prim ")), f31(v0, f14(Str::lit("operation"))))), List::cons(f32(v2.clone(), f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(" : "))), List::cons(f32(v2, f14(Str::lit("result"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                                                                                } else {
                                                                                                    if f61(v1, f14(Str::lit("checked_apply"))) {
                                                                                                        Ok(f65(f14(Str::lit("checked")), List::cons(f27(f30(f14(Str::lit(" ")), f67(v0.clone(), f14(Str::lit("model")))?)), {
                                                                                                            let a84 = f68(v0.clone(), v2.clone());
                                                                                                            let a85 = List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(" with "))), List::cons(f27(f14(Str::lit("["))), {
                                                                                                                let a82 = f38(f66(v0, f14(Str::lit("checks"))), f14(Str::lit(", ")));
                                                                                                                let a83 = List::cons(f27(f14(Str::lit("]"))), List::<Adt1>::nil());
                                                                                                                append_list(a82, a83)
                                                                                                            })));
                                                                                                            append_list(a84, a85)
                                                                                                        })))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C6(std::rc::Rc::new(Adt1::C3(2u64, std::rc::Rc::new(f29(List::cons(f27(f14(Str::lit("(match "))), List::cons(f32(v2.clone(), f14(Str::lit("scrutinee"))), List::cons(f32(v2, f14(Str::lit("branches"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil()))))))))))
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
        Some(v3) => {
            Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f59(v1.clone())), List::cons(f27(f30(f30(f14(Str::lit(" ")), v3), f14(Str::lit(" ")))), List::cons(f32(v2, f60(v1)), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
        }
    }
}

pub fn f22(v0: Adt0) -> Option<Bytes> {
    f69(f51(v0, f14(Str::lit("kind"))))
}

pub fn f23(v0: Adt0, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    f47(f48(v0)?, v1)
}

pub fn f24(v0: Adt0) -> R<bool> {
    let m86 = f22(v0.clone());
    match m86 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f70(v0, v1)
        }
    }
}

pub fn f25(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt1)>) -> R<Adt1> {
    let m87 = f71(v1.clone());
    match m87 {
        None => {
            if f61(v1.clone(), f14(Str::lit("parameter"))) {
                Ok(f27(f30(f14(Str::lit("'")), f31(v0, f14(Str::lit("name"))))))
            } else {
                if f61(v1.clone(), f14(Str::lit("list"))) {
                    Ok(f65(f14(Str::lit("List")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                } else {
                    if f61(v1.clone(), f14(Str::lit("set"))) {
                        Ok(f65(f14(Str::lit("Set")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("element"))), List::<Adt1>::nil()))))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("option"))) {
                            Ok(f65(f14(Str::lit("Option")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("result"))) {
                                Ok(f65(f14(Str::lit("Result")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("ok"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("error"))), List::<Adt1>::nil()))))))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("map"))) {
                                    Ok(f65(f14(Str::lit("Map")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("key"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("value"))), List::<Adt1>::nil()))))))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("product"))) {
                                        Ok(f29(List::cons(f27(f14(Str::lit("("))), List::cons(f32(v2.clone(), f14(Str::lit("left"))), List::cons(f27(f14(Str::lit(" * "))), List::cons(f32(v2, f14(Str::lit("right"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))))
                                    } else {
                                        if f61(v1, f14(Str::lit("function"))) {
                                            Ok(f65(f14(Str::lit("Fn")), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2.clone(), f14(Str::lit("parameters"))), List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v2, f14(Str::lit("result"))), List::<Adt1>::nil()))))))
                                        } else {
                                            let v4: Bytes = f67(v0.clone(), f14(Str::lit("member")))?;
                                            let m88 = f72(f51(v0, f14(Str::lit("arguments"))));
                                            match m88 {
                                                None => {
                                                    Ok(f27(v4))
                                                }
                                                Some(v5) => {
                                                    let a89 = f73(v5)?;
                                                    let a90 = 0u64;
                                                    if nat_eq(a89, a90) {
                                                        Ok(f27(v4))
                                                    } else {
                                                        Ok(f29(List::cons(f27(f30(f14(Str::lit("(")), v4)), List::cons(f32(v2, f14(Str::lit("arguments"))), List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil())))))
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
        Some(v3) => {
            Ok(f27(v3))
        }
    }
}

pub fn f26(v0: Adt0) -> R<Option<Bytes>> {
    let m91 = f50(v0);
    match m91 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m92 = f69(f51(v1.clone(), f14(Str::lit("name"))));
            match m92 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let v3: Option<Adt0> = f51(v1.clone(), f14(Str::lit("module")));
                    if f74({
                        let a93 = f73(v1)?;
                        let a94 = if f75(v3.clone()) {
                            2u64
                        } else {
                            1u64
                        };
                        nat_eq(a93, a94)
                    }, f76(v2.clone())?) {
                        let m95 = v3;
                        match m95 {
                            None => {
                                Ok(Some(v2))
                            }
                            Some(v4) => {
                                let m96 = f77(v4);
                                match m96 {
                                    None => {
                                        Ok(None::<Bytes>)
                                    }
                                    Some(v5) => {
                                        if f76(v5.clone())? {
                                            Ok(Some(f30(f30(v5, f14(Str::lit("::"))), v2)))
                                        } else {
                                            Ok(None::<Bytes>)
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f27(v0: Bytes) -> Adt1 {
    Adt1::C1(v0)
}

pub fn f28(v0: Adt0) -> R<bool> {
    let m97 = f50(v0);
    match m97 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m98 = f69(f51(v1.clone(), f14(Str::lit("name"))));
            match m98 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    Ok(f74(f75(f51(v1.clone(), f14(Str::lit("type")))), f74({
                        let a99 = f73(v1)?;
                        let a100 = 2u64;
                        nat_eq(a99, a100)
                    }, f76(v2)?)))
                }
            }
        }
    }
}

pub fn f29(v0: List<Adt1>) -> Adt1 {
    f78(Fn2::F79, Adt1::C0, f42(v0))
}

pub fn f30(v0: Bytes, v1: Bytes) -> Bytes {
    let a101 = v0;
    let a102 = v1;
    append_bytes(a101, a102)
}

pub fn f31(v0: Adt0, v1: Bytes) -> Bytes {
    let m103 = f69(f51(v0, v1));
    match m103 {
        None => {
            f14(Str::lit(""))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt1)>, v1: Bytes) -> Adt1 {
    f80(Fn3::F81(v1), Adt1::C0, v0)
}

pub fn f33(v0: Adt0, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    Ok(f29(List::cons(f27(f30(f14(Str::lit("| ")), f67(v0.clone(), f14(Str::lit("constructor")))?)), {
        let a104 = f37(f66(v0, f14(Str::lit("binders"))), f14(Str::lit(" ")), f14(Str::lit("")));
        let a105 = List::cons(f27(f14(Str::lit(" => "))), List::cons(f32(v1, f14(Str::lit("body"))), List::<Adt1>::nil()));
        append_list(a104, a105)
    })))
}

pub fn f34(v0: Adt5) -> bool {
    let m106 = v0;
    match m106 {
        Adt5::C0(_) => {
            false
        }
        Adt5::C1(_) => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
        Adt5::C5 => {
            false
        }
        Adt5::C6 => {
            true
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt5, v2: Bytes) -> R<Adt2> {
    let m107 = v1;
    match m107 {
        Adt5::C0(v3) => {
            f82(v0, v3, v2)
        }
        Adt5::C1(v4) => {
            Ok(f83(v4, v2))
        }
        Adt5::C2 => {
            Ok(f84(v2))
        }
        Adt5::C3 => {
            if f61(v2, f14(Str::lit("type"))) {
                Ok(Adt2::C1)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt5::C4 => {
            if f61(v2, f14(Str::lit("value"))) {
                Ok(Adt2::C0)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt5::C5 => {
            if f61(v2.clone(), f14(Str::lit("body"))) {
                Ok(Adt2::C0)
            } else {
                if f61(v2, f14(Str::lit("constructor"))) {
                    Ok(Adt2::C2)
                } else {
                    Ok(Adt2::C5)
                }
            }
        }
        Adt5::C6 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Option<Adt0>> {
    let v3: Option<Bytes> = f48(v1)?;
    let m108 = v2.clone();
    match m108 {
        Adt2::C0 => {
            let m109 = v3;
            match m109 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    let m110 = f85(v0.clone(), v2)?;
                    match m110 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v5) => {
                            f86(v0, v5, v4)
                        }
                    }
                }
            }
        }
        Adt2::C1 => {
            let m111 = v3;
            match m111 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v6) => {
                    let m112 = f85(v0.clone(), v2)?;
                    match m112 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v7) => {
                            f86(v0, v7, v6)
                        }
                    }
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(f16(v0, f14(Str::lit("parameter"))))
        }
        Adt2::C4(r114) => {
            let v8 = (*r114).clone();
            let m113 = v3;
            match m113 {
                None => {
                    Ok(Some(v8))
                }
                Some(v9) => {
                    if f87(v0.clone(), v8.clone())? {
                        f86(v0, v8, v9)
                    } else {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f37(v0: List<Adt1>, v1: Bytes, v2: Bytes) -> List<Adt1> {
    f42(f39(Fn1::F88(v2, v1), List::<Adt1>::nil(), v0))
}

pub fn f38(v0: List<Adt1>, v1: Bytes) -> List<Adt1> {
    f42({
        let (_, h115) = f89(Fn4::F90(v1), (true, List::<Adt1>::nil()), v0);
        h115
    })
}

pub fn f39(v0: Fn1, p119: List<Adt1>, p120: List<Adt1>) -> List<Adt1> {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m116 = v2;
                match m116.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c117: Fn1 = v0.clone();
                            c117.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    let a121 = v0;
    let a122 = List::cons(Adt1::C4, List::cons(v1, List::<Adt1>::nil()));
    append_list(a121, a122)
}

pub fn f41(v0: Bytes, v1: Bytes, v2: List<Adt1>) -> Adt1 {
    let m123 = v2.clone();
    match m123.uncons() {
        None => {
            Adt1::C1(f91(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt1) = f92(Fn5::F93, (true, Adt1::C0), v2);
            Adt1::C6(std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v0)), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C3(2u64, std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new({
                let (_, h124) = v5;
                h124
            }), std::rc::Rc::new(Adt1::C0))))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v1)), std::rc::Rc::new(Adt1::C0))))))))))
        }
    }
}

pub fn f42(v0: List<Adt1>) -> List<Adt1> {
    f39(Fn1::F94, List::<Adt1>::nil(), v0)
}

pub fn f43(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a125 = 0u64;
        let a126 = v1;
        nat_lt(a125, a126)
    } else {
        false
    } {
        let a127 = Bytes::lit(&[0x2d]);
        let a128 = f95(v1);
        append_bytes(a127, a128)
    } else {
        f95(v1)
    }
}

pub fn f44(v0: Bytes) -> R<Bytes> {
    if f76(v0.clone())? {
        Ok(v0)
    } else {
        f63(v0)
    }
}

pub fn f45(v0: Str) -> Bytes {
    let a129 = v0;
    utf8_encode(a129)
}

pub fn f46(v0: Adt0) -> R<List<Adt1>> {
    let m130 = v0;
    match m130 {
        Adt0::C0(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C6(r131, r132) => {
            let v7 = (*r131).clone();
            let v8 = (*r132).clone();
            Ok(List::cons(f12(v7)?, f46(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt1>::nil())
        }
    }
}

pub fn f47(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    let v2: List<(Bytes, Adt1)> = f96(Fn6::F97(v0.clone()), List::<(Bytes, Adt1)>::nil(), v1)?;
    Ok(f41({
        let m133 = v0;
        match m133 {
            None => {
                f45(Str::lit("{"))
            }
            Some(v3) => {
                f91(v3, f45(Str::lit("{")))
            }
        }
    }, f45(Str::lit("}")), f98(Fn7::F99, List::<Adt1>::nil(), f100(v2)?)))
}

pub fn f48(p141: Adt0) -> R<Option<Bytes>> {
    let m140 = p141;
    {
        let mut m140 = m140;
        loop {
            let step = {
                let v0 = m140;
                let m134 = v0;
                match m134 {
                    Adt0::C0(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C1(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C2(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C3(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C6(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C7(v9, r138, r139) => {
                        let v10 = (*r138).clone();
                        let v11 = (*r139).clone();
                        let a135 = v9;
                        let a136 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a135, a136) {
                            let m137 = v10;
                            match m137 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f76(v12.clone())? {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt0::C3(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C4(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C5 => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C6(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C7(_, _, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt0, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Adt0) -> R<List<(Bytes, Adt1)>> {
    let m142 = v0;
    match m142 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C7(v9, r143, r144) => {
            let v10 = (*r143).clone();
            let v11 = (*r144).clone();
            Ok(List::cons((v9, f12(v10)?), f49(v11)?))
        }
    }
}

pub fn f50(v0: Adt0) -> Option<Adt0> {
    let m145 = v0;
    match m145 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(_) => {
            None::<Adt0>
        }
        Adt0::C4(r146) => {
            let v1 = (*r146).clone();
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f51(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m149 = {
        let (h148, _) = f101(Fn8::F102(v1), 1000000u64, Adt6::C0(std::rc::Rc::new(v0), None::<Adt0>));
        h148
    };
    match m149 {
        Adt6::C0(_, h147) => {
            h147
        }
    }
}

pub fn f52(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h150, _) = f103(Fn9::F104(std::rc::Rc::new(v0)), 4096u64, v1)?;
    Ok(h150)
}

pub fn f53(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m151 = f69(f13(v1.clone(), f14(Str::lit("$ref"))));
    match m151 {
        None => {
            Ok(Adt2::C4(std::rc::Rc::new(v1)))
        }
        Some(v2) => {
            let v3: Bytes = f105(v2)?;
            if f61(v3.clone(), f14(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f61(v3.clone(), f14(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f61(v3.clone(), f14(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f61(v3, f14(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(std::rc::Rc::new(f52(v0, v1)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f54(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m153 = match v2.clone() {
        Adt3::C0(_, _, h152) => {
            h152
        }
    };
    match m153.uncons() {
        None => {
            Ok(None::<Adt3>)
        }
        Some((v3, v4)) => {
            Ok(Some(f106(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f55(v0: List<Bytes>) -> Bytes {
    let m156 = {
        let (h154, _) = f107(Fn10::F108, {
            let a155 = v0.clone();
            length_list(a155)
        }, v0);
        h154
    };
    match m156.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f56(v0: List<Bytes>) -> List<Bytes> {
    f109(Fn11::F110, List::<Bytes>::nil(), v0)
}

pub fn f57(v0: Adt0, v1: Bytes) -> R<bool> {
    let m157 = f58(v1.clone());
    match m157 {
        None => {
            if f61(v1.clone(), f14(Str::lit("var"))) {
                Ok(f74(f111(v0.clone(), f112(Str::lit("name")))?, {
                    let m158 = f69(f51(v0, f14(Str::lit("name"))));
                    match m158 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f113(v3)?
                        }
                    }
                }))
            } else {
                if f61(v1.clone(), f14(Str::lit("nat"))) {
                    Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                        let m159 = f69(f51(v0, f14(Str::lit("value"))));
                        match m159 {
                            None => {
                                false
                            }
                            Some(v4) => {
                                f114(v4)?
                            }
                        }
                    }))
                } else {
                    if f61(v1.clone(), f14(Str::lit("bool"))) {
                        Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                            let m160 = f51(v0, f14(Str::lit("value")));
                            match m160 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    let m161 = f62(v5);
                                    match m161 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("string"))) {
                            Ok(f74(f111(v0.clone(), f112(Str::lit("value")))?, {
                                let m162 = f69(f51(v0, f14(Str::lit("value"))));
                                match m162 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("bytes"))) {
                                Ok(f74(f111(v0.clone(), f112(Str::lit("hex")))?, {
                                    let m163 = f69(f51(v0, f14(Str::lit("hex"))));
                                    match m163 {
                                        None => {
                                            false
                                        }
                                        Some(v8) => {
                                            f115(v8)?
                                        }
                                    }
                                }))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("unit"))) {
                                    f111(v0, List::cons(f14(Str::lit("kind")), List::<Bytes>::nil()))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("integer"))) {
                                        Ok(f74(f111(v0.clone(), f116(Str::lit("representation"), Str::lit("value")))?, {
                                            let m164 = f69(f51(v0.clone(), f14(Str::lit("value"))));
                                            match m164 {
                                                None => {
                                                    false
                                                }
                                                Some(v9) => {
                                                    let m165 = f69(f51(v0, f14(Str::lit("representation"))));
                                                    match m165 {
                                                        None => {
                                                            false
                                                        }
                                                        Some(v10) => {
                                                            f74(f75({
                                                                let m166 = f64(v10);
                                                                match m166 {
                                                                    None => {
                                                                        None::<Adt0>
                                                                    }
                                                                    Some(_) => {
                                                                        Some(Adt0::C5)
                                                                    }
                                                                }
                                                            }), f117(v9)?)
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("pair"))) {
                                            f111(v0, f116(Str::lit("left"), Str::lit("right")))
                                        } else {
                                            if if f61(v1.clone(), f14(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f61(v1.clone(), f14(Str::lit("second")))
                                                }
                                            } {
                                                f111(v0, f112(Str::lit("value")))
                                            } else {
                                                if f61(v1.clone(), f14(Str::lit("cons"))) {
                                                    f111(v0, f116(Str::lit("head"), Str::lit("tail")))
                                                } else {
                                                    if f61(v1.clone(), f14(Str::lit("nil"))) {
                                                        f111(v0, f112(Str::lit("element")))
                                                    } else {
                                                        if f61(v1.clone(), f14(Str::lit("if"))) {
                                                            f111(v0, f118(Str::lit("condition"), Str::lit("then_value"), Str::lit("else_value")))
                                                        } else {
                                                            if f61(v1.clone(), f14(Str::lit("let"))) {
                                                                Ok(f74(f111(v0.clone(), f118(Str::lit("binder"), Str::lit("value"), Str::lit("body")))?, f119(f51(v0, f14(Str::lit("binder"))))?))
                                                            } else {
                                                                if f61(v1.clone(), f14(Str::lit("forall"))) {
                                                                    Ok(f74(f111(v0.clone(), f116(Str::lit("binder"), Str::lit("body")))?, f119(f51(v0, f14(Str::lit("binder"))))?))
                                                                } else {
                                                                    if f61(v1.clone(), f14(Str::lit("lambda"))) {
                                                                        Ok(f74(f111(v0.clone(), f118(Str::lit("parameters"), Str::lit("captures"), Str::lit("body")))?, f74(f120(v0.clone(), f14(Str::lit("parameters")), Fn12::F121)?, f120(v0, f14(Str::lit("captures")), Fn12::F122)?)))
                                                                    } else {
                                                                        if f61(v1.clone(), f14(Str::lit("apply"))) {
                                                                            Ok(f74(f111(v0.clone(), f116(Str::lit("function"), Str::lit("arguments")))?, f123(v0, f14(Str::lit("arguments")))))
                                                                        } else {
                                                                            if f61(v1.clone(), f14(Str::lit("project"))) {
                                                                                Ok(f74(f111(v0.clone(), f116(Str::lit("value"), Str::lit("field")))?, {
                                                                                    let m167 = f69(f51(v0, f14(Str::lit("field"))));
                                                                                    match m167 {
                                                                                        None => {
                                                                                            false
                                                                                        }
                                                                                        Some(v12) => {
                                                                                            f76(v12)?
                                                                                        }
                                                                                    }
                                                                                }))
                                                                            } else {
                                                                                if f61(v1.clone(), f14(Str::lit("call"))) {
                                                                                    Ok(f74(f124(v0.clone(), f116(Str::lit("function"), Str::lit("arguments")))?, f74({
                                                                                        let m168 = f51(v0.clone(), f14(Str::lit("function")));
                                                                                        match m168 {
                                                                                            None => {
                                                                                                false
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let m169 = f26(v13.clone())?;
                                                                                                match m169 {
                                                                                                    None => {
                                                                                                        false
                                                                                                    }
                                                                                                    Some(_) => {
                                                                                                        let m170 = f69(f13(v13, f14(Str::lit("name"))));
                                                                                                        match m170 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v15) => {
                                                                                                                f113(v15)?
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }, f74(f125(v0.clone())?, f123(v0, f14(Str::lit("arguments")))))))
                                                                                } else {
                                                                                    if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                                                                        Ok(f74(f124(v0.clone(), f116(Str::lit("constructor"), Str::lit("arguments")))?, f74(f126(f51(v0.clone(), f14(Str::lit("constructor"))))?, f74(f125(v0.clone())?, f123(v0, f14(Str::lit("arguments")))))))
                                                                                    } else {
                                                                                        if f61(v1.clone(), f14(Str::lit("function_ref"))) {
                                                                                            Ok(f74(f124(v0.clone(), f112(Str::lit("function")))?, f74(f126(f51(v0.clone(), f14(Str::lit("function"))))?, f125(v0)?)))
                                                                                        } else {
                                                                                            if f61(v1.clone(), f14(Str::lit("record"))) {
                                                                                                Ok(f74(f124(v0.clone(), f116(Str::lit("type"), Str::lit("fields")))?, f74(f126(f51(v0.clone(), f14(Str::lit("type"))))?, f74(f125(v0.clone())?, f120(v0, f14(Str::lit("fields")), Fn12::F127)?))))
                                                                                            } else {
                                                                                                if f61(v1.clone(), f14(Str::lit("primitive"))) {
                                                                                                    Ok(f74(f111(v0.clone(), f118(Str::lit("operation"), Str::lit("arguments"), Str::lit("result")))?, f74({
                                                                                                        let m171 = f69(f51(v0.clone(), f14(Str::lit("operation"))));
                                                                                                        match m171 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v16) => {
                                                                                                                f76(v16)?
                                                                                                            }
                                                                                                        }
                                                                                                    }, f123(v0, f14(Str::lit("arguments"))))))
                                                                                                } else {
                                                                                                    if f61(v1.clone(), f14(Str::lit("checked_apply"))) {
                                                                                                        Ok(f74(f124(v0.clone(), f118(Str::lit("model"), Str::lit("arguments"), Str::lit("checks")))?, f74(f126(f51(v0.clone(), f14(Str::lit("model"))))?, f74(f125(v0.clone())?, f74(f123(v0.clone(), f14(Str::lit("arguments"))), f120(v0, f14(Str::lit("checks")), Fn12::F128)?)))))
                                                                                                    } else {
                                                                                                        if f61(v1, f14(Str::lit("match"))) {
                                                                                                            Ok(f74(f111(v0.clone(), f116(Str::lit("scrutinee"), Str::lit("branches")))?, f120(v0, f14(Str::lit("branches")), Fn12::F129)?))
                                                                                                        } else {
                                                                                                            Ok(false)
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
        Some(_) => {
            f111(v0, List::cons(f14(Str::lit("kind")), List::cons(f59(v1.clone()), List::cons(f60(v1), List::<Bytes>::nil()))))
        }
    }
}

pub fn f58(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("add"))) {
        Some(f14(Str::lit("+")))
    } else {
        if f61(v0.clone(), f14(Str::lit("eq"))) {
            Some(f14(Str::lit("=")))
        } else {
            if f61(v0.clone(), f14(Str::lit("le"))) {
                Some(f14(Str::lit("<=")))
            } else {
                if f61(v0.clone(), f14(Str::lit("lt"))) {
                    Some(f14(Str::lit("<")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("beq"))) {
                        Some(f14(Str::lit("=?")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("ble"))) {
                            Some(f14(Str::lit("<=?")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("blt"))) {
                                Some(f14(Str::lit("<?")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("and"))) {
                                    Some(f14(Str::lit("&&")))
                                } else {
                                    if f61(v0.clone(), f14(Str::lit("or"))) {
                                        Some(f14(Str::lit("||")))
                                    } else {
                                        if f61(v0.clone(), f14(Str::lit("prop_and"))) {
                                            Some(f14(Str::lit("/\\")))
                                        } else {
                                            if f61(v0.clone(), f14(Str::lit("iff"))) {
                                                Some(f14(Str::lit("<->")))
                                            } else {
                                                if f61(v0, f14(Str::lit("implies"))) {
                                                    Some(f14(Str::lit("->")))
                                                } else {
                                                    None::<Bytes>
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

pub fn f59(v0: Bytes) -> Bytes {
    if f61(v0, f14(Str::lit("implies"))) {
        f14(Str::lit("premise"))
    } else {
        f14(Str::lit("left"))
    }
}

pub fn f60(v0: Bytes) -> Bytes {
    if f61(v0, f14(Str::lit("implies"))) {
        f14(Str::lit("conclusion"))
    } else {
        f14(Str::lit("right"))
    }
}

pub fn f61(v0: Bytes, v1: Bytes) -> bool {
    let a172 = v0;
    let a173 = v1;
    equal(a172, a173)
}

pub fn f62(v0: Adt0) -> Option<bool> {
    let m174 = v0;
    match m174 {
        Adt0::C0(v1) => {
            Some(v1)
        }
        Adt0::C1(_, _) => {
            None::<bool>
        }
        Adt0::C2(_) => {
            None::<bool>
        }
        Adt0::C3(_) => {
            None::<bool>
        }
        Adt0::C4(_) => {
            None::<bool>
        }
        Adt0::C5 => {
            None::<bool>
        }
        Adt0::C6(_, _) => {
            None::<bool>
        }
        Adt0::C7(_, _, _) => {
            None::<bool>
        }
    }
}

pub fn f63(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a175 = v0.clone();
        length_bytes(a175)
    };
    let v2: Adt7 = {
        let (h176, _) = f130(Fn13::F131(v1, v0), {
            let a177 = v1;
            let a178 = 1u64;
            nat_add(a177, a178)?
        }, Adt7::C0(0u64, List::<Bytes>::nil()))?;
        h176
    };
    Ok(f132(f132(f133(Str::lit("\"")), f134(match v2 {
        Adt7::C0(_, h179) => {
            h179
        }
    })), f133(Str::lit("\""))))
}

pub fn f64(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("int"))) {
        Some(f14(Str::lit("Int")))
    } else {
        if f61(v0.clone(), f14(Str::lit("int8"))) {
            Some(f14(Str::lit("Int8")))
        } else {
            if f61(v0.clone(), f14(Str::lit("int16"))) {
                Some(f14(Str::lit("Int16")))
            } else {
                if f61(v0.clone(), f14(Str::lit("int32"))) {
                    Some(f14(Str::lit("Int32")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("int64"))) {
                        Some(f14(Str::lit("Int64")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("uint8"))) {
                            Some(f14(Str::lit("UInt8")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("uint16"))) {
                                Some(f14(Str::lit("UInt16")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("uint32"))) {
                                    Some(f14(Str::lit("UInt32")))
                                } else {
                                    if f61(v0, f14(Str::lit("uint64"))) {
                                        Some(f14(Str::lit("UInt64")))
                                    } else {
                                        None::<Bytes>
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

pub fn f65(v0: Bytes, v1: List<Adt1>) -> Adt1 {
    f29(List::cons(f27(f30(f14(Str::lit("(")), v0)), {
        let a180 = v1;
        let a181 = List::cons(f27(f14(Str::lit(")"))), List::<Adt1>::nil());
        append_list(a180, a181)
    }))
}

pub fn f66(v0: Adt0, v1: Bytes) -> List<Adt1> {
    let m182 = f72(f51(v0, v1));
    match m182 {
        None => {
            List::<Adt1>::nil()
        }
        Some(v2) => {
            f135(Fn14::F136, List::<Adt1>::nil(), f137(v2))
        }
    }
}

pub fn f67(v0: Adt0, v1: Bytes) -> R<Bytes> {
    let m183 = f51(v0, v1);
    match m183 {
        None => {
            Ok(f14(Str::lit("")))
        }
        Some(v2) => {
            let m184 = f26(v2)?;
            match m184 {
                None => {
                    Ok(f14(Str::lit("")))
                }
                Some(v3) => {
                    Ok(v3)
                }
            }
        }
    }
}

pub fn f68(v0: Adt0, v1: List<(Bytes, Adt1)>) -> List<Adt1> {
    if f75(f51(v0, f14(Str::lit("type_arguments")))) {
        List::cons(f27(f14(Str::lit(" "))), List::cons(f32(v1, f14(Str::lit("type_arguments"))), List::<Adt1>::nil()))
    } else {
        List::<Adt1>::nil()
    }
}

pub fn f69(v0: Option<Adt0>) -> Option<Bytes> {
    let m185 = v0;
    match m185 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f77(v1)
        }
    }
}

pub fn f70(v0: Adt0, v1: Bytes) -> R<bool> {
    let m186 = f71(v1.clone());
    match m186 {
        None => {
            if f61(v1.clone(), f14(Str::lit("parameter"))) {
                Ok(f74(f111(v0.clone(), f112(Str::lit("name")))?, {
                    let m187 = f69(f51(v0, f14(Str::lit("name"))));
                    match m187 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f76(v3)?
                        }
                    }
                }))
            } else {
                if if f61(v1.clone(), f14(Str::lit("list"))) {
                    true
                } else {
                    f61(v1.clone(), f14(Str::lit("set")))
                } {
                    f111(v0, f112(Str::lit("element")))
                } else {
                    if f61(v1.clone(), f14(Str::lit("option"))) {
                        f111(v0, f112(Str::lit("value")))
                    } else {
                        if f61(v1.clone(), f14(Str::lit("result"))) {
                            f111(v0, f116(Str::lit("ok"), Str::lit("error")))
                        } else {
                            if f61(v1.clone(), f14(Str::lit("map"))) {
                                f111(v0, f116(Str::lit("key"), Str::lit("value")))
                            } else {
                                if f61(v1.clone(), f14(Str::lit("product"))) {
                                    f111(v0, f116(Str::lit("left"), Str::lit("right")))
                                } else {
                                    if f61(v1.clone(), f14(Str::lit("function"))) {
                                        Ok(f74(f111(v0.clone(), f116(Str::lit("parameters"), Str::lit("result")))?, f123(v0, f14(Str::lit("parameters")))))
                                    } else {
                                        if f61(v1, f14(Str::lit("named"))) {
                                            Ok(f74(f111(v0.clone(), f116(Str::lit("member"), Str::lit("arguments")))?, {
                                                let m188 = f51(v0.clone(), f14(Str::lit("member")));
                                                match m188 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(v4) => {
                                                        let m189 = f50(v4.clone());
                                                        match m189 {
                                                            None => {
                                                                false
                                                            }
                                                            Some(v5) => {
                                                                let m190 = f69(f51(v5.clone(), f14(Str::lit("name"))));
                                                                match m190 {
                                                                    None => {
                                                                        false
                                                                    }
                                                                    Some(v6) => {
                                                                        f74(!f74(!f75(f51(v5, f14(Str::lit("module")))), f138(v6)), f74(f123(v0, f14(Str::lit("arguments"))), f75({
                                                                            let m191 = f26(v4)?;
                                                                            match m191 {
                                                                                None => {
                                                                                    None::<Adt0>
                                                                                }
                                                                                Some(_) => {
                                                                                    Some(Adt0::C5)
                                                                                }
                                                                            }
                                                                        })))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            f111(v0, List::cons(f14(Str::lit("kind")), List::<Bytes>::nil()))
        }
    }
}

pub fn f71(v0: Bytes) -> Option<Bytes> {
    if f61(v0.clone(), f14(Str::lit("bool"))) {
        Some(f14(Str::lit("Bool")))
    } else {
        if f61(v0.clone(), f14(Str::lit("bytes"))) {
            Some(f14(Str::lit("Bytes")))
        } else {
            if f61(v0.clone(), f14(Str::lit("int"))) {
                Some(f14(Str::lit("Int")))
            } else {
                if f61(v0.clone(), f14(Str::lit("int8"))) {
                    Some(f14(Str::lit("Int8")))
                } else {
                    if f61(v0.clone(), f14(Str::lit("int16"))) {
                        Some(f14(Str::lit("Int16")))
                    } else {
                        if f61(v0.clone(), f14(Str::lit("int32"))) {
                            Some(f14(Str::lit("Int32")))
                        } else {
                            if f61(v0.clone(), f14(Str::lit("int64"))) {
                                Some(f14(Str::lit("Int64")))
                            } else {
                                if f61(v0.clone(), f14(Str::lit("nat"))) {
                                    Some(f14(Str::lit("Nat")))
                                } else {
                                    if f61(v0.clone(), f14(Str::lit("ordering"))) {
                                        Some(f14(Str::lit("Ordering")))
                                    } else {
                                        if f61(v0.clone(), f14(Str::lit("prop"))) {
                                            Some(f14(Str::lit("Prop")))
                                        } else {
                                            if f61(v0.clone(), f14(Str::lit("string"))) {
                                                Some(f14(Str::lit("String")))
                                            } else {
                                                if f61(v0.clone(), f14(Str::lit("type"))) {
                                                    Some(f14(Str::lit("Type")))
                                                } else {
                                                    if f61(v0.clone(), f14(Str::lit("uint8"))) {
                                                        Some(f14(Str::lit("UInt8")))
                                                    } else {
                                                        if f61(v0.clone(), f14(Str::lit("uint16"))) {
                                                            Some(f14(Str::lit("UInt16")))
                                                        } else {
                                                            if f61(v0.clone(), f14(Str::lit("uint32"))) {
                                                                Some(f14(Str::lit("UInt32")))
                                                            } else {
                                                                if f61(v0.clone(), f14(Str::lit("uint64"))) {
                                                                    Some(f14(Str::lit("UInt64")))
                                                                } else {
                                                                    if f61(v0.clone(), f14(Str::lit("unit"))) {
                                                                        Some(f14(Str::lit("Unit")))
                                                                    } else {
                                                                        if f61(v0.clone(), f14(Str::lit("contract_violation"))) {
                                                                            Some(f14(Str::lit("ContractViolation")))
                                                                        } else {
                                                                            if f61(v0, f14(Str::lit("reasoning_failure"))) {
                                                                                Some(f14(Str::lit("ReasoningFailure")))
                                                                            } else {
                                                                                None::<Bytes>
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

pub fn f72(v0: Option<Adt0>) -> Option<Adt0> {
    let m192 = v0;
    match m192 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f139(v1)
        }
    }
}

pub fn f73(v0: Adt0) -> R<u64> {
    let m195 = {
        let (h194, _) = f140(Fn15::F141, 1000000u64, Adt8::C0(std::rc::Rc::new(v0), 0u64))?;
        h194
    };
    match m195 {
        Adt8::C0(_, h193) => {
            Ok(h193)
        }
    }
}

pub fn f74(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f75(v0: Option<Adt0>) -> bool {
    let m196 = v0;
    match m196 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f76(v0: Bytes) -> R<bool> {
    let a198 = {
        let a197 = v0.clone();
        length_bytes(a197)
    };
    let a199 = 0u64;
    if nat_eq(a198, a199) {
        Ok(false)
    } else {
        if f142(f143(v0.clone(), 0u64)?) {
            let a201 = f144(v0.clone(), 1u64)?;
            let a202 = {
                let a200 = v0.clone();
                length_bytes(a200)
            };
            if nat_eq(a201, a202) {
                let a203 = f145(v0);
                Ok(bool_not(a203))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f77(v0: Adt0) -> Option<Bytes> {
    let m204 = v0;
    match m204 {
        Adt0::C0(_) => {
            None::<Bytes>
        }
        Adt0::C1(_, _) => {
            None::<Bytes>
        }
        Adt0::C2(v1) => {
            Some(v1)
        }
        Adt0::C3(_) => {
            None::<Bytes>
        }
        Adt0::C4(_) => {
            None::<Bytes>
        }
        Adt0::C5 => {
            None::<Bytes>
        }
        Adt0::C6(_, _) => {
            None::<Bytes>
        }
        Adt0::C7(_, _, _) => {
            None::<Bytes>
        }
    }
}

pub fn f78(v0: Fn2, p208: Adt1, p209: List<Adt1>) -> Adt1 {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m205 = v2;
                match m205.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c206: Fn2 = v0.clone();
                            c206.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f80(v0: Fn3, p213: Adt1, p214: List<(Bytes, Adt1)>) -> Adt1 {
    let m212 = (p213, p214);
    {
        let mut m212 = m212;
        loop {
            let step = {
                let (v1, v2) = m212;
                let m210 = v2;
                match m210.uncons() {
                    None => {
                        Err::<(Adt1, List<(Bytes, Adt1)>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<(Bytes, Adt1)>), Adt1>(({
                            let c211: Fn3 = v0.clone();
                            c211.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: Adt1, v2: (Bytes, Adt1)) -> Adt1 {
    if f61({
        let (h215, _) = v2;
        h215
    }, v0) {
        let (_, h216) = v2;
        h216
    } else {
        v1
    }
}

pub fn f82(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m217 = v1;
    match m217 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m218 = f13(f52(v0.clone(), v3)?, f14(Str::lit("properties")));
            match m218 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m219 = f13(v4, v2);
                    match m219 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f53(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: Bytes) -> Adt2 {
    if f61(v1.clone(), f14(Str::lit("element"))) {
        Adt2::C1
    } else {
        if f61(v1.clone(), f14(Str::lit("binder"))) {
            Adt2::C3
        } else {
            if f61(v1.clone(), f14(Str::lit("parameters"))) {
                Adt2::C8
            } else {
                if f61(v1.clone(), f14(Str::lit("arguments"))) {
                    Adt2::C6
                } else {
                    if f61(v1.clone(), f14(Str::lit("type_arguments"))) {
                        Adt2::C7
                    } else {
                        if f61(v1.clone(), f14(Str::lit("fields"))) {
                            Adt2::C11
                        } else {
                            if f61(v1.clone(), f14(Str::lit("branches"))) {
                                Adt2::C13
                            } else {
                                if f61(v1.clone(), f14(Str::lit("result"))) {
                                    Adt2::C1
                                } else {
                                    if if f61(v1.clone(), f14(Str::lit("type"))) {
                                        true
                                    } else {
                                        if f61(v1.clone(), f14(Str::lit("constructor"))) {
                                            true
                                        } else {
                                            f61(v1.clone(), f14(Str::lit("model")))
                                        }
                                    } {
                                        Adt2::C2
                                    } else {
                                        if f61(v1, f14(Str::lit("function"))) {
                                            if f61(v0, f14(Str::lit("apply"))) {
                                                Adt2::C0
                                            } else {
                                                Adt2::C2
                                            }
                                        } else {
                                            Adt2::C0
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

pub fn f84(v0: Bytes) -> Adt2 {
    if f61(v0.clone(), f14(Str::lit("parameters"))) {
        Adt2::C9
    } else {
        if f61(v0.clone(), f14(Str::lit("arguments"))) {
            Adt2::C10
        } else {
            if f61(v0.clone(), f14(Str::lit("member"))) {
                Adt2::C2
            } else {
                if f61(v0, f14(Str::lit("name"))) {
                    Adt2::C5
                } else {
                    Adt2::C1
                }
            }
        }
    }
}

pub fn f85(v0: Adt0, v1: Adt2) -> R<Option<Adt0>> {
    let m220 = v1;
    match m220 {
        Adt2::C0 => {
            Ok(f16(v0, f14(Str::lit("term"))))
        }
        Adt2::C1 => {
            Ok(f16(v0, f14(Str::lit("type"))))
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(r221) => {
            let v2 = (*r221).clone();
            if f87(v0, v2.clone())? {
                Ok(Some(v2))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f86(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m222 = f72(f13(f52(v0.clone(), v1)?, f14(Str::lit("oneOf"))));
    match m222 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f146(Fn16::F147(v2, std::rc::Rc::new(v0)), None::<Adt0>, f137(v3))
        }
    }
}

pub fn f87(v0: Adt0, v1: Adt0) -> R<bool> {
    let m223 = f72(f13(f52(v0.clone(), v1)?, f14(Str::lit("oneOf"))));
    match m223 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f148(Fn17::F149(std::rc::Rc::new(v0)), true, f137(v2))
        }
    }
}

pub fn f88(v0: Bytes, v1: Bytes, v2: List<Adt1>, v3: Adt1) -> List<Adt1> {
    List::cons(f27(v0), List::cons(v3, List::cons(f27(v1), v2)))
}

pub fn f89(v0: Fn4, p227: (bool, List<Adt1>), p228: List<Adt1>) -> (bool, List<Adt1>) {
    let m226 = (p227, p228);
    {
        let mut m226 = m226;
        loop {
            let step = {
                let (v1, v2) = m226;
                let m224 = v2;
                match m224.uncons() {
                    None => {
                        Err::<((bool, List<Adt1>), List<Adt1>), (bool, List<Adt1>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt1>), List<Adt1>), (bool, List<Adt1>)>(({
                            let c225: Fn4 = v0.clone();
                            c225.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: (bool, List<Adt1>), v2: Adt1) -> (bool, List<Adt1>) {
    let (h229, _) = v1;
    if h229 {
        (false, List::cons(v2, {
            let (_, h230) = v1;
            h230
        }))
    } else {
        (false, List::cons(v2, List::cons(f27(v0), {
            let (_, h231) = v1;
            h231
        })))
    }
}

pub fn f91(v0: Bytes, v1: Bytes) -> Bytes {
    let a232 = v0;
    let a233 = v1;
    append_bytes(a232, a233)
}

pub fn f92(v0: Fn5, p237: (bool, Adt1), p238: List<Adt1>) -> (bool, Adt1) {
    let m236 = (p237, p238);
    {
        let mut m236 = m236;
        loop {
            let step = {
                let (v1, v2) = m236;
                let m234 = v2;
                match m234.uncons() {
                    None => {
                        Err::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(({
                            let c235: Fn5 = v0.clone();
                            c235.apply(v1, v3)
                        }, v4))
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

pub fn f93(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    f150(v0, v1)
}

pub fn f94(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f95(v0: u64) -> Bytes {
    f151(f152(), 10u64, v0)
}

pub fn f96(v0: Fn6, p242: List<(Bytes, Adt1)>, p243: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let m241 = (p242, p243);
    {
        let mut m241 = m241;
        loop {
            let step = {
                let (v1, v2) = m241;
                let m239 = v2;
                match m239.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(({
                            let c240: Fn6 = v0.clone();
                            c240.apply(v1, v3)?
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

pub fn f97(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>, v2: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
    let m244 = v0;
    match m244 {
        None => {
            Ok(List::cons((f153({
                let (h251, _) = v2.clone();
                h251
            }), f154({
                let (h252, _) = v2;
                h252
            }, {
                let (_, h253) = v2;
                h253
            })?), v1))
        }
        Some(_) => {
            let a246 = {
                let (h245, _) = v2.clone();
                h245
            };
            let a247 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a246, a247) {
                Ok(v1)
            } else {
                Ok(List::cons((f153({
                    let (h248, _) = v2.clone();
                    h248
                }), f154({
                    let (h249, _) = v2;
                    h249
                }, {
                    let (_, h250) = v2;
                    h250
                })?), v1))
            }
        }
    }
}

pub fn f98(v0: Fn7, p257: List<Adt1>, p258: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m256 = (p257, p258);
    {
        let mut m256 = m256;
        loop {
            let step = {
                let (v1, v2) = m256;
                let m254 = v2;
                match m254.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c255: Fn7 = v0.clone();
                            c255.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h259) = v1;
        h259
    }, v0)
}

pub fn f100(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f155(f156(Fn18::F157, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m264 = {
        let (h260, _) = f158(Fn19::F159, {
            let a262 = {
                let a261 = v0;
                length_list(a261)
            };
            let a263 = 1u64;
            nat_add(a262, a263)?
        }, v1)?;
        h260
    };
    match m264.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f101(v0: Fn8, p269: u64, p270: Adt6) -> (Adt6, bool) {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m265 = v1;
                if m265 == 0 {
                    Err::<(u64, Adt6), (Adt6, bool)>((v2, false))
                } else {
                    let v3 = m265 - 1;
                    let m267 = {
                        let c266: Fn8 = v0.clone();
                        c266.apply(v2.clone())
                    };
                    match m267 {
                        None => {
                            Err::<(u64, Adt6), (Adt6, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), (Adt6, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Bytes, v1: Adt6) -> Option<Adt6> {
    f160(v0, v1)
}

pub fn f103(v0: Fn9, p275: u64, p276: Adt0) -> R<(Adt0, bool)> {
    let m274 = (p275, p276);
    {
        let mut m274 = m274;
        loop {
            let step = {
                let (v1, v2) = m274;
                let m271 = v1;
                if m271 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m271 - 1;
                    let m273 = {
                        let c272: Fn9 = v0.clone();
                        c272.apply(v2.clone())?
                    };
                    match m273 {
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
                Ok(next) => { m274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f161(v0, v1)
}

pub fn f105(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m282 = {
            let (h278, _) = f162(Fn20::F163(v0.clone()), {
                let a280 = {
                    let a279 = v0.clone();
                    length_bytes(a279)
                };
                let a281 = 1u64;
                nat_add(a280, a281)?
            }, Adt9::C0(0u64, 0u64))?;
            h278
        };
        match m282 {
            Adt9::C0(_, h277) => {
                h277
            }
        }
    };
    Ok(f164(v0.clone(), v1, {
        let a284 = {
            let a283 = v0;
            length_bytes(a283)
        };
        let a285 = v1;
        nat_sub(a284, a285)
    }))
}

pub fn f106(v0: u64, v1: u64, v2: Adt3, v3: Adt4, v4: List<Adt4>) -> R<Adt3> {
    let v5: u64 = match v3.clone() {
        Adt4::C0(h286, _, _) => {
            h286
        }
    };
    let v6: bool = match v3.clone() {
        Adt4::C0(_, h287, _) => {
            h287
        }
    };
    let m289 = match v3 {
        Adt4::C0(_, _, h288) => {
            (*h288).clone()
        }
    };
    match m289 {
        Adt1::C0 => {
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h290, _, _) => {
                    h290
                }
            }, match v2 {
                Adt3::C0(_, h291, _) => {
                    h291
                }
            }, v4))
        }
        Adt1::C1(v7) => {
            Ok(Adt3::C0(List::cons(v7.clone(), match v2.clone() {
                Adt3::C0(h292, _, _) => {
                    h292
                }
            }), {
                let a294 = match v2 {
                    Adt3::C0(_, h293, _) => {
                        h293
                    }
                };
                let a295 = f165(v7)?;
                nat_add(a294, a295)?
            }, v4))
        }
        Adt1::C2(r312, r313) => {
            let v8 = (*r312).clone();
            let v9 = (*r313).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h296, _, _) => {
                    h296
                }
            }, match v2 {
                Adt3::C0(_, h297, _) => {
                    h297
                }
            }, List::cons(Adt4::C0(v5, v6, std::rc::Rc::new(v8)), List::cons(Adt4::C0(v5, v6, std::rc::Rc::new(v9)), v4))))
        }
        Adt1::C3(v10, r314) => {
            let v11 = (*r314).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h298, _, _) => {
                    h298
                }
            }, match v2 {
                Adt3::C0(_, h299, _) => {
                    h299
                }
            }, List::cons(Adt4::C0({
                let a300 = v5;
                let a301 = v10;
                nat_add(a300, a301)?
            }, v6, std::rc::Rc::new(v11)), v4)))
        }
        Adt1::C4 => {
            if v6 {
                Ok(Adt3::C0(List::cons({
                    let a302 = Str::lit(" ");
                    utf8_encode(a302)
                }, match v2.clone() {
                    Adt3::C0(h303, _, _) => {
                        h303
                    }
                }), {
                    let a305 = match v2 {
                        Adt3::C0(_, h304, _) => {
                            h304
                        }
                    };
                    let a306 = 1u64;
                    nat_add(a305, a306)?
                }, v4))
            } else {
                f166(v2, v5, v4)
            }
        }
        Adt1::C5 => {
            if v6 {
                Ok(Adt3::C0(match v2.clone() {
                    Adt3::C0(h307, _, _) => {
                        h307
                    }
                }, match v2 {
                    Adt3::C0(_, h308, _) => {
                        h308
                    }
                }, v4))
            } else {
                f166(v2, v5, v4)
            }
        }
        Adt1::C6(r315) => {
            let v12 = (*r315).clone();
            let v13: bool = f167(v1, v0, match v2.clone() {
                Adt3::C0(_, h309, _) => {
                    h309
                }
            }, v4.clone(), Adt4::C0(v5, true, std::rc::Rc::new(v12.clone())))?;
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h310, _, _) => {
                    h310
                }
            }, match v2 {
                Adt3::C0(_, h311, _) => {
                    h311
                }
            }, List::cons(Adt4::C0(v5, v13, std::rc::Rc::new(v12)), v4)))
        }
    }
}

pub fn f107(v0: Fn10, p320: u64, p321: List<Bytes>) -> (List<Bytes>, bool) {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v1, v2) = m319;
                let m316 = v1;
                if m316 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m316 - 1;
                    let m318 = {
                        let c317: Fn10 = v0.clone();
                        c317.apply(v2.clone())
                    };
                    match m318 {
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
                Ok(next) => { m319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: List<Bytes>) -> Option<List<Bytes>> {
    f168(v0)
}

pub fn f109(v0: Fn11, p325: List<Bytes>, p326: List<Bytes>) -> List<Bytes> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m322 = v2;
                match m322.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c323: Fn11 = v0.clone();
                            c323.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f111(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f74({
        let a328 = f73(v0.clone())?;
        let a329 = {
            let a327 = v1.clone();
            length_list(a327)
        };
        nat_eq(a328, a329)
    }, f169(v0, v1)))
}

pub fn f112(v0: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::<Bytes>::nil()))
}

pub fn f113(v0: Bytes) -> R<bool> {
    Ok(f74(f76(v0.clone())?, f74(!f170(v0.clone(), f171()), !f61(v0, f14(Str::lit("false"))))))
}

pub fn f114(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a330 = v0.clone();
        length_bytes(a330)
    };
    Ok(f74({
        let a331 = 0u64;
        let a332 = v1;
        nat_lt(a331, a332)
    }, f74(f172(v0.clone(), 0u64, false)?, {
        let a333 = v1;
        let a334 = 1u64;
        if nat_eq(a333, a334) {
            true
        } else {
            !f173(v0, f14(Str::lit("0")))
        }
    })))
}

pub fn f115(v0: Bytes) -> R<bool> {
    Ok(f74({
        let a339 = {
            let a336 = {
                let a335 = v0.clone();
                length_bytes(a335)
            };
            let a337 = 2u64;
            let a338 = 0u64;
            nat_rem(a336, a337, a338)
        };
        let a340 = 0u64;
        nat_eq(a339, a340)
    }, f172(v0, 0u64, true)?))
}

pub fn f116(v0: Str, v1: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::cons(f14(v1), List::<Bytes>::nil())))
}

pub fn f117(v0: Bytes) -> R<bool> {
    if f173(v0.clone(), f14(Str::lit("-"))) {
        let v1: Bytes = f164(v0.clone(), 1u64, {
            let a342 = {
                let a341 = v0;
                length_bytes(a341)
            };
            let a343 = 1u64;
            nat_sub(a342, a343)
        });
        Ok(f74(f114(v1.clone())?, !f61(v1, f14(Str::lit("0")))))
    } else {
        f114(v0)
    }
}

pub fn f118(v0: Str, v1: Str, v2: Str) -> List<Bytes> {
    List::cons(f14(Str::lit("kind")), List::cons(f14(v0), List::cons(f14(v1), List::cons(f14(v2), List::<Bytes>::nil()))))
}

pub fn f119(v0: Option<Adt0>) -> R<bool> {
    let m344 = v0;
    match m344 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f28(v1)
        }
    }
}

pub fn f120(v0: Adt0, v1: Bytes, v2: Fn12) -> R<bool> {
    let m345 = f72(f51(v0, v1));
    match m345 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            f148(Fn17::F174(v2), true, f137(v3))
        }
    }
}

pub fn f121(v0: Adt0) -> R<bool> {
    f28(v0)
}

pub fn f122(v0: Adt0) -> R<bool> {
    f175(v0)
}

pub fn f123(v0: Adt0, v1: Bytes) -> bool {
    f75(f72(f51(v0, v1)))
}

pub fn f124(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f74({
        let a349 = f73(v0.clone())?;
        let a350 = {
            let a347 = {
                let a346 = v1.clone();
                length_list(a346)
            };
            let a348 = if f75(f51(v0.clone(), f14(Str::lit("type_arguments")))) {
                1u64
            } else {
                0u64
            };
            nat_add(a347, a348)?
        };
        nat_eq(a349, a350)
    }, f169(v0, v1)))
}

pub fn f125(v0: Adt0) -> R<bool> {
    let m351 = f51(v0, f14(Str::lit("type_arguments")));
    match m351 {
        None => {
            Ok(true)
        }
        Some(v1) => {
            let m352 = f139(v1);
            match m352 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    let a353 = 0u64;
                    let a354 = f73(v2)?;
                    Ok(nat_lt(a353, a354))
                }
            }
        }
    }
}

pub fn f126(v0: Option<Adt0>) -> R<bool> {
    let m355 = v0;
    match m355 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m356 = f26(v1)?;
            match m356 {
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

pub fn f127(v0: Adt0) -> R<bool> {
    f176(v0)
}

pub fn f128(v0: Adt0) -> R<bool> {
    f175(v0)
}

pub fn f129(v0: Adt0) -> R<bool> {
    f177(v0)
}

pub fn f130(v0: Fn13, p361: u64, p362: Adt7) -> R<(Adt7, bool)> {
    let m360 = (p361, p362);
    {
        let mut m360 = m360;
        loop {
            let step = {
                let (v1, v2) = m360;
                let m357 = v1;
                if m357 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m357 - 1;
                    let m359 = {
                        let c358: Fn13 = v0.clone();
                        c358.apply(v2.clone())?
                    };
                    match m359 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f178(v1, v0, v2)
}

pub fn f132(v0: Bytes, v1: Bytes) -> Bytes {
    let a363 = v0;
    let a364 = v1;
    append_bytes(a363, a364)
}

pub fn f133(v0: Str) -> Bytes {
    let a365 = v0;
    utf8_encode(a365)
}

pub fn f134(v0: List<Bytes>) -> Bytes {
    f55(f56(v0))
}

pub fn f135(v0: Fn14, p369: List<Adt1>, p370: List<Adt0>) -> List<Adt1> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v1, v2) = m368;
                let m366 = v2;
                match m366.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt0>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt0>), List<Adt1>>(({
                            let c367: Fn14 = v0.clone();
                            c367.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: List<Adt1>, v1: Adt0) -> List<Adt1> {
    let a372 = v0;
    let a373 = List::cons(f27({
        let m371 = f77(v1);
        match m371 {
            None => {
                f14(Str::lit(""))
            }
            Some(v2) => {
                v2
            }
        }
    }), List::<Adt1>::nil());
    append_list(a372, a373)
}

pub fn f137(v0: Adt0) -> List<Adt0> {
    f179({
        let m376 = {
            let (h375, _) = f180(Fn21::F181, 1000000u64, Adt10::C0(std::rc::Rc::new(v0), List::<Adt0>::nil()));
            h375
        };
        match m376 {
            Adt10::C0(_, h374) => {
                h374
            }
        }
    })
}

pub fn f138(v0: Bytes) -> bool {
    f170(v0, List::cons(f14(Str::lit("Bool")), List::cons(f14(Str::lit("Bytes")), List::cons(f14(Str::lit("Int")), List::cons(f14(Str::lit("Int8")), List::cons(f14(Str::lit("Int16")), List::cons(f14(Str::lit("Int32")), List::cons(f14(Str::lit("Int64")), List::cons(f14(Str::lit("Nat")), List::cons(f14(Str::lit("Ordering")), List::cons(f14(Str::lit("Prop")), List::cons(f14(Str::lit("String")), List::cons(f14(Str::lit("Type")), List::cons(f14(Str::lit("UInt8")), List::cons(f14(Str::lit("UInt16")), List::cons(f14(Str::lit("UInt32")), List::cons(f14(Str::lit("UInt64")), List::cons(f14(Str::lit("Unit")), List::cons(f14(Str::lit("ContractViolation")), List::cons(f14(Str::lit("ReasoningFailure")), List::cons(f14(Str::lit("List")), List::cons(f14(Str::lit("Option")), List::cons(f14(Str::lit("Result")), List::cons(f14(Str::lit("Set")), List::cons(f14(Str::lit("Map")), List::cons(f14(Str::lit("Fn")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f139(v0: Adt0) -> Option<Adt0> {
    let m377 = v0;
    match m377 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r378) => {
            let v1 = (*r378).clone();
            Some(v1)
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f140(v0: Fn15, p383: u64, p384: Adt8) -> R<(Adt8, bool)> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v1, v2) = m382;
                let m379 = v1;
                if m379 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m379 - 1;
                    let m381 = {
                        let c380: Fn15 = v0.clone();
                        c380.apply(v2.clone())?
                    };
                    match m381 {
                        None => {
                            Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), R<(Adt8, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt8) -> R<Option<Adt8>> {
    f182(v0)
}

pub fn f142(v0: u64) -> bool {
    let a385 = 64u64;
    let a386 = v0;
    if if nat_lt(a385, a386) {
        let a387 = v0;
        let a388 = 91u64;
        nat_lt(a387, a388)
    } else {
        false
    } {
        true
    } else {
        let a389 = 96u64;
        let a390 = v0;
        if if nat_lt(a389, a390) {
            let a391 = v0;
            let a392 = 123u64;
            nat_lt(a391, a392)
        } else {
            false
        } {
            true
        } else {
            let a393 = v0;
            let a394 = 95u64;
            nat_eq(a393, a394)
        }
    }
}

pub fn f143(v0: Bytes, v1: u64) -> R<u64> {
    let m395 = f183(v0, v1);
    match m395 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a440 = {
                let a416 = {
                    let a404 = {
                        let a398 = {
                            let a396 = v3;
                            let a397 = 128u8;
                            fixed_u8::bit_and(a396, a397)
                        };
                        let a399 = 128u8;
                        if equal(a398, a399) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a405 = {
                        let a402 = {
                            let a400 = v3;
                            let a401 = 64u8;
                            fixed_u8::bit_and(a400, a401)
                        };
                        let a403 = 64u8;
                        if equal(a402, a403) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a404, a405)?
                };
                let a417 = {
                    let a414 = {
                        let a408 = {
                            let a406 = v3;
                            let a407 = 32u8;
                            fixed_u8::bit_and(a406, a407)
                        };
                        let a409 = 32u8;
                        if equal(a408, a409) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a415 = {
                        let a412 = {
                            let a410 = v3;
                            let a411 = 16u8;
                            fixed_u8::bit_and(a410, a411)
                        };
                        let a413 = 16u8;
                        if equal(a412, a413) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a414, a415)?
                };
                nat_add(a416, a417)?
            };
            let a441 = {
                let a438 = {
                    let a426 = {
                        let a420 = {
                            let a418 = v3;
                            let a419 = 8u8;
                            fixed_u8::bit_and(a418, a419)
                        };
                        let a421 = 8u8;
                        if equal(a420, a421) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a427 = {
                        let a424 = {
                            let a422 = v3;
                            let a423 = 4u8;
                            fixed_u8::bit_and(a422, a423)
                        };
                        let a425 = 4u8;
                        if equal(a424, a425) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a426, a427)?
                };
                let a439 = {
                    let a436 = {
                        let a430 = {
                            let a428 = v3;
                            let a429 = 2u8;
                            fixed_u8::bit_and(a428, a429)
                        };
                        let a431 = 2u8;
                        if equal(a430, a431) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a437 = {
                        let a434 = {
                            let a432 = v3;
                            let a433 = 1u8;
                            fixed_u8::bit_and(a432, a433)
                        };
                        let a435 = 1u8;
                        if equal(a434, a435) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a436, a437)?
                };
                nat_add(a438, a439)?
            };
            nat_add(a440, a441)
        }
    }
}

pub fn f144(v0: Bytes, v1: u64) -> R<u64> {
    let (h442, _) = f184(Fn22::F185(v0.clone()), {
        let a444 = {
            let a443 = v0;
            length_bytes(a443)
        };
        let a445 = 1u64;
        nat_add(a444, a445)?
    }, v1)?;
    Ok(h442)
}

pub fn f145(v0: Bytes) -> bool {
    let a446 = v0.clone();
    let a447 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a446, a447) {
        true
    } else {
        let a448 = v0;
        let a449 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a448, a449)
    }
}

pub fn f146(v0: Fn16, p453: Option<Adt0>, p454: List<Adt0>) -> R<Option<Adt0>> {
    let m452 = (p453, p454);
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v1, v2) = m452;
                let m450 = v2;
                match m450.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c451: Fn16 = v0.clone();
                            c451.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m455 = v2.clone();
    match m455 {
        None => {
            let v5: Adt0 = f52(v1, v3)?;
            if f186(v5.clone(), v0)? {
                Ok(Some(v5))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f148(v0: Fn17, p459: bool, p460: List<Adt0>) -> R<bool> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v1, v2) = m458;
                let m456 = v2;
                match m456.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c457: Fn17 = v0.clone();
                            c457.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m461 = f13(f52(v0, v2)?, f14(Str::lit("properties")));
        match m461 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f75(f13(v3, f14(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f150(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    let (h462, _) = v0;
    if h462 {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))
    } else {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(f45(Str::lit(",")))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C4), std::rc::Rc::new({
            let (_, h463) = v0;
            h463
        })))))))
    }
}

pub fn f151(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a464 = v2;
    let a465 = 0u64;
    if nat_eq(a464, a465) {
        f187(v0, 0u64)
    } else {
        let (_, h466) = {
            let (h467, _) = f188(Fn23::F189(v1, v0), v2, (v2, Bytes::lit(&[])));
            h467
        };
        h466
    }
}

pub fn f152() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f153(v0: Bytes) -> Bytes {
    f91(f190(f191(v0.clone())), v0)
}

pub fn f154(v0: Bytes, v1: Adt1) -> R<Adt1> {
    Ok(Adt1::C2(std::rc::Rc::new(Adt1::C1(f91(f44(v0)?, f45(Str::lit(": "))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))))
}

pub fn f155(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f192(Fn24::F193, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f156(v0: Fn18, p471: List<List<(Bytes, Adt1)>>, p472: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m470 = (p471, p472);
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v1, v2) = m470;
                let m468 = v2;
                match m468.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c469: Fn18 = v0.clone();
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

pub fn f157(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f158(v0: Fn19, p477: u64, p478: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m476 = (p477, p478);
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v1, v2) = m476;
                let m473 = v1;
                if m473 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m473 - 1;
                    let m475 = {
                        let c474: Fn19 = v0.clone();
                        c474.apply(v2.clone())?
                    };
                    match m475 {
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
                Ok(next) => { m476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f194(v0)
}

pub fn f160(v0: Bytes, v1: Adt6) -> Option<Adt6> {
    let m480 = match v1.clone() {
        Adt6::C0(_, h479) => {
            h479
        }
    };
    match m480 {
        None => {
            let m482 = f195(match v1 {
                Adt6::C0(h481, _) => {
                    (*h481).clone()
                }
            });
            match m482 {
                None => {
                    None::<Adt6>
                }
                Some(v3) => {
                    if f61(match v3.clone() {
                        Adt12::C0(h483, _, _) => {
                            h483
                        }
                    }, v0) {
                        Some(Adt6::C0(std::rc::Rc::new(Adt0::C5), Some(match v3 {
                            Adt12::C0(_, h484, _) => {
                                (*h484).clone()
                            }
                        })))
                    } else {
                        Some(Adt6::C0(std::rc::Rc::new(match v3 {
                            Adt12::C0(_, _, h485) => {
                                (*h485).clone()
                            }
                        }), None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt6>
        }
    }
}

pub fn f161(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m486 = f69(f13(v1, f14(Str::lit("$ref"))));
    match m486 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f16(v0, f105(v2)?))
        }
    }
}

pub fn f162(v0: Fn20, p491: u64, p492: Adt9) -> R<(Adt9, bool)> {
    let m490 = (p491, p492);
    {
        let mut m490 = m490;
        loop {
            let step = {
                let (v1, v2) = m490;
                let m487 = v1;
                if m487 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m487 - 1;
                    let m489 = {
                        let c488: Fn20 = v0.clone();
                        c488.apply(v2.clone())?
                    };
                    match m489 {
                        None => {
                            Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt9), R<(Adt9, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    f196(v0, v1)
}

pub fn f164(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m496 = {
        let a493 = v0;
        let a494 = v1;
        let a495 = v2;
        slice_bytes(a493, a494, a495)
    };
    match m496 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f165(v0: Bytes) -> R<u64> {
    let (_, h497) = {
        let (h498, _) = f197(Fn25::F198(v0.clone()), {
            let a499 = v0;
            length_bytes(a499)
        }, (0u64, 0u64))?;
        h498
    };
    Ok(h497)
}

pub fn f166(v0: Adt3, v1: u64, v2: List<Adt4>) -> R<Adt3> {
    Ok(Adt3::C0(f199(match v0 {
        Adt3::C0(h500, _, _) => {
            h500
        }
    }, v1)?, v1, v2))
}

pub fn f167(v0: u64, v1: u64, v2: u64, v3: List<Adt4>, v4: Adt4) -> R<bool> {
    let v5: Adt11 = {
        let (h501, _) = f200(Fn26::F201, {
            let a502 = v0;
            let a503 = 1u64;
            nat_add(a502, a503)?
        }, Adt11::C0(0u64, {
            let a504 = v1;
            let a505 = v2;
            nat_sub(a504, a505)
        }, {
            let a506 = v1;
            let a507 = v2;
            nat_lt(a506, a507)
        }, List::cons(v4, List::<Adt4>::nil()), v3, None::<bool>))?;
        h501
    };
    let m509 = match v5 {
        Adt11::C0(_, _, _, _, _, h508) => {
            h508
        }
    };
    match m509 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f168(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m510 = v0.clone();
    match m510.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m511 = v2;
            match m511.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f202(Fn27::F203, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m513 = {
                        let (h512, _) = v5;
                        h512
                    };
                    match m513 {
                        None => {
                            Some(f56({
                                let (_, h515) = v5;
                                h515
                            }))
                        }
                        Some(v6) => {
                            Some(f56(List::cons(v6, {
                                let (_, h514) = v5;
                                h514
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f169(v0: Adt0, v1: List<Bytes>) -> bool {
    f204(Fn28::F205(std::rc::Rc::new(v0)), true, v1)
}

pub fn f170(v0: Bytes, v1: List<Bytes>) -> bool {
    f204(Fn28::F206(v0), false, v1)
}

pub fn f171() -> List<Bytes> {
    List::cons(f14(Str::lit("let")), List::cons(f14(Str::lit("if")), List::cons(f14(Str::lit("match")), List::cons(f14(Str::lit("fun")), List::cons(f14(Str::lit("forall")), List::cons(f14(Str::lit("nil")), List::cons(f14(Str::lit("cons")), List::cons(f14(Str::lit("first")), List::cons(f14(Str::lit("second")), List::cons(f14(Str::lit("not")), List::cons(f14(Str::lit("apply")), List::cons(f14(Str::lit("ctor")), List::cons(f14(Str::lit("record")), List::cons(f14(Str::lit("project")), List::cons(f14(Str::lit("prim")), List::cons(f14(Str::lit("ref")), List::cons(f14(Str::lit("checked")), List::cons(f14(Str::lit("then")), List::cons(f14(Str::lit("else")), List::cons(f14(Str::lit("with")), List::cons(f14(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f172(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = {
        let (h516, _) = f184(Fn22::F207(v2, v0.clone()), {
            let a518 = {
                let a517 = v0.clone();
                length_bytes(a517)
            };
            let a519 = 1u64;
            nat_add(a518, a519)?
        }, v1)?;
        h516
    };
    let a521 = v3;
    let a522 = {
        let a520 = v0;
        length_bytes(a520)
    };
    Ok(nat_eq(a521, a522))
}

pub fn f173(v0: Bytes, v1: Bytes) -> bool {
    let m527 = {
        let a524 = v0;
        let a525 = 0u64;
        let a526 = {
            let a523 = v1.clone();
            length_bytes(a523)
        };
        slice_bytes(a524, a525, a526)
    };
    match m527 {
        None => {
            false
        }
        Some(v2) => {
            let a528 = v2;
            let a529 = v1;
            equal(a528, a529)
        }
    }
}

pub fn f174(v0: Fn12, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let c530: Fn12 = v0;
        c530.apply(v2)
    } else {
        Ok(false)
    }
}

pub fn f175(v0: Adt0) -> R<bool> {
    let m531 = f77(v0);
    match m531 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f76(v1)
        }
    }
}

pub fn f176(v0: Adt0) -> R<bool> {
    let m532 = f50(v0);
    match m532 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f74(f111(v1.clone(), List::cons(f14(Str::lit("field")), List::cons(f14(Str::lit("value")), List::<Bytes>::nil())))?, {
                let m533 = f69(f51(v1, f14(Str::lit("field"))));
                match m533 {
                    None => {
                        false
                    }
                    Some(v2) => {
                        f76(v2)?
                    }
                }
            }))
        }
    }
}

pub fn f177(v0: Adt0) -> R<bool> {
    let m534 = f50(v0);
    match m534 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f74(f111(v1.clone(), List::cons(f14(Str::lit("constructor")), List::cons(f14(Str::lit("binders")), List::cons(f14(Str::lit("body")), List::<Bytes>::nil()))))?, f74(f126(f51(v1.clone(), f14(Str::lit("constructor"))))?, f120(v1, f14(Str::lit("binders")), Fn12::F208)?)))
        }
    }
}

pub fn f178(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h535, _) => {
            h535
        }
    };
    let a536 = v3;
    let a537 = v1;
    if nat_lt(a536, a537) {
        let v4: u64 = f209(v0.clone(), v3)?;
        let a538 = v4;
        let a539 = 32u64;
        if if nat_lt(a538, a539) {
            true
        } else {
            let a540 = v4;
            let a541 = 34u64;
            if nat_eq(a540, a541) {
                true
            } else {
                let a542 = v4;
                let a543 = 92u64;
                nat_eq(a542, a543)
            }
        } {
            Ok(Some(Adt7::C0({
                let a544 = v3;
                let a545 = 1u64;
                nat_add(a544, a545)?
            }, List::cons(f210(v4), match v2 {
                Adt7::C0(_, h546) => {
                    h546
                }
            }))))
        } else {
            let v5: u64 = f211(v0.clone(), v1, v3)?;
            Ok(Some(Adt7::C0(v5, List::cons(f164(v0, v3, {
                let a547 = v5;
                let a548 = v3;
                nat_sub(a547, a548)
            }), match v2 {
                Adt7::C0(_, h549) => {
                    h549
                }
            }))))
        }
    } else {
        Ok(None::<Adt7>)
    }
}

pub fn f179(v0: List<Adt0>) -> List<Adt0> {
    f212(Fn29::F213, List::<Adt0>::nil(), v0)
}

pub fn f180(v0: Fn21, p554: u64, p555: Adt10) -> (Adt10, bool) {
    let m553 = (p554, p555);
    {
        let mut m553 = m553;
        loop {
            let step = {
                let (v1, v2) = m553;
                let m550 = v1;
                if m550 == 0 {
                    Err::<(u64, Adt10), (Adt10, bool)>((v2, false))
                } else {
                    let v3 = m550 - 1;
                    let m552 = {
                        let c551: Fn21 = v0.clone();
                        c551.apply(v2.clone())
                    };
                    match m552 {
                        None => {
                            Err::<(u64, Adt10), (Adt10, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), (Adt10, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Adt10) -> Option<Adt10> {
    f214(v0)
}

pub fn f182(v0: Adt8) -> R<Option<Adt8>> {
    let m557 = f195(match v0.clone() {
        Adt8::C0(h556, _) => {
            (*h556).clone()
        }
    });
    match m557 {
        None => {
            let m563 = f215(match v0.clone() {
                Adt8::C0(h562, _) => {
                    (*h562).clone()
                }
            });
            match m563 {
                None => {
                    Ok(None::<Adt8>)
                }
                Some(v2) => {
                    Ok(Some(Adt8::C0(std::rc::Rc::new(match v2 {
                        Adt13::C0(_, h564) => {
                            (*h564).clone()
                        }
                    }), {
                        let a566 = match v0 {
                            Adt8::C0(_, h565) => {
                                h565
                            }
                        };
                        let a567 = 1u64;
                        nat_add(a566, a567)?
                    })))
                }
            }
        }
        Some(v1) => {
            Ok(Some(Adt8::C0(std::rc::Rc::new(match v1 {
                Adt12::C0(_, _, h558) => {
                    (*h558).clone()
                }
            }), {
                let a560 = match v0 {
                    Adt8::C0(_, h559) => {
                        h559
                    }
                };
                let a561 = 1u64;
                nat_add(a560, a561)?
            })))
        }
    }
}

pub fn f183(v0: Bytes, v1: u64) -> Option<u8> {
    let a568 = v0;
    let a569 = v1;
    index_bytes(a568, a569)
}

pub fn f184(v0: Fn22, p574: u64, p575: u64) -> R<(u64, bool)> {
    let m573 = (p574, p575);
    {
        let mut m573 = m573;
        loop {
            let step = {
                let (v1, v2) = m573;
                let m570 = v1;
                if m570 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m570 - 1;
                    let m572 = {
                        let c571: Fn22 = v0.clone();
                        c571.apply(v2)?
                    };
                    match m572 {
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
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f216(v0, v1)
}

pub fn f186(v0: Adt0, v1: Bytes) -> R<bool> {
    let m576 = f13(v0, f14(Str::lit("properties")));
    match m576 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m577 = f13(v2, f14(Str::lit("kind")));
            match m577 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m578 = f69(f13(v3.clone(), f14(Str::lit("const"))));
                    if match m578 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f61(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m579 = f72(f13(v3, f14(Str::lit("enum"))));
                        match m579 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f148(Fn17::F217(v1), false, f137(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: u64) -> Bytes {
    let m583 = {
        let a580 = v0;
        let a581 = v1;
        let a582 = 1u64;
        slice_bytes(a580, a581, a582)
    };
    match m583 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f188(v0: Fn23, p588: u64, p589: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m587 = (p588, p589);
    {
        let mut m587 = m587;
        loop {
            let step = {
                let (v1, v2) = m587;
                let m584 = v1;
                if m584 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m584 - 1;
                    let m586 = {
                        let c585: Fn23 = v0.clone();
                        c585.apply(v2.clone())
                    };
                    match m586 {
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
                Ok(next) => { m587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f218(v1, v0, v2)
}

pub fn f190(v0: u64) -> Bytes {
    f164(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f191(v0: Bytes) -> u64 {
    let a590 = v0.clone();
    let a591 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a590, a591) {
        0u64
    } else {
        let a592 = v0.clone();
        let a593 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a592, a593) {
            1u64
        } else {
            let a594 = v0.clone();
            let a595 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a594, a595) {
                2u64
            } else {
                let a596 = v0.clone();
                let a597 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a596, a597) {
                    3u64
                } else {
                    let a598 = v0.clone();
                    let a599 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a598, a599) {
                        4u64
                    } else {
                        let a600 = v0.clone();
                        let a601 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a600, a601) {
                            5u64
                        } else {
                            let a602 = v0.clone();
                            let a603 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a602, a603) {
                                6u64
                            } else {
                                let a604 = v0.clone();
                                let a605 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a604, a605) {
                                    7u64
                                } else {
                                    let a606 = v0.clone();
                                    let a607 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a606, a607) {
                                        8u64
                                    } else {
                                        let a608 = v0.clone();
                                        let a609 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a608, a609) {
                                            9u64
                                        } else {
                                            let a610 = v0.clone();
                                            let a611 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a610, a611) {
                                                10u64
                                            } else {
                                                let a612 = v0;
                                                let a613 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a612, a613) {
                                                    11u64
                                                } else {
                                                    12u64
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

pub fn f192(v0: Fn24, p617: List<List<(Bytes, Adt1)>>, p618: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v1, v2) = m616;
                let m614 = v2;
                match m614.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c615: Fn24 = v0.clone();
                            c615.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f194(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a620 = {
        let a619 = v0.clone();
        length_list(a619)
    };
    let a621 = 1u64;
    if nat_le(a620, a621) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f219(v0)?))
    }
}

pub fn f195(v0: Adt0) -> Option<Adt12> {
    let m622 = v0;
    match m622 {
        Adt0::C0(_) => {
            None::<Adt12>
        }
        Adt0::C1(_, _) => {
            None::<Adt12>
        }
        Adt0::C2(_) => {
            None::<Adt12>
        }
        Adt0::C3(_) => {
            None::<Adt12>
        }
        Adt0::C4(_) => {
            None::<Adt12>
        }
        Adt0::C5 => {
            None::<Adt12>
        }
        Adt0::C6(_, _) => {
            None::<Adt12>
        }
        Adt0::C7(v1, r623, r624) => {
            let v2 = (*r623).clone();
            let v3 = (*r624).clone();
            Some(Adt12::C0(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3)))
        }
    }
}

pub fn f196(v0: Bytes, v1: Adt9) -> R<Option<Adt9>> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h625, _) => {
            h625
        }
    };
    let m626 = f183(v0, v2);
    match m626 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v3) => {
            Ok(Some(Adt9::C0({
                let a627 = v2;
                let a628 = 1u64;
                nat_add(a627, a628)?
            }, {
                let a675 = {
                    let v4: u8 = v3;
                    let a673 = {
                        let a649 = {
                            let a637 = {
                                let a631 = {
                                    let a629 = v4;
                                    let a630 = 128u8;
                                    fixed_u8::bit_and(a629, a630)
                                };
                                let a632 = 128u8;
                                if equal(a631, a632) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a638 = {
                                let a635 = {
                                    let a633 = v4;
                                    let a634 = 64u8;
                                    fixed_u8::bit_and(a633, a634)
                                };
                                let a636 = 64u8;
                                if equal(a635, a636) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a637, a638)?
                        };
                        let a650 = {
                            let a647 = {
                                let a641 = {
                                    let a639 = v4;
                                    let a640 = 32u8;
                                    fixed_u8::bit_and(a639, a640)
                                };
                                let a642 = 32u8;
                                if equal(a641, a642) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a648 = {
                                let a645 = {
                                    let a643 = v4;
                                    let a644 = 16u8;
                                    fixed_u8::bit_and(a643, a644)
                                };
                                let a646 = 16u8;
                                if equal(a645, a646) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a647, a648)?
                        };
                        nat_add(a649, a650)?
                    };
                    let a674 = {
                        let a671 = {
                            let a659 = {
                                let a653 = {
                                    let a651 = v4;
                                    let a652 = 8u8;
                                    fixed_u8::bit_and(a651, a652)
                                };
                                let a654 = 8u8;
                                if equal(a653, a654) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a660 = {
                                let a657 = {
                                    let a655 = v4;
                                    let a656 = 4u8;
                                    fixed_u8::bit_and(a655, a656)
                                };
                                let a658 = 4u8;
                                if equal(a657, a658) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a659, a660)?
                        };
                        let a672 = {
                            let a669 = {
                                let a663 = {
                                    let a661 = v4;
                                    let a662 = 2u8;
                                    fixed_u8::bit_and(a661, a662)
                                };
                                let a664 = 2u8;
                                if equal(a663, a664) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a670 = {
                                let a667 = {
                                    let a665 = v4;
                                    let a666 = 1u8;
                                    fixed_u8::bit_and(a665, a666)
                                };
                                let a668 = 1u8;
                                if equal(a667, a668) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a669, a670)?
                        };
                        nat_add(a671, a672)?
                    };
                    nat_add(a673, a674)?
                };
                let a676 = 47u64;
                if nat_eq(a675, a676) {
                    let a677 = v2;
                    let a678 = 1u64;
                    nat_add(a677, a678)?
                } else {
                    match v1 {
                        Adt9::C0(_, h679) => {
                            h679
                        }
                    }
                }
            })))
        }
    }
}

pub fn f197(v0: Fn25, p684: u64, p685: (u64, u64)) -> R<((u64, u64), bool)> {
    let m683 = (p684, p685);
    {
        let mut m683 = m683;
        loop {
            let step = {
                let (v1, v2) = m683;
                let m680 = v1;
                if m680 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m680 - 1;
                    let m682 = {
                        let c681: Fn25 = v0.clone();
                        c681.apply(v2)?
                    };
                    match m682 {
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
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f220(v0, v1)
}

pub fn f199(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a686 = v1;
        let a687 = 256u64;
        let a688 = 0u64;
        nat_quot(a686, a687, a688)
    };
    let v3: List<Bytes> = {
        let (_, h689) = {
            let (h690, _) = f221(Fn30::F222(v2), {
                let a691 = v2;
                let a692 = 1u64;
                nat_add(a691, a692)?
            }, (0u64, List::cons({
                let a693 = Str::lit("\u{a}");
                utf8_encode(a693)
            }, v0)))?;
            h690
        };
        h689
    };
    Ok(List::cons(f164(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a694 = v1;
        let a695 = 256u64;
        let a696 = 0u64;
        nat_rem(a694, a695, a696)
    }), v3))
}

pub fn f200(v0: Fn26, p701: u64, p702: Adt11) -> R<(Adt11, bool)> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v1, v2) = m700;
                let m697 = v1;
                if m697 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m697 - 1;
                    let m699 = {
                        let c698: Fn26 = v0.clone();
                        c698.apply(v2.clone())?
                    };
                    match m699 {
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
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Adt11) -> R<Option<Adt11>> {
    f223(v0)
}

pub fn f202(v0: Fn27, p706: (Option<Bytes>, List<Bytes>), p707: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v1, v2) = m705;
                let m703 = v2;
                match m703.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c704: Fn27 = v0.clone();
                            c704.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f224(v0, v1)
}

pub fn f204(v0: Fn28, p711: bool, p712: List<Bytes>) -> bool {
    let m710 = (p711, p712);
    {
        let mut m710 = m710;
        loop {
            let step = {
                let (v1, v2) = m710;
                let m708 = v2;
                match m708.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c709: Fn28 = v0.clone();
                            c709.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Adt0, v1: bool, v2: Bytes) -> bool {
    if v1 {
        f75(f51(v0, v2))
    } else {
        false
    }
}

pub fn f206(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a713 = v2;
        let a714 = v0;
        equal(a713, a714)
    }
}

pub fn f207(v0: bool, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let m715 = f183(v1, v2);
    match m715 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let v5: u64 = {
                let v4: u8 = v3;
                let a760 = {
                    let a736 = {
                        let a724 = {
                            let a718 = {
                                let a716 = v4;
                                let a717 = 128u8;
                                fixed_u8::bit_and(a716, a717)
                            };
                            let a719 = 128u8;
                            if equal(a718, a719) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a725 = {
                            let a722 = {
                                let a720 = v4;
                                let a721 = 64u8;
                                fixed_u8::bit_and(a720, a721)
                            };
                            let a723 = 64u8;
                            if equal(a722, a723) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a724, a725)?
                    };
                    let a737 = {
                        let a734 = {
                            let a728 = {
                                let a726 = v4;
                                let a727 = 32u8;
                                fixed_u8::bit_and(a726, a727)
                            };
                            let a729 = 32u8;
                            if equal(a728, a729) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a735 = {
                            let a732 = {
                                let a730 = v4;
                                let a731 = 16u8;
                                fixed_u8::bit_and(a730, a731)
                            };
                            let a733 = 16u8;
                            if equal(a732, a733) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a734, a735)?
                    };
                    nat_add(a736, a737)?
                };
                let a761 = {
                    let a758 = {
                        let a746 = {
                            let a740 = {
                                let a738 = v4;
                                let a739 = 8u8;
                                fixed_u8::bit_and(a738, a739)
                            };
                            let a741 = 8u8;
                            if equal(a740, a741) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a747 = {
                            let a744 = {
                                let a742 = v4;
                                let a743 = 4u8;
                                fixed_u8::bit_and(a742, a743)
                            };
                            let a745 = 4u8;
                            if equal(a744, a745) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a746, a747)?
                    };
                    let a759 = {
                        let a756 = {
                            let a750 = {
                                let a748 = v4;
                                let a749 = 2u8;
                                fixed_u8::bit_and(a748, a749)
                            };
                            let a751 = 2u8;
                            if equal(a750, a751) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a757 = {
                            let a754 = {
                                let a752 = v4;
                                let a753 = 1u8;
                                fixed_u8::bit_and(a752, a753)
                            };
                            let a755 = 1u8;
                            if equal(a754, a755) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a756, a757)?
                    };
                    nat_add(a758, a759)?
                };
                nat_add(a760, a761)?
            };
            if if f74({
                let a762 = 48u64;
                let a763 = v5;
                nat_le(a762, a763)
            }, {
                let a764 = v5;
                let a765 = 57u64;
                nat_le(a764, a765)
            }) {
                true
            } else {
                f74(v0, f74({
                    let a766 = 97u64;
                    let a767 = v5;
                    nat_le(a766, a767)
                }, {
                    let a768 = v5;
                    let a769 = 102u64;
                    nat_le(a768, a769)
                }))
            } {
                Ok(Some({
                    let a770 = v2;
                    let a771 = 1u64;
                    nat_add(a770, a771)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f208(v0: Adt0) -> R<bool> {
    let m772 = f77(v0);
    match m772 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f113(v1)
        }
    }
}

pub fn f209(v0: Bytes, v1: u64) -> R<u64> {
    let m773 = f183(v0, v1);
    match m773 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a818 = {
                let a794 = {
                    let a782 = {
                        let a776 = {
                            let a774 = v3;
                            let a775 = 128u8;
                            fixed_u8::bit_and(a774, a775)
                        };
                        let a777 = 128u8;
                        if equal(a776, a777) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a783 = {
                        let a780 = {
                            let a778 = v3;
                            let a779 = 64u8;
                            fixed_u8::bit_and(a778, a779)
                        };
                        let a781 = 64u8;
                        if equal(a780, a781) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a782, a783)?
                };
                let a795 = {
                    let a792 = {
                        let a786 = {
                            let a784 = v3;
                            let a785 = 32u8;
                            fixed_u8::bit_and(a784, a785)
                        };
                        let a787 = 32u8;
                        if equal(a786, a787) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a793 = {
                        let a790 = {
                            let a788 = v3;
                            let a789 = 16u8;
                            fixed_u8::bit_and(a788, a789)
                        };
                        let a791 = 16u8;
                        if equal(a790, a791) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a792, a793)?
                };
                nat_add(a794, a795)?
            };
            let a819 = {
                let a816 = {
                    let a804 = {
                        let a798 = {
                            let a796 = v3;
                            let a797 = 8u8;
                            fixed_u8::bit_and(a796, a797)
                        };
                        let a799 = 8u8;
                        if equal(a798, a799) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a805 = {
                        let a802 = {
                            let a800 = v3;
                            let a801 = 4u8;
                            fixed_u8::bit_and(a800, a801)
                        };
                        let a803 = 4u8;
                        if equal(a802, a803) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a804, a805)?
                };
                let a817 = {
                    let a814 = {
                        let a808 = {
                            let a806 = v3;
                            let a807 = 2u8;
                            fixed_u8::bit_and(a806, a807)
                        };
                        let a809 = 2u8;
                        if equal(a808, a809) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a815 = {
                        let a812 = {
                            let a810 = v3;
                            let a811 = 1u8;
                            fixed_u8::bit_and(a810, a811)
                        };
                        let a813 = 1u8;
                        if equal(a812, a813) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a814, a815)?
                };
                nat_add(a816, a817)?
            };
            nat_add(a818, a819)
        }
    }
}

pub fn f210(v0: u64) -> Bytes {
    let a820 = v0;
    let a821 = 34u64;
    if nat_eq(a820, a821) {
        f133(Str::lit("\\\""))
    } else {
        let a822 = v0;
        let a823 = 92u64;
        if nat_eq(a822, a823) {
            f133(Str::lit("\\\\"))
        } else {
            let a824 = v0;
            let a825 = 8u64;
            if nat_eq(a824, a825) {
                f133(Str::lit("\\b"))
            } else {
                let a826 = v0;
                let a827 = 12u64;
                if nat_eq(a826, a827) {
                    f133(Str::lit("\\f"))
                } else {
                    let a828 = v0;
                    let a829 = 10u64;
                    if nat_eq(a828, a829) {
                        f133(Str::lit("\\n"))
                    } else {
                        let a830 = v0;
                        let a831 = 13u64;
                        if nat_eq(a830, a831) {
                            f133(Str::lit("\\r"))
                        } else {
                            let a832 = v0;
                            let a833 = 9u64;
                            if nat_eq(a832, a833) {
                                f133(Str::lit("\\t"))
                            } else {
                                f132(f133(Str::lit("\\u00")), f225(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h834, _) = f184(Fn22::F226(v1, v0), {
        let a835 = v1;
        let a836 = v2;
        nat_sub(a835, a836)
    }, v2)?;
    Ok(h834)
}

pub fn f212(v0: Fn29, p840: List<Adt0>, p841: List<Adt0>) -> List<Adt0> {
    let m839 = (p840, p841);
    {
        let mut m839 = m839;
        loop {
            let step = {
                let (v1, v2) = m839;
                let m837 = v2;
                match m837.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c838: Fn29 = v0.clone();
                            c838.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f214(v0: Adt10) -> Option<Adt10> {
    let m843 = f215(match v0.clone() {
        Adt10::C0(h842, _) => {
            (*h842).clone()
        }
    });
    match m843 {
        None => {
            None::<Adt10>
        }
        Some(v1) => {
            Some(Adt10::C0(std::rc::Rc::new(match v1.clone() {
                Adt13::C0(_, h844) => {
                    (*h844).clone()
                }
            }), List::cons(match v1 {
                Adt13::C0(h845, _) => {
                    (*h845).clone()
                }
            }, match v0 {
                Adt10::C0(_, h846) => {
                    h846
                }
            })))
        }
    }
}

pub fn f215(v0: Adt0) -> Option<Adt13> {
    let m847 = v0;
    match m847 {
        Adt0::C0(_) => {
            None::<Adt13>
        }
        Adt0::C1(_, _) => {
            None::<Adt13>
        }
        Adt0::C2(_) => {
            None::<Adt13>
        }
        Adt0::C3(_) => {
            None::<Adt13>
        }
        Adt0::C4(_) => {
            None::<Adt13>
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6(r848, r849) => {
            let v1 = (*r848).clone();
            let v2 = (*r849).clone();
            Some(Adt13::C0(std::rc::Rc::new(v1), std::rc::Rc::new(v2)))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt13>
        }
    }
}

pub fn f216(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f227(f143(v0, v1)?) {
        Ok(Some({
            let a850 = v1;
            let a851 = 1u64;
            nat_add(a850, a851)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f217(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m852 = f77(v2);
        match m852 {
            None => {
                false
            }
            Some(v3) => {
                f61(v3, v0)
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h853, _) = v2;
        h853
    };
    let a854 = v3;
    let a855 = 0u64;
    if nat_eq(a854, a855) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a856 = v3;
            let a857 = v1;
            let a858 = 0u64;
            nat_quot(a856, a857, a858)
        }, {
            let a863 = f187(v0, {
                let a859 = v3;
                let a860 = v1;
                let a861 = 0u64;
                nat_rem(a859, a860, a861)
            });
            let a864 = {
                let (_, h862) = v2;
                h862
            };
            append_bytes(a863, a864)
        }))
    }
}

pub fn f219(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f155({
        let (_, h865) = {
            let (h866, _) = f228(Fn31::F229, {
                let a868 = {
                    let a867 = v0.clone();
                    length_list(a867)
                };
                let a869 = 1u64;
                nat_add(a868, a869)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h866
        };
        h865
    }))
}

pub fn f220(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m871 = f183(v0, {
        let (h870, _) = v1;
        h870
    });
    match m871 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a916 = {
                    let a892 = {
                        let a880 = {
                            let a874 = {
                                let a872 = v3;
                                let a873 = 128u8;
                                fixed_u8::bit_and(a872, a873)
                            };
                            let a875 = 128u8;
                            if equal(a874, a875) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a881 = {
                            let a878 = {
                                let a876 = v3;
                                let a877 = 64u8;
                                fixed_u8::bit_and(a876, a877)
                            };
                            let a879 = 64u8;
                            if equal(a878, a879) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a880, a881)?
                    };
                    let a893 = {
                        let a890 = {
                            let a884 = {
                                let a882 = v3;
                                let a883 = 32u8;
                                fixed_u8::bit_and(a882, a883)
                            };
                            let a885 = 32u8;
                            if equal(a884, a885) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a891 = {
                            let a888 = {
                                let a886 = v3;
                                let a887 = 16u8;
                                fixed_u8::bit_and(a886, a887)
                            };
                            let a889 = 16u8;
                            if equal(a888, a889) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a890, a891)?
                    };
                    nat_add(a892, a893)?
                };
                let a917 = {
                    let a914 = {
                        let a902 = {
                            let a896 = {
                                let a894 = v3;
                                let a895 = 8u8;
                                fixed_u8::bit_and(a894, a895)
                            };
                            let a897 = 8u8;
                            if equal(a896, a897) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a903 = {
                            let a900 = {
                                let a898 = v3;
                                let a899 = 4u8;
                                fixed_u8::bit_and(a898, a899)
                            };
                            let a901 = 4u8;
                            if equal(a900, a901) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a902, a903)?
                    };
                    let a915 = {
                        let a912 = {
                            let a906 = {
                                let a904 = v3;
                                let a905 = 2u8;
                                fixed_u8::bit_and(a904, a905)
                            };
                            let a907 = 2u8;
                            if equal(a906, a907) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a913 = {
                            let a910 = {
                                let a908 = v3;
                                let a909 = 1u8;
                                fixed_u8::bit_and(a908, a909)
                            };
                            let a911 = 1u8;
                            if equal(a910, a911) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a912, a913)?
                    };
                    nat_add(a914, a915)?
                };
                nat_add(a916, a917)?
            };
            let a918 = v4;
            let a919 = 128u64;
            if nat_lt(a918, a919) {
                Ok(Some(({
                    let a921 = {
                        let (h920, _) = v1;
                        h920
                    };
                    let a922 = 1u64;
                    nat_add(a921, a922)?
                }, {
                    let a924 = {
                        let (_, h923) = v1;
                        h923
                    };
                    let a925 = 1u64;
                    nat_add(a924, a925)?
                })))
            } else {
                let a926 = v4;
                let a927 = 192u64;
                if nat_lt(a926, a927) {
                    Ok(Some(({
                        let a929 = {
                            let (h928, _) = v1;
                            h928
                        };
                        let a930 = 1u64;
                        nat_add(a929, a930)?
                    }, {
                        let (_, h931) = v1;
                        h931
                    })))
                } else {
                    Ok(Some(({
                        let a933 = {
                            let (h932, _) = v1;
                            h932
                        };
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    }, {
                        let a936 = {
                            let (_, h935) = v1;
                            h935
                        };
                        let a937 = 1u64;
                        nat_add(a936, a937)?
                    })))
                }
            }
        }
    }
}

pub fn f221(v0: Fn30, p942: u64, p943: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m941 = (p942, p943);
    {
        let mut m941 = m941;
        loop {
            let step = {
                let (v1, v2) = m941;
                let m938 = v1;
                if m938 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m938 - 1;
                    let m940 = {
                        let c939: Fn30 = v0.clone();
                        c939.apply(v2.clone())?
                    };
                    match m940 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f230(v0, v1)
}

pub fn f223(v0: Adt11) -> R<Option<Adt11>> {
    let m945 = match v0.clone() {
        Adt11::C0(_, _, _, _, _, h944) => {
            h944
        }
    };
    match m945 {
        None => {
            if if match v0.clone() {
                Adt11::C0(_, _, h946, _, _, _) => {
                    h946
                }
            } {
                true
            } else {
                let a949 = match v0.clone() {
                    Adt11::C0(_, h947, _, _, _, _) => {
                        h947
                    }
                };
                let a950 = match v0.clone() {
                    Adt11::C0(h948, _, _, _, _, _) => {
                        h948
                    }
                };
                nat_lt(a949, a950)
            } {
                Ok(Some(f231(v0.clone(), match v0.clone() {
                    Adt11::C0(h951, _, _, _, _, _) => {
                        h951
                    }
                }, match v0.clone() {
                    Adt11::C0(_, _, _, h952, _, _) => {
                        h952
                    }
                }, match v0 {
                    Adt11::C0(_, _, _, _, h953, _) => {
                        h953
                    }
                }, Some(false))))
            } else {
                let m955 = match v0.clone() {
                    Adt11::C0(_, _, _, h954, _, _) => {
                        h954
                    }
                };
                match m955.uncons() {
                    None => {
                        let m958 = match v0.clone() {
                            Adt11::C0(_, _, _, _, h957, _) => {
                                h957
                            }
                        };
                        match m958.uncons() {
                            None => {
                                Ok(Some(f231(v0.clone(), match v0 {
                                    Adt11::C0(h959, _, _, _, _, _) => {
                                        h959
                                    }
                                }, List::<Adt4>::nil(), List::<Adt4>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f232(v0, v4, List::<Adt4>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f232(v0.clone(), v2, v3, match v0 {
                            Adt11::C0(_, _, _, _, h956, _) => {
                                h956
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt11>)
        }
    }
}

pub fn f224(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m961 = {
        let (h960, _) = v0;
        h960
    };
    match m961 {
        None => {
            (Some(v1), {
                let (_, h962) = v0;
                h962
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a963 = v2;
                let a964 = v1;
                append_bytes(a963, a964)
            }, {
                let (_, h965) = v0;
                h965
            }))
        }
    }
}

pub fn f225(v0: u64) -> Bytes {
    f132(f233(v0, 16u64), f234(v0))
}

pub fn f226(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f235(v1, v0, v2)
}

pub fn f227(v0: u64) -> bool {
    if f142(v0) {
        true
    } else {
        let a966 = 47u64;
        let a967 = v0;
        if if nat_lt(a966, a967) {
            let a968 = v0;
            let a969 = 58u64;
            nat_lt(a968, a969)
        } else {
            false
        } {
            true
        } else {
            let a970 = v0;
            let a971 = 46u64;
            if nat_eq(a970, a971) {
                true
            } else {
                let a972 = v0;
                let a973 = 39u64;
                if nat_eq(a972, a973) {
                    true
                } else {
                    let a974 = v0;
                    let a975 = 47u64;
                    if nat_eq(a974, a975) {
                        true
                    } else {
                        let a976 = v0;
                        let a977 = 45u64;
                        nat_eq(a976, a977)
                    }
                }
            }
        }
    }
}

pub fn f228(v0: Fn31, p982: u64, p983: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m981 = (p982, p983);
    {
        let mut m981 = m981;
        loop {
            let step = {
                let (v1, v2) = m981;
                let m978 = v1;
                if m978 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m978 - 1;
                    let m980 = {
                        let c979: Fn31 = v0.clone();
                        c979.apply(v2.clone())?
                    };
                    match m980 {
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
                Ok(next) => { m981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f236(v0)
}

pub fn f230(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a985 = {
        let (h984, _) = v1.clone();
        h984
    };
    let a986 = v0;
    if nat_lt(a985, a986) {
        Ok(Some(({
            let a988 = {
                let (h987, _) = v1;
                h987
            };
            let a989 = 1u64;
            nat_add(a988, a989)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h990) = v1;
            h990
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f231(v0: Adt11, v1: u64, v2: List<Adt4>, v3: List<Adt4>, v4: Option<bool>) -> Adt11 {
    Adt11::C0(v1, match v0.clone() {
        Adt11::C0(_, h991, _, _, _, _) => {
            h991
        }
    }, match v0 {
        Adt11::C0(_, _, h992, _, _, _) => {
            h992
        }
    }, v2, v3, v4)
}

pub fn f232(v0: Adt11, v1: Adt4, v2: List<Adt4>, v3: List<Adt4>) -> R<Adt11> {
    let v4: u64 = match v1.clone() {
        Adt4::C0(h993, _, _) => {
            h993
        }
    };
    let v5: bool = match v1.clone() {
        Adt4::C0(_, h994, _) => {
            h994
        }
    };
    let v6: u64 = match v0.clone() {
        Adt11::C0(h995, _, _, _, _, _) => {
            h995
        }
    };
    let m997 = match v1 {
        Adt4::C0(_, _, h996) => {
            (*h996).clone()
        }
    };
    match m997 {
        Adt1::C0 => {
            Ok(f231(v0, v6, v2, v3, None::<bool>))
        }
        Adt1::C1(v7) => {
            Ok(f231(v0, {
                let a998 = v6;
                let a999 = f165(v7)?;
                nat_add(a998, a999)?
            }, v2, v3, None::<bool>))
        }
        Adt1::C2(r1004, r1005) => {
            let v8 = (*r1004).clone();
            let v9 = (*r1005).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0(v4, v5, std::rc::Rc::new(v8)), List::cons(Adt4::C0(v4, v5, std::rc::Rc::new(v9)), v2)), v3, None::<bool>))
        }
        Adt1::C3(v10, r1006) => {
            let v11 = (*r1006).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0({
                let a1000 = v4;
                let a1001 = v10;
                nat_add(a1000, a1001)?
            }, v5, std::rc::Rc::new(v11)), v2), v3, None::<bool>))
        }
        Adt1::C4 => {
            if v5 {
                Ok(f231(v0, {
                    let a1002 = v6;
                    let a1003 = 1u64;
                    nat_add(a1002, a1003)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f231(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C5 => {
            if v5 {
                Ok(f231(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f231(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C6(r1007) => {
            let v12 = (*r1007).clone();
            Ok(f231(v0, v6, List::cons(Adt4::C0(v4, true, std::rc::Rc::new(v12)), v2), v3, None::<bool>))
        }
    }
}

pub fn f233(v0: u64, v1: u64) -> Bytes {
    f234({
        let a1008 = v0;
        let a1009 = v1;
        let a1010 = 0u64;
        nat_quot(a1008, a1009, a1010)
    })
}

pub fn f234(v0: u64) -> Bytes {
    f164(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1011 = v0;
        let a1012 = 16u64;
        let a1013 = 0u64;
        nat_rem(a1011, a1012, a1013)
    }, 1u64)
}

pub fn f235(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1014 = v2;
    let a1015 = v1;
    if nat_lt(a1014, a1015) {
        let v3: u64 = f209(v0, v2)?;
        let a1016 = v3;
        let a1017 = 32u64;
        if nat_lt(a1016, a1017) {
            Ok(None::<u64>)
        } else {
            let a1018 = v3;
            let a1019 = 34u64;
            if nat_eq(a1018, a1019) {
                Ok(None::<u64>)
            } else {
                let a1020 = v3;
                let a1021 = 92u64;
                if nat_eq(a1020, a1021) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1022 = v2;
                        let a1023 = 1u64;
                        nat_add(a1022, a1023)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f236(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h1024, _) = v0;
        h1024
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h1025) = v0;
        h1025
    };
    let m1026 = v1;
    match m1026.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m1027 = v4;
            match m1027.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f237(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f237(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a1032 = {
            let a1030 = {
                let a1028 = v0.clone();
                length_list(a1028)
            };
            let a1031 = {
                let a1029 = v1.clone();
                length_list(a1029)
            };
            nat_add(a1030, a1031)?
        };
        let a1033 = 1u64;
        nat_add(a1032, a1033)?
    };
    f238({
        let (_, h1034) = {
            let (_, h1035) = {
                let (h1036, _) = f239(Fn32::F240, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h1036
            };
            h1035
        };
        h1034
    })
}

pub fn f238(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    f96(Fn6::F241, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f239(v0: Fn32, p1041: u64, p1042: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m1040 = (p1041, p1042);
    {
        let mut m1040 = m1040;
        loop {
            let step = {
                let (v1, v2) = m1040;
                let m1037 = v1;
                if m1037 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m1037 - 1;
                    let m1039 = {
                        let c1038: Fn32 = v0.clone();
                        c1038.apply(v2.clone())
                    };
                    match m1039 {
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
                Ok(next) => { m1040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f242(v0)
}

pub fn f241(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f242(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h1043, _) = v0.clone();
        h1043
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h1044, _) = {
            let (_, h1045) = v0.clone();
            h1045
        };
        h1044
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h1046) = {
            let (_, h1047) = v0;
            h1047
        };
        h1046
    };
    let m1048 = v1.clone();
    match m1048.uncons() {
        None => {
            let m1049 = v2;
            match m1049.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1050 = v2.clone();
            match m1050.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f243({
                        let (h1051, _) = v8.clone();
                        h1051
                    }, {
                        let (h1052, _) = v6.clone();
                        h1052
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

pub fn f243(v0: Bytes, v1: Bytes) -> bool {
    let a1057 = {
        let a1053 = v0;
        let a1054 = v1;
        compare_bytes(a1053, a1054)
    };
    let a1058 = {
        let a1055 = Bytes::lit(&[]);
        let a1056 = Bytes::lit(&[0x01]);
        compare_bytes(a1055, a1056)
    };
    equal(a1057, a1058)
}
