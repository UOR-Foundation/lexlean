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
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Fn0 {
    F19,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn0::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21(std::rc::Rc<Adt9>, std::rc::Rc<Adt8>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt10>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F21(k0, k1, k2, k3, k4) => f21((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: List<bool>) -> R<Adt7> {
    let v8: Adt10 = Adt10::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5.clone()));
    let v9: Adt11 = f1(v8.clone(), Adt11::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt9::C0)), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v10: Adt0 = f2(match v9.clone() {
        Adt11::C0(h1, _) => {
            (*h1).clone()
        }
    }, v1.clone());
    let v11: Adt11 = f1(v8.clone(), Adt11::C0(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(Adt9::C0)), v5, 0u64, v7, v6)?;
    let v12: Adt0 = f3(f4(f5(v1)?), v10.clone());
    let v13: Adt8 = f6(v8.clone(), v10.clone(), match v9.clone() {
        Adt11::C0(_, h2) => {
            (*h2).clone()
        }
    }, v12.clone(), Adt8::C0);
    let v14: Adt8 = f7(v13, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v12.clone())?;
            let a4 = f5(v12.clone())?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    Ok(Adt7::C0(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v14.clone()), std::rc::Rc::new(f8(v8, v10, match v9 {
        Adt11::C0(_, h7) => {
            (*h7).clone()
        }
    }, v14, v12)?), std::rc::Rc::new(f9(match v11 {
        Adt11::C0(_, h8) => {
            (*h8).clone()
        }
    }, Adt9::C0))))
}

pub fn f1(v0: Adt10, p19: Adt11, p20: Adt2, p21: u64, v4: List<bool>, p22: Adt0) -> R<Adt11> {
    let m18 = (p19, (p20, (p21, p22)));
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m18;
                let m9 = v2;
                match m9 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                    Adt2::C14(r16, r17) => {
                        let v34 = (*r16).clone();
                        let v35 = (*r17).clone();
                        let v37: bool = {
                            let m10 = f10(v4.clone(), v3);
                            match m10 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a11 = v36;
                                    bool_not(a11)
                                }
                            }
                        };
                        let v38: Adt11 = Adt11::C0(std::rc::Rc::new(f11(v0.clone(), match v1.clone() {
                            Adt11::C0(h12, _) => {
                                (*h12).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt11::C0(_, h13) => {
                                (*h13).clone()
                            }
                        }));
                        let v39: Adt11 = if v37 {
                            f12(v0.clone(), v38, v34, f13(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>((v39, (v35, ({
                            let a14 = v3;
                            let a15 = 1u64;
                            nat_add(a14, a15)?
                        }, f14(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt11, (Adt2, (u64, Adt0))), R<Adt11>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p27: Adt0, p28: Adt0) -> Adt0 {
    let m26 = (p27, p28);
    {
        let mut m26 = m26;
        loop {
            let step = {
                let (v0, v1) = m26;
                let m23 = v1;
                match m23 {
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
                    Adt0::C15(r24, r25) => {
                        let v12 = (*r24).clone();
                        let v13 = (*r25).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f15(v0, v12), v13))
                    }
                }
            };
            match step {
                Ok(next) => { m26 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt0, v1: Adt0) -> Adt0 {
    let m29 = v0;
    match m29 {
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
        Adt0::C15(r30, r31) => {
            let v12 = (*r30).clone();
            let v13 = (*r31).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f3(v13, v1)))
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let m32 = v0;
    if m32 == 0 {
        Adt0::C14
    } else {
        let v1 = m32 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f4(v1)))
    }
}

pub fn f5(v0: Adt0) -> R<u64> {
    let m33 = v0;
    match m33 {
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
        Adt0::C15(_, r36) => {
            let v12 = (*r36).clone();
            let v13: u64 = f5(v12)?;
            let a34 = 1u64;
            let a35 = v13;
            nat_add(a34, a35)
        }
    }
}

pub fn f6(v0: Adt10, v1: Adt0, v2: Adt9, p41: Adt0, p42: Adt8) -> Adt8 {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v3, v4) = m40;
                let m37 = v3;
                match m37 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v4)
                    }
                    Adt0::C15(r38, r39) => {
                        let v15 = (*r38).clone();
                        let v16 = (*r39).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v16, f16(v1.clone(), v15.clone(), f17(v0.clone(), v2.clone(), v15), v4)))
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

pub fn f7(v0: Adt8, v1: u64) -> R<Adt8> {
    let (h43, _) = f18(Fn0::F19, v1, v0)?;
    Ok(h43)
}

pub fn f8(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0) -> R<Adt0> {
    let (h44, _) = f20(Fn1::F21(std::rc::Rc::new(v2), std::rc::Rc::new(v3), std::rc::Rc::new(v1), std::rc::Rc::new(v4.clone()), std::rc::Rc::new(v0)), {
        let a45 = 1u64;
        let a46 = f5(v4)?;
        nat_add(a45, a46)?
    }, Adt0::C14)?;
    Ok(h44)
}

pub fn f9(p52: Adt9, p53: Adt9) -> Adt9 {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v0, v1) = m51;
                let m47 = v0;
                match m47 {
                    Adt9::C0 => {
                        Err::<(Adt9, Adt9), Adt9>(v1)
                    }
                    Adt9::C1(r48, v3, r49, r50) => {
                        let v2 = (*r48).clone();
                        let v4 = (*r49).clone();
                        let v5 = (*r50).clone();
                        Ok::<(Adt9, Adt9), Adt9>((v5, Adt9::C1(std::rc::Rc::new(v2), v3, std::rc::Rc::new(v4), std::rc::Rc::new(v1))))
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

pub fn f10(p57: List<bool>, p58: u64) -> Option<bool> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v0, v1) = m56;
                let m54 = v0;
                match m54.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m55 = v1;
                        if m55 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m55 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Adt10, v1: Adt0, v2: u64) -> Adt0 {
    f22(f23(v1, f24(v0.clone(), v2)), f25(v0, v2))
}

pub fn f12(v0: Adt10, v1: Adt11, v2: Adt2, v3: Adt0) -> R<Adt11> {
    let m59 = v2;
    match m59 {
        Adt2::C0(r70, _) => {
            let v4 = (*r70).clone();
            Ok(Adt11::C0(std::rc::Rc::new(f22(match v1.clone() {
                Adt11::C0(h60, _) => {
                    (*h60).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt11::C0(_, h61) => {
                    (*h61).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r71, r72, r73) => {
            let v8 = (*r71).clone();
            let v9 = (*r72).clone();
            let v10 = (*r73).clone();
            let v11: Adt11 = Adt11::C0(std::rc::Rc::new(f22(match v1.clone() {
                Adt11::C0(h62, _) => {
                    (*h62).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt11::C0(_, h63) => {
                    (*h63).clone()
                }
            }));
            let v12: Adt11 = f12(v0.clone(), v11, v9, f13(v3.clone()))?;
            f12(v0, v12, v10, f14(v3))
        }
        Adt2::C3(r74, r75, r76) => {
            let v13 = (*r74).clone();
            let v14 = (*r75).clone();
            let v15 = (*r76).clone();
            let v16: Adt11 = f12(v0.clone(), v1, v13, f13(v3.clone()))?;
            let v17: Adt11 = f12(v0.clone(), v16, v14, f14(v3.clone()))?;
            f12(v0, v17, v15, f26(v3))
        }
        Adt2::C4(r77, r78, r79) => {
            let v18 = (*r77).clone();
            let v19 = (*r78).clone();
            let v20 = (*r79).clone();
            let v21: Adt11 = Adt11::C0(std::rc::Rc::new(f22(match v1.clone() {
                Adt11::C0(h64, _) => {
                    (*h64).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt11::C0(_, h65) => {
                    (*h65).clone()
                }
            }));
            let v22: Adt11 = f12(v0.clone(), v21, v19, f13(v3.clone()))?;
            f12(v0, v22, v20, f14(v3))
        }
        Adt2::C5(_, r80, r81) => {
            let v24 = (*r80).clone();
            let v25 = (*r81).clone();
            let v26: Adt11 = Adt11::C0(std::rc::Rc::new(f22(match v1.clone() {
                Adt11::C0(h66, _) => {
                    (*h66).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt11::C0(_, h67) => {
                    (*h67).clone()
                }
            }));
            f12(v0, v26, v25, f13(v3))
        }
        Adt2::C6(_, r82) => {
            let v28 = (*r82).clone();
            f12(v0, v1, v28, f13(v3))
        }
        Adt2::C7(v29, r83) => {
            let v30 = (*r83).clone();
            let v31: Adt0 = f24(v0.clone(), v29);
            let v32: u64 = f27(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f28(v31.clone(), v32)), std::rc::Rc::new(f25(v0.clone(), v29)));
            let v34: Adt11 = Adt11::C0(std::rc::Rc::new(f22(match v1.clone() {
                Adt11::C0(h68, _) => {
                    (*h68).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f29(match v1 {
                Adt11::C0(_, h69) => {
                    (*h69).clone()
                }
            }, v33, v29, f30(v31, v32))));
            f12(v0, v34, v30, f13(v3))
        }
        Adt2::C8(r84, r85) => {
            let v35 = (*r84).clone();
            let v36 = (*r85).clone();
            let v37: Adt11 = f12(v0.clone(), v1, v35, f13(v3.clone()))?;
            f12(v0, v37, v36, f14(v3))
        }
        Adt2::C9(_, r86) => {
            let v39 = (*r86).clone();
            f12(v0, v1, v39, f13(v3))
        }
        Adt2::C10(r87) => {
            let v40 = (*r87).clone();
            f12(v0, v1, v40, f13(v3))
        }
        Adt2::C11(r88) => {
            let v41 = (*r88).clone();
            f12(v0, v1, v41, f13(v3))
        }
        Adt2::C12(r89, _) => {
            let v42 = (*r89).clone();
            f12(v0, v1, v42, f13(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r90, r91) => {
            let v44 = (*r90).clone();
            let v45 = (*r91).clone();
            let v46: Adt11 = f12(v0.clone(), v1, v44, f13(v3.clone()))?;
            f12(v0, v46, v45, f14(v3))
        }
        Adt2::C15(_, _, r92) => {
            let v49 = (*r92).clone();
            f12(v0, v1, v49, f13(v3))
        }
    }
}

pub fn f13(v0: Adt0) -> Adt0 {
    f31(f32(v0))
}

pub fn f14(v0: Adt0) -> Adt0 {
    f31(f32(f32(v0)))
}

pub fn f15(p97: Adt0, p98: Adt0) -> Adt0 {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v0, v1) = m96;
                let m93 = v1;
                match m93 {
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
                    Adt0::C15(r94, r95) => {
                        let v12 = (*r94).clone();
                        let v13 = (*r95).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f23(v0, v12), v13))
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

pub fn f16(v0: Adt0, v1: Adt0, p103: Adt0, p104: Adt8) -> Adt8 {
    let m102 = (p103, p104);
    {
        let mut m102 = m102;
        loop {
            let step = {
                let (v2, v3) = m102;
                let m99 = v2;
                match m99 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C15(r100, r101) => {
                        let v14 = (*r100).clone();
                        let v15 = (*r101).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v15, f33(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt10, v1: Adt9, v2: Adt0) -> Adt0 {
    let m105 = v2;
    match m105 {
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
        Adt0::C12(v10) => {
            let m106 = f34(v0, v10);
            match m106 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r107, r108) => {
            let v12 = (*r107).clone();
            let v13 = (*r108).clone();
            f35(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f18(v0: Fn0, p113: u64, p114: Adt8) -> R<(Adt8, bool)> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m109 = v1;
                if m109 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m109 - 1;
                    let m111 = {
                        let c110: Fn0 = v0.clone();
                        c110.apply(v2.clone())?
                    };
                    match m111 {
                        None => {
                            Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), R<(Adt8, bool)>>((v3, v4))
                        }
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

pub fn f19(v0: Adt8) -> R<Option<Adt8>> {
    f36(v0)
}

pub fn f20(v0: Fn1, p119: u64, p120: Adt0) -> R<(Adt0, bool)> {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m115 = v1;
                if m115 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m115 - 1;
                    let m117 = {
                        let c116: Fn1 = v0.clone();
                        c116.apply(v2.clone())?
                    };
                    match m117 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
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

pub fn f21(v0: Adt9, v1: Adt8, v2: Adt0, v3: Adt0, v4: Adt10, v5: Adt0) -> R<Option<Adt0>> {
    f37(v4, v2, v0, v1, v3, v5)
}

pub fn f22(v0: Adt0, v1: Adt0) -> Adt0 {
    let m121 = v1;
    match m121 {
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
        Adt0::C8(r122) => {
            let v3 = (*r122).clone();
            f22(v0, v3)
        }
        Adt0::C9(r123, r124) => {
            let v4 = (*r123).clone();
            let v5 = (*r124).clone();
            f22(f22(v0, v4), v5)
        }
        Adt0::C10(r125) => {
            let v6 = (*r125).clone();
            f22(v0, v6)
        }
        Adt0::C11(r126, r127) => {
            let v7 = (*r126).clone();
            let v8 = (*r127).clone();
            f22(f22(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r128, r129) => {
            let v10 = (*r128).clone();
            let v11 = (*r129).clone();
            let v12: Adt0 = f38(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f22(f22(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r130, r131) => {
            let v13 = (*r130).clone();
            let v14 = (*r131).clone();
            f22(f22(v0, v13), v14)
        }
    }
}

pub fn f23(p136: Adt0, p137: Adt0) -> Adt0 {
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
                        Ok::<(Adt0, Adt0), Adt0>((f22(v0, v2), v3))
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

pub fn f24(v0: Adt10, v1: u64) -> Adt0 {
    let m138 = f39(v0, v1);
    match m138 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f25(v0: Adt10, v1: u64) -> Adt0 {
    let m140 = f40(match v0 {
        Adt10::C0(_, _, _, _, h139, _) => {
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

pub fn f26(v0: Adt0) -> Adt0 {
    f31(f32(f32(f32(v0))))
}

pub fn f27(v0: Adt2) -> R<u64> {
    let m141 = v0;
    match m141 {
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
        Adt2::C14(_, r144) => {
            let v33 = (*r144).clone();
            let v34: u64 = f27(v33)?;
            let a142 = 1u64;
            let a143 = v34;
            nat_add(a142, a143)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f28(p150: Adt0, p151: u64) -> Adt0 {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v0, v1) = m149;
                let m145 = v0;
                match m145 {
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
                    Adt0::C15(r147, r148) => {
                        let v12 = (*r147).clone();
                        let v13 = (*r148).clone();
                        let m146 = v1;
                        if m146 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m146 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f41(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f30(v0: Adt0, v1: u64) -> Adt0 {
    let m152 = v0;
    match m152 {
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
        Adt0::C15(r154, r155) => {
            let v12 = (*r154).clone();
            let v13 = (*r155).clone();
            let m153 = v1;
            if m153 == 0 {
                Adt0::C14
            } else {
                let v14 = m153 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f30(v13, v14)))
            }
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    let m156 = f42(v0);
    match m156 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    let m157 = f43(v0);
    match m157 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0, p162: Adt0, p163: Adt8) -> Adt8 {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v2, v3) = m161;
                let m158 = v2;
                match m158 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v3)
                    }
                    Adt0::C15(r159, r160) => {
                        let v14 = (*r159).clone();
                        let v15 = (*r160).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v15, f44(v1.clone(), f45(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Adt10, v1: u64) -> Option<Adt0> {
    f40(match v0 {
        Adt10::C0(_, h164, _, _, _, _) => {
            (*h164).clone()
        }
    }, v1)
}

pub fn f35(v0: Adt9, v1: Adt0) -> Adt0 {
    let m165 = v0;
    match m165 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(r166, _, r167, r168) => {
            let v2 = (*r166).clone();
            let v4 = (*r167).clone();
            let v5 = (*r168).clone();
            if f46(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f35(v5, v1)))
            } else {
                f35(v5, v1)
            }
        }
    }
}

pub fn f36(v0: Adt8) -> R<Option<Adt8>> {
    let v1: Adt8 = f47(v0.clone(), v0.clone(), v0.clone());
    let a169 = f48(v1.clone())?;
    let a170 = f48(v0)?;
    if nat_eq(a169, a170) {
        Ok(None::<Adt8>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f37(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f49(v0, v1, v2, v3, v4, v5.clone());
    let a171 = f5(v6.clone())?;
    let a172 = f5(v5)?;
    if nat_eq(a171, a172) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f38(v0: Adt0, v1: Adt0) -> Adt0 {
    if f50(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f39(v0: Adt10, v1: u64) -> Option<Adt0> {
    f40(match v0 {
        Adt10::C0(_, _, _, h173, _, _) => {
            (*h173).clone()
        }
    }, v1)
}

pub fn f40(p179: Adt0, p180: u64) -> Option<Adt0> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v0, v1) = m178;
                let m174 = v0;
                match m174 {
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
                    Adt0::C15(r176, r177) => {
                        let v12 = (*r176).clone();
                        let v13 = (*r177).clone();
                        let m175 = v1;
                        if m175 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m175 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(p187: Adt9, v1: Adt0, v2: u64) -> bool {
    let m186 = p187;
    {
        let mut m186 = m186;
        loop {
            let step = {
                let v0 = m186;
                let m181 = v0;
                match m181 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(r184, v4, _, r185) => {
                        let v3 = (*r184).clone();
                        let v6 = (*r185).clone();
                        let a182 = v4;
                        let a183 = v2;
                        if nat_eq(a182, a183) {
                            if f46(v3, v1.clone()) {
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
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Adt0) -> Option<Adt0> {
    let m188 = v0;
    match m188 {
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
        Adt0::C8(r189) => {
            let v2 = (*r189).clone();
            Some(v2)
        }
        Adt0::C9(r190, _) => {
            let v3 = (*r190).clone();
            Some(v3)
        }
        Adt0::C10(r191) => {
            let v5 = (*r191).clone();
            Some(v5)
        }
        Adt0::C11(r192, _) => {
            let v6 = (*r192).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r193, _) => {
            let v9 = (*r193).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r194, _) => {
            let v11 = (*r194).clone();
            Some(v11)
        }
    }
}

pub fn f43(v0: Adt0) -> Option<Adt0> {
    let m195 = v0;
    match m195 {
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
        Adt0::C9(_, r196) => {
            let v4 = (*r196).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r197) => {
            let v7 = (*r197).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r198) => {
            let v10 = (*r198).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r199) => {
            let v12 = (*r199).clone();
            Some(v12)
        }
    }
}

pub fn f44(v0: Adt0, p204: Adt0, p205: Adt8) -> Adt8 {
    let m203 = (p204, p205);
    {
        let mut m203 = m203;
        loop {
            let step = {
                let (v1, v2) = m203;
                let m200 = v1;
                match m200 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt8), Adt8>(v2)
                    }
                    Adt0::C15(r201, r202) => {
                        let v13 = (*r201).clone();
                        let v14 = (*r202).clone();
                        Ok::<(Adt0, Adt8), Adt8>((v14, f51(v2, v0.clone(), v13)))
                    }
                }
            };
            match step {
                Ok(next) => { m203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m206 = v2;
    match m206 {
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
        Adt0::C8(r207) => {
            let v4 = (*r207).clone();
            f45(v0, v1, v4)
        }
        Adt0::C9(r208, r209) => {
            let v5 = (*r208).clone();
            let v6 = (*r209).clone();
            f45(v0.clone(), f45(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r210, r211) => {
            let v8 = (*r210).clone();
            let v9 = (*r211).clone();
            f45(v0.clone(), f45(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f38(v1, Adt0::C12(v10))
        }
        Adt0::C13(r212, r213) => {
            let v11 = (*r212).clone();
            let v12 = (*r213).clone();
            if f50(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f38(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f46(v0: Adt0, v1: Adt0) -> bool {
    let m214 = v0;
    match m214 {
        Adt0::C0 => {
            let a215 = f52(v1);
            let a216 = 0u64;
            nat_eq(a215, a216)
        }
        Adt0::C1 => {
            let a217 = f52(v1);
            let a218 = 1u64;
            nat_eq(a217, a218)
        }
        Adt0::C2 => {
            let a219 = f52(v1);
            let a220 = 2u64;
            nat_eq(a219, a220)
        }
        Adt0::C3 => {
            let a221 = f52(v1);
            let a222 = 3u64;
            nat_eq(a221, a222)
        }
        Adt0::C4(v2) => {
            let a231 = f52(v1.clone());
            let a232 = 4u64;
            if nat_eq(a231, a232) {
                let a233 = f53(v2);
                let a234 = f53(f54(v1));
                nat_eq(a233, a234)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a223 = f52(v1);
            let a224 = 5u64;
            nat_eq(a223, a224)
        }
        Adt0::C6 => {
            let a225 = f52(v1);
            let a226 = 6u64;
            nat_eq(a225, a226)
        }
        Adt0::C7 => {
            let a227 = f52(v1);
            let a228 = 7u64;
            nat_eq(a227, a228)
        }
        Adt0::C8(r251) => {
            let v3 = (*r251).clone();
            let v4: bool = f46(v3, f31(v1.clone()));
            let a235 = f52(v1);
            let a236 = 8u64;
            if nat_eq(a235, a236) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r252, r253) => {
            let v5 = (*r252).clone();
            let v6 = (*r253).clone();
            let v7: bool = f46(v5, f31(v1.clone()));
            let v8: bool = f46(v6, f32(v1.clone()));
            let a237 = f52(v1);
            let a238 = 9u64;
            if nat_eq(a237, a238) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r254) => {
            let v9 = (*r254).clone();
            let v10: bool = f46(v9, f31(v1.clone()));
            let a239 = f52(v1);
            let a240 = 10u64;
            if nat_eq(a239, a240) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r255, r256) => {
            let v11 = (*r255).clone();
            let v12 = (*r256).clone();
            let v13: bool = f46(v11, f31(v1.clone()));
            let v14: bool = f46(v12, f32(v1.clone()));
            let a241 = f52(v1);
            let a242 = 11u64;
            if nat_eq(a241, a242) {
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
            let a243 = f52(v1.clone());
            let a244 = 12u64;
            if nat_eq(a243, a244) {
                let a245 = v15;
                let a246 = f55(v1);
                nat_eq(a245, a246)
            } else {
                false
            }
        }
        Adt0::C13(r257, r258) => {
            let v16 = (*r257).clone();
            let v17 = (*r258).clone();
            let v18: bool = f46(v16, f31(v1.clone()));
            let v19: bool = f46(v17, f32(v1.clone()));
            let a247 = f52(v1);
            let a248 = 13u64;
            if nat_eq(a247, a248) {
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
            let a229 = f52(v1);
            let a230 = 14u64;
            nat_eq(a229, a230)
        }
        Adt0::C15(r259, r260) => {
            let v20 = (*r259).clone();
            let v21 = (*r260).clone();
            let v22: bool = f46(v20, f31(v1.clone()));
            let v23: bool = f46(v21, f32(v1.clone()));
            let a249 = f52(v1);
            let a250 = 15u64;
            if nat_eq(a249, a250) {
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

pub fn f47(v0: Adt8, p266: Adt8, p267: Adt8) -> Adt8 {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m261 = v1;
                match m261 {
                    Adt8::C0 => {
                        Err::<(Adt8, Adt8), Adt8>(v2)
                    }
                    Adt8::C1(r262, r263, r264) => {
                        let v3 = (*r262).clone();
                        let v4 = (*r263).clone();
                        let v5 = (*r264).clone();
                        Ok::<(Adt8, Adt8), Adt8>((v5, f56(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt8) -> R<u64> {
    let m268 = v0;
    match m268 {
        Adt8::C0 => {
            Ok(0u64)
        }
        Adt8::C1(_, _, r271) => {
            let v3 = (*r271).clone();
            let a269 = 1u64;
            let a270 = f48(v3)?;
            nat_add(a269, a270)
        }
    }
}

pub fn f49(v0: Adt10, v1: Adt0, v2: Adt9, v3: Adt8, p276: Adt0, p277: Adt0) -> Adt0 {
    let m275 = (p276, p277);
    {
        let mut m275 = m275;
        loop {
            let step = {
                let (v4, v5) = m275;
                let m272 = v4;
                match m272 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r273, r274) => {
                        let v16 = (*r273).clone();
                        let v17 = (*r274).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f50(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f57(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f17(v0.clone(), v2.clone(), v16.clone())) {
                                f38(v5, v16)
                            } else {
                                v5
                            }
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(p282: Adt0, v1: Adt0) -> bool {
    let m281 = p282;
    {
        let mut m281 = m281;
        loop {
            let step = {
                let v0 = m281;
                let m278 = v0;
                match m278 {
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
                    Adt0::C15(r279, r280) => {
                        let v12 = (*r279).clone();
                        let v13 = (*r280).clone();
                        if f46(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
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

pub fn f51(v0: Adt8, v1: Adt0, v2: Adt0) -> Adt8 {
    if f58(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt8::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v2), std::rc::Rc::new(v0))
    }
}

pub fn f52(v0: Adt0) -> u64 {
    let m283 = v0;
    match m283 {
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

pub fn f53(v0: Adt1) -> u64 {
    let m284 = v0;
    match m284 {
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

pub fn f54(v0: Adt0) -> Adt1 {
    let m285 = v0;
    match m285 {
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

pub fn f55(v0: Adt0) -> u64 {
    let m286 = v0;
    match m286 {
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

pub fn f56(p292: Adt8, v1: Adt0, v2: Adt0, p293: Adt8) -> Adt8 {
    let m291 = (p292, p293);
    {
        let mut m291 = m291;
        loop {
            let step = {
                let (v0, v3) = m291;
                let m287 = v0;
                match m287 {
                    Adt8::C0 => {
                        Err::<(Adt8, Adt8), Adt8>(v3)
                    }
                    Adt8::C1(r288, r289, r290) => {
                        let v4 = (*r288).clone();
                        let v5 = (*r289).clone();
                        let v6 = (*r290).clone();
                        Ok::<(Adt8, Adt8), Adt8>((v6, if f46(v4, v1.clone()) {
                            f51(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p298: Adt0) -> bool {
    let m297 = p298;
    {
        let mut m297 = m297;
        loop {
            let step = {
                let v4 = m297;
                let m294 = v4;
                match m294 {
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
                    Adt0::C15(r295, r296) => {
                        let v15 = (*r295).clone();
                        let v16 = (*r296).clone();
                        if f59(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
                        }
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

pub fn f58(p304: Adt8, v1: Adt0, v2: Adt0) -> bool {
    let m303 = p304;
    {
        let mut m303 = m303;
        loop {
            let step = {
                let v0 = m303;
                let m299 = v0;
                match m299 {
                    Adt8::C0 => {
                        Err::<Adt8, bool>(false)
                    }
                    Adt8::C1(r300, r301, r302) => {
                        let v3 = (*r300).clone();
                        let v4 = (*r301).clone();
                        let v5 = (*r302).clone();
                        if f46(v3, v1.clone()) {
                            if f46(v4, v2.clone()) {
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
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, p309: Adt0) -> bool {
    let m308 = p309;
    {
        let mut m308 = m308;
        loop {
            let step = {
                let v4 = m308;
                let m305 = v4;
                match m305 {
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
                    Adt0::C15(r306, r307) => {
                        let v15 = (*r306).clone();
                        let v16 = (*r307).clone();
                        if if f60(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f61(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
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

pub fn f60(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    f62(v2, v1, f45(v0, Adt0::C14, v3))
}

pub fn f61(v0: Adt0, v1: Adt0) -> bool {
    let m310 = v0;
    match m310 {
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
        Adt0::C9(r311, r312) => {
            let v4 = (*r311).clone();
            let v5 = (*r312).clone();
            if f61(v4, v1.clone()) {
                true
            } else {
                f61(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r313, r314) => {
            let v7 = (*r313).clone();
            let v8 = (*r314).clone();
            if f61(v7, v1.clone()) {
                f61(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f50(v1, Adt0::C12(v9))
        }
        Adt0::C13(r315, r316) => {
            let v10 = (*r315).clone();
            let v11 = (*r316).clone();
            f50(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f62(v0: Adt0, v1: Adt8, p321: Adt0) -> bool {
    let m320 = p321;
    {
        let mut m320 = m320;
        loop {
            let step = {
                let v2 = m320;
                let m317 = v2;
                match m317 {
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
                    Adt0::C15(r318, r319) => {
                        let v13 = (*r318).clone();
                        let v14 = (*r319).clone();
                        if f46(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f58(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
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

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: List<bool>) -> R<Adt7> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7)
}
