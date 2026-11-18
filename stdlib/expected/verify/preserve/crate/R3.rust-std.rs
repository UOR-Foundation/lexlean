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
    F2,
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (List<Adt0>, Option<Adt0>)), p1: Adt0) -> R<(u64, (List<Adt0>, Option<Adt0>))> {
        match self {
            Fn0::F2 => f2(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8(bool),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0), p1: Adt0) -> (u64, Adt0) {
        match self {
            Fn1::F8(k0) => f8(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F10,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
        match self {
            Fn2::F10 => f10(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F13(u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn3::F13(k0, k1, k2) => f13(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F16(u64, u64, u64, Bytes, bool, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn4::F16(k0, k1, k2, k3, k4, k5, k6) => f16(*k0, *k1, *k2, k3.clone(), *k4, *k5, *k6, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F22,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt1, Adt1)) -> (Adt1, Adt1) {
        match self {
            Fn5::F22 => f22(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F24,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn6::F24 => f24(p0),
        }
    }
}

pub fn f0(v0: List<Adt0>) -> R<Adt0> {
    let v1: (u64, (List<Adt0>, Option<Adt0>)) = f1(Fn0::F2, (0u64, (List::<Adt0>::nil(), None::<Adt0>)), v0)?;
    let m3 = {
        let (_, h1) = {
            let (_, h2) = v1.clone();
            h2
        };
        h1
    };
    match m3 {
        None => {
            f3(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(f4({
                let (h4, _) = {
                    let (_, h5) = v1;
                    h5
                };
                h4
            }, v2, true))
        }
    }
}

pub fn f1(v0: Fn0, p9: (u64, (List<Adt0>, Option<Adt0>)), p10: List<Adt0>) -> R<(u64, (List<Adt0>, Option<Adt0>))> {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<((u64, (List<Adt0>, Option<Adt0>)), List<Adt0>), R<(u64, (List<Adt0>, Option<Adt0>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (List<Adt0>, Option<Adt0>)), List<Adt0>), R<(u64, (List<Adt0>, Option<Adt0>))>>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: (u64, (List<Adt0>, Option<Adt0>)), v1: Adt0) -> R<(u64, (List<Adt0>, Option<Adt0>))> {
    let v2: u64 = {
        let (h11, _) = v0.clone();
        h11
    };
    let v3: List<Adt0> = {
        let (h12, _) = {
            let (_, h13) = v0.clone();
            h13
        };
        h12
    };
    let m16 = {
        let (_, h14) = {
            let (_, h15) = v0;
            h15
        };
        h14
    };
    match m16 {
        None => {
            Ok((v2, (v3, Some(v1))))
        }
        Some(v4) => {
            Ok(({
                let a17 = v2;
                let a18 = 1u64;
                nat_add(a17, a18)?
            }, (f5(v3, v4, {
                let a19 = v2;
                let a20 = 1u64;
                nat_add(a19, a20)?
            }), Some(v1))))
        }
    }
}

pub fn f3(v0: Bytes) -> R<Adt0> {
    f6(v0, 0u64, true)
}

pub fn f4(v0: List<Adt0>, v1: Adt0, v2: bool) -> Adt0 {
    let (_, h21) = f7(Fn1::F8(v2), ({
        let a22 = v0.clone();
        length_list(a22)
    }, v1), v0);
    h21
}

pub fn f5(v0: List<Adt0>, v1: Adt0, v2: u64) -> List<Adt0> {
    let v3: (u64, (Adt0, List<Adt0>)) = f9(Fn2::F10, 64u64, (v2, (v1, v0)));
    List::cons({
        let (h23, _) = {
            let (_, h24) = v3.clone();
            h24
        };
        h23
    }, {
        let (_, h25) = {
            let (_, h26) = v3;
            h26
        };
        h25
    })
}

pub fn f6(v0: Bytes, v1: u64, v2: bool) -> R<Adt0> {
    let v3: u64 = {
        let a27 = v0.clone();
        length_bytes(a27)
    };
    let v4: u64 = {
        let a28 = v3;
        let a29 = 0u64;
        if nat_eq(a28, a29) {
            1u64
        } else {
            let a32 = {
                let a30 = v3;
                let a31 = 1023u64;
                nat_add(a30, a31)?
            };
            let a33 = 1024u64;
            let a34 = 0u64;
            nat_quot(a32, a33, a34)
        }
    };
    let v5: u64 = {
        let a35 = v4;
        let a36 = 1u64;
        nat_sub(a35, a36)
    };
    let a37 = v4;
    let a38 = 1u64;
    if nat_eq(a37, a38) {
        f11(v0, v3, 0u64, v1, v2)
    } else {
        let v6: List<Adt0> = {
            let (_, h39) = f12(Fn3::F13(v1, v0.clone(), v3), v5, (0u64, List::<Adt0>::nil()))?;
            h39
        };
        Ok(f4(v6, f11(v0, v3, v5, {
            let a40 = v1;
            let a41 = v5;
            nat_add(a40, a41)?
        }, false)?, v2))
    }
}

pub fn f7(v0: Fn1, p45: (u64, Adt0), p46: List<Adt0>) -> (u64, Adt0) {
    let m44 = (p45, p46);
    {
        let mut m44 = m44;
        loop {
            let step = {
                let (v1, v2) = m44;
                let m42 = v2;
                match m42.uncons() {
                    None => {
                        Err::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(({
                            let c43: Fn1 = v0.clone();
                            c43.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m44 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: bool, v1: (u64, Adt0), v2: Adt0) -> (u64, Adt0) {
    ({
        let a48 = {
            let (h47, _) = v1.clone();
            h47
        };
        let a49 = 1u64;
        nat_sub(a48, a49)
    }, f14(v2, {
        let (_, h50) = v1;
        h50
    }, if v0 {
        let a52 = {
            let (h51, _) = v1;
            h51
        };
        let a53 = 1u64;
        nat_eq(a52, a53)
    } else {
        false
    }))
}

pub fn f9(v0: Fn2, p57: u64, p58: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m54 = v1;
                if m54 == 0 {
                    Err::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>(v2)
                } else {
                    let v3 = m54 - 1;
                    Ok::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>((v3, {
                        let c55: Fn2 = v0.clone();
                        c55.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let v1: u64 = {
        let (h59, _) = v0.clone();
        h59
    };
    let v2: Adt0 = {
        let (h60, _) = {
            let (_, h61) = v0.clone();
            h61
        };
        h60
    };
    let v3: List<Adt0> = {
        let (_, h62) = {
            let (_, h63) = v0.clone();
            h63
        };
        h62
    };
    let a67 = {
        let a64 = v1;
        let a65 = 2u64;
        let a66 = 0u64;
        nat_rem(a64, a65, a66)
    };
    let a68 = 0u64;
    if nat_eq(a67, a68) {
        let m69 = v3;
        match m69.uncons() {
            None => {
                v0
            }
            Some((v4, v5)) => {
                ({
                    let a70 = v1;
                    let a71 = 2u64;
                    let a72 = 0u64;
                    nat_quot(a70, a71, a72)
                }, (f14(v4, v2, false), v5))
            }
        }
    } else {
        v0
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = {
        let a73 = v2;
        let a74 = 1024u64;
        nat_mul(a73, a74)?
    };
    let v6: u64 = {
        let a75 = v1;
        let a76 = v5;
        nat_sub(a75, a76)
    };
    let v7: u64 = {
        let a77 = v6;
        let a78 = 1024u64;
        if nat_lt(a77, a78) {
            v6
        } else {
            1024u64
        }
    };
    let v8: u64 = {
        let a79 = v7;
        let a80 = 0u64;
        if nat_eq(a79, a80) {
            1u64
        } else {
            let a83 = {
                let a81 = v7;
                let a82 = 63u64;
                nat_add(a81, a82)?
            };
            let a84 = 64u64;
            let a85 = 0u64;
            nat_quot(a83, a84, a85)
        }
    };
    let (_, h86) = f15(Fn4::F16(v8, v3, v7, v0, v4, v1, v5), v8, (0u64, f17()))?;
    Ok(h86)
}

pub fn f12(v0: Fn3, p90: u64, p91: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m89 = (p90, p91);
    {
        let mut m89 = m89;
        loop {
            let step = {
                let (v1, v2) = m89;
                let m87 = v1;
                if m87 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m87 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c88: Fn3 = v0.clone();
                        c88.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m89 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Bytes, v2: u64, v3: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let v4: u64 = {
        let (h92, _) = v3;
        h92
    };
    Ok(({
        let a93 = v4;
        let a94 = 1u64;
        nat_add(a93, a94)?
    }, f5({
        let (_, h95) = v3;
        h95
    }, f11(v1, v2, v4, {
        let a96 = v0;
        let a97 = v4;
        nat_add(a96, a97)?
    }, false)?, {
        let a98 = v4;
        let a99 = 1u64;
        nat_add(a98, a99)?
    })))
}

pub fn f14(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f18(f17(), Adt1::C0(match v0.clone() {
        Adt0::C0(h100, _, _, _, _, _, _, _) => {
            h100
        }
    }, match v0.clone() {
        Adt0::C0(_, h101, _, _, _, _, _, _) => {
            h101
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h102, _, _, _, _, _) => {
            h102
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h103, _, _, _, _) => {
            h103
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h104, _, _, _) => {
            h104
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h105, _, _) => {
            h105
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h106, _) => {
            h106
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h107) => {
            h107
        }
    }, match v1.clone() {
        Adt0::C0(h108, _, _, _, _, _, _, _) => {
            h108
        }
    }, match v1.clone() {
        Adt0::C0(_, h109, _, _, _, _, _, _) => {
            h109
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h110, _, _, _, _, _) => {
            h110
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h111, _, _, _, _) => {
            h111
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h112, _, _, _) => {
            h112
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h113, _, _) => {
            h113
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h114, _) => {
            h114
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h115) => {
            h115
        }
    }), 0u64, 64u32, if v2 {
        12u32
    } else {
        4u32
    })
}

pub fn f15(v0: Fn4, p119: u64, p120: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m116 = v1;
                if m116 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m116 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c117: Fn4 = v0.clone();
                        c117.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: Bytes, v4: bool, v5: u64, v6: u64, v7: (u64, Adt0)) -> R<(u64, Adt0)> {
    let v8: u64 = {
        let (h121, _) = v7;
        h121
    };
    let v9: bool = {
        let a124 = {
            let a122 = v8;
            let a123 = 1u64;
            nat_add(a122, a123)?
        };
        let a125 = v0;
        nat_eq(a124, a125)
    };
    let v10: u32 = {
        let a128 = {
            let a126 = v8;
            let a127 = 0u64;
            if nat_eq(a126, a127) {
                1u32
            } else {
                0u32
            }
        };
        let a129 = if v9 {
            if v4 {
                10u32
            } else {
                2u32
            }
        } else {
            0u32
        };
        fixed_u32::bit_or(a128, a129)
    };
    let v11: u32 = if v9 {
        f19({
            let a132 = v2;
            let a133 = {
                let a130 = v8;
                let a131 = 64u64;
                nat_mul(a130, a131)?
            };
            nat_sub(a132, a133)
        })
    } else {
        64u32
    };
    Ok(({
        let a134 = v8;
        let a135 = 1u64;
        nat_add(a134, a135)?
    }, f18({
        let (_, h136) = v7;
        h136
    }, f20(v3, v5, {
        let a139 = v6;
        let a140 = {
            let a137 = v8;
            let a138 = 64u64;
            nat_mul(a137, a138)?
        };
        nat_add(a139, a140)?
    })?, v1, v11, v10)))
}

pub fn f17() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f18(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt1 = Adt1::C0(match v0.clone() {
        Adt0::C0(h141, _, _, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, h142, _, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h143, _, _, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h144, _, _, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h145, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h146, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h147, _) => {
            h147
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h148) => {
            h148
        }
    }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f19({
        let a149 = v2;
        let a150 = 4294967296u64;
        let a151 = 0u64;
        nat_rem(a149, a150, a151)
    }), f19({
        let a152 = v2;
        let a153 = 4294967296u64;
        let a154 = 0u64;
        nat_quot(a152, a153, a154)
    }), v3, v4);
    let v6: Adt1 = {
        let (h155, _) = f21(Fn5::F22, 7u64, (v5, v1));
        h155
    };
    Adt0::C0({
        let a158 = match v6.clone() {
            Adt1::C0(h156, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h156
            }
        };
        let a159 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h157, _, _, _, _, _, _, _) => {
                h157
            }
        };
        fixed_u32::bit_xor(a158, a159)
    }, {
        let a162 = match v6.clone() {
            Adt1::C0(_, h160, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h161, _, _, _, _, _, _) => {
                h161
            }
        };
        fixed_u32::bit_xor(a162, a163)
    }, {
        let a166 = match v6.clone() {
            Adt1::C0(_, _, h164, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h164
            }
        };
        let a167 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h165, _, _, _, _, _) => {
                h165
            }
        };
        fixed_u32::bit_xor(a166, a167)
    }, {
        let a170 = match v6.clone() {
            Adt1::C0(_, _, _, h168, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h168
            }
        };
        let a171 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h169, _, _, _, _) => {
                h169
            }
        };
        fixed_u32::bit_xor(a170, a171)
    }, {
        let a174 = match v6.clone() {
            Adt1::C0(_, _, _, _, h172, _, _, _, _, _, _, _, _, _, _, _) => {
                h172
            }
        };
        let a175 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h173, _, _, _) => {
                h173
            }
        };
        fixed_u32::bit_xor(a174, a175)
    }, {
        let a178 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, h176, _, _, _, _, _, _, _, _, _, _) => {
                h176
            }
        };
        let a179 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h177, _, _) => {
                h177
            }
        };
        fixed_u32::bit_xor(a178, a179)
    }, {
        let a182 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, h180, _, _, _, _, _, _, _, _, _) => {
                h180
            }
        };
        let a183 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h181, _) => {
                h181
            }
        };
        fixed_u32::bit_xor(a182, a183)
    }, {
        let a186 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h184, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a187 = match v6 {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h185) => {
                h185
            }
        };
        fixed_u32::bit_xor(a186, a187)
    })
}

pub fn f19(v0: u64) -> u32 {
    let (_, h188) = {
        let (_, h189) = f23(Fn6::F24, 32u64, (v0, (1u32, 0u32)));
        h189
    };
    h188
}

pub fn f20(v0: Bytes, v1: u64, v2: u64) -> R<Adt1> {
    Ok(Adt1::C0(f25(v0.clone(), v1, v2)?, f25(v0.clone(), v1, {
        let a190 = v2;
        let a191 = 4u64;
        nat_add(a190, a191)?
    })?, f25(v0.clone(), v1, {
        let a192 = v2;
        let a193 = 8u64;
        nat_add(a192, a193)?
    })?, f25(v0.clone(), v1, {
        let a194 = v2;
        let a195 = 12u64;
        nat_add(a194, a195)?
    })?, f25(v0.clone(), v1, {
        let a196 = v2;
        let a197 = 16u64;
        nat_add(a196, a197)?
    })?, f25(v0.clone(), v1, {
        let a198 = v2;
        let a199 = 20u64;
        nat_add(a198, a199)?
    })?, f25(v0.clone(), v1, {
        let a200 = v2;
        let a201 = 24u64;
        nat_add(a200, a201)?
    })?, f25(v0.clone(), v1, {
        let a202 = v2;
        let a203 = 28u64;
        nat_add(a202, a203)?
    })?, f25(v0.clone(), v1, {
        let a204 = v2;
        let a205 = 32u64;
        nat_add(a204, a205)?
    })?, f25(v0.clone(), v1, {
        let a206 = v2;
        let a207 = 36u64;
        nat_add(a206, a207)?
    })?, f25(v0.clone(), v1, {
        let a208 = v2;
        let a209 = 40u64;
        nat_add(a208, a209)?
    })?, f25(v0.clone(), v1, {
        let a210 = v2;
        let a211 = 44u64;
        nat_add(a210, a211)?
    })?, f25(v0.clone(), v1, {
        let a212 = v2;
        let a213 = 48u64;
        nat_add(a212, a213)?
    })?, f25(v0.clone(), v1, {
        let a214 = v2;
        let a215 = 52u64;
        nat_add(a214, a215)?
    })?, f25(v0.clone(), v1, {
        let a216 = v2;
        let a217 = 56u64;
        nat_add(a216, a217)?
    })?, f25(v0, v1, {
        let a218 = v2;
        let a219 = 60u64;
        nat_add(a218, a219)?
    })?))
}

pub fn f21(v0: Fn5, p223: u64, p224: (Adt1, Adt1)) -> (Adt1, Adt1) {
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

pub fn f22(v0: (Adt1, Adt1)) -> (Adt1, Adt1) {
    (f26({
        let (h225, _) = v0.clone();
        h225
    }, {
        let (_, h226) = v0.clone();
        h226
    }), f27({
        let (_, h227) = v0;
        h227
    }))
}

pub fn f23(v0: Fn6, p231: u64, p232: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m230 = (p231, p232);
    {
        let mut m230 = m230;
        loop {
            let step = {
                let (v1, v2) = m230;
                let m228 = v1;
                if m228 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m228 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c229: Fn6 = v0.clone();
                        c229.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h233, _) = v0;
        h233
    };
    let v2: u32 = {
        let (h234, _) = {
            let (_, h235) = v0;
            h235
        };
        h234
    };
    let v3: u32 = {
        let (_, h236) = {
            let (_, h237) = v0;
            h237
        };
        h236
    };
    ({
        let a238 = v1;
        let a239 = 2u64;
        let a240 = 0u64;
        nat_quot(a238, a239, a240)
    }, (f28(v2, 1u32), {
        let a244 = {
            let a241 = v1;
            let a242 = 2u64;
            let a243 = 0u64;
            nat_rem(a241, a242, a243)
        };
        let a245 = 1u64;
        if nat_eq(a244, a245) {
            let a246 = v3;
            let a247 = v2;
            fixed_u32::bit_or(a246, a247)
        } else {
            v3
        }
    }))
}

pub fn f25(v0: Bytes, v1: u64, v2: u64) -> R<u32> {
    let a258 = f29(v0.clone(), v1, v2);
    let a259 = {
        let a256 = f28(f29(v0.clone(), v1, {
            let a248 = v2;
            let a249 = 1u64;
            nat_add(a248, a249)?
        }), 8u32);
        let a257 = {
            let a254 = f28(f29(v0.clone(), v1, {
                let a250 = v2;
                let a251 = 2u64;
                nat_add(a250, a251)?
            }), 16u32);
            let a255 = f28(f29(v0, v1, {
                let a252 = v2;
                let a253 = 3u64;
                nat_add(a252, a253)?
            }), 24u32);
            fixed_u32::bit_or(a254, a255)
        };
        fixed_u32::bit_or(a256, a257)
    };
    Ok(fixed_u32::bit_or(a258, a259))
}

pub fn f26(v0: Adt1, v1: Adt1) -> Adt1 {
    let v2: Adt2 = f30(match v0.clone() {
        Adt1::C0(h260, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h260
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h261, _, _, _, _, _, _, _, _, _, _, _) => {
            h261
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h262, _, _, _, _, _, _, _) => {
            h262
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h263, _, _, _) => {
            h263
        }
    }, match v1.clone() {
        Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h264
        }
    }, match v1.clone() {
        Adt1::C0(_, h265, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h265
        }
    });
    let v3: Adt2 = f30(match v0.clone() {
        Adt1::C0(_, h266, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h266
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h267, _, _, _, _, _, _, _, _, _, _) => {
            h267
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h268, _, _, _, _, _, _) => {
            h268
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h269, _, _) => {
            h269
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h270, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h270
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h271
        }
    });
    let v4: Adt2 = f30(match v0.clone() {
        Adt1::C0(_, _, h272, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h272
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h273, _, _, _, _, _, _, _, _, _) => {
            h273
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h274, _, _, _, _, _) => {
            h274
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h275, _) => {
            h275
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h276, _, _, _, _, _, _, _, _, _, _, _) => {
            h276
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h277, _, _, _, _, _, _, _, _, _, _) => {
            h277
        }
    });
    let v5: Adt2 = f30(match v0.clone() {
        Adt1::C0(_, _, _, h278, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h278
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h279, _, _, _, _, _, _, _, _) => {
            h279
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h280, _, _, _, _) => {
            h280
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h281) => {
            h281
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h282, _, _, _, _, _, _, _, _, _) => {
            h282
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h283, _, _, _, _, _, _, _, _) => {
            h283
        }
    });
    let v6: Adt2 = f30(match v2.clone() {
        Adt2::C0(h284, _, _, _) => {
            h284
        }
    }, match v3.clone() {
        Adt2::C0(_, h285, _, _) => {
            h285
        }
    }, match v4.clone() {
        Adt2::C0(_, _, h286, _) => {
            h286
        }
    }, match v5.clone() {
        Adt2::C0(_, _, _, h287) => {
            h287
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h288, _, _, _, _, _, _, _) => {
            h288
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h289, _, _, _, _, _, _) => {
            h289
        }
    });
    let v7: Adt2 = f30(match v3.clone() {
        Adt2::C0(h290, _, _, _) => {
            h290
        }
    }, match v4.clone() {
        Adt2::C0(_, h291, _, _) => {
            h291
        }
    }, match v5.clone() {
        Adt2::C0(_, _, h292, _) => {
            h292
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h293) => {
            h293
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h294, _, _, _, _, _) => {
            h294
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h295, _, _, _, _) => {
            h295
        }
    });
    let v8: Adt2 = f30(match v4.clone() {
        Adt2::C0(h296, _, _, _) => {
            h296
        }
    }, match v5.clone() {
        Adt2::C0(_, h297, _, _) => {
            h297
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h298, _) => {
            h298
        }
    }, match v3.clone() {
        Adt2::C0(_, _, _, h299) => {
            h299
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h300, _, _, _) => {
            h300
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h301, _, _) => {
            h301
        }
    });
    let v9: Adt2 = f30(match v5 {
        Adt2::C0(h302, _, _, _) => {
            h302
        }
    }, match v2 {
        Adt2::C0(_, h303, _, _) => {
            h303
        }
    }, match v3 {
        Adt2::C0(_, _, h304, _) => {
            h304
        }
    }, match v4 {
        Adt2::C0(_, _, _, h305) => {
            h305
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h306, _) => {
            h306
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h307) => {
            h307
        }
    });
    Adt1::C0(match v6.clone() {
        Adt2::C0(h308, _, _, _) => {
            h308
        }
    }, match v7.clone() {
        Adt2::C0(h309, _, _, _) => {
            h309
        }
    }, match v8.clone() {
        Adt2::C0(h310, _, _, _) => {
            h310
        }
    }, match v9.clone() {
        Adt2::C0(h311, _, _, _) => {
            h311
        }
    }, match v9.clone() {
        Adt2::C0(_, h312, _, _) => {
            h312
        }
    }, match v6.clone() {
        Adt2::C0(_, h313, _, _) => {
            h313
        }
    }, match v7.clone() {
        Adt2::C0(_, h314, _, _) => {
            h314
        }
    }, match v8.clone() {
        Adt2::C0(_, h315, _, _) => {
            h315
        }
    }, match v8.clone() {
        Adt2::C0(_, _, h316, _) => {
            h316
        }
    }, match v9.clone() {
        Adt2::C0(_, _, h317, _) => {
            h317
        }
    }, match v6.clone() {
        Adt2::C0(_, _, h318, _) => {
            h318
        }
    }, match v7.clone() {
        Adt2::C0(_, _, h319, _) => {
            h319
        }
    }, match v7 {
        Adt2::C0(_, _, _, h320) => {
            h320
        }
    }, match v8 {
        Adt2::C0(_, _, _, h321) => {
            h321
        }
    }, match v9 {
        Adt2::C0(_, _, _, h322) => {
            h322
        }
    }, match v6 {
        Adt2::C0(_, _, _, h323) => {
            h323
        }
    })
}

pub fn f27(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h324, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h325, _, _, _, _, _, _, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h326, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h326
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h327, _, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h328, _, _, _, _, _, _, _, _) => {
            h328
        }
    }, match v0.clone() {
        Adt1::C0(h329, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h329
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h330, _, _, _, _, _, _, _, _, _, _, _) => {
            h330
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h331, _, _) => {
            h331
        }
    }, match v0.clone() {
        Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h332
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h333, _, _, _, _) => {
            h333
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h334, _, _, _) => {
            h334
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h335, _, _, _, _, _, _, _, _, _, _) => {
            h335
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h336, _, _, _, _, _, _) => {
            h336
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h337, _) => {
            h337
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h338) => {
            h338
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h339, _, _, _, _, _, _, _) => {
            h339
        }
    })
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m342 = {
        let a340 = v0;
        let a341 = v1;
        fixed_u32::shift_left(a340, a341)
    };
    match m342 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f29(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a343 = v2;
    let a344 = v1;
    if nat_lt(a343, a344) {
        let m347 = {
            let a345 = v0;
            let a346 = v2;
            index_bytes(a345, a346)
        };
        match m347 {
            None => {
                0u32
            }
            Some(v3) => {
                let m349 = {
                    let a348 = v3;
                    fixed_u32::convert(i128::from(a348))
                };
                match m349 {
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

pub fn f30(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt2 {
    let v6: u32 = f31(f31(v0, v1), v4);
    let v7: u32 = f32({
        let a350 = v3;
        let a351 = v6;
        fixed_u32::bit_xor(a350, a351)
    }, 16u32, 16u32);
    let v8: u32 = f31(v2, v7);
    let v9: u32 = f32({
        let a352 = v1;
        let a353 = v8;
        fixed_u32::bit_xor(a352, a353)
    }, 12u32, 20u32);
    let v10: u32 = f31(f31(v6, v9), v5);
    let v11: u32 = f32({
        let a354 = v7;
        let a355 = v10;
        fixed_u32::bit_xor(a354, a355)
    }, 8u32, 24u32);
    let v12: u32 = f31(v8, v11);
    let v13: u32 = f32({
        let a356 = v9;
        let a357 = v12;
        fixed_u32::bit_xor(a356, a357)
    }, 7u32, 25u32);
    Adt2::C0(v10, v13, v12, v11)
}

pub fn f31(v0: u32, v1: u32) -> u32 {
    let m360 = {
        let a358 = f33(v0);
        let a359 = f33(v1);
        fixed_u64::checked_add(a358, a359)
    };
    match m360 {
        None => {
            0u32
        }
        Some(v2) => {
            f34(v2)
        }
    }
}

pub fn f32(v0: u32, v1: u32, v2: u32) -> u32 {
    let a361 = f35(v0, v1);
    let a362 = f28(v0, v2);
    fixed_u32::bit_or(a361, a362)
}

pub fn f33(v0: u32) -> u64 {
    let m364 = {
        let a363 = v0;
        fixed_u64::convert(i128::from(a363))
    };
    match m364 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f34(v0: u64) -> u32 {
    let m368 = {
        let a367 = {
            let a365 = v0;
            let a366 = 4294967295u64;
            fixed_u64::bit_and(a365, a366)
        };
        fixed_u32::convert(i128::from(a367))
    };
    match m368 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f35(v0: u32, v1: u32) -> u32 {
    let m371 = {
        let a369 = v0;
        let a370 = v1;
        fixed_u32::shift_right(a369, a370)
    };
    match m371 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}
