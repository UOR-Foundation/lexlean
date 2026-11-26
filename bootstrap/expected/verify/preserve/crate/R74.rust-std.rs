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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4(std::rc::Rc<Adt0>),
    C5,
}

#[derive(Clone)]
pub enum Adt3 {
    C0(std::rc::Rc<Adt4>, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
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
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<Adt3>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<Adt7>, std::rc::Rc<Adt0>, u64, u64, std::rc::Rc<Adt2>, u64, Option<Adt1>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64, u64, Bytes, std::rc::Rc<Adt2>, Option<Adt0>, List<Adt0>, List<(Bytes, Adt0)>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Bytes>, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(std::rc::Rc<Adt0>, u64, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, u64, List<Adt14>, std::rc::Rc<Adt0>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Adt0>),
    C1(Bytes, u64, List<(Bytes, Adt0)>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Adt0>, u64, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(std::rc::Rc<Adt0>, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F8(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn0::F8(k0, k1) => f8(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(std::rc::Rc<Adt0>, u64, List<Adt3>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn1::F15(k0, k1, k2) => f15((**k0).clone(), *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn2::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt8) -> Option<Adt8> {
        match self {
            Fn3::F21(k0) => f21(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33(std::rc::Rc<Adt0>),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn4::F33(k0) => f33((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F52(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn5::F52(k0) => f52(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F67(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn6::F67(k0, k1) => f67(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F71(u64, Bytes),
    F180(Bytes),
    F181(Bytes),
    F194(Bytes),
    F209(u64, Bytes),
    F272(Bytes),
    F282(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F71(k0, k1) => f71(*k0, k1.clone(), p0),
            Fn7::F180(k0) => f180(k0.clone(), p0),
            Fn7::F181(k0) => f181(k0.clone(), p0),
            Fn7::F194(k0) => f194(k0.clone(), p0),
            Fn7::F209(k0, k1) => f209(*k0, k1.clone(), p0),
            Fn7::F272(k0) => f272(k0.clone(), p0),
            Fn7::F282(k0) => f282(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F130,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn8::F130 => f130(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F132,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F146(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn10::F146(k0) => f146(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F159(u64, List<Adt3>),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn11::F159(k0, k1) => f159(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F161(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn12::F161(k0) => f161(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F168(u64, List<Adt3>),
    F222(u64, List<Adt3>, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn13::F168(k0, k1) => f168(*k0, k1.clone(), p0),
            Fn13::F222(k0, k1, k2) => f222(*k0, k1.clone(), k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F178,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn14::F178 => f178(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F184(Bytes, std::rc::Rc<Adt0>),
}

impl Fn15 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn15::F184(k0, k1) => f184(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F187(std::rc::Rc<Adt0>),
    F236(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn16::F187(k0) => f187((**k0).clone(), p0, p1),
            Fn16::F236(k0) => Ok(f236(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F197,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn17::F197 => f197(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F201,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn18::F201 => f201(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F203,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn19::F203 => f203(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F211,
}

impl Fn20 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn20::F211 => f211(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F218,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt16) -> Option<Adt16> {
        match self {
            Fn21::F218 => f218(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F221(u64, Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn22::F221(k0, k1) => f221(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F226,
}

impl Fn23 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn23::F226 => f226(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F228,
}

impl Fn24 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn24::F228 => f228(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F238,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn25::F238 => f238(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F261,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn26::F261 => f261(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F275(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn27::F275(k0) => f275(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F277,
}

impl Fn28 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn28::F277 => f277(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Bytes, v2: u64) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v1.clone())?;
    match m1 {
        Ok(v3) => {
            let v4: Adt2 = f2(v0.clone(), v2)?;
            Ok(f3(f4(v3, {
                let a2 = v1;
                length_bytes(a2)
            }, v0, f5(f6(0u64, 1u64), v4.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), v4)?))
        }
        Err(v5) => {
            Ok(Err::<Adt0, Adt1>(v5))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt3>, Adt1>> {
    let v1: u64 = {
        let a3 = v0.clone();
        length_bytes(a3)
    };
    let v2: Adt5 = {
        let (h4, _) = f7(Fn0::F8(v1, v0), {
            let a5 = v1;
            let a6 = 1u64;
            nat_add(a5, a6)?
        }, Adt5::C0(0u64, List::<Adt3>::nil(), None::<Adt1>))?;
        h4
    };
    let m8 = match v2.clone() {
        Adt5::C0(_, _, h7) => {
            h7
        }
    };
    match m8 {
        None => {
            Ok(Ok::<List<Adt3>, Adt1>(f9(match v2 {
                Adt5::C0(_, h9, _) => {
                    h9
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt3>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: Adt0, v1: u64) -> R<Adt2> {
    let a10 = v1;
    let a11 = 0u64;
    if nat_eq(a10, a11) {
        let m12 = f10(v0.clone(), f11(Str::lit("properties")));
        match m12 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m13 = f10(v2, f11(Str::lit("declarations")));
                match m13 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f12(v0, v3)
                    }
                }
            }
        }
    } else {
        let a14 = v1;
        let a15 = 1u64;
        if nat_eq(a14, a15) {
            Ok(Adt2::C0)
        } else {
            let a16 = v1;
            let a17 = 2u64;
            if nat_eq(a16, a17) {
                Ok(Adt2::C1)
            } else {
                let m18 = f13(v0, f11(Str::lit("proof")));
                match m18 {
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

pub fn f3(v0: Adt6) -> Result<Adt0, Adt1> {
    let m20 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h19, _) => {
            h19
        }
    };
    match m20 {
        None => {
            let a22 = match v0.clone() {
                Adt6::C0(_, _, _, h21, _, _, _, _, _) => {
                    h21
                }
            };
            let a23 = 9u64;
            if nat_eq(a22, a23) {
                Ok::<Adt0, Adt1>(match v0 {
                    Adt6::C0(_, _, h24, _, _, _, _, _, _) => {
                        (*h24).clone()
                    }
                })
            } else {
                Err::<Adt0, Adt1>(Adt1::C0(0u64, f11(Str::lit("the reader ran out of steps"))))
            }
        }
        Some(v1) => {
            Err::<Adt0, Adt1>(v1)
        }
    }
}

pub fn f4(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt7, v4: Adt2) -> R<Adt6> {
    let v5: Adt6 = Adt6::C0(0u64, List::cons(v3, List::<Adt7>::nil()), std::rc::Rc::new(Adt0::C5), 0u64, 0u64, std::rc::Rc::new(v4), 1u64, None::<Adt1>, List::<(u64, u64)>::nil());
    let (h25, _) = f14(Fn1::F15(std::rc::Rc::new(v2), v1, v0.clone()), {
        let a29 = {
            let a27 = {
                let a26 = v0;
                length_list(a26)
            };
            let a28 = 16u64;
            nat_mul(a27, a28)?
        };
        let a30 = 64u64;
        nat_add(a29, a30)?
    }, v5)?;
    Ok(h25)
}

pub fn f5(v0: Adt7, v1: Adt2, v2: Option<Adt0>, v3: List<(Bytes, Adt0)>) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h31, _, _, _, _, _, _, _, _, _, _, _) => {
            h31
        }
    }, match v0.clone() {
        Adt7::C0(_, h32, _, _, _, _, _, _, _, _, _, _) => {
            h32
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h33, _, _, _, _, _, _, _, _, _) => {
            h33
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h34, _, _, _, _, _, _, _, _) => {
            h34
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h35, _, _, _, _, _, _, _) => {
            h35
        }
    }, std::rc::Rc::new(v1), v2, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h36, _, _, _, _) => {
            h36
        }
    }, v3, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h37, _, _) => {
            (*h37).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h38, _) => {
            (*h38).clone()
        }
    }), match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h39) => {
            h39
        }
    })
}

pub fn f6(v0: u64, v1: u64) -> Adt7 {
    Adt7::C0(v0, 0u64, v1, 0u64, f11(Str::lit("")), std::rc::Rc::new(Adt2::C5), None::<Adt0>, List::<Adt0>::nil(), List::<(Bytes, Adt0)>::nil(), std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C5), List::<(u64, u64)>::nil())
}

pub fn f7(v0: Fn0, p44: u64, p45: Adt5) -> R<(Adt5, bool)> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m40 = v1;
                if m40 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m40 - 1;
                    let m42 = {
                        let c41: Fn0 = v0.clone();
                        c41.apply(v2.clone())?
                    };
                    match m42 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f16(v1, v0, v2)
}

pub fn f9(v0: List<Adt3>) -> List<Adt3> {
    f17(Fn2::F18, List::<Adt3>::nil(), v0)
}

pub fn f10(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m46 = f19(v0);
    match m46 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            let m49 = {
                let (h48, _) = f20(Fn3::F21(v1), 1000000u64, Adt8::C0(std::rc::Rc::new(v2), None::<Adt0>));
                h48
            };
            match m49 {
                Adt8::C0(_, h47) => {
                    h47
                }
            }
        }
    }
}

pub fn f11(v0: Str) -> Bytes {
    let a50 = v0;
    utf8_encode(a50)
}

pub fn f12(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m51 = f10(f22(v0.clone(), v1)?, f11(Str::lit("items")));
    match m51 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f23(v0, v2)
        }
    }
}

pub fn f13(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m52 = f10(v0, f11(Str::lit("$defs")));
    match m52 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f10(v2, v1)
        }
    }
}

pub fn f14(v0: Fn1, p57: u64, p58: Adt6) -> R<(Adt6, bool)> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m53 = v1;
                if m53 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m53 - 1;
                    let m55 = {
                        let c54: Fn1 = v0.clone();
                        c54.apply(v2.clone())?
                    };
                    match m55 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Adt0, v1: u64, v2: List<Adt3>, v3: Adt6) -> R<Option<Adt6>> {
    f24(v2, v1, v0, v3)
}

pub fn f16(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m60 = match v2.clone() {
        Adt5::C0(_, _, h59) => {
            h59
        }
    };
    match m60 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt5::C0(h61, _, _) => {
                    h61
                }
            };
            let a62 = v4;
            let a63 = v1;
            if nat_lt(a62, a63) {
                let v5: u64 = f25(v0.clone(), v4)?;
                let a64 = 0u64;
                let a65 = v5;
                if nat_lt(a64, a65) {
                    Ok(Some(Adt5::C0({
                        let a66 = v4;
                        let a67 = v5;
                        nat_add(a66, a67)?
                    }, match v2 {
                        Adt5::C0(_, h68, _) => {
                            h68
                        }
                    }, None::<Adt1>)))
                } else {
                    if f26(v0.clone(), v4, 123u8) {
                        Ok(f27(v2, Adt4::C0, {
                            let a69 = v4;
                            let a70 = 1u64;
                            nat_add(a69, a70)?
                        }))
                    } else {
                        if f26(v0.clone(), v4, 125u8) {
                            Ok(f27(v2, Adt4::C1, {
                                let a71 = v4;
                                let a72 = 1u64;
                                nat_add(a71, a72)?
                            }))
                        } else {
                            if f26(v0.clone(), v4, 91u8) {
                                Ok(f27(v2, Adt4::C2, {
                                    let a73 = v4;
                                    let a74 = 1u64;
                                    nat_add(a73, a74)?
                                }))
                            } else {
                                if f26(v0.clone(), v4, 93u8) {
                                    Ok(f27(v2, Adt4::C3, {
                                        let a75 = v4;
                                        let a76 = 1u64;
                                        nat_add(a75, a76)?
                                    }))
                                } else {
                                    if f26(v0.clone(), v4, 40u8) {
                                        Ok(f27(v2, Adt4::C4, {
                                            let a77 = v4;
                                            let a78 = 1u64;
                                            nat_add(a77, a78)?
                                        }))
                                    } else {
                                        if f26(v0.clone(), v4, 41u8) {
                                            Ok(f27(v2, Adt4::C5, {
                                                let a79 = v4;
                                                let a80 = 1u64;
                                                nat_add(a79, a80)?
                                            }))
                                        } else {
                                            if f26(v0.clone(), v4, 44u8) {
                                                Ok(f27(v2, Adt4::C6, {
                                                    let a81 = v4;
                                                    let a82 = 1u64;
                                                    nat_add(a81, a82)?
                                                }))
                                            } else {
                                                if f26(v0.clone(), v4, 59u8) {
                                                    Ok(f27(v2, Adt4::C10, {
                                                        let a83 = v4;
                                                        let a84 = 1u64;
                                                        nat_add(a83, a84)?
                                                    }))
                                                } else {
                                                    if f26(v0.clone(), v4, 58u8) {
                                                        f28(v0, v2)
                                                    } else {
                                                        if f26(v0.clone(), v4, 34u8) {
                                                            f29(v0, v1, v2)
                                                        } else {
                                                            f30(v0, v1, v2)
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
                Ok(None::<Adt5>)
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f17(v0: Fn2, p88: List<Adt3>, p89: List<Adt3>) -> List<Adt3> {
    let m87 = (p88, p89);
    {
        let mut m87 = m87;
        loop {
            let step = {
                let (v1, v2) = m87;
                let m85 = v2;
                match m85.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c86: Fn2 = v0.clone();
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

pub fn f18(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f19(v0: Adt0) -> Option<Adt0> {
    let m90 = v0;
    match m90 {
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
        Adt0::C4(r91) => {
            let v1 = (*r91).clone();
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

pub fn f20(v0: Fn3, p96: u64, p97: Adt8) -> (Adt8, bool) {
    let m95 = (p96, p97);
    {
        let mut m95 = m95;
        loop {
            let step = {
                let (v1, v2) = m95;
                let m92 = v1;
                if m92 == 0 {
                    Err::<(u64, Adt8), (Adt8, bool)>((v2, false))
                } else {
                    let v3 = m92 - 1;
                    let m94 = {
                        let c93: Fn3 = v0.clone();
                        c93.apply(v2.clone())
                    };
                    match m94 {
                        None => {
                            Err::<(u64, Adt8), (Adt8, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), (Adt8, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m95 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    f31(v0, v1)
}

pub fn f22(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h98, _) = f32(Fn4::F33(std::rc::Rc::new(v0)), 4096u64, v1)?;
    Ok(h98)
}

pub fn f23(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m99 = f34(f10(v1.clone(), f11(Str::lit("$ref"))));
    match m99 {
        None => {
            Ok(Adt2::C4(std::rc::Rc::new(v1)))
        }
        Some(v2) => {
            let v3: Bytes = f35(v2)?;
            if f36(v3.clone(), f11(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f36(v3.clone(), f11(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f36(v3.clone(), f11(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f36(v3, f11(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(std::rc::Rc::new(f22(v0, v1)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f24(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Option<Adt6>> {
    let m101 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h100, _) => {
            h100
        }
    };
    match m101 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt6::C0(_, _, _, h102, _, _, _, _, _) => {
                    h102
                }
            };
            let a103 = v5;
            let a104 = 9u64;
            if nat_eq(a103, a104) {
                Ok(None::<Adt6>)
            } else {
                Ok(Some({
                    let a105 = v5;
                    let a106 = 0u64;
                    if nat_eq(a105, a106) {
                        f37(v0, v1, v2, v3)?
                    } else {
                        f38(v0, v1, v2, v3)?
                    }
                }))
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    if if f39(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f40(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f40(v0.clone(), v1, 194u8) {
            if if f40(v0.clone(), {
                let a107 = v1;
                let a108 = 1u64;
                nat_add(a107, a108)?
            }, 133u8) {
                true
            } else {
                f40(v0, {
                    let a109 = v1;
                    let a110 = 1u64;
                    nat_add(a109, a110)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f40(v0.clone(), v1, 225u8) {
                if if f40(v0.clone(), {
                    let a111 = v1;
                    let a112 = 1u64;
                    nat_add(a111, a112)?
                }, 154u8) {
                    f40(v0, {
                        let a113 = v1;
                        let a114 = 2u64;
                        nat_add(a113, a114)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f40(v0.clone(), v1, 226u8) {
                    if f40(v0.clone(), {
                        let a115 = v1;
                        let a116 = 1u64;
                        nat_add(a115, a116)?
                    }, 128u8) {
                        if if f39(v0.clone(), {
                            let a117 = v1;
                            let a118 = 2u64;
                            nat_add(a117, a118)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f40(v0.clone(), {
                                let a119 = v1;
                                let a120 = 2u64;
                                nat_add(a119, a120)?
                            }, 168u8) {
                                true
                            } else {
                                if f40(v0.clone(), {
                                    let a121 = v1;
                                    let a122 = 2u64;
                                    nat_add(a121, a122)?
                                }, 169u8) {
                                    true
                                } else {
                                    f40(v0, {
                                        let a123 = v1;
                                        let a124 = 2u64;
                                        nat_add(a123, a124)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f40(v0.clone(), {
                            let a125 = v1;
                            let a126 = 1u64;
                            nat_add(a125, a126)?
                        }, 129u8) {
                            f40(v0, {
                                let a127 = v1;
                                let a128 = 2u64;
                                nat_add(a127, a128)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f40(v0.clone(), v1, 227u8) {
                        if if f40(v0.clone(), {
                            let a129 = v1;
                            let a130 = 1u64;
                            nat_add(a129, a130)?
                        }, 128u8) {
                            f40(v0, {
                                let a131 = v1;
                                let a132 = 2u64;
                                nat_add(a131, a132)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m133 = f41(v0, v1);
    match m133 {
        None => {
            false
        }
        Some(v3) => {
            let a134 = v3;
            let a135 = v2;
            equal(a134, a135)
        }
    }
}

pub fn f27(v0: Adt5, v1: Adt4, v2: u64) -> Option<Adt5> {
    Some(Adt5::C0(v2, List::cons(Adt3::C0(std::rc::Rc::new(v1), match v0.clone() {
        Adt5::C0(h136, _, _) => {
            h136
        }
    }, v2), match v0 {
        Adt5::C0(_, h137, _) => {
            h137
        }
    }), None::<Adt1>))
}

pub fn f28(v0: Bytes, v1: Adt5) -> R<Option<Adt5>> {
    let v2: u64 = match v1.clone() {
        Adt5::C0(h138, _, _) => {
            h138
        }
    };
    if f26(v0.clone(), {
        let a139 = v2;
        let a140 = 1u64;
        nat_add(a139, a140)?
    }, 61u8) {
        Ok(f27(v1, Adt4::C8, {
            let a141 = v2;
            let a142 = 2u64;
            nat_add(a141, a142)?
        }))
    } else {
        if f26(v0, {
            let a143 = v2;
            let a144 = 1u64;
            nat_add(a143, a144)?
        }, 58u8) {
            Ok(f27(v1, Adt4::C9, {
                let a145 = v2;
                let a146 = 2u64;
                nat_add(a145, a146)?
            }))
        } else {
            Ok(f27(v1, Adt4::C7, {
                let a147 = v2;
                let a148 = 1u64;
                nat_add(a147, a148)?
            }))
        }
    }
}

pub fn f29(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h149, _, _) => {
            h149
        }
    };
    let v4: Adt9 = f42(v0, v1, v3)?;
    let m151 = match v4.clone() {
        Adt9::C0(_, _, _, h150) => {
            h150
        }
    };
    match m151 {
        None => {
            if match v4.clone() {
                Adt9::C0(_, _, h153, _) => {
                    h153
                }
            } {
                Ok(f27(v2, Adt4::C11(f43(match v4.clone() {
                    Adt9::C0(_, h154, _, _) => {
                        h154
                    }
                })), match v4 {
                    Adt9::C0(h155, _, _, _) => {
                        h155
                    }
                }))
            } else {
                Ok(Some(Adt5::C0(v3, match v2 {
                    Adt5::C0(_, h156, _) => {
                        h156
                    }
                }, Some(f44(v3, f45(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt5::C0(v3, match v2 {
                Adt5::C0(_, h152, _) => {
                    h152
                }
            }, Some(v5))))
        }
    }
}

pub fn f30(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h157, _, _) => {
            h157
        }
    };
    let v4: u64 = f46(v0.clone(), v1, v3)?;
    Ok(f27(v2, Adt4::C12(f47(v0, v3, {
        let a158 = v4;
        let a159 = v3;
        nat_sub(a158, a159)
    })), v4))
}

pub fn f31(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    let m161 = match v1.clone() {
        Adt8::C0(_, h160) => {
            h160
        }
    };
    match m161 {
        None => {
            let m163 = f48(match v1 {
                Adt8::C0(h162, _) => {
                    (*h162).clone()
                }
            });
            match m163 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    if f36(match v3.clone() {
                        Adt11::C0(h164, _, _) => {
                            h164
                        }
                    }, v0) {
                        Some(Adt8::C0(std::rc::Rc::new(Adt0::C5), Some(match v3 {
                            Adt11::C0(_, h165, _) => {
                                (*h165).clone()
                            }
                        })))
                    } else {
                        Some(Adt8::C0(std::rc::Rc::new(match v3 {
                            Adt11::C0(_, _, h166) => {
                                (*h166).clone()
                            }
                        }), None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt8>
        }
    }
}

pub fn f32(v0: Fn4, p171: u64, p172: Adt0) -> R<(Adt0, bool)> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m167 = v1;
                if m167 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m167 - 1;
                    let m169 = {
                        let c168: Fn4 = v0.clone();
                        c168.apply(v2.clone())?
                    };
                    match m169 {
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
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f49(v0, v1)
}

pub fn f34(v0: Option<Adt0>) -> Option<Bytes> {
    let m173 = v0;
    match m173 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f50(v1)
        }
    }
}

pub fn f35(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m179 = {
            let (h175, _) = f51(Fn5::F52(v0.clone()), {
                let a177 = {
                    let a176 = v0.clone();
                    length_bytes(a176)
                };
                let a178 = 1u64;
                nat_add(a177, a178)?
            }, Adt10::C0(0u64, 0u64))?;
            h175
        };
        match m179 {
            Adt10::C0(_, h174) => {
                h174
            }
        }
    };
    Ok(f47(v0.clone(), v1, {
        let a181 = {
            let a180 = v0;
            length_bytes(a180)
        };
        let a182 = v1;
        nat_sub(a181, a182)
    }))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a183 = v0;
    let a184 = v1;
    equal(a183, a184)
}

pub fn f37(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, h185, _, _, _, _) => {
            h185
        }
    };
    let a186 = v4;
    let a187 = 0u64;
    if nat_eq(a186, a187) {
        f53(v0, v1, v2, v3)
    } else {
        let a188 = v4;
        let a189 = 1u64;
        if nat_eq(a188, a189) {
            f54(v0, v1, v2, v3)
        } else {
            let a190 = v4;
            let a191 = 2u64;
            if nat_eq(a190, a191) {
                f55(v0, v1, v3)
            } else {
                let a192 = v4;
                let a193 = 3u64;
                if nat_eq(a192, a193) {
                    f56(v0, v1, v3)
                } else {
                    let a194 = v4;
                    let a195 = 4u64;
                    if nat_eq(a194, a195) {
                        f57(v0, v1, v3)
                    } else {
                        let a196 = v4;
                        let a197 = 5u64;
                        if nat_eq(a196, a197) {
                            f58(v0, v1, v3.clone(), match v3 {
                                Adt6::C0(_, _, _, _, _, _, h198, _, _) => {
                                    h198
                                }
                            })
                        } else {
                            f59(v0, v1, v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f38(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let m200 = match v3.clone() {
        Adt6::C0(_, h199, _, _, _, _, _, _, _) => {
            h199
        }
    };
    match m200.uncons() {
        None => {
            Ok(f60(v3.clone(), match v3 {
                Adt6::C0(_, _, h201, _, _, _, _, _, _) => {
                    (*h201).clone()
                }
            }, List::<(u64, u64)>::nil()))
        }
        Some((v4, v5)) => {
            let v6: Adt6 = Adt6::C0(match v3.clone() {
                Adt6::C0(h202, _, _, _, _, _, _, _, _) => {
                    h202
                }
            }, v5, std::rc::Rc::new(match v3.clone() {
                Adt6::C0(_, _, h203, _, _, _, _, _, _) => {
                    (*h203).clone()
                }
            }), 1u64, match v3.clone() {
                Adt6::C0(_, _, _, _, h204, _, _, _, _) => {
                    h204
                }
            }, std::rc::Rc::new(match v3.clone() {
                Adt6::C0(_, _, _, _, _, h205, _, _, _) => {
                    (*h205).clone()
                }
            }), match v3.clone() {
                Adt6::C0(_, _, _, _, _, _, h206, _, _) => {
                    h206
                }
            }, None::<Adt1>, match v3 {
                Adt6::C0(_, _, _, _, _, _, _, _, h207) => {
                    h207
                }
            });
            let v7: u64 = match v4.clone() {
                Adt7::C0(h208, _, _, _, _, _, _, _, _, _, _, _) => {
                    h208
                }
            };
            let a209 = v7;
            let a210 = 0u64;
            if nat_eq(a209, a210) {
                Ok(f61(v0, v1, v2, v6, v4))
            } else {
                let a211 = v7;
                let a212 = 10u64;
                if nat_lt(a211, a212) {
                    f62(v0, v1, v2, v6, v4)
                } else {
                    let a213 = v7;
                    let a214 = 30u64;
                    if nat_lt(a213, a214) {
                        f63(v0, v1, v6, v4)
                    } else {
                        f64(v0, v1, v6, v4)
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m215 = f41(v0, v1);
    match m215 {
        None => {
            false
        }
        Some(v4) => {
            f65(v4, v2, v3)
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m216 = f41(v0, v1);
    match m216 {
        None => {
            false
        }
        Some(v3) => {
            let a217 = v3;
            let a218 = v2;
            equal(a217, a218)
        }
    }
}

pub fn f41(v0: Bytes, v1: u64) -> Option<u8> {
    let a219 = v0;
    let a220 = v1;
    index_bytes(a219, a220)
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> R<Adt9> {
    let (h221, _) = f66(Fn6::F67(v1, v0), {
        let a224 = {
            let a222 = v1;
            let a223 = v2;
            nat_sub(a222, a223)
        };
        let a225 = 1u64;
        nat_add(a224, a225)?
    }, Adt9::C0({
        let a226 = v2;
        let a227 = 1u64;
        nat_add(a226, a227)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h221)
}

pub fn f43(v0: List<Bytes>) -> Bytes {
    f68(f69(v0))
}

pub fn f44(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f45(v0: Str) -> Bytes {
    let a228 = v0;
    utf8_encode(a228)
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h229, _) = f70(Fn7::F71(v1, v0), {
        let a230 = v1;
        let a231 = v2;
        nat_sub(a230, a231)
    }, v2)?;
    Ok(h229)
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m235 = {
        let a232 = v0;
        let a233 = v1;
        let a234 = v2;
        slice_bytes(a232, a233, a234)
    };
    match m235 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f48(v0: Adt0) -> Option<Adt11> {
    let m236 = v0;
    match m236 {
        Adt0::C0(_) => {
            None::<Adt11>
        }
        Adt0::C1(_, _) => {
            None::<Adt11>
        }
        Adt0::C2(_) => {
            None::<Adt11>
        }
        Adt0::C3(_) => {
            None::<Adt11>
        }
        Adt0::C4(_) => {
            None::<Adt11>
        }
        Adt0::C5 => {
            None::<Adt11>
        }
        Adt0::C6(_, _) => {
            None::<Adt11>
        }
        Adt0::C7(v1, r237, r238) => {
            let v2 = (*r237).clone();
            let v3 = (*r238).clone();
            Some(Adt11::C0(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3)))
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m239 = f34(f10(v1, f11(Str::lit("$ref"))));
    match m239 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f13(v0, f35(v2)?))
        }
    }
}

pub fn f50(v0: Adt0) -> Option<Bytes> {
    let m240 = v0;
    match m240 {
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

pub fn f51(v0: Fn5, p245: u64, p246: Adt10) -> R<(Adt10, bool)> {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, v2) = m244;
                let m241 = v1;
                if m241 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m241 - 1;
                    let m243 = {
                        let c242: Fn5 = v0.clone();
                        c242.apply(v2.clone())?
                    };
                    match m243 {
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
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    f72(v0, v1)
}

pub fn f53(v0: List<Adt3>, v1: u64, _: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h247, _, _, _, _, _, _, _, _) => {
            h247
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, h248, _, _) => {
            h248
        }
    };
    let a249 = 256u64;
    let a250 = v5;
    if nat_lt(a249, a250) {
        Ok(f73(v0, v1, v3, v4, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let m252 = match v3.clone() {
            Adt6::C0(_, _, _, _, _, h251, _, _, _) => {
                (*h251).clone()
            }
        };
        match m252 {
            Adt2::C0 => {
                Ok(f74(v3, v4, 3u64, Adt2::C0, v5))
            }
            Adt2::C1 => {
                Ok(f74(v3, v4, 2u64, Adt2::C1, v5))
            }
            Adt2::C2 => {
                let a253 = f75(v0.clone(), v4);
                let a254 = 0u64;
                if nat_eq(a253, a254) {
                    f58(v0, v1, v3, 0u64)
                } else {
                    let v6: Adt12 = f76(v0, v1, v4)?;
                    let m256 = match v6.clone() {
                        Adt12::C0(_, _, h255) => {
                            h255
                        }
                    };
                    match m256 {
                        None => {
                            Ok(f78(v3, match v6.clone() {
                                Adt12::C0(_, h258, _) => {
                                    h258
                                }
                            }, match v6 {
                                Adt12::C0(h259, _, _) => {
                                    (*h259).clone()
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(f77(v3, match v6 {
                                Adt12::C0(_, _, h257) => {
                                    h257
                                }
                            }))
                        }
                    }
                }
            }
            Adt2::C3 => {
                Ok(f74(v3, v4, 4u64, Adt2::C3, v5))
            }
            Adt2::C4(_) => {
                Ok(f74(v3.clone(), v4, 1u64, match v3 {
                    Adt6::C0(_, _, _, _, _, h260, _, _, _) => {
                        (*h260).clone()
                    }
                }, v5))
            }
            Adt2::C5 => {
                Ok(f74(v3, v4, 5u64, Adt2::C5, v5))
            }
        }
    }
}

pub fn f54(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6) -> R<Adt6> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h261, _, _, _, _, _, _, _, _) => {
            h261
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, _, h262, _, _) => {
            h262
        }
    };
    let v6: Adt2 = match v3.clone() {
        Adt6::C0(_, _, _, _, _, h263, _, _, _) => {
            (*h263).clone()
        }
    };
    let v7: u64 = f75(v0.clone(), v4);
    let a264 = v7;
    let a265 = 2u64;
    if nat_eq(a264, a265) {
        let a268 = f75(v0, {
            let a266 = v4;
            let a267 = 1u64;
            nat_add(a266, a267)?
        });
        let a269 = 3u64;
        if nat_eq(a268, a269) {
            Ok(f78(v3, {
                let a270 = v4;
                let a271 = 2u64;
                nat_add(a270, a271)?
            }, f79(List::<Adt0>::nil())))
        } else {
            let v8: Adt2 = f80(v2, v6)?;
            Ok(f81(v3, f5(f6(1u64, v5), v8.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), {
                let a272 = v4;
                let a273 = 1u64;
                nat_add(a272, a273)?
            }, 0u64, v8, {
                let a274 = v5;
                let a275 = 1u64;
                nat_add(a274, a275)?
            }))
        }
    } else {
        let a276 = v7;
        let a277 = 0u64;
        if nat_eq(a276, a277) {
            f82(v0, v1, v2.clone(), v3, f5(f6(2u64, v5), v6.clone(), f83(v2, v6), List::<(Bytes, Adt0)>::nil()), {
                let a278 = v4;
                let a279 = 1u64;
                nat_add(a278, a279)?
            })
        } else {
            let v9: Bytes = f84(v0.clone(), v4);
            if f85({
                let a280 = v7;
                let a281 = 12u64;
                nat_eq(a280, a281)
            }, f85({
                let a284 = f75(v0.clone(), {
                    let a282 = v4;
                    let a283 = 1u64;
                    nat_add(a282, a283)?
                });
                let a285 = 0u64;
                nat_eq(a284, a285)
            }, f85(f86(v9.clone())?, !f87(v9.clone())?))) {
                f82(v0, v1, v2.clone(), v3, f5(f6(2u64, v5), v6.clone(), f88(v2, v6, v9.clone())?, List::cons((f11(Str::lit("kind")), f89(v9)), List::<(Bytes, Adt0)>::nil())), {
                    let a286 = v4;
                    let a287 = 2u64;
                    nat_add(a286, a287)?
                })
            } else {
                Ok(f74(v3, v4, 5u64, v6, v5))
            }
        }
    }
}

pub fn f55(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h288, _, _, _, _, _, _, _, _) => {
            h288
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h289, _, _) => {
            h289
        }
    };
    let a290 = 256u64;
    let a291 = v4;
    if nat_lt(a290, a291) {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f75(v0.clone(), v3);
        let v6: Bytes = f84(v0.clone(), v3);
        let a292 = v5;
        let a293 = 12u64;
        if nat_eq(a292, a293) {
            if f85({
                let a296 = f75(v0.clone(), {
                    let a294 = v3;
                    let a295 = 1u64;
                    nat_add(a294, a295)?
                });
                let a297 = 0u64;
                nat_eq(a296, a297)
            }, f86(v6.clone())?) {
                Ok(f74(v2, v3, 1u64, Adt2::C1, v4))
            } else {
                let m298 = f90(v6.clone());
                match m298 {
                    None => {
                        if f85(f92(v6.clone(), f11(Str::lit("'"))), f86(f93(v6.clone(), 1u64))?) {
                            Ok(f78(v2, {
                                let a301 = v3;
                                let a302 = 1u64;
                                nat_add(a301, a302)?
                            }, f91(f11(Str::lit("parameter")), f94(Str::lit("name"), f89(f93(v6, 1u64))))?))
                        } else {
                            if f95(v6.clone(), f96()) {
                                Ok(f73(v0, v1, v2, v3, f97(f97(f97(f97(f11(Str::lit("`")), v6.clone()), f11(Str::lit("` takes arguments: write `("))), v6), f11(Str::lit(" \u{2026})`")))))
                            } else {
                                let v8: Adt12 = f76(v0, v1, v3)?;
                                let m304 = match v8.clone() {
                                    Adt12::C0(_, _, h303) => {
                                        h303
                                    }
                                };
                                match m304 {
                                    None => {
                                        Ok(f78(v2, match v8.clone() {
                                            Adt12::C0(_, h306, _) => {
                                                h306
                                            }
                                        }, f91(f11(Str::lit("named")), f98(Str::lit("member"), match v8 {
                                            Adt12::C0(h307, _, _) => {
                                                (*h307).clone()
                                            }
                                        }, Str::lit("arguments"), f79(List::<Adt0>::nil())))?))
                                    }
                                    Some(_) => {
                                        Ok(f77(v2, match v8 {
                                            Adt12::C0(_, _, h305) => {
                                                h305
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    Some(v7) => {
                        Ok(f78(v2, {
                            let a299 = v3;
                            let a300 = 1u64;
                            nat_add(a299, a300)?
                        }, f91(v7, List::<(Bytes, Adt0)>::nil())?))
                    }
                }
            }
        } else {
            let a308 = v5;
            let a309 = 4u64;
            if nat_eq(a308, a309) {
                f99(v0, v1, f81(v2, f6(11u64, v4), {
                    let a310 = v3;
                    let a311 = 1u64;
                    nat_add(a310, a311)?
                }, 2u64, Adt2::C1, v4), {
                    let a312 = v3;
                    let a313 = 1u64;
                    nat_add(a312, a313)?
                }, v4)
            } else {
                Ok(f73(v0.clone(), v1, v2, v3, f97(f11(Str::lit("expected a type, found ")), f100(v0, v3)?)))
            }
        }
    }
}

pub fn f56(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h314, _, _, _, _, _, _, _, _) => {
            h314
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h315, _, _) => {
            h315
        }
    };
    let a316 = 256u64;
    let a317 = v4;
    if nat_lt(a316, a317) {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f75(v0.clone(), v3);
        let v6: Bytes = f84(v0.clone(), v3);
        let a318 = v5;
        let a319 = 11u64;
        if nat_eq(a318, a319) {
            Ok(f78(v2, {
                let a320 = v3;
                let a321 = 1u64;
                nat_add(a320, a321)?
            }, f91(f11(Str::lit("string")), f94(Str::lit("value"), f89(v6)))?))
        } else {
            let a322 = v5;
            let a323 = 4u64;
            if nat_eq(a322, a323) {
                f101(v0, v1, f81(v2, f6(30u64, v4), {
                    let a324 = v3;
                    let a325 = 1u64;
                    nat_add(a324, a325)?
                }, 3u64, Adt2::C0, v4), {
                    let a326 = v3;
                    let a327 = 1u64;
                    nat_add(a326, a327)?
                }, v4)
            } else {
                let a328 = v5;
                let a329 = 12u64;
                if nat_eq(a328, a329) {
                    if f85({
                        let a332 = f75(v0.clone(), {
                            let a330 = v3;
                            let a331 = 1u64;
                            nat_add(a330, a331)?
                        });
                        let a333 = 0u64;
                        nat_eq(a332, a333)
                    }, f86(v6.clone())?) {
                        Ok(f74(v2, v3, 1u64, Adt2::C0, v4))
                    } else {
                        f102(v0, v1, v2, v3, v6)
                    }
                } else {
                    Ok(f73(v0.clone(), v1, v2, v3, f97(f11(Str::lit("expected a term, found ")), f100(v0, v3)?)))
                }
            }
        }
    }
}

pub fn f57(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h334, _, _, _, _, _, _, _, _) => {
            h334
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, _, _, _, _, _, h335, _, _) => {
            h335
        }
    };
    let a336 = f75(v0.clone(), v3);
    let a337 = 0u64;
    if nat_eq(a336, a337) {
        Ok(f74(v2, v3, 1u64, Adt2::C3, v4))
    } else {
        let v5: Adt12 = f103(v0.clone(), v1, v3, f11(Str::lit("a parameter name")))?;
        let m339 = match v5.clone() {
            Adt12::C0(_, _, h338) => {
                h338
            }
        };
        match m339 {
            None => {
                let a342 = f75(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h341, _) => {
                        h341
                    }
                });
                let a343 = 7u64;
                if nat_eq(a342, a343) {
                    Ok(f81(v2, f104(f6(20u64, v4), match v5.clone() {
                        Adt12::C0(h344, _, _) => {
                            (*h344).clone()
                        }
                    }, Adt0::C5), {
                        let a346 = match v5 {
                            Adt12::C0(_, h345, _) => {
                                h345
                            }
                        };
                        let a347 = 1u64;
                        nat_add(a346, a347)?
                    }, 2u64, Adt2::C1, {
                        let a348 = v4;
                        let a349 = 1u64;
                        nat_add(a348, a349)?
                    }))
                } else {
                    Ok(f73(v0, v1, v2, match v5 {
                        Adt12::C0(_, h350, _) => {
                            h350
                        }
                    }, f11(Str::lit("expected `:` after a parameter name"))))
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h340) => {
                        h340
                    }
                }))
            }
        }
    }
}

pub fn f58(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64) -> R<Adt6> {
    let v4: Adt13 = f105(v0, v1, match v2.clone() {
        Adt6::C0(h351, _, _, _, _, _, _, _, _) => {
            h351
        }
    }, v3)?;
    let m353 = match v4.clone() {
        Adt13::C0(_, _, _, _, _, h352) => {
            h352
        }
    };
    match m353 {
        None => {
            Ok(f78(v2, match v4.clone() {
                Adt13::C0(h355, _, _, _, _, _) => {
                    h355
                }
            }, match v4 {
                Adt13::C0(_, _, _, _, h356, _) => {
                    (*h356).clone()
                }
            }))
        }
        Some(_) => {
            Ok(f77(v2, match v4 {
                Adt13::C0(_, _, _, _, _, h354) => {
                    h354
                }
            }))
        }
    }
}

pub fn f59(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Adt6> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h357, _, _, _, _, _, _, _, _) => {
            h357
        }
    };
    let a358 = f75(v0.clone(), v3);
    let a359 = 2u64;
    if nat_eq(a358, a359) {
        f106(v0, v1, v2.clone(), f6(18u64, match v2 {
            Adt6::C0(_, _, _, _, _, _, h360, _, _) => {
                h360
            }
        }), {
            let a361 = v3;
            let a362 = 1u64;
            nat_add(a361, a362)?
        })
    } else {
        Ok(f73(v0, v1, v2, v3, f11(Str::lit("expected `[`"))))
    }
}

pub fn f60(v0: Adt6, v1: Adt0, v2: List<(u64, u64)>) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h363, _, _, _, _, _, _, _, _) => {
            h363
        }
    }, match v0.clone() {
        Adt6::C0(_, h364, _, _, _, _, _, _, _) => {
            h364
        }
    }, std::rc::Rc::new(v1), 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h365, _, _, _, _) => {
            h365
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h366, _, _, _) => {
            (*h366).clone()
        }
    }), match v0 {
        Adt6::C0(_, _, _, _, _, _, h367, _, _) => {
            h367
        }
    }, None::<Adt1>, v2)
}

pub fn f61(v0: List<Adt3>, v1: u64, _: Adt0, v3: Adt6, v4: Adt7) -> Adt6 {
    let v5: u64 = match v3.clone() {
        Adt6::C0(h368, _, _, _, _, _, _, _, _) => {
            h368
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt6::C0(_, _, h369, _, _, _, _, _, _) => {
            (*h369).clone()
        }
    };
    let a371 = match v4.clone() {
        Adt7::C0(_, h370, _, _, _, _, _, _, _, _, _, _) => {
            h370
        }
    };
    let a372 = 0u64;
    if nat_eq(a371, a372) {
        let a373 = f75(v0.clone(), v5);
        let a374 = 99u64;
        if nat_eq(a373, a374) {
            f60(v3, v6, List::<(u64, u64)>::nil())
        } else {
            f73(v0, v1, v3, v5, f11(Str::lit("unexpected text after the value")))
        }
    } else {
        let v7: Adt7 = f107(f108(v4.clone(), v6), (match v4.clone() {
            Adt7::C0(_, _, _, h375, _, _, _, _, _, _, _, _) => {
                h375
            }
        }, f109(v0.clone(), v5)), 0u64);
        let a376 = f75(v0.clone(), v5);
        let a377 = 99u64;
        if nat_eq(a376, a377) {
            f60(v3, f79(match v7.clone() {
                Adt7::C0(_, _, _, _, _, _, _, h378, _, _, _, _) => {
                    h378
                }
            }), match v7 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h379) => {
                    h379
                }
            })
        } else {
            f81(v3, f110(v7, f11(Str::lit("")), f111(v0, v1, v5)), v5, 0u64, match v4 {
                Adt7::C0(_, _, _, _, _, h380, _, _, _, _, _, _) => {
                    (*h380).clone()
                }
            }, 1u64)
        }
    }
}

pub fn f62(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6, v4: Adt7) -> R<Adt6> {
    let v5: u64 = match v3.clone() {
        Adt6::C0(h381, _, _, _, _, _, _, _, _) => {
            h381
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt6::C0(_, _, h382, _, _, _, _, _, _) => {
            (*h382).clone()
        }
    };
    let a384 = match v4.clone() {
        Adt7::C0(h383, _, _, _, _, _, _, _, _, _, _, _) => {
            h383
        }
    };
    let a385 = 1u64;
    if nat_eq(a384, a385) {
        let v7: Adt7 = f108(v4.clone(), v6);
        let a386 = f75(v0.clone(), v5);
        let a387 = 6u64;
        if nat_eq(a386, a387) {
            let a390 = f75(v0, {
                let a388 = v5;
                let a389 = 1u64;
                nat_add(a388, a389)?
            });
            let a391 = 3u64;
            if nat_eq(a390, a391) {
                Ok(f78(v3, {
                    let a392 = v5;
                    let a393 = 2u64;
                    nat_add(a392, a393)?
                }, f79(match v7 {
                    Adt7::C0(_, _, _, _, _, _, _, h394, _, _, _, _) => {
                        h394
                    }
                })))
            } else {
                Ok(f81(v3, v7, {
                    let a395 = v5;
                    let a396 = 1u64;
                    nat_add(a395, a396)?
                }, 0u64, match v4.clone() {
                    Adt7::C0(_, _, _, _, _, h397, _, _, _, _, _, _) => {
                        (*h397).clone()
                    }
                }, {
                    let a399 = match v4 {
                        Adt7::C0(_, _, h398, _, _, _, _, _, _, _, _, _) => {
                            h398
                        }
                    };
                    let a400 = 1u64;
                    nat_add(a399, a400)?
                }))
            }
        } else {
            let a401 = f75(v0.clone(), v5);
            let a402 = 3u64;
            if nat_eq(a401, a402) {
                Ok(f78(v3, {
                    let a403 = v5;
                    let a404 = 1u64;
                    nat_add(a403, a404)?
                }, f79(match v7 {
                    Adt7::C0(_, _, _, _, _, _, _, h405, _, _, _, _) => {
                        h405
                    }
                })))
            } else {
                Ok(f73(v0, v1, v3, v5, f11(Str::lit("expected `,` or `]`"))))
            }
        }
    } else {
        let v8: Bytes = match v4.clone() {
            Adt7::C0(_, _, _, _, h406, _, _, _, _, _, _, _) => {
                h406
            }
        };
        if f112(match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h407, _, _, _) => {
                h407
            }
        }, v8.clone()) {
            Ok(f113(v3, match v4 {
                Adt7::C0(_, _, _, h408, _, _, _, _, _, _, _, _) => {
                    h408
                }
            }, f97(f97(f11(Str::lit("the member `")), v8), f11(Str::lit("` is written twice")))))
        } else {
            let v9: Adt7 = f5(v4.clone(), match v4.clone() {
                Adt7::C0(_, _, _, _, _, h409, _, _, _, _, _, _) => {
                    (*h409).clone()
                }
            }, match v4.clone() {
                Adt7::C0(_, _, _, _, _, _, h410, _, _, _, _, _) => {
                    h410
                }
            }, List::cons((v8, v6), match v4 {
                Adt7::C0(_, _, _, _, _, _, _, _, h411, _, _, _) => {
                    h411
                }
            }));
            let a412 = f75(v0.clone(), v5);
            let a413 = 6u64;
            if nat_eq(a412, a413) {
                f82(v0, v1, v2, v3, v9, {
                    let a414 = v5;
                    let a415 = 1u64;
                    nat_add(a414, a415)?
                })
            } else {
                let a416 = f75(v0.clone(), v5);
                let a417 = 1u64;
                if nat_eq(a416, a417) {
                    Ok(f78(v3, {
                        let a418 = v5;
                        let a419 = 1u64;
                        nat_add(a418, a419)?
                    }, f114(match v9 {
                        Adt7::C0(_, _, _, _, _, _, _, _, h420, _, _, _) => {
                            h420
                        }
                    })?))
                } else {
                    Ok(f73(v0, v1, v3, v5, f11(Str::lit("expected `,` or `}`"))))
                }
            }
        }
    }
}

pub fn f63(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h421, _, _, _, _, _, _, _, _) => {
            h421
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt6::C0(_, _, h422, _, _, _, _, _, _) => {
            (*h422).clone()
        }
    };
    let v6: u64 = match v3.clone() {
        Adt7::C0(h423, _, _, _, _, _, _, _, _, _, _, _) => {
            h423
        }
    };
    let v7: u64 = match v3.clone() {
        Adt7::C0(_, h424, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    };
    let a425 = v6;
    let a426 = 11u64;
    if nat_eq(a425, a426) {
        let a427 = f75(v0.clone(), v4);
        let a428 = 5u64;
        if nat_eq(a427, a428) {
            Ok(f78(v2, {
                let a429 = v4;
                let a430 = 1u64;
                nat_add(a429, a430)?
            }, v5))
        } else {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
        }
    } else {
        let a431 = v6;
        let a432 = 12u64;
        if nat_eq(a431, a432) {
            let v8: Bytes = match v3 {
                Adt7::C0(_, _, _, _, h433, _, _, _, _, _, _, _) => {
                    h433
                }
            };
            Ok(f78(v2, v4, if f36(v8.clone(), f11(Str::lit("List"))) {
                f91(f11(Str::lit("list")), f94(Str::lit("element"), v5))?
            } else {
                if f36(v8, f11(Str::lit("Set"))) {
                    f91(f11(Str::lit("set")), f94(Str::lit("element"), v5))?
                } else {
                    f91(f11(Str::lit("option")), f94(Str::lit("value"), v5))?
                }
            }))
        } else {
            let a434 = v6;
            let a435 = 13u64;
            if nat_eq(a434, a435) {
                let a436 = v7;
                let a437 = 0u64;
                if nat_eq(a436, a437) {
                    f115(v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f78(v2, v4, if f36(match v3.clone() {
                        Adt7::C0(_, _, _, _, h438, _, _, _, _, _, _, _) => {
                            h438
                        }
                    }, f11(Str::lit("Result"))) {
                        f91(f11(Str::lit("result")), f98(Str::lit("ok"), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h439, _, _) => {
                                (*h439).clone()
                            }
                        }, Str::lit("error"), v5))?
                    } else {
                        f91(f11(Str::lit("map")), f98(Str::lit("key"), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h440, _, _) => {
                                (*h440).clone()
                            }
                        }, Str::lit("value"), v5))?
                    }))
                }
            } else {
                let a441 = v6;
                let a442 = 14u64;
                if nat_eq(a441, a442) {
                    let a443 = v7;
                    let a444 = 0u64;
                    if nat_eq(a443, a444) {
                        f117(v0, v1, v2, f108(v3, v5), v4)
                    } else {
                        Ok(f78(v2, v4, f91(f11(Str::lit("function")), f98(Str::lit("parameters"), f79(match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, h445, _, _, _, _) => {
                                h445
                            }
                        }), Str::lit("result"), v5))?))
                    }
                } else {
                    let a446 = v6;
                    let a447 = 16u64;
                    if nat_eq(a446, a447) {
                        f118(v0, v1, v2, f108(v3, v5), v4)
                    } else {
                        let a448 = v6;
                        let a449 = 17u64;
                        if nat_eq(a448, a449) {
                            let a450 = v7;
                            let a451 = 0u64;
                            if nat_eq(a450, a451) {
                                if f119(v0.clone(), v4, f11(Str::lit("*"))) {
                                    f115(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                        let a452 = v4;
                                        let a453 = 1u64;
                                        nat_add(a452, a453)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `*` between the two types of a product"))))
                                }
                            } else {
                                Ok(f78(v2, v4, f91(f11(Str::lit("product")), f98(Str::lit("left"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h454, _, _) => {
                                        (*h454).clone()
                                    }
                                }, Str::lit("right"), v5))?))
                            }
                        } else {
                            let a455 = v6;
                            let a456 = 18u64;
                            if nat_eq(a455, a456) {
                                let v9: Adt7 = f108(v3, v5);
                                let a457 = f75(v0.clone(), v4);
                                let a458 = 6u64;
                                if nat_eq(a457, a458) {
                                    f106(v0, v1, v2, v9, {
                                        let a459 = v4;
                                        let a460 = 1u64;
                                        nat_add(a459, a460)?
                                    })
                                } else {
                                    let a461 = f75(v0.clone(), v4);
                                    let a462 = 3u64;
                                    if nat_eq(a461, a462) {
                                        Ok(f78(v2, {
                                            let a463 = v4;
                                            let a464 = 1u64;
                                            nat_add(a463, a464)?
                                        }, f79(match v9 {
                                            Adt7::C0(_, _, _, _, _, _, _, h465, _, _, _, _) => {
                                                h465
                                            }
                                        })))
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `,` or `]`"))))
                                    }
                                }
                            } else {
                                Ok(f78(v2, v4, f114(f98(Str::lit("name"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h466, _, _) => {
                                        (*h466).clone()
                                    }
                                }, Str::lit("type"), v5))?))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f64(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h467, _, _, _, _, _, _, _, _) => {
            h467
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt6::C0(_, _, h468, _, _, _, _, _, _) => {
            (*h468).clone()
        }
    };
    let v6: u64 = match v3.clone() {
        Adt7::C0(h469, _, _, _, _, _, _, _, _, _, _, _) => {
            h469
        }
    };
    let v7: u64 = match v3.clone() {
        Adt7::C0(_, h470, _, _, _, _, _, _, _, _, _, _) => {
            h470
        }
    };
    let a471 = v6;
    let a472 = 30u64;
    if nat_eq(a471, a472) {
        let a473 = f75(v0.clone(), v4);
        let a474 = 5u64;
        if nat_eq(a473, a474) {
            Ok(f78(v2, {
                let a475 = v4;
                let a476 = 1u64;
                nat_add(a475, a476)?
            }, v5))
        } else {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
        }
    } else {
        let a477 = v6;
        let a478 = 31u64;
        if nat_eq(a477, a478) {
            Ok(f78(v2, v4, f91(match v3 {
                Adt7::C0(_, _, _, _, h479, _, _, _, _, _, _, _) => {
                    h479
                }
            }, f94(Str::lit("value"), v5))?))
        } else {
            let a480 = v6;
            let a481 = 32u64;
            if nat_eq(a480, a481) {
                let a482 = v7;
                let a483 = 0u64;
                if nat_eq(a482, a483) {
                    f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f78(v2, v4, f91(f11(Str::lit("cons")), f98(Str::lit("head"), match v3 {
                        Adt7::C0(_, _, _, _, _, _, _, _, _, h484, _, _) => {
                            (*h484).clone()
                        }
                    }, Str::lit("tail"), v5))?))
                }
            } else {
                let a485 = v6;
                let a486 = 33u64;
                if nat_eq(a485, a486) {
                    Ok(f78(v2, v4, f91(f11(Str::lit("nil")), f94(Str::lit("element"), v5))?))
                } else {
                    let a487 = v6;
                    let a488 = 34u64;
                    if nat_eq(a487, a488) {
                        let a489 = v7;
                        let a490 = 0u64;
                        if nat_eq(a489, a490) {
                            if f119(v0.clone(), v4, f11(Str::lit("then"))) {
                                f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                    let a491 = v4;
                                    let a492 = 1u64;
                                    nat_add(a491, a492)?
                                })
                            } else {
                                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `then`"))))
                            }
                        } else {
                            let a493 = v7;
                            let a494 = 1u64;
                            if nat_eq(a493, a494) {
                                if f119(v0.clone(), v4, f11(Str::lit("else"))) {
                                    f120(v2, f116(f104(v3.clone(), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h495, _, _) => {
                                            (*h495).clone()
                                        }
                                    }, v5), 2u64), {
                                        let a496 = v4;
                                        let a497 = 1u64;
                                        nat_add(a496, a497)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `else`"))))
                                }
                            } else {
                                Ok(f78(v2, v4, f91(f11(Str::lit("if")), List::cons((f11(Str::lit("else_value")), v5), f98(Str::lit("condition"), match v3.clone() {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h498, _, _) => {
                                        (*h498).clone()
                                    }
                                }, Str::lit("then_value"), match v3 {
                                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h499, _) => {
                                        (*h499).clone()
                                    }
                                })))?))
                            }
                        }
                    } else {
                        let a500 = v6;
                        let a501 = 35u64;
                        if nat_eq(a500, a501) {
                            let a502 = v7;
                            let a503 = 0u64;
                            if nat_eq(a502, a503) {
                                let a504 = f75(v0.clone(), v4);
                                let a505 = 8u64;
                                if nat_eq(a504, a505) {
                                    f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                        let a506 = v4;
                                        let a507 = 1u64;
                                        nat_add(a506, a507)?
                                    })
                                } else {
                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `:=`"))))
                                }
                            } else {
                                let a508 = v7;
                                let a509 = 1u64;
                                if nat_eq(a508, a509) {
                                    let a510 = f75(v0.clone(), v4);
                                    let a511 = 10u64;
                                    if nat_eq(a510, a511) {
                                        f120(v2, f116(f104(v3.clone(), match v3 {
                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h512, _, _) => {
                                                (*h512).clone()
                                            }
                                        }, v5), 2u64), {
                                            let a513 = v4;
                                            let a514 = 1u64;
                                            nat_add(a513, a514)?
                                        })
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `;`"))))
                                    }
                                } else {
                                    Ok(f78(v2, v4, f91(f11(Str::lit("let")), List::cons((f11(Str::lit("body")), v5), f98(Str::lit("binder"), match v3.clone() {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h515, _, _) => {
                                            (*h515).clone()
                                        }
                                    }, Str::lit("value"), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h516, _) => {
                                            (*h516).clone()
                                        }
                                    })))?))
                                }
                            }
                        } else {
                            let a517 = v6;
                            let a518 = 36u64;
                            if nat_eq(a517, a518) {
                                let a519 = v7;
                                let a520 = 0u64;
                                if nat_eq(a519, a520) {
                                    let a521 = f75(v0.clone(), v4);
                                    let a522 = 5u64;
                                    if nat_eq(a521, a522) {
                                        let a525 = f75(v0.clone(), {
                                            let a523 = v4;
                                            let a524 = 1u64;
                                            nat_add(a523, a524)?
                                        });
                                        let a526 = 6u64;
                                        if nat_eq(a525, a526) {
                                            f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                                let a527 = v4;
                                                let a528 = 2u64;
                                                nat_add(a527, a528)?
                                            })
                                        } else {
                                            Ok(f73(v0, v1, v2, {
                                                let a529 = v4;
                                                let a530 = 1u64;
                                                nat_add(a529, a530)?
                                            }, f11(Str::lit("expected `,` before the body"))))
                                        }
                                    } else {
                                        Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
                                    }
                                } else {
                                    Ok(f78(v2, v4, f91(f11(Str::lit("forall")), f98(Str::lit("binder"), match v3 {
                                        Adt7::C0(_, _, _, _, _, _, _, _, _, h531, _, _) => {
                                            (*h531).clone()
                                        }
                                    }, Str::lit("body"), v5))?))
                                }
                            } else {
                                let a532 = v6;
                                let a533 = 37u64;
                                if nat_eq(a532, a533) {
                                    let a534 = v7;
                                    let a535 = 0u64;
                                    if nat_eq(a534, a535) {
                                        let a536 = f75(v0.clone(), v4);
                                        let a537 = 5u64;
                                        if nat_eq(a536, a537) {
                                            f121(v0, v1, v2, f108(v3, v5), {
                                                let a538 = v4;
                                                let a539 = 1u64;
                                                nat_add(a538, a539)?
                                            })
                                        } else {
                                            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `)`"))))
                                        }
                                    } else {
                                        Ok(f78(v2, v4, f91(f11(Str::lit("lambda")), List::cons((f11(Str::lit("body")), v5), f98(Str::lit("parameters"), f79(match v3.clone() {
                                            Adt7::C0(_, _, _, _, _, _, _, h540, _, _, _, _) => {
                                                h540
                                            }
                                        }), Str::lit("captures"), match v3 {
                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h541, _, _) => {
                                                (*h541).clone()
                                            }
                                        })))?))
                                    }
                                } else {
                                    let a542 = v6;
                                    let a543 = 38u64;
                                    if nat_eq(a542, a543) {
                                        let a544 = v7;
                                        let a545 = 0u64;
                                        if nat_eq(a544, a545) {
                                            f122(v0, v1, v2, f116(f104(v3, v5, Adt0::C5), 1u64), v4)
                                        } else {
                                            f122(v0, v1, v2, f108(v3, v5), v4)
                                        }
                                    } else {
                                        let a546 = v6;
                                        let a547 = 39u64;
                                        if nat_eq(a546, a547) {
                                            let v8: Adt12 = f103(v0, v1, v4, f11(Str::lit("a field name")))?;
                                            let m549 = match v8.clone() {
                                                Adt12::C0(_, _, h548) => {
                                                    h548
                                                }
                                            };
                                            match m549 {
                                                None => {
                                                    Ok(f78(v2, match v8.clone() {
                                                        Adt12::C0(_, h551, _) => {
                                                            h551
                                                        }
                                                    }, f91(f11(Str::lit("project")), f98(Str::lit("value"), v5, Str::lit("field"), match v8 {
                                                        Adt12::C0(h552, _, _) => {
                                                            (*h552).clone()
                                                        }
                                                    }))?))
                                                }
                                                Some(_) => {
                                                    Ok(f77(v2, match v8 {
                                                        Adt12::C0(_, _, h550) => {
                                                            h550
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            let a553 = v6;
                                            let a554 = 40u64;
                                            if if nat_eq(a553, a554) {
                                                true
                                            } else {
                                                let a555 = v6;
                                                let a556 = 44u64;
                                                if nat_eq(a555, a556) {
                                                    true
                                                } else {
                                                    let a557 = v6;
                                                    let a558 = 46u64;
                                                    nat_eq(a557, a558)
                                                }
                                            } {
                                                let a559 = v7;
                                                let a560 = 0u64;
                                                if nat_eq(a559, a560) {
                                                    f123(v0, v1, v2, v3)
                                                } else {
                                                    f122(v0, v1, v2, f108(v3, v5), v4)
                                                }
                                            } else {
                                                let a561 = v6;
                                                let a562 = 41u64;
                                                if nat_eq(a561, a562) {
                                                    f123(v0, v1, v2, v3)
                                                } else {
                                                    let a563 = v6;
                                                    let a564 = 42u64;
                                                    if nat_eq(a563, a564) {
                                                        let a565 = v7;
                                                        let a566 = 0u64;
                                                        if nat_eq(a565, a566) {
                                                            f123(v0, v1, v2, v3)
                                                        } else {
                                                            let v10: Adt7 = f108(v3.clone(), f114(f98(Str::lit("field"), f89(match v3 {
                                                                Adt7::C0(_, _, _, _, h567, _, _, _, _, _, _, _) => {
                                                                    h567
                                                                }
                                                            }), Str::lit("value"), v5))?);
                                                            let a568 = f75(v0.clone(), v4);
                                                            let a569 = 6u64;
                                                            if nat_eq(a568, a569) {
                                                                f124(v0, v1, v2, v10, {
                                                                    let a570 = v4;
                                                                    let a571 = 1u64;
                                                                    nat_add(a570, a571)?
                                                                })
                                                            } else {
                                                                let a572 = f75(v0.clone(), v4);
                                                                let a573 = 1u64;
                                                                if nat_eq(a572, a573) {
                                                                    f124(v0, v1, v2, v10, v4)
                                                                } else {
                                                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `,` or `}`"))))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a574 = v6;
                                                        let a575 = 43u64;
                                                        if nat_eq(a574, a575) {
                                                            let a576 = v7;
                                                            let a577 = 0u64;
                                                            if nat_eq(a576, a577) {
                                                                f125(v0, v1, v2, f108(v3, v5), v4)
                                                            } else {
                                                                Ok(f78(v2, v4, f91(f11(Str::lit("primitive")), List::cons((f11(Str::lit("result")), v5), f98(Str::lit("operation"), match v3.clone() {
                                                                    Adt7::C0(_, _, _, _, _, _, _, _, _, h578, _, _) => {
                                                                        (*h578).clone()
                                                                    }
                                                                }, Str::lit("arguments"), f79(match v3 {
                                                                    Adt7::C0(_, _, _, _, _, _, _, h579, _, _, _, _) => {
                                                                        h579
                                                                    }
                                                                }))))?))
                                                            }
                                                        } else {
                                                            let a580 = v6;
                                                            let a581 = 45u64;
                                                            if nat_eq(a580, a581) {
                                                                let a582 = v7;
                                                                let a583 = 0u64;
                                                                if nat_eq(a582, a583) {
                                                                    f126(v0, v1, v2, f116(f104(v3, v5, Adt0::C5), 2u64), v4)
                                                                } else {
                                                                    let a584 = v7;
                                                                    let a585 = 1u64;
                                                                    if nat_eq(a584, a585) {
                                                                        f126(v0, v1, v2, f116(f108(v3.clone(), f114(List::cons((f11(Str::lit("body")), v5), match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, h586, _, _, _) => {
                                                                                h586
                                                                            }
                                                                        }))?), 2u64), v4)
                                                                    } else {
                                                                        f126(v0, v1, v2, v3, v4)
                                                                    }
                                                                }
                                                            } else {
                                                                let a587 = v7;
                                                                let a588 = 0u64;
                                                                if nat_eq(a587, a588) {
                                                                    let a589 = f75(v0.clone(), v4);
                                                                    let a590 = 6u64;
                                                                    if nat_eq(a589, a590) {
                                                                        f120(v2, f116(f104(v3, v5, Adt0::C5), 1u64), {
                                                                            let a591 = v4;
                                                                            let a592 = 1u64;
                                                                            nat_add(a591, a592)?
                                                                        })
                                                                    } else {
                                                                        let m593 = f127(f84(v0.clone(), v4));
                                                                        match m593 {
                                                                            None => {
                                                                                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected an operator or `,` after the first operand"))))
                                                                            }
                                                                            Some(v11) => {
                                                                                let a594 = f75(v0.clone(), v4);
                                                                                let a595 = 12u64;
                                                                                if nat_eq(a594, a595) {
                                                                                    f120(v2, f110(f116(f104(v3, v5, Adt0::C5), 2u64), v11, 0u64), {
                                                                                        let a596 = v4;
                                                                                        let a597 = 1u64;
                                                                                        nat_add(a596, a597)?
                                                                                    })
                                                                                } else {
                                                                                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected an operator or `,` after the first operand"))))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a598 = v7;
                                                                    let a599 = 1u64;
                                                                    if nat_eq(a598, a599) {
                                                                        Ok(f78(v2, v4, f91(f11(Str::lit("pair")), f98(Str::lit("left"), match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h600, _, _) => {
                                                                                (*h600).clone()
                                                                            }
                                                                        }, Str::lit("right"), v5))?))
                                                                    } else {
                                                                        let v12: Bytes = match v3.clone() {
                                                                            Adt7::C0(_, _, _, _, h601, _, _, _, _, _, _, _) => {
                                                                                h601
                                                                            }
                                                                        };
                                                                        let v13: bool = f36(v12.clone(), f11(Str::lit("implies")));
                                                                        Ok(f78(v2, v4, f91(v12, List::cons((if v13 {
                                                                            f11(Str::lit("premise"))
                                                                        } else {
                                                                            f11(Str::lit("left"))
                                                                        }, match v3 {
                                                                            Adt7::C0(_, _, _, _, _, _, _, _, _, h602, _, _) => {
                                                                                (*h602).clone()
                                                                            }
                                                                        }), List::cons((if v13 {
                                                                            f11(Str::lit("conclusion"))
                                                                        } else {
                                                                            f11(Str::lit("right"))
                                                                        }, v5), List::<(Bytes, Adt0)>::nil())))?))
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

pub fn f65(v0: u8, v1: u8, v2: u8) -> bool {
    let m605 = {
        let a603 = v0;
        let a604 = v1;
        fixed_u8::checked_sub(a603, a604)
    };
    match m605 {
        None => {
            false
        }
        Some(_) => {
            let m608 = {
                let a606 = v2;
                let a607 = v0;
                fixed_u8::checked_sub(a606, a607)
            };
            match m608 {
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

pub fn f66(v0: Fn6, p613: u64, p614: Adt9) -> R<(Adt9, bool)> {
    let m612 = (p613, p614);
    {
        let mut m612 = m612;
        loop {
            let step = {
                let (v1, v2) = m612;
                let m609 = v1;
                if m609 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m609 - 1;
                    let m611 = {
                        let c610: Fn6 = v0.clone();
                        c610.apply(v2.clone())?
                    };
                    match m611 {
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
                Ok(next) => { m612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: u64, v1: Bytes, v2: Adt9) -> R<Option<Adt9>> {
    f128(v1, v0, v2)
}

pub fn f68(v0: List<Bytes>) -> Bytes {
    let m617 = {
        let (h615, _) = f129(Fn8::F130, {
            let a616 = v0.clone();
            length_list(a616)
        }, v0);
        h615
    };
    match m617.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f69(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn9::F132, List::<Bytes>::nil(), v0)
}

pub fn f70(v0: Fn7, p622: u64, p623: u64) -> R<(u64, bool)> {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m618 = v1;
                if m618 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m618 - 1;
                    let m620 = {
                        let c619: Fn7 = v0.clone();
                        c619.apply(v2)?
                    };
                    match m620 {
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
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f133(v1, v0, v2)
}

pub fn f72(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    let v2: u64 = match v1.clone() {
        Adt10::C0(h624, _) => {
            h624
        }
    };
    let m625 = f41(v0, v2);
    match m625 {
        None => {
            Ok(None::<Adt10>)
        }
        Some(v3) => {
            Ok(Some(Adt10::C0({
                let a626 = v2;
                let a627 = 1u64;
                nat_add(a626, a627)?
            }, {
                let a674 = {
                    let v4: u8 = v3;
                    let a672 = {
                        let a648 = {
                            let a636 = {
                                let a630 = {
                                    let a628 = v4;
                                    let a629 = 128u8;
                                    fixed_u8::bit_and(a628, a629)
                                };
                                let a631 = 128u8;
                                if equal(a630, a631) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a637 = {
                                let a634 = {
                                    let a632 = v4;
                                    let a633 = 64u8;
                                    fixed_u8::bit_and(a632, a633)
                                };
                                let a635 = 64u8;
                                if equal(a634, a635) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a636, a637)?
                        };
                        let a649 = {
                            let a646 = {
                                let a640 = {
                                    let a638 = v4;
                                    let a639 = 32u8;
                                    fixed_u8::bit_and(a638, a639)
                                };
                                let a641 = 32u8;
                                if equal(a640, a641) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a647 = {
                                let a644 = {
                                    let a642 = v4;
                                    let a643 = 16u8;
                                    fixed_u8::bit_and(a642, a643)
                                };
                                let a645 = 16u8;
                                if equal(a644, a645) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a646, a647)?
                        };
                        nat_add(a648, a649)?
                    };
                    let a673 = {
                        let a670 = {
                            let a658 = {
                                let a652 = {
                                    let a650 = v4;
                                    let a651 = 8u8;
                                    fixed_u8::bit_and(a650, a651)
                                };
                                let a653 = 8u8;
                                if equal(a652, a653) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a659 = {
                                let a656 = {
                                    let a654 = v4;
                                    let a655 = 4u8;
                                    fixed_u8::bit_and(a654, a655)
                                };
                                let a657 = 4u8;
                                if equal(a656, a657) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a658, a659)?
                        };
                        let a671 = {
                            let a668 = {
                                let a662 = {
                                    let a660 = v4;
                                    let a661 = 2u8;
                                    fixed_u8::bit_and(a660, a661)
                                };
                                let a663 = 2u8;
                                if equal(a662, a663) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a669 = {
                                let a666 = {
                                    let a664 = v4;
                                    let a665 = 1u8;
                                    fixed_u8::bit_and(a664, a665)
                                };
                                let a667 = 1u8;
                                if equal(a666, a667) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a668, a669)?
                        };
                        nat_add(a670, a671)?
                    };
                    nat_add(a672, a673)?
                };
                let a675 = 47u64;
                if nat_eq(a674, a675) {
                    let a676 = v2;
                    let a677 = 1u64;
                    nat_add(a676, a677)?
                } else {
                    match v1 {
                        Adt10::C0(_, h678) => {
                            h678
                        }
                    }
                }
            })))
        }
    }
}

pub fn f73(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> Adt6 {
    f113(v2, f111(v0, v1, v3), v4)
}

pub fn f74(v0: Adt6, v1: u64, v2: u64, v3: Adt2, v4: u64) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h679, _, _, _, _, _, _, _) => {
            h679
        }
    }, std::rc::Rc::new(Adt0::C5), 0u64, v2, std::rc::Rc::new(v3), v4, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h680) => {
            h680
        }
    })
}

pub fn f75(v0: List<Adt3>, v1: u64) -> u64 {
    f134(v0, v1)
}

pub fn f76(v0: List<Adt3>, v1: u64, v2: u64) -> R<Adt12> {
    let v3: Adt12 = f103(v0.clone(), v1, v2, f11(Str::lit("a name")))?;
    let m682 = match v3.clone() {
        Adt12::C0(_, _, h681) => {
            h681
        }
    };
    match m682 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt12::C0(_, h683, _) => {
                    h683
                }
            };
            let a684 = f75(v0.clone(), v5);
            let a685 = 9u64;
            if nat_eq(a684, a685) {
                let v6: Adt12 = f103(v0, v1, {
                    let a686 = v5;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                }, f11(Str::lit("a name after `::`")))?;
                let m689 = match v6.clone() {
                    Adt12::C0(_, _, h688) => {
                        h688
                    }
                };
                match m689 {
                    None => {
                        Ok(Adt12::C0(std::rc::Rc::new(f114(f98(Str::lit("module"), match v3 {
                            Adt12::C0(h690, _, _) => {
                                (*h690).clone()
                            }
                        }, Str::lit("name"), match v6.clone() {
                            Adt12::C0(h691, _, _) => {
                                (*h691).clone()
                            }
                        }))?), match v6 {
                            Adt12::C0(_, h692, _) => {
                                h692
                            }
                        }, None::<Adt1>))
                    }
                    Some(_) => {
                        Ok(v6)
                    }
                }
            } else {
                Ok(Adt12::C0(std::rc::Rc::new(f114(f94(Str::lit("name"), match v3 {
                    Adt12::C0(h693, _, _) => {
                        (*h693).clone()
                    }
                }))?), v5, None::<Adt1>))
            }
        }
        Some(_) => {
            Ok(v3)
        }
    }
}

pub fn f77(v0: Adt6, v1: Option<Adt1>) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h694, _, _, _, _, _, _, _, _) => {
            h694
        }
    }, match v0.clone() {
        Adt6::C0(_, h695, _, _, _, _, _, _, _) => {
            h695
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, h696, _, _, _, _, _, _) => {
            (*h696).clone()
        }
    }), 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h697, _, _, _, _) => {
            h697
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h698, _, _, _) => {
            (*h698).clone()
        }
    }), match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h699, _, _) => {
            h699
        }
    }, v1, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h700) => {
            h700
        }
    })
}

pub fn f78(v0: Adt6, v1: u64, v2: Adt0) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h701, _, _, _, _, _, _, _) => {
            h701
        }
    }, std::rc::Rc::new(v2), 1u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h702, _, _, _, _) => {
            h702
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h703, _, _, _) => {
            (*h703).clone()
        }
    }), match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h704, _, _) => {
            h704
        }
    }, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h705) => {
            h705
        }
    })
}

pub fn f79(v0: List<Adt0>) -> Adt0 {
    Adt0::C3(std::rc::Rc::new(f135(v0)))
}

pub fn f80(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m706 = v1;
    match m706 {
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
        Adt2::C4(r707) => {
            let v2 = (*r707).clone();
            f12(v0, v2)
        }
        Adt2::C5 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f81(v0: Adt6, v1: Adt7, v2: u64, v3: u64, v4: Adt2, v5: u64) -> Adt6 {
    Adt6::C0(v2, List::cons(v1, match v0.clone() {
        Adt6::C0(_, h708, _, _, _, _, _, _, _) => {
            h708
        }
    }), std::rc::Rc::new(Adt0::C5), 0u64, v3, std::rc::Rc::new(v4), v5, None::<Adt1>, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h709) => {
            h709
        }
    })
}

pub fn f82(v0: List<Adt3>, v1: u64, v2: Adt0, v3: Adt6, v4: Adt7, v5: u64) -> R<Adt6> {
    let v6: u64 = f75(v0.clone(), v5);
    let a710 = v6;
    let a711 = 1u64;
    if nat_eq(a710, a711) {
        Ok(f78(v3, {
            let a712 = v5;
            let a713 = 1u64;
            nat_add(a712, a713)?
        }, f114(match v4 {
            Adt7::C0(_, _, _, _, _, _, _, _, h714, _, _, _) => {
                h714
            }
        })?))
    } else {
        let v7: u64 = f111(v0.clone(), v1, v5);
        let v8: Bytes = f84(v0.clone(), v5);
        let a715 = v6;
        let a716 = 11u64;
        if if nat_eq(a715, a716) {
            true
        } else {
            f85({
                let a717 = v6;
                let a718 = 12u64;
                nat_eq(a717, a718)
            }, f86(v8.clone())?)
        } {
            let a721 = f75(v0.clone(), {
                let a719 = v5;
                let a720 = 1u64;
                nat_add(a719, a720)?
            });
            let a722 = 7u64;
            if nat_eq(a721, a722) {
                Ok(f81(v3, f110(v4.clone(), v8.clone(), v7), {
                    let a723 = v5;
                    let a724 = 2u64;
                    nat_add(a723, a724)?
                }, 0u64, f136(v2, match v4.clone() {
                    Adt7::C0(_, _, _, _, _, _, h725, _, _, _, _, _) => {
                        h725
                    }
                }, v8)?, {
                    let a727 = match v4 {
                        Adt7::C0(_, _, h726, _, _, _, _, _, _, _, _, _) => {
                            h726
                        }
                    };
                    let a728 = 1u64;
                    nat_add(a727, a728)?
                }))
            } else {
                Ok(f73(v0, v1, v3, {
                    let a729 = v5;
                    let a730 = 1u64;
                    nat_add(a729, a730)?
                }, f11(Str::lit("expected `:` after a member name"))))
            }
        } else {
            Ok(f113(v3, v7, f11(Str::lit("expected a member name"))))
        }
    }
}

pub fn f83(v0: Adt0, v1: Adt2) -> Option<Adt0> {
    let m731 = v1;
    match m731 {
        Adt2::C0 => {
            None::<Adt0>
        }
        Adt2::C1 => {
            None::<Adt0>
        }
        Adt2::C2 => {
            None::<Adt0>
        }
        Adt2::C3 => {
            f13(v0, f11(Str::lit("parameter")))
        }
        Adt2::C4(r732) => {
            let v2 = (*r732).clone();
            Some(v2)
        }
        Adt2::C5 => {
            None::<Adt0>
        }
    }
}

pub fn f84(v0: List<Adt3>, v1: u64) -> Bytes {
    f137(v0, v1)
}

pub fn f85(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f86(v0: Bytes) -> R<bool> {
    let a734 = {
        let a733 = v0.clone();
        length_bytes(a733)
    };
    let a735 = 0u64;
    if nat_eq(a734, a735) {
        Ok(false)
    } else {
        if f138(f139(v0.clone(), 0u64)?) {
            let a737 = f140(v0.clone(), 1u64)?;
            let a738 = {
                let a736 = v0.clone();
                length_bytes(a736)
            };
            if nat_eq(a737, a738) {
                let a739 = f141(v0);
                Ok(bool_not(a739))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f87(v0: Bytes) -> R<bool> {
    Ok(f85({
        let a741 = 0u64;
        let a742 = {
            let a740 = v0.clone();
            length_bytes(a740)
        };
        nat_lt(a741, a742)
    }, {
        let a744 = f142(v0.clone(), 0u64)?;
        let a745 = {
            let a743 = v0;
            length_bytes(a743)
        };
        nat_eq(a744, a745)
    }))
}

pub fn f88(v0: Adt0, v1: Adt2, v2: Bytes) -> R<Option<Adt0>> {
    let m746 = v1;
    match m746 {
        Adt2::C0 => {
            let m747 = f13(v0.clone(), f11(Str::lit("term")));
            match m747 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v3) => {
                    f143(v0, v3, v2)
                }
            }
        }
        Adt2::C1 => {
            let m748 = f13(v0.clone(), f11(Str::lit("type")));
            match m748 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    f143(v0, v4, v2)
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(r749) => {
            let v5 = (*r749).clone();
            if f144(v0.clone(), v5.clone())? {
                f143(v0, v5, v2)
            } else {
                Ok(Some(v5))
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f89(v0: Bytes) -> Adt0 {
    Adt0::C2(v0)
}

pub fn f90(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("Bool"))) {
        Some(f11(Str::lit("bool")))
    } else {
        if f36(v0.clone(), f11(Str::lit("Bytes"))) {
            Some(f11(Str::lit("bytes")))
        } else {
            if f36(v0.clone(), f11(Str::lit("Int"))) {
                Some(f11(Str::lit("int")))
            } else {
                if f36(v0.clone(), f11(Str::lit("Int8"))) {
                    Some(f11(Str::lit("int8")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("Int16"))) {
                        Some(f11(Str::lit("int16")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("Int32"))) {
                            Some(f11(Str::lit("int32")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("Int64"))) {
                                Some(f11(Str::lit("int64")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("Nat"))) {
                                    Some(f11(Str::lit("nat")))
                                } else {
                                    if f36(v0.clone(), f11(Str::lit("Ordering"))) {
                                        Some(f11(Str::lit("ordering")))
                                    } else {
                                        if f36(v0.clone(), f11(Str::lit("Prop"))) {
                                            Some(f11(Str::lit("prop")))
                                        } else {
                                            if f36(v0.clone(), f11(Str::lit("String"))) {
                                                Some(f11(Str::lit("string")))
                                            } else {
                                                if f36(v0.clone(), f11(Str::lit("Type"))) {
                                                    Some(f11(Str::lit("type")))
                                                } else {
                                                    if f36(v0.clone(), f11(Str::lit("UInt8"))) {
                                                        Some(f11(Str::lit("uint8")))
                                                    } else {
                                                        if f36(v0.clone(), f11(Str::lit("UInt16"))) {
                                                            Some(f11(Str::lit("uint16")))
                                                        } else {
                                                            if f36(v0.clone(), f11(Str::lit("UInt32"))) {
                                                                Some(f11(Str::lit("uint32")))
                                                            } else {
                                                                if f36(v0.clone(), f11(Str::lit("UInt64"))) {
                                                                    Some(f11(Str::lit("uint64")))
                                                                } else {
                                                                    if f36(v0.clone(), f11(Str::lit("Unit"))) {
                                                                        Some(f11(Str::lit("unit")))
                                                                    } else {
                                                                        if f36(v0.clone(), f11(Str::lit("ContractViolation"))) {
                                                                            Some(f11(Str::lit("contract_violation")))
                                                                        } else {
                                                                            if f36(v0, f11(Str::lit("ReasoningFailure"))) {
                                                                                Some(f11(Str::lit("reasoning_failure")))
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

pub fn f91(v0: Bytes, v1: List<(Bytes, Adt0)>) -> R<Adt0> {
    f114(List::cons((f11(Str::lit("kind")), Adt0::C2(v0)), v1))
}

pub fn f92(v0: Bytes, v1: Bytes) -> bool {
    let m754 = {
        let a751 = v0;
        let a752 = 0u64;
        let a753 = {
            let a750 = v1.clone();
            length_bytes(a750)
        };
        slice_bytes(a751, a752, a753)
    };
    match m754 {
        None => {
            false
        }
        Some(v2) => {
            let a755 = v2;
            let a756 = v1;
            equal(a755, a756)
        }
    }
}

pub fn f93(v0: Bytes, v1: u64) -> Bytes {
    f47(v0.clone(), v1, {
        let a758 = {
            let a757 = v0;
            length_bytes(a757)
        };
        let a759 = v1;
        nat_sub(a758, a759)
    })
}

pub fn f94(v0: Str, v1: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f11(v0), v1), List::<(Bytes, Adt0)>::nil())
}

pub fn f95(v0: Bytes, v1: List<Bytes>) -> bool {
    f145(Fn10::F146(v0), false, v1)
}

pub fn f96() -> List<Bytes> {
    List::cons(f11(Str::lit("List")), List::cons(f11(Str::lit("Option")), List::cons(f11(Str::lit("Result")), List::cons(f11(Str::lit("Set")), List::cons(f11(Str::lit("Map")), List::cons(f11(Str::lit("Fn")), List::<Bytes>::nil()))))))
}

pub fn f97(v0: Bytes, v1: Bytes) -> Bytes {
    let a760 = v0;
    let a761 = v1;
    append_bytes(a760, a761)
}

pub fn f98(v0: Str, v1: Adt0, v2: Str, v3: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f11(v0), v1), List::cons((f11(v2), v3), List::<(Bytes, Adt0)>::nil()))
}

pub fn f99(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: Bytes = f84(v0.clone(), v3);
    let v6: bool = {
        let a762 = f75(v0.clone(), v3);
        let a763 = 12u64;
        nat_eq(a762, a763)
    };
    if f85(v6, if f36(v5.clone(), f11(Str::lit("List"))) {
        true
    } else {
        if f36(v5.clone(), f11(Str::lit("Set"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("Option")))
        }
    }) {
        Ok(f81(v2, f110(f6(12u64, v4), v5, 0u64), {
            let a764 = v3;
            let a765 = 1u64;
            nat_add(a764, a765)?
        }, 2u64, Adt2::C1, {
            let a766 = v4;
            let a767 = 1u64;
            nat_add(a766, a767)?
        }))
    } else {
        if f85(v6, if f36(v5.clone(), f11(Str::lit("Result"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("Map")))
        }) {
            Ok(f81(v2, f110(f6(13u64, v4), v5, 0u64), {
                let a768 = v3;
                let a769 = 1u64;
                nat_add(a768, a769)?
            }, 2u64, Adt2::C1, {
                let a770 = v4;
                let a771 = 1u64;
                nat_add(a770, a771)?
            }))
        } else {
            if f85(v6, f36(v5.clone(), f11(Str::lit("Fn")))) {
                let a774 = f75(v0.clone(), {
                    let a772 = v3;
                    let a773 = 1u64;
                    nat_add(a772, a773)?
                });
                let a775 = 4u64;
                if nat_eq(a774, a775) {
                    f117(v0, v1, v2, f6(14u64, v4), {
                        let a776 = v3;
                        let a777 = 2u64;
                        nat_add(a776, a777)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, {
                        let a778 = v3;
                        let a779 = 1u64;
                        nat_add(a778, a779)?
                    }, f11(Str::lit("expected `(` before the parameter types"))))
                }
            } else {
                let v7: u64 = {
                    let a782 = f75(v0.clone(), {
                        let a780 = v3;
                        let a781 = 1u64;
                        nat_add(a780, a781)?
                    });
                    let a783 = 9u64;
                    if nat_eq(a782, a783) {
                        3u64
                    } else {
                        1u64
                    }
                };
                let v8: bool = f119(v0.clone(), {
                    let a784 = v3;
                    let a785 = v7;
                    nat_add(a784, a785)?
                }, f11(Str::lit("*")));
                if f85(v6, f85(!v8, f85(!f92(v5.clone(), f11(Str::lit("'"))), !f147(v5)))) {
                    let v9: Adt12 = f76(v0.clone(), v1, v3)?;
                    let m787 = match v9.clone() {
                        Adt12::C0(_, _, h786) => {
                            h786
                        }
                    };
                    match m787 {
                        None => {
                            f118(v0, v1, v2, f104(f6(16u64, v4), match v9.clone() {
                                Adt12::C0(h789, _, _) => {
                                    (*h789).clone()
                                }
                            }, Adt0::C5), match v9 {
                                Adt12::C0(_, h790, _) => {
                                    h790
                                }
                            })
                        }
                        Some(_) => {
                            Ok(f77(v2, match v9 {
                                Adt12::C0(_, _, h788) => {
                                    h788
                                }
                            }))
                        }
                    }
                } else {
                    Ok(f81(v2, f6(17u64, v4), v3, 2u64, Adt2::C1, {
                        let a791 = v4;
                        let a792 = 1u64;
                        nat_add(a791, a792)?
                    }))
                }
            }
        }
    }
}

pub fn f100(v0: List<Adt3>, v1: u64) -> R<Bytes> {
    let m793 = f148(v0, v1);
    match m793 {
        None => {
            Ok(f150(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f149(match v2 {
                Adt3::C0(h794, _, _) => {
                    (*h794).clone()
                }
            })
        }
    }
}

pub fn f101(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: u64 = f75(v0.clone(), v3);
    let v6: Bytes = f84(v0.clone(), v3);
    let a795 = v5;
    let a796 = 5u64;
    if nat_eq(a795, a796) {
        Ok(f78(v2, v3, f91(f11(Str::lit("unit")), List::<(Bytes, Adt0)>::nil())?))
    } else {
        let a797 = v5;
        let a798 = 12u64;
        if nat_eq(a797, a798) {
            let v7: Bytes = if f92(v6.clone(), f11(Str::lit("-"))) {
                f93(v6.clone(), 1u64)
            } else {
                v6.clone()
            };
            if f85(f87(v7)?, {
                let a801 = f75(v0.clone(), {
                    let a799 = v3;
                    let a800 = 1u64;
                    nat_add(a799, a800)?
                });
                let a802 = 7u64;
                nat_eq(a801, a802)
            }) {
                f151(v0, v1, v2, v3, v6)
            } else {
                let v8: bool = f85(f36(v6.clone(), f11(Str::lit("let"))), f119(v0.clone(), {
                    let a803 = v3;
                    let a804 = 2u64;
                    nat_add(a803, a804)?
                }, f11(Str::lit("name"))));
                let v9: Bytes = if f85({
                    let a807 = f75(v0.clone(), {
                        let a805 = v3;
                        let a806 = 1u64;
                        nat_add(a805, a806)?
                    });
                    let a808 = 0u64;
                    nat_eq(a807, a808)
                }, !v8) {
                    f11(Str::lit(""))
                } else {
                    v6
                };
                f152(v0, v1, v2, v3, v4, v9)
            }
        } else {
            f120(v2, f6(47u64, v4), v3)
        }
    }
}

pub fn f102(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> R<Adt6> {
    let v5: u64 = f111(v0, v1, v3);
    if f36(v4.clone(), f11(Str::lit("true"))) {
        Ok(f78(v2, {
            let a809 = v3;
            let a810 = 1u64;
            nat_add(a809, a810)?
        }, f91(f11(Str::lit("bool")), f94(Str::lit("value"), Adt0::C0(true)))?))
    } else {
        if f36(v4.clone(), f11(Str::lit("false"))) {
            Ok(f78(v2, {
                let a811 = v3;
                let a812 = 1u64;
                nat_add(a811, a812)?
            }, f91(f11(Str::lit("bool")), f94(Str::lit("value"), Adt0::C0(false)))?))
        } else {
            if f87(v4.clone())? {
                if f85({
                    let a814 = 1u64;
                    let a815 = {
                        let a813 = v4.clone();
                        length_bytes(a813)
                    };
                    nat_lt(a814, a815)
                }, f92(v4.clone(), f11(Str::lit("0")))) {
                    Ok(f113(v2, v5, f97(f153(v4), f11(Str::lit(" has a leading zero")))))
                } else {
                    Ok(f78(v2, {
                        let a816 = v3;
                        let a817 = 1u64;
                        nat_add(a816, a817)?
                    }, f91(f11(Str::lit("nat")), f94(Str::lit("value"), f89(v4)))?))
                }
            } else {
                if f92(v4.clone(), f11(Str::lit("0x"))) {
                    if f154(f93(v4.clone(), 2u64))? {
                        Ok(f78(v2, {
                            let a818 = v3;
                            let a819 = 1u64;
                            nat_add(a818, a819)?
                        }, f91(f11(Str::lit("bytes")), f94(Str::lit("hex"), f89(f93(v4, 2u64))))?))
                    } else {
                        Ok(f113(v2, v5, f11(Str::lit("a byte string is `0x` and an even number of lower-case hexadecimal digits"))))
                    }
                } else {
                    if f155(v4.clone())? {
                        Ok(f78(v2, {
                            let a820 = v3;
                            let a821 = 1u64;
                            nat_add(a820, a821)?
                        }, f91(f11(Str::lit("var")), f94(Str::lit("name"), f89(v4)))?))
                    } else {
                        Ok(f113(v2, v5, if f95(v4.clone(), f156()) {
                            f97(f97(f97(f153(v4.clone()), f11(Str::lit(" is a keyword; write `var{name: "))), v4), f11(Str::lit("}` for a variable of that name")))
                        } else {
                            f97(f153(v4), f11(Str::lit(" is not a term")))
                        }))
                    }
                }
            }
        }
    }
}

pub fn f103(v0: List<Adt3>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let v4: Bytes = f84(v0.clone(), v2);
    if f85({
        let a822 = f75(v0.clone(), v2);
        let a823 = 12u64;
        nat_eq(a822, a823)
    }, f86(v4.clone())?) {
        Ok(Adt12::C0(std::rc::Rc::new(f89(v4)), {
            let a824 = v2;
            let a825 = 1u64;
            nat_add(a824, a825)?
        }, None::<Adt1>))
    } else {
        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), v2, f157(f111(v0.clone(), v1, v2), f97(f97(f97(f11(Str::lit("expected ")), v3), f11(Str::lit(", found "))), f100(v0, v2)?))))
    }
}

pub fn f104(v0: Adt7, v1: Adt0, v2: Adt0) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h826, _, _, _, _, _, _, _, _, _, _, _) => {
            h826
        }
    }, match v0.clone() {
        Adt7::C0(_, h827, _, _, _, _, _, _, _, _, _, _) => {
            h827
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h828, _, _, _, _, _, _, _, _, _) => {
            h828
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h829, _, _, _, _, _, _, _, _) => {
            h829
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h830, _, _, _, _, _, _, _) => {
            h830
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h831, _, _, _, _, _, _) => {
            (*h831).clone()
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h832, _, _, _, _, _) => {
            h832
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h833, _, _, _, _) => {
            h833
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h834, _, _, _) => {
            h834
        }
    }, std::rc::Rc::new(v1), std::rc::Rc::new(v2), match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h835) => {
            h835
        }
    })
}

pub fn f105(v0: List<Adt3>, v1: u64, v2: u64, v3: u64) -> R<Adt13> {
    let (h836, _) = f158(Fn11::F159(v1, v0.clone()), {
        let a840 = {
            let a838 = {
                let a837 = v0;
                length_list(a837)
            };
            let a839 = 4u64;
            nat_mul(a838, a839)?
        };
        let a841 = 8u64;
        nat_add(a840, a841)?
    }, Adt13::C0(v2, v3, 0u64, List::<Adt14>::nil(), std::rc::Rc::new(Adt0::C5), None::<Adt1>))?;
    Ok(h836)
}

pub fn f106(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a842 = f75(v0, v4);
    let a843 = 3u64;
    if nat_eq(a842, a843) {
        Ok(f78(v2, {
            let a844 = v4;
            let a845 = 1u64;
            nat_add(a844, a845)?
        }, f79(match v3 {
            Adt7::C0(_, _, _, _, _, _, _, h846, _, _, _, _) => {
                h846
            }
        })))
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f107(v0: Adt7, v1: (u64, u64), v2: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h847, _, _, _, _, _, _, _, _, _, _, _) => {
            h847
        }
    }, match v0.clone() {
        Adt7::C0(_, h848, _, _, _, _, _, _, _, _, _, _) => {
            h848
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h849, _, _, _, _, _, _, _, _, _) => {
            h849
        }
    }, v2, match v0.clone() {
        Adt7::C0(_, _, _, _, h850, _, _, _, _, _, _, _) => {
            h850
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h851, _, _, _, _, _, _) => {
            (*h851).clone()
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h852, _, _, _, _, _) => {
            h852
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h853, _, _, _, _) => {
            h853
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h854, _, _, _) => {
            h854
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h855, _, _) => {
            (*h855).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h856, _) => {
            (*h856).clone()
        }
    }), List::cons(v1, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h857) => {
            h857
        }
    }))
}

pub fn f108(v0: Adt7, v1: Adt0) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h858, _, _, _, _, _, _, _, _, _, _, _) => {
            h858
        }
    }, match v0.clone() {
        Adt7::C0(_, h859, _, _, _, _, _, _, _, _, _, _) => {
            h859
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h860, _, _, _, _, _, _, _, _, _) => {
            h860
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h861, _, _, _, _, _, _, _, _) => {
            h861
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h862, _, _, _, _, _, _, _) => {
            h862
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h863, _, _, _, _, _, _) => {
            (*h863).clone()
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h864, _, _, _, _, _) => {
            h864
        }
    }, List::cons(v1, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h865, _, _, _, _) => {
            h865
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h866, _, _, _) => {
            h866
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h867, _, _) => {
            (*h867).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h868, _) => {
            (*h868).clone()
        }
    }), match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h869) => {
            h869
        }
    })
}

pub fn f109(v0: List<Adt3>, v1: u64) -> u64 {
    let a870 = v1;
    let a871 = 0u64;
    if nat_eq(a870, a871) {
        0u64
    } else {
        let m874 = f148(v0, {
            let a872 = v1;
            let a873 = 1u64;
            nat_sub(a872, a873)
        });
        match m874 {
            None => {
                0u64
            }
            Some(v2) => {
                match v2 {
                    Adt3::C0(_, _, h875) => {
                        h875
                    }
                }
            }
        }
    }
}

pub fn f110(v0: Adt7, v1: Bytes, v2: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h876, _, _, _, _, _, _, _, _, _, _, _) => {
            h876
        }
    }, match v0.clone() {
        Adt7::C0(_, h877, _, _, _, _, _, _, _, _, _, _) => {
            h877
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h878, _, _, _, _, _, _, _, _, _) => {
            h878
        }
    }, v2, v1, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h879, _, _, _, _, _, _) => {
            (*h879).clone()
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h880, _, _, _, _, _) => {
            h880
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h881, _, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h882, _, _, _) => {
            h882
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h883, _, _) => {
            (*h883).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h884, _) => {
            (*h884).clone()
        }
    }), match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h885) => {
            h885
        }
    })
}

pub fn f111(v0: List<Adt3>, v1: u64, v2: u64) -> u64 {
    let m886 = f148(v0, v2);
    match m886 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt3::C0(_, h887, _) => {
                    h887
                }
            }
        }
    }
}

pub fn f112(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f160(Fn12::F161(v1), false, v0)
}

pub fn f113(v0: Adt6, v1: u64, v2: Bytes) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h888, _, _, _, _, _, _, _, _) => {
            h888
        }
    }, match v0.clone() {
        Adt6::C0(_, h889, _, _, _, _, _, _, _) => {
            h889
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, h890, _, _, _, _, _, _) => {
            (*h890).clone()
        }
    }), 9u64, match v0.clone() {
        Adt6::C0(_, _, _, _, h891, _, _, _, _) => {
            h891
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h892, _, _, _) => {
            (*h892).clone()
        }
    }), match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h893, _, _) => {
            h893
        }
    }, f157(v1, v2), match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, h894) => {
            h894
        }
    })
}

pub fn f114(v0: List<(Bytes, Adt0)>) -> R<Adt0> {
    Ok(Adt0::C4(std::rc::Rc::new(f162(f163(v0)?))))
}

pub fn f115(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 2u64, Adt2::C1, {
        let a896 = match v1 {
            Adt7::C0(_, _, h895, _, _, _, _, _, _, _, _, _) => {
                h895
            }
        };
        let a897 = 1u64;
        nat_add(a896, a897)?
    }))
}

pub fn f116(v0: Adt7, v1: u64) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h898, _, _, _, _, _, _, _, _, _, _, _) => {
            h898
        }
    }, v1, match v0.clone() {
        Adt7::C0(_, _, h899, _, _, _, _, _, _, _, _, _) => {
            h899
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h900, _, _, _, _, _, _, _, _) => {
            h900
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h901, _, _, _, _, _, _, _) => {
            h901
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h902, _, _, _, _, _, _) => {
            (*h902).clone()
        }
    }), match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h903, _, _, _, _, _) => {
            h903
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h904, _, _, _, _) => {
            h904
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h905, _, _, _) => {
            h905
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h906, _, _) => {
            (*h906).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h907, _) => {
            (*h907).clone()
        }
    }), match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h908) => {
            h908
        }
    })
}

pub fn f117(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a909 = f75(v0, v4);
    let a910 = 5u64;
    if nat_eq(a909, a910) {
        f115(v2, f116(v3, 1u64), {
            let a911 = v4;
            let a912 = 1u64;
            nat_add(a911, a912)?
        })
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f118(v0: List<Adt3>, _: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a913 = f75(v0, v4);
    let a914 = 5u64;
    if nat_eq(a913, a914) {
        Ok(f78(v2, v4, f91(f11(Str::lit("named")), f98(Str::lit("member"), match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h915, _, _) => {
                (*h915).clone()
            }
        }, Str::lit("arguments"), f79(match v3 {
            Adt7::C0(_, _, _, _, _, _, _, h916, _, _, _, _) => {
                h916
            }
        })))?))
    } else {
        f115(v2, v3, v4)
    }
}

pub fn f119(v0: List<Adt3>, v1: u64, v2: Bytes) -> bool {
    f85({
        let a917 = f75(v0.clone(), v1);
        let a918 = 12u64;
        nat_eq(a917, a918)
    }, f36(f84(v0, v1), v2))
}

pub fn f120(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 3u64, Adt2::C0, {
        let a920 = match v1 {
            Adt7::C0(_, _, h919, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 1u64;
        nat_add(a920, a921)?
    }))
}

pub fn f121(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a922 = f75(v0.clone(), v4);
    let a923 = 4u64;
    if nat_eq(a922, a923) {
        f164(v2, v3, {
            let a924 = v4;
            let a925 = 1u64;
            nat_add(a924, a925)?
        })
    } else {
        let m927 = match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h926, _, _, _, _) => {
                h926
            }
        };
        match m927.uncons() {
            None => {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("a function takes at least one parameter"))))
            }
            Some((_, _)) => {
                if f119(v0.clone(), v4, f11(Str::lit("=>"))) {
                    f120(v2, f116(v3, 1u64), {
                        let a928 = v4;
                        let a929 = 1u64;
                        nat_add(a928, a929)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `=>`"))))
                }
            }
        }
    }
}

pub fn f122(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: Bytes = match v3.clone() {
        Adt7::C0(_, _, _, _, h930, _, _, _, _, _, _, _) => {
            h930
        }
    };
    let a932 = {
        let a931 = v5.clone();
        length_bytes(a931)
    };
    let a933 = 0u64;
    if if nat_eq(a932, a933) {
        let a934 = f75(v0.clone(), v4);
        let a935 = 5u64;
        nat_eq(a934, a935)
    } else {
        f119(v0.clone(), v4, v5)
    } {
        f165(v0, v1, v2, v3, v4)
    } else {
        let a936 = f75(v0, v4);
        let a937 = 99u64;
        if nat_eq(a936, a937) {
            Ok(f113(v2, v1, f11(Str::lit("expected `)`"))))
        } else {
            f120(v2, v3, v4)
        }
    }
}

pub fn f123(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7) -> R<Adt6> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h938, _, _, _, _, _, _, _, _) => {
            h938
        }
    };
    let v5: u64 = match v3.clone() {
        Adt7::C0(h939, _, _, _, _, _, _, _, _, _, _, _) => {
            h939
        }
    };
    let v6: Adt7 = f104(v3.clone(), match v3.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h940, _, _) => {
            (*h940).clone()
        }
    }, match v2.clone() {
        Adt6::C0(_, _, h941, _, _, _, _, _, _) => {
            (*h941).clone()
        }
    });
    let a942 = v5;
    let a943 = 41u64;
    if nat_eq(a942, a943) {
        Ok(f78(v2.clone(), v4, f91(f11(Str::lit("function_ref")), f166(f94(Str::lit("function"), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h944, _, _) => {
                (*h944).clone()
            }
        }), match v2 {
            Adt6::C0(_, _, h945, _, _, _, _, _, _) => {
                (*h945).clone()
            }
        }))?))
    } else {
        let a946 = v5;
        let a947 = 42u64;
        if nat_eq(a946, a947) {
            let a948 = f75(v0.clone(), v4);
            let a949 = 0u64;
            if nat_eq(a948, a949) {
                f124(v0, v1, v2, f116(v6, 1u64), {
                    let a950 = v4;
                    let a951 = 1u64;
                    nat_add(a950, a951)?
                })
            } else {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `{` before the fields"))))
            }
        } else {
            f122(v0, v1, v2, f116(v6, 1u64), v4)
        }
    }
}

pub fn f124(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a952 = f75(v0.clone(), v4);
    let a953 = 1u64;
    if nat_eq(a952, a953) {
        Ok(f78(v2, {
            let a954 = v4;
            let a955 = 1u64;
            nat_add(a954, a955)?
        }, f91(f11(Str::lit("record")), f166(f98(Str::lit("type"), match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h956, _, _) => {
                (*h956).clone()
            }
        }, Str::lit("fields"), f79(match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h957, _, _, _, _) => {
                h957
            }
        })), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h958, _) => {
                (*h958).clone()
            }
        }))?))
    } else {
        let v5: Adt12 = f103(v0.clone(), v1, v4, f11(Str::lit("a field name")))?;
        let m960 = match v5.clone() {
            Adt12::C0(_, _, h959) => {
                h959
            }
        };
        match m960 {
            None => {
                let a963 = f75(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h962, _) => {
                        h962
                    }
                });
                let a964 = 8u64;
                if nat_eq(a963, a964) {
                    f120(v2, f110(v3, {
                        let m966 = f50(match v5.clone() {
                            Adt12::C0(h965, _, _) => {
                                (*h965).clone()
                            }
                        });
                        match m966 {
                            None => {
                                f11(Str::lit(""))
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, 0u64), {
                        let a968 = match v5 {
                            Adt12::C0(_, h967, _) => {
                                h967
                            }
                        };
                        let a969 = 1u64;
                        nat_add(a968, a969)?
                    })
                } else {
                    Ok(f73(v0, v1, v2, match v5 {
                        Adt12::C0(_, h970, _) => {
                            h970
                        }
                    }, f11(Str::lit("expected `:=` after a field name"))))
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h961) => {
                        h961
                    }
                }))
            }
        }
    }
}

pub fn f125(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: u64 = f75(v0.clone(), v4);
    let a971 = v5;
    let a972 = 7u64;
    if nat_eq(a971, a972) {
        f115(v2, f116(v3, 1u64), {
            let a973 = v4;
            let a974 = 1u64;
            nat_add(a973, a974)?
        })
    } else {
        let a975 = v5;
        let a976 = 99u64;
        if if nat_eq(a975, a976) {
            true
        } else {
            let a977 = v5;
            let a978 = 5u64;
            nat_eq(a977, a978)
        } {
            Ok(f73(v0, v1, v2, v4, f11(Str::lit("expected `:` and the result type"))))
        } else {
            f120(v2, v3, v4)
        }
    }
}

pub fn f126(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    if f119(v0.clone(), v4, f11(Str::lit("|"))) {
        let v5: Adt12 = f76(v0.clone(), v1, {
            let a979 = v4;
            let a980 = 1u64;
            nat_add(a979, a980)?
        })?;
        let m982 = match v5.clone() {
            Adt12::C0(_, _, h981) => {
                h981
            }
        };
        match m982 {
            None => {
                let v7: Adt15 = {
                    let (h984, _) = f167(Fn13::F168(v1, v0.clone()), {
                        let a986 = {
                            let a985 = v0;
                            length_list(a985)
                        };
                        let a987 = 2u64;
                        nat_add(a986, a987)?
                    }, Adt15::C0(List::<Adt0>::nil(), match v5.clone() {
                        Adt12::C0(_, h988, _) => {
                            h988
                        }
                    }, true, None::<Adt1>))?;
                    h984
                };
                let m990 = match v7.clone() {
                    Adt15::C0(_, _, _, h989) => {
                        h989
                    }
                };
                match m990 {
                    None => {
                        f120(v2, f5(f116(v3.clone(), 1u64), match v3.clone() {
                            Adt7::C0(_, _, _, _, _, h992, _, _, _, _, _, _) => {
                                (*h992).clone()
                            }
                        }, match v3 {
                            Adt7::C0(_, _, _, _, _, _, h993, _, _, _, _, _) => {
                                h993
                            }
                        }, f98(Str::lit("constructor"), match v5 {
                            Adt12::C0(h994, _, _) => {
                                (*h994).clone()
                            }
                        }, Str::lit("binders"), f79(match v7.clone() {
                            Adt15::C0(h995, _, _, _) => {
                                h995
                            }
                        }))), {
                            let a997 = match v7 {
                                Adt15::C0(_, h996, _, _) => {
                                    h996
                                }
                            };
                            let a998 = 1u64;
                            nat_add(a997, a998)?
                        })
                    }
                    Some(_) => {
                        Ok(f77(v2, match v7 {
                            Adt15::C0(_, _, _, h991) => {
                                h991
                            }
                        }))
                    }
                }
            }
            Some(_) => {
                Ok(f77(v2, match v5 {
                    Adt12::C0(_, _, h983) => {
                        h983
                    }
                }))
            }
        }
    } else {
        let m1000 = match v3.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h999, _, _, _, _) => {
                h999
            }
        };
        match m1000.uncons() {
            None => {
                Ok(f73(v0, v1, v2, v4, f11(Str::lit("a match has at least one `| \u{2026}` branch"))))
            }
            Some((_, _)) => {
                Ok(f78(v2, v4, f91(f11(Str::lit("match")), f98(Str::lit("scrutinee"), match v3.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h1001, _, _) => {
                        (*h1001).clone()
                    }
                }, Str::lit("branches"), f79(match v3 {
                    Adt7::C0(_, _, _, _, _, _, _, h1002, _, _, _, _) => {
                        h1002
                    }
                })))?))
            }
        }
    }
}

pub fn f127(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("+"))) {
        Some(f11(Str::lit("add")))
    } else {
        if f36(v0.clone(), f11(Str::lit("="))) {
            Some(f11(Str::lit("eq")))
        } else {
            if f36(v0.clone(), f11(Str::lit("<="))) {
                Some(f11(Str::lit("le")))
            } else {
                if f36(v0.clone(), f11(Str::lit("<"))) {
                    Some(f11(Str::lit("lt")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("=?"))) {
                        Some(f11(Str::lit("beq")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("<=?"))) {
                            Some(f11(Str::lit("ble")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("<?"))) {
                                Some(f11(Str::lit("blt")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("&&"))) {
                                    Some(f11(Str::lit("and")))
                                } else {
                                    if f36(v0.clone(), f11(Str::lit("||"))) {
                                        Some(f11(Str::lit("or")))
                                    } else {
                                        if f36(v0.clone(), f11(Str::lit("/\\"))) {
                                            Some(f11(Str::lit("prop_and")))
                                        } else {
                                            if f36(v0.clone(), f11(Str::lit("<->"))) {
                                                Some(f11(Str::lit("iff")))
                                            } else {
                                                if f36(v0, f11(Str::lit("->"))) {
                                                    Some(f11(Str::lit("implies")))
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

pub fn f128(v0: Bytes, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let m1004 = match v2.clone() {
        Adt9::C0(_, _, _, h1003) => {
            h1003
        }
    };
    match m1004 {
        None => {
            if match v2.clone() {
                Adt9::C0(_, _, h1005, _) => {
                    h1005
                }
            } {
                Ok(None::<Adt9>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt9::C0(h1006, _, _, _) => {
                        h1006
                    }
                };
                let a1007 = v4;
                let a1008 = v1;
                if nat_lt(a1007, a1008) {
                    if f26(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt9::C0({
                            let a1009 = v4;
                            let a1010 = 1u64;
                            nat_add(a1009, a1010)?
                        }, match v2 {
                            Adt9::C0(_, h1011, _, _) => {
                                h1011
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f26(v0.clone(), v4, 92u8) {
                            Ok(Some(f169(v0, v1, v2)?))
                        } else {
                            let a1012 = f170(v0.clone(), v4)?;
                            let a1013 = 32u64;
                            if nat_lt(a1012, a1013) {
                                Ok(Some(f171(v2, v4, f45(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f172(v0.clone(), v1, v4)?;
                                Ok(Some(f173(v2, v5, f47(v0, v4, {
                                    let a1014 = v5;
                                    let a1015 = v4;
                                    nat_sub(a1014, a1015)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt9>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt9>)
        }
    }
}

pub fn f129(v0: Fn8, p1020: u64, p1021: List<Bytes>) -> (List<Bytes>, bool) {
    let m1019 = (p1020, p1021);
    {
        let mut m1019 = m1019;
        loop {
            let step = {
                let (v1, v2) = m1019;
                let m1016 = v1;
                if m1016 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m1016 - 1;
                    let m1018 = {
                        let c1017: Fn8 = v0.clone();
                        c1017.apply(v2.clone())
                    };
                    match m1018 {
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
                Ok(next) => { m1019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>) -> Option<List<Bytes>> {
    f174(v0)
}

pub fn f131(v0: Fn9, p1025: List<Bytes>, p1026: List<Bytes>) -> List<Bytes> {
    let m1024 = (p1025, p1026);
    {
        let mut m1024 = m1024;
        loop {
            let step = {
                let (v1, v2) = m1024;
                let m1022 = v2;
                match m1022.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1023: Fn9 = v0.clone();
                            c1023.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f133(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1027 = v2;
    let a1028 = v1;
    if nat_lt(a1027, a1028) {
        if f175(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a1029 = v2;
                let a1030 = 1u64;
                nat_add(a1029, a1030)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f134(v0: List<Adt3>, v1: u64) -> u64 {
    let m1031 = f148(v0, v1);
    match m1031 {
        None => {
            99u64
        }
        Some(v2) => {
            f176(match v2 {
                Adt3::C0(h1032, _, _) => {
                    (*h1032).clone()
                }
            })
        }
    }
}

pub fn f135(v0: List<Adt0>) -> Adt0 {
    f177(Fn14::F178, Adt0::C5, v0)
}

pub fn f136(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m1033 = v1;
    match m1033 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m1034 = f10(f22(v0.clone(), v3)?, f11(Str::lit("properties")));
            match m1034 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m1035 = f10(v4, v2);
                    match m1035 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f23(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: List<Adt3>, v1: u64) -> Bytes {
    let m1036 = f148(v0, v1);
    match m1036 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f179(match v2 {
                Adt3::C0(h1037, _, _) => {
                    (*h1037).clone()
                }
            })
        }
    }
}

pub fn f138(v0: u64) -> bool {
    let a1038 = 64u64;
    let a1039 = v0;
    if if nat_lt(a1038, a1039) {
        let a1040 = v0;
        let a1041 = 91u64;
        nat_lt(a1040, a1041)
    } else {
        false
    } {
        true
    } else {
        let a1042 = 96u64;
        let a1043 = v0;
        if if nat_lt(a1042, a1043) {
            let a1044 = v0;
            let a1045 = 123u64;
            nat_lt(a1044, a1045)
        } else {
            false
        } {
            true
        } else {
            let a1046 = v0;
            let a1047 = 95u64;
            nat_eq(a1046, a1047)
        }
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let m1048 = f41(v0, v1);
    match m1048 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1093 = {
                let a1069 = {
                    let a1057 = {
                        let a1051 = {
                            let a1049 = v3;
                            let a1050 = 128u8;
                            fixed_u8::bit_and(a1049, a1050)
                        };
                        let a1052 = 128u8;
                        if equal(a1051, a1052) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1058 = {
                        let a1055 = {
                            let a1053 = v3;
                            let a1054 = 64u8;
                            fixed_u8::bit_and(a1053, a1054)
                        };
                        let a1056 = 64u8;
                        if equal(a1055, a1056) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1057, a1058)?
                };
                let a1070 = {
                    let a1067 = {
                        let a1061 = {
                            let a1059 = v3;
                            let a1060 = 32u8;
                            fixed_u8::bit_and(a1059, a1060)
                        };
                        let a1062 = 32u8;
                        if equal(a1061, a1062) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1068 = {
                        let a1065 = {
                            let a1063 = v3;
                            let a1064 = 16u8;
                            fixed_u8::bit_and(a1063, a1064)
                        };
                        let a1066 = 16u8;
                        if equal(a1065, a1066) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1067, a1068)?
                };
                nat_add(a1069, a1070)?
            };
            let a1094 = {
                let a1091 = {
                    let a1079 = {
                        let a1073 = {
                            let a1071 = v3;
                            let a1072 = 8u8;
                            fixed_u8::bit_and(a1071, a1072)
                        };
                        let a1074 = 8u8;
                        if equal(a1073, a1074) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1080 = {
                        let a1077 = {
                            let a1075 = v3;
                            let a1076 = 4u8;
                            fixed_u8::bit_and(a1075, a1076)
                        };
                        let a1078 = 4u8;
                        if equal(a1077, a1078) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1079, a1080)?
                };
                let a1092 = {
                    let a1089 = {
                        let a1083 = {
                            let a1081 = v3;
                            let a1082 = 2u8;
                            fixed_u8::bit_and(a1081, a1082)
                        };
                        let a1084 = 2u8;
                        if equal(a1083, a1084) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1090 = {
                        let a1087 = {
                            let a1085 = v3;
                            let a1086 = 1u8;
                            fixed_u8::bit_and(a1085, a1086)
                        };
                        let a1088 = 1u8;
                        if equal(a1087, a1088) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1089, a1090)?
                };
                nat_add(a1091, a1092)?
            };
            nat_add(a1093, a1094)
        }
    }
}

pub fn f140(v0: Bytes, v1: u64) -> R<u64> {
    let (h1095, _) = f70(Fn7::F180(v0.clone()), {
        let a1097 = {
            let a1096 = v0;
            length_bytes(a1096)
        };
        let a1098 = 1u64;
        nat_add(a1097, a1098)?
    }, v1)?;
    Ok(h1095)
}

pub fn f141(v0: Bytes) -> bool {
    let a1099 = v0.clone();
    let a1100 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1099, a1100) {
        true
    } else {
        let a1101 = v0;
        let a1102 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a1101, a1102)
    }
}

pub fn f142(v0: Bytes, v1: u64) -> R<u64> {
    let (h1103, _) = f70(Fn7::F181(v0.clone()), {
        let a1105 = {
            let a1104 = v0;
            length_bytes(a1104)
        };
        let a1106 = 1u64;
        nat_add(a1105, a1106)?
    }, v1)?;
    Ok(h1103)
}

pub fn f143(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m1107 = f182(f10(f22(v0.clone(), v1)?, f11(Str::lit("oneOf"))));
    match m1107 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f183(Fn15::F184(v2, std::rc::Rc::new(v0)), None::<Adt0>, f185(v3))
        }
    }
}

pub fn f144(v0: Adt0, v1: Adt0) -> R<bool> {
    let m1108 = f182(f10(f22(v0.clone(), v1)?, f11(Str::lit("oneOf"))));
    match m1108 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f186(Fn16::F187(std::rc::Rc::new(v0)), true, f185(v2))
        }
    }
}

pub fn f145(v0: Fn10, p1112: bool, p1113: List<Bytes>) -> bool {
    let m1111 = (p1112, p1113);
    {
        let mut m1111 = m1111;
        loop {
            let step = {
                let (v1, v2) = m1111;
                let m1109 = v2;
                match m1109.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1110: Fn10 = v0.clone();
                            c1110.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1114 = v2;
        let a1115 = v0;
        equal(a1114, a1115)
    }
}

pub fn f147(v0: Bytes) -> bool {
    let m1116 = f90(v0.clone());
    match m1116 {
        None => {
            f95(v0, f96())
        }
        Some(_) => {
            true
        }
    }
}

pub fn f148(v0: List<Adt3>, v1: u64) -> Option<Adt3> {
    let a1117 = v0;
    let a1118 = v1;
    index_list(a1117, a1118)
}

pub fn f149(v0: Adt4) -> R<Bytes> {
    let m1119 = v0;
    match m1119 {
        Adt4::C0 => {
            Ok(f150(Str::lit("`{`")))
        }
        Adt4::C1 => {
            Ok(f150(Str::lit("`}`")))
        }
        Adt4::C2 => {
            Ok(f150(Str::lit("`[`")))
        }
        Adt4::C3 => {
            Ok(f150(Str::lit("`]`")))
        }
        Adt4::C4 => {
            Ok(f150(Str::lit("`(`")))
        }
        Adt4::C5 => {
            Ok(f150(Str::lit("`)`")))
        }
        Adt4::C6 => {
            Ok(f150(Str::lit("`,`")))
        }
        Adt4::C7 => {
            Ok(f150(Str::lit("`:`")))
        }
        Adt4::C8 => {
            Ok(f150(Str::lit("`:=`")))
        }
        Adt4::C9 => {
            Ok(f150(Str::lit("`::`")))
        }
        Adt4::C10 => {
            Ok(f150(Str::lit("`;`")))
        }
        Adt4::C11(v1) => {
            Ok(f188(f150(Str::lit("the string ")), f189(v1)?))
        }
        Adt4::C12(v2) => {
            Ok(f188(f188(f150(Str::lit("`")), v2), f150(Str::lit("`"))))
        }
    }
}

pub fn f150(v0: Str) -> Bytes {
    let a1120 = v0;
    utf8_encode(a1120)
}

pub fn f151(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: Bytes) -> R<Adt6> {
    let v5: u64 = f111(v0.clone(), v1, v3);
    let v6: Bytes = if f92(v4.clone(), f11(Str::lit("-"))) {
        f93(v4.clone(), 1u64)
    } else {
        v4.clone()
    };
    let v7: Adt12 = f103(v0, v1, {
        let a1121 = v3;
        let a1122 = 2u64;
        nat_add(a1121, a1122)?
    }, f11(Str::lit("an integer type")))?;
    let m1124 = match v7.clone() {
        Adt12::C0(_, _, h1123) => {
            h1123
        }
    };
    match m1124 {
        None => {
            let v10: Bytes = {
                let m1127 = f50(match v7.clone() {
                    Adt12::C0(h1126, _, _) => {
                        (*h1126).clone()
                    }
                });
                match m1127 {
                    None => {
                        f11(Str::lit(""))
                    }
                    Some(v9) => {
                        v9
                    }
                }
            };
            let m1128 = f190(v10.clone());
            match m1128 {
                None => {
                    Ok(f113(v2, v5, f97(f153(v10), f11(Str::lit(" is not an integer type")))))
                }
                Some(v11) => {
                    if if f36(v4.clone(), f11(Str::lit("-0"))) {
                        true
                    } else {
                        f85({
                            let a1130 = 1u64;
                            let a1131 = {
                                let a1129 = v6.clone();
                                length_bytes(a1129)
                            };
                            nat_lt(a1130, a1131)
                        }, f92(v6, f11(Str::lit("0"))))
                    } {
                        Ok(f113(v2, v5, f97(f153(v4), f11(Str::lit(" is not a canonical integer")))))
                    } else {
                        Ok(f78(v2, match v7 {
                            Adt12::C0(_, h1132, _) => {
                                h1132
                            }
                        }, f91(f11(Str::lit("integer")), f98(Str::lit("representation"), f89(v11), Str::lit("value"), f89(v4)))?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(f77(v2, match v7 {
                Adt12::C0(_, _, h1125) => {
                    h1125
                }
            }))
        }
    }
}

pub fn f152(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64, v5: Bytes) -> R<Adt6> {
    let v6: u64 = {
        let a1133 = v3;
        let a1134 = 1u64;
        nat_add(a1133, a1134)?
    };
    if if f36(v5.clone(), f11(Str::lit("not"))) {
        true
    } else {
        if f36(v5.clone(), f11(Str::lit("first"))) {
            true
        } else {
            f36(v5.clone(), f11(Str::lit("second")))
        }
    } {
        f120(v2, f110(f6(31u64, v4), v5, 0u64), v6)
    } else {
        if f36(v5.clone(), f11(Str::lit("cons"))) {
            f120(v2, f6(32u64, v4), v6)
        } else {
            if f36(v5.clone(), f11(Str::lit("nil"))) {
                f115(v2, f6(33u64, v4), v6)
            } else {
                if f36(v5.clone(), f11(Str::lit("if"))) {
                    f120(v2, f6(34u64, v4), v6)
                } else {
                    if f36(v5.clone(), f11(Str::lit("let"))) {
                        f164(v2, f6(35u64, v4), v6)
                    } else {
                        if f36(v5.clone(), f11(Str::lit("forall"))) {
                            let a1135 = f75(v0.clone(), v6);
                            let a1136 = 4u64;
                            if nat_eq(a1135, a1136) {
                                f164(v2, f6(36u64, v4), {
                                    let a1137 = v6;
                                    let a1138 = 1u64;
                                    nat_add(a1137, a1138)?
                                })
                            } else {
                                Ok(f73(v0, v1, v2, v6, f11(Str::lit("expected `(` before the bound variable"))))
                            }
                        } else {
                            if f36(v5.clone(), f11(Str::lit("fun"))) {
                                let a1139 = f75(v0.clone(), v6);
                                let a1140 = 2u64;
                                if nat_eq(a1139, a1140) {
                                    let v7: Adt12 = f191(v0.clone(), v1, v6, f11(Str::lit("a name")))?;
                                    let m1142 = match v7.clone() {
                                        Adt12::C0(_, _, h1141) => {
                                            h1141
                                        }
                                    };
                                    match m1142 {
                                        None => {
                                            f121(v0, v1, v2, f104(f6(37u64, v4), match v7.clone() {
                                                Adt12::C0(h1144, _, _) => {
                                                    (*h1144).clone()
                                                }
                                            }, Adt0::C5), match v7 {
                                                Adt12::C0(_, h1145, _) => {
                                                    h1145
                                                }
                                            })
                                        }
                                        Some(_) => {
                                            Ok(f77(v2, match v7 {
                                                Adt12::C0(_, _, h1143) => {
                                                    h1143
                                                }
                                            }))
                                        }
                                    }
                                } else {
                                    f121(v0, v1, v2, f104(f6(37u64, v4), f79(List::<Adt0>::nil()), Adt0::C5), v6)
                                }
                            } else {
                                if f36(v5.clone(), f11(Str::lit("apply"))) {
                                    f120(v2, f6(38u64, v4), v6)
                                } else {
                                    if f36(v5.clone(), f11(Str::lit("project"))) {
                                        f120(v2, f6(39u64, v4), v6)
                                    } else {
                                        if if f36(v5.clone(), f11(Str::lit("ctor"))) {
                                            true
                                        } else {
                                            if f36(v5.clone(), f11(Str::lit("ref"))) {
                                                true
                                            } else {
                                                if f36(v5.clone(), f11(Str::lit("record"))) {
                                                    true
                                                } else {
                                                    f36(v5.clone(), f11(Str::lit("checked")))
                                                }
                                            }
                                        } {
                                            let v9: Adt12 = f76(v0.clone(), v1, v6)?;
                                            let m1147 = match v9.clone() {
                                                Adt12::C0(_, _, h1146) => {
                                                    h1146
                                                }
                                            };
                                            match m1147 {
                                                None => {
                                                    f192(v0, v1, v2, f110(f104(f6(if f36(v5.clone(), f11(Str::lit("ctor"))) {
                                                        40u64
                                                    } else {
                                                        if f36(v5.clone(), f11(Str::lit("ref"))) {
                                                            41u64
                                                        } else {
                                                            if f36(v5.clone(), f11(Str::lit("record"))) {
                                                                42u64
                                                            } else {
                                                                44u64
                                                            }
                                                        }
                                                    }, v4), match v9.clone() {
                                                        Adt12::C0(h1149, _, _) => {
                                                            (*h1149).clone()
                                                        }
                                                    }, Adt0::C5), if f36(v5, f11(Str::lit("checked"))) {
                                                        f11(Str::lit("with"))
                                                    } else {
                                                        f11(Str::lit(""))
                                                    }, 0u64), match v9 {
                                                        Adt12::C0(_, h1150, _) => {
                                                            h1150
                                                        }
                                                    })
                                                }
                                                Some(_) => {
                                                    Ok(f77(v2, match v9 {
                                                        Adt12::C0(_, _, h1148) => {
                                                            h1148
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            if f36(v5.clone(), f11(Str::lit("prim"))) {
                                                let v11: Adt12 = f103(v0.clone(), v1, v6, f11(Str::lit("a primitive operation")))?;
                                                let m1152 = match v11.clone() {
                                                    Adt12::C0(_, _, h1151) => {
                                                        h1151
                                                    }
                                                };
                                                match m1152 {
                                                    None => {
                                                        f125(v0, v1, v2, f104(f6(43u64, v4), match v11.clone() {
                                                            Adt12::C0(h1154, _, _) => {
                                                                (*h1154).clone()
                                                            }
                                                        }, Adt0::C5), match v11 {
                                                            Adt12::C0(_, h1155, _) => {
                                                                h1155
                                                            }
                                                        })
                                                    }
                                                    Some(_) => {
                                                        Ok(f77(v2, match v11 {
                                                            Adt12::C0(_, _, h1153) => {
                                                                h1153
                                                            }
                                                        }))
                                                    }
                                                }
                                            } else {
                                                if f36(v5, f11(Str::lit("match"))) {
                                                    f120(v2, f6(45u64, v4), v6)
                                                } else {
                                                    f193(v0, v1, v2, v3, v4)
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

pub fn f153(v0: Bytes) -> Bytes {
    f97(f97(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f154(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h1156, _) = f70(Fn7::F194(v0.clone()), {
            let a1158 = {
                let a1157 = v0.clone();
                length_bytes(a1157)
            };
            let a1159 = 1u64;
            nat_add(a1158, a1159)?
        }, 0u64)?;
        h1156
    };
    Ok(f85({
        let a1161 = v1;
        let a1162 = {
            let a1160 = v0.clone();
            length_bytes(a1160)
        };
        nat_eq(a1161, a1162)
    }, {
        let a1167 = {
            let a1164 = {
                let a1163 = v0;
                length_bytes(a1163)
            };
            let a1165 = 2u64;
            let a1166 = 0u64;
            nat_rem(a1164, a1165, a1166)
        };
        let a1168 = 0u64;
        nat_eq(a1167, a1168)
    }))
}

pub fn f155(v0: Bytes) -> R<bool> {
    Ok(f85(f86(v0.clone())?, f85(!f95(v0.clone(), f156()), !f36(v0, f11(Str::lit("false"))))))
}

pub fn f156() -> List<Bytes> {
    List::cons(f11(Str::lit("let")), List::cons(f11(Str::lit("if")), List::cons(f11(Str::lit("match")), List::cons(f11(Str::lit("fun")), List::cons(f11(Str::lit("forall")), List::cons(f11(Str::lit("nil")), List::cons(f11(Str::lit("cons")), List::cons(f11(Str::lit("first")), List::cons(f11(Str::lit("second")), List::cons(f11(Str::lit("not")), List::cons(f11(Str::lit("apply")), List::cons(f11(Str::lit("ctor")), List::cons(f11(Str::lit("record")), List::cons(f11(Str::lit("project")), List::cons(f11(Str::lit("prim")), List::cons(f11(Str::lit("ref")), List::cons(f11(Str::lit("checked")), List::cons(f11(Str::lit("then")), List::cons(f11(Str::lit("else")), List::cons(f11(Str::lit("with")), List::cons(f11(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f157(v0: u64, v1: Bytes) -> Option<Adt1> {
    Some(Adt1::C0(v0, v1))
}

pub fn f158(v0: Fn11, p1173: u64, p1174: Adt13) -> R<(Adt13, bool)> {
    let m1172 = (p1173, p1174);
    {
        let mut m1172 = m1172;
        loop {
            let step = {
                let (v1, v2) = m1172;
                let m1169 = v1;
                if m1169 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1169 - 1;
                    let m1171 = {
                        let c1170: Fn11 = v0.clone();
                        c1170.apply(v2.clone())?
                    };
                    match m1171 {
                        None => {
                            Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt13), R<(Adt13, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: u64, v1: List<Adt3>, v2: Adt13) -> R<Option<Adt13>> {
    f195(v1, v0, v2)
}

pub fn f160(v0: Fn12, p1178: bool, p1179: List<(Bytes, Adt0)>) -> bool {
    let m1177 = (p1178, p1179);
    {
        let mut m1177 = m1177;
        loop {
            let step = {
                let (v1, v2) = m1177;
                let m1175 = v2;
                match m1175.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c1176: Fn12 = v0.clone();
                            c1176.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a1181 = {
            let (h1180, _) = v2;
            h1180
        };
        let a1182 = v0;
        equal(a1181, a1182)
    }
}

pub fn f162(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f196(Fn17::F197, Adt0::C5, f198(v0))
}

pub fn f163(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f199(f200(Fn18::F201, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m1187 = {
        let (h1183, _) = f202(Fn19::F203, {
            let a1185 = {
                let a1184 = v0;
                length_list(a1184)
            };
            let a1186 = 1u64;
            nat_add(a1185, a1186)?
        }, v1)?;
        h1183
    };
    match m1187.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f164(v0: Adt6, v1: Adt7, v2: u64) -> R<Adt6> {
    Ok(f81(v0, v1.clone(), v2, 4u64, Adt2::C3, {
        let a1189 = match v1 {
            Adt7::C0(_, _, h1188, _, _, _, _, _, _, _, _, _) => {
                h1188
            }
        };
        let a1190 = 1u64;
        nat_add(a1189, a1190)?
    }))
}

pub fn f165(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let v5: u64 = match v3.clone() {
        Adt7::C0(h1191, _, _, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    };
    let v6: Adt0 = f79(match v3.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h1192, _, _, _, _) => {
            h1192
        }
    });
    let a1193 = v5;
    let a1194 = 38u64;
    if nat_eq(a1193, a1194) {
        Ok(f78(v2, v4, f91(f11(Str::lit("apply")), f98(Str::lit("function"), match v3 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1195, _, _) => {
                (*h1195).clone()
            }
        }, Str::lit("arguments"), v6))?))
    } else {
        let a1196 = v5;
        let a1197 = 40u64;
        if nat_eq(a1196, a1197) {
            Ok(f78(v2, v4, f91(f11(Str::lit("constructor")), f166(f98(Str::lit("constructor"), match v3.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, h1198, _, _) => {
                    (*h1198).clone()
                }
            }, Str::lit("arguments"), v6), match v3 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1199, _) => {
                    (*h1199).clone()
                }
            }))?))
        } else {
            let a1200 = v5;
            let a1201 = 46u64;
            if nat_eq(a1200, a1201) {
                Ok(f78(v2, v4, f91(f11(Str::lit("call")), f166(f98(Str::lit("function"), match v3.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h1202, _, _) => {
                        (*h1202).clone()
                    }
                }, Str::lit("arguments"), v6), match v3 {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1203, _) => {
                        (*h1203).clone()
                    }
                }))?))
            } else {
                let v7: Adt12 = f191(v0, v1, {
                    let a1204 = v4;
                    let a1205 = 1u64;
                    nat_add(a1204, a1205)?
                }, f11(Str::lit("a check")))?;
                let m1207 = match v7.clone() {
                    Adt12::C0(_, _, h1206) => {
                        h1206
                    }
                };
                match m1207 {
                    None => {
                        Ok(f78(v2, match v7.clone() {
                            Adt12::C0(_, h1209, _) => {
                                h1209
                            }
                        }, f91(f11(Str::lit("checked_apply")), f166(List::cons((f11(Str::lit("checks")), match v7 {
                            Adt12::C0(h1210, _, _) => {
                                (*h1210).clone()
                            }
                        }), f98(Str::lit("model"), match v3.clone() {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, h1211, _, _) => {
                                (*h1211).clone()
                            }
                        }, Str::lit("arguments"), v6)), match v3 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1212, _) => {
                                (*h1212).clone()
                            }
                        }))?))
                    }
                    Some(_) => {
                        Ok(f77(v2, match v7 {
                            Adt12::C0(_, _, h1208) => {
                                h1208
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f166(v0: List<(Bytes, Adt0)>, v1: Adt0) -> List<(Bytes, Adt0)> {
    let m1213 = f204(v1.clone());
    match m1213 {
        None => {
            v0
        }
        Some(_) => {
            List::cons((f11(Str::lit("type_arguments")), v1), v0)
        }
    }
}

pub fn f167(v0: Fn13, p1218: u64, p1219: Adt15) -> R<(Adt15, bool)> {
    let m1217 = (p1218, p1219);
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let (v1, v2) = m1217;
                let m1214 = v1;
                if m1214 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1214 - 1;
                    let m1216 = {
                        let c1215: Fn13 = v0.clone();
                        c1215.apply(v2.clone())?
                    };
                    match m1216 {
                        None => {
                            Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt15), R<(Adt15, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: u64, v1: List<Adt3>, v2: Adt15) -> R<Option<Adt15>> {
    let m1221 = match v2.clone() {
        Adt15::C0(_, _, _, h1220) => {
            h1220
        }
    };
    match m1221 {
        None => {
            if f119(v1.clone(), match v2.clone() {
                Adt15::C0(_, h1222, _, _) => {
                    h1222
                }
            }, f11(Str::lit("=>"))) {
                Ok(None::<Adt15>)
            } else {
                let v4: Adt12 = f103(v1, v0, match v2.clone() {
                    Adt15::C0(_, h1223, _, _) => {
                        h1223
                    }
                }, f11(Str::lit("a binder name")))?;
                let m1225 = match v4.clone() {
                    Adt12::C0(_, _, h1224) => {
                        h1224
                    }
                };
                match m1225 {
                    None => {
                        Ok(Some(Adt15::C0(List::cons(match v4.clone() {
                            Adt12::C0(h1229, _, _) => {
                                (*h1229).clone()
                            }
                        }, match v2 {
                            Adt15::C0(h1230, _, _, _) => {
                                h1230
                            }
                        }), match v4 {
                            Adt12::C0(_, h1231, _) => {
                                h1231
                            }
                        }, true, None::<Adt1>)))
                    }
                    Some(_) => {
                        Ok(Some(Adt15::C0(match v2.clone() {
                            Adt15::C0(h1226, _, _, _) => {
                                h1226
                            }
                        }, match v2 {
                            Adt15::C0(_, h1227, _, _) => {
                                h1227
                            }
                        }, false, match v4 {
                            Adt12::C0(_, _, h1228) => {
                                h1228
                            }
                        })))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f169(v0: Bytes, v1: u64, v2: Adt9) -> R<Adt9> {
    let v3: u64 = match v2.clone() {
        Adt9::C0(h1232, _, _, _) => {
            h1232
        }
    };
    let m1235 = f41(v0.clone(), {
        let a1233 = v3;
        let a1234 = 1u64;
        nat_add(a1233, a1234)?
    });
    match m1235 {
        None => {
            Ok(Adt9::C0(v1, match v2 {
                Adt9::C0(_, h1236, _, _) => {
                    h1236
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a1237 = v4;
            let a1238 = 34u8;
            if equal(a1237, a1238) {
                Ok(f173(v2, {
                    let a1239 = v3;
                    let a1240 = 2u64;
                    nat_add(a1239, a1240)?
                }, f45(Str::lit("\""))))
            } else {
                let a1241 = v4;
                let a1242 = 92u8;
                if equal(a1241, a1242) {
                    Ok(f173(v2, {
                        let a1243 = v3;
                        let a1244 = 2u64;
                        nat_add(a1243, a1244)?
                    }, f45(Str::lit("\\"))))
                } else {
                    let a1245 = v4;
                    let a1246 = 47u8;
                    if equal(a1245, a1246) {
                        Ok(f173(v2, {
                            let a1247 = v3;
                            let a1248 = 2u64;
                            nat_add(a1247, a1248)?
                        }, f45(Str::lit("/"))))
                    } else {
                        let a1249 = v4;
                        let a1250 = 98u8;
                        if equal(a1249, a1250) {
                            Ok(f173(v2, {
                                let a1251 = v3;
                                let a1252 = 2u64;
                                nat_add(a1251, a1252)?
                            }, f205(8u64)))
                        } else {
                            let a1253 = v4;
                            let a1254 = 102u8;
                            if equal(a1253, a1254) {
                                Ok(f173(v2, {
                                    let a1255 = v3;
                                    let a1256 = 2u64;
                                    nat_add(a1255, a1256)?
                                }, f205(12u64)))
                            } else {
                                let a1257 = v4;
                                let a1258 = 110u8;
                                if equal(a1257, a1258) {
                                    Ok(f173(v2, {
                                        let a1259 = v3;
                                        let a1260 = 2u64;
                                        nat_add(a1259, a1260)?
                                    }, f205(10u64)))
                                } else {
                                    let a1261 = v4;
                                    let a1262 = 114u8;
                                    if equal(a1261, a1262) {
                                        Ok(f173(v2, {
                                            let a1263 = v3;
                                            let a1264 = 2u64;
                                            nat_add(a1263, a1264)?
                                        }, f205(13u64)))
                                    } else {
                                        let a1265 = v4;
                                        let a1266 = 116u8;
                                        if equal(a1265, a1266) {
                                            Ok(f173(v2, {
                                                let a1267 = v3;
                                                let a1268 = 2u64;
                                                nat_add(a1267, a1268)?
                                            }, f205(9u64)))
                                        } else {
                                            let a1269 = v4;
                                            let a1270 = 117u8;
                                            if equal(a1269, a1270) {
                                                f206(v0, v2)
                                            } else {
                                                Ok(f171(v2, v3, f207(f207(f45(Str::lit("`\\")), f47(v0.clone(), {
                                                    let a1271 = v3;
                                                    let a1272 = 1u64;
                                                    nat_add(a1271, a1272)?
                                                }, f208(v0, {
                                                    let a1273 = v3;
                                                    let a1274 = 1u64;
                                                    nat_add(a1273, a1274)?
                                                })?)), f45(Str::lit("` is not an escape")))))
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

pub fn f170(v0: Bytes, v1: u64) -> R<u64> {
    let m1275 = f41(v0, v1);
    match m1275 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1320 = {
                let a1296 = {
                    let a1284 = {
                        let a1278 = {
                            let a1276 = v3;
                            let a1277 = 128u8;
                            fixed_u8::bit_and(a1276, a1277)
                        };
                        let a1279 = 128u8;
                        if equal(a1278, a1279) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1285 = {
                        let a1282 = {
                            let a1280 = v3;
                            let a1281 = 64u8;
                            fixed_u8::bit_and(a1280, a1281)
                        };
                        let a1283 = 64u8;
                        if equal(a1282, a1283) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1284, a1285)?
                };
                let a1297 = {
                    let a1294 = {
                        let a1288 = {
                            let a1286 = v3;
                            let a1287 = 32u8;
                            fixed_u8::bit_and(a1286, a1287)
                        };
                        let a1289 = 32u8;
                        if equal(a1288, a1289) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1295 = {
                        let a1292 = {
                            let a1290 = v3;
                            let a1291 = 16u8;
                            fixed_u8::bit_and(a1290, a1291)
                        };
                        let a1293 = 16u8;
                        if equal(a1292, a1293) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1294, a1295)?
                };
                nat_add(a1296, a1297)?
            };
            let a1321 = {
                let a1318 = {
                    let a1306 = {
                        let a1300 = {
                            let a1298 = v3;
                            let a1299 = 8u8;
                            fixed_u8::bit_and(a1298, a1299)
                        };
                        let a1301 = 8u8;
                        if equal(a1300, a1301) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1307 = {
                        let a1304 = {
                            let a1302 = v3;
                            let a1303 = 4u8;
                            fixed_u8::bit_and(a1302, a1303)
                        };
                        let a1305 = 4u8;
                        if equal(a1304, a1305) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1306, a1307)?
                };
                let a1319 = {
                    let a1316 = {
                        let a1310 = {
                            let a1308 = v3;
                            let a1309 = 2u8;
                            fixed_u8::bit_and(a1308, a1309)
                        };
                        let a1311 = 2u8;
                        if equal(a1310, a1311) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1317 = {
                        let a1314 = {
                            let a1312 = v3;
                            let a1313 = 1u8;
                            fixed_u8::bit_and(a1312, a1313)
                        };
                        let a1315 = 1u8;
                        if equal(a1314, a1315) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1316, a1317)?
                };
                nat_add(a1318, a1319)?
            };
            nat_add(a1320, a1321)
        }
    }
}

pub fn f171(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(h1322, _, _, _) => {
            h1322
        }
    }, match v0 {
        Adt9::C0(_, h1323, _, _) => {
            h1323
        }
    }, false, Some(f44(v1, v2)))
}

pub fn f172(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1324, _) = f70(Fn7::F209(v1, v0), {
        let a1325 = v1;
        let a1326 = v2;
        nat_sub(a1325, a1326)
    }, v2)?;
    Ok(h1324)
}

pub fn f173(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(v1, List::cons(v2, match v0 {
        Adt9::C0(_, h1327, _, _) => {
            h1327
        }
    }), false, None::<Adt1>)
}

pub fn f174(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1328 = v0.clone();
    match m1328.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m1329 = v2;
            match m1329.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f210(Fn20::F211, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m1331 = {
                        let (h1330, _) = v5;
                        h1330
                    };
                    match m1331 {
                        None => {
                            Some(f69({
                                let (_, h1333) = v5;
                                h1333
                            }))
                        }
                        Some(v6) => {
                            Some(f69(List::cons(v6, {
                                let (_, h1332) = v5;
                                h1332
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: u64) -> R<bool> {
    let a1334 = 0u64;
    let a1335 = f25(v0.clone(), v1)?;
    if nat_lt(a1334, a1335) {
        Ok(true)
    } else {
        Ok(f212(v0, v1))
    }
}

pub fn f176(v0: Adt4) -> u64 {
    let m1336 = v0;
    match m1336 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            1u64
        }
        Adt4::C2 => {
            2u64
        }
        Adt4::C3 => {
            3u64
        }
        Adt4::C4 => {
            4u64
        }
        Adt4::C5 => {
            5u64
        }
        Adt4::C6 => {
            6u64
        }
        Adt4::C7 => {
            7u64
        }
        Adt4::C8 => {
            8u64
        }
        Adt4::C9 => {
            9u64
        }
        Adt4::C10 => {
            10u64
        }
        Adt4::C11(_) => {
            11u64
        }
        Adt4::C12(_) => {
            12u64
        }
    }
}

pub fn f177(v0: Fn14, p1340: Adt0, p1341: List<Adt0>) -> Adt0 {
    let m1339 = (p1340, p1341);
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let (v1, v2) = m1339;
                let m1337 = v2;
                match m1337.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c1338: Fn14 = v0.clone();
                            c1338.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f179(v0: Adt4) -> Bytes {
    let m1342 = v0;
    match m1342 {
        Adt4::C0 => {
            Bytes::lit(&[])
        }
        Adt4::C1 => {
            Bytes::lit(&[])
        }
        Adt4::C2 => {
            Bytes::lit(&[])
        }
        Adt4::C3 => {
            Bytes::lit(&[])
        }
        Adt4::C4 => {
            Bytes::lit(&[])
        }
        Adt4::C5 => {
            Bytes::lit(&[])
        }
        Adt4::C6 => {
            Bytes::lit(&[])
        }
        Adt4::C7 => {
            Bytes::lit(&[])
        }
        Adt4::C8 => {
            Bytes::lit(&[])
        }
        Adt4::C9 => {
            Bytes::lit(&[])
        }
        Adt4::C10 => {
            Bytes::lit(&[])
        }
        Adt4::C11(v1) => {
            v1
        }
        Adt4::C12(v2) => {
            v2
        }
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f213(v0, v1)
}

pub fn f181(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f214(v0, v1)
}

pub fn f182(v0: Option<Adt0>) -> Option<Adt0> {
    let m1343 = v0;
    match m1343 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f204(v1)
        }
    }
}

pub fn f183(v0: Fn15, p1347: Option<Adt0>, p1348: List<Adt0>) -> R<Option<Adt0>> {
    let m1346 = (p1347, p1348);
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let (v1, v2) = m1346;
                let m1344 = v2;
                match m1344.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c1345: Fn15 = v0.clone();
                            c1345.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m1349 = v2.clone();
    match m1349 {
        None => {
            let v5: Adt0 = f22(v1, v3)?;
            if f215(v5.clone(), v0)? {
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

pub fn f185(v0: Adt0) -> List<Adt0> {
    f216({
        let m1352 = {
            let (h1351, _) = f217(Fn21::F218, 1000000u64, Adt16::C0(std::rc::Rc::new(v0), List::<Adt0>::nil()));
            h1351
        };
        match m1352 {
            Adt16::C0(_, h1350) => {
                h1350
            }
        }
    })
}

pub fn f186(v0: Fn16, p1356: bool, p1357: List<Adt0>) -> R<bool> {
    let m1355 = (p1356, p1357);
    {
        let mut m1355 = m1355;
        loop {
            let step = {
                let (v1, v2) = m1355;
                let m1353 = v2;
                match m1353.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c1354: Fn16 = v0.clone();
                            c1354.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m1358 = f10(f22(v0, v2)?, f11(Str::lit("properties")));
        match m1358 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f219(f10(v3, f11(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f188(v0: Bytes, v1: Bytes) -> Bytes {
    let a1359 = v0;
    let a1360 = v1;
    append_bytes(a1359, a1360)
}

pub fn f189(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a1361 = v0.clone();
        length_bytes(a1361)
    };
    let v2: Adt17 = {
        let (h1362, _) = f220(Fn22::F221(v1, v0), {
            let a1363 = v1;
            let a1364 = 1u64;
            nat_add(a1363, a1364)?
        }, Adt17::C0(0u64, List::<Bytes>::nil()))?;
        h1362
    };
    Ok(f207(f207(f45(Str::lit("\"")), f43(match v2 {
        Adt17::C0(_, h1365) => {
            h1365
        }
    })), f45(Str::lit("\""))))
}

pub fn f190(v0: Bytes) -> Option<Bytes> {
    if f36(v0.clone(), f11(Str::lit("Int"))) {
        Some(f11(Str::lit("int")))
    } else {
        if f36(v0.clone(), f11(Str::lit("Int8"))) {
            Some(f11(Str::lit("int8")))
        } else {
            if f36(v0.clone(), f11(Str::lit("Int16"))) {
                Some(f11(Str::lit("int16")))
            } else {
                if f36(v0.clone(), f11(Str::lit("Int32"))) {
                    Some(f11(Str::lit("int32")))
                } else {
                    if f36(v0.clone(), f11(Str::lit("Int64"))) {
                        Some(f11(Str::lit("int64")))
                    } else {
                        if f36(v0.clone(), f11(Str::lit("UInt8"))) {
                            Some(f11(Str::lit("uint8")))
                        } else {
                            if f36(v0.clone(), f11(Str::lit("UInt16"))) {
                                Some(f11(Str::lit("uint16")))
                            } else {
                                if f36(v0.clone(), f11(Str::lit("UInt32"))) {
                                    Some(f11(Str::lit("uint32")))
                                } else {
                                    if f36(v0, f11(Str::lit("UInt64"))) {
                                        Some(f11(Str::lit("uint64")))
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

pub fn f191(v0: List<Adt3>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let a1366 = f75(v0.clone(), v2);
    let a1367 = 2u64;
    if nat_eq(a1366, a1367) {
        let v4: Adt15 = {
            let (h1368, _) = f167(Fn13::F222(v1, v0.clone(), v3), {
                let a1370 = {
                    let a1369 = v0.clone();
                    length_list(a1369)
                };
                let a1371 = 2u64;
                nat_add(a1370, a1371)?
            }, Adt15::C0(List::<Adt0>::nil(), {
                let a1372 = v2;
                let a1373 = 1u64;
                nat_add(a1372, a1373)?
            }, true, None::<Adt1>))?;
            h1368
        };
        let m1375 = match v4.clone() {
            Adt15::C0(_, _, _, h1374) => {
                h1374
            }
        };
        match m1375 {
            None => {
                let a1379 = f75(v0.clone(), match v4.clone() {
                    Adt15::C0(_, h1378, _, _) => {
                        h1378
                    }
                });
                let a1380 = 3u64;
                if nat_eq(a1379, a1380) {
                    Ok(Adt12::C0(std::rc::Rc::new(f79(match v4.clone() {
                        Adt15::C0(h1381, _, _, _) => {
                            h1381
                        }
                    })), {
                        let a1383 = match v4 {
                            Adt15::C0(_, h1382, _, _) => {
                                h1382
                            }
                        };
                        let a1384 = 1u64;
                        nat_add(a1383, a1384)?
                    }, None::<Adt1>))
                } else {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), match v4.clone() {
                        Adt15::C0(_, h1385, _, _) => {
                            h1385
                        }
                    }, f157(f111(v0, v1, match v4 {
                        Adt15::C0(_, h1386, _, _) => {
                            h1386
                        }
                    }), f11(Str::lit("expected `,` or `]`")))))
                }
            }
            Some(_) => {
                Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), match v4.clone() {
                    Adt15::C0(_, h1376, _, _) => {
                        h1376
                    }
                }, match v4 {
                    Adt15::C0(_, _, _, h1377) => {
                        h1377
                    }
                }))
            }
        }
    } else {
        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), v2, f157(f111(v0, v1, v2), f11(Str::lit("expected `[`")))))
    }
}

pub fn f192(v0: List<Adt3>, v1: u64, v2: Adt6, v3: Adt7, v4: u64) -> R<Adt6> {
    let a1387 = f75(v0.clone(), v4);
    let a1388 = 2u64;
    if nat_eq(a1387, a1388) {
        Ok(f81(v2, v3.clone(), v4, 6u64, Adt2::C5, match v3 {
            Adt7::C0(_, _, h1389, _, _, _, _, _, _, _, _, _) => {
                h1389
            }
        }))
    } else {
        f123(v0, v1, Adt6::C0(v4, match v2.clone() {
            Adt6::C0(_, h1390, _, _, _, _, _, _, _) => {
                h1390
            }
        }, std::rc::Rc::new(Adt0::C5), 1u64, match v2.clone() {
            Adt6::C0(_, _, _, _, h1391, _, _, _, _) => {
                h1391
            }
        }, std::rc::Rc::new(match v2.clone() {
            Adt6::C0(_, _, _, _, _, h1392, _, _, _) => {
                (*h1392).clone()
            }
        }), match v2.clone() {
            Adt6::C0(_, _, _, _, _, _, h1393, _, _) => {
                h1393
            }
        }, None::<Adt1>, match v2 {
            Adt6::C0(_, _, _, _, _, _, _, _, h1394) => {
                h1394
            }
        }), v3)
    }
}

pub fn f193(v0: List<Adt3>, v1: u64, v2: Adt6, v3: u64, v4: u64) -> R<Adt6> {
    let v5: Bytes = f84(v0.clone(), v3);
    let v6: u64 = {
        let a1397 = f75(v0.clone(), {
            let a1395 = v3;
            let a1396 = 1u64;
            nat_add(a1395, a1396)?
        });
        let a1398 = 9u64;
        if nat_eq(a1397, a1398) {
            3u64
        } else {
            1u64
        }
    };
    let v7: bool = {
        let a1401 = f75(v0.clone(), {
            let a1399 = v3;
            let a1400 = v6;
            nat_add(a1399, a1400)?
        });
        let a1402 = 6u64;
        if nat_eq(a1401, a1402) {
            true
        } else {
            f85({
                let a1405 = f75(v0.clone(), {
                    let a1403 = v3;
                    let a1404 = v6;
                    nat_add(a1403, a1404)?
                });
                let a1406 = 12u64;
                nat_eq(a1405, a1406)
            }, f223(f84(v0.clone(), {
                let a1407 = v3;
                let a1408 = v6;
                nat_add(a1407, a1408)?
            })))
        }
    };
    let v8: bool = {
        let a1411 = f75(v0.clone(), {
            let a1409 = v3;
            let a1410 = 1u64;
            nat_add(a1409, a1410)?
        });
        let a1412 = 0u64;
        nat_eq(a1411, a1412)
    };
    if f85({
        let a1413 = f75(v0.clone(), v3);
        let a1414 = 12u64;
        nat_eq(a1413, a1414)
    }, f85(f155(v5.clone())?, f85(!v7, f85(!v8, !f87(v5)?)))) {
        let v9: Adt12 = f76(v0.clone(), v1, v3)?;
        let m1416 = match v9.clone() {
            Adt12::C0(_, _, h1415) => {
                h1415
            }
        };
        match m1416 {
            None => {
                f192(v0, v1, v2, f104(f6(46u64, v4), match v9.clone() {
                    Adt12::C0(h1418, _, _) => {
                        (*h1418).clone()
                    }
                }, Adt0::C5), match v9 {
                    Adt12::C0(_, h1419, _) => {
                        h1419
                    }
                })
            }
            Some(_) => {
                Ok(f77(v2, match v9 {
                    Adt12::C0(_, _, h1417) => {
                        h1417
                    }
                }))
            }
        }
    } else {
        f120(v2, f6(47u64, v4), v3)
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1420 = f41(v0, v1);
    match m1420 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1465 = {
                    let a1441 = {
                        let a1429 = {
                            let a1423 = {
                                let a1421 = v3;
                                let a1422 = 128u8;
                                fixed_u8::bit_and(a1421, a1422)
                            };
                            let a1424 = 128u8;
                            if equal(a1423, a1424) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1430 = {
                            let a1427 = {
                                let a1425 = v3;
                                let a1426 = 64u8;
                                fixed_u8::bit_and(a1425, a1426)
                            };
                            let a1428 = 64u8;
                            if equal(a1427, a1428) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1429, a1430)?
                    };
                    let a1442 = {
                        let a1439 = {
                            let a1433 = {
                                let a1431 = v3;
                                let a1432 = 32u8;
                                fixed_u8::bit_and(a1431, a1432)
                            };
                            let a1434 = 32u8;
                            if equal(a1433, a1434) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1440 = {
                            let a1437 = {
                                let a1435 = v3;
                                let a1436 = 16u8;
                                fixed_u8::bit_and(a1435, a1436)
                            };
                            let a1438 = 16u8;
                            if equal(a1437, a1438) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1439, a1440)?
                    };
                    nat_add(a1441, a1442)?
                };
                let a1466 = {
                    let a1463 = {
                        let a1451 = {
                            let a1445 = {
                                let a1443 = v3;
                                let a1444 = 8u8;
                                fixed_u8::bit_and(a1443, a1444)
                            };
                            let a1446 = 8u8;
                            if equal(a1445, a1446) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1452 = {
                            let a1449 = {
                                let a1447 = v3;
                                let a1448 = 4u8;
                                fixed_u8::bit_and(a1447, a1448)
                            };
                            let a1450 = 4u8;
                            if equal(a1449, a1450) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1451, a1452)?
                    };
                    let a1464 = {
                        let a1461 = {
                            let a1455 = {
                                let a1453 = v3;
                                let a1454 = 2u8;
                                fixed_u8::bit_and(a1453, a1454)
                            };
                            let a1456 = 2u8;
                            if equal(a1455, a1456) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1462 = {
                            let a1459 = {
                                let a1457 = v3;
                                let a1458 = 1u8;
                                fixed_u8::bit_and(a1457, a1458)
                            };
                            let a1460 = 1u8;
                            if equal(a1459, a1460) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1461, a1462)?
                    };
                    nat_add(a1463, a1464)?
                };
                nat_add(a1465, a1466)?
            };
            if if f85({
                let a1467 = 48u64;
                let a1468 = v4;
                nat_le(a1467, a1468)
            }, {
                let a1469 = v4;
                let a1470 = 57u64;
                nat_le(a1469, a1470)
            }) {
                true
            } else {
                f85({
                    let a1471 = 97u64;
                    let a1472 = v4;
                    nat_le(a1471, a1472)
                }, {
                    let a1473 = v4;
                    let a1474 = 102u64;
                    nat_le(a1473, a1474)
                })
            } {
                Ok(Some({
                    let a1475 = v1;
                    let a1476 = 1u64;
                    nat_add(a1475, a1476)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f195(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m1478 = match v2.clone() {
        Adt13::C0(_, _, _, _, _, h1477) => {
            h1477
        }
    };
    match m1478 {
        None => {
            let a1480 = match v2.clone() {
                Adt13::C0(_, _, h1479, _, _, _) => {
                    h1479
                }
            };
            let a1481 = 9u64;
            if nat_eq(a1480, a1481) {
                Ok(None::<Adt13>)
            } else {
                Ok(Some(f224(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt13>)
        }
    }
}

pub fn f196(v0: Fn17, p1485: Adt0, p1486: List<(Bytes, Adt0)>) -> Adt0 {
    let m1484 = (p1485, p1486);
    {
        let mut m1484 = m1484;
        loop {
            let step = {
                let (v1, v2) = m1484;
                let m1482 = v2;
                match m1482.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c1483: Fn17 = v0.clone();
                            c1483.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h1487, _) = v1;
        h1487
    }, std::rc::Rc::new({
        let (_, h1488) = v1;
        h1488
    }), std::rc::Rc::new(v0))
}

pub fn f198(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f225(Fn23::F226, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f199(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f227(Fn24::F228, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f200(v0: Fn18, p1492: List<List<(Bytes, Adt0)>>, p1493: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m1491 = (p1492, p1493);
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let (v1, v2) = m1491;
                let m1489 = v2;
                match m1489.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c1490: Fn18 = v0.clone();
                            c1490.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f202(v0: Fn19, p1498: u64, p1499: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m1497 = (p1498, p1499);
    {
        let mut m1497 = m1497;
        loop {
            let step = {
                let (v1, v2) = m1497;
                let m1494 = v1;
                if m1494 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1494 - 1;
                    let m1496 = {
                        let c1495: Fn19 = v0.clone();
                        c1495.apply(v2.clone())?
                    };
                    match m1496 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f229(v0)
}

pub fn f204(v0: Adt0) -> Option<Adt0> {
    let m1500 = v0;
    match m1500 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r1501) => {
            let v1 = (*r1501).clone();
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

pub fn f205(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f206(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h1502, _, _, _) => {
            h1502
        }
    };
    let m1505 = f230(v0, {
        let a1503 = v2;
        let a1504 = 2u64;
        nat_add(a1503, a1504)?
    })?;
    match m1505 {
        None => {
            Ok(f171(v1, v2, f45(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a1506 = 55296u64;
            let a1507 = v3;
            if nat_le(a1506, a1507) {
                let a1508 = v3;
                let a1509 = 57343u64;
                if nat_le(a1508, a1509) {
                    Ok(f171(v1, v2, f207(f207(f45(Str::lit("`\\u")), f231(v3)), f45(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f173(v1, {
                        let a1510 = v2;
                        let a1511 = 6u64;
                        nat_add(a1510, a1511)?
                    }, f232(v3)?))
                }
            } else {
                Ok(f173(v1, {
                    let a1512 = v2;
                    let a1513 = 6u64;
                    nat_add(a1512, a1513)?
                }, f232(v3)?))
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: Bytes) -> Bytes {
    let a1514 = v0;
    let a1515 = v1;
    append_bytes(a1514, a1515)
}

pub fn f208(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f170(v0, v1)?;
    let a1516 = v2;
    let a1517 = 128u64;
    if nat_lt(a1516, a1517) {
        Ok(1u64)
    } else {
        let a1518 = v2;
        let a1519 = 224u64;
        if nat_lt(a1518, a1519) {
            Ok(2u64)
        } else {
            let a1520 = v2;
            let a1521 = 240u64;
            if nat_lt(a1520, a1521) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f209(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f233(v1, v0, v2)
}

pub fn f210(v0: Fn20, p1525: (Option<Bytes>, List<Bytes>), p1526: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1524 = (p1525, p1526);
    {
        let mut m1524 = m1524;
        loop {
            let step = {
                let (v1, v2) = m1524;
                let m1522 = v2;
                match m1522.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1523: Fn20 = v0.clone();
                            c1523.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f234(v0, v1)
}

pub fn f212(v0: Bytes, v1: u64) -> bool {
    if f26(v0.clone(), v1, 123u8) {
        true
    } else {
        if f26(v0.clone(), v1, 125u8) {
            true
        } else {
            if f26(v0.clone(), v1, 91u8) {
                true
            } else {
                if f26(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f26(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f26(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f26(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f26(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f26(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f26(v0, v1, 34u8)
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

pub fn f213(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f235(f139(v0, v1)?) {
        Ok(Some({
            let a1527 = v1;
            let a1528 = 1u64;
            nat_add(a1527, a1528)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f214(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f139(v0, v1)?;
    let a1529 = 47u64;
    let a1530 = v2;
    if nat_lt(a1529, a1530) {
        let a1531 = v2;
        let a1532 = 58u64;
        if nat_lt(a1531, a1532) {
            Ok(Some({
                let a1533 = v1;
                let a1534 = 1u64;
                nat_add(a1533, a1534)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f215(v0: Adt0, v1: Bytes) -> R<bool> {
    let m1535 = f10(v0, f11(Str::lit("properties")));
    match m1535 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1536 = f10(v2, f11(Str::lit("kind")));
            match m1536 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1537 = f34(f10(v3.clone(), f11(Str::lit("const"))));
                    if match m1537 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f36(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m1538 = f182(f10(v3, f11(Str::lit("enum"))));
                        match m1538 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f186(Fn16::F236(v1), false, f185(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f216(v0: List<Adt0>) -> List<Adt0> {
    f237(Fn25::F238, List::<Adt0>::nil(), v0)
}

pub fn f217(v0: Fn21, p1543: u64, p1544: Adt16) -> (Adt16, bool) {
    let m1542 = (p1543, p1544);
    {
        let mut m1542 = m1542;
        loop {
            let step = {
                let (v1, v2) = m1542;
                let m1539 = v1;
                if m1539 == 0 {
                    Err::<(u64, Adt16), (Adt16, bool)>((v2, false))
                } else {
                    let v3 = m1539 - 1;
                    let m1541 = {
                        let c1540: Fn21 = v0.clone();
                        c1540.apply(v2.clone())
                    };
                    match m1541 {
                        None => {
                            Err::<(u64, Adt16), (Adt16, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt16), (Adt16, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Adt16) -> Option<Adt16> {
    f239(v0)
}

pub fn f219(v0: Option<Adt0>) -> bool {
    let m1545 = v0;
    match m1545 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f220(v0: Fn22, p1550: u64, p1551: Adt17) -> R<(Adt17, bool)> {
    let m1549 = (p1550, p1551);
    {
        let mut m1549 = m1549;
        loop {
            let step = {
                let (v1, v2) = m1549;
                let m1546 = v1;
                if m1546 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1546 - 1;
                    let m1548 = {
                        let c1547: Fn22 = v0.clone();
                        c1547.apply(v2.clone())?
                    };
                    match m1548 {
                        None => {
                            Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt17), R<(Adt17, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: u64, v1: Bytes, v2: Adt17) -> R<Option<Adt17>> {
    f240(v1, v0, v2)
}

pub fn f222(v0: u64, v1: List<Adt3>, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    f241(v1, v0, v2, v3)
}

pub fn f223(v0: Bytes) -> bool {
    let m1552 = f127(v0);
    match m1552 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f224(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(_, _, h1553, _, _, _) => {
            h1553
        }
    };
    let a1554 = v3;
    let a1555 = 0u64;
    if nat_eq(a1554, a1555) {
        f242(v0, v1, v2)
    } else {
        let a1556 = v3;
        let a1557 = 1u64;
        if nat_eq(a1556, a1557) {
            f243(v0, v2)
        } else {
            let a1558 = v3;
            let a1559 = 2u64;
            if nat_eq(a1558, a1559) {
                f244(v0, v1, v2)
            } else {
                let a1560 = v3;
                let a1561 = 3u64;
                if nat_eq(a1560, a1561) {
                    f245(v0, v2)
                } else {
                    let a1562 = v3;
                    let a1563 = 4u64;
                    if nat_eq(a1562, a1563) {
                        f246(v0, v1, v2, 3u64, f150(Str::lit("expected `,` or `]`")))
                    } else {
                        f246(v0, v1, v2, 1u64, f150(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f225(v0: Fn23, p1567: List<(Bytes, Adt0)>, p1568: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m1566 = (p1567, p1568);
    {
        let mut m1566 = m1566;
        loop {
            let step = {
                let (v1, v2) = m1566;
                let m1564 = v2;
                match m1564.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c1565: Fn23 = v0.clone();
                            c1565.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f227(v0: Fn24, p1572: List<List<(Bytes, Adt0)>>, p1573: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m1571 = (p1572, p1573);
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let (v1, v2) = m1571;
                let m1569 = v2;
                match m1569.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c1570: Fn24 = v0.clone();
                            c1570.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f229(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a1575 = {
        let a1574 = v0.clone();
        length_list(a1574)
    };
    let a1576 = 1u64;
    if nat_le(a1575, a1576) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f247(v0)?))
    }
}

pub fn f230(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1577 = f248(v0.clone(), v1)?;
    match m1577 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1580 = f248(v0.clone(), {
                let a1578 = v1;
                let a1579 = 1u64;
                nat_add(a1578, a1579)?
            })?;
            match m1580 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m1583 = f248(v0.clone(), {
                        let a1581 = v1;
                        let a1582 = 2u64;
                        nat_add(a1581, a1582)?
                    })?;
                    match m1583 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m1586 = f248(v0, {
                                let a1584 = v1;
                                let a1585 = 3u64;
                                nat_add(a1584, a1585)?
                            })?;
                            match m1586 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a1597 = {
                                            let a1595 = {
                                                let a1593 = {
                                                    let a1591 = {
                                                        let a1589 = {
                                                            let a1587 = v2;
                                                            let a1588 = 16u64;
                                                            nat_mul(a1587, a1588)?
                                                        };
                                                        let a1590 = v3;
                                                        nat_add(a1589, a1590)?
                                                    };
                                                    let a1592 = 16u64;
                                                    nat_mul(a1591, a1592)?
                                                };
                                                let a1594 = v4;
                                                nat_add(a1593, a1594)?
                                            };
                                            let a1596 = 16u64;
                                            nat_mul(a1595, a1596)?
                                        };
                                        let a1598 = v5;
                                        nat_add(a1597, a1598)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: u64) -> Bytes {
    f207(f207(f249(v0, 4096u64), f249(v0, 256u64)), f207(f249(v0, 16u64), f250(v0)))
}

pub fn f232(v0: u64) -> R<Bytes> {
    let a1599 = v0;
    let a1600 = 128u64;
    if nat_lt(a1599, a1600) {
        Ok(f205(v0))
    } else {
        let a1601 = v0;
        let a1602 = 2048u64;
        if nat_lt(a1601, a1602) {
            Ok(f207(f205({
                let a1606 = 192u64;
                let a1607 = {
                    let a1603 = v0;
                    let a1604 = 64u64;
                    let a1605 = 0u64;
                    nat_quot(a1603, a1604, a1605)
                };
                nat_add(a1606, a1607)?
            }), f205({
                let a1611 = 128u64;
                let a1612 = {
                    let a1608 = v0;
                    let a1609 = 64u64;
                    let a1610 = 0u64;
                    nat_rem(a1608, a1609, a1610)
                };
                nat_add(a1611, a1612)?
            })))
        } else {
            Ok(f207(f205({
                let a1616 = 224u64;
                let a1617 = {
                    let a1613 = v0;
                    let a1614 = 4096u64;
                    let a1615 = 0u64;
                    nat_quot(a1613, a1614, a1615)
                };
                nat_add(a1616, a1617)?
            }), f207(f205({
                let a1624 = 128u64;
                let a1625 = {
                    let a1621 = {
                        let a1618 = v0;
                        let a1619 = 64u64;
                        let a1620 = 0u64;
                        nat_quot(a1618, a1619, a1620)
                    };
                    let a1622 = 64u64;
                    let a1623 = 0u64;
                    nat_rem(a1621, a1622, a1623)
                };
                nat_add(a1624, a1625)?
            }), f205({
                let a1629 = 128u64;
                let a1630 = {
                    let a1626 = v0;
                    let a1627 = 64u64;
                    let a1628 = 0u64;
                    nat_rem(a1626, a1627, a1628)
                };
                nat_add(a1629, a1630)?
            }))))
        }
    }
}

pub fn f233(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1631 = v2;
    let a1632 = v1;
    if nat_lt(a1631, a1632) {
        let v3: u64 = f170(v0, v2)?;
        let a1633 = v3;
        let a1634 = 32u64;
        if nat_lt(a1633, a1634) {
            Ok(None::<u64>)
        } else {
            let a1635 = v3;
            let a1636 = 34u64;
            if nat_eq(a1635, a1636) {
                Ok(None::<u64>)
            } else {
                let a1637 = v3;
                let a1638 = 92u64;
                if nat_eq(a1637, a1638) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1639 = v2;
                        let a1640 = 1u64;
                        nat_add(a1639, a1640)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f234(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1642 = {
        let (h1641, _) = v0;
        h1641
    };
    match m1642 {
        None => {
            (Some(v1), {
                let (_, h1643) = v0;
                h1643
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1644 = v2;
                let a1645 = v1;
                append_bytes(a1644, a1645)
            }, {
                let (_, h1646) = v0;
                h1646
            }))
        }
    }
}

pub fn f235(v0: u64) -> bool {
    if f138(v0) {
        true
    } else {
        let a1647 = 47u64;
        let a1648 = v0;
        if if nat_lt(a1647, a1648) {
            let a1649 = v0;
            let a1650 = 58u64;
            nat_lt(a1649, a1650)
        } else {
            false
        } {
            true
        } else {
            let a1651 = v0;
            let a1652 = 46u64;
            if nat_eq(a1651, a1652) {
                true
            } else {
                let a1653 = v0;
                let a1654 = 39u64;
                if nat_eq(a1653, a1654) {
                    true
                } else {
                    let a1655 = v0;
                    let a1656 = 47u64;
                    if nat_eq(a1655, a1656) {
                        true
                    } else {
                        let a1657 = v0;
                        let a1658 = 45u64;
                        nat_eq(a1657, a1658)
                    }
                }
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m1659 = f50(v2);
        match m1659 {
            None => {
                false
            }
            Some(v3) => {
                f36(v3, v0)
            }
        }
    }
}

pub fn f237(v0: Fn25, p1663: List<Adt0>, p1664: List<Adt0>) -> List<Adt0> {
    let m1662 = (p1663, p1664);
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let (v1, v2) = m1662;
                let m1660 = v2;
                match m1660.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c1661: Fn25 = v0.clone();
                            c1661.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f239(v0: Adt16) -> Option<Adt16> {
    let m1666 = f251(match v0.clone() {
        Adt16::C0(h1665, _) => {
            (*h1665).clone()
        }
    });
    match m1666 {
        None => {
            None::<Adt16>
        }
        Some(v1) => {
            Some(Adt16::C0(std::rc::Rc::new(match v1.clone() {
                Adt18::C0(_, h1667) => {
                    (*h1667).clone()
                }
            }), List::cons(match v1 {
                Adt18::C0(h1668, _) => {
                    (*h1668).clone()
                }
            }, match v0 {
                Adt16::C0(_, h1669) => {
                    h1669
                }
            })))
        }
    }
}

pub fn f240(v0: Bytes, v1: u64, v2: Adt17) -> R<Option<Adt17>> {
    let v3: u64 = match v2.clone() {
        Adt17::C0(h1670, _) => {
            h1670
        }
    };
    let a1671 = v3;
    let a1672 = v1;
    if nat_lt(a1671, a1672) {
        let v4: u64 = f170(v0.clone(), v3)?;
        let a1673 = v4;
        let a1674 = 32u64;
        if if nat_lt(a1673, a1674) {
            true
        } else {
            let a1675 = v4;
            let a1676 = 34u64;
            if nat_eq(a1675, a1676) {
                true
            } else {
                let a1677 = v4;
                let a1678 = 92u64;
                nat_eq(a1677, a1678)
            }
        } {
            Ok(Some(Adt17::C0({
                let a1679 = v3;
                let a1680 = 1u64;
                nat_add(a1679, a1680)?
            }, List::cons(f252(v4), match v2 {
                Adt17::C0(_, h1681) => {
                    h1681
                }
            }))))
        } else {
            let v5: u64 = f172(v0.clone(), v1, v3)?;
            Ok(Some(Adt17::C0(v5, List::cons(f47(v0, v3, {
                let a1682 = v5;
                let a1683 = v3;
                nat_sub(a1682, a1683)
            }), match v2 {
                Adt17::C0(_, h1684) => {
                    h1684
                }
            }))))
        }
    } else {
        Ok(None::<Adt17>)
    }
}

pub fn f241(v0: List<Adt3>, v1: u64, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    let m1686 = match v3.clone() {
        Adt15::C0(_, _, _, h1685) => {
            h1685
        }
    };
    match m1686 {
        None => {
            if match v3.clone() {
                Adt15::C0(_, _, h1687, _) => {
                    h1687
                }
            } {
                let v5: u64 = match v3.clone() {
                    Adt15::C0(_, h1688, _, _) => {
                        h1688
                    }
                };
                let a1689 = f75(v0.clone(), v5);
                let a1690 = 3u64;
                if nat_eq(a1689, a1690) {
                    Ok(None::<Adt15>)
                } else {
                    let v6: Adt12 = f103(v0.clone(), v1, v5, v2)?;
                    let m1692 = match v6.clone() {
                        Adt12::C0(_, _, h1691) => {
                            h1691
                        }
                    };
                    match m1692 {
                        None => {
                            let a1696 = f75(v0, match v6.clone() {
                                Adt12::C0(_, h1695, _) => {
                                    h1695
                                }
                            });
                            let a1697 = 6u64;
                            if nat_eq(a1696, a1697) {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1698, _, _) => {
                                        (*h1698).clone()
                                    }
                                }, match v3 {
                                    Adt15::C0(h1699, _, _, _) => {
                                        h1699
                                    }
                                }), {
                                    let a1701 = match v6 {
                                        Adt12::C0(_, h1700, _) => {
                                            h1700
                                        }
                                    };
                                    let a1702 = 1u64;
                                    nat_add(a1701, a1702)?
                                }, true, None::<Adt1>)))
                            } else {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1703, _, _) => {
                                        (*h1703).clone()
                                    }
                                }, match v3 {
                                    Adt15::C0(h1704, _, _, _) => {
                                        h1704
                                    }
                                }), match v6 {
                                    Adt12::C0(_, h1705, _) => {
                                        h1705
                                    }
                                }, false, None::<Adt1>)))
                            }
                        }
                        Some(_) => {
                            Ok(Some(Adt15::C0(match v3 {
                                Adt15::C0(h1693, _, _, _) => {
                                    h1693
                                }
                            }, v5, false, match v6 {
                                Adt12::C0(_, _, h1694) => {
                                    h1694
                                }
                            })))
                        }
                    }
                }
            } else {
                Ok(None::<Adt15>)
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f242(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h1706, _, _, _, _, _) => {
            h1706
        }
    };
    let a1708 = 256u64;
    let a1709 = match v2.clone() {
        Adt13::C0(_, h1707, _, _, _, _) => {
            h1707
        }
    };
    if nat_lt(a1708, a1709) {
        Ok(f253(v2, f111(v0, v1, v3), f150(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f134(v0.clone(), v3);
        let a1710 = v4;
        let a1711 = 99u64;
        if nat_eq(a1710, a1711) {
            Ok(f253(v2, v1, f150(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a1712 = v4;
            let a1713 = 11u64;
            if nat_eq(a1712, a1713) {
                Ok(f254(v2, {
                    let a1714 = v3;
                    let a1715 = 1u64;
                    nat_add(a1714, a1715)?
                }, Adt0::C2(f137(v0, v3))))
            } else {
                let a1716 = v4;
                let a1717 = 2u64;
                if nat_eq(a1716, a1717) {
                    f255(v2, {
                        let a1718 = v3;
                        let a1719 = 1u64;
                        nat_add(a1718, a1719)?
                    }, Adt14::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a1720 = v4;
                    let a1721 = 0u64;
                    if nat_eq(a1720, a1721) {
                        f255(v2, {
                            let a1722 = v3;
                            let a1723 = 1u64;
                            nat_add(a1722, a1723)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a1724 = v4;
                        let a1725 = 12u64;
                        if nat_eq(a1724, a1725) {
                            f256(v0.clone(), v1, v2, f111(v0, v1, v3))
                        } else {
                            Ok(f253(v2, f111(v0.clone(), v1, v3), f188(f150(Str::lit("expected a value, found ")), f100(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f243(v0: List<Adt3>, v1: Adt13) -> R<Adt13> {
    let a1727 = f134(v0, match v1.clone() {
        Adt13::C0(h1726, _, _, _, _, _) => {
            h1726
        }
    });
    let a1728 = 3u64;
    if nat_eq(a1727, a1728) {
        f257(v1)
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1729, _, _, _, _, _) => {
                h1729
            }
        }, match v1.clone() {
            Adt13::C0(_, h1730, _, _, _, _) => {
                h1730
            }
        }, 0u64, match v1 {
            Adt13::C0(_, _, _, h1731, _, _) => {
                h1731
            }
        }, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    }
}

pub fn f244(v0: List<Adt3>, v1: u64, v2: Adt13) -> R<Adt13> {
    let m1733 = match v2.clone() {
        Adt13::C0(_, _, _, h1732, _, _) => {
            h1732
        }
    };
    match m1733.uncons() {
        None => {
            Ok(f253(v2, 0u64, f150(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m1734 = v3;
            match m1734 {
                Adt14::C0(_) => {
                    Ok(f253(v2, 0u64, f150(Str::lit("no object is open"))))
                }
                Adt14::C1(_, _, v8) => {
                    let a1736 = f134(v0.clone(), match v2.clone() {
                        Adt13::C0(h1735, _, _, _, _, _) => {
                            h1735
                        }
                    });
                    let a1737 = 1u64;
                    if nat_eq(a1736, a1737) {
                        f257(v2)
                    } else {
                        f258(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f245(v0: List<Adt3>, v1: Adt13) -> R<Adt13> {
    let m1739 = match v1.clone() {
        Adt13::C0(_, _, _, h1738, _, _) => {
            h1738
        }
    };
    match m1739.uncons() {
        None => {
            Ok(Adt13::C0(match v1.clone() {
                Adt13::C0(h1740, _, _, _, _, _) => {
                    h1740
                }
            }, match v1.clone() {
                Adt13::C0(_, h1741, _, _, _, _) => {
                    h1741
                }
            }, 9u64, List::<Adt14>::nil(), std::rc::Rc::new(match v1 {
                Adt13::C0(_, _, _, _, h1742, _) => {
                    (*h1742).clone()
                }
            }), None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m1743 = v2;
            match m1743 {
                Adt14::C0(v4) => {
                    f259(v0, v1.clone(), List::cons(Adt14::C0(List::cons(match v1 {
                        Adt13::C0(_, _, _, _, h1744, _) => {
                            (*h1744).clone()
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt14::C1(v5, v6, v7) => {
                    if f112(v7.clone(), v5.clone()) {
                        Ok(f253(v1, v6, f188(f188(f150(Str::lit("the member `")), v5), f150(Str::lit("` is written twice")))))
                    } else {
                        f259(v0, v1.clone(), List::cons(Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt13::C0(_, _, _, _, h1745, _) => {
                                (*h1745).clone()
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f246(v0: List<Adt3>, v1: u64, v2: Adt13, v3: u64, v4: Bytes) -> R<Adt13> {
    let a1747 = f134(v0.clone(), match v2.clone() {
        Adt13::C0(h1746, _, _, _, _, _) => {
            h1746
        }
    });
    let a1748 = v3;
    if nat_eq(a1747, a1748) {
        f257(v2)
    } else {
        Ok(f253(v2.clone(), f111(v0, v1, match v2 {
            Adt13::C0(h1749, _, _, _, _, _) => {
                h1749
            }
        }), v4))
    }
}

pub fn f247(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f199({
        let (_, h1750) = {
            let (h1751, _) = f260(Fn26::F261, {
                let a1753 = {
                    let a1752 = v0.clone();
                    length_list(a1752)
                };
                let a1754 = 1u64;
                nat_add(a1753, a1754)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h1751
        };
        h1750
    }))
}

pub fn f248(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1755 = f41(v0, v1);
    match m1755 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f262(v2)
        }
    }
}

pub fn f249(v0: u64, v1: u64) -> Bytes {
    f250({
        let a1756 = v0;
        let a1757 = v1;
        let a1758 = 0u64;
        nat_quot(a1756, a1757, a1758)
    })
}

pub fn f250(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1759 = v0;
        let a1760 = 16u64;
        let a1761 = 0u64;
        nat_rem(a1759, a1760, a1761)
    }, 1u64)
}

pub fn f251(v0: Adt0) -> Option<Adt18> {
    let m1762 = v0;
    match m1762 {
        Adt0::C0(_) => {
            None::<Adt18>
        }
        Adt0::C1(_, _) => {
            None::<Adt18>
        }
        Adt0::C2(_) => {
            None::<Adt18>
        }
        Adt0::C3(_) => {
            None::<Adt18>
        }
        Adt0::C4(_) => {
            None::<Adt18>
        }
        Adt0::C5 => {
            None::<Adt18>
        }
        Adt0::C6(r1763, r1764) => {
            let v1 = (*r1763).clone();
            let v2 = (*r1764).clone();
            Some(Adt18::C0(std::rc::Rc::new(v1), std::rc::Rc::new(v2)))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt18>
        }
    }
}

pub fn f252(v0: u64) -> Bytes {
    let a1765 = v0;
    let a1766 = 34u64;
    if nat_eq(a1765, a1766) {
        f45(Str::lit("\\\""))
    } else {
        let a1767 = v0;
        let a1768 = 92u64;
        if nat_eq(a1767, a1768) {
            f45(Str::lit("\\\\"))
        } else {
            let a1769 = v0;
            let a1770 = 8u64;
            if nat_eq(a1769, a1770) {
                f45(Str::lit("\\b"))
            } else {
                let a1771 = v0;
                let a1772 = 12u64;
                if nat_eq(a1771, a1772) {
                    f45(Str::lit("\\f"))
                } else {
                    let a1773 = v0;
                    let a1774 = 10u64;
                    if nat_eq(a1773, a1774) {
                        f45(Str::lit("\\n"))
                    } else {
                        let a1775 = v0;
                        let a1776 = 13u64;
                        if nat_eq(a1775, a1776) {
                            f45(Str::lit("\\r"))
                        } else {
                            let a1777 = v0;
                            let a1778 = 9u64;
                            if nat_eq(a1777, a1778) {
                                f45(Str::lit("\\t"))
                            } else {
                                f207(f45(Str::lit("\\u00")), f263(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f253(v0: Adt13, v1: u64, v2: Bytes) -> Adt13 {
    Adt13::C0(match v0.clone() {
        Adt13::C0(h1779, _, _, _, _, _) => {
            h1779
        }
    }, match v0.clone() {
        Adt13::C0(_, h1780, _, _, _, _) => {
            h1780
        }
    }, 9u64, match v0.clone() {
        Adt13::C0(_, _, _, h1781, _, _) => {
            h1781
        }
    }, std::rc::Rc::new(match v0 {
        Adt13::C0(_, _, _, _, h1782, _) => {
            (*h1782).clone()
        }
    }), Some(f264(v1, v2)))
}

pub fn f254(v0: Adt13, v1: u64, v2: Adt0) -> Adt13 {
    Adt13::C0(v1, match v0.clone() {
        Adt13::C0(_, h1783, _, _, _, _) => {
            h1783
        }
    }, 3u64, match v0 {
        Adt13::C0(_, _, _, h1784, _, _) => {
            h1784
        }
    }, std::rc::Rc::new(v2), None::<Adt1>)
}

pub fn f255(v0: Adt13, v1: u64, v2: Adt14, v3: u64) -> R<Adt13> {
    Ok(Adt13::C0(v1, {
        let a1786 = match v0.clone() {
            Adt13::C0(_, h1785, _, _, _, _) => {
                h1785
            }
        };
        let a1787 = 1u64;
        nat_add(a1786, a1787)?
    }, v3, List::cons(v2, match v0 {
        Adt13::C0(_, _, _, h1788, _, _) => {
            h1788
        }
    }), std::rc::Rc::new(Adt0::C5), None::<Adt1>))
}

pub fn f256(v0: List<Adt3>, _: u64, v2: Adt13, v3: u64) -> R<Adt13> {
    let v4: u64 = match v2.clone() {
        Adt13::C0(h1789, _, _, _, _, _) => {
            h1789
        }
    };
    let v5: Bytes = f137(v0.clone(), v4);
    let a1790 = v5.clone();
    let a1791 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1790, a1791) {
        Ok(f254(v2, {
            let a1792 = v4;
            let a1793 = 1u64;
            nat_add(a1792, a1793)?
        }, Adt0::C0(true)))
    } else {
        let a1794 = v5.clone();
        let a1795 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a1794, a1795) {
            Ok(f254(v2, {
                let a1796 = v4;
                let a1797 = 1u64;
                nat_add(a1796, a1797)?
            }, Adt0::C0(false)))
        } else {
            if f265(v5.clone())? {
                let m1798 = f266(v5.clone())?;
                match m1798 {
                    None => {
                        Ok(f253(v2, v3, f188(f188(f150(Str::lit("`")), v5), f150(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f254(v2, {
                            let a1799 = v4;
                            let a1800 = 1u64;
                            nat_add(a1799, a1800)?
                        }, v6))
                    }
                }
            } else {
                if f86(v5.clone())? {
                    let a1803 = f134(v0, {
                        let a1801 = v4;
                        let a1802 = 1u64;
                        nat_add(a1801, a1802)?
                    });
                    let a1804 = 0u64;
                    if nat_eq(a1803, a1804) {
                        f255(v2, {
                            let a1805 = v4;
                            let a1806 = 2u64;
                            nat_add(a1805, a1806)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((f150(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f254(v2, {
                            let a1807 = v4;
                            let a1808 = 1u64;
                            nat_add(a1807, a1808)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f253(v2, v3, f188(f188(f150(Str::lit("`")), v5), f150(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f257(v0: Adt13) -> R<Adt13> {
    let m1810 = match v0.clone() {
        Adt13::C0(_, _, _, h1809, _, _) => {
            h1809
        }
    };
    match m1810.uncons() {
        None => {
            Ok(f253(v0, 0u64, f150(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m1811 = v1;
            match m1811 {
                Adt14::C0(v3) => {
                    Ok(Adt13::C0({
                        let a1813 = match v0.clone() {
                            Adt13::C0(h1812, _, _, _, _, _) => {
                                h1812
                            }
                        };
                        let a1814 = 1u64;
                        nat_add(a1813, a1814)?
                    }, {
                        let a1816 = match v0 {
                            Adt13::C0(_, h1815, _, _, _, _) => {
                                h1815
                            }
                        };
                        let a1817 = 1u64;
                        nat_sub(a1816, a1817)
                    }, 3u64, v2, std::rc::Rc::new(Adt0::C3(std::rc::Rc::new(f135(v3)))), None::<Adt1>))
                }
                Adt14::C1(_, _, v6) => {
                    Ok(Adt13::C0({
                        let a1819 = match v0.clone() {
                            Adt13::C0(h1818, _, _, _, _, _) => {
                                h1818
                            }
                        };
                        let a1820 = 1u64;
                        nat_add(a1819, a1820)?
                    }, {
                        let a1822 = match v0 {
                            Adt13::C0(_, h1821, _, _, _, _) => {
                                h1821
                            }
                        };
                        let a1823 = 1u64;
                        nat_sub(a1822, a1823)
                    }, 3u64, v2, std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(f162(f163(v6)?)))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f258(v0: List<Adt3>, v1: u64, v2: Adt13, v3: List<(Bytes, Adt0)>, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v2.clone() {
        Adt13::C0(h1824, _, _, _, _, _) => {
            h1824
        }
    };
    let v6: u64 = f134(v0.clone(), v5);
    let v7: Bytes = f137(v0.clone(), v5);
    let a1825 = v6;
    let a1826 = 11u64;
    if if nat_eq(a1825, a1826) {
        true
    } else {
        let a1827 = v6;
        let a1828 = 12u64;
        if nat_eq(a1827, a1828) {
            f86(v7.clone())?
        } else {
            false
        }
    } {
        let a1831 = f134(v0.clone(), {
            let a1829 = v5;
            let a1830 = 1u64;
            nat_add(a1829, a1830)?
        });
        let a1832 = 7u64;
        if nat_eq(a1831, a1832) {
            Ok(Adt13::C0({
                let a1833 = v5;
                let a1834 = 2u64;
                nat_add(a1833, a1834)?
            }, match v2 {
                Adt13::C0(_, h1835, _, _, _, _) => {
                    h1835
                }
            }, 0u64, List::cons(Adt14::C1(v7, f111(v0, v1, v5), v3), v4), std::rc::Rc::new(Adt0::C5), None::<Adt1>))
        } else {
            Ok(f253(v2, f111(v0, v1, {
                let a1836 = v5;
                let a1837 = 1u64;
                nat_add(a1836, a1837)?
            }), f150(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f253(v2, f111(v0, v1, v5), f150(Str::lit("expected a member name"))))
    }
}

pub fn f259(v0: List<Adt3>, v1: Adt13, v2: List<Adt14>, v3: u64, v4: u64) -> R<Adt13> {
    let a1839 = f134(v0, match v1.clone() {
        Adt13::C0(h1838, _, _, _, _, _) => {
            h1838
        }
    });
    let a1840 = 6u64;
    if nat_eq(a1839, a1840) {
        Ok(Adt13::C0({
            let a1842 = match v1.clone() {
                Adt13::C0(h1841, _, _, _, _, _) => {
                    h1841
                }
            };
            let a1843 = 1u64;
            nat_add(a1842, a1843)?
        }, match v1 {
            Adt13::C0(_, h1844, _, _, _, _) => {
                h1844
            }
        }, v3, v2, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1845, _, _, _, _, _) => {
                h1845
            }
        }, match v1 {
            Adt13::C0(_, h1846, _, _, _, _) => {
                h1846
            }
        }, v4, v2, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    }
}

pub fn f260(v0: Fn26, p1851: u64, p1852: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m1850 = (p1851, p1852);
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let (v1, v2) = m1850;
                let m1847 = v1;
                if m1847 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1847 - 1;
                    let m1849 = {
                        let c1848: Fn26 = v0.clone();
                        c1848.apply(v2.clone())?
                    };
                    match m1849 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f267(v0)
}

pub fn f262(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1897 = {
            let a1873 = {
                let a1861 = {
                    let a1855 = {
                        let a1853 = v1;
                        let a1854 = 128u8;
                        fixed_u8::bit_and(a1853, a1854)
                    };
                    let a1856 = 128u8;
                    if equal(a1855, a1856) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1862 = {
                    let a1859 = {
                        let a1857 = v1;
                        let a1858 = 64u8;
                        fixed_u8::bit_and(a1857, a1858)
                    };
                    let a1860 = 64u8;
                    if equal(a1859, a1860) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1861, a1862)?
            };
            let a1874 = {
                let a1871 = {
                    let a1865 = {
                        let a1863 = v1;
                        let a1864 = 32u8;
                        fixed_u8::bit_and(a1863, a1864)
                    };
                    let a1866 = 32u8;
                    if equal(a1865, a1866) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1872 = {
                    let a1869 = {
                        let a1867 = v1;
                        let a1868 = 16u8;
                        fixed_u8::bit_and(a1867, a1868)
                    };
                    let a1870 = 16u8;
                    if equal(a1869, a1870) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1871, a1872)?
            };
            nat_add(a1873, a1874)?
        };
        let a1898 = {
            let a1895 = {
                let a1883 = {
                    let a1877 = {
                        let a1875 = v1;
                        let a1876 = 8u8;
                        fixed_u8::bit_and(a1875, a1876)
                    };
                    let a1878 = 8u8;
                    if equal(a1877, a1878) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1884 = {
                    let a1881 = {
                        let a1879 = v1;
                        let a1880 = 4u8;
                        fixed_u8::bit_and(a1879, a1880)
                    };
                    let a1882 = 4u8;
                    if equal(a1881, a1882) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1883, a1884)?
            };
            let a1896 = {
                let a1893 = {
                    let a1887 = {
                        let a1885 = v1;
                        let a1886 = 2u8;
                        fixed_u8::bit_and(a1885, a1886)
                    };
                    let a1888 = 2u8;
                    if equal(a1887, a1888) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1894 = {
                    let a1891 = {
                        let a1889 = v1;
                        let a1890 = 1u8;
                        fixed_u8::bit_and(a1889, a1890)
                    };
                    let a1892 = 1u8;
                    if equal(a1891, a1892) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1893, a1894)?
            };
            nat_add(a1895, a1896)?
        };
        nat_add(a1897, a1898)?
    };
    let a1899 = v2;
    let a1900 = 48u64;
    if nat_lt(a1899, a1900) {
        Ok(None::<u64>)
    } else {
        let a1901 = v2;
        let a1902 = 57u64;
        if nat_le(a1901, a1902) {
            Ok(Some({
                let a1903 = v2;
                let a1904 = 48u64;
                nat_sub(a1903, a1904)
            }))
        } else {
            let a1905 = v2;
            let a1906 = 65u64;
            if nat_lt(a1905, a1906) {
                Ok(None::<u64>)
            } else {
                let a1907 = v2;
                let a1908 = 70u64;
                if nat_le(a1907, a1908) {
                    Ok(Some({
                        let a1909 = v2;
                        let a1910 = 55u64;
                        nat_sub(a1909, a1910)
                    }))
                } else {
                    let a1911 = v2;
                    let a1912 = 97u64;
                    if nat_lt(a1911, a1912) {
                        Ok(None::<u64>)
                    } else {
                        let a1913 = v2;
                        let a1914 = 102u64;
                        if nat_le(a1913, a1914) {
                            Ok(Some({
                                let a1915 = v2;
                                let a1916 = 87u64;
                                nat_sub(a1915, a1916)
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f263(v0: u64) -> Bytes {
    f207(f249(v0, 16u64), f250(v0))
}

pub fn f264(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f265(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1917 = f139(v0.clone(), 0u64)?;
        let a1918 = 45u64;
        if nat_eq(a1917, a1918) {
            1u64
        } else {
            0u64
        }
    };
    let a1920 = v1;
    let a1921 = {
        let a1919 = v0.clone();
        length_bytes(a1919)
    };
    if nat_lt(a1920, a1921) {
        let a1923 = f142(v0.clone(), v1)?;
        let a1924 = {
            let a1922 = v0;
            length_bytes(a1922)
        };
        Ok(nat_eq(a1923, a1924))
    } else {
        Ok(false)
    }
}

pub fn f266(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a1925 = f139(v0.clone(), 0u64)?;
        let a1926 = 45u64;
        nat_eq(a1925, a1926)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f268(v0.clone(), v2)?;
    let v4: Bytes = f47(v0.clone(), v3, {
        let a1928 = {
            let a1927 = v0;
            length_bytes(a1927)
        };
        let a1929 = v3;
        nat_sub(a1928, a1929)
    });
    let a1931 = {
        let a1930 = v4.clone();
        length_bytes(a1930)
    };
    let a1932 = 0u64;
    if nat_eq(a1931, a1932) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a1934 = 19u64;
        let a1935 = {
            let a1933 = v4.clone();
            length_bytes(a1933)
        };
        if nat_lt(a1934, a1935) {
            Ok(None::<Adt0>)
        } else {
            let m1936 = f269(v4)?;
            match m1936 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f270(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f267(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h1937, _) = v0;
        h1937
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h1938) = v0;
        h1938
    };
    let m1939 = v1;
    match m1939.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m1940 = v4;
            match m1940.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f271(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: u64) -> R<u64> {
    let (h1941, _) = f70(Fn7::F272(v0.clone()), {
        let a1943 = {
            let a1942 = v0;
            length_bytes(a1942)
        };
        let a1944 = 1u64;
        nat_add(a1943, a1944)?
    }, v1)?;
    Ok(h1941)
}

pub fn f269(v0: Bytes) -> R<Option<u64>> {
    if f273(v0.clone())? {
        Ok(Some({
            let (_, h1945) = {
                let (h1946, _) = f274(Fn27::F275(v0.clone()), {
                    let a1947 = v0;
                    length_bytes(a1947)
                }, (0u64, 0u64))?;
                h1946
            };
            h1945
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f270(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1948 = 0u64;
        let a1949 = v1;
        if nat_lt(a1948, a1949) {
            let a1950 = v1;
            let a1951 = 9223372036854775808u64;
            nat_le(a1950, a1951)
        } else {
            false
        }
    } else {
        let a1952 = v1;
        let a1953 = 9223372036854775807u64;
        nat_le(a1952, a1953)
    }
}

pub fn f271(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a1958 = {
            let a1956 = {
                let a1954 = v0.clone();
                length_list(a1954)
            };
            let a1957 = {
                let a1955 = v1.clone();
                length_list(a1955)
            };
            nat_add(a1956, a1957)?
        };
        let a1959 = 1u64;
        nat_add(a1958, a1959)?
    };
    Ok(f198({
        let (_, h1960) = {
            let (_, h1961) = {
                let (h1962, _) = f276(Fn28::F277, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h1962
            };
            h1961
        };
        h1960
    }))
}

pub fn f272(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f278(v0, v1)
}

pub fn f273(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1963 = v0.clone();
        length_bytes(a1963)
    };
    let a1964 = 0u64;
    let a1965 = v1;
    if nat_lt(a1964, a1965) {
        let a1966 = f279(v0.clone())?;
        let a1967 = v1;
        if nat_eq(a1966, a1967) {
            let a1968 = 1u64;
            let a1969 = v1;
            if nat_lt(a1968, a1969) {
                let m1972 = {
                    let a1970 = v0;
                    let a1971 = 0u64;
                    index_bytes(a1970, a1971)
                };
                match m1972 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1973 = v2;
                        let a1974 = 48u8;
                        Ok(!equal(a1973, a1974))
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

pub fn f274(v0: Fn27, p1979: u64, p1980: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v1, v2) = m1978;
                let m1975 = v1;
                if m1975 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1975 - 1;
                    let m1977 = {
                        let c1976: Fn27 = v0.clone();
                        c1976.apply(v2)?
                    };
                    match m1977 {
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
                Ok(next) => { m1978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f280(v0, v1)
}

pub fn f276(v0: Fn28, p1985: u64, p1986: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m1984 = (p1985, p1986);
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let (v1, v2) = m1984;
                let m1981 = v1;
                if m1981 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m1981 - 1;
                    let m1983 = {
                        let c1982: Fn28 = v0.clone();
                        c1982.apply(v2.clone())
                    };
                    match m1983 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f281(v0)
}

pub fn f278(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1987 = f139(v0, v1)?;
    let a1988 = 48u64;
    if nat_eq(a1987, a1988) {
        Ok(Some({
            let a1989 = v1;
            let a1990 = 1u64;
            nat_add(a1989, a1990)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f279(v0: Bytes) -> R<u64> {
    let (h1991, _) = f70(Fn7::F282(v0.clone()), {
        let a1992 = v0;
        length_bytes(a1992)
    }, 0u64)?;
    Ok(h1991)
}

pub fn f280(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1996 = {
        let a1994 = v0;
        let a1995 = {
            let (h1993, _) = v1;
            h1993
        };
        index_bytes(a1994, a1995)
    };
    match m1996 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1998 = {
                    let (h1997, _) = v1;
                    h1997
                };
                let a1999 = 1u64;
                nat_add(a1998, a1999)?
            }, {
                let a2003 = {
                    let a2001 = {
                        let (_, h2000) = v1;
                        h2000
                    };
                    let a2002 = 10u64;
                    nat_mul(a2001, a2002)?
                };
                let a2004 = f283(v2)?;
                nat_add(a2003, a2004)?
            })))
        }
    }
}

pub fn f281(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h2005, _) = v0.clone();
        h2005
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h2006, _) = {
            let (_, h2007) = v0.clone();
            h2007
        };
        h2006
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h2008) = {
            let (_, h2009) = v0;
            h2009
        };
        h2008
    };
    let m2010 = v1.clone();
    match m2010.uncons() {
        None => {
            let m2011 = v2;
            match m2011.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2012 = v2.clone();
            match m2012.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h2013, _) = v8.clone();
                        h2013
                    }, {
                        let (h2014, _) = v6.clone();
                        h2014
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

pub fn f282(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2017 = {
        let a2015 = v0;
        let a2016 = v1;
        index_bytes(a2015, a2016)
    };
    match m2017 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f285(v2)? {
                Ok(Some({
                    let a2018 = v1;
                    let a2019 = 1u64;
                    nat_add(a2018, a2019)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f283(v0: u8) -> R<u64> {
    let a2066 = {
        let v1: u8 = v0;
        let a2064 = {
            let a2040 = {
                let a2028 = {
                    let a2022 = {
                        let a2020 = v1;
                        let a2021 = 128u8;
                        fixed_u8::bit_and(a2020, a2021)
                    };
                    let a2023 = 128u8;
                    if equal(a2022, a2023) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2029 = {
                    let a2026 = {
                        let a2024 = v1;
                        let a2025 = 64u8;
                        fixed_u8::bit_and(a2024, a2025)
                    };
                    let a2027 = 64u8;
                    if equal(a2026, a2027) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2028, a2029)?
            };
            let a2041 = {
                let a2038 = {
                    let a2032 = {
                        let a2030 = v1;
                        let a2031 = 32u8;
                        fixed_u8::bit_and(a2030, a2031)
                    };
                    let a2033 = 32u8;
                    if equal(a2032, a2033) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2039 = {
                    let a2036 = {
                        let a2034 = v1;
                        let a2035 = 16u8;
                        fixed_u8::bit_and(a2034, a2035)
                    };
                    let a2037 = 16u8;
                    if equal(a2036, a2037) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2038, a2039)?
            };
            nat_add(a2040, a2041)?
        };
        let a2065 = {
            let a2062 = {
                let a2050 = {
                    let a2044 = {
                        let a2042 = v1;
                        let a2043 = 8u8;
                        fixed_u8::bit_and(a2042, a2043)
                    };
                    let a2045 = 8u8;
                    if equal(a2044, a2045) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2051 = {
                    let a2048 = {
                        let a2046 = v1;
                        let a2047 = 4u8;
                        fixed_u8::bit_and(a2046, a2047)
                    };
                    let a2049 = 4u8;
                    if equal(a2048, a2049) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2050, a2051)?
            };
            let a2063 = {
                let a2060 = {
                    let a2054 = {
                        let a2052 = v1;
                        let a2053 = 2u8;
                        fixed_u8::bit_and(a2052, a2053)
                    };
                    let a2055 = 2u8;
                    if equal(a2054, a2055) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2061 = {
                    let a2058 = {
                        let a2056 = v1;
                        let a2057 = 1u8;
                        fixed_u8::bit_and(a2056, a2057)
                    };
                    let a2059 = 1u8;
                    if equal(a2058, a2059) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2060, a2061)?
            };
            nat_add(a2062, a2063)?
        };
        nat_add(a2064, a2065)?
    };
    let a2067 = 48u64;
    Ok(nat_sub(a2066, a2067))
}

pub fn f284(v0: Bytes, v1: Bytes) -> bool {
    let a2072 = {
        let a2068 = v0;
        let a2069 = v1;
        compare_bytes(a2068, a2069)
    };
    let a2073 = {
        let a2070 = Bytes::lit(&[]);
        let a2071 = Bytes::lit(&[0x01]);
        compare_bytes(a2070, a2071)
    };
    equal(a2072, a2073)
}

pub fn f285(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2118 = {
            let a2094 = {
                let a2082 = {
                    let a2076 = {
                        let a2074 = v1;
                        let a2075 = 128u8;
                        fixed_u8::bit_and(a2074, a2075)
                    };
                    let a2077 = 128u8;
                    if equal(a2076, a2077) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2083 = {
                    let a2080 = {
                        let a2078 = v1;
                        let a2079 = 64u8;
                        fixed_u8::bit_and(a2078, a2079)
                    };
                    let a2081 = 64u8;
                    if equal(a2080, a2081) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2082, a2083)?
            };
            let a2095 = {
                let a2092 = {
                    let a2086 = {
                        let a2084 = v1;
                        let a2085 = 32u8;
                        fixed_u8::bit_and(a2084, a2085)
                    };
                    let a2087 = 32u8;
                    if equal(a2086, a2087) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2093 = {
                    let a2090 = {
                        let a2088 = v1;
                        let a2089 = 16u8;
                        fixed_u8::bit_and(a2088, a2089)
                    };
                    let a2091 = 16u8;
                    if equal(a2090, a2091) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2092, a2093)?
            };
            nat_add(a2094, a2095)?
        };
        let a2119 = {
            let a2116 = {
                let a2104 = {
                    let a2098 = {
                        let a2096 = v1;
                        let a2097 = 8u8;
                        fixed_u8::bit_and(a2096, a2097)
                    };
                    let a2099 = 8u8;
                    if equal(a2098, a2099) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2105 = {
                    let a2102 = {
                        let a2100 = v1;
                        let a2101 = 4u8;
                        fixed_u8::bit_and(a2100, a2101)
                    };
                    let a2103 = 4u8;
                    if equal(a2102, a2103) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2104, a2105)?
            };
            let a2117 = {
                let a2114 = {
                    let a2108 = {
                        let a2106 = v1;
                        let a2107 = 2u8;
                        fixed_u8::bit_and(a2106, a2107)
                    };
                    let a2109 = 2u8;
                    if equal(a2108, a2109) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2115 = {
                    let a2112 = {
                        let a2110 = v1;
                        let a2111 = 1u8;
                        fixed_u8::bit_and(a2110, a2111)
                    };
                    let a2113 = 1u8;
                    if equal(a2112, a2113) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2114, a2115)?
            };
            nat_add(a2116, a2117)?
        };
        nat_add(a2118, a2119)?
    };
    let a2120 = 48u64;
    let a2121 = v2;
    if nat_le(a2120, a2121) {
        let a2122 = v2;
        let a2123 = 57u64;
        Ok(nat_le(a2122, a2123))
    } else {
        Ok(false)
    }
}
