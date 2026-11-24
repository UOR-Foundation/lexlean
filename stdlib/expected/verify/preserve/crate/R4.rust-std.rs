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
    C0(Adt0, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F2,
}

impl Fn0 {
    pub fn apply(&self, p0: ((u64, List<Adt0>), Option<Adt0>), p1: Adt0) -> R<((u64, List<Adt0>), Option<Adt0>)> {
        match self {
            Fn0::F2 => f2(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F5,
    F9,
}

impl Fn1 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn1::F5 => f5(p0),
            Fn1::F9 => f9(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F12,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> Adt1 {
        match self {
            Fn2::F12 => f12(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F14(Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn3::F14(k0, k1) => f14(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F24,
}

impl Fn4 {
    pub fn apply(&self, p0: (Adt3, Adt2)) -> (Adt3, Adt2) {
        match self {
            Fn4::F24 => f24(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F26(u64, Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn5::F26(k0, k1, k2) => f26(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F30,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn6::F30 => f30(p0),
        }
    }
}

pub fn f0(v0: List<Adt0>) -> R<Adt0> {
    let v1: ((u64, List<Adt0>), Option<Adt0>) = f1(Fn0::F2, ((0u64, List::<Adt0>::nil()), None::<Adt0>), v0)?;
    let m2 = {
        let (_, h1) = v1;
        h1
    };
    match m2 {
        None => {
            f3(Bytes::lit(&[]))
        }
        Some(v2) => {
            let m5 = {
                let (_, h3) = {
                    let (h4, _) = v1;
                    h4
                };
                h3
            };
            match m5.uncons() {
                None => {
                    Ok(v2)
                }
                Some((v3, v4)) => {
                    let (_, h6) = f4(Fn1::F5, 64u64, (List::cons(v3, v4), v2));
                    Ok(h6)
                }
            }
        }
    }
}

pub fn f1(v0: Fn0, p10: ((u64, List<Adt0>), Option<Adt0>), p11: List<Adt0>) -> R<((u64, List<Adt0>), Option<Adt0>)> {
    let m9 = (p10, p11);
    {
        let mut m9 = m9;
        loop {
            let step = {
                let (v1, v2) = m9;
                let m7 = v2;
                match m7.uncons() {
                    None => {
                        Err::<(((u64, List<Adt0>), Option<Adt0>), List<Adt0>), R<((u64, List<Adt0>), Option<Adt0>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(((u64, List<Adt0>), Option<Adt0>), List<Adt0>), R<((u64, List<Adt0>), Option<Adt0>)>>(({
                            let c8: Fn0 = v0.clone();
                            c8.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m9 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: ((u64, List<Adt0>), Option<Adt0>), v1: Adt0) -> R<((u64, List<Adt0>), Option<Adt0>)> {
    let m13 = {
        let (_, h12) = v0;
        h12
    };
    match m13 {
        None => {
            Ok(({
                let (h14, _) = v0;
                h14
            }, Some(v1)))
        }
        Some(v2) => {
            Ok((f6({
                let (h15, _) = v0;
                h15
            }, v2)?, Some(v1)))
        }
    }
}

pub fn f3(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a16 = v0.clone();
        length_bytes(a16)
    };
    let v2: u64 = {
        let a17 = v1;
        let a18 = 0u64;
        if nat_eq(a17, a18) {
            1u64
        } else {
            let a21 = {
                let a19 = v1;
                let a20 = 1023u64;
                nat_add(a19, a20)?
            };
            let a22 = 1024u64;
            let a23 = 0u64;
            nat_quot(a21, a22, a23)
        }
    };
    let v3: u64 = {
        let a24 = v2;
        let a25 = 1u64;
        nat_sub(a24, a25)
    };
    let m27 = {
        let (_, h26) = f7(v0.clone(), v1, v3)?;
        h26
    };
    match m27.uncons() {
        None => {
            f8(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h28) = f4(Fn1::F9, 64u64, (List::cons(v4, v5), f8(v0, v1, v3, false)?));
            Ok(h28)
        }
    }
}

pub fn f4(v0: Fn1, p32: u64, p33: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m31 = (p32, p33);
    {
        let mut m31 = m31;
        loop {
            let step = {
                let (v1, v2) = m31;
                let m29 = v1;
                if m29 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m29 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c30: Fn1 = v0.clone();
                        c30.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f10(v0)
}

pub fn f6(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt1 = f11(Fn2::F12, 40u64, Adt1::C0(v1, {
        let a35 = {
            let (h34, _) = v0.clone();
            h34
        };
        let a36 = 1u64;
        nat_add(a35, a36)?
    }, {
        let (_, h37) = v0;
        h37
    }));
    Ok(({
        let a39 = {
            let (h38, _) = v0;
            h38
        };
        let a40 = 1u64;
        nat_add(a39, a40)?
    }, List::cons(match v2.clone() {
        Adt1::C0(h41, _, _) => {
            h41
        }
    }, match v2 {
        Adt1::C0(_, _, h42) => {
            h42
        }
    })))
}

pub fn f7(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f13(Fn3::F14(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f8(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a43 = 1024u64;
        let a44 = v2;
        nat_mul(a43, a44)?
    };
    let v5: u64 = f15(v1, v4);
    let v6: u64 = f16(v5)?;
    let v7: u64 = {
        let a45 = v6;
        let a46 = 1u64;
        nat_sub(a45, a46)
    };
    Ok(f17(f18(v0.clone(), v4, v2, v7)?, f19(v0, {
        let a49 = v4;
        let a50 = {
            let a47 = 64u64;
            let a48 = v7;
            nat_mul(a47, a48)?
        };
        nat_add(a49, a50)?
    })?, v2, f20({
        let a53 = v5;
        let a54 = {
            let a51 = 64u64;
            let a52 = v7;
            nat_mul(a51, a52)?
        };
        nat_sub(a53, a54)
    }), {
        let a57 = {
            let a55 = v7;
            let a56 = 0u64;
            if nat_eq(a55, a56) {
                3u32
            } else {
                2u32
            }
        };
        let a58 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a57, a58)
    }))
}

pub fn f9(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f10(v0)
}

pub fn f10(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m60 = {
        let (h59, _) = v0.clone();
        h59
    };
    match m60.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f21(v1, {
                let (_, h61) = v0;
                h61
            }, {
                let m62 = v2;
                match m62.uncons() {
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

pub fn f11(v0: Fn2, p66: u64, p67: Adt1) -> Adt1 {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v1;
                if m63 == 0 {
                    Err::<(u64, Adt1), Adt1>(v2)
                } else {
                    let v3 = m63 - 1;
                    Ok::<(u64, Adt1), Adt1>((v3, {
                        let c64: Fn2 = v0.clone();
                        c64.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt1) -> Adt1 {
    f22(v0)
}

pub fn f13(v0: Fn3, p71: u64, p72: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v1;
                if m68 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m68 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c69: Fn3 = v0.clone();
                        c69.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m70 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f6(v2.clone(), f8(v0, v1, {
        let (h73, _) = v2;
        h73
    }, false)?)
}

pub fn f15(v0: u64, v1: u64) -> u64 {
    let a76 = 1024u64;
    let a77 = {
        let a74 = v0;
        let a75 = v1;
        nat_sub(a74, a75)
    };
    if nat_lt(a76, a77) {
        1024u64
    } else {
        let a78 = v0;
        let a79 = v1;
        nat_sub(a78, a79)
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a80 = v0;
    let a81 = 0u64;
    if nat_eq(a80, a81) {
        Ok(1u64)
    } else {
        let a84 = {
            let a82 = v0;
            let a83 = 63u64;
            nat_add(a82, a83)?
        };
        let a85 = 64u64;
        let a86 = 0u64;
        Ok(nat_quot(a84, a85, a86))
    }
}

pub fn f17(v0: Adt0, v1: Adt2, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt3 = {
        let (h87, _) = f23(Fn4::F24, 7u64, (Adt3::C0(match v0.clone() {
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
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f20({
            let a96 = v2;
            let a97 = 4294967296u64;
            let a98 = 0u64;
            nat_rem(a96, a97, a98)
        }), f20({
            let a99 = v2;
            let a100 = 4294967296u64;
            let a101 = 0u64;
            nat_quot(a99, a100, a101)
        }), v3, v4), v1));
        h87
    };
    Adt0::C0({
        let a104 = match v5.clone() {
            Adt3::C0(h102, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h102
            }
        };
        let a105 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h103, _, _, _, _, _, _, _) => {
                h103
            }
        };
        fixed_u32::bit_xor(a104, a105)
    }, {
        let a108 = match v5.clone() {
            Adt3::C0(_, h106, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h106
            }
        };
        let a109 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h107, _, _, _, _, _, _) => {
                h107
            }
        };
        fixed_u32::bit_xor(a108, a109)
    }, {
        let a112 = match v5.clone() {
            Adt3::C0(_, _, h110, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        let a113 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h111, _, _, _, _, _) => {
                h111
            }
        };
        fixed_u32::bit_xor(a112, a113)
    }, {
        let a116 = match v5.clone() {
            Adt3::C0(_, _, _, h114, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h114
            }
        };
        let a117 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h115, _, _, _, _) => {
                h115
            }
        };
        fixed_u32::bit_xor(a116, a117)
    }, {
        let a120 = match v5.clone() {
            Adt3::C0(_, _, _, _, h118, _, _, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        };
        let a121 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h119, _, _, _) => {
                h119
            }
        };
        fixed_u32::bit_xor(a120, a121)
    }, {
        let a124 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, h122, _, _, _, _, _, _, _, _, _, _) => {
                h122
            }
        };
        let a125 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h123, _, _) => {
                h123
            }
        };
        fixed_u32::bit_xor(a124, a125)
    }, {
        let a128 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, h126, _, _, _, _, _, _, _, _, _) => {
                h126
            }
        };
        let a129 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h127, _) => {
                h127
            }
        };
        fixed_u32::bit_xor(a128, a129)
    }, {
        let a132 = match v5.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h130, _, _, _, _, _, _, _, _) => {
                h130
            }
        };
        let a133 = match v5 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h131) => {
                h131
            }
        };
        fixed_u32::bit_xor(a132, a133)
    })
}

pub fn f18(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h134) = f25(Fn5::F26(v2, v0, v1), v3, (0u64, f27()))?;
    Ok(h134)
}

pub fn f19(v0: Bytes, v1: u64) -> R<Adt2> {
    Ok(Adt2::C0(f28(v0.clone(), {
        let a135 = v1;
        let a136 = 0u64;
        nat_add(a135, a136)?
    })?, f28(v0.clone(), {
        let a137 = v1;
        let a138 = 4u64;
        nat_add(a137, a138)?
    })?, f28(v0.clone(), {
        let a139 = v1;
        let a140 = 8u64;
        nat_add(a139, a140)?
    })?, f28(v0.clone(), {
        let a141 = v1;
        let a142 = 12u64;
        nat_add(a141, a142)?
    })?, f28(v0.clone(), {
        let a143 = v1;
        let a144 = 16u64;
        nat_add(a143, a144)?
    })?, f28(v0.clone(), {
        let a145 = v1;
        let a146 = 20u64;
        nat_add(a145, a146)?
    })?, f28(v0.clone(), {
        let a147 = v1;
        let a148 = 24u64;
        nat_add(a147, a148)?
    })?, f28(v0.clone(), {
        let a149 = v1;
        let a150 = 28u64;
        nat_add(a149, a150)?
    })?, f28(v0.clone(), {
        let a151 = v1;
        let a152 = 32u64;
        nat_add(a151, a152)?
    })?, f28(v0.clone(), {
        let a153 = v1;
        let a154 = 36u64;
        nat_add(a153, a154)?
    })?, f28(v0.clone(), {
        let a155 = v1;
        let a156 = 40u64;
        nat_add(a155, a156)?
    })?, f28(v0.clone(), {
        let a157 = v1;
        let a158 = 44u64;
        nat_add(a157, a158)?
    })?, f28(v0.clone(), {
        let a159 = v1;
        let a160 = 48u64;
        nat_add(a159, a160)?
    })?, f28(v0.clone(), {
        let a161 = v1;
        let a162 = 52u64;
        nat_add(a161, a162)?
    })?, f28(v0.clone(), {
        let a163 = v1;
        let a164 = 56u64;
        nat_add(a163, a164)?
    })?, f28(v0, {
        let a165 = v1;
        let a166 = 60u64;
        nat_add(a165, a166)?
    })?))
}

pub fn f20(v0: u64) -> u32 {
    let (_, h167) = {
        let (_, h168) = f29(Fn6::F30, 32u64, (v0, (1u32, 0u32)));
        h168
    };
    h167
}

pub fn f21(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f17(f27(), f31(v0, v1), 0u64, 64u32, {
        let a169 = 4u32;
        let a170 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a169, a170)
    })
}

pub fn f22(v0: Adt1) -> Adt1 {
    let a175 = {
        let a172 = match v0.clone() {
            Adt1::C0(_, h171, _) => {
                h171
            }
        };
        let a173 = 2u64;
        let a174 = 0u64;
        nat_rem(a172, a173, a174)
    };
    let a176 = 0u64;
    if nat_eq(a175, a176) {
        let m178 = match v0.clone() {
            Adt1::C0(_, _, h177) => {
                h177
            }
        };
        match m178.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt1::C0(f21(v1, match v0.clone() {
                    Adt1::C0(h179, _, _) => {
                        h179
                    }
                }, false), {
                    let a181 = match v0 {
                        Adt1::C0(_, h180, _) => {
                            h180
                        }
                    };
                    let a182 = 2u64;
                    let a183 = 0u64;
                    nat_quot(a181, a182, a183)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f23(v0: Fn4, p187: u64, p188: (Adt3, Adt2)) -> (Adt3, Adt2) {
    let m186 = (p187, p188);
    {
        let mut m186 = m186;
        loop {
            let step = {
                let (v1, v2) = m186;
                let m184 = v1;
                if m184 == 0 {
                    Err::<(u64, (Adt3, Adt2)), (Adt3, Adt2)>(v2)
                } else {
                    let v3 = m184 - 1;
                    Ok::<(u64, (Adt3, Adt2)), (Adt3, Adt2)>((v3, {
                        let c185: Fn4 = v0.clone();
                        c185.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: (Adt3, Adt2)) -> (Adt3, Adt2) {
    f32(v0)
}

pub fn f25(v0: Fn5, p192: u64, p193: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m191 = (p192, p193);
    {
        let mut m191 = m191;
        loop {
            let step = {
                let (v1, v2) = m191;
                let m189 = v1;
                if m189 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m189 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c190: Fn5 = v0.clone();
                        c190.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a195 = {
            let (h194, _) = v3.clone();
            h194
        };
        let a196 = 1u64;
        nat_add(a195, a196)?
    }, f17({
        let (_, h197) = v3.clone();
        h197
    }, f19(v1, {
        let a201 = v2;
        let a202 = {
            let a199 = 64u64;
            let a200 = {
                let (h198, _) = v3.clone();
                h198
            };
            nat_mul(a199, a200)?
        };
        nat_add(a201, a202)?
    })?, v0, 64u32, {
        let a204 = {
            let (h203, _) = v3;
            h203
        };
        let a205 = 0u64;
        if nat_eq(a204, a205) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f27() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f28(v0: Bytes, v1: u64) -> R<u32> {
    let a216 = f33(v0.clone(), v1);
    let a217 = {
        let a214 = f34(f33(v0.clone(), {
            let a206 = v1;
            let a207 = 1u64;
            nat_add(a206, a207)?
        }), 8u32);
        let a215 = {
            let a212 = f34(f33(v0.clone(), {
                let a208 = v1;
                let a209 = 2u64;
                nat_add(a208, a209)?
            }), 16u32);
            let a213 = f34(f33(v0, {
                let a210 = v1;
                let a211 = 3u64;
                nat_add(a210, a211)?
            }), 24u32);
            fixed_u32::bit_or(a212, a213)
        };
        fixed_u32::bit_or(a214, a215)
    };
    Ok(fixed_u32::bit_or(a216, a217))
}

pub fn f29(v0: Fn6, p221: u64, p222: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v1, v2) = m220;
                let m218 = v1;
                if m218 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m218 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c219: Fn6 = v0.clone();
                        c219.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h223, _) = v0;
        h223
    };
    let v2: u32 = {
        let (h224, _) = {
            let (_, h225) = v0;
            h225
        };
        h224
    };
    let v3: u32 = {
        let (_, h226) = {
            let (_, h227) = v0;
            h227
        };
        h226
    };
    ({
        let a228 = v1;
        let a229 = 2u64;
        let a230 = 0u64;
        nat_quot(a228, a229, a230)
    }, (f34(v2, 1u32), {
        let a234 = {
            let a231 = v1;
            let a232 = 2u64;
            let a233 = 0u64;
            nat_rem(a231, a232, a233)
        };
        let a235 = 1u64;
        if nat_eq(a234, a235) {
            let a236 = v3;
            let a237 = v2;
            fixed_u32::bit_or(a236, a237)
        } else {
            v3
        }
    }))
}

pub fn f31(v0: Adt0, v1: Adt0) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt0::C0(h238, _, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt0::C0(_, h239, _, _, _, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h240, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h241, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h242, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h243, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h244, _) => {
            h244
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h245) => {
            h245
        }
    }, match v1.clone() {
        Adt0::C0(h246, _, _, _, _, _, _, _) => {
            h246
        }
    }, match v1.clone() {
        Adt0::C0(_, h247, _, _, _, _, _, _) => {
            h247
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h248, _, _, _, _, _) => {
            h248
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h249, _, _, _, _) => {
            h249
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h250, _, _, _) => {
            h250
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h251, _, _) => {
            h251
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h252, _) => {
            h252
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h253) => {
            h253
        }
    })
}

pub fn f32(v0: (Adt3, Adt2)) -> (Adt3, Adt2) {
    (f35(f36({
        let (h254, _) = v0.clone();
        h254
    }, {
        let (_, h255) = v0.clone();
        h255
    }), {
        let (_, h256) = v0.clone();
        h256
    }), f37({
        let (_, h257) = v0;
        h257
    }))
}

pub fn f33(v0: Bytes, v1: u64) -> u32 {
    let m260 = {
        let a258 = v0;
        let a259 = v1;
        index_bytes(a258, a259)
    };
    match m260 {
        None => {
            0u32
        }
        Some(v2) => {
            let m262 = {
                let a261 = v2;
                fixed_u32::convert(i128::from(a261))
            };
            match m262 {
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

pub fn f34(v0: u32, v1: u32) -> u32 {
    let m265 = {
        let a263 = v0;
        let a264 = v1;
        fixed_u32::shift_left(a263, a264)
    };
    match m265 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f35(v0: Adt3, v1: Adt2) -> Adt3 {
    let v2: Adt4 = f38(match v0.clone() {
        Adt3::C0(h266, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h266
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h267, _, _, _, _, _, _, _, _, _, _) => {
            h267
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h268, _, _, _, _, _) => {
            h268
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h269) => {
            h269
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h270, _, _, _, _, _, _, _) => {
            h270
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h271, _, _, _, _, _, _) => {
            h271
        }
    });
    let v3: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, h272, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h272
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h273, _, _, _, _, _, _, _, _, _) => {
            h273
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h274, _, _, _, _) => {
            h274
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h275, _, _, _) => {
            h275
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h276, _, _, _, _, _) => {
            h276
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h277, _, _, _, _) => {
            h277
        }
    });
    let v4: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, _, h278, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h278
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h279, _, _, _, _, _, _, _, _) => {
            h279
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h280, _, _, _, _, _, _, _) => {
            h280
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h281, _, _) => {
            h281
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h282, _, _, _) => {
            h282
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h283, _, _) => {
            h283
        }
    });
    let v5: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, _, _, h284, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h284
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h285, _, _, _, _, _, _, _, _, _, _, _) => {
            h285
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h286, _, _, _, _, _, _) => {
            h286
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h287, _) => {
            h287
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h288, _) => {
            h288
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h289) => {
            h289
        }
    });
    Adt3::C0(match v2.clone() {
        Adt4::C0(h290, _, _, _) => {
            h290
        }
    }, match v3.clone() {
        Adt4::C0(h291, _, _, _) => {
            h291
        }
    }, match v4.clone() {
        Adt4::C0(h292, _, _, _) => {
            h292
        }
    }, match v5.clone() {
        Adt4::C0(h293, _, _, _) => {
            h293
        }
    }, match v5.clone() {
        Adt4::C0(_, h294, _, _) => {
            h294
        }
    }, match v2.clone() {
        Adt4::C0(_, h295, _, _) => {
            h295
        }
    }, match v3.clone() {
        Adt4::C0(_, h296, _, _) => {
            h296
        }
    }, match v4.clone() {
        Adt4::C0(_, h297, _, _) => {
            h297
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h298, _) => {
            h298
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h299, _) => {
            h299
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h300, _) => {
            h300
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h301, _) => {
            h301
        }
    }, match v3 {
        Adt4::C0(_, _, _, h302) => {
            h302
        }
    }, match v4 {
        Adt4::C0(_, _, _, h303) => {
            h303
        }
    }, match v5 {
        Adt4::C0(_, _, _, h304) => {
            h304
        }
    }, match v2 {
        Adt4::C0(_, _, _, h305) => {
            h305
        }
    })
}

pub fn f36(v0: Adt3, v1: Adt2) -> Adt3 {
    let v2: Adt4 = f38(match v0.clone() {
        Adt3::C0(h306, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h306
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h307, _, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h308, _, _, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h309, _, _, _) => {
            h309
        }
    }, match v1.clone() {
        Adt2::C0(h310, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h310
        }
    }, match v1.clone() {
        Adt2::C0(_, h311, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h311
        }
    });
    let v3: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, h312, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h313, _, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h314, _, _, _, _, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h315, _, _) => {
            h315
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h316
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h317
        }
    });
    let v4: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, _, h318, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h319, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h320, _, _, _, _, _) => {
            h320
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h321, _) => {
            h321
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h322, _, _, _, _, _, _, _, _, _, _, _) => {
            h322
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h323, _, _, _, _, _, _, _, _, _, _) => {
            h323
        }
    });
    let v5: Adt4 = f38(match v0.clone() {
        Adt3::C0(_, _, _, h324, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h325, _, _, _, _, _, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h326, _, _, _, _) => {
            h326
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h327) => {
            h327
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h328, _, _, _, _, _, _, _, _, _) => {
            h328
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h329, _, _, _, _, _, _, _, _) => {
            h329
        }
    });
    Adt3::C0(match v2.clone() {
        Adt4::C0(h330, _, _, _) => {
            h330
        }
    }, match v3.clone() {
        Adt4::C0(h331, _, _, _) => {
            h331
        }
    }, match v4.clone() {
        Adt4::C0(h332, _, _, _) => {
            h332
        }
    }, match v5.clone() {
        Adt4::C0(h333, _, _, _) => {
            h333
        }
    }, match v2.clone() {
        Adt4::C0(_, h334, _, _) => {
            h334
        }
    }, match v3.clone() {
        Adt4::C0(_, h335, _, _) => {
            h335
        }
    }, match v4.clone() {
        Adt4::C0(_, h336, _, _) => {
            h336
        }
    }, match v5.clone() {
        Adt4::C0(_, h337, _, _) => {
            h337
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h338, _) => {
            h338
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h339, _) => {
            h339
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h340, _) => {
            h340
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h341, _) => {
            h341
        }
    }, match v2 {
        Adt4::C0(_, _, _, h342) => {
            h342
        }
    }, match v3 {
        Adt4::C0(_, _, _, h343) => {
            h343
        }
    }, match v4 {
        Adt4::C0(_, _, _, h344) => {
            h344
        }
    }, match v5 {
        Adt4::C0(_, _, _, h345) => {
            h345
        }
    })
}

pub fn f37(v0: Adt2) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(_, _, h346, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h346
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h347, _, _, _, _, _, _, _, _, _) => {
            h347
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h348, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h348
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h349, _, _, _, _, _) => {
            h349
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h350, _, _, _, _, _, _, _, _) => {
            h350
        }
    }, match v0.clone() {
        Adt2::C0(h351, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h351
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h352, _, _, _, _, _, _, _, _, _, _, _) => {
            h352
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h353, _, _) => {
            h353
        }
    }, match v0.clone() {
        Adt2::C0(_, h354, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h354
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h355, _, _, _, _) => {
            h355
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h356, _, _, _) => {
            h356
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h357, _, _, _, _, _, _, _, _, _, _) => {
            h357
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h358, _, _, _, _, _, _) => {
            h358
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h359, _) => {
            h359
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h360) => {
            h360
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h361, _, _, _, _, _, _, _) => {
            h361
        }
    })
}

pub fn f38(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f39(f39(v0, v1), v4);
    let v7: u32 = f40({
        let a362 = v3;
        let a363 = v6;
        fixed_u32::bit_xor(a362, a363)
    }, 16u32, 16u32);
    let v8: u32 = f39(v2, v7);
    let v9: u32 = f40({
        let a364 = v1;
        let a365 = v8;
        fixed_u32::bit_xor(a364, a365)
    }, 12u32, 20u32);
    let v10: u32 = f39(f39(v6, v9), v5);
    let v11: u32 = f40({
        let a366 = v7;
        let a367 = v10;
        fixed_u32::bit_xor(a366, a367)
    }, 8u32, 24u32);
    let v12: u32 = f39(v8, v11);
    let v13: u32 = f40({
        let a368 = v9;
        let a369 = v12;
        fixed_u32::bit_xor(a368, a369)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f39(v0: u32, v1: u32) -> u32 {
    let m372 = {
        let a370 = f41(v0);
        let a371 = f41(v1);
        fixed_u64::checked_add(a370, a371)
    };
    match m372 {
        None => {
            0u32
        }
        Some(v2) => {
            f42(v2)
        }
    }
}

pub fn f40(v0: u32, v1: u32, v2: u32) -> u32 {
    let a373 = f43(v0, v1);
    let a374 = f34(v0, v2);
    fixed_u32::bit_or(a373, a374)
}

pub fn f41(v0: u32) -> u64 {
    let m376 = {
        let a375 = v0;
        fixed_u64::convert(i128::from(a375))
    };
    match m376 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f42(v0: u64) -> u32 {
    let m380 = {
        let a379 = {
            let a377 = v0;
            let a378 = 4294967295u64;
            fixed_u64::bit_and(a377, a378)
        };
        fixed_u32::convert(i128::from(a379))
    };
    match m380 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f43(v0: u32, v1: u32) -> u32 {
    let m383 = {
        let a381 = v0;
        let a382 = v1;
        fixed_u32::shift_right(a381, a382)
    };
    match m383 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}
