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
    C0(Bytes, Bytes, Bytes, List<(Bytes, Bytes)>, List<Adt1>, List<Adt2>, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Bytes, List<Bytes>, Bytes, List<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: Adt1) -> R<Bytes> {
        match self {
            Fn0::F8 => f8(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes, p1: Adt2) -> R<Bytes> {
        match self {
            Fn1::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> R<Bytes> {
        match self {
            Fn2::F17(k0) => f17(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn3::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: Adt1) -> R<List<(Bytes, Adt1)>> {
        match self {
            Fn4::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt2>, p1: (Bytes, Adt2)) -> List<Adt2> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F32,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: Adt2) -> List<(Bytes, Adt2)> {
        match self {
            Fn6::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F36(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn7::F36(k0) => f36(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F38,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F38 => f38(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F42,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, Bytes)) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn9::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F44,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn10::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F47,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn11::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F49,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn12::F49 => f49(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F52,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn13::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F55,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn14::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F57,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
        match self {
            Fn15::F57 => f57(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F61,
    F79,
    F104,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn16::F61 => f61(p0, p1),
            Fn16::F79 => Ok(f79(p0, p1)),
            Fn16::F104 => Ok(f104(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F64,
}

impl Fn17 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn17::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F67,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>, p1: (Bytes, (Bytes, Bytes))) -> List<List<(Bytes, (Bytes, Bytes))>> {
        match self {
            Fn18::F67 => f67(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F69,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
        match self {
            Fn19::F69 => f69(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F71,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn20::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F75,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn21::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F78(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn22::F78(k0) => f78(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F83,
}

impl Fn23 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>, p1: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
        match self {
            Fn23::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F93,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn24::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F95(u8, Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn25::F95(k0, k1) => f95(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F99,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
        match self {
            Fn26::F99 => f99(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F101,
}

impl Fn27 {
    pub fn apply(&self, p0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
        match self {
            Fn27::F101 => f101(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F111,
}

impl Fn28 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn28::F111 => f111(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F113,
}

impl Fn29 {
    pub fn apply(&self, p0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
        match self {
            Fn29::F113 => f113(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F115,
}

impl Fn30 {
    pub fn apply(&self, p0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
        match self {
            Fn30::F115 => f115(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let v1: Bytes = {
        let a2 = match v0.clone() {
            Adt0::C0(h1, _, _, _, _, _, _) => {
                h1
            }
        };
        let a3 = f1(Str::lit("1.2"));
        if equal(a2, a3) {
            f1(Str::lit("lexlean/lock/4"))
        } else {
            f1(Str::lit("lexlean/lock/3"))
        }
    };
    let v2: Bytes = f2(f2(f2(f2(f3(f1(Str::lit("spec")), v1)?, f3(f1(Str::lit("language")), match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _) => {
            h4
        }
    })?), f4(f1(Str::lit("compiler_semantics")), match v0.clone() {
        Adt0::C0(_, h5, _, _, _, _, _) => {
            h5
        }
    })?), f4(f1(Str::lit("project_config_kappa")), match v0.clone() {
        Adt0::C0(_, _, h6, _, _, _, _) => {
            h6
        }
    })?), f3(f1(Str::lit("lean_toolchain")), f5())?);
    let v3: Bytes = f6(f1(Str::lit("[[workspace_file]]")), match v0.clone() {
        Adt0::C0(_, _, _, h7, _, _, _) => {
            h7
        }
    })?;
    let v4: Bytes = f7(Fn0::F8, Bytes::lit(&[]), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, h8, _, _) => {
            h8
        }
    })?)?;
    let v5: Bytes = f10(Fn1::F11, Bytes::lit(&[]), f12(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h9, _) => {
            h9
        }
    })?)?;
    let v7: Bytes = {
        let m11 = match v0 {
            Adt0::C0(_, _, _, _, _, _, h10) => {
                h10
            }
        };
        match m11 {
            None => {
                Bytes::lit(&[])
            }
            Some(v6) => {
                f13(v6)?
            }
        }
    };
    Ok(f2(f2(f2(f2(v2, v3), v4), v5), v7))
}

pub fn f1(v0: Str) -> Bytes {
    let a12 = v0;
    utf8_encode(a12)
}

pub fn f2(v0: Bytes, v1: Bytes) -> Bytes {
    let a13 = v0;
    let a14 = v1;
    append_bytes(a13, a14)
}

pub fn f3(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f15(v1)?))
}

pub fn f4(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f15(v1)?))
}

pub fn f5() -> Bytes {
    f1(Str::lit("leanprover/lean4:v4.32.2"))
}

pub fn f6(v0: Bytes, v1: List<(Bytes, Bytes)>) -> R<Bytes> {
    f16(Fn2::F17(v0), Bytes::lit(&[]), f18(v1)?)
}

pub fn f7(v0: Fn0, p18: Bytes, p19: List<Adt1>) -> R<Bytes> {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v1, v2) = m17;
                let m15 = v2;
                match m15.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt1>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt1>), R<Bytes>>(({
                            let c16: Fn0 = v0.clone();
                            c16.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: Adt1) -> R<Bytes> {
    Ok(f2(v0, f19(v1)?))
}

pub fn f9(v0: List<Adt1>) -> R<List<Adt1>> {
    Ok(f20(Fn3::F21, List::<Adt1>::nil(), f22(f23(f22(f24(Fn4::F25, List::<(Bytes, Adt1)>::nil(), v0)?))?)))
}

pub fn f10(v0: Fn1, p23: Bytes, p24: List<Adt2>) -> R<Bytes> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m20 = v2;
                match m20.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt2>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt2>), R<Bytes>>(({
                            let c21: Fn1 = v0.clone();
                            c21.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m22 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: Adt2) -> R<Bytes> {
    Ok(f2(v0, f26(v1)?))
}

pub fn f12(v0: List<Adt2>) -> R<List<Adt2>> {
    Ok(f27(Fn5::F28, List::<Adt2>::nil(), f29(f30(f29(f31(Fn6::F32, List::<(Bytes, Adt2)>::nil(), v0)))?)))
}

pub fn f13(v0: Adt3) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f33(f1(Str::lit("[pdf]"))), f3(f1(Str::lit("program")), match v0.clone() {
        Adt3::C0(h25, _, _, _, _, _, _) => {
            h25
        }
    })?), f4(f1(Str::lit("program_kappa")), match v0.clone() {
        Adt3::C0(_, h26, _, _, _, _, _) => {
            h26
        }
    })?), f14(f1(Str::lit("version_argv")), f34(match v0.clone() {
        Adt3::C0(_, _, h27, _, _, _, _) => {
            h27
        }
    })?)), f4(f1(Str::lit("version_stdout_kappa")), match v0.clone() {
        Adt3::C0(_, _, _, h28, _, _, _) => {
            h28
        }
    })?), f14(f1(Str::lit("compile_argv")), f34(match v0.clone() {
        Adt3::C0(_, _, _, _, h29, _, _) => {
            h29
        }
    })?)), f3(f1(Str::lit("output")), match v0.clone() {
        Adt3::C0(_, _, _, _, _, h30, _) => {
            h30
        }
    })?), f6(f1(Str::lit("[[pdf.resource]]")), match v0 {
        Adt3::C0(_, _, _, _, _, _, h31) => {
            h31
        }
    })?))
}

pub fn f14(v0: Bytes, v1: Bytes) -> Bytes {
    f2(f2(f2(v0, f1(Str::lit(" = "))), v1), Bytes::lit(&[0x0a]))
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    Ok(f2(f2(Bytes::lit(&[0x22]), {
        let (_, h32) = {
            let (h33, _) = f35(Fn7::F36(v0.clone()), {
                let a35 = {
                    let a34 = v0;
                    length_bytes(a34)
                };
                let a36 = 1u64;
                nat_add(a35, a36)?
            }, (0u64, Bytes::lit(&[])))?;
            h33
        };
        h32
    }), Bytes::lit(&[0x22])))
}

pub fn f16(v0: Fn2, p40: Bytes, p41: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m39 = (p40, p41);
    {
        let mut m39 = m39;
        loop {
            let step = {
                let (v1, v2) = m39;
                let m37 = v2;
                match m37.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), R<Bytes>>(({
                            let c38: Fn2 = v0.clone();
                            c38.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m39 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Bytes, v2: (Bytes, Bytes)) -> R<Bytes> {
    Ok(f2(v1, f2(f2(f33(v0), f3(f1(Str::lit("path")), {
        let (h42, _) = v2;
        h42
    })?), f4(f1(Str::lit("kappa")), {
        let (_, h43) = v2;
        h43
    })?)))
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    Ok(f37(Fn8::F38, List::<(Bytes, Bytes)>::nil(), f39(f40(f39(f41(Fn9::F42, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)))?)))
}

pub fn f19(v0: Adt1) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f2(f33(f1(Str::lit("[[package]]"))), f3(f1(Str::lit("id")), match v0.clone() {
        Adt1::C0(h44, _, _, _, _, _, _, _) => {
            h44
        }
    })?), f3(f1(Str::lit("version")), match v0.clone() {
        Adt1::C0(_, h45, _, _, _, _, _, _) => {
            h45
        }
    })?), f3(f1(Str::lit("kind")), match v0.clone() {
        Adt1::C0(_, _, h46, _, _, _, _, _) => {
            h46
        }
    })?), f3(f1(Str::lit("source")), match v0.clone() {
        Adt1::C0(_, _, _, h47, _, _, _, _) => {
            h47
        }
    })?), f3(f1(Str::lit("revision")), match v0.clone() {
        Adt1::C0(_, _, _, _, h48, _, _, _) => {
            h48
        }
    })?), f4(f1(Str::lit("tree_kappa")), match v0.clone() {
        Adt1::C0(_, _, _, _, _, h49, _, _) => {
            h49
        }
    })?), f4(f1(Str::lit("manifest_kappa")), match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h50, _) => {
            h50
        }
    })?), f14(f1(Str::lit("imports")), f34(match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h51) => {
            h51
        }
    })?)))
}

pub fn f20(v0: Fn3, p55: List<Adt1>, p56: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v1, v2) = m54;
                let m52 = v2;
                match m52.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c53: Fn3 = v0.clone();
                            c53.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h57) = v1;
        h57
    }, v0)
}

pub fn f22(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f43(Fn10::F44, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f23(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f45(f46(Fn11::F47, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m62 = {
        let (h58, _) = f48(Fn12::F49, {
            let a60 = {
                let a59 = v0;
                length_list(a59)
            };
            let a61 = 1u64;
            nat_add(a60, a61)?
        }, v1)?;
        h58
    };
    match m62.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f24(v0: Fn4, p66: List<(Bytes, Adt1)>, p67: List<Adt1>) -> R<List<(Bytes, Adt1)>> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v2;
                match m63.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<Adt1>), R<List<(Bytes, Adt1)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<Adt1>), R<List<(Bytes, Adt1)>>>(({
                            let c64: Fn4 = v0.clone();
                            c64.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: List<(Bytes, Adt1)>, v1: Adt1) -> R<List<(Bytes, Adt1)>> {
    Ok(List::cons((f50(v1.clone())?, v1), v0))
}

pub fn f26(v0: Adt2) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f2(f33(f1(Str::lit("[[source_package]]"))), f3(f1(Str::lit("id")), match v0.clone() {
        Adt2::C0(h68, _, _, _, _, _, _, _) => {
            h68
        }
    })?), f3(f1(Str::lit("version")), match v0.clone() {
        Adt2::C0(_, h69, _, _, _, _, _, _) => {
            h69
        }
    })?), f3(f1(Str::lit("kind")), match v0.clone() {
        Adt2::C0(_, _, h70, _, _, _, _, _) => {
            h70
        }
    })?), f3(f1(Str::lit("source")), match v0.clone() {
        Adt2::C0(_, _, _, h71, _, _, _, _) => {
            h71
        }
    })?), f3(f1(Str::lit("module_prefix")), match v0.clone() {
        Adt2::C0(_, _, _, _, h72, _, _, _) => {
            h72
        }
    })?), f4(f1(Str::lit("tree_kappa")), match v0.clone() {
        Adt2::C0(_, _, _, _, _, h73, _, _) => {
            h73
        }
    })?), f4(f1(Str::lit("manifest_kappa")), match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h74, _) => {
            h74
        }
    })?), f14(f1(Str::lit("imports")), f34(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h75) => {
            h75
        }
    })?)))
}

pub fn f27(v0: Fn5, p79: List<Adt2>, p80: List<(Bytes, Adt2)>) -> List<Adt2> {
    let m78 = (p79, p80);
    {
        let mut m78 = m78;
        loop {
            let step = {
                let (v1, v2) = m78;
                let m76 = v2;
                match m76.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(({
                            let c77: Fn5 = v0.clone();
                            c77.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m78 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: (Bytes, Adt2)) -> List<Adt2> {
    List::cons({
        let (_, h81) = v1;
        h81
    }, v0)
}

pub fn f29(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f51(Fn13::F52, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f30(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v1: List<List<(Bytes, Adt2)>> = f53(f54(Fn14::F55, List::<List<(Bytes, Adt2)>>::nil(), v0.clone()));
    let m86 = {
        let (h82, _) = f56(Fn15::F57, {
            let a84 = {
                let a83 = v0;
                length_list(a83)
            };
            let a85 = 1u64;
            nat_add(a84, a85)?
        }, v1)?;
        h82
    };
    match m86.uncons() {
        None => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f31(v0: Fn6, p90: List<(Bytes, Adt2)>, p91: List<Adt2>) -> List<(Bytes, Adt2)> {
    let m89 = (p90, p91);
    {
        let mut m89 = m89;
        loop {
            let step = {
                let (v1, v2) = m89;
                let m87 = v2;
                match m87.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<Adt2>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<Adt2>), List<(Bytes, Adt2)>>(({
                            let c88: Fn6 = v0.clone();
                            c88.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m89 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt2)>, v1: Adt2) -> List<(Bytes, Adt2)> {
    List::cons((match v1.clone() {
        Adt2::C0(h92, _, _, _, _, _, _, _) => {
            h92
        }
    }, v1), v0)
}

pub fn f33(v0: Bytes) -> Bytes {
    f2(f2(Bytes::lit(&[0x0a]), v0), Bytes::lit(&[0x0a]))
}

pub fn f34(v0: List<Bytes>) -> R<Bytes> {
    Ok(f2(f2(f1(Str::lit("[")), f58(f59(f60(Fn16::F61, List::<Bytes>::nil(), v0)?)?, f1(Str::lit(", ")))), f1(Str::lit("]"))))
}

pub fn f35(v0: Fn7, p97: u64, p98: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m93 = v1;
                if m93 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m93 - 1;
                    let m95 = {
                        let c94: Fn7 = v0.clone();
                        c94.apply(v2.clone())?
                    };
                    match m95 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f62(v0, v1)
}

pub fn f37(v0: Fn8, p102: List<(Bytes, Bytes)>, p103: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, Bytes)> {
    let m101 = (p102, p103);
    {
        let mut m101 = m101;
        loop {
            let step = {
                let (v1, v2) = m101;
                let m99 = v2;
                match m99.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, Bytes)>>(({
                            let c100: Fn8 = v0.clone();
                            c100.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Bytes)>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, Bytes)> {
    List::cons({
        let (_, h104) = v1;
        h104
    }, v0)
}

pub fn f39(v0: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    f63(Fn17::F64, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)
}

pub fn f40(v0: List<(Bytes, (Bytes, Bytes))>) -> R<List<(Bytes, (Bytes, Bytes))>> {
    let v1: List<List<(Bytes, (Bytes, Bytes))>> = f65(f66(Fn18::F67, List::<List<(Bytes, (Bytes, Bytes))>>::nil(), v0.clone()));
    let m109 = {
        let (h105, _) = f68(Fn19::F69, {
            let a107 = {
                let a106 = v0;
                length_list(a106)
            };
            let a108 = 1u64;
            nat_add(a107, a108)?
        }, v1)?;
        h105
    };
    match m109.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, Bytes))>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f41(v0: Fn9, p113: List<(Bytes, (Bytes, Bytes))>, p114: List<(Bytes, Bytes)>) -> List<(Bytes, (Bytes, Bytes))> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m110 = v2;
                match m110.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, Bytes)>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, Bytes)>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c111: Fn9 = v0.clone();
                            c111.apply(v1, v3)
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

pub fn f42(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, Bytes)) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(({
        let (h115, _) = v1.clone();
        h115
    }, v1), v0)
}

pub fn f43(v0: Fn10, p119: List<(Bytes, Adt1)>, p120: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m116 = v2;
                match m116.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c117: Fn10 = v0.clone();
                            c117.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f45(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f70(Fn20::F71, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f46(v0: Fn11, p124: List<List<(Bytes, Adt1)>>, p125: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v1, v2) = m123;
                let m121 = v2;
                match m121.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c122: Fn11 = v0.clone();
                            c122.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f48(v0: Fn12, p130: u64, p131: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v1, v2) = m129;
                let m126 = v1;
                if m126 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m126 - 1;
                    let m128 = {
                        let c127: Fn12 = v0.clone();
                        c127.apply(v2.clone())?
                    };
                    match m128 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f72(v0)
}

pub fn f50(v0: Adt1) -> R<Bytes> {
    Ok(f2(f73(match v0.clone() {
        Adt1::C0(h132, _, _, _, _, _, _, _) => {
            h132
        }
    })?, f73(match v0 {
        Adt1::C0(_, h133, _, _, _, _, _, _) => {
            h133
        }
    })?))
}

pub fn f51(v0: Fn13, p137: List<(Bytes, Adt2)>, p138: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m136 = (p137, p138);
    {
        let mut m136 = m136;
        loop {
            let step = {
                let (v1, v2) = m136;
                let m134 = v2;
                match m134.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c135: Fn13 = v0.clone();
                            c135.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f53(v0: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    f74(Fn21::F75, List::<List<(Bytes, Adt2)>>::nil(), v0)
}

pub fn f54(v0: Fn14, p142: List<List<(Bytes, Adt2)>>, p143: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    let m141 = (p142, p143);
    {
        let mut m141 = m141;
        loop {
            let step = {
                let (v1, v2) = m141;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(({
                            let c140: Fn14 = v0.clone();
                            c140.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<List<(Bytes, Adt2)>>, v1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt2)>::nil()), v0)
}

pub fn f56(v0: Fn15, p148: u64, p149: List<List<(Bytes, Adt2)>>) -> R<(List<List<(Bytes, Adt2)>>, bool)> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v1, v2) = m147;
                let m144 = v1;
                if m144 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m144 - 1;
                    let m146 = {
                        let c145: Fn15 = v0.clone();
                        c145.apply(v2.clone())?
                    };
                    match m146 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    f76(v0)
}

pub fn f58(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m150 = f77(Fn22::F78(v1), None::<Bytes>, v0);
    match m150 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f59(v0: List<Bytes>) -> R<List<Bytes>> {
    f60(Fn16::F79, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Fn16, p154: List<Bytes>, p155: List<Bytes>) -> R<List<Bytes>> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v1, v2) = m153;
                let m151 = v2;
                match m151.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c152: Fn16 = v0.clone();
                            c152.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f15(v1)?, v0))
}

pub fn f62(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m157 = f80(v0, {
        let (h156, _) = v1.clone();
        h156
    });
    match m157 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a159 = {
                    let (h158, _) = v1;
                    h158
                };
                let a160 = 1u64;
                nat_add(a159, a160)?
            }, f2({
                let (_, h161) = v1;
                h161
            }, f81({
                let v3: u8 = v2;
                let a206 = {
                    let a182 = {
                        let a170 = {
                            let a164 = {
                                let a162 = v3;
                                let a163 = 128u8;
                                fixed_u8::bit_and(a162, a163)
                            };
                            let a165 = 128u8;
                            if equal(a164, a165) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a171 = {
                            let a168 = {
                                let a166 = v3;
                                let a167 = 64u8;
                                fixed_u8::bit_and(a166, a167)
                            };
                            let a169 = 64u8;
                            if equal(a168, a169) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a170, a171)?
                    };
                    let a183 = {
                        let a180 = {
                            let a174 = {
                                let a172 = v3;
                                let a173 = 32u8;
                                fixed_u8::bit_and(a172, a173)
                            };
                            let a175 = 32u8;
                            if equal(a174, a175) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a181 = {
                            let a178 = {
                                let a176 = v3;
                                let a177 = 16u8;
                                fixed_u8::bit_and(a176, a177)
                            };
                            let a179 = 16u8;
                            if equal(a178, a179) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a180, a181)?
                    };
                    nat_add(a182, a183)?
                };
                let a207 = {
                    let a204 = {
                        let a192 = {
                            let a186 = {
                                let a184 = v3;
                                let a185 = 8u8;
                                fixed_u8::bit_and(a184, a185)
                            };
                            let a187 = 8u8;
                            if equal(a186, a187) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a193 = {
                            let a190 = {
                                let a188 = v3;
                                let a189 = 4u8;
                                fixed_u8::bit_and(a188, a189)
                            };
                            let a191 = 4u8;
                            if equal(a190, a191) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a192, a193)?
                    };
                    let a205 = {
                        let a202 = {
                            let a196 = {
                                let a194 = v3;
                                let a195 = 2u8;
                                fixed_u8::bit_and(a194, a195)
                            };
                            let a197 = 2u8;
                            if equal(a196, a197) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a203 = {
                            let a200 = {
                                let a198 = v3;
                                let a199 = 1u8;
                                fixed_u8::bit_and(a198, a199)
                            };
                            let a201 = 1u8;
                            if equal(a200, a201) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a202, a203)?
                    };
                    nat_add(a204, a205)?
                };
                nat_add(a206, a207)?
            })))))
        }
    }
}

pub fn f63(v0: Fn17, p211: List<(Bytes, (Bytes, Bytes))>, p212: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m210 = (p211, p212);
    {
        let mut m210 = m210;
        loop {
            let step = {
                let (v1, v2) = m210;
                let m208 = v2;
                match m208.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c209: Fn17 = v0.clone();
                            c209.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(v1, v0)
}

pub fn f65(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    f82(Fn23::F83, List::<List<(Bytes, (Bytes, Bytes))>>::nil(), v0)
}

pub fn f66(v0: Fn18, p216: List<List<(Bytes, (Bytes, Bytes))>>, p217: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    let m215 = (p216, p217);
    {
        let mut m215 = m215;
        loop {
            let step = {
                let (v1, v2) = m215;
                let m213 = v2;
                match m213.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, (Bytes, Bytes))>>, List<(Bytes, (Bytes, Bytes))>), List<List<(Bytes, (Bytes, Bytes))>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, (Bytes, Bytes))>>, List<(Bytes, (Bytes, Bytes))>), List<List<(Bytes, (Bytes, Bytes))>>>(({
                            let c214: Fn18 = v0.clone();
                            c214.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<List<(Bytes, (Bytes, Bytes))>>, v1: (Bytes, (Bytes, Bytes))) -> List<List<(Bytes, (Bytes, Bytes))>> {
    List::cons(List::cons(v1, List::<(Bytes, (Bytes, Bytes))>::nil()), v0)
}

pub fn f68(v0: Fn19, p222: u64, p223: List<List<(Bytes, (Bytes, Bytes))>>) -> R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)> {
    let m221 = (p222, p223);
    {
        let mut m221 = m221;
        loop {
            let step = {
                let (v1, v2) = m221;
                let m218 = v1;
                if m218 == 0 {
                    Err::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m218 - 1;
                    let m220 = {
                        let c219: Fn19 = v0.clone();
                        c219.apply(v2.clone())?
                    };
                    match m220 {
                        None => {
                            Err::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
    f84(v0)
}

pub fn f70(v0: Fn20, p227: List<List<(Bytes, Adt1)>>, p228: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m226 = (p227, p228);
    {
        let mut m226 = m226;
        loop {
            let step = {
                let (v1, v2) = m226;
                let m224 = v2;
                match m224.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c225: Fn20 = v0.clone();
                            c225.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f72(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a230 = {
        let a229 = v0.clone();
        length_list(a229)
    };
    let a231 = 1u64;
    if nat_le(a230, a231) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f85(v0)?))
    }
}

pub fn f73(v0: Bytes) -> R<Bytes> {
    Ok(f2(f58(f86(v0, 0u8)?, Bytes::lit(&[0x00, 0x01])), Bytes::lit(&[0x00, 0x00])))
}

pub fn f74(v0: Fn21, p235: List<List<(Bytes, Adt2)>>, p236: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    let m234 = (p235, p236);
    {
        let mut m234 = m234;
        loop {
            let step = {
                let (v1, v2) = m234;
                let m232 = v2;
                match m232.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(({
                            let c233: Fn21 = v0.clone();
                            c233.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<List<(Bytes, Adt2)>>, v1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    List::cons(v1, v0)
}

pub fn f76(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    let a238 = {
        let a237 = v0.clone();
        length_list(a237)
    };
    let a239 = 1u64;
    if nat_le(a238, a239) {
        Ok(None::<List<List<(Bytes, Adt2)>>>)
    } else {
        Ok(Some(f87(v0)?))
    }
}

pub fn f77(v0: Fn22, p243: Option<Bytes>, p244: List<Bytes>) -> Option<Bytes> {
    let m242 = (p243, p244);
    {
        let mut m242 = m242;
        loop {
            let step = {
                let (v1, v2) = m242;
                let m240 = v2;
                match m240.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c241: Fn22 = v0.clone();
                            c241.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m245 = v1;
    match m245 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a248 = {
                    let a246 = v3;
                    let a247 = v0;
                    append_bytes(a246, a247)
                };
                let a249 = v2;
                append_bytes(a248, a249)
            })
        }
    }
}

pub fn f79(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f80(v0: Bytes, v1: u64) -> Option<u8> {
    let a250 = v0;
    let a251 = v1;
    index_bytes(a250, a251)
}

pub fn f81(v0: u64) -> Bytes {
    let a252 = v0;
    let a253 = 34u64;
    if nat_eq(a252, a253) {
        Bytes::lit(&[0x5c, 0x22])
    } else {
        let a254 = v0;
        let a255 = 92u64;
        if nat_eq(a254, a255) {
            Bytes::lit(&[0x5c, 0x5c])
        } else {
            let a256 = v0;
            let a257 = 32u64;
            if if nat_lt(a256, a257) {
                true
            } else {
                let a258 = v0;
                let a259 = 127u64;
                nat_eq(a258, a259)
            } {
                f2(f2(Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]), f88({
                    let a260 = v0;
                    let a261 = 16u64;
                    let a262 = 0u64;
                    nat_quot(a260, a261, a262)
                })), f88({
                    let a263 = v0;
                    let a264 = 16u64;
                    let a265 = 0u64;
                    nat_rem(a263, a264, a265)
                }))
            } else {
                f89(f90(), v0)
            }
        }
    }
}

pub fn f82(v0: Fn23, p269: List<List<(Bytes, (Bytes, Bytes))>>, p270: List<List<(Bytes, (Bytes, Bytes))>>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m266 = v2;
                match m266.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), List<List<(Bytes, (Bytes, Bytes))>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), List<List<(Bytes, (Bytes, Bytes))>>>(({
                            let c267: Fn23 = v0.clone();
                            c267.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<List<(Bytes, (Bytes, Bytes))>>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    List::cons(v1, v0)
}

pub fn f84(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
    let a272 = {
        let a271 = v0.clone();
        length_list(a271)
    };
    let a273 = 1u64;
    if nat_le(a272, a273) {
        Ok(None::<List<List<(Bytes, (Bytes, Bytes))>>>)
    } else {
        Ok(Some(f91(v0)?))
    }
}

pub fn f85(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f45({
        let (_, h274) = {
            let (h275, _) = f92(Fn24::F93, {
                let a277 = {
                    let a276 = v0.clone();
                    length_list(a276)
                };
                let a278 = 1u64;
                nat_add(a277, a278)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h275
        };
        h274
    }))
}

pub fn f86(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a279 = v0.clone();
        length_bytes(a279)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h280, _) = f94(Fn25::F95(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h280
    };
    let v4: u64 = {
        let (h281, _) = {
            let (_, h282) = v3.clone();
            h282
        };
        h281
    };
    f96(List::cons(f97(v0, v4, {
        let a283 = v2;
        let a284 = v4;
        nat_sub(a283, a284)
    }), {
        let (_, h285) = {
            let (_, h286) = v3;
            h286
        };
        h285
    }))
}

pub fn f87(v0: List<List<(Bytes, Adt2)>>) -> R<List<List<(Bytes, Adt2)>>> {
    Ok(f53({
        let (_, h287) = {
            let (h288, _) = f98(Fn26::F99, {
                let a290 = {
                    let a289 = v0.clone();
                    length_list(a289)
                };
                let a291 = 1u64;
                nat_add(a290, a291)?
            }, (v0, List::<List<(Bytes, Adt2)>>::nil()))?;
            h288
        };
        h287
    }))
}

pub fn f88(v0: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v0)
}

pub fn f89(v0: Bytes, v1: u64) -> Bytes {
    let m295 = {
        let a292 = v0;
        let a293 = v1;
        let a294 = 1u64;
        slice_bytes(a292, a293, a294)
    };
    match m295 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f90() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f91(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<List<List<(Bytes, (Bytes, Bytes))>>> {
    Ok(f65({
        let (_, h296) = {
            let (h297, _) = f100(Fn27::F101, {
                let a299 = {
                    let a298 = v0.clone();
                    length_list(a298)
                };
                let a300 = 1u64;
                nat_add(a299, a300)?
            }, (v0, List::<List<(Bytes, (Bytes, Bytes))>>::nil()))?;
            h297
        };
        h296
    }))
}

pub fn f92(v0: Fn24, p305: u64, p306: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m304 = (p305, p306);
    {
        let mut m304 = m304;
        loop {
            let step = {
                let (v1, v2) = m304;
                let m301 = v1;
                if m301 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m301 - 1;
                    let m303 = {
                        let c302: Fn24 = v0.clone();
                        c302.apply(v2.clone())?
                    };
                    match m303 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f102(v0)
}

pub fn f94(v0: Fn25, p311: u64, p312: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m310 = (p311, p312);
    {
        let mut m310 = m310;
        loop {
            let step = {
                let (v1, v2) = m310;
                let m307 = v1;
                if m307 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m307 - 1;
                    let m309 = {
                        let c308: Fn25 = v0.clone();
                        c308.apply(v2.clone())?
                    };
                    match m309 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f103(v1, v0, v2)
}

pub fn f96(v0: List<Bytes>) -> R<List<Bytes>> {
    f60(Fn16::F104, List::<Bytes>::nil(), v0)
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m316 = {
        let a313 = v0;
        let a314 = v1;
        let a315 = v2;
        slice_bytes(a313, a314, a315)
    };
    match m316 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f98(v0: Fn26, p321: u64, p322: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m317 = v1;
                if m317 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m317 - 1;
                    let m319 = {
                        let c318: Fn26 = v0.clone();
                        c318.apply(v2.clone())?
                    };
                    match m319 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    f105(v0)
}

pub fn f100(v0: Fn27, p327: u64, p328: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)> {
    let m326 = (p327, p328);
    {
        let mut m326 = m326;
        loop {
            let step = {
                let (v1, v2) = m326;
                let m323 = v1;
                if m323 == 0 {
                    Err::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m323 - 1;
                    let m325 = {
                        let c324: Fn27 = v0.clone();
                        c324.apply(v2.clone())?
                    };
                    match m325 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
    f106(v0)
}

pub fn f102(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h329, _) = v0;
        h329
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h330) = v0;
        h330
    };
    let m331 = v1;
    match m331.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m332 = v4;
            match m332.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f107(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h333, _) = v2.clone();
        h333
    };
    let v4: u64 = {
        let (h334, _) = {
            let (_, h335) = v2.clone();
            h335
        };
        h334
    };
    let v5: List<Bytes> = {
        let (_, h336) = {
            let (_, h337) = v2;
            h337
        };
        h336
    };
    let m338 = f80(v0.clone(), v3);
    match m338 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a339 = v6;
            let a340 = v1;
            if equal(a339, a340) {
                Ok(Some(({
                    let a341 = v3;
                    let a342 = 1u64;
                    nat_add(a341, a342)?
                }, ({
                    let a343 = v3;
                    let a344 = 1u64;
                    nat_add(a343, a344)?
                }, List::cons(f97(v0, v4, {
                    let a345 = v3;
                    let a346 = v4;
                    nat_sub(a345, a346)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a347 = v3;
                    let a348 = 1u64;
                    nat_add(a347, a348)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f104(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f105(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    let v1: List<List<(Bytes, Adt2)>> = {
        let (h349, _) = v0;
        h349
    };
    let v2: List<List<(Bytes, Adt2)>> = {
        let (_, h350) = v0;
        h350
    };
    let m351 = v1;
    match m351.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>)
        }
        Some((v3, v4)) => {
            let m352 = v4;
            match m352.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt2)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f108(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f106(v0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
    let v1: List<List<(Bytes, (Bytes, Bytes))>> = {
        let (h353, _) = v0;
        h353
    };
    let v2: List<List<(Bytes, (Bytes, Bytes))>> = {
        let (_, h354) = v0;
        h354
    };
    let m355 = v1;
    match m355.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>)
        }
        Some((v3, v4)) => {
            let m356 = v4;
            match m356.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, (Bytes, Bytes))>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f109(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f107(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a361 = {
            let a359 = {
                let a357 = v0.clone();
                length_list(a357)
            };
            let a360 = {
                let a358 = v1.clone();
                length_list(a358)
            };
            nat_add(a359, a360)?
        };
        let a362 = 1u64;
        nat_add(a361, a362)?
    };
    Ok(f22({
        let (_, h363) = {
            let (_, h364) = {
                let (h365, _) = f110(Fn28::F111, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h365
            };
            h364
        };
        h363
    }))
}

pub fn f108(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v2: u64 = {
        let a370 = {
            let a368 = {
                let a366 = v0.clone();
                length_list(a366)
            };
            let a369 = {
                let a367 = v1.clone();
                length_list(a367)
            };
            nat_add(a368, a369)?
        };
        let a371 = 1u64;
        nat_add(a370, a371)?
    };
    Ok(f29({
        let (_, h372) = {
            let (_, h373) = {
                let (h374, _) = f112(Fn29::F113, v2, (v0, (v1, List::<(Bytes, Adt2)>::nil())));
                h374
            };
            h373
        };
        h372
    }))
}

pub fn f109(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> R<List<(Bytes, (Bytes, Bytes))>> {
    let v2: u64 = {
        let a379 = {
            let a377 = {
                let a375 = v0.clone();
                length_list(a375)
            };
            let a378 = {
                let a376 = v1.clone();
                length_list(a376)
            };
            nat_add(a377, a378)?
        };
        let a380 = 1u64;
        nat_add(a379, a380)?
    };
    Ok(f39({
        let (_, h381) = {
            let (_, h382) = {
                let (h383, _) = f114(Fn30::F115, v2, (v0, (v1, List::<(Bytes, (Bytes, Bytes))>::nil())));
                h383
            };
            h382
        };
        h381
    }))
}

pub fn f110(v0: Fn28, p388: u64, p389: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m387 = (p388, p389);
    {
        let mut m387 = m387;
        loop {
            let step = {
                let (v1, v2) = m387;
                let m384 = v1;
                if m384 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m384 - 1;
                    let m386 = {
                        let c385: Fn28 = v0.clone();
                        c385.apply(v2.clone())
                    };
                    match m386 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f116(v0)
}

pub fn f112(v0: Fn29, p394: u64, p395: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool) {
    let m393 = (p394, p395);
    {
        let mut m393 = m393;
        loop {
            let step = {
                let (v1, v2) = m393;
                let m390 = v1;
                if m390 == 0 {
                    Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, false))
                } else {
                    let v3 = m390 - 1;
                    let m392 = {
                        let c391: Fn29 = v0.clone();
                        c391.apply(v2.clone())
                    };
                    match m392 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    f117(v0)
}

pub fn f114(v0: Fn30, p400: u64, p401: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool) {
    let m399 = (p400, p401);
    {
        let mut m399 = m399;
        loop {
            let step = {
                let (v1, v2) = m399;
                let m396 = v1;
                if m396 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v2, false))
                } else {
                    let v3 = m396 - 1;
                    let m398 = {
                        let c397: Fn30 = v0.clone();
                        c397.apply(v2.clone())
                    };
                    match m398 {
                        None => {
                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
    f118(v0)
}

pub fn f116(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h402, _) = v0.clone();
        h402
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h403, _) = {
            let (_, h404) = v0.clone();
            h404
        };
        h403
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h405) = {
            let (_, h406) = v0;
            h406
        };
        h405
    };
    let m407 = v1.clone();
    match m407.uncons() {
        None => {
            let m408 = v2;
            match m408.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m409 = v2.clone();
            match m409.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f119({
                        let (h410, _) = v8.clone();
                        h410
                    }, {
                        let (h411, _) = v6.clone();
                        h411
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

pub fn f117(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    let v1: List<(Bytes, Adt2)> = {
        let (h412, _) = v0.clone();
        h412
    };
    let v2: List<(Bytes, Adt2)> = {
        let (h413, _) = {
            let (_, h414) = v0.clone();
            h414
        };
        h413
    };
    let v3: List<(Bytes, Adt2)> = {
        let (_, h415) = {
            let (_, h416) = v0;
            h416
        };
        h415
    };
    let m417 = v1.clone();
    match m417.uncons() {
        None => {
            let m418 = v2;
            match m418.uncons() {
                None => {
                    None::<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt2)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m419 = v2.clone();
            match m419.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt2)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f119({
                        let (h420, _) = v8.clone();
                        h420
                    }, {
                        let (h421, _) = v6.clone();
                        h421
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

pub fn f118(v0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
    let v1: List<(Bytes, (Bytes, Bytes))> = {
        let (h422, _) = v0.clone();
        h422
    };
    let v2: List<(Bytes, (Bytes, Bytes))> = {
        let (h423, _) = {
            let (_, h424) = v0.clone();
            h424
        };
        h423
    };
    let v3: List<(Bytes, (Bytes, Bytes))> = {
        let (_, h425) = {
            let (_, h426) = v0;
            h426
        };
        h425
    };
    let m427 = v1.clone();
    match m427.uncons() {
        None => {
            let m428 = v2;
            match m428.uncons() {
                None => {
                    None::<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, (Bytes, Bytes))>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m429 = v2.clone();
            match m429.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, (Bytes, Bytes))>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f119({
                        let (h430, _) = v8.clone();
                        h430
                    }, {
                        let (h431, _) = v6.clone();
                        h431
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

pub fn f119(v0: Bytes, v1: Bytes) -> bool {
    let a436 = {
        let a432 = v0;
        let a433 = v1;
        compare_bytes(a432, a433)
    };
    let a437 = {
        let a434 = Bytes::lit(&[]);
        let a435 = Bytes::lit(&[0x01]);
        compare_bytes(a434, a435)
    };
    equal(a436, a437)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
