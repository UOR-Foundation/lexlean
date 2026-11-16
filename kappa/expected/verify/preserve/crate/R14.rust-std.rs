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
    C0(u64, bool, u64, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, bool, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, bool, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(bool, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, std::rc::Rc<Adt9>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Adt0> {
        match self {
            Fn0::F6(k0) => f6(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Adt1> {
        match self {
            Fn1::F8(k0) => f8(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn2::F13(k0, k1) => f13(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F18(u64, u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt3, p1: Adt2) -> R<Adt3> {
        match self {
            Fn3::F18(k0, k1, k2) => f18(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31,
    F58(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn4::F31 => Ok(f31(p0)),
            Fn4::F58(k0) => f58(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F33,
}

impl Fn5 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn5::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F36(Bytes, u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<(u64, Adt4)> {
        match self {
            Fn6::F36(k0, k1, k2) => f36(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F52(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn7::F52(k0) => f52(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F54,
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> Option<u64> {
        match self {
            Fn8::F54 => f54(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F56(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt7, p1: (Bytes, Bytes)) -> Adt7 {
        match self {
            Fn9::F56(k0, k1) => f56(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F61(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
        match self {
            Fn10::F61(k0, k1) => f61(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F63(std::rc::Rc<Adt2>),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt8, p1: Adt2) -> Adt8 {
        match self {
            Fn11::F63(k0) => f63((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F66,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt10) -> R<Adt10> {
        match self {
            Fn12::F66 => f66(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F72,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn13::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F75,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn14::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F91,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F91 => f91(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f2(v1)?))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2 = v0.clone();
        length_bytes(a2)
    };
    let a3 = 24u64;
    let a4 = v1;
    if if nat_le(a3, a4) {
        let a5 = f3(v0.clone(), 0u64, 8u64);
        let a6 = Bytes::lit(&[0x47, 0x47, 0x55, 0x46, 0x03, 0x00, 0x00, 0x00]);
        equal(a5, a6)
    } else {
        false
    } {
        let v2: u64 = f4(v0.clone(), 8u64, 8u64)?;
        let v3: u64 = f4(v0.clone(), 16u64, 8u64)?;
        let a7 = v3;
        let a8 = v1;
        if if nat_le(a7, a8) {
            let a9 = v2;
            let a10 = v1;
            nat_le(a9, a10)
        } else {
            false
        } {
            let v4: Adt0 = f5(Fn0::F6(v0.clone()), v3, Adt0::C0(24u64, true, 32u64, List::<(Bytes, Bytes)>::nil()))?;
            if match v4.clone() {
                Adt0::C0(_, h11, _, _) => {
                    h11
                }
            } {
                let v5: Adt1 = f7(Fn1::F8(v0.clone()), v2, Adt1::C0(match v4.clone() {
                    Adt0::C0(h12, _, _, _) => {
                        h12
                    }
                }, true, List::<Adt2>::nil()))?;
                if match v5.clone() {
                    Adt1::C0(_, h13, _) => {
                        h13
                    }
                } {
                    f9(v0, v4, v5)
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    let a14 = f10(Str::lit("sha256:"));
    let a15 = f11(v0)?;
    Ok(append_bytes(a14, a15))
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m19 = {
        let a16 = v0;
        let a17 = v1;
        let a18 = v2;
        slice_bytes(a16, a17, a18)
    };
    match m19 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f4(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h20) = f12(Fn2::F13(v1, v0), v2, (v2, 0u64))?;
    Ok(h20)
}

pub fn f5(v0: Fn0, p24: u64, p25: Adt0) -> R<Adt0> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m21 = v1;
                if m21 == 0 {
                    Err::<(u64, Adt0), R<Adt0>>(Ok(v2))
                } else {
                    let v3 = m21 - 1;
                    Ok::<(u64, Adt0), R<Adt0>>((v3, {
                        let c22: Fn0 = v0.clone();
                        c22.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: Adt0) -> R<Adt0> {
    f14(v0, v1)
}

pub fn f7(v0: Fn1, p29: u64, p30: Adt1) -> R<Adt1> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v1;
                if m26 == 0 {
                    Err::<(u64, Adt1), R<Adt1>>(Ok(v2))
                } else {
                    let v3 = m26 - 1;
                    Ok::<(u64, Adt1), R<Adt1>>((v3, {
                        let c27: Fn1 = v0.clone();
                        c27.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m28 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: Adt1) -> R<Adt1> {
    f15(v0, v1)
}

pub fn f9(v0: Bytes, v1: Adt0, v2: Adt1) -> R<Option<Bytes>> {
    let v3: u64 = match v1.clone() {
        Adt0::C0(_, _, h31, _) => {
            h31
        }
    };
    let v4: u64 = f16(match v2.clone() {
        Adt1::C0(h32, _, _) => {
            h32
        }
    }, v3)?;
    let v5: Adt3 = f17(Fn3::F18(v3, v4, v0.clone()), Adt3::C0(true, 0u64, Bytes::lit(&[])), match v2 {
        Adt1::C0(_, _, h33) => {
            h33
        }
    })?;
    if match v5.clone() {
        Adt3::C0(h34, _, _) => {
            h34
        }
    } {
        Ok(Some({
            let a41 = f3(v0, 0u64, 24u64);
            let a42 = {
                let a39 = f19(v3, 8u64)?;
                let a40 = {
                    let a37 = f20(match v1 {
                        Adt0::C0(_, _, _, h35) => {
                            h35
                        }
                    });
                    let a38 = match v5 {
                        Adt3::C0(_, _, h36) => {
                            h36
                        }
                    };
                    append_bytes(a37, a38)
                };
                append_bytes(a39, a40)
            };
            append_bytes(a41, a42)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f10(v0: Str) -> Bytes {
    let a43 = v0;
    utf8_encode(a43)
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    Ok(f21(f22(v0)?))
}

pub fn f12(v0: Fn2, p47: u64, p48: (u64, u64)) -> R<(u64, u64)> {
    let m46 = (p47, p48);
    {
        let mut m46 = m46;
        loop {
            let step = {
                let (v1, v2) = m46;
                let m44 = v1;
                if m44 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m44 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c45: Fn2 = v0.clone();
                        c45.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m46 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a50 = {
            let (h49, _) = v2;
            h49
        };
        let a51 = 1u64;
        nat_sub(a50, a51)
    };
    Ok((v3, {
        let a57 = {
            let a53 = {
                let (_, h52) = v2;
                h52
            };
            let a54 = 256u64;
            nat_mul(a53, a54)?
        };
        let a58 = f23(v1, {
            let a55 = v0;
            let a56 = v3;
            nat_add(a55, a56)?
        })?;
        nat_add(a57, a58)?
    }))
}

pub fn f14(v0: Bytes, v1: Adt0) -> R<Adt0> {
    if match v1.clone() {
        Adt0::C0(_, h59, _, _) => {
            h59
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt0::C0(h60, _, _, _) => {
                h60
            }
        };
        if f24(v0.clone(), v2, 8u64) {
            let v3: u64 = f4(v0.clone(), v2, 8u64)?;
            if if f24(v0.clone(), {
                let a61 = v2;
                let a62 = 8u64;
                nat_add(a61, a62)?
            }, v3) {
                f24(v0.clone(), {
                    let a65 = {
                        let a63 = v2;
                        let a64 = 8u64;
                        nat_add(a63, a64)?
                    };
                    let a66 = v3;
                    nat_add(a65, a66)?
                }, 4u64)
            } else {
                false
            } {
                f25(v0.clone(), v1, f3(v0.clone(), {
                    let a67 = v2;
                    let a68 = 8u64;
                    nat_add(a67, a68)?
                }, v3), f4(v0, {
                    let a71 = {
                        let a69 = v2;
                        let a70 = 8u64;
                        nat_add(a69, a70)?
                    };
                    let a72 = v3;
                    nat_add(a71, a72)?
                }, 4u64)?, {
                    let a77 = {
                        let a75 = {
                            let a73 = v2;
                            let a74 = 8u64;
                            nat_add(a73, a74)?
                        };
                        let a76 = v3;
                        nat_add(a75, a76)?
                    };
                    let a78 = 4u64;
                    nat_add(a77, a78)?
                })
            } else {
                Ok(f26(v1))
            }
        } else {
            Ok(f26(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f15(v0: Bytes, v1: Adt1) -> R<Adt1> {
    if match v1.clone() {
        Adt1::C0(_, h79, _) => {
            h79
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt1::C0(h80, _, _) => {
                h80
            }
        };
        if f24(v0.clone(), v2, 8u64) {
            let v3: u64 = f4(v0.clone(), v2, 8u64)?;
            if if f24(v0.clone(), {
                let a81 = v2;
                let a82 = 8u64;
                nat_add(a81, a82)?
            }, v3) {
                f24(v0.clone(), {
                    let a85 = {
                        let a83 = v2;
                        let a84 = 8u64;
                        nat_add(a83, a84)?
                    };
                    let a86 = v3;
                    nat_add(a85, a86)?
                }, 4u64)
            } else {
                false
            } {
                f27(v0.clone(), v1, f3(v0, {
                    let a87 = v2;
                    let a88 = 8u64;
                    nat_add(a87, a88)?
                }, v3), {
                    let a91 = {
                        let a89 = v2;
                        let a90 = 8u64;
                        nat_add(a89, a90)?
                    };
                    let a92 = v3;
                    nat_add(a91, a92)?
                })
            } else {
                Ok(f28(v1))
            }
        } else {
            Ok(f28(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a93 = v0;
        let a94 = v1;
        let a95 = 0u64;
        nat_rem(a93, a94, a95)
    };
    let a96 = v2;
    let a97 = 0u64;
    if nat_eq(a96, a97) {
        Ok(v0)
    } else {
        let a100 = v0;
        let a101 = {
            let a98 = v1;
            let a99 = v2;
            nat_sub(a98, a99)
        };
        nat_add(a100, a101)
    }
}

pub fn f17(v0: Fn3, p105: Adt3, p106: List<Adt2>) -> R<Adt3> {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m102 = v2;
                match m102.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt2>), R<Adt3>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt2>), R<Adt3>>(({
                            let c103: Fn3 = v0.clone();
                            c103.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: Bytes, v3: Adt3, v4: Adt2) -> R<Adt3> {
    f29(v2, v1, v0, v3, v4)
}

pub fn f19(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h107) = f30(Fn4::F31, v1, (v0, Bytes::lit(&[])))?;
    Ok(h107)
}

pub fn f20(v0: List<(Bytes, Bytes)>) -> Bytes {
    f32(Fn5::F33, Bytes::lit(&[]), v0)
}

pub fn f21(v0: Adt4) -> Bytes {
    let a128 = f34(match v0.clone() {
        Adt4::C0(h108, _, _, _, _, _, _, _) => {
            h108
        }
    });
    let a129 = {
        let a126 = f34(match v0.clone() {
            Adt4::C0(_, h109, _, _, _, _, _, _) => {
                h109
            }
        });
        let a127 = {
            let a124 = f34(match v0.clone() {
                Adt4::C0(_, _, h110, _, _, _, _, _) => {
                    h110
                }
            });
            let a125 = {
                let a122 = f34(match v0.clone() {
                    Adt4::C0(_, _, _, h111, _, _, _, _) => {
                        h111
                    }
                });
                let a123 = {
                    let a120 = f34(match v0.clone() {
                        Adt4::C0(_, _, _, _, h112, _, _, _) => {
                            h112
                        }
                    });
                    let a121 = {
                        let a118 = f34(match v0.clone() {
                            Adt4::C0(_, _, _, _, _, h113, _, _) => {
                                h113
                            }
                        });
                        let a119 = {
                            let a116 = f34(match v0.clone() {
                                Adt4::C0(_, _, _, _, _, _, h114, _) => {
                                    h114
                                }
                            });
                            let a117 = f34(match v0 {
                                Adt4::C0(_, _, _, _, _, _, _, h115) => {
                                    h115
                                }
                            });
                            append_bytes(a116, a117)
                        };
                        append_bytes(a118, a119)
                    };
                    append_bytes(a120, a121)
                };
                append_bytes(a122, a123)
            };
            append_bytes(a124, a125)
        };
        append_bytes(a126, a127)
    };
    append_bytes(a128, a129)
}

pub fn f22(v0: Bytes) -> R<Adt4> {
    let v1: u64 = {
        let a130 = v0.clone();
        length_bytes(a130)
    };
    let v2: u64 = {
        let a136 = {
            let a133 = {
                let a131 = v1;
                let a132 = 72u64;
                nat_add(a131, a132)?
            };
            let a134 = 64u64;
            let a135 = 0u64;
            nat_quot(a133, a134, a135)
        };
        let a137 = 64u64;
        nat_mul(a136, a137)?
    };
    let (_, h138) = f35(Fn6::F36(v0, v1, v2), {
        let a139 = v2;
        let a140 = 64u64;
        let a141 = 0u64;
        nat_quot(a139, a140, a141)
    }, (0u64, f37()))?;
    Ok(h138)
}

pub fn f23(v0: Bytes, v1: u64) -> R<u64> {
    let m144 = {
        let a142 = v0;
        let a143 = v1;
        index_bytes(a142, a143)
    };
    match m144 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a189 = {
                let a165 = {
                    let a153 = {
                        let a147 = {
                            let a145 = v3;
                            let a146 = 128u8;
                            fixed_u8::bit_and(a145, a146)
                        };
                        let a148 = 128u8;
                        if equal(a147, a148) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a154 = {
                        let a151 = {
                            let a149 = v3;
                            let a150 = 64u8;
                            fixed_u8::bit_and(a149, a150)
                        };
                        let a152 = 64u8;
                        if equal(a151, a152) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a153, a154)?
                };
                let a166 = {
                    let a163 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 32u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 32u8;
                        if equal(a157, a158) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a164 = {
                        let a161 = {
                            let a159 = v3;
                            let a160 = 16u8;
                            fixed_u8::bit_and(a159, a160)
                        };
                        let a162 = 16u8;
                        if equal(a161, a162) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a163, a164)?
                };
                nat_add(a165, a166)?
            };
            let a190 = {
                let a187 = {
                    let a175 = {
                        let a169 = {
                            let a167 = v3;
                            let a168 = 8u8;
                            fixed_u8::bit_and(a167, a168)
                        };
                        let a170 = 8u8;
                        if equal(a169, a170) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a176 = {
                        let a173 = {
                            let a171 = v3;
                            let a172 = 4u8;
                            fixed_u8::bit_and(a171, a172)
                        };
                        let a174 = 4u8;
                        if equal(a173, a174) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a175, a176)?
                };
                let a188 = {
                    let a185 = {
                        let a179 = {
                            let a177 = v3;
                            let a178 = 2u8;
                            fixed_u8::bit_and(a177, a178)
                        };
                        let a180 = 2u8;
                        if equal(a179, a180) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a186 = {
                        let a183 = {
                            let a181 = v3;
                            let a182 = 1u8;
                            fixed_u8::bit_and(a181, a182)
                        };
                        let a184 = 1u8;
                        if equal(a183, a184) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a185, a186)?
                };
                nat_add(a187, a188)?
            };
            nat_add(a189, a190)
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a191 = v0;
        length_bytes(a191)
    };
    let a192 = v1;
    let a193 = v3;
    if nat_le(a192, a193) {
        let a196 = v2;
        let a197 = {
            let a194 = v3;
            let a195 = v1;
            nat_sub(a194, a195)
        };
        nat_le(a196, a197)
    } else {
        false
    }
}

pub fn f25(v0: Bytes, v1: Adt0, v2: Bytes, v3: u64, v4: u64) -> R<Adt0> {
    let v5: (bool, u64) = f38(v0.clone(), v4, v3)?;
    let (h198, _) = v5;
    if h198 {
        let v6: u64 = {
            let a199 = v2.clone();
            let a200 = f10(Str::lit("general.alignment"));
            if if equal(a199, a200) {
                let a201 = v3;
                let a202 = 4u64;
                nat_eq(a201, a202)
            } else {
                false
            } {
                f4(v0.clone(), v4, 4u64)?
            } else {
                match v1.clone() {
                    Adt0::C0(_, _, h203, _) => {
                        h203
                    }
                }
            }
        };
        let a204 = 8u64;
        let a205 = v6;
        if if nat_le(a204, a205) {
            f39(v6)
        } else {
            false
        } {
            Ok(Adt0::C0({
                let (_, h206) = v5;
                h206
            }, true, v6, f40(v2.clone(), {
                let a212 = f41(v2)?;
                let a213 = {
                    let a210 = f19(v3, 4u64)?;
                    let a211 = f42(v0, v3, v4, {
                        let a208 = {
                            let (_, h207) = v5;
                            h207
                        };
                        let a209 = v4;
                        nat_sub(a208, a209)
                    })?;
                    append_bytes(a210, a211)
                };
                append_bytes(a212, a213)
            }, match v1 {
                Adt0::C0(_, _, _, h214) => {
                    h214
                }
            })))
        } else {
            Ok(f26(v1))
        }
    } else {
        Ok(f26(v1))
    }
}

pub fn f26(v0: Adt0) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h215, _, _, _) => {
            h215
        }
    }, false, match v0 {
        Adt0::C0(_, _, h216, _) => {
            h216
        }
    }, List::<(Bytes, Bytes)>::nil())
}

pub fn f27(v0: Bytes, v1: Adt1, v2: Bytes, v3: u64) -> R<Adt1> {
    let v4: u64 = f4(v0.clone(), v3, 4u64)?;
    let a217 = v4;
    let a218 = 4u64;
    if if nat_le(a217, a218) {
        f24(v0.clone(), {
            let a219 = v3;
            let a220 = 4u64;
            nat_add(a219, a220)?
        }, {
            let a223 = {
                let a221 = v4;
                let a222 = 8u64;
                nat_mul(a221, a222)?
            };
            let a224 = 12u64;
            nat_add(a223, a224)?
        })
    } else {
        false
    } {
        let v5: (bool, u64) = f43(v0.clone(), {
            let a225 = v3;
            let a226 = 4u64;
            nat_add(a225, a226)?
        }, v4)?;
        let v6: u64 = {
            let a231 = {
                let a227 = v3;
                let a228 = 4u64;
                nat_add(a227, a228)?
            };
            let a232 = {
                let a229 = v4;
                let a230 = 8u64;
                nat_mul(a229, a230)?
            };
            nat_add(a231, a232)?
        };
        let v7: (u64, u64) = f44(f4(v0.clone(), v6, 4u64)?)?;
        let v8: u64 = {
            let (_, h233) = v7;
            h233
        };
        let (h234, _) = v5;
        if if h234 {
            let a235 = 0u64;
            let a236 = v8;
            if nat_lt(a235, a236) {
                let a241 = {
                    let a238 = {
                        let (_, h237) = v5;
                        h237
                    };
                    let a239 = v8;
                    let a240 = 0u64;
                    nat_rem(a238, a239, a240)
                };
                let a242 = 0u64;
                nat_eq(a241, a242)
            } else {
                false
            }
        } else {
            false
        } {
            let v9: u64 = {
                let a244 = {
                    let (_, h243) = v5;
                    h243
                };
                let a245 = v8;
                let a246 = 0u64;
                nat_quot(a244, a245, a246)
            };
            let a251 = v9;
            let a252 = {
                let a248 = 18446744073709551615u64;
                let a249 = {
                    let (h247, _) = v7;
                    h247
                };
                let a250 = 0u64;
                nat_quot(a248, a249, a250)
            };
            if nat_le(a251, a252) {
                Ok(Adt1::C0({
                    let a253 = v6;
                    let a254 = 12u64;
                    nat_add(a253, a254)?
                }, true, f45(Adt2::C0(v2, f3(v0.clone(), v3, {
                    let a257 = {
                        let a255 = v4;
                        let a256 = 8u64;
                        nat_mul(a255, a256)?
                    };
                    let a258 = 8u64;
                    nat_add(a257, a258)?
                }), f4(v0, {
                    let a259 = v6;
                    let a260 = 4u64;
                    nat_add(a259, a260)?
                }, 8u64)?, {
                    let a262 = v9;
                    let a263 = {
                        let (h261, _) = v7;
                        h261
                    };
                    nat_mul(a262, a263)?
                }), match v1 {
                    Adt1::C0(_, _, h264) => {
                        h264
                    }
                })))
            } else {
                Ok(f28(v1))
            }
        } else {
            Ok(f28(v1))
        }
    } else {
        Ok(f28(v1))
    }
}

pub fn f28(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0 {
        Adt1::C0(h265, _, _) => {
            h265
        }
    }, false, List::<Adt2>::nil())
}

pub fn f29(v0: Bytes, v1: u64, v2: u64, v3: Adt3, v4: Adt2) -> R<Adt3> {
    if if match v3.clone() {
        Adt3::C0(h266, _, _) => {
            h266
        }
    } {
        let a269 = match v4.clone() {
            Adt2::C0(_, _, h267, _) => {
                h267
            }
        };
        let a270 = {
            let a268 = v0.clone();
            length_bytes(a268)
        };
        if nat_le(a269, a270) {
            let a275 = match v3.clone() {
                Adt3::C0(_, h271, _) => {
                    h271
                }
            };
            let a276 = {
                let a273 = 18446744073709551615u64;
                let a274 = match v4.clone() {
                    Adt2::C0(_, _, _, h272) => {
                        h272
                    }
                };
                nat_sub(a273, a274)
            };
            nat_le(a275, a276)
        } else {
            false
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a278 = v1;
            let a279 = match v4.clone() {
                Adt2::C0(_, _, h277, _) => {
                    h277
                }
            };
            nat_add(a278, a279)?
        };
        if f24(v0.clone(), v5, match v4.clone() {
            Adt2::C0(_, _, _, h280) => {
                h280
            }
        }) {
            Ok(Adt3::C0(true, f16({
                let a283 = match v3.clone() {
                    Adt3::C0(_, h281, _) => {
                        h281
                    }
                };
                let a284 = match v4.clone() {
                    Adt2::C0(_, _, _, h282) => {
                        h282
                    }
                };
                nat_add(a283, a284)?
            }, v2)?, {
                let a296 = match v3.clone() {
                    Adt3::C0(_, _, h285) => {
                        h285
                    }
                };
                let a297 = {
                    let a294 = f41(match v4.clone() {
                        Adt2::C0(h286, _, _, _) => {
                            h286
                        }
                    })?;
                    let a295 = {
                        let a292 = match v4.clone() {
                            Adt2::C0(_, h287, _, _) => {
                                h287
                            }
                        };
                        let a293 = {
                            let a290 = f19(match v3 {
                                Adt3::C0(_, h288, _) => {
                                    h288
                                }
                            }, 8u64)?;
                            let a291 = f41(f3(v0, v5, match v4 {
                                Adt2::C0(_, _, _, h289) => {
                                    h289
                                }
                            }))?;
                            append_bytes(a290, a291)
                        };
                        append_bytes(a292, a293)
                    };
                    append_bytes(a294, a295)
                };
                append_bytes(a296, a297)
            }))
        } else {
            Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
        }
    } else {
        Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
    }
}

pub fn f30(v0: Fn4, p301: u64, p302: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m300 = (p301, p302);
    {
        let mut m300 = m300;
        loop {
            let step = {
                let (v1, v2) = m300;
                let m298 = v1;
                if m298 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m298 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c299: Fn4 = v0.clone();
                        c299.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a304 = {
            let (h303, _) = v0.clone();
            h303
        };
        let a305 = 256u64;
        let a306 = 0u64;
        nat_quot(a304, a305, a306)
    }, {
        let a312 = {
            let (_, h307) = v0;
            h307
        };
        let a313 = f46({
            let a309 = {
                let (h308, _) = v0;
                h308
            };
            let a310 = 256u64;
            let a311 = 0u64;
            nat_rem(a309, a310, a311)
        });
        append_bytes(a312, a313)
    })
}

pub fn f32(v0: Fn5, p317: Bytes, p318: List<(Bytes, Bytes)>) -> Bytes {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, v2) = m316;
                let m314 = v2;
                match m314.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c315: Fn5 = v0.clone();
                            c315.apply(v1, v3)
                        }, v4))
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

pub fn f33(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    let a320 = v0;
    let a321 = {
        let (_, h319) = v1;
        h319
    };
    append_bytes(a320, a321)
}

pub fn f34(v0: u32) -> Bytes {
    let a350 = f47({
        let a322 = f48(v0, 28u32);
        let a323 = 15u32;
        fixed_u32::bit_and(a322, a323)
    });
    let a351 = {
        let a348 = f47({
            let a324 = f48(v0, 24u32);
            let a325 = 15u32;
            fixed_u32::bit_and(a324, a325)
        });
        let a349 = {
            let a346 = f47({
                let a326 = f48(v0, 20u32);
                let a327 = 15u32;
                fixed_u32::bit_and(a326, a327)
            });
            let a347 = {
                let a344 = f47({
                    let a328 = f48(v0, 16u32);
                    let a329 = 15u32;
                    fixed_u32::bit_and(a328, a329)
                });
                let a345 = {
                    let a342 = f47({
                        let a330 = f48(v0, 12u32);
                        let a331 = 15u32;
                        fixed_u32::bit_and(a330, a331)
                    });
                    let a343 = {
                        let a340 = f47({
                            let a332 = f48(v0, 8u32);
                            let a333 = 15u32;
                            fixed_u32::bit_and(a332, a333)
                        });
                        let a341 = {
                            let a338 = f47({
                                let a334 = f48(v0, 4u32);
                                let a335 = 15u32;
                                fixed_u32::bit_and(a334, a335)
                            });
                            let a339 = f47({
                                let a336 = v0;
                                let a337 = 15u32;
                                fixed_u32::bit_and(a336, a337)
                            });
                            append_bytes(a338, a339)
                        };
                        append_bytes(a340, a341)
                    };
                    append_bytes(a342, a343)
                };
                append_bytes(a344, a345)
            };
            append_bytes(a346, a347)
        };
        append_bytes(a348, a349)
    };
    append_bytes(a350, a351)
}

pub fn f35(v0: Fn6, p355: u64, p356: (u64, Adt4)) -> R<(u64, Adt4)> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v1, v2) = m354;
                let m352 = v1;
                if m352 == 0 {
                    Err::<(u64, (u64, Adt4)), R<(u64, Adt4)>>(Ok(v2))
                } else {
                    let v3 = m352 - 1;
                    Ok::<(u64, (u64, Adt4)), R<(u64, Adt4)>>((v3, {
                        let c353: Fn6 = v0.clone();
                        c353.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt4)) -> R<(u64, Adt4)> {
    Ok(({
        let a358 = {
            let (h357, _) = v3.clone();
            h357
        };
        let a359 = 64u64;
        nat_add(a358, a359)?
    }, f49({
        let (_, h360) = v3;
        h360
    }, f50(v0, v1, v2, {
        let (h361, _) = v3;
        h361
    })?)?))
}

pub fn f37() -> Adt4 {
    Adt4::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let v3: Adt5 = {
        let (h362, _) = f51(Fn7::F52(v0.clone()), {
            let a366 = {
                let a364 = {
                    let a363 = v0;
                    length_bytes(a363)
                };
                let a365 = 2u64;
                nat_mul(a364, a365)?
            };
            let a367 = 4u64;
            nat_add(a366, a367)?
        }, Adt5::C0(v1, true, List::cons(Adt6::C0(v2, 1u64, 0u64), List::<Adt6>::nil())))?;
        h362
    };
    if match v3.clone() {
        Adt5::C0(_, h368, _) => {
            h368
        }
    } {
        let m370 = match v3.clone() {
            Adt5::C0(_, _, h369) => {
                h369
            }
        };
        match m370.uncons() {
            None => {
                Ok((true, match v3 {
                    Adt5::C0(h371, _, _) => {
                        h371
                    }
                }))
            }
            Some((_, _)) => {
                Ok((false, 0u64))
            }
        }
    } else {
        Ok((false, 0u64))
    }
}

pub fn f39(v0: u64) -> bool {
    let a373 = {
        let (h372, _) = f53(Fn8::F54, 64u64, v0);
        h372
    };
    let a374 = 1u64;
    nat_eq(a373, a374)
}

pub fn f40(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt7 = f55(Fn9::F56(v1.clone(), v0.clone()), Adt7::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt7::C0(h375, _) => {
            h375
        }
    } {
        f57(match v3 {
            Adt7::C0(_, h376) => {
                h376
            }
        })
    } else {
        f57(List::cons((v0, v1), match v3 {
            Adt7::C0(_, h377) => {
                h377
            }
        }))
    }
}

pub fn f41(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f11(v0)?;
    let (_, h378) = f30(Fn4::F58(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h378)
}

pub fn f42(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Bytes> {
    let a379 = 0u64;
    let a380 = f59(v1);
    if nat_lt(a379, a380) {
        Ok(f3(v0, v2, v3))
    } else {
        let a381 = v1;
        let a382 = 8u64;
        if nat_eq(a381, a382) {
            let a387 = f3(v0.clone(), v2, 8u64);
            let a388 = f41(f3(v0, {
                let a383 = v2;
                let a384 = 8u64;
                nat_add(a383, a384)?
            }, {
                let a385 = v3;
                let a386 = 8u64;
                nat_sub(a385, a386)
            }))?;
            Ok(append_bytes(a387, a388))
        } else {
            let a393 = f3(v0.clone(), v2, 12u64);
            let a394 = f41(f3(v0, {
                let a389 = v2;
                let a390 = 12u64;
                nat_add(a389, a390)?
            }, {
                let a391 = v3;
                let a392 = 12u64;
                nat_sub(a391, a392)
            }))?;
            Ok(append_bytes(a393, a394))
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let (_, h395) = f60(Fn10::F61(v1, v0), v2, (0u64, (true, 1u64)))?;
    Ok(h395)
}

pub fn f44(v0: u64) -> R<(u64, u64)> {
    let v1: Bytes = Bytes::lit(&[0x00, 0x04, 0x01, 0x00, 0x02, 0x01, 0x00, 0x12, 0x02, 0x00, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x02, 0x00, 0x18, 0x02, 0x00, 0x22, 0x02, 0x00, 0x24, 0x02, 0x00, 0x54, 0x03, 0x00, 0x6e, 0x03, 0x00, 0x90, 0x03, 0x00, 0xb0, 0x03, 0x00, 0xd2, 0x03, 0x01, 0x24, 0x03, 0x00, 0x42, 0x03, 0x00, 0x4a, 0x03, 0x00, 0x62, 0x03, 0x00, 0x32, 0x03, 0x00, 0x12, 0x02, 0x00, 0x6e, 0x03, 0x00, 0x52, 0x03, 0x00, 0x88, 0x03, 0x00, 0x01, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x01, 0x00, 0x08, 0x01, 0x00, 0x08, 0x01, 0x00, 0x38, 0x03, 0x00, 0x02, 0x01]);
    let v2: u64 = {
        let a396 = v0;
        let a397 = 3u64;
        nat_mul(a396, a397)?
    };
    let v3: u64 = {
        let a398 = v0;
        let a399 = 31u64;
        if nat_lt(a398, a399) {
            f23(v1.clone(), {
                let a400 = v2;
                let a401 = 2u64;
                nat_add(a400, a401)?
            })?
        } else {
            0u64
        }
    };
    Ok(({
        let a406 = {
            let a402 = f23(v1.clone(), v2)?;
            let a403 = 256u64;
            nat_mul(a402, a403)?
        };
        let a407 = f23(v1, {
            let a404 = v2;
            let a405 = 1u64;
            nat_add(a404, a405)?
        })?;
        nat_add(a406, a407)?
    }, {
        let a408 = v3;
        let a409 = 1u64;
        if nat_eq(a408, a409) {
            1u64
        } else {
            let a410 = v3;
            let a411 = 2u64;
            if nat_eq(a410, a411) {
                32u64
            } else {
                let a412 = v3;
                let a413 = 3u64;
                if nat_eq(a412, a413) {
                    256u64
                } else {
                    0u64
                }
            }
        }
    }))
}

pub fn f45(v0: Adt2, v1: List<Adt2>) -> List<Adt2> {
    let v2: Adt8 = f62(Fn11::F63(std::rc::Rc::new(v0.clone())), Adt8::C0(false, List::<Adt2>::nil()), v1);
    if match v2.clone() {
        Adt8::C0(h414, _) => {
            h414
        }
    } {
        f64(match v2 {
            Adt8::C0(_, h415) => {
                h415
            }
        })
    } else {
        f64(List::cons(v0, match v2 {
            Adt8::C0(_, h416) => {
                h416
            }
        }))
    }
}

pub fn f46(v0: u64) -> Bytes {
    f3(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f47(v0: u32) -> Bytes {
    let a419 = {
        let a417 = v0;
        let a418 = 8u32;
        fixed_u32::bit_and(a417, a418)
    };
    let a420 = 0u32;
    if equal(a419, a420) {
        let a423 = {
            let a421 = v0;
            let a422 = 4u32;
            fixed_u32::bit_and(a421, a422)
        };
        let a424 = 0u32;
        if equal(a423, a424) {
            let a427 = {
                let a425 = v0;
                let a426 = 2u32;
                fixed_u32::bit_and(a425, a426)
            };
            let a428 = 0u32;
            if equal(a427, a428) {
                let a431 = {
                    let a429 = v0;
                    let a430 = 1u32;
                    fixed_u32::bit_and(a429, a430)
                };
                let a432 = 0u32;
                if equal(a431, a432) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a435 = {
                    let a433 = v0;
                    let a434 = 1u32;
                    fixed_u32::bit_and(a433, a434)
                };
                let a436 = 0u32;
                if equal(a435, a436) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a439 = {
                let a437 = v0;
                let a438 = 2u32;
                fixed_u32::bit_and(a437, a438)
            };
            let a440 = 0u32;
            if equal(a439, a440) {
                let a443 = {
                    let a441 = v0;
                    let a442 = 1u32;
                    fixed_u32::bit_and(a441, a442)
                };
                let a444 = 0u32;
                if equal(a443, a444) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a447 = {
                    let a445 = v0;
                    let a446 = 1u32;
                    fixed_u32::bit_and(a445, a446)
                };
                let a448 = 0u32;
                if equal(a447, a448) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a451 = {
            let a449 = v0;
            let a450 = 4u32;
            fixed_u32::bit_and(a449, a450)
        };
        let a452 = 0u32;
        if equal(a451, a452) {
            let a455 = {
                let a453 = v0;
                let a454 = 2u32;
                fixed_u32::bit_and(a453, a454)
            };
            let a456 = 0u32;
            if equal(a455, a456) {
                let a459 = {
                    let a457 = v0;
                    let a458 = 1u32;
                    fixed_u32::bit_and(a457, a458)
                };
                let a460 = 0u32;
                if equal(a459, a460) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a463 = {
                    let a461 = v0;
                    let a462 = 1u32;
                    fixed_u32::bit_and(a461, a462)
                };
                let a464 = 0u32;
                if equal(a463, a464) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a467 = {
                let a465 = v0;
                let a466 = 2u32;
                fixed_u32::bit_and(a465, a466)
            };
            let a468 = 0u32;
            if equal(a467, a468) {
                let a471 = {
                    let a469 = v0;
                    let a470 = 1u32;
                    fixed_u32::bit_and(a469, a470)
                };
                let a472 = 0u32;
                if equal(a471, a472) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a475 = {
                    let a473 = v0;
                    let a474 = 1u32;
                    fixed_u32::bit_and(a473, a474)
                };
                let a476 = 0u32;
                if equal(a475, a476) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f48(v0: u32, v1: u32) -> u32 {
    let m479 = {
        let a477 = v0;
        let a478 = v1;
        fixed_u32::shift_right(a477, a478)
    };
    match m479 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f49(v0: Adt4, v1: Adt9) -> R<Adt4> {
    let v2: Adt10 = f65(Fn12::F66, 64u64, Adt10::C0(match v0.clone() {
        Adt4::C0(h480, _, _, _, _, _, _, _) => {
            h480
        }
    }, match v0.clone() {
        Adt4::C0(_, h481, _, _, _, _, _, _) => {
            h481
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h482, _, _, _, _, _) => {
            h482
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h483, _, _, _, _) => {
            h483
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h484, _, _, _) => {
            h484
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h485, _, _) => {
            h485
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h486, _) => {
            h486
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h487) => {
            h487
        }
    }, std::rc::Rc::new(v1), 0u64))?;
    Ok(Adt4::C0(f67(match v0.clone() {
        Adt4::C0(h488, _, _, _, _, _, _, _) => {
            h488
        }
    }, match v2.clone() {
        Adt10::C0(h489, _, _, _, _, _, _, _, _, _) => {
            h489
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, h490, _, _, _, _, _, _) => {
            h490
        }
    }, match v2.clone() {
        Adt10::C0(_, h491, _, _, _, _, _, _, _, _) => {
            h491
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, h492, _, _, _, _, _) => {
            h492
        }
    }, match v2.clone() {
        Adt10::C0(_, _, h493, _, _, _, _, _, _, _) => {
            h493
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, _, h494, _, _, _, _) => {
            h494
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, h495, _, _, _, _, _, _) => {
            h495
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, _, _, h496, _, _, _) => {
            h496
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, h497, _, _, _, _, _) => {
            h497
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, _, _, _, h498, _, _) => {
            h498
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, h499, _, _, _, _) => {
            h499
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h500, _) => {
            h500
        }
    }, match v2.clone() {
        Adt10::C0(_, _, _, _, _, _, h501, _, _, _) => {
            h501
        }
    }), f67(match v0 {
        Adt4::C0(_, _, _, _, _, _, _, h502) => {
            h502
        }
    }, match v2 {
        Adt10::C0(_, _, _, _, _, _, _, h503, _, _) => {
            h503
        }
    })))
}

pub fn f50(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    Ok(Adt9::C0(f68(v0.clone(), v1, v2, v3)?, f68(v0.clone(), v1, v2, {
        let a504 = v3;
        let a505 = 4u64;
        nat_add(a504, a505)?
    })?, f68(v0.clone(), v1, v2, {
        let a506 = v3;
        let a507 = 8u64;
        nat_add(a506, a507)?
    })?, f68(v0.clone(), v1, v2, {
        let a508 = v3;
        let a509 = 12u64;
        nat_add(a508, a509)?
    })?, f68(v0.clone(), v1, v2, {
        let a510 = v3;
        let a511 = 16u64;
        nat_add(a510, a511)?
    })?, f68(v0.clone(), v1, v2, {
        let a512 = v3;
        let a513 = 20u64;
        nat_add(a512, a513)?
    })?, f68(v0.clone(), v1, v2, {
        let a514 = v3;
        let a515 = 24u64;
        nat_add(a514, a515)?
    })?, f68(v0.clone(), v1, v2, {
        let a516 = v3;
        let a517 = 28u64;
        nat_add(a516, a517)?
    })?, f68(v0.clone(), v1, v2, {
        let a518 = v3;
        let a519 = 32u64;
        nat_add(a518, a519)?
    })?, f68(v0.clone(), v1, v2, {
        let a520 = v3;
        let a521 = 36u64;
        nat_add(a520, a521)?
    })?, f68(v0.clone(), v1, v2, {
        let a522 = v3;
        let a523 = 40u64;
        nat_add(a522, a523)?
    })?, f68(v0.clone(), v1, v2, {
        let a524 = v3;
        let a525 = 44u64;
        nat_add(a524, a525)?
    })?, f68(v0.clone(), v1, v2, {
        let a526 = v3;
        let a527 = 48u64;
        nat_add(a526, a527)?
    })?, f68(v0.clone(), v1, v2, {
        let a528 = v3;
        let a529 = 52u64;
        nat_add(a528, a529)?
    })?, f68(v0.clone(), v1, v2, {
        let a530 = v3;
        let a531 = 56u64;
        nat_add(a530, a531)?
    })?, f68(v0, v1, v2, {
        let a532 = v3;
        let a533 = 60u64;
        nat_add(a532, a533)?
    })?))
}

pub fn f51(v0: Fn7, p538: u64, p539: Adt5) -> R<(Adt5, bool)> {
    let m537 = (p538, p539);
    {
        let mut m537 = m537;
        loop {
            let step = {
                let (v1, v2) = m537;
                let m534 = v1;
                if m534 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m534 - 1;
                    let m536 = {
                        let c535: Fn7 = v0.clone();
                        c535.apply(v2.clone())?
                    };
                    match m536 {
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
                Ok(next) => { m537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: Adt5) -> R<Option<Adt5>> {
    f69(v0, v1)
}

pub fn f53(v0: Fn8, p544: u64, p545: u64) -> (u64, bool) {
    let m543 = (p544, p545);
    {
        let mut m543 = m543;
        loop {
            let step = {
                let (v1, v2) = m543;
                let m540 = v1;
                if m540 == 0 {
                    Err::<(u64, u64), (u64, bool)>((v2, false))
                } else {
                    let v3 = m540 - 1;
                    let m542 = {
                        let c541: Fn8 = v0.clone();
                        c541.apply(v2)
                    };
                    match m542 {
                        None => {
                            Err::<(u64, u64), (u64, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), (u64, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: u64) -> Option<u64> {
    let a546 = 1u64;
    let a547 = v0;
    if if nat_lt(a546, a547) {
        let a551 = {
            let a548 = v0;
            let a549 = 2u64;
            let a550 = 0u64;
            nat_rem(a548, a549, a550)
        };
        let a552 = 0u64;
        nat_eq(a551, a552)
    } else {
        false
    } {
        Some({
            let a553 = v0;
            let a554 = 2u64;
            let a555 = 0u64;
            nat_quot(a553, a554, a555)
        })
    } else {
        None::<u64>
    }
}

pub fn f55(v0: Fn9, p559: Adt7, p560: List<(Bytes, Bytes)>) -> Adt7 {
    let m558 = (p559, p560);
    {
        let mut m558 = m558;
        loop {
            let step = {
                let (v1, v2) = m558;
                let m556 = v2;
                match m556.uncons() {
                    None => {
                        Err::<(Adt7, List<(Bytes, Bytes)>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<(Bytes, Bytes)>), Adt7>(({
                            let c557: Fn9 = v0.clone();
                            c557.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Bytes, v2: Adt7, v3: (Bytes, Bytes)) -> Adt7 {
    if match v2.clone() {
        Adt7::C0(h561, _) => {
            h561
        }
    } {
        Adt7::C0(true, List::cons(v3, match v2 {
            Adt7::C0(_, h562) => {
                h562
            }
        }))
    } else {
        if f70(v1.clone(), {
            let (h563, _) = v3.clone();
            h563
        }) {
            Adt7::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt7::C0(_, h564) => {
                    h564
                }
            })))
        } else {
            Adt7::C0(false, List::cons(v3, match v2 {
                Adt7::C0(_, h565) => {
                    h565
                }
            }))
        }
    }
}

pub fn f57(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f71(Fn13::F72, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f58(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a567 = {
            let (h566, _) = v1.clone();
            h566
        };
        let a568 = 2u64;
        nat_mul(a567, a568)?
    };
    Ok(({
        let a570 = {
            let (h569, _) = v1;
            h569
        };
        let a571 = 1u64;
        nat_add(a570, a571)?
    }, {
        let a579 = {
            let (_, h572) = v1;
            h572
        };
        let a580 = f46({
            let a577 = {
                let a573 = f73(f23(v0.clone(), v2)?);
                let a574 = 16u64;
                nat_mul(a573, a574)?
            };
            let a578 = f73(f23(v0, {
                let a575 = v2;
                let a576 = 1u64;
                nat_add(a575, a576)?
            })?);
            nat_add(a577, a578)?
        });
        append_bytes(a579, a580)
    }))
}

pub fn f59(v0: u64) -> u64 {
    let a581 = v0;
    let a582 = 0u64;
    if if nat_eq(a581, a582) {
        true
    } else {
        let a583 = v0;
        let a584 = 1u64;
        if nat_eq(a583, a584) {
            true
        } else {
            let a585 = v0;
            let a586 = 7u64;
            nat_eq(a585, a586)
        }
    } {
        1u64
    } else {
        let a587 = v0;
        let a588 = 2u64;
        if if nat_eq(a587, a588) {
            true
        } else {
            let a589 = v0;
            let a590 = 3u64;
            nat_eq(a589, a590)
        } {
            2u64
        } else {
            let a591 = v0;
            let a592 = 4u64;
            if if nat_eq(a591, a592) {
                true
            } else {
                let a593 = v0;
                let a594 = 5u64;
                if nat_eq(a593, a594) {
                    true
                } else {
                    let a595 = v0;
                    let a596 = 6u64;
                    nat_eq(a595, a596)
                }
            } {
                4u64
            } else {
                let a597 = v0;
                let a598 = 10u64;
                if if nat_eq(a597, a598) {
                    true
                } else {
                    let a599 = v0;
                    let a600 = 11u64;
                    if nat_eq(a599, a600) {
                        true
                    } else {
                        let a601 = v0;
                        let a602 = 12u64;
                        nat_eq(a601, a602)
                    }
                } {
                    8u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f60(v0: Fn10, p606: u64, p607: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let m605 = (p606, p607);
    {
        let mut m605 = m605;
        loop {
            let step = {
                let (v1, v2) = m605;
                let m603 = v1;
                if m603 == 0 {
                    Err::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>(Ok(v2))
                } else {
                    let v3 = m603 - 1;
                    Ok::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>((v3, {
                        let c604: Fn10 = v0.clone();
                        c604.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m605 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u64, v1: Bytes, v2: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let v3: u64 = f4(v1, {
        let a611 = v0;
        let a612 = {
            let a609 = {
                let (h608, _) = v2;
                h608
            };
            let a610 = 8u64;
            nat_mul(a609, a610)?
        };
        nat_add(a611, a612)?
    }, 8u64)?;
    let v4: u64 = {
        let (_, h613) = {
            let (_, h614) = v2;
            h614
        };
        h613
    };
    let v5: bool = {
        let (h615, _) = {
            let (_, h616) = v2;
            h616
        };
        if h615 {
            let a617 = v3;
            let a618 = 0u64;
            if nat_eq(a617, a618) {
                true
            } else {
                let a622 = v4;
                let a623 = {
                    let a619 = 18446744073709551615u64;
                    let a620 = v3;
                    let a621 = 0u64;
                    nat_quot(a619, a620, a621)
                };
                nat_le(a622, a623)
            }
        } else {
            false
        }
    };
    Ok(({
        let a625 = {
            let (h624, _) = v2;
            h624
        };
        let a626 = 1u64;
        nat_add(a625, a626)?
    }, (v5, if v5 {
        let a627 = v4;
        let a628 = v3;
        nat_mul(a627, a628)?
    } else {
        0u64
    })))
}

pub fn f62(v0: Fn11, p632: Adt8, p633: List<Adt2>) -> Adt8 {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m629 = v2;
                match m629.uncons() {
                    None => {
                        Err::<(Adt8, List<Adt2>), Adt8>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt8, List<Adt2>), Adt8>(({
                            let c630: Fn11 = v0.clone();
                            c630.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt2, v1: Adt8, v2: Adt2) -> Adt8 {
    if match v1.clone() {
        Adt8::C0(h634, _) => {
            h634
        }
    } {
        Adt8::C0(true, List::cons(v2, match v1 {
            Adt8::C0(_, h635) => {
                h635
            }
        }))
    } else {
        if f70(match v0.clone() {
            Adt2::C0(h636, _, _, _) => {
                h636
            }
        }, match v2.clone() {
            Adt2::C0(h637, _, _, _) => {
                h637
            }
        }) {
            Adt8::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt8::C0(_, h638) => {
                    h638
                }
            })))
        } else {
            Adt8::C0(false, List::cons(v2, match v1 {
                Adt8::C0(_, h639) => {
                    h639
                }
            }))
        }
    }
}

pub fn f64(v0: List<Adt2>) -> List<Adt2> {
    f74(Fn14::F75, List::<Adt2>::nil(), v0)
}

pub fn f65(v0: Fn12, p643: u64, p644: Adt10) -> R<Adt10> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m640 = v1;
                if m640 == 0 {
                    Err::<(u64, Adt10), R<Adt10>>(Ok(v2))
                } else {
                    let v3 = m640 - 1;
                    Ok::<(u64, Adt10), R<Adt10>>((v3, {
                        let c641: Fn12 = v0.clone();
                        c641.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt10) -> R<Adt10> {
    f76(v0)
}

pub fn f67(v0: u32, v1: u32) -> u32 {
    let m647 = {
        let a645 = f77(v0);
        let a646 = f77(v1);
        fixed_u64::checked_add(a645, a646)
    };
    match m647 {
        None => {
            0u32
        }
        Some(v2) => {
            f78(v2)
        }
    }
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a650 = {
        let a648 = v3;
        let a649 = 8u64;
        nat_add(a648, a649)?
    };
    let a651 = v2;
    if nat_eq(a650, a651) {
        Ok(f79({
            let a654 = {
                let a652 = v1;
                let a653 = 8u64;
                nat_mul(a652, a653)?
            };
            let a655 = 4294967296u64;
            let a656 = 0u64;
            nat_quot(a654, a655, a656)
        }))
    } else {
        let a659 = {
            let a657 = v3;
            let a658 = 4u64;
            nat_add(a657, a658)?
        };
        let a660 = v2;
        if nat_eq(a659, a660) {
            Ok(f79({
                let a663 = {
                    let a661 = v1;
                    let a662 = 8u64;
                    nat_mul(a661, a662)?
                };
                let a664 = 4294967296u64;
                let a665 = 0u64;
                nat_rem(a663, a664, a665)
            }))
        } else {
            Ok(f80(f81(v0.clone(), v1, v3), f81(v0.clone(), v1, {
                let a666 = v3;
                let a667 = 1u64;
                nat_add(a666, a667)?
            }), f81(v0.clone(), v1, {
                let a668 = v3;
                let a669 = 2u64;
                nat_add(a668, a669)?
            }), f81(v0, v1, {
                let a670 = v3;
                let a671 = 3u64;
                nat_add(a670, a671)?
            })))
        }
    }
}

pub fn f69(v0: Bytes, v1: Adt5) -> R<Option<Adt5>> {
    if match v1.clone() {
        Adt5::C0(_, h672, _) => {
            h672
        }
    } {
        let m674 = match v1.clone() {
            Adt5::C0(_, _, h673) => {
                h673
            }
        };
        match m674.uncons() {
            None => {
                Ok(None::<Adt5>)
            }
            Some((v2, v3)) => {
                let a676 = match v2.clone() {
                    Adt6::C0(_, h675, _) => {
                        h675
                    }
                };
                let a677 = 0u64;
                if nat_eq(a676, a677) {
                    Ok(f82(match v1 {
                        Adt5::C0(h678, _, _) => {
                            h678
                        }
                    }, v3))
                } else {
                    f83(v0, match v1 {
                        Adt5::C0(h679, _, _) => {
                            h679
                        }
                    }, v2.clone(), List::cons(Adt6::C0(match v2.clone() {
                        Adt6::C0(h680, _, _) => {
                            h680
                        }
                    }, {
                        let a682 = match v2.clone() {
                            Adt6::C0(_, h681, _) => {
                                h681
                            }
                        };
                        let a683 = 1u64;
                        nat_sub(a682, a683)
                    }, match v2 {
                        Adt6::C0(_, _, h684) => {
                            h684
                        }
                    }), v3))
                }
            }
        }
    } else {
        Ok(None::<Adt5>)
    }
}

pub fn f70(v0: Bytes, v1: Bytes) -> bool {
    let a689 = {
        let a685 = v0;
        let a686 = v1;
        compare_bytes(a685, a686)
    };
    let a690 = {
        let a687 = Bytes::lit(&[]);
        let a688 = Bytes::lit(&[0x01]);
        compare_bytes(a687, a688)
    };
    equal(a689, a690)
}

pub fn f71(v0: Fn13, p694: List<(Bytes, Bytes)>, p695: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v1, v2) = m693;
                let m691 = v2;
                match m691.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c692: Fn13 = v0.clone();
                            c692.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f73(v0: u64) -> u64 {
    let a696 = 47u64;
    let a697 = v0;
    if nat_lt(a696, a697) {
        let a698 = v0;
        let a699 = 58u64;
        if nat_lt(a698, a699) {
            let a700 = v0;
            let a701 = 48u64;
            nat_sub(a700, a701)
        } else {
            let a702 = 64u64;
            let a703 = v0;
            if nat_lt(a702, a703) {
                let a704 = v0;
                let a705 = 71u64;
                if nat_lt(a704, a705) {
                    let a706 = v0;
                    let a707 = 55u64;
                    nat_sub(a706, a707)
                } else {
                    let a708 = 96u64;
                    let a709 = v0;
                    if nat_lt(a708, a709) {
                        let a710 = v0;
                        let a711 = 103u64;
                        if nat_lt(a710, a711) {
                            let a712 = v0;
                            let a713 = 87u64;
                            nat_sub(a712, a713)
                        } else {
                            16u64
                        }
                    } else {
                        16u64
                    }
                }
            } else {
                16u64
            }
        }
    } else {
        16u64
    }
}

pub fn f74(v0: Fn14, p717: List<Adt2>, p718: List<Adt2>) -> List<Adt2> {
    let m716 = (p717, p718);
    {
        let mut m716 = m716;
        loop {
            let step = {
                let (v1, v2) = m716;
                let m714 = v2;
                match m714.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c715: Fn14 = v0.clone();
                            c715.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f76(v0: Adt10) -> R<Adt10> {
    let v1: u32 = f84(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, h719) => {
            h719
        }
    });
    let v2: u32 = f67(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, h720, _, _) => {
            h720
        }
    }, f67(f85(match v0.clone() {
        Adt10::C0(_, _, _, _, h721, _, _, _, _, _) => {
            h721
        }
    }), f67(f86(match v0.clone() {
        Adt10::C0(_, _, _, _, h722, _, _, _, _, _) => {
            h722
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h723, _, _, _, _) => {
            h723
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h724, _, _, _) => {
            h724
        }
    }), f67(v1, {
        let m727 = match v0.clone() {
            Adt10::C0(_, _, _, _, _, _, _, _, h726, _) => {
                (*h726).clone()
            }
        };
        match m727 {
            Adt9::C0(h725, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h725
            }
        }
    }))));
    let v3: u32 = f67(f87(match v0.clone() {
        Adt10::C0(h728, _, _, _, _, _, _, _, _, _) => {
            h728
        }
    }), f88(match v0.clone() {
        Adt10::C0(h729, _, _, _, _, _, _, _, _, _) => {
            h729
        }
    }, match v0.clone() {
        Adt10::C0(_, h730, _, _, _, _, _, _, _, _) => {
            h730
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h731, _, _, _, _, _, _, _) => {
            h731
        }
    }));
    Ok(Adt10::C0(f67(v2, v3), match v0.clone() {
        Adt10::C0(h732, _, _, _, _, _, _, _, _, _) => {
            h732
        }
    }, match v0.clone() {
        Adt10::C0(_, h733, _, _, _, _, _, _, _, _) => {
            h733
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h734, _, _, _, _, _, _, _) => {
            h734
        }
    }, f67(match v0.clone() {
        Adt10::C0(_, _, _, h735, _, _, _, _, _, _) => {
            h735
        }
    }, v2), match v0.clone() {
        Adt10::C0(_, _, _, _, h736, _, _, _, _, _) => {
            h736
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h737, _, _, _, _) => {
            h737
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h738, _, _, _) => {
            h738
        }
    }, std::rc::Rc::new(f89(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, h739, _) => {
            (*h739).clone()
        }
    })), {
        let a741 = match v0 {
            Adt10::C0(_, _, _, _, _, _, _, _, _, h740) => {
                h740
            }
        };
        let a742 = 1u64;
        nat_add(a741, a742)?
    }))
}

pub fn f77(v0: u32) -> u64 {
    let m744 = {
        let a743 = v0;
        fixed_u64::convert(i128::from(a743))
    };
    match m744 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f78(v0: u64) -> u32 {
    let m748 = {
        let a747 = {
            let a745 = v0;
            let a746 = 4294967295u64;
            fixed_u64::bit_and(a745, a746)
        };
        fixed_u32::convert(i128::from(a747))
    };
    match m748 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f79(v0: u64) -> u32 {
    let (_, h749) = {
        let (_, h750) = f90(Fn15::F91, 32u64, (v0, (1u32, 0u32)));
        h750
    };
    h749
}

pub fn f80(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a755 = f92(v0, 24u32);
    let a756 = {
        let a753 = f92(v1, 16u32);
        let a754 = {
            let a751 = f92(v2, 8u32);
            let a752 = v3;
            fixed_u32::bit_or(a751, a752)
        };
        fixed_u32::bit_or(a753, a754)
    };
    fixed_u32::bit_or(a755, a756)
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a757 = v2;
    let a758 = v1;
    if nat_lt(a757, a758) {
        let m761 = {
            let a759 = v0;
            let a760 = v2;
            index_bytes(a759, a760)
        };
        match m761 {
            None => {
                0u32
            }
            Some(v3) => {
                let m763 = {
                    let a762 = v3;
                    fixed_u32::convert(i128::from(a762))
                };
                match m763 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a764 = v2;
        let a765 = v1;
        if nat_eq(a764, a765) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f82(v0: u64, v1: List<Adt6>) -> Option<Adt5> {
    Some(Adt5::C0(v0, true, v1))
}

pub fn f83(v0: Bytes, v1: u64, v2: Adt6, v3: List<Adt6>) -> R<Option<Adt5>> {
    let v4: u64 = match v2.clone() {
        Adt6::C0(h766, _, _) => {
            h766
        }
    };
    let v5: u64 = f59(v4);
    let a767 = 0u64;
    let a768 = v5;
    if nat_lt(a767, a768) {
        if f24(v0, v1, v5) {
            Ok(f82({
                let a769 = v1;
                let a770 = v5;
                nat_add(a769, a770)?
            }, v3))
        } else {
            Ok(f93(v1))
        }
    } else {
        let a771 = v4;
        let a772 = 8u64;
        if nat_eq(a771, a772) {
            if f24(v0.clone(), v1, 8u64) {
                let v6: u64 = f4(v0.clone(), v1, 8u64)?;
                if f24(v0, {
                    let a773 = v1;
                    let a774 = 8u64;
                    nat_add(a773, a774)?
                }, v6) {
                    Ok(f82({
                        let a777 = {
                            let a775 = v1;
                            let a776 = 8u64;
                            nat_add(a775, a776)?
                        };
                        let a778 = v6;
                        nat_add(a777, a778)?
                    }, v3))
                } else {
                    Ok(f93(v1))
                }
            } else {
                Ok(f93(v1))
            }
        } else {
            let a779 = v4;
            let a780 = 9u64;
            if nat_eq(a779, a780) {
                let a782 = 64u64;
                let a783 = match v2.clone() {
                    Adt6::C0(_, _, h781) => {
                        h781
                    }
                };
                if nat_le(a782, a783) {
                    Ok(f93(v1))
                } else {
                    if f24(v0.clone(), v1, 12u64) {
                        Ok(f82({
                            let a784 = v1;
                            let a785 = 12u64;
                            nat_add(a784, a785)?
                        }, List::cons(Adt6::C0(f4(v0.clone(), v1, 4u64)?, f4(v0, {
                            let a786 = v1;
                            let a787 = 4u64;
                            nat_add(a786, a787)?
                        }, 8u64)?, {
                            let a789 = match v2 {
                                Adt6::C0(_, _, h788) => {
                                    h788
                                }
                            };
                            let a790 = 1u64;
                            nat_add(a789, a790)?
                        }), v3)))
                    } else {
                        Ok(f93(v1))
                    }
                }
            } else {
                Ok(f93(v1))
            }
        }
    }
}

pub fn f84(v0: u64) -> u32 {
    let a791 = v0;
    let a792 = 32u64;
    if nat_lt(a791, a792) {
        let a793 = v0;
        let a794 = 16u64;
        if nat_lt(a793, a794) {
            let a795 = v0;
            let a796 = 8u64;
            if nat_lt(a795, a796) {
                let a797 = v0;
                let a798 = 4u64;
                if nat_lt(a797, a798) {
                    let a799 = v0;
                    let a800 = 2u64;
                    if nat_lt(a799, a800) {
                        let a801 = v0;
                        let a802 = 1u64;
                        if nat_lt(a801, a802) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a803 = v0;
                        let a804 = 3u64;
                        if nat_lt(a803, a804) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a805 = v0;
                    let a806 = 6u64;
                    if nat_lt(a805, a806) {
                        let a807 = v0;
                        let a808 = 5u64;
                        if nat_lt(a807, a808) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a809 = v0;
                        let a810 = 7u64;
                        if nat_lt(a809, a810) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a811 = v0;
                let a812 = 12u64;
                if nat_lt(a811, a812) {
                    let a813 = v0;
                    let a814 = 10u64;
                    if nat_lt(a813, a814) {
                        let a815 = v0;
                        let a816 = 9u64;
                        if nat_lt(a815, a816) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a817 = v0;
                        let a818 = 11u64;
                        if nat_lt(a817, a818) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a819 = v0;
                    let a820 = 14u64;
                    if nat_lt(a819, a820) {
                        let a821 = v0;
                        let a822 = 13u64;
                        if nat_lt(a821, a822) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a823 = v0;
                        let a824 = 15u64;
                        if nat_lt(a823, a824) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a825 = v0;
            let a826 = 24u64;
            if nat_lt(a825, a826) {
                let a827 = v0;
                let a828 = 20u64;
                if nat_lt(a827, a828) {
                    let a829 = v0;
                    let a830 = 18u64;
                    if nat_lt(a829, a830) {
                        let a831 = v0;
                        let a832 = 17u64;
                        if nat_lt(a831, a832) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a833 = v0;
                        let a834 = 19u64;
                        if nat_lt(a833, a834) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a835 = v0;
                    let a836 = 22u64;
                    if nat_lt(a835, a836) {
                        let a837 = v0;
                        let a838 = 21u64;
                        if nat_lt(a837, a838) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a839 = v0;
                        let a840 = 23u64;
                        if nat_lt(a839, a840) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a841 = v0;
                let a842 = 28u64;
                if nat_lt(a841, a842) {
                    let a843 = v0;
                    let a844 = 26u64;
                    if nat_lt(a843, a844) {
                        let a845 = v0;
                        let a846 = 25u64;
                        if nat_lt(a845, a846) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a847 = v0;
                        let a848 = 27u64;
                        if nat_lt(a847, a848) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a849 = v0;
                    let a850 = 30u64;
                    if nat_lt(a849, a850) {
                        let a851 = v0;
                        let a852 = 29u64;
                        if nat_lt(a851, a852) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a853 = v0;
                        let a854 = 31u64;
                        if nat_lt(a853, a854) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a855 = v0;
        let a856 = 48u64;
        if nat_lt(a855, a856) {
            let a857 = v0;
            let a858 = 40u64;
            if nat_lt(a857, a858) {
                let a859 = v0;
                let a860 = 36u64;
                if nat_lt(a859, a860) {
                    let a861 = v0;
                    let a862 = 34u64;
                    if nat_lt(a861, a862) {
                        let a863 = v0;
                        let a864 = 33u64;
                        if nat_lt(a863, a864) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a865 = v0;
                        let a866 = 35u64;
                        if nat_lt(a865, a866) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a867 = v0;
                    let a868 = 38u64;
                    if nat_lt(a867, a868) {
                        let a869 = v0;
                        let a870 = 37u64;
                        if nat_lt(a869, a870) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a871 = v0;
                        let a872 = 39u64;
                        if nat_lt(a871, a872) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a873 = v0;
                let a874 = 44u64;
                if nat_lt(a873, a874) {
                    let a875 = v0;
                    let a876 = 42u64;
                    if nat_lt(a875, a876) {
                        let a877 = v0;
                        let a878 = 41u64;
                        if nat_lt(a877, a878) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a879 = v0;
                        let a880 = 43u64;
                        if nat_lt(a879, a880) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a881 = v0;
                    let a882 = 46u64;
                    if nat_lt(a881, a882) {
                        let a883 = v0;
                        let a884 = 45u64;
                        if nat_lt(a883, a884) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a885 = v0;
                        let a886 = 47u64;
                        if nat_lt(a885, a886) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a887 = v0;
            let a888 = 56u64;
            if nat_lt(a887, a888) {
                let a889 = v0;
                let a890 = 52u64;
                if nat_lt(a889, a890) {
                    let a891 = v0;
                    let a892 = 50u64;
                    if nat_lt(a891, a892) {
                        let a893 = v0;
                        let a894 = 49u64;
                        if nat_lt(a893, a894) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a895 = v0;
                        let a896 = 51u64;
                        if nat_lt(a895, a896) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a897 = v0;
                    let a898 = 54u64;
                    if nat_lt(a897, a898) {
                        let a899 = v0;
                        let a900 = 53u64;
                        if nat_lt(a899, a900) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a901 = v0;
                        let a902 = 55u64;
                        if nat_lt(a901, a902) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a903 = v0;
                let a904 = 60u64;
                if nat_lt(a903, a904) {
                    let a905 = v0;
                    let a906 = 58u64;
                    if nat_lt(a905, a906) {
                        let a907 = v0;
                        let a908 = 57u64;
                        if nat_lt(a907, a908) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a909 = v0;
                        let a910 = 59u64;
                        if nat_lt(a909, a910) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a911 = v0;
                    let a912 = 62u64;
                    if nat_lt(a911, a912) {
                        let a913 = v0;
                        let a914 = 61u64;
                        if nat_lt(a913, a914) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a915 = v0;
                        let a916 = 63u64;
                        if nat_lt(a915, a916) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f85(v0: u32) -> u32 {
    let a919 = {
        let a917 = f94(v0, 6u32, 26u32);
        let a918 = f94(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a917, a918)
    };
    let a920 = f94(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a919, a920)
}

pub fn f86(v0: u32, v1: u32, v2: u32) -> u32 {
    let a926 = {
        let a921 = v0;
        let a922 = v1;
        fixed_u32::bit_and(a921, a922)
    };
    let a927 = {
        let a924 = {
            let a923 = v0;
            fixed_u32::bit_not(a923)
        };
        let a925 = v2;
        fixed_u32::bit_and(a924, a925)
    };
    fixed_u32::bit_xor(a926, a927)
}

pub fn f87(v0: u32) -> u32 {
    let a930 = {
        let a928 = f94(v0, 2u32, 30u32);
        let a929 = f94(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a928, a929)
    };
    let a931 = f94(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a930, a931)
}

pub fn f88(v0: u32, v1: u32, v2: u32) -> u32 {
    let a940 = {
        let a936 = {
            let a932 = v0;
            let a933 = v1;
            fixed_u32::bit_and(a932, a933)
        };
        let a937 = {
            let a934 = v0;
            let a935 = v2;
            fixed_u32::bit_and(a934, a935)
        };
        fixed_u32::bit_xor(a936, a937)
    };
    let a941 = {
        let a938 = v1;
        let a939 = v2;
        fixed_u32::bit_and(a938, a939)
    };
    fixed_u32::bit_xor(a940, a941)
}

pub fn f89(v0: Adt9) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(_, h942, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h943, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h943
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, h944, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h944
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h945, _, _, _, _, _, _, _, _, _, _, _) => {
            h945
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h946, _, _, _, _, _, _, _, _, _, _) => {
            h946
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h947, _, _, _, _, _, _, _, _, _) => {
            h947
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h948, _, _, _, _, _, _, _, _) => {
            h948
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h949, _, _, _, _, _, _, _) => {
            h949
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h950, _, _, _, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h951, _, _, _, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h952, _, _, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h953, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h954, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h955, _) => {
            h955
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h956) => {
            h956
        }
    }, f67(f95(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h957, _) => {
            h957
        }
    }), f67(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h958, _, _, _, _, _, _) => {
            h958
        }
    }, f67(f96(match v0.clone() {
        Adt9::C0(_, h959, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h959
        }
    }), match v0 {
        Adt9::C0(h960, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h960
        }
    }))))
}

pub fn f90(v0: Fn15, p964: u64, p965: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m963 = (p964, p965);
    {
        let mut m963 = m963;
        loop {
            let step = {
                let (v1, v2) = m963;
                let m961 = v1;
                if m961 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m961 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c962: Fn15 = v0.clone();
                        c962.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h966, _) = v0;
        h966
    };
    let v2: u32 = {
        let (h967, _) = {
            let (_, h968) = v0;
            h968
        };
        h967
    };
    let v3: u32 = {
        let (_, h969) = {
            let (_, h970) = v0;
            h970
        };
        h969
    };
    ({
        let a971 = v1;
        let a972 = 2u64;
        let a973 = 0u64;
        nat_quot(a971, a972, a973)
    }, (f92(v2, 1u32), {
        let a977 = {
            let a974 = v1;
            let a975 = 2u64;
            let a976 = 0u64;
            nat_rem(a974, a975, a976)
        };
        let a978 = 1u64;
        if nat_eq(a977, a978) {
            let a979 = v3;
            let a980 = v2;
            fixed_u32::bit_or(a979, a980)
        } else {
            v3
        }
    }))
}

pub fn f92(v0: u32, v1: u32) -> u32 {
    let m983 = {
        let a981 = v0;
        let a982 = v1;
        fixed_u32::shift_left(a981, a982)
    };
    match m983 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f93(v0: u64) -> Option<Adt5> {
    Some(Adt5::C0(v0, false, List::<Adt6>::nil()))
}

pub fn f94(v0: u32, v1: u32, v2: u32) -> u32 {
    let a984 = f48(v0, v1);
    let a985 = f92(v0, v2);
    fixed_u32::bit_or(a984, a985)
}

pub fn f95(v0: u32) -> u32 {
    let a988 = {
        let a986 = f94(v0, 17u32, 15u32);
        let a987 = f94(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a986, a987)
    };
    let a989 = f48(v0, 10u32);
    fixed_u32::bit_xor(a988, a989)
}

pub fn f96(v0: u32) -> u32 {
    let a992 = {
        let a990 = f94(v0, 7u32, 25u32);
        let a991 = f94(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a990, a991)
    };
    let a993 = f48(v0, 3u32);
    fixed_u32::bit_xor(a992, a993)
}
