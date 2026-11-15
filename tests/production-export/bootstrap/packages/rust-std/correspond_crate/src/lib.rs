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
    C0(bool, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(std::rc::Rc<Adt2>, std::rc::Rc<Adt8>, std::rc::Rc<Adt9>),
    C1(u64, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, bool, std::rc::Rc<Adt10>, std::rc::Rc<Adt9>),
    C2(std::rc::Rc<Adt12>, std::rc::Rc<Adt13>, std::rc::Rc<Adt2>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C3,
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Adt4),
    C2(Adt5, std::rc::Rc<Adt2>),
    C3(Adt6, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt7, u64),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(u64, std::rc::Rc<Adt2>, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(u64, List<Adt11>, bool, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1(std::rc::Rc<Adt14>, std::rc::Rc<Adt2>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt15),
    C1(std::rc::Rc<Adt12>),
    C2(std::rc::Rc<Adt14>),
    C3(Adt16, std::rc::Rc<Adt14>),
    C4(u64, u64, std::rc::Rc<Adt14>),
    C5,
    C6(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(std::rc::Rc<Adt18>, std::rc::Rc<Adt9>),
    C1(Adt19, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C2(Adt20, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C3(std::rc::Rc<Adt21>, std::rc::Rc<Adt17>, bool, std::rc::Rc<Adt9>),
    C4(std::rc::Rc<Adt12>, std::rc::Rc<Adt17>, bool, std::rc::Rc<Adt9>),
    C5(std::rc::Rc<Adt23>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, std::rc::Rc<Adt12>, std::rc::Rc<Adt9>),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(std::rc::Rc<Adt14>, bool, std::rc::Rc<Adt2>, std::rc::Rc<Adt17>, std::rc::Rc<Adt9>, std::rc::Rc<Adt17>),
    C15(std::rc::Rc<Adt14>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(std::rc::Rc<Adt12>, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt4),
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(std::rc::Rc<Adt22>),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(std::rc::Rc<Adt2>),
    C1,
    C2(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, Bytes, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<Adt27>, List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Bytes, Option<Adt4>, Option<Adt4>),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(List<Bytes>, List<(Bytes, List<Bytes>)>),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt27) -> Option<Bytes> {
        match self {
            Fn0::F5(k0, k1) => f5(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8(List<Bytes>, List<(Bytes, List<Bytes>)>),
    F79(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F8(k0, k1) => f8(k0.clone(), k1.clone(), p0, p1),
            Fn1::F79(k0) => f79(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt27>, p1: Adt27) -> List<Adt27> {
        match self {
            Fn2::F17 => f17(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F61(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<List<Bytes>>, p1: (Bytes, List<Bytes>)) -> Option<List<Bytes>> {
        match self {
            Fn3::F61(k0) => f61(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F63(Bytes),
    F65(List<Bytes>),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn4::F63(k0) => f63(k0.clone(), p0, p1),
            Fn4::F65(k0) => f65(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F68(std::rc::Rc<Adt27>),
}

impl Fn5 {
    pub fn apply(&self, p0: bool, p1: Adt27) -> bool {
        match self {
            Fn5::F68(k0) => f68((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F70(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn6::F70(k0) => f70(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F73(Bytes, u64, Bytes, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F73(k0, k1, k2, k3) => f73(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F76(u64),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn8::F76(k0) => f76(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F83,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F83 => f83(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0, v1: List<Bytes>) -> R<Result<(), Bytes>> {
    let v2: List<Adt25> = List::cons(Adt25::C0(0u64, f1(Str::lit("nat_add")), 0u64, true, false), List::cons(Adt25::C0(1u64, f1(Str::lit("nat_sub")), 0u64, false, false), List::cons(Adt25::C0(2u64, f1(Str::lit("nat_mul")), 0u64, true, false), List::cons(Adt25::C0(3u64, f1(Str::lit("nat_quot")), 0u64, false, false), List::cons(Adt25::C0(4u64, f1(Str::lit("nat_rem")), 0u64, false, false), List::cons(Adt25::C0(5u64, f1(Str::lit("nat_eq")), 0u64, false, false), List::cons(Adt25::C0(6u64, f1(Str::lit("nat_le")), 0u64, false, false), List::cons(Adt25::C0(7u64, f1(Str::lit("nat_lt")), 0u64, false, false), List::cons(Adt25::C0(8u64, f1(Str::lit("nat_succ")), 0u64, true, false), List::cons(Adt25::C0(9u64, f1(Str::lit("int_add")), 0u64, true, false), List::<Adt25>::nil()))))))))));
    let v3: List<Adt25> = List::cons(Adt25::C0(10u64, f1(Str::lit("int_sub")), 0u64, true, false), List::cons(Adt25::C0(11u64, f1(Str::lit("int_mul")), 0u64, true, false), List::cons(Adt25::C0(12u64, f1(Str::lit("int_neg")), 0u64, true, false), List::cons(Adt25::C0(13u64, f1(Str::lit("int_quot")), 0u64, true, false), List::cons(Adt25::C0(14u64, f1(Str::lit("int_rem")), 0u64, false, false), List::cons(Adt25::C0(15u64, f1(Str::lit("bool_not")), 0u64, false, false), List::cons(Adt25::C0(16u64, f1(Str::lit("bool_and")), 0u64, false, false), List::cons(Adt25::C0(17u64, f1(Str::lit("bool_or")), 0u64, false, false), List::cons(Adt25::C0(18u64, f1(Str::lit("equal")), 0u64, false, false), List::cons(Adt25::C0(19u64, f1(Str::lit("compare")), 0u64, false, false), v2))))))))));
    let v4: List<Adt25> = List::cons(Adt25::C0(20u64, f1(Str::lit("checked_add")), 1u64, false, false), List::cons(Adt25::C0(21u64, f1(Str::lit("checked_sub")), 1u64, false, false), List::cons(Adt25::C0(22u64, f1(Str::lit("checked_mul")), 1u64, false, false), List::cons(Adt25::C0(23u64, f1(Str::lit("checked_quot")), 1u64, false, false), List::cons(Adt25::C0(24u64, f1(Str::lit("checked_neg_")), 2u64, false, false), List::cons(Adt25::C0(25u64, f1(Str::lit("bit_and")), 1u64, false, false), List::cons(Adt25::C0(26u64, f1(Str::lit("bit_or")), 1u64, false, false), List::cons(Adt25::C0(27u64, f1(Str::lit("bit_xor")), 1u64, false, false), List::cons(Adt25::C0(28u64, f1(Str::lit("bit_not")), 1u64, false, false), List::cons(Adt25::C0(29u64, f1(Str::lit("shift_left")), 1u64, false, false), v3))))))))));
    let v5: List<Adt25> = List::cons(Adt25::C0(30u64, f1(Str::lit("shift_right")), 1u64, false, false), List::cons(Adt25::C0(31u64, f1(Str::lit("convert")), 1u64, false, false), List::cons(Adt25::C0(32u64, f1(Str::lit("append_list")), 0u64, false, true), List::cons(Adt25::C0(33u64, f1(Str::lit("append_bytes")), 0u64, false, true), List::cons(Adt25::C0(34u64, f1(Str::lit("length_list")), 0u64, false, true), List::cons(Adt25::C0(35u64, f1(Str::lit("length_bytes")), 0u64, false, true), List::cons(Adt25::C0(36u64, f1(Str::lit("length_string")), 0u64, false, true), List::cons(Adt25::C0(37u64, f1(Str::lit("index_list")), 0u64, false, true), List::cons(Adt25::C0(38u64, f1(Str::lit("index_bytes")), 0u64, false, true), List::cons(Adt25::C0(39u64, f1(Str::lit("slice_list")), 0u64, false, true), v4))))))))));
    let v6: List<Adt25> = List::cons(Adt25::C0(40u64, f1(Str::lit("slice_bytes")), 0u64, false, true), List::cons(Adt25::C0(41u64, f1(Str::lit("utf8_encode")), 0u64, false, true), List::cons(Adt25::C0(42u64, f1(Str::lit("utf8_decode")), 0u64, false, true), List::cons(Adt25::C0(43u64, f1(Str::lit("compare_bytes")), 0u64, false, true), List::cons(Adt25::C0(44u64, f1(Str::lit("split_exact")), 0u64, false, true), List::cons(Adt25::C0(45u64, f1(Str::lit("join")), 0u64, false, true), List::cons(Adt25::C0(46u64, f1(Str::lit("format_int")), 0u64, false, true), List::cons(Adt25::C0(47u64, f1(Str::lit("format_")), 2u64, false, true), List::cons(Adt25::C0(48u64, f1(Str::lit("parse_int")), 0u64, true, true), List::cons(Adt25::C0(49u64, f1(Str::lit("parse_")), 2u64, false, true), v5))))))))));
    let v7: List<(Bytes, List<Bytes>)> = f2(());
    let v8: Adt26 = f3(Adt26::C0(List::<Adt27>::nil(), List::<Bytes>::nil(), v6), match v0 {
        Adt0::C0(_, h1) => {
            (*h1).clone()
        }
    })?;
    let m3 = f4(Fn0::F5(v1.clone(), v7.clone()), None::<Bytes>, f6(match v8.clone() {
        Adt26::C0(h2, _, _) => {
            h2
        }
    }));
    match m3 {
        None => {
            let m5 = f7(Fn1::F8(v1, v7), None::<Bytes>, match v8 {
                Adt26::C0(_, h4, _) => {
                    h4
                }
            });
            match m5 {
                None => {
                    Ok(Ok::<(), Bytes>(()))
                }
                Some(v10) => {
                    Ok(Err::<(), Bytes>(v10))
                }
            }
        }
        Some(v9) => {
            Ok(Err::<(), Bytes>(v9))
        }
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a6 = v0;
    utf8_encode(a6)
}

pub fn f2(_: ()) -> List<(Bytes, List<Bytes>)> {
    let v1: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("function")), List::cons(f1(Str::lit("function")), List::cons(f1(Str::lit("export")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("enum:adt")), List::cons(f1(Str::lit("type:adt")), List::<Bytes>::nil())), List::cons((f1(Str::lit("enum:closures")), List::cons(f1(Str::lit("type:fn")), List::cons(f1(Str::lit("expr:closure")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("apply:dispatch")), List::cons(f1(Str::lit("type:fn")), List::cons(f1(Str::lit("expr:closure")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("let")), List::cons(f1(Str::lit("expr:let")), List::cons(f1(Str::lit("expr:match")), List::cons(f1(Str::lit("expr:prim")), List::cons(f1(Str::lit("expr:apply")), List::cons(f1(Str::lit("expr:call")), List::cons(f1(Str::lit("expr:closure")), List::cons(f1(Str::lit("expr:cond")), List::cons(f1(Str::lit("expr:field")), List::cons(f1(Str::lit("shape:some")), List::cons(f1(Str::lit("shape:ok")), List::cons(f1(Str::lit("shape:error")), List::cons(f1(Str::lit("shape:cons")), List::cons(f1(Str::lit("shape:adt")), List::cons(f1(Str::lit("shape:succ")), List::cons(f1(Str::lit("shape:zero")), List::cons(f1(Str::lit("shape:unit")), List::cons(f1(Str::lit("shape:true")), List::cons(f1(Str::lit("indirection")), List::cons(f1(Str::lit("overflow")), List::<Bytes>::nil())))))))))))))))))))), List::cons((f1(Str::lit("read")), List::cons(f1(Str::lit("expr:var")), List::cons(f1(Str::lit("expr:match")), List::cons(f1(Str::lit("expr:prim")), List::cons(f1(Str::lit("expr:call")), List::cons(f1(Str::lit("expr:apply")), List::cons(f1(Str::lit("expr:closure")), List::cons(f1(Str::lit("expr:first")), List::cons(f1(Str::lit("expr:second")), List::cons(f1(Str::lit("expr:field")), List::cons(f1(Str::lit("expr:cond")), List::cons(f1(Str::lit("shape:true")), List::cons(f1(Str::lit("shape:zero")), List::cons(f1(Str::lit("shape:some")), List::cons(f1(Str::lit("shape:ok")), List::cons(f1(Str::lit("shape:error")), List::cons(f1(Str::lit("shape:cons")), List::cons(f1(Str::lit("shape:adt")), List::cons(f1(Str::lit("export")), List::cons(f1(Str::lit("type:fn")), List::cons(f1(Str::lit("type:adt")), List::cons(f1(Str::lit("type:pair")), List::cons(f1(Str::lit("type:result")), List::<Bytes>::nil()))))))))))))))))))))))), List::<(Bytes, List<Bytes>)>::nil()))))));
    let v2: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("box")), List::cons(f1(Str::lit("indirection")), List::<Bytes>::nil())), List::cons((f1(Str::lit("destructure:pair")), List::cons(f1(Str::lit("expr:first")), List::cons(f1(Str::lit("expr:second")), List::cons(f1(Str::lit("shape:pair")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil()))))), List::cons((f1(Str::lit("type:unit")), List::cons(f1(Str::lit("type:unit")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:bool")), List::cons(f1(Str::lit("type:bool")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:nat")), List::cons(f1(Str::lit("type:nat")), List::cons(f1(Str::lit("type:fixed")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("type:int")), List::cons(f1(Str::lit("type:int")), List::cons(f1(Str::lit("type:fixed")), List::<Bytes>::nil()))), v1))))));
    let v3: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("type:fixed")), List::cons(f1(Str::lit("type:fixed")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:ordering")), List::cons(f1(Str::lit("type:ordering")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:string")), List::cons(f1(Str::lit("type:string")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:bytes")), List::cons(f1(Str::lit("type:bytes")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:option")), List::cons(f1(Str::lit("type:option")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:result")), List::cons(f1(Str::lit("type:result")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil()))), v2))))));
    let v4: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("type:list")), List::cons(f1(Str::lit("type:list")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:pair")), List::cons(f1(Str::lit("type:pair")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("type:adt")), List::cons(f1(Str::lit("type:adt")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:fn")), List::cons(f1(Str::lit("type:fn")), List::cons(f1(Str::lit("expr:closure")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("type:rc")), List::cons(f1(Str::lit("indirection")), List::<Bytes>::nil())), List::cons((f1(Str::lit("type:fallible")), List::cons(f1(Str::lit("overflow")), List::<Bytes>::nil())), v3))))));
    let v5: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("type:ref")), List::cons(f1(Str::lit("export")), List::<Bytes>::nil())), List::cons((f1(Str::lit("lit:unit")), List::cons(f1(Str::lit("value:unit")), List::cons(f1(Str::lit("shape:unit")), List::cons(f1(Str::lit("expr:var")), List::cons(f1(Str::lit("expr:call")), List::cons(f1(Str::lit("expr:apply")), List::cons(f1(Str::lit("expr:closure")), List::cons(f1(Str::lit("shape:some")), List::cons(f1(Str::lit("shape:ok")), List::cons(f1(Str::lit("shape:error")), List::cons(f1(Str::lit("shape:cons")), List::cons(f1(Str::lit("shape:adt")), List::cons(f1(Str::lit("export")), List::cons(f1(Str::lit("overflow")), List::<Bytes>::nil())))))))))))))), List::cons((f1(Str::lit("lit:bool")), List::cons(f1(Str::lit("value:bool")), List::cons(f1(Str::lit("shape:true")), List::cons(f1(Str::lit("shape:false")), List::cons(f1(Str::lit("expr:cond")), List::<Bytes>::nil()))))), List::cons((f1(Str::lit("lit:nat")), List::cons(f1(Str::lit("value:nat")), List::cons(f1(Str::lit("shape:zero")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("lit:int")), List::cons(f1(Str::lit("value:int")), List::<Bytes>::nil())), List::cons((f1(Str::lit("lit:fixed")), List::cons(f1(Str::lit("value:u8")), List::cons(f1(Str::lit("value:u16")), List::cons(f1(Str::lit("value:u32")), List::cons(f1(Str::lit("value:u64")), List::cons(f1(Str::lit("value:i8")), List::cons(f1(Str::lit("value:i16")), List::cons(f1(Str::lit("value:i32")), List::cons(f1(Str::lit("value:i64")), List::<Bytes>::nil()))))))))), v4))))));
    let v6: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("lit:string")), List::cons(f1(Str::lit("value:string")), List::<Bytes>::nil())), List::cons((f1(Str::lit("lit:bytes")), List::cons(f1(Str::lit("value:bytes")), List::<Bytes>::nil())), List::cons((f1(Str::lit("lit:ordering")), List::cons(f1(Str::lit("value:ordering")), List::cons(f1(Str::lit("shape:lt")), List::cons(f1(Str::lit("shape:eq")), List::cons(f1(Str::lit("shape:gt")), List::<Bytes>::nil()))))), List::cons((f1(Str::lit("construct:none")), List::cons(f1(Str::lit("value:none")), List::cons(f1(Str::lit("shape:none")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("construct:some")), List::cons(f1(Str::lit("value:some")), List::cons(f1(Str::lit("shape:some")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("construct:ok")), List::cons(f1(Str::lit("value:ok")), List::cons(f1(Str::lit("shape:ok")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil())))), v5))))));
    let v7: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("construct:err")), List::cons(f1(Str::lit("value:error")), List::cons(f1(Str::lit("shape:error")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil())))), List::cons((f1(Str::lit("construct:adt")), List::cons(f1(Str::lit("value:adt")), List::cons(f1(Str::lit("shape:adt")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("construct:closure")), List::cons(f1(Str::lit("expr:closure")), List::<Bytes>::nil())), List::cons((f1(Str::lit("construct:cons")), List::cons(f1(Str::lit("value:list")), List::cons(f1(Str::lit("shape:cons")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("construct:nil")), List::cons(f1(Str::lit("value:list")), List::cons(f1(Str::lit("shape:nil")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("construct:pair")), List::cons(f1(Str::lit("value:pair")), List::cons(f1(Str::lit("shape:pair")), List::cons(f1(Str::lit("expr:call")), List::<Bytes>::nil())))), v6))))));
    let v8: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("if")), List::cons(f1(Str::lit("expr:cond")), List::cons(f1(Str::lit("shape:true")), List::cons(f1(Str::lit("shape:zero")), List::<Bytes>::nil())))), List::cons((f1(Str::lit("match:nat")), List::cons(f1(Str::lit("shape:zero")), List::<Bytes>::nil())), List::cons((f1(Str::lit("match:list")), List::cons(f1(Str::lit("shape:nil")), List::<Bytes>::nil())), List::cons((f1(Str::lit("match:option")), List::cons(f1(Str::lit("shape:none")), List::<Bytes>::nil())), List::cons((f1(Str::lit("match:result")), List::cons(f1(Str::lit("shape:ok")), List::<Bytes>::nil())), List::cons((f1(Str::lit("match:ordering")), List::cons(f1(Str::lit("shape:lt")), List::<Bytes>::nil())), v7))))));
    let v9: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("match:adt")), List::cons(f1(Str::lit("shape:adt")), List::cons(f1(Str::lit("expr:field")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("match:empty")), List::cons(f1(Str::lit("type:fn")), List::cons(f1(Str::lit("type:adt")), List::cons(f1(Str::lit("type:pair")), List::cons(f1(Str::lit("type:result")), List::<Bytes>::nil()))))), List::cons((f1(Str::lit("call:function")), List::cons(f1(Str::lit("expr:call")), List::cons(f1(Str::lit("export")), List::<Bytes>::nil()))), List::cons((f1(Str::lit("apply")), List::cons(f1(Str::lit("expr:apply")), List::<Bytes>::nil())), List::cons((f1(Str::lit("widen")), List::cons(f1(Str::lit("prim:convert")), List::<Bytes>::nil())), List::cons((f1(Str::lit("succeed")), List::cons(f1(Str::lit("overflow")), List::<Bytes>::nil())), v8))))));
    let v10: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:succ")), List::cons(f1(Str::lit("shape:succ")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_add")), List::cons(f1(Str::lit("prim:nat_add")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_sub")), List::cons(f1(Str::lit("prim:nat_sub")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_mul")), List::cons(f1(Str::lit("prim:nat_mul")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_quot")), List::cons(f1(Str::lit("prim:nat_quot")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_rem")), List::cons(f1(Str::lit("prim:nat_rem")), List::<Bytes>::nil())), v9))))));
    let v11: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:nat_eq")), List::cons(f1(Str::lit("prim:nat_eq")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_le")), List::cons(f1(Str::lit("prim:nat_le")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:nat_lt")), List::cons(f1(Str::lit("prim:nat_lt")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:int_add")), List::cons(f1(Str::lit("prim:int_add")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:int_sub")), List::cons(f1(Str::lit("prim:int_sub")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:int_mul")), List::cons(f1(Str::lit("prim:int_mul")), List::<Bytes>::nil())), v10))))));
    let v12: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:int_neg")), List::cons(f1(Str::lit("prim:int_neg")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:int_quot")), List::cons(f1(Str::lit("prim:int_quot")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:int_rem")), List::cons(f1(Str::lit("prim:int_rem")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bool_not")), List::cons(f1(Str::lit("prim:bool_not")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bool_and")), List::cons(f1(Str::lit("prim:bool_and")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bool_or")), List::cons(f1(Str::lit("prim:bool_or")), List::<Bytes>::nil())), v11))))));
    let v13: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:equal")), List::cons(f1(Str::lit("prim:equal")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:compare")), List::cons(f1(Str::lit("prim:compare")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:checked_add")), List::cons(f1(Str::lit("prim:checked_add")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:checked_sub")), List::cons(f1(Str::lit("prim:checked_sub")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:checked_mul")), List::cons(f1(Str::lit("prim:checked_mul")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:checked_quot")), List::cons(f1(Str::lit("prim:checked_quot")), List::<Bytes>::nil())), v12))))));
    let v14: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:checked_neg")), List::cons(f1(Str::lit("prim:checked_neg")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bit_and")), List::cons(f1(Str::lit("prim:bit_and")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bit_or")), List::cons(f1(Str::lit("prim:bit_or")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bit_xor")), List::cons(f1(Str::lit("prim:bit_xor")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:bit_not")), List::cons(f1(Str::lit("prim:bit_not")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:shift_left")), List::cons(f1(Str::lit("prim:shift_left")), List::<Bytes>::nil())), v13))))));
    let v15: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:shift_right")), List::cons(f1(Str::lit("prim:shift_right")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:convert")), List::cons(f1(Str::lit("prim:convert")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:append")), List::cons(f1(Str::lit("prim:append")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:length")), List::cons(f1(Str::lit("prim:length")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:index")), List::cons(f1(Str::lit("prim:index")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:slice")), List::cons(f1(Str::lit("prim:slice")), List::<Bytes>::nil())), v14))))));
    let v16: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:utf8_encode")), List::cons(f1(Str::lit("prim:utf8_encode")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:utf8_decode")), List::cons(f1(Str::lit("prim:utf8_decode")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:compare_bytes")), List::cons(f1(Str::lit("prim:compare_bytes")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:split_exact")), List::cons(f1(Str::lit("prim:split_exact")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:join")), List::cons(f1(Str::lit("prim:join")), List::<Bytes>::nil())), List::cons((f1(Str::lit("call:runtime:format_decimal")), List::cons(f1(Str::lit("prim:format_decimal")), List::<Bytes>::nil())), v15))))));
    let v17: List<(Bytes, List<Bytes>)> = List::cons((f1(Str::lit("call:runtime:parse_decimal")), List::cons(f1(Str::lit("prim:parse_decimal")), List::<Bytes>::nil())), v16);
    v17
}

pub fn f3(v0: Adt26, v1: Adt1) -> R<Adt26> {
    let m7 = v1;
    match m7 {
        Adt1::C0(r9, r10, r11) => {
            let v2 = (*r9).clone();
            let v3 = (*r10).clone();
            let v4 = (*r11).clone();
            Ok(f9(f10(v0, {
                let m8 = f11(v2);
                if m8 {
                    f1(Str::lit("enum:closures"))
                } else {
                    f1(Str::lit("enum:adt"))
                }
            }, v4), v3))
        }
        Adt1::C1(_, r12, r13, _, _, r14) => {
            let v6 = (*r12).clone();
            let v7 = (*r13).clone();
            let v10 = (*r14).clone();
            Ok(f12(f12(f10(v0, f1(Str::lit("apply:dispatch")), v10), v6), v7))
        }
        Adt1::C2(_, r15, r16, r17, r18) => {
            let v12 = (*r15).clone();
            let v13 = (*r16).clone();
            let v14 = (*r17).clone();
            let v15 = (*r18).clone();
            f13(f12(f14(f10(v0, f1(Str::lit("function")), v15), v12), v13), v14)
        }
        Adt1::C3 => {
            Ok(v0)
        }
        Adt1::C4(r19, r20) => {
            let v16 = (*r19).clone();
            let v17 = (*r20).clone();
            let v18: Adt26 = f3(v0, v16)?;
            f3(v18, v17)
        }
    }
}

pub fn f4(v0: Fn0, p24: Option<Bytes>, p25: List<Adt27>) -> Option<Bytes> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m21 = v2;
                match m21.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt27>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt27>), Option<Bytes>>(({
                            let c22: Fn0 = v0.clone();
                            c22.apply(v1, v3)
                        }, v4))
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

pub fn f5(v0: List<Bytes>, v1: List<(Bytes, List<Bytes>)>, v2: Option<Bytes>, v3: Adt27) -> Option<Bytes> {
    let m26 = v2;
    match m26 {
        None => {
            f15(v1, v0, v3)
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f6(v0: List<Adt27>) -> List<Adt27> {
    f16(Fn2::F17, List::<Adt27>::nil(), v0)
}

pub fn f7(v0: Fn1, p30: Option<Bytes>, p31: List<Bytes>) -> Option<Bytes> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v2;
                match m27.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c28: Fn1 = v0.clone();
                            c28.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m29 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: List<Bytes>, v1: List<(Bytes, List<Bytes>)>, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m32 = v2;
    match m32 {
        None => {
            f18(v1, v0, v3)
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f9(p37: Adt26, p38: Adt8) -> Adt26 {
    let m36 = (p37, p38);
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v0, v1) = m36;
                let m33 = v1;
                match m33 {
                    Adt8::C0 => {
                        Err::<(Adt26, Adt8), Adt26>(v0)
                    }
                    Adt8::C1(_, r34, r35) => {
                        let v3 = (*r34).clone();
                        let v4 = (*r35).clone();
                        Ok::<(Adt26, Adt8), Adt26>((f12(v0, v3), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt26, v1: Bytes, v2: Adt9) -> Adt26 {
    f19(v0, v1, v2, None::<Adt4>)
}

pub fn f11(v0: Adt2) -> bool {
    let m39 = v0;
    match m39 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(v8, _) => {
            f20(v8)
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f12(v0: Adt26, v1: Adt2) -> Adt26 {
    let m40 = v1;
    match m40 {
        Adt2::C0(v2) => {
            f21(v0, f22(f1(Str::lit("type:")), f23(v2)))
        }
        Adt2::C1(_) => {
            f21(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r41) => {
            let v5 = (*r41).clone();
            let v6: Adt26 = f21(v0, f22(f1(Str::lit("type:")), f24(v4)));
            f12(v6, v5)
        }
        Adt2::C3(v7, r42, r43) => {
            let v8 = (*r42).clone();
            let v9 = (*r43).clone();
            let v10: Adt26 = f21(v0, f22(f1(Str::lit("type:")), f25(v7)));
            let v11: Adt26 = f12(v10, v8);
            f12(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f21(v0, f22(f1(Str::lit("type:")), f26(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r44, r45) => {
            let v14 = (*r44).clone();
            let v15 = (*r45).clone();
            let v16: Adt26 = f12(v0, v14);
            f12(v16, v15)
        }
    }
}

pub fn f13(v0: Adt26, v1: Adt17) -> R<Adt26> {
    let m46 = v1;
    match m46 {
        Adt17::C0(r57, r58) => {
            let v2 = (*r57).clone();
            let v3 = (*r58).clone();
            let m47 = f27(v2.clone());
            if m47 {
                Ok(v0)
            } else {
                Ok(f19(v0, f28(v2.clone()), v3, f29(v2)))
            }
        }
        Adt17::C1(v4, _, r59) => {
            let v6 = (*r59).clone();
            Ok(f10(v0, f30(v4), v6))
        }
        Adt17::C2(v7, r60, r61) => {
            let v8 = (*r60).clone();
            let v9 = (*r61).clone();
            let v10: Adt26 = f10(v0, f31(v7), v9);
            f13(v10, v8)
        }
        Adt17::C3(r62, r63, _, r64) => {
            let v11 = (*r62).clone();
            let v12 = (*r63).clone();
            let v14 = (*r64).clone();
            let v18: Adt26 = {
                let m48 = v11;
                match m48 {
                    Adt21::C0(_) => {
                        f10(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(r51) => {
                        let v17 = (*r51).clone();
                        f19(v0.clone(), f22(f1(Str::lit("call:runtime:")), f32(f33(f34(match v0.clone() {
                            Adt26::C0(_, _, h49) => {
                                h49
                            }
                        }, v17.clone()))?)), v14, f35(match v0 {
                            Adt26::C0(_, _, h50) => {
                                h50
                            }
                        }, v17))
                    }
                    Adt21::C2(_) => {
                        f10(v0, f1(Str::lit("call:machine")), v14)
                    }
                }
            };
            f13(v18, v12)
        }
        Adt17::C4(_, r65, _, r66) => {
            let v20 = (*r65).clone();
            let v22 = (*r66).clone();
            let v23: Adt26 = f10(v0, f1(Str::lit("apply")), v22);
            f13(v23, v20)
        }
        Adt17::C5(r67, r68, r69) => {
            let v24 = (*r67).clone();
            let v25 = (*r68).clone();
            let v26 = (*r69).clone();
            let v27: Adt26 = f10(f36(v0, v24.clone()), f22(f1(Str::lit("construct:")), f37(v24)), v26);
            f13(v27, v25)
        }
        Adt17::C6(r70, r71, r72) => {
            let v28 = (*r70).clone();
            let v29 = (*r71).clone();
            let v30 = (*r72).clone();
            let v31: Adt26 = f10(v0, f1(Str::lit("construct:pair")), v30);
            let v32: Adt26 = f13(v31, v28)?;
            f13(v32, v29)
        }
        Adt17::C7(r73, r74, r75, r76) => {
            let v33 = (*r73).clone();
            let v34 = (*r74).clone();
            let v35 = (*r75).clone();
            let v36 = (*r76).clone();
            let v37: Adt26 = f10(v0, f1(Str::lit("if")), v36);
            let v38: Adt26 = f13(v37, v33)?;
            let v39: Adt26 = f13(v38, v34)?;
            f13(v39, v35)
        }
        Adt17::C8(r77, r78, r79) => {
            let v40 = (*r77).clone();
            let v41 = (*r78).clone();
            let v42 = (*r79).clone();
            let v43: Adt26 = {
                let m52 = f38(v41.clone());
                if m52 {
                    f10(v0, f1(Str::lit("match:empty")), v42)
                } else {
                    let m53 = f39(v40.clone());
                    if m53 {
                        f10(v0, f1(Str::lit("match:list")), v42)
                    } else {
                        f40(v0, v41.clone(), v42)
                    }
                }
            };
            let v44: Adt26 = f13(v43, v40)?;
            f13(v44, v41)
        }
        Adt17::C9(r80) => {
            let v45 = (*r80).clone();
            f13(v0, v45)
        }
        Adt17::C10(v46, _, r81) => {
            let v48 = (*r81).clone();
            let m54 = f41(v46);
            if m54 {
                Ok(f10(v0, f1(Str::lit("match:nat")), v48))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r82, r83) => {
            let v49 = (*r82).clone();
            let v50 = (*r83).clone();
            let v51: Adt26 = f13(v0, v49)?;
            f13(v51, v50)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r84, r85) => {
            let v52 = (*r84).clone();
            let v53 = (*r85).clone();
            let v54: Adt26 = f13(v0, v52)?;
            f13(v54, v53)
        }
        Adt17::C14(r86, v56, r87, r88, r89, r90) => {
            let v55 = (*r86).clone();
            let v57 = (*r87).clone();
            let v58 = (*r88).clone();
            let v59 = (*r89).clone();
            let v60 = (*r90).clone();
            let v61: Bytes = {
                let m55 = f42(v55);
                if m55 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v62: Adt26 = {
                let m56 = v56;
                if m56 {
                    f12(v0, v57)
                } else {
                    v0
                }
            };
            let v63: Adt26 = f10(v62, v61, v59);
            let v64: Adt26 = f13(v63, v58)?;
            f13(v64, v60)
        }
        Adt17::C15(_, r91, r92) => {
            let v66 = (*r91).clone();
            let v67 = (*r92).clone();
            let v68: Adt26 = f13(v0, v66)?;
            f13(v68, v67)
        }
        Adt17::C16(_, r93) => {
            let v70 = (*r93).clone();
            f13(v0, v70)
        }
    }
}

pub fn f14(p98: Adt26, p99: Adt13) -> Adt26 {
    let m97 = (p98, p99);
    {
        let mut m97 = m97;
        loop {
            let step = {
                let (v0, v1) = m97;
                let m94 = v1;
                match m94 {
                    Adt13::C0 => {
                        Err::<(Adt26, Adt13), Adt26>(v0)
                    }
                    Adt13::C1(_, r95, r96) => {
                        let v3 = (*r95).clone();
                        let v4 = (*r96).clone();
                        Ok::<(Adt26, Adt13), Adt26>((f12(v0, v3), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m97 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Adt27) -> Option<Bytes> {
    let v3: Bytes = match v2.clone() {
        Adt27::C0(h100, _, _, _) => {
            h100
        }
    };
    let v4: Bytes = match v2.clone() {
        Adt27::C0(_, h101, _, _) => {
            h101
        }
    };
    let m102 = f43(v0, v3.clone());
    match m102 {
        None => {
            Some(f22(f1(Str::lit("the construct ")), f22(f44(v3), f1(Str::lit(" has no target-semantics correspondence")))))
        }
        Some(v5) => {
            let m103 = f45(v5, v4.clone());
            if m103 {
                let m104 = f45(v1, v4.clone());
                if m104 {
                    let v6: bool = {
                        let a107 = f46(match v2.clone() {
                            Adt27::C0(_, _, _, h105) => {
                                h105
                            }
                        }, match v2.clone() {
                            Adt27::C0(_, _, h106, _) => {
                                h106
                            }
                        });
                        bool_not(a107)
                    };
                    let v8: bool = {
                        let m109 = match v2.clone() {
                            Adt27::C0(_, _, _, h108) => {
                                h108
                            }
                        };
                        match m109 {
                            None => {
                                f47(v3.clone(), f1(Str::lit("call:runtime:")))
                            }
                            Some(_) => {
                                true
                            }
                        }
                    };
                    let m110 = if v6 {
                        v8
                    } else {
                        false
                    };
                    if m110 {
                        Some(f22(f1(Str::lit("the construct ")), f22(f44(v3), f22(f1(Str::lit(" works at width ")), f22(f48(match v2.clone() {
                            Adt27::C0(_, _, _, h111) => {
                                h111
                            }
                        }), f22(f1(Str::lit(", but ")), f22(f44(v4), f22(f1(Str::lit(" is at ")), f48(match v2 {
                            Adt27::C0(_, _, h112, _) => {
                                h112
                            }
                        })))))))))
                    } else {
                        None::<Bytes>
                    }
                } else {
                    Some(f22(f1(Str::lit("the construct ")), f22(f44(v3), f22(f1(Str::lit(" realizes ")), f22(f44(v4), f1(Str::lit(", which the program does not use")))))))
                }
            } else {
                Some(f22(f1(Str::lit("the construct ")), f22(f44(v3), f22(f1(Str::lit(" does not realize ")), f22(f44(v4), f1(Str::lit(", the element it was lowered from")))))))
            }
        }
    }
}

pub fn f16(v0: Fn2, p116: List<Adt27>, p117: List<Adt27>) -> List<Adt27> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v1, v2) = m115;
                let m113 = v2;
                match m113.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), List<Adt27>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt27>, List<Adt27>), List<Adt27>>(({
                            let c114: Fn2 = v0.clone();
                            c114.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: List<Adt27>, v1: Adt27) -> List<Adt27> {
    List::cons(v1, v0)
}

pub fn f18(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m118 = f43(v0, v2.clone());
    match m118 {
        None => {
            Some(f22(f1(Str::lit("the construct ")), f22(f44(v2), f1(Str::lit(" has no target-semantics correspondence")))))
        }
        Some(v3) => {
            let m119 = f49(v3.clone(), v1);
            if m119 {
                None::<Bytes>
            } else {
                Some(f22(f1(Str::lit("the construct ")), f22(f44(v2), f22(f1(Str::lit(" realizes none of ")), f22(f50(v3), f1(Str::lit(", which the program does not use")))))))
            }
        }
    }
}

pub fn f19(v0: Adt26, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt26 {
    let v4: Adt27 = Adt27::C0(v1, match v2.clone() {
        Adt9::C0(h120, _) => {
            h120
        }
    }, match v2 {
        Adt9::C0(_, h121) => {
            h121
        }
    }, v3);
    let m123 = f51(match v0.clone() {
        Adt26::C0(h122, _, _) => {
            h122
        }
    }, v4.clone());
    if m123 {
        v0
    } else {
        Adt26::C0(List::cons(v4, match v0.clone() {
            Adt26::C0(h124, _, _) => {
                h124
            }
        }), match v0.clone() {
            Adt26::C0(_, h125, _) => {
                h125
            }
        }, match v0 {
            Adt26::C0(_, _, h126) => {
                h126
            }
        })
    }
}

pub fn f20(v0: Adt7) -> bool {
    let m127 = v0;
    match m127 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            true
        }
        Adt7::C2 => {
            false
        }
    }
}

pub fn f21(v0: Adt26, v1: Bytes) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h128, _, _) => {
            h128
        }
    }, f52(match v0.clone() {
        Adt26::C0(_, h129, _) => {
            h129
        }
    }, v1), match v0 {
        Adt26::C0(_, _, h130) => {
            h130
        }
    })
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    let a131 = v0;
    let a132 = v1;
    append_bytes(a131, a132)
}

pub fn f23(v0: Adt3) -> Bytes {
    let m133 = v0;
    match m133 {
        Adt3::C0 => {
            f1(Str::lit("unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("int"))
        }
        Adt3::C4 => {
            f1(Str::lit("ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("string"))
        }
        Adt3::C6 => {
            f1(Str::lit("bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("i128"))
        }
    }
}

pub fn f24(v0: Adt5) -> Bytes {
    let m134 = v0;
    match m134 {
        Adt5::C0 => {
            f1(Str::lit("option"))
        }
        Adt5::C1 => {
            f1(Str::lit("list"))
        }
        Adt5::C2 => {
            f1(Str::lit("rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("ref"))
        }
    }
}

pub fn f25(v0: Adt6) -> Bytes {
    let m135 = v0;
    match m135 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f26(v0: Adt7) -> Bytes {
    let m136 = v0;
    match m136 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("param"))
        }
    }
}

pub fn f27(v0: Adt18) -> bool {
    let m137 = v0;
    match m137 {
        Adt18::C0 => {
            false
        }
        Adt18::C1 => {
            false
        }
        Adt18::C2 => {
            false
        }
        Adt18::C3 => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5 => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7 => {
            false
        }
        Adt18::C8 => {
            true
        }
    }
}

pub fn f28(v0: Adt18) -> Bytes {
    let m138 = v0;
    match m138 {
        Adt18::C0 => {
            f1(Str::lit("lit:unit"))
        }
        Adt18::C1 => {
            f1(Str::lit("lit:bool"))
        }
        Adt18::C2 => {
            f1(Str::lit("lit:nat"))
        }
        Adt18::C3 => {
            f1(Str::lit("lit:int"))
        }
        Adt18::C4(_) => {
            f1(Str::lit("lit:fixed"))
        }
        Adt18::C5 => {
            f1(Str::lit("lit:string"))
        }
        Adt18::C6 => {
            f1(Str::lit("lit:bytes"))
        }
        Adt18::C7 => {
            f1(Str::lit("lit:ordering"))
        }
        Adt18::C8 => {
            f1(Str::lit("lit:overflow"))
        }
    }
}

pub fn f29(v0: Adt18) -> Option<Adt4> {
    let m139 = v0;
    match m139 {
        Adt18::C0 => {
            None::<Adt4>
        }
        Adt18::C1 => {
            None::<Adt4>
        }
        Adt18::C2 => {
            None::<Adt4>
        }
        Adt18::C3 => {
            None::<Adt4>
        }
        Adt18::C4(v1) => {
            Some(v1)
        }
        Adt18::C5 => {
            None::<Adt4>
        }
        Adt18::C6 => {
            None::<Adt4>
        }
        Adt18::C7 => {
            None::<Adt4>
        }
        Adt18::C8 => {
            None::<Adt4>
        }
    }
}

pub fn f30(v0: Adt19) -> Bytes {
    let m140 = v0;
    match m140 {
        Adt19::C0 => {
            f1(Str::lit("read"))
        }
        Adt19::C1 => {
            f1(Str::lit("read"))
        }
        Adt19::C2 => {
            f1(Str::lit("read"))
        }
        Adt19::C3 => {
            f1(Str::lit("read"))
        }
        Adt19::C4 => {
            f1(Str::lit("box"))
        }
    }
}

pub fn f31(v0: Adt20) -> Bytes {
    let m141 = v0;
    match m141 {
        Adt20::C0 => {
            f1(Str::lit("if"))
        }
        Adt20::C1 => {
            f1(Str::lit("box"))
        }
        Adt20::C2 => {
            f1(Str::lit("widen"))
        }
        Adt20::C3 => {
            f1(Str::lit("succeed"))
        }
    }
}

pub fn f32(v0: Bytes) -> Bytes {
    let m142 = f47(v0.clone(), f1(Str::lit("checked_neg_")));
    if m142 {
        f1(Str::lit("checked_neg"))
    } else {
        let m143 = f47(v0.clone(), f1(Str::lit("format_")));
        if m143 {
            f1(Str::lit("format_decimal"))
        } else {
            let m144 = f47(v0.clone(), f1(Str::lit("parse_")));
            if m144 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m145 = f47(v0.clone(), f1(Str::lit("append_")));
                if m145 {
                    f1(Str::lit("append"))
                } else {
                    let m146 = f47(v0.clone(), f1(Str::lit("length_")));
                    if m146 {
                        f1(Str::lit("length"))
                    } else {
                        let m147 = f47(v0.clone(), f1(Str::lit("index_")));
                        if m147 {
                            f1(Str::lit("index"))
                        } else {
                            let m148 = f47(v0.clone(), f1(Str::lit("slice_")));
                            if m148 {
                                f1(Str::lit("slice"))
                            } else {
                                let m151 = {
                                    let a149 = v0.clone();
                                    let a150 = f1(Str::lit("nat_succ"));
                                    equal(a149, a150)
                                };
                                if m151 {
                                    f1(Str::lit("succ"))
                                } else {
                                    v0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f33(v0: Bytes) -> R<Bytes> {
    let m152 = f53(v0.clone(), f1(Str::lit("::")))?;
    match m152 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f54(v0.clone(), {
                let a153 = v1;
                let a154 = 2u64;
                nat_add(a153, a154)?
            }, {
                let a158 = {
                    let a155 = v0;
                    length_bytes(a155)
                };
                let a159 = {
                    let a156 = v1;
                    let a157 = 2u64;
                    nat_add(a156, a157)?
                };
                nat_sub(a158, a159)
            }))
        }
    }
}

pub fn f34(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m161 = f55(v0, match v1.clone() {
        Adt22::C0(h160, _) => {
            h160
        }
    });
    match m161 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a163 = match v2.clone() {
                Adt25::C0(_, _, h162, _, _) => {
                    h162
                }
            };
            let a164 = 0u64;
            if nat_eq(a163, a164) {
                match v2 {
                    Adt25::C0(_, h165, _, _, _) => {
                        h165
                    }
                }
            } else {
                let a167 = match v2.clone() {
                    Adt25::C0(_, _, h166, _, _) => {
                        h166
                    }
                };
                let a168 = 1u64;
                if nat_eq(a167, a168) {
                    f22(f1(Str::lit("fixed_")), f22(f56(f57(match v1 {
                        Adt22::C0(_, h169) => {
                            h169
                        }
                    })), f22(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h170, _, _, _) => {
                            h170
                        }
                    })))
                } else {
                    f22(match v2 {
                        Adt25::C0(_, h171, _, _, _) => {
                            h171
                        }
                    }, f56(f57(match v1 {
                        Adt22::C0(_, h172) => {
                            h172
                        }
                    })))
                }
            }
        }
    }
}

pub fn f35(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m174 = f55(v0, match v1.clone() {
        Adt22::C0(h173, _) => {
            h173
        }
    });
    match m174 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a176 = match v2 {
                Adt25::C0(_, _, h175, _, _) => {
                    h175
                }
            };
            let a177 = 0u64;
            if nat_eq(a176, a177) {
                None::<Adt4>
            } else {
                Some(f57(match v1 {
                    Adt22::C0(_, h178) => {
                        h178
                    }
                }))
            }
        }
    }
}

pub fn f36(v0: Adt26, v1: Adt23) -> Adt26 {
    let m179 = v1;
    match m179 {
        Adt23::C0(r180) => {
            let v2 = (*r180).clone();
            f12(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(r181, r182) => {
            let v3 = (*r181).clone();
            let v4 = (*r182).clone();
            f12(f12(v0, v3), v4)
        }
        Adt23::C3(r183, r184) => {
            let v5 = (*r183).clone();
            let v6 = (*r184).clone();
            f12(f12(v0, v5), v6)
        }
        Adt23::C4(_, _) => {
            v0
        }
        Adt23::C5(_, _) => {
            v0
        }
        Adt23::C6 => {
            v0
        }
        Adt23::C7(r185) => {
            let v11 = (*r185).clone();
            f12(v0, v11)
        }
    }
}

pub fn f37(v0: Adt23) -> Bytes {
    let m186 = v0;
    match m186 {
        Adt23::C0(_) => {
            f1(Str::lit("none"))
        }
        Adt23::C1 => {
            f1(Str::lit("some"))
        }
        Adt23::C2(_, _) => {
            f1(Str::lit("ok"))
        }
        Adt23::C3(_, _) => {
            f1(Str::lit("err"))
        }
        Adt23::C4(_, _) => {
            f1(Str::lit("adt"))
        }
        Adt23::C5(_, _) => {
            f1(Str::lit("closure"))
        }
        Adt23::C6 => {
            f1(Str::lit("cons"))
        }
        Adt23::C7(_) => {
            f1(Str::lit("nil"))
        }
    }
}

pub fn f38(v0: Adt17) -> bool {
    let m187 = v0;
    match m187 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            true
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f39(v0: Adt17) -> bool {
    let m188 = v0;
    match m188 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f58(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f40(p194: Adt26, p195: Adt17, v2: Adt9) -> Adt26 {
    let m193 = (p194, p195);
    {
        let mut m193 = m193;
        loop {
            let step = {
                let (v0, v1) = m193;
                let m189 = v1;
                match m189 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C12 => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                    Adt17::C15(r191, _, r192) => {
                        let v46 = (*r191).clone();
                        let v48 = (*r192).clone();
                        let m190 = f59(v46);
                        match m190 {
                            None => {
                                Ok::<(Adt26, Adt17), Adt26>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt26, Adt17), Adt26>((f10(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt26, Adt17), Adt26>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt24) -> bool {
    let m196 = v0;
    match m196 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f42(v0: Adt14) -> bool {
    let m197 = v0;
    match m197 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            true
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f43(v0: List<(Bytes, List<Bytes>)>, v1: Bytes) -> Option<List<Bytes>> {
    f60(Fn3::F61(v1), None::<List<Bytes>>, v0)
}

pub fn f44(v0: Bytes) -> Bytes {
    f22(f1(Str::lit("`")), f22(v0, f1(Str::lit("`"))))
}

pub fn f45(v0: List<Bytes>, v1: Bytes) -> bool {
    f62(Fn4::F63(v1), false, v0)
}

pub fn f46(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a198 = f48(v0);
    let a199 = f48(v1);
    equal(a198, a199)
}

pub fn f47(v0: Bytes, v1: Bytes) -> bool {
    let m204 = {
        let a201 = v0;
        let a202 = 0u64;
        let a203 = {
            let a200 = v1.clone();
            length_bytes(a200)
        };
        slice_bytes(a201, a202, a203)
    };
    match m204 {
        None => {
            false
        }
        Some(v2) => {
            let a205 = v2;
            let a206 = v1;
            equal(a205, a206)
        }
    }
}

pub fn f48(v0: Option<Adt4>) -> Bytes {
    let m207 = v0;
    match m207 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f22(f1(Str::lit("Some(")), f22(f64(v1), f1(Str::lit(")"))))
        }
    }
}

pub fn f49(v0: List<Bytes>, v1: List<Bytes>) -> bool {
    f62(Fn4::F65(v1), false, v0)
}

pub fn f50(v0: List<Bytes>) -> Bytes {
    f22(Bytes::lit(&[0x5b, 0x22]), f22(f66(v0, Bytes::lit(&[0x22, 0x2c, 0x20, 0x22])), Bytes::lit(&[0x22, 0x5d])))
}

pub fn f51(v0: List<Adt27>, v1: Adt27) -> bool {
    f67(Fn5::F68(std::rc::Rc::new(v1)), false, v0)
}

pub fn f52(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m208 = f45(v0.clone(), v1.clone());
    if m208 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f69(Fn6::F70(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m210 = {
            let (h209, _) = v2;
            h209
        };
        if m210 {
            f71({
                let (_, h211) = v2;
                h211
            })
        } else {
            f71(List::cons(v1, {
                let (_, h212) = v2;
                h212
            }))
        }
    }
}

pub fn f53(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a213 = v0.clone();
        length_bytes(a213)
    };
    let v3: u64 = {
        let a214 = v1.clone();
        length_bytes(a214)
    };
    let v4: u64 = {
        let (h215, _) = f72(Fn7::F73(v1.clone(), v3, v0.clone(), v2), {
            let a216 = v2;
            let a217 = 1u64;
            nat_add(a216, a217)?
        }, 0u64)?;
        h215
    };
    if f74(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m221 = {
        let a218 = v0;
        let a219 = v1;
        let a220 = v2;
        slice_bytes(a218, a219, a220)
    };
    match m221 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f55(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f75(Fn8::F76(v1), None::<Adt25>, v0)
}

pub fn f56(v0: Adt4) -> Bytes {
    let m222 = v0;
    match m222 {
        Adt4::C0 => {
            f1(Str::lit("u8"))
        }
        Adt4::C1 => {
            f1(Str::lit("u16"))
        }
        Adt4::C2 => {
            f1(Str::lit("u32"))
        }
        Adt4::C3 => {
            f1(Str::lit("u64"))
        }
        Adt4::C4 => {
            f1(Str::lit("i8"))
        }
        Adt4::C5 => {
            f1(Str::lit("i16"))
        }
        Adt4::C6 => {
            f1(Str::lit("i32"))
        }
        Adt4::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f57(v0: Option<Adt4>) -> Adt4 {
    let m223 = v0;
    match m223 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f58(v0: Adt24) -> bool {
    let m224 = v0;
    match m224 {
        Adt24::C0 => {
            false
        }
        Adt24::C1 => {
            false
        }
        Adt24::C2 => {
            true
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f59(v0: Adt14) -> Option<Bytes> {
    let m225 = v0;
    match m225 {
        Adt14::C0(v1) => {
            f77(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f78(v4))
        }
        Adt14::C4(_, _, _) => {
            Some(f1(Str::lit("match:adt")))
        }
        Adt14::C5 => {
            None::<Bytes>
        }
        Adt14::C6(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f60(v0: Fn3, p229: Option<List<Bytes>>, p230: List<(Bytes, List<Bytes>)>) -> Option<List<Bytes>> {
    let m228 = (p229, p230);
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v1, v2) = m228;
                let m226 = v2;
                match m226.uncons() {
                    None => {
                        Err::<(Option<List<Bytes>>, List<(Bytes, List<Bytes>)>), Option<List<Bytes>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Bytes>>, List<(Bytes, List<Bytes>)>), Option<List<Bytes>>>(({
                            let c227: Fn3 = v0.clone();
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

pub fn f61(v0: Bytes, v1: Option<List<Bytes>>, v2: (Bytes, List<Bytes>)) -> Option<List<Bytes>> {
    let m231 = v1;
    match m231 {
        None => {
            let a233 = {
                let (h232, _) = v2;
                h232
            };
            let a234 = v0;
            if equal(a233, a234) {
                Some({
                    let (_, h235) = v2;
                    h235
                })
            } else {
                None::<List<Bytes>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f62(v0: Fn4, p239: bool, p240: List<Bytes>) -> bool {
    let m238 = (p239, p240);
    {
        let mut m238 = m238;
        loop {
            let step = {
                let (v1, v2) = m238;
                let m236 = v2;
                match m236.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c237: Fn4 = v0.clone();
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

pub fn f63(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a241 = v2;
        let a242 = v0;
        equal(a241, a242)
    }
}

pub fn f64(v0: Adt4) -> Bytes {
    let m243 = v0;
    match m243 {
        Adt4::C0 => {
            f1(Str::lit("U8"))
        }
        Adt4::C1 => {
            f1(Str::lit("U16"))
        }
        Adt4::C2 => {
            f1(Str::lit("U32"))
        }
        Adt4::C3 => {
            f1(Str::lit("U64"))
        }
        Adt4::C4 => {
            f1(Str::lit("I8"))
        }
        Adt4::C5 => {
            f1(Str::lit("I16"))
        }
        Adt4::C6 => {
            f1(Str::lit("I32"))
        }
        Adt4::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f65(v0: List<Bytes>, v1: bool, v2: Bytes) -> bool {
    let v3: bool = f45(v0, v2);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f66(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m244 = f7(Fn1::F79(v1), None::<Bytes>, v0);
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f67(v0: Fn5, p248: bool, p249: List<Adt27>) -> bool {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m245 = v2;
                match m245.uncons() {
                    None => {
                        Err::<(bool, List<Adt27>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt27>), bool>(({
                            let c246: Fn5 = v0.clone();
                            c246.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt27, v1: bool, v2: Adt27) -> bool {
    let v3: bool = f80(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f69(v0: Fn6, p253: (bool, List<Bytes>), p254: List<Bytes>) -> (bool, List<Bytes>) {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m250 = v2;
                match m250.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c251: Fn6 = v0.clone();
                            c251.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f81(v0, v1, v2)
}

pub fn f71(v0: List<Bytes>) -> List<Bytes> {
    f82(Fn9::F83, List::<Bytes>::nil(), v0)
}

pub fn f72(v0: Fn7, p259: u64, p260: u64) -> R<(u64, bool)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m255 = v1;
                if m255 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m255 - 1;
                    let m257 = {
                        let c256: Fn7 = v0.clone();
                        c256.apply(v2)?
                    };
                    match m257 {
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
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f74(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a263 = {
            let a261 = v4;
            let a262 = v1;
            nat_add(a261, a262)?
        };
        let a264 = v3;
        if nat_lt(a263, a264) {
            Ok(Some({
                let a265 = v4;
                let a266 = 1u64;
                nat_add(a265, a266)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m271 = {
        let a268 = v0;
        let a269 = v2;
        let a270 = {
            let a267 = v1.clone();
            length_bytes(a267)
        };
        slice_bytes(a268, a269, a270)
    };
    match m271 {
        None => {
            false
        }
        Some(v3) => {
            let a272 = v3;
            let a273 = v1;
            equal(a272, a273)
        }
    }
}

pub fn f75(v0: Fn8, p277: Option<Adt25>, p278: List<Adt25>) -> Option<Adt25> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m274 = v2;
                match m274.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c275: Fn8 = v0.clone();
                            c275.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m279 = v1;
    match m279 {
        None => {
            let a281 = match v2.clone() {
                Adt25::C0(h280, _, _, _, _) => {
                    h280
                }
            };
            let a282 = v0;
            if nat_eq(a281, a282) {
                Some(v2)
            } else {
                None::<Adt25>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f77(v0: Adt15) -> Option<Bytes> {
    let m283 = v0;
    match m283 {
        Adt15::C0 => {
            None::<Bytes>
        }
        Adt15::C1 => {
            None::<Bytes>
        }
        Adt15::C2 => {
            Some(f1(Str::lit("match:option")))
        }
        Adt15::C3 => {
            Some(f1(Str::lit("match:ordering")))
        }
    }
}

pub fn f78(v0: Adt16) -> Bytes {
    let m284 = v0;
    match m284 {
        Adt16::C0 => {
            f1(Str::lit("match:option"))
        }
        Adt16::C1 => {
            f1(Str::lit("match:result"))
        }
        Adt16::C2 => {
            f1(Str::lit("match:result"))
        }
    }
}

pub fn f79(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m285 = v1;
    match m285 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a288 = {
                    let a286 = v3;
                    let a287 = v0;
                    append_bytes(a286, a287)
                };
                let a289 = v2;
                append_bytes(a288, a289)
            })
        }
    }
}

pub fn f80(v0: Adt27, v1: Adt27) -> bool {
    let a292 = match v0.clone() {
        Adt27::C0(h290, _, _, _) => {
            h290
        }
    };
    let a293 = match v1.clone() {
        Adt27::C0(h291, _, _, _) => {
            h291
        }
    };
    if equal(a292, a293) {
        let a296 = match v0.clone() {
            Adt27::C0(_, h294, _, _) => {
                h294
            }
        };
        let a297 = match v1.clone() {
            Adt27::C0(_, h295, _, _) => {
                h295
            }
        };
        if equal(a296, a297) {
            if f46(match v0.clone() {
                Adt27::C0(_, _, h298, _) => {
                    h298
                }
            }, match v1.clone() {
                Adt27::C0(_, _, h299, _) => {
                    h299
                }
            }) {
                f46(match v0 {
                    Adt27::C0(_, _, _, h300) => {
                        h300
                    }
                }, match v1 {
                    Adt27::C0(_, _, _, h301) => {
                        h301
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f81(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h302, _) = v1;
        h302
    };
    let v4: List<Bytes> = {
        let (_, h303) = v1;
        h303
    };
    let m304 = v3;
    if m304 {
        (true, List::cons(v2, v4))
    } else {
        let m305 = f84(v0.clone(), v2.clone());
        if m305 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f82(v0: Fn9, p309: List<Bytes>, p310: List<Bytes>) -> List<Bytes> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m306 = v2;
                match m306.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c307: Fn9 = v0.clone();
                            c307.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f84(v0: Bytes, v1: Bytes) -> bool {
    let a315 = {
        let a311 = v0;
        let a312 = v1;
        compare_bytes(a311, a312)
    };
    let a316 = {
        let a313 = Bytes::lit(&[]);
        let a314 = Bytes::lit(&[0x01]);
        compare_bytes(a313, a314)
    };
    equal(a315, a316)
}

pub fn run(p0: Adt0, p1: List<Bytes>) -> R<Result<(), Bytes>> {
    f0(p0, p1)
}
