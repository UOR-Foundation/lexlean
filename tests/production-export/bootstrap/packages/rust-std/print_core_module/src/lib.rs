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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt8>, std::rc::Rc<Adt8>),
    C3(u64, std::rc::Rc<Adt8>),
    C4,
    C5,
    C6(std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Adt10>, List<u64>, List<u64>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64, u64, u64, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(std::rc::Rc<Adt12>),
    C1(std::rc::Rc<Adt12>, List<Adt12>),
    C2(std::rc::Rc<Adt12>),
    C3(std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C3(u64, std::rc::Rc<Adt12>),
    C4,
    C5(std::rc::Rc<Adt12>),
    C6(u64),
    C7(Bytes, Bytes),
    C8(Bytes, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<Bytes>, u64, List<Adt14>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, bool, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Adt10>, List<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(std::rc::Rc<Adt12>, List<Adt12>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, bool),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, bool),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, bool, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, bool, List<Adt14>, List<Adt14>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: Result<Bytes, Bytes>, p1: Adt6) -> R<Result<Bytes, Bytes>> {
        match self {
            Fn0::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt8>, p1: Bytes) -> R<List<Adt8>> {
        match self {
            Fn1::F15 => f15(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Option<Bytes>) -> Option<Bytes> {
        match self {
            Fn2::F17 => f17(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F19,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Option<Bytes>), p1: Adt1) -> R<(u64, Option<Bytes>)> {
        match self {
            Fn3::F19 => f19(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F21(u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (bool, Option<u64>), p1: u64) -> (bool, Option<u64>) {
        match self {
            Fn4::F21(k0) => f21(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F23(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> Option<Bytes> {
        match self {
            Fn5::F23(k0) => f23(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F26,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt9, p1: Adt1) -> R<Adt9> {
        match self {
            Fn6::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F29,
}

impl Fn7 {
    pub fn apply(&self, p0: Bytes, p1: Adt10) -> R<Bytes> {
        match self {
            Fn7::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F35,
}

impl Fn8 {
    pub fn apply(&self, p0: (bool, Adt8), p1: Adt8) -> (bool, Adt8) {
        match self {
            Fn8::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F38(u64),
    F155(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: u64) -> Option<Bytes> {
        match self {
            Fn9::F38(k0) => f38(*k0, p0, p1),
            Fn9::F155(k0) => f155(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F42(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn10::F42(k0) => f42(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F47,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt6) -> R<List<Bytes>> {
        match self {
            Fn11::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F56,
    F79,
}

impl Fn12 {
    pub fn apply(&self, p0: List<u64>, p1: Adt6) -> List<u64> {
        match self {
            Fn12::F56 => f56(p0, p1),
            Fn12::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F58,
}

impl Fn13 {
    pub fn apply(&self, p0: List<u64>, p1: Adt1) -> List<u64> {
        match self {
            Fn13::F58 => f58(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F60,
}

impl Fn14 {
    pub fn apply(&self, p0: List<u64>, p1: (Bytes, u64)) -> List<u64> {
        match self {
            Fn14::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F64,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, u64)>, p1: u64) -> List<(Bytes, u64)> {
        match self {
            Fn15::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F70,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Adt10>, p1: Adt10) -> List<Adt10> {
        match self {
            Fn16::F70 => f70(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F72(u64, u64),
}

impl Fn17 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn17::F72(k0, k1) => f72(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F81(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
        match self {
            Fn18::F81(k0) => f81(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F84,
}

impl Fn19 {
    pub fn apply(&self, p0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
        match self {
            Fn19::F84 => f84(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F86(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Bytes) -> (bool, Bytes) {
        match self {
            Fn20::F86(k0) => f86(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F89,
}

impl Fn21 {
    pub fn apply(&self, p0: (Adt6, List<(Bytes, Adt6)>)) -> Option<(Adt6, List<(Bytes, Adt6)>)> {
        match self {
            Fn21::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F91,
}

impl Fn22 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Adt6)) -> Adt6 {
        match self {
            Fn22::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F102,
}

impl Fn23 {
    pub fn apply(&self, p0: Adt15, p1: u64) -> Adt15 {
        match self {
            Fn23::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F108,
    F109,
    F154,
}

impl Fn24 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn24::F108 => f108(p0, p1),
            Fn24::F109 => f109(p0, p1),
            Fn24::F154 => f154(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F111,
}

impl Fn25 {
    pub fn apply(&self, p0: List<(Bytes, u64)>, p1: (Bytes, u64)) -> List<(Bytes, u64)> {
        match self {
            Fn25::F111 => f111(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F114,
}

impl Fn26 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>, p1: (Bytes, u64)) -> List<List<(Bytes, u64)>> {
        match self {
            Fn26::F114 => f114(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F116,
}

impl Fn27 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
        match self {
            Fn27::F116 => f116(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F121,
}

impl Fn28 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn28::F121 => f121(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F133(u64, Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: Adt16) -> R<Option<Adt16>> {
        match self {
            Fn29::F133(k0, k1) => f133(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F139,
}

impl Fn30 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn30::F139 => f139(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F145,
}

impl Fn31 {
    pub fn apply(&self, p0: List<Adt8>, p1: Adt8) -> List<Adt8> {
        match self {
            Fn31::F145 => f145(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F147(Option<Bytes>),
    F285,
}

impl Fn32 {
    pub fn apply(&self, p0: List<(Bytes, Adt8)>, p1: (Bytes, Adt8)) -> R<List<(Bytes, Adt8)>> {
        match self {
            Fn32::F147(k0) => f147(k0.clone(), p0, p1),
            Fn32::F285 => Ok(f285(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F149,
}

impl Fn33 {
    pub fn apply(&self, p0: List<Adt8>, p1: (Bytes, Adt8)) -> List<Adt8> {
        match self {
            Fn33::F149 => f149(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F152(u64),
}

impl Fn34 {
    pub fn apply(&self, p0: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
        match self {
            Fn34::F152(k0) => f152(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F167,
}

impl Fn35 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn35::F167 => f167(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F170,
}

impl Fn36 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt4) -> R<List<Bytes>> {
        match self {
            Fn36::F170 => f170(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F172,
}

impl Fn37 {
    pub fn apply(&self, p0: List<List<(Bytes, u64)>>, p1: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
        match self {
            Fn37::F172 => f172(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F175,
}

impl Fn38 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn38::F175 => f175(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F177(Bytes, u64),
}

impl Fn39 {
    pub fn apply(&self, p0: Adt18, p1: Bytes) -> R<Adt18> {
        match self {
            Fn39::F177(k0, k1) => f177(k0.clone(), *k1, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F179(Bytes),
}

impl Fn40 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn40::F179(k0) => f179(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn41 {
    F181,
    F182,
}

impl Fn41 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn41::F181 => f181(p0, p1),
            Fn41::F182 => f182(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn42 {
    F185,
}

impl Fn42 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn42::F185 => f185(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn43 {
    F187,
    F219,
}

impl Fn43 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn43::F187 => f187(p0, p1),
            Fn43::F219 => f219(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn44 {
    F192(Bytes),
    F238(Bytes),
    F243(Bytes),
    F257(u64, Bytes),
}

impl Fn44 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn44::F192(k0) => f192(k0.clone(), p0),
            Fn44::F238(k0) => f238(k0.clone(), p0),
            Fn44::F243(k0) => f243(k0.clone(), p0),
            Fn44::F257(k0, k1) => f257(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn45 {
    F194(Bytes),
}

impl Fn45 {
    pub fn apply(&self, p0: Adt19) -> R<Option<Adt19>> {
        match self {
            Fn45::F194(k0) => f194(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn46 {
    F198(u64, Bytes),
}

impl Fn46 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn46::F198(k0, k1) => f198(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn47 {
    F207,
}

impl Fn47 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn47::F207 => f207(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn48 {
    F209,
}

impl Fn48 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
        match self {
            Fn48::F209 => f209(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn49 {
    F214,
}

impl Fn49 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt2) -> R<List<Bytes>> {
        match self {
            Fn49::F214 => f214(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn50 {
    F241,
}

impl Fn50 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt8)>>, p1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
        match self {
            Fn50::F241 => f241(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn51 {
    F246,
}

impl Fn51 {
    pub fn apply(&self, p0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
        match self {
            Fn51::F246 => f246(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn52 {
    F248(Bytes),
}

impl Fn52 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn52::F248(k0) => f248(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn53 {
    F251,
}

impl Fn53 {
    pub fn apply(&self, p0: Adt21) -> R<Option<Adt21>> {
        match self {
            Fn53::F251 => f251(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn54 {
    F253,
}

impl Fn54 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn54::F253 => f253(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn55 {
    F263(u64),
}

impl Fn55 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn55::F263(k0) => f263(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn56 {
    F271,
}

impl Fn56 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
        match self {
            Fn56::F271 => f271(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn57 {
    F278,
}

impl Fn57 {
    pub fn apply(&self, p0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
        match self {
            Fn57::F278 => f278(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn58 {
    F283,
}

impl Fn58 {
    pub fn apply(&self, p0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
        match self {
            Fn58::F283 => f283(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<Bytes, Adt7>> {
    if f1(match v0.clone() {
        Adt0::C0(h1, _, _, _, _) => {
            h1
        }
    }, f2(Str::lit("lexlean/core-module/2"))) {
        let v1: u64 = {
            let a3 = match v0.clone() {
                Adt0::C0(_, _, h2, _, _) => {
                    h2
                }
            };
            length_list(a3)
        };
        let m7 = f3(List::cons(f4(match v0.clone() {
            Adt0::C0(_, _, h4, _, _) => {
                h4
            }
        })?, List::cons(f5(match v0.clone() {
            Adt0::C0(_, _, _, h5, _) => {
                h5
            }
        }, v1), List::cons(f6(match v0.clone() {
            Adt0::C0(_, _, _, _, h6) => {
                h6
            }
        }, v1), List::<Option<Bytes>>::nil()))));
        match m7 {
            None => {
                let v3: Result<Bytes, Bytes> = f8(Fn0::F9, Ok::<Bytes, Bytes>(f2(Str::lit(""))), match v0.clone() {
                    Adt0::C0(_, _, _, _, h8) => {
                        h8
                    }
                })?;
                let m9 = v3;
                match m9 {
                    Ok(v5) => {
                        let m10 = f10(v0.clone())?;
                        match m10 {
                            Ok(v7) => {
                                Ok(Ok::<Bytes, Adt7>(f11(f11(f11(f11(f12(Adt8::C2(std::rc::Rc::new(Adt8::C1(f2(Str::lit("imports ")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(f13(f2(Str::lit("[")), f2(Str::lit("]")), f14(Fn1::F15, List::<Adt8>::nil(), match v0 {
                                    Adt0::C0(_, h11, _, _, _) => {
                                        h11
                                    }
                                })?)), std::rc::Rc::new(Adt8::C0)))))?, f2(Str::lit("\u{a}\u{a}"))), v5), f2(Str::lit("\u{a}"))), v7)))
                            }
                            Err(v6) => {
                                Ok(f7(v6))
                            }
                        }
                    }
                    Err(v4) => {
                        Ok(f7(v4))
                    }
                }
            }
            Some(v2) => {
                Ok(f7(v2))
            }
        }
    } else {
        Ok(f7(f11(f11(f2(Str::lit("the notation names the schema `lexlean/core-module/2`, not `")), match v0 {
            Adt0::C0(h12, _, _, _, _) => {
                h12
            }
        }), f2(Str::lit("`")))))
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> bool {
    let a13 = v0;
    let a14 = v1;
    equal(a13, a14)
}

pub fn f2(v0: Str) -> Bytes {
    let a15 = v0;
    utf8_encode(a15)
}

pub fn f3(v0: List<Option<Bytes>>) -> Option<Bytes> {
    f16(Fn2::F17, None::<Bytes>, v0)
}

pub fn f4(v0: List<Adt1>) -> R<Option<Bytes>> {
    let (_, h16) = f18(Fn3::F19, (0u64, None::<Bytes>), v0)?;
    Ok(h16)
}

pub fn f5(v0: List<u64>, v1: u64) -> Option<Bytes> {
    let v2: (bool, Option<u64>) = f20(Fn4::F21(v1), (true, None::<u64>), v0);
    let (h17, _) = v2;
    if h17 {
        None::<Bytes>
    } else {
        Some(f2(Str::lit("the proof nodes are not strictly increasing nodes of the module")))
    }
}

pub fn f6(v0: List<Adt6>, v1: u64) -> Option<Bytes> {
    f22(Fn5::F23(v1), None::<Bytes>, v0)
}

pub fn f7(v0: Bytes) -> Result<Bytes, Adt7> {
    Err::<Bytes, Adt7>(Adt7::C0(0u64, v0))
}

pub fn f8(v0: Fn0, p21: Result<Bytes, Bytes>, p22: List<Adt6>) -> R<Result<Bytes, Bytes>> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m18 = v2;
                match m18.uncons() {
                    None => {
                        Err::<(Result<Bytes, Bytes>, List<Adt6>), R<Result<Bytes, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Bytes, Bytes>, List<Adt6>), R<Result<Bytes, Bytes>>>(({
                            let c19: Fn0 = v0.clone();
                            c19.apply(v1, v3)?
                        }, v4))
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

pub fn f9(v0: Result<Bytes, Bytes>, v1: Adt6) -> R<Result<Bytes, Bytes>> {
    let m23 = v0.clone();
    match m23 {
        Ok(v2) => {
            let m24 = f24(v1)?;
            match m24 {
                Ok(v3) => {
                    Ok(Ok::<Bytes, Bytes>(f11(f11(v2, v3), f2(Str::lit("\u{a}")))))
                }
                Err(v4) => {
                    Ok(Err::<Bytes, Bytes>(v4))
                }
            }
        }
        Err(_) => {
            Ok(v0)
        }
    }
}

pub fn f10(v0: Adt0) -> R<Result<Bytes, Bytes>> {
    let v1: Adt9 = f25(Fn6::F26, Adt9::C0(0u64, List::<Adt10>::nil(), f27(match v0.clone() {
        Adt0::C0(_, _, h25, _, _) => {
            h25
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h26) => {
            h26
        }
    })?, match v0.clone() {
        Adt0::C0(_, _, _, h27, _) => {
            h27
        }
    }, None::<Bytes>), match v0 {
        Adt0::C0(_, _, h28, _, _) => {
            h28
        }
    })?;
    let m30 = match v1.clone() {
        Adt9::C0(_, _, _, _, h29) => {
            h29
        }
    };
    match m30 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f28(Fn7::F29, f2(Str::lit("")), f30(match v1 {
                Adt9::C0(_, h31, _, _, _) => {
                    h31
                }
            }))?))
        }
        Some(v2) => {
            Ok(Err::<Bytes, Bytes>(v2))
        }
    }
}

pub fn f11(v0: Bytes, v1: Bytes) -> Bytes {
    let a32 = v0;
    let a33 = v1;
    append_bytes(a32, a33)
}

pub fn f12(v0: Adt8) -> R<Bytes> {
    f31(v0.clone(), 100u64, f32(v0)?)
}

pub fn f13(v0: Bytes, v1: Bytes, v2: List<Adt8>) -> Adt8 {
    let m34 = v2.clone();
    match m34.uncons() {
        None => {
            Adt8::C1(f33(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt8) = f34(Fn8::F35, (true, Adt8::C0), v2);
            Adt8::C6(std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(v0)), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C3(2u64, std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C5), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new({
                let (_, h35) = v5;
                h35
            }), std::rc::Rc::new(Adt8::C0))))))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C5), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(v1)), std::rc::Rc::new(Adt8::C0))))))))))
        }
    }
}

pub fn f14(v0: Fn1, p39: List<Adt8>, p40: List<Bytes>) -> R<List<Adt8>> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Bytes>), R<List<Adt8>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Bytes>), R<List<Adt8>>>(({
                            let c37: Fn1 = v0.clone();
                            c37.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Adt8>, v1: Bytes) -> R<List<Adt8>> {
    Ok(List::cons(Adt8::C1(f36(v1)?), v0))
}

pub fn f16(v0: Fn2, p44: Option<Bytes>, p45: List<Option<Bytes>>) -> Option<Bytes> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m41 = v2;
                match m41.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Option<Bytes>>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Option<Bytes>>), Option<Bytes>>(({
                            let c42: Fn2 = v0.clone();
                            c42.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m46 = v0.clone();
    match m46 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f18(v0: Fn3, p50: (u64, Option<Bytes>), p51: List<Adt1>) -> R<(u64, Option<Bytes>)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m47 = v2;
                match m47.uncons() {
                    None => {
                        Err::<((u64, Option<Bytes>), List<Adt1>), R<(u64, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<Bytes>), List<Adt1>), R<(u64, Option<Bytes>)>>(({
                            let c48: Fn3 = v0.clone();
                            c48.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: (u64, Option<Bytes>), v1: Adt1) -> R<(u64, Option<Bytes>)> {
    let v2: u64 = {
        let (h52, _) = v0.clone();
        h52
    };
    Ok(({
        let a53 = v2;
        let a54 = 1u64;
        nat_add(a53, a54)?
    }, {
        let m56 = {
            let (_, h55) = v0.clone();
            h55
        };
        match m56 {
            None => {
                f37(Fn9::F38(v2), None::<Bytes>, f39(v1))
            }
            Some(_) => {
                let (_, h57) = v0;
                h57
            }
        }
    }))
}

pub fn f20(v0: Fn4, p61: (bool, Option<u64>), p62: List<u64>) -> (bool, Option<u64>) {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m58 = v2;
                match m58.uncons() {
                    None => {
                        Err::<((bool, Option<u64>), List<u64>), (bool, Option<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Option<u64>), List<u64>), (bool, Option<u64>)>(({
                            let c59: Fn4 = v0.clone();
                            c59.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: u64, v1: (bool, Option<u64>), v2: u64) -> (bool, Option<u64>) {
    (f40({
        let (h63, _) = v1;
        h63
    }, f40({
        let a64 = v2;
        let a65 = v0;
        nat_lt(a64, a65)
    }, {
        let m67 = {
            let (_, h66) = v1;
            h66
        };
        match m67 {
            None => {
                true
            }
            Some(v3) => {
                let a68 = v3;
                let a69 = v2;
                nat_lt(a68, a69)
            }
        }
    })), Some(v2))
}

pub fn f22(v0: Fn5, p73: Option<Bytes>, p74: List<Adt6>) -> Option<Bytes> {
    let m72 = (p73, p74);
    {
        let mut m72 = m72;
        loop {
            let step = {
                let (v1, v2) = m72;
                let m70 = v2;
                match m70.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), Option<Bytes>>(({
                            let c71: Fn5 = v0.clone();
                            c71.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m72 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: u64, v1: Option<Bytes>, v2: Adt6) -> Option<Bytes> {
    let m75 = v1.clone();
    match m75 {
        None => {
            if f41(Fn10::F42(v0), false, f43(v2.clone())) {
                Some(f11(f11(f2(Str::lit("`")), f44(f45(v2, f2(Str::lit("name"))))), f2(Str::lit("` has a root that is not a node of the module"))))
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f24(v0: Adt6) -> R<Result<Bytes, Bytes>> {
    let m76 = v0.clone();
    match m76 {
        Adt6::C0(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C1(_, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C2(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C3(_) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C4(r85) => {
            let v1 = (*r85).clone();
            let m77 = f45(v0.clone(), f2(Str::lit("kind")));
            match m77 {
                None => {
                    Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                }
                Some(v2) => {
                    let m78 = v2;
                    match m78 {
                        Adt6::C0(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C1(_, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C2(v3) => {
                            let v4: List<Bytes> = f46(Fn11::F47, List::<Bytes>::nil(), f48(f45(v0.clone(), f2(Str::lit("levels")))))?;
                            let v7: Bytes = f11(f11(f11(f11(f11(v3, f2(Str::lit(" "))), f36(f44(f45(v0.clone(), f2(Str::lit("name")))))?), {
                                let a80 = {
                                    let a79 = v4.clone();
                                    length_list(a79)
                                };
                                let a81 = 0u64;
                                if nat_eq(a80, a81) {
                                    f2(Str::lit(""))
                                } else {
                                    f11(f11(f2(Str::lit(".{")), f49(v4, f2(Str::lit(", ")))), f2(Str::lit("}")))
                                }
                            }), {
                                let m82 = f50(f45(v0.clone(), f2(Str::lit("type"))));
                                match m82 {
                                    None => {
                                        f2(Str::lit(""))
                                    }
                                    Some(v5) => {
                                        f11(f2(Str::lit(" : @")), f51(v5))
                                    }
                                }
                            }), {
                                let m83 = f50(f45(v0, f2(Str::lit("value"))));
                                match m83 {
                                    None => {
                                        f2(Str::lit(""))
                                    }
                                    Some(v6) => {
                                        f11(f2(Str::lit(" := @")), f51(v6))
                                    }
                                }
                            });
                            let v8: Adt6 = f52(v1);
                            Ok(Ok::<Bytes, Bytes>(f12({
                                let m84 = v8.clone();
                                match m84 {
                                    Adt6::C0(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C1(_, _) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C2(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C3(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C4(_) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C5 => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C6(_, _) => {
                                        Adt8::C1(v7)
                                    }
                                    Adt6::C7(_, _, _) => {
                                        Adt8::C2(std::rc::Rc::new(Adt8::C1(v7)), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(f2(Str::lit(" ")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(f53(Adt6::C4(std::rc::Rc::new(v8)))?), std::rc::Rc::new(Adt8::C0))))))
                                    }
                                }
                            })?))
                        }
                        Adt6::C3(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C4(_) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C5 => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C6(_, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                        Adt6::C7(_, _, _) => {
                            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration has a kind"))))
                        }
                    }
                }
            }
        }
        Adt6::C5 => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C6(_, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
        Adt6::C7(_, _, _) => {
            Ok(Err::<Bytes, Bytes>(f2(Str::lit("a declaration is an object"))))
        }
    }
}

pub fn f25(v0: Fn6, p89: Adt9, p90: List<Adt1>) -> R<Adt9> {
    let m88 = (p89, p90);
    {
        let mut m88 = m88;
        loop {
            let step = {
                let (v1, v2) = m88;
                let m86 = v2;
                match m86.uncons() {
                    None => {
                        Err::<(Adt9, List<Adt1>), R<Adt9>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt9, List<Adt1>), R<Adt9>>(({
                            let c87: Fn6 = v0.clone();
                            c87.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m88 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt9, v1: Adt1) -> R<Adt9> {
    f54(v0, v1)
}

pub fn f27(v0: List<Adt1>, v1: List<Adt6>) -> R<List<u64>> {
    let v2: List<u64> = f55(Fn12::F56, f57(Fn13::F58, List::<u64>::nil(), v0), v1);
    Ok(f59(Fn14::F60, List::<u64>::nil(), f61(f62(f63(Fn15::F64, List::<(Bytes, u64)>::nil(), v2))?)))
}

pub fn f28(v0: Fn7, p94: Bytes, p95: List<Adt10>) -> R<Bytes> {
    let m93 = (p94, p95);
    {
        let mut m93 = m93;
        loop {
            let step = {
                let (v1, v2) = m93;
                let m91 = v2;
                match m91.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt10>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt10>), R<Bytes>>(({
                            let c92: Fn7 = v0.clone();
                            c92.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m93 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Bytes, v1: Adt10) -> R<Bytes> {
    Ok(f11(v0, f11(f12(f65(f66(List::cons(f67(f11(f11(f2(Str::lit("@")), f51(match v1.clone() {
        Adt10::C0(h96, _, _, _, _) => {
            h96
        }
    })), f2(Str::lit(" := ")))), List::cons(Adt12::C3(2u64, std::rc::Rc::new(f68(match v1 {
        Adt10::C0(_, _, _, _, h97) => {
            (*h97).clone()
        }
    }))), List::<Adt12>::nil()))), List::<Bytes>::nil())?)?, f2(Str::lit("\u{a}")))))
}

pub fn f30(v0: List<Adt10>) -> List<Adt10> {
    f69(Fn16::F70, List::<Adt10>::nil(), v0)
}

pub fn f31(v0: Adt8, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt13 = {
        let (h98, _) = f71(Fn17::F72(v2, v1), {
            let a99 = v2;
            let a100 = 1u64;
            nat_add(a99, a100)?
        }, Adt13::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt14::C0(0u64, false, std::rc::Rc::new(v0)), List::<Adt14>::nil())))?;
        h98
    };
    Ok(f73(match v3 {
        Adt13::C0(h101, _, _) => {
            h101
        }
    }))
}

pub fn f32(v0: Adt8) -> R<u64> {
    let m102 = v0;
    match m102 {
        Adt8::C0 => {
            Ok(1u64)
        }
        Adt8::C1(_) => {
            Ok(1u64)
        }
        Adt8::C2(r111, r112) => {
            let v2 = (*r111).clone();
            let v3 = (*r112).clone();
            let v4: u64 = f32(v2)?;
            let v5: u64 = f32(v3)?;
            let a105 = {
                let a103 = 1u64;
                let a104 = v4;
                nat_add(a103, a104)?
            };
            let a106 = v5;
            nat_add(a105, a106)
        }
        Adt8::C3(_, r113) => {
            let v7 = (*r113).clone();
            let v8: u64 = f32(v7)?;
            let a107 = 1u64;
            let a108 = v8;
            nat_add(a107, a108)
        }
        Adt8::C4 => {
            Ok(1u64)
        }
        Adt8::C5 => {
            Ok(1u64)
        }
        Adt8::C6(r114) => {
            let v9 = (*r114).clone();
            let v10: u64 = f32(v9)?;
            let a109 = 1u64;
            let a110 = v10;
            nat_add(a109, a110)
        }
    }
}

pub fn f33(v0: Bytes, v1: Bytes) -> Bytes {
    let a115 = v0;
    let a116 = v1;
    append_bytes(a115, a116)
}

pub fn f34(v0: Fn8, p120: (bool, Adt8), p121: List<Adt8>) -> (bool, Adt8) {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v1, v2) = m119;
                let m117 = v2;
                match m117.uncons() {
                    None => {
                        Err::<((bool, Adt8), List<Adt8>), (bool, Adt8)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt8), List<Adt8>), (bool, Adt8)>(({
                            let c118: Fn8 = v0.clone();
                            c118.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: (bool, Adt8), v1: Adt8) -> (bool, Adt8) {
    f74(v0, v1)
}

pub fn f36(v0: Bytes) -> R<Bytes> {
    if f75(v0.clone())? {
        Ok(v0)
    } else {
        f76(v0)
    }
}

pub fn f37(v0: Fn9, p125: Option<Bytes>, p126: List<u64>) -> Option<Bytes> {
    let m124 = (p125, p126);
    {
        let mut m124 = m124;
        loop {
            let step = {
                let (v1, v2) = m124;
                let m122 = v2;
                match m122.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<u64>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<u64>), Option<Bytes>>(({
                            let c123: Fn9 = v0.clone();
                            c123.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: u64, v1: Option<Bytes>, v2: u64) -> Option<Bytes> {
    let m127 = v1.clone();
    match m127 {
        None => {
            let a128 = v0;
            let a129 = v2;
            if nat_le(a128, a129) {
                Some(f11(f11(f11(f11(f2(Str::lit("node ")), f51(v0)), f2(Str::lit(" points to the node "))), f51(v2)), f2(Str::lit(", which is not earlier"))))
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f39(v0: Adt1) -> List<u64> {
    let m130 = v0;
    match m130 {
        Adt1::C0(_) => {
            List::<u64>::nil()
        }
        Adt1::C1(_) => {
            List::<u64>::nil()
        }
        Adt1::C2(_, _) => {
            List::<u64>::nil()
        }
        Adt1::C3(v1, v2) => {
            List::cons(v1, List::cons(v2, List::<u64>::nil()))
        }
        Adt1::C4(_, _, v5, v6) => {
            List::cons(v5, List::cons(v6, List::<u64>::nil()))
        }
        Adt1::C5(_, _, v9, v10) => {
            List::cons(v9, List::cons(v10, List::<u64>::nil()))
        }
        Adt1::C6(_, v12, v13, v14, _) => {
            List::cons(v12, List::cons(v13, List::cons(v14, List::<u64>::nil())))
        }
        Adt1::C7(_) => {
            List::<u64>::nil()
        }
        Adt1::C8(_) => {
            List::<u64>::nil()
        }
        Adt1::C9(_, v17) => {
            List::cons(v17, List::<u64>::nil())
        }
        Adt1::C10(_, _, _, v21) => {
            List::cons(v21, List::<u64>::nil())
        }
    }
}

pub fn f40(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f41(v0: Fn10, p134: bool, p135: List<u64>) -> bool {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m131 = v2;
                match m131.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c132: Fn10 = v0.clone();
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

pub fn f42(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a136 = v0;
        let a137 = v2;
        nat_le(a136, a137)
    }
}

pub fn f43(v0: Adt6) -> List<u64> {
    let v1: Option<Adt6> = f45(v0.clone(), f2(Str::lit("eqn_info")));
    let v2: List<u64> = f77(f77(f77(f77(List::<u64>::nil(), f50(f45(v0.clone(), f2(Str::lit("type"))))), f50(f45(v0.clone(), f2(Str::lit("value"))))), f50(f78(v1.clone(), f2(Str::lit("type"))))), f50(f78(v1, f2(Str::lit("value")))));
    f55(Fn12::F79, v2, f48(f78(f78(f45(v0, f2(Str::lit("inductive"))), f2(Str::lit("structure"))), f2(Str::lit("fields")))))
}

pub fn f44(v0: Option<Adt6>) -> Bytes {
    let m138 = v0;
    match m138 {
        None => {
            f2(Str::lit(""))
        }
        Some(v1) => {
            let m139 = v1;
            match m139 {
                Adt6::C0(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C1(_, _) => {
                    f2(Str::lit(""))
                }
                Adt6::C2(v2) => {
                    v2
                }
                Adt6::C3(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C4(_) => {
                    f2(Str::lit(""))
                }
                Adt6::C5 => {
                    f2(Str::lit(""))
                }
                Adt6::C6(_, _) => {
                    f2(Str::lit(""))
                }
                Adt6::C7(_, _, _) => {
                    f2(Str::lit(""))
                }
            }
        }
    }
}

pub fn f45(v0: Adt6, v1: Bytes) -> Option<Adt6> {
    let m140 = v0;
    match m140 {
        Adt6::C0(_) => {
            None::<Adt6>
        }
        Adt6::C1(_, _) => {
            None::<Adt6>
        }
        Adt6::C2(_) => {
            None::<Adt6>
        }
        Adt6::C3(_) => {
            None::<Adt6>
        }
        Adt6::C4(r143) => {
            let v2 = (*r143).clone();
            let (_, h141) = {
                let (h142, _) = f80(Fn18::F81(v1), 1000000u64, (v2, None::<Adt6>));
                h142
            };
            h141
        }
        Adt6::C5 => {
            None::<Adt6>
        }
        Adt6::C6(_, _) => {
            None::<Adt6>
        }
        Adt6::C7(_, _, _) => {
            None::<Adt6>
        }
    }
}

pub fn f46(v0: Fn11, p147: List<Bytes>, p148: List<Adt6>) -> R<List<Bytes>> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v1, v2) = m146;
                let m144 = v2;
                match m144.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt6>), R<List<Bytes>>>(({
                            let c145: Fn11 = v0.clone();
                            c145.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<Bytes>, v1: Adt6) -> R<List<Bytes>> {
    let a149 = v0;
    let a150 = List::cons(f36(f44(Some(v1)))?, List::<Bytes>::nil());
    Ok(append_list(a149, a150))
}

pub fn f48(v0: Option<Adt6>) -> List<Adt6> {
    let m151 = v0;
    match m151 {
        None => {
            List::<Adt6>::nil()
        }
        Some(v1) => {
            let m152 = v1;
            match m152 {
                Adt6::C0(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C1(_, _) => {
                    List::<Adt6>::nil()
                }
                Adt6::C2(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C3(r155) => {
                    let v2 = (*r155).clone();
                    f82({
                        let (_, h153) = {
                            let (h154, _) = f83(Fn19::F84, 1000000u64, (v2, List::<Adt6>::nil()));
                            h154
                        };
                        h153
                    })
                }
                Adt6::C4(_) => {
                    List::<Adt6>::nil()
                }
                Adt6::C5 => {
                    List::<Adt6>::nil()
                }
                Adt6::C6(_, _) => {
                    List::<Adt6>::nil()
                }
                Adt6::C7(_, _, _) => {
                    List::<Adt6>::nil()
                }
            }
        }
    }
}

pub fn f49(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let (_, h156) = f85(Fn20::F86(v1), (true, f2(Str::lit(""))), v0);
    h156
}

pub fn f50(v0: Option<Adt6>) -> Option<u64> {
    let m157 = v0;
    match m157 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m158 = v1;
            match m158 {
                Adt6::C0(_) => {
                    None::<u64>
                }
                Adt6::C1(v2, v3) => {
                    if v2 {
                        None::<u64>
                    } else {
                        Some(v3)
                    }
                }
                Adt6::C2(_) => {
                    None::<u64>
                }
                Adt6::C3(_) => {
                    None::<u64>
                }
                Adt6::C4(_) => {
                    None::<u64>
                }
                Adt6::C5 => {
                    None::<u64>
                }
                Adt6::C6(_, _) => {
                    None::<u64>
                }
                Adt6::C7(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f51(v0: u64) -> Bytes {
    f87(v0)
}

pub fn f52(v0: Adt6) -> Adt6 {
    let v1: List<(Bytes, Adt6)> = {
        let (_, h159) = {
            let (h160, _) = f88(Fn21::F89, 1000000u64, (v0, List::<(Bytes, Adt6)>::nil()));
            h160
        };
        h159
    };
    f90(Fn22::F91, Adt6::C5, v1)
}

pub fn f53(v0: Adt6) -> R<Adt8> {
    let m161 = v0;
    match m161 {
        Adt6::C0(v1) => {
            Ok(Adt8::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt6::C1(v2, v3) => {
            Ok(Adt8::C1(f92(v2, v3)))
        }
        Adt6::C2(v4) => {
            Ok(Adt8::C1(f93(v4)?))
        }
        Adt6::C3(r162) => {
            let v5 = (*r162).clone();
            Ok(f13(f94(Str::lit("[")), f94(Str::lit("]")), f95(f96(v5)?)))
        }
        Adt6::C4(r163) => {
            let v6 = (*r163).clone();
            f97(f98(v6.clone())?, f99(v6)?)
        }
        Adt6::C5 => {
            Ok(Adt8::C0)
        }
        Adt6::C6(_, _) => {
            Ok(Adt8::C0)
        }
        Adt6::C7(_, _, _) => {
            Ok(Adt8::C0)
        }
    }
}

pub fn f54(v0: Adt9, v1: Adt1) -> R<Adt9> {
    let m165 = match v0.clone() {
        Adt9::C0(_, _, _, _, h164) => {
            h164
        }
    };
    match m165 {
        None => {
            let v3: u64 = match v0.clone() {
                Adt9::C0(h166, _, _, _, _) => {
                    h166
                }
            };
            let v4: (u64, List<u64>) = f100(match v0.clone() {
                Adt9::C0(_, _, h167, _, _) => {
                    h167
                }
            }, v3)?;
            let v7: bool = {
                let m169 = match v0.clone() {
                    Adt9::C0(_, _, _, h168, _) => {
                        h168
                    }
                };
                match m169.uncons() {
                    None => {
                        false
                    }
                    Some((v5, _)) => {
                        let a170 = v5;
                        let a171 = v3;
                        nat_eq(a170, a171)
                    }
                }
            };
            let v8: Adt15 = f101(Fn23::F102, Adt15::C0(match v0.clone() {
                Adt9::C0(_, h172, _, _, _) => {
                    h172
                }
            }, List::<Adt11>::nil(), 0u64), f103(f39(v1.clone())));
            let v36: Option<Bytes> = {
                let m173 = v1.clone();
                match m173 {
                    Adt1::C0(_) => {
                        None::<Bytes>
                    }
                    Adt1::C1(_) => {
                        None::<Bytes>
                    }
                    Adt1::C2(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C3(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C4(_, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C5(_, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C6(_, _, _, _, _) => {
                        None::<Bytes>
                    }
                    Adt1::C7(_) => {
                        None::<Bytes>
                    }
                    Adt1::C8(v9) => {
                        f104(v3, v9)
                    }
                    Adt1::C9(_, _) => {
                        None::<Bytes>
                    }
                    Adt1::C10(_, _, _, _) => {
                        None::<Bytes>
                    }
                }
            };
            let v39: u64 = {
                let m175 = match v8.clone() {
                    Adt15::C0(h174, _, _) => {
                        h174
                    }
                };
                match m175.uncons() {
                    None => {
                        0u64
                    }
                    Some((v37, _)) => {
                        let a177 = match v37 {
                            Adt10::C0(h176, _, _, _, _) => {
                                h176
                            }
                        };
                        let a178 = 1u64;
                        nat_add(a177, a178)?
                    }
                }
            };
            Ok(Adt9::C0({
                let a179 = v3;
                let a180 = 1u64;
                nat_add(a179, a180)?
            }, List::cons(Adt10::C0(v3, v39, {
                let a182 = match v8.clone() {
                    Adt15::C0(_, _, h181) => {
                        h181
                    }
                };
                let a183 = 1u64;
                nat_add(a182, a183)?
            }, {
                let (h184, _) = v4;
                h184
            }, std::rc::Rc::new(f105(f106(v1, match v8.clone() {
                Adt15::C0(_, h185, _) => {
                    h185
                }
            })?, v7))), match v8 {
                Adt15::C0(h186, _, _) => {
                    h186
                }
            }), {
                let (_, h187) = v4;
                h187
            }, if v7 {
                let m189 = match v0 {
                    Adt9::C0(_, _, _, h188, _) => {
                        h188
                    }
                };
                match m189.uncons() {
                    None => {
                        List::<u64>::nil()
                    }
                    Some((_, v41)) => {
                        v41
                    }
                }
            } else {
                match v0 {
                    Adt9::C0(_, _, _, h190, _) => {
                        h190
                    }
                }
            }, v36))
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f55(v0: Fn12, p194: List<u64>, p195: List<Adt6>) -> List<u64> {
    let m193 = (p194, p195);
    {
        let mut m193 = m193;
        loop {
            let step = {
                let (v1, v2) = m193;
                let m191 = v2;
                match m191.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt6>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt6>), List<u64>>(({
                            let c192: Fn12 = v0.clone();
                            c192.apply(v1, v3)
                        }, v4))
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

pub fn f56(v0: List<u64>, v1: Adt6) -> List<u64> {
    f107(Fn24::F108, v0, f43(v1))
}

pub fn f57(v0: Fn13, p199: List<u64>, p200: List<Adt1>) -> List<u64> {
    let m198 = (p199, p200);
    {
        let mut m198 = m198;
        loop {
            let step = {
                let (v1, v2) = m198;
                let m196 = v2;
                match m196.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt1>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt1>), List<u64>>(({
                            let c197: Fn13 = v0.clone();
                            c197.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<u64>, v1: Adt1) -> List<u64> {
    f107(Fn24::F109, v0, f39(v1))
}

pub fn f59(v0: Fn14, p204: List<u64>, p205: List<(Bytes, u64)>) -> List<u64> {
    let m203 = (p204, p205);
    {
        let mut m203 = m203;
        loop {
            let step = {
                let (v1, v2) = m203;
                let m201 = v2;
                match m201.uncons() {
                    None => {
                        Err::<(List<u64>, List<(Bytes, u64)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(Bytes, u64)>), List<u64>>(({
                            let c202: Fn14 = v0.clone();
                            c202.apply(v1, v3)
                        }, v4))
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

pub fn f60(v0: List<u64>, v1: (Bytes, u64)) -> List<u64> {
    List::cons({
        let (_, h206) = v1;
        h206
    }, v0)
}

pub fn f61(v0: List<(Bytes, u64)>) -> List<(Bytes, u64)> {
    f110(Fn25::F111, List::<(Bytes, u64)>::nil(), v0)
}

pub fn f62(v0: List<(Bytes, u64)>) -> R<List<(Bytes, u64)>> {
    let v1: List<List<(Bytes, u64)>> = f112(f113(Fn26::F114, List::<List<(Bytes, u64)>>::nil(), v0.clone()));
    let m211 = {
        let (h207, _) = f115(Fn27::F116, {
            let a209 = {
                let a208 = v0;
                length_list(a208)
            };
            let a210 = 1u64;
            nat_add(a209, a210)?
        }, v1)?;
        h207
    };
    match m211.uncons() {
        None => {
            Ok(List::<(Bytes, u64)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f63(v0: Fn15, p215: List<(Bytes, u64)>, p216: List<u64>) -> List<(Bytes, u64)> {
    let m214 = (p215, p216);
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, v2) = m214;
                let m212 = v2;
                match m212.uncons() {
                    None => {
                        Err::<(List<(Bytes, u64)>, List<u64>), List<(Bytes, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, u64)>, List<u64>), List<(Bytes, u64)>>(({
                            let c213: Fn15 = v0.clone();
                            c213.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, u64)>, v1: u64) -> List<(Bytes, u64)> {
    List::cons((f117(v1), v1), v0)
}

pub fn f65(v0: Adt12, v1: List<Bytes>) -> R<Adt8> {
    let m217 = v0;
    match m217 {
        Adt12::C0 => {
            Ok(Adt8::C0)
        }
        Adt12::C1(v2) => {
            Ok(Adt8::C1(v2))
        }
        Adt12::C2(r218, r219) => {
            let v3 = (*r218).clone();
            let v4 = (*r219).clone();
            Ok(Adt8::C2(std::rc::Rc::new(f65(v3, v1.clone())?), std::rc::Rc::new(f65(v4, v1)?)))
        }
        Adt12::C3(v5, r220) => {
            let v6 = (*r220).clone();
            Ok(Adt8::C3(v5, std::rc::Rc::new(f65(v6, v1)?)))
        }
        Adt12::C4 => {
            Ok(Adt8::C4)
        }
        Adt12::C5(r221) => {
            let v7 = (*r221).clone();
            Ok(Adt8::C6(std::rc::Rc::new(f65(v7, v1)?)))
        }
        Adt12::C6(v8) => {
            Ok(Adt8::C1(f118(v1, v8)?))
        }
        Adt12::C7(v9, v10) => {
            Ok(Adt8::C1(if f40(f75(v9.clone())?, !f119(v9.clone(), v1)) {
                f11(v9, v10)
            } else {
                f11(f11(f2(Str::lit("const ")), f76(v9)?), v10)
            }))
        }
        Adt12::C8(v11, r222) => {
            let v12 = (*r222).clone();
            f65(v12, List::cons(v11, v1))
        }
    }
}

pub fn f66(v0: List<Adt12>) -> Adt12 {
    f120(Fn28::F121, Adt12::C0, f122(v0))
}

pub fn f67(v0: Bytes) -> Adt12 {
    Adt12::C1(v0)
}

pub fn f68(v0: Adt11) -> Adt12 {
    let m223 = v0;
    match m223 {
        Adt11::C0(r224) => {
            (*r224).clone()
        }
        Adt11::C1(r225, v3) => {
            let v2 = (*r225).clone();
            f123(v2, v3)
        }
        Adt11::C2(r226) => {
            (*r226).clone()
        }
        Adt11::C3(r227) => {
            (*r227).clone()
        }
    }
}

pub fn f69(v0: Fn16, p231: List<Adt10>, p232: List<Adt10>) -> List<Adt10> {
    let m230 = (p231, p232);
    {
        let mut m230 = m230;
        loop {
            let step = {
                let (v1, v2) = m230;
                let m228 = v2;
                match m228.uncons() {
                    None => {
                        Err::<(List<Adt10>, List<Adt10>), List<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt10>, List<Adt10>), List<Adt10>>(({
                            let c229: Fn16 = v0.clone();
                            c229.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt10>, v1: Adt10) -> List<Adt10> {
    List::cons(v1, v0)
}

pub fn f71(v0: Fn17, p237: u64, p238: Adt13) -> R<(Adt13, bool)> {
    let m236 = (p237, p238);
    {
        let mut m236 = m236;
        loop {
            let step = {
                let (v1, v2) = m236;
                let m233 = v1;
                if m233 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m233 - 1;
                    let m235 = {
                        let c234: Fn17 = v0.clone();
                        c234.apply(v2.clone())?
                    };
                    match m235 {
                        None => {
                            Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt13), R<(Adt13, bool)>>((v3, v4))
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

pub fn f72(v0: u64, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    f124(v1, v0, v2)
}

pub fn f73(v0: List<Bytes>) -> Bytes {
    f125(f126(v0))
}

pub fn f74(v0: (bool, Adt8), v1: Adt8) -> (bool, Adt8) {
    let (h239, _) = v0;
    if h239 {
        (false, Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C0)))
    } else {
        (false, Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C1(f94(Str::lit(",")))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(Adt8::C4), std::rc::Rc::new({
            let (_, h240) = v0;
            h240
        })))))))
    }
}

pub fn f75(v0: Bytes) -> R<bool> {
    Ok(f40({
        let a242 = 0u64;
        let a243 = {
            let a241 = v0.clone();
            length_bytes(a241)
        };
        nat_lt(a242, a243)
    }, f40(f127(v0.clone())?, f40(f128(v0.clone())?, f40(!f129(v0.clone(), f2(Str::lit("."))), f40(!f1(v0.clone(), f2(Str::lit("_"))), f40(!f119(v0.clone(), f130()), !f131(v0)?)))))))
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a244 = v0.clone();
        length_bytes(a244)
    };
    let v2: Adt16 = {
        let (h245, _) = f132(Fn29::F133(v1, v0), {
            let a246 = v1;
            let a247 = 1u64;
            nat_add(a246, a247)?
        }, Adt16::C0(0u64, List::<Bytes>::nil()))?;
        h245
    };
    Ok(f134(f134(f135(Str::lit("\"")), f136(match v2 {
        Adt16::C0(_, h248) => {
            h248
        }
    })), f135(Str::lit("\""))))
}

pub fn f77(v0: List<u64>, v1: Option<u64>) -> List<u64> {
    let m249 = v1;
    match m249 {
        None => {
            v0
        }
        Some(v2) => {
            let a250 = v0;
            let a251 = List::cons(v2, List::<u64>::nil());
            append_list(a250, a251)
        }
    }
}

pub fn f78(v0: Option<Adt6>, v1: Bytes) -> Option<Adt6> {
    let m252 = v0;
    match m252 {
        None => {
            None::<Adt6>
        }
        Some(v2) => {
            f45(v2, v1)
        }
    }
}

pub fn f79(v0: List<u64>, v1: Adt6) -> List<u64> {
    f77(v0, f50(f45(v1, f2(Str::lit("auto_param")))))
}

pub fn f80(v0: Fn18, p257: u64, p258: (Adt6, Option<Adt6>)) -> ((Adt6, Option<Adt6>), bool) {
    let m256 = (p257, p258);
    {
        let mut m256 = m256;
        loop {
            let step = {
                let (v1, v2) = m256;
                let m253 = v1;
                if m253 == 0 {
                    Err::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v2, false))
                } else {
                    let v3 = m253 - 1;
                    let m255 = {
                        let c254: Fn18 = v0.clone();
                        c254.apply(v2.clone())
                    };
                    match m255 {
                        None => {
                            Err::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, Option<Adt6>)), ((Adt6, Option<Adt6>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
    f137(v0, v1)
}

pub fn f82(v0: List<Adt6>) -> List<Adt6> {
    f138(Fn30::F139, List::<Adt6>::nil(), v0)
}

pub fn f83(v0: Fn19, p263: u64, p264: (Adt6, List<Adt6>)) -> ((Adt6, List<Adt6>), bool) {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v1, v2) = m262;
                let m259 = v1;
                if m259 == 0 {
                    Err::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v2, false))
                } else {
                    let v3 = m259 - 1;
                    let m261 = {
                        let c260: Fn19 = v0.clone();
                        c260.apply(v2.clone())
                    };
                    match m261 {
                        None => {
                            Err::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, List<Adt6>)), ((Adt6, List<Adt6>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
    f140(v0)
}

pub fn f85(v0: Fn20, p268: (bool, Bytes), p269: List<Bytes>) -> (bool, Bytes) {
    let m267 = (p268, p269);
    {
        let mut m267 = m267;
        loop {
            let step = {
                let (v1, v2) = m267;
                let m265 = v2;
                match m265.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Bytes>), (bool, Bytes)>(({
                            let c266: Fn20 = v0.clone();
                            c266.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: (bool, Bytes), v2: Bytes) -> (bool, Bytes) {
    (false, {
        let (h270, _) = v1;
        if h270 {
            v2
        } else {
            f11(f11({
                let (_, h271) = v1;
                h271
            }, v0), v2)
        }
    })
}

pub fn f87(v0: u64) -> Bytes {
    f141(f142(), 10u64, v0)
}

pub fn f88(v0: Fn21, p276: u64, p277: (Adt6, List<(Bytes, Adt6)>)) -> ((Adt6, List<(Bytes, Adt6)>), bool) {
    let m275 = (p276, p277);
    {
        let mut m275 = m275;
        loop {
            let step = {
                let (v1, v2) = m275;
                let m272 = v1;
                if m272 == 0 {
                    Err::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v2, false))
                } else {
                    let v3 = m272 - 1;
                    let m274 = {
                        let c273: Fn21 = v0.clone();
                        c273.apply(v2.clone())
                    };
                    match m274 {
                        None => {
                            Err::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt6, List<(Bytes, Adt6)>)), ((Adt6, List<(Bytes, Adt6)>), bool)>((v3, v4))
                        }
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

pub fn f89(v0: (Adt6, List<(Bytes, Adt6)>)) -> Option<(Adt6, List<(Bytes, Adt6)>)> {
    let m279 = {
        let (h278, _) = v0;
        h278
    };
    match m279 {
        Adt6::C0(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C2(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C4(_) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C5 => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C6(_, _) => {
            None::<(Adt6, List<(Bytes, Adt6)>)>
        }
        Adt6::C7(v1, r282, r283) => {
            let v2 = (*r282).clone();
            let v3 = (*r283).clone();
            Some((v3, if f119(v1.clone(), List::cons(f2(Str::lit("kind")), List::cons(f2(Str::lit("name")), List::cons(f2(Str::lit("levels")), List::cons(f2(Str::lit("type")), List::cons(f2(Str::lit("value")), List::<Bytes>::nil())))))) {
                let (_, h280) = v0;
                h280
            } else {
                List::cons((v1, v2), {
                    let (_, h281) = v0;
                    h281
                })
            }))
        }
    }
}

pub fn f90(v0: Fn22, p287: Adt6, p288: List<(Bytes, Adt6)>) -> Adt6 {
    let m286 = (p287, p288);
    {
        let mut m286 = m286;
        loop {
            let step = {
                let (v1, v2) = m286;
                let m284 = v2;
                match m284.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Adt6)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Adt6)>), Adt6>(({
                            let c285: Fn22 = v0.clone();
                            c285.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt6, v1: (Bytes, Adt6)) -> Adt6 {
    Adt6::C7({
        let (h289, _) = v1;
        h289
    }, std::rc::Rc::new({
        let (_, h290) = v1;
        h290
    }), std::rc::Rc::new(v0))
}

pub fn f92(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a291 = 0u64;
        let a292 = v1;
        nat_lt(a291, a292)
    } else {
        false
    } {
        let a293 = Bytes::lit(&[0x2d]);
        let a294 = f87(v1);
        append_bytes(a293, a294)
    } else {
        f87(v1)
    }
}

pub fn f93(v0: Bytes) -> R<Bytes> {
    if f143(v0.clone())? {
        Ok(v0)
    } else {
        f76(v0)
    }
}

pub fn f94(v0: Str) -> Bytes {
    let a295 = v0;
    utf8_encode(a295)
}

pub fn f95(v0: List<Adt8>) -> List<Adt8> {
    f144(Fn31::F145, List::<Adt8>::nil(), v0)
}

pub fn f96(v0: Adt6) -> R<List<Adt8>> {
    let m296 = v0;
    match m296 {
        Adt6::C0(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C5 => {
            Ok(List::<Adt8>::nil())
        }
        Adt6::C6(r297, r298) => {
            let v7 = (*r297).clone();
            let v8 = (*r298).clone();
            Ok(List::cons(f53(v7)?, f96(v8)?))
        }
        Adt6::C7(_, _, _) => {
            Ok(List::<Adt8>::nil())
        }
    }
}

pub fn f97(v0: Option<Bytes>, v1: List<(Bytes, Adt8)>) -> R<Adt8> {
    let v2: List<(Bytes, Adt8)> = f146(Fn32::F147(v0.clone()), List::<(Bytes, Adt8)>::nil(), v1)?;
    Ok(f13({
        let m299 = v0;
        match m299 {
            None => {
                f94(Str::lit("{"))
            }
            Some(v3) => {
                f33(v3, f94(Str::lit("{")))
            }
        }
    }, f94(Str::lit("}")), f148(Fn33::F149, List::<Adt8>::nil(), f150(v2)?)))
}

pub fn f98(p307: Adt6) -> R<Option<Bytes>> {
    let m306 = p307;
    {
        let mut m306 = m306;
        loop {
            let step = {
                let v0 = m306;
                let m300 = v0;
                match m300 {
                    Adt6::C0(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C1(_, _) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C2(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C3(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C4(_) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C5 => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C6(_, _) => {
                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt6::C7(v9, r304, r305) => {
                        let v10 = (*r304).clone();
                        let v11 = (*r305).clone();
                        let a301 = v9;
                        let a302 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a301, a302) {
                            let m303 = v10;
                            match m303 {
                                Adt6::C0(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C1(_, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C2(v12) => {
                                    if f143(v12.clone())? {
                                        Err::<Adt6, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt6::C3(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C4(_) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C5 => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C6(_, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt6::C7(_, _, _) => {
                                    Err::<Adt6, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt6, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Adt6) -> R<List<(Bytes, Adt8)>> {
    let m308 = v0;
    match m308 {
        Adt6::C0(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C1(_, _) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C3(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C4(_) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C5 => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C6(_, _) => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Adt6::C7(v9, r309, r310) => {
            let v10 = (*r309).clone();
            let v11 = (*r310).clone();
            Ok(List::cons((v9, f53(v10)?), f99(v11)?))
        }
    }
}

pub fn f100(v0: List<u64>, v1: u64) -> R<(u64, List<u64>)> {
    let (h311, _) = f151(Fn34::F152(v1), {
        let a313 = {
            let a312 = v0.clone();
            length_list(a312)
        };
        let a314 = 1u64;
        nat_add(a313, a314)?
    }, (0u64, v0))?;
    Ok(h311)
}

pub fn f101(v0: Fn23, p318: Adt15, p319: List<u64>) -> Adt15 {
    let m317 = (p318, p319);
    {
        let mut m317 = m317;
        loop {
            let step = {
                let (v1, v2) = m317;
                let m315 = v2;
                match m315.uncons() {
                    None => {
                        Err::<(Adt15, List<u64>), Adt15>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt15, List<u64>), Adt15>(({
                            let c316: Fn23 = v0.clone();
                            c316.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt15, v1: u64) -> Adt15 {
    f153(v0, v1)
}

pub fn f103(v0: List<u64>) -> List<u64> {
    f107(Fn24::F154, List::<u64>::nil(), v0)
}

pub fn f104(v0: u64, v1: List<u64>) -> Option<Bytes> {
    f37(Fn9::F155(v0), None::<Bytes>, v1)
}

pub fn f105(v0: Adt11, v1: bool) -> Adt11 {
    if v1 {
        Adt11::C0(std::rc::Rc::new(f66(List::cons(f67(f2(Str::lit("proof("))), List::cons(Adt12::C3(2u64, std::rc::Rc::new(f68(v0))), List::cons(f67(f2(Str::lit(")"))), List::<Adt12>::nil()))))))
    } else {
        v0
    }
}

pub fn f106(v0: Adt1, v1: List<Adt11>) -> R<Adt11> {
    let m320 = v0;
    match m320 {
        Adt1::C0(v2) => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C6(v2))))
        }
        Adt1::C1(r326) => {
            let v3 = (*r326).clone();
            Ok(Adt11::C2(std::rc::Rc::new(f67(f11(f2(Str::lit("Sort ")), f156(v3)?)))))
        }
        Adt1::C2(v4, v5) => {
            Ok(Adt11::C0(std::rc::Rc::new(Adt12::C7(v4, f157(v5)?))))
        }
        Adt1::C3(_, _) => {
            let v8: Adt17 = f158(f159(v1.clone(), 0u64));
            Ok(Adt11::C1(std::rc::Rc::new(match v8.clone() {
                Adt17::C0(h321, _) => {
                    (*h321).clone()
                }
            }), List::cons(f160(f159(v1, 1u64)), match v8 {
                Adt17::C0(_, h322) => {
                    h322
                }
            })))
        }
        Adt1::C4(v9, v10, _, _) => {
            Ok(Adt11::C3(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f2(Str::lit("fun "))), List::cons(f161(v10, v9.clone(), f159(v1.clone(), 0u64))?, List::cons(f67(f2(Str::lit(" =>"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v9, std::rc::Rc::new(f68(f159(v1, 1u64)))), List::<Adt12>::nil())))))))))))
        }
        Adt1::C5(v13, v14, _, _) => {
            Ok(Adt11::C3(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(f66(List::cons(if f40({
                let a324 = {
                    let a323 = v13.clone();
                    length_bytes(a323)
                };
                let a325 = 0u64;
                nat_eq(a324, a325)
            }, f162(v14.clone())) {
                f163(f159(v1.clone(), 0u64))
            } else {
                f161(v14, v13.clone(), f159(v1.clone(), 0u64))?
            }, List::cons(f67(f2(Str::lit(" ->"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v13, std::rc::Rc::new(f68(f159(v1, 1u64)))), List::<Adt12>::nil()))))))))))
        }
        Adt1::C6(v17, _, _, _, v21) => {
            Ok(Adt11::C3(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(if v21 {
                f2(Str::lit("have "))
            } else {
                f2(Str::lit("let "))
            }, f164(v17.clone())?), f2(Str::lit(" : ")))), List::cons(f68(f159(v1.clone(), 0u64)), List::cons(f67(f2(Str::lit(" := "))), List::cons(f68(f159(v1.clone(), 1u64)), List::cons(f67(f2(Str::lit(";"))), List::cons(Adt12::C4, List::cons(Adt12::C8(v17, std::rc::Rc::new(f68(f159(v1, 2u64)))), List::<Adt12>::nil())))))))))))))
        }
        Adt1::C7(v22) => {
            Ok(Adt11::C0(std::rc::Rc::new(f67(if f165(v22.clone())? {
                v22
            } else {
                f11(f2(Str::lit("nat ")), f76(v22)?)
            }))))
        }
        Adt1::C8(v23) => {
            Ok(Adt11::C0(std::rc::Rc::new(f67(f76(f166(Fn35::F167, f2(Str::lit("")), v23)?)?))))
        }
        Adt1::C9(v24, _) => {
            Ok(Adt11::C2(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(f2(Str::lit("annot [")), f49(f168(f169(Fn36::F170, List::<Bytes>::nil(), v24)?), f2(Str::lit(", ")))), f2(Str::lit("]")))), List::cons(Adt12::C4, List::cons(f160(f159(v1, 0u64)), List::<Adt12>::nil())))))))))
        }
        Adt1::C10(v26, v27, v28, _) => {
            Ok(Adt11::C2(std::rc::Rc::new(Adt12::C5(std::rc::Rc::new(f66(List::cons(f67(f11(f11(f11(f11(f11(f2(Str::lit("proj ")), f36(v27)?), f2(Str::lit(" "))), f36(v26)?), f2(Str::lit(" "))), f51(v28))), List::cons(Adt12::C4, List::cons(f160(f159(v1, 0u64)), List::<Adt12>::nil())))))))))
        }
    }
}

pub fn f107(v0: Fn24, p330: List<u64>, p331: List<u64>) -> List<u64> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m327 = v2;
                match m327.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c328: Fn24 = v0.clone();
                            c328.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f109(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f110(v0: Fn25, p335: List<(Bytes, u64)>, p336: List<(Bytes, u64)>) -> List<(Bytes, u64)> {
    let m334 = (p335, p336);
    {
        let mut m334 = m334;
        loop {
            let step = {
                let (v1, v2) = m334;
                let m332 = v2;
                match m332.uncons() {
                    None => {
                        Err::<(List<(Bytes, u64)>, List<(Bytes, u64)>), List<(Bytes, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, u64)>, List<(Bytes, u64)>), List<(Bytes, u64)>>(({
                            let c333: Fn25 = v0.clone();
                            c333.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<(Bytes, u64)>, v1: (Bytes, u64)) -> List<(Bytes, u64)> {
    List::cons(v1, v0)
}

pub fn f112(v0: List<List<(Bytes, u64)>>) -> List<List<(Bytes, u64)>> {
    f171(Fn37::F172, List::<List<(Bytes, u64)>>::nil(), v0)
}

pub fn f113(v0: Fn26, p340: List<List<(Bytes, u64)>>, p341: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v2;
                match m337.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, u64)>>, List<(Bytes, u64)>), List<List<(Bytes, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, u64)>>, List<(Bytes, u64)>), List<List<(Bytes, u64)>>>(({
                            let c338: Fn26 = v0.clone();
                            c338.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<(Bytes, u64)>>, v1: (Bytes, u64)) -> List<List<(Bytes, u64)>> {
    List::cons(List::cons(v1, List::<(Bytes, u64)>::nil()), v0)
}

pub fn f115(v0: Fn27, p346: u64, p347: List<List<(Bytes, u64)>>) -> R<(List<List<(Bytes, u64)>>, bool)> {
    let m345 = (p346, p347);
    {
        let mut m345 = m345;
        loop {
            let step = {
                let (v1, v2) = m345;
                let m342 = v1;
                if m342 == 0 {
                    Err::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m342 - 1;
                    let m344 = {
                        let c343: Fn27 = v0.clone();
                        c343.apply(v2.clone())?
                    };
                    match m344 {
                        None => {
                            Err::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, u64)>>), R<(List<List<(Bytes, u64)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
    f173(v0)
}

pub fn f117(v0: u64) -> Bytes {
    let (_, h348) = f174(Fn38::F175, 8u64, (v0, f2(Str::lit(""))));
    h348
}

pub fn f118(v0: List<Bytes>, v1: u64) -> R<Bytes> {
    let v2: Bytes = f11(f2(Str::lit("#")), f51(v1));
    let m351 = {
        let a349 = v0.clone();
        let a350 = v1;
        index_list(a349, a350)
    };
    match m351 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            if f40(f75(v3.clone())?, {
                let m353 = f176(Fn39::F177(v3.clone(), v1), Adt18::C0(0u64, false), v0)?;
                !match m353 {
                    Adt18::C0(_, h352) => {
                        h352
                    }
                }
            }) {
                Ok(v3)
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f119(v0: Bytes, v1: List<Bytes>) -> bool {
    f178(Fn40::F179(v0), false, v1)
}

pub fn f120(v0: Fn28, p357: Adt12, p358: List<Adt12>) -> Adt12 {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v1, v2) = m356;
                let m354 = v2;
                match m354.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c355: Fn28 = v0.clone();
                            c355.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f122(v0: List<Adt12>) -> List<Adt12> {
    f180(Fn41::F181, List::<Adt12>::nil(), v0)
}

pub fn f123(v0: Adt12, v1: List<Adt12>) -> Adt12 {
    let m359 = v1.clone();
    match m359.uncons() {
        None => {
            v0
        }
        Some((_, _)) => {
            Adt12::C5(std::rc::Rc::new(f66(List::cons(v0, List::cons(Adt12::C3(2u64, std::rc::Rc::new(f66(f180(Fn41::F182, List::<Adt12>::nil(), v1)))), List::<Adt12>::nil())))))
        }
    }
}

pub fn f124(v0: u64, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m361 = match v2.clone() {
        Adt13::C0(_, _, h360) => {
            h360
        }
    };
    match m361.uncons() {
        None => {
            Ok(None::<Adt13>)
        }
        Some((v3, v4)) => {
            Ok(Some(f183(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f125(v0: List<Bytes>) -> Bytes {
    let m364 = {
        let (h362, _) = f184(Fn42::F185, {
            let a363 = v0.clone();
            length_list(a363)
        }, v0);
        h362
    };
    match m364.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f126(v0: List<Bytes>) -> List<Bytes> {
    f186(Fn43::F187, List::<Bytes>::nil(), v0)
}

pub fn f127(v0: Bytes) -> R<bool> {
    let v1: u64 = f188(v0.clone(), 0u64)?;
    if f189(v1) {
        Ok(true)
    } else {
        let a365 = v1;
        let a366 = 95u64;
        if nat_eq(a365, a366) {
            Ok(true)
        } else {
            Ok(f40({
                let a367 = 192u64;
                let a368 = v1;
                nat_le(a367, a368)
            }, {
                let a369 = f190(v0, 0u64)?;
                let a370 = 0u64;
                nat_eq(a369, a370)
            }))
        }
    }
}

pub fn f128(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h371, _) = f191(Fn44::F192(v0.clone()), {
            let a373 = {
                let a372 = v0.clone();
                length_bytes(a372)
            };
            let a374 = 1u64;
            nat_add(a373, a374)?
        }, 0u64)?;
        h371
    };
    let a376 = v1;
    let a377 = {
        let a375 = v0;
        length_bytes(a375)
    };
    Ok(nat_eq(a376, a377))
}

pub fn f129(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a378 = v0.clone();
        length_bytes(a378)
    };
    let v3: u64 = {
        let a379 = v1.clone();
        length_bytes(a379)
    };
    let a380 = v3;
    let a381 = v2;
    if nat_le(a380, a381) {
        let m387 = {
            let a384 = v0;
            let a385 = {
                let a382 = v2;
                let a383 = v3;
                nat_sub(a382, a383)
            };
            let a386 = v3;
            slice_bytes(a384, a385, a386)
        };
        match m387 {
            None => {
                false
            }
            Some(v4) => {
                let a388 = v4;
                let a389 = v1;
                equal(a388, a389)
            }
        }
    } else {
        false
    }
}

pub fn f130() -> List<Bytes> {
    List::cons(f2(Str::lit("fun")), List::cons(f2(Str::lit("let")), List::cons(f2(Str::lit("have")), List::cons(f2(Str::lit("proof")), List::cons(f2(Str::lit("Sort")), List::cons(f2(Str::lit("proj")), List::cons(f2(Str::lit("annot")), List::cons(f2(Str::lit("const")), List::cons(f2(Str::lit("max")), List::cons(f2(Str::lit("imax")), List::cons(f2(Str::lit("true")), List::cons(f2(Str::lit("false")), List::cons(f2(Str::lit("bool")), List::cons(f2(Str::lit("nat")), List::cons(f2(Str::lit("int")), List::cons(f2(Str::lit("str")), List::cons(f2(Str::lit("name")), List::<Bytes>::nil())))))))))))))))))
}

pub fn f131(v0: Bytes) -> R<bool> {
    let m395 = {
        let (h391, _) = f193(Fn45::F194(v0.clone()), {
            let a393 = {
                let a392 = v0;
                length_bytes(a392)
            };
            let a394 = 1u64;
            nat_add(a393, a394)?
        }, Adt19::C0(0u64, false))?;
        h391
    };
    match m395 {
        Adt19::C0(_, h390) => {
            Ok(h390)
        }
    }
}

pub fn f132(v0: Fn29, p400: u64, p401: Adt16) -> R<(Adt16, bool)> {
    let m399 = (p400, p401);
    {
        let mut m399 = m399;
        loop {
            let step = {
                let (v1, v2) = m399;
                let m396 = v1;
                if m396 == 0 {
                    Err::<(u64, Adt16), R<(Adt16, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m396 - 1;
                    let m398 = {
                        let c397: Fn29 = v0.clone();
                        c397.apply(v2.clone())?
                    };
                    match m398 {
                        None => {
                            Err::<(u64, Adt16), R<(Adt16, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt16), R<(Adt16, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: u64, v1: Bytes, v2: Adt16) -> R<Option<Adt16>> {
    f195(v1, v0, v2)
}

pub fn f134(v0: Bytes, v1: Bytes) -> Bytes {
    let a402 = v0;
    let a403 = v1;
    append_bytes(a402, a403)
}

pub fn f135(v0: Str) -> Bytes {
    let a404 = v0;
    utf8_encode(a404)
}

pub fn f136(v0: List<Bytes>) -> Bytes {
    f125(f126(v0))
}

pub fn f137(v0: Bytes, v1: (Adt6, Option<Adt6>)) -> Option<(Adt6, Option<Adt6>)> {
    let m406 = {
        let (_, h405) = v1;
        h405
    };
    match m406 {
        None => {
            let m408 = {
                let (h407, _) = v1;
                h407
            };
            match m408 {
                Adt6::C0(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C1(_, _) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C2(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C3(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C4(_) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C5 => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C6(_, _) => {
                    None::<(Adt6, Option<Adt6>)>
                }
                Adt6::C7(v3, r409, r410) => {
                    let v4 = (*r409).clone();
                    let v5 = (*r410).clone();
                    Some(if f1(v3, v0) {
                        (Adt6::C5, Some(v4))
                    } else {
                        (v5, None::<Adt6>)
                    })
                }
            }
        }
        Some(_) => {
            None::<(Adt6, Option<Adt6>)>
        }
    }
}

pub fn f138(v0: Fn30, p414: List<Adt6>, p415: List<Adt6>) -> List<Adt6> {
    let m413 = (p414, p415);
    {
        let mut m413 = m413;
        loop {
            let step = {
                let (v1, v2) = m413;
                let m411 = v2;
                match m411.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c412: Fn30 = v0.clone();
                            c412.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f140(v0: (Adt6, List<Adt6>)) -> Option<(Adt6, List<Adt6>)> {
    let m417 = {
        let (h416, _) = v0;
        h416
    };
    match m417 {
        Adt6::C0(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C1(_, _) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C2(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C3(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C4(_) => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C5 => {
            None::<(Adt6, List<Adt6>)>
        }
        Adt6::C6(r419, r420) => {
            let v1 = (*r419).clone();
            let v2 = (*r420).clone();
            Some((v2, List::cons(v1, {
                let (_, h418) = v0;
                h418
            })))
        }
        Adt6::C7(_, _, _) => {
            None::<(Adt6, List<Adt6>)>
        }
    }
}

pub fn f141(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a421 = v2;
    let a422 = 0u64;
    if nat_eq(a421, a422) {
        f196(v0, 0u64)
    } else {
        let (_, h423) = {
            let (h424, _) = f197(Fn46::F198(v1, v0), v2, (v2, Bytes::lit(&[])));
            h424
        };
        h423
    }
}

pub fn f142() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f143(v0: Bytes) -> R<bool> {
    let a426 = {
        let a425 = v0.clone();
        length_bytes(a425)
    };
    let a427 = 0u64;
    if nat_eq(a426, a427) {
        Ok(false)
    } else {
        if f199(f200(v0.clone(), 0u64)?) {
            let a429 = f201(v0.clone(), 1u64)?;
            let a430 = {
                let a428 = v0.clone();
                length_bytes(a428)
            };
            if nat_eq(a429, a430) {
                let a431 = f202(v0);
                Ok(bool_not(a431))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f144(v0: Fn31, p435: List<Adt8>, p436: List<Adt8>) -> List<Adt8> {
    let m434 = (p435, p436);
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v1, v2) = m434;
                let m432 = v2;
                match m432.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Adt8>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Adt8>), List<Adt8>>(({
                            let c433: Fn31 = v0.clone();
                            c433.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: List<Adt8>, v1: Adt8) -> List<Adt8> {
    List::cons(v1, v0)
}

pub fn f146(v0: Fn32, p440: List<(Bytes, Adt8)>, p441: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let m439 = (p440, p441);
    {
        let mut m439 = m439;
        loop {
            let step = {
                let (v1, v2) = m439;
                let m437 = v2;
                match m437.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), R<List<(Bytes, Adt8)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt8)>, List<(Bytes, Adt8)>), R<List<(Bytes, Adt8)>>>(({
                            let c438: Fn32 = v0.clone();
                            c438.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Option<Bytes>, v1: List<(Bytes, Adt8)>, v2: (Bytes, Adt8)) -> R<List<(Bytes, Adt8)>> {
    let m442 = v0;
    match m442 {
        None => {
            Ok(List::cons((f203({
                let (h449, _) = v2.clone();
                h449
            }), f204({
                let (h450, _) = v2;
                h450
            }, {
                let (_, h451) = v2;
                h451
            })?), v1))
        }
        Some(_) => {
            let a444 = {
                let (h443, _) = v2.clone();
                h443
            };
            let a445 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a444, a445) {
                Ok(v1)
            } else {
                Ok(List::cons((f203({
                    let (h446, _) = v2.clone();
                    h446
                }), f204({
                    let (h447, _) = v2;
                    h447
                }, {
                    let (_, h448) = v2;
                    h448
                })?), v1))
            }
        }
    }
}

pub fn f148(v0: Fn33, p455: List<Adt8>, p456: List<(Bytes, Adt8)>) -> List<Adt8> {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m452 = v2;
                match m452.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<(Bytes, Adt8)>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<(Bytes, Adt8)>), List<Adt8>>(({
                            let c453: Fn33 = v0.clone();
                            c453.apply(v1, v3)
                        }, v4))
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

pub fn f149(v0: List<Adt8>, v1: (Bytes, Adt8)) -> List<Adt8> {
    List::cons({
        let (_, h457) = v1;
        h457
    }, v0)
}

pub fn f150(v0: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v1: List<List<(Bytes, Adt8)>> = f205(f206(Fn47::F207, List::<List<(Bytes, Adt8)>>::nil(), v0.clone()));
    let m462 = {
        let (h458, _) = f208(Fn48::F209, {
            let a460 = {
                let a459 = v0;
                length_list(a459)
            };
            let a461 = 1u64;
            nat_add(a460, a461)?
        }, v1)?;
        h458
    };
    match m462.uncons() {
        None => {
            Ok(List::<(Bytes, Adt8)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f151(v0: Fn34, p467: u64, p468: (u64, List<u64>)) -> R<((u64, List<u64>), bool)> {
    let m466 = (p467, p468);
    {
        let mut m466 = m466;
        loop {
            let step = {
                let (v1, v2) = m466;
                let m463 = v1;
                if m463 == 0 {
                    Err::<(u64, (u64, List<u64>)), R<((u64, List<u64>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m463 - 1;
                    let m465 = {
                        let c464: Fn34 = v0.clone();
                        c464.apply(v2.clone())?
                    };
                    match m465 {
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
                Ok(next) => { m466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: u64, v1: (u64, List<u64>)) -> R<Option<(u64, List<u64>)>> {
    let m470 = {
        let (_, h469) = v1;
        h469
    };
    match m470.uncons() {
        None => {
            Ok(None::<(u64, List<u64>)>)
        }
        Some((v2, v3)) => {
            let a471 = v2;
            let a472 = v0;
            if nat_eq(a471, a472) {
                Ok(Some(({
                    let a474 = {
                        let (h473, _) = v1;
                        h473
                    };
                    let a475 = 1u64;
                    nat_add(a474, a475)?
                }, v3)))
            } else {
                Ok(None::<(u64, List<u64>)>)
            }
        }
    }
}

pub fn f153(v0: Adt15, v1: u64) -> Adt15 {
    let v2: Adt15 = Adt15::C0(match v0.clone() {
        Adt15::C0(h476, _, _) => {
            h476
        }
    }, List::cons(Adt11::C0(std::rc::Rc::new(f67(f11(f2(Str::lit("@")), f51(v1))))), match v0.clone() {
        Adt15::C0(_, h477, _) => {
            h477
        }
    }), match v0.clone() {
        Adt15::C0(_, _, h478) => {
            h478
        }
    });
    let m480 = match v0.clone() {
        Adt15::C0(h479, _, _) => {
            h479
        }
    };
    match m480.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            if f40({
                let a482 = match v3.clone() {
                    Adt10::C0(h481, _, _, _, _) => {
                        h481
                    }
                };
                let a483 = v1;
                nat_eq(a482, a483)
            }, f40({
                let a485 = match v3.clone() {
                    Adt10::C0(_, _, _, h484, _) => {
                        h484
                    }
                };
                let a486 = 1u64;
                nat_eq(a485, a486)
            }, {
                let a488 = match v3.clone() {
                    Adt10::C0(_, _, h487, _, _) => {
                        h487
                    }
                };
                let a489 = 48u64;
                nat_lt(a488, a489)
            })) {
                Adt15::C0(v4, List::cons(match v3.clone() {
                    Adt10::C0(_, _, _, _, h490) => {
                        (*h490).clone()
                    }
                }, match v0.clone() {
                    Adt15::C0(_, h491, _) => {
                        h491
                    }
                }), {
                    let a494 = match v0.clone() {
                        Adt15::C0(_, _, h492) => {
                            h492
                        }
                    };
                    let a495 = match v3.clone() {
                        Adt10::C0(_, _, h493, _, _) => {
                            h493
                        }
                    };
                    if nat_lt(a494, a495) {
                        match v3 {
                            Adt10::C0(_, _, h496, _, _) => {
                                h496
                            }
                        }
                    } else {
                        match v0 {
                            Adt15::C0(_, _, h497) => {
                                h497
                            }
                        }
                    }
                })
            } else {
                v2
            }
        }
    }
}

pub fn f154(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f155(v0: u64, v1: Option<Bytes>, v2: u64) -> Option<Bytes> {
    let m498 = v1.clone();
    match m498 {
        None => {
            if f210(v2) {
                None::<Bytes>
            } else {
                Some(f11(f11(f11(f11(f2(Str::lit("node ")), f51(v0)), f2(Str::lit(": "))), f51(v2)), f2(Str::lit(" is not a Unicode scalar value"))))
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f156(v0: Adt2) -> R<Bytes> {
    Ok(f211(f212(v0)?))
}

pub fn f157(v0: List<Adt2>) -> R<Bytes> {
    let a500 = {
        let a499 = v0.clone();
        length_list(a499)
    };
    let a501 = 0u64;
    if nat_eq(a500, a501) {
        Ok(f2(Str::lit("")))
    } else {
        Ok(f11(f11(f2(Str::lit(".{")), f49(f168(f213(Fn49::F214, List::<Bytes>::nil(), v0)?), f2(Str::lit(", ")))), f2(Str::lit("}"))))
    }
}

pub fn f158(v0: Adt11) -> Adt17 {
    let m502 = v0;
    match m502 {
        Adt11::C0(r503) => {
            let v1 = (*r503).clone();
            Adt17::C0(std::rc::Rc::new(v1), List::<Adt12>::nil())
        }
        Adt11::C1(r504, v3) => {
            let v2 = (*r504).clone();
            Adt17::C0(std::rc::Rc::new(v2), v3)
        }
        Adt11::C2(r505) => {
            let v4 = (*r505).clone();
            Adt17::C0(std::rc::Rc::new(f215(v4)), List::<Adt12>::nil())
        }
        Adt11::C3(r506) => {
            let v5 = (*r506).clone();
            Adt17::C0(std::rc::Rc::new(f215(v5)), List::<Adt12>::nil())
        }
    }
}

pub fn f159(v0: List<Adt11>, v1: u64) -> Adt11 {
    let m509 = {
        let a507 = v0;
        let a508 = v1;
        index_list(a507, a508)
    };
    match m509 {
        None => {
            Adt11::C0(std::rc::Rc::new(Adt12::C0))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f160(v0: Adt11) -> Adt12 {
    let m510 = v0.clone();
    match m510 {
        Adt11::C0(r511) => {
            (*r511).clone()
        }
        Adt11::C1(_, _) => {
            f215(f68(v0))
        }
        Adt11::C2(r512) => {
            let v4 = (*r512).clone();
            f215(v4)
        }
        Adt11::C3(r513) => {
            let v5 = (*r513).clone();
            f215(v5)
        }
    }
}

pub fn f161(v0: Adt3, v1: Bytes, v2: Adt11) -> R<Adt12> {
    let v3: (Bytes, Bytes) = {
        let m514 = v0;
        match m514 {
            Adt3::C0 => {
                (f2(Str::lit("(")), f2(Str::lit(")")))
            }
            Adt3::C1 => {
                (f2(Str::lit("{")), f2(Str::lit("}")))
            }
            Adt3::C2 => {
                (f2(Str::lit("{{")), f2(Str::lit("}}")))
            }
            Adt3::C3 => {
                (f2(Str::lit("[")), f2(Str::lit("]")))
            }
        }
    };
    Ok(f66(List::cons(f67(f11(f11({
        let (h515, _) = v3;
        h515
    }, f164(v1)?), f2(Str::lit(" : ")))), List::cons(f68(v2), List::cons(f67({
        let (_, h516) = v3;
        h516
    }), List::<Adt12>::nil())))))
}

pub fn f162(v0: Adt3) -> bool {
    let m517 = v0;
    match m517 {
        Adt3::C0 => {
            true
        }
        Adt3::C1 => {
            false
        }
        Adt3::C2 => {
            false
        }
        Adt3::C3 => {
            false
        }
    }
}

pub fn f163(v0: Adt11) -> Adt12 {
    let m518 = v0.clone();
    match m518 {
        Adt11::C0(r519) => {
            (*r519).clone()
        }
        Adt11::C1(_, _) => {
            f68(v0)
        }
        Adt11::C2(r520) => {
            (*r520).clone()
        }
        Adt11::C3(r521) => {
            let v1 = (*r521).clone();
            f215(v1)
        }
    }
}

pub fn f164(v0: Bytes) -> R<Bytes> {
    let a523 = {
        let a522 = v0.clone();
        length_bytes(a522)
    };
    let a524 = 0u64;
    if nat_eq(a523, a524) {
        Ok(f2(Str::lit("_")))
    } else {
        f36(v0)
    }
}

pub fn f165(v0: Bytes) -> R<bool> {
    Ok(f40({
        let a526 = 0u64;
        let a527 = {
            let a525 = v0.clone();
            length_bytes(a525)
        };
        nat_lt(a526, a527)
    }, f40({
        let a529 = f216(v0.clone(), 0u64)?;
        let a530 = {
            let a528 = v0.clone();
            length_bytes(a528)
        };
        nat_eq(a529, a530)
    }, if f1(v0.clone(), f2(Str::lit("0"))) {
        true
    } else {
        !f217(v0, f2(Str::lit("0")))
    })))
}

pub fn f166(v0: Fn35, p534: Bytes, p535: List<u64>) -> R<Bytes> {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v1, v2) = m533;
                let m531 = v2;
                match m531.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c532: Fn35 = v0.clone();
                            c532.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<Bytes> {
    Ok(f11(v0, f218(v1)?))
}

pub fn f168(v0: List<Bytes>) -> List<Bytes> {
    f186(Fn43::F219, List::<Bytes>::nil(), v0)
}

pub fn f169(v0: Fn36, p539: List<Bytes>, p540: List<Adt4>) -> R<List<Bytes>> {
    let m538 = (p539, p540);
    {
        let mut m538 = m538;
        loop {
            let step = {
                let (v1, v2) = m538;
                let m536 = v2;
                match m536.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt4>), R<List<Bytes>>>(({
                            let c537: Fn36 = v0.clone();
                            c537.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: Adt4) -> R<List<Bytes>> {
    Ok(List::cons(f220(v1)?, v0))
}

pub fn f171(v0: Fn37, p544: List<List<(Bytes, u64)>>, p545: List<List<(Bytes, u64)>>) -> List<List<(Bytes, u64)>> {
    let m543 = (p544, p545);
    {
        let mut m543 = m543;
        loop {
            let step = {
                let (v1, v2) = m543;
                let m541 = v2;
                match m541.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), List<List<(Bytes, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), List<List<(Bytes, u64)>>>(({
                            let c542: Fn37 = v0.clone();
                            c542.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: List<List<(Bytes, u64)>>, v1: List<(Bytes, u64)>) -> List<List<(Bytes, u64)>> {
    List::cons(v1, v0)
}

pub fn f173(v0: List<List<(Bytes, u64)>>) -> R<Option<List<List<(Bytes, u64)>>>> {
    let a547 = {
        let a546 = v0.clone();
        length_list(a546)
    };
    let a548 = 1u64;
    if nat_le(a547, a548) {
        Ok(None::<List<List<(Bytes, u64)>>>)
    } else {
        Ok(Some(f221(v0)?))
    }
}

pub fn f174(v0: Fn38, p552: u64, p553: (u64, Bytes)) -> (u64, Bytes) {
    let m551 = (p552, p553);
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v1, v2) = m551;
                let m549 = v1;
                if m549 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m549 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c550: Fn38 = v0.clone();
                        c550.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a555 = {
            let (h554, _) = v0.clone();
            h554
        };
        let a556 = 256u64;
        let a557 = 0u64;
        nat_quot(a555, a556, a557)
    }, f11(f222({
        let a559 = {
            let (h558, _) = v0;
            h558
        };
        let a560 = 256u64;
        let a561 = 0u64;
        nat_rem(a559, a560, a561)
    }), {
        let (_, h562) = v0;
        h562
    }))
}

pub fn f176(v0: Fn39, p566: Adt18, p567: List<Bytes>) -> R<Adt18> {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m563 = v2;
                match m563.uncons() {
                    None => {
                        Err::<(Adt18, List<Bytes>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Bytes>), R<Adt18>>(({
                            let c564: Fn39 = v0.clone();
                            c564.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: u64, v2: Adt18, v3: Bytes) -> R<Adt18> {
    f223(v2, v3, v0, v1)
}

pub fn f178(v0: Fn40, p571: bool, p572: List<Bytes>) -> bool {
    let m570 = (p571, p572);
    {
        let mut m570 = m570;
        loop {
            let step = {
                let (v1, v2) = m570;
                let m568 = v2;
                match m568.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c569: Fn40 = v0.clone();
                            c569.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a573 = v2;
        let a574 = v0;
        equal(a573, a574)
    }
}

pub fn f180(v0: Fn41, p578: List<Adt12>, p579: List<Adt12>) -> List<Adt12> {
    let m577 = (p578, p579);
    {
        let mut m577 = m577;
        loop {
            let step = {
                let (v1, v2) = m577;
                let m575 = v2;
                match m575.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c576: Fn41 = v0.clone();
                            c576.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(v1, v0)
}

pub fn f182(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(Adt12::C4, List::cons(v1, v0))
}

pub fn f183(v0: u64, v1: u64, v2: Adt13, v3: Adt14, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v3.clone() {
        Adt14::C0(h580, _, _) => {
            h580
        }
    };
    let v6: bool = match v3.clone() {
        Adt14::C0(_, h581, _) => {
            h581
        }
    };
    let m583 = match v3 {
        Adt14::C0(_, _, h582) => {
            (*h582).clone()
        }
    };
    match m583 {
        Adt8::C0 => {
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h584, _, _) => {
                    h584
                }
            }, match v2 {
                Adt13::C0(_, h585, _) => {
                    h585
                }
            }, v4))
        }
        Adt8::C1(v7) => {
            Ok(Adt13::C0(List::cons(v7.clone(), match v2.clone() {
                Adt13::C0(h586, _, _) => {
                    h586
                }
            }), {
                let a588 = match v2 {
                    Adt13::C0(_, h587, _) => {
                        h587
                    }
                };
                let a589 = f224(v7)?;
                nat_add(a588, a589)?
            }, v4))
        }
        Adt8::C2(r606, r607) => {
            let v8 = (*r606).clone();
            let v9 = (*r607).clone();
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h590, _, _) => {
                    h590
                }
            }, match v2 {
                Adt13::C0(_, h591, _) => {
                    h591
                }
            }, List::cons(Adt14::C0(v5, v6, std::rc::Rc::new(v8)), List::cons(Adt14::C0(v5, v6, std::rc::Rc::new(v9)), v4))))
        }
        Adt8::C3(v10, r608) => {
            let v11 = (*r608).clone();
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h592, _, _) => {
                    h592
                }
            }, match v2 {
                Adt13::C0(_, h593, _) => {
                    h593
                }
            }, List::cons(Adt14::C0({
                let a594 = v5;
                let a595 = v10;
                nat_add(a594, a595)?
            }, v6, std::rc::Rc::new(v11)), v4)))
        }
        Adt8::C4 => {
            if v6 {
                Ok(Adt13::C0(List::cons({
                    let a596 = Str::lit(" ");
                    utf8_encode(a596)
                }, match v2.clone() {
                    Adt13::C0(h597, _, _) => {
                        h597
                    }
                }), {
                    let a599 = match v2 {
                        Adt13::C0(_, h598, _) => {
                            h598
                        }
                    };
                    let a600 = 1u64;
                    nat_add(a599, a600)?
                }, v4))
            } else {
                f225(v2, v5, v4)
            }
        }
        Adt8::C5 => {
            if v6 {
                Ok(Adt13::C0(match v2.clone() {
                    Adt13::C0(h601, _, _) => {
                        h601
                    }
                }, match v2 {
                    Adt13::C0(_, h602, _) => {
                        h602
                    }
                }, v4))
            } else {
                f225(v2, v5, v4)
            }
        }
        Adt8::C6(r609) => {
            let v12 = (*r609).clone();
            let v13: bool = f226(v1, v0, match v2.clone() {
                Adt13::C0(_, h603, _) => {
                    h603
                }
            }, v4.clone(), Adt14::C0(v5, true, std::rc::Rc::new(v12.clone())))?;
            Ok(Adt13::C0(match v2.clone() {
                Adt13::C0(h604, _, _) => {
                    h604
                }
            }, match v2 {
                Adt13::C0(_, h605, _) => {
                    h605
                }
            }, List::cons(Adt14::C0(v5, v13, std::rc::Rc::new(v12)), v4)))
        }
    }
}

pub fn f184(v0: Fn42, p614: u64, p615: List<Bytes>) -> (List<Bytes>, bool) {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m610 = v1;
                if m610 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m610 - 1;
                    let m612 = {
                        let c611: Fn42 = v0.clone();
                        c611.apply(v2.clone())
                    };
                    match m612 {
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
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: List<Bytes>) -> Option<List<Bytes>> {
    f227(v0)
}

pub fn f186(v0: Fn43, p619: List<Bytes>, p620: List<Bytes>) -> List<Bytes> {
    let m618 = (p619, p620);
    {
        let mut m618 = m618;
        loop {
            let step = {
                let (v1, v2) = m618;
                let m616 = v2;
                match m616.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c617: Fn43 = v0.clone();
                            c617.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f188(v0: Bytes, v1: u64) -> R<u64> {
    let m621 = f228(v0, v1);
    match m621 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a666 = {
                let a642 = {
                    let a630 = {
                        let a624 = {
                            let a622 = v3;
                            let a623 = 128u8;
                            fixed_u8::bit_and(a622, a623)
                        };
                        let a625 = 128u8;
                        if equal(a624, a625) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a631 = {
                        let a628 = {
                            let a626 = v3;
                            let a627 = 64u8;
                            fixed_u8::bit_and(a626, a627)
                        };
                        let a629 = 64u8;
                        if equal(a628, a629) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a630, a631)?
                };
                let a643 = {
                    let a640 = {
                        let a634 = {
                            let a632 = v3;
                            let a633 = 32u8;
                            fixed_u8::bit_and(a632, a633)
                        };
                        let a635 = 32u8;
                        if equal(a634, a635) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a641 = {
                        let a638 = {
                            let a636 = v3;
                            let a637 = 16u8;
                            fixed_u8::bit_and(a636, a637)
                        };
                        let a639 = 16u8;
                        if equal(a638, a639) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a640, a641)?
                };
                nat_add(a642, a643)?
            };
            let a667 = {
                let a664 = {
                    let a652 = {
                        let a646 = {
                            let a644 = v3;
                            let a645 = 8u8;
                            fixed_u8::bit_and(a644, a645)
                        };
                        let a647 = 8u8;
                        if equal(a646, a647) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a653 = {
                        let a650 = {
                            let a648 = v3;
                            let a649 = 4u8;
                            fixed_u8::bit_and(a648, a649)
                        };
                        let a651 = 4u8;
                        if equal(a650, a651) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a652, a653)?
                };
                let a665 = {
                    let a662 = {
                        let a656 = {
                            let a654 = v3;
                            let a655 = 2u8;
                            fixed_u8::bit_and(a654, a655)
                        };
                        let a657 = 2u8;
                        if equal(a656, a657) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a663 = {
                        let a660 = {
                            let a658 = v3;
                            let a659 = 1u8;
                            fixed_u8::bit_and(a658, a659)
                        };
                        let a661 = 1u8;
                        if equal(a660, a661) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a662, a663)?
                };
                nat_add(a664, a665)?
            };
            nat_add(a666, a667)
        }
    }
}

pub fn f189(v0: u64) -> bool {
    if f40({
        let a668 = 65u64;
        let a669 = v0;
        nat_le(a668, a669)
    }, {
        let a670 = v0;
        let a671 = 90u64;
        nat_le(a670, a671)
    }) {
        true
    } else {
        f40({
            let a672 = 97u64;
            let a673 = v0;
            nat_le(a672, a673)
        }, {
            let a674 = v0;
            let a675 = 122u64;
            nat_le(a674, a675)
        })
    }
}

pub fn f190(v0: Bytes, v1: u64) -> R<u64> {
    if if f229(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f230(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f230(v0.clone(), v1, 194u8) {
            if if f230(v0.clone(), {
                let a676 = v1;
                let a677 = 1u64;
                nat_add(a676, a677)?
            }, 133u8) {
                true
            } else {
                f230(v0, {
                    let a678 = v1;
                    let a679 = 1u64;
                    nat_add(a678, a679)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f230(v0.clone(), v1, 225u8) {
                if if f230(v0.clone(), {
                    let a680 = v1;
                    let a681 = 1u64;
                    nat_add(a680, a681)?
                }, 154u8) {
                    f230(v0, {
                        let a682 = v1;
                        let a683 = 2u64;
                        nat_add(a682, a683)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f230(v0.clone(), v1, 226u8) {
                    if f230(v0.clone(), {
                        let a684 = v1;
                        let a685 = 1u64;
                        nat_add(a684, a685)?
                    }, 128u8) {
                        if if f229(v0.clone(), {
                            let a686 = v1;
                            let a687 = 2u64;
                            nat_add(a686, a687)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f230(v0.clone(), {
                                let a688 = v1;
                                let a689 = 2u64;
                                nat_add(a688, a689)?
                            }, 168u8) {
                                true
                            } else {
                                if f230(v0.clone(), {
                                    let a690 = v1;
                                    let a691 = 2u64;
                                    nat_add(a690, a691)?
                                }, 169u8) {
                                    true
                                } else {
                                    f230(v0, {
                                        let a692 = v1;
                                        let a693 = 2u64;
                                        nat_add(a692, a693)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f230(v0.clone(), {
                            let a694 = v1;
                            let a695 = 1u64;
                            nat_add(a694, a695)?
                        }, 129u8) {
                            f230(v0, {
                                let a696 = v1;
                                let a697 = 2u64;
                                nat_add(a696, a697)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f230(v0.clone(), v1, 227u8) {
                        if if f230(v0.clone(), {
                            let a698 = v1;
                            let a699 = 1u64;
                            nat_add(a698, a699)?
                        }, 128u8) {
                            f230(v0, {
                                let a700 = v1;
                                let a701 = 2u64;
                                nat_add(a700, a701)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f191(v0: Fn44, p706: u64, p707: u64) -> R<(u64, bool)> {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v1, v2) = m705;
                let m702 = v1;
                if m702 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m702 - 1;
                    let m704 = {
                        let c703: Fn44 = v0.clone();
                        c703.apply(v2)?
                    };
                    match m704 {
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
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f40({
        let a709 = v1;
        let a710 = {
            let a708 = v0.clone();
            length_bytes(a708)
        };
        nat_lt(a709, a710)
    }, f231(v0, v1)?) {
        Ok(Some({
            let a711 = v1;
            let a712 = 1u64;
            nat_add(a711, a712)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f193(v0: Fn45, p717: u64, p718: Adt19) -> R<(Adt19, bool)> {
    let m716 = (p717, p718);
    {
        let mut m716 = m716;
        loop {
            let step = {
                let (v1, v2) = m716;
                let m713 = v1;
                if m713 == 0 {
                    Err::<(u64, Adt19), R<(Adt19, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m713 - 1;
                    let m715 = {
                        let c714: Fn45 = v0.clone();
                        c714.apply(v2.clone())?
                    };
                    match m715 {
                        None => {
                            Err::<(u64, Adt19), R<(Adt19, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt19), R<(Adt19, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: Adt19) -> R<Option<Adt19>> {
    f232(v0, v1)
}

pub fn f195(v0: Bytes, v1: u64, v2: Adt16) -> R<Option<Adt16>> {
    let v3: u64 = match v2.clone() {
        Adt16::C0(h719, _) => {
            h719
        }
    };
    let a720 = v3;
    let a721 = v1;
    if nat_lt(a720, a721) {
        let v4: u64 = f233(v0.clone(), v3)?;
        let a722 = v4;
        let a723 = 32u64;
        if if nat_lt(a722, a723) {
            true
        } else {
            let a724 = v4;
            let a725 = 34u64;
            if nat_eq(a724, a725) {
                true
            } else {
                let a726 = v4;
                let a727 = 92u64;
                nat_eq(a726, a727)
            }
        } {
            Ok(Some(Adt16::C0({
                let a728 = v3;
                let a729 = 1u64;
                nat_add(a728, a729)?
            }, List::cons(f234(v4), match v2 {
                Adt16::C0(_, h730) => {
                    h730
                }
            }))))
        } else {
            let v5: u64 = f235(v0.clone(), v1, v3)?;
            Ok(Some(Adt16::C0(v5, List::cons(f236(v0, v3, {
                let a731 = v5;
                let a732 = v3;
                nat_sub(a731, a732)
            }), match v2 {
                Adt16::C0(_, h733) => {
                    h733
                }
            }))))
        }
    } else {
        Ok(None::<Adt16>)
    }
}

pub fn f196(v0: Bytes, v1: u64) -> Bytes {
    let m737 = {
        let a734 = v0;
        let a735 = v1;
        let a736 = 1u64;
        slice_bytes(a734, a735, a736)
    };
    match m737 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f197(v0: Fn46, p742: u64, p743: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m741 = (p742, p743);
    {
        let mut m741 = m741;
        loop {
            let step = {
                let (v1, v2) = m741;
                let m738 = v1;
                if m738 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m738 - 1;
                    let m740 = {
                        let c739: Fn46 = v0.clone();
                        c739.apply(v2.clone())
                    };
                    match m740 {
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
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f237(v1, v0, v2)
}

pub fn f199(v0: u64) -> bool {
    let a744 = 64u64;
    let a745 = v0;
    if if nat_lt(a744, a745) {
        let a746 = v0;
        let a747 = 91u64;
        nat_lt(a746, a747)
    } else {
        false
    } {
        true
    } else {
        let a748 = 96u64;
        let a749 = v0;
        if if nat_lt(a748, a749) {
            let a750 = v0;
            let a751 = 123u64;
            nat_lt(a750, a751)
        } else {
            false
        } {
            true
        } else {
            let a752 = v0;
            let a753 = 95u64;
            nat_eq(a752, a753)
        }
    }
}

pub fn f200(v0: Bytes, v1: u64) -> R<u64> {
    let m754 = f228(v0, v1);
    match m754 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a799 = {
                let a775 = {
                    let a763 = {
                        let a757 = {
                            let a755 = v3;
                            let a756 = 128u8;
                            fixed_u8::bit_and(a755, a756)
                        };
                        let a758 = 128u8;
                        if equal(a757, a758) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a764 = {
                        let a761 = {
                            let a759 = v3;
                            let a760 = 64u8;
                            fixed_u8::bit_and(a759, a760)
                        };
                        let a762 = 64u8;
                        if equal(a761, a762) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a763, a764)?
                };
                let a776 = {
                    let a773 = {
                        let a767 = {
                            let a765 = v3;
                            let a766 = 32u8;
                            fixed_u8::bit_and(a765, a766)
                        };
                        let a768 = 32u8;
                        if equal(a767, a768) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a774 = {
                        let a771 = {
                            let a769 = v3;
                            let a770 = 16u8;
                            fixed_u8::bit_and(a769, a770)
                        };
                        let a772 = 16u8;
                        if equal(a771, a772) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a773, a774)?
                };
                nat_add(a775, a776)?
            };
            let a800 = {
                let a797 = {
                    let a785 = {
                        let a779 = {
                            let a777 = v3;
                            let a778 = 8u8;
                            fixed_u8::bit_and(a777, a778)
                        };
                        let a780 = 8u8;
                        if equal(a779, a780) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a786 = {
                        let a783 = {
                            let a781 = v3;
                            let a782 = 4u8;
                            fixed_u8::bit_and(a781, a782)
                        };
                        let a784 = 4u8;
                        if equal(a783, a784) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a785, a786)?
                };
                let a798 = {
                    let a795 = {
                        let a789 = {
                            let a787 = v3;
                            let a788 = 2u8;
                            fixed_u8::bit_and(a787, a788)
                        };
                        let a790 = 2u8;
                        if equal(a789, a790) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a796 = {
                        let a793 = {
                            let a791 = v3;
                            let a792 = 1u8;
                            fixed_u8::bit_and(a791, a792)
                        };
                        let a794 = 1u8;
                        if equal(a793, a794) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a795, a796)?
                };
                nat_add(a797, a798)?
            };
            nat_add(a799, a800)
        }
    }
}

pub fn f201(v0: Bytes, v1: u64) -> R<u64> {
    let (h801, _) = f191(Fn44::F238(v0.clone()), {
        let a803 = {
            let a802 = v0;
            length_bytes(a802)
        };
        let a804 = 1u64;
        nat_add(a803, a804)?
    }, v1)?;
    Ok(h801)
}

pub fn f202(v0: Bytes) -> bool {
    let a805 = v0.clone();
    let a806 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a805, a806) {
        true
    } else {
        let a807 = v0;
        let a808 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a807, a808)
    }
}

pub fn f203(v0: Bytes) -> Bytes {
    f33(f222(f239(v0.clone())), v0)
}

pub fn f204(v0: Bytes, v1: Adt8) -> R<Adt8> {
    Ok(Adt8::C2(std::rc::Rc::new(Adt8::C1(f33(f93(v0)?, f94(Str::lit(": "))))), std::rc::Rc::new(Adt8::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt8::C0)))))
}

pub fn f205(v0: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    f240(Fn50::F241, List::<List<(Bytes, Adt8)>>::nil(), v0)
}

pub fn f206(v0: Fn47, p812: List<List<(Bytes, Adt8)>>, p813: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    let m811 = (p812, p813);
    {
        let mut m811 = m811;
        loop {
            let step = {
                let (v1, v2) = m811;
                let m809 = v2;
                match m809.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<(Bytes, Adt8)>), List<List<(Bytes, Adt8)>>>(({
                            let c810: Fn47 = v0.clone();
                            c810.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: List<List<(Bytes, Adt8)>>, v1: (Bytes, Adt8)) -> List<List<(Bytes, Adt8)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt8)>::nil()), v0)
}

pub fn f208(v0: Fn48, p818: u64, p819: List<List<(Bytes, Adt8)>>) -> R<(List<List<(Bytes, Adt8)>>, bool)> {
    let m817 = (p818, p819);
    {
        let mut m817 = m817;
        loop {
            let step = {
                let (v1, v2) = m817;
                let m814 = v1;
                if m814 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m814 - 1;
                    let m816 = {
                        let c815: Fn48 = v0.clone();
                        c815.apply(v2.clone())?
                    };
                    match m816 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt8)>>), R<(List<List<(Bytes, Adt8)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    f242(v0)
}

pub fn f210(v0: u64) -> bool {
    f40({
        let a820 = v0;
        let a821 = 1114111u64;
        nat_le(a820, a821)
    }, !f40({
        let a822 = 55296u64;
        let a823 = v0;
        nat_le(a822, a823)
    }, {
        let a824 = v0;
        let a825 = 57343u64;
        nat_le(a824, a825)
    }))
}

pub fn f211(v0: Adt20) -> Bytes {
    let a827 = match v0.clone() {
        Adt20::C0(_, _, h826) => {
            h826
        }
    };
    let a828 = 0u64;
    if nat_eq(a827, a828) {
        if match v0.clone() {
            Adt20::C0(_, h829, _) => {
                h829
            }
        } {
            f2(Str::lit("0"))
        } else {
            match v0 {
                Adt20::C0(h830, _, _) => {
                    h830
                }
            }
        }
    } else {
        if match v0.clone() {
            Adt20::C0(_, h831, _) => {
                h831
            }
        } {
            f51(match v0 {
                Adt20::C0(_, _, h832) => {
                    h832
                }
            })
        } else {
            f11(f11(match v0.clone() {
                Adt20::C0(h833, _, _) => {
                    h833
                }
            }, f2(Str::lit("+"))), f51(match v0 {
                Adt20::C0(_, _, h834) => {
                    h834
                }
            }))
        }
    }
}

pub fn f212(v0: Adt2) -> R<Adt20> {
    let m835 = v0;
    match m835 {
        Adt2::C0 => {
            Ok(Adt20::C0(f2(Str::lit("")), true, 0u64))
        }
        Adt2::C1(r841) => {
            let v2 = (*r841).clone();
            let v3: Adt20 = f212(v2)?;
            Ok(Adt20::C0(match v3.clone() {
                Adt20::C0(h836, _, _) => {
                    h836
                }
            }, match v3.clone() {
                Adt20::C0(_, h837, _) => {
                    h837
                }
            }, {
                let a839 = match v3 {
                    Adt20::C0(_, _, h838) => {
                        h838
                    }
                };
                let a840 = 1u64;
                nat_add(a839, a840)?
            }))
        }
        Adt2::C2(r842, r843) => {
            let v4 = (*r842).clone();
            let v5 = (*r843).clone();
            let v6: Adt20 = f212(v4)?;
            let v7: Adt20 = f212(v5)?;
            Ok(Adt20::C0(f11(f11(f11(f11(f2(Str::lit("max(")), f211(v6)), f2(Str::lit(", "))), f211(v7)), f2(Str::lit(")"))), false, 0u64))
        }
        Adt2::C3(r844, r845) => {
            let v8 = (*r844).clone();
            let v9 = (*r845).clone();
            let v10: Adt20 = f212(v8)?;
            let v11: Adt20 = f212(v9)?;
            Ok(Adt20::C0(f11(f11(f11(f11(f2(Str::lit("imax(")), f211(v10)), f2(Str::lit(", "))), f211(v11)), f2(Str::lit(")"))), false, 0u64))
        }
        Adt2::C4(v1) => {
            Ok(Adt20::C0(f36(v1)?, false, 0u64))
        }
    }
}

pub fn f213(v0: Fn49, p849: List<Bytes>, p850: List<Adt2>) -> R<List<Bytes>> {
    let m848 = (p849, p850);
    {
        let mut m848 = m848;
        loop {
            let step = {
                let (v1, v2) = m848;
                let m846 = v2;
                match m846.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Adt2>), R<List<Bytes>>>(({
                            let c847: Fn49 = v0.clone();
                            c847.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: List<Bytes>, v1: Adt2) -> R<List<Bytes>> {
    Ok(List::cons(f156(v1)?, v0))
}

pub fn f215(v0: Adt12) -> Adt12 {
    f66(List::cons(f67(f2(Str::lit("("))), List::cons(v0, List::cons(f67(f2(Str::lit(")"))), List::<Adt12>::nil()))))
}

pub fn f216(v0: Bytes, v1: u64) -> R<u64> {
    let (h851, _) = f191(Fn44::F243(v0.clone()), {
        let a853 = {
            let a852 = v0;
            length_bytes(a852)
        };
        let a854 = 1u64;
        nat_add(a853, a854)?
    }, v1)?;
    Ok(h851)
}

pub fn f217(v0: Bytes, v1: Bytes) -> bool {
    let m859 = {
        let a856 = v0;
        let a857 = 0u64;
        let a858 = {
            let a855 = v1.clone();
            length_bytes(a855)
        };
        slice_bytes(a856, a857, a858)
    };
    match m859 {
        None => {
            false
        }
        Some(v2) => {
            let a860 = v2;
            let a861 = v1;
            equal(a860, a861)
        }
    }
}

pub fn f218(v0: u64) -> R<Bytes> {
    let a862 = v0;
    let a863 = 128u64;
    if nat_lt(a862, a863) {
        Ok(f222(v0))
    } else {
        let a864 = v0;
        let a865 = 2048u64;
        if nat_lt(a864, a865) {
            Ok(f11(f222({
                let a869 = 192u64;
                let a870 = {
                    let a866 = v0;
                    let a867 = 64u64;
                    let a868 = 0u64;
                    nat_quot(a866, a867, a868)
                };
                nat_add(a869, a870)?
            }), f222({
                let a874 = 128u64;
                let a875 = {
                    let a871 = v0;
                    let a872 = 64u64;
                    let a873 = 0u64;
                    nat_rem(a871, a872, a873)
                };
                nat_add(a874, a875)?
            })))
        } else {
            let a876 = v0;
            let a877 = 65536u64;
            if nat_lt(a876, a877) {
                Ok(f11(f11(f222({
                    let a881 = 224u64;
                    let a882 = {
                        let a878 = v0;
                        let a879 = 4096u64;
                        let a880 = 0u64;
                        nat_quot(a878, a879, a880)
                    };
                    nat_add(a881, a882)?
                }), f222({
                    let a889 = 128u64;
                    let a890 = {
                        let a886 = {
                            let a883 = v0;
                            let a884 = 64u64;
                            let a885 = 0u64;
                            nat_quot(a883, a884, a885)
                        };
                        let a887 = 64u64;
                        let a888 = 0u64;
                        nat_rem(a886, a887, a888)
                    };
                    nat_add(a889, a890)?
                })), f222({
                    let a894 = 128u64;
                    let a895 = {
                        let a891 = v0;
                        let a892 = 64u64;
                        let a893 = 0u64;
                        nat_rem(a891, a892, a893)
                    };
                    nat_add(a894, a895)?
                })))
            } else {
                Ok(f11(f11(f11(f222({
                    let a899 = 240u64;
                    let a900 = {
                        let a896 = v0;
                        let a897 = 262144u64;
                        let a898 = 0u64;
                        nat_quot(a896, a897, a898)
                    };
                    nat_add(a899, a900)?
                }), f222({
                    let a907 = 128u64;
                    let a908 = {
                        let a904 = {
                            let a901 = v0;
                            let a902 = 4096u64;
                            let a903 = 0u64;
                            nat_quot(a901, a902, a903)
                        };
                        let a905 = 64u64;
                        let a906 = 0u64;
                        nat_rem(a904, a905, a906)
                    };
                    nat_add(a907, a908)?
                })), f222({
                    let a915 = 128u64;
                    let a916 = {
                        let a912 = {
                            let a909 = v0;
                            let a910 = 64u64;
                            let a911 = 0u64;
                            nat_quot(a909, a910, a911)
                        };
                        let a913 = 64u64;
                        let a914 = 0u64;
                        nat_rem(a912, a913, a914)
                    };
                    nat_add(a915, a916)?
                })), f222({
                    let a920 = 128u64;
                    let a921 = {
                        let a917 = v0;
                        let a918 = 64u64;
                        let a919 = 0u64;
                        nat_rem(a917, a918, a919)
                    };
                    nat_add(a920, a921)?
                })))
            }
        }
    }
}

pub fn f219(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f220(v0: Adt4) -> R<Bytes> {
    Ok(f11(f11(f36(match v0.clone() {
        Adt4::C0(h922, _) => {
            h922
        }
    })?, f2(Str::lit(" := "))), {
        let m924 = match v0 {
            Adt4::C0(_, h923) => {
                (*h923).clone()
            }
        };
        match m924 {
            Adt5::C0(v1) => {
                if v1 {
                    f2(Str::lit("bool true"))
                } else {
                    f2(Str::lit("bool false"))
                }
            }
            Adt5::C1(v2) => {
                f11(f2(Str::lit("nat ")), if f165(v2.clone())? {
                    v2
                } else {
                    f76(v2)?
                })
            }
            Adt5::C2(v3) => {
                f11(f2(Str::lit("int ")), if f244(v3.clone())? {
                    v3
                } else {
                    f76(v3)?
                })
            }
            Adt5::C3(v4) => {
                f11(f2(Str::lit("str ")), f76(v4)?)
            }
            Adt5::C4(v5) => {
                f11(f2(Str::lit("name ")), f36(v5)?)
            }
        }
    }))
}

pub fn f221(v0: List<List<(Bytes, u64)>>) -> R<List<List<(Bytes, u64)>>> {
    Ok(f112({
        let (_, h925) = {
            let (h926, _) = f245(Fn51::F246, {
                let a928 = {
                    let a927 = v0.clone();
                    length_list(a927)
                };
                let a929 = 1u64;
                nat_add(a928, a929)?
            }, (v0, List::<List<(Bytes, u64)>>::nil()))?;
            h926
        };
        h925
    }))
}

pub fn f222(v0: u64) -> Bytes {
    f236(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f223(v0: Adt18, v1: Bytes, v2: Bytes, v3: u64) -> R<Adt18> {
    Ok(Adt18::C0({
        let a931 = match v0.clone() {
            Adt18::C0(h930, _) => {
                h930
            }
        };
        let a932 = 1u64;
        nat_add(a931, a932)?
    }, if match v0.clone() {
        Adt18::C0(_, h933) => {
            h933
        }
    } {
        true
    } else {
        f40({
            let a935 = match v0 {
                Adt18::C0(h934, _) => {
                    h934
                }
            };
            let a936 = v3;
            nat_lt(a935, a936)
        }, f1(v1, v2))
    }))
}

pub fn f224(v0: Bytes) -> R<u64> {
    let (_, h937) = {
        let (h938, _) = f247(Fn52::F248(v0.clone()), {
            let a939 = v0;
            length_bytes(a939)
        }, (0u64, 0u64))?;
        h938
    };
    Ok(h937)
}

pub fn f225(v0: Adt13, v1: u64, v2: List<Adt14>) -> R<Adt13> {
    Ok(Adt13::C0(f249(match v0 {
        Adt13::C0(h940, _, _) => {
            h940
        }
    }, v1)?, v1, v2))
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: List<Adt14>, v4: Adt14) -> R<bool> {
    let v5: Adt21 = {
        let (h941, _) = f250(Fn53::F251, {
            let a942 = v0;
            let a943 = 1u64;
            nat_add(a942, a943)?
        }, Adt21::C0(0u64, {
            let a944 = v1;
            let a945 = v2;
            nat_sub(a944, a945)
        }, {
            let a946 = v1;
            let a947 = v2;
            nat_lt(a946, a947)
        }, List::cons(v4, List::<Adt14>::nil()), v3, None::<bool>))?;
        h941
    };
    let m949 = match v5 {
        Adt21::C0(_, _, _, _, _, h948) => {
            h948
        }
    };
    match m949 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f227(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m950 = v0.clone();
    match m950.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m951 = v2;
            match m951.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f252(Fn54::F253, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m953 = {
                        let (h952, _) = v5;
                        h952
                    };
                    match m953 {
                        None => {
                            Some(f126({
                                let (_, h955) = v5;
                                h955
                            }))
                        }
                        Some(v6) => {
                            Some(f126(List::cons(v6, {
                                let (_, h954) = v5;
                                h954
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: u64) -> Option<u8> {
    let a956 = v0;
    let a957 = v1;
    index_bytes(a956, a957)
}

pub fn f229(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m958 = f228(v0, v1);
    match m958 {
        None => {
            false
        }
        Some(v4) => {
            f254(v4, v2, v3)
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m959 = f228(v0, v1);
    match m959 {
        None => {
            false
        }
        Some(v3) => {
            let a960 = v3;
            let a961 = v2;
            equal(a960, a961)
        }
    }
}

pub fn f231(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f188(v0.clone(), v1)?;
    if if f189(v2) {
        true
    } else {
        f255(v2)
    } {
        Ok(true)
    } else {
        if f119(f236(v0.clone(), v1, 1u64), List::cons(f2(Str::lit("_")), List::cons(f2(Str::lit("'")), List::cons(f2(Str::lit("!")), List::cons(f2(Str::lit("?")), List::cons(f2(Str::lit(".")), List::cons(f2(Str::lit("@")), List::cons(f2(Str::lit("#")), List::<Bytes>::nil())))))))) {
            Ok(true)
        } else {
            let a962 = v2;
            let a963 = 128u64;
            if nat_lt(a962, a963) {
                Ok(false)
            } else {
                let a964 = v2;
                let a965 = 192u64;
                if nat_lt(a964, a965) {
                    Ok(true)
                } else {
                    let a966 = f190(v0, v1)?;
                    let a967 = 0u64;
                    Ok(nat_eq(a966, a967))
                }
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: Adt19) -> R<Option<Adt19>> {
    let v2: u64 = match v1.clone() {
        Adt19::C0(h968, _) => {
            h968
        }
    };
    if if match v1 {
        Adt19::C0(_, h969) => {
            h969
        }
    } {
        true
    } else {
        let a971 = {
            let a970 = v0.clone();
            length_bytes(a970)
        };
        let a972 = v2;
        nat_le(a971, a972)
    } {
        Ok(None::<Adt19>)
    } else {
        if f255(f188(v0.clone(), v2)?) {
            let v3: u64 = f216(v0.clone(), v2)?;
            let v4: u64 = f188(v0.clone(), {
                let a973 = v2;
                let a974 = 1u64;
                nat_sub(a973, a974)
            })?;
            let v5: bool = f40({
                let a975 = 0u64;
                let a976 = v2;
                nat_lt(a975, a976)
            }, if f189(v4) {
                true
            } else {
                let a977 = v4;
                let a978 = 95u64;
                if nat_eq(a977, a978) {
                    true
                } else {
                    let a979 = v4;
                    let a980 = 39u64;
                    nat_eq(a979, a980)
                }
            });
            Ok(Some(Adt19::C0(v3, f40({
                let a983 = 1u64;
                let a984 = {
                    let a981 = v3;
                    let a982 = v2;
                    nat_sub(a981, a982)
                };
                nat_lt(a983, a984)
            }, f40({
                let a985 = f188(v0, v2)?;
                let a986 = 48u64;
                nat_eq(a985, a986)
            }, !v5)))))
        } else {
            Ok(Some(Adt19::C0({
                let a987 = v2;
                let a988 = 1u64;
                nat_add(a987, a988)?
            }, false)))
        }
    }
}

pub fn f233(v0: Bytes, v1: u64) -> R<u64> {
    let m989 = f228(v0, v1);
    match m989 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1034 = {
                let a1010 = {
                    let a998 = {
                        let a992 = {
                            let a990 = v3;
                            let a991 = 128u8;
                            fixed_u8::bit_and(a990, a991)
                        };
                        let a993 = 128u8;
                        if equal(a992, a993) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a999 = {
                        let a996 = {
                            let a994 = v3;
                            let a995 = 64u8;
                            fixed_u8::bit_and(a994, a995)
                        };
                        let a997 = 64u8;
                        if equal(a996, a997) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a998, a999)?
                };
                let a1011 = {
                    let a1008 = {
                        let a1002 = {
                            let a1000 = v3;
                            let a1001 = 32u8;
                            fixed_u8::bit_and(a1000, a1001)
                        };
                        let a1003 = 32u8;
                        if equal(a1002, a1003) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1009 = {
                        let a1006 = {
                            let a1004 = v3;
                            let a1005 = 16u8;
                            fixed_u8::bit_and(a1004, a1005)
                        };
                        let a1007 = 16u8;
                        if equal(a1006, a1007) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1008, a1009)?
                };
                nat_add(a1010, a1011)?
            };
            let a1035 = {
                let a1032 = {
                    let a1020 = {
                        let a1014 = {
                            let a1012 = v3;
                            let a1013 = 8u8;
                            fixed_u8::bit_and(a1012, a1013)
                        };
                        let a1015 = 8u8;
                        if equal(a1014, a1015) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1021 = {
                        let a1018 = {
                            let a1016 = v3;
                            let a1017 = 4u8;
                            fixed_u8::bit_and(a1016, a1017)
                        };
                        let a1019 = 4u8;
                        if equal(a1018, a1019) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1020, a1021)?
                };
                let a1033 = {
                    let a1030 = {
                        let a1024 = {
                            let a1022 = v3;
                            let a1023 = 2u8;
                            fixed_u8::bit_and(a1022, a1023)
                        };
                        let a1025 = 2u8;
                        if equal(a1024, a1025) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1031 = {
                        let a1028 = {
                            let a1026 = v3;
                            let a1027 = 1u8;
                            fixed_u8::bit_and(a1026, a1027)
                        };
                        let a1029 = 1u8;
                        if equal(a1028, a1029) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1030, a1031)?
                };
                nat_add(a1032, a1033)?
            };
            nat_add(a1034, a1035)
        }
    }
}

pub fn f234(v0: u64) -> Bytes {
    let a1036 = v0;
    let a1037 = 34u64;
    if nat_eq(a1036, a1037) {
        f135(Str::lit("\\\""))
    } else {
        let a1038 = v0;
        let a1039 = 92u64;
        if nat_eq(a1038, a1039) {
            f135(Str::lit("\\\\"))
        } else {
            let a1040 = v0;
            let a1041 = 8u64;
            if nat_eq(a1040, a1041) {
                f135(Str::lit("\\b"))
            } else {
                let a1042 = v0;
                let a1043 = 12u64;
                if nat_eq(a1042, a1043) {
                    f135(Str::lit("\\f"))
                } else {
                    let a1044 = v0;
                    let a1045 = 10u64;
                    if nat_eq(a1044, a1045) {
                        f135(Str::lit("\\n"))
                    } else {
                        let a1046 = v0;
                        let a1047 = 13u64;
                        if nat_eq(a1046, a1047) {
                            f135(Str::lit("\\r"))
                        } else {
                            let a1048 = v0;
                            let a1049 = 9u64;
                            if nat_eq(a1048, a1049) {
                                f135(Str::lit("\\t"))
                            } else {
                                f134(f135(Str::lit("\\u00")), f256(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f235(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1050, _) = f191(Fn44::F257(v1, v0), {
        let a1051 = v1;
        let a1052 = v2;
        nat_sub(a1051, a1052)
    }, v2)?;
    Ok(h1050)
}

pub fn f236(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1056 = {
        let a1053 = v0;
        let a1054 = v1;
        let a1055 = v2;
        slice_bytes(a1053, a1054, a1055)
    };
    match m1056 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f237(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1057, _) = v2;
        h1057
    };
    let a1058 = v3;
    let a1059 = 0u64;
    if nat_eq(a1058, a1059) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1060 = v3;
            let a1061 = v1;
            let a1062 = 0u64;
            nat_quot(a1060, a1061, a1062)
        }, {
            let a1067 = f196(v0, {
                let a1063 = v3;
                let a1064 = v1;
                let a1065 = 0u64;
                nat_rem(a1063, a1064, a1065)
            });
            let a1068 = {
                let (_, h1066) = v2;
                h1066
            };
            append_bytes(a1067, a1068)
        }))
    }
}

pub fn f238(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f258(v0, v1)
}

pub fn f239(v0: Bytes) -> u64 {
    let a1069 = v0.clone();
    let a1070 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a1069, a1070) {
        0u64
    } else {
        let a1071 = v0.clone();
        let a1072 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a1071, a1072) {
            1u64
        } else {
            let a1073 = v0.clone();
            let a1074 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a1073, a1074) {
                2u64
            } else {
                let a1075 = v0.clone();
                let a1076 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a1075, a1076) {
                    3u64
                } else {
                    let a1077 = v0.clone();
                    let a1078 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a1077, a1078) {
                        4u64
                    } else {
                        let a1079 = v0.clone();
                        let a1080 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a1079, a1080) {
                            5u64
                        } else {
                            let a1081 = v0.clone();
                            let a1082 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a1081, a1082) {
                                6u64
                            } else {
                                let a1083 = v0.clone();
                                let a1084 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a1083, a1084) {
                                    7u64
                                } else {
                                    let a1085 = v0.clone();
                                    let a1086 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a1085, a1086) {
                                        8u64
                                    } else {
                                        let a1087 = v0.clone();
                                        let a1088 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a1087, a1088) {
                                            9u64
                                        } else {
                                            let a1089 = v0.clone();
                                            let a1090 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a1089, a1090) {
                                                10u64
                                            } else {
                                                let a1091 = v0;
                                                let a1092 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a1091, a1092) {
                                                    11u64
                                                } else {
                                                    12u64
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

pub fn f240(v0: Fn50, p1096: List<List<(Bytes, Adt8)>>, p1097: List<List<(Bytes, Adt8)>>) -> List<List<(Bytes, Adt8)>> {
    let m1095 = (p1096, p1097);
    {
        let mut m1095 = m1095;
        loop {
            let step = {
                let (v1, v2) = m1095;
                let m1093 = v2;
                match m1093.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), List<List<(Bytes, Adt8)>>>(({
                            let c1094: Fn50 = v0.clone();
                            c1094.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1095 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: List<List<(Bytes, Adt8)>>, v1: List<(Bytes, Adt8)>) -> List<List<(Bytes, Adt8)>> {
    List::cons(v1, v0)
}

pub fn f242(v0: List<List<(Bytes, Adt8)>>) -> R<Option<List<List<(Bytes, Adt8)>>>> {
    let a1099 = {
        let a1098 = v0.clone();
        length_list(a1098)
    };
    let a1100 = 1u64;
    if nat_le(a1099, a1100) {
        Ok(None::<List<List<(Bytes, Adt8)>>>)
    } else {
        Ok(Some(f259(v0)?))
    }
}

pub fn f243(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f40({
        let a1102 = v1;
        let a1103 = {
            let a1101 = v0.clone();
            length_bytes(a1101)
        };
        nat_lt(a1102, a1103)
    }, f255(f188(v0, v1)?)) {
        Ok(Some({
            let a1104 = v1;
            let a1105 = 1u64;
            nat_add(a1104, a1105)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f244(v0: Bytes) -> R<bool> {
    let v1: Bytes = if f217(v0.clone(), f2(Str::lit("-"))) {
        f236(v0.clone(), 1u64, {
            let a1107 = {
                let a1106 = v0.clone();
                length_bytes(a1106)
            };
            let a1108 = 1u64;
            nat_sub(a1107, a1108)
        })
    } else {
        v0.clone()
    };
    Ok(f40(f165(v1)?, !f1(v0, f2(Str::lit("-0")))))
}

pub fn f245(v0: Fn51, p1113: u64, p1114: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)> {
    let m1112 = (p1113, p1114);
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, v2) = m1112;
                let m1109 = v1;
                if m1109 == 0 {
                    Err::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1109 - 1;
                    let m1111 = {
                        let c1110: Fn51 = v0.clone();
                        c1110.apply(v2.clone())?
                    };
                    match m1111 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)), R<((List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
    f260(v0)
}

pub fn f247(v0: Fn52, p1119: u64, p1120: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1118 = (p1119, p1120);
    {
        let mut m1118 = m1118;
        loop {
            let step = {
                let (v1, v2) = m1118;
                let m1115 = v1;
                if m1115 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1115 - 1;
                    let m1117 = {
                        let c1116: Fn52 = v0.clone();
                        c1116.apply(v2)?
                    };
                    match m1117 {
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
                Ok(next) => { m1118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f248(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f261(v0, v1)
}

pub fn f249(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a1121 = v1;
        let a1122 = 256u64;
        let a1123 = 0u64;
        nat_quot(a1121, a1122, a1123)
    };
    let v3: List<Bytes> = {
        let (_, h1124) = {
            let (h1125, _) = f262(Fn55::F263(v2), {
                let a1126 = v2;
                let a1127 = 1u64;
                nat_add(a1126, a1127)?
            }, (0u64, List::cons({
                let a1128 = Str::lit("\u{a}");
                utf8_encode(a1128)
            }, v0)))?;
            h1125
        };
        h1124
    };
    Ok(List::cons(f236(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a1129 = v1;
        let a1130 = 256u64;
        let a1131 = 0u64;
        nat_rem(a1129, a1130, a1131)
    }), v3))
}

pub fn f250(v0: Fn53, p1136: u64, p1137: Adt21) -> R<(Adt21, bool)> {
    let m1135 = (p1136, p1137);
    {
        let mut m1135 = m1135;
        loop {
            let step = {
                let (v1, v2) = m1135;
                let m1132 = v1;
                if m1132 == 0 {
                    Err::<(u64, Adt21), R<(Adt21, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1132 - 1;
                    let m1134 = {
                        let c1133: Fn53 = v0.clone();
                        c1133.apply(v2.clone())?
                    };
                    match m1134 {
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
                Ok(next) => { m1135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Adt21) -> R<Option<Adt21>> {
    f264(v0)
}

pub fn f252(v0: Fn54, p1141: (Option<Bytes>, List<Bytes>), p1142: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1140 = (p1141, p1142);
    {
        let mut m1140 = m1140;
        loop {
            let step = {
                let (v1, v2) = m1140;
                let m1138 = v2;
                match m1138.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1139: Fn54 = v0.clone();
                            c1139.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f265(v0, v1)
}

pub fn f254(v0: u8, v1: u8, v2: u8) -> bool {
    let m1145 = {
        let a1143 = v0;
        let a1144 = v1;
        fixed_u8::checked_sub(a1143, a1144)
    };
    match m1145 {
        None => {
            false
        }
        Some(_) => {
            let m1148 = {
                let a1146 = v2;
                let a1147 = v0;
                fixed_u8::checked_sub(a1146, a1147)
            };
            match m1148 {
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

pub fn f255(v0: u64) -> bool {
    f40({
        let a1149 = 48u64;
        let a1150 = v0;
        nat_le(a1149, a1150)
    }, {
        let a1151 = v0;
        let a1152 = 57u64;
        nat_le(a1151, a1152)
    })
}

pub fn f256(v0: u64) -> Bytes {
    f134(f266(v0, 16u64), f267(v0))
}

pub fn f257(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f268(v1, v0, v2)
}

pub fn f258(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f269(f200(v0, v1)?) {
        Ok(Some({
            let a1153 = v1;
            let a1154 = 1u64;
            nat_add(a1153, a1154)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f259(v0: List<List<(Bytes, Adt8)>>) -> R<List<List<(Bytes, Adt8)>>> {
    Ok(f205({
        let (_, h1155) = {
            let (h1156, _) = f270(Fn56::F271, {
                let a1158 = {
                    let a1157 = v0.clone();
                    length_list(a1157)
                };
                let a1159 = 1u64;
                nat_add(a1158, a1159)?
            }, (v0, List::<List<(Bytes, Adt8)>>::nil()))?;
            h1156
        };
        h1155
    }))
}

pub fn f260(v0: (List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)) -> R<Option<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>> {
    let v1: List<List<(Bytes, u64)>> = {
        let (h1160, _) = v0;
        h1160
    };
    let v2: List<List<(Bytes, u64)>> = {
        let (_, h1161) = v0;
        h1161
    };
    let m1162 = v1;
    match m1162.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, u64)>>, List<List<(Bytes, u64)>>)>)
        }
        Some((v3, v4)) => {
            let m1163 = v4;
            match m1163.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, u64)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f272(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f261(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1165 = f228(v0, {
        let (h1164, _) = v1;
        h1164
    });
    match m1165 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1210 = {
                    let a1186 = {
                        let a1174 = {
                            let a1168 = {
                                let a1166 = v3;
                                let a1167 = 128u8;
                                fixed_u8::bit_and(a1166, a1167)
                            };
                            let a1169 = 128u8;
                            if equal(a1168, a1169) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1175 = {
                            let a1172 = {
                                let a1170 = v3;
                                let a1171 = 64u8;
                                fixed_u8::bit_and(a1170, a1171)
                            };
                            let a1173 = 64u8;
                            if equal(a1172, a1173) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1174, a1175)?
                    };
                    let a1187 = {
                        let a1184 = {
                            let a1178 = {
                                let a1176 = v3;
                                let a1177 = 32u8;
                                fixed_u8::bit_and(a1176, a1177)
                            };
                            let a1179 = 32u8;
                            if equal(a1178, a1179) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1185 = {
                            let a1182 = {
                                let a1180 = v3;
                                let a1181 = 16u8;
                                fixed_u8::bit_and(a1180, a1181)
                            };
                            let a1183 = 16u8;
                            if equal(a1182, a1183) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1184, a1185)?
                    };
                    nat_add(a1186, a1187)?
                };
                let a1211 = {
                    let a1208 = {
                        let a1196 = {
                            let a1190 = {
                                let a1188 = v3;
                                let a1189 = 8u8;
                                fixed_u8::bit_and(a1188, a1189)
                            };
                            let a1191 = 8u8;
                            if equal(a1190, a1191) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1197 = {
                            let a1194 = {
                                let a1192 = v3;
                                let a1193 = 4u8;
                                fixed_u8::bit_and(a1192, a1193)
                            };
                            let a1195 = 4u8;
                            if equal(a1194, a1195) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1196, a1197)?
                    };
                    let a1209 = {
                        let a1206 = {
                            let a1200 = {
                                let a1198 = v3;
                                let a1199 = 2u8;
                                fixed_u8::bit_and(a1198, a1199)
                            };
                            let a1201 = 2u8;
                            if equal(a1200, a1201) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1207 = {
                            let a1204 = {
                                let a1202 = v3;
                                let a1203 = 1u8;
                                fixed_u8::bit_and(a1202, a1203)
                            };
                            let a1205 = 1u8;
                            if equal(a1204, a1205) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1206, a1207)?
                    };
                    nat_add(a1208, a1209)?
                };
                nat_add(a1210, a1211)?
            };
            let a1212 = v4;
            let a1213 = 128u64;
            if nat_lt(a1212, a1213) {
                Ok(Some(({
                    let a1215 = {
                        let (h1214, _) = v1;
                        h1214
                    };
                    let a1216 = 1u64;
                    nat_add(a1215, a1216)?
                }, {
                    let a1218 = {
                        let (_, h1217) = v1;
                        h1217
                    };
                    let a1219 = 1u64;
                    nat_add(a1218, a1219)?
                })))
            } else {
                let a1220 = v4;
                let a1221 = 192u64;
                if nat_lt(a1220, a1221) {
                    Ok(Some(({
                        let a1223 = {
                            let (h1222, _) = v1;
                            h1222
                        };
                        let a1224 = 1u64;
                        nat_add(a1223, a1224)?
                    }, {
                        let (_, h1225) = v1;
                        h1225
                    })))
                } else {
                    Ok(Some(({
                        let a1227 = {
                            let (h1226, _) = v1;
                            h1226
                        };
                        let a1228 = 1u64;
                        nat_add(a1227, a1228)?
                    }, {
                        let a1230 = {
                            let (_, h1229) = v1;
                            h1229
                        };
                        let a1231 = 1u64;
                        nat_add(a1230, a1231)?
                    })))
                }
            }
        }
    }
}

pub fn f262(v0: Fn55, p1236: u64, p1237: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m1235 = (p1236, p1237);
    {
        let mut m1235 = m1235;
        loop {
            let step = {
                let (v1, v2) = m1235;
                let m1232 = v1;
                if m1232 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1232 - 1;
                    let m1234 = {
                        let c1233: Fn55 = v0.clone();
                        c1233.apply(v2.clone())?
                    };
                    match m1234 {
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
                Ok(next) => { m1235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f273(v0, v1)
}

pub fn f264(v0: Adt21) -> R<Option<Adt21>> {
    let m1239 = match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1238) => {
            h1238
        }
    };
    match m1239 {
        None => {
            if if match v0.clone() {
                Adt21::C0(_, _, h1240, _, _, _) => {
                    h1240
                }
            } {
                true
            } else {
                let a1243 = match v0.clone() {
                    Adt21::C0(_, h1241, _, _, _, _) => {
                        h1241
                    }
                };
                let a1244 = match v0.clone() {
                    Adt21::C0(h1242, _, _, _, _, _) => {
                        h1242
                    }
                };
                nat_lt(a1243, a1244)
            } {
                Ok(Some(f274(v0.clone(), match v0.clone() {
                    Adt21::C0(h1245, _, _, _, _, _) => {
                        h1245
                    }
                }, match v0.clone() {
                    Adt21::C0(_, _, _, h1246, _, _) => {
                        h1246
                    }
                }, match v0 {
                    Adt21::C0(_, _, _, _, h1247, _) => {
                        h1247
                    }
                }, Some(false))))
            } else {
                let m1249 = match v0.clone() {
                    Adt21::C0(_, _, _, h1248, _, _) => {
                        h1248
                    }
                };
                match m1249.uncons() {
                    None => {
                        let m1252 = match v0.clone() {
                            Adt21::C0(_, _, _, _, h1251, _) => {
                                h1251
                            }
                        };
                        match m1252.uncons() {
                            None => {
                                Ok(Some(f274(v0.clone(), match v0 {
                                    Adt21::C0(h1253, _, _, _, _, _) => {
                                        h1253
                                    }
                                }, List::<Adt14>::nil(), List::<Adt14>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f275(v0, v4, List::<Adt14>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f275(v0.clone(), v2, v3, match v0 {
                            Adt21::C0(_, _, _, _, h1250, _) => {
                                h1250
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt21>)
        }
    }
}

pub fn f265(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1255 = {
        let (h1254, _) = v0;
        h1254
    };
    match m1255 {
        None => {
            (Some(v1), {
                let (_, h1256) = v0;
                h1256
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1257 = v2;
                let a1258 = v1;
                append_bytes(a1257, a1258)
            }, {
                let (_, h1259) = v0;
                h1259
            }))
        }
    }
}

pub fn f266(v0: u64, v1: u64) -> Bytes {
    f267({
        let a1260 = v0;
        let a1261 = v1;
        let a1262 = 0u64;
        nat_quot(a1260, a1261, a1262)
    })
}

pub fn f267(v0: u64) -> Bytes {
    f236(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1263 = v0;
        let a1264 = 16u64;
        let a1265 = 0u64;
        nat_rem(a1263, a1264, a1265)
    }, 1u64)
}

pub fn f268(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1266 = v2;
    let a1267 = v1;
    if nat_lt(a1266, a1267) {
        let v3: u64 = f233(v0, v2)?;
        let a1268 = v3;
        let a1269 = 32u64;
        if nat_lt(a1268, a1269) {
            Ok(None::<u64>)
        } else {
            let a1270 = v3;
            let a1271 = 34u64;
            if nat_eq(a1270, a1271) {
                Ok(None::<u64>)
            } else {
                let a1272 = v3;
                let a1273 = 92u64;
                if nat_eq(a1272, a1273) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1274 = v2;
                        let a1275 = 1u64;
                        nat_add(a1274, a1275)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f269(v0: u64) -> bool {
    if f199(v0) {
        true
    } else {
        let a1276 = 47u64;
        let a1277 = v0;
        if if nat_lt(a1276, a1277) {
            let a1278 = v0;
            let a1279 = 58u64;
            nat_lt(a1278, a1279)
        } else {
            false
        } {
            true
        } else {
            let a1280 = v0;
            let a1281 = 46u64;
            if nat_eq(a1280, a1281) {
                true
            } else {
                let a1282 = v0;
                let a1283 = 39u64;
                if nat_eq(a1282, a1283) {
                    true
                } else {
                    let a1284 = v0;
                    let a1285 = 47u64;
                    if nat_eq(a1284, a1285) {
                        true
                    } else {
                        let a1286 = v0;
                        let a1287 = 45u64;
                        nat_eq(a1286, a1287)
                    }
                }
            }
        }
    }
}

pub fn f270(v0: Fn56, p1292: u64, p1293: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)> {
    let m1291 = (p1292, p1293);
    {
        let mut m1291 = m1291;
        loop {
            let step = {
                let (v1, v2) = m1291;
                let m1288 = v1;
                if m1288 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1288 - 1;
                    let m1290 = {
                        let c1289: Fn56 = v0.clone();
                        c1289.apply(v2.clone())?
                    };
                    match m1290 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)), R<((List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    f276(v0)
}

pub fn f272(v0: List<(Bytes, u64)>, v1: List<(Bytes, u64)>) -> R<List<(Bytes, u64)>> {
    let v2: u64 = {
        let a1298 = {
            let a1296 = {
                let a1294 = v0.clone();
                length_list(a1294)
            };
            let a1297 = {
                let a1295 = v1.clone();
                length_list(a1295)
            };
            nat_add(a1296, a1297)?
        };
        let a1299 = 1u64;
        nat_add(a1298, a1299)?
    };
    Ok(f61({
        let (_, h1300) = {
            let (_, h1301) = {
                let (h1302, _) = f277(Fn57::F278, v2, (v0, (v1, List::<(Bytes, u64)>::nil())));
                h1302
            };
            h1301
        };
        h1300
    }))
}

pub fn f273(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a1304 = {
        let (h1303, _) = v1.clone();
        h1303
    };
    let a1305 = v0;
    if nat_lt(a1304, a1305) {
        Ok(Some(({
            let a1307 = {
                let (h1306, _) = v1;
                h1306
            };
            let a1308 = 1u64;
            nat_add(a1307, a1308)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h1309) = v1;
            h1309
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f274(v0: Adt21, v1: u64, v2: List<Adt14>, v3: List<Adt14>, v4: Option<bool>) -> Adt21 {
    Adt21::C0(v1, match v0.clone() {
        Adt21::C0(_, h1310, _, _, _, _) => {
            h1310
        }
    }, match v0 {
        Adt21::C0(_, _, h1311, _, _, _) => {
            h1311
        }
    }, v2, v3, v4)
}

pub fn f275(v0: Adt21, v1: Adt14, v2: List<Adt14>, v3: List<Adt14>) -> R<Adt21> {
    let v4: u64 = match v1.clone() {
        Adt14::C0(h1312, _, _) => {
            h1312
        }
    };
    let v5: bool = match v1.clone() {
        Adt14::C0(_, h1313, _) => {
            h1313
        }
    };
    let v6: u64 = match v0.clone() {
        Adt21::C0(h1314, _, _, _, _, _) => {
            h1314
        }
    };
    let m1316 = match v1 {
        Adt14::C0(_, _, h1315) => {
            (*h1315).clone()
        }
    };
    match m1316 {
        Adt8::C0 => {
            Ok(f274(v0, v6, v2, v3, None::<bool>))
        }
        Adt8::C1(v7) => {
            Ok(f274(v0, {
                let a1317 = v6;
                let a1318 = f224(v7)?;
                nat_add(a1317, a1318)?
            }, v2, v3, None::<bool>))
        }
        Adt8::C2(r1323, r1324) => {
            let v8 = (*r1323).clone();
            let v9 = (*r1324).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0(v4, v5, std::rc::Rc::new(v8)), List::cons(Adt14::C0(v4, v5, std::rc::Rc::new(v9)), v2)), v3, None::<bool>))
        }
        Adt8::C3(v10, r1325) => {
            let v11 = (*r1325).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0({
                let a1319 = v4;
                let a1320 = v10;
                nat_add(a1319, a1320)?
            }, v5, std::rc::Rc::new(v11)), v2), v3, None::<bool>))
        }
        Adt8::C4 => {
            if v5 {
                Ok(f274(v0, {
                    let a1321 = v6;
                    let a1322 = 1u64;
                    nat_add(a1321, a1322)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f274(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt8::C5 => {
            if v5 {
                Ok(f274(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f274(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt8::C6(r1326) => {
            let v12 = (*r1326).clone();
            Ok(f274(v0, v6, List::cons(Adt14::C0(v4, true, std::rc::Rc::new(v12)), v2), v3, None::<bool>))
        }
    }
}

pub fn f276(v0: (List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)) -> R<Option<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>> {
    let v1: List<List<(Bytes, Adt8)>> = {
        let (h1327, _) = v0;
        h1327
    };
    let v2: List<List<(Bytes, Adt8)>> = {
        let (_, h1328) = v0;
        h1328
    };
    let m1329 = v1;
    match m1329.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt8)>>, List<List<(Bytes, Adt8)>>)>)
        }
        Some((v3, v4)) => {
            let m1330 = v4;
            match m1330.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt8)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f279(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f277(v0: Fn57, p1335: u64, p1336: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool) {
    let m1334 = (p1335, p1336);
    {
        let mut m1334 = m1334;
        loop {
            let step = {
                let (v1, v2) = m1334;
                let m1331 = v1;
                if m1331 == 0 {
                    Err::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v2, false))
                } else {
                    let v3 = m1331 - 1;
                    let m1333 = {
                        let c1332: Fn57 = v0.clone();
                        c1332.apply(v2.clone())
                    };
                    match m1333 {
                        None => {
                            Err::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))), ((List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
    f280(v0)
}

pub fn f279(v0: List<(Bytes, Adt8)>, v1: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    let v2: u64 = {
        let a1341 = {
            let a1339 = {
                let a1337 = v0.clone();
                length_list(a1337)
            };
            let a1340 = {
                let a1338 = v1.clone();
                length_list(a1338)
            };
            nat_add(a1339, a1340)?
        };
        let a1342 = 1u64;
        nat_add(a1341, a1342)?
    };
    f281({
        let (_, h1343) = {
            let (_, h1344) = {
                let (h1345, _) = f282(Fn58::F283, v2, (v0, (v1, List::<(Bytes, Adt8)>::nil())));
                h1345
            };
            h1344
        };
        h1343
    })
}

pub fn f280(v0: (List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))) -> Option<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))> {
    let v1: List<(Bytes, u64)> = {
        let (h1346, _) = v0.clone();
        h1346
    };
    let v2: List<(Bytes, u64)> = {
        let (h1347, _) = {
            let (_, h1348) = v0.clone();
            h1348
        };
        h1347
    };
    let v3: List<(Bytes, u64)> = {
        let (_, h1349) = {
            let (_, h1350) = v0;
            h1350
        };
        h1349
    };
    let m1351 = v1.clone();
    match m1351.uncons() {
        None => {
            let m1352 = v2;
            match m1352.uncons() {
                None => {
                    None::<(List<(Bytes, u64)>, (List<(Bytes, u64)>, List<(Bytes, u64)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, u64)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1353 = v2.clone();
            match m1353.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, u64)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h1354, _) = v8.clone();
                        h1354
                    }, {
                        let (h1355, _) = v6.clone();
                        h1355
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

pub fn f281(v0: List<(Bytes, Adt8)>) -> R<List<(Bytes, Adt8)>> {
    f146(Fn32::F285, List::<(Bytes, Adt8)>::nil(), v0)
}

pub fn f282(v0: Fn58, p1360: u64, p1361: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool) {
    let m1359 = (p1360, p1361);
    {
        let mut m1359 = m1359;
        loop {
            let step = {
                let (v1, v2) = m1359;
                let m1356 = v1;
                if m1356 == 0 {
                    Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, false))
                } else {
                    let v3 = m1356 - 1;
                    let m1358 = {
                        let c1357: Fn58 = v0.clone();
                        c1357.apply(v2.clone())
                    };
                    match m1358 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))), ((List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    f286(v0)
}

pub fn f284(v0: Bytes, v1: Bytes) -> bool {
    let a1366 = {
        let a1362 = v0;
        let a1363 = v1;
        compare_bytes(a1362, a1363)
    };
    let a1367 = {
        let a1364 = Bytes::lit(&[]);
        let a1365 = Bytes::lit(&[0x01]);
        compare_bytes(a1364, a1365)
    };
    equal(a1366, a1367)
}

pub fn f285(v0: List<(Bytes, Adt8)>, v1: (Bytes, Adt8)) -> List<(Bytes, Adt8)> {
    List::cons(v1, v0)
}

pub fn f286(v0: (List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))) -> Option<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))> {
    let v1: List<(Bytes, Adt8)> = {
        let (h1368, _) = v0.clone();
        h1368
    };
    let v2: List<(Bytes, Adt8)> = {
        let (h1369, _) = {
            let (_, h1370) = v0.clone();
            h1370
        };
        h1369
    };
    let v3: List<(Bytes, Adt8)> = {
        let (_, h1371) = {
            let (_, h1372) = v0;
            h1372
        };
        h1371
    };
    let m1373 = v1.clone();
    match m1373.uncons() {
        None => {
            let m1374 = v2;
            match m1374.uncons() {
                None => {
                    None::<(List<(Bytes, Adt8)>, (List<(Bytes, Adt8)>, List<(Bytes, Adt8)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt8)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1375 = v2.clone();
            match m1375.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt8)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f284({
                        let (h1376, _) = v8.clone();
                        h1376
                    }, {
                        let (h1377, _) = v6.clone();
                        h1377
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

pub fn run(p0: Adt0) -> R<Result<Bytes, Adt7>> {
    f0(p0)
}
