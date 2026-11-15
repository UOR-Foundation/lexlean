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
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, List<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(bool, List<Adt2>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt12, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt14, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt5, u64, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u32, u32, u32, u32),
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
    F36,
    F77(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn4::F36 => Ok(f36(p0)),
            Fn4::F77(k0) => f77(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F38,
}

impl Fn5 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn5::F38 => f38(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F57(Bytes, u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Adt6)) -> R<(u64, Adt6)> {
        match self {
            Fn6::F57(k0, k1, k2) => f57(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F62(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<(u64, Adt4)> {
        match self {
            Fn7::F62(k0, k1, k2) => f62(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F68,
}

impl Fn8 {
    pub fn apply(&self, p0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
        match self {
            Fn8::F68 => f68(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F71(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn9::F71(k0) => f71(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F73,
}

impl Fn10 {
    pub fn apply(&self, p0: u64) -> Option<u64> {
        match self {
            Fn10::F73 => f73(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F75(Bytes, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt10, p1: (Bytes, Bytes)) -> Adt10 {
        match self {
            Fn11::F75(k0, k1) => f75(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F80(u64, Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
        match self {
            Fn12::F80(k0, k1) => f80(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F82(Adt2),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt11, p1: Adt2) -> Adt11 {
        match self {
            Fn13::F82(k0) => f82(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F89(Bytes, u64, u64, u64),
    F157,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Adt7)) -> R<(u64, Adt7)> {
        match self {
            Fn14::F89(k0, k1, k2, k3) => f89(k0.clone(), *k1, *k2, *k3, p0),
            Fn14::F157 => f157(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F96(Bytes, u64),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
        match self {
            Fn15::F96(k0, k1) => f96(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F107,
}

impl Fn16 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn16::F107 => f107(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F110,
}

impl Fn17 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn17::F110 => f110(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F112,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt13) -> R<Adt13> {
        match self {
            Fn18::F112 => f112(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F117,
}

impl Fn19 {
    pub fn apply(&self, p0: Adt15) -> R<Adt15> {
        match self {
            Fn19::F117 => f117(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F122,
}

impl Fn20 {
    pub fn apply(&self, p0: (Adt17, Adt16)) -> (Adt17, Adt16) {
        match self {
            Fn20::F122 => f122(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F124(u64, Bytes, u64),
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, Adt5)) -> R<(u64, Adt5)> {
        match self {
            Fn21::F124(k0, k1, k2) => f124(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F128,
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn22::F128 => f128(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F145,
}

impl Fn23 {
    pub fn apply(&self, p0: Adt18) -> Adt18 {
        match self {
            Fn23::F145 => f145(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F167,
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn24::F167 => f167(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v1)?;
    match m1 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            f2(v0, v2)
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

pub fn f2(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m14 = f10(v0.clone(), v1)?;
    match m14 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a15 = f11(v0);
                let a16 = v2;
                append_bytes(a15, a16)
            }))
        }
    }
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m20 = {
        let a17 = v0;
        let a18 = v1;
        let a19 = v2;
        slice_bytes(a17, a18, a19)
    };
    match m20 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f4(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h21) = f12(Fn2::F13(v1, v0), v2, (v2, 0u64))?;
    Ok(h21)
}

pub fn f5(v0: Fn0, p25: u64, p26: Adt0) -> R<Adt0> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m22 = v1;
                if m22 == 0 {
                    Err::<(u64, Adt0), R<Adt0>>(Ok(v2))
                } else {
                    let v3 = m22 - 1;
                    Ok::<(u64, Adt0), R<Adt0>>((v3, {
                        let c23: Fn0 = v0.clone();
                        c23.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: Adt0) -> R<Adt0> {
    f14(v0, v1)
}

pub fn f7(v0: Fn1, p30: u64, p31: Adt1) -> R<Adt1> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v1;
                if m27 == 0 {
                    Err::<(u64, Adt1), R<Adt1>>(Ok(v2))
                } else {
                    let v3 = m27 - 1;
                    Ok::<(u64, Adt1), R<Adt1>>((v3, {
                        let c28: Fn1 = v0.clone();
                        c28.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m29 = next; }
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
        Adt0::C0(_, _, h32, _) => {
            h32
        }
    };
    let v4: u64 = f16(match v2.clone() {
        Adt1::C0(h33, _, _) => {
            h33
        }
    }, v3)?;
    let v5: Adt3 = f17(Fn3::F18(v3, v4, v0.clone()), Adt3::C0(true, 0u64, Bytes::lit(&[])), match v2 {
        Adt1::C0(_, _, h34) => {
            h34
        }
    })?;
    if match v5.clone() {
        Adt3::C0(h35, _, _) => {
            h35
        }
    } {
        Ok(Some({
            let a42 = f3(v0, 0u64, 24u64);
            let a43 = {
                let a40 = f19(v3, 8u64)?;
                let a41 = {
                    let a38 = f20(match v1 {
                        Adt0::C0(_, _, _, h36) => {
                            h36
                        }
                    });
                    let a39 = match v5 {
                        Adt3::C0(_, _, h37) => {
                            h37
                        }
                    };
                    append_bytes(a38, a39)
                };
                append_bytes(a40, a41)
            };
            append_bytes(a42, a43)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f10(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f21(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f22(v1)?))
    } else {
        if f21(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f23(v1)?))
        } else {
            if f21(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f24(v1)?))
            } else {
                if f21(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f25(v1)?))
                } else {
                    if f21(v0, Str::lit("blake3")) {
                        Ok(Some(f26(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Bytes) -> Bytes {
    let a44 = v0;
    let a45 = f27(Str::lit(":"));
    append_bytes(a44, a45)
}

pub fn f12(v0: Fn2, p49: u64, p50: (u64, u64)) -> R<(u64, u64)> {
    let m48 = (p49, p50);
    {
        let mut m48 = m48;
        loop {
            let step = {
                let (v1, v2) = m48;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m46 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c47: Fn2 = v0.clone();
                        c47.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m48 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a52 = {
            let (h51, _) = v2;
            h51
        };
        let a53 = 1u64;
        nat_sub(a52, a53)
    };
    Ok((v3, {
        let a59 = {
            let a55 = {
                let (_, h54) = v2;
                h54
            };
            let a56 = 256u64;
            nat_mul(a55, a56)?
        };
        let a60 = f28(v1, {
            let a57 = v0;
            let a58 = v3;
            nat_add(a57, a58)?
        })?;
        nat_add(a59, a60)?
    }))
}

pub fn f14(v0: Bytes, v1: Adt0) -> R<Adt0> {
    if match v1.clone() {
        Adt0::C0(_, h61, _, _) => {
            h61
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt0::C0(h62, _, _, _) => {
                h62
            }
        };
        if f29(v0.clone(), v2, 8u64) {
            let v3: u64 = f4(v0.clone(), v2, 8u64)?;
            if if f29(v0.clone(), {
                let a63 = v2;
                let a64 = 8u64;
                nat_add(a63, a64)?
            }, v3) {
                f29(v0.clone(), {
                    let a67 = {
                        let a65 = v2;
                        let a66 = 8u64;
                        nat_add(a65, a66)?
                    };
                    let a68 = v3;
                    nat_add(a67, a68)?
                }, 4u64)
            } else {
                false
            } {
                f30(v0.clone(), v1, f3(v0.clone(), {
                    let a69 = v2;
                    let a70 = 8u64;
                    nat_add(a69, a70)?
                }, v3), f4(v0, {
                    let a73 = {
                        let a71 = v2;
                        let a72 = 8u64;
                        nat_add(a71, a72)?
                    };
                    let a74 = v3;
                    nat_add(a73, a74)?
                }, 4u64)?, {
                    let a79 = {
                        let a77 = {
                            let a75 = v2;
                            let a76 = 8u64;
                            nat_add(a75, a76)?
                        };
                        let a78 = v3;
                        nat_add(a77, a78)?
                    };
                    let a80 = 4u64;
                    nat_add(a79, a80)?
                })
            } else {
                Ok(f31(v1))
            }
        } else {
            Ok(f31(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f15(v0: Bytes, v1: Adt1) -> R<Adt1> {
    if match v1.clone() {
        Adt1::C0(_, h81, _) => {
            h81
        }
    } {
        let v2: u64 = match v1.clone() {
            Adt1::C0(h82, _, _) => {
                h82
            }
        };
        if f29(v0.clone(), v2, 8u64) {
            let v3: u64 = f4(v0.clone(), v2, 8u64)?;
            if if f29(v0.clone(), {
                let a83 = v2;
                let a84 = 8u64;
                nat_add(a83, a84)?
            }, v3) {
                f29(v0.clone(), {
                    let a87 = {
                        let a85 = v2;
                        let a86 = 8u64;
                        nat_add(a85, a86)?
                    };
                    let a88 = v3;
                    nat_add(a87, a88)?
                }, 4u64)
            } else {
                false
            } {
                f32(v0.clone(), v1, f3(v0, {
                    let a89 = v2;
                    let a90 = 8u64;
                    nat_add(a89, a90)?
                }, v3), {
                    let a93 = {
                        let a91 = v2;
                        let a92 = 8u64;
                        nat_add(a91, a92)?
                    };
                    let a94 = v3;
                    nat_add(a93, a94)?
                })
            } else {
                Ok(f33(v1))
            }
        } else {
            Ok(f33(v1))
        }
    } else {
        Ok(v1)
    }
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a95 = v0;
        let a96 = v1;
        let a97 = 0u64;
        nat_rem(a95, a96, a97)
    };
    let a98 = v2;
    let a99 = 0u64;
    if nat_eq(a98, a99) {
        Ok(v0)
    } else {
        let a102 = v0;
        let a103 = {
            let a100 = v1;
            let a101 = v2;
            nat_sub(a100, a101)
        };
        nat_add(a102, a103)
    }
}

pub fn f17(v0: Fn3, p107: Adt3, p108: List<Adt2>) -> R<Adt3> {
    let m106 = (p107, p108);
    {
        let mut m106 = m106;
        loop {
            let step = {
                let (v1, v2) = m106;
                let m104 = v2;
                match m104.uncons() {
                    None => {
                        Err::<(Adt3, List<Adt2>), R<Adt3>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<Adt2>), R<Adt3>>(({
                            let c105: Fn3 = v0.clone();
                            c105.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: Bytes, v3: Adt3, v4: Adt2) -> R<Adt3> {
    f34(v2, v1, v0, v3, v4)
}

pub fn f19(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h109) = f35(Fn4::F36, v1, (v0, Bytes::lit(&[])))?;
    Ok(h109)
}

pub fn f20(v0: List<(Bytes, Bytes)>) -> Bytes {
    f37(Fn5::F38, Bytes::lit(&[]), v0)
}

pub fn f21(v0: Bytes, v1: Str) -> bool {
    let a110 = v0;
    let a111 = f27(v1);
    equal(a110, a111)
}

pub fn f22(v0: Bytes) -> R<Bytes> {
    Ok(f39(f40(v0)?))
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    f41(v0, 6u64)
}

pub fn f24(v0: Bytes) -> R<Bytes> {
    f41(v0, 1u64)
}

pub fn f25(v0: Bytes) -> R<Bytes> {
    let v1: Adt4 = f42(v0)?;
    let a132 = f43(match v1.clone() {
        Adt4::C0(h112, _, _, _, _, _, _, _) => {
            h112
        }
    });
    let a133 = {
        let a130 = f43(match v1.clone() {
            Adt4::C0(_, h113, _, _, _, _, _, _) => {
                h113
            }
        });
        let a131 = {
            let a128 = f43(match v1.clone() {
                Adt4::C0(_, _, h114, _, _, _, _, _) => {
                    h114
                }
            });
            let a129 = {
                let a126 = f43(match v1.clone() {
                    Adt4::C0(_, _, _, h115, _, _, _, _) => {
                        h115
                    }
                });
                let a127 = {
                    let a124 = f43(match v1.clone() {
                        Adt4::C0(_, _, _, _, h116, _, _, _) => {
                            h116
                        }
                    });
                    let a125 = {
                        let a122 = f43(match v1.clone() {
                            Adt4::C0(_, _, _, _, _, h117, _, _) => {
                                h117
                            }
                        });
                        let a123 = {
                            let a120 = f43(match v1.clone() {
                                Adt4::C0(_, _, _, _, _, _, h118, _) => {
                                    h118
                                }
                            });
                            let a121 = f43(match v1 {
                                Adt4::C0(_, _, _, _, _, _, _, h119) => {
                                    h119
                                }
                            });
                            append_bytes(a120, a121)
                        };
                        append_bytes(a122, a123)
                    };
                    append_bytes(a124, a125)
                };
                append_bytes(a126, a127)
            };
            append_bytes(a128, a129)
        };
        append_bytes(a130, a131)
    };
    Ok(append_bytes(a132, a133))
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    let v1: Adt5 = f44(v0)?;
    let a154 = f45(match v1.clone() {
        Adt5::C0(h134, _, _, _, _, _, _, _) => {
            h134
        }
    });
    let a155 = {
        let a152 = f45(match v1.clone() {
            Adt5::C0(_, h135, _, _, _, _, _, _) => {
                h135
            }
        });
        let a153 = {
            let a150 = f45(match v1.clone() {
                Adt5::C0(_, _, h136, _, _, _, _, _) => {
                    h136
                }
            });
            let a151 = {
                let a148 = f45(match v1.clone() {
                    Adt5::C0(_, _, _, h137, _, _, _, _) => {
                        h137
                    }
                });
                let a149 = {
                    let a146 = f45(match v1.clone() {
                        Adt5::C0(_, _, _, _, h138, _, _, _) => {
                            h138
                        }
                    });
                    let a147 = {
                        let a144 = f45(match v1.clone() {
                            Adt5::C0(_, _, _, _, _, h139, _, _) => {
                                h139
                            }
                        });
                        let a145 = {
                            let a142 = f45(match v1.clone() {
                                Adt5::C0(_, _, _, _, _, _, h140, _) => {
                                    h140
                                }
                            });
                            let a143 = f45(match v1 {
                                Adt5::C0(_, _, _, _, _, _, _, h141) => {
                                    h141
                                }
                            });
                            append_bytes(a142, a143)
                        };
                        append_bytes(a144, a145)
                    };
                    append_bytes(a146, a147)
                };
                append_bytes(a148, a149)
            };
            append_bytes(a150, a151)
        };
        append_bytes(a152, a153)
    };
    Ok(append_bytes(a154, a155))
}

pub fn f27(v0: Str) -> Bytes {
    let a156 = v0;
    utf8_encode(a156)
}

pub fn f28(v0: Bytes, v1: u64) -> R<u64> {
    let m159 = {
        let a157 = v0;
        let a158 = v1;
        index_bytes(a157, a158)
    };
    match m159 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a204 = {
                let a180 = {
                    let a168 = {
                        let a162 = {
                            let a160 = v3;
                            let a161 = 128u8;
                            fixed_u8::bit_and(a160, a161)
                        };
                        let a163 = 128u8;
                        if equal(a162, a163) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a169 = {
                        let a166 = {
                            let a164 = v3;
                            let a165 = 64u8;
                            fixed_u8::bit_and(a164, a165)
                        };
                        let a167 = 64u8;
                        if equal(a166, a167) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a168, a169)?
                };
                let a181 = {
                    let a178 = {
                        let a172 = {
                            let a170 = v3;
                            let a171 = 32u8;
                            fixed_u8::bit_and(a170, a171)
                        };
                        let a173 = 32u8;
                        if equal(a172, a173) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a179 = {
                        let a176 = {
                            let a174 = v3;
                            let a175 = 16u8;
                            fixed_u8::bit_and(a174, a175)
                        };
                        let a177 = 16u8;
                        if equal(a176, a177) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a178, a179)?
                };
                nat_add(a180, a181)?
            };
            let a205 = {
                let a202 = {
                    let a190 = {
                        let a184 = {
                            let a182 = v3;
                            let a183 = 8u8;
                            fixed_u8::bit_and(a182, a183)
                        };
                        let a185 = 8u8;
                        if equal(a184, a185) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a191 = {
                        let a188 = {
                            let a186 = v3;
                            let a187 = 4u8;
                            fixed_u8::bit_and(a186, a187)
                        };
                        let a189 = 4u8;
                        if equal(a188, a189) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a190, a191)?
                };
                let a203 = {
                    let a200 = {
                        let a194 = {
                            let a192 = v3;
                            let a193 = 2u8;
                            fixed_u8::bit_and(a192, a193)
                        };
                        let a195 = 2u8;
                        if equal(a194, a195) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a201 = {
                        let a198 = {
                            let a196 = v3;
                            let a197 = 1u8;
                            fixed_u8::bit_and(a196, a197)
                        };
                        let a199 = 1u8;
                        if equal(a198, a199) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a200, a201)?
                };
                nat_add(a202, a203)?
            };
            nat_add(a204, a205)
        }
    }
}

pub fn f29(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a206 = v0;
        length_bytes(a206)
    };
    let a207 = v1;
    let a208 = v3;
    if nat_le(a207, a208) {
        let a211 = v2;
        let a212 = {
            let a209 = v3;
            let a210 = v1;
            nat_sub(a209, a210)
        };
        nat_le(a211, a212)
    } else {
        false
    }
}

pub fn f30(v0: Bytes, v1: Adt0, v2: Bytes, v3: u64, v4: u64) -> R<Adt0> {
    let v5: (bool, u64) = f46(v0.clone(), v4, v3)?;
    let (h213, _) = v5;
    if h213 {
        let v6: u64 = {
            let a214 = v2.clone();
            let a215 = f27(Str::lit("general.alignment"));
            if if equal(a214, a215) {
                let a216 = v3;
                let a217 = 4u64;
                nat_eq(a216, a217)
            } else {
                false
            } {
                f4(v0.clone(), v4, 4u64)?
            } else {
                match v1.clone() {
                    Adt0::C0(_, _, h218, _) => {
                        h218
                    }
                }
            }
        };
        let a219 = 8u64;
        let a220 = v6;
        if if nat_le(a219, a220) {
            f47(v6)
        } else {
            false
        } {
            Ok(Adt0::C0({
                let (_, h221) = v5;
                h221
            }, true, v6, f48(v2.clone(), {
                let a227 = f49(v2)?;
                let a228 = {
                    let a225 = f19(v3, 4u64)?;
                    let a226 = f50(v0, v3, v4, {
                        let a223 = {
                            let (_, h222) = v5;
                            h222
                        };
                        let a224 = v4;
                        nat_sub(a223, a224)
                    })?;
                    append_bytes(a225, a226)
                };
                append_bytes(a227, a228)
            }, match v1 {
                Adt0::C0(_, _, _, h229) => {
                    h229
                }
            })))
        } else {
            Ok(f31(v1))
        }
    } else {
        Ok(f31(v1))
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h230, _, _, _) => {
            h230
        }
    }, false, match v0 {
        Adt0::C0(_, _, h231, _) => {
            h231
        }
    }, List::<(Bytes, Bytes)>::nil())
}

pub fn f32(v0: Bytes, v1: Adt1, v2: Bytes, v3: u64) -> R<Adt1> {
    let v4: u64 = f4(v0.clone(), v3, 4u64)?;
    let a232 = v4;
    let a233 = 4u64;
    if if nat_le(a232, a233) {
        f29(v0.clone(), {
            let a234 = v3;
            let a235 = 4u64;
            nat_add(a234, a235)?
        }, {
            let a238 = {
                let a236 = v4;
                let a237 = 8u64;
                nat_mul(a236, a237)?
            };
            let a239 = 12u64;
            nat_add(a238, a239)?
        })
    } else {
        false
    } {
        let v5: (bool, u64) = f51(v0.clone(), {
            let a240 = v3;
            let a241 = 4u64;
            nat_add(a240, a241)?
        }, v4)?;
        let v6: u64 = {
            let a246 = {
                let a242 = v3;
                let a243 = 4u64;
                nat_add(a242, a243)?
            };
            let a247 = {
                let a244 = v4;
                let a245 = 8u64;
                nat_mul(a244, a245)?
            };
            nat_add(a246, a247)?
        };
        let v7: (u64, u64) = f52(f4(v0.clone(), v6, 4u64)?)?;
        let v8: u64 = {
            let (_, h248) = v7;
            h248
        };
        let (h249, _) = v5;
        if if h249 {
            let a250 = 0u64;
            let a251 = v8;
            if nat_lt(a250, a251) {
                let a256 = {
                    let a253 = {
                        let (_, h252) = v5;
                        h252
                    };
                    let a254 = v8;
                    let a255 = 0u64;
                    nat_rem(a253, a254, a255)
                };
                let a257 = 0u64;
                nat_eq(a256, a257)
            } else {
                false
            }
        } else {
            false
        } {
            let v9: u64 = {
                let a259 = {
                    let (_, h258) = v5;
                    h258
                };
                let a260 = v8;
                let a261 = 0u64;
                nat_quot(a259, a260, a261)
            };
            let a266 = v9;
            let a267 = {
                let a263 = 18446744073709551615u64;
                let a264 = {
                    let (h262, _) = v7;
                    h262
                };
                let a265 = 0u64;
                nat_quot(a263, a264, a265)
            };
            if nat_le(a266, a267) {
                Ok(Adt1::C0({
                    let a268 = v6;
                    let a269 = 12u64;
                    nat_add(a268, a269)?
                }, true, f53(Adt2::C0(v2, f3(v0.clone(), v3, {
                    let a272 = {
                        let a270 = v4;
                        let a271 = 8u64;
                        nat_mul(a270, a271)?
                    };
                    let a273 = 8u64;
                    nat_add(a272, a273)?
                }), f4(v0, {
                    let a274 = v6;
                    let a275 = 4u64;
                    nat_add(a274, a275)?
                }, 8u64)?, {
                    let a277 = v9;
                    let a278 = {
                        let (h276, _) = v7;
                        h276
                    };
                    nat_mul(a277, a278)?
                }), match v1 {
                    Adt1::C0(_, _, h279) => {
                        h279
                    }
                })))
            } else {
                Ok(f33(v1))
            }
        } else {
            Ok(f33(v1))
        }
    } else {
        Ok(f33(v1))
    }
}

pub fn f33(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0 {
        Adt1::C0(h280, _, _) => {
            h280
        }
    }, false, List::<Adt2>::nil())
}

pub fn f34(v0: Bytes, v1: u64, v2: u64, v3: Adt3, v4: Adt2) -> R<Adt3> {
    if if match v3.clone() {
        Adt3::C0(h281, _, _) => {
            h281
        }
    } {
        let a284 = match v4.clone() {
            Adt2::C0(_, _, h282, _) => {
                h282
            }
        };
        let a285 = {
            let a283 = v0.clone();
            length_bytes(a283)
        };
        if nat_le(a284, a285) {
            let a290 = match v3.clone() {
                Adt3::C0(_, h286, _) => {
                    h286
                }
            };
            let a291 = {
                let a288 = 18446744073709551615u64;
                let a289 = match v4.clone() {
                    Adt2::C0(_, _, _, h287) => {
                        h287
                    }
                };
                nat_sub(a288, a289)
            };
            nat_le(a290, a291)
        } else {
            false
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a293 = v1;
            let a294 = match v4.clone() {
                Adt2::C0(_, _, h292, _) => {
                    h292
                }
            };
            nat_add(a293, a294)?
        };
        if f29(v0.clone(), v5, match v4.clone() {
            Adt2::C0(_, _, _, h295) => {
                h295
            }
        }) {
            Ok(Adt3::C0(true, f16({
                let a298 = match v3.clone() {
                    Adt3::C0(_, h296, _) => {
                        h296
                    }
                };
                let a299 = match v4.clone() {
                    Adt2::C0(_, _, _, h297) => {
                        h297
                    }
                };
                nat_add(a298, a299)?
            }, v2)?, {
                let a311 = match v3.clone() {
                    Adt3::C0(_, _, h300) => {
                        h300
                    }
                };
                let a312 = {
                    let a309 = f49(match v4.clone() {
                        Adt2::C0(h301, _, _, _) => {
                            h301
                        }
                    })?;
                    let a310 = {
                        let a307 = match v4.clone() {
                            Adt2::C0(_, h302, _, _) => {
                                h302
                            }
                        };
                        let a308 = {
                            let a305 = f19(match v3 {
                                Adt3::C0(_, h303, _) => {
                                    h303
                                }
                            }, 8u64)?;
                            let a306 = f49(f3(v0, v5, match v4 {
                                Adt2::C0(_, _, _, h304) => {
                                    h304
                                }
                            }))?;
                            append_bytes(a305, a306)
                        };
                        append_bytes(a307, a308)
                    };
                    append_bytes(a309, a310)
                };
                append_bytes(a311, a312)
            }))
        } else {
            Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
        }
    } else {
        Ok(Adt3::C0(false, 0u64, Bytes::lit(&[])))
    }
}

pub fn f35(v0: Fn4, p316: u64, p317: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m315 = (p316, p317);
    {
        let mut m315 = m315;
        loop {
            let step = {
                let (v1, v2) = m315;
                let m313 = v1;
                if m313 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m313 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c314: Fn4 = v0.clone();
                        c314.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a319 = {
            let (h318, _) = v0.clone();
            h318
        };
        let a320 = 256u64;
        let a321 = 0u64;
        nat_quot(a319, a320, a321)
    }, {
        let a327 = {
            let (_, h322) = v0;
            h322
        };
        let a328 = f54({
            let a324 = {
                let (h323, _) = v0;
                h323
            };
            let a325 = 256u64;
            let a326 = 0u64;
            nat_rem(a324, a325, a326)
        });
        append_bytes(a327, a328)
    })
}

pub fn f37(v0: Fn5, p332: Bytes, p333: List<(Bytes, Bytes)>) -> Bytes {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m329 = v2;
                match m329.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c330: Fn5 = v0.clone();
                            c330.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    let a335 = v0;
    let a336 = {
        let (_, h334) = v1;
        h334
    };
    append_bytes(a335, a336)
}

pub fn f39(v0: Adt6) -> Bytes {
    let a357 = f55(match v0.clone() {
        Adt6::C0(h337, _, _, _, _, _, _, _) => {
            h337
        }
    });
    let a358 = {
        let a355 = f55(match v0.clone() {
            Adt6::C0(_, h338, _, _, _, _, _, _) => {
                h338
            }
        });
        let a356 = {
            let a353 = f55(match v0.clone() {
                Adt6::C0(_, _, h339, _, _, _, _, _) => {
                    h339
                }
            });
            let a354 = {
                let a351 = f55(match v0.clone() {
                    Adt6::C0(_, _, _, h340, _, _, _, _) => {
                        h340
                    }
                });
                let a352 = {
                    let a349 = f55(match v0.clone() {
                        Adt6::C0(_, _, _, _, h341, _, _, _) => {
                            h341
                        }
                    });
                    let a350 = {
                        let a347 = f55(match v0.clone() {
                            Adt6::C0(_, _, _, _, _, h342, _, _) => {
                                h342
                            }
                        });
                        let a348 = {
                            let a345 = f55(match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, h343, _) => {
                                    h343
                                }
                            });
                            let a346 = f55(match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, h344) => {
                                    h344
                                }
                            });
                            append_bytes(a345, a346)
                        };
                        append_bytes(a347, a348)
                    };
                    append_bytes(a349, a350)
                };
                append_bytes(a351, a352)
            };
            append_bytes(a353, a354)
        };
        append_bytes(a355, a356)
    };
    append_bytes(a357, a358)
}

pub fn f40(v0: Bytes) -> R<Adt6> {
    let v1: u64 = {
        let a359 = v0.clone();
        length_bytes(a359)
    };
    let v2: u64 = {
        let a365 = {
            let a362 = {
                let a360 = v1;
                let a361 = 72u64;
                nat_add(a360, a361)?
            };
            let a363 = 64u64;
            let a364 = 0u64;
            nat_quot(a362, a363, a364)
        };
        let a366 = 64u64;
        nat_mul(a365, a366)?
    };
    let (_, h367) = f56(Fn6::F57(v0, v1, v2), {
        let a368 = v2;
        let a369 = 64u64;
        let a370 = 0u64;
        nat_quot(a368, a369, a370)
    }, (0u64, f58()))?;
    Ok(h367)
}

pub fn f41(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt7 = f59(v0, v1)?;
    let a379 = f60(match v2.clone() {
        Adt7::C0(h371, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h371
        }
    });
    let a380 = {
        let a377 = f60(match v2.clone() {
            Adt7::C0(_, h372, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        });
        let a378 = {
            let a375 = f60(match v2.clone() {
                Adt7::C0(_, _, h373, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h373
                }
            });
            let a376 = f60(match v2 {
                Adt7::C0(_, _, _, h374, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h374
                }
            });
            append_bytes(a375, a376)
        };
        append_bytes(a377, a378)
    };
    Ok(append_bytes(a379, a380))
}

pub fn f42(v0: Bytes) -> R<Adt4> {
    let v1: u64 = {
        let a381 = v0.clone();
        length_bytes(a381)
    };
    let v2: u64 = {
        let a387 = {
            let a384 = {
                let a382 = v1;
                let a383 = 144u64;
                nat_add(a382, a383)?
            };
            let a385 = 128u64;
            let a386 = 0u64;
            nat_quot(a384, a385, a386)
        };
        let a388 = 128u64;
        nat_mul(a387, a388)?
    };
    let (_, h389) = f61(Fn7::F62(v0, v1, v2), {
        let a390 = v2;
        let a391 = 128u64;
        let a392 = 0u64;
        nat_quot(a390, a391, a392)
    }, (0u64, f63()))?;
    Ok(h389)
}

pub fn f43(v0: u64) -> Bytes {
    let a421 = f64(v0, 60u32);
    let a422 = {
        let a419 = f64(v0, 56u32);
        let a420 = {
            let a417 = f64(v0, 52u32);
            let a418 = {
                let a415 = f64(v0, 48u32);
                let a416 = {
                    let a413 = f64(v0, 44u32);
                    let a414 = {
                        let a411 = f64(v0, 40u32);
                        let a412 = {
                            let a409 = f64(v0, 36u32);
                            let a410 = {
                                let a407 = f64(v0, 32u32);
                                let a408 = {
                                    let a405 = f64(v0, 28u32);
                                    let a406 = {
                                        let a403 = f64(v0, 24u32);
                                        let a404 = {
                                            let a401 = f64(v0, 20u32);
                                            let a402 = {
                                                let a399 = f64(v0, 16u32);
                                                let a400 = {
                                                    let a397 = f64(v0, 12u32);
                                                    let a398 = {
                                                        let a395 = f64(v0, 8u32);
                                                        let a396 = {
                                                            let a393 = f64(v0, 4u32);
                                                            let a394 = f64(v0, 0u32);
                                                            append_bytes(a393, a394)
                                                        };
                                                        append_bytes(a395, a396)
                                                    };
                                                    append_bytes(a397, a398)
                                                };
                                                append_bytes(a399, a400)
                                            };
                                            append_bytes(a401, a402)
                                        };
                                        append_bytes(a403, a404)
                                    };
                                    append_bytes(a405, a406)
                                };
                                append_bytes(a407, a408)
                            };
                            append_bytes(a409, a410)
                        };
                        append_bytes(a411, a412)
                    };
                    append_bytes(a413, a414)
                };
                append_bytes(a415, a416)
            };
            append_bytes(a417, a418)
        };
        append_bytes(a419, a420)
    };
    append_bytes(a421, a422)
}

pub fn f44(v0: Bytes) -> R<Adt5> {
    let v1: u64 = {
        let a423 = v0.clone();
        length_bytes(a423)
    };
    let v2: u64 = {
        let a424 = v1;
        let a425 = 0u64;
        if nat_eq(a424, a425) {
            1u64
        } else {
            let a428 = {
                let a426 = v1;
                let a427 = 1023u64;
                nat_add(a426, a427)?
            };
            let a429 = 1024u64;
            let a430 = 0u64;
            nat_quot(a428, a429, a430)
        }
    };
    let v3: u64 = {
        let a431 = v2;
        let a432 = 1u64;
        nat_sub(a431, a432)
    };
    let m434 = {
        let (_, h433) = f65(v0.clone(), v1, v3)?;
        h433
    };
    match m434.uncons() {
        None => {
            f66(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h435) = f67(Fn8::F68, 64u64, (List::cons(v4, v5), f66(v0, v1, v3, false)?));
            Ok(h435)
        }
    }
}

pub fn f45(v0: u32) -> Bytes {
    let a448 = {
        let a436 = f69(v0, 4u32);
        let a437 = f69(v0, 0u32);
        append_bytes(a436, a437)
    };
    let a449 = {
        let a446 = {
            let a438 = f69(v0, 12u32);
            let a439 = f69(v0, 8u32);
            append_bytes(a438, a439)
        };
        let a447 = {
            let a444 = {
                let a440 = f69(v0, 20u32);
                let a441 = f69(v0, 16u32);
                append_bytes(a440, a441)
            };
            let a445 = {
                let a442 = f69(v0, 28u32);
                let a443 = f69(v0, 24u32);
                append_bytes(a442, a443)
            };
            append_bytes(a444, a445)
        };
        append_bytes(a446, a447)
    };
    append_bytes(a448, a449)
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let v3: Adt8 = {
        let (h450, _) = f70(Fn9::F71(v0.clone()), {
            let a454 = {
                let a452 = {
                    let a451 = v0;
                    length_bytes(a451)
                };
                let a453 = 2u64;
                nat_mul(a452, a453)?
            };
            let a455 = 4u64;
            nat_add(a454, a455)?
        }, Adt8::C0(v1, true, List::cons(Adt9::C0(v2, 1u64, 0u64), List::<Adt9>::nil())))?;
        h450
    };
    if match v3.clone() {
        Adt8::C0(_, h456, _) => {
            h456
        }
    } {
        let m458 = match v3.clone() {
            Adt8::C0(_, _, h457) => {
                h457
            }
        };
        match m458.uncons() {
            None => {
                Ok((true, match v3 {
                    Adt8::C0(h459, _, _) => {
                        h459
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

pub fn f47(v0: u64) -> bool {
    let a461 = {
        let (h460, _) = f72(Fn10::F73, 64u64, v0);
        h460
    };
    let a462 = 1u64;
    nat_eq(a461, a462)
}

pub fn f48(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt10 = f74(Fn11::F75(v1.clone(), v0.clone()), Adt10::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt10::C0(h463, _) => {
            h463
        }
    } {
        f76(match v3 {
            Adt10::C0(_, h464) => {
                h464
            }
        })
    } else {
        f76(List::cons((v0, v1), match v3 {
            Adt10::C0(_, h465) => {
                h465
            }
        }))
    }
}

pub fn f49(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f22(v0)?;
    let (_, h466) = f35(Fn4::F77(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h466)
}

pub fn f50(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Bytes> {
    let a467 = 0u64;
    let a468 = f78(v1);
    if nat_lt(a467, a468) {
        Ok(f3(v0, v2, v3))
    } else {
        let a469 = v1;
        let a470 = 8u64;
        if nat_eq(a469, a470) {
            let a475 = f3(v0.clone(), v2, 8u64);
            let a476 = f49(f3(v0, {
                let a471 = v2;
                let a472 = 8u64;
                nat_add(a471, a472)?
            }, {
                let a473 = v3;
                let a474 = 8u64;
                nat_sub(a473, a474)
            }))?;
            Ok(append_bytes(a475, a476))
        } else {
            let a481 = f3(v0.clone(), v2, 12u64);
            let a482 = f49(f3(v0, {
                let a477 = v2;
                let a478 = 12u64;
                nat_add(a477, a478)?
            }, {
                let a479 = v3;
                let a480 = 12u64;
                nat_sub(a479, a480)
            }))?;
            Ok(append_bytes(a481, a482))
        }
    }
}

pub fn f51(v0: Bytes, v1: u64, v2: u64) -> R<(bool, u64)> {
    let (_, h483) = f79(Fn12::F80(v1, v0), v2, (0u64, (true, 1u64)))?;
    Ok(h483)
}

pub fn f52(v0: u64) -> R<(u64, u64)> {
    let v1: Bytes = Bytes::lit(&[0x00, 0x04, 0x01, 0x00, 0x02, 0x01, 0x00, 0x12, 0x02, 0x00, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x02, 0x00, 0x18, 0x02, 0x00, 0x22, 0x02, 0x00, 0x24, 0x02, 0x00, 0x54, 0x03, 0x00, 0x6e, 0x03, 0x00, 0x90, 0x03, 0x00, 0xb0, 0x03, 0x00, 0xd2, 0x03, 0x01, 0x24, 0x03, 0x00, 0x42, 0x03, 0x00, 0x4a, 0x03, 0x00, 0x62, 0x03, 0x00, 0x32, 0x03, 0x00, 0x12, 0x02, 0x00, 0x6e, 0x03, 0x00, 0x52, 0x03, 0x00, 0x88, 0x03, 0x00, 0x01, 0x01, 0x00, 0x02, 0x01, 0x00, 0x04, 0x01, 0x00, 0x08, 0x01, 0x00, 0x08, 0x01, 0x00, 0x38, 0x03, 0x00, 0x02, 0x01]);
    let v2: u64 = {
        let a484 = v0;
        let a485 = 3u64;
        nat_mul(a484, a485)?
    };
    let v3: u64 = {
        let a486 = v0;
        let a487 = 31u64;
        if nat_lt(a486, a487) {
            f28(v1.clone(), {
                let a488 = v2;
                let a489 = 2u64;
                nat_add(a488, a489)?
            })?
        } else {
            0u64
        }
    };
    Ok(({
        let a494 = {
            let a490 = f28(v1.clone(), v2)?;
            let a491 = 256u64;
            nat_mul(a490, a491)?
        };
        let a495 = f28(v1, {
            let a492 = v2;
            let a493 = 1u64;
            nat_add(a492, a493)?
        })?;
        nat_add(a494, a495)?
    }, {
        let a496 = v3;
        let a497 = 1u64;
        if nat_eq(a496, a497) {
            1u64
        } else {
            let a498 = v3;
            let a499 = 2u64;
            if nat_eq(a498, a499) {
                32u64
            } else {
                let a500 = v3;
                let a501 = 3u64;
                if nat_eq(a500, a501) {
                    256u64
                } else {
                    0u64
                }
            }
        }
    }))
}

pub fn f53(v0: Adt2, v1: List<Adt2>) -> List<Adt2> {
    let v2: Adt11 = f81(Fn13::F82(v0.clone()), Adt11::C0(false, List::<Adt2>::nil()), v1);
    if match v2.clone() {
        Adt11::C0(h502, _) => {
            h502
        }
    } {
        f83(match v2 {
            Adt11::C0(_, h503) => {
                h503
            }
        })
    } else {
        f83(List::cons(v0, match v2 {
            Adt11::C0(_, h504) => {
                h504
            }
        }))
    }
}

pub fn f54(v0: u64) -> Bytes {
    f3(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f55(v0: u32) -> Bytes {
    let a533 = f84({
        let a505 = f85(v0, 28u32);
        let a506 = 15u32;
        fixed_u32::bit_and(a505, a506)
    });
    let a534 = {
        let a531 = f84({
            let a507 = f85(v0, 24u32);
            let a508 = 15u32;
            fixed_u32::bit_and(a507, a508)
        });
        let a532 = {
            let a529 = f84({
                let a509 = f85(v0, 20u32);
                let a510 = 15u32;
                fixed_u32::bit_and(a509, a510)
            });
            let a530 = {
                let a527 = f84({
                    let a511 = f85(v0, 16u32);
                    let a512 = 15u32;
                    fixed_u32::bit_and(a511, a512)
                });
                let a528 = {
                    let a525 = f84({
                        let a513 = f85(v0, 12u32);
                        let a514 = 15u32;
                        fixed_u32::bit_and(a513, a514)
                    });
                    let a526 = {
                        let a523 = f84({
                            let a515 = f85(v0, 8u32);
                            let a516 = 15u32;
                            fixed_u32::bit_and(a515, a516)
                        });
                        let a524 = {
                            let a521 = f84({
                                let a517 = f85(v0, 4u32);
                                let a518 = 15u32;
                                fixed_u32::bit_and(a517, a518)
                            });
                            let a522 = f84({
                                let a519 = v0;
                                let a520 = 15u32;
                                fixed_u32::bit_and(a519, a520)
                            });
                            append_bytes(a521, a522)
                        };
                        append_bytes(a523, a524)
                    };
                    append_bytes(a525, a526)
                };
                append_bytes(a527, a528)
            };
            append_bytes(a529, a530)
        };
        append_bytes(a531, a532)
    };
    append_bytes(a533, a534)
}

pub fn f56(v0: Fn6, p538: u64, p539: (u64, Adt6)) -> R<(u64, Adt6)> {
    let m537 = (p538, p539);
    {
        let mut m537 = m537;
        loop {
            let step = {
                let (v1, v2) = m537;
                let m535 = v1;
                if m535 == 0 {
                    Err::<(u64, (u64, Adt6)), R<(u64, Adt6)>>(Ok(v2))
                } else {
                    let v3 = m535 - 1;
                    Ok::<(u64, (u64, Adt6)), R<(u64, Adt6)>>((v3, {
                        let c536: Fn6 = v0.clone();
                        c536.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a541 = {
            let (h540, _) = v3.clone();
            h540
        };
        let a542 = 64u64;
        nat_add(a541, a542)?
    }, f86({
        let (_, h543) = v3;
        h543
    }, f87(v0, v1, v2, {
        let (h544, _) = v3;
        h544
    })?)?))
}

pub fn f58() -> Adt6 {
    Adt6::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f59(v0: Bytes, v1: u64) -> R<Adt7> {
    let v2: u64 = {
        let a545 = v0.clone();
        length_bytes(a545)
    };
    let v3: u64 = {
        let a551 = {
            let a549 = {
                let a546 = v2;
                let a547 = 136u64;
                let a548 = 0u64;
                nat_quot(a546, a547, a548)
            };
            let a550 = 1u64;
            nat_add(a549, a550)?
        };
        let a552 = 136u64;
        nat_mul(a551, a552)?
    };
    let (_, h553) = f88(Fn14::F89(v0, v2, v1, v3), {
        let a554 = v3;
        let a555 = 136u64;
        let a556 = 0u64;
        nat_quot(a554, a555, a556)
    }, (0u64, f90()))?;
    Ok(h553)
}

pub fn f60(v0: u64) -> Bytes {
    let a585 = {
        let a557 = f64(v0, 4u32);
        let a558 = f64(v0, 0u32);
        append_bytes(a557, a558)
    };
    let a586 = {
        let a583 = {
            let a559 = f64(v0, 12u32);
            let a560 = f64(v0, 8u32);
            append_bytes(a559, a560)
        };
        let a584 = {
            let a581 = {
                let a561 = f64(v0, 20u32);
                let a562 = f64(v0, 16u32);
                append_bytes(a561, a562)
            };
            let a582 = {
                let a579 = {
                    let a563 = f64(v0, 28u32);
                    let a564 = f64(v0, 24u32);
                    append_bytes(a563, a564)
                };
                let a580 = {
                    let a577 = {
                        let a565 = f64(v0, 36u32);
                        let a566 = f64(v0, 32u32);
                        append_bytes(a565, a566)
                    };
                    let a578 = {
                        let a575 = {
                            let a567 = f64(v0, 44u32);
                            let a568 = f64(v0, 40u32);
                            append_bytes(a567, a568)
                        };
                        let a576 = {
                            let a573 = {
                                let a569 = f64(v0, 52u32);
                                let a570 = f64(v0, 48u32);
                                append_bytes(a569, a570)
                            };
                            let a574 = {
                                let a571 = f64(v0, 60u32);
                                let a572 = f64(v0, 56u32);
                                append_bytes(a571, a572)
                            };
                            append_bytes(a573, a574)
                        };
                        append_bytes(a575, a576)
                    };
                    append_bytes(a577, a578)
                };
                append_bytes(a579, a580)
            };
            append_bytes(a581, a582)
        };
        append_bytes(a583, a584)
    };
    append_bytes(a585, a586)
}

pub fn f61(v0: Fn7, p590: u64, p591: (u64, Adt4)) -> R<(u64, Adt4)> {
    let m589 = (p590, p591);
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v1, v2) = m589;
                let m587 = v1;
                if m587 == 0 {
                    Err::<(u64, (u64, Adt4)), R<(u64, Adt4)>>(Ok(v2))
                } else {
                    let v3 = m587 - 1;
                    Ok::<(u64, (u64, Adt4)), R<(u64, Adt4)>>((v3, {
                        let c588: Fn7 = v0.clone();
                        c588.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt4)) -> R<(u64, Adt4)> {
    Ok(({
        let a593 = {
            let (h592, _) = v3.clone();
            h592
        };
        let a594 = 128u64;
        nat_add(a593, a594)?
    }, f91({
        let (_, h595) = v3;
        h595
    }, f92(v0, v1, v2, {
        let (h596, _) = v3;
        h596
    })?)?))
}

pub fn f63() -> Adt4 {
    Adt4::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f64(v0: u64, v1: u32) -> Bytes {
    f84(f93(f94(v0, v1)))
}

pub fn f65(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt5>)> {
    f95(Fn15::F96(v0, v1), v2, (0u64, List::<Adt5>::nil()))
}

pub fn f66(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt5> {
    let v4: u64 = {
        let a597 = 1024u64;
        let a598 = v2;
        nat_mul(a597, a598)?
    };
    let v5: u64 = f97(v1, v4);
    let v6: u64 = f98(v5)?;
    let v7: u64 = {
        let a599 = v6;
        let a600 = 1u64;
        nat_sub(a599, a600)
    };
    Ok(f99(f100(v0.clone(), v4, v2, v7)?, f101(v0, {
        let a603 = v4;
        let a604 = {
            let a601 = 64u64;
            let a602 = v7;
            nat_mul(a601, a602)?
        };
        nat_add(a603, a604)?
    })?, v2, f102({
        let a607 = v5;
        let a608 = {
            let a605 = 64u64;
            let a606 = v7;
            nat_mul(a605, a606)?
        };
        nat_sub(a607, a608)
    }), {
        let a611 = {
            let a609 = v7;
            let a610 = 0u64;
            if nat_eq(a609, a610) {
                3u32
            } else {
                2u32
            }
        };
        let a612 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a611, a612)
    }))
}

pub fn f67(v0: Fn8, p616: u64, p617: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m615 = (p616, p617);
    {
        let mut m615 = m615;
        loop {
            let step = {
                let (v1, v2) = m615;
                let m613 = v1;
                if m613 == 0 {
                    Err::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>(v2)
                } else {
                    let v3 = m613 - 1;
                    Ok::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>((v3, {
                        let c614: Fn8 = v0.clone();
                        c614.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    f103(v0)
}

pub fn f69(v0: u32, v1: u32) -> Bytes {
    f84(f85(v0, v1))
}

pub fn f70(v0: Fn9, p622: u64, p623: Adt8) -> R<(Adt8, bool)> {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m618 = v1;
                if m618 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m618 - 1;
                    let m620 = {
                        let c619: Fn9 = v0.clone();
                        c619.apply(v2.clone())?
                    };
                    match m620 {
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
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    f104(v0, v1)
}

pub fn f72(v0: Fn10, p628: u64, p629: u64) -> (u64, bool) {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, v2) = m627;
                let m624 = v1;
                if m624 == 0 {
                    Err::<(u64, u64), (u64, bool)>((v2, false))
                } else {
                    let v3 = m624 - 1;
                    let m626 = {
                        let c625: Fn10 = v0.clone();
                        c625.apply(v2)
                    };
                    match m626 {
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
                Ok(next) => { m627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: u64) -> Option<u64> {
    let a630 = 1u64;
    let a631 = v0;
    if if nat_lt(a630, a631) {
        let a635 = {
            let a632 = v0;
            let a633 = 2u64;
            let a634 = 0u64;
            nat_rem(a632, a633, a634)
        };
        let a636 = 0u64;
        nat_eq(a635, a636)
    } else {
        false
    } {
        Some({
            let a637 = v0;
            let a638 = 2u64;
            let a639 = 0u64;
            nat_quot(a637, a638, a639)
        })
    } else {
        None::<u64>
    }
}

pub fn f74(v0: Fn11, p643: Adt10, p644: List<(Bytes, Bytes)>) -> Adt10 {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m640 = v2;
                match m640.uncons() {
                    None => {
                        Err::<(Adt10, List<(Bytes, Bytes)>), Adt10>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt10, List<(Bytes, Bytes)>), Adt10>(({
                            let c641: Fn11 = v0.clone();
                            c641.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: Bytes, v2: Adt10, v3: (Bytes, Bytes)) -> Adt10 {
    if match v2.clone() {
        Adt10::C0(h645, _) => {
            h645
        }
    } {
        Adt10::C0(true, List::cons(v3, match v2 {
            Adt10::C0(_, h646) => {
                h646
            }
        }))
    } else {
        if f105(v1.clone(), {
            let (h647, _) = v3.clone();
            h647
        }) {
            Adt10::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt10::C0(_, h648) => {
                    h648
                }
            })))
        } else {
            Adt10::C0(false, List::cons(v3, match v2 {
                Adt10::C0(_, h649) => {
                    h649
                }
            }))
        }
    }
}

pub fn f76(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f106(Fn16::F107, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f77(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a651 = {
            let (h650, _) = v1.clone();
            h650
        };
        let a652 = 2u64;
        nat_mul(a651, a652)?
    };
    Ok(({
        let a654 = {
            let (h653, _) = v1;
            h653
        };
        let a655 = 1u64;
        nat_add(a654, a655)?
    }, {
        let a663 = {
            let (_, h656) = v1;
            h656
        };
        let a664 = f54({
            let a661 = {
                let a657 = f108(f28(v0.clone(), v2)?);
                let a658 = 16u64;
                nat_mul(a657, a658)?
            };
            let a662 = f108(f28(v0, {
                let a659 = v2;
                let a660 = 1u64;
                nat_add(a659, a660)?
            })?);
            nat_add(a661, a662)?
        });
        append_bytes(a663, a664)
    }))
}

pub fn f78(v0: u64) -> u64 {
    let a665 = v0;
    let a666 = 0u64;
    if if nat_eq(a665, a666) {
        true
    } else {
        let a667 = v0;
        let a668 = 1u64;
        if nat_eq(a667, a668) {
            true
        } else {
            let a669 = v0;
            let a670 = 7u64;
            nat_eq(a669, a670)
        }
    } {
        1u64
    } else {
        let a671 = v0;
        let a672 = 2u64;
        if if nat_eq(a671, a672) {
            true
        } else {
            let a673 = v0;
            let a674 = 3u64;
            nat_eq(a673, a674)
        } {
            2u64
        } else {
            let a675 = v0;
            let a676 = 4u64;
            if if nat_eq(a675, a676) {
                true
            } else {
                let a677 = v0;
                let a678 = 5u64;
                if nat_eq(a677, a678) {
                    true
                } else {
                    let a679 = v0;
                    let a680 = 6u64;
                    nat_eq(a679, a680)
                }
            } {
                4u64
            } else {
                let a681 = v0;
                let a682 = 10u64;
                if if nat_eq(a681, a682) {
                    true
                } else {
                    let a683 = v0;
                    let a684 = 11u64;
                    if nat_eq(a683, a684) {
                        true
                    } else {
                        let a685 = v0;
                        let a686 = 12u64;
                        nat_eq(a685, a686)
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

pub fn f79(v0: Fn12, p690: u64, p691: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let m689 = (p690, p691);
    {
        let mut m689 = m689;
        loop {
            let step = {
                let (v1, v2) = m689;
                let m687 = v1;
                if m687 == 0 {
                    Err::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>(Ok(v2))
                } else {
                    let v3 = m687 - 1;
                    Ok::<(u64, (u64, (bool, u64))), R<(u64, (bool, u64))>>((v3, {
                        let c688: Fn12 = v0.clone();
                        c688.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: u64, v1: Bytes, v2: (u64, (bool, u64))) -> R<(u64, (bool, u64))> {
    let v3: u64 = f4(v1, {
        let a695 = v0;
        let a696 = {
            let a693 = {
                let (h692, _) = v2;
                h692
            };
            let a694 = 8u64;
            nat_mul(a693, a694)?
        };
        nat_add(a695, a696)?
    }, 8u64)?;
    let v4: u64 = {
        let (_, h697) = {
            let (_, h698) = v2;
            h698
        };
        h697
    };
    let v5: bool = {
        let (h699, _) = {
            let (_, h700) = v2;
            h700
        };
        if h699 {
            let a701 = v3;
            let a702 = 0u64;
            if nat_eq(a701, a702) {
                true
            } else {
                let a706 = v4;
                let a707 = {
                    let a703 = 18446744073709551615u64;
                    let a704 = v3;
                    let a705 = 0u64;
                    nat_quot(a703, a704, a705)
                };
                nat_le(a706, a707)
            }
        } else {
            false
        }
    };
    Ok(({
        let a709 = {
            let (h708, _) = v2;
            h708
        };
        let a710 = 1u64;
        nat_add(a709, a710)?
    }, (v5, if v5 {
        let a711 = v4;
        let a712 = v3;
        nat_mul(a711, a712)?
    } else {
        0u64
    })))
}

pub fn f81(v0: Fn13, p716: Adt11, p717: List<Adt2>) -> Adt11 {
    let m715 = (p716, p717);
    {
        let mut m715 = m715;
        loop {
            let step = {
                let (v1, v2) = m715;
                let m713 = v2;
                match m713.uncons() {
                    None => {
                        Err::<(Adt11, List<Adt2>), Adt11>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt11, List<Adt2>), Adt11>(({
                            let c714: Fn13 = v0.clone();
                            c714.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Adt2, v1: Adt11, v2: Adt2) -> Adt11 {
    if match v1.clone() {
        Adt11::C0(h718, _) => {
            h718
        }
    } {
        Adt11::C0(true, List::cons(v2, match v1 {
            Adt11::C0(_, h719) => {
                h719
            }
        }))
    } else {
        if f105(match v0.clone() {
            Adt2::C0(h720, _, _, _) => {
                h720
            }
        }, match v2.clone() {
            Adt2::C0(h721, _, _, _) => {
                h721
            }
        }) {
            Adt11::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt11::C0(_, h722) => {
                    h722
                }
            })))
        } else {
            Adt11::C0(false, List::cons(v2, match v1 {
                Adt11::C0(_, h723) => {
                    h723
                }
            }))
        }
    }
}

pub fn f83(v0: List<Adt2>) -> List<Adt2> {
    f109(Fn17::F110, List::<Adt2>::nil(), v0)
}

pub fn f84(v0: u32) -> Bytes {
    let a726 = {
        let a724 = v0;
        let a725 = 8u32;
        fixed_u32::bit_and(a724, a725)
    };
    let a727 = 0u32;
    if equal(a726, a727) {
        let a730 = {
            let a728 = v0;
            let a729 = 4u32;
            fixed_u32::bit_and(a728, a729)
        };
        let a731 = 0u32;
        if equal(a730, a731) {
            let a734 = {
                let a732 = v0;
                let a733 = 2u32;
                fixed_u32::bit_and(a732, a733)
            };
            let a735 = 0u32;
            if equal(a734, a735) {
                let a738 = {
                    let a736 = v0;
                    let a737 = 1u32;
                    fixed_u32::bit_and(a736, a737)
                };
                let a739 = 0u32;
                if equal(a738, a739) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a742 = {
                    let a740 = v0;
                    let a741 = 1u32;
                    fixed_u32::bit_and(a740, a741)
                };
                let a743 = 0u32;
                if equal(a742, a743) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a746 = {
                let a744 = v0;
                let a745 = 2u32;
                fixed_u32::bit_and(a744, a745)
            };
            let a747 = 0u32;
            if equal(a746, a747) {
                let a750 = {
                    let a748 = v0;
                    let a749 = 1u32;
                    fixed_u32::bit_and(a748, a749)
                };
                let a751 = 0u32;
                if equal(a750, a751) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a754 = {
                    let a752 = v0;
                    let a753 = 1u32;
                    fixed_u32::bit_and(a752, a753)
                };
                let a755 = 0u32;
                if equal(a754, a755) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a758 = {
            let a756 = v0;
            let a757 = 4u32;
            fixed_u32::bit_and(a756, a757)
        };
        let a759 = 0u32;
        if equal(a758, a759) {
            let a762 = {
                let a760 = v0;
                let a761 = 2u32;
                fixed_u32::bit_and(a760, a761)
            };
            let a763 = 0u32;
            if equal(a762, a763) {
                let a766 = {
                    let a764 = v0;
                    let a765 = 1u32;
                    fixed_u32::bit_and(a764, a765)
                };
                let a767 = 0u32;
                if equal(a766, a767) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a770 = {
                    let a768 = v0;
                    let a769 = 1u32;
                    fixed_u32::bit_and(a768, a769)
                };
                let a771 = 0u32;
                if equal(a770, a771) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a774 = {
                let a772 = v0;
                let a773 = 2u32;
                fixed_u32::bit_and(a772, a773)
            };
            let a775 = 0u32;
            if equal(a774, a775) {
                let a778 = {
                    let a776 = v0;
                    let a777 = 1u32;
                    fixed_u32::bit_and(a776, a777)
                };
                let a779 = 0u32;
                if equal(a778, a779) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a782 = {
                    let a780 = v0;
                    let a781 = 1u32;
                    fixed_u32::bit_and(a780, a781)
                };
                let a783 = 0u32;
                if equal(a782, a783) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f85(v0: u32, v1: u32) -> u32 {
    let m786 = {
        let a784 = v0;
        let a785 = v1;
        fixed_u32::shift_right(a784, a785)
    };
    match m786 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f86(v0: Adt6, v1: Adt12) -> R<Adt6> {
    let v2: Adt13 = f111(Fn18::F112, 64u64, Adt13::C0(match v0.clone() {
        Adt6::C0(h787, _, _, _, _, _, _, _) => {
            h787
        }
    }, match v0.clone() {
        Adt6::C0(_, h788, _, _, _, _, _, _) => {
            h788
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h789, _, _, _, _, _) => {
            h789
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h790, _, _, _, _) => {
            h790
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h791, _, _, _) => {
            h791
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h792, _, _) => {
            h792
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h793, _) => {
            h793
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h794) => {
            h794
        }
    }, v1, 0u64))?;
    Ok(Adt6::C0(f113(match v0.clone() {
        Adt6::C0(h795, _, _, _, _, _, _, _) => {
            h795
        }
    }, match v2.clone() {
        Adt13::C0(h796, _, _, _, _, _, _, _, _, _) => {
            h796
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, h797, _, _, _, _, _, _) => {
            h797
        }
    }, match v2.clone() {
        Adt13::C0(_, h798, _, _, _, _, _, _, _, _) => {
            h798
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, _, h799, _, _, _, _, _) => {
            h799
        }
    }, match v2.clone() {
        Adt13::C0(_, _, h800, _, _, _, _, _, _, _) => {
            h800
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, _, _, h801, _, _, _, _) => {
            h801
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, h802, _, _, _, _, _, _) => {
            h802
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, _, _, _, h803, _, _, _) => {
            h803
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, h804, _, _, _, _, _) => {
            h804
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h805, _, _) => {
            h805
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, _, h806, _, _, _, _) => {
            h806
        }
    }), f113(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h807, _) => {
            h807
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, _, _, h808, _, _, _) => {
            h808
        }
    }), f113(match v0 {
        Adt6::C0(_, _, _, _, _, _, _, h809) => {
            h809
        }
    }, match v2 {
        Adt13::C0(_, _, _, _, _, _, _, h810, _, _) => {
            h810
        }
    })))
}

pub fn f87(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt12> {
    Ok(Adt12::C0(f114(v0.clone(), v1, v2, v3)?, f114(v0.clone(), v1, v2, {
        let a811 = v3;
        let a812 = 4u64;
        nat_add(a811, a812)?
    })?, f114(v0.clone(), v1, v2, {
        let a813 = v3;
        let a814 = 8u64;
        nat_add(a813, a814)?
    })?, f114(v0.clone(), v1, v2, {
        let a815 = v3;
        let a816 = 12u64;
        nat_add(a815, a816)?
    })?, f114(v0.clone(), v1, v2, {
        let a817 = v3;
        let a818 = 16u64;
        nat_add(a817, a818)?
    })?, f114(v0.clone(), v1, v2, {
        let a819 = v3;
        let a820 = 20u64;
        nat_add(a819, a820)?
    })?, f114(v0.clone(), v1, v2, {
        let a821 = v3;
        let a822 = 24u64;
        nat_add(a821, a822)?
    })?, f114(v0.clone(), v1, v2, {
        let a823 = v3;
        let a824 = 28u64;
        nat_add(a823, a824)?
    })?, f114(v0.clone(), v1, v2, {
        let a825 = v3;
        let a826 = 32u64;
        nat_add(a825, a826)?
    })?, f114(v0.clone(), v1, v2, {
        let a827 = v3;
        let a828 = 36u64;
        nat_add(a827, a828)?
    })?, f114(v0.clone(), v1, v2, {
        let a829 = v3;
        let a830 = 40u64;
        nat_add(a829, a830)?
    })?, f114(v0.clone(), v1, v2, {
        let a831 = v3;
        let a832 = 44u64;
        nat_add(a831, a832)?
    })?, f114(v0.clone(), v1, v2, {
        let a833 = v3;
        let a834 = 48u64;
        nat_add(a833, a834)?
    })?, f114(v0.clone(), v1, v2, {
        let a835 = v3;
        let a836 = 52u64;
        nat_add(a835, a836)?
    })?, f114(v0.clone(), v1, v2, {
        let a837 = v3;
        let a838 = 56u64;
        nat_add(a837, a838)?
    })?, f114(v0, v1, v2, {
        let a839 = v3;
        let a840 = 60u64;
        nat_add(a839, a840)?
    })?))
}

pub fn f88(v0: Fn14, p844: u64, p845: (u64, Adt7)) -> R<(u64, Adt7)> {
    let m843 = (p844, p845);
    {
        let mut m843 = m843;
        loop {
            let step = {
                let (v1, v2) = m843;
                let m841 = v1;
                if m841 == 0 {
                    Err::<(u64, (u64, Adt7)), R<(u64, Adt7)>>(Ok(v2))
                } else {
                    let v3 = m841 - 1;
                    Ok::<(u64, (u64, Adt7)), R<(u64, Adt7)>>((v3, {
                        let c842: Fn14 = v0.clone();
                        c842.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt7)) -> R<(u64, Adt7)> {
    Ok(({
        let a847 = {
            let (h846, _) = v4.clone();
            h846
        };
        let a848 = 136u64;
        nat_add(a847, a848)?
    }, f115(v0, v1, v3, v2, {
        let (_, h849) = v4;
        h849
    }, {
        let (h850, _) = v4;
        h850
    })?))
}

pub fn f90() -> Adt7 {
    Adt7::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f91(v0: Adt4, v1: Adt14) -> R<Adt4> {
    let v2: Adt15 = f116(Fn19::F117, 80u64, Adt15::C0(match v0.clone() {
        Adt4::C0(h851, _, _, _, _, _, _, _) => {
            h851
        }
    }, match v0.clone() {
        Adt4::C0(_, h852, _, _, _, _, _, _) => {
            h852
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h853, _, _, _, _, _) => {
            h853
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h854, _, _, _, _) => {
            h854
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h855, _, _, _) => {
            h855
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h856, _, _) => {
            h856
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h857, _) => {
            h857
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h858) => {
            h858
        }
    }, v1, 0u64))?;
    Ok(Adt4::C0(f118(match v0.clone() {
        Adt4::C0(h859, _, _, _, _, _, _, _) => {
            h859
        }
    }, match v2.clone() {
        Adt15::C0(h860, _, _, _, _, _, _, _, _, _) => {
            h860
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, h861, _, _, _, _, _, _) => {
            h861
        }
    }, match v2.clone() {
        Adt15::C0(_, h862, _, _, _, _, _, _, _, _) => {
            h862
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, _, h863, _, _, _, _, _) => {
            h863
        }
    }, match v2.clone() {
        Adt15::C0(_, _, h864, _, _, _, _, _, _, _) => {
            h864
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, _, _, h865, _, _, _, _) => {
            h865
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, h866, _, _, _, _, _, _) => {
            h866
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, _, _, _, h867, _, _, _) => {
            h867
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, h868, _, _, _, _, _) => {
            h868
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, _, _, _, _, h869, _, _) => {
            h869
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, _, h870, _, _, _, _) => {
            h870
        }
    }), f118(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h871, _) => {
            h871
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, _, _, h872, _, _, _) => {
            h872
        }
    }), f118(match v0 {
        Adt4::C0(_, _, _, _, _, _, _, h873) => {
            h873
        }
    }, match v2 {
        Adt15::C0(_, _, _, _, _, _, _, h874, _, _) => {
            h874
        }
    })))
}

pub fn f92(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt14> {
    Ok(Adt14::C0(f119(v0.clone(), v1, v2, {
        let a875 = v3;
        let a876 = 0u64;
        nat_add(a875, a876)?
    })?, f119(v0.clone(), v1, v2, {
        let a877 = v3;
        let a878 = 8u64;
        nat_add(a877, a878)?
    })?, f119(v0.clone(), v1, v2, {
        let a879 = v3;
        let a880 = 16u64;
        nat_add(a879, a880)?
    })?, f119(v0.clone(), v1, v2, {
        let a881 = v3;
        let a882 = 24u64;
        nat_add(a881, a882)?
    })?, f119(v0.clone(), v1, v2, {
        let a883 = v3;
        let a884 = 32u64;
        nat_add(a883, a884)?
    })?, f119(v0.clone(), v1, v2, {
        let a885 = v3;
        let a886 = 40u64;
        nat_add(a885, a886)?
    })?, f119(v0.clone(), v1, v2, {
        let a887 = v3;
        let a888 = 48u64;
        nat_add(a887, a888)?
    })?, f119(v0.clone(), v1, v2, {
        let a889 = v3;
        let a890 = 56u64;
        nat_add(a889, a890)?
    })?, f119(v0.clone(), v1, v2, {
        let a891 = v3;
        let a892 = 64u64;
        nat_add(a891, a892)?
    })?, f119(v0.clone(), v1, v2, {
        let a893 = v3;
        let a894 = 72u64;
        nat_add(a893, a894)?
    })?, f119(v0.clone(), v1, v2, {
        let a895 = v3;
        let a896 = 80u64;
        nat_add(a895, a896)?
    })?, f119(v0.clone(), v1, v2, {
        let a897 = v3;
        let a898 = 88u64;
        nat_add(a897, a898)?
    })?, f119(v0.clone(), v1, v2, {
        let a899 = v3;
        let a900 = 96u64;
        nat_add(a899, a900)?
    })?, f119(v0.clone(), v1, v2, {
        let a901 = v3;
        let a902 = 104u64;
        nat_add(a901, a902)?
    })?, f119(v0.clone(), v1, v2, {
        let a903 = v3;
        let a904 = 112u64;
        nat_add(a903, a904)?
    })?, f119(v0, v1, v2, {
        let a905 = v3;
        let a906 = 120u64;
        nat_add(a905, a906)?
    })?))
}

pub fn f93(v0: u64) -> u32 {
    let m910 = {
        let a909 = {
            let a907 = v0;
            let a908 = 4294967295u64;
            fixed_u64::bit_and(a907, a908)
        };
        fixed_u32::convert(i128::from(a909))
    };
    match m910 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f94(v0: u64, v1: u32) -> u64 {
    let m913 = {
        let a911 = v0;
        let a912 = v1;
        fixed_u64::shift_right(a911, a912)
    };
    match m913 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f95(v0: Fn15, p917: u64, p918: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    let m916 = (p917, p918);
    {
        let mut m916 = m916;
        loop {
            let step = {
                let (v1, v2) = m916;
                let m914 = v1;
                if m914 == 0 {
                    Err::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>(Ok(v2))
                } else {
                    let v3 = m914 - 1;
                    Ok::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>((v3, {
                        let c915: Fn15 = v0.clone();
                        c915.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64, v2: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    f120(v2.clone(), f66(v0, v1, {
        let (h919, _) = v2;
        h919
    }, false)?)
}

pub fn f97(v0: u64, v1: u64) -> u64 {
    let a922 = 1024u64;
    let a923 = {
        let a920 = v0;
        let a921 = v1;
        nat_sub(a920, a921)
    };
    if nat_lt(a922, a923) {
        1024u64
    } else {
        let a924 = v0;
        let a925 = v1;
        nat_sub(a924, a925)
    }
}

pub fn f98(v0: u64) -> R<u64> {
    let a926 = v0;
    let a927 = 0u64;
    if nat_eq(a926, a927) {
        Ok(1u64)
    } else {
        let a930 = {
            let a928 = v0;
            let a929 = 63u64;
            nat_add(a928, a929)?
        };
        let a931 = 64u64;
        let a932 = 0u64;
        Ok(nat_quot(a930, a931, a932))
    }
}

pub fn f99(v0: Adt5, v1: Adt16, v2: u64, v3: u32, v4: u32) -> Adt5 {
    let v5: Adt17 = {
        let (h933, _) = f121(Fn20::F122, 7u64, (Adt17::C0(match v0.clone() {
            Adt5::C0(h934, _, _, _, _, _, _, _) => {
                h934
            }
        }, match v0.clone() {
            Adt5::C0(_, h935, _, _, _, _, _, _) => {
                h935
            }
        }, match v0.clone() {
            Adt5::C0(_, _, h936, _, _, _, _, _) => {
                h936
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, h937, _, _, _, _) => {
                h937
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, h938, _, _, _) => {
                h938
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, h939, _, _) => {
                h939
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, h940, _) => {
                h940
            }
        }, match v0 {
            Adt5::C0(_, _, _, _, _, _, _, h941) => {
                h941
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f102({
            let a942 = v2;
            let a943 = 4294967296u64;
            let a944 = 0u64;
            nat_rem(a942, a943, a944)
        }), f102({
            let a945 = v2;
            let a946 = 4294967296u64;
            let a947 = 0u64;
            nat_quot(a945, a946, a947)
        }), v3, v4), v1));
        h933
    };
    Adt5::C0({
        let a950 = match v5.clone() {
            Adt17::C0(h948, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a951 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, h949, _, _, _, _, _, _, _) => {
                h949
            }
        };
        fixed_u32::bit_xor(a950, a951)
    }, {
        let a954 = match v5.clone() {
            Adt17::C0(_, h952, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        let a955 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, h953, _, _, _, _, _, _) => {
                h953
            }
        };
        fixed_u32::bit_xor(a954, a955)
    }, {
        let a958 = match v5.clone() {
            Adt17::C0(_, _, h956, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h956
            }
        };
        let a959 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, h957, _, _, _, _, _) => {
                h957
            }
        };
        fixed_u32::bit_xor(a958, a959)
    }, {
        let a962 = match v5.clone() {
            Adt17::C0(_, _, _, h960, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h960
            }
        };
        let a963 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h961, _, _, _, _) => {
                h961
            }
        };
        fixed_u32::bit_xor(a962, a963)
    }, {
        let a966 = match v5.clone() {
            Adt17::C0(_, _, _, _, h964, _, _, _, _, _, _, _, _, _, _, _) => {
                h964
            }
        };
        let a967 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h965, _, _, _) => {
                h965
            }
        };
        fixed_u32::bit_xor(a966, a967)
    }, {
        let a970 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, h968, _, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a971 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h969, _, _) => {
                h969
            }
        };
        fixed_u32::bit_xor(a970, a971)
    }, {
        let a974 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, h972, _, _, _, _, _, _, _, _, _) => {
                h972
            }
        };
        let a975 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h973, _) => {
                h973
            }
        };
        fixed_u32::bit_xor(a974, a975)
    }, {
        let a978 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, h976, _, _, _, _, _, _, _, _) => {
                h976
            }
        };
        let a979 = match v5 {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h977) => {
                h977
            }
        };
        fixed_u32::bit_xor(a978, a979)
    })
}

pub fn f100(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let (_, h980) = f123(Fn21::F124(v2, v0, v1), v3, (0u64, f125()))?;
    Ok(h980)
}

pub fn f101(v0: Bytes, v1: u64) -> R<Adt16> {
    Ok(Adt16::C0(f126(v0.clone(), {
        let a981 = v1;
        let a982 = 0u64;
        nat_add(a981, a982)?
    })?, f126(v0.clone(), {
        let a983 = v1;
        let a984 = 4u64;
        nat_add(a983, a984)?
    })?, f126(v0.clone(), {
        let a985 = v1;
        let a986 = 8u64;
        nat_add(a985, a986)?
    })?, f126(v0.clone(), {
        let a987 = v1;
        let a988 = 12u64;
        nat_add(a987, a988)?
    })?, f126(v0.clone(), {
        let a989 = v1;
        let a990 = 16u64;
        nat_add(a989, a990)?
    })?, f126(v0.clone(), {
        let a991 = v1;
        let a992 = 20u64;
        nat_add(a991, a992)?
    })?, f126(v0.clone(), {
        let a993 = v1;
        let a994 = 24u64;
        nat_add(a993, a994)?
    })?, f126(v0.clone(), {
        let a995 = v1;
        let a996 = 28u64;
        nat_add(a995, a996)?
    })?, f126(v0.clone(), {
        let a997 = v1;
        let a998 = 32u64;
        nat_add(a997, a998)?
    })?, f126(v0.clone(), {
        let a999 = v1;
        let a1000 = 36u64;
        nat_add(a999, a1000)?
    })?, f126(v0.clone(), {
        let a1001 = v1;
        let a1002 = 40u64;
        nat_add(a1001, a1002)?
    })?, f126(v0.clone(), {
        let a1003 = v1;
        let a1004 = 44u64;
        nat_add(a1003, a1004)?
    })?, f126(v0.clone(), {
        let a1005 = v1;
        let a1006 = 48u64;
        nat_add(a1005, a1006)?
    })?, f126(v0.clone(), {
        let a1007 = v1;
        let a1008 = 52u64;
        nat_add(a1007, a1008)?
    })?, f126(v0.clone(), {
        let a1009 = v1;
        let a1010 = 56u64;
        nat_add(a1009, a1010)?
    })?, f126(v0, {
        let a1011 = v1;
        let a1012 = 60u64;
        nat_add(a1011, a1012)?
    })?))
}

pub fn f102(v0: u64) -> u32 {
    let (_, h1013) = {
        let (_, h1014) = f127(Fn22::F128, 32u64, (v0, (1u32, 0u32)));
        h1014
    };
    h1013
}

pub fn f103(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m1016 = {
        let (h1015, _) = v0.clone();
        h1015
    };
    match m1016.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f129(v1, {
                let (_, h1017) = v0;
                h1017
            }, {
                let m1018 = v2;
                match m1018.uncons() {
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

pub fn f104(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    if match v1.clone() {
        Adt8::C0(_, h1019, _) => {
            h1019
        }
    } {
        let m1021 = match v1.clone() {
            Adt8::C0(_, _, h1020) => {
                h1020
            }
        };
        match m1021.uncons() {
            None => {
                Ok(None::<Adt8>)
            }
            Some((v2, v3)) => {
                let a1023 = match v2.clone() {
                    Adt9::C0(_, h1022, _) => {
                        h1022
                    }
                };
                let a1024 = 0u64;
                if nat_eq(a1023, a1024) {
                    Ok(f130(match v1 {
                        Adt8::C0(h1025, _, _) => {
                            h1025
                        }
                    }, v3))
                } else {
                    f131(v0, match v1 {
                        Adt8::C0(h1026, _, _) => {
                            h1026
                        }
                    }, v2.clone(), List::cons(Adt9::C0(match v2.clone() {
                        Adt9::C0(h1027, _, _) => {
                            h1027
                        }
                    }, {
                        let a1029 = match v2.clone() {
                            Adt9::C0(_, h1028, _) => {
                                h1028
                            }
                        };
                        let a1030 = 1u64;
                        nat_sub(a1029, a1030)
                    }, match v2 {
                        Adt9::C0(_, _, h1031) => {
                            h1031
                        }
                    }), v3))
                }
            }
        }
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f105(v0: Bytes, v1: Bytes) -> bool {
    let a1036 = {
        let a1032 = v0;
        let a1033 = v1;
        compare_bytes(a1032, a1033)
    };
    let a1037 = {
        let a1034 = Bytes::lit(&[]);
        let a1035 = Bytes::lit(&[0x01]);
        compare_bytes(a1034, a1035)
    };
    equal(a1036, a1037)
}

pub fn f106(v0: Fn16, p1041: List<(Bytes, Bytes)>, p1042: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1040 = (p1041, p1042);
    {
        let mut m1040 = m1040;
        loop {
            let step = {
                let (v1, v2) = m1040;
                let m1038 = v2;
                match m1038.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1039: Fn16 = v0.clone();
                            c1039.apply(v1, v3)
                        }, v4))
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

pub fn f107(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f108(v0: u64) -> u64 {
    let a1043 = 47u64;
    let a1044 = v0;
    if nat_lt(a1043, a1044) {
        let a1045 = v0;
        let a1046 = 58u64;
        if nat_lt(a1045, a1046) {
            let a1047 = v0;
            let a1048 = 48u64;
            nat_sub(a1047, a1048)
        } else {
            let a1049 = 64u64;
            let a1050 = v0;
            if nat_lt(a1049, a1050) {
                let a1051 = v0;
                let a1052 = 71u64;
                if nat_lt(a1051, a1052) {
                    let a1053 = v0;
                    let a1054 = 55u64;
                    nat_sub(a1053, a1054)
                } else {
                    let a1055 = 96u64;
                    let a1056 = v0;
                    if nat_lt(a1055, a1056) {
                        let a1057 = v0;
                        let a1058 = 103u64;
                        if nat_lt(a1057, a1058) {
                            let a1059 = v0;
                            let a1060 = 87u64;
                            nat_sub(a1059, a1060)
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

pub fn f109(v0: Fn17, p1064: List<Adt2>, p1065: List<Adt2>) -> List<Adt2> {
    let m1063 = (p1064, p1065);
    {
        let mut m1063 = m1063;
        loop {
            let step = {
                let (v1, v2) = m1063;
                let m1061 = v2;
                match m1061.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c1062: Fn17 = v0.clone();
                            c1062.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f111(v0: Fn18, p1069: u64, p1070: Adt13) -> R<Adt13> {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v1, v2) = m1068;
                let m1066 = v1;
                if m1066 == 0 {
                    Err::<(u64, Adt13), R<Adt13>>(Ok(v2))
                } else {
                    let v3 = m1066 - 1;
                    Ok::<(u64, Adt13), R<Adt13>>((v3, {
                        let c1067: Fn18 = v0.clone();
                        c1067.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Adt13) -> R<Adt13> {
    f132(v0)
}

pub fn f113(v0: u32, v1: u32) -> u32 {
    let m1073 = {
        let a1071 = f133(v0);
        let a1072 = f133(v1);
        fixed_u64::checked_add(a1071, a1072)
    };
    match m1073 {
        None => {
            0u32
        }
        Some(v2) => {
            f93(v2)
        }
    }
}

pub fn f114(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1076 = {
        let a1074 = v3;
        let a1075 = 8u64;
        nat_add(a1074, a1075)?
    };
    let a1077 = v2;
    if nat_eq(a1076, a1077) {
        Ok(f102({
            let a1080 = {
                let a1078 = v1;
                let a1079 = 8u64;
                nat_mul(a1078, a1079)?
            };
            let a1081 = 4294967296u64;
            let a1082 = 0u64;
            nat_quot(a1080, a1081, a1082)
        }))
    } else {
        let a1085 = {
            let a1083 = v3;
            let a1084 = 4u64;
            nat_add(a1083, a1084)?
        };
        let a1086 = v2;
        if nat_eq(a1085, a1086) {
            Ok(f102({
                let a1089 = {
                    let a1087 = v1;
                    let a1088 = 8u64;
                    nat_mul(a1087, a1088)?
                };
                let a1090 = 4294967296u64;
                let a1091 = 0u64;
                nat_rem(a1089, a1090, a1091)
            }))
        } else {
            Ok(f134(f135(v0.clone(), v1, v3), f135(v0.clone(), v1, {
                let a1092 = v3;
                let a1093 = 1u64;
                nat_add(a1092, a1093)?
            }), f135(v0.clone(), v1, {
                let a1094 = v3;
                let a1095 = 2u64;
                nat_add(a1094, a1095)?
            }), f135(v0, v1, {
                let a1096 = v3;
                let a1097 = 3u64;
                nat_add(a1096, a1097)?
            })))
        }
    }
}

pub fn f115(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt7, v5: u64) -> R<Adt7> {
    f136(Adt7::C0({
        let a1101 = match v4.clone() {
            Adt7::C0(h1098, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1098
            }
        };
        let a1102 = f137(v0.clone(), v1, v2, v3, {
            let a1099 = v5;
            let a1100 = 0u64;
            nat_add(a1099, a1100)?
        })?;
        fixed_u64::bit_xor(a1101, a1102)
    }, {
        let a1106 = match v4.clone() {
            Adt7::C0(_, h1103, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1107 = f137(v0.clone(), v1, v2, v3, {
            let a1104 = v5;
            let a1105 = 8u64;
            nat_add(a1104, a1105)?
        })?;
        fixed_u64::bit_xor(a1106, a1107)
    }, {
        let a1111 = match v4.clone() {
            Adt7::C0(_, _, h1108, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1108
            }
        };
        let a1112 = f137(v0.clone(), v1, v2, v3, {
            let a1109 = v5;
            let a1110 = 16u64;
            nat_add(a1109, a1110)?
        })?;
        fixed_u64::bit_xor(a1111, a1112)
    }, {
        let a1116 = match v4.clone() {
            Adt7::C0(_, _, _, h1113, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1113
            }
        };
        let a1117 = f137(v0.clone(), v1, v2, v3, {
            let a1114 = v5;
            let a1115 = 24u64;
            nat_add(a1114, a1115)?
        })?;
        fixed_u64::bit_xor(a1116, a1117)
    }, {
        let a1121 = match v4.clone() {
            Adt7::C0(_, _, _, _, h1118, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1122 = f137(v0.clone(), v1, v2, v3, {
            let a1119 = v5;
            let a1120 = 32u64;
            nat_add(a1119, a1120)?
        })?;
        fixed_u64::bit_xor(a1121, a1122)
    }, {
        let a1126 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, h1123, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1123
            }
        };
        let a1127 = f137(v0.clone(), v1, v2, v3, {
            let a1124 = v5;
            let a1125 = 40u64;
            nat_add(a1124, a1125)?
        })?;
        fixed_u64::bit_xor(a1126, a1127)
    }, {
        let a1131 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, h1128, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1128
            }
        };
        let a1132 = f137(v0.clone(), v1, v2, v3, {
            let a1129 = v5;
            let a1130 = 48u64;
            nat_add(a1129, a1130)?
        })?;
        fixed_u64::bit_xor(a1131, a1132)
    }, {
        let a1136 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h1133, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        let a1137 = f137(v0.clone(), v1, v2, v3, {
            let a1134 = v5;
            let a1135 = 56u64;
            nat_add(a1134, a1135)?
        })?;
        fixed_u64::bit_xor(a1136, a1137)
    }, {
        let a1141 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h1138, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1138
            }
        };
        let a1142 = f137(v0.clone(), v1, v2, v3, {
            let a1139 = v5;
            let a1140 = 64u64;
            nat_add(a1139, a1140)?
        })?;
        fixed_u64::bit_xor(a1141, a1142)
    }, {
        let a1146 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1143, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1143
            }
        };
        let a1147 = f137(v0.clone(), v1, v2, v3, {
            let a1144 = v5;
            let a1145 = 72u64;
            nat_add(a1144, a1145)?
        })?;
        fixed_u64::bit_xor(a1146, a1147)
    }, {
        let a1151 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1148, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1148
            }
        };
        let a1152 = f137(v0.clone(), v1, v2, v3, {
            let a1149 = v5;
            let a1150 = 80u64;
            nat_add(a1149, a1150)?
        })?;
        fixed_u64::bit_xor(a1151, a1152)
    }, {
        let a1156 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h1153, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1153
            }
        };
        let a1157 = f137(v0.clone(), v1, v2, v3, {
            let a1154 = v5;
            let a1155 = 88u64;
            nat_add(a1154, a1155)?
        })?;
        fixed_u64::bit_xor(a1156, a1157)
    }, {
        let a1161 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1158, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1158
            }
        };
        let a1162 = f137(v0.clone(), v1, v2, v3, {
            let a1159 = v5;
            let a1160 = 96u64;
            nat_add(a1159, a1160)?
        })?;
        fixed_u64::bit_xor(a1161, a1162)
    }, {
        let a1166 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1163, _, _, _, _, _, _, _, _, _, _, _) => {
                h1163
            }
        };
        let a1167 = f137(v0.clone(), v1, v2, v3, {
            let a1164 = v5;
            let a1165 = 104u64;
            nat_add(a1164, a1165)?
        })?;
        fixed_u64::bit_xor(a1166, a1167)
    }, {
        let a1171 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1168, _, _, _, _, _, _, _, _, _, _) => {
                h1168
            }
        };
        let a1172 = f137(v0.clone(), v1, v2, v3, {
            let a1169 = v5;
            let a1170 = 112u64;
            nat_add(a1169, a1170)?
        })?;
        fixed_u64::bit_xor(a1171, a1172)
    }, {
        let a1176 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1173, _, _, _, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1177 = f137(v0.clone(), v1, v2, v3, {
            let a1174 = v5;
            let a1175 = 120u64;
            nat_add(a1174, a1175)?
        })?;
        fixed_u64::bit_xor(a1176, a1177)
    }, {
        let a1181 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1178, _, _, _, _, _, _, _, _) => {
                h1178
            }
        };
        let a1182 = f137(v0, v1, v2, v3, {
            let a1179 = v5;
            let a1180 = 128u64;
            nat_add(a1179, a1180)?
        })?;
        fixed_u64::bit_xor(a1181, a1182)
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1183, _, _, _, _, _, _, _) => {
            h1183
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1184, _, _, _, _, _, _) => {
            h1184
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1185, _, _, _, _, _) => {
            h1185
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1186, _, _, _, _) => {
            h1186
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1187, _, _, _) => {
            h1187
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1188, _, _) => {
            h1188
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1189, _) => {
            h1189
        }
    }, match v4 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1190) => {
            h1190
        }
    }))
}

pub fn f116(v0: Fn19, p1194: u64, p1195: Adt15) -> R<Adt15> {
    let m1193 = (p1194, p1195);
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v1, v2) = m1193;
                let m1191 = v1;
                if m1191 == 0 {
                    Err::<(u64, Adt15), R<Adt15>>(Ok(v2))
                } else {
                    let v3 = m1191 - 1;
                    Ok::<(u64, Adt15), R<Adt15>>((v3, {
                        let c1192: Fn19 = v0.clone();
                        c1192.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt15) -> R<Adt15> {
    f138(v0)
}

pub fn f118(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f139({
        let a1196 = v0;
        let a1197 = v2;
        fixed_u64::bit_and(a1196, a1197)
    }, {
        let a1198 = v1;
        let a1199 = v2;
        fixed_u64::bit_and(a1198, a1199)
    });
    let v4: u64 = f139(f139(f94(v0, 32u32), f94(v1, 32u32)), f94(v3, 32u32));
    let a1204 = f140({
        let a1200 = v4;
        let a1201 = v2;
        fixed_u64::bit_and(a1200, a1201)
    }, 32u32);
    let a1205 = {
        let a1202 = v3;
        let a1203 = v2;
        fixed_u64::bit_and(a1202, a1203)
    };
    fixed_u64::bit_or(a1204, a1205)
}

pub fn f119(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1208 = {
        let a1206 = v3;
        let a1207 = 16u64;
        nat_add(a1206, a1207)?
    };
    let a1209 = v2;
    if nat_eq(a1208, a1209) {
        Ok(0u64)
    } else {
        let a1212 = {
            let a1210 = v3;
            let a1211 = 8u64;
            nat_add(a1210, a1211)?
        };
        let a1213 = v2;
        if nat_eq(a1212, a1213) {
            Ok(f141({
                let a1214 = v1;
                let a1215 = 8u64;
                nat_mul(a1214, a1215)?
            }))
        } else {
            Ok(f142(f143(v0.clone(), v1, {
                let a1216 = v3;
                let a1217 = 0u64;
                nat_add(a1216, a1217)?
            }), f143(v0.clone(), v1, {
                let a1218 = v3;
                let a1219 = 1u64;
                nat_add(a1218, a1219)?
            }), f143(v0.clone(), v1, {
                let a1220 = v3;
                let a1221 = 2u64;
                nat_add(a1220, a1221)?
            }), f143(v0.clone(), v1, {
                let a1222 = v3;
                let a1223 = 3u64;
                nat_add(a1222, a1223)?
            }), f143(v0.clone(), v1, {
                let a1224 = v3;
                let a1225 = 4u64;
                nat_add(a1224, a1225)?
            }), f143(v0.clone(), v1, {
                let a1226 = v3;
                let a1227 = 5u64;
                nat_add(a1226, a1227)?
            }), f143(v0.clone(), v1, {
                let a1228 = v3;
                let a1229 = 6u64;
                nat_add(a1228, a1229)?
            }), f143(v0, v1, {
                let a1230 = v3;
                let a1231 = 7u64;
                nat_add(a1230, a1231)?
            })))
        }
    }
}

pub fn f120(v0: (u64, List<Adt5>), v1: Adt5) -> R<(u64, List<Adt5>)> {
    let v2: Adt18 = f144(Fn23::F145, 40u64, Adt18::C0(v1, {
        let a1233 = {
            let (h1232, _) = v0.clone();
            h1232
        };
        let a1234 = 1u64;
        nat_add(a1233, a1234)?
    }, {
        let (_, h1235) = v0;
        h1235
    }));
    Ok(({
        let a1237 = {
            let (h1236, _) = v0;
            h1236
        };
        let a1238 = 1u64;
        nat_add(a1237, a1238)?
    }, List::cons(match v2.clone() {
        Adt18::C0(h1239, _, _) => {
            h1239
        }
    }, match v2 {
        Adt18::C0(_, _, h1240) => {
            h1240
        }
    })))
}

pub fn f121(v0: Fn20, p1244: u64, p1245: (Adt17, Adt16)) -> (Adt17, Adt16) {
    let m1243 = (p1244, p1245);
    {
        let mut m1243 = m1243;
        loop {
            let step = {
                let (v1, v2) = m1243;
                let m1241 = v1;
                if m1241 == 0 {
                    Err::<(u64, (Adt17, Adt16)), (Adt17, Adt16)>(v2)
                } else {
                    let v3 = m1241 - 1;
                    Ok::<(u64, (Adt17, Adt16)), (Adt17, Adt16)>((v3, {
                        let c1242: Fn20 = v0.clone();
                        c1242.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: (Adt17, Adt16)) -> (Adt17, Adt16) {
    f146(v0)
}

pub fn f123(v0: Fn21, p1249: u64, p1250: (u64, Adt5)) -> R<(u64, Adt5)> {
    let m1248 = (p1249, p1250);
    {
        let mut m1248 = m1248;
        loop {
            let step = {
                let (v1, v2) = m1248;
                let m1246 = v1;
                if m1246 == 0 {
                    Err::<(u64, (u64, Adt5)), R<(u64, Adt5)>>(Ok(v2))
                } else {
                    let v3 = m1246 - 1;
                    Ok::<(u64, (u64, Adt5)), R<(u64, Adt5)>>((v3, {
                        let c1247: Fn21 = v0.clone();
                        c1247.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt5)) -> R<(u64, Adt5)> {
    Ok(({
        let a1252 = {
            let (h1251, _) = v3.clone();
            h1251
        };
        let a1253 = 1u64;
        nat_add(a1252, a1253)?
    }, f99({
        let (_, h1254) = v3.clone();
        h1254
    }, f101(v1, {
        let a1258 = v2;
        let a1259 = {
            let a1256 = 64u64;
            let a1257 = {
                let (h1255, _) = v3.clone();
                h1255
            };
            nat_mul(a1256, a1257)?
        };
        nat_add(a1258, a1259)?
    })?, v0, 64u32, {
        let a1261 = {
            let (h1260, _) = v3;
            h1260
        };
        let a1262 = 0u64;
        if nat_eq(a1261, a1262) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f125() -> Adt5 {
    Adt5::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f126(v0: Bytes, v1: u64) -> R<u32> {
    let a1273 = f147(v0.clone(), v1);
    let a1274 = {
        let a1271 = f148(f147(v0.clone(), {
            let a1263 = v1;
            let a1264 = 1u64;
            nat_add(a1263, a1264)?
        }), 8u32);
        let a1272 = {
            let a1269 = f148(f147(v0.clone(), {
                let a1265 = v1;
                let a1266 = 2u64;
                nat_add(a1265, a1266)?
            }), 16u32);
            let a1270 = f148(f147(v0, {
                let a1267 = v1;
                let a1268 = 3u64;
                nat_add(a1267, a1268)?
            }), 24u32);
            fixed_u32::bit_or(a1269, a1270)
        };
        fixed_u32::bit_or(a1271, a1272)
    };
    Ok(fixed_u32::bit_or(a1273, a1274))
}

pub fn f127(v0: Fn22, p1278: u64, p1279: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1277 = (p1278, p1279);
    {
        let mut m1277 = m1277;
        loop {
            let step = {
                let (v1, v2) = m1277;
                let m1275 = v1;
                if m1275 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1275 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1276: Fn22 = v0.clone();
                        c1276.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1280, _) = v0;
        h1280
    };
    let v2: u32 = {
        let (h1281, _) = {
            let (_, h1282) = v0;
            h1282
        };
        h1281
    };
    let v3: u32 = {
        let (_, h1283) = {
            let (_, h1284) = v0;
            h1284
        };
        h1283
    };
    ({
        let a1285 = v1;
        let a1286 = 2u64;
        let a1287 = 0u64;
        nat_quot(a1285, a1286, a1287)
    }, (f148(v2, 1u32), {
        let a1291 = {
            let a1288 = v1;
            let a1289 = 2u64;
            let a1290 = 0u64;
            nat_rem(a1288, a1289, a1290)
        };
        let a1292 = 1u64;
        if nat_eq(a1291, a1292) {
            let a1293 = v3;
            let a1294 = v2;
            fixed_u32::bit_or(a1293, a1294)
        } else {
            v3
        }
    }))
}

pub fn f129(v0: Adt5, v1: Adt5, v2: bool) -> Adt5 {
    f99(f125(), f149(v0, v1), 0u64, 64u32, {
        let a1295 = 4u32;
        let a1296 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a1295, a1296)
    })
}

pub fn f130(v0: u64, v1: List<Adt9>) -> Option<Adt8> {
    Some(Adt8::C0(v0, true, v1))
}

pub fn f131(v0: Bytes, v1: u64, v2: Adt9, v3: List<Adt9>) -> R<Option<Adt8>> {
    let v4: u64 = match v2.clone() {
        Adt9::C0(h1297, _, _) => {
            h1297
        }
    };
    let v5: u64 = f78(v4);
    let a1298 = 0u64;
    let a1299 = v5;
    if nat_lt(a1298, a1299) {
        if f29(v0, v1, v5) {
            Ok(f130({
                let a1300 = v1;
                let a1301 = v5;
                nat_add(a1300, a1301)?
            }, v3))
        } else {
            Ok(f150(v1))
        }
    } else {
        let a1302 = v4;
        let a1303 = 8u64;
        if nat_eq(a1302, a1303) {
            if f29(v0.clone(), v1, 8u64) {
                let v6: u64 = f4(v0.clone(), v1, 8u64)?;
                if f29(v0, {
                    let a1304 = v1;
                    let a1305 = 8u64;
                    nat_add(a1304, a1305)?
                }, v6) {
                    Ok(f130({
                        let a1308 = {
                            let a1306 = v1;
                            let a1307 = 8u64;
                            nat_add(a1306, a1307)?
                        };
                        let a1309 = v6;
                        nat_add(a1308, a1309)?
                    }, v3))
                } else {
                    Ok(f150(v1))
                }
            } else {
                Ok(f150(v1))
            }
        } else {
            let a1310 = v4;
            let a1311 = 9u64;
            if nat_eq(a1310, a1311) {
                let a1313 = 64u64;
                let a1314 = match v2.clone() {
                    Adt9::C0(_, _, h1312) => {
                        h1312
                    }
                };
                if nat_le(a1313, a1314) {
                    Ok(f150(v1))
                } else {
                    if f29(v0.clone(), v1, 12u64) {
                        Ok(f130({
                            let a1315 = v1;
                            let a1316 = 12u64;
                            nat_add(a1315, a1316)?
                        }, List::cons(Adt9::C0(f4(v0.clone(), v1, 4u64)?, f4(v0, {
                            let a1317 = v1;
                            let a1318 = 4u64;
                            nat_add(a1317, a1318)?
                        }, 8u64)?, {
                            let a1320 = match v2 {
                                Adt9::C0(_, _, h1319) => {
                                    h1319
                                }
                            };
                            let a1321 = 1u64;
                            nat_add(a1320, a1321)?
                        }), v3)))
                    } else {
                        Ok(f150(v1))
                    }
                }
            } else {
                Ok(f150(v1))
            }
        }
    }
}

pub fn f132(v0: Adt13) -> R<Adt13> {
    let v1: u32 = f151(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, h1322) => {
            h1322
        }
    });
    let v2: u32 = f113(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h1323, _, _) => {
            h1323
        }
    }, f113(f152(match v0.clone() {
        Adt13::C0(_, _, _, _, h1324, _, _, _, _, _) => {
            h1324
        }
    }), f113(f153(match v0.clone() {
        Adt13::C0(_, _, _, _, h1325, _, _, _, _, _) => {
            h1325
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1326, _, _, _, _) => {
            h1326
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1327, _, _, _) => {
            h1327
        }
    }), f113(v1, {
        let m1330 = match v0.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, h1329, _) => {
                h1329
            }
        };
        match m1330 {
            Adt12::C0(h1328, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1328
            }
        }
    }))));
    let v3: u32 = f113(f154(match v0.clone() {
        Adt13::C0(h1331, _, _, _, _, _, _, _, _, _) => {
            h1331
        }
    }), f155(match v0.clone() {
        Adt13::C0(h1332, _, _, _, _, _, _, _, _, _) => {
            h1332
        }
    }, match v0.clone() {
        Adt13::C0(_, h1333, _, _, _, _, _, _, _, _) => {
            h1333
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h1334, _, _, _, _, _, _, _) => {
            h1334
        }
    }));
    Ok(Adt13::C0(f113(v2, v3), match v0.clone() {
        Adt13::C0(h1335, _, _, _, _, _, _, _, _, _) => {
            h1335
        }
    }, match v0.clone() {
        Adt13::C0(_, h1336, _, _, _, _, _, _, _, _) => {
            h1336
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h1337, _, _, _, _, _, _, _) => {
            h1337
        }
    }, f113(match v0.clone() {
        Adt13::C0(_, _, _, h1338, _, _, _, _, _, _) => {
            h1338
        }
    }, v2), match v0.clone() {
        Adt13::C0(_, _, _, _, h1339, _, _, _, _, _) => {
            h1339
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1340, _, _, _, _) => {
            h1340
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1341, _, _, _) => {
            h1341
        }
    }, f156(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, h1342, _) => {
            h1342
        }
    }), {
        let a1344 = match v0 {
            Adt13::C0(_, _, _, _, _, _, _, _, _, h1343) => {
                h1343
            }
        };
        let a1345 = 1u64;
        nat_add(a1344, a1345)?
    }))
}

pub fn f133(v0: u32) -> u64 {
    let m1347 = {
        let a1346 = v0;
        fixed_u64::convert(i128::from(a1346))
    };
    match m1347 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f134(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1352 = f148(v0, 24u32);
    let a1353 = {
        let a1350 = f148(v1, 16u32);
        let a1351 = {
            let a1348 = f148(v2, 8u32);
            let a1349 = v3;
            fixed_u32::bit_or(a1348, a1349)
        };
        fixed_u32::bit_or(a1350, a1351)
    };
    fixed_u32::bit_or(a1352, a1353)
}

pub fn f135(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1354 = v2;
    let a1355 = v1;
    if nat_lt(a1354, a1355) {
        let m1358 = {
            let a1356 = v0;
            let a1357 = v2;
            index_bytes(a1356, a1357)
        };
        match m1358 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1360 = {
                    let a1359 = v3;
                    fixed_u32::convert(i128::from(a1359))
                };
                match m1360 {
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
        let a1361 = v2;
        let a1362 = v1;
        if nat_eq(a1361, a1362) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f136(v0: Adt7) -> R<Adt7> {
    let (_, h1363) = f88(Fn14::F157, 24u64, (0u64, v0))?;
    Ok(h1363)
}

pub fn f137(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f158(f159(v0.clone(), v1, v2, v3, {
        let a1364 = v4;
        let a1365 = 0u64;
        nat_add(a1364, a1365)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1366 = v4;
        let a1367 = 1u64;
        nat_add(a1366, a1367)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1368 = v4;
        let a1369 = 2u64;
        nat_add(a1368, a1369)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1370 = v4;
        let a1371 = 3u64;
        nat_add(a1370, a1371)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1372 = v4;
        let a1373 = 4u64;
        nat_add(a1372, a1373)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1374 = v4;
        let a1375 = 5u64;
        nat_add(a1374, a1375)?
    })?, f159(v0.clone(), v1, v2, v3, {
        let a1376 = v4;
        let a1377 = 6u64;
        nat_add(a1376, a1377)?
    })?, f159(v0, v1, v2, v3, {
        let a1378 = v4;
        let a1379 = 7u64;
        nat_add(a1378, a1379)?
    })?))
}

pub fn f138(v0: Adt15) -> R<Adt15> {
    let v1: u64 = f160(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h1380) => {
            h1380
        }
    });
    let v2: u64 = f118(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, h1381, _, _) => {
            h1381
        }
    }, f118(f161(match v0.clone() {
        Adt15::C0(_, _, _, _, h1382, _, _, _, _, _) => {
            h1382
        }
    }), f118(f162(match v0.clone() {
        Adt15::C0(_, _, _, _, h1383, _, _, _, _, _) => {
            h1383
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h1384, _, _, _, _) => {
            h1384
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h1385, _, _, _) => {
            h1385
        }
    }), f118(v1, {
        let m1388 = match v0.clone() {
            Adt15::C0(_, _, _, _, _, _, _, _, h1387, _) => {
                h1387
            }
        };
        match m1388 {
            Adt14::C0(h1386, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1386
            }
        }
    }))));
    let v3: u64 = f118(f163(match v0.clone() {
        Adt15::C0(h1389, _, _, _, _, _, _, _, _, _) => {
            h1389
        }
    }), f164(match v0.clone() {
        Adt15::C0(h1390, _, _, _, _, _, _, _, _, _) => {
            h1390
        }
    }, match v0.clone() {
        Adt15::C0(_, h1391, _, _, _, _, _, _, _, _) => {
            h1391
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h1392, _, _, _, _, _, _, _) => {
            h1392
        }
    }));
    Ok(Adt15::C0(f118(v2, v3), match v0.clone() {
        Adt15::C0(h1393, _, _, _, _, _, _, _, _, _) => {
            h1393
        }
    }, match v0.clone() {
        Adt15::C0(_, h1394, _, _, _, _, _, _, _, _) => {
            h1394
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h1395, _, _, _, _, _, _, _) => {
            h1395
        }
    }, f118(match v0.clone() {
        Adt15::C0(_, _, _, h1396, _, _, _, _, _, _) => {
            h1396
        }
    }, v2), match v0.clone() {
        Adt15::C0(_, _, _, _, h1397, _, _, _, _, _) => {
            h1397
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h1398, _, _, _, _) => {
            h1398
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h1399, _, _, _) => {
            h1399
        }
    }, f165(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, h1400, _) => {
            h1400
        }
    }), {
        let a1402 = match v0 {
            Adt15::C0(_, _, _, _, _, _, _, _, _, h1401) => {
                h1401
            }
        };
        let a1403 = 1u64;
        nat_add(a1402, a1403)?
    }))
}

pub fn f139(v0: u64, v1: u64) -> u64 {
    let m1406 = {
        let a1404 = v0;
        let a1405 = v1;
        fixed_u64::checked_add(a1404, a1405)
    };
    match m1406 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f140(v0: u64, v1: u32) -> u64 {
    let m1409 = {
        let a1407 = v0;
        let a1408 = v1;
        fixed_u64::shift_left(a1407, a1408)
    };
    match m1409 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f141(v0: u64) -> u64 {
    let (_, h1410) = {
        let (_, h1411) = f166(Fn24::F167, 64u64, (v0, (1u64, 0u64)));
        h1411
    };
    h1410
}

pub fn f142(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1424 = f140(v0, 56u32);
    let a1425 = {
        let a1422 = f140(v1, 48u32);
        let a1423 = {
            let a1420 = f140(v2, 40u32);
            let a1421 = {
                let a1418 = f140(v3, 32u32);
                let a1419 = {
                    let a1416 = f140(v4, 24u32);
                    let a1417 = {
                        let a1414 = f140(v5, 16u32);
                        let a1415 = {
                            let a1412 = f140(v6, 8u32);
                            let a1413 = v7;
                            fixed_u64::bit_or(a1412, a1413)
                        };
                        fixed_u64::bit_or(a1414, a1415)
                    };
                    fixed_u64::bit_or(a1416, a1417)
                };
                fixed_u64::bit_or(a1418, a1419)
            };
            fixed_u64::bit_or(a1420, a1421)
        };
        fixed_u64::bit_or(a1422, a1423)
    };
    fixed_u64::bit_or(a1424, a1425)
}

pub fn f143(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1426 = v2;
    let a1427 = v1;
    if nat_lt(a1426, a1427) {
        f168(v0, v2)
    } else {
        let a1428 = v2;
        let a1429 = v1;
        if nat_eq(a1428, a1429) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f144(v0: Fn23, p1433: u64, p1434: Adt18) -> Adt18 {
    let m1432 = (p1433, p1434);
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let (v1, v2) = m1432;
                let m1430 = v1;
                if m1430 == 0 {
                    Err::<(u64, Adt18), Adt18>(v2)
                } else {
                    let v3 = m1430 - 1;
                    Ok::<(u64, Adt18), Adt18>((v3, {
                        let c1431: Fn23 = v0.clone();
                        c1431.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Adt18) -> Adt18 {
    f169(v0)
}

pub fn f146(v0: (Adt17, Adt16)) -> (Adt17, Adt16) {
    (f170(f171({
        let (h1435, _) = v0.clone();
        h1435
    }, {
        let (_, h1436) = v0.clone();
        h1436
    }), {
        let (_, h1437) = v0.clone();
        h1437
    }), f172({
        let (_, h1438) = v0;
        h1438
    }))
}

pub fn f147(v0: Bytes, v1: u64) -> u32 {
    let m1441 = {
        let a1439 = v0;
        let a1440 = v1;
        index_bytes(a1439, a1440)
    };
    match m1441 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1443 = {
                let a1442 = v2;
                fixed_u32::convert(i128::from(a1442))
            };
            match m1443 {
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

pub fn f148(v0: u32, v1: u32) -> u32 {
    let m1446 = {
        let a1444 = v0;
        let a1445 = v1;
        fixed_u32::shift_left(a1444, a1445)
    };
    match m1446 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f149(v0: Adt5, v1: Adt5) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt5::C0(h1447, _, _, _, _, _, _, _) => {
            h1447
        }
    }, match v0.clone() {
        Adt5::C0(_, h1448, _, _, _, _, _, _) => {
            h1448
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h1449, _, _, _, _, _) => {
            h1449
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, h1450, _, _, _, _) => {
            h1450
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h1451, _, _, _) => {
            h1451
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h1452, _, _) => {
            h1452
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h1453, _) => {
            h1453
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, h1454) => {
            h1454
        }
    }, match v1.clone() {
        Adt5::C0(h1455, _, _, _, _, _, _, _) => {
            h1455
        }
    }, match v1.clone() {
        Adt5::C0(_, h1456, _, _, _, _, _, _) => {
            h1456
        }
    }, match v1.clone() {
        Adt5::C0(_, _, h1457, _, _, _, _, _) => {
            h1457
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, h1458, _, _, _, _) => {
            h1458
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, h1459, _, _, _) => {
            h1459
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, h1460, _, _) => {
            h1460
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, _, h1461, _) => {
            h1461
        }
    }, match v1 {
        Adt5::C0(_, _, _, _, _, _, _, h1462) => {
            h1462
        }
    })
}

pub fn f150(v0: u64) -> Option<Adt8> {
    Some(Adt8::C0(v0, false, List::<Adt9>::nil()))
}

pub fn f151(v0: u64) -> u32 {
    let a1463 = v0;
    let a1464 = 32u64;
    if nat_lt(a1463, a1464) {
        let a1465 = v0;
        let a1466 = 16u64;
        if nat_lt(a1465, a1466) {
            let a1467 = v0;
            let a1468 = 8u64;
            if nat_lt(a1467, a1468) {
                let a1469 = v0;
                let a1470 = 4u64;
                if nat_lt(a1469, a1470) {
                    let a1471 = v0;
                    let a1472 = 2u64;
                    if nat_lt(a1471, a1472) {
                        let a1473 = v0;
                        let a1474 = 1u64;
                        if nat_lt(a1473, a1474) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1475 = v0;
                        let a1476 = 3u64;
                        if nat_lt(a1475, a1476) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1477 = v0;
                    let a1478 = 6u64;
                    if nat_lt(a1477, a1478) {
                        let a1479 = v0;
                        let a1480 = 5u64;
                        if nat_lt(a1479, a1480) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1481 = v0;
                        let a1482 = 7u64;
                        if nat_lt(a1481, a1482) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1483 = v0;
                let a1484 = 12u64;
                if nat_lt(a1483, a1484) {
                    let a1485 = v0;
                    let a1486 = 10u64;
                    if nat_lt(a1485, a1486) {
                        let a1487 = v0;
                        let a1488 = 9u64;
                        if nat_lt(a1487, a1488) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1489 = v0;
                        let a1490 = 11u64;
                        if nat_lt(a1489, a1490) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1491 = v0;
                    let a1492 = 14u64;
                    if nat_lt(a1491, a1492) {
                        let a1493 = v0;
                        let a1494 = 13u64;
                        if nat_lt(a1493, a1494) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1495 = v0;
                        let a1496 = 15u64;
                        if nat_lt(a1495, a1496) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1497 = v0;
            let a1498 = 24u64;
            if nat_lt(a1497, a1498) {
                let a1499 = v0;
                let a1500 = 20u64;
                if nat_lt(a1499, a1500) {
                    let a1501 = v0;
                    let a1502 = 18u64;
                    if nat_lt(a1501, a1502) {
                        let a1503 = v0;
                        let a1504 = 17u64;
                        if nat_lt(a1503, a1504) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1505 = v0;
                        let a1506 = 19u64;
                        if nat_lt(a1505, a1506) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1507 = v0;
                    let a1508 = 22u64;
                    if nat_lt(a1507, a1508) {
                        let a1509 = v0;
                        let a1510 = 21u64;
                        if nat_lt(a1509, a1510) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1511 = v0;
                        let a1512 = 23u64;
                        if nat_lt(a1511, a1512) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1513 = v0;
                let a1514 = 28u64;
                if nat_lt(a1513, a1514) {
                    let a1515 = v0;
                    let a1516 = 26u64;
                    if nat_lt(a1515, a1516) {
                        let a1517 = v0;
                        let a1518 = 25u64;
                        if nat_lt(a1517, a1518) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1519 = v0;
                        let a1520 = 27u64;
                        if nat_lt(a1519, a1520) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1521 = v0;
                    let a1522 = 30u64;
                    if nat_lt(a1521, a1522) {
                        let a1523 = v0;
                        let a1524 = 29u64;
                        if nat_lt(a1523, a1524) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1525 = v0;
                        let a1526 = 31u64;
                        if nat_lt(a1525, a1526) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1527 = v0;
        let a1528 = 48u64;
        if nat_lt(a1527, a1528) {
            let a1529 = v0;
            let a1530 = 40u64;
            if nat_lt(a1529, a1530) {
                let a1531 = v0;
                let a1532 = 36u64;
                if nat_lt(a1531, a1532) {
                    let a1533 = v0;
                    let a1534 = 34u64;
                    if nat_lt(a1533, a1534) {
                        let a1535 = v0;
                        let a1536 = 33u64;
                        if nat_lt(a1535, a1536) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1537 = v0;
                        let a1538 = 35u64;
                        if nat_lt(a1537, a1538) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1539 = v0;
                    let a1540 = 38u64;
                    if nat_lt(a1539, a1540) {
                        let a1541 = v0;
                        let a1542 = 37u64;
                        if nat_lt(a1541, a1542) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1543 = v0;
                        let a1544 = 39u64;
                        if nat_lt(a1543, a1544) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1545 = v0;
                let a1546 = 44u64;
                if nat_lt(a1545, a1546) {
                    let a1547 = v0;
                    let a1548 = 42u64;
                    if nat_lt(a1547, a1548) {
                        let a1549 = v0;
                        let a1550 = 41u64;
                        if nat_lt(a1549, a1550) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1551 = v0;
                        let a1552 = 43u64;
                        if nat_lt(a1551, a1552) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1553 = v0;
                    let a1554 = 46u64;
                    if nat_lt(a1553, a1554) {
                        let a1555 = v0;
                        let a1556 = 45u64;
                        if nat_lt(a1555, a1556) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1557 = v0;
                        let a1558 = 47u64;
                        if nat_lt(a1557, a1558) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1559 = v0;
            let a1560 = 56u64;
            if nat_lt(a1559, a1560) {
                let a1561 = v0;
                let a1562 = 52u64;
                if nat_lt(a1561, a1562) {
                    let a1563 = v0;
                    let a1564 = 50u64;
                    if nat_lt(a1563, a1564) {
                        let a1565 = v0;
                        let a1566 = 49u64;
                        if nat_lt(a1565, a1566) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1567 = v0;
                        let a1568 = 51u64;
                        if nat_lt(a1567, a1568) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1569 = v0;
                    let a1570 = 54u64;
                    if nat_lt(a1569, a1570) {
                        let a1571 = v0;
                        let a1572 = 53u64;
                        if nat_lt(a1571, a1572) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1573 = v0;
                        let a1574 = 55u64;
                        if nat_lt(a1573, a1574) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1575 = v0;
                let a1576 = 60u64;
                if nat_lt(a1575, a1576) {
                    let a1577 = v0;
                    let a1578 = 58u64;
                    if nat_lt(a1577, a1578) {
                        let a1579 = v0;
                        let a1580 = 57u64;
                        if nat_lt(a1579, a1580) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1581 = v0;
                        let a1582 = 59u64;
                        if nat_lt(a1581, a1582) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1583 = v0;
                    let a1584 = 62u64;
                    if nat_lt(a1583, a1584) {
                        let a1585 = v0;
                        let a1586 = 61u64;
                        if nat_lt(a1585, a1586) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1587 = v0;
                        let a1588 = 63u64;
                        if nat_lt(a1587, a1588) {
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

pub fn f152(v0: u32) -> u32 {
    let a1591 = {
        let a1589 = f173(v0, 6u32, 26u32);
        let a1590 = f173(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1589, a1590)
    };
    let a1592 = f173(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1591, a1592)
}

pub fn f153(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1598 = {
        let a1593 = v0;
        let a1594 = v1;
        fixed_u32::bit_and(a1593, a1594)
    };
    let a1599 = {
        let a1596 = {
            let a1595 = v0;
            fixed_u32::bit_not(a1595)
        };
        let a1597 = v2;
        fixed_u32::bit_and(a1596, a1597)
    };
    fixed_u32::bit_xor(a1598, a1599)
}

pub fn f154(v0: u32) -> u32 {
    let a1602 = {
        let a1600 = f173(v0, 2u32, 30u32);
        let a1601 = f173(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1600, a1601)
    };
    let a1603 = f173(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1602, a1603)
}

pub fn f155(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1612 = {
        let a1608 = {
            let a1604 = v0;
            let a1605 = v1;
            fixed_u32::bit_and(a1604, a1605)
        };
        let a1609 = {
            let a1606 = v0;
            let a1607 = v2;
            fixed_u32::bit_and(a1606, a1607)
        };
        fixed_u32::bit_xor(a1608, a1609)
    };
    let a1613 = {
        let a1610 = v1;
        let a1611 = v2;
        fixed_u32::bit_and(a1610, a1611)
    };
    fixed_u32::bit_xor(a1612, a1613)
}

pub fn f156(v0: Adt12) -> Adt12 {
    Adt12::C0(match v0.clone() {
        Adt12::C0(_, h1614, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1614
        }
    }, match v0.clone() {
        Adt12::C0(_, _, h1615, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1615
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, h1616, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1616
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, h1617, _, _, _, _, _, _, _, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, h1618, _, _, _, _, _, _, _, _, _, _) => {
            h1618
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, h1619, _, _, _, _, _, _, _, _, _) => {
            h1619
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, h1620, _, _, _, _, _, _, _, _) => {
            h1620
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, h1621, _, _, _, _, _, _, _) => {
            h1621
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1622, _, _, _, _, _, _) => {
            h1622
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, h1623, _, _, _, _, _) => {
            h1623
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, h1624, _, _, _, _) => {
            h1624
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1625, _, _, _) => {
            h1625
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1626, _, _) => {
            h1626
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1627, _) => {
            h1627
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1628) => {
            h1628
        }
    }, f113(f174(match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1629, _) => {
            h1629
        }
    }), f113(match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1630, _, _, _, _, _, _) => {
            h1630
        }
    }, f113(f175(match v0.clone() {
        Adt12::C0(_, h1631, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    }), match v0 {
        Adt12::C0(h1632, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1632
        }
    }))))
}

pub fn f157(v0: (u64, Adt7)) -> R<(u64, Adt7)> {
    f176(v0)
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1645 = f140(v7, 56u32);
    let a1646 = {
        let a1643 = f140(v6, 48u32);
        let a1644 = {
            let a1641 = f140(v5, 40u32);
            let a1642 = {
                let a1639 = f140(v4, 32u32);
                let a1640 = {
                    let a1637 = f140(v3, 24u32);
                    let a1638 = {
                        let a1635 = f140(v2, 16u32);
                        let a1636 = {
                            let a1633 = f140(v1, 8u32);
                            let a1634 = v0;
                            fixed_u64::bit_or(a1633, a1634)
                        };
                        fixed_u64::bit_or(a1635, a1636)
                    };
                    fixed_u64::bit_or(a1637, a1638)
                };
                fixed_u64::bit_or(a1639, a1640)
            };
            fixed_u64::bit_or(a1641, a1642)
        };
        fixed_u64::bit_or(a1643, a1644)
    };
    fixed_u64::bit_or(a1645, a1646)
}

pub fn f159(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1647 = v4;
        let a1648 = v1;
        if nat_lt(a1647, a1648) {
            f168(v0, v4)
        } else {
            let a1649 = v4;
            let a1650 = v1;
            if nat_eq(a1649, a1650) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1653 = {
        let a1651 = v4;
        let a1652 = 1u64;
        nat_add(a1651, a1652)?
    };
    let a1654 = v2;
    if nat_eq(a1653, a1654) {
        let a1655 = v5;
        let a1656 = 128u64;
        Ok(fixed_u64::bit_or(a1655, a1656))
    } else {
        Ok(v5)
    }
}

pub fn f160(v0: u64) -> u64 {
    let a1657 = v0;
    let a1658 = 40u64;
    if nat_lt(a1657, a1658) {
        let a1659 = v0;
        let a1660 = 20u64;
        if nat_lt(a1659, a1660) {
            let a1661 = v0;
            let a1662 = 10u64;
            if nat_lt(a1661, a1662) {
                let a1663 = v0;
                let a1664 = 5u64;
                if nat_lt(a1663, a1664) {
                    let a1665 = v0;
                    let a1666 = 2u64;
                    if nat_lt(a1665, a1666) {
                        let a1667 = v0;
                        let a1668 = 1u64;
                        if nat_lt(a1667, a1668) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1669 = v0;
                        let a1670 = 3u64;
                        if nat_lt(a1669, a1670) {
                            13096744586834688815u64
                        } else {
                            let a1671 = v0;
                            let a1672 = 4u64;
                            if nat_lt(a1671, a1672) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1673 = v0;
                    let a1674 = 7u64;
                    if nat_lt(a1673, a1674) {
                        let a1675 = v0;
                        let a1676 = 6u64;
                        if nat_lt(a1675, a1676) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1677 = v0;
                        let a1678 = 8u64;
                        if nat_lt(a1677, a1678) {
                            12329834152419229976u64
                        } else {
                            let a1679 = v0;
                            let a1680 = 9u64;
                            if nat_lt(a1679, a1680) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1681 = v0;
                let a1682 = 15u64;
                if nat_lt(a1681, a1682) {
                    let a1683 = v0;
                    let a1684 = 12u64;
                    if nat_lt(a1683, a1684) {
                        let a1685 = v0;
                        let a1686 = 11u64;
                        if nat_lt(a1685, a1686) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1687 = v0;
                        let a1688 = 13u64;
                        if nat_lt(a1687, a1688) {
                            8268148722764581231u64
                        } else {
                            let a1689 = v0;
                            let a1690 = 14u64;
                            if nat_lt(a1689, a1690) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1691 = v0;
                    let a1692 = 17u64;
                    if nat_lt(a1691, a1692) {
                        let a1693 = v0;
                        let a1694 = 16u64;
                        if nat_lt(a1693, a1694) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1695 = v0;
                        let a1696 = 18u64;
                        if nat_lt(a1695, a1696) {
                            17275323862435702243u64
                        } else {
                            let a1697 = v0;
                            let a1698 = 19u64;
                            if nat_lt(a1697, a1698) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1699 = v0;
            let a1700 = 30u64;
            if nat_lt(a1699, a1700) {
                let a1701 = v0;
                let a1702 = 25u64;
                if nat_lt(a1701, a1702) {
                    let a1703 = v0;
                    let a1704 = 22u64;
                    if nat_lt(a1703, a1704) {
                        let a1705 = v0;
                        let a1706 = 21u64;
                        if nat_lt(a1705, a1706) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1707 = v0;
                        let a1708 = 23u64;
                        if nat_lt(a1707, a1708) {
                            6679025012923562964u64
                        } else {
                            let a1709 = v0;
                            let a1710 = 24u64;
                            if nat_lt(a1709, a1710) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1711 = v0;
                    let a1712 = 27u64;
                    if nat_lt(a1711, a1712) {
                        let a1713 = v0;
                        let a1714 = 26u64;
                        if nat_lt(a1713, a1714) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1715 = v0;
                        let a1716 = 28u64;
                        if nat_lt(a1715, a1716) {
                            13788192230050041572u64
                        } else {
                            let a1717 = v0;
                            let a1718 = 29u64;
                            if nat_lt(a1717, a1718) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1719 = v0;
                let a1720 = 35u64;
                if nat_lt(a1719, a1720) {
                    let a1721 = v0;
                    let a1722 = 32u64;
                    if nat_lt(a1721, a1722) {
                        let a1723 = v0;
                        let a1724 = 31u64;
                        if nat_lt(a1723, a1724) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1725 = v0;
                        let a1726 = 33u64;
                        if nat_lt(a1725, a1726) {
                            2861767655752347644u64
                        } else {
                            let a1727 = v0;
                            let a1728 = 34u64;
                            if nat_lt(a1727, a1728) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1729 = v0;
                    let a1730 = 37u64;
                    if nat_lt(a1729, a1730) {
                        let a1731 = v0;
                        let a1732 = 36u64;
                        if nat_lt(a1731, a1732) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1733 = v0;
                        let a1734 = 38u64;
                        if nat_lt(a1733, a1734) {
                            8532644243296465576u64
                        } else {
                            let a1735 = v0;
                            let a1736 = 39u64;
                            if nat_lt(a1735, a1736) {
                                9350256976987008742u64
                            } else {
                                10552545826968843579u64
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a1737 = v0;
        let a1738 = 60u64;
        if nat_lt(a1737, a1738) {
            let a1739 = v0;
            let a1740 = 50u64;
            if nat_lt(a1739, a1740) {
                let a1741 = v0;
                let a1742 = 45u64;
                if nat_lt(a1741, a1742) {
                    let a1743 = v0;
                    let a1744 = 42u64;
                    if nat_lt(a1743, a1744) {
                        let a1745 = v0;
                        let a1746 = 41u64;
                        if nat_lt(a1745, a1746) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1747 = v0;
                        let a1748 = 43u64;
                        if nat_lt(a1747, a1748) {
                            14000437183269869457u64
                        } else {
                            let a1749 = v0;
                            let a1750 = 44u64;
                            if nat_lt(a1749, a1750) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1751 = v0;
                    let a1752 = 47u64;
                    if nat_lt(a1751, a1752) {
                        let a1753 = v0;
                        let a1754 = 46u64;
                        if nat_lt(a1753, a1754) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1755 = v0;
                        let a1756 = 48u64;
                        if nat_lt(a1755, a1756) {
                            1182934255886127544u64
                        } else {
                            let a1757 = v0;
                            let a1758 = 49u64;
                            if nat_lt(a1757, a1758) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1759 = v0;
                let a1760 = 55u64;
                if nat_lt(a1759, a1760) {
                    let a1761 = v0;
                    let a1762 = 52u64;
                    if nat_lt(a1761, a1762) {
                        let a1763 = v0;
                        let a1764 = 51u64;
                        if nat_lt(a1763, a1764) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1765 = v0;
                        let a1766 = 53u64;
                        if nat_lt(a1765, a1766) {
                            4115178125766777443u64
                        } else {
                            let a1767 = v0;
                            let a1768 = 54u64;
                            if nat_lt(a1767, a1768) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1769 = v0;
                    let a1770 = 57u64;
                    if nat_lt(a1769, a1770) {
                        let a1771 = v0;
                        let a1772 = 56u64;
                        if nat_lt(a1771, a1772) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1773 = v0;
                        let a1774 = 58u64;
                        if nat_lt(a1773, a1774) {
                            8693463985226723168u64
                        } else {
                            let a1775 = v0;
                            let a1776 = 59u64;
                            if nat_lt(a1775, a1776) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1777 = v0;
            let a1778 = 70u64;
            if nat_lt(a1777, a1778) {
                let a1779 = v0;
                let a1780 = 65u64;
                if nat_lt(a1779, a1780) {
                    let a1781 = v0;
                    let a1782 = 62u64;
                    if nat_lt(a1781, a1782) {
                        let a1783 = v0;
                        let a1784 = 61u64;
                        if nat_lt(a1783, a1784) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1785 = v0;
                        let a1786 = 63u64;
                        if nat_lt(a1785, a1786) {
                            13761210420658862357u64
                        } else {
                            let a1787 = v0;
                            let a1788 = 64u64;
                            if nat_lt(a1787, a1788) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1789 = v0;
                    let a1790 = 67u64;
                    if nat_lt(a1789, a1790) {
                        let a1791 = v0;
                        let a1792 = 66u64;
                        if nat_lt(a1791, a1792) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1793 = v0;
                        let a1794 = 68u64;
                        if nat_lt(a1793, a1794) {
                            17689382322260857208u64
                        } else {
                            let a1795 = v0;
                            let a1796 = 69u64;
                            if nat_lt(a1795, a1796) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1797 = v0;
                let a1798 = 75u64;
                if nat_lt(a1797, a1798) {
                    let a1799 = v0;
                    let a1800 = 72u64;
                    if nat_lt(a1799, a1800) {
                        let a1801 = v0;
                        let a1802 = 71u64;
                        if nat_lt(a1801, a1802) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1803 = v0;
                        let a1804 = 73u64;
                        if nat_lt(a1803, a1804) {
                            2944078676154940804u64
                        } else {
                            let a1805 = v0;
                            let a1806 = 74u64;
                            if nat_lt(a1805, a1806) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1807 = v0;
                    let a1808 = 77u64;
                    if nat_lt(a1807, a1808) {
                        let a1809 = v0;
                        let a1810 = 76u64;
                        if nat_lt(a1809, a1810) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1811 = v0;
                        let a1812 = 78u64;
                        if nat_lt(a1811, a1812) {
                            6448918945643986474u64
                        } else {
                            let a1813 = v0;
                            let a1814 = 79u64;
                            if nat_lt(a1813, a1814) {
                                6902733635092675308u64
                            } else {
                                7801388544844847127u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(v0: u64) -> u64 {
    let a1817 = {
        let a1815 = f177(v0, 14u32, 50u32);
        let a1816 = f177(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1815, a1816)
    };
    let a1818 = f177(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1817, a1818)
}

pub fn f162(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1824 = {
        let a1819 = v0;
        let a1820 = v1;
        fixed_u64::bit_and(a1819, a1820)
    };
    let a1825 = {
        let a1822 = {
            let a1821 = v0;
            fixed_u64::bit_not(a1821)
        };
        let a1823 = v2;
        fixed_u64::bit_and(a1822, a1823)
    };
    fixed_u64::bit_xor(a1824, a1825)
}

pub fn f163(v0: u64) -> u64 {
    let a1828 = {
        let a1826 = f177(v0, 28u32, 36u32);
        let a1827 = f177(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1826, a1827)
    };
    let a1829 = f177(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1828, a1829)
}

pub fn f164(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1838 = {
        let a1834 = {
            let a1830 = v0;
            let a1831 = v1;
            fixed_u64::bit_and(a1830, a1831)
        };
        let a1835 = {
            let a1832 = v0;
            let a1833 = v2;
            fixed_u64::bit_and(a1832, a1833)
        };
        fixed_u64::bit_xor(a1834, a1835)
    };
    let a1839 = {
        let a1836 = v1;
        let a1837 = v2;
        fixed_u64::bit_and(a1836, a1837)
    };
    fixed_u64::bit_xor(a1838, a1839)
}

pub fn f165(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(_, h1840, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1840
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h1841, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1841
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h1842, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1842
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h1843, _, _, _, _, _, _, _, _, _, _, _) => {
            h1843
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h1844, _, _, _, _, _, _, _, _, _, _) => {
            h1844
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h1845, _, _, _, _, _, _, _, _, _) => {
            h1845
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h1846, _, _, _, _, _, _, _, _) => {
            h1846
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h1847, _, _, _, _, _, _, _) => {
            h1847
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h1848, _, _, _, _, _, _) => {
            h1848
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h1849, _, _, _, _, _) => {
            h1849
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, h1850, _, _, _, _) => {
            h1850
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1851, _, _, _) => {
            h1851
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1852, _, _) => {
            h1852
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1853, _) => {
            h1853
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1854) => {
            h1854
        }
    }, f118(f178(match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1855, _) => {
            h1855
        }
    }), f118(match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h1856, _, _, _, _, _, _) => {
            h1856
        }
    }, f118(f179(match v0.clone() {
        Adt14::C0(_, h1857, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1857
        }
    }), match v0 {
        Adt14::C0(h1858, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1858
        }
    }))))
}

pub fn f166(v0: Fn24, p1862: u64, p1863: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1861 = (p1862, p1863);
    {
        let mut m1861 = m1861;
        loop {
            let step = {
                let (v1, v2) = m1861;
                let m1859 = v1;
                if m1859 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1859 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1860: Fn24 = v0.clone();
                        c1860.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1864, _) = v0;
        h1864
    };
    let v2: u64 = {
        let (h1865, _) = {
            let (_, h1866) = v0;
            h1866
        };
        h1865
    };
    let v3: u64 = {
        let (_, h1867) = {
            let (_, h1868) = v0;
            h1868
        };
        h1867
    };
    ({
        let a1869 = v1;
        let a1870 = 2u64;
        let a1871 = 0u64;
        nat_quot(a1869, a1870, a1871)
    }, (f140(v2, 1u32), {
        let a1875 = {
            let a1872 = v1;
            let a1873 = 2u64;
            let a1874 = 0u64;
            nat_rem(a1872, a1873, a1874)
        };
        let a1876 = 1u64;
        if nat_eq(a1875, a1876) {
            let a1877 = v3;
            let a1878 = v2;
            fixed_u64::bit_or(a1877, a1878)
        } else {
            v3
        }
    }))
}

pub fn f168(v0: Bytes, v1: u64) -> u64 {
    let m1881 = {
        let a1879 = v0;
        let a1880 = v1;
        index_bytes(a1879, a1880)
    };
    match m1881 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1883 = {
                let a1882 = v2;
                fixed_u64::convert(i128::from(a1882))
            };
            match m1883 {
                None => {
                    0u64
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f169(v0: Adt18) -> Adt18 {
    let a1888 = {
        let a1885 = match v0.clone() {
            Adt18::C0(_, h1884, _) => {
                h1884
            }
        };
        let a1886 = 2u64;
        let a1887 = 0u64;
        nat_rem(a1885, a1886, a1887)
    };
    let a1889 = 0u64;
    if nat_eq(a1888, a1889) {
        let m1891 = match v0.clone() {
            Adt18::C0(_, _, h1890) => {
                h1890
            }
        };
        match m1891.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt18::C0(f129(v1, match v0.clone() {
                    Adt18::C0(h1892, _, _) => {
                        h1892
                    }
                }, false), {
                    let a1894 = match v0 {
                        Adt18::C0(_, h1893, _) => {
                            h1893
                        }
                    };
                    let a1895 = 2u64;
                    let a1896 = 0u64;
                    nat_quot(a1894, a1895, a1896)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f170(v0: Adt17, v1: Adt16) -> Adt17 {
    let v2: Adt19 = f180(match v0.clone() {
        Adt17::C0(h1897, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1897
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1898, _, _, _, _, _, _, _, _, _, _) => {
            h1898
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, h1899, _, _, _, _, _) => {
            h1899
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1900) => {
            h1900
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, h1901, _, _, _, _, _, _, _) => {
            h1901
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h1902, _, _, _, _, _, _) => {
            h1902
        }
    });
    let v3: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, h1903, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1903
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1904, _, _, _, _, _, _, _, _, _) => {
            h1904
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h1905, _, _, _, _) => {
            h1905
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1906, _, _, _) => {
            h1906
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, h1907, _, _, _, _, _) => {
            h1907
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, h1908, _, _, _, _) => {
            h1908
        }
    });
    let v4: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, _, h1909, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1909
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h1910, _, _, _, _, _, _, _, _) => {
            h1910
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, h1911, _, _, _, _, _, _, _) => {
            h1911
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1912, _, _) => {
            h1912
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1913, _, _, _) => {
            h1913
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1914, _, _) => {
            h1914
        }
    });
    let v5: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, _, _, h1915, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1915
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h1916, _, _, _, _, _, _, _, _, _, _, _) => {
            h1916
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, h1917, _, _, _, _, _, _) => {
            h1917
        }
    }, match v0 {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1918, _) => {
            h1918
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1919, _) => {
            h1919
        }
    }, match v1 {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1920) => {
            h1920
        }
    });
    Adt17::C0(match v2.clone() {
        Adt19::C0(h1921, _, _, _) => {
            h1921
        }
    }, match v3.clone() {
        Adt19::C0(h1922, _, _, _) => {
            h1922
        }
    }, match v4.clone() {
        Adt19::C0(h1923, _, _, _) => {
            h1923
        }
    }, match v5.clone() {
        Adt19::C0(h1924, _, _, _) => {
            h1924
        }
    }, match v5.clone() {
        Adt19::C0(_, h1925, _, _) => {
            h1925
        }
    }, match v2.clone() {
        Adt19::C0(_, h1926, _, _) => {
            h1926
        }
    }, match v3.clone() {
        Adt19::C0(_, h1927, _, _) => {
            h1927
        }
    }, match v4.clone() {
        Adt19::C0(_, h1928, _, _) => {
            h1928
        }
    }, match v4.clone() {
        Adt19::C0(_, _, h1929, _) => {
            h1929
        }
    }, match v5.clone() {
        Adt19::C0(_, _, h1930, _) => {
            h1930
        }
    }, match v2.clone() {
        Adt19::C0(_, _, h1931, _) => {
            h1931
        }
    }, match v3.clone() {
        Adt19::C0(_, _, h1932, _) => {
            h1932
        }
    }, match v3 {
        Adt19::C0(_, _, _, h1933) => {
            h1933
        }
    }, match v4 {
        Adt19::C0(_, _, _, h1934) => {
            h1934
        }
    }, match v5 {
        Adt19::C0(_, _, _, h1935) => {
            h1935
        }
    }, match v2 {
        Adt19::C0(_, _, _, h1936) => {
            h1936
        }
    })
}

pub fn f171(v0: Adt17, v1: Adt16) -> Adt17 {
    let v2: Adt19 = f180(match v0.clone() {
        Adt17::C0(h1937, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1937
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h1938, _, _, _, _, _, _, _, _, _, _, _) => {
            h1938
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, h1939, _, _, _, _, _, _, _) => {
            h1939
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1940, _, _, _) => {
            h1940
        }
    }, match v1.clone() {
        Adt16::C0(h1941, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1941
        }
    }, match v1.clone() {
        Adt16::C0(_, h1942, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1942
        }
    });
    let v3: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, h1943, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1943
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1944, _, _, _, _, _, _, _, _, _, _) => {
            h1944
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, h1945, _, _, _, _, _, _) => {
            h1945
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1946, _, _) => {
            h1946
        }
    }, match v1.clone() {
        Adt16::C0(_, _, h1947, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1947
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, h1948, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1948
        }
    });
    let v4: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, _, h1949, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1949
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1950, _, _, _, _, _, _, _, _, _) => {
            h1950
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, h1951, _, _, _, _, _) => {
            h1951
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1952, _) => {
            h1952
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, h1953, _, _, _, _, _, _, _, _, _, _, _) => {
            h1953
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, h1954, _, _, _, _, _, _, _, _, _, _) => {
            h1954
        }
    });
    let v5: Adt19 = f180(match v0.clone() {
        Adt17::C0(_, _, _, h1955, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1955
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h1956, _, _, _, _, _, _, _, _) => {
            h1956
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h1957, _, _, _, _) => {
            h1957
        }
    }, match v0 {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1958) => {
            h1958
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, h1959, _, _, _, _, _, _, _, _, _) => {
            h1959
        }
    }, match v1 {
        Adt16::C0(_, _, _, _, _, _, _, h1960, _, _, _, _, _, _, _, _) => {
            h1960
        }
    });
    Adt17::C0(match v2.clone() {
        Adt19::C0(h1961, _, _, _) => {
            h1961
        }
    }, match v3.clone() {
        Adt19::C0(h1962, _, _, _) => {
            h1962
        }
    }, match v4.clone() {
        Adt19::C0(h1963, _, _, _) => {
            h1963
        }
    }, match v5.clone() {
        Adt19::C0(h1964, _, _, _) => {
            h1964
        }
    }, match v2.clone() {
        Adt19::C0(_, h1965, _, _) => {
            h1965
        }
    }, match v3.clone() {
        Adt19::C0(_, h1966, _, _) => {
            h1966
        }
    }, match v4.clone() {
        Adt19::C0(_, h1967, _, _) => {
            h1967
        }
    }, match v5.clone() {
        Adt19::C0(_, h1968, _, _) => {
            h1968
        }
    }, match v2.clone() {
        Adt19::C0(_, _, h1969, _) => {
            h1969
        }
    }, match v3.clone() {
        Adt19::C0(_, _, h1970, _) => {
            h1970
        }
    }, match v4.clone() {
        Adt19::C0(_, _, h1971, _) => {
            h1971
        }
    }, match v5.clone() {
        Adt19::C0(_, _, h1972, _) => {
            h1972
        }
    }, match v2 {
        Adt19::C0(_, _, _, h1973) => {
            h1973
        }
    }, match v3 {
        Adt19::C0(_, _, _, h1974) => {
            h1974
        }
    }, match v4 {
        Adt19::C0(_, _, _, h1975) => {
            h1975
        }
    }, match v5 {
        Adt19::C0(_, _, _, h1976) => {
            h1976
        }
    })
}

pub fn f172(v0: Adt16) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(_, _, h1977, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1977
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h1978, _, _, _, _, _, _, _, _, _) => {
            h1978
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h1979, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1979
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, h1980, _, _, _, _, _) => {
            h1980
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, h1981, _, _, _, _, _, _, _, _) => {
            h1981
        }
    }, match v0.clone() {
        Adt16::C0(h1982, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1982
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, h1983, _, _, _, _, _, _, _, _, _, _, _) => {
            h1983
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1984, _, _) => {
            h1984
        }
    }, match v0.clone() {
        Adt16::C0(_, h1985, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1985
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, h1986, _, _, _, _) => {
            h1986
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1987, _, _, _) => {
            h1987
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h1988, _, _, _, _, _, _, _, _, _, _) => {
            h1988
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h1989, _, _, _, _, _, _) => {
            h1989
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1990, _) => {
            h1990
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1991) => {
            h1991
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, _, _, _, _, h1992, _, _, _, _, _, _, _) => {
            h1992
        }
    })
}

pub fn f173(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1993 = f85(v0, v1);
    let a1994 = f148(v0, v2);
    fixed_u32::bit_or(a1993, a1994)
}

pub fn f174(v0: u32) -> u32 {
    let a1997 = {
        let a1995 = f173(v0, 17u32, 15u32);
        let a1996 = f173(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1995, a1996)
    };
    let a1998 = f85(v0, 10u32);
    fixed_u32::bit_xor(a1997, a1998)
}

pub fn f175(v0: u32) -> u32 {
    let a2001 = {
        let a1999 = f173(v0, 7u32, 25u32);
        let a2000 = f173(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1999, a2000)
    };
    let a2002 = f85(v0, 3u32);
    fixed_u32::bit_xor(a2001, a2002)
}

pub fn f176(v0: (u64, Adt7)) -> R<(u64, Adt7)> {
    Ok(({
        let a2004 = {
            let (h2003, _) = v0.clone();
            h2003
        };
        let a2005 = 1u64;
        nat_add(a2004, a2005)?
    }, f181(f182(f183(f184({
        let (_, h2006) = v0;
        h2006
    }))), {
        let (h2007, _) = v0;
        h2007
    })))
}

pub fn f177(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2008 = f94(v0, v1);
    let a2009 = f140(v0, v2);
    fixed_u64::bit_or(a2008, a2009)
}

pub fn f178(v0: u64) -> u64 {
    let a2012 = {
        let a2010 = f177(v0, 19u32, 45u32);
        let a2011 = f177(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a2010, a2011)
    };
    let a2013 = f94(v0, 6u32);
    fixed_u64::bit_xor(a2012, a2013)
}

pub fn f179(v0: u64) -> u64 {
    let a2016 = {
        let a2014 = f177(v0, 1u32, 63u32);
        let a2015 = f177(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a2014, a2015)
    };
    let a2017 = f94(v0, 7u32);
    fixed_u64::bit_xor(a2016, a2017)
}

pub fn f180(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt19 {
    let v6: u32 = f113(f113(v0, v1), v4);
    let v7: u32 = f173({
        let a2018 = v3;
        let a2019 = v6;
        fixed_u32::bit_xor(a2018, a2019)
    }, 16u32, 16u32);
    let v8: u32 = f113(v2, v7);
    let v9: u32 = f173({
        let a2020 = v1;
        let a2021 = v8;
        fixed_u32::bit_xor(a2020, a2021)
    }, 12u32, 20u32);
    let v10: u32 = f113(f113(v6, v9), v5);
    let v11: u32 = f173({
        let a2022 = v7;
        let a2023 = v10;
        fixed_u32::bit_xor(a2022, a2023)
    }, 8u32, 24u32);
    let v12: u32 = f113(v8, v11);
    let v13: u32 = f173({
        let a2024 = v9;
        let a2025 = v12;
        fixed_u32::bit_xor(a2024, a2025)
    }, 7u32, 25u32);
    Adt19::C0(v10, v13, v12, v11)
}

pub fn f181(v0: Adt7, v1: u64) -> Adt7 {
    let v2: u64 = f185(v1);
    Adt7::C0({
        let a2027 = match v0.clone() {
            Adt7::C0(h2026, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2026
            }
        };
        let a2028 = v2;
        fixed_u64::bit_xor(a2027, a2028)
    }, match v0.clone() {
        Adt7::C0(_, h2029, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2029
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h2030, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2030
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h2031, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2031
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h2032, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2032
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h2033, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2033
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h2034, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2034
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h2035, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2035
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h2036, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2036
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h2037, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2037
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2038, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2038
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2039, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2039
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2040, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2040
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2041, _, _, _, _, _, _, _, _, _, _, _) => {
            h2041
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2042, _, _, _, _, _, _, _, _, _, _) => {
            h2042
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2043, _, _, _, _, _, _, _, _, _) => {
            h2043
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2044, _, _, _, _, _, _, _, _) => {
            h2044
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2045, _, _, _, _, _, _, _) => {
            h2045
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2046, _, _, _, _, _, _) => {
            h2046
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2047, _, _, _, _, _) => {
            h2047
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2048, _, _, _, _) => {
            h2048
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2049, _, _, _) => {
            h2049
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2050, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2051, _) => {
            h2051
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2052) => {
            h2052
        }
    })
}

pub fn f182(v0: Adt7) -> Adt7 {
    Adt7::C0({
        let a2059 = match v0.clone() {
            Adt7::C0(h2053, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2053
            }
        };
        let a2060 = {
            let a2057 = {
                let a2055 = match v0.clone() {
                    Adt7::C0(_, h2054, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2054
                    }
                };
                fixed_u64::bit_not(a2055)
            };
            let a2058 = match v0.clone() {
                Adt7::C0(_, _, h2056, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2056
                }
            };
            fixed_u64::bit_and(a2057, a2058)
        };
        fixed_u64::bit_xor(a2059, a2060)
    }, {
        let a2067 = match v0.clone() {
            Adt7::C0(_, h2061, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2061
            }
        };
        let a2068 = {
            let a2065 = {
                let a2063 = match v0.clone() {
                    Adt7::C0(_, _, h2062, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2062
                    }
                };
                fixed_u64::bit_not(a2063)
            };
            let a2066 = match v0.clone() {
                Adt7::C0(_, _, _, h2064, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2064
                }
            };
            fixed_u64::bit_and(a2065, a2066)
        };
        fixed_u64::bit_xor(a2067, a2068)
    }, {
        let a2075 = match v0.clone() {
            Adt7::C0(_, _, h2069, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2069
            }
        };
        let a2076 = {
            let a2073 = {
                let a2071 = match v0.clone() {
                    Adt7::C0(_, _, _, h2070, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2070
                    }
                };
                fixed_u64::bit_not(a2071)
            };
            let a2074 = match v0.clone() {
                Adt7::C0(_, _, _, _, h2072, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2072
                }
            };
            fixed_u64::bit_and(a2073, a2074)
        };
        fixed_u64::bit_xor(a2075, a2076)
    }, {
        let a2083 = match v0.clone() {
            Adt7::C0(_, _, _, h2077, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2077
            }
        };
        let a2084 = {
            let a2081 = {
                let a2079 = match v0.clone() {
                    Adt7::C0(_, _, _, _, h2078, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2078
                    }
                };
                fixed_u64::bit_not(a2079)
            };
            let a2082 = match v0.clone() {
                Adt7::C0(h2080, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2080
                }
            };
            fixed_u64::bit_and(a2081, a2082)
        };
        fixed_u64::bit_xor(a2083, a2084)
    }, {
        let a2091 = match v0.clone() {
            Adt7::C0(_, _, _, _, h2085, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2085
            }
        };
        let a2092 = {
            let a2089 = {
                let a2087 = match v0.clone() {
                    Adt7::C0(h2086, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2086
                    }
                };
                fixed_u64::bit_not(a2087)
            };
            let a2090 = match v0.clone() {
                Adt7::C0(_, h2088, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2088
                }
            };
            fixed_u64::bit_and(a2089, a2090)
        };
        fixed_u64::bit_xor(a2091, a2092)
    }, {
        let a2099 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, h2093, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2093
            }
        };
        let a2100 = {
            let a2097 = {
                let a2095 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, h2094, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2094
                    }
                };
                fixed_u64::bit_not(a2095)
            };
            let a2098 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, h2096, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2096
                }
            };
            fixed_u64::bit_and(a2097, a2098)
        };
        fixed_u64::bit_xor(a2099, a2100)
    }, {
        let a2107 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, h2101, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2101
            }
        };
        let a2108 = {
            let a2105 = {
                let a2103 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, h2102, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2102
                    }
                };
                fixed_u64::bit_not(a2103)
            };
            let a2106 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, h2104, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2104
                }
            };
            fixed_u64::bit_and(a2105, a2106)
        };
        fixed_u64::bit_xor(a2107, a2108)
    }, {
        let a2115 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h2109, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2109
            }
        };
        let a2116 = {
            let a2113 = {
                let a2111 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, h2110, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2110
                    }
                };
                fixed_u64::bit_not(a2111)
            };
            let a2114 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, h2112, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2112
                }
            };
            fixed_u64::bit_and(a2113, a2114)
        };
        fixed_u64::bit_xor(a2115, a2116)
    }, {
        let a2123 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h2117, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2117
            }
        };
        let a2124 = {
            let a2121 = {
                let a2119 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h2118, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2118
                    }
                };
                fixed_u64::bit_not(a2119)
            };
            let a2122 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, h2120, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2120
                }
            };
            fixed_u64::bit_and(a2121, a2122)
        };
        fixed_u64::bit_xor(a2123, a2124)
    }, {
        let a2131 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h2125, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2125
            }
        };
        let a2132 = {
            let a2129 = {
                let a2127 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, h2126, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2126
                    }
                };
                fixed_u64::bit_not(a2127)
            };
            let a2130 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, h2128, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2128
                }
            };
            fixed_u64::bit_and(a2129, a2130)
        };
        fixed_u64::bit_xor(a2131, a2132)
    }, {
        let a2139 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2133, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2133
            }
        };
        let a2140 = {
            let a2137 = {
                let a2135 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2134, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2134
                    }
                };
                fixed_u64::bit_not(a2135)
            };
            let a2138 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2136, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2136
                }
            };
            fixed_u64::bit_and(a2137, a2138)
        };
        fixed_u64::bit_xor(a2139, a2140)
    }, {
        let a2147 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2141, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2141
            }
        };
        let a2148 = {
            let a2145 = {
                let a2143 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2142, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2142
                    }
                };
                fixed_u64::bit_not(a2143)
            };
            let a2146 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2144, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2144
                }
            };
            fixed_u64::bit_and(a2145, a2146)
        };
        fixed_u64::bit_xor(a2147, a2148)
    }, {
        let a2155 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2149, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2149
            }
        };
        let a2156 = {
            let a2153 = {
                let a2151 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2150, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2150
                    }
                };
                fixed_u64::bit_not(a2151)
            };
            let a2154 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2152, _, _, _, _, _, _, _, _, _, _) => {
                    h2152
                }
            };
            fixed_u64::bit_and(a2153, a2154)
        };
        fixed_u64::bit_xor(a2155, a2156)
    }, {
        let a2163 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2157, _, _, _, _, _, _, _, _, _, _, _) => {
                h2157
            }
        };
        let a2164 = {
            let a2161 = {
                let a2159 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2158, _, _, _, _, _, _, _, _, _, _) => {
                        h2158
                    }
                };
                fixed_u64::bit_not(a2159)
            };
            let a2162 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2160, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2160
                }
            };
            fixed_u64::bit_and(a2161, a2162)
        };
        fixed_u64::bit_xor(a2163, a2164)
    }, {
        let a2171 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2165, _, _, _, _, _, _, _, _, _, _) => {
                h2165
            }
        };
        let a2172 = {
            let a2169 = {
                let a2167 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2166, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2166
                    }
                };
                fixed_u64::bit_not(a2167)
            };
            let a2170 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2168, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2168
                }
            };
            fixed_u64::bit_and(a2169, a2170)
        };
        fixed_u64::bit_xor(a2171, a2172)
    }, {
        let a2179 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2173, _, _, _, _, _, _, _, _, _) => {
                h2173
            }
        };
        let a2180 = {
            let a2177 = {
                let a2175 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2174, _, _, _, _, _, _, _, _) => {
                        h2174
                    }
                };
                fixed_u64::bit_not(a2175)
            };
            let a2178 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2176, _, _, _, _, _, _, _) => {
                    h2176
                }
            };
            fixed_u64::bit_and(a2177, a2178)
        };
        fixed_u64::bit_xor(a2179, a2180)
    }, {
        let a2187 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2181, _, _, _, _, _, _, _, _) => {
                h2181
            }
        };
        let a2188 = {
            let a2185 = {
                let a2183 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2182, _, _, _, _, _, _, _) => {
                        h2182
                    }
                };
                fixed_u64::bit_not(a2183)
            };
            let a2186 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2184, _, _, _, _, _, _) => {
                    h2184
                }
            };
            fixed_u64::bit_and(a2185, a2186)
        };
        fixed_u64::bit_xor(a2187, a2188)
    }, {
        let a2195 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2189, _, _, _, _, _, _, _) => {
                h2189
            }
        };
        let a2196 = {
            let a2193 = {
                let a2191 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2190, _, _, _, _, _, _) => {
                        h2190
                    }
                };
                fixed_u64::bit_not(a2191)
            };
            let a2194 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2192, _, _, _, _, _) => {
                    h2192
                }
            };
            fixed_u64::bit_and(a2193, a2194)
        };
        fixed_u64::bit_xor(a2195, a2196)
    }, {
        let a2203 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2197, _, _, _, _, _, _) => {
                h2197
            }
        };
        let a2204 = {
            let a2201 = {
                let a2199 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2198, _, _, _, _, _) => {
                        h2198
                    }
                };
                fixed_u64::bit_not(a2199)
            };
            let a2202 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2200, _, _, _, _, _, _, _, _, _) => {
                    h2200
                }
            };
            fixed_u64::bit_and(a2201, a2202)
        };
        fixed_u64::bit_xor(a2203, a2204)
    }, {
        let a2211 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2205, _, _, _, _, _) => {
                h2205
            }
        };
        let a2212 = {
            let a2209 = {
                let a2207 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2206, _, _, _, _, _, _, _, _, _) => {
                        h2206
                    }
                };
                fixed_u64::bit_not(a2207)
            };
            let a2210 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2208, _, _, _, _, _, _, _, _) => {
                    h2208
                }
            };
            fixed_u64::bit_and(a2209, a2210)
        };
        fixed_u64::bit_xor(a2211, a2212)
    }, {
        let a2219 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2213, _, _, _, _) => {
                h2213
            }
        };
        let a2220 = {
            let a2217 = {
                let a2215 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2214, _, _, _) => {
                        h2214
                    }
                };
                fixed_u64::bit_not(a2215)
            };
            let a2218 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2216, _, _) => {
                    h2216
                }
            };
            fixed_u64::bit_and(a2217, a2218)
        };
        fixed_u64::bit_xor(a2219, a2220)
    }, {
        let a2227 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2221, _, _, _) => {
                h2221
            }
        };
        let a2228 = {
            let a2225 = {
                let a2223 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2222, _, _) => {
                        h2222
                    }
                };
                fixed_u64::bit_not(a2223)
            };
            let a2226 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2224, _) => {
                    h2224
                }
            };
            fixed_u64::bit_and(a2225, a2226)
        };
        fixed_u64::bit_xor(a2227, a2228)
    }, {
        let a2235 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2229, _, _) => {
                h2229
            }
        };
        let a2236 = {
            let a2233 = {
                let a2231 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2230, _) => {
                        h2230
                    }
                };
                fixed_u64::bit_not(a2231)
            };
            let a2234 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2232) => {
                    h2232
                }
            };
            fixed_u64::bit_and(a2233, a2234)
        };
        fixed_u64::bit_xor(a2235, a2236)
    }, {
        let a2243 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2237, _) => {
                h2237
            }
        };
        let a2244 = {
            let a2241 = {
                let a2239 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2238) => {
                        h2238
                    }
                };
                fixed_u64::bit_not(a2239)
            };
            let a2242 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2240, _, _, _, _) => {
                    h2240
                }
            };
            fixed_u64::bit_and(a2241, a2242)
        };
        fixed_u64::bit_xor(a2243, a2244)
    }, {
        let a2251 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2245) => {
                h2245
            }
        };
        let a2252 = {
            let a2249 = {
                let a2247 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2246, _, _, _, _) => {
                        h2246
                    }
                };
                fixed_u64::bit_not(a2247)
            };
            let a2250 = match v0 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2248, _, _, _) => {
                    h2248
                }
            };
            fixed_u64::bit_and(a2249, a2250)
        };
        fixed_u64::bit_xor(a2251, a2252)
    })
}

pub fn f183(v0: Adt7) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h2253, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2253
        }
    }, f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h2254, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2254
        }
    }, 44u32, 20u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2255, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2255
        }
    }, 43u32, 21u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2256, _, _, _, _, _, _) => {
            h2256
        }
    }, 21u32, 43u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2257) => {
            h2257
        }
    }, 14u32, 50u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, h2258, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2258
        }
    }, 28u32, 36u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h2259, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2259
        }
    }, 20u32, 44u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2260, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2260
        }
    }, 3u32, 61u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2261, _, _, _, _, _, _, _, _) => {
            h2261
        }
    }, 45u32, 19u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2262, _, _) => {
            h2262
        }
    }, 61u32, 3u32), f186(match v0.clone() {
        Adt7::C0(_, h2263, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2263
        }
    }, 1u32, 63u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h2264, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2264
        }
    }, 6u32, 58u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2265, _, _, _, _, _, _, _, _, _, _, _) => {
            h2265
        }
    }, 25u32, 39u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2266, _, _, _, _, _) => {
            h2266
        }
    }, 8u32, 56u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2267, _, _, _, _) => {
            h2267
        }
    }, 18u32, 46u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, h2268, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2268
        }
    }, 27u32, 37u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h2269, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2269
        }
    }, 36u32, 28u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2270, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2270
        }
    }, 10u32, 54u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2271, _, _, _, _, _, _, _) => {
            h2271
        }
    }, 15u32, 49u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2272, _) => {
            h2272
        }
    }, 56u32, 8u32), f186(match v0.clone() {
        Adt7::C0(_, _, h2273, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2273
        }
    }, 62u32, 2u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h2274, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2274
        }
    }, 55u32, 9u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2275, _, _, _, _, _, _, _, _, _, _) => {
            h2275
        }
    }, 39u32, 25u32), f186(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2276, _, _, _, _, _, _, _, _, _) => {
            h2276
        }
    }, 41u32, 23u32), f186(match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2277, _, _, _) => {
            h2277
        }
    }, 2u32, 62u32))
}

pub fn f184(v0: Adt7) -> Adt7 {
    let v1: u64 = {
        let a2289 = {
            let a2286 = {
                let a2283 = {
                    let a2280 = match v0.clone() {
                        Adt7::C0(h2278, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2278
                        }
                    };
                    let a2281 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, h2279, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2279
                        }
                    };
                    fixed_u64::bit_xor(a2280, a2281)
                };
                let a2284 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2282, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2282
                    }
                };
                fixed_u64::bit_xor(a2283, a2284)
            };
            let a2287 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2285, _, _, _, _, _, _, _, _, _) => {
                    h2285
                }
            };
            fixed_u64::bit_xor(a2286, a2287)
        };
        let a2290 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2288, _, _, _, _) => {
                h2288
            }
        };
        fixed_u64::bit_xor(a2289, a2290)
    };
    let v2: u64 = {
        let a2302 = {
            let a2299 = {
                let a2296 = {
                    let a2293 = match v0.clone() {
                        Adt7::C0(_, h2291, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2291
                        }
                    };
                    let a2294 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, h2292, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2292
                        }
                    };
                    fixed_u64::bit_xor(a2293, a2294)
                };
                let a2297 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2295, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2295
                    }
                };
                fixed_u64::bit_xor(a2296, a2297)
            };
            let a2300 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2298, _, _, _, _, _, _, _, _) => {
                    h2298
                }
            };
            fixed_u64::bit_xor(a2299, a2300)
        };
        let a2303 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2301, _, _, _) => {
                h2301
            }
        };
        fixed_u64::bit_xor(a2302, a2303)
    };
    let v3: u64 = {
        let a2315 = {
            let a2312 = {
                let a2309 = {
                    let a2306 = match v0.clone() {
                        Adt7::C0(_, _, h2304, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2304
                        }
                    };
                    let a2307 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, h2305, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2305
                        }
                    };
                    fixed_u64::bit_xor(a2306, a2307)
                };
                let a2310 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2308, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2308
                    }
                };
                fixed_u64::bit_xor(a2309, a2310)
            };
            let a2313 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2311, _, _, _, _, _, _, _) => {
                    h2311
                }
            };
            fixed_u64::bit_xor(a2312, a2313)
        };
        let a2316 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2314, _, _) => {
                h2314
            }
        };
        fixed_u64::bit_xor(a2315, a2316)
    };
    let v4: u64 = {
        let a2328 = {
            let a2325 = {
                let a2322 = {
                    let a2319 = match v0.clone() {
                        Adt7::C0(_, _, _, h2317, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2317
                        }
                    };
                    let a2320 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, _, h2318, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2318
                        }
                    };
                    fixed_u64::bit_xor(a2319, a2320)
                };
                let a2323 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2321, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2321
                    }
                };
                fixed_u64::bit_xor(a2322, a2323)
            };
            let a2326 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2324, _, _, _, _, _, _) => {
                    h2324
                }
            };
            fixed_u64::bit_xor(a2325, a2326)
        };
        let a2329 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2327, _) => {
                h2327
            }
        };
        fixed_u64::bit_xor(a2328, a2329)
    };
    let v5: u64 = {
        let a2341 = {
            let a2338 = {
                let a2335 = {
                    let a2332 = match v0.clone() {
                        Adt7::C0(_, _, _, _, h2330, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2330
                        }
                    };
                    let a2333 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, _, _, h2331, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2331
                        }
                    };
                    fixed_u64::bit_xor(a2332, a2333)
                };
                let a2336 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2334, _, _, _, _, _, _, _, _, _, _) => {
                        h2334
                    }
                };
                fixed_u64::bit_xor(a2335, a2336)
            };
            let a2339 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2337, _, _, _, _, _) => {
                    h2337
                }
            };
            fixed_u64::bit_xor(a2338, a2339)
        };
        let a2342 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2340) => {
                h2340
            }
        };
        fixed_u64::bit_xor(a2341, a2342)
    };
    let v6: u64 = {
        let a2343 = v5;
        let a2344 = f186(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a2343, a2344)
    };
    let v7: u64 = {
        let a2345 = v1;
        let a2346 = f186(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a2345, a2346)
    };
    let v8: u64 = {
        let a2347 = v2;
        let a2348 = f186(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a2347, a2348)
    };
    let v9: u64 = {
        let a2349 = v3;
        let a2350 = f186(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a2349, a2350)
    };
    let v10: u64 = {
        let a2351 = v4;
        let a2352 = f186(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a2351, a2352)
    };
    Adt7::C0({
        let a2354 = match v0.clone() {
            Adt7::C0(h2353, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2353
            }
        };
        let a2355 = v6;
        fixed_u64::bit_xor(a2354, a2355)
    }, {
        let a2357 = match v0.clone() {
            Adt7::C0(_, h2356, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2356
            }
        };
        let a2358 = v7;
        fixed_u64::bit_xor(a2357, a2358)
    }, {
        let a2360 = match v0.clone() {
            Adt7::C0(_, _, h2359, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2359
            }
        };
        let a2361 = v8;
        fixed_u64::bit_xor(a2360, a2361)
    }, {
        let a2363 = match v0.clone() {
            Adt7::C0(_, _, _, h2362, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2362
            }
        };
        let a2364 = v9;
        fixed_u64::bit_xor(a2363, a2364)
    }, {
        let a2366 = match v0.clone() {
            Adt7::C0(_, _, _, _, h2365, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2365
            }
        };
        let a2367 = v10;
        fixed_u64::bit_xor(a2366, a2367)
    }, {
        let a2369 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, h2368, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2368
            }
        };
        let a2370 = v6;
        fixed_u64::bit_xor(a2369, a2370)
    }, {
        let a2372 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, h2371, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2371
            }
        };
        let a2373 = v7;
        fixed_u64::bit_xor(a2372, a2373)
    }, {
        let a2375 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h2374, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2374
            }
        };
        let a2376 = v8;
        fixed_u64::bit_xor(a2375, a2376)
    }, {
        let a2378 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h2377, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2377
            }
        };
        let a2379 = v9;
        fixed_u64::bit_xor(a2378, a2379)
    }, {
        let a2381 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h2380, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2380
            }
        };
        let a2382 = v10;
        fixed_u64::bit_xor(a2381, a2382)
    }, {
        let a2384 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2383, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2383
            }
        };
        let a2385 = v6;
        fixed_u64::bit_xor(a2384, a2385)
    }, {
        let a2387 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2386, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2386
            }
        };
        let a2388 = v7;
        fixed_u64::bit_xor(a2387, a2388)
    }, {
        let a2390 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2389, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2389
            }
        };
        let a2391 = v8;
        fixed_u64::bit_xor(a2390, a2391)
    }, {
        let a2393 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2392, _, _, _, _, _, _, _, _, _, _, _) => {
                h2392
            }
        };
        let a2394 = v9;
        fixed_u64::bit_xor(a2393, a2394)
    }, {
        let a2396 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2395, _, _, _, _, _, _, _, _, _, _) => {
                h2395
            }
        };
        let a2397 = v10;
        fixed_u64::bit_xor(a2396, a2397)
    }, {
        let a2399 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2398, _, _, _, _, _, _, _, _, _) => {
                h2398
            }
        };
        let a2400 = v6;
        fixed_u64::bit_xor(a2399, a2400)
    }, {
        let a2402 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2401, _, _, _, _, _, _, _, _) => {
                h2401
            }
        };
        let a2403 = v7;
        fixed_u64::bit_xor(a2402, a2403)
    }, {
        let a2405 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2404, _, _, _, _, _, _, _) => {
                h2404
            }
        };
        let a2406 = v8;
        fixed_u64::bit_xor(a2405, a2406)
    }, {
        let a2408 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2407, _, _, _, _, _, _) => {
                h2407
            }
        };
        let a2409 = v9;
        fixed_u64::bit_xor(a2408, a2409)
    }, {
        let a2411 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2410, _, _, _, _, _) => {
                h2410
            }
        };
        let a2412 = v10;
        fixed_u64::bit_xor(a2411, a2412)
    }, {
        let a2414 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2413, _, _, _, _) => {
                h2413
            }
        };
        let a2415 = v6;
        fixed_u64::bit_xor(a2414, a2415)
    }, {
        let a2417 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2416, _, _, _) => {
                h2416
            }
        };
        let a2418 = v7;
        fixed_u64::bit_xor(a2417, a2418)
    }, {
        let a2420 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2419, _, _) => {
                h2419
            }
        };
        let a2421 = v8;
        fixed_u64::bit_xor(a2420, a2421)
    }, {
        let a2423 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2422, _) => {
                h2422
            }
        };
        let a2424 = v9;
        fixed_u64::bit_xor(a2423, a2424)
    }, {
        let a2426 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2425) => {
                h2425
            }
        };
        let a2427 = v10;
        fixed_u64::bit_xor(a2426, a2427)
    })
}

pub fn f185(v0: u64) -> u64 {
    let a2428 = v0;
    let a2429 = 12u64;
    if nat_lt(a2428, a2429) {
        let a2430 = v0;
        let a2431 = 6u64;
        if nat_lt(a2430, a2431) {
            let a2432 = v0;
            let a2433 = 3u64;
            if nat_lt(a2432, a2433) {
                let a2434 = v0;
                let a2435 = 1u64;
                if nat_lt(a2434, a2435) {
                    1u64
                } else {
                    let a2436 = v0;
                    let a2437 = 2u64;
                    if nat_lt(a2436, a2437) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2438 = v0;
                let a2439 = 4u64;
                if nat_lt(a2438, a2439) {
                    9223372039002292224u64
                } else {
                    let a2440 = v0;
                    let a2441 = 5u64;
                    if nat_lt(a2440, a2441) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2442 = v0;
            let a2443 = 9u64;
            if nat_lt(a2442, a2443) {
                let a2444 = v0;
                let a2445 = 7u64;
                if nat_lt(a2444, a2445) {
                    9223372039002292353u64
                } else {
                    let a2446 = v0;
                    let a2447 = 8u64;
                    if nat_lt(a2446, a2447) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2448 = v0;
                let a2449 = 10u64;
                if nat_lt(a2448, a2449) {
                    136u64
                } else {
                    let a2450 = v0;
                    let a2451 = 11u64;
                    if nat_lt(a2450, a2451) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2452 = v0;
        let a2453 = 18u64;
        if nat_lt(a2452, a2453) {
            let a2454 = v0;
            let a2455 = 15u64;
            if nat_lt(a2454, a2455) {
                let a2456 = v0;
                let a2457 = 13u64;
                if nat_lt(a2456, a2457) {
                    2147516555u64
                } else {
                    let a2458 = v0;
                    let a2459 = 14u64;
                    if nat_lt(a2458, a2459) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2460 = v0;
                let a2461 = 16u64;
                if nat_lt(a2460, a2461) {
                    9223372036854808579u64
                } else {
                    let a2462 = v0;
                    let a2463 = 17u64;
                    if nat_lt(a2462, a2463) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2464 = v0;
            let a2465 = 21u64;
            if nat_lt(a2464, a2465) {
                let a2466 = v0;
                let a2467 = 19u64;
                if nat_lt(a2466, a2467) {
                    32778u64
                } else {
                    let a2468 = v0;
                    let a2469 = 20u64;
                    if nat_lt(a2468, a2469) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2470 = v0;
                let a2471 = 22u64;
                if nat_lt(a2470, a2471) {
                    9223372036854808704u64
                } else {
                    let a2472 = v0;
                    let a2473 = 23u64;
                    if nat_lt(a2472, a2473) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f186(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2474 = f140(v0, v1);
    let a2475 = f94(v0, v2);
    fixed_u64::bit_or(a2474, a2475)
}
