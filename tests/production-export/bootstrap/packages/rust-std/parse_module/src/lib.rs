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
    C0(Adt1, u64, u64, u64, u64, u64, u64, Bytes),
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
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt3>, Bytes, std::rc::Rc<Adt5>, List<Adt5>, List<Adt5>, u64, u64, std::rc::Rc<Adt6>, Option<Adt14>, Option<Adt14>, Option<Adt14>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64, u64, Adt1, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Bytes),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C2(std::rc::Rc<Adt5>, u64, u64, Option<(u64, u64)>, std::rc::Rc<Adt6>, u64),
    C3(std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8, std::rc::Rc<Adt5>, Option<Adt5>, std::rc::Rc<Adt9>, std::rc::Rc<Adt11>, Option<Adt12>, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Adt10, List<Bytes>, u64, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C2(std::rc::Rc<Adt12>, u64),
    C3(std::rc::Rc<Adt11>),
    C4(std::rc::Rc<Adt5>, std::rc::Rc<Adt11>, std::rc::Rc<Adt12>, u64),
    C5(std::rc::Rc<Adt5>, List<(bool, Adt5)>, u64),
    C6(std::rc::Rc<Adt5>, List<Adt5>, u64),
    C7(std::rc::Rc<Adt5>, std::rc::Rc<Adt12>, u64),
    C8(std::rc::Rc<Adt12>, u64),
    C9(bool, std::rc::Rc<Adt5>, std::rc::Rc<Adt12>, u64),
    C10(std::rc::Rc<Adt5>, List<Adt13>, u64),
    C11(u64, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C12(std::rc::Rc<Adt5>, List<Bytes>, std::rc::Rc<Adt5>, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(std::rc::Rc<Adt5>, bool, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Bytes, Option<Adt16>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, List<Adt0>, u64, u64),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, Bytes, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, List<Adt3>, Option<Adt15>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(std::rc::Rc<Adt5>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, List<Bytes>, bool, bool, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(std::rc::Rc<Adt19>, List<Adt23>, std::rc::Rc<Adt25>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(std::rc::Rc<Adt24>, std::rc::Rc<Adt5>, u64, u64, Option<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, u64, List<Adt6>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(std::rc::Rc<Adt24>),
    C1(std::rc::Rc<Adt26>, std::rc::Rc<Adt27>, List<Adt28>),
    C2(std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u64, Bytes, u64, List<Adt12>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(std::rc::Rc<Adt24>, Bytes, Adt8, std::rc::Rc<Adt5>, Option<Adt5>, std::rc::Rc<Adt9>, std::rc::Rc<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(std::rc::Rc<Adt26>, std::rc::Rc<Adt5>, std::rc::Rc<Adt11>, u64),
    C1(std::rc::Rc<Adt29>, u64),
    C2(std::rc::Rc<Adt30>, std::rc::Rc<Adt5>, List<Bytes>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(std::rc::Rc<Adt26>, Bytes, Bytes, Option<Adt5>, List<(u64, Adt12)>, u64),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(std::rc::Rc<Adt26>, bool, Bytes, std::rc::Rc<Adt5>, List<Adt31>, u64),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(std::rc::Rc<Adt5>, List<Bytes>, std::rc::Rc<Adt5>, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(std::rc::Rc<Adt11>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(std::rc::Rc<Adt9>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(std::rc::Rc<Adt12>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F20(Bytes, std::rc::Rc<Adt17>, Bytes),
    F124(std::rc::Rc<Adt17>),
}

impl Fn0 {
    pub fn apply(&self, p0: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
        match self {
            Fn0::F20(k0, k1, k2) => f20(k0.clone(), (**k1).clone(), k2.clone(), p0),
            Fn0::F124(k0) => f124((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F31,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn1::F31 => f31(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(std::rc::Rc<Adt4>, std::rc::Rc<Adt17>, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
        match self {
            Fn2::F33(k0, k1, k2) => f33((**k0).clone(), (**k1).clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F37(std::rc::Rc<Adt17>, bool),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt21) -> R<Option<Adt21>> {
        match self {
            Fn3::F37(k0, k1) => f37((**k0).clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F45(std::rc::Rc<Adt17>),
    F134(Bytes),
    F157(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F45(k0) => f45((**k0).clone(), p0),
            Fn4::F134(k0) => f134(k0.clone(), p0),
            Fn4::F157(k0) => f157(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F53(std::rc::Rc<Adt17>),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt22) -> R<Option<Adt22>> {
        match self {
            Fn5::F53(k0) => f53((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F64(std::rc::Rc<Adt17>),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn6::F64(k0) => f64((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F66,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn7::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F70,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> R<Option<List<Bytes>>> {
        match self {
            Fn8::F70 => f70(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F72,
    F137,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn9::F72 => Ok(f72(p0, p1)),
            Fn9::F137 => f137(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F88(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt32) -> R<Option<Adt32>> {
        match self {
            Fn10::F88(k0) => f88(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F90,
}

impl Fn11 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn11::F90 => f90(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F102,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn12::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F109(std::rc::Rc<Adt17>),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt35) -> R<Option<Adt35>> {
        match self {
            Fn13::F109(k0) => f109((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F113,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn14::F113 => f113(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F122(std::rc::Rc<Adt17>),
}

impl Fn15 {
    pub fn apply(&self, p0: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
        match self {
            Fn15::F122(k0) => f122((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F126(std::rc::Rc<Adt17>),
}

impl Fn16 {
    pub fn apply(&self, p0: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
        match self {
            Fn16::F126(k0) => f126((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F136,
    F167,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<Bytes>, Option<Bytes>), p1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
        match self {
            Fn17::F136 => f136(p0, p1),
            Fn17::F167 => Ok(f167(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F140(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt37) -> R<Option<Adt37>> {
        match self {
            Fn18::F140(k0) => f140(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F142,
}

impl Fn19 {
    pub fn apply(&self, p0: Adt12, p1: (u64, Adt12)) -> Adt12 {
        match self {
            Fn19::F142 => f142(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F144(u8, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn20::F144(k0, k1) => f144(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F146,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt12, p1: Adt31) -> Adt12 {
        match self {
            Fn21::F146 => f146(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F149,
}

impl Fn22 {
    pub fn apply(&self, p0: List<(bool, Adt5)>, p1: (bool, Adt5)) -> List<(bool, Adt5)> {
        match self {
            Fn22::F149 => f149(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F153,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt13) -> List<Adt13> {
        match self {
            Fn23::F153 => f153(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F171(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn24::F171(k0) => f171(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F173(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn25::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: Bytes, v3: u64) -> R<Result<Adt2, Adt15>> {
    let v4: Adt17 = Adt17::C0(v0, v1.clone(), {
        let a1 = v1;
        length_list(a1)
    }, v3);
    let v5: Adt18 = f1(v4.clone(), Adt19::C0(0u64, List::<Adt3>::nil(), None::<Adt15>))?;
    let v6: Adt19 = if f2(match v5.clone() {
        Adt18::C0(_, h2, _) => {
            h2
        }
    }, f3(Str::lit("lexlean"))) {
        match v5 {
            Adt18::C0(_, _, h3) => {
                (*h3).clone()
            }
        }
    } else {
        f4(v4.clone(), match v5 {
            Adt18::C0(_, _, h4) => {
                (*h4).clone()
            }
        }, f3(Str::lit("a module begins with \\begin{lexlean}{module-name}")))?
    };
    let v7: Adt20 = f5(v4.clone(), v6)?;
    let v8: Bytes = {
        let m7 = match v7.clone() {
            Adt20::C0(h6, _) => {
                (*h6).clone()
            }
        };
        match m7 {
            Adt5::C0(_, _, h5) => {
                h5
            }
        }
    };
    let v9: Adt19 = f6(v4.clone(), match v7.clone() {
        Adt20::C0(_, h8) => {
            (*h8).clone()
        }
    }, match v7.clone() {
        Adt20::C0(h9, _) => {
            (*h9).clone()
        }
    }, f3(Str::lit("env.lexlean")))?;
    let v10: Adt19 = if f2(v8.clone(), v2.clone()) {
        v9
    } else {
        f4(v4.clone(), v9, f7(f7(f7(f3(Str::lit("the declared module ")), f8(v8)), f3(Str::lit(" does not match the source path module "))), f8(v2)))?
    };
    let v11: (Adt19, List<Adt5>) = f9(v4.clone(), v10, f3(Str::lit("\\useglossary")), f3(Str::lit("useglossary")))?;
    let v12: (Adt19, List<Adt5>) = f9(v4.clone(), {
        let (h10, _) = v11;
        h10
    }, f3(Str::lit("\\importmodule")), f3(Str::lit("importmodule")))?;
    let v13: Adt19 = if f10(v4.clone(), {
        let m13 = {
            let (h12, _) = v12.clone();
            h12
        };
        match m13 {
            Adt19::C0(h11, _, _) => {
                h11
            }
        }
    }, f3(Str::lit("\\useglossary")))? {
        f4(v4.clone(), {
            let (h14, _) = v12;
            h14
        }, f3(Str::lit("glossary imports occur before module imports in the header")))?
    } else {
        let (h15, _) = v12;
        h15
    };
    let v14: Adt20 = f5(v4.clone(), f11(v4.clone(), v13, f3(Str::lit("\\title")))?)?;
    let v15: Adt19 = if f10(v4.clone(), {
        let m18 = match v14.clone() {
            Adt20::C0(_, h17) => {
                (*h17).clone()
            }
        };
        match m18 {
            Adt19::C0(h16, _, _) => {
                h16
            }
        }
    }, f3(Str::lit("\\title")))? {
        f4(v4.clone(), match v14.clone() {
            Adt20::C0(_, h19) => {
                (*h19).clone()
            }
        }, f3(Str::lit("each module has exactly one title")))?
    } else {
        match v14.clone() {
            Adt20::C0(_, h20) => {
                (*h20).clone()
            }
        }
    };
    f12(v4, v15, match v7 {
        Adt20::C0(h21, _) => {
            (*h21).clone()
        }
    }, f13({
        let (_, h22) = v11;
        h22
    }), f13({
        let (_, h23) = v12;
        h23
    }), match v14 {
        Adt20::C0(h24, _) => {
            (*h24).clone()
        }
    })
}

pub fn f1(v0: Adt17, v1: Adt19) -> R<Adt18> {
    let v2: Adt19 = f11(v0.clone(), v1, f3(Str::lit("\\begin")))?;
    let v3: Adt20 = f5(v0.clone(), v2.clone())?;
    Ok(Adt18::C0({
        let a26 = match v2 {
            Adt19::C0(h25, _, _) => {
                h25
            }
        };
        let a27 = 1u64;
        nat_sub(a26, a27)
    }, {
        let m30 = match v3.clone() {
            Adt20::C0(h29, _) => {
                (*h29).clone()
            }
        };
        match m30 {
            Adt5::C0(_, _, h28) => {
                h28
            }
        }
    }, std::rc::Rc::new(f14(v0, match v3.clone() {
        Adt20::C0(_, h31) => {
            (*h31).clone()
        }
    }, match v3.clone() {
        Adt20::C0(h32, _) => {
            (*h32).clone()
        }
    }, f15({
        let m35 = match v3 {
            Adt20::C0(h34, _) => {
                (*h34).clone()
            }
        };
        match m35 {
            Adt5::C0(_, _, h33) => {
                h33
            }
        }
    }))?)))
}

pub fn f2(v0: Bytes, v1: Bytes) -> bool {
    let a36 = v0;
    let a37 = v1;
    equal(a36, a37)
}

pub fn f3(v0: Str) -> Bytes {
    let a38 = v0;
    utf8_encode(a38)
}

pub fn f4(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    f16(v0, v1, f3(Str::lit("LLP2003")), v2)
}

pub fn f5(v0: Adt17, v1: Adt19) -> R<Adt20> {
    f17(v0, v1, false)
}

pub fn f6(v0: Adt17, v1: Adt19, v2: Adt5, v3: Bytes) -> R<Adt19> {
    f14(v0, v1, v2, f18(v3))
}

pub fn f7(v0: Bytes, v1: Bytes) -> Bytes {
    let a39 = v0;
    let a40 = v1;
    append_bytes(a39, a40)
}

pub fn f8(v0: Bytes) -> Bytes {
    f7(f7(f3(Str::lit("`")), v0), f3(Str::lit("`")))
}

pub fn f9(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes) -> R<(Adt19, List<Adt5>)> {
    let (h41, _) = f19(Fn0::F20(v2, std::rc::Rc::new(v0.clone()), v3), {
        let a43 = match v0 {
            Adt17::C0(_, _, h42, _) => {
                h42
            }
        };
        let a44 = 1u64;
        nat_add(a43, a44)?
    }, (v1, List::<Adt5>::nil()))?;
    Ok(h41)
}

pub fn f10(v0: Adt17, v1: u64, v2: Bytes) -> R<bool> {
    Ok(f21(v0.clone(), f22(v0, v1)?, 0u64, v2))
}

pub fn f11(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    f23(v0, v1, 0u64, v2.clone(), f24(v2.clone(), 1u64, {
        let a46 = {
            let a45 = v2;
            length_bytes(a45)
        };
        let a47 = 1u64;
        nat_sub(a46, a47)
    }))
}

pub fn f12(v0: Adt17, v1: Adt19, v2: Adt5, v3: List<Adt5>, v4: List<Adt5>, v5: Adt5) -> R<Result<Adt2, Adt15>> {
    let v6: u64 = match v1.clone() {
        Adt19::C0(h48, _, _) => {
            h48
        }
    };
    if f25(v0.clone(), v6, f3(Str::lit("coremodule")))? {
        let v7: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("coremodule")), f3(Str::lit("\\coredata")), f3(Str::lit("coredata")), f3(Str::lit("coresource")))?;
        f27(v0, {
            let (_, h49) = v7;
            h49
        }, v2, v3, v4, v5, Adt6::C0, Some({
            let (h50, _) = v7;
            h50
        }), f28(0u64), f28(0u64))
    } else {
        if f25(v0.clone(), v6, f3(Str::lit("semanticmodule")))? {
            let v8: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("semanticmodule")), f3(Str::lit("\\semanticdata")), f3(Str::lit("semanticdata")), f3(Str::lit("semanticsource")))?;
            f27(v0, {
                let (_, h51) = v8;
                h51
            }, v2, v3, v4, v5, Adt6::C0, f28(0u64), Some({
                let (h52, _) = v8;
                h52
            }), f28(0u64))
        } else {
            if f25(v0.clone(), v6, f3(Str::lit("toolingmodule")))? {
                let v9: (Adt14, Adt19) = f26(v0.clone(), v1, f3(Str::lit("toolingmodule")), f3(Str::lit("\\toolingdata")), f3(Str::lit("toolingdata")), f3(Str::lit("toolingsource")))?;
                f27(v0, {
                    let (_, h53) = v9;
                    h53
                }, v2, v3, v4, v5, Adt6::C0, f28(0u64), f28(0u64), Some({
                    let (h54, _) = v9;
                    h54
                }))
            } else {
                let v10: (Adt6, Adt19) = f29(v0.clone(), v1)?;
                f27(v0, {
                    let (_, h55) = v10;
                    h55
                }, v2, v3, v4, v5, {
                    let (h56, _) = v10;
                    h56
                }, f28(0u64), f28(0u64), f28(0u64))
            }
        }
    }
}

pub fn f13(v0: List<Adt5>) -> List<Adt5> {
    f30(Fn1::F31, List::<Adt5>::nil(), v0)
}

pub fn f14(v0: Adt17, v1: Adt19, v2: Adt5, v3: Adt4) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h57, _) => {
            h57
        }
    };
    Ok(Adt19::C0(match v1.clone() {
        Adt19::C0(h58, _, _) => {
            h58
        }
    }, {
        let (_, h59) = {
            let (h60, _) = f32(Fn2::F33(std::rc::Rc::new(v3), std::rc::Rc::new(v0), v4), {
                let a62 = v4;
                let a63 = match v2.clone() {
                    Adt5::C0(h61, _, _) => {
                        h61
                    }
                };
                nat_sub(a62, a63)
            }, (match v2 {
                Adt5::C0(h64, _, _) => {
                    h64
                }
            }, match v1.clone() {
                Adt19::C0(_, h65, _) => {
                    h65
                }
            }))?;
            h60
        };
        h59
    }, match v1 {
        Adt19::C0(_, _, h66) => {
            h66
        }
    }))
}

pub fn f15(v0: Bytes) -> Adt4 {
    f34(f7(f3(Str::lit("env.")), v0))
}

pub fn f16(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes) -> R<Adt19> {
    let m68 = match v1.clone() {
        Adt19::C0(_, _, h67) => {
            h67
        }
    };
    match m68 {
        None => {
            let v5: u64 = f22(v0.clone(), match v1.clone() {
                Adt19::C0(h69, _, _) => {
                    h69
                }
            })?;
            Ok(Adt19::C0(v5, match v1 {
                Adt19::C0(_, h70, _) => {
                    h70
                }
            }, Some(Adt15::C0(v2, v3, Some(f35(v0, v5)), None::<Bytes>))))
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f17(v0: Adt17, v1: Adt19, v2: bool) -> R<Adt20> {
    let v3: Adt19 = f23(v0.clone(), v1, 3u64, f3(Str::lit("{")), f3(Str::lit("brace-open")))?;
    let v4: u64 = match v3.clone() {
        Adt19::C0(h71, _, _) => {
            h71
        }
    };
    let v5: Adt21 = {
        let (h72, _) = f36(Fn3::F37(std::rc::Rc::new(v0.clone()), v2), {
            let a74 = match v0.clone() {
                Adt17::C0(_, _, h73, _) => {
                    h73
                }
            };
            let a75 = 1u64;
            nat_add(a74, a75)?
        }, Adt21::C0(v4, 0u64, List::<Bytes>::nil(), false, false, None::<Adt0>))?;
        h72
    };
    let v6: u64 = match v5.clone() {
        Adt21::C0(h76, _, _, _, _, _) => {
            h76
        }
    };
    let m78 = match v5.clone() {
        Adt21::C0(_, _, _, _, _, h77) => {
            h77
        }
    };
    match m78 {
        None => {
            Ok(Adt20::C0(std::rc::Rc::new(Adt5::C0(v4, v6, Bytes::lit(&[]))), std::rc::Rc::new(f4(v0, f40(v3, v6), f3(Str::lit("unclosed `{` argument")))?)))
        }
        Some(v7) => {
            Ok(Adt20::C0(std::rc::Rc::new(Adt5::C0(v4, v6, f38(match v5 {
                Adt21::C0(_, _, h79, _, _, _) => {
                    h79
                }
            })?)), std::rc::Rc::new(f39(v0, f40(v3, {
                let a80 = v6;
                let a81 = 1u64;
                nat_add(a80, a81)?
            }), v7, f34(f3(Str::lit("brace-close")))))))
        }
    }
}

pub fn f18(v0: Bytes) -> Adt4 {
    Adt4::C1(f7(f3(Str::lit("lexlean.core::")), v0))
}

pub fn f19(v0: Fn0, p86: u64, p87: (Adt19, List<Adt5>)) -> R<((Adt19, List<Adt5>), bool)> {
    let m85 = (p86, p87);
    {
        let mut m85 = m85;
        loop {
            let step = {
                let (v1, v2) = m85;
                let m82 = v1;
                if m82 == 0 {
                    Err::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m82 - 1;
                    let m84 = {
                        let c83: Fn0 = v0.clone();
                        c83.apply(v2.clone())?
                    };
                    match m84 {
                        None => {
                            Err::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<Adt5>)), R<((Adt19, List<Adt5>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m85 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes, v1: Adt17, v2: Bytes, v3: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    f41(v1, v0, v2, v3)
}

pub fn f21(v0: Adt17, v1: u64, v2: u64, v3: Bytes) -> bool {
    let m88 = f42(v0, v1);
    match m88 {
        None => {
            false
        }
        Some(v4) => {
            f43(v4, v2, v3)
        }
    }
}

pub fn f22(v0: Adt17, v1: u64) -> R<u64> {
    let (h89, _) = f44(Fn4::F45(std::rc::Rc::new(v0.clone())), match v0 {
        Adt17::C0(_, _, h90, _) => {
            h90
        }
    }, v1)?;
    Ok(h89)
}

pub fn f23(v0: Adt17, v1: Adt19, v2: u64, v3: Bytes, v4: Bytes) -> R<Adt19> {
    let v5: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h91, _, _) => {
            h91
        }
    })?;
    let m92 = f42(v0.clone(), v5);
    match m92 {
        None => {
            f4(v0, f40(v1, v5), f47(v3))
        }
        Some(v6) => {
            if f43(v6.clone(), v2, v3.clone()) {
                Ok(f39(v0, f40(v1, {
                    let a93 = v5;
                    let a94 = 1u64;
                    nat_add(a93, a94)?
                }), v6, f34(v4)))
            } else {
                f4(v0, f40(v1, v5), f46(v3, match v6 {
                    Adt0::C0(_, _, _, _, _, _, _, h95) => {
                        h95
                    }
                }))
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m99 = {
        let a96 = v0;
        let a97 = v1;
        let a98 = v2;
        slice_bytes(a96, a97, a98)
    };
    match m99 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f25(v0: Adt17, v1: u64, v2: Bytes) -> R<bool> {
    if f10(v0.clone(), v1, f3(Str::lit("\\begin")))? {
        let v3: u64 = f22(v0.clone(), {
            let a100 = f22(v0.clone(), v1)?;
            let a101 = 1u64;
            nat_add(a100, a101)?
        })?;
        if f21(v0.clone(), v3, 3u64, f3(Str::lit("{"))) {
            let m104 = f42(v0.clone(), {
                let a102 = v3;
                let a103 = 1u64;
                nat_add(a102, a103)?
            });
            match m104 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if f2(match v4 {
                        Adt0::C0(_, _, _, _, _, _, _, h105) => {
                            h105
                        }
                    }, v2) {
                        Ok(f21(v0, {
                            let a106 = v3;
                            let a107 = 2u64;
                            nat_add(a106, a107)?
                        }, 3u64, f3(Str::lit("}"))))
                    } else {
                        Ok(false)
                    }
                }
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f26(v0: Adt17, v1: Adt19, v2: Bytes, v3: Bytes, v4: Bytes, v5: Bytes) -> R<(Adt14, Adt19)> {
    let v6: Adt18 = f1(v0.clone(), v1)?;
    let v7: Adt19 = match v6.clone() {
        Adt18::C0(_, _, h108) => {
            (*h108).clone()
        }
    };
    let v8: bool = {
        let a110 = f10(v0.clone(), match v7.clone() {
            Adt19::C0(h109, _, _) => {
                h109
            }
        }, v3.clone())?;
        bool_not(a110)
    };
    let v9: Adt20 = if v8 {
        f48(v0.clone(), f40(v7.clone(), f22(v0.clone(), match v7 {
            Adt19::C0(h111, _, _) => {
                h111
            }
        })?))?
    } else {
        f17(v0.clone(), f11(v0.clone(), v7, v3)?, true)?
    };
    let v10: Adt19 = f49(v0.clone(), match v9.clone() {
        Adt20::C0(_, h112) => {
            (*h112).clone()
        }
    }, match v9.clone() {
        Adt20::C0(h113, _) => {
            (*h113).clone()
        }
    }, if v8 {
        v5
    } else {
        v4
    });
    let v11: Adt19 = f50(v0.clone(), f50(v0, v10, v2)?, f3(Str::lit("lexlean")))?;
    Ok((Adt14::C0(std::rc::Rc::new(match v9 {
        Adt20::C0(h114, _) => {
            (*h114).clone()
        }
    }), v8, match v6 {
        Adt18::C0(h115, _, _) => {
            h115
        }
    }), v11))
}

pub fn f27(v0: Adt17, v1: Adt19, v2: Adt5, v3: List<Adt5>, v4: List<Adt5>, v5: Adt5, v6: Adt6, v7: Option<Adt14>, v8: Option<Adt14>, v9: Option<Adt14>) -> R<Result<Adt2, Adt15>> {
    let v10: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h116, _, _) => {
            h116
        }
    })?;
    let v12: Adt19 = {
        let m117 = f42(v0.clone(), v10);
        match m117 {
            None => {
                v1
            }
            Some(v11) => {
                f4(v0, f40(v1, v10), f7(f7(f3(Str::lit("unexpected ")), f8(match v11 {
                    Adt0::C0(_, _, _, _, _, _, _, h118) => {
                        h118
                    }
                })), f3(Str::lit(" after \\end{lexlean}"))))?
            }
        }
    };
    let m120 = match v12.clone() {
        Adt19::C0(_, _, h119) => {
            h119
        }
    };
    match m120 {
        None => {
            Ok(Ok::<Adt2, Adt15>(Adt2::C0(f51(match v12 {
                Adt19::C0(_, h121, _) => {
                    h121
                }
            }), match v2.clone() {
                Adt5::C0(_, _, h122) => {
                    h122
                }
            }, std::rc::Rc::new(v2), v3, v4, match v5.clone() {
                Adt5::C0(h123, _, _) => {
                    h123
                }
            }, match v5 {
                Adt5::C0(_, h124, _) => {
                    h124
                }
            }, std::rc::Rc::new(v6), v7, v8, v9)))
        }
        Some(v13) => {
            Ok(Err::<Adt2, Adt15>(v13))
        }
    }
}

pub fn f28(_: u64) -> Option<Adt14> {
    None::<Adt14>
}

pub fn f29(v0: Adt17, v1: Adt19) -> R<(Adt6, Adt19)> {
    let v2: Adt22 = {
        let (h125, _) = f52(Fn5::F53(std::rc::Rc::new(v0.clone())), {
            let a127 = match v0 {
                Adt17::C0(_, _, h126, _) => {
                    h126
                }
            };
            let a128 = 1u64;
            nat_add(a127, a128)?
        }, Adt22::C0(std::rc::Rc::new(v1), List::<Adt23>::nil(), std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(Adt24::C0(f3(Str::lit("lexlean")), 0u64, List::<Adt6>::nil()))))))?;
        h125
    };
    let m130 = match v2.clone() {
        Adt22::C0(_, _, h129) => {
            (*h129).clone()
        }
    };
    match m130 {
        Adt25::C0(_) => {
            Ok((Adt6::C0, match v2 {
                Adt22::C0(h132, _, _) => {
                    (*h132).clone()
                }
            }))
        }
        Adt25::C1(_, _, _) => {
            Ok((Adt6::C0, match v2 {
                Adt22::C0(h133, _, _) => {
                    (*h133).clone()
                }
            }))
        }
        Adt25::C2(r134) => {
            let v3 = (*r134).clone();
            Ok((v3, match v2 {
                Adt22::C0(h131, _, _) => {
                    (*h131).clone()
                }
            }))
        }
    }
}

pub fn f30(v0: Fn1, p138: List<Adt5>, p139: List<Adt5>) -> List<Adt5> {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v1, v2) = m137;
                let m135 = v2;
                match m135.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c136: Fn1 = v0.clone();
                            c136.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f32(v0: Fn2, p144: u64, p145: (u64, List<Adt3>)) -> R<((u64, List<Adt3>), bool)> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v1, v2) = m143;
                let m140 = v1;
                if m140 == 0 {
                    Err::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m140 - 1;
                    let m142 = {
                        let c141: Fn2 = v0.clone();
                        c141.apply(v2.clone())?
                    };
                    match m142 {
                        None => {
                            Err::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Adt3>)), R<((u64, List<Adt3>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt4, v1: Adt17, v2: u64, v3: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
    f54(v1, v0, v2, v3)
}

pub fn f34(v0: Bytes) -> Adt4 {
    Adt4::C0(f3(Str::lit("lexlean.core")), v0)
}

pub fn f35(v0: Adt17, v1: u64) -> Adt16 {
    let v2: u64 = {
        let a147 = match v0.clone() {
            Adt17::C0(_, _, h146, _) => {
                h146
            }
        };
        let a148 = 1u64;
        nat_sub(a147, a148)
    };
    let m151 = f42(v0.clone(), {
        let a149 = v1;
        let a150 = v2;
        if nat_lt(a149, a150) {
            v1
        } else {
            v2
        }
    });
    match m151 {
        None => {
            f56(match v0 {
                Adt17::C0(h153, _, _, _) => {
                    h153
                }
            })
        }
        Some(v3) => {
            f55(match v0 {
                Adt17::C0(h152, _, _, _) => {
                    h152
                }
            }, v3)
        }
    }
}

pub fn f36(v0: Fn3, p158: u64, p159: Adt21) -> R<(Adt21, bool)> {
    let m157 = (p158, p159);
    {
        let mut m157 = m157;
        loop {
            let step = {
                let (v1, v2) = m157;
                let m154 = v1;
                if m154 == 0 {
                    Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m154 - 1;
                    let m156 = {
                        let c155: Fn3 = v0.clone();
                        c155.apply(v2.clone())?
                    };
                    match m156 {
                        None => {
                            Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt21), R<(Adt21, bool)>>((v3, v4))
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

pub fn f37(v0: Adt17, v1: bool, v2: Adt21) -> R<Option<Adt21>> {
    f57(v0, v1, v2)
}

pub fn f38(v0: List<Bytes>) -> R<Bytes> {
    f58(f59(v0)?)
}

pub fn f39(v0: Adt17, v1: Adt19, v2: Adt0, v3: Adt4) -> Adt19 {
    Adt19::C0(match v1.clone() {
        Adt19::C0(h160, _, _) => {
            h160
        }
    }, List::cons(f60(v0, v2, v3), match v1.clone() {
        Adt19::C0(_, h161, _) => {
            h161
        }
    }), match v1 {
        Adt19::C0(_, _, h162) => {
            h162
        }
    })
}

pub fn f40(v0: Adt19, v1: u64) -> Adt19 {
    Adt19::C0(v1, match v0.clone() {
        Adt19::C0(_, h163, _) => {
            h163
        }
    }, match v0 {
        Adt19::C0(_, _, h164) => {
            h164
        }
    })
}

pub fn f41(v0: Adt17, v1: Bytes, v2: Bytes, v3: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    let v4: Adt19 = {
        let (h165, _) = v3;
        h165
    };
    let m167 = match v4.clone() {
        Adt19::C0(_, _, h166) => {
            h166
        }
    };
    match m167 {
        None => {
            if f10(v0.clone(), match v4.clone() {
                Adt19::C0(h168, _, _) => {
                    h168
                }
            }, v1.clone())? {
                let v6: Adt20 = f5(v0.clone(), f11(v0.clone(), v4, v1)?)?;
                Ok(Some((f6(v0, match v6.clone() {
                    Adt20::C0(_, h169) => {
                        (*h169).clone()
                    }
                }, match v6.clone() {
                    Adt20::C0(h170, _) => {
                        (*h170).clone()
                    }
                }, v2)?, List::cons(match v6 {
                    Adt20::C0(h171, _) => {
                        (*h171).clone()
                    }
                }, {
                    let (_, h172) = v3;
                    h172
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt5>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt5>)>)
        }
    }
}

pub fn f42(v0: Adt17, v1: u64) -> Option<Adt0> {
    let a174 = match v0 {
        Adt17::C0(_, h173, _, _) => {
            h173
        }
    };
    let a175 = v1;
    index_list(a174, a175)
}

pub fn f43(v0: Adt0, v1: u64, v2: Bytes) -> bool {
    let a176 = f61(v0.clone());
    let a177 = v1;
    if nat_eq(a176, a177) {
        f2(match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h178) => {
                h178
            }
        }, v2)
    } else {
        false
    }
}

pub fn f44(v0: Fn4, p183: u64, p184: u64) -> R<(u64, bool)> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m179 = v1;
                if m179 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m179 - 1;
                    let m181 = {
                        let c180: Fn4 = v0.clone();
                        c180.apply(v2)?
                    };
                    match m181 {
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
                Ok(next) => { m182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Adt17, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f46(v0: Bytes, v1: Bytes) -> Bytes {
    f7(f7(f7(f3(Str::lit("expected ")), f8(v0)), f3(Str::lit(", found "))), f8(v1))
}

pub fn f47(v0: Bytes) -> Bytes {
    f7(f7(f3(Str::lit("expected ")), f8(v0)), f3(Str::lit(", found end of file")))
}

pub fn f48(v0: Adt17, v1: Adt19) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h185, _, _) => {
            h185
        }
    };
    let v3: (u64, List<Bytes>) = {
        let (h186, _) = f63(Fn6::F64(std::rc::Rc::new(v0.clone())), {
            let a188 = match v0.clone() {
                Adt17::C0(_, _, h187, _) => {
                    h187
                }
            };
            let a189 = 1u64;
            nat_add(a188, a189)?
        }, (v2, List::<Bytes>::nil()))?;
        h186
    };
    let v4: u64 = {
        let (h190, _) = v3;
        h190
    };
    Ok(Adt20::C0(std::rc::Rc::new(Adt5::C0(v2, v4, f38({
        let (_, h191) = v3;
        h191
    })?)), std::rc::Rc::new({
        let m192 = f42(v0.clone(), v4);
        match m192 {
            None => {
                f4(v0, f40(v1, v4), f3(Str::lit("unclosed environment")))?
            }
            Some(_) => {
                f40(v1, v4)
            }
        }
    })))
}

pub fn f49(v0: Adt17, v1: Adt19, v2: Adt5, v3: Bytes) -> Adt19 {
    let m194 = f42(v0.clone(), match v2.clone() {
        Adt5::C0(h193, _, _) => {
            h193
        }
    });
    match m194 {
        None => {
            v1
        }
        Some(v4) => {
            let v6: Adt0 = {
                let m198 = f42(v0.clone(), {
                    let a196 = match v2 {
                        Adt5::C0(_, h195, _) => {
                            h195
                        }
                    };
                    let a197 = 1u64;
                    nat_sub(a196, a197)
                });
                match m198 {
                    None => {
                        v4.clone()
                    }
                    Some(v5) => {
                        v5
                    }
                }
            };
            Adt19::C0(match v1.clone() {
                Adt19::C0(h199, _, _) => {
                    h199
                }
            }, List::cons(Adt3::C0(match v0 {
                Adt17::C0(h200, _, _, _) => {
                    h200
                }
            }, match v4.clone() {
                Adt0::C0(_, h201, _, _, _, _, _, _) => {
                    h201
                }
            }, match v6 {
                Adt0::C0(_, _, h202, _, _, _, _, _) => {
                    h202
                }
            }, match v4 {
                Adt0::C0(h203, _, _, _, _, _, _, _) => {
                    h203
                }
            }, std::rc::Rc::new(f18(v3))), match v1.clone() {
                Adt19::C0(_, h204, _) => {
                    h204
                }
            }), match v1 {
                Adt19::C0(_, _, h205) => {
                    h205
                }
            })
        }
    }
}

pub fn f50(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt19> {
    let v3: Adt19 = f11(v0.clone(), v1, f3(Str::lit("\\end")))?;
    let v4: Adt20 = f5(v0.clone(), v3)?;
    if f2({
        let m208 = match v4.clone() {
            Adt20::C0(h207, _) => {
                (*h207).clone()
            }
        };
        match m208 {
            Adt5::C0(_, _, h206) => {
                h206
            }
        }
    }, v2.clone()) {
        f14(v0, match v4.clone() {
            Adt20::C0(_, h209) => {
                (*h209).clone()
            }
        }, match v4 {
            Adt20::C0(h210, _) => {
                (*h210).clone()
            }
        }, f15(v2))
    } else {
        f4(v0, match v4.clone() {
            Adt20::C0(_, h211) => {
                (*h211).clone()
            }
        }, f7(f7(f7(f7(f3(Str::lit("expected `\\end{")), v2), f3(Str::lit("}`, found `\\end{"))), {
            let m214 = match v4 {
                Adt20::C0(h213, _) => {
                    (*h213).clone()
                }
            };
            match m214 {
                Adt5::C0(_, _, h212) => {
                    h212
                }
            }
        }), f3(Str::lit("}`"))))
    }
}

pub fn f51(v0: List<Adt3>) -> List<Adt3> {
    f65(Fn7::F66, List::<Adt3>::nil(), v0)
}

pub fn f52(v0: Fn5, p219: u64, p220: Adt22) -> R<(Adt22, bool)> {
    let m218 = (p219, p220);
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, v2) = m218;
                let m215 = v1;
                if m215 == 0 {
                    Err::<(u64, Adt22), R<(Adt22, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m215 - 1;
                    let m217 = {
                        let c216: Fn5 = v0.clone();
                        c216.apply(v2.clone())?
                    };
                    match m217 {
                        None => {
                            Err::<(u64, Adt22), R<(Adt22, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt22), R<(Adt22, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Adt17, v1: Adt22) -> R<Option<Adt22>> {
    f67(v0, v1)
}

pub fn f54(v0: Adt17, v1: Adt4, v2: u64, v3: (u64, List<Adt3>)) -> R<Option<(u64, List<Adt3>)>> {
    let a222 = {
        let (h221, _) = v3.clone();
        h221
    };
    let a223 = v2;
    if nat_lt(a222, a223) {
        let m225 = f42(v0.clone(), {
            let (h224, _) = v3.clone();
            h224
        });
        match m225 {
            None => {
                Ok(None::<(u64, List<Adt3>)>)
            }
            Some(v4) => {
                let a226 = f61(v4.clone());
                let a227 = 6u64;
                if nat_eq(a226, a227) {
                    Ok(Some(({
                        let a229 = {
                            let (h228, _) = v3;
                            h228
                        };
                        let a230 = 1u64;
                        nat_add(a229, a230)?
                    }, {
                        let (_, h231) = v3;
                        h231
                    })))
                } else {
                    Ok(Some(({
                        let a233 = {
                            let (h232, _) = v3;
                            h232
                        };
                        let a234 = 1u64;
                        nat_add(a233, a234)?
                    }, List::cons(f60(v0, v4, v1), {
                        let (_, h235) = v3;
                        h235
                    }))))
                }
            }
        }
    } else {
        Ok(None::<(u64, List<Adt3>)>)
    }
}

pub fn f55(v0: Bytes, v1: Adt0) -> Adt16 {
    Adt16::C0(v0, match v1.clone() {
        Adt0::C0(_, h236, _, _, _, _, _, _) => {
            h236
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h237, _, _, _, _, _) => {
            h237
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h238, _, _, _, _) => {
            h238
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h239, _, _, _) => {
            h239
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h240, _, _) => {
            h240
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h241, _) => {
            h241
        }
    })
}

pub fn f56(v0: Bytes) -> Adt16 {
    Adt16::C0(v0, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64)
}

pub fn f57(v0: Adt17, v1: bool, v2: Adt21) -> R<Option<Adt21>> {
    let m243 = match v2.clone() {
        Adt21::C0(_, _, _, _, _, h242) => {
            h242
        }
    };
    match m243 {
        None => {
            let m245 = f42(v0, match v2.clone() {
                Adt21::C0(h244, _, _, _, _, _) => {
                    h244
                }
            });
            match m245 {
                None => {
                    Ok(None::<Adt21>)
                }
                Some(v4) => {
                    let v5: bool = match v2.clone() {
                        Adt21::C0(_, _, _, h246, _, _) => {
                            h246
                        }
                    };
                    let v6: u64 = match v2.clone() {
                        Adt21::C0(_, h247, _, _, _, _) => {
                            h247
                        }
                    };
                    let v7: List<Bytes> = match v2.clone() {
                        Adt21::C0(_, _, h248, _, _, _) => {
                            h248
                        }
                    };
                    if if v5 {
                        false
                    } else {
                        f43(v4.clone(), 3u64, f3(Str::lit("{")))
                    } {
                        f68(v1, v2, v4, {
                            let a249 = v6;
                            let a250 = 1u64;
                            nat_add(a249, a250)?
                        }, List::cons(f3(Str::lit("{")), v7))
                    } else {
                        if if v5 {
                            false
                        } else {
                            f43(v4.clone(), 3u64, f3(Str::lit("}")))
                        } {
                            let a251 = v6;
                            let a252 = 0u64;
                            if nat_eq(a251, a252) {
                                Ok(Some(Adt21::C0(match v2.clone() {
                                    Adt21::C0(h253, _, _, _, _, _) => {
                                        h253
                                    }
                                }, v6, v7, v5, match v2 {
                                    Adt21::C0(_, _, _, _, h254, _) => {
                                        h254
                                    }
                                }, Some(v4))))
                            } else {
                                f68(v1, v2, v4, {
                                    let a255 = v6;
                                    let a256 = 1u64;
                                    nat_sub(a255, a256)
                                }, List::cons(f3(Str::lit("}")), v7))
                            }
                        } else {
                            if if v5 {
                                false
                            } else {
                                let a257 = f61(v4.clone());
                                let a258 = 6u64;
                                nat_eq(a257, a258)
                            } {
                                f68(v1, v2, v4, v6, v7)
                            } else {
                                f68(v1, v2, v4.clone(), v6, List::cons(match v4 {
                                    Adt0::C0(_, _, _, _, _, _, _, h259) => {
                                        h259
                                    }
                                }, v7))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt21>)
        }
    }
}

pub fn f58(v0: List<Bytes>) -> R<Bytes> {
    let m262 = {
        let (h260, _) = f69(Fn8::F70, {
            let a261 = v0.clone();
            length_list(a261)
        }, v0)?;
        h260
    };
    match m262.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, _)) => {
            Ok(v1)
        }
    }
}

pub fn f59(v0: List<Bytes>) -> R<List<Bytes>> {
    f71(Fn9::F72, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Adt17, v1: Adt0, v2: Adt4) -> Adt3 {
    Adt3::C0(match v0 {
        Adt17::C0(h263, _, _, _) => {
            h263
        }
    }, match v1.clone() {
        Adt0::C0(_, h264, _, _, _, _, _, _) => {
            h264
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h265, _, _, _, _, _) => {
            h265
        }
    }, match v1 {
        Adt0::C0(h266, _, _, _, _, _, _, _) => {
            h266
        }
    }, std::rc::Rc::new(v2))
}

pub fn f61(v0: Adt0) -> u64 {
    let m268 = match v0 {
        Adt0::C0(h267, _, _, _, _, _, _, _) => {
            h267
        }
    };
    match m268 {
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
    }
}

pub fn f62(v0: Adt17, v1: u64) -> R<Option<u64>> {
    let m269 = f42(v0, v1);
    match m269 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a270 = f61(v2);
            let a271 = 6u64;
            if nat_eq(a270, a271) {
                Ok(Some({
                    let a272 = v1;
                    let a273 = 1u64;
                    nat_add(a272, a273)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f63(v0: Fn6, p278: u64, p279: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m277 = (p278, p279);
    {
        let mut m277 = m277;
        loop {
            let step = {
                let (v1, v2) = m277;
                let m274 = v1;
                if m274 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m274 - 1;
                    let m276 = {
                        let c275: Fn6 = v0.clone();
                        c275.apply(v2.clone())?
                    };
                    match m276 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt17, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f73(v0, v1)
}

pub fn f65(v0: Fn7, p283: List<Adt3>, p284: List<Adt3>) -> List<Adt3> {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v2;
                match m280.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c281: Fn7 = v0.clone();
                            c281.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f67(v0: Adt17, v1: Adt22) -> R<Option<Adt22>> {
    let v2: Adt19 = match v1.clone() {
        Adt22::C0(h285, _, _) => {
            (*h285).clone()
        }
    };
    let m287 = match v2.clone() {
        Adt19::C0(_, _, h286) => {
            h286
        }
    };
    match m287 {
        None => {
            let m289 = match v1.clone() {
                Adt22::C0(_, _, h288) => {
                    (*h288).clone()
                }
            };
            match m289 {
                Adt25::C0(r292) => {
                    let v4 = (*r292).clone();
                    Ok(Some(f74(v0, v2, match v1 {
                        Adt22::C0(_, h290, _) => {
                            h290
                        }
                    }, v4)?))
                }
                Adt25::C1(r293, r294, v7) => {
                    let v5 = (*r293).clone();
                    let v6 = (*r294).clone();
                    Ok(Some(f75(v0, v2, match v1 {
                        Adt22::C0(_, h291, _) => {
                            h291
                        }
                    }, v5, v6, v7)?))
                }
                Adt25::C2(_) => {
                    Ok(None::<Adt22>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt22>)
        }
    }
}

pub fn f68(v0: bool, v1: Adt21, v2: Adt0, v3: u64, v4: List<Bytes>) -> R<Option<Adt21>> {
    let v5: Adt32 = if v0 {
        f76(match v2 {
            Adt0::C0(_, _, _, _, _, _, _, h295) => {
                h295
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, h296, _, _) => {
                h296
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, _, h297, _) => {
                h297
            }
        })?
    } else {
        Adt32::C0(0u64, match v1.clone() {
            Adt21::C0(_, _, _, h298, _, _) => {
                h298
            }
        }, match v1.clone() {
            Adt21::C0(_, _, _, _, h299, _) => {
                h299
            }
        })
    };
    Ok(Some(Adt21::C0({
        let a301 = match v1 {
            Adt21::C0(h300, _, _, _, _, _) => {
                h300
            }
        };
        let a302 = 1u64;
        nat_add(a301, a302)?
    }, v3, v4, match v5.clone() {
        Adt32::C0(_, h303, _) => {
            h303
        }
    }, match v5 {
        Adt32::C0(_, _, h304) => {
            h304
        }
    }, None::<Adt0>)))
}

pub fn f69(v0: Fn8, p309: u64, p310: List<Bytes>) -> R<(List<Bytes>, bool)> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m305 = v1;
                if m305 == 0 {
                    Err::<(u64, List<Bytes>), R<(List<Bytes>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m305 - 1;
                    let m307 = {
                        let c306: Fn8 = v0.clone();
                        c306.apply(v2.clone())?
                    };
                    match m307 {
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
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    f77(v0)
}

pub fn f71(v0: Fn9, p314: List<Bytes>, p315: List<Bytes>) -> R<List<Bytes>> {
    let m313 = (p314, p315);
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v1, v2) = m313;
                let m311 = v2;
                match m311.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c312: Fn9 = v0.clone();
                            c312.apply(v1, v3)?
                        }, v4))
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

pub fn f72(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f73(v0: Adt17, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let m317 = f42(v0, {
        let (h316, _) = v1.clone();
        h316
    });
    match m317 {
        None => {
            Ok(None::<(u64, List<Bytes>)>)
        }
        Some(v2) => {
            if f43(v2.clone(), 0u64, f3(Str::lit("\\end"))) {
                Ok(None::<(u64, List<Bytes>)>)
            } else {
                Ok(Some(({
                    let a319 = {
                        let (h318, _) = v1;
                        h318
                    };
                    let a320 = 1u64;
                    nat_add(a319, a320)?
                }, List::cons(match v2 {
                    Adt0::C0(_, _, _, _, _, _, _, h321) => {
                        h321
                    }
                }, {
                    let (_, h322) = v1;
                    h322
                }))))
            }
        }
    }
}

pub fn f74(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24) -> R<Adt22> {
    let v4: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h323, _, _) => {
            h323
        }
    })?;
    let v5: Bytes = match v3.clone() {
        Adt24::C0(h324, _, _) => {
            h324
        }
    };
    let m325 = f42(v0.clone(), v4);
    match m325 {
        None => {
            Ok(Adt22::C0(std::rc::Rc::new(f4(v0, f40(v1, v4), f7(f7(f3(Str::lit("unclosed ")), f8(v5)), f3(Str::lit(" environment"))))?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
        }
        Some(v6) => {
            if f43(v6.clone(), 0u64, f3(Str::lit("\\end"))) {
                f78(v0, v1, v2, v3)
            } else {
                if f43(v6.clone(), 0u64, f3(Str::lit("\\begin"))) {
                    let v7: Adt18 = f1(v0.clone(), v1)?;
                    let v8: Bytes = match v7.clone() {
                        Adt18::C0(_, h326, _) => {
                            h326
                        }
                    };
                    if f2(v8.clone(), f3(Str::lit("section"))) {
                        f79(v0, match v7.clone() {
                            Adt18::C0(_, _, h327) => {
                                (*h327).clone()
                            }
                        }, v2, v3, match v7 {
                            Adt18::C0(h328, _, _) => {
                                h328
                            }
                        })
                    } else {
                        let m329 = f80(v8.clone());
                        match m329 {
                            None => {
                                Ok(Adt22::C0(std::rc::Rc::new(f16(v0, match v7 {
                                    Adt18::C0(_, _, h332) => {
                                        (*h332).clone()
                                    }
                                }, f3(Str::lit("LLL1004")), f7(f8(v8), f3(Str::lit(" is not a block environment"))))?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
                            }
                            Some(v9) => {
                                f81(v0, match v7.clone() {
                                    Adt18::C0(_, _, h330) => {
                                        (*h330).clone()
                                    }
                                }, v2, v3, match v7 {
                                    Adt18::C0(h331, _, _) => {
                                        h331
                                    }
                                }, v8, v9)
                            }
                        }
                    }
                } else {
                    Ok(Adt22::C0(std::rc::Rc::new(f4(v0, f40(v1, v4), f7(f7(f7(f7(f3(Str::lit("expected a block or `\\end{")), v5), f3(Str::lit("}`, found `"))), match v6 {
                        Adt0::C0(_, _, _, _, _, _, _, h333) => {
                            h333
                        }
                    }), f3(Str::lit("`"))))?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
                }
            }
        }
    }
}

pub fn f75(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>) -> R<Adt22> {
    let v6: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h334, _, _) => {
            h334
        }
    })?;
    let m335 = f42(v0.clone(), v6);
    match m335 {
        None => {
            Ok(Adt22::C0(std::rc::Rc::new(f4(v0, f40(v1, v6), f7(f7(f3(Str::lit("unclosed ")), f8(match v3.clone() {
                Adt26::C0(_, h336, _, _) => {
                    h336
                }
            })), f3(Str::lit(" environment"))))?), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(v3), std::rc::Rc::new(v4), v5))))
        }
        Some(v7) => {
            if f43(v7.clone(), 0u64, f3(Str::lit("\\end"))) {
                let v8: Adt19 = f50(v0.clone(), v1, match v3.clone() {
                    Adt26::C0(_, h337, _, _) => {
                        h337
                    }
                })?;
                f82(v0, v8, v2, v4, v5, Adt12::C2(std::rc::Rc::new(f83(match v3.clone() {
                    Adt26::C0(_, _, _, h338) => {
                        h338
                    }
                })), match v3 {
                    Adt26::C0(h339, _, _, _) => {
                        h339
                    }
                }))
            } else {
                if f43(v7, 0u64, f3(Str::lit("\\begin"))) {
                    f84(v0, v1, v2, v3, v4, v5)
                } else {
                    let v9: Adt33 = f85(v0, v1)?;
                    Ok(Adt22::C0(std::rc::Rc::new(match v9.clone() {
                        Adt33::C0(_, h340) => {
                            (*h340).clone()
                        }
                    }), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(f86(v3, Adt12::C3(std::rc::Rc::new(match v9 {
                        Adt33::C0(h341, _) => {
                            (*h341).clone()
                        }
                    })))), std::rc::Rc::new(v4), v5))))
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: bool, v2: bool) -> R<Adt32> {
    let (h342, _) = f87(Fn10::F88(v0.clone()), {
        let a343 = v0;
        length_bytes(a343)
    }, Adt32::C0(0u64, v1, v2))?;
    Ok(h342)
}

pub fn f77(v0: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m344 = v0.clone();
    match m344.uncons() {
        None => {
            Ok(None::<List<Bytes>>)
        }
        Some((_, v2)) => {
            let m345 = v2;
            match m345.uncons() {
                None => {
                    Ok(None::<List<Bytes>>)
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f89(Fn11::F90, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m347 = {
                        let (h346, _) = v5;
                        h346
                    };
                    match m347 {
                        None => {
                            Ok(Some(f59({
                                let (_, h349) = v5;
                                h349
                            })?))
                        }
                        Some(v6) => {
                            Ok(Some(f59(List::cons(v6, {
                                let (_, h348) = v5;
                                h348
                            }))?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f78(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24) -> R<Adt22> {
    let v4: Adt19 = f50(v0, v1, match v3.clone() {
        Adt24::C0(h350, _, _) => {
            h350
        }
    })?;
    let v5: Adt6 = f91(match v3 {
        Adt24::C0(_, _, h351) => {
            h351
        }
    });
    let m352 = v2.clone();
    match m352.uncons() {
        None => {
            Ok(Adt22::C0(std::rc::Rc::new(v4), v2, std::rc::Rc::new(Adt25::C2(std::rc::Rc::new(v5)))))
        }
        Some((v6, v7)) => {
            Ok(Adt22::C0(std::rc::Rc::new(v4), v7, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(f92(match v6.clone() {
                Adt23::C0(h353, _, _, _, _, _) => {
                    (*h353).clone()
                }
            }, Adt6::C2(std::rc::Rc::new(match v6.clone() {
                Adt23::C0(_, h354, _, _, _, _) => {
                    (*h354).clone()
                }
            }), match v6.clone() {
                Adt23::C0(_, _, h355, _, _, _) => {
                    h355
                }
            }, match v6.clone() {
                Adt23::C0(_, _, _, h356, _, _) => {
                    h356
                }
            }, match v6.clone() {
                Adt23::C0(_, _, _, _, h357, _) => {
                    h357
                }
            }, std::rc::Rc::new(v5), match v6 {
                Adt23::C0(_, _, _, _, _, h358) => {
                    h358
                }
            })))))))
        }
    }
}

pub fn f79(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24, v4: u64) -> R<Adt22> {
    let v5: u64 = {
        let a360 = match v3.clone() {
            Adt24::C0(_, h359, _) => {
                h359
            }
        };
        let a361 = 1u64;
        nat_add(a360, a361)?
    };
    let a363 = match v0.clone() {
        Adt17::C0(_, _, _, h362) => {
            h362
        }
    };
    let a364 = v5;
    if nat_lt(a363, a364) {
        Ok(Adt22::C0(std::rc::Rc::new(f16(v0.clone(), v1, f3(Str::lit("LLS8002")), f93(v0, f3(Str::lit("section nesting")), v5)?)?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
    } else {
        let v6: Adt20 = f94(v0.clone(), v1, f3(Str::lit("env.section")))?;
        let v7: Adt20 = f5(v0.clone(), f11(v0.clone(), match v6.clone() {
            Adt20::C0(_, h365) => {
                (*h365).clone()
            }
        }, f3(Str::lit("\\heading")))?)?;
        let v8: Option<Adt20> = if f10(v0.clone(), {
            let m368 = match v7.clone() {
                Adt20::C0(_, h367) => {
                    (*h367).clone()
                }
            };
            match m368 {
                Adt19::C0(h366, _, _) => {
                    h366
                }
            }
        }, f3(Str::lit("\\parameters")))? {
            Some(f5(v0.clone(), f11(v0, match v7.clone() {
                Adt20::C0(_, h369) => {
                    (*h369).clone()
                }
            }, f3(Str::lit("\\parameters")))?)?)
        } else {
            None::<Adt20>
        };
        let v10: Adt19 = {
            let m370 = v8.clone();
            match m370 {
                None => {
                    match v7.clone() {
                        Adt20::C0(_, h372) => {
                            (*h372).clone()
                        }
                    }
                }
                Some(v9) => {
                    match v9 {
                        Adt20::C0(_, h371) => {
                            (*h371).clone()
                        }
                    }
                }
            }
        };
        Ok(Adt22::C0(std::rc::Rc::new(v10), List::cons(Adt23::C0(std::rc::Rc::new(v3), std::rc::Rc::new(match v6 {
            Adt20::C0(h373, _) => {
                (*h373).clone()
            }
        }), {
            let m376 = match v7.clone() {
                Adt20::C0(h375, _) => {
                    (*h375).clone()
                }
            };
            match m376 {
                Adt5::C0(h374, _, _) => {
                    h374
                }
            }
        }, {
            let m379 = match v7 {
                Adt20::C0(h378, _) => {
                    (*h378).clone()
                }
            };
            match m379 {
                Adt5::C0(_, h377, _) => {
                    h377
                }
            }
        }, {
            let m380 = v8;
            match m380 {
                None => {
                    None::<(u64, u64)>
                }
                Some(v11) => {
                    Some(({
                        let m383 = match v11.clone() {
                            Adt20::C0(h382, _) => {
                                (*h382).clone()
                            }
                        };
                        match m383 {
                            Adt5::C0(h381, _, _) => {
                                h381
                            }
                        }
                    }, {
                        let m386 = match v11 {
                            Adt20::C0(h385, _) => {
                                (*h385).clone()
                            }
                        };
                        match m386 {
                            Adt5::C0(_, h384, _) => {
                                h384
                            }
                        }
                    }))
                }
            }
        }, v4), v2), std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(Adt24::C0(f3(Str::lit("section")), v5, List::<Adt6>::nil()))))))
    }
}

pub fn f80(v0: Bytes) -> Option<Adt8> {
    if f2(v0.clone(), f3(Str::lit("typedefinition"))) {
        Some(Adt8::C0)
    } else {
        if f2(v0.clone(), f3(Str::lit("termdefinition"))) {
            Some(Adt8::C1)
        } else {
            if f2(v0.clone(), f3(Str::lit("predicatedefinition"))) {
                Some(Adt8::C2)
            } else {
                if f2(v0.clone(), f3(Str::lit("theorem"))) {
                    Some(Adt8::C3)
                } else {
                    if f2(v0.clone(), f3(Str::lit("lemma"))) {
                        Some(Adt8::C4)
                    } else {
                        if f2(v0, f3(Str::lit("corollary"))) {
                            Some(Adt8::C5)
                        } else {
                            None::<Adt8>
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt24, v4: u64, v5: Bytes, v6: Adt8) -> R<Adt22> {
    let v7: Bytes = f7(f3(Str::lit("env.")), v5.clone());
    let v8: Adt20 = f94(v0.clone(), v1, v7.clone())?;
    let v9: bool = f95(v6.clone());
    let v11: Option<Adt20> = if v9 {
        None::<Adt20>
    } else {
        let v10: Adt20 = f5(v0.clone(), match v8.clone() {
            Adt20::C0(_, h387) => {
                (*h387).clone()
            }
        })?;
        Some(Adt20::C0(std::rc::Rc::new(match v10.clone() {
            Adt20::C0(h388, _) => {
                (*h388).clone()
            }
        }), std::rc::Rc::new(f6(v0.clone(), match v10.clone() {
            Adt20::C0(_, h389) => {
                (*h389).clone()
            }
        }, match v10 {
            Adt20::C0(h390, _) => {
                (*h390).clone()
            }
        }, v7)?)))
    };
    let v13: Adt34 = f96(v0.clone(), {
        let m391 = v11.clone();
        match m391 {
            None => {
                match v8.clone() {
                    Adt20::C0(_, h393) => {
                        (*h393).clone()
                    }
                }
            }
            Some(v12) => {
                match v12 {
                    Adt20::C0(_, h392) => {
                        (*h392).clone()
                    }
                }
            }
        }
    })?;
    let v14: Adt33 = f85(v0.clone(), match v13.clone() {
        Adt34::C0(_, h394) => {
            (*h394).clone()
        }
    })?;
    let v15: Adt19 = match v14.clone() {
        Adt33::C0(_, h395) => {
            (*h395).clone()
        }
    };
    let v17: Adt27 = Adt27::C0(std::rc::Rc::new(v3.clone()), v5, v6, std::rc::Rc::new(match v8 {
        Adt20::C0(h396, _) => {
            (*h396).clone()
        }
    }), {
        let m397 = v11;
        match m397 {
            None => {
                None::<Adt5>
            }
            Some(v16) => {
                Some(match v16 {
                    Adt20::C0(h398, _) => {
                        (*h398).clone()
                    }
                })
            }
        }
    }, std::rc::Rc::new(match v13 {
        Adt34::C0(h399, _) => {
            (*h399).clone()
        }
    }), std::rc::Rc::new(match v14 {
        Adt33::C0(h400, _) => {
            (*h400).clone()
        }
    }), v4);
    let v18: bool = f25(v0.clone(), match v15.clone() {
        Adt19::C0(h401, _, _) => {
            h401
        }
    }, f3(Str::lit("proof")))?;
    if v9 {
        if v18 {
            let v19: Adt18 = f1(v0.clone(), v15)?;
            f97(v0, match v19.clone() {
                Adt18::C0(_, _, h402) => {
                    (*h402).clone()
                }
            }, v2, match v19 {
                Adt18::C0(h403, _, _) => {
                    h403
                }
            }, f3(Str::lit("proof")), 1u64, v17, List::<Adt28>::nil())
        } else {
            Ok(Adt22::C0(std::rc::Rc::new(f16(v0, v15, f3(Str::lit("LLF5005")), f3(Str::lit("a theorem-like declaration without a proof is not valid")))?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
        }
    } else {
        if v18 {
            Ok(Adt22::C0(std::rc::Rc::new(f4(v0, v15, f3(Str::lit("no definitional component accepts a proof environment")))?), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(v3)))))
        } else {
            f98(v0, v15, v2, v17, None::<Adt12>)
        }
    }
}

pub fn f82(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt12) -> R<Adt22> {
    let m404 = v4;
    match m404.uncons() {
        None => {
            f98(v0, v1, v2, v3, Some(v5))
        }
        Some((v6, v7)) => {
            let m405 = v6;
            match m405 {
                Adt28::C0(r418, r419, r420, v11) => {
                    let v8 = (*r418).clone();
                    let v9 = (*r419).clone();
                    let v10 = (*r420).clone();
                    let v12: Adt19 = f50(v0, v1, f3(Str::lit("have")))?;
                    Ok(Adt22::C0(std::rc::Rc::new(v12), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(f86(v8, Adt12::C4(std::rc::Rc::new(v9), std::rc::Rc::new(v10), std::rc::Rc::new(v5), v11))), std::rc::Rc::new(v3), v7))))
                }
                Adt28::C1(r421, v14) => {
                    let v13 = (*r421).clone();
                    f99(v0, v1, v2, v3, v7, Adt29::C0(std::rc::Rc::new(match v13.clone() {
                        Adt29::C0(h406, _, _, _, _, _) => {
                            (*h406).clone()
                        }
                    }), match v13.clone() {
                        Adt29::C0(_, h407, _, _, _, _) => {
                            h407
                        }
                    }, match v13.clone() {
                        Adt29::C0(_, _, h408, _, _, _) => {
                            h408
                        }
                    }, match v13.clone() {
                        Adt29::C0(_, _, _, h409, _, _) => {
                            h409
                        }
                    }, List::cons((v14, v5), match v13.clone() {
                        Adt29::C0(_, _, _, _, h410, _) => {
                            h410
                        }
                    }), match v13 {
                        Adt29::C0(_, _, _, _, _, h411) => {
                            h411
                        }
                    }))
                }
                Adt28::C2(r422, r423, v17, r424) => {
                    let v15 = (*r422).clone();
                    let v16 = (*r423).clone();
                    let v18 = (*r424).clone();
                    f100(v0, v1, v2, v3, v7, Adt30::C0(std::rc::Rc::new(match v15.clone() {
                        Adt30::C0(h412, _, _, _, _, _) => {
                            (*h412).clone()
                        }
                    }), match v15.clone() {
                        Adt30::C0(_, h413, _, _, _, _) => {
                            h413
                        }
                    }, match v15.clone() {
                        Adt30::C0(_, _, h414, _, _, _) => {
                            h414
                        }
                    }, std::rc::Rc::new(match v15.clone() {
                        Adt30::C0(_, _, _, h415, _, _) => {
                            (*h415).clone()
                        }
                    }), List::cons(Adt31::C0(std::rc::Rc::new(v16), v17, std::rc::Rc::new(v18), std::rc::Rc::new(v5)), match v15.clone() {
                        Adt30::C0(_, _, _, _, h416, _) => {
                            h416
                        }
                    }), match v15 {
                        Adt30::C0(_, _, _, _, _, h417) => {
                            h417
                        }
                    }))
                }
            }
        }
    }
}

pub fn f83(v0: List<Adt12>) -> Adt12 {
    f101(Fn12::F102, Adt12::C0, v0)
}

pub fn f84(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>) -> R<Adt22> {
    let v6: Adt18 = f1(v0.clone(), v1)?;
    let v7: Bytes = match v6.clone() {
        Adt18::C0(_, h425, _) => {
            h425
        }
    };
    let v8: u64 = match v6.clone() {
        Adt18::C0(h426, _, _) => {
            h426
        }
    };
    let v9: Adt19 = match v6 {
        Adt18::C0(_, _, h427) => {
            (*h427).clone()
        }
    };
    if f2(v7.clone(), f3(Str::lit("have"))) {
        f103(v0, v9, v2, v3, v4, v5, v8)
    } else {
        if f2(v7.clone(), f3(Str::lit("rewrite"))) {
            Ok(f104(v9.clone(), v2, v3, v4, v5, f105(v0, v9, v8)?))
        } else {
            if f2(v7.clone(), f3(Str::lit("simplify"))) {
                Ok(f104(v9.clone(), v2, v3, v4, v5, f106(v0, v9, v8)?))
            } else {
                if f2(v7.clone(), f3(Str::lit("apply"))) {
                    let v10: Adt20 = f5(v0.clone(), v9)?;
                    f99(v0, match v10.clone() {
                        Adt20::C0(_, h428) => {
                            (*h428).clone()
                        }
                    }, v2, v4, v5, Adt29::C0(std::rc::Rc::new(v3), f3(Str::lit("premise")), f3(Str::lit("apply")), Some(match v10 {
                        Adt20::C0(h429, _) => {
                            (*h429).clone()
                        }
                    }), List::<(u64, Adt12)>::nil(), v8))
                } else {
                    if f2(v7.clone(), f3(Str::lit("constructor"))) {
                        f99(v0, v9, v2, v4, v5, Adt29::C0(std::rc::Rc::new(v3), f3(Str::lit("branch")), f3(Str::lit("constructor")), None::<Adt5>, List::<(u64, Adt12)>::nil(), v8))
                    } else {
                        if if f2(v7.clone(), f3(Str::lit("cases"))) {
                            true
                        } else {
                            f2(v7.clone(), f3(Str::lit("induction")))
                        } {
                            let v11: Adt20 = f5(v0.clone(), v9)?;
                            f100(v0, match v11.clone() {
                                Adt20::C0(_, h430) => {
                                    (*h430).clone()
                                }
                            }, v2, v4, v5, Adt30::C0(std::rc::Rc::new(v3), f2(v7.clone(), f3(Str::lit("induction"))), v7, std::rc::Rc::new(match v11 {
                                Adt20::C0(h431, _) => {
                                    (*h431).clone()
                                }
                            }), List::<Adt31>::nil(), v8))
                        } else {
                            if f2(v7.clone(), f3(Str::lit("calculate"))) {
                                Ok(f104(v9.clone(), v2, v3, v4, v5, f107(v0, v9, v8)?))
                            } else {
                                Ok(Adt22::C0(std::rc::Rc::new(f16(v0, v9, f3(Str::lit("LLL1004")), f7(f8(v7), f3(Str::lit(" is not a proof environment"))))?), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(v3), std::rc::Rc::new(v4), v5))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f85(v0: Adt17, v1: Adt19) -> R<Adt33> {
    let v2: u64 = f22(v0.clone(), match v1.clone() {
        Adt19::C0(h432, _, _) => {
            h432
        }
    })?;
    let v3: Adt35 = {
        let (h433, _) = f108(Fn13::F109(std::rc::Rc::new(v0.clone())), {
            let a435 = match v0.clone() {
                Adt17::C0(_, _, h434, _) => {
                    h434
                }
            };
            let a436 = 1u64;
            nat_add(a435, a436)?
        }, Adt35::C0(v2, 0u64, 0u64, 0u64))?;
        h433
    };
    let v4: u64 = match v3.clone() {
        Adt35::C0(h437, _, _, _) => {
            h437
        }
    };
    let a439 = match v3 {
        Adt35::C0(_, _, _, h438) => {
            h438
        }
    };
    let a440 = 1u64;
    if nat_eq(a439, a440) {
        Ok(Adt33::C0(std::rc::Rc::new(Adt11::C0(v2, v4)), std::rc::Rc::new(f40(v1, {
            let a441 = v4;
            let a442 = 1u64;
            nat_add(a441, a442)?
        }))))
    } else {
        Ok(Adt33::C0(std::rc::Rc::new(Adt11::C0(v2, v4)), std::rc::Rc::new(f16(v0, f40(v1, v4), f3(Str::lit("LLP2001")), f3(Str::lit("a sentence must end with a period")))?)))
    }
}

pub fn f86(v0: Adt26, v1: Adt12) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h443, _, _, _) => {
            h443
        }
    }, match v0.clone() {
        Adt26::C0(_, h444, _, _) => {
            h444
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h445, _) => {
            h445
        }
    }, List::cons(v1, match v0 {
        Adt26::C0(_, _, _, h446) => {
            h446
        }
    }))
}

pub fn f87(v0: Fn10, p451: u64, p452: Adt32) -> R<(Adt32, bool)> {
    let m450 = (p451, p452);
    {
        let mut m450 = m450;
        loop {
            let step = {
                let (v1, v2) = m450;
                let m447 = v1;
                if m447 == 0 {
                    Err::<(u64, Adt32), R<(Adt32, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m447 - 1;
                    let m449 = {
                        let c448: Fn10 = v0.clone();
                        c448.apply(v2.clone())?
                    };
                    match m449 {
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
                Ok(next) => { m450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: Adt32) -> R<Option<Adt32>> {
    f110(v0, v1)
}

pub fn f89(v0: Fn11, p456: (Option<Bytes>, List<Bytes>), p457: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m455 = (p456, p457);
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, v2) = m455;
                let m453 = v2;
                match m453.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c454: Fn11 = v0.clone();
                            c454.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f111(v0, v1)
}

pub fn f91(v0: List<Adt6>) -> Adt6 {
    f112(Fn14::F113, Adt6::C0, v0)
}

pub fn f92(v0: Adt24, v1: Adt6) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(h458, _, _) => {
            h458
        }
    }, match v0.clone() {
        Adt24::C0(_, h459, _) => {
            h459
        }
    }, List::cons(v1, match v0 {
        Adt24::C0(_, _, h460) => {
            h460
        }
    }))
}

pub fn f93(v0: Adt17, v1: Bytes, v2: u64) -> R<Bytes> {
    Ok(f7(f7(f7(f7(f7(f3(Str::lit("max_scope_depth exceeded in phase parse (")), v1), f3(Str::lit("): configured "))), f114(match v0 {
        Adt17::C0(_, _, _, h461) => {
            h461
        }
    })?), f3(Str::lit(", nesting depth "))), f114(v2)?))
}

pub fn f94(v0: Adt17, v1: Adt19, v2: Bytes) -> R<Adt20> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Bytes = {
        let m464 = match v3.clone() {
            Adt20::C0(h463, _) => {
                (*h463).clone()
            }
        };
        match m464 {
            Adt5::C0(_, _, h462) => {
                h462
            }
        }
    };
    Ok(Adt20::C0(std::rc::Rc::new(match v3.clone() {
        Adt20::C0(h465, _) => {
            (*h465).clone()
        }
    }), std::rc::Rc::new(if f115(v4.clone())? {
        f6(v0, match v3.clone() {
            Adt20::C0(_, h466) => {
                (*h466).clone()
            }
        }, match v3 {
            Adt20::C0(h467, _) => {
                (*h467).clone()
            }
        }, v2)?
    } else {
        f4(v0, match v3 {
            Adt20::C0(_, h468) => {
                (*h468).clone()
            }
        }, f7(f8(v4), f3(Str::lit(" is not a component ID"))))?
    })))
}

pub fn f95(v0: Adt8) -> bool {
    let m469 = v0;
    match m469 {
        Adt8::C0 => {
            false
        }
        Adt8::C1 => {
            false
        }
        Adt8::C2 => {
            false
        }
        Adt8::C3 => {
            true
        }
        Adt8::C4 => {
            true
        }
        Adt8::C5 => {
            true
        }
    }
}

pub fn f96(v0: Adt17, v1: Adt19) -> R<Adt34> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h470, _, _) => {
            h470
        }
    };
    if f10(v0.clone(), v2, f3(Str::lit("\\noaxioms")))? {
        let v3: Adt19 = f11(v0, v1, f3(Str::lit("\\noaxioms")))?;
        Ok(Adt34::C0(std::rc::Rc::new(Adt9::C0(Adt10::C0, List::<Bytes>::nil(), {
            let a472 = match v3.clone() {
                Adt19::C0(h471, _, _) => {
                    h471
                }
            };
            let a473 = 1u64;
            nat_sub(a472, a473)
        }, None::<Adt5>)), std::rc::Rc::new(v3)))
    } else {
        if f10(v0.clone(), v2, f3(Str::lit("\\allowaxioms")))? {
            f116(v0, v1, Adt10::C1, f3(Str::lit("\\allowaxioms")), f3(Str::lit("allowaxioms")))
        } else {
            if f10(v0.clone(), v2, f3(Str::lit("\\exactaxioms")))? {
                f116(v0, v1, Adt10::C2, f3(Str::lit("\\exactaxioms")), f3(Str::lit("exactaxioms")))
            } else {
                Ok(Adt34::C0(std::rc::Rc::new(Adt9::C0(Adt10::C0, List::<Bytes>::nil(), 0u64, None::<Adt5>)), std::rc::Rc::new(f4(v0, v1, f3(Str::lit("every definition and theorem-like declaration requires exactly one axiom policy")))?)))
            }
        }
    }
}

pub fn f97(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: u64, v4: Bytes, v5: u64, v6: Adt27, v7: List<Adt28>) -> R<Adt22> {
    Ok(Adt22::C0(std::rc::Rc::new({
        let a475 = match v0.clone() {
            Adt17::C0(_, _, _, h474) => {
                h474
            }
        };
        let a476 = v5;
        if nat_lt(a475, a476) {
            f16(v0.clone(), v1, f3(Str::lit("LLS8002")), f93(v0, f3(Str::lit("proof environment nesting")), v5)?)?
        } else {
            v1
        }
    }), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(Adt26::C0(v3, v4, v5, List::<Adt12>::nil())), std::rc::Rc::new(v6), v7))))
}

pub fn f98(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: Option<Adt12>) -> R<Adt22> {
    let v5: Adt19 = f50(v0, v1, match v3.clone() {
        Adt27::C0(_, h477, _, _, _, _, _, _) => {
            h477
        }
    })?;
    Ok(Adt22::C0(std::rc::Rc::new(v5.clone()), v2, std::rc::Rc::new(Adt25::C0(std::rc::Rc::new(f92(match v3.clone() {
        Adt27::C0(h478, _, _, _, _, _, _, _) => {
            (*h478).clone()
        }
    }, Adt6::C3(std::rc::Rc::new(Adt7::C0(match v3.clone() {
        Adt27::C0(_, _, h479, _, _, _, _, _) => {
            h479
        }
    }, std::rc::Rc::new(match v3.clone() {
        Adt27::C0(_, _, _, h480, _, _, _, _) => {
            (*h480).clone()
        }
    }), match v3.clone() {
        Adt27::C0(_, _, _, _, h481, _, _, _) => {
            h481
        }
    }, std::rc::Rc::new(match v3.clone() {
        Adt27::C0(_, _, _, _, _, h482, _, _) => {
            (*h482).clone()
        }
    }), std::rc::Rc::new(match v3.clone() {
        Adt27::C0(_, _, _, _, _, _, h483, _) => {
            (*h483).clone()
        }
    }), v4, match v3 {
        Adt27::C0(_, _, _, _, _, _, _, h484) => {
            h484
        }
    }, match v5 {
        Adt19::C0(h485, _, _) => {
            h485
        }
    })))))))))
}

pub fn f99(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt29) -> R<Adt22> {
    let v6: Bytes = match v5.clone() {
        Adt29::C0(_, h486, _, _, _, _) => {
            h486
        }
    };
    if f25(v0.clone(), match v1.clone() {
        Adt19::C0(h487, _, _) => {
            h487
        }
    }, v6.clone())? {
        let v7: Adt18 = f1(v0.clone(), v1)?;
        let v8: Adt20 = f5(v0.clone(), match v7.clone() {
            Adt18::C0(_, _, h488) => {
                (*h488).clone()
            }
        })?;
        let v9: Adt19 = f6(v0.clone(), match v8.clone() {
            Adt20::C0(_, h489) => {
                (*h489).clone()
            }
        }, match v8.clone() {
            Adt20::C0(h490, _) => {
                (*h490).clone()
            }
        }, f7(f3(Str::lit("env.")), v6.clone()))?;
        let m494 = f117({
            let m493 = match v8.clone() {
                Adt20::C0(h492, _) => {
                    (*h492).clone()
                }
            };
            match m493 {
                Adt5::C0(_, _, h491) => {
                    h491
                }
            }
        })?;
        match m494 {
            None => {
                Ok(Adt22::C0(std::rc::Rc::new(f4(v0, v9, f7(f8({
                    let m503 = match v8 {
                        Adt20::C0(h502, _) => {
                            (*h502).clone()
                        }
                    };
                    match m503 {
                        Adt5::C0(_, _, h501) => {
                            h501
                        }
                    }
                }), f3(Str::lit(" is not a decimal label"))))?), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(match v5 {
                    Adt29::C0(h504, _, _, _, _, _) => {
                        (*h504).clone()
                    }
                }), std::rc::Rc::new(v3), v4))))
            }
            Some(v10) => {
                f97(v0, v9, v2, match v7 {
                    Adt18::C0(h495, _, _) => {
                        h495
                    }
                }, v6, {
                    let a499 = {
                        let m498 = match v5.clone() {
                            Adt29::C0(h497, _, _, _, _, _) => {
                                (*h497).clone()
                            }
                        };
                        match m498 {
                            Adt26::C0(_, _, h496, _) => {
                                h496
                            }
                        }
                    };
                    let a500 = 1u64;
                    nat_add(a499, a500)?
                }, v3, List::cons(Adt28::C1(std::rc::Rc::new(v5), v10), v4))
            }
        }
    } else {
        let v11: Adt19 = f50(v0, v1, match v5.clone() {
            Adt29::C0(_, _, h505, _, _, _) => {
                h505
            }
        })?;
        let v12: Adt12 = f118(match v5.clone() {
            Adt29::C0(_, _, _, _, h506, _) => {
                h506
            }
        });
        Ok(Adt22::C0(std::rc::Rc::new(v11), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(f86(match v5.clone() {
            Adt29::C0(h507, _, _, _, _, _) => {
                (*h507).clone()
            }
        }, {
            let m509 = match v5.clone() {
                Adt29::C0(_, _, _, h508, _, _) => {
                    h508
                }
            };
            match m509 {
                None => {
                    Adt12::C8(std::rc::Rc::new(v12), match v5 {
                        Adt29::C0(_, _, _, _, _, h511) => {
                            h511
                        }
                    })
                }
                Some(v13) => {
                    Adt12::C7(std::rc::Rc::new(v13), std::rc::Rc::new(v12), match v5 {
                        Adt29::C0(_, _, _, _, _, h510) => {
                            h510
                        }
                    })
                }
            }
        })), std::rc::Rc::new(v3), v4))))
    }
}

pub fn f100(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt27, v4: List<Adt28>, v5: Adt30) -> R<Adt22> {
    if f25(v0.clone(), match v1.clone() {
        Adt19::C0(h512, _, _) => {
            h512
        }
    }, f3(Str::lit("case")))? {
        let v6: Adt18 = f1(v0.clone(), v1)?;
        let v7: Adt20 = f5(v0.clone(), match v6.clone() {
            Adt18::C0(_, _, h513) => {
                (*h513).clone()
            }
        })?;
        let v8: Adt20 = f5(v0.clone(), f11(v0.clone(), f6(v0.clone(), match v7.clone() {
            Adt20::C0(_, h514) => {
                (*h514).clone()
            }
        }, match v7.clone() {
            Adt20::C0(h515, _) => {
                (*h515).clone()
            }
        }, f3(Str::lit("env.case")))?, f3(Str::lit("\\bind")))?)?;
        let v9: Adt19 = f6(v0.clone(), match v8.clone() {
            Adt20::C0(_, h516) => {
                (*h516).clone()
            }
        }, match v8.clone() {
            Adt20::C0(h517, _) => {
                (*h517).clone()
            }
        }, f3(Str::lit("bind")))?;
        let v10: Bytes = {
            let m520 = match v8.clone() {
                Adt20::C0(h519, _) => {
                    (*h519).clone()
                }
            };
            match m520 {
                Adt5::C0(_, _, h518) => {
                    h518
                }
            }
        };
        f97(v0, v9, v2, match v6 {
            Adt18::C0(h521, _, _) => {
                h521
            }
        }, f3(Str::lit("case")), {
            let a525 = {
                let m524 = match v5.clone() {
                    Adt30::C0(h523, _, _, _, _, _) => {
                        (*h523).clone()
                    }
                };
                match m524 {
                    Adt26::C0(_, _, h522, _) => {
                        h522
                    }
                }
            };
            let a526 = 1u64;
            nat_add(a525, a526)?
        }, v3, List::cons(Adt28::C2(std::rc::Rc::new(v5), std::rc::Rc::new(match v7 {
            Adt20::C0(h527, _) => {
                (*h527).clone()
            }
        }), {
            let a529 = {
                let a528 = v10.clone();
                length_bytes(a528)
            };
            let a530 = 0u64;
            if nat_eq(a529, a530) {
                List::<Bytes>::nil()
            } else {
                f119(v10, 59u8)?
            }
        }, std::rc::Rc::new(match v8 {
            Adt20::C0(h531, _) => {
                (*h531).clone()
            }
        })), v4))
    } else {
        let v11: Adt19 = f50(v0, v1, match v5.clone() {
            Adt30::C0(_, _, h532, _, _, _) => {
                h532
            }
        })?;
        Ok(Adt22::C0(std::rc::Rc::new(v11), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(f86(match v5.clone() {
            Adt30::C0(h533, _, _, _, _, _) => {
                (*h533).clone()
            }
        }, Adt12::C9(match v5.clone() {
            Adt30::C0(_, h534, _, _, _, _) => {
                h534
            }
        }, std::rc::Rc::new(match v5.clone() {
            Adt30::C0(_, _, _, h535, _, _) => {
                (*h535).clone()
            }
        }), std::rc::Rc::new(f120(match v5.clone() {
            Adt30::C0(_, _, _, _, h536, _) => {
                h536
            }
        })), match v5 {
            Adt30::C0(_, _, _, _, _, h537) => {
                h537
            }
        }))), std::rc::Rc::new(v3), v4))))
    }
}

pub fn f101(v0: Fn12, p541: Adt12, p542: List<Adt12>) -> Adt12 {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v1, v2) = m540;
                let m538 = v2;
                match m538.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c539: Fn12 = v0.clone();
                            c539.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f103(v0: Adt17, v1: Adt19, v2: List<Adt23>, v3: Adt26, v4: Adt27, v5: List<Adt28>, v6: u64) -> R<Adt22> {
    let v7: Adt20 = f5(v0.clone(), v1)?;
    let v8: Adt33 = f85(v0.clone(), f6(v0.clone(), match v7.clone() {
        Adt20::C0(_, h543) => {
            (*h543).clone()
        }
    }, match v7.clone() {
        Adt20::C0(h544, _) => {
            (*h544).clone()
        }
    }, f3(Str::lit("env.have")))?)?;
    let v9: Adt18 = f1(v0.clone(), match v8.clone() {
        Adt33::C0(_, h545) => {
            (*h545).clone()
        }
    })?;
    if f2(match v9.clone() {
        Adt18::C0(_, h546, _) => {
            h546
        }
    }, f3(Str::lit("proof"))) {
        f97(v0, match v9.clone() {
            Adt18::C0(_, _, h547) => {
                (*h547).clone()
            }
        }, v2, match v9 {
            Adt18::C0(h548, _, _) => {
                h548
            }
        }, f3(Str::lit("proof")), {
            let a550 = match v3.clone() {
                Adt26::C0(_, _, h549, _) => {
                    h549
                }
            };
            let a551 = 1u64;
            nat_add(a550, a551)?
        }, v4, List::cons(Adt28::C0(std::rc::Rc::new(v3), std::rc::Rc::new(match v7 {
            Adt20::C0(h552, _) => {
                (*h552).clone()
            }
        }), std::rc::Rc::new(match v8 {
            Adt33::C0(h553, _) => {
                (*h553).clone()
            }
        }), v6), v5))
    } else {
        Ok(Adt22::C0(std::rc::Rc::new(f4(v0, match v9 {
            Adt18::C0(_, _, h554) => {
                (*h554).clone()
            }
        }, f3(Str::lit("a have establishes its proposition with a nested proof")))?), v2, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(v3), std::rc::Rc::new(v4), v5))))
    }
}

pub fn f104(_: Adt19, v1: List<Adt23>, v2: Adt26, v3: Adt27, v4: List<Adt28>, v5: Adt36) -> Adt22 {
    Adt22::C0(std::rc::Rc::new(match v5.clone() {
        Adt36::C0(_, h555) => {
            (*h555).clone()
        }
    }), v1, std::rc::Rc::new(Adt25::C1(std::rc::Rc::new(f86(v2, match v5 {
        Adt36::C0(h556, _) => {
            (*h556).clone()
        }
    })), std::rc::Rc::new(v3), v4)))
}

pub fn f105(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Adt19 = f6(v0.clone(), match v3.clone() {
        Adt20::C0(_, h557) => {
            (*h557).clone()
        }
    }, match v3.clone() {
        Adt20::C0(h558, _) => {
            (*h558).clone()
        }
    }, f3(Str::lit("env.rewrite")))?;
    let v5: (Adt19, List<(bool, Adt5)>) = {
        let (h559, _) = f121(Fn15::F122(std::rc::Rc::new(v0.clone())), {
            let a561 = match v0.clone() {
                Adt17::C0(_, _, h560, _) => {
                    h560
                }
            };
            let a562 = 1u64;
            nat_add(a561, a562)?
        }, (v4, List::<(bool, Adt5)>::nil()))?;
        h559
    };
    let v6: Adt19 = f50(v0.clone(), {
        let (h563, _) = v5.clone();
        h563
    }, f3(Str::lit("rewrite")))?;
    Ok(Adt36::C0(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(match v3 {
        Adt20::C0(h564, _) => {
            (*h564).clone()
        }
    }), f123({
        let (_, h565) = v5.clone();
        h565
    }), v2)), std::rc::Rc::new({
        let a568 = {
            let a567 = {
                let (_, h566) = v5;
                h566
            };
            length_list(a567)
        };
        let a569 = 0u64;
        if nat_eq(a568, a569) {
            f16(v0, v6, f3(Str::lit("LLF5003")), f3(Str::lit("a rewrite requires at least one rule")))?
        } else {
            v6
        }
    })))
}

pub fn f106(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), v1)?;
    let v4: Adt19 = f6(v0.clone(), match v3.clone() {
        Adt20::C0(_, h570) => {
            (*h570).clone()
        }
    }, match v3.clone() {
        Adt20::C0(h571, _) => {
            (*h571).clone()
        }
    }, f3(Str::lit("env.simplify")))?;
    let v5: (Adt19, List<Adt5>) = {
        let (h572, _) = f19(Fn0::F124(std::rc::Rc::new(v0.clone())), {
            let a574 = match v0.clone() {
                Adt17::C0(_, _, h573, _) => {
                    h573
                }
            };
            let a575 = 1u64;
            nat_add(a574, a575)?
        }, (v4, List::<Adt5>::nil()))?;
        h572
    };
    let v6: Adt19 = f50(v0.clone(), {
        let (h576, _) = v5.clone();
        h576
    }, f3(Str::lit("simplify")))?;
    Ok(Adt36::C0(std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(match v3 {
        Adt20::C0(h577, _) => {
            (*h577).clone()
        }
    }), f13({
        let (_, h578) = v5.clone();
        h578
    }), v2)), std::rc::Rc::new({
        let a581 = {
            let a580 = {
                let (_, h579) = v5;
                h579
            };
            length_list(a580)
        };
        let a582 = 0u64;
        if nat_eq(a581, a582) {
            f16(v0, v6, f3(Str::lit("LLF5003")), f3(Str::lit("a simplify requires at least one rule")))?
        } else {
            v6
        }
    })))
}

pub fn f107(v0: Adt17, v1: Adt19, v2: u64) -> R<Adt36> {
    let v3: Adt20 = f5(v0.clone(), f11(v0.clone(), v1, f3(Str::lit("\\start")))?)?;
    let v4: (Adt19, List<Adt13>) = {
        let (h583, _) = f125(Fn16::F126(std::rc::Rc::new(v0.clone())), {
            let a585 = match v0.clone() {
                Adt17::C0(_, _, h584, _) => {
                    h584
                }
            };
            let a586 = 1u64;
            nat_add(a585, a586)?
        }, (match v3.clone() {
            Adt20::C0(_, h587) => {
                (*h587).clone()
            }
        }, List::<Adt13>::nil()))?;
        h583
    };
    let v5: Adt19 = f50(v0.clone(), {
        let (h588, _) = v4.clone();
        h588
    }, f3(Str::lit("calculate")))?;
    Ok(Adt36::C0(std::rc::Rc::new(Adt12::C10(std::rc::Rc::new(match v3 {
        Adt20::C0(h589, _) => {
            (*h589).clone()
        }
    }), f127({
        let (_, h590) = v4.clone();
        h590
    }), v2)), std::rc::Rc::new({
        let a593 = {
            let a592 = {
                let (_, h591) = v4;
                h591
            };
            length_list(a592)
        };
        let a594 = 0u64;
        if nat_eq(a593, a594) {
            f16(v0, v5, f3(Str::lit("LLF5003")), f3(Str::lit("a calculation requires at least one step")))?
        } else {
            v5
        }
    })))
}

pub fn f108(v0: Fn13, p599: u64, p600: Adt35) -> R<(Adt35, bool)> {
    let m598 = (p599, p600);
    {
        let mut m598 = m598;
        loop {
            let step = {
                let (v1, v2) = m598;
                let m595 = v1;
                if m595 == 0 {
                    Err::<(u64, Adt35), R<(Adt35, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m595 - 1;
                    let m597 = {
                        let c596: Fn13 = v0.clone();
                        c596.apply(v2.clone())?
                    };
                    match m597 {
                        None => {
                            Err::<(u64, Adt35), R<(Adt35, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt35), R<(Adt35, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Adt17, v1: Adt35) -> R<Option<Adt35>> {
    f128(v0, v1)
}

pub fn f110(v0: Bytes, v1: Adt32) -> R<Option<Adt32>> {
    let m602 = f129(v0, match v1.clone() {
        Adt32::C0(h601, _, _) => {
            h601
        }
    });
    match m602 {
        None => {
            Ok(None::<Adt32>)
        }
        Some(v2) => {
            let v3: u64 = {
                let a604 = match v1.clone() {
                    Adt32::C0(h603, _, _) => {
                        h603
                    }
                };
                let a605 = 1u64;
                nat_add(a604, a605)?
            };
            let v4: bool = match v1.clone() {
                Adt32::C0(_, h606, _) => {
                    h606
                }
            };
            if match v1 {
                Adt32::C0(_, _, h607) => {
                    h607
                }
            } {
                Ok(Some(Adt32::C0(v3, v4, false)))
            } else {
                if if v4 {
                    let a608 = v2;
                    let a609 = 92u8;
                    equal(a608, a609)
                } else {
                    false
                } {
                    Ok(Some(Adt32::C0(v3, v4, true)))
                } else {
                    let a610 = v2;
                    let a611 = 34u8;
                    if equal(a610, a611) {
                        Ok(Some(Adt32::C0(v3, {
                            let a612 = v4;
                            bool_not(a612)
                        }, false)))
                    } else {
                        Ok(Some(Adt32::C0(v3, v4, false)))
                    }
                }
            }
        }
    }
}

pub fn f111(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m614 = {
        let (h613, _) = v0;
        h613
    };
    match m614 {
        None => {
            (Some(v1), {
                let (_, h615) = v0;
                h615
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a616 = v2;
                let a617 = v1;
                append_bytes(a616, a617)
            }, {
                let (_, h618) = v0;
                h618
            }))
        }
    }
}

pub fn f112(v0: Fn14, p622: Adt6, p623: List<Adt6>) -> Adt6 {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m619 = v2;
                match m619.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c620: Fn14 = v0.clone();
                            c620.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C1(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f114(v0: u64) -> R<Bytes> {
    let v1: Bytes = f130(Str::lit("0"));
    let v2: Bytes = f131({
        let a660 = f132(v0, 10000000000000000000u64);
        let a661 = {
            let a658 = f132(v0, 1000000000000000000u64);
            let a659 = {
                let a656 = f132(v0, 100000000000000000u64);
                let a657 = {
                    let a654 = f132(v0, 10000000000000000u64);
                    let a655 = {
                        let a652 = f132(v0, 1000000000000000u64);
                        let a653 = {
                            let a650 = f132(v0, 100000000000000u64);
                            let a651 = {
                                let a648 = f132(v0, 10000000000000u64);
                                let a649 = {
                                    let a646 = f132(v0, 1000000000000u64);
                                    let a647 = {
                                        let a644 = f132(v0, 100000000000u64);
                                        let a645 = {
                                            let a642 = f132(v0, 10000000000u64);
                                            let a643 = {
                                                let a640 = f132(v0, 1000000000u64);
                                                let a641 = {
                                                    let a638 = f132(v0, 100000000u64);
                                                    let a639 = {
                                                        let a636 = f132(v0, 10000000u64);
                                                        let a637 = {
                                                            let a634 = f132(v0, 1000000u64);
                                                            let a635 = {
                                                                let a632 = f132(v0, 100000u64);
                                                                let a633 = {
                                                                    let a630 = f132(v0, 10000u64);
                                                                    let a631 = {
                                                                        let a628 = f132(v0, 1000u64);
                                                                        let a629 = {
                                                                            let a626 = f132(v0, 100u64);
                                                                            let a627 = {
                                                                                let a624 = f132(v0, 10u64);
                                                                                let a625 = f132(v0, 1u64);
                                                                                append_bytes(a624, a625)
                                                                            };
                                                                            append_bytes(a626, a627)
                                                                        };
                                                                        append_bytes(a628, a629)
                                                                    };
                                                                    append_bytes(a630, a631)
                                                                };
                                                                append_bytes(a632, a633)
                                                            };
                                                            append_bytes(a634, a635)
                                                        };
                                                        append_bytes(a636, a637)
                                                    };
                                                    append_bytes(a638, a639)
                                                };
                                                append_bytes(a640, a641)
                                            };
                                            append_bytes(a642, a643)
                                        };
                                        append_bytes(a644, a645)
                                    };
                                    append_bytes(a646, a647)
                                };
                                append_bytes(a648, a649)
                            };
                            append_bytes(a650, a651)
                        };
                        append_bytes(a652, a653)
                    };
                    append_bytes(a654, a655)
                };
                append_bytes(a656, a657)
            };
            append_bytes(a658, a659)
        };
        append_bytes(a660, a661)
    })?;
    let a662 = v0;
    let a663 = 0u64;
    if nat_eq(a662, a663) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f115(v0: Bytes) -> R<bool> {
    let m664 = f129(v0.clone(), 0u64);
    match m664 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f133(v1, 97u8, 122u8) {
                let a668 = {
                    let (h665, _) = f44(Fn4::F134(v0.clone()), {
                        let a666 = v0.clone();
                        length_bytes(a666)
                    }, 1u64)?;
                    h665
                };
                let a669 = {
                    let a667 = v0;
                    length_bytes(a667)
                };
                Ok(nat_eq(a668, a669))
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f116(v0: Adt17, v1: Adt19, v2: Adt10, v3: Bytes, v4: Bytes) -> R<Adt34> {
    let v5: Adt19 = f11(v0.clone(), v1, v3)?;
    let v6: u64 = {
        let a671 = match v5.clone() {
            Adt19::C0(h670, _, _) => {
                h670
            }
        };
        let a672 = 1u64;
        nat_sub(a671, a672)
    };
    let v7: Adt20 = f5(v0.clone(), v5)?;
    let v8: Adt19 = f6(v0.clone(), match v7.clone() {
        Adt20::C0(_, h673) => {
            (*h673).clone()
        }
    }, match v7.clone() {
        Adt20::C0(h674, _) => {
            (*h674).clone()
        }
    }, v4)?;
    let v9: (List<Bytes>, Option<Bytes>) = f135(Fn17::F136, (List::<Bytes>::nil(), None::<Bytes>), f119({
        let m677 = match v7.clone() {
            Adt20::C0(h676, _) => {
                (*h676).clone()
            }
        };
        match m677 {
            Adt5::C0(_, _, h675) => {
                h675
            }
        }
    }, 59u8)?)?;
    Ok(Adt34::C0(std::rc::Rc::new(Adt9::C0(v2, f71(Fn9::F137, List::<Bytes>::nil(), {
        let (h678, _) = v9;
        h678
    })?, v6, Some(match v7 {
        Adt20::C0(h679, _) => {
            (*h679).clone()
        }
    }))), std::rc::Rc::new({
        let m681 = {
            let (_, h680) = v9;
            h680
        };
        match m681 {
            None => {
                v8
            }
            Some(v10) => {
                f4(v0, v8, v10)?
            }
        }
    })))
}

pub fn f117(v0: Bytes) -> R<Option<u64>> {
    let v1: u64 = if f138(v0.clone(), 0u64, 43u8) {
        1u64
    } else {
        0u64
    };
    let a683 = v1;
    let a684 = {
        let a682 = v0.clone();
        length_bytes(a682)
    };
    if nat_lt(a683, a684) {
        let v2: Adt37 = {
            let (h685, _) = f139(Fn18::F140(v0.clone()), {
                let a686 = v0;
                length_bytes(a686)
            }, Adt37::C0(v1, 0u64, true))?;
            h685
        };
        if match v2.clone() {
            Adt37::C0(_, _, h687) => {
                h687
            }
        } {
            Ok(Some(match v2 {
                Adt37::C0(_, h688, _) => {
                    h688
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f118(v0: List<(u64, Adt12)>) -> Adt12 {
    f141(Fn19::F142, Adt12::C0, v0)
}

pub fn f119(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a689 = v0.clone();
        length_bytes(a689)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h690, _) = f143(Fn20::F144(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h690
    };
    let v4: u64 = {
        let (h691, _) = {
            let (_, h692) = v3.clone();
            h692
        };
        h691
    };
    f59(List::cons(f24(v0, v4, {
        let a693 = v2;
        let a694 = v4;
        nat_sub(a693, a694)
    }), {
        let (_, h695) = {
            let (_, h696) = v3;
            h696
        };
        h695
    }))
}

pub fn f120(v0: List<Adt31>) -> Adt12 {
    f145(Fn21::F146, Adt12::C0, v0)
}

pub fn f121(v0: Fn15, p701: u64, p702: (Adt19, List<(bool, Adt5)>)) -> R<((Adt19, List<(bool, Adt5)>), bool)> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v1, v2) = m700;
                let m697 = v1;
                if m697 == 0 {
                    Err::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m697 - 1;
                    let m699 = {
                        let c698: Fn15 = v0.clone();
                        c698.apply(v2.clone())?
                    };
                    match m699 {
                        None => {
                            Err::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<(bool, Adt5)>)), R<((Adt19, List<(bool, Adt5)>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt17, v1: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
    f147(v0, v1)
}

pub fn f123(v0: List<(bool, Adt5)>) -> List<(bool, Adt5)> {
    f148(Fn22::F149, List::<(bool, Adt5)>::nil(), v0)
}

pub fn f124(v0: Adt17, v1: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    f150(v0, v1)
}

pub fn f125(v0: Fn16, p707: u64, p708: (Adt19, List<Adt13>)) -> R<((Adt19, List<Adt13>), bool)> {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v1, v2) = m706;
                let m703 = v1;
                if m703 == 0 {
                    Err::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m703 - 1;
                    let m705 = {
                        let c704: Fn16 = v0.clone();
                        c704.apply(v2.clone())?
                    };
                    match m705 {
                        None => {
                            Err::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt19, List<Adt13>)), R<((Adt19, List<Adt13>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: Adt17, v1: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
    f151(v0, v1)
}

pub fn f127(v0: List<Adt13>) -> List<Adt13> {
    f152(Fn23::F153, List::<Adt13>::nil(), v0)
}

pub fn f128(v0: Adt17, v1: Adt35) -> R<Option<Adt35>> {
    let a710 = match v1.clone() {
        Adt35::C0(_, _, _, h709) => {
            h709
        }
    };
    let a711 = 0u64;
    if nat_eq(a710, a711) {
        let m713 = f42(v0, match v1.clone() {
            Adt35::C0(h712, _, _, _) => {
                h712
            }
        });
        match m713 {
            None => {
                Ok(f154(v1, 2u64))
            }
            Some(v2) => {
                let v3: u64 = f61(v2.clone());
                let v4: u64 = match v1.clone() {
                    Adt35::C0(_, h714, _, _) => {
                        h714
                    }
                };
                let v5: u64 = match v1.clone() {
                    Adt35::C0(_, _, h715, _) => {
                        h715
                    }
                };
                let a716 = v3;
                let a717 = 0u64;
                if nat_eq(a716, a717) {
                    f155(v1, match v2 {
                        Adt0::C0(_, _, _, _, _, _, _, h718) => {
                            h718
                        }
                    })
                } else {
                    let a719 = v3;
                    let a720 = 3u64;
                    if nat_eq(a719, a720) {
                        let a721 = v5;
                        let a722 = 0u64;
                        if nat_eq(a721, a722) {
                            if f2(match v2.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, h723) => {
                                    h723
                                }
                            }, f3(Str::lit("{"))) {
                                f156(v1, {
                                    let a724 = v4;
                                    let a725 = 1u64;
                                    nat_add(a724, a725)?
                                }, v5)
                            } else {
                                if f2(match v2 {
                                    Adt0::C0(_, _, _, _, _, _, _, h726) => {
                                        h726
                                    }
                                }, f3(Str::lit("}"))) {
                                    f156(v1, {
                                        let a727 = v4;
                                        let a728 = 1u64;
                                        nat_sub(a727, a728)
                                    }, v5)
                                } else {
                                    f156(v1, v4, v5)
                                }
                            }
                        } else {
                            f156(v1, v4, v5)
                        }
                    } else {
                        let a729 = v3;
                        let a730 = 4u64;
                        if if nat_eq(a729, a730) {
                            let a731 = v5;
                            let a732 = 0u64;
                            if nat_eq(a731, a732) {
                                let a733 = v4;
                                let a734 = 0u64;
                                if nat_eq(a733, a734) {
                                    f2(match v2 {
                                        Adt0::C0(_, _, _, _, _, _, _, h735) => {
                                            h735
                                        }
                                    }, f3(Str::lit(".")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(f154(v1, 1u64))
                        } else {
                            f156(v1, v4, v5)
                        }
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt35>)
    }
}

pub fn f129(v0: Bytes, v1: u64) -> Option<u8> {
    let a736 = v0;
    let a737 = v1;
    index_bytes(a736, a737)
}

pub fn f130(v0: Str) -> Bytes {
    let a738 = v0;
    utf8_encode(a738)
}

pub fn f131(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h739, _) = f44(Fn4::F157(v0.clone()), {
            let a740 = v0.clone();
            length_bytes(a740)
        }, 0u64)?;
        h739
    };
    Ok(f24(v0.clone(), v1, {
        let a742 = {
            let a741 = v0;
            length_bytes(a741)
        };
        let a743 = v1;
        nat_sub(a742, a743)
    }))
}

pub fn f132(v0: u64, v1: u64) -> Bytes {
    f24(f130(Str::lit("0123456789")), {
        let a747 = {
            let a744 = v0;
            let a745 = v1;
            let a746 = 0u64;
            nat_quot(a744, a745, a746)
        };
        let a748 = 10u64;
        let a749 = 0u64;
        nat_rem(a747, a748, a749)
    }, 1u64)
}

pub fn f133(v0: u8, v1: u8, v2: u8) -> bool {
    let m752 = {
        let a750 = v0;
        let a751 = v1;
        fixed_u8::checked_sub(a750, a751)
    };
    match m752 {
        None => {
            false
        }
        Some(_) => {
            let m755 = {
                let a753 = v2;
                let a754 = v0;
                fixed_u8::checked_sub(a753, a754)
            };
            match m755 {
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

pub fn f134(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f158(v0, v1)
}

pub fn f135(v0: Fn17, p759: (List<Bytes>, Option<Bytes>), p760: List<Bytes>) -> R<(List<Bytes>, Option<Bytes>)> {
    let m758 = (p759, p760);
    {
        let mut m758 = m758;
        loop {
            let step = {
                let (v1, v2) = m758;
                let m756 = v2;
                match m756.uncons() {
                    None => {
                        Err::<((List<Bytes>, Option<Bytes>), List<Bytes>), R<(List<Bytes>, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Bytes>, Option<Bytes>), List<Bytes>), R<(List<Bytes>, Option<Bytes>)>>(({
                            let c757: Fn17 = v0.clone();
                            c757.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
    f159(v0, v1)
}

pub fn f137(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    f160(v0, v1)
}

pub fn f138(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m761 = f129(v0, v1);
    match m761 {
        None => {
            false
        }
        Some(v3) => {
            let a762 = v3;
            let a763 = v2;
            equal(a762, a763)
        }
    }
}

pub fn f139(v0: Fn18, p768: u64, p769: Adt37) -> R<(Adt37, bool)> {
    let m767 = (p768, p769);
    {
        let mut m767 = m767;
        loop {
            let step = {
                let (v1, v2) = m767;
                let m764 = v1;
                if m764 == 0 {
                    Err::<(u64, Adt37), R<(Adt37, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m764 - 1;
                    let m766 = {
                        let c765: Fn18 = v0.clone();
                        c765.apply(v2.clone())?
                    };
                    match m766 {
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
                Ok(next) => { m767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    f161(v0, v1)
}

pub fn f141(v0: Fn19, p773: Adt12, p774: List<(u64, Adt12)>) -> Adt12 {
    let m772 = (p773, p774);
    {
        let mut m772 = m772;
        loop {
            let step = {
                let (v1, v2) = m772;
                let m770 = v2;
                match m770.uncons() {
                    None => {
                        Err::<(Adt12, List<(u64, Adt12)>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<(u64, Adt12)>), Adt12>(({
                            let c771: Fn19 = v0.clone();
                            c771.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m772 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Adt12, v1: (u64, Adt12)) -> Adt12 {
    Adt12::C11({
        let (h775, _) = v1;
        h775
    }, std::rc::Rc::new({
        let (_, h776) = v1;
        h776
    }), std::rc::Rc::new(v0))
}

pub fn f143(v0: Fn20, p781: u64, p782: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m780 = (p781, p782);
    {
        let mut m780 = m780;
        loop {
            let step = {
                let (v1, v2) = m780;
                let m777 = v1;
                if m777 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m777 - 1;
                    let m779 = {
                        let c778: Fn20 = v0.clone();
                        c778.apply(v2.clone())?
                    };
                    match m779 {
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
                Ok(next) => { m780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f162(v1, v0, v2)
}

pub fn f145(v0: Fn21, p786: Adt12, p787: List<Adt31>) -> Adt12 {
    let m785 = (p786, p787);
    {
        let mut m785 = m785;
        loop {
            let step = {
                let (v1, v2) = m785;
                let m783 = v2;
                match m783.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt31>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt31>), Adt12>(({
                            let c784: Fn21 = v0.clone();
                            c784.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Adt12, v1: Adt31) -> Adt12 {
    Adt12::C12(std::rc::Rc::new(match v1.clone() {
        Adt31::C0(h788, _, _, _) => {
            (*h788).clone()
        }
    }), match v1.clone() {
        Adt31::C0(_, h789, _, _) => {
            h789
        }
    }, std::rc::Rc::new(match v1.clone() {
        Adt31::C0(_, _, h790, _) => {
            (*h790).clone()
        }
    }), std::rc::Rc::new(match v1 {
        Adt31::C0(_, _, _, h791) => {
            (*h791).clone()
        }
    }), std::rc::Rc::new(v0))
}

pub fn f147(v0: Adt17, v1: (Adt19, List<(bool, Adt5)>)) -> R<Option<(Adt19, List<(bool, Adt5)>)>> {
    let v2: Adt19 = {
        let (h792, _) = v1;
        h792
    };
    let m794 = match v2.clone() {
        Adt19::C0(_, _, h793) => {
            h793
        }
    };
    match m794 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h795, _, _) => {
                    h795
                }
            }, f3(Str::lit("\\forward")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\forward")))?)?;
                Ok(Some((match v4.clone() {
                    Adt20::C0(_, h796) => {
                        (*h796).clone()
                    }
                }, List::cons((false, match v4 {
                    Adt20::C0(h797, _) => {
                        (*h797).clone()
                    }
                }), {
                    let (_, h798) = v1;
                    h798
                }))))
            } else {
                if f10(v0.clone(), match v2.clone() {
                    Adt19::C0(h799, _, _) => {
                        h799
                    }
                }, f3(Str::lit("\\backward")))? {
                    let v5: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\backward")))?)?;
                    Ok(Some((match v5.clone() {
                        Adt20::C0(_, h800) => {
                            (*h800).clone()
                        }
                    }, List::cons((true, match v5 {
                        Adt20::C0(h801, _) => {
                            (*h801).clone()
                        }
                    }), {
                        let (_, h802) = v1;
                        h802
                    }))))
                } else {
                    Ok(None::<(Adt19, List<(bool, Adt5)>)>)
                }
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<(bool, Adt5)>)>)
        }
    }
}

pub fn f148(v0: Fn22, p806: List<(bool, Adt5)>, p807: List<(bool, Adt5)>) -> List<(bool, Adt5)> {
    let m805 = (p806, p807);
    {
        let mut m805 = m805;
        loop {
            let step = {
                let (v1, v2) = m805;
                let m803 = v2;
                match m803.uncons() {
                    None => {
                        Err::<(List<(bool, Adt5)>, List<(bool, Adt5)>), List<(bool, Adt5)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(bool, Adt5)>, List<(bool, Adt5)>), List<(bool, Adt5)>>(({
                            let c804: Fn22 = v0.clone();
                            c804.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<(bool, Adt5)>, v1: (bool, Adt5)) -> List<(bool, Adt5)> {
    List::cons(v1, v0)
}

pub fn f150(v0: Adt17, v1: (Adt19, List<Adt5>)) -> R<Option<(Adt19, List<Adt5>)>> {
    let v2: Adt19 = {
        let (h808, _) = v1;
        h808
    };
    let m810 = match v2.clone() {
        Adt19::C0(_, _, h809) => {
            h809
        }
    };
    match m810 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h811, _, _) => {
                    h811
                }
            }, f3(Str::lit("\\rule")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0, v2, f3(Str::lit("\\rule")))?)?;
                Ok(Some((match v4.clone() {
                    Adt20::C0(_, h812) => {
                        (*h812).clone()
                    }
                }, List::cons(match v4 {
                    Adt20::C0(h813, _) => {
                        (*h813).clone()
                    }
                }, {
                    let (_, h814) = v1;
                    h814
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt5>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt5>)>)
        }
    }
}

pub fn f151(v0: Adt17, v1: (Adt19, List<Adt13>)) -> R<Option<(Adt19, List<Adt13>)>> {
    let v2: Adt19 = {
        let (h815, _) = v1;
        h815
    };
    let m817 = match v2.clone() {
        Adt19::C0(_, _, h816) => {
            h816
        }
    };
    match m817 {
        None => {
            if f10(v0.clone(), match v2.clone() {
                Adt19::C0(h818, _, _) => {
                    h818
                }
            }, f3(Str::lit("\\step")))? {
                let v4: Adt20 = f5(v0.clone(), f11(v0.clone(), v2, f3(Str::lit("\\step")))?)?;
                let v5: Adt20 = f5(v0.clone(), f6(v0.clone(), match v4.clone() {
                    Adt20::C0(_, h819) => {
                        (*h819).clone()
                    }
                }, match v4.clone() {
                    Adt20::C0(h820, _) => {
                        (*h820).clone()
                    }
                }, f3(Str::lit("step")))?)?;
                let v6: Adt20 = f5(v0, match v5.clone() {
                    Adt20::C0(_, h821) => {
                        (*h821).clone()
                    }
                })?;
                Ok(Some((match v6.clone() {
                    Adt20::C0(_, h822) => {
                        (*h822).clone()
                    }
                }, List::cons(Adt13::C0(std::rc::Rc::new(match v4 {
                    Adt20::C0(h823, _) => {
                        (*h823).clone()
                    }
                }), std::rc::Rc::new(match v5 {
                    Adt20::C0(h824, _) => {
                        (*h824).clone()
                    }
                }), std::rc::Rc::new(match v6 {
                    Adt20::C0(h825, _) => {
                        (*h825).clone()
                    }
                })), {
                    let (_, h826) = v1;
                    h826
                }))))
            } else {
                Ok(None::<(Adt19, List<Adt13>)>)
            }
        }
        Some(_) => {
            Ok(None::<(Adt19, List<Adt13>)>)
        }
    }
}

pub fn f152(v0: Fn23, p830: List<Adt13>, p831: List<Adt13>) -> List<Adt13> {
    let m829 = (p830, p831);
    {
        let mut m829 = m829;
        loop {
            let step = {
                let (v1, v2) = m829;
                let m827 = v2;
                match m827.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>(({
                            let c828: Fn23 = v0.clone();
                            c828.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: List<Adt13>, v1: Adt13) -> List<Adt13> {
    List::cons(v1, v0)
}

pub fn f154(v0: Adt35, v1: u64) -> Option<Adt35> {
    Some(Adt35::C0(match v0.clone() {
        Adt35::C0(h832, _, _, _) => {
            h832
        }
    }, match v0.clone() {
        Adt35::C0(_, h833, _, _) => {
            h833
        }
    }, match v0 {
        Adt35::C0(_, _, h834, _) => {
            h834
        }
    }, v1))
}

pub fn f155(v0: Adt35, v1: Bytes) -> R<Option<Adt35>> {
    let v2: u64 = match v0.clone() {
        Adt35::C0(_, h835, _, _) => {
            h835
        }
    };
    let v3: u64 = match v0.clone() {
        Adt35::C0(_, _, h836, _) => {
            h836
        }
    };
    let a837 = v3;
    let a838 = 0u64;
    if nat_eq(a837, a838) {
        if f2(v1.clone(), f3(Str::lit("\\("))) {
            f156(v0, v2, 1u64)
        } else {
            if f2(v1.clone(), f3(Str::lit("\\["))) {
                f156(v0, v2, 2u64)
            } else {
                let a839 = v2;
                let a840 = 0u64;
                if if nat_eq(a839, a840) {
                    if f2(v1.clone(), f3(Str::lit("\\begin"))) {
                        true
                    } else {
                        f2(v1, f3(Str::lit("\\end")))
                    }
                } else {
                    false
                } {
                    Ok(f154(v0, 2u64))
                } else {
                    f156(v0, v2, 0u64)
                }
            }
        }
    } else {
        let a841 = v3;
        let a842 = 1u64;
        if if nat_eq(a841, a842) {
            f2(v1, f3(Str::lit("\\)")))
        } else {
            f2(v1, f3(Str::lit("\\]")))
        } {
            f156(v0, v2, 0u64)
        } else {
            f156(v0, v2, v3)
        }
    }
}

pub fn f156(v0: Adt35, v1: u64, v2: u64) -> R<Option<Adt35>> {
    Ok(Some(Adt35::C0({
        let a844 = match v0 {
            Adt35::C0(h843, _, _, _) => {
                h843
            }
        };
        let a845 = 1u64;
        nat_add(a844, a845)?
    }, v1, v2, 0u64)))
}

pub fn f157(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f163(v0, v1)
}

pub fn f158(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m846 = f129(v0, v1);
    match m846 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f164(v2) {
                Ok(Some({
                    let a847 = v1;
                    let a848 = 1u64;
                    nat_add(a847, a848)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f159(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> R<(List<Bytes>, Option<Bytes>)> {
    let m850 = {
        let (_, h849) = v0.clone();
        h849
    };
    match m850 {
        None => {
            let a852 = {
                let a851 = v1.clone();
                length_bytes(a851)
            };
            let a853 = 0u64;
            if nat_eq(a852, a853) {
                Ok(({
                    let (h854, _) = v0;
                    h854
                }, Some(f3(Str::lit("empty entry in an axiom name list")))))
            } else {
                if f165(v1.clone())? {
                    if f166(v1.clone(), {
                        let (h855, _) = v0.clone();
                        h855
                    }) {
                        Ok(({
                            let (h856, _) = v0;
                            h856
                        }, Some(f7(f3(Str::lit("duplicate axiom name ")), f8(v1)))))
                    } else {
                        Ok((List::cons(v1, {
                            let (h857, _) = v0;
                            h857
                        }), None::<Bytes>))
                    }
                } else {
                    Ok(({
                        let (h858, _) = v0;
                        h858
                    }, Some(f7(f8(v1), f3(Str::lit(" is not a conservative ASCII Lean name"))))))
                }
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f160(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let v2: (List<Bytes>, Option<Bytes>) = f135(Fn17::F167, (List::<Bytes>::nil(), Some(v1)), v0)?;
    f59({
        let m860 = {
            let (_, h859) = v2;
            h859
        };
        match m860 {
            None => {
                let (h862, _) = v2;
                h862
            }
            Some(v3) => {
                List::cons(v3, {
                    let (h861, _) = v2;
                    h861
                })
            }
        }
    })
}

pub fn f161(v0: Bytes, v1: Adt37) -> R<Option<Adt37>> {
    if match v1.clone() {
        Adt37::C0(_, _, h863) => {
            h863
        }
    } {
        let m865 = f129(v0, match v1.clone() {
            Adt37::C0(h864, _, _) => {
                h864
            }
        });
        match m865 {
            None => {
                Ok(None::<Adt37>)
            }
            Some(v2) => {
                let v3: u64 = {
                    let a867 = match v1.clone() {
                        Adt37::C0(h866, _, _) => {
                            h866
                        }
                    };
                    let a868 = 1u64;
                    nat_add(a867, a868)?
                };
                if f168(v2) {
                    let v4: u64 = f169(v2)?;
                    let a875 = match v1.clone() {
                        Adt37::C0(_, h869, _) => {
                            h869
                        }
                    };
                    let a876 = {
                        let a872 = {
                            let a870 = 18446744073709551615u64;
                            let a871 = v4;
                            nat_sub(a870, a871)
                        };
                        let a873 = 10u64;
                        let a874 = 0u64;
                        nat_quot(a872, a873, a874)
                    };
                    if nat_le(a875, a876) {
                        Ok(Some(Adt37::C0(v3, {
                            let a880 = {
                                let a878 = match v1 {
                                    Adt37::C0(_, h877, _) => {
                                        h877
                                    }
                                };
                                let a879 = 10u64;
                                nat_mul(a878, a879)?
                            };
                            let a881 = v4;
                            nat_add(a880, a881)?
                        }, true)))
                    } else {
                        Ok(Some(Adt37::C0(v3, 0u64, false)))
                    }
                } else {
                    Ok(Some(Adt37::C0(v3, 0u64, false)))
                }
            }
        }
    } else {
        Ok(None::<Adt37>)
    }
}

pub fn f162(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h882, _) = v2.clone();
        h882
    };
    let v4: u64 = {
        let (h883, _) = {
            let (_, h884) = v2.clone();
            h884
        };
        h883
    };
    let v5: List<Bytes> = {
        let (_, h885) = {
            let (_, h886) = v2;
            h886
        };
        h885
    };
    let m887 = f129(v0.clone(), v3);
    match m887 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a888 = v6;
            let a889 = v1;
            if equal(a888, a889) {
                Ok(Some(({
                    let a890 = v3;
                    let a891 = 1u64;
                    nat_add(a890, a891)?
                }, ({
                    let a892 = v3;
                    let a893 = 1u64;
                    nat_add(a892, a893)?
                }, List::cons(f24(v0, v4, {
                    let a894 = v3;
                    let a895 = v4;
                    nat_sub(a894, a895)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a896 = v3;
                    let a897 = 1u64;
                    nat_add(a896, a897)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f138(v0, v1, 48u8) {
        Ok(Some({
            let a898 = v1;
            let a899 = 1u64;
            nat_add(a898, a899)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f164(v0: u8) -> bool {
    if f133(v0, 97u8, 122u8) {
        true
    } else {
        if f168(v0) {
            true
        } else {
            let a900 = v0;
            let a901 = 45u8;
            equal(a900, a901)
        }
    }
}

pub fn f165(v0: Bytes) -> R<bool> {
    let v1: (u64, bool) = {
        let (h902, _) = f170(Fn24::F171(v0.clone()), {
            let a903 = v0.clone();
            length_bytes(a903)
        }, (0u64, true))?;
        h902
    };
    let a906 = {
        let (h904, _) = v1;
        h904
    };
    let a907 = {
        let a905 = v0;
        length_bytes(a905)
    };
    if nat_eq(a906, a907) {
        let a909 = {
            let (_, h908) = v1;
            h908
        };
        Ok(bool_not(a909))
    } else {
        Ok(false)
    }
}

pub fn f166(v0: Bytes, v1: List<Bytes>) -> bool {
    f172(Fn25::F173(v0), false, v1)
}

pub fn f167(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> (List<Bytes>, Option<Bytes>) {
    f174(v0, v1)
}

pub fn f168(v0: u8) -> bool {
    f133(v0, 48u8, 57u8)
}

pub fn f169(v0: u8) -> R<u64> {
    let a956 = {
        let v1: u8 = v0;
        let a954 = {
            let a930 = {
                let a918 = {
                    let a912 = {
                        let a910 = v1;
                        let a911 = 128u8;
                        fixed_u8::bit_and(a910, a911)
                    };
                    let a913 = 128u8;
                    if equal(a912, a913) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a919 = {
                    let a916 = {
                        let a914 = v1;
                        let a915 = 64u8;
                        fixed_u8::bit_and(a914, a915)
                    };
                    let a917 = 64u8;
                    if equal(a916, a917) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a918, a919)?
            };
            let a931 = {
                let a928 = {
                    let a922 = {
                        let a920 = v1;
                        let a921 = 32u8;
                        fixed_u8::bit_and(a920, a921)
                    };
                    let a923 = 32u8;
                    if equal(a922, a923) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a929 = {
                    let a926 = {
                        let a924 = v1;
                        let a925 = 16u8;
                        fixed_u8::bit_and(a924, a925)
                    };
                    let a927 = 16u8;
                    if equal(a926, a927) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a928, a929)?
            };
            nat_add(a930, a931)?
        };
        let a955 = {
            let a952 = {
                let a940 = {
                    let a934 = {
                        let a932 = v1;
                        let a933 = 8u8;
                        fixed_u8::bit_and(a932, a933)
                    };
                    let a935 = 8u8;
                    if equal(a934, a935) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a941 = {
                    let a938 = {
                        let a936 = v1;
                        let a937 = 4u8;
                        fixed_u8::bit_and(a936, a937)
                    };
                    let a939 = 4u8;
                    if equal(a938, a939) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a940, a941)?
            };
            let a953 = {
                let a950 = {
                    let a944 = {
                        let a942 = v1;
                        let a943 = 2u8;
                        fixed_u8::bit_and(a942, a943)
                    };
                    let a945 = 2u8;
                    if equal(a944, a945) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a951 = {
                    let a948 = {
                        let a946 = v1;
                        let a947 = 1u8;
                        fixed_u8::bit_and(a946, a947)
                    };
                    let a949 = 1u8;
                    if equal(a948, a949) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a950, a951)?
            };
            nat_add(a952, a953)?
        };
        nat_add(a954, a955)?
    };
    let a957 = 48u64;
    Ok(nat_sub(a956, a957))
}

pub fn f170(v0: Fn24, p962: u64, p963: (u64, bool)) -> R<((u64, bool), bool)> {
    let m961 = (p962, p963);
    {
        let mut m961 = m961;
        loop {
            let step = {
                let (v1, v2) = m961;
                let m958 = v1;
                if m958 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m958 - 1;
                    let m960 = {
                        let c959: Fn24 = v0.clone();
                        c959.apply(v2)?
                    };
                    match m960 {
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
                Ok(next) => { m961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Bytes, v1: (u64, bool)) -> R<Option<(u64, bool)>> {
    f175(v0, v1)
}

pub fn f172(v0: Fn25, p967: bool, p968: List<Bytes>) -> bool {
    let m966 = (p967, p968);
    {
        let mut m966 = m966;
        loop {
            let step = {
                let (v1, v2) = m966;
                let m964 = v2;
                match m964.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c965: Fn25 = v0.clone();
                            c965.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a969 = v2;
        let a970 = v0;
        equal(a969, a970)
    }
}

pub fn f174(v0: (List<Bytes>, Option<Bytes>), v1: Bytes) -> (List<Bytes>, Option<Bytes>) {
    let m972 = {
        let (_, h971) = v0;
        h971
    };
    match m972 {
        None => {
            (List::cons(v1, {
                let (h975, _) = v0;
                h975
            }), None::<Bytes>)
        }
        Some(v2) => {
            if f176(v2.clone(), v1.clone()) {
                (List::cons(v1, List::cons(v2, {
                    let (h973, _) = v0;
                    h973
                })), None::<Bytes>)
            } else {
                (List::cons(v1, {
                    let (h974, _) = v0;
                    h974
                }), Some(v2))
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: (u64, bool)) -> R<Option<(u64, bool)>> {
    let m977 = f129(v0, {
        let (h976, _) = v1;
        h976
    });
    match m977 {
        None => {
            Ok(None::<(u64, bool)>)
        }
        Some(v2) => {
            let v3: u64 = {
                let a979 = {
                    let (h978, _) = v1;
                    h978
                };
                let a980 = 1u64;
                nat_add(a979, a980)?
            };
            let v4: bool = if f177(v2) {
                true
            } else {
                let a981 = v2;
                let a982 = 95u8;
                equal(a981, a982)
            };
            let (_, h983) = v1;
            if h983 {
                if v4 {
                    Ok(Some((v3, false)))
                } else {
                    Ok(None::<(u64, bool)>)
                }
            } else {
                let a984 = v2;
                let a985 = 46u8;
                if equal(a984, a985) {
                    Ok(Some((v3, true)))
                } else {
                    if if v4 {
                        true
                    } else {
                        f168(v2)
                    } {
                        Ok(Some((v3, false)))
                    } else {
                        Ok(None::<(u64, bool)>)
                    }
                }
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: Bytes) -> bool {
    let a990 = {
        let a986 = v0;
        let a987 = v1;
        compare_bytes(a986, a987)
    };
    let a991 = {
        let a988 = Bytes::lit(&[]);
        let a989 = Bytes::lit(&[0x01]);
        compare_bytes(a988, a989)
    };
    equal(a990, a991)
}

pub fn f177(v0: u8) -> bool {
    let v1: bool = f133(v0, 97u8, 122u8);
    let v2: bool = f133(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn run(p0: Bytes, p1: List<Adt0>, p2: Bytes, p3: u64) -> R<Result<Adt2, Adt15>> {
    f0(p0, p1, p2, p3)
}
