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
    F5(bool),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn0::F5(k0) => f5(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn1::F7(k0, k1, k2) => f7(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn2::F17 => f17(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F19(u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn3::F19(k0, k1, k2) => f19(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F23,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F26,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn5::F26 => f26(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: u64, v2: u64) -> R<Adt0> {
    f1(v0, {
        let a1 = v1;
        let a2 = 1024u64;
        nat_mul(a1, a2)?
    }, {
        let a3 = v2;
        let a4 = 1u64;
        nat_eq(a3, a4)
    })
}

pub fn f1(v0: Bytes, v1: u64, v2: bool) -> R<Adt0> {
    let v3: u64 = {
        let a5 = v0.clone();
        length_bytes(a5)
    };
    let v4: u64 = {
        let a6 = v3;
        let a7 = 0u64;
        if nat_eq(a6, a7) {
            1u64
        } else {
            let a10 = {
                let a8 = v3;
                let a9 = 1023u64;
                nat_add(a8, a9)?
            };
            let a11 = 1024u64;
            let a12 = 0u64;
            nat_quot(a10, a11, a12)
        }
    };
    let v5: u64 = {
        let a13 = v4;
        let a14 = 1u64;
        nat_sub(a13, a14)
    };
    let m16 = {
        let (_, h15) = f2(v0.clone(), v3, v5, v1)?;
        h15
    };
    match m16.uncons() {
        None => {
            f3(v0, v3, v5, {
                let a17 = v1;
                let a18 = v5;
                nat_add(a17, a18)?
            }, v2)
        }
        Some((v6, v7)) => {
            let (_, h19) = f4(Fn0::F5(v2), 64u64, (List::cons(v6, v7), f3(v0, v3, v5, {
                let a20 = v1;
                let a21 = v5;
                nat_add(a20, a21)?
            }, false)?));
            Ok(h19)
        }
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<(u64, List<Adt0>)> {
    f6(Fn1::F7(v0, v3, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f3(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = {
        let a22 = 1024u64;
        let a23 = v2;
        nat_mul(a22, a23)?
    };
    let v6: u64 = f8(v1, v5);
    let v7: u64 = f9(v6)?;
    let v8: u64 = {
        let a24 = v7;
        let a25 = 1u64;
        nat_sub(a24, a25)
    };
    Ok(f10(f11(v0.clone(), v5, v3, v8)?, f12(v0, {
        let a28 = v5;
        let a29 = {
            let a26 = 64u64;
            let a27 = v8;
            nat_mul(a26, a27)?
        };
        nat_add(a28, a29)?
    })?, v3, f13({
        let a32 = v6;
        let a33 = {
            let a30 = 64u64;
            let a31 = v8;
            nat_mul(a30, a31)?
        };
        nat_sub(a32, a33)
    }), {
        let a36 = {
            let a34 = v8;
            let a35 = 0u64;
            if nat_eq(a34, a35) {
                3u32
            } else {
                2u32
            }
        };
        let a37 = if v4 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a36, a37)
    }))
}

pub fn f4(v0: Fn0, p41: u64, p42: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v1, v2) = m40;
                let m38 = v1;
                if m38 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m38 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c39: Fn0 = v0.clone();
                        c39.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: bool, v1: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f14(v1, v0)
}

pub fn f6(v0: Fn1, p46: u64, p47: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m43 = v1;
                if m43 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m43 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c44: Fn1 = v0.clone();
                        c44.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f15(v3.clone(), f3(v0, v2, {
        let (h48, _) = v3.clone();
        h48
    }, {
        let a50 = v1;
        let a51 = {
            let (h49, _) = v3;
            h49
        };
        nat_add(a50, a51)?
    }, false)?)
}

pub fn f8(v0: u64, v1: u64) -> u64 {
    let a54 = 1024u64;
    let a55 = {
        let a52 = v0;
        let a53 = v1;
        nat_sub(a52, a53)
    };
    if nat_lt(a54, a55) {
        1024u64
    } else {
        let a56 = v0;
        let a57 = v1;
        nat_sub(a56, a57)
    }
}

pub fn f9(v0: u64) -> R<u64> {
    let a58 = v0;
    let a59 = 0u64;
    if nat_eq(a58, a59) {
        Ok(1u64)
    } else {
        let a62 = {
            let a60 = v0;
            let a61 = 63u64;
            nat_add(a60, a61)?
        };
        let a63 = 64u64;
        let a64 = 0u64;
        Ok(nat_quot(a62, a63, a64))
    }
}

pub fn f10(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h65, _) = f16(Fn2::F17, 7u64, (Adt2::C0(match v0.clone() {
            Adt0::C0(h66, _, _, _, _, _, _, _) => {
                h66
            }
        }, match v0.clone() {
            Adt0::C0(_, h67, _, _, _, _, _, _) => {
                h67
            }
        }, match v0.clone() {
            Adt0::C0(_, _, h68, _, _, _, _, _) => {
                h68
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, h69, _, _, _, _) => {
                h69
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, h70, _, _, _) => {
                h70
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, h71, _, _) => {
                h71
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h72, _) => {
                h72
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h73) => {
                h73
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f13({
            let a74 = v2;
            let a75 = 4294967296u64;
            let a76 = 0u64;
            nat_rem(a74, a75, a76)
        }), f13({
            let a77 = v2;
            let a78 = 4294967296u64;
            let a79 = 0u64;
            nat_quot(a77, a78, a79)
        }), v3, v4), v1));
        h65
    };
    Adt0::C0({
        let a82 = match v5.clone() {
            Adt2::C0(h80, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        let a83 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h81, _, _, _, _, _, _, _) => {
                h81
            }
        };
        fixed_u32::bit_xor(a82, a83)
    }, {
        let a86 = match v5.clone() {
            Adt2::C0(_, h84, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h84
            }
        };
        let a87 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h85, _, _, _, _, _, _) => {
                h85
            }
        };
        fixed_u32::bit_xor(a86, a87)
    }, {
        let a90 = match v5.clone() {
            Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h89, _, _, _, _, _) => {
                h89
            }
        };
        fixed_u32::bit_xor(a90, a91)
    }, {
        let a94 = match v5.clone() {
            Adt2::C0(_, _, _, h92, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h93, _, _, _, _) => {
                h93
            }
        };
        fixed_u32::bit_xor(a94, a95)
    }, {
        let a98 = match v5.clone() {
            Adt2::C0(_, _, _, _, h96, _, _, _, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a99 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h97, _, _, _) => {
                h97
            }
        };
        fixed_u32::bit_xor(a98, a99)
    }, {
        let a102 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, h100, _, _, _, _, _, _, _, _, _, _) => {
                h100
            }
        };
        let a103 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h101, _, _) => {
                h101
            }
        };
        fixed_u32::bit_xor(a102, a103)
    }, {
        let a106 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, h104, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a107 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h105, _) => {
                h105
            }
        };
        fixed_u32::bit_xor(a106, a107)
    }, {
        let a110 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h108, _, _, _, _, _, _, _, _) => {
                h108
            }
        };
        let a111 = match v5 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h109) => {
                h109
            }
        };
        fixed_u32::bit_xor(a110, a111)
    })
}

pub fn f11(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h112) = f18(Fn3::F19(v2, v0, v1), v3, (0u64, f20()))?;
    Ok(h112)
}

pub fn f12(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(f21(v0.clone(), {
        let a113 = v1;
        let a114 = 0u64;
        nat_add(a113, a114)?
    })?, f21(v0.clone(), {
        let a115 = v1;
        let a116 = 4u64;
        nat_add(a115, a116)?
    })?, f21(v0.clone(), {
        let a117 = v1;
        let a118 = 8u64;
        nat_add(a117, a118)?
    })?, f21(v0.clone(), {
        let a119 = v1;
        let a120 = 12u64;
        nat_add(a119, a120)?
    })?, f21(v0.clone(), {
        let a121 = v1;
        let a122 = 16u64;
        nat_add(a121, a122)?
    })?, f21(v0.clone(), {
        let a123 = v1;
        let a124 = 20u64;
        nat_add(a123, a124)?
    })?, f21(v0.clone(), {
        let a125 = v1;
        let a126 = 24u64;
        nat_add(a125, a126)?
    })?, f21(v0.clone(), {
        let a127 = v1;
        let a128 = 28u64;
        nat_add(a127, a128)?
    })?, f21(v0.clone(), {
        let a129 = v1;
        let a130 = 32u64;
        nat_add(a129, a130)?
    })?, f21(v0.clone(), {
        let a131 = v1;
        let a132 = 36u64;
        nat_add(a131, a132)?
    })?, f21(v0.clone(), {
        let a133 = v1;
        let a134 = 40u64;
        nat_add(a133, a134)?
    })?, f21(v0.clone(), {
        let a135 = v1;
        let a136 = 44u64;
        nat_add(a135, a136)?
    })?, f21(v0.clone(), {
        let a137 = v1;
        let a138 = 48u64;
        nat_add(a137, a138)?
    })?, f21(v0.clone(), {
        let a139 = v1;
        let a140 = 52u64;
        nat_add(a139, a140)?
    })?, f21(v0.clone(), {
        let a141 = v1;
        let a142 = 56u64;
        nat_add(a141, a142)?
    })?, f21(v0, {
        let a143 = v1;
        let a144 = 60u64;
        nat_add(a143, a144)?
    })?))
}

pub fn f13(v0: u64) -> u32 {
    let (_, h145) = {
        let (_, h146) = f22(Fn4::F23, 32u64, (v0, (1u32, 0u32)));
        h146
    };
    h145
}

pub fn f14(v0: (List<Adt0>, Adt0), v1: bool) -> (List<Adt0>, Adt0) {
    let m148 = {
        let (h147, _) = v0.clone();
        h147
    };
    match m148.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            (v3.clone(), f24(v2, {
                let (_, h149) = v0;
                h149
            }, {
                let m150 = v3;
                match m150.uncons() {
                    None => {
                        v1
                    }
                    Some((_, _)) => {
                        false
                    }
                }
            }))
        }
    }
}

pub fn f15(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f25(Fn5::F26, 40u64, Adt3::C0(v1, {
        let a152 = {
            let (h151, _) = v0.clone();
            h151
        };
        let a153 = 1u64;
        nat_add(a152, a153)?
    }, {
        let (_, h154) = v0;
        h154
    }));
    Ok(({
        let a156 = {
            let (h155, _) = v0;
            h155
        };
        let a157 = 1u64;
        nat_add(a156, a157)?
    }, List::cons(match v2.clone() {
        Adt3::C0(h158, _, _) => {
            h158
        }
    }, match v2 {
        Adt3::C0(_, _, h159) => {
            h159
        }
    })))
}

pub fn f16(v0: Fn2, p163: u64, p164: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m160 = v1;
                if m160 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m160 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c161: Fn2 = v0.clone();
                        c161.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f27(v0)
}

pub fn f18(v0: Fn3, p168: u64, p169: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m167 = (p168, p169);
    {
        let mut m167 = m167;
        loop {
            let step = {
                let (v1, v2) = m167;
                let m165 = v1;
                if m165 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m165 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c166: Fn3 = v0.clone();
                        c166.apply(v2)?
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

pub fn f19(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a171 = {
            let (h170, _) = v3.clone();
            h170
        };
        let a172 = 1u64;
        nat_add(a171, a172)?
    }, f10({
        let (_, h173) = v3.clone();
        h173
    }, f12(v1, {
        let a177 = v2;
        let a178 = {
            let a175 = 64u64;
            let a176 = {
                let (h174, _) = v3.clone();
                h174
            };
            nat_mul(a175, a176)?
        };
        nat_add(a177, a178)?
    })?, v0, 64u32, {
        let a180 = {
            let (h179, _) = v3;
            h179
        };
        let a181 = 0u64;
        if nat_eq(a180, a181) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f20() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Bytes, v1: u64) -> R<u32> {
    let a192 = f28(v0.clone(), v1);
    let a193 = {
        let a190 = f29(f28(v0.clone(), {
            let a182 = v1;
            let a183 = 1u64;
            nat_add(a182, a183)?
        }), 8u32);
        let a191 = {
            let a188 = f29(f28(v0.clone(), {
                let a184 = v1;
                let a185 = 2u64;
                nat_add(a184, a185)?
            }), 16u32);
            let a189 = f29(f28(v0, {
                let a186 = v1;
                let a187 = 3u64;
                nat_add(a186, a187)?
            }), 24u32);
            fixed_u32::bit_or(a188, a189)
        };
        fixed_u32::bit_or(a190, a191)
    };
    Ok(fixed_u32::bit_or(a192, a193))
}

pub fn f22(v0: Fn4, p197: u64, p198: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m196 = (p197, p198);
    {
        let mut m196 = m196;
        loop {
            let step = {
                let (v1, v2) = m196;
                let m194 = v1;
                if m194 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m194 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c195: Fn4 = v0.clone();
                        c195.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h199, _) = v0;
        h199
    };
    let v2: u32 = {
        let (h200, _) = {
            let (_, h201) = v0;
            h201
        };
        h200
    };
    let v3: u32 = {
        let (_, h202) = {
            let (_, h203) = v0;
            h203
        };
        h202
    };
    ({
        let a204 = v1;
        let a205 = 2u64;
        let a206 = 0u64;
        nat_quot(a204, a205, a206)
    }, (f29(v2, 1u32), {
        let a210 = {
            let a207 = v1;
            let a208 = 2u64;
            let a209 = 0u64;
            nat_rem(a207, a208, a209)
        };
        let a211 = 1u64;
        if nat_eq(a210, a211) {
            let a212 = v3;
            let a213 = v2;
            fixed_u32::bit_or(a212, a213)
        } else {
            v3
        }
    }))
}

pub fn f24(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f10(f20(), f30(v0, v1), 0u64, 64u32, {
        let a214 = 4u32;
        let a215 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a214, a215)
    })
}

pub fn f25(v0: Fn5, p219: u64, p220: Adt3) -> Adt3 {
    let m218 = (p219, p220);
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, v2) = m218;
                let m216 = v1;
                if m216 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m216 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c217: Fn5 = v0.clone();
                        c217.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt3) -> Adt3 {
    f31(v0)
}

pub fn f27(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (f32(f33({
        let (h221, _) = v0.clone();
        h221
    }, {
        let (_, h222) = v0.clone();
        h222
    }), {
        let (_, h223) = v0.clone();
        h223
    }), f34({
        let (_, h224) = v0;
        h224
    }))
}

pub fn f28(v0: Bytes, v1: u64) -> u32 {
    let m227 = {
        let a225 = v0;
        let a226 = v1;
        index_bytes(a225, a226)
    };
    match m227 {
        None => {
            0u32
        }
        Some(v2) => {
            let m229 = {
                let a228 = v2;
                fixed_u32::convert(i128::from(a228))
            };
            match m229 {
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

pub fn f29(v0: u32, v1: u32) -> u32 {
    let m232 = {
        let a230 = v0;
        let a231 = v1;
        fixed_u32::shift_left(a230, a231)
    };
    match m232 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt0::C0(h233, _, _, _, _, _, _, _) => {
            h233
        }
    }, match v0.clone() {
        Adt0::C0(_, h234, _, _, _, _, _, _) => {
            h234
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h235, _, _, _, _, _) => {
            h235
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h236, _, _, _, _) => {
            h236
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h237, _, _, _) => {
            h237
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h238, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h239, _) => {
            h239
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h240) => {
            h240
        }
    }, match v1.clone() {
        Adt0::C0(h241, _, _, _, _, _, _, _) => {
            h241
        }
    }, match v1.clone() {
        Adt0::C0(_, h242, _, _, _, _, _, _) => {
            h242
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h243, _, _, _, _, _) => {
            h243
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h244, _, _, _, _) => {
            h244
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h245, _, _, _) => {
            h245
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h246, _, _) => {
            h246
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h247, _) => {
            h247
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h248) => {
            h248
        }
    })
}

pub fn f31(v0: Adt3) -> Adt3 {
    let a253 = {
        let a250 = match v0.clone() {
            Adt3::C0(_, h249, _) => {
                h249
            }
        };
        let a251 = 2u64;
        let a252 = 0u64;
        nat_rem(a250, a251, a252)
    };
    let a254 = 0u64;
    if nat_eq(a253, a254) {
        let m256 = match v0.clone() {
            Adt3::C0(_, _, h255) => {
                h255
            }
        };
        match m256.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt3::C0(f24(v1, match v0.clone() {
                    Adt3::C0(h257, _, _) => {
                        h257
                    }
                }, false), {
                    let a259 = match v0 {
                        Adt3::C0(_, h258, _) => {
                            h258
                        }
                    };
                    let a260 = 2u64;
                    let a261 = 0u64;
                    nat_quot(a259, a260, a261)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f32(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f35(match v0.clone() {
        Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h262
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h263, _, _, _, _, _, _, _, _, _, _) => {
            h263
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h264, _, _, _, _, _) => {
            h264
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h265) => {
            h265
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h266, _, _, _, _, _, _, _) => {
            h266
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h267, _, _, _, _, _, _) => {
            h267
        }
    });
    let v3: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, h268, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h268
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h269, _, _, _, _, _, _, _, _, _) => {
            h269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h270, _, _, _, _) => {
            h270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h271, _, _, _) => {
            h271
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h272, _, _, _, _, _) => {
            h272
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h273, _, _, _, _) => {
            h273
        }
    });
    let v4: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, h274, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h274
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h275, _, _, _, _, _, _, _, _) => {
            h275
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h276, _, _, _, _, _, _, _) => {
            h276
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h277, _, _) => {
            h277
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h278, _, _, _) => {
            h278
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h279, _, _) => {
            h279
        }
    });
    let v5: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, _, h280, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h280
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h281, _, _, _, _, _, _, _, _, _, _, _) => {
            h281
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h282, _, _, _, _, _, _) => {
            h282
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h283, _) => {
            h283
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h284, _) => {
            h284
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h285) => {
            h285
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h286, _, _, _) => {
            h286
        }
    }, match v3.clone() {
        Adt4::C0(h287, _, _, _) => {
            h287
        }
    }, match v4.clone() {
        Adt4::C0(h288, _, _, _) => {
            h288
        }
    }, match v5.clone() {
        Adt4::C0(h289, _, _, _) => {
            h289
        }
    }, match v5.clone() {
        Adt4::C0(_, h290, _, _) => {
            h290
        }
    }, match v2.clone() {
        Adt4::C0(_, h291, _, _) => {
            h291
        }
    }, match v3.clone() {
        Adt4::C0(_, h292, _, _) => {
            h292
        }
    }, match v4.clone() {
        Adt4::C0(_, h293, _, _) => {
            h293
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h294, _) => {
            h294
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h295, _) => {
            h295
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h296, _) => {
            h296
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h297, _) => {
            h297
        }
    }, match v3 {
        Adt4::C0(_, _, _, h298) => {
            h298
        }
    }, match v4 {
        Adt4::C0(_, _, _, h299) => {
            h299
        }
    }, match v5 {
        Adt4::C0(_, _, _, h300) => {
            h300
        }
    }, match v2 {
        Adt4::C0(_, _, _, h301) => {
            h301
        }
    })
}

pub fn f33(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f35(match v0.clone() {
        Adt2::C0(h302, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h302
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h303, _, _, _, _, _, _, _, _, _, _, _) => {
            h303
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h304, _, _, _, _, _, _, _) => {
            h304
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h305, _, _, _) => {
            h305
        }
    }, match v1.clone() {
        Adt1::C0(h306, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h306
        }
    }, match v1.clone() {
        Adt1::C0(_, h307, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    });
    let v3: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, h308, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h309, _, _, _, _, _, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h310, _, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h311, _, _) => {
            h311
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h312, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h313, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    });
    let v4: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, h314, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h315, _, _, _, _, _, _, _, _, _) => {
            h315
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h316, _, _, _, _, _) => {
            h316
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h317, _) => {
            h317
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h318, _, _, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h319, _, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    });
    let v5: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, _, h320, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h320
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h321, _, _, _, _, _, _, _, _) => {
            h321
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h322, _, _, _, _) => {
            h322
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h323) => {
            h323
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h324, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h325, _, _, _, _, _, _, _, _) => {
            h325
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h326, _, _, _) => {
            h326
        }
    }, match v3.clone() {
        Adt4::C0(h327, _, _, _) => {
            h327
        }
    }, match v4.clone() {
        Adt4::C0(h328, _, _, _) => {
            h328
        }
    }, match v5.clone() {
        Adt4::C0(h329, _, _, _) => {
            h329
        }
    }, match v2.clone() {
        Adt4::C0(_, h330, _, _) => {
            h330
        }
    }, match v3.clone() {
        Adt4::C0(_, h331, _, _) => {
            h331
        }
    }, match v4.clone() {
        Adt4::C0(_, h332, _, _) => {
            h332
        }
    }, match v5.clone() {
        Adt4::C0(_, h333, _, _) => {
            h333
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h334, _) => {
            h334
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h335, _) => {
            h335
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h336, _) => {
            h336
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h337, _) => {
            h337
        }
    }, match v2 {
        Adt4::C0(_, _, _, h338) => {
            h338
        }
    }, match v3 {
        Adt4::C0(_, _, _, h339) => {
            h339
        }
    }, match v4 {
        Adt4::C0(_, _, _, h340) => {
            h340
        }
    }, match v5 {
        Adt4::C0(_, _, _, h341) => {
            h341
        }
    })
}

pub fn f34(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h342, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h343, _, _, _, _, _, _, _, _, _) => {
            h343
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h344, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h344
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h345, _, _, _, _, _) => {
            h345
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h346, _, _, _, _, _, _, _, _) => {
            h346
        }
    }, match v0.clone() {
        Adt1::C0(h347, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h347
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _, _, _, _, _, _) => {
            h348
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h349, _, _) => {
            h349
        }
    }, match v0.clone() {
        Adt1::C0(_, h350, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h350
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h351, _, _, _, _) => {
            h351
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h352, _, _, _) => {
            h352
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h353, _, _, _, _, _, _, _, _, _, _) => {
            h353
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h354, _, _, _, _, _, _) => {
            h354
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h355, _) => {
            h355
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h356) => {
            h356
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h357, _, _, _, _, _, _, _) => {
            h357
        }
    })
}

pub fn f35(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f36(f36(v0, v1), v4);
    let v7: u32 = f37({
        let a358 = v3;
        let a359 = v6;
        fixed_u32::bit_xor(a358, a359)
    }, 16u32, 16u32);
    let v8: u32 = f36(v2, v7);
    let v9: u32 = f37({
        let a360 = v1;
        let a361 = v8;
        fixed_u32::bit_xor(a360, a361)
    }, 12u32, 20u32);
    let v10: u32 = f36(f36(v6, v9), v5);
    let v11: u32 = f37({
        let a362 = v7;
        let a363 = v10;
        fixed_u32::bit_xor(a362, a363)
    }, 8u32, 24u32);
    let v12: u32 = f36(v8, v11);
    let v13: u32 = f37({
        let a364 = v9;
        let a365 = v12;
        fixed_u32::bit_xor(a364, a365)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f36(v0: u32, v1: u32) -> u32 {
    let m368 = {
        let a366 = f38(v0);
        let a367 = f38(v1);
        fixed_u64::checked_add(a366, a367)
    };
    match m368 {
        None => {
            0u32
        }
        Some(v2) => {
            f39(v2)
        }
    }
}

pub fn f37(v0: u32, v1: u32, v2: u32) -> u32 {
    let a369 = f40(v0, v1);
    let a370 = f29(v0, v2);
    fixed_u32::bit_or(a369, a370)
}

pub fn f38(v0: u32) -> u64 {
    let m372 = {
        let a371 = v0;
        fixed_u64::convert(i128::from(a371))
    };
    match m372 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f39(v0: u64) -> u32 {
    let m376 = {
        let a375 = {
            let a373 = v0;
            let a374 = 4294967295u64;
            fixed_u64::bit_and(a373, a374)
        };
        fixed_u32::convert(i128::from(a375))
    };
    match m376 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f40(v0: u32, v1: u32) -> u32 {
    let m379 = {
        let a377 = v0;
        let a378 = v1;
        fixed_u32::shift_right(a377, a378)
    };
    match m379 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}
