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
    C0(Bytes, u64, u64, u64, Option<Adt1>, Option<Bytes>, Option<Bytes>, List<Bytes>, Option<u64>, Option<u64>, Adt3, List<Adt4>, Option<Adt7>, Option<Adt7>, bool, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(Adt2),
    C2(Bytes, Bytes, Adt2),
    C3(Bytes),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C7(Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C11(u64, Bytes, std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>),
    C4(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes),
    C1(Bytes, Bytes),
    C2(Bytes, Bytes),
    C3(Adt1, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, u64, Bytes, List<Adt5>, bool, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, u64, u64, u64, u64, u64, u64, Bytes),
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
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Bytes),
    C1(Bytes),
    C2(u64),
    C3(std::rc::Rc<Adt7>),
    C4,
    C5(Bytes),
    C6(std::rc::Rc<Adt7>),
    C7(std::rc::Rc<Adt7>),
    C8(std::rc::Rc<Adt7>),
    C9(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C10(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C11(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C12(Bytes),
    C13,
    C14(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Bytes, List<Adt9>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(bool, Option<u64>, bool, bool, List<u64>, bool),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<(Bytes, Bytes)>, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(List<(Bytes, Bytes)>, List<Bytes>, Option<Adt1>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<(Bytes, Bytes)>, List<Adt4>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<(Bytes, Bytes)>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<(Bytes, Bytes)>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Bytes>, bool, List<List<(Bytes, Adt17)>>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes),
    C1(bool),
    C2(u64),
    C3(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Adt19>, List<Bytes>, bool, List<(Bytes, Adt17)>, Option<Bytes>, List<Bytes>, bool),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<Bytes>, bool, List<(Bytes, Adt17)>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Bytes, Bytes, Option<Bytes>, Option<List<Bytes>>, u64, Bytes, Option<u64>, Option<Bytes>, Bytes, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, List<Adt21>, bool, Option<Bytes>, Option<Bytes>, bool, Bytes, Option<Adt22>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, Bytes, Bytes, bool, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, Bytes, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<(Bytes, Bytes)>, List<Adt4>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(List<(Bytes, Bytes)>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(Bytes, Bytes)>, List<Adt9>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(List<Adt30>, List<Bytes>, u64, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt1),
    C1(Bytes),
    C2(u64),
    C3(Adt2),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(List<u64>, List<Bytes>, List<(Bytes, (Bytes, Bytes))>, List<Bytes>, List<Bytes>, Option<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Adt33 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(u64, u64, bool, List<Adt33>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Bytes),
    C1(std::rc::Rc<Adt35>),
    C2,
    C3(std::rc::Rc<Adt35>, std::rc::Rc<Adt35>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(u64, List<List<Adt35>>, Option<Adt35>, Option<Adt28>, bool),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(List<Adt38>, List<Adt1>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Adt35),
    C1(Adt35),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(List<Adt7>, Adt31),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(u64, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(List<(Bytes, Bytes)>, Option<u64>, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(List<Adt44>, List<Adt7>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Adt35),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(u64, Bytes, u64, bool),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Bytes, Bytes, Option<Adt49>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt51, List<Adt5>, u64, Option<Adt48>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Adt39, u64, List<Adt39>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt6, Adt51, Option<Adt48>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(u64, Bytes, Adt35),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(List<Adt57>, List<Adt2>, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Adt35),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F20,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt18, p1: Bytes) -> R<Adt18> {
        match self {
            Fn0::F20 => f20(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, List<Adt16>), p1: Adt19) -> R<(bool, List<Adt16>)> {
        match self {
            Fn1::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F62(List<Bytes>, bool),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt24, p1: Adt21) -> R<Adt24> {
        match self {
            Fn2::F62(k0, k1) => f62(k0.clone(), *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F71,
}

impl Fn3 {
    pub fn apply(&self, p0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
        match self {
            Fn3::F71 => f71(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F75,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt26, p1: Adt23) -> R<Adt26> {
        match self {
            Fn4::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F78,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F81(Bytes),
    F308(u8, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn6::F81(k0) => f81(k0.clone(), p0),
            Fn6::F308(k0, k1) => f308(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F85,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt19) -> List<Adt19> {
        match self {
            Fn7::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F89(List<Bytes>),
    F283(Bytes),
    F389(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn8::F89(k0) => f89(k0.clone(), p0, p1),
            Fn8::F283(k0) => Ok(f283(k0.clone(), p0, p1)),
            Fn8::F389(k0) => Ok(f389(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F93(bool, List<Bytes>),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Adt16>, p1: Adt16) -> R<Option<Adt16>> {
        match self {
            Fn9::F93(k0, k1) => f93(*k0, k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F97,
}

impl Fn10 {
    pub fn apply(&self, p0: Option<List<Adt21>>, p1: List<(Bytes, Adt17)>) -> R<Option<List<Adt21>>> {
        match self {
            Fn10::F97 => f97(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F103(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Option<u64>), p1: Bytes) -> R<(u64, Option<u64>)> {
        match self {
            Fn11::F103(k0) => f103(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F116,
}

impl Fn12 {
    pub fn apply(&self, p0: (Option<Bytes>, bool), p1: Bytes) -> (Option<Bytes>, bool) {
        match self {
            Fn12::F116 => f116(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F118,
    F375(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn13::F118 => f118(p0, p1),
            Fn13::F375(k0) => Ok(f375(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F123(List<Bytes>),
}

impl Fn14 {
    pub fn apply(&self, p0: Adt29) -> R<Option<Adt29>> {
        match self {
            Fn14::F123(k0) => f123(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn15::F130 => f130(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F132,
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt4) -> bool {
        match self {
            Fn16::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F134(u64),
}

impl Fn17 {
    pub fn apply(&self, p0: u64, p1: Adt4) -> R<u64> {
        match self {
            Fn17::F134(k0) => f134(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F136(u64),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn18::F136(k0) => f136(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F146(u64, Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn19::F146(k0, k1) => f146(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F150,
}

impl Fn20 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn20::F150 => f150(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F157,
    F165,
    F236,
}

impl Fn21 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn21::F157 => f157(p0, p1),
            Fn21::F165 => f165(p0, p1),
            Fn21::F236 => f236(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F167,
}

impl Fn22 {
    pub fn apply(&self, p0: List<(Bytes, Adt17)>, p1: (Bytes, Adt17)) -> List<(Bytes, Adt17)> {
        match self {
            Fn22::F167 => f167(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F173,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Adt17)) -> List<Bytes> {
        match self {
            Fn23::F173 => f173(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F175(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: Option<Adt17>, p1: (Bytes, Adt17)) -> Option<Adt17> {
        match self {
            Fn24::F175(k0) => f175(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F180,
}

impl Fn25 {
    pub fn apply(&self, p0: List<Adt21>, p1: Adt21) -> List<Adt21> {
        match self {
            Fn25::F180 => f180(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F182,
}

impl Fn26 {
    pub fn apply(&self, p0: Option<List<Adt23>>, p1: List<(Bytes, Adt17)>) -> R<Option<List<Adt23>>> {
        match self {
            Fn26::F182 => f182(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F186(u64),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64), p1: u64) -> R<(u64, u64)> {
        match self {
            Fn27::F186(k0) => f186(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F188(bool, u64, Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Adt32) -> R<Option<Adt32>> {
        match self {
            Fn28::F188(k0, k1, k2) => f188(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F191(u64, Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn29::F191(k0, k1) => f191(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F195(u64, Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: Adt34) -> R<Option<Adt34>> {
        match self {
            Fn30::F195(k0, k1) => f195(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F199(u64, List<Adt33>),
}

impl Fn31 {
    pub fn apply(&self, p0: Adt36) -> R<Option<Adt36>> {
        match self {
            Fn31::F199(k0, k1) => f199(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F203(u64),
}

impl Fn32 {
    pub fn apply(&self, p0: Adt37) -> R<Option<Adt37>> {
        match self {
            Fn32::F203(k0) => f203(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F219(u64),
}

impl Fn33 {
    pub fn apply(&self, p0: Adt40) -> R<Option<Adt40>> {
        match self {
            Fn33::F219(k0) => f219(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F223,
}

impl Fn34 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> Bytes {
        match self {
            Fn34::F223 => f223(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F225,
    F294,
}

impl Fn35 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn35::F225 => f225(p0, p1),
            Fn35::F294 => f294(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F227(u64),
}

impl Fn36 {
    pub fn apply(&self, p0: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
        match self {
            Fn36::F227(k0) => f227(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F229(Bytes, List<Adt4>, u64),
    F354(Bytes),
}

impl Fn37 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: Bytes) -> R<List<(Bytes, Bytes)>> {
        match self {
            Fn37::F229(k0, k1, k2) => f229(k0.clone(), k1.clone(), *k2, p0, p1),
            Fn37::F354(k0) => f354(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F238(Bytes, u64),
}

impl Fn38 {
    pub fn apply(&self, p0: Adt41) -> R<Option<Adt41>> {
        match self {
            Fn38::F238(k0, k1) => f238(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F241(Bytes, u64, Bytes, u64),
    F265(u64, Bytes),
    F305(Bytes),
    F306(u64, Bytes),
    F400(u64, Bytes),
    F418(Bytes),
    F517(u64, Bytes),
    F542(u64, Bytes),
    F549(u64, Bytes),
    F553(Bytes),
}

impl Fn39 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn39::F241(k0, k1, k2, k3) => f241(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn39::F265(k0, k1) => f265(*k0, k1.clone(), p0),
            Fn39::F305(k0) => Ok(f305(k0.clone(), p0)),
            Fn39::F306(k0, k1) => f306(*k0, k1.clone(), p0),
            Fn39::F400(k0, k1) => f400(*k0, k1.clone(), p0),
            Fn39::F418(k0) => f418(k0.clone(), p0),
            Fn39::F517(k0, k1) => f517(*k0, k1.clone(), p0),
            Fn39::F542(k0, k1) => f542(*k0, k1.clone(), p0),
            Fn39::F549(k0, k1) => f549(*k0, k1.clone(), p0),
            Fn39::F553(k0) => f553(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F244(Bytes),
}

impl Fn40 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt17)) -> bool {
        match self {
            Fn40::F244(k0) => f244(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F250(bool, List<Bytes>),
}

impl Fn41 {
    pub fn apply(&self, p0: bool, p1: Adt16) -> R<bool> {
        match self {
            Fn41::F250(k0, k1) => f250(*k0, k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F253(bool, List<Bytes>, List<(Bytes, Adt17)>),
    F314,
}

impl Fn42 {
    pub fn apply(&self, p0: List<Adt16>, p1: Adt16) -> R<List<Adt16>> {
        match self {
            Fn42::F253(k0, k1, k2) => f253(*k0, k1.clone(), k2.clone(), p0, p1),
            Fn42::F314 => Ok(f314(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F259,
}

impl Fn43 {
    pub fn apply(&self, p0: List<Adt23>, p1: Adt23) -> List<Adt23> {
        match self {
            Fn43::F259 => f259(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F268,
}

impl Fn44 {
    pub fn apply(&self, p0: List<Adt33>, p1: Adt33) -> List<Adt33> {
        match self {
            Fn44::F268 => f268(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F276(Bytes),
}

impl Fn45 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn45::F276(k0) => f276(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F281,
}

impl Fn46 {
    pub fn apply(&self, p0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
        match self {
            Fn46::F281 => f281(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F292(u64),
}

impl Fn47 {
    pub fn apply(&self, p0: Adt43) -> R<Option<Adt43>> {
        match self {
            Fn47::F292(k0) => f292(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn48 {
    F296,
}

impl Fn48 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn48::F296 => f296(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn49 {
    F304,
}

impl Fn49 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, List<Bytes>), p1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
        match self {
            Fn49::F304 => f304(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn50 {
    F310(Bytes, u64),
}

impl Fn50 {
    pub fn apply(&self, p0: Adt45) -> R<Option<Adt45>> {
        match self {
            Fn50::F310(k0, k1) => f310(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn51 {
    F313(Bytes),
}

impl Fn51 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn51::F313(k0) => f313(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn52 {
    F333,
}

impl Fn52 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn52::F333 => f333(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn53 {
    F335(Bytes, u64),
}

impl Fn53 {
    pub fn apply(&self, p0: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
        match self {
            Fn53::F335(k0, k1) => f335(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn54 {
    F349(List<Bytes>, Bytes),
}

impl Fn54 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: Adt5) -> R<List<(Bytes, Bytes)>> {
        match self {
            Fn54::F349(k0, k1) => f349(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn55 {
    F351,
}

impl Fn55 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn55::F351 => f351(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn56 {
    F353(u64),
}

impl Fn56 {
    pub fn apply(&self, p0: (u64, Option<Bytes>), p1: Adt5) -> R<(u64, Option<Bytes>)> {
        match self {
            Fn56::F353(k0) => f353(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn57 {
    F358,
    F409,
    F558,
}

impl Fn57 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn57::F358 => f358(p0, p1),
            Fn57::F409 => f409(p0, p1),
            Fn57::F558 => f558(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn58 {
    F364(u64),
}

impl Fn58 {
    pub fn apply(&self, p0: (bool, List<u64>), p1: u64) -> (bool, List<u64>) {
        match self {
            Fn58::F364(k0) => f364(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn59 {
    F392,
}

impl Fn59 {
    pub fn apply(&self, p0: (Adt47, Adt46)) -> (Adt47, Adt46) {
        match self {
            Fn59::F392 => f392(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn60 {
    F394(u64, Bytes, u64),
}

impl Fn60 {
    pub fn apply(&self, p0: (u64, Adt39)) -> R<(u64, Adt39)> {
        match self {
            Fn60::F394(k0, k1, k2) => f394(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn61 {
    F398,
}

impl Fn61 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn61::F398 => f398(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn62 {
    F402(u64, u64, Bytes, Bytes),
}

impl Fn62 {
    pub fn apply(&self, p0: Adt50) -> R<Option<Adt50>> {
        match self {
            Fn62::F402(k0, k1, k2, k3) => f402(*k0, *k1, k2.clone(), k3.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn63 {
    F411,
}

impl Fn63 {
    pub fn apply(&self, p0: (Adt7, List<Adt7>)) -> Option<(Adt7, List<Adt7>)> {
        match self {
            Fn63::F411 => f411(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn64 {
    F415(Bytes),
}

impl Fn64 {
    pub fn apply(&self, p0: Option<Adt4>, p1: Adt4) -> Option<Adt4> {
        match self {
            Fn64::F415(k0) => f415(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn65 {
    F421,
}

impl Fn65 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt17)>>, p1: List<(Bytes, Adt17)>) -> List<List<(Bytes, Adt17)>> {
        match self {
            Fn65::F421 => f421(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn66 {
    F423,
}

impl Fn66 {
    pub fn apply(&self, p0: Adt35, p1: Adt35) -> Adt35 {
        match self {
            Fn66::F423 => f423(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn67 {
    F426,
}

impl Fn67 {
    pub fn apply(&self, p0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
        match self {
            Fn67::F426 => f426(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn68 {
    F433,
}

impl Fn68 {
    pub fn apply(&self, p0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
        match self {
            Fn68::F433 => f433(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn69 {
    F435,
}

impl Fn69 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn69::F435 => f435(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn70 {
    F437,
}

impl Fn70 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn70::F437 => f437(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn71 {
    F439(u64),
}

impl Fn71 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1), p1: Adt35) -> R<(List<Adt1>, Adt1)> {
        match self {
            Fn71::F439(k0) => f439(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn72 {
    F441,
}

impl Fn72 {
    pub fn apply(&self, p0: Adt52) -> Adt52 {
        match self {
            Fn72::F441 => f441(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn73 {
    F449,
}

impl Fn73 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn73::F449 => f449(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn74 {
    F466,
}

impl Fn74 {
    pub fn apply(&self, p0: List<Adt35>, p1: Adt35) -> List<Adt35> {
        match self {
            Fn74::F466 => f466(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn75 {
    F473,
}

impl Fn75 {
    pub fn apply(&self, p0: List<Adt38>, p1: Adt35) -> List<Adt38> {
        match self {
            Fn75::F473 => f473(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn76 {
    F492,
}

impl Fn76 {
    pub fn apply(&self, p0: (List<Adt7>, List<Adt7>)) -> Option<(List<Adt7>, List<Adt7>)> {
        match self {
            Fn76::F492 => f492(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn77 {
    F494,
}

impl Fn77 {
    pub fn apply(&self, p0: Adt7, p1: Adt7) -> Adt7 {
        match self {
            Fn77::F494 => f494(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn78 {
    F503(u64),
}

impl Fn78 {
    pub fn apply(&self, p0: Result<List<Adt38>, Adt28>, p1: Adt35) -> R<Result<List<Adt38>, Adt28>> {
        match self {
            Fn78::F503(k0) => f503(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn79 {
    F521,
}

impl Fn79 {
    pub fn apply(&self, p0: List<Adt44>, p1: Adt35) -> List<Adt44> {
        match self {
            Fn79::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn80 {
    F523(u64),
}

impl Fn80 {
    pub fn apply(&self, p0: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
        match self {
            Fn80::F523(k0) => f523(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn81 {
    F525(u64),
}

impl Fn81 {
    pub fn apply(&self, p0: Adt56) -> R<Option<Adt56>> {
        match self {
            Fn81::F525(k0) => f525(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn82 {
    F527(u64),
}

impl Fn82 {
    pub fn apply(&self, p0: Result<List<Adt2>, Adt28>, p1: Adt35) -> R<Result<List<Adt2>, Adt28>> {
        match self {
            Fn82::F527(k0) => f527(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn83 {
    F534(u64, u64, Bytes),
}

impl Fn83 {
    pub fn apply(&self, p0: Adt51) -> R<Option<Adt51>> {
        match self {
            Fn83::F534(k0, k1, k2) => f534(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn84 {
    F546,
}

impl Fn84 {
    pub fn apply(&self, p0: Adt2, p1: Adt2) -> Adt2 {
        match self {
            Fn84::F546 => f546(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn85 {
    F548,
}

impl Fn85 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn85::F548 => f548(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn86 {
    F570,
}

impl Fn86 {
    pub fn apply(&self, p0: List<Adt57>, p1: Adt35) -> List<Adt57> {
        match self {
            Fn86::F570 => f570(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn87 {
    F572,
}

impl Fn87 {
    pub fn apply(&self, p0: (List<Adt2>, List<Adt2>)) -> Option<(List<Adt2>, List<Adt2>)> {
        match self {
            Fn87::F572 => f572(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: bool, v2: List<Bytes>, v3: u64) -> R<Result<Adt0, List<(Bytes, Bytes)>>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        None => {
            Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f3(Str::lit("invalid entry file"))), List::<(Bytes, Bytes)>::nil())))
        }
        Some(v4) => {
            let m2 = f4(v4)?;
            match m2 {
                None => {
                    Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f3(Str::lit("invalid entry file"))), List::<(Bytes, Bytes)>::nil())))
                }
                Some(v5) => {
                    let a4 = match v5.clone() {
                        Adt20::C0(h3, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3
                        }
                    };
                    let a5 = f3(Str::lit("lexlean/entry/1"));
                    if !equal(a4, a5) {
                        Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons((f3(Str::lit("LLC0103")), f5(f5(f3(Str::lit("unsupported entry schema ")), f6(match v5 {
                            Adt20::C0(h6, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h6
                            }
                        })), Bytes::lit(&[]))), List::<(Bytes, Bytes)>::nil())))
                    } else {
                        let m8 = f7(0u64, match v5.clone() {
                            Adt20::C0(_, _, h7, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h7
                            }
                        })?;
                        match m8 {
                            None => {
                                Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f5(f6(match v5 {
                                    Adt20::C0(_, _, h9, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h9
                                    }
                                }), f3(Str::lit(" is not a category")))), List::<(Bytes, Bytes)>::nil())))
                            }
                            Some(v6) => {
                                let m11 = f7(1u64, match v5.clone() {
                                    Adt20::C0(_, _, _, _, _, _, h10, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h10
                                    }
                                })?;
                                match m11 {
                                    None => {
                                        Ok(Err::<Adt0, List<(Bytes, Bytes)>>(List::cons(f2(f5(f6(match v5 {
                                            Adt20::C0(_, _, _, _, _, _, h12, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h12
                                            }
                                        }), f3(Str::lit(" is not a frame")))), List::<(Bytes, Bytes)>::nil())))
                                    }
                                    Some(v7) => {
                                        let v8: Adt10 = f8(v6, v7, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h13, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h13
                                            }
                                        })?;
                                        let v9: Adt11 = f9(match v5.clone() {
                                            Adt20::C0(_, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h14
                                            }
                                        }, v6, v7, match v5.clone() {
                                            Adt20::C0(_, _, h15, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h15
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, h16, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h16
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h17, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h17
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, h18, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h18
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, h19, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h19
                                            }
                                        }, v1)?;
                                        let v10: Adt12 = f10(match v5.clone() {
                                            Adt20::C0(_, h20, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h20
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, h21, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h21
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, h22, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h22
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h23
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, h24, _, _) => {
                                                h24
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h25, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h25
                                            }
                                        }, v3, match v9.clone() {
                                            Adt11::C0(h26, _, _) => {
                                                h26
                                            }
                                        })?;
                                        let v11: (List<(Bytes, Bytes)>, Adt3) = f11(v1, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, h27, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h27
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, h28, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h28
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, h29, _, _, _, _, _, _, _, _, _, _) => {
                                                h29
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, h30, _, _, _, _, _, _, _, _, _) => {
                                                h30
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h31, _, _, _, _, _, _, _, _) => {
                                                h31
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h32, _, _, _, _, _, _, _) => {
                                                h32
                                            }
                                        }, v3, match v10.clone() {
                                            Adt12::C0(_, h33, _, _, _) => {
                                                h33
                                            }
                                        }, match v10.clone() {
                                            Adt12::C0(h34, _, _, _, _) => {
                                                h34
                                            }
                                        })?;
                                        let v12: Adt13 = f12(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h35, _, _, _, _, _, _) => {
                                                h35
                                            }
                                        }, v1, v2, {
                                            let (h36, _) = v11;
                                            h36
                                        })?;
                                        let v13: List<(Bytes, Bytes)> = f13(match v5.clone() {
                                            Adt20::C0(_, _, h37, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h37
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, _, h38) => {
                                                h38
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, h39, _) => {
                                                h39
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(_, h40) => {
                                                h40
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(h41, _) => {
                                                h41
                                            }
                                        })?;
                                        let v14: Adt14 = f14(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h42, _, _, _, _, _) => {
                                                h42
                                            }
                                        }, v6, match v5.clone() {
                                            Adt20::C0(_, _, h43, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h43
                                            }
                                        }, match v8.clone() {
                                            Adt10::C0(_, _, _, _, _, h44) => {
                                                h44
                                            }
                                        }, match v8 {
                                            Adt10::C0(_, _, _, _, h45, _) => {
                                                h45
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h46, _, _, _, _) => {
                                                h46
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h47, _, _, _) => {
                                                h47
                                            }
                                        }, match v12.clone() {
                                            Adt13::C0(_, h48) => {
                                                h48
                                            }
                                        }, v3, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, h49, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h49
                                            }
                                        }, v7, v13)?;
                                        let v15: (List<(Bytes, Bytes)>, bool) = f15(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h50, _, _) => {
                                                h50
                                            }
                                        }, match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h51, _) => {
                                                h51
                                            }
                                        }, v6, match v14.clone() {
                                            Adt14::C0(h52, _, _) => {
                                                h52
                                            }
                                        });
                                        let v17: bool = {
                                            let m54 = match v10.clone() {
                                                Adt12::C0(_, _, h53, _, _) => {
                                                    h53
                                                }
                                            };
                                            match m54 {
                                                None => {
                                                    false
                                                }
                                                Some(v16) => {
                                                    let (_, h55) = f16(v16, {
                                                        let a56 = v0;
                                                        length_bytes(a56)
                                                    })?;
                                                    h55
                                                }
                                            }
                                        };
                                        let v18: Adt15 = f17(match v5.clone() {
                                            Adt20::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h57) => {
                                                h57
                                            }
                                        }, v17, match v5.clone() {
                                            Adt20::C0(_, h58, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                h58
                                            }
                                        }, {
                                            let (h59, _) = v15;
                                            h59
                                        })?;
                                        let m61 = match v18.clone() {
                                            Adt15::C0(h60, _) => {
                                                h60
                                            }
                                        };
                                        match m61.uncons() {
                                            None => {
                                                Ok(Ok::<Adt0, List<(Bytes, Bytes)>>(Adt0::C0(match v5.clone() {
                                                    Adt20::C0(_, h63, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        h63
                                                    }
                                                }, v6, v7, match v5 {
                                                    Adt20::C0(_, _, _, _, _, h64, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        h64
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, h65, _, _) => {
                                                        h65
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, _, h66, _) => {
                                                        h66
                                                    }
                                                }, match v10.clone() {
                                                    Adt12::C0(_, _, _, _, h67) => {
                                                        h67
                                                    }
                                                }, match v10 {
                                                    Adt12::C0(_, h68, _, _, _) => {
                                                        h68
                                                    }
                                                }, match v9.clone() {
                                                    Adt11::C0(_, h69, _) => {
                                                        h69
                                                    }
                                                }, match v9 {
                                                    Adt11::C0(_, _, h70) => {
                                                        h70
                                                    }
                                                }, {
                                                    let (_, h71) = v11;
                                                    h71
                                                }, match v12 {
                                                    Adt13::C0(_, h72) => {
                                                        h72
                                                    }
                                                }, match v14.clone() {
                                                    Adt14::C0(_, h73, _) => {
                                                        h73
                                                    }
                                                }, match v14 {
                                                    Adt14::C0(_, _, h74) => {
                                                        h74
                                                    }
                                                }, {
                                                    let (_, h75) = v15;
                                                    h75
                                                }, match v18 {
                                                    Adt15::C0(_, h76) => {
                                                        h76
                                                    }
                                                })))
                                            }
                                            Some((_, _)) => {
                                                Ok(Err::<Adt0, List<(Bytes, Bytes)>>(f18(match v18 {
                                                    Adt15::C0(h62, _) => {
                                                        h62
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Bytes) -> R<Option<List<Adt16>>> {
    let v1: Adt18 = f19(Fn0::F20, Adt18::C0(List::<Adt19>::nil(), List::<Bytes>::nil(), false, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false), f21(v0)?)?;
    if match v1.clone() {
        Adt18::C0(_, _, _, _, _, _, h77) => {
            h77
        }
    } {
        Ok(None::<List<Adt16>>)
    } else {
        let m79 = match v1.clone() {
            Adt18::C0(_, _, _, _, h78, _, _) => {
                h78
            }
        };
        match m79 {
            None => {
                let v3: List<Adt19> = f22(f23(match v1.clone() {
                    Adt18::C0(h80, _, _, _, _, _, _) => {
                        h80
                    }
                }, match v1.clone() {
                    Adt18::C0(_, h81, _, _, _, _, _) => {
                        h81
                    }
                }, match v1.clone() {
                    Adt18::C0(_, _, h82, _, _, _, _) => {
                        h82
                    }
                }, match v1 {
                    Adt18::C0(_, _, _, h83, _, _, _) => {
                        h83
                    }
                }));
                let v4: (bool, List<Adt16>) = f24(Fn1::F25, (false, List::<Adt16>::nil()), v3)?;
                let (h84, _) = v4;
                if h84 {
                    Ok(None::<List<Adt16>>)
                } else {
                    Ok(Some({
                        let (_, h85) = v4;
                        h85
                    }))
                }
            }
            Some(_) => {
                Ok(None::<List<Adt16>>)
            }
        }
    }
}

pub fn f2(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLR3004")), v0)
}

pub fn f3(v0: Str) -> Bytes {
    let a86 = v0;
    utf8_encode(a86)
}

pub fn f4(v0: List<Adt16>) -> R<Option<Adt20>> {
    let v1: List<(Bytes, Adt17)> = f26(v0.clone())?;
    if f27(v1.clone(), f28())? {
        let m87 = f29(v1.clone(), f3(Str::lit("spec")));
        match m87 {
            None => {
                Ok(None::<Adt20>)
            }
            Some(v2) => {
                let m88 = f29(v1.clone(), f3(Str::lit("id")));
                match m88 {
                    None => {
                        Ok(None::<Adt20>)
                    }
                    Some(v3) => {
                        let m89 = f29(v1.clone(), f3(Str::lit("category")));
                        match m89 {
                            None => {
                                Ok(None::<Adt20>)
                            }
                            Some(v4) => {
                                let m90 = f30(v1.clone(), f3(Str::lit("signature")));
                                match m90 {
                                    None => {
                                        Ok(None::<Adt20>)
                                    }
                                    Some(v5) => {
                                        let m91 = f31(v1.clone(), f3(Str::lit("universes")));
                                        match m91 {
                                            None => {
                                                Ok(None::<Adt20>)
                                            }
                                            Some(v6) => {
                                                let m92 = f32(v1.clone(), f3(Str::lit("surface_arity")));
                                                match m92 {
                                                    None => {
                                                        Ok(None::<Adt20>)
                                                    }
                                                    Some(v7) => {
                                                        let m93 = f29(v1.clone(), f3(Str::lit("frame")));
                                                        match m93 {
                                                            None => {
                                                                Ok(None::<Adt20>)
                                                            }
                                                            Some(v8) => {
                                                                let m94 = f33(v1.clone(), f3(Str::lit("precedence")));
                                                                match m94 {
                                                                    None => {
                                                                        Ok(None::<Adt20>)
                                                                    }
                                                                    Some(v9) => {
                                                                        let m95 = f30(v1.clone(), f3(Str::lit("associativity")));
                                                                        match m95 {
                                                                            None => {
                                                                                Ok(None::<Adt20>)
                                                                            }
                                                                            Some(v10) => {
                                                                                let m96 = f34(v0.clone(), List::cons(f3(Str::lit("denotation")), List::<Bytes>::nil()), false)?;
                                                                                match m96 {
                                                                                    None => {
                                                                                        Ok(None::<Adt20>)
                                                                                    }
                                                                                    Some(v11) => {
                                                                                        let v12: List<(Bytes, Adt17)> = f35(v11);
                                                                                        let m97 = f36(v12)?;
                                                                                        match m97 {
                                                                                            None => {
                                                                                                Ok(None::<Adt20>)
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let v14: Bytes = match v13.clone() {
                                                                                                    Adt27::C0(h98, _, _, _, _, _) => {
                                                                                                        h98
                                                                                                    }
                                                                                                };
                                                                                                let v15: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, h99, _, _, _, _) => {
                                                                                                        h99
                                                                                                    }
                                                                                                };
                                                                                                let v16: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, h100, _, _, _) => {
                                                                                                        h100
                                                                                                    }
                                                                                                };
                                                                                                let v17: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, _, h101, _, _) => {
                                                                                                        h101
                                                                                                    }
                                                                                                };
                                                                                                let v18: Option<Bytes> = match v13.clone() {
                                                                                                    Adt27::C0(_, _, _, _, h102, _) => {
                                                                                                        h102
                                                                                                    }
                                                                                                };
                                                                                                let v19: Option<Bytes> = match v13 {
                                                                                                    Adt27::C0(_, _, _, _, _, h103) => {
                                                                                                        h103
                                                                                                    }
                                                                                                };
                                                                                                let m104 = f37(v0.clone(), v1)?;
                                                                                                match m104 {
                                                                                                    None => {
                                                                                                        Ok(None::<Adt20>)
                                                                                                    }
                                                                                                    Some(v20) => {
                                                                                                        let m105 = f38(v20)?;
                                                                                                        match m105 {
                                                                                                            None => {
                                                                                                                Ok(None::<Adt20>)
                                                                                                            }
                                                                                                            Some(v21) => {
                                                                                                                let v23: bool = {
                                                                                                                    let m106 = f34(v0.clone(), List::cons(f3(Str::lit("render")), List::<Bytes>::nil()), false)?;
                                                                                                                    match m106 {
                                                                                                                        None => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        Some(_) => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                let v25: List<(Bytes, Adt17)> = {
                                                                                                                    let m107 = f34(v0.clone(), List::cons(f3(Str::lit("render")), List::<Bytes>::nil()), false)?;
                                                                                                                    match m107 {
                                                                                                                        None => {
                                                                                                                            List::<(Bytes, Adt17)>::nil()
                                                                                                                        }
                                                                                                                        Some(v24) => {
                                                                                                                            f35(v24)
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                let m108 = f30(v25.clone(), f3(Str::lit("math")));
                                                                                                                match m108 {
                                                                                                                    None => {
                                                                                                                        Ok(None::<Adt20>)
                                                                                                                    }
                                                                                                                    Some(v26) => {
                                                                                                                        let m109 = f30(v25, f3(Str::lit("text")));
                                                                                                                        match m109 {
                                                                                                                            None => {
                                                                                                                                Ok(None::<Adt20>)
                                                                                                                            }
                                                                                                                            Some(v27) => {
                                                                                                                                let v29: bool = {
                                                                                                                                    let m110 = f34(v0.clone(), List::cons(f3(Str::lit("calculation")), List::<Bytes>::nil()), false)?;
                                                                                                                                    match m110 {
                                                                                                                                        None => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        Some(_) => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let v31: List<(Bytes, Adt17)> = {
                                                                                                                                    let m111 = f34(v0.clone(), List::cons(f3(Str::lit("calculation")), List::<Bytes>::nil()), false)?;
                                                                                                                                    match m111 {
                                                                                                                                        None => {
                                                                                                                                            List::<(Bytes, Adt17)>::nil()
                                                                                                                                        }
                                                                                                                                        Some(v30) => {
                                                                                                                                            f35(v30)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let m112 = if v29 {
                                                                                                                                    f29(v31, f3(Str::lit("kind")))
                                                                                                                                } else {
                                                                                                                                    Some(Bytes::lit(&[]))
                                                                                                                                };
                                                                                                                                match m112 {
                                                                                                                                    None => {
                                                                                                                                        Ok(None::<Adt20>)
                                                                                                                                    }
                                                                                                                                    Some(v32) => {
                                                                                                                                        let m113 = f39(v0)?;
                                                                                                                                        match m113 {
                                                                                                                                            None => {
                                                                                                                                                Ok(None::<Adt20>)
                                                                                                                                            }
                                                                                                                                            Some(v33) => {
                                                                                                                                                Ok(Some(Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9, v10, v14, v15, v16, v17, v18, v19, v21, v23, v26, v27, v29, v32, v33)))
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt20>)
    }
}

pub fn f5(v0: Bytes, v1: Bytes) -> Bytes {
    let a114 = v0;
    let a115 = v1;
    append_bytes(a114, a115)
}

pub fn f6(v0: Bytes) -> Bytes {
    f40(f3(Str::lit("`")), v0, f3(Str::lit("`")))
}

pub fn f7(v0: u64, v1: Bytes) -> R<Option<u64>> {
    f41(f42(v0), v1)
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<Adt10> {
    Ok(Adt10::C0({
        let a116 = v1;
        let a117 = f43(v0, v2)?;
        nat_eq(a116, a117)
    }, f44(v1), {
        let a118 = v0;
        let a119 = 2u64;
        nat_lt(a118, a119)
    }, {
        let a120 = 2u64;
        let a121 = v0;
        nat_lt(a120, a121)
    }, f45(v0)?, {
        let a122 = v0;
        let a123 = 3u64;
        if nat_eq(a122, a123) {
            true
        } else {
            let a124 = v0;
            let a125 = 4u64;
            nat_eq(a124, a125)
        }
    }))
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: Bytes, v4: Bytes, v5: u64, v6: Option<u64>, v7: Option<Bytes>, v8: bool) -> R<Adt11> {
    let v9: Adt10 = f8(v1, v2, v5)?;
    let v10: List<(Bytes, Bytes)> = f46(List::<(Bytes, Bytes)>::nil(), if f47(v0.clone())? {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f6(v0), f3(Str::lit(" is not a valid entry ID")))))
    });
    let v11: List<(Bytes, Bytes)> = f46(v10, if if match v9.clone() {
        Adt10::C0(_, _, h126, _, _, _) => {
            h126
        }
    } {
        !v8
    } else {
        false
    } {
        Some(f2(f40(f3(Str::lit("a non-core entry cannot use category `")), v3.clone(), f3(Str::lit("`")))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v12: List<(Bytes, Bytes)> = f46(v11, if match v9.clone() {
        Adt10::C0(h127, _, _, _, _, _) => {
            h127
        }
    } {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f5(f5(f5(f3(Str::lit("category `")), v3), f3(Str::lit("` does not permit frame `"))), v4.clone()), f3(Str::lit("`")))))
    });
    let v14: List<(Bytes, Bytes)> = f46(v12, {
        let m129 = match v9 {
            Adt10::C0(_, h128, _, _, _, _) => {
                h128
            }
        };
        match m129 {
            None => {
                let a132 = v5;
                let a133 = 0u64;
                if nat_eq(a132, a133) {
                    Some(f2(f3(Str::lit("a call frame needs surface_arity >= 1"))))
                } else {
                    None::<(Bytes, Bytes)>
                }
            }
            Some(v13) => {
                let a130 = v5;
                let a131 = v13;
                if nat_eq(a130, a131) {
                    None::<(Bytes, Bytes)>
                } else {
                    Some(f2(f5(f5(f5(f5(f3(Str::lit("frame `")), v4), f3(Str::lit("` fixes surface_arity "))), f48(v13)), f5(f3(Str::lit(", found ")), f48(v5)))))
                }
            }
        }
    });
    let v15: bool = {
        let a134 = v2;
        let a135 = 2u64;
        if nat_eq(a134, a135) {
            true
        } else {
            let a136 = v2;
            let a137 = 3u64;
            if nat_eq(a136, a137) {
                true
            } else {
                let a138 = v2;
                let a139 = 4u64;
                nat_eq(a138, a139)
            }
        }
    };
    let v17: (List<(Bytes, Bytes)>, Option<u64>) = {
        let m140 = v6;
        match m140 {
            None => {
                if v15 {
                    (f46(v14, Some(f2(f3(Str::lit("this frame requires a precedence"))))), None::<u64>)
                } else {
                    (v14, None::<u64>)
                }
            }
            Some(v16) => {
                if v15 {
                    let a141 = v16;
                    let a142 = 255u64;
                    if nat_le(a141, a142) {
                        (v14, Some(v16))
                    } else {
                        (f46(v14, Some(f2(f3(Str::lit("precedence is 0..255"))))), None::<u64>)
                    }
                } else {
                    (f46(v14, Some(f2(f3(Str::lit("precedence does not apply to this frame"))))), None::<u64>)
                }
            }
        }
    };
    let v18: List<(Bytes, Bytes)> = {
        let (h143, _) = v17;
        h143
    };
    let v19: Option<u64> = {
        let (_, h144) = v17;
        h144
    };
    let v20: bool = {
        let a145 = v2;
        let a146 = 4u64;
        nat_eq(a145, a146)
    };
    let v22: (List<(Bytes, Bytes)>, Option<u64>) = {
        let m147 = v7;
        match m147 {
            None => {
                if v20 {
                    (f46(v18, Some(f2(f3(Str::lit("an infix frame requires an associativity"))))), None::<u64>)
                } else {
                    (v18, None::<u64>)
                }
            }
            Some(v21) => {
                if v20 {
                    let a148 = v21.clone();
                    let a149 = f3(Str::lit("left"));
                    if equal(a148, a149) {
                        (v18, Some(0u64))
                    } else {
                        let a150 = v21.clone();
                        let a151 = f3(Str::lit("right"));
                        if equal(a150, a151) {
                            (v18, Some(1u64))
                        } else {
                            let a152 = v21;
                            let a153 = f3(Str::lit("none"));
                            if equal(a152, a153) {
                                (v18, Some(2u64))
                            } else {
                                (f46(v18, Some(f2(f3(Str::lit("associativity is left, right, or none"))))), None::<u64>)
                            }
                        }
                    }
                } else {
                    (f46(v18, Some(f2(f3(Str::lit("associativity applies only to infix frames"))))), None::<u64>)
                }
            }
        }
    };
    Ok(Adt11::C0({
        let (h154, _) = v22;
        h154
    }, v19, {
        let (_, h155) = v22;
        h155
    }))
}

pub fn f10(v0: Bytes, v1: Bytes, v2: Option<List<Bytes>>, v3: Option<Bytes>, v4: bool, v5: u64, v6: u64, v7: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v9: List<Bytes> = {
        let m156 = v2.clone();
        match m156 {
            None => {
                List::<Bytes>::nil()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v10: List<(Bytes, Bytes)> = f46(v7, if f49(v9.clone()) {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f3(Str::lit("universes must be sorted and unique"))))
    });
    let v12: List<(Bytes, Bytes)> = {
        let m157 = f50(v9.clone())?;
        match m157 {
            None => {
                v10
            }
            Some(v11) => {
                f46(v10, Some(f2(f5(f6(v11), f3(Str::lit(" is not a universe name"))))))
            }
        }
    };
    let m158 = v3;
    match m158 {
        None => {
            if v4 {
                Ok(Adt12::C0(f46(v12, Some(f2(f3(Str::lit("every semantic entry has a complete LSE signature"))))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            } else {
                Ok(Adt12::C0(f46(v12, {
                    let m167 = v2;
                    match m167 {
                        None => {
                            None::<(Bytes, Bytes)>
                        }
                        Some(_) => {
                            Some(f2(f3(Str::lit("universes apply only with a signature"))))
                        }
                    }
                }), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            }
        }
        Some(v13) => {
            if v4 {
                let m159 = f51(v13.clone(), v6)?;
                match m159 {
                    Ok(v14) => {
                        let v15: u64 = {
                            let a160 = v13;
                            length_bytes(a160)
                        };
                        let v17: List<(Bytes, Bytes)> = {
                            let m161 = f52(v14.clone(), v9.clone(), v15)?;
                            match m161 {
                                None => {
                                    v12
                                }
                                Some(v16) => {
                                    f46(v12, Some(f2(f5(f3(Str::lit("invalid signature: ")), v16))))
                                }
                            }
                        };
                        let v18: (u64, bool) = f16(v14.clone(), v15)?;
                        let v19: u64 = {
                            let (h162, _) = v18;
                            h162
                        };
                        let v20: List<(Bytes, Bytes)> = {
                            let a163 = v5;
                            let a164 = v19;
                            if nat_eq(a163, a164) {
                                v17
                            } else {
                                f46(v17, Some(f2(f40(f40(f6(v0), f3(Str::lit(": surface_arity ")), f48(v5)), f3(Str::lit(" does not match the ")), f5(f48(v19), {
                                    let a165 = v19;
                                    let a166 = 1u64;
                                    if nat_eq(a165, a166) {
                                        f3(Str::lit(" explicit binder of the signature's outer pi"))
                                    } else {
                                        f3(Str::lit(" explicit binders of the signature's outer pi"))
                                    }
                                })))))
                            }
                        };
                        let v21: Bytes = f53(v14.clone(), true)?;
                        let v22: Bytes = f54(v21.clone())?;
                        Ok(Adt12::C0(v20, v9, Some(v14), Some(v21), Some(v22)))
                    }
                    Err(v23) => {
                        Ok(Adt12::C0(f46(v12, Some(f55(f3(Str::lit("signature")), v23))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
                    }
                }
            } else {
                Ok(Adt12::C0(f46(v12, Some(f2(f40(f3(Str::lit("category `")), v1, f3(Str::lit("` does not carry a signature")))))), v9, None::<Adt1>, None::<Bytes>, None::<Bytes>))
            }
        }
    }
}

pub fn f11(v0: bool, v1: Bytes, v2: Option<Bytes>, v3: Option<Bytes>, v4: Option<Bytes>, v5: Option<Bytes>, v6: Option<Bytes>, v7: u64, v8: List<Bytes>, v9: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v10: u64 = f56(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())?;
    let a168 = v1.clone();
    let a169 = f3(Str::lit("core"));
    if equal(a168, a169) {
        f57(v0, v2, v10, v9)
    } else {
        let a170 = v1.clone();
        let a171 = f3(Str::lit("lean"));
        if equal(a170, a171) {
            f58(v3, v4, v10, v9)
        } else {
            let a172 = v1.clone();
            let a173 = f3(Str::lit("document"));
            if equal(a172, a173) {
                Ok(f59(v3, v5, v10, v9))
            } else {
                let a174 = v1.clone();
                let a175 = f3(Str::lit("defined"));
                if equal(a174, a175) {
                    f60(v6, v10, v7, v8, v9)
                } else {
                    Ok((f46(v9, Some(f2(f5(f6(v1), f3(Str::lit(" is not a denotation kind")))))), Adt3::C0(Bytes::lit(&[]))))
                }
            }
        }
    }
}

pub fn f12(v0: List<Adt21>, v1: bool, v2: List<Bytes>, v3: List<(Bytes, Bytes)>) -> R<Adt13> {
    let v6: List<(Bytes, Bytes)> = f46(v3, {
        let m176 = v0.clone();
        match m176.uncons() {
            None => {
                Some(f2(f3(Str::lit("an entry declares at least one form"))))
            }
            Some((_, _)) => {
                None::<(Bytes, Bytes)>
            }
        }
    });
    let v7: Adt24 = f61(Fn2::F62(v2, v1), Adt24::C0(v6, List::<Adt4>::nil(), List::<Bytes>::nil()), v0)?;
    Ok(Adt13::C0(match v7.clone() {
        Adt24::C0(h177, _, _) => {
            h177
        }
    }, f63(match v7 {
        Adt24::C0(_, h178, _) => {
            h178
        }
    })))
}

pub fn f13(v0: Bytes, v1: bool, v2: List<u64>, v3: List<Adt4>, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v5: List<(Bytes, Bytes)> = f46(v4, if if v1 {
        !f64(v3.clone())
    } else {
        false
    } {
        Some(f2(f5(f5(f3(Str::lit("category `")), v0.clone()), f3(Str::lit("` requires a canonical source form in the text or the math channel")))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v6: List<(Bytes, Bytes)> = f65(v0.clone(), v2.clone(), v3.clone(), 0u64, f3(Str::lit("text")), v5)?;
    f65(v0, v2, v3, 1u64, f3(Str::lit("math")), v6)
}

pub fn f14(v0: bool, v1: u64, v2: Bytes, v3: bool, v4: List<u64>, v5: Option<Bytes>, v6: Option<Bytes>, v7: List<Adt4>, v8: u64, v9: u64, v10: u64, v11: List<(Bytes, Bytes)>) -> R<Adt14> {
    let v12: List<(Bytes, Bytes)> = if v0 {
        f46(v11, {
            let a179 = v1;
            let a180 = 0u64;
            if if nat_eq(a179, a180) {
                true
            } else {
                let a181 = v1;
                let a182 = 1u64;
                nat_eq(a181, a182)
            } {
                Some(f2(f3(Str::lit("structural and grammar entries do not carry render templates"))))
            } else {
                None::<(Bytes, Bytes)>
            }
        })
    } else {
        v11
    };
    let v13: bool = if v3 {
        let a183 = 0u64;
        let a184 = f66(v7.clone(), 0u64)?;
        nat_lt(a183, a184)
    } else {
        false
    };
    let v14: List<(Bytes, Bytes)> = if v0 {
        f46(v12, if if f67(v6.clone()) {
            if !f68(v4, 0u64) {
                !v13
            } else {
                false
            }
        } else {
            false
        } {
            Some(f2(f5(f5(f3(Str::lit("category `")), v2), f3(Str::lit("` has no text canonical form, so a text render does not apply")))))
        } else {
            None::<(Bytes, Bytes)>
        })
    } else {
        v12
    };
    let v15: Adt25 = f69(f3(Str::lit("math")), 1u64, v5, v7.clone(), v8, v9, v14)?;
    let v16: Adt25 = f69(f3(Str::lit("text")), 0u64, v6, v7, v8, v9, match v15.clone() {
        Adt25::C0(h185, _) => {
            h185
        }
    })?;
    let v17: bool = {
        let a186 = v10;
        let a187 = 1u64;
        if nat_eq(a186, a187) {
            true
        } else {
            let a188 = v10;
            let a189 = 2u64;
            if nat_eq(a188, a189) {
                true
            } else {
                let a190 = v10;
                let a191 = 3u64;
                if nat_eq(a190, a191) {
                    true
                } else {
                    let a192 = v10;
                    let a193 = 4u64;
                    nat_eq(a192, a193)
                }
            }
        }
    };
    let v19: List<(Bytes, Bytes)> = f46(match v16.clone() {
        Adt25::C0(h194, _) => {
            h194
        }
    }, if if v17 {
        let m196 = match v15.clone() {
            Adt25::C0(_, h195) => {
                h195
            }
        };
        match m196 {
            None => {
                true
            }
            Some(_) => {
                false
            }
        }
    } else {
        false
    } {
        Some(f2(f3(Str::lit("a math frame with surface arguments requires a canonical math render"))))
    } else {
        None::<(Bytes, Bytes)>
    });
    Ok(Adt14::C0(v19, match v15 {
        Adt25::C0(_, h197) => {
            h197
        }
    }, match v16 {
        Adt25::C0(_, h198) => {
            h198
        }
    }))
}

pub fn f15(v0: bool, v1: Bytes, v2: u64, v3: List<(Bytes, Bytes)>) -> (List<(Bytes, Bytes)>, bool) {
    if !v0 {
        (v3, false)
    } else {
        let v4: List<(Bytes, Bytes)> = f46(v3, {
            let a199 = v1;
            let a200 = f3(Str::lit("equality"));
            if equal(a199, a200) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f3(Str::lit("language 1.0 ships exactly the equality calculation descriptor"))))
            }
        });
        let v5: List<(Bytes, Bytes)> = f46(v4, {
            let a201 = v2;
            let a202 = 15u64;
            if nat_eq(a201, a202) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f3(Str::lit("a calculation descriptor belongs to an infix relation"))))
            }
        });
        (v5, true)
    }
}

pub fn f16(v0: Adt1, v1: u64) -> R<(u64, bool)> {
    let m203 = v0;
    match m203 {
        Adt1::C0 => {
            Ok((0u64, true))
        }
        Adt1::C1(_) => {
            Ok((0u64, true))
        }
        Adt1::C2(_, _, _) => {
            Ok((0u64, false))
        }
        Adt1::C3(_) => {
            Ok((0u64, false))
        }
        Adt1::C4(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C5(r208, r209) => {
            let v2 = (*r208).clone();
            let v3 = (*r209).clone();
            Ok(({
                let (_, h204) = {
                    let (h205, _) = f70(Fn3::F71, {
                        let a206 = v1;
                        let a207 = 1u64;
                        nat_add(a206, a207)?
                    }, (v2, 0u64))?;
                    h205
                };
                h204
            }, f72(v3)))
        }
        Adt1::C6(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C7(_, _, _, _) => {
            Ok((0u64, false))
        }
        Adt1::C8(_) => {
            Ok((0u64, false))
        }
        Adt1::C9 => {
            Ok((0u64, false))
        }
        Adt1::C10(_, _) => {
            Ok((0u64, false))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((0u64, false))
        }
    }
}

pub fn f17(v0: Option<Adt22>, v1: bool, v2: Bytes, v3: List<(Bytes, Bytes)>) -> R<Adt15> {
    let m210 = v0;
    match m210 {
        None => {
            Ok(Adt15::C0(v3, None::<Adt8>))
        }
        Some(v4) => {
            let v5: List<(Bytes, Bytes)> = f46(v3, if v1 {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f6(v2), f3(Str::lit(": an eliminator descriptor belongs to a type entry, whose signature results in a sort"))), Bytes::lit(&[]))))
            });
            let v6: List<(Bytes, Bytes)> = f46(v5, if f73(match v4.clone() {
                Adt22::C0(h211, _, _) => {
                    h211
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f3(Str::lit("eliminator cases_lean_name ")), f6(match v4.clone() {
                    Adt22::C0(h212, _, _) => {
                        h212
                    }
                })), f3(Str::lit(" is not a Lean name")))))
            });
            let v7: List<(Bytes, Bytes)> = f46(v6, if f73(match v4.clone() {
                Adt22::C0(_, h213, _) => {
                    h213
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f3(Str::lit("eliminator induction_lean_name ")), f6(match v4.clone() {
                    Adt22::C0(_, h214, _) => {
                        h214
                    }
                })), f3(Str::lit(" is not a Lean name")))))
            });
            let v8: Adt26 = f74(Fn4::F75, Adt26::C0(v7, List::<Adt9>::nil(), List::<Bytes>::nil()), match v4.clone() {
                Adt22::C0(_, _, h215) => {
                    h215
                }
            })?;
            let v9: List<Adt9> = f76(match v8.clone() {
                Adt26::C0(_, h216, _) => {
                    h216
                }
            });
            let v10: List<(Bytes, Bytes)> = f46(match v8 {
                Adt26::C0(h217, _, _) => {
                    h217
                }
            }, {
                let a219 = {
                    let a218 = v9.clone();
                    length_list(a218)
                };
                let a220 = 0u64;
                if nat_eq(a219, a220) {
                    Some(f2(f3(Str::lit("an eliminator needs at least one constructor"))))
                } else {
                    None::<(Bytes, Bytes)>
                }
            });
            Ok(Adt15::C0(v10, Some(Adt8::C0(match v4.clone() {
                Adt22::C0(h221, _, _) => {
                    h221
                }
            }, match v4 {
                Adt22::C0(_, h222, _) => {
                    h222
                }
            }, v9))))
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f77(Fn5::F78, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f19(v0: Fn0, p226: Adt18, p227: List<Bytes>) -> R<Adt18> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(Adt18, List<Bytes>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Bytes>), R<Adt18>>(({
                            let c224: Fn0 = v0.clone();
                            c224.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt18, v1: Bytes) -> R<Adt18> {
    f79(v0, v1)
}

pub fn f21(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a228 = v0.clone();
        length_bytes(a228)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h229, _) = f80(Fn6::F81(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h229
    };
    let v3: u64 = {
        let (h230, _) = {
            let (_, h231) = v2.clone();
            h231
        };
        h230
    };
    let v4: List<Bytes> = {
        let (_, h232) = {
            let (_, h233) = v2;
            h233
        };
        h232
    };
    let a234 = v3;
    let a235 = v1;
    if nat_lt(a234, a235) {
        Ok(f82(List::cons(f83(v0, v3, {
            let a236 = v1;
            let a237 = v3;
            nat_sub(a236, a237)
        }), v4)))
    } else {
        Ok(f82(v4))
    }
}

pub fn f22(v0: List<Adt19>) -> List<Adt19> {
    f84(Fn7::F85, List::<Adt19>::nil(), v0)
}

pub fn f23(v0: List<Adt19>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> List<Adt19> {
    List::cons(Adt19::C0(v1, v2, f86(v3)), v0)
}

pub fn f24(v0: Fn1, p241: (bool, List<Adt16>), p242: List<Adt19>) -> R<(bool, List<Adt16>)> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v1, v2) = m240;
                let m238 = v2;
                match m238.uncons() {
                    None => {
                        Err::<((bool, List<Adt16>), List<Adt19>), R<(bool, List<Adt16>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt16>), List<Adt19>), R<(bool, List<Adt16>)>>(({
                            let c239: Fn1 = v0.clone();
                            c239.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: (bool, List<Adt16>), v1: Adt19) -> R<(bool, List<Adt16>)> {
    f87(v0, v1)
}

pub fn f26(v0: List<Adt16>) -> R<List<(Bytes, Adt17)>> {
    let m243 = f34(v0, List::<Bytes>::nil(), false)?;
    match m243 {
        None => {
            Ok(List::<(Bytes, Adt17)>::nil())
        }
        Some(v1) => {
            Ok(f35(v1))
        }
    }
}

pub fn f27(v0: List<(Bytes, Adt17)>, v1: List<Bytes>) -> R<bool> {
    f88(Fn8::F89(v1), true, f90(v0))
}

pub fn f28() -> List<Bytes> {
    List::cons(f3(Str::lit("spec")), List::cons(f3(Str::lit("id")), List::cons(f3(Str::lit("category")), List::cons(f3(Str::lit("signature")), List::cons(f3(Str::lit("universes")), List::cons(f3(Str::lit("surface_arity")), List::cons(f3(Str::lit("frame")), List::cons(f3(Str::lit("precedence")), List::cons(f3(Str::lit("associativity")), List::cons(f3(Str::lit("form")), List::<Bytes>::nil()))))))))))
}

pub fn f29(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Bytes> {
    let m244 = f91(v0, v1);
    match m244 {
        None => {
            None::<Bytes>
        }
        Some(v2) => {
            let m245 = v2;
            match m245 {
                Adt17::C0(v3) => {
                    Some(v3)
                }
                Adt17::C1(_) => {
                    None::<Bytes>
                }
                Adt17::C2(_) => {
                    None::<Bytes>
                }
                Adt17::C3(_) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f30(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m246 = f91(v0, v1);
    match m246 {
        None => {
            Some(None::<Bytes>)
        }
        Some(v2) => {
            let m247 = v2;
            match m247 {
                Adt17::C0(v3) => {
                    Some(Some(v3))
                }
                Adt17::C1(_) => {
                    None::<Option<Bytes>>
                }
                Adt17::C2(_) => {
                    None::<Option<Bytes>>
                }
                Adt17::C3(_) => {
                    None::<Option<Bytes>>
                }
            }
        }
    }
}

pub fn f31(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<List<Bytes>>> {
    let m248 = f91(v0, v1);
    match m248 {
        None => {
            Some(None::<List<Bytes>>)
        }
        Some(v2) => {
            let m249 = v2;
            match m249 {
                Adt17::C0(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C1(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C2(_) => {
                    None::<Option<List<Bytes>>>
                }
                Adt17::C3(v6) => {
                    Some(Some(v6))
                }
            }
        }
    }
}

pub fn f32(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<u64> {
    let m250 = f91(v0, v1);
    match m250 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let m251 = v2;
            match m251 {
                Adt17::C0(_) => {
                    None::<u64>
                }
                Adt17::C1(_) => {
                    None::<u64>
                }
                Adt17::C2(v5) => {
                    Some(v5)
                }
                Adt17::C3(_) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Option<u64>> {
    let m252 = f91(v0, v1);
    match m252 {
        None => {
            Some(None::<u64>)
        }
        Some(v2) => {
            let m253 = v2;
            match m253 {
                Adt17::C0(_) => {
                    None::<Option<u64>>
                }
                Adt17::C1(_) => {
                    None::<Option<u64>>
                }
                Adt17::C2(v5) => {
                    Some(Some(v5))
                }
                Adt17::C3(_) => {
                    None::<Option<u64>>
                }
            }
        }
    }
}

pub fn f34(v0: List<Adt16>, v1: List<Bytes>, v2: bool) -> R<Option<Adt16>> {
    f92(Fn9::F93(v2, v1), None::<Adt16>, v0)
}

pub fn f35(v0: Adt16) -> List<(Bytes, Adt17)> {
    let m255 = match v0 {
        Adt16::C0(_, _, h254) => {
            h254
        }
    };
    match m255.uncons() {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f36(v0: List<(Bytes, Adt17)>) -> R<Option<Adt27>> {
    if f27(v0.clone(), f94())? {
        let m256 = f29(v0.clone(), f3(Str::lit("kind")));
        match m256 {
            None => {
                Ok(None::<Adt27>)
            }
            Some(v1) => {
                let m257 = f30(v0.clone(), f3(Str::lit("constructor")));
                match m257 {
                    None => {
                        Ok(None::<Adt27>)
                    }
                    Some(v2) => {
                        let m258 = f30(v0.clone(), f3(Str::lit("module")));
                        match m258 {
                            None => {
                                Ok(None::<Adt27>)
                            }
                            Some(v3) => {
                                let m259 = f30(v0.clone(), f3(Str::lit("name")));
                                match m259 {
                                    None => {
                                        Ok(None::<Adt27>)
                                    }
                                    Some(v4) => {
                                        let m260 = f30(v0.clone(), f3(Str::lit("component")));
                                        match m260 {
                                            None => {
                                                Ok(None::<Adt27>)
                                            }
                                            Some(v5) => {
                                                let m261 = f30(v0, f3(Str::lit("value")));
                                                match m261 {
                                                    None => {
                                                        Ok(None::<Adt27>)
                                                    }
                                                    Some(v6) => {
                                                        Ok(Some(Adt27::C0(v1, v2, v3, v4, v5, v6)))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt27>)
    }
}

pub fn f37(v0: List<Adt16>, v1: List<(Bytes, Adt17)>) -> R<Option<List<List<(Bytes, Adt17)>>>> {
    let m262 = f34(v0, List::cons(f3(Str::lit("form")), List::<Bytes>::nil()), true)?;
    match m262 {
        None => {
            Ok(f95(v1, f3(Str::lit("form"))))
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt16::C0(_, _, h263) => {
                    h263
                }
            }))
        }
    }
}

pub fn f38(v0: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt21>>> {
    let m264 = f96(Fn10::F97, Some(List::<Adt21>::nil()), v0)?;
    match m264 {
        None => {
            Ok(None::<List<Adt21>>)
        }
        Some(v1) => {
            Ok(Some(f98(v1)))
        }
    }
}

pub fn f39(v0: List<Adt16>) -> R<Option<Option<Adt22>>> {
    let m265 = f34(v0.clone(), List::cons(f3(Str::lit("eliminator")), List::<Bytes>::nil()), false)?;
    match m265 {
        None => {
            Ok(Some(None::<Adt22>))
        }
        Some(v1) => {
            let v2: List<(Bytes, Adt17)> = f35(v1);
            if f27(v2.clone(), f99())? {
                let m266 = f29(v2.clone(), f3(Str::lit("cases_lean_name")));
                match m266 {
                    None => {
                        Ok(None::<Option<Adt22>>)
                    }
                    Some(v3) => {
                        let m267 = f29(v2.clone(), f3(Str::lit("induction_lean_name")));
                        match m267 {
                            None => {
                                Ok(None::<Option<Adt22>>)
                            }
                            Some(v4) => {
                                let m268 = f100(v0, v2)?;
                                match m268 {
                                    None => {
                                        Ok(None::<Option<Adt22>>)
                                    }
                                    Some(v5) => {
                                        let m269 = f101(v5)?;
                                        match m269 {
                                            None => {
                                                Ok(None::<Option<Adt22>>)
                                            }
                                            Some(v6) => {
                                                Ok(Some(Some(Adt22::C0(v3, v4, v6))))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Option<Adt22>>)
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f5(f5(v0, v1), v2)
}

pub fn f41(v0: List<Bytes>, v1: Bytes) -> R<Option<u64>> {
    let (_, h270) = f102(Fn11::F103(v1), (0u64, None::<u64>), v0)?;
    Ok(h270)
}

pub fn f42(v0: u64) -> List<Bytes> {
    let a271 = v0;
    let a272 = 0u64;
    if nat_eq(a271, a272) {
        f104()
    } else {
        let a273 = v0;
        let a274 = 1u64;
        if nat_eq(a273, a274) {
            f105()
        } else {
            let a275 = v0;
            let a276 = 2u64;
            if nat_eq(a275, a276) {
                f106()
            } else {
                let a277 = v0;
                let a278 = 3u64;
                if nat_eq(a277, a278) {
                    f107()
                } else {
                    f108()
                }
            }
        }
    }
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    let a279 = v0;
    let a280 = 16u64;
    if nat_eq(a279, a280) {
        let a281 = v1;
        let a282 = 0u64;
        if nat_eq(a281, a282) {
            Ok(0u64)
        } else {
            Ok(1u64)
        }
    } else {
        f109(f110(), v0)
    }
}

pub fn f44(v0: u64) -> Option<u64> {
    let a283 = v0;
    let a284 = 1u64;
    if nat_eq(a283, a284) {
        None::<u64>
    } else {
        let a285 = v0;
        let a286 = 0u64;
        if nat_eq(a285, a286) {
            Some(0u64)
        } else {
            let a287 = v0;
            let a288 = 4u64;
            if if nat_eq(a287, a288) {
                true
            } else {
                let a289 = v0;
                let a290 = 6u64;
                if nat_eq(a289, a290) {
                    true
                } else {
                    let a291 = v0;
                    let a292 = 9u64;
                    nat_eq(a291, a292)
                }
            } {
                Some(2u64)
            } else {
                Some(1u64)
            }
        }
    }
}

pub fn f45(v0: u64) -> R<List<u64>> {
    let v1: u64 = f109(f111(), v0)?;
    let a293 = v1;
    let a294 = 1u64;
    if nat_eq(a293, a294) {
        Ok(List::cons(0u64, List::<u64>::nil()))
    } else {
        let a295 = v1;
        let a296 = 2u64;
        if nat_eq(a295, a296) {
            Ok(List::cons(1u64, List::<u64>::nil()))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f46(v0: List<(Bytes, Bytes)>, v1: Option<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m297 = v1;
    match m297 {
        None => {
            v0
        }
        Some(v2) => {
            List::cons(v2, v0)
        }
    }
}

pub fn f47(v0: Bytes) -> R<bool> {
    f112(v0, true)
}

pub fn f48(v0: u64) -> Bytes {
    f113(f114(), 10u64, v0)
}

pub fn f49(v0: List<Bytes>) -> bool {
    let (_, h298) = f115(Fn12::F116, (None::<Bytes>, true), v0);
    h298
}

pub fn f50(v0: List<Bytes>) -> R<Option<Bytes>> {
    f117(Fn13::F118, None::<Bytes>, v0)
}

pub fn f51(v0: Bytes, v1: u64) -> R<Result<Adt1, Adt28>> {
    let m299 = f119(v0)?;
    match m299 {
        Ok(v3) => {
            let m300 = f120(v3.clone(), v1)?;
            match m300 {
                Ok(v5) => {
                    f121(v5, {
                        let a301 = v3;
                        length_list(a301)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt1, Adt28>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt1, Adt28>(v2))
        }
    }
}

pub fn f52(v0: Adt1, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    let m308 = {
        let (h303, _) = f122(Fn14::F123(v1), {
            let a306 = {
                let a304 = v2;
                let a305 = 4u64;
                nat_mul(a304, a305)?
            };
            let a307 = 4u64;
            nat_add(a306, a307)?
        }, Adt29::C0(List::cons(Adt30::C0(v0), List::<Adt30>::nil()), List::<Bytes>::nil(), 0u64, None::<Bytes>))?;
        h303
    };
    match m308 {
        Adt29::C0(_, _, _, h302) => {
            Ok(h302)
        }
    }
}

pub fn f53(v0: Adt1, v1: bool) -> R<Bytes> {
    let (h309, _) = f124(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h309)
}

pub fn f54(v0: Bytes) -> R<Bytes> {
    let a311 = {
        let a310 = Str::lit("blake3:");
        utf8_encode(a310)
    };
    let a312 = f125(v0)?;
    Ok(append_bytes(a311, a312))
}

pub fn f55(v0: Bytes, v1: Adt28) -> (Bytes, Bytes) {
    let a314 = match v1.clone() {
        Adt28::C0(h313, _) => {
            h313
        }
    };
    let a315 = 1u64;
    if nat_eq(a314, a315) {
        f126(f5(f5(v0, f3(Str::lit(": "))), match v1 {
            Adt28::C0(_, h316) => {
                h316
            }
        }))
    } else {
        f2(f5(f5(f5(f3(Str::lit("invalid ")), v0), f3(Str::lit(" LSE: "))), match v1 {
            Adt28::C0(_, h317) => {
                h317
            }
        }))
    }
}

pub fn f56(v0: Option<Bytes>, v1: Option<Bytes>, v2: Option<Bytes>, v3: Option<Bytes>, v4: Option<Bytes>) -> R<u64> {
    let a324 = {
        let a322 = {
            let a320 = {
                let a318 = f127(v0);
                let a319 = f127(v1);
                nat_add(a318, a319)?
            };
            let a321 = f127(v2);
            nat_add(a320, a321)?
        };
        let a323 = f127(v3);
        nat_add(a322, a323)?
    };
    let a325 = f127(v4);
    nat_add(a324, a325)
}

pub fn f57(v0: bool, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v4: List<(Bytes, Bytes)> = f46(v3, if v0 {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f3(Str::lit("only lexlean.core may use a core denotation"))))
    });
    let v6: bool = {
        let m326 = v1.clone();
        match m326 {
            None => {
                false
            }
            Some(v5) => {
                let a327 = v2;
                let a328 = 1u64;
                if nat_eq(a327, a328) {
                    f47(v5)?
                } else {
                    false
                }
            }
        }
    };
    if v6 {
        Ok((v4, Adt3::C0({
            let m329 = v1;
            match m329 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        })))
    } else {
        Ok((f46(v4, Some(f2(f3(Str::lit("a core denotation has exactly `constructor`"))))), Adt3::C0(Bytes::lit(&[]))))
    }
}

pub fn f58(v0: Option<Bytes>, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let v6: bool = {
        let m330 = v0.clone();
        match m330 {
            None => {
                false
            }
            Some(v4) => {
                let m331 = v1.clone();
                match m331 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        let a332 = v2;
                        let a333 = 2u64;
                        if nat_eq(a332, a333) {
                            if f73(v4)? {
                                f73(v5)?
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                }
            }
        }
    };
    if v6 {
        Ok((v3, Adt3::C1({
            let m334 = v0;
            match m334 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        }, {
            let m335 = v1;
            match m335 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v8) => {
                    v8
                }
            }
        })))
    } else {
        Ok((f46(v3, Some(f2(f3(Str::lit("a lean denotation has exactly `module` and `name` in the conservative ASCII Lean-name grammar"))))), Adt3::C1(Bytes::lit(&[]), Bytes::lit(&[]))))
    }
}

pub fn f59(v0: Option<Bytes>, v1: Option<Bytes>, v2: u64, v3: List<(Bytes, Bytes)>) -> (List<(Bytes, Bytes)>, Adt3) {
    let v6: bool = {
        let m336 = v0.clone();
        match m336 {
            None => {
                false
            }
            Some(_) => {
                let m337 = v1.clone();
                match m337 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let a338 = v2;
                        let a339 = 2u64;
                        nat_eq(a338, a339)
                    }
                }
            }
        }
    };
    if v6 {
        (v3, Adt3::C2({
            let m340 = v0;
            match m340 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v7) => {
                    v7
                }
            }
        }, {
            let m341 = v1;
            match m341 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v8) => {
                    v8
                }
            }
        }))
    } else {
        (f46(v3, Some(f2(f3(Str::lit("a document denotation has exactly `module` and `component`"))))), Adt3::C2(Bytes::lit(&[]), Bytes::lit(&[])))
    }
}

pub fn f60(v0: Option<Bytes>, v1: u64, v2: u64, v3: List<Bytes>, v4: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Adt3)> {
    let m342 = v0;
    match m342 {
        None => {
            Ok((f46(v4, Some(f2(f3(Str::lit("a defined denotation has exactly `value`"))))), Adt3::C3(Adt1::C0, Bytes::lit(&[]))))
        }
        Some(v5) => {
            let a343 = v1;
            let a344 = 1u64;
            if nat_eq(a343, a344) {
                let m345 = f51(v5.clone(), v2)?;
                match m345 {
                    Ok(v6) => {
                        let v7: u64 = {
                            let a346 = v5;
                            length_bytes(a346)
                        };
                        let v9: List<(Bytes, Bytes)> = {
                            let m347 = f52(v6.clone(), v3, v7)?;
                            match m347 {
                                None => {
                                    v4
                                }
                                Some(v8) => {
                                    f46(v4, Some(f2(f5(f3(Str::lit("invalid defined value: ")), v8))))
                                }
                            }
                        };
                        Ok((v9, Adt3::C3(v6.clone(), f53(v6, true)?)))
                    }
                    Err(v10) => {
                        Ok((f46(v4, Some(f55(f3(Str::lit("defined value")), v10))), Adt3::C3(Adt1::C0, Bytes::lit(&[]))))
                    }
                }
            } else {
                Ok((f46(v4, Some(f2(f3(Str::lit("a defined denotation has exactly `value`"))))), Adt3::C3(Adt1::C0, Bytes::lit(&[]))))
            }
        }
    }
}

pub fn f61(v0: Fn2, p351: Adt24, p352: List<Adt21>) -> R<Adt24> {
    let m350 = (p351, p352);
    {
        let mut m350 = m350;
        loop {
            let step = {
                let (v1, v2) = m350;
                let m348 = v2;
                match m348.uncons() {
                    None => {
                        Err::<(Adt24, List<Adt21>), R<Adt24>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt24, List<Adt21>), R<Adt24>>(({
                            let c349: Fn2 = v0.clone();
                            c349.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<Bytes>, v1: bool, v2: Adt24, v3: Adt21) -> R<Adt24> {
    f128(v1, v0, v2, v3)
}

pub fn f63(v0: List<Adt4>) -> List<Adt4> {
    f129(Fn15::F130, List::<Adt4>::nil(), v0)
}

pub fn f64(v0: List<Adt4>) -> bool {
    f131(Fn16::F132, false, v0)
}

pub fn f65(v0: Bytes, v1: List<u64>, v2: List<Adt4>, v3: u64, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v6: u64 = f66(v2, v3)?;
    let v7: List<(Bytes, Bytes)> = f46(v5, {
        let a353 = 1u64;
        let a354 = v6;
        if nat_lt(a353, a354) {
            Some(f2(f5(f3(Str::lit("more than one canonical source form for the ")), f5(v4.clone(), f3(Str::lit(" channel"))))))
        } else {
            None::<(Bytes, Bytes)>
        }
    });
    Ok(f46(v7, if if f68(v1, v3) {
        let a355 = v6;
        let a356 = 0u64;
        nat_eq(a355, a356)
    } else {
        false
    } {
        Some(f2(f5(f5(f3(Str::lit("category `")), v0), f5(f3(Str::lit("` requires a canonical source form in the ")), f5(v4, f3(Str::lit(" channel")))))))
    } else {
        None::<(Bytes, Bytes)>
    }))
}

pub fn f66(v0: List<Adt4>, v1: u64) -> R<u64> {
    f133(Fn17::F134(v1), 0u64, v0)
}

pub fn f67(v0: Option<Bytes>) -> bool {
    let m357 = v0;
    match m357 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f68(v0: List<u64>, v1: u64) -> bool {
    f135(Fn18::F136(v1), false, v0)
}

pub fn f69(v0: Bytes, v1: u64, v2: Option<Bytes>, v3: List<Adt4>, v4: u64, v5: u64, v6: List<(Bytes, Bytes)>) -> R<Adt25> {
    let m358 = v2;
    match m358 {
        None => {
            Ok(Adt25::C0(v6, None::<Adt7>))
        }
        Some(v7) => {
            let m359 = f137(v7.clone(), v4)?;
            match m359 {
                Ok(v9) => {
                    let v10: u64 = {
                        let a361 = {
                            let a360 = v7;
                            length_bytes(a360)
                        };
                        let a362 = 2u64;
                        nat_add(a361, a362)?
                    };
                    let v11: Adt31 = f139(v9.clone(), v10)?;
                    let v12: List<(Bytes, Bytes)> = f46(v6, {
                        let a364 = f140(f141(match v11.clone() {
                            Adt31::C0(h363, _, _, _, _, _, _) => {
                                h363
                            }
                        }));
                        let a365 = f140(f142(v5)?);
                        if equal(a364, a365) {
                            None::<(Bytes, Bytes)>
                        } else {
                            Some(f2(f5(f5(v0.clone(), f3(Str::lit(" render must use every slot 0.."))), f5(f48(v5), f3(Str::lit(" exactly once"))))))
                        }
                    });
                    let v14: List<(Bytes, Bytes)> = {
                        let m367 = match v11.clone() {
                            Adt31::C0(_, _, _, _, _, _, h366) => {
                                h366
                            }
                        };
                        match m367 {
                            None => {
                                v12
                            }
                            Some(v13) => {
                                f46(v12, Some(f2(f5(f5(v0.clone(), f3(Str::lit(" render: "))), v13))))
                            }
                        }
                    };
                    let v16: List<(Bytes, Bytes)> = {
                        let a368 = v1;
                        let a369 = 0u64;
                        if nat_eq(a368, a369) {
                            let m371 = match v11.clone() {
                                Adt31::C0(_, _, _, _, _, h370, _) => {
                                    h370
                                }
                            };
                            match m371 {
                                None => {
                                    v14
                                }
                                Some(v15) => {
                                    f46(v14, Some(f2(f5(f5(f3(Str::lit("text render uses `")), v15), f3(Str::lit("`, a mathematical construct with no text rendering"))))))
                                }
                            }
                        } else {
                            v14
                        }
                    };
                    let v17: List<(Bytes, Bytes)> = f143(match v11 {
                        Adt31::C0(_, _, _, h372, _, _, _) => {
                            h372
                        }
                    }, v3, v1, v0, v16)?;
                    Ok(Adt25::C0(v17, Some(v9)))
                }
                Err(v8) => {
                    Ok(Adt25::C0(f46(v6, Some(f138(v0, v8))), None::<Adt7>))
                }
            }
        }
    }
}

pub fn f70(v0: Fn3, p377: u64, p378: (Adt1, u64)) -> R<((Adt1, u64), bool)> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v1, v2) = m376;
                let m373 = v1;
                if m373 == 0 {
                    Err::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m373 - 1;
                    let m375 = {
                        let c374: Fn3 = v0.clone();
                        c374.apply(v2.clone())?
                    };
                    match m375 {
                        None => {
                            Err::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt1, u64)), R<((Adt1, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
    f144(v0)
}

pub fn f72(v0: Adt1) -> bool {
    let m379 = v0;
    match m379 {
        Adt1::C0 => {
            true
        }
        Adt1::C1(_) => {
            true
        }
        Adt1::C2(_, _, _) => {
            false
        }
        Adt1::C3(_) => {
            false
        }
        Adt1::C4(_, _) => {
            false
        }
        Adt1::C5(_, _) => {
            false
        }
        Adt1::C6(_, _) => {
            false
        }
        Adt1::C7(_, _, _, _) => {
            false
        }
        Adt1::C8(_) => {
            false
        }
        Adt1::C9 => {
            false
        }
        Adt1::C10(_, _) => {
            false
        }
        Adt1::C11(_, _, _, _) => {
            false
        }
    }
}

pub fn f73(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a380 = v0.clone();
        length_bytes(a380)
    };
    let v2: (u64, bool) = {
        let (h381, _) = f145(Fn19::F146(v1, v0), {
            let a382 = v1;
            let a383 = 1u64;
            nat_add(a382, a383)?
        }, (0u64, true))?;
        h381
    };
    let a385 = {
        let (h384, _) = v2;
        h384
    };
    let a386 = v1;
    if nat_eq(a385, a386) {
        Ok(!f147(v2))
    } else {
        Ok(false)
    }
}

pub fn f74(v0: Fn4, p390: Adt26, p391: List<Adt23>) -> R<Adt26> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v1, v2) = m389;
                let m387 = v2;
                match m387.uncons() {
                    None => {
                        Err::<(Adt26, List<Adt23>), R<Adt26>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt26, List<Adt23>), R<Adt26>>(({
                            let c388: Fn4 = v0.clone();
                            c388.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Adt26, v1: Adt23) -> R<Adt26> {
    f148(v0, v1)
}

pub fn f76(v0: List<Adt9>) -> List<Adt9> {
    f149(Fn20::F150, List::<Adt9>::nil(), v0)
}

pub fn f77(v0: Fn5, p395: List<(Bytes, Bytes)>, p396: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m392 = v2;
                match m392.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c393: Fn5 = v0.clone();
                            c393.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f79(v0: Adt18, v1: Bytes) -> R<Adt18> {
    if match v0.clone() {
        Adt18::C0(_, _, _, _, _, _, h397) => {
            h397
        }
    } {
        Ok(v0)
    } else {
        let m399 = match v0.clone() {
            Adt18::C0(_, _, _, _, h398, _, _) => {
                h398
            }
        };
        match m399 {
            None => {
                if f158(v1.clone())? {
                    Ok(v0)
                } else {
                    let v8: Bytes = f151(v1)?;
                    if f159(v8.clone(), f152(Str::lit("["))) {
                        let v9: List<Adt19> = f23(match v0.clone() {
                            Adt18::C0(h426, _, _, _, _, _, _) => {
                                h426
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h427, _, _, _, _, _) => {
                                h427
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h428, _, _, _, _) => {
                                h428
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h429, _, _, _) => {
                                h429
                            }
                        });
                        let m430 = f160(v8)?;
                        match m430 {
                            None => {
                                Ok(Adt18::C0(v9, List::<Bytes>::nil(), false, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v10) => {
                                Ok(Adt18::C0(v9, {
                                    let (h431, _) = v10;
                                    h431
                                }, {
                                    let (_, h432) = v10;
                                    h432
                                }, List::<(Bytes, Adt17)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false))
                            }
                        }
                    } else {
                        let m433 = f161(v8.clone(), f152(Str::lit("=")))?;
                        match m433 {
                            None => {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h434, _, _, _, _, _, _) => {
                                        h434
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h435, _, _, _, _, _) => {
                                        h435
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h436, _, _, _, _) => {
                                        h436
                                    }
                                }, match v0 {
                                    Adt18::C0(_, _, _, h437, _, _, _) => {
                                        h437
                                    }
                                }, None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v11) => {
                                let v12: Bytes = f151(f83(v8.clone(), 0u64, v11))?;
                                let v13: Bytes = f151(f83(v8.clone(), {
                                    let a438 = v11;
                                    let a439 = 1u64;
                                    nat_add(a438, a439)?
                                }, {
                                    let a443 = {
                                        let a440 = v8;
                                        length_bytes(a440)
                                    };
                                    let a444 = {
                                        let a441 = v11;
                                        let a442 = 1u64;
                                        nat_add(a441, a442)?
                                    };
                                    nat_sub(a443, a444)
                                }))?;
                                let a446 = {
                                    let a445 = v12.clone();
                                    length_bytes(a445)
                                };
                                let a447 = 0u64;
                                if if nat_eq(a446, a447) {
                                    true
                                } else {
                                    let m448 = f161(v12.clone(), f152(Str::lit(".")))?;
                                    match m448 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                } {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h449, _, _, _, _, _, _) => {
                                            h449
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h450, _, _, _, _, _) => {
                                            h450
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h451, _, _, _, _) => {
                                            h451
                                        }
                                    }, match v0 {
                                        Adt18::C0(_, _, _, h452, _, _, _) => {
                                            h452
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                } else {
                                    if f162(match v0.clone() {
                                        Adt18::C0(_, _, _, h453, _, _, _) => {
                                            h453
                                        }
                                    }, v12.clone()) {
                                        Ok(Adt18::C0(match v0.clone() {
                                            Adt18::C0(h454, _, _, _, _, _, _) => {
                                                h454
                                            }
                                        }, match v0.clone() {
                                            Adt18::C0(_, h455, _, _, _, _, _) => {
                                                h455
                                            }
                                        }, match v0.clone() {
                                            Adt18::C0(_, _, h456, _, _, _, _) => {
                                                h456
                                            }
                                        }, match v0 {
                                            Adt18::C0(_, _, _, h457, _, _, _) => {
                                                h457
                                            }
                                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                                    } else {
                                        f163(v0, v12, v13)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(v2) => {
                let v3: Bytes = f151(v1)?;
                let a400 = v3.clone();
                let a401 = f152(Str::lit("]"));
                if equal(a400, a401) {
                    Ok(Adt18::C0(match v0.clone() {
                        Adt18::C0(h402, _, _, _, _, _, _) => {
                            h402
                        }
                    }, match v0.clone() {
                        Adt18::C0(_, h403, _, _, _, _, _) => {
                            h403
                        }
                    }, match v0.clone() {
                        Adt18::C0(_, _, h404, _, _, _, _) => {
                            h404
                        }
                    }, List::cons((v2, Adt17::C3(f153(match v0.clone() {
                        Adt18::C0(_, _, _, _, _, h405, _) => {
                            h405
                        }
                    }))), match v0 {
                        Adt18::C0(_, _, _, h406, _, _, _) => {
                            h406
                        }
                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                } else {
                    let v4: bool = f154(v3.clone(), f152(Str::lit("]")));
                    let v5: Bytes = if v4 {
                        f83(v3.clone(), 0u64, {
                            let a408 = {
                                let a407 = v3;
                                length_bytes(a407)
                            };
                            let a409 = 1u64;
                            nat_sub(a408, a409)
                        })
                    } else {
                        v3
                    };
                    let m410 = f155(v5)?;
                    match m410 {
                        None => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h420, _, _, _, _, _, _) => {
                                    h420
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h421, _, _, _, _, _) => {
                                    h421
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h422, _, _, _, _) => {
                                    h422
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, _, h423, _, _, _) => {
                                    h423
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, _, _, h424, _, _) => {
                                    h424
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, _, _, h425, _) => {
                                    h425
                                }
                            }, true))
                        }
                        Some(v6) => {
                            let v7: List<Bytes> = f156(Fn21::F157, match v0.clone() {
                                Adt18::C0(_, _, _, _, _, h411, _) => {
                                    h411
                                }
                            }, v6);
                            if v4 {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h412, _, _, _, _, _, _) => {
                                        h412
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h413, _, _, _, _, _) => {
                                        h413
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h414, _, _, _, _) => {
                                        h414
                                    }
                                }, List::cons((v2, Adt17::C3(f153(v7))), match v0 {
                                    Adt18::C0(_, _, _, h415, _, _, _) => {
                                        h415
                                    }
                                }), None::<Bytes>, List::<Bytes>::nil(), false))
                            } else {
                                Ok(Adt18::C0(match v0.clone() {
                                    Adt18::C0(h416, _, _, _, _, _, _) => {
                                        h416
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, h417, _, _, _, _, _) => {
                                        h417
                                    }
                                }, match v0.clone() {
                                    Adt18::C0(_, _, h418, _, _, _, _) => {
                                        h418
                                    }
                                }, match v0 {
                                    Adt18::C0(_, _, _, h419, _, _, _) => {
                                        h419
                                    }
                                }, Some(v2), v7, false))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f80(v0: Fn6, p462: u64, p463: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m461 = (p462, p463);
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v1, v2) = m461;
                let m458 = v1;
                if m458 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m458 - 1;
                    let m460 = {
                        let c459: Fn6 = v0.clone();
                        c459.apply(v2.clone())?
                    };
                    match m460 {
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
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f164(v0, v1)
}

pub fn f82(v0: List<Bytes>) -> List<Bytes> {
    f156(Fn21::F165, List::<Bytes>::nil(), v0)
}

pub fn f83(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m467 = {
        let a464 = v0;
        let a465 = v1;
        let a466 = v2;
        slice_bytes(a464, a465, a466)
    };
    match m467 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f84(v0: Fn7, p471: List<Adt19>, p472: List<Adt19>) -> List<Adt19> {
    let m470 = (p471, p472);
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v1, v2) = m470;
                let m468 = v2;
                match m468.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>(({
                            let c469: Fn7 = v0.clone();
                            c469.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt19>, v1: Adt19) -> List<Adt19> {
    List::cons(v1, v0)
}

pub fn f86(v0: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    f166(Fn22::F167, List::<(Bytes, Adt17)>::nil(), v0)
}

pub fn f87(v0: (bool, List<Adt16>), v1: Adt19) -> R<(bool, List<Adt16>)> {
    let (h473, _) = v0.clone();
    if h473 {
        Ok(v0)
    } else {
        let v2: List<Bytes> = match v1.clone() {
            Adt19::C0(h474, _, _) => {
                h474
            }
        };
        let v3: bool = match v1.clone() {
            Adt19::C0(_, h475, _) => {
                h475
            }
        };
        let v4: List<(Bytes, Adt17)> = match v1 {
            Adt19::C0(_, _, h476) => {
                h476
            }
        };
        let v5: bool = f168({
            let (_, h477) = v0.clone();
            h477
        }, v2.clone(), v3)?;
        if v5 {
            if v3 {
                Ok((false, f169({
                    let (_, h478) = v0;
                    h478
                }, v2, v3, v4)?))
            } else {
                Ok((true, {
                    let (_, h479) = v0;
                    h479
                }))
            }
        } else {
            Ok((false, f170({
                let (_, h480) = v0;
                h480
            }, v2, v3, v4)?))
        }
    }
}

pub fn f88(v0: Fn8, p484: bool, p485: List<Bytes>) -> R<bool> {
    let m483 = (p484, p485);
    {
        let mut m483 = m483;
        loop {
            let step = {
                let (v1, v2) = m483;
                let m481 = v2;
                match m481.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c482: Fn8 = v0.clone();
                            c482.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Bytes>, v1: bool, v2: Bytes) -> R<bool> {
    if v1 {
        f171(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f90(v0: List<(Bytes, Adt17)>) -> List<Bytes> {
    f153(f172(Fn23::F173, List::<Bytes>::nil(), v0))
}

pub fn f91(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<Adt17> {
    f174(Fn24::F175(v1), None::<Adt17>, v0)
}

pub fn f92(v0: Fn9, p489: Option<Adt16>, p490: List<Adt16>) -> R<Option<Adt16>> {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v1, v2) = m488;
                let m486 = v2;
                match m486.uncons() {
                    None => {
                        Err::<(Option<Adt16>, List<Adt16>), R<Option<Adt16>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt16>, List<Adt16>), R<Option<Adt16>>>(({
                            let c487: Fn9 = v0.clone();
                            c487.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: bool, v1: List<Bytes>, v2: Option<Adt16>, v3: Adt16) -> R<Option<Adt16>> {
    let m491 = v2.clone();
    match m491 {
        None => {
            if f176(v3.clone(), v1, v0)? {
                Ok(Some(v3))
            } else {
                Ok(None::<Adt16>)
            }
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f94() -> List<Bytes> {
    List::cons(f3(Str::lit("kind")), List::cons(f3(Str::lit("constructor")), List::cons(f3(Str::lit("module")), List::cons(f3(Str::lit("name")), List::cons(f3(Str::lit("component")), List::cons(f3(Str::lit("value")), List::<Bytes>::nil()))))))
}

pub fn f95(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<List<List<(Bytes, Adt17)>>> {
    let m492 = f177(v0, v1);
    match m492 {
        None => {
            None::<List<List<(Bytes, Adt17)>>>
        }
        Some(v2) => {
            let m493 = v2;
            match m493.uncons() {
                None => {
                    Some(List::<List<(Bytes, Adt17)>>::nil())
                }
                Some((_, _)) => {
                    None::<List<List<(Bytes, Adt17)>>>
                }
            }
        }
    }
}

pub fn f96(v0: Fn10, p497: Option<List<Adt21>>, p498: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt21>>> {
    let m496 = (p497, p498);
    {
        let mut m496 = m496;
        loop {
            let step = {
                let (v1, v2) = m496;
                let m494 = v2;
                match m494.uncons() {
                    None => {
                        Err::<(Option<List<Adt21>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt21>>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt21>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt21>>>>(({
                            let c495: Fn10 = v0.clone();
                            c495.apply(v1, v3)?
                        }, v4))
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

pub fn f97(v0: Option<List<Adt21>>, v1: List<(Bytes, Adt17)>) -> R<Option<List<Adt21>>> {
    let m499 = v0;
    match m499 {
        None => {
            Ok(None::<List<Adt21>>)
        }
        Some(v2) => {
            let m500 = f178(v1)?;
            match m500 {
                None => {
                    Ok(None::<List<Adt21>>)
                }
                Some(v3) => {
                    Ok(Some(List::cons(v3, v2)))
                }
            }
        }
    }
}

pub fn f98(v0: List<Adt21>) -> List<Adt21> {
    f179(Fn25::F180, List::<Adt21>::nil(), v0)
}

pub fn f99() -> List<Bytes> {
    List::cons(f3(Str::lit("cases_lean_name")), List::cons(f3(Str::lit("induction_lean_name")), List::cons(f3(Str::lit("constructor")), List::<Bytes>::nil())))
}

pub fn f100(v0: List<Adt16>, v1: List<(Bytes, Adt17)>) -> R<Option<List<List<(Bytes, Adt17)>>>> {
    let m501 = f34(v0, List::cons(f3(Str::lit("eliminator")), List::cons(f3(Str::lit("constructor")), List::<Bytes>::nil())), true)?;
    match m501 {
        None => {
            Ok(f95(v1, f3(Str::lit("constructor"))))
        }
        Some(v2) => {
            Ok(Some(match v2 {
                Adt16::C0(_, _, h502) => {
                    h502
                }
            }))
        }
    }
}

pub fn f101(v0: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt23>>> {
    let m503 = f181(Fn26::F182, Some(List::<Adt23>::nil()), v0)?;
    match m503 {
        None => {
            Ok(None::<List<Adt23>>)
        }
        Some(v1) => {
            Ok(Some(f183(v1)))
        }
    }
}

pub fn f102(v0: Fn11, p507: (u64, Option<u64>), p508: List<Bytes>) -> R<(u64, Option<u64>)> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m504 = v2;
                match m504.uncons() {
                    None => {
                        Err::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(({
                            let c505: Fn11 = v0.clone();
                            c505.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: (u64, Option<u64>), v2: Bytes) -> R<(u64, Option<u64>)> {
    let m510 = {
        let (_, h509) = v1;
        h509
    };
    match m510 {
        None => {
            let a511 = v2;
            let a512 = v0;
            if equal(a511, a512) {
                Ok(({
                    let (h513, _) = v1;
                    h513
                }, Some({
                    let (h514, _) = v1;
                    h514
                })))
            } else {
                Ok(({
                    let a516 = {
                        let (h515, _) = v1;
                        h515
                    };
                    let a517 = 1u64;
                    nat_add(a516, a517)?
                }, None::<u64>))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f104() -> List<Bytes> {
    List::cons(f184(Str::lit("structural")), List::cons(f184(Str::lit("grammar")), List::cons(f184(Str::lit("label-word")), List::cons(f184(Str::lit("type-noun")), List::cons(f184(Str::lit("term-constant")), List::cons(f184(Str::lit("function")), List::cons(f184(Str::lit("prefix-function")), List::cons(f184(Str::lit("postfix-function")), List::cons(f184(Str::lit("infix-function")), List::cons(f184(Str::lit("noun-function")), List::cons(f184(Str::lit("binary-noun-function")), List::cons(f184(Str::lit("predicate-constant")), List::cons(f184(Str::lit("adjective-predicate")), List::cons(f184(Str::lit("intransitive-predicate")), List::cons(f184(Str::lit("transitive-predicate")), List::cons(f184(Str::lit("infix-predicate")), List::cons(f184(Str::lit("proof-constant")), List::<Bytes>::nil())))))))))))))))))
}

pub fn f105() -> List<Bytes> {
    List::cons(f184(Str::lit("atom")), List::cons(f184(Str::lit("call")), List::cons(f184(Str::lit("prefix")), List::cons(f184(Str::lit("postfix")), List::cons(f184(Str::lit("infix")), List::cons(f184(Str::lit("noun-of")), List::cons(f184(Str::lit("binary-noun-of")), List::cons(f184(Str::lit("adjective")), List::cons(f184(Str::lit("intransitive")), List::cons(f184(Str::lit("transitive")), List::<Bytes>::nil()))))))))))
}

pub fn f106() -> List<Bytes> {
    List::cons(f184(Str::lit("text")), List::cons(f184(Str::lit("math")), List::cons(f184(Str::lit("both")), List::<Bytes>::nil())))
}

pub fn f107() -> List<Bytes> {
    List::cons(f184(Str::lit("left")), List::cons(f184(Str::lit("right")), List::cons(f184(Str::lit("none")), List::<Bytes>::nil())))
}

pub fn f108() -> List<Bytes> {
    List::cons(f184(Str::lit("singular")), List::cons(f184(Str::lit("plural")), List::cons(f184(Str::lit("sentence-case")), List::cons(f184(Str::lit("lower-case")), List::cons(f184(Str::lit("article-a")), List::cons(f184(Str::lit("article-an")), List::<Bytes>::nil()))))))
}

pub fn f109(v0: List<u64>, v1: u64) -> R<u64> {
    let (_, h518) = f185(Fn27::F186(v1), (0u64, 0u64), v0)?;
    Ok(h518)
}

pub fn f110() -> List<u64> {
    List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(0u64, List::cons(1u64, List::cons(2u64, List::cons(3u64, List::cons(4u64, List::cons(5u64, List::cons(6u64, List::cons(0u64, List::cons(7u64, List::cons(8u64, List::cons(9u64, List::cons(4u64, List::cons(0u64, List::<u64>::nil())))))))))))))))))
}

pub fn f111() -> List<u64> {
    List::cons(0u64, List::cons(0u64, List::cons(1u64, List::cons(0u64, List::cons(0u64, List::cons(2u64, List::cons(2u64, List::cons(2u64, List::cons(2u64, List::cons(1u64, List::cons(1u64, List::cons(2u64, List::cons(1u64, List::cons(1u64, List::cons(1u64, List::cons(2u64, List::cons(2u64, List::<u64>::nil())))))))))))))))))
}

pub fn f112(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a519 = v0.clone();
        length_bytes(a519)
    };
    let v3: Adt32 = {
        let (h520, _) = f187(Fn28::F188(v1, v2, v0), {
            let a521 = v2;
            let a522 = 1u64;
            nat_add(a521, a522)?
        }, Adt32::C0(0u64, true, true, true))?;
        h520
    };
    if match v3.clone() {
        Adt32::C0(_, _, _, h523) => {
            h523
        }
    } {
        Ok(!match v3 {
            Adt32::C0(_, h524, _, _) => {
                h524
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f113(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a525 = v2;
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        f189(v0, 0u64)
    } else {
        let (_, h527) = {
            let (h528, _) = f190(Fn29::F191(v1, v0), v2, (v2, Bytes::lit(&[])));
            h528
        };
        h527
    }
}

pub fn f114() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f115(v0: Fn12, p532: (Option<Bytes>, bool), p533: List<Bytes>) -> (Option<Bytes>, bool) {
    let m531 = (p532, p533);
    {
        let mut m531 = m531;
        loop {
            let step = {
                let (v1, v2) = m531;
                let m529 = v2;
                match m529.uncons() {
                    None => {
                        Err::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, bool), List<Bytes>), (Option<Bytes>, bool)>(({
                            let c530: Fn12 = v0.clone();
                            c530.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    f192(v0, v1)
}

pub fn f117(v0: Fn13, p537: Option<Bytes>, p538: List<Bytes>) -> R<Option<Bytes>> {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v1, v2) = m536;
                let m534 = v2;
                match m534.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c535: Fn13 = v0.clone();
                            c535.apply(v1, v3)?
                        }, v4))
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

pub fn f118(v0: Option<Bytes>, v1: Bytes) -> R<Option<Bytes>> {
    let m539 = v0.clone();
    match m539 {
        None => {
            if f193(v1.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f119(v0: Bytes) -> R<Result<List<Adt33>, Adt28>> {
    let v1: u64 = {
        let a540 = v0.clone();
        length_bytes(a540)
    };
    let v2: Adt34 = {
        let (h541, _) = f194(Fn30::F195(v1, v0.clone()), {
            let a542 = v1;
            let a543 = 1u64;
            nat_add(a542, a543)?
        }, Adt34::C0(0u64, 0u64, false, List::<Adt33>::nil(), None::<Adt28>))?;
        h541
    };
    let m545 = match v2.clone() {
        Adt34::C0(_, _, _, _, h544) => {
            h544
        }
    };
    match m545 {
        None => {
            Ok(Ok::<List<Adt33>, Adt28>(f196(f197(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt33>, Adt28>(v3))
        }
    }
}

pub fn f120(v0: List<Adt33>, v1: u64) -> R<Result<Adt35, Adt28>> {
    let v2: Adt36 = {
        let (h546, _) = f198(Fn31::F199(v1, v0.clone()), {
            let a548 = {
                let a547 = v0;
                length_list(a547)
            };
            let a549 = 2u64;
            nat_add(a548, a549)?
        }, Adt36::C0(0u64, List::<List<Adt35>>::nil(), None::<Adt35>, None::<Adt28>, false))?;
        h546
    };
    let m551 = match v2.clone() {
        Adt36::C0(_, _, _, h550, _) => {
            h550
        }
    };
    match m551 {
        None => {
            let m553 = match v2 {
                Adt36::C0(_, _, h552, _, _) => {
                    h552
                }
            };
            match m553 {
                None => {
                    Ok(Err::<Adt35, Adt28>(f200(f201(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt35, Adt28>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt35, Adt28>(v3))
        }
    }
}

pub fn f121(v0: Adt35, v1: u64) -> R<Result<Adt1, Adt28>> {
    let v2: Adt37 = {
        let (h554, _) = f202(Fn32::F203(v1), {
            let a557 = {
                let a555 = v1;
                let a556 = 2u64;
                nat_mul(a555, a556)?
            };
            let a558 = 2u64;
            nat_add(a557, a558)?
        }, Adt37::C0(List::cons(Adt38::C0(v0), List::<Adt38>::nil()), List::<Adt1>::nil(), None::<Adt28>))?;
        h554
    };
    let m560 = match v2.clone() {
        Adt37::C0(_, _, h559) => {
            h559
        }
    };
    match m560 {
        None => {
            let m562 = match v2 {
                Adt37::C0(_, h561, _) => {
                    h561
                }
            };
            match m562.uncons() {
                None => {
                    Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("malformed expression")))))
                }
                Some((v4, v5)) => {
                    let m563 = v5;
                    match m563.uncons() {
                        None => {
                            Ok(Ok::<Adt1, Adt28>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("malformed expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt1, Adt28>(v3))
        }
    }
}

pub fn f122(v0: Fn14, p568: u64, p569: Adt29) -> R<(Adt29, bool)> {
    let m567 = (p568, p569);
    {
        let mut m567 = m567;
        loop {
            let step = {
                let (v1, v2) = m567;
                let m564 = v1;
                if m564 == 0 {
                    Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m564 - 1;
                    let m566 = {
                        let c565: Fn14 = v0.clone();
                        c565.apply(v2.clone())?
                    };
                    match m566 {
                        None => {
                            Err::<(u64, Adt29), R<(Adt29, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt29), R<(Adt29, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: List<Bytes>, v1: Adt29) -> R<Option<Adt29>> {
    f204(v0, v1)
}

pub fn f124(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m570 = v0;
    match m570 {
        Adt1::C0 => {
            Ok((f201(Str::lit("(sort prop)")), v3))
        }
        Adt1::C1(v4) => {
            Ok((f205(f205(f201(Str::lit("(sort (type ")), f206(v4)), f201(Str::lit("))"))), v3))
        }
        Adt1::C2(v5, v6, v7) => {
            Ok((f207(v5, v6, v7), v3))
        }
        Adt1::C3(v8) => {
            Ok((f205(f205(f201(Str::lit("(local ")), if v1 {
                f208(v2, v8)
            } else {
                v8
            }), f201(Str::lit(")"))), v3))
        }
        Adt1::C4(r598, r599) => {
            let v9 = (*r598).clone();
            let v10 = (*r599).clone();
            let v11: (Bytes, u64) = f124(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f209(v10, v1, v2, {
                let (_, h571) = v11;
                h571
            })?;
            Ok((f205(f205(f205(f201(Str::lit("(app ")), {
                let (h572, _) = v11;
                h572
            }), {
                let (h573, _) = v12;
                h573
            }), f201(Str::lit(")"))), {
                let (_, h574) = v12;
                h574
            }))
        }
        Adt1::C5(r600, r601) => {
            let v13 = (*r600).clone();
            let v14 = (*r601).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f124(v14, v1, {
                let (_, h575) = {
                    let (h576, _) = v15.clone();
                    h576
                };
                h575
            }, {
                let (_, h577) = v15;
                h577
            })?;
            Ok((f205(f205(f205(f205(f201(Str::lit("(pi (")), {
                let (h578, _) = {
                    let (h579, _) = v15;
                    h579
                };
                h578
            }), f201(Str::lit(") "))), {
                let (h580, _) = v16;
                h580
            }), f201(Str::lit(")"))), {
                let (_, h581) = v16;
                h581
            }))
        }
        Adt1::C6(r602, r603) => {
            let v17 = (*r602).clone();
            let v18 = (*r603).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f124(v18, v1, {
                let (_, h582) = {
                    let (h583, _) = v19.clone();
                    h583
                };
                h582
            }, {
                let (_, h584) = v19;
                h584
            })?;
            Ok((f205(f205(f205(f205(f201(Str::lit("(lam (")), {
                let (h585, _) = {
                    let (h586, _) = v19;
                    h586
                };
                h585
            }), f201(Str::lit(") "))), {
                let (h587, _) = v20;
                h587
            }), f201(Str::lit(")"))), {
                let (_, h588) = v20;
                h588
            }))
        }
        Adt1::C7(v21, r604, r605, r606) => {
            let v22 = (*r604).clone();
            let v23 = (*r605).clone();
            let v24 = (*r606).clone();
            let v25: (Bytes, u64) = f211(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h589, _) = v25.clone();
                h589
            }), v2);
            let v27: (Bytes, u64) = f124(v22, v1, v26.clone(), {
                let (_, h590) = v25;
                h590
            })?;
            let v28: (Bytes, u64) = f124(v23, v1, v26.clone(), {
                let (_, h591) = v27;
                h591
            })?;
            let v29: (Bytes, u64) = f124(v24, v1, v26, {
                let (_, h592) = v28;
                h592
            })?;
            Ok((f205(f205(f205(f205(f205(f205(f205(f201(Str::lit("(let ")), {
                let (h593, _) = v25;
                h593
            }), f201(Str::lit(" "))), {
                let (h594, _) = v27;
                h594
            }), f201(Str::lit(" "))), {
                let (h595, _) = v28;
                h595
            }), f201(Str::lit(" "))), f205({
                let (h596, _) = v29;
                h596
            }, f201(Str::lit(")")))), {
                let (_, h597) = v29;
                h597
            }))
        }
        Adt1::C8(v30) => {
            Ok((f205(f205(f201(Str::lit("(nat ")), v30), f201(Str::lit(")"))), v3))
        }
        Adt1::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C10(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f125(v0: Bytes) -> R<Bytes> {
    let v1: Adt39 = f212(v0)?;
    let a627 = f213(match v1.clone() {
        Adt39::C0(h607, _, _, _, _, _, _, _) => {
            h607
        }
    });
    let a628 = {
        let a625 = f213(match v1.clone() {
            Adt39::C0(_, h608, _, _, _, _, _, _) => {
                h608
            }
        });
        let a626 = {
            let a623 = f213(match v1.clone() {
                Adt39::C0(_, _, h609, _, _, _, _, _) => {
                    h609
                }
            });
            let a624 = {
                let a621 = f213(match v1.clone() {
                    Adt39::C0(_, _, _, h610, _, _, _, _) => {
                        h610
                    }
                });
                let a622 = {
                    let a619 = f213(match v1.clone() {
                        Adt39::C0(_, _, _, _, h611, _, _, _) => {
                            h611
                        }
                    });
                    let a620 = {
                        let a617 = f213(match v1.clone() {
                            Adt39::C0(_, _, _, _, _, h612, _, _) => {
                                h612
                            }
                        });
                        let a618 = {
                            let a615 = f213(match v1.clone() {
                                Adt39::C0(_, _, _, _, _, _, h613, _) => {
                                    h613
                                }
                            });
                            let a616 = f213(match v1 {
                                Adt39::C0(_, _, _, _, _, _, _, h614) => {
                                    h614
                                }
                            });
                            append_bytes(a615, a616)
                        };
                        append_bytes(a617, a618)
                    };
                    append_bytes(a619, a620)
                };
                append_bytes(a621, a622)
            };
            append_bytes(a623, a624)
        };
        append_bytes(a625, a626)
    };
    Ok(append_bytes(a627, a628))
}

pub fn f126(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLS8002")), v0)
}

pub fn f127(v0: Option<Bytes>) -> u64 {
    let m629 = v0;
    match m629 {
        None => {
            0u64
        }
        Some(_) => {
            1u64
        }
    }
}

pub fn f128(v0: bool, v1: List<Bytes>, v2: Adt24, v3: Adt21) -> R<Adt24> {
    let v4: bool = f214(match v2.clone() {
        Adt24::C0(_, _, h630) => {
            h630
        }
    }, match v3.clone() {
        Adt21::C0(h631, _, _, _, _) => {
            h631
        }
    })?;
    let v5: (List<(Bytes, Bytes)>, Option<Adt4>) = f215(v3.clone(), v4, v0, v1, match v2.clone() {
        Adt24::C0(h632, _, _) => {
            h632
        }
    })?;
    Ok(Adt24::C0({
        let (h633, _) = v5;
        h633
    }, {
        let m635 = {
            let (_, h634) = v5;
            h634
        };
        match m635 {
            None => {
                match v2.clone() {
                    Adt24::C0(_, h637, _) => {
                        h637
                    }
                }
            }
            Some(v6) => {
                List::cons(v6, match v2.clone() {
                    Adt24::C0(_, h636, _) => {
                        h636
                    }
                })
            }
        }
    }, List::cons(match v3 {
        Adt21::C0(h638, _, _, _, _) => {
            h638
        }
    }, match v2 {
        Adt24::C0(_, _, h639) => {
            h639
        }
    })))
}

pub fn f129(v0: Fn15, p643: List<Adt4>, p644: List<Adt4>) -> List<Adt4> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m640 = v2;
                match m640.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c641: Fn15 = v0.clone();
                            c641.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f131(v0: Fn16, p648: bool, p649: List<Adt4>) -> bool {
    let m647 = (p648, p649);
    {
        let mut m647 = m647;
        loop {
            let step = {
                let (v1, v2) = m647;
                let m645 = v2;
                match m645.uncons() {
                    None => {
                        Err::<(bool, List<Adt4>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt4>), bool>(({
                            let c646: Fn16 = v0.clone();
                            c646.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: bool, v1: Adt4) -> bool {
    if v0 {
        true
    } else {
        match v1 {
            Adt4::C0(_, _, _, _, h650, _) => {
                h650
            }
        }
    }
}

pub fn f133(v0: Fn17, p654: u64, p655: List<Adt4>) -> R<u64> {
    let m653 = (p654, p655);
    {
        let mut m653 = m653;
        loop {
            let step = {
                let (v1, v2) = m653;
                let m651 = v2;
                match m651.uncons() {
                    None => {
                        Err::<(u64, List<Adt4>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt4>), R<u64>>(({
                            let c652: Fn17 = v0.clone();
                            c652.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: u64, v1: u64, v2: Adt4) -> R<u64> {
    if if match v2.clone() {
        Adt4::C0(_, _, _, _, h656, _) => {
            h656
        }
    } {
        f216(match v2 {
            Adt4::C0(_, h657, _, _, _, _) => {
                h657
            }
        }, v0)
    } else {
        false
    } {
        let a658 = v1;
        let a659 = 1u64;
        nat_add(a658, a659)
    } else {
        Ok(v1)
    }
}

pub fn f135(v0: Fn18, p663: bool, p664: List<u64>) -> bool {
    let m662 = (p663, p664);
    {
        let mut m662 = m662;
        loop {
            let step = {
                let (v1, v2) = m662;
                let m660 = v2;
                match m660.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c661: Fn18 = v0.clone();
                            c661.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a665 = v2;
        let a666 = v0;
        nat_eq(a665, a666)
    }
}

pub fn f137(v0: Bytes, v1: u64) -> R<Result<Adt7, Adt28>> {
    let m667 = f119(v0)?;
    match m667 {
        Ok(v3) => {
            let m668 = f120(v3.clone(), v1)?;
            match m668 {
                Ok(v5) => {
                    f217(v5, {
                        let a669 = v3;
                        length_list(a669)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt7, Adt28>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt7, Adt28>(v2))
        }
    }
}

pub fn f138(v0: Bytes, v1: Adt28) -> (Bytes, Bytes) {
    let a671 = match v1.clone() {
        Adt28::C0(h670, _) => {
            h670
        }
    };
    let a672 = 1u64;
    if nat_eq(a671, a672) {
        f126(f5(f5(v0, f3(Str::lit(" render: "))), match v1 {
            Adt28::C0(_, h673) => {
                h673
            }
        }))
    } else {
        f2(f5(f5(f5(f3(Str::lit("invalid ")), v0), f3(Str::lit(" LRE: "))), match v1 {
            Adt28::C0(_, h674) => {
                h674
            }
        }))
    }
}

pub fn f139(v0: Adt7, v1: u64) -> R<Adt31> {
    let v2: Adt40 = {
        let (h675, _) = f218(Fn33::F219(v1), {
            let a676 = v1;
            let a677 = 1u64;
            nat_add(a676, a677)?
        }, Adt40::C0(List::cons(v0, List::<Adt7>::nil()), Adt31::C0(List::<u64>::nil(), List::<Bytes>::nil(), List::<(Bytes, (Bytes, Bytes))>::nil(), List::<Bytes>::nil(), List::<Bytes>::nil(), None::<Bytes>, None::<Bytes>)))?;
        h675
    };
    let v3: Adt31 = match v2 {
        Adt40::C0(_, h678) => {
            h678
        }
    };
    Ok(Adt31::C0(f220(match v3.clone() {
        Adt31::C0(h679, _, _, _, _, _, _) => {
            h679
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, h680, _, _, _, _, _) => {
            h680
        }
    }), f221(match v3.clone() {
        Adt31::C0(_, _, h681, _, _, _, _) => {
            h681
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, _, _, h682, _, _, _) => {
            h682
        }
    }), f153(match v3.clone() {
        Adt31::C0(_, _, _, _, h683, _, _) => {
            h683
        }
    }), match v3.clone() {
        Adt31::C0(_, _, _, _, _, h684, _) => {
            h684
        }
    }, match v3 {
        Adt31::C0(_, _, _, _, _, _, h685) => {
            h685
        }
    }))
}

pub fn f140(v0: List<u64>) -> Bytes {
    f222(Fn34::F223, Bytes::lit(&[]), v0)
}

pub fn f141(v0: List<u64>) -> List<u64> {
    f224(Fn35::F225, List::<u64>::nil(), v0)
}

pub fn f142(v0: u64) -> R<List<u64>> {
    Ok(f220({
        let (_, h686) = {
            let (h687, _) = f226(Fn36::F227(v0), {
                let a688 = v0;
                let a689 = 1u64;
                nat_add(a688, a689)?
            }, (0u64, List::<u64>::nil()))?;
            h687
        };
        h686
    }))
}

pub fn f143(v0: List<Bytes>, v1: List<Adt4>, v2: u64, v3: Bytes, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f228(Fn37::F229(v3, v1, v2), v4, v0)
}

pub fn f144(v0: (Adt1, u64)) -> R<Option<(Adt1, u64)>> {
    let m691 = {
        let (h690, _) = v0;
        h690
    };
    match m691 {
        Adt1::C0 => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C1(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C2(_, _, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C3(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C4(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C5(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C6(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C7(_, _, _, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C8(_) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C9 => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C10(_, _) => {
            Ok(None::<(Adt1, u64)>)
        }
        Adt1::C11(v1, _, _, r698) => {
            let v4 = (*r698).clone();
            Ok(Some((v4, {
                let a692 = v1;
                let a693 = 0u64;
                if nat_eq(a692, a693) {
                    let a695 = {
                        let (_, h694) = v0;
                        h694
                    };
                    let a696 = 1u64;
                    nat_add(a695, a696)?
                } else {
                    let (_, h697) = v0;
                    h697
                }
            })))
        }
    }
}

pub fn f145(v0: Fn19, p703: u64, p704: (u64, bool)) -> R<((u64, bool), bool)> {
    let m702 = (p703, p704);
    {
        let mut m702 = m702;
        loop {
            let step = {
                let (v1, v2) = m702;
                let m699 = v1;
                if m699 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m699 - 1;
                    let m701 = {
                        let c700: Fn19 = v0.clone();
                        c700.apply(v2)?
                    };
                    match m701 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f230(v1, v0, v2)
}

pub fn f147(v0: (u64, bool)) -> bool {
    let (_, h705) = v0;
    h705
}

pub fn f148(v0: Adt26, v1: Adt23) -> R<Adt26> {
    let m707 = f231(match v1.clone() {
        Adt23::C0(h706, _, _, _) => {
            h706
        }
    })?;
    match m707 {
        Ok(v3) => {
            let v4: Bytes = {
                let (h712, _) = v3;
                h712
            };
            let v5: Bytes = {
                let (_, h713) = v3;
                h713
            };
            let v6: Bytes = f232(v4.clone(), v5.clone());
            let v7: bool = f214(match v0.clone() {
                Adt26::C0(_, _, h714) => {
                    h714
                }
            }, v6.clone())?;
            let v8: List<(Bytes, Bytes)> = f46(match v0.clone() {
                Adt26::C0(h715, _, _) => {
                    h715
                }
            }, if v7 {
                Some(f2(f5(f3(Str::lit("duplicate eliminator constructor ")), f6(v6.clone()))))
            } else {
                None::<(Bytes, Bytes)>
            });
            let v9: List<(Bytes, Bytes)> = f46(v8, if f73(match v1.clone() {
                Adt23::C0(_, h716, _, _) => {
                    h716
                }
            })? {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f6(match v1.clone() {
                    Adt23::C0(_, h717, _, _) => {
                        h717
                    }
                }), f3(Str::lit(" is not a Lean name")))))
            });
            let v10: List<(Bytes, Bytes)> = f233(match v1.clone() {
                Adt23::C0(_, _, h718, _) => {
                    h718
                }
            }, v9)?;
            let v11: List<(Bytes, Bytes)> = f233(match v1.clone() {
                Adt23::C0(_, _, _, h719) => {
                    h719
                }
            }, v10)?;
            Ok(Adt26::C0(v11, List::cons(Adt9::C0(v4, v5, match v1.clone() {
                Adt23::C0(_, h720, _, _) => {
                    h720
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h721, _) => {
                    h721
                }
            }, match v1 {
                Adt23::C0(_, _, _, h722) => {
                    h722
                }
            }), match v0.clone() {
                Adt26::C0(_, h723, _) => {
                    h723
                }
            }), List::cons(v6, match v0 {
                Adt26::C0(_, _, h724) => {
                    h724
                }
            })))
        }
        Err(v2) => {
            Ok(Adt26::C0(f46(match v0.clone() {
                Adt26::C0(h708, _, _) => {
                    h708
                }
            }, Some(f2(match v2 {
                Adt28::C0(_, h709) => {
                    h709
                }
            }))), match v0.clone() {
                Adt26::C0(_, h710, _) => {
                    h710
                }
            }, match v0 {
                Adt26::C0(_, _, h711) => {
                    h711
                }
            }))
        }
    }
}

pub fn f149(v0: Fn20, p728: List<Adt9>, p729: List<Adt9>) -> List<Adt9> {
    let m727 = (p728, p729);
    {
        let mut m727 = m727;
        loop {
            let step = {
                let (v1, v2) = m727;
                let m725 = v2;
                match m725.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c726: Fn20 = v0.clone();
                            c726.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f151(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f234(v0.clone())?;
    let v2: u64 = f235(v0.clone(), v1)?;
    Ok(f83(v0, v2, {
        let a730 = v1;
        let a731 = v2;
        nat_sub(a730, a731)
    }))
}

pub fn f152(v0: Str) -> Bytes {
    let a732 = v0;
    utf8_encode(a732)
}

pub fn f153(v0: List<Bytes>) -> List<Bytes> {
    f156(Fn21::F236, List::<Bytes>::nil(), v0)
}

pub fn f154(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a733 = v0.clone();
        length_bytes(a733)
    };
    let v3: u64 = {
        let a734 = v1.clone();
        length_bytes(a734)
    };
    let a735 = v3;
    let a736 = v2;
    if nat_le(a735, a736) {
        let m742 = {
            let a739 = v0;
            let a740 = {
                let a737 = v2;
                let a738 = v3;
                nat_sub(a737, a738)
            };
            let a741 = v3;
            slice_bytes(a739, a740, a741)
        };
        match m742 {
            None => {
                false
            }
            Some(v4) => {
                let a743 = v4;
                let a744 = v1;
                equal(a743, a744)
            }
        }
    } else {
        false
    }
}

pub fn f155(v0: Bytes) -> R<Option<List<Bytes>>> {
    let v1: u64 = {
        let a745 = v0.clone();
        length_bytes(a745)
    };
    let v2: Adt41 = {
        let (h746, _) = f237(Fn38::F238(v0, v1), {
            let a747 = v1;
            let a748 = 2u64;
            nat_add(a747, a748)?
        }, Adt41::C0(0u64, List::<Bytes>::nil(), 0u64))?;
        h746
    };
    let a750 = match v2.clone() {
        Adt41::C0(_, _, h749) => {
            h749
        }
    };
    let a751 = 1u64;
    if equal(a750, a751) {
        Ok(Some(f153(match v2 {
            Adt41::C0(_, h752, _) => {
                h752
            }
        })))
    } else {
        Ok(None::<List<Bytes>>)
    }
}

pub fn f156(v0: Fn21, p756: List<Bytes>, p757: List<Bytes>) -> List<Bytes> {
    let m755 = (p756, p757);
    {
        let mut m755 = m755;
        loop {
            let step = {
                let (v1, v2) = m755;
                let m753 = v2;
                match m753.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c754: Fn21 = v0.clone();
                            c754.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f158(v0: Bytes) -> R<bool> {
    let v1: Bytes = f151(v0)?;
    let a759 = {
        let a758 = v1.clone();
        length_bytes(a758)
    };
    let a760 = 0u64;
    if nat_eq(a759, a760) {
        Ok(true)
    } else {
        Ok(f159(v1, f152(Str::lit("#"))))
    }
}

pub fn f159(v0: Bytes, v1: Bytes) -> bool {
    let m765 = {
        let a762 = v0;
        let a763 = 0u64;
        let a764 = {
            let a761 = v1.clone();
            length_bytes(a761)
        };
        slice_bytes(a762, a763, a764)
    };
    match m765 {
        None => {
            false
        }
        Some(v2) => {
            let a766 = v2;
            let a767 = v1;
            equal(a766, a767)
        }
    }
}

pub fn f160(v0: Bytes) -> R<Option<(List<Bytes>, bool)>> {
    let v1: bool = f159(v0.clone(), f152(Str::lit("[[")));
    let v2: u64 = {
        let a768 = v0.clone();
        length_bytes(a768)
    };
    let v3: Bytes = if v1 {
        f83(v0, 2u64, {
            let a769 = v2;
            let a770 = 2u64;
            nat_sub(a769, a770)
        })
    } else {
        f83(v0, 1u64, {
            let a771 = v2;
            let a772 = 1u64;
            nat_sub(a771, a772)
        })
    };
    let v4: bool = if v1 {
        f154(v3.clone(), f152(Str::lit("]]")))
    } else {
        f154(v3.clone(), f152(Str::lit("]")))
    };
    if v4 {
        let v5: u64 = {
            let a773 = v3.clone();
            length_bytes(a773)
        };
        let v6: u64 = if v1 {
            2u64
        } else {
            1u64
        };
        let v7: Bytes = f83(v3, 0u64, {
            let a774 = v5;
            let a775 = v6;
            nat_sub(a774, a775)
        });
        let a777 = {
            let a776 = v7.clone();
            length_bytes(a776)
        };
        let a778 = 0u64;
        if nat_eq(a777, a778) {
            Ok(None::<(List<Bytes>, bool)>)
        } else {
            Ok(Some((f239(v7, 46u8)?, v1)))
        }
    } else {
        Ok(None::<(List<Bytes>, bool)>)
    }
}

pub fn f161(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a779 = v0.clone();
        length_bytes(a779)
    };
    let v3: u64 = {
        let a780 = v1.clone();
        length_bytes(a780)
    };
    let v4: u64 = {
        let (h781, _) = f240(Fn39::F241(v1.clone(), v3, v0.clone(), v2), {
            let a782 = v2;
            let a783 = 1u64;
            nat_add(a782, a783)?
        }, 0u64)?;
        h781
    };
    if f242(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f162(v0: List<(Bytes, Adt17)>, v1: Bytes) -> bool {
    f243(Fn40::F244(v1), false, v0)
}

pub fn f163(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let a784 = v2.clone();
    let a785 = f152(Str::lit("true"));
    if equal(a784, a785) {
        Ok(Adt18::C0(match v0.clone() {
            Adt18::C0(h786, _, _, _, _, _, _) => {
                h786
            }
        }, match v0.clone() {
            Adt18::C0(_, h787, _, _, _, _, _) => {
                h787
            }
        }, match v0.clone() {
            Adt18::C0(_, _, h788, _, _, _, _) => {
                h788
            }
        }, List::cons((v1, Adt17::C1(true)), match v0 {
            Adt18::C0(_, _, _, h789, _, _, _) => {
                h789
            }
        }), None::<Bytes>, List::<Bytes>::nil(), false))
    } else {
        let a790 = v2.clone();
        let a791 = f152(Str::lit("false"));
        if equal(a790, a791) {
            Ok(Adt18::C0(match v0.clone() {
                Adt18::C0(h792, _, _, _, _, _, _) => {
                    h792
                }
            }, match v0.clone() {
                Adt18::C0(_, h793, _, _, _, _, _) => {
                    h793
                }
            }, match v0.clone() {
                Adt18::C0(_, _, h794, _, _, _, _) => {
                    h794
                }
            }, List::cons((v1, Adt17::C1(false)), match v0 {
                Adt18::C0(_, _, _, h795, _, _, _) => {
                    h795
                }
            }), None::<Bytes>, List::<Bytes>::nil(), false))
        } else {
            if f159(v2.clone(), f152(Str::lit("\""))) {
                let m796 = f245(v2.clone(), 0u64)?;
                match m796 {
                    None => {
                        Ok(Adt18::C0(match v0.clone() {
                            Adt18::C0(h810, _, _, _, _, _, _) => {
                                h810
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h811, _, _, _, _, _) => {
                                h811
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h812, _, _, _, _) => {
                                h812
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h813, _, _, _) => {
                                h813
                            }
                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                    }
                    Some(v3) => {
                        let a799 = {
                            let (_, h797) = v3;
                            h797
                        };
                        let a800 = {
                            let a798 = v2;
                            length_bytes(a798)
                        };
                        if nat_eq(a799, a800) {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h801, _, _, _, _, _, _) => {
                                    h801
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h802, _, _, _, _, _) => {
                                    h802
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h803, _, _, _, _) => {
                                    h803
                                }
                            }, List::cons((v1, Adt17::C0({
                                let (h804, _) = v3;
                                h804
                            })), match v0 {
                                Adt18::C0(_, _, _, h805, _, _, _) => {
                                    h805
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        } else {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h806, _, _, _, _, _, _) => {
                                    h806
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h807, _, _, _, _, _) => {
                                    h807
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h808, _, _, _, _) => {
                                    h808
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h809, _, _, _) => {
                                    h809
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                }
            } else {
                if f159(v2.clone(), f152(Str::lit("["))) {
                    let a814 = v2.clone();
                    let a815 = f152(Str::lit("["));
                    if equal(a814, a815) {
                        Ok(Adt18::C0(match v0.clone() {
                            Adt18::C0(h816, _, _, _, _, _, _) => {
                                h816
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, h817, _, _, _, _, _) => {
                                h817
                            }
                        }, match v0.clone() {
                            Adt18::C0(_, _, h818, _, _, _, _) => {
                                h818
                            }
                        }, match v0 {
                            Adt18::C0(_, _, _, h819, _, _, _) => {
                                h819
                            }
                        }, Some(v1), List::<Bytes>::nil(), false))
                    } else {
                        if f154(v2.clone(), f152(Str::lit("]"))) {
                            let m823 = f155(f83(v2.clone(), 1u64, {
                                let a821 = {
                                    let a820 = v2;
                                    length_bytes(a820)
                                };
                                let a822 = 2u64;
                                nat_sub(a821, a822)
                            }))?;
                            match m823 {
                                None => {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h828, _, _, _, _, _, _) => {
                                            h828
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h829, _, _, _, _, _) => {
                                            h829
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h830, _, _, _, _) => {
                                            h830
                                        }
                                    }, match v0 {
                                        Adt18::C0(_, _, _, h831, _, _, _) => {
                                            h831
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                }
                                Some(v4) => {
                                    Ok(Adt18::C0(match v0.clone() {
                                        Adt18::C0(h824, _, _, _, _, _, _) => {
                                            h824
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, h825, _, _, _, _, _) => {
                                            h825
                                        }
                                    }, match v0.clone() {
                                        Adt18::C0(_, _, h826, _, _, _, _) => {
                                            h826
                                        }
                                    }, List::cons((v1, Adt17::C3(v4)), match v0 {
                                        Adt18::C0(_, _, _, h827, _, _, _) => {
                                            h827
                                        }
                                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                                }
                            }
                        } else {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h832, _, _, _, _, _, _) => {
                                    h832
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h833, _, _, _, _, _) => {
                                    h833
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h834, _, _, _, _) => {
                                    h834
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h835, _, _, _) => {
                                    h835
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                } else {
                    let m836 = f246(v2)?;
                    match m836 {
                        None => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h841, _, _, _, _, _, _) => {
                                    h841
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h842, _, _, _, _, _) => {
                                    h842
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h843, _, _, _, _) => {
                                    h843
                                }
                            }, match v0 {
                                Adt18::C0(_, _, _, h844, _, _, _) => {
                                    h844
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                        Some(v5) => {
                            Ok(Adt18::C0(match v0.clone() {
                                Adt18::C0(h837, _, _, _, _, _, _) => {
                                    h837
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, h838, _, _, _, _, _) => {
                                    h838
                                }
                            }, match v0.clone() {
                                Adt18::C0(_, _, h839, _, _, _, _) => {
                                    h839
                                }
                            }, List::cons((v1, Adt17::C2(v5)), match v0 {
                                Adt18::C0(_, _, _, h840, _, _, _) => {
                                    h840
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        }
                    }
                }
            }
        }
    }
}

pub fn f164(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h845, _) = v1.clone();
        h845
    };
    let v3: u64 = {
        let (h846, _) = {
            let (_, h847) = v1.clone();
            h847
        };
        h846
    };
    let v4: List<Bytes> = {
        let (_, h848) = {
            let (_, h849) = v1;
            h849
        };
        h848
    };
    let m850 = f247(v0.clone(), v2);
    match m850 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a851 = v5;
            let a852 = 10u8;
            if equal(a851, a852) {
                Ok(Some(({
                    let a853 = v2;
                    let a854 = 1u64;
                    nat_add(a853, a854)?
                }, ({
                    let a855 = v2;
                    let a856 = 1u64;
                    nat_add(a855, a856)?
                }, List::cons(f248(f83(v0, v3, {
                    let a857 = v2;
                    let a858 = v3;
                    nat_sub(a857, a858)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a859 = v2;
                    let a860 = 1u64;
                    nat_add(a859, a860)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f165(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f166(v0: Fn22, p864: List<(Bytes, Adt17)>, p865: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v1, v2) = m863;
                let m861 = v2;
                match m861.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt17)>, List<(Bytes, Adt17)>), List<(Bytes, Adt17)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt17)>, List<(Bytes, Adt17)>), List<(Bytes, Adt17)>>(({
                            let c862: Fn22 = v0.clone();
                            c862.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: List<(Bytes, Adt17)>, v1: (Bytes, Adt17)) -> List<(Bytes, Adt17)> {
    List::cons(v1, v0)
}

pub fn f168(v0: List<Adt16>, v1: List<Bytes>, v2: bool) -> R<bool> {
    f249(Fn41::F250(v2, v1), false, v0)
}

pub fn f169(v0: List<Adt16>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<List<Adt16>> {
    f251(f252(Fn42::F253(v2, v1, v3), List::<Adt16>::nil(), v0)?)
}

pub fn f170(v0: List<Adt16>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<List<Adt16>> {
    f251(List::cons(Adt16::C0(v1, v2, List::cons(v3, List::<List<(Bytes, Adt17)>>::nil())), f251(v0)?))
}

pub fn f171(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m866 = f41(v0, v1)?;
    match m866 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f172(v0: Fn23, p870: List<Bytes>, p871: List<(Bytes, Adt17)>) -> List<Bytes> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m867 = v2;
                match m867.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Adt17)>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Adt17)>), List<Bytes>>(({
                            let c868: Fn23 = v0.clone();
                            c868.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: List<Bytes>, v1: (Bytes, Adt17)) -> List<Bytes> {
    List::cons({
        let (h872, _) = v1;
        h872
    }, v0)
}

pub fn f174(v0: Fn24, p876: Option<Adt17>, p877: List<(Bytes, Adt17)>) -> Option<Adt17> {
    let m875 = (p876, p877);
    {
        let mut m875 = m875;
        loop {
            let step = {
                let (v1, v2) = m875;
                let m873 = v2;
                match m873.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<(Bytes, Adt17)>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<(Bytes, Adt17)>), Option<Adt17>>(({
                            let c874: Fn24 = v0.clone();
                            c874.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: Option<Adt17>, v2: (Bytes, Adt17)) -> Option<Adt17> {
    let m878 = v1.clone();
    match m878 {
        None => {
            let a880 = {
                let (h879, _) = v2;
                h879
            };
            let a881 = v0;
            if equal(a880, a881) {
                Some({
                    let (_, h882) = v2;
                    h882
                })
            } else {
                None::<Adt17>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f176(v0: Adt16, v1: List<Bytes>, v2: bool) -> R<bool> {
    if f254(match v0.clone() {
        Adt16::C0(h883, _, _) => {
            h883
        }
    }, v1)? {
        let a885 = match v0 {
            Adt16::C0(_, h884, _) => {
                h884
            }
        };
        let a886 = v2;
        Ok(equal(a885, a886))
    } else {
        Ok(false)
    }
}

pub fn f177(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<List<Bytes>> {
    let m887 = f91(v0, v1);
    match m887 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m888 = v2;
            match m888 {
                Adt17::C0(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C1(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C2(_) => {
                    None::<List<Bytes>>
                }
                Adt17::C3(v6) => {
                    Some(v6)
                }
            }
        }
    }
}

pub fn f178(v0: List<(Bytes, Adt17)>) -> R<Option<Adt21>> {
    if f27(v0.clone(), f255())? {
        let m889 = f29(v0.clone(), f3(Str::lit("id")));
        match m889 {
            None => {
                Ok(None::<Adt21>)
            }
            Some(v1) => {
                let m890 = f29(v0.clone(), f3(Str::lit("channel")));
                match m890 {
                    None => {
                        Ok(None::<Adt21>)
                    }
                    Some(v2) => {
                        let m891 = f29(v0.clone(), f3(Str::lit("surface")));
                        match m891 {
                            None => {
                                Ok(None::<Adt21>)
                            }
                            Some(v3) => {
                                let m892 = f256(v0.clone(), f3(Str::lit("canonical_source")));
                                match m892 {
                                    None => {
                                        Ok(None::<Adt21>)
                                    }
                                    Some(v4) => {
                                        let m893 = f177(v0, f3(Str::lit("features")));
                                        match m893 {
                                            None => {
                                                Ok(None::<Adt21>)
                                            }
                                            Some(v5) => {
                                                Ok(Some(Adt21::C0(v1, v2, v3, v4, v5)))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt21>)
    }
}

pub fn f179(v0: Fn25, p897: List<Adt21>, p898: List<Adt21>) -> List<Adt21> {
    let m896 = (p897, p898);
    {
        let mut m896 = m896;
        loop {
            let step = {
                let (v1, v2) = m896;
                let m894 = v2;
                match m894.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>(({
                            let c895: Fn25 = v0.clone();
                            c895.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<Adt21>, v1: Adt21) -> List<Adt21> {
    List::cons(v1, v0)
}

pub fn f181(v0: Fn26, p902: Option<List<Adt23>>, p903: List<List<(Bytes, Adt17)>>) -> R<Option<List<Adt23>>> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m899 = v2;
                match m899.uncons() {
                    None => {
                        Err::<(Option<List<Adt23>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt23>>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<Adt23>>, List<List<(Bytes, Adt17)>>), R<Option<List<Adt23>>>>(({
                            let c900: Fn26 = v0.clone();
                            c900.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Option<List<Adt23>>, v1: List<(Bytes, Adt17)>) -> R<Option<List<Adt23>>> {
    let m904 = v0;
    match m904 {
        None => {
            Ok(None::<List<Adt23>>)
        }
        Some(v2) => {
            let m905 = f257(v1)?;
            match m905 {
                None => {
                    Ok(None::<List<Adt23>>)
                }
                Some(v3) => {
                    Ok(Some(List::cons(v3, v2)))
                }
            }
        }
    }
}

pub fn f183(v0: List<Adt23>) -> List<Adt23> {
    f258(Fn43::F259, List::<Adt23>::nil(), v0)
}

pub fn f184(v0: Str) -> Bytes {
    let a906 = v0;
    utf8_encode(a906)
}

pub fn f185(v0: Fn27, p910: (u64, u64), p911: List<u64>) -> R<(u64, u64)> {
    let m909 = (p910, p911);
    {
        let mut m909 = m909;
        loop {
            let step = {
                let (v1, v2) = m909;
                let m907 = v2;
                match m907.uncons() {
                    None => {
                        Err::<((u64, u64), List<u64>), R<(u64, u64)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, u64), List<u64>), R<(u64, u64)>>(({
                            let c908: Fn27 = v0.clone();
                            c908.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: u64, v1: (u64, u64), v2: u64) -> R<(u64, u64)> {
    let a913 = {
        let (h912, _) = v1;
        h912
    };
    let a914 = v0;
    if nat_eq(a913, a914) {
        Ok(({
            let a916 = {
                let (h915, _) = v1;
                h915
            };
            let a917 = 1u64;
            nat_add(a916, a917)?
        }, v2))
    } else {
        Ok(({
            let a919 = {
                let (h918, _) = v1;
                h918
            };
            let a920 = 1u64;
            nat_add(a919, a920)?
        }, {
            let (_, h921) = v1;
            h921
        }))
    }
}

pub fn f187(v0: Fn28, p926: u64, p927: Adt32) -> R<(Adt32, bool)> {
    let m925 = (p926, p927);
    {
        let mut m925 = m925;
        loop {
            let step = {
                let (v1, v2) = m925;
                let m922 = v1;
                if m922 == 0 {
                    Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m922 - 1;
                    let m924 = {
                        let c923: Fn28 = v0.clone();
                        c923.apply(v2.clone())?
                    };
                    match m924 {
                        None => {
                            Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt32), R<(Adt32, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: bool, v1: u64, v2: Bytes, v3: Adt32) -> R<Option<Adt32>> {
    f260(v2, v1, v0, v3)
}

pub fn f189(v0: Bytes, v1: u64) -> Bytes {
    let m931 = {
        let a928 = v0;
        let a929 = v1;
        let a930 = 1u64;
        slice_bytes(a928, a929, a930)
    };
    match m931 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f190(v0: Fn29, p936: u64, p937: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m935 = (p936, p937);
    {
        let mut m935 = m935;
        loop {
            let step = {
                let (v1, v2) = m935;
                let m932 = v1;
                if m932 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m932 - 1;
                    let m934 = {
                        let c933: Fn29 = v0.clone();
                        c933.apply(v2.clone())
                    };
                    match m934 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f261(v1, v0, v2)
}

pub fn f192(v0: (Option<Bytes>, bool), v1: Bytes) -> (Option<Bytes>, bool) {
    let m939 = {
        let (h938, _) = v0;
        h938
    };
    match m939 {
        None => {
            (Some(v1), {
                let (_, h940) = v0;
                h940
            })
        }
        Some(v2) => {
            (Some(v1.clone()), {
                let (_, h941) = v0;
                if h941 {
                    f262(v2, v1)
                } else {
                    false
                }
            })
        }
    }
}

pub fn f193(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a942 = v0.clone();
        length_bytes(a942)
    };
    let a943 = v1;
    let a944 = 0u64;
    if nat_eq(a943, a944) {
        Ok(false)
    } else {
        if f263(f264(v0.clone(), 0u64)?) {
            let a946 = {
                let (h945, _) = f240(Fn39::F265(v1, v0), v1, 1u64)?;
                h945
            };
            let a947 = v1;
            Ok(nat_eq(a946, a947))
        } else {
            Ok(false)
        }
    }
}

pub fn f194(v0: Fn30, p952: u64, p953: Adt34) -> R<(Adt34, bool)> {
    let m951 = (p952, p953);
    {
        let mut m951 = m951;
        loop {
            let step = {
                let (v1, v2) = m951;
                let m948 = v1;
                if m948 == 0 {
                    Err::<(u64, Adt34), R<(Adt34, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m948 - 1;
                    let m950 = {
                        let c949: Fn30 = v0.clone();
                        c949.apply(v2.clone())?
                    };
                    match m950 {
                        None => {
                            Err::<(u64, Adt34), R<(Adt34, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt34), R<(Adt34, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m951 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: u64, v1: Bytes, v2: Adt34) -> R<Option<Adt34>> {
    f266(v1, v0, v2)
}

pub fn f196(v0: List<Adt33>) -> List<Adt33> {
    f267(Fn44::F268, List::<Adt33>::nil(), v0)
}

pub fn f197(v0: Bytes, v1: Adt34) -> List<Adt33> {
    if match v1.clone() {
        Adt34::C0(_, _, h954, _, _) => {
            h954
        }
    } {
        List::cons(Adt33::C2(f83(v0, match v1.clone() {
            Adt34::C0(_, h955, _, _, _) => {
                h955
            }
        }, {
            let a958 = match v1.clone() {
                Adt34::C0(h956, _, _, _, _) => {
                    h956
                }
            };
            let a959 = match v1.clone() {
                Adt34::C0(_, h957, _, _, _) => {
                    h957
                }
            };
            nat_sub(a958, a959)
        })), match v1 {
            Adt34::C0(_, _, _, h960, _) => {
                h960
            }
        })
    } else {
        match v1 {
            Adt34::C0(_, _, _, h961, _) => {
                h961
            }
        }
    }
}

pub fn f198(v0: Fn31, p966: u64, p967: Adt36) -> R<(Adt36, bool)> {
    let m965 = (p966, p967);
    {
        let mut m965 = m965;
        loop {
            let step = {
                let (v1, v2) = m965;
                let m962 = v1;
                if m962 == 0 {
                    Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m962 - 1;
                    let m964 = {
                        let c963: Fn31 = v0.clone();
                        c963.apply(v2.clone())?
                    };
                    match m964 {
                        None => {
                            Err::<(u64, Adt36), R<(Adt36, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt36), R<(Adt36, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: u64, v1: List<Adt33>, v2: Adt36) -> R<Option<Adt36>> {
    f269(v1, v0, v2)
}

pub fn f200(v0: Bytes) -> Adt28 {
    Adt28::C0(0u64, v0)
}

pub fn f201(v0: Str) -> Bytes {
    let a968 = v0;
    utf8_encode(a968)
}

pub fn f202(v0: Fn32, p973: u64, p974: Adt37) -> R<(Adt37, bool)> {
    let m972 = (p973, p974);
    {
        let mut m972 = m972;
        loop {
            let step = {
                let (v1, v2) = m972;
                let m969 = v1;
                if m969 == 0 {
                    Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m969 - 1;
                    let m971 = {
                        let c970: Fn32 = v0.clone();
                        c970.apply(v2.clone())?
                    };
                    match m971 {
                        None => {
                            Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt37), R<(Adt37, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m972 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: u64, v1: Adt37) -> R<Option<Adt37>> {
    f270(v0, v1)
}

pub fn f204(v0: List<Bytes>, v1: Adt29) -> R<Option<Adt29>> {
    let m976 = match v1.clone() {
        Adt29::C0(_, _, _, h975) => {
            h975
        }
    };
    match m976 {
        None => {
            let m978 = match v1.clone() {
                Adt29::C0(h977, _, _, _) => {
                    h977
                }
            };
            match m978.uncons() {
                None => {
                    Ok(None::<Adt29>)
                }
                Some((v3, v4)) => {
                    let m979 = v3;
                    match m979 {
                        Adt30::C0(v5) => {
                            f271(v1, v4, v5)
                        }
                        Adt30::C1(v6) => {
                            Ok(Some(Adt29::C0(v4, List::cons(v6, match v1.clone() {
                                Adt29::C0(_, h980, _, _) => {
                                    h980
                                }
                            }), {
                                let a982 = match v1 {
                                    Adt29::C0(_, _, h981, _) => {
                                        h981
                                    }
                                };
                                let a983 = 1u64;
                                nat_add(a982, a983)?
                            }, None::<Bytes>)))
                        }
                        Adt30::C2(v7) => {
                            Ok(Some(Adt29::C0(v4, f272(match v1.clone() {
                                Adt29::C0(_, h984, _, _) => {
                                    h984
                                }
                            }, {
                                let a986 = match v1 {
                                    Adt29::C0(_, _, h985, _) => {
                                        h985
                                    }
                                };
                                let a987 = v7;
                                nat_sub(a986, a987)
                            }), v7, None::<Bytes>)))
                        }
                        Adt30::C3(v8) => {
                            f273(v1, v4, v0, v8)
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt29>)
        }
    }
}

pub fn f205(v0: Bytes, v1: Bytes) -> Bytes {
    let a988 = v0;
    let a989 = v1;
    append_bytes(a988, a989)
}

pub fn f206(v0: Adt2) -> Bytes {
    let m990 = v0;
    match m990 {
        Adt2::C0(v1) => {
            f48(v1)
        }
        Adt2::C1(v2) => {
            v2
        }
        Adt2::C2(r991) => {
            let v3 = (*r991).clone();
            f205(f205(f201(Str::lit("(succ ")), f206(v3)), f201(Str::lit(")")))
        }
        Adt2::C3(r992) => {
            let v4 = (*r992).clone();
            f205(f205(f201(Str::lit("(max")), f274(v4)), f201(Str::lit(")")))
        }
        Adt2::C4(r993, r994) => {
            let v5 = (*r993).clone();
            let v6 = (*r994).clone();
            f205(f205(f205(f205(f201(Str::lit("(imax ")), f206(v5)), f201(Str::lit(" "))), f206(v6)), f201(Str::lit(")")))
        }
        Adt2::C5 => {
            Bytes::lit(&[])
        }
        Adt2::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f207(v0: Bytes, v1: Bytes, v2: Adt2) -> Bytes {
    let v3: Bytes = f205(f205(f205(f201(Str::lit("(const ")), v0), f201(Str::lit("::"))), v1);
    let m995 = v2.clone();
    match m995 {
        Adt2::C0(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C1(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C2(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C3(_) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C4(_, _) => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C5 => {
            f205(v3, f201(Str::lit(")")))
        }
        Adt2::C6(_, _) => {
            let v6: Bytes = f274(v2);
            f205(f205(f205(v3, f201(Str::lit(" ("))), f83(v6.clone(), 1u64, {
                let a997 = {
                    let a996 = v6;
                    length_bytes(a996)
                };
                let a998 = 1u64;
                nat_sub(a997, a998)
            })), f201(Str::lit("))")))
        }
    }
}

pub fn f208(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m999 = f275(Fn45::F276(v1.clone()), None::<Bytes>, v0);
    match m999 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f209(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m1000 = v0;
    match m1000 {
        Adt1::C0 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C1(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C2(_, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C3(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C4(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C5(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C6(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C7(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C8(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt1::C10(r1005, r1006) => {
            let v4 = (*r1005).clone();
            let v5 = (*r1006).clone();
            let v6: (Bytes, u64) = f124(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f209(v5, v1, v2, {
                let (_, h1001) = v6;
                h1001
            })?;
            Ok((f205(f205(f201(Str::lit(" ")), {
                let (h1002, _) = v6;
                h1002
            }), {
                let (h1003, _) = v7;
                h1003
            }), {
                let (_, h1004) = v7;
                h1004
            }))
        }
        Adt1::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f210(v0: Adt1, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m1007 = v0;
    match m1007 {
        Adt1::C0 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C1(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C2(_, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C3(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C4(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C5(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C6(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C7(_, _, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C8(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C9 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C10(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt1::C11(v4, v5, r1030, r1031) => {
            let v6 = (*r1030).clone();
            let v7 = (*r1031).clone();
            let v8: (Bytes, u64) = f211(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f124(v6, v1, v2.clone(), {
                let (_, h1008) = v8.clone();
                h1008
            })?;
            let v10: Bytes = f205(f205(f205(f205(f205(f205(f201(Str::lit("(")), f277(v4)), f201(Str::lit(" "))), {
                let (h1009, _) = v8.clone();
                h1009
            }), f201(Str::lit(" "))), {
                let (h1010, _) = v9;
                h1010
            }), f201(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h1011, _) = v8;
                h1011
            }), v2);
            let m1012 = v7.clone();
            match m1012 {
                Adt1::C0 => {
                    Ok(((v10, v11), {
                        let (_, h1020) = v9;
                        h1020
                    }))
                }
                Adt1::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h1021) = v9;
                        h1021
                    }))
                }
                Adt1::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h1022) = v9;
                        h1022
                    }))
                }
                Adt1::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h1023) = v9;
                        h1023
                    }))
                }
                Adt1::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1024) = v9;
                        h1024
                    }))
                }
                Adt1::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1025) = v9;
                        h1025
                    }))
                }
                Adt1::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1026) = v9;
                        h1026
                    }))
                }
                Adt1::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h1027) = v9;
                        h1027
                    }))
                }
                Adt1::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h1028) = v9;
                        h1028
                    }))
                }
                Adt1::C9 => {
                    Ok(((v10, v11), {
                        let (_, h1013) = v9;
                        h1013
                    }))
                }
                Adt1::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h1029) = v9;
                        h1029
                    }))
                }
                Adt1::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f210(v7, v1, v11, {
                        let (_, h1014) = v9;
                        h1014
                    })?;
                    Ok(((f205(f205(v10, f201(Str::lit(" "))), {
                        let (h1015, _) = {
                            let (h1016, _) = v16.clone();
                            h1016
                        };
                        h1015
                    }), {
                        let (_, h1017) = {
                            let (h1018, _) = v16;
                            h1018
                        };
                        h1017
                    }), {
                        let (_, h1019) = v16;
                        h1019
                    }))
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f205(f201(Str::lit("x")), f48(v2)), {
            let a1032 = v2;
            let a1033 = 1u64;
            nat_add(a1032, a1033)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f212(v0: Bytes) -> R<Adt39> {
    let v1: u64 = {
        let a1034 = v0.clone();
        length_bytes(a1034)
    };
    let v2: u64 = {
        let a1035 = v1;
        let a1036 = 0u64;
        if nat_eq(a1035, a1036) {
            1u64
        } else {
            let a1039 = {
                let a1037 = v1;
                let a1038 = 1023u64;
                nat_add(a1037, a1038)?
            };
            let a1040 = 1024u64;
            let a1041 = 0u64;
            nat_quot(a1039, a1040, a1041)
        }
    };
    let v3: u64 = {
        let a1042 = v2;
        let a1043 = 1u64;
        nat_sub(a1042, a1043)
    };
    let m1045 = {
        let (_, h1044) = f278(v0.clone(), v1, v3)?;
        h1044
    };
    match m1045.uncons() {
        None => {
            f279(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h1046) = f280(Fn46::F281, 64u64, (List::cons(v4, v5), f279(v0, v1, v3, false)?));
            Ok(h1046)
        }
    }
}

pub fn f213(v0: u32) -> Bytes {
    let a1059 = {
        let a1047 = f282(v0, 4u32);
        let a1048 = f282(v0, 0u32);
        append_bytes(a1047, a1048)
    };
    let a1060 = {
        let a1057 = {
            let a1049 = f282(v0, 12u32);
            let a1050 = f282(v0, 8u32);
            append_bytes(a1049, a1050)
        };
        let a1058 = {
            let a1055 = {
                let a1051 = f282(v0, 20u32);
                let a1052 = f282(v0, 16u32);
                append_bytes(a1051, a1052)
            };
            let a1056 = {
                let a1053 = f282(v0, 28u32);
                let a1054 = f282(v0, 24u32);
                append_bytes(a1053, a1054)
            };
            append_bytes(a1055, a1056)
        };
        append_bytes(a1057, a1058)
    };
    append_bytes(a1059, a1060)
}

pub fn f214(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f88(Fn8::F283(v1), false, v0)
}

pub fn f215(v0: Adt21, v1: bool, v2: bool, v3: List<Bytes>, v4: List<(Bytes, Bytes)>) -> R<(List<(Bytes, Bytes)>, Option<Adt4>)> {
    let v5: Bytes = match v0.clone() {
        Adt21::C0(h1061, _, _, _, _) => {
            h1061
        }
    };
    let v6: List<(Bytes, Bytes)> = f46(v4, if f284(v5.clone())? {
        None::<(Bytes, Bytes)>
    } else {
        Some(f2(f5(f6(v5.clone()), f3(Str::lit(" is not a form ID ([a-z][a-z0-9-]*)")))))
    });
    let v7: List<(Bytes, Bytes)> = f46(v6, if v1 {
        Some(f2(f5(f3(Str::lit("duplicate form ID ")), f6(v5.clone()))))
    } else {
        None::<(Bytes, Bytes)>
    });
    let v8: Adt42 = f285(v5.clone(), match v0.clone() {
        Adt21::C0(_, h1062, _, _, _) => {
            h1062
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1063, _, _) => {
            h1063
        }
    }, v7)?;
    let m1065 = match v8.clone() {
        Adt42::C0(_, h1064, _) => {
            h1064
        }
    };
    match m1065 {
        None => {
            Ok((match v8 {
                Adt42::C0(h1066, _, _) => {
                    h1066
                }
            }, None::<Adt4>))
        }
        Some(v9) => {
            let v10: List<Adt5> = match v8.clone() {
                Adt42::C0(_, _, h1067) => {
                    h1067
                }
            };
            let v11: List<(Bytes, Bytes)> = f286(v3, v5.clone(), v10.clone(), match v8 {
                Adt42::C0(h1068, _, _) => {
                    h1068
                }
            })?;
            let v12: bool = f287(v10.clone());
            let v13: List<(Bytes, Bytes)> = f46(v11, if if if v12 {
                match v0.clone() {
                    Adt21::C0(_, _, _, h1069, _) => {
                        h1069
                    }
                }
            } else {
                false
            } {
                !v2
            } else {
                false
            } {
                Some(f2(f5(f5(f3(Str::lit("form `")), v5.clone()), f3(Str::lit("`: control-sequence forms are aliases only for non-core entries")))))
            } else {
                None::<(Bytes, Bytes)>
            });
            let v14: bool = if if !match v0.clone() {
                Adt21::C0(_, _, _, h1070, _) => {
                    h1070
                }
            } {
                let a1072 = {
                    let a1071 = v10.clone();
                    length_list(a1071)
                };
                let a1073 = 1u64;
                nat_eq(a1072, a1073)
            } else {
                false
            } {
                f287(v10.clone())
            } else {
                false
            };
            let v16: List<(Bytes, Bytes)> = if if !v2 {
                !v14
            } else {
                false
            } {
                let m1074 = f288(v10.clone(), v9)?;
                match m1074 {
                    None => {
                        v13
                    }
                    Some(v15) => {
                        f46(v13, Some(f289(f5(f5(f5(f5(f5(f3(Str::lit("form `")), v5.clone()), f3(Str::lit("`: the "))), if match v0.clone() {
                            Adt21::C0(_, _, _, h1075, _) => {
                                h1075
                            }
                        } {
                            f3(Str::lit("canonical"))
                        } else {
                            f3(Str::lit("alias"))
                        }), f5(f3(Str::lit(" surface `")), match v0.clone() {
                            Adt21::C0(_, _, h1076, _, _) => {
                                h1076
                            }
                        })), f5(f3(Str::lit("` is not renderer-safe: ")), v15)))))
                    }
                }
            } else {
                v13
            };
            let v17: List<(Bytes, Bytes)> = f290(v5.clone(), match v0.clone() {
                Adt21::C0(_, _, _, _, h1077) => {
                    h1077
                }
            }, v16)?;
            Ok((v17, Some(Adt4::C0(v5, v9, match v0.clone() {
                Adt21::C0(_, _, h1078, _, _) => {
                    h1078
                }
            }, v10, match v0.clone() {
                Adt21::C0(_, _, _, h1079, _) => {
                    h1079
                }
            }, match v0 {
                Adt21::C0(_, _, _, _, h1080) => {
                    h1080
                }
            }))))
        }
    }
}

pub fn f216(v0: u64, v1: u64) -> bool {
    let a1081 = v0;
    let a1082 = 2u64;
    if nat_eq(a1081, a1082) {
        true
    } else {
        let a1083 = v0;
        let a1084 = v1;
        nat_eq(a1083, a1084)
    }
}

pub fn f217(v0: Adt35, v1: u64) -> R<Result<Adt7, Adt28>> {
    let v2: Adt43 = {
        let (h1085, _) = f291(Fn47::F292(v1), {
            let a1088 = {
                let a1086 = v1;
                let a1087 = 2u64;
                nat_mul(a1086, a1087)?
            };
            let a1089 = 2u64;
            nat_add(a1088, a1089)?
        }, Adt43::C0(List::cons(Adt44::C0(v0), List::<Adt44>::nil()), List::<Adt7>::nil(), None::<Adt28>))?;
        h1085
    };
    let m1091 = match v2.clone() {
        Adt43::C0(_, _, h1090) => {
            h1090
        }
    };
    match m1091 {
        None => {
            let m1093 = match v2 {
                Adt43::C0(_, h1092, _) => {
                    h1092
                }
            };
            match m1093.uncons() {
                None => {
                    Ok(Err::<Adt7, Adt28>(f200(f201(Str::lit("malformed render expression")))))
                }
                Some((v4, v5)) => {
                    let m1094 = v5;
                    match m1094.uncons() {
                        None => {
                            Ok(Ok::<Adt7, Adt28>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt7, Adt28>(f200(f201(Str::lit("malformed render expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt7, Adt28>(v3))
        }
    }
}

pub fn f218(v0: Fn33, p1099: u64, p1100: Adt40) -> R<(Adt40, bool)> {
    let m1098 = (p1099, p1100);
    {
        let mut m1098 = m1098;
        loop {
            let step = {
                let (v1, v2) = m1098;
                let m1095 = v1;
                if m1095 == 0 {
                    Err::<(u64, Adt40), R<(Adt40, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1095 - 1;
                    let m1097 = {
                        let c1096: Fn33 = v0.clone();
                        c1096.apply(v2.clone())?
                    };
                    match m1097 {
                        None => {
                            Err::<(u64, Adt40), R<(Adt40, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt40), R<(Adt40, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: u64, v1: Adt40) -> R<Option<Adt40>> {
    f293(v0, v1)
}

pub fn f220(v0: List<u64>) -> List<u64> {
    f224(Fn35::F294, List::<u64>::nil(), v0)
}

pub fn f221(v0: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    f295(Fn48::F296, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)
}

pub fn f222(v0: Fn34, p1104: Bytes, p1105: List<u64>) -> Bytes {
    let m1103 = (p1104, p1105);
    {
        let mut m1103 = m1103;
        loop {
            let step = {
                let (v1, v2) = m1103;
                let m1101 = v2;
                match m1101.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let c1102: Fn34 = v0.clone();
                            c1102.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Bytes, v1: u64) -> Bytes {
    f5(f5(v0, f3(Str::lit(","))), f48(v1))
}

pub fn f224(v0: Fn35, p1109: List<u64>, p1110: List<u64>) -> List<u64> {
    let m1108 = (p1109, p1110);
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let (v1, v2) = m1108;
                let m1106 = v2;
                match m1106.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1107: Fn35 = v0.clone();
                            c1107.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<u64>, v1: u64) -> List<u64> {
    f297(v0, v1)
}

pub fn f226(v0: Fn36, p1115: u64, p1116: (u64, List<u64>)) -> R<((u64, List<u64>), bool)> {
    let m1114 = (p1115, p1116);
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let (v1, v2) = m1114;
                let m1111 = v1;
                if m1111 == 0 {
                    Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1111 - 1;
                    let m1113 = {
                        let c1112: Fn36 = v0.clone();
                        c1112.apply(v2.clone())?
                    };
                    match m1113 {
                        None => {
                            Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: u64, v1: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
    let a1118 = {
        let (h1117, _) = v1.clone();
        h1117
    };
    let a1119 = v0;
    if nat_lt(a1118, a1119) {
        Ok(Some(({
            let a1121 = {
                let (h1120, _) = v1.clone();
                h1120
            };
            let a1122 = 1u64;
            nat_add(a1121, a1122)?
        }, List::cons({
            let (h1123, _) = v1;
            h1123
        }, {
            let (_, h1124) = v1;
            h1124
        }))))
    } else {
        Ok(None::<(u64, List<u64>)>)
    }
}

pub fn f228(v0: Fn37, p1128: List<(Bytes, Bytes)>, p1129: List<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1127 = (p1128, p1129);
    {
        let mut m1127 = m1127;
        loop {
            let step = {
                let (v1, v2) = m1127;
                let m1125 = v2;
                match m1125.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<Bytes>), R<List<(Bytes, Bytes)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<Bytes>), R<List<(Bytes, Bytes)>>>(({
                            let c1126: Fn37 = v0.clone();
                            c1126.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: List<Adt4>, v2: u64, v3: List<(Bytes, Bytes)>, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    f298(v1, v2, v0, v4, v3)
}

pub fn f230(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h1130, _) = v2;
        h1130
    };
    let a1131 = v3;
    let a1132 = v1;
    if nat_lt(a1131, a1132) {
        let v4: u64 = f264(v0, v3)?;
        let a1133 = v4;
        let a1134 = 46u64;
        if nat_eq(a1133, a1134) {
            if f147(v2) {
                Ok(None::<(u64, bool)>)
            } else {
                Ok(Some(({
                    let a1135 = v3;
                    let a1136 = 1u64;
                    nat_add(a1135, a1136)?
                }, true)))
            }
        } else {
            if f147(v2) {
                if f299(v4) {
                    Ok(Some(({
                        let a1137 = v3;
                        let a1138 = 1u64;
                        nat_add(a1137, a1138)?
                    }, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            } else {
                if f300(v4) {
                    Ok(Some(({
                        let a1139 = v3;
                        let a1140 = 1u64;
                        nat_add(a1139, a1140)?
                    }, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            }
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f231(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt28>> {
    let m1141 = f161(v0.clone(), f201(Str::lit("::")))?;
    match m1141 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v0), f201(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f83(v0.clone(), 0u64, v1);
            let v3: Bytes = f83(v0.clone(), {
                let a1142 = v1;
                let a1143 = 2u64;
                nat_add(a1142, a1143)?
            }, {
                let a1147 = {
                    let a1144 = v0;
                    length_bytes(a1144)
                };
                let a1148 = {
                    let a1145 = v1;
                    let a1146 = 2u64;
                    nat_add(a1145, a1146)?
                };
                nat_sub(a1147, a1148)
            });
            if f302(v2.clone())? {
                if f47(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt28>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v3), f201(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt28>(f200(f205(f301(v2), f201(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: Bytes) -> Bytes {
    f5(f5(v0, f3(Str::lit("::"))), v1)
}

pub fn f233(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let (h1149, _) = f303(Fn49::F304, (v1, List::<Bytes>::nil()), v0)?;
    Ok(h1149)
}

pub fn f234(v0: Bytes) -> R<u64> {
    let (h1150, _) = f240(Fn39::F305(v0.clone()), {
        let a1151 = v0.clone();
        length_bytes(a1151)
    }, {
        let a1152 = v0;
        length_bytes(a1152)
    })?;
    Ok(h1150)
}

pub fn f235(v0: Bytes, v1: u64) -> R<u64> {
    let (h1153, _) = f240(Fn39::F306(v1, v0), v1, 0u64)?;
    Ok(h1153)
}

pub fn f236(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f237(v0: Fn38, p1158: u64, p1159: Adt41) -> R<(Adt41, bool)> {
    let m1157 = (p1158, p1159);
    {
        let mut m1157 = m1157;
        loop {
            let step = {
                let (v1, v2) = m1157;
                let m1154 = v1;
                if m1154 == 0 {
                    Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1154 - 1;
                    let m1156 = {
                        let c1155: Fn38 = v0.clone();
                        c1155.apply(v2.clone())?
                    };
                    match m1156 {
                        None => {
                            Err::<(u64, Adt41), R<(Adt41, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt41), R<(Adt41, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes, v1: u64, v2: Adt41) -> R<Option<Adt41>> {
    f307(v0, v1, v2)
}

pub fn f239(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a1160 = v0.clone();
        length_bytes(a1160)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h1161, _) = f80(Fn6::F308(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1161
    };
    let v4: u64 = {
        let (h1162, _) = {
            let (_, h1163) = v3.clone();
            h1163
        };
        h1162
    };
    Ok(f82(List::cons(f83(v0, v4, {
        let a1164 = v2;
        let a1165 = v4;
        nat_sub(a1164, a1165)
    }), {
        let (_, h1166) = {
            let (_, h1167) = v3;
            h1167
        };
        h1166
    })))
}

pub fn f240(v0: Fn39, p1172: u64, p1173: u64) -> R<(u64, bool)> {
    let m1171 = (p1172, p1173);
    {
        let mut m1171 = m1171;
        loop {
            let step = {
                let (v1, v2) = m1171;
                let m1168 = v1;
                if m1168 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1168 - 1;
                    let m1170 = {
                        let c1169: Fn39 = v0.clone();
                        c1169.apply(v2)?
                    };
                    match m1170 {
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
                Ok(next) => { m1171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f242(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1176 = {
            let a1174 = v4;
            let a1175 = v1;
            nat_add(a1174, a1175)?
        };
        let a1177 = v3;
        if nat_lt(a1176, a1177) {
            Ok(Some({
                let a1178 = v4;
                let a1179 = 1u64;
                nat_add(a1178, a1179)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f242(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1184 = {
        let a1181 = v0;
        let a1182 = v2;
        let a1183 = {
            let a1180 = v1.clone();
            length_bytes(a1180)
        };
        slice_bytes(a1181, a1182, a1183)
    };
    match m1184 {
        None => {
            false
        }
        Some(v3) => {
            let a1185 = v3;
            let a1186 = v1;
            equal(a1185, a1186)
        }
    }
}

pub fn f243(v0: Fn40, p1190: bool, p1191: List<(Bytes, Adt17)>) -> bool {
    let m1189 = (p1190, p1191);
    {
        let mut m1189 = m1189;
        loop {
            let step = {
                let (v1, v2) = m1189;
                let m1187 = v2;
                match m1187.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt17)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt17)>), bool>(({
                            let c1188: Fn40 = v0.clone();
                            c1188.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes, v1: bool, v2: (Bytes, Adt17)) -> bool {
    if v1 {
        true
    } else {
        let a1193 = {
            let (h1192, _) = v2;
            h1192
        };
        let a1194 = v0;
        equal(a1193, a1194)
    }
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<(Bytes, u64)>> {
    let m1195 = f247(v0.clone(), v1);
    match m1195 {
        None => {
            Ok(None::<(Bytes, u64)>)
        }
        Some(v2) => {
            let a1196 = v2;
            let a1197 = 34u8;
            if equal(a1196, a1197) {
                let v3: u64 = {
                    let a1198 = v0.clone();
                    length_bytes(a1198)
                };
                let v4: Adt45 = {
                    let (h1199, _) = f309(Fn50::F310(v0, v3), {
                        let a1200 = v3;
                        let a1201 = 2u64;
                        nat_add(a1200, a1201)?
                    }, Adt45::C0({
                        let a1202 = v1;
                        let a1203 = 1u64;
                        nat_add(a1202, a1203)?
                    }, Bytes::lit(&[]), 0u64, false))?;
                    h1199
                };
                if match v4.clone() {
                    Adt45::C0(_, _, _, h1204) => {
                        h1204
                    }
                } {
                    Ok(Some((match v4.clone() {
                        Adt45::C0(_, h1205, _, _) => {
                            h1205
                        }
                    }, match v4 {
                        Adt45::C0(h1206, _, _, _) => {
                            h1206
                        }
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f246(v0: Bytes) -> R<Option<u64>> {
    if f311(v0.clone())? {
        Ok(Some({
            let (_, h1207) = {
                let (h1208, _) = f312(Fn51::F313(v0.clone()), {
                    let a1209 = v0;
                    length_bytes(a1209)
                }, (0u64, 0u64))?;
                h1208
            };
            h1207
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f247(v0: Bytes, v1: u64) -> Option<u8> {
    let a1210 = v0;
    let a1211 = v1;
    index_bytes(a1210, a1211)
}

pub fn f248(v0: Bytes) -> Bytes {
    if f154(v0.clone(), Bytes::lit(&[0x0d])) {
        f83(v0.clone(), 0u64, {
            let a1213 = {
                let a1212 = v0;
                length_bytes(a1212)
            };
            let a1214 = 1u64;
            nat_sub(a1213, a1214)
        })
    } else {
        v0
    }
}

pub fn f249(v0: Fn41, p1218: bool, p1219: List<Adt16>) -> R<bool> {
    let m1217 = (p1218, p1219);
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let (v1, v2) = m1217;
                let m1215 = v2;
                match m1215.uncons() {
                    None => {
                        Err::<(bool, List<Adt16>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt16>), R<bool>>(({
                            let c1216: Fn41 = v0.clone();
                            c1216.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: bool, v1: List<Bytes>, v2: bool, v3: Adt16) -> R<bool> {
    if v2 {
        Ok(true)
    } else {
        f176(v3, v1, v0)
    }
}

pub fn f251(v0: List<Adt16>) -> R<List<Adt16>> {
    f252(Fn42::F314, List::<Adt16>::nil(), v0)
}

pub fn f252(v0: Fn42, p1223: List<Adt16>, p1224: List<Adt16>) -> R<List<Adt16>> {
    let m1222 = (p1223, p1224);
    {
        let mut m1222 = m1222;
        loop {
            let step = {
                let (v1, v2) = m1222;
                let m1220 = v2;
                match m1220.uncons() {
                    None => {
                        Err::<(List<Adt16>, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt16>, List<Adt16>), R<List<Adt16>>>(({
                            let c1221: Fn42 = v0.clone();
                            c1221.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: bool, v1: List<Bytes>, v2: List<(Bytes, Adt17)>, v3: List<Adt16>, v4: Adt16) -> R<List<Adt16>> {
    Ok(List::cons(f315(v4, v1, v0, v2)?, v3))
}

pub fn f254(v0: List<Bytes>, v1: List<Bytes>) -> R<bool> {
    let a1225 = f316(v0, Bytes::lit(&[0x2e]))?;
    let a1226 = f316(v1, Bytes::lit(&[0x2e]))?;
    Ok(equal(a1225, a1226))
}

pub fn f255() -> List<Bytes> {
    List::cons(f3(Str::lit("id")), List::cons(f3(Str::lit("channel")), List::cons(f3(Str::lit("surface")), List::cons(f3(Str::lit("canonical_source")), List::cons(f3(Str::lit("features")), List::<Bytes>::nil())))))
}

pub fn f256(v0: List<(Bytes, Adt17)>, v1: Bytes) -> Option<bool> {
    let m1227 = f91(v0, v1);
    match m1227 {
        None => {
            None::<bool>
        }
        Some(v2) => {
            let m1228 = v2;
            match m1228 {
                Adt17::C0(_) => {
                    None::<bool>
                }
                Adt17::C1(v4) => {
                    Some(v4)
                }
                Adt17::C2(_) => {
                    None::<bool>
                }
                Adt17::C3(_) => {
                    None::<bool>
                }
            }
        }
    }
}

pub fn f257(v0: List<(Bytes, Adt17)>) -> R<Option<Adt23>> {
    if f27(v0.clone(), f317())? {
        let m1229 = f29(v0.clone(), f3(Str::lit("entry")));
        match m1229 {
            None => {
                Ok(None::<Adt23>)
            }
            Some(v1) => {
                let m1230 = f29(v0.clone(), f3(Str::lit("lean_name")));
                match m1230 {
                    None => {
                        Ok(None::<Adt23>)
                    }
                    Some(v2) => {
                        let m1231 = f177(v0.clone(), f3(Str::lit("fields")));
                        match m1231 {
                            None => {
                                Ok(None::<Adt23>)
                            }
                            Some(v3) => {
                                let m1232 = f177(v0, f3(Str::lit("induction_hypotheses")));
                                match m1232 {
                                    None => {
                                        Ok(None::<Adt23>)
                                    }
                                    Some(v4) => {
                                        Ok(Some(Adt23::C0(v1, v2, v3, v4)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt23>)
    }
}

pub fn f258(v0: Fn43, p1236: List<Adt23>, p1237: List<Adt23>) -> List<Adt23> {
    let m1235 = (p1236, p1237);
    {
        let mut m1235 = m1235;
        loop {
            let step = {
                let (v1, v2) = m1235;
                let m1233 = v2;
                match m1233.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>(({
                            let c1234: Fn43 = v0.clone();
                            c1234.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Adt23>, v1: Adt23) -> List<Adt23> {
    List::cons(v1, v0)
}

pub fn f260(v0: Bytes, v1: u64, v2: bool, v3: Adt32) -> R<Option<Adt32>> {
    if match v3.clone() {
        Adt32::C0(_, _, _, h1238) => {
            h1238
        }
    } {
        let a1240 = match v3.clone() {
            Adt32::C0(h1239, _, _, _) => {
                h1239
            }
        };
        let a1241 = v1;
        if nat_lt(a1240, a1241) {
            let v4: u64 = f264(v0, match v3.clone() {
                Adt32::C0(h1242, _, _, _) => {
                    h1242
                }
            })?;
            let a1243 = v4;
            let a1244 = 46u64;
            if nat_eq(a1243, a1244) {
                if match v3.clone() {
                    Adt32::C0(_, h1245, _, _) => {
                        h1245
                    }
                } {
                    Ok(f318(v3))
                } else {
                    f319(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt32::C0(_, h1246, _, _) => {
                        h1246
                    }
                } {
                    if f320(v4) {
                        f319(v3.clone(), false, match v3 {
                            Adt32::C0(_, _, h1247, _) => {
                                h1247
                            }
                        })
                    } else {
                        Ok(f318(v3))
                    }
                } else {
                    if if f320(v4) {
                        true
                    } else {
                        f321(v4)
                    } {
                        f319(v3.clone(), false, match v3 {
                            Adt32::C0(_, _, h1248, _) => {
                                h1248
                            }
                        })
                    } else {
                        let a1249 = v4;
                        let a1250 = 45u64;
                        if if nat_eq(a1249, a1250) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt32::C0(_, _, h1251, _) => {
                                        h1251
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f319(v3.clone(), false, match v3 {
                                Adt32::C0(_, _, h1252, _) => {
                                    h1252
                                }
                            })
                        } else {
                            Ok(f318(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt32>)
        }
    } else {
        Ok(None::<Adt32>)
    }
}

pub fn f261(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1253, _) = v2;
        h1253
    };
    let a1254 = v3;
    let a1255 = 0u64;
    if nat_eq(a1254, a1255) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1256 = v3;
            let a1257 = v1;
            let a1258 = 0u64;
            nat_quot(a1256, a1257, a1258)
        }, {
            let a1263 = f189(v0, {
                let a1259 = v3;
                let a1260 = v1;
                let a1261 = 0u64;
                nat_rem(a1259, a1260, a1261)
            });
            let a1264 = {
                let (_, h1262) = v2;
                h1262
            };
            append_bytes(a1263, a1264)
        }))
    }
}

pub fn f262(v0: Bytes, v1: Bytes) -> bool {
    let a1269 = {
        let a1265 = v0;
        let a1266 = v1;
        compare_bytes(a1265, a1266)
    };
    let a1270 = {
        let a1267 = Bytes::lit(&[]);
        let a1268 = Bytes::lit(&[0x01]);
        compare_bytes(a1267, a1268)
    };
    equal(a1269, a1270)
}

pub fn f263(v0: u64) -> bool {
    if f320(v0) {
        true
    } else {
        let a1271 = v0;
        let a1272 = 65u64;
        if nat_lt(a1271, a1272) {
            false
        } else {
            let a1273 = v0;
            let a1274 = 90u64;
            nat_le(a1273, a1274)
        }
    }
}

pub fn f264(v0: Bytes, v1: u64) -> R<u64> {
    let m1275 = f247(v0, v1);
    match m1275 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1320 = {
                let a1296 = {
                    let a1284 = {
                        let a1278 = {
                            let a1276 = v3;
                            let a1277 = 128u8;
                            fixed_u8::bit_and(a1276, a1277)
                        };
                        let a1279 = 128u8;
                        if equal(a1278, a1279) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1285 = {
                        let a1282 = {
                            let a1280 = v3;
                            let a1281 = 64u8;
                            fixed_u8::bit_and(a1280, a1281)
                        };
                        let a1283 = 64u8;
                        if equal(a1282, a1283) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1284, a1285)?
                };
                let a1297 = {
                    let a1294 = {
                        let a1288 = {
                            let a1286 = v3;
                            let a1287 = 32u8;
                            fixed_u8::bit_and(a1286, a1287)
                        };
                        let a1289 = 32u8;
                        if equal(a1288, a1289) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1295 = {
                        let a1292 = {
                            let a1290 = v3;
                            let a1291 = 16u8;
                            fixed_u8::bit_and(a1290, a1291)
                        };
                        let a1293 = 16u8;
                        if equal(a1292, a1293) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1294, a1295)?
                };
                nat_add(a1296, a1297)?
            };
            let a1321 = {
                let a1318 = {
                    let a1306 = {
                        let a1300 = {
                            let a1298 = v3;
                            let a1299 = 8u8;
                            fixed_u8::bit_and(a1298, a1299)
                        };
                        let a1301 = 8u8;
                        if equal(a1300, a1301) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1307 = {
                        let a1304 = {
                            let a1302 = v3;
                            let a1303 = 4u8;
                            fixed_u8::bit_and(a1302, a1303)
                        };
                        let a1305 = 4u8;
                        if equal(a1304, a1305) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1306, a1307)?
                };
                let a1319 = {
                    let a1316 = {
                        let a1310 = {
                            let a1308 = v3;
                            let a1309 = 2u8;
                            fixed_u8::bit_and(a1308, a1309)
                        };
                        let a1311 = 2u8;
                        if equal(a1310, a1311) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1317 = {
                        let a1314 = {
                            let a1312 = v3;
                            let a1313 = 1u8;
                            fixed_u8::bit_and(a1312, a1313)
                        };
                        let a1315 = 1u8;
                        if equal(a1314, a1315) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1316, a1317)?
                };
                nat_add(a1318, a1319)?
            };
            nat_add(a1320, a1321)
        }
    }
}

pub fn f265(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f322(v1, v0, v2)
}

pub fn f266(v0: Bytes, v1: u64, v2: Adt34) -> R<Option<Adt34>> {
    let m1323 = match v2.clone() {
        Adt34::C0(_, _, _, _, h1322) => {
            h1322
        }
    };
    match m1323 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt34::C0(h1324, _, _, _, _) => {
                    h1324
                }
            };
            let a1325 = v4;
            let a1326 = v1;
            if nat_lt(a1325, a1326) {
                let v5: u64 = f264(v0.clone(), v4)?;
                let a1327 = v5;
                let a1328 = 40u64;
                if nat_eq(a1327, a1328) {
                    Ok(Some(Adt34::C0({
                        let a1329 = v4;
                        let a1330 = 1u64;
                        nat_add(a1329, a1330)?
                    }, match v2.clone() {
                        Adt34::C0(_, h1331, _, _, _) => {
                            h1331
                        }
                    }, false, List::cons(Adt33::C0, f197(v0, v2)), None::<Adt28>)))
                } else {
                    let a1332 = v5;
                    let a1333 = 41u64;
                    if nat_eq(a1332, a1333) {
                        Ok(Some(Adt34::C0({
                            let a1334 = v4;
                            let a1335 = 1u64;
                            nat_add(a1334, a1335)?
                        }, match v2.clone() {
                            Adt34::C0(_, h1336, _, _, _) => {
                                h1336
                            }
                        }, false, List::cons(Adt33::C1, f197(v0, v2)), None::<Adt28>)))
                    } else {
                        let a1337 = v5;
                        let a1338 = 32u64;
                        if if nat_eq(a1337, a1338) {
                            true
                        } else {
                            let a1339 = v5;
                            let a1340 = 10u64;
                            nat_eq(a1339, a1340)
                        } {
                            Ok(Some(Adt34::C0({
                                let a1341 = v4;
                                let a1342 = 1u64;
                                nat_add(a1341, a1342)?
                            }, match v2.clone() {
                                Adt34::C0(_, h1343, _, _, _) => {
                                    h1343
                                }
                            }, false, f197(v0, v2), None::<Adt28>)))
                        } else {
                            let a1344 = v5;
                            let a1345 = 32u64;
                            if if nat_lt(a1344, a1345) {
                                true
                            } else {
                                let a1346 = v5;
                                let a1347 = 127u64;
                                nat_eq(a1346, a1347)
                            } {
                                Ok(Some(Adt34::C0(v4, match v2.clone() {
                                    Adt34::C0(_, h1348, _, _, _) => {
                                        h1348
                                    }
                                }, match v2.clone() {
                                    Adt34::C0(_, _, h1349, _, _) => {
                                        h1349
                                    }
                                }, match v2 {
                                    Adt34::C0(_, _, _, h1350, _) => {
                                        h1350
                                    }
                                }, Some(f200(f201(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt34::C0(_, _, h1351, _, _) => {
                                        h1351
                                    }
                                } {
                                    Ok(Some(Adt34::C0({
                                        let a1352 = v4;
                                        let a1353 = 1u64;
                                        nat_add(a1352, a1353)?
                                    }, match v2.clone() {
                                        Adt34::C0(_, h1354, _, _, _) => {
                                            h1354
                                        }
                                    }, true, match v2 {
                                        Adt34::C0(_, _, _, h1355, _) => {
                                            h1355
                                        }
                                    }, None::<Adt28>)))
                                } else {
                                    Ok(Some(Adt34::C0({
                                        let a1356 = v4;
                                        let a1357 = 1u64;
                                        nat_add(a1356, a1357)?
                                    }, v4, true, match v2 {
                                        Adt34::C0(_, _, _, h1358, _) => {
                                            h1358
                                        }
                                    }, None::<Adt28>)))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Adt34>)
            }
        }
        Some(_) => {
            Ok(None::<Adt34>)
        }
    }
}

pub fn f267(v0: Fn44, p1362: List<Adt33>, p1363: List<Adt33>) -> List<Adt33> {
    let m1361 = (p1362, p1363);
    {
        let mut m1361 = m1361;
        loop {
            let step = {
                let (v1, v2) = m1361;
                let m1359 = v2;
                match m1359.uncons() {
                    None => {
                        Err::<(List<Adt33>, List<Adt33>), List<Adt33>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt33>, List<Adt33>), List<Adt33>>(({
                            let c1360: Fn44 = v0.clone();
                            c1360.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: List<Adt33>, v1: Adt33) -> List<Adt33> {
    List::cons(v1, v0)
}

pub fn f269(v0: List<Adt33>, v1: u64, v2: Adt36) -> R<Option<Adt36>> {
    let m1365 = match v2.clone() {
        Adt36::C0(_, _, _, h1364, _) => {
            h1364
        }
    };
    match m1365 {
        None => {
            if match v2.clone() {
                Adt36::C0(_, _, _, _, h1366) => {
                    h1366
                }
            } {
                Ok(None::<Adt36>)
            } else {
                let m1370 = {
                    let a1368 = v0;
                    let a1369 = match v2.clone() {
                        Adt36::C0(h1367, _, _, _, _) => {
                            h1367
                        }
                    };
                    index_list(a1368, a1369)
                };
                match m1370 {
                    None => {
                        let m1372 = match v2.clone() {
                            Adt36::C0(_, h1371, _, _, _) => {
                                h1371
                            }
                        };
                        match m1372.uncons() {
                            None => {
                                let m1374 = match v2.clone() {
                                    Adt36::C0(_, _, h1373, _, _) => {
                                        h1373
                                    }
                                };
                                match m1374 {
                                    None => {
                                        Ok(Some(f323(v2, f200(f201(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt36::C0(match v2.clone() {
                                            Adt36::C0(h1375, _, _, _, _) => {
                                                h1375
                                            }
                                        }, List::<List<Adt35>>::nil(), match v2 {
                                            Adt36::C0(_, _, h1376, _, _) => {
                                                h1376
                                            }
                                        }, None::<Adt28>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f323(v2, f200(f201(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m1378 = match v2.clone() {
                            Adt36::C0(_, _, h1377, _, _) => {
                                h1377
                            }
                        };
                        match m1378 {
                            None => {
                                Ok(Some(f324(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f323(v2, f200(f201(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt36>)
        }
    }
}

pub fn f270(v0: u64, v1: Adt37) -> R<Option<Adt37>> {
    let m1380 = match v1.clone() {
        Adt37::C0(_, _, h1379) => {
            h1379
        }
    };
    match m1380 {
        None => {
            let m1382 = match v1.clone() {
                Adt37::C0(h1381, _, _) => {
                    h1381
                }
            };
            match m1382.uncons() {
                None => {
                    Ok(None::<Adt37>)
                }
                Some((v3, v4)) => {
                    let m1383 = v3;
                    match m1383 {
                        Adt38::C0(v5) => {
                            Ok(Some(f325(v1, v4, v5, v0)?))
                        }
                        Adt38::C1(v6) => {
                            Ok(Some(f326(v1, v4, v6, v0)?))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt37>)
        }
    }
}

pub fn f271(v0: Adt29, v1: List<Adt30>, v2: Adt1) -> R<Option<Adt29>> {
    let m1384 = v2;
    match m1384 {
        Adt1::C0 => {
            Ok(f327(v0, v1))
        }
        Adt1::C1(v3) => {
            Ok(f327(v0, List::cons(Adt30::C3(v3), v1)))
        }
        Adt1::C2(_, _, v6) => {
            Ok(f327(v0, List::cons(Adt30::C3(v6), v1)))
        }
        Adt1::C3(v7) => {
            if f328(match v0.clone() {
                Adt29::C0(_, h1385, _, _) => {
                    h1385
                }
            }, v7.clone())? {
                Ok(f327(v0, v1))
            } else {
                Ok(f329(v0, f330(f330(f331(Str::lit("local `")), v7), f331(Str::lit("` is not in scope")))))
            }
        }
        Adt1::C4(r1389, r1390) => {
            let v8 = (*r1389).clone();
            let v9 = (*r1390).clone();
            Ok(f327(v0, List::cons(Adt30::C0(v8), List::cons(Adt30::C0(v9), v1))))
        }
        Adt1::C5(r1391, r1392) => {
            let v10 = (*r1391).clone();
            let v11 = (*r1392).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(v10), List::cons(Adt30::C0(v11), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1386, _) => {
                    h1386
                }
            }), v1)))))
        }
        Adt1::C6(r1393, r1394) => {
            let v12 = (*r1393).clone();
            let v13 = (*r1394).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(v12), List::cons(Adt30::C0(v13), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1387, _) => {
                    h1387
                }
            }), v1)))))
        }
        Adt1::C7(v14, r1395, r1396, r1397) => {
            let v15 = (*r1395).clone();
            let v16 = (*r1396).clone();
            let v17 = (*r1397).clone();
            Ok(f327(v0.clone(), List::cons(Adt30::C0(v15), List::cons(Adt30::C0(v16), List::cons(Adt30::C1(v14), List::cons(Adt30::C0(v17), List::cons(Adt30::C2(match v0 {
                Adt29::C0(_, _, h1388, _) => {
                    h1388
                }
            }), v1)))))))
        }
        Adt1::C8(_) => {
            Ok(f327(v0, v1))
        }
        Adt1::C9 => {
            Ok(f327(v0, v1))
        }
        Adt1::C10(r1398, r1399) => {
            let v19 = (*r1398).clone();
            let v20 = (*r1399).clone();
            Ok(f327(v0, List::cons(Adt30::C0(v19), List::cons(Adt30::C0(v20), v1))))
        }
        Adt1::C11(_, v22, r1400, r1401) => {
            let v23 = (*r1400).clone();
            let v24 = (*r1401).clone();
            Ok(f327(v0, List::cons(Adt30::C0(v23), List::cons(Adt30::C1(v22), List::cons(Adt30::C0(v24), v1)))))
        }
    }
}

pub fn f272(v0: List<Bytes>, v1: u64) -> List<Bytes> {
    let (h1402, _) = f332(Fn52::F333, v1, v0);
    h1402
}

pub fn f273(v0: Adt29, v1: List<Adt30>, v2: List<Bytes>, v3: Adt2) -> R<Option<Adt29>> {
    let m1403 = v3;
    match m1403 {
        Adt2::C0(_) => {
            Ok(f327(v0, v1))
        }
        Adt2::C1(v5) => {
            if f328(v2, v5.clone())? {
                Ok(f327(v0, v1))
            } else {
                Ok(f329(v0, f330(f330(f331(Str::lit("universe variable `")), v5), f331(Str::lit("` is not declared")))))
            }
        }
        Adt2::C2(r1404) => {
            let v6 = (*r1404).clone();
            Ok(f327(v0, List::cons(Adt30::C3(v6), v1)))
        }
        Adt2::C3(r1405) => {
            let v7 = (*r1405).clone();
            Ok(f327(v0, List::cons(Adt30::C3(v7), v1)))
        }
        Adt2::C4(r1406, r1407) => {
            let v8 = (*r1406).clone();
            let v9 = (*r1407).clone();
            Ok(f327(v0, List::cons(Adt30::C3(v8), List::cons(Adt30::C3(v9), v1))))
        }
        Adt2::C5 => {
            Ok(f327(v0, v1))
        }
        Adt2::C6(r1408, r1409) => {
            let v10 = (*r1408).clone();
            let v11 = (*r1409).clone();
            Ok(f327(v0, List::cons(Adt30::C3(v10), List::cons(Adt30::C3(v11), v1))))
        }
    }
}

pub fn f274(v0: Adt2) -> Bytes {
    let m1410 = v0;
    match m1410 {
        Adt2::C0(_) => {
            Bytes::lit(&[])
        }
        Adt2::C1(_) => {
            Bytes::lit(&[])
        }
        Adt2::C2(_) => {
            Bytes::lit(&[])
        }
        Adt2::C3(_) => {
            Bytes::lit(&[])
        }
        Adt2::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt2::C5 => {
            Bytes::lit(&[])
        }
        Adt2::C6(r1411, r1412) => {
            let v7 = (*r1411).clone();
            let v8 = (*r1412).clone();
            f205(f205(f201(Str::lit(" ")), f206(v7)), f274(v8))
        }
    }
}

pub fn f275(v0: Fn45, p1416: Option<Bytes>, p1417: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m1415 = (p1416, p1417);
    {
        let mut m1415 = m1415;
        loop {
            let step = {
                let (v1, v2) = m1415;
                let m1413 = v2;
                match m1413.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c1414: Fn45 = v0.clone();
                            c1414.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m1418 = v1.clone();
    match m1418 {
        None => {
            let a1420 = {
                let (h1419, _) = v2;
                h1419
            };
            let a1421 = v0;
            if equal(a1420, a1421) {
                Some({
                    let (_, h1422) = v2;
                    h1422
                })
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f277(v0: u64) -> Bytes {
    let a1423 = v0;
    let a1424 = 1u64;
    if nat_eq(a1423, a1424) {
        f201(Str::lit("implicit"))
    } else {
        let a1425 = v0;
        let a1426 = 2u64;
        if nat_eq(a1425, a1426) {
            f201(Str::lit("instance"))
        } else {
            f201(Str::lit("explicit"))
        }
    }
}

pub fn f278(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt39>)> {
    f334(Fn53::F335(v0, v1), v2, (0u64, List::<Adt39>::nil()))
}

pub fn f279(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt39> {
    let v4: u64 = {
        let a1427 = 1024u64;
        let a1428 = v2;
        nat_mul(a1427, a1428)?
    };
    let v5: u64 = f336(v1, v4);
    let v6: u64 = f337(v5)?;
    let v7: u64 = {
        let a1429 = v6;
        let a1430 = 1u64;
        nat_sub(a1429, a1430)
    };
    Ok(f338(f339(v0.clone(), v4, v2, v7)?, f340(v0, {
        let a1433 = v4;
        let a1434 = {
            let a1431 = 64u64;
            let a1432 = v7;
            nat_mul(a1431, a1432)?
        };
        nat_add(a1433, a1434)?
    })?, v2, f341({
        let a1437 = v5;
        let a1438 = {
            let a1435 = 64u64;
            let a1436 = v7;
            nat_mul(a1435, a1436)?
        };
        nat_sub(a1437, a1438)
    }), {
        let a1441 = {
            let a1439 = v7;
            let a1440 = 0u64;
            if nat_eq(a1439, a1440) {
                3u32
            } else {
                2u32
            }
        };
        let a1442 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a1441, a1442)
    }))
}

pub fn f280(v0: Fn46, p1446: u64, p1447: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    let m1445 = (p1446, p1447);
    {
        let mut m1445 = m1445;
        loop {
            let step = {
                let (v1, v2) = m1445;
                let m1443 = v1;
                if m1443 == 0 {
                    Err::<(u64, (List<Adt39>, Adt39)), (List<Adt39>, Adt39)>(v2)
                } else {
                    let v3 = m1443 - 1;
                    Ok::<(u64, (List<Adt39>, Adt39)), (List<Adt39>, Adt39)>((v3, {
                        let c1444: Fn46 = v0.clone();
                        c1444.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    f342(v0)
}

pub fn f282(v0: u32, v1: u32) -> Bytes {
    f343(f344(v0, v1))
}

pub fn f283(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1448 = v2;
        let a1449 = v0;
        equal(a1448, a1449)
    }
}

pub fn f284(v0: Bytes) -> R<bool> {
    f345(v0)
}

pub fn f285(v0: Bytes, v1: Bytes, v2: Bytes, v3: List<(Bytes, Bytes)>) -> R<Adt42> {
    let m1450 = f7(2u64, v1)?;
    match m1450 {
        None => {
            Ok(Adt42::C0(f46(v3, Some(f2(f3(Str::lit("a form channel is text, math, or both"))))), None::<u64>, List::<Adt5>::nil()))
        }
        Some(v4) => {
            if f346(v2.clone())? {
                Ok(Adt42::C0(f46(v3, Some(f2(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: leading, trailing, or repeated whitespace in the surface is forbidden")))))), None::<u64>, List::<Adt5>::nil()))
            } else {
                let m1452 = f347(f3(Str::lit("<form>")), v2.clone(), {
                    let a1451 = v2;
                    length_bytes(a1451)
                })?;
                match m1452 {
                    Ok(v6) => {
                        Ok(Adt42::C0(v3, Some(v4), v6))
                    }
                    Err(v5) => {
                        Ok(Adt42::C0(f46(v3, Some(f2(f5(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: surface does not scan: "))), match v5 {
                            Adt48::C0(_, h1453, _, _) => {
                                h1453
                            }
                        })))), None::<u64>, List::<Adt5>::nil()))
                    }
                }
            }
        }
    }
}

pub fn f286(v0: List<Bytes>, v1: Bytes, v2: List<Adt5>, v3: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f348(Fn54::F349(v0, v1), v3, v2)
}

pub fn f287(v0: List<Adt5>) -> bool {
    f350(Fn55::F351, false, v0)
}

pub fn f288(v0: List<Adt5>, v1: u64) -> R<Option<Bytes>> {
    let (_, h1454) = f352(Fn56::F353(v1), (0u64, None::<Bytes>), v0)?;
    Ok(h1454)
}

pub fn f289(v0: Bytes) -> (Bytes, Bytes) {
    (f3(Str::lit("LLR3006")), v0)
}

pub fn f290(v0: Bytes, v1: List<Bytes>, v2: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v3: List<(Bytes, Bytes)> = if f49(v1.clone()) {
        v2
    } else {
        List::cons(f2(f5(f5(f3(Str::lit("form `")), v0.clone()), f3(Str::lit("`: features must be sorted and unique")))), v2)
    };
    f228(Fn37::F354(v0), v3, v1)
}

pub fn f291(v0: Fn47, p1459: u64, p1460: Adt43) -> R<(Adt43, bool)> {
    let m1458 = (p1459, p1460);
    {
        let mut m1458 = m1458;
        loop {
            let step = {
                let (v1, v2) = m1458;
                let m1455 = v1;
                if m1455 == 0 {
                    Err::<(u64, Adt43), R<(Adt43, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1455 - 1;
                    let m1457 = {
                        let c1456: Fn47 = v0.clone();
                        c1456.apply(v2.clone())?
                    };
                    match m1457 {
                        None => {
                            Err::<(u64, Adt43), R<(Adt43, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt43), R<(Adt43, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: u64, v1: Adt43) -> R<Option<Adt43>> {
    f355(v0, v1)
}

pub fn f293(v0: u64, v1: Adt40) -> R<Option<Adt40>> {
    let m1462 = match v1.clone() {
        Adt40::C0(h1461, _) => {
            h1461
        }
    };
    match m1462.uncons() {
        None => {
            Ok(None::<Adt40>)
        }
        Some((v2, v3)) => {
            let v4: Adt31 = match v1 {
                Adt40::C0(_, h1463) => {
                    h1463
                }
            };
            let m1464 = v2;
            match m1464 {
                Adt7::C0(v5, v6, v7) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1465, _, _, _, _, _, _) => {
                            h1465
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1466, _, _, _, _, _) => {
                            h1466
                        }
                    }, List::cons((v5, (v6, v7)), match v4.clone() {
                        Adt31::C0(_, _, h1467, _, _, _, _) => {
                            h1467
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, _, h1468, _, _, _) => {
                            h1468
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1469, _, _) => {
                            h1469
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1470, _) => {
                            h1470
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1471) => {
                            h1471
                        }
                    })))
                }
                Adt7::C1(v8) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1472, _, _, _, _, _, _) => {
                            h1472
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1473, _, _, _, _, _) => {
                            h1473
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1474, _, _, _, _) => {
                            h1474
                        }
                    }, List::cons(v8, match v4.clone() {
                        Adt31::C0(_, _, _, h1475, _, _, _) => {
                            h1475
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, _, _, h1476, _, _) => {
                            h1476
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1477, _) => {
                            h1477
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1478) => {
                            h1478
                        }
                    })))
                }
                Adt7::C2(v9) => {
                    Ok(f356(v3, Adt31::C0(List::cons(v9, match v4.clone() {
                        Adt31::C0(h1479, _, _, _, _, _, _) => {
                            h1479
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, h1480, _, _, _, _, _) => {
                            h1480
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1481, _, _, _, _) => {
                            h1481
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1482, _, _, _) => {
                            h1482
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1483, _, _) => {
                            h1483
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1484, _) => {
                            h1484
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1485) => {
                            h1485
                        }
                    })))
                }
                Adt7::C3(r1500) => {
                    let v10 = (*r1500).clone();
                    Ok(f356(f357(Fn57::F358, v3, f359(f360(v10, v0)?)), v4))
                }
                Adt7::C4 => {
                    Ok(f356(v3, v4))
                }
                Adt7::C5(v11) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1486, _, _, _, _, _, _) => {
                            h1486
                        }
                    }, List::cons(v11, match v4.clone() {
                        Adt31::C0(_, h1487, _, _, _, _, _) => {
                            h1487
                        }
                    }), match v4.clone() {
                        Adt31::C0(_, _, h1488, _, _, _, _) => {
                            h1488
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1489, _, _, _) => {
                            h1489
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1490, _, _) => {
                            h1490
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1491, _) => {
                            h1491
                        }
                    }, match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1492) => {
                            h1492
                        }
                    })))
                }
                Adt7::C6(r1501) => {
                    let v12 = (*r1501).clone();
                    Ok(f356(List::cons(v12, v3), v4))
                }
                Adt7::C7(r1502) => {
                    let v13 = (*r1502).clone();
                    Ok(f356(List::cons(v13, v3), v4))
                }
                Adt7::C8(r1503) => {
                    let v14 = (*r1503).clone();
                    Ok(f356(List::cons(v14, v3), v4))
                }
                Adt7::C9(r1504, r1505) => {
                    let v15 = (*r1504).clone();
                    let v16 = (*r1505).clone();
                    Ok(f356(List::cons(v15.clone(), List::cons(v16.clone(), v3)), f361(v4, f201(Str::lit("sub")), v15, v16, v0)?))
                }
                Adt7::C10(r1506, r1507) => {
                    let v17 = (*r1506).clone();
                    let v18 = (*r1507).clone();
                    Ok(f356(List::cons(v17.clone(), List::cons(v18.clone(), v3)), f361(v4, f201(Str::lit("sup")), v17, v18, v0)?))
                }
                Adt7::C11(r1508, r1509) => {
                    let v19 = (*r1508).clone();
                    let v20 = (*r1509).clone();
                    Ok(f356(List::cons(v19.clone(), List::cons(v20.clone(), v3)), f361(v4, f201(Str::lit("frac")), v19, v20, v0)?))
                }
                Adt7::C12(v21) => {
                    Ok(f356(v3, Adt31::C0(match v4.clone() {
                        Adt31::C0(h1493, _, _, _, _, _, _) => {
                            h1493
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, h1494, _, _, _, _, _) => {
                            h1494
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, h1495, _, _, _, _) => {
                            h1495
                        }
                    }, match v4.clone() {
                        Adt31::C0(_, _, _, h1496, _, _, _) => {
                            h1496
                        }
                    }, List::cons(v21, match v4.clone() {
                        Adt31::C0(_, _, _, _, h1497, _, _) => {
                            h1497
                        }
                    }), f362(match v4.clone() {
                        Adt31::C0(_, _, _, _, _, h1498, _) => {
                            h1498
                        }
                    }, f201(Str::lit("operator-name"))), match v4 {
                        Adt31::C0(_, _, _, _, _, _, h1499) => {
                            h1499
                        }
                    })))
                }
                Adt7::C13 => {
                    Ok(f356(v3, v4))
                }
                Adt7::C14(_, _) => {
                    Ok(f356(v3, v4))
                }
            }
        }
    }
}

pub fn f294(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f295(v0: Fn48, p1513: List<(Bytes, (Bytes, Bytes))>, p1514: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1512 = (p1513, p1514);
    {
        let mut m1512 = m1512;
        loop {
            let step = {
                let (v1, v2) = m1512;
                let m1510 = v2;
                match m1510.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c1511: Fn48 = v0.clone();
                            c1511.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(v1, v0)
}

pub fn f297(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: (bool, List<u64>) = f363(Fn58::F364(v1), (false, List::<u64>::nil()), v0);
    let m1516 = {
        let (h1515, _) = v2;
        h1515
    };
    if m1516 {
        f220({
            let (_, h1517) = v2;
            h1517
        })
    } else {
        f220(List::cons(v1, {
            let (_, h1518) = v2;
            h1518
        }))
    }
}

pub fn f298(v0: List<Adt4>, v1: u64, v2: Bytes, v3: Bytes, v4: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let m1519 = f365(v0, v3.clone());
    match m1519 {
        None => {
            Ok(List::cons(f2(f5(f5(f5(v2, f3(Str::lit(" render references unknown self form "))), f6(v3)), Bytes::lit(&[]))), v4))
        }
        Some(v5) => {
            let v6: List<(Bytes, Bytes)> = f46(v4, if f216(match v5.clone() {
                Adt4::C0(_, h1520, _, _, _, _) => {
                    h1520
                }
            }, v1) {
                None::<(Bytes, Bytes)>
            } else {
                Some(f2(f5(f5(f5(v2.clone(), f3(Str::lit(" render references self form "))), f6(v3.clone())), f5(f3(Str::lit(", which is not a ")), f5(v2.clone(), f3(Str::lit(" form")))))))
            });
            let m1523 = f288(match v5.clone() {
                Adt4::C0(_, _, _, h1521, _, _) => {
                    h1521
                }
            }, match v5.clone() {
                Adt4::C0(_, h1522, _, _, _, _) => {
                    h1522
                }
            })?;
            match m1523 {
                None => {
                    Ok(v6)
                }
                Some(v7) => {
                    Ok(f46(v6, Some(f289(f5(f5(f5(f5(f5(v2, f3(Str::lit(" render references self form "))), f6(v3)), f3(Str::lit(" whose surface `"))), match v5 {
                        Adt4::C0(_, _, h1524, _, _, _) => {
                            h1524
                        }
                    }), f5(f3(Str::lit("` is not renderer-safe: ")), v7))))))
                }
            }
        }
    }
}

pub fn f299(v0: u64) -> bool {
    if f263(v0) {
        true
    } else {
        let a1525 = v0;
        let a1526 = 95u64;
        nat_eq(a1525, a1526)
    }
}

pub fn f300(v0: u64) -> bool {
    if f366(v0) {
        true
    } else {
        let a1527 = v0;
        let a1528 = 95u64;
        nat_eq(a1527, a1528)
    }
}

pub fn f301(v0: Bytes) -> Bytes {
    f205(f205(f201(Str::lit("`")), v0), f201(Str::lit("`")))
}

pub fn f302(v0: Bytes) -> R<bool> {
    f112(v0, false)
}

pub fn f303(v0: Fn49, p1532: (List<(Bytes, Bytes)>, List<Bytes>), p1533: List<Bytes>) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    let m1531 = (p1532, p1533);
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let (v1, v2) = m1531;
                let m1529 = v2;
                match m1529.uncons() {
                    None => {
                        Err::<((List<(Bytes, Bytes)>, List<Bytes>), List<Bytes>), R<(List<(Bytes, Bytes)>, List<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<(Bytes, Bytes)>, List<Bytes>), List<Bytes>), R<(List<(Bytes, Bytes)>, List<Bytes>)>>(({
                            let c1530: Fn49 = v0.clone();
                            c1530.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: (List<(Bytes, Bytes)>, List<Bytes>), v1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    f367(v0, v1)
}

pub fn f305(v0: Bytes, v1: u64) -> Option<u64> {
    let a1534 = 0u64;
    let a1535 = v1;
    if nat_lt(a1534, a1535) {
        if f368(v0, {
            let a1536 = v1;
            let a1537 = 1u64;
            nat_sub(a1536, a1537)
        }) {
            Some({
                let a1538 = v1;
                let a1539 = 1u64;
                nat_sub(a1538, a1539)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f306(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1540 = v2;
    let a1541 = v0;
    if nat_lt(a1540, a1541) {
        if f368(v1, v2) {
            Ok(Some({
                let a1542 = v2;
                let a1543 = 1u64;
                nat_add(a1542, a1543)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f307(v0: Bytes, v1: u64, v2: Adt41) -> R<Option<Adt41>> {
    let a1545 = match v2.clone() {
        Adt41::C0(_, _, h1544) => {
            h1544
        }
    };
    let a1546 = 0u64;
    if equal(a1545, a1546) {
        let v3: u64 = f369(v0.clone(), match v2.clone() {
            Adt41::C0(h1547, _, _) => {
                h1547
            }
        })?;
        let a1548 = v3;
        let a1549 = v1;
        if nat_lt(a1548, a1549) {
            let m1550 = f247(v0.clone(), v3);
            match m1550 {
                None => {
                    Ok(None::<Adt41>)
                }
                Some(v4) => {
                    let a1551 = v4;
                    let a1552 = 34u8;
                    if equal(a1551, a1552) {
                        let m1553 = f245(v0.clone(), v3)?;
                        match m1553 {
                            None => {
                                Ok(Some(Adt41::C0(v3, match v2 {
                                    Adt41::C0(_, h1568, _) => {
                                        h1568
                                    }
                                }, 2u64)))
                            }
                            Some(v5) => {
                                let v6: u64 = f369(v0.clone(), {
                                    let (_, h1554) = v5;
                                    h1554
                                })?;
                                let a1555 = v6;
                                let a1556 = v1;
                                if nat_lt(a1555, a1556) {
                                    let m1557 = f247(v0.clone(), v6);
                                    match m1557 {
                                        None => {
                                            Ok(None::<Adt41>)
                                        }
                                        Some(v7) => {
                                            let a1558 = v7;
                                            let a1559 = 44u8;
                                            if equal(a1558, a1559) {
                                                Ok(Some(Adt41::C0(f369(v0, {
                                                    let a1560 = v6;
                                                    let a1561 = 1u64;
                                                    nat_add(a1560, a1561)?
                                                })?, List::cons({
                                                    let (h1562, _) = v5;
                                                    h1562
                                                }, match v2 {
                                                    Adt41::C0(_, h1563, _) => {
                                                        h1563
                                                    }
                                                }), 0u64)))
                                            } else {
                                                Ok(Some(Adt41::C0(v6, List::cons({
                                                    let (h1564, _) = v5;
                                                    h1564
                                                }, match v2 {
                                                    Adt41::C0(_, h1565, _) => {
                                                        h1565
                                                    }
                                                }), 2u64)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok(Some(Adt41::C0(v6, List::cons({
                                        let (h1566, _) = v5;
                                        h1566
                                    }, match v2 {
                                        Adt41::C0(_, h1567, _) => {
                                            h1567
                                        }
                                    }), 1u64)))
                                }
                            }
                        }
                    } else {
                        Ok(Some(Adt41::C0(v3, match v2 {
                            Adt41::C0(_, h1569, _) => {
                                h1569
                            }
                        }, 2u64)))
                    }
                }
            }
        } else {
            Ok(Some(Adt41::C0(v3, match v2 {
                Adt41::C0(_, h1570, _) => {
                    h1570
                }
            }, 1u64)))
        }
    } else {
        Ok(None::<Adt41>)
    }
}

pub fn f308(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f370(v1, v0, v2)
}

pub fn f309(v0: Fn50, p1575: u64, p1576: Adt45) -> R<(Adt45, bool)> {
    let m1574 = (p1575, p1576);
    {
        let mut m1574 = m1574;
        loop {
            let step = {
                let (v1, v2) = m1574;
                let m1571 = v1;
                if m1571 == 0 {
                    Err::<(u64, Adt45), R<(Adt45, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1571 - 1;
                    let m1573 = {
                        let c1572: Fn50 = v0.clone();
                        c1572.apply(v2.clone())?
                    };
                    match m1573 {
                        None => {
                            Err::<(u64, Adt45), R<(Adt45, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt45), R<(Adt45, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: u64, v2: Adt45) -> R<Option<Adt45>> {
    f371(v0, v1, v2)
}

pub fn f311(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1577 = v0.clone();
        length_bytes(a1577)
    };
    let a1578 = 0u64;
    let a1579 = v1;
    if nat_lt(a1578, a1579) {
        let a1580 = f372(v0.clone())?;
        let a1581 = v1;
        if nat_eq(a1580, a1581) {
            let a1582 = 1u64;
            let a1583 = v1;
            if nat_lt(a1582, a1583) {
                let m1586 = {
                    let a1584 = v0;
                    let a1585 = 0u64;
                    index_bytes(a1584, a1585)
                };
                match m1586 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1587 = v2;
                        let a1588 = 48u8;
                        Ok(!equal(a1587, a1588))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f312(v0: Fn51, p1593: u64, p1594: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1592 = (p1593, p1594);
    {
        let mut m1592 = m1592;
        loop {
            let step = {
                let (v1, v2) = m1592;
                let m1589 = v1;
                if m1589 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1589 - 1;
                    let m1591 = {
                        let c1590: Fn51 = v0.clone();
                        c1590.apply(v2)?
                    };
                    match m1591 {
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
                Ok(next) => { m1592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f373(v0, v1)
}

pub fn f314(v0: List<Adt16>, v1: Adt16) -> List<Adt16> {
    List::cons(v1, v0)
}

pub fn f315(v0: Adt16, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt17)>) -> R<Adt16> {
    if f176(v0.clone(), v1, v2)? {
        Ok(Adt16::C0(match v0.clone() {
            Adt16::C0(h1595, _, _) => {
                h1595
            }
        }, match v0.clone() {
            Adt16::C0(_, h1596, _) => {
                h1596
            }
        }, f374(List::cons(v3, f374(match v0 {
            Adt16::C0(_, _, h1597) => {
                h1597
            }
        })))))
    } else {
        Ok(v0)
    }
}

pub fn f316(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m1598 = f117(Fn13::F375(v1), None::<Bytes>, v0)?;
    match m1598 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f317() -> List<Bytes> {
    List::cons(f3(Str::lit("entry")), List::cons(f3(Str::lit("lean_name")), List::cons(f3(Str::lit("fields")), List::cons(f3(Str::lit("induction_hypotheses")), List::<Bytes>::nil()))))
}

pub fn f318(v0: Adt32) -> Option<Adt32> {
    Some(Adt32::C0(match v0.clone() {
        Adt32::C0(h1599, _, _, _) => {
            h1599
        }
    }, match v0.clone() {
        Adt32::C0(_, h1600, _, _) => {
            h1600
        }
    }, match v0 {
        Adt32::C0(_, _, h1601, _) => {
            h1601
        }
    }, false))
}

pub fn f319(v0: Adt32, v1: bool, v2: bool) -> R<Option<Adt32>> {
    Ok(Some(Adt32::C0({
        let a1603 = match v0 {
            Adt32::C0(h1602, _, _, _) => {
                h1602
            }
        };
        let a1604 = 1u64;
        nat_add(a1603, a1604)?
    }, v1, v2, true)))
}

pub fn f320(v0: u64) -> bool {
    let a1605 = v0;
    let a1606 = 97u64;
    if nat_lt(a1605, a1606) {
        false
    } else {
        let a1607 = v0;
        let a1608 = 122u64;
        nat_le(a1607, a1608)
    }
}

pub fn f321(v0: u64) -> bool {
    let a1609 = v0;
    let a1610 = 48u64;
    if nat_lt(a1609, a1610) {
        false
    } else {
        let a1611 = v0;
        let a1612 = 57u64;
        nat_le(a1611, a1612)
    }
}

pub fn f322(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1613 = v2;
    let a1614 = v1;
    if nat_lt(a1613, a1614) {
        if f366(f264(v0, v2)?) {
            Ok(Some({
                let a1615 = v2;
                let a1616 = 1u64;
                nat_add(a1615, a1616)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f323(v0: Adt36, v1: Adt28) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h1617, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt36::C0(_, h1618, _, _, _) => {
            h1618
        }
    }, match v0 {
        Adt36::C0(_, _, h1619, _, _) => {
            h1619
        }
    }, Some(v1), false)
}

pub fn f324(v0: u64, v1: Adt36, v2: Adt33) -> R<Adt36> {
    let m1620 = v2;
    match m1620 {
        Adt33::C0 => {
            let a1623 = {
                let a1622 = match v1.clone() {
                    Adt36::C0(_, h1621, _, _, _) => {
                        h1621
                    }
                };
                length_list(a1622)
            };
            let a1624 = v0;
            if nat_lt(a1623, a1624) {
                Ok(Adt36::C0({
                    let a1626 = match v1.clone() {
                        Adt36::C0(h1625, _, _, _, _) => {
                            h1625
                        }
                    };
                    let a1627 = 1u64;
                    nat_add(a1626, a1627)?
                }, List::cons(List::<Adt35>::nil(), match v1.clone() {
                    Adt36::C0(_, h1628, _, _, _) => {
                        h1628
                    }
                }), match v1 {
                    Adt36::C0(_, _, h1629, _, _) => {
                        h1629
                    }
                }, None::<Adt28>, false))
            } else {
                Ok(f323(v1, Adt28::C0(1u64, f205(f201(Str::lit("max_scope_depth exceeded: configured ")), f48(v0)))))
            }
        }
        Adt33::C1 => {
            let m1631 = match v1.clone() {
                Adt36::C0(_, h1630, _, _, _) => {
                    h1630
                }
            };
            match m1631.uncons() {
                None => {
                    Ok(f323(v1, f200(f201(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f376(v1, Adt35::C1(std::rc::Rc::new(f377(v3))), v4)
                }
            }
        }
        Adt33::C2(v5) => {
            f376(v1.clone(), Adt35::C0(v5), match v1 {
                Adt36::C0(_, h1632, _, _, _) => {
                    h1632
                }
            })
        }
    }
}

pub fn f325(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: u64) -> R<Adt37> {
    let m1633 = v2.clone();
    match m1633 {
        Adt35::C0(_) => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C1(r1636) => {
            let v4 = (*r1636).clone();
            let m1634 = f378(v4, v3)?;
            match m1634.uncons() {
                None => {
                    Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                }
                Some((v5, v6)) => {
                    let m1635 = v5;
                    match m1635 {
                        Adt35::C0(v7) => {
                            f380(v0, v1, v2, v7, v6, v3)
                        }
                        Adt35::C1(_) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                        Adt35::C2 => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected an LSE form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C3(_, _) => {
            Ok(f379(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
    }
}

pub fn f326(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: u64) -> R<Adt37> {
    let m1637 = v2;
    match m1637 {
        Adt35::C0(_) => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
        Adt35::C1(r1665) => {
            let v4 = (*r1665).clone();
            let v5: List<Adt35> = f378(v4, v3)?;
            let m1638 = v5;
            match m1638.uncons() {
                None => {
                    Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
                }
                Some((v6, v7)) => {
                    let m1639 = v6;
                    match m1639 {
                        Adt35::C0(v8) => {
                            let a1640 = v8.clone();
                            let a1641 = f201(Str::lit("app"));
                            if equal(a1640, a1641) {
                                let v9: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                    Adt37::C0(_, h1642, _) => {
                                        h1642
                                    }
                                }, {
                                    let a1643 = v7;
                                    length_list(a1643)
                                });
                                let m1645 = {
                                    let (h1644, _) = v9;
                                    h1644
                                };
                                match m1645.uncons() {
                                    None => {
                                        Ok(f379(v0, f200(f201(Str::lit("missing function")))))
                                    }
                                    Some((v10, v11)) => {
                                        Ok(Adt37::C0(v1, List::cons(Adt1::C4(std::rc::Rc::new(v10), std::rc::Rc::new(f383(v11))), {
                                            let (_, h1646) = v9;
                                            h1646
                                        }), None::<Adt28>))
                                    }
                                }
                            } else {
                                if f384(v8.clone()) {
                                    let m1647 = f385(v7, 0u64);
                                    match m1647 {
                                        Adt35::C0(_) => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                        Adt35::C1(r1657) => {
                                            let v12 = (*r1657).clone();
                                            let v13: List<Adt35> = f378(v12, v3)?;
                                            let v14: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                                Adt37::C0(_, h1648, _) => {
                                                    h1648
                                                }
                                            }, {
                                                let a1650 = {
                                                    let a1649 = v13.clone();
                                                    length_list(a1649)
                                                };
                                                let a1651 = 1u64;
                                                nat_add(a1650, a1651)?
                                            });
                                            let m1653 = f386({
                                                let (h1652, _) = v14;
                                                h1652
                                            });
                                            match m1653.uncons() {
                                                None => {
                                                    Ok(f379(v0, f200(f201(Str::lit("missing body")))))
                                                }
                                                Some((v15, v16)) => {
                                                    let v17: Adt1 = f387(v13, f386(v16), v3)?;
                                                    Ok(Adt37::C0(v1, List::cons({
                                                        let a1654 = v8;
                                                        let a1655 = f201(Str::lit("pi"));
                                                        if equal(a1654, a1655) {
                                                            Adt1::C5(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        } else {
                                                            Adt1::C6(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        }
                                                    }, {
                                                        let (_, h1656) = v14;
                                                        h1656
                                                    }), None::<Adt28>))
                                                }
                                            }
                                        }
                                        Adt35::C2 => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                        Adt35::C3(_, _) => {
                                            Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
                                        }
                                    }
                                } else {
                                    let v21: (List<Adt1>, List<Adt1>) = f382(match v0.clone() {
                                        Adt37::C0(_, h1658, _) => {
                                            h1658
                                        }
                                    }, 3u64);
                                    let m1660 = {
                                        let (h1659, _) = v21;
                                        h1659
                                    };
                                    match m1660.uncons() {
                                        None => {
                                            Ok(f379(v0, f200(f201(Str::lit("missing type")))))
                                        }
                                        Some((v22, v23)) => {
                                            let m1661 = v23;
                                            match m1661.uncons() {
                                                None => {
                                                    Ok(f379(v0, f200(f201(Str::lit("missing value")))))
                                                }
                                                Some((v24, v25)) => {
                                                    let m1662 = v25;
                                                    match m1662.uncons() {
                                                        None => {
                                                            Ok(f379(v0, f200(f201(Str::lit("missing body")))))
                                                        }
                                                        Some((v26, _)) => {
                                                            let m1663 = f388(f385(v7, 0u64), f201(Str::lit("a local ID")), v3);
                                                            match m1663 {
                                                                Ok(v28) => {
                                                                    Ok(Adt37::C0(v1, List::cons(Adt1::C7(v28, std::rc::Rc::new(v22), std::rc::Rc::new(v24), std::rc::Rc::new(v26)), {
                                                                        let (_, h1664) = v21;
                                                                        h1664
                                                                    }), None::<Adt28>))
                                                                }
                                                                Err(v29) => {
                                                                    Ok(f379(v0, v29))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Adt35::C1(_) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                        Adt35::C2 => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f379(v0, f200(f201(Str::lit("expected a form, found a list")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
        Adt35::C3(_, _) => {
            Ok(f379(v0, f200(f201(Str::lit("expected a list")))))
        }
    }
}

pub fn f327(v0: Adt29, v1: List<Adt30>) -> Option<Adt29> {
    Some(Adt29::C0(v1, match v0.clone() {
        Adt29::C0(_, h1666, _, _) => {
            h1666
        }
    }, match v0 {
        Adt29::C0(_, _, h1667, _) => {
            h1667
        }
    }, None::<Bytes>))
}

pub fn f328(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f88(Fn8::F389(v1), false, v0)
}

pub fn f329(v0: Adt29, v1: Bytes) -> Option<Adt29> {
    Some(Adt29::C0(match v0.clone() {
        Adt29::C0(h1668, _, _, _) => {
            h1668
        }
    }, match v0.clone() {
        Adt29::C0(_, h1669, _, _) => {
            h1669
        }
    }, match v0 {
        Adt29::C0(_, _, h1670, _) => {
            h1670
        }
    }, Some(v1)))
}

pub fn f330(v0: Bytes, v1: Bytes) -> Bytes {
    let a1671 = v0;
    let a1672 = v1;
    append_bytes(a1671, a1672)
}

pub fn f331(v0: Str) -> Bytes {
    let a1673 = v0;
    utf8_encode(a1673)
}

pub fn f332(v0: Fn52, p1678: u64, p1679: List<Bytes>) -> (List<Bytes>, bool) {
    let m1677 = (p1678, p1679);
    {
        let mut m1677 = m1677;
        loop {
            let step = {
                let (v1, v2) = m1677;
                let m1674 = v1;
                if m1674 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m1674 - 1;
                    let m1676 = {
                        let c1675: Fn52 = v0.clone();
                        c1675.apply(v2.clone())
                    };
                    match m1676 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1680 = v0;
    match m1680.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            Some(v2)
        }
    }
}

pub fn f334(v0: Fn53, p1684: u64, p1685: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
    let m1683 = (p1684, p1685);
    {
        let mut m1683 = m1683;
        loop {
            let step = {
                let (v1, v2) = m1683;
                let m1681 = v1;
                if m1681 == 0 {
                    Err::<(u64, (u64, List<Adt39>)), R<(u64, List<Adt39>)>>(Ok(v2))
                } else {
                    let v3 = m1681 - 1;
                    Ok::<(u64, (u64, List<Adt39>)), R<(u64, List<Adt39>)>>((v3, {
                        let c1682: Fn53 = v0.clone();
                        c1682.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes, v1: u64, v2: (u64, List<Adt39>)) -> R<(u64, List<Adt39>)> {
    f390(v2.clone(), f279(v0, v1, {
        let (h1686, _) = v2;
        h1686
    }, false)?)
}

pub fn f336(v0: u64, v1: u64) -> u64 {
    let a1689 = 1024u64;
    let a1690 = {
        let a1687 = v0;
        let a1688 = v1;
        nat_sub(a1687, a1688)
    };
    if nat_lt(a1689, a1690) {
        1024u64
    } else {
        let a1691 = v0;
        let a1692 = v1;
        nat_sub(a1691, a1692)
    }
}

pub fn f337(v0: u64) -> R<u64> {
    let a1693 = v0;
    let a1694 = 0u64;
    if nat_eq(a1693, a1694) {
        Ok(1u64)
    } else {
        let a1697 = {
            let a1695 = v0;
            let a1696 = 63u64;
            nat_add(a1695, a1696)?
        };
        let a1698 = 64u64;
        let a1699 = 0u64;
        Ok(nat_quot(a1697, a1698, a1699))
    }
}

pub fn f338(v0: Adt39, v1: Adt46, v2: u64, v3: u32, v4: u32) -> Adt39 {
    let v5: Adt47 = {
        let (h1700, _) = f391(Fn59::F392, 7u64, (Adt47::C0(match v0.clone() {
            Adt39::C0(h1701, _, _, _, _, _, _, _) => {
                h1701
            }
        }, match v0.clone() {
            Adt39::C0(_, h1702, _, _, _, _, _, _) => {
                h1702
            }
        }, match v0.clone() {
            Adt39::C0(_, _, h1703, _, _, _, _, _) => {
                h1703
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, h1704, _, _, _, _) => {
                h1704
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, h1705, _, _, _) => {
                h1705
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, _, h1706, _, _) => {
                h1706
            }
        }, match v0.clone() {
            Adt39::C0(_, _, _, _, _, _, h1707, _) => {
                h1707
            }
        }, match v0 {
            Adt39::C0(_, _, _, _, _, _, _, h1708) => {
                h1708
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f341({
            let a1709 = v2;
            let a1710 = 4294967296u64;
            let a1711 = 0u64;
            nat_rem(a1709, a1710, a1711)
        }), f341({
            let a1712 = v2;
            let a1713 = 4294967296u64;
            let a1714 = 0u64;
            nat_quot(a1712, a1713, a1714)
        }), v3, v4), v1));
        h1700
    };
    Adt39::C0({
        let a1717 = match v5.clone() {
            Adt47::C0(h1715, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1715
            }
        };
        let a1718 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, h1716, _, _, _, _, _, _, _) => {
                h1716
            }
        };
        fixed_u32::bit_xor(a1717, a1718)
    }, {
        let a1721 = match v5.clone() {
            Adt47::C0(_, h1719, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1719
            }
        };
        let a1722 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, h1720, _, _, _, _, _, _) => {
                h1720
            }
        };
        fixed_u32::bit_xor(a1721, a1722)
    }, {
        let a1725 = match v5.clone() {
            Adt47::C0(_, _, h1723, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1723
            }
        };
        let a1726 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, h1724, _, _, _, _, _) => {
                h1724
            }
        };
        fixed_u32::bit_xor(a1725, a1726)
    }, {
        let a1729 = match v5.clone() {
            Adt47::C0(_, _, _, h1727, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1727
            }
        };
        let a1730 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h1728, _, _, _, _) => {
                h1728
            }
        };
        fixed_u32::bit_xor(a1729, a1730)
    }, {
        let a1733 = match v5.clone() {
            Adt47::C0(_, _, _, _, h1731, _, _, _, _, _, _, _, _, _, _, _) => {
                h1731
            }
        };
        let a1734 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1732, _, _, _) => {
                h1732
            }
        };
        fixed_u32::bit_xor(a1733, a1734)
    }, {
        let a1737 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, h1735, _, _, _, _, _, _, _, _, _, _) => {
                h1735
            }
        };
        let a1738 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1736, _, _) => {
                h1736
            }
        };
        fixed_u32::bit_xor(a1737, a1738)
    }, {
        let a1741 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, h1739, _, _, _, _, _, _, _, _, _) => {
                h1739
            }
        };
        let a1742 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1740, _) => {
                h1740
            }
        };
        fixed_u32::bit_xor(a1741, a1742)
    }, {
        let a1745 = match v5.clone() {
            Adt47::C0(_, _, _, _, _, _, _, h1743, _, _, _, _, _, _, _, _) => {
                h1743
            }
        };
        let a1746 = match v5 {
            Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1744) => {
                h1744
            }
        };
        fixed_u32::bit_xor(a1745, a1746)
    })
}

pub fn f339(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt39> {
    let (_, h1747) = f393(Fn60::F394(v2, v0, v1), v3, (0u64, f395()))?;
    Ok(h1747)
}

pub fn f340(v0: Bytes, v1: u64) -> R<Adt46> {
    Ok(Adt46::C0(f396(v0.clone(), {
        let a1748 = v1;
        let a1749 = 0u64;
        nat_add(a1748, a1749)?
    })?, f396(v0.clone(), {
        let a1750 = v1;
        let a1751 = 4u64;
        nat_add(a1750, a1751)?
    })?, f396(v0.clone(), {
        let a1752 = v1;
        let a1753 = 8u64;
        nat_add(a1752, a1753)?
    })?, f396(v0.clone(), {
        let a1754 = v1;
        let a1755 = 12u64;
        nat_add(a1754, a1755)?
    })?, f396(v0.clone(), {
        let a1756 = v1;
        let a1757 = 16u64;
        nat_add(a1756, a1757)?
    })?, f396(v0.clone(), {
        let a1758 = v1;
        let a1759 = 20u64;
        nat_add(a1758, a1759)?
    })?, f396(v0.clone(), {
        let a1760 = v1;
        let a1761 = 24u64;
        nat_add(a1760, a1761)?
    })?, f396(v0.clone(), {
        let a1762 = v1;
        let a1763 = 28u64;
        nat_add(a1762, a1763)?
    })?, f396(v0.clone(), {
        let a1764 = v1;
        let a1765 = 32u64;
        nat_add(a1764, a1765)?
    })?, f396(v0.clone(), {
        let a1766 = v1;
        let a1767 = 36u64;
        nat_add(a1766, a1767)?
    })?, f396(v0.clone(), {
        let a1768 = v1;
        let a1769 = 40u64;
        nat_add(a1768, a1769)?
    })?, f396(v0.clone(), {
        let a1770 = v1;
        let a1771 = 44u64;
        nat_add(a1770, a1771)?
    })?, f396(v0.clone(), {
        let a1772 = v1;
        let a1773 = 48u64;
        nat_add(a1772, a1773)?
    })?, f396(v0.clone(), {
        let a1774 = v1;
        let a1775 = 52u64;
        nat_add(a1774, a1775)?
    })?, f396(v0.clone(), {
        let a1776 = v1;
        let a1777 = 56u64;
        nat_add(a1776, a1777)?
    })?, f396(v0, {
        let a1778 = v1;
        let a1779 = 60u64;
        nat_add(a1778, a1779)?
    })?))
}

pub fn f341(v0: u64) -> u32 {
    let (_, h1780) = {
        let (_, h1781) = f397(Fn61::F398, 32u64, (v0, (1u32, 0u32)));
        h1781
    };
    h1780
}

pub fn f342(v0: (List<Adt39>, Adt39)) -> (List<Adt39>, Adt39) {
    let m1783 = {
        let (h1782, _) = v0.clone();
        h1782
    };
    match m1783.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f399(v1, {
                let (_, h1784) = v0;
                h1784
            }, {
                let m1785 = v2;
                match m1785.uncons() {
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

pub fn f343(v0: u32) -> Bytes {
    let a1788 = {
        let a1786 = v0;
        let a1787 = 8u32;
        fixed_u32::bit_and(a1786, a1787)
    };
    let a1789 = 0u32;
    if equal(a1788, a1789) {
        let a1792 = {
            let a1790 = v0;
            let a1791 = 4u32;
            fixed_u32::bit_and(a1790, a1791)
        };
        let a1793 = 0u32;
        if equal(a1792, a1793) {
            let a1796 = {
                let a1794 = v0;
                let a1795 = 2u32;
                fixed_u32::bit_and(a1794, a1795)
            };
            let a1797 = 0u32;
            if equal(a1796, a1797) {
                let a1800 = {
                    let a1798 = v0;
                    let a1799 = 1u32;
                    fixed_u32::bit_and(a1798, a1799)
                };
                let a1801 = 0u32;
                if equal(a1800, a1801) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1804 = {
                    let a1802 = v0;
                    let a1803 = 1u32;
                    fixed_u32::bit_and(a1802, a1803)
                };
                let a1805 = 0u32;
                if equal(a1804, a1805) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1808 = {
                let a1806 = v0;
                let a1807 = 2u32;
                fixed_u32::bit_and(a1806, a1807)
            };
            let a1809 = 0u32;
            if equal(a1808, a1809) {
                let a1812 = {
                    let a1810 = v0;
                    let a1811 = 1u32;
                    fixed_u32::bit_and(a1810, a1811)
                };
                let a1813 = 0u32;
                if equal(a1812, a1813) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1816 = {
                    let a1814 = v0;
                    let a1815 = 1u32;
                    fixed_u32::bit_and(a1814, a1815)
                };
                let a1817 = 0u32;
                if equal(a1816, a1817) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1820 = {
            let a1818 = v0;
            let a1819 = 4u32;
            fixed_u32::bit_and(a1818, a1819)
        };
        let a1821 = 0u32;
        if equal(a1820, a1821) {
            let a1824 = {
                let a1822 = v0;
                let a1823 = 2u32;
                fixed_u32::bit_and(a1822, a1823)
            };
            let a1825 = 0u32;
            if equal(a1824, a1825) {
                let a1828 = {
                    let a1826 = v0;
                    let a1827 = 1u32;
                    fixed_u32::bit_and(a1826, a1827)
                };
                let a1829 = 0u32;
                if equal(a1828, a1829) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1832 = {
                    let a1830 = v0;
                    let a1831 = 1u32;
                    fixed_u32::bit_and(a1830, a1831)
                };
                let a1833 = 0u32;
                if equal(a1832, a1833) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1836 = {
                let a1834 = v0;
                let a1835 = 2u32;
                fixed_u32::bit_and(a1834, a1835)
            };
            let a1837 = 0u32;
            if equal(a1836, a1837) {
                let a1840 = {
                    let a1838 = v0;
                    let a1839 = 1u32;
                    fixed_u32::bit_and(a1838, a1839)
                };
                let a1841 = 0u32;
                if equal(a1840, a1841) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1844 = {
                    let a1842 = v0;
                    let a1843 = 1u32;
                    fixed_u32::bit_and(a1842, a1843)
                };
                let a1845 = 0u32;
                if equal(a1844, a1845) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f344(v0: u32, v1: u32) -> u32 {
    let m1848 = {
        let a1846 = v0;
        let a1847 = v1;
        fixed_u32::shift_right(a1846, a1847)
    };
    match m1848 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f345(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1849 = v0.clone();
        length_bytes(a1849)
    };
    let a1850 = v1;
    let a1851 = 0u64;
    if nat_eq(a1850, a1851) {
        Ok(false)
    } else {
        if f320(f264(v0.clone(), 0u64)?) {
            let a1853 = {
                let (h1852, _) = f240(Fn39::F400(v1, v0), v1, 1u64)?;
                h1852
            };
            let a1854 = v1;
            Ok(nat_eq(a1853, a1854))
        } else {
            Ok(false)
        }
    }
}

pub fn f346(v0: Bytes) -> R<bool> {
    let a1856 = {
        let a1855 = v0.clone();
        length_bytes(a1855)
    };
    let a1857 = 0u64;
    if nat_eq(a1856, a1857) {
        Ok(true)
    } else {
        if f159(v0.clone(), f3(Str::lit(" "))) {
            Ok(true)
        } else {
            if f154(v0.clone(), f3(Str::lit(" "))) {
                Ok(true)
            } else {
                let m1858 = f161(v0.clone(), f5(f3(Str::lit(" ")), f3(Str::lit(" "))))?;
                if match m1858 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                } {
                    Ok(true)
                } else {
                    let m1859 = f161(v0, Bytes::lit(&[0x0a]))?;
                    match m1859 {
                        None => {
                            Ok(false)
                        }
                        Some(_) => {
                            Ok(true)
                        }
                    }
                }
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: Bytes, v2: u64) -> R<Result<List<Adt5>, Adt48>> {
    let v3: u64 = {
        let a1860 = v1.clone();
        length_bytes(a1860)
    };
    let v4: Adt50 = {
        let (h1861, _) = f401(Fn62::F402(v2, v3, v0, v1), {
            let a1862 = v3;
            let a1863 = 1u64;
            nat_add(a1862, a1863)?
        }, Adt50::C0(Adt51::C0(0u64, 1u64, 1u64), List::<Adt5>::nil(), 0u64, None::<Adt48>))?;
        h1861
    };
    let m1865 = match v4.clone() {
        Adt50::C0(_, _, _, h1864) => {
            h1864
        }
    };
    match m1865 {
        None => {
            Ok(Ok::<List<Adt5>, Adt48>(f403(match v4 {
                Adt50::C0(_, h1866, _, _) => {
                    h1866
                }
            })))
        }
        Some(v5) => {
            Ok(Err::<List<Adt5>, Adt48>(v5))
        }
    }
}

pub fn f348(v0: Fn54, p1870: List<(Bytes, Bytes)>, p1871: List<Adt5>) -> R<List<(Bytes, Bytes)>> {
    let m1869 = (p1870, p1871);
    {
        let mut m1869 = m1869;
        loop {
            let step = {
                let (v1, v2) = m1869;
                let m1867 = v2;
                match m1867.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<Adt5>), R<List<(Bytes, Bytes)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<Adt5>), R<List<(Bytes, Bytes)>>>(({
                            let c1868: Fn54 = v0.clone();
                            c1868.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: List<Bytes>, v1: Bytes, v2: List<(Bytes, Bytes)>, v3: Adt5) -> R<List<(Bytes, Bytes)>> {
    if f404(v3.clone()) {
        if f214(v0, match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, _, h1872) => {
                h1872
            }
        })? {
            Ok(List::cons(f2(f5(f5(f5(f3(Str::lit("form `")), v1), f3(Str::lit("` declares the always-forbidden control `"))), f5(match v3 {
                Adt5::C0(_, _, _, _, _, _, _, h1873) => {
                    h1873
                }
            }, f3(Str::lit("` (\u{a7}12.4)"))))), v2))
        } else {
            Ok(v2)
        }
    } else {
        Ok(v2)
    }
}

pub fn f350(v0: Fn55, p1877: bool, p1878: List<Adt5>) -> bool {
    let m1876 = (p1877, p1878);
    {
        let mut m1876 = m1876;
        loop {
            let step = {
                let (v1, v2) = m1876;
                let m1874 = v2;
                match m1874.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1875: Fn55 = v0.clone();
                            c1875.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: bool, v1: Adt5) -> bool {
    if v0 {
        true
    } else {
        f404(v1)
    }
}

pub fn f352(v0: Fn56, p1882: (u64, Option<Bytes>), p1883: List<Adt5>) -> R<(u64, Option<Bytes>)> {
    let m1881 = (p1882, p1883);
    {
        let mut m1881 = m1881;
        loop {
            let step = {
                let (v1, v2) = m1881;
                let m1879 = v2;
                match m1879.uncons() {
                    None => {
                        Err::<((u64, Option<Bytes>), List<Adt5>), R<(u64, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<Bytes>), List<Adt5>), R<(u64, Option<Bytes>)>>(({
                            let c1880: Fn56 = v0.clone();
                            c1880.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: u64, v1: (u64, Option<Bytes>), v2: Adt5) -> R<(u64, Option<Bytes>)> {
    let m1885 = {
        let (_, h1884) = v1.clone();
        h1884
    };
    match m1885 {
        None => {
            if f405(v2.clone(), {
                let (h1886, _) = v1.clone();
                h1886
            }, v0)? {
                Ok(({
                    let a1888 = {
                        let (h1887, _) = v1;
                        h1887
                    };
                    let a1889 = 1u64;
                    nat_add(a1888, a1889)?
                }, None::<Bytes>))
            } else {
                Ok(({
                    let a1891 = {
                        let (h1890, _) = v1;
                        h1890
                    };
                    let a1892 = 1u64;
                    nat_add(a1891, a1892)?
                }, Some(f406(v2, v0))))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f354(v0: Bytes, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<List<(Bytes, Bytes)>> {
    if f171(f108(), v2.clone())? {
        Ok(v1)
    } else {
        Ok(List::cons(f2(f5(f5(f5(f5(f3(Str::lit("form `")), v0), f3(Str::lit("`: `"))), v2), f3(Str::lit("` is not a feature")))), v1))
    }
}

pub fn f355(v0: u64, v1: Adt43) -> R<Option<Adt43>> {
    let m1894 = match v1.clone() {
        Adt43::C0(_, _, h1893) => {
            h1893
        }
    };
    match m1894 {
        None => {
            let m1896 = match v1.clone() {
                Adt43::C0(h1895, _, _) => {
                    h1895
                }
            };
            match m1896.uncons() {
                None => {
                    Ok(None::<Adt43>)
                }
                Some((v3, v4)) => {
                    let m1897 = v3;
                    match m1897 {
                        Adt44::C0(v5) => {
                            Ok(Some(f407(v1, v4, v5, v0)?))
                        }
                        Adt44::C1(v6, v7) => {
                            Ok(Some(f408(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt43>)
        }
    }
}

pub fn f356(v0: List<Adt7>, v1: Adt31) -> Option<Adt40> {
    Some(Adt40::C0(v0, v1))
}

pub fn f357(v0: Fn57, p1901: List<Adt7>, p1902: List<Adt7>) -> List<Adt7> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1898 = v2;
                match m1898.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c1899: Fn57 = v0.clone();
                            c1899.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f359(v0: List<Adt7>) -> List<Adt7> {
    f357(Fn57::F409, List::<Adt7>::nil(), v0)
}

pub fn f360(v0: Adt7, v1: u64) -> R<List<Adt7>> {
    Ok(f359({
        let (_, h1903) = {
            let (h1904, _) = f410(Fn63::F411, {
                let a1905 = v1;
                let a1906 = 1u64;
                nat_add(a1905, a1906)?
            }, (v0, List::<Adt7>::nil()));
            h1904
        };
        h1903
    }))
}

pub fn f361(v0: Adt31, v1: Bytes, v2: Adt7, v3: Adt7, v4: u64) -> R<Adt31> {
    Ok(Adt31::C0(match v0.clone() {
        Adt31::C0(h1907, _, _, _, _, _, _) => {
            h1907
        }
    }, match v0.clone() {
        Adt31::C0(_, h1908, _, _, _, _, _) => {
            h1908
        }
    }, match v0.clone() {
        Adt31::C0(_, _, h1909, _, _, _, _) => {
            h1909
        }
    }, match v0.clone() {
        Adt31::C0(_, _, _, h1910, _, _, _) => {
            h1910
        }
    }, match v0.clone() {
        Adt31::C0(_, _, _, _, h1911, _, _) => {
            h1911
        }
    }, f362(match v0.clone() {
        Adt31::C0(_, _, _, _, _, h1912, _) => {
            h1912
        }
    }, v1.clone()), f412(match v0 {
        Adt31::C0(_, _, _, _, _, _, h1913) => {
            h1913
        }
    }, f413(v1, v2, v3, v4)?)))
}

pub fn f362(v0: Option<Bytes>, v1: Bytes) -> Option<Bytes> {
    let m1914 = v0.clone();
    match m1914 {
        None => {
            Some(v1)
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f363(v0: Fn58, p1918: (bool, List<u64>), p1919: List<u64>) -> (bool, List<u64>) {
    let m1917 = (p1918, p1919);
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v1, v2) = m1917;
                let m1915 = v2;
                match m1915.uncons() {
                    None => {
                        Err::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<u64>), List<u64>), (bool, List<u64>)>(({
                            let c1916: Fn58 = v0.clone();
                            c1916.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: u64, v1: (bool, List<u64>), v2: u64) -> (bool, List<u64>) {
    let (h1920, _) = v1;
    if h1920 {
        (true, List::cons(v2, {
            let (_, h1921) = v1;
            h1921
        }))
    } else {
        let a1922 = v0;
        let a1923 = v2;
        if nat_le(a1922, a1923) {
            (true, List::cons(v2, List::cons(v0, {
                let (_, h1924) = v1;
                h1924
            })))
        } else {
            (false, List::cons(v2, {
                let (_, h1925) = v1;
                h1925
            }))
        }
    }
}

pub fn f365(v0: List<Adt4>, v1: Bytes) -> Option<Adt4> {
    f414(Fn64::F415(v1), None::<Adt4>, v0)
}

pub fn f366(v0: u64) -> bool {
    if f263(v0) {
        true
    } else {
        f321(v0)
    }
}

pub fn f367(v0: (List<(Bytes, Bytes)>, List<Bytes>), v1: Bytes) -> R<(List<(Bytes, Bytes)>, List<Bytes>)> {
    let v2: bool = f214({
        let (_, h1926) = v0.clone();
        h1926
    }, v1.clone())?;
    let v3: bool = if f193(v1.clone())? {
        v2
    } else {
        true
    };
    Ok((f46({
        let (h1927, _) = v0;
        h1927
    }, if v3 {
        Some(f2(f5(f5(f3(Str::lit("invalid or duplicate eliminator binder ")), f6(v1.clone())), Bytes::lit(&[]))))
    } else {
        None::<(Bytes, Bytes)>
    }), List::cons(v1, {
        let (_, h1928) = v0;
        h1928
    })))
}

pub fn f368(v0: Bytes, v1: u64) -> bool {
    let m1929 = f247(v0, v1);
    match m1929 {
        None => {
            false
        }
        Some(v2) => {
            f416(v2)
        }
    }
}

pub fn f369(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1930 = v0.clone();
        length_bytes(a1930)
    };
    let v3: u64 = {
        let a1931 = v2;
        let a1932 = v1;
        nat_sub(a1931, a1932)
    };
    let a1933 = v1;
    let a1934 = f235(f83(v0, v1, v3), v3)?;
    nat_add(a1933, a1934)
}

pub fn f370(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h1935, _) = v2.clone();
        h1935
    };
    let v4: u64 = {
        let (h1936, _) = {
            let (_, h1937) = v2.clone();
            h1937
        };
        h1936
    };
    let v5: List<Bytes> = {
        let (_, h1938) = {
            let (_, h1939) = v2;
            h1939
        };
        h1938
    };
    let m1940 = f247(v0.clone(), v3);
    match m1940 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a1941 = v6;
            let a1942 = v1;
            if equal(a1941, a1942) {
                Ok(Some(({
                    let a1943 = v3;
                    let a1944 = 1u64;
                    nat_add(a1943, a1944)?
                }, ({
                    let a1945 = v3;
                    let a1946 = 1u64;
                    nat_add(a1945, a1946)?
                }, List::cons(f83(v0, v4, {
                    let a1947 = v3;
                    let a1948 = v4;
                    nat_sub(a1947, a1948)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a1949 = v3;
                    let a1950 = 1u64;
                    nat_add(a1949, a1950)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: Adt45) -> R<Option<Adt45>> {
    if match v2.clone() {
        Adt45::C0(_, _, _, h1951) => {
            h1951
        }
    } {
        Ok(None::<Adt45>)
    } else {
        let v3: u64 = match v2.clone() {
            Adt45::C0(h1952, _, _, _) => {
                h1952
            }
        };
        let a1953 = v3;
        let a1954 = v1;
        if nat_lt(a1953, a1954) {
            let v4: u64 = match v2.clone() {
                Adt45::C0(_, _, h1955, _) => {
                    h1955
                }
            };
            let a1956 = v4;
            let a1957 = 1u64;
            if nat_eq(a1956, a1957) {
                let m1958 = f247(v0, v3);
                match m1958 {
                    None => {
                        Ok(None::<Adt45>)
                    }
                    Some(v5) => {
                        let a1959 = v5;
                        let a1960 = 34u8;
                        if if equal(a1959, a1960) {
                            true
                        } else {
                            let a1961 = v5;
                            let a1962 = 92u8;
                            if equal(a1961, a1962) {
                                true
                            } else {
                                let a1963 = v5;
                                let a1964 = 110u8;
                                if equal(a1963, a1964) {
                                    true
                                } else {
                                    let a1965 = v5;
                                    let a1966 = 116u8;
                                    if equal(a1965, a1966) {
                                        true
                                    } else {
                                        let a1967 = v5;
                                        let a1968 = 114u8;
                                        equal(a1967, a1968)
                                    }
                                }
                            }
                        } {
                            Ok(Some(Adt45::C0({
                                let a1969 = v3;
                                let a1970 = 1u64;
                                nat_add(a1969, a1970)?
                            }, f417(match v2 {
                                Adt45::C0(_, h1971, _, _) => {
                                    h1971
                                }
                            }, {
                                let a1972 = v5;
                                let a1973 = 34u8;
                                if equal(a1972, a1973) {
                                    Bytes::lit(&[0x22])
                                } else {
                                    let a1974 = v5;
                                    let a1975 = 92u8;
                                    if equal(a1974, a1975) {
                                        Bytes::lit(&[0x5c])
                                    } else {
                                        let a1976 = v5;
                                        let a1977 = 110u8;
                                        if equal(a1976, a1977) {
                                            Bytes::lit(&[0x0a])
                                        } else {
                                            let a1978 = v5;
                                            let a1979 = 116u8;
                                            if equal(a1978, a1979) {
                                                Bytes::lit(&[0x09])
                                            } else {
                                                Bytes::lit(&[0x0d])
                                            }
                                        }
                                    }
                                }
                            }), 0u64, false)))
                        } else {
                            Ok(None::<Adt45>)
                        }
                    }
                }
            } else {
                let m1980 = f247(v0.clone(), v3);
                match m1980 {
                    None => {
                        Ok(None::<Adt45>)
                    }
                    Some(v6) => {
                        let a1981 = v6;
                        let a1982 = 34u8;
                        if equal(a1981, a1982) {
                            Ok(Some(Adt45::C0({
                                let a1983 = v3;
                                let a1984 = 1u64;
                                nat_add(a1983, a1984)?
                            }, match v2 {
                                Adt45::C0(_, h1985, _, _) => {
                                    h1985
                                }
                            }, 0u64, true)))
                        } else {
                            let a1986 = v6;
                            let a1987 = 92u8;
                            if equal(a1986, a1987) {
                                Ok(Some(Adt45::C0({
                                    let a1988 = v3;
                                    let a1989 = 1u64;
                                    nat_add(a1988, a1989)?
                                }, match v2 {
                                    Adt45::C0(_, h1990, _, _) => {
                                        h1990
                                    }
                                }, 1u64, false)))
                            } else {
                                Ok(Some(Adt45::C0({
                                    let a1991 = v3;
                                    let a1992 = 1u64;
                                    nat_add(a1991, a1992)?
                                }, f417(match v2 {
                                    Adt45::C0(_, h1993, _, _) => {
                                        h1993
                                    }
                                }, f83(v0, v3, 1u64)), 0u64, false)))
                            }
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt45>)
        }
    }
}

pub fn f372(v0: Bytes) -> R<u64> {
    let (h1994, _) = f240(Fn39::F418(v0.clone()), {
        let a1995 = v0;
        length_bytes(a1995)
    }, 0u64)?;
    Ok(h1994)
}

pub fn f373(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1999 = {
        let a1997 = v0;
        let a1998 = {
            let (h1996, _) = v1;
            h1996
        };
        index_bytes(a1997, a1998)
    };
    match m1999 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2001 = {
                    let (h2000, _) = v1;
                    h2000
                };
                let a2002 = 1u64;
                nat_add(a2001, a2002)?
            }, {
                let a2006 = {
                    let a2004 = {
                        let (_, h2003) = v1;
                        h2003
                    };
                    let a2005 = 10u64;
                    nat_mul(a2004, a2005)?
                };
                let a2007 = f419(v2)?;
                nat_add(a2006, a2007)?
            })))
        }
    }
}

pub fn f374(v0: List<List<(Bytes, Adt17)>>) -> List<List<(Bytes, Adt17)>> {
    f420(Fn65::F421, List::<List<(Bytes, Adt17)>>::nil(), v0)
}

pub fn f375(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m2008 = v1;
    match m2008 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a2011 = {
                    let a2009 = v3;
                    let a2010 = v0;
                    append_bytes(a2009, a2010)
                };
                let a2012 = v2;
                append_bytes(a2011, a2012)
            })
        }
    }
}

pub fn f376(v0: Adt36, v1: Adt35, v2: List<List<Adt35>>) -> R<Adt36> {
    let m2013 = v2;
    match m2013.uncons() {
        None => {
            Ok(Adt36::C0({
                let a2015 = match v0 {
                    Adt36::C0(h2014, _, _, _, _) => {
                        h2014
                    }
                };
                let a2016 = 1u64;
                nat_add(a2015, a2016)?
            }, List::<List<Adt35>>::nil(), Some(v1), None::<Adt28>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt36::C0({
                let a2018 = match v0.clone() {
                    Adt36::C0(h2017, _, _, _, _) => {
                        h2017
                    }
                };
                let a2019 = 1u64;
                nat_add(a2018, a2019)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt36::C0(_, _, h2020, _, _) => {
                    h2020
                }
            }, None::<Adt28>, false))
        }
    }
}

pub fn f377(v0: List<Adt35>) -> Adt35 {
    f422(Fn66::F423, Adt35::C2, v0)
}

pub fn f378(v0: Adt35, v1: u64) -> R<List<Adt35>> {
    Ok(f424({
        let (_, h2021) = {
            let (h2022, _) = f425(Fn67::F426, {
                let a2023 = v1;
                let a2024 = 1u64;
                nat_add(a2023, a2024)?
            }, (v0, List::<Adt35>::nil()));
            h2022
        };
        h2021
    }))
}

pub fn f379(v0: Adt37, v1: Adt28) -> Adt37 {
    Adt37::C0(match v0.clone() {
        Adt37::C0(h2025, _, _) => {
            h2025
        }
    }, match v0 {
        Adt37::C0(_, h2026, _) => {
            h2026
        }
    }, Some(v1))
}

pub fn f380(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: Bytes, v4: List<Adt35>, v5: u64) -> R<Adt37> {
    if f427(v3.clone()) {
        let m2027 = f428(v3, v4, v5)?;
        match m2027 {
            Ok(v6) => {
                Ok(Adt37::C0(v1, List::cons(v6, match v0 {
                    Adt37::C0(_, h2028, _) => {
                        h2028
                    }
                }), None::<Adt28>))
            }
            Err(v7) => {
                Ok(f379(v0, v7))
            }
        }
    } else {
        let a2029 = v3.clone();
        let a2030 = f201(Str::lit("app"));
        if equal(a2029, a2030) {
            let a2032 = {
                let a2031 = v4.clone();
                length_list(a2031)
            };
            let a2033 = 2u64;
            if nat_lt(a2032, a2033) {
                Ok(f379(v0, f200(f201(Str::lit("`app` has at least a function and one argument")))))
            } else {
                Ok(Adt37::C0(f429(v4, List::cons(Adt38::C1(v2), v1)), match v0 {
                    Adt37::C0(_, h2034, _) => {
                        h2034
                    }
                }, None::<Adt28>))
            }
        } else {
            if f384(v3.clone()) {
                f430(v0, v1, v2, v3, v4, v5)
            } else {
                let a2035 = v3.clone();
                let a2036 = f201(Str::lit("let"));
                if equal(a2035, a2036) {
                    f431(v0, v1, v2, v4, v5)
                } else {
                    Ok(f379(v0, f200(f205(f301(v3), f201(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f381(v0: Adt35) -> Bytes {
    let m2037 = v0;
    match m2037 {
        Adt35::C0(v1) => {
            f301(v1)
        }
        Adt35::C1(r2041) => {
            let v2 = (*r2041).clone();
            let m2038 = v2;
            match m2038 {
                Adt35::C0(_) => {
                    f201(Str::lit("a list"))
                }
                Adt35::C1(_) => {
                    f201(Str::lit("a list"))
                }
                Adt35::C2 => {
                    f201(Str::lit("a list"))
                }
                Adt35::C3(r2040, _) => {
                    let v3 = (*r2040).clone();
                    let m2039 = v3;
                    match m2039 {
                        Adt35::C0(v5) => {
                            f205(f205(f201(Str::lit("`(")), v5), f201(Str::lit(" ...)`")))
                        }
                        Adt35::C1(_) => {
                            f201(Str::lit("a list"))
                        }
                        Adt35::C2 => {
                            f201(Str::lit("a list"))
                        }
                        Adt35::C3(_, _) => {
                            f201(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            f201(Str::lit("a list"))
        }
        Adt35::C3(_, _) => {
            f201(Str::lit("a list"))
        }
    }
}

pub fn f382(v0: List<Adt1>, v1: u64) -> (List<Adt1>, List<Adt1>) {
    let (h2042, _) = f432(Fn68::F433, v1, (List::<Adt1>::nil(), v0));
    h2042
}

pub fn f383(v0: List<Adt1>) -> Adt1 {
    f434(Fn69::F435, Adt1::C9, f386(v0))
}

pub fn f384(v0: Bytes) -> bool {
    let a2043 = v0.clone();
    let a2044 = f201(Str::lit("pi"));
    if equal(a2043, a2044) {
        true
    } else {
        let a2045 = v0;
        let a2046 = f201(Str::lit("lam"));
        equal(a2045, a2046)
    }
}

pub fn f385(v0: List<Adt35>, v1: u64) -> Adt35 {
    let m2049 = {
        let a2047 = v0;
        let a2048 = v1;
        index_list(a2047, a2048)
    };
    match m2049 {
        None => {
            Adt35::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f386(v0: List<Adt1>) -> List<Adt1> {
    f436(Fn70::F437, List::<Adt1>::nil(), v0)
}

pub fn f387(v0: List<Adt35>, v1: List<Adt1>, v2: u64) -> R<Adt1> {
    let (_, h2050) = f438(Fn71::F439(v2), (f386(v1), Adt1::C9), f424(v0))?;
    Ok(h2050)
}

pub fn f388(v0: Adt35, v1: Bytes, _: u64) -> Result<Bytes, Adt28> {
    let m2051 = v0.clone();
    match m2051 {
        Adt35::C0(v3) => {
            Ok::<Bytes, Adt28>(v3)
        }
        Adt35::C1(_) => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
        Adt35::C2 => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
        Adt35::C3(_, _) => {
            Err::<Bytes, Adt28>(f200(f205(f205(f205(f201(Str::lit("expected ")), v1), f201(Str::lit(", found "))), f381(v0))))
        }
    }
}

pub fn f389(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a2052 = v2;
        let a2053 = v0;
        equal(a2052, a2053)
    }
}

pub fn f390(v0: (u64, List<Adt39>), v1: Adt39) -> R<(u64, List<Adt39>)> {
    let v2: Adt52 = f440(Fn72::F441, 40u64, Adt52::C0(v1, {
        let a2055 = {
            let (h2054, _) = v0.clone();
            h2054
        };
        let a2056 = 1u64;
        nat_add(a2055, a2056)?
    }, {
        let (_, h2057) = v0;
        h2057
    }));
    Ok(({
        let a2059 = {
            let (h2058, _) = v0;
            h2058
        };
        let a2060 = 1u64;
        nat_add(a2059, a2060)?
    }, List::cons(match v2.clone() {
        Adt52::C0(h2061, _, _) => {
            h2061
        }
    }, match v2 {
        Adt52::C0(_, _, h2062) => {
            h2062
        }
    })))
}

pub fn f391(v0: Fn59, p2066: u64, p2067: (Adt47, Adt46)) -> (Adt47, Adt46) {
    let m2065 = (p2066, p2067);
    {
        let mut m2065 = m2065;
        loop {
            let step = {
                let (v1, v2) = m2065;
                let m2063 = v1;
                if m2063 == 0 {
                    Err::<(u64, (Adt47, Adt46)), (Adt47, Adt46)>(v2)
                } else {
                    let v3 = m2063 - 1;
                    Ok::<(u64, (Adt47, Adt46)), (Adt47, Adt46)>((v3, {
                        let c2064: Fn59 = v0.clone();
                        c2064.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: (Adt47, Adt46)) -> (Adt47, Adt46) {
    f442(v0)
}

pub fn f393(v0: Fn60, p2071: u64, p2072: (u64, Adt39)) -> R<(u64, Adt39)> {
    let m2070 = (p2071, p2072);
    {
        let mut m2070 = m2070;
        loop {
            let step = {
                let (v1, v2) = m2070;
                let m2068 = v1;
                if m2068 == 0 {
                    Err::<(u64, (u64, Adt39)), R<(u64, Adt39)>>(Ok(v2))
                } else {
                    let v3 = m2068 - 1;
                    Ok::<(u64, (u64, Adt39)), R<(u64, Adt39)>>((v3, {
                        let c2069: Fn60 = v0.clone();
                        c2069.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt39)) -> R<(u64, Adt39)> {
    Ok(({
        let a2074 = {
            let (h2073, _) = v3.clone();
            h2073
        };
        let a2075 = 1u64;
        nat_add(a2074, a2075)?
    }, f338({
        let (_, h2076) = v3.clone();
        h2076
    }, f340(v1, {
        let a2080 = v2;
        let a2081 = {
            let a2078 = 64u64;
            let a2079 = {
                let (h2077, _) = v3.clone();
                h2077
            };
            nat_mul(a2078, a2079)?
        };
        nat_add(a2080, a2081)?
    })?, v0, 64u32, {
        let a2083 = {
            let (h2082, _) = v3;
            h2082
        };
        let a2084 = 0u64;
        if nat_eq(a2083, a2084) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f395() -> Adt39 {
    Adt39::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f396(v0: Bytes, v1: u64) -> R<u32> {
    let a2095 = f443(v0.clone(), v1);
    let a2096 = {
        let a2093 = f444(f443(v0.clone(), {
            let a2085 = v1;
            let a2086 = 1u64;
            nat_add(a2085, a2086)?
        }), 8u32);
        let a2094 = {
            let a2091 = f444(f443(v0.clone(), {
                let a2087 = v1;
                let a2088 = 2u64;
                nat_add(a2087, a2088)?
            }), 16u32);
            let a2092 = f444(f443(v0, {
                let a2089 = v1;
                let a2090 = 3u64;
                nat_add(a2089, a2090)?
            }), 24u32);
            fixed_u32::bit_or(a2091, a2092)
        };
        fixed_u32::bit_or(a2093, a2094)
    };
    Ok(fixed_u32::bit_or(a2095, a2096))
}

pub fn f397(v0: Fn61, p2100: u64, p2101: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2099 = (p2100, p2101);
    {
        let mut m2099 = m2099;
        loop {
            let step = {
                let (v1, v2) = m2099;
                let m2097 = v1;
                if m2097 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2097 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2098: Fn61 = v0.clone();
                        c2098.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2102, _) = v0;
        h2102
    };
    let v2: u32 = {
        let (h2103, _) = {
            let (_, h2104) = v0;
            h2104
        };
        h2103
    };
    let v3: u32 = {
        let (_, h2105) = {
            let (_, h2106) = v0;
            h2106
        };
        h2105
    };
    ({
        let a2107 = v1;
        let a2108 = 2u64;
        let a2109 = 0u64;
        nat_quot(a2107, a2108, a2109)
    }, (f444(v2, 1u32), {
        let a2113 = {
            let a2110 = v1;
            let a2111 = 2u64;
            let a2112 = 0u64;
            nat_rem(a2110, a2111, a2112)
        };
        let a2114 = 1u64;
        if nat_eq(a2113, a2114) {
            let a2115 = v3;
            let a2116 = v2;
            fixed_u32::bit_or(a2115, a2116)
        } else {
            v3
        }
    }))
}

pub fn f399(v0: Adt39, v1: Adt39, v2: bool) -> Adt39 {
    f338(f395(), f445(v0, v1), 0u64, 64u32, {
        let a2117 = 4u32;
        let a2118 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a2117, a2118)
    })
}

pub fn f400(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f446(v1, v0, v2)
}

pub fn f401(v0: Fn62, p2123: u64, p2124: Adt50) -> R<(Adt50, bool)> {
    let m2122 = (p2123, p2124);
    {
        let mut m2122 = m2122;
        loop {
            let step = {
                let (v1, v2) = m2122;
                let m2119 = v1;
                if m2119 == 0 {
                    Err::<(u64, Adt50), R<(Adt50, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2119 - 1;
                    let m2121 = {
                        let c2120: Fn62 = v0.clone();
                        c2120.apply(v2.clone())?
                    };
                    match m2121 {
                        None => {
                            Err::<(u64, Adt50), R<(Adt50, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt50), R<(Adt50, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: u64, v1: u64, v2: Bytes, v3: Bytes, v4: Adt50) -> R<Option<Adt50>> {
    f447(v2, v3, v1, v0, v4)
}

pub fn f403(v0: List<Adt5>) -> List<Adt5> {
    f448(Fn73::F449, List::<Adt5>::nil(), v0)
}

pub fn f404(v0: Adt5) -> bool {
    let m2126 = match v0 {
        Adt5::C0(h2125, _, _, _, _, _, _, _) => {
            h2125
        }
    };
    match m2126 {
        Adt6::C0 => {
            true
        }
        Adt6::C1 => {
            false
        }
        Adt6::C2 => {
            false
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
    }
}

pub fn f405(v0: Adt5, v1: u64, v2: u64) -> R<bool> {
    let a2127 = v2;
    let a2128 = 0u64;
    if nat_eq(a2127, a2128) {
        f450(v0)
    } else {
        let a2129 = v2;
        let a2130 = 1u64;
        if nat_eq(a2129, a2130) {
            f451(v0, v1)
        } else {
            if f450(v0.clone())? {
                f451(v0, v1)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f406(v0: Adt5, v1: u64) -> Bytes {
    f452(f452(f452(f452(f452(f452(f184(Str::lit("`")), match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2131) => {
            h2131
        }
    }), f184(Str::lit("` ("))), f453(v0)), f184(Str::lit(") is not renderer-safe in the "))), f454(v1)), f184(Str::lit(" channel")))
}

pub fn f407(v0: Adt43, v1: List<Adt44>, v2: Adt35, v3: u64) -> R<Adt43> {
    let m2132 = v2.clone();
    match m2132 {
        Adt35::C0(_) => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C1(r2135) => {
            let v4 = (*r2135).clone();
            let m2133 = f378(v4, v3)?;
            match m2133.uncons() {
                None => {
                    Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                }
                Some((v5, v6)) => {
                    let m2134 = v5;
                    match m2134 {
                        Adt35::C0(v7) => {
                            f456(v0, v1, v7, v6, v3)
                        }
                        Adt35::C1(_) => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                        Adt35::C2 => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f455(v0, f200(f201(Str::lit("expected an LRE form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
        Adt35::C3(_, _) => {
            Ok(f455(v0, f200(f205(f201(Str::lit("expected `(`, found ")), f381(v2)))))
        }
    }
}

pub fn f408(v0: Adt43, v1: List<Adt44>, v2: Bytes, v3: u64) -> Adt43 {
    let v4: (List<Adt7>, List<Adt7>) = f457(match v0.clone() {
        Adt43::C0(_, h2136, _) => {
            h2136
        }
    }, v3);
    let v5: List<Adt7> = {
        let (h2137, _) = v4;
        h2137
    };
    let a2138 = v2.clone();
    let a2139 = f201(Str::lit("seq"));
    if equal(a2138, a2139) {
        f458(Adt43::C0(v1.clone(), {
            let (_, h2140) = v4;
            h2140
        }, None::<Adt28>), v1, Adt7::C3(std::rc::Rc::new(f459(v5))))
    } else {
        if f460(v2.clone()) {
            let m2141 = v5;
            match m2141.uncons() {
                None => {
                    f455(v0, f200(f201(Str::lit("missing operand"))))
                }
                Some((v6, _)) => {
                    f458(Adt43::C0(v1.clone(), {
                        let (_, h2142) = v4;
                        h2142
                    }, None::<Adt28>), v1, f461(v2, v6))
                }
            }
        } else {
            let m2143 = v5;
            match m2143.uncons() {
                None => {
                    f455(v0, f200(f201(Str::lit("missing operand"))))
                }
                Some((v8, v9)) => {
                    let m2144 = v9;
                    match m2144.uncons() {
                        None => {
                            f455(v0, f200(f201(Str::lit("missing operand"))))
                        }
                        Some((v10, _)) => {
                            f458(Adt43::C0(v1.clone(), {
                                let (_, h2145) = v4;
                                h2145
                            }, None::<Adt28>), v1, f462(v2, v8, v10))
                        }
                    }
                }
            }
        }
    }
}

pub fn f409(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f410(v0: Fn63, p2150: u64, p2151: (Adt7, List<Adt7>)) -> ((Adt7, List<Adt7>), bool) {
    let m2149 = (p2150, p2151);
    {
        let mut m2149 = m2149;
        loop {
            let step = {
                let (v1, v2) = m2149;
                let m2146 = v1;
                if m2146 == 0 {
                    Err::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v2, false))
                } else {
                    let v3 = m2146 - 1;
                    let m2148 = {
                        let c2147: Fn63 = v0.clone();
                        c2147.apply(v2.clone())
                    };
                    match m2148 {
                        None => {
                            Err::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt7, List<Adt7>)), ((Adt7, List<Adt7>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: (Adt7, List<Adt7>)) -> Option<(Adt7, List<Adt7>)> {
    let m2153 = {
        let (h2152, _) = v0;
        h2152
    };
    match m2153 {
        Adt7::C0(_, _, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C1(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C2(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C3(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C4 => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C5(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C6(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C7(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C8(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C9(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C10(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C11(_, _) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C12(_) => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C13 => {
            None::<(Adt7, List<Adt7>)>
        }
        Adt7::C14(r2155, r2156) => {
            let v1 = (*r2155).clone();
            let v2 = (*r2156).clone();
            Some((v2, List::cons(v1, {
                let (_, h2154) = v0;
                h2154
            })))
        }
    }
}

pub fn f412(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2157 = v0.clone();
    match m2157 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f413(v0: Bytes, v1: Adt7, v2: Adt7, v3: u64) -> R<Option<Bytes>> {
    let m2158 = f463(v0.clone(), v1, v3)?;
    match m2158 {
        None => {
            f463(v0, v2, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f414(v0: Fn64, p2162: Option<Adt4>, p2163: List<Adt4>) -> Option<Adt4> {
    let m2161 = (p2162, p2163);
    {
        let mut m2161 = m2161;
        loop {
            let step = {
                let (v1, v2) = m2161;
                let m2159 = v2;
                match m2159.uncons() {
                    None => {
                        Err::<(Option<Adt4>, List<Adt4>), Option<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt4>, List<Adt4>), Option<Adt4>>(({
                            let c2160: Fn64 = v0.clone();
                            c2160.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: Bytes, v1: Option<Adt4>, v2: Adt4) -> Option<Adt4> {
    let m2164 = v1.clone();
    match m2164 {
        None => {
            let a2166 = match v2.clone() {
                Adt4::C0(h2165, _, _, _, _, _) => {
                    h2165
                }
            };
            let a2167 = v0;
            if equal(a2166, a2167) {
                Some(v2)
            } else {
                None::<Adt4>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f416(v0: u8) -> bool {
    let a2168 = v0;
    let a2169 = 32u8;
    if if if equal(a2168, a2169) {
        true
    } else {
        let a2170 = v0;
        let a2171 = 9u8;
        equal(a2170, a2171)
    } {
        true
    } else {
        let a2172 = v0;
        let a2173 = 10u8;
        equal(a2172, a2173)
    } {
        true
    } else {
        let a2174 = v0;
        let a2175 = 12u8;
        if equal(a2174, a2175) {
            true
        } else {
            let a2176 = v0;
            let a2177 = 13u8;
            equal(a2176, a2177)
        }
    }
}

pub fn f417(v0: Bytes, v1: Bytes) -> Bytes {
    let a2178 = v0;
    let a2179 = v1;
    append_bytes(a2178, a2179)
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2182 = {
        let a2180 = v0;
        let a2181 = v1;
        index_bytes(a2180, a2181)
    };
    match m2182 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f464(v2)? {
                Ok(Some({
                    let a2183 = v1;
                    let a2184 = 1u64;
                    nat_add(a2183, a2184)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f419(v0: u8) -> R<u64> {
    let a2231 = {
        let v1: u8 = v0;
        let a2229 = {
            let a2205 = {
                let a2193 = {
                    let a2187 = {
                        let a2185 = v1;
                        let a2186 = 128u8;
                        fixed_u8::bit_and(a2185, a2186)
                    };
                    let a2188 = 128u8;
                    if equal(a2187, a2188) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2194 = {
                    let a2191 = {
                        let a2189 = v1;
                        let a2190 = 64u8;
                        fixed_u8::bit_and(a2189, a2190)
                    };
                    let a2192 = 64u8;
                    if equal(a2191, a2192) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2193, a2194)?
            };
            let a2206 = {
                let a2203 = {
                    let a2197 = {
                        let a2195 = v1;
                        let a2196 = 32u8;
                        fixed_u8::bit_and(a2195, a2196)
                    };
                    let a2198 = 32u8;
                    if equal(a2197, a2198) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2204 = {
                    let a2201 = {
                        let a2199 = v1;
                        let a2200 = 16u8;
                        fixed_u8::bit_and(a2199, a2200)
                    };
                    let a2202 = 16u8;
                    if equal(a2201, a2202) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2203, a2204)?
            };
            nat_add(a2205, a2206)?
        };
        let a2230 = {
            let a2227 = {
                let a2215 = {
                    let a2209 = {
                        let a2207 = v1;
                        let a2208 = 8u8;
                        fixed_u8::bit_and(a2207, a2208)
                    };
                    let a2210 = 8u8;
                    if equal(a2209, a2210) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2216 = {
                    let a2213 = {
                        let a2211 = v1;
                        let a2212 = 4u8;
                        fixed_u8::bit_and(a2211, a2212)
                    };
                    let a2214 = 4u8;
                    if equal(a2213, a2214) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2215, a2216)?
            };
            let a2228 = {
                let a2225 = {
                    let a2219 = {
                        let a2217 = v1;
                        let a2218 = 2u8;
                        fixed_u8::bit_and(a2217, a2218)
                    };
                    let a2220 = 2u8;
                    if equal(a2219, a2220) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2226 = {
                    let a2223 = {
                        let a2221 = v1;
                        let a2222 = 1u8;
                        fixed_u8::bit_and(a2221, a2222)
                    };
                    let a2224 = 1u8;
                    if equal(a2223, a2224) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2225, a2226)?
            };
            nat_add(a2227, a2228)?
        };
        nat_add(a2229, a2230)?
    };
    let a2232 = 48u64;
    Ok(nat_sub(a2231, a2232))
}

pub fn f420(v0: Fn65, p2236: List<List<(Bytes, Adt17)>>, p2237: List<List<(Bytes, Adt17)>>) -> List<List<(Bytes, Adt17)>> {
    let m2235 = (p2236, p2237);
    {
        let mut m2235 = m2235;
        loop {
            let step = {
                let (v1, v2) = m2235;
                let m2233 = v2;
                match m2233.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt17)>>, List<List<(Bytes, Adt17)>>), List<List<(Bytes, Adt17)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt17)>>, List<List<(Bytes, Adt17)>>), List<List<(Bytes, Adt17)>>>(({
                            let c2234: Fn65 = v0.clone();
                            c2234.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: List<List<(Bytes, Adt17)>>, v1: List<(Bytes, Adt17)>) -> List<List<(Bytes, Adt17)>> {
    List::cons(v1, v0)
}

pub fn f422(v0: Fn66, p2241: Adt35, p2242: List<Adt35>) -> Adt35 {
    let m2240 = (p2241, p2242);
    {
        let mut m2240 = m2240;
        loop {
            let step = {
                let (v1, v2) = m2240;
                let m2238 = v2;
                match m2238.uncons() {
                    None => {
                        Err::<(Adt35, List<Adt35>), Adt35>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt35, List<Adt35>), Adt35>(({
                            let c2239: Fn66 = v0.clone();
                            c2239.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: Adt35, v1: Adt35) -> Adt35 {
    Adt35::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f424(v0: List<Adt35>) -> List<Adt35> {
    f465(Fn74::F466, List::<Adt35>::nil(), v0)
}

pub fn f425(v0: Fn67, p2247: u64, p2248: (Adt35, List<Adt35>)) -> ((Adt35, List<Adt35>), bool) {
    let m2246 = (p2247, p2248);
    {
        let mut m2246 = m2246;
        loop {
            let step = {
                let (v1, v2) = m2246;
                let m2243 = v1;
                if m2243 == 0 {
                    Err::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v2, false))
                } else {
                    let v3 = m2243 - 1;
                    let m2245 = {
                        let c2244: Fn67 = v0.clone();
                        c2244.apply(v2.clone())
                    };
                    match m2245 {
                        None => {
                            Err::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt35, List<Adt35>)), ((Adt35, List<Adt35>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
    f467(v0)
}

pub fn f427(v0: Bytes) -> bool {
    let a2249 = v0.clone();
    let a2250 = f201(Str::lit("sort"));
    if equal(a2249, a2250) {
        true
    } else {
        let a2251 = v0.clone();
        let a2252 = f201(Str::lit("const"));
        if equal(a2251, a2252) {
            true
        } else {
            let a2253 = v0.clone();
            let a2254 = f201(Str::lit("local"));
            if equal(a2253, a2254) {
                true
            } else {
                let a2255 = v0;
                let a2256 = f201(Str::lit("nat"));
                equal(a2255, a2256)
            }
        }
    }
}

pub fn f428(v0: Bytes, v1: List<Adt35>, v2: u64) -> R<Result<Adt1, Adt28>> {
    let a2257 = v0.clone();
    let a2258 = f201(Str::lit("sort"));
    if equal(a2257, a2258) {
        f468(v1, v2)
    } else {
        let a2259 = v0.clone();
        let a2260 = f201(Str::lit("const"));
        if equal(a2259, a2260) {
            f469(v1, v2)
        } else {
            let a2261 = v0.clone();
            let a2262 = f201(Str::lit("local"));
            if equal(a2261, a2262) {
                f470(v1)
            } else {
                let a2263 = v0.clone();
                let a2264 = f201(Str::lit("nat"));
                if equal(a2263, a2264) {
                    f471(v1)
                } else {
                    Ok(Err::<Adt1, Adt28>(f200(f205(f301(v0), f201(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f429(v0: List<Adt35>, v1: List<Adt38>) -> List<Adt38> {
    f472(Fn75::F473, v1, f424(v0))
}

pub fn f430(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: Bytes, v4: List<Adt35>, v5: u64) -> R<Adt37> {
    let a2266 = {
        let a2265 = v4.clone();
        length_list(a2265)
    };
    let a2267 = 2u64;
    if nat_eq(a2266, a2267) {
        let m2268 = f385(v4.clone(), 0u64);
        match m2268 {
            Adt35::C0(_) => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
            Adt35::C1(r2272) => {
                let v6 = (*r2272).clone();
                let v7: List<Adt35> = f378(v6, v5)?;
                let m2269 = v7.clone();
                match m2269.uncons() {
                    None => {
                        Ok(f379(v0, f200(f205(f301(v3), f201(Str::lit(" has at least one binder"))))))
                    }
                    Some((_, _)) => {
                        let m2270 = f474(v7, List::cons(Adt38::C0(f385(v4, 1u64)), List::cons(Adt38::C1(v2), v1)), v5)?;
                        match m2270 {
                            Ok(v10) => {
                                Ok(Adt37::C0(v10, match v0 {
                                    Adt37::C0(_, h2271, _) => {
                                        h2271
                                    }
                                }, None::<Adt28>))
                            }
                            Err(v11) => {
                                Ok(f379(v0, v11))
                            }
                        }
                    }
                }
            }
            Adt35::C2 => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
            Adt35::C3(_, _) => {
                Ok(f379(v0, f200(f201(Str::lit("expected a binder list")))))
            }
        }
    } else {
        Ok(f379(v0, f200(f205(f205(f205(f205(f201(Str::lit("`")), v3.clone()), f201(Str::lit("` is `("))), v3), f201(Str::lit(" (binders...) body)`"))))))
    }
}

pub fn f431(v0: Adt37, v1: List<Adt38>, v2: Adt35, v3: List<Adt35>, v4: u64) -> R<Adt37> {
    let a2274 = {
        let a2273 = v3.clone();
        length_list(a2273)
    };
    let a2275 = 4u64;
    if nat_eq(a2274, a2275) {
        let m2276 = f388(f385(v3.clone(), 0u64), f201(Str::lit("a local ID")), v4);
        match m2276 {
            Ok(v5) => {
                if f193(v5.clone())? {
                    Ok(Adt37::C0(List::cons(Adt38::C0(f385(v3.clone(), 1u64)), List::cons(Adt38::C0(f385(v3.clone(), 2u64)), List::cons(Adt38::C0(f385(v3, 3u64)), List::cons(Adt38::C1(v2), v1)))), match v0 {
                        Adt37::C0(_, h2277, _) => {
                            h2277
                        }
                    }, None::<Adt28>))
                } else {
                    Ok(f379(v0, f200(f205(f301(v5), f201(Str::lit(" is not a valid local ID"))))))
                }
            }
            Err(v6) => {
                Ok(f379(v0, v6))
            }
        }
    } else {
        Ok(f379(v0, f200(f201(Str::lit("`let` is `(let name type value body)`")))))
    }
}

pub fn f432(v0: Fn68, p2282: u64, p2283: (List<Adt1>, List<Adt1>)) -> ((List<Adt1>, List<Adt1>), bool) {
    let m2281 = (p2282, p2283);
    {
        let mut m2281 = m2281;
        loop {
            let step = {
                let (v1, v2) = m2281;
                let m2278 = v1;
                if m2278 == 0 {
                    Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, false))
                } else {
                    let v3 = m2278 - 1;
                    let m2280 = {
                        let c2279: Fn68 = v0.clone();
                        c2279.apply(v2.clone())
                    };
                    match m2280 {
                        None => {
                            Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
    let m2285 = {
        let (_, h2284) = v0;
        h2284
    };
    match m2285.uncons() {
        None => {
            None::<(List<Adt1>, List<Adt1>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h2286, _) = v0;
                h2286
            }), v2))
        }
    }
}

pub fn f434(v0: Fn69, p2290: Adt1, p2291: List<Adt1>) -> Adt1 {
    let m2289 = (p2290, p2291);
    {
        let mut m2289 = m2289;
        loop {
            let step = {
                let (v1, v2) = m2289;
                let m2287 = v2;
                match m2287.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c2288: Fn69 = v0.clone();
                            c2288.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f436(v0: Fn70, p2295: List<Adt1>, p2296: List<Adt1>) -> List<Adt1> {
    let m2294 = (p2295, p2296);
    {
        let mut m2294 = m2294;
        loop {
            let step = {
                let (v1, v2) = m2294;
                let m2292 = v2;
                match m2292.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c2293: Fn70 = v0.clone();
                            c2293.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f438(v0: Fn71, p2300: (List<Adt1>, Adt1), p2301: List<Adt35>) -> R<(List<Adt1>, Adt1)> {
    let m2299 = (p2300, p2301);
    {
        let mut m2299 = m2299;
        loop {
            let step = {
                let (v1, v2) = m2299;
                let m2297 = v2;
                match m2297.uncons() {
                    None => {
                        Err::<((List<Adt1>, Adt1), List<Adt35>), R<(List<Adt1>, Adt1)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt1>, Adt1), List<Adt35>), R<(List<Adt1>, Adt1)>>(({
                            let c2298: Fn71 = v0.clone();
                            c2298.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: u64, v1: (List<Adt1>, Adt1), v2: Adt35) -> R<(List<Adt1>, Adt1)> {
    let m2303 = {
        let (h2302, _) = v1.clone();
        h2302
    };
    match m2303.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            let m2304 = f475(v2, v0)?;
            match m2304 {
                Ok(v5) => {
                    Ok((v4, Adt1::C11(match v5.clone() {
                        Adt54::C0(h2305, _, _) => {
                            h2305
                        }
                    }, match v5 {
                        Adt54::C0(_, h2306, _) => {
                            h2306
                        }
                    }, std::rc::Rc::new(v3), std::rc::Rc::new({
                        let (_, h2307) = v1;
                        h2307
                    }))))
                }
                Err(_) => {
                    Ok((v4, {
                        let (_, h2308) = v1;
                        h2308
                    }))
                }
            }
        }
    }
}

pub fn f440(v0: Fn72, p2312: u64, p2313: Adt52) -> Adt52 {
    let m2311 = (p2312, p2313);
    {
        let mut m2311 = m2311;
        loop {
            let step = {
                let (v1, v2) = m2311;
                let m2309 = v1;
                if m2309 == 0 {
                    Err::<(u64, Adt52), Adt52>(v2)
                } else {
                    let v3 = m2309 - 1;
                    Ok::<(u64, Adt52), Adt52>((v3, {
                        let c2310: Fn72 = v0.clone();
                        c2310.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Adt52) -> Adt52 {
    f476(v0)
}

pub fn f442(v0: (Adt47, Adt46)) -> (Adt47, Adt46) {
    (f477(f478({
        let (h2314, _) = v0.clone();
        h2314
    }, {
        let (_, h2315) = v0.clone();
        h2315
    }), {
        let (_, h2316) = v0.clone();
        h2316
    }), f479({
        let (_, h2317) = v0;
        h2317
    }))
}

pub fn f443(v0: Bytes, v1: u64) -> u32 {
    let m2320 = {
        let a2318 = v0;
        let a2319 = v1;
        index_bytes(a2318, a2319)
    };
    match m2320 {
        None => {
            0u32
        }
        Some(v2) => {
            let m2322 = {
                let a2321 = v2;
                fixed_u32::convert(i128::from(a2321))
            };
            match m2322 {
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

pub fn f444(v0: u32, v1: u32) -> u32 {
    let m2325 = {
        let a2323 = v0;
        let a2324 = v1;
        fixed_u32::shift_left(a2323, a2324)
    };
    match m2325 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f445(v0: Adt39, v1: Adt39) -> Adt46 {
    Adt46::C0(match v0.clone() {
        Adt39::C0(h2326, _, _, _, _, _, _, _) => {
            h2326
        }
    }, match v0.clone() {
        Adt39::C0(_, h2327, _, _, _, _, _, _) => {
            h2327
        }
    }, match v0.clone() {
        Adt39::C0(_, _, h2328, _, _, _, _, _) => {
            h2328
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, h2329, _, _, _, _) => {
            h2329
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, h2330, _, _, _) => {
            h2330
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, _, h2331, _, _) => {
            h2331
        }
    }, match v0.clone() {
        Adt39::C0(_, _, _, _, _, _, h2332, _) => {
            h2332
        }
    }, match v0 {
        Adt39::C0(_, _, _, _, _, _, _, h2333) => {
            h2333
        }
    }, match v1.clone() {
        Adt39::C0(h2334, _, _, _, _, _, _, _) => {
            h2334
        }
    }, match v1.clone() {
        Adt39::C0(_, h2335, _, _, _, _, _, _) => {
            h2335
        }
    }, match v1.clone() {
        Adt39::C0(_, _, h2336, _, _, _, _, _) => {
            h2336
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, h2337, _, _, _, _) => {
            h2337
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, h2338, _, _, _) => {
            h2338
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, _, h2339, _, _) => {
            h2339
        }
    }, match v1.clone() {
        Adt39::C0(_, _, _, _, _, _, h2340, _) => {
            h2340
        }
    }, match v1 {
        Adt39::C0(_, _, _, _, _, _, _, h2341) => {
            h2341
        }
    })
}

pub fn f446(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2342 = v2;
    let a2343 = v1;
    if nat_lt(a2342, a2343) {
        let v3: u64 = f264(v0, v2)?;
        if if f320(v3) {
            true
        } else {
            if f321(v3) {
                true
            } else {
                let a2344 = v3;
                let a2345 = 45u64;
                nat_eq(a2344, a2345)
            }
        } {
            Ok(Some({
                let a2346 = v2;
                let a2347 = 1u64;
                nat_add(a2346, a2347)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f447(v0: Bytes, v1: Bytes, v2: u64, v3: u64, v4: Adt50) -> R<Option<Adt50>> {
    let v5: Adt51 = match v4.clone() {
        Adt50::C0(h2348, _, _, _) => {
            h2348
        }
    };
    let v6: u64 = match v5.clone() {
        Adt51::C0(h2349, _, _) => {
            h2349
        }
    };
    let m2351 = match v4.clone() {
        Adt50::C0(_, _, _, h2350) => {
            h2350
        }
    };
    match m2351 {
        None => {
            let a2352 = v6;
            let a2353 = v2;
            if nat_lt(a2352, a2353) {
                let v8: Adt53 = f480(v0.clone(), v1.clone(), v5.clone())?;
                let m2355 = match v8.clone() {
                    Adt53::C0(_, _, h2354) => {
                        h2354
                    }
                };
                match m2355 {
                    None => {
                        let v10: Adt51 = match v8.clone() {
                            Adt53::C0(_, h2358, _) => {
                                h2358
                            }
                        };
                        let v11: Adt5 = Adt5::C0(match v8 {
                            Adt53::C0(h2359, _, _) => {
                                h2359
                            }
                        }, v6, match v10.clone() {
                            Adt51::C0(h2360, _, _) => {
                                h2360
                            }
                        }, match v5.clone() {
                            Adt51::C0(_, h2361, _) => {
                                h2361
                            }
                        }, match v5 {
                            Adt51::C0(_, _, h2362) => {
                                h2362
                            }
                        }, match v10.clone() {
                            Adt51::C0(_, h2363, _) => {
                                h2363
                            }
                        }, match v10.clone() {
                            Adt51::C0(_, _, h2364) => {
                                h2364
                            }
                        }, f83(v1, v6, {
                            let a2366 = match v10.clone() {
                                Adt51::C0(h2365, _, _) => {
                                    h2365
                                }
                            };
                            let a2367 = v6;
                            nat_sub(a2366, a2367)
                        }));
                        let v12: u64 = {
                            let a2369 = match v4.clone() {
                                Adt50::C0(_, _, h2368, _) => {
                                    h2368
                                }
                            };
                            let a2370 = 1u64;
                            nat_add(a2369, a2370)?
                        };
                        let a2371 = v3;
                        let a2372 = v12;
                        if nat_lt(a2371, a2372) {
                            let v13: Adt48 = f481(v0, v3)?;
                            Ok(Some(Adt50::C0(v10, match v4 {
                                Adt50::C0(_, h2373, _, _) => {
                                    h2373
                                }
                            }, v12, Some(v13))))
                        } else {
                            Ok(Some(Adt50::C0(v10, List::cons(v11, match v4 {
                                Adt50::C0(_, h2374, _, _) => {
                                    h2374
                                }
                            }), v12, None::<Adt48>)))
                        }
                    }
                    Some(v9) => {
                        Ok(Some(Adt50::C0(v5, match v4.clone() {
                            Adt50::C0(_, h2356, _, _) => {
                                h2356
                            }
                        }, match v4 {
                            Adt50::C0(_, _, h2357, _) => {
                                h2357
                            }
                        }, Some(v9))))
                    }
                }
            } else {
                Ok(None::<Adt50>)
            }
        }
        Some(_) => {
            Ok(None::<Adt50>)
        }
    }
}

pub fn f448(v0: Fn73, p2378: List<Adt5>, p2379: List<Adt5>) -> List<Adt5> {
    let m2377 = (p2378, p2379);
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let (v1, v2) = m2377;
                let m2375 = v2;
                match m2375.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c2376: Fn73 = v0.clone();
                            c2376.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f450(v0: Adt5) -> R<bool> {
    let m2381 = match v0.clone() {
        Adt5::C0(h2380, _, _, _, _, _, _, _) => {
            h2380
        }
    };
    match m2381 {
        Adt6::C0 => {
            Ok(false)
        }
        Adt6::C1 => {
            Ok(true)
        }
        Adt6::C2 => {
            Ok(false)
        }
        Adt6::C3 => {
            Ok(false)
        }
        Adt6::C4 => {
            f171(f482(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2382) => {
                    h2382
                }
            })
        }
        Adt6::C5 => {
            Ok(false)
        }
        Adt6::C6 => {
            Ok(true)
        }
    }
}

pub fn f451(v0: Adt5, v1: u64) -> R<bool> {
    let m2384 = match v0.clone() {
        Adt5::C0(h2383, _, _, _, _, _, _, _) => {
            h2383
        }
    };
    match m2384 {
        Adt6::C0 => {
            Ok(false)
        }
        Adt6::C1 => {
            Ok(true)
        }
        Adt6::C2 => {
            let a2385 = 0u64;
            let a2386 = v1;
            Ok(nat_lt(a2385, a2386))
        }
        Adt6::C3 => {
            f171(f483(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2387) => {
                    h2387
                }
            })
        }
        Adt6::C4 => {
            Ok(!f171(f484(), match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2388) => {
                    h2388
                }
            })?)
        }
        Adt6::C5 => {
            f485(match v0 {
                Adt5::C0(_, _, _, _, _, _, _, h2389) => {
                    h2389
                }
            })
        }
        Adt6::C6 => {
            Ok(true)
        }
    }
}

pub fn f452(v0: Bytes, v1: Bytes) -> Bytes {
    let a2390 = v0;
    let a2391 = v1;
    append_bytes(a2390, a2391)
}

pub fn f453(v0: Adt5) -> Bytes {
    let m2393 = match v0 {
        Adt5::C0(h2392, _, _, _, _, _, _, _) => {
            h2392
        }
    };
    match m2393 {
        Adt6::C0 => {
            f184(Str::lit("control"))
        }
        Adt6::C1 => {
            f184(Str::lit("word"))
        }
        Adt6::C2 => {
            f184(Str::lit("numeral"))
        }
        Adt6::C3 => {
            f184(Str::lit("delimiter"))
        }
        Adt6::C4 => {
            f184(Str::lit("ascii-symbol"))
        }
        Adt6::C5 => {
            f184(Str::lit("unicode-symbol"))
        }
        Adt6::C6 => {
            f184(Str::lit("whitespace"))
        }
    }
}

pub fn f454(v0: u64) -> Bytes {
    let a2394 = v0;
    let a2395 = 0u64;
    if nat_eq(a2394, a2395) {
        f184(Str::lit("text"))
    } else {
        let a2396 = v0;
        let a2397 = 1u64;
        if nat_eq(a2396, a2397) {
            f184(Str::lit("math"))
        } else {
            f184(Str::lit("text and math"))
        }
    }
}

pub fn f455(v0: Adt43, v1: Adt28) -> Adt43 {
    Adt43::C0(match v0.clone() {
        Adt43::C0(h2398, _, _) => {
            h2398
        }
    }, match v0 {
        Adt43::C0(_, h2399, _) => {
            h2399
        }
    }, Some(v1))
}

pub fn f456(v0: Adt43, v1: List<Adt44>, v2: Bytes, v3: List<Adt35>, v4: u64) -> R<Adt43> {
    let v5: u64 = {
        let a2400 = v3.clone();
        length_list(a2400)
    };
    let a2401 = v2.clone();
    let a2402 = f201(Str::lit("form"));
    if equal(a2401, a2402) {
        f486(v0, v1, v3, v4)
    } else {
        let a2403 = v2.clone();
        let a2404 = f201(Str::lit("self-form"));
        if equal(a2403, a2404) {
            f487(v0, v1, v3, v4, f201(Str::lit("a form ID")), f201(Str::lit("a self form is `(self-form form-id)`")), 0u64)
        } else {
            let a2405 = v2.clone();
            let a2406 = f201(Str::lit("slot"));
            if equal(a2405, a2406) {
                f488(v0, v1, v3, v4)
            } else {
                let a2407 = v2.clone();
                let a2408 = f201(Str::lit("space"));
                if equal(a2407, a2408) {
                    let a2409 = v5;
                    let a2410 = 0u64;
                    if nat_eq(a2409, a2410) {
                        Ok(f458(v0, v1, Adt7::C4))
                    } else {
                        Ok(f455(v0, f200(f201(Str::lit("`(space)` takes no operand")))))
                    }
                } else {
                    let a2411 = v2.clone();
                    let a2412 = f201(Str::lit("token"));
                    if equal(a2411, a2412) {
                        f487(v0, v1, v3, v4, f201(Str::lit("a renderer-token ID")), f201(Str::lit("a token is `(token renderer-token-id)`")), 1u64)
                    } else {
                        let a2413 = v2.clone();
                        let a2414 = f201(Str::lit("operator-name"));
                        if equal(a2413, a2414) {
                            f487(v0, v1, v3, v4, f201(Str::lit("an operator name")), f201(Str::lit("an operator name is `(operator-name identifier)`")), 2u64)
                        } else {
                            let a2415 = v2.clone();
                            let a2416 = f201(Str::lit("seq"));
                            if equal(a2415, a2416) {
                                let a2417 = v5;
                                let a2418 = 0u64;
                                if nat_eq(a2417, a2418) {
                                    Ok(f455(v0, f200(f201(Str::lit("`seq` has at least one item")))))
                                } else {
                                    Ok(Adt43::C0(f489(v3, List::cons(Adt44::C1(v2, v5), v1)), match v0 {
                                        Adt43::C0(_, h2419, _) => {
                                            h2419
                                        }
                                    }, None::<Adt28>))
                                }
                            } else {
                                if f460(v2.clone()) {
                                    let a2420 = v5;
                                    let a2421 = 1u64;
                                    if nat_eq(a2420, a2421) {
                                        Ok(Adt43::C0(List::cons(Adt44::C0(f385(v3, 0u64)), List::cons(Adt44::C1(v2, 1u64), v1)), match v0 {
                                            Adt43::C0(_, h2422, _) => {
                                                h2422
                                            }
                                        }, None::<Adt28>))
                                    } else {
                                        Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" takes exactly one operand"))))))
                                    }
                                } else {
                                    if f490(v2.clone()) {
                                        let a2423 = v5;
                                        let a2424 = 2u64;
                                        if nat_eq(a2423, a2424) {
                                            Ok(Adt43::C0(List::cons(Adt44::C0(f385(v3.clone(), 0u64)), List::cons(Adt44::C0(f385(v3, 1u64)), List::cons(Adt44::C1(v2, 2u64), v1))), match v0 {
                                                Adt43::C0(_, h2425, _) => {
                                                    h2425
                                                }
                                            }, None::<Adt28>))
                                        } else {
                                            Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" takes exactly two operands"))))))
                                        }
                                    } else {
                                        Ok(f455(v0, f200(f205(f301(v2), f201(Str::lit(" is not an LRE form"))))))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f457(v0: List<Adt7>, v1: u64) -> (List<Adt7>, List<Adt7>) {
    let (h2426, _) = f491(Fn76::F492, v1, (List::<Adt7>::nil(), v0));
    h2426
}

pub fn f458(v0: Adt43, v1: List<Adt44>, v2: Adt7) -> Adt43 {
    Adt43::C0(v1, List::cons(v2, match v0 {
        Adt43::C0(_, h2427, _) => {
            h2427
        }
    }), None::<Adt28>)
}

pub fn f459(v0: List<Adt7>) -> Adt7 {
    f493(Fn77::F494, Adt7::C13, f359(v0))
}

pub fn f460(v0: Bytes) -> bool {
    let a2428 = v0.clone();
    let a2429 = f201(Str::lit("group"));
    if equal(a2428, a2429) {
        true
    } else {
        let a2430 = v0.clone();
        let a2431 = f201(Str::lit("paren"));
        if equal(a2430, a2431) {
            true
        } else {
            let a2432 = v0;
            let a2433 = f201(Str::lit("bracket"));
            equal(a2432, a2433)
        }
    }
}

pub fn f461(v0: Bytes, v1: Adt7) -> Adt7 {
    let a2434 = v0.clone();
    let a2435 = f201(Str::lit("group"));
    if equal(a2434, a2435) {
        Adt7::C6(std::rc::Rc::new(v1))
    } else {
        let a2436 = v0;
        let a2437 = f201(Str::lit("paren"));
        if equal(a2436, a2437) {
            Adt7::C7(std::rc::Rc::new(v1))
        } else {
            Adt7::C8(std::rc::Rc::new(v1))
        }
    }
}

pub fn f462(v0: Bytes, v1: Adt7, v2: Adt7) -> Adt7 {
    let a2438 = v0.clone();
    let a2439 = f201(Str::lit("sub"));
    if equal(a2438, a2439) {
        Adt7::C9(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
    } else {
        let a2440 = v0;
        let a2441 = f201(Str::lit("sup"));
        if equal(a2440, a2441) {
            Adt7::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        } else {
            Adt7::C11(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
    }
}

pub fn f463(v0: Bytes, v1: Adt7, v2: u64) -> R<Option<Bytes>> {
    if f495(v1.clone(), v2)? {
        Ok(Some(f205(f301(v0), f201(Str::lit(" operand renders nothing")))))
    } else {
        let a2442 = v0.clone();
        let a2443 = f201(Str::lit("frac"));
        if if equal(a2442, a2443) {
            false
        } else {
            f496(v1)
        } {
            Ok(Some(f205(f301(v0), f201(Str::lit(" operand is a bare script; wrap nested scripts in `(group ...)`")))))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f464(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2488 = {
            let a2464 = {
                let a2452 = {
                    let a2446 = {
                        let a2444 = v1;
                        let a2445 = 128u8;
                        fixed_u8::bit_and(a2444, a2445)
                    };
                    let a2447 = 128u8;
                    if equal(a2446, a2447) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2453 = {
                    let a2450 = {
                        let a2448 = v1;
                        let a2449 = 64u8;
                        fixed_u8::bit_and(a2448, a2449)
                    };
                    let a2451 = 64u8;
                    if equal(a2450, a2451) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2452, a2453)?
            };
            let a2465 = {
                let a2462 = {
                    let a2456 = {
                        let a2454 = v1;
                        let a2455 = 32u8;
                        fixed_u8::bit_and(a2454, a2455)
                    };
                    let a2457 = 32u8;
                    if equal(a2456, a2457) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2463 = {
                    let a2460 = {
                        let a2458 = v1;
                        let a2459 = 16u8;
                        fixed_u8::bit_and(a2458, a2459)
                    };
                    let a2461 = 16u8;
                    if equal(a2460, a2461) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2462, a2463)?
            };
            nat_add(a2464, a2465)?
        };
        let a2489 = {
            let a2486 = {
                let a2474 = {
                    let a2468 = {
                        let a2466 = v1;
                        let a2467 = 8u8;
                        fixed_u8::bit_and(a2466, a2467)
                    };
                    let a2469 = 8u8;
                    if equal(a2468, a2469) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2475 = {
                    let a2472 = {
                        let a2470 = v1;
                        let a2471 = 4u8;
                        fixed_u8::bit_and(a2470, a2471)
                    };
                    let a2473 = 4u8;
                    if equal(a2472, a2473) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2474, a2475)?
            };
            let a2487 = {
                let a2484 = {
                    let a2478 = {
                        let a2476 = v1;
                        let a2477 = 2u8;
                        fixed_u8::bit_and(a2476, a2477)
                    };
                    let a2479 = 2u8;
                    if equal(a2478, a2479) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2485 = {
                    let a2482 = {
                        let a2480 = v1;
                        let a2481 = 1u8;
                        fixed_u8::bit_and(a2480, a2481)
                    };
                    let a2483 = 1u8;
                    if equal(a2482, a2483) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2484, a2485)?
            };
            nat_add(a2486, a2487)?
        };
        nat_add(a2488, a2489)?
    };
    let a2490 = 48u64;
    let a2491 = v2;
    if nat_le(a2490, a2491) {
        let a2492 = v2;
        let a2493 = 57u64;
        Ok(nat_le(a2492, a2493))
    } else {
        Ok(false)
    }
}

pub fn f465(v0: Fn74, p2497: List<Adt35>, p2498: List<Adt35>) -> List<Adt35> {
    let m2496 = (p2497, p2498);
    {
        let mut m2496 = m2496;
        loop {
            let step = {
                let (v1, v2) = m2496;
                let m2494 = v2;
                match m2494.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt35>), List<Adt35>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt35>, List<Adt35>), List<Adt35>>(({
                            let c2495: Fn74 = v0.clone();
                            c2495.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: List<Adt35>, v1: Adt35) -> List<Adt35> {
    List::cons(v1, v0)
}

pub fn f467(v0: (Adt35, List<Adt35>)) -> Option<(Adt35, List<Adt35>)> {
    let m2500 = {
        let (h2499, _) = v0;
        h2499
    };
    match m2500 {
        Adt35::C0(_) => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C1(_) => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C2 => {
            None::<(Adt35, List<Adt35>)>
        }
        Adt35::C3(r2502, r2503) => {
            let v3 = (*r2502).clone();
            let v4 = (*r2503).clone();
            Some((v4, List::cons(v3, {
                let (_, h2501) = v0;
                h2501
            })))
        }
    }
}

pub fn f468(v0: List<Adt35>, v1: u64) -> R<Result<Adt1, Adt28>> {
    let a2505 = {
        let a2504 = v0.clone();
        length_list(a2504)
    };
    let a2506 = 1u64;
    if nat_eq(a2505, a2506) {
        let m2507 = f385(v0, 0u64);
        match m2507 {
            Adt35::C0(v2) => {
                let a2508 = v2;
                let a2509 = f201(Str::lit("prop"));
                if equal(a2508, a2509) {
                    Ok(Ok::<Adt1, Adt28>(Adt1::C0))
                } else {
                    Ok(Err::<Adt1, Adt28>(f497()))
                }
            }
            Adt35::C1(r2517) => {
                let v3 = (*r2517).clone();
                let v4: List<Adt35> = f378(v3, v1)?;
                let a2511 = {
                    let a2510 = v4.clone();
                    length_list(a2510)
                };
                let a2512 = 2u64;
                if nat_eq(a2511, a2512) {
                    let m2513 = f385(v4.clone(), 0u64);
                    match m2513 {
                        Adt35::C0(v5) => {
                            let a2514 = v5;
                            let a2515 = f201(Str::lit("type"));
                            if equal(a2514, a2515) {
                                let m2516 = f498(f385(v4, 1u64), v1)?;
                                match m2516 {
                                    Ok(v6) => {
                                        Ok(Ok::<Adt1, Adt28>(Adt1::C1(v6)))
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt1, Adt28>(v7))
                                    }
                                }
                            } else {
                                Ok(Err::<Adt1, Adt28>(f497()))
                            }
                        }
                        Adt35::C1(_) => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                        Adt35::C2 => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                        Adt35::C3(_, _) => {
                            Ok(Err::<Adt1, Adt28>(f497()))
                        }
                    }
                } else {
                    Ok(Err::<Adt1, Adt28>(f497()))
                }
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f497()))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f497()))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f497()))
    }
}

pub fn f469(v0: List<Adt35>, v1: u64) -> R<Result<Adt1, Adt28>> {
    let v2: u64 = {
        let a2518 = v0.clone();
        length_list(a2518)
    };
    let m2519 = f385(v0.clone(), 0u64);
    match m2519 {
        Adt35::C0(v3) => {
            let a2520 = v2;
            let a2521 = 1u64;
            if nat_eq(a2520, a2521) {
                let m2522 = f231(v3)?;
                match m2522 {
                    Ok(v4) => {
                        Ok(Ok::<Adt1, Adt28>(Adt1::C2({
                            let (h2523, _) = v4;
                            h2523
                        }, {
                            let (_, h2524) = v4;
                            h2524
                        }, Adt2::C5)))
                    }
                    Err(v5) => {
                        Ok(Err::<Adt1, Adt28>(v5))
                    }
                }
            } else {
                let a2525 = v2;
                let a2526 = 2u64;
                if nat_eq(a2525, a2526) {
                    let m2527 = f385(v0, 1u64);
                    match m2527 {
                        Adt35::C0(_) => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                        Adt35::C1(r2533) => {
                            let v6 = (*r2533).clone();
                            let v7: List<Adt35> = f378(v6, v1)?;
                            let m2528 = v7.clone();
                            match m2528.uncons() {
                                None => {
                                    Ok(Err::<Adt1, Adt28>(f499()))
                                }
                                Some((_, _)) => {
                                    let m2529 = f231(v3)?;
                                    match m2529 {
                                        Ok(v10) => {
                                            let m2530 = f500(v7, v1)?;
                                            match m2530 {
                                                Ok(v11) => {
                                                    Ok(Ok::<Adt1, Adt28>(Adt1::C2({
                                                        let (h2531, _) = v10;
                                                        h2531
                                                    }, {
                                                        let (_, h2532) = v10;
                                                        h2532
                                                    }, v11)))
                                                }
                                                Err(v12) => {
                                                    Ok(Err::<Adt1, Adt28>(v12))
                                                }
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<Adt1, Adt28>(v13))
                                        }
                                    }
                                }
                            }
                        }
                        Adt35::C2 => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                        Adt35::C3(_, _) => {
                            Ok(Err::<Adt1, Adt28>(f499()))
                        }
                    }
                } else {
                    Ok(Err::<Adt1, Adt28>(f499()))
                }
            }
        }
        Adt35::C1(_) => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
        Adt35::C2 => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
        Adt35::C3(_, _) => {
            Ok(Err::<Adt1, Adt28>(f499()))
        }
    }
}

pub fn f470(v0: List<Adt35>) -> R<Result<Adt1, Adt28>> {
    let a2535 = {
        let a2534 = v0.clone();
        length_list(a2534)
    };
    let a2536 = 1u64;
    if nat_eq(a2535, a2536) {
        let m2537 = f385(v0, 0u64);
        match m2537 {
            Adt35::C0(v1) => {
                if f193(v1.clone())? {
                    Ok(Ok::<Adt1, Adt28>(Adt1::C3(v1)))
                } else {
                    Ok(Err::<Adt1, Adt28>(f200(f205(f301(v1), f201(Str::lit(" is not a valid local ID"))))))
                }
            }
            Adt35::C1(_) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a local is `(local id)`")))))
    }
}

pub fn f471(v0: List<Adt35>) -> R<Result<Adt1, Adt28>> {
    let a2539 = {
        let a2538 = v0.clone();
        length_list(a2538)
    };
    let a2540 = 1u64;
    if nat_eq(a2539, a2540) {
        let m2541 = f385(v0, 0u64);
        match m2541 {
            Adt35::C0(v1) => {
                let m2542 = f501(v1.clone())?;
                match m2542 {
                    None => {
                        Ok(Ok::<Adt1, Adt28>(Adt1::C8(v1)))
                    }
                    Some(v2) => {
                        Ok(Err::<Adt1, Adt28>(v2))
                    }
                }
            }
            Adt35::C1(_) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt35::C2 => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt35::C3(_, _) => {
                Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
            }
        }
    } else {
        Ok(Err::<Adt1, Adt28>(f200(f201(Str::lit("a nat is `(nat decimal)`")))))
    }
}

pub fn f472(v0: Fn75, p2546: List<Adt38>, p2547: List<Adt35>) -> List<Adt38> {
    let m2545 = (p2546, p2547);
    {
        let mut m2545 = m2545;
        loop {
            let step = {
                let (v1, v2) = m2545;
                let m2543 = v2;
                match m2543.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt35>), List<Adt38>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt38>, List<Adt35>), List<Adt38>>(({
                            let c2544: Fn75 = v0.clone();
                            c2544.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: List<Adt38>, v1: Adt35) -> List<Adt38> {
    List::cons(Adt38::C0(v1), v0)
}

pub fn f474(v0: List<Adt35>, v1: List<Adt38>, v2: u64) -> R<Result<List<Adt38>, Adt28>> {
    f502(Fn78::F503(v2), Ok::<List<Adt38>, Adt28>(v1), f424(v0))
}

pub fn f475(v0: Adt35, v1: u64) -> R<Result<Adt54, Adt28>> {
    let m2548 = v0.clone();
    match m2548 {
        Adt35::C0(_) => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
        Adt35::C1(r2555) => {
            let v2 = (*r2555).clone();
            let v3: List<Adt35> = f378(v2, v1)?;
            let a2550 = {
                let a2549 = v3.clone();
                length_list(a2549)
            };
            let a2551 = 3u64;
            if nat_eq(a2550, a2551) {
                let m2552 = f388(f385(v3.clone(), 0u64), f201(Str::lit("a binder mode")), v1);
                match m2552 {
                    Ok(v4) => {
                        let m2553 = f504(v4.clone());
                        match m2553 {
                            None => {
                                Ok(Err::<Adt54, Adt28>(f200(f205(f301(v4), f201(Str::lit(" is not a binder mode"))))))
                            }
                            Some(v5) => {
                                let m2554 = f388(f385(v3.clone(), 1u64), f201(Str::lit("a local ID")), v1);
                                match m2554 {
                                    Ok(v6) => {
                                        if f193(v6.clone())? {
                                            Ok(Ok::<Adt54, Adt28>(Adt54::C0(v5, v6, f385(v3, 2u64))))
                                        } else {
                                            Ok(Err::<Adt54, Adt28>(f200(f205(f301(v6), f201(Str::lit(" is not a valid local ID"))))))
                                        }
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt54, Adt28>(v7))
                                    }
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<Adt54, Adt28>(v8))
                    }
                }
            } else {
                Ok(Err::<Adt54, Adt28>(f200(f201(Str::lit("a binder is `(mode name type)`")))))
            }
        }
        Adt35::C2 => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
        Adt35::C3(_, _) => {
            Ok(Err::<Adt54, Adt28>(f200(f205(f201(Str::lit("expected a binder, found ")), f381(v0)))))
        }
    }
}

pub fn f476(v0: Adt52) -> Adt52 {
    let a2560 = {
        let a2557 = match v0.clone() {
            Adt52::C0(_, h2556, _) => {
                h2556
            }
        };
        let a2558 = 2u64;
        let a2559 = 0u64;
        nat_rem(a2557, a2558, a2559)
    };
    let a2561 = 0u64;
    if nat_eq(a2560, a2561) {
        let m2563 = match v0.clone() {
            Adt52::C0(_, _, h2562) => {
                h2562
            }
        };
        match m2563.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt52::C0(f399(v1, match v0.clone() {
                    Adt52::C0(h2564, _, _) => {
                        h2564
                    }
                }, false), {
                    let a2566 = match v0 {
                        Adt52::C0(_, h2565, _) => {
                            h2565
                        }
                    };
                    let a2567 = 2u64;
                    let a2568 = 0u64;
                    nat_quot(a2566, a2567, a2568)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f477(v0: Adt47, v1: Adt46) -> Adt47 {
    let v2: Adt55 = f505(match v0.clone() {
        Adt47::C0(h2569, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2569
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, h2570, _, _, _, _, _, _, _, _, _, _) => {
            h2570
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, h2571, _, _, _, _, _) => {
            h2571
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2572) => {
            h2572
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, h2573, _, _, _, _, _, _, _) => {
            h2573
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, h2574, _, _, _, _, _, _) => {
            h2574
        }
    });
    let v3: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, h2575, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2575
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, h2576, _, _, _, _, _, _, _, _, _) => {
            h2576
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h2577, _, _, _, _) => {
            h2577
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2578, _, _, _) => {
            h2578
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, h2579, _, _, _, _, _) => {
            h2579
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, h2580, _, _, _, _) => {
            h2580
        }
    });
    let v4: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, h2581, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2581
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, h2582, _, _, _, _, _, _, _, _) => {
            h2582
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, h2583, _, _, _, _, _, _, _) => {
            h2583
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2584, _, _) => {
            h2584
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2585, _, _, _) => {
            h2585
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2586, _, _) => {
            h2586
        }
    });
    let v5: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, _, h2587, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2587
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, h2588, _, _, _, _, _, _, _, _, _, _, _) => {
            h2588
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, h2589, _, _, _, _, _, _) => {
            h2589
        }
    }, match v0 {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2590, _) => {
            h2590
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2591, _) => {
            h2591
        }
    }, match v1 {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2592) => {
            h2592
        }
    });
    Adt47::C0(match v2.clone() {
        Adt55::C0(h2593, _, _, _) => {
            h2593
        }
    }, match v3.clone() {
        Adt55::C0(h2594, _, _, _) => {
            h2594
        }
    }, match v4.clone() {
        Adt55::C0(h2595, _, _, _) => {
            h2595
        }
    }, match v5.clone() {
        Adt55::C0(h2596, _, _, _) => {
            h2596
        }
    }, match v5.clone() {
        Adt55::C0(_, h2597, _, _) => {
            h2597
        }
    }, match v2.clone() {
        Adt55::C0(_, h2598, _, _) => {
            h2598
        }
    }, match v3.clone() {
        Adt55::C0(_, h2599, _, _) => {
            h2599
        }
    }, match v4.clone() {
        Adt55::C0(_, h2600, _, _) => {
            h2600
        }
    }, match v4.clone() {
        Adt55::C0(_, _, h2601, _) => {
            h2601
        }
    }, match v5.clone() {
        Adt55::C0(_, _, h2602, _) => {
            h2602
        }
    }, match v2.clone() {
        Adt55::C0(_, _, h2603, _) => {
            h2603
        }
    }, match v3.clone() {
        Adt55::C0(_, _, h2604, _) => {
            h2604
        }
    }, match v3 {
        Adt55::C0(_, _, _, h2605) => {
            h2605
        }
    }, match v4 {
        Adt55::C0(_, _, _, h2606) => {
            h2606
        }
    }, match v5 {
        Adt55::C0(_, _, _, h2607) => {
            h2607
        }
    }, match v2 {
        Adt55::C0(_, _, _, h2608) => {
            h2608
        }
    })
}

pub fn f478(v0: Adt47, v1: Adt46) -> Adt47 {
    let v2: Adt55 = f505(match v0.clone() {
        Adt47::C0(h2609, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2609
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, h2610, _, _, _, _, _, _, _, _, _, _, _) => {
            h2610
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, h2611, _, _, _, _, _, _, _) => {
            h2611
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2612, _, _, _) => {
            h2612
        }
    }, match v1.clone() {
        Adt46::C0(h2613, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2613
        }
    }, match v1.clone() {
        Adt46::C0(_, h2614, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2614
        }
    });
    let v3: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, h2615, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2615
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, h2616, _, _, _, _, _, _, _, _, _, _) => {
            h2616
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, h2617, _, _, _, _, _, _) => {
            h2617
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2618, _, _) => {
            h2618
        }
    }, match v1.clone() {
        Adt46::C0(_, _, h2619, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2619
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, h2620, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2620
        }
    });
    let v4: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, h2621, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2621
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, h2622, _, _, _, _, _, _, _, _, _) => {
            h2622
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, h2623, _, _, _, _, _) => {
            h2623
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2624, _) => {
            h2624
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, h2625, _, _, _, _, _, _, _, _, _, _, _) => {
            h2625
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, h2626, _, _, _, _, _, _, _, _, _, _) => {
            h2626
        }
    });
    let v5: Adt55 = f505(match v0.clone() {
        Adt47::C0(_, _, _, h2627, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2627
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, h2628, _, _, _, _, _, _, _, _) => {
            h2628
        }
    }, match v0.clone() {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, h2629, _, _, _, _) => {
            h2629
        }
    }, match v0 {
        Adt47::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2630) => {
            h2630
        }
    }, match v1.clone() {
        Adt46::C0(_, _, _, _, _, _, h2631, _, _, _, _, _, _, _, _, _) => {
            h2631
        }
    }, match v1 {
        Adt46::C0(_, _, _, _, _, _, _, h2632, _, _, _, _, _, _, _, _) => {
            h2632
        }
    });
    Adt47::C0(match v2.clone() {
        Adt55::C0(h2633, _, _, _) => {
            h2633
        }
    }, match v3.clone() {
        Adt55::C0(h2634, _, _, _) => {
            h2634
        }
    }, match v4.clone() {
        Adt55::C0(h2635, _, _, _) => {
            h2635
        }
    }, match v5.clone() {
        Adt55::C0(h2636, _, _, _) => {
            h2636
        }
    }, match v2.clone() {
        Adt55::C0(_, h2637, _, _) => {
            h2637
        }
    }, match v3.clone() {
        Adt55::C0(_, h2638, _, _) => {
            h2638
        }
    }, match v4.clone() {
        Adt55::C0(_, h2639, _, _) => {
            h2639
        }
    }, match v5.clone() {
        Adt55::C0(_, h2640, _, _) => {
            h2640
        }
    }, match v2.clone() {
        Adt55::C0(_, _, h2641, _) => {
            h2641
        }
    }, match v3.clone() {
        Adt55::C0(_, _, h2642, _) => {
            h2642
        }
    }, match v4.clone() {
        Adt55::C0(_, _, h2643, _) => {
            h2643
        }
    }, match v5.clone() {
        Adt55::C0(_, _, h2644, _) => {
            h2644
        }
    }, match v2 {
        Adt55::C0(_, _, _, h2645) => {
            h2645
        }
    }, match v3 {
        Adt55::C0(_, _, _, h2646) => {
            h2646
        }
    }, match v4 {
        Adt55::C0(_, _, _, h2647) => {
            h2647
        }
    }, match v5 {
        Adt55::C0(_, _, _, h2648) => {
            h2648
        }
    })
}

pub fn f479(v0: Adt46) -> Adt46 {
    Adt46::C0(match v0.clone() {
        Adt46::C0(_, _, h2649, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2649
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, h2650, _, _, _, _, _, _, _, _, _) => {
            h2650
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, h2651, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2651
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, h2652, _, _, _, _, _) => {
            h2652
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, h2653, _, _, _, _, _, _, _, _) => {
            h2653
        }
    }, match v0.clone() {
        Adt46::C0(h2654, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2654
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, h2655, _, _, _, _, _, _, _, _, _, _, _) => {
            h2655
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2656, _, _) => {
            h2656
        }
    }, match v0.clone() {
        Adt46::C0(_, h2657, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2657
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, h2658, _, _, _, _) => {
            h2658
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2659, _, _, _) => {
            h2659
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, h2660, _, _, _, _, _, _, _, _, _, _) => {
            h2660
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, h2661, _, _, _, _, _, _) => {
            h2661
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2662, _) => {
            h2662
        }
    }, match v0.clone() {
        Adt46::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2663) => {
            h2663
        }
    }, match v0 {
        Adt46::C0(_, _, _, _, _, _, _, _, h2664, _, _, _, _, _, _, _) => {
            h2664
        }
    })
}

pub fn f480(v0: Bytes, v1: Bytes, v2: Adt51) -> R<Adt53> {
    let v3: u64 = {
        let a2665 = v1.clone();
        length_bytes(a2665)
    };
    let v4: u64 = match v2.clone() {
        Adt51::C0(h2666, _, _) => {
            h2666
        }
    };
    let v5: u8 = f506(v1.clone(), v4);
    let v6: Adt51 = f507(v1.clone(), 0u64, v2.clone())?;
    let v7: Adt51 = f507(v1.clone(), 1u64, v2.clone())?;
    let v8: Adt51 = f507(v1.clone(), 2u64, v2.clone())?;
    let v9: Adt51 = f508(v1.clone(), v2.clone())?;
    let v10: u64 = match v9.clone() {
        Adt51::C0(h2667, _, _) => {
            h2667
        }
    };
    let v11: u8 = f506(v1.clone(), v10);
    let v12: bool = {
        let a2668 = v10;
        let a2669 = v3;
        nat_lt(a2668, a2669)
    };
    let v13: bool = f509(v11);
    let v14: bool = f510(v11, 0u8, 127u8);
    let v15: Adt51 = f507(v1.clone(), 1u64, v9.clone())?;
    let v16: Adt51 = f508(v1.clone(), v9.clone())?;
    let v17: bool = f509(v5);
    let v18: bool = f511(v5);
    let v19: bool = f510(v5, 32u8, 126u8);
    let v20: bool = f510(v5, 0u8, 127u8);
    let v21: bool = {
        let a2670 = v5;
        let a2671 = 123u8;
        if equal(a2670, a2671) {
            true
        } else {
            let a2672 = v5;
            let a2673 = 125u8;
            if equal(a2672, a2673) {
                true
            } else {
                let a2674 = v5;
                let a2675 = 40u8;
                if equal(a2674, a2675) {
                    true
                } else {
                    let a2676 = v5;
                    let a2677 = 41u8;
                    if equal(a2676, a2677) {
                        true
                    } else {
                        let a2678 = v5;
                        let a2679 = 91u8;
                        if equal(a2678, a2679) {
                            true
                        } else {
                            let a2680 = v5;
                            let a2681 = 93u8;
                            equal(a2680, a2681)
                        }
                    }
                }
            }
        }
    };
    let a2682 = v5;
    let a2683 = 32u8;
    if if equal(a2682, a2683) {
        true
    } else {
        let a2684 = v5;
        let a2685 = 10u8;
        equal(a2684, a2685)
    } {
        Ok(Adt53::C0(Adt6::C6, v6, None::<Adt48>))
    } else {
        let a2686 = v5;
        let a2687 = 92u8;
        if equal(a2686, a2687) {
            if if v12 {
                v13
            } else {
                false
            } {
                Ok(Adt53::C0(Adt6::C0, v15, None::<Adt48>))
            } else {
                if if v12 {
                    v14
                } else {
                    false
                } {
                    Ok(Adt53::C0(Adt6::C0, v16, None::<Adt48>))
                } else {
                    let v22: Adt48 = f512(v0, v2, v9.clone());
                    Ok(Adt53::C0(Adt6::C0, v9, Some(v22)))
                }
            }
        } else {
            if v17 {
                Ok(Adt53::C0(Adt6::C1, v7, None::<Adt48>))
            } else {
                if v18 {
                    Ok(Adt53::C0(Adt6::C2, v8, None::<Adt48>))
                } else {
                    if v21 {
                        Ok(Adt53::C0(Adt6::C3, v9, None::<Adt48>))
                    } else {
                        if v19 {
                            Ok(Adt53::C0(Adt6::C4, v9, None::<Adt48>))
                        } else {
                            if v20 {
                                let v23: Adt48 = f513(v0, v1, v2.clone())?;
                                Ok(Adt53::C0(Adt6::C0, v2, Some(v23)))
                            } else {
                                Ok(Adt53::C0(Adt6::C5, v9, None::<Adt48>))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f481(v0: Bytes, v1: u64) -> R<Adt48> {
    Ok(Adt48::C0(f514(Str::lit("LLS8002")), f515(f515(f515(f514(Str::lit("max_primitive_atoms exceeded: configured ")), f516(v1)?), f514(Str::lit(", scanning "))), v0), None::<Adt49>, None::<Bytes>))
}

pub fn f482() -> List<Bytes> {
    List::cons(f184(Str::lit(":")), List::cons(f184(Str::lit("-")), List::cons(f184(Str::lit(",")), List::cons(f184(Str::lit(".")), List::cons(f184(Str::lit(";")), List::cons(f184(Str::lit("'")), List::<Bytes>::nil()))))))
}

pub fn f483() -> List<Bytes> {
    List::cons(f184(Str::lit("(")), List::cons(f184(Str::lit(")")), List::cons(f184(Str::lit("[")), List::cons(f184(Str::lit("]")), List::<Bytes>::nil()))))
}

pub fn f484() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x60]), List::<Bytes>::nil()))))))))))))
}

pub fn f485(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2688 = v0.clone();
        length_bytes(a2688)
    };
    let a2692 = v1;
    let a2693 = {
        let (h2689, _) = f240(Fn39::F517(v1, v0), {
            let a2690 = v1;
            let a2691 = 1u64;
            nat_add(a2690, a2691)?
        }, 0u64)?;
        h2689
    };
    Ok(nat_le(a2692, a2693))
}

pub fn f486(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64) -> R<Adt43> {
    let a2695 = {
        let a2694 = v2.clone();
        length_list(a2694)
    };
    let a2696 = 2u64;
    if nat_eq(a2695, a2696) {
        let m2697 = f388(f385(v2.clone(), 0u64), f201(Str::lit("a qualified ID")), v3);
        match m2697 {
            Ok(v4) => {
                let m2698 = f231(v4)?;
                match m2698 {
                    Ok(v5) => {
                        let m2699 = f388(f385(v2, 1u64), f201(Str::lit("a form ID")), v3);
                        match m2699 {
                            Ok(v6) => {
                                if f284(v6.clone())? {
                                    Ok(f458(v0, v1, Adt7::C0({
                                        let (h2700, _) = v5;
                                        h2700
                                    }, {
                                        let (_, h2701) = v5;
                                        h2701
                                    }, v6)))
                                } else {
                                    Ok(f455(v0, f200(f205(f301(v6), f201(Str::lit(" is not a form ID"))))))
                                }
                            }
                            Err(v7) => {
                                Ok(f455(v0, v7))
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(f455(v0, v8))
                    }
                }
            }
            Err(v9) => {
                Ok(f455(v0, v9))
            }
        }
    } else {
        Ok(f455(v0, f200(f201(Str::lit("a form reference is `(form qualified-id form-id)`")))))
    }
}

pub fn f487(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64, v4: Bytes, v5: Bytes, v6: u64) -> R<Adt43> {
    let a2703 = {
        let a2702 = v2.clone();
        length_list(a2702)
    };
    let a2704 = 1u64;
    if nat_eq(a2703, a2704) {
        let m2705 = f388(f385(v2, 0u64), v4, v3);
        match m2705 {
            Ok(v7) => {
                let a2706 = v6;
                let a2707 = 0u64;
                if nat_eq(a2706, a2707) {
                    if f284(v7.clone())? {
                        Ok(f458(v0, v1, Adt7::C1(v7)))
                    } else {
                        Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not a form ID"))))))
                    }
                } else {
                    let a2708 = v6;
                    let a2709 = 1u64;
                    if nat_eq(a2708, a2709) {
                        if f345(v7.clone())? {
                            Ok(f458(v0, v1, Adt7::C5(v7)))
                        } else {
                            Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not a renderer-token ID"))))))
                        }
                    } else {
                        if f518(v7.clone())? {
                            Ok(f458(v0, v1, Adt7::C12(v7)))
                        } else {
                            Ok(f455(v0, f200(f205(f301(v7), f201(Str::lit(" is not an operator name"))))))
                        }
                    }
                }
            }
            Err(v8) => {
                Ok(f455(v0, v8))
            }
        }
    } else {
        Ok(f455(v0, f200(v5)))
    }
}

pub fn f488(v0: Adt43, v1: List<Adt44>, v2: List<Adt35>, v3: u64) -> R<Adt43> {
    let a2711 = {
        let a2710 = v2.clone();
        length_list(a2710)
    };
    let a2712 = 1u64;
    if nat_eq(a2711, a2712) {
        let m2713 = f388(f385(v2, 0u64), f201(Str::lit("a slot index")), v3);
        match m2713 {
            Ok(v4) => {
                let m2714 = f501(v4.clone())?;
                match m2714 {
                    None => {
                        let v6: u64 = f519(v4.clone())?;
                        let a2715 = 4294967295u64;
                        let a2716 = v6;
                        if nat_lt(a2715, a2716) {
                            Ok(f455(v0, f200(f205(f301(v4), f201(Str::lit(" is not a slot index"))))))
                        } else {
                            Ok(f458(v0, v1, Adt7::C2(v6)))
                        }
                    }
                    Some(_) => {
                        Ok(f455(v0, f200(f205(f301(v4), f201(Str::lit(" is not a slot index"))))))
                    }
                }
            }
            Err(v7) => {
                Ok(f455(v0, v7))
            }
        }
    } else {
        Ok(f455(v0, f200(f201(Str::lit("a slot is `(slot decimal)`")))))
    }
}

pub fn f489(v0: List<Adt35>, v1: List<Adt44>) -> List<Adt44> {
    f520(Fn79::F521, v1, f424(v0))
}

pub fn f490(v0: Bytes) -> bool {
    let a2717 = v0.clone();
    let a2718 = f201(Str::lit("sub"));
    if equal(a2717, a2718) {
        true
    } else {
        let a2719 = v0.clone();
        let a2720 = f201(Str::lit("sup"));
        if equal(a2719, a2720) {
            true
        } else {
            let a2721 = v0;
            let a2722 = f201(Str::lit("frac"));
            equal(a2721, a2722)
        }
    }
}

pub fn f491(v0: Fn76, p2727: u64, p2728: (List<Adt7>, List<Adt7>)) -> ((List<Adt7>, List<Adt7>), bool) {
    let m2726 = (p2727, p2728);
    {
        let mut m2726 = m2726;
        loop {
            let step = {
                let (v1, v2) = m2726;
                let m2723 = v1;
                if m2723 == 0 {
                    Err::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v2, false))
                } else {
                    let v3 = m2723 - 1;
                    let m2725 = {
                        let c2724: Fn76 = v0.clone();
                        c2724.apply(v2.clone())
                    };
                    match m2725 {
                        None => {
                            Err::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt7>, List<Adt7>)), ((List<Adt7>, List<Adt7>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: (List<Adt7>, List<Adt7>)) -> Option<(List<Adt7>, List<Adt7>)> {
    let m2730 = {
        let (_, h2729) = v0;
        h2729
    };
    match m2730.uncons() {
        None => {
            None::<(List<Adt7>, List<Adt7>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h2731, _) = v0;
                h2731
            }), v2))
        }
    }
}

pub fn f493(v0: Fn77, p2735: Adt7, p2736: List<Adt7>) -> Adt7 {
    let m2734 = (p2735, p2736);
    {
        let mut m2734 = m2734;
        loop {
            let step = {
                let (v1, v2) = m2734;
                let m2732 = v2;
                match m2732.uncons() {
                    None => {
                        Err::<(Adt7, List<Adt7>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<Adt7>), Adt7>(({
                            let c2733: Fn77 = v0.clone();
                            c2733.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Adt7, v1: Adt7) -> Adt7 {
    Adt7::C14(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f495(v0: Adt7, v1: u64) -> R<bool> {
    let a2741 = {
        let (_, h2737) = {
            let (h2738, _) = f522(Fn80::F523(v1), {
                let a2739 = v1;
                let a2740 = 1u64;
                nat_add(a2739, a2740)?
            }, (List::cons(v0, List::<Adt7>::nil()), 0u64))?;
            h2738
        };
        h2737
    };
    let a2742 = 0u64;
    Ok(nat_eq(a2741, a2742))
}

pub fn f496(v0: Adt7) -> bool {
    let m2743 = v0;
    match m2743 {
        Adt7::C0(_, _, _) => {
            false
        }
        Adt7::C1(_) => {
            false
        }
        Adt7::C2(_) => {
            false
        }
        Adt7::C3(_) => {
            false
        }
        Adt7::C4 => {
            false
        }
        Adt7::C5(_) => {
            false
        }
        Adt7::C6(_) => {
            false
        }
        Adt7::C7(_) => {
            false
        }
        Adt7::C8(_) => {
            false
        }
        Adt7::C9(_, _) => {
            true
        }
        Adt7::C10(_, _) => {
            true
        }
        Adt7::C11(_, _) => {
            false
        }
        Adt7::C12(_) => {
            false
        }
        Adt7::C13 => {
            false
        }
        Adt7::C14(_, _) => {
            false
        }
    }
}

pub fn f497() -> Adt28 {
    f200(f201(Str::lit("a sort is `(sort prop)` or `(sort (type u))`")))
}

pub fn f498(v0: Adt35, v1: u64) -> R<Result<Adt2, Adt28>> {
    let v2: Adt56 = {
        let (h2744, _) = f524(Fn81::F525(v1), {
            let a2747 = {
                let a2745 = v1;
                let a2746 = 2u64;
                nat_mul(a2745, a2746)?
            };
            let a2748 = 2u64;
            nat_add(a2747, a2748)?
        }, Adt56::C0(List::cons(Adt57::C0(v0), List::<Adt57>::nil()), List::<Adt2>::nil(), None::<Adt28>))?;
        h2744
    };
    let m2750 = match v2.clone() {
        Adt56::C0(_, _, h2749) => {
            h2749
        }
    };
    match m2750 {
        None => {
            let m2752 = match v2 {
                Adt56::C0(_, h2751, _) => {
                    h2751
                }
            };
            match m2752.uncons() {
                None => {
                    Ok(Err::<Adt2, Adt28>(f200(f201(Str::lit("expected a universe")))))
                }
                Some((v4, _)) => {
                    Ok(Ok::<Adt2, Adt28>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt2, Adt28>(v3))
        }
    }
}

pub fn f499() -> Adt28 {
    f200(f201(Str::lit("a const is `(const qualified-id [universes])`")))
}

pub fn f500(v0: List<Adt35>, v1: u64) -> R<Result<Adt2, Adt28>> {
    let m2753 = f526(Fn82::F527(v1), Ok::<List<Adt2>, Adt28>(List::<Adt2>::nil()), v0)?;
    match m2753 {
        Ok(v2) => {
            Ok(Ok::<Adt2, Adt28>(f528(f529(v2))))
        }
        Err(v3) => {
            Ok(Err::<Adt2, Adt28>(v3))
        }
    }
}

pub fn f501(v0: Bytes) -> R<Option<Adt28>> {
    let v1: u64 = {
        let a2754 = v0.clone();
        length_bytes(a2754)
    };
    let a2755 = v1;
    let a2756 = 0u64;
    if if nat_eq(a2755, a2756) {
        true
    } else {
        !f530(v0.clone())?
    } {
        Ok(Some(f200(f205(f301(v0), f201(Str::lit(" is not a decimal"))))))
    } else {
        let a2757 = 1u64;
        let a2758 = v1;
        if if nat_lt(a2757, a2758) {
            let a2759 = f264(v0.clone(), 0u64)?;
            let a2760 = 48u64;
            nat_eq(a2759, a2760)
        } else {
            false
        } {
            Ok(Some(f200(f205(f301(v0), f201(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a2761 = 20u64;
            let a2762 = v1;
            if if nat_lt(a2761, a2762) {
                true
            } else {
                let a2763 = v1;
                let a2764 = 20u64;
                if nat_eq(a2763, a2764) {
                    let a2769 = {
                        let a2765 = v0.clone();
                        let a2766 = f201(Str::lit("18446744073709551615"));
                        compare_bytes(a2765, a2766)
                    };
                    let a2770 = {
                        let a2767 = Bytes::lit(&[0x01]);
                        let a2768 = Bytes::lit(&[]);
                        compare_bytes(a2767, a2768)
                    };
                    equal(a2769, a2770)
                } else {
                    false
                }
            } {
                Ok(Some(f200(f205(f301(v0), f201(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt28>)
            }
        }
    }
}

pub fn f502(v0: Fn78, p2774: Result<List<Adt38>, Adt28>, p2775: List<Adt35>) -> R<Result<List<Adt38>, Adt28>> {
    let m2773 = (p2774, p2775);
    {
        let mut m2773 = m2773;
        loop {
            let step = {
                let (v1, v2) = m2773;
                let m2771 = v2;
                match m2771.uncons() {
                    None => {
                        Err::<(Result<List<Adt38>, Adt28>, List<Adt35>), R<Result<List<Adt38>, Adt28>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt38>, Adt28>, List<Adt35>), R<Result<List<Adt38>, Adt28>>>(({
                            let c2772: Fn78 = v0.clone();
                            c2772.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: u64, v1: Result<List<Adt38>, Adt28>, v2: Adt35) -> R<Result<List<Adt38>, Adt28>> {
    let m2776 = v1;
    match m2776 {
        Ok(v3) => {
            let m2777 = f475(v2, v0)?;
            match m2777 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt38>, Adt28>(List::cons(Adt38::C0(match v4 {
                        Adt54::C0(_, _, h2778) => {
                            h2778
                        }
                    }), v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt38>, Adt28>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt38>, Adt28>(v6))
        }
    }
}

pub fn f504(v0: Bytes) -> Option<u64> {
    let a2779 = v0.clone();
    let a2780 = f201(Str::lit("explicit"));
    if equal(a2779, a2780) {
        Some(0u64)
    } else {
        let a2781 = v0.clone();
        let a2782 = f201(Str::lit("implicit"));
        if equal(a2781, a2782) {
            Some(1u64)
        } else {
            let a2783 = v0;
            let a2784 = f201(Str::lit("instance"));
            if equal(a2783, a2784) {
                Some(2u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f505(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt55 {
    let v6: u32 = f531(f531(v0, v1), v4);
    let v7: u32 = f532({
        let a2785 = v3;
        let a2786 = v6;
        fixed_u32::bit_xor(a2785, a2786)
    }, 16u32, 16u32);
    let v8: u32 = f531(v2, v7);
    let v9: u32 = f532({
        let a2787 = v1;
        let a2788 = v8;
        fixed_u32::bit_xor(a2787, a2788)
    }, 12u32, 20u32);
    let v10: u32 = f531(f531(v6, v9), v5);
    let v11: u32 = f532({
        let a2789 = v7;
        let a2790 = v10;
        fixed_u32::bit_xor(a2789, a2790)
    }, 8u32, 24u32);
    let v12: u32 = f531(v8, v11);
    let v13: u32 = f532({
        let a2791 = v9;
        let a2792 = v12;
        fixed_u32::bit_xor(a2791, a2792)
    }, 7u32, 25u32);
    Adt55::C0(v10, v13, v12, v11)
}

pub fn f506(v0: Bytes, v1: u64) -> u8 {
    let m2793 = f247(v0, v1);
    match m2793 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f507(v0: Bytes, v1: u64, v2: Adt51) -> R<Adt51> {
    let v3: u64 = {
        let a2794 = v0.clone();
        length_bytes(a2794)
    };
    let (h2795, _) = f533(Fn83::F534(v1, v3, v0), {
        let a2797 = v3;
        let a2798 = match v2.clone() {
            Adt51::C0(h2796, _, _) => {
                h2796
            }
        };
        nat_sub(a2797, a2798)
    }, v2)?;
    Ok(h2795)
}

pub fn f508(v0: Bytes, v1: Adt51) -> R<Adt51> {
    let v2: u64 = match v1.clone() {
        Adt51::C0(h2799, _, _) => {
            h2799
        }
    };
    let v3: u64 = f535(v0.clone(), v2);
    let v4: bool = f536(v0, v2, 10u8);
    if v4 {
        Ok(Adt51::C0({
            let a2800 = v2;
            let a2801 = 1u64;
            nat_add(a2800, a2801)?
        }, {
            let a2803 = match v1 {
                Adt51::C0(_, h2802, _) => {
                    h2802
                }
            };
            let a2804 = 1u64;
            nat_add(a2803, a2804)?
        }, 1u64))
    } else {
        Ok(Adt51::C0({
            let a2805 = v2;
            let a2806 = v3;
            nat_add(a2805, a2806)?
        }, match v1.clone() {
            Adt51::C0(_, h2807, _) => {
                h2807
            }
        }, {
            let a2809 = match v1 {
                Adt51::C0(_, _, h2808) => {
                    h2808
                }
            };
            let a2810 = 1u64;
            nat_add(a2809, a2810)?
        }))
    }
}

pub fn f509(v0: u8) -> bool {
    let v1: bool = f510(v0, 97u8, 122u8);
    let v2: bool = f510(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f510(v0: u8, v1: u8, v2: u8) -> bool {
    let m2813 = {
        let a2811 = v0;
        let a2812 = v1;
        fixed_u8::checked_sub(a2811, a2812)
    };
    match m2813 {
        None => {
            false
        }
        Some(_) => {
            let m2816 = {
                let a2814 = v2;
                let a2815 = v0;
                fixed_u8::checked_sub(a2814, a2815)
            };
            match m2816 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f511(v0: u8) -> bool {
    f510(v0, 48u8, 57u8)
}

pub fn f512(v0: Bytes, v1: Adt51, v2: Adt51) -> Adt48 {
    let v3: Adt49 = f537(v0, v1, v2);
    Adt48::C0(f514(Str::lit("LLL1004")), f514(Str::lit("a control sequence needs ASCII letters or one ASCII nonletter after the backslash")), Some(v3), None::<Bytes>)
}

pub fn f513(v0: Bytes, v1: Bytes, v2: Adt51) -> R<Adt48> {
    let v3: u8 = f506(v1, match v2.clone() {
        Adt51::C0(h2817, _, _) => {
            h2817
        }
    });
    let v5: u8 = {
        let m2820 = {
            let a2818 = v3;
            let a2819 = 4u32;
            fixed_u8::shift_right(a2818, a2819)
        };
        match m2820 {
            None => {
                0u8
            }
            Some(v4) => {
                v4
            }
        }
    };
    let v6: u8 = {
        let a2821 = v3;
        let a2822 = 15u8;
        fixed_u8::bit_and(a2821, a2822)
    };
    let v7: Bytes = f538(v5);
    let v8: Bytes = f538(v6);
    let v9: Adt51 = Adt51::C0({
        let a2824 = match v2.clone() {
            Adt51::C0(h2823, _, _) => {
                h2823
            }
        };
        let a2825 = 1u64;
        nat_add(a2824, a2825)?
    }, match v2.clone() {
        Adt51::C0(_, h2826, _) => {
            h2826
        }
    }, {
        let a2828 = match v2.clone() {
            Adt51::C0(_, _, h2827) => {
                h2827
            }
        };
        let a2829 = 1u64;
        nat_add(a2828, a2829)?
    });
    let v10: Adt49 = f537(v0, v2, v9);
    Ok(Adt48::C0(f514(Str::lit("LLL1001")), f515(f515(f514(Str::lit("forbidden scalar U+00")), v7), v8), Some(v10), None::<Bytes>))
}

pub fn f514(v0: Str) -> Bytes {
    let a2830 = v0;
    utf8_encode(a2830)
}

pub fn f515(v0: Bytes, v1: Bytes) -> Bytes {
    let a2831 = v0;
    let a2832 = v1;
    append_bytes(a2831, a2832)
}

pub fn f516(v0: u64) -> R<Bytes> {
    let v1: Bytes = f514(Str::lit("0"));
    let v2: Bytes = f539({
        let a2869 = f540(v0, 10000000000000000000u64);
        let a2870 = {
            let a2867 = f540(v0, 1000000000000000000u64);
            let a2868 = {
                let a2865 = f540(v0, 100000000000000000u64);
                let a2866 = {
                    let a2863 = f540(v0, 10000000000000000u64);
                    let a2864 = {
                        let a2861 = f540(v0, 1000000000000000u64);
                        let a2862 = {
                            let a2859 = f540(v0, 100000000000000u64);
                            let a2860 = {
                                let a2857 = f540(v0, 10000000000000u64);
                                let a2858 = {
                                    let a2855 = f540(v0, 1000000000000u64);
                                    let a2856 = {
                                        let a2853 = f540(v0, 100000000000u64);
                                        let a2854 = {
                                            let a2851 = f540(v0, 10000000000u64);
                                            let a2852 = {
                                                let a2849 = f540(v0, 1000000000u64);
                                                let a2850 = {
                                                    let a2847 = f540(v0, 100000000u64);
                                                    let a2848 = {
                                                        let a2845 = f540(v0, 10000000u64);
                                                        let a2846 = {
                                                            let a2843 = f540(v0, 1000000u64);
                                                            let a2844 = {
                                                                let a2841 = f540(v0, 100000u64);
                                                                let a2842 = {
                                                                    let a2839 = f540(v0, 10000u64);
                                                                    let a2840 = {
                                                                        let a2837 = f540(v0, 1000u64);
                                                                        let a2838 = {
                                                                            let a2835 = f540(v0, 100u64);
                                                                            let a2836 = {
                                                                                let a2833 = f540(v0, 10u64);
                                                                                let a2834 = f540(v0, 1u64);
                                                                                append_bytes(a2833, a2834)
                                                                            };
                                                                            append_bytes(a2835, a2836)
                                                                        };
                                                                        append_bytes(a2837, a2838)
                                                                    };
                                                                    append_bytes(a2839, a2840)
                                                                };
                                                                append_bytes(a2841, a2842)
                                                            };
                                                            append_bytes(a2843, a2844)
                                                        };
                                                        append_bytes(a2845, a2846)
                                                    };
                                                    append_bytes(a2847, a2848)
                                                };
                                                append_bytes(a2849, a2850)
                                            };
                                            append_bytes(a2851, a2852)
                                        };
                                        append_bytes(a2853, a2854)
                                    };
                                    append_bytes(a2855, a2856)
                                };
                                append_bytes(a2857, a2858)
                            };
                            append_bytes(a2859, a2860)
                        };
                        append_bytes(a2861, a2862)
                    };
                    append_bytes(a2863, a2864)
                };
                append_bytes(a2865, a2866)
            };
            append_bytes(a2867, a2868)
        };
        append_bytes(a2869, a2870)
    })?;
    let a2871 = v0;
    let a2872 = 0u64;
    if nat_eq(a2871, a2872) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f517(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f541(v1, v0, v2)
}

pub fn f518(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2873 = v0.clone();
        length_bytes(a2873)
    };
    let a2874 = v1;
    let a2875 = 0u64;
    if nat_eq(a2874, a2875) {
        Ok(false)
    } else {
        if f263(f264(v0.clone(), 0u64)?) {
            let a2877 = {
                let (h2876, _) = f240(Fn39::F542(v1, v0), v1, 1u64)?;
                h2876
            };
            let a2878 = v1;
            Ok(nat_eq(a2877, a2878))
        } else {
            Ok(false)
        }
    }
}

pub fn f519(v0: Bytes) -> R<u64> {
    let m2879 = f246(v0)?;
    match m2879 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f520(v0: Fn79, p2883: List<Adt44>, p2884: List<Adt35>) -> List<Adt44> {
    let m2882 = (p2883, p2884);
    {
        let mut m2882 = m2882;
        loop {
            let step = {
                let (v1, v2) = m2882;
                let m2880 = v2;
                match m2880.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt35>), List<Adt44>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt44>, List<Adt35>), List<Adt44>>(({
                            let c2881: Fn79 = v0.clone();
                            c2881.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<Adt44>, v1: Adt35) -> List<Adt44> {
    List::cons(Adt44::C0(v1), v0)
}

pub fn f522(v0: Fn80, p2889: u64, p2890: (List<Adt7>, u64)) -> R<((List<Adt7>, u64), bool)> {
    let m2888 = (p2889, p2890);
    {
        let mut m2888 = m2888;
        loop {
            let step = {
                let (v1, v2) = m2888;
                let m2885 = v1;
                if m2885 == 0 {
                    Err::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2885 - 1;
                    let m2887 = {
                        let c2886: Fn80 = v0.clone();
                        c2886.apply(v2.clone())?
                    };
                    match m2887 {
                        None => {
                            Err::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt7>, u64)), R<((List<Adt7>, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: u64, v1: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
    f543(v0, v1)
}

pub fn f524(v0: Fn81, p2895: u64, p2896: Adt56) -> R<(Adt56, bool)> {
    let m2894 = (p2895, p2896);
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let (v1, v2) = m2894;
                let m2891 = v1;
                if m2891 == 0 {
                    Err::<(u64, Adt56), R<(Adt56, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2891 - 1;
                    let m2893 = {
                        let c2892: Fn81 = v0.clone();
                        c2892.apply(v2.clone())?
                    };
                    match m2893 {
                        None => {
                            Err::<(u64, Adt56), R<(Adt56, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt56), R<(Adt56, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: u64, v1: Adt56) -> R<Option<Adt56>> {
    f544(v0, v1)
}

pub fn f526(v0: Fn82, p2900: Result<List<Adt2>, Adt28>, p2901: List<Adt35>) -> R<Result<List<Adt2>, Adt28>> {
    let m2899 = (p2900, p2901);
    {
        let mut m2899 = m2899;
        loop {
            let step = {
                let (v1, v2) = m2899;
                let m2897 = v2;
                match m2897.uncons() {
                    None => {
                        Err::<(Result<List<Adt2>, Adt28>, List<Adt35>), R<Result<List<Adt2>, Adt28>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt2>, Adt28>, List<Adt35>), R<Result<List<Adt2>, Adt28>>>(({
                            let c2898: Fn82 = v0.clone();
                            c2898.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: u64, v1: Result<List<Adt2>, Adt28>, v2: Adt35) -> R<Result<List<Adt2>, Adt28>> {
    let m2902 = v1;
    match m2902 {
        Ok(v3) => {
            let m2903 = f498(v2, v0)?;
            match m2903 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt2>, Adt28>(List::cons(v4, v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt2>, Adt28>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt2>, Adt28>(v6))
        }
    }
}

pub fn f528(v0: List<Adt2>) -> Adt2 {
    f545(Fn84::F546, Adt2::C5, f529(v0))
}

pub fn f529(v0: List<Adt2>) -> List<Adt2> {
    f547(Fn85::F548, List::<Adt2>::nil(), v0)
}

pub fn f530(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2904 = v0.clone();
        length_bytes(a2904)
    };
    let a2908 = {
        let (h2905, _) = f240(Fn39::F549(v1, v0), {
            let a2906 = v1;
            let a2907 = 1u64;
            nat_add(a2906, a2907)?
        }, 0u64)?;
        h2905
    };
    let a2909 = v1;
    Ok(nat_eq(a2908, a2909))
}

pub fn f531(v0: u32, v1: u32) -> u32 {
    let m2912 = {
        let a2910 = f550(v0);
        let a2911 = f550(v1);
        fixed_u64::checked_add(a2910, a2911)
    };
    match m2912 {
        None => {
            0u32
        }
        Some(v2) => {
            f551(v2)
        }
    }
}

pub fn f532(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2913 = f344(v0, v1);
    let a2914 = f444(v0, v2);
    fixed_u32::bit_or(a2913, a2914)
}

pub fn f533(v0: Fn83, p2919: u64, p2920: Adt51) -> R<(Adt51, bool)> {
    let m2918 = (p2919, p2920);
    {
        let mut m2918 = m2918;
        loop {
            let step = {
                let (v1, v2) = m2918;
                let m2915 = v1;
                if m2915 == 0 {
                    Err::<(u64, Adt51), R<(Adt51, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2915 - 1;
                    let m2917 = {
                        let c2916: Fn83 = v0.clone();
                        c2916.apply(v2.clone())?
                    };
                    match m2917 {
                        None => {
                            Err::<(u64, Adt51), R<(Adt51, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt51), R<(Adt51, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: u64, v1: u64, v2: Bytes, v3: Adt51) -> R<Option<Adt51>> {
    f552(v2, v1, v0, v3)
}

pub fn f535(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f506(v0, v1);
    let v3: bool = f510(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f510(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f510(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m2921 = f247(v0, v1);
    match m2921 {
        None => {
            false
        }
        Some(v3) => {
            let a2922 = v3;
            let a2923 = v2;
            equal(a2922, a2923)
        }
    }
}

pub fn f537(v0: Bytes, v1: Adt51, v2: Adt51) -> Adt49 {
    Adt49::C0(v0, match v1.clone() {
        Adt51::C0(h2924, _, _) => {
            h2924
        }
    }, match v2.clone() {
        Adt51::C0(h2925, _, _) => {
            h2925
        }
    }, match v1.clone() {
        Adt51::C0(_, h2926, _) => {
            h2926
        }
    }, match v1 {
        Adt51::C0(_, _, h2927) => {
            h2927
        }
    }, match v2.clone() {
        Adt51::C0(_, h2928, _) => {
            h2928
        }
    }, match v2 {
        Adt51::C0(_, _, h2929) => {
            h2929
        }
    })
}

pub fn f538(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a2930 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2930)
    };
    let v2: u64 = {
        let a2931 = v0;
        let a2932 = 0u8;
        if equal(a2931, a2932) {
            0u64
        } else {
            let a2933 = v0;
            let a2934 = 1u8;
            if equal(a2933, a2934) {
                1u64
            } else {
                let a2935 = v0;
                let a2936 = 2u8;
                if equal(a2935, a2936) {
                    2u64
                } else {
                    let a2937 = v0;
                    let a2938 = 3u8;
                    if equal(a2937, a2938) {
                        3u64
                    } else {
                        let a2939 = v0;
                        let a2940 = 4u8;
                        if equal(a2939, a2940) {
                            4u64
                        } else {
                            let a2941 = v0;
                            let a2942 = 5u8;
                            if equal(a2941, a2942) {
                                5u64
                            } else {
                                let a2943 = v0;
                                let a2944 = 6u8;
                                if equal(a2943, a2944) {
                                    6u64
                                } else {
                                    let a2945 = v0;
                                    let a2946 = 7u8;
                                    if equal(a2945, a2946) {
                                        7u64
                                    } else {
                                        let a2947 = v0;
                                        let a2948 = 8u8;
                                        if equal(a2947, a2948) {
                                            8u64
                                        } else {
                                            let a2949 = v0;
                                            let a2950 = 9u8;
                                            if equal(a2949, a2950) {
                                                9u64
                                            } else {
                                                let a2951 = v0;
                                                let a2952 = 10u8;
                                                if equal(a2951, a2952) {
                                                    10u64
                                                } else {
                                                    let a2953 = v0;
                                                    let a2954 = 11u8;
                                                    if equal(a2953, a2954) {
                                                        11u64
                                                    } else {
                                                        let a2955 = v0;
                                                        let a2956 = 12u8;
                                                        if equal(a2955, a2956) {
                                                            12u64
                                                        } else {
                                                            let a2957 = v0;
                                                            let a2958 = 13u8;
                                                            if equal(a2957, a2958) {
                                                                13u64
                                                            } else {
                                                                let a2959 = v0;
                                                                let a2960 = 14u8;
                                                                if equal(a2959, a2960) {
                                                                    14u64
                                                                } else {
                                                                    15u64
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    f83(v1, v2, 1u64)
}

pub fn f539(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h2961, _) = f240(Fn39::F553(v0.clone()), {
            let a2962 = v0.clone();
            length_bytes(a2962)
        }, 0u64)?;
        h2961
    };
    Ok(f83(v0.clone(), v1, {
        let a2964 = {
            let a2963 = v0;
            length_bytes(a2963)
        };
        let a2965 = v1;
        nat_sub(a2964, a2965)
    }))
}

pub fn f540(v0: u64, v1: u64) -> Bytes {
    f83(f514(Str::lit("0123456789")), {
        let a2969 = {
            let a2966 = v0;
            let a2967 = v1;
            let a2968 = 0u64;
            nat_quot(a2966, a2967, a2968)
        };
        let a2970 = 10u64;
        let a2971 = 0u64;
        nat_rem(a2969, a2970, a2971)
    }, 1u64)
}

pub fn f541(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a2972 = v2;
    let a2973 = v1;
    if nat_lt(a2972, a2973) {
        if f554(f555(v0.clone(), v2)?) {
            Ok(Some({
                let a2974 = v2;
                let a2975 = f556(f264(v0, v2)?);
                nat_add(a2974, a2975)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f542(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f557(v1, v0, v2)
}

pub fn f543(v0: u64, v1: (List<Adt7>, u64)) -> R<Option<(List<Adt7>, u64)>> {
    let a2977 = {
        let (_, h2976) = v1;
        h2976
    };
    let a2978 = 1u64;
    if nat_eq(a2977, a2978) {
        Ok(None::<(List<Adt7>, u64)>)
    } else {
        let m2980 = {
            let (h2979, _) = v1;
            h2979
        };
        match m2980.uncons() {
            None => {
                Ok(None::<(List<Adt7>, u64)>)
            }
            Some((v2, v3)) => {
                let m2981 = v2;
                match m2981 {
                    Adt7::C0(_, _, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C1(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C2(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C3(r2982) => {
                        let v4 = (*r2982).clone();
                        Ok(Some((f357(Fn57::F558, v3, f360(v4, v0)?), 0u64)))
                    }
                    Adt7::C4 => {
                        Ok(Some((v3, 0u64)))
                    }
                    Adt7::C5(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C6(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C7(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C8(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C9(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C10(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C11(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C12(_) => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C13 => {
                        Ok(Some((v3, 1u64)))
                    }
                    Adt7::C14(_, _) => {
                        Ok(Some((v3, 1u64)))
                    }
                }
            }
        }
    }
}

pub fn f544(v0: u64, v1: Adt56) -> R<Option<Adt56>> {
    let m2984 = match v1.clone() {
        Adt56::C0(_, _, h2983) => {
            h2983
        }
    };
    match m2984 {
        None => {
            let m2986 = match v1.clone() {
                Adt56::C0(h2985, _, _) => {
                    h2985
                }
            };
            match m2986.uncons() {
                None => {
                    Ok(None::<Adt56>)
                }
                Some((v3, v4)) => {
                    let m2987 = v3;
                    match m2987 {
                        Adt57::C0(v5) => {
                            Ok(Some(f559(v0, v1, v4, v5)?))
                        }
                        Adt57::C1(v6, v7) => {
                            Ok(Some(f560(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt56>)
        }
    }
}

pub fn f545(v0: Fn84, p2991: Adt2, p2992: List<Adt2>) -> Adt2 {
    let m2990 = (p2991, p2992);
    {
        let mut m2990 = m2990;
        loop {
            let step = {
                let (v1, v2) = m2990;
                let m2988 = v2;
                match m2988.uncons() {
                    None => {
                        Err::<(Adt2, List<Adt2>), Adt2>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt2, List<Adt2>), Adt2>(({
                            let c2989: Fn84 = v0.clone();
                            c2989.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2990 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: Adt2, v1: Adt2) -> Adt2 {
    Adt2::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f547(v0: Fn85, p2996: List<Adt2>, p2997: List<Adt2>) -> List<Adt2> {
    let m2995 = (p2996, p2997);
    {
        let mut m2995 = m2995;
        loop {
            let step = {
                let (v1, v2) = m2995;
                let m2993 = v2;
                match m2993.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c2994: Fn85 = v0.clone();
                            c2994.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f549(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f561(v1, v0, v2)
}

pub fn f550(v0: u32) -> u64 {
    let m2999 = {
        let a2998 = v0;
        fixed_u64::convert(i128::from(a2998))
    };
    match m2999 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f551(v0: u64) -> u32 {
    let m3003 = {
        let a3002 = {
            let a3000 = v0;
            let a3001 = 4294967295u64;
            fixed_u64::bit_and(a3000, a3001)
        };
        fixed_u32::convert(i128::from(a3002))
    };
    match m3003 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f552(v0: Bytes, v1: u64, v2: u64, v3: Adt51) -> R<Option<Adt51>> {
    let v4: u64 = match v3.clone() {
        Adt51::C0(h3004, _, _) => {
            h3004
        }
    };
    let a3005 = v4;
    let a3006 = v1;
    if nat_lt(a3005, a3006) {
        let v5: u8 = f506(v0, v4);
        let v6: bool = f562(v2, v5);
        if v6 {
            let a3007 = v5;
            let a3008 = 10u8;
            if equal(a3007, a3008) {
                Ok(Some(Adt51::C0({
                    let a3009 = v4;
                    let a3010 = 1u64;
                    nat_add(a3009, a3010)?
                }, {
                    let a3012 = match v3 {
                        Adt51::C0(_, h3011, _) => {
                            h3011
                        }
                    };
                    let a3013 = 1u64;
                    nat_add(a3012, a3013)?
                }, 1u64)))
            } else {
                Ok(Some(Adt51::C0({
                    let a3014 = v4;
                    let a3015 = 1u64;
                    nat_add(a3014, a3015)?
                }, match v3.clone() {
                    Adt51::C0(_, h3016, _) => {
                        h3016
                    }
                }, {
                    let a3018 = match v3 {
                        Adt51::C0(_, _, h3017) => {
                            h3017
                        }
                    };
                    let a3019 = 1u64;
                    nat_add(a3018, a3019)?
                })))
            }
        } else {
            Ok(None::<Adt51>)
        }
    } else {
        Ok(None::<Adt51>)
    }
}

pub fn f553(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f563(v0, v1)
}

pub fn f554(v0: u64) -> bool {
    let a3020 = v0;
    let a3021 = 172u64;
    if if if if if nat_eq(a3020, a3021) {
        true
    } else {
        let a3022 = v0;
        let a3023 = 177u64;
        nat_eq(a3022, a3023)
    } {
        true
    } else {
        let a3024 = v0;
        let a3025 = 183u64;
        if nat_eq(a3024, a3025) {
            true
        } else {
            let a3026 = v0;
            let a3027 = 215u64;
            nat_eq(a3026, a3027)
        }
    } {
        true
    } else {
        let a3028 = v0;
        let a3029 = 247u64;
        if if nat_eq(a3028, a3029) {
            true
        } else {
            if f564(v0, 913u64, 937u64) {
                let a3030 = v0;
                let a3031 = 930u64;
                !nat_eq(a3030, a3031)
            } else {
                false
            }
        } {
            true
        } else {
            if f564(v0, 945u64, 969u64) {
                true
            } else {
                let a3032 = v0;
                let a3033 = 8230u64;
                nat_eq(a3032, a3033)
            }
        }
    } {
        true
    } else {
        if if if f564(v0, 8242u64, 8244u64) {
            true
        } else {
            f564(v0, 8448u64, 8527u64)
        } {
            true
        } else {
            if f564(v0, 8592u64, 8703u64) {
                true
            } else {
                f564(v0, 8704u64, 8959u64)
            }
        } {
            true
        } else {
            if if f564(v0, 8968u64, 8971u64) {
                true
            } else {
                f564(v0, 10176u64, 10223u64)
            } {
                true
            } else {
                if f564(v0, 10624u64, 10751u64) {
                    true
                } else {
                    f564(v0, 10752u64, 11007u64)
                }
            }
        }
    } {
        true
    } else {
        f564(v0, 119808u64, 120831u64)
    }
}

pub fn f555(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f264(v0.clone(), v1)?;
    let v3: u64 = f556(v2);
    let a3034 = v3;
    let a3035 = 1u64;
    if nat_eq(a3034, a3035) {
        Ok(v2)
    } else {
        let a3036 = v3;
        let a3037 = 2u64;
        if nat_eq(a3036, a3037) {
            let a3045 = {
                let a3041 = {
                    let a3038 = v2;
                    let a3039 = 32u64;
                    let a3040 = 0u64;
                    nat_rem(a3038, a3039, a3040)
                };
                let a3042 = 64u64;
                nat_mul(a3041, a3042)?
            };
            let a3046 = f565(v0, {
                let a3043 = v1;
                let a3044 = 1u64;
                nat_add(a3043, a3044)?
            })?;
            nat_add(a3045, a3046)
        } else {
            let a3047 = v3;
            let a3048 = 3u64;
            if nat_eq(a3047, a3048) {
                let a3062 = {
                    let a3058 = {
                        let a3052 = {
                            let a3049 = v2;
                            let a3050 = 16u64;
                            let a3051 = 0u64;
                            nat_rem(a3049, a3050, a3051)
                        };
                        let a3053 = 4096u64;
                        nat_mul(a3052, a3053)?
                    };
                    let a3059 = {
                        let a3056 = f565(v0.clone(), {
                            let a3054 = v1;
                            let a3055 = 1u64;
                            nat_add(a3054, a3055)?
                        })?;
                        let a3057 = 64u64;
                        nat_mul(a3056, a3057)?
                    };
                    nat_add(a3058, a3059)?
                };
                let a3063 = f565(v0, {
                    let a3060 = v1;
                    let a3061 = 2u64;
                    nat_add(a3060, a3061)?
                })?;
                nat_add(a3062, a3063)
            } else {
                let a3083 = {
                    let a3079 = {
                        let a3073 = {
                            let a3067 = {
                                let a3064 = v2;
                                let a3065 = 8u64;
                                let a3066 = 0u64;
                                nat_rem(a3064, a3065, a3066)
                            };
                            let a3068 = 262144u64;
                            nat_mul(a3067, a3068)?
                        };
                        let a3074 = {
                            let a3071 = f565(v0.clone(), {
                                let a3069 = v1;
                                let a3070 = 1u64;
                                nat_add(a3069, a3070)?
                            })?;
                            let a3072 = 4096u64;
                            nat_mul(a3071, a3072)?
                        };
                        nat_add(a3073, a3074)?
                    };
                    let a3080 = {
                        let a3077 = f565(v0.clone(), {
                            let a3075 = v1;
                            let a3076 = 2u64;
                            nat_add(a3075, a3076)?
                        })?;
                        let a3078 = 64u64;
                        nat_mul(a3077, a3078)?
                    };
                    nat_add(a3079, a3080)?
                };
                let a3084 = f565(v0, {
                    let a3081 = v1;
                    let a3082 = 3u64;
                    nat_add(a3081, a3082)?
                })?;
                nat_add(a3083, a3084)
            }
        }
    }
}

pub fn f556(v0: u64) -> u64 {
    let a3085 = v0;
    let a3086 = 128u64;
    if nat_lt(a3085, a3086) {
        1u64
    } else {
        let a3087 = v0;
        let a3088 = 224u64;
        if nat_lt(a3087, a3088) {
            2u64
        } else {
            let a3089 = v0;
            let a3090 = 240u64;
            if nat_lt(a3089, a3090) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f557(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3091 = v2;
    let a3092 = v1;
    if nat_lt(a3091, a3092) {
        let v3: u64 = f264(v0, v2)?;
        if if f366(v3) {
            true
        } else {
            let a3093 = v3;
            let a3094 = 95u64;
            nat_eq(a3093, a3094)
        } {
            Ok(Some({
                let a3095 = v2;
                let a3096 = 1u64;
                nat_add(a3095, a3096)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f558(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f559(v0: u64, v1: Adt56, v2: List<Adt57>, v3: Adt35) -> R<Adt56> {
    let m3097 = v3;
    match m3097 {
        Adt35::C0(v4) => {
            if f530(v4.clone())? {
                let m3098 = f501(v4.clone())?;
                match m3098 {
                    None => {
                        Ok(Adt56::C0(v2, List::cons(Adt2::C0(f519(v4)?), match v1 {
                            Adt56::C0(_, h3099, _) => {
                                h3099
                            }
                        }), None::<Adt28>))
                    }
                    Some(v5) => {
                        Ok(f566(v1, v5))
                    }
                }
            } else {
                if f193(v4.clone())? {
                    Ok(Adt56::C0(v2, List::cons(Adt2::C1(v4), match v1 {
                        Adt56::C0(_, h3100, _) => {
                            h3100
                        }
                    }), None::<Adt28>))
                } else {
                    Ok(f566(v1, f200(f205(f301(v4), f201(Str::lit(" is not a universe"))))))
                }
            }
        }
        Adt35::C1(r3123) => {
            let v6 = (*r3123).clone();
            let v7: List<Adt35> = f378(v6, v0)?;
            let m3101 = v7;
            match m3101.uncons() {
                None => {
                    Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                }
                Some((v8, v9)) => {
                    let m3102 = v8;
                    match m3102 {
                        Adt35::C0(v10) => {
                            let v11: u64 = {
                                let a3103 = v9.clone();
                                length_list(a3103)
                            };
                            let v12: bool = {
                                let a3104 = v10.clone();
                                let a3105 = f201(Str::lit("succ"));
                                if equal(a3104, a3105) {
                                    true
                                } else {
                                    let a3106 = v10.clone();
                                    let a3107 = f201(Str::lit("max"));
                                    if equal(a3106, a3107) {
                                        true
                                    } else {
                                        let a3108 = v10.clone();
                                        let a3109 = f201(Str::lit("imax"));
                                        equal(a3108, a3109)
                                    }
                                }
                            };
                            let v13: bool = {
                                let a3110 = v10.clone();
                                let a3111 = f201(Str::lit("succ"));
                                if equal(a3110, a3111) {
                                    let a3112 = v11;
                                    let a3113 = 1u64;
                                    nat_eq(a3112, a3113)
                                } else {
                                    let a3114 = v10.clone();
                                    let a3115 = f201(Str::lit("max"));
                                    if equal(a3114, a3115) {
                                        let a3116 = 2u64;
                                        let a3117 = v11;
                                        nat_le(a3116, a3117)
                                    } else {
                                        let a3118 = v10.clone();
                                        let a3119 = f201(Str::lit("imax"));
                                        if equal(a3118, a3119) {
                                            let a3120 = v11;
                                            let a3121 = 2u64;
                                            nat_eq(a3120, a3121)
                                        } else {
                                            false
                                        }
                                    }
                                }
                            };
                            if v13 {
                                Ok(Adt56::C0(f567(v9, List::cons(Adt57::C1(v10, v11), v2)), match v1 {
                                    Adt56::C0(_, h3122, _) => {
                                        h3122
                                    }
                                }, None::<Adt28>))
                            } else {
                                if v12 {
                                    Ok(f566(v1, f200(f205(f205(f201(Str::lit("`(")), v10), f201(Str::lit(" ...)` has the wrong arity"))))))
                                } else {
                                    Ok(f566(v1, f200(f205(f301(v10), f201(Str::lit(" is not a universe form"))))))
                                }
                            }
                        }
                        Adt35::C1(_) => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                        Adt35::C2 => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                        Adt35::C3(_, _) => {
                            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
                        }
                    }
                }
            }
        }
        Adt35::C2 => {
            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
        }
        Adt35::C3(_, _) => {
            Ok(f566(v1, f200(f201(Str::lit("expected a universe form")))))
        }
    }
}

pub fn f560(v0: Adt56, v1: List<Adt57>, v2: Bytes, v3: u64) -> Adt56 {
    let v4: (List<Adt2>, List<Adt2>) = f568(match v0 {
        Adt56::C0(_, h3124, _) => {
            h3124
        }
    }, v3);
    let v5: List<Adt2> = {
        let (h3125, _) = v4;
        h3125
    };
    let v12: Adt2 = {
        let a3126 = v2.clone();
        let a3127 = f201(Str::lit("succ"));
        if equal(a3126, a3127) {
            let m3128 = v5;
            match m3128.uncons() {
                None => {
                    Adt2::C5
                }
                Some((v6, _)) => {
                    Adt2::C2(std::rc::Rc::new(v6))
                }
            }
        } else {
            let a3129 = v2;
            let a3130 = f201(Str::lit("imax"));
            if equal(a3129, a3130) {
                let m3131 = v5;
                match m3131.uncons() {
                    None => {
                        Adt2::C5
                    }
                    Some((v8, v9)) => {
                        let m3132 = v9;
                        match m3132.uncons() {
                            None => {
                                Adt2::C5
                            }
                            Some((v10, _)) => {
                                Adt2::C4(std::rc::Rc::new(v8), std::rc::Rc::new(v10))
                            }
                        }
                    }
                }
            } else {
                Adt2::C3(std::rc::Rc::new(f528(v5)))
            }
        }
    };
    Adt56::C0(v1, List::cons(v12, {
        let (_, h3133) = v4;
        h3133
    }), None::<Adt28>)
}

pub fn f561(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3134 = v2;
    let a3135 = v1;
    if nat_lt(a3134, a3135) {
        if f321(f264(v0, v2)?) {
            Ok(Some({
                let a3136 = v2;
                let a3137 = 1u64;
                nat_add(a3136, a3137)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f562(v0: u64, v1: u8) -> bool {
    let v2: bool = {
        let a3138 = v1;
        let a3139 = 32u8;
        equal(a3138, a3139)
    };
    let v3: bool = {
        let a3140 = v1;
        let a3141 = 10u8;
        equal(a3140, a3141)
    };
    let v4: bool = f509(v1);
    let v5: bool = f511(v1);
    let a3142 = v0;
    let a3143 = 0u64;
    if nat_eq(a3142, a3143) {
        if v2 {
            true
        } else {
            v3
        }
    } else {
        let a3144 = v0;
        let a3145 = 1u64;
        if nat_eq(a3144, a3145) {
            v4
        } else {
            v5
        }
    }
}

pub fn f563(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f536(v0, v1, 48u8) {
        Ok(Some({
            let a3146 = v1;
            let a3147 = 1u64;
            nat_add(a3146, a3147)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f564(v0: u64, v1: u64, v2: u64) -> bool {
    let a3148 = v0;
    let a3149 = v1;
    if nat_lt(a3148, a3149) {
        false
    } else {
        let a3150 = v0;
        let a3151 = v2;
        nat_le(a3150, a3151)
    }
}

pub fn f565(v0: Bytes, v1: u64) -> R<u64> {
    let a3152 = f264(v0, v1)?;
    let a3153 = 64u64;
    let a3154 = 0u64;
    Ok(nat_rem(a3152, a3153, a3154))
}

pub fn f566(v0: Adt56, v1: Adt28) -> Adt56 {
    Adt56::C0(match v0.clone() {
        Adt56::C0(h3155, _, _) => {
            h3155
        }
    }, match v0 {
        Adt56::C0(_, h3156, _) => {
            h3156
        }
    }, Some(v1))
}

pub fn f567(v0: List<Adt35>, v1: List<Adt57>) -> List<Adt57> {
    f569(Fn86::F570, v1, f424(v0))
}

pub fn f568(v0: List<Adt2>, v1: u64) -> (List<Adt2>, List<Adt2>) {
    let (h3157, _) = f571(Fn87::F572, v1, (List::<Adt2>::nil(), v0));
    h3157
}

pub fn f569(v0: Fn86, p3161: List<Adt57>, p3162: List<Adt35>) -> List<Adt57> {
    let m3160 = (p3161, p3162);
    {
        let mut m3160 = m3160;
        loop {
            let step = {
                let (v1, v2) = m3160;
                let m3158 = v2;
                match m3158.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt35>), List<Adt57>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt57>, List<Adt35>), List<Adt57>>(({
                            let c3159: Fn86 = v0.clone();
                            c3159.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: List<Adt57>, v1: Adt35) -> List<Adt57> {
    List::cons(Adt57::C0(v1), v0)
}

pub fn f571(v0: Fn87, p3167: u64, p3168: (List<Adt2>, List<Adt2>)) -> ((List<Adt2>, List<Adt2>), bool) {
    let m3166 = (p3167, p3168);
    {
        let mut m3166 = m3166;
        loop {
            let step = {
                let (v1, v2) = m3166;
                let m3163 = v1;
                if m3163 == 0 {
                    Err::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v2, false))
                } else {
                    let v3 = m3163 - 1;
                    let m3165 = {
                        let c3164: Fn87 = v0.clone();
                        c3164.apply(v2.clone())
                    };
                    match m3165 {
                        None => {
                            Err::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt2>, List<Adt2>)), ((List<Adt2>, List<Adt2>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: (List<Adt2>, List<Adt2>)) -> Option<(List<Adt2>, List<Adt2>)> {
    let m3170 = {
        let (_, h3169) = v0;
        h3169
    };
    match m3170.uncons() {
        None => {
            None::<(List<Adt2>, List<Adt2>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h3171, _) = v0;
                h3171
            }), v2))
        }
    }
}

pub fn run(p0: Bytes, p1: bool, p2: List<Bytes>, p3: u64) -> R<Result<Adt0, List<(Bytes, Bytes)>>> {
    f0(p0, p1, p2, p3)
}
