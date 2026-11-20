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
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F7(Bytes, u64, Bytes, u64),
    F42(Bytes),
    F45(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F7(k0, k1, k2, k3) => f7(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn0::F42(k0) => f42(k0.clone(), p0),
            Fn0::F45(k0) => f45(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(bool, u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F15(k0, k1, k2) => f15(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
    F23,
    F31,
}

impl Fn2 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn2::F17 => f17(p0, p1),
            Fn2::F23 => f23(p0, p1),
            Fn2::F31 => f31(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F19(u8, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn3::F19(k0, k1) => f19(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F28,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn4::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F30,
}

impl Fn5 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn5::F30 => f30(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Adt0>> {
    let m1 = f1(v0.clone(), f2(Str::lit("@")))?;
    match m1 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v1) => {
            let v2: Bytes = f3(v0.clone(), 0u64, v1);
            let v3: Bytes = f3(v0.clone(), {
                let a2 = v1;
                let a3 = 1u64;
                nat_add(a2, a3)?
            }, {
                let a7 = {
                    let a4 = v0;
                    length_bytes(a4)
                };
                let a8 = {
                    let a5 = v1;
                    let a6 = 1u64;
                    nat_add(a5, a6)?
                };
                nat_sub(a7, a8)
            });
            if f4(v2.clone())? {
                if f5(v3.clone())? {
                    Ok(Some(Adt0::C0(v2, v3)))
                } else {
                    Ok(None::<Adt0>)
                }
            } else {
                Ok(None::<Adt0>)
            }
        }
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a9 = v0.clone();
        length_bytes(a9)
    };
    let v3: u64 = {
        let a10 = v1.clone();
        length_bytes(a10)
    };
    let v4: u64 = {
        let (h11, _) = f6(Fn0::F7(v1.clone(), v3, v0.clone(), v2), {
            let a12 = v2;
            let a13 = 1u64;
            nat_add(a12, a13)?
        }, 0u64)?;
        h11
    };
    if f8(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: Str) -> Bytes {
    let a14 = v0;
    utf8_encode(a14)
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m18 = {
        let a15 = v0;
        let a16 = v1;
        let a17 = v2;
        slice_bytes(a15, a16, a17)
    };
    match m18 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f4(v0: Bytes) -> R<bool> {
    f9(v0, false)
}

pub fn f5(v0: Bytes) -> R<bool> {
    let m19 = f1(v0.clone(), f2(Str::lit("+")))?;
    match m19 {
        None => {
            let m35 = f1(v0.clone(), f2(Str::lit("-")))?;
            match m35 {
                None => {
                    f12(v0)
                }
                Some(v5) => {
                    if f12(f3(v0.clone(), 0u64, v5))? {
                        f13(f11(f3(v0.clone(), {
                            let a36 = v5;
                            let a37 = 1u64;
                            nat_add(a36, a37)?
                        }, {
                            let a41 = {
                                let a38 = v0;
                                length_bytes(a38)
                            };
                            let a42 = {
                                let a39 = v5;
                                let a40 = 1u64;
                                nat_add(a39, a40)?
                            };
                            nat_sub(a41, a42)
                        }), 46u8)?)
                    } else {
                        Ok(false)
                    }
                }
            }
        }
        Some(v1) => {
            let v2: Bytes = f3(v0.clone(), 0u64, v1);
            let v3: Bytes = f3(v0.clone(), {
                let a20 = v1;
                let a21 = 1u64;
                nat_add(a20, a21)?
            }, {
                let a25 = {
                    let a22 = v0;
                    length_bytes(a22)
                };
                let a26 = {
                    let a23 = v1;
                    let a24 = 1u64;
                    nat_add(a23, a24)?
                };
                nat_sub(a25, a26)
            });
            if f10(f11(v3, 46u8)?)? {
                let m27 = f1(v2.clone(), f2(Str::lit("-")))?;
                match m27 {
                    None => {
                        f12(v2)
                    }
                    Some(v4) => {
                        if f12(f3(v2.clone(), 0u64, v4))? {
                            f13(f11(f3(v2.clone(), {
                                let a28 = v4;
                                let a29 = 1u64;
                                nat_add(a28, a29)?
                            }, {
                                let a33 = {
                                    let a30 = v2;
                                    length_bytes(a30)
                                };
                                let a34 = {
                                    let a31 = v4;
                                    let a32 = 1u64;
                                    nat_add(a31, a32)?
                                };
                                nat_sub(a33, a34)
                            }), 46u8)?)
                        } else {
                            Ok(false)
                        }
                    }
                }
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f6(v0: Fn0, p47: u64, p48: u64) -> R<(u64, bool)> {
    let m46 = (p47, p48);
    {
        let mut m46 = m46;
        loop {
            let step = {
                let (v1, v2) = m46;
                let m43 = v1;
                if m43 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m43 - 1;
                    let m45 = {
                        let c44: Fn0 = v0.clone();
                        c44.apply(v2)?
                    };
                    match m45 {
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
                Ok(next) => { m46 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f8(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a51 = {
            let a49 = v4;
            let a50 = v1;
            nat_add(a49, a50)?
        };
        let a52 = v3;
        if nat_lt(a51, a52) {
            Ok(Some({
                let a53 = v4;
                let a54 = 1u64;
                nat_add(a53, a54)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f8(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m59 = {
        let a56 = v0;
        let a57 = v2;
        let a58 = {
            let a55 = v1.clone();
            length_bytes(a55)
        };
        slice_bytes(a56, a57, a58)
    };
    match m59 {
        None => {
            false
        }
        Some(v3) => {
            let a60 = v3;
            let a61 = v1;
            equal(a60, a61)
        }
    }
}

pub fn f9(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a62 = v0.clone();
        length_bytes(a62)
    };
    let v3: Adt1 = {
        let (h63, _) = f14(Fn1::F15(v1, v2, v0), {
            let a64 = v2;
            let a65 = 1u64;
            nat_add(a64, a65)?
        }, Adt1::C0(0u64, true, true, true))?;
        h63
    };
    if match v3.clone() {
        Adt1::C0(_, _, _, h66) => {
            h66
        }
    } {
        Ok(!match v3 {
            Adt1::C0(_, h67, _, _) => {
                h67
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f10(v0: List<Bytes>) -> R<bool> {
    f16(Fn2::F17, true, v0)
}

pub fn f11(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a68 = v0.clone();
        length_bytes(a68)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h69, _) = f18(Fn3::F19(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h69
    };
    let v4: u64 = {
        let (h70, _) = {
            let (_, h71) = v3.clone();
            h71
        };
        h70
    };
    Ok(f20(List::cons(f3(v0, v4, {
        let a72 = v2;
        let a73 = v4;
        nat_sub(a72, a73)
    }), {
        let (_, h74) = {
            let (_, h75) = v3;
            h75
        };
        h74
    })))
}

pub fn f12(v0: Bytes) -> R<bool> {
    let v1: List<Bytes> = f11(v0, 46u8)?;
    let a76 = f21(v1.clone())?;
    let a77 = 3u64;
    if nat_eq(a76, a77) {
        f22(v1)
    } else {
        Ok(false)
    }
}

pub fn f13(v0: List<Bytes>) -> R<bool> {
    f16(Fn2::F23, true, v0)
}

pub fn f14(v0: Fn1, p82: u64, p83: Adt1) -> R<(Adt1, bool)> {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v1, v2) = m81;
                let m78 = v1;
                if m78 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m78 - 1;
                    let m80 = {
                        let c79: Fn1 = v0.clone();
                        c79.apply(v2.clone())?
                    };
                    match m80 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m81 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: bool, v1: u64, v2: Bytes, v3: Adt1) -> R<Option<Adt1>> {
    f24(v2, v1, v0, v3)
}

pub fn f16(v0: Fn2, p87: bool, p88: List<Bytes>) -> R<bool> {
    let m86 = (p87, p88);
    {
        let mut m86 = m86;
        loop {
            let step = {
                let (v1, v2) = m86;
                let m84 = v2;
                match m84.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c85: Fn2 = v0.clone();
                            c85.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m86 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        f25(v1)
    } else {
        Ok(false)
    }
}

pub fn f18(v0: Fn3, p93: u64, p94: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m92 = (p93, p94);
    {
        let mut m92 = m92;
        loop {
            let step = {
                let (v1, v2) = m92;
                let m89 = v1;
                if m89 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m89 - 1;
                    let m91 = {
                        let c90: Fn3 = v0.clone();
                        c90.apply(v2.clone())?
                    };
                    match m91 {
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
                Ok(next) => { m92 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f26(v1, v0, v2)
}

pub fn f20(v0: List<Bytes>) -> List<Bytes> {
    f27(Fn4::F28, List::<Bytes>::nil(), v0)
}

pub fn f21(v0: List<Bytes>) -> R<u64> {
    f29(Fn5::F30, 0u64, v0)
}

pub fn f22(v0: List<Bytes>) -> R<bool> {
    f16(Fn2::F31, true, v0)
}

pub fn f23(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        f32(v1)
    } else {
        Ok(false)
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: bool, v3: Adt1) -> R<Option<Adt1>> {
    if match v3.clone() {
        Adt1::C0(_, _, _, h95) => {
            h95
        }
    } {
        let a97 = match v3.clone() {
            Adt1::C0(h96, _, _, _) => {
                h96
            }
        };
        let a98 = v1;
        if nat_lt(a97, a98) {
            let v4: u64 = f33(v0, match v3.clone() {
                Adt1::C0(h99, _, _, _) => {
                    h99
                }
            })?;
            let a100 = v4;
            let a101 = 46u64;
            if nat_eq(a100, a101) {
                if match v3.clone() {
                    Adt1::C0(_, h102, _, _) => {
                        h102
                    }
                } {
                    Ok(f34(v3))
                } else {
                    f35(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt1::C0(_, h103, _, _) => {
                        h103
                    }
                } {
                    if f36(v4) {
                        f35(v3.clone(), false, match v3 {
                            Adt1::C0(_, _, h104, _) => {
                                h104
                            }
                        })
                    } else {
                        Ok(f34(v3))
                    }
                } else {
                    if if f36(v4) {
                        true
                    } else {
                        f37(v4)
                    } {
                        f35(v3.clone(), false, match v3 {
                            Adt1::C0(_, _, h105, _) => {
                                h105
                            }
                        })
                    } else {
                        let a106 = v4;
                        let a107 = 45u64;
                        if if nat_eq(a106, a107) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt1::C0(_, _, h108, _) => {
                                        h108
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f35(v3.clone(), false, match v3 {
                                Adt1::C0(_, _, h109, _) => {
                                    h109
                                }
                            })
                        } else {
                            Ok(f34(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt1>)
        }
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f25(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a110 = v0.clone();
        length_bytes(a110)
    };
    let a111 = v1;
    let a112 = 0u64;
    if nat_eq(a111, a112) {
        Ok(false)
    } else {
        let a113 = f38(v0)?;
        let a114 = v1;
        Ok(nat_eq(a113, a114))
    }
}

pub fn f26(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h115, _) = v2.clone();
        h115
    };
    let v4: u64 = {
        let (h116, _) = {
            let (_, h117) = v2.clone();
            h117
        };
        h116
    };
    let v5: List<Bytes> = {
        let (_, h118) = {
            let (_, h119) = v2;
            h119
        };
        h118
    };
    let m120 = f39(v0.clone(), v3);
    match m120 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a121 = v6;
            let a122 = v1;
            if equal(a121, a122) {
                Ok(Some(({
                    let a123 = v3;
                    let a124 = 1u64;
                    nat_add(a123, a124)?
                }, ({
                    let a125 = v3;
                    let a126 = 1u64;
                    nat_add(a125, a126)?
                }, List::cons(f3(v0, v4, {
                    let a127 = v3;
                    let a128 = v4;
                    nat_sub(a127, a128)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a129 = v3;
                    let a130 = 1u64;
                    nat_add(a129, a130)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f27(v0: Fn4, p134: List<Bytes>, p135: List<Bytes>) -> List<Bytes> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c132: Fn4 = v0.clone();
                            c132.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f29(v0: Fn5, p139: u64, p140: List<Bytes>) -> R<u64> {
    let m138 = (p139, p140);
    {
        let mut m138 = m138;
        loop {
            let step = {
                let (v1, v2) = m138;
                let m136 = v2;
                match m136.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c137: Fn5 = v0.clone();
                            c137.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: u64, _: Bytes) -> R<u64> {
    let a141 = v0;
    let a142 = 1u64;
    nat_add(a141, a142)
}

pub fn f31(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        f40(v1)
    } else {
        Ok(false)
    }
}

pub fn f32(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a143 = v0.clone();
        length_bytes(a143)
    };
    let a144 = v1;
    let a145 = 0u64;
    if nat_eq(a144, a145) {
        Ok(false)
    } else {
        let a146 = f38(v0.clone())?;
        let a147 = v1;
        if nat_eq(a146, a147) {
            if f41(v0.clone())? {
                f40(v0)
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f33(v0: Bytes, v1: u64) -> R<u64> {
    let m148 = f39(v0, v1);
    match m148 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a193 = {
                let a169 = {
                    let a157 = {
                        let a151 = {
                            let a149 = v3;
                            let a150 = 128u8;
                            fixed_u8::bit_and(a149, a150)
                        };
                        let a152 = 128u8;
                        if equal(a151, a152) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a158 = {
                        let a155 = {
                            let a153 = v3;
                            let a154 = 64u8;
                            fixed_u8::bit_and(a153, a154)
                        };
                        let a156 = 64u8;
                        if equal(a155, a156) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a157, a158)?
                };
                let a170 = {
                    let a167 = {
                        let a161 = {
                            let a159 = v3;
                            let a160 = 32u8;
                            fixed_u8::bit_and(a159, a160)
                        };
                        let a162 = 32u8;
                        if equal(a161, a162) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a168 = {
                        let a165 = {
                            let a163 = v3;
                            let a164 = 16u8;
                            fixed_u8::bit_and(a163, a164)
                        };
                        let a166 = 16u8;
                        if equal(a165, a166) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a167, a168)?
                };
                nat_add(a169, a170)?
            };
            let a194 = {
                let a191 = {
                    let a179 = {
                        let a173 = {
                            let a171 = v3;
                            let a172 = 8u8;
                            fixed_u8::bit_and(a171, a172)
                        };
                        let a174 = 8u8;
                        if equal(a173, a174) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a180 = {
                        let a177 = {
                            let a175 = v3;
                            let a176 = 4u8;
                            fixed_u8::bit_and(a175, a176)
                        };
                        let a178 = 4u8;
                        if equal(a177, a178) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a179, a180)?
                };
                let a192 = {
                    let a189 = {
                        let a183 = {
                            let a181 = v3;
                            let a182 = 2u8;
                            fixed_u8::bit_and(a181, a182)
                        };
                        let a184 = 2u8;
                        if equal(a183, a184) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a190 = {
                        let a187 = {
                            let a185 = v3;
                            let a186 = 1u8;
                            fixed_u8::bit_and(a185, a186)
                        };
                        let a188 = 1u8;
                        if equal(a187, a188) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a189, a190)?
                };
                nat_add(a191, a192)?
            };
            nat_add(a193, a194)
        }
    }
}

pub fn f34(v0: Adt1) -> Option<Adt1> {
    Some(Adt1::C0(match v0.clone() {
        Adt1::C0(h195, _, _, _) => {
            h195
        }
    }, match v0.clone() {
        Adt1::C0(_, h196, _, _) => {
            h196
        }
    }, match v0 {
        Adt1::C0(_, _, h197, _) => {
            h197
        }
    }, false))
}

pub fn f35(v0: Adt1, v1: bool, v2: bool) -> R<Option<Adt1>> {
    Ok(Some(Adt1::C0({
        let a199 = match v0 {
            Adt1::C0(h198, _, _, _) => {
                h198
            }
        };
        let a200 = 1u64;
        nat_add(a199, a200)?
    }, v1, v2, true)))
}

pub fn f36(v0: u64) -> bool {
    let a201 = v0;
    let a202 = 97u64;
    if nat_lt(a201, a202) {
        false
    } else {
        let a203 = v0;
        let a204 = 122u64;
        nat_le(a203, a204)
    }
}

pub fn f37(v0: u64) -> bool {
    let a205 = v0;
    let a206 = 48u64;
    if nat_lt(a205, a206) {
        false
    } else {
        let a207 = v0;
        let a208 = 57u64;
        nat_le(a207, a208)
    }
}

pub fn f38(v0: Bytes) -> R<u64> {
    let (h209, _) = f6(Fn0::F42(v0.clone()), {
        let a210 = v0;
        length_bytes(a210)
    }, 0u64)?;
    Ok(h209)
}

pub fn f39(v0: Bytes, v1: u64) -> Option<u8> {
    let a211 = v0;
    let a212 = v1;
    index_bytes(a211, a212)
}

pub fn f40(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a213 = v0.clone();
        length_bytes(a213)
    };
    let a214 = 0u64;
    let a215 = v1;
    if nat_lt(a214, a215) {
        let a216 = f43(v0.clone())?;
        let a217 = v1;
        if nat_eq(a216, a217) {
            let a218 = 1u64;
            let a219 = v1;
            if nat_lt(a218, a219) {
                let m222 = {
                    let a220 = v0;
                    let a221 = 0u64;
                    index_bytes(a220, a221)
                };
                match m222 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a223 = v2;
                        let a224 = 48u8;
                        Ok(!equal(a223, a224))
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

pub fn f41(v0: Bytes) -> R<bool> {
    let a226 = f43(v0.clone())?;
    let a227 = {
        let a225 = v0;
        length_bytes(a225)
    };
    Ok(nat_eq(a226, a227))
}

pub fn f42(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m228 = f39(v0, v1);
    match m228 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f44(v2)? {
                Ok(Some({
                    let a229 = v1;
                    let a230 = 1u64;
                    nat_add(a229, a230)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f43(v0: Bytes) -> R<u64> {
    let (h231, _) = f6(Fn0::F45(v0.clone()), {
        let a232 = v0;
        length_bytes(a232)
    }, 0u64)?;
    Ok(h231)
}

pub fn f44(v0: u8) -> R<bool> {
    if f46(v0)? {
        Ok(true)
    } else {
        if f47(v0)? {
            Ok(true)
        } else {
            let a233 = v0;
            let a234 = 45u8;
            Ok(equal(a233, a234))
        }
    }
}

pub fn f45(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m237 = {
        let a235 = v0;
        let a236 = v1;
        index_bytes(a235, a236)
    };
    match m237 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f46(v2)? {
                Ok(Some({
                    let a238 = v1;
                    let a239 = 1u64;
                    nat_add(a238, a239)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f46(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a284 = {
            let a260 = {
                let a248 = {
                    let a242 = {
                        let a240 = v1;
                        let a241 = 128u8;
                        fixed_u8::bit_and(a240, a241)
                    };
                    let a243 = 128u8;
                    if equal(a242, a243) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a249 = {
                    let a246 = {
                        let a244 = v1;
                        let a245 = 64u8;
                        fixed_u8::bit_and(a244, a245)
                    };
                    let a247 = 64u8;
                    if equal(a246, a247) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a248, a249)?
            };
            let a261 = {
                let a258 = {
                    let a252 = {
                        let a250 = v1;
                        let a251 = 32u8;
                        fixed_u8::bit_and(a250, a251)
                    };
                    let a253 = 32u8;
                    if equal(a252, a253) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a259 = {
                    let a256 = {
                        let a254 = v1;
                        let a255 = 16u8;
                        fixed_u8::bit_and(a254, a255)
                    };
                    let a257 = 16u8;
                    if equal(a256, a257) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a258, a259)?
            };
            nat_add(a260, a261)?
        };
        let a285 = {
            let a282 = {
                let a270 = {
                    let a264 = {
                        let a262 = v1;
                        let a263 = 8u8;
                        fixed_u8::bit_and(a262, a263)
                    };
                    let a265 = 8u8;
                    if equal(a264, a265) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a271 = {
                    let a268 = {
                        let a266 = v1;
                        let a267 = 4u8;
                        fixed_u8::bit_and(a266, a267)
                    };
                    let a269 = 4u8;
                    if equal(a268, a269) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a270, a271)?
            };
            let a283 = {
                let a280 = {
                    let a274 = {
                        let a272 = v1;
                        let a273 = 2u8;
                        fixed_u8::bit_and(a272, a273)
                    };
                    let a275 = 2u8;
                    if equal(a274, a275) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a281 = {
                    let a278 = {
                        let a276 = v1;
                        let a277 = 1u8;
                        fixed_u8::bit_and(a276, a277)
                    };
                    let a279 = 1u8;
                    if equal(a278, a279) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a280, a281)?
            };
            nat_add(a282, a283)?
        };
        nat_add(a284, a285)?
    };
    let a286 = 48u64;
    let a287 = v2;
    if nat_le(a286, a287) {
        let a288 = v2;
        let a289 = 57u64;
        Ok(nat_le(a288, a289))
    } else {
        Ok(false)
    }
}

pub fn f47(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a334 = {
            let a310 = {
                let a298 = {
                    let a292 = {
                        let a290 = v1;
                        let a291 = 128u8;
                        fixed_u8::bit_and(a290, a291)
                    };
                    let a293 = 128u8;
                    if equal(a292, a293) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a299 = {
                    let a296 = {
                        let a294 = v1;
                        let a295 = 64u8;
                        fixed_u8::bit_and(a294, a295)
                    };
                    let a297 = 64u8;
                    if equal(a296, a297) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a298, a299)?
            };
            let a311 = {
                let a308 = {
                    let a302 = {
                        let a300 = v1;
                        let a301 = 32u8;
                        fixed_u8::bit_and(a300, a301)
                    };
                    let a303 = 32u8;
                    if equal(a302, a303) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a309 = {
                    let a306 = {
                        let a304 = v1;
                        let a305 = 16u8;
                        fixed_u8::bit_and(a304, a305)
                    };
                    let a307 = 16u8;
                    if equal(a306, a307) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a308, a309)?
            };
            nat_add(a310, a311)?
        };
        let a335 = {
            let a332 = {
                let a320 = {
                    let a314 = {
                        let a312 = v1;
                        let a313 = 8u8;
                        fixed_u8::bit_and(a312, a313)
                    };
                    let a315 = 8u8;
                    if equal(a314, a315) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a321 = {
                    let a318 = {
                        let a316 = v1;
                        let a317 = 4u8;
                        fixed_u8::bit_and(a316, a317)
                    };
                    let a319 = 4u8;
                    if equal(a318, a319) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a320, a321)?
            };
            let a333 = {
                let a330 = {
                    let a324 = {
                        let a322 = v1;
                        let a323 = 2u8;
                        fixed_u8::bit_and(a322, a323)
                    };
                    let a325 = 2u8;
                    if equal(a324, a325) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a331 = {
                    let a328 = {
                        let a326 = v1;
                        let a327 = 1u8;
                        fixed_u8::bit_and(a326, a327)
                    };
                    let a329 = 1u8;
                    if equal(a328, a329) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a330, a331)?
            };
            nat_add(a332, a333)?
        };
        nat_add(a334, a335)?
    };
    let a336 = 65u64;
    let a337 = v2;
    if if nat_le(a336, a337) {
        let a338 = v2;
        let a339 = 90u64;
        nat_le(a338, a339)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a340 = 97u64;
        let a341 = v2;
        if nat_le(a340, a341) {
            let a342 = v2;
            let a343 = 122u64;
            Ok(nat_le(a342, a343))
        } else {
            Ok(false)
        }
    }
}

pub fn run(p0: Bytes) -> R<Option<Adt0>> {
    f0(p0)
}
