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
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt8>, std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(std::rc::Rc<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(std::rc::Rc<Adt12>, std::rc::Rc<Adt15>),
    C1(Adt16, std::rc::Rc<Adt17>, std::rc::Rc<Adt15>),
    C2(Adt18, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>),
    C3(std::rc::Rc<Adt19>, std::rc::Rc<Adt11>, bool, std::rc::Rc<Adt15>),
    C4(std::rc::Rc<Adt17>, std::rc::Rc<Adt11>, bool, std::rc::Rc<Adt15>),
    C5(std::rc::Rc<Adt21>, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>),
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>),
    C7(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>),
    C8(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>),
    C9(std::rc::Rc<Adt11>),
    C10(Adt27, std::rc::Rc<Adt17>, std::rc::Rc<Adt15>),
    C11(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C12,
    C13(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C14(std::rc::Rc<Adt28>, bool, std::rc::Rc<Adt22>, std::rc::Rc<Adt11>, std::rc::Rc<Adt15>, std::rc::Rc<Adt11>),
    C15(std::rc::Rc<Adt28>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C16(std::rc::Rc<Adt17>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt13, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt14),
    C8,
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt17 {
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
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64),
    C1(std::rc::Rc<Adt20>),
    C2(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(std::rc::Rc<Adt22>),
    C1,
    C2(std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
    C3(std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt22>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt23),
    C1(Adt13),
    C2(Adt24, std::rc::Rc<Adt22>),
    C3(Adt25, std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
    C4(Adt26, u64),
    C5,
    C6(std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
}

#[derive(Clone)]
pub enum Adt23 {
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
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt29),
    C1(std::rc::Rc<Adt17>),
    C2(std::rc::Rc<Adt28>),
    C3(Adt30, std::rc::Rc<Adt28>),
    C4(u64, u64, std::rc::Rc<Adt28>),
    C5,
    C6(std::rc::Rc<Adt28>, std::rc::Rc<Adt28>),
    C7(Adt14),
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(std::rc::Rc<Adt31>, std::rc::Rc<Adt0>, std::rc::Rc<Adt8>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, List<bool>, std::rc::Rc<Adt9>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(List<(u64, u64)>, List<Bytes>, List<(Adt17, u64)>, u64),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt21>, bool),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(std::rc::Rc<Adt11>, std::rc::Rc<Adt28>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F119(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F119(k0) => f119(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F166(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn1::F166(k0) => f166(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: Adt0, v8: List<bool>, v9: List<bool>, v10: Adt7, v11: u64) -> R<Adt10> {
    let v12: Adt31 = Adt31::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3.clone()), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v5.clone()));
    let v13: Adt32 = Adt32::C0(std::rc::Rc::new(v12), std::rc::Rc::new(match v10.clone() {
        Adt7::C0(h1, _, _, _) => {
            (*h1).clone()
        }
    }), std::rc::Rc::new(match v10.clone() {
        Adt7::C0(_, h2, _, _) => {
            (*h2).clone()
        }
    }), std::rc::Rc::new(v1), std::rc::Rc::new(v3), std::rc::Rc::new(v4), v9, std::rc::Rc::new(match v10.clone() {
        Adt7::C0(_, _, _, h3) => {
            (*h3).clone()
        }
    }), std::rc::Rc::new(match v10 {
        Adt7::C0(_, _, h4, _) => {
            (*h4).clone()
        }
    }));
    f1(v13, v5, v6, v7, v8, 0u64, v11)
}

pub fn f1(v0: Adt32, v1: Adt2, v2: Adt0, v3: Adt0, v4: List<bool>, v5: u64, v6: u64) -> R<Adt10> {
    let m5 = v1;
    match m5 {
        Adt2::C0(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C1(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C2(_, _, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C3(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C4(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C5(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C6(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C7(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C8(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C9(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C10(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C11(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C12(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C13 => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C14(r18, r19) => {
            let v35 = (*r18).clone();
            let v36 = (*r19).clone();
            if f3(v4.clone(), v5) {
                let v37: Adt10 = f1(v0, v36, f4(v2), f4(v3), v4, {
                    let a6 = v5;
                    let a7 = 1u64;
                    nat_add(a6, a7)?
                }, v6)?;
                Ok(f2(Adt11::C13(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(match v37.clone() {
                    Adt10::C0(h8, _) => {
                        (*h8).clone()
                    }
                })), match v37 {
                    Adt10::C0(_, h9) => {
                        h9
                    }
                }))
            } else {
                let v38: List<(u64, Adt0)> = f5(f6(match v0.clone() {
                    Adt32::C0(h10, _, _, _, _, _, _, _, _) => {
                        (*h10).clone()
                    }
                }, v5), f7(match v0.clone() {
                    Adt32::C0(h11, _, _, _, _, _, _, _, _) => {
                        (*h11).clone()
                    }
                }, v5), List::<(u64, Adt0)>::nil());
                let v39: Adt10 = f8(v0.clone(), v35.clone(), f9(v2.clone()), f9(v3.clone()), v38, Adt0::C14, 0u64, f10(v35), v6)?;
                let v40: Adt10 = f1(v0, v36, f4(v2), f4(v3), v4, {
                    let a12 = v5;
                    let a13 = 1u64;
                    nat_add(a12, a13)?
                }, match v39.clone() {
                    Adt10::C0(_, h14) => {
                        h14
                    }
                })?;
                Ok(f2(Adt11::C13(std::rc::Rc::new(f11(match v39 {
                    Adt10::C0(h15, _) => {
                        (*h15).clone()
                    }
                })), std::rc::Rc::new(match v40.clone() {
                    Adt10::C0(h16, _) => {
                        (*h16).clone()
                    }
                })), match v40 {
                    Adt10::C0(_, h17) => {
                        h17
                    }
                }))
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
    }
}

pub fn f2(v0: Adt11, v1: u64) -> Adt10 {
    Adt10::C0(std::rc::Rc::new(v0), v1)
}

pub fn f3(v0: List<bool>, v1: u64) -> bool {
    let m20 = f12(v0, v1);
    match m20 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f4(v0: Adt0) -> Adt0 {
    f13(f14(f14(v0)))
}

pub fn f5(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m21 = v0;
    match m21.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f5(v4, f14(v1.clone()), f15(v3, f13(v1.clone()), v2.clone()));
            if f16(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f6(v0: Adt31, v1: u64) -> List<u64> {
    let m23 = f17(match v0 {
        Adt31::C0(_, _, h22, _, _, _) => {
            h22
        }
    }, v1);
    match m23 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f7(v0: Adt31, v1: u64) -> Adt0 {
    let m24 = f18(v0, v1);
    match m24 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f8(v0: Adt32, v1: Adt2, v2: Adt0, v3: Adt0, v4: List<(u64, Adt0)>, v5: Adt0, v6: u64, v7: Adt15, v8: u64) -> R<Adt10> {
    let m25 = v1;
    match m25 {
        Adt2::C0(r152, r153) => {
            let v9 = (*r152).clone();
            let v10 = (*r153).clone();
            Ok(f2(f19(match v0.clone() {
                Adt32::C0(_, h26, _, _, _, _, _, _, _) => {
                    (*h26).clone()
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h27, _, _, _, _, _, _) => {
                    (*h27).clone()
                }
            }, match v0 {
                Adt32::C0(_, _, _, h28, _, _, _, _, _) => {
                    (*h28).clone()
                }
            }, Adt0::C14, 0u64, v10, v9)?, v8))
        }
        Adt2::C1(v11) => {
            let v12: Adt0 = f20(v4, v11);
            Ok(f2(if f21(v12.clone(), Adt0::C0) {
                Adt11::C0(std::rc::Rc::new(Adt12::C0), std::rc::Rc::new(v7))
            } else {
                Adt11::C1(if f22(v12) {
                    Adt16::C2
                } else {
                    Adt16::C1
                }, std::rc::Rc::new(Adt17::C0(v11)), std::rc::Rc::new(v7))
            }, v8))
        }
        Adt2::C2(v13, r154, r155, r156) => {
            let v14 = (*r154).clone();
            let v15 = (*r155).clone();
            let v16 = (*r156).clone();
            let v17: Adt10 = f8(v0.clone(), v15.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v15), v8)?;
            let v18: Adt10 = f8(v0.clone(), v16.clone(), f4(v2), f4(v3), f15(v13, v14.clone(), v4), Adt0::C14, 0u64, f10(v16.clone()), match v17.clone() {
                Adt10::C0(_, h29) => {
                    h29
                }
            })?;
            let v19: Adt11 = f11(match v18.clone() {
                Adt10::C0(h30, _) => {
                    (*h30).clone()
                }
            });
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(f23(v13, v14.clone(), v16)), true, std::rc::Rc::new(f24(match v0 {
                Adt32::C0(_, h31, _, _, _, _, _, _, _) => {
                    (*h31).clone()
                }
            }, v14)?), std::rc::Rc::new(match v17 {
                Adt10::C0(h32, _) => {
                    (*h32).clone()
                }
            }), std::rc::Rc::new(v7), std::rc::Rc::new(f25(v19.clone())))), std::rc::Rc::new(f26(v19))))), match v18 {
                Adt10::C0(_, h33) => {
                    h33
                }
            }))
        }
        Adt2::C3(r157, r158, r159) => {
            let v20 = (*r157).clone();
            let v21 = (*r158).clone();
            let v22 = (*r159).clone();
            let v23: Adt10 = f8(v0.clone(), v20.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v20), v8)?;
            let v24: Adt11 = f11(match v23.clone() {
                Adt10::C0(h34, _) => {
                    (*h34).clone()
                }
            });
            let v25: Adt10 = f8(v0.clone(), v21.clone(), f4(v2.clone()), f4(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v21), match v23 {
                Adt10::C0(_, h35) => {
                    h35
                }
            })?;
            let v26: Adt10 = f8(v0, v22.clone(), f27(v2), f27(v3), v4, Adt0::C14, 0u64, f10(v22), match v25.clone() {
                Adt10::C0(_, h36) => {
                    h36
                }
            })?;
            let v27: Adt10 = f28(f26(v24.clone()), f11(match v25 {
                Adt10::C0(h37, _) => {
                    (*h37).clone()
                }
            }), f11(match v26.clone() {
                Adt10::C0(h38, _) => {
                    (*h38).clone()
                }
            }), v7.clone(), match v26 {
                Adt10::C0(_, h39) => {
                    h39
                }
            })?;
            let v28: Adt11 = f29(Adt11::C11(std::rc::Rc::new(f25(v24)), std::rc::Rc::new(Adt11::C0(std::rc::Rc::new(Adt12::C0), std::rc::Rc::new(v7)))), match v27.clone() {
                Adt10::C0(h40, _) => {
                    (*h40).clone()
                }
            });
            Ok(f2(if f30(f25(v28.clone())) {
                f26(v28)
            } else {
                Adt11::C9(std::rc::Rc::new(v28))
            }, match v27 {
                Adt10::C0(_, h41) => {
                    h41
                }
            }))
        }
        Adt2::C4(_, r160, r161) => {
            let v30 = (*r160).clone();
            let v31 = (*r161).clone();
            let v32: Adt0 = f31(v2.clone());
            let v33: Adt10 = f8(v0.clone(), v30.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v30), v8)?;
            if f21(v32.clone(), Adt0::C0) {
                let v34: Adt10 = f8(v0, v31.clone(), f4(v2), f4(v3), v4, Adt0::C14, 3u64, f10(v31), match v33.clone() {
                    Adt10::C0(_, h42) => {
                        h42
                    }
                })?;
                let v35: Adt11 = f11(match v34.clone() {
                    Adt10::C0(h43, _) => {
                        (*h43).clone()
                    }
                });
                Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C0(Adt29::C1)), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(match v33 {
                    Adt10::C0(h44, _) => {
                        (*h44).clone()
                    }
                }), std::rc::Rc::new(f33(Adt5::C11)), std::rc::Rc::new(f25(v35.clone())))), std::rc::Rc::new(f26(v35))))), match v34 {
                    Adt10::C0(_, h45) => {
                        h45
                    }
                }))
            } else {
                let v36: Adt17 = Adt17::C1({
                    let a47 = match v33.clone() {
                        Adt10::C0(_, h46) => {
                            h46
                        }
                    };
                    let a48 = 1u64;
                    nat_add(a47, a48)?
                });
                let v37: Adt10 = f8(v0.clone(), v31.clone(), f4(v2), f4(v3), v4, v32.clone(), 2u64, f10(v31.clone()), {
                    let a50 = match v33.clone() {
                        Adt10::C0(_, h49) => {
                            h49
                        }
                    };
                    let a51 = 1u64;
                    nat_add(a50, a51)?
                })?;
                let v38: Adt10 = f34(v0, v36.clone(), v32, v31, match v37.clone() {
                    Adt10::C0(h52, _) => {
                        (*h52).clone()
                    }
                }, match v37 {
                    Adt10::C0(_, h53) => {
                        h53
                    }
                })?;
                if f35(match v38.clone() {
                    Adt10::C0(h54, _) => {
                        (*h54).clone()
                    }
                }, v36.clone()) {
                    Ok(f2(match v33 {
                        Adt10::C0(h55, _) => {
                            (*h55).clone()
                        }
                    }, match v38 {
                        Adt10::C0(_, h56) => {
                            h56
                        }
                    }))
                } else {
                    let v39: Adt11 = f11(match v38.clone() {
                        Adt10::C0(h57, _) => {
                            (*h57).clone()
                        }
                    });
                    Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v36))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(match v33 {
                        Adt10::C0(h58, _) => {
                            (*h58).clone()
                        }
                    }), std::rc::Rc::new(v7), std::rc::Rc::new(f25(v39.clone())))), std::rc::Rc::new(f26(v39))))), match v38 {
                        Adt10::C0(_, h59) => {
                            h59
                        }
                    }))
                }
            }
        }
        Adt2::C5(r162, r163, r164) => {
            let v40 = (*r162).clone();
            let v41 = (*r163).clone();
            let v42 = (*r164).clone();
            let v43: Adt10 = f8(v0.clone(), v42.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v42), v8)?;
            f36(match v0.clone() {
                Adt32::C0(_, h60, _, _, _, _, _, _, _) => {
                    (*h60).clone()
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h61, _, _, _, _, _, _) => {
                    (*h61).clone()
                }
            }, match v0 {
                Adt32::C0(_, _, _, h62, _, _, _, _, _) => {
                    (*h62).clone()
                }
            }, v40.clone(), v41, match v43.clone() {
                Adt10::C0(h63, _) => {
                    (*h63).clone()
                }
            }, f33(v40), match v43 {
                Adt10::C0(_, h64) => {
                    h64
                }
            })
        }
        Adt2::C6(v44, r165) => {
            let v45 = (*r165).clone();
            let v46: Adt10 = f8(v0.clone(), v45.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v45), v8)?;
            let v47: Adt10 = f37(match v46.clone() {
                Adt10::C0(h65, _) => {
                    (*h65).clone()
                }
            }, f38(v0.clone(), v44), v7.clone(), match v46 {
                Adt10::C0(_, h66) => {
                    h66
                }
            })?;
            Ok(f2(f39(f25(match v47.clone() {
                Adt10::C0(h67, _) => {
                    (*h67).clone()
                }
            }), Adt11::C3(std::rc::Rc::new(Adt19::C0(v44)), std::rc::Rc::new(f26(match v47.clone() {
                Adt10::C0(h68, _) => {
                    (*h68).clone()
                }
            })), f3(match v0 {
                Adt32::C0(_, _, _, _, _, _, h69, _, _) => {
                    h69
                }
            }, v44), std::rc::Rc::new(v7))), match v47 {
                Adt10::C0(_, h70) => {
                    h70
                }
            }))
        }
        Adt2::C7(v48, r166) => {
            let v49 = (*r166).clone();
            let v50: Adt10 = f8(v0.clone(), v49.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v49.clone()), v8)?;
            let v51: u64 = f40(v49)?;
            let v52: Adt0 = f41(f38(v0.clone(), v48), v51);
            let v53: Adt0 = Adt0::C13(std::rc::Rc::new(f42(f38(v0.clone(), v48), v51)), std::rc::Rc::new(f43(v0.clone(), v48)));
            let v54: u64 = f44(match v0.clone() {
                Adt32::C0(_, h71, _, _, _, _, _, _, _) => {
                    (*h71).clone()
                }
            }, v53.clone(), 0u64)?;
            let v55: Adt10 = f37(match v50.clone() {
                Adt10::C0(h72, _) => {
                    (*h72).clone()
                }
            }, v52.clone(), v7.clone(), match v50 {
                Adt10::C0(_, h73) => {
                    h73
                }
            })?;
            Ok(f2(f39(f25(match v55.clone() {
                Adt10::C0(h74, _) => {
                    (*h74).clone()
                }
            }), Adt11::C5(std::rc::Rc::new(Adt21::C5(v54, v48)), std::rc::Rc::new(f45(match v0.clone() {
                Adt32::C0(_, h75, _, _, _, _, _, _, _) => {
                    (*h75).clone()
                }
            }, match v0 {
                Adt32::C0(_, _, h76, _, _, _, _, _, _) => {
                    (*h76).clone()
                }
            }, v53, f26(match v55.clone() {
                Adt10::C0(h77, _) => {
                    (*h77).clone()
                }
            }), v52)), std::rc::Rc::new(v7))), match v55 {
                Adt10::C0(_, h78) => {
                    h78
                }
            }))
        }
        Adt2::C8(r167, r168) => {
            let v56 = (*r167).clone();
            let v57 = (*r168).clone();
            let v58: Adt0 = f31(v2.clone());
            let v59: (Adt0, Adt0) = f46(v58.clone());
            let v60: u64 = f44(match v0.clone() {
                Adt32::C0(_, h79, _, _, _, _, _, _, _) => {
                    (*h79).clone()
                }
            }, v58.clone(), 0u64)?;
            let v61: Adt17 = Adt17::C3({
                let a80 = v8;
                let a81 = 1u64;
                nat_add(a80, a81)?
            });
            let v62: Adt10 = f8(v0.clone(), v56.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v56), {
                let a82 = v8;
                let a83 = 1u64;
                nat_add(a82, a83)?
            })?;
            let v63: Adt10 = f8(v0.clone(), v57.clone(), f4(v2), f4(v3), v4, Adt0::C14, 1u64, f10(v57), match v62.clone() {
                Adt10::C0(_, h84) => {
                    h84
                }
            })?;
            let v64: Adt10 = f37(match v63.clone() {
                Adt10::C0(h85, _) => {
                    (*h85).clone()
                }
            }, {
                let (h86, _) = v59;
                h86
            }, v7.clone(), match v63 {
                Adt10::C0(_, h87) => {
                    h87
                }
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v61.clone()))), true, std::rc::Rc::new(Adt22::C4(Adt26::C1, v60)), std::rc::Rc::new(match v62 {
                Adt10::C0(h88, _) => {
                    (*h88).clone()
                }
            }), std::rc::Rc::new(v7.clone()), std::rc::Rc::new(f25(match v64.clone() {
                Adt10::C0(h89, _) => {
                    (*h89).clone()
                }
            })))), std::rc::Rc::new(Adt11::C4(std::rc::Rc::new(v61), std::rc::Rc::new(f26(match v64.clone() {
                Adt10::C0(h90, _) => {
                    (*h90).clone()
                }
            })), f47(v58, match v0.clone() {
                Adt32::C0(_, _, _, _, _, _, h91, _, _) => {
                    h91
                }
            }, match v0 {
                Adt32::C0(_, _, _, _, _, _, _, h92, _) => {
                    (*h92).clone()
                }
            }), std::rc::Rc::new(v7)))))), match v64 {
                Adt10::C0(_, h93) => {
                    h93
                }
            }))
        }
        Adt2::C9(r169, r170) => {
            let v65 = (*r169).clone();
            let v66 = (*r170).clone();
            let v67: Adt0 = f48(v66.clone(), f9(v3.clone()));
            let v68: Adt10 = f8(v0, v66.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v66), v8)?;
            let v69: u64 = f49(v65.clone(), v67.clone());
            let v70: Adt15 = f50(v65.clone(), v67.clone());
            let v71: Adt10 = f51(match v68.clone() {
                Adt10::C0(h94, _) => {
                    (*h94).clone()
                }
            }, {
                let a95 = v69;
                let a96 = 31u64;
                nat_eq(a95, a96)
            }, v70.clone(), v7, match v68 {
                Adt10::C0(_, h97) => {
                    h97
                }
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f25(match v71.clone() {
                Adt10::C0(h98, _) => {
                    (*h98).clone()
                }
            })), std::rc::Rc::new(Adt11::C3(std::rc::Rc::new(Adt19::C1(std::rc::Rc::new(Adt20::C0(v69, f52(v65, v67))))), std::rc::Rc::new(f26(match v71.clone() {
                Adt10::C0(h99, _) => {
                    (*h99).clone()
                }
            })), f53(v69), std::rc::Rc::new(v70)))))), match v71 {
                Adt10::C0(_, h100) => {
                    h100
                }
            }))
        }
        Adt2::C10(r171) => {
            let v72 = (*r171).clone();
            let v73: Adt17 = Adt17::C5({
                let a101 = v8;
                let a102 = 1u64;
                nat_add(a101, a102)?
            });
            let v74: Adt10 = f8(v0, v72.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v72), {
                let a103 = v8;
                let a104 = 1u64;
                nat_add(a103, a104)?
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v73.clone()))), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C0(Adt29::C0)), std::rc::Rc::new(Adt28::C5))))))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(match v74.clone() {
                Adt10::C0(h105, _) => {
                    (*h105).clone()
                }
            }), std::rc::Rc::new(v7.clone()), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(v73), std::rc::Rc::new(v7)))))), match v74 {
                Adt10::C0(_, h106) => {
                    h106
                }
            }))
        }
        Adt2::C11(r172) => {
            let v75 = (*r172).clone();
            let v76: Adt17 = Adt17::C5({
                let a107 = v8;
                let a108 = 1u64;
                nat_add(a107, a108)?
            });
            let v77: Adt10 = f8(v0, v75.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v75), {
                let a109 = v8;
                let a110 = 1u64;
                nat_add(a109, a110)?
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C0(Adt29::C0)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v76.clone()))), std::rc::Rc::new(Adt28::C5))))))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(match v77.clone() {
                Adt10::C0(h111, _) => {
                    (*h111).clone()
                }
            }), std::rc::Rc::new(v7.clone()), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(v76), std::rc::Rc::new(v7)))))), match v77 {
                Adt10::C0(_, h112) => {
                    h112
                }
            }))
        }
        Adt2::C12(r173, v79) => {
            let v78 = (*r173).clone();
            let m113 = f54(f31(f9(v3.clone())));
            match m113 {
                None => {
                    Ok(f2(Adt11::C12, v8))
                }
                Some(v80) => {
                    let m115 = f55(match v0.clone() {
                        Adt32::C0(_, _, _, h114, _, _, _, _, _) => {
                            (*h114).clone()
                        }
                    }, v80, 0u64);
                    match m115 {
                        None => {
                            Ok(f2(Adt11::C12, v8))
                        }
                        Some(v81) => {
                            let m116 = f56(v81.clone(), v79);
                            match m116 {
                                None => {
                                    Ok(f2(Adt11::C12, v8))
                                }
                                Some(v82) => {
                                    let v83: Adt17 = Adt17::C5({
                                        let a117 = v8;
                                        let a118 = 1u64;
                                        nat_add(a117, a118)?
                                    });
                                    let v84: Adt11 = if f57(match v0.clone() {
                                        Adt32::C0(_, h119, _, _, _, _, _, _, _) => {
                                            (*h119).clone()
                                        }
                                    }, match v0.clone() {
                                        Adt32::C0(_, _, h120, _, _, _, _, _, _) => {
                                            (*h120).clone()
                                        }
                                    }, Adt0::C12(v80), v82) {
                                        Adt11::C1(Adt16::C4, std::rc::Rc::new(v83.clone()), std::rc::Rc::new(Adt15::C0({
                                            let a121 = Str::lit("indirection");
                                            utf8_encode(a121)
                                        }, None::<Adt13>)))
                                    } else {
                                        Adt11::C1(Adt16::C0, std::rc::Rc::new(v83.clone()), std::rc::Rc::new(v7.clone()))
                                    };
                                    let v85: Adt10 = f8(v0, v78.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v78), {
                                        let a122 = v8;
                                        let a123 = 1u64;
                                        nat_add(a122, a123)?
                                    })?;
                                    let v86: Adt28 = Adt28::C4(v80, 0u64, std::rc::Rc::new(f58(v81, v79, v83, 0u64)?));
                                    if f59(match v85.clone() {
                                        Adt10::C0(h124, _) => {
                                            (*h124).clone()
                                        }
                                    }) {
                                        Ok(f2(Adt11::C8(std::rc::Rc::new(match v85.clone() {
                                            Adt10::C0(h125, _) => {
                                                (*h125).clone()
                                            }
                                        }), std::rc::Rc::new(Adt11::C15(std::rc::Rc::new(v86), std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v84))), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(v7)), match v85 {
                                            Adt10::C0(_, h126) => {
                                                h126
                                            }
                                        }))
                                    } else {
                                        let v87: Adt17 = Adt17::C1({
                                            let a128 = match v85.clone() {
                                                Adt10::C0(_, h127) => {
                                                    h127
                                                }
                                            };
                                            let a129 = 1u64;
                                            nat_add(a128, a129)?
                                        });
                                        Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v87.clone()))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(match v85.clone() {
                                            Adt10::C0(h130, _) => {
                                                (*h130).clone()
                                            }
                                        }), std::rc::Rc::new(v7.clone()), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C8(std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(v87), std::rc::Rc::new(v7.clone()))), std::rc::Rc::new(Adt11::C15(std::rc::Rc::new(v86), std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v84))), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(v7)))))), {
                                            let a132 = match v85 {
                                                Adt10::C0(_, h131) => {
                                                    h131
                                                }
                                            };
                                            let a133 = 1u64;
                                            nat_add(a132, a133)?
                                        }))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt2::C13 => {
            Ok(f2(Adt11::C12, v8))
        }
        Adt2::C14(r174, r175) => {
            let v88 = (*r174).clone();
            let v89 = (*r175).clone();
            let a134 = v6;
            let a135 = 3u64;
            if nat_eq(a134, a135) {
                f8(v0, v88.clone(), f9(v2), f9(v3), v4, Adt0::C14, 5u64, f10(v88), v8)
            } else {
                let a136 = v6;
                let a137 = 2u64;
                if nat_eq(a136, a137) {
                    let v90: Adt10 = f8(v0.clone(), v88.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), v5.clone(), 4u64, f10(v88), v8)?;
                    let v91: Adt10 = f8(v0, v89.clone(), f4(v2), f4(v3), v4, v5, 2u64, f10(v89), match v90.clone() {
                        Adt10::C0(_, h138) => {
                            h138
                        }
                    })?;
                    Ok(f2(Adt11::C13(std::rc::Rc::new(match v90 {
                        Adt10::C0(h139, _) => {
                            (*h139).clone()
                        }
                    }), std::rc::Rc::new(match v91.clone() {
                        Adt10::C0(h140, _) => {
                            (*h140).clone()
                        }
                    })), match v91 {
                        Adt10::C0(_, h141) => {
                            h141
                        }
                    }))
                } else {
                    let v92: Adt10 = f8(v0.clone(), v88.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v88), v8)?;
                    let v93: Adt10 = f8(v0, v89.clone(), f4(v2), f4(v3), v4, v5, 1u64, f10(v89), match v92.clone() {
                        Adt10::C0(_, h142) => {
                            h142
                        }
                    })?;
                    Ok(f2(Adt11::C13(std::rc::Rc::new(match v92 {
                        Adt10::C0(h143, _) => {
                            (*h143).clone()
                        }
                    }), std::rc::Rc::new(match v93.clone() {
                        Adt10::C0(h144, _) => {
                            (*h144).clone()
                        }
                    })), match v93 {
                        Adt10::C0(_, h145) => {
                            h145
                        }
                    }))
                }
            }
        }
        Adt2::C15(r176, v95, r177) => {
            let v94 = (*r176).clone();
            let v96 = (*r177).clone();
            let a146 = v6;
            let a147 = 5u64;
            if nat_eq(a146, a147) {
                f8(v0, v96.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v96), v8)
            } else {
                let v97: Adt0 = f60(match v0.clone() {
                    Adt32::C0(_, _, _, h148, _, _, _, _, _) => {
                        (*h148).clone()
                    }
                }, v5, v94);
                if f61(v97.clone(), match v0.clone() {
                    Adt32::C0(_, _, _, _, _, _, _, _, h149) => {
                        (*h149).clone()
                    }
                }) {
                    let v98: Adt10 = f8(v0, v96.clone(), f9(v2), f9(v3), f5(v95.clone(), v97.clone(), v4), Adt0::C14, 0u64, f10(v96.clone()), v8)?;
                    Ok(f2(Adt11::C15(std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(f62(v95, v97, v96)))), std::rc::Rc::new(f11(match v98.clone() {
                        Adt10::C0(h150, _) => {
                            (*h150).clone()
                        }
                    })), std::rc::Rc::new(Adt11::C12)), match v98 {
                        Adt10::C0(_, h151) => {
                            h151
                        }
                    }))
                } else {
                    Ok(f2(Adt11::C12, v8))
                }
            }
        }
    }
}

pub fn f9(v0: Adt0) -> Adt0 {
    f13(f14(v0))
}

pub fn f10(v0: Adt2) -> Adt15 {
    Adt15::C0({
        let a179 = {
            let a178 = Str::lit("expr:");
            utf8_encode(a178)
        };
        let a180 = f63(v0);
        append_bytes(a179, a180)
    }, None::<Adt13>)
}

pub fn f11(v0: Adt11) -> Adt11 {
    f64(v0.clone(), v0)
}

pub fn f12(p184: List<bool>, p185: u64) -> Option<bool> {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v0, v1) = m183;
                let m181 = v0;
                match m181.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m182 = v1;
                        if m182 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m182 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Adt0) -> Adt0 {
    let m186 = f65(v0);
    match m186 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f14(v0: Adt0) -> Adt0 {
    let m187 = f66(v0);
    match m187 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f15(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f16(v0: Adt0) -> bool {
    let a188 = f67(v0);
    let a189 = 14u64;
    nat_eq(a188, a189)
}

pub fn f17(p193: List<List<u64>>, p194: u64) -> Option<List<u64>> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v0, v1) = m192;
                let m190 = v0;
                match m190.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m191 = v1;
                        if m191 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m191 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt31, v1: u64) -> Option<Adt0> {
    f68(match v0 {
        Adt31::C0(_, _, _, h195, _, _) => {
            (*h195).clone()
        }
    }, v1)
}

pub fn f19(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, v4: u64, v5: Adt3, v6: Adt0) -> R<Adt11> {
    let m196 = v5;
    match m196 {
        Adt3::C0 => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C0), std::rc::Rc::new(f69({
                let a197 = Str::lit("value:unit");
                utf8_encode(a197)
            }, v6))))
        }
        Adt3::C1(v7) => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C1(v7)), std::rc::Rc::new(f69({
                let a198 = Str::lit("value:bool");
                utf8_encode(a198)
            }, v6))))
        }
        Adt3::C2(v8) => {
            let m199 = f70(v8)?;
            match m199 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v9) => {
                    Ok(Adt11::C0(std::rc::Rc::new(Adt12::C2(v9)), std::rc::Rc::new(f69({
                        let a200 = Str::lit("value:nat");
                        utf8_encode(a200)
                    }, v6))))
                }
            }
        }
        Adt3::C3(v10) => {
            let m201 = f72(v10)?;
            match m201 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v11) => {
                    Ok(Adt11::C0(std::rc::Rc::new(Adt12::C3({
                        let (h202, _) = v11;
                        h202
                    }, {
                        let (_, h203) = v11;
                        h203
                    })), std::rc::Rc::new(f69({
                        let a204 = Str::lit("value:int");
                        utf8_encode(a204)
                    }, v6))))
                }
            }
        }
        Adt3::C4(v12) => {
            f73(v12, v6, {
                let a205 = Str::lit("value:u8");
                utf8_encode(a205)
            })
        }
        Adt3::C5(v13) => {
            f73(v13, v6, {
                let a206 = Str::lit("value:u16");
                utf8_encode(a206)
            })
        }
        Adt3::C6(v14) => {
            f73(v14, v6, {
                let a207 = Str::lit("value:u32");
                utf8_encode(a207)
            })
        }
        Adt3::C7(v15) => {
            f73(v15, v6, {
                let a208 = Str::lit("value:u64");
                utf8_encode(a208)
            })
        }
        Adt3::C8(v16) => {
            f73(v16, v6, {
                let a209 = Str::lit("value:i8");
                utf8_encode(a209)
            })
        }
        Adt3::C9(v17) => {
            f73(v17, v6, {
                let a210 = Str::lit("value:i16");
                utf8_encode(a210)
            })
        }
        Adt3::C10(v18) => {
            f73(v18, v6, {
                let a211 = Str::lit("value:i32");
                utf8_encode(a211)
            })
        }
        Adt3::C11(v19) => {
            f73(v19, v6, {
                let a212 = Str::lit("value:i64");
                utf8_encode(a212)
            })
        }
        Adt3::C12(v20) => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C5(v20)), std::rc::Rc::new(f69({
                let a213 = Str::lit("value:string");
                utf8_encode(a213)
            }, v6))))
        }
        Adt3::C13(v21) => {
            let m214 = f74(v21)?;
            match m214 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v22) => {
                    Ok(Adt11::C0(std::rc::Rc::new(Adt12::C6(v22)), std::rc::Rc::new(f69({
                        let a215 = Str::lit("value:bytes");
                        utf8_encode(a215)
                    }, v6))))
                }
            }
        }
        Adt3::C14(v23) => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C7({
                let m216 = v23;
                match m216 {
                    Adt4::C0 => {
                        Adt14::C0
                    }
                    Adt4::C1 => {
                        Adt14::C1
                    }
                    Adt4::C2 => {
                        Adt14::C2
                    }
                }
            })), std::rc::Rc::new(f69({
                let a217 = Str::lit("value:ordering");
                utf8_encode(a217)
            }, v6))))
        }
        Adt3::C15 => {
            let v24: Adt0 = f13(v6.clone());
            Ok(Adt11::C5(std::rc::Rc::new(Adt21::C0(std::rc::Rc::new(f24(v0, v24)?))), std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(f69({
                let a218 = Str::lit("value:none");
                utf8_encode(a218)
            }, v6))))
        }
        Adt3::C16(r239) => {
            let v25 = (*r239).clone();
            let v26: Adt0 = f13(v6.clone());
            let v27: Adt11 = f19(v0, v1, v2, v3, 0u64, v25, v26)?;
            if f75(v27.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(std::rc::Rc::new(Adt21::C1), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v27), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(f69({
                    let a219 = Str::lit("value:some");
                    utf8_encode(a219)
                }, v6))))
            }
        }
        Adt3::C17(r240) => {
            let v28 = (*r240).clone();
            let v29: Adt11 = f19(v0.clone(), v1, v2, v3, 0u64, v28, f13(v6.clone()))?;
            if f75(v29.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(std::rc::Rc::new(Adt21::C2(std::rc::Rc::new(f24(v0.clone(), f13(v6.clone()))?), std::rc::Rc::new(f24(v0, f14(v6.clone()))?))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v29), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(f69({
                    let a220 = Str::lit("value:ok");
                    utf8_encode(a220)
                }, v6))))
            }
        }
        Adt3::C18(r241) => {
            let v30 = (*r241).clone();
            let v31: Adt11 = f19(v0.clone(), v1, v2, v3, 0u64, v30, f14(v6.clone()))?;
            if f75(v31.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(std::rc::Rc::new(Adt21::C3(std::rc::Rc::new(f24(v0.clone(), f13(v6.clone()))?), std::rc::Rc::new(f24(v0, f14(v6.clone()))?))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v31), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(f69({
                    let a221 = Str::lit("value:error");
                    utf8_encode(a221)
                }, v6))))
            }
        }
        Adt3::C19(r242) => {
            let v32 = (*r242).clone();
            f19(v0, v1, v2, v3, 1u64, v32, f13(v6))
        }
        Adt3::C20(r243, r244) => {
            let v33 = (*r243).clone();
            let v34 = (*r244).clone();
            let v35: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v33, f13(v6.clone()))?;
            if f75(v35.clone()) {
                Ok(f71(0u64))
            } else {
                let v36: Adt11 = f19(v0, v1, v2, v3, 0u64, v34, f14(v6.clone()))?;
                if f75(v36.clone()) {
                    Ok(f71(0u64))
                } else {
                    Ok(Adt11::C6(std::rc::Rc::new(v35), std::rc::Rc::new(v36), std::rc::Rc::new(f69({
                        let a222 = Str::lit("value:pair");
                        utf8_encode(a222)
                    }, v6))))
                }
            }
        }
        Adt3::C21(v37, r245) => {
            let v38 = (*r245).clone();
            let v39: u64 = f76(v6.clone());
            let m223 = f55(v2.clone(), v39, v37);
            match m223 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v40) => {
                    let v41: Adt11 = f19(v0, v1, v2, Adt0::C12(v39), 2u64, v38, v40)?;
                    if f75(v41.clone()) {
                        Ok(f71(0u64))
                    } else {
                        Ok(Adt11::C5(std::rc::Rc::new(Adt21::C4(v39, v37)), std::rc::Rc::new(v41), std::rc::Rc::new(f69({
                            let a224 = Str::lit("value:adt");
                            utf8_encode(a224)
                        }, v6))))
                    }
                }
            }
        }
        Adt3::C22(_, _) => {
            Ok(f71(0u64))
        }
        Adt3::C23 => {
            let a225 = v4;
            let a226 = 1u64;
            if nat_eq(a225, a226) {
                Ok(Adt11::C5(std::rc::Rc::new(Adt21::C7(std::rc::Rc::new(f24(v0, v6)?))), std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt15::C0({
                    let a227 = Str::lit("value:list");
                    utf8_encode(a227)
                }, None::<Adt13>))))
            } else {
                let a228 = v4;
                let a229 = 2u64;
                if nat_eq(a228, a229) {
                    Ok(Adt11::C12)
                } else {
                    Ok(f71(0u64))
                }
            }
        }
        Adt3::C24(r246, r247) => {
            let v44 = (*r246).clone();
            let v45 = (*r247).clone();
            let a230 = v4;
            let a231 = 1u64;
            if nat_eq(a230, a231) {
                let v46: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v6.clone())?;
                if f75(v46.clone()) {
                    Ok(f71(0u64))
                } else {
                    let v47: Adt11 = f19(v0, v1, v2, v3, 1u64, v45, v6)?;
                    if f75(v47.clone()) {
                        Ok(f71(0u64))
                    } else {
                        Ok(Adt11::C5(std::rc::Rc::new(Adt21::C6), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v46), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v47), std::rc::Rc::new(Adt11::C12))))), std::rc::Rc::new(Adt15::C0({
                            let a232 = Str::lit("value:list");
                            utf8_encode(a232)
                        }, None::<Adt13>))))
                    }
                }
            } else {
                let a233 = v4;
                let a234 = 2u64;
                if nat_eq(a233, a234) {
                    let m235 = v6;
                    match m235 {
                        Adt0::C0 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C1 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C2 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C3 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C5 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C6 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C7 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C14 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C15(r237, r238) => {
                            let v58 = (*r237).clone();
                            let v59 = (*r238).clone();
                            let v60: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v58.clone())?;
                            if f75(v60.clone()) {
                                Ok(f71(0u64))
                            } else {
                                let v61: Adt11 = f19(v0.clone(), v1.clone(), v2, v3.clone(), 2u64, v45, v59)?;
                                if f75(v61.clone()) {
                                    Ok(f71(0u64))
                                } else {
                                    Ok(Adt11::C13(std::rc::Rc::new(if f57(v0, v1, v3, v58) {
                                        Adt11::C2(Adt18::C1, std::rc::Rc::new(v60), std::rc::Rc::new(Adt15::C0({
                                            let a236 = Str::lit("indirection");
                                            utf8_encode(a236)
                                        }, None::<Adt13>)))
                                    } else {
                                        v60
                                    }), std::rc::Rc::new(v61)))
                                }
                            }
                        }
                    }
                } else {
                    Ok(f71(0u64))
                }
            }
        }
    }
}

pub fn f20(p254: List<(u64, Adt0)>, v1: u64) -> Adt0 {
    let m253 = p254;
    {
        let mut m253 = m253;
        loop {
            let step = {
                let v0 = m253;
                let m248 = v0;
                match m248.uncons() {
                    None => {
                        Err::<List<(u64, Adt0)>, Adt0>(Adt0::C14)
                    }
                    Some((v2, v3)) => {
                        let a250 = {
                            let (h249, _) = v2;
                            h249
                        };
                        let a251 = v1;
                        if nat_eq(a250, a251) {
                            let (_, h252) = v2;
                            Err::<List<(u64, Adt0)>, Adt0>(h252)
                        } else {
                            Ok::<List<(u64, Adt0)>, Adt0>(v3)
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

pub fn f21(v0: Adt0, v1: Adt0) -> bool {
    let m255 = v0;
    match m255 {
        Adt0::C0 => {
            let a256 = f67(v1);
            let a257 = 0u64;
            nat_eq(a256, a257)
        }
        Adt0::C1 => {
            let a258 = f67(v1);
            let a259 = 1u64;
            nat_eq(a258, a259)
        }
        Adt0::C2 => {
            let a260 = f67(v1);
            let a261 = 2u64;
            nat_eq(a260, a261)
        }
        Adt0::C3 => {
            let a262 = f67(v1);
            let a263 = 3u64;
            nat_eq(a262, a263)
        }
        Adt0::C4(v2) => {
            let a272 = f67(v1.clone());
            let a273 = 4u64;
            if nat_eq(a272, a273) {
                let a274 = f77(v2);
                let a275 = f77(f78(v1));
                nat_eq(a274, a275)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a264 = f67(v1);
            let a265 = 5u64;
            nat_eq(a264, a265)
        }
        Adt0::C6 => {
            let a266 = f67(v1);
            let a267 = 6u64;
            nat_eq(a266, a267)
        }
        Adt0::C7 => {
            let a268 = f67(v1);
            let a269 = 7u64;
            nat_eq(a268, a269)
        }
        Adt0::C8(r292) => {
            let v3 = (*r292).clone();
            let v4: bool = f21(v3, f13(v1.clone()));
            let a276 = f67(v1);
            let a277 = 8u64;
            if nat_eq(a276, a277) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r293, r294) => {
            let v5 = (*r293).clone();
            let v6 = (*r294).clone();
            let v7: bool = f21(v5, f13(v1.clone()));
            let v8: bool = f21(v6, f14(v1.clone()));
            let a278 = f67(v1);
            let a279 = 9u64;
            if nat_eq(a278, a279) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r295) => {
            let v9 = (*r295).clone();
            let v10: bool = f21(v9, f13(v1.clone()));
            let a280 = f67(v1);
            let a281 = 10u64;
            if nat_eq(a280, a281) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r296, r297) => {
            let v11 = (*r296).clone();
            let v12 = (*r297).clone();
            let v13: bool = f21(v11, f13(v1.clone()));
            let v14: bool = f21(v12, f14(v1.clone()));
            let a282 = f67(v1);
            let a283 = 11u64;
            if nat_eq(a282, a283) {
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
            let a284 = f67(v1.clone());
            let a285 = 12u64;
            if nat_eq(a284, a285) {
                let a286 = v15;
                let a287 = f76(v1);
                nat_eq(a286, a287)
            } else {
                false
            }
        }
        Adt0::C13(r298, r299) => {
            let v16 = (*r298).clone();
            let v17 = (*r299).clone();
            let v18: bool = f21(v16, f13(v1.clone()));
            let v19: bool = f21(v17, f14(v1.clone()));
            let a288 = f67(v1);
            let a289 = 13u64;
            if nat_eq(a288, a289) {
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
            let a270 = f67(v1);
            let a271 = 14u64;
            nat_eq(a270, a271)
        }
        Adt0::C15(r300, r301) => {
            let v20 = (*r300).clone();
            let v21 = (*r301).clone();
            let v22: bool = f21(v20, f13(v1.clone()));
            let v23: bool = f21(v21, f14(v1.clone()));
            let a290 = f67(v1);
            let a291 = 15u64;
            if nat_eq(a290, a291) {
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

pub fn f22(v0: Adt0) -> bool {
    let m302 = v0;
    match m302 {
        Adt0::C0 => {
            true
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            false
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(r303) => {
            let v2 = (*r303).clone();
            f22(v2)
        }
        Adt0::C9(r304, r305) => {
            let v3 = (*r304).clone();
            let v4 = (*r305).clone();
            if f22(v3) {
                f22(v4)
            } else {
                false
            }
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r306, r307) => {
            let v6 = (*r306).clone();
            let v7 = (*r307).clone();
            if f22(v6) {
                f22(v7)
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f23(v0: u64, v1: Adt0, v2: Adt2) -> Adt28 {
    if if f21(v1, Adt0::C0) {
        true
    } else {
        let a308 = f79(v2, v0);
        bool_not(a308)
    } {
        Adt28::C0(Adt29::C0)
    } else {
        Adt28::C1(std::rc::Rc::new(Adt17::C0(v0)))
    }
}

pub fn f24(v0: Adt0, v1: Adt0) -> R<Adt22> {
    let m309 = v1;
    match m309 {
        Adt0::C0 => {
            Ok(Adt22::C0(Adt23::C0))
        }
        Adt0::C1 => {
            Ok(Adt22::C0(Adt23::C1))
        }
        Adt0::C2 => {
            Ok(Adt22::C0(Adt23::C2))
        }
        Adt0::C3 => {
            Ok(Adt22::C0(Adt23::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt22::C1(f80(v2)))
        }
        Adt0::C5 => {
            Ok(Adt22::C0(Adt23::C5))
        }
        Adt0::C6 => {
            Ok(Adt22::C0(Adt23::C6))
        }
        Adt0::C7 => {
            Ok(Adt22::C0(Adt23::C4))
        }
        Adt0::C8(r310) => {
            let v3 = (*r310).clone();
            Ok(Adt22::C2(Adt24::C0, std::rc::Rc::new(f24(v0, v3)?)))
        }
        Adt0::C9(r311, r312) => {
            let v4 = (*r311).clone();
            let v5 = (*r312).clone();
            Ok(Adt22::C3(Adt25::C0, std::rc::Rc::new(f24(v0.clone(), v4)?), std::rc::Rc::new(f24(v0, v5)?)))
        }
        Adt0::C10(r313) => {
            let v6 = (*r313).clone();
            Ok(Adt22::C2(Adt24::C1, std::rc::Rc::new(f24(v0, v6)?)))
        }
        Adt0::C11(r314, r315) => {
            let v7 = (*r314).clone();
            let v8 = (*r315).clone();
            Ok(Adt22::C3(Adt25::C1, std::rc::Rc::new(f24(v0.clone(), v7)?), std::rc::Rc::new(f24(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt22::C4(Adt26::C0, v9))
        }
        Adt0::C13(r316, r317) => {
            let v10 = (*r316).clone();
            let v11 = (*r317).clone();
            Ok(Adt22::C4(Adt26::C1, f44(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt22::C5)
        }
        Adt0::C15(r318, r319) => {
            let v12 = (*r318).clone();
            let v13 = (*r319).clone();
            Ok(Adt22::C6(std::rc::Rc::new(f24(v0.clone(), v12)?), std::rc::Rc::new(f24(v0, v13)?)))
        }
    }
}

pub fn f25(v0: Adt11) -> Adt11 {
    let m320 = v0;
    match m320 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(r321, _) => {
            (*r321).clone()
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f26(v0: Adt11) -> Adt11 {
    let m322 = v0;
    match m322 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, r323) => {
            (*r323).clone()
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f27(v0: Adt0) -> Adt0 {
    f13(f14(f14(f14(v0))))
}

pub fn f28(v0: Adt11, v1: Adt11, v2: Adt11, v3: Adt15, v4: u64) -> R<Adt10> {
    let v5: Adt10 = if if f81(v2.clone()) {
        if f30(f25(v1.clone())) {
            f82(f26(v1.clone()))
        } else {
            false
        }
    } else {
        false
    } {
        f83(v1, v4)?
    } else {
        f2(v1, v4)
    };
    let v6: Adt11 = match v5.clone() {
        Adt10::C0(h324, _) => {
            (*h324).clone()
        }
    };
    let v7: u64 = match v5 {
        Adt10::C0(_, h325) => {
            h325
        }
    };
    let a326 = f84(v6.clone());
    let a327 = 1u64;
    if if nat_eq(a326, a327) {
        let a328 = f84(v2.clone());
        let a329 = 2u64;
        nat_eq(a328, a329)
    } else {
        false
    } {
        Ok(f2(v0, v7))
    } else {
        let a330 = f84(v6.clone());
        let a331 = 2u64;
        if if nat_eq(a330, a331) {
            let a332 = f84(v2.clone());
            let a333 = 1u64;
            nat_eq(a332, a333)
        } else {
            false
        } {
            Ok(f2(f85(v0, v3), v7))
        } else {
            if f86(v6.clone(), v2.clone())? {
                Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C0(Adt29::C0)), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(v0), std::rc::Rc::new(v3), std::rc::Rc::new(f25(v6.clone())))), std::rc::Rc::new(f26(v6))))), v7))
            } else {
                let a334 = 0u64;
                let a335 = f87(v0.clone());
                if nat_lt(a334, a335) {
                    Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(Adt17::C1({
                        let a336 = v7;
                        let a337 = 1u64;
                        nat_add(a336, a337)?
                    })))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(v0), std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C7(std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(Adt17::C1({
                        let a338 = v7;
                        let a339 = 1u64;
                        nat_add(a338, a339)?
                    })), std::rc::Rc::new(v3.clone()))), std::rc::Rc::new(v6), std::rc::Rc::new(v2), std::rc::Rc::new(v3)))))), {
                        let a340 = v7;
                        let a341 = 1u64;
                        nat_add(a340, a341)?
                    }))
                } else {
                    Ok(f2(Adt11::C7(std::rc::Rc::new(v0), std::rc::Rc::new(v6), std::rc::Rc::new(v2), std::rc::Rc::new(v3)), v7))
                }
            }
        }
    }
}

pub fn f29(v0: Adt11, v1: Adt11) -> Adt11 {
    f88(v0, v1.clone(), v1)
}

pub fn f30(v0: Adt11) -> bool {
    let m342 = v0;
    match m342 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            true
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    f13(v0)
}

pub fn f32(_: u64) -> Adt22 {
    Adt22::C0(Adt23::C0)
}

pub fn f33(v0: Adt5) -> Adt15 {
    Adt15::C0({
        let a344 = {
            let a343 = Str::lit("shape:");
            utf8_encode(a343)
        };
        let a345 = f89(v0);
        append_bytes(a344, a345)
    }, None::<Adt13>)
}

pub fn f34(v0: Adt32, v1: Adt17, v2: Adt0, v3: Adt2, v4: Adt11, v5: u64) -> R<Adt10> {
    let m346 = v2;
    match m346 {
        Adt0::C0 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C1 => {
            f90(v1, v3, v4, v5)
        }
        Adt0::C2 => {
            f91(v1, v3, v4, v5)
        }
        Adt0::C3 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C4(_) => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C5 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C6 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C7 => {
            f92(v1, v3, v4, v5)
        }
        Adt0::C8(_) => {
            f93(v1, v3, v4, v5)
        }
        Adt0::C9(_, _) => {
            f94(v1, v3, v4, v5)
        }
        Adt0::C10(_) => {
            f95(v1, v3, v4, v5)
        }
        Adt0::C11(_, _) => {
            f96(v1, v3, v4, v5)
        }
        Adt0::C12(v13) => {
            f97(match v0.clone() {
                Adt32::C0(_, h347, _, _, _, _, _, _, _) => {
                    (*h347).clone()
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h348, _, _, _, _, _, _) => {
                    (*h348).clone()
                }
            }, match v0 {
                Adt32::C0(_, _, _, h349, _, _, _, _, _) => {
                    (*h349).clone()
                }
            }, v1, v13, v3, v4, v5)
        }
        Adt0::C13(_, _) => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C14 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C15(_, _) => {
            Ok(f2(Adt11::C12, v5))
        }
    }
}

pub fn f35(v0: Adt11, v1: Adt17) -> bool {
    let m350 = v0;
    match m350 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(v4, r352, _) => {
            let v5 = (*r352).clone();
            let m351 = v4;
            match m351 {
                Adt16::C0 => {
                    f98(v5, v1)
                }
                Adt16::C1 => {
                    false
                }
                Adt16::C2 => {
                    false
                }
                Adt16::C3 => {
                    false
                }
                Adt16::C4 => {
                    false
                }
            }
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt5, v4: Adt0, v5: Adt11, v6: Adt15, v7: u64) -> R<Adt10> {
    let m353 = v3.clone();
    match m353 {
        Adt5::C0 => {
            let m354 = v4;
            match m354 {
                Adt0::C0 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C1 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C2 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C3 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C4(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C5 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C6 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C7 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C8(r355) => {
                    let v9 = (*r355).clone();
                    Ok(f2(Adt11::C5(std::rc::Rc::new(Adt21::C0(std::rc::Rc::new(f24(v0, v9)?))), std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v6)), v7))
                }
                Adt0::C9(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C10(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C11(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C12(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C13(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C14 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C15(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
            }
        }
        Adt5::C1 => {
            f99(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C2 => {
            f99(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C3 => {
            f99(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C4 => {
            let m356 = v4;
            match m356 {
                Adt0::C0 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C1 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C2 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C3 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C4(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C5 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C6 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C7 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C8(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C9(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C10(r357) => {
                    let v24 = (*r357).clone();
                    Ok(f2(Adt11::C5(std::rc::Rc::new(Adt21::C7(std::rc::Rc::new(f24(v0, v24)?))), std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v6)), v7))
                }
                Adt0::C11(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C12(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C13(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C14 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C15(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
            }
        }
        Adt5::C5 => {
            f99(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C6 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C7 => {
            Ok(f2(Adt11::C3(std::rc::Rc::new(Adt19::C1(std::rc::Rc::new(Adt20::C0(8u64, None::<Adt13>)))), std::rc::Rc::new(v5), true, std::rc::Rc::new(v6)), v7))
        }
        Adt5::C8 => {
            Ok(f2(Adt11::C6(std::rc::Rc::new(f101(v5.clone(), 0u64)), std::rc::Rc::new(f101(v5, 1u64)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C9 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C10 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C11 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C12 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C13 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C14 => {
            Ok(f2(Adt11::C0(std::rc::Rc::new(f100(v3)), std::rc::Rc::new(v6)), v7))
        }
        Adt5::C15(_) => {
            f99(v0, v1, v2, v3, v4, v5, v6, v7)
        }
    }
}

pub fn f37(v0: Adt11, v1: Adt0, v2: Adt15, v3: u64) -> R<Adt10> {
    if f102(v0.clone(), v1.clone()) {
        f103(v0, v1, v2, v3)
    } else {
        Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v0)), v3))
    }
}

pub fn f38(v0: Adt32, v1: u64) -> Adt0 {
    let m359 = f56(match v0 {
        Adt32::C0(_, _, _, _, h358, _, _, _, _) => {
            (*h358).clone()
        }
    }, v1);
    match m359 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f39(v0: Adt11, v1: Adt11) -> Adt11 {
    if f30(v0.clone()) {
        v1
    } else {
        let v2: Adt11 = f11(v1);
        Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f104(v0, f25(v2.clone()))), std::rc::Rc::new(f26(v2)))))
    }
}

pub fn f40(v0: Adt2) -> R<u64> {
    let m360 = v0;
    match m360 {
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
        Adt2::C14(_, r363) => {
            let v33 = (*r363).clone();
            let v34: u64 = f40(v33)?;
            let a361 = 1u64;
            let a362 = v34;
            nat_add(a361, a362)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f41(v0: Adt0, v1: u64) -> Adt0 {
    let m364 = v0;
    match m364 {
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
        Adt0::C15(r366, r367) => {
            let v12 = (*r366).clone();
            let v13 = (*r367).clone();
            let m365 = v1;
            if m365 == 0 {
                Adt0::C14
            } else {
                let v14 = m365 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f41(v13, v14)))
            }
        }
    }
}

pub fn f42(p373: Adt0, p374: u64) -> Adt0 {
    let m372 = (p373, p374);
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v0, v1) = m372;
                let m368 = v0;
                match m368 {
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
                    Adt0::C15(r370, r371) => {
                        let v12 = (*r370).clone();
                        let v13 = (*r371).clone();
                        let m369 = v1;
                        if m369 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m369 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt32, v1: u64) -> Adt0 {
    let m376 = f56(match v0 {
        Adt32::C0(_, _, _, _, _, h375, _, _, _) => {
            (*h375).clone()
        }
    }, v1);
    match m376 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f44(p383: Adt0, v1: Adt0, p384: u64) -> R<u64> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v0, v2) = m382;
                let m377 = v0;
                match m377 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r380, r381) => {
                        let v13 = (*r380).clone();
                        let v14 = (*r381).clone();
                        if f21(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a378 = v2;
                                let a379 = 1u64;
                                nat_add(a378, a379)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt11, v4: Adt0) -> Adt11 {
    let m385 = v3;
    match m385 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(r387, r388) => {
            let v40 = (*r387).clone();
            let v41 = (*r388).clone();
            if f105(v4.clone()) {
                Adt11::C13(std::rc::Rc::new(if f57(v0.clone(), v1.clone(), v2.clone(), f106(v4.clone())) {
                    Adt11::C2(Adt18::C1, std::rc::Rc::new(v40), std::rc::Rc::new(Adt15::C0({
                        let a386 = Str::lit("indirection");
                        utf8_encode(a386)
                    }, None::<Adt13>)))
                } else {
                    v40
                }), std::rc::Rc::new(f45(v0, v1, v2, v41, f107(v4))))
            } else {
                Adt11::C12
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f46(v0: Adt0) -> (Adt0, Adt0) {
    (f13(v0.clone()), f14(v0))
}

pub fn f47(v0: Adt0, v1: List<bool>, p393: Adt9) -> bool {
    let m392 = p393;
    {
        let mut m392 = m392;
        loop {
            let step = {
                let v2 = m392;
                let m389 = v2;
                match m389 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r390, v4, _, r391) => {
                        let v3 = (*r390).clone();
                        let v6 = (*r391).clone();
                        if f21(v3, v0.clone()) {
                            if f3(v1.clone(), v4) {
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
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt2, v1: Adt0) -> Adt0 {
    let m394 = v0;
    match m394 {
        Adt2::C0(_, _) => {
            Adt0::C0
        }
        Adt2::C1(_) => {
            Adt0::C0
        }
        Adt2::C2(_, _, _, _) => {
            Adt0::C0
        }
        Adt2::C3(_, _, _) => {
            Adt0::C0
        }
        Adt2::C4(_, _, _) => {
            Adt0::C0
        }
        Adt2::C5(_, _, _) => {
            Adt0::C0
        }
        Adt2::C6(_, _) => {
            Adt0::C0
        }
        Adt2::C7(_, _) => {
            Adt0::C0
        }
        Adt2::C8(_, _) => {
            Adt0::C0
        }
        Adt2::C9(_, _) => {
            Adt0::C0
        }
        Adt2::C10(_) => {
            Adt0::C0
        }
        Adt2::C11(_) => {
            Adt0::C0
        }
        Adt2::C12(_, _) => {
            Adt0::C0
        }
        Adt2::C13 => {
            Adt0::C0
        }
        Adt2::C14(_, _) => {
            f31(f9(v1))
        }
        Adt2::C15(_, _, _) => {
            Adt0::C0
        }
    }
}

pub fn f49(v0: Adt6, v1: Adt0) -> u64 {
    let m395 = v0;
    match m395 {
        Adt6::C0 => {
            0u64
        }
        Adt6::C1 => {
            1u64
        }
        Adt6::C2 => {
            2u64
        }
        Adt6::C3 => {
            3u64
        }
        Adt6::C4 => {
            4u64
        }
        Adt6::C5 => {
            5u64
        }
        Adt6::C6 => {
            6u64
        }
        Adt6::C7 => {
            7u64
        }
        Adt6::C8 => {
            9u64
        }
        Adt6::C9 => {
            10u64
        }
        Adt6::C10 => {
            11u64
        }
        Adt6::C11 => {
            12u64
        }
        Adt6::C12 => {
            13u64
        }
        Adt6::C13 => {
            14u64
        }
        Adt6::C14 => {
            20u64
        }
        Adt6::C15 => {
            21u64
        }
        Adt6::C16 => {
            22u64
        }
        Adt6::C17 => {
            24u64
        }
        Adt6::C18 => {
            23u64
        }
        Adt6::C19 => {
            25u64
        }
        Adt6::C20 => {
            26u64
        }
        Adt6::C21 => {
            27u64
        }
        Adt6::C22 => {
            28u64
        }
        Adt6::C23 => {
            29u64
        }
        Adt6::C24 => {
            30u64
        }
        Adt6::C25 => {
            18u64
        }
        Adt6::C26 => {
            15u64
        }
        Adt6::C27 => {
            16u64
        }
        Adt6::C28 => {
            17u64
        }
        Adt6::C29 => {
            f108(v1, 32u64, 33u64, 32u64)
        }
        Adt6::C30 => {
            f108(v1, 34u64, 35u64, 36u64)
        }
        Adt6::C31 => {
            f108(v1, 37u64, 38u64, 37u64)
        }
        Adt6::C32 => {
            f108(v1, 39u64, 40u64, 39u64)
        }
        Adt6::C33 => {
            41u64
        }
        Adt6::C34 => {
            42u64
        }
        Adt6::C35 => {
            43u64
        }
        Adt6::C36 => {
            44u64
        }
        Adt6::C37 => {
            45u64
        }
        Adt6::C38 => {
            let m396 = v1;
            match m396 {
                Adt0::C0 => {
                    47u64
                }
                Adt0::C1 => {
                    47u64
                }
                Adt0::C2 => {
                    47u64
                }
                Adt0::C3 => {
                    46u64
                }
                Adt0::C4(_) => {
                    47u64
                }
                Adt0::C5 => {
                    47u64
                }
                Adt0::C6 => {
                    47u64
                }
                Adt0::C7 => {
                    47u64
                }
                Adt0::C8(_) => {
                    47u64
                }
                Adt0::C9(_, _) => {
                    47u64
                }
                Adt0::C10(_) => {
                    47u64
                }
                Adt0::C11(_, _) => {
                    47u64
                }
                Adt0::C12(_) => {
                    47u64
                }
                Adt0::C13(_, _) => {
                    47u64
                }
                Adt0::C14 => {
                    47u64
                }
                Adt0::C15(_, _) => {
                    47u64
                }
            }
        }
        Adt6::C39 => {
            19u64
        }
        Adt6::C40(_) => {
            31u64
        }
        Adt6::C41(r398) => {
            let v15 = (*r398).clone();
            let m397 = v15;
            match m397 {
                Adt0::C0 => {
                    49u64
                }
                Adt0::C1 => {
                    49u64
                }
                Adt0::C2 => {
                    49u64
                }
                Adt0::C3 => {
                    48u64
                }
                Adt0::C4(_) => {
                    49u64
                }
                Adt0::C5 => {
                    49u64
                }
                Adt0::C6 => {
                    49u64
                }
                Adt0::C7 => {
                    49u64
                }
                Adt0::C8(_) => {
                    49u64
                }
                Adt0::C9(_, _) => {
                    49u64
                }
                Adt0::C10(_) => {
                    49u64
                }
                Adt0::C11(_, _) => {
                    49u64
                }
                Adt0::C12(_) => {
                    49u64
                }
                Adt0::C13(_, _) => {
                    49u64
                }
                Adt0::C14 => {
                    49u64
                }
                Adt0::C15(_, _) => {
                    49u64
                }
            }
        }
    }
}

pub fn f50(v0: Adt6, v1: Adt0) -> Adt15 {
    Adt15::C0({
        let a400 = {
            let a399 = Str::lit("prim:");
            utf8_encode(a399)
        };
        let a401 = f109(v0.clone());
        append_bytes(a400, a401)
    }, f52(v0, v1))
}

pub fn f51(v0: Adt11, v1: bool, v2: Adt15, v3: Adt15, v4: u64) -> R<Adt10> {
    let m402 = v0;
    match m402 {
        Adt11::C0(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C7(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C9(_) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C11(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C12 => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C13(r415, r416) => {
            let v40 = (*r415).clone();
            let v41 = (*r416).clone();
            let v42: Adt10 = f51(v41, v1, v2.clone(), v3.clone(), {
                let a403 = v4;
                let a404 = 1u64;
                nat_add(a403, a404)?
            })?;
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(Adt17::C2({
                let a405 = v4;
                let a406 = 1u64;
                nat_add(a405, a406)?
            })))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(v40), std::rc::Rc::new(v3.clone()), std::rc::Rc::new(f25(match v42.clone() {
                Adt10::C0(h407, _) => {
                    (*h407).clone()
                }
            })))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(if v1 {
                Adt11::C2(Adt18::C2, std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(Adt17::C2({
                    let a408 = v4;
                    let a409 = 1u64;
                    nat_add(a408, a409)?
                })), std::rc::Rc::new(v3))), std::rc::Rc::new(Adt15::C0(match v2 {
                    Adt15::C0(h410, _) => {
                        h410
                    }
                }, None::<Adt13>)))
            } else {
                Adt11::C1(Adt16::C0, std::rc::Rc::new(Adt17::C2({
                    let a411 = v4;
                    let a412 = 1u64;
                    nat_add(a411, a412)?
                })), std::rc::Rc::new(v3))
            }), std::rc::Rc::new(f26(match v42.clone() {
                Adt10::C0(h413, _) => {
                    (*h413).clone()
                }
            }))))), match v42 {
                Adt10::C0(_, h414) => {
                    h414
                }
            }))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C16(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
    }
}

pub fn f52(v0: Adt6, v1: Adt0) -> Option<Adt13> {
    let m417 = v0;
    match m417 {
        Adt6::C0 => {
            None::<Adt13>
        }
        Adt6::C1 => {
            None::<Adt13>
        }
        Adt6::C2 => {
            None::<Adt13>
        }
        Adt6::C3 => {
            None::<Adt13>
        }
        Adt6::C4 => {
            None::<Adt13>
        }
        Adt6::C5 => {
            None::<Adt13>
        }
        Adt6::C6 => {
            None::<Adt13>
        }
        Adt6::C7 => {
            None::<Adt13>
        }
        Adt6::C8 => {
            None::<Adt13>
        }
        Adt6::C9 => {
            None::<Adt13>
        }
        Adt6::C10 => {
            None::<Adt13>
        }
        Adt6::C11 => {
            None::<Adt13>
        }
        Adt6::C12 => {
            None::<Adt13>
        }
        Adt6::C13 => {
            None::<Adt13>
        }
        Adt6::C14 => {
            f110(v1)
        }
        Adt6::C15 => {
            f110(v1)
        }
        Adt6::C16 => {
            f110(v1)
        }
        Adt6::C17 => {
            f110(v1)
        }
        Adt6::C18 => {
            f110(v1)
        }
        Adt6::C19 => {
            f110(v1)
        }
        Adt6::C20 => {
            f110(v1)
        }
        Adt6::C21 => {
            f110(v1)
        }
        Adt6::C22 => {
            f110(v1)
        }
        Adt6::C23 => {
            f110(v1)
        }
        Adt6::C24 => {
            f110(v1)
        }
        Adt6::C25 => {
            None::<Adt13>
        }
        Adt6::C26 => {
            None::<Adt13>
        }
        Adt6::C27 => {
            None::<Adt13>
        }
        Adt6::C28 => {
            None::<Adt13>
        }
        Adt6::C29 => {
            None::<Adt13>
        }
        Adt6::C30 => {
            None::<Adt13>
        }
        Adt6::C31 => {
            None::<Adt13>
        }
        Adt6::C32 => {
            None::<Adt13>
        }
        Adt6::C33 => {
            None::<Adt13>
        }
        Adt6::C34 => {
            None::<Adt13>
        }
        Adt6::C35 => {
            None::<Adt13>
        }
        Adt6::C36 => {
            None::<Adt13>
        }
        Adt6::C37 => {
            None::<Adt13>
        }
        Adt6::C38 => {
            f110(v1)
        }
        Adt6::C39 => {
            None::<Adt13>
        }
        Adt6::C40(v2) => {
            Some(f80(v2))
        }
        Adt6::C41(r418) => {
            let v3 = (*r418).clone();
            f110(v3)
        }
    }
}

pub fn f53(v0: u64) -> bool {
    let a419 = v0;
    let a420 = 0u64;
    if nat_eq(a419, a420) {
        true
    } else {
        let a421 = v0;
        let a422 = 2u64;
        if nat_eq(a421, a422) {
            true
        } else {
            let a423 = v0;
            let a424 = 8u64;
            if nat_eq(a423, a424) {
                true
            } else {
                let a425 = v0;
                let a426 = 48u64;
                if nat_eq(a425, a426) {
                    true
                } else {
                    let a427 = 7u64;
                    let a428 = v0;
                    if nat_lt(a427, a428) {
                        let a429 = v0;
                        let a430 = 14u64;
                        nat_lt(a429, a430)
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub fn f54(v0: Adt0) -> Option<u64> {
    let m431 = v0;
    match m431 {
        Adt0::C0 => {
            None::<u64>
        }
        Adt0::C1 => {
            None::<u64>
        }
        Adt0::C2 => {
            None::<u64>
        }
        Adt0::C3 => {
            None::<u64>
        }
        Adt0::C4(_) => {
            None::<u64>
        }
        Adt0::C5 => {
            None::<u64>
        }
        Adt0::C6 => {
            None::<u64>
        }
        Adt0::C7 => {
            None::<u64>
        }
        Adt0::C8(_) => {
            None::<u64>
        }
        Adt0::C9(_, _) => {
            None::<u64>
        }
        Adt0::C10(_) => {
            None::<u64>
        }
        Adt0::C11(_, _) => {
            None::<u64>
        }
        Adt0::C12(v8) => {
            Some(v8)
        }
        Adt0::C13(_, _) => {
            None::<u64>
        }
        Adt0::C14 => {
            None::<u64>
        }
        Adt0::C15(_, _) => {
            None::<u64>
        }
    }
}

pub fn f55(v0: Adt0, v1: u64, v2: u64) -> Option<Adt0> {
    let m432 = f56(v0, v1);
    match m432 {
        None => {
            None::<Adt0>
        }
        Some(v3) => {
            f56(v3, v2)
        }
    }
}

pub fn f56(p438: Adt0, p439: u64) -> Option<Adt0> {
    let m437 = (p438, p439);
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v0, v1) = m437;
                let m433 = v0;
                match m433 {
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
                    Adt0::C15(r435, r436) => {
                        let v12 = (*r435).clone();
                        let v13 = (*r436).clone();
                        let m434 = v1;
                        if m434 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m434 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    if f111(v0, v1.clone(), v2, v3.clone()) {
        true
    } else {
        f112(v1, v3, Adt0::C0)
    }
}

pub fn f58(v0: Adt0, v1: u64, v2: Adt17, v3: u64) -> R<Adt28> {
    let m440 = v0;
    match m440 {
        Adt0::C0 => {
            Ok(Adt28::C5)
        }
        Adt0::C1 => {
            Ok(Adt28::C5)
        }
        Adt0::C2 => {
            Ok(Adt28::C5)
        }
        Adt0::C3 => {
            Ok(Adt28::C5)
        }
        Adt0::C4(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C5 => {
            Ok(Adt28::C5)
        }
        Adt0::C6 => {
            Ok(Adt28::C5)
        }
        Adt0::C7 => {
            Ok(Adt28::C5)
        }
        Adt0::C8(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C9(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C10(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C11(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C12(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C13(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C14 => {
            Ok(Adt28::C5)
        }
        Adt0::C15(_, r445) => {
            let v15 = (*r445).clone();
            Ok(Adt28::C6(std::rc::Rc::new({
                let a441 = v3;
                let a442 = v1;
                if nat_eq(a441, a442) {
                    Adt28::C1(std::rc::Rc::new(v2.clone()))
                } else {
                    Adt28::C0(Adt29::C0)
                }
            }), std::rc::Rc::new(f58(v15, v1, v2, {
                let a443 = v3;
                let a444 = 1u64;
                nat_add(a443, a444)?
            })?)))
        }
    }
}

pub fn f59(v0: Adt11) -> bool {
    let m446 = v0;
    match m446 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(v3, _, _) => {
            let m447 = v3;
            match m447 {
                Adt16::C0 => {
                    true
                }
                Adt16::C1 => {
                    true
                }
                Adt16::C2 => {
                    true
                }
                Adt16::C3 => {
                    false
                }
                Adt16::C4 => {
                    false
                }
            }
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f60(v0: Adt0, v1: Adt0, v2: Adt5) -> Adt0 {
    let m448 = v2;
    match m448 {
        Adt5::C0 => {
            Adt0::C14
        }
        Adt5::C1 => {
            Adt0::C15(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C2 => {
            Adt0::C15(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C3 => {
            Adt0::C15(std::rc::Rc::new(f14(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C4 => {
            Adt0::C14
        }
        Adt5::C5 => {
            Adt0::C15(std::rc::Rc::new(f13(v1.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14))))
        }
        Adt5::C6 => {
            Adt0::C14
        }
        Adt5::C7 => {
            Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C8 => {
            Adt0::C15(std::rc::Rc::new(f13(v1.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f14(v1)), std::rc::Rc::new(Adt0::C14))))
        }
        Adt5::C9 => {
            Adt0::C14
        }
        Adt5::C10 => {
            Adt0::C14
        }
        Adt5::C11 => {
            Adt0::C14
        }
        Adt5::C12 => {
            Adt0::C14
        }
        Adt5::C13 => {
            Adt0::C14
        }
        Adt5::C14 => {
            Adt0::C14
        }
        Adt5::C15(v3) => {
            let m449 = f54(v1);
            match m449 {
                None => {
                    Adt0::C14
                }
                Some(v4) => {
                    let m450 = f55(v0, v4, v3);
                    match m450 {
                        None => {
                            Adt0::C14
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                }
            }
        }
    }
}

pub fn f61(p455: Adt0, v1: Adt0) -> bool {
    let m454 = p455;
    {
        let mut m454 = m454;
        loop {
            let step = {
                let v0 = m454;
                let m451 = v0;
                match m451 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r452, r453) => {
                        let v12 = (*r452).clone();
                        let v13 = (*r453).clone();
                        if f113(v12, v1.clone()) {
                            Ok::<Adt0, bool>(v13)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<u64>, v1: Adt0, v2: Adt2) -> Adt28 {
    let m456 = v0;
    match m456.uncons() {
        None => {
            Adt28::C5
        }
        Some((v3, v4)) => {
            let m457 = v1;
            match m457 {
                Adt0::C0 => {
                    Adt28::C5
                }
                Adt0::C1 => {
                    Adt28::C5
                }
                Adt0::C2 => {
                    Adt28::C5
                }
                Adt0::C3 => {
                    Adt28::C5
                }
                Adt0::C4(_) => {
                    Adt28::C5
                }
                Adt0::C5 => {
                    Adt28::C5
                }
                Adt0::C6 => {
                    Adt28::C5
                }
                Adt0::C7 => {
                    Adt28::C5
                }
                Adt0::C8(_) => {
                    Adt28::C5
                }
                Adt0::C9(_, _) => {
                    Adt28::C5
                }
                Adt0::C10(_) => {
                    Adt28::C5
                }
                Adt0::C11(_, _) => {
                    Adt28::C5
                }
                Adt0::C12(_) => {
                    Adt28::C5
                }
                Adt0::C13(_, _) => {
                    Adt28::C5
                }
                Adt0::C14 => {
                    Adt28::C5
                }
                Adt0::C15(r458, r459) => {
                    let v15 = (*r458).clone();
                    let v16 = (*r459).clone();
                    Adt28::C6(std::rc::Rc::new(f23(v3, v15, v2.clone())), std::rc::Rc::new(f62(v4, v16, v2)))
                }
            }
        }
    }
}

pub fn f63(v0: Adt2) -> Bytes {
    let m460 = v0;
    match m460 {
        Adt2::C0(_, _) => {
            let a461 = Str::lit("value");
            utf8_encode(a461)
        }
        Adt2::C1(_) => {
            let a462 = Str::lit("var");
            utf8_encode(a462)
        }
        Adt2::C2(_, _, _, _) => {
            let a463 = Str::lit("let");
            utf8_encode(a463)
        }
        Adt2::C3(_, _, _) => {
            let a464 = Str::lit("cond");
            utf8_encode(a464)
        }
        Adt2::C4(_, _, _) => {
            let a465 = Str::lit("match");
            utf8_encode(a465)
        }
        Adt2::C5(_, _, _) => {
            let a466 = Str::lit("build");
            utf8_encode(a466)
        }
        Adt2::C6(_, _) => {
            let a467 = Str::lit("call");
            utf8_encode(a467)
        }
        Adt2::C7(_, _) => {
            let a468 = Str::lit("closure");
            utf8_encode(a468)
        }
        Adt2::C8(_, _) => {
            let a469 = Str::lit("apply");
            utf8_encode(a469)
        }
        Adt2::C9(_, _) => {
            let a470 = Str::lit("prim");
            utf8_encode(a470)
        }
        Adt2::C10(_) => {
            let a471 = Str::lit("first");
            utf8_encode(a471)
        }
        Adt2::C11(_) => {
            let a472 = Str::lit("second");
            utf8_encode(a472)
        }
        Adt2::C12(_, _) => {
            let a473 = Str::lit("field");
            utf8_encode(a473)
        }
        Adt2::C13 => {
            let a474 = Str::lit("");
            utf8_encode(a474)
        }
        Adt2::C14(_, _) => {
            let a475 = Str::lit("");
            utf8_encode(a475)
        }
        Adt2::C15(_, _, _) => {
            let a476 = Str::lit("");
            utf8_encode(a476)
        }
    }
}

pub fn f64(v0: Adt11, v1: Adt11) -> Adt11 {
    let m477 = v0;
    match m477 {
        Adt11::C0(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C1(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C2(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C5(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C6(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C8(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C9(r478) => {
            (*r478).clone()
        }
        Adt11::C10(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C11(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C12 => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C13(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C15(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C16(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
    }
}

pub fn f65(v0: Adt0) -> Option<Adt0> {
    let m479 = v0;
    match m479 {
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
        Adt0::C8(r480) => {
            let v2 = (*r480).clone();
            Some(v2)
        }
        Adt0::C9(r481, _) => {
            let v3 = (*r481).clone();
            Some(v3)
        }
        Adt0::C10(r482) => {
            let v5 = (*r482).clone();
            Some(v5)
        }
        Adt0::C11(r483, _) => {
            let v6 = (*r483).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r484, _) => {
            let v9 = (*r484).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r485, _) => {
            let v11 = (*r485).clone();
            Some(v11)
        }
    }
}

pub fn f66(v0: Adt0) -> Option<Adt0> {
    let m486 = v0;
    match m486 {
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
        Adt0::C9(_, r487) => {
            let v4 = (*r487).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r488) => {
            let v7 = (*r488).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r489) => {
            let v10 = (*r489).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r490) => {
            let v12 = (*r490).clone();
            Some(v12)
        }
    }
}

pub fn f67(v0: Adt0) -> u64 {
    let m491 = v0;
    match m491 {
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

pub fn f68(p497: Adt0, p498: u64) -> Option<Adt0> {
    let m496 = (p497, p498);
    {
        let mut m496 = m496;
        loop {
            let step = {
                let (v0, v1) = m496;
                let m492 = v0;
                match m492 {
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
                    Adt0::C15(r494, r495) => {
                        let v12 = (*r494).clone();
                        let v13 = (*r495).clone();
                        let m493 = v1;
                        if m493 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m493 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Bytes, v1: Adt0) -> Adt15 {
    Adt15::C0(v0, f114(v1))
}

pub fn f70(v0: Bytes) -> R<Option<u64>> {
    let m499 = f115(v0, false)?;
    match m499 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            Ok(Some({
                let (_, h500) = v1;
                h500
            }))
        }
    }
}

pub fn f71(_: u64) -> Adt11 {
    Adt11::C0(std::rc::Rc::new(Adt12::C8), std::rc::Rc::new(Adt15::C0({
        let a501 = Str::lit("");
        utf8_encode(a501)
    }, None::<Adt13>)))
}

pub fn f72(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m502 = f115(v0, true)?;
    match m502 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            let (h503, _) = v1;
            if h503 {
                let a505 = {
                    let (_, h504) = v1;
                    h504
                };
                let a506 = 9223372036854775809u64;
                if nat_lt(a505, a506) {
                    Ok(Some(({
                        let a508 = 0u64;
                        let a509 = {
                            let (_, h507) = v1;
                            h507
                        };
                        nat_lt(a508, a509)
                    }, {
                        let (_, h510) = v1;
                        h510
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            } else {
                let a512 = {
                    let (_, h511) = v1;
                    h511
                };
                let a513 = 9223372036854775808u64;
                if nat_lt(a512, a513) {
                    Ok(Some((false, {
                        let (_, h514) = v1;
                        h514
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            }
        }
    }
}

pub fn f73(v0: Bytes, v1: Adt0, v2: Bytes) -> R<Adt11> {
    let m515 = f116(v1.clone());
    match m515 {
        None => {
            Ok(f71(0u64))
        }
        Some(v3) => {
            let m516 = f117(v0)?;
            match m516 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v4) => {
                    Ok(Adt11::C0(std::rc::Rc::new(Adt12::C4(f80(v3), {
                        let (h517, _) = v4;
                        h517
                    }, {
                        let (_, h518) = v4;
                        h518
                    })), std::rc::Rc::new(f69(v2, v1))))
                }
            }
        }
    }
}

pub fn f74(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a519 = v0.clone();
        length_bytes(a519)
    };
    let v2: ((u64, Bytes), bool) = f118(Fn0::F119(v0), v1, (0u64, {
        let a520 = Str::lit("");
        utf8_encode(a520)
    }))?;
    let a523 = {
        let (h521, _) = {
            let (h522, _) = v2.clone();
            h522
        };
        h521
    };
    let a524 = v1;
    if nat_eq(a523, a524) {
        Ok(Some({
            let (_, h525) = {
                let (h526, _) = v2;
                h526
            };
            h525
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f75(v0: Adt11) -> bool {
    let m527 = v0;
    match m527 {
        Adt11::C0(r529, _) => {
            let v1 = (*r529).clone();
            let m528 = v1;
            match m528 {
                Adt12::C0 => {
                    false
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(_) => {
                    false
                }
                Adt12::C8 => {
                    true
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f76(v0: Adt0) -> u64 {
    let m530 = v0;
    match m530 {
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

pub fn f77(v0: Adt1) -> u64 {
    let m531 = v0;
    match m531 {
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

pub fn f78(v0: Adt0) -> Adt1 {
    let m532 = v0;
    match m532 {
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

pub fn f79(v0: Adt2, v1: u64) -> bool {
    let m533 = v0;
    match m533 {
        Adt2::C0(_, _) => {
            false
        }
        Adt2::C1(v4) => {
            let a534 = v4;
            let a535 = v1;
            nat_eq(a534, a535)
        }
        Adt2::C2(v5, _, r538, r539) => {
            let v7 = (*r538).clone();
            let v8 = (*r539).clone();
            if f79(v7, v1) {
                true
            } else {
                let a536 = v5;
                let a537 = v1;
                if nat_eq(a536, a537) {
                    false
                } else {
                    f79(v8, v1)
                }
            }
        }
        Adt2::C3(r540, r541, r542) => {
            let v9 = (*r540).clone();
            let v10 = (*r541).clone();
            let v11 = (*r542).clone();
            if f79(v9, v1) {
                true
            } else {
                if f79(v10, v1) {
                    true
                } else {
                    f79(v11, v1)
                }
            }
        }
        Adt2::C4(_, r543, r544) => {
            let v13 = (*r543).clone();
            let v14 = (*r544).clone();
            if f79(v13, v1) {
                true
            } else {
                f79(v14, v1)
            }
        }
        Adt2::C5(_, _, r545) => {
            let v17 = (*r545).clone();
            f79(v17, v1)
        }
        Adt2::C6(_, r546) => {
            let v19 = (*r546).clone();
            f79(v19, v1)
        }
        Adt2::C7(_, r547) => {
            let v21 = (*r547).clone();
            f79(v21, v1)
        }
        Adt2::C8(r548, r549) => {
            let v22 = (*r548).clone();
            let v23 = (*r549).clone();
            if f79(v22, v1) {
                true
            } else {
                f79(v23, v1)
            }
        }
        Adt2::C9(_, r550) => {
            let v25 = (*r550).clone();
            f79(v25, v1)
        }
        Adt2::C10(r551) => {
            let v26 = (*r551).clone();
            f79(v26, v1)
        }
        Adt2::C11(r552) => {
            let v27 = (*r552).clone();
            f79(v27, v1)
        }
        Adt2::C12(r553, _) => {
            let v28 = (*r553).clone();
            f79(v28, v1)
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14(r554, r555) => {
            let v30 = (*r554).clone();
            let v31 = (*r555).clone();
            if f79(v30, v1) {
                true
            } else {
                f79(v31, v1)
            }
        }
        Adt2::C15(_, v33, r556) => {
            let v34 = (*r556).clone();
            if f120(v33, v1) {
                false
            } else {
                f79(v34, v1)
            }
        }
    }
}

pub fn f80(v0: Adt1) -> Adt13 {
    let m557 = v0;
    match m557 {
        Adt1::C0 => {
            Adt13::C0
        }
        Adt1::C1 => {
            Adt13::C1
        }
        Adt1::C2 => {
            Adt13::C2
        }
        Adt1::C3 => {
            Adt13::C3
        }
        Adt1::C4 => {
            Adt13::C4
        }
        Adt1::C5 => {
            Adt13::C5
        }
        Adt1::C6 => {
            Adt13::C6
        }
        Adt1::C7 => {
            Adt13::C7
        }
    }
}

pub fn f81(v0: Adt11) -> bool {
    if f30(f25(v0.clone())) {
        f121(f26(v0))
    } else {
        false
    }
}

pub fn f82(v0: Adt11) -> bool {
    let m558 = v0;
    match m558 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, r559, _) => {
            let v25 = (*r559).clone();
            f81(v25)
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f83(v0: Adt11, v1: u64) -> R<Adt10> {
    let m560 = f26(v0.clone());
    match m560 {
        Adt11::C0(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C7(r565, r566, r567, r568) => {
            let v24 = (*r565).clone();
            let v25 = (*r566).clone();
            let v26 = (*r567).clone();
            let v27 = (*r568).clone();
            let v28: Adt17 = Adt17::C1({
                let a561 = v1;
                let a562 = 1u64;
                nat_add(a561, a562)?
            });
            Ok(f2(Adt11::C11(std::rc::Rc::new(f104(f25(v0), Adt11::C14(std::rc::Rc::new(Adt28::C1(std::rc::Rc::new(v28.clone()))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(v24), std::rc::Rc::new(v27.clone()), std::rc::Rc::new(Adt11::C12)))), std::rc::Rc::new(Adt11::C7(std::rc::Rc::new(Adt11::C1(Adt16::C0, std::rc::Rc::new(v28), std::rc::Rc::new(v27.clone()))), std::rc::Rc::new(v25), std::rc::Rc::new(v26), std::rc::Rc::new(v27)))), {
                let a563 = v1;
                let a564 = 1u64;
                nat_add(a563, a564)?
            }))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C9(_) => {
            Ok(f2(v0, v1))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C11(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C12 => {
            Ok(f2(v0, v1))
        }
        Adt11::C13(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C16(_, _) => {
            Ok(f2(v0, v1))
        }
    }
}

pub fn f84(v0: Adt11) -> u64 {
    if f30(f25(v0.clone())) {
        f87(f26(v0))
    } else {
        0u64
    }
}

pub fn f85(v0: Adt11, v1: Adt15) -> Adt11 {
    f122(v0.clone(), v0, v1)
}

pub fn f86(v0: Adt11, v1: Adt11) -> R<bool> {
    let v2: Adt33 = f123(v0, f124(0u64))?;
    let v3: Adt33 = f123(v1, f124(0u64))?;
    if f125(match v2.clone() {
        Adt33::C0(h569, _, _, _) => {
            h569
        }
    }, match v3.clone() {
        Adt33::C0(h570, _, _, _) => {
            h570
        }
    }) {
        Ok(f126(match v2 {
            Adt33::C0(_, h571, _, _) => {
                h571
            }
        }, match v3 {
            Adt33::C0(_, h572, _, _) => {
                h572
            }
        }))
    } else {
        Ok(false)
    }
}

pub fn f87(v0: Adt11) -> u64 {
    let m573 = v0;
    match m573 {
        Adt11::C0(r575, _) => {
            let v1 = (*r575).clone();
            let m574 = v1;
            match m574 {
                Adt12::C0 => {
                    0u64
                }
                Adt12::C1(v3) => {
                    if v3 {
                        1u64
                    } else {
                        2u64
                    }
                }
                Adt12::C2(_) => {
                    0u64
                }
                Adt12::C3(_, _) => {
                    0u64
                }
                Adt12::C4(_, _, _) => {
                    0u64
                }
                Adt12::C5(_) => {
                    0u64
                }
                Adt12::C6(_) => {
                    0u64
                }
                Adt12::C7(_) => {
                    0u64
                }
                Adt12::C8 => {
                    0u64
                }
            }
        }
        Adt11::C1(_, _, _) => {
            0u64
        }
        Adt11::C2(_, _, _) => {
            0u64
        }
        Adt11::C3(_, _, _, _) => {
            0u64
        }
        Adt11::C4(_, _, _, _) => {
            0u64
        }
        Adt11::C5(_, _, _) => {
            0u64
        }
        Adt11::C6(_, _, _) => {
            0u64
        }
        Adt11::C7(_, _, _, _) => {
            0u64
        }
        Adt11::C8(_, _, _) => {
            0u64
        }
        Adt11::C9(_) => {
            0u64
        }
        Adt11::C10(_, _, _) => {
            0u64
        }
        Adt11::C11(_, _) => {
            0u64
        }
        Adt11::C12 => {
            0u64
        }
        Adt11::C13(_, _) => {
            0u64
        }
        Adt11::C14(_, _, _, _, _, _) => {
            0u64
        }
        Adt11::C15(_, _, _) => {
            0u64
        }
        Adt11::C16(_, _) => {
            0u64
        }
    }
}

pub fn f88(v0: Adt11, v1: Adt11, v2: Adt11) -> Adt11 {
    let m576 = v1;
    match m576 {
        Adt11::C0(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C1(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C2(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C5(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C6(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C8(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C9(r577) => {
            let v32 = (*r577).clone();
            Adt11::C11(std::rc::Rc::new(f104(f25(v0), f25(v32.clone()))), std::rc::Rc::new(f26(v32)))
        }
        Adt11::C10(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C11(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C12 => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C13(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C15(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C16(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
    }
}

pub fn f89(v0: Adt5) -> Bytes {
    let m578 = v0;
    match m578 {
        Adt5::C0 => {
            let a579 = Str::lit("none");
            utf8_encode(a579)
        }
        Adt5::C1 => {
            let a580 = Str::lit("some");
            utf8_encode(a580)
        }
        Adt5::C2 => {
            let a581 = Str::lit("ok");
            utf8_encode(a581)
        }
        Adt5::C3 => {
            let a582 = Str::lit("error");
            utf8_encode(a582)
        }
        Adt5::C4 => {
            let a583 = Str::lit("nil");
            utf8_encode(a583)
        }
        Adt5::C5 => {
            let a584 = Str::lit("cons");
            utf8_encode(a584)
        }
        Adt5::C6 => {
            let a585 = Str::lit("zero");
            utf8_encode(a585)
        }
        Adt5::C7 => {
            let a586 = Str::lit("succ");
            utf8_encode(a586)
        }
        Adt5::C8 => {
            let a587 = Str::lit("pair");
            utf8_encode(a587)
        }
        Adt5::C9 => {
            let a588 = Str::lit("true");
            utf8_encode(a588)
        }
        Adt5::C10 => {
            let a589 = Str::lit("false");
            utf8_encode(a589)
        }
        Adt5::C11 => {
            let a590 = Str::lit("unit");
            utf8_encode(a590)
        }
        Adt5::C12 => {
            let a591 = Str::lit("lt");
            utf8_encode(a591)
        }
        Adt5::C13 => {
            let a592 = Str::lit("eq");
            utf8_encode(a592)
        }
        Adt5::C14 => {
            let a593 = Str::lit("gt");
            utf8_encode(a593)
        }
        Adt5::C15(_) => {
            let a594 = Str::lit("adt");
            utf8_encode(a594)
        }
    }
}

pub fn f90(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m595 = f127(v1.clone(), v2.clone(), 9u64)?;
    match m595 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let m596 = f127(v1, v2, 10u64)?;
            match m596 {
                None => {
                    Ok(f2(Adt11::C12, v3))
                }
                Some(v5) => {
                    f28(f128(v0), {
                        let (_, h597) = v4;
                        h597
                    }, {
                        let (_, h598) = v5;
                        h598
                    }, f33(Adt5::C9), v3)
                }
            }
        }
    }
}

pub fn f91(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m599 = f127(v1.clone(), v2.clone(), 6u64)?;
    match m599 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let m600 = f127(v1, v2, 7u64)?;
            match m600 {
                None => {
                    Ok(f2(Adt11::C12, v3))
                }
                Some(v5) => {
                    let v6: Adt11 = {
                        let (_, h601) = v5;
                        h601
                    };
                    let v7: Adt28 = f129({
                        let (h602, _) = v5;
                        h602
                    }, 0u64);
                    let v8: Adt11 = if f130(v7.clone()) {
                        v6
                    } else {
                        Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(v7), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(Adt11::C10(Adt27::C3, std::rc::Rc::new(v0.clone()), std::rc::Rc::new(Adt15::C0({
                            let a603 = Str::lit("");
                            utf8_encode(a603)
                        }, None::<Adt13>)))), std::rc::Rc::new(f33(Adt5::C7)), std::rc::Rc::new(f25(v6.clone())))), std::rc::Rc::new(f26(v6)))
                    };
                    f28(Adt11::C10(Adt27::C0, std::rc::Rc::new(v0), std::rc::Rc::new(f33(Adt5::C6))), {
                        let (_, h604) = v4;
                        h604
                    }, v8, f33(Adt5::C6), v3)
                }
            }
        }
    }
}

pub fn f92(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v5: Adt11 = {
        let m605 = f127(v1.clone(), v2.clone(), 14u64)?;
        match m605 {
            None => {
                Adt11::C12
            }
            Some(v4) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C7(Adt14::C2)), std::rc::Rc::new({
                    let (_, h606) = v4;
                    h606
                }), std::rc::Rc::new(Adt11::C12))
            }
        }
    };
    let v7: Adt11 = {
        let m607 = f127(v1.clone(), v2.clone(), 13u64)?;
        match m607 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C7(Adt14::C1)), std::rc::Rc::new({
                    let (_, h608) = v6;
                    h608
                }), std::rc::Rc::new(v5))
            }
        }
    };
    let v9: Adt11 = {
        let m609 = f127(v1, v2, 12u64)?;
        match m609 {
            None => {
                v7
            }
            Some(v8) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C7(Adt14::C0)), std::rc::Rc::new({
                    let (_, h610) = v8;
                    h610
                }), std::rc::Rc::new(v7))
            }
        }
    };
    Ok(f2(f131(v9, f128(v0), f33(Adt5::C12))?, v3))
}

pub fn f93(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m611 = f127(v1.clone(), v2.clone(), 1u64)?;
            match m611 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(std::rc::Rc::new(Adt28::C3(Adt30::C0, std::rc::Rc::new(f129({
                        let (h612, _) = v4;
                        h612
                    }, 0u64)))), std::rc::Rc::new({
                        let (_, h613) = v4;
                        h613
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m614 = f127(v1, v2, 0u64)?;
        match m614 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C0(Adt29::C2)), std::rc::Rc::new({
                    let (_, h615) = v6;
                    h615
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(f131(v7, f128(v0), f33(Adt5::C0))?, v3))
}

pub fn f94(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m616 = f127(v1.clone(), v2.clone(), 3u64)?;
            match m616 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(std::rc::Rc::new(Adt28::C3(Adt30::C2, std::rc::Rc::new(f129({
                        let (h617, _) = v4;
                        h617
                    }, 0u64)))), std::rc::Rc::new({
                        let (_, h618) = v4;
                        h618
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m619 = f127(v1, v2, 2u64)?;
        match m619 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C3(Adt30::C1, std::rc::Rc::new(f129({
                    let (h620, _) = v6;
                    h620
                }, 0u64)))), std::rc::Rc::new({
                    let (_, h621) = v6;
                    h621
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(f131(v7, f128(v0), f33(Adt5::C2))?, v3))
}

pub fn f95(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m622 = f127(v1.clone(), v2.clone(), 5u64)?;
            match m622 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(std::rc::Rc::new(Adt28::C3(Adt30::C0, std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f129({
                        let (h623, _) = v4.clone();
                        h623
                    }, 0u64)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f129({
                        let (h624, _) = v4;
                        h624
                    }, 1u64)), std::rc::Rc::new(Adt28::C5))))))))), std::rc::Rc::new({
                        let (_, h625) = v4;
                        h625
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m626 = f127(v1, v2, 4u64)?;
        match m626 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(std::rc::Rc::new(Adt28::C0(Adt29::C2)), std::rc::Rc::new({
                    let (_, h627) = v6;
                    h627
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(Adt11::C8(std::rc::Rc::new(Adt11::C10(Adt27::C2, std::rc::Rc::new(v0), std::rc::Rc::new(Adt15::C0({
        let a628 = Str::lit("");
        utf8_encode(a628)
    }, None::<Adt13>)))), std::rc::Rc::new(v7), std::rc::Rc::new(f33(Adt5::C4))), v3))
}

pub fn f96(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m629 = f127(v1, v2, 8u64)?;
    match m629 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let v5: Adt11 = {
                let (_, h630) = v4.clone();
                h630
            };
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f129({
                let (h631, _) = v4.clone();
                h631
            }, 0u64)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f129({
                let (h632, _) = v4;
                h632
            }, 1u64)), std::rc::Rc::new(Adt28::C5))))))), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(f128(v0)), std::rc::Rc::new(f33(Adt5::C8)), std::rc::Rc::new(f25(v5.clone())))), std::rc::Rc::new(f26(v5))))), v3))
        }
    }
}

pub fn f97(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt17, v4: u64, v5: Adt2, v6: Adt11, v7: u64) -> R<Adt10> {
    let v8: Adt10 = f132(v0, v1, v2.clone(), v4, 0u64, f133(v2, v4)?, v5, v6, v7)?;
    Ok(f2(f131(match v8.clone() {
        Adt10::C0(h633, _) => {
            (*h633).clone()
        }
    }, f128(v3), f33(Adt5::C15(0u64)))?, match v8 {
        Adt10::C0(_, h634) => {
            h634
        }
    }))
}

pub fn f98(v0: Adt17, v1: Adt17) -> bool {
    let a635 = f134(v0.clone());
    let a636 = f134(v1.clone());
    if nat_eq(a635, a636) {
        let a637 = f135(v0);
        let a638 = f135(v1);
        nat_eq(a637, a638)
    } else {
        false
    }
}

pub fn f99(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt5, v4: Adt0, v5: Adt11, v6: Adt15, v7: u64) -> R<Adt10> {
    let v8: Adt34 = f136(v0.clone(), v2, v3, v4.clone())?;
    let v9: Adt10 = f37(v5, match v8.clone() {
        Adt34::C0(h639, _, _) => {
            (*h639).clone()
        }
    }, v6.clone(), v7)?;
    Ok(f2(f39(f25(match v9.clone() {
        Adt10::C0(h640, _) => {
            (*h640).clone()
        }
    }), Adt11::C5(std::rc::Rc::new(match v8.clone() {
        Adt34::C0(_, h641, _) => {
            (*h641).clone()
        }
    }), std::rc::Rc::new(if match v8.clone() {
        Adt34::C0(_, _, h642) => {
            h642
        }
    } {
        f45(v0, v1, v4, f26(match v9.clone() {
            Adt10::C0(h643, _) => {
                (*h643).clone()
            }
        }), match v8 {
            Adt34::C0(h644, _, _) => {
                (*h644).clone()
            }
        })
    } else {
        f26(match v9.clone() {
            Adt10::C0(h645, _) => {
                (*h645).clone()
            }
        })
    }), std::rc::Rc::new(v6))), match v9 {
        Adt10::C0(_, h646) => {
            h646
        }
    }))
}

pub fn f100(v0: Adt5) -> Adt12 {
    let m647 = v0;
    match m647 {
        Adt5::C0 => {
            Adt12::C0
        }
        Adt5::C1 => {
            Adt12::C0
        }
        Adt5::C2 => {
            Adt12::C0
        }
        Adt5::C3 => {
            Adt12::C0
        }
        Adt5::C4 => {
            Adt12::C0
        }
        Adt5::C5 => {
            Adt12::C0
        }
        Adt5::C6 => {
            Adt12::C2(0u64)
        }
        Adt5::C7 => {
            Adt12::C0
        }
        Adt5::C8 => {
            Adt12::C0
        }
        Adt5::C9 => {
            Adt12::C1(true)
        }
        Adt5::C10 => {
            Adt12::C1(false)
        }
        Adt5::C11 => {
            Adt12::C0
        }
        Adt5::C12 => {
            Adt12::C7(Adt14::C0)
        }
        Adt5::C13 => {
            Adt12::C7(Adt14::C1)
        }
        Adt5::C14 => {
            Adt12::C7(Adt14::C2)
        }
        Adt5::C15(_) => {
            Adt12::C0
        }
    }
}

pub fn f101(p653: Adt11, p654: u64) -> Adt11 {
    let m652 = (p653, p654);
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v0, v1) = m652;
                let m648 = v0;
                match m648 {
                    Adt11::C0(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C9(_) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C11(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C12 => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C13(r650, r651) => {
                        let v37 = (*r650).clone();
                        let v38 = (*r651).clone();
                        let m649 = v1;
                        if m649 == 0 {
                            Err::<(Adt11, u64), Adt11>(v37)
                        } else {
                            let v39 = m649 - 1;
                            Ok::<(Adt11, u64), Adt11>((v38, v39))
                        }
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C15(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C16(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                }
            };
            match step {
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(p660: Adt11, p661: Adt0) -> bool {
    let m659 = (p660, p661);
    {
        let mut m659 = m659;
        loop {
            let step = {
                let (v0, v1) = m659;
                let m655 = v0;
                match m655 {
                    Adt11::C0(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C9(_) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C11(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C12 => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C13(r657, r658) => {
                        let v37 = (*r657).clone();
                        let v38 = (*r658).clone();
                        if f105(v1.clone()) {
                            if if f21(f106(v1.clone()), Adt0::C0) {
                                let a656 = f121(v37);
                                bool_not(a656)
                            } else {
                                false
                            } {
                                Err::<(Adt11, Adt0), bool>(true)
                            } else {
                                Ok::<(Adt11, Adt0), bool>((v38, f107(v1)))
                            }
                        } else {
                            Err::<(Adt11, Adt0), bool>(false)
                        }
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C15(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C16(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Adt11, v1: Adt0, v2: Adt15, v3: u64) -> R<Adt10> {
    let m662 = v0;
    match m662 {
        Adt11::C0(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C7(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C9(_) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C11(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C12 => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C13(r672, r673) => {
            let v39 = (*r672).clone();
            let v40 = (*r673).clone();
            if f105(v1.clone()) {
                let v41: bool = f21(f106(v1.clone()), Adt0::C0);
                let v42: Adt10 = f103(v40, f107(v1), v2.clone(), if v41 {
                    v3
                } else {
                    let a663 = v3;
                    let a664 = 1u64;
                    nat_add(a663, a664)?
                })?;
                Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C14(std::rc::Rc::new(if v41 {
                    Adt28::C0(Adt29::C1)
                } else {
                    Adt28::C1(std::rc::Rc::new(Adt17::C2({
                        let a665 = v3;
                        let a666 = 1u64;
                        nat_add(a665, a666)?
                    })))
                }), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(v39), std::rc::Rc::new(v2.clone()), std::rc::Rc::new(f25(match v42.clone() {
                    Adt10::C0(h667, _) => {
                        (*h667).clone()
                    }
                })))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(if v41 {
                    Adt11::C0(std::rc::Rc::new(Adt12::C0), std::rc::Rc::new(v2))
                } else {
                    Adt11::C1(Adt16::C0, std::rc::Rc::new(Adt17::C2({
                        let a668 = v3;
                        let a669 = 1u64;
                        nat_add(a668, a669)?
                    })), std::rc::Rc::new(v2))
                }), std::rc::Rc::new(f26(match v42.clone() {
                    Adt10::C0(h670, _) => {
                        (*h670).clone()
                    }
                }))))), match v42 {
                    Adt10::C0(_, h671) => {
                        h671
                    }
                }))
            } else {
                Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C16(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
    }
}

pub fn f104(v0: Adt11, v1: Adt11) -> Adt11 {
    let m674 = v0;
    match m674 {
        Adt11::C0(_, _) => {
            v1
        }
        Adt11::C1(_, _, _) => {
            v1
        }
        Adt11::C2(_, _, _) => {
            v1
        }
        Adt11::C3(_, _, _, _) => {
            v1
        }
        Adt11::C4(_, _, _, _) => {
            v1
        }
        Adt11::C5(_, _, _) => {
            v1
        }
        Adt11::C6(_, _, _) => {
            v1
        }
        Adt11::C7(_, _, _, _) => {
            v1
        }
        Adt11::C8(_, _, _) => {
            v1
        }
        Adt11::C9(_) => {
            v1
        }
        Adt11::C10(_, _, _) => {
            v1
        }
        Adt11::C11(_, _) => {
            v1
        }
        Adt11::C12 => {
            v1
        }
        Adt11::C13(_, _) => {
            v1
        }
        Adt11::C14(r675, v40, r676, r677, r678, r679) => {
            let v39 = (*r675).clone();
            let v41 = (*r676).clone();
            let v42 = (*r677).clone();
            let v43 = (*r678).clone();
            let v44 = (*r679).clone();
            Adt11::C14(std::rc::Rc::new(v39), v40, std::rc::Rc::new(v41), std::rc::Rc::new(v42), std::rc::Rc::new(v43), std::rc::Rc::new(f104(v44, v1)))
        }
        Adt11::C15(_, _, _) => {
            v1
        }
        Adt11::C16(_, _) => {
            v1
        }
    }
}

pub fn f105(v0: Adt0) -> bool {
    let a680 = f67(v0);
    let a681 = 15u64;
    nat_eq(a680, a681)
}

pub fn f106(v0: Adt0) -> Adt0 {
    f13(v0)
}

pub fn f107(v0: Adt0) -> Adt0 {
    f14(v0)
}

pub fn f108(v0: Adt0, v1: u64, v2: u64, v3: u64) -> u64 {
    let m682 = v0;
    match m682 {
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
            v3
        }
        Adt0::C6 => {
            v2
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
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f109(v0: Adt6) -> Bytes {
    let m683 = v0;
    match m683 {
        Adt6::C0 => {
            let a684 = Str::lit("nat_add");
            utf8_encode(a684)
        }
        Adt6::C1 => {
            let a685 = Str::lit("nat_sub");
            utf8_encode(a685)
        }
        Adt6::C2 => {
            let a686 = Str::lit("nat_mul");
            utf8_encode(a686)
        }
        Adt6::C3 => {
            let a687 = Str::lit("nat_quot");
            utf8_encode(a687)
        }
        Adt6::C4 => {
            let a688 = Str::lit("nat_rem");
            utf8_encode(a688)
        }
        Adt6::C5 => {
            let a689 = Str::lit("nat_eq");
            utf8_encode(a689)
        }
        Adt6::C6 => {
            let a690 = Str::lit("nat_le");
            utf8_encode(a690)
        }
        Adt6::C7 => {
            let a691 = Str::lit("nat_lt");
            utf8_encode(a691)
        }
        Adt6::C8 => {
            let a692 = Str::lit("int_add");
            utf8_encode(a692)
        }
        Adt6::C9 => {
            let a693 = Str::lit("int_sub");
            utf8_encode(a693)
        }
        Adt6::C10 => {
            let a694 = Str::lit("int_mul");
            utf8_encode(a694)
        }
        Adt6::C11 => {
            let a695 = Str::lit("int_neg");
            utf8_encode(a695)
        }
        Adt6::C12 => {
            let a696 = Str::lit("int_quot");
            utf8_encode(a696)
        }
        Adt6::C13 => {
            let a697 = Str::lit("int_rem");
            utf8_encode(a697)
        }
        Adt6::C14 => {
            let a698 = Str::lit("checked_add");
            utf8_encode(a698)
        }
        Adt6::C15 => {
            let a699 = Str::lit("checked_sub");
            utf8_encode(a699)
        }
        Adt6::C16 => {
            let a700 = Str::lit("checked_mul");
            utf8_encode(a700)
        }
        Adt6::C17 => {
            let a701 = Str::lit("checked_neg");
            utf8_encode(a701)
        }
        Adt6::C18 => {
            let a702 = Str::lit("checked_quot");
            utf8_encode(a702)
        }
        Adt6::C19 => {
            let a703 = Str::lit("bit_and");
            utf8_encode(a703)
        }
        Adt6::C20 => {
            let a704 = Str::lit("bit_or");
            utf8_encode(a704)
        }
        Adt6::C21 => {
            let a705 = Str::lit("bit_xor");
            utf8_encode(a705)
        }
        Adt6::C22 => {
            let a706 = Str::lit("bit_not");
            utf8_encode(a706)
        }
        Adt6::C23 => {
            let a707 = Str::lit("shift_left");
            utf8_encode(a707)
        }
        Adt6::C24 => {
            let a708 = Str::lit("shift_right");
            utf8_encode(a708)
        }
        Adt6::C25 => {
            let a709 = Str::lit("equal");
            utf8_encode(a709)
        }
        Adt6::C26 => {
            let a710 = Str::lit("bool_not");
            utf8_encode(a710)
        }
        Adt6::C27 => {
            let a711 = Str::lit("bool_and");
            utf8_encode(a711)
        }
        Adt6::C28 => {
            let a712 = Str::lit("bool_or");
            utf8_encode(a712)
        }
        Adt6::C29 => {
            let a713 = Str::lit("append");
            utf8_encode(a713)
        }
        Adt6::C30 => {
            let a714 = Str::lit("length");
            utf8_encode(a714)
        }
        Adt6::C31 => {
            let a715 = Str::lit("index");
            utf8_encode(a715)
        }
        Adt6::C32 => {
            let a716 = Str::lit("slice");
            utf8_encode(a716)
        }
        Adt6::C33 => {
            let a717 = Str::lit("utf8_encode");
            utf8_encode(a717)
        }
        Adt6::C34 => {
            let a718 = Str::lit("utf8_decode");
            utf8_encode(a718)
        }
        Adt6::C35 => {
            let a719 = Str::lit("compare_bytes");
            utf8_encode(a719)
        }
        Adt6::C36 => {
            let a720 = Str::lit("split_exact");
            utf8_encode(a720)
        }
        Adt6::C37 => {
            let a721 = Str::lit("join");
            utf8_encode(a721)
        }
        Adt6::C38 => {
            let a722 = Str::lit("format_decimal");
            utf8_encode(a722)
        }
        Adt6::C39 => {
            let a723 = Str::lit("compare");
            utf8_encode(a723)
        }
        Adt6::C40(_) => {
            let a724 = Str::lit("convert");
            utf8_encode(a724)
        }
        Adt6::C41(_) => {
            let a725 = Str::lit("parse_decimal");
            utf8_encode(a725)
        }
    }
}

pub fn f110(v0: Adt0) -> Option<Adt13> {
    let m726 = v0;
    match m726 {
        Adt0::C0 => {
            None::<Adt13>
        }
        Adt0::C1 => {
            None::<Adt13>
        }
        Adt0::C2 => {
            None::<Adt13>
        }
        Adt0::C3 => {
            None::<Adt13>
        }
        Adt0::C4(v1) => {
            Some(f80(v1))
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6 => {
            None::<Adt13>
        }
        Adt0::C7 => {
            None::<Adt13>
        }
        Adt0::C8(_) => {
            None::<Adt13>
        }
        Adt0::C9(_, _) => {
            None::<Adt13>
        }
        Adt0::C10(_) => {
            None::<Adt13>
        }
        Adt0::C11(_, _) => {
            None::<Adt13>
        }
        Adt0::C12(_) => {
            None::<Adt13>
        }
        Adt0::C13(_, _) => {
            None::<Adt13>
        }
        Adt0::C14 => {
            None::<Adt13>
        }
        Adt0::C15(_, _) => {
            None::<Adt13>
        }
    }
}

pub fn f111(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    f137(v2, v1, f138(v0, Adt0::C14, v3))
}

pub fn f112(p732: Adt8, v1: Adt0, v2: Adt0) -> bool {
    let m731 = p732;
    {
        let mut m731 = m731;
        loop {
            let step = {
                let v0 = m731;
                let m727 = v0;
                match m727 {
                    Adt8::C0 => {
                        Err::<Adt8, bool>(false)
                    }
                    Adt8::C1(r728, r729, r730) => {
                        let v3 = (*r728).clone();
                        let v4 = (*r729).clone();
                        let v5 = (*r730).clone();
                        if f21(v3, v1.clone()) {
                            if f21(v4, v2.clone()) {
                                Err::<Adt8, bool>(true)
                            } else {
                                Ok::<Adt8, bool>(v5)
                            }
                        } else {
                            Ok::<Adt8, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt0, v1: Adt0) -> bool {
    let m733 = v0;
    match m733 {
        Adt0::C0 => {
            true
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r734, r735) => {
            let v4 = (*r734).clone();
            let v5 = (*r735).clone();
            if f113(v4, v1.clone()) {
                true
            } else {
                f113(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r736, r737) => {
            let v7 = (*r736).clone();
            let v8 = (*r737).clone();
            if f113(v7, v1.clone()) {
                f113(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f139(v1, Adt0::C12(v9))
        }
        Adt0::C13(r738, r739) => {
            let v10 = (*r738).clone();
            let v11 = (*r739).clone();
            f139(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f114(v0: Adt0) -> Option<Adt13> {
    let m740 = v0;
    match m740 {
        Adt0::C0 => {
            None::<Adt13>
        }
        Adt0::C1 => {
            None::<Adt13>
        }
        Adt0::C2 => {
            None::<Adt13>
        }
        Adt0::C3 => {
            None::<Adt13>
        }
        Adt0::C4(v1) => {
            Some(f80(v1))
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6 => {
            None::<Adt13>
        }
        Adt0::C7 => {
            None::<Adt13>
        }
        Adt0::C8(_) => {
            None::<Adt13>
        }
        Adt0::C9(_, _) => {
            None::<Adt13>
        }
        Adt0::C10(_) => {
            None::<Adt13>
        }
        Adt0::C11(_, _) => {
            None::<Adt13>
        }
        Adt0::C12(_) => {
            None::<Adt13>
        }
        Adt0::C13(_, _) => {
            None::<Adt13>
        }
        Adt0::C14 => {
            None::<Adt13>
        }
        Adt0::C15(_, _) => {
            None::<Adt13>
        }
    }
}

pub fn f115(v0: Bytes, v1: bool) -> R<Option<(bool, u64)>> {
    let m741 = f140(v0.clone(), 0u64);
    match m741 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v2) => {
            if if v1 {
                let a742 = v2;
                let a743 = 45u8;
                equal(a742, a743)
            } else {
                false
            } {
                let m744 = f141(v0, 1u64)?;
                match m744 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v3) => {
                        Ok(Some((true, v3)))
                    }
                }
            } else {
                let m745 = f141(v0, 0u64)?;
                match m745 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v4) => {
                        Ok(Some((false, v4)))
                    }
                }
            }
        }
    }
}

pub fn f116(v0: Adt0) -> Option<Adt1> {
    let m746 = v0;
    match m746 {
        Adt0::C0 => {
            None::<Adt1>
        }
        Adt0::C1 => {
            None::<Adt1>
        }
        Adt0::C2 => {
            None::<Adt1>
        }
        Adt0::C3 => {
            None::<Adt1>
        }
        Adt0::C4(v1) => {
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt1>
        }
        Adt0::C6 => {
            None::<Adt1>
        }
        Adt0::C7 => {
            None::<Adt1>
        }
        Adt0::C8(_) => {
            None::<Adt1>
        }
        Adt0::C9(_, _) => {
            None::<Adt1>
        }
        Adt0::C10(_) => {
            None::<Adt1>
        }
        Adt0::C11(_, _) => {
            None::<Adt1>
        }
        Adt0::C12(_) => {
            None::<Adt1>
        }
        Adt0::C13(_, _) => {
            None::<Adt1>
        }
        Adt0::C14 => {
            None::<Adt1>
        }
        Adt0::C15(_, _) => {
            None::<Adt1>
        }
    }
}

pub fn f117(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m747 = f115(v0, true)?;
    match m747 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            Ok(Some(({
                let (h748, _) = v1;
                if h748 {
                    let a750 = 0u64;
                    let a751 = {
                        let (_, h749) = v1;
                        h749
                    };
                    nat_lt(a750, a751)
                } else {
                    false
                }
            }, {
                let (_, h752) = v1;
                h752
            })))
        }
    }
}

pub fn f118(v0: Fn0, p757: u64, p758: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m756 = (p757, p758);
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v1, v2) = m756;
                let m753 = v1;
                if m753 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m753 - 1;
                    let m755 = {
                        let c754: Fn0 = v0.clone();
                        c754.apply(v2.clone())?
                    };
                    match m755 {
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
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f142(v0, v1)
}

pub fn f120(p763: List<u64>, v1: u64) -> bool {
    let m762 = p763;
    {
        let mut m762 = m762;
        loop {
            let step = {
                let v0 = m762;
                let m759 = v0;
                match m759.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a760 = v2;
                        let a761 = v1;
                        if nat_eq(a760, a761) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt11) -> bool {
    let m764 = v0;
    match m764 {
        Adt11::C0(r766, _) => {
            let v1 = (*r766).clone();
            let m765 = v1;
            match m765 {
                Adt12::C0 => {
                    true
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(_) => {
                    false
                }
                Adt12::C8 => {
                    false
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f122(v0: Adt11, v1: Adt11, v2: Adt15) -> Adt11 {
    let m767 = v0;
    match m767 {
        Adt11::C0(_, _) => {
            let a768 = f87(v1.clone());
            let a769 = 1u64;
            if nat_eq(a768, a769) {
                Adt11::C0(std::rc::Rc::new(Adt12::C1(false)), std::rc::Rc::new(v2))
            } else {
                let a770 = f87(v1.clone());
                let a771 = 2u64;
                if nat_eq(a770, a771) {
                    Adt11::C0(std::rc::Rc::new(Adt12::C1(true)), std::rc::Rc::new(v2))
                } else {
                    Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
                }
            }
        }
        Adt11::C1(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C2(v8, r778, _) => {
            let v9 = (*r778).clone();
            let a772 = f143(v8);
            let a773 = 0u64;
            if nat_eq(a772, a773) {
                v9
            } else {
                Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
            }
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C5(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C6(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C8(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C9(_) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C10(v33, r779, r780) => {
            let v34 = (*r779).clone();
            let v35 = (*r780).clone();
            let a774 = f144(v33.clone());
            let a775 = 0u64;
            if nat_eq(a774, a775) {
                Adt11::C10(Adt27::C1, std::rc::Rc::new(v34), std::rc::Rc::new(v35))
            } else {
                let a776 = f144(v33);
                let a777 = 1u64;
                if nat_eq(a776, a777) {
                    Adt11::C10(Adt27::C0, std::rc::Rc::new(v34), std::rc::Rc::new(v35))
                } else {
                    Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
                }
            }
        }
        Adt11::C11(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C12 => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C13(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C15(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
        Adt11::C16(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
    }
}

pub fn f123(v0: Adt11, v1: Adt33) -> R<Adt33> {
    let m781 = v0;
    match m781 {
        Adt11::C0(r782, _) => {
            let v2 = (*r782).clone();
            Ok(f145(v2, f146(v1, 1u64, 0u64)))
        }
        Adt11::C1(v4, r783, _) => {
            let v5 = (*r783).clone();
            f147(v5, f146(v1, 2u64, f148(v4)))
        }
        Adt11::C2(v7, r784, _) => {
            let v8 = (*r784).clone();
            f123(v8, f146(v1, 3u64, f143(v7)))
        }
        Adt11::C3(r785, r786, v12, _) => {
            let v10 = (*r785).clone();
            let v11 = (*r786).clone();
            f123(v11, f149(v10, f146(v1, 4u64, if v12 {
                1u64
            } else {
                0u64
            }))?)
        }
        Adt11::C4(r787, r788, v16, _) => {
            let v14 = (*r787).clone();
            let v15 = (*r788).clone();
            f123(v15, f147(v14, f146(v1, 5u64, if v16 {
                1u64
            } else {
                0u64
            }))?)
        }
        Adt11::C5(r789, r790, _) => {
            let v18 = (*r789).clone();
            let v19 = (*r790).clone();
            f123(v19, f150(v18, f146(v1, 6u64, 0u64)))
        }
        Adt11::C6(r791, r792, _) => {
            let v21 = (*r791).clone();
            let v22 = (*r792).clone();
            f123(v22, f123(v21, f146(v1, 7u64, 0u64))?)
        }
        Adt11::C7(r793, r794, r795, _) => {
            let v24 = (*r793).clone();
            let v25 = (*r794).clone();
            let v26 = (*r795).clone();
            f123(v26, f123(v25, f123(v24, f146(v1, 8u64, 0u64))?)?)
        }
        Adt11::C8(r796, r797, _) => {
            let v28 = (*r796).clone();
            let v29 = (*r797).clone();
            f123(v29, f123(v28, f146(v1, 9u64, 0u64))?)
        }
        Adt11::C9(r798) => {
            let v31 = (*r798).clone();
            f123(v31, v1)
        }
        Adt11::C10(v32, r799, _) => {
            let v33 = (*r799).clone();
            f147(v33, f146(v1, 10u64, f144(v32)))
        }
        Adt11::C11(r800, r801) => {
            let v35 = (*r800).clone();
            let v36 = (*r801).clone();
            f123(v36, f146(f123(v35, f146(v1, 11u64, 0u64))?, 12u64, 0u64))
        }
        Adt11::C12 => {
            Ok(f146(v1, 13u64, 0u64))
        }
        Adt11::C13(r802, r803) => {
            let v37 = (*r802).clone();
            let v38 = (*r803).clone();
            f123(v38, f123(v37, f146(v1, 14u64, 0u64))?)
        }
        Adt11::C14(r804, v40, r805, r806, _, r807) => {
            let v39 = (*r804).clone();
            let v41 = (*r805).clone();
            let v42 = (*r806).clone();
            let v44 = (*r807).clone();
            f123(v44, f151(v39, f123(v42, if v40 {
                f152(v41, f146(v1, 15u64, 1u64))
            } else {
                f146(v1, 15u64, 0u64)
            })?)?)
        }
        Adt11::C15(r808, r809, r810) => {
            let v45 = (*r808).clone();
            let v46 = (*r809).clone();
            let v47 = (*r810).clone();
            f123(v47, f123(v46, f151(v45, f146(v1, 16u64, 0u64))?)?)
        }
        Adt11::C16(r811, r812) => {
            let v48 = (*r811).clone();
            let v49 = (*r812).clone();
            f123(v49, f147(v48, f146(v1, 17u64, 0u64))?)
        }
    }
}

pub fn f124(_: u64) -> Adt33 {
    Adt33::C0(List::<(u64, u64)>::nil(), List::<Bytes>::nil(), List::<(Adt17, u64)>::nil(), 0u64)
}

pub fn f125(p825: List<(u64, u64)>, p826: List<(u64, u64)>) -> bool {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v0, v1) = m824;
                let m813 = v0;
                match m813.uncons() {
                    None => {
                        let m814 = v1;
                        match m814.uncons() {
                            None => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m815 = v1;
                        match m815.uncons() {
                            None => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a818 = {
                                    let (h816, _) = v4;
                                    h816
                                };
                                let a819 = {
                                    let (h817, _) = v6;
                                    h817
                                };
                                if if nat_eq(a818, a819) {
                                    let a822 = {
                                        let (_, h820) = v4;
                                        h820
                                    };
                                    let a823 = {
                                        let (_, h821) = v6;
                                        h821
                                    };
                                    nat_eq(a822, a823)
                                } else {
                                    false
                                } {
                                    Ok::<(List<(u64, u64)>, List<(u64, u64)>), bool>((v5, v7))
                                } else {
                                    Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(p833: List<Bytes>, p834: List<Bytes>) -> bool {
    let m832 = (p833, p834);
    {
        let mut m832 = m832;
        loop {
            let step = {
                let (v0, v1) = m832;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        let m828 = v1;
                        match m828.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m829 = v1;
                        match m829.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a830 = v4;
                                let a831 = v6;
                                if equal(a830, a831) {
                                    Ok::<(List<Bytes>, List<Bytes>), bool>((v5, v7))
                                } else {
                                    Err::<(List<Bytes>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m832 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: Adt2, v1: Adt11, v2: u64) -> R<Option<(Adt28, Adt11)>> {
    let m835 = v0;
    match m835 {
        Adt2::C0(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C1(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C3(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C4(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C5(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C6(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C7(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C8(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C10(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C11(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C12(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C13 => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C14(r839, r840) => {
            let v31 = (*r839).clone();
            let v32 = (*r840).clone();
            let m836 = f127(v32, f153(v1.clone()), v2)?;
            match m836 {
                None => {
                    let a837 = f154(v31)?;
                    let a838 = v2;
                    if if nat_eq(a837, a838) {
                        f155(f156(v1.clone()))
                    } else {
                        false
                    } {
                        Ok(Some((f157(f156(v1.clone())), f158(f156(v1)))))
                    } else {
                        Ok(None::<(Adt28, Adt11)>)
                    }
                }
                Some(v33) => {
                    Ok(Some(v33))
                }
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
    }
}

pub fn f128(v0: Adt17) -> Adt11 {
    Adt11::C1(Adt16::C0, std::rc::Rc::new(v0), std::rc::Rc::new(Adt15::C0({
        let a841 = Str::lit("expr:match");
        utf8_encode(a841)
    }, None::<Adt13>)))
}

pub fn f129(p848: Adt28, p849: u64) -> Adt28 {
    let m847 = (p848, p849);
    {
        let mut m847 = m847;
        loop {
            let step = {
                let (v0, v1) = m847;
                let m842 = v0;
                match m842 {
                    Adt28::C0(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C1(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C2(r844) => {
                        let v4 = (*r844).clone();
                        Ok::<(Adt28, u64), Adt28>((v4, v1))
                    }
                    Adt28::C3(_, _) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C4(_, _, _) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C5 => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C6(r845, r846) => {
                        let v10 = (*r845).clone();
                        let v11 = (*r846).clone();
                        let m843 = v1;
                        if m843 == 0 {
                            Err::<(Adt28, u64), Adt28>(v10)
                        } else {
                            let v12 = m843 - 1;
                            Ok::<(Adt28, u64), Adt28>((v11, v12))
                        }
                    }
                    Adt28::C7(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                }
            };
            match step {
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Adt28) -> bool {
    let m850 = v0;
    match m850 {
        Adt28::C0(v1) => {
            let m851 = v1;
            match m851 {
                Adt29::C0 => {
                    true
                }
                Adt29::C1 => {
                    false
                }
                Adt29::C2 => {
                    false
                }
            }
        }
        Adt28::C1(_) => {
            false
        }
        Adt28::C2(_) => {
            false
        }
        Adt28::C3(_, _) => {
            false
        }
        Adt28::C4(_, _, _) => {
            false
        }
        Adt28::C5 => {
            false
        }
        Adt28::C6(_, _) => {
            false
        }
        Adt28::C7(_) => {
            false
        }
    }
}

pub fn f131(v0: Adt11, v1: Adt11, v2: Adt15) -> R<Adt11> {
    let a852 = f30(v0.clone());
    if if bool_not(a852) {
        f159(v0.clone())?
    } else {
        false
    } {
        Ok(v1)
    } else {
        Ok(Adt11::C8(std::rc::Rc::new(v1), std::rc::Rc::new(v0), std::rc::Rc::new(v2)))
    }
}

pub fn f132(v0: Adt0, v1: Adt8, v2: Adt0, v3: u64, v4: u64, v5: u64, v6: Adt2, v7: Adt11, v8: u64) -> R<Adt10> {
    let m853 = v5;
    if m853 == 0 {
        Ok(f2(Adt11::C12, v8))
    } else {
        let v9 = m853 - 1;
        let m856 = f127(v6.clone(), v7.clone(), {
            let a854 = 16u64;
            let a855 = v4;
            nat_add(a854, a855)?
        })?;
        match m856 {
            None => {
                f132(v0, v1, v2, v3, {
                    let a869 = v4;
                    let a870 = 1u64;
                    nat_add(a869, a870)?
                }, v9, v6, v7, v8)
            }
            Some(v10) => {
                let m857 = f55(v2.clone(), v3, v4);
                match m857 {
                    None => {
                        f132(v0, v1, v2, v3, {
                            let a867 = v4;
                            let a868 = 1u64;
                            nat_add(a867, a868)?
                        }, v9, v6, v7, v8)
                    }
                    Some(v11) => {
                        let v12: Adt35 = f160(v0.clone(), v1.clone(), v3, f161({
                            let (h858, _) = v10;
                            h858
                        }), v11, v8)?;
                        let v13: Adt11 = {
                            let (_, h859) = v10;
                            h859
                        };
                        let v14: Adt10 = f132(v0, v1, v2, v3, {
                            let a860 = v4;
                            let a861 = 1u64;
                            nat_add(a860, a861)?
                        }, v9, v6, v7, match v12.clone() {
                            Adt35::C0(_, _, h862) => {
                                h862
                            }
                        })?;
                        Ok(f2(Adt11::C15(std::rc::Rc::new(Adt28::C4(v3, v4, std::rc::Rc::new(match v12.clone() {
                            Adt35::C0(_, h863, _) => {
                                (*h863).clone()
                            }
                        }))), std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f104(match v12 {
                            Adt35::C0(h864, _, _) => {
                                (*h864).clone()
                            }
                        }, f25(v13.clone()))), std::rc::Rc::new(f26(v13)))), std::rc::Rc::new(match v14.clone() {
                            Adt10::C0(h865, _) => {
                                (*h865).clone()
                            }
                        })), match v14 {
                            Adt10::C0(_, h866) => {
                                h866
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f133(v0: Adt0, v1: u64) -> R<u64> {
    let m871 = f56(v0, v1);
    match m871 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            f162(v2)
        }
    }
}

pub fn f134(v0: Adt17) -> u64 {
    let m872 = v0;
    match m872 {
        Adt17::C0(_) => {
            0u64
        }
        Adt17::C1(_) => {
            1u64
        }
        Adt17::C2(_) => {
            2u64
        }
        Adt17::C3(_) => {
            3u64
        }
        Adt17::C4(_) => {
            4u64
        }
        Adt17::C5(_) => {
            5u64
        }
        Adt17::C6(_) => {
            6u64
        }
        Adt17::C7(_) => {
            7u64
        }
        Adt17::C8(_) => {
            8u64
        }
        Adt17::C9(_) => {
            9u64
        }
    }
}

pub fn f135(v0: Adt17) -> u64 {
    let m873 = v0;
    match m873 {
        Adt17::C0(v1) => {
            v1
        }
        Adt17::C1(v2) => {
            v2
        }
        Adt17::C2(v3) => {
            v3
        }
        Adt17::C3(v4) => {
            v4
        }
        Adt17::C4(v5) => {
            v5
        }
        Adt17::C5(v6) => {
            v6
        }
        Adt17::C6(v7) => {
            v7
        }
        Adt17::C7(v8) => {
            v8
        }
        Adt17::C8(v9) => {
            v9
        }
        Adt17::C9(_) => {
            0u64
        }
    }
}

pub fn f136(v0: Adt0, v1: Adt0, v2: Adt5, v3: Adt0) -> R<Adt34> {
    let m874 = v2;
    match m874 {
        Adt5::C0 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C1 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f13(v3)), std::rc::Rc::new(Adt0::C14))), std::rc::Rc::new(Adt21::C1), false))
        }
        Adt5::C2 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f13(v3.clone())), std::rc::Rc::new(Adt0::C14))), std::rc::Rc::new(Adt21::C2(std::rc::Rc::new(f24(v0.clone(), f13(v3.clone()))?), std::rc::Rc::new(f24(v0, f14(v3))?))), false))
        }
        Adt5::C3 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f14(v3.clone())), std::rc::Rc::new(Adt0::C14))), std::rc::Rc::new(Adt21::C3(std::rc::Rc::new(f24(v0.clone(), f13(v3.clone()))?), std::rc::Rc::new(f24(v0, f14(v3))?))), false))
        }
        Adt5::C4 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C5 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f13(v3.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14))))), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C6 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C7 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C8 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C9 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C10 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C11 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C12 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C13 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C14 => {
            Ok(Adt34::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt21::C6), false))
        }
        Adt5::C15(v4) => {
            Ok(Adt34::C0(std::rc::Rc::new(f163(v1, f76(v3.clone()), v4)), std::rc::Rc::new(Adt21::C4(f76(v3), v4)), true))
        }
    }
}

pub fn f137(v0: Adt0, v1: Adt8, p879: Adt0) -> bool {
    let m878 = p879;
    {
        let mut m878 = m878;
        loop {
            let step = {
                let v2 = m878;
                let m875 = v2;
                match m875 {
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
                    Adt0::C15(r876, r877) => {
                        let v13 = (*r876).clone();
                        let v14 = (*r877).clone();
                        if f21(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f112(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m880 = v2;
    match m880 {
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
        Adt0::C8(r881) => {
            let v4 = (*r881).clone();
            f138(v0, v1, v4)
        }
        Adt0::C9(r882, r883) => {
            let v5 = (*r882).clone();
            let v6 = (*r883).clone();
            f138(v0.clone(), f138(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r884, r885) => {
            let v8 = (*r884).clone();
            let v9 = (*r885).clone();
            f138(v0.clone(), f138(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f164(v1, Adt0::C12(v10))
        }
        Adt0::C13(r886, r887) => {
            let v11 = (*r886).clone();
            let v12 = (*r887).clone();
            if f139(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f164(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
            } else {
                v1
            }
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f139(p892: Adt0, v1: Adt0) -> bool {
    let m891 = p892;
    {
        let mut m891 = m891;
        loop {
            let step = {
                let v0 = m891;
                let m888 = v0;
                match m888 {
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
                    Adt0::C15(r889, r890) => {
                        let v12 = (*r889).clone();
                        let v13 = (*r890).clone();
                        if f21(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: u64) -> Option<u8> {
    let a893 = v0;
    let a894 = v1;
    index_bytes(a893, a894)
}

pub fn f141(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a895 = v0.clone();
        length_bytes(a895)
    };
    let a896 = v1;
    let a897 = v2;
    if nat_lt(a896, a897) {
        let v3: ((u64, u64), bool) = f165(Fn1::F166(v0), {
            let a898 = v2;
            let a899 = v1;
            nat_sub(a898, a899)
        }, (v1, 0u64))?;
        let a902 = {
            let (h900, _) = {
                let (h901, _) = v3;
                h901
            };
            h900
        };
        let a903 = v2;
        if nat_eq(a902, a903) {
            Ok(Some({
                let (_, h904) = {
                    let (h905, _) = v3;
                    h905
                };
                h904
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f142(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m907 = f140(v0.clone(), {
        let (h906, _) = v1.clone();
        h906
    });
    match m907 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            let m911 = f140(v0, {
                let a909 = {
                    let (h908, _) = v1.clone();
                    h908
                };
                let a910 = 1u64;
                nat_add(a909, a910)?
            });
            match m911 {
                None => {
                    Ok(None::<(u64, Bytes)>)
                }
                Some(v3) => {
                    let m912 = f167(v2, v3)?;
                    match m912 {
                        None => {
                            Ok(None::<(u64, Bytes)>)
                        }
                        Some(v4) => {
                            Ok(Some(({
                                let a914 = {
                                    let (h913, _) = v1;
                                    h913
                                };
                                let a915 = 2u64;
                                nat_add(a914, a915)?
                            }, {
                                let a917 = {
                                    let (_, h916) = v1;
                                    h916
                                };
                                let a918 = f168(f169(0u64), v4, 1u64);
                                append_bytes(a917, a918)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f143(v0: Adt18) -> u64 {
    let m919 = v0;
    match m919 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1 => {
            1u64
        }
        Adt18::C2 => {
            2u64
        }
        Adt18::C3 => {
            3u64
        }
    }
}

pub fn f144(v0: Adt27) -> u64 {
    let m920 = v0;
    match m920 {
        Adt27::C0 => {
            0u64
        }
        Adt27::C1 => {
            1u64
        }
        Adt27::C2 => {
            2u64
        }
        Adt27::C3 => {
            3u64
        }
    }
}

pub fn f145(v0: Adt12, v1: Adt33) -> Adt33 {
    let m921 = v0;
    match m921 {
        Adt12::C0 => {
            f146(v1, 0u64, 0u64)
        }
        Adt12::C1(v2) => {
            f146(v1, 1u64, if v2 {
                1u64
            } else {
                0u64
            })
        }
        Adt12::C2(v3) => {
            f146(v1, 2u64, v3)
        }
        Adt12::C3(v4, v5) => {
            f146(f146(v1, 3u64, if v4 {
                1u64
            } else {
                0u64
            }), 4u64, v5)
        }
        Adt12::C4(v6, v7, v8) => {
            f146(f146(f146(v1, 5u64, f170(v6)), 3u64, if v7 {
                1u64
            } else {
                0u64
            }), 4u64, v8)
        }
        Adt12::C5(v9) => {
            f171(v1, 6u64, v9)
        }
        Adt12::C6(v10) => {
            f171(v1, 7u64, v10)
        }
        Adt12::C7(v11) => {
            f146(v1, 8u64, f172(v11))
        }
        Adt12::C8 => {
            f146(v1, 9u64, 0u64)
        }
    }
}

pub fn f146(v0: Adt33, v1: u64, v2: u64) -> Adt33 {
    Adt33::C0(List::cons((v1, v2), match v0.clone() {
        Adt33::C0(h922, _, _, _) => {
            h922
        }
    }), match v0.clone() {
        Adt33::C0(_, h923, _, _) => {
            h923
        }
    }, match v0.clone() {
        Adt33::C0(_, _, h924, _) => {
            h924
        }
    }, match v0 {
        Adt33::C0(_, _, _, h925) => {
            h925
        }
    })
}

pub fn f147(v0: Adt17, v1: Adt33) -> R<Adt33> {
    let m927 = f173(match v1.clone() {
        Adt33::C0(_, _, h926, _) => {
            h926
        }
    }, v0.clone());
    match m927 {
        None => {
            Ok(f146(v1, {
                let a930 = 60u64;
                let a931 = f134(v0.clone());
                nat_add(a930, a931)?
            }, f135(v0)))
        }
        Some(v2) => {
            Ok(f146(f146(v1, {
                let a928 = 100u64;
                let a929 = f134(v0);
                nat_add(a928, a929)?
            }, v2), 99u64, 0u64))
        }
    }
}

pub fn f148(v0: Adt16) -> u64 {
    let m932 = v0;
    match m932 {
        Adt16::C0 => {
            0u64
        }
        Adt16::C1 => {
            1u64
        }
        Adt16::C2 => {
            0u64
        }
        Adt16::C3 => {
            2u64
        }
        Adt16::C4 => {
            3u64
        }
    }
}

pub fn f149(v0: Adt19, v1: Adt33) -> R<Adt33> {
    let m933 = v0;
    match m933 {
        Adt19::C0(v2) => {
            Ok(f146(v1, 50u64, v2))
        }
        Adt19::C1(r936) => {
            let v3 = (*r936).clone();
            Ok(f146(f146(v1, 51u64, match v3.clone() {
                Adt20::C0(h934, _) => {
                    h934
                }
            }), 52u64, f174(match v3 {
                Adt20::C0(_, h935) => {
                    h935
                }
            })?))
        }
        Adt19::C2(v4, v5) => {
            Ok(f146(f146(v1, 53u64, v4), 54u64, f174(v5)?))
        }
    }
}

pub fn f150(v0: Adt21, v1: Adt33) -> Adt33 {
    let m937 = v0;
    match m937 {
        Adt21::C0(r938) => {
            let v2 = (*r938).clone();
            f152(v2, f146(v1, 40u64, 0u64))
        }
        Adt21::C1 => {
            f146(v1, 41u64, 0u64)
        }
        Adt21::C2(r939, r940) => {
            let v3 = (*r939).clone();
            let v4 = (*r940).clone();
            f152(v4, f152(v3, f146(v1, 42u64, 0u64)))
        }
        Adt21::C3(r941, r942) => {
            let v5 = (*r941).clone();
            let v6 = (*r942).clone();
            f152(v6, f152(v5, f146(v1, 43u64, 0u64)))
        }
        Adt21::C4(v7, v8) => {
            f146(f146(v1, 44u64, v7), 45u64, v8)
        }
        Adt21::C5(v9, v10) => {
            f146(f146(v1, 46u64, v9), 47u64, v10)
        }
        Adt21::C6 => {
            f146(v1, 48u64, 0u64)
        }
        Adt21::C7(r943) => {
            let v11 = (*r943).clone();
            f152(v11, f146(v1, 49u64, 0u64))
        }
    }
}

pub fn f151(v0: Adt28, v1: Adt33) -> R<Adt33> {
    let m944 = v0;
    match m944 {
        Adt28::C0(v2) => {
            Ok(f146(v1, 80u64, f175(v2)))
        }
        Adt28::C1(r954) => {
            let v3 = (*r954).clone();
            Ok(Adt33::C0(List::cons(({
                let a945 = 90u64;
                let a946 = f134(v3.clone());
                nat_add(a945, a946)?
            }, 0u64), match v1.clone() {
                Adt33::C0(h947, _, _, _) => {
                    h947
                }
            }), match v1.clone() {
                Adt33::C0(_, h948, _, _) => {
                    h948
                }
            }, List::cons((v3, match v1.clone() {
                Adt33::C0(_, _, _, h949) => {
                    h949
                }
            }), match v1.clone() {
                Adt33::C0(_, _, h950, _) => {
                    h950
                }
            }), {
                let a952 = match v1 {
                    Adt33::C0(_, _, _, h951) => {
                        h951
                    }
                };
                let a953 = 1u64;
                nat_add(a952, a953)?
            }))
        }
        Adt28::C2(r955) => {
            let v4 = (*r955).clone();
            f151(v4, f146(v1, 81u64, 0u64))
        }
        Adt28::C3(v5, r956) => {
            let v6 = (*r956).clone();
            f151(v6, f146(v1, 82u64, f176(v5)))
        }
        Adt28::C4(v7, v8, r957) => {
            let v9 = (*r957).clone();
            f151(v9, f146(f146(v1, 83u64, v7), 84u64, v8))
        }
        Adt28::C5 => {
            Ok(f146(v1, 85u64, 0u64))
        }
        Adt28::C6(r958, r959) => {
            let v10 = (*r958).clone();
            let v11 = (*r959).clone();
            f151(v11, f151(v10, f146(v1, 86u64, 0u64))?)
        }
        Adt28::C7(v12) => {
            Ok(f146(v1, 87u64, f172(v12)))
        }
    }
}

pub fn f152(v0: Adt22, v1: Adt33) -> Adt33 {
    let m960 = v0;
    match m960 {
        Adt22::C0(v2) => {
            f146(v1, 30u64, f177(v2))
        }
        Adt22::C1(v3) => {
            f146(v1, 31u64, f170(v3))
        }
        Adt22::C2(v4, r963) => {
            let v5 = (*r963).clone();
            f152(v5, f146(v1, 32u64, f178(v4)))
        }
        Adt22::C3(v6, r964, r965) => {
            let v7 = (*r964).clone();
            let v8 = (*r965).clone();
            f152(v8, f152(v7, f146(v1, 33u64, {
                let m961 = v6;
                match m961 {
                    Adt25::C0 => {
                        0u64
                    }
                    Adt25::C1 => {
                        1u64
                    }
                }
            })))
        }
        Adt22::C4(v9, v10) => {
            f146(f146(v1, 34u64, {
                let m962 = v9;
                match m962 {
                    Adt26::C0 => {
                        0u64
                    }
                    Adt26::C1 => {
                        1u64
                    }
                    Adt26::C2 => {
                        2u64
                    }
                }
            }), 35u64, v10)
        }
        Adt22::C5 => {
            f146(v1, 36u64, 0u64)
        }
        Adt22::C6(r966, r967) => {
            let v11 = (*r966).clone();
            let v12 = (*r967).clone();
            f152(v12, f152(v11, f146(v1, 37u64, 0u64)))
        }
    }
}

pub fn f153(v0: Adt11) -> Adt11 {
    let m968 = v0;
    match m968 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, r969) => {
            (*r969).clone()
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f154(v0: Adt2) -> R<u64> {
    let m970 = v0;
    match m970 {
        Adt2::C0(_, _) => {
            Ok(9999u64)
        }
        Adt2::C1(_) => {
            Ok(9999u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(9999u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C6(_, _) => {
            Ok(9999u64)
        }
        Adt2::C7(_, _) => {
            Ok(9999u64)
        }
        Adt2::C8(_, _) => {
            Ok(9999u64)
        }
        Adt2::C9(_, _) => {
            Ok(9999u64)
        }
        Adt2::C10(_) => {
            Ok(9999u64)
        }
        Adt2::C11(_) => {
            Ok(9999u64)
        }
        Adt2::C12(_, _) => {
            Ok(9999u64)
        }
        Adt2::C13 => {
            Ok(9999u64)
        }
        Adt2::C14(_, _) => {
            Ok(9999u64)
        }
        Adt2::C15(r971, _, _) => {
            let v31 = (*r971).clone();
            f179(v31)
        }
    }
}

pub fn f155(v0: Adt11) -> bool {
    let m972 = v0;
    match m972 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            true
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f156(v0: Adt11) -> Adt11 {
    let m973 = v0;
    match m973 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(r974, _) => {
            (*r974).clone()
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f157(v0: Adt11) -> Adt28 {
    let m975 = v0;
    match m975 {
        Adt11::C0(_, _) => {
            Adt28::C5
        }
        Adt11::C1(_, _, _) => {
            Adt28::C5
        }
        Adt11::C2(_, _, _) => {
            Adt28::C5
        }
        Adt11::C3(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C4(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C5(_, _, _) => {
            Adt28::C5
        }
        Adt11::C6(_, _, _) => {
            Adt28::C5
        }
        Adt11::C7(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C8(_, _, _) => {
            Adt28::C5
        }
        Adt11::C9(_) => {
            Adt28::C5
        }
        Adt11::C10(_, _, _) => {
            Adt28::C5
        }
        Adt11::C11(_, _) => {
            Adt28::C5
        }
        Adt11::C12 => {
            Adt28::C5
        }
        Adt11::C13(_, _) => {
            Adt28::C5
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt28::C5
        }
        Adt11::C15(r976, _, _) => {
            (*r976).clone()
        }
        Adt11::C16(_, _) => {
            Adt28::C5
        }
    }
}

pub fn f158(v0: Adt11) -> Adt11 {
    let m977 = v0;
    match m977 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, r978, _) => {
            (*r978).clone()
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f159(p984: Adt11) -> R<bool> {
    let m983 = p984;
    {
        let mut m983 = m983;
        loop {
            let step = {
                let v0 = m983;
                let m979 = v0;
                match m979 {
                    Adt11::C0(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C9(_) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C11(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C12 => {
                        Err::<Adt11, R<bool>>(Ok(true))
                    }
                    Adt11::C13(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C15(r980, r981, r982) => {
                        let v44 = (*r980).clone();
                        let v45 = (*r981).clone();
                        let v46 = (*r982).clone();
                        if f180(v44, v45)? {
                            Ok::<Adt11, R<bool>>(v46)
                        } else {
                            Err::<Adt11, R<bool>>(Ok(false))
                        }
                    }
                    Adt11::C16(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                }
            };
            match step {
                Ok(next) => { m983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Adt0, v1: Adt8, v2: u64, v3: Adt28, v4: Adt0, v5: u64) -> R<Adt35> {
    let m985 = v3;
    match m985 {
        Adt28::C0(_) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C1(_) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C2(_) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C3(_, _) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C4(_, _, _) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C5 => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
        Adt28::C6(r996, r997) => {
            let v14 = (*r996).clone();
            let v15 = (*r997).clone();
            if f105(v4.clone()) {
                let v16: bool = if f130(v14.clone()) {
                    false
                } else {
                    f57(v0.clone(), v1.clone(), Adt0::C12(v2), f106(v4.clone()))
                };
                let v17: Adt17 = Adt17::C4({
                    let a986 = v5;
                    let a987 = 1u64;
                    nat_add(a986, a987)?
                });
                let v18: Adt35 = f160(v0, v1, v2, v15, f107(v4), if v16 {
                    let a988 = v5;
                    let a989 = 1u64;
                    nat_add(a988, a989)?
                } else {
                    v5
                })?;
                Ok(Adt35::C0(std::rc::Rc::new(if v16 {
                    Adt11::C14(std::rc::Rc::new(v14.clone()), false, std::rc::Rc::new(f32(0u64)), std::rc::Rc::new(Adt11::C1(Adt16::C4, std::rc::Rc::new(v17.clone()), std::rc::Rc::new(Adt15::C0({
                        let a990 = Str::lit("indirection");
                        utf8_encode(a990)
                    }, None::<Adt13>)))), std::rc::Rc::new(Adt15::C0({
                        let a991 = Str::lit("indirection");
                        utf8_encode(a991)
                    }, None::<Adt13>)), std::rc::Rc::new(match v18.clone() {
                        Adt35::C0(h992, _, _) => {
                            (*h992).clone()
                        }
                    }))
                } else {
                    match v18.clone() {
                        Adt35::C0(h993, _, _) => {
                            (*h993).clone()
                        }
                    }
                }), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(if v16 {
                    Adt28::C1(std::rc::Rc::new(v17))
                } else {
                    v14
                }), std::rc::Rc::new(match v18.clone() {
                    Adt35::C0(_, h994, _) => {
                        (*h994).clone()
                    }
                }))), match v18 {
                    Adt35::C0(_, _, h995) => {
                        h995
                    }
                }))
            } else {
                Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
            }
        }
        Adt28::C7(_) => {
            Ok(Adt35::C0(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt28::C5), v5))
        }
    }
}

pub fn f161(v0: Adt28) -> Adt28 {
    let m998 = v0;
    match m998 {
        Adt28::C0(_) => {
            Adt28::C5
        }
        Adt28::C1(_) => {
            Adt28::C5
        }
        Adt28::C2(r999) => {
            (*r999).clone()
        }
        Adt28::C3(_, _) => {
            Adt28::C5
        }
        Adt28::C4(_, _, _) => {
            Adt28::C5
        }
        Adt28::C5 => {
            Adt28::C5
        }
        Adt28::C6(_, _) => {
            Adt28::C5
        }
        Adt28::C7(_) => {
            Adt28::C5
        }
    }
}

pub fn f162(v0: Adt0) -> R<u64> {
    let m1000 = v0;
    match m1000 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r1003) => {
            let v12 = (*r1003).clone();
            let v13: u64 = f162(v12)?;
            let a1001 = 1u64;
            let a1002 = v13;
            nat_add(a1001, a1002)
        }
    }
}

pub fn f163(v0: Adt0, v1: u64, v2: u64) -> Adt0 {
    let m1004 = f55(v0, v1, v2);
    match m1004 {
        None => {
            Adt0::C14
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f164(v0: Adt0, v1: Adt0) -> Adt0 {
    if f139(v0.clone(), v1.clone()) {
        v0
    } else {
        f181(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f165(v0: Fn1, p1009: u64, p1010: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1008 = (p1009, p1010);
    {
        let mut m1008 = m1008;
        loop {
            let step = {
                let (v1, v2) = m1008;
                let m1005 = v1;
                if m1005 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1005 - 1;
                    let m1007 = {
                        let c1006: Fn1 = v0.clone();
                        c1006.apply(v2)?
                    };
                    match m1007 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1008 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f182(v0, v1)
}

pub fn f167(v0: u8, v1: u8) -> R<Option<u64>> {
    let m1011 = f183(v0)?;
    match m1011 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1012 = f183(v1)?;
            match m1012 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    Ok(Some({
                        let a1015 = {
                            let a1013 = v2;
                            let a1014 = 16u64;
                            nat_mul(a1013, a1014)?
                        };
                        let a1016 = v3;
                        nat_add(a1015, a1016)?
                    }))
                }
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1020 = {
        let a1017 = v0;
        let a1018 = v1;
        let a1019 = v2;
        slice_bytes(a1017, a1018, a1019)
    };
    match m1020 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f169(_: u64) -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f170(v0: Adt13) -> u64 {
    let m1021 = v0;
    match m1021 {
        Adt13::C0 => {
            0u64
        }
        Adt13::C1 => {
            1u64
        }
        Adt13::C2 => {
            2u64
        }
        Adt13::C3 => {
            3u64
        }
        Adt13::C4 => {
            4u64
        }
        Adt13::C5 => {
            5u64
        }
        Adt13::C6 => {
            6u64
        }
        Adt13::C7 => {
            7u64
        }
    }
}

pub fn f171(v0: Adt33, v1: u64, v2: Bytes) -> Adt33 {
    Adt33::C0(List::cons((v1, {
        let a1022 = v2.clone();
        length_bytes(a1022)
    }), match v0.clone() {
        Adt33::C0(h1023, _, _, _) => {
            h1023
        }
    }), List::cons(v2, match v0.clone() {
        Adt33::C0(_, h1024, _, _) => {
            h1024
        }
    }), match v0.clone() {
        Adt33::C0(_, _, h1025, _) => {
            h1025
        }
    }, match v0 {
        Adt33::C0(_, _, _, h1026) => {
            h1026
        }
    })
}

pub fn f172(v0: Adt14) -> u64 {
    let m1027 = v0;
    match m1027 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1 => {
            1u64
        }
        Adt14::C2 => {
            2u64
        }
    }
}

pub fn f173(p1032: List<(Adt17, u64)>, v1: Adt17) -> Option<u64> {
    let m1031 = p1032;
    {
        let mut m1031 = m1031;
        loop {
            let step = {
                let v0 = m1031;
                let m1028 = v0;
                match m1028.uncons() {
                    None => {
                        Err::<List<(Adt17, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f98({
                            let (h1029, _) = v2;
                            h1029
                        }, v1.clone()) {
                            Err::<List<(Adt17, u64)>, Option<u64>>(Some({
                                let (_, h1030) = v2;
                                h1030
                            }))
                        } else {
                            Ok::<List<(Adt17, u64)>, Option<u64>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Option<Adt13>) -> R<u64> {
    let m1033 = v0;
    match m1033 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            let a1034 = 1u64;
            let a1035 = f170(v1);
            nat_add(a1034, a1035)
        }
    }
}

pub fn f175(v0: Adt29) -> u64 {
    let m1036 = v0;
    match m1036 {
        Adt29::C0 => {
            0u64
        }
        Adt29::C1 => {
            1u64
        }
        Adt29::C2 => {
            2u64
        }
    }
}

pub fn f176(v0: Adt30) -> u64 {
    let m1037 = v0;
    match m1037 {
        Adt30::C0 => {
            0u64
        }
        Adt30::C1 => {
            1u64
        }
        Adt30::C2 => {
            2u64
        }
    }
}

pub fn f177(v0: Adt23) -> u64 {
    let m1038 = v0;
    match m1038 {
        Adt23::C0 => {
            0u64
        }
        Adt23::C1 => {
            1u64
        }
        Adt23::C2 => {
            2u64
        }
        Adt23::C3 => {
            3u64
        }
        Adt23::C4 => {
            4u64
        }
        Adt23::C5 => {
            5u64
        }
        Adt23::C6 => {
            6u64
        }
        Adt23::C7 => {
            7u64
        }
        Adt23::C8 => {
            8u64
        }
    }
}

pub fn f178(v0: Adt24) -> u64 {
    let m1039 = v0;
    match m1039 {
        Adt24::C0 => {
            0u64
        }
        Adt24::C1 => {
            1u64
        }
        Adt24::C2 => {
            2u64
        }
        Adt24::C3 => {
            3u64
        }
        Adt24::C4 => {
            4u64
        }
    }
}

pub fn f179(v0: Adt5) -> R<u64> {
    let m1040 = v0;
    match m1040 {
        Adt5::C0 => {
            Ok(0u64)
        }
        Adt5::C1 => {
            Ok(1u64)
        }
        Adt5::C2 => {
            Ok(2u64)
        }
        Adt5::C3 => {
            Ok(3u64)
        }
        Adt5::C4 => {
            Ok(4u64)
        }
        Adt5::C5 => {
            Ok(5u64)
        }
        Adt5::C6 => {
            Ok(6u64)
        }
        Adt5::C7 => {
            Ok(7u64)
        }
        Adt5::C8 => {
            Ok(8u64)
        }
        Adt5::C9 => {
            Ok(9u64)
        }
        Adt5::C10 => {
            Ok(10u64)
        }
        Adt5::C11 => {
            Ok(11u64)
        }
        Adt5::C12 => {
            Ok(12u64)
        }
        Adt5::C13 => {
            Ok(13u64)
        }
        Adt5::C14 => {
            Ok(14u64)
        }
        Adt5::C15(v1) => {
            let a1041 = 16u64;
            let a1042 = v1;
            nat_add(a1041, a1042)
        }
    }
}

pub fn f180(v0: Adt28, v1: Adt11) -> R<bool> {
    if f30(f25(v1.clone())) {
        f184(v0, f26(v1))
    } else {
        Ok(false)
    }
}

pub fn f181(v0: Adt0, v1: Adt0) -> Adt0 {
    let m1043 = v0;
    match m1043 {
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
        Adt0::C15(r1044, r1045) => {
            let v12 = (*r1044).clone();
            let v13 = (*r1045).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f181(v13, v1)))
        }
    }
}

pub fn f182(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1047 = f140(v0, {
        let (h1046, _) = v1;
        h1046
    });
    match m1047 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let m1048 = f185(v2)?;
            match m1048 {
                None => {
                    Ok(None::<(u64, u64)>)
                }
                Some(v3) => {
                    Ok(Some(({
                        let a1050 = {
                            let (h1049, _) = v1;
                            h1049
                        };
                        let a1051 = 1u64;
                        nat_add(a1050, a1051)?
                    }, {
                        let a1055 = {
                            let a1053 = {
                                let (_, h1052) = v1;
                                h1052
                            };
                            let a1054 = 10u64;
                            nat_mul(a1053, a1054)?
                        };
                        let a1056 = v3;
                        nat_add(a1055, a1056)?
                    })))
                }
            }
        }
    }
}

pub fn f183(v0: u8) -> R<Option<u64>> {
    let m1057 = f185(v0)?;
    match m1057 {
        None => {
            let v3: u64 = {
                let v2: u8 = v0;
                let a1102 = {
                    let a1078 = {
                        let a1066 = {
                            let a1060 = {
                                let a1058 = v2;
                                let a1059 = 128u8;
                                fixed_u8::bit_and(a1058, a1059)
                            };
                            let a1061 = 128u8;
                            if equal(a1060, a1061) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1067 = {
                            let a1064 = {
                                let a1062 = v2;
                                let a1063 = 64u8;
                                fixed_u8::bit_and(a1062, a1063)
                            };
                            let a1065 = 64u8;
                            if equal(a1064, a1065) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1066, a1067)?
                    };
                    let a1079 = {
                        let a1076 = {
                            let a1070 = {
                                let a1068 = v2;
                                let a1069 = 32u8;
                                fixed_u8::bit_and(a1068, a1069)
                            };
                            let a1071 = 32u8;
                            if equal(a1070, a1071) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1077 = {
                            let a1074 = {
                                let a1072 = v2;
                                let a1073 = 16u8;
                                fixed_u8::bit_and(a1072, a1073)
                            };
                            let a1075 = 16u8;
                            if equal(a1074, a1075) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1076, a1077)?
                    };
                    nat_add(a1078, a1079)?
                };
                let a1103 = {
                    let a1100 = {
                        let a1088 = {
                            let a1082 = {
                                let a1080 = v2;
                                let a1081 = 8u8;
                                fixed_u8::bit_and(a1080, a1081)
                            };
                            let a1083 = 8u8;
                            if equal(a1082, a1083) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1089 = {
                            let a1086 = {
                                let a1084 = v2;
                                let a1085 = 4u8;
                                fixed_u8::bit_and(a1084, a1085)
                            };
                            let a1087 = 4u8;
                            if equal(a1086, a1087) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1088, a1089)?
                    };
                    let a1101 = {
                        let a1098 = {
                            let a1092 = {
                                let a1090 = v2;
                                let a1091 = 2u8;
                                fixed_u8::bit_and(a1090, a1091)
                            };
                            let a1093 = 2u8;
                            if equal(a1092, a1093) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1099 = {
                            let a1096 = {
                                let a1094 = v2;
                                let a1095 = 1u8;
                                fixed_u8::bit_and(a1094, a1095)
                            };
                            let a1097 = 1u8;
                            if equal(a1096, a1097) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1098, a1099)?
                    };
                    nat_add(a1100, a1101)?
                };
                nat_add(a1102, a1103)?
            };
            let a1104 = v3;
            let a1105 = 97u64;
            if nat_lt(a1104, a1105) {
                Ok(None::<u64>)
            } else {
                let a1106 = v3;
                let a1107 = 103u64;
                if nat_lt(a1106, a1107) {
                    Ok(Some({
                        let a1108 = v3;
                        let a1109 = 87u64;
                        nat_sub(a1108, a1109)
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f184(v0: Adt28, v1: Adt11) -> R<bool> {
    let m1110 = v0;
    match m1110 {
        Adt28::C0(v2) => {
            let a1111 = f175(v2);
            let a1112 = 2u64;
            if nat_eq(a1111, a1112) {
                let a1113 = f186(v1);
                let a1114 = 0u64;
                Ok(nat_eq(a1113, a1114))
            } else {
                Ok(false)
            }
        }
        Adt28::C1(_) => {
            Ok(false)
        }
        Adt28::C2(_) => {
            Ok(false)
        }
        Adt28::C3(v5, r1119) => {
            let v6 = (*r1119).clone();
            let a1117 = f186(v1.clone());
            let a1118 = {
                let a1115 = 1u64;
                let a1116 = f176(v5);
                nat_add(a1115, a1116)?
            };
            if nat_eq(a1117, a1118) {
                Ok(f187(v6, f188(v1)))
            } else {
                Ok(false)
            }
        }
        Adt28::C4(v7, v8, r1120) => {
            let v9 = (*r1120).clone();
            if f189(v1.clone(), v7, v8) {
                Ok(f190(v9, f188(v1)))
            } else {
                Ok(false)
            }
        }
        Adt28::C5 => {
            Ok(false)
        }
        Adt28::C6(_, _) => {
            Ok(false)
        }
        Adt28::C7(v12) => {
            Ok(f191(v1, v12))
        }
    }
}

pub fn f185(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1165 = {
            let a1141 = {
                let a1129 = {
                    let a1123 = {
                        let a1121 = v1;
                        let a1122 = 128u8;
                        fixed_u8::bit_and(a1121, a1122)
                    };
                    let a1124 = 128u8;
                    if equal(a1123, a1124) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1130 = {
                    let a1127 = {
                        let a1125 = v1;
                        let a1126 = 64u8;
                        fixed_u8::bit_and(a1125, a1126)
                    };
                    let a1128 = 64u8;
                    if equal(a1127, a1128) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1129, a1130)?
            };
            let a1142 = {
                let a1139 = {
                    let a1133 = {
                        let a1131 = v1;
                        let a1132 = 32u8;
                        fixed_u8::bit_and(a1131, a1132)
                    };
                    let a1134 = 32u8;
                    if equal(a1133, a1134) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1140 = {
                    let a1137 = {
                        let a1135 = v1;
                        let a1136 = 16u8;
                        fixed_u8::bit_and(a1135, a1136)
                    };
                    let a1138 = 16u8;
                    if equal(a1137, a1138) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1139, a1140)?
            };
            nat_add(a1141, a1142)?
        };
        let a1166 = {
            let a1163 = {
                let a1151 = {
                    let a1145 = {
                        let a1143 = v1;
                        let a1144 = 8u8;
                        fixed_u8::bit_and(a1143, a1144)
                    };
                    let a1146 = 8u8;
                    if equal(a1145, a1146) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1152 = {
                    let a1149 = {
                        let a1147 = v1;
                        let a1148 = 4u8;
                        fixed_u8::bit_and(a1147, a1148)
                    };
                    let a1150 = 4u8;
                    if equal(a1149, a1150) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1151, a1152)?
            };
            let a1164 = {
                let a1161 = {
                    let a1155 = {
                        let a1153 = v1;
                        let a1154 = 2u8;
                        fixed_u8::bit_and(a1153, a1154)
                    };
                    let a1156 = 2u8;
                    if equal(a1155, a1156) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1162 = {
                    let a1159 = {
                        let a1157 = v1;
                        let a1158 = 1u8;
                        fixed_u8::bit_and(a1157, a1158)
                    };
                    let a1160 = 1u8;
                    if equal(a1159, a1160) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1161, a1162)?
            };
            nat_add(a1163, a1164)?
        };
        nat_add(a1165, a1166)?
    };
    let a1167 = v2;
    let a1168 = 48u64;
    if nat_lt(a1167, a1168) {
        Ok(None::<u64>)
    } else {
        let a1169 = v2;
        let a1170 = 58u64;
        if nat_lt(a1169, a1170) {
            Ok(Some({
                let a1171 = v2;
                let a1172 = 48u64;
                nat_sub(a1171, a1172)
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f186(v0: Adt11) -> u64 {
    let m1173 = v0;
    match m1173 {
        Adt11::C0(_, _) => {
            99u64
        }
        Adt11::C1(_, _, _) => {
            99u64
        }
        Adt11::C2(_, _, _) => {
            99u64
        }
        Adt11::C3(_, _, _, _) => {
            99u64
        }
        Adt11::C4(_, _, _, _) => {
            99u64
        }
        Adt11::C5(r1174, _, _) => {
            let v17 = (*r1174).clone();
            f192(v17)
        }
        Adt11::C6(_, _, _) => {
            99u64
        }
        Adt11::C7(_, _, _, _) => {
            99u64
        }
        Adt11::C8(_, _, _) => {
            99u64
        }
        Adt11::C9(_) => {
            99u64
        }
        Adt11::C10(_, _, _) => {
            99u64
        }
        Adt11::C11(_, _) => {
            99u64
        }
        Adt11::C12 => {
            99u64
        }
        Adt11::C13(_, _) => {
            99u64
        }
        Adt11::C14(_, _, _, _, _, _) => {
            99u64
        }
        Adt11::C15(_, _, _) => {
            99u64
        }
        Adt11::C16(_, _) => {
            99u64
        }
    }
}

pub fn f187(v0: Adt28, v1: Adt11) -> bool {
    let m1175 = v1;
    match m1175 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(r1176, r1177) => {
            let v37 = (*r1176).clone();
            let v38 = (*r1177).clone();
            if f30(v38) {
                f193(v0, v37)
            } else {
                false
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f188(v0: Adt11) -> Adt11 {
    let m1178 = v0;
    match m1178 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, r1179, _) => {
            (*r1179).clone()
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f189(v0: Adt11, v1: u64, v2: u64) -> bool {
    let m1180 = v0;
    match m1180 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(r1186, _, _) => {
            let v19 = (*r1186).clone();
            let m1181 = v19;
            match m1181 {
                Adt21::C0(_) => {
                    false
                }
                Adt21::C1 => {
                    false
                }
                Adt21::C2(_, _) => {
                    false
                }
                Adt21::C3(_, _) => {
                    false
                }
                Adt21::C4(v27, v28) => {
                    let a1182 = v1;
                    let a1183 = v27;
                    if nat_eq(a1182, a1183) {
                        let a1184 = v2;
                        let a1185 = v28;
                        nat_eq(a1184, a1185)
                    } else {
                        false
                    }
                }
                Adt21::C5(_, _) => {
                    false
                }
                Adt21::C6 => {
                    false
                }
                Adt21::C7(_) => {
                    false
                }
            }
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f190(p1194: Adt28, p1195: Adt11) -> bool {
    let m1193 = (p1194, p1195);
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v0, v1) = m1193;
                let m1187 = v0;
                match m1187 {
                    Adt28::C0(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C1(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C2(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C3(_, _) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C4(_, _, _) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C5 => {
                        Err::<(Adt28, Adt11), bool>(f30(v1))
                    }
                    Adt28::C6(r1191, r1192) => {
                        let v10 = (*r1191).clone();
                        let v11 = (*r1192).clone();
                        let m1188 = v1;
                        match m1188 {
                            Adt11::C0(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C1(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C2(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C3(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C4(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C5(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C6(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C7(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C8(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C9(_) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C10(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C11(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C12 => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C13(r1189, r1190) => {
                                let v47 = (*r1189).clone();
                                let v48 = (*r1190).clone();
                                if f193(v10, v47) {
                                    Ok::<(Adt28, Adt11), bool>((v11, v48))
                                } else {
                                    Err::<(Adt28, Adt11), bool>(false)
                                }
                            }
                            Adt11::C14(_, _, _, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C15(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C16(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                        }
                    }
                    Adt28::C7(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt11, v1: Adt14) -> bool {
    let m1196 = v0;
    match m1196 {
        Adt11::C0(r1198, _) => {
            let v2 = (*r1198).clone();
            let m1197 = v2;
            match m1197 {
                Adt12::C0 => {
                    false
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(v13) => {
                    f194(v1, v13)
                }
                Adt12::C8 => {
                    false
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f192(v0: Adt21) -> u64 {
    let m1199 = v0;
    match m1199 {
        Adt21::C0(_) => {
            0u64
        }
        Adt21::C1 => {
            1u64
        }
        Adt21::C2(_, _) => {
            2u64
        }
        Adt21::C3(_, _) => {
            3u64
        }
        Adt21::C4(_, _) => {
            4u64
        }
        Adt21::C5(_, _) => {
            5u64
        }
        Adt21::C6 => {
            6u64
        }
        Adt21::C7(_) => {
            7u64
        }
    }
}

pub fn f193(v0: Adt28, v1: Adt11) -> bool {
    let m1200 = v0;
    match m1200 {
        Adt28::C0(v2) => {
            let a1201 = f175(v2);
            let a1202 = 0u64;
            if nat_eq(a1201, a1202) {
                f121(v1)
            } else {
                false
            }
        }
        Adt28::C1(r1207) => {
            let v3 = (*r1207).clone();
            let m1203 = v1;
            match m1203 {
                Adt11::C0(_, _) => {
                    false
                }
                Adt11::C1(v6, r1206, _) => {
                    let v7 = (*r1206).clone();
                    let a1204 = f148(v6);
                    let a1205 = 0u64;
                    if nat_eq(a1204, a1205) {
                        f98(v3, v7)
                    } else {
                        false
                    }
                }
                Adt11::C2(_, _, _) => {
                    false
                }
                Adt11::C3(_, _, _, _) => {
                    false
                }
                Adt11::C4(_, _, _, _) => {
                    false
                }
                Adt11::C5(_, _, _) => {
                    false
                }
                Adt11::C6(_, _, _) => {
                    false
                }
                Adt11::C7(_, _, _, _) => {
                    false
                }
                Adt11::C8(_, _, _) => {
                    false
                }
                Adt11::C9(_) => {
                    false
                }
                Adt11::C10(_, _, _) => {
                    false
                }
                Adt11::C11(_, _) => {
                    false
                }
                Adt11::C12 => {
                    false
                }
                Adt11::C13(_, _) => {
                    false
                }
                Adt11::C14(_, _, _, _, _, _) => {
                    false
                }
                Adt11::C15(_, _, _) => {
                    false
                }
                Adt11::C16(_, _) => {
                    false
                }
            }
        }
        Adt28::C2(_) => {
            false
        }
        Adt28::C3(_, _) => {
            false
        }
        Adt28::C4(_, _, _) => {
            false
        }
        Adt28::C5 => {
            false
        }
        Adt28::C6(_, _) => {
            false
        }
        Adt28::C7(_) => {
            false
        }
    }
}

pub fn f194(v0: Adt14, v1: Adt14) -> bool {
    let a1208 = f172(v0);
    let a1209 = f172(v1);
    nat_eq(a1208, a1209)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: Adt0, p8: List<bool>, p9: List<bool>, p10: Adt7, p11: u64) -> R<Adt10> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
}
