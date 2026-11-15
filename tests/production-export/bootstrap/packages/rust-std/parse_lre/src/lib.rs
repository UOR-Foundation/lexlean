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
    C0(Bytes, Bytes, Bytes),
    C1(Bytes),
    C2(u64),
    C3(std::rc::Rc<Adt0>),
    C4,
    C5(Bytes),
    C6(std::rc::Rc<Adt0>),
    C7(std::rc::Rc<Adt0>),
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(Bytes),
    C13,
    C14(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, List<Adt2>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes),
    C1(std::rc::Rc<Adt4>),
    C2,
    C3(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<List<Adt4>>, Option<Adt4>, Option<Adt1>, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<Adt7>, List<Adt0>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(std::rc::Rc<Adt4>),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F5(k0, k1) => f5(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9(u64, List<Adt2>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn1::F9(k0, k1) => f9(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn2::F13(k0) => f13(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn3::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F43,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt4, p1: Adt4) -> Adt4 {
        match self {
            Fn4::F43 => f43(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F46,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
        match self {
            Fn5::F46 => f46(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F55,
}

impl Fn6 {
    pub fn apply(&self, p0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
        match self {
            Fn6::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F57,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn7::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F61(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn8::F61(k0, k1) => f61(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F63,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn9::F63 => f63(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F73,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt4) -> List<Adt7> {
        match self {
            Fn10::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F75,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn11::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F82(u64, Bytes),
    F84(u64, Bytes),
    F87(Bytes, u64, Bytes, u64),
    F92(u64, Bytes),
    F104(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn12::F82(k0, k1) => f82(*k0, k1.clone(), p0),
            Fn12::F84(k0, k1) => f84(*k0, k1.clone(), p0),
            Fn12::F87(k0, k1, k2, k3) => f87(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn12::F92(k0, k1) => f92(*k0, k1.clone(), p0),
            Fn12::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F95(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F95(k0) => f95(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F97(bool, u64, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn14::F97(k0, k1, k2) => f97(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: u64) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let m2 = f2(v3.clone(), v1)?;
            match m2 {
                Ok(v5) => {
                    f3(v5, {
                        let a3 = v3;
                        length_list(a3)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt0, Adt1>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt1>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt2>, Adt1>> {
    let v1: u64 = {
        let a4 = v0.clone();
        length_bytes(a4)
    };
    let v2: Adt3 = {
        let (h5, _) = f4(Fn0::F5(v1, v0.clone()), {
            let a6 = v1;
            let a7 = 1u64;
            nat_add(a6, a7)?
        }, Adt3::C0(0u64, 0u64, false, List::<Adt2>::nil(), None::<Adt1>))?;
        h5
    };
    let m9 = match v2.clone() {
        Adt3::C0(_, _, _, _, h8) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt2>, Adt1>(f6(f7(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: List<Adt2>, v1: u64) -> R<Result<Adt4, Adt1>> {
    let v2: Adt5 = {
        let (h10, _) = f8(Fn1::F9(v1, v0.clone()), {
            let a12 = {
                let a11 = v0;
                length_list(a11)
            };
            let a13 = 2u64;
            nat_add(a12, a13)?
        }, Adt5::C0(0u64, List::<List<Adt4>>::nil(), None::<Adt4>, None::<Adt1>, false))?;
        h10
    };
    let m15 = match v2.clone() {
        Adt5::C0(_, _, _, h14, _) => {
            h14
        }
    };
    match m15 {
        None => {
            let m17 = match v2 {
                Adt5::C0(_, _, h16, _, _) => {
                    h16
                }
            };
            match m17 {
                None => {
                    Ok(Err::<Adt4, Adt1>(f10(f11(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt4, Adt1>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt4, Adt1>(v3))
        }
    }
}

pub fn f3(v0: Adt4, v1: u64) -> R<Result<Adt0, Adt1>> {
    let v2: Adt6 = {
        let (h18, _) = f12(Fn2::F13(v1), {
            let a21 = {
                let a19 = v1;
                let a20 = 2u64;
                nat_mul(a19, a20)?
            };
            let a22 = 2u64;
            nat_add(a21, a22)?
        }, Adt6::C0(List::cons(Adt7::C0(std::rc::Rc::new(v0)), List::<Adt7>::nil()), List::<Adt0>::nil(), None::<Adt1>))?;
        h18
    };
    let m24 = match v2.clone() {
        Adt6::C0(_, _, h23) => {
            h23
        }
    };
    match m24 {
        None => {
            let m26 = match v2 {
                Adt6::C0(_, h25, _) => {
                    h25
                }
            };
            match m26.uncons() {
                None => {
                    Ok(Err::<Adt0, Adt1>(f10(f11(Str::lit("malformed render expression")))))
                }
                Some((v4, v5)) => {
                    let m27 = v5;
                    match m27.uncons() {
                        None => {
                            Ok(Ok::<Adt0, Adt1>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt0, Adt1>(f10(f11(Str::lit("malformed render expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt1>(v3))
        }
    }
}

pub fn f4(v0: Fn0, p32: u64, p33: Adt3) -> R<(Adt3, bool)> {
    let m31 = (p32, p33);
    {
        let mut m31 = m31;
        loop {
            let step = {
                let (v1, v2) = m31;
                let m28 = v1;
                if m28 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m28 - 1;
                    let m30 = {
                        let c29: Fn0 = v0.clone();
                        c29.apply(v2.clone())?
                    };
                    match m30 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: u64, v1: Bytes, v2: Adt3) -> R<Option<Adt3>> {
    f14(v1, v0, v2)
}

pub fn f6(v0: List<Adt2>) -> List<Adt2> {
    f15(Fn3::F16, List::<Adt2>::nil(), v0)
}

pub fn f7(v0: Bytes, v1: Adt3) -> List<Adt2> {
    if match v1.clone() {
        Adt3::C0(_, _, h34, _, _) => {
            h34
        }
    } {
        List::cons(Adt2::C2(f17(v0, match v1.clone() {
            Adt3::C0(_, h35, _, _, _) => {
                h35
            }
        }, {
            let a38 = match v1.clone() {
                Adt3::C0(h36, _, _, _, _) => {
                    h36
                }
            };
            let a39 = match v1.clone() {
                Adt3::C0(_, h37, _, _, _) => {
                    h37
                }
            };
            nat_sub(a38, a39)
        })), match v1 {
            Adt3::C0(_, _, _, h40, _) => {
                h40
            }
        })
    } else {
        match v1 {
            Adt3::C0(_, _, _, h41, _) => {
                h41
            }
        }
    }
}

pub fn f8(v0: Fn1, p46: u64, p47: Adt5) -> R<(Adt5, bool)> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn1 = v0.clone();
                        c43.apply(v2.clone())?
                    };
                    match m44 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64, v1: List<Adt2>, v2: Adt5) -> R<Option<Adt5>> {
    f18(v1, v0, v2)
}

pub fn f10(v0: Bytes) -> Adt1 {
    Adt1::C0(0u64, v0)
}

pub fn f11(v0: Str) -> Bytes {
    let a48 = v0;
    utf8_encode(a48)
}

pub fn f12(v0: Fn2, p53: u64, p54: Adt6) -> R<(Adt6, bool)> {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v1, v2) = m52;
                let m49 = v1;
                if m49 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m49 - 1;
                    let m51 = {
                        let c50: Fn2 = v0.clone();
                        c50.apply(v2.clone())?
                    };
                    match m51 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Adt6) -> R<Option<Adt6>> {
    f19(v0, v1)
}

pub fn f14(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m56 = match v2.clone() {
        Adt3::C0(_, _, _, _, h55) => {
            h55
        }
    };
    match m56 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt3::C0(h57, _, _, _, _) => {
                    h57
                }
            };
            let a58 = v4;
            let a59 = v1;
            if nat_lt(a58, a59) {
                let v5: u64 = f20(v0.clone(), v4)?;
                let a60 = v5;
                let a61 = 40u64;
                if nat_eq(a60, a61) {
                    Ok(Some(Adt3::C0({
                        let a62 = v4;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    }, match v2.clone() {
                        Adt3::C0(_, h64, _, _, _) => {
                            h64
                        }
                    }, false, List::cons(Adt2::C0, f7(v0, v2)), None::<Adt1>)))
                } else {
                    let a65 = v5;
                    let a66 = 41u64;
                    if nat_eq(a65, a66) {
                        Ok(Some(Adt3::C0({
                            let a67 = v4;
                            let a68 = 1u64;
                            nat_add(a67, a68)?
                        }, match v2.clone() {
                            Adt3::C0(_, h69, _, _, _) => {
                                h69
                            }
                        }, false, List::cons(Adt2::C1, f7(v0, v2)), None::<Adt1>)))
                    } else {
                        let a70 = v5;
                        let a71 = 32u64;
                        if if nat_eq(a70, a71) {
                            true
                        } else {
                            let a72 = v5;
                            let a73 = 10u64;
                            nat_eq(a72, a73)
                        } {
                            Ok(Some(Adt3::C0({
                                let a74 = v4;
                                let a75 = 1u64;
                                nat_add(a74, a75)?
                            }, match v2.clone() {
                                Adt3::C0(_, h76, _, _, _) => {
                                    h76
                                }
                            }, false, f7(v0, v2), None::<Adt1>)))
                        } else {
                            let a77 = v5;
                            let a78 = 32u64;
                            if if nat_lt(a77, a78) {
                                true
                            } else {
                                let a79 = v5;
                                let a80 = 127u64;
                                nat_eq(a79, a80)
                            } {
                                Ok(Some(Adt3::C0(v4, match v2.clone() {
                                    Adt3::C0(_, h81, _, _, _) => {
                                        h81
                                    }
                                }, match v2.clone() {
                                    Adt3::C0(_, _, h82, _, _) => {
                                        h82
                                    }
                                }, match v2 {
                                    Adt3::C0(_, _, _, h83, _) => {
                                        h83
                                    }
                                }, Some(f10(f11(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt3::C0(_, _, h84, _, _) => {
                                        h84
                                    }
                                } {
                                    Ok(Some(Adt3::C0({
                                        let a85 = v4;
                                        let a86 = 1u64;
                                        nat_add(a85, a86)?
                                    }, match v2.clone() {
                                        Adt3::C0(_, h87, _, _, _) => {
                                            h87
                                        }
                                    }, true, match v2 {
                                        Adt3::C0(_, _, _, h88, _) => {
                                            h88
                                        }
                                    }, None::<Adt1>)))
                                } else {
                                    Ok(Some(Adt3::C0({
                                        let a89 = v4;
                                        let a90 = 1u64;
                                        nat_add(a89, a90)?
                                    }, v4, true, match v2 {
                                        Adt3::C0(_, _, _, h91, _) => {
                                            h91
                                        }
                                    }, None::<Adt1>)))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f15(v0: Fn3, p95: List<Adt2>, p96: List<Adt2>) -> List<Adt2> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m92 = v2;
                match m92.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c93: Fn3 = v0.clone();
                            c93.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m100 = {
        let a97 = v0;
        let a98 = v1;
        let a99 = v2;
        slice_bytes(a97, a98, a99)
    };
    match m100 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f18(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m102 = match v2.clone() {
        Adt5::C0(_, _, _, h101, _) => {
            h101
        }
    };
    match m102 {
        None => {
            if match v2.clone() {
                Adt5::C0(_, _, _, _, h103) => {
                    h103
                }
            } {
                Ok(None::<Adt5>)
            } else {
                let m107 = {
                    let a105 = v0;
                    let a106 = match v2.clone() {
                        Adt5::C0(h104, _, _, _, _) => {
                            h104
                        }
                    };
                    index_list(a105, a106)
                };
                match m107 {
                    None => {
                        let m109 = match v2.clone() {
                            Adt5::C0(_, h108, _, _, _) => {
                                h108
                            }
                        };
                        match m109.uncons() {
                            None => {
                                let m111 = match v2.clone() {
                                    Adt5::C0(_, _, h110, _, _) => {
                                        h110
                                    }
                                };
                                match m111 {
                                    None => {
                                        Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt5::C0(match v2.clone() {
                                            Adt5::C0(h112, _, _, _, _) => {
                                                h112
                                            }
                                        }, List::<List<Adt4>>::nil(), match v2 {
                                            Adt5::C0(_, _, h113, _, _) => {
                                                h113
                                            }
                                        }, None::<Adt1>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m115 = match v2.clone() {
                            Adt5::C0(_, _, h114, _, _) => {
                                h114
                            }
                        };
                        match m115 {
                            None => {
                                Ok(Some(f22(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f19(v0: u64, v1: Adt6) -> R<Option<Adt6>> {
    let m117 = match v1.clone() {
        Adt6::C0(_, _, h116) => {
            h116
        }
    };
    match m117 {
        None => {
            let m119 = match v1.clone() {
                Adt6::C0(h118, _, _) => {
                    h118
                }
            };
            match m119.uncons() {
                None => {
                    Ok(None::<Adt6>)
                }
                Some((v3, v4)) => {
                    let m120 = v3;
                    match m120 {
                        Adt7::C0(r121) => {
                            let v5 = (*r121).clone();
                            Ok(Some(f23(v1, v4, v5, v0)?))
                        }
                        Adt7::C1(v6, v7) => {
                            Ok(Some(f24(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let m122 = f25(v0, v1);
    match m122 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a167 = {
                let a143 = {
                    let a131 = {
                        let a125 = {
                            let a123 = v3;
                            let a124 = 128u8;
                            fixed_u8::bit_and(a123, a124)
                        };
                        let a126 = 128u8;
                        if equal(a125, a126) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a132 = {
                        let a129 = {
                            let a127 = v3;
                            let a128 = 64u8;
                            fixed_u8::bit_and(a127, a128)
                        };
                        let a130 = 64u8;
                        if equal(a129, a130) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a131, a132)?
                };
                let a144 = {
                    let a141 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 32u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 32u8;
                        if equal(a135, a136) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a142 = {
                        let a139 = {
                            let a137 = v3;
                            let a138 = 16u8;
                            fixed_u8::bit_and(a137, a138)
                        };
                        let a140 = 16u8;
                        if equal(a139, a140) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a141, a142)?
                };
                nat_add(a143, a144)?
            };
            let a168 = {
                let a165 = {
                    let a153 = {
                        let a147 = {
                            let a145 = v3;
                            let a146 = 8u8;
                            fixed_u8::bit_and(a145, a146)
                        };
                        let a148 = 8u8;
                        if equal(a147, a148) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a154 = {
                        let a151 = {
                            let a149 = v3;
                            let a150 = 4u8;
                            fixed_u8::bit_and(a149, a150)
                        };
                        let a152 = 4u8;
                        if equal(a151, a152) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a153, a154)?
                };
                let a166 = {
                    let a163 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 2u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 2u8;
                        if equal(a157, a158) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a164 = {
                        let a161 = {
                            let a159 = v3;
                            let a160 = 1u8;
                            fixed_u8::bit_and(a159, a160)
                        };
                        let a162 = 1u8;
                        if equal(a161, a162) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a163, a164)?
                };
                nat_add(a165, a166)?
            };
            nat_add(a167, a168)
        }
    }
}

pub fn f21(v0: Adt5, v1: Adt1) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h169, _, _, _, _) => {
            h169
        }
    }, match v0.clone() {
        Adt5::C0(_, h170, _, _, _) => {
            h170
        }
    }, match v0 {
        Adt5::C0(_, _, h171, _, _) => {
            h171
        }
    }, Some(v1), false)
}

pub fn f22(v0: u64, v1: Adt5, v2: Adt2) -> R<Adt5> {
    let m172 = v2;
    match m172 {
        Adt2::C0 => {
            let a175 = {
                let a174 = match v1.clone() {
                    Adt5::C0(_, h173, _, _, _) => {
                        h173
                    }
                };
                length_list(a174)
            };
            let a176 = v0;
            if nat_lt(a175, a176) {
                Ok(Adt5::C0({
                    let a178 = match v1.clone() {
                        Adt5::C0(h177, _, _, _, _) => {
                            h177
                        }
                    };
                    let a179 = 1u64;
                    nat_add(a178, a179)?
                }, List::cons(List::<Adt4>::nil(), match v1.clone() {
                    Adt5::C0(_, h180, _, _, _) => {
                        h180
                    }
                }), match v1 {
                    Adt5::C0(_, _, h181, _, _) => {
                        h181
                    }
                }, None::<Adt1>, false))
            } else {
                Ok(f21(v1, Adt1::C0(1u64, f26(f11(Str::lit("max_scope_depth exceeded: configured ")), f27(v0)))))
            }
        }
        Adt2::C1 => {
            let m183 = match v1.clone() {
                Adt5::C0(_, h182, _, _, _) => {
                    h182
                }
            };
            match m183.uncons() {
                None => {
                    Ok(f21(v1, f10(f11(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f28(v1, Adt4::C1(std::rc::Rc::new(f29(v3))), v4)
                }
            }
        }
        Adt2::C2(v5) => {
            f28(v1.clone(), Adt4::C0(v5), match v1 {
                Adt5::C0(_, h184, _, _, _) => {
                    h184
                }
            })
        }
    }
}

pub fn f23(v0: Adt6, v1: List<Adt7>, v2: Adt4, v3: u64) -> R<Adt6> {
    let m185 = v2.clone();
    match m185 {
        Adt4::C0(_) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt4::C1(r188) => {
            let v4 = (*r188).clone();
            let m186 = f30(v4, v3)?;
            match m186.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                }
                Some((v5, v6)) => {
                    let m187 = v5;
                    match m187 {
                        Adt4::C0(v7) => {
                            f32(v0, v1, v7, v6, v3)
                        }
                        Adt4::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                        Adt4::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                        Adt4::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LRE form")))))
                        }
                    }
                }
            }
        }
        Adt4::C2 => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt4::C3(_, _) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
    }
}

pub fn f24(v0: Adt6, v1: List<Adt7>, v2: Bytes, v3: u64) -> Adt6 {
    let v4: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
        Adt6::C0(_, h189, _) => {
            h189
        }
    }, v3);
    let v5: List<Adt0> = {
        let (h190, _) = v4;
        h190
    };
    let a191 = v2.clone();
    let a192 = f11(Str::lit("seq"));
    if equal(a191, a192) {
        f35(Adt6::C0(v1.clone(), {
            let (_, h193) = v4;
            h193
        }, None::<Adt1>), v1, Adt0::C3(std::rc::Rc::new(f36(v5))))
    } else {
        if f37(v2.clone()) {
            let m194 = v5;
            match m194.uncons() {
                None => {
                    f31(v0, f10(f11(Str::lit("missing operand"))))
                }
                Some((v6, _)) => {
                    f35(Adt6::C0(v1.clone(), {
                        let (_, h195) = v4;
                        h195
                    }, None::<Adt1>), v1, f38(v2, v6))
                }
            }
        } else {
            let m196 = v5;
            match m196.uncons() {
                None => {
                    f31(v0, f10(f11(Str::lit("missing operand"))))
                }
                Some((v8, v9)) => {
                    let m197 = v9;
                    match m197.uncons() {
                        None => {
                            f31(v0, f10(f11(Str::lit("missing operand"))))
                        }
                        Some((v10, _)) => {
                            f35(Adt6::C0(v1.clone(), {
                                let (_, h198) = v4;
                                h198
                            }, None::<Adt1>), v1, f39(v2, v8, v10))
                        }
                    }
                }
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a199 = v0;
    let a200 = v1;
    index_bytes(a199, a200)
}

pub fn f26(v0: Bytes, v1: Bytes) -> Bytes {
    let a201 = v0;
    let a202 = v1;
    append_bytes(a201, a202)
}

pub fn f27(v0: u64) -> Bytes {
    f40(f41(), 10u64, v0)
}

pub fn f28(v0: Adt5, v1: Adt4, v2: List<List<Adt4>>) -> R<Adt5> {
    let m203 = v2;
    match m203.uncons() {
        None => {
            Ok(Adt5::C0({
                let a205 = match v0 {
                    Adt5::C0(h204, _, _, _, _) => {
                        h204
                    }
                };
                let a206 = 1u64;
                nat_add(a205, a206)?
            }, List::<List<Adt4>>::nil(), Some(v1), None::<Adt1>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt5::C0({
                let a208 = match v0.clone() {
                    Adt5::C0(h207, _, _, _, _) => {
                        h207
                    }
                };
                let a209 = 1u64;
                nat_add(a208, a209)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt5::C0(_, _, h210, _, _) => {
                    h210
                }
            }, None::<Adt1>, false))
        }
    }
}

pub fn f29(v0: List<Adt4>) -> Adt4 {
    f42(Fn4::F43, Adt4::C2, v0)
}

pub fn f30(v0: Adt4, v1: u64) -> R<List<Adt4>> {
    Ok(f44({
        let (_, h211) = {
            let (h212, _) = f45(Fn5::F46, {
                let a213 = v1;
                let a214 = 1u64;
                nat_add(a213, a214)?
            }, (v0, List::<Adt4>::nil()));
            h212
        };
        h211
    }))
}

pub fn f31(v0: Adt6, v1: Adt1) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h215, _, _) => {
            h215
        }
    }, match v0 {
        Adt6::C0(_, h216, _) => {
            h216
        }
    }, Some(v1))
}

pub fn f32(v0: Adt6, v1: List<Adt7>, v2: Bytes, v3: List<Adt4>, v4: u64) -> R<Adt6> {
    let v5: u64 = {
        let a217 = v3.clone();
        length_list(a217)
    };
    let a218 = v2.clone();
    let a219 = f11(Str::lit("form"));
    if equal(a218, a219) {
        f47(v0, v1, v3, v4)
    } else {
        let a220 = v2.clone();
        let a221 = f11(Str::lit("self-form"));
        if equal(a220, a221) {
            f48(v0, v1, v3, v4, f11(Str::lit("a form ID")), f11(Str::lit("a self form is `(self-form form-id)`")), 0u64)
        } else {
            let a222 = v2.clone();
            let a223 = f11(Str::lit("slot"));
            if equal(a222, a223) {
                f49(v0, v1, v3, v4)
            } else {
                let a224 = v2.clone();
                let a225 = f11(Str::lit("space"));
                if equal(a224, a225) {
                    let a226 = v5;
                    let a227 = 0u64;
                    if nat_eq(a226, a227) {
                        Ok(f35(v0, v1, Adt0::C4))
                    } else {
                        Ok(f31(v0, f10(f11(Str::lit("`(space)` takes no operand")))))
                    }
                } else {
                    let a228 = v2.clone();
                    let a229 = f11(Str::lit("token"));
                    if equal(a228, a229) {
                        f48(v0, v1, v3, v4, f11(Str::lit("a renderer-token ID")), f11(Str::lit("a token is `(token renderer-token-id)`")), 1u64)
                    } else {
                        let a230 = v2.clone();
                        let a231 = f11(Str::lit("operator-name"));
                        if equal(a230, a231) {
                            f48(v0, v1, v3, v4, f11(Str::lit("an operator name")), f11(Str::lit("an operator name is `(operator-name identifier)`")), 2u64)
                        } else {
                            let a232 = v2.clone();
                            let a233 = f11(Str::lit("seq"));
                            if equal(a232, a233) {
                                let a234 = v5;
                                let a235 = 0u64;
                                if nat_eq(a234, a235) {
                                    Ok(f31(v0, f10(f11(Str::lit("`seq` has at least one item")))))
                                } else {
                                    Ok(Adt6::C0(f50(v3, List::cons(Adt7::C1(v2, v5), v1)), match v0 {
                                        Adt6::C0(_, h236, _) => {
                                            h236
                                        }
                                    }, None::<Adt1>))
                                }
                            } else {
                                if f37(v2.clone()) {
                                    let a237 = v5;
                                    let a238 = 1u64;
                                    if nat_eq(a237, a238) {
                                        Ok(Adt6::C0(List::cons(Adt7::C0(std::rc::Rc::new(f51(v3, 0u64))), List::cons(Adt7::C1(v2, 1u64), v1)), match v0 {
                                            Adt6::C0(_, h239, _) => {
                                                h239
                                            }
                                        }, None::<Adt1>))
                                    } else {
                                        Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" takes exactly one operand"))))))
                                    }
                                } else {
                                    if f53(v2.clone()) {
                                        let a240 = v5;
                                        let a241 = 2u64;
                                        if nat_eq(a240, a241) {
                                            Ok(Adt6::C0(List::cons(Adt7::C0(std::rc::Rc::new(f51(v3.clone(), 0u64))), List::cons(Adt7::C0(std::rc::Rc::new(f51(v3, 1u64))), List::cons(Adt7::C1(v2, 2u64), v1))), match v0 {
                                                Adt6::C0(_, h242, _) => {
                                                    h242
                                                }
                                            }, None::<Adt1>))
                                        } else {
                                            Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" takes exactly two operands"))))))
                                        }
                                    } else {
                                        Ok(f31(v0, f10(f26(f52(v2), f11(Str::lit(" is not an LRE form"))))))
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

pub fn f33(v0: Adt4) -> Bytes {
    let m243 = v0;
    match m243 {
        Adt4::C0(v1) => {
            f52(v1)
        }
        Adt4::C1(r247) => {
            let v2 = (*r247).clone();
            let m244 = v2;
            match m244 {
                Adt4::C0(_) => {
                    f11(Str::lit("a list"))
                }
                Adt4::C1(_) => {
                    f11(Str::lit("a list"))
                }
                Adt4::C2 => {
                    f11(Str::lit("a list"))
                }
                Adt4::C3(r246, _) => {
                    let v3 = (*r246).clone();
                    let m245 = v3;
                    match m245 {
                        Adt4::C0(v5) => {
                            f26(f26(f11(Str::lit("`(")), v5), f11(Str::lit(" ...)`")))
                        }
                        Adt4::C1(_) => {
                            f11(Str::lit("a list"))
                        }
                        Adt4::C2 => {
                            f11(Str::lit("a list"))
                        }
                        Adt4::C3(_, _) => {
                            f11(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt4::C2 => {
            f11(Str::lit("a list"))
        }
        Adt4::C3(_, _) => {
            f11(Str::lit("a list"))
        }
    }
}

pub fn f34(v0: List<Adt0>, v1: u64) -> (List<Adt0>, List<Adt0>) {
    let (h248, _) = f54(Fn6::F55, v1, (List::<Adt0>::nil(), v0));
    h248
}

pub fn f35(v0: Adt6, v1: List<Adt7>, v2: Adt0) -> Adt6 {
    Adt6::C0(v1, List::cons(v2, match v0 {
        Adt6::C0(_, h249, _) => {
            h249
        }
    }), None::<Adt1>)
}

pub fn f36(v0: List<Adt0>) -> Adt0 {
    f56(Fn7::F57, Adt0::C13, f58(v0))
}

pub fn f37(v0: Bytes) -> bool {
    let a250 = v0.clone();
    let a251 = f11(Str::lit("group"));
    if equal(a250, a251) {
        true
    } else {
        let a252 = v0.clone();
        let a253 = f11(Str::lit("paren"));
        if equal(a252, a253) {
            true
        } else {
            let a254 = v0;
            let a255 = f11(Str::lit("bracket"));
            equal(a254, a255)
        }
    }
}

pub fn f38(v0: Bytes, v1: Adt0) -> Adt0 {
    let a256 = v0.clone();
    let a257 = f11(Str::lit("group"));
    if equal(a256, a257) {
        Adt0::C6(std::rc::Rc::new(v1))
    } else {
        let a258 = v0;
        let a259 = f11(Str::lit("paren"));
        if equal(a258, a259) {
            Adt0::C7(std::rc::Rc::new(v1))
        } else {
            Adt0::C8(std::rc::Rc::new(v1))
        }
    }
}

pub fn f39(v0: Bytes, v1: Adt0, v2: Adt0) -> Adt0 {
    let a260 = v0.clone();
    let a261 = f11(Str::lit("sub"));
    if equal(a260, a261) {
        Adt0::C9(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
    } else {
        let a262 = v0;
        let a263 = f11(Str::lit("sup"));
        if equal(a262, a263) {
            Adt0::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        } else {
            Adt0::C11(std::rc::Rc::new(v1), std::rc::Rc::new(v2))
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a264 = v2;
    let a265 = 0u64;
    if nat_eq(a264, a265) {
        f59(v0, 0u64)
    } else {
        let (_, h266) = {
            let (h267, _) = f60(Fn8::F61(v1, v0), v2, (v2, Bytes::lit(&[])));
            h267
        };
        h266
    }
}

pub fn f41() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f42(v0: Fn4, p271: Adt4, p272: List<Adt4>) -> Adt4 {
    let m270 = (p271, p272);
    {
        let mut m270 = m270;
        loop {
            let step = {
                let (v1, v2) = m270;
                let m268 = v2;
                match m268.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt4>), Adt4>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt4>), Adt4>(({
                            let c269: Fn4 = v0.clone();
                            c269.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt4, v1: Adt4) -> Adt4 {
    Adt4::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f44(v0: List<Adt4>) -> List<Adt4> {
    f62(Fn9::F63, List::<Adt4>::nil(), v0)
}

pub fn f45(v0: Fn5, p277: u64, p278: (Adt4, List<Adt4>)) -> ((Adt4, List<Adt4>), bool) {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m273 = v1;
                if m273 == 0 {
                    Err::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v2, false))
                } else {
                    let v3 = m273 - 1;
                    let m275 = {
                        let c274: Fn5 = v0.clone();
                        c274.apply(v2.clone())
                    };
                    match m275 {
                        None => {
                            Err::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt4, List<Adt4>)), ((Adt4, List<Adt4>), bool)>((v3, v4))
                        }
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

pub fn f46(v0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
    f64(v0)
}

pub fn f47(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64) -> R<Adt6> {
    let a280 = {
        let a279 = v2.clone();
        length_list(a279)
    };
    let a281 = 2u64;
    if nat_eq(a280, a281) {
        let m282 = f65(f51(v2.clone(), 0u64), f11(Str::lit("a qualified ID")), v3);
        match m282 {
            Ok(v4) => {
                let m283 = f66(v4)?;
                match m283 {
                    Ok(v5) => {
                        let m284 = f65(f51(v2, 1u64), f11(Str::lit("a form ID")), v3);
                        match m284 {
                            Ok(v6) => {
                                if f67(v6.clone())? {
                                    Ok(f35(v0, v1, Adt0::C0({
                                        let (h285, _) = v5;
                                        h285
                                    }, {
                                        let (_, h286) = v5;
                                        h286
                                    }, v6)))
                                } else {
                                    Ok(f31(v0, f10(f26(f52(v6), f11(Str::lit(" is not a form ID"))))))
                                }
                            }
                            Err(v7) => {
                                Ok(f31(v0, v7))
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(f31(v0, v8))
                    }
                }
            }
            Err(v9) => {
                Ok(f31(v0, v9))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("a form reference is `(form qualified-id form-id)`")))))
    }
}

pub fn f48(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64, v4: Bytes, v5: Bytes, v6: u64) -> R<Adt6> {
    let a288 = {
        let a287 = v2.clone();
        length_list(a287)
    };
    let a289 = 1u64;
    if nat_eq(a288, a289) {
        let m290 = f65(f51(v2, 0u64), v4, v3);
        match m290 {
            Ok(v7) => {
                let a291 = v6;
                let a292 = 0u64;
                if nat_eq(a291, a292) {
                    if f67(v7.clone())? {
                        Ok(f35(v0, v1, Adt0::C1(v7)))
                    } else {
                        Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not a form ID"))))))
                    }
                } else {
                    let a293 = v6;
                    let a294 = 1u64;
                    if nat_eq(a293, a294) {
                        if f68(v7.clone())? {
                            Ok(f35(v0, v1, Adt0::C5(v7)))
                        } else {
                            Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not a renderer-token ID"))))))
                        }
                    } else {
                        if f69(v7.clone())? {
                            Ok(f35(v0, v1, Adt0::C12(v7)))
                        } else {
                            Ok(f31(v0, f10(f26(f52(v7), f11(Str::lit(" is not an operator name"))))))
                        }
                    }
                }
            }
            Err(v8) => {
                Ok(f31(v0, v8))
            }
        }
    } else {
        Ok(f31(v0, f10(v5)))
    }
}

pub fn f49(v0: Adt6, v1: List<Adt7>, v2: List<Adt4>, v3: u64) -> R<Adt6> {
    let a296 = {
        let a295 = v2.clone();
        length_list(a295)
    };
    let a297 = 1u64;
    if nat_eq(a296, a297) {
        let m298 = f65(f51(v2, 0u64), f11(Str::lit("a slot index")), v3);
        match m298 {
            Ok(v4) => {
                let m299 = f70(v4.clone())?;
                match m299 {
                    None => {
                        let v6: u64 = f71(v4.clone())?;
                        let a300 = 4294967295u64;
                        let a301 = v6;
                        if nat_lt(a300, a301) {
                            Ok(f31(v0, f10(f26(f52(v4), f11(Str::lit(" is not a slot index"))))))
                        } else {
                            Ok(f35(v0, v1, Adt0::C2(v6)))
                        }
                    }
                    Some(_) => {
                        Ok(f31(v0, f10(f26(f52(v4), f11(Str::lit(" is not a slot index"))))))
                    }
                }
            }
            Err(v7) => {
                Ok(f31(v0, v7))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("a slot is `(slot decimal)`")))))
    }
}

pub fn f50(v0: List<Adt4>, v1: List<Adt7>) -> List<Adt7> {
    f72(Fn10::F73, v1, f44(v0))
}

pub fn f51(v0: List<Adt4>, v1: u64) -> Adt4 {
    let m304 = {
        let a302 = v0;
        let a303 = v1;
        index_list(a302, a303)
    };
    match m304 {
        None => {
            Adt4::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f52(v0: Bytes) -> Bytes {
    f26(f26(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f53(v0: Bytes) -> bool {
    let a305 = v0.clone();
    let a306 = f11(Str::lit("sub"));
    if equal(a305, a306) {
        true
    } else {
        let a307 = v0.clone();
        let a308 = f11(Str::lit("sup"));
        if equal(a307, a308) {
            true
        } else {
            let a309 = v0;
            let a310 = f11(Str::lit("frac"));
            equal(a309, a310)
        }
    }
}

pub fn f54(v0: Fn6, p315: u64, p316: (List<Adt0>, List<Adt0>)) -> ((List<Adt0>, List<Adt0>), bool) {
    let m314 = (p315, p316);
    {
        let mut m314 = m314;
        loop {
            let step = {
                let (v1, v2) = m314;
                let m311 = v1;
                if m311 == 0 {
                    Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, false))
                } else {
                    let v3 = m311 - 1;
                    let m313 = {
                        let c312: Fn6 = v0.clone();
                        c312.apply(v2.clone())
                    };
                    match m313 {
                        None => {
                            Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
    let m318 = {
        let (_, h317) = v0;
        h317
    };
    match m318.uncons() {
        None => {
            None::<(List<Adt0>, List<Adt0>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h319, _) = v0;
                h319
            }), v2))
        }
    }
}

pub fn f56(v0: Fn7, p323: Adt0, p324: List<Adt0>) -> Adt0 {
    let m322 = (p323, p324);
    {
        let mut m322 = m322;
        loop {
            let step = {
                let (v1, v2) = m322;
                let m320 = v2;
                match m320.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c321: Fn7 = v0.clone();
                            c321.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C14(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f58(v0: List<Adt0>) -> List<Adt0> {
    f74(Fn11::F75, List::<Adt0>::nil(), v0)
}

pub fn f59(v0: Bytes, v1: u64) -> Bytes {
    let m328 = {
        let a325 = v0;
        let a326 = v1;
        let a327 = 1u64;
        slice_bytes(a325, a326, a327)
    };
    match m328 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f60(v0: Fn8, p333: u64, p334: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m332 = (p333, p334);
    {
        let mut m332 = m332;
        loop {
            let step = {
                let (v1, v2) = m332;
                let m329 = v1;
                if m329 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m329 - 1;
                    let m331 = {
                        let c330: Fn8 = v0.clone();
                        c330.apply(v2.clone())
                    };
                    match m331 {
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
                Ok(next) => { m332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f76(v1, v0, v2)
}

pub fn f62(v0: Fn9, p338: List<Adt4>, p339: List<Adt4>) -> List<Adt4> {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c336: Fn9 = v0.clone();
                            c336.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f64(v0: (Adt4, List<Adt4>)) -> Option<(Adt4, List<Adt4>)> {
    let m341 = {
        let (h340, _) = v0;
        h340
    };
    match m341 {
        Adt4::C0(_) => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C1(_) => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C2 => {
            None::<(Adt4, List<Adt4>)>
        }
        Adt4::C3(r343, r344) => {
            let v3 = (*r343).clone();
            let v4 = (*r344).clone();
            Some((v4, List::cons(v3, {
                let (_, h342) = v0;
                h342
            })))
        }
    }
}

pub fn f65(v0: Adt4, v1: Bytes, _: u64) -> Result<Bytes, Adt1> {
    let m345 = v0.clone();
    match m345 {
        Adt4::C0(v3) => {
            Ok::<Bytes, Adt1>(v3)
        }
        Adt4::C1(_) => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt4::C2 => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt4::C3(_, _) => {
            Err::<Bytes, Adt1>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
    }
}

pub fn f66(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt1>> {
    let m346 = f77(v0.clone(), f11(Str::lit("::")))?;
    match m346 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v0), f11(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f17(v0.clone(), 0u64, v1);
            let v3: Bytes = f17(v0.clone(), {
                let a347 = v1;
                let a348 = 2u64;
                nat_add(a347, a348)?
            }, {
                let a352 = {
                    let a349 = v0;
                    length_bytes(a349)
                };
                let a353 = {
                    let a350 = v1;
                    let a351 = 2u64;
                    nat_add(a350, a351)?
                };
                nat_sub(a352, a353)
            });
            if f78(v2.clone())? {
                if f79(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt1>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v3), f11(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt1>(f10(f26(f52(v2), f11(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<bool> {
    f68(v0)
}

pub fn f68(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a354 = v0.clone();
        length_bytes(a354)
    };
    let a355 = v1;
    let a356 = 0u64;
    if nat_eq(a355, a356) {
        Ok(false)
    } else {
        if f80(f20(v0.clone(), 0u64)?) {
            let a358 = {
                let (h357, _) = f81(Fn12::F82(v1, v0), v1, 1u64)?;
                h357
            };
            let a359 = v1;
            Ok(nat_eq(a358, a359))
        } else {
            Ok(false)
        }
    }
}

pub fn f69(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a360 = v0.clone();
        length_bytes(a360)
    };
    let a361 = v1;
    let a362 = 0u64;
    if nat_eq(a361, a362) {
        Ok(false)
    } else {
        if f83(f20(v0.clone(), 0u64)?) {
            let a364 = {
                let (h363, _) = f81(Fn12::F84(v1, v0), v1, 1u64)?;
                h363
            };
            let a365 = v1;
            Ok(nat_eq(a364, a365))
        } else {
            Ok(false)
        }
    }
}

pub fn f70(v0: Bytes) -> R<Option<Adt1>> {
    let v1: u64 = {
        let a366 = v0.clone();
        length_bytes(a366)
    };
    let a367 = v1;
    let a368 = 0u64;
    if if nat_eq(a367, a368) {
        true
    } else {
        !f85(v0.clone())?
    } {
        Ok(Some(f10(f26(f52(v0), f11(Str::lit(" is not a decimal"))))))
    } else {
        let a369 = 1u64;
        let a370 = v1;
        if if nat_lt(a369, a370) {
            let a371 = f20(v0.clone(), 0u64)?;
            let a372 = 48u64;
            nat_eq(a371, a372)
        } else {
            false
        } {
            Ok(Some(f10(f26(f52(v0), f11(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a373 = 20u64;
            let a374 = v1;
            if if nat_lt(a373, a374) {
                true
            } else {
                let a375 = v1;
                let a376 = 20u64;
                if nat_eq(a375, a376) {
                    let a381 = {
                        let a377 = v0.clone();
                        let a378 = f11(Str::lit("18446744073709551615"));
                        compare_bytes(a377, a378)
                    };
                    let a382 = {
                        let a379 = Bytes::lit(&[0x01]);
                        let a380 = Bytes::lit(&[]);
                        compare_bytes(a379, a380)
                    };
                    equal(a381, a382)
                } else {
                    false
                }
            } {
                Ok(Some(f10(f26(f52(v0), f11(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt1>)
            }
        }
    }
}

pub fn f71(v0: Bytes) -> R<u64> {
    let m383 = f86(v0)?;
    match m383 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f72(v0: Fn10, p387: List<Adt7>, p388: List<Adt4>) -> List<Adt7> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m384 = v2;
                match m384.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt4>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt4>), List<Adt7>>(({
                            let c385: Fn10 = v0.clone();
                            c385.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt7>, v1: Adt4) -> List<Adt7> {
    List::cons(Adt7::C0(std::rc::Rc::new(v1)), v0)
}

pub fn f74(v0: Fn11, p392: List<Adt0>, p393: List<Adt0>) -> List<Adt0> {
    let m391 = (p392, p393);
    {
        let mut m391 = m391;
        loop {
            let step = {
                let (v1, v2) = m391;
                let m389 = v2;
                match m389.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c390: Fn11 = v0.clone();
                            c390.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f76(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h394, _) = v2;
        h394
    };
    let a395 = v3;
    let a396 = 0u64;
    if nat_eq(a395, a396) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a397 = v3;
            let a398 = v1;
            let a399 = 0u64;
            nat_quot(a397, a398, a399)
        }, {
            let a404 = f59(v0, {
                let a400 = v3;
                let a401 = v1;
                let a402 = 0u64;
                nat_rem(a400, a401, a402)
            });
            let a405 = {
                let (_, h403) = v2;
                h403
            };
            append_bytes(a404, a405)
        }))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a406 = v0.clone();
        length_bytes(a406)
    };
    let v3: u64 = {
        let a407 = v1.clone();
        length_bytes(a407)
    };
    let v4: u64 = {
        let (h408, _) = f81(Fn12::F87(v1.clone(), v3, v0.clone(), v2), {
            let a409 = v2;
            let a410 = 1u64;
            nat_add(a409, a410)?
        }, 0u64)?;
        h408
    };
    if f88(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    f89(v0, false)
}

pub fn f79(v0: Bytes) -> R<bool> {
    f89(v0, true)
}

pub fn f80(v0: u64) -> bool {
    let a411 = v0;
    let a412 = 97u64;
    if nat_lt(a411, a412) {
        false
    } else {
        let a413 = v0;
        let a414 = 122u64;
        nat_le(a413, a414)
    }
}

pub fn f81(v0: Fn12, p419: u64, p420: u64) -> R<(u64, bool)> {
    let m418 = (p419, p420);
    {
        let mut m418 = m418;
        loop {
            let step = {
                let (v1, v2) = m418;
                let m415 = v1;
                if m415 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m415 - 1;
                    let m417 = {
                        let c416: Fn12 = v0.clone();
                        c416.apply(v2)?
                    };
                    match m417 {
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
                Ok(next) => { m418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f90(v1, v0, v2)
}

pub fn f83(v0: u64) -> bool {
    if f80(v0) {
        true
    } else {
        let a421 = v0;
        let a422 = 65u64;
        if nat_lt(a421, a422) {
            false
        } else {
            let a423 = v0;
            let a424 = 90u64;
            nat_le(a423, a424)
        }
    }
}

pub fn f84(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f91(v1, v0, v2)
}

pub fn f85(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a425 = v0.clone();
        length_bytes(a425)
    };
    let a429 = {
        let (h426, _) = f81(Fn12::F92(v1, v0), {
            let a427 = v1;
            let a428 = 1u64;
            nat_add(a427, a428)?
        }, 0u64)?;
        h426
    };
    let a430 = v1;
    Ok(nat_eq(a429, a430))
}

pub fn f86(v0: Bytes) -> R<Option<u64>> {
    if f93(v0.clone())? {
        Ok(Some({
            let (_, h431) = {
                let (h432, _) = f94(Fn13::F95(v0.clone()), {
                    let a433 = v0;
                    length_bytes(a433)
                }, (0u64, 0u64))?;
                h432
            };
            h431
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f88(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a436 = {
            let a434 = v4;
            let a435 = v1;
            nat_add(a434, a435)?
        };
        let a437 = v3;
        if nat_lt(a436, a437) {
            Ok(Some({
                let a438 = v4;
                let a439 = 1u64;
                nat_add(a438, a439)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f88(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m444 = {
        let a441 = v0;
        let a442 = v2;
        let a443 = {
            let a440 = v1.clone();
            length_bytes(a440)
        };
        slice_bytes(a441, a442, a443)
    };
    match m444 {
        None => {
            false
        }
        Some(v3) => {
            let a445 = v3;
            let a446 = v1;
            equal(a445, a446)
        }
    }
}

pub fn f89(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a447 = v0.clone();
        length_bytes(a447)
    };
    let v3: Adt8 = {
        let (h448, _) = f96(Fn14::F97(v1, v2, v0), {
            let a449 = v2;
            let a450 = 1u64;
            nat_add(a449, a450)?
        }, Adt8::C0(0u64, true, true, true))?;
        h448
    };
    if match v3.clone() {
        Adt8::C0(_, _, _, h451) => {
            h451
        }
    } {
        Ok(!match v3 {
            Adt8::C0(_, h452, _, _) => {
                h452
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f90(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a453 = v2;
    let a454 = v1;
    if nat_lt(a453, a454) {
        let v3: u64 = f20(v0, v2)?;
        if if f80(v3) {
            true
        } else {
            if f98(v3) {
                true
            } else {
                let a455 = v3;
                let a456 = 45u64;
                nat_eq(a455, a456)
            }
        } {
            Ok(Some({
                let a457 = v2;
                let a458 = 1u64;
                nat_add(a457, a458)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f91(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a459 = v2;
    let a460 = v1;
    if nat_lt(a459, a460) {
        let v3: u64 = f20(v0, v2)?;
        if if f99(v3) {
            true
        } else {
            let a461 = v3;
            let a462 = 95u64;
            nat_eq(a461, a462)
        } {
            Ok(Some({
                let a463 = v2;
                let a464 = 1u64;
                nat_add(a463, a464)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f92(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f100(v1, v0, v2)
}

pub fn f93(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a465 = v0.clone();
        length_bytes(a465)
    };
    let a466 = 0u64;
    let a467 = v1;
    if nat_lt(a466, a467) {
        let a468 = f101(v0.clone())?;
        let a469 = v1;
        if nat_eq(a468, a469) {
            let a470 = 1u64;
            let a471 = v1;
            if nat_lt(a470, a471) {
                let m474 = {
                    let a472 = v0;
                    let a473 = 0u64;
                    index_bytes(a472, a473)
                };
                match m474 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a475 = v2;
                        let a476 = 48u8;
                        Ok(!equal(a475, a476))
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

pub fn f94(v0: Fn13, p481: u64, p482: (u64, u64)) -> R<((u64, u64), bool)> {
    let m480 = (p481, p482);
    {
        let mut m480 = m480;
        loop {
            let step = {
                let (v1, v2) = m480;
                let m477 = v1;
                if m477 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m477 - 1;
                    let m479 = {
                        let c478: Fn13 = v0.clone();
                        c478.apply(v2)?
                    };
                    match m479 {
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
                Ok(next) => { m480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f102(v0, v1)
}

pub fn f96(v0: Fn14, p487: u64, p488: Adt8) -> R<(Adt8, bool)> {
    let m486 = (p487, p488);
    {
        let mut m486 = m486;
        loop {
            let step = {
                let (v1, v2) = m486;
                let m483 = v1;
                if m483 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m483 - 1;
                    let m485 = {
                        let c484: Fn14 = v0.clone();
                        c484.apply(v2.clone())?
                    };
                    match m485 {
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
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: bool, v1: u64, v2: Bytes, v3: Adt8) -> R<Option<Adt8>> {
    f103(v2, v1, v0, v3)
}

pub fn f98(v0: u64) -> bool {
    let a489 = v0;
    let a490 = 48u64;
    if nat_lt(a489, a490) {
        false
    } else {
        let a491 = v0;
        let a492 = 57u64;
        nat_le(a491, a492)
    }
}

pub fn f99(v0: u64) -> bool {
    if f83(v0) {
        true
    } else {
        f98(v0)
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a493 = v2;
    let a494 = v1;
    if nat_lt(a493, a494) {
        if f98(f20(v0, v2)?) {
            Ok(Some({
                let a495 = v2;
                let a496 = 1u64;
                nat_add(a495, a496)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h497, _) = f81(Fn12::F104(v0.clone()), {
        let a498 = v0;
        length_bytes(a498)
    }, 0u64)?;
    Ok(h497)
}

pub fn f102(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m502 = {
        let a500 = v0;
        let a501 = {
            let (h499, _) = v1;
            h499
        };
        index_bytes(a500, a501)
    };
    match m502 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a504 = {
                    let (h503, _) = v1;
                    h503
                };
                let a505 = 1u64;
                nat_add(a504, a505)?
            }, {
                let a509 = {
                    let a507 = {
                        let (_, h506) = v1;
                        h506
                    };
                    let a508 = 10u64;
                    nat_mul(a507, a508)?
                };
                let a510 = f105(v2)?;
                nat_add(a509, a510)?
            })))
        }
    }
}

pub fn f103(v0: Bytes, v1: u64, v2: bool, v3: Adt8) -> R<Option<Adt8>> {
    if match v3.clone() {
        Adt8::C0(_, _, _, h511) => {
            h511
        }
    } {
        let a513 = match v3.clone() {
            Adt8::C0(h512, _, _, _) => {
                h512
            }
        };
        let a514 = v1;
        if nat_lt(a513, a514) {
            let v4: u64 = f20(v0, match v3.clone() {
                Adt8::C0(h515, _, _, _) => {
                    h515
                }
            })?;
            let a516 = v4;
            let a517 = 46u64;
            if nat_eq(a516, a517) {
                if match v3.clone() {
                    Adt8::C0(_, h518, _, _) => {
                        h518
                    }
                } {
                    Ok(f106(v3))
                } else {
                    f107(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt8::C0(_, h519, _, _) => {
                        h519
                    }
                } {
                    if f80(v4) {
                        f107(v3.clone(), false, match v3 {
                            Adt8::C0(_, _, h520, _) => {
                                h520
                            }
                        })
                    } else {
                        Ok(f106(v3))
                    }
                } else {
                    if if f80(v4) {
                        true
                    } else {
                        f98(v4)
                    } {
                        f107(v3.clone(), false, match v3 {
                            Adt8::C0(_, _, h521, _) => {
                                h521
                            }
                        })
                    } else {
                        let a522 = v4;
                        let a523 = 45u64;
                        if if nat_eq(a522, a523) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt8::C0(_, _, h524, _) => {
                                        h524
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f107(v3.clone(), false, match v3 {
                                Adt8::C0(_, _, h525, _) => {
                                    h525
                                }
                            })
                        } else {
                            Ok(f106(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt8>)
        }
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m528 = {
        let a526 = v0;
        let a527 = v1;
        index_bytes(a526, a527)
    };
    match m528 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f108(v2)? {
                Ok(Some({
                    let a529 = v1;
                    let a530 = 1u64;
                    nat_add(a529, a530)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f105(v0: u8) -> R<u64> {
    let a577 = {
        let v1: u8 = v0;
        let a575 = {
            let a551 = {
                let a539 = {
                    let a533 = {
                        let a531 = v1;
                        let a532 = 128u8;
                        fixed_u8::bit_and(a531, a532)
                    };
                    let a534 = 128u8;
                    if equal(a533, a534) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a540 = {
                    let a537 = {
                        let a535 = v1;
                        let a536 = 64u8;
                        fixed_u8::bit_and(a535, a536)
                    };
                    let a538 = 64u8;
                    if equal(a537, a538) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a539, a540)?
            };
            let a552 = {
                let a549 = {
                    let a543 = {
                        let a541 = v1;
                        let a542 = 32u8;
                        fixed_u8::bit_and(a541, a542)
                    };
                    let a544 = 32u8;
                    if equal(a543, a544) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a550 = {
                    let a547 = {
                        let a545 = v1;
                        let a546 = 16u8;
                        fixed_u8::bit_and(a545, a546)
                    };
                    let a548 = 16u8;
                    if equal(a547, a548) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a549, a550)?
            };
            nat_add(a551, a552)?
        };
        let a576 = {
            let a573 = {
                let a561 = {
                    let a555 = {
                        let a553 = v1;
                        let a554 = 8u8;
                        fixed_u8::bit_and(a553, a554)
                    };
                    let a556 = 8u8;
                    if equal(a555, a556) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a562 = {
                    let a559 = {
                        let a557 = v1;
                        let a558 = 4u8;
                        fixed_u8::bit_and(a557, a558)
                    };
                    let a560 = 4u8;
                    if equal(a559, a560) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a561, a562)?
            };
            let a574 = {
                let a571 = {
                    let a565 = {
                        let a563 = v1;
                        let a564 = 2u8;
                        fixed_u8::bit_and(a563, a564)
                    };
                    let a566 = 2u8;
                    if equal(a565, a566) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a572 = {
                    let a569 = {
                        let a567 = v1;
                        let a568 = 1u8;
                        fixed_u8::bit_and(a567, a568)
                    };
                    let a570 = 1u8;
                    if equal(a569, a570) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a571, a572)?
            };
            nat_add(a573, a574)?
        };
        nat_add(a575, a576)?
    };
    let a578 = 48u64;
    Ok(nat_sub(a577, a578))
}

pub fn f106(v0: Adt8) -> Option<Adt8> {
    Some(Adt8::C0(match v0.clone() {
        Adt8::C0(h579, _, _, _) => {
            h579
        }
    }, match v0.clone() {
        Adt8::C0(_, h580, _, _) => {
            h580
        }
    }, match v0 {
        Adt8::C0(_, _, h581, _) => {
            h581
        }
    }, false))
}

pub fn f107(v0: Adt8, v1: bool, v2: bool) -> R<Option<Adt8>> {
    Ok(Some(Adt8::C0({
        let a583 = match v0 {
            Adt8::C0(h582, _, _, _) => {
                h582
            }
        };
        let a584 = 1u64;
        nat_add(a583, a584)?
    }, v1, v2, true)))
}

pub fn f108(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a629 = {
            let a605 = {
                let a593 = {
                    let a587 = {
                        let a585 = v1;
                        let a586 = 128u8;
                        fixed_u8::bit_and(a585, a586)
                    };
                    let a588 = 128u8;
                    if equal(a587, a588) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a594 = {
                    let a591 = {
                        let a589 = v1;
                        let a590 = 64u8;
                        fixed_u8::bit_and(a589, a590)
                    };
                    let a592 = 64u8;
                    if equal(a591, a592) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a593, a594)?
            };
            let a606 = {
                let a603 = {
                    let a597 = {
                        let a595 = v1;
                        let a596 = 32u8;
                        fixed_u8::bit_and(a595, a596)
                    };
                    let a598 = 32u8;
                    if equal(a597, a598) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a604 = {
                    let a601 = {
                        let a599 = v1;
                        let a600 = 16u8;
                        fixed_u8::bit_and(a599, a600)
                    };
                    let a602 = 16u8;
                    if equal(a601, a602) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a603, a604)?
            };
            nat_add(a605, a606)?
        };
        let a630 = {
            let a627 = {
                let a615 = {
                    let a609 = {
                        let a607 = v1;
                        let a608 = 8u8;
                        fixed_u8::bit_and(a607, a608)
                    };
                    let a610 = 8u8;
                    if equal(a609, a610) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a616 = {
                    let a613 = {
                        let a611 = v1;
                        let a612 = 4u8;
                        fixed_u8::bit_and(a611, a612)
                    };
                    let a614 = 4u8;
                    if equal(a613, a614) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a615, a616)?
            };
            let a628 = {
                let a625 = {
                    let a619 = {
                        let a617 = v1;
                        let a618 = 2u8;
                        fixed_u8::bit_and(a617, a618)
                    };
                    let a620 = 2u8;
                    if equal(a619, a620) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a626 = {
                    let a623 = {
                        let a621 = v1;
                        let a622 = 1u8;
                        fixed_u8::bit_and(a621, a622)
                    };
                    let a624 = 1u8;
                    if equal(a623, a624) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a625, a626)?
            };
            nat_add(a627, a628)?
        };
        nat_add(a629, a630)?
    };
    let a631 = 48u64;
    let a632 = v2;
    if nat_le(a631, a632) {
        let a633 = v2;
        let a634 = 57u64;
        Ok(nat_le(a633, a634))
    } else {
        Ok(false)
    }
}

pub fn run(p0: Bytes, p1: u64) -> R<Result<Adt0, Adt1>> {
    f0(p0, p1)
}
