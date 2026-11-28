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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4(std::rc::Rc<Adt0>),
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
}

#[derive(Clone)]
pub enum Adt3 {
    C0(std::rc::Rc<Adt0>, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(std::rc::Rc<Adt0>, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(std::rc::Rc<Adt0>, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C3(u64, std::rc::Rc<Adt6>),
    C4,
    C5,
    C6(std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(List<Bytes>, u64, List<Adt8>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Option<Adt0>),
    C1(Bytes),
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, bool, List<Adt8>, List<Adt8>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F10(std::rc::Rc<Adt2>, std::rc::Rc<Adt0>),
}

impl Fn0 {
    pub fn apply(&self, p0: (bool, Bytes), p1: Adt0) -> R<(bool, Bytes)> {
        match self {
            Fn0::F10(k0, k1) => f10((**k0).clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt3) -> Option<Adt3> {
        match self {
            Fn1::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn2::F17 => f17(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F27,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt5) -> Option<Adt5> {
        match self {
            Fn3::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F42,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn4::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F48(std::rc::Rc<Adt0>),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn5::F48(k0) => f48((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F51(u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn6::F51(k0, k1) => f51(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F72,
    F114(Bytes, Bytes),
    F120,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt6) -> List<Adt6> {
        match self {
            Fn7::F72 => f72(p0, p1),
            Fn7::F114(k0, k1) => f114(k0.clone(), k1.clone(), p0, p1),
            Fn7::F120 => f120(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F84(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn8::F84(k0) => f84(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F105,
}

impl Fn9 {
    pub fn apply(&self, p0: Adt6, p1: Adt6) -> Adt6 {
        match self {
            Fn9::F105 => f105(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F107(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt6, p1: (Bytes, Adt6)) -> Adt6 {
        match self {
            Fn10::F107(k0) => f107(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F116(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (bool, List<Adt6>), p1: Adt6) -> (bool, List<Adt6>) {
        match self {
            Fn11::F116(k0) => f116(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F119,
}

impl Fn12 {
    pub fn apply(&self, p0: (bool, Adt6), p1: Adt6) -> (bool, Adt6) {
        match self {
            Fn12::F119 => f119(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F123(Option<Bytes>),
    F244,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Adt6)>, p1: (Bytes, Adt6)) -> R<List<(Bytes, Adt6)>> {
        match self {
            Fn13::F123(k0) => f123(k0.clone(), p0, p1),
            Fn13::F244 => Ok(f244(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F125,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt6>, p1: (Bytes, Adt6)) -> List<Adt6> {
        match self {
            Fn14::F125 => f125(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F130,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn15::F130 => f130(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F132,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn16::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F143,
    F144,
    F149,
    F150,
    F151,
    F215,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt0) -> R<bool> {
        match self {
            Fn17::F143 => f143(p0),
            Fn17::F144 => f144(p0),
            Fn17::F149 => f149(p0),
            Fn17::F150 => f150(p0),
            Fn17::F151 => f151(p0),
            Fn17::F215 => f215(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F153(u64, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn18::F153(k0, k1) => f153(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F158,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Adt6>, p1: Adt0) -> List<Adt6> {
        match self {
            Fn19::F158 => f158(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F165(Bytes, std::rc::Rc<Adt0>),
}

impl Fn20 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn20::F165(k0, k1) => f165(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F167(std::rc::Rc<Adt0>),
    F188(Fn17),
    F220(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn21::F167(k0) => f167((**k0).clone(), p0, p1),
            Fn21::F188(k0) => f188(k0.clone(), p0, p1),
            Fn21::F220(k0) => Ok(f220(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F175,
}

impl Fn22 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn22::F175 => f175(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F177,
}

impl Fn23 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
        match self {
            Fn23::F177 => f177(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F194(Bytes),
    F214(bool, Bytes),
    F229(u64, Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn24::F194(k0) => f194(k0.clone(), p0),
            Fn24::F214(k0, k1) => f214(*k0, k1.clone(), p0),
            Fn24::F229(k0, k1) => f229(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F198(u64, Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn25::F198(k0, k1) => f198(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F202,
}

impl Fn26 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt6)>>, p1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
        match self {
            Fn26::F202 => f202(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F205(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn27::F205(k0) => f205(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F208,
}

impl Fn28 {
    pub fn apply(&self, p0: Adt14) -> R<Option<Adt14>> {
        match self {
            Fn28::F208 => f208(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F210,
}

impl Fn29 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn29::F210 => f210(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F212(std::rc::Rc<Adt0>),
    F213(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn30::F212(k0) => f212((**k0).clone(), p0, p1),
            Fn30::F213(k0) => f213(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F225(u64),
}

impl Fn31 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn31::F225(k0) => f225(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F232,
}

impl Fn32 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
        match self {
            Fn32::F232 => f232(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F243,
}

impl Fn33 {
    pub fn apply(&self, p0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
        match self {
            Fn33::F243 => f243(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Adt0) -> R<Result<Bytes, Adt1>> {
    let m1 = f1(v1);
    match m1 {
        None => {
            Ok(f12(f4(Str::lit("a module is an object"))))
        }
        Some(v2) => {
            let m2 = f2(f3(v2.clone(), f4(Str::lit("declarations"))));
            match m2 {
                None => {
                    Ok(f12(f4(Str::lit("a module has a `declarations` array"))))
                }
                Some(v3) => {
                    if f5({
                        let a3 = f6(v2.clone())?;
                        let a4 = 2u64;
                        nat_eq(a3, a4)
                    }, {
                        let m5 = f7(f3(v2, f4(Str::lit("spec"))));
                        match m5 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }) {
                        let v5: Adt2 = f8(v0.clone(), 0u64)?;
                        Ok(Ok::<Bytes, Adt1>({
                            let (_, h6) = f9(Fn0::F10(std::rc::Rc::new(v5), std::rc::Rc::new(v0)), (true, f4(Str::lit(""))), f11(v3))?;
                            h6
                        }))
                    } else {
                        Ok(f12(f4(Str::lit("a module has exactly a `spec` and its `declarations`"))))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Adt0) -> Option<Adt0> {
    let m7 = v0;
    match m7 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(_) => {
            None::<Adt0>
        }
        Adt0::C4(r8) => {
            let v1 = (*r8).clone();
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f2(v0: Option<Adt0>) -> Option<Adt0> {
    let m9 = v0;
    match m9 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f13(v1)
        }
    }
}

pub fn f3(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m12 = {
        let (h11, _) = f14(Fn1::F15(v1), 1000000u64, Adt3::C0(std::rc::Rc::new(v0), None::<Adt0>));
        h11
    };
    match m12 {
        Adt3::C0(_, h10) => {
            h10
        }
    }
}

pub fn f4(v0: Str) -> Bytes {
    let a13 = v0;
    utf8_encode(a13)
}

pub fn f5(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f6(v0: Adt0) -> R<u64> {
    let m16 = {
        let (h15, _) = f16(Fn2::F17, 1000000u64, Adt4::C0(std::rc::Rc::new(v0), 0u64))?;
        h15
    };
    match m16 {
        Adt4::C0(_, h14) => {
            Ok(h14)
        }
    }
}

pub fn f7(v0: Option<Adt0>) -> Option<Bytes> {
    let m17 = v0;
    match m17 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f18(v1)
        }
    }
}

pub fn f8(v0: Adt0, v1: u64) -> R<Adt2> {
    let a18 = v1;
    let a19 = 0u64;
    if nat_eq(a18, a19) {
        let m20 = f19(v0.clone(), f4(Str::lit("properties")));
        match m20 {
            None => {
                Ok(Adt2::C5)
            }
            Some(v2) => {
                let m21 = f19(v2, f4(Str::lit("declarations")));
                match m21 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v3) => {
                        f20(v0, v3)
                    }
                }
            }
        }
    } else {
        let a22 = v1;
        let a23 = 1u64;
        if nat_eq(a22, a23) {
            Ok(Adt2::C0)
        } else {
            let a24 = v1;
            let a25 = 2u64;
            if nat_eq(a24, a25) {
                Ok(Adt2::C1)
            } else {
                let m26 = f21(v0, f4(Str::lit("proof")));
                match m26 {
                    None => {
                        Ok(Adt2::C5)
                    }
                    Some(v4) => {
                        Ok(Adt2::C4(std::rc::Rc::new(v4)))
                    }
                }
            }
        }
    }
}

pub fn f9(v0: Fn0, p30: (bool, Bytes), p31: List<Adt0>) -> R<(bool, Bytes)> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v2;
                match m27.uncons() {
                    None => {
                        Err::<((bool, Bytes), List<Adt0>), R<(bool, Bytes)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Bytes), List<Adt0>), R<(bool, Bytes)>>(({
                            let c28: Fn0 = v0.clone();
                            c28.apply(v1, v3)?
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

pub fn f10(v0: Adt2, v1: Adt0, v2: (bool, Bytes), v3: Adt0) -> R<(bool, Bytes)> {
    Ok((false, f22({
        let (h32, _) = v2;
        if h32 {
            let (_, h33) = v2;
            h33
        } else {
            f22({
                let (_, h34) = v2;
                h34
            }, f4(Str::lit("\u{a}")))
        }
    }, f22(f23(f24(v1, v3, v0)?)?, f4(Str::lit("\u{a}"))))))
}

pub fn f11(v0: Adt0) -> List<Adt0> {
    f25({
        let m37 = {
            let (h36, _) = f26(Fn3::F27, 1000000u64, Adt5::C0(std::rc::Rc::new(v0), List::<Adt0>::nil()));
            h36
        };
        match m37 {
            Adt5::C0(_, h35) => {
                h35
            }
        }
    })
}

pub fn f12(v0: Bytes) -> Result<Bytes, Adt1> {
    Err::<Bytes, Adt1>(Adt1::C0(0u64, v0))
}

pub fn f13(v0: Adt0) -> Option<Adt0> {
    let m38 = v0;
    match m38 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r39) => {
            let v1 = (*r39).clone();
            Some(v1)
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6(_, _) => {
            None::<Adt0>
        }
        Adt0::C7(_, _, _) => {
            None::<Adt0>
        }
    }
}

pub fn f14(v0: Fn1, p44: u64, p45: Adt3) -> (Adt3, bool) {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m40 = v1;
                if m40 == 0 {
                    Err::<(u64, Adt3), (Adt3, bool)>((v2, false))
                } else {
                    let v3 = m40 - 1;
                    let m42 = {
                        let c41: Fn1 = v0.clone();
                        c41.apply(v2.clone())
                    };
                    match m42 {
                        None => {
                            Err::<(u64, Adt3), (Adt3, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), (Adt3, bool)>((v3, v4))
                        }
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

pub fn f15(v0: Bytes, v1: Adt3) -> Option<Adt3> {
    f28(v0, v1)
}

pub fn f16(v0: Fn2, p50: u64, p51: Adt4) -> R<(Adt4, bool)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m46 - 1;
                    let m48 = {
                        let c47: Fn2 = v0.clone();
                        c47.apply(v2.clone())?
                    };
                    match m48 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
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

pub fn f17(v0: Adt4) -> R<Option<Adt4>> {
    f29(v0)
}

pub fn f18(v0: Adt0) -> Option<Bytes> {
    let m52 = v0;
    match m52 {
        Adt0::C0(_) => {
            None::<Bytes>
        }
        Adt0::C1(_, _) => {
            None::<Bytes>
        }
        Adt0::C2(v1) => {
            Some(v1)
        }
        Adt0::C3(_) => {
            None::<Bytes>
        }
        Adt0::C4(_) => {
            None::<Bytes>
        }
        Adt0::C5 => {
            None::<Bytes>
        }
        Adt0::C6(_, _) => {
            None::<Bytes>
        }
        Adt0::C7(_, _, _) => {
            None::<Bytes>
        }
    }
}

pub fn f19(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m53 = f1(v0);
    match m53 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f3(v2, v1)
        }
    }
}

pub fn f20(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m54 = f19(f30(v0.clone(), v1)?, f4(Str::lit("items")));
    match m54 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v2) => {
            f31(v0, v2)
        }
    }
}

pub fn f21(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m55 = f19(v0, f4(Str::lit("$defs")));
    match m55 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f19(v2, v1)
        }
    }
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    let a56 = v0;
    let a57 = v1;
    append_bytes(a56, a57)
}

pub fn f23(v0: Adt6) -> R<Bytes> {
    f32(v0.clone(), 100u64, f33(v0)?)
}

pub fn f24(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt6> {
    let m58 = v1;
    match m58 {
        Adt0::C0(v5) => {
            f40(Adt0::C0(v5))
        }
        Adt0::C1(v6, v7) => {
            f40(Adt0::C1(v6, v7))
        }
        Adt0::C2(v8) => {
            f40(Adt0::C2(v8))
        }
        Adt0::C3(r59) => {
            let v4 = (*r59).clone();
            Ok(f37(v2.clone(), f38(v0.clone(), v4, f39(v0, v2)?)?))
        }
        Adt0::C4(r60) => {
            let v3 = (*r60).clone();
            f34(v3.clone(), v2.clone(), f35(v0.clone(), v3.clone(), f36(v0, v3, v2)?)?)
        }
        Adt0::C5 => {
            Ok(Adt6::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt6::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt6::C0)
        }
    }
}

pub fn f25(v0: List<Adt0>) -> List<Adt0> {
    f41(Fn4::F42, List::<Adt0>::nil(), v0)
}

pub fn f26(v0: Fn3, p65: u64, p66: Adt5) -> (Adt5, bool) {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m61 = v1;
                if m61 == 0 {
                    Err::<(u64, Adt5), (Adt5, bool)>((v2, false))
                } else {
                    let v3 = m61 - 1;
                    let m63 = {
                        let c62: Fn3 = v0.clone();
                        c62.apply(v2.clone())
                    };
                    match m63 {
                        None => {
                            Err::<(u64, Adt5), (Adt5, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), (Adt5, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt5) -> Option<Adt5> {
    f43(v0)
}

pub fn f28(v0: Bytes, v1: Adt3) -> Option<Adt3> {
    let m68 = match v1.clone() {
        Adt3::C0(_, h67) => {
            h67
        }
    };
    match m68 {
        None => {
            let m70 = f44(match v1 {
                Adt3::C0(h69, _) => {
                    (*h69).clone()
                }
            });
            match m70 {
                None => {
                    None::<Adt3>
                }
                Some(v3) => {
                    if f45(match v3.clone() {
                        Adt10::C0(h71, _, _) => {
                            h71
                        }
                    }, v0) {
                        Some(Adt3::C0(std::rc::Rc::new(Adt0::C5), Some(match v3 {
                            Adt10::C0(_, h72, _) => {
                                (*h72).clone()
                            }
                        })))
                    } else {
                        Some(Adt3::C0(std::rc::Rc::new(match v3 {
                            Adt10::C0(_, _, h73) => {
                                (*h73).clone()
                            }
                        }), None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt3>
        }
    }
}

pub fn f29(v0: Adt4) -> R<Option<Adt4>> {
    let m75 = f44(match v0.clone() {
        Adt4::C0(h74, _) => {
            (*h74).clone()
        }
    });
    match m75 {
        None => {
            let m81 = f46(match v0.clone() {
                Adt4::C0(h80, _) => {
                    (*h80).clone()
                }
            });
            match m81 {
                None => {
                    Ok(None::<Adt4>)
                }
                Some(v2) => {
                    Ok(Some(Adt4::C0(std::rc::Rc::new(match v2 {
                        Adt11::C0(_, h82) => {
                            (*h82).clone()
                        }
                    }), {
                        let a84 = match v0 {
                            Adt4::C0(_, h83) => {
                                h83
                            }
                        };
                        let a85 = 1u64;
                        nat_add(a84, a85)?
                    })))
                }
            }
        }
        Some(v1) => {
            Ok(Some(Adt4::C0(std::rc::Rc::new(match v1 {
                Adt10::C0(_, _, h76) => {
                    (*h76).clone()
                }
            }), {
                let a78 = match v0 {
                    Adt4::C0(_, h77) => {
                        h77
                    }
                };
                let a79 = 1u64;
                nat_add(a78, a79)?
            })))
        }
    }
}

pub fn f30(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h86, _) = f47(Fn5::F48(std::rc::Rc::new(v0)), 4096u64, v1)?;
    Ok(h86)
}

pub fn f31(v0: Adt0, v1: Adt0) -> R<Adt2> {
    let m87 = f7(f19(v1.clone(), f4(Str::lit("$ref"))));
    match m87 {
        None => {
            Ok(Adt2::C4(std::rc::Rc::new(v1)))
        }
        Some(v2) => {
            let v3: Bytes = f49(v2)?;
            if f45(v3.clone(), f4(Str::lit("term"))) {
                Ok(Adt2::C0)
            } else {
                if f45(v3.clone(), f4(Str::lit("type"))) {
                    Ok(Adt2::C1)
                } else {
                    if f45(v3.clone(), f4(Str::lit("member"))) {
                        Ok(Adt2::C2)
                    } else {
                        if f45(v3, f4(Str::lit("parameter"))) {
                            Ok(Adt2::C3)
                        } else {
                            Ok(Adt2::C4(std::rc::Rc::new(f30(v0, v1)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt6, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt7 = {
        let (h88, _) = f50(Fn6::F51(v2, v1), {
            let a89 = v2;
            let a90 = 1u64;
            nat_add(a89, a90)?
        }, Adt7::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt8::C0(0u64, false, std::rc::Rc::new(v0)), List::<Adt8>::nil())))?;
        h88
    };
    Ok(f52(match v3 {
        Adt7::C0(h91, _, _) => {
            h91
        }
    }))
}

pub fn f33(v0: Adt6) -> R<u64> {
    let m92 = v0;
    match m92 {
        Adt6::C0 => {
            Ok(1u64)
        }
        Adt6::C1(_) => {
            Ok(1u64)
        }
        Adt6::C2(r101, r102) => {
            let v2 = (*r101).clone();
            let v3 = (*r102).clone();
            let v4: u64 = f33(v2)?;
            let v5: u64 = f33(v3)?;
            let a95 = {
                let a93 = 1u64;
                let a94 = v4;
                nat_add(a93, a94)?
            };
            let a96 = v5;
            nat_add(a95, a96)
        }
        Adt6::C3(_, r103) => {
            let v7 = (*r103).clone();
            let v8: u64 = f33(v7)?;
            let a97 = 1u64;
            let a98 = v8;
            nat_add(a97, a98)
        }
        Adt6::C4 => {
            Ok(1u64)
        }
        Adt6::C5 => {
            Ok(1u64)
        }
        Adt6::C6(r104) => {
            let v9 = (*r104).clone();
            let v10: u64 = f33(v9)?;
            let a99 = 1u64;
            let a100 = v10;
            nat_add(a99, a100)
        }
    }
}

pub fn f34(v0: Adt0, v1: Adt2, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m105 = v1;
    match m105 {
        Adt2::C0 => {
            if f53(v0.clone())? {
                f54(v0.clone(), {
                    let m106 = f55(v0);
                    match m106 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }, v2)
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C1 => {
            if f57(v0.clone())? {
                f58(v0.clone(), {
                    let m107 = f55(v0);
                    match m107 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v4) => {
                            v4
                        }
                    }
                }, v2)
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C2 => {
            let m108 = f59(Adt0::C4(std::rc::Rc::new(v0.clone())))?;
            match m108 {
                None => {
                    f40(Adt0::C4(std::rc::Rc::new(v0)))
                }
                Some(v5) => {
                    Ok(f60(v5))
                }
            }
        }
        Adt2::C3 => {
            if f61(Adt0::C4(std::rc::Rc::new(v0.clone())))? {
                Ok(f62(List::cons(f60(f22(f63(v0, f4(Str::lit("name"))), f4(Str::lit(" : ")))), List::cons(f64(v2, f4(Str::lit("type"))), List::<Adt6>::nil()))))
            } else {
                f56(v0, v2)
            }
        }
        Adt2::C4(_) => {
            f56(v0, v2)
        }
        Adt2::C5 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C6 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C7 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C8 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C9 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C10 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C11 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C12 => {
            Ok(f62(List::cons(f60(f22(f63(v0, f4(Str::lit("field"))), f4(Str::lit(" := ")))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
        }
        Adt2::C13 => {
            f40(Adt0::C4(std::rc::Rc::new(v0)))
        }
        Adt2::C14 => {
            f65(v0, v2)
        }
    }
}

pub fn f35(v0: Adt0, v1: Adt0, v2: Adt9) -> R<List<(Bytes, Adt6)>> {
    let m109 = v1;
    match m109 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C7(v3, r110, r111) => {
            let v4 = (*r110).clone();
            let v5 = (*r111).clone();
            if f66(v2.clone()) {
                Ok(List::<(Bytes, Adt6)>::nil())
            } else {
                Ok(List::cons((v3.clone(), f24(v0.clone(), v4, f67(v0.clone(), v2.clone(), v3)?)?), f35(v0, v5, v2)?))
            }
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Adt9> {
    let m112 = v2.clone();
    match m112 {
        Adt2::C0 => {
            if f53(v1.clone())? {
                Ok(Adt9::C1({
                    let m113 = f55(v1);
                    match m113 {
                        None => {
                            f4(Str::lit(""))
                        }
                        Some(v3) => {
                            v3
                        }
                    }
                }))
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C1 => {
            if f57(v1.clone())? {
                Ok(Adt9::C2)
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C2 => {
            Ok(Adt9::C6)
        }
        Adt2::C3 => {
            if f61(Adt0::C4(std::rc::Rc::new(v1.clone())))? {
                Ok(Adt9::C3)
            } else {
                Ok(Adt9::C0(f68(v0, v1, v2)?))
            }
        }
        Adt2::C4(_) => {
            Ok(Adt9::C0(f68(v0, v1, v2)?))
        }
        Adt2::C5 => {
            Ok(Adt9::C6)
        }
        Adt2::C6 => {
            Ok(Adt9::C6)
        }
        Adt2::C7 => {
            Ok(Adt9::C6)
        }
        Adt2::C8 => {
            Ok(Adt9::C6)
        }
        Adt2::C9 => {
            Ok(Adt9::C6)
        }
        Adt2::C10 => {
            Ok(Adt9::C6)
        }
        Adt2::C11 => {
            Ok(Adt9::C6)
        }
        Adt2::C12 => {
            Ok(Adt9::C4)
        }
        Adt2::C13 => {
            Ok(Adt9::C6)
        }
        Adt2::C14 => {
            Ok(Adt9::C5)
        }
    }
}

pub fn f37(v0: Adt2, v1: List<Adt6>) -> Adt6 {
    let m114 = v0;
    match m114 {
        Adt2::C0 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C1 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C2 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C3 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C4(_) => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C5 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C6 => {
            f62(f69(v1, f4(Str::lit(" ")), f4(Str::lit(""))))
        }
        Adt2::C7 => {
            f62(List::cons(f60(f4(Str::lit("["))), {
                let a115 = f70(v1, f4(Str::lit(", ")));
                let a116 = List::cons(f60(f4(Str::lit("]"))), List::<Adt6>::nil());
                append_list(a115, a116)
            }))
        }
        Adt2::C8 => {
            f62(f69(v1, f4(Str::lit("(")), f4(Str::lit(") "))))
        }
        Adt2::C9 => {
            f62(List::cons(f60(f4(Str::lit("("))), {
                let a117 = f70(v1, f4(Str::lit(" ")));
                let a118 = List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil());
                append_list(a117, a118)
            }))
        }
        Adt2::C10 => {
            f62(f69(v1, f4(Str::lit(" ")), f4(Str::lit(""))))
        }
        Adt2::C11 => {
            f62(List::cons(f60(f4(Str::lit("{"))), {
                let a119 = f70(v1, f4(Str::lit(", ")));
                let a120 = List::cons(f60(f4(Str::lit("}"))), List::<Adt6>::nil());
                append_list(a119, a120)
            }))
        }
        Adt2::C12 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
        Adt2::C13 => {
            f62(f71(Fn7::F72, List::<Adt6>::nil(), v1))
        }
        Adt2::C14 => {
            f73(f4(Str::lit("[")), f4(Str::lit("]")), f74(v1))
        }
    }
}

pub fn f38(v0: Adt0, v1: Adt0, v2: Adt2) -> R<List<Adt6>> {
    let m121 = v1;
    match m121 {
        Adt0::C0(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C6(r122, r123) => {
            let v3 = (*r122).clone();
            let v4 = (*r123).clone();
            Ok(List::cons(f24(v0.clone(), v3, v2.clone())?, f38(v0, v4, v2)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt6>::nil())
        }
    }
}

pub fn f39(v0: Adt0, v1: Adt2) -> R<Adt2> {
    let m124 = v1;
    match m124 {
        Adt2::C0 => {
            Ok(Adt2::C5)
        }
        Adt2::C1 => {
            Ok(Adt2::C5)
        }
        Adt2::C2 => {
            Ok(Adt2::C5)
        }
        Adt2::C3 => {
            Ok(Adt2::C5)
        }
        Adt2::C4(r125) => {
            let v2 = (*r125).clone();
            f20(v0, v2)
        }
        Adt2::C5 => {
            Ok(Adt2::C5)
        }
        Adt2::C6 => {
            Ok(Adt2::C0)
        }
        Adt2::C7 => {
            Ok(Adt2::C1)
        }
        Adt2::C8 => {
            Ok(Adt2::C3)
        }
        Adt2::C9 => {
            Ok(Adt2::C1)
        }
        Adt2::C10 => {
            Ok(Adt2::C1)
        }
        Adt2::C11 => {
            Ok(Adt2::C12)
        }
        Adt2::C12 => {
            Ok(Adt2::C5)
        }
        Adt2::C13 => {
            Ok(Adt2::C14)
        }
        Adt2::C14 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f40(v0: Adt0) -> R<Adt6> {
    let m126 = v0;
    match m126 {
        Adt0::C0(v1) => {
            Ok(Adt6::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt6::C1(f75(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt6::C1(f76(v4)?))
        }
        Adt0::C3(r127) => {
            let v5 = (*r127).clone();
            Ok(f73(f77(Str::lit("[")), f77(Str::lit("]")), f74(f78(v5)?)))
        }
        Adt0::C4(r128) => {
            let v6 = (*r128).clone();
            f79(f80(v6.clone())?, f81(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt6::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt6::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt6::C0)
        }
    }
}

pub fn f41(v0: Fn4, p132: List<Adt0>, p133: List<Adt0>) -> List<Adt0> {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v1, v2) = m131;
                let m129 = v2;
                match m129.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c130: Fn4 = v0.clone();
                            c130.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f43(v0: Adt5) -> Option<Adt5> {
    let m135 = f46(match v0.clone() {
        Adt5::C0(h134, _) => {
            (*h134).clone()
        }
    });
    match m135 {
        None => {
            None::<Adt5>
        }
        Some(v1) => {
            Some(Adt5::C0(std::rc::Rc::new(match v1.clone() {
                Adt11::C0(_, h136) => {
                    (*h136).clone()
                }
            }), List::cons(match v1 {
                Adt11::C0(h137, _) => {
                    (*h137).clone()
                }
            }, match v0 {
                Adt5::C0(_, h138) => {
                    h138
                }
            })))
        }
    }
}

pub fn f44(v0: Adt0) -> Option<Adt10> {
    let m139 = v0;
    match m139 {
        Adt0::C0(_) => {
            None::<Adt10>
        }
        Adt0::C1(_, _) => {
            None::<Adt10>
        }
        Adt0::C2(_) => {
            None::<Adt10>
        }
        Adt0::C3(_) => {
            None::<Adt10>
        }
        Adt0::C4(_) => {
            None::<Adt10>
        }
        Adt0::C5 => {
            None::<Adt10>
        }
        Adt0::C6(_, _) => {
            None::<Adt10>
        }
        Adt0::C7(v1, r140, r141) => {
            let v2 = (*r140).clone();
            let v3 = (*r141).clone();
            Some(Adt10::C0(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3)))
        }
    }
}

pub fn f45(v0: Bytes, v1: Bytes) -> bool {
    let a142 = v0;
    let a143 = v1;
    equal(a142, a143)
}

pub fn f46(v0: Adt0) -> Option<Adt11> {
    let m144 = v0;
    match m144 {
        Adt0::C0(_) => {
            None::<Adt11>
        }
        Adt0::C1(_, _) => {
            None::<Adt11>
        }
        Adt0::C2(_) => {
            None::<Adt11>
        }
        Adt0::C3(_) => {
            None::<Adt11>
        }
        Adt0::C4(_) => {
            None::<Adt11>
        }
        Adt0::C5 => {
            None::<Adt11>
        }
        Adt0::C6(r145, r146) => {
            let v1 = (*r145).clone();
            let v2 = (*r146).clone();
            Some(Adt11::C0(std::rc::Rc::new(v1), std::rc::Rc::new(v2)))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt11>
        }
    }
}

pub fn f47(v0: Fn5, p151: u64, p152: Adt0) -> R<(Adt0, bool)> {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v1, v2) = m150;
                let m147 = v1;
                if m147 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m147 - 1;
                    let m149 = {
                        let c148: Fn5 = v0.clone();
                        c148.apply(v2.clone())?
                    };
                    match m149 {
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
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f82(v0, v1)
}

pub fn f49(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m158 = {
            let (h154, _) = f83(Fn8::F84(v0.clone()), {
                let a156 = {
                    let a155 = v0.clone();
                    length_bytes(a155)
                };
                let a157 = 1u64;
                nat_add(a156, a157)?
            }, Adt12::C0(0u64, 0u64))?;
            h154
        };
        match m158 {
            Adt12::C0(_, h153) => {
                h153
            }
        }
    };
    Ok(f85(v0.clone(), v1, {
        let a160 = {
            let a159 = v0;
            length_bytes(a159)
        };
        let a161 = v1;
        nat_sub(a160, a161)
    }))
}

pub fn f50(v0: Fn6, p166: u64, p167: Adt7) -> R<(Adt7, bool)> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m162 = v1;
                if m162 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m162 - 1;
                    let m164 = {
                        let c163: Fn6 = v0.clone();
                        c163.apply(v2.clone())?
                    };
                    match m164 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: u64, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    f86(v1, v0, v2)
}

pub fn f52(v0: List<Bytes>) -> Bytes {
    f87(f88(v0))
}

pub fn f53(v0: Adt0) -> R<bool> {
    let m168 = f55(v0.clone());
    match m168 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f89(v0, v1)
        }
    }
}

pub fn f54(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m169 = f90(v1.clone());
    match m169 {
        None => {
            if f45(v1.clone(), f4(Str::lit("var"))) {
                Ok(f60(f63(v0, f4(Str::lit("name")))))
            } else {
                if f45(v1.clone(), f4(Str::lit("nat"))) {
                    Ok(f60(f63(v0, f4(Str::lit("value")))))
                } else {
                    if f45(v1.clone(), f4(Str::lit("bool"))) {
                        Ok(f60({
                            let m170 = f3(v0, f4(Str::lit("value")));
                            match m170 {
                                None => {
                                    f4(Str::lit(""))
                                }
                                Some(v4) => {
                                    let m171 = f93(v4);
                                    match m171 {
                                        None => {
                                            f4(Str::lit(""))
                                        }
                                        Some(v5) => {
                                            if v5 {
                                                f4(Str::lit("true"))
                                            } else {
                                                f4(Str::lit("false"))
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("string"))) {
                            Ok(f60(f94(f63(v0, f4(Str::lit("value"))))?))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("bytes"))) {
                                Ok(f60(f22(f4(Str::lit("0x")), f63(v0, f4(Str::lit("hex"))))))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("unit"))) {
                                    Ok(f60(f4(Str::lit("()"))))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("integer"))) {
                                        Ok(f60(f22(f22(f22(f22(f4(Str::lit("(")), f63(v0.clone(), f4(Str::lit("value")))), f4(Str::lit(" : "))), {
                                            let m172 = f95(f63(v0, f4(Str::lit("representation"))));
                                            match m172 {
                                                None => {
                                                    f4(Str::lit(""))
                                                }
                                                Some(v6) => {
                                                    v6
                                                }
                                            }
                                        }), f4(Str::lit(")")))))
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("pair"))) {
                                            Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f4(Str::lit("left"))), List::cons(f60(f4(Str::lit(", "))), List::cons(f64(v2, f4(Str::lit("right"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                        } else {
                                            if if f45(v1.clone(), f4(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f45(v1.clone(), f4(Str::lit("second")))
                                                }
                                            } {
                                                Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("cons"))) {
                                                    Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("head"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("tail"))), List::<Adt6>::nil()))))))
                                                } else {
                                                    if f45(v1.clone(), f4(Str::lit("nil"))) {
                                                        Ok(f96(v1, List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                                                    } else {
                                                        if f45(v1.clone(), f4(Str::lit("if"))) {
                                                            Ok(f62(List::cons(f60(f4(Str::lit("(if "))), List::cons(f64(v2.clone(), f4(Str::lit("condition"))), List::cons(f60(f4(Str::lit(" then "))), List::cons(f64(v2.clone(), f4(Str::lit("then_value"))), List::cons(f60(f4(Str::lit(" else "))), List::cons(f64(v2, f4(Str::lit("else_value"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))))
                                                        } else {
                                                            if f45(v1.clone(), f4(Str::lit("let"))) {
                                                                Ok(f62(List::cons(f60(f4(Str::lit("(let "))), List::cons(f64(v2.clone(), f4(Str::lit("binder"))), List::cons(f60(f4(Str::lit(" := "))), List::cons(f64(v2.clone(), f4(Str::lit("value"))), List::cons(f60(f4(Str::lit("; "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))))
                                                            } else {
                                                                if f45(v1.clone(), f4(Str::lit("forall"))) {
                                                                    Ok(f62(List::cons(f60(f4(Str::lit("(forall ("))), List::cons(f64(v2.clone(), f4(Str::lit("binder"))), List::cons(f60(f4(Str::lit("), "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                                                } else {
                                                                    if f45(v1.clone(), f4(Str::lit("lambda"))) {
                                                                        let v7: List<Adt6> = f97(v0, f4(Str::lit("captures")));
                                                                        Ok(f62(List::cons(f60(f4(Str::lit("(fun "))), {
                                                                            let a178 = {
                                                                                let a174 = 0u64;
                                                                                let a175 = {
                                                                                    let a173 = v7.clone();
                                                                                    length_list(a173)
                                                                                };
                                                                                if nat_lt(a174, a175) {
                                                                                    List::cons(f60(f4(Str::lit("["))), {
                                                                                        let a176 = f70(v7, f4(Str::lit(", ")));
                                                                                        let a177 = List::cons(f60(f4(Str::lit("]"))), List::cons(f60(f4(Str::lit(" "))), List::<Adt6>::nil()));
                                                                                        append_list(a176, a177)
                                                                                    })
                                                                                } else {
                                                                                    List::<Adt6>::nil()
                                                                                }
                                                                            };
                                                                            let a179 = List::cons(f64(v2.clone(), f4(Str::lit("parameters"))), List::cons(f60(f4(Str::lit("=> "))), List::cons(f64(v2, f4(Str::lit("body"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()))));
                                                                            append_list(a178, a179)
                                                                        })))
                                                                    } else {
                                                                        if f45(v1.clone(), f4(Str::lit("apply"))) {
                                                                            Ok(f96(f4(Str::lit("apply")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("function"))), List::cons(f64(v2, f4(Str::lit("arguments"))), List::<Adt6>::nil())))))
                                                                        } else {
                                                                            if f45(v1.clone(), f4(Str::lit("project"))) {
                                                                                Ok(f96(f4(Str::lit("project")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::cons(f60(f22(f4(Str::lit(" ")), f63(v0, f4(Str::lit("field"))))), List::<Adt6>::nil())))))
                                                                            } else {
                                                                                if f45(v1.clone(), f4(Str::lit("call"))) {
                                                                                    Ok(f62(List::cons(f60(f22(f4(Str::lit("(")), f98(v0.clone(), f4(Str::lit("function")))?)), {
                                                                                        let a180 = f99(v0, v2.clone());
                                                                                        let a181 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()));
                                                                                        append_list(a180, a181)
                                                                                    })))
                                                                                } else {
                                                                                    if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                                                                        Ok(f96(f4(Str::lit("ctor")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("constructor")))?)), {
                                                                                            let a182 = f99(v0, v2.clone());
                                                                                            let a183 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::<Adt6>::nil());
                                                                                            append_list(a182, a183)
                                                                                        })))
                                                                                    } else {
                                                                                        if f45(v1.clone(), f4(Str::lit("function_ref"))) {
                                                                                            Ok(f96(f4(Str::lit("ref")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("function")))?)), f99(v0, v2))))
                                                                                        } else {
                                                                                            if f45(v1.clone(), f4(Str::lit("record"))) {
                                                                                                Ok(f96(f4(Str::lit("record")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("type")))?)), {
                                                                                                    let a184 = f99(v0, v2.clone());
                                                                                                    let a185 = List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("fields"))), List::<Adt6>::nil()));
                                                                                                    append_list(a184, a185)
                                                                                                })))
                                                                                            } else {
                                                                                                if f45(v1.clone(), f4(Str::lit("primitive"))) {
                                                                                                    Ok(f62(List::cons(f60(f22(f4(Str::lit("(prim ")), f63(v0, f4(Str::lit("operation"))))), List::cons(f64(v2.clone(), f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(" : "))), List::cons(f64(v2, f4(Str::lit("result"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                                                                                } else {
                                                                                                    if f45(v1, f4(Str::lit("checked_apply"))) {
                                                                                                        Ok(f96(f4(Str::lit("checked")), List::cons(f60(f22(f4(Str::lit(" ")), f98(v0.clone(), f4(Str::lit("model")))?)), {
                                                                                                            let a188 = f99(v0.clone(), v2.clone());
                                                                                                            let a189 = List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(" with "))), List::cons(f60(f4(Str::lit("["))), {
                                                                                                                let a186 = f70(f97(v0, f4(Str::lit("checks"))), f4(Str::lit(", ")));
                                                                                                                let a187 = List::cons(f60(f4(Str::lit("]"))), List::<Adt6>::nil());
                                                                                                                append_list(a186, a187)
                                                                                                            })));
                                                                                                            append_list(a188, a189)
                                                                                                        })))
                                                                                                    } else {
                                                                                                        Ok(Adt6::C6(std::rc::Rc::new(Adt6::C3(2u64, std::rc::Rc::new(f62(List::cons(f60(f4(Str::lit("(match "))), List::cons(f64(v2.clone(), f4(Str::lit("scrutinee"))), List::cons(f64(v2, f4(Str::lit("branches"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil()))))))))))
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
        Some(v3) => {
            Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f91(v1.clone())), List::cons(f60(f22(f22(f4(Str::lit(" ")), v3), f4(Str::lit(" ")))), List::cons(f64(v2, f92(v1)), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
        }
    }
}

pub fn f55(v0: Adt0) -> Option<Bytes> {
    f7(f3(v0, f4(Str::lit("kind"))))
}

pub fn f56(v0: Adt0, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    f79(f80(v0)?, v1)
}

pub fn f57(v0: Adt0) -> R<bool> {
    let m190 = f55(v0.clone());
    match m190 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f100(v0, v1)
        }
    }
}

pub fn f58(v0: Adt0, v1: Bytes, v2: List<(Bytes, Adt6)>) -> R<Adt6> {
    let m191 = f101(v1.clone());
    match m191 {
        None => {
            if f45(v1.clone(), f4(Str::lit("parameter"))) {
                Ok(f60(f22(f4(Str::lit("'")), f63(v0, f4(Str::lit("name"))))))
            } else {
                if f45(v1.clone(), f4(Str::lit("list"))) {
                    Ok(f96(f4(Str::lit("List")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                } else {
                    if f45(v1.clone(), f4(Str::lit("set"))) {
                        Ok(f96(f4(Str::lit("Set")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("element"))), List::<Adt6>::nil()))))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("option"))) {
                            Ok(f96(f4(Str::lit("Option")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("result"))) {
                                Ok(f96(f4(Str::lit("Result")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("ok"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("error"))), List::<Adt6>::nil()))))))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("map"))) {
                                    Ok(f96(f4(Str::lit("Map")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("key"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("value"))), List::<Adt6>::nil()))))))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("product"))) {
                                        Ok(f62(List::cons(f60(f4(Str::lit("("))), List::cons(f64(v2.clone(), f4(Str::lit("left"))), List::cons(f60(f4(Str::lit(" * "))), List::cons(f64(v2, f4(Str::lit("right"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))))
                                    } else {
                                        if f45(v1, f4(Str::lit("function"))) {
                                            Ok(f96(f4(Str::lit("Fn")), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2.clone(), f4(Str::lit("parameters"))), List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v2, f4(Str::lit("result"))), List::<Adt6>::nil()))))))
                                        } else {
                                            let v4: Bytes = f98(v0.clone(), f4(Str::lit("member")))?;
                                            let m192 = f2(f3(v0, f4(Str::lit("arguments"))));
                                            match m192 {
                                                None => {
                                                    Ok(f60(v4))
                                                }
                                                Some(v5) => {
                                                    let a193 = f6(v5)?;
                                                    let a194 = 0u64;
                                                    if nat_eq(a193, a194) {
                                                        Ok(f60(v4))
                                                    } else {
                                                        Ok(f62(List::cons(f60(f22(f4(Str::lit("(")), v4)), List::cons(f64(v2, f4(Str::lit("arguments"))), List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil())))))
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
        Some(v3) => {
            Ok(f60(v3))
        }
    }
}

pub fn f59(v0: Adt0) -> R<Option<Bytes>> {
    let m195 = f1(v0);
    match m195 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m196 = f7(f3(v1.clone(), f4(Str::lit("name"))));
            match m196 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let v3: Option<Adt0> = f3(v1.clone(), f4(Str::lit("module")));
                    if f5({
                        let a197 = f6(v1)?;
                        let a198 = if f102(v3.clone()) {
                            2u64
                        } else {
                            1u64
                        };
                        nat_eq(a197, a198)
                    }, f103(v2.clone())?) {
                        let m199 = v3;
                        match m199 {
                            None => {
                                Ok(Some(v2))
                            }
                            Some(v4) => {
                                let m200 = f18(v4);
                                match m200 {
                                    None => {
                                        Ok(None::<Bytes>)
                                    }
                                    Some(v5) => {
                                        if f103(v5.clone())? {
                                            Ok(Some(f22(f22(v5, f4(Str::lit("::"))), v2)))
                                        } else {
                                            Ok(None::<Bytes>)
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f60(v0: Bytes) -> Adt6 {
    Adt6::C1(v0)
}

pub fn f61(v0: Adt0) -> R<bool> {
    let m201 = f1(v0);
    match m201 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m202 = f7(f3(v1.clone(), f4(Str::lit("name"))));
            match m202 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    Ok(f5(f102(f3(v1.clone(), f4(Str::lit("type")))), f5({
                        let a203 = f6(v1)?;
                        let a204 = 2u64;
                        nat_eq(a203, a204)
                    }, f103(v2)?)))
                }
            }
        }
    }
}

pub fn f62(v0: List<Adt6>) -> Adt6 {
    f104(Fn9::F105, Adt6::C0, f74(v0))
}

pub fn f63(v0: Adt0, v1: Bytes) -> Bytes {
    let m205 = f7(f3(v0, v1));
    match m205 {
        None => {
            f4(Str::lit(""))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f64(v0: List<(Bytes, Adt6)>, v1: Bytes) -> Adt6 {
    f106(Fn10::F107(v1), Adt6::C0, v0)
}

pub fn f65(v0: Adt0, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    Ok(f62(List::cons(f60(f22(f4(Str::lit("| ")), f98(v0.clone(), f4(Str::lit("constructor")))?)), {
        let a206 = f69(f97(v0, f4(Str::lit("binders"))), f4(Str::lit(" ")), f4(Str::lit("")));
        let a207 = List::cons(f60(f4(Str::lit(" => "))), List::cons(f64(v1, f4(Str::lit("body"))), List::<Adt6>::nil()));
        append_list(a206, a207)
    })))
}

pub fn f66(v0: Adt9) -> bool {
    let m208 = v0;
    match m208 {
        Adt9::C0(_) => {
            false
        }
        Adt9::C1(_) => {
            false
        }
        Adt9::C2 => {
            false
        }
        Adt9::C3 => {
            false
        }
        Adt9::C4 => {
            false
        }
        Adt9::C5 => {
            false
        }
        Adt9::C6 => {
            true
        }
    }
}

pub fn f67(v0: Adt0, v1: Adt9, v2: Bytes) -> R<Adt2> {
    let m209 = v1;
    match m209 {
        Adt9::C0(v3) => {
            f108(v0, v3, v2)
        }
        Adt9::C1(v4) => {
            Ok(f109(v4, v2))
        }
        Adt9::C2 => {
            Ok(f110(v2))
        }
        Adt9::C3 => {
            if f45(v2, f4(Str::lit("type"))) {
                Ok(Adt2::C1)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt9::C4 => {
            if f45(v2, f4(Str::lit("value"))) {
                Ok(Adt2::C0)
            } else {
                Ok(Adt2::C5)
            }
        }
        Adt9::C5 => {
            if f45(v2.clone(), f4(Str::lit("body"))) {
                Ok(Adt2::C0)
            } else {
                if f45(v2, f4(Str::lit("constructor"))) {
                    Ok(Adt2::C2)
                } else {
                    Ok(Adt2::C5)
                }
            }
        }
        Adt9::C6 => {
            Ok(Adt2::C5)
        }
    }
}

pub fn f68(v0: Adt0, v1: Adt0, v2: Adt2) -> R<Option<Adt0>> {
    let v3: Option<Bytes> = f80(v1)?;
    let m210 = v2.clone();
    match m210 {
        Adt2::C0 => {
            let m211 = v3;
            match m211 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    let m212 = f111(v0.clone(), v2)?;
                    match m212 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v5) => {
                            f112(v0, v5, v4)
                        }
                    }
                }
            }
        }
        Adt2::C1 => {
            let m213 = v3;
            match m213 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v6) => {
                    let m214 = f111(v0.clone(), v2)?;
                    match m214 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v7) => {
                            f112(v0, v7, v6)
                        }
                    }
                }
            }
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(f21(v0, f4(Str::lit("parameter"))))
        }
        Adt2::C4(r216) => {
            let v8 = (*r216).clone();
            let m215 = v3;
            match m215 {
                None => {
                    Ok(Some(v8))
                }
                Some(v9) => {
                    if f113(v0.clone(), v8.clone())? {
                        f112(v0, v8, v9)
                    } else {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f69(v0: List<Adt6>, v1: Bytes, v2: Bytes) -> List<Adt6> {
    f74(f71(Fn7::F114(v2, v1), List::<Adt6>::nil(), v0))
}

pub fn f70(v0: List<Adt6>, v1: Bytes) -> List<Adt6> {
    f74({
        let (_, h217) = f115(Fn11::F116(v1), (true, List::<Adt6>::nil()), v0);
        h217
    })
}

pub fn f71(v0: Fn7, p221: List<Adt6>, p222: List<Adt6>) -> List<Adt6> {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v1, v2) = m220;
                let m218 = v2;
                match m218.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>(({
                            let c219: Fn7 = v0.clone();
                            c219.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    let a223 = v0;
    let a224 = List::cons(Adt6::C4, List::cons(v1, List::<Adt6>::nil()));
    append_list(a223, a224)
}

pub fn f73(v0: Bytes, v1: Bytes, v2: List<Adt6>) -> Adt6 {
    let m225 = v2.clone();
    match m225.uncons() {
        None => {
            Adt6::C1(f117(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt6) = f118(Fn12::F119, (true, Adt6::C0), v2);
            Adt6::C6(std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(v0)), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C3(2u64, std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C5), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new({
                let (_, h226) = v5;
                h226
            }), std::rc::Rc::new(Adt6::C0))))))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C5), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(v1)), std::rc::Rc::new(Adt6::C0))))))))))
        }
    }
}

pub fn f74(v0: List<Adt6>) -> List<Adt6> {
    f71(Fn7::F120, List::<Adt6>::nil(), v0)
}

pub fn f75(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a227 = 0u64;
        let a228 = v1;
        nat_lt(a227, a228)
    } else {
        false
    } {
        let a229 = Bytes::lit(&[0x2d]);
        let a230 = f121(v1);
        append_bytes(a229, a230)
    } else {
        f121(v1)
    }
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    if f103(v0.clone())? {
        Ok(v0)
    } else {
        f94(v0)
    }
}

pub fn f77(v0: Str) -> Bytes {
    let a231 = v0;
    utf8_encode(a231)
}

pub fn f78(v0: Adt0) -> R<List<Adt6>> {
    let m232 = v0;
    match m232 {
        Adt0::C0(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt6>::nil())
        }
        Adt0::C6(r233, r234) => {
            let v7 = (*r233).clone();
            let v8 = (*r234).clone();
            Ok(List::cons(f40(v7)?, f78(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt6>::nil())
        }
    }
}

pub fn f79(v0: Option<Bytes>, v1: List<(Bytes, Adt6)>) -> R<Adt6> {
    let v2: List<(Bytes, Adt6)> = f122(Fn13::F123(v0.clone()), List::<(Bytes, Adt6)>::nil(), v1)?;
    Ok(f73({
        let m235 = v0;
        match m235 {
            None => {
                f77(Str::lit("{"))
            }
            Some(v3) => {
                f117(v3, f77(Str::lit("{")))
            }
        }
    }, f77(Str::lit("}")), f124(Fn14::F125, List::<Adt6>::nil(), f126(v2)?)))
}

pub fn f80(p243: Adt0) -> R<Option<Bytes>> {
    let m242 = p243;
    {
        let mut m242 = m242;
        loop {
            let step = {
                let v0 = m242;
                let m236 = v0;
                match m236 {
                    Adt0::C0(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C1(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C2(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C3(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C6(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C7(v9, r240, r241) => {
                        let v10 = (*r240).clone();
                        let v11 = (*r241).clone();
                        let a237 = v9;
                        let a238 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a237, a238) {
                            let m239 = v10;
                            match m239 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f103(v12.clone())? {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt0::C3(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C4(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C5 => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C6(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C7(_, _, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt0, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt0) -> R<List<(Bytes, Adt6)>> {
    let m244 = v0;
    match m244 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Adt0::C7(v9, r245, r246) => {
            let v10 = (*r245).clone();
            let v11 = (*r246).clone();
            Ok(List::cons((v9, f40(v10)?), f81(v11)?))
        }
    }
}

pub fn f82(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m247 = f7(f19(v1, f4(Str::lit("$ref"))));
    match m247 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f21(v0, f49(v2)?))
        }
    }
}

pub fn f83(v0: Fn8, p252: u64, p253: Adt12) -> R<(Adt12, bool)> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m248 - 1;
                    let m250 = {
                        let c249: Fn8 = v0.clone();
                        c249.apply(v2.clone())?
                    };
                    match m250 {
                        None => {
                            Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt12), R<(Adt12, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: Adt12) -> R<Option<Adt12>> {
    f127(v0, v1)
}

pub fn f85(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m257 = {
        let a254 = v0;
        let a255 = v1;
        let a256 = v2;
        slice_bytes(a254, a255, a256)
    };
    match m257 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f86(v0: u64, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let m259 = match v2.clone() {
        Adt7::C0(_, _, h258) => {
            h258
        }
    };
    match m259.uncons() {
        None => {
            Ok(None::<Adt7>)
        }
        Some((v3, v4)) => {
            Ok(Some(f128(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f87(v0: List<Bytes>) -> Bytes {
    let m262 = {
        let (h260, _) = f129(Fn15::F130, {
            let a261 = v0.clone();
            length_list(a261)
        }, v0);
        h260
    };
    match m262.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f88(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn16::F132, List::<Bytes>::nil(), v0)
}

pub fn f89(v0: Adt0, v1: Bytes) -> R<bool> {
    let m263 = f90(v1.clone());
    match m263 {
        None => {
            if f45(v1.clone(), f4(Str::lit("var"))) {
                Ok(f5(f133(v0.clone(), f134(Str::lit("name")))?, {
                    let m264 = f7(f3(v0, f4(Str::lit("name"))));
                    match m264 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f135(v3)?
                        }
                    }
                }))
            } else {
                if f45(v1.clone(), f4(Str::lit("nat"))) {
                    Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                        let m265 = f7(f3(v0, f4(Str::lit("value"))));
                        match m265 {
                            None => {
                                false
                            }
                            Some(v4) => {
                                f136(v4)?
                            }
                        }
                    }))
                } else {
                    if f45(v1.clone(), f4(Str::lit("bool"))) {
                        Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                            let m266 = f3(v0, f4(Str::lit("value")));
                            match m266 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    let m267 = f93(v5);
                                    match m267 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("string"))) {
                            Ok(f5(f133(v0.clone(), f134(Str::lit("value")))?, {
                                let m268 = f7(f3(v0, f4(Str::lit("value"))));
                                match m268 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("bytes"))) {
                                Ok(f5(f133(v0.clone(), f134(Str::lit("hex")))?, {
                                    let m269 = f7(f3(v0, f4(Str::lit("hex"))));
                                    match m269 {
                                        None => {
                                            false
                                        }
                                        Some(v8) => {
                                            f137(v8)?
                                        }
                                    }
                                }))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("unit"))) {
                                    f133(v0, List::cons(f4(Str::lit("kind")), List::<Bytes>::nil()))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("integer"))) {
                                        Ok(f5(f133(v0.clone(), f138(Str::lit("representation"), Str::lit("value")))?, {
                                            let m270 = f7(f3(v0.clone(), f4(Str::lit("value"))));
                                            match m270 {
                                                None => {
                                                    false
                                                }
                                                Some(v9) => {
                                                    let m271 = f7(f3(v0, f4(Str::lit("representation"))));
                                                    match m271 {
                                                        None => {
                                                            false
                                                        }
                                                        Some(v10) => {
                                                            f5(f102({
                                                                let m272 = f95(v10);
                                                                match m272 {
                                                                    None => {
                                                                        None::<Adt0>
                                                                    }
                                                                    Some(_) => {
                                                                        Some(Adt0::C5)
                                                                    }
                                                                }
                                                            }), f139(v9)?)
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("pair"))) {
                                            f133(v0, f138(Str::lit("left"), Str::lit("right")))
                                        } else {
                                            if if f45(v1.clone(), f4(Str::lit("not"))) {
                                                true
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("first"))) {
                                                    true
                                                } else {
                                                    f45(v1.clone(), f4(Str::lit("second")))
                                                }
                                            } {
                                                f133(v0, f134(Str::lit("value")))
                                            } else {
                                                if f45(v1.clone(), f4(Str::lit("cons"))) {
                                                    f133(v0, f138(Str::lit("head"), Str::lit("tail")))
                                                } else {
                                                    if f45(v1.clone(), f4(Str::lit("nil"))) {
                                                        f133(v0, f134(Str::lit("element")))
                                                    } else {
                                                        if f45(v1.clone(), f4(Str::lit("if"))) {
                                                            f133(v0, f140(Str::lit("condition"), Str::lit("then_value"), Str::lit("else_value")))
                                                        } else {
                                                            if f45(v1.clone(), f4(Str::lit("let"))) {
                                                                Ok(f5(f133(v0.clone(), f140(Str::lit("binder"), Str::lit("value"), Str::lit("body")))?, f141(f3(v0, f4(Str::lit("binder"))))?))
                                                            } else {
                                                                if f45(v1.clone(), f4(Str::lit("forall"))) {
                                                                    Ok(f5(f133(v0.clone(), f138(Str::lit("binder"), Str::lit("body")))?, f141(f3(v0, f4(Str::lit("binder"))))?))
                                                                } else {
                                                                    if f45(v1.clone(), f4(Str::lit("lambda"))) {
                                                                        Ok(f5(f133(v0.clone(), f140(Str::lit("parameters"), Str::lit("captures"), Str::lit("body")))?, f5(f142(v0.clone(), f4(Str::lit("parameters")), Fn17::F143)?, f142(v0, f4(Str::lit("captures")), Fn17::F144)?)))
                                                                    } else {
                                                                        if f45(v1.clone(), f4(Str::lit("apply"))) {
                                                                            Ok(f5(f133(v0.clone(), f138(Str::lit("function"), Str::lit("arguments")))?, f145(v0, f4(Str::lit("arguments")))))
                                                                        } else {
                                                                            if f45(v1.clone(), f4(Str::lit("project"))) {
                                                                                Ok(f5(f133(v0.clone(), f138(Str::lit("value"), Str::lit("field")))?, {
                                                                                    let m273 = f7(f3(v0, f4(Str::lit("field"))));
                                                                                    match m273 {
                                                                                        None => {
                                                                                            false
                                                                                        }
                                                                                        Some(v12) => {
                                                                                            f103(v12)?
                                                                                        }
                                                                                    }
                                                                                }))
                                                                            } else {
                                                                                if f45(v1.clone(), f4(Str::lit("call"))) {
                                                                                    Ok(f5(f146(v0.clone(), f138(Str::lit("function"), Str::lit("arguments")))?, f5({
                                                                                        let m274 = f3(v0.clone(), f4(Str::lit("function")));
                                                                                        match m274 {
                                                                                            None => {
                                                                                                false
                                                                                            }
                                                                                            Some(v13) => {
                                                                                                let m275 = f59(v13.clone())?;
                                                                                                match m275 {
                                                                                                    None => {
                                                                                                        false
                                                                                                    }
                                                                                                    Some(_) => {
                                                                                                        let m276 = f7(f19(v13, f4(Str::lit("name"))));
                                                                                                        match m276 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v15) => {
                                                                                                                f135(v15)?
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }, f5(f147(v0.clone())?, f145(v0, f4(Str::lit("arguments")))))))
                                                                                } else {
                                                                                    if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                                                                        Ok(f5(f146(v0.clone(), f138(Str::lit("constructor"), Str::lit("arguments")))?, f5(f148(f3(v0.clone(), f4(Str::lit("constructor"))))?, f5(f147(v0.clone())?, f145(v0, f4(Str::lit("arguments")))))))
                                                                                    } else {
                                                                                        if f45(v1.clone(), f4(Str::lit("function_ref"))) {
                                                                                            Ok(f5(f146(v0.clone(), f134(Str::lit("function")))?, f5(f148(f3(v0.clone(), f4(Str::lit("function"))))?, f147(v0)?)))
                                                                                        } else {
                                                                                            if f45(v1.clone(), f4(Str::lit("record"))) {
                                                                                                Ok(f5(f146(v0.clone(), f138(Str::lit("type"), Str::lit("fields")))?, f5(f148(f3(v0.clone(), f4(Str::lit("type"))))?, f5(f147(v0.clone())?, f142(v0, f4(Str::lit("fields")), Fn17::F149)?))))
                                                                                            } else {
                                                                                                if f45(v1.clone(), f4(Str::lit("primitive"))) {
                                                                                                    Ok(f5(f133(v0.clone(), f140(Str::lit("operation"), Str::lit("arguments"), Str::lit("result")))?, f5({
                                                                                                        let m277 = f7(f3(v0.clone(), f4(Str::lit("operation"))));
                                                                                                        match m277 {
                                                                                                            None => {
                                                                                                                false
                                                                                                            }
                                                                                                            Some(v16) => {
                                                                                                                f103(v16)?
                                                                                                            }
                                                                                                        }
                                                                                                    }, f145(v0, f4(Str::lit("arguments"))))))
                                                                                                } else {
                                                                                                    if f45(v1.clone(), f4(Str::lit("checked_apply"))) {
                                                                                                        Ok(f5(f146(v0.clone(), f140(Str::lit("model"), Str::lit("arguments"), Str::lit("checks")))?, f5(f148(f3(v0.clone(), f4(Str::lit("model"))))?, f5(f147(v0.clone())?, f5(f145(v0.clone(), f4(Str::lit("arguments"))), f142(v0, f4(Str::lit("checks")), Fn17::F150)?)))))
                                                                                                    } else {
                                                                                                        if f45(v1, f4(Str::lit("match"))) {
                                                                                                            Ok(f5(f133(v0.clone(), f138(Str::lit("scrutinee"), Str::lit("branches")))?, f142(v0, f4(Str::lit("branches")), Fn17::F151)?))
                                                                                                        } else {
                                                                                                            Ok(false)
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
        Some(_) => {
            f133(v0, List::cons(f4(Str::lit("kind")), List::cons(f91(v1.clone()), List::cons(f92(v1), List::<Bytes>::nil()))))
        }
    }
}

pub fn f90(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("add"))) {
        Some(f4(Str::lit("+")))
    } else {
        if f45(v0.clone(), f4(Str::lit("eq"))) {
            Some(f4(Str::lit("=")))
        } else {
            if f45(v0.clone(), f4(Str::lit("le"))) {
                Some(f4(Str::lit("<=")))
            } else {
                if f45(v0.clone(), f4(Str::lit("lt"))) {
                    Some(f4(Str::lit("<")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("beq"))) {
                        Some(f4(Str::lit("=?")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("ble"))) {
                            Some(f4(Str::lit("<=?")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("blt"))) {
                                Some(f4(Str::lit("<?")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("and"))) {
                                    Some(f4(Str::lit("&&")))
                                } else {
                                    if f45(v0.clone(), f4(Str::lit("or"))) {
                                        Some(f4(Str::lit("||")))
                                    } else {
                                        if f45(v0.clone(), f4(Str::lit("prop_and"))) {
                                            Some(f4(Str::lit("/\\")))
                                        } else {
                                            if f45(v0.clone(), f4(Str::lit("iff"))) {
                                                Some(f4(Str::lit("<->")))
                                            } else {
                                                if f45(v0, f4(Str::lit("implies"))) {
                                                    Some(f4(Str::lit("->")))
                                                } else {
                                                    None::<Bytes>
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

pub fn f91(v0: Bytes) -> Bytes {
    if f45(v0, f4(Str::lit("implies"))) {
        f4(Str::lit("premise"))
    } else {
        f4(Str::lit("left"))
    }
}

pub fn f92(v0: Bytes) -> Bytes {
    if f45(v0, f4(Str::lit("implies"))) {
        f4(Str::lit("conclusion"))
    } else {
        f4(Str::lit("right"))
    }
}

pub fn f93(v0: Adt0) -> Option<bool> {
    let m278 = v0;
    match m278 {
        Adt0::C0(v1) => {
            Some(v1)
        }
        Adt0::C1(_, _) => {
            None::<bool>
        }
        Adt0::C2(_) => {
            None::<bool>
        }
        Adt0::C3(_) => {
            None::<bool>
        }
        Adt0::C4(_) => {
            None::<bool>
        }
        Adt0::C5 => {
            None::<bool>
        }
        Adt0::C6(_, _) => {
            None::<bool>
        }
        Adt0::C7(_, _, _) => {
            None::<bool>
        }
    }
}

pub fn f94(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a279 = v0.clone();
        length_bytes(a279)
    };
    let v2: Adt13 = {
        let (h280, _) = f152(Fn18::F153(v1, v0), {
            let a281 = v1;
            let a282 = 1u64;
            nat_add(a281, a282)?
        }, Adt13::C0(0u64, List::<Bytes>::nil()))?;
        h280
    };
    Ok(f154(f154(f155(Str::lit("\"")), f156(match v2 {
        Adt13::C0(_, h283) => {
            h283
        }
    })), f155(Str::lit("\""))))
}

pub fn f95(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("int"))) {
        Some(f4(Str::lit("Int")))
    } else {
        if f45(v0.clone(), f4(Str::lit("int8"))) {
            Some(f4(Str::lit("Int8")))
        } else {
            if f45(v0.clone(), f4(Str::lit("int16"))) {
                Some(f4(Str::lit("Int16")))
            } else {
                if f45(v0.clone(), f4(Str::lit("int32"))) {
                    Some(f4(Str::lit("Int32")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("int64"))) {
                        Some(f4(Str::lit("Int64")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("uint8"))) {
                            Some(f4(Str::lit("UInt8")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("uint16"))) {
                                Some(f4(Str::lit("UInt16")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("uint32"))) {
                                    Some(f4(Str::lit("UInt32")))
                                } else {
                                    if f45(v0, f4(Str::lit("uint64"))) {
                                        Some(f4(Str::lit("UInt64")))
                                    } else {
                                        None::<Bytes>
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

pub fn f96(v0: Bytes, v1: List<Adt6>) -> Adt6 {
    f62(List::cons(f60(f22(f4(Str::lit("(")), v0)), {
        let a284 = v1;
        let a285 = List::cons(f60(f4(Str::lit(")"))), List::<Adt6>::nil());
        append_list(a284, a285)
    }))
}

pub fn f97(v0: Adt0, v1: Bytes) -> List<Adt6> {
    let m286 = f2(f3(v0, v1));
    match m286 {
        None => {
            List::<Adt6>::nil()
        }
        Some(v2) => {
            f157(Fn19::F158, List::<Adt6>::nil(), f11(v2))
        }
    }
}

pub fn f98(v0: Adt0, v1: Bytes) -> R<Bytes> {
    let m287 = f3(v0, v1);
    match m287 {
        None => {
            Ok(f4(Str::lit("")))
        }
        Some(v2) => {
            let m288 = f59(v2)?;
            match m288 {
                None => {
                    Ok(f4(Str::lit("")))
                }
                Some(v3) => {
                    Ok(v3)
                }
            }
        }
    }
}

pub fn f99(v0: Adt0, v1: List<(Bytes, Adt6)>) -> List<Adt6> {
    if f102(f3(v0, f4(Str::lit("type_arguments")))) {
        List::cons(f60(f4(Str::lit(" "))), List::cons(f64(v1, f4(Str::lit("type_arguments"))), List::<Adt6>::nil()))
    } else {
        List::<Adt6>::nil()
    }
}

pub fn f100(v0: Adt0, v1: Bytes) -> R<bool> {
    let m289 = f101(v1.clone());
    match m289 {
        None => {
            if f45(v1.clone(), f4(Str::lit("parameter"))) {
                Ok(f5(f133(v0.clone(), f134(Str::lit("name")))?, {
                    let m290 = f7(f3(v0, f4(Str::lit("name"))));
                    match m290 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            f103(v3)?
                        }
                    }
                }))
            } else {
                if if f45(v1.clone(), f4(Str::lit("list"))) {
                    true
                } else {
                    f45(v1.clone(), f4(Str::lit("set")))
                } {
                    f133(v0, f134(Str::lit("element")))
                } else {
                    if f45(v1.clone(), f4(Str::lit("option"))) {
                        f133(v0, f134(Str::lit("value")))
                    } else {
                        if f45(v1.clone(), f4(Str::lit("result"))) {
                            f133(v0, f138(Str::lit("ok"), Str::lit("error")))
                        } else {
                            if f45(v1.clone(), f4(Str::lit("map"))) {
                                f133(v0, f138(Str::lit("key"), Str::lit("value")))
                            } else {
                                if f45(v1.clone(), f4(Str::lit("product"))) {
                                    f133(v0, f138(Str::lit("left"), Str::lit("right")))
                                } else {
                                    if f45(v1.clone(), f4(Str::lit("function"))) {
                                        Ok(f5(f133(v0.clone(), f138(Str::lit("parameters"), Str::lit("result")))?, f145(v0, f4(Str::lit("parameters")))))
                                    } else {
                                        if f45(v1, f4(Str::lit("named"))) {
                                            Ok(f5(f133(v0.clone(), f138(Str::lit("member"), Str::lit("arguments")))?, {
                                                let m291 = f3(v0.clone(), f4(Str::lit("member")));
                                                match m291 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(v4) => {
                                                        let m292 = f1(v4.clone());
                                                        match m292 {
                                                            None => {
                                                                false
                                                            }
                                                            Some(v5) => {
                                                                let m293 = f7(f3(v5.clone(), f4(Str::lit("name"))));
                                                                match m293 {
                                                                    None => {
                                                                        false
                                                                    }
                                                                    Some(v6) => {
                                                                        f5(!f5(!f102(f3(v5, f4(Str::lit("module")))), f159(v6)), f5(f145(v0, f4(Str::lit("arguments"))), f102({
                                                                            let m294 = f59(v4)?;
                                                                            match m294 {
                                                                                None => {
                                                                                    None::<Adt0>
                                                                                }
                                                                                Some(_) => {
                                                                                    Some(Adt0::C5)
                                                                                }
                                                                            }
                                                                        })))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            f133(v0, List::cons(f4(Str::lit("kind")), List::<Bytes>::nil()))
        }
    }
}

pub fn f101(v0: Bytes) -> Option<Bytes> {
    if f45(v0.clone(), f4(Str::lit("bool"))) {
        Some(f4(Str::lit("Bool")))
    } else {
        if f45(v0.clone(), f4(Str::lit("bytes"))) {
            Some(f4(Str::lit("Bytes")))
        } else {
            if f45(v0.clone(), f4(Str::lit("int"))) {
                Some(f4(Str::lit("Int")))
            } else {
                if f45(v0.clone(), f4(Str::lit("int8"))) {
                    Some(f4(Str::lit("Int8")))
                } else {
                    if f45(v0.clone(), f4(Str::lit("int16"))) {
                        Some(f4(Str::lit("Int16")))
                    } else {
                        if f45(v0.clone(), f4(Str::lit("int32"))) {
                            Some(f4(Str::lit("Int32")))
                        } else {
                            if f45(v0.clone(), f4(Str::lit("int64"))) {
                                Some(f4(Str::lit("Int64")))
                            } else {
                                if f45(v0.clone(), f4(Str::lit("nat"))) {
                                    Some(f4(Str::lit("Nat")))
                                } else {
                                    if f45(v0.clone(), f4(Str::lit("ordering"))) {
                                        Some(f4(Str::lit("Ordering")))
                                    } else {
                                        if f45(v0.clone(), f4(Str::lit("prop"))) {
                                            Some(f4(Str::lit("Prop")))
                                        } else {
                                            if f45(v0.clone(), f4(Str::lit("string"))) {
                                                Some(f4(Str::lit("String")))
                                            } else {
                                                if f45(v0.clone(), f4(Str::lit("type"))) {
                                                    Some(f4(Str::lit("Type")))
                                                } else {
                                                    if f45(v0.clone(), f4(Str::lit("uint8"))) {
                                                        Some(f4(Str::lit("UInt8")))
                                                    } else {
                                                        if f45(v0.clone(), f4(Str::lit("uint16"))) {
                                                            Some(f4(Str::lit("UInt16")))
                                                        } else {
                                                            if f45(v0.clone(), f4(Str::lit("uint32"))) {
                                                                Some(f4(Str::lit("UInt32")))
                                                            } else {
                                                                if f45(v0.clone(), f4(Str::lit("uint64"))) {
                                                                    Some(f4(Str::lit("UInt64")))
                                                                } else {
                                                                    if f45(v0.clone(), f4(Str::lit("unit"))) {
                                                                        Some(f4(Str::lit("Unit")))
                                                                    } else {
                                                                        if f45(v0.clone(), f4(Str::lit("contract_violation"))) {
                                                                            Some(f4(Str::lit("ContractViolation")))
                                                                        } else {
                                                                            if f45(v0, f4(Str::lit("reasoning_failure"))) {
                                                                                Some(f4(Str::lit("ReasoningFailure")))
                                                                            } else {
                                                                                None::<Bytes>
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

pub fn f102(v0: Option<Adt0>) -> bool {
    let m295 = v0;
    match m295 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f103(v0: Bytes) -> R<bool> {
    let a297 = {
        let a296 = v0.clone();
        length_bytes(a296)
    };
    let a298 = 0u64;
    if nat_eq(a297, a298) {
        Ok(false)
    } else {
        if f160(f161(v0.clone(), 0u64)?) {
            let a300 = f162(v0.clone(), 1u64)?;
            let a301 = {
                let a299 = v0.clone();
                length_bytes(a299)
            };
            if nat_eq(a300, a301) {
                let a302 = f163(v0);
                Ok(bool_not(a302))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f104(v0: Fn9, p306: Adt6, p307: List<Adt6>) -> Adt6 {
    let m305 = (p306, p307);
    {
        let mut m305 = m305;
        loop {
            let step = {
                let (v1, v2) = m305;
                let m303 = v2;
                match m303.uncons() {
                    None => {
                        Err::<(Adt6, List<Adt6>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<Adt6>), Adt6>(({
                            let c304: Fn9 = v0.clone();
                            c304.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Adt6, v1: Adt6) -> Adt6 {
    Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f106(v0: Fn10, p311: Adt6, p312: List<(Bytes, Adt6)>) -> Adt6 {
    let m310 = (p311, p312);
    {
        let mut m310 = m310;
        loop {
            let step = {
                let (v1, v2) = m310;
                let m308 = v2;
                match m308.uncons() {
                    None => {
                        Err::<(Adt6, List<(Bytes, Adt6)>), Adt6>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt6, List<(Bytes, Adt6)>), Adt6>(({
                            let c309: Fn10 = v0.clone();
                            c309.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: Bytes, v1: Adt6, v2: (Bytes, Adt6)) -> Adt6 {
    if f45({
        let (h313, _) = v2;
        h313
    }, v0) {
        let (_, h314) = v2;
        h314
    } else {
        v1
    }
}

pub fn f108(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt2> {
    let m315 = v1;
    match m315 {
        None => {
            Ok(Adt2::C5)
        }
        Some(v3) => {
            let m316 = f19(f30(v0.clone(), v3)?, f4(Str::lit("properties")));
            match m316 {
                None => {
                    Ok(Adt2::C5)
                }
                Some(v4) => {
                    let m317 = f19(v4, v2);
                    match m317 {
                        None => {
                            Ok(Adt2::C5)
                        }
                        Some(v5) => {
                            f31(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f109(v0: Bytes, v1: Bytes) -> Adt2 {
    if f45(v1.clone(), f4(Str::lit("element"))) {
        Adt2::C1
    } else {
        if f45(v1.clone(), f4(Str::lit("binder"))) {
            Adt2::C3
        } else {
            if f45(v1.clone(), f4(Str::lit("parameters"))) {
                Adt2::C8
            } else {
                if f45(v1.clone(), f4(Str::lit("arguments"))) {
                    Adt2::C6
                } else {
                    if f45(v1.clone(), f4(Str::lit("type_arguments"))) {
                        Adt2::C7
                    } else {
                        if f45(v1.clone(), f4(Str::lit("fields"))) {
                            Adt2::C11
                        } else {
                            if f45(v1.clone(), f4(Str::lit("branches"))) {
                                Adt2::C13
                            } else {
                                if f45(v1.clone(), f4(Str::lit("result"))) {
                                    Adt2::C1
                                } else {
                                    if if f45(v1.clone(), f4(Str::lit("type"))) {
                                        true
                                    } else {
                                        if f45(v1.clone(), f4(Str::lit("constructor"))) {
                                            true
                                        } else {
                                            f45(v1.clone(), f4(Str::lit("model")))
                                        }
                                    } {
                                        Adt2::C2
                                    } else {
                                        if f45(v1, f4(Str::lit("function"))) {
                                            if f45(v0, f4(Str::lit("apply"))) {
                                                Adt2::C0
                                            } else {
                                                Adt2::C2
                                            }
                                        } else {
                                            Adt2::C0
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

pub fn f110(v0: Bytes) -> Adt2 {
    if f45(v0.clone(), f4(Str::lit("parameters"))) {
        Adt2::C9
    } else {
        if f45(v0.clone(), f4(Str::lit("arguments"))) {
            Adt2::C10
        } else {
            if f45(v0.clone(), f4(Str::lit("member"))) {
                Adt2::C2
            } else {
                if f45(v0, f4(Str::lit("name"))) {
                    Adt2::C5
                } else {
                    Adt2::C1
                }
            }
        }
    }
}

pub fn f111(v0: Adt0, v1: Adt2) -> R<Option<Adt0>> {
    let m318 = v1;
    match m318 {
        Adt2::C0 => {
            Ok(f21(v0, f4(Str::lit("term"))))
        }
        Adt2::C1 => {
            Ok(f21(v0, f4(Str::lit("type"))))
        }
        Adt2::C2 => {
            Ok(None::<Adt0>)
        }
        Adt2::C3 => {
            Ok(None::<Adt0>)
        }
        Adt2::C4(r319) => {
            let v2 = (*r319).clone();
            if f113(v0, v2.clone())? {
                Ok(Some(v2))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Adt2::C5 => {
            Ok(None::<Adt0>)
        }
        Adt2::C6 => {
            Ok(None::<Adt0>)
        }
        Adt2::C7 => {
            Ok(None::<Adt0>)
        }
        Adt2::C8 => {
            Ok(None::<Adt0>)
        }
        Adt2::C9 => {
            Ok(None::<Adt0>)
        }
        Adt2::C10 => {
            Ok(None::<Adt0>)
        }
        Adt2::C11 => {
            Ok(None::<Adt0>)
        }
        Adt2::C12 => {
            Ok(None::<Adt0>)
        }
        Adt2::C13 => {
            Ok(None::<Adt0>)
        }
        Adt2::C14 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f112(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m320 = f2(f19(f30(v0.clone(), v1)?, f4(Str::lit("oneOf"))));
    match m320 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f164(Fn20::F165(v2, std::rc::Rc::new(v0)), None::<Adt0>, f11(v3))
        }
    }
}

pub fn f113(v0: Adt0, v1: Adt0) -> R<bool> {
    let m321 = f2(f19(f30(v0.clone(), v1)?, f4(Str::lit("oneOf"))));
    match m321 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f166(Fn21::F167(std::rc::Rc::new(v0)), true, f11(v2))
        }
    }
}

pub fn f114(v0: Bytes, v1: Bytes, v2: List<Adt6>, v3: Adt6) -> List<Adt6> {
    List::cons(f60(v0), List::cons(v3, List::cons(f60(v1), v2)))
}

pub fn f115(v0: Fn11, p325: (bool, List<Adt6>), p326: List<Adt6>) -> (bool, List<Adt6>) {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m322 = v2;
                match m322.uncons() {
                    None => {
                        Err::<((bool, List<Adt6>), List<Adt6>), (bool, List<Adt6>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt6>), List<Adt6>), (bool, List<Adt6>)>(({
                            let c323: Fn11 = v0.clone();
                            c323.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: (bool, List<Adt6>), v2: Adt6) -> (bool, List<Adt6>) {
    let (h327, _) = v1;
    if h327 {
        (false, List::cons(v2, {
            let (_, h328) = v1;
            h328
        }))
    } else {
        (false, List::cons(v2, List::cons(f60(v0), {
            let (_, h329) = v1;
            h329
        })))
    }
}

pub fn f117(v0: Bytes, v1: Bytes) -> Bytes {
    let a330 = v0;
    let a331 = v1;
    append_bytes(a330, a331)
}

pub fn f118(v0: Fn12, p335: (bool, Adt6), p336: List<Adt6>) -> (bool, Adt6) {
    let m334 = (p335, p336);
    {
        let mut m334 = m334;
        loop {
            let step = {
                let (v1, v2) = m334;
                let m332 = v2;
                match m332.uncons() {
                    None => {
                        Err::<((bool, Adt6), List<Adt6>), (bool, Adt6)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt6), List<Adt6>), (bool, Adt6)>(({
                            let c333: Fn12 = v0.clone();
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

pub fn f119(v0: (bool, Adt6), v1: Adt6) -> (bool, Adt6) {
    f168(v0, v1)
}

pub fn f120(v0: List<Adt6>, v1: Adt6) -> List<Adt6> {
    List::cons(v1, v0)
}

pub fn f121(v0: u64) -> Bytes {
    f169(f170(), 10u64, v0)
}

pub fn f122(v0: Fn13, p340: List<(Bytes, Adt6)>, p341: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v2;
                match m337.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), R<List<(Bytes, Adt6)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt6)>, List<(Bytes, Adt6)>), R<List<(Bytes, Adt6)>>>(({
                            let c338: Fn13 = v0.clone();
                            c338.apply(v1, v3)?
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

pub fn f123(v0: Option<Bytes>, v1: List<(Bytes, Adt6)>, v2: (Bytes, Adt6)) -> R<List<(Bytes, Adt6)>> {
    let m342 = v0;
    match m342 {
        None => {
            Ok(List::cons((f171({
                let (h349, _) = v2.clone();
                h349
            }), f172({
                let (h350, _) = v2;
                h350
            }, {
                let (_, h351) = v2;
                h351
            })?), v1))
        }
        Some(_) => {
            let a344 = {
                let (h343, _) = v2.clone();
                h343
            };
            let a345 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a344, a345) {
                Ok(v1)
            } else {
                Ok(List::cons((f171({
                    let (h346, _) = v2.clone();
                    h346
                }), f172({
                    let (h347, _) = v2;
                    h347
                }, {
                    let (_, h348) = v2;
                    h348
                })?), v1))
            }
        }
    }
}

pub fn f124(v0: Fn14, p355: List<Adt6>, p356: List<(Bytes, Adt6)>) -> List<Adt6> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v1, v2) = m354;
                let m352 = v2;
                match m352.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<(Bytes, Adt6)>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<(Bytes, Adt6)>), List<Adt6>>(({
                            let c353: Fn14 = v0.clone();
                            c353.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<Adt6>, v1: (Bytes, Adt6)) -> List<Adt6> {
    List::cons({
        let (_, h357) = v1;
        h357
    }, v0)
}

pub fn f126(v0: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v1: List<List<(Bytes, Adt6)>> = f173(f174(Fn22::F175, List::<List<(Bytes, Adt6)>>::nil(), v0.clone()));
    let m362 = {
        let (h358, _) = f176(Fn23::F177, {
            let a360 = {
                let a359 = v0;
                length_list(a359)
            };
            let a361 = 1u64;
            nat_add(a360, a361)?
        }, v1)?;
        h358
    };
    match m362.uncons() {
        None => {
            Ok(List::<(Bytes, Adt6)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f127(v0: Bytes, v1: Adt12) -> R<Option<Adt12>> {
    let v2: u64 = match v1.clone() {
        Adt12::C0(h363, _) => {
            h363
        }
    };
    let m364 = f178(v0, v2);
    match m364 {
        None => {
            Ok(None::<Adt12>)
        }
        Some(v3) => {
            Ok(Some(Adt12::C0({
                let a365 = v2;
                let a366 = 1u64;
                nat_add(a365, a366)?
            }, {
                let a413 = {
                    let v4: u8 = v3;
                    let a411 = {
                        let a387 = {
                            let a375 = {
                                let a369 = {
                                    let a367 = v4;
                                    let a368 = 128u8;
                                    fixed_u8::bit_and(a367, a368)
                                };
                                let a370 = 128u8;
                                if equal(a369, a370) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a376 = {
                                let a373 = {
                                    let a371 = v4;
                                    let a372 = 64u8;
                                    fixed_u8::bit_and(a371, a372)
                                };
                                let a374 = 64u8;
                                if equal(a373, a374) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a375, a376)?
                        };
                        let a388 = {
                            let a385 = {
                                let a379 = {
                                    let a377 = v4;
                                    let a378 = 32u8;
                                    fixed_u8::bit_and(a377, a378)
                                };
                                let a380 = 32u8;
                                if equal(a379, a380) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a386 = {
                                let a383 = {
                                    let a381 = v4;
                                    let a382 = 16u8;
                                    fixed_u8::bit_and(a381, a382)
                                };
                                let a384 = 16u8;
                                if equal(a383, a384) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a385, a386)?
                        };
                        nat_add(a387, a388)?
                    };
                    let a412 = {
                        let a409 = {
                            let a397 = {
                                let a391 = {
                                    let a389 = v4;
                                    let a390 = 8u8;
                                    fixed_u8::bit_and(a389, a390)
                                };
                                let a392 = 8u8;
                                if equal(a391, a392) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a398 = {
                                let a395 = {
                                    let a393 = v4;
                                    let a394 = 4u8;
                                    fixed_u8::bit_and(a393, a394)
                                };
                                let a396 = 4u8;
                                if equal(a395, a396) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a397, a398)?
                        };
                        let a410 = {
                            let a407 = {
                                let a401 = {
                                    let a399 = v4;
                                    let a400 = 2u8;
                                    fixed_u8::bit_and(a399, a400)
                                };
                                let a402 = 2u8;
                                if equal(a401, a402) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a408 = {
                                let a405 = {
                                    let a403 = v4;
                                    let a404 = 1u8;
                                    fixed_u8::bit_and(a403, a404)
                                };
                                let a406 = 1u8;
                                if equal(a405, a406) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a407, a408)?
                        };
                        nat_add(a409, a410)?
                    };
                    nat_add(a411, a412)?
                };
                let a414 = 47u64;
                if nat_eq(a413, a414) {
                    let a415 = v2;
                    let a416 = 1u64;
                    nat_add(a415, a416)?
                } else {
                    match v1 {
                        Adt12::C0(_, h417) => {
                            h417
                        }
                    }
                }
            })))
        }
    }
}

pub fn f128(v0: u64, v1: u64, v2: Adt7, v3: Adt8, v4: List<Adt8>) -> R<Adt7> {
    let v5: u64 = match v3.clone() {
        Adt8::C0(h418, _, _) => {
            h418
        }
    };
    let v6: bool = match v3.clone() {
        Adt8::C0(_, h419, _) => {
            h419
        }
    };
    let m421 = match v3 {
        Adt8::C0(_, _, h420) => {
            (*h420).clone()
        }
    };
    match m421 {
        Adt6::C0 => {
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h422, _, _) => {
                    h422
                }
            }, match v2 {
                Adt7::C0(_, h423, _) => {
                    h423
                }
            }, v4))
        }
        Adt6::C1(v7) => {
            Ok(Adt7::C0(List::cons(v7.clone(), match v2.clone() {
                Adt7::C0(h424, _, _) => {
                    h424
                }
            }), {
                let a426 = match v2 {
                    Adt7::C0(_, h425, _) => {
                        h425
                    }
                };
                let a427 = f179(v7)?;
                nat_add(a426, a427)?
            }, v4))
        }
        Adt6::C2(r444, r445) => {
            let v8 = (*r444).clone();
            let v9 = (*r445).clone();
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h428, _, _) => {
                    h428
                }
            }, match v2 {
                Adt7::C0(_, h429, _) => {
                    h429
                }
            }, List::cons(Adt8::C0(v5, v6, std::rc::Rc::new(v8)), List::cons(Adt8::C0(v5, v6, std::rc::Rc::new(v9)), v4))))
        }
        Adt6::C3(v10, r446) => {
            let v11 = (*r446).clone();
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h430, _, _) => {
                    h430
                }
            }, match v2 {
                Adt7::C0(_, h431, _) => {
                    h431
                }
            }, List::cons(Adt8::C0({
                let a432 = v5;
                let a433 = v10;
                nat_add(a432, a433)?
            }, v6, std::rc::Rc::new(v11)), v4)))
        }
        Adt6::C4 => {
            if v6 {
                Ok(Adt7::C0(List::cons({
                    let a434 = Str::lit(" ");
                    utf8_encode(a434)
                }, match v2.clone() {
                    Adt7::C0(h435, _, _) => {
                        h435
                    }
                }), {
                    let a437 = match v2 {
                        Adt7::C0(_, h436, _) => {
                            h436
                        }
                    };
                    let a438 = 1u64;
                    nat_add(a437, a438)?
                }, v4))
            } else {
                f180(v2, v5, v4)
            }
        }
        Adt6::C5 => {
            if v6 {
                Ok(Adt7::C0(match v2.clone() {
                    Adt7::C0(h439, _, _) => {
                        h439
                    }
                }, match v2 {
                    Adt7::C0(_, h440, _) => {
                        h440
                    }
                }, v4))
            } else {
                f180(v2, v5, v4)
            }
        }
        Adt6::C6(r447) => {
            let v12 = (*r447).clone();
            let v13: bool = f181(v1, v0, match v2.clone() {
                Adt7::C0(_, h441, _) => {
                    h441
                }
            }, v4.clone(), Adt8::C0(v5, true, std::rc::Rc::new(v12.clone())))?;
            Ok(Adt7::C0(match v2.clone() {
                Adt7::C0(h442, _, _) => {
                    h442
                }
            }, match v2 {
                Adt7::C0(_, h443, _) => {
                    h443
                }
            }, List::cons(Adt8::C0(v5, v13, std::rc::Rc::new(v12)), v4)))
        }
    }
}

pub fn f129(v0: Fn15, p452: u64, p453: List<Bytes>) -> (List<Bytes>, bool) {
    let m451 = (p452, p453);
    {
        let mut m451 = m451;
        loop {
            let step = {
                let (v1, v2) = m451;
                let m448 = v1;
                if m448 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m448 - 1;
                    let m450 = {
                        let c449: Fn15 = v0.clone();
                        c449.apply(v2.clone())
                    };
                    match m450 {
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
                Ok(next) => { m451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>) -> Option<List<Bytes>> {
    f182(v0)
}

pub fn f131(v0: Fn16, p457: List<Bytes>, p458: List<Bytes>) -> List<Bytes> {
    let m456 = (p457, p458);
    {
        let mut m456 = m456;
        loop {
            let step = {
                let (v1, v2) = m456;
                let m454 = v2;
                match m454.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c455: Fn16 = v0.clone();
                            c455.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f133(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f5({
        let a460 = f6(v0.clone())?;
        let a461 = {
            let a459 = v1.clone();
            length_list(a459)
        };
        nat_eq(a460, a461)
    }, f183(v0, v1)))
}

pub fn f134(v0: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::<Bytes>::nil()))
}

pub fn f135(v0: Bytes) -> R<bool> {
    Ok(f5(f103(v0.clone())?, f5(!f184(v0.clone(), f185()), !f45(v0, f4(Str::lit("false"))))))
}

pub fn f136(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a462 = v0.clone();
        length_bytes(a462)
    };
    Ok(f5({
        let a463 = 0u64;
        let a464 = v1;
        nat_lt(a463, a464)
    }, f5(f186(v0.clone(), 0u64, false)?, {
        let a465 = v1;
        let a466 = 1u64;
        if nat_eq(a465, a466) {
            true
        } else {
            !f187(v0, f4(Str::lit("0")))
        }
    })))
}

pub fn f137(v0: Bytes) -> R<bool> {
    Ok(f5({
        let a471 = {
            let a468 = {
                let a467 = v0.clone();
                length_bytes(a467)
            };
            let a469 = 2u64;
            let a470 = 0u64;
            nat_rem(a468, a469, a470)
        };
        let a472 = 0u64;
        nat_eq(a471, a472)
    }, f186(v0, 0u64, true)?))
}

pub fn f138(v0: Str, v1: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::cons(f4(v1), List::<Bytes>::nil())))
}

pub fn f139(v0: Bytes) -> R<bool> {
    if f187(v0.clone(), f4(Str::lit("-"))) {
        let v1: Bytes = f85(v0.clone(), 1u64, {
            let a474 = {
                let a473 = v0;
                length_bytes(a473)
            };
            let a475 = 1u64;
            nat_sub(a474, a475)
        });
        Ok(f5(f136(v1.clone())?, !f45(v1, f4(Str::lit("0")))))
    } else {
        f136(v0)
    }
}

pub fn f140(v0: Str, v1: Str, v2: Str) -> List<Bytes> {
    List::cons(f4(Str::lit("kind")), List::cons(f4(v0), List::cons(f4(v1), List::cons(f4(v2), List::<Bytes>::nil()))))
}

pub fn f141(v0: Option<Adt0>) -> R<bool> {
    let m476 = v0;
    match m476 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f61(v1)
        }
    }
}

pub fn f142(v0: Adt0, v1: Bytes, v2: Fn17) -> R<bool> {
    let m477 = f2(f3(v0, v1));
    match m477 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            f166(Fn21::F188(v2), true, f11(v3))
        }
    }
}

pub fn f143(v0: Adt0) -> R<bool> {
    f61(v0)
}

pub fn f144(v0: Adt0) -> R<bool> {
    f189(v0)
}

pub fn f145(v0: Adt0, v1: Bytes) -> bool {
    f102(f2(f3(v0, v1)))
}

pub fn f146(v0: Adt0, v1: List<Bytes>) -> R<bool> {
    Ok(f5({
        let a481 = f6(v0.clone())?;
        let a482 = {
            let a479 = {
                let a478 = v1.clone();
                length_list(a478)
            };
            let a480 = if f102(f3(v0.clone(), f4(Str::lit("type_arguments")))) {
                1u64
            } else {
                0u64
            };
            nat_add(a479, a480)?
        };
        nat_eq(a481, a482)
    }, f183(v0, v1)))
}

pub fn f147(v0: Adt0) -> R<bool> {
    let m483 = f3(v0, f4(Str::lit("type_arguments")));
    match m483 {
        None => {
            Ok(true)
        }
        Some(v1) => {
            let m484 = f13(v1);
            match m484 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    let a485 = 0u64;
                    let a486 = f6(v2)?;
                    Ok(nat_lt(a485, a486))
                }
            }
        }
    }
}

pub fn f148(v0: Option<Adt0>) -> R<bool> {
    let m487 = v0;
    match m487 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            let m488 = f59(v1)?;
            match m488 {
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

pub fn f149(v0: Adt0) -> R<bool> {
    f190(v0)
}

pub fn f150(v0: Adt0) -> R<bool> {
    f189(v0)
}

pub fn f151(v0: Adt0) -> R<bool> {
    f191(v0)
}

pub fn f152(v0: Fn18, p493: u64, p494: Adt13) -> R<(Adt13, bool)> {
    let m492 = (p493, p494);
    {
        let mut m492 = m492;
        loop {
            let step = {
                let (v1, v2) = m492;
                let m489 = v1;
                if m489 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m489 - 1;
                    let m491 = {
                        let c490: Fn18 = v0.clone();
                        c490.apply(v2.clone())?
                    };
                    match m491 {
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
                Ok(next) => { m492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: u64, v1: Bytes, v2: Adt13) -> R<Option<Adt13>> {
    f192(v1, v0, v2)
}

pub fn f154(v0: Bytes, v1: Bytes) -> Bytes {
    let a495 = v0;
    let a496 = v1;
    append_bytes(a495, a496)
}

pub fn f155(v0: Str) -> Bytes {
    let a497 = v0;
    utf8_encode(a497)
}

pub fn f156(v0: List<Bytes>) -> Bytes {
    f87(f88(v0))
}

pub fn f157(v0: Fn19, p501: List<Adt6>, p502: List<Adt0>) -> List<Adt6> {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, v2) = m500;
                let m498 = v2;
                match m498.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt0>), List<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt6>, List<Adt0>), List<Adt6>>(({
                            let c499: Fn19 = v0.clone();
                            c499.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: List<Adt6>, v1: Adt0) -> List<Adt6> {
    let a504 = v0;
    let a505 = List::cons(f60({
        let m503 = f18(v1);
        match m503 {
            None => {
                f4(Str::lit(""))
            }
            Some(v2) => {
                v2
            }
        }
    }), List::<Adt6>::nil());
    append_list(a504, a505)
}

pub fn f159(v0: Bytes) -> bool {
    f184(v0, List::cons(f4(Str::lit("Bool")), List::cons(f4(Str::lit("Bytes")), List::cons(f4(Str::lit("Int")), List::cons(f4(Str::lit("Int8")), List::cons(f4(Str::lit("Int16")), List::cons(f4(Str::lit("Int32")), List::cons(f4(Str::lit("Int64")), List::cons(f4(Str::lit("Nat")), List::cons(f4(Str::lit("Ordering")), List::cons(f4(Str::lit("Prop")), List::cons(f4(Str::lit("String")), List::cons(f4(Str::lit("Type")), List::cons(f4(Str::lit("UInt8")), List::cons(f4(Str::lit("UInt16")), List::cons(f4(Str::lit("UInt32")), List::cons(f4(Str::lit("UInt64")), List::cons(f4(Str::lit("Unit")), List::cons(f4(Str::lit("ContractViolation")), List::cons(f4(Str::lit("ReasoningFailure")), List::cons(f4(Str::lit("List")), List::cons(f4(Str::lit("Option")), List::cons(f4(Str::lit("Result")), List::cons(f4(Str::lit("Set")), List::cons(f4(Str::lit("Map")), List::cons(f4(Str::lit("Fn")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f160(v0: u64) -> bool {
    let a506 = 64u64;
    let a507 = v0;
    if if nat_lt(a506, a507) {
        let a508 = v0;
        let a509 = 91u64;
        nat_lt(a508, a509)
    } else {
        false
    } {
        true
    } else {
        let a510 = 96u64;
        let a511 = v0;
        if if nat_lt(a510, a511) {
            let a512 = v0;
            let a513 = 123u64;
            nat_lt(a512, a513)
        } else {
            false
        } {
            true
        } else {
            let a514 = v0;
            let a515 = 95u64;
            nat_eq(a514, a515)
        }
    }
}

pub fn f161(v0: Bytes, v1: u64) -> R<u64> {
    let m516 = f178(v0, v1);
    match m516 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a561 = {
                let a537 = {
                    let a525 = {
                        let a519 = {
                            let a517 = v3;
                            let a518 = 128u8;
                            fixed_u8::bit_and(a517, a518)
                        };
                        let a520 = 128u8;
                        if equal(a519, a520) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a526 = {
                        let a523 = {
                            let a521 = v3;
                            let a522 = 64u8;
                            fixed_u8::bit_and(a521, a522)
                        };
                        let a524 = 64u8;
                        if equal(a523, a524) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a525, a526)?
                };
                let a538 = {
                    let a535 = {
                        let a529 = {
                            let a527 = v3;
                            let a528 = 32u8;
                            fixed_u8::bit_and(a527, a528)
                        };
                        let a530 = 32u8;
                        if equal(a529, a530) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a536 = {
                        let a533 = {
                            let a531 = v3;
                            let a532 = 16u8;
                            fixed_u8::bit_and(a531, a532)
                        };
                        let a534 = 16u8;
                        if equal(a533, a534) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a535, a536)?
                };
                nat_add(a537, a538)?
            };
            let a562 = {
                let a559 = {
                    let a547 = {
                        let a541 = {
                            let a539 = v3;
                            let a540 = 8u8;
                            fixed_u8::bit_and(a539, a540)
                        };
                        let a542 = 8u8;
                        if equal(a541, a542) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a548 = {
                        let a545 = {
                            let a543 = v3;
                            let a544 = 4u8;
                            fixed_u8::bit_and(a543, a544)
                        };
                        let a546 = 4u8;
                        if equal(a545, a546) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a547, a548)?
                };
                let a560 = {
                    let a557 = {
                        let a551 = {
                            let a549 = v3;
                            let a550 = 2u8;
                            fixed_u8::bit_and(a549, a550)
                        };
                        let a552 = 2u8;
                        if equal(a551, a552) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a558 = {
                        let a555 = {
                            let a553 = v3;
                            let a554 = 1u8;
                            fixed_u8::bit_and(a553, a554)
                        };
                        let a556 = 1u8;
                        if equal(a555, a556) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a557, a558)?
                };
                nat_add(a559, a560)?
            };
            nat_add(a561, a562)
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<u64> {
    let (h563, _) = f193(Fn24::F194(v0.clone()), {
        let a565 = {
            let a564 = v0;
            length_bytes(a564)
        };
        let a566 = 1u64;
        nat_add(a565, a566)?
    }, v1)?;
    Ok(h563)
}

pub fn f163(v0: Bytes) -> bool {
    let a567 = v0.clone();
    let a568 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a567, a568) {
        true
    } else {
        let a569 = v0;
        let a570 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a569, a570)
    }
}

pub fn f164(v0: Fn20, p574: Option<Adt0>, p575: List<Adt0>) -> R<Option<Adt0>> {
    let m573 = (p574, p575);
    {
        let mut m573 = m573;
        loop {
            let step = {
                let (v1, v2) = m573;
                let m571 = v2;
                match m571.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c572: Fn20 = v0.clone();
                            c572.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m576 = v2.clone();
    match m576 {
        None => {
            let v5: Adt0 = f30(v1, v3)?;
            if f195(v5.clone(), v0)? {
                Ok(Some(v5))
            } else {
                Ok(None::<Adt0>)
            }
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f166(v0: Fn21, p580: bool, p581: List<Adt0>) -> R<bool> {
    let m579 = (p580, p581);
    {
        let mut m579 = m579;
        loop {
            let step = {
                let (v1, v2) = m579;
                let m577 = v2;
                match m577.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c578: Fn21 = v0.clone();
                            c578.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m582 = f19(f30(v0, v2)?, f4(Str::lit("properties")));
        match m582 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f102(f19(v3, f4(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f168(v0: (bool, Adt6), v1: Adt6) -> (bool, Adt6) {
    let (h583, _) = v0;
    if h583 {
        (false, Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C0)))
    } else {
        (false, Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C1(f77(Str::lit(",")))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(Adt6::C4), std::rc::Rc::new({
            let (_, h584) = v0;
            h584
        })))))))
    }
}

pub fn f169(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a585 = v2;
    let a586 = 0u64;
    if nat_eq(a585, a586) {
        f196(v0, 0u64)
    } else {
        let (_, h587) = {
            let (h588, _) = f197(Fn25::F198(v1, v0), v2, (v2, Bytes::lit(&[])));
            h588
        };
        h587
    }
}

pub fn f170() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f171(v0: Bytes) -> Bytes {
    f117(f199(f200(v0.clone())), v0)
}

pub fn f172(v0: Bytes, v1: Adt6) -> R<Adt6> {
    Ok(Adt6::C2(std::rc::Rc::new(Adt6::C1(f117(f76(v0)?, f77(Str::lit(": "))))), std::rc::Rc::new(Adt6::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt6::C0)))))
}

pub fn f173(v0: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    f201(Fn26::F202, List::<List<(Bytes, Adt6)>>::nil(), v0)
}

pub fn f174(v0: Fn22, p592: List<List<(Bytes, Adt6)>>, p593: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    let m591 = (p592, p593);
    {
        let mut m591 = m591;
        loop {
            let step = {
                let (v1, v2) = m591;
                let m589 = v2;
                match m589.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<(Bytes, Adt6)>), List<List<(Bytes, Adt6)>>>(({
                            let c590: Fn22 = v0.clone();
                            c590.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: List<List<(Bytes, Adt6)>>, v1: (Bytes, Adt6)) -> List<List<(Bytes, Adt6)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt6)>::nil()), v0)
}

pub fn f176(v0: Fn23, p598: u64, p599: List<List<(Bytes, Adt6)>>) -> R<(List<List<(Bytes, Adt6)>>, bool)> {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v1, v2) = m597;
                let m594 = v1;
                if m594 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m594 - 1;
                    let m596 = {
                        let c595: Fn23 = v0.clone();
                        c595.apply(v2.clone())?
                    };
                    match m596 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt6)>>), R<(List<List<(Bytes, Adt6)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    f203(v0)
}

pub fn f178(v0: Bytes, v1: u64) -> Option<u8> {
    let a600 = v0;
    let a601 = v1;
    index_bytes(a600, a601)
}

pub fn f179(v0: Bytes) -> R<u64> {
    let (_, h602) = {
        let (h603, _) = f204(Fn27::F205(v0.clone()), {
            let a604 = v0;
            length_bytes(a604)
        }, (0u64, 0u64))?;
        h603
    };
    Ok(h602)
}

pub fn f180(v0: Adt7, v1: u64, v2: List<Adt8>) -> R<Adt7> {
    Ok(Adt7::C0(f206(match v0 {
        Adt7::C0(h605, _, _) => {
            h605
        }
    }, v1)?, v1, v2))
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: List<Adt8>, v4: Adt8) -> R<bool> {
    let v5: Adt14 = {
        let (h606, _) = f207(Fn28::F208, {
            let a607 = v0;
            let a608 = 1u64;
            nat_add(a607, a608)?
        }, Adt14::C0(0u64, {
            let a609 = v1;
            let a610 = v2;
            nat_sub(a609, a610)
        }, {
            let a611 = v1;
            let a612 = v2;
            nat_lt(a611, a612)
        }, List::cons(v4, List::<Adt8>::nil()), v3, None::<bool>))?;
        h606
    };
    let m614 = match v5 {
        Adt14::C0(_, _, _, _, _, h613) => {
            h613
        }
    };
    match m614 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f182(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m615 = v0.clone();
    match m615.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m616 = v2;
            match m616.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f209(Fn29::F210, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m618 = {
                        let (h617, _) = v5;
                        h617
                    };
                    match m618 {
                        None => {
                            Some(f88({
                                let (_, h620) = v5;
                                h620
                            }))
                        }
                        Some(v6) => {
                            Some(f88(List::cons(v6, {
                                let (_, h619) = v5;
                                h619
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f183(v0: Adt0, v1: List<Bytes>) -> bool {
    f211(Fn30::F212(std::rc::Rc::new(v0)), true, v1)
}

pub fn f184(v0: Bytes, v1: List<Bytes>) -> bool {
    f211(Fn30::F213(v0), false, v1)
}

pub fn f185() -> List<Bytes> {
    List::cons(f4(Str::lit("let")), List::cons(f4(Str::lit("if")), List::cons(f4(Str::lit("match")), List::cons(f4(Str::lit("fun")), List::cons(f4(Str::lit("forall")), List::cons(f4(Str::lit("nil")), List::cons(f4(Str::lit("cons")), List::cons(f4(Str::lit("first")), List::cons(f4(Str::lit("second")), List::cons(f4(Str::lit("not")), List::cons(f4(Str::lit("apply")), List::cons(f4(Str::lit("ctor")), List::cons(f4(Str::lit("record")), List::cons(f4(Str::lit("project")), List::cons(f4(Str::lit("prim")), List::cons(f4(Str::lit("ref")), List::cons(f4(Str::lit("checked")), List::cons(f4(Str::lit("then")), List::cons(f4(Str::lit("else")), List::cons(f4(Str::lit("with")), List::cons(f4(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f186(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = {
        let (h621, _) = f193(Fn24::F214(v2, v0.clone()), {
            let a623 = {
                let a622 = v0.clone();
                length_bytes(a622)
            };
            let a624 = 1u64;
            nat_add(a623, a624)?
        }, v1)?;
        h621
    };
    let a626 = v3;
    let a627 = {
        let a625 = v0;
        length_bytes(a625)
    };
    Ok(nat_eq(a626, a627))
}

pub fn f187(v0: Bytes, v1: Bytes) -> bool {
    let m632 = {
        let a629 = v0;
        let a630 = 0u64;
        let a631 = {
            let a628 = v1.clone();
            length_bytes(a628)
        };
        slice_bytes(a629, a630, a631)
    };
    match m632 {
        None => {
            false
        }
        Some(v2) => {
            let a633 = v2;
            let a634 = v1;
            equal(a633, a634)
        }
    }
}

pub fn f188(v0: Fn17, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let c635: Fn17 = v0;
        c635.apply(v2)
    } else {
        Ok(false)
    }
}

pub fn f189(v0: Adt0) -> R<bool> {
    let m636 = f18(v0);
    match m636 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f103(v1)
        }
    }
}

pub fn f190(v0: Adt0) -> R<bool> {
    let m637 = f1(v0);
    match m637 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f5(f133(v1.clone(), List::cons(f4(Str::lit("field")), List::cons(f4(Str::lit("value")), List::<Bytes>::nil())))?, {
                let m638 = f7(f3(v1, f4(Str::lit("field"))));
                match m638 {
                    None => {
                        false
                    }
                    Some(v2) => {
                        f103(v2)?
                    }
                }
            }))
        }
    }
}

pub fn f191(v0: Adt0) -> R<bool> {
    let m639 = f1(v0);
    match m639 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f5(f133(v1.clone(), List::cons(f4(Str::lit("constructor")), List::cons(f4(Str::lit("binders")), List::cons(f4(Str::lit("body")), List::<Bytes>::nil()))))?, f5(f148(f3(v1.clone(), f4(Str::lit("constructor"))))?, f142(v1, f4(Str::lit("binders")), Fn17::F215)?)))
        }
    }
}

pub fn f192(v0: Bytes, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h640, _) => {
            h640
        }
    };
    let a641 = v3;
    let a642 = v1;
    if nat_lt(a641, a642) {
        let v4: u64 = f216(v0.clone(), v3)?;
        let a643 = v4;
        let a644 = 32u64;
        if if nat_lt(a643, a644) {
            true
        } else {
            let a645 = v4;
            let a646 = 34u64;
            if nat_eq(a645, a646) {
                true
            } else {
                let a647 = v4;
                let a648 = 92u64;
                nat_eq(a647, a648)
            }
        } {
            Ok(Some(Adt13::C0({
                let a649 = v3;
                let a650 = 1u64;
                nat_add(a649, a650)?
            }, List::cons(f217(v4), match v2 {
                Adt13::C0(_, h651) => {
                    h651
                }
            }))))
        } else {
            let v5: u64 = f218(v0.clone(), v1, v3)?;
            Ok(Some(Adt13::C0(v5, List::cons(f85(v0, v3, {
                let a652 = v5;
                let a653 = v3;
                nat_sub(a652, a653)
            }), match v2 {
                Adt13::C0(_, h654) => {
                    h654
                }
            }))))
        }
    } else {
        Ok(None::<Adt13>)
    }
}

pub fn f193(v0: Fn24, p659: u64, p660: u64) -> R<(u64, bool)> {
    let m658 = (p659, p660);
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v1, v2) = m658;
                let m655 = v1;
                if m655 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m655 - 1;
                    let m657 = {
                        let c656: Fn24 = v0.clone();
                        c656.apply(v2)?
                    };
                    match m657 {
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f219(v0, v1)
}

pub fn f195(v0: Adt0, v1: Bytes) -> R<bool> {
    let m661 = f19(v0, f4(Str::lit("properties")));
    match m661 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m662 = f19(v2, f4(Str::lit("kind")));
            match m662 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m663 = f7(f19(v3.clone(), f4(Str::lit("const"))));
                    if match m663 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f45(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m664 = f2(f19(v3, f4(Str::lit("enum"))));
                        match m664 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f166(Fn21::F220(v1), false, f11(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: u64) -> Bytes {
    let m668 = {
        let a665 = v0;
        let a666 = v1;
        let a667 = 1u64;
        slice_bytes(a665, a666, a667)
    };
    match m668 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f197(v0: Fn25, p673: u64, p674: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m672 = (p673, p674);
    {
        let mut m672 = m672;
        loop {
            let step = {
                let (v1, v2) = m672;
                let m669 = v1;
                if m669 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m669 - 1;
                    let m671 = {
                        let c670: Fn25 = v0.clone();
                        c670.apply(v2.clone())
                    };
                    match m671 {
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
                Ok(next) => { m672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f221(v1, v0, v2)
}

pub fn f199(v0: u64) -> Bytes {
    f85(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f200(v0: Bytes) -> u64 {
    let a675 = v0.clone();
    let a676 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a675, a676) {
        0u64
    } else {
        let a677 = v0.clone();
        let a678 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a677, a678) {
            1u64
        } else {
            let a679 = v0.clone();
            let a680 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a679, a680) {
                2u64
            } else {
                let a681 = v0.clone();
                let a682 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a681, a682) {
                    3u64
                } else {
                    let a683 = v0.clone();
                    let a684 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a683, a684) {
                        4u64
                    } else {
                        let a685 = v0.clone();
                        let a686 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a685, a686) {
                            5u64
                        } else {
                            let a687 = v0.clone();
                            let a688 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a687, a688) {
                                6u64
                            } else {
                                let a689 = v0.clone();
                                let a690 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a689, a690) {
                                    7u64
                                } else {
                                    let a691 = v0.clone();
                                    let a692 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a691, a692) {
                                        8u64
                                    } else {
                                        let a693 = v0.clone();
                                        let a694 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a693, a694) {
                                            9u64
                                        } else {
                                            let a695 = v0.clone();
                                            let a696 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a695, a696) {
                                                10u64
                                            } else {
                                                let a697 = v0;
                                                let a698 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a697, a698) {
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

pub fn f201(v0: Fn26, p702: List<List<(Bytes, Adt6)>>, p703: List<List<(Bytes, Adt6)>>) -> List<List<(Bytes, Adt6)>> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m699 = v2;
                match m699.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), List<List<(Bytes, Adt6)>>>(({
                            let c700: Fn26 = v0.clone();
                            c700.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: List<List<(Bytes, Adt6)>>, v1: List<(Bytes, Adt6)>) -> List<List<(Bytes, Adt6)>> {
    List::cons(v1, v0)
}

pub fn f203(v0: List<List<(Bytes, Adt6)>>) -> R<Option<List<List<(Bytes, Adt6)>>>> {
    let a705 = {
        let a704 = v0.clone();
        length_list(a704)
    };
    let a706 = 1u64;
    if nat_le(a705, a706) {
        Ok(None::<List<List<(Bytes, Adt6)>>>)
    } else {
        Ok(Some(f222(v0)?))
    }
}

pub fn f204(v0: Fn27, p711: u64, p712: (u64, u64)) -> R<((u64, u64), bool)> {
    let m710 = (p711, p712);
    {
        let mut m710 = m710;
        loop {
            let step = {
                let (v1, v2) = m710;
                let m707 = v1;
                if m707 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m707 - 1;
                    let m709 = {
                        let c708: Fn27 = v0.clone();
                        c708.apply(v2)?
                    };
                    match m709 {
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
                Ok(next) => { m710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f223(v0, v1)
}

pub fn f206(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a713 = v1;
        let a714 = 256u64;
        let a715 = 0u64;
        nat_quot(a713, a714, a715)
    };
    let v3: List<Bytes> = {
        let (_, h716) = {
            let (h717, _) = f224(Fn31::F225(v2), {
                let a718 = v2;
                let a719 = 1u64;
                nat_add(a718, a719)?
            }, (0u64, List::cons({
                let a720 = Str::lit("\u{a}");
                utf8_encode(a720)
            }, v0)))?;
            h717
        };
        h716
    };
    Ok(List::cons(f85(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a721 = v1;
        let a722 = 256u64;
        let a723 = 0u64;
        nat_rem(a721, a722, a723)
    }), v3))
}

pub fn f207(v0: Fn28, p728: u64, p729: Adt14) -> R<(Adt14, bool)> {
    let m727 = (p728, p729);
    {
        let mut m727 = m727;
        loop {
            let step = {
                let (v1, v2) = m727;
                let m724 = v1;
                if m724 == 0 {
                    Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m724 - 1;
                    let m726 = {
                        let c725: Fn28 = v0.clone();
                        c725.apply(v2.clone())?
                    };
                    match m726 {
                        None => {
                            Err::<(u64, Adt14), R<(Adt14, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt14), R<(Adt14, bool)>>((v3, v4))
                        }
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

pub fn f208(v0: Adt14) -> R<Option<Adt14>> {
    f226(v0)
}

pub fn f209(v0: Fn29, p733: (Option<Bytes>, List<Bytes>), p734: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m732 = (p733, p734);
    {
        let mut m732 = m732;
        loop {
            let step = {
                let (v1, v2) = m732;
                let m730 = v2;
                match m730.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c731: Fn29 = v0.clone();
                            c731.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f227(v0, v1)
}

pub fn f211(v0: Fn30, p738: bool, p739: List<Bytes>) -> bool {
    let m737 = (p738, p739);
    {
        let mut m737 = m737;
        loop {
            let step = {
                let (v1, v2) = m737;
                let m735 = v2;
                match m735.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c736: Fn30 = v0.clone();
                            c736.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Adt0, v1: bool, v2: Bytes) -> bool {
    if v1 {
        f102(f3(v0, v2))
    } else {
        false
    }
}

pub fn f213(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a740 = v2;
        let a741 = v0;
        equal(a740, a741)
    }
}

pub fn f214(v0: bool, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let m742 = f178(v1, v2);
    match m742 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let v5: u64 = {
                let v4: u8 = v3;
                let a787 = {
                    let a763 = {
                        let a751 = {
                            let a745 = {
                                let a743 = v4;
                                let a744 = 128u8;
                                fixed_u8::bit_and(a743, a744)
                            };
                            let a746 = 128u8;
                            if equal(a745, a746) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a752 = {
                            let a749 = {
                                let a747 = v4;
                                let a748 = 64u8;
                                fixed_u8::bit_and(a747, a748)
                            };
                            let a750 = 64u8;
                            if equal(a749, a750) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a751, a752)?
                    };
                    let a764 = {
                        let a761 = {
                            let a755 = {
                                let a753 = v4;
                                let a754 = 32u8;
                                fixed_u8::bit_and(a753, a754)
                            };
                            let a756 = 32u8;
                            if equal(a755, a756) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a762 = {
                            let a759 = {
                                let a757 = v4;
                                let a758 = 16u8;
                                fixed_u8::bit_and(a757, a758)
                            };
                            let a760 = 16u8;
                            if equal(a759, a760) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a761, a762)?
                    };
                    nat_add(a763, a764)?
                };
                let a788 = {
                    let a785 = {
                        let a773 = {
                            let a767 = {
                                let a765 = v4;
                                let a766 = 8u8;
                                fixed_u8::bit_and(a765, a766)
                            };
                            let a768 = 8u8;
                            if equal(a767, a768) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a774 = {
                            let a771 = {
                                let a769 = v4;
                                let a770 = 4u8;
                                fixed_u8::bit_and(a769, a770)
                            };
                            let a772 = 4u8;
                            if equal(a771, a772) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a773, a774)?
                    };
                    let a786 = {
                        let a783 = {
                            let a777 = {
                                let a775 = v4;
                                let a776 = 2u8;
                                fixed_u8::bit_and(a775, a776)
                            };
                            let a778 = 2u8;
                            if equal(a777, a778) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a784 = {
                            let a781 = {
                                let a779 = v4;
                                let a780 = 1u8;
                                fixed_u8::bit_and(a779, a780)
                            };
                            let a782 = 1u8;
                            if equal(a781, a782) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a783, a784)?
                    };
                    nat_add(a785, a786)?
                };
                nat_add(a787, a788)?
            };
            if if f5({
                let a789 = 48u64;
                let a790 = v5;
                nat_le(a789, a790)
            }, {
                let a791 = v5;
                let a792 = 57u64;
                nat_le(a791, a792)
            }) {
                true
            } else {
                f5(v0, f5({
                    let a793 = 97u64;
                    let a794 = v5;
                    nat_le(a793, a794)
                }, {
                    let a795 = v5;
                    let a796 = 102u64;
                    nat_le(a795, a796)
                }))
            } {
                Ok(Some({
                    let a797 = v2;
                    let a798 = 1u64;
                    nat_add(a797, a798)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f215(v0: Adt0) -> R<bool> {
    let m799 = f18(v0);
    match m799 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            f135(v1)
        }
    }
}

pub fn f216(v0: Bytes, v1: u64) -> R<u64> {
    let m800 = f178(v0, v1);
    match m800 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a845 = {
                let a821 = {
                    let a809 = {
                        let a803 = {
                            let a801 = v3;
                            let a802 = 128u8;
                            fixed_u8::bit_and(a801, a802)
                        };
                        let a804 = 128u8;
                        if equal(a803, a804) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a810 = {
                        let a807 = {
                            let a805 = v3;
                            let a806 = 64u8;
                            fixed_u8::bit_and(a805, a806)
                        };
                        let a808 = 64u8;
                        if equal(a807, a808) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a809, a810)?
                };
                let a822 = {
                    let a819 = {
                        let a813 = {
                            let a811 = v3;
                            let a812 = 32u8;
                            fixed_u8::bit_and(a811, a812)
                        };
                        let a814 = 32u8;
                        if equal(a813, a814) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a820 = {
                        let a817 = {
                            let a815 = v3;
                            let a816 = 16u8;
                            fixed_u8::bit_and(a815, a816)
                        };
                        let a818 = 16u8;
                        if equal(a817, a818) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a819, a820)?
                };
                nat_add(a821, a822)?
            };
            let a846 = {
                let a843 = {
                    let a831 = {
                        let a825 = {
                            let a823 = v3;
                            let a824 = 8u8;
                            fixed_u8::bit_and(a823, a824)
                        };
                        let a826 = 8u8;
                        if equal(a825, a826) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a832 = {
                        let a829 = {
                            let a827 = v3;
                            let a828 = 4u8;
                            fixed_u8::bit_and(a827, a828)
                        };
                        let a830 = 4u8;
                        if equal(a829, a830) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a831, a832)?
                };
                let a844 = {
                    let a841 = {
                        let a835 = {
                            let a833 = v3;
                            let a834 = 2u8;
                            fixed_u8::bit_and(a833, a834)
                        };
                        let a836 = 2u8;
                        if equal(a835, a836) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a842 = {
                        let a839 = {
                            let a837 = v3;
                            let a838 = 1u8;
                            fixed_u8::bit_and(a837, a838)
                        };
                        let a840 = 1u8;
                        if equal(a839, a840) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a841, a842)?
                };
                nat_add(a843, a844)?
            };
            nat_add(a845, a846)
        }
    }
}

pub fn f217(v0: u64) -> Bytes {
    let a847 = v0;
    let a848 = 34u64;
    if nat_eq(a847, a848) {
        f155(Str::lit("\\\""))
    } else {
        let a849 = v0;
        let a850 = 92u64;
        if nat_eq(a849, a850) {
            f155(Str::lit("\\\\"))
        } else {
            let a851 = v0;
            let a852 = 8u64;
            if nat_eq(a851, a852) {
                f155(Str::lit("\\b"))
            } else {
                let a853 = v0;
                let a854 = 12u64;
                if nat_eq(a853, a854) {
                    f155(Str::lit("\\f"))
                } else {
                    let a855 = v0;
                    let a856 = 10u64;
                    if nat_eq(a855, a856) {
                        f155(Str::lit("\\n"))
                    } else {
                        let a857 = v0;
                        let a858 = 13u64;
                        if nat_eq(a857, a858) {
                            f155(Str::lit("\\r"))
                        } else {
                            let a859 = v0;
                            let a860 = 9u64;
                            if nat_eq(a859, a860) {
                                f155(Str::lit("\\t"))
                            } else {
                                f154(f155(Str::lit("\\u00")), f228(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h861, _) = f193(Fn24::F229(v1, v0), {
        let a862 = v1;
        let a863 = v2;
        nat_sub(a862, a863)
    }, v2)?;
    Ok(h861)
}

pub fn f219(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f230(f161(v0, v1)?) {
        Ok(Some({
            let a864 = v1;
            let a865 = 1u64;
            nat_add(a864, a865)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f220(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m866 = f18(v2);
        match m866 {
            None => {
                false
            }
            Some(v3) => {
                f45(v3, v0)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h867, _) = v2;
        h867
    };
    let a868 = v3;
    let a869 = 0u64;
    if nat_eq(a868, a869) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a870 = v3;
            let a871 = v1;
            let a872 = 0u64;
            nat_quot(a870, a871, a872)
        }, {
            let a877 = f196(v0, {
                let a873 = v3;
                let a874 = v1;
                let a875 = 0u64;
                nat_rem(a873, a874, a875)
            });
            let a878 = {
                let (_, h876) = v2;
                h876
            };
            append_bytes(a877, a878)
        }))
    }
}

pub fn f222(v0: List<List<(Bytes, Adt6)>>) -> R<List<List<(Bytes, Adt6)>>> {
    Ok(f173({
        let (_, h879) = {
            let (h880, _) = f231(Fn32::F232, {
                let a882 = {
                    let a881 = v0.clone();
                    length_list(a881)
                };
                let a883 = 1u64;
                nat_add(a882, a883)?
            }, (v0, List::<List<(Bytes, Adt6)>>::nil()))?;
            h880
        };
        h879
    }))
}

pub fn f223(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m885 = f178(v0, {
        let (h884, _) = v1;
        h884
    });
    match m885 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a930 = {
                    let a906 = {
                        let a894 = {
                            let a888 = {
                                let a886 = v3;
                                let a887 = 128u8;
                                fixed_u8::bit_and(a886, a887)
                            };
                            let a889 = 128u8;
                            if equal(a888, a889) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a895 = {
                            let a892 = {
                                let a890 = v3;
                                let a891 = 64u8;
                                fixed_u8::bit_and(a890, a891)
                            };
                            let a893 = 64u8;
                            if equal(a892, a893) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a894, a895)?
                    };
                    let a907 = {
                        let a904 = {
                            let a898 = {
                                let a896 = v3;
                                let a897 = 32u8;
                                fixed_u8::bit_and(a896, a897)
                            };
                            let a899 = 32u8;
                            if equal(a898, a899) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a905 = {
                            let a902 = {
                                let a900 = v3;
                                let a901 = 16u8;
                                fixed_u8::bit_and(a900, a901)
                            };
                            let a903 = 16u8;
                            if equal(a902, a903) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a904, a905)?
                    };
                    nat_add(a906, a907)?
                };
                let a931 = {
                    let a928 = {
                        let a916 = {
                            let a910 = {
                                let a908 = v3;
                                let a909 = 8u8;
                                fixed_u8::bit_and(a908, a909)
                            };
                            let a911 = 8u8;
                            if equal(a910, a911) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a917 = {
                            let a914 = {
                                let a912 = v3;
                                let a913 = 4u8;
                                fixed_u8::bit_and(a912, a913)
                            };
                            let a915 = 4u8;
                            if equal(a914, a915) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a916, a917)?
                    };
                    let a929 = {
                        let a926 = {
                            let a920 = {
                                let a918 = v3;
                                let a919 = 2u8;
                                fixed_u8::bit_and(a918, a919)
                            };
                            let a921 = 2u8;
                            if equal(a920, a921) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a927 = {
                            let a924 = {
                                let a922 = v3;
                                let a923 = 1u8;
                                fixed_u8::bit_and(a922, a923)
                            };
                            let a925 = 1u8;
                            if equal(a924, a925) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a926, a927)?
                    };
                    nat_add(a928, a929)?
                };
                nat_add(a930, a931)?
            };
            let a932 = v4;
            let a933 = 128u64;
            if nat_lt(a932, a933) {
                Ok(Some(({
                    let a935 = {
                        let (h934, _) = v1;
                        h934
                    };
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                }, {
                    let a938 = {
                        let (_, h937) = v1;
                        h937
                    };
                    let a939 = 1u64;
                    nat_add(a938, a939)?
                })))
            } else {
                let a940 = v4;
                let a941 = 192u64;
                if nat_lt(a940, a941) {
                    Ok(Some(({
                        let a943 = {
                            let (h942, _) = v1;
                            h942
                        };
                        let a944 = 1u64;
                        nat_add(a943, a944)?
                    }, {
                        let (_, h945) = v1;
                        h945
                    })))
                } else {
                    Ok(Some(({
                        let a947 = {
                            let (h946, _) = v1;
                            h946
                        };
                        let a948 = 1u64;
                        nat_add(a947, a948)?
                    }, {
                        let a950 = {
                            let (_, h949) = v1;
                            h949
                        };
                        let a951 = 1u64;
                        nat_add(a950, a951)?
                    })))
                }
            }
        }
    }
}

pub fn f224(v0: Fn31, p956: u64, p957: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m955 = (p956, p957);
    {
        let mut m955 = m955;
        loop {
            let step = {
                let (v1, v2) = m955;
                let m952 = v1;
                if m952 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m952 - 1;
                    let m954 = {
                        let c953: Fn31 = v0.clone();
                        c953.apply(v2.clone())?
                    };
                    match m954 {
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
                Ok(next) => { m955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f233(v0, v1)
}

pub fn f226(v0: Adt14) -> R<Option<Adt14>> {
    let m959 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, h958) => {
            h958
        }
    };
    match m959 {
        None => {
            if if match v0.clone() {
                Adt14::C0(_, _, h960, _, _, _) => {
                    h960
                }
            } {
                true
            } else {
                let a963 = match v0.clone() {
                    Adt14::C0(_, h961, _, _, _, _) => {
                        h961
                    }
                };
                let a964 = match v0.clone() {
                    Adt14::C0(h962, _, _, _, _, _) => {
                        h962
                    }
                };
                nat_lt(a963, a964)
            } {
                Ok(Some(f234(v0.clone(), match v0.clone() {
                    Adt14::C0(h965, _, _, _, _, _) => {
                        h965
                    }
                }, match v0.clone() {
                    Adt14::C0(_, _, _, h966, _, _) => {
                        h966
                    }
                }, match v0 {
                    Adt14::C0(_, _, _, _, h967, _) => {
                        h967
                    }
                }, Some(false))))
            } else {
                let m969 = match v0.clone() {
                    Adt14::C0(_, _, _, h968, _, _) => {
                        h968
                    }
                };
                match m969.uncons() {
                    None => {
                        let m972 = match v0.clone() {
                            Adt14::C0(_, _, _, _, h971, _) => {
                                h971
                            }
                        };
                        match m972.uncons() {
                            None => {
                                Ok(Some(f234(v0.clone(), match v0 {
                                    Adt14::C0(h973, _, _, _, _, _) => {
                                        h973
                                    }
                                }, List::<Adt8>::nil(), List::<Adt8>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f235(v0, v4, List::<Adt8>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f235(v0.clone(), v2, v3, match v0 {
                            Adt14::C0(_, _, _, _, h970, _) => {
                                h970
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt14>)
        }
    }
}

pub fn f227(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m975 = {
        let (h974, _) = v0;
        h974
    };
    match m975 {
        None => {
            (Some(v1), {
                let (_, h976) = v0;
                h976
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a977 = v2;
                let a978 = v1;
                append_bytes(a977, a978)
            }, {
                let (_, h979) = v0;
                h979
            }))
        }
    }
}

pub fn f228(v0: u64) -> Bytes {
    f154(f236(v0, 16u64), f237(v0))
}

pub fn f229(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f238(v1, v0, v2)
}

pub fn f230(v0: u64) -> bool {
    if f160(v0) {
        true
    } else {
        let a980 = 47u64;
        let a981 = v0;
        if if nat_lt(a980, a981) {
            let a982 = v0;
            let a983 = 58u64;
            nat_lt(a982, a983)
        } else {
            false
        } {
            true
        } else {
            let a984 = v0;
            let a985 = 46u64;
            if nat_eq(a984, a985) {
                true
            } else {
                let a986 = v0;
                let a987 = 39u64;
                if nat_eq(a986, a987) {
                    true
                } else {
                    let a988 = v0;
                    let a989 = 47u64;
                    if nat_eq(a988, a989) {
                        true
                    } else {
                        let a990 = v0;
                        let a991 = 45u64;
                        nat_eq(a990, a991)
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Fn32, p996: u64, p997: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)> {
    let m995 = (p996, p997);
    {
        let mut m995 = m995;
        loop {
            let step = {
                let (v1, v2) = m995;
                let m992 = v1;
                if m992 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m992 - 1;
                    let m994 = {
                        let c993: Fn32 = v0.clone();
                        c993.apply(v2.clone())?
                    };
                    match m994 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)), R<((List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    f239(v0)
}

pub fn f233(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a999 = {
        let (h998, _) = v1.clone();
        h998
    };
    let a1000 = v0;
    if nat_lt(a999, a1000) {
        Ok(Some(({
            let a1002 = {
                let (h1001, _) = v1;
                h1001
            };
            let a1003 = 1u64;
            nat_add(a1002, a1003)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h1004) = v1;
            h1004
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f234(v0: Adt14, v1: u64, v2: List<Adt8>, v3: List<Adt8>, v4: Option<bool>) -> Adt14 {
    Adt14::C0(v1, match v0.clone() {
        Adt14::C0(_, h1005, _, _, _, _) => {
            h1005
        }
    }, match v0 {
        Adt14::C0(_, _, h1006, _, _, _) => {
            h1006
        }
    }, v2, v3, v4)
}

pub fn f235(v0: Adt14, v1: Adt8, v2: List<Adt8>, v3: List<Adt8>) -> R<Adt14> {
    let v4: u64 = match v1.clone() {
        Adt8::C0(h1007, _, _) => {
            h1007
        }
    };
    let v5: bool = match v1.clone() {
        Adt8::C0(_, h1008, _) => {
            h1008
        }
    };
    let v6: u64 = match v0.clone() {
        Adt14::C0(h1009, _, _, _, _, _) => {
            h1009
        }
    };
    let m1011 = match v1 {
        Adt8::C0(_, _, h1010) => {
            (*h1010).clone()
        }
    };
    match m1011 {
        Adt6::C0 => {
            Ok(f234(v0, v6, v2, v3, None::<bool>))
        }
        Adt6::C1(v7) => {
            Ok(f234(v0, {
                let a1012 = v6;
                let a1013 = f179(v7)?;
                nat_add(a1012, a1013)?
            }, v2, v3, None::<bool>))
        }
        Adt6::C2(r1018, r1019) => {
            let v8 = (*r1018).clone();
            let v9 = (*r1019).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0(v4, v5, std::rc::Rc::new(v8)), List::cons(Adt8::C0(v4, v5, std::rc::Rc::new(v9)), v2)), v3, None::<bool>))
        }
        Adt6::C3(v10, r1020) => {
            let v11 = (*r1020).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0({
                let a1014 = v4;
                let a1015 = v10;
                nat_add(a1014, a1015)?
            }, v5, std::rc::Rc::new(v11)), v2), v3, None::<bool>))
        }
        Adt6::C4 => {
            if v5 {
                Ok(f234(v0, {
                    let a1016 = v6;
                    let a1017 = 1u64;
                    nat_add(a1016, a1017)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f234(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt6::C5 => {
            if v5 {
                Ok(f234(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f234(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt6::C6(r1021) => {
            let v12 = (*r1021).clone();
            Ok(f234(v0, v6, List::cons(Adt8::C0(v4, true, std::rc::Rc::new(v12)), v2), v3, None::<bool>))
        }
    }
}

pub fn f236(v0: u64, v1: u64) -> Bytes {
    f237({
        let a1022 = v0;
        let a1023 = v1;
        let a1024 = 0u64;
        nat_quot(a1022, a1023, a1024)
    })
}

pub fn f237(v0: u64) -> Bytes {
    f85(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1025 = v0;
        let a1026 = 16u64;
        let a1027 = 0u64;
        nat_rem(a1025, a1026, a1027)
    }, 1u64)
}

pub fn f238(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1028 = v2;
    let a1029 = v1;
    if nat_lt(a1028, a1029) {
        let v3: u64 = f216(v0, v2)?;
        let a1030 = v3;
        let a1031 = 32u64;
        if nat_lt(a1030, a1031) {
            Ok(None::<u64>)
        } else {
            let a1032 = v3;
            let a1033 = 34u64;
            if nat_eq(a1032, a1033) {
                Ok(None::<u64>)
            } else {
                let a1034 = v3;
                let a1035 = 92u64;
                if nat_eq(a1034, a1035) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1036 = v2;
                        let a1037 = 1u64;
                        nat_add(a1036, a1037)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f239(v0: (List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)) -> R<Option<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>> {
    let v1: List<List<(Bytes, Adt6)>> = {
        let (h1038, _) = v0;
        h1038
    };
    let v2: List<List<(Bytes, Adt6)>> = {
        let (_, h1039) = v0;
        h1039
    };
    let m1040 = v1;
    match m1040.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt6)>>, List<List<(Bytes, Adt6)>>)>)
        }
        Some((v3, v4)) => {
            let m1041 = v4;
            match m1041.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt6)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f240(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, Adt6)>, v1: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    let v2: u64 = {
        let a1046 = {
            let a1044 = {
                let a1042 = v0.clone();
                length_list(a1042)
            };
            let a1045 = {
                let a1043 = v1.clone();
                length_list(a1043)
            };
            nat_add(a1044, a1045)?
        };
        let a1047 = 1u64;
        nat_add(a1046, a1047)?
    };
    f241({
        let (_, h1048) = {
            let (_, h1049) = {
                let (h1050, _) = f242(Fn33::F243, v2, (v0, (v1, List::<(Bytes, Adt6)>::nil())));
                h1050
            };
            h1049
        };
        h1048
    })
}

pub fn f241(v0: List<(Bytes, Adt6)>) -> R<List<(Bytes, Adt6)>> {
    f122(Fn13::F244, List::<(Bytes, Adt6)>::nil(), v0)
}

pub fn f242(v0: Fn33, p1055: u64, p1056: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool) {
    let m1054 = (p1055, p1056);
    {
        let mut m1054 = m1054;
        loop {
            let step = {
                let (v1, v2) = m1054;
                let m1051 = v1;
                if m1051 == 0 {
                    Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, false))
                } else {
                    let v3 = m1051 - 1;
                    let m1053 = {
                        let c1052: Fn33 = v0.clone();
                        c1052.apply(v2.clone())
                    };
                    match m1053 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))), ((List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    f245(v0)
}

pub fn f244(v0: List<(Bytes, Adt6)>, v1: (Bytes, Adt6)) -> List<(Bytes, Adt6)> {
    List::cons(v1, v0)
}

pub fn f245(v0: (List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))) -> Option<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))> {
    let v1: List<(Bytes, Adt6)> = {
        let (h1057, _) = v0.clone();
        h1057
    };
    let v2: List<(Bytes, Adt6)> = {
        let (h1058, _) = {
            let (_, h1059) = v0.clone();
            h1059
        };
        h1058
    };
    let v3: List<(Bytes, Adt6)> = {
        let (_, h1060) = {
            let (_, h1061) = v0;
            h1061
        };
        h1060
    };
    let m1062 = v1.clone();
    match m1062.uncons() {
        None => {
            let m1063 = v2;
            match m1063.uncons() {
                None => {
                    None::<(List<(Bytes, Adt6)>, (List<(Bytes, Adt6)>, List<(Bytes, Adt6)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt6)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1064 = v2.clone();
            match m1064.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt6)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f246({
                        let (h1065, _) = v8.clone();
                        h1065
                    }, {
                        let (h1066, _) = v6.clone();
                        h1066
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

pub fn f246(v0: Bytes, v1: Bytes) -> bool {
    let a1071 = {
        let a1067 = v0;
        let a1068 = v1;
        compare_bytes(a1067, a1068)
    };
    let a1072 = {
        let a1069 = Bytes::lit(&[]);
        let a1070 = Bytes::lit(&[0x01]);
        compare_bytes(a1069, a1070)
    };
    equal(a1071, a1072)
}

pub fn run(p0: Adt0, p1: Adt0) -> R<Result<Bytes, Adt1>> {
    f0(p0, p1)
}
