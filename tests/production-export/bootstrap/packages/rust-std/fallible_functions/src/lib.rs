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
    C0,
    C1,
    C2,
    C3,
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
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
pub enum Adt2 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt3>),
    C1(u64),
    C2(u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt5>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(std::rc::Rc<Adt6>, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(std::rc::Rc<Adt5>, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(bool, List<u64>, std::rc::Rc<Adt0>, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(bool, List<u64>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(std::rc::Rc<Adt10>, std::rc::Rc<Adt8>),
}

impl Fn0 {
    pub fn apply(&self, p0: List<bool>) -> R<Option<List<bool>>> {
        match self {
            Fn0::F6(k0, k1) => f6((**k0).clone(), (**k1).clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: List<bool>) -> R<List<bool>> {
    let v8: Adt7 = Adt7::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3.clone()), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v5.clone()));
    let v9: u64 = f1(v5.clone())?;
    let v10: Adt0 = f2(f2(f2(Adt0::C14, v3), v4), v1);
    let v11: Adt8 = f3(v8.clone(), Adt8::C0(std::rc::Rc::new(v10), std::rc::Rc::new(Adt9::C0)), v5.clone(), 0u64, v7, v6.clone())?;
    let v12: Adt10 = f4(v8, v5, 0u64, v6)?;
    let (h1, _) = f5(Fn0::F6(std::rc::Rc::new(v12), std::rc::Rc::new(v11)), {
        let a2 = 1u64;
        let a3 = v9;
        nat_add(a2, a3)?
    }, f7(v9))?;
    Ok(h1)
}

pub fn f1(v0: Adt2) -> R<u64> {
    let m4 = v0;
    match m4 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r7) => {
            let v33 = (*r7).clone();
            let v34: u64 = f1(v33)?;
            let a5 = 1u64;
            let a6 = v34;
            nat_add(a5, a6)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0) -> Adt0 {
    let m8 = v1;
    match m8 {
        Adt0::C0 => {
            v0
        }
        Adt0::C1 => {
            v0
        }
        Adt0::C2 => {
            v0
        }
        Adt0::C3 => {
            v0
        }
        Adt0::C4(_) => {
            v0
        }
        Adt0::C5 => {
            v0
        }
        Adt0::C6 => {
            v0
        }
        Adt0::C7 => {
            v0
        }
        Adt0::C8(r9) => {
            let v3 = (*r9).clone();
            f2(v0, v3)
        }
        Adt0::C9(r10, r11) => {
            let v4 = (*r10).clone();
            let v5 = (*r11).clone();
            f2(f2(v0, v4), v5)
        }
        Adt0::C10(r12) => {
            let v6 = (*r12).clone();
            f2(v0, v6)
        }
        Adt0::C11(r13, r14) => {
            let v7 = (*r13).clone();
            let v8 = (*r14).clone();
            f2(f2(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r15, r16) => {
            let v10 = (*r15).clone();
            let v11 = (*r16).clone();
            let v12: Adt0 = f8(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f2(f2(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r17, r18) => {
            let v13 = (*r17).clone();
            let v14 = (*r18).clone();
            f2(f2(v0, v13), v14)
        }
    }
}

pub fn f3(v0: Adt7, p29: Adt8, p30: Adt2, p31: u64, v4: List<bool>, p32: Adt0) -> R<Adt8> {
    let m28 = (p29, (p30, (p31, p32)));
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m28;
                let m19 = v2;
                match m19 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C14(r26, r27) => {
                        let v34 = (*r26).clone();
                        let v35 = (*r27).clone();
                        let v37: bool = {
                            let m20 = f9(v4.clone(), v3);
                            match m20 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a21 = v36;
                                    bool_not(a21)
                                }
                            }
                        };
                        let v38: Adt8 = Adt8::C0(std::rc::Rc::new(f10(v0.clone(), match v1.clone() {
                            Adt8::C0(h22, _) => {
                                (*h22).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt8::C0(_, h23) => {
                                (*h23).clone()
                            }
                        }));
                        let v39: Adt8 = if v37 {
                            f11(v0.clone(), v38, v34, f12(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>((v39, (v35, ({
                            let a24 = v3;
                            let a25 = 1u64;
                            nat_add(a24, a25)?
                        }, f13(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
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

pub fn f4(v0: Adt7, v1: Adt2, v2: u64, v3: Adt0) -> R<Adt10> {
    let m33 = v1;
    match m33 {
        Adt2::C0(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C1(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C3(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C4(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C5(_, _, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C6(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C7(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C8(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C9(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C10(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C11(_) => {
            Ok(Adt10::C0)
        }
        Adt2::C12(_, _) => {
            Ok(Adt10::C0)
        }
        Adt2::C13 => {
            Ok(Adt10::C0)
        }
        Adt2::C14(r39, r40) => {
            let v32 = (*r39).clone();
            let v33 = (*r40).clone();
            let v34: Adt11 = f14(v0.clone(), Adt11::C0(false, List::<u64>::nil(), std::rc::Rc::new(Adt0::C14)), v32, f12(v3.clone()));
            Ok(Adt10::C1(match v34.clone() {
                Adt11::C0(h34, _, _) => {
                    h34
                }
            }, match v34.clone() {
                Adt11::C0(_, h35, _) => {
                    h35
                }
            }, std::rc::Rc::new(match v34 {
                Adt11::C0(_, _, h36) => {
                    (*h36).clone()
                }
            }), std::rc::Rc::new(f4(v0, v33, {
                let a37 = v2;
                let a38 = 1u64;
                nat_add(a37, a38)?
            }, f13(v3))?)))
        }
        Adt2::C15(_, _, _) => {
            Ok(Adt10::C0)
        }
    }
}

pub fn f5(v0: Fn0, p45: u64, p46: List<bool>) -> R<(List<bool>, bool)> {
    let m44 = (p45, p46);
    {
        let mut m44 = m44;
        loop {
            let step = {
                let (v1, v2) = m44;
                let m41 = v1;
                if m41 == 0 {
                    Err::<(u64, List<bool>), R<(List<bool>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m41 - 1;
                    let m43 = {
                        let c42: Fn0 = v0.clone();
                        c42.apply(v2.clone())?
                    };
                    match m43 {
                        None => {
                            Err::<(u64, List<bool>), R<(List<bool>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<bool>), R<(List<bool>, bool)>>((v3, v4))
                        }
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

pub fn f6(v0: Adt10, v1: Adt8, v2: List<bool>) -> R<Option<List<bool>>> {
    f15(v0, match v1 {
        Adt8::C0(_, h47) => {
            (*h47).clone()
        }
    }, v2)
}

pub fn f7(v0: u64) -> List<bool> {
    let m48 = v0;
    if m48 == 0 {
        List::<bool>::nil()
    } else {
        let v1 = m48 - 1;
        List::cons(false, f7(v1))
    }
}

pub fn f8(v0: Adt0, v1: Adt0) -> Adt0 {
    if f16(v0.clone(), v1.clone()) {
        v0
    } else {
        f17(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f9(p52: List<bool>, p53: u64) -> Option<bool> {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v0, v1) = m51;
                let m49 = v0;
                match m49.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m50 = v1;
                        if m50 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m50 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m51 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt7, v1: Adt0, v2: u64) -> Adt0 {
    f2(f18(v1, f19(v0.clone(), v2)), f20(v0, v2))
}

pub fn f11(v0: Adt7, v1: Adt8, v2: Adt2, v3: Adt0) -> R<Adt8> {
    let m54 = v2;
    match m54 {
        Adt2::C0(r65, _) => {
            let v4 = (*r65).clone();
            Ok(Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h55, _) => {
                    (*h55).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h56) => {
                    (*h56).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r66, r67, r68) => {
            let v8 = (*r66).clone();
            let v9 = (*r67).clone();
            let v10 = (*r68).clone();
            let v11: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h57, _) => {
                    (*h57).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h58) => {
                    (*h58).clone()
                }
            }));
            let v12: Adt8 = f11(v0.clone(), v11, v9, f12(v3.clone()))?;
            f11(v0, v12, v10, f13(v3))
        }
        Adt2::C3(r69, r70, r71) => {
            let v13 = (*r69).clone();
            let v14 = (*r70).clone();
            let v15 = (*r71).clone();
            let v16: Adt8 = f11(v0.clone(), v1, v13, f12(v3.clone()))?;
            let v17: Adt8 = f11(v0.clone(), v16, v14, f13(v3.clone()))?;
            f11(v0, v17, v15, f21(v3))
        }
        Adt2::C4(r72, r73, r74) => {
            let v18 = (*r72).clone();
            let v19 = (*r73).clone();
            let v20 = (*r74).clone();
            let v21: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h59, _) => {
                    (*h59).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h60) => {
                    (*h60).clone()
                }
            }));
            let v22: Adt8 = f11(v0.clone(), v21, v19, f12(v3.clone()))?;
            f11(v0, v22, v20, f13(v3))
        }
        Adt2::C5(_, r75, r76) => {
            let v24 = (*r75).clone();
            let v25 = (*r76).clone();
            let v26: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h61, _) => {
                    (*h61).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt8::C0(_, h62) => {
                    (*h62).clone()
                }
            }));
            f11(v0, v26, v25, f12(v3))
        }
        Adt2::C6(_, r77) => {
            let v28 = (*r77).clone();
            f11(v0, v1, v28, f12(v3))
        }
        Adt2::C7(v29, r78) => {
            let v30 = (*r78).clone();
            let v31: Adt0 = f19(v0.clone(), v29);
            let v32: u64 = f1(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f22(v31.clone(), v32)), std::rc::Rc::new(f20(v0.clone(), v29)));
            let v34: Adt8 = Adt8::C0(std::rc::Rc::new(f2(match v1.clone() {
                Adt8::C0(h63, _) => {
                    (*h63).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f23(match v1 {
                Adt8::C0(_, h64) => {
                    (*h64).clone()
                }
            }, v33, v29, f24(v31, v32))));
            f11(v0, v34, v30, f12(v3))
        }
        Adt2::C8(r79, r80) => {
            let v35 = (*r79).clone();
            let v36 = (*r80).clone();
            let v37: Adt8 = f11(v0.clone(), v1, v35, f12(v3.clone()))?;
            f11(v0, v37, v36, f13(v3))
        }
        Adt2::C9(_, r81) => {
            let v39 = (*r81).clone();
            f11(v0, v1, v39, f12(v3))
        }
        Adt2::C10(r82) => {
            let v40 = (*r82).clone();
            f11(v0, v1, v40, f12(v3))
        }
        Adt2::C11(r83) => {
            let v41 = (*r83).clone();
            f11(v0, v1, v41, f12(v3))
        }
        Adt2::C12(r84, _) => {
            let v42 = (*r84).clone();
            f11(v0, v1, v42, f12(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r85, r86) => {
            let v44 = (*r85).clone();
            let v45 = (*r86).clone();
            let v46: Adt8 = f11(v0.clone(), v1, v44, f12(v3.clone()))?;
            f11(v0, v46, v45, f13(v3))
        }
        Adt2::C15(_, _, r87) => {
            let v49 = (*r87).clone();
            f11(v0, v1, v49, f12(v3))
        }
    }
}

pub fn f12(v0: Adt0) -> Adt0 {
    f25(f26(v0))
}

pub fn f13(v0: Adt0) -> Adt0 {
    f25(f26(f26(v0)))
}

pub fn f14(v0: Adt7, v1: Adt11, v2: Adt2, v3: Adt0) -> Adt11 {
    let m88 = v2;
    match m88 {
        Adt2::C0(_, _) => {
            v1
        }
        Adt2::C1(_) => {
            v1
        }
        Adt2::C2(_, _, r103, r104) => {
            let v9 = (*r103).clone();
            let v10 = (*r104).clone();
            let v11: Adt11 = f14(v0.clone(), v1, v9, f12(v3.clone()));
            f14(v0, v11, v10, f13(v3))
        }
        Adt2::C3(r105, r106, r107) => {
            let v12 = (*r105).clone();
            let v13 = (*r106).clone();
            let v14 = (*r107).clone();
            let v15: Adt11 = f14(v0.clone(), v1, v12, f12(v3.clone()));
            let v16: Adt11 = f14(v0.clone(), v15, v13, f13(v3.clone()));
            f14(v0, v16, v14, f21(v3))
        }
        Adt2::C4(_, r108, r109) => {
            let v18 = (*r108).clone();
            let v19 = (*r109).clone();
            let v20: Adt11 = f14(v0.clone(), v1, v18, f12(v3.clone()));
            f14(v0, v20, v19, f13(v3))
        }
        Adt2::C5(r110, _, r111) => {
            let v21 = (*r110).clone();
            let v23 = (*r111).clone();
            let v24: Adt11 = f14(v0, v1, v23, f12(v3));
            let m89 = v21;
            match m89 {
                Adt5::C0 => {
                    v24
                }
                Adt5::C1 => {
                    v24
                }
                Adt5::C2 => {
                    v24
                }
                Adt5::C3 => {
                    v24
                }
                Adt5::C4 => {
                    v24
                }
                Adt5::C5 => {
                    v24
                }
                Adt5::C6 => {
                    v24
                }
                Adt5::C7 => {
                    Adt11::C0(true, match v24.clone() {
                        Adt11::C0(_, h90, _) => {
                            h90
                        }
                    }, std::rc::Rc::new(match v24 {
                        Adt11::C0(_, _, h91) => {
                            (*h91).clone()
                        }
                    }))
                }
                Adt5::C8 => {
                    v24
                }
                Adt5::C9 => {
                    v24
                }
                Adt5::C10 => {
                    v24
                }
                Adt5::C11 => {
                    v24
                }
                Adt5::C12 => {
                    v24
                }
                Adt5::C13 => {
                    v24
                }
                Adt5::C14 => {
                    v24
                }
                Adt5::C15(_) => {
                    v24
                }
            }
        }
        Adt2::C6(v26, r112) => {
            let v27 = (*r112).clone();
            let v28: Adt11 = f14(v0, v1, v27, f12(v3));
            Adt11::C0(match v28.clone() {
                Adt11::C0(h92, _, _) => {
                    h92
                }
            }, if f27(match v28.clone() {
                Adt11::C0(_, h93, _) => {
                    h93
                }
            }, v26) {
                match v28.clone() {
                    Adt11::C0(_, h94, _) => {
                        h94
                    }
                }
            } else {
                List::cons(v26, match v28.clone() {
                    Adt11::C0(_, h95, _) => {
                        h95
                    }
                })
            }, std::rc::Rc::new(match v28 {
                Adt11::C0(_, _, h96) => {
                    (*h96).clone()
                }
            }))
        }
        Adt2::C7(_, r113) => {
            let v30 = (*r113).clone();
            f14(v0, v1, v30, f12(v3))
        }
        Adt2::C8(r114, r115) => {
            let v31 = (*r114).clone();
            let v32 = (*r115).clone();
            let v33: Adt11 = f14(v0.clone(), v1, v31, f12(v3.clone()));
            let v34: Adt11 = f14(v0, v33, v32, f13(v3.clone()));
            Adt11::C0(match v34.clone() {
                Adt11::C0(h97, _, _) => {
                    h97
                }
            }, match v34.clone() {
                Adt11::C0(_, h98, _) => {
                    h98
                }
            }, std::rc::Rc::new(f8(match v34 {
                Adt11::C0(_, _, h99) => {
                    (*h99).clone()
                }
            }, f28(v3))))
        }
        Adt2::C9(r116, r117) => {
            let v35 = (*r116).clone();
            let v36 = (*r117).clone();
            let v37: Adt11 = f14(v0, v1, v36, f12(v3));
            Adt11::C0(if match v37.clone() {
                Adt11::C0(h100, _, _) => {
                    h100
                }
            } {
                true
            } else {
                f29(v35)
            }, match v37.clone() {
                Adt11::C0(_, h101, _) => {
                    h101
                }
            }, std::rc::Rc::new(match v37 {
                Adt11::C0(_, _, h102) => {
                    (*h102).clone()
                }
            }))
        }
        Adt2::C10(r118) => {
            let v38 = (*r118).clone();
            f14(v0, v1, v38, f12(v3))
        }
        Adt2::C11(r119) => {
            let v39 = (*r119).clone();
            f14(v0, v1, v39, f12(v3))
        }
        Adt2::C12(r120, _) => {
            let v40 = (*r120).clone();
            f14(v0, v1, v40, f12(v3))
        }
        Adt2::C13 => {
            v1
        }
        Adt2::C14(r121, r122) => {
            let v42 = (*r121).clone();
            let v43 = (*r122).clone();
            let v44: Adt11 = f14(v0.clone(), v1, v42, f12(v3.clone()));
            f14(v0, v44, v43, f13(v3))
        }
        Adt2::C15(_, _, r123) => {
            let v47 = (*r123).clone();
            f14(v0, v1, v47, f12(v3))
        }
    }
}

pub fn f15(v0: Adt10, v1: Adt9, v2: List<bool>) -> R<Option<List<bool>>> {
    let v3: List<bool> = f30(v0, v1, v2.clone(), 0u64)?;
    if f31(v3.clone(), v2) {
        Ok(None::<List<bool>>)
    } else {
        Ok(Some(v3))
    }
}

pub fn f16(p128: Adt0, v1: Adt0) -> bool {
    let m127 = p128;
    {
        let mut m127 = m127;
        loop {
            let step = {
                let v0 = m127;
                let m124 = v0;
                match m124 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r125, r126) => {
                        let v12 = (*r125).clone();
                        let v13 = (*r126).clone();
                        if f32(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt0, v1: Adt0) -> Adt0 {
    let m129 = v0;
    match m129 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(_) => {
            v1
        }
        Adt0::C9(_, _) => {
            v1
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(_, _) => {
            v1
        }
        Adt0::C12(_) => {
            v1
        }
        Adt0::C13(_, _) => {
            v1
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(r130, r131) => {
            let v12 = (*r130).clone();
            let v13 = (*r131).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f17(v13, v1)))
        }
    }
}

pub fn f18(p136: Adt0, p137: Adt0) -> Adt0 {
    let m135 = (p136, p137);
    {
        let mut m135 = m135;
        loop {
            let step = {
                let (v0, v1) = m135;
                let m132 = v1;
                match m132 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r133, r134) => {
                        let v2 = (*r133).clone();
                        let v3 = (*r134).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f2(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt7, v1: u64) -> Adt0 {
    let m138 = f33(v0, v1);
    match m138 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f20(v0: Adt7, v1: u64) -> Adt0 {
    let m140 = f34(match v0 {
        Adt7::C0(_, _, _, _, h139, _) => {
            (*h139).clone()
        }
    }, v1);
    match m140 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f21(v0: Adt0) -> Adt0 {
    f25(f26(f26(f26(v0))))
}

pub fn f22(p146: Adt0, p147: u64) -> Adt0 {
    let m145 = (p146, p147);
    {
        let mut m145 = m145;
        loop {
            let step = {
                let (v0, v1) = m145;
                let m141 = v0;
                match m141 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C15(r143, r144) => {
                        let v12 = (*r143).clone();
                        let v13 = (*r144).clone();
                        let m142 = v1;
                        if m142 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m142 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f35(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f24(v0: Adt0, v1: u64) -> Adt0 {
    let m148 = v0;
    match m148 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(_) => {
            Adt0::C14
        }
        Adt0::C13(_, _) => {
            Adt0::C14
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(r150, r151) => {
            let v12 = (*r150).clone();
            let v13 = (*r151).clone();
            let m149 = v1;
            if m149 == 0 {
                Adt0::C14
            } else {
                let v14 = m149 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f24(v13, v14)))
            }
        }
    }
}

pub fn f25(v0: Adt0) -> Adt0 {
    let m152 = f36(v0);
    match m152 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f26(v0: Adt0) -> Adt0 {
    let m153 = f37(v0);
    match m153 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f27(p158: List<u64>, v1: u64) -> bool {
    let m157 = p158;
    {
        let mut m157 = m157;
        loop {
            let step = {
                let v0 = m157;
                let m154 = v0;
                match m154.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a155 = v2;
                        let a156 = v1;
                        if nat_eq(a155, a156) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt0) -> Adt0 {
    f25(v0)
}

pub fn f29(v0: Adt6) -> bool {
    let m159 = v0;
    match m159 {
        Adt6::C0 => {
            true
        }
        Adt6::C1 => {
            false
        }
        Adt6::C2 => {
            true
        }
        Adt6::C3 => {
            false
        }
        Adt6::C4 => {
            false
        }
        Adt6::C5 => {
            false
        }
        Adt6::C6 => {
            false
        }
        Adt6::C7 => {
            false
        }
        Adt6::C8 => {
            true
        }
        Adt6::C9 => {
            true
        }
        Adt6::C10 => {
            true
        }
        Adt6::C11 => {
            true
        }
        Adt6::C12 => {
            true
        }
        Adt6::C13 => {
            false
        }
        Adt6::C14 => {
            false
        }
        Adt6::C15 => {
            false
        }
        Adt6::C16 => {
            false
        }
        Adt6::C17 => {
            false
        }
        Adt6::C18 => {
            false
        }
        Adt6::C19 => {
            false
        }
        Adt6::C20 => {
            false
        }
        Adt6::C21 => {
            false
        }
        Adt6::C22 => {
            false
        }
        Adt6::C23 => {
            false
        }
        Adt6::C24 => {
            false
        }
        Adt6::C25 => {
            false
        }
        Adt6::C26 => {
            false
        }
        Adt6::C27 => {
            false
        }
        Adt6::C28 => {
            false
        }
        Adt6::C29 => {
            false
        }
        Adt6::C30 => {
            false
        }
        Adt6::C31 => {
            false
        }
        Adt6::C32 => {
            false
        }
        Adt6::C33 => {
            false
        }
        Adt6::C34 => {
            false
        }
        Adt6::C35 => {
            false
        }
        Adt6::C36 => {
            false
        }
        Adt6::C37 => {
            false
        }
        Adt6::C38 => {
            false
        }
        Adt6::C39 => {
            false
        }
        Adt6::C40(_) => {
            false
        }
        Adt6::C41(r162) => {
            let v2 = (*r162).clone();
            let a160 = f38(v2);
            let a161 = 3u64;
            nat_eq(a160, a161)
        }
    }
}

pub fn f30(v0: Adt10, v1: Adt9, v2: List<bool>, v3: u64) -> R<List<bool>> {
    let m163 = v0;
    match m163 {
        Adt10::C0 => {
            Ok(List::<bool>::nil())
        }
        Adt10::C1(v4, v5, r166, r167) => {
            let v6 = (*r166).clone();
            let v7 = (*r167).clone();
            Ok(List::cons(if if v4 {
                true
            } else {
                if f39(v2.clone(), v3) {
                    true
                } else {
                    f40(v5, v2.clone())
                }
            } {
                true
            } else {
                f41(v6, v1.clone(), v2.clone())
            }, f30(v7, v1, v2, {
                let a164 = v3;
                let a165 = 1u64;
                nat_add(a164, a165)?
            })?))
        }
    }
}

pub fn f31(p174: List<bool>, p175: List<bool>) -> bool {
    let m173 = (p174, p175);
    {
        let mut m173 = m173;
        loop {
            let step = {
                let (v0, v1) = m173;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        let m169 = v1;
                        match m169.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m170 = v1;
                        match m170.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a171 = v4;
                                let a172 = v6;
                                if equal(a171, a172) {
                                    Ok::<(List<bool>, List<bool>), bool>((v5, v7))
                                } else {
                                    Err::<(List<bool>, List<bool>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Adt0, v1: Adt0) -> bool {
    let m176 = v0;
    match m176 {
        Adt0::C0 => {
            let a177 = f38(v1);
            let a178 = 0u64;
            nat_eq(a177, a178)
        }
        Adt0::C1 => {
            let a179 = f38(v1);
            let a180 = 1u64;
            nat_eq(a179, a180)
        }
        Adt0::C2 => {
            let a181 = f38(v1);
            let a182 = 2u64;
            nat_eq(a181, a182)
        }
        Adt0::C3 => {
            let a183 = f38(v1);
            let a184 = 3u64;
            nat_eq(a183, a184)
        }
        Adt0::C4(v2) => {
            let a193 = f38(v1.clone());
            let a194 = 4u64;
            if nat_eq(a193, a194) {
                let a195 = f42(v2);
                let a196 = f42(f43(v1));
                nat_eq(a195, a196)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a185 = f38(v1);
            let a186 = 5u64;
            nat_eq(a185, a186)
        }
        Adt0::C6 => {
            let a187 = f38(v1);
            let a188 = 6u64;
            nat_eq(a187, a188)
        }
        Adt0::C7 => {
            let a189 = f38(v1);
            let a190 = 7u64;
            nat_eq(a189, a190)
        }
        Adt0::C8(r213) => {
            let v3 = (*r213).clone();
            let v4: bool = f32(v3, f25(v1.clone()));
            let a197 = f38(v1);
            let a198 = 8u64;
            if nat_eq(a197, a198) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r214, r215) => {
            let v5 = (*r214).clone();
            let v6 = (*r215).clone();
            let v7: bool = f32(v5, f25(v1.clone()));
            let v8: bool = f32(v6, f26(v1.clone()));
            let a199 = f38(v1);
            let a200 = 9u64;
            if nat_eq(a199, a200) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r216) => {
            let v9 = (*r216).clone();
            let v10: bool = f32(v9, f25(v1.clone()));
            let a201 = f38(v1);
            let a202 = 10u64;
            if nat_eq(a201, a202) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r217, r218) => {
            let v11 = (*r217).clone();
            let v12 = (*r218).clone();
            let v13: bool = f32(v11, f25(v1.clone()));
            let v14: bool = f32(v12, f26(v1.clone()));
            let a203 = f38(v1);
            let a204 = 11u64;
            if nat_eq(a203, a204) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a205 = f38(v1.clone());
            let a206 = 12u64;
            if nat_eq(a205, a206) {
                let a207 = v15;
                let a208 = f44(v1);
                nat_eq(a207, a208)
            } else {
                false
            }
        }
        Adt0::C13(r219, r220) => {
            let v16 = (*r219).clone();
            let v17 = (*r220).clone();
            let v18: bool = f32(v16, f25(v1.clone()));
            let v19: bool = f32(v17, f26(v1.clone()));
            let a209 = f38(v1);
            let a210 = 13u64;
            if nat_eq(a209, a210) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a191 = f38(v1);
            let a192 = 14u64;
            nat_eq(a191, a192)
        }
        Adt0::C15(r221, r222) => {
            let v20 = (*r221).clone();
            let v21 = (*r222).clone();
            let v22: bool = f32(v20, f25(v1.clone()));
            let v23: bool = f32(v21, f26(v1.clone()));
            let a211 = f38(v1);
            let a212 = 15u64;
            if nat_eq(a211, a212) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f33(v0: Adt7, v1: u64) -> Option<Adt0> {
    f34(match v0 {
        Adt7::C0(_, _, _, h223, _, _) => {
            (*h223).clone()
        }
    }, v1)
}

pub fn f34(p229: Adt0, p230: u64) -> Option<Adt0> {
    let m228 = (p229, p230);
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v0, v1) = m228;
                let m224 = v0;
                match m224 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r226, r227) => {
                        let v12 = (*r226).clone();
                        let v13 = (*r227).clone();
                        let m225 = v1;
                        if m225 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m225 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
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

pub fn f35(p237: Adt9, v1: Adt0, v2: u64) -> bool {
    let m236 = p237;
    {
        let mut m236 = m236;
        loop {
            let step = {
                let v0 = m236;
                let m231 = v0;
                match m231 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r234, v4, _, r235) => {
                        let v3 = (*r234).clone();
                        let v6 = (*r235).clone();
                        let a232 = v4;
                        let a233 = v2;
                        if nat_eq(a232, a233) {
                            if f32(v3, v1.clone()) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt0) -> Option<Adt0> {
    let m238 = v0;
    match m238 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r239) => {
            let v2 = (*r239).clone();
            Some(v2)
        }
        Adt0::C9(r240, _) => {
            let v3 = (*r240).clone();
            Some(v3)
        }
        Adt0::C10(r241) => {
            let v5 = (*r241).clone();
            Some(v5)
        }
        Adt0::C11(r242, _) => {
            let v6 = (*r242).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r243, _) => {
            let v9 = (*r243).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r244, _) => {
            let v11 = (*r244).clone();
            Some(v11)
        }
    }
}

pub fn f37(v0: Adt0) -> Option<Adt0> {
    let m245 = v0;
    match m245 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r246) => {
            let v4 = (*r246).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r247) => {
            let v7 = (*r247).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r248) => {
            let v10 = (*r248).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r249) => {
            let v12 = (*r249).clone();
            Some(v12)
        }
    }
}

pub fn f38(v0: Adt0) -> u64 {
    let m250 = v0;
    match m250 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4(_) => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f39(v0: List<bool>, v1: u64) -> bool {
    let m251 = f9(v0, v1);
    match m251 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(p254: List<u64>, v1: List<bool>) -> bool {
    let m253 = p254;
    {
        let mut m253 = m253;
        loop {
            let step = {
                let v0 = m253;
                let m252 = v0;
                match m252.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f39(v1.clone(), v2) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(p259: Adt0, v1: Adt9, v2: List<bool>) -> bool {
    let m258 = p259;
    {
        let mut m258 = m258;
        loop {
            let step = {
                let v0 = m258;
                let m255 = v0;
                match m255 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r256, r257) => {
                        let v13 = (*r256).clone();
                        let v14 = (*r257).clone();
                        if f45(v1.clone(), v13, v2.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v14)
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

pub fn f42(v0: Adt1) -> u64 {
    let m260 = v0;
    match m260 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f43(v0: Adt0) -> Adt1 {
    let m261 = v0;
    match m261 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f44(v0: Adt0) -> u64 {
    let m262 = v0;
    match m262 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f45(p267: Adt9, v1: Adt0, v2: List<bool>) -> bool {
    let m266 = p267;
    {
        let mut m266 = m266;
        loop {
            let step = {
                let v0 = m266;
                let m263 = v0;
                match m263 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r264, v4, _, r265) => {
                        let v3 = (*r264).clone();
                        let v6 = (*r265).clone();
                        if f32(v3, v1.clone()) {
                            if f39(v2.clone(), v4) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
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

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: List<bool>) -> R<List<bool>> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7)
}
