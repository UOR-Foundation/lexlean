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
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(u64, Bytes, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn0::F4(k0, k1, k2) => f4(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7(u64, u64, u64, Bytes, bool, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F7(k0, k1, k2, k3, k4, k5, k6) => f7(*k0, *k1, *k2, k3.clone(), *k4, *k5, *k6, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F11(bool),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0), p1: Adt0) -> (u64, Adt0) {
        match self {
            Fn2::F11(k0) => f11(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
        match self {
            Fn3::F16 => f16(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F19,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F21,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt1, Adt1)) -> (Adt1, Adt1) {
        match self {
            Fn5::F21 => f21(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Adt0> {
    f1(v0, 0u64, true)
}

pub fn f1(v0: Bytes, v1: u64, v2: bool) -> R<Adt0> {
    let v3: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v4: u64 = {
        let a2 = v3;
        let a3 = 0u64;
        if nat_eq(a2, a3) {
            1u64
        } else {
            let a6 = {
                let a4 = v3;
                let a5 = 1023u64;
                nat_add(a4, a5)?
            };
            let a7 = 1024u64;
            let a8 = 0u64;
            nat_quot(a6, a7, a8)
        }
    };
    let v5: u64 = {
        let a9 = v4;
        let a10 = 1u64;
        nat_sub(a9, a10)
    };
    let a11 = v4;
    let a12 = 1u64;
    if nat_eq(a11, a12) {
        f2(v0, v3, 0u64, v1, v2)
    } else {
        let v6: List<Adt0> = {
            let (_, h13) = f3(Fn0::F4(v1, v0.clone(), v3), v5, (0u64, List::<Adt0>::nil()))?;
            h13
        };
        Ok(f5(v6, f2(v0, v3, v5, {
            let a14 = v1;
            let a15 = v5;
            nat_add(a14, a15)?
        }, false)?, v2))
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = {
        let a16 = v2;
        let a17 = 1024u64;
        nat_mul(a16, a17)?
    };
    let v6: u64 = {
        let a18 = v1;
        let a19 = v5;
        nat_sub(a18, a19)
    };
    let v7: u64 = {
        let a20 = v6;
        let a21 = 1024u64;
        if nat_lt(a20, a21) {
            v6
        } else {
            1024u64
        }
    };
    let v8: u64 = {
        let a22 = v7;
        let a23 = 0u64;
        if nat_eq(a22, a23) {
            1u64
        } else {
            let a26 = {
                let a24 = v7;
                let a25 = 63u64;
                nat_add(a24, a25)?
            };
            let a27 = 64u64;
            let a28 = 0u64;
            nat_quot(a26, a27, a28)
        }
    };
    let (_, h29) = f6(Fn1::F7(v8, v3, v7, v0, v4, v1, v5), v8, (0u64, f8()))?;
    Ok(h29)
}

pub fn f3(v0: Fn0, p33: u64, p34: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v1;
                if m30 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m30 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c31: Fn0 = v0.clone();
                        c31.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: u64, v1: Bytes, v2: u64, v3: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let v4: u64 = {
        let (h35, _) = v3;
        h35
    };
    Ok(({
        let a36 = v4;
        let a37 = 1u64;
        nat_add(a36, a37)?
    }, f9({
        let (_, h38) = v3;
        h38
    }, f2(v1, v2, v4, {
        let a39 = v0;
        let a40 = v4;
        nat_add(a39, a40)?
    }, false)?, {
        let a41 = v4;
        let a42 = 1u64;
        nat_add(a41, a42)?
    })))
}

pub fn f5(v0: List<Adt0>, v1: Adt0, v2: bool) -> Adt0 {
    let (_, h43) = f10(Fn2::F11(v2), ({
        let a44 = v0.clone();
        length_list(a44)
    }, v1), v0);
    h43
}

pub fn f6(v0: Fn1, p48: u64, p49: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m47 = (p48, p49);
    {
        let mut m47 = m47;
        loop {
            let step = {
                let (v1, v2) = m47;
                let m45 = v1;
                if m45 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m45 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c46: Fn1 = v0.clone();
                        c46.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: Bytes, v4: bool, v5: u64, v6: u64, v7: (u64, Adt0)) -> R<(u64, Adt0)> {
    let v8: u64 = {
        let (h50, _) = v7;
        h50
    };
    let v9: bool = {
        let a53 = {
            let a51 = v8;
            let a52 = 1u64;
            nat_add(a51, a52)?
        };
        let a54 = v0;
        nat_eq(a53, a54)
    };
    let v10: u32 = {
        let a57 = {
            let a55 = v8;
            let a56 = 0u64;
            if nat_eq(a55, a56) {
                1u32
            } else {
                0u32
            }
        };
        let a58 = if v9 {
            if v4 {
                10u32
            } else {
                2u32
            }
        } else {
            0u32
        };
        fixed_u32::bit_or(a57, a58)
    };
    let v11: u32 = if v9 {
        f12({
            let a61 = v2;
            let a62 = {
                let a59 = v8;
                let a60 = 64u64;
                nat_mul(a59, a60)?
            };
            nat_sub(a61, a62)
        })
    } else {
        64u32
    };
    Ok(({
        let a63 = v8;
        let a64 = 1u64;
        nat_add(a63, a64)?
    }, f13({
        let (_, h65) = v7;
        h65
    }, f14(v3, v5, {
        let a68 = v6;
        let a69 = {
            let a66 = v8;
            let a67 = 64u64;
            nat_mul(a66, a67)?
        };
        nat_add(a68, a69)?
    })?, v1, v11, v10)))
}

pub fn f8() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f9(v0: List<Adt0>, v1: Adt0, v2: u64) -> List<Adt0> {
    let v3: (u64, (Adt0, List<Adt0>)) = f15(Fn3::F16, 64u64, (v2, (v1, v0)));
    List::cons({
        let (h70, _) = {
            let (_, h71) = v3.clone();
            h71
        };
        h70
    }, {
        let (_, h72) = {
            let (_, h73) = v3;
            h73
        };
        h72
    })
}

pub fn f10(v0: Fn2, p77: (u64, Adt0), p78: List<Adt0>) -> (u64, Adt0) {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, v2) = m76;
                let m74 = v2;
                match m74.uncons() {
                    None => {
                        Err::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(({
                            let c75: Fn2 = v0.clone();
                            c75.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: bool, v1: (u64, Adt0), v2: Adt0) -> (u64, Adt0) {
    ({
        let a80 = {
            let (h79, _) = v1.clone();
            h79
        };
        let a81 = 1u64;
        nat_sub(a80, a81)
    }, f17(v2, {
        let (_, h82) = v1;
        h82
    }, if v0 {
        let a84 = {
            let (h83, _) = v1;
            h83
        };
        let a85 = 1u64;
        nat_eq(a84, a85)
    } else {
        false
    }))
}

pub fn f12(v0: u64) -> u32 {
    let (_, h86) = {
        let (_, h87) = f18(Fn4::F19, 32u64, (v0, (1u32, 0u32)));
        h87
    };
    h86
}

pub fn f13(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt1 = Adt1::C0(match v0.clone() {
        Adt0::C0(h88, _, _, _, _, _, _, _) => {
            h88
        }
    }, match v0.clone() {
        Adt0::C0(_, h89, _, _, _, _, _, _) => {
            h89
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h90, _, _, _, _, _) => {
            h90
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h91, _, _, _, _) => {
            h91
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h92, _, _, _) => {
            h92
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h93, _, _) => {
            h93
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h94, _) => {
            h94
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h95) => {
            h95
        }
    }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f12({
        let a96 = v2;
        let a97 = 4294967296u64;
        let a98 = 0u64;
        nat_rem(a96, a97, a98)
    }), f12({
        let a99 = v2;
        let a100 = 4294967296u64;
        let a101 = 0u64;
        nat_quot(a99, a100, a101)
    }), v3, v4);
    let v6: Adt1 = {
        let (h102, _) = f20(Fn5::F21, 7u64, (v5, v1));
        h102
    };
    Adt0::C0({
        let a105 = match v6.clone() {
            Adt1::C0(h103, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h103
            }
        };
        let a106 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h104, _, _, _, _, _, _, _) => {
                h104
            }
        };
        fixed_u32::bit_xor(a105, a106)
    }, {
        let a109 = match v6.clone() {
            Adt1::C0(_, h107, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h107
            }
        };
        let a110 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h108, _, _, _, _, _, _) => {
                h108
            }
        };
        fixed_u32::bit_xor(a109, a110)
    }, {
        let a113 = match v6.clone() {
            Adt1::C0(_, _, h111, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h111
            }
        };
        let a114 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h112, _, _, _, _, _) => {
                h112
            }
        };
        fixed_u32::bit_xor(a113, a114)
    }, {
        let a117 = match v6.clone() {
            Adt1::C0(_, _, _, h115, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h115
            }
        };
        let a118 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h116, _, _, _, _) => {
                h116
            }
        };
        fixed_u32::bit_xor(a117, a118)
    }, {
        let a121 = match v6.clone() {
            Adt1::C0(_, _, _, _, h119, _, _, _, _, _, _, _, _, _, _, _) => {
                h119
            }
        };
        let a122 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h120, _, _, _) => {
                h120
            }
        };
        fixed_u32::bit_xor(a121, a122)
    }, {
        let a125 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, h123, _, _, _, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a126 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h124, _, _) => {
                h124
            }
        };
        fixed_u32::bit_xor(a125, a126)
    }, {
        let a129 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, h127, _, _, _, _, _, _, _, _, _) => {
                h127
            }
        };
        let a130 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h128, _) => {
                h128
            }
        };
        fixed_u32::bit_xor(a129, a130)
    }, {
        let a133 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h131, _, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a134 = match v6 {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h132) => {
                h132
            }
        };
        fixed_u32::bit_xor(a133, a134)
    })
}

pub fn f14(v0: Bytes, v1: u64, v2: u64) -> R<Adt1> {
    Ok(Adt1::C0(f22(v0.clone(), v1, v2)?, f22(v0.clone(), v1, {
        let a135 = v2;
        let a136 = 4u64;
        nat_add(a135, a136)?
    })?, f22(v0.clone(), v1, {
        let a137 = v2;
        let a138 = 8u64;
        nat_add(a137, a138)?
    })?, f22(v0.clone(), v1, {
        let a139 = v2;
        let a140 = 12u64;
        nat_add(a139, a140)?
    })?, f22(v0.clone(), v1, {
        let a141 = v2;
        let a142 = 16u64;
        nat_add(a141, a142)?
    })?, f22(v0.clone(), v1, {
        let a143 = v2;
        let a144 = 20u64;
        nat_add(a143, a144)?
    })?, f22(v0.clone(), v1, {
        let a145 = v2;
        let a146 = 24u64;
        nat_add(a145, a146)?
    })?, f22(v0.clone(), v1, {
        let a147 = v2;
        let a148 = 28u64;
        nat_add(a147, a148)?
    })?, f22(v0.clone(), v1, {
        let a149 = v2;
        let a150 = 32u64;
        nat_add(a149, a150)?
    })?, f22(v0.clone(), v1, {
        let a151 = v2;
        let a152 = 36u64;
        nat_add(a151, a152)?
    })?, f22(v0.clone(), v1, {
        let a153 = v2;
        let a154 = 40u64;
        nat_add(a153, a154)?
    })?, f22(v0.clone(), v1, {
        let a155 = v2;
        let a156 = 44u64;
        nat_add(a155, a156)?
    })?, f22(v0.clone(), v1, {
        let a157 = v2;
        let a158 = 48u64;
        nat_add(a157, a158)?
    })?, f22(v0.clone(), v1, {
        let a159 = v2;
        let a160 = 52u64;
        nat_add(a159, a160)?
    })?, f22(v0.clone(), v1, {
        let a161 = v2;
        let a162 = 56u64;
        nat_add(a161, a162)?
    })?, f22(v0, v1, {
        let a163 = v2;
        let a164 = 60u64;
        nat_add(a163, a164)?
    })?))
}

pub fn f15(v0: Fn3, p168: u64, p169: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let m167 = (p168, p169);
    {
        let mut m167 = m167;
        loop {
            let step = {
                let (v1, v2) = m167;
                let m165 = v1;
                if m165 == 0 {
                    Err::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>(v2)
                } else {
                    let v3 = m165 - 1;
                    Ok::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>((v3, {
                        let c166: Fn3 = v0.clone();
                        c166.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let v1: u64 = {
        let (h170, _) = v0.clone();
        h170
    };
    let v2: Adt0 = {
        let (h171, _) = {
            let (_, h172) = v0.clone();
            h172
        };
        h171
    };
    let v3: List<Adt0> = {
        let (_, h173) = {
            let (_, h174) = v0.clone();
            h174
        };
        h173
    };
    let a178 = {
        let a175 = v1;
        let a176 = 2u64;
        let a177 = 0u64;
        nat_rem(a175, a176, a177)
    };
    let a179 = 0u64;
    if nat_eq(a178, a179) {
        let m180 = v3;
        match m180.uncons() {
            None => {
                v0
            }
            Some((v4, v5)) => {
                ({
                    let a181 = v1;
                    let a182 = 2u64;
                    let a183 = 0u64;
                    nat_quot(a181, a182, a183)
                }, (f17(v4, v2, false), v5))
            }
        }
    } else {
        v0
    }
}

pub fn f17(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f13(f8(), Adt1::C0(match v0.clone() {
        Adt0::C0(h184, _, _, _, _, _, _, _) => {
            h184
        }
    }, match v0.clone() {
        Adt0::C0(_, h185, _, _, _, _, _, _) => {
            h185
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h186, _, _, _, _, _) => {
            h186
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h187, _, _, _, _) => {
            h187
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h188, _, _, _) => {
            h188
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h189, _, _) => {
            h189
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h190, _) => {
            h190
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h191) => {
            h191
        }
    }, match v1.clone() {
        Adt0::C0(h192, _, _, _, _, _, _, _) => {
            h192
        }
    }, match v1.clone() {
        Adt0::C0(_, h193, _, _, _, _, _, _) => {
            h193
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h194, _, _, _, _, _) => {
            h194
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h195, _, _, _, _) => {
            h195
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h196, _, _, _) => {
            h196
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h197, _, _) => {
            h197
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h198, _) => {
            h198
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h199) => {
            h199
        }
    }), 0u64, 64u32, if v2 {
        12u32
    } else {
        4u32
    })
}

pub fn f18(v0: Fn4, p203: u64, p204: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m202 = (p203, p204);
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, v2) = m202;
                let m200 = v1;
                if m200 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m200 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c201: Fn4 = v0.clone();
                        c201.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h205, _) = v0;
        h205
    };
    let v2: u32 = {
        let (h206, _) = {
            let (_, h207) = v0;
            h207
        };
        h206
    };
    let v3: u32 = {
        let (_, h208) = {
            let (_, h209) = v0;
            h209
        };
        h208
    };
    ({
        let a210 = v1;
        let a211 = 2u64;
        let a212 = 0u64;
        nat_quot(a210, a211, a212)
    }, (f23(v2, 1u32), {
        let a216 = {
            let a213 = v1;
            let a214 = 2u64;
            let a215 = 0u64;
            nat_rem(a213, a214, a215)
        };
        let a217 = 1u64;
        if nat_eq(a216, a217) {
            let a218 = v3;
            let a219 = v2;
            fixed_u32::bit_or(a218, a219)
        } else {
            v3
        }
    }))
}

pub fn f20(v0: Fn5, p223: u64, p224: (Adt1, Adt1)) -> (Adt1, Adt1) {
    let m222 = (p223, p224);
    {
        let mut m222 = m222;
        loop {
            let step = {
                let (v1, v2) = m222;
                let m220 = v1;
                if m220 == 0 {
                    Err::<(u64, (Adt1, Adt1)), (Adt1, Adt1)>(v2)
                } else {
                    let v3 = m220 - 1;
                    Ok::<(u64, (Adt1, Adt1)), (Adt1, Adt1)>((v3, {
                        let c221: Fn5 = v0.clone();
                        c221.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: (Adt1, Adt1)) -> (Adt1, Adt1) {
    (f24({
        let (h225, _) = v0.clone();
        h225
    }, {
        let (_, h226) = v0.clone();
        h226
    }), f25({
        let (_, h227) = v0;
        h227
    }))
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<u32> {
    let a238 = f26(v0.clone(), v1, v2);
    let a239 = {
        let a236 = f23(f26(v0.clone(), v1, {
            let a228 = v2;
            let a229 = 1u64;
            nat_add(a228, a229)?
        }), 8u32);
        let a237 = {
            let a234 = f23(f26(v0.clone(), v1, {
                let a230 = v2;
                let a231 = 2u64;
                nat_add(a230, a231)?
            }), 16u32);
            let a235 = f23(f26(v0, v1, {
                let a232 = v2;
                let a233 = 3u64;
                nat_add(a232, a233)?
            }), 24u32);
            fixed_u32::bit_or(a234, a235)
        };
        fixed_u32::bit_or(a236, a237)
    };
    Ok(fixed_u32::bit_or(a238, a239))
}

pub fn f23(v0: u32, v1: u32) -> u32 {
    let m242 = {
        let a240 = v0;
        let a241 = v1;
        fixed_u32::shift_left(a240, a241)
    };
    match m242 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> Adt1 {
    let v2: Adt2 = f27(match v0.clone() {
        Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h244, _, _, _, _, _, _, _, _, _, _, _) => {
            h244
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h245, _, _, _, _, _, _, _) => {
            h245
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h246, _, _, _) => {
            h246
        }
    }, match v1.clone() {
        Adt1::C0(h247, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h247
        }
    }, match v1.clone() {
        Adt1::C0(_, h248, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h248
        }
    });
    let v3: Adt2 = f27(match v0.clone() {
        Adt1::C0(_, h249, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h249
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h250, _, _, _, _, _, _, _, _, _, _) => {
            h250
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h251, _, _, _, _, _, _) => {
            h251
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h252, _, _) => {
            h252
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h253, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h253
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h254, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h254
        }
    });
    let v4: Adt2 = f27(match v0.clone() {
        Adt1::C0(_, _, h255, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h255
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h256, _, _, _, _, _, _, _, _, _) => {
            h256
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h257, _, _, _, _, _) => {
            h257
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h258, _) => {
            h258
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h259, _, _, _, _, _, _, _, _, _, _, _) => {
            h259
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h260, _, _, _, _, _, _, _, _, _, _) => {
            h260
        }
    });
    let v5: Adt2 = f27(match v0.clone() {
        Adt1::C0(_, _, _, h261, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h261
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h262, _, _, _, _, _, _, _, _) => {
            h262
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h263, _, _, _, _) => {
            h263
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h264) => {
            h264
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h265, _, _, _, _, _, _, _, _, _) => {
            h265
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h266, _, _, _, _, _, _, _, _) => {
            h266
        }
    });
    let v6: Adt2 = f27(match v2.clone() {
        Adt2::C0(h267, _, _, _) => {
            h267
        }
    }, match v3.clone() {
        Adt2::C0(_, h268, _, _) => {
            h268
        }
    }, match v4.clone() {
        Adt2::C0(_, _, h269, _) => {
            h269
        }
    }, match v5.clone() {
        Adt2::C0(_, _, _, h270) => {
            h270
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h271, _, _, _, _, _, _, _) => {
            h271
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h272, _, _, _, _, _, _) => {
            h272
        }
    });
    let v7: Adt2 = f27(match v3.clone() {
        Adt2::C0(h273, _, _, _) => {
            h273
        }
    }, match v4.clone() {
        Adt2::C0(_, h274, _, _) => {
            h274
        }
    }, match v5.clone() {
        Adt2::C0(_, _, h275, _) => {
            h275
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h276) => {
            h276
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h277, _, _, _, _, _) => {
            h277
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h278, _, _, _, _) => {
            h278
        }
    });
    let v8: Adt2 = f27(match v4.clone() {
        Adt2::C0(h279, _, _, _) => {
            h279
        }
    }, match v5.clone() {
        Adt2::C0(_, h280, _, _) => {
            h280
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h281, _) => {
            h281
        }
    }, match v3.clone() {
        Adt2::C0(_, _, _, h282) => {
            h282
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h283, _, _, _) => {
            h283
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h284, _, _) => {
            h284
        }
    });
    let v9: Adt2 = f27(match v5 {
        Adt2::C0(h285, _, _, _) => {
            h285
        }
    }, match v2 {
        Adt2::C0(_, h286, _, _) => {
            h286
        }
    }, match v3 {
        Adt2::C0(_, _, h287, _) => {
            h287
        }
    }, match v4 {
        Adt2::C0(_, _, _, h288) => {
            h288
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h289, _) => {
            h289
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h290) => {
            h290
        }
    });
    Adt1::C0(match v6.clone() {
        Adt2::C0(h291, _, _, _) => {
            h291
        }
    }, match v7.clone() {
        Adt2::C0(h292, _, _, _) => {
            h292
        }
    }, match v8.clone() {
        Adt2::C0(h293, _, _, _) => {
            h293
        }
    }, match v9.clone() {
        Adt2::C0(h294, _, _, _) => {
            h294
        }
    }, match v9.clone() {
        Adt2::C0(_, h295, _, _) => {
            h295
        }
    }, match v6.clone() {
        Adt2::C0(_, h296, _, _) => {
            h296
        }
    }, match v7.clone() {
        Adt2::C0(_, h297, _, _) => {
            h297
        }
    }, match v8.clone() {
        Adt2::C0(_, h298, _, _) => {
            h298
        }
    }, match v8.clone() {
        Adt2::C0(_, _, h299, _) => {
            h299
        }
    }, match v9.clone() {
        Adt2::C0(_, _, h300, _) => {
            h300
        }
    }, match v6.clone() {
        Adt2::C0(_, _, h301, _) => {
            h301
        }
    }, match v7.clone() {
        Adt2::C0(_, _, h302, _) => {
            h302
        }
    }, match v7 {
        Adt2::C0(_, _, _, h303) => {
            h303
        }
    }, match v8 {
        Adt2::C0(_, _, _, h304) => {
            h304
        }
    }, match v9 {
        Adt2::C0(_, _, _, h305) => {
            h305
        }
    }, match v6 {
        Adt2::C0(_, _, _, h306) => {
            h306
        }
    })
}

pub fn f25(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h307, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h308, _, _, _, _, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h309, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h310, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h311, _, _, _, _, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt1::C0(h312, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h313, _, _, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h314, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt1::C0(_, h315, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h315
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h316, _, _, _, _) => {
            h316
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h317, _, _, _) => {
            h317
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h318, _, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h319, _, _, _, _, _, _) => {
            h319
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h320, _) => {
            h320
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h321) => {
            h321
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h322, _, _, _, _, _, _, _) => {
            h322
        }
    })
}

pub fn f26(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a323 = v2;
    let a324 = v1;
    if nat_lt(a323, a324) {
        let m327 = {
            let a325 = v0;
            let a326 = v2;
            index_bytes(a325, a326)
        };
        match m327 {
            None => {
                0u32
            }
            Some(v3) => {
                let m329 = {
                    let a328 = v3;
                    fixed_u32::convert(i128::from(a328))
                };
                match m329 {
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
        0u32
    }
}

pub fn f27(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt2 {
    let v6: u32 = f28(f28(v0, v1), v4);
    let v7: u32 = f29({
        let a330 = v3;
        let a331 = v6;
        fixed_u32::bit_xor(a330, a331)
    }, 16u32, 16u32);
    let v8: u32 = f28(v2, v7);
    let v9: u32 = f29({
        let a332 = v1;
        let a333 = v8;
        fixed_u32::bit_xor(a332, a333)
    }, 12u32, 20u32);
    let v10: u32 = f28(f28(v6, v9), v5);
    let v11: u32 = f29({
        let a334 = v7;
        let a335 = v10;
        fixed_u32::bit_xor(a334, a335)
    }, 8u32, 24u32);
    let v12: u32 = f28(v8, v11);
    let v13: u32 = f29({
        let a336 = v9;
        let a337 = v12;
        fixed_u32::bit_xor(a336, a337)
    }, 7u32, 25u32);
    Adt2::C0(v10, v13, v12, v11)
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m340 = {
        let a338 = f30(v0);
        let a339 = f30(v1);
        fixed_u64::checked_add(a338, a339)
    };
    match m340 {
        None => {
            0u32
        }
        Some(v2) => {
            f31(v2)
        }
    }
}

pub fn f29(v0: u32, v1: u32, v2: u32) -> u32 {
    let a341 = f32(v0, v1);
    let a342 = f23(v0, v2);
    fixed_u32::bit_or(a341, a342)
}

pub fn f30(v0: u32) -> u64 {
    let m344 = {
        let a343 = v0;
        fixed_u64::convert(i128::from(a343))
    };
    match m344 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f31(v0: u64) -> u32 {
    let m348 = {
        let a347 = {
            let a345 = v0;
            let a346 = 4294967295u64;
            fixed_u64::bit_and(a345, a346)
        };
        fixed_u32::convert(i128::from(a347))
    };
    match m348 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: u32, v1: u32) -> u32 {
    let m351 = {
        let a349 = v0;
        let a350 = v1;
        fixed_u32::shift_right(a349, a350)
    };
    match m351 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}
