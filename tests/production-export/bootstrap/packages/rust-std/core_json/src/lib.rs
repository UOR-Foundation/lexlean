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
    C0(Bytes, List<Bytes>, List<Adt1>, List<u64>, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(std::rc::Rc<Adt2>),
    C2(Bytes, List<Adt2>),
    C3(u64, u64),
    C4(Bytes, Adt3, u64, u64),
    C5(Bytes, Adt3, u64, u64),
    C6(Bytes, u64, u64, u64, bool),
    C7(Bytes),
    C8(List<u64>),
    C9(List<Adt4>, u64),
    C10(Bytes, Bytes, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(std::rc::Rc<Adt2>),
    C2(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool),
    C1(Bytes),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt6>),
    C4(std::rc::Rc<Adt6>),
    C5,
    C6(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C7(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt6) -> R<Bytes> {
        match self {
            Fn0::F4 => f4(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes) -> R<Bytes> {
        match self {
            Fn1::F7 => f7(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F9,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt1) -> R<Bytes> {
        match self {
            Fn2::F9 => f9(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F11,
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Bytes> {
        match self {
            Fn3::F11 => f11(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F14(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> (bool, Bytes) {
        match self {
            Fn4::F14(k0) => f14(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F16(Bytes),
    F63(Bytes),
    F71(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F16(k0) => f16(k0.clone(), p0),
            Fn5::F63(k0) => f63(k0.clone(), p0),
            Fn5::F71(k0, k1) => Ok(f71(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F19(Fn0),
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt6) -> R<List<Bytes>> {
        match self {
            Fn6::F19(k0) => f19(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F22(Fn1),
    F32,
    F77,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn7::F22(k0) => f22(k0.clone(), p0, p1),
            Fn7::F32 => Ok(f32(p0, p1)),
            Fn7::F77 => Ok(f77(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F24(Fn2),
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt1) -> R<List<Bytes>> {
        match self {
            Fn8::F24(k0) => f24(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F27(Fn3),
    F37,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: u64) -> R<List<Bytes>> {
        match self {
            Fn9::F27(k0) => f27(k0.clone(), p0, p1),
            Fn9::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F39,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt4) -> R<List<Bytes>> {
        match self {
            Fn10::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F42(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn11::F42(k0, k1) => f42(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F55,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> R<List<Bytes>> {
        match self {
            Fn12::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F66,
    F114,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn13::F66 => f66(p0, p1),
            Fn13::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F69,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn14::F69 => f69(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F76,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>) -> R<Option<List<Bytes>>> {
        match self {
            Fn15::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F80(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn16::F80(k0) => f80(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F87(u64, Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn17::F87(k0, k1) => f87(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F92,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn18::F92 => f92(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F94,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn19::F94 => f94(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F99,
}

impl Fn20 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn20::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F102,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F108,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F113,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn23::F113 => f113(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    Ok(f1(List::cons(f2(Str::lit("{'declarations':["))?, List::cons(f1(f3(match v0.clone() {
        Adt0::C0(_, _, _, _, h1) => {
            h1
        }
    }, Fn0::F4)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'imports':["))?, List::cons(f1(f6(match v0.clone() {
        Adt0::C0(_, h2, _, _, _) => {
            h2
        }
    }, Fn1::F7)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'nodes':["))?, List::cons(f1(f8(match v0.clone() {
        Adt0::C0(_, _, h3, _, _) => {
            h3
        }
    }, Fn2::F9)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'proof_nodes':["))?, List::cons(f1(f10(match v0.clone() {
        Adt0::C0(_, _, _, h4, _) => {
            h4
        }
    }, Fn3::F11)?, f5(Str::lit(","))), List::cons(f2(Str::lit("],'spec':"))?, List::cons(f12(match v0 {
        Adt0::C0(h5, _, _, _, _) => {
            h5
        }
    })?, List::cons(f5(Str::lit("}")), List::<Bytes>::nil()))))))))))), f5(Str::lit(""))))
}

pub fn f1(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let (_, h6) = f13(Fn4::F14(v1), (true, f5(Str::lit(""))), v0);
    h6
}

pub fn f2(v0: Str) -> R<Bytes> {
    let v1: Bytes = f5(v0);
    let (_, h7) = {
        let (h8, _) = f15(Fn5::F16(v1.clone()), {
            let a10 = {
                let a9 = v1;
                length_bytes(a9)
            };
            let a11 = 1u64;
            nat_add(a10, a11)?
        }, (0u64, Bytes::lit(&[])))?;
        h8
    };
    Ok(h7)
}

pub fn f3(v0: List<Adt6>, v1: Fn0) -> R<List<Bytes>> {
    f17(f18(Fn6::F19(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f4(v0: Adt6) -> R<Bytes> {
    f20(v0)
}

pub fn f5(v0: Str) -> Bytes {
    let a12 = v0;
    utf8_encode(a12)
}

pub fn f6(v0: List<Bytes>, v1: Fn1) -> R<List<Bytes>> {
    f17(f21(Fn7::F22(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    f12(v0)
}

pub fn f8(v0: List<Adt1>, v1: Fn2) -> R<List<Bytes>> {
    f17(f23(Fn8::F24(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f9(v0: Adt1) -> R<Bytes> {
    f25(v0)
}

pub fn f10(v0: List<u64>, v1: Fn3) -> R<List<Bytes>> {
    f17(f26(Fn9::F27(v1), List::<Bytes>::nil(), v0)?)
}

pub fn f11(v0: u64) -> R<Bytes> {
    f28(v0)
}

pub fn f12(v0: Bytes) -> R<Bytes> {
    f29(v0)
}

pub fn f13(v0: Fn4, p16: (bool, Bytes), p17: List<Bytes>) -> (bool, Bytes) {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m13 = v2;
                match m13.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(({
                            let c14: Fn4 = v0.clone();
                            c14.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: (bool, Bytes), v2: Bytes) -> (bool, Bytes) {
    (false, {
        let (h18, _) = v1;
        if h18 {
            v2
        } else {
            f30(f30({
                let (_, h19) = v1;
                h19
            }, v0), v2)
        }
    })
}

pub fn f15(v0: Fn5, p24: u64, p25: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn5 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
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
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f31(v0, v1)
}

pub fn f17(v0: List<Bytes>) -> R<List<Bytes>> {
    f21(Fn7::F32, List::<Bytes>::nil(), v0)
}

pub fn f18(v0: Fn6, p29: List<Bytes>, p30: List<Adt6>) -> R<List<Bytes>> {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, v2) = m28;
                let m26 = v2;
                match m26.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(({
                            let c27: Fn6 = v0.clone();
                            c27.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m28 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Fn0, v1: List<Bytes>, v2: Adt6) -> R<List<Bytes>> {
    Ok(List::cons({
        let c31: Fn0 = v0;
        c31.apply(v2)?
    }, v1))
}

pub fn f20(v0: Adt6) -> R<Bytes> {
    f33(v0)
}

pub fn f21(v0: Fn7, p35: List<Bytes>, p36: List<Bytes>) -> R<List<Bytes>> {
    let m34 = (p35, p36);
    {
        let mut m34 = m34;
        loop {
            let step = {
                let (v1, v2) = m34;
                let m32 = v2;
                match m32.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c33: Fn7 = v0.clone();
                            c33.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m34 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Fn1, v1: List<Bytes>, v2: Bytes) -> R<List<Bytes>> {
    Ok(List::cons({
        let c37: Fn1 = v0;
        c37.apply(v2)?
    }, v1))
}

pub fn f23(v0: Fn8, p41: List<Bytes>, p42: List<Adt1>) -> R<List<Bytes>> {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v1, v2) = m40;
                let m38 = v2;
                match m38.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt1>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt1>), R<List<Bytes>>>(({
                            let c39: Fn8 = v0.clone();
                            c39.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Fn2, v1: List<Bytes>, v2: Adt1) -> R<List<Bytes>> {
    Ok(List::cons({
        let c43: Fn2 = v0;
        c43.apply(v2)?
    }, v1))
}

pub fn f25(v0: Adt1) -> R<Bytes> {
    let m44 = v0;
    match m44 {
        Adt1::C0(v1) => {
            Ok(f30(f30(f2(Str::lit("{'i':"))?, f28(v1)?), f2(Str::lit(",'k':'b'}"))?))
        }
        Adt1::C1(r45) => {
            let v2 = (*r45).clone();
            Ok(f30(f30(f2(Str::lit("{'k':'s','l':"))?, f34(v2)?), f5(Str::lit("}"))))
        }
        Adt1::C2(v3, v4) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'k':'c','n':"))?, f12(v3)?), f2(Str::lit(",'u':["))?), f35(v4)?), f5(Str::lit("]}"))))
        }
        Adt1::C3(v5, v6) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'f':"))?, f28(v5)?), f2(Str::lit(",'k':'a','x':"))?), f28(v6)?), f5(Str::lit("}"))))
        }
        Adt1::C4(v7, v8, v9, v10) => {
            f36(f5(Str::lit("l")), v7, v8, v9, v10)
        }
        Adt1::C5(v11, v12, v13, v14) => {
            f36(f5(Str::lit("p")), v11, v12, v13, v14)
        }
        Adt1::C6(v15, v16, v17, v18, v19) => {
            Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'body':"))?, f28(v18)?), f2(Str::lit(",'d':"))?), if v19 {
                f5(Str::lit("true"))
            } else {
                f5(Str::lit("false"))
            }), f2(Str::lit(",'k':'e','n':"))?), f12(v15)?), f30(f2(Str::lit(",'t':"))?, f28(v16)?)), f30(f2(Str::lit(",'v':"))?, f28(v17)?)), f5(Str::lit("}"))))
        }
        Adt1::C7(v20) => {
            Ok(f30(f30(f2(Str::lit("{'k':'n','v':"))?, f12(v20)?), f5(Str::lit("}"))))
        }
        Adt1::C8(v21) => {
            Ok(f30(f30(f2(Str::lit("{'k':'t','v':["))?, f1(f17(f26(Fn9::F37, List::<Bytes>::nil(), v21)?)?, f5(Str::lit(",")))), f5(Str::lit("]}"))))
        }
        Adt1::C9(v22, v23) => {
            Ok(f30(f30(f30(f30(f2(Str::lit("{'k':'d','m':["))?, f1(f17(f38(Fn10::F39, List::<Bytes>::nil(), v22)?)?, f5(Str::lit(",")))), f2(Str::lit("],'x':"))?), f28(v23)?), f5(Str::lit("}"))))
        }
        Adt1::C10(v24, v25, v26, v27) => {
            Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'i':"))?, f28(v26)?), f2(Str::lit(",'k':'j','n':"))?), f12(v24)?), f2(Str::lit(",'s':"))?), f12(v25)?), f2(Str::lit(",'x':"))?), f28(v27)?), f5(Str::lit("}"))))
        }
    }
}

pub fn f26(v0: Fn9, p49: List<Bytes>, p50: List<u64>) -> R<List<Bytes>> {
    let m48 = (p49, p50);
    {
        let mut m48 = m48;
        loop {
            let step = {
                let (v1, v2) = m48;
                let m46 = v2;
                match m46.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<u64>), R<List<Bytes>>>(({
                            let c47: Fn9 = v0.clone();
                            c47.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m48 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Fn3, v1: List<Bytes>, v2: u64) -> R<List<Bytes>> {
    Ok(List::cons({
        let c51: Fn3 = v0;
        c51.apply(v2)?
    }, v1))
}

pub fn f28(v0: u64) -> R<Bytes> {
    f40(v0)
}

pub fn f29(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a52 = v0.clone();
        length_bytes(a52)
    };
    let v2: Adt7 = {
        let (h53, _) = f41(Fn11::F42(v1, v0), {
            let a54 = v1;
            let a55 = 1u64;
            nat_add(a54, a55)?
        }, Adt7::C0(0u64, List::<Bytes>::nil()))?;
        h53
    };
    Ok(f43(f43(f44(Str::lit("\"")), f45(match v2 {
        Adt7::C0(_, h56) => {
            h56
        }
    })?), f44(Str::lit("\""))))
}

pub fn f30(v0: Bytes, v1: Bytes) -> Bytes {
    let a57 = v0;
    let a58 = v1;
    append_bytes(a57, a58)
}

pub fn f31(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m60 = f46(v0.clone(), {
        let (h59, _) = v1.clone();
        h59
    });
    match m60 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a62 = {
                    let (h61, _) = v1.clone();
                    h61
                };
                let a63 = 1u64;
                nat_add(a62, a63)?
            }, f30({
                let (_, h64) = v1;
                h64
            }, {
                let a111 = {
                    let v3: u8 = v2;
                    let a109 = {
                        let a85 = {
                            let a73 = {
                                let a67 = {
                                    let a65 = v3;
                                    let a66 = 128u8;
                                    fixed_u8::bit_and(a65, a66)
                                };
                                let a68 = 128u8;
                                if equal(a67, a68) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a74 = {
                                let a71 = {
                                    let a69 = v3;
                                    let a70 = 64u8;
                                    fixed_u8::bit_and(a69, a70)
                                };
                                let a72 = 64u8;
                                if equal(a71, a72) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a73, a74)?
                        };
                        let a86 = {
                            let a83 = {
                                let a77 = {
                                    let a75 = v3;
                                    let a76 = 32u8;
                                    fixed_u8::bit_and(a75, a76)
                                };
                                let a78 = 32u8;
                                if equal(a77, a78) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a84 = {
                                let a81 = {
                                    let a79 = v3;
                                    let a80 = 16u8;
                                    fixed_u8::bit_and(a79, a80)
                                };
                                let a82 = 16u8;
                                if equal(a81, a82) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a83, a84)?
                        };
                        nat_add(a85, a86)?
                    };
                    let a110 = {
                        let a107 = {
                            let a95 = {
                                let a89 = {
                                    let a87 = v3;
                                    let a88 = 8u8;
                                    fixed_u8::bit_and(a87, a88)
                                };
                                let a90 = 8u8;
                                if equal(a89, a90) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a96 = {
                                let a93 = {
                                    let a91 = v3;
                                    let a92 = 4u8;
                                    fixed_u8::bit_and(a91, a92)
                                };
                                let a94 = 4u8;
                                if equal(a93, a94) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a95, a96)?
                        };
                        let a108 = {
                            let a105 = {
                                let a99 = {
                                    let a97 = v3;
                                    let a98 = 2u8;
                                    fixed_u8::bit_and(a97, a98)
                                };
                                let a100 = 2u8;
                                if equal(a99, a100) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a106 = {
                                let a103 = {
                                    let a101 = v3;
                                    let a102 = 1u8;
                                    fixed_u8::bit_and(a101, a102)
                                };
                                let a104 = 1u8;
                                if equal(a103, a104) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a105, a106)?
                        };
                        nat_add(a107, a108)?
                    };
                    nat_add(a109, a110)?
                };
                let a112 = 39u64;
                if nat_eq(a111, a112) {
                    Bytes::lit(&[0x22])
                } else {
                    f47(v0, {
                        let (h113, _) = v1;
                        h113
                    }, 1u64)
                }
            }))))
        }
    }
}

pub fn f32(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f33(v0: Adt6) -> R<Bytes> {
    let m114 = v0;
    match m114 {
        Adt6::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt6::C1(v2, v3) => {
            f48(v2, v3)
        }
        Adt6::C2(v4) => {
            f49(v4)
        }
        Adt6::C3(r115) => {
            let v5 = (*r115).clone();
            Ok(f50(f51(v5)?))
        }
        Adt6::C4(r116) => {
            let v6 = (*r116).clone();
            f52(f53(v6)?)
        }
        Adt6::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt6::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f34(v0: Adt2) -> R<Bytes> {
    let m117 = v0;
    match m117 {
        Adt2::C0 => {
            f2(Str::lit("{'k':'z'}"))
        }
        Adt2::C1(r118) => {
            let v1 = (*r118).clone();
            Ok(f30(f30(f2(Str::lit("{'a':"))?, f34(v1)?), f2(Str::lit(",'k':'s'}"))?))
        }
        Adt2::C2(r119, r120) => {
            let v2 = (*r119).clone();
            let v3 = (*r120).clone();
            Ok(f30(f30(f30(f30(f2(Str::lit("{'a':["))?, f34(v2)?), f5(Str::lit(","))), f34(v3)?), f2(Str::lit("],'k':'m'}"))?))
        }
        Adt2::C3(r121, r122) => {
            let v4 = (*r121).clone();
            let v5 = (*r122).clone();
            Ok(f30(f30(f30(f30(f2(Str::lit("{'a':["))?, f34(v4)?), f5(Str::lit(","))), f34(v5)?), f2(Str::lit("],'k':'i'}"))?))
        }
        Adt2::C4(v6) => {
            Ok(f30(f30(f2(Str::lit("{'a':"))?, f12(v6)?), f2(Str::lit(",'k':'p'}"))?))
        }
    }
}

pub fn f35(v0: List<Adt2>) -> R<Bytes> {
    Ok(f1(f17(f54(Fn12::F55, List::<Bytes>::nil(), v0)?)?, f5(Str::lit(","))))
}

pub fn f36(v0: Bytes, v1: Bytes, v2: Adt3, v3: u64, v4: u64) -> R<Bytes> {
    Ok(f30(f30(f30(f30(f30(f30(f30(f30(f2(Str::lit("{'b':'"))?, f56(v2)), f2(Str::lit("','k':'"))?), v0), f2(Str::lit("','n':"))?), f12(v1)?), f30(f2(Str::lit(",'t':"))?, f28(v3)?)), f30(f2(Str::lit(",'v':"))?, f28(v4)?)), f5(Str::lit("}"))))
}

pub fn f37(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    Ok(List::cons(f28(v1)?, v0))
}

pub fn f38(v0: Fn10, p126: List<Bytes>, p127: List<Adt4>) -> R<List<Bytes>> {
    let m125 = (p126, p127);
    {
        let mut m125 = m125;
        loop {
            let step = {
                let (v1, v2) = m125;
                let m123 = v2;
                match m123.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(({
                            let c124: Fn10 = v0.clone();
                            c124.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: Adt4) -> R<List<Bytes>> {
    Ok(List::cons(f57(v1)?, v0))
}

pub fn f40(v0: u64) -> R<Bytes> {
    f58(f59(), 10u64, v0)
}

pub fn f41(v0: Fn11, p132: u64, p133: Adt7) -> R<(Adt7, bool)> {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v1, v2) = m131;
                let m128 = v1;
                if m128 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m128 - 1;
                    let m130 = {
                        let c129: Fn11 = v0.clone();
                        c129.apply(v2.clone())?
                    };
                    match m130 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f60(v1, v0, v2)
}

pub fn f43(v0: Bytes, v1: Bytes) -> Bytes {
    let a134 = v0;
    let a135 = v1;
    append_bytes(a134, a135)
}

pub fn f44(v0: Str) -> Bytes {
    let a136 = v0;
    utf8_encode(a136)
}

pub fn f45(v0: List<Bytes>) -> R<Bytes> {
    f61(f62(v0)?)
}

pub fn f46(v0: Bytes, v1: u64) -> Option<u8> {
    let a137 = v0;
    let a138 = v1;
    index_bytes(a137, a138)
}

pub fn f47(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m142 = {
        let a139 = v0;
        let a140 = v1;
        let a141 = v2;
        slice_bytes(a139, a140, a141)
    };
    match m142 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f48(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a143 = 0u64;
        let a144 = v1;
        nat_lt(a143, a144)
    } else {
        false
    } {
        let a145 = Bytes::lit(&[0x2d]);
        let a146 = f40(v1)?;
        Ok(append_bytes(a145, a146))
    } else {
        f40(v1)
    }
}

pub fn f49(v0: Bytes) -> R<Bytes> {
    let a152 = {
        let a150 = Bytes::lit(&[0x22]);
        let a151 = {
            let (_, h147) = {
                let (h148, _) = f15(Fn5::F63(v0.clone()), {
                    let a149 = v0;
                    length_bytes(a149)
                }, (0u64, Bytes::lit(&[])))?;
                h148
            };
            h147
        };
        append_bytes(a150, a151)
    };
    let a153 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a152, a153))
}

pub fn f50(v0: List<Bytes>) -> Bytes {
    let a156 = {
        let a154 = Bytes::lit(&[0x5b]);
        let a155 = f64(v0, Bytes::lit(&[0x2c]));
        append_bytes(a154, a155)
    };
    let a157 = Bytes::lit(&[0x5d]);
    append_bytes(a156, a157)
}

pub fn f51(v0: Adt6) -> R<List<Bytes>> {
    let m158 = v0;
    match m158 {
        Adt6::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt6::C6(r159, r160) => {
            let v7 = (*r159).clone();
            let v8 = (*r160).clone();
            Ok(List::cons(f33(v7)?, f51(v8)?))
        }
        Adt6::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f52(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f65(Fn13::F66, List::<(Bytes, Bytes)>::nil(), f67(v0)?);
    let v2: List<Bytes> = f68(Fn14::F69, List::<Bytes>::nil(), v1)?;
    let a163 = {
        let a161 = Bytes::lit(&[0x7b]);
        let a162 = f64(v2, Bytes::lit(&[0x2c]));
        append_bytes(a161, a162)
    };
    let a164 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a163, a164))
}

pub fn f53(v0: Adt6) -> R<List<(Bytes, Bytes)>> {
    let m165 = v0;
    match m165 {
        Adt6::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt6::C7(v9, r166, r167) => {
            let v10 = (*r166).clone();
            let v11 = (*r167).clone();
            Ok(List::cons((v9, f33(v10)?), f53(v11)?))
        }
    }
}

pub fn f54(v0: Fn12, p171: List<Bytes>, p172: List<Adt2>) -> R<List<Bytes>> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m168 = v2;
                match m168.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(({
                            let c169: Fn12 = v0.clone();
                            c169.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Adt2) -> R<List<Bytes>> {
    Ok(List::cons(f34(v1)?, v0))
}

pub fn f56(v0: Adt3) -> Bytes {
    let m173 = v0;
    match m173 {
        Adt3::C0 => {
            f5(Str::lit("e"))
        }
        Adt3::C1 => {
            f5(Str::lit("i"))
        }
        Adt3::C2 => {
            f5(Str::lit("s"))
        }
        Adt3::C3 => {
            f5(Str::lit("c"))
        }
    }
}

pub fn f57(v0: Adt4) -> R<Bytes> {
    Ok(f30(f30(f30(f30(f2(Str::lit("{'n':"))?, f12(match v0.clone() {
        Adt4::C0(h174, _) => {
            h174
        }
    })?), f2(Str::lit(",'v':"))?), {
        let m176 = match v0 {
            Adt4::C0(_, h175) => {
                (*h175).clone()
            }
        };
        match m176 {
            Adt5::C0(v1) => {
                f30(f30(f2(Str::lit("{'t':'b','v':"))?, if v1 {
                    f5(Str::lit("true"))
                } else {
                    f5(Str::lit("false"))
                }), f5(Str::lit("}")))
            }
            Adt5::C1(v2) => {
                f30(f30(f2(Str::lit("{'t':'n','v':"))?, f12(v2)?), f5(Str::lit("}")))
            }
            Adt5::C2(v3) => {
                f30(f30(f2(Str::lit("{'t':'i','v':"))?, f12(v3)?), f5(Str::lit("}")))
            }
            Adt5::C3(v4) => {
                f30(f30(f2(Str::lit("{'t':'s','v':"))?, f12(v4)?), f5(Str::lit("}")))
            }
            Adt5::C4(v5) => {
                f30(f30(f2(Str::lit("{'t':'m','v':"))?, f12(v5)?), f5(Str::lit("}")))
            }
        }
    }), f5(Str::lit("}"))))
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a177 = v2;
    let a178 = 0u64;
    if nat_eq(a177, a178) {
        Ok(f70(v0, 0u64))
    } else {
        let (_, h179) = {
            let (h180, _) = f15(Fn5::F71(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h180
        };
        Ok(h179)
    }
}

pub fn f59() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f60(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h181, _) => {
            h181
        }
    };
    let a182 = v3;
    let a183 = v1;
    if nat_lt(a182, a183) {
        let v4: u64 = f72(v0.clone(), v3)?;
        let a184 = v4;
        let a185 = 32u64;
        if if nat_lt(a184, a185) {
            true
        } else {
            let a186 = v4;
            let a187 = 34u64;
            if nat_eq(a186, a187) {
                true
            } else {
                let a188 = v4;
                let a189 = 92u64;
                nat_eq(a188, a189)
            }
        } {
            Ok(Some(Adt7::C0({
                let a190 = v3;
                let a191 = 1u64;
                nat_add(a190, a191)?
            }, List::cons(f73(v4), match v2 {
                Adt7::C0(_, h192) => {
                    h192
                }
            }))))
        } else {
            let v5: u64 = f74(v0.clone(), v1, v3)?;
            Ok(Some(Adt7::C0(v5, List::cons(f47(v0, v3, {
                let a193 = v5;
                let a194 = v3;
                nat_sub(a193, a194)
            }), match v2 {
                Adt7::C0(_, h195) => {
                    h195
                }
            }))))
        }
    } else {
        Ok(None::<Adt7>)
    }
}

pub fn f61(v0: List<Bytes>) -> R<Bytes> {
    let m198 = {
        let (h196, _) = f75(Fn15::F76, {
            let a197 = v0.clone();
            length_list(a197)
        }, v0)?;
        h196
    };
    match m198.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, _)) => {
            Ok(v1)
        }
    }
}

pub fn f62(v0: List<Bytes>) -> R<List<Bytes>> {
    f21(Fn7::F77, List::<Bytes>::nil(), v0)
}

pub fn f63(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f78(v0, v1)
}

pub fn f64(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m199 = f79(Fn16::F80(v1), None::<Bytes>, v0);
    match m199 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f65(v0: Fn13, p203: List<(Bytes, Bytes)>, p204: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m202 = (p203, p204);
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, v2) = m202;
                let m200 = v2;
                match m200.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c201: Fn13 = v0.clone();
                            c201.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f81(v0, v1)
}

pub fn f67(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f82(v0)
}

pub fn f68(v0: Fn14, p208: List<Bytes>, p209: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m205 = v2;
                match m205.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c206: Fn14 = v0.clone();
                            c206.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f83(v1)?, v0))
}

pub fn f70(v0: Bytes, v1: u64) -> Bytes {
    let m213 = {
        let a210 = v0;
        let a211 = v1;
        let a212 = 1u64;
        slice_bytes(a210, a211, a212)
    };
    match m213 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f71(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f84(v1, v0, v2)
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let m214 = f46(v0, v1);
    match m214 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a259 = {
                let a235 = {
                    let a223 = {
                        let a217 = {
                            let a215 = v3;
                            let a216 = 128u8;
                            fixed_u8::bit_and(a215, a216)
                        };
                        let a218 = 128u8;
                        if equal(a217, a218) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a224 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 64u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 64u8;
                        if equal(a221, a222) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a223, a224)?
                };
                let a236 = {
                    let a233 = {
                        let a227 = {
                            let a225 = v3;
                            let a226 = 32u8;
                            fixed_u8::bit_and(a225, a226)
                        };
                        let a228 = 32u8;
                        if equal(a227, a228) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a234 = {
                        let a231 = {
                            let a229 = v3;
                            let a230 = 16u8;
                            fixed_u8::bit_and(a229, a230)
                        };
                        let a232 = 16u8;
                        if equal(a231, a232) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a233, a234)?
                };
                nat_add(a235, a236)?
            };
            let a260 = {
                let a257 = {
                    let a245 = {
                        let a239 = {
                            let a237 = v3;
                            let a238 = 8u8;
                            fixed_u8::bit_and(a237, a238)
                        };
                        let a240 = 8u8;
                        if equal(a239, a240) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a246 = {
                        let a243 = {
                            let a241 = v3;
                            let a242 = 4u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 4u8;
                        if equal(a243, a244) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a245, a246)?
                };
                let a258 = {
                    let a255 = {
                        let a249 = {
                            let a247 = v3;
                            let a248 = 2u8;
                            fixed_u8::bit_and(a247, a248)
                        };
                        let a250 = 2u8;
                        if equal(a249, a250) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a256 = {
                        let a253 = {
                            let a251 = v3;
                            let a252 = 1u8;
                            fixed_u8::bit_and(a251, a252)
                        };
                        let a254 = 1u8;
                        if equal(a253, a254) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a255, a256)?
                };
                nat_add(a257, a258)?
            };
            nat_add(a259, a260)
        }
    }
}

pub fn f73(v0: u64) -> Bytes {
    let a261 = v0;
    let a262 = 34u64;
    if nat_eq(a261, a262) {
        f44(Str::lit("\\\""))
    } else {
        let a263 = v0;
        let a264 = 92u64;
        if nat_eq(a263, a264) {
            f44(Str::lit("\\\\"))
        } else {
            let a265 = v0;
            let a266 = 8u64;
            if nat_eq(a265, a266) {
                f44(Str::lit("\\b"))
            } else {
                let a267 = v0;
                let a268 = 12u64;
                if nat_eq(a267, a268) {
                    f44(Str::lit("\\f"))
                } else {
                    let a269 = v0;
                    let a270 = 10u64;
                    if nat_eq(a269, a270) {
                        f44(Str::lit("\\n"))
                    } else {
                        let a271 = v0;
                        let a272 = 13u64;
                        if nat_eq(a271, a272) {
                            f44(Str::lit("\\r"))
                        } else {
                            let a273 = v0;
                            let a274 = 9u64;
                            if nat_eq(a273, a274) {
                                f44(Str::lit("\\t"))
                            } else {
                                f43(f44(Str::lit("\\u00")), f85(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h275, _) = f86(Fn17::F87(v1, v0), {
        let a276 = v1;
        let a277 = v2;
        nat_sub(a276, a277)
    }, v2)?;
    Ok(h275)
}

pub fn f75(v0: Fn15, p282: u64, p283: List<Bytes>) -> R<(List<Bytes>, bool)> {
    let m281 = (p282, p283);
    {
        let mut m281 = m281;
        loop {
            let step = {
                let (v1, v2) = m281;
                let m278 = v1;
                if m278 == 0 {
                    Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m278 - 1;
                    let m280 = {
                        let c279: Fn15 = v0.clone();
                        c279.apply(v2.clone())?
                    };
                    match m280 {
                        None => {
                            Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    f88(v0)
}

pub fn f77(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f78(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m287 = {
        let a285 = v0.clone();
        let a286 = {
            let (h284, _) = v1.clone();
            h284
        };
        index_bytes(a285, a286)
    };
    match m287 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a289 = {
                    let (h288, _) = v1.clone();
                    h288
                };
                let a290 = 1u64;
                nat_add(a289, a290)?
            }, {
                let a293 = {
                    let (_, h291) = v1;
                    h291
                };
                let a294 = f89(v0, {
                    let (h292, _) = v1;
                    h292
                }, v2)?;
                append_bytes(a293, a294)
            })))
        }
    }
}

pub fn f79(v0: Fn16, p298: Option<Bytes>, p299: List<Bytes>) -> Option<Bytes> {
    let m297 = (p298, p299);
    {
        let mut m297 = m297;
        loop {
            let step = {
                let (v1, v2) = m297;
                let m295 = v2;
                match m295.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c296: Fn16 = v0.clone();
                            c296.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m300 = v1;
    match m300 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a303 = {
                    let a301 = v3;
                    let a302 = v0;
                    append_bytes(a301, a302)
                };
                let a304 = v2;
                append_bytes(a303, a304)
            })
        }
    }
}

pub fn f81(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m305 = v0.clone();
    match m305.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a308 = {
                let (h306, _) = v2;
                h306
            };
            let a309 = {
                let (h307, _) = v1.clone();
                h307
            };
            if equal(a308, a309) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f82(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f90(f91(Fn18::F92, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m314 = {
        let (h310, _) = f93(Fn19::F94, {
            let a312 = {
                let a311 = v0;
                length_list(a311)
            };
            let a313 = 1u64;
            nat_add(a312, a313)?
        }, v1)?;
        h310
    };
    match m314.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f83(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a319 = {
        let a316 = f49({
            let (h315, _) = v0;
            h315
        })?;
        let a317 = Bytes::lit(&[0x3a]);
        append_bytes(a316, a317)
    };
    let a320 = {
        let (_, h318) = v0;
        h318
    };
    Ok(append_bytes(a319, a320))
}

pub fn f84(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h321, _) = v2;
        h321
    };
    let a322 = v3;
    let a323 = 0u64;
    if nat_eq(a322, a323) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a324 = v3;
            let a325 = v1;
            let a326 = 0u64;
            nat_quot(a324, a325, a326)
        }, {
            let a331 = f70(v0, {
                let a327 = v3;
                let a328 = v1;
                let a329 = 0u64;
                nat_rem(a327, a328, a329)
            });
            let a332 = {
                let (_, h330) = v2;
                h330
            };
            append_bytes(a331, a332)
        }))
    }
}

pub fn f85(v0: u64) -> Bytes {
    f43(f95(v0, 16u64), f96(v0))
}

pub fn f86(v0: Fn17, p337: u64, p338: u64) -> R<(u64, bool)> {
    let m336 = (p337, p338);
    {
        let mut m336 = m336;
        loop {
            let step = {
                let (v1, v2) = m336;
                let m333 = v1;
                if m333 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m333 - 1;
                    let m335 = {
                        let c334: Fn17 = v0.clone();
                        c334.apply(v2)?
                    };
                    match m335 {
                        None => {
                            Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), R<(u64, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f97(v1, v0, v2)
}

pub fn f88(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m339 = v0.clone();
    match m339.uncons() {
        None => {
            Ok(None::<List<Bytes>>)
        }
        Some((_, v2)) => {
            let m340 = v2;
            match m340.uncons() {
                None => {
                    Ok(None::<List<Bytes>>)
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f98(Fn20::F99, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m342 = {
                        let (h341, _) = v5;
                        h341
                    };
                    match m342 {
                        None => {
                            Ok(Some(f62({
                                let (_, h344) = v5;
                                h344
                            })?))
                        }
                        Some(v6) => {
                            Ok(Some(f62(List::cons(v6, {
                                let (_, h343) = v5;
                                h343
                            }))?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a389 = {
            let a365 = {
                let a353 = {
                    let a347 = {
                        let a345 = v3;
                        let a346 = 128u8;
                        fixed_u8::bit_and(a345, a346)
                    };
                    let a348 = 128u8;
                    if equal(a347, a348) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a354 = {
                    let a351 = {
                        let a349 = v3;
                        let a350 = 64u8;
                        fixed_u8::bit_and(a349, a350)
                    };
                    let a352 = 64u8;
                    if equal(a351, a352) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a353, a354)?
            };
            let a366 = {
                let a363 = {
                    let a357 = {
                        let a355 = v3;
                        let a356 = 32u8;
                        fixed_u8::bit_and(a355, a356)
                    };
                    let a358 = 32u8;
                    if equal(a357, a358) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a364 = {
                    let a361 = {
                        let a359 = v3;
                        let a360 = 16u8;
                        fixed_u8::bit_and(a359, a360)
                    };
                    let a362 = 16u8;
                    if equal(a361, a362) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a363, a364)?
            };
            nat_add(a365, a366)?
        };
        let a390 = {
            let a387 = {
                let a375 = {
                    let a369 = {
                        let a367 = v3;
                        let a368 = 8u8;
                        fixed_u8::bit_and(a367, a368)
                    };
                    let a370 = 8u8;
                    if equal(a369, a370) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a376 = {
                    let a373 = {
                        let a371 = v3;
                        let a372 = 4u8;
                        fixed_u8::bit_and(a371, a372)
                    };
                    let a374 = 4u8;
                    if equal(a373, a374) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a375, a376)?
            };
            let a388 = {
                let a385 = {
                    let a379 = {
                        let a377 = v3;
                        let a378 = 2u8;
                        fixed_u8::bit_and(a377, a378)
                    };
                    let a380 = 2u8;
                    if equal(a379, a380) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a386 = {
                    let a383 = {
                        let a381 = v3;
                        let a382 = 1u8;
                        fixed_u8::bit_and(a381, a382)
                    };
                    let a384 = 1u8;
                    if equal(a383, a384) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a385, a386)?
            };
            nat_add(a387, a388)?
        };
        nat_add(a389, a390)?
    };
    let a391 = v4;
    let a392 = 34u64;
    if nat_eq(a391, a392) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a393 = v4;
        let a394 = 92u64;
        if nat_eq(a393, a394) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a395 = v4;
            let a396 = 8u64;
            if nat_eq(a395, a396) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a397 = v4;
                let a398 = 12u64;
                if nat_eq(a397, a398) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a399 = v4;
                    let a400 = 10u64;
                    if nat_eq(a399, a400) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a401 = v4;
                        let a402 = 13u64;
                        if nat_eq(a401, a402) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a403 = v4;
                            let a404 = 9u64;
                            if nat_eq(a403, a404) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a405 = v4;
                                let a406 = 32u64;
                                if nat_lt(a405, a406) {
                                    let a407 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a408 = f100(v2)?;
                                    Ok(append_bytes(a407, a408))
                                } else {
                                    Ok(f47(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f90(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f101(Fn21::F102, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f91(v0: Fn18, p412: List<List<(Bytes, Bytes)>>, p413: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, v2) = m411;
                let m409 = v2;
                match m409.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c410: Fn18 = v0.clone();
                            c410.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f93(v0: Fn19, p418: u64, p419: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m417 = (p418, p419);
    {
        let mut m417 = m417;
        loop {
            let step = {
                let (v1, v2) = m417;
                let m414 = v1;
                if m414 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m414 - 1;
                    let m416 = {
                        let c415: Fn19 = v0.clone();
                        c415.apply(v2.clone())?
                    };
                    match m416 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f103(v0)
}

pub fn f95(v0: u64, v1: u64) -> Bytes {
    f96({
        let a420 = v0;
        let a421 = v1;
        let a422 = 0u64;
        nat_quot(a420, a421, a422)
    })
}

pub fn f96(v0: u64) -> Bytes {
    f47(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a423 = v0;
        let a424 = 16u64;
        let a425 = 0u64;
        nat_rem(a423, a424, a425)
    }, 1u64)
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a426 = v2;
    let a427 = v1;
    if nat_lt(a426, a427) {
        let v3: u64 = f72(v0, v2)?;
        let a428 = v3;
        let a429 = 32u64;
        if nat_lt(a428, a429) {
            Ok(None::<u64>)
        } else {
            let a430 = v3;
            let a431 = 34u64;
            if nat_eq(a430, a431) {
                Ok(None::<u64>)
            } else {
                let a432 = v3;
                let a433 = 92u64;
                if nat_eq(a432, a433) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a434 = v2;
                        let a435 = 1u64;
                        nat_add(a434, a435)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f98(v0: Fn20, p439: (Option<Bytes>, List<Bytes>), p440: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m438 = (p439, p440);
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v1, v2) = m438;
                let m436 = v2;
                match m436.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c437: Fn20 = v0.clone();
                            c437.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f104(v0, v1)
}

pub fn f100(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a485 = {
            let a461 = {
                let a449 = {
                    let a443 = {
                        let a441 = v1;
                        let a442 = 128u8;
                        fixed_u8::bit_and(a441, a442)
                    };
                    let a444 = 128u8;
                    if equal(a443, a444) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a450 = {
                    let a447 = {
                        let a445 = v1;
                        let a446 = 64u8;
                        fixed_u8::bit_and(a445, a446)
                    };
                    let a448 = 64u8;
                    if equal(a447, a448) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a449, a450)?
            };
            let a462 = {
                let a459 = {
                    let a453 = {
                        let a451 = v1;
                        let a452 = 32u8;
                        fixed_u8::bit_and(a451, a452)
                    };
                    let a454 = 32u8;
                    if equal(a453, a454) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a460 = {
                    let a457 = {
                        let a455 = v1;
                        let a456 = 16u8;
                        fixed_u8::bit_and(a455, a456)
                    };
                    let a458 = 16u8;
                    if equal(a457, a458) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a459, a460)?
            };
            nat_add(a461, a462)?
        };
        let a486 = {
            let a483 = {
                let a471 = {
                    let a465 = {
                        let a463 = v1;
                        let a464 = 8u8;
                        fixed_u8::bit_and(a463, a464)
                    };
                    let a466 = 8u8;
                    if equal(a465, a466) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a472 = {
                    let a469 = {
                        let a467 = v1;
                        let a468 = 4u8;
                        fixed_u8::bit_and(a467, a468)
                    };
                    let a470 = 4u8;
                    if equal(a469, a470) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a471, a472)?
            };
            let a484 = {
                let a481 = {
                    let a475 = {
                        let a473 = v1;
                        let a474 = 2u8;
                        fixed_u8::bit_and(a473, a474)
                    };
                    let a476 = 2u8;
                    if equal(a475, a476) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a482 = {
                    let a479 = {
                        let a477 = v1;
                        let a478 = 1u8;
                        fixed_u8::bit_and(a477, a478)
                    };
                    let a480 = 1u8;
                    if equal(a479, a480) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a481, a482)?
            };
            nat_add(a483, a484)?
        };
        nat_add(a485, a486)?
    };
    let a493 = f70(f105(), {
        let a487 = v2;
        let a488 = 16u64;
        let a489 = 0u64;
        nat_quot(a487, a488, a489)
    });
    let a494 = f70(f105(), {
        let a490 = v2;
        let a491 = 16u64;
        let a492 = 0u64;
        nat_rem(a490, a491, a492)
    });
    Ok(append_bytes(a493, a494))
}

pub fn f101(v0: Fn21, p498: List<List<(Bytes, Bytes)>>, p499: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m495 = v2;
                match m495.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c496: Fn21 = v0.clone();
                            c496.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f103(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a501 = {
        let a500 = v0.clone();
        length_list(a500)
    };
    let a502 = 1u64;
    if nat_le(a501, a502) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f106(v0)?))
    }
}

pub fn f104(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m504 = {
        let (h503, _) = v0;
        h503
    };
    match m504 {
        None => {
            (Some(v1), {
                let (_, h505) = v0;
                h505
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a506 = v2;
                let a507 = v1;
                append_bytes(a506, a507)
            }, {
                let (_, h508) = v0;
                h508
            }))
        }
    }
}

pub fn f105() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f106(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f90({
        let (_, h509) = {
            let (h510, _) = f107(Fn22::F108, {
                let a512 = {
                    let a511 = v0.clone();
                    length_list(a511)
                };
                let a513 = 1u64;
                nat_add(a512, a513)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h510
        };
        h509
    }))
}

pub fn f107(v0: Fn22, p518: u64, p519: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v1, v2) = m517;
                let m514 = v1;
                if m514 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m514 - 1;
                    let m516 = {
                        let c515: Fn22 = v0.clone();
                        c515.apply(v2.clone())?
                    };
                    match m516 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f109(v0)
}

pub fn f109(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h520, _) = v0;
        h520
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h521) = v0;
        h521
    };
    let m522 = v1;
    match m522.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m523 = v4;
            match m523.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f110(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f110(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a528 = {
            let a526 = {
                let a524 = v0.clone();
                length_list(a524)
            };
            let a527 = {
                let a525 = v1.clone();
                length_list(a525)
            };
            nat_add(a526, a527)?
        };
        let a529 = 1u64;
        nat_add(a528, a529)?
    };
    Ok(f111({
        let (_, h530) = {
            let (_, h531) = {
                let (h532, _) = f112(Fn23::F113, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h532
            };
            h531
        };
        h530
    }))
}

pub fn f111(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f65(Fn13::F114, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f112(v0: Fn23, p537: u64, p538: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v1, v2) = m536;
                let m533 = v1;
                if m533 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m533 - 1;
                    let m535 = {
                        let c534: Fn23 = v0.clone();
                        c534.apply(v2.clone())
                    };
                    match m535 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f115(v0)
}

pub fn f114(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f115(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h539, _) = v0.clone();
        h539
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h540, _) = {
            let (_, h541) = v0.clone();
            h541
        };
        h540
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h542) = {
            let (_, h543) = v0;
            h543
        };
        h542
    };
    let m544 = v1.clone();
    match m544.uncons() {
        None => {
            let m545 = v2;
            match m545.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m546 = v2.clone();
            match m546.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f116({
                        let (h547, _) = v8.clone();
                        h547
                    }, {
                        let (h548, _) = v6.clone();
                        h548
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

pub fn f116(v0: Bytes, v1: Bytes) -> bool {
    let a553 = {
        let a549 = v0;
        let a550 = v1;
        compare_bytes(a549, a550)
    };
    let a554 = {
        let a551 = Bytes::lit(&[]);
        let a552 = Bytes::lit(&[0x01]);
        compare_bytes(a551, a552)
    };
    equal(a553, a554)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
