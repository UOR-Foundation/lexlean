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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt0, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn0::F6 => f6(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9(Bytes, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn1::F9(k0, k1) => f9(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn2::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23(u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn3::F23(k0, k1, k2) => f23(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F27,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F30,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn5::F30 => f30(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f1(v0)?;
    let a21 = f2(match v1.clone() {
        Adt0::C0(h1, _, _, _, _, _, _, _) => {
            h1
        }
    });
    let a22 = {
        let a19 = f2(match v1.clone() {
            Adt0::C0(_, h2, _, _, _, _, _, _) => {
                h2
            }
        });
        let a20 = {
            let a17 = f2(match v1.clone() {
                Adt0::C0(_, _, h3, _, _, _, _, _) => {
                    h3
                }
            });
            let a18 = {
                let a15 = f2(match v1.clone() {
                    Adt0::C0(_, _, _, h4, _, _, _, _) => {
                        h4
                    }
                });
                let a16 = {
                    let a13 = f2(match v1.clone() {
                        Adt0::C0(_, _, _, _, h5, _, _, _) => {
                            h5
                        }
                    });
                    let a14 = {
                        let a11 = f2(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h6, _, _) => {
                                h6
                            }
                        });
                        let a12 = {
                            let a9 = f2(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h7, _) => {
                                    h7
                                }
                            });
                            let a10 = f2(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h8) => {
                                    h8
                                }
                            });
                            append_bytes(a9, a10)
                        };
                        append_bytes(a11, a12)
                    };
                    append_bytes(a13, a14)
                };
                append_bytes(a15, a16)
            };
            append_bytes(a17, a18)
        };
        append_bytes(a19, a20)
    };
    Ok(append_bytes(a21, a22))
}

pub fn f1(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a23 = v0.clone();
        length_bytes(a23)
    };
    let v2: u64 = {
        let a24 = v1;
        let a25 = 0u64;
        if nat_eq(a24, a25) {
            1u64
        } else {
            let a28 = {
                let a26 = v1;
                let a27 = 1023u64;
                nat_add(a26, a27)?
            };
            let a29 = 1024u64;
            let a30 = 0u64;
            nat_quot(a28, a29, a30)
        }
    };
    let v3: u64 = {
        let a31 = v2;
        let a32 = 1u64;
        nat_sub(a31, a32)
    };
    let m34 = {
        let (_, h33) = f3(v0.clone(), v1, v3)?;
        h33
    };
    match m34.uncons() {
        None => {
            f4(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h35) = f5(Fn0::F6, 64u64, (List::cons(v4, v5), f4(v0, v1, v3, false)?));
            Ok(h35)
        }
    }
}

pub fn f2(v0: u32) -> Bytes {
    let a48 = {
        let a36 = f7(v0, 4u32);
        let a37 = f7(v0, 0u32);
        append_bytes(a36, a37)
    };
    let a49 = {
        let a46 = {
            let a38 = f7(v0, 12u32);
            let a39 = f7(v0, 8u32);
            append_bytes(a38, a39)
        };
        let a47 = {
            let a44 = {
                let a40 = f7(v0, 20u32);
                let a41 = f7(v0, 16u32);
                append_bytes(a40, a41)
            };
            let a45 = {
                let a42 = f7(v0, 28u32);
                let a43 = f7(v0, 24u32);
                append_bytes(a42, a43)
            };
            append_bytes(a44, a45)
        };
        append_bytes(a46, a47)
    };
    append_bytes(a48, a49)
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f8(Fn1::F9(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f4(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a50 = 1024u64;
        let a51 = v2;
        nat_mul(a50, a51)?
    };
    let v5: u64 = f10(v1, v4);
    let v6: u64 = f11(v5)?;
    let v7: u64 = {
        let a52 = v6;
        let a53 = 1u64;
        nat_sub(a52, a53)
    };
    Ok(f12(f13(v0.clone(), v4, v2, v7)?, f14(v0, {
        let a56 = v4;
        let a57 = {
            let a54 = 64u64;
            let a55 = v7;
            nat_mul(a54, a55)?
        };
        nat_add(a56, a57)?
    })?, v2, f15({
        let a60 = v5;
        let a61 = {
            let a58 = 64u64;
            let a59 = v7;
            nat_mul(a58, a59)?
        };
        nat_sub(a60, a61)
    }), {
        let a64 = {
            let a62 = v7;
            let a63 = 0u64;
            if nat_eq(a62, a63) {
                3u32
            } else {
                2u32
            }
        };
        let a65 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a64, a65)
    }))
}

pub fn f5(v0: Fn0, p69: u64, p70: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m66 = v1;
                if m66 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m66 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c67: Fn0 = v0.clone();
                        c67.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f16(v0)
}

pub fn f7(v0: u32, v1: u32) -> Bytes {
    f17(f18(v0, v1))
}

pub fn f8(v0: Fn1, p74: u64, p75: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m73 = (p74, p75);
    {
        let mut m73 = m73;
        loop {
            let step = {
                let (v1, v2) = m73;
                let m71 = v1;
                if m71 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m71 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c72: Fn1 = v0.clone();
                        c72.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m73 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f19(v2.clone(), f4(v0, v1, {
        let (h76, _) = v2;
        h76
    }, false)?)
}

pub fn f10(v0: u64, v1: u64) -> u64 {
    let a79 = 1024u64;
    let a80 = {
        let a77 = v0;
        let a78 = v1;
        nat_sub(a77, a78)
    };
    if nat_lt(a79, a80) {
        1024u64
    } else {
        let a81 = v0;
        let a82 = v1;
        nat_sub(a81, a82)
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a83 = v0;
    let a84 = 0u64;
    if nat_eq(a83, a84) {
        Ok(1u64)
    } else {
        let a87 = {
            let a85 = v0;
            let a86 = 63u64;
            nat_add(a85, a86)?
        };
        let a88 = 64u64;
        let a89 = 0u64;
        Ok(nat_quot(a87, a88, a89))
    }
}

pub fn f12(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h90, _) = f20(Fn2::F21, 7u64, (Adt2::C0(match v0.clone() {
            Adt0::C0(h91, _, _, _, _, _, _, _) => {
                h91
            }
        }, match v0.clone() {
            Adt0::C0(_, h92, _, _, _, _, _, _) => {
                h92
            }
        }, match v0.clone() {
            Adt0::C0(_, _, h93, _, _, _, _, _) => {
                h93
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, h94, _, _, _, _) => {
                h94
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, h95, _, _, _) => {
                h95
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, h96, _, _) => {
                h96
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h97, _) => {
                h97
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h98) => {
                h98
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f15({
            let a99 = v2;
            let a100 = 4294967296u64;
            let a101 = 0u64;
            nat_rem(a99, a100, a101)
        }), f15({
            let a102 = v2;
            let a103 = 4294967296u64;
            let a104 = 0u64;
            nat_quot(a102, a103, a104)
        }), v3, v4), v1));
        h90
    };
    Adt0::C0({
        let a107 = match v5.clone() {
            Adt2::C0(h105, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h105
            }
        };
        let a108 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h106, _, _, _, _, _, _, _) => {
                h106
            }
        };
        fixed_u32::bit_xor(a107, a108)
    }, {
        let a111 = match v5.clone() {
            Adt2::C0(_, h109, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h109
            }
        };
        let a112 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h110, _, _, _, _, _, _) => {
                h110
            }
        };
        fixed_u32::bit_xor(a111, a112)
    }, {
        let a115 = match v5.clone() {
            Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        };
        let a116 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h114, _, _, _, _, _) => {
                h114
            }
        };
        fixed_u32::bit_xor(a115, a116)
    }, {
        let a119 = match v5.clone() {
            Adt2::C0(_, _, _, h117, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        };
        let a120 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h118, _, _, _, _) => {
                h118
            }
        };
        fixed_u32::bit_xor(a119, a120)
    }, {
        let a123 = match v5.clone() {
            Adt2::C0(_, _, _, _, h121, _, _, _, _, _, _, _, _, _, _, _) => {
                h121
            }
        };
        let a124 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h122, _, _, _) => {
                h122
            }
        };
        fixed_u32::bit_xor(a123, a124)
    }, {
        let a127 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, h125, _, _, _, _, _, _, _, _, _, _) => {
                h125
            }
        };
        let a128 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h126, _, _) => {
                h126
            }
        };
        fixed_u32::bit_xor(a127, a128)
    }, {
        let a131 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, h129, _, _, _, _, _, _, _, _, _) => {
                h129
            }
        };
        let a132 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h130, _) => {
                h130
            }
        };
        fixed_u32::bit_xor(a131, a132)
    }, {
        let a135 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h133, _, _, _, _, _, _, _, _) => {
                h133
            }
        };
        let a136 = match v5 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h134) => {
                h134
            }
        };
        fixed_u32::bit_xor(a135, a136)
    })
}

pub fn f13(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h137) = f22(Fn3::F23(v2, v0, v1), v3, (0u64, f24()))?;
    Ok(h137)
}

pub fn f14(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(f25(v0.clone(), {
        let a138 = v1;
        let a139 = 0u64;
        nat_add(a138, a139)?
    })?, f25(v0.clone(), {
        let a140 = v1;
        let a141 = 4u64;
        nat_add(a140, a141)?
    })?, f25(v0.clone(), {
        let a142 = v1;
        let a143 = 8u64;
        nat_add(a142, a143)?
    })?, f25(v0.clone(), {
        let a144 = v1;
        let a145 = 12u64;
        nat_add(a144, a145)?
    })?, f25(v0.clone(), {
        let a146 = v1;
        let a147 = 16u64;
        nat_add(a146, a147)?
    })?, f25(v0.clone(), {
        let a148 = v1;
        let a149 = 20u64;
        nat_add(a148, a149)?
    })?, f25(v0.clone(), {
        let a150 = v1;
        let a151 = 24u64;
        nat_add(a150, a151)?
    })?, f25(v0.clone(), {
        let a152 = v1;
        let a153 = 28u64;
        nat_add(a152, a153)?
    })?, f25(v0.clone(), {
        let a154 = v1;
        let a155 = 32u64;
        nat_add(a154, a155)?
    })?, f25(v0.clone(), {
        let a156 = v1;
        let a157 = 36u64;
        nat_add(a156, a157)?
    })?, f25(v0.clone(), {
        let a158 = v1;
        let a159 = 40u64;
        nat_add(a158, a159)?
    })?, f25(v0.clone(), {
        let a160 = v1;
        let a161 = 44u64;
        nat_add(a160, a161)?
    })?, f25(v0.clone(), {
        let a162 = v1;
        let a163 = 48u64;
        nat_add(a162, a163)?
    })?, f25(v0.clone(), {
        let a164 = v1;
        let a165 = 52u64;
        nat_add(a164, a165)?
    })?, f25(v0.clone(), {
        let a166 = v1;
        let a167 = 56u64;
        nat_add(a166, a167)?
    })?, f25(v0, {
        let a168 = v1;
        let a169 = 60u64;
        nat_add(a168, a169)?
    })?))
}

pub fn f15(v0: u64) -> u32 {
    let (_, h170) = {
        let (_, h171) = f26(Fn4::F27, 32u64, (v0, (1u32, 0u32)));
        h171
    };
    h170
}

pub fn f16(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m173 = {
        let (h172, _) = v0.clone();
        h172
    };
    match m173.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f28(v1, {
                let (_, h174) = v0;
                h174
            }, {
                let m175 = v2;
                match m175.uncons() {
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

pub fn f17(v0: u32) -> Bytes {
    let a178 = {
        let a176 = v0;
        let a177 = 8u32;
        fixed_u32::bit_and(a176, a177)
    };
    let a179 = 0u32;
    if equal(a178, a179) {
        let a182 = {
            let a180 = v0;
            let a181 = 4u32;
            fixed_u32::bit_and(a180, a181)
        };
        let a183 = 0u32;
        if equal(a182, a183) {
            let a186 = {
                let a184 = v0;
                let a185 = 2u32;
                fixed_u32::bit_and(a184, a185)
            };
            let a187 = 0u32;
            if equal(a186, a187) {
                let a190 = {
                    let a188 = v0;
                    let a189 = 1u32;
                    fixed_u32::bit_and(a188, a189)
                };
                let a191 = 0u32;
                if equal(a190, a191) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a194 = {
                    let a192 = v0;
                    let a193 = 1u32;
                    fixed_u32::bit_and(a192, a193)
                };
                let a195 = 0u32;
                if equal(a194, a195) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a198 = {
                let a196 = v0;
                let a197 = 2u32;
                fixed_u32::bit_and(a196, a197)
            };
            let a199 = 0u32;
            if equal(a198, a199) {
                let a202 = {
                    let a200 = v0;
                    let a201 = 1u32;
                    fixed_u32::bit_and(a200, a201)
                };
                let a203 = 0u32;
                if equal(a202, a203) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a206 = {
                    let a204 = v0;
                    let a205 = 1u32;
                    fixed_u32::bit_and(a204, a205)
                };
                let a207 = 0u32;
                if equal(a206, a207) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a210 = {
            let a208 = v0;
            let a209 = 4u32;
            fixed_u32::bit_and(a208, a209)
        };
        let a211 = 0u32;
        if equal(a210, a211) {
            let a214 = {
                let a212 = v0;
                let a213 = 2u32;
                fixed_u32::bit_and(a212, a213)
            };
            let a215 = 0u32;
            if equal(a214, a215) {
                let a218 = {
                    let a216 = v0;
                    let a217 = 1u32;
                    fixed_u32::bit_and(a216, a217)
                };
                let a219 = 0u32;
                if equal(a218, a219) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a222 = {
                    let a220 = v0;
                    let a221 = 1u32;
                    fixed_u32::bit_and(a220, a221)
                };
                let a223 = 0u32;
                if equal(a222, a223) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a226 = {
                let a224 = v0;
                let a225 = 2u32;
                fixed_u32::bit_and(a224, a225)
            };
            let a227 = 0u32;
            if equal(a226, a227) {
                let a230 = {
                    let a228 = v0;
                    let a229 = 1u32;
                    fixed_u32::bit_and(a228, a229)
                };
                let a231 = 0u32;
                if equal(a230, a231) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a234 = {
                    let a232 = v0;
                    let a233 = 1u32;
                    fixed_u32::bit_and(a232, a233)
                };
                let a235 = 0u32;
                if equal(a234, a235) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f18(v0: u32, v1: u32) -> u32 {
    let m238 = {
        let a236 = v0;
        let a237 = v1;
        fixed_u32::shift_right(a236, a237)
    };
    match m238 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f29(Fn5::F30, 40u64, Adt3::C0(v1, {
        let a240 = {
            let (h239, _) = v0.clone();
            h239
        };
        let a241 = 1u64;
        nat_add(a240, a241)?
    }, {
        let (_, h242) = v0;
        h242
    }));
    Ok(({
        let a244 = {
            let (h243, _) = v0;
            h243
        };
        let a245 = 1u64;
        nat_add(a244, a245)?
    }, List::cons(match v2.clone() {
        Adt3::C0(h246, _, _) => {
            h246
        }
    }, match v2 {
        Adt3::C0(_, _, h247) => {
            h247
        }
    })))
}

pub fn f20(v0: Fn2, p251: u64, p252: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m250 = (p251, p252);
    {
        let mut m250 = m250;
        loop {
            let step = {
                let (v1, v2) = m250;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m248 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c249: Fn2 = v0.clone();
                        c249.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f31(v0)
}

pub fn f22(v0: Fn3, p256: u64, p257: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v1;
                if m253 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m253 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c254: Fn3 = v0.clone();
                        c254.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a259 = {
            let (h258, _) = v3.clone();
            h258
        };
        let a260 = 1u64;
        nat_add(a259, a260)?
    }, f12({
        let (_, h261) = v3.clone();
        h261
    }, f14(v1, {
        let a265 = v2;
        let a266 = {
            let a263 = 64u64;
            let a264 = {
                let (h262, _) = v3.clone();
                h262
            };
            nat_mul(a263, a264)?
        };
        nat_add(a265, a266)?
    })?, v0, 64u32, {
        let a268 = {
            let (h267, _) = v3;
            h267
        };
        let a269 = 0u64;
        if nat_eq(a268, a269) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f24() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f25(v0: Bytes, v1: u64) -> R<u32> {
    let a280 = f32(v0.clone(), v1);
    let a281 = {
        let a278 = f33(f32(v0.clone(), {
            let a270 = v1;
            let a271 = 1u64;
            nat_add(a270, a271)?
        }), 8u32);
        let a279 = {
            let a276 = f33(f32(v0.clone(), {
                let a272 = v1;
                let a273 = 2u64;
                nat_add(a272, a273)?
            }), 16u32);
            let a277 = f33(f32(v0, {
                let a274 = v1;
                let a275 = 3u64;
                nat_add(a274, a275)?
            }), 24u32);
            fixed_u32::bit_or(a276, a277)
        };
        fixed_u32::bit_or(a278, a279)
    };
    Ok(fixed_u32::bit_or(a280, a281))
}

pub fn f26(v0: Fn4, p285: u64, p286: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m282 = v1;
                if m282 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m282 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c283: Fn4 = v0.clone();
                        c283.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h287, _) = v0;
        h287
    };
    let v2: u32 = {
        let (h288, _) = {
            let (_, h289) = v0;
            h289
        };
        h288
    };
    let v3: u32 = {
        let (_, h290) = {
            let (_, h291) = v0;
            h291
        };
        h290
    };
    ({
        let a292 = v1;
        let a293 = 2u64;
        let a294 = 0u64;
        nat_quot(a292, a293, a294)
    }, (f33(v2, 1u32), {
        let a298 = {
            let a295 = v1;
            let a296 = 2u64;
            let a297 = 0u64;
            nat_rem(a295, a296, a297)
        };
        let a299 = 1u64;
        if nat_eq(a298, a299) {
            let a300 = v3;
            let a301 = v2;
            fixed_u32::bit_or(a300, a301)
        } else {
            v3
        }
    }))
}

pub fn f28(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f12(f24(), f34(v0, v1), 0u64, 64u32, {
        let a302 = 4u32;
        let a303 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a302, a303)
    })
}

pub fn f29(v0: Fn5, p307: u64, p308: Adt3) -> Adt3 {
    let m306 = (p307, p308);
    {
        let mut m306 = m306;
        loop {
            let step = {
                let (v1, v2) = m306;
                let m304 = v1;
                if m304 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m304 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c305: Fn5 = v0.clone();
                        c305.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt3) -> Adt3 {
    f35(v0)
}

pub fn f31(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (f36(f37({
        let (h309, _) = v0.clone();
        h309
    }, {
        let (_, h310) = v0.clone();
        h310
    }), {
        let (_, h311) = v0.clone();
        h311
    }), f38({
        let (_, h312) = v0;
        h312
    }))
}

pub fn f32(v0: Bytes, v1: u64) -> u32 {
    let m315 = {
        let a313 = v0;
        let a314 = v1;
        index_bytes(a313, a314)
    };
    match m315 {
        None => {
            0u32
        }
        Some(v2) => {
            let m317 = {
                let a316 = v2;
                fixed_u32::convert(i128::from(a316))
            };
            match m317 {
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

pub fn f33(v0: u32, v1: u32) -> u32 {
    let m320 = {
        let a318 = v0;
        let a319 = v1;
        fixed_u32::shift_left(a318, a319)
    };
    match m320 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f34(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt0::C0(h321, _, _, _, _, _, _, _) => {
            h321
        }
    }, match v0.clone() {
        Adt0::C0(_, h322, _, _, _, _, _, _) => {
            h322
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h323, _, _, _, _, _) => {
            h323
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h324, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h325, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h326, _, _) => {
            h326
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h327, _) => {
            h327
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h328) => {
            h328
        }
    }, match v1.clone() {
        Adt0::C0(h329, _, _, _, _, _, _, _) => {
            h329
        }
    }, match v1.clone() {
        Adt0::C0(_, h330, _, _, _, _, _, _) => {
            h330
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h331, _, _, _, _, _) => {
            h331
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h332, _, _, _, _) => {
            h332
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h333, _, _, _) => {
            h333
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h334, _, _) => {
            h334
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h335, _) => {
            h335
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h336) => {
            h336
        }
    })
}

pub fn f35(v0: Adt3) -> Adt3 {
    let a341 = {
        let a338 = match v0.clone() {
            Adt3::C0(_, h337, _) => {
                h337
            }
        };
        let a339 = 2u64;
        let a340 = 0u64;
        nat_rem(a338, a339, a340)
    };
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        let m344 = match v0.clone() {
            Adt3::C0(_, _, h343) => {
                h343
            }
        };
        match m344.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt3::C0(f28(v1, match v0.clone() {
                    Adt3::C0(h345, _, _) => {
                        h345
                    }
                }, false), {
                    let a347 = match v0 {
                        Adt3::C0(_, h346, _) => {
                            h346
                        }
                    };
                    let a348 = 2u64;
                    let a349 = 0u64;
                    nat_quot(a347, a348, a349)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f36(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f39(match v0.clone() {
        Adt2::C0(h350, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h350
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h351, _, _, _, _, _, _, _, _, _, _) => {
            h351
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h352, _, _, _, _, _) => {
            h352
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h353) => {
            h353
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h354, _, _, _, _, _, _, _) => {
            h354
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h355, _, _, _, _, _, _) => {
            h355
        }
    });
    let v3: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, h356, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h356
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h357, _, _, _, _, _, _, _, _, _) => {
            h357
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h358, _, _, _, _) => {
            h358
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h359, _, _, _) => {
            h359
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h360, _, _, _, _, _) => {
            h360
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h361, _, _, _, _) => {
            h361
        }
    });
    let v4: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, _, h362, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h362
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h363, _, _, _, _, _, _, _, _) => {
            h363
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h364, _, _, _, _, _, _, _) => {
            h364
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h365, _, _) => {
            h365
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h366, _, _, _) => {
            h366
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h367, _, _) => {
            h367
        }
    });
    let v5: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, _, _, h368, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h368
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h369, _, _, _, _, _, _, _, _, _, _, _) => {
            h369
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h370, _, _, _, _, _, _) => {
            h370
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h371, _) => {
            h371
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h372, _) => {
            h372
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h373) => {
            h373
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h374, _, _, _) => {
            h374
        }
    }, match v3.clone() {
        Adt4::C0(h375, _, _, _) => {
            h375
        }
    }, match v4.clone() {
        Adt4::C0(h376, _, _, _) => {
            h376
        }
    }, match v5.clone() {
        Adt4::C0(h377, _, _, _) => {
            h377
        }
    }, match v5.clone() {
        Adt4::C0(_, h378, _, _) => {
            h378
        }
    }, match v2.clone() {
        Adt4::C0(_, h379, _, _) => {
            h379
        }
    }, match v3.clone() {
        Adt4::C0(_, h380, _, _) => {
            h380
        }
    }, match v4.clone() {
        Adt4::C0(_, h381, _, _) => {
            h381
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h382, _) => {
            h382
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h383, _) => {
            h383
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h384, _) => {
            h384
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h385, _) => {
            h385
        }
    }, match v3 {
        Adt4::C0(_, _, _, h386) => {
            h386
        }
    }, match v4 {
        Adt4::C0(_, _, _, h387) => {
            h387
        }
    }, match v5 {
        Adt4::C0(_, _, _, h388) => {
            h388
        }
    }, match v2 {
        Adt4::C0(_, _, _, h389) => {
            h389
        }
    })
}

pub fn f37(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f39(match v0.clone() {
        Adt2::C0(h390, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h390
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h391, _, _, _, _, _, _, _, _, _, _, _) => {
            h391
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h392, _, _, _, _, _, _, _) => {
            h392
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v1.clone() {
        Adt1::C0(h394, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h394
        }
    }, match v1.clone() {
        Adt1::C0(_, h395, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h395
        }
    });
    let v3: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, h396, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h396
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h397, _, _, _, _, _, _, _, _, _, _) => {
            h397
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h398, _, _, _, _, _, _) => {
            h398
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h399, _, _) => {
            h399
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h400, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h400
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h401, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h401
        }
    });
    let v4: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, _, h402, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h402
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h403, _, _, _, _, _, _, _, _, _) => {
            h403
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h404, _, _, _, _, _) => {
            h404
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h405, _) => {
            h405
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h406, _, _, _, _, _, _, _, _, _, _, _) => {
            h406
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h407, _, _, _, _, _, _, _, _, _, _) => {
            h407
        }
    });
    let v5: Adt4 = f39(match v0.clone() {
        Adt2::C0(_, _, _, h408, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h408
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h409, _, _, _, _, _, _, _, _) => {
            h409
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h410, _, _, _, _) => {
            h410
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h411) => {
            h411
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h412, _, _, _, _, _, _, _, _, _) => {
            h412
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h413, _, _, _, _, _, _, _, _) => {
            h413
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h414, _, _, _) => {
            h414
        }
    }, match v3.clone() {
        Adt4::C0(h415, _, _, _) => {
            h415
        }
    }, match v4.clone() {
        Adt4::C0(h416, _, _, _) => {
            h416
        }
    }, match v5.clone() {
        Adt4::C0(h417, _, _, _) => {
            h417
        }
    }, match v2.clone() {
        Adt4::C0(_, h418, _, _) => {
            h418
        }
    }, match v3.clone() {
        Adt4::C0(_, h419, _, _) => {
            h419
        }
    }, match v4.clone() {
        Adt4::C0(_, h420, _, _) => {
            h420
        }
    }, match v5.clone() {
        Adt4::C0(_, h421, _, _) => {
            h421
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h422, _) => {
            h422
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h423, _) => {
            h423
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h424, _) => {
            h424
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h425, _) => {
            h425
        }
    }, match v2 {
        Adt4::C0(_, _, _, h426) => {
            h426
        }
    }, match v3 {
        Adt4::C0(_, _, _, h427) => {
            h427
        }
    }, match v4 {
        Adt4::C0(_, _, _, h428) => {
            h428
        }
    }, match v5 {
        Adt4::C0(_, _, _, h429) => {
            h429
        }
    })
}

pub fn f38(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h430, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h431, _, _, _, _, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h432, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h433, _, _, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h434, _, _, _, _, _, _, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt1::C0(h435, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h435
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h436, _, _, _, _, _, _, _, _, _, _, _) => {
            h436
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h437, _, _) => {
            h437
        }
    }, match v0.clone() {
        Adt1::C0(_, h438, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h438
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h439, _, _, _, _) => {
            h439
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h440, _, _, _) => {
            h440
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h441, _, _, _, _, _, _, _, _, _, _) => {
            h441
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h442, _, _, _, _, _, _) => {
            h442
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h443, _) => {
            h443
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h444) => {
            h444
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h445, _, _, _, _, _, _, _) => {
            h445
        }
    })
}

pub fn f39(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f40(f40(v0, v1), v4);
    let v7: u32 = f41({
        let a446 = v3;
        let a447 = v6;
        fixed_u32::bit_xor(a446, a447)
    }, 16u32, 16u32);
    let v8: u32 = f40(v2, v7);
    let v9: u32 = f41({
        let a448 = v1;
        let a449 = v8;
        fixed_u32::bit_xor(a448, a449)
    }, 12u32, 20u32);
    let v10: u32 = f40(f40(v6, v9), v5);
    let v11: u32 = f41({
        let a450 = v7;
        let a451 = v10;
        fixed_u32::bit_xor(a450, a451)
    }, 8u32, 24u32);
    let v12: u32 = f40(v8, v11);
    let v13: u32 = f41({
        let a452 = v9;
        let a453 = v12;
        fixed_u32::bit_xor(a452, a453)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f40(v0: u32, v1: u32) -> u32 {
    let m456 = {
        let a454 = f42(v0);
        let a455 = f42(v1);
        fixed_u64::checked_add(a454, a455)
    };
    match m456 {
        None => {
            0u32
        }
        Some(v2) => {
            f43(v2)
        }
    }
}

pub fn f41(v0: u32, v1: u32, v2: u32) -> u32 {
    let a457 = f18(v0, v1);
    let a458 = f33(v0, v2);
    fixed_u32::bit_or(a457, a458)
}

pub fn f42(v0: u32) -> u64 {
    let m460 = {
        let a459 = v0;
        fixed_u64::convert(i128::from(a459))
    };
    match m460 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f43(v0: u64) -> u32 {
    let m464 = {
        let a463 = {
            let a461 = v0;
            let a462 = 4294967295u64;
            fixed_u64::bit_and(a461, a462)
        };
        fixed_u32::convert(i128::from(a463))
    };
    match m464 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}
