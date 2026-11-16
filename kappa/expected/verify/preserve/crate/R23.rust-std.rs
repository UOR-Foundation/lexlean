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
    C0(bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(bool, List<u64>, List<Adt9>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, std::rc::Rc<Adt12>, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, std::rc::Rc<Adt14>, u64),
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
    C0(bool, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(std::rc::Rc<Adt5>, u64, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(List<Adt21>, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(List<Adt0>, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(bool, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt24 {
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
    F62(u64),
    F63,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn2::F24 => f24(p0, p1),
            Fn2::F25 => Ok(f25(p0, p1)),
            Fn2::F62(k0) => f62(*k0, p0, p1),
            Fn2::F63 => Ok(f63(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28,
    F206,
}

impl Fn3 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> R<Bytes> {
        match self {
            Fn3::F28 => f28(p0, p1),
            Fn3::F206 => f206(p0, p1),
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
    F147,
    F174(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> R<(bool, Bytes)> {
        match self {
            Fn5::F33 => f33(p0, p1),
            Fn5::F147 => f147(p0, p1),
            Fn5::F174(k0) => f174(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F36,
    F38(Bytes),
    F75(Bytes),
    F76,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn6::F36 => Ok(f36(p0)),
            Fn6::F38(k0) => f38(k0.clone(), p0),
            Fn6::F75(k0) => f75(k0.clone(), p0),
            Fn6::F76 => Ok(f76(p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F46(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn7::F46(k0, k1) => f46(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F48,
    F240,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn8::F48 => f48(p0, p1),
            Fn8::F240 => f240(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F50(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt0) -> List<Bytes> {
        match self {
            Fn9::F50(k0) => f50(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F55,
}

impl Fn10 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn10::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F57,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Bytes>, List<(Bytes, Bytes)>), p1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
        match self {
            Fn11::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F59,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> List<Bytes> {
        match self {
            Fn12::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F79,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn13::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F82,
    F115,
    F149,
    F208,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn14::F82 => f82(p0, p1),
            Fn14::F115 => f115(p0, p1),
            Fn14::F149 => f149(p0, p1),
            Fn14::F208 => f208(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F87(Bytes, u64, u64),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, Adt6)) -> R<(u64, Adt6)> {
        match self {
            Fn15::F87(k0, k1, k2) => f87(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F92(Bytes, u64, u64),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, Adt4)) -> R<(u64, Adt4)> {
        match self {
            Fn16::F92(k0, k1, k2) => f92(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F98,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
        match self {
            Fn17::F98 => f98(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F104(Bytes, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt8, p1: (Bytes, Bytes)) -> Adt8 {
        match self {
            Fn18::F104(k0, k1) => f104(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F106,
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, (bool, List<Adt9>)), p1: Bytes) -> R<(u64, (bool, List<Adt9>))> {
        match self {
            Fn19::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F109(List<Adt9>),
}

impl Fn20 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn20::F109(k0) => f109(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F114(u64),
}

impl Fn21 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Adt9) -> R<(bool, List<Adt3>)> {
        match self {
            Fn21::F114(k0) => f114(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F121(Bytes, u64, u64, u64),
    F215,
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, Adt7)) -> R<(u64, Adt7)> {
        match self {
            Fn22::F121(k0, k1, k2, k3) => f121(k0.clone(), *k1, *k2, *k3, p0),
            Fn22::F215 => f215(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F128(Bytes, u64),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
        match self {
            Fn23::F128(k0, k1) => f128(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F137(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn24::F137(k0) => f137(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F144,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn25::F144 => f144(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F151,
}

impl Fn26 {
    pub fn apply(&self, p0: Adt13) -> R<Adt13> {
        match self {
            Fn26::F151 => f151(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F156,
}

impl Fn27 {
    pub fn apply(&self, p0: Adt15) -> R<Adt15> {
        match self {
            Fn27::F156 => f156(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F161,
}

impl Fn28 {
    pub fn apply(&self, p0: (Adt17, Adt16)) -> (Adt17, Adt16) {
        match self {
            Fn28::F161 => f161(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F163(u64, Bytes, u64),
}

impl Fn29 {
    pub fn apply(&self, p0: (u64, Adt5)) -> R<(u64, Adt5)> {
        match self {
            Fn29::F163(k0, k1, k2) => f163(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F167,
}

impl Fn30 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn30::F167 => f167(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F171(u64, Bytes),
}

impl Fn31 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn31::F171(k0, k1) => f171(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F173(List<Adt9>, std::rc::Rc<Adt10>),
}

impl Fn32 {
    pub fn apply(&self, p0: Adt18, p1: Adt9) -> Adt18 {
        match self {
            Fn32::F173(k0, k1) => f173(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F179(u64),
}

impl Fn33 {
    pub fn apply(&self, p0: (bool, List<Adt3>), p1: Bytes) -> R<(bool, List<Adt3>)> {
        match self {
            Fn33::F179(k0) => f179(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F193,
}

impl Fn34 {
    pub fn apply(&self, p0: Adt19) -> Adt19 {
        match self {
            Fn34::F193 => f193(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F202,
}

impl Fn35 {
    pub fn apply(&self, p0: Adt20) -> R<Option<Adt20>> {
        match self {
            Fn35::F202 => f202(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F205(u64),
}

impl Fn36 {
    pub fn apply(&self, p0: u64, p1: Adt0) -> R<u64> {
        match self {
            Fn36::F205(k0) => f205(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F225,
}

impl Fn37 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn37::F225 => f225(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F232(u64),
}

impl Fn38 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn38::F232(k0) => f232(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F234(List<u64>, List<Adt9>),
    F283(Bytes),
}

impl Fn39 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn39::F234(k0, k1) => f234(k0.clone(), k1.clone(), p0, p1),
            Fn39::F283(k0) => f283(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F242(u64),
}

impl Fn40 {
    pub fn apply(&self, p0: List<Adt3>, p1: Bytes) -> R<List<Adt3>> {
        match self {
            Fn40::F242(k0) => f242(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F253(List<u64>, Bytes),
}

impl Fn41 {
    pub fn apply(&self, p0: bool, p1: Adt9) -> bool {
        match self {
            Fn41::F253(k0, k1) => f253(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F255(u64),
}

impl Fn42 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: Adt0) -> R<(bool, List<u64>)> {
        match self {
            Fn42::F255(k0) => f255(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F257,
    F274,
    F275,
}

impl Fn43 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn43::F257 => f257(p0, p1),
            Fn43::F274 => f274(p0, p1),
            Fn43::F275 => f275(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F261,
}

impl Fn44 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn44::F261 => f261(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F263(u64),
}

impl Fn45 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn45::F263(k0) => f263(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F277(u64),
    F280,
}

impl Fn46 {
    pub fn apply(&self, p0: Bytes, p1: Adt0) -> R<Bytes> {
        match self {
            Fn46::F277(k0) => f277(*k0, p0, p1),
            Fn46::F280 => f280(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F279(std::rc::Rc<Adt0>),
}

impl Fn47 {
    pub fn apply(&self, p0: Adt23, p1: Adt0) -> Adt23 {
        match self {
            Fn47::F279(k0) => f279((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn48 {
    F285(Bytes),
}

impl Fn48 {
    pub fn apply(&self, p0: Adt24) -> R<Option<Adt24>> {
        match self {
            Fn48::F285(k0) => f285(k0.clone(), p0),
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

pub fn f2(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m34 = f13(v0.clone(), v1)?;
    match m34 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a35 = f14(v0);
                let a36 = v2;
                append_bytes(a35, a36)
            }))
        }
    }
}

pub fn f3(v0: Bytes) -> R<(bool, List<Adt0>)> {
    let v1: Adt1 = f15(v0, 0u64)?;
    Ok((match v1.clone() {
        Adt1::C0(_, h37, _, _) => {
            h37
        }
    }, f16(match v1 {
        Adt1::C0(_, _, _, h38) => {
            h38
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
        let (h39, _) = v2;
        if h39 {
            let (_, h40) = v2;
            if h40 {
                true
            } else {
                let a41 = f26(v1.clone())?;
                let a42 = 0u64;
                nat_eq(a41, a42)
            }
        } else {
            false
        }
    }, f27(Fn3::F28, Bytes::lit(&[]), f29(v1, v3))?))
}

pub fn f6(v0: Adt0) -> Bytes {
    let a44 = match v0.clone() {
        Adt0::C0(_, h43, _, _) => {
            h43
        }
    };
    let a45 = 2u64;
    if nat_eq(a44, a45) {
        match v0 {
            Adt0::C0(_, _, _, h46) => {
                h46
            }
        }
    } else {
        Bytes::lit(&[])
    }
}

pub fn f7(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: Adt2 = {
        let (h47, _) = f30(Fn4::F31, {
            let a51 = {
                let a49 = {
                    let a48 = v0.clone();
                    length_bytes(a48)
                };
                let a50 = 8u64;
                nat_mul(a49, a50)?
            };
            let a52 = 16u64;
            nat_add(a51, a52)?
        }, Adt2::C0(true, Bytes::lit(&[]), List::cons(Adt3::C0(true, v0, 0u64), List::<Adt3>::nil())))?;
        h47
    };
    let v4: bool = {
        let m54 = match v1.clone() {
            Adt2::C0(_, _, h53) => {
                h53
            }
        };
        match m54.uncons() {
            None => {
                true
            }
            Some((_, _)) => {
                false
            }
        }
    };
    Ok((if match v1.clone() {
        Adt2::C0(h55, _, _) => {
            h55
        }
    } {
        v4
    } else {
        false
    }, match v1 {
        Adt2::C0(_, h56, _) => {
            h56
        }
    }))
}

pub fn f8(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f3(v0)?;
    let v3: List<Bytes> = f20({
        let (_, h57) = v2;
        h57
    }, v1);
    f32(Fn5::F33, ({
        let (h58, _) = v2;
        h58
    }, f10(f26(v3.clone())?, 4u64)?), f29(v3.clone(), f34(v3, 1u64)?))
}

pub fn f9(v0: Adt0) -> bool {
    let a60 = 0u64;
    let a61 = match v0.clone() {
        Adt0::C0(h59, _, _, _) => {
            h59
        }
    };
    if nat_lt(a60, a61) {
        let a63 = match v0 {
            Adt0::C0(_, h62, _, _) => {
                h62
            }
        };
        let a64 = 0u64;
        nat_eq(a63, a64)
    } else {
        false
    }
}

pub fn f10(v0: u64, v1: u64) -> R<Bytes> {
    let (_, h65) = f35(Fn6::F36, v1, (v0, Bytes::lit(&[])))?;
    Ok(h65)
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f37(v0)?;
    let (_, h66) = f35(Fn6::F38(v1), 32u64, (0u64, Bytes::lit(&[])))?;
    Ok(h66)
}

pub fn f12(v0: Adt0) -> u64 {
    let a68 = 0u64;
    let a69 = match v0.clone() {
        Adt0::C0(h67, _, _, _) => {
            h67
        }
    };
    if if nat_lt(a68, a69) {
        let a71 = match v0.clone() {
            Adt0::C0(_, h70, _, _) => {
                h70
            }
        };
        let a72 = 0u64;
        nat_eq(a71, a72)
    } else {
        false
    } {
        match v0 {
            Adt0::C0(_, _, h73, _) => {
                h73
            }
        }
    } else {
        0u64
    }
}

pub fn f13(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f39(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f37(v1)?))
    } else {
        if f39(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f40(v1)?))
        } else {
            if f39(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f41(v1)?))
            } else {
                if f39(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f42(v1)?))
                } else {
                    if f39(v0, Str::lit("blake3")) {
                        Ok(Some(f43(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f14(v0: Bytes) -> Bytes {
    let a74 = v0;
    let a75 = f44(Str::lit(":"));
    append_bytes(a74, a75)
}

pub fn f15(v0: Bytes, v1: u64) -> R<Adt1> {
    let (h76, _) = f45(Fn7::F46(v1, v0.clone()), {
        let a78 = {
            let a77 = v0;
            length_bytes(a77)
        };
        let a79 = 2u64;
        nat_add(a78, a79)?
    }, Adt1::C0(0u64, true, false, List::<Adt0>::nil()))?;
    Ok(h76)
}

pub fn f16(v0: List<Adt0>) -> List<Adt0> {
    f47(Fn8::F48, List::<Adt0>::nil(), v0)
}

pub fn f17(v0: Fn0, p83: Adt0, p84: List<Adt0>) -> Adt0 {
    let m82 = (p83, p84);
    {
        let mut m82 = m82;
        loop {
            let step = {
                let (v1, v2) = m82;
                let m80 = v2;
                match m80.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c81: Fn0 = v0.clone();
                            c81.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m82 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: Adt0, v2: Adt0) -> Adt0 {
    let a86 = 0u64;
    let a87 = match v1.clone() {
        Adt0::C0(h85, _, _, _) => {
            h85
        }
    };
    if nat_lt(a86, a87) {
        v1
    } else {
        let a89 = match v2.clone() {
            Adt0::C0(h88, _, _, _) => {
                h88
            }
        };
        let a90 = v0;
        if nat_eq(a89, a90) {
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
    f49(Fn9::F50(v1), List::<Bytes>::nil(), f16(v0))
}

pub fn f21(v0: Fn1, p94: (bool, bool), p95: List<Bytes>) -> R<(bool, bool)> {
    let m93 = (p94, p95);
    {
        let mut m93 = m93;
        loop {
            let step = {
                let (v1, v2) = m93;
                let m91 = v2;
                match m91.uncons() {
                    None => {
                        Err::<((bool, bool), List<Bytes>), R<(bool, bool)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, bool), List<Bytes>), R<(bool, bool)>>(({
                            let c92: Fn1 = v0.clone();
                            c92.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m93 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: (bool, bool), v1: Bytes) -> R<(bool, bool)> {
    let v2: (bool, Adt0) = f51(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f51(v1, 2u64)?;
    Ok(({
        let (h96, _) = v0;
        if h96 {
            let (h97, _) = v2;
            if h97 {
                let (h98, _) = v3.clone();
                h98
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let (_, h99) = v0;
        if h99 {
            true
        } else {
            let a102 = {
                let a101 = f6({
                    let (_, h100) = v2;
                    h100
                });
                length_bytes(a101)
            };
            let a103 = 0u64;
            if nat_eq(a102, a103) {
                let a105 = 0u64;
                let a106 = f12({
                    let (_, h104) = v3.clone();
                    h104
                });
                if nat_lt(a105, a106) {
                    let a108 = f12({
                        let (_, h107) = v3;
                        h107
                    });
                    let a109 = 9223372036854775807u64;
                    nat_le(a108, a109)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }))
}

pub fn f23(v0: Fn2, p113: List<Bytes>, p114: List<Bytes>) -> R<List<Bytes>> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m110 = v2;
                match m110.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c111: Fn2 = v0.clone();
                            c111.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons({
        let a117 = f52(f6({
            let (_, h115) = f51(v1.clone(), 1u64)?;
            h115
        }))?;
        let a118 = f53(f12({
            let (_, h116) = f51(v1, 2u64)?;
            h116
        }))?;
        append_bytes(a117, a118)
    }, v0))
}

pub fn f25(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f26(v0: List<Bytes>) -> R<u64> {
    f54(Fn10::F55, 0u64, v0)
}

pub fn f27(v0: Fn3, p122: Bytes, p123: List<Bytes>) -> R<Bytes> {
    let m121 = (p122, p123);
    {
        let mut m121 = m121;
        loop {
            let step = {
                let (v1, v2) = m121;
                let m119 = v2;
                match m119.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), R<Bytes>>(({
                            let c120: Fn3 = v0.clone();
                            c120.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a128 = v0;
    let a129 = {
        let a126 = f11(f6({
            let (_, h124) = f51(v1.clone(), 1u64)?;
            h124
        }))?;
        let a127 = f10(f12({
            let (_, h125) = f51(v1, 2u64)?;
            h125
        }), 8u64)?;
        append_bytes(a126, a127)
    };
    Ok(append_bytes(a128, a129))
}

pub fn f29(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let v2: List<(Bytes, Bytes)> = {
        let (_, h130) = f56(Fn11::F57, (v1, List::<(Bytes, Bytes)>::nil()), v0);
        h130
    };
    f58(Fn12::F59, List::<Bytes>::nil(), f60(v2))
}

pub fn f30(v0: Fn4, p135: u64, p136: Adt2) -> R<(Adt2, bool)> {
    let m134 = (p135, p136);
    {
        let mut m134 = m134;
        loop {
            let step = {
                let (v1, v2) = m134;
                let m131 = v1;
                if m131 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m131 - 1;
                    let m133 = {
                        let c132: Fn4 = v0.clone();
                        c132.apply(v2.clone())?
                    };
                    match m133 {
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
                Ok(next) => { m134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<Option<Adt2>> {
    f61(v0)
}

pub fn f32(v0: Fn5, p140: (bool, Bytes), p141: List<Bytes>) -> R<(bool, Bytes)> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), R<(bool, Bytes)>>(({
                            let c138: Fn5 = v0.clone();
                            c138.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f51(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f51(v1, 2u64)?;
    Ok(({
        let (h142, _) = v0;
        if h142 {
            let (h143, _) = v2;
            if h143 {
                let (h144, _) = v3;
                h144
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a150 = {
            let (_, h145) = v0;
            h145
        };
        let a151 = {
            let a148 = f11(f6({
                let (_, h146) = v2;
                h146
            }))?;
            let a149 = f11(f6({
                let (_, h147) = v3;
                h147
            }))?;
            append_bytes(a148, a149)
        };
        append_bytes(a150, a151)
    }))
}

pub fn f34(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    f23(Fn2::F62(v1), List::<Bytes>::nil(), f23(Fn2::F63, List::<Bytes>::nil(), v0)?)
}

pub fn f35(v0: Fn6, p155: u64, p156: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v1, v2) = m154;
                let m152 = v1;
                if m152 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m152 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c153: Fn6 = v0.clone();
                        c153.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a158 = {
            let (h157, _) = v0.clone();
            h157
        };
        let a159 = 256u64;
        let a160 = 0u64;
        nat_quot(a158, a159, a160)
    }, {
        let a166 = {
            let (_, h161) = v0;
            h161
        };
        let a167 = f64({
            let a163 = {
                let (h162, _) = v0;
                h162
            };
            let a164 = 256u64;
            let a165 = 0u64;
            nat_rem(a163, a164, a165)
        });
        append_bytes(a166, a167)
    })
}

pub fn f37(v0: Bytes) -> R<Bytes> {
    Ok(f65(f66(v0)?))
}

pub fn f38(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    let v2: u64 = {
        let a169 = {
            let (h168, _) = v1.clone();
            h168
        };
        let a170 = 2u64;
        nat_mul(a169, a170)?
    };
    Ok(({
        let a172 = {
            let (h171, _) = v1;
            h171
        };
        let a173 = 1u64;
        nat_add(a172, a173)?
    }, {
        let a181 = {
            let (_, h174) = v1;
            h174
        };
        let a182 = f64({
            let a179 = {
                let a175 = f67(f68(v0.clone(), v2)?);
                let a176 = 16u64;
                nat_mul(a175, a176)?
            };
            let a180 = f67(f68(v0, {
                let a177 = v2;
                let a178 = 1u64;
                nat_add(a177, a178)?
            })?);
            nat_add(a179, a180)?
        });
        append_bytes(a181, a182)
    }))
}

pub fn f39(v0: Bytes, v1: Str) -> bool {
    let a183 = v0;
    let a184 = f44(v1);
    equal(a183, a184)
}

pub fn f40(v0: Bytes) -> R<Bytes> {
    f69(v0, 6u64)
}

pub fn f41(v0: Bytes) -> R<Bytes> {
    f69(v0, 1u64)
}

pub fn f42(v0: Bytes) -> R<Bytes> {
    let v1: Adt4 = f70(v0)?;
    let a205 = f71(match v1.clone() {
        Adt4::C0(h185, _, _, _, _, _, _, _) => {
            h185
        }
    });
    let a206 = {
        let a203 = f71(match v1.clone() {
            Adt4::C0(_, h186, _, _, _, _, _, _) => {
                h186
            }
        });
        let a204 = {
            let a201 = f71(match v1.clone() {
                Adt4::C0(_, _, h187, _, _, _, _, _) => {
                    h187
                }
            });
            let a202 = {
                let a199 = f71(match v1.clone() {
                    Adt4::C0(_, _, _, h188, _, _, _, _) => {
                        h188
                    }
                });
                let a200 = {
                    let a197 = f71(match v1.clone() {
                        Adt4::C0(_, _, _, _, h189, _, _, _) => {
                            h189
                        }
                    });
                    let a198 = {
                        let a195 = f71(match v1.clone() {
                            Adt4::C0(_, _, _, _, _, h190, _, _) => {
                                h190
                            }
                        });
                        let a196 = {
                            let a193 = f71(match v1.clone() {
                                Adt4::C0(_, _, _, _, _, _, h191, _) => {
                                    h191
                                }
                            });
                            let a194 = f71(match v1 {
                                Adt4::C0(_, _, _, _, _, _, _, h192) => {
                                    h192
                                }
                            });
                            append_bytes(a193, a194)
                        };
                        append_bytes(a195, a196)
                    };
                    append_bytes(a197, a198)
                };
                append_bytes(a199, a200)
            };
            append_bytes(a201, a202)
        };
        append_bytes(a203, a204)
    };
    Ok(append_bytes(a205, a206))
}

pub fn f43(v0: Bytes) -> R<Bytes> {
    let v1: Adt5 = f72(v0)?;
    let a227 = f73(match v1.clone() {
        Adt5::C0(h207, _, _, _, _, _, _, _) => {
            h207
        }
    });
    let a228 = {
        let a225 = f73(match v1.clone() {
            Adt5::C0(_, h208, _, _, _, _, _, _) => {
                h208
            }
        });
        let a226 = {
            let a223 = f73(match v1.clone() {
                Adt5::C0(_, _, h209, _, _, _, _, _) => {
                    h209
                }
            });
            let a224 = {
                let a221 = f73(match v1.clone() {
                    Adt5::C0(_, _, _, h210, _, _, _, _) => {
                        h210
                    }
                });
                let a222 = {
                    let a219 = f73(match v1.clone() {
                        Adt5::C0(_, _, _, _, h211, _, _, _) => {
                            h211
                        }
                    });
                    let a220 = {
                        let a217 = f73(match v1.clone() {
                            Adt5::C0(_, _, _, _, _, h212, _, _) => {
                                h212
                            }
                        });
                        let a218 = {
                            let a215 = f73(match v1.clone() {
                                Adt5::C0(_, _, _, _, _, _, h213, _) => {
                                    h213
                                }
                            });
                            let a216 = f73(match v1 {
                                Adt5::C0(_, _, _, _, _, _, _, h214) => {
                                    h214
                                }
                            });
                            append_bytes(a215, a216)
                        };
                        append_bytes(a217, a218)
                    };
                    append_bytes(a219, a220)
                };
                append_bytes(a221, a222)
            };
            append_bytes(a223, a224)
        };
        append_bytes(a225, a226)
    };
    Ok(append_bytes(a227, a228))
}

pub fn f44(v0: Str) -> Bytes {
    let a229 = v0;
    utf8_encode(a229)
}

pub fn f45(v0: Fn7, p234: u64, p235: Adt1) -> R<(Adt1, bool)> {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m230 = v1;
                if m230 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m230 - 1;
                    let m232 = {
                        let c231: Fn7 = v0.clone();
                        c231.apply(v2.clone())?
                    };
                    match m232 {
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
                Ok(next) => { m233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: u64, v1: Bytes, v2: Adt1) -> R<Option<Adt1>> {
    f74(v1, v0, v2)
}

pub fn f47(v0: Fn8, p239: List<Adt0>, p240: List<Adt0>) -> List<Adt0> {
    let m238 = (p239, p240);
    {
        let mut m238 = m238;
        loop {
            let step = {
                let (v1, v2) = m238;
                let m236 = v2;
                match m236.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c237: Fn8 = v0.clone();
                            c237.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f49(v0: Fn9, p244: List<Bytes>, p245: List<Adt0>) -> List<Bytes> {
    let m243 = (p244, p245);
    {
        let mut m243 = m243;
        loop {
            let step = {
                let (v1, v2) = m243;
                let m241 = v2;
                match m241.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt0>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt0>), List<Bytes>>(({
                            let c242: Fn9 = v0.clone();
                            c242.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: u64, v1: List<Bytes>, v2: Adt0) -> List<Bytes> {
    let a247 = match v2.clone() {
        Adt0::C0(h246, _, _, _) => {
            h246
        }
    };
    let a248 = v0;
    if if nat_eq(a247, a248) {
        let a250 = match v2.clone() {
            Adt0::C0(_, h249, _, _) => {
                h249
            }
        };
        let a251 = 2u64;
        nat_eq(a250, a251)
    } else {
        false
    } {
        List::cons(match v2 {
            Adt0::C0(_, _, _, h252) => {
                h252
            }
        }, v1)
    } else {
        v1
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<(bool, Adt0)> {
    let v2: Adt1 = f15(v0, v1)?;
    if match v2.clone() {
        Adt1::C0(_, h253, _, _) => {
            h253
        }
    } {
        let m255 = match v2 {
            Adt1::C0(_, _, _, h254) => {
                h254
            }
        };
        match m255.uncons() {
            None => {
                Ok((true, f19()))
            }
            Some((v3, _)) => {
                Ok((true, {
                    let a257 = match v3.clone() {
                        Adt0::C0(h256, _, _, _) => {
                            h256
                        }
                    };
                    let a258 = v1;
                    if nat_eq(a257, a258) {
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

pub fn f52(v0: Bytes) -> R<Bytes> {
    let a261 = {
        let (_, h259) = f35(Fn6::F75(v0.clone()), {
            let a260 = v0;
            length_bytes(a260)
        }, (0u64, Bytes::lit(&[])))?;
        h259
    };
    let a262 = Bytes::lit(&[0x00]);
    Ok(append_bytes(a261, a262))
}

pub fn f53(v0: u64) -> R<Bytes> {
    let (_, h263) = f35(Fn6::F76, 8u64, (v0, Bytes::lit(&[])))?;
    Ok(h263)
}

pub fn f54(v0: Fn10, p267: u64, p268: List<Bytes>) -> R<u64> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m264 = v2;
                match m264.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c265: Fn10 = v0.clone();
                            c265.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u64, _: Bytes) -> R<u64> {
    let a269 = v0;
    let a270 = 1u64;
    nat_add(a269, a270)
}

pub fn f56(v0: Fn11, p274: (List<Bytes>, List<(Bytes, Bytes)>), p275: List<Bytes>) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m273 = (p274, p275);
    {
        let mut m273 = m273;
        loop {
            let step = {
                let (v1, v2) = m273;
                let m271 = v2;
                match m271.uncons() {
                    None => {
                        Err::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>), (List<Bytes>, List<(Bytes, Bytes)>)>(({
                            let c272: Fn11 = v0.clone();
                            c272.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: (List<Bytes>, List<(Bytes, Bytes)>), v1: Bytes) -> (List<Bytes>, List<(Bytes, Bytes)>) {
    let m277 = {
        let (h276, _) = v0;
        h276
    };
    match m277.uncons() {
        None => {
            (List::<Bytes>::nil(), f77(Bytes::lit(&[]), v1, {
                let (_, h279) = v0;
                h279
            }))
        }
        Some((v2, v3)) => {
            (v3, f77(v2, v1, {
                let (_, h278) = v0;
                h278
            }))
        }
    }
}

pub fn f58(v0: Fn12, p283: List<Bytes>, p284: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v2;
                match m280.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), List<Bytes>>(({
                            let c281: Fn12 = v0.clone();
                            c281.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<Bytes>, v1: (Bytes, Bytes)) -> List<Bytes> {
    List::cons({
        let (_, h285) = v1;
        h285
    }, v0)
}

pub fn f60(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f78(Fn13::F79, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f61(v0: Adt2) -> R<Option<Adt2>> {
    if match v0.clone() {
        Adt2::C0(h286, _, _) => {
            h286
        }
    } {
        let m288 = match v0.clone() {
            Adt2::C0(_, _, h287) => {
                h287
            }
        };
        match m288.uncons() {
            None => {
                Ok(None::<Adt2>)
            }
            Some((v1, v2)) => {
                if match v1.clone() {
                    Adt3::C0(h289, _, _) => {
                        h289
                    }
                } {
                    let v3: (bool, List<Adt3>) = f80(match v1.clone() {
                        Adt3::C0(_, h290, _) => {
                            h290
                        }
                    }, match v1 {
                        Adt3::C0(_, _, h291) => {
                            h291
                        }
                    })?;
                    let (h292, _) = v3;
                    if h292 {
                        Ok(Some(Adt2::C0(true, match v0 {
                            Adt2::C0(_, h293, _) => {
                                h293
                            }
                        }, f81(Fn14::F82, v2, f83({
                            let (_, h294) = v3;
                            h294
                        })))))
                    } else {
                        Ok(Some(Adt2::C0(false, Bytes::lit(&[]), List::<Adt3>::nil())))
                    }
                } else {
                    Ok(Some(Adt2::C0(true, {
                        let a297 = match v0 {
                            Adt2::C0(_, h295, _) => {
                                h295
                            }
                        };
                        let a298 = match v1 {
                            Adt3::C0(_, h296, _) => {
                                h296
                            }
                        };
                        append_bytes(a297, a298)
                    }, v2)))
                }
            }
        }
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f62(v0: u64, v1: List<Bytes>, v2: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f6({
        let (_, h299) = f51(v2, v0)?;
        h299
    }), v1))
}

pub fn f63(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f64(v0: u64) -> Bytes {
    f84(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f65(v0: Adt6) -> Bytes {
    let a320 = f85(match v0.clone() {
        Adt6::C0(h300, _, _, _, _, _, _, _) => {
            h300
        }
    });
    let a321 = {
        let a318 = f85(match v0.clone() {
            Adt6::C0(_, h301, _, _, _, _, _, _) => {
                h301
            }
        });
        let a319 = {
            let a316 = f85(match v0.clone() {
                Adt6::C0(_, _, h302, _, _, _, _, _) => {
                    h302
                }
            });
            let a317 = {
                let a314 = f85(match v0.clone() {
                    Adt6::C0(_, _, _, h303, _, _, _, _) => {
                        h303
                    }
                });
                let a315 = {
                    let a312 = f85(match v0.clone() {
                        Adt6::C0(_, _, _, _, h304, _, _, _) => {
                            h304
                        }
                    });
                    let a313 = {
                        let a310 = f85(match v0.clone() {
                            Adt6::C0(_, _, _, _, _, h305, _, _) => {
                                h305
                            }
                        });
                        let a311 = {
                            let a308 = f85(match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, h306, _) => {
                                    h306
                                }
                            });
                            let a309 = f85(match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, h307) => {
                                    h307
                                }
                            });
                            append_bytes(a308, a309)
                        };
                        append_bytes(a310, a311)
                    };
                    append_bytes(a312, a313)
                };
                append_bytes(a314, a315)
            };
            append_bytes(a316, a317)
        };
        append_bytes(a318, a319)
    };
    append_bytes(a320, a321)
}

pub fn f66(v0: Bytes) -> R<Adt6> {
    let v1: u64 = {
        let a322 = v0.clone();
        length_bytes(a322)
    };
    let v2: u64 = {
        let a328 = {
            let a325 = {
                let a323 = v1;
                let a324 = 72u64;
                nat_add(a323, a324)?
            };
            let a326 = 64u64;
            let a327 = 0u64;
            nat_quot(a325, a326, a327)
        };
        let a329 = 64u64;
        nat_mul(a328, a329)?
    };
    let (_, h330) = f86(Fn15::F87(v0, v1, v2), {
        let a331 = v2;
        let a332 = 64u64;
        let a333 = 0u64;
        nat_quot(a331, a332, a333)
    }, (0u64, f88()))?;
    Ok(h330)
}

pub fn f67(v0: u64) -> u64 {
    let a334 = 47u64;
    let a335 = v0;
    if nat_lt(a334, a335) {
        let a336 = v0;
        let a337 = 58u64;
        if nat_lt(a336, a337) {
            let a338 = v0;
            let a339 = 48u64;
            nat_sub(a338, a339)
        } else {
            let a340 = 64u64;
            let a341 = v0;
            if nat_lt(a340, a341) {
                let a342 = v0;
                let a343 = 71u64;
                if nat_lt(a342, a343) {
                    let a344 = v0;
                    let a345 = 55u64;
                    nat_sub(a344, a345)
                } else {
                    let a346 = 96u64;
                    let a347 = v0;
                    if nat_lt(a346, a347) {
                        let a348 = v0;
                        let a349 = 103u64;
                        if nat_lt(a348, a349) {
                            let a350 = v0;
                            let a351 = 87u64;
                            nat_sub(a350, a351)
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

pub fn f68(v0: Bytes, v1: u64) -> R<u64> {
    let m354 = {
        let a352 = v0;
        let a353 = v1;
        index_bytes(a352, a353)
    };
    match m354 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a399 = {
                let a375 = {
                    let a363 = {
                        let a357 = {
                            let a355 = v3;
                            let a356 = 128u8;
                            fixed_u8::bit_and(a355, a356)
                        };
                        let a358 = 128u8;
                        if equal(a357, a358) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a364 = {
                        let a361 = {
                            let a359 = v3;
                            let a360 = 64u8;
                            fixed_u8::bit_and(a359, a360)
                        };
                        let a362 = 64u8;
                        if equal(a361, a362) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a363, a364)?
                };
                let a376 = {
                    let a373 = {
                        let a367 = {
                            let a365 = v3;
                            let a366 = 32u8;
                            fixed_u8::bit_and(a365, a366)
                        };
                        let a368 = 32u8;
                        if equal(a367, a368) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a374 = {
                        let a371 = {
                            let a369 = v3;
                            let a370 = 16u8;
                            fixed_u8::bit_and(a369, a370)
                        };
                        let a372 = 16u8;
                        if equal(a371, a372) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a373, a374)?
                };
                nat_add(a375, a376)?
            };
            let a400 = {
                let a397 = {
                    let a385 = {
                        let a379 = {
                            let a377 = v3;
                            let a378 = 8u8;
                            fixed_u8::bit_and(a377, a378)
                        };
                        let a380 = 8u8;
                        if equal(a379, a380) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a386 = {
                        let a383 = {
                            let a381 = v3;
                            let a382 = 4u8;
                            fixed_u8::bit_and(a381, a382)
                        };
                        let a384 = 4u8;
                        if equal(a383, a384) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a385, a386)?
                };
                let a398 = {
                    let a395 = {
                        let a389 = {
                            let a387 = v3;
                            let a388 = 2u8;
                            fixed_u8::bit_and(a387, a388)
                        };
                        let a390 = 2u8;
                        if equal(a389, a390) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a396 = {
                        let a393 = {
                            let a391 = v3;
                            let a392 = 1u8;
                            fixed_u8::bit_and(a391, a392)
                        };
                        let a394 = 1u8;
                        if equal(a393, a394) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a395, a396)?
                };
                nat_add(a397, a398)?
            };
            nat_add(a399, a400)
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt7 = f89(v0, v1)?;
    let a409 = f90(match v2.clone() {
        Adt7::C0(h401, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h401
        }
    });
    let a410 = {
        let a407 = f90(match v2.clone() {
            Adt7::C0(_, h402, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        });
        let a408 = {
            let a405 = f90(match v2.clone() {
                Adt7::C0(_, _, h403, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h403
                }
            });
            let a406 = f90(match v2 {
                Adt7::C0(_, _, _, h404, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h404
                }
            });
            append_bytes(a405, a406)
        };
        append_bytes(a407, a408)
    };
    Ok(append_bytes(a409, a410))
}

pub fn f70(v0: Bytes) -> R<Adt4> {
    let v1: u64 = {
        let a411 = v0.clone();
        length_bytes(a411)
    };
    let v2: u64 = {
        let a417 = {
            let a414 = {
                let a412 = v1;
                let a413 = 144u64;
                nat_add(a412, a413)?
            };
            let a415 = 128u64;
            let a416 = 0u64;
            nat_quot(a414, a415, a416)
        };
        let a418 = 128u64;
        nat_mul(a417, a418)?
    };
    let (_, h419) = f91(Fn16::F92(v0, v1, v2), {
        let a420 = v2;
        let a421 = 128u64;
        let a422 = 0u64;
        nat_quot(a420, a421, a422)
    }, (0u64, f93()))?;
    Ok(h419)
}

pub fn f71(v0: u64) -> Bytes {
    let a451 = f94(v0, 60u32);
    let a452 = {
        let a449 = f94(v0, 56u32);
        let a450 = {
            let a447 = f94(v0, 52u32);
            let a448 = {
                let a445 = f94(v0, 48u32);
                let a446 = {
                    let a443 = f94(v0, 44u32);
                    let a444 = {
                        let a441 = f94(v0, 40u32);
                        let a442 = {
                            let a439 = f94(v0, 36u32);
                            let a440 = {
                                let a437 = f94(v0, 32u32);
                                let a438 = {
                                    let a435 = f94(v0, 28u32);
                                    let a436 = {
                                        let a433 = f94(v0, 24u32);
                                        let a434 = {
                                            let a431 = f94(v0, 20u32);
                                            let a432 = {
                                                let a429 = f94(v0, 16u32);
                                                let a430 = {
                                                    let a427 = f94(v0, 12u32);
                                                    let a428 = {
                                                        let a425 = f94(v0, 8u32);
                                                        let a426 = {
                                                            let a423 = f94(v0, 4u32);
                                                            let a424 = f94(v0, 0u32);
                                                            append_bytes(a423, a424)
                                                        };
                                                        append_bytes(a425, a426)
                                                    };
                                                    append_bytes(a427, a428)
                                                };
                                                append_bytes(a429, a430)
                                            };
                                            append_bytes(a431, a432)
                                        };
                                        append_bytes(a433, a434)
                                    };
                                    append_bytes(a435, a436)
                                };
                                append_bytes(a437, a438)
                            };
                            append_bytes(a439, a440)
                        };
                        append_bytes(a441, a442)
                    };
                    append_bytes(a443, a444)
                };
                append_bytes(a445, a446)
            };
            append_bytes(a447, a448)
        };
        append_bytes(a449, a450)
    };
    append_bytes(a451, a452)
}

pub fn f72(v0: Bytes) -> R<Adt5> {
    let v1: u64 = {
        let a453 = v0.clone();
        length_bytes(a453)
    };
    let v2: u64 = {
        let a454 = v1;
        let a455 = 0u64;
        if nat_eq(a454, a455) {
            1u64
        } else {
            let a458 = {
                let a456 = v1;
                let a457 = 1023u64;
                nat_add(a456, a457)?
            };
            let a459 = 1024u64;
            let a460 = 0u64;
            nat_quot(a458, a459, a460)
        }
    };
    let v3: u64 = {
        let a461 = v2;
        let a462 = 1u64;
        nat_sub(a461, a462)
    };
    let m464 = {
        let (_, h463) = f95(v0.clone(), v1, v3)?;
        h463
    };
    match m464.uncons() {
        None => {
            f96(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h465) = f97(Fn17::F98, 64u64, (List::cons(v4, v5), f96(v0, v1, v3, false)?));
            Ok(h465)
        }
    }
}

pub fn f73(v0: u32) -> Bytes {
    let a478 = {
        let a466 = f99(v0, 4u32);
        let a467 = f99(v0, 0u32);
        append_bytes(a466, a467)
    };
    let a479 = {
        let a476 = {
            let a468 = f99(v0, 12u32);
            let a469 = f99(v0, 8u32);
            append_bytes(a468, a469)
        };
        let a477 = {
            let a474 = {
                let a470 = f99(v0, 20u32);
                let a471 = f99(v0, 16u32);
                append_bytes(a470, a471)
            };
            let a475 = {
                let a472 = f99(v0, 28u32);
                let a473 = f99(v0, 24u32);
                append_bytes(a472, a473)
            };
            append_bytes(a474, a475)
        };
        append_bytes(a476, a477)
    };
    append_bytes(a478, a479)
}

pub fn f74(v0: Bytes, v1: u64, v2: Adt1) -> R<Option<Adt1>> {
    if if match v2.clone() {
        Adt1::C0(_, _, h480, _) => {
            h480
        }
    } {
        true
    } else {
        let a483 = {
            let a481 = v0.clone();
            length_bytes(a481)
        };
        let a484 = match v2.clone() {
            Adt1::C0(h482, _, _, _) => {
                h482
            }
        };
        nat_le(a483, a484)
    } {
        Ok(None::<Adt1>)
    } else {
        let v3: (bool, (u64, u64)) = f100(v0.clone(), match v2.clone() {
            Adt1::C0(h485, _, _, _) => {
                h485
            }
        })?;
        let v4: u64 = {
            let (h486, _) = {
                let (_, h487) = v3;
                h487
            };
            h486
        };
        let v5: u64 = {
            let a488 = v4;
            let a489 = 8u64;
            let a490 = 0u64;
            nat_quot(a488, a489, a490)
        };
        let (h491, _) = v3;
        if if h491 {
            let a492 = 0u64;
            let a493 = v5;
            nat_lt(a492, a493)
        } else {
            false
        } {
            f101(v0, v1, v2, v5, {
                let a494 = v4;
                let a495 = 8u64;
                let a496 = 0u64;
                nat_rem(a494, a495, a496)
            }, {
                let (_, h497) = {
                    let (_, h498) = v3;
                    h498
                };
                h497
            })
        } else {
            Ok(f102(v2))
        }
    }
}

pub fn f75(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a500 = {
            let (h499, _) = v1.clone();
            h499
        };
        let a501 = 1u64;
        nat_add(a500, a501)?
    }, {
        let a506 = {
            let (_, h502) = v1;
            h502
        };
        let a507 = {
            let a504 = Bytes::lit(&[0x01]);
            let a505 = f64(f68(v0, {
                let (h503, _) = v1;
                h503
            })?);
            append_bytes(a504, a505)
        };
        append_bytes(a506, a507)
    }))
}

pub fn f76(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a509 = {
            let (h508, _) = v0.clone();
            h508
        };
        let a510 = 256u64;
        let a511 = 0u64;
        nat_quot(a509, a510, a511)
    }, {
        let a517 = f64({
            let a513 = {
                let (h512, _) = v0;
                h512
            };
            let a514 = 256u64;
            let a515 = 0u64;
            nat_rem(a513, a514, a515)
        });
        let a518 = {
            let (_, h516) = v0;
            h516
        };
        append_bytes(a517, a518)
    })
}

pub fn f77(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let v3: Adt8 = f103(Fn18::F104(v1.clone(), v0.clone()), Adt8::C0(false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt8::C0(h519, _) => {
            h519
        }
    } {
        f60(match v3 {
            Adt8::C0(_, h520) => {
                h520
            }
        })
    } else {
        f60(List::cons((v0, v1), match v3 {
            Adt8::C0(_, h521) => {
                h521
            }
        }))
    }
}

pub fn f78(v0: Fn13, p525: List<(Bytes, Bytes)>, p526: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v1, v2) = m524;
                let m522 = v2;
                match m522.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c523: Fn13 = v0.clone();
                            c523.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f80(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, List<Adt0>) = f3(v0)?;
    let v3: List<Adt0> = {
        let (_, h527) = v2;
        h527
    };
    let v4: List<Bytes> = f20(v3.clone(), 1u64);
    let v5: (u64, (bool, List<Adt9>)) = f105(Fn19::F106, (0u64, (true, List::<Adt9>::nil())), v4)?;
    let v6: List<Adt9> = f107({
        let (_, h528) = {
            let (_, h529) = v5.clone();
            h529
        };
        h528
    });
    let v7: Adt10 = f108(Fn20::F109(v6), {
        let (h530, _) = v5.clone();
        h530
    }, Adt10::C0(true, List::<u64>::nil(), List::<Adt9>::nil()));
    let v8: (bool, Bytes) = f110(v3.clone())?;
    let v9: (bool, Bytes) = f111(v3.clone(), 11u64)?;
    let v10: (bool, Bytes) = f111(v3.clone(), 12u64)?;
    let v11: (bool, Bytes) = f111(v3.clone(), 13u64)?;
    let v12: Adt3 = f112({
        let a539 = {
            let (_, h531) = v8;
            h531
        };
        let a540 = {
            let a537 = {
                let (_, h532) = v9;
                h532
            };
            let a538 = {
                let a535 = {
                    let (_, h533) = v10;
                    h533
                };
                let a536 = {
                    let (_, h534) = v11;
                    h534
                };
                append_bytes(a535, a536)
            };
            append_bytes(a537, a538)
        };
        append_bytes(a539, a540)
    });
    let v13: (bool, List<Adt3>) = f113(Fn21::F114(v1), (true, List::cons(f112({
        let a542 = f11(f6(f4(v3, 2u64)))?;
        let a543 = f10({
            let (h541, _) = v5;
            h541
        }, 4u64)?;
        append_bytes(a542, a543)
    }), List::<Adt3>::nil())), f107(match v7.clone() {
        Adt10::C0(_, _, h544) => {
            h544
        }
    }))?;
    Ok(({
        let a545 = v1;
        let a546 = 64u64;
        if if nat_le(a545, a546) {
            let (h547, _) = v2;
            h547
        } else {
            false
        } {
            let (h548, _) = {
                let (_, h549) = v5;
                h549
            };
            if if h548 {
                match v7 {
                    Adt10::C0(h550, _, _) => {
                        h550
                    }
                }
            } else {
                false
            } {
                let (h551, _) = v13;
                if h551 {
                    let (h552, _) = v8;
                    if h552 {
                        let (h553, _) = v9;
                        if h553 {
                            let (h554, _) = v10;
                            if h554 {
                                let (h555, _) = v11;
                                h555
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
    }, f83(List::cons(v12, {
        let (_, h556) = v13;
        h556
    }))))
}

pub fn f81(v0: Fn14, p560: List<Adt3>, p561: List<Adt3>) -> List<Adt3> {
    let m559 = (p560, p561);
    {
        let mut m559 = m559;
        loop {
            let step = {
                let (v1, v2) = m559;
                let m557 = v2;
                match m557.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c558: Fn14 = v0.clone();
                            c558.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f83(v0: List<Adt3>) -> List<Adt3> {
    f81(Fn14::F115, List::<Adt3>::nil(), v0)
}

pub fn f84(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m565 = {
        let a562 = v0;
        let a563 = v1;
        let a564 = v2;
        slice_bytes(a562, a563, a564)
    };
    match m565 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f85(v0: u32) -> Bytes {
    let a594 = f116({
        let a566 = f117(v0, 28u32);
        let a567 = 15u32;
        fixed_u32::bit_and(a566, a567)
    });
    let a595 = {
        let a592 = f116({
            let a568 = f117(v0, 24u32);
            let a569 = 15u32;
            fixed_u32::bit_and(a568, a569)
        });
        let a593 = {
            let a590 = f116({
                let a570 = f117(v0, 20u32);
                let a571 = 15u32;
                fixed_u32::bit_and(a570, a571)
            });
            let a591 = {
                let a588 = f116({
                    let a572 = f117(v0, 16u32);
                    let a573 = 15u32;
                    fixed_u32::bit_and(a572, a573)
                });
                let a589 = {
                    let a586 = f116({
                        let a574 = f117(v0, 12u32);
                        let a575 = 15u32;
                        fixed_u32::bit_and(a574, a575)
                    });
                    let a587 = {
                        let a584 = f116({
                            let a576 = f117(v0, 8u32);
                            let a577 = 15u32;
                            fixed_u32::bit_and(a576, a577)
                        });
                        let a585 = {
                            let a582 = f116({
                                let a578 = f117(v0, 4u32);
                                let a579 = 15u32;
                                fixed_u32::bit_and(a578, a579)
                            });
                            let a583 = f116({
                                let a580 = v0;
                                let a581 = 15u32;
                                fixed_u32::bit_and(a580, a581)
                            });
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
}

pub fn f86(v0: Fn15, p599: u64, p600: (u64, Adt6)) -> R<(u64, Adt6)> {
    let m598 = (p599, p600);
    {
        let mut m598 = m598;
        loop {
            let step = {
                let (v1, v2) = m598;
                let m596 = v1;
                if m596 == 0 {
                    Err::<(u64, (u64, Adt6)), R<(u64, Adt6)>>(Ok(v2))
                } else {
                    let v3 = m596 - 1;
                    Ok::<(u64, (u64, Adt6)), R<(u64, Adt6)>>((v3, {
                        let c597: Fn15 = v0.clone();
                        c597.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a602 = {
            let (h601, _) = v3.clone();
            h601
        };
        let a603 = 64u64;
        nat_add(a602, a603)?
    }, f118({
        let (_, h604) = v3;
        h604
    }, f119(v0, v1, v2, {
        let (h605, _) = v3;
        h605
    })?)?))
}

pub fn f88() -> Adt6 {
    Adt6::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f89(v0: Bytes, v1: u64) -> R<Adt7> {
    let v2: u64 = {
        let a606 = v0.clone();
        length_bytes(a606)
    };
    let v3: u64 = {
        let a612 = {
            let a610 = {
                let a607 = v2;
                let a608 = 136u64;
                let a609 = 0u64;
                nat_quot(a607, a608, a609)
            };
            let a611 = 1u64;
            nat_add(a610, a611)?
        };
        let a613 = 136u64;
        nat_mul(a612, a613)?
    };
    let (_, h614) = f120(Fn22::F121(v0, v2, v1, v3), {
        let a615 = v3;
        let a616 = 136u64;
        let a617 = 0u64;
        nat_quot(a615, a616, a617)
    }, (0u64, f122()))?;
    Ok(h614)
}

pub fn f90(v0: u64) -> Bytes {
    let a646 = {
        let a618 = f94(v0, 4u32);
        let a619 = f94(v0, 0u32);
        append_bytes(a618, a619)
    };
    let a647 = {
        let a644 = {
            let a620 = f94(v0, 12u32);
            let a621 = f94(v0, 8u32);
            append_bytes(a620, a621)
        };
        let a645 = {
            let a642 = {
                let a622 = f94(v0, 20u32);
                let a623 = f94(v0, 16u32);
                append_bytes(a622, a623)
            };
            let a643 = {
                let a640 = {
                    let a624 = f94(v0, 28u32);
                    let a625 = f94(v0, 24u32);
                    append_bytes(a624, a625)
                };
                let a641 = {
                    let a638 = {
                        let a626 = f94(v0, 36u32);
                        let a627 = f94(v0, 32u32);
                        append_bytes(a626, a627)
                    };
                    let a639 = {
                        let a636 = {
                            let a628 = f94(v0, 44u32);
                            let a629 = f94(v0, 40u32);
                            append_bytes(a628, a629)
                        };
                        let a637 = {
                            let a634 = {
                                let a630 = f94(v0, 52u32);
                                let a631 = f94(v0, 48u32);
                                append_bytes(a630, a631)
                            };
                            let a635 = {
                                let a632 = f94(v0, 60u32);
                                let a633 = f94(v0, 56u32);
                                append_bytes(a632, a633)
                            };
                            append_bytes(a634, a635)
                        };
                        append_bytes(a636, a637)
                    };
                    append_bytes(a638, a639)
                };
                append_bytes(a640, a641)
            };
            append_bytes(a642, a643)
        };
        append_bytes(a644, a645)
    };
    append_bytes(a646, a647)
}

pub fn f91(v0: Fn16, p651: u64, p652: (u64, Adt4)) -> R<(u64, Adt4)> {
    let m650 = (p651, p652);
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v1, v2) = m650;
                let m648 = v1;
                if m648 == 0 {
                    Err::<(u64, (u64, Adt4)), R<(u64, Adt4)>>(Ok(v2))
                } else {
                    let v3 = m648 - 1;
                    Ok::<(u64, (u64, Adt4)), R<(u64, Adt4)>>((v3, {
                        let c649: Fn16 = v0.clone();
                        c649.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt4)) -> R<(u64, Adt4)> {
    Ok(({
        let a654 = {
            let (h653, _) = v3.clone();
            h653
        };
        let a655 = 128u64;
        nat_add(a654, a655)?
    }, f123({
        let (_, h656) = v3;
        h656
    }, f124(v0, v1, v2, {
        let (h657, _) = v3;
        h657
    })?)?))
}

pub fn f93() -> Adt4 {
    Adt4::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f94(v0: u64, v1: u32) -> Bytes {
    f116(f125(f126(v0, v1)))
}

pub fn f95(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt5>)> {
    f127(Fn23::F128(v0, v1), v2, (0u64, List::<Adt5>::nil()))
}

pub fn f96(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt5> {
    let v4: u64 = {
        let a658 = 1024u64;
        let a659 = v2;
        nat_mul(a658, a659)?
    };
    let v5: u64 = f129(v1, v4);
    let v6: u64 = f130(v5)?;
    let v7: u64 = {
        let a660 = v6;
        let a661 = 1u64;
        nat_sub(a660, a661)
    };
    Ok(f131(f132(v0.clone(), v4, v2, v7)?, f133(v0, {
        let a664 = v4;
        let a665 = {
            let a662 = 64u64;
            let a663 = v7;
            nat_mul(a662, a663)?
        };
        nat_add(a664, a665)?
    })?, v2, f134({
        let a668 = v5;
        let a669 = {
            let a666 = 64u64;
            let a667 = v7;
            nat_mul(a666, a667)?
        };
        nat_sub(a668, a669)
    }), {
        let a672 = {
            let a670 = v7;
            let a671 = 0u64;
            if nat_eq(a670, a671) {
                3u32
            } else {
                2u32
            }
        };
        let a673 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a672, a673)
    }))
}

pub fn f97(v0: Fn17, p677: u64, p678: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m676 = (p677, p678);
    {
        let mut m676 = m676;
        loop {
            let step = {
                let (v1, v2) = m676;
                let m674 = v1;
                if m674 == 0 {
                    Err::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>(v2)
                } else {
                    let v3 = m674 - 1;
                    Ok::<(u64, (List<Adt5>, Adt5)), (List<Adt5>, Adt5)>((v3, {
                        let c675: Fn17 = v0.clone();
                        c675.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    f135(v0)
}

pub fn f99(v0: u32, v1: u32) -> Bytes {
    f116(f117(v0, v1))
}

pub fn f100(v0: Bytes, v1: u64) -> R<(bool, (u64, u64))> {
    let v2: Adt11 = {
        let (h679, _) = f136(Fn24::F137(v0), 12u64, Adt11::C0(v1, 0u64, 0u64, 1u64, true, false))?;
        h679
    };
    Ok((if match v2.clone() {
        Adt11::C0(_, _, _, _, h680, _) => {
            h680
        }
    } {
        match v2.clone() {
            Adt11::C0(_, _, _, _, _, h681) => {
                h681
            }
        }
    } else {
        false
    }, (match v2.clone() {
        Adt11::C0(_, h682, _, _, _, _) => {
            h682
        }
    }, match v2 {
        Adt11::C0(h683, _, _, _, _, _) => {
            h683
        }
    })))
}

pub fn f101(v0: Bytes, v1: u64, v2: Adt1, v3: u64, v4: u64, v5: u64) -> R<Option<Adt1>> {
    let a684 = v4;
    let a685 = 0u64;
    if nat_eq(a684, a685) {
        let v6: (bool, (u64, u64)) = f100(v0, v5)?;
        let (h686, _) = v6;
        if h686 {
            Ok(f138(v2, v1, v3, 0u64, {
                let (h687, _) = {
                    let (_, h688) = v6;
                    h688
                };
                h687
            }, Bytes::lit(&[]), {
                let (_, h689) = {
                    let (_, h690) = v6;
                    h690
                };
                h689
            }))
        } else {
            Ok(f102(v2))
        }
    } else {
        let a691 = v4;
        let a692 = 1u64;
        if nat_eq(a691, a692) {
            if f139(v0.clone(), v5, 8u64) {
                Ok(f138(v2, v1, v3, 1u64, f140(v0, v5, 8u64)?, Bytes::lit(&[]), {
                    let a693 = v5;
                    let a694 = 8u64;
                    nat_add(a693, a694)?
                }))
            } else {
                Ok(f102(v2))
            }
        } else {
            let a695 = v4;
            let a696 = 5u64;
            if nat_eq(a695, a696) {
                if f139(v0.clone(), v5, 4u64) {
                    Ok(f138(v2, v1, v3, 5u64, f140(v0, v5, 4u64)?, Bytes::lit(&[]), {
                        let a697 = v5;
                        let a698 = 4u64;
                        nat_add(a697, a698)?
                    }))
                } else {
                    Ok(f102(v2))
                }
            } else {
                let a699 = v4;
                let a700 = 2u64;
                if nat_eq(a699, a700) {
                    let v7: (bool, (u64, u64)) = f100(v0.clone(), v5)?;
                    let v8: u64 = {
                        let (h701, _) = {
                            let (_, h702) = v7;
                            h702
                        };
                        h701
                    };
                    let v9: u64 = {
                        let (_, h703) = {
                            let (_, h704) = v7;
                            h704
                        };
                        h703
                    };
                    let (h705, _) = v7;
                    if if h705 {
                        f139(v0.clone(), v9, v8)
                    } else {
                        false
                    } {
                        Ok(f138(v2, v1, v3, 2u64, 0u64, f84(v0, v9, v8), {
                            let a706 = v9;
                            let a707 = v8;
                            nat_add(a706, a707)?
                        }))
                    } else {
                        Ok(f102(v2))
                    }
                } else {
                    Ok(f102(v2))
                }
            }
        }
    }
}

pub fn f102(v0: Adt1) -> Option<Adt1> {
    Some(Adt1::C0(match v0 {
        Adt1::C0(h708, _, _, _) => {
            h708
        }
    }, false, true, List::<Adt0>::nil()))
}

pub fn f103(v0: Fn18, p712: Adt8, p713: List<(Bytes, Bytes)>) -> Adt8 {
    let m711 = (p712, p713);
    {
        let mut m711 = m711;
        loop {
            let step = {
                let (v1, v2) = m711;
                let m709 = v2;
                match m709.uncons() {
                    None => {
                        Err::<(Adt8, List<(Bytes, Bytes)>), Adt8>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt8, List<(Bytes, Bytes)>), Adt8>(({
                            let c710: Fn18 = v0.clone();
                            c710.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Bytes, v1: Bytes, v2: Adt8, v3: (Bytes, Bytes)) -> Adt8 {
    if match v2.clone() {
        Adt8::C0(h714, _) => {
            h714
        }
    } {
        Adt8::C0(true, List::cons(v3, match v2 {
            Adt8::C0(_, h715) => {
                h715
            }
        }))
    } else {
        if f141(v1.clone(), {
            let (h716, _) = v3.clone();
            h716
        }) {
            Adt8::C0(true, List::cons(v3, List::cons((v1, v0), match v2 {
                Adt8::C0(_, h717) => {
                    h717
                }
            })))
        } else {
            Adt8::C0(false, List::cons(v3, match v2 {
                Adt8::C0(_, h718) => {
                    h718
                }
            }))
        }
    }
}

pub fn f105(v0: Fn19, p722: (u64, (bool, List<Adt9>)), p723: List<Bytes>) -> R<(u64, (bool, List<Adt9>))> {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v1, v2) = m721;
                let m719 = v2;
                match m719.uncons() {
                    None => {
                        Err::<((u64, (bool, List<Adt9>)), List<Bytes>), R<(u64, (bool, List<Adt9>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (bool, List<Adt9>)), List<Bytes>), R<(u64, (bool, List<Adt9>))>>(({
                            let c720: Fn19 = v0.clone();
                            c720.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: (u64, (bool, List<Adt9>)), v1: Bytes) -> R<(u64, (bool, List<Adt9>))> {
    let v2: (bool, Adt9) = f142({
        let (h724, _) = v0.clone();
        h724
    }, v1)?;
    Ok(({
        let a726 = {
            let (h725, _) = v0.clone();
            h725
        };
        let a727 = 1u64;
        nat_add(a726, a727)?
    }, ({
        let (h728, _) = {
            let (_, h729) = v0.clone();
            h729
        };
        if h728 {
            let (h730, _) = v2;
            h730
        } else {
            false
        }
    }, List::cons({
        let (_, h731) = v2;
        h731
    }, {
        let (_, h732) = {
            let (_, h733) = v0;
            h733
        };
        h732
    }))))
}

pub fn f107(v0: List<Adt9>) -> List<Adt9> {
    f143(Fn25::F144, List::<Adt9>::nil(), v0)
}

pub fn f108(v0: Fn20, p737: u64, p738: Adt10) -> Adt10 {
    let m736 = (p737, p738);
    {
        let mut m736 = m736;
        loop {
            let step = {
                let (v1, v2) = m736;
                let m734 = v1;
                if m734 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m734 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c735: Fn20 = v0.clone();
                        c735.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt9>, v1: Adt10) -> Adt10 {
    f145(v0, v1)
}

pub fn f110(v0: List<Adt0>) -> R<(bool, Bytes)> {
    let v1: List<Bytes> = f20(v0, 5u64);
    let v2: (bool, Bytes) = f146(f29(v1.clone(), f34(v1.clone(), 8u64)?), 0u64)?;
    Ok(({
        let (h739, _) = v2;
        h739
    }, {
        let a741 = f10(f26(v1)?, 4u64)?;
        let a742 = {
            let (_, h740) = v2;
            h740
        };
        append_bytes(a741, a742)
    }))
}

pub fn f111(v0: List<Adt0>, v1: u64) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f20(v0, v1);
    f32(Fn5::F147, (true, f10(f26(v2.clone())?, 4u64)?), f29(v2.clone(), f34(v2, 1u64)?))
}

pub fn f112(v0: Bytes) -> Adt3 {
    Adt3::C0(false, v0, 0u64)
}

pub fn f113(v0: Fn21, p746: (bool, List<Adt3>), p747: List<Adt9>) -> R<(bool, List<Adt3>)> {
    let m745 = (p746, p747);
    {
        let mut m745 = m745;
        loop {
            let step = {
                let (v1, v2) = m745;
                let m743 = v2;
                match m743.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Adt9>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Adt9>), R<(bool, List<Adt3>)>>(({
                            let c744: Fn21 = v0.clone();
                            c744.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: u64, v1: (bool, List<Adt3>), v2: Adt9) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f148(v2, v0)?;
    Ok(({
        let (h748, _) = v1;
        if h748 {
            let (h749, _) = v3;
            h749
        } else {
            false
        }
    }, f81(Fn14::F149, {
        let (_, h750) = v1;
        h750
    }, f83({
        let (_, h751) = v3;
        h751
    }))))
}

pub fn f115(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f116(v0: u32) -> Bytes {
    let a754 = {
        let a752 = v0;
        let a753 = 8u32;
        fixed_u32::bit_and(a752, a753)
    };
    let a755 = 0u32;
    if equal(a754, a755) {
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
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a770 = {
                    let a768 = v0;
                    let a769 = 1u32;
                    fixed_u32::bit_and(a768, a769)
                };
                let a771 = 0u32;
                if equal(a770, a771) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
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
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a782 = {
                    let a780 = v0;
                    let a781 = 1u32;
                    fixed_u32::bit_and(a780, a781)
                };
                let a783 = 0u32;
                if equal(a782, a783) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a786 = {
            let a784 = v0;
            let a785 = 4u32;
            fixed_u32::bit_and(a784, a785)
        };
        let a787 = 0u32;
        if equal(a786, a787) {
            let a790 = {
                let a788 = v0;
                let a789 = 2u32;
                fixed_u32::bit_and(a788, a789)
            };
            let a791 = 0u32;
            if equal(a790, a791) {
                let a794 = {
                    let a792 = v0;
                    let a793 = 1u32;
                    fixed_u32::bit_and(a792, a793)
                };
                let a795 = 0u32;
                if equal(a794, a795) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a798 = {
                    let a796 = v0;
                    let a797 = 1u32;
                    fixed_u32::bit_and(a796, a797)
                };
                let a799 = 0u32;
                if equal(a798, a799) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a802 = {
                let a800 = v0;
                let a801 = 2u32;
                fixed_u32::bit_and(a800, a801)
            };
            let a803 = 0u32;
            if equal(a802, a803) {
                let a806 = {
                    let a804 = v0;
                    let a805 = 1u32;
                    fixed_u32::bit_and(a804, a805)
                };
                let a807 = 0u32;
                if equal(a806, a807) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a810 = {
                    let a808 = v0;
                    let a809 = 1u32;
                    fixed_u32::bit_and(a808, a809)
                };
                let a811 = 0u32;
                if equal(a810, a811) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f117(v0: u32, v1: u32) -> u32 {
    let m814 = {
        let a812 = v0;
        let a813 = v1;
        fixed_u32::shift_right(a812, a813)
    };
    match m814 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f118(v0: Adt6, v1: Adt12) -> R<Adt6> {
    let v2: Adt13 = f150(Fn26::F151, 64u64, Adt13::C0(match v0.clone() {
        Adt6::C0(h815, _, _, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt6::C0(_, h816, _, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h817, _, _, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h818, _, _, _, _) => {
            h818
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h819, _, _, _) => {
            h819
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h820, _, _) => {
            h820
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h821, _) => {
            h821
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h822) => {
            h822
        }
    }, std::rc::Rc::new(v1), 0u64))?;
    Ok(Adt6::C0(f152(match v0.clone() {
        Adt6::C0(h823, _, _, _, _, _, _, _) => {
            h823
        }
    }, match v2.clone() {
        Adt13::C0(h824, _, _, _, _, _, _, _, _, _) => {
            h824
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, h825, _, _, _, _, _, _) => {
            h825
        }
    }, match v2.clone() {
        Adt13::C0(_, h826, _, _, _, _, _, _, _, _) => {
            h826
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, _, h827, _, _, _, _, _) => {
            h827
        }
    }, match v2.clone() {
        Adt13::C0(_, _, h828, _, _, _, _, _, _, _) => {
            h828
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, _, _, h829, _, _, _, _) => {
            h829
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, h830, _, _, _, _, _, _) => {
            h830
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, _, _, _, h831, _, _, _) => {
            h831
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, h832, _, _, _, _, _) => {
            h832
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h833, _, _) => {
            h833
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, _, h834, _, _, _, _) => {
            h834
        }
    }), f152(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h835, _) => {
            h835
        }
    }, match v2.clone() {
        Adt13::C0(_, _, _, _, _, _, h836, _, _, _) => {
            h836
        }
    }), f152(match v0 {
        Adt6::C0(_, _, _, _, _, _, _, h837) => {
            h837
        }
    }, match v2 {
        Adt13::C0(_, _, _, _, _, _, _, h838, _, _) => {
            h838
        }
    })))
}

pub fn f119(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt12> {
    Ok(Adt12::C0(f153(v0.clone(), v1, v2, v3)?, f153(v0.clone(), v1, v2, {
        let a839 = v3;
        let a840 = 4u64;
        nat_add(a839, a840)?
    })?, f153(v0.clone(), v1, v2, {
        let a841 = v3;
        let a842 = 8u64;
        nat_add(a841, a842)?
    })?, f153(v0.clone(), v1, v2, {
        let a843 = v3;
        let a844 = 12u64;
        nat_add(a843, a844)?
    })?, f153(v0.clone(), v1, v2, {
        let a845 = v3;
        let a846 = 16u64;
        nat_add(a845, a846)?
    })?, f153(v0.clone(), v1, v2, {
        let a847 = v3;
        let a848 = 20u64;
        nat_add(a847, a848)?
    })?, f153(v0.clone(), v1, v2, {
        let a849 = v3;
        let a850 = 24u64;
        nat_add(a849, a850)?
    })?, f153(v0.clone(), v1, v2, {
        let a851 = v3;
        let a852 = 28u64;
        nat_add(a851, a852)?
    })?, f153(v0.clone(), v1, v2, {
        let a853 = v3;
        let a854 = 32u64;
        nat_add(a853, a854)?
    })?, f153(v0.clone(), v1, v2, {
        let a855 = v3;
        let a856 = 36u64;
        nat_add(a855, a856)?
    })?, f153(v0.clone(), v1, v2, {
        let a857 = v3;
        let a858 = 40u64;
        nat_add(a857, a858)?
    })?, f153(v0.clone(), v1, v2, {
        let a859 = v3;
        let a860 = 44u64;
        nat_add(a859, a860)?
    })?, f153(v0.clone(), v1, v2, {
        let a861 = v3;
        let a862 = 48u64;
        nat_add(a861, a862)?
    })?, f153(v0.clone(), v1, v2, {
        let a863 = v3;
        let a864 = 52u64;
        nat_add(a863, a864)?
    })?, f153(v0.clone(), v1, v2, {
        let a865 = v3;
        let a866 = 56u64;
        nat_add(a865, a866)?
    })?, f153(v0, v1, v2, {
        let a867 = v3;
        let a868 = 60u64;
        nat_add(a867, a868)?
    })?))
}

pub fn f120(v0: Fn22, p872: u64, p873: (u64, Adt7)) -> R<(u64, Adt7)> {
    let m871 = (p872, p873);
    {
        let mut m871 = m871;
        loop {
            let step = {
                let (v1, v2) = m871;
                let m869 = v1;
                if m869 == 0 {
                    Err::<(u64, (u64, Adt7)), R<(u64, Adt7)>>(Ok(v2))
                } else {
                    let v3 = m869 - 1;
                    Ok::<(u64, (u64, Adt7)), R<(u64, Adt7)>>((v3, {
                        let c870: Fn22 = v0.clone();
                        c870.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt7)) -> R<(u64, Adt7)> {
    Ok(({
        let a875 = {
            let (h874, _) = v4.clone();
            h874
        };
        let a876 = 136u64;
        nat_add(a875, a876)?
    }, f154(v0, v1, v3, v2, {
        let (_, h877) = v4;
        h877
    }, {
        let (h878, _) = v4;
        h878
    })?))
}

pub fn f122() -> Adt7 {
    Adt7::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f123(v0: Adt4, v1: Adt14) -> R<Adt4> {
    let v2: Adt15 = f155(Fn27::F156, 80u64, Adt15::C0(match v0.clone() {
        Adt4::C0(h879, _, _, _, _, _, _, _) => {
            h879
        }
    }, match v0.clone() {
        Adt4::C0(_, h880, _, _, _, _, _, _) => {
            h880
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h881, _, _, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h882, _, _, _, _) => {
            h882
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h883, _, _, _) => {
            h883
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h884, _, _) => {
            h884
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h885, _) => {
            h885
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h886) => {
            h886
        }
    }, std::rc::Rc::new(v1), 0u64))?;
    Ok(Adt4::C0(f157(match v0.clone() {
        Adt4::C0(h887, _, _, _, _, _, _, _) => {
            h887
        }
    }, match v2.clone() {
        Adt15::C0(h888, _, _, _, _, _, _, _, _, _) => {
            h888
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, h889, _, _, _, _, _, _) => {
            h889
        }
    }, match v2.clone() {
        Adt15::C0(_, h890, _, _, _, _, _, _, _, _) => {
            h890
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, _, h891, _, _, _, _, _) => {
            h891
        }
    }, match v2.clone() {
        Adt15::C0(_, _, h892, _, _, _, _, _, _, _) => {
            h892
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, _, _, h893, _, _, _, _) => {
            h893
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, h894, _, _, _, _, _, _) => {
            h894
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, _, _, _, h895, _, _, _) => {
            h895
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, h896, _, _, _, _, _) => {
            h896
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, _, _, _, _, h897, _, _) => {
            h897
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, _, h898, _, _, _, _) => {
            h898
        }
    }), f157(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h899, _) => {
            h899
        }
    }, match v2.clone() {
        Adt15::C0(_, _, _, _, _, _, h900, _, _, _) => {
            h900
        }
    }), f157(match v0 {
        Adt4::C0(_, _, _, _, _, _, _, h901) => {
            h901
        }
    }, match v2 {
        Adt15::C0(_, _, _, _, _, _, _, h902, _, _) => {
            h902
        }
    })))
}

pub fn f124(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt14> {
    Ok(Adt14::C0(f158(v0.clone(), v1, v2, {
        let a903 = v3;
        let a904 = 0u64;
        nat_add(a903, a904)?
    })?, f158(v0.clone(), v1, v2, {
        let a905 = v3;
        let a906 = 8u64;
        nat_add(a905, a906)?
    })?, f158(v0.clone(), v1, v2, {
        let a907 = v3;
        let a908 = 16u64;
        nat_add(a907, a908)?
    })?, f158(v0.clone(), v1, v2, {
        let a909 = v3;
        let a910 = 24u64;
        nat_add(a909, a910)?
    })?, f158(v0.clone(), v1, v2, {
        let a911 = v3;
        let a912 = 32u64;
        nat_add(a911, a912)?
    })?, f158(v0.clone(), v1, v2, {
        let a913 = v3;
        let a914 = 40u64;
        nat_add(a913, a914)?
    })?, f158(v0.clone(), v1, v2, {
        let a915 = v3;
        let a916 = 48u64;
        nat_add(a915, a916)?
    })?, f158(v0.clone(), v1, v2, {
        let a917 = v3;
        let a918 = 56u64;
        nat_add(a917, a918)?
    })?, f158(v0.clone(), v1, v2, {
        let a919 = v3;
        let a920 = 64u64;
        nat_add(a919, a920)?
    })?, f158(v0.clone(), v1, v2, {
        let a921 = v3;
        let a922 = 72u64;
        nat_add(a921, a922)?
    })?, f158(v0.clone(), v1, v2, {
        let a923 = v3;
        let a924 = 80u64;
        nat_add(a923, a924)?
    })?, f158(v0.clone(), v1, v2, {
        let a925 = v3;
        let a926 = 88u64;
        nat_add(a925, a926)?
    })?, f158(v0.clone(), v1, v2, {
        let a927 = v3;
        let a928 = 96u64;
        nat_add(a927, a928)?
    })?, f158(v0.clone(), v1, v2, {
        let a929 = v3;
        let a930 = 104u64;
        nat_add(a929, a930)?
    })?, f158(v0.clone(), v1, v2, {
        let a931 = v3;
        let a932 = 112u64;
        nat_add(a931, a932)?
    })?, f158(v0, v1, v2, {
        let a933 = v3;
        let a934 = 120u64;
        nat_add(a933, a934)?
    })?))
}

pub fn f125(v0: u64) -> u32 {
    let m938 = {
        let a937 = {
            let a935 = v0;
            let a936 = 4294967295u64;
            fixed_u64::bit_and(a935, a936)
        };
        fixed_u32::convert(i128::from(a937))
    };
    match m938 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f126(v0: u64, v1: u32) -> u64 {
    let m941 = {
        let a939 = v0;
        let a940 = v1;
        fixed_u64::shift_right(a939, a940)
    };
    match m941 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f127(v0: Fn23, p945: u64, p946: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    let m944 = (p945, p946);
    {
        let mut m944 = m944;
        loop {
            let step = {
                let (v1, v2) = m944;
                let m942 = v1;
                if m942 == 0 {
                    Err::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>(Ok(v2))
                } else {
                    let v3 = m942 - 1;
                    Ok::<(u64, (u64, List<Adt5>)), R<(u64, List<Adt5>)>>((v3, {
                        let c943: Fn23 = v0.clone();
                        c943.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Bytes, v1: u64, v2: (u64, List<Adt5>)) -> R<(u64, List<Adt5>)> {
    f159(v2.clone(), f96(v0, v1, {
        let (h947, _) = v2;
        h947
    }, false)?)
}

pub fn f129(v0: u64, v1: u64) -> u64 {
    let a950 = 1024u64;
    let a951 = {
        let a948 = v0;
        let a949 = v1;
        nat_sub(a948, a949)
    };
    if nat_lt(a950, a951) {
        1024u64
    } else {
        let a952 = v0;
        let a953 = v1;
        nat_sub(a952, a953)
    }
}

pub fn f130(v0: u64) -> R<u64> {
    let a954 = v0;
    let a955 = 0u64;
    if nat_eq(a954, a955) {
        Ok(1u64)
    } else {
        let a958 = {
            let a956 = v0;
            let a957 = 63u64;
            nat_add(a956, a957)?
        };
        let a959 = 64u64;
        let a960 = 0u64;
        Ok(nat_quot(a958, a959, a960))
    }
}

pub fn f131(v0: Adt5, v1: Adt16, v2: u64, v3: u32, v4: u32) -> Adt5 {
    let v5: Adt17 = {
        let (h961, _) = f160(Fn28::F161, 7u64, (Adt17::C0(match v0.clone() {
            Adt5::C0(h962, _, _, _, _, _, _, _) => {
                h962
            }
        }, match v0.clone() {
            Adt5::C0(_, h963, _, _, _, _, _, _) => {
                h963
            }
        }, match v0.clone() {
            Adt5::C0(_, _, h964, _, _, _, _, _) => {
                h964
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, h965, _, _, _, _) => {
                h965
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, h966, _, _, _) => {
                h966
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, h967, _, _) => {
                h967
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, h968, _) => {
                h968
            }
        }, match v0 {
            Adt5::C0(_, _, _, _, _, _, _, h969) => {
                h969
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f134({
            let a970 = v2;
            let a971 = 4294967296u64;
            let a972 = 0u64;
            nat_rem(a970, a971, a972)
        }), f134({
            let a973 = v2;
            let a974 = 4294967296u64;
            let a975 = 0u64;
            nat_quot(a973, a974, a975)
        }), v3, v4), v1));
        h961
    };
    Adt5::C0({
        let a978 = match v5.clone() {
            Adt17::C0(h976, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h976
            }
        };
        let a979 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, h977, _, _, _, _, _, _, _) => {
                h977
            }
        };
        fixed_u32::bit_xor(a978, a979)
    }, {
        let a982 = match v5.clone() {
            Adt17::C0(_, h980, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a983 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, h981, _, _, _, _, _, _) => {
                h981
            }
        };
        fixed_u32::bit_xor(a982, a983)
    }, {
        let a986 = match v5.clone() {
            Adt17::C0(_, _, h984, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h984
            }
        };
        let a987 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, h985, _, _, _, _, _) => {
                h985
            }
        };
        fixed_u32::bit_xor(a986, a987)
    }, {
        let a990 = match v5.clone() {
            Adt17::C0(_, _, _, h988, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h989, _, _, _, _) => {
                h989
            }
        };
        fixed_u32::bit_xor(a990, a991)
    }, {
        let a994 = match v5.clone() {
            Adt17::C0(_, _, _, _, h992, _, _, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a995 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h993, _, _, _) => {
                h993
            }
        };
        fixed_u32::bit_xor(a994, a995)
    }, {
        let a998 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, h996, _, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a999 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h997, _, _) => {
                h997
            }
        };
        fixed_u32::bit_xor(a998, a999)
    }, {
        let a1002 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, h1000, _, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        let a1003 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1001, _) => {
                h1001
            }
        };
        fixed_u32::bit_xor(a1002, a1003)
    }, {
        let a1006 = match v5.clone() {
            Adt17::C0(_, _, _, _, _, _, _, h1004, _, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = match v5 {
            Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1005) => {
                h1005
            }
        };
        fixed_u32::bit_xor(a1006, a1007)
    })
}

pub fn f132(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let (_, h1008) = f162(Fn29::F163(v2, v0, v1), v3, (0u64, f164()))?;
    Ok(h1008)
}

pub fn f133(v0: Bytes, v1: u64) -> R<Adt16> {
    Ok(Adt16::C0(f165(v0.clone(), {
        let a1009 = v1;
        let a1010 = 0u64;
        nat_add(a1009, a1010)?
    })?, f165(v0.clone(), {
        let a1011 = v1;
        let a1012 = 4u64;
        nat_add(a1011, a1012)?
    })?, f165(v0.clone(), {
        let a1013 = v1;
        let a1014 = 8u64;
        nat_add(a1013, a1014)?
    })?, f165(v0.clone(), {
        let a1015 = v1;
        let a1016 = 12u64;
        nat_add(a1015, a1016)?
    })?, f165(v0.clone(), {
        let a1017 = v1;
        let a1018 = 16u64;
        nat_add(a1017, a1018)?
    })?, f165(v0.clone(), {
        let a1019 = v1;
        let a1020 = 20u64;
        nat_add(a1019, a1020)?
    })?, f165(v0.clone(), {
        let a1021 = v1;
        let a1022 = 24u64;
        nat_add(a1021, a1022)?
    })?, f165(v0.clone(), {
        let a1023 = v1;
        let a1024 = 28u64;
        nat_add(a1023, a1024)?
    })?, f165(v0.clone(), {
        let a1025 = v1;
        let a1026 = 32u64;
        nat_add(a1025, a1026)?
    })?, f165(v0.clone(), {
        let a1027 = v1;
        let a1028 = 36u64;
        nat_add(a1027, a1028)?
    })?, f165(v0.clone(), {
        let a1029 = v1;
        let a1030 = 40u64;
        nat_add(a1029, a1030)?
    })?, f165(v0.clone(), {
        let a1031 = v1;
        let a1032 = 44u64;
        nat_add(a1031, a1032)?
    })?, f165(v0.clone(), {
        let a1033 = v1;
        let a1034 = 48u64;
        nat_add(a1033, a1034)?
    })?, f165(v0.clone(), {
        let a1035 = v1;
        let a1036 = 52u64;
        nat_add(a1035, a1036)?
    })?, f165(v0.clone(), {
        let a1037 = v1;
        let a1038 = 56u64;
        nat_add(a1037, a1038)?
    })?, f165(v0, {
        let a1039 = v1;
        let a1040 = 60u64;
        nat_add(a1039, a1040)?
    })?))
}

pub fn f134(v0: u64) -> u32 {
    let (_, h1041) = {
        let (_, h1042) = f166(Fn30::F167, 32u64, (v0, (1u32, 0u32)));
        h1042
    };
    h1041
}

pub fn f135(v0: (List<Adt5>, Adt5)) -> (List<Adt5>, Adt5) {
    let m1044 = {
        let (h1043, _) = v0.clone();
        h1043
    };
    match m1044.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f168(v1, {
                let (_, h1045) = v0;
                h1045
            }, {
                let m1046 = v2;
                match m1046.uncons() {
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

pub fn f136(v0: Fn24, p1051: u64, p1052: Adt11) -> R<(Adt11, bool)> {
    let m1050 = (p1051, p1052);
    {
        let mut m1050 = m1050;
        loop {
            let step = {
                let (v1, v2) = m1050;
                let m1047 = v1;
                if m1047 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1047 - 1;
                    let m1049 = {
                        let c1048: Fn24 = v0.clone();
                        c1048.apply(v2.clone())?
                    };
                    match m1049 {
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
                Ok(next) => { m1050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes, v1: Adt11) -> R<Option<Adt11>> {
    f169(v0, v1)
}

pub fn f138(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64, v5: Bytes, v6: u64) -> Option<Adt1> {
    Some(Adt1::C0(v6, true, {
        let a1053 = v2;
        let a1054 = v1;
        nat_eq(a1053, a1054)
    }, List::cons(Adt0::C0(v2, v3, v4, v5), match v0 {
        Adt1::C0(_, _, _, h1055) => {
            h1055
        }
    })))
}

pub fn f139(v0: Bytes, v1: u64, v2: u64) -> bool {
    let v3: u64 = {
        let a1056 = v0;
        length_bytes(a1056)
    };
    let a1057 = v1;
    let a1058 = v3;
    if nat_le(a1057, a1058) {
        let a1061 = v2;
        let a1062 = {
            let a1059 = v3;
            let a1060 = v1;
            nat_sub(a1059, a1060)
        };
        nat_le(a1061, a1062)
    } else {
        false
    }
}

pub fn f140(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h1063) = f170(Fn31::F171(v1, v0), v2, (v2, 0u64))?;
    Ok(h1063)
}

pub fn f141(v0: Bytes, v1: Bytes) -> bool {
    let a1068 = {
        let a1064 = v0;
        let a1065 = v1;
        compare_bytes(a1064, a1065)
    };
    let a1069 = {
        let a1066 = Bytes::lit(&[]);
        let a1067 = Bytes::lit(&[0x01]);
        compare_bytes(a1066, a1067)
    };
    equal(a1068, a1069)
}

pub fn f142(v0: u64, v1: Bytes) -> R<(bool, Adt9)> {
    let v2: (bool, List<Adt0>) = f3(v1.clone())?;
    let v3: List<Adt0> = {
        let (_, h1070) = v2;
        h1070
    };
    Ok(({
        let (h1071, _) = v2;
        h1071
    }, Adt9::C0(v0, v1, {
        let a1074 = f52(f6(f4(v3.clone(), 3u64)))?;
        let a1075 = {
            let a1072 = f52(f6(f4(v3.clone(), 4u64)))?;
            let a1073 = f52(f6(f4(v3.clone(), 7u64)))?;
            append_bytes(a1072, a1073)
        };
        append_bytes(a1074, a1075)
    }, f20(v3.clone(), 1u64), f20(v3, 2u64))))
}

pub fn f143(v0: Fn25, p1079: List<Adt9>, p1080: List<Adt9>) -> List<Adt9> {
    let m1078 = (p1079, p1080);
    {
        let mut m1078 = m1078;
        loop {
            let step = {
                let (v1, v2) = m1078;
                let m1076 = v2;
                match m1076.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c1077: Fn25 = v0.clone();
                            c1077.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f145(v0: List<Adt9>, v1: Adt10) -> Adt10 {
    if match v1.clone() {
        Adt10::C0(h1081, _, _) => {
            h1081
        }
    } {
        let v2: Adt18 = f172(Fn32::F173(v0.clone(), std::rc::Rc::new(v1.clone())), Adt18::C0(false, std::rc::Rc::new(Adt9::C0(0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), List::<Bytes>::nil()))), v0);
        if match v2.clone() {
            Adt18::C0(h1082, _) => {
                h1082
            }
        } {
            Adt10::C0(true, List::cons({
                let m1085 = match v2.clone() {
                    Adt18::C0(_, h1084) => {
                        (*h1084).clone()
                    }
                };
                match m1085 {
                    Adt9::C0(h1083, _, _, _, _) => {
                        h1083
                    }
                }
            }, match v1.clone() {
                Adt10::C0(_, h1086, _) => {
                    h1086
                }
            }), List::cons(match v2 {
                Adt18::C0(_, h1087) => {
                    (*h1087).clone()
                }
            }, match v1 {
                Adt10::C0(_, _, h1088) => {
                    h1088
                }
            }))
        } else {
            Adt10::C0(false, List::<u64>::nil(), List::<Adt9>::nil())
        }
    } else {
        v1
    }
}

pub fn f146(v0: List<Bytes>, v1: u64) -> R<(bool, Bytes)> {
    f32(Fn5::F174(v1), (true, Bytes::lit(&[])), v0)
}

pub fn f147(v0: (bool, Bytes), v1: Bytes) -> R<(bool, Bytes)> {
    let v2: (bool, Adt0) = f51(v1.clone(), 1u64)?;
    let v3: (bool, Adt0) = f51(v1, 2u64)?;
    let v4: (bool, Bytes) = f175(f6({
        let (_, h1089) = v3;
        h1089
    }))?;
    Ok(({
        let (h1090, _) = v0;
        if h1090 {
            let (h1091, _) = v2;
            if h1091 {
                let (h1092, _) = v3;
                if h1092 {
                    let (h1093, _) = v4;
                    h1093
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
        let a1099 = {
            let (_, h1094) = v0;
            h1094
        };
        let a1100 = {
            let a1097 = f11(f6({
                let (_, h1095) = v2;
                h1095
            }))?;
            let a1098 = {
                let (_, h1096) = v4;
                h1096
            };
            append_bytes(a1097, a1098)
        };
        append_bytes(a1099, a1100)
    }))
}

pub fn f148(v0: Adt9, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: List<Adt0> = {
        let (_, h1101) = f3(match v0 {
            Adt9::C0(_, h1102, _, _, _) => {
                h1102
            }
        })?;
        h1101
    };
    let v3: List<Bytes> = f20(v2.clone(), 5u64);
    let v4: Bytes = {
        let a1117 = f11(f6(f4(v2.clone(), 3u64)))?;
        let a1118 = {
            let a1115 = f11(f6(f4(v2.clone(), 4u64)))?;
            let a1116 = {
                let a1113 = f11(f6(f4(v2.clone(), 7u64)))?;
                let a1114 = {
                    let a1111 = f11(f6(f4(v2.clone(), 8u64)))?;
                    let a1112 = {
                        let a1109 = f10(f176(v2.clone(), 1u64)?, 4u64)?;
                        let a1110 = {
                            let a1107 = f177(f20(v2.clone(), 1u64))?;
                            let a1108 = {
                                let a1105 = f10(f176(v2.clone(), 2u64)?, 4u64)?;
                                let a1106 = {
                                    let a1103 = f177(f20(v2, 2u64))?;
                                    let a1104 = f10(f26(v3.clone())?, 4u64)?;
                                    append_bytes(a1103, a1104)
                                };
                                append_bytes(a1105, a1106)
                            };
                            append_bytes(a1107, a1108)
                        };
                        append_bytes(a1109, a1110)
                    };
                    append_bytes(a1111, a1112)
                };
                append_bytes(a1113, a1114)
            };
            append_bytes(a1115, a1116)
        };
        append_bytes(a1117, a1118)
    };
    f178(Fn33::F179(v1), (true, List::cons(f112(v4), List::<Adt3>::nil())), f29(v3.clone(), f34(v3, 1u64)?))
}

pub fn f149(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f150(v0: Fn26, p1122: u64, p1123: Adt13) -> R<Adt13> {
    let m1121 = (p1122, p1123);
    {
        let mut m1121 = m1121;
        loop {
            let step = {
                let (v1, v2) = m1121;
                let m1119 = v1;
                if m1119 == 0 {
                    Err::<(u64, Adt13), R<Adt13>>(Ok(v2))
                } else {
                    let v3 = m1119 - 1;
                    Ok::<(u64, Adt13), R<Adt13>>((v3, {
                        let c1120: Fn26 = v0.clone();
                        c1120.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Adt13) -> R<Adt13> {
    f180(v0)
}

pub fn f152(v0: u32, v1: u32) -> u32 {
    let m1126 = {
        let a1124 = f181(v0);
        let a1125 = f181(v1);
        fixed_u64::checked_add(a1124, a1125)
    };
    match m1126 {
        None => {
            0u32
        }
        Some(v2) => {
            f125(v2)
        }
    }
}

pub fn f153(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1129 = {
        let a1127 = v3;
        let a1128 = 8u64;
        nat_add(a1127, a1128)?
    };
    let a1130 = v2;
    if nat_eq(a1129, a1130) {
        Ok(f134({
            let a1133 = {
                let a1131 = v1;
                let a1132 = 8u64;
                nat_mul(a1131, a1132)?
            };
            let a1134 = 4294967296u64;
            let a1135 = 0u64;
            nat_quot(a1133, a1134, a1135)
        }))
    } else {
        let a1138 = {
            let a1136 = v3;
            let a1137 = 4u64;
            nat_add(a1136, a1137)?
        };
        let a1139 = v2;
        if nat_eq(a1138, a1139) {
            Ok(f134({
                let a1142 = {
                    let a1140 = v1;
                    let a1141 = 8u64;
                    nat_mul(a1140, a1141)?
                };
                let a1143 = 4294967296u64;
                let a1144 = 0u64;
                nat_rem(a1142, a1143, a1144)
            }))
        } else {
            Ok(f182(f183(v0.clone(), v1, v3), f183(v0.clone(), v1, {
                let a1145 = v3;
                let a1146 = 1u64;
                nat_add(a1145, a1146)?
            }), f183(v0.clone(), v1, {
                let a1147 = v3;
                let a1148 = 2u64;
                nat_add(a1147, a1148)?
            }), f183(v0, v1, {
                let a1149 = v3;
                let a1150 = 3u64;
                nat_add(a1149, a1150)?
            })))
        }
    }
}

pub fn f154(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt7, v5: u64) -> R<Adt7> {
    f184(Adt7::C0({
        let a1154 = match v4.clone() {
            Adt7::C0(h1151, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1151
            }
        };
        let a1155 = f185(v0.clone(), v1, v2, v3, {
            let a1152 = v5;
            let a1153 = 0u64;
            nat_add(a1152, a1153)?
        })?;
        fixed_u64::bit_xor(a1154, a1155)
    }, {
        let a1159 = match v4.clone() {
            Adt7::C0(_, h1156, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1156
            }
        };
        let a1160 = f185(v0.clone(), v1, v2, v3, {
            let a1157 = v5;
            let a1158 = 8u64;
            nat_add(a1157, a1158)?
        })?;
        fixed_u64::bit_xor(a1159, a1160)
    }, {
        let a1164 = match v4.clone() {
            Adt7::C0(_, _, h1161, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1161
            }
        };
        let a1165 = f185(v0.clone(), v1, v2, v3, {
            let a1162 = v5;
            let a1163 = 16u64;
            nat_add(a1162, a1163)?
        })?;
        fixed_u64::bit_xor(a1164, a1165)
    }, {
        let a1169 = match v4.clone() {
            Adt7::C0(_, _, _, h1166, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1166
            }
        };
        let a1170 = f185(v0.clone(), v1, v2, v3, {
            let a1167 = v5;
            let a1168 = 24u64;
            nat_add(a1167, a1168)?
        })?;
        fixed_u64::bit_xor(a1169, a1170)
    }, {
        let a1174 = match v4.clone() {
            Adt7::C0(_, _, _, _, h1171, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1171
            }
        };
        let a1175 = f185(v0.clone(), v1, v2, v3, {
            let a1172 = v5;
            let a1173 = 32u64;
            nat_add(a1172, a1173)?
        })?;
        fixed_u64::bit_xor(a1174, a1175)
    }, {
        let a1179 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, h1176, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1176
            }
        };
        let a1180 = f185(v0.clone(), v1, v2, v3, {
            let a1177 = v5;
            let a1178 = 40u64;
            nat_add(a1177, a1178)?
        })?;
        fixed_u64::bit_xor(a1179, a1180)
    }, {
        let a1184 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, h1181, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1185 = f185(v0.clone(), v1, v2, v3, {
            let a1182 = v5;
            let a1183 = 48u64;
            nat_add(a1182, a1183)?
        })?;
        fixed_u64::bit_xor(a1184, a1185)
    }, {
        let a1189 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h1186, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1186
            }
        };
        let a1190 = f185(v0.clone(), v1, v2, v3, {
            let a1187 = v5;
            let a1188 = 56u64;
            nat_add(a1187, a1188)?
        })?;
        fixed_u64::bit_xor(a1189, a1190)
    }, {
        let a1194 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h1191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1191
            }
        };
        let a1195 = f185(v0.clone(), v1, v2, v3, {
            let a1192 = v5;
            let a1193 = 64u64;
            nat_add(a1192, a1193)?
        })?;
        fixed_u64::bit_xor(a1194, a1195)
    }, {
        let a1199 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1196, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1196
            }
        };
        let a1200 = f185(v0.clone(), v1, v2, v3, {
            let a1197 = v5;
            let a1198 = 72u64;
            nat_add(a1197, a1198)?
        })?;
        fixed_u64::bit_xor(a1199, a1200)
    }, {
        let a1204 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h1201, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1205 = f185(v0.clone(), v1, v2, v3, {
            let a1202 = v5;
            let a1203 = 80u64;
            nat_add(a1202, a1203)?
        })?;
        fixed_u64::bit_xor(a1204, a1205)
    }, {
        let a1209 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h1206, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1210 = f185(v0.clone(), v1, v2, v3, {
            let a1207 = v5;
            let a1208 = 88u64;
            nat_add(a1207, a1208)?
        })?;
        fixed_u64::bit_xor(a1209, a1210)
    }, {
        let a1214 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1211, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1211
            }
        };
        let a1215 = f185(v0.clone(), v1, v2, v3, {
            let a1212 = v5;
            let a1213 = 96u64;
            nat_add(a1212, a1213)?
        })?;
        fixed_u64::bit_xor(a1214, a1215)
    }, {
        let a1219 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1216, _, _, _, _, _, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1220 = f185(v0.clone(), v1, v2, v3, {
            let a1217 = v5;
            let a1218 = 104u64;
            nat_add(a1217, a1218)?
        })?;
        fixed_u64::bit_xor(a1219, a1220)
    }, {
        let a1224 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1221, _, _, _, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1225 = f185(v0.clone(), v1, v2, v3, {
            let a1222 = v5;
            let a1223 = 112u64;
            nat_add(a1222, a1223)?
        })?;
        fixed_u64::bit_xor(a1224, a1225)
    }, {
        let a1229 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1226, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1230 = f185(v0.clone(), v1, v2, v3, {
            let a1227 = v5;
            let a1228 = 120u64;
            nat_add(a1227, a1228)?
        })?;
        fixed_u64::bit_xor(a1229, a1230)
    }, {
        let a1234 = match v4.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1231, _, _, _, _, _, _, _, _) => {
                h1231
            }
        };
        let a1235 = f185(v0, v1, v2, v3, {
            let a1232 = v5;
            let a1233 = 128u64;
            nat_add(a1232, a1233)?
        })?;
        fixed_u64::bit_xor(a1234, a1235)
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1236, _, _, _, _, _, _, _) => {
            h1236
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1237, _, _, _, _, _, _) => {
            h1237
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1238, _, _, _, _, _) => {
            h1238
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1239, _, _, _, _) => {
            h1239
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1240, _, _, _) => {
            h1240
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1241, _, _) => {
            h1241
        }
    }, match v4.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1242, _) => {
            h1242
        }
    }, match v4 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1243) => {
            h1243
        }
    }))
}

pub fn f155(v0: Fn27, p1247: u64, p1248: Adt15) -> R<Adt15> {
    let m1246 = (p1247, p1248);
    {
        let mut m1246 = m1246;
        loop {
            let step = {
                let (v1, v2) = m1246;
                let m1244 = v1;
                if m1244 == 0 {
                    Err::<(u64, Adt15), R<Adt15>>(Ok(v2))
                } else {
                    let v3 = m1244 - 1;
                    Ok::<(u64, Adt15), R<Adt15>>((v3, {
                        let c1245: Fn27 = v0.clone();
                        c1245.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt15) -> R<Adt15> {
    f186(v0)
}

pub fn f157(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f187({
        let a1249 = v0;
        let a1250 = v2;
        fixed_u64::bit_and(a1249, a1250)
    }, {
        let a1251 = v1;
        let a1252 = v2;
        fixed_u64::bit_and(a1251, a1252)
    });
    let v4: u64 = f187(f187(f126(v0, 32u32), f126(v1, 32u32)), f126(v3, 32u32));
    let a1257 = f188({
        let a1253 = v4;
        let a1254 = v2;
        fixed_u64::bit_and(a1253, a1254)
    }, 32u32);
    let a1258 = {
        let a1255 = v3;
        let a1256 = v2;
        fixed_u64::bit_and(a1255, a1256)
    };
    fixed_u64::bit_or(a1257, a1258)
}

pub fn f158(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1261 = {
        let a1259 = v3;
        let a1260 = 16u64;
        nat_add(a1259, a1260)?
    };
    let a1262 = v2;
    if nat_eq(a1261, a1262) {
        Ok(0u64)
    } else {
        let a1265 = {
            let a1263 = v3;
            let a1264 = 8u64;
            nat_add(a1263, a1264)?
        };
        let a1266 = v2;
        if nat_eq(a1265, a1266) {
            Ok(f189({
                let a1267 = v1;
                let a1268 = 8u64;
                nat_mul(a1267, a1268)?
            }))
        } else {
            Ok(f190(f191(v0.clone(), v1, {
                let a1269 = v3;
                let a1270 = 0u64;
                nat_add(a1269, a1270)?
            }), f191(v0.clone(), v1, {
                let a1271 = v3;
                let a1272 = 1u64;
                nat_add(a1271, a1272)?
            }), f191(v0.clone(), v1, {
                let a1273 = v3;
                let a1274 = 2u64;
                nat_add(a1273, a1274)?
            }), f191(v0.clone(), v1, {
                let a1275 = v3;
                let a1276 = 3u64;
                nat_add(a1275, a1276)?
            }), f191(v0.clone(), v1, {
                let a1277 = v3;
                let a1278 = 4u64;
                nat_add(a1277, a1278)?
            }), f191(v0.clone(), v1, {
                let a1279 = v3;
                let a1280 = 5u64;
                nat_add(a1279, a1280)?
            }), f191(v0.clone(), v1, {
                let a1281 = v3;
                let a1282 = 6u64;
                nat_add(a1281, a1282)?
            }), f191(v0, v1, {
                let a1283 = v3;
                let a1284 = 7u64;
                nat_add(a1283, a1284)?
            })))
        }
    }
}

pub fn f159(v0: (u64, List<Adt5>), v1: Adt5) -> R<(u64, List<Adt5>)> {
    let v2: Adt19 = f192(Fn34::F193, 40u64, Adt19::C0(std::rc::Rc::new(v1), {
        let a1286 = {
            let (h1285, _) = v0.clone();
            h1285
        };
        let a1287 = 1u64;
        nat_add(a1286, a1287)?
    }, {
        let (_, h1288) = v0;
        h1288
    }));
    Ok(({
        let a1290 = {
            let (h1289, _) = v0;
            h1289
        };
        let a1291 = 1u64;
        nat_add(a1290, a1291)?
    }, List::cons(match v2.clone() {
        Adt19::C0(h1292, _, _) => {
            (*h1292).clone()
        }
    }, match v2 {
        Adt19::C0(_, _, h1293) => {
            h1293
        }
    })))
}

pub fn f160(v0: Fn28, p1297: u64, p1298: (Adt17, Adt16)) -> (Adt17, Adt16) {
    let m1296 = (p1297, p1298);
    {
        let mut m1296 = m1296;
        loop {
            let step = {
                let (v1, v2) = m1296;
                let m1294 = v1;
                if m1294 == 0 {
                    Err::<(u64, (Adt17, Adt16)), (Adt17, Adt16)>(v2)
                } else {
                    let v3 = m1294 - 1;
                    Ok::<(u64, (Adt17, Adt16)), (Adt17, Adt16)>((v3, {
                        let c1295: Fn28 = v0.clone();
                        c1295.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: (Adt17, Adt16)) -> (Adt17, Adt16) {
    f194(v0)
}

pub fn f162(v0: Fn29, p1302: u64, p1303: (u64, Adt5)) -> R<(u64, Adt5)> {
    let m1301 = (p1302, p1303);
    {
        let mut m1301 = m1301;
        loop {
            let step = {
                let (v1, v2) = m1301;
                let m1299 = v1;
                if m1299 == 0 {
                    Err::<(u64, (u64, Adt5)), R<(u64, Adt5)>>(Ok(v2))
                } else {
                    let v3 = m1299 - 1;
                    Ok::<(u64, (u64, Adt5)), R<(u64, Adt5)>>((v3, {
                        let c1300: Fn29 = v0.clone();
                        c1300.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt5)) -> R<(u64, Adt5)> {
    Ok(({
        let a1305 = {
            let (h1304, _) = v3.clone();
            h1304
        };
        let a1306 = 1u64;
        nat_add(a1305, a1306)?
    }, f131({
        let (_, h1307) = v3.clone();
        h1307
    }, f133(v1, {
        let a1311 = v2;
        let a1312 = {
            let a1309 = 64u64;
            let a1310 = {
                let (h1308, _) = v3.clone();
                h1308
            };
            nat_mul(a1309, a1310)?
        };
        nat_add(a1311, a1312)?
    })?, v0, 64u32, {
        let a1314 = {
            let (h1313, _) = v3;
            h1313
        };
        let a1315 = 0u64;
        if nat_eq(a1314, a1315) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f164() -> Adt5 {
    Adt5::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f165(v0: Bytes, v1: u64) -> R<u32> {
    let a1326 = f195(v0.clone(), v1);
    let a1327 = {
        let a1324 = f196(f195(v0.clone(), {
            let a1316 = v1;
            let a1317 = 1u64;
            nat_add(a1316, a1317)?
        }), 8u32);
        let a1325 = {
            let a1322 = f196(f195(v0.clone(), {
                let a1318 = v1;
                let a1319 = 2u64;
                nat_add(a1318, a1319)?
            }), 16u32);
            let a1323 = f196(f195(v0, {
                let a1320 = v1;
                let a1321 = 3u64;
                nat_add(a1320, a1321)?
            }), 24u32);
            fixed_u32::bit_or(a1322, a1323)
        };
        fixed_u32::bit_or(a1324, a1325)
    };
    Ok(fixed_u32::bit_or(a1326, a1327))
}

pub fn f166(v0: Fn30, p1331: u64, p1332: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1330 = (p1331, p1332);
    {
        let mut m1330 = m1330;
        loop {
            let step = {
                let (v1, v2) = m1330;
                let m1328 = v1;
                if m1328 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1328 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1329: Fn30 = v0.clone();
                        c1329.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1333, _) = v0;
        h1333
    };
    let v2: u32 = {
        let (h1334, _) = {
            let (_, h1335) = v0;
            h1335
        };
        h1334
    };
    let v3: u32 = {
        let (_, h1336) = {
            let (_, h1337) = v0;
            h1337
        };
        h1336
    };
    ({
        let a1338 = v1;
        let a1339 = 2u64;
        let a1340 = 0u64;
        nat_quot(a1338, a1339, a1340)
    }, (f196(v2, 1u32), {
        let a1344 = {
            let a1341 = v1;
            let a1342 = 2u64;
            let a1343 = 0u64;
            nat_rem(a1341, a1342, a1343)
        };
        let a1345 = 1u64;
        if nat_eq(a1344, a1345) {
            let a1346 = v3;
            let a1347 = v2;
            fixed_u32::bit_or(a1346, a1347)
        } else {
            v3
        }
    }))
}

pub fn f168(v0: Adt5, v1: Adt5, v2: bool) -> Adt5 {
    f131(f164(), f197(v0, v1), 0u64, 64u32, {
        let a1348 = 4u32;
        let a1349 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a1348, a1349)
    })
}

pub fn f169(v0: Bytes, v1: Adt11) -> R<Option<Adt11>> {
    if match v1.clone() {
        Adt11::C0(_, _, _, _, _, h1350) => {
            h1350
        }
    } {
        Ok(None::<Adt11>)
    } else {
        let v2: u64 = match v1.clone() {
            Adt11::C0(h1351, _, _, _, _, _) => {
                h1351
            }
        };
        let a1353 = {
            let a1352 = v0.clone();
            length_bytes(a1352)
        };
        let a1354 = v2;
        if if nat_le(a1353, a1354) {
            true
        } else {
            let a1356 = 70u64;
            let a1357 = match v1.clone() {
                Adt11::C0(_, _, h1355, _, _, _) => {
                    h1355
                }
            };
            nat_le(a1356, a1357)
        } {
            Ok(Some(Adt11::C0(v2, 0u64, 0u64, 1u64, false, true)))
        } else {
            let v3: u64 = f68(v0, v2)?;
            let v4: u64 = {
                let a1358 = v3;
                let a1359 = 128u64;
                let a1360 = 0u64;
                nat_rem(a1358, a1359, a1360)
            };
            let v5: u64 = {
                let a1362 = match v1.clone() {
                    Adt11::C0(_, _, h1361, _, _, _) => {
                        h1361
                    }
                };
                let a1363 = 63u64;
                if nat_eq(a1362, a1363) {
                    let a1368 = {
                        let a1364 = v4;
                        let a1365 = 2u64;
                        let a1366 = 0u64;
                        nat_rem(a1364, a1365, a1366)
                    };
                    let a1369 = match v1.clone() {
                        Adt11::C0(_, _, _, h1367, _, _) => {
                            h1367
                        }
                    };
                    nat_mul(a1368, a1369)?
                } else {
                    let a1371 = v4;
                    let a1372 = match v1.clone() {
                        Adt11::C0(_, _, _, h1370, _, _) => {
                            h1370
                        }
                    };
                    nat_mul(a1371, a1372)?
                }
            };
            Ok(Some(Adt11::C0({
                let a1373 = v2;
                let a1374 = 1u64;
                nat_add(a1373, a1374)?
            }, {
                let a1376 = match v1.clone() {
                    Adt11::C0(_, h1375, _, _, _, _) => {
                        h1375
                    }
                };
                let a1377 = v5;
                nat_add(a1376, a1377)?
            }, {
                let a1379 = match v1.clone() {
                    Adt11::C0(_, _, h1378, _, _, _) => {
                        h1378
                    }
                };
                let a1380 = 7u64;
                nat_add(a1379, a1380)?
            }, {
                let a1382 = match v1.clone() {
                    Adt11::C0(_, _, h1381, _, _, _) => {
                        h1381
                    }
                };
                let a1383 = 63u64;
                if nat_lt(a1382, a1383) {
                    let a1385 = match v1 {
                        Adt11::C0(_, _, _, h1384, _, _) => {
                            h1384
                        }
                    };
                    let a1386 = 128u64;
                    nat_mul(a1385, a1386)?
                } else {
                    match v1 {
                        Adt11::C0(_, _, _, h1387, _, _) => {
                            h1387
                        }
                    }
                }
            }, true, {
                let a1388 = v3;
                let a1389 = 128u64;
                nat_lt(a1388, a1389)
            })))
        }
    }
}

pub fn f170(v0: Fn31, p1393: u64, p1394: (u64, u64)) -> R<(u64, u64)> {
    let m1392 = (p1393, p1394);
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let (v1, v2) = m1392;
                let m1390 = v1;
                if m1390 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m1390 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c1391: Fn31 = v0.clone();
                        c1391.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    let v3: u64 = {
        let a1396 = {
            let (h1395, _) = v2;
            h1395
        };
        let a1397 = 1u64;
        nat_sub(a1396, a1397)
    };
    Ok((v3, {
        let a1403 = {
            let a1399 = {
                let (_, h1398) = v2;
                h1398
            };
            let a1400 = 256u64;
            nat_mul(a1399, a1400)?
        };
        let a1404 = f68(v1, {
            let a1401 = v0;
            let a1402 = v3;
            nat_add(a1401, a1402)?
        })?;
        nat_add(a1403, a1404)?
    }))
}

pub fn f172(v0: Fn32, p1408: Adt18, p1409: List<Adt9>) -> Adt18 {
    let m1407 = (p1408, p1409);
    {
        let mut m1407 = m1407;
        loop {
            let step = {
                let (v1, v2) = m1407;
                let m1405 = v2;
                match m1405.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt9>), Adt18>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt9>), Adt18>(({
                            let c1406: Fn32 = v0.clone();
                            c1406.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: List<Adt9>, v1: Adt10, v2: Adt18, v3: Adt9) -> Adt18 {
    if if f198(match v1.clone() {
        Adt10::C0(_, h1410, _) => {
            h1410
        }
    }, match v3.clone() {
        Adt9::C0(h1411, _, _, _, _) => {
            h1411
        }
    }) {
        true
    } else {
        !f199(v0, match v1 {
            Adt10::C0(_, h1412, _) => {
                h1412
            }
        }, v3.clone())
    } {
        v2
    } else {
        if match v2.clone() {
            Adt18::C0(h1413, _) => {
                h1413
            }
        } {
            if f141({
                let m1416 = match v2.clone() {
                    Adt18::C0(_, h1415) => {
                        (*h1415).clone()
                    }
                };
                match m1416 {
                    Adt9::C0(_, _, h1414, _, _) => {
                        h1414
                    }
                }
            }, match v3.clone() {
                Adt9::C0(_, _, h1417, _, _) => {
                    h1417
                }
            }) {
                v2
            } else {
                Adt18::C0(true, std::rc::Rc::new(v3))
            }
        } else {
            Adt18::C0(true, std::rc::Rc::new(v3))
        }
    }
}

pub fn f174(v0: u64, v1: (bool, Bytes), v2: Bytes) -> R<(bool, Bytes)> {
    let v3: (bool, Bytes) = {
        let a1418 = v0;
        let a1419 = 0u64;
        if nat_eq(a1418, a1419) {
            f200(v2)?
        } else {
            f175(v2)?
        }
    };
    Ok(({
        let (h1420, _) = v1;
        if h1420 {
            let (h1421, _) = v3;
            h1421
        } else {
            false
        }
    }, {
        let a1424 = {
            let (_, h1422) = v1;
            h1422
        };
        let a1425 = {
            let (_, h1423) = v3;
            h1423
        };
        append_bytes(a1424, a1425)
    }))
}

pub fn f175(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f3(v0.clone())?;
    let v2: Adt20 = {
        let (h1426, _) = f201(Fn35::F202, {
            let a1430 = {
                let a1428 = {
                    let a1427 = v0;
                    length_bytes(a1427)
                };
                let a1429 = 40u64;
                nat_mul(a1428, a1429)?
            };
            let a1431 = 8u64;
            nat_add(a1430, a1431)?
        }, Adt20::C0(List::cons(Adt21::C0(f203({
            let (_, h1432) = v1;
            h1432
        }), Bytes::lit(&[]), 0u64), List::<Adt21>::nil()), Bytes::lit(&[]), false))?;
        h1426
    };
    Ok(({
        let (h1433, _) = v1;
        if h1433 {
            match v2.clone() {
                Adt20::C0(_, _, h1434) => {
                    h1434
                }
            }
        } else {
            false
        }
    }, match v2 {
        Adt20::C0(_, h1435, _) => {
            h1435
        }
    }))
}

pub fn f176(v0: List<Adt0>, v1: u64) -> R<u64> {
    f204(Fn36::F205(v1), 0u64, v0)
}

pub fn f177(v0: List<Bytes>) -> R<Bytes> {
    f27(Fn3::F206, Bytes::lit(&[]), v0)
}

pub fn f178(v0: Fn33, p1439: (bool, List<Adt3>), p1440: List<Bytes>) -> R<(bool, List<Adt3>)> {
    let m1438 = (p1439, p1440);
    {
        let mut m1438 = m1438;
        loop {
            let step = {
                let (v1, v2) = m1438;
                let m1436 = v2;
                match m1436.uncons() {
                    None => {
                        Err::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt3>), List<Bytes>), R<(bool, List<Adt3>)>>(({
                            let c1437: Fn33 = v0.clone();
                            c1437.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: u64, v1: (bool, List<Adt3>), v2: Bytes) -> R<(bool, List<Adt3>)> {
    let v3: (bool, List<Adt3>) = f207(v2, v0)?;
    Ok(({
        let (h1441, _) = v1;
        if h1441 {
            let (h1442, _) = v3;
            h1442
        } else {
            false
        }
    }, f81(Fn14::F208, {
        let (_, h1443) = v1;
        h1443
    }, f83({
        let (_, h1444) = v3;
        h1444
    }))))
}

pub fn f180(v0: Adt13) -> R<Adt13> {
    let v1: u32 = f209(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, h1445) => {
            h1445
        }
    });
    let v2: u32 = f152(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h1446, _, _) => {
            h1446
        }
    }, f152(f210(match v0.clone() {
        Adt13::C0(_, _, _, _, h1447, _, _, _, _, _) => {
            h1447
        }
    }), f152(f211(match v0.clone() {
        Adt13::C0(_, _, _, _, h1448, _, _, _, _, _) => {
            h1448
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1449, _, _, _, _) => {
            h1449
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1450, _, _, _) => {
            h1450
        }
    }), f152(v1, {
        let m1453 = match v0.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, h1452, _) => {
                (*h1452).clone()
            }
        };
        match m1453 {
            Adt12::C0(h1451, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1451
            }
        }
    }))));
    let v3: u32 = f152(f212(match v0.clone() {
        Adt13::C0(h1454, _, _, _, _, _, _, _, _, _) => {
            h1454
        }
    }), f213(match v0.clone() {
        Adt13::C0(h1455, _, _, _, _, _, _, _, _, _) => {
            h1455
        }
    }, match v0.clone() {
        Adt13::C0(_, h1456, _, _, _, _, _, _, _, _) => {
            h1456
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h1457, _, _, _, _, _, _, _) => {
            h1457
        }
    }));
    Ok(Adt13::C0(f152(v2, v3), match v0.clone() {
        Adt13::C0(h1458, _, _, _, _, _, _, _, _, _) => {
            h1458
        }
    }, match v0.clone() {
        Adt13::C0(_, h1459, _, _, _, _, _, _, _, _) => {
            h1459
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h1460, _, _, _, _, _, _, _) => {
            h1460
        }
    }, f152(match v0.clone() {
        Adt13::C0(_, _, _, h1461, _, _, _, _, _, _) => {
            h1461
        }
    }, v2), match v0.clone() {
        Adt13::C0(_, _, _, _, h1462, _, _, _, _, _) => {
            h1462
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1463, _, _, _, _) => {
            h1463
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1464, _, _, _) => {
            h1464
        }
    }, std::rc::Rc::new(f214(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, h1465, _) => {
            (*h1465).clone()
        }
    })), {
        let a1467 = match v0 {
            Adt13::C0(_, _, _, _, _, _, _, _, _, h1466) => {
                h1466
            }
        };
        let a1468 = 1u64;
        nat_add(a1467, a1468)?
    }))
}

pub fn f181(v0: u32) -> u64 {
    let m1470 = {
        let a1469 = v0;
        fixed_u64::convert(i128::from(a1469))
    };
    match m1470 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f182(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1475 = f196(v0, 24u32);
    let a1476 = {
        let a1473 = f196(v1, 16u32);
        let a1474 = {
            let a1471 = f196(v2, 8u32);
            let a1472 = v3;
            fixed_u32::bit_or(a1471, a1472)
        };
        fixed_u32::bit_or(a1473, a1474)
    };
    fixed_u32::bit_or(a1475, a1476)
}

pub fn f183(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1477 = v2;
    let a1478 = v1;
    if nat_lt(a1477, a1478) {
        let m1481 = {
            let a1479 = v0;
            let a1480 = v2;
            index_bytes(a1479, a1480)
        };
        match m1481 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1483 = {
                    let a1482 = v3;
                    fixed_u32::convert(i128::from(a1482))
                };
                match m1483 {
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
        let a1484 = v2;
        let a1485 = v1;
        if nat_eq(a1484, a1485) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f184(v0: Adt7) -> R<Adt7> {
    let (_, h1486) = f120(Fn22::F215, 24u64, (0u64, v0))?;
    Ok(h1486)
}

pub fn f185(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f216(f217(v0.clone(), v1, v2, v3, {
        let a1487 = v4;
        let a1488 = 0u64;
        nat_add(a1487, a1488)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1489 = v4;
        let a1490 = 1u64;
        nat_add(a1489, a1490)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1491 = v4;
        let a1492 = 2u64;
        nat_add(a1491, a1492)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1493 = v4;
        let a1494 = 3u64;
        nat_add(a1493, a1494)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1495 = v4;
        let a1496 = 4u64;
        nat_add(a1495, a1496)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1497 = v4;
        let a1498 = 5u64;
        nat_add(a1497, a1498)?
    })?, f217(v0.clone(), v1, v2, v3, {
        let a1499 = v4;
        let a1500 = 6u64;
        nat_add(a1499, a1500)?
    })?, f217(v0, v1, v2, v3, {
        let a1501 = v4;
        let a1502 = 7u64;
        nat_add(a1501, a1502)?
    })?))
}

pub fn f186(v0: Adt15) -> R<Adt15> {
    let v1: u64 = f218(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h1503) => {
            h1503
        }
    });
    let v2: u64 = f157(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, h1504, _, _) => {
            h1504
        }
    }, f157(f219(match v0.clone() {
        Adt15::C0(_, _, _, _, h1505, _, _, _, _, _) => {
            h1505
        }
    }), f157(f220(match v0.clone() {
        Adt15::C0(_, _, _, _, h1506, _, _, _, _, _) => {
            h1506
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h1507, _, _, _, _) => {
            h1507
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h1508, _, _, _) => {
            h1508
        }
    }), f157(v1, {
        let m1511 = match v0.clone() {
            Adt15::C0(_, _, _, _, _, _, _, _, h1510, _) => {
                (*h1510).clone()
            }
        };
        match m1511 {
            Adt14::C0(h1509, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1509
            }
        }
    }))));
    let v3: u64 = f157(f221(match v0.clone() {
        Adt15::C0(h1512, _, _, _, _, _, _, _, _, _) => {
            h1512
        }
    }), f222(match v0.clone() {
        Adt15::C0(h1513, _, _, _, _, _, _, _, _, _) => {
            h1513
        }
    }, match v0.clone() {
        Adt15::C0(_, h1514, _, _, _, _, _, _, _, _) => {
            h1514
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h1515, _, _, _, _, _, _, _) => {
            h1515
        }
    }));
    Ok(Adt15::C0(f157(v2, v3), match v0.clone() {
        Adt15::C0(h1516, _, _, _, _, _, _, _, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt15::C0(_, h1517, _, _, _, _, _, _, _, _) => {
            h1517
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h1518, _, _, _, _, _, _, _) => {
            h1518
        }
    }, f157(match v0.clone() {
        Adt15::C0(_, _, _, h1519, _, _, _, _, _, _) => {
            h1519
        }
    }, v2), match v0.clone() {
        Adt15::C0(_, _, _, _, h1520, _, _, _, _, _) => {
            h1520
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h1521, _, _, _, _) => {
            h1521
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h1522, _, _, _) => {
            h1522
        }
    }, std::rc::Rc::new(f223(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, h1523, _) => {
            (*h1523).clone()
        }
    })), {
        let a1525 = match v0 {
            Adt15::C0(_, _, _, _, _, _, _, _, _, h1524) => {
                h1524
            }
        };
        let a1526 = 1u64;
        nat_add(a1525, a1526)?
    }))
}

pub fn f187(v0: u64, v1: u64) -> u64 {
    let m1529 = {
        let a1527 = v0;
        let a1528 = v1;
        fixed_u64::checked_add(a1527, a1528)
    };
    match m1529 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f188(v0: u64, v1: u32) -> u64 {
    let m1532 = {
        let a1530 = v0;
        let a1531 = v1;
        fixed_u64::shift_left(a1530, a1531)
    };
    match m1532 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f189(v0: u64) -> u64 {
    let (_, h1533) = {
        let (_, h1534) = f224(Fn37::F225, 64u64, (v0, (1u64, 0u64)));
        h1534
    };
    h1533
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1547 = f188(v0, 56u32);
    let a1548 = {
        let a1545 = f188(v1, 48u32);
        let a1546 = {
            let a1543 = f188(v2, 40u32);
            let a1544 = {
                let a1541 = f188(v3, 32u32);
                let a1542 = {
                    let a1539 = f188(v4, 24u32);
                    let a1540 = {
                        let a1537 = f188(v5, 16u32);
                        let a1538 = {
                            let a1535 = f188(v6, 8u32);
                            let a1536 = v7;
                            fixed_u64::bit_or(a1535, a1536)
                        };
                        fixed_u64::bit_or(a1537, a1538)
                    };
                    fixed_u64::bit_or(a1539, a1540)
                };
                fixed_u64::bit_or(a1541, a1542)
            };
            fixed_u64::bit_or(a1543, a1544)
        };
        fixed_u64::bit_or(a1545, a1546)
    };
    fixed_u64::bit_or(a1547, a1548)
}

pub fn f191(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1549 = v2;
    let a1550 = v1;
    if nat_lt(a1549, a1550) {
        f226(v0, v2)
    } else {
        let a1551 = v2;
        let a1552 = v1;
        if nat_eq(a1551, a1552) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f192(v0: Fn34, p1556: u64, p1557: Adt19) -> Adt19 {
    let m1555 = (p1556, p1557);
    {
        let mut m1555 = m1555;
        loop {
            let step = {
                let (v1, v2) = m1555;
                let m1553 = v1;
                if m1553 == 0 {
                    Err::<(u64, Adt19), Adt19>(v2)
                } else {
                    let v3 = m1553 - 1;
                    Ok::<(u64, Adt19), Adt19>((v3, {
                        let c1554: Fn34 = v0.clone();
                        c1554.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt19) -> Adt19 {
    f227(v0)
}

pub fn f194(v0: (Adt17, Adt16)) -> (Adt17, Adt16) {
    (f228(f229({
        let (h1558, _) = v0.clone();
        h1558
    }, {
        let (_, h1559) = v0.clone();
        h1559
    }), {
        let (_, h1560) = v0.clone();
        h1560
    }), f230({
        let (_, h1561) = v0;
        h1561
    }))
}

pub fn f195(v0: Bytes, v1: u64) -> u32 {
    let m1564 = {
        let a1562 = v0;
        let a1563 = v1;
        index_bytes(a1562, a1563)
    };
    match m1564 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1566 = {
                let a1565 = v2;
                fixed_u32::convert(i128::from(a1565))
            };
            match m1566 {
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

pub fn f196(v0: u32, v1: u32) -> u32 {
    let m1569 = {
        let a1567 = v0;
        let a1568 = v1;
        fixed_u32::shift_left(a1567, a1568)
    };
    match m1569 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f197(v0: Adt5, v1: Adt5) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt5::C0(h1570, _, _, _, _, _, _, _) => {
            h1570
        }
    }, match v0.clone() {
        Adt5::C0(_, h1571, _, _, _, _, _, _) => {
            h1571
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h1572, _, _, _, _, _) => {
            h1572
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, h1573, _, _, _, _) => {
            h1573
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h1574, _, _, _) => {
            h1574
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h1575, _, _) => {
            h1575
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h1576, _) => {
            h1576
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, _, _, _, h1577) => {
            h1577
        }
    }, match v1.clone() {
        Adt5::C0(h1578, _, _, _, _, _, _, _) => {
            h1578
        }
    }, match v1.clone() {
        Adt5::C0(_, h1579, _, _, _, _, _, _) => {
            h1579
        }
    }, match v1.clone() {
        Adt5::C0(_, _, h1580, _, _, _, _, _) => {
            h1580
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, h1581, _, _, _, _) => {
            h1581
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, h1582, _, _, _) => {
            h1582
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, h1583, _, _) => {
            h1583
        }
    }, match v1.clone() {
        Adt5::C0(_, _, _, _, _, _, h1584, _) => {
            h1584
        }
    }, match v1 {
        Adt5::C0(_, _, _, _, _, _, _, h1585) => {
            h1585
        }
    })
}

pub fn f198(v0: List<u64>, v1: u64) -> bool {
    f231(Fn38::F232(v1), false, v0)
}

pub fn f199(v0: List<Adt9>, v1: List<u64>, v2: Adt9) -> bool {
    f233(Fn39::F234(v1, v0), true, match v2 {
        Adt9::C0(_, _, _, h1586, _) => {
            h1586
        }
    })
}

pub fn f200(v0: Bytes) -> R<(bool, Bytes)> {
    let v1: (bool, List<Adt0>) = f3(v0.clone())?;
    let v2: List<Adt0> = {
        let (_, h1587) = v1;
        h1587
    };
    let v3: u64 = {
        let a1588 = f12(f4(v2.clone(), 2u64));
        let a1589 = 4294967296u64;
        let a1590 = 0u64;
        nat_rem(a1588, a1589, a1590)
    };
    let v4: (bool, List<u64>) = f235(v2.clone(), 1u64)?;
    let v5: (bool, Bytes) = f236(v0, v2.clone(), v3)?;
    Ok(({
        let (h1591, _) = v1;
        if if h1591 {
            let a1592 = 0u64;
            let a1593 = v3;
            if nat_lt(a1592, a1593) {
                let a1594 = v3;
                let a1595 = 24u64;
                nat_lt(a1594, a1595)
            } else {
                false
            }
        } else {
            false
        } {
            let (h1596, _) = v4.clone();
            if h1596 {
                let (h1597, _) = v5;
                h1597
            } else {
                false
            }
        } else {
            false
        }
    }, {
        let a1607 = f11(f6(f4(v2, 8u64)))?;
        let a1608 = {
            let a1605 = f10(v3, 4u64)?;
            let a1606 = {
                let a1603 = f10(f237({
                    let (_, h1598) = v4.clone();
                    h1598
                })?, 4u64)?;
                let a1604 = {
                    let a1601 = f238({
                        let (_, h1599) = v4;
                        h1599
                    }, 8u64)?;
                    let a1602 = {
                        let (_, h1600) = v5;
                        h1600
                    };
                    append_bytes(a1601, a1602)
                };
                append_bytes(a1603, a1604)
            };
            append_bytes(a1605, a1606)
        };
        append_bytes(a1607, a1608)
    }))
}

pub fn f201(v0: Fn35, p1613: u64, p1614: Adt20) -> R<(Adt20, bool)> {
    let m1612 = (p1613, p1614);
    {
        let mut m1612 = m1612;
        loop {
            let step = {
                let (v1, v2) = m1612;
                let m1609 = v1;
                if m1609 == 0 {
                    Err::<(u64, Adt20), R<(Adt20, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1609 - 1;
                    let m1611 = {
                        let c1610: Fn35 = v0.clone();
                        c1610.apply(v2.clone())?
                    };
                    match m1611 {
                        None => {
                            Err::<(u64, Adt20), R<(Adt20, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt20), R<(Adt20, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Adt20) -> R<Option<Adt20>> {
    f239(v0)
}

pub fn f203(v0: List<Adt0>) -> List<Adt0> {
    f47(Fn8::F240, List::<Adt0>::nil(), v0)
}

pub fn f204(v0: Fn36, p1618: u64, p1619: List<Adt0>) -> R<u64> {
    let m1617 = (p1618, p1619);
    {
        let mut m1617 = m1617;
        loop {
            let step = {
                let (v1, v2) = m1617;
                let m1615 = v2;
                match m1615.uncons() {
                    None => {
                        Err::<(u64, List<Adt0>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt0>), R<u64>>(({
                            let c1616: Fn36 = v0.clone();
                            c1616.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: u64, v1: u64, v2: Adt0) -> R<u64> {
    let a1621 = match v2 {
        Adt0::C0(h1620, _, _, _) => {
            h1620
        }
    };
    let a1622 = v0;
    if nat_eq(a1621, a1622) {
        let a1623 = v1;
        let a1624 = 1u64;
        nat_add(a1623, a1624)
    } else {
        Ok(v1)
    }
}

pub fn f206(v0: Bytes, v1: Bytes) -> R<Bytes> {
    let a1625 = v0;
    let a1626 = f11(v1)?;
    Ok(append_bytes(a1625, a1626))
}

pub fn f207(v0: Bytes, v1: u64) -> R<(bool, List<Adt3>)> {
    let v2: (bool, Adt0) = f51(v0.clone(), 1u64)?;
    let v3: (bool, Adt0) = f51(v0.clone(), 20u64)?;
    let v4: u64 = {
        let a1628 = f12({
            let (_, h1627) = v3;
            h1627
        });
        let a1629 = 4294967296u64;
        let a1630 = 0u64;
        nat_rem(a1628, a1629, a1630)
    };
    let v5: Adt3 = f112({
        let a1632 = f11(f6({
            let (_, h1631) = v2;
            h1631
        }))?;
        let a1633 = f10(v4, 4u64)?;
        append_bytes(a1632, a1633)
    });
    let v6: bool = {
        let (h1634, _) = v2;
        if h1634 {
            let (h1635, _) = v3;
            h1635
        } else {
            false
        }
    };
    let a1636 = v4;
    let a1637 = 5u64;
    if nat_eq(a1636, a1637) {
        let v7: (bool, Adt0) = f51(v0, 6u64)?;
        Ok((if v6 {
            let (h1638, _) = v7;
            h1638
        } else {
            false
        }, List::cons(Adt3::C0(true, f6({
            let (_, h1639) = v7;
            h1639
        }), {
            let a1640 = v1;
            let a1641 = 1u64;
            nat_add(a1640, a1641)?
        }), List::cons(v5, List::<Adt3>::nil()))))
    } else {
        let a1642 = v4;
        let a1643 = 10u64;
        if nat_eq(a1642, a1643) {
            let v8: (bool, List<Adt0>) = f3(v0)?;
            Ok((if v6 {
                let (h1644, _) = v8;
                h1644
            } else {
                false
            }, f241(Fn40::F242(v1), List::cons(v5, List::<Adt3>::nil()), f20({
                let (_, h1645) = v8;
                h1645
            }, 11u64))?))
        } else {
            let v9: (bool, Bytes) = f243(v0, v4)?;
            Ok((if v6 {
                let (h1646, _) = v9;
                h1646
            } else {
                false
            }, List::cons(f112({
                let (_, h1647) = v9;
                h1647
            }), List::cons(v5, List::<Adt3>::nil()))))
        }
    }
}

pub fn f208(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f209(v0: u64) -> u32 {
    let a1648 = v0;
    let a1649 = 32u64;
    if nat_lt(a1648, a1649) {
        let a1650 = v0;
        let a1651 = 16u64;
        if nat_lt(a1650, a1651) {
            let a1652 = v0;
            let a1653 = 8u64;
            if nat_lt(a1652, a1653) {
                let a1654 = v0;
                let a1655 = 4u64;
                if nat_lt(a1654, a1655) {
                    let a1656 = v0;
                    let a1657 = 2u64;
                    if nat_lt(a1656, a1657) {
                        let a1658 = v0;
                        let a1659 = 1u64;
                        if nat_lt(a1658, a1659) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1660 = v0;
                        let a1661 = 3u64;
                        if nat_lt(a1660, a1661) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1662 = v0;
                    let a1663 = 6u64;
                    if nat_lt(a1662, a1663) {
                        let a1664 = v0;
                        let a1665 = 5u64;
                        if nat_lt(a1664, a1665) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1666 = v0;
                        let a1667 = 7u64;
                        if nat_lt(a1666, a1667) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1668 = v0;
                let a1669 = 12u64;
                if nat_lt(a1668, a1669) {
                    let a1670 = v0;
                    let a1671 = 10u64;
                    if nat_lt(a1670, a1671) {
                        let a1672 = v0;
                        let a1673 = 9u64;
                        if nat_lt(a1672, a1673) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1674 = v0;
                        let a1675 = 11u64;
                        if nat_lt(a1674, a1675) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1676 = v0;
                    let a1677 = 14u64;
                    if nat_lt(a1676, a1677) {
                        let a1678 = v0;
                        let a1679 = 13u64;
                        if nat_lt(a1678, a1679) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1680 = v0;
                        let a1681 = 15u64;
                        if nat_lt(a1680, a1681) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1682 = v0;
            let a1683 = 24u64;
            if nat_lt(a1682, a1683) {
                let a1684 = v0;
                let a1685 = 20u64;
                if nat_lt(a1684, a1685) {
                    let a1686 = v0;
                    let a1687 = 18u64;
                    if nat_lt(a1686, a1687) {
                        let a1688 = v0;
                        let a1689 = 17u64;
                        if nat_lt(a1688, a1689) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1690 = v0;
                        let a1691 = 19u64;
                        if nat_lt(a1690, a1691) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1692 = v0;
                    let a1693 = 22u64;
                    if nat_lt(a1692, a1693) {
                        let a1694 = v0;
                        let a1695 = 21u64;
                        if nat_lt(a1694, a1695) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1696 = v0;
                        let a1697 = 23u64;
                        if nat_lt(a1696, a1697) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1698 = v0;
                let a1699 = 28u64;
                if nat_lt(a1698, a1699) {
                    let a1700 = v0;
                    let a1701 = 26u64;
                    if nat_lt(a1700, a1701) {
                        let a1702 = v0;
                        let a1703 = 25u64;
                        if nat_lt(a1702, a1703) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1704 = v0;
                        let a1705 = 27u64;
                        if nat_lt(a1704, a1705) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1706 = v0;
                    let a1707 = 30u64;
                    if nat_lt(a1706, a1707) {
                        let a1708 = v0;
                        let a1709 = 29u64;
                        if nat_lt(a1708, a1709) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1710 = v0;
                        let a1711 = 31u64;
                        if nat_lt(a1710, a1711) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1712 = v0;
        let a1713 = 48u64;
        if nat_lt(a1712, a1713) {
            let a1714 = v0;
            let a1715 = 40u64;
            if nat_lt(a1714, a1715) {
                let a1716 = v0;
                let a1717 = 36u64;
                if nat_lt(a1716, a1717) {
                    let a1718 = v0;
                    let a1719 = 34u64;
                    if nat_lt(a1718, a1719) {
                        let a1720 = v0;
                        let a1721 = 33u64;
                        if nat_lt(a1720, a1721) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1722 = v0;
                        let a1723 = 35u64;
                        if nat_lt(a1722, a1723) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1724 = v0;
                    let a1725 = 38u64;
                    if nat_lt(a1724, a1725) {
                        let a1726 = v0;
                        let a1727 = 37u64;
                        if nat_lt(a1726, a1727) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1728 = v0;
                        let a1729 = 39u64;
                        if nat_lt(a1728, a1729) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1730 = v0;
                let a1731 = 44u64;
                if nat_lt(a1730, a1731) {
                    let a1732 = v0;
                    let a1733 = 42u64;
                    if nat_lt(a1732, a1733) {
                        let a1734 = v0;
                        let a1735 = 41u64;
                        if nat_lt(a1734, a1735) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1736 = v0;
                        let a1737 = 43u64;
                        if nat_lt(a1736, a1737) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1738 = v0;
                    let a1739 = 46u64;
                    if nat_lt(a1738, a1739) {
                        let a1740 = v0;
                        let a1741 = 45u64;
                        if nat_lt(a1740, a1741) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1742 = v0;
                        let a1743 = 47u64;
                        if nat_lt(a1742, a1743) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1744 = v0;
            let a1745 = 56u64;
            if nat_lt(a1744, a1745) {
                let a1746 = v0;
                let a1747 = 52u64;
                if nat_lt(a1746, a1747) {
                    let a1748 = v0;
                    let a1749 = 50u64;
                    if nat_lt(a1748, a1749) {
                        let a1750 = v0;
                        let a1751 = 49u64;
                        if nat_lt(a1750, a1751) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1752 = v0;
                        let a1753 = 51u64;
                        if nat_lt(a1752, a1753) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1754 = v0;
                    let a1755 = 54u64;
                    if nat_lt(a1754, a1755) {
                        let a1756 = v0;
                        let a1757 = 53u64;
                        if nat_lt(a1756, a1757) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1758 = v0;
                        let a1759 = 55u64;
                        if nat_lt(a1758, a1759) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1760 = v0;
                let a1761 = 60u64;
                if nat_lt(a1760, a1761) {
                    let a1762 = v0;
                    let a1763 = 58u64;
                    if nat_lt(a1762, a1763) {
                        let a1764 = v0;
                        let a1765 = 57u64;
                        if nat_lt(a1764, a1765) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1766 = v0;
                        let a1767 = 59u64;
                        if nat_lt(a1766, a1767) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1768 = v0;
                    let a1769 = 62u64;
                    if nat_lt(a1768, a1769) {
                        let a1770 = v0;
                        let a1771 = 61u64;
                        if nat_lt(a1770, a1771) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1772 = v0;
                        let a1773 = 63u64;
                        if nat_lt(a1772, a1773) {
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

pub fn f210(v0: u32) -> u32 {
    let a1776 = {
        let a1774 = f244(v0, 6u32, 26u32);
        let a1775 = f244(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1774, a1775)
    };
    let a1777 = f244(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1776, a1777)
}

pub fn f211(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1783 = {
        let a1778 = v0;
        let a1779 = v1;
        fixed_u32::bit_and(a1778, a1779)
    };
    let a1784 = {
        let a1781 = {
            let a1780 = v0;
            fixed_u32::bit_not(a1780)
        };
        let a1782 = v2;
        fixed_u32::bit_and(a1781, a1782)
    };
    fixed_u32::bit_xor(a1783, a1784)
}

pub fn f212(v0: u32) -> u32 {
    let a1787 = {
        let a1785 = f244(v0, 2u32, 30u32);
        let a1786 = f244(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1785, a1786)
    };
    let a1788 = f244(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1787, a1788)
}

pub fn f213(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1797 = {
        let a1793 = {
            let a1789 = v0;
            let a1790 = v1;
            fixed_u32::bit_and(a1789, a1790)
        };
        let a1794 = {
            let a1791 = v0;
            let a1792 = v2;
            fixed_u32::bit_and(a1791, a1792)
        };
        fixed_u32::bit_xor(a1793, a1794)
    };
    let a1798 = {
        let a1795 = v1;
        let a1796 = v2;
        fixed_u32::bit_and(a1795, a1796)
    };
    fixed_u32::bit_xor(a1797, a1798)
}

pub fn f214(v0: Adt12) -> Adt12 {
    Adt12::C0(match v0.clone() {
        Adt12::C0(_, h1799, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1799
        }
    }, match v0.clone() {
        Adt12::C0(_, _, h1800, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1800
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, h1801, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1801
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, h1802, _, _, _, _, _, _, _, _, _, _, _) => {
            h1802
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, h1803, _, _, _, _, _, _, _, _, _, _) => {
            h1803
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, h1804, _, _, _, _, _, _, _, _, _) => {
            h1804
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, h1805, _, _, _, _, _, _, _, _) => {
            h1805
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, h1806, _, _, _, _, _, _, _) => {
            h1806
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1807, _, _, _, _, _, _) => {
            h1807
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, h1808, _, _, _, _, _) => {
            h1808
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, h1809, _, _, _, _) => {
            h1809
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1810, _, _, _) => {
            h1810
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1811, _, _) => {
            h1811
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1812, _) => {
            h1812
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1813) => {
            h1813
        }
    }, f152(f245(match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1814, _) => {
            h1814
        }
    }), f152(match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1815, _, _, _, _, _, _) => {
            h1815
        }
    }, f152(f246(match v0.clone() {
        Adt12::C0(_, h1816, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1816
        }
    }), match v0 {
        Adt12::C0(h1817, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1817
        }
    }))))
}

pub fn f215(v0: (u64, Adt7)) -> R<(u64, Adt7)> {
    f247(v0)
}

pub fn f216(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1830 = f188(v7, 56u32);
    let a1831 = {
        let a1828 = f188(v6, 48u32);
        let a1829 = {
            let a1826 = f188(v5, 40u32);
            let a1827 = {
                let a1824 = f188(v4, 32u32);
                let a1825 = {
                    let a1822 = f188(v3, 24u32);
                    let a1823 = {
                        let a1820 = f188(v2, 16u32);
                        let a1821 = {
                            let a1818 = f188(v1, 8u32);
                            let a1819 = v0;
                            fixed_u64::bit_or(a1818, a1819)
                        };
                        fixed_u64::bit_or(a1820, a1821)
                    };
                    fixed_u64::bit_or(a1822, a1823)
                };
                fixed_u64::bit_or(a1824, a1825)
            };
            fixed_u64::bit_or(a1826, a1827)
        };
        fixed_u64::bit_or(a1828, a1829)
    };
    fixed_u64::bit_or(a1830, a1831)
}

pub fn f217(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1832 = v4;
        let a1833 = v1;
        if nat_lt(a1832, a1833) {
            f226(v0, v4)
        } else {
            let a1834 = v4;
            let a1835 = v1;
            if nat_eq(a1834, a1835) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1838 = {
        let a1836 = v4;
        let a1837 = 1u64;
        nat_add(a1836, a1837)?
    };
    let a1839 = v2;
    if nat_eq(a1838, a1839) {
        let a1840 = v5;
        let a1841 = 128u64;
        Ok(fixed_u64::bit_or(a1840, a1841))
    } else {
        Ok(v5)
    }
}

pub fn f218(v0: u64) -> u64 {
    let a1842 = v0;
    let a1843 = 40u64;
    if nat_lt(a1842, a1843) {
        let a1844 = v0;
        let a1845 = 20u64;
        if nat_lt(a1844, a1845) {
            let a1846 = v0;
            let a1847 = 10u64;
            if nat_lt(a1846, a1847) {
                let a1848 = v0;
                let a1849 = 5u64;
                if nat_lt(a1848, a1849) {
                    let a1850 = v0;
                    let a1851 = 2u64;
                    if nat_lt(a1850, a1851) {
                        let a1852 = v0;
                        let a1853 = 1u64;
                        if nat_lt(a1852, a1853) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1854 = v0;
                        let a1855 = 3u64;
                        if nat_lt(a1854, a1855) {
                            13096744586834688815u64
                        } else {
                            let a1856 = v0;
                            let a1857 = 4u64;
                            if nat_lt(a1856, a1857) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1858 = v0;
                    let a1859 = 7u64;
                    if nat_lt(a1858, a1859) {
                        let a1860 = v0;
                        let a1861 = 6u64;
                        if nat_lt(a1860, a1861) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1862 = v0;
                        let a1863 = 8u64;
                        if nat_lt(a1862, a1863) {
                            12329834152419229976u64
                        } else {
                            let a1864 = v0;
                            let a1865 = 9u64;
                            if nat_lt(a1864, a1865) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1866 = v0;
                let a1867 = 15u64;
                if nat_lt(a1866, a1867) {
                    let a1868 = v0;
                    let a1869 = 12u64;
                    if nat_lt(a1868, a1869) {
                        let a1870 = v0;
                        let a1871 = 11u64;
                        if nat_lt(a1870, a1871) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1872 = v0;
                        let a1873 = 13u64;
                        if nat_lt(a1872, a1873) {
                            8268148722764581231u64
                        } else {
                            let a1874 = v0;
                            let a1875 = 14u64;
                            if nat_lt(a1874, a1875) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1876 = v0;
                    let a1877 = 17u64;
                    if nat_lt(a1876, a1877) {
                        let a1878 = v0;
                        let a1879 = 16u64;
                        if nat_lt(a1878, a1879) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1880 = v0;
                        let a1881 = 18u64;
                        if nat_lt(a1880, a1881) {
                            17275323862435702243u64
                        } else {
                            let a1882 = v0;
                            let a1883 = 19u64;
                            if nat_lt(a1882, a1883) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1884 = v0;
            let a1885 = 30u64;
            if nat_lt(a1884, a1885) {
                let a1886 = v0;
                let a1887 = 25u64;
                if nat_lt(a1886, a1887) {
                    let a1888 = v0;
                    let a1889 = 22u64;
                    if nat_lt(a1888, a1889) {
                        let a1890 = v0;
                        let a1891 = 21u64;
                        if nat_lt(a1890, a1891) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1892 = v0;
                        let a1893 = 23u64;
                        if nat_lt(a1892, a1893) {
                            6679025012923562964u64
                        } else {
                            let a1894 = v0;
                            let a1895 = 24u64;
                            if nat_lt(a1894, a1895) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1896 = v0;
                    let a1897 = 27u64;
                    if nat_lt(a1896, a1897) {
                        let a1898 = v0;
                        let a1899 = 26u64;
                        if nat_lt(a1898, a1899) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1900 = v0;
                        let a1901 = 28u64;
                        if nat_lt(a1900, a1901) {
                            13788192230050041572u64
                        } else {
                            let a1902 = v0;
                            let a1903 = 29u64;
                            if nat_lt(a1902, a1903) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1904 = v0;
                let a1905 = 35u64;
                if nat_lt(a1904, a1905) {
                    let a1906 = v0;
                    let a1907 = 32u64;
                    if nat_lt(a1906, a1907) {
                        let a1908 = v0;
                        let a1909 = 31u64;
                        if nat_lt(a1908, a1909) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1910 = v0;
                        let a1911 = 33u64;
                        if nat_lt(a1910, a1911) {
                            2861767655752347644u64
                        } else {
                            let a1912 = v0;
                            let a1913 = 34u64;
                            if nat_lt(a1912, a1913) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1914 = v0;
                    let a1915 = 37u64;
                    if nat_lt(a1914, a1915) {
                        let a1916 = v0;
                        let a1917 = 36u64;
                        if nat_lt(a1916, a1917) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1918 = v0;
                        let a1919 = 38u64;
                        if nat_lt(a1918, a1919) {
                            8532644243296465576u64
                        } else {
                            let a1920 = v0;
                            let a1921 = 39u64;
                            if nat_lt(a1920, a1921) {
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
        let a1922 = v0;
        let a1923 = 60u64;
        if nat_lt(a1922, a1923) {
            let a1924 = v0;
            let a1925 = 50u64;
            if nat_lt(a1924, a1925) {
                let a1926 = v0;
                let a1927 = 45u64;
                if nat_lt(a1926, a1927) {
                    let a1928 = v0;
                    let a1929 = 42u64;
                    if nat_lt(a1928, a1929) {
                        let a1930 = v0;
                        let a1931 = 41u64;
                        if nat_lt(a1930, a1931) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1932 = v0;
                        let a1933 = 43u64;
                        if nat_lt(a1932, a1933) {
                            14000437183269869457u64
                        } else {
                            let a1934 = v0;
                            let a1935 = 44u64;
                            if nat_lt(a1934, a1935) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1936 = v0;
                    let a1937 = 47u64;
                    if nat_lt(a1936, a1937) {
                        let a1938 = v0;
                        let a1939 = 46u64;
                        if nat_lt(a1938, a1939) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1940 = v0;
                        let a1941 = 48u64;
                        if nat_lt(a1940, a1941) {
                            1182934255886127544u64
                        } else {
                            let a1942 = v0;
                            let a1943 = 49u64;
                            if nat_lt(a1942, a1943) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1944 = v0;
                let a1945 = 55u64;
                if nat_lt(a1944, a1945) {
                    let a1946 = v0;
                    let a1947 = 52u64;
                    if nat_lt(a1946, a1947) {
                        let a1948 = v0;
                        let a1949 = 51u64;
                        if nat_lt(a1948, a1949) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1950 = v0;
                        let a1951 = 53u64;
                        if nat_lt(a1950, a1951) {
                            4115178125766777443u64
                        } else {
                            let a1952 = v0;
                            let a1953 = 54u64;
                            if nat_lt(a1952, a1953) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1954 = v0;
                    let a1955 = 57u64;
                    if nat_lt(a1954, a1955) {
                        let a1956 = v0;
                        let a1957 = 56u64;
                        if nat_lt(a1956, a1957) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1958 = v0;
                        let a1959 = 58u64;
                        if nat_lt(a1958, a1959) {
                            8693463985226723168u64
                        } else {
                            let a1960 = v0;
                            let a1961 = 59u64;
                            if nat_lt(a1960, a1961) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1962 = v0;
            let a1963 = 70u64;
            if nat_lt(a1962, a1963) {
                let a1964 = v0;
                let a1965 = 65u64;
                if nat_lt(a1964, a1965) {
                    let a1966 = v0;
                    let a1967 = 62u64;
                    if nat_lt(a1966, a1967) {
                        let a1968 = v0;
                        let a1969 = 61u64;
                        if nat_lt(a1968, a1969) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1970 = v0;
                        let a1971 = 63u64;
                        if nat_lt(a1970, a1971) {
                            13761210420658862357u64
                        } else {
                            let a1972 = v0;
                            let a1973 = 64u64;
                            if nat_lt(a1972, a1973) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1974 = v0;
                    let a1975 = 67u64;
                    if nat_lt(a1974, a1975) {
                        let a1976 = v0;
                        let a1977 = 66u64;
                        if nat_lt(a1976, a1977) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1978 = v0;
                        let a1979 = 68u64;
                        if nat_lt(a1978, a1979) {
                            17689382322260857208u64
                        } else {
                            let a1980 = v0;
                            let a1981 = 69u64;
                            if nat_lt(a1980, a1981) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1982 = v0;
                let a1983 = 75u64;
                if nat_lt(a1982, a1983) {
                    let a1984 = v0;
                    let a1985 = 72u64;
                    if nat_lt(a1984, a1985) {
                        let a1986 = v0;
                        let a1987 = 71u64;
                        if nat_lt(a1986, a1987) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1988 = v0;
                        let a1989 = 73u64;
                        if nat_lt(a1988, a1989) {
                            2944078676154940804u64
                        } else {
                            let a1990 = v0;
                            let a1991 = 74u64;
                            if nat_lt(a1990, a1991) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1992 = v0;
                    let a1993 = 77u64;
                    if nat_lt(a1992, a1993) {
                        let a1994 = v0;
                        let a1995 = 76u64;
                        if nat_lt(a1994, a1995) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1996 = v0;
                        let a1997 = 78u64;
                        if nat_lt(a1996, a1997) {
                            6448918945643986474u64
                        } else {
                            let a1998 = v0;
                            let a1999 = 79u64;
                            if nat_lt(a1998, a1999) {
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

pub fn f219(v0: u64) -> u64 {
    let a2002 = {
        let a2000 = f248(v0, 14u32, 50u32);
        let a2001 = f248(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a2000, a2001)
    };
    let a2003 = f248(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a2002, a2003)
}

pub fn f220(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2009 = {
        let a2004 = v0;
        let a2005 = v1;
        fixed_u64::bit_and(a2004, a2005)
    };
    let a2010 = {
        let a2007 = {
            let a2006 = v0;
            fixed_u64::bit_not(a2006)
        };
        let a2008 = v2;
        fixed_u64::bit_and(a2007, a2008)
    };
    fixed_u64::bit_xor(a2009, a2010)
}

pub fn f221(v0: u64) -> u64 {
    let a2013 = {
        let a2011 = f248(v0, 28u32, 36u32);
        let a2012 = f248(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a2011, a2012)
    };
    let a2014 = f248(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a2013, a2014)
}

pub fn f222(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2023 = {
        let a2019 = {
            let a2015 = v0;
            let a2016 = v1;
            fixed_u64::bit_and(a2015, a2016)
        };
        let a2020 = {
            let a2017 = v0;
            let a2018 = v2;
            fixed_u64::bit_and(a2017, a2018)
        };
        fixed_u64::bit_xor(a2019, a2020)
    };
    let a2024 = {
        let a2021 = v1;
        let a2022 = v2;
        fixed_u64::bit_and(a2021, a2022)
    };
    fixed_u64::bit_xor(a2023, a2024)
}

pub fn f223(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(_, h2025, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2025
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h2026, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2026
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h2027, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2027
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h2028, _, _, _, _, _, _, _, _, _, _, _) => {
            h2028
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h2029, _, _, _, _, _, _, _, _, _, _) => {
            h2029
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h2030, _, _, _, _, _, _, _, _, _) => {
            h2030
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h2031, _, _, _, _, _, _, _, _) => {
            h2031
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h2032, _, _, _, _, _, _, _) => {
            h2032
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h2033, _, _, _, _, _, _) => {
            h2033
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h2034, _, _, _, _, _) => {
            h2034
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, h2035, _, _, _, _) => {
            h2035
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2036, _, _, _) => {
            h2036
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2037, _, _) => {
            h2037
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2038, _) => {
            h2038
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2039) => {
            h2039
        }
    }, f157(f249(match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2040, _) => {
            h2040
        }
    }), f157(match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h2041, _, _, _, _, _, _) => {
            h2041
        }
    }, f157(f250(match v0.clone() {
        Adt14::C0(_, h2042, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2042
        }
    }), match v0 {
        Adt14::C0(h2043, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2043
        }
    }))))
}

pub fn f224(v0: Fn37, p2047: u64, p2048: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m2046 = (p2047, p2048);
    {
        let mut m2046 = m2046;
        loop {
            let step = {
                let (v1, v2) = m2046;
                let m2044 = v1;
                if m2044 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m2044 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c2045: Fn37 = v0.clone();
                        c2045.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h2049, _) = v0;
        h2049
    };
    let v2: u64 = {
        let (h2050, _) = {
            let (_, h2051) = v0;
            h2051
        };
        h2050
    };
    let v3: u64 = {
        let (_, h2052) = {
            let (_, h2053) = v0;
            h2053
        };
        h2052
    };
    ({
        let a2054 = v1;
        let a2055 = 2u64;
        let a2056 = 0u64;
        nat_quot(a2054, a2055, a2056)
    }, (f188(v2, 1u32), {
        let a2060 = {
            let a2057 = v1;
            let a2058 = 2u64;
            let a2059 = 0u64;
            nat_rem(a2057, a2058, a2059)
        };
        let a2061 = 1u64;
        if nat_eq(a2060, a2061) {
            let a2062 = v3;
            let a2063 = v2;
            fixed_u64::bit_or(a2062, a2063)
        } else {
            v3
        }
    }))
}

pub fn f226(v0: Bytes, v1: u64) -> u64 {
    let m2066 = {
        let a2064 = v0;
        let a2065 = v1;
        index_bytes(a2064, a2065)
    };
    match m2066 {
        None => {
            0u64
        }
        Some(v2) => {
            let m2068 = {
                let a2067 = v2;
                fixed_u64::convert(i128::from(a2067))
            };
            match m2068 {
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

pub fn f227(v0: Adt19) -> Adt19 {
    let a2073 = {
        let a2070 = match v0.clone() {
            Adt19::C0(_, h2069, _) => {
                h2069
            }
        };
        let a2071 = 2u64;
        let a2072 = 0u64;
        nat_rem(a2070, a2071, a2072)
    };
    let a2074 = 0u64;
    if nat_eq(a2073, a2074) {
        let m2076 = match v0.clone() {
            Adt19::C0(_, _, h2075) => {
                h2075
            }
        };
        match m2076.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt19::C0(std::rc::Rc::new(f168(v1, match v0.clone() {
                    Adt19::C0(h2077, _, _) => {
                        (*h2077).clone()
                    }
                }, false)), {
                    let a2079 = match v0 {
                        Adt19::C0(_, h2078, _) => {
                            h2078
                        }
                    };
                    let a2080 = 2u64;
                    let a2081 = 0u64;
                    nat_quot(a2079, a2080, a2081)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f228(v0: Adt17, v1: Adt16) -> Adt17 {
    let v2: Adt22 = f251(match v0.clone() {
        Adt17::C0(h2082, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2082
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h2083, _, _, _, _, _, _, _, _, _, _) => {
            h2083
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, h2084, _, _, _, _, _) => {
            h2084
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2085) => {
            h2085
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, h2086, _, _, _, _, _, _, _) => {
            h2086
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h2087, _, _, _, _, _, _) => {
            h2087
        }
    });
    let v3: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, h2088, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2088
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h2089, _, _, _, _, _, _, _, _, _) => {
            h2089
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h2090, _, _, _, _) => {
            h2090
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2091, _, _, _) => {
            h2091
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, h2092, _, _, _, _, _) => {
            h2092
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, h2093, _, _, _, _) => {
            h2093
        }
    });
    let v4: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, _, h2094, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2094
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h2095, _, _, _, _, _, _, _, _) => {
            h2095
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, h2096, _, _, _, _, _, _, _) => {
            h2096
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2097, _, _) => {
            h2097
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2098, _, _, _) => {
            h2098
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2099, _, _) => {
            h2099
        }
    });
    let v5: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, _, _, h2100, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2100
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h2101, _, _, _, _, _, _, _, _, _, _, _) => {
            h2101
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, h2102, _, _, _, _, _, _) => {
            h2102
        }
    }, match v0 {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2103, _) => {
            h2103
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2104, _) => {
            h2104
        }
    }, match v1 {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2105) => {
            h2105
        }
    });
    Adt17::C0(match v2.clone() {
        Adt22::C0(h2106, _, _, _) => {
            h2106
        }
    }, match v3.clone() {
        Adt22::C0(h2107, _, _, _) => {
            h2107
        }
    }, match v4.clone() {
        Adt22::C0(h2108, _, _, _) => {
            h2108
        }
    }, match v5.clone() {
        Adt22::C0(h2109, _, _, _) => {
            h2109
        }
    }, match v5.clone() {
        Adt22::C0(_, h2110, _, _) => {
            h2110
        }
    }, match v2.clone() {
        Adt22::C0(_, h2111, _, _) => {
            h2111
        }
    }, match v3.clone() {
        Adt22::C0(_, h2112, _, _) => {
            h2112
        }
    }, match v4.clone() {
        Adt22::C0(_, h2113, _, _) => {
            h2113
        }
    }, match v4.clone() {
        Adt22::C0(_, _, h2114, _) => {
            h2114
        }
    }, match v5.clone() {
        Adt22::C0(_, _, h2115, _) => {
            h2115
        }
    }, match v2.clone() {
        Adt22::C0(_, _, h2116, _) => {
            h2116
        }
    }, match v3.clone() {
        Adt22::C0(_, _, h2117, _) => {
            h2117
        }
    }, match v3 {
        Adt22::C0(_, _, _, h2118) => {
            h2118
        }
    }, match v4 {
        Adt22::C0(_, _, _, h2119) => {
            h2119
        }
    }, match v5 {
        Adt22::C0(_, _, _, h2120) => {
            h2120
        }
    }, match v2 {
        Adt22::C0(_, _, _, h2121) => {
            h2121
        }
    })
}

pub fn f229(v0: Adt17, v1: Adt16) -> Adt17 {
    let v2: Adt22 = f251(match v0.clone() {
        Adt17::C0(h2122, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2122
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h2123, _, _, _, _, _, _, _, _, _, _, _) => {
            h2123
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, h2124, _, _, _, _, _, _, _) => {
            h2124
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2125, _, _, _) => {
            h2125
        }
    }, match v1.clone() {
        Adt16::C0(h2126, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2126
        }
    }, match v1.clone() {
        Adt16::C0(_, h2127, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2127
        }
    });
    let v3: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, h2128, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2128
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h2129, _, _, _, _, _, _, _, _, _, _) => {
            h2129
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, h2130, _, _, _, _, _, _) => {
            h2130
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2131, _, _) => {
            h2131
        }
    }, match v1.clone() {
        Adt16::C0(_, _, h2132, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2132
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, h2133, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2133
        }
    });
    let v4: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, _, h2134, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2134
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h2135, _, _, _, _, _, _, _, _, _) => {
            h2135
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, h2136, _, _, _, _, _) => {
            h2136
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2137, _) => {
            h2137
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, h2138, _, _, _, _, _, _, _, _, _, _, _) => {
            h2138
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, h2139, _, _, _, _, _, _, _, _, _, _) => {
            h2139
        }
    });
    let v5: Adt22 = f251(match v0.clone() {
        Adt17::C0(_, _, _, h2140, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2140
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h2141, _, _, _, _, _, _, _, _) => {
            h2141
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, h2142, _, _, _, _) => {
            h2142
        }
    }, match v0 {
        Adt17::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2143) => {
            h2143
        }
    }, match v1.clone() {
        Adt16::C0(_, _, _, _, _, _, h2144, _, _, _, _, _, _, _, _, _) => {
            h2144
        }
    }, match v1 {
        Adt16::C0(_, _, _, _, _, _, _, h2145, _, _, _, _, _, _, _, _) => {
            h2145
        }
    });
    Adt17::C0(match v2.clone() {
        Adt22::C0(h2146, _, _, _) => {
            h2146
        }
    }, match v3.clone() {
        Adt22::C0(h2147, _, _, _) => {
            h2147
        }
    }, match v4.clone() {
        Adt22::C0(h2148, _, _, _) => {
            h2148
        }
    }, match v5.clone() {
        Adt22::C0(h2149, _, _, _) => {
            h2149
        }
    }, match v2.clone() {
        Adt22::C0(_, h2150, _, _) => {
            h2150
        }
    }, match v3.clone() {
        Adt22::C0(_, h2151, _, _) => {
            h2151
        }
    }, match v4.clone() {
        Adt22::C0(_, h2152, _, _) => {
            h2152
        }
    }, match v5.clone() {
        Adt22::C0(_, h2153, _, _) => {
            h2153
        }
    }, match v2.clone() {
        Adt22::C0(_, _, h2154, _) => {
            h2154
        }
    }, match v3.clone() {
        Adt22::C0(_, _, h2155, _) => {
            h2155
        }
    }, match v4.clone() {
        Adt22::C0(_, _, h2156, _) => {
            h2156
        }
    }, match v5.clone() {
        Adt22::C0(_, _, h2157, _) => {
            h2157
        }
    }, match v2 {
        Adt22::C0(_, _, _, h2158) => {
            h2158
        }
    }, match v3 {
        Adt22::C0(_, _, _, h2159) => {
            h2159
        }
    }, match v4 {
        Adt22::C0(_, _, _, h2160) => {
            h2160
        }
    }, match v5 {
        Adt22::C0(_, _, _, h2161) => {
            h2161
        }
    })
}

pub fn f230(v0: Adt16) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(_, _, h2162, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2162
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h2163, _, _, _, _, _, _, _, _, _) => {
            h2163
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h2164, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2164
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, h2165, _, _, _, _, _) => {
            h2165
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, h2166, _, _, _, _, _, _, _, _) => {
            h2166
        }
    }, match v0.clone() {
        Adt16::C0(h2167, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2167
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, h2168, _, _, _, _, _, _, _, _, _, _, _) => {
            h2168
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2169, _, _) => {
            h2169
        }
    }, match v0.clone() {
        Adt16::C0(_, h2170, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2170
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, h2171, _, _, _, _) => {
            h2171
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2172, _, _, _) => {
            h2172
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h2173, _, _, _, _, _, _, _, _, _, _) => {
            h2173
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h2174, _, _, _, _, _, _) => {
            h2174
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2175, _) => {
            h2175
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2176) => {
            h2176
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, _, _, _, _, h2177, _, _, _, _, _, _, _) => {
            h2177
        }
    })
}

pub fn f231(v0: Fn38, p2181: bool, p2182: List<u64>) -> bool {
    let m2180 = (p2181, p2182);
    {
        let mut m2180 = m2180;
        loop {
            let step = {
                let (v1, v2) = m2180;
                let m2178 = v2;
                match m2178.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c2179: Fn38 = v0.clone();
                            c2179.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a2183 = v2;
        let a2184 = v0;
        nat_eq(a2183, a2184)
    }
}

pub fn f233(v0: Fn39, p2188: bool, p2189: List<Bytes>) -> bool {
    let m2187 = (p2188, p2189);
    {
        let mut m2187 = m2187;
        loop {
            let step = {
                let (v1, v2) = m2187;
                let m2185 = v2;
                match m2185.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c2186: Fn39 = v0.clone();
                            c2186.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: List<u64>, v1: List<Adt9>, v2: bool, v3: Bytes) -> bool {
    let a2191 = 0u64;
    let a2192 = {
        let a2190 = v3.clone();
        length_bytes(a2190)
    };
    if nat_lt(a2191, a2192) {
        f252(Fn41::F253(v0, v3), v2, v1)
    } else {
        v2
    }
}

pub fn f235(v0: List<Adt0>, v1: u64) -> R<(bool, List<u64>)> {
    let v2: (bool, List<u64>) = f254(Fn42::F255(v1), (true, List::<u64>::nil()), v0)?;
    Ok(({
        let (h2193, _) = v2;
        h2193
    }, f256(Fn43::F257, List::<u64>::nil(), {
        let (_, h2194) = v2;
        h2194
    })))
}

pub fn f236(v0: Bytes, v1: List<Adt0>, v2: u64) -> R<(bool, Bytes)> {
    let a2195 = f12(f4(v1.clone(), 14u64));
    let a2196 = 1u64;
    if nat_eq(a2195, a2196) {
        let v3: (bool, Bytes) = f8(v0, 13u64)?;
        Ok(({
            let (h2197, _) = v3;
            h2197
        }, f11({
            let a2199 = f44(Str::lit("onnx:external-data:v1"));
            let a2200 = {
                let (_, h2198) = v3;
                h2198
            };
            append_bytes(a2199, a2200)
        })?))
    } else {
        let v4: Adt0 = f4(v1.clone(), 9u64);
        let a2202 = match v4.clone() {
            Adt0::C0(_, h2201, _, _) => {
                h2201
            }
        };
        let a2203 = 2u64;
        if if nat_eq(a2202, a2203) {
            let a2206 = 0u64;
            let a2207 = {
                let a2205 = match v4.clone() {
                    Adt0::C0(_, _, _, h2204) => {
                        h2204
                    }
                };
                length_bytes(a2205)
            };
            nat_lt(a2206, a2207)
        } else {
            false
        } {
            Ok((true, f11(match v4 {
                Adt0::C0(_, _, _, h2208) => {
                    h2208
                }
            })?))
        } else {
            let v5: (u64, u64) = f258(v2);
            let a2210 = {
                let (_, h2209) = v5;
                h2209
            };
            let a2211 = 0u64;
            if nat_eq(a2210, a2211) {
                let a2212 = v2;
                let a2213 = 8u64;
                if nat_eq(a2212, a2213) {
                    Ok((true, f11(f177(f20(v1, 6u64))?)?))
                } else {
                    Ok((true, f11(f259(v1, {
                        let (h2214, _) = v5;
                        h2214
                    })?)?))
                }
            } else {
                let v6: (bool, List<u64>) = f235(v1, {
                    let (h2215, _) = v5;
                    h2215
                })?;
                Ok(({
                    let (h2216, _) = v6;
                    h2216
                }, f11(f238({
                    let (_, h2217) = v6;
                    h2217
                }, {
                    let (_, h2218) = v5;
                    h2218
                })?)?))
            }
        }
    }
}

pub fn f237(v0: List<u64>) -> R<u64> {
    f260(Fn44::F261, 0u64, v0)
}

pub fn f238(v0: List<u64>, v1: u64) -> R<Bytes> {
    f262(Fn45::F263(v1), Bytes::lit(&[]), v0)
}

pub fn f239(v0: Adt20) -> R<Option<Adt20>> {
    if match v0.clone() {
        Adt20::C0(_, _, h2219) => {
            h2219
        }
    } {
        Ok(None::<Adt20>)
    } else {
        let m2221 = match v0 {
            Adt20::C0(h2220, _, _) => {
                h2220
            }
        };
        match m2221.uncons() {
            None => {
                Ok(None::<Adt20>)
            }
            Some((v1, v2)) => {
                let m2223 = match v1.clone() {
                    Adt21::C0(h2222, _, _) => {
                        h2222
                    }
                };
                match m2223.uncons() {
                    None => {
                        Ok(Some(f264(v2, f11(match v1 {
                            Adt21::C0(_, h2224, _) => {
                                h2224
                            }
                        })?)))
                    }
                    Some((v3, v4)) => {
                        let v5: Bytes = {
                            let a2226 = match v1.clone() {
                                Adt21::C0(_, h2225, _) => {
                                    h2225
                                }
                            };
                            let a2227 = f265(v3.clone())?;
                            append_bytes(a2226, a2227)
                        };
                        let v6: u64 = match v1 {
                            Adt21::C0(_, _, h2228) => {
                                h2228
                            }
                        };
                        let v7: Bytes = match v3.clone() {
                            Adt0::C0(_, _, _, h2229) => {
                                h2229
                            }
                        };
                        let v8: (bool, List<Adt0>) = f3(v7.clone())?;
                        let v9: bool = {
                            let a2231 = match v3.clone() {
                                Adt0::C0(_, h2230, _, _) => {
                                    h2230
                                }
                            };
                            let a2232 = 2u64;
                            if if nat_eq(a2231, a2232) {
                                let a2233 = v6;
                                let a2234 = 32u64;
                                nat_lt(a2233, a2234)
                            } else {
                                false
                            } {
                                let a2236 = 0u64;
                                let a2237 = {
                                    let a2235 = v7.clone();
                                    length_bytes(a2235)
                                };
                                if nat_lt(a2236, a2237) {
                                    let (h2238, _) = v8;
                                    h2238
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        if v9 {
                            Ok(Some(Adt20::C0(List::cons(Adt21::C0(f203({
                                let (_, h2239) = v8;
                                h2239
                            }), Bytes::lit(&[]), {
                                let a2240 = v6;
                                let a2241 = 1u64;
                                nat_add(a2240, a2241)?
                            }), List::cons(Adt21::C0(v4, v5, v6), v2)), Bytes::lit(&[]), false)))
                        } else {
                            Ok(Some(Adt20::C0(List::cons(Adt21::C0(v4, {
                                let a2243 = match v3 {
                                    Adt0::C0(_, h2242, _, _) => {
                                        h2242
                                    }
                                };
                                let a2244 = 2u64;
                                if nat_eq(a2243, a2244) {
                                    let a2245 = v5;
                                    let a2246 = f11(v7)?;
                                    append_bytes(a2245, a2246)
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

pub fn f240(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    f266(v1, v0)
}

pub fn f241(v0: Fn40, p2250: List<Adt3>, p2251: List<Bytes>) -> R<List<Adt3>> {
    let m2249 = (p2250, p2251);
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let (v1, v2) = m2249;
                let m2247 = v2;
                match m2247.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Bytes>), R<List<Adt3>>>(({
                            let c2248: Fn40 = v0.clone();
                            c2248.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: u64, v1: List<Adt3>, v2: Bytes) -> R<List<Adt3>> {
    Ok(List::cons(Adt3::C0(true, v2, {
        let a2252 = v0;
        let a2253 = 1u64;
        nat_add(a2252, a2253)?
    }), v1))
}

pub fn f243(v0: Bytes, v1: u64) -> R<(bool, Bytes)> {
    let v2: (bool, List<Adt0>) = f3(v0.clone())?;
    let v3: List<Adt0> = {
        let (_, h2254) = v2;
        h2254
    };
    let a2255 = v1;
    let a2256 = 1u64;
    if nat_eq(a2255, a2256) {
        let v4: (bool, Adt0) = f51(v0, 2u64)?;
        Ok(({
            let (h2257, _) = v4.clone();
            h2257
        }, {
            let a2261 = {
                let m2260 = {
                    let (_, h2259) = v4.clone();
                    h2259
                };
                match m2260 {
                    Adt0::C0(_, h2258, _, _) => {
                        h2258
                    }
                }
            };
            let a2262 = 5u64;
            if nat_eq(a2261, a2262) {
                f10({
                    let m2265 = {
                        let (_, h2264) = v4;
                        h2264
                    };
                    match m2265 {
                        Adt0::C0(_, _, h2263, _) => {
                            h2263
                        }
                    }
                }, 4u64)?
            } else {
                Bytes::lit(&[])
            }
        }))
    } else {
        let a2266 = v1;
        let a2267 = 2u64;
        if nat_eq(a2266, a2267) {
            let v5: (bool, Adt0) = f51(v0, 3u64)?;
            Ok(({
                let (h2268, _) = v5;
                h2268
            }, f10(f12({
                let (_, h2269) = v5;
                h2269
            }), 8u64)?))
        } else {
            let a2270 = v1;
            let a2271 = 3u64;
            if nat_eq(a2270, a2271) {
                let v6: (bool, Adt0) = f51(v0, 4u64)?;
                Ok(({
                    let (h2272, _) = v6;
                    h2272
                }, f11(f6({
                    let (_, h2273) = v6;
                    h2273
                }))?))
            } else {
                let a2274 = v1;
                let a2275 = 4u64;
                if nat_eq(a2274, a2275) {
                    let v7: (bool, Adt0) = f51(v0, 5u64)?;
                    let v8: (bool, Bytes) = f200(f6({
                        let (_, h2276) = v7;
                        h2276
                    }))?;
                    Ok(({
                        let (h2277, _) = v7;
                        if h2277 {
                            let (h2278, _) = v8;
                            h2278
                        } else {
                            false
                        }
                    }, {
                        let (_, h2279) = v8;
                        h2279
                    }))
                } else {
                    let a2280 = v1;
                    let a2281 = 6u64;
                    if nat_eq(a2280, a2281) {
                        Ok(({
                            let (h2282, _) = v2;
                            h2282
                        }, f267(v3)?))
                    } else {
                        let a2283 = v1;
                        let a2284 = 7u64;
                        if nat_eq(a2283, a2284) {
                            let v9: (bool, List<u64>) = f235(v3, 8u64)?;
                            Ok(({
                                let (h2285, _) = v2;
                                if h2285 {
                                    let (h2286, _) = v9;
                                    h2286
                                } else {
                                    false
                                }
                            }, f238({
                                let (_, h2287) = v9;
                                h2287
                            }, 8u64)?))
                        } else {
                            let a2288 = v1;
                            let a2289 = 8u64;
                            if nat_eq(a2288, a2289) {
                                Ok(({
                                    let (h2290, _) = v2;
                                    h2290
                                }, f177(f20(v3, 9u64))?))
                            } else {
                                let a2291 = v1;
                                let a2292 = 9u64;
                                if nat_eq(a2291, a2292) {
                                    let v10: (bool, Bytes) = f146(f20(v3, 10u64), 0u64)?;
                                    Ok(({
                                        let (h2293, _) = v2;
                                        if h2293 {
                                            let (h2294, _) = v10;
                                            h2294
                                        } else {
                                            false
                                        }
                                    }, {
                                        let (_, h2295) = v10;
                                        h2295
                                    }))
                                } else {
                                    let a2296 = v1;
                                    let a2297 = 11u64;
                                    if nat_eq(a2296, a2297) {
                                        let v11: (bool, Adt0) = f51(v0, 22u64)?;
                                        let v12: (bool, Bytes) = f175(f6({
                                            let (_, h2298) = v11;
                                            h2298
                                        }))?;
                                        Ok(({
                                            let (h2299, _) = v11;
                                            if h2299 {
                                                let (h2300, _) = v12;
                                                h2300
                                            } else {
                                                false
                                            }
                                        }, {
                                            let (_, h2301) = v12;
                                            h2301
                                        }))
                                    } else {
                                        let a2302 = v1;
                                        let a2303 = 12u64;
                                        if nat_eq(a2302, a2303) {
                                            let v13: (bool, Bytes) = f146(f20(v3, 23u64), 1u64)?;
                                            Ok(({
                                                let (h2304, _) = v2;
                                                if h2304 {
                                                    let (h2305, _) = v13;
                                                    h2305
                                                } else {
                                                    false
                                                }
                                            }, {
                                                let (_, h2306) = v13;
                                                h2306
                                            }))
                                        } else {
                                            let a2307 = v1;
                                            let a2308 = 13u64;
                                            if nat_eq(a2307, a2308) {
                                                let v14: (bool, Adt0) = f51(v0, 14u64)?;
                                                let v15: (bool, Bytes) = f175(f6({
                                                    let (_, h2309) = v14;
                                                    h2309
                                                }))?;
                                                Ok(({
                                                    let (h2310, _) = v14;
                                                    if h2310 {
                                                        let (h2311, _) = v15;
                                                        h2311
                                                    } else {
                                                        false
                                                    }
                                                }, {
                                                    let (_, h2312) = v15;
                                                    h2312
                                                }))
                                            } else {
                                                let a2313 = v1;
                                                let a2314 = 14u64;
                                                if nat_eq(a2313, a2314) {
                                                    let v16: (bool, Bytes) = f146(f20(v3, 15u64), 1u64)?;
                                                    Ok(({
                                                        let (h2315, _) = v2;
                                                        if h2315 {
                                                            let (h2316, _) = v16;
                                                            h2316
                                                        } else {
                                                            false
                                                        }
                                                    }, {
                                                        let (_, h2317) = v16;
                                                        h2317
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

pub fn f244(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2318 = f117(v0, v1);
    let a2319 = f196(v0, v2);
    fixed_u32::bit_or(a2318, a2319)
}

pub fn f245(v0: u32) -> u32 {
    let a2322 = {
        let a2320 = f244(v0, 17u32, 15u32);
        let a2321 = f244(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2320, a2321)
    };
    let a2323 = f117(v0, 10u32);
    fixed_u32::bit_xor(a2322, a2323)
}

pub fn f246(v0: u32) -> u32 {
    let a2326 = {
        let a2324 = f244(v0, 7u32, 25u32);
        let a2325 = f244(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2324, a2325)
    };
    let a2327 = f117(v0, 3u32);
    fixed_u32::bit_xor(a2326, a2327)
}

pub fn f247(v0: (u64, Adt7)) -> R<(u64, Adt7)> {
    Ok(({
        let a2329 = {
            let (h2328, _) = v0.clone();
            h2328
        };
        let a2330 = 1u64;
        nat_add(a2329, a2330)?
    }, f268(f269(f270(f271({
        let (_, h2331) = v0;
        h2331
    }))), {
        let (h2332, _) = v0;
        h2332
    })))
}

pub fn f248(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2333 = f126(v0, v1);
    let a2334 = f188(v0, v2);
    fixed_u64::bit_or(a2333, a2334)
}

pub fn f249(v0: u64) -> u64 {
    let a2337 = {
        let a2335 = f248(v0, 19u32, 45u32);
        let a2336 = f248(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a2335, a2336)
    };
    let a2338 = f126(v0, 6u32);
    fixed_u64::bit_xor(a2337, a2338)
}

pub fn f250(v0: u64) -> u64 {
    let a2341 = {
        let a2339 = f248(v0, 1u32, 63u32);
        let a2340 = f248(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a2339, a2340)
    };
    let a2342 = f126(v0, 7u32);
    fixed_u64::bit_xor(a2341, a2342)
}

pub fn f251(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt22 {
    let v6: u32 = f152(f152(v0, v1), v4);
    let v7: u32 = f244({
        let a2343 = v3;
        let a2344 = v6;
        fixed_u32::bit_xor(a2343, a2344)
    }, 16u32, 16u32);
    let v8: u32 = f152(v2, v7);
    let v9: u32 = f244({
        let a2345 = v1;
        let a2346 = v8;
        fixed_u32::bit_xor(a2345, a2346)
    }, 12u32, 20u32);
    let v10: u32 = f152(f152(v6, v9), v5);
    let v11: u32 = f244({
        let a2347 = v7;
        let a2348 = v10;
        fixed_u32::bit_xor(a2347, a2348)
    }, 8u32, 24u32);
    let v12: u32 = f152(v8, v11);
    let v13: u32 = f244({
        let a2349 = v9;
        let a2350 = v12;
        fixed_u32::bit_xor(a2349, a2350)
    }, 7u32, 25u32);
    Adt22::C0(v10, v13, v12, v11)
}

pub fn f252(v0: Fn41, p2354: bool, p2355: List<Adt9>) -> bool {
    let m2353 = (p2354, p2355);
    {
        let mut m2353 = m2353;
        loop {
            let step = {
                let (v1, v2) = m2353;
                let m2351 = v2;
                match m2351.uncons() {
                    None => {
                        Err::<(bool, List<Adt9>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt9>), bool>(({
                            let c2352: Fn41 = v0.clone();
                            c2352.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: List<u64>, v1: Bytes, v2: bool, v3: Adt9) -> bool {
    if if f272(match v3.clone() {
        Adt9::C0(_, _, _, _, h2356) => {
            h2356
        }
    }, v1) {
        !f198(v0, match v3 {
            Adt9::C0(h2357, _, _, _, _) => {
                h2357
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

pub fn f254(v0: Fn42, p2361: (bool, List<u64>), p2362: List<Adt0>) -> R<(bool, List<u64>)> {
    let m2360 = (p2361, p2362);
    {
        let mut m2360 = m2360;
        loop {
            let step = {
                let (v1, v2) = m2360;
                let m2358 = v2;
                match m2358.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<Adt0>), R<(bool, List<u64>)>>(({
                            let c2359: Fn42 = v0.clone();
                            c2359.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: u64, v1: (bool, List<u64>), v2: Adt0) -> R<(bool, List<u64>)> {
    let a2364 = match v2.clone() {
        Adt0::C0(h2363, _, _, _) => {
            h2363
        }
    };
    let a2365 = v0;
    if nat_eq(a2364, a2365) {
        let a2367 = match v2.clone() {
            Adt0::C0(_, h2366, _, _) => {
                h2366
            }
        };
        let a2368 = 2u64;
        if nat_eq(a2367, a2368) {
            let v3: (bool, List<u64>) = f273(match v2 {
                Adt0::C0(_, _, _, h2369) => {
                    h2369
                }
            })?;
            Ok(({
                let (h2370, _) = v1;
                if h2370 {
                    let (h2371, _) = v3;
                    h2371
                } else {
                    false
                }
            }, f256(Fn43::F274, {
                let (_, h2372) = v1;
                h2372
            }, f256(Fn43::F275, List::<u64>::nil(), {
                let (_, h2373) = v3;
                h2373
            }))))
        } else {
            let a2375 = match v2.clone() {
                Adt0::C0(_, h2374, _, _) => {
                    h2374
                }
            };
            let a2376 = 0u64;
            if nat_eq(a2375, a2376) {
                Ok(({
                    let (h2377, _) = v1;
                    h2377
                }, List::cons(match v2 {
                    Adt0::C0(_, _, h2378, _) => {
                        h2378
                    }
                }, {
                    let (_, h2379) = v1;
                    h2379
                })))
            } else {
                Ok(v1)
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f256(v0: Fn43, p2383: List<u64>, p2384: List<u64>) -> List<u64> {
    let m2382 = (p2383, p2384);
    {
        let mut m2382 = m2382;
        loop {
            let step = {
                let (v1, v2) = m2382;
                let m2380 = v2;
                match m2380.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c2381: Fn43 = v0.clone();
                            c2381.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f258(v0: u64) -> (u64, u64) {
    let a2385 = v0;
    let a2386 = 1u64;
    if if nat_eq(a2385, a2386) {
        true
    } else {
        let a2387 = v0;
        let a2388 = 14u64;
        nat_eq(a2387, a2388)
    } {
        (4u64, 0u64)
    } else {
        let a2389 = v0;
        let a2390 = 11u64;
        if if nat_eq(a2389, a2390) {
            true
        } else {
            let a2391 = v0;
            let a2392 = 15u64;
            nat_eq(a2391, a2392)
        } {
            (10u64, 0u64)
        } else {
            let a2393 = v0;
            let a2394 = 7u64;
            if nat_eq(a2393, a2394) {
                (7u64, 8u64)
            } else {
                let a2395 = v0;
                let a2396 = 13u64;
                if nat_eq(a2395, a2396) {
                    (11u64, 8u64)
                } else {
                    let a2397 = v0;
                    let a2398 = 12u64;
                    if nat_eq(a2397, a2398) {
                        (11u64, 4u64)
                    } else {
                        let a2399 = v0;
                        let a2400 = 6u64;
                        if nat_eq(a2399, a2400) {
                            (5u64, 4u64)
                        } else {
                            let a2401 = v0;
                            let a2402 = 4u64;
                            if if nat_eq(a2401, a2402) {
                                true
                            } else {
                                let a2403 = v0;
                                let a2404 = 5u64;
                                if nat_eq(a2403, a2404) {
                                    true
                                } else {
                                    let a2405 = v0;
                                    let a2406 = 10u64;
                                    if nat_eq(a2405, a2406) {
                                        true
                                    } else {
                                        let a2407 = v0;
                                        let a2408 = 16u64;
                                        nat_eq(a2407, a2408)
                                    }
                                }
                            } {
                                (5u64, 2u64)
                            } else {
                                let a2409 = v0;
                                let a2410 = 8u64;
                                if nat_eq(a2409, a2410) {
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

pub fn f259(v0: List<Adt0>, v1: u64) -> R<Bytes> {
    f276(Fn46::F277(v1), Bytes::lit(&[]), v0)
}

pub fn f260(v0: Fn44, p2414: u64, p2415: List<u64>) -> R<u64> {
    let m2413 = (p2414, p2415);
    {
        let mut m2413 = m2413;
        loop {
            let step = {
                let (v1, v2) = m2413;
                let m2411 = v2;
                match m2411.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c2412: Fn44 = v0.clone();
                            c2412.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: u64, _: u64) -> R<u64> {
    let a2416 = v0;
    let a2417 = 1u64;
    nat_add(a2416, a2417)
}

pub fn f262(v0: Fn45, p2421: Bytes, p2422: List<u64>) -> R<Bytes> {
    let m2420 = (p2421, p2422);
    {
        let mut m2420 = m2420;
        loop {
            let step = {
                let (v1, v2) = m2420;
                let m2418 = v2;
                match m2418.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c2419: Fn45 = v0.clone();
                            c2419.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: u64, v1: Bytes, v2: u64) -> R<Bytes> {
    let a2423 = v1;
    let a2424 = f10(v2, v0)?;
    Ok(append_bytes(a2423, a2424))
}

pub fn f264(v0: List<Adt21>, v1: Bytes) -> Adt20 {
    let m2425 = v0;
    match m2425.uncons() {
        None => {
            Adt20::C0(List::<Adt21>::nil(), v1, true)
        }
        Some((v2, v3)) => {
            Adt20::C0(List::cons(Adt21::C0(match v2.clone() {
                Adt21::C0(h2426, _, _) => {
                    h2426
                }
            }, {
                let a2428 = match v2.clone() {
                    Adt21::C0(_, h2427, _) => {
                        h2427
                    }
                };
                let a2429 = v1;
                append_bytes(a2428, a2429)
            }, match v2 {
                Adt21::C0(_, _, h2430) => {
                    h2430
                }
            }), v3), Bytes::lit(&[]), false)
        }
    }
}

pub fn f265(v0: Adt0) -> R<Bytes> {
    let v1: u64 = match v0.clone() {
        Adt0::C0(_, h2431, _, _) => {
            h2431
        }
    };
    let a2443 = f10(match v0.clone() {
        Adt0::C0(h2432, _, _, _) => {
            h2432
        }
    }, 8u64)?;
    let a2444 = {
        let a2441 = f64(v1);
        let a2442 = {
            let a2433 = v1;
            let a2434 = 0u64;
            if if nat_eq(a2433, a2434) {
                true
            } else {
                let a2435 = v1;
                let a2436 = 1u64;
                nat_eq(a2435, a2436)
            } {
                f10(match v0 {
                    Adt0::C0(_, _, h2437, _) => {
                        h2437
                    }
                }, 8u64)?
            } else {
                let a2438 = v1;
                let a2439 = 5u64;
                if nat_eq(a2438, a2439) {
                    f10(match v0 {
                        Adt0::C0(_, _, h2440, _) => {
                            h2440
                        }
                    }, 4u64)?
                } else {
                    Bytes::lit(&[])
                }
            }
        };
        append_bytes(a2441, a2442)
    };
    Ok(append_bytes(a2443, a2444))
}

pub fn f266(v0: Adt0, v1: List<Adt0>) -> List<Adt0> {
    let v2: Adt23 = f278(Fn47::F279(std::rc::Rc::new(v0.clone())), Adt23::C0(false, List::<Adt0>::nil()), v1);
    if match v2.clone() {
        Adt23::C0(h2445, _) => {
            h2445
        }
    } {
        f16(match v2 {
            Adt23::C0(_, h2446) => {
                h2446
            }
        })
    } else {
        f16(List::cons(v0, match v2 {
            Adt23::C0(_, h2447) => {
                h2447
            }
        }))
    }
}

pub fn f267(v0: List<Adt0>) -> R<Bytes> {
    f276(Fn46::F280, Bytes::lit(&[]), v0)
}

pub fn f268(v0: Adt7, v1: u64) -> Adt7 {
    let v2: u64 = f281(v1);
    Adt7::C0({
        let a2449 = match v0.clone() {
            Adt7::C0(h2448, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2448
            }
        };
        let a2450 = v2;
        fixed_u64::bit_xor(a2449, a2450)
    }, match v0.clone() {
        Adt7::C0(_, h2451, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2451
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h2452, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2452
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h2453, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2453
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h2454, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2454
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h2455, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2455
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h2456, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2456
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h2457, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2457
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h2458, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2458
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h2459, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2459
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2460, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2460
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2461, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2461
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2462, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2462
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2463, _, _, _, _, _, _, _, _, _, _, _) => {
            h2463
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2464, _, _, _, _, _, _, _, _, _, _) => {
            h2464
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2465, _, _, _, _, _, _, _, _, _) => {
            h2465
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2466, _, _, _, _, _, _, _, _) => {
            h2466
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2467, _, _, _, _, _, _, _) => {
            h2467
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2468, _, _, _, _, _, _) => {
            h2468
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2469, _, _, _, _, _) => {
            h2469
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2470, _, _, _, _) => {
            h2470
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2471, _, _, _) => {
            h2471
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2472, _, _) => {
            h2472
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2473, _) => {
            h2473
        }
    }, match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2474) => {
            h2474
        }
    })
}

pub fn f269(v0: Adt7) -> Adt7 {
    Adt7::C0({
        let a2481 = match v0.clone() {
            Adt7::C0(h2475, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2475
            }
        };
        let a2482 = {
            let a2479 = {
                let a2477 = match v0.clone() {
                    Adt7::C0(_, h2476, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2476
                    }
                };
                fixed_u64::bit_not(a2477)
            };
            let a2480 = match v0.clone() {
                Adt7::C0(_, _, h2478, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2478
                }
            };
            fixed_u64::bit_and(a2479, a2480)
        };
        fixed_u64::bit_xor(a2481, a2482)
    }, {
        let a2489 = match v0.clone() {
            Adt7::C0(_, h2483, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2483
            }
        };
        let a2490 = {
            let a2487 = {
                let a2485 = match v0.clone() {
                    Adt7::C0(_, _, h2484, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2484
                    }
                };
                fixed_u64::bit_not(a2485)
            };
            let a2488 = match v0.clone() {
                Adt7::C0(_, _, _, h2486, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2486
                }
            };
            fixed_u64::bit_and(a2487, a2488)
        };
        fixed_u64::bit_xor(a2489, a2490)
    }, {
        let a2497 = match v0.clone() {
            Adt7::C0(_, _, h2491, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2491
            }
        };
        let a2498 = {
            let a2495 = {
                let a2493 = match v0.clone() {
                    Adt7::C0(_, _, _, h2492, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2492
                    }
                };
                fixed_u64::bit_not(a2493)
            };
            let a2496 = match v0.clone() {
                Adt7::C0(_, _, _, _, h2494, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2494
                }
            };
            fixed_u64::bit_and(a2495, a2496)
        };
        fixed_u64::bit_xor(a2497, a2498)
    }, {
        let a2505 = match v0.clone() {
            Adt7::C0(_, _, _, h2499, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2499
            }
        };
        let a2506 = {
            let a2503 = {
                let a2501 = match v0.clone() {
                    Adt7::C0(_, _, _, _, h2500, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2500
                    }
                };
                fixed_u64::bit_not(a2501)
            };
            let a2504 = match v0.clone() {
                Adt7::C0(h2502, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2502
                }
            };
            fixed_u64::bit_and(a2503, a2504)
        };
        fixed_u64::bit_xor(a2505, a2506)
    }, {
        let a2513 = match v0.clone() {
            Adt7::C0(_, _, _, _, h2507, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2507
            }
        };
        let a2514 = {
            let a2511 = {
                let a2509 = match v0.clone() {
                    Adt7::C0(h2508, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2508
                    }
                };
                fixed_u64::bit_not(a2509)
            };
            let a2512 = match v0.clone() {
                Adt7::C0(_, h2510, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2510
                }
            };
            fixed_u64::bit_and(a2511, a2512)
        };
        fixed_u64::bit_xor(a2513, a2514)
    }, {
        let a2521 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, h2515, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2515
            }
        };
        let a2522 = {
            let a2519 = {
                let a2517 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, h2516, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2516
                    }
                };
                fixed_u64::bit_not(a2517)
            };
            let a2520 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, h2518, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2518
                }
            };
            fixed_u64::bit_and(a2519, a2520)
        };
        fixed_u64::bit_xor(a2521, a2522)
    }, {
        let a2529 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, h2523, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2523
            }
        };
        let a2530 = {
            let a2527 = {
                let a2525 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, h2524, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2524
                    }
                };
                fixed_u64::bit_not(a2525)
            };
            let a2528 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, h2526, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2526
                }
            };
            fixed_u64::bit_and(a2527, a2528)
        };
        fixed_u64::bit_xor(a2529, a2530)
    }, {
        let a2537 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h2531, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2531
            }
        };
        let a2538 = {
            let a2535 = {
                let a2533 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, h2532, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2532
                    }
                };
                fixed_u64::bit_not(a2533)
            };
            let a2536 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, h2534, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2534
                }
            };
            fixed_u64::bit_and(a2535, a2536)
        };
        fixed_u64::bit_xor(a2537, a2538)
    }, {
        let a2545 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h2539, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2539
            }
        };
        let a2546 = {
            let a2543 = {
                let a2541 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h2540, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2540
                    }
                };
                fixed_u64::bit_not(a2541)
            };
            let a2544 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, h2542, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2542
                }
            };
            fixed_u64::bit_and(a2543, a2544)
        };
        fixed_u64::bit_xor(a2545, a2546)
    }, {
        let a2553 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h2547, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2547
            }
        };
        let a2554 = {
            let a2551 = {
                let a2549 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, h2548, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2548
                    }
                };
                fixed_u64::bit_not(a2549)
            };
            let a2552 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, h2550, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2550
                }
            };
            fixed_u64::bit_and(a2551, a2552)
        };
        fixed_u64::bit_xor(a2553, a2554)
    }, {
        let a2561 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2555, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2555
            }
        };
        let a2562 = {
            let a2559 = {
                let a2557 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2556, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2556
                    }
                };
                fixed_u64::bit_not(a2557)
            };
            let a2560 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2558, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2558
                }
            };
            fixed_u64::bit_and(a2559, a2560)
        };
        fixed_u64::bit_xor(a2561, a2562)
    }, {
        let a2569 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2563, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2563
            }
        };
        let a2570 = {
            let a2567 = {
                let a2565 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2564, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2564
                    }
                };
                fixed_u64::bit_not(a2565)
            };
            let a2568 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2566, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2566
                }
            };
            fixed_u64::bit_and(a2567, a2568)
        };
        fixed_u64::bit_xor(a2569, a2570)
    }, {
        let a2577 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2571, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2571
            }
        };
        let a2578 = {
            let a2575 = {
                let a2573 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2572, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2572
                    }
                };
                fixed_u64::bit_not(a2573)
            };
            let a2576 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2574, _, _, _, _, _, _, _, _, _, _) => {
                    h2574
                }
            };
            fixed_u64::bit_and(a2575, a2576)
        };
        fixed_u64::bit_xor(a2577, a2578)
    }, {
        let a2585 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2579, _, _, _, _, _, _, _, _, _, _, _) => {
                h2579
            }
        };
        let a2586 = {
            let a2583 = {
                let a2581 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2580, _, _, _, _, _, _, _, _, _, _) => {
                        h2580
                    }
                };
                fixed_u64::bit_not(a2581)
            };
            let a2584 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2582, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2582
                }
            };
            fixed_u64::bit_and(a2583, a2584)
        };
        fixed_u64::bit_xor(a2585, a2586)
    }, {
        let a2593 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2587, _, _, _, _, _, _, _, _, _, _) => {
                h2587
            }
        };
        let a2594 = {
            let a2591 = {
                let a2589 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2588, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2588
                    }
                };
                fixed_u64::bit_not(a2589)
            };
            let a2592 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2590, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2590
                }
            };
            fixed_u64::bit_and(a2591, a2592)
        };
        fixed_u64::bit_xor(a2593, a2594)
    }, {
        let a2601 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2595, _, _, _, _, _, _, _, _, _) => {
                h2595
            }
        };
        let a2602 = {
            let a2599 = {
                let a2597 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2596, _, _, _, _, _, _, _, _) => {
                        h2596
                    }
                };
                fixed_u64::bit_not(a2597)
            };
            let a2600 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2598, _, _, _, _, _, _, _) => {
                    h2598
                }
            };
            fixed_u64::bit_and(a2599, a2600)
        };
        fixed_u64::bit_xor(a2601, a2602)
    }, {
        let a2609 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2603, _, _, _, _, _, _, _, _) => {
                h2603
            }
        };
        let a2610 = {
            let a2607 = {
                let a2605 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2604, _, _, _, _, _, _, _) => {
                        h2604
                    }
                };
                fixed_u64::bit_not(a2605)
            };
            let a2608 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2606, _, _, _, _, _, _) => {
                    h2606
                }
            };
            fixed_u64::bit_and(a2607, a2608)
        };
        fixed_u64::bit_xor(a2609, a2610)
    }, {
        let a2617 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2611, _, _, _, _, _, _, _) => {
                h2611
            }
        };
        let a2618 = {
            let a2615 = {
                let a2613 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2612, _, _, _, _, _, _) => {
                        h2612
                    }
                };
                fixed_u64::bit_not(a2613)
            };
            let a2616 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2614, _, _, _, _, _) => {
                    h2614
                }
            };
            fixed_u64::bit_and(a2615, a2616)
        };
        fixed_u64::bit_xor(a2617, a2618)
    }, {
        let a2625 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2619, _, _, _, _, _, _) => {
                h2619
            }
        };
        let a2626 = {
            let a2623 = {
                let a2621 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2620, _, _, _, _, _) => {
                        h2620
                    }
                };
                fixed_u64::bit_not(a2621)
            };
            let a2624 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2622, _, _, _, _, _, _, _, _, _) => {
                    h2622
                }
            };
            fixed_u64::bit_and(a2623, a2624)
        };
        fixed_u64::bit_xor(a2625, a2626)
    }, {
        let a2633 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2627, _, _, _, _, _) => {
                h2627
            }
        };
        let a2634 = {
            let a2631 = {
                let a2629 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2628, _, _, _, _, _, _, _, _, _) => {
                        h2628
                    }
                };
                fixed_u64::bit_not(a2629)
            };
            let a2632 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2630, _, _, _, _, _, _, _, _) => {
                    h2630
                }
            };
            fixed_u64::bit_and(a2631, a2632)
        };
        fixed_u64::bit_xor(a2633, a2634)
    }, {
        let a2641 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2635, _, _, _, _) => {
                h2635
            }
        };
        let a2642 = {
            let a2639 = {
                let a2637 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2636, _, _, _) => {
                        h2636
                    }
                };
                fixed_u64::bit_not(a2637)
            };
            let a2640 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2638, _, _) => {
                    h2638
                }
            };
            fixed_u64::bit_and(a2639, a2640)
        };
        fixed_u64::bit_xor(a2641, a2642)
    }, {
        let a2649 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2643, _, _, _) => {
                h2643
            }
        };
        let a2650 = {
            let a2647 = {
                let a2645 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2644, _, _) => {
                        h2644
                    }
                };
                fixed_u64::bit_not(a2645)
            };
            let a2648 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2646, _) => {
                    h2646
                }
            };
            fixed_u64::bit_and(a2647, a2648)
        };
        fixed_u64::bit_xor(a2649, a2650)
    }, {
        let a2657 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2651, _, _) => {
                h2651
            }
        };
        let a2658 = {
            let a2655 = {
                let a2653 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2652, _) => {
                        h2652
                    }
                };
                fixed_u64::bit_not(a2653)
            };
            let a2656 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2654) => {
                    h2654
                }
            };
            fixed_u64::bit_and(a2655, a2656)
        };
        fixed_u64::bit_xor(a2657, a2658)
    }, {
        let a2665 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2659, _) => {
                h2659
            }
        };
        let a2666 = {
            let a2663 = {
                let a2661 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2660) => {
                        h2660
                    }
                };
                fixed_u64::bit_not(a2661)
            };
            let a2664 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2662, _, _, _, _) => {
                    h2662
                }
            };
            fixed_u64::bit_and(a2663, a2664)
        };
        fixed_u64::bit_xor(a2665, a2666)
    }, {
        let a2673 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2667) => {
                h2667
            }
        };
        let a2674 = {
            let a2671 = {
                let a2669 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2668, _, _, _, _) => {
                        h2668
                    }
                };
                fixed_u64::bit_not(a2669)
            };
            let a2672 = match v0 {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2670, _, _, _) => {
                    h2670
                }
            };
            fixed_u64::bit_and(a2671, a2672)
        };
        fixed_u64::bit_xor(a2673, a2674)
    })
}

pub fn f270(v0: Adt7) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h2675, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2675
        }
    }, f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h2676, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2676
        }
    }, 44u32, 20u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2677, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2677
        }
    }, 43u32, 21u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2678, _, _, _, _, _, _) => {
            h2678
        }
    }, 21u32, 43u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2679) => {
            h2679
        }
    }, 14u32, 50u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, h2680, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2680
        }
    }, 28u32, 36u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h2681, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2681
        }
    }, 20u32, 44u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2682, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2682
        }
    }, 3u32, 61u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2683, _, _, _, _, _, _, _, _) => {
            h2683
        }
    }, 45u32, 19u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2684, _, _) => {
            h2684
        }
    }, 61u32, 3u32), f282(match v0.clone() {
        Adt7::C0(_, h2685, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2685
        }
    }, 1u32, 63u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h2686, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2686
        }
    }, 6u32, 58u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2687, _, _, _, _, _, _, _, _, _, _, _) => {
            h2687
        }
    }, 25u32, 39u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2688, _, _, _, _, _) => {
            h2688
        }
    }, 8u32, 56u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2689, _, _, _, _) => {
            h2689
        }
    }, 18u32, 46u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, h2690, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2690
        }
    }, 27u32, 37u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, h2691, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2691
        }
    }, 36u32, 28u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2692, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2692
        }
    }, 10u32, 54u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2693, _, _, _, _, _, _, _) => {
            h2693
        }
    }, 15u32, 49u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2694, _) => {
            h2694
        }
    }, 56u32, 8u32), f282(match v0.clone() {
        Adt7::C0(_, _, h2695, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2695
        }
    }, 62u32, 2u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h2696, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2696
        }
    }, 55u32, 9u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2697, _, _, _, _, _, _, _, _, _, _) => {
            h2697
        }
    }, 39u32, 25u32), f282(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2698, _, _, _, _, _, _, _, _, _) => {
            h2698
        }
    }, 41u32, 23u32), f282(match v0 {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2699, _, _, _) => {
            h2699
        }
    }, 2u32, 62u32))
}

pub fn f271(v0: Adt7) -> Adt7 {
    let v1: u64 = {
        let a2711 = {
            let a2708 = {
                let a2705 = {
                    let a2702 = match v0.clone() {
                        Adt7::C0(h2700, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2700
                        }
                    };
                    let a2703 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, h2701, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2701
                        }
                    };
                    fixed_u64::bit_xor(a2702, a2703)
                };
                let a2706 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2704, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2704
                    }
                };
                fixed_u64::bit_xor(a2705, a2706)
            };
            let a2709 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2707, _, _, _, _, _, _, _, _, _) => {
                    h2707
                }
            };
            fixed_u64::bit_xor(a2708, a2709)
        };
        let a2712 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2710, _, _, _, _) => {
                h2710
            }
        };
        fixed_u64::bit_xor(a2711, a2712)
    };
    let v2: u64 = {
        let a2724 = {
            let a2721 = {
                let a2718 = {
                    let a2715 = match v0.clone() {
                        Adt7::C0(_, h2713, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2713
                        }
                    };
                    let a2716 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, h2714, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2714
                        }
                    };
                    fixed_u64::bit_xor(a2715, a2716)
                };
                let a2719 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2717, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2717
                    }
                };
                fixed_u64::bit_xor(a2718, a2719)
            };
            let a2722 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2720, _, _, _, _, _, _, _, _) => {
                    h2720
                }
            };
            fixed_u64::bit_xor(a2721, a2722)
        };
        let a2725 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2723, _, _, _) => {
                h2723
            }
        };
        fixed_u64::bit_xor(a2724, a2725)
    };
    let v3: u64 = {
        let a2737 = {
            let a2734 = {
                let a2731 = {
                    let a2728 = match v0.clone() {
                        Adt7::C0(_, _, h2726, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2726
                        }
                    };
                    let a2729 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, h2727, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2727
                        }
                    };
                    fixed_u64::bit_xor(a2728, a2729)
                };
                let a2732 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2730, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2730
                    }
                };
                fixed_u64::bit_xor(a2731, a2732)
            };
            let a2735 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2733, _, _, _, _, _, _, _) => {
                    h2733
                }
            };
            fixed_u64::bit_xor(a2734, a2735)
        };
        let a2738 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2736, _, _) => {
                h2736
            }
        };
        fixed_u64::bit_xor(a2737, a2738)
    };
    let v4: u64 = {
        let a2750 = {
            let a2747 = {
                let a2744 = {
                    let a2741 = match v0.clone() {
                        Adt7::C0(_, _, _, h2739, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2739
                        }
                    };
                    let a2742 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, _, h2740, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2740
                        }
                    };
                    fixed_u64::bit_xor(a2741, a2742)
                };
                let a2745 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2743, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2743
                    }
                };
                fixed_u64::bit_xor(a2744, a2745)
            };
            let a2748 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2746, _, _, _, _, _, _) => {
                    h2746
                }
            };
            fixed_u64::bit_xor(a2747, a2748)
        };
        let a2751 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2749, _) => {
                h2749
            }
        };
        fixed_u64::bit_xor(a2750, a2751)
    };
    let v5: u64 = {
        let a2763 = {
            let a2760 = {
                let a2757 = {
                    let a2754 = match v0.clone() {
                        Adt7::C0(_, _, _, _, h2752, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2752
                        }
                    };
                    let a2755 = match v0.clone() {
                        Adt7::C0(_, _, _, _, _, _, _, _, _, h2753, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2753
                        }
                    };
                    fixed_u64::bit_xor(a2754, a2755)
                };
                let a2758 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2756, _, _, _, _, _, _, _, _, _, _) => {
                        h2756
                    }
                };
                fixed_u64::bit_xor(a2757, a2758)
            };
            let a2761 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2759, _, _, _, _, _) => {
                    h2759
                }
            };
            fixed_u64::bit_xor(a2760, a2761)
        };
        let a2764 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2762) => {
                h2762
            }
        };
        fixed_u64::bit_xor(a2763, a2764)
    };
    let v6: u64 = {
        let a2765 = v5;
        let a2766 = f282(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a2765, a2766)
    };
    let v7: u64 = {
        let a2767 = v1;
        let a2768 = f282(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a2767, a2768)
    };
    let v8: u64 = {
        let a2769 = v2;
        let a2770 = f282(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a2769, a2770)
    };
    let v9: u64 = {
        let a2771 = v3;
        let a2772 = f282(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a2771, a2772)
    };
    let v10: u64 = {
        let a2773 = v4;
        let a2774 = f282(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a2773, a2774)
    };
    Adt7::C0({
        let a2776 = match v0.clone() {
            Adt7::C0(h2775, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2775
            }
        };
        let a2777 = v6;
        fixed_u64::bit_xor(a2776, a2777)
    }, {
        let a2779 = match v0.clone() {
            Adt7::C0(_, h2778, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2778
            }
        };
        let a2780 = v7;
        fixed_u64::bit_xor(a2779, a2780)
    }, {
        let a2782 = match v0.clone() {
            Adt7::C0(_, _, h2781, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2781
            }
        };
        let a2783 = v8;
        fixed_u64::bit_xor(a2782, a2783)
    }, {
        let a2785 = match v0.clone() {
            Adt7::C0(_, _, _, h2784, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2784
            }
        };
        let a2786 = v9;
        fixed_u64::bit_xor(a2785, a2786)
    }, {
        let a2788 = match v0.clone() {
            Adt7::C0(_, _, _, _, h2787, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2787
            }
        };
        let a2789 = v10;
        fixed_u64::bit_xor(a2788, a2789)
    }, {
        let a2791 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, h2790, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2790
            }
        };
        let a2792 = v6;
        fixed_u64::bit_xor(a2791, a2792)
    }, {
        let a2794 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, h2793, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2793
            }
        };
        let a2795 = v7;
        fixed_u64::bit_xor(a2794, a2795)
    }, {
        let a2797 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, h2796, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2796
            }
        };
        let a2798 = v8;
        fixed_u64::bit_xor(a2797, a2798)
    }, {
        let a2800 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h2799, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2799
            }
        };
        let a2801 = v9;
        fixed_u64::bit_xor(a2800, a2801)
    }, {
        let a2803 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h2802, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2802
            }
        };
        let a2804 = v10;
        fixed_u64::bit_xor(a2803, a2804)
    }, {
        let a2806 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h2805, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2805
            }
        };
        let a2807 = v6;
        fixed_u64::bit_xor(a2806, a2807)
    }, {
        let a2809 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h2808, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2808
            }
        };
        let a2810 = v7;
        fixed_u64::bit_xor(a2809, a2810)
    }, {
        let a2812 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2811, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2811
            }
        };
        let a2813 = v8;
        fixed_u64::bit_xor(a2812, a2813)
    }, {
        let a2815 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2814, _, _, _, _, _, _, _, _, _, _, _) => {
                h2814
            }
        };
        let a2816 = v9;
        fixed_u64::bit_xor(a2815, a2816)
    }, {
        let a2818 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2817, _, _, _, _, _, _, _, _, _, _) => {
                h2817
            }
        };
        let a2819 = v10;
        fixed_u64::bit_xor(a2818, a2819)
    }, {
        let a2821 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2820, _, _, _, _, _, _, _, _, _) => {
                h2820
            }
        };
        let a2822 = v6;
        fixed_u64::bit_xor(a2821, a2822)
    }, {
        let a2824 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2823, _, _, _, _, _, _, _, _) => {
                h2823
            }
        };
        let a2825 = v7;
        fixed_u64::bit_xor(a2824, a2825)
    }, {
        let a2827 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2826, _, _, _, _, _, _, _) => {
                h2826
            }
        };
        let a2828 = v8;
        fixed_u64::bit_xor(a2827, a2828)
    }, {
        let a2830 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2829, _, _, _, _, _, _) => {
                h2829
            }
        };
        let a2831 = v9;
        fixed_u64::bit_xor(a2830, a2831)
    }, {
        let a2833 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2832, _, _, _, _, _) => {
                h2832
            }
        };
        let a2834 = v10;
        fixed_u64::bit_xor(a2833, a2834)
    }, {
        let a2836 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2835, _, _, _, _) => {
                h2835
            }
        };
        let a2837 = v6;
        fixed_u64::bit_xor(a2836, a2837)
    }, {
        let a2839 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2838, _, _, _) => {
                h2838
            }
        };
        let a2840 = v7;
        fixed_u64::bit_xor(a2839, a2840)
    }, {
        let a2842 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2841, _, _) => {
                h2841
            }
        };
        let a2843 = v8;
        fixed_u64::bit_xor(a2842, a2843)
    }, {
        let a2845 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2844, _) => {
                h2844
            }
        };
        let a2846 = v9;
        fixed_u64::bit_xor(a2845, a2846)
    }, {
        let a2848 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2847) => {
                h2847
            }
        };
        let a2849 = v10;
        fixed_u64::bit_xor(a2848, a2849)
    })
}

pub fn f272(v0: List<Bytes>, v1: Bytes) -> bool {
    f233(Fn39::F283(v1), false, v0)
}

pub fn f273(v0: Bytes) -> R<(bool, List<u64>)> {
    let v1: Adt24 = {
        let (h2850, _) = f284(Fn48::F285(v0.clone()), {
            let a2852 = {
                let a2851 = v0;
                length_bytes(a2851)
            };
            let a2853 = 1u64;
            nat_add(a2852, a2853)?
        }, Adt24::C0(0u64, true, List::<u64>::nil()))?;
        h2850
    };
    Ok((match v1.clone() {
        Adt24::C0(_, h2854, _) => {
            h2854
        }
    }, match v1 {
        Adt24::C0(_, _, h2855) => {
            h2855
        }
    }))
}

pub fn f274(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f275(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f276(v0: Fn46, p2859: Bytes, p2860: List<Adt0>) -> R<Bytes> {
    let m2858 = (p2859, p2860);
    {
        let mut m2858 = m2858;
        loop {
            let step = {
                let (v1, v2) = m2858;
                let m2856 = v2;
                match m2856.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt0>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt0>), R<Bytes>>(({
                            let c2857: Fn46 = v0.clone();
                            c2857.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: u64, v1: Bytes, v2: Adt0) -> R<Bytes> {
    let a2862 = match v2.clone() {
        Adt0::C0(h2861, _, _, _) => {
            h2861
        }
    };
    let a2863 = v0;
    if nat_eq(a2862, a2863) {
        let a2865 = match v2.clone() {
            Adt0::C0(_, h2864, _, _) => {
                h2864
            }
        };
        let a2866 = 2u64;
        if nat_eq(a2865, a2866) {
            let a2868 = v1;
            let a2869 = match v2 {
                Adt0::C0(_, _, _, h2867) => {
                    h2867
                }
            };
            Ok(append_bytes(a2868, a2869))
        } else {
            let a2871 = match v2.clone() {
                Adt0::C0(_, h2870, _, _) => {
                    h2870
                }
            };
            let a2872 = 5u64;
            if nat_eq(a2871, a2872) {
                let a2874 = v1;
                let a2875 = f10(match v2 {
                    Adt0::C0(_, _, h2873, _) => {
                        h2873
                    }
                }, 4u64)?;
                Ok(append_bytes(a2874, a2875))
            } else {
                let a2877 = match v2.clone() {
                    Adt0::C0(_, h2876, _, _) => {
                        h2876
                    }
                };
                let a2878 = 1u64;
                if nat_eq(a2877, a2878) {
                    let a2880 = v1;
                    let a2881 = f10(match v2 {
                        Adt0::C0(_, _, h2879, _) => {
                            h2879
                        }
                    }, 8u64)?;
                    Ok(append_bytes(a2880, a2881))
                } else {
                    Ok(v1)
                }
            }
        }
    } else {
        Ok(v1)
    }
}

pub fn f278(v0: Fn47, p2885: Adt23, p2886: List<Adt0>) -> Adt23 {
    let m2884 = (p2885, p2886);
    {
        let mut m2884 = m2884;
        loop {
            let step = {
                let (v1, v2) = m2884;
                let m2882 = v2;
                match m2882.uncons() {
                    None => {
                        Err::<(Adt23, List<Adt0>), Adt23>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt23, List<Adt0>), Adt23>(({
                            let c2883: Fn47 = v0.clone();
                            c2883.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Adt0, v1: Adt23, v2: Adt0) -> Adt23 {
    if match v1.clone() {
        Adt23::C0(h2887, _) => {
            h2887
        }
    } {
        Adt23::C0(true, List::cons(v2, match v1 {
            Adt23::C0(_, h2888) => {
                h2888
            }
        }))
    } else {
        let a2891 = match v0.clone() {
            Adt0::C0(h2889, _, _, _) => {
                h2889
            }
        };
        let a2892 = match v2.clone() {
            Adt0::C0(h2890, _, _, _) => {
                h2890
            }
        };
        if nat_lt(a2891, a2892) {
            Adt23::C0(true, List::cons(v2, List::cons(v0, match v1 {
                Adt23::C0(_, h2893) => {
                    h2893
                }
            })))
        } else {
            Adt23::C0(false, List::cons(v2, match v1 {
                Adt23::C0(_, h2894) => {
                    h2894
                }
            }))
        }
    }
}

pub fn f280(v0: Bytes, v1: Adt0) -> R<Bytes> {
    let a2896 = match v1.clone() {
        Adt0::C0(h2895, _, _, _) => {
            h2895
        }
    };
    let a2897 = 7u64;
    if nat_eq(a2896, a2897) {
        let a2899 = match v1.clone() {
            Adt0::C0(_, h2898, _, _) => {
                h2898
            }
        };
        let a2900 = 2u64;
        if nat_eq(a2899, a2900) {
            let a2902 = v0;
            let a2903 = f11(match v1 {
                Adt0::C0(_, _, _, h2901) => {
                    h2901
                }
            })?;
            Ok(append_bytes(a2902, a2903))
        } else {
            let a2905 = match v1.clone() {
                Adt0::C0(_, h2904, _, _) => {
                    h2904
                }
            };
            let a2906 = 5u64;
            if nat_eq(a2905, a2906) {
                let a2908 = v0;
                let a2909 = f10(match v1 {
                    Adt0::C0(_, _, h2907, _) => {
                        h2907
                    }
                }, 4u64)?;
                Ok(append_bytes(a2908, a2909))
            } else {
                Ok(v0)
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f281(v0: u64) -> u64 {
    let a2910 = v0;
    let a2911 = 12u64;
    if nat_lt(a2910, a2911) {
        let a2912 = v0;
        let a2913 = 6u64;
        if nat_lt(a2912, a2913) {
            let a2914 = v0;
            let a2915 = 3u64;
            if nat_lt(a2914, a2915) {
                let a2916 = v0;
                let a2917 = 1u64;
                if nat_lt(a2916, a2917) {
                    1u64
                } else {
                    let a2918 = v0;
                    let a2919 = 2u64;
                    if nat_lt(a2918, a2919) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2920 = v0;
                let a2921 = 4u64;
                if nat_lt(a2920, a2921) {
                    9223372039002292224u64
                } else {
                    let a2922 = v0;
                    let a2923 = 5u64;
                    if nat_lt(a2922, a2923) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2924 = v0;
            let a2925 = 9u64;
            if nat_lt(a2924, a2925) {
                let a2926 = v0;
                let a2927 = 7u64;
                if nat_lt(a2926, a2927) {
                    9223372039002292353u64
                } else {
                    let a2928 = v0;
                    let a2929 = 8u64;
                    if nat_lt(a2928, a2929) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2930 = v0;
                let a2931 = 10u64;
                if nat_lt(a2930, a2931) {
                    136u64
                } else {
                    let a2932 = v0;
                    let a2933 = 11u64;
                    if nat_lt(a2932, a2933) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2934 = v0;
        let a2935 = 18u64;
        if nat_lt(a2934, a2935) {
            let a2936 = v0;
            let a2937 = 15u64;
            if nat_lt(a2936, a2937) {
                let a2938 = v0;
                let a2939 = 13u64;
                if nat_lt(a2938, a2939) {
                    2147516555u64
                } else {
                    let a2940 = v0;
                    let a2941 = 14u64;
                    if nat_lt(a2940, a2941) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2942 = v0;
                let a2943 = 16u64;
                if nat_lt(a2942, a2943) {
                    9223372036854808579u64
                } else {
                    let a2944 = v0;
                    let a2945 = 17u64;
                    if nat_lt(a2944, a2945) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2946 = v0;
            let a2947 = 21u64;
            if nat_lt(a2946, a2947) {
                let a2948 = v0;
                let a2949 = 19u64;
                if nat_lt(a2948, a2949) {
                    32778u64
                } else {
                    let a2950 = v0;
                    let a2951 = 20u64;
                    if nat_lt(a2950, a2951) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2952 = v0;
                let a2953 = 22u64;
                if nat_lt(a2952, a2953) {
                    9223372036854808704u64
                } else {
                    let a2954 = v0;
                    let a2955 = 23u64;
                    if nat_lt(a2954, a2955) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f282(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2956 = f188(v0, v1);
    let a2957 = f126(v0, v2);
    fixed_u64::bit_or(a2956, a2957)
}

pub fn f283(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a2958 = v2;
        let a2959 = v0;
        equal(a2958, a2959)
    }
}

pub fn f284(v0: Fn48, p2964: u64, p2965: Adt24) -> R<(Adt24, bool)> {
    let m2963 = (p2964, p2965);
    {
        let mut m2963 = m2963;
        loop {
            let step = {
                let (v1, v2) = m2963;
                let m2960 = v1;
                if m2960 == 0 {
                    Err::<(u64, Adt24), R<(Adt24, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2960 - 1;
                    let m2962 = {
                        let c2961: Fn48 = v0.clone();
                        c2961.apply(v2.clone())?
                    };
                    match m2962 {
                        None => {
                            Err::<(u64, Adt24), R<(Adt24, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt24), R<(Adt24, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Bytes, v1: Adt24) -> R<Option<Adt24>> {
    f286(v0, v1)
}

pub fn f286(v0: Bytes, v1: Adt24) -> R<Option<Adt24>> {
    if if match v1.clone() {
        Adt24::C0(_, h2966, _) => {
            h2966
        }
    } {
        let a2969 = match v1.clone() {
            Adt24::C0(h2967, _, _) => {
                h2967
            }
        };
        let a2970 = {
            let a2968 = v0.clone();
            length_bytes(a2968)
        };
        nat_lt(a2969, a2970)
    } else {
        false
    } {
        let v2: (bool, (u64, u64)) = f100(v0, match v1.clone() {
            Adt24::C0(h2971, _, _) => {
                h2971
            }
        })?;
        let (h2972, _) = v2;
        if h2972 {
            Ok(Some(Adt24::C0({
                let (_, h2973) = {
                    let (_, h2974) = v2;
                    h2974
                };
                h2973
            }, true, List::cons({
                let (h2975, _) = {
                    let (_, h2976) = v2;
                    h2976
                };
                h2975
            }, match v1 {
                Adt24::C0(_, _, h2977) => {
                    h2977
                }
            }))))
        } else {
            Ok(Some(Adt24::C0(0u64, false, List::<u64>::nil())))
        }
    } else {
        Ok(None::<Adt24>)
    }
}
