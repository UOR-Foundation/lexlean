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
    C0(std::rc::Rc<Adt8>, std::rc::Rc<Adt11>),
    C1(Adt12, std::rc::Rc<Adt13>, std::rc::Rc<Adt11>),
    C2(Adt14, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C3(std::rc::Rc<Adt15>, std::rc::Rc<Adt7>, bool, std::rc::Rc<Adt11>),
    C4(std::rc::Rc<Adt13>, std::rc::Rc<Adt7>, bool, std::rc::Rc<Adt11>),
    C5(std::rc::Rc<Adt17>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C6(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C7(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C8(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>),
    C9(std::rc::Rc<Adt7>),
    C10(Adt23, std::rc::Rc<Adt13>, std::rc::Rc<Adt11>),
    C11(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C12,
    C13(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C14(std::rc::Rc<Adt24>, bool, std::rc::Rc<Adt18>, std::rc::Rc<Adt7>, std::rc::Rc<Adt11>, std::rc::Rc<Adt7>),
    C15(std::rc::Rc<Adt24>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C16(std::rc::Rc<Adt13>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt9, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt10),
    C8,
}

#[derive(Clone)]
pub enum Adt9 {
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
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64),
    C1(std::rc::Rc<Adt16>),
    C2(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(std::rc::Rc<Adt18>),
    C1,
    C2(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19),
    C1(Adt9),
    C2(Adt20, std::rc::Rc<Adt18>),
    C3(Adt21, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4(Adt22, u64),
    C5,
    C6(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
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
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt25),
    C1(std::rc::Rc<Adt13>),
    C2(std::rc::Rc<Adt24>),
    C3(Adt26, std::rc::Rc<Adt24>),
    C4(u64, u64, std::rc::Rc<Adt24>),
    C5,
    C6(std::rc::Rc<Adt24>, std::rc::Rc<Adt24>),
    C7(Adt10),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, std::rc::Rc<Adt0>, List<List<u64>>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt29>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1(std::rc::Rc<Adt0>, u64, std::rc::Rc<Adt0>, std::rc::Rc<Adt29>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt30>),
}

#[derive(Clone)]
pub enum Fn0 {
    F21,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt30) -> R<Option<Adt30>> {
        match self {
            Fn0::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F53(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn1::F53(k0) => f53(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F79(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn2::F79(k0) => f79(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: u64) -> R<Option<Adt7>> {
    let v8: Adt27 = Adt27::C0(v0, std::rc::Rc::new(v1.clone()), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v4), std::rc::Rc::new(v5.clone()));
    let v9: Adt28 = f1(v8.clone(), Adt28::C0(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt29::C0)), v5.clone(), 0u64, List::<bool>::nil(), v6)?;
    let v10: Adt0 = f2(match v9.clone() {
        Adt28::C0(h1, _) => {
            (*h1).clone()
        }
    }, v1.clone());
    let v11: Adt0 = f3(f4(f5(v1.clone())?), v10.clone());
    let v12: Adt30 = f6(v8, v10.clone(), match v9 {
        Adt28::C0(_, h2) => {
            (*h2).clone()
        }
    }, v11.clone(), Adt30::C0);
    let v13: Adt30 = f7(v12, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f5(v11.clone())?;
            let a4 = f5(v11)?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    let m7 = f8(v5, v7);
    match m7 {
        None => {
            Ok(None::<Adt7>)
        }
        Some(v14) => {
            let m8 = f9(v14);
            match m8 {
                None => {
                    Ok(None::<Adt7>)
                }
                Some(v15) => {
                    let v16: Adt7 = f10(v10, v13, v1, Adt0::C14, 0u64, {
                        let (_, h9) = v15;
                        h9
                    }, {
                        let (h10, _) = v15;
                        h10
                    })?;
                    if f11(v16.clone()) {
                        Ok(None::<Adt7>)
                    } else {
                        Ok(Some(v16))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Adt27, p21: Adt28, p22: Adt2, p23: u64, v4: List<bool>, p24: Adt0) -> R<Adt28> {
    let m20 = (p21, (p22, (p23, p24)));
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m20;
                let m11 = v2;
                match m11 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                    Adt2::C14(r18, r19) => {
                        let v34 = (*r18).clone();
                        let v35 = (*r19).clone();
                        let v37: bool = {
                            let m12 = f12(v4.clone(), v3);
                            match m12 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a13 = v36;
                                    bool_not(a13)
                                }
                            }
                        };
                        let v38: Adt28 = Adt28::C0(std::rc::Rc::new(f13(v0.clone(), match v1.clone() {
                            Adt28::C0(h14, _) => {
                                (*h14).clone()
                            }
                        }, v3)), std::rc::Rc::new(match v1 {
                            Adt28::C0(_, h15) => {
                                (*h15).clone()
                            }
                        }));
                        let v39: Adt28 = if v37 {
                            f14(v0.clone(), v38, v34, f15(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>((v39, (v35, ({
                            let a16 = v3;
                            let a17 = 1u64;
                            nat_add(a16, a17)?
                        }, f16(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt28, (Adt2, (u64, Adt0))), R<Adt28>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p29: Adt0, p30: Adt0) -> Adt0 {
    let m28 = (p29, p30);
    {
        let mut m28 = m28;
        loop {
            let step = {
                let (v0, v1) = m28;
                let m25 = v1;
                match m25 {
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
                    Adt0::C15(r26, r27) => {
                        let v12 = (*r26).clone();
                        let v13 = (*r27).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f17(v0, v12), v13))
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

pub fn f3(v0: Adt0, v1: Adt0) -> Adt0 {
    let m31 = v0;
    match m31 {
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
        Adt0::C15(r32, r33) => {
            let v12 = (*r32).clone();
            let v13 = (*r33).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f3(v13, v1)))
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let m34 = v0;
    if m34 == 0 {
        Adt0::C14
    } else {
        let v1 = m34 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f4(v1)))
    }
}

pub fn f5(v0: Adt0) -> R<u64> {
    let m35 = v0;
    match m35 {
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
        Adt0::C15(_, r38) => {
            let v12 = (*r38).clone();
            let v13: u64 = f5(v12)?;
            let a36 = 1u64;
            let a37 = v13;
            nat_add(a36, a37)
        }
    }
}

pub fn f6(v0: Adt27, v1: Adt0, v2: Adt29, p43: Adt0, p44: Adt30) -> Adt30 {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v3, v4) = m42;
                let m39 = v3;
                match m39 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v4)
                    }
                    Adt0::C15(r40, r41) => {
                        let v15 = (*r40).clone();
                        let v16 = (*r41).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v16, f18(v1.clone(), v15.clone(), f19(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt30, v1: u64) -> R<Adt30> {
    let (h45, _) = f20(Fn0::F21, v1, v0)?;
    Ok(h45)
}

pub fn f8(p51: Adt2, p52: u64) -> Option<Adt2> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v0, v1) = m50;
                let m46 = v0;
                match m46 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C13 => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C14(r48, r49) => {
                        let v30 = (*r48).clone();
                        let v31 = (*r49).clone();
                        let m47 = v1;
                        if m47 == 0 {
                            Err::<(Adt2, u64), Option<Adt2>>(Some(v30))
                        } else {
                            let v32 = m47 - 1;
                            Ok::<(Adt2, u64), Option<Adt2>>((v31, v32))
                        }
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt2, u64), Option<Adt2>>(None::<Adt2>)
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Adt2) -> Option<(Adt0, Adt3)> {
    let m53 = v0;
    match m53 {
        Adt2::C0(r54, r55) => {
            let v1 = (*r54).clone();
            let v2 = (*r55).clone();
            Some((v1, v2))
        }
        Adt2::C1(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C2(_, _, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C3(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C4(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C5(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C6(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C7(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C8(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C9(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C10(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C11(_) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C12(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C13 => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C14(_, _) => {
            None::<(Adt0, Adt3)>
        }
        Adt2::C15(_, _, _) => {
            None::<(Adt0, Adt3)>
        }
    }
}

pub fn f10(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0, v4: u64, v5: Adt3, v6: Adt0) -> R<Adt7> {
    let m56 = v5;
    match m56 {
        Adt3::C0 => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C0), std::rc::Rc::new(f22({
                let a57 = Str::lit("value:unit");
                utf8_encode(a57)
            }, v6))))
        }
        Adt3::C1(v7) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C1(v7)), std::rc::Rc::new(f22({
                let a58 = Str::lit("value:bool");
                utf8_encode(a58)
            }, v6))))
        }
        Adt3::C2(v8) => {
            let m59 = f23(v8)?;
            match m59 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v9) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C2(v9)), std::rc::Rc::new(f22({
                        let a60 = Str::lit("value:nat");
                        utf8_encode(a60)
                    }, v6))))
                }
            }
        }
        Adt3::C3(v10) => {
            let m61 = f25(v10)?;
            match m61 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v11) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C3({
                        let (h62, _) = v11;
                        h62
                    }, {
                        let (_, h63) = v11;
                        h63
                    })), std::rc::Rc::new(f22({
                        let a64 = Str::lit("value:int");
                        utf8_encode(a64)
                    }, v6))))
                }
            }
        }
        Adt3::C4(v12) => {
            f26(v12, v6, {
                let a65 = Str::lit("value:u8");
                utf8_encode(a65)
            })
        }
        Adt3::C5(v13) => {
            f26(v13, v6, {
                let a66 = Str::lit("value:u16");
                utf8_encode(a66)
            })
        }
        Adt3::C6(v14) => {
            f26(v14, v6, {
                let a67 = Str::lit("value:u32");
                utf8_encode(a67)
            })
        }
        Adt3::C7(v15) => {
            f26(v15, v6, {
                let a68 = Str::lit("value:u64");
                utf8_encode(a68)
            })
        }
        Adt3::C8(v16) => {
            f26(v16, v6, {
                let a69 = Str::lit("value:i8");
                utf8_encode(a69)
            })
        }
        Adt3::C9(v17) => {
            f26(v17, v6, {
                let a70 = Str::lit("value:i16");
                utf8_encode(a70)
            })
        }
        Adt3::C10(v18) => {
            f26(v18, v6, {
                let a71 = Str::lit("value:i32");
                utf8_encode(a71)
            })
        }
        Adt3::C11(v19) => {
            f26(v19, v6, {
                let a72 = Str::lit("value:i64");
                utf8_encode(a72)
            })
        }
        Adt3::C12(v20) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C5(v20)), std::rc::Rc::new(f22({
                let a73 = Str::lit("value:string");
                utf8_encode(a73)
            }, v6))))
        }
        Adt3::C13(v21) => {
            let m74 = f27(v21)?;
            match m74 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v22) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C6(v22)), std::rc::Rc::new(f22({
                        let a75 = Str::lit("value:bytes");
                        utf8_encode(a75)
                    }, v6))))
                }
            }
        }
        Adt3::C14(v23) => {
            Ok(Adt7::C0(std::rc::Rc::new(Adt8::C7({
                let m76 = v23;
                match m76 {
                    Adt4::C0 => {
                        Adt10::C0
                    }
                    Adt4::C1 => {
                        Adt10::C1
                    }
                    Adt4::C2 => {
                        Adt10::C2
                    }
                }
            })), std::rc::Rc::new(f22({
                let a77 = Str::lit("value:ordering");
                utf8_encode(a77)
            }, v6))))
        }
        Adt3::C15 => {
            let v24: Adt0 = f28(v6.clone());
            Ok(Adt7::C5(std::rc::Rc::new(Adt17::C0(std::rc::Rc::new(f29(v0, v24)?))), std::rc::Rc::new(Adt7::C12), std::rc::Rc::new(f22({
                let a78 = Str::lit("value:none");
                utf8_encode(a78)
            }, v6))))
        }
        Adt3::C16(r99) => {
            let v25 = (*r99).clone();
            let v26: Adt0 = f28(v6.clone());
            let v27: Adt7 = f10(v0, v1, v2, v3, 0u64, v25, v26)?;
            if f11(v27.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C1), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v27), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f22({
                    let a79 = Str::lit("value:some");
                    utf8_encode(a79)
                }, v6))))
            }
        }
        Adt3::C17(r100) => {
            let v28 = (*r100).clone();
            let v29: Adt7 = f10(v0.clone(), v1, v2, v3, 0u64, v28, f28(v6.clone()))?;
            if f11(v29.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C2(std::rc::Rc::new(f29(v0.clone(), f28(v6.clone()))?), std::rc::Rc::new(f29(v0, f30(v6.clone()))?))), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v29), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f22({
                    let a80 = Str::lit("value:ok");
                    utf8_encode(a80)
                }, v6))))
            }
        }
        Adt3::C18(r101) => {
            let v30 = (*r101).clone();
            let v31: Adt7 = f10(v0.clone(), v1, v2, v3, 0u64, v30, f30(v6.clone()))?;
            if f11(v31.clone()) {
                Ok(f24(0u64))
            } else {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C3(std::rc::Rc::new(f29(v0.clone(), f28(v6.clone()))?), std::rc::Rc::new(f29(v0, f30(v6.clone()))?))), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v31), std::rc::Rc::new(Adt7::C12))), std::rc::Rc::new(f22({
                    let a81 = Str::lit("value:error");
                    utf8_encode(a81)
                }, v6))))
            }
        }
        Adt3::C19(r102) => {
            let v32 = (*r102).clone();
            f10(v0, v1, v2, v3, 1u64, v32, f28(v6))
        }
        Adt3::C20(r103, r104) => {
            let v33 = (*r103).clone();
            let v34 = (*r104).clone();
            let v35: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v33, f28(v6.clone()))?;
            if f11(v35.clone()) {
                Ok(f24(0u64))
            } else {
                let v36: Adt7 = f10(v0, v1, v2, v3, 0u64, v34, f30(v6.clone()))?;
                if f11(v36.clone()) {
                    Ok(f24(0u64))
                } else {
                    Ok(Adt7::C6(std::rc::Rc::new(v35), std::rc::Rc::new(v36), std::rc::Rc::new(f22({
                        let a82 = Str::lit("value:pair");
                        utf8_encode(a82)
                    }, v6))))
                }
            }
        }
        Adt3::C21(v37, r105) => {
            let v38 = (*r105).clone();
            let v39: u64 = f31(v6.clone());
            let m83 = f32(v2.clone(), v39, v37);
            match m83 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v40) => {
                    let v41: Adt7 = f10(v0, v1, v2, Adt0::C12(v39), 2u64, v38, v40)?;
                    if f11(v41.clone()) {
                        Ok(f24(0u64))
                    } else {
                        Ok(Adt7::C5(std::rc::Rc::new(Adt17::C4(v39, v37)), std::rc::Rc::new(v41), std::rc::Rc::new(f22({
                            let a84 = Str::lit("value:adt");
                            utf8_encode(a84)
                        }, v6))))
                    }
                }
            }
        }
        Adt3::C22(_, _) => {
            Ok(f24(0u64))
        }
        Adt3::C23 => {
            let a85 = v4;
            let a86 = 1u64;
            if nat_eq(a85, a86) {
                Ok(Adt7::C5(std::rc::Rc::new(Adt17::C7(std::rc::Rc::new(f29(v0, v6)?))), std::rc::Rc::new(Adt7::C12), std::rc::Rc::new(Adt11::C0({
                    let a87 = Str::lit("value:list");
                    utf8_encode(a87)
                }, None::<Adt9>))))
            } else {
                let a88 = v4;
                let a89 = 2u64;
                if nat_eq(a88, a89) {
                    Ok(Adt7::C12)
                } else {
                    Ok(f24(0u64))
                }
            }
        }
        Adt3::C24(r106, r107) => {
            let v44 = (*r106).clone();
            let v45 = (*r107).clone();
            let a90 = v4;
            let a91 = 1u64;
            if nat_eq(a90, a91) {
                let v46: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v6.clone())?;
                if f11(v46.clone()) {
                    Ok(f24(0u64))
                } else {
                    let v47: Adt7 = f10(v0, v1, v2, v3, 1u64, v45, v6)?;
                    if f11(v47.clone()) {
                        Ok(f24(0u64))
                    } else {
                        Ok(Adt7::C5(std::rc::Rc::new(Adt17::C6), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v46), std::rc::Rc::new(Adt7::C13(std::rc::Rc::new(v47), std::rc::Rc::new(Adt7::C12))))), std::rc::Rc::new(Adt11::C0({
                            let a92 = Str::lit("value:list");
                            utf8_encode(a92)
                        }, None::<Adt9>))))
                    }
                }
            } else {
                let a93 = v4;
                let a94 = 2u64;
                if nat_eq(a93, a94) {
                    let m95 = v6;
                    match m95 {
                        Adt0::C0 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C1 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C2 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C3 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C5 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C6 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C7 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C14 => {
                            Ok(Adt7::C12)
                        }
                        Adt0::C15(r97, r98) => {
                            let v58 = (*r97).clone();
                            let v59 = (*r98).clone();
                            let v60: Adt7 = f10(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v58.clone())?;
                            if f11(v60.clone()) {
                                Ok(f24(0u64))
                            } else {
                                let v61: Adt7 = f10(v0.clone(), v1.clone(), v2, v3.clone(), 2u64, v45, v59)?;
                                if f11(v61.clone()) {
                                    Ok(f24(0u64))
                                } else {
                                    Ok(Adt7::C13(std::rc::Rc::new(if f33(v0, v1, v3, v58) {
                                        Adt7::C2(Adt14::C1, std::rc::Rc::new(v60), std::rc::Rc::new(Adt11::C0({
                                            let a96 = Str::lit("indirection");
                                            utf8_encode(a96)
                                        }, None::<Adt9>)))
                                    } else {
                                        v60
                                    }), std::rc::Rc::new(v61)))
                                }
                            }
                        }
                    }
                } else {
                    Ok(f24(0u64))
                }
            }
        }
    }
}

pub fn f11(v0: Adt7) -> bool {
    let m108 = v0;
    match m108 {
        Adt7::C0(r110, _) => {
            let v1 = (*r110).clone();
            let m109 = v1;
            match m109 {
                Adt8::C0 => {
                    false
                }
                Adt8::C1(_) => {
                    false
                }
                Adt8::C2(_) => {
                    false
                }
                Adt8::C3(_, _) => {
                    false
                }
                Adt8::C4(_, _, _) => {
                    false
                }
                Adt8::C5(_) => {
                    false
                }
                Adt8::C6(_) => {
                    false
                }
                Adt8::C7(_) => {
                    false
                }
                Adt8::C8 => {
                    true
                }
            }
        }
        Adt7::C1(_, _, _) => {
            false
        }
        Adt7::C2(_, _, _) => {
            false
        }
        Adt7::C3(_, _, _, _) => {
            false
        }
        Adt7::C4(_, _, _, _) => {
            false
        }
        Adt7::C5(_, _, _) => {
            false
        }
        Adt7::C6(_, _, _) => {
            false
        }
        Adt7::C7(_, _, _, _) => {
            false
        }
        Adt7::C8(_, _, _) => {
            false
        }
        Adt7::C9(_) => {
            false
        }
        Adt7::C10(_, _, _) => {
            false
        }
        Adt7::C11(_, _) => {
            false
        }
        Adt7::C12 => {
            false
        }
        Adt7::C13(_, _) => {
            false
        }
        Adt7::C14(_, _, _, _, _, _) => {
            false
        }
        Adt7::C15(_, _, _) => {
            false
        }
        Adt7::C16(_, _) => {
            false
        }
    }
}

pub fn f12(p114: List<bool>, p115: u64) -> Option<bool> {
    let m113 = (p114, p115);
    {
        let mut m113 = m113;
        loop {
            let step = {
                let (v0, v1) = m113;
                let m111 = v0;
                match m111.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m112 = v1;
                        if m112 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m112 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m113 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Adt27, v1: Adt0, v2: u64) -> Adt0 {
    f34(f35(v1, f36(v0.clone(), v2)), f37(v0, v2))
}

pub fn f14(v0: Adt27, v1: Adt28, v2: Adt2, v3: Adt0) -> R<Adt28> {
    let m116 = v2;
    match m116 {
        Adt2::C0(r127, _) => {
            let v4 = (*r127).clone();
            Ok(Adt28::C0(std::rc::Rc::new(f34(match v1.clone() {
                Adt28::C0(h117, _) => {
                    (*h117).clone()
                }
            }, v4)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h118) => {
                    (*h118).clone()
                }
            })))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, r128, r129, r130) => {
            let v8 = (*r128).clone();
            let v9 = (*r129).clone();
            let v10 = (*r130).clone();
            let v11: Adt28 = Adt28::C0(std::rc::Rc::new(f34(match v1.clone() {
                Adt28::C0(h119, _) => {
                    (*h119).clone()
                }
            }, v8)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h120) => {
                    (*h120).clone()
                }
            }));
            let v12: Adt28 = f14(v0.clone(), v11, v9, f15(v3.clone()))?;
            f14(v0, v12, v10, f16(v3))
        }
        Adt2::C3(r131, r132, r133) => {
            let v13 = (*r131).clone();
            let v14 = (*r132).clone();
            let v15 = (*r133).clone();
            let v16: Adt28 = f14(v0.clone(), v1, v13, f15(v3.clone()))?;
            let v17: Adt28 = f14(v0.clone(), v16, v14, f16(v3.clone()))?;
            f14(v0, v17, v15, f38(v3))
        }
        Adt2::C4(r134, r135, r136) => {
            let v18 = (*r134).clone();
            let v19 = (*r135).clone();
            let v20 = (*r136).clone();
            let v21: Adt28 = Adt28::C0(std::rc::Rc::new(f34(match v1.clone() {
                Adt28::C0(h121, _) => {
                    (*h121).clone()
                }
            }, v18)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h122) => {
                    (*h122).clone()
                }
            }));
            let v22: Adt28 = f14(v0.clone(), v21, v19, f15(v3.clone()))?;
            f14(v0, v22, v20, f16(v3))
        }
        Adt2::C5(_, r137, r138) => {
            let v24 = (*r137).clone();
            let v25 = (*r138).clone();
            let v26: Adt28 = Adt28::C0(std::rc::Rc::new(f34(match v1.clone() {
                Adt28::C0(h123, _) => {
                    (*h123).clone()
                }
            }, v24)), std::rc::Rc::new(match v1 {
                Adt28::C0(_, h124) => {
                    (*h124).clone()
                }
            }));
            f14(v0, v26, v25, f15(v3))
        }
        Adt2::C6(_, r139) => {
            let v28 = (*r139).clone();
            f14(v0, v1, v28, f15(v3))
        }
        Adt2::C7(v29, r140) => {
            let v30 = (*r140).clone();
            let v31: Adt0 = f36(v0.clone(), v29);
            let v32: u64 = f39(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f40(v31.clone(), v32)), std::rc::Rc::new(f37(v0.clone(), v29)));
            let v34: Adt28 = Adt28::C0(std::rc::Rc::new(f34(match v1.clone() {
                Adt28::C0(h125, _) => {
                    (*h125).clone()
                }
            }, v33.clone())), std::rc::Rc::new(f41(match v1 {
                Adt28::C0(_, h126) => {
                    (*h126).clone()
                }
            }, v33, v29, f42(v31, v32))));
            f14(v0, v34, v30, f15(v3))
        }
        Adt2::C8(r141, r142) => {
            let v35 = (*r141).clone();
            let v36 = (*r142).clone();
            let v37: Adt28 = f14(v0.clone(), v1, v35, f15(v3.clone()))?;
            f14(v0, v37, v36, f16(v3))
        }
        Adt2::C9(_, r143) => {
            let v39 = (*r143).clone();
            f14(v0, v1, v39, f15(v3))
        }
        Adt2::C10(r144) => {
            let v40 = (*r144).clone();
            f14(v0, v1, v40, f15(v3))
        }
        Adt2::C11(r145) => {
            let v41 = (*r145).clone();
            f14(v0, v1, v41, f15(v3))
        }
        Adt2::C12(r146, _) => {
            let v42 = (*r146).clone();
            f14(v0, v1, v42, f15(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r147, r148) => {
            let v44 = (*r147).clone();
            let v45 = (*r148).clone();
            let v46: Adt28 = f14(v0.clone(), v1, v44, f15(v3.clone()))?;
            f14(v0, v46, v45, f16(v3))
        }
        Adt2::C15(_, _, r149) => {
            let v49 = (*r149).clone();
            f14(v0, v1, v49, f15(v3))
        }
    }
}

pub fn f15(v0: Adt0) -> Adt0 {
    f28(f30(v0))
}

pub fn f16(v0: Adt0) -> Adt0 {
    f28(f30(f30(v0)))
}

pub fn f17(p154: Adt0, p155: Adt0) -> Adt0 {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v0, v1) = m153;
                let m150 = v1;
                match m150 {
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
                    Adt0::C15(r151, r152) => {
                        let v12 = (*r151).clone();
                        let v13 = (*r152).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f35(v0, v12), v13))
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

pub fn f18(v0: Adt0, v1: Adt0, p160: Adt0, p161: Adt30) -> Adt30 {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v2, v3) = m159;
                let m156 = v2;
                match m156 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r157, r158) => {
                        let v14 = (*r157).clone();
                        let v15 = (*r158).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f43(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt27, v1: Adt29, v2: Adt0) -> Adt0 {
    let m162 = v2;
    match m162 {
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
            let m163 = f44(v0, v10);
            match m163 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r164, r165) => {
            let v12 = (*r164).clone();
            let v13 = (*r165).clone();
            f45(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f20(v0: Fn0, p170: u64, p171: Adt30) -> R<(Adt30, bool)> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v1, v2) = m169;
                let m166 = v1;
                if m166 == 0 {
                    Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m166 - 1;
                    let m168 = {
                        let c167: Fn0 = v0.clone();
                        c167.apply(v2.clone())?
                    };
                    match m168 {
                        None => {
                            Err::<(u64, Adt30), R<(Adt30, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt30), R<(Adt30, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt30) -> R<Option<Adt30>> {
    f46(v0)
}

pub fn f22(v0: Bytes, v1: Adt0) -> Adt11 {
    Adt11::C0(v0, f47(v1))
}

pub fn f23(v0: Bytes) -> R<Option<u64>> {
    let m172 = f48(v0, false)?;
    match m172 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            Ok(Some({
                let (_, h173) = v1;
                h173
            }))
        }
    }
}

pub fn f24(_: u64) -> Adt7 {
    Adt7::C0(std::rc::Rc::new(Adt8::C8), std::rc::Rc::new(Adt11::C0({
        let a174 = Str::lit("");
        utf8_encode(a174)
    }, None::<Adt9>)))
}

pub fn f25(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m175 = f48(v0, true)?;
    match m175 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            let (h176, _) = v1;
            if h176 {
                let a178 = {
                    let (_, h177) = v1;
                    h177
                };
                let a179 = 9223372036854775809u64;
                if nat_lt(a178, a179) {
                    Ok(Some(({
                        let a181 = 0u64;
                        let a182 = {
                            let (_, h180) = v1;
                            h180
                        };
                        nat_lt(a181, a182)
                    }, {
                        let (_, h183) = v1;
                        h183
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            } else {
                let a185 = {
                    let (_, h184) = v1;
                    h184
                };
                let a186 = 9223372036854775808u64;
                if nat_lt(a185, a186) {
                    Ok(Some((false, {
                        let (_, h187) = v1;
                        h187
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt0, v2: Bytes) -> R<Adt7> {
    let m188 = f49(v1.clone());
    match m188 {
        None => {
            Ok(f24(0u64))
        }
        Some(v3) => {
            let m189 = f50(v0)?;
            match m189 {
                None => {
                    Ok(f24(0u64))
                }
                Some(v4) => {
                    Ok(Adt7::C0(std::rc::Rc::new(Adt8::C4(f51(v3), {
                        let (h190, _) = v4;
                        h190
                    }, {
                        let (_, h191) = v4;
                        h191
                    })), std::rc::Rc::new(f22(v2, v1))))
                }
            }
        }
    }
}

pub fn f27(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a192 = v0.clone();
        length_bytes(a192)
    };
    let v2: ((u64, Bytes), bool) = f52(Fn1::F53(v0), v1, (0u64, {
        let a193 = Str::lit("");
        utf8_encode(a193)
    }))?;
    let a196 = {
        let (h194, _) = {
            let (h195, _) = v2.clone();
            h195
        };
        h194
    };
    let a197 = v1;
    if nat_eq(a196, a197) {
        Ok(Some({
            let (_, h198) = {
                let (h199, _) = v2;
                h199
            };
            h198
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f28(v0: Adt0) -> Adt0 {
    let m200 = f54(v0);
    match m200 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f29(v0: Adt0, v1: Adt0) -> R<Adt18> {
    let m201 = v1;
    match m201 {
        Adt0::C0 => {
            Ok(Adt18::C0(Adt19::C0))
        }
        Adt0::C1 => {
            Ok(Adt18::C0(Adt19::C1))
        }
        Adt0::C2 => {
            Ok(Adt18::C0(Adt19::C2))
        }
        Adt0::C3 => {
            Ok(Adt18::C0(Adt19::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt18::C1(f51(v2)))
        }
        Adt0::C5 => {
            Ok(Adt18::C0(Adt19::C5))
        }
        Adt0::C6 => {
            Ok(Adt18::C0(Adt19::C6))
        }
        Adt0::C7 => {
            Ok(Adt18::C0(Adt19::C4))
        }
        Adt0::C8(r202) => {
            let v3 = (*r202).clone();
            Ok(Adt18::C2(Adt20::C0, std::rc::Rc::new(f29(v0, v3)?)))
        }
        Adt0::C9(r203, r204) => {
            let v4 = (*r203).clone();
            let v5 = (*r204).clone();
            Ok(Adt18::C3(Adt21::C0, std::rc::Rc::new(f29(v0.clone(), v4)?), std::rc::Rc::new(f29(v0, v5)?)))
        }
        Adt0::C10(r205) => {
            let v6 = (*r205).clone();
            Ok(Adt18::C2(Adt20::C1, std::rc::Rc::new(f29(v0, v6)?)))
        }
        Adt0::C11(r206, r207) => {
            let v7 = (*r206).clone();
            let v8 = (*r207).clone();
            Ok(Adt18::C3(Adt21::C1, std::rc::Rc::new(f29(v0.clone(), v7)?), std::rc::Rc::new(f29(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt18::C4(Adt22::C0, v9))
        }
        Adt0::C13(r208, r209) => {
            let v10 = (*r208).clone();
            let v11 = (*r209).clone();
            Ok(Adt18::C4(Adt22::C1, f55(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt18::C5)
        }
        Adt0::C15(r210, r211) => {
            let v12 = (*r210).clone();
            let v13 = (*r211).clone();
            Ok(Adt18::C6(std::rc::Rc::new(f29(v0.clone(), v12)?), std::rc::Rc::new(f29(v0, v13)?)))
        }
    }
}

pub fn f30(v0: Adt0) -> Adt0 {
    let m212 = f56(v0);
    match m212 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f31(v0: Adt0) -> u64 {
    let m213 = v0;
    match m213 {
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

pub fn f32(v0: Adt0, v1: u64, v2: u64) -> Option<Adt0> {
    let m214 = f57(v0, v1);
    match m214 {
        None => {
            None::<Adt0>
        }
        Some(v3) => {
            f57(v3, v2)
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt30, v2: Adt0, v3: Adt0) -> bool {
    f58(v2, v1, f59(v0, Adt0::C14, v3))
}

pub fn f34(v0: Adt0, v1: Adt0) -> Adt0 {
    let m215 = v1;
    match m215 {
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
        Adt0::C8(r216) => {
            let v3 = (*r216).clone();
            f34(v0, v3)
        }
        Adt0::C9(r217, r218) => {
            let v4 = (*r217).clone();
            let v5 = (*r218).clone();
            f34(f34(v0, v4), v5)
        }
        Adt0::C10(r219) => {
            let v6 = (*r219).clone();
            f34(v0, v6)
        }
        Adt0::C11(r220, r221) => {
            let v7 = (*r220).clone();
            let v8 = (*r221).clone();
            f34(f34(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r222, r223) => {
            let v10 = (*r222).clone();
            let v11 = (*r223).clone();
            let v12: Adt0 = f60(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f34(f34(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r224, r225) => {
            let v13 = (*r224).clone();
            let v14 = (*r225).clone();
            f34(f34(v0, v13), v14)
        }
    }
}

pub fn f35(p230: Adt0, p231: Adt0) -> Adt0 {
    let m229 = (p230, p231);
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v0, v1) = m229;
                let m226 = v1;
                match m226 {
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
                    Adt0::C15(r227, r228) => {
                        let v2 = (*r227).clone();
                        let v3 = (*r228).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f34(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt27, v1: u64) -> Adt0 {
    let m232 = f61(v0, v1);
    match m232 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f37(v0: Adt27, v1: u64) -> Adt0 {
    let m234 = f62(match v0 {
        Adt27::C0(_, _, _, _, h233, _) => {
            (*h233).clone()
        }
    }, v1);
    match m234 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: Adt0) -> Adt0 {
    f28(f30(f30(f30(v0))))
}

pub fn f39(v0: Adt2) -> R<u64> {
    let m235 = v0;
    match m235 {
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
        Adt2::C14(_, r238) => {
            let v33 = (*r238).clone();
            let v34: u64 = f39(v33)?;
            let a236 = 1u64;
            let a237 = v34;
            nat_add(a236, a237)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f40(p244: Adt0, p245: u64) -> Adt0 {
    let m243 = (p244, p245);
    {
        let mut m243 = m243;
        loop {
            let step = {
                let (v0, v1) = m243;
                let m239 = v0;
                match m239 {
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
                    Adt0::C15(r241, r242) => {
                        let v12 = (*r241).clone();
                        let v13 = (*r242).clone();
                        let m240 = v1;
                        if m240 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m240 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt29, v1: Adt0, v2: u64, v3: Adt0) -> Adt29 {
    if f63(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt29::C1(std::rc::Rc::new(v1), v2, std::rc::Rc::new(v3), std::rc::Rc::new(v0))
    }
}

pub fn f42(v0: Adt0, v1: u64) -> Adt0 {
    let m246 = v0;
    match m246 {
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
        Adt0::C15(r248, r249) => {
            let v12 = (*r248).clone();
            let v13 = (*r249).clone();
            let m247 = v1;
            if m247 == 0 {
                Adt0::C14
            } else {
                let v14 = m247 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f42(v13, v14)))
            }
        }
    }
}

pub fn f43(v0: Adt0, v1: Adt0, p254: Adt0, p255: Adt30) -> Adt30 {
    let m253 = (p254, p255);
    {
        let mut m253 = m253;
        loop {
            let step = {
                let (v2, v3) = m253;
                let m250 = v2;
                match m250 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v3)
                    }
                    Adt0::C15(r251, r252) => {
                        let v14 = (*r251).clone();
                        let v15 = (*r252).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v15, f64(v1.clone(), f59(v0.clone(), Adt0::C14, v14), v3)))
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

pub fn f44(v0: Adt27, v1: u64) -> Option<Adt0> {
    f62(match v0 {
        Adt27::C0(_, h256, _, _, _, _) => {
            (*h256).clone()
        }
    }, v1)
}

pub fn f45(v0: Adt29, v1: Adt0) -> Adt0 {
    let m257 = v0;
    match m257 {
        Adt29::C0 => {
            Adt0::C14
        }
        Adt29::C1(r258, _, r259, r260) => {
            let v2 = (*r258).clone();
            let v4 = (*r259).clone();
            let v5 = (*r260).clone();
            if f65(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f45(v5, v1)))
            } else {
                f45(v5, v1)
            }
        }
    }
}

pub fn f46(v0: Adt30) -> R<Option<Adt30>> {
    let v1: Adt30 = f66(v0.clone(), v0.clone(), v0.clone());
    let a261 = f67(v1.clone())?;
    let a262 = f67(v0)?;
    if nat_eq(a261, a262) {
        Ok(None::<Adt30>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f47(v0: Adt0) -> Option<Adt9> {
    let m263 = v0;
    match m263 {
        Adt0::C0 => {
            None::<Adt9>
        }
        Adt0::C1 => {
            None::<Adt9>
        }
        Adt0::C2 => {
            None::<Adt9>
        }
        Adt0::C3 => {
            None::<Adt9>
        }
        Adt0::C4(v1) => {
            Some(f51(v1))
        }
        Adt0::C5 => {
            None::<Adt9>
        }
        Adt0::C6 => {
            None::<Adt9>
        }
        Adt0::C7 => {
            None::<Adt9>
        }
        Adt0::C8(_) => {
            None::<Adt9>
        }
        Adt0::C9(_, _) => {
            None::<Adt9>
        }
        Adt0::C10(_) => {
            None::<Adt9>
        }
        Adt0::C11(_, _) => {
            None::<Adt9>
        }
        Adt0::C12(_) => {
            None::<Adt9>
        }
        Adt0::C13(_, _) => {
            None::<Adt9>
        }
        Adt0::C14 => {
            None::<Adt9>
        }
        Adt0::C15(_, _) => {
            None::<Adt9>
        }
    }
}

pub fn f48(v0: Bytes, v1: bool) -> R<Option<(bool, u64)>> {
    let m264 = f68(v0.clone(), 0u64);
    match m264 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v2) => {
            if if v1 {
                let a265 = v2;
                let a266 = 45u8;
                equal(a265, a266)
            } else {
                false
            } {
                let m267 = f69(v0, 1u64)?;
                match m267 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v3) => {
                        Ok(Some((true, v3)))
                    }
                }
            } else {
                let m268 = f69(v0, 0u64)?;
                match m268 {
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

pub fn f49(v0: Adt0) -> Option<Adt1> {
    let m269 = v0;
    match m269 {
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

pub fn f50(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m270 = f48(v0, true)?;
    match m270 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            Ok(Some(({
                let (h271, _) = v1;
                if h271 {
                    let a273 = 0u64;
                    let a274 = {
                        let (_, h272) = v1;
                        h272
                    };
                    nat_lt(a273, a274)
                } else {
                    false
                }
            }, {
                let (_, h275) = v1;
                h275
            })))
        }
    }
}

pub fn f51(v0: Adt1) -> Adt9 {
    let m276 = v0;
    match m276 {
        Adt1::C0 => {
            Adt9::C0
        }
        Adt1::C1 => {
            Adt9::C1
        }
        Adt1::C2 => {
            Adt9::C2
        }
        Adt1::C3 => {
            Adt9::C3
        }
        Adt1::C4 => {
            Adt9::C4
        }
        Adt1::C5 => {
            Adt9::C5
        }
        Adt1::C6 => {
            Adt9::C6
        }
        Adt1::C7 => {
            Adt9::C7
        }
    }
}

pub fn f52(v0: Fn1, p281: u64, p282: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m280 = (p281, p282);
    {
        let mut m280 = m280;
        loop {
            let step = {
                let (v1, v2) = m280;
                let m277 = v1;
                if m277 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m277 - 1;
                    let m279 = {
                        let c278: Fn1 = v0.clone();
                        c278.apply(v2.clone())?
                    };
                    match m279 {
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
                Ok(next) => { m280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f70(v0, v1)
}

pub fn f54(v0: Adt0) -> Option<Adt0> {
    let m283 = v0;
    match m283 {
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
        Adt0::C8(r284) => {
            let v2 = (*r284).clone();
            Some(v2)
        }
        Adt0::C9(r285, _) => {
            let v3 = (*r285).clone();
            Some(v3)
        }
        Adt0::C10(r286) => {
            let v5 = (*r286).clone();
            Some(v5)
        }
        Adt0::C11(r287, _) => {
            let v6 = (*r287).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r288, _) => {
            let v9 = (*r288).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r289, _) => {
            let v11 = (*r289).clone();
            Some(v11)
        }
    }
}

pub fn f55(p296: Adt0, v1: Adt0, p297: u64) -> R<u64> {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v0, v2) = m295;
                let m290 = v0;
                match m290 {
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
                    Adt0::C15(r293, r294) => {
                        let v13 = (*r293).clone();
                        let v14 = (*r294).clone();
                        if f65(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a291 = v2;
                                let a292 = 1u64;
                                nat_add(a291, a292)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt0) -> Option<Adt0> {
    let m298 = v0;
    match m298 {
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
        Adt0::C9(_, r299) => {
            let v4 = (*r299).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r300) => {
            let v7 = (*r300).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r301) => {
            let v10 = (*r301).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r302) => {
            let v12 = (*r302).clone();
            Some(v12)
        }
    }
}

pub fn f57(p308: Adt0, p309: u64) -> Option<Adt0> {
    let m307 = (p308, p309);
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v0, v1) = m307;
                let m303 = v0;
                match m303 {
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
                    Adt0::C15(r305, r306) => {
                        let v12 = (*r305).clone();
                        let v13 = (*r306).clone();
                        let m304 = v1;
                        if m304 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m304 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt0, v1: Adt30, p314: Adt0) -> bool {
    let m313 = p314;
    {
        let mut m313 = m313;
        loop {
            let step = {
                let v2 = m313;
                let m310 = v2;
                match m310 {
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
                    Adt0::C15(r311, r312) => {
                        let v13 = (*r311).clone();
                        let v14 = (*r312).clone();
                        if f65(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f71(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m315 = v2;
    match m315 {
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
        Adt0::C8(r316) => {
            let v4 = (*r316).clone();
            f59(v0, v1, v4)
        }
        Adt0::C9(r317, r318) => {
            let v5 = (*r317).clone();
            let v6 = (*r318).clone();
            f59(v0.clone(), f59(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r319, r320) => {
            let v8 = (*r319).clone();
            let v9 = (*r320).clone();
            f59(v0.clone(), f59(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f60(v1, Adt0::C12(v10))
        }
        Adt0::C13(r321, r322) => {
            let v11 = (*r321).clone();
            let v12 = (*r322).clone();
            if f72(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f60(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f60(v0: Adt0, v1: Adt0) -> Adt0 {
    if f72(v0.clone(), v1.clone()) {
        v0
    } else {
        f3(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f61(v0: Adt27, v1: u64) -> Option<Adt0> {
    f62(match v0 {
        Adt27::C0(_, _, _, h323, _, _) => {
            (*h323).clone()
        }
    }, v1)
}

pub fn f62(p329: Adt0, p330: u64) -> Option<Adt0> {
    let m328 = (p329, p330);
    {
        let mut m328 = m328;
        loop {
            let step = {
                let (v0, v1) = m328;
                let m324 = v0;
                match m324 {
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
                    Adt0::C15(r326, r327) => {
                        let v12 = (*r326).clone();
                        let v13 = (*r327).clone();
                        let m325 = v1;
                        if m325 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m325 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(p337: Adt29, v1: Adt0, v2: u64) -> bool {
    let m336 = p337;
    {
        let mut m336 = m336;
        loop {
            let step = {
                let v0 = m336;
                let m331 = v0;
                match m331 {
                    Adt29::C0 => {
                        Err::<Adt29, bool>(false)
                    }
                    Adt29::C1(r334, v4, _, r335) => {
                        let v3 = (*r334).clone();
                        let v6 = (*r335).clone();
                        let a332 = v4;
                        let a333 = v2;
                        if nat_eq(a332, a333) {
                            if f65(v3, v1.clone()) {
                                Err::<Adt29, bool>(true)
                            } else {
                                Ok::<Adt29, bool>(v6)
                            }
                        } else {
                            Ok::<Adt29, bool>(v6)
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

pub fn f64(v0: Adt0, p342: Adt0, p343: Adt30) -> Adt30 {
    let m341 = (p342, p343);
    {
        let mut m341 = m341;
        loop {
            let step = {
                let (v1, v2) = m341;
                let m338 = v1;
                match m338 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt30), Adt30>(v2)
                    }
                    Adt0::C15(r339, r340) => {
                        let v13 = (*r339).clone();
                        let v14 = (*r340).clone();
                        Ok::<(Adt0, Adt30), Adt30>((v14, f73(v2, v0.clone(), v13)))
                    }
                }
            };
            match step {
                Ok(next) => { m341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt0, v1: Adt0) -> bool {
    let m344 = v0;
    match m344 {
        Adt0::C0 => {
            let a345 = f74(v1);
            let a346 = 0u64;
            nat_eq(a345, a346)
        }
        Adt0::C1 => {
            let a347 = f74(v1);
            let a348 = 1u64;
            nat_eq(a347, a348)
        }
        Adt0::C2 => {
            let a349 = f74(v1);
            let a350 = 2u64;
            nat_eq(a349, a350)
        }
        Adt0::C3 => {
            let a351 = f74(v1);
            let a352 = 3u64;
            nat_eq(a351, a352)
        }
        Adt0::C4(v2) => {
            let a361 = f74(v1.clone());
            let a362 = 4u64;
            if nat_eq(a361, a362) {
                let a363 = f75(v2);
                let a364 = f75(f76(v1));
                nat_eq(a363, a364)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a353 = f74(v1);
            let a354 = 5u64;
            nat_eq(a353, a354)
        }
        Adt0::C6 => {
            let a355 = f74(v1);
            let a356 = 6u64;
            nat_eq(a355, a356)
        }
        Adt0::C7 => {
            let a357 = f74(v1);
            let a358 = 7u64;
            nat_eq(a357, a358)
        }
        Adt0::C8(r381) => {
            let v3 = (*r381).clone();
            let v4: bool = f65(v3, f28(v1.clone()));
            let a365 = f74(v1);
            let a366 = 8u64;
            if nat_eq(a365, a366) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r382, r383) => {
            let v5 = (*r382).clone();
            let v6 = (*r383).clone();
            let v7: bool = f65(v5, f28(v1.clone()));
            let v8: bool = f65(v6, f30(v1.clone()));
            let a367 = f74(v1);
            let a368 = 9u64;
            if nat_eq(a367, a368) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r384) => {
            let v9 = (*r384).clone();
            let v10: bool = f65(v9, f28(v1.clone()));
            let a369 = f74(v1);
            let a370 = 10u64;
            if nat_eq(a369, a370) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r385, r386) => {
            let v11 = (*r385).clone();
            let v12 = (*r386).clone();
            let v13: bool = f65(v11, f28(v1.clone()));
            let v14: bool = f65(v12, f30(v1.clone()));
            let a371 = f74(v1);
            let a372 = 11u64;
            if nat_eq(a371, a372) {
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
            let a373 = f74(v1.clone());
            let a374 = 12u64;
            if nat_eq(a373, a374) {
                let a375 = v15;
                let a376 = f31(v1);
                nat_eq(a375, a376)
            } else {
                false
            }
        }
        Adt0::C13(r387, r388) => {
            let v16 = (*r387).clone();
            let v17 = (*r388).clone();
            let v18: bool = f65(v16, f28(v1.clone()));
            let v19: bool = f65(v17, f30(v1.clone()));
            let a377 = f74(v1);
            let a378 = 13u64;
            if nat_eq(a377, a378) {
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
            let a359 = f74(v1);
            let a360 = 14u64;
            nat_eq(a359, a360)
        }
        Adt0::C15(r389, r390) => {
            let v20 = (*r389).clone();
            let v21 = (*r390).clone();
            let v22: bool = f65(v20, f28(v1.clone()));
            let v23: bool = f65(v21, f30(v1.clone()));
            let a379 = f74(v1);
            let a380 = 15u64;
            if nat_eq(a379, a380) {
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

pub fn f66(v0: Adt30, p396: Adt30, p397: Adt30) -> Adt30 {
    let m395 = (p396, p397);
    {
        let mut m395 = m395;
        loop {
            let step = {
                let (v1, v2) = m395;
                let m391 = v1;
                match m391 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v2)
                    }
                    Adt30::C1(r392, r393, r394) => {
                        let v3 = (*r392).clone();
                        let v4 = (*r393).clone();
                        let v5 = (*r394).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v5, f77(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt30) -> R<u64> {
    let m398 = v0;
    match m398 {
        Adt30::C0 => {
            Ok(0u64)
        }
        Adt30::C1(_, _, r401) => {
            let v3 = (*r401).clone();
            let a399 = 1u64;
            let a400 = f67(v3)?;
            nat_add(a399, a400)
        }
    }
}

pub fn f68(v0: Bytes, v1: u64) -> Option<u8> {
    let a402 = v0;
    let a403 = v1;
    index_bytes(a402, a403)
}

pub fn f69(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a404 = v0.clone();
        length_bytes(a404)
    };
    let a405 = v1;
    let a406 = v2;
    if nat_lt(a405, a406) {
        let v3: ((u64, u64), bool) = f78(Fn2::F79(v0), {
            let a407 = v2;
            let a408 = v1;
            nat_sub(a407, a408)
        }, (v1, 0u64))?;
        let a411 = {
            let (h409, _) = {
                let (h410, _) = v3;
                h410
            };
            h409
        };
        let a412 = v2;
        if nat_eq(a411, a412) {
            Ok(Some({
                let (_, h413) = {
                    let (h414, _) = v3;
                    h414
                };
                h413
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f70(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m416 = f68(v0.clone(), {
        let (h415, _) = v1.clone();
        h415
    });
    match m416 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            let m420 = f68(v0, {
                let a418 = {
                    let (h417, _) = v1.clone();
                    h417
                };
                let a419 = 1u64;
                nat_add(a418, a419)?
            });
            match m420 {
                None => {
                    Ok(None::<(u64, Bytes)>)
                }
                Some(v3) => {
                    let m421 = f80(v2, v3)?;
                    match m421 {
                        None => {
                            Ok(None::<(u64, Bytes)>)
                        }
                        Some(v4) => {
                            Ok(Some(({
                                let a423 = {
                                    let (h422, _) = v1;
                                    h422
                                };
                                let a424 = 2u64;
                                nat_add(a423, a424)?
                            }, {
                                let a426 = {
                                    let (_, h425) = v1;
                                    h425
                                };
                                let a427 = f81(f82(0u64), v4, 1u64);
                                append_bytes(a426, a427)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f71(p433: Adt30, v1: Adt0, v2: Adt0) -> bool {
    let m432 = p433;
    {
        let mut m432 = m432;
        loop {
            let step = {
                let v0 = m432;
                let m428 = v0;
                match m428 {
                    Adt30::C0 => {
                        Err::<Adt30, bool>(false)
                    }
                    Adt30::C1(r429, r430, r431) => {
                        let v3 = (*r429).clone();
                        let v4 = (*r430).clone();
                        let v5 = (*r431).clone();
                        if f65(v3, v1.clone()) {
                            if f65(v4, v2.clone()) {
                                Err::<Adt30, bool>(true)
                            } else {
                                Ok::<Adt30, bool>(v5)
                            }
                        } else {
                            Ok::<Adt30, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(p438: Adt0, v1: Adt0) -> bool {
    let m437 = p438;
    {
        let mut m437 = m437;
        loop {
            let step = {
                let v0 = m437;
                let m434 = v0;
                match m434 {
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
                    Adt0::C15(r435, r436) => {
                        let v12 = (*r435).clone();
                        let v13 = (*r436).clone();
                        if f65(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
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

pub fn f73(v0: Adt30, v1: Adt0, v2: Adt0) -> Adt30 {
    if f71(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt30::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v2), std::rc::Rc::new(v0))
    }
}

pub fn f74(v0: Adt0) -> u64 {
    let m439 = v0;
    match m439 {
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

pub fn f75(v0: Adt1) -> u64 {
    let m440 = v0;
    match m440 {
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

pub fn f76(v0: Adt0) -> Adt1 {
    let m441 = v0;
    match m441 {
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

pub fn f77(p447: Adt30, v1: Adt0, v2: Adt0, p448: Adt30) -> Adt30 {
    let m446 = (p447, p448);
    {
        let mut m446 = m446;
        loop {
            let step = {
                let (v0, v3) = m446;
                let m442 = v0;
                match m442 {
                    Adt30::C0 => {
                        Err::<(Adt30, Adt30), Adt30>(v3)
                    }
                    Adt30::C1(r443, r444, r445) => {
                        let v4 = (*r443).clone();
                        let v5 = (*r444).clone();
                        let v6 = (*r445).clone();
                        Ok::<(Adt30, Adt30), Adt30>((v6, if f65(v4, v1.clone()) {
                            f73(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Fn2, p453: u64, p454: (u64, u64)) -> R<((u64, u64), bool)> {
    let m452 = (p453, p454);
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v1, v2) = m452;
                let m449 = v1;
                if m449 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m449 - 1;
                    let m451 = {
                        let c450: Fn2 = v0.clone();
                        c450.apply(v2)?
                    };
                    match m451 {
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
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f83(v0, v1)
}

pub fn f80(v0: u8, v1: u8) -> R<Option<u64>> {
    let m455 = f84(v0)?;
    match m455 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m456 = f84(v1)?;
            match m456 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    Ok(Some({
                        let a459 = {
                            let a457 = v2;
                            let a458 = 16u64;
                            nat_mul(a457, a458)?
                        };
                        let a460 = v3;
                        nat_add(a459, a460)?
                    }))
                }
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m464 = {
        let a461 = v0;
        let a462 = v1;
        let a463 = v2;
        slice_bytes(a461, a462, a463)
    };
    match m464 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f82(_: u64) -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f83(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m466 = f68(v0, {
        let (h465, _) = v1;
        h465
    });
    match m466 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let m467 = f85(v2)?;
            match m467 {
                None => {
                    Ok(None::<(u64, u64)>)
                }
                Some(v3) => {
                    Ok(Some(({
                        let a469 = {
                            let (h468, _) = v1;
                            h468
                        };
                        let a470 = 1u64;
                        nat_add(a469, a470)?
                    }, {
                        let a474 = {
                            let a472 = {
                                let (_, h471) = v1;
                                h471
                            };
                            let a473 = 10u64;
                            nat_mul(a472, a473)?
                        };
                        let a475 = v3;
                        nat_add(a474, a475)?
                    })))
                }
            }
        }
    }
}

pub fn f84(v0: u8) -> R<Option<u64>> {
    let m476 = f85(v0)?;
    match m476 {
        None => {
            let v3: u64 = {
                let v2: u8 = v0;
                let a521 = {
                    let a497 = {
                        let a485 = {
                            let a479 = {
                                let a477 = v2;
                                let a478 = 128u8;
                                fixed_u8::bit_and(a477, a478)
                            };
                            let a480 = 128u8;
                            if equal(a479, a480) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a486 = {
                            let a483 = {
                                let a481 = v2;
                                let a482 = 64u8;
                                fixed_u8::bit_and(a481, a482)
                            };
                            let a484 = 64u8;
                            if equal(a483, a484) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a485, a486)?
                    };
                    let a498 = {
                        let a495 = {
                            let a489 = {
                                let a487 = v2;
                                let a488 = 32u8;
                                fixed_u8::bit_and(a487, a488)
                            };
                            let a490 = 32u8;
                            if equal(a489, a490) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a496 = {
                            let a493 = {
                                let a491 = v2;
                                let a492 = 16u8;
                                fixed_u8::bit_and(a491, a492)
                            };
                            let a494 = 16u8;
                            if equal(a493, a494) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a495, a496)?
                    };
                    nat_add(a497, a498)?
                };
                let a522 = {
                    let a519 = {
                        let a507 = {
                            let a501 = {
                                let a499 = v2;
                                let a500 = 8u8;
                                fixed_u8::bit_and(a499, a500)
                            };
                            let a502 = 8u8;
                            if equal(a501, a502) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a508 = {
                            let a505 = {
                                let a503 = v2;
                                let a504 = 4u8;
                                fixed_u8::bit_and(a503, a504)
                            };
                            let a506 = 4u8;
                            if equal(a505, a506) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a507, a508)?
                    };
                    let a520 = {
                        let a517 = {
                            let a511 = {
                                let a509 = v2;
                                let a510 = 2u8;
                                fixed_u8::bit_and(a509, a510)
                            };
                            let a512 = 2u8;
                            if equal(a511, a512) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a518 = {
                            let a515 = {
                                let a513 = v2;
                                let a514 = 1u8;
                                fixed_u8::bit_and(a513, a514)
                            };
                            let a516 = 1u8;
                            if equal(a515, a516) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a517, a518)?
                    };
                    nat_add(a519, a520)?
                };
                nat_add(a521, a522)?
            };
            let a523 = v3;
            let a524 = 97u64;
            if nat_lt(a523, a524) {
                Ok(None::<u64>)
            } else {
                let a525 = v3;
                let a526 = 103u64;
                if nat_lt(a525, a526) {
                    Ok(Some({
                        let a527 = v3;
                        let a528 = 87u64;
                        nat_sub(a527, a528)
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

pub fn f85(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a573 = {
            let a549 = {
                let a537 = {
                    let a531 = {
                        let a529 = v1;
                        let a530 = 128u8;
                        fixed_u8::bit_and(a529, a530)
                    };
                    let a532 = 128u8;
                    if equal(a531, a532) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a538 = {
                    let a535 = {
                        let a533 = v1;
                        let a534 = 64u8;
                        fixed_u8::bit_and(a533, a534)
                    };
                    let a536 = 64u8;
                    if equal(a535, a536) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a537, a538)?
            };
            let a550 = {
                let a547 = {
                    let a541 = {
                        let a539 = v1;
                        let a540 = 32u8;
                        fixed_u8::bit_and(a539, a540)
                    };
                    let a542 = 32u8;
                    if equal(a541, a542) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a548 = {
                    let a545 = {
                        let a543 = v1;
                        let a544 = 16u8;
                        fixed_u8::bit_and(a543, a544)
                    };
                    let a546 = 16u8;
                    if equal(a545, a546) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a547, a548)?
            };
            nat_add(a549, a550)?
        };
        let a574 = {
            let a571 = {
                let a559 = {
                    let a553 = {
                        let a551 = v1;
                        let a552 = 8u8;
                        fixed_u8::bit_and(a551, a552)
                    };
                    let a554 = 8u8;
                    if equal(a553, a554) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a560 = {
                    let a557 = {
                        let a555 = v1;
                        let a556 = 4u8;
                        fixed_u8::bit_and(a555, a556)
                    };
                    let a558 = 4u8;
                    if equal(a557, a558) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a559, a560)?
            };
            let a572 = {
                let a569 = {
                    let a563 = {
                        let a561 = v1;
                        let a562 = 2u8;
                        fixed_u8::bit_and(a561, a562)
                    };
                    let a564 = 2u8;
                    if equal(a563, a564) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a570 = {
                    let a567 = {
                        let a565 = v1;
                        let a566 = 1u8;
                        fixed_u8::bit_and(a565, a566)
                    };
                    let a568 = 1u8;
                    if equal(a567, a568) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a569, a570)?
            };
            nat_add(a571, a572)?
        };
        nat_add(a573, a574)?
    };
    let a575 = v2;
    let a576 = 48u64;
    if nat_lt(a575, a576) {
        Ok(None::<u64>)
    } else {
        let a577 = v2;
        let a578 = 58u64;
        if nat_lt(a577, a578) {
            Ok(Some({
                let a579 = v2;
                let a580 = 48u64;
                nat_sub(a579, a580)
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: u64) -> R<Option<Adt7>> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7)
}
