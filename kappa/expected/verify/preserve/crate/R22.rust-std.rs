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
    C0(u64, u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, bool, bool, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(bool, Bytes, List<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool, List<u64>, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt8, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64, u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(bool, Adt6),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(List<Adt13>, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<Adt0>, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, bool, List<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F18(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn0::F18(k0) => f18(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, bool), p1: Bytes) -> R<(bool, bool)> {
        match self {
            Fn1::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F24,
    F25,
    F57(u64),
    F58,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn2::F24 => f24(p0, p1),
            Fn2::F25 => Ok(f25(p0, p1)),
            Fn2::F57(k0) => f57(*k0, p0, p1),
            Fn2::F58 => Ok(f58(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28,
    F149,
}

impl Fn3 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> R<Bytes> {
        match self {
            Fn3::F28 => f28(p0, p1),
            Fn3::F149 => f149(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn4::F31 => f31(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F33,
    F112,
    F126(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> R<(bool, Bytes)> {
        match self {
            Fn5::F33 => f33(p0, p1),
            Fn5::F112 => f112(p0, p1),
            Fn5::F126(k0) => f126(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F36,
    F37(Bytes),
    F67(Bytes),
    F68,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn6::F36 => Ok(f36(p0)),
            Fn6::F37(k0) => f37(k0.clone(), p0),
            Fn6::F67(k0) => f67(k0.clone(), p0),
            Fn6::F68 => Ok(f68(p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F41(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn7::F41(k0, k1) => f41(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F43,
    F164,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn8::F43 => f43(p0, p1),
            Fn8::F164 => f164(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F45(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt0) -> List<Bytes> {
        match self {
            Fn9::F45(k0) => f45(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F50,
}

impl Fn10 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn10::F50 => f50(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F52,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Bytes>, List<(Bytes, Bytes)>), p1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
        match self {
            Fn11::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F54,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> List<Bytes> {
        match self {
            Fn12::F54 => f54(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F64(Bytes, u64, u64),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<(u64, Adt4)> {
        match self {
            Fn13::F64(k0, k1, k2) => f64(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F71,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn14::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F74,
    F96,
    F114,
    F151,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn15::F74 => f74(p0, p1),
            Fn15::F96 => f96(p0, p1),
            Fn15::F114 => f114(p0, p1),
            Fn15::F151 => f151(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F85(Bytes, Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Adt5, p1: (Bytes, Bytes)) -> Adt5 {
        match self {
            Fn16::F85(k0, k1) => f85(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F87,
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, (bool, List<Adt6>)), p1: Bytes) -> R<(u64, (bool, List<Adt6>))> {
        match self {
            Fn17::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F90(List<Adt6>),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt7) -> Adt7 {
        match self {
            Fn18::F90(k0) => f90(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F95(u64),
}

impl Fn19 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Adt6) -> R<(bool, List<Adt3>)> {
        match self {
            Fn19::F95(k0) => f95(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F98,
}

impl Fn20 {
    pub fn apply(&self, p0: Adt9) -> R<Adt9> {
        match self {
            Fn20::F98 => f98(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F102(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn21::F102(k0) => f102(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F109,
}

impl Fn22 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn22::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F123(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn23::F123(k0, k1) => f123(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F125(List<Adt6>, Adt7),
}

impl Fn24 {
    pub fn apply(&self, p0: Adt11, p1: Adt6) -> Adt11 {
        match self {
            Fn24::F125(k0, k1) => f125(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F131(u64),
}

impl Fn25 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Bytes) -> R<(bool, List<Adt3>)> {
        match self {
            Fn25::F131(k0) => f131(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F139,
}

impl Fn26 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn26::F139 => f139(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F145,
}

impl Fn27 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn27::F145 => f145(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F148(u64),
}

impl Fn28 {
    pub fn apply(&self, p0: u64, p1: Adt0) -> R<u64> {
        match self {
            Fn28::F148(k0) => f148(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F156(u64),
}

impl Fn29 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn29::F156(k0) => f156(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F158(List<u64>, List<Adt6>),
    F193(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn30::F158(k0, k1) => f158(k0.clone(), k1.clone(), p0, p1),
            Fn30::F193(k0) => f193(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F166(u64),
}

impl Fn31 {
    pub fn apply(&self, p0: List<Adt3>, p1: Bytes) -> R<List<Adt3>> {
        match self {
            Fn31::F166(k0) => f166(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F169(List<u64>, Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: bool, p1: Adt6) -> bool {
        match self {
            Fn32::F169(k0, k1) => f169(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F171(u64),
}

impl Fn33 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: Adt0) -> R<(bool, List<u64>)> {
        match self {
            Fn33::F171(k0) => f171(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F173,
    F186,
    F187,
}

impl Fn34 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn34::F173 => f173(p0, p1),
            Fn34::F186 => f186(p0, p1),
            Fn34::F187 => f187(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F177,
}

impl Fn35 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn35::F177 => f177(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F179(u64),
}

impl Fn36 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn36::F179(k0) => f179(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F189(u64),
    F192,
}

impl Fn37 {
    pub fn apply(&self, p0: Bytes, p1: Adt0) -> R<Bytes> {
        match self {
            Fn37::F189(k0) => f189(*k0, p0, p1),
            Fn37::F192 => f192(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F191(Adt0),
}

impl Fn38 {
    pub fn apply(&self, p0: Adt14, p1: Adt0) -> Adt14 {
        match self {
            Fn38::F191(k0) => f191(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F195(Bytes),
}

impl Fn39 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn39::F195(k0) => f195(k0.clone(), p0),
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
    let v1: (bool, List<Adt0>) = f3(v0.clone())?;
    let v2: List<Adt0> = {
        let (_, h2) = v1;
        h2
    };
    let v3: Adt0 = f4(v2.clone(), 1u64);
    let v4: (bool, Bytes) = f5(v2.clone())?;
    let v5: Bytes = f6(f4(v2.clone(), 7u64));
    let v6: (bool, Bytes) = f7(v5.clone())?;
    let v7: (bool, Bytes) = f8(v0, 14u64)?;
    let (h3, _) = v1;
    if if if if h3 {
        f9(v3.clone())
    } else {
        false
    } {
        let a5 = 0u64;
        let a6 = match v3.clone() {
            Adt0::C0(_, _, h4, _) => {
                h4
            }
        };
        if if nat_lt(a5, a6) {
            let a8 = match v3.clone() {
                Adt0::C0(_, _, h7, _) => {
                    h7
                }
            };
            let a9 = 13u64;
            nat_le(a8, a9)
        } else {
            false
        } {
            let (h10, _) = v4;
            h10
        } else {
            false
        }
    } else {
        false
    } {
        let a12 = 0u64;
        let a13 = {
            let a11 = v5;
            length_bytes(a11)
        };
        if if nat_lt(a12, a13) {
            let (h14, _) = v6;
            h14
        } else {
            false
        } {
            let (h15, _) = v7;
            h15
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a32 = f10(match v3 {
                Adt0::C0(_, _, h16, _) => {
                    h16
                }
            }, 8u64)?;
            let a33 = {
                let a30 = {
                    let (_, h17) = v4;
                    h17
                };
                let a31 = {
                    let a28 = {
                        let (_, h18) = v6;
                        h18
                    };
                    let a29 = {
                        let a26 = f11(f6(f4(v2.clone(), 2u64)))?;
                        let a27 = {
                            let a24 = f11(f6(f4(v2.clone(), 3u64)))?;
                            let a25 = {
                                let a22 = f11(f6(f4(v2.clone(), 4u64)))?;
                                let a23 = {
                                    let a20 = f10(f12(f4(v2, 5u64)), 8u64)?;
                                    let a21 = {
                                        let (_, h19) = v7;
                                        h19
                                    };
                                    append_bytes(a20, a21)
                                };
                                append_bytes(a22, a23)
                            };
                            append_bytes(a24, a25)
                        };
                        append_bytes(a26, a27)
                    };
                    append_bytes(a28, a29)
                };
                append_bytes(a30, a31)
            };
            append_bytes(a32, a33)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    let a34 = f13(Str::lit("sha256:"));
    let a35 = f14(v0)?;
    Ok(append_bytes(a34, a35))
}

pub fn f3(v0: Bytes) -> R<(bool, List<Adt0>)> {
    let v1: Adt1 = f15(v0, 0u64)?;
    Ok((match v1.clone() {
        Adt1::C0(_, h36, _, _) => {
            h36
        }
    }, f16(match v1 {
        Adt1::C0(_, _, _, h37) => {
            h37
        }
    })))
}

pub fn f4(v0: List<Adt0>, v1: u64) -> Adt0 {
    f17(Fn0::F18(v1), f19(), v0)
}

pub fn f5(v0: List<Adt0>) -> R<(bool, Bytes)> {
    let v1: List<Bytes> = f20(v0, 8u64);
    let v2: (bool, bool) = f21(Fn1::F22, (true, false), v1.clone())?;
    let v3: List<Bytes> = f23(Fn2::F24, List::<Bytes>::nil(), f23(Fn2::F25, List::<Bytes>::nil(), v1.clone())?)?;
    Ok(({
        let (h38, _) = v2;
        if h38 {
            let (_, h39) = v2;
            if h39 {
                true
            } else {
                let a40 = f26(v1.clone())?;
                let a41 = 0u64;
                nat_eq(a40, a41)
            }
        } else {
            false
        }
    }, f27(Fn3::F28, Bytes::lit(&[]), f29(v1, v3))?))
}

pub fn f6(v0: Adt0) -> Bytes {
    let a43 = match v0.clone() {
        Adt0::C0(_, h42, _, _) => {
            h42
        }
    };
    let a44 = 2u64;
    if nat_eq(a43, a44) {
        match v0 {
            Adt0::C0(_, _, _, h45) => {
                h45
            }
        }
    } else {
        Bytes::lit(&[])
    }
}

pub fn f7(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: Adt2 = {
        let (h46, _) = f30(Fn4::F31, {
            let a50 = {
                let a48 = {
                    let a47 = v0.clone();
                    length_bytes(a47)
                };
                let a49 = 8u64;
                nat_mul(a48, a49)?
            };
            let a51 = 16u64;
            nat_add(a50, a51)?
        }, Adt2::C0(true, Bytes::lit(&[]), List::cons(Adt3::C0(true, v0, 0u64), List::<Adt3>::nil())))?;
        h46
    };
    let v4: bool = {
        let m53 = match v1.clone() {
            Adt2::C0(_, _, h52) => {
                h52
            }
        };
        match m53.uncons() {
            None => {
                true
            }
            Some((_, _)) => {
                false
            }
        }
    };
    Ok((if match v1.clone() {
        Adt2::C0(h54, _, _) => {
            h54
        }
    } {
        v4
    } else {
        false
    }, match v1 {
        Adt2::C0(_, h55, _) => {
            h55
        }
    }))
}

pub fn f8(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f3(v0)?;
    let v3: List<Bytes> = f20({
        let (_, h56) = v2;
        h56
    }, v1);
    f32(Fn5::F33, ({
        let (h57, _) = v2;
        h57
    }, f10(f26(v3.clone())?, 4u64)?), f29(v3.clone(), f34(v3, 1u64)?))
}

pub fn f9(v0: Adt0) -> bool {
    let a59 = 0u64;
    let a60 = match v0.clone() {
        Adt0::C0(h58, _, _, _) => {
            h58
        }
    };
    if nat_lt(a59, a60) {
        let a62 = match v0 {
            Adt0::C0(_, h61, _, _) => {
                h61
            }
        };
        let a63 = 0u64;
        nat_eq(a62, a63)
    } else {
        false
    }
}

pub fn f10(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h64) = f35(Fn6::F36, v1, (v0, Bytes::lit(&[])))?;
    Ok(h64)
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f14(v0)?;
    let (_, h65) = f35(Fn6::F37(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h65)
}

pub fn f12(v0: Adt0) -> u64 {
    let a67 = 0u64;
    let a68 = match v0.clone() {
        Adt0::C0(h66, _, _, _) => {
            h66
        }
    };
    if if nat_lt(a67, a68) {
        let a70 = match v0.clone() {
            Adt0::C0(_, h69, _, _) => {
                h69
            }
        };
        let a71 = 0u64;
        nat_eq(a70, a71)
    } else {
        false
    } {
        match v0 {
            Adt0::C0(_, _, h72, _) => {
                h72
            }
        }
    } else {
        0u64
    }
}

pub fn f13(v0: Str) -> Bytes {
    let a73 = v0;
    utf8_encode(a73)
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    Ok(f38(f39(v0)?))
}

pub fn f15(v0: Bytes, v1: u64) -> R<Adt1> {
    let (h74, _) = f40(Fn7::F41(v1, v0.clone()), {
        let a76 = {
            let a75 = v0;
            length_bytes(a75)
        };
        let a77 = 2u64;
        nat_add(a76, a77)?
    }, Adt1::C0(0u64, true, false, List::<Adt0>::nil()))?;
    Ok(h74)
}

pub fn f16(v0: List<Adt0>) -> List<Adt0> {
    f42(Fn8::F43, List::<Adt0>::nil(), v0)
}

pub fn f17(v0: Fn0, p81: Adt0, p82: List<Adt0>) -> Adt0 {
    let m80 = (p81, p82);
    {
        let mut m80 = m80;
        loop {
            let step = {
                let (v1, v2) = m80;
                let m78 = v2;
                match m78.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c79: Fn0 = v0.clone();
                            c79.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m80 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: Adt0, v2: Adt0) -> Adt0 {
    let a84 = 0u64;
    let a85 = match v1.clone() {
        Adt0::C0(h83, _, _, _) => {
            h83
        }
    };
    if nat_lt(a84, a85) {
        v1
    } else {
        let a87 = match v2.clone() {
            Adt0::C0(h86, _, _, _) => {
                h86
            }
        };
        let a88 = v0;
        if nat_eq(a87, a88) {
            v2
        } else {
            v1
        }
    }
}

pub fn f19() -> Adt0 {
    Adt0::C0(0u64, 0u64, 0u64, Bytes::lit(&[]))
}

pub fn f20(v0: List<Adt0>, v1: u64) -> List<Bytes> {
    f44(Fn9::F45(v1), List::<Bytes>::nil(), f16(v0))
}

pub fn f21(v0: Fn1, p92: (bool, bool), p93: List<Bytes>) -> R<(bool, bool)> {
    let m91 = (p92, p93);
    {
        let mut m91 = m91;
        loop {
            let step = {
                let (v1, v2) = m91;
                let m89 = v2;
                match m89.uncons() {
                    None => {
                        Err::<((bool, bool), List<Bytes>), R<(bool, bool)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, bool), List<Bytes>), R<(bool, bool)>>(({
                            let c90: Fn1 = v0.clone();
                            c90.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m91 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: (bool, bool), v1: Bytes) -> R<(bool, bool)> {
    let v2: (bool, Adt0) = f46(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f46(v1, 2u64)?;
    Ok(({
        let (h94, _) = v0;
        if h94 {
            let (h95, _) = v2;
            if h95 {
                let (h96, _) = v3.clone();
                h96
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let (_, h97) = v0;
        if h97 {
            true
        } else {
            let a100 = {
                let a99 = f6({
                    let (_, h98) = v2;
                    h98
                });
                length_bytes(a99)
            };
            let a101 = 0u64;
            if nat_eq(a100, a101) {
                let a103 = 0u64;
                let a104 = f12({
                    let (_, h102) = v3.clone();
                    h102
                });
                if nat_lt(a103, a104) {
                    let a106 = f12({
                        let (_, h105) = v3;
                        h105
                    });
                    let a107 = 9223372036854775807u64;
                    nat_le(a106, a107)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }))
}

pub fn f23(v0: Fn2, p111: List<Bytes>, p112: List<Bytes>) -> R<List<Bytes>> {
    let m110 = (p111, p112);
    {
        let mut m110 = m110;
        loop {
            let step = {
                let (v1, v2) = m110;
                let m108 = v2;
                match m108.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c109: Fn2 = v0.clone();
                            c109.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons({
        let a115 = f47(f6({
            let (_, h113) = f46(v1.clone(), 1u64)?;
            h113
        }))?;
        let a116 = f48(f12({
            let (_, h114) = f46(v1, 2u64)?;
            h114
        }))?;
        append_bytes(a115, a116)
    }, v0))
}

pub fn f25(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f26(v0: List<Bytes>) -> R<u64> {
    f49(Fn10::F50, 0u64, v0)
}

pub fn f27(v0: Fn3, p120: Bytes, p121: List<Bytes>) -> R<Bytes> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m117 = v2;
                match m117.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), R<Bytes>>(({
                            let c118: Fn3 = v0.clone();
                            c118.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a126 = v0;
    let a127 = {
        let a124 = f11(f6({
            let (_, h122) = f46(v1.clone(), 1u64)?;
            h122
        }))?;
        let a125 = f10(f12({
            let (_, h123) = f46(v1, 2u64)?;
            h123
        }), 8u64)?;
        append_bytes(a124, a125)
    };
    Ok(append_bytes(a126, a127))
}

pub fn f29(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let v2: List<(Bytes, Bytes)> = {
        let (_, h128) = f51(Fn11::F52, (v1, List::<(Bytes, Bytes)>::nil()), v0);
        h128
    };
    f53(Fn12::F54, List::<Bytes>::nil(), f55(v2))
}

pub fn f30(v0: Fn4, p133: u64, p134: Adt2) -> R<(Adt2, bool)> {
    let m132 = (p133, p134);
    {
        let mut m132 = m132;
        loop {
            let step = {
                let (v1, v2) = m132;
                let m129 = v1;
                if m129 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m129 - 1;
                    let m131 = {
                        let c130: Fn4 = v0.clone();
                        c130.apply(v2.clone())?
                    };
                    match m131 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<Option<Adt2>> {
    f56(v0)
}

pub fn f32(v0: Fn5, p138: (bool, Bytes), p139: List<Bytes>) -> R<(bool, Bytes)> {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v1, v2) = m137;
                let m135 = v2;
                match m135.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(({
                            let c136: Fn5 = v0.clone();
                            c136.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f46(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f46(v1, 2u64)?;
    Ok(({
        let (h140, _) = v0;
        if h140 {
            let (h141, _) = v2;
            if h141 {
                let (h142, _) = v3;
                h142
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a148 = {
            let (_, h143) = v0;
            h143
        };
        let a149 = {
            let a146 = f11(f6({
                let (_, h144) = v2;
                h144
            }))?;
            let a147 = f11(f6({
                let (_, h145) = v3;
                h145
            }))?;
            append_bytes(a146, a147)
        };
        append_bytes(a148, a149)
    }))
}

pub fn f34(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    f23(Fn2::F57(v1), List::<Bytes>::nil(), f23(Fn2::F58, List::<Bytes>::nil(), v0)?)
}

pub fn f35(v0: Fn6, p153: u64, p154: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v1, v2) = m152;
                let m150 = v1;
                if m150 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m150 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c151: Fn6 = v0.clone();
                        c151.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a156 = {
            let (h155, _) = v0.clone();
            h155
        };
        let a157 = 256u64;
        let a158 = 0u64;
        nat_quot(a156, a157, a158)
    }, {
        let a164 = {
            let (_, h159) = v0;
            h159
        };
        let a165 = f59({
            let a161 = {
                let (h160, _) = v0;
                h160
            };
            let a162 = 256u64;
            let a163 = 0u64;
            nat_rem(a161, a162, a163)
        });
        append_bytes(a164, a165)
    })
}

pub fn f37(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a167 = {
            let (h166, _) = v1.clone();
            h166
        };
        let a168 = 2u64;
        nat_mul(a167, a168)?
    };
    Ok(({
        let a170 = {
            let (h169, _) = v1;
            h169
        };
        let a171 = 1u64;
        nat_add(a170, a171)?
    }, {
        let a179 = {
            let (_, h172) = v1;
            h172
        };
        let a180 = f59({
            let a177 = {
                let a173 = f60(f61(v0.clone(), v2)?);
                let a174 = 16u64;
                nat_mul(a173, a174)?
            };
            let a178 = f60(f61(v0, {
                let a175 = v2;
                let a176 = 1u64;
                nat_add(a175, a176)?
            })?);
            nat_add(a177, a178)?
        });
        append_bytes(a179, a180)
    }))
}

pub fn f38(v0: Adt4) -> Bytes {
    let a201 = f62(match v0.clone() {
        Adt4::C0(h181, _, _, _, _, _, _, _) => {
            h181
        }
    });
    let a202 = {
        let a199 = f62(match v0.clone() {
            Adt4::C0(_, h182, _, _, _, _, _, _) => {
                h182
            }
        });
        let a200 = {
            let a197 = f62(match v0.clone() {
                Adt4::C0(_, _, h183, _, _, _, _, _) => {
                    h183
                }
            });
            let a198 = {
                let a195 = f62(match v0.clone() {
                    Adt4::C0(_, _, _, h184, _, _, _, _) => {
                        h184
                    }
                });
                let a196 = {
                    let a193 = f62(match v0.clone() {
                        Adt4::C0(_, _, _, _, h185, _, _, _) => {
                            h185
                        }
                    });
                    let a194 = {
                        let a191 = f62(match v0.clone() {
                            Adt4::C0(_, _, _, _, _, h186, _, _) => {
                                h186
                            }
                        });
                        let a192 = {
                            let a189 = f62(match v0.clone() {
                                Adt4::C0(_, _, _, _, _, _, h187, _) => {
                                    h187
                                }
                            });
                            let a190 = f62(match v0 {
                                Adt4::C0(_, _, _, _, _, _, _, h188) => {
                                    h188
                                }
                            });
                            append_bytes(a189, a190)
                        };
                        append_bytes(a191, a192)
                    };
                    append_bytes(a193, a194)
                };
                append_bytes(a195, a196)
            };
            append_bytes(a197, a198)
        };
        append_bytes(a199, a200)
    };
    append_bytes(a201, a202)
}

pub fn f39(v0: Bytes) -> R<Adt4> {
    let v1: u64 = {
        let a203 = v0.clone();
        length_bytes(a203)
    };
    let v2: u64 = {
        let a209 = {
            let a206 = {
                let a204 = v1;
                let a205 = 72u64;
                nat_add(a204, a205)?
            };
            let a207 = 64u64;
            let a208 = 0u64;
            nat_quot(a206, a207, a208)
        };
        let a210 = 64u64;
        nat_mul(a209, a210)?
    };
    let (_, h211) = f63(Fn13::F64(v0, v1, v2), {
        let a212 = v2;
        let a213 = 64u64;
        let a214 = 0u64;
        nat_quot(a212, a213, a214)
    }, (0u64, f65()))?;
    Ok(h211)
}

pub fn f40(v0: Fn7, p219: u64, p220: Adt1) -> R<(Adt1, bool)> {
    let m218 = (p219, p220);
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, v2) = m218;
                let m215 = v1;
                if m215 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m215 - 1;
                    let m217 = {
                        let c216: Fn7 = v0.clone();
                        c216.apply(v2.clone())?
                    };
                    match m217 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: u64, v1: Bytes, v2: Adt1) -> R<Option<Adt1>> {
    f66(v1, v0, v2)
}

pub fn f42(v0: Fn8, p224: List<Adt0>, p225: List<Adt0>) -> List<Adt0> {
    let m223 = (p224, p225);
    {
        let mut m223 = m223;
        loop {
            let step = {
                let (v1, v2) = m223;
                let m221 = v2;
                match m221.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c222: Fn8 = v0.clone();
                            c222.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f44(v0: Fn9, p229: List<Bytes>, p230: List<Adt0>) -> List<Bytes> {
    let m228 = (p229, p230);
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v1, v2) = m228;
                let m226 = v2;
                match m226.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt0>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt0>), List<Bytes>>(({
                            let c227: Fn9 = v0.clone();
                            c227.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: List<Bytes>, v2: Adt0) -> List<Bytes> {
    let a232 = match v2.clone() {
        Adt0::C0(h231, _, _, _) => {
            h231
        }
    };
    let a233 = v0;
    if if nat_eq(a232, a233) {
        let a235 = match v2.clone() {
            Adt0::C0(_, h234, _, _) => {
                h234
            }
        };
        let a236 = 2u64;
        nat_eq(a235, a236)
    } else {
        false
    } {
        List::cons(match v2 {
            Adt0::C0(_, _, _, h237) => {
                h237
            }
        }, v1)
    } else {
        v1
    }
}

pub fn f46(v0: Bytes, v1: u64) -> R<(bool, Adt0)> {
    let v2: Adt1 = f15(v0, v1)?;
    if match v2.clone() {
        Adt1::C0(_, h238, _, _) => {
            h238
        }
    } {
        let m240 = match v2 {
            Adt1::C0(_, _, _, h239) => {
                h239
            }
        };
        match m240.uncons() {
            None => {
                Ok((true, f19()))
            }
            Some((v3, _)) => {
                Ok((true, {
                    let a242 = match v3.clone() {
                        Adt0::C0(h241, _, _, _) => {
                            h241
                        }
                    };
                    let a243 = v1;
                    if nat_eq(a242, a243) {
                        v3
                    } else {
                        f19()
                    }
                }))
            }
        }
    } else {
        Ok((false, f19()))
    }
}

pub fn f47(v0: Bytes) -> R<Bytes> {
    let a246 = {
        let (_, h244) = f35(Fn6::F67(v0.clone()), {
            let a245 = v0;
            length_bytes(a245)
        }, (0u64, Bytes::lit(&[])))?;
        h244
    };
    let a247 = Bytes::lit(&[0x00]);
    Ok(append_bytes(a246, a247))
}

pub fn f48(v0: u64) -> R<Bytes> {
    let (_, h248) = f35(Fn6::F68, 8u64, (v0, Bytes::lit(&[])))?;
    Ok(h248)
}

pub fn f49(v0: Fn10, p252: u64, p253: List<Bytes>) -> R<u64> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m249 = v2;
                match m249.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c250: Fn10 = v0.clone();
                            c250.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: u64, _: Bytes) -> R<u64> {
    let a254 = v0;
    let a255 = 1u64;
    nat_add(a254, a255)
}

pub fn f51(v0: Fn11, p259: (List<Bytes>, List<(Bytes, Bytes)>), p260: List<Bytes>) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m256 = v2;
                match m256.uncons() {
                    None => {
                        Err::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(({
                            let c257: Fn11 = v0.clone();
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

pub fn f52(v0: (List<Bytes>, List<(Bytes, Bytes)>), v1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m262 = {
        let (h261, _) = v0;
        h261
    };
    match m262.uncons() {
        None => {
            (List::<Bytes>::nil(), f69(Bytes::lit(&[]), v1, {
                let (_, h264) = v0;
                h264
            }))
        }
        Some((v2, v3)) => {
            (v3, f69(v2, v1, {
                let (_, h263) = v0;
                h263
            }))
        }
    }
}

pub fn f53(v0: Fn12, p268: List<Bytes>, p269: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m267 = (p268, p269);
    {
        let mut m267 = m267;
        loop {
            let step = {
                let (v1, v2) = m267;
                let m265 = v2;
                match m265.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(({
                            let c266: Fn12 = v0.clone();
                            c266.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: List<Bytes>, v1: (Bytes, Bytes)) -> List<Bytes> {
    List::cons({
        let (_, h270) = v1;
        h270
    }, v0)
}

pub fn f55(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f70(Fn14::F71, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f56(v0: Adt2) -> R<Option<Adt2>> {
    if match v0.clone() {
        Adt2::C0(h271, _, _) => {
            h271
        }
    } {
        let m273 = match v0.clone() {
            Adt2::C0(_, _, h272) => {
                h272
            }
        };
        match m273.uncons() {
            None => {
                Ok(None::<Adt2>)
            }
            Some((v1, v2)) => {
                if match v1.clone() {
                    Adt3::C0(h274, _, _) => {
                        h274
                    }
                } {
                    let v3: (bool, List<Adt3>) = f72(match v1.clone() {
                        Adt3::C0(_, h275, _) => {
                            h275
                        }
                    }, match v1 {
                        Adt3::C0(_, _, h276) => {
                            h276
                        }
                    })?;
                    let (h277, _) = v3;
                    if h277 {
                        Ok(Some(Adt2::C0(true, match v0 {
                            Adt2::C0(_, h278, _) => {
                                h278
                            }
                        }, f73(Fn15::F74, v2, f75({
                            let (_, h279) = v3;
                            h279
                        })))))
                    } else {
                        Ok(Some(Adt2::C0(false, Bytes::lit(&[]), List::<Adt3>::nil())))
                    }
                } else {
                    Ok(Some(Adt2::C0(true, {
                        let a282 = match v0 {
                            Adt2::C0(_, h280, _) => {
                                h280
                            }
                        };
                        let a283 = match v1 {
                            Adt3::C0(_, h281, _) => {
                                h281
                            }
                        };
                        append_bytes(a282, a283)
                    }, v2)))
                }
            }
        }
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f57(v0: u64, v1: List<Bytes>, v2: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f6({
        let (_, h284) = f46(v2, v0)?;
        h284
    }), v1))
}

pub fn f58(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f59(v0: u64) -> Bytes {
    f76(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f60(v0: u64) -> u64 {
    let a285 = 47u64;
    let a286 = v0;
    if nat_lt(a285, a286) {
        let a287 = v0;
        let a288 = 58u64;
        if nat_lt(a287, a288) {
            let a289 = v0;
            let a290 = 48u64;
            nat_sub(a289, a290)
        } else {
            let a291 = 64u64;
            let a292 = v0;
            if nat_lt(a291, a292) {
                let a293 = v0;
                let a294 = 71u64;
                if nat_lt(a293, a294) {
                    let a295 = v0;
                    let a296 = 55u64;
                    nat_sub(a295, a296)
                } else {
                    let a297 = 96u64;
                    let a298 = v0;
                    if nat_lt(a297, a298) {
                        let a299 = v0;
                        let a300 = 103u64;
                        if nat_lt(a299, a300) {
                            let a301 = v0;
                            let a302 = 87u64;
                            nat_sub(a301, a302)
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

pub fn f61(v0: Bytes, v1: u64) -> R<u64> {
    let m305 = {
        let a303 = v0;
        let a304 = v1;
        index_bytes(a303, a304)
    };
    match m305 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a350 = {
                let a326 = {
                    let a314 = {
                        let a308 = {
                            let a306 = v3;
                            let a307 = 128u8;
                            fixed_u8::bit_and(a306, a307)
                        };
                        let a309 = 128u8;
                        if equal(a308, a309) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a315 = {
                        let a312 = {
                            let a310 = v3;
                            let a311 = 64u8;
                            fixed_u8::bit_and(a310, a311)
                        };
                        let a313 = 64u8;
                        if equal(a312, a313) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a314, a315)?
                };
                let a327 = {
                    let a324 = {
                        let a318 = {
                            let a316 = v3;
                            let a317 = 32u8;
                            fixed_u8::bit_and(a316, a317)
                        };
                        let a319 = 32u8;
                        if equal(a318, a319) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a325 = {
                        let a322 = {
                            let a320 = v3;
                            let a321 = 16u8;
                            fixed_u8::bit_and(a320, a321)
                        };
                        let a323 = 16u8;
                        if equal(a322, a323) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a324, a325)?
                };
                nat_add(a326, a327)?
            };
            let a351 = {
                let a348 = {
                    let a336 = {
                        let a330 = {
                            let a328 = v3;
                            let a329 = 8u8;
                            fixed_u8::bit_and(a328, a329)
                        };
                        let a331 = 8u8;
                        if equal(a330, a331) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a337 = {
                        let a334 = {
                            let a332 = v3;
                            let a333 = 4u8;
                            fixed_u8::bit_and(a332, a333)
                        };
                        let a335 = 4u8;
                        if equal(a334, a335) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a336, a337)?
                };
                let a349 = {
                    let a346 = {
                        let a340 = {
                            let a338 = v3;
                            let a339 = 2u8;
                            fixed_u8::bit_and(a338, a339)
                        };
                        let a341 = 2u8;
                        if equal(a340, a341) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a347 = {
                        let a344 = {
                            let a342 = v3;
                            let a343 = 1u8;
                            fixed_u8::bit_and(a342, a343)
                        };
                        let a345 = 1u8;
                        if equal(a344, a345) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a346, a347)?
                };
                nat_add(a348, a349)?
            };
            nat_add(a350, a351)
        }
    }
}

pub fn f62(v0: u32) -> Bytes {
    let a380 = f77({
        let a352 = f78(v0, 28u32);
        let a353 = 15u32;
        fixed_u32::bit_and(a352, a353)
    });
    let a381 = {
        let a378 = f77({
            let a354 = f78(v0, 24u32);
            let a355 = 15u32;
            fixed_u32::bit_and(a354, a355)
        });
        let a379 = {
            let a376 = f77({
                let a356 = f78(v0, 20u32);
                let a357 = 15u32;
                fixed_u32::bit_and(a356, a357)
            });
            let a377 = {
                let a374 = f77({
                    let a358 = f78(v0, 16u32);
                    let a359 = 15u32;
                    fixed_u32::bit_and(a358, a359)
                });
                let a375 = {
                    let a372 = f77({
                        let a360 = f78(v0, 12u32);
                        let a361 = 15u32;
                        fixed_u32::bit_and(a360, a361)
                    });
                    let a373 = {
                        let a370 = f77({
                            let a362 = f78(v0, 8u32);
                            let a363 = 15u32;
                            fixed_u32::bit_and(a362, a363)
                        });
                        let a371 = {
                            let a368 = f77({
                                let a364 = f78(v0, 4u32);
                                let a365 = 15u32;
                                fixed_u32::bit_and(a364, a365)
                            });
                            let a369 = f77({
                                let a366 = v0;
                                let a367 = 15u32;
                                fixed_u32::bit_and(a366, a367)
                            });
                            append_bytes(a368, a369)
                        };
                        append_bytes(a370, a371)
                    };
                    append_bytes(a372, a373)
                };
                append_bytes(a374, a375)
            };
            append_bytes(a376, a377)
        };
        append_bytes(a378, a379)
    };
    append_bytes(a380, a381)
}

pub fn f63(v0: Fn13, p385: u64, p386: (u64, Adt4)) -> R<(u64, Adt4)> {
    let m384 = (p385, p386);
    {
        let mut m384 = m384;
        loop {
            let step = {
                let (v1, v2) = m384;
                let m382 = v1;
                if m382 == 0 {
                    Err::<(u64, (u64, Adt4)), R<(u64, Adt4)>>(Ok(v2))
                } else {
                    let v3 = m382 - 1;
                    Ok::<(u64, (u64, Adt4)), R<(u64, Adt4)>>((v3, {
                        let c383: Fn13 = v0.clone();
                        c383.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt4)) -> R<(u64, Adt4)> {
    Ok(({
        let a388 = {
            let (h387, _) = v3.clone();
            h387
        };
        let a389 = 64u64;
        nat_add(a388, a389)?
    }, f79({
        let (_, h390) = v3;
        h390
    }, f80(v0, v1, v2, {
        let (h391, _) = v3;
        h391
    })?)?))
}

pub fn f65() -> Adt4 {
    Adt4::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f66(v0: Bytes, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    if if match v2.clone() {
        Adt1::C0(_, _, h392, _) => {
            h392
        }
    } {
        true
    } else {
        let a395 = {
            let a393 = v0.clone();
            length_bytes(a393)
        };
        let a396 = match v2.clone() {
            Adt1::C0(h394, _, _, _) => {
                h394
            }
        };
        nat_le(a395, a396)
    } {
        Ok(None::<Adt1>)
    } else {
        let v3: (bool, (u64, u64)) = f81(v0.clone(), match v2.clone() {
            Adt1::C0(h397, _, _, _) => {
                h397
            }
        })?;
        let v4: u64 = {
            let (h398, _) = {
                let (_, h399) = v3;
                h399
            };
            h398
        };
        let v5: u64 = {
            let a400 = v4;
            let a401 = 8u64;
            let a402 = 0u64;
            nat_quot(a400, a401, a402)
        };
        let (h403, _) = v3;
        if if h403 {
            let a404 = 0u64;
            let a405 = v5;
            nat_lt(a404, a405)
        } else {
            false
        } {
            f82(v0, v1, v2, v5, {
                let a406 = v4;
                let a407 = 8u64;
                let a408 = 0u64;
                nat_rem(a406, a407, a408)
            }, {
                let (_, h409) = {
                    let (_, h410) = v3;
                    h410
                };
                h409
            })
        } else {
            Ok(f83(v2))
        }
    }
}

pub fn f67(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a412 = {
            let (h411, _) = v1.clone();
            h411
        };
        let a413 = 1u64;
        nat_add(a412, a413)?
    }, {
        let a418 = {
            let (_, h414) = v1;
            h414
        };
        let a419 = {
            let a416 = Bytes::lit(&[0x01]);
            let a417 = f59(f61(v0, {
                let (h415, _) = v1;
                h415
            })?);
            append_bytes(a416, a417)
        };
        append_bytes(a418, a419)
    }))
}

pub fn f68(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a421 = {
            let (h420, _) = v0.clone();
            h420
        };
        let a422 = 256u64;
        let a423 = 0u64;
        nat_quot(a421, a422, a423)
    }, {
        let a429 = f59({
            let a425 = {
                let (h424, _) = v0;
                h424
            };
            let a426 = 256u64;
            let a427 = 0u64;
            nat_rem(a425, a426, a427)
        });
        let a430 = {
            let (_, h428) = v0;
            h428
        };
        append_bytes(a429, a430)
    })
}

pub fn f69(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt5 = f84(Fn16::F85(v1.clone(), v0.clone()), Adt5::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt5::C0(h431, _) => {
            h431
        }
    } {
        f55(match v3 {
            Adt5::C0(_, h432) => {
                h432
            }
        })
    } else {
        f55(List::cons((v0, v1), match v3 {
            Adt5::C0(_, h433) => {
                h433
            }
        }))
    }
}

pub fn f70(v0: Fn14, p437: List<(Bytes, Bytes)>, p438: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m436 = (p437, p438);
    {
        let mut m436 = m436;
        loop {
            let step = {
                let (v1, v2) = m436;
                let m434 = v2;
                match m434.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c435: Fn14 = v0.clone();
                            c435.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f72(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, List<Adt0>) = f3(v0)?;
    let v3: List<Adt0> = {
        let (_, h439) = v2;
        h439
    };
    let v4: List<Bytes> = f20(v3.clone(), 1u64);
    let v5: (u64, (bool, List<Adt6>)) = f86(Fn17::F87, (0u64, (true, List::<Adt6>::nil())), v4)?;
    let v6: List<Adt6> = f88({
        let (_, h440) = {
            let (_, h441) = v5.clone();
            h441
        };
        h440
    });
    let v7: Adt7 = f89(Fn18::F90(v6), {
        let (h442, _) = v5.clone();
        h442
    }, Adt7::C0(true, List::<u64>::nil(), List::<Adt6>::nil()));
    let v8: (bool, Bytes) = f91(v3.clone())?;
    let v9: (bool, Bytes) = f92(v3.clone(), 11u64)?;
    let v10: (bool, Bytes) = f92(v3.clone(), 12u64)?;
    let v11: (bool, Bytes) = f92(v3.clone(), 13u64)?;
    let v12: Adt3 = f93({
        let a451 = {
            let (_, h443) = v8;
            h443
        };
        let a452 = {
            let a449 = {
                let (_, h444) = v9;
                h444
            };
            let a450 = {
                let a447 = {
                    let (_, h445) = v10;
                    h445
                };
                let a448 = {
                    let (_, h446) = v11;
                    h446
                };
                append_bytes(a447, a448)
            };
            append_bytes(a449, a450)
        };
        append_bytes(a451, a452)
    });
    let v13: (bool, List<Adt3>) = f94(Fn19::F95(v1), (true, List::cons(f93({
        let a454 = f11(f6(f4(v3, 2u64)))?;
        let a455 = f10({
            let (h453, _) = v5;
            h453
        }, 4u64)?;
        append_bytes(a454, a455)
    }), List::<Adt3>::nil())), f88(match v7.clone() {
        Adt7::C0(_, _, h456) => {
            h456
        }
    }))?;
    Ok(({
        let a457 = v1;
        let a458 = 64u64;
        if if nat_le(a457, a458) {
            let (h459, _) = v2;
            h459
        } else {
            false
        } {
            let (h460, _) = {
                let (_, h461) = v5;
                h461
            };
            if if h460 {
                match v7 {
                    Adt7::C0(h462, _, _) => {
                        h462
                    }
                }
            } else {
                false
            } {
                let (h463, _) = v13;
                if h463 {
                    let (h464, _) = v8;
                    if h464 {
                        let (h465, _) = v9;
                        if h465 {
                            let (h466, _) = v10;
                            if h466 {
                                let (h467, _) = v11;
                                h467
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }, f75(List::cons(v12, {
        let (_, h468) = v13;
        h468
    }))))
}

pub fn f73(v0: Fn15, p472: List<Adt3>, p473: List<Adt3>) -> List<Adt3> {
    let m471 = (p472, p473);
    {
        let mut m471 = m471;
        loop {
            let step = {
                let (v1, v2) = m471;
                let m469 = v2;
                match m469.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c470: Fn15 = v0.clone();
                            c470.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f75(v0: List<Adt3>) -> List<Adt3> {
    f73(Fn15::F96, List::<Adt3>::nil(), v0)
}

pub fn f76(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m477 = {
        let a474 = v0;
        let a475 = v1;
        let a476 = v2;
        slice_bytes(a474, a475, a476)
    };
    match m477 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f77(v0: u32) -> Bytes {
    let a480 = {
        let a478 = v0;
        let a479 = 8u32;
        fixed_u32::bit_and(a478, a479)
    };
    let a481 = 0u32;
    if equal(a480, a481) {
        let a484 = {
            let a482 = v0;
            let a483 = 4u32;
            fixed_u32::bit_and(a482, a483)
        };
        let a485 = 0u32;
        if equal(a484, a485) {
            let a488 = {
                let a486 = v0;
                let a487 = 2u32;
                fixed_u32::bit_and(a486, a487)
            };
            let a489 = 0u32;
            if equal(a488, a489) {
                let a492 = {
                    let a490 = v0;
                    let a491 = 1u32;
                    fixed_u32::bit_and(a490, a491)
                };
                let a493 = 0u32;
                if equal(a492, a493) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a496 = {
                    let a494 = v0;
                    let a495 = 1u32;
                    fixed_u32::bit_and(a494, a495)
                };
                let a497 = 0u32;
                if equal(a496, a497) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a500 = {
                let a498 = v0;
                let a499 = 2u32;
                fixed_u32::bit_and(a498, a499)
            };
            let a501 = 0u32;
            if equal(a500, a501) {
                let a504 = {
                    let a502 = v0;
                    let a503 = 1u32;
                    fixed_u32::bit_and(a502, a503)
                };
                let a505 = 0u32;
                if equal(a504, a505) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a508 = {
                    let a506 = v0;
                    let a507 = 1u32;
                    fixed_u32::bit_and(a506, a507)
                };
                let a509 = 0u32;
                if equal(a508, a509) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a512 = {
            let a510 = v0;
            let a511 = 4u32;
            fixed_u32::bit_and(a510, a511)
        };
        let a513 = 0u32;
        if equal(a512, a513) {
            let a516 = {
                let a514 = v0;
                let a515 = 2u32;
                fixed_u32::bit_and(a514, a515)
            };
            let a517 = 0u32;
            if equal(a516, a517) {
                let a520 = {
                    let a518 = v0;
                    let a519 = 1u32;
                    fixed_u32::bit_and(a518, a519)
                };
                let a521 = 0u32;
                if equal(a520, a521) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a524 = {
                    let a522 = v0;
                    let a523 = 1u32;
                    fixed_u32::bit_and(a522, a523)
                };
                let a525 = 0u32;
                if equal(a524, a525) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a528 = {
                let a526 = v0;
                let a527 = 2u32;
                fixed_u32::bit_and(a526, a527)
            };
            let a529 = 0u32;
            if equal(a528, a529) {
                let a532 = {
                    let a530 = v0;
                    let a531 = 1u32;
                    fixed_u32::bit_and(a530, a531)
                };
                let a533 = 0u32;
                if equal(a532, a533) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a536 = {
                    let a534 = v0;
                    let a535 = 1u32;
                    fixed_u32::bit_and(a534, a535)
                };
                let a537 = 0u32;
                if equal(a536, a537) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f78(v0: u32, v1: u32) -> u32 {
    let m540 = {
        let a538 = v0;
        let a539 = v1;
        fixed_u32::shift_right(a538, a539)
    };
    match m540 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f79(v0: Adt4, v1: Adt8) -> R<Adt4> {
    let v2: Adt9 = f97(Fn20::F98, 64u64, Adt9::C0(match v0.clone() {
        Adt4::C0(h541, _, _, _, _, _, _, _) => {
            h541
        }
    }, match v0.clone() {
        Adt4::C0(_, h542, _, _, _, _, _, _) => {
            h542
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h543, _, _, _, _, _) => {
            h543
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h544, _, _, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h545, _, _, _) => {
            h545
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h546, _, _) => {
            h546
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h547, _) => {
            h547
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h548) => {
            h548
        }
    }, v1, 0u64))?;
    Ok(Adt4::C0(f99(match v0.clone() {
        Adt4::C0(h549, _, _, _, _, _, _, _) => {
            h549
        }
    }, match v2.clone() {
        Adt9::C0(h550, _, _, _, _, _, _, _, _, _) => {
            h550
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, h551, _, _, _, _, _, _) => {
            h551
        }
    }, match v2.clone() {
        Adt9::C0(_, h552, _, _, _, _, _, _, _, _) => {
            h552
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, _, h553, _, _, _, _, _) => {
            h553
        }
    }, match v2.clone() {
        Adt9::C0(_, _, h554, _, _, _, _, _, _, _) => {
            h554
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, _, _, h555, _, _, _, _) => {
            h555
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, h556, _, _, _, _, _, _) => {
            h556
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, _, _, _, h557, _, _, _) => {
            h557
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, h558, _, _, _, _, _) => {
            h558
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, _, _, _, _, h559, _, _) => {
            h559
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, _, h560, _, _, _, _) => {
            h560
        }
    }), f99(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h561, _) => {
            h561
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, _, _, h562, _, _, _) => {
            h562
        }
    }), f99(match v0 {
        Adt4::C0(_, _, _, _, _, _, _, h563) => {
            h563
        }
    }, match v2 {
        Adt9::C0(_, _, _, _, _, _, _, h564, _, _) => {
            h564
        }
    })))
}

pub fn f80(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt8> {
    Ok(Adt8::C0(f100(v0.clone(), v1, v2, v3)?, f100(v0.clone(), v1, v2, {
        let a565 = v3;
        let a566 = 4u64;
        nat_add(a565, a566)?
    })?, f100(v0.clone(), v1, v2, {
        let a567 = v3;
        let a568 = 8u64;
        nat_add(a567, a568)?
    })?, f100(v0.clone(), v1, v2, {
        let a569 = v3;
        let a570 = 12u64;
        nat_add(a569, a570)?
    })?, f100(v0.clone(), v1, v2, {
        let a571 = v3;
        let a572 = 16u64;
        nat_add(a571, a572)?
    })?, f100(v0.clone(), v1, v2, {
        let a573 = v3;
        let a574 = 20u64;
        nat_add(a573, a574)?
    })?, f100(v0.clone(), v1, v2, {
        let a575 = v3;
        let a576 = 24u64;
        nat_add(a575, a576)?
    })?, f100(v0.clone(), v1, v2, {
        let a577 = v3;
        let a578 = 28u64;
        nat_add(a577, a578)?
    })?, f100(v0.clone(), v1, v2, {
        let a579 = v3;
        let a580 = 32u64;
        nat_add(a579, a580)?
    })?, f100(v0.clone(), v1, v2, {
        let a581 = v3;
        let a582 = 36u64;
        nat_add(a581, a582)?
    })?, f100(v0.clone(), v1, v2, {
        let a583 = v3;
        let a584 = 40u64;
        nat_add(a583, a584)?
    })?, f100(v0.clone(), v1, v2, {
        let a585 = v3;
        let a586 = 44u64;
        nat_add(a585, a586)?
    })?, f100(v0.clone(), v1, v2, {
        let a587 = v3;
        let a588 = 48u64;
        nat_add(a587, a588)?
    })?, f100(v0.clone(), v1, v2, {
        let a589 = v3;
        let a590 = 52u64;
        nat_add(a589, a590)?
    })?, f100(v0.clone(), v1, v2, {
        let a591 = v3;
        let a592 = 56u64;
        nat_add(a591, a592)?
    })?, f100(v0, v1, v2, {
        let a593 = v3;
        let a594 = 60u64;
        nat_add(a593, a594)?
    })?))
}

pub fn f81(v0: Bytes, v1: u64) -> R<(bool, (u64, u64))> {
    let v2: Adt10 = {
        let (h595, _) = f101(Fn21::F102(v0), 12u64, Adt10::C0(v1, 0u64, 0u64, 1u64, true, false))?;
        h595
    };
    Ok((if match v2.clone() {
        Adt10::C0(_, _, _, _, h596, _) => {
            h596
        }
    } {
        match v2.clone() {
            Adt10::C0(_, _, _, _, _, h597) => {
                h597
            }
        }
    } else {
        false
    }, (match v2.clone() {
        Adt10::C0(_, h598, _, _, _, _) => {
            h598
        }
    }, match v2 {
        Adt10::C0(h599, _, _, _, _, _) => {
            h599
        }
    })))
}

pub fn f82(v0: Bytes, v1: u64, v2: Adt1, v3: u64, v4: u64, v5: u64) -> R<Option<Adt1>> {
    let a600 = v4;
    let a601 = 0u64;
    if nat_eq(a600, a601) {
        let v6: (bool, (u64, u64)) = f81(v0, v5)?;
        let (h602, _) = v6;
        if h602 {
            Ok(f103(v2, v1, v3, 0u64, {
                let (h603, _) = {
                    let (_, h604) = v6;
                    h604
                };
                h603
            }, Bytes::lit(&[]), {
                let (_, h605) = {
                    let (_, h606) = v6;
                    h606
                };
                h605
            }))
        } else {
            Ok(f83(v2))
        }
    } else {
        let a607 = v4;
        let a608 = 1u64;
        if nat_eq(a607, a608) {
            if f104(v0.clone(), v5, 8u64) {
                Ok(f103(v2, v1, v3, 1u64, f105(v0, v5, 8u64)?, Bytes::lit(&[]), {
                    let a609 = v5;
                    let a610 = 8u64;
                    nat_add(a609, a610)?
                }))
            } else {
                Ok(f83(v2))
            }
        } else {
            let a611 = v4;
            let a612 = 5u64;
            if nat_eq(a611, a612) {
                if f104(v0.clone(), v5, 4u64) {
                    Ok(f103(v2, v1, v3, 5u64, f105(v0, v5, 4u64)?, Bytes::lit(&[]), {
                        let a613 = v5;
                        let a614 = 4u64;
                        nat_add(a613, a614)?
                    }))
                } else {
                    Ok(f83(v2))
                }
            } else {
                let a615 = v4;
                let a616 = 2u64;
                if nat_eq(a615, a616) {
                    let v7: (bool, (u64, u64)) = f81(v0.clone(), v5)?;
                    let v8: u64 = {
                        let (h617, _) = {
                            let (_, h618) = v7;
                            h618
                        };
                        h617
                    };
                    let v9: u64 = {
                        let (_, h619) = {
                            let (_, h620) = v7;
                            h620
                        };
                        h619
                    };
                    let (h621, _) = v7;
                    if if h621 {
                        f104(v0.clone(), v9, v8)
                    } else {
                        false
                    } {
                        Ok(f103(v2, v1, v3, 2u64, 0u64, f76(v0, v9, v8), {
                            let a622 = v9;
                            let a623 = v8;
                            nat_add(a622, a623)?
                        }))
                    } else {
                        Ok(f83(v2))
                    }
                } else {
                    Ok(f83(v2))
                }
            }
        }
    }
}

pub fn f83(v0: Adt1) -> Option<Adt1> {
    Some(Adt1::C0(match v0 {
        Adt1::C0(h624, _, _, _) => {
            h624
        }
    }, false, true, List::<Adt0>::nil()))
}

pub fn f84(v0: Fn16, p628: Adt5, p629: List<(Bytes, Bytes)>) -> Adt5 {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, v2) = m627;
                let m625 = v2;
                match m625.uncons() {
                    None => {
                        Err::<(Adt5, List<(Bytes, Bytes)>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<(Bytes, Bytes)>), Adt5>(({
                            let c626: Fn16 = v0.clone();
                            c626.apply(v1, v3)
                        }, v4))
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

pub fn f85(v0: Bytes, v1: Bytes, v2: Adt5, v3: (Bytes, Bytes)) -> Adt5 {
    if match v2.clone() {
        Adt5::C0(h630, _) => {
            h630
        }
    } {
        Adt5::C0(true, List::cons(v3, match v2 {
            Adt5::C0(_, h631) => {
                h631
            }
        }))
    } else {
        if f106(v1.clone(), {
            let (h632, _) = v3.clone();
            h632
        }) {
            Adt5::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt5::C0(_, h633) => {
                    h633
                }
            })))
        } else {
            Adt5::C0(false, List::cons(v3, match v2 {
                Adt5::C0(_, h634) => {
                    h634
                }
            }))
        }
    }
}

pub fn f86(v0: Fn17, p638: (u64, (bool, List<Adt6>)), p639: List<Bytes>) -> R<(u64, (bool, List<Adt6>))> {
    let m637 = (p638, p639);
    {
        let mut m637 = m637;
        loop {
            let step = {
                let (v1, v2) = m637;
                let m635 = v2;
                match m635.uncons() {
                    None => {
                        Err::<((u64, (bool, List<Adt6>)), List<Bytes>), R<(u64, (bool, List<Adt6>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (bool, List<Adt6>)), List<Bytes>), R<(u64, (bool, List<Adt6>))>>(({
                            let c636: Fn17 = v0.clone();
                            c636.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: (u64, (bool, List<Adt6>)), v1: Bytes) -> R<(u64, (bool, List<Adt6>))> {
    let v2: (bool, Adt6) = f107({
        let (h640, _) = v0.clone();
        h640
    }, v1)?;
    Ok(({
        let a642 = {
            let (h641, _) = v0.clone();
            h641
        };
        let a643 = 1u64;
        nat_add(a642, a643)?
    }, ({
        let (h644, _) = {
            let (_, h645) = v0.clone();
            h645
        };
        if h644 {
            let (h646, _) = v2;
            h646
        } else {
            false
        }
    }, List::cons({
        let (_, h647) = v2;
        h647
    }, {
        let (_, h648) = {
            let (_, h649) = v0;
            h649
        };
        h648
    }))))
}

pub fn f88(v0: List<Adt6>) -> List<Adt6> {
    f108(Fn22::F109, List::<Adt6>::nil(), v0)
}

pub fn f89(v0: Fn18, p653: u64, p654: Adt7) -> Adt7 {
    let m652 = (p653, p654);
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v1, v2) = m652;
                let m650 = v1;
                if m650 == 0 {
                    Err::<(u64, Adt7), Adt7>(v2)
                } else {
                    let v3 = m650 - 1;
                    Ok::<(u64, Adt7), Adt7>((v3, {
                        let c651: Fn18 = v0.clone();
                        c651.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: List<Adt6>, v1: Adt7) -> Adt7 {
    f110(v0, v1)
}

pub fn f91(v0: List<Adt0>) -> R<(bool, Bytes)> {
    let v1: List<Bytes> = f20(v0, 5u64);
    let v2: (bool, Bytes) = f111(f29(v1.clone(), f34(v1.clone(), 8u64)?), 0u64)?;
    Ok(({
        let (h655, _) = v2;
        h655
    }, {
        let a657 = f10(f26(v1)?, 4u64)?;
        let a658 = {
            let (_, h656) = v2;
            h656
        };
        append_bytes(a657, a658)
    }))
}

pub fn f92(v0: List<Adt0>, v1: u64) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f20(v0, v1);
    f32(Fn5::F112, (true, f10(f26(v2.clone())?, 4u64)?), f29(v2.clone(), f34(v2, 1u64)?))
}

pub fn f93(v0: Bytes) -> Adt3 {
    Adt3::C0(false, v0, 0u64)
}

pub fn f94(v0: Fn19, p662: (bool, List<Adt3>), p663: List<Adt6>) -> R<(bool, List<Adt3>)> {
    let m661 = (p662, p663);
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v1, v2) = m661;
                let m659 = v2;
                match m659.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Adt6>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Adt6>), R<(bool, List<Adt3>)>>(({
                            let c660: Fn19 = v0.clone();
                            c660.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: u64, v1: (bool, List<Adt3>), v2: Adt6) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f113(v2, v0)?;
    Ok(({
        let (h664, _) = v1;
        if h664 {
            let (h665, _) = v3;
            h665
        } else {
            false
        }
    }, f73(Fn15::F114, {
        let (_, h666) = v1;
        h666
    }, f75({
        let (_, h667) = v3;
        h667
    }))))
}

pub fn f96(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f97(v0: Fn20, p671: u64, p672: Adt9) -> R<Adt9> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m668 = v1;
                if m668 == 0 {
                    Err::<(u64, Adt9), R<Adt9>>(Ok(v2))
                } else {
                    let v3 = m668 - 1;
                    Ok::<(u64, Adt9), R<Adt9>>((v3, {
                        let c669: Fn20 = v0.clone();
                        c669.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: Adt9) -> R<Adt9> {
    f115(v0)
}

pub fn f99(v0: u32, v1: u32) -> u32 {
    let m675 = {
        let a673 = f116(v0);
        let a674 = f116(v1);
        fixed_u64::checked_add(a673, a674)
    };
    match m675 {
        None => {
            0u32
        }
        Some(v2) => {
            f117(v2)
        }
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a678 = {
        let a676 = v3;
        let a677 = 8u64;
        nat_add(a676, a677)?
    };
    let a679 = v2;
    if nat_eq(a678, a679) {
        Ok(f118({
            let a682 = {
                let a680 = v1;
                let a681 = 8u64;
                nat_mul(a680, a681)?
            };
            let a683 = 4294967296u64;
            let a684 = 0u64;
            nat_quot(a682, a683, a684)
        }))
    } else {
        let a687 = {
            let a685 = v3;
            let a686 = 4u64;
            nat_add(a685, a686)?
        };
        let a688 = v2;
        if nat_eq(a687, a688) {
            Ok(f118({
                let a691 = {
                    let a689 = v1;
                    let a690 = 8u64;
                    nat_mul(a689, a690)?
                };
                let a692 = 4294967296u64;
                let a693 = 0u64;
                nat_rem(a691, a692, a693)
            }))
        } else {
            Ok(f119(f120(v0.clone(), v1, v3), f120(v0.clone(), v1, {
                let a694 = v3;
                let a695 = 1u64;
                nat_add(a694, a695)?
            }), f120(v0.clone(), v1, {
                let a696 = v3;
                let a697 = 2u64;
                nat_add(a696, a697)?
            }), f120(v0, v1, {
                let a698 = v3;
                let a699 = 3u64;
                nat_add(a698, a699)?
            })))
        }
    }
}

pub fn f101(v0: Fn21, p704: u64, p705: Adt10) -> R<(Adt10, bool)> {
    let m703 = (p704, p705);
    {
        let mut m703 = m703;
        loop {
            let step = {
                let (v1, v2) = m703;
                let m700 = v1;
                if m700 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m700 - 1;
                    let m702 = {
                        let c701: Fn21 = v0.clone();
                        c701.apply(v2.clone())?
                    };
                    match m702 {
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
                Ok(next) => { m703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    f121(v0, v1)
}

pub fn f103(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64, v5: Bytes, v6: u64) -> Option<Adt1> {
    Some(Adt1::C0(v6, true, {
        let a706 = v2;
        let a707 = v1;
        nat_eq(a706, a707)
    }, List::cons(Adt0::C0(v2, v3, v4, v5), match v0 {
        Adt1::C0(_, _, _, h708) => {
            h708
        }
    })))
}

pub fn f104(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a709 = v0;
        length_bytes(a709)
    };
    let a710 = v1;
    let a711 = v3;
    if nat_le(a710, a711) {
        let a714 = v2;
        let a715 = {
            let a712 = v3;
            let a713 = v1;
            nat_sub(a712, a713)
        };
        nat_le(a714, a715)
    } else {
        false
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h716) = f122(Fn23::F123(v1, v0), v2, (v2, 0u64))?;
    Ok(h716)
}

pub fn f106(v0: Bytes, v1: Bytes) -> bool {
    let a721 = {
        let a717 = v0;
        let a718 = v1;
        compare_bytes(a717, a718)
    };
    let a722 = {
        let a719 = Bytes::lit(&[]);
        let a720 = Bytes::lit(&[0x01]);
        compare_bytes(a719, a720)
    };
    equal(a721, a722)
}

pub fn f107(v0: u64, v1: Bytes) -> R<(bool, Adt6)> {
    let v2: (bool, List<Adt0>) = f3(v1.clone())?;
    let v3: List<Adt0> = {
        let (_, h723) = v2;
        h723
    };
    Ok(({
        let (h724, _) = v2;
        h724
    }, Adt6::C0(v0, v1, {
        let a727 = f47(f6(f4(v3.clone(), 3u64)))?;
        let a728 = {
            let a725 = f47(f6(f4(v3.clone(), 4u64)))?;
            let a726 = f47(f6(f4(v3.clone(), 7u64)))?;
            append_bytes(a725, a726)
        };
        append_bytes(a727, a728)
    }, f20(v3.clone(), 1u64), f20(v3, 2u64))))
}

pub fn f108(v0: Fn22, p732: List<Adt6>, p733: List<Adt6>) -> List<Adt6> {
    let m731 = (p732, p733);
    {
        let mut m731 = m731;
        loop {
            let step = {
                let (v1, v2) = m731;
                let m729 = v2;
                match m729.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c730: Fn22 = v0.clone();
                            c730.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f110(v0: List<Adt6>, v1: Adt7) -> Adt7 {
    if match v1.clone() {
        Adt7::C0(h734, _, _) => {
            h734
        }
    } {
        let v2: Adt11 = f124(Fn24::F125(v0.clone(), v1.clone()), Adt11::C0(false, Adt6::C0(0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), List::<Bytes>::nil())), v0);
        if match v2.clone() {
            Adt11::C0(h735, _) => {
                h735
            }
        } {
            Adt7::C0(true, List::cons({
                let m738 = match v2.clone() {
                    Adt11::C0(_, h737) => {
                        h737
                    }
                };
                match m738 {
                    Adt6::C0(h736, _, _, _, _) => {
                        h736
                    }
                }
            }, match v1.clone() {
                Adt7::C0(_, h739, _) => {
                    h739
                }
            }), List::cons(match v2 {
                Adt11::C0(_, h740) => {
                    h740
                }
            }, match v1 {
                Adt7::C0(_, _, h741) => {
                    h741
                }
            }))
        } else {
            Adt7::C0(false, List::<u64>::nil(), List::<Adt6>::nil())
        }
    } else {
        v1
    }
}

pub fn f111(v0: List<Bytes>, v1: u64) -> R<(bool, Bytes)> {
    f32(Fn5::F126(v1), (true, Bytes::lit(&[])), v0)
}

pub fn f112(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f46(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f46(v1, 2u64)?;
    let v4: (bool, Bytes) = f127(f6({
        let (_, h742) = v3;
        h742
    }))?;
    Ok(({
        let (h743, _) = v0;
        if h743 {
            let (h744, _) = v2;
            if h744 {
                let (h745, _) = v3;
                if h745 {
                    let (h746, _) = v4;
                    h746
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a752 = {
            let (_, h747) = v0;
            h747
        };
        let a753 = {
            let a750 = f11(f6({
                let (_, h748) = v2;
                h748
            }))?;
            let a751 = {
                let (_, h749) = v4;
                h749
            };
            append_bytes(a750, a751)
        };
        append_bytes(a752, a753)
    }))
}

pub fn f113(v0: Adt6, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: List<Adt0> = {
        let (_, h754) = f3(match v0 {
            Adt6::C0(_, h755, _, _, _) => {
                h755
            }
        })?;
        h754
    };
    let v3: List<Bytes> = f20(v2.clone(), 5u64);
    let v4: Bytes = {
        let a770 = f11(f6(f4(v2.clone(), 3u64)))?;
        let a771 = {
            let a768 = f11(f6(f4(v2.clone(), 4u64)))?;
            let a769 = {
                let a766 = f11(f6(f4(v2.clone(), 7u64)))?;
                let a767 = {
                    let a764 = f11(f6(f4(v2.clone(), 8u64)))?;
                    let a765 = {
                        let a762 = f10(f128(v2.clone(), 1u64)?, 4u64)?;
                        let a763 = {
                            let a760 = f129(f20(v2.clone(), 1u64))?;
                            let a761 = {
                                let a758 = f10(f128(v2.clone(), 2u64)?, 4u64)?;
                                let a759 = {
                                    let a756 = f129(f20(v2, 2u64))?;
                                    let a757 = f10(f26(v3.clone())?, 4u64)?;
                                    append_bytes(a756, a757)
                                };
                                append_bytes(a758, a759)
                            };
                            append_bytes(a760, a761)
                        };
                        append_bytes(a762, a763)
                    };
                    append_bytes(a764, a765)
                };
                append_bytes(a766, a767)
            };
            append_bytes(a768, a769)
        };
        append_bytes(a770, a771)
    };
    f130(Fn25::F131(v1), (true, List::cons(f93(v4), List::<Adt3>::nil())), f29(v3.clone(), f34(v3, 1u64)?))
}

pub fn f114(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f115(v0: Adt9) -> R<Adt9> {
    let v1: u32 = f132(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h772) => {
            h772
        }
    });
    let v2: u32 = f99(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h773, _, _) => {
            h773
        }
    }, f99(f133(match v0.clone() {
        Adt9::C0(_, _, _, _, h774, _, _, _, _, _) => {
            h774
        }
    }), f99(f134(match v0.clone() {
        Adt9::C0(_, _, _, _, h775, _, _, _, _, _) => {
            h775
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h776, _, _, _, _) => {
            h776
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h777, _, _, _) => {
            h777
        }
    }), f99(v1, {
        let m780 = match v0.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h779, _) => {
                h779
            }
        };
        match m780 {
            Adt8::C0(h778, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h778
            }
        }
    }))));
    let v3: u32 = f99(f135(match v0.clone() {
        Adt9::C0(h781, _, _, _, _, _, _, _, _, _) => {
            h781
        }
    }), f136(match v0.clone() {
        Adt9::C0(h782, _, _, _, _, _, _, _, _, _) => {
            h782
        }
    }, match v0.clone() {
        Adt9::C0(_, h783, _, _, _, _, _, _, _, _) => {
            h783
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h784, _, _, _, _, _, _, _) => {
            h784
        }
    }));
    Ok(Adt9::C0(f99(v2, v3), match v0.clone() {
        Adt9::C0(h785, _, _, _, _, _, _, _, _, _) => {
            h785
        }
    }, match v0.clone() {
        Adt9::C0(_, h786, _, _, _, _, _, _, _, _) => {
            h786
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h787, _, _, _, _, _, _, _) => {
            h787
        }
    }, f99(match v0.clone() {
        Adt9::C0(_, _, _, h788, _, _, _, _, _, _) => {
            h788
        }
    }, v2), match v0.clone() {
        Adt9::C0(_, _, _, _, h789, _, _, _, _, _) => {
            h789
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h790, _, _, _, _) => {
            h790
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h791, _, _, _) => {
            h791
        }
    }, f137(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h792, _) => {
            h792
        }
    }), {
        let a794 = match v0 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h793) => {
                h793
            }
        };
        let a795 = 1u64;
        nat_add(a794, a795)?
    }))
}

pub fn f116(v0: u32) -> u64 {
    let m797 = {
        let a796 = v0;
        fixed_u64::convert(i128::from(a796))
    };
    match m797 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f117(v0: u64) -> u32 {
    let m801 = {
        let a800 = {
            let a798 = v0;
            let a799 = 4294967295u64;
            fixed_u64::bit_and(a798, a799)
        };
        fixed_u32::convert(i128::from(a800))
    };
    match m801 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f118(v0: u64) -> u32 {
    let (_, h802) = {
        let (_, h803) = f138(Fn26::F139, 32u64, (v0, (1u32, 0u32)));
        h803
    };
    h802
}

pub fn f119(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a808 = f140(v0, 24u32);
    let a809 = {
        let a806 = f140(v1, 16u32);
        let a807 = {
            let a804 = f140(v2, 8u32);
            let a805 = v3;
            fixed_u32::bit_or(a804, a805)
        };
        fixed_u32::bit_or(a806, a807)
    };
    fixed_u32::bit_or(a808, a809)
}

pub fn f120(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a810 = v2;
    let a811 = v1;
    if nat_lt(a810, a811) {
        let m814 = {
            let a812 = v0;
            let a813 = v2;
            index_bytes(a812, a813)
        };
        match m814 {
            None => {
                0u32
            }
            Some(v3) => {
                let m816 = {
                    let a815 = v3;
                    fixed_u32::convert(i128::from(a815))
                };
                match m816 {
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
        let a817 = v2;
        let a818 = v1;
        if nat_eq(a817, a818) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f121(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    if match v1.clone() {
        Adt10::C0(_, _, _, _, _, h819) => {
            h819
        }
    } {
        Ok(None::<Adt10>)
    } else {
        let v2: u64 = match v1.clone() {
            Adt10::C0(h820, _, _, _, _, _) => {
                h820
            }
        };
        let a822 = {
            let a821 = v0.clone();
            length_bytes(a821)
        };
        let a823 = v2;
        if if nat_le(a822, a823) {
            true
        } else {
            let a825 = 70u64;
            let a826 = match v1.clone() {
                Adt10::C0(_, _, h824, _, _, _) => {
                    h824
                }
            };
            nat_le(a825, a826)
        } {
            Ok(Some(Adt10::C0(v2, 0u64, 0u64, 1u64, false, true)))
        } else {
            let v3: u64 = f61(v0, v2)?;
            let v4: u64 = {
                let a827 = v3;
                let a828 = 128u64;
                let a829 = 0u64;
                nat_rem(a827, a828, a829)
            };
            let v5: u64 = {
                let a831 = match v1.clone() {
                    Adt10::C0(_, _, h830, _, _, _) => {
                        h830
                    }
                };
                let a832 = 63u64;
                if nat_eq(a831, a832) {
                    let a837 = {
                        let a833 = v4;
                        let a834 = 2u64;
                        let a835 = 0u64;
                        nat_rem(a833, a834, a835)
                    };
                    let a838 = match v1.clone() {
                        Adt10::C0(_, _, _, h836, _, _) => {
                            h836
                        }
                    };
                    nat_mul(a837, a838)?
                } else {
                    let a840 = v4;
                    let a841 = match v1.clone() {
                        Adt10::C0(_, _, _, h839, _, _) => {
                            h839
                        }
                    };
                    nat_mul(a840, a841)?
                }
            };
            Ok(Some(Adt10::C0({
                let a842 = v2;
                let a843 = 1u64;
                nat_add(a842, a843)?
            }, {
                let a845 = match v1.clone() {
                    Adt10::C0(_, h844, _, _, _, _) => {
                        h844
                    }
                };
                let a846 = v5;
                nat_add(a845, a846)?
            }, {
                let a848 = match v1.clone() {
                    Adt10::C0(_, _, h847, _, _, _) => {
                        h847
                    }
                };
                let a849 = 7u64;
                nat_add(a848, a849)?
            }, {
                let a851 = match v1.clone() {
                    Adt10::C0(_, _, h850, _, _, _) => {
                        h850
                    }
                };
                let a852 = 63u64;
                if nat_lt(a851, a852) {
                    let a854 = match v1 {
                        Adt10::C0(_, _, _, h853, _, _) => {
                            h853
                        }
                    };
                    let a855 = 128u64;
                    nat_mul(a854, a855)?
                } else {
                    match v1 {
                        Adt10::C0(_, _, _, h856, _, _) => {
                            h856
                        }
                    }
                }
            }, true, {
                let a857 = v3;
                let a858 = 128u64;
                nat_lt(a857, a858)
            })))
        }
    }
}

pub fn f122(v0: Fn23, p862: u64, p863: (u64, u64)) -> R<(u64, u64)> {
    let m861 = (p862, p863);
    {
        let mut m861 = m861;
        loop {
            let step = {
                let (v1, v2) = m861;
                let m859 = v1;
                if m859 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m859 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c860: Fn23 = v0.clone();
                        c860.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a865 = {
            let (h864, _) = v2;
            h864
        };
        let a866 = 1u64;
        nat_sub(a865, a866)
    };
    Ok((v3, {
        let a872 = {
            let a868 = {
                let (_, h867) = v2;
                h867
            };
            let a869 = 256u64;
            nat_mul(a868, a869)?
        };
        let a873 = f61(v1, {
            let a870 = v0;
            let a871 = v3;
            nat_add(a870, a871)?
        })?;
        nat_add(a872, a873)?
    }))
}

pub fn f124(v0: Fn24, p877: Adt11, p878: List<Adt6>) -> Adt11 {
    let m876 = (p877, p878);
    {
        let mut m876 = m876;
        loop {
            let step = {
                let (v1, v2) = m876;
                let m874 = v2;
                match m874.uncons() {
                    None => {
                        Err::<(Adt11, List<Adt6>), Adt11>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt11, List<Adt6>), Adt11>(({
                            let c875: Fn24 = v0.clone();
                            c875.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<Adt6>, v1: Adt7, v2: Adt11, v3: Adt6) -> Adt11 {
    if if f141(match v1.clone() {
        Adt7::C0(_, h879, _) => {
            h879
        }
    }, match v3.clone() {
        Adt6::C0(h880, _, _, _, _) => {
            h880
        }
    }) {
        true
    } else {
        !f142(v0, match v1 {
            Adt7::C0(_, h881, _) => {
                h881
            }
        }, v3.clone())
    } {
        v2
    } else {
        if match v2.clone() {
            Adt11::C0(h882, _) => {
                h882
            }
        } {
            if f106({
                let m885 = match v2.clone() {
                    Adt11::C0(_, h884) => {
                        h884
                    }
                };
                match m885 {
                    Adt6::C0(_, _, h883, _, _) => {
                        h883
                    }
                }
            }, match v3.clone() {
                Adt6::C0(_, _, h886, _, _) => {
                    h886
                }
            }) {
                v2
            } else {
                Adt11::C0(true, v3)
            }
        } else {
            Adt11::C0(true, v3)
        }
    }
}

pub fn f126(v0: u64, v1: (bool, Bytes), v2: Bytes) -> R<(bool, Bytes)> {
    let v3: (bool, Bytes) = {
        let a887 = v0;
        let a888 = 0u64;
        if nat_eq(a887, a888) {
            f143(v2)?
        } else {
            f127(v2)?
        }
    };
    Ok(({
        let (h889, _) = v1;
        if h889 {
            let (h890, _) = v3;
            h890
        } else {
            false
        }
    }, {
        let a893 = {
            let (_, h891) = v1;
            h891
        };
        let a894 = {
            let (_, h892) = v3;
            h892
        };
        append_bytes(a893, a894)
    }))
}

pub fn f127(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f3(v0.clone())?;
    let v2: Adt12 = {
        let (h895, _) = f144(Fn27::F145, {
            let a899 = {
                let a897 = {
                    let a896 = v0;
                    length_bytes(a896)
                };
                let a898 = 40u64;
                nat_mul(a897, a898)?
            };
            let a900 = 8u64;
            nat_add(a899, a900)?
        }, Adt12::C0(List::cons(Adt13::C0(f146({
            let (_, h901) = v1;
            h901
        }), Bytes::lit(&[]), 0u64), List::<Adt13>::nil()), Bytes::lit(&[]), false))?;
        h895
    };
    Ok(({
        let (h902, _) = v1;
        if h902 {
            match v2.clone() {
                Adt12::C0(_, _, h903) => {
                    h903
                }
            }
        } else {
            false
        }
    }, match v2 {
        Adt12::C0(_, h904, _) => {
            h904
        }
    }))
}

pub fn f128(v0: List<Adt0>, v1: u64) -> R<u64> {
    f147(Fn28::F148(v1), 0u64, v0)
}

pub fn f129(v0: List<Bytes>) -> R<Bytes> {
    f27(Fn3::F149, Bytes::lit(&[]), v0)
}

pub fn f130(v0: Fn25, p908: (bool, List<Adt3>), p909: List<Bytes>) -> R<(bool, List<Adt3>)> {
    let m907 = (p908, p909);
    {
        let mut m907 = m907;
        loop {
            let step = {
                let (v1, v2) = m907;
                let m905 = v2;
                match m905.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(({
                            let c906: Fn25 = v0.clone();
                            c906.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: u64, v1: (bool, List<Adt3>), v2: Bytes) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f150(v2, v0)?;
    Ok(({
        let (h910, _) = v1;
        if h910 {
            let (h911, _) = v3;
            h911
        } else {
            false
        }
    }, f73(Fn15::F151, {
        let (_, h912) = v1;
        h912
    }, f75({
        let (_, h913) = v3;
        h913
    }))))
}

pub fn f132(v0: u64) -> u32 {
    let a914 = v0;
    let a915 = 32u64;
    if nat_lt(a914, a915) {
        let a916 = v0;
        let a917 = 16u64;
        if nat_lt(a916, a917) {
            let a918 = v0;
            let a919 = 8u64;
            if nat_lt(a918, a919) {
                let a920 = v0;
                let a921 = 4u64;
                if nat_lt(a920, a921) {
                    let a922 = v0;
                    let a923 = 2u64;
                    if nat_lt(a922, a923) {
                        let a924 = v0;
                        let a925 = 1u64;
                        if nat_lt(a924, a925) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a926 = v0;
                        let a927 = 3u64;
                        if nat_lt(a926, a927) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a928 = v0;
                    let a929 = 6u64;
                    if nat_lt(a928, a929) {
                        let a930 = v0;
                        let a931 = 5u64;
                        if nat_lt(a930, a931) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a932 = v0;
                        let a933 = 7u64;
                        if nat_lt(a932, a933) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a934 = v0;
                let a935 = 12u64;
                if nat_lt(a934, a935) {
                    let a936 = v0;
                    let a937 = 10u64;
                    if nat_lt(a936, a937) {
                        let a938 = v0;
                        let a939 = 9u64;
                        if nat_lt(a938, a939) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a940 = v0;
                        let a941 = 11u64;
                        if nat_lt(a940, a941) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a942 = v0;
                    let a943 = 14u64;
                    if nat_lt(a942, a943) {
                        let a944 = v0;
                        let a945 = 13u64;
                        if nat_lt(a944, a945) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a946 = v0;
                        let a947 = 15u64;
                        if nat_lt(a946, a947) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a948 = v0;
            let a949 = 24u64;
            if nat_lt(a948, a949) {
                let a950 = v0;
                let a951 = 20u64;
                if nat_lt(a950, a951) {
                    let a952 = v0;
                    let a953 = 18u64;
                    if nat_lt(a952, a953) {
                        let a954 = v0;
                        let a955 = 17u64;
                        if nat_lt(a954, a955) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a956 = v0;
                        let a957 = 19u64;
                        if nat_lt(a956, a957) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a958 = v0;
                    let a959 = 22u64;
                    if nat_lt(a958, a959) {
                        let a960 = v0;
                        let a961 = 21u64;
                        if nat_lt(a960, a961) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a962 = v0;
                        let a963 = 23u64;
                        if nat_lt(a962, a963) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a964 = v0;
                let a965 = 28u64;
                if nat_lt(a964, a965) {
                    let a966 = v0;
                    let a967 = 26u64;
                    if nat_lt(a966, a967) {
                        let a968 = v0;
                        let a969 = 25u64;
                        if nat_lt(a968, a969) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a970 = v0;
                        let a971 = 27u64;
                        if nat_lt(a970, a971) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a972 = v0;
                    let a973 = 30u64;
                    if nat_lt(a972, a973) {
                        let a974 = v0;
                        let a975 = 29u64;
                        if nat_lt(a974, a975) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a976 = v0;
                        let a977 = 31u64;
                        if nat_lt(a976, a977) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a978 = v0;
        let a979 = 48u64;
        if nat_lt(a978, a979) {
            let a980 = v0;
            let a981 = 40u64;
            if nat_lt(a980, a981) {
                let a982 = v0;
                let a983 = 36u64;
                if nat_lt(a982, a983) {
                    let a984 = v0;
                    let a985 = 34u64;
                    if nat_lt(a984, a985) {
                        let a986 = v0;
                        let a987 = 33u64;
                        if nat_lt(a986, a987) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a988 = v0;
                        let a989 = 35u64;
                        if nat_lt(a988, a989) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a990 = v0;
                    let a991 = 38u64;
                    if nat_lt(a990, a991) {
                        let a992 = v0;
                        let a993 = 37u64;
                        if nat_lt(a992, a993) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a994 = v0;
                        let a995 = 39u64;
                        if nat_lt(a994, a995) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a996 = v0;
                let a997 = 44u64;
                if nat_lt(a996, a997) {
                    let a998 = v0;
                    let a999 = 42u64;
                    if nat_lt(a998, a999) {
                        let a1000 = v0;
                        let a1001 = 41u64;
                        if nat_lt(a1000, a1001) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1002 = v0;
                        let a1003 = 43u64;
                        if nat_lt(a1002, a1003) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1004 = v0;
                    let a1005 = 46u64;
                    if nat_lt(a1004, a1005) {
                        let a1006 = v0;
                        let a1007 = 45u64;
                        if nat_lt(a1006, a1007) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1008 = v0;
                        let a1009 = 47u64;
                        if nat_lt(a1008, a1009) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1010 = v0;
            let a1011 = 56u64;
            if nat_lt(a1010, a1011) {
                let a1012 = v0;
                let a1013 = 52u64;
                if nat_lt(a1012, a1013) {
                    let a1014 = v0;
                    let a1015 = 50u64;
                    if nat_lt(a1014, a1015) {
                        let a1016 = v0;
                        let a1017 = 49u64;
                        if nat_lt(a1016, a1017) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1018 = v0;
                        let a1019 = 51u64;
                        if nat_lt(a1018, a1019) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1020 = v0;
                    let a1021 = 54u64;
                    if nat_lt(a1020, a1021) {
                        let a1022 = v0;
                        let a1023 = 53u64;
                        if nat_lt(a1022, a1023) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1024 = v0;
                        let a1025 = 55u64;
                        if nat_lt(a1024, a1025) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1026 = v0;
                let a1027 = 60u64;
                if nat_lt(a1026, a1027) {
                    let a1028 = v0;
                    let a1029 = 58u64;
                    if nat_lt(a1028, a1029) {
                        let a1030 = v0;
                        let a1031 = 57u64;
                        if nat_lt(a1030, a1031) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1032 = v0;
                        let a1033 = 59u64;
                        if nat_lt(a1032, a1033) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1034 = v0;
                    let a1035 = 62u64;
                    if nat_lt(a1034, a1035) {
                        let a1036 = v0;
                        let a1037 = 61u64;
                        if nat_lt(a1036, a1037) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1038 = v0;
                        let a1039 = 63u64;
                        if nat_lt(a1038, a1039) {
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

pub fn f133(v0: u32) -> u32 {
    let a1042 = {
        let a1040 = f152(v0, 6u32, 26u32);
        let a1041 = f152(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1040, a1041)
    };
    let a1043 = f152(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1042, a1043)
}

pub fn f134(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1049 = {
        let a1044 = v0;
        let a1045 = v1;
        fixed_u32::bit_and(a1044, a1045)
    };
    let a1050 = {
        let a1047 = {
            let a1046 = v0;
            fixed_u32::bit_not(a1046)
        };
        let a1048 = v2;
        fixed_u32::bit_and(a1047, a1048)
    };
    fixed_u32::bit_xor(a1049, a1050)
}

pub fn f135(v0: u32) -> u32 {
    let a1053 = {
        let a1051 = f152(v0, 2u32, 30u32);
        let a1052 = f152(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1051, a1052)
    };
    let a1054 = f152(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1053, a1054)
}

pub fn f136(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1063 = {
        let a1059 = {
            let a1055 = v0;
            let a1056 = v1;
            fixed_u32::bit_and(a1055, a1056)
        };
        let a1060 = {
            let a1057 = v0;
            let a1058 = v2;
            fixed_u32::bit_and(a1057, a1058)
        };
        fixed_u32::bit_xor(a1059, a1060)
    };
    let a1064 = {
        let a1061 = v1;
        let a1062 = v2;
        fixed_u32::bit_and(a1061, a1062)
    };
    fixed_u32::bit_xor(a1063, a1064)
}

pub fn f137(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, h1065, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1065
        }
    }, match v0.clone() {
        Adt8::C0(_, _, h1066, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1066
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1067, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1067
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1068, _, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1069, _, _, _, _, _, _, _, _, _, _) => {
            h1069
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1070, _, _, _, _, _, _, _, _, _) => {
            h1070
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1071, _, _, _, _, _, _, _, _) => {
            h1071
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1072, _, _, _, _, _, _, _) => {
            h1072
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1073, _, _, _, _, _, _) => {
            h1073
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1074, _, _, _, _, _) => {
            h1074
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1075, _, _, _, _) => {
            h1075
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1076, _, _, _) => {
            h1076
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1077, _, _) => {
            h1077
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1078, _) => {
            h1078
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1079) => {
            h1079
        }
    }, f99(f153(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1080, _) => {
            h1080
        }
    }), f99(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1081, _, _, _, _, _, _) => {
            h1081
        }
    }, f99(f154(match v0.clone() {
        Adt8::C0(_, h1082, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1082
        }
    }), match v0 {
        Adt8::C0(h1083, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1083
        }
    }))))
}

pub fn f138(v0: Fn26, p1087: u64, p1088: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1086 = (p1087, p1088);
    {
        let mut m1086 = m1086;
        loop {
            let step = {
                let (v1, v2) = m1086;
                let m1084 = v1;
                if m1084 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1084 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1085: Fn26 = v0.clone();
                        c1085.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1089, _) = v0;
        h1089
    };
    let v2: u32 = {
        let (h1090, _) = {
            let (_, h1091) = v0;
            h1091
        };
        h1090
    };
    let v3: u32 = {
        let (_, h1092) = {
            let (_, h1093) = v0;
            h1093
        };
        h1092
    };
    ({
        let a1094 = v1;
        let a1095 = 2u64;
        let a1096 = 0u64;
        nat_quot(a1094, a1095, a1096)
    }, (f140(v2, 1u32), {
        let a1100 = {
            let a1097 = v1;
            let a1098 = 2u64;
            let a1099 = 0u64;
            nat_rem(a1097, a1098, a1099)
        };
        let a1101 = 1u64;
        if nat_eq(a1100, a1101) {
            let a1102 = v3;
            let a1103 = v2;
            fixed_u32::bit_or(a1102, a1103)
        } else {
            v3
        }
    }))
}

pub fn f140(v0: u32, v1: u32) -> u32 {
    let m1106 = {
        let a1104 = v0;
        let a1105 = v1;
        fixed_u32::shift_left(a1104, a1105)
    };
    match m1106 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f141(v0: List<u64>, v1: u64) -> bool {
    f155(Fn29::F156(v1), false, v0)
}

pub fn f142(v0: List<Adt6>, v1: List<u64>, v2: Adt6) -> bool {
    f157(Fn30::F158(v1, v0), true, match v2 {
        Adt6::C0(_, _, _, h1107, _) => {
            h1107
        }
    })
}

pub fn f143(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f3(v0.clone())?;
    let v2: List<Adt0> = {
        let (_, h1108) = v1;
        h1108
    };
    let v3: u64 = {
        let a1109 = f12(f4(v2.clone(), 2u64));
        let a1110 = 4294967296u64;
        let a1111 = 0u64;
        nat_rem(a1109, a1110, a1111)
    };
    let v4: (bool, List<u64>) = f159(v2.clone(), 1u64)?;
    let v5: (bool, Bytes) = f160(v0, v2.clone(), v3)?;
    Ok(({
        let (h1112, _) = v1;
        if if h1112 {
            let a1113 = 0u64;
            let a1114 = v3;
            if nat_lt(a1113, a1114) {
                let a1115 = v3;
                let a1116 = 24u64;
                nat_lt(a1115, a1116)
            } else {
                false
            }
        } else {
            false
        } {
            let (h1117, _) = v4.clone();
            if h1117 {
                let (h1118, _) = v5;
                h1118
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a1128 = f11(f6(f4(v2, 8u64)))?;
        let a1129 = {
            let a1126 = f10(v3, 4u64)?;
            let a1127 = {
                let a1124 = f10(f161({
                    let (_, h1119) = v4.clone();
                    h1119
                })?, 4u64)?;
                let a1125 = {
                    let a1122 = f162({
                        let (_, h1120) = v4;
                        h1120
                    }, 8u64)?;
                    let a1123 = {
                        let (_, h1121) = v5;
                        h1121
                    };
                    append_bytes(a1122, a1123)
                };
                append_bytes(a1124, a1125)
            };
            append_bytes(a1126, a1127)
        };
        append_bytes(a1128, a1129)
    }))
}

pub fn f144(v0: Fn27, p1134: u64, p1135: Adt12) -> R<(Adt12, bool)> {
    let m1133 = (p1134, p1135);
    {
        let mut m1133 = m1133;
        loop {
            let step = {
                let (v1, v2) = m1133;
                let m1130 = v1;
                if m1130 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1130 - 1;
                    let m1132 = {
                        let c1131: Fn27 = v0.clone();
                        c1131.apply(v2.clone())?
                    };
                    match m1132 {
                        None => {
                            Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt12), R<(Adt12, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Adt12) -> R<Option<Adt12>> {
    f163(v0)
}

pub fn f146(v0: List<Adt0>) -> List<Adt0> {
    f42(Fn8::F164, List::<Adt0>::nil(), v0)
}

pub fn f147(v0: Fn28, p1139: u64, p1140: List<Adt0>) -> R<u64> {
    let m1138 = (p1139, p1140);
    {
        let mut m1138 = m1138;
        loop {
            let step = {
                let (v1, v2) = m1138;
                let m1136 = v2;
                match m1136.uncons() {
                    None => {
                        Err::<(u64, List<Adt0>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt0>), R<u64>>(({
                            let c1137: Fn28 = v0.clone();
                            c1137.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: u64, v1: u64, v2: Adt0) -> R<u64> {
    let a1142 = match v2 {
        Adt0::C0(h1141, _, _, _) => {
            h1141
        }
    };
    let a1143 = v0;
    if nat_eq(a1142, a1143) {
        let a1144 = v1;
        let a1145 = 1u64;
        nat_add(a1144, a1145)
    } else {
        Ok(v1)
    }
}

pub fn f149(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a1146 = v0;
    let a1147 = f11(v1)?;
    Ok(append_bytes(a1146, a1147))
}

pub fn f150(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, Adt0) = f46(v0.clone(), 1u64)?;
    let v3: (bool, Adt0) = f46(v0.clone(), 20u64)?;
    let v4: u64 = {
        let a1149 = f12({
            let (_, h1148) = v3;
            h1148
        });
        let a1150 = 4294967296u64;
        let a1151 = 0u64;
        nat_rem(a1149, a1150, a1151)
    };
    let v5: Adt3 = f93({
        let a1153 = f11(f6({
            let (_, h1152) = v2;
            h1152
        }))?;
        let a1154 = f10(v4, 4u64)?;
        append_bytes(a1153, a1154)
    });
    let v6: bool = {
        let (h1155, _) = v2;
        if h1155 {
            let (h1156, _) = v3;
            h1156
        } else {
            false
        }
    };
    let a1157 = v4;
    let a1158 = 5u64;
    if nat_eq(a1157, a1158) {
        let v7: (bool, Adt0) = f46(v0, 6u64)?;
        Ok((if v6 {
            let (h1159, _) = v7;
            h1159
        } else {
            false
        }, List::cons(Adt3::C0(true, f6({
            let (_, h1160) = v7;
            h1160
        }), {
            let a1161 = v1;
            let a1162 = 1u64;
            nat_add(a1161, a1162)?
        }), List::cons(v5, List::<Adt3>::nil()))))
    } else {
        let a1163 = v4;
        let a1164 = 10u64;
        if nat_eq(a1163, a1164) {
            let v8: (bool, List<Adt0>) = f3(v0)?;
            Ok((if v6 {
                let (h1165, _) = v8;
                h1165
            } else {
                false
            }, f165(Fn31::F166(v1), List::cons(v5, List::<Adt3>::nil()), f20({
                let (_, h1166) = v8;
                h1166
            }, 11u64))?))
        } else {
            let v9: (bool, Bytes) = f167(v0, v4)?;
            Ok((if v6 {
                let (h1167, _) = v9;
                h1167
            } else {
                false
            }, List::cons(f93({
                let (_, h1168) = v9;
                h1168
            }), List::cons(v5, List::<Adt3>::nil()))))
        }
    }
}

pub fn f151(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f152(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1169 = f78(v0, v1);
    let a1170 = f140(v0, v2);
    fixed_u32::bit_or(a1169, a1170)
}

pub fn f153(v0: u32) -> u32 {
    let a1173 = {
        let a1171 = f152(v0, 17u32, 15u32);
        let a1172 = f152(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1171, a1172)
    };
    let a1174 = f78(v0, 10u32);
    fixed_u32::bit_xor(a1173, a1174)
}

pub fn f154(v0: u32) -> u32 {
    let a1177 = {
        let a1175 = f152(v0, 7u32, 25u32);
        let a1176 = f152(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1175, a1176)
    };
    let a1178 = f78(v0, 3u32);
    fixed_u32::bit_xor(a1177, a1178)
}

pub fn f155(v0: Fn29, p1182: bool, p1183: List<u64>) -> bool {
    let m1181 = (p1182, p1183);
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let (v1, v2) = m1181;
                let m1179 = v2;
                match m1179.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1180: Fn29 = v0.clone();
                            c1180.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1184 = v2;
        let a1185 = v0;
        nat_eq(a1184, a1185)
    }
}

pub fn f157(v0: Fn30, p1189: bool, p1190: List<Bytes>) -> bool {
    let m1188 = (p1189, p1190);
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v1, v2) = m1188;
                let m1186 = v2;
                match m1186.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1187: Fn30 = v0.clone();
                            c1187.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: List<u64>, v1: List<Adt6>, v2: bool, v3: Bytes) -> bool {
    let a1192 = 0u64;
    let a1193 = {
        let a1191 = v3.clone();
        length_bytes(a1191)
    };
    if nat_lt(a1192, a1193) {
        f168(Fn32::F169(v0, v3), v2, v1)
    } else {
        v2
    }
}

pub fn f159(v0: List<Adt0>, v1: u64) -> R<(bool, List<u64>)> {
    let v2: (bool, List<u64>) = f170(Fn33::F171(v1), (true, List::<u64>::nil()), v0)?;
    Ok(({
        let (h1194, _) = v2;
        h1194
    }, f172(Fn34::F173, List::<u64>::nil(), {
        let (_, h1195) = v2;
        h1195
    })))
}

pub fn f160(v0: Bytes, v1: List<Adt0>, v2: u64) -> R<(bool, Bytes)> {
    let a1196 = f12(f4(v1.clone(), 14u64));
    let a1197 = 1u64;
    if nat_eq(a1196, a1197) {
        let v3: (bool, Bytes) = f8(v0, 13u64)?;
        Ok(({
            let (h1198, _) = v3;
            h1198
        }, f11({
            let a1200 = f13(Str::lit("onnx:external-data:v1"));
            let a1201 = {
                let (_, h1199) = v3;
                h1199
            };
            append_bytes(a1200, a1201)
        })?))
    } else {
        let v4: Adt0 = f4(v1.clone(), 9u64);
        let a1203 = match v4.clone() {
            Adt0::C0(_, h1202, _, _) => {
                h1202
            }
        };
        let a1204 = 2u64;
        if if nat_eq(a1203, a1204) {
            let a1207 = 0u64;
            let a1208 = {
                let a1206 = match v4.clone() {
                    Adt0::C0(_, _, _, h1205) => {
                        h1205
                    }
                };
                length_bytes(a1206)
            };
            nat_lt(a1207, a1208)
        } else {
            false
        } {
            Ok((true, f11(match v4 {
                Adt0::C0(_, _, _, h1209) => {
                    h1209
                }
            })?))
        } else {
            let v5: (u64, u64) = f174(v2);
            let a1211 = {
                let (_, h1210) = v5;
                h1210
            };
            let a1212 = 0u64;
            if nat_eq(a1211, a1212) {
                let a1213 = v2;
                let a1214 = 8u64;
                if nat_eq(a1213, a1214) {
                    Ok((true, f11(f129(f20(v1, 6u64))?)?))
                } else {
                    Ok((true, f11(f175(v1, {
                        let (h1215, _) = v5;
                        h1215
                    })?)?))
                }
            } else {
                let v6: (bool, List<u64>) = f159(v1, {
                    let (h1216, _) = v5;
                    h1216
                })?;
                Ok(({
                    let (h1217, _) = v6;
                    h1217
                }, f11(f162({
                    let (_, h1218) = v6;
                    h1218
                }, {
                    let (_, h1219) = v5;
                    h1219
                })?)?))
            }
        }
    }
}

pub fn f161(v0: List<u64>) -> R<u64> {
    f176(Fn35::F177, 0u64, v0)
}

pub fn f162(v0: List<u64>, v1: u64) -> R<Bytes> {
    f178(Fn36::F179(v1), Bytes::lit(&[]), v0)
}

pub fn f163(v0: Adt12) -> R<Option<Adt12>> {
    if match v0.clone() {
        Adt12::C0(_, _, h1220) => {
            h1220
        }
    } {
        Ok(None::<Adt12>)
    } else {
        let m1222 = match v0 {
            Adt12::C0(h1221, _, _) => {
                h1221
            }
        };
        match m1222.uncons() {
            None => {
                Ok(None::<Adt12>)
            }
            Some((v1, v2)) => {
                let m1224 = match v1.clone() {
                    Adt13::C0(h1223, _, _) => {
                        h1223
                    }
                };
                match m1224.uncons() {
                    None => {
                        Ok(Some(f180(v2, f11(match v1 {
                            Adt13::C0(_, h1225, _) => {
                                h1225
                            }
                        })?)))
                    }
                    Some((v3, v4)) => {
                        let v5: Bytes = {
                            let a1227 = match v1.clone() {
                                Adt13::C0(_, h1226, _) => {
                                    h1226
                                }
                            };
                            let a1228 = f181(v3.clone())?;
                            append_bytes(a1227, a1228)
                        };
                        let v6: u64 = match v1 {
                            Adt13::C0(_, _, h1229) => {
                                h1229
                            }
                        };
                        let v7: Bytes = match v3.clone() {
                            Adt0::C0(_, _, _, h1230) => {
                                h1230
                            }
                        };
                        let v8: (bool, List<Adt0>) = f3(v7.clone())?;
                        let v9: bool = {
                            let a1232 = match v3.clone() {
                                Adt0::C0(_, h1231, _, _) => {
                                    h1231
                                }
                            };
                            let a1233 = 2u64;
                            if if nat_eq(a1232, a1233) {
                                let a1234 = v6;
                                let a1235 = 32u64;
                                nat_lt(a1234, a1235)
                            } else {
                                false
                            } {
                                let a1237 = 0u64;
                                let a1238 = {
                                    let a1236 = v7.clone();
                                    length_bytes(a1236)
                                };
                                if nat_lt(a1237, a1238) {
                                    let (h1239, _) = v8;
                                    h1239
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        if v9 {
                            Ok(Some(Adt12::C0(List::cons(Adt13::C0(f146({
                                let (_, h1240) = v8;
                                h1240
                            }), Bytes::lit(&[]), {
                                let a1241 = v6;
                                let a1242 = 1u64;
                                nat_add(a1241, a1242)?
                            }), List::cons(Adt13::C0(v4, v5, v6), v2)), Bytes::lit(&[]), false)))
                        } else {
                            Ok(Some(Adt12::C0(List::cons(Adt13::C0(v4, {
                                let a1244 = match v3 {
                                    Adt0::C0(_, h1243, _, _) => {
                                        h1243
                                    }
                                };
                                let a1245 = 2u64;
                                if nat_eq(a1244, a1245) {
                                    let a1246 = v5;
                                    let a1247 = f11(v7)?;
                                    append_bytes(a1246, a1247)
                                } else {
                                    v5
                                }
                            }, v6), v2), Bytes::lit(&[]), false)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f164(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    f182(v1, v0)
}

pub fn f165(v0: Fn31, p1251: List<Adt3>, p1252: List<Bytes>) -> R<List<Adt3>> {
    let m1250 = (p1251, p1252);
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let (v1, v2) = m1250;
                let m1248 = v2;
                match m1248.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(({
                            let c1249: Fn31 = v0.clone();
                            c1249.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: u64, v1: List<Adt3>, v2: Bytes) -> R<List<Adt3>> {
    Ok(List::cons(Adt3::C0(true, v2, {
        let a1253 = v0;
        let a1254 = 1u64;
        nat_add(a1253, a1254)?
    }), v1))
}

pub fn f167(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f3(v0.clone())?;
    let v3: List<Adt0> = {
        let (_, h1255) = v2;
        h1255
    };
    let a1256 = v1;
    let a1257 = 1u64;
    if nat_eq(a1256, a1257) {
        let v4: (bool, Adt0) = f46(v0, 2u64)?;
        Ok(({
            let (h1258, _) = v4.clone();
            h1258
        }, {
            let a1262 = {
                let m1261 = {
                    let (_, h1260) = v4.clone();
                    h1260
                };
                match m1261 {
                    Adt0::C0(_, h1259, _, _) => {
                        h1259
                    }
                }
            };
            let a1263 = 5u64;
            if nat_eq(a1262, a1263) {
                f10({
                    let m1266 = {
                        let (_, h1265) = v4;
                        h1265
                    };
                    match m1266 {
                        Adt0::C0(_, _, h1264, _) => {
                            h1264
                        }
                    }
                }, 4u64)?
            } else {
                Bytes::lit(&[])
            }
        }))
    } else {
        let a1267 = v1;
        let a1268 = 2u64;
        if nat_eq(a1267, a1268) {
            let v5: (bool, Adt0) = f46(v0, 3u64)?;
            Ok(({
                let (h1269, _) = v5;
                h1269
            }, f10(f12({
                let (_, h1270) = v5;
                h1270
            }), 8u64)?))
        } else {
            let a1271 = v1;
            let a1272 = 3u64;
            if nat_eq(a1271, a1272) {
                let v6: (bool, Adt0) = f46(v0, 4u64)?;
                Ok(({
                    let (h1273, _) = v6;
                    h1273
                }, f11(f6({
                    let (_, h1274) = v6;
                    h1274
                }))?))
            } else {
                let a1275 = v1;
                let a1276 = 4u64;
                if nat_eq(a1275, a1276) {
                    let v7: (bool, Adt0) = f46(v0, 5u64)?;
                    let v8: (bool, Bytes) = f143(f6({
                        let (_, h1277) = v7;
                        h1277
                    }))?;
                    Ok(({
                        let (h1278, _) = v7;
                        if h1278 {
                            let (h1279, _) = v8;
                            h1279
                        } else {
                            false
                        }
                    }, {
                        let (_, h1280) = v8;
                        h1280
                    }))
                } else {
                    let a1281 = v1;
                    let a1282 = 6u64;
                    if nat_eq(a1281, a1282) {
                        Ok(({
                            let (h1283, _) = v2;
                            h1283
                        }, f183(v3)?))
                    } else {
                        let a1284 = v1;
                        let a1285 = 7u64;
                        if nat_eq(a1284, a1285) {
                            let v9: (bool, List<u64>) = f159(v3, 8u64)?;
                            Ok(({
                                let (h1286, _) = v2;
                                if h1286 {
                                    let (h1287, _) = v9;
                                    h1287
                                } else {
                                    false
                                }
                            }, f162({
                                let (_, h1288) = v9;
                                h1288
                            }, 8u64)?))
                        } else {
                            let a1289 = v1;
                            let a1290 = 8u64;
                            if nat_eq(a1289, a1290) {
                                Ok(({
                                    let (h1291, _) = v2;
                                    h1291
                                }, f129(f20(v3, 9u64))?))
                            } else {
                                let a1292 = v1;
                                let a1293 = 9u64;
                                if nat_eq(a1292, a1293) {
                                    let v10: (bool, Bytes) = f111(f20(v3, 10u64), 0u64)?;
                                    Ok(({
                                        let (h1294, _) = v2;
                                        if h1294 {
                                            let (h1295, _) = v10;
                                            h1295
                                        } else {
                                            false
                                        }
                                    }, {
                                        let (_, h1296) = v10;
                                        h1296
                                    }))
                                } else {
                                    let a1297 = v1;
                                    let a1298 = 11u64;
                                    if nat_eq(a1297, a1298) {
                                        let v11: (bool, Adt0) = f46(v0, 22u64)?;
                                        let v12: (bool, Bytes) = f127(f6({
                                            let (_, h1299) = v11;
                                            h1299
                                        }))?;
                                        Ok(({
                                            let (h1300, _) = v11;
                                            if h1300 {
                                                let (h1301, _) = v12;
                                                h1301
                                            } else {
                                                false
                                            }
                                        }, {
                                            let (_, h1302) = v12;
                                            h1302
                                        }))
                                    } else {
                                        let a1303 = v1;
                                        let a1304 = 12u64;
                                        if nat_eq(a1303, a1304) {
                                            let v13: (bool, Bytes) = f111(f20(v3, 23u64), 1u64)?;
                                            Ok(({
                                                let (h1305, _) = v2;
                                                if h1305 {
                                                    let (h1306, _) = v13;
                                                    h1306
                                                } else {
                                                    false
                                                }
                                            }, {
                                                let (_, h1307) = v13;
                                                h1307
                                            }))
                                        } else {
                                            let a1308 = v1;
                                            let a1309 = 13u64;
                                            if nat_eq(a1308, a1309) {
                                                let v14: (bool, Adt0) = f46(v0, 14u64)?;
                                                let v15: (bool, Bytes) = f127(f6({
                                                    let (_, h1310) = v14;
                                                    h1310
                                                }))?;
                                                Ok(({
                                                    let (h1311, _) = v14;
                                                    if h1311 {
                                                        let (h1312, _) = v15;
                                                        h1312
                                                    } else {
                                                        false
                                                    }
                                                }, {
                                                    let (_, h1313) = v15;
                                                    h1313
                                                }))
                                            } else {
                                                let a1314 = v1;
                                                let a1315 = 14u64;
                                                if nat_eq(a1314, a1315) {
                                                    let v16: (bool, Bytes) = f111(f20(v3, 15u64), 1u64)?;
                                                    Ok(({
                                                        let (h1316, _) = v2;
                                                        if h1316 {
                                                            let (h1317, _) = v16;
                                                            h1317
                                                        } else {
                                                            false
                                                        }
                                                    }, {
                                                        let (_, h1318) = v16;
                                                        h1318
                                                    }))
                                                } else {
                                                    Ok((true, Bytes::lit(&[])))
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

pub fn f168(v0: Fn32, p1322: bool, p1323: List<Adt6>) -> bool {
    let m1321 = (p1322, p1323);
    {
        let mut m1321 = m1321;
        loop {
            let step = {
                let (v1, v2) = m1321;
                let m1319 = v2;
                match m1319.uncons() {
                    None => {
                        Err::<(bool, List<Adt6>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt6>), bool>(({
                            let c1320: Fn32 = v0.clone();
                            c1320.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: List<u64>, v1: Bytes, v2: bool, v3: Adt6) -> bool {
    if if f184(match v3.clone() {
        Adt6::C0(_, _, _, _, h1324) => {
            h1324
        }
    }, v1) {
        !f141(v0, match v3 {
            Adt6::C0(h1325, _, _, _, _) => {
                h1325
            }
        })
    } else {
        false
    } {
        false
    } else {
        v2
    }
}

pub fn f170(v0: Fn33, p1329: (bool, List<u64>), p1330: List<Adt0>) -> R<(bool, List<u64>)> {
    let m1328 = (p1329, p1330);
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let (v1, v2) = m1328;
                let m1326 = v2;
                match m1326.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(({
                            let c1327: Fn33 = v0.clone();
                            c1327.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: u64, v1: (bool, List<u64>), v2: Adt0) -> R<(bool, List<u64>)> {
    let a1332 = match v2.clone() {
        Adt0::C0(h1331, _, _, _) => {
            h1331
        }
    };
    let a1333 = v0;
    if nat_eq(a1332, a1333) {
        let a1335 = match v2.clone() {
            Adt0::C0(_, h1334, _, _) => {
                h1334
            }
        };
        let a1336 = 2u64;
        if nat_eq(a1335, a1336) {
            let v3: (bool, List<u64>) = f185(match v2 {
                Adt0::C0(_, _, _, h1337) => {
                    h1337
                }
            })?;
            Ok(({
                let (h1338, _) = v1;
                if h1338 {
                    let (h1339, _) = v3;
                    h1339
                } else {
                    false
                }
            }, f172(Fn34::F186, {
                let (_, h1340) = v1;
                h1340
            }, f172(Fn34::F187, List::<u64>::nil(), {
                let (_, h1341) = v3;
                h1341
            }))))
        } else {
            let a1343 = match v2.clone() {
                Adt0::C0(_, h1342, _, _) => {
                    h1342
                }
            };
            let a1344 = 0u64;
            if nat_eq(a1343, a1344) {
                Ok(({
                    let (h1345, _) = v1;
                    h1345
                }, List::cons(match v2 {
                    Adt0::C0(_, _, h1346, _) => {
                        h1346
                    }
                }, {
                    let (_, h1347) = v1;
                    h1347
                })))
            } else {
                Ok(v1)
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f172(v0: Fn34, p1351: List<u64>, p1352: List<u64>) -> List<u64> {
    let m1350 = (p1351, p1352);
    {
        let mut m1350 = m1350;
        loop {
            let step = {
                let (v1, v2) = m1350;
                let m1348 = v2;
                match m1348.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1349: Fn34 = v0.clone();
                            c1349.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f174(v0: u64) -> (u64, u64) {
    let a1353 = v0;
    let a1354 = 1u64;
    if if nat_eq(a1353, a1354) {
        true
    } else {
        let a1355 = v0;
        let a1356 = 14u64;
        nat_eq(a1355, a1356)
    } {
        (4u64, 0u64)
    } else {
        let a1357 = v0;
        let a1358 = 11u64;
        if if nat_eq(a1357, a1358) {
            true
        } else {
            let a1359 = v0;
            let a1360 = 15u64;
            nat_eq(a1359, a1360)
        } {
            (10u64, 0u64)
        } else {
            let a1361 = v0;
            let a1362 = 7u64;
            if nat_eq(a1361, a1362) {
                (7u64, 8u64)
            } else {
                let a1363 = v0;
                let a1364 = 13u64;
                if nat_eq(a1363, a1364) {
                    (11u64, 8u64)
                } else {
                    let a1365 = v0;
                    let a1366 = 12u64;
                    if nat_eq(a1365, a1366) {
                        (11u64, 4u64)
                    } else {
                        let a1367 = v0;
                        let a1368 = 6u64;
                        if nat_eq(a1367, a1368) {
                            (5u64, 4u64)
                        } else {
                            let a1369 = v0;
                            let a1370 = 4u64;
                            if if nat_eq(a1369, a1370) {
                                true
                            } else {
                                let a1371 = v0;
                                let a1372 = 5u64;
                                if nat_eq(a1371, a1372) {
                                    true
                                } else {
                                    let a1373 = v0;
                                    let a1374 = 10u64;
                                    if nat_eq(a1373, a1374) {
                                        true
                                    } else {
                                        let a1375 = v0;
                                        let a1376 = 16u64;
                                        nat_eq(a1375, a1376)
                                    }
                                }
                            } {
                                (5u64, 2u64)
                            } else {
                                let a1377 = v0;
                                let a1378 = 8u64;
                                if nat_eq(a1377, a1378) {
                                    (6u64, 0u64)
                                } else {
                                    (5u64, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(v0: List<Adt0>, v1: u64) -> R<Bytes> {
    f188(Fn37::F189(v1), Bytes::lit(&[]), v0)
}

pub fn f176(v0: Fn35, p1382: u64, p1383: List<u64>) -> R<u64> {
    let m1381 = (p1382, p1383);
    {
        let mut m1381 = m1381;
        loop {
            let step = {
                let (v1, v2) = m1381;
                let m1379 = v2;
                match m1379.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c1380: Fn35 = v0.clone();
                            c1380.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: u64, _: u64) -> R<u64> {
    let a1384 = v0;
    let a1385 = 1u64;
    nat_add(a1384, a1385)
}

pub fn f178(v0: Fn36, p1389: Bytes, p1390: List<u64>) -> R<Bytes> {
    let m1388 = (p1389, p1390);
    {
        let mut m1388 = m1388;
        loop {
            let step = {
                let (v1, v2) = m1388;
                let m1386 = v2;
                match m1386.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1387: Fn36 = v0.clone();
                            c1387.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1388 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: u64, v1: Bytes, v2: u64) -> R<Bytes> {
    let a1391 = v1;
    let a1392 = f10(v2, v0)?;
    Ok(append_bytes(a1391, a1392))
}

pub fn f180(v0: List<Adt13>, v1: Bytes) -> Adt12 {
    let m1393 = v0;
    match m1393.uncons() {
        None => {
            Adt12::C0(List::<Adt13>::nil(), v1, true)
        }
        Some((v2, v3)) => {
            Adt12::C0(List::cons(Adt13::C0(match v2.clone() {
                Adt13::C0(h1394, _, _) => {
                    h1394
                }
            }, {
                let a1396 = match v2.clone() {
                    Adt13::C0(_, h1395, _) => {
                        h1395
                    }
                };
                let a1397 = v1;
                append_bytes(a1396, a1397)
            }, match v2 {
                Adt13::C0(_, _, h1398) => {
                    h1398
                }
            }), v3), Bytes::lit(&[]), false)
        }
    }
}

pub fn f181(v0: Adt0) -> R<Bytes> {
    let v1: u64 = match v0.clone() {
        Adt0::C0(_, h1399, _, _) => {
            h1399
        }
    };
    let a1411 = f10(match v0.clone() {
        Adt0::C0(h1400, _, _, _) => {
            h1400
        }
    }, 8u64)?;
    let a1412 = {
        let a1409 = f59(v1);
        let a1410 = {
            let a1401 = v1;
            let a1402 = 0u64;
            if if nat_eq(a1401, a1402) {
                true
            } else {
                let a1403 = v1;
                let a1404 = 1u64;
                nat_eq(a1403, a1404)
            } {
                f10(match v0 {
                    Adt0::C0(_, _, h1405, _) => {
                        h1405
                    }
                }, 8u64)?
            } else {
                let a1406 = v1;
                let a1407 = 5u64;
                if nat_eq(a1406, a1407) {
                    f10(match v0 {
                        Adt0::C0(_, _, h1408, _) => {
                            h1408
                        }
                    }, 4u64)?
                } else {
                    Bytes::lit(&[])
                }
            }
        };
        append_bytes(a1409, a1410)
    };
    Ok(append_bytes(a1411, a1412))
}

pub fn f182(v0: Adt0, v1: List<Adt0>) -> List<Adt0> {
    let v2: Adt14 = f190(Fn38::F191(v0.clone()), Adt14::C0(false, List::<Adt0>::nil()), v1);
    if match v2.clone() {
        Adt14::C0(h1413, _) => {
            h1413
        }
    } {
        f16(match v2 {
            Adt14::C0(_, h1414) => {
                h1414
            }
        })
    } else {
        f16(List::cons(v0, match v2 {
            Adt14::C0(_, h1415) => {
                h1415
            }
        }))
    }
}

pub fn f183(v0: List<Adt0>) -> R<Bytes> {
    f188(Fn37::F192, Bytes::lit(&[]), v0)
}

pub fn f184(v0: List<Bytes>, v1: Bytes) -> bool {
    f157(Fn30::F193(v1), false, v0)
}

pub fn f185(v0: Bytes) -> R<(bool, List<u64>)> {
    let v1: Adt15 = {
        let (h1416, _) = f194(Fn39::F195(v0.clone()), {
            let a1418 = {
                let a1417 = v0;
                length_bytes(a1417)
            };
            let a1419 = 1u64;
            nat_add(a1418, a1419)?
        }, Adt15::C0(0u64, true, List::<u64>::nil()))?;
        h1416
    };
    Ok((match v1.clone() {
        Adt15::C0(_, h1420, _) => {
            h1420
        }
    }, match v1 {
        Adt15::C0(_, _, h1421) => {
            h1421
        }
    }))
}

pub fn f186(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f187(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f188(v0: Fn37, p1425: Bytes, p1426: List<Adt0>) -> R<Bytes> {
    let m1424 = (p1425, p1426);
    {
        let mut m1424 = m1424;
        loop {
            let step = {
                let (v1, v2) = m1424;
                let m1422 = v2;
                match m1422.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt0>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt0>), R<Bytes>>(({
                            let c1423: Fn37 = v0.clone();
                            c1423.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: u64, v1: Bytes, v2: Adt0) -> R<Bytes> {
    let a1428 = match v2.clone() {
        Adt0::C0(h1427, _, _, _) => {
            h1427
        }
    };
    let a1429 = v0;
    if nat_eq(a1428, a1429) {
        let a1431 = match v2.clone() {
            Adt0::C0(_, h1430, _, _) => {
                h1430
            }
        };
        let a1432 = 2u64;
        if nat_eq(a1431, a1432) {
            let a1434 = v1;
            let a1435 = match v2 {
                Adt0::C0(_, _, _, h1433) => {
                    h1433
                }
            };
            Ok(append_bytes(a1434, a1435))
        } else {
            let a1437 = match v2.clone() {
                Adt0::C0(_, h1436, _, _) => {
                    h1436
                }
            };
            let a1438 = 5u64;
            if nat_eq(a1437, a1438) {
                let a1440 = v1;
                let a1441 = f10(match v2 {
                    Adt0::C0(_, _, h1439, _) => {
                        h1439
                    }
                }, 4u64)?;
                Ok(append_bytes(a1440, a1441))
            } else {
                let a1443 = match v2.clone() {
                    Adt0::C0(_, h1442, _, _) => {
                        h1442
                    }
                };
                let a1444 = 1u64;
                if nat_eq(a1443, a1444) {
                    let a1446 = v1;
                    let a1447 = f10(match v2 {
                        Adt0::C0(_, _, h1445, _) => {
                            h1445
                        }
                    }, 8u64)?;
                    Ok(append_bytes(a1446, a1447))
                } else {
                    Ok(v1)
                }
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f190(v0: Fn38, p1451: Adt14, p1452: List<Adt0>) -> Adt14 {
    let m1450 = (p1451, p1452);
    {
        let mut m1450 = m1450;
        loop {
            let step = {
                let (v1, v2) = m1450;
                let m1448 = v2;
                match m1448.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt0>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt0>), Adt14>(({
                            let c1449: Fn38 = v0.clone();
                            c1449.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt0, v1: Adt14, v2: Adt0) -> Adt14 {
    if match v1.clone() {
        Adt14::C0(h1453, _) => {
            h1453
        }
    } {
        Adt14::C0(true, List::cons(v2, match v1 {
            Adt14::C0(_, h1454) => {
                h1454
            }
        }))
    } else {
        let a1457 = match v0.clone() {
            Adt0::C0(h1455, _, _, _) => {
                h1455
            }
        };
        let a1458 = match v2.clone() {
            Adt0::C0(h1456, _, _, _) => {
                h1456
            }
        };
        if nat_lt(a1457, a1458) {
            Adt14::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt14::C0(_, h1459) => {
                    h1459
                }
            })))
        } else {
            Adt14::C0(false, List::cons(v2, match v1 {
                Adt14::C0(_, h1460) => {
                    h1460
                }
            }))
        }
    }
}

pub fn f192(v0: Bytes, v1: Adt0) -> R<Bytes> {
    let a1462 = match v1.clone() {
        Adt0::C0(h1461, _, _, _) => {
            h1461
        }
    };
    let a1463 = 7u64;
    if nat_eq(a1462, a1463) {
        let a1465 = match v1.clone() {
            Adt0::C0(_, h1464, _, _) => {
                h1464
            }
        };
        let a1466 = 2u64;
        if nat_eq(a1465, a1466) {
            let a1468 = v0;
            let a1469 = f11(match v1 {
                Adt0::C0(_, _, _, h1467) => {
                    h1467
                }
            })?;
            Ok(append_bytes(a1468, a1469))
        } else {
            let a1471 = match v1.clone() {
                Adt0::C0(_, h1470, _, _) => {
                    h1470
                }
            };
            let a1472 = 5u64;
            if nat_eq(a1471, a1472) {
                let a1474 = v0;
                let a1475 = f10(match v1 {
                    Adt0::C0(_, _, h1473, _) => {
                        h1473
                    }
                }, 4u64)?;
                Ok(append_bytes(a1474, a1475))
            } else {
                Ok(v0)
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f193(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1476 = v2;
        let a1477 = v0;
        equal(a1476, a1477)
    }
}

pub fn f194(v0: Fn39, p1482: u64, p1483: Adt15) -> R<(Adt15, bool)> {
    let m1481 = (p1482, p1483);
    {
        let mut m1481 = m1481;
        loop {
            let step = {
                let (v1, v2) = m1481;
                let m1478 = v1;
                if m1478 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1478 - 1;
                    let m1480 = {
                        let c1479: Fn39 = v0.clone();
                        c1479.apply(v2.clone())?
                    };
                    match m1480 {
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
                Ok(next) => { m1481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes, v1: Adt15) -> R<Option<Adt15>> {
    f196(v0, v1)
}

pub fn f196(v0: Bytes, v1: Adt15) -> R<Option<Adt15>> {
    if if match v1.clone() {
        Adt15::C0(_, h1484, _) => {
            h1484
        }
    } {
        let a1487 = match v1.clone() {
            Adt15::C0(h1485, _, _) => {
                h1485
            }
        };
        let a1488 = {
            let a1486 = v0.clone();
            length_bytes(a1486)
        };
        nat_lt(a1487, a1488)
    } else {
        false
    } {
        let v2: (bool, (u64, u64)) = f81(v0, match v1.clone() {
            Adt15::C0(h1489, _, _) => {
                h1489
            }
        })?;
        let (h1490, _) = v2;
        if h1490 {
            Ok(Some(Adt15::C0({
                let (_, h1491) = {
                    let (_, h1492) = v2;
                    h1492
                };
                h1491
            }, true, List::cons({
                let (h1493, _) = {
                    let (_, h1494) = v2;
                    h1494
                };
                h1493
            }, match v1 {
                Adt15::C0(_, _, h1495) => {
                    h1495
                }
            }))))
        } else {
            Ok(Some(Adt15::C0(0u64, false, List::<u64>::nil())))
        }
    } else {
        Ok(None::<Adt15>)
    }
}
