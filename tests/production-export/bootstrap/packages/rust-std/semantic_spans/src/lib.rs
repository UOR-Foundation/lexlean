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
    C0(u64, List<Adt3>, std::rc::Rc<Adt0>, u64, u64, std::rc::Rc<Adt4>, u64, Option<Adt1>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, Bytes, std::rc::Rc<Adt4>, Option<Adt0>, List<Adt0>, List<(Bytes, Adt0)>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, List<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
    C4(std::rc::Rc<Adt0>),
    C5,
}

#[derive(Clone)]
pub enum Adt5 {
    C0(std::rc::Rc<Adt6>, u64, u64),
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
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Adt5>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(std::rc::Rc<Adt0>, Option<Adt0>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, List<Bytes>, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(std::rc::Rc<Adt0>, u64, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, u64, List<Adt14>, std::rc::Rc<Adt0>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Adt0>),
    C1(Bytes, u64, List<(Bytes, Adt0)>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Adt0>, u64, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(std::rc::Rc<Adt0>, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn0::F6(k0, k1) => f6(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(u64, u64)>, p1: (u64, u64)) -> List<(u64, u64)> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F27(std::rc::Rc<Adt0>, u64, List<Adt5>),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn3::F27(k0, k1, k2) => f27((**k0).clone(), *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F37(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt8) -> Option<Adt8> {
        match self {
            Fn4::F37(k0) => f37(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F41,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn5::F41 => f41(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F54(std::rc::Rc<Adt0>),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn6::F54(k0) => f54((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F62(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Adt9) -> R<Option<Adt9>> {
        match self {
            Fn7::F62(k0, k1) => f62(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F66(u64, Bytes),
    F186(u64, Bytes),
    F192(Bytes),
    F193(Bytes),
    F206(Bytes),
    F276(Bytes),
    F286(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F66(k0, k1) => f66(*k0, k1.clone(), p0),
            Fn8::F186(k0, k1) => f186(*k0, k1.clone(), p0),
            Fn8::F192(k0) => f192(k0.clone(), p0),
            Fn8::F193(k0) => f193(k0.clone(), p0),
            Fn8::F206(k0) => f206(k0.clone(), p0),
            Fn8::F276(k0) => f276(k0.clone(), p0),
            Fn8::F286(k0) => f286(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F71(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn9::F71(k0) => f71(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F86,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn10::F86 => f86(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F88,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn11::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F160(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn12::F160(k0) => f160(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F172(u64, List<Adt5>),
}

impl Fn13 {
    pub fn apply(&self, p0: Adt13) -> R<Option<Adt13>> {
        match self {
            Fn13::F172(k0, k1) => f172(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F174(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn14::F174(k0) => f174(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F181(u64, List<Adt5>),
    F231(u64, List<Adt5>, Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt15) -> R<Option<Adt15>> {
        match self {
            Fn15::F181(k0, k1) => f181(*k0, k1.clone(), p0),
            Fn15::F231(k0, k1, k2) => f231(*k0, k1.clone(), k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F188,
}

impl Fn16 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn16::F188 => f188(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F196(Bytes, std::rc::Rc<Adt0>),
}

impl Fn17 {
    pub fn apply(&self, p0: Option<Adt0>, p1: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn17::F196(k0, k1) => f196(k0.clone(), (**k1).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F199(std::rc::Rc<Adt0>),
    F243(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> R<bool> {
        match self {
            Fn18::F199(k0) => f199((**k0).clone(), p0, p1),
            Fn18::F243(k0) => Ok(f243(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F209,
}

impl Fn19 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn19::F209 => f209(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F213,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn20::F213 => f213(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F215,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn21::F215 => f215(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F227,
}

impl Fn22 {
    pub fn apply(&self, p0: Adt16) -> Option<Adt16> {
        match self {
            Fn22::F227 => f227(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F230(u64, Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Adt17) -> R<Option<Adt17>> {
        match self {
            Fn23::F230(k0, k1) => f230(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F235,
}

impl Fn24 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn24::F235 => f235(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F237,
}

impl Fn25 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn25::F237 => f237(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F245,
}

impl Fn26 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn26::F245 => f245(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F266,
}

impl Fn27 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn27::F266 => f266(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F279(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn28::F279(k0) => f279(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F281,
}

impl Fn29 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn29::F281 => f281(p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: Bytes) -> R<Result<List<(u64, u64)>, Adt1>> {
    let m1 = f1(v1.clone())?;
    match m1 {
        Ok(v2) => {
            let v3: Adt2 = f2(v0, v2, {
                let a2 = v1;
                length_bytes(a2)
            })?;
            let m3 = f3(v3.clone());
            match m3 {
                Ok(_) => {
                    Ok(Ok::<List<(u64, u64)>, Adt1>(f4(match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4) => {
                            h4
                        }
                    })))
                }
                Err(v5) => {
                    Ok(Err::<List<(u64, u64)>, Adt1>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<(u64, u64)>, Adt1>(v6))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt5>, Adt1>> {
    let v1: u64 = {
        let a5 = v0.clone();
        length_bytes(a5)
    };
    let v2: Adt7 = {
        let (h6, _) = f5(Fn0::F6(v1, v0), {
            let a7 = v1;
            let a8 = 1u64;
            nat_add(a7, a8)?
        }, Adt7::C0(0u64, List::<Adt5>::nil(), None::<Adt1>))?;
        h6
    };
    let m10 = match v2.clone() {
        Adt7::C0(_, _, h9) => {
            h9
        }
    };
    match m10 {
        None => {
            Ok(Ok::<List<Adt5>, Adt1>(f7(match v2 {
                Adt7::C0(_, h11, _) => {
                    h11
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt5>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: Adt0, v1: List<Adt5>, v2: u64) -> R<Adt2> {
    let v3: Adt4 = f8(v0.clone(), 0u64)?;
    let a13 = {
        let a12 = v1.clone();
        length_list(a12)
    };
    let a14 = 0u64;
    if nat_eq(a13, a14) {
        Ok(Adt2::C0(0u64, List::<Adt3>::nil(), std::rc::Rc::new(f9(List::<Adt0>::nil())), 9u64, 0u64, std::rc::Rc::new(v3), 1u64, None::<Adt1>, List::<(u64, u64)>::nil()))
    } else {
        f10(v1.clone(), v2, v0, f11(f12(f13(f14(0u64, 1u64), v3.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), 1u64), f15(Str::lit("")), f16(v1, v2, 0u64)), v3)
    }
}

pub fn f3(v0: Adt2) -> Result<Adt0, Adt1> {
    let m16 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h15, _) => {
            h15
        }
    };
    match m16 {
        None => {
            let a18 = match v0.clone() {
                Adt2::C0(_, _, _, h17, _, _, _, _, _) => {
                    h17
                }
            };
            let a19 = 9u64;
            if nat_eq(a18, a19) {
                Ok::<Adt0, Adt1>(match v0 {
                    Adt2::C0(_, _, h20, _, _, _, _, _, _) => {
                        (*h20).clone()
                    }
                })
            } else {
                Err::<Adt0, Adt1>(Adt1::C0(0u64, f15(Str::lit("the reader ran out of steps"))))
            }
        }
        Some(v1) => {
            Err::<Adt0, Adt1>(v1)
        }
    }
}

pub fn f4(v0: List<(u64, u64)>) -> List<(u64, u64)> {
    f17(Fn1::F18, List::<(u64, u64)>::nil(), v0)
}

pub fn f5(v0: Fn0, p25: u64, p26: Adt7) -> R<(Adt7, bool)> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m21 = v1;
                if m21 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m21 - 1;
                    let m23 = {
                        let c22: Fn0 = v0.clone();
                        c22.apply(v2.clone())?
                    };
                    match m23 {
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
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f19(v1, v0, v2)
}

pub fn f7(v0: List<Adt5>) -> List<Adt5> {
    f20(Fn2::F21, List::<Adt5>::nil(), v0)
}

pub fn f8(v0: Adt0, v1: u64) -> R<Adt4> {
    let a27 = v1;
    let a28 = 0u64;
    if nat_eq(a27, a28) {
        let m29 = f22(v0.clone(), f15(Str::lit("properties")));
        match m29 {
            None => {
                Ok(Adt4::C5)
            }
            Some(v2) => {
                let m30 = f22(v2, f15(Str::lit("declarations")));
                match m30 {
                    None => {
                        Ok(Adt4::C5)
                    }
                    Some(v3) => {
                        f23(v0, v3)
                    }
                }
            }
        }
    } else {
        let a31 = v1;
        let a32 = 1u64;
        if nat_eq(a31, a32) {
            Ok(Adt4::C0)
        } else {
            let a33 = v1;
            let a34 = 2u64;
            if nat_eq(a33, a34) {
                Ok(Adt4::C1)
            } else {
                let m35 = f24(v0, f15(Str::lit("proof")));
                match m35 {
                    None => {
                        Ok(Adt4::C5)
                    }
                    Some(v4) => {
                        Ok(Adt4::C4(std::rc::Rc::new(v4)))
                    }
                }
            }
        }
    }
}

pub fn f9(v0: List<Adt0>) -> Adt0 {
    Adt0::C3(std::rc::Rc::new(f25(v0)))
}

pub fn f10(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt3, v4: Adt4) -> R<Adt2> {
    let v5: Adt2 = Adt2::C0(0u64, List::cons(v3, List::<Adt3>::nil()), std::rc::Rc::new(Adt0::C5), 0u64, 0u64, std::rc::Rc::new(v4), 1u64, None::<Adt1>, List::<(u64, u64)>::nil());
    let (h36, _) = f26(Fn3::F27(std::rc::Rc::new(v2), v1, v0.clone()), {
        let a40 = {
            let a38 = {
                let a37 = v0;
                length_list(a37)
            };
            let a39 = 16u64;
            nat_mul(a38, a39)?
        };
        let a41 = 64u64;
        nat_add(a40, a41)?
    }, v5)?;
    Ok(h36)
}

pub fn f11(v0: Adt3, v1: Bytes, v2: u64) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h42, _, _, _, _, _, _, _, _, _, _, _) => {
            h42
        }
    }, match v0.clone() {
        Adt3::C0(_, h43, _, _, _, _, _, _, _, _, _, _) => {
            h43
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h44, _, _, _, _, _, _, _, _, _) => {
            h44
        }
    }, v2, v1, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h45, _, _, _, _, _, _) => {
            (*h45).clone()
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h46, _, _, _, _, _) => {
            h46
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h47, _, _, _, _) => {
            h47
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h48, _, _, _) => {
            h48
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h49, _, _) => {
            (*h49).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h50, _) => {
            (*h50).clone()
        }
    }), match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h51) => {
            h51
        }
    })
}

pub fn f12(v0: Adt3, v1: u64) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h52, _, _, _, _, _, _, _, _, _, _, _) => {
            h52
        }
    }, v1, match v0.clone() {
        Adt3::C0(_, _, h53, _, _, _, _, _, _, _, _, _) => {
            h53
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h54, _, _, _, _, _, _, _, _) => {
            h54
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h55, _, _, _, _, _, _, _) => {
            h55
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h56, _, _, _, _, _, _) => {
            (*h56).clone()
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h57, _, _, _, _, _) => {
            h57
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h58, _, _, _, _) => {
            h58
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h59, _, _, _) => {
            h59
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h60, _, _) => {
            (*h60).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h61, _) => {
            (*h61).clone()
        }
    }), match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h62) => {
            h62
        }
    })
}

pub fn f13(v0: Adt3, v1: Adt4, v2: Option<Adt0>, v3: List<(Bytes, Adt0)>) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h63, _, _, _, _, _, _, _, _, _, _, _) => {
            h63
        }
    }, match v0.clone() {
        Adt3::C0(_, h64, _, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h65, _, _, _, _, _, _, _, _, _) => {
            h65
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h66, _, _, _, _, _, _, _, _) => {
            h66
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h67, _, _, _, _, _, _, _) => {
            h67
        }
    }, std::rc::Rc::new(v1), v2, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h68, _, _, _, _) => {
            h68
        }
    }, v3, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h69, _, _) => {
            (*h69).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h70, _) => {
            (*h70).clone()
        }
    }), match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h71) => {
            h71
        }
    })
}

pub fn f14(v0: u64, v1: u64) -> Adt3 {
    Adt3::C0(v0, 0u64, v1, 0u64, f15(Str::lit("")), std::rc::Rc::new(Adt4::C5), None::<Adt0>, List::<Adt0>::nil(), List::<(Bytes, Adt0)>::nil(), std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C5), List::<(u64, u64)>::nil())
}

pub fn f15(v0: Str) -> Bytes {
    let a72 = v0;
    utf8_encode(a72)
}

pub fn f16(v0: List<Adt5>, v1: u64, v2: u64) -> u64 {
    let m73 = f28(v0, v2);
    match m73 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt5::C0(_, h74, _) => {
                    h74
                }
            }
        }
    }
}

pub fn f17(v0: Fn1, p78: List<(u64, u64)>, p79: List<(u64, u64)>) -> List<(u64, u64)> {
    let m77 = (p78, p79);
    {
        let mut m77 = m77;
        loop {
            let step = {
                let (v1, v2) = m77;
                let m75 = v2;
                match m75.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(({
                            let c76: Fn1 = v0.clone();
                            c76.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m77 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    List::cons(v1, v0)
}

pub fn f19(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let m81 = match v2.clone() {
        Adt7::C0(_, _, h80) => {
            h80
        }
    };
    match m81 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt7::C0(h82, _, _) => {
                    h82
                }
            };
            let a83 = v4;
            let a84 = v1;
            if nat_lt(a83, a84) {
                let v5: u64 = f29(v0.clone(), v4)?;
                let a85 = 0u64;
                let a86 = v5;
                if nat_lt(a85, a86) {
                    Ok(Some(Adt7::C0({
                        let a87 = v4;
                        let a88 = v5;
                        nat_add(a87, a88)?
                    }, match v2 {
                        Adt7::C0(_, h89, _) => {
                            h89
                        }
                    }, None::<Adt1>)))
                } else {
                    if f30(v0.clone(), v4, 123u8) {
                        Ok(f31(v2, Adt6::C0, {
                            let a90 = v4;
                            let a91 = 1u64;
                            nat_add(a90, a91)?
                        }))
                    } else {
                        if f30(v0.clone(), v4, 125u8) {
                            Ok(f31(v2, Adt6::C1, {
                                let a92 = v4;
                                let a93 = 1u64;
                                nat_add(a92, a93)?
                            }))
                        } else {
                            if f30(v0.clone(), v4, 91u8) {
                                Ok(f31(v2, Adt6::C2, {
                                    let a94 = v4;
                                    let a95 = 1u64;
                                    nat_add(a94, a95)?
                                }))
                            } else {
                                if f30(v0.clone(), v4, 93u8) {
                                    Ok(f31(v2, Adt6::C3, {
                                        let a96 = v4;
                                        let a97 = 1u64;
                                        nat_add(a96, a97)?
                                    }))
                                } else {
                                    if f30(v0.clone(), v4, 40u8) {
                                        Ok(f31(v2, Adt6::C4, {
                                            let a98 = v4;
                                            let a99 = 1u64;
                                            nat_add(a98, a99)?
                                        }))
                                    } else {
                                        if f30(v0.clone(), v4, 41u8) {
                                            Ok(f31(v2, Adt6::C5, {
                                                let a100 = v4;
                                                let a101 = 1u64;
                                                nat_add(a100, a101)?
                                            }))
                                        } else {
                                            if f30(v0.clone(), v4, 44u8) {
                                                Ok(f31(v2, Adt6::C6, {
                                                    let a102 = v4;
                                                    let a103 = 1u64;
                                                    nat_add(a102, a103)?
                                                }))
                                            } else {
                                                if f30(v0.clone(), v4, 59u8) {
                                                    Ok(f31(v2, Adt6::C10, {
                                                        let a104 = v4;
                                                        let a105 = 1u64;
                                                        nat_add(a104, a105)?
                                                    }))
                                                } else {
                                                    if f30(v0.clone(), v4, 58u8) {
                                                        f32(v0, v2)
                                                    } else {
                                                        if f30(v0.clone(), v4, 34u8) {
                                                            f33(v0, v1, v2)
                                                        } else {
                                                            f34(v0, v1, v2)
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
                Ok(None::<Adt7>)
            }
        }
        Some(_) => {
            Ok(None::<Adt7>)
        }
    }
}

pub fn f20(v0: Fn2, p109: List<Adt5>, p110: List<Adt5>) -> List<Adt5> {
    let m108 = (p109, p110);
    {
        let mut m108 = m108;
        loop {
            let step = {
                let (v1, v2) = m108;
                let m106 = v2;
                match m106.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c107: Fn2 = v0.clone();
                            c107.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f22(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m111 = f35(v0);
    match m111 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            let m114 = {
                let (h113, _) = f36(Fn4::F37(v1), 1000000u64, Adt8::C0(std::rc::Rc::new(v2), None::<Adt0>));
                h113
            };
            match m114 {
                Adt8::C0(_, h112) => {
                    h112
                }
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Adt0) -> R<Adt4> {
    let m115 = f22(f38(v0.clone(), v1)?, f15(Str::lit("items")));
    match m115 {
        None => {
            Ok(Adt4::C5)
        }
        Some(v2) => {
            f39(v0, v2)
        }
    }
}

pub fn f24(v0: Adt0, v1: Bytes) -> Option<Adt0> {
    let m116 = f22(v0, f15(Str::lit("$defs")));
    match m116 {
        None => {
            None::<Adt0>
        }
        Some(v2) => {
            f22(v2, v1)
        }
    }
}

pub fn f25(v0: List<Adt0>) -> Adt0 {
    f40(Fn5::F41, Adt0::C5, v0)
}

pub fn f26(v0: Fn3, p121: u64, p122: Adt2) -> R<(Adt2, bool)> {
    let m120 = (p121, p122);
    {
        let mut m120 = m120;
        loop {
            let step = {
                let (v1, v2) = m120;
                let m117 = v1;
                if m117 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m117 - 1;
                    let m119 = {
                        let c118: Fn3 = v0.clone();
                        c118.apply(v2.clone())?
                    };
                    match m119 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt0, v1: u64, v2: List<Adt5>, v3: Adt2) -> R<Option<Adt2>> {
    f42(v2, v1, v0, v3)
}

pub fn f28(v0: List<Adt5>, v1: u64) -> Option<Adt5> {
    let a123 = v0;
    let a124 = v1;
    index_list(a123, a124)
}

pub fn f29(v0: Bytes, v1: u64) -> R<u64> {
    if if f43(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f44(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f44(v0.clone(), v1, 194u8) {
            if if f44(v0.clone(), {
                let a125 = v1;
                let a126 = 1u64;
                nat_add(a125, a126)?
            }, 133u8) {
                true
            } else {
                f44(v0, {
                    let a127 = v1;
                    let a128 = 1u64;
                    nat_add(a127, a128)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f44(v0.clone(), v1, 225u8) {
                if if f44(v0.clone(), {
                    let a129 = v1;
                    let a130 = 1u64;
                    nat_add(a129, a130)?
                }, 154u8) {
                    f44(v0, {
                        let a131 = v1;
                        let a132 = 2u64;
                        nat_add(a131, a132)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f44(v0.clone(), v1, 226u8) {
                    if f44(v0.clone(), {
                        let a133 = v1;
                        let a134 = 1u64;
                        nat_add(a133, a134)?
                    }, 128u8) {
                        if if f43(v0.clone(), {
                            let a135 = v1;
                            let a136 = 2u64;
                            nat_add(a135, a136)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f44(v0.clone(), {
                                let a137 = v1;
                                let a138 = 2u64;
                                nat_add(a137, a138)?
                            }, 168u8) {
                                true
                            } else {
                                if f44(v0.clone(), {
                                    let a139 = v1;
                                    let a140 = 2u64;
                                    nat_add(a139, a140)?
                                }, 169u8) {
                                    true
                                } else {
                                    f44(v0, {
                                        let a141 = v1;
                                        let a142 = 2u64;
                                        nat_add(a141, a142)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f44(v0.clone(), {
                            let a143 = v1;
                            let a144 = 1u64;
                            nat_add(a143, a144)?
                        }, 129u8) {
                            f44(v0, {
                                let a145 = v1;
                                let a146 = 2u64;
                                nat_add(a145, a146)?
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
                    if f44(v0.clone(), v1, 227u8) {
                        if if f44(v0.clone(), {
                            let a147 = v1;
                            let a148 = 1u64;
                            nat_add(a147, a148)?
                        }, 128u8) {
                            f44(v0, {
                                let a149 = v1;
                                let a150 = 2u64;
                                nat_add(a149, a150)?
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

pub fn f30(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m151 = f45(v0, v1);
    match m151 {
        None => {
            false
        }
        Some(v3) => {
            let a152 = v3;
            let a153 = v2;
            equal(a152, a153)
        }
    }
}

pub fn f31(v0: Adt7, v1: Adt6, v2: u64) -> Option<Adt7> {
    Some(Adt7::C0(v2, List::cons(Adt5::C0(std::rc::Rc::new(v1), match v0.clone() {
        Adt7::C0(h154, _, _) => {
            h154
        }
    }, v2), match v0 {
        Adt7::C0(_, h155, _) => {
            h155
        }
    }), None::<Adt1>))
}

pub fn f32(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    let v2: u64 = match v1.clone() {
        Adt7::C0(h156, _, _) => {
            h156
        }
    };
    if f30(v0.clone(), {
        let a157 = v2;
        let a158 = 1u64;
        nat_add(a157, a158)?
    }, 61u8) {
        Ok(f31(v1, Adt6::C8, {
            let a159 = v2;
            let a160 = 2u64;
            nat_add(a159, a160)?
        }))
    } else {
        if f30(v0, {
            let a161 = v2;
            let a162 = 1u64;
            nat_add(a161, a162)?
        }, 58u8) {
            Ok(f31(v1, Adt6::C9, {
                let a163 = v2;
                let a164 = 2u64;
                nat_add(a163, a164)?
            }))
        } else {
            Ok(f31(v1, Adt6::C7, {
                let a165 = v2;
                let a166 = 1u64;
                nat_add(a165, a166)?
            }))
        }
    }
}

pub fn f33(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h167, _, _) => {
            h167
        }
    };
    let v4: Adt9 = f46(v0, v1, v3)?;
    let m169 = match v4.clone() {
        Adt9::C0(_, _, _, h168) => {
            h168
        }
    };
    match m169 {
        None => {
            if match v4.clone() {
                Adt9::C0(_, _, h171, _) => {
                    h171
                }
            } {
                Ok(f31(v2, Adt6::C11(f47(match v4.clone() {
                    Adt9::C0(_, h172, _, _) => {
                        h172
                    }
                })), match v4 {
                    Adt9::C0(h173, _, _, _) => {
                        h173
                    }
                }))
            } else {
                Ok(Some(Adt7::C0(v3, match v2 {
                    Adt7::C0(_, h174, _) => {
                        h174
                    }
                }, Some(f48(v3, f49(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt7::C0(v3, match v2 {
                Adt7::C0(_, h170, _) => {
                    h170
                }
            }, Some(v5))))
        }
    }
}

pub fn f34(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h175, _, _) => {
            h175
        }
    };
    let v4: u64 = f50(v0.clone(), v1, v3)?;
    Ok(f31(v2, Adt6::C12(f51(v0, v3, {
        let a176 = v4;
        let a177 = v3;
        nat_sub(a176, a177)
    })), v4))
}

pub fn f35(v0: Adt0) -> Option<Adt0> {
    let m178 = v0;
    match m178 {
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
        Adt0::C4(r179) => {
            let v1 = (*r179).clone();
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

pub fn f36(v0: Fn4, p184: u64, p185: Adt8) -> (Adt8, bool) {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v1, v2) = m183;
                let m180 = v1;
                if m180 == 0 {
                    Err::<(u64, Adt8), (Adt8, bool)>((v2, false))
                } else {
                    let v3 = m180 - 1;
                    let m182 = {
                        let c181: Fn4 = v0.clone();
                        c181.apply(v2.clone())
                    };
                    match m182 {
                        None => {
                            Err::<(u64, Adt8), (Adt8, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), (Adt8, bool)>((v3, v4))
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

pub fn f37(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    f52(v0, v1)
}

pub fn f38(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let (h186, _) = f53(Fn6::F54(std::rc::Rc::new(v0)), 4096u64, v1)?;
    Ok(h186)
}

pub fn f39(v0: Adt0, v1: Adt0) -> R<Adt4> {
    let m187 = f55(f22(v1.clone(), f15(Str::lit("$ref"))));
    match m187 {
        None => {
            Ok(Adt4::C4(std::rc::Rc::new(v1)))
        }
        Some(v2) => {
            let v3: Bytes = f56(v2)?;
            if f57(v3.clone(), f15(Str::lit("term"))) {
                Ok(Adt4::C0)
            } else {
                if f57(v3.clone(), f15(Str::lit("type"))) {
                    Ok(Adt4::C1)
                } else {
                    if f57(v3.clone(), f15(Str::lit("member"))) {
                        Ok(Adt4::C2)
                    } else {
                        if f57(v3, f15(Str::lit("parameter"))) {
                            Ok(Adt4::C3)
                        } else {
                            Ok(Adt4::C4(std::rc::Rc::new(f38(v0, v1)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f40(v0: Fn5, p191: Adt0, p192: List<Adt0>) -> Adt0 {
    let m190 = (p191, p192);
    {
        let mut m190 = m190;
        loop {
            let step = {
                let (v1, v2) = m190;
                let m188 = v2;
                match m188.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c189: Fn5 = v0.clone();
                            c189.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f42(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2) -> R<Option<Adt2>> {
    let m194 = match v3.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h193, _) => {
            h193
        }
    };
    match m194 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt2::C0(_, _, _, h195, _, _, _, _, _) => {
                    h195
                }
            };
            let a196 = v5;
            let a197 = 9u64;
            if nat_eq(a196, a197) {
                Ok(None::<Adt2>)
            } else {
                Ok(Some({
                    let a198 = v5;
                    let a199 = 0u64;
                    if nat_eq(a198, a199) {
                        f58(v0, v1, v2, v3)?
                    } else {
                        f59(v0, v1, v2, v3)?
                    }
                }))
            }
        }
        Some(_) => {
            Ok(None::<Adt2>)
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m200 = f45(v0, v1);
    match m200 {
        None => {
            false
        }
        Some(v4) => {
            f60(v4, v2, v3)
        }
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m201 = f45(v0, v1);
    match m201 {
        None => {
            false
        }
        Some(v3) => {
            let a202 = v3;
            let a203 = v2;
            equal(a202, a203)
        }
    }
}

pub fn f45(v0: Bytes, v1: u64) -> Option<u8> {
    let a204 = v0;
    let a205 = v1;
    index_bytes(a204, a205)
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<Adt9> {
    let (h206, _) = f61(Fn7::F62(v1, v0), {
        let a209 = {
            let a207 = v1;
            let a208 = v2;
            nat_sub(a207, a208)
        };
        let a210 = 1u64;
        nat_add(a209, a210)?
    }, Adt9::C0({
        let a211 = v2;
        let a212 = 1u64;
        nat_add(a211, a212)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h206)
}

pub fn f47(v0: List<Bytes>) -> Bytes {
    f63(f64(v0))
}

pub fn f48(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f49(v0: Str) -> Bytes {
    let a213 = v0;
    utf8_encode(a213)
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h214, _) = f65(Fn8::F66(v1, v0), {
        let a215 = v1;
        let a216 = v2;
        nat_sub(a215, a216)
    }, v2)?;
    Ok(h214)
}

pub fn f51(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m220 = {
        let a217 = v0;
        let a218 = v1;
        let a219 = v2;
        slice_bytes(a217, a218, a219)
    };
    match m220 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f52(v0: Bytes, v1: Adt8) -> Option<Adt8> {
    let m222 = match v1.clone() {
        Adt8::C0(_, h221) => {
            h221
        }
    };
    match m222 {
        None => {
            let m224 = f67(match v1 {
                Adt8::C0(h223, _) => {
                    (*h223).clone()
                }
            });
            match m224 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    if f57(match v3.clone() {
                        Adt11::C0(h225, _, _) => {
                            h225
                        }
                    }, v0) {
                        Some(Adt8::C0(std::rc::Rc::new(Adt0::C5), Some(match v3 {
                            Adt11::C0(_, h226, _) => {
                                (*h226).clone()
                            }
                        })))
                    } else {
                        Some(Adt8::C0(std::rc::Rc::new(match v3 {
                            Adt11::C0(_, _, h227) => {
                                (*h227).clone()
                            }
                        }), None::<Adt0>))
                    }
                }
            }
        }
        Some(_) => {
            None::<Adt8>
        }
    }
}

pub fn f53(v0: Fn6, p232: u64, p233: Adt0) -> R<(Adt0, bool)> {
    let m231 = (p232, p233);
    {
        let mut m231 = m231;
        loop {
            let step = {
                let (v1, v2) = m231;
                let m228 = v1;
                if m228 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m228 - 1;
                    let m230 = {
                        let c229: Fn6 = v0.clone();
                        c229.apply(v2.clone())?
                    };
                    match m230 {
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
                Ok(next) => { m231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    f68(v0, v1)
}

pub fn f55(v0: Option<Adt0>) -> Option<Bytes> {
    let m234 = v0;
    match m234 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f69(v1)
        }
    }
}

pub fn f56(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let m240 = {
            let (h236, _) = f70(Fn9::F71(v0.clone()), {
                let a238 = {
                    let a237 = v0.clone();
                    length_bytes(a237)
                };
                let a239 = 1u64;
                nat_add(a238, a239)?
            }, Adt10::C0(0u64, 0u64))?;
            h236
        };
        match m240 {
            Adt10::C0(_, h235) => {
                h235
            }
        }
    };
    Ok(f51(v0.clone(), v1, {
        let a242 = {
            let a241 = v0;
            length_bytes(a241)
        };
        let a243 = v1;
        nat_sub(a242, a243)
    }))
}

pub fn f57(v0: Bytes, v1: Bytes) -> bool {
    let a244 = v0;
    let a245 = v1;
    equal(a244, a245)
}

pub fn f58(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2) -> R<Adt2> {
    let v4: u64 = match v3.clone() {
        Adt2::C0(_, _, _, _, h246, _, _, _, _) => {
            h246
        }
    };
    let a247 = v4;
    let a248 = 0u64;
    if nat_eq(a247, a248) {
        f72(v0, v1, v2, v3)
    } else {
        let a249 = v4;
        let a250 = 1u64;
        if nat_eq(a249, a250) {
            f73(v0, v1, v2, v3)
        } else {
            let a251 = v4;
            let a252 = 2u64;
            if nat_eq(a251, a252) {
                f74(v0, v1, v3)
            } else {
                let a253 = v4;
                let a254 = 3u64;
                if nat_eq(a253, a254) {
                    f75(v0, v1, v3)
                } else {
                    let a255 = v4;
                    let a256 = 4u64;
                    if nat_eq(a255, a256) {
                        f76(v0, v1, v3)
                    } else {
                        let a257 = v4;
                        let a258 = 5u64;
                        if nat_eq(a257, a258) {
                            f77(v0, v1, v3.clone(), match v3 {
                                Adt2::C0(_, _, _, _, _, _, h259, _, _) => {
                                    h259
                                }
                            })
                        } else {
                            f78(v0, v1, v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f59(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2) -> R<Adt2> {
    let m261 = match v3.clone() {
        Adt2::C0(_, h260, _, _, _, _, _, _, _) => {
            h260
        }
    };
    match m261.uncons() {
        None => {
            Ok(f79(v3.clone(), match v3 {
                Adt2::C0(_, _, h262, _, _, _, _, _, _) => {
                    (*h262).clone()
                }
            }, List::<(u64, u64)>::nil()))
        }
        Some((v4, v5)) => {
            let v6: Adt2 = Adt2::C0(match v3.clone() {
                Adt2::C0(h263, _, _, _, _, _, _, _, _) => {
                    h263
                }
            }, v5, std::rc::Rc::new(match v3.clone() {
                Adt2::C0(_, _, h264, _, _, _, _, _, _) => {
                    (*h264).clone()
                }
            }), 1u64, match v3.clone() {
                Adt2::C0(_, _, _, _, h265, _, _, _, _) => {
                    h265
                }
            }, std::rc::Rc::new(match v3.clone() {
                Adt2::C0(_, _, _, _, _, h266, _, _, _) => {
                    (*h266).clone()
                }
            }), match v3.clone() {
                Adt2::C0(_, _, _, _, _, _, h267, _, _) => {
                    h267
                }
            }, None::<Adt1>, match v3 {
                Adt2::C0(_, _, _, _, _, _, _, _, h268) => {
                    h268
                }
            });
            let v7: u64 = match v4.clone() {
                Adt3::C0(h269, _, _, _, _, _, _, _, _, _, _, _) => {
                    h269
                }
            };
            let a270 = v7;
            let a271 = 0u64;
            if nat_eq(a270, a271) {
                Ok(f80(v0, v1, v2, v6, v4))
            } else {
                let a272 = v7;
                let a273 = 10u64;
                if nat_lt(a272, a273) {
                    f81(v0, v1, v2, v6, v4)
                } else {
                    let a274 = v7;
                    let a275 = 30u64;
                    if nat_lt(a274, a275) {
                        f82(v0, v1, v6, v4)
                    } else {
                        f83(v0, v1, v6, v4)
                    }
                }
            }
        }
    }
}

pub fn f60(v0: u8, v1: u8, v2: u8) -> bool {
    let m278 = {
        let a276 = v0;
        let a277 = v1;
        fixed_u8::checked_sub(a276, a277)
    };
    match m278 {
        None => {
            false
        }
        Some(_) => {
            let m281 = {
                let a279 = v2;
                let a280 = v0;
                fixed_u8::checked_sub(a279, a280)
            };
            match m281 {
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

pub fn f61(v0: Fn7, p286: u64, p287: Adt9) -> R<(Adt9, bool)> {
    let m285 = (p286, p287);
    {
        let mut m285 = m285;
        loop {
            let step = {
                let (v1, v2) = m285;
                let m282 = v1;
                if m282 == 0 {
                    Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m282 - 1;
                    let m284 = {
                        let c283: Fn7 = v0.clone();
                        c283.apply(v2.clone())?
                    };
                    match m284 {
                        None => {
                            Err::<(u64, Adt9), R<(Adt9, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt9), R<(Adt9, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: u64, v1: Bytes, v2: Adt9) -> R<Option<Adt9>> {
    f84(v1, v0, v2)
}

pub fn f63(v0: List<Bytes>) -> Bytes {
    let m290 = {
        let (h288, _) = f85(Fn10::F86, {
            let a289 = v0.clone();
            length_list(a289)
        }, v0);
        h288
    };
    match m290.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f64(v0: List<Bytes>) -> List<Bytes> {
    f87(Fn11::F88, List::<Bytes>::nil(), v0)
}

pub fn f65(v0: Fn8, p295: u64, p296: u64) -> R<(u64, bool)> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v1, v2) = m294;
                let m291 = v1;
                if m291 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m291 - 1;
                    let m293 = {
                        let c292: Fn8 = v0.clone();
                        c292.apply(v2)?
                    };
                    match m293 {
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
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f89(v1, v0, v2)
}

pub fn f67(v0: Adt0) -> Option<Adt11> {
    let m297 = v0;
    match m297 {
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
        Adt0::C6(_, _) => {
            None::<Adt11>
        }
        Adt0::C7(v1, r298, r299) => {
            let v2 = (*r298).clone();
            let v3 = (*r299).clone();
            Some(Adt11::C0(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3)))
        }
    }
}

pub fn f68(v0: Adt0, v1: Adt0) -> R<Option<Adt0>> {
    let m300 = f55(f22(v1, f15(Str::lit("$ref"))));
    match m300 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v2) => {
            Ok(f24(v0, f56(v2)?))
        }
    }
}

pub fn f69(v0: Adt0) -> Option<Bytes> {
    let m301 = v0;
    match m301 {
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

pub fn f70(v0: Fn9, p306: u64, p307: Adt10) -> R<(Adt10, bool)> {
    let m305 = (p306, p307);
    {
        let mut m305 = m305;
        loop {
            let step = {
                let (v1, v2) = m305;
                let m302 = v1;
                if m302 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m302 - 1;
                    let m304 = {
                        let c303: Fn9 = v0.clone();
                        c303.apply(v2.clone())?
                    };
                    match m304 {
                        None => {
                            Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), R<(Adt10, bool)>>((v3, v4))
                        }
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

pub fn f71(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    f90(v0, v1)
}

pub fn f72(v0: List<Adt5>, v1: u64, _: Adt0, v3: Adt2) -> R<Adt2> {
    let v4: u64 = match v3.clone() {
        Adt2::C0(h308, _, _, _, _, _, _, _, _) => {
            h308
        }
    };
    let v5: u64 = match v3.clone() {
        Adt2::C0(_, _, _, _, _, _, h309, _, _) => {
            h309
        }
    };
    let a310 = 256u64;
    let a311 = v5;
    if nat_lt(a310, a311) {
        Ok(f91(v0, v1, v3, v4, f15(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let m313 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h312, _, _, _) => {
                (*h312).clone()
            }
        };
        match m313 {
            Adt4::C0 => {
                Ok(f92(v3, v4, 3u64, Adt4::C0, v5))
            }
            Adt4::C1 => {
                Ok(f92(v3, v4, 2u64, Adt4::C1, v5))
            }
            Adt4::C2 => {
                let a314 = f93(v0.clone(), v4);
                let a315 = 0u64;
                if nat_eq(a314, a315) {
                    f77(v0, v1, v3, 0u64)
                } else {
                    let v6: Adt12 = f94(v0, v1, v4)?;
                    let m317 = match v6.clone() {
                        Adt12::C0(_, _, h316) => {
                            h316
                        }
                    };
                    match m317 {
                        None => {
                            Ok(f96(v3, match v6.clone() {
                                Adt12::C0(_, h319, _) => {
                                    h319
                                }
                            }, match v6 {
                                Adt12::C0(h320, _, _) => {
                                    (*h320).clone()
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(f95(v3, match v6 {
                                Adt12::C0(_, _, h318) => {
                                    h318
                                }
                            }))
                        }
                    }
                }
            }
            Adt4::C3 => {
                Ok(f92(v3, v4, 4u64, Adt4::C3, v5))
            }
            Adt4::C4(_) => {
                Ok(f92(v3.clone(), v4, 1u64, match v3 {
                    Adt2::C0(_, _, _, _, _, h321, _, _, _) => {
                        (*h321).clone()
                    }
                }, v5))
            }
            Adt4::C5 => {
                Ok(f92(v3, v4, 5u64, Adt4::C5, v5))
            }
        }
    }
}

pub fn f73(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2) -> R<Adt2> {
    let v4: u64 = match v3.clone() {
        Adt2::C0(h322, _, _, _, _, _, _, _, _) => {
            h322
        }
    };
    let v5: u64 = match v3.clone() {
        Adt2::C0(_, _, _, _, _, _, h323, _, _) => {
            h323
        }
    };
    let v6: Adt4 = match v3.clone() {
        Adt2::C0(_, _, _, _, _, h324, _, _, _) => {
            (*h324).clone()
        }
    };
    let v7: u64 = f93(v0.clone(), v4);
    let a325 = v7;
    let a326 = 2u64;
    if nat_eq(a325, a326) {
        let a329 = f93(v0, {
            let a327 = v4;
            let a328 = 1u64;
            nat_add(a327, a328)?
        });
        let a330 = 3u64;
        if nat_eq(a329, a330) {
            Ok(f96(v3, {
                let a331 = v4;
                let a332 = 2u64;
                nat_add(a331, a332)?
            }, f9(List::<Adt0>::nil())))
        } else {
            let v8: Adt4 = f97(v2, v6)?;
            Ok(f98(v3, f13(f14(1u64, v5), v8.clone(), None::<Adt0>, List::<(Bytes, Adt0)>::nil()), {
                let a333 = v4;
                let a334 = 1u64;
                nat_add(a333, a334)?
            }, 0u64, v8, {
                let a335 = v5;
                let a336 = 1u64;
                nat_add(a335, a336)?
            }))
        }
    } else {
        let a337 = v7;
        let a338 = 0u64;
        if nat_eq(a337, a338) {
            f99(v0, v1, v2.clone(), v3, f13(f14(2u64, v5), v6.clone(), f100(v2, v6), List::<(Bytes, Adt0)>::nil()), {
                let a339 = v4;
                let a340 = 1u64;
                nat_add(a339, a340)?
            })
        } else {
            let v9: Bytes = f101(v0.clone(), v4);
            if f102({
                let a341 = v7;
                let a342 = 12u64;
                nat_eq(a341, a342)
            }, f102({
                let a345 = f93(v0.clone(), {
                    let a343 = v4;
                    let a344 = 1u64;
                    nat_add(a343, a344)?
                });
                let a346 = 0u64;
                nat_eq(a345, a346)
            }, f102(f103(v9.clone())?, !f104(v9.clone())?))) {
                f99(v0, v1, v2.clone(), v3, f13(f14(2u64, v5), v6.clone(), f105(v2, v6, v9.clone())?, List::cons((f15(Str::lit("kind")), f106(v9)), List::<(Bytes, Adt0)>::nil())), {
                    let a347 = v4;
                    let a348 = 2u64;
                    nat_add(a347, a348)?
                })
            } else {
                Ok(f92(v3, v4, 5u64, v6, v5))
            }
        }
    }
}

pub fn f74(v0: List<Adt5>, v1: u64, v2: Adt2) -> R<Adt2> {
    let v3: u64 = match v2.clone() {
        Adt2::C0(h349, _, _, _, _, _, _, _, _) => {
            h349
        }
    };
    let v4: u64 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h350, _, _) => {
            h350
        }
    };
    let a351 = 256u64;
    let a352 = v4;
    if nat_lt(a351, a352) {
        Ok(f91(v0, v1, v2, v3, f15(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f93(v0.clone(), v3);
        let v6: Bytes = f101(v0.clone(), v3);
        let a353 = v5;
        let a354 = 12u64;
        if nat_eq(a353, a354) {
            if f102({
                let a357 = f93(v0.clone(), {
                    let a355 = v3;
                    let a356 = 1u64;
                    nat_add(a355, a356)?
                });
                let a358 = 0u64;
                nat_eq(a357, a358)
            }, f103(v6.clone())?) {
                Ok(f92(v2, v3, 1u64, Adt4::C1, v4))
            } else {
                let m359 = f107(v6.clone());
                match m359 {
                    None => {
                        if f102(f109(v6.clone(), f15(Str::lit("'"))), f103(f110(v6.clone(), 1u64))?) {
                            Ok(f96(v2, {
                                let a362 = v3;
                                let a363 = 1u64;
                                nat_add(a362, a363)?
                            }, f108(f15(Str::lit("parameter")), f111(Str::lit("name"), f106(f110(v6, 1u64))))?))
                        } else {
                            if f112(v6.clone(), f113()) {
                                Ok(f91(v0, v1, v2, v3, f114(f114(f114(f114(f15(Str::lit("`")), v6.clone()), f15(Str::lit("` takes arguments: write `("))), v6), f15(Str::lit(" \u{2026})`")))))
                            } else {
                                let v8: Adt12 = f94(v0, v1, v3)?;
                                let m365 = match v8.clone() {
                                    Adt12::C0(_, _, h364) => {
                                        h364
                                    }
                                };
                                match m365 {
                                    None => {
                                        Ok(f96(v2, match v8.clone() {
                                            Adt12::C0(_, h367, _) => {
                                                h367
                                            }
                                        }, f108(f15(Str::lit("named")), f115(Str::lit("member"), match v8 {
                                            Adt12::C0(h368, _, _) => {
                                                (*h368).clone()
                                            }
                                        }, Str::lit("arguments"), f9(List::<Adt0>::nil())))?))
                                    }
                                    Some(_) => {
                                        Ok(f95(v2, match v8 {
                                            Adt12::C0(_, _, h366) => {
                                                h366
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    Some(v7) => {
                        Ok(f96(v2, {
                            let a360 = v3;
                            let a361 = 1u64;
                            nat_add(a360, a361)?
                        }, f108(v7, List::<(Bytes, Adt0)>::nil())?))
                    }
                }
            }
        } else {
            let a369 = v5;
            let a370 = 4u64;
            if nat_eq(a369, a370) {
                f116(v0, v1, f98(v2, f14(11u64, v4), {
                    let a371 = v3;
                    let a372 = 1u64;
                    nat_add(a371, a372)?
                }, 2u64, Adt4::C1, v4), {
                    let a373 = v3;
                    let a374 = 1u64;
                    nat_add(a373, a374)?
                }, v4)
            } else {
                Ok(f91(v0.clone(), v1, v2, v3, f114(f15(Str::lit("expected a type, found ")), f117(v0, v3)?)))
            }
        }
    }
}

pub fn f75(v0: List<Adt5>, v1: u64, v2: Adt2) -> R<Adt2> {
    let v3: u64 = match v2.clone() {
        Adt2::C0(h375, _, _, _, _, _, _, _, _) => {
            h375
        }
    };
    let v4: u64 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h376, _, _) => {
            h376
        }
    };
    let a377 = 256u64;
    let a378 = v4;
    if nat_lt(a377, a378) {
        Ok(f91(v0, v1, v2, v3, f15(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v5: u64 = f93(v0.clone(), v3);
        let v6: Bytes = f101(v0.clone(), v3);
        let a379 = v5;
        let a380 = 11u64;
        if nat_eq(a379, a380) {
            Ok(f96(v2, {
                let a381 = v3;
                let a382 = 1u64;
                nat_add(a381, a382)?
            }, f108(f15(Str::lit("string")), f111(Str::lit("value"), f106(v6)))?))
        } else {
            let a383 = v5;
            let a384 = 4u64;
            if nat_eq(a383, a384) {
                f118(v0, v1, f98(v2, f14(30u64, v4), {
                    let a385 = v3;
                    let a386 = 1u64;
                    nat_add(a385, a386)?
                }, 3u64, Adt4::C0, v4), {
                    let a387 = v3;
                    let a388 = 1u64;
                    nat_add(a387, a388)?
                }, v4)
            } else {
                let a389 = v5;
                let a390 = 12u64;
                if nat_eq(a389, a390) {
                    if f102({
                        let a393 = f93(v0.clone(), {
                            let a391 = v3;
                            let a392 = 1u64;
                            nat_add(a391, a392)?
                        });
                        let a394 = 0u64;
                        nat_eq(a393, a394)
                    }, f103(v6.clone())?) {
                        Ok(f92(v2, v3, 1u64, Adt4::C0, v4))
                    } else {
                        f119(v0, v1, v2, v3, v6)
                    }
                } else {
                    Ok(f91(v0.clone(), v1, v2, v3, f114(f15(Str::lit("expected a term, found ")), f117(v0, v3)?)))
                }
            }
        }
    }
}

pub fn f76(v0: List<Adt5>, v1: u64, v2: Adt2) -> R<Adt2> {
    let v3: u64 = match v2.clone() {
        Adt2::C0(h395, _, _, _, _, _, _, _, _) => {
            h395
        }
    };
    let v4: u64 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h396, _, _) => {
            h396
        }
    };
    let a397 = f93(v0.clone(), v3);
    let a398 = 0u64;
    if nat_eq(a397, a398) {
        Ok(f92(v2, v3, 1u64, Adt4::C3, v4))
    } else {
        let v5: Adt12 = f120(v0.clone(), v1, v3, f15(Str::lit("a parameter name")))?;
        let m400 = match v5.clone() {
            Adt12::C0(_, _, h399) => {
                h399
            }
        };
        match m400 {
            None => {
                let a403 = f93(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h402, _) => {
                        h402
                    }
                });
                let a404 = 7u64;
                if nat_eq(a403, a404) {
                    Ok(f98(v2, f121(f14(20u64, v4), match v5.clone() {
                        Adt12::C0(h405, _, _) => {
                            (*h405).clone()
                        }
                    }, Adt0::C5), {
                        let a407 = match v5 {
                            Adt12::C0(_, h406, _) => {
                                h406
                            }
                        };
                        let a408 = 1u64;
                        nat_add(a407, a408)?
                    }, 2u64, Adt4::C1, {
                        let a409 = v4;
                        let a410 = 1u64;
                        nat_add(a409, a410)?
                    }))
                } else {
                    Ok(f91(v0, v1, v2, match v5 {
                        Adt12::C0(_, h411, _) => {
                            h411
                        }
                    }, f15(Str::lit("expected `:` after a parameter name"))))
                }
            }
            Some(_) => {
                Ok(f95(v2, match v5 {
                    Adt12::C0(_, _, h401) => {
                        h401
                    }
                }))
            }
        }
    }
}

pub fn f77(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64) -> R<Adt2> {
    let v4: Adt13 = f122(v0, v1, match v2.clone() {
        Adt2::C0(h412, _, _, _, _, _, _, _, _) => {
            h412
        }
    }, v3)?;
    let m414 = match v4.clone() {
        Adt13::C0(_, _, _, _, _, h413) => {
            h413
        }
    };
    match m414 {
        None => {
            Ok(f96(v2, match v4.clone() {
                Adt13::C0(h416, _, _, _, _, _) => {
                    h416
                }
            }, match v4 {
                Adt13::C0(_, _, _, _, h417, _) => {
                    (*h417).clone()
                }
            }))
        }
        Some(_) => {
            Ok(f95(v2, match v4 {
                Adt13::C0(_, _, _, _, _, h415) => {
                    h415
                }
            }))
        }
    }
}

pub fn f78(v0: List<Adt5>, v1: u64, v2: Adt2) -> R<Adt2> {
    let v3: u64 = match v2.clone() {
        Adt2::C0(h418, _, _, _, _, _, _, _, _) => {
            h418
        }
    };
    let a419 = f93(v0.clone(), v3);
    let a420 = 2u64;
    if nat_eq(a419, a420) {
        f123(v0, v1, v2.clone(), f14(18u64, match v2 {
            Adt2::C0(_, _, _, _, _, _, h421, _, _) => {
                h421
            }
        }), {
            let a422 = v3;
            let a423 = 1u64;
            nat_add(a422, a423)?
        })
    } else {
        Ok(f91(v0, v1, v2, v3, f15(Str::lit("expected `[`"))))
    }
}

pub fn f79(v0: Adt2, v1: Adt0, v2: List<(u64, u64)>) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(h424, _, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt2::C0(_, h425, _, _, _, _, _, _, _) => {
            h425
        }
    }, std::rc::Rc::new(v1), 9u64, match v0.clone() {
        Adt2::C0(_, _, _, _, h426, _, _, _, _) => {
            h426
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h427, _, _, _) => {
            (*h427).clone()
        }
    }), match v0 {
        Adt2::C0(_, _, _, _, _, _, h428, _, _) => {
            h428
        }
    }, None::<Adt1>, v2)
}

pub fn f80(v0: List<Adt5>, v1: u64, _: Adt0, v3: Adt2, v4: Adt3) -> Adt2 {
    let v5: u64 = match v3.clone() {
        Adt2::C0(h429, _, _, _, _, _, _, _, _) => {
            h429
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt2::C0(_, _, h430, _, _, _, _, _, _) => {
            (*h430).clone()
        }
    };
    let a432 = match v4.clone() {
        Adt3::C0(_, h431, _, _, _, _, _, _, _, _, _, _) => {
            h431
        }
    };
    let a433 = 0u64;
    if nat_eq(a432, a433) {
        let a434 = f93(v0.clone(), v5);
        let a435 = 99u64;
        if nat_eq(a434, a435) {
            f79(v3, v6, List::<(u64, u64)>::nil())
        } else {
            f91(v0, v1, v3, v5, f15(Str::lit("unexpected text after the value")))
        }
    } else {
        let v7: Adt3 = f124(f125(v4.clone(), v6), (match v4.clone() {
            Adt3::C0(_, _, _, h436, _, _, _, _, _, _, _, _) => {
                h436
            }
        }, f126(v0.clone(), v5)), 0u64);
        let a437 = f93(v0.clone(), v5);
        let a438 = 99u64;
        if nat_eq(a437, a438) {
            f79(v3, f9(match v7.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h439, _, _, _, _) => {
                    h439
                }
            }), match v7 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h440) => {
                    h440
                }
            })
        } else {
            f98(v3, f11(v7, f15(Str::lit("")), f16(v0, v1, v5)), v5, 0u64, match v4 {
                Adt3::C0(_, _, _, _, _, h441, _, _, _, _, _, _) => {
                    (*h441).clone()
                }
            }, 1u64)
        }
    }
}

pub fn f81(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2, v4: Adt3) -> R<Adt2> {
    let v5: u64 = match v3.clone() {
        Adt2::C0(h442, _, _, _, _, _, _, _, _) => {
            h442
        }
    };
    let v6: Adt0 = match v3.clone() {
        Adt2::C0(_, _, h443, _, _, _, _, _, _) => {
            (*h443).clone()
        }
    };
    let a445 = match v4.clone() {
        Adt3::C0(h444, _, _, _, _, _, _, _, _, _, _, _) => {
            h444
        }
    };
    let a446 = 1u64;
    if nat_eq(a445, a446) {
        let v7: Adt3 = f125(v4.clone(), v6);
        let a447 = f93(v0.clone(), v5);
        let a448 = 6u64;
        if nat_eq(a447, a448) {
            let a451 = f93(v0, {
                let a449 = v5;
                let a450 = 1u64;
                nat_add(a449, a450)?
            });
            let a452 = 3u64;
            if nat_eq(a451, a452) {
                Ok(f96(v3, {
                    let a453 = v5;
                    let a454 = 2u64;
                    nat_add(a453, a454)?
                }, f9(match v7 {
                    Adt3::C0(_, _, _, _, _, _, _, h455, _, _, _, _) => {
                        h455
                    }
                })))
            } else {
                Ok(f98(v3, v7, {
                    let a456 = v5;
                    let a457 = 1u64;
                    nat_add(a456, a457)?
                }, 0u64, match v4.clone() {
                    Adt3::C0(_, _, _, _, _, h458, _, _, _, _, _, _) => {
                        (*h458).clone()
                    }
                }, {
                    let a460 = match v4 {
                        Adt3::C0(_, _, h459, _, _, _, _, _, _, _, _, _) => {
                            h459
                        }
                    };
                    let a461 = 1u64;
                    nat_add(a460, a461)?
                }))
            }
        } else {
            let a462 = f93(v0.clone(), v5);
            let a463 = 3u64;
            if nat_eq(a462, a463) {
                Ok(f96(v3, {
                    let a464 = v5;
                    let a465 = 1u64;
                    nat_add(a464, a465)?
                }, f9(match v7 {
                    Adt3::C0(_, _, _, _, _, _, _, h466, _, _, _, _) => {
                        h466
                    }
                })))
            } else {
                Ok(f91(v0, v1, v3, v5, f15(Str::lit("expected `,` or `]`"))))
            }
        }
    } else {
        let v8: Bytes = match v4.clone() {
            Adt3::C0(_, _, _, _, h467, _, _, _, _, _, _, _) => {
                h467
            }
        };
        if f127(match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h468, _, _, _) => {
                h468
            }
        }, v8.clone()) {
            Ok(f128(v3, match v4 {
                Adt3::C0(_, _, _, h469, _, _, _, _, _, _, _, _) => {
                    h469
                }
            }, f114(f114(f15(Str::lit("the member `")), v8), f15(Str::lit("` is written twice")))))
        } else {
            let v9: Adt3 = f13(v4.clone(), match v4.clone() {
                Adt3::C0(_, _, _, _, _, h470, _, _, _, _, _, _) => {
                    (*h470).clone()
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h471, _, _, _, _, _) => {
                    h471
                }
            }, List::cons((v8, v6), match v4 {
                Adt3::C0(_, _, _, _, _, _, _, _, h472, _, _, _) => {
                    h472
                }
            }));
            let a473 = f93(v0.clone(), v5);
            let a474 = 6u64;
            if nat_eq(a473, a474) {
                f99(v0, v1, v2, v3, v9, {
                    let a475 = v5;
                    let a476 = 1u64;
                    nat_add(a475, a476)?
                })
            } else {
                let a477 = f93(v0.clone(), v5);
                let a478 = 1u64;
                if nat_eq(a477, a478) {
                    Ok(f96(v3, {
                        let a479 = v5;
                        let a480 = 1u64;
                        nat_add(a479, a480)?
                    }, f129(match v9 {
                        Adt3::C0(_, _, _, _, _, _, _, _, h481, _, _, _) => {
                            h481
                        }
                    })?))
                } else {
                    Ok(f91(v0, v1, v3, v5, f15(Str::lit("expected `,` or `}`"))))
                }
            }
        }
    }
}

pub fn f82(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3) -> R<Adt2> {
    let v4: u64 = match v2.clone() {
        Adt2::C0(h482, _, _, _, _, _, _, _, _) => {
            h482
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt2::C0(_, _, h483, _, _, _, _, _, _) => {
            (*h483).clone()
        }
    };
    let v6: u64 = match v3.clone() {
        Adt3::C0(h484, _, _, _, _, _, _, _, _, _, _, _) => {
            h484
        }
    };
    let v7: u64 = match v3.clone() {
        Adt3::C0(_, h485, _, _, _, _, _, _, _, _, _, _) => {
            h485
        }
    };
    let a486 = v6;
    let a487 = 11u64;
    if nat_eq(a486, a487) {
        let a488 = f93(v0.clone(), v4);
        let a489 = 5u64;
        if nat_eq(a488, a489) {
            Ok(f96(v2, {
                let a490 = v4;
                let a491 = 1u64;
                nat_add(a490, a491)?
            }, v5))
        } else {
            Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `)`"))))
        }
    } else {
        let a492 = v6;
        let a493 = 12u64;
        if nat_eq(a492, a493) {
            let v8: Bytes = match v3 {
                Adt3::C0(_, _, _, _, h494, _, _, _, _, _, _, _) => {
                    h494
                }
            };
            Ok(f96(v2, v4, if f57(v8.clone(), f15(Str::lit("List"))) {
                f108(f15(Str::lit("list")), f111(Str::lit("element"), v5))?
            } else {
                if f57(v8, f15(Str::lit("Set"))) {
                    f108(f15(Str::lit("set")), f111(Str::lit("element"), v5))?
                } else {
                    f108(f15(Str::lit("option")), f111(Str::lit("value"), v5))?
                }
            }))
        } else {
            let a495 = v6;
            let a496 = 13u64;
            if nat_eq(a495, a496) {
                let a497 = v7;
                let a498 = 0u64;
                if nat_eq(a497, a498) {
                    f130(v2, f12(f121(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f96(v2, v4, if f57(match v3.clone() {
                        Adt3::C0(_, _, _, _, h499, _, _, _, _, _, _, _) => {
                            h499
                        }
                    }, f15(Str::lit("Result"))) {
                        f108(f15(Str::lit("result")), f115(Str::lit("ok"), match v3 {
                            Adt3::C0(_, _, _, _, _, _, _, _, _, h500, _, _) => {
                                (*h500).clone()
                            }
                        }, Str::lit("error"), v5))?
                    } else {
                        f108(f15(Str::lit("map")), f115(Str::lit("key"), match v3 {
                            Adt3::C0(_, _, _, _, _, _, _, _, _, h501, _, _) => {
                                (*h501).clone()
                            }
                        }, Str::lit("value"), v5))?
                    }))
                }
            } else {
                let a502 = v6;
                let a503 = 14u64;
                if nat_eq(a502, a503) {
                    let a504 = v7;
                    let a505 = 0u64;
                    if nat_eq(a504, a505) {
                        f131(v0, v1, v2, f125(v3, v5), v4)
                    } else {
                        Ok(f96(v2, v4, f108(f15(Str::lit("function")), f115(Str::lit("parameters"), f9(match v3 {
                            Adt3::C0(_, _, _, _, _, _, _, h506, _, _, _, _) => {
                                h506
                            }
                        }), Str::lit("result"), v5))?))
                    }
                } else {
                    let a507 = v6;
                    let a508 = 16u64;
                    if nat_eq(a507, a508) {
                        f132(v0, v1, v2, f125(v3, v5), v4)
                    } else {
                        let a509 = v6;
                        let a510 = 17u64;
                        if nat_eq(a509, a510) {
                            let a511 = v7;
                            let a512 = 0u64;
                            if nat_eq(a511, a512) {
                                if f133(v0.clone(), v4, f15(Str::lit("*"))) {
                                    f130(v2, f12(f121(v3, v5, Adt0::C5), 1u64), {
                                        let a513 = v4;
                                        let a514 = 1u64;
                                        nat_add(a513, a514)?
                                    })
                                } else {
                                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `*` between the two types of a product"))))
                                }
                            } else {
                                Ok(f96(v2, v4, f108(f15(Str::lit("product")), f115(Str::lit("left"), match v3 {
                                    Adt3::C0(_, _, _, _, _, _, _, _, _, h515, _, _) => {
                                        (*h515).clone()
                                    }
                                }, Str::lit("right"), v5))?))
                            }
                        } else {
                            let a516 = v6;
                            let a517 = 18u64;
                            if nat_eq(a516, a517) {
                                let v9: Adt3 = f125(v3, v5);
                                let a518 = f93(v0.clone(), v4);
                                let a519 = 6u64;
                                if nat_eq(a518, a519) {
                                    f123(v0, v1, v2, v9, {
                                        let a520 = v4;
                                        let a521 = 1u64;
                                        nat_add(a520, a521)?
                                    })
                                } else {
                                    let a522 = f93(v0.clone(), v4);
                                    let a523 = 3u64;
                                    if nat_eq(a522, a523) {
                                        Ok(f96(v2, {
                                            let a524 = v4;
                                            let a525 = 1u64;
                                            nat_add(a524, a525)?
                                        }, f9(match v9 {
                                            Adt3::C0(_, _, _, _, _, _, _, h526, _, _, _, _) => {
                                                h526
                                            }
                                        })))
                                    } else {
                                        Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `,` or `]`"))))
                                    }
                                }
                            } else {
                                Ok(f96(v2, v4, f129(f115(Str::lit("name"), match v3 {
                                    Adt3::C0(_, _, _, _, _, _, _, _, _, h527, _, _) => {
                                        (*h527).clone()
                                    }
                                }, Str::lit("type"), v5))?))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3) -> R<Adt2> {
    let v4: u64 = match v2.clone() {
        Adt2::C0(h528, _, _, _, _, _, _, _, _) => {
            h528
        }
    };
    let v5: Adt0 = match v2.clone() {
        Adt2::C0(_, _, h529, _, _, _, _, _, _) => {
            (*h529).clone()
        }
    };
    let v6: u64 = match v3.clone() {
        Adt3::C0(h530, _, _, _, _, _, _, _, _, _, _, _) => {
            h530
        }
    };
    let v7: u64 = match v3.clone() {
        Adt3::C0(_, h531, _, _, _, _, _, _, _, _, _, _) => {
            h531
        }
    };
    let a532 = v6;
    let a533 = 30u64;
    if nat_eq(a532, a533) {
        let a534 = f93(v0.clone(), v4);
        let a535 = 5u64;
        if nat_eq(a534, a535) {
            Ok(f96(v2, {
                let a536 = v4;
                let a537 = 1u64;
                nat_add(a536, a537)?
            }, v5))
        } else {
            Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `)`"))))
        }
    } else {
        let a538 = v6;
        let a539 = 31u64;
        if nat_eq(a538, a539) {
            Ok(f96(v2, v4, f108(match v3 {
                Adt3::C0(_, _, _, _, h540, _, _, _, _, _, _, _) => {
                    h540
                }
            }, f111(Str::lit("value"), v5))?))
        } else {
            let a541 = v6;
            let a542 = 32u64;
            if nat_eq(a541, a542) {
                let a543 = v7;
                let a544 = 0u64;
                if nat_eq(a543, a544) {
                    f134(v2, f12(f121(v3, v5, Adt0::C5), 1u64), v4)
                } else {
                    Ok(f96(v2, v4, f108(f15(Str::lit("cons")), f115(Str::lit("head"), match v3 {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h545, _, _) => {
                            (*h545).clone()
                        }
                    }, Str::lit("tail"), v5))?))
                }
            } else {
                let a546 = v6;
                let a547 = 33u64;
                if nat_eq(a546, a547) {
                    Ok(f96(v2, v4, f108(f15(Str::lit("nil")), f111(Str::lit("element"), v5))?))
                } else {
                    let a548 = v6;
                    let a549 = 34u64;
                    if nat_eq(a548, a549) {
                        let a550 = v7;
                        let a551 = 0u64;
                        if nat_eq(a550, a551) {
                            if f133(v0.clone(), v4, f15(Str::lit("then"))) {
                                f134(v2, f12(f121(v3, v5, Adt0::C5), 1u64), {
                                    let a552 = v4;
                                    let a553 = 1u64;
                                    nat_add(a552, a553)?
                                })
                            } else {
                                Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `then`"))))
                            }
                        } else {
                            let a554 = v7;
                            let a555 = 1u64;
                            if nat_eq(a554, a555) {
                                if f133(v0.clone(), v4, f15(Str::lit("else"))) {
                                    f134(v2, f12(f121(v3.clone(), match v3 {
                                        Adt3::C0(_, _, _, _, _, _, _, _, _, h556, _, _) => {
                                            (*h556).clone()
                                        }
                                    }, v5), 2u64), {
                                        let a557 = v4;
                                        let a558 = 1u64;
                                        nat_add(a557, a558)?
                                    })
                                } else {
                                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `else`"))))
                                }
                            } else {
                                Ok(f96(v2, v4, f108(f15(Str::lit("if")), List::cons((f15(Str::lit("else_value")), v5), f115(Str::lit("condition"), match v3.clone() {
                                    Adt3::C0(_, _, _, _, _, _, _, _, _, h559, _, _) => {
                                        (*h559).clone()
                                    }
                                }, Str::lit("then_value"), match v3 {
                                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h560, _) => {
                                        (*h560).clone()
                                    }
                                })))?))
                            }
                        }
                    } else {
                        let a561 = v6;
                        let a562 = 35u64;
                        if nat_eq(a561, a562) {
                            let a563 = v7;
                            let a564 = 0u64;
                            if nat_eq(a563, a564) {
                                let a565 = f93(v0.clone(), v4);
                                let a566 = 8u64;
                                if nat_eq(a565, a566) {
                                    f134(v2, f12(f121(v3, v5, Adt0::C5), 1u64), {
                                        let a567 = v4;
                                        let a568 = 1u64;
                                        nat_add(a567, a568)?
                                    })
                                } else {
                                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `:=`"))))
                                }
                            } else {
                                let a569 = v7;
                                let a570 = 1u64;
                                if nat_eq(a569, a570) {
                                    let a571 = f93(v0.clone(), v4);
                                    let a572 = 10u64;
                                    if nat_eq(a571, a572) {
                                        f134(v2, f12(f121(v3.clone(), match v3 {
                                            Adt3::C0(_, _, _, _, _, _, _, _, _, h573, _, _) => {
                                                (*h573).clone()
                                            }
                                        }, v5), 2u64), {
                                            let a574 = v4;
                                            let a575 = 1u64;
                                            nat_add(a574, a575)?
                                        })
                                    } else {
                                        Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `;`"))))
                                    }
                                } else {
                                    Ok(f96(v2, v4, f108(f15(Str::lit("let")), List::cons((f15(Str::lit("body")), v5), f115(Str::lit("binder"), match v3.clone() {
                                        Adt3::C0(_, _, _, _, _, _, _, _, _, h576, _, _) => {
                                            (*h576).clone()
                                        }
                                    }, Str::lit("value"), match v3 {
                                        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h577, _) => {
                                            (*h577).clone()
                                        }
                                    })))?))
                                }
                            }
                        } else {
                            let a578 = v6;
                            let a579 = 36u64;
                            if nat_eq(a578, a579) {
                                let a580 = v7;
                                let a581 = 0u64;
                                if nat_eq(a580, a581) {
                                    let a582 = f93(v0.clone(), v4);
                                    let a583 = 5u64;
                                    if nat_eq(a582, a583) {
                                        let a586 = f93(v0.clone(), {
                                            let a584 = v4;
                                            let a585 = 1u64;
                                            nat_add(a584, a585)?
                                        });
                                        let a587 = 6u64;
                                        if nat_eq(a586, a587) {
                                            f134(v2, f12(f121(v3, v5, Adt0::C5), 1u64), {
                                                let a588 = v4;
                                                let a589 = 2u64;
                                                nat_add(a588, a589)?
                                            })
                                        } else {
                                            Ok(f91(v0, v1, v2, {
                                                let a590 = v4;
                                                let a591 = 1u64;
                                                nat_add(a590, a591)?
                                            }, f15(Str::lit("expected `,` before the body"))))
                                        }
                                    } else {
                                        Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `)`"))))
                                    }
                                } else {
                                    Ok(f96(v2, v4, f108(f15(Str::lit("forall")), f115(Str::lit("binder"), match v3 {
                                        Adt3::C0(_, _, _, _, _, _, _, _, _, h592, _, _) => {
                                            (*h592).clone()
                                        }
                                    }, Str::lit("body"), v5))?))
                                }
                            } else {
                                let a593 = v6;
                                let a594 = 37u64;
                                if nat_eq(a593, a594) {
                                    let a595 = v7;
                                    let a596 = 0u64;
                                    if nat_eq(a595, a596) {
                                        let a597 = f93(v0.clone(), v4);
                                        let a598 = 5u64;
                                        if nat_eq(a597, a598) {
                                            f135(v0, v1, v2, f125(v3, v5), {
                                                let a599 = v4;
                                                let a600 = 1u64;
                                                nat_add(a599, a600)?
                                            })
                                        } else {
                                            Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `)`"))))
                                        }
                                    } else {
                                        Ok(f96(v2, v4, f108(f15(Str::lit("lambda")), List::cons((f15(Str::lit("body")), v5), f115(Str::lit("parameters"), f9(match v3.clone() {
                                            Adt3::C0(_, _, _, _, _, _, _, h601, _, _, _, _) => {
                                                h601
                                            }
                                        }), Str::lit("captures"), match v3 {
                                            Adt3::C0(_, _, _, _, _, _, _, _, _, h602, _, _) => {
                                                (*h602).clone()
                                            }
                                        })))?))
                                    }
                                } else {
                                    let a603 = v6;
                                    let a604 = 38u64;
                                    if nat_eq(a603, a604) {
                                        let a605 = v7;
                                        let a606 = 0u64;
                                        if nat_eq(a605, a606) {
                                            f136(v0, v1, v2, f12(f121(v3, v5, Adt0::C5), 1u64), v4)
                                        } else {
                                            f136(v0, v1, v2, f125(v3, v5), v4)
                                        }
                                    } else {
                                        let a607 = v6;
                                        let a608 = 39u64;
                                        if nat_eq(a607, a608) {
                                            let v8: Adt12 = f120(v0, v1, v4, f15(Str::lit("a field name")))?;
                                            let m610 = match v8.clone() {
                                                Adt12::C0(_, _, h609) => {
                                                    h609
                                                }
                                            };
                                            match m610 {
                                                None => {
                                                    Ok(f96(v2, match v8.clone() {
                                                        Adt12::C0(_, h612, _) => {
                                                            h612
                                                        }
                                                    }, f108(f15(Str::lit("project")), f115(Str::lit("value"), v5, Str::lit("field"), match v8 {
                                                        Adt12::C0(h613, _, _) => {
                                                            (*h613).clone()
                                                        }
                                                    }))?))
                                                }
                                                Some(_) => {
                                                    Ok(f95(v2, match v8 {
                                                        Adt12::C0(_, _, h611) => {
                                                            h611
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            let a614 = v6;
                                            let a615 = 40u64;
                                            if if nat_eq(a614, a615) {
                                                true
                                            } else {
                                                let a616 = v6;
                                                let a617 = 44u64;
                                                if nat_eq(a616, a617) {
                                                    true
                                                } else {
                                                    let a618 = v6;
                                                    let a619 = 46u64;
                                                    nat_eq(a618, a619)
                                                }
                                            } {
                                                let a620 = v7;
                                                let a621 = 0u64;
                                                if nat_eq(a620, a621) {
                                                    f137(v0, v1, v2, v3)
                                                } else {
                                                    f136(v0, v1, v2, f125(v3, v5), v4)
                                                }
                                            } else {
                                                let a622 = v6;
                                                let a623 = 41u64;
                                                if nat_eq(a622, a623) {
                                                    f137(v0, v1, v2, v3)
                                                } else {
                                                    let a624 = v6;
                                                    let a625 = 42u64;
                                                    if nat_eq(a624, a625) {
                                                        let a626 = v7;
                                                        let a627 = 0u64;
                                                        if nat_eq(a626, a627) {
                                                            f137(v0, v1, v2, v3)
                                                        } else {
                                                            let v10: Adt3 = f125(v3.clone(), f129(f115(Str::lit("field"), f106(match v3 {
                                                                Adt3::C0(_, _, _, _, h628, _, _, _, _, _, _, _) => {
                                                                    h628
                                                                }
                                                            }), Str::lit("value"), v5))?);
                                                            let a629 = f93(v0.clone(), v4);
                                                            let a630 = 6u64;
                                                            if nat_eq(a629, a630) {
                                                                f138(v0, v1, v2, v10, {
                                                                    let a631 = v4;
                                                                    let a632 = 1u64;
                                                                    nat_add(a631, a632)?
                                                                })
                                                            } else {
                                                                let a633 = f93(v0.clone(), v4);
                                                                let a634 = 1u64;
                                                                if nat_eq(a633, a634) {
                                                                    f138(v0, v1, v2, v10, v4)
                                                                } else {
                                                                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `,` or `}`"))))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a635 = v6;
                                                        let a636 = 43u64;
                                                        if nat_eq(a635, a636) {
                                                            let a637 = v7;
                                                            let a638 = 0u64;
                                                            if nat_eq(a637, a638) {
                                                                f139(v0, v1, v2, f125(v3, v5), v4)
                                                            } else {
                                                                Ok(f96(v2, v4, f108(f15(Str::lit("primitive")), List::cons((f15(Str::lit("result")), v5), f115(Str::lit("operation"), match v3.clone() {
                                                                    Adt3::C0(_, _, _, _, _, _, _, _, _, h639, _, _) => {
                                                                        (*h639).clone()
                                                                    }
                                                                }, Str::lit("arguments"), f9(match v3 {
                                                                    Adt3::C0(_, _, _, _, _, _, _, h640, _, _, _, _) => {
                                                                        h640
                                                                    }
                                                                }))))?))
                                                            }
                                                        } else {
                                                            let a641 = v6;
                                                            let a642 = 45u64;
                                                            if nat_eq(a641, a642) {
                                                                let a643 = v7;
                                                                let a644 = 0u64;
                                                                if nat_eq(a643, a644) {
                                                                    f140(v0, v1, v2, f12(f121(v3, v5, Adt0::C5), 2u64), v4)
                                                                } else {
                                                                    let a645 = v7;
                                                                    let a646 = 1u64;
                                                                    if nat_eq(a645, a646) {
                                                                        f140(v0, v1, v2, f12(f125(v3.clone(), f129(List::cons((f15(Str::lit("body")), v5), match v3 {
                                                                            Adt3::C0(_, _, _, _, _, _, _, _, h647, _, _, _) => {
                                                                                h647
                                                                            }
                                                                        }))?), 2u64), v4)
                                                                    } else {
                                                                        f140(v0, v1, v2, v3, v4)
                                                                    }
                                                                }
                                                            } else {
                                                                let a648 = v7;
                                                                let a649 = 0u64;
                                                                if nat_eq(a648, a649) {
                                                                    let a650 = f93(v0.clone(), v4);
                                                                    let a651 = 6u64;
                                                                    if nat_eq(a650, a651) {
                                                                        f134(v2, f12(f121(v3, v5, Adt0::C5), 1u64), {
                                                                            let a652 = v4;
                                                                            let a653 = 1u64;
                                                                            nat_add(a652, a653)?
                                                                        })
                                                                    } else {
                                                                        let m654 = f141(f101(v0.clone(), v4));
                                                                        match m654 {
                                                                            None => {
                                                                                Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected an operator or `,` after the first operand"))))
                                                                            }
                                                                            Some(v11) => {
                                                                                let a655 = f93(v0.clone(), v4);
                                                                                let a656 = 12u64;
                                                                                if nat_eq(a655, a656) {
                                                                                    f134(v2, f11(f12(f121(v3, v5, Adt0::C5), 2u64), v11, 0u64), {
                                                                                        let a657 = v4;
                                                                                        let a658 = 1u64;
                                                                                        nat_add(a657, a658)?
                                                                                    })
                                                                                } else {
                                                                                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected an operator or `,` after the first operand"))))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a659 = v7;
                                                                    let a660 = 1u64;
                                                                    if nat_eq(a659, a660) {
                                                                        Ok(f96(v2, v4, f108(f15(Str::lit("pair")), f115(Str::lit("left"), match v3 {
                                                                            Adt3::C0(_, _, _, _, _, _, _, _, _, h661, _, _) => {
                                                                                (*h661).clone()
                                                                            }
                                                                        }, Str::lit("right"), v5))?))
                                                                    } else {
                                                                        let v12: Bytes = match v3.clone() {
                                                                            Adt3::C0(_, _, _, _, h662, _, _, _, _, _, _, _) => {
                                                                                h662
                                                                            }
                                                                        };
                                                                        let v13: bool = f57(v12.clone(), f15(Str::lit("implies")));
                                                                        Ok(f96(v2, v4, f108(v12, List::cons((if v13 {
                                                                            f15(Str::lit("premise"))
                                                                        } else {
                                                                            f15(Str::lit("left"))
                                                                        }, match v3 {
                                                                            Adt3::C0(_, _, _, _, _, _, _, _, _, h663, _, _) => {
                                                                                (*h663).clone()
                                                                            }
                                                                        }), List::cons((if v13 {
                                                                            f15(Str::lit("conclusion"))
                                                                        } else {
                                                                            f15(Str::lit("right"))
                                                                        }, v5), List::<(Bytes, Adt0)>::nil())))?))
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

pub fn f84(v0: Bytes, v1: u64, v2: Adt9) -> R<Option<Adt9>> {
    let m665 = match v2.clone() {
        Adt9::C0(_, _, _, h664) => {
            h664
        }
    };
    match m665 {
        None => {
            if match v2.clone() {
                Adt9::C0(_, _, h666, _) => {
                    h666
                }
            } {
                Ok(None::<Adt9>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt9::C0(h667, _, _, _) => {
                        h667
                    }
                };
                let a668 = v4;
                let a669 = v1;
                if nat_lt(a668, a669) {
                    if f30(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt9::C0({
                            let a670 = v4;
                            let a671 = 1u64;
                            nat_add(a670, a671)?
                        }, match v2 {
                            Adt9::C0(_, h672, _, _) => {
                                h672
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f30(v0.clone(), v4, 92u8) {
                            Ok(Some(f142(v0, v1, v2)?))
                        } else {
                            let a673 = f143(v0.clone(), v4)?;
                            let a674 = 32u64;
                            if nat_lt(a673, a674) {
                                Ok(Some(f144(v2, v4, f49(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f145(v0.clone(), v1, v4)?;
                                Ok(Some(f146(v2, v5, f51(v0, v4, {
                                    let a675 = v5;
                                    let a676 = v4;
                                    nat_sub(a675, a676)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt9>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt9>)
        }
    }
}

pub fn f85(v0: Fn10, p681: u64, p682: List<Bytes>) -> (List<Bytes>, bool) {
    let m680 = (p681, p682);
    {
        let mut m680 = m680;
        loop {
            let step = {
                let (v1, v2) = m680;
                let m677 = v1;
                if m677 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m677 - 1;
                    let m679 = {
                        let c678: Fn10 = v0.clone();
                        c678.apply(v2.clone())
                    };
                    match m679 {
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
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: List<Bytes>) -> Option<List<Bytes>> {
    f147(v0)
}

pub fn f87(v0: Fn11, p686: List<Bytes>, p687: List<Bytes>) -> List<Bytes> {
    let m685 = (p686, p687);
    {
        let mut m685 = m685;
        loop {
            let step = {
                let (v1, v2) = m685;
                let m683 = v2;
                match m683.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c684: Fn11 = v0.clone();
                            c684.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f89(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a688 = v2;
    let a689 = v1;
    if nat_lt(a688, a689) {
        if f148(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a690 = v2;
                let a691 = 1u64;
                nat_add(a690, a691)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f90(v0: Bytes, v1: Adt10) -> R<Option<Adt10>> {
    let v2: u64 = match v1.clone() {
        Adt10::C0(h692, _) => {
            h692
        }
    };
    let m693 = f45(v0, v2);
    match m693 {
        None => {
            Ok(None::<Adt10>)
        }
        Some(v3) => {
            Ok(Some(Adt10::C0({
                let a694 = v2;
                let a695 = 1u64;
                nat_add(a694, a695)?
            }, {
                let a742 = {
                    let v4: u8 = v3;
                    let a740 = {
                        let a716 = {
                            let a704 = {
                                let a698 = {
                                    let a696 = v4;
                                    let a697 = 128u8;
                                    fixed_u8::bit_and(a696, a697)
                                };
                                let a699 = 128u8;
                                if equal(a698, a699) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a705 = {
                                let a702 = {
                                    let a700 = v4;
                                    let a701 = 64u8;
                                    fixed_u8::bit_and(a700, a701)
                                };
                                let a703 = 64u8;
                                if equal(a702, a703) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a704, a705)?
                        };
                        let a717 = {
                            let a714 = {
                                let a708 = {
                                    let a706 = v4;
                                    let a707 = 32u8;
                                    fixed_u8::bit_and(a706, a707)
                                };
                                let a709 = 32u8;
                                if equal(a708, a709) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a715 = {
                                let a712 = {
                                    let a710 = v4;
                                    let a711 = 16u8;
                                    fixed_u8::bit_and(a710, a711)
                                };
                                let a713 = 16u8;
                                if equal(a712, a713) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a714, a715)?
                        };
                        nat_add(a716, a717)?
                    };
                    let a741 = {
                        let a738 = {
                            let a726 = {
                                let a720 = {
                                    let a718 = v4;
                                    let a719 = 8u8;
                                    fixed_u8::bit_and(a718, a719)
                                };
                                let a721 = 8u8;
                                if equal(a720, a721) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a727 = {
                                let a724 = {
                                    let a722 = v4;
                                    let a723 = 4u8;
                                    fixed_u8::bit_and(a722, a723)
                                };
                                let a725 = 4u8;
                                if equal(a724, a725) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a726, a727)?
                        };
                        let a739 = {
                            let a736 = {
                                let a730 = {
                                    let a728 = v4;
                                    let a729 = 2u8;
                                    fixed_u8::bit_and(a728, a729)
                                };
                                let a731 = 2u8;
                                if equal(a730, a731) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a737 = {
                                let a734 = {
                                    let a732 = v4;
                                    let a733 = 1u8;
                                    fixed_u8::bit_and(a732, a733)
                                };
                                let a735 = 1u8;
                                if equal(a734, a735) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a736, a737)?
                        };
                        nat_add(a738, a739)?
                    };
                    nat_add(a740, a741)?
                };
                let a743 = 47u64;
                if nat_eq(a742, a743) {
                    let a744 = v2;
                    let a745 = 1u64;
                    nat_add(a744, a745)?
                } else {
                    match v1 {
                        Adt10::C0(_, h746) => {
                            h746
                        }
                    }
                }
            })))
        }
    }
}

pub fn f91(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: Bytes) -> Adt2 {
    f128(v2, f16(v0, v1, v3), v4)
}

pub fn f92(v0: Adt2, v1: u64, v2: u64, v3: Adt4, v4: u64) -> Adt2 {
    Adt2::C0(v1, match v0.clone() {
        Adt2::C0(_, h747, _, _, _, _, _, _, _) => {
            h747
        }
    }, std::rc::Rc::new(Adt0::C5), 0u64, v2, std::rc::Rc::new(v3), v4, None::<Adt1>, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h748) => {
            h748
        }
    })
}

pub fn f93(v0: List<Adt5>, v1: u64) -> u64 {
    f149(v0, v1)
}

pub fn f94(v0: List<Adt5>, v1: u64, v2: u64) -> R<Adt12> {
    let v3: Adt12 = f120(v0.clone(), v1, v2, f15(Str::lit("a name")))?;
    let m750 = match v3.clone() {
        Adt12::C0(_, _, h749) => {
            h749
        }
    };
    match m750 {
        None => {
            let v5: u64 = match v3.clone() {
                Adt12::C0(_, h751, _) => {
                    h751
                }
            };
            let a752 = f93(v0.clone(), v5);
            let a753 = 9u64;
            if nat_eq(a752, a753) {
                let v6: Adt12 = f120(v0, v1, {
                    let a754 = v5;
                    let a755 = 1u64;
                    nat_add(a754, a755)?
                }, f15(Str::lit("a name after `::`")))?;
                let m757 = match v6.clone() {
                    Adt12::C0(_, _, h756) => {
                        h756
                    }
                };
                match m757 {
                    None => {
                        Ok(Adt12::C0(std::rc::Rc::new(f129(f115(Str::lit("module"), match v3 {
                            Adt12::C0(h758, _, _) => {
                                (*h758).clone()
                            }
                        }, Str::lit("name"), match v6.clone() {
                            Adt12::C0(h759, _, _) => {
                                (*h759).clone()
                            }
                        }))?), match v6 {
                            Adt12::C0(_, h760, _) => {
                                h760
                            }
                        }, None::<Adt1>))
                    }
                    Some(_) => {
                        Ok(v6)
                    }
                }
            } else {
                Ok(Adt12::C0(std::rc::Rc::new(f129(f111(Str::lit("name"), match v3 {
                    Adt12::C0(h761, _, _) => {
                        (*h761).clone()
                    }
                }))?), v5, None::<Adt1>))
            }
        }
        Some(_) => {
            Ok(v3)
        }
    }
}

pub fn f95(v0: Adt2, v1: Option<Adt1>) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(h762, _, _, _, _, _, _, _, _) => {
            h762
        }
    }, match v0.clone() {
        Adt2::C0(_, h763, _, _, _, _, _, _, _) => {
            h763
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, h764, _, _, _, _, _, _) => {
            (*h764).clone()
        }
    }), 9u64, match v0.clone() {
        Adt2::C0(_, _, _, _, h765, _, _, _, _) => {
            h765
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h766, _, _, _) => {
            (*h766).clone()
        }
    }), match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h767, _, _) => {
            h767
        }
    }, v1, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h768) => {
            h768
        }
    })
}

pub fn f96(v0: Adt2, v1: u64, v2: Adt0) -> Adt2 {
    Adt2::C0(v1, match v0.clone() {
        Adt2::C0(_, h769, _, _, _, _, _, _, _) => {
            h769
        }
    }, std::rc::Rc::new(v2), 1u64, match v0.clone() {
        Adt2::C0(_, _, _, _, h770, _, _, _, _) => {
            h770
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h771, _, _, _) => {
            (*h771).clone()
        }
    }), match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h772, _, _) => {
            h772
        }
    }, None::<Adt1>, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h773) => {
            h773
        }
    })
}

pub fn f97(v0: Adt0, v1: Adt4) -> R<Adt4> {
    let m774 = v1;
    match m774 {
        Adt4::C0 => {
            Ok(Adt4::C5)
        }
        Adt4::C1 => {
            Ok(Adt4::C5)
        }
        Adt4::C2 => {
            Ok(Adt4::C5)
        }
        Adt4::C3 => {
            Ok(Adt4::C5)
        }
        Adt4::C4(r775) => {
            let v2 = (*r775).clone();
            f23(v0, v2)
        }
        Adt4::C5 => {
            Ok(Adt4::C5)
        }
    }
}

pub fn f98(v0: Adt2, v1: Adt3, v2: u64, v3: u64, v4: Adt4, v5: u64) -> Adt2 {
    Adt2::C0(v2, List::cons(v1, match v0.clone() {
        Adt2::C0(_, h776, _, _, _, _, _, _, _) => {
            h776
        }
    }), std::rc::Rc::new(Adt0::C5), 0u64, v3, std::rc::Rc::new(v4), v5, None::<Adt1>, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h777) => {
            h777
        }
    })
}

pub fn f99(v0: List<Adt5>, v1: u64, v2: Adt0, v3: Adt2, v4: Adt3, v5: u64) -> R<Adt2> {
    let v6: u64 = f93(v0.clone(), v5);
    let a778 = v6;
    let a779 = 1u64;
    if nat_eq(a778, a779) {
        Ok(f96(v3, {
            let a780 = v5;
            let a781 = 1u64;
            nat_add(a780, a781)?
        }, f129(match v4 {
            Adt3::C0(_, _, _, _, _, _, _, _, h782, _, _, _) => {
                h782
            }
        })?))
    } else {
        let v7: u64 = f16(v0.clone(), v1, v5);
        let v8: Bytes = f101(v0.clone(), v5);
        let a783 = v6;
        let a784 = 11u64;
        if if nat_eq(a783, a784) {
            true
        } else {
            f102({
                let a785 = v6;
                let a786 = 12u64;
                nat_eq(a785, a786)
            }, f103(v8.clone())?)
        } {
            let a789 = f93(v0.clone(), {
                let a787 = v5;
                let a788 = 1u64;
                nat_add(a787, a788)?
            });
            let a790 = 7u64;
            if nat_eq(a789, a790) {
                Ok(f98(v3, f11(v4.clone(), v8.clone(), v7), {
                    let a791 = v5;
                    let a792 = 2u64;
                    nat_add(a791, a792)?
                }, 0u64, f150(v2, match v4.clone() {
                    Adt3::C0(_, _, _, _, _, _, h793, _, _, _, _, _) => {
                        h793
                    }
                }, v8)?, {
                    let a795 = match v4 {
                        Adt3::C0(_, _, h794, _, _, _, _, _, _, _, _, _) => {
                            h794
                        }
                    };
                    let a796 = 1u64;
                    nat_add(a795, a796)?
                }))
            } else {
                Ok(f91(v0, v1, v3, {
                    let a797 = v5;
                    let a798 = 1u64;
                    nat_add(a797, a798)?
                }, f15(Str::lit("expected `:` after a member name"))))
            }
        } else {
            Ok(f128(v3, v7, f15(Str::lit("expected a member name"))))
        }
    }
}

pub fn f100(v0: Adt0, v1: Adt4) -> Option<Adt0> {
    let m799 = v1;
    match m799 {
        Adt4::C0 => {
            None::<Adt0>
        }
        Adt4::C1 => {
            None::<Adt0>
        }
        Adt4::C2 => {
            None::<Adt0>
        }
        Adt4::C3 => {
            f24(v0, f15(Str::lit("parameter")))
        }
        Adt4::C4(r800) => {
            let v2 = (*r800).clone();
            Some(v2)
        }
        Adt4::C5 => {
            None::<Adt0>
        }
    }
}

pub fn f101(v0: List<Adt5>, v1: u64) -> Bytes {
    f151(v0, v1)
}

pub fn f102(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f103(v0: Bytes) -> R<bool> {
    let a802 = {
        let a801 = v0.clone();
        length_bytes(a801)
    };
    let a803 = 0u64;
    if nat_eq(a802, a803) {
        Ok(false)
    } else {
        if f152(f153(v0.clone(), 0u64)?) {
            let a805 = f154(v0.clone(), 1u64)?;
            let a806 = {
                let a804 = v0.clone();
                length_bytes(a804)
            };
            if nat_eq(a805, a806) {
                let a807 = f155(v0);
                Ok(bool_not(a807))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f104(v0: Bytes) -> R<bool> {
    Ok(f102({
        let a809 = 0u64;
        let a810 = {
            let a808 = v0.clone();
            length_bytes(a808)
        };
        nat_lt(a809, a810)
    }, {
        let a812 = f156(v0.clone(), 0u64)?;
        let a813 = {
            let a811 = v0;
            length_bytes(a811)
        };
        nat_eq(a812, a813)
    }))
}

pub fn f105(v0: Adt0, v1: Adt4, v2: Bytes) -> R<Option<Adt0>> {
    let m814 = v1;
    match m814 {
        Adt4::C0 => {
            let m815 = f24(v0.clone(), f15(Str::lit("term")));
            match m815 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v3) => {
                    f157(v0, v3, v2)
                }
            }
        }
        Adt4::C1 => {
            let m816 = f24(v0.clone(), f15(Str::lit("type")));
            match m816 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    f157(v0, v4, v2)
                }
            }
        }
        Adt4::C2 => {
            Ok(None::<Adt0>)
        }
        Adt4::C3 => {
            Ok(None::<Adt0>)
        }
        Adt4::C4(r817) => {
            let v5 = (*r817).clone();
            if f158(v0.clone(), v5.clone())? {
                f157(v0, v5, v2)
            } else {
                Ok(Some(v5))
            }
        }
        Adt4::C5 => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f106(v0: Bytes) -> Adt0 {
    Adt0::C2(v0)
}

pub fn f107(v0: Bytes) -> Option<Bytes> {
    if f57(v0.clone(), f15(Str::lit("Bool"))) {
        Some(f15(Str::lit("bool")))
    } else {
        if f57(v0.clone(), f15(Str::lit("Bytes"))) {
            Some(f15(Str::lit("bytes")))
        } else {
            if f57(v0.clone(), f15(Str::lit("Int"))) {
                Some(f15(Str::lit("int")))
            } else {
                if f57(v0.clone(), f15(Str::lit("Int8"))) {
                    Some(f15(Str::lit("int8")))
                } else {
                    if f57(v0.clone(), f15(Str::lit("Int16"))) {
                        Some(f15(Str::lit("int16")))
                    } else {
                        if f57(v0.clone(), f15(Str::lit("Int32"))) {
                            Some(f15(Str::lit("int32")))
                        } else {
                            if f57(v0.clone(), f15(Str::lit("Int64"))) {
                                Some(f15(Str::lit("int64")))
                            } else {
                                if f57(v0.clone(), f15(Str::lit("Nat"))) {
                                    Some(f15(Str::lit("nat")))
                                } else {
                                    if f57(v0.clone(), f15(Str::lit("Ordering"))) {
                                        Some(f15(Str::lit("ordering")))
                                    } else {
                                        if f57(v0.clone(), f15(Str::lit("Prop"))) {
                                            Some(f15(Str::lit("prop")))
                                        } else {
                                            if f57(v0.clone(), f15(Str::lit("String"))) {
                                                Some(f15(Str::lit("string")))
                                            } else {
                                                if f57(v0.clone(), f15(Str::lit("Type"))) {
                                                    Some(f15(Str::lit("type")))
                                                } else {
                                                    if f57(v0.clone(), f15(Str::lit("UInt8"))) {
                                                        Some(f15(Str::lit("uint8")))
                                                    } else {
                                                        if f57(v0.clone(), f15(Str::lit("UInt16"))) {
                                                            Some(f15(Str::lit("uint16")))
                                                        } else {
                                                            if f57(v0.clone(), f15(Str::lit("UInt32"))) {
                                                                Some(f15(Str::lit("uint32")))
                                                            } else {
                                                                if f57(v0.clone(), f15(Str::lit("UInt64"))) {
                                                                    Some(f15(Str::lit("uint64")))
                                                                } else {
                                                                    if f57(v0.clone(), f15(Str::lit("Unit"))) {
                                                                        Some(f15(Str::lit("unit")))
                                                                    } else {
                                                                        if f57(v0.clone(), f15(Str::lit("ContractViolation"))) {
                                                                            Some(f15(Str::lit("contract_violation")))
                                                                        } else {
                                                                            if f57(v0, f15(Str::lit("ReasoningFailure"))) {
                                                                                Some(f15(Str::lit("reasoning_failure")))
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

pub fn f108(v0: Bytes, v1: List<(Bytes, Adt0)>) -> R<Adt0> {
    f129(List::cons((f15(Str::lit("kind")), Adt0::C2(v0)), v1))
}

pub fn f109(v0: Bytes, v1: Bytes) -> bool {
    let m822 = {
        let a819 = v0;
        let a820 = 0u64;
        let a821 = {
            let a818 = v1.clone();
            length_bytes(a818)
        };
        slice_bytes(a819, a820, a821)
    };
    match m822 {
        None => {
            false
        }
        Some(v2) => {
            let a823 = v2;
            let a824 = v1;
            equal(a823, a824)
        }
    }
}

pub fn f110(v0: Bytes, v1: u64) -> Bytes {
    f51(v0.clone(), v1, {
        let a826 = {
            let a825 = v0;
            length_bytes(a825)
        };
        let a827 = v1;
        nat_sub(a826, a827)
    })
}

pub fn f111(v0: Str, v1: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f15(v0), v1), List::<(Bytes, Adt0)>::nil())
}

pub fn f112(v0: Bytes, v1: List<Bytes>) -> bool {
    f159(Fn12::F160(v0), false, v1)
}

pub fn f113() -> List<Bytes> {
    List::cons(f15(Str::lit("List")), List::cons(f15(Str::lit("Option")), List::cons(f15(Str::lit("Result")), List::cons(f15(Str::lit("Set")), List::cons(f15(Str::lit("Map")), List::cons(f15(Str::lit("Fn")), List::<Bytes>::nil()))))))
}

pub fn f114(v0: Bytes, v1: Bytes) -> Bytes {
    let a828 = v0;
    let a829 = v1;
    append_bytes(a828, a829)
}

pub fn f115(v0: Str, v1: Adt0, v2: Str, v3: Adt0) -> List<(Bytes, Adt0)> {
    List::cons((f15(v0), v1), List::cons((f15(v2), v3), List::<(Bytes, Adt0)>::nil()))
}

pub fn f116(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: u64) -> R<Adt2> {
    let v5: Bytes = f101(v0.clone(), v3);
    let v6: bool = {
        let a830 = f93(v0.clone(), v3);
        let a831 = 12u64;
        nat_eq(a830, a831)
    };
    if f102(v6, if f57(v5.clone(), f15(Str::lit("List"))) {
        true
    } else {
        if f57(v5.clone(), f15(Str::lit("Set"))) {
            true
        } else {
            f57(v5.clone(), f15(Str::lit("Option")))
        }
    }) {
        Ok(f98(v2, f11(f14(12u64, v4), v5, 0u64), {
            let a832 = v3;
            let a833 = 1u64;
            nat_add(a832, a833)?
        }, 2u64, Adt4::C1, {
            let a834 = v4;
            let a835 = 1u64;
            nat_add(a834, a835)?
        }))
    } else {
        if f102(v6, if f57(v5.clone(), f15(Str::lit("Result"))) {
            true
        } else {
            f57(v5.clone(), f15(Str::lit("Map")))
        }) {
            Ok(f98(v2, f11(f14(13u64, v4), v5, 0u64), {
                let a836 = v3;
                let a837 = 1u64;
                nat_add(a836, a837)?
            }, 2u64, Adt4::C1, {
                let a838 = v4;
                let a839 = 1u64;
                nat_add(a838, a839)?
            }))
        } else {
            if f102(v6, f57(v5.clone(), f15(Str::lit("Fn")))) {
                let a842 = f93(v0.clone(), {
                    let a840 = v3;
                    let a841 = 1u64;
                    nat_add(a840, a841)?
                });
                let a843 = 4u64;
                if nat_eq(a842, a843) {
                    f131(v0, v1, v2, f14(14u64, v4), {
                        let a844 = v3;
                        let a845 = 2u64;
                        nat_add(a844, a845)?
                    })
                } else {
                    Ok(f91(v0, v1, v2, {
                        let a846 = v3;
                        let a847 = 1u64;
                        nat_add(a846, a847)?
                    }, f15(Str::lit("expected `(` before the parameter types"))))
                }
            } else {
                let v7: u64 = {
                    let a850 = f93(v0.clone(), {
                        let a848 = v3;
                        let a849 = 1u64;
                        nat_add(a848, a849)?
                    });
                    let a851 = 9u64;
                    if nat_eq(a850, a851) {
                        3u64
                    } else {
                        1u64
                    }
                };
                let v8: bool = f133(v0.clone(), {
                    let a852 = v3;
                    let a853 = v7;
                    nat_add(a852, a853)?
                }, f15(Str::lit("*")));
                if f102(v6, f102(!v8, f102(!f109(v5.clone(), f15(Str::lit("'"))), !f161(v5)))) {
                    let v9: Adt12 = f94(v0.clone(), v1, v3)?;
                    let m855 = match v9.clone() {
                        Adt12::C0(_, _, h854) => {
                            h854
                        }
                    };
                    match m855 {
                        None => {
                            f132(v0, v1, v2, f121(f14(16u64, v4), match v9.clone() {
                                Adt12::C0(h857, _, _) => {
                                    (*h857).clone()
                                }
                            }, Adt0::C5), match v9 {
                                Adt12::C0(_, h858, _) => {
                                    h858
                                }
                            })
                        }
                        Some(_) => {
                            Ok(f95(v2, match v9 {
                                Adt12::C0(_, _, h856) => {
                                    h856
                                }
                            }))
                        }
                    }
                } else {
                    Ok(f98(v2, f14(17u64, v4), v3, 2u64, Adt4::C1, {
                        let a859 = v4;
                        let a860 = 1u64;
                        nat_add(a859, a860)?
                    }))
                }
            }
        }
    }
}

pub fn f117(v0: List<Adt5>, v1: u64) -> R<Bytes> {
    let m861 = f28(v0, v1);
    match m861 {
        None => {
            Ok(f163(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f162(match v2 {
                Adt5::C0(h862, _, _) => {
                    (*h862).clone()
                }
            })
        }
    }
}

pub fn f118(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: u64) -> R<Adt2> {
    let v5: u64 = f93(v0.clone(), v3);
    let v6: Bytes = f101(v0.clone(), v3);
    let a863 = v5;
    let a864 = 5u64;
    if nat_eq(a863, a864) {
        Ok(f96(v2, v3, f108(f15(Str::lit("unit")), List::<(Bytes, Adt0)>::nil())?))
    } else {
        let a865 = v5;
        let a866 = 12u64;
        if nat_eq(a865, a866) {
            let v7: Bytes = if f109(v6.clone(), f15(Str::lit("-"))) {
                f110(v6.clone(), 1u64)
            } else {
                v6.clone()
            };
            if f102(f104(v7)?, {
                let a869 = f93(v0.clone(), {
                    let a867 = v3;
                    let a868 = 1u64;
                    nat_add(a867, a868)?
                });
                let a870 = 7u64;
                nat_eq(a869, a870)
            }) {
                f164(v0, v1, v2, v3, v6)
            } else {
                let v8: bool = f102(f57(v6.clone(), f15(Str::lit("let"))), f133(v0.clone(), {
                    let a871 = v3;
                    let a872 = 2u64;
                    nat_add(a871, a872)?
                }, f15(Str::lit("name"))));
                let v9: Bytes = if f102({
                    let a875 = f93(v0.clone(), {
                        let a873 = v3;
                        let a874 = 1u64;
                        nat_add(a873, a874)?
                    });
                    let a876 = 0u64;
                    nat_eq(a875, a876)
                }, !v8) {
                    f15(Str::lit(""))
                } else {
                    v6
                };
                f165(v0, v1, v2, v3, v4, v9)
            }
        } else {
            f134(v2, f14(47u64, v4), v3)
        }
    }
}

pub fn f119(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: Bytes) -> R<Adt2> {
    let v5: u64 = f16(v0, v1, v3);
    if f57(v4.clone(), f15(Str::lit("true"))) {
        Ok(f96(v2, {
            let a877 = v3;
            let a878 = 1u64;
            nat_add(a877, a878)?
        }, f108(f15(Str::lit("bool")), f111(Str::lit("value"), Adt0::C0(true)))?))
    } else {
        if f57(v4.clone(), f15(Str::lit("false"))) {
            Ok(f96(v2, {
                let a879 = v3;
                let a880 = 1u64;
                nat_add(a879, a880)?
            }, f108(f15(Str::lit("bool")), f111(Str::lit("value"), Adt0::C0(false)))?))
        } else {
            if f104(v4.clone())? {
                if f102({
                    let a882 = 1u64;
                    let a883 = {
                        let a881 = v4.clone();
                        length_bytes(a881)
                    };
                    nat_lt(a882, a883)
                }, f109(v4.clone(), f15(Str::lit("0")))) {
                    Ok(f128(v2, v5, f114(f166(v4), f15(Str::lit(" has a leading zero")))))
                } else {
                    Ok(f96(v2, {
                        let a884 = v3;
                        let a885 = 1u64;
                        nat_add(a884, a885)?
                    }, f108(f15(Str::lit("nat")), f111(Str::lit("value"), f106(v4)))?))
                }
            } else {
                if f109(v4.clone(), f15(Str::lit("0x"))) {
                    if f167(f110(v4.clone(), 2u64))? {
                        Ok(f96(v2, {
                            let a886 = v3;
                            let a887 = 1u64;
                            nat_add(a886, a887)?
                        }, f108(f15(Str::lit("bytes")), f111(Str::lit("hex"), f106(f110(v4, 2u64))))?))
                    } else {
                        Ok(f128(v2, v5, f15(Str::lit("a byte string is `0x` and an even number of lower-case hexadecimal digits"))))
                    }
                } else {
                    if f168(v4.clone())? {
                        Ok(f96(v2, {
                            let a888 = v3;
                            let a889 = 1u64;
                            nat_add(a888, a889)?
                        }, f108(f15(Str::lit("var")), f111(Str::lit("name"), f106(v4)))?))
                    } else {
                        Ok(f128(v2, v5, if f112(v4.clone(), f169()) {
                            f114(f114(f114(f166(v4.clone()), f15(Str::lit(" is a keyword; write `var{name: "))), v4), f15(Str::lit("}` for a variable of that name")))
                        } else {
                            f114(f166(v4), f15(Str::lit(" is not a term")))
                        }))
                    }
                }
            }
        }
    }
}

pub fn f120(v0: List<Adt5>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let v4: Bytes = f101(v0.clone(), v2);
    if f102({
        let a890 = f93(v0.clone(), v2);
        let a891 = 12u64;
        nat_eq(a890, a891)
    }, f103(v4.clone())?) {
        Ok(Adt12::C0(std::rc::Rc::new(f106(v4)), {
            let a892 = v2;
            let a893 = 1u64;
            nat_add(a892, a893)?
        }, None::<Adt1>))
    } else {
        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), v2, f170(f16(v0.clone(), v1, v2), f114(f114(f114(f15(Str::lit("expected ")), v3), f15(Str::lit(", found "))), f117(v0, v2)?))))
    }
}

pub fn f121(v0: Adt3, v1: Adt0, v2: Adt0) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h894, _, _, _, _, _, _, _, _, _, _, _) => {
            h894
        }
    }, match v0.clone() {
        Adt3::C0(_, h895, _, _, _, _, _, _, _, _, _, _) => {
            h895
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h896, _, _, _, _, _, _, _, _, _) => {
            h896
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h897, _, _, _, _, _, _, _, _) => {
            h897
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h898, _, _, _, _, _, _, _) => {
            h898
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h899, _, _, _, _, _, _) => {
            (*h899).clone()
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h900, _, _, _, _, _) => {
            h900
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h901, _, _, _, _) => {
            h901
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h902, _, _, _) => {
            h902
        }
    }, std::rc::Rc::new(v1), std::rc::Rc::new(v2), match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h903) => {
            h903
        }
    })
}

pub fn f122(v0: List<Adt5>, v1: u64, v2: u64, v3: u64) -> R<Adt13> {
    let (h904, _) = f171(Fn13::F172(v1, v0.clone()), {
        let a908 = {
            let a906 = {
                let a905 = v0;
                length_list(a905)
            };
            let a907 = 4u64;
            nat_mul(a906, a907)?
        };
        let a909 = 8u64;
        nat_add(a908, a909)?
    }, Adt13::C0(v2, v3, 0u64, List::<Adt14>::nil(), std::rc::Rc::new(Adt0::C5), None::<Adt1>))?;
    Ok(h904)
}

pub fn f123(v0: List<Adt5>, _: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a910 = f93(v0, v4);
    let a911 = 3u64;
    if nat_eq(a910, a911) {
        Ok(f96(v2, {
            let a912 = v4;
            let a913 = 1u64;
            nat_add(a912, a913)?
        }, f9(match v3 {
            Adt3::C0(_, _, _, _, _, _, _, h914, _, _, _, _) => {
                h914
            }
        })))
    } else {
        f130(v2, v3, v4)
    }
}

pub fn f124(v0: Adt3, v1: (u64, u64), v2: u64) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h915, _, _, _, _, _, _, _, _, _, _, _) => {
            h915
        }
    }, match v0.clone() {
        Adt3::C0(_, h916, _, _, _, _, _, _, _, _, _, _) => {
            h916
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h917, _, _, _, _, _, _, _, _, _) => {
            h917
        }
    }, v2, match v0.clone() {
        Adt3::C0(_, _, _, _, h918, _, _, _, _, _, _, _) => {
            h918
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h919, _, _, _, _, _, _) => {
            (*h919).clone()
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h920, _, _, _, _, _) => {
            h920
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h921, _, _, _, _) => {
            h921
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h922, _, _, _) => {
            h922
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h923, _, _) => {
            (*h923).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h924, _) => {
            (*h924).clone()
        }
    }), List::cons(v1, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h925) => {
            h925
        }
    }))
}

pub fn f125(v0: Adt3, v1: Adt0) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h926, _, _, _, _, _, _, _, _, _, _, _) => {
            h926
        }
    }, match v0.clone() {
        Adt3::C0(_, h927, _, _, _, _, _, _, _, _, _, _) => {
            h927
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h928, _, _, _, _, _, _, _, _, _) => {
            h928
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h929, _, _, _, _, _, _, _, _) => {
            h929
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h930, _, _, _, _, _, _, _) => {
            h930
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h931, _, _, _, _, _, _) => {
            (*h931).clone()
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h932, _, _, _, _, _) => {
            h932
        }
    }, List::cons(v1, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h933, _, _, _, _) => {
            h933
        }
    }), match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h934, _, _, _) => {
            h934
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h935, _, _) => {
            (*h935).clone()
        }
    }), std::rc::Rc::new(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h936, _) => {
            (*h936).clone()
        }
    }), match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h937) => {
            h937
        }
    })
}

pub fn f126(v0: List<Adt5>, v1: u64) -> u64 {
    let a938 = v1;
    let a939 = 0u64;
    if nat_eq(a938, a939) {
        0u64
    } else {
        let m942 = f28(v0, {
            let a940 = v1;
            let a941 = 1u64;
            nat_sub(a940, a941)
        });
        match m942 {
            None => {
                0u64
            }
            Some(v2) => {
                match v2 {
                    Adt5::C0(_, _, h943) => {
                        h943
                    }
                }
            }
        }
    }
}

pub fn f127(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f173(Fn14::F174(v1), false, v0)
}

pub fn f128(v0: Adt2, v1: u64, v2: Bytes) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(h944, _, _, _, _, _, _, _, _) => {
            h944
        }
    }, match v0.clone() {
        Adt2::C0(_, h945, _, _, _, _, _, _, _) => {
            h945
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, h946, _, _, _, _, _, _) => {
            (*h946).clone()
        }
    }), 9u64, match v0.clone() {
        Adt2::C0(_, _, _, _, h947, _, _, _, _) => {
            h947
        }
    }, std::rc::Rc::new(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h948, _, _, _) => {
            (*h948).clone()
        }
    }), match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h949, _, _) => {
            h949
        }
    }, f170(v1, v2), match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h950) => {
            h950
        }
    })
}

pub fn f129(v0: List<(Bytes, Adt0)>) -> R<Adt0> {
    Ok(Adt0::C4(std::rc::Rc::new(f175(f176(v0)?))))
}

pub fn f130(v0: Adt2, v1: Adt3, v2: u64) -> R<Adt2> {
    Ok(f98(v0, v1.clone(), v2, 2u64, Adt4::C1, {
        let a952 = match v1 {
            Adt3::C0(_, _, h951, _, _, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a953 = 1u64;
        nat_add(a952, a953)?
    }))
}

pub fn f131(v0: List<Adt5>, _: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a954 = f93(v0, v4);
    let a955 = 5u64;
    if nat_eq(a954, a955) {
        f130(v2, f12(v3, 1u64), {
            let a956 = v4;
            let a957 = 1u64;
            nat_add(a956, a957)?
        })
    } else {
        f130(v2, v3, v4)
    }
}

pub fn f132(v0: List<Adt5>, _: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a958 = f93(v0, v4);
    let a959 = 5u64;
    if nat_eq(a958, a959) {
        Ok(f96(v2, v4, f108(f15(Str::lit("named")), f115(Str::lit("member"), match v3.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h960, _, _) => {
                (*h960).clone()
            }
        }, Str::lit("arguments"), f9(match v3 {
            Adt3::C0(_, _, _, _, _, _, _, h961, _, _, _, _) => {
                h961
            }
        })))?))
    } else {
        f130(v2, v3, v4)
    }
}

pub fn f133(v0: List<Adt5>, v1: u64, v2: Bytes) -> bool {
    f102({
        let a962 = f93(v0.clone(), v1);
        let a963 = 12u64;
        nat_eq(a962, a963)
    }, f57(f101(v0, v1), v2))
}

pub fn f134(v0: Adt2, v1: Adt3, v2: u64) -> R<Adt2> {
    Ok(f98(v0, v1.clone(), v2, 3u64, Adt4::C0, {
        let a965 = match v1 {
            Adt3::C0(_, _, h964, _, _, _, _, _, _, _, _, _) => {
                h964
            }
        };
        let a966 = 1u64;
        nat_add(a965, a966)?
    }))
}

pub fn f135(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a967 = f93(v0.clone(), v4);
    let a968 = 4u64;
    if nat_eq(a967, a968) {
        f177(v2, v3, {
            let a969 = v4;
            let a970 = 1u64;
            nat_add(a969, a970)?
        })
    } else {
        let m972 = match v3.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h971, _, _, _, _) => {
                h971
            }
        };
        match m972.uncons() {
            None => {
                Ok(f91(v0, v1, v2, v4, f15(Str::lit("a function takes at least one parameter"))))
            }
            Some((_, _)) => {
                if f133(v0.clone(), v4, f15(Str::lit("=>"))) {
                    f134(v2, f12(v3, 1u64), {
                        let a973 = v4;
                        let a974 = 1u64;
                        nat_add(a973, a974)?
                    })
                } else {
                    Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `=>`"))))
                }
            }
        }
    }
}

pub fn f136(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let v5: Bytes = match v3.clone() {
        Adt3::C0(_, _, _, _, h975, _, _, _, _, _, _, _) => {
            h975
        }
    };
    let a977 = {
        let a976 = v5.clone();
        length_bytes(a976)
    };
    let a978 = 0u64;
    if if nat_eq(a977, a978) {
        let a979 = f93(v0.clone(), v4);
        let a980 = 5u64;
        nat_eq(a979, a980)
    } else {
        f133(v0.clone(), v4, v5)
    } {
        f178(v0, v1, v2, v3, v4)
    } else {
        let a981 = f93(v0, v4);
        let a982 = 99u64;
        if nat_eq(a981, a982) {
            Ok(f128(v2, v1, f15(Str::lit("expected `)`"))))
        } else {
            f134(v2, v3, v4)
        }
    }
}

pub fn f137(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3) -> R<Adt2> {
    let v4: u64 = match v2.clone() {
        Adt2::C0(h983, _, _, _, _, _, _, _, _) => {
            h983
        }
    };
    let v5: u64 = match v3.clone() {
        Adt3::C0(h984, _, _, _, _, _, _, _, _, _, _, _) => {
            h984
        }
    };
    let v6: Adt3 = f121(v3.clone(), match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h985, _, _) => {
            (*h985).clone()
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h986, _, _, _, _, _, _) => {
            (*h986).clone()
        }
    });
    let a987 = v5;
    let a988 = 41u64;
    if nat_eq(a987, a988) {
        Ok(f96(v2.clone(), v4, f108(f15(Str::lit("function_ref")), f179(f111(Str::lit("function"), match v3 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h989, _, _) => {
                (*h989).clone()
            }
        }), match v2 {
            Adt2::C0(_, _, h990, _, _, _, _, _, _) => {
                (*h990).clone()
            }
        }))?))
    } else {
        let a991 = v5;
        let a992 = 42u64;
        if nat_eq(a991, a992) {
            let a993 = f93(v0.clone(), v4);
            let a994 = 0u64;
            if nat_eq(a993, a994) {
                f138(v0, v1, v2, f12(v6, 1u64), {
                    let a995 = v4;
                    let a996 = 1u64;
                    nat_add(a995, a996)?
                })
            } else {
                Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `{` before the fields"))))
            }
        } else {
            f136(v0, v1, v2, f12(v6, 1u64), v4)
        }
    }
}

pub fn f138(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a997 = f93(v0.clone(), v4);
    let a998 = 1u64;
    if nat_eq(a997, a998) {
        Ok(f96(v2, {
            let a999 = v4;
            let a1000 = 1u64;
            nat_add(a999, a1000)?
        }, f108(f15(Str::lit("record")), f179(f115(Str::lit("type"), match v3.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1001, _, _) => {
                (*h1001).clone()
            }
        }, Str::lit("fields"), f9(match v3.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1002, _, _, _, _) => {
                h1002
            }
        })), match v3 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1003, _) => {
                (*h1003).clone()
            }
        }))?))
    } else {
        let v5: Adt12 = f120(v0.clone(), v1, v4, f15(Str::lit("a field name")))?;
        let m1005 = match v5.clone() {
            Adt12::C0(_, _, h1004) => {
                h1004
            }
        };
        match m1005 {
            None => {
                let a1008 = f93(v0.clone(), match v5.clone() {
                    Adt12::C0(_, h1007, _) => {
                        h1007
                    }
                });
                let a1009 = 8u64;
                if nat_eq(a1008, a1009) {
                    f134(v2, f11(v3, {
                        let m1011 = f69(match v5.clone() {
                            Adt12::C0(h1010, _, _) => {
                                (*h1010).clone()
                            }
                        });
                        match m1011 {
                            None => {
                                f15(Str::lit(""))
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, 0u64), {
                        let a1013 = match v5 {
                            Adt12::C0(_, h1012, _) => {
                                h1012
                            }
                        };
                        let a1014 = 1u64;
                        nat_add(a1013, a1014)?
                    })
                } else {
                    Ok(f91(v0, v1, v2, match v5 {
                        Adt12::C0(_, h1015, _) => {
                            h1015
                        }
                    }, f15(Str::lit("expected `:=` after a field name"))))
                }
            }
            Some(_) => {
                Ok(f95(v2, match v5 {
                    Adt12::C0(_, _, h1006) => {
                        h1006
                    }
                }))
            }
        }
    }
}

pub fn f139(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let v5: u64 = f93(v0.clone(), v4);
    let a1016 = v5;
    let a1017 = 7u64;
    if nat_eq(a1016, a1017) {
        f130(v2, f12(v3, 1u64), {
            let a1018 = v4;
            let a1019 = 1u64;
            nat_add(a1018, a1019)?
        })
    } else {
        let a1020 = v5;
        let a1021 = 99u64;
        if if nat_eq(a1020, a1021) {
            true
        } else {
            let a1022 = v5;
            let a1023 = 5u64;
            nat_eq(a1022, a1023)
        } {
            Ok(f91(v0, v1, v2, v4, f15(Str::lit("expected `:` and the result type"))))
        } else {
            f134(v2, v3, v4)
        }
    }
}

pub fn f140(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    if f133(v0.clone(), v4, f15(Str::lit("|"))) {
        let v5: Adt12 = f94(v0.clone(), v1, {
            let a1024 = v4;
            let a1025 = 1u64;
            nat_add(a1024, a1025)?
        })?;
        let m1027 = match v5.clone() {
            Adt12::C0(_, _, h1026) => {
                h1026
            }
        };
        match m1027 {
            None => {
                let v7: Adt15 = {
                    let (h1029, _) = f180(Fn15::F181(v1, v0.clone()), {
                        let a1031 = {
                            let a1030 = v0;
                            length_list(a1030)
                        };
                        let a1032 = 2u64;
                        nat_add(a1031, a1032)?
                    }, Adt15::C0(List::<Adt0>::nil(), match v5.clone() {
                        Adt12::C0(_, h1033, _) => {
                            h1033
                        }
                    }, true, None::<Adt1>))?;
                    h1029
                };
                let m1035 = match v7.clone() {
                    Adt15::C0(_, _, _, h1034) => {
                        h1034
                    }
                };
                match m1035 {
                    None => {
                        f134(v2, f13(f12(v3.clone(), 1u64), match v3.clone() {
                            Adt3::C0(_, _, _, _, _, h1037, _, _, _, _, _, _) => {
                                (*h1037).clone()
                            }
                        }, match v3 {
                            Adt3::C0(_, _, _, _, _, _, h1038, _, _, _, _, _) => {
                                h1038
                            }
                        }, f115(Str::lit("constructor"), match v5 {
                            Adt12::C0(h1039, _, _) => {
                                (*h1039).clone()
                            }
                        }, Str::lit("binders"), f9(match v7.clone() {
                            Adt15::C0(h1040, _, _, _) => {
                                h1040
                            }
                        }))), {
                            let a1042 = match v7 {
                                Adt15::C0(_, h1041, _, _) => {
                                    h1041
                                }
                            };
                            let a1043 = 1u64;
                            nat_add(a1042, a1043)?
                        })
                    }
                    Some(_) => {
                        Ok(f95(v2, match v7 {
                            Adt15::C0(_, _, _, h1036) => {
                                h1036
                            }
                        }))
                    }
                }
            }
            Some(_) => {
                Ok(f95(v2, match v5 {
                    Adt12::C0(_, _, h1028) => {
                        h1028
                    }
                }))
            }
        }
    } else {
        let m1045 = match v3.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1044, _, _, _, _) => {
                h1044
            }
        };
        match m1045.uncons() {
            None => {
                Ok(f91(v0, v1, v2, v4, f15(Str::lit("a match has at least one `| \u{2026}` branch"))))
            }
            Some((_, _)) => {
                Ok(f96(v2, v4, f108(f15(Str::lit("match")), f115(Str::lit("scrutinee"), match v3.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1046, _, _) => {
                        (*h1046).clone()
                    }
                }, Str::lit("branches"), f9(match v3 {
                    Adt3::C0(_, _, _, _, _, _, _, h1047, _, _, _, _) => {
                        h1047
                    }
                })))?))
            }
        }
    }
}

pub fn f141(v0: Bytes) -> Option<Bytes> {
    if f57(v0.clone(), f15(Str::lit("+"))) {
        Some(f15(Str::lit("add")))
    } else {
        if f57(v0.clone(), f15(Str::lit("="))) {
            Some(f15(Str::lit("eq")))
        } else {
            if f57(v0.clone(), f15(Str::lit("<="))) {
                Some(f15(Str::lit("le")))
            } else {
                if f57(v0.clone(), f15(Str::lit("<"))) {
                    Some(f15(Str::lit("lt")))
                } else {
                    if f57(v0.clone(), f15(Str::lit("=?"))) {
                        Some(f15(Str::lit("beq")))
                    } else {
                        if f57(v0.clone(), f15(Str::lit("<=?"))) {
                            Some(f15(Str::lit("ble")))
                        } else {
                            if f57(v0.clone(), f15(Str::lit("<?"))) {
                                Some(f15(Str::lit("blt")))
                            } else {
                                if f57(v0.clone(), f15(Str::lit("&&"))) {
                                    Some(f15(Str::lit("and")))
                                } else {
                                    if f57(v0.clone(), f15(Str::lit("||"))) {
                                        Some(f15(Str::lit("or")))
                                    } else {
                                        if f57(v0.clone(), f15(Str::lit("/\\"))) {
                                            Some(f15(Str::lit("prop_and")))
                                        } else {
                                            if f57(v0.clone(), f15(Str::lit("<->"))) {
                                                Some(f15(Str::lit("iff")))
                                            } else {
                                                if f57(v0, f15(Str::lit("->"))) {
                                                    Some(f15(Str::lit("implies")))
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

pub fn f142(v0: Bytes, v1: u64, v2: Adt9) -> R<Adt9> {
    let v3: u64 = match v2.clone() {
        Adt9::C0(h1048, _, _, _) => {
            h1048
        }
    };
    let m1051 = f45(v0.clone(), {
        let a1049 = v3;
        let a1050 = 1u64;
        nat_add(a1049, a1050)?
    });
    match m1051 {
        None => {
            Ok(Adt9::C0(v1, match v2 {
                Adt9::C0(_, h1052, _, _) => {
                    h1052
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a1053 = v4;
            let a1054 = 34u8;
            if equal(a1053, a1054) {
                Ok(f146(v2, {
                    let a1055 = v3;
                    let a1056 = 2u64;
                    nat_add(a1055, a1056)?
                }, f49(Str::lit("\""))))
            } else {
                let a1057 = v4;
                let a1058 = 92u8;
                if equal(a1057, a1058) {
                    Ok(f146(v2, {
                        let a1059 = v3;
                        let a1060 = 2u64;
                        nat_add(a1059, a1060)?
                    }, f49(Str::lit("\\"))))
                } else {
                    let a1061 = v4;
                    let a1062 = 47u8;
                    if equal(a1061, a1062) {
                        Ok(f146(v2, {
                            let a1063 = v3;
                            let a1064 = 2u64;
                            nat_add(a1063, a1064)?
                        }, f49(Str::lit("/"))))
                    } else {
                        let a1065 = v4;
                        let a1066 = 98u8;
                        if equal(a1065, a1066) {
                            Ok(f146(v2, {
                                let a1067 = v3;
                                let a1068 = 2u64;
                                nat_add(a1067, a1068)?
                            }, f182(8u64)))
                        } else {
                            let a1069 = v4;
                            let a1070 = 102u8;
                            if equal(a1069, a1070) {
                                Ok(f146(v2, {
                                    let a1071 = v3;
                                    let a1072 = 2u64;
                                    nat_add(a1071, a1072)?
                                }, f182(12u64)))
                            } else {
                                let a1073 = v4;
                                let a1074 = 110u8;
                                if equal(a1073, a1074) {
                                    Ok(f146(v2, {
                                        let a1075 = v3;
                                        let a1076 = 2u64;
                                        nat_add(a1075, a1076)?
                                    }, f182(10u64)))
                                } else {
                                    let a1077 = v4;
                                    let a1078 = 114u8;
                                    if equal(a1077, a1078) {
                                        Ok(f146(v2, {
                                            let a1079 = v3;
                                            let a1080 = 2u64;
                                            nat_add(a1079, a1080)?
                                        }, f182(13u64)))
                                    } else {
                                        let a1081 = v4;
                                        let a1082 = 116u8;
                                        if equal(a1081, a1082) {
                                            Ok(f146(v2, {
                                                let a1083 = v3;
                                                let a1084 = 2u64;
                                                nat_add(a1083, a1084)?
                                            }, f182(9u64)))
                                        } else {
                                            let a1085 = v4;
                                            let a1086 = 117u8;
                                            if equal(a1085, a1086) {
                                                f183(v0, v2)
                                            } else {
                                                Ok(f144(v2, v3, f184(f184(f49(Str::lit("`\\")), f51(v0.clone(), {
                                                    let a1087 = v3;
                                                    let a1088 = 1u64;
                                                    nat_add(a1087, a1088)?
                                                }, f185(v0, {
                                                    let a1089 = v3;
                                                    let a1090 = 1u64;
                                                    nat_add(a1089, a1090)?
                                                })?)), f49(Str::lit("` is not an escape")))))
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

pub fn f143(v0: Bytes, v1: u64) -> R<u64> {
    let m1091 = f45(v0, v1);
    match m1091 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1136 = {
                let a1112 = {
                    let a1100 = {
                        let a1094 = {
                            let a1092 = v3;
                            let a1093 = 128u8;
                            fixed_u8::bit_and(a1092, a1093)
                        };
                        let a1095 = 128u8;
                        if equal(a1094, a1095) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1101 = {
                        let a1098 = {
                            let a1096 = v3;
                            let a1097 = 64u8;
                            fixed_u8::bit_and(a1096, a1097)
                        };
                        let a1099 = 64u8;
                        if equal(a1098, a1099) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1100, a1101)?
                };
                let a1113 = {
                    let a1110 = {
                        let a1104 = {
                            let a1102 = v3;
                            let a1103 = 32u8;
                            fixed_u8::bit_and(a1102, a1103)
                        };
                        let a1105 = 32u8;
                        if equal(a1104, a1105) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1111 = {
                        let a1108 = {
                            let a1106 = v3;
                            let a1107 = 16u8;
                            fixed_u8::bit_and(a1106, a1107)
                        };
                        let a1109 = 16u8;
                        if equal(a1108, a1109) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1110, a1111)?
                };
                nat_add(a1112, a1113)?
            };
            let a1137 = {
                let a1134 = {
                    let a1122 = {
                        let a1116 = {
                            let a1114 = v3;
                            let a1115 = 8u8;
                            fixed_u8::bit_and(a1114, a1115)
                        };
                        let a1117 = 8u8;
                        if equal(a1116, a1117) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1123 = {
                        let a1120 = {
                            let a1118 = v3;
                            let a1119 = 4u8;
                            fixed_u8::bit_and(a1118, a1119)
                        };
                        let a1121 = 4u8;
                        if equal(a1120, a1121) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1122, a1123)?
                };
                let a1135 = {
                    let a1132 = {
                        let a1126 = {
                            let a1124 = v3;
                            let a1125 = 2u8;
                            fixed_u8::bit_and(a1124, a1125)
                        };
                        let a1127 = 2u8;
                        if equal(a1126, a1127) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1133 = {
                        let a1130 = {
                            let a1128 = v3;
                            let a1129 = 1u8;
                            fixed_u8::bit_and(a1128, a1129)
                        };
                        let a1131 = 1u8;
                        if equal(a1130, a1131) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1132, a1133)?
                };
                nat_add(a1134, a1135)?
            };
            nat_add(a1136, a1137)
        }
    }
}

pub fn f144(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(match v0.clone() {
        Adt9::C0(h1138, _, _, _) => {
            h1138
        }
    }, match v0 {
        Adt9::C0(_, h1139, _, _) => {
            h1139
        }
    }, false, Some(f48(v1, v2)))
}

pub fn f145(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1140, _) = f65(Fn8::F186(v1, v0), {
        let a1141 = v1;
        let a1142 = v2;
        nat_sub(a1141, a1142)
    }, v2)?;
    Ok(h1140)
}

pub fn f146(v0: Adt9, v1: u64, v2: Bytes) -> Adt9 {
    Adt9::C0(v1, List::cons(v2, match v0 {
        Adt9::C0(_, h1143, _, _) => {
            h1143
        }
    }), false, None::<Adt1>)
}

pub fn f147(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m1144 = v0.clone();
    match m1144.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m1145 = v2;
            match m1145.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f187(Fn16::F188, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m1147 = {
                        let (h1146, _) = v5;
                        h1146
                    };
                    match m1147 {
                        None => {
                            Some(f64({
                                let (_, h1149) = v5;
                                h1149
                            }))
                        }
                        Some(v6) => {
                            Some(f64(List::cons(v6, {
                                let (_, h1148) = v5;
                                h1148
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f148(v0: Bytes, v1: u64) -> R<bool> {
    let a1150 = 0u64;
    let a1151 = f29(v0.clone(), v1)?;
    if nat_lt(a1150, a1151) {
        Ok(true)
    } else {
        Ok(f189(v0, v1))
    }
}

pub fn f149(v0: List<Adt5>, v1: u64) -> u64 {
    let m1152 = f28(v0, v1);
    match m1152 {
        None => {
            99u64
        }
        Some(v2) => {
            f190(match v2 {
                Adt5::C0(h1153, _, _) => {
                    (*h1153).clone()
                }
            })
        }
    }
}

pub fn f150(v0: Adt0, v1: Option<Adt0>, v2: Bytes) -> R<Adt4> {
    let m1154 = v1;
    match m1154 {
        None => {
            Ok(Adt4::C5)
        }
        Some(v3) => {
            let m1155 = f22(f38(v0.clone(), v3)?, f15(Str::lit("properties")));
            match m1155 {
                None => {
                    Ok(Adt4::C5)
                }
                Some(v4) => {
                    let m1156 = f22(v4, v2);
                    match m1156 {
                        None => {
                            Ok(Adt4::C5)
                        }
                        Some(v5) => {
                            f39(v0, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f151(v0: List<Adt5>, v1: u64) -> Bytes {
    let m1157 = f28(v0, v1);
    match m1157 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f191(match v2 {
                Adt5::C0(h1158, _, _) => {
                    (*h1158).clone()
                }
            })
        }
    }
}

pub fn f152(v0: u64) -> bool {
    let a1159 = 64u64;
    let a1160 = v0;
    if if nat_lt(a1159, a1160) {
        let a1161 = v0;
        let a1162 = 91u64;
        nat_lt(a1161, a1162)
    } else {
        false
    } {
        true
    } else {
        let a1163 = 96u64;
        let a1164 = v0;
        if if nat_lt(a1163, a1164) {
            let a1165 = v0;
            let a1166 = 123u64;
            nat_lt(a1165, a1166)
        } else {
            false
        } {
            true
        } else {
            let a1167 = v0;
            let a1168 = 95u64;
            nat_eq(a1167, a1168)
        }
    }
}

pub fn f153(v0: Bytes, v1: u64) -> R<u64> {
    let m1169 = f45(v0, v1);
    match m1169 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1214 = {
                let a1190 = {
                    let a1178 = {
                        let a1172 = {
                            let a1170 = v3;
                            let a1171 = 128u8;
                            fixed_u8::bit_and(a1170, a1171)
                        };
                        let a1173 = 128u8;
                        if equal(a1172, a1173) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1179 = {
                        let a1176 = {
                            let a1174 = v3;
                            let a1175 = 64u8;
                            fixed_u8::bit_and(a1174, a1175)
                        };
                        let a1177 = 64u8;
                        if equal(a1176, a1177) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1178, a1179)?
                };
                let a1191 = {
                    let a1188 = {
                        let a1182 = {
                            let a1180 = v3;
                            let a1181 = 32u8;
                            fixed_u8::bit_and(a1180, a1181)
                        };
                        let a1183 = 32u8;
                        if equal(a1182, a1183) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1189 = {
                        let a1186 = {
                            let a1184 = v3;
                            let a1185 = 16u8;
                            fixed_u8::bit_and(a1184, a1185)
                        };
                        let a1187 = 16u8;
                        if equal(a1186, a1187) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1188, a1189)?
                };
                nat_add(a1190, a1191)?
            };
            let a1215 = {
                let a1212 = {
                    let a1200 = {
                        let a1194 = {
                            let a1192 = v3;
                            let a1193 = 8u8;
                            fixed_u8::bit_and(a1192, a1193)
                        };
                        let a1195 = 8u8;
                        if equal(a1194, a1195) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1201 = {
                        let a1198 = {
                            let a1196 = v3;
                            let a1197 = 4u8;
                            fixed_u8::bit_and(a1196, a1197)
                        };
                        let a1199 = 4u8;
                        if equal(a1198, a1199) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1200, a1201)?
                };
                let a1213 = {
                    let a1210 = {
                        let a1204 = {
                            let a1202 = v3;
                            let a1203 = 2u8;
                            fixed_u8::bit_and(a1202, a1203)
                        };
                        let a1205 = 2u8;
                        if equal(a1204, a1205) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1211 = {
                        let a1208 = {
                            let a1206 = v3;
                            let a1207 = 1u8;
                            fixed_u8::bit_and(a1206, a1207)
                        };
                        let a1209 = 1u8;
                        if equal(a1208, a1209) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1210, a1211)?
                };
                nat_add(a1212, a1213)?
            };
            nat_add(a1214, a1215)
        }
    }
}

pub fn f154(v0: Bytes, v1: u64) -> R<u64> {
    let (h1216, _) = f65(Fn8::F192(v0.clone()), {
        let a1218 = {
            let a1217 = v0;
            length_bytes(a1217)
        };
        let a1219 = 1u64;
        nat_add(a1218, a1219)?
    }, v1)?;
    Ok(h1216)
}

pub fn f155(v0: Bytes) -> bool {
    let a1220 = v0.clone();
    let a1221 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1220, a1221) {
        true
    } else {
        let a1222 = v0;
        let a1223 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a1222, a1223)
    }
}

pub fn f156(v0: Bytes, v1: u64) -> R<u64> {
    let (h1224, _) = f65(Fn8::F193(v0.clone()), {
        let a1226 = {
            let a1225 = v0;
            length_bytes(a1225)
        };
        let a1227 = 1u64;
        nat_add(a1226, a1227)?
    }, v1)?;
    Ok(h1224)
}

pub fn f157(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Option<Adt0>> {
    let m1228 = f194(f22(f38(v0.clone(), v1)?, f15(Str::lit("oneOf"))));
    match m1228 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v3) => {
            f195(Fn17::F196(v2, std::rc::Rc::new(v0)), None::<Adt0>, f197(v3))
        }
    }
}

pub fn f158(v0: Adt0, v1: Adt0) -> R<bool> {
    let m1229 = f194(f22(f38(v0.clone(), v1)?, f15(Str::lit("oneOf"))));
    match m1229 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            f198(Fn18::F199(std::rc::Rc::new(v0)), true, f197(v2))
        }
    }
}

pub fn f159(v0: Fn12, p1233: bool, p1234: List<Bytes>) -> bool {
    let m1232 = (p1233, p1234);
    {
        let mut m1232 = m1232;
        loop {
            let step = {
                let (v1, v2) = m1232;
                let m1230 = v2;
                match m1230.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1231: Fn12 = v0.clone();
                            c1231.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1235 = v2;
        let a1236 = v0;
        equal(a1235, a1236)
    }
}

pub fn f161(v0: Bytes) -> bool {
    let m1237 = f107(v0.clone());
    match m1237 {
        None => {
            f112(v0, f113())
        }
        Some(_) => {
            true
        }
    }
}

pub fn f162(v0: Adt6) -> R<Bytes> {
    let m1238 = v0;
    match m1238 {
        Adt6::C0 => {
            Ok(f163(Str::lit("`{`")))
        }
        Adt6::C1 => {
            Ok(f163(Str::lit("`}`")))
        }
        Adt6::C2 => {
            Ok(f163(Str::lit("`[`")))
        }
        Adt6::C3 => {
            Ok(f163(Str::lit("`]`")))
        }
        Adt6::C4 => {
            Ok(f163(Str::lit("`(`")))
        }
        Adt6::C5 => {
            Ok(f163(Str::lit("`)`")))
        }
        Adt6::C6 => {
            Ok(f163(Str::lit("`,`")))
        }
        Adt6::C7 => {
            Ok(f163(Str::lit("`:`")))
        }
        Adt6::C8 => {
            Ok(f163(Str::lit("`:=`")))
        }
        Adt6::C9 => {
            Ok(f163(Str::lit("`::`")))
        }
        Adt6::C10 => {
            Ok(f163(Str::lit("`;`")))
        }
        Adt6::C11(v1) => {
            Ok(f200(f163(Str::lit("the string ")), f201(v1)?))
        }
        Adt6::C12(v2) => {
            Ok(f200(f200(f163(Str::lit("`")), v2), f163(Str::lit("`"))))
        }
    }
}

pub fn f163(v0: Str) -> Bytes {
    let a1239 = v0;
    utf8_encode(a1239)
}

pub fn f164(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: Bytes) -> R<Adt2> {
    let v5: u64 = f16(v0.clone(), v1, v3);
    let v6: Bytes = if f109(v4.clone(), f15(Str::lit("-"))) {
        f110(v4.clone(), 1u64)
    } else {
        v4.clone()
    };
    let v7: Adt12 = f120(v0, v1, {
        let a1240 = v3;
        let a1241 = 2u64;
        nat_add(a1240, a1241)?
    }, f15(Str::lit("an integer type")))?;
    let m1243 = match v7.clone() {
        Adt12::C0(_, _, h1242) => {
            h1242
        }
    };
    match m1243 {
        None => {
            let v10: Bytes = {
                let m1246 = f69(match v7.clone() {
                    Adt12::C0(h1245, _, _) => {
                        (*h1245).clone()
                    }
                });
                match m1246 {
                    None => {
                        f15(Str::lit(""))
                    }
                    Some(v9) => {
                        v9
                    }
                }
            };
            let m1247 = f202(v10.clone());
            match m1247 {
                None => {
                    Ok(f128(v2, v5, f114(f166(v10), f15(Str::lit(" is not an integer type")))))
                }
                Some(v11) => {
                    if if f57(v4.clone(), f15(Str::lit("-0"))) {
                        true
                    } else {
                        f102({
                            let a1249 = 1u64;
                            let a1250 = {
                                let a1248 = v6.clone();
                                length_bytes(a1248)
                            };
                            nat_lt(a1249, a1250)
                        }, f109(v6, f15(Str::lit("0"))))
                    } {
                        Ok(f128(v2, v5, f114(f166(v4), f15(Str::lit(" is not a canonical integer")))))
                    } else {
                        Ok(f96(v2, match v7 {
                            Adt12::C0(_, h1251, _) => {
                                h1251
                            }
                        }, f108(f15(Str::lit("integer")), f115(Str::lit("representation"), f106(v11), Str::lit("value"), f106(v4)))?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(f95(v2, match v7 {
                Adt12::C0(_, _, h1244) => {
                    h1244
                }
            }))
        }
    }
}

pub fn f165(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: u64, v5: Bytes) -> R<Adt2> {
    let v6: u64 = {
        let a1252 = v3;
        let a1253 = 1u64;
        nat_add(a1252, a1253)?
    };
    if if f57(v5.clone(), f15(Str::lit("not"))) {
        true
    } else {
        if f57(v5.clone(), f15(Str::lit("first"))) {
            true
        } else {
            f57(v5.clone(), f15(Str::lit("second")))
        }
    } {
        f134(v2, f11(f14(31u64, v4), v5, 0u64), v6)
    } else {
        if f57(v5.clone(), f15(Str::lit("cons"))) {
            f134(v2, f14(32u64, v4), v6)
        } else {
            if f57(v5.clone(), f15(Str::lit("nil"))) {
                f130(v2, f14(33u64, v4), v6)
            } else {
                if f57(v5.clone(), f15(Str::lit("if"))) {
                    f134(v2, f14(34u64, v4), v6)
                } else {
                    if f57(v5.clone(), f15(Str::lit("let"))) {
                        f177(v2, f14(35u64, v4), v6)
                    } else {
                        if f57(v5.clone(), f15(Str::lit("forall"))) {
                            let a1254 = f93(v0.clone(), v6);
                            let a1255 = 4u64;
                            if nat_eq(a1254, a1255) {
                                f177(v2, f14(36u64, v4), {
                                    let a1256 = v6;
                                    let a1257 = 1u64;
                                    nat_add(a1256, a1257)?
                                })
                            } else {
                                Ok(f91(v0, v1, v2, v6, f15(Str::lit("expected `(` before the bound variable"))))
                            }
                        } else {
                            if f57(v5.clone(), f15(Str::lit("fun"))) {
                                let a1258 = f93(v0.clone(), v6);
                                let a1259 = 2u64;
                                if nat_eq(a1258, a1259) {
                                    let v7: Adt12 = f203(v0.clone(), v1, v6, f15(Str::lit("a name")))?;
                                    let m1261 = match v7.clone() {
                                        Adt12::C0(_, _, h1260) => {
                                            h1260
                                        }
                                    };
                                    match m1261 {
                                        None => {
                                            f135(v0, v1, v2, f121(f14(37u64, v4), match v7.clone() {
                                                Adt12::C0(h1263, _, _) => {
                                                    (*h1263).clone()
                                                }
                                            }, Adt0::C5), match v7 {
                                                Adt12::C0(_, h1264, _) => {
                                                    h1264
                                                }
                                            })
                                        }
                                        Some(_) => {
                                            Ok(f95(v2, match v7 {
                                                Adt12::C0(_, _, h1262) => {
                                                    h1262
                                                }
                                            }))
                                        }
                                    }
                                } else {
                                    f135(v0, v1, v2, f121(f14(37u64, v4), f9(List::<Adt0>::nil()), Adt0::C5), v6)
                                }
                            } else {
                                if f57(v5.clone(), f15(Str::lit("apply"))) {
                                    f134(v2, f14(38u64, v4), v6)
                                } else {
                                    if f57(v5.clone(), f15(Str::lit("project"))) {
                                        f134(v2, f14(39u64, v4), v6)
                                    } else {
                                        if if f57(v5.clone(), f15(Str::lit("ctor"))) {
                                            true
                                        } else {
                                            if f57(v5.clone(), f15(Str::lit("ref"))) {
                                                true
                                            } else {
                                                if f57(v5.clone(), f15(Str::lit("record"))) {
                                                    true
                                                } else {
                                                    f57(v5.clone(), f15(Str::lit("checked")))
                                                }
                                            }
                                        } {
                                            let v9: Adt12 = f94(v0.clone(), v1, v6)?;
                                            let m1266 = match v9.clone() {
                                                Adt12::C0(_, _, h1265) => {
                                                    h1265
                                                }
                                            };
                                            match m1266 {
                                                None => {
                                                    f204(v0, v1, v2, f11(f121(f14(if f57(v5.clone(), f15(Str::lit("ctor"))) {
                                                        40u64
                                                    } else {
                                                        if f57(v5.clone(), f15(Str::lit("ref"))) {
                                                            41u64
                                                        } else {
                                                            if f57(v5.clone(), f15(Str::lit("record"))) {
                                                                42u64
                                                            } else {
                                                                44u64
                                                            }
                                                        }
                                                    }, v4), match v9.clone() {
                                                        Adt12::C0(h1268, _, _) => {
                                                            (*h1268).clone()
                                                        }
                                                    }, Adt0::C5), if f57(v5, f15(Str::lit("checked"))) {
                                                        f15(Str::lit("with"))
                                                    } else {
                                                        f15(Str::lit(""))
                                                    }, 0u64), match v9 {
                                                        Adt12::C0(_, h1269, _) => {
                                                            h1269
                                                        }
                                                    })
                                                }
                                                Some(_) => {
                                                    Ok(f95(v2, match v9 {
                                                        Adt12::C0(_, _, h1267) => {
                                                            h1267
                                                        }
                                                    }))
                                                }
                                            }
                                        } else {
                                            if f57(v5.clone(), f15(Str::lit("prim"))) {
                                                let v11: Adt12 = f120(v0.clone(), v1, v6, f15(Str::lit("a primitive operation")))?;
                                                let m1271 = match v11.clone() {
                                                    Adt12::C0(_, _, h1270) => {
                                                        h1270
                                                    }
                                                };
                                                match m1271 {
                                                    None => {
                                                        f139(v0, v1, v2, f121(f14(43u64, v4), match v11.clone() {
                                                            Adt12::C0(h1273, _, _) => {
                                                                (*h1273).clone()
                                                            }
                                                        }, Adt0::C5), match v11 {
                                                            Adt12::C0(_, h1274, _) => {
                                                                h1274
                                                            }
                                                        })
                                                    }
                                                    Some(_) => {
                                                        Ok(f95(v2, match v11 {
                                                            Adt12::C0(_, _, h1272) => {
                                                                h1272
                                                            }
                                                        }))
                                                    }
                                                }
                                            } else {
                                                if f57(v5, f15(Str::lit("match"))) {
                                                    f134(v2, f14(45u64, v4), v6)
                                                } else {
                                                    f205(v0, v1, v2, v3, v4)
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

pub fn f166(v0: Bytes) -> Bytes {
    f114(f114(f15(Str::lit("`")), v0), f15(Str::lit("`")))
}

pub fn f167(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let (h1275, _) = f65(Fn8::F206(v0.clone()), {
            let a1277 = {
                let a1276 = v0.clone();
                length_bytes(a1276)
            };
            let a1278 = 1u64;
            nat_add(a1277, a1278)?
        }, 0u64)?;
        h1275
    };
    Ok(f102({
        let a1280 = v1;
        let a1281 = {
            let a1279 = v0.clone();
            length_bytes(a1279)
        };
        nat_eq(a1280, a1281)
    }, {
        let a1286 = {
            let a1283 = {
                let a1282 = v0;
                length_bytes(a1282)
            };
            let a1284 = 2u64;
            let a1285 = 0u64;
            nat_rem(a1283, a1284, a1285)
        };
        let a1287 = 0u64;
        nat_eq(a1286, a1287)
    }))
}

pub fn f168(v0: Bytes) -> R<bool> {
    Ok(f102(f103(v0.clone())?, f102(!f112(v0.clone(), f169()), !f57(v0, f15(Str::lit("false"))))))
}

pub fn f169() -> List<Bytes> {
    List::cons(f15(Str::lit("let")), List::cons(f15(Str::lit("if")), List::cons(f15(Str::lit("match")), List::cons(f15(Str::lit("fun")), List::cons(f15(Str::lit("forall")), List::cons(f15(Str::lit("nil")), List::cons(f15(Str::lit("cons")), List::cons(f15(Str::lit("first")), List::cons(f15(Str::lit("second")), List::cons(f15(Str::lit("not")), List::cons(f15(Str::lit("apply")), List::cons(f15(Str::lit("ctor")), List::cons(f15(Str::lit("record")), List::cons(f15(Str::lit("project")), List::cons(f15(Str::lit("prim")), List::cons(f15(Str::lit("ref")), List::cons(f15(Str::lit("checked")), List::cons(f15(Str::lit("then")), List::cons(f15(Str::lit("else")), List::cons(f15(Str::lit("with")), List::cons(f15(Str::lit("true")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f170(v0: u64, v1: Bytes) -> Option<Adt1> {
    Some(Adt1::C0(v0, v1))
}

pub fn f171(v0: Fn13, p1292: u64, p1293: Adt13) -> R<(Adt13, bool)> {
    let m1291 = (p1292, p1293);
    {
        let mut m1291 = m1291;
        loop {
            let step = {
                let (v1, v2) = m1291;
                let m1288 = v1;
                if m1288 == 0 {
                    Err::<(u64, Adt13), R<(Adt13, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1288 - 1;
                    let m1290 = {
                        let c1289: Fn13 = v0.clone();
                        c1289.apply(v2.clone())?
                    };
                    match m1290 {
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
                Ok(next) => { m1291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: u64, v1: List<Adt5>, v2: Adt13) -> R<Option<Adt13>> {
    f207(v1, v0, v2)
}

pub fn f173(v0: Fn14, p1297: bool, p1298: List<(Bytes, Adt0)>) -> bool {
    let m1296 = (p1297, p1298);
    {
        let mut m1296 = m1296;
        loop {
            let step = {
                let (v1, v2) = m1296;
                let m1294 = v2;
                match m1294.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c1295: Fn14 = v0.clone();
                            c1295.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a1300 = {
            let (h1299, _) = v2;
            h1299
        };
        let a1301 = v0;
        equal(a1300, a1301)
    }
}

pub fn f175(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f208(Fn19::F209, Adt0::C5, f210(v0))
}

pub fn f176(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f211(f212(Fn20::F213, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m1306 = {
        let (h1302, _) = f214(Fn21::F215, {
            let a1304 = {
                let a1303 = v0;
                length_list(a1303)
            };
            let a1305 = 1u64;
            nat_add(a1304, a1305)?
        }, v1)?;
        h1302
    };
    match m1306.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f177(v0: Adt2, v1: Adt3, v2: u64) -> R<Adt2> {
    Ok(f98(v0, v1.clone(), v2, 4u64, Adt4::C3, {
        let a1308 = match v1 {
            Adt3::C0(_, _, h1307, _, _, _, _, _, _, _, _, _) => {
                h1307
            }
        };
        let a1309 = 1u64;
        nat_add(a1308, a1309)?
    }))
}

pub fn f178(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let v5: u64 = match v3.clone() {
        Adt3::C0(h1310, _, _, _, _, _, _, _, _, _, _, _) => {
            h1310
        }
    };
    let v6: Adt0 = f9(match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1311, _, _, _, _) => {
            h1311
        }
    });
    let a1312 = v5;
    let a1313 = 38u64;
    if nat_eq(a1312, a1313) {
        Ok(f96(v2, v4, f108(f15(Str::lit("apply")), f115(Str::lit("function"), match v3 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1314, _, _) => {
                (*h1314).clone()
            }
        }, Str::lit("arguments"), v6))?))
    } else {
        let a1315 = v5;
        let a1316 = 40u64;
        if nat_eq(a1315, a1316) {
            Ok(f96(v2, v4, f108(f15(Str::lit("constructor")), f179(f115(Str::lit("constructor"), match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1317, _, _) => {
                    (*h1317).clone()
                }
            }, Str::lit("arguments"), v6), match v3 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1318, _) => {
                    (*h1318).clone()
                }
            }))?))
        } else {
            let a1319 = v5;
            let a1320 = 46u64;
            if nat_eq(a1319, a1320) {
                Ok(f96(v2, v4, f108(f15(Str::lit("call")), f179(f115(Str::lit("function"), match v3.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1321, _, _) => {
                        (*h1321).clone()
                    }
                }, Str::lit("arguments"), v6), match v3 {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1322, _) => {
                        (*h1322).clone()
                    }
                }))?))
            } else {
                let v7: Adt12 = f203(v0, v1, {
                    let a1323 = v4;
                    let a1324 = 1u64;
                    nat_add(a1323, a1324)?
                }, f15(Str::lit("a check")))?;
                let m1326 = match v7.clone() {
                    Adt12::C0(_, _, h1325) => {
                        h1325
                    }
                };
                match m1326 {
                    None => {
                        Ok(f96(v2, match v7.clone() {
                            Adt12::C0(_, h1328, _) => {
                                h1328
                            }
                        }, f108(f15(Str::lit("checked_apply")), f179(List::cons((f15(Str::lit("checks")), match v7 {
                            Adt12::C0(h1329, _, _) => {
                                (*h1329).clone()
                            }
                        }), f115(Str::lit("model"), match v3.clone() {
                            Adt3::C0(_, _, _, _, _, _, _, _, _, h1330, _, _) => {
                                (*h1330).clone()
                            }
                        }, Str::lit("arguments"), v6)), match v3 {
                            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1331, _) => {
                                (*h1331).clone()
                            }
                        }))?))
                    }
                    Some(_) => {
                        Ok(f95(v2, match v7 {
                            Adt12::C0(_, _, h1327) => {
                                h1327
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f179(v0: List<(Bytes, Adt0)>, v1: Adt0) -> List<(Bytes, Adt0)> {
    let m1332 = f216(v1.clone());
    match m1332 {
        None => {
            v0
        }
        Some(_) => {
            List::cons((f15(Str::lit("type_arguments")), v1), v0)
        }
    }
}

pub fn f180(v0: Fn15, p1337: u64, p1338: Adt15) -> R<(Adt15, bool)> {
    let m1336 = (p1337, p1338);
    {
        let mut m1336 = m1336;
        loop {
            let step = {
                let (v1, v2) = m1336;
                let m1333 = v1;
                if m1333 == 0 {
                    Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1333 - 1;
                    let m1335 = {
                        let c1334: Fn15 = v0.clone();
                        c1334.apply(v2.clone())?
                    };
                    match m1335 {
                        None => {
                            Err::<(u64, Adt15), R<(Adt15, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt15), R<(Adt15, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: u64, v1: List<Adt5>, v2: Adt15) -> R<Option<Adt15>> {
    let m1340 = match v2.clone() {
        Adt15::C0(_, _, _, h1339) => {
            h1339
        }
    };
    match m1340 {
        None => {
            if f133(v1.clone(), match v2.clone() {
                Adt15::C0(_, h1341, _, _) => {
                    h1341
                }
            }, f15(Str::lit("=>"))) {
                Ok(None::<Adt15>)
            } else {
                let v4: Adt12 = f120(v1, v0, match v2.clone() {
                    Adt15::C0(_, h1342, _, _) => {
                        h1342
                    }
                }, f15(Str::lit("a binder name")))?;
                let m1344 = match v4.clone() {
                    Adt12::C0(_, _, h1343) => {
                        h1343
                    }
                };
                match m1344 {
                    None => {
                        Ok(Some(Adt15::C0(List::cons(match v4.clone() {
                            Adt12::C0(h1348, _, _) => {
                                (*h1348).clone()
                            }
                        }, match v2 {
                            Adt15::C0(h1349, _, _, _) => {
                                h1349
                            }
                        }), match v4 {
                            Adt12::C0(_, h1350, _) => {
                                h1350
                            }
                        }, true, None::<Adt1>)))
                    }
                    Some(_) => {
                        Ok(Some(Adt15::C0(match v2.clone() {
                            Adt15::C0(h1345, _, _, _) => {
                                h1345
                            }
                        }, match v2 {
                            Adt15::C0(_, h1346, _, _) => {
                                h1346
                            }
                        }, false, match v4 {
                            Adt12::C0(_, _, h1347) => {
                                h1347
                            }
                        })))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f182(v0: u64) -> Bytes {
    f51(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f183(v0: Bytes, v1: Adt9) -> R<Adt9> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h1351, _, _, _) => {
            h1351
        }
    };
    let m1354 = f217(v0, {
        let a1352 = v2;
        let a1353 = 2u64;
        nat_add(a1352, a1353)?
    })?;
    match m1354 {
        None => {
            Ok(f144(v1, v2, f49(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a1355 = 55296u64;
            let a1356 = v3;
            if nat_le(a1355, a1356) {
                let a1357 = v3;
                let a1358 = 57343u64;
                if nat_le(a1357, a1358) {
                    Ok(f144(v1, v2, f184(f184(f49(Str::lit("`\\u")), f218(v3)), f49(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f146(v1, {
                        let a1359 = v2;
                        let a1360 = 6u64;
                        nat_add(a1359, a1360)?
                    }, f219(v3)?))
                }
            } else {
                Ok(f146(v1, {
                    let a1361 = v2;
                    let a1362 = 6u64;
                    nat_add(a1361, a1362)?
                }, f219(v3)?))
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Bytes) -> Bytes {
    let a1363 = v0;
    let a1364 = v1;
    append_bytes(a1363, a1364)
}

pub fn f185(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f143(v0, v1)?;
    let a1365 = v2;
    let a1366 = 128u64;
    if nat_lt(a1365, a1366) {
        Ok(1u64)
    } else {
        let a1367 = v2;
        let a1368 = 224u64;
        if nat_lt(a1367, a1368) {
            Ok(2u64)
        } else {
            let a1369 = v2;
            let a1370 = 240u64;
            if nat_lt(a1369, a1370) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f186(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f220(v1, v0, v2)
}

pub fn f187(v0: Fn16, p1374: (Option<Bytes>, List<Bytes>), p1375: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m1373 = (p1374, p1375);
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let (v1, v2) = m1373;
                let m1371 = v2;
                match m1371.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c1372: Fn16 = v0.clone();
                            c1372.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f221(v0, v1)
}

pub fn f189(v0: Bytes, v1: u64) -> bool {
    if f30(v0.clone(), v1, 123u8) {
        true
    } else {
        if f30(v0.clone(), v1, 125u8) {
            true
        } else {
            if f30(v0.clone(), v1, 91u8) {
                true
            } else {
                if f30(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f30(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f30(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f30(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f30(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f30(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f30(v0, v1, 34u8)
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

pub fn f190(v0: Adt6) -> u64 {
    let m1376 = v0;
    match m1376 {
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
            8u64
        }
        Adt6::C9 => {
            9u64
        }
        Adt6::C10 => {
            10u64
        }
        Adt6::C11(_) => {
            11u64
        }
        Adt6::C12(_) => {
            12u64
        }
    }
}

pub fn f191(v0: Adt6) -> Bytes {
    let m1377 = v0;
    match m1377 {
        Adt6::C0 => {
            Bytes::lit(&[])
        }
        Adt6::C1 => {
            Bytes::lit(&[])
        }
        Adt6::C2 => {
            Bytes::lit(&[])
        }
        Adt6::C3 => {
            Bytes::lit(&[])
        }
        Adt6::C4 => {
            Bytes::lit(&[])
        }
        Adt6::C5 => {
            Bytes::lit(&[])
        }
        Adt6::C6 => {
            Bytes::lit(&[])
        }
        Adt6::C7 => {
            Bytes::lit(&[])
        }
        Adt6::C8 => {
            Bytes::lit(&[])
        }
        Adt6::C9 => {
            Bytes::lit(&[])
        }
        Adt6::C10 => {
            Bytes::lit(&[])
        }
        Adt6::C11(v1) => {
            v1
        }
        Adt6::C12(v2) => {
            v2
        }
    }
}

pub fn f192(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f222(v0, v1)
}

pub fn f193(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f223(v0, v1)
}

pub fn f194(v0: Option<Adt0>) -> Option<Adt0> {
    let m1378 = v0;
    match m1378 {
        None => {
            None::<Adt0>
        }
        Some(v1) => {
            f216(v1)
        }
    }
}

pub fn f195(v0: Fn17, p1382: Option<Adt0>, p1383: List<Adt0>) -> R<Option<Adt0>> {
    let m1381 = (p1382, p1383);
    {
        let mut m1381 = m1381;
        loop {
            let step = {
                let (v1, v2) = m1381;
                let m1379 = v2;
                match m1379.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<Adt0>), R<Option<Adt0>>>(({
                            let c1380: Fn17 = v0.clone();
                            c1380.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt0) -> R<Option<Adt0>> {
    let m1384 = v2.clone();
    match m1384 {
        None => {
            let v5: Adt0 = f38(v1, v3)?;
            if f224(v5.clone(), v0)? {
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

pub fn f197(v0: Adt0) -> List<Adt0> {
    f225({
        let m1387 = {
            let (h1386, _) = f226(Fn22::F227, 1000000u64, Adt16::C0(std::rc::Rc::new(v0), List::<Adt0>::nil()));
            h1386
        };
        match m1387 {
            Adt16::C0(_, h1385) => {
                h1385
            }
        }
    })
}

pub fn f198(v0: Fn18, p1391: bool, p1392: List<Adt0>) -> R<bool> {
    let m1390 = (p1391, p1392);
    {
        let mut m1390 = m1390;
        loop {
            let step = {
                let (v1, v2) = m1390;
                let m1388 = v2;
                match m1388.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), R<bool>>(({
                            let c1389: Fn18 = v0.clone();
                            c1389.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Adt0, v1: bool, v2: Adt0) -> R<bool> {
    if v1 {
        let m1393 = f22(f38(v0, v2)?, f15(Str::lit("properties")));
        match m1393 {
            None => {
                Ok(false)
            }
            Some(v3) => {
                Ok(f228(f22(v3, f15(Str::lit("kind")))))
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    let a1394 = v0;
    let a1395 = v1;
    append_bytes(a1394, a1395)
}

pub fn f201(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a1396 = v0.clone();
        length_bytes(a1396)
    };
    let v2: Adt17 = {
        let (h1397, _) = f229(Fn23::F230(v1, v0), {
            let a1398 = v1;
            let a1399 = 1u64;
            nat_add(a1398, a1399)?
        }, Adt17::C0(0u64, List::<Bytes>::nil()))?;
        h1397
    };
    Ok(f184(f184(f49(Str::lit("\"")), f47(match v2 {
        Adt17::C0(_, h1400) => {
            h1400
        }
    })), f49(Str::lit("\""))))
}

pub fn f202(v0: Bytes) -> Option<Bytes> {
    if f57(v0.clone(), f15(Str::lit("Int"))) {
        Some(f15(Str::lit("int")))
    } else {
        if f57(v0.clone(), f15(Str::lit("Int8"))) {
            Some(f15(Str::lit("int8")))
        } else {
            if f57(v0.clone(), f15(Str::lit("Int16"))) {
                Some(f15(Str::lit("int16")))
            } else {
                if f57(v0.clone(), f15(Str::lit("Int32"))) {
                    Some(f15(Str::lit("int32")))
                } else {
                    if f57(v0.clone(), f15(Str::lit("Int64"))) {
                        Some(f15(Str::lit("int64")))
                    } else {
                        if f57(v0.clone(), f15(Str::lit("UInt8"))) {
                            Some(f15(Str::lit("uint8")))
                        } else {
                            if f57(v0.clone(), f15(Str::lit("UInt16"))) {
                                Some(f15(Str::lit("uint16")))
                            } else {
                                if f57(v0.clone(), f15(Str::lit("UInt32"))) {
                                    Some(f15(Str::lit("uint32")))
                                } else {
                                    if f57(v0, f15(Str::lit("UInt64"))) {
                                        Some(f15(Str::lit("uint64")))
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

pub fn f203(v0: List<Adt5>, v1: u64, v2: u64, v3: Bytes) -> R<Adt12> {
    let a1401 = f93(v0.clone(), v2);
    let a1402 = 2u64;
    if nat_eq(a1401, a1402) {
        let v4: Adt15 = {
            let (h1403, _) = f180(Fn15::F231(v1, v0.clone(), v3), {
                let a1405 = {
                    let a1404 = v0.clone();
                    length_list(a1404)
                };
                let a1406 = 2u64;
                nat_add(a1405, a1406)?
            }, Adt15::C0(List::<Adt0>::nil(), {
                let a1407 = v2;
                let a1408 = 1u64;
                nat_add(a1407, a1408)?
            }, true, None::<Adt1>))?;
            h1403
        };
        let m1410 = match v4.clone() {
            Adt15::C0(_, _, _, h1409) => {
                h1409
            }
        };
        match m1410 {
            None => {
                let a1414 = f93(v0.clone(), match v4.clone() {
                    Adt15::C0(_, h1413, _, _) => {
                        h1413
                    }
                });
                let a1415 = 3u64;
                if nat_eq(a1414, a1415) {
                    Ok(Adt12::C0(std::rc::Rc::new(f9(match v4.clone() {
                        Adt15::C0(h1416, _, _, _) => {
                            h1416
                        }
                    })), {
                        let a1418 = match v4 {
                            Adt15::C0(_, h1417, _, _) => {
                                h1417
                            }
                        };
                        let a1419 = 1u64;
                        nat_add(a1418, a1419)?
                    }, None::<Adt1>))
                } else {
                    Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), match v4.clone() {
                        Adt15::C0(_, h1420, _, _) => {
                            h1420
                        }
                    }, f170(f16(v0, v1, match v4 {
                        Adt15::C0(_, h1421, _, _) => {
                            h1421
                        }
                    }), f15(Str::lit("expected `,` or `]`")))))
                }
            }
            Some(_) => {
                Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), match v4.clone() {
                    Adt15::C0(_, h1411, _, _) => {
                        h1411
                    }
                }, match v4 {
                    Adt15::C0(_, _, _, h1412) => {
                        h1412
                    }
                }))
            }
        }
    } else {
        Ok(Adt12::C0(std::rc::Rc::new(Adt0::C5), v2, f170(f16(v0, v1, v2), f15(Str::lit("expected `[`")))))
    }
}

pub fn f204(v0: List<Adt5>, v1: u64, v2: Adt2, v3: Adt3, v4: u64) -> R<Adt2> {
    let a1422 = f93(v0.clone(), v4);
    let a1423 = 2u64;
    if nat_eq(a1422, a1423) {
        Ok(f98(v2, v3.clone(), v4, 6u64, Adt4::C5, match v3 {
            Adt3::C0(_, _, h1424, _, _, _, _, _, _, _, _, _) => {
                h1424
            }
        }))
    } else {
        f137(v0, v1, Adt2::C0(v4, match v2.clone() {
            Adt2::C0(_, h1425, _, _, _, _, _, _, _) => {
                h1425
            }
        }, std::rc::Rc::new(Adt0::C5), 1u64, match v2.clone() {
            Adt2::C0(_, _, _, _, h1426, _, _, _, _) => {
                h1426
            }
        }, std::rc::Rc::new(match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1427, _, _, _) => {
                (*h1427).clone()
            }
        }), match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h1428, _, _) => {
                h1428
            }
        }, None::<Adt1>, match v2 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1429) => {
                h1429
            }
        }), v3)
    }
}

pub fn f205(v0: List<Adt5>, v1: u64, v2: Adt2, v3: u64, v4: u64) -> R<Adt2> {
    let v5: Bytes = f101(v0.clone(), v3);
    let v6: u64 = {
        let a1432 = f93(v0.clone(), {
            let a1430 = v3;
            let a1431 = 1u64;
            nat_add(a1430, a1431)?
        });
        let a1433 = 9u64;
        if nat_eq(a1432, a1433) {
            3u64
        } else {
            1u64
        }
    };
    let v7: bool = {
        let a1436 = f93(v0.clone(), {
            let a1434 = v3;
            let a1435 = v6;
            nat_add(a1434, a1435)?
        });
        let a1437 = 6u64;
        if nat_eq(a1436, a1437) {
            true
        } else {
            f102({
                let a1440 = f93(v0.clone(), {
                    let a1438 = v3;
                    let a1439 = v6;
                    nat_add(a1438, a1439)?
                });
                let a1441 = 12u64;
                nat_eq(a1440, a1441)
            }, f232(f101(v0.clone(), {
                let a1442 = v3;
                let a1443 = v6;
                nat_add(a1442, a1443)?
            })))
        }
    };
    let v8: bool = {
        let a1446 = f93(v0.clone(), {
            let a1444 = v3;
            let a1445 = 1u64;
            nat_add(a1444, a1445)?
        });
        let a1447 = 0u64;
        nat_eq(a1446, a1447)
    };
    if f102({
        let a1448 = f93(v0.clone(), v3);
        let a1449 = 12u64;
        nat_eq(a1448, a1449)
    }, f102(f168(v5.clone())?, f102(!v7, f102(!v8, !f104(v5)?)))) {
        let v9: Adt12 = f94(v0.clone(), v1, v3)?;
        let m1451 = match v9.clone() {
            Adt12::C0(_, _, h1450) => {
                h1450
            }
        };
        match m1451 {
            None => {
                f204(v0, v1, v2, f121(f14(46u64, v4), match v9.clone() {
                    Adt12::C0(h1453, _, _) => {
                        (*h1453).clone()
                    }
                }, Adt0::C5), match v9 {
                    Adt12::C0(_, h1454, _) => {
                        h1454
                    }
                })
            }
            Some(_) => {
                Ok(f95(v2, match v9 {
                    Adt12::C0(_, _, h1452) => {
                        h1452
                    }
                }))
            }
        }
    } else {
        f134(v2, f14(47u64, v4), v3)
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1455 = f45(v0, v1);
    match m1455 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1500 = {
                    let a1476 = {
                        let a1464 = {
                            let a1458 = {
                                let a1456 = v3;
                                let a1457 = 128u8;
                                fixed_u8::bit_and(a1456, a1457)
                            };
                            let a1459 = 128u8;
                            if equal(a1458, a1459) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1465 = {
                            let a1462 = {
                                let a1460 = v3;
                                let a1461 = 64u8;
                                fixed_u8::bit_and(a1460, a1461)
                            };
                            let a1463 = 64u8;
                            if equal(a1462, a1463) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1464, a1465)?
                    };
                    let a1477 = {
                        let a1474 = {
                            let a1468 = {
                                let a1466 = v3;
                                let a1467 = 32u8;
                                fixed_u8::bit_and(a1466, a1467)
                            };
                            let a1469 = 32u8;
                            if equal(a1468, a1469) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1475 = {
                            let a1472 = {
                                let a1470 = v3;
                                let a1471 = 16u8;
                                fixed_u8::bit_and(a1470, a1471)
                            };
                            let a1473 = 16u8;
                            if equal(a1472, a1473) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1474, a1475)?
                    };
                    nat_add(a1476, a1477)?
                };
                let a1501 = {
                    let a1498 = {
                        let a1486 = {
                            let a1480 = {
                                let a1478 = v3;
                                let a1479 = 8u8;
                                fixed_u8::bit_and(a1478, a1479)
                            };
                            let a1481 = 8u8;
                            if equal(a1480, a1481) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1487 = {
                            let a1484 = {
                                let a1482 = v3;
                                let a1483 = 4u8;
                                fixed_u8::bit_and(a1482, a1483)
                            };
                            let a1485 = 4u8;
                            if equal(a1484, a1485) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1486, a1487)?
                    };
                    let a1499 = {
                        let a1496 = {
                            let a1490 = {
                                let a1488 = v3;
                                let a1489 = 2u8;
                                fixed_u8::bit_and(a1488, a1489)
                            };
                            let a1491 = 2u8;
                            if equal(a1490, a1491) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1497 = {
                            let a1494 = {
                                let a1492 = v3;
                                let a1493 = 1u8;
                                fixed_u8::bit_and(a1492, a1493)
                            };
                            let a1495 = 1u8;
                            if equal(a1494, a1495) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1496, a1497)?
                    };
                    nat_add(a1498, a1499)?
                };
                nat_add(a1500, a1501)?
            };
            if if f102({
                let a1502 = 48u64;
                let a1503 = v4;
                nat_le(a1502, a1503)
            }, {
                let a1504 = v4;
                let a1505 = 57u64;
                nat_le(a1504, a1505)
            }) {
                true
            } else {
                f102({
                    let a1506 = 97u64;
                    let a1507 = v4;
                    nat_le(a1506, a1507)
                }, {
                    let a1508 = v4;
                    let a1509 = 102u64;
                    nat_le(a1508, a1509)
                })
            } {
                Ok(Some({
                    let a1510 = v1;
                    let a1511 = 1u64;
                    nat_add(a1510, a1511)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f207(v0: List<Adt5>, v1: u64, v2: Adt13) -> R<Option<Adt13>> {
    let m1513 = match v2.clone() {
        Adt13::C0(_, _, _, _, _, h1512) => {
            h1512
        }
    };
    match m1513 {
        None => {
            let a1515 = match v2.clone() {
                Adt13::C0(_, _, h1514, _, _, _) => {
                    h1514
                }
            };
            let a1516 = 9u64;
            if nat_eq(a1515, a1516) {
                Ok(None::<Adt13>)
            } else {
                Ok(Some(f233(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt13>)
        }
    }
}

pub fn f208(v0: Fn19, p1520: Adt0, p1521: List<(Bytes, Adt0)>) -> Adt0 {
    let m1519 = (p1520, p1521);
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let (v1, v2) = m1519;
                let m1517 = v2;
                match m1517.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c1518: Fn19 = v0.clone();
                            c1518.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h1522, _) = v1;
        h1522
    }, std::rc::Rc::new({
        let (_, h1523) = v1;
        h1523
    }), std::rc::Rc::new(v0))
}

pub fn f210(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f234(Fn24::F235, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f211(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f236(Fn25::F237, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f212(v0: Fn20, p1527: List<List<(Bytes, Adt0)>>, p1528: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m1526 = (p1527, p1528);
    {
        let mut m1526 = m1526;
        loop {
            let step = {
                let (v1, v2) = m1526;
                let m1524 = v2;
                match m1524.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c1525: Fn20 = v0.clone();
                            c1525.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f214(v0: Fn21, p1533: u64, p1534: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m1532 = (p1533, p1534);
    {
        let mut m1532 = m1532;
        loop {
            let step = {
                let (v1, v2) = m1532;
                let m1529 = v1;
                if m1529 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1529 - 1;
                    let m1531 = {
                        let c1530: Fn21 = v0.clone();
                        c1530.apply(v2.clone())?
                    };
                    match m1531 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f238(v0)
}

pub fn f216(v0: Adt0) -> Option<Adt0> {
    let m1535 = v0;
    match m1535 {
        Adt0::C0(_) => {
            None::<Adt0>
        }
        Adt0::C1(_, _) => {
            None::<Adt0>
        }
        Adt0::C2(_) => {
            None::<Adt0>
        }
        Adt0::C3(r1536) => {
            let v1 = (*r1536).clone();
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

pub fn f217(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1537 = f239(v0.clone(), v1)?;
    match m1537 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1540 = f239(v0.clone(), {
                let a1538 = v1;
                let a1539 = 1u64;
                nat_add(a1538, a1539)?
            })?;
            match m1540 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m1543 = f239(v0.clone(), {
                        let a1541 = v1;
                        let a1542 = 2u64;
                        nat_add(a1541, a1542)?
                    })?;
                    match m1543 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m1546 = f239(v0, {
                                let a1544 = v1;
                                let a1545 = 3u64;
                                nat_add(a1544, a1545)?
                            })?;
                            match m1546 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a1557 = {
                                            let a1555 = {
                                                let a1553 = {
                                                    let a1551 = {
                                                        let a1549 = {
                                                            let a1547 = v2;
                                                            let a1548 = 16u64;
                                                            nat_mul(a1547, a1548)?
                                                        };
                                                        let a1550 = v3;
                                                        nat_add(a1549, a1550)?
                                                    };
                                                    let a1552 = 16u64;
                                                    nat_mul(a1551, a1552)?
                                                };
                                                let a1554 = v4;
                                                nat_add(a1553, a1554)?
                                            };
                                            let a1556 = 16u64;
                                            nat_mul(a1555, a1556)?
                                        };
                                        let a1558 = v5;
                                        nat_add(a1557, a1558)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f218(v0: u64) -> Bytes {
    f184(f184(f240(v0, 4096u64), f240(v0, 256u64)), f184(f240(v0, 16u64), f241(v0)))
}

pub fn f219(v0: u64) -> R<Bytes> {
    let a1559 = v0;
    let a1560 = 128u64;
    if nat_lt(a1559, a1560) {
        Ok(f182(v0))
    } else {
        let a1561 = v0;
        let a1562 = 2048u64;
        if nat_lt(a1561, a1562) {
            Ok(f184(f182({
                let a1566 = 192u64;
                let a1567 = {
                    let a1563 = v0;
                    let a1564 = 64u64;
                    let a1565 = 0u64;
                    nat_quot(a1563, a1564, a1565)
                };
                nat_add(a1566, a1567)?
            }), f182({
                let a1571 = 128u64;
                let a1572 = {
                    let a1568 = v0;
                    let a1569 = 64u64;
                    let a1570 = 0u64;
                    nat_rem(a1568, a1569, a1570)
                };
                nat_add(a1571, a1572)?
            })))
        } else {
            Ok(f184(f182({
                let a1576 = 224u64;
                let a1577 = {
                    let a1573 = v0;
                    let a1574 = 4096u64;
                    let a1575 = 0u64;
                    nat_quot(a1573, a1574, a1575)
                };
                nat_add(a1576, a1577)?
            }), f184(f182({
                let a1584 = 128u64;
                let a1585 = {
                    let a1581 = {
                        let a1578 = v0;
                        let a1579 = 64u64;
                        let a1580 = 0u64;
                        nat_quot(a1578, a1579, a1580)
                    };
                    let a1582 = 64u64;
                    let a1583 = 0u64;
                    nat_rem(a1581, a1582, a1583)
                };
                nat_add(a1584, a1585)?
            }), f182({
                let a1589 = 128u64;
                let a1590 = {
                    let a1586 = v0;
                    let a1587 = 64u64;
                    let a1588 = 0u64;
                    nat_rem(a1586, a1587, a1588)
                };
                nat_add(a1589, a1590)?
            }))))
        }
    }
}

pub fn f220(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a1591 = v2;
    let a1592 = v1;
    if nat_lt(a1591, a1592) {
        let v3: u64 = f143(v0, v2)?;
        let a1593 = v3;
        let a1594 = 32u64;
        if nat_lt(a1593, a1594) {
            Ok(None::<u64>)
        } else {
            let a1595 = v3;
            let a1596 = 34u64;
            if nat_eq(a1595, a1596) {
                Ok(None::<u64>)
            } else {
                let a1597 = v3;
                let a1598 = 92u64;
                if nat_eq(a1597, a1598) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a1599 = v2;
                        let a1600 = 1u64;
                        nat_add(a1599, a1600)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f221(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m1602 = {
        let (h1601, _) = v0;
        h1601
    };
    match m1602 {
        None => {
            (Some(v1), {
                let (_, h1603) = v0;
                h1603
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a1604 = v2;
                let a1605 = v1;
                append_bytes(a1604, a1605)
            }, {
                let (_, h1606) = v0;
                h1606
            }))
        }
    }
}

pub fn f222(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f242(f153(v0, v1)?) {
        Ok(Some({
            let a1607 = v1;
            let a1608 = 1u64;
            nat_add(a1607, a1608)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f223(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f153(v0, v1)?;
    let a1609 = 47u64;
    let a1610 = v2;
    if nat_lt(a1609, a1610) {
        let a1611 = v2;
        let a1612 = 58u64;
        if nat_lt(a1611, a1612) {
            Ok(Some({
                let a1613 = v1;
                let a1614 = 1u64;
                nat_add(a1613, a1614)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f224(v0: Adt0, v1: Bytes) -> R<bool> {
    let m1615 = f22(v0, f15(Str::lit("properties")));
    match m1615 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1616 = f22(v2, f15(Str::lit("kind")));
            match m1616 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1617 = f55(f22(v3.clone(), f15(Str::lit("const"))));
                    if match m1617 {
                        None => {
                            false
                        }
                        Some(v4) => {
                            f57(v4, v1.clone())
                        }
                    } {
                        Ok(true)
                    } else {
                        let m1618 = f194(f22(v3, f15(Str::lit("enum"))));
                        match m1618 {
                            None => {
                                Ok(false)
                            }
                            Some(v5) => {
                                f198(Fn18::F243(v1), false, f197(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f225(v0: List<Adt0>) -> List<Adt0> {
    f244(Fn26::F245, List::<Adt0>::nil(), v0)
}

pub fn f226(v0: Fn22, p1623: u64, p1624: Adt16) -> (Adt16, bool) {
    let m1622 = (p1623, p1624);
    {
        let mut m1622 = m1622;
        loop {
            let step = {
                let (v1, v2) = m1622;
                let m1619 = v1;
                if m1619 == 0 {
                    Err::<(u64, Adt16), (Adt16, bool)>((v2, false))
                } else {
                    let v3 = m1619 - 1;
                    let m1621 = {
                        let c1620: Fn22 = v0.clone();
                        c1620.apply(v2.clone())
                    };
                    match m1621 {
                        None => {
                            Err::<(u64, Adt16), (Adt16, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt16), (Adt16, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Adt16) -> Option<Adt16> {
    f246(v0)
}

pub fn f228(v0: Option<Adt0>) -> bool {
    let m1625 = v0;
    match m1625 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f229(v0: Fn23, p1630: u64, p1631: Adt17) -> R<(Adt17, bool)> {
    let m1629 = (p1630, p1631);
    {
        let mut m1629 = m1629;
        loop {
            let step = {
                let (v1, v2) = m1629;
                let m1626 = v1;
                if m1626 == 0 {
                    Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1626 - 1;
                    let m1628 = {
                        let c1627: Fn23 = v0.clone();
                        c1627.apply(v2.clone())?
                    };
                    match m1628 {
                        None => {
                            Err::<(u64, Adt17), R<(Adt17, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt17), R<(Adt17, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: u64, v1: Bytes, v2: Adt17) -> R<Option<Adt17>> {
    f247(v1, v0, v2)
}

pub fn f231(v0: u64, v1: List<Adt5>, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    f248(v1, v0, v2, v3)
}

pub fn f232(v0: Bytes) -> bool {
    let m1632 = f141(v0);
    match m1632 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f233(v0: List<Adt5>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(_, _, h1633, _, _, _) => {
            h1633
        }
    };
    let a1634 = v3;
    let a1635 = 0u64;
    if nat_eq(a1634, a1635) {
        f249(v0, v1, v2)
    } else {
        let a1636 = v3;
        let a1637 = 1u64;
        if nat_eq(a1636, a1637) {
            f250(v0, v2)
        } else {
            let a1638 = v3;
            let a1639 = 2u64;
            if nat_eq(a1638, a1639) {
                f251(v0, v1, v2)
            } else {
                let a1640 = v3;
                let a1641 = 3u64;
                if nat_eq(a1640, a1641) {
                    f252(v0, v2)
                } else {
                    let a1642 = v3;
                    let a1643 = 4u64;
                    if nat_eq(a1642, a1643) {
                        f253(v0, v1, v2, 3u64, f163(Str::lit("expected `,` or `]`")))
                    } else {
                        f253(v0, v1, v2, 1u64, f163(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f234(v0: Fn24, p1647: List<(Bytes, Adt0)>, p1648: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m1646 = (p1647, p1648);
    {
        let mut m1646 = m1646;
        loop {
            let step = {
                let (v1, v2) = m1646;
                let m1644 = v2;
                match m1644.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c1645: Fn24 = v0.clone();
                            c1645.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f236(v0: Fn25, p1652: List<List<(Bytes, Adt0)>>, p1653: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m1651 = (p1652, p1653);
    {
        let mut m1651 = m1651;
        loop {
            let step = {
                let (v1, v2) = m1651;
                let m1649 = v2;
                match m1649.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c1650: Fn25 = v0.clone();
                            c1650.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f238(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a1655 = {
        let a1654 = v0.clone();
        length_list(a1654)
    };
    let a1656 = 1u64;
    if nat_le(a1655, a1656) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f254(v0)?))
    }
}

pub fn f239(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1657 = f45(v0, v1);
    match m1657 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f255(v2)
        }
    }
}

pub fn f240(v0: u64, v1: u64) -> Bytes {
    f241({
        let a1658 = v0;
        let a1659 = v1;
        let a1660 = 0u64;
        nat_quot(a1658, a1659, a1660)
    })
}

pub fn f241(v0: u64) -> Bytes {
    f51(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1661 = v0;
        let a1662 = 16u64;
        let a1663 = 0u64;
        nat_rem(a1661, a1662, a1663)
    }, 1u64)
}

pub fn f242(v0: u64) -> bool {
    if f152(v0) {
        true
    } else {
        let a1664 = 47u64;
        let a1665 = v0;
        if if nat_lt(a1664, a1665) {
            let a1666 = v0;
            let a1667 = 58u64;
            nat_lt(a1666, a1667)
        } else {
            false
        } {
            true
        } else {
            let a1668 = v0;
            let a1669 = 46u64;
            if nat_eq(a1668, a1669) {
                true
            } else {
                let a1670 = v0;
                let a1671 = 39u64;
                if nat_eq(a1670, a1671) {
                    true
                } else {
                    let a1672 = v0;
                    let a1673 = 47u64;
                    if nat_eq(a1672, a1673) {
                        true
                    } else {
                        let a1674 = v0;
                        let a1675 = 45u64;
                        nat_eq(a1674, a1675)
                    }
                }
            }
        }
    }
}

pub fn f243(v0: Bytes, v1: bool, v2: Adt0) -> bool {
    if v1 {
        true
    } else {
        let m1676 = f69(v2);
        match m1676 {
            None => {
                false
            }
            Some(v3) => {
                f57(v3, v0)
            }
        }
    }
}

pub fn f244(v0: Fn26, p1680: List<Adt0>, p1681: List<Adt0>) -> List<Adt0> {
    let m1679 = (p1680, p1681);
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, v2) = m1679;
                let m1677 = v2;
                match m1677.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c1678: Fn26 = v0.clone();
                            c1678.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f246(v0: Adt16) -> Option<Adt16> {
    let m1683 = f256(match v0.clone() {
        Adt16::C0(h1682, _) => {
            (*h1682).clone()
        }
    });
    match m1683 {
        None => {
            None::<Adt16>
        }
        Some(v1) => {
            Some(Adt16::C0(std::rc::Rc::new(match v1.clone() {
                Adt18::C0(_, h1684) => {
                    (*h1684).clone()
                }
            }), List::cons(match v1 {
                Adt18::C0(h1685, _) => {
                    (*h1685).clone()
                }
            }, match v0 {
                Adt16::C0(_, h1686) => {
                    h1686
                }
            })))
        }
    }
}

pub fn f247(v0: Bytes, v1: u64, v2: Adt17) -> R<Option<Adt17>> {
    let v3: u64 = match v2.clone() {
        Adt17::C0(h1687, _) => {
            h1687
        }
    };
    let a1688 = v3;
    let a1689 = v1;
    if nat_lt(a1688, a1689) {
        let v4: u64 = f143(v0.clone(), v3)?;
        let a1690 = v4;
        let a1691 = 32u64;
        if if nat_lt(a1690, a1691) {
            true
        } else {
            let a1692 = v4;
            let a1693 = 34u64;
            if nat_eq(a1692, a1693) {
                true
            } else {
                let a1694 = v4;
                let a1695 = 92u64;
                nat_eq(a1694, a1695)
            }
        } {
            Ok(Some(Adt17::C0({
                let a1696 = v3;
                let a1697 = 1u64;
                nat_add(a1696, a1697)?
            }, List::cons(f257(v4), match v2 {
                Adt17::C0(_, h1698) => {
                    h1698
                }
            }))))
        } else {
            let v5: u64 = f145(v0.clone(), v1, v3)?;
            Ok(Some(Adt17::C0(v5, List::cons(f51(v0, v3, {
                let a1699 = v5;
                let a1700 = v3;
                nat_sub(a1699, a1700)
            }), match v2 {
                Adt17::C0(_, h1701) => {
                    h1701
                }
            }))))
        }
    } else {
        Ok(None::<Adt17>)
    }
}

pub fn f248(v0: List<Adt5>, v1: u64, v2: Bytes, v3: Adt15) -> R<Option<Adt15>> {
    let m1703 = match v3.clone() {
        Adt15::C0(_, _, _, h1702) => {
            h1702
        }
    };
    match m1703 {
        None => {
            if match v3.clone() {
                Adt15::C0(_, _, h1704, _) => {
                    h1704
                }
            } {
                let v5: u64 = match v3.clone() {
                    Adt15::C0(_, h1705, _, _) => {
                        h1705
                    }
                };
                let a1706 = f93(v0.clone(), v5);
                let a1707 = 3u64;
                if nat_eq(a1706, a1707) {
                    Ok(None::<Adt15>)
                } else {
                    let v6: Adt12 = f120(v0.clone(), v1, v5, v2)?;
                    let m1709 = match v6.clone() {
                        Adt12::C0(_, _, h1708) => {
                            h1708
                        }
                    };
                    match m1709 {
                        None => {
                            let a1713 = f93(v0, match v6.clone() {
                                Adt12::C0(_, h1712, _) => {
                                    h1712
                                }
                            });
                            let a1714 = 6u64;
                            if nat_eq(a1713, a1714) {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1715, _, _) => {
                                        (*h1715).clone()
                                    }
                                }, match v3 {
                                    Adt15::C0(h1716, _, _, _) => {
                                        h1716
                                    }
                                }), {
                                    let a1718 = match v6 {
                                        Adt12::C0(_, h1717, _) => {
                                            h1717
                                        }
                                    };
                                    let a1719 = 1u64;
                                    nat_add(a1718, a1719)?
                                }, true, None::<Adt1>)))
                            } else {
                                Ok(Some(Adt15::C0(List::cons(match v6.clone() {
                                    Adt12::C0(h1720, _, _) => {
                                        (*h1720).clone()
                                    }
                                }, match v3 {
                                    Adt15::C0(h1721, _, _, _) => {
                                        h1721
                                    }
                                }), match v6 {
                                    Adt12::C0(_, h1722, _) => {
                                        h1722
                                    }
                                }, false, None::<Adt1>)))
                            }
                        }
                        Some(_) => {
                            Ok(Some(Adt15::C0(match v3 {
                                Adt15::C0(h1710, _, _, _) => {
                                    h1710
                                }
                            }, v5, false, match v6 {
                                Adt12::C0(_, _, h1711) => {
                                    h1711
                                }
                            })))
                        }
                    }
                }
            } else {
                Ok(None::<Adt15>)
            }
        }
        Some(_) => {
            Ok(None::<Adt15>)
        }
    }
}

pub fn f249(v0: List<Adt5>, v1: u64, v2: Adt13) -> R<Adt13> {
    let v3: u64 = match v2.clone() {
        Adt13::C0(h1723, _, _, _, _, _) => {
            h1723
        }
    };
    let a1725 = 256u64;
    let a1726 = match v2.clone() {
        Adt13::C0(_, h1724, _, _, _, _) => {
            h1724
        }
    };
    if nat_lt(a1725, a1726) {
        Ok(f258(v2, f16(v0, v1, v3), f163(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f149(v0.clone(), v3);
        let a1727 = v4;
        let a1728 = 99u64;
        if nat_eq(a1727, a1728) {
            Ok(f258(v2, v1, f163(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a1729 = v4;
            let a1730 = 11u64;
            if nat_eq(a1729, a1730) {
                Ok(f259(v2, {
                    let a1731 = v3;
                    let a1732 = 1u64;
                    nat_add(a1731, a1732)?
                }, Adt0::C2(f151(v0, v3))))
            } else {
                let a1733 = v4;
                let a1734 = 2u64;
                if nat_eq(a1733, a1734) {
                    f260(v2, {
                        let a1735 = v3;
                        let a1736 = 1u64;
                        nat_add(a1735, a1736)?
                    }, Adt14::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a1737 = v4;
                    let a1738 = 0u64;
                    if nat_eq(a1737, a1738) {
                        f260(v2, {
                            let a1739 = v3;
                            let a1740 = 1u64;
                            nat_add(a1739, a1740)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a1741 = v4;
                        let a1742 = 12u64;
                        if nat_eq(a1741, a1742) {
                            f261(v0.clone(), v1, v2, f16(v0, v1, v3))
                        } else {
                            Ok(f258(v2, f16(v0.clone(), v1, v3), f200(f163(Str::lit("expected a value, found ")), f117(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f250(v0: List<Adt5>, v1: Adt13) -> R<Adt13> {
    let a1744 = f149(v0, match v1.clone() {
        Adt13::C0(h1743, _, _, _, _, _) => {
            h1743
        }
    });
    let a1745 = 3u64;
    if nat_eq(a1744, a1745) {
        f262(v1)
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1746, _, _, _, _, _) => {
                h1746
            }
        }, match v1.clone() {
            Adt13::C0(_, h1747, _, _, _, _) => {
                h1747
            }
        }, 0u64, match v1 {
            Adt13::C0(_, _, _, h1748, _, _) => {
                h1748
            }
        }, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    }
}

pub fn f251(v0: List<Adt5>, v1: u64, v2: Adt13) -> R<Adt13> {
    let m1750 = match v2.clone() {
        Adt13::C0(_, _, _, h1749, _, _) => {
            h1749
        }
    };
    match m1750.uncons() {
        None => {
            Ok(f258(v2, 0u64, f163(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m1751 = v3;
            match m1751 {
                Adt14::C0(_) => {
                    Ok(f258(v2, 0u64, f163(Str::lit("no object is open"))))
                }
                Adt14::C1(_, _, v8) => {
                    let a1753 = f149(v0.clone(), match v2.clone() {
                        Adt13::C0(h1752, _, _, _, _, _) => {
                            h1752
                        }
                    });
                    let a1754 = 1u64;
                    if nat_eq(a1753, a1754) {
                        f262(v2)
                    } else {
                        f263(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f252(v0: List<Adt5>, v1: Adt13) -> R<Adt13> {
    let m1756 = match v1.clone() {
        Adt13::C0(_, _, _, h1755, _, _) => {
            h1755
        }
    };
    match m1756.uncons() {
        None => {
            Ok(Adt13::C0(match v1.clone() {
                Adt13::C0(h1757, _, _, _, _, _) => {
                    h1757
                }
            }, match v1.clone() {
                Adt13::C0(_, h1758, _, _, _, _) => {
                    h1758
                }
            }, 9u64, List::<Adt14>::nil(), std::rc::Rc::new(match v1 {
                Adt13::C0(_, _, _, _, h1759, _) => {
                    (*h1759).clone()
                }
            }), None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m1760 = v2;
            match m1760 {
                Adt14::C0(v4) => {
                    f264(v0, v1.clone(), List::cons(Adt14::C0(List::cons(match v1 {
                        Adt13::C0(_, _, _, _, h1761, _) => {
                            (*h1761).clone()
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt14::C1(v5, v6, v7) => {
                    if f127(v7.clone(), v5.clone()) {
                        Ok(f258(v1, v6, f200(f200(f163(Str::lit("the member `")), v5), f163(Str::lit("` is written twice")))))
                    } else {
                        f264(v0, v1.clone(), List::cons(Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt13::C0(_, _, _, _, h1762, _) => {
                                (*h1762).clone()
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f253(v0: List<Adt5>, v1: u64, v2: Adt13, v3: u64, v4: Bytes) -> R<Adt13> {
    let a1764 = f149(v0.clone(), match v2.clone() {
        Adt13::C0(h1763, _, _, _, _, _) => {
            h1763
        }
    });
    let a1765 = v3;
    if nat_eq(a1764, a1765) {
        f262(v2)
    } else {
        Ok(f258(v2.clone(), f16(v0, v1, match v2 {
            Adt13::C0(h1766, _, _, _, _, _) => {
                h1766
            }
        }), v4))
    }
}

pub fn f254(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f211({
        let (_, h1767) = {
            let (h1768, _) = f265(Fn27::F266, {
                let a1770 = {
                    let a1769 = v0.clone();
                    length_list(a1769)
                };
                let a1771 = 1u64;
                nat_add(a1770, a1771)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h1768
        };
        h1767
    }))
}

pub fn f255(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1816 = {
            let a1792 = {
                let a1780 = {
                    let a1774 = {
                        let a1772 = v1;
                        let a1773 = 128u8;
                        fixed_u8::bit_and(a1772, a1773)
                    };
                    let a1775 = 128u8;
                    if equal(a1774, a1775) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1781 = {
                    let a1778 = {
                        let a1776 = v1;
                        let a1777 = 64u8;
                        fixed_u8::bit_and(a1776, a1777)
                    };
                    let a1779 = 64u8;
                    if equal(a1778, a1779) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1780, a1781)?
            };
            let a1793 = {
                let a1790 = {
                    let a1784 = {
                        let a1782 = v1;
                        let a1783 = 32u8;
                        fixed_u8::bit_and(a1782, a1783)
                    };
                    let a1785 = 32u8;
                    if equal(a1784, a1785) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1791 = {
                    let a1788 = {
                        let a1786 = v1;
                        let a1787 = 16u8;
                        fixed_u8::bit_and(a1786, a1787)
                    };
                    let a1789 = 16u8;
                    if equal(a1788, a1789) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1790, a1791)?
            };
            nat_add(a1792, a1793)?
        };
        let a1817 = {
            let a1814 = {
                let a1802 = {
                    let a1796 = {
                        let a1794 = v1;
                        let a1795 = 8u8;
                        fixed_u8::bit_and(a1794, a1795)
                    };
                    let a1797 = 8u8;
                    if equal(a1796, a1797) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1803 = {
                    let a1800 = {
                        let a1798 = v1;
                        let a1799 = 4u8;
                        fixed_u8::bit_and(a1798, a1799)
                    };
                    let a1801 = 4u8;
                    if equal(a1800, a1801) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1802, a1803)?
            };
            let a1815 = {
                let a1812 = {
                    let a1806 = {
                        let a1804 = v1;
                        let a1805 = 2u8;
                        fixed_u8::bit_and(a1804, a1805)
                    };
                    let a1807 = 2u8;
                    if equal(a1806, a1807) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1813 = {
                    let a1810 = {
                        let a1808 = v1;
                        let a1809 = 1u8;
                        fixed_u8::bit_and(a1808, a1809)
                    };
                    let a1811 = 1u8;
                    if equal(a1810, a1811) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1812, a1813)?
            };
            nat_add(a1814, a1815)?
        };
        nat_add(a1816, a1817)?
    };
    let a1818 = v2;
    let a1819 = 48u64;
    if nat_lt(a1818, a1819) {
        Ok(None::<u64>)
    } else {
        let a1820 = v2;
        let a1821 = 57u64;
        if nat_le(a1820, a1821) {
            Ok(Some({
                let a1822 = v2;
                let a1823 = 48u64;
                nat_sub(a1822, a1823)
            }))
        } else {
            let a1824 = v2;
            let a1825 = 65u64;
            if nat_lt(a1824, a1825) {
                Ok(None::<u64>)
            } else {
                let a1826 = v2;
                let a1827 = 70u64;
                if nat_le(a1826, a1827) {
                    Ok(Some({
                        let a1828 = v2;
                        let a1829 = 55u64;
                        nat_sub(a1828, a1829)
                    }))
                } else {
                    let a1830 = v2;
                    let a1831 = 97u64;
                    if nat_lt(a1830, a1831) {
                        Ok(None::<u64>)
                    } else {
                        let a1832 = v2;
                        let a1833 = 102u64;
                        if nat_le(a1832, a1833) {
                            Ok(Some({
                                let a1834 = v2;
                                let a1835 = 87u64;
                                nat_sub(a1834, a1835)
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f256(v0: Adt0) -> Option<Adt18> {
    let m1836 = v0;
    match m1836 {
        Adt0::C0(_) => {
            None::<Adt18>
        }
        Adt0::C1(_, _) => {
            None::<Adt18>
        }
        Adt0::C2(_) => {
            None::<Adt18>
        }
        Adt0::C3(_) => {
            None::<Adt18>
        }
        Adt0::C4(_) => {
            None::<Adt18>
        }
        Adt0::C5 => {
            None::<Adt18>
        }
        Adt0::C6(r1837, r1838) => {
            let v1 = (*r1837).clone();
            let v2 = (*r1838).clone();
            Some(Adt18::C0(std::rc::Rc::new(v1), std::rc::Rc::new(v2)))
        }
        Adt0::C7(_, _, _) => {
            None::<Adt18>
        }
    }
}

pub fn f257(v0: u64) -> Bytes {
    let a1839 = v0;
    let a1840 = 34u64;
    if nat_eq(a1839, a1840) {
        f49(Str::lit("\\\""))
    } else {
        let a1841 = v0;
        let a1842 = 92u64;
        if nat_eq(a1841, a1842) {
            f49(Str::lit("\\\\"))
        } else {
            let a1843 = v0;
            let a1844 = 8u64;
            if nat_eq(a1843, a1844) {
                f49(Str::lit("\\b"))
            } else {
                let a1845 = v0;
                let a1846 = 12u64;
                if nat_eq(a1845, a1846) {
                    f49(Str::lit("\\f"))
                } else {
                    let a1847 = v0;
                    let a1848 = 10u64;
                    if nat_eq(a1847, a1848) {
                        f49(Str::lit("\\n"))
                    } else {
                        let a1849 = v0;
                        let a1850 = 13u64;
                        if nat_eq(a1849, a1850) {
                            f49(Str::lit("\\r"))
                        } else {
                            let a1851 = v0;
                            let a1852 = 9u64;
                            if nat_eq(a1851, a1852) {
                                f49(Str::lit("\\t"))
                            } else {
                                f184(f49(Str::lit("\\u00")), f267(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: Adt13, v1: u64, v2: Bytes) -> Adt13 {
    Adt13::C0(match v0.clone() {
        Adt13::C0(h1853, _, _, _, _, _) => {
            h1853
        }
    }, match v0.clone() {
        Adt13::C0(_, h1854, _, _, _, _) => {
            h1854
        }
    }, 9u64, match v0.clone() {
        Adt13::C0(_, _, _, h1855, _, _) => {
            h1855
        }
    }, std::rc::Rc::new(match v0 {
        Adt13::C0(_, _, _, _, h1856, _) => {
            (*h1856).clone()
        }
    }), Some(f268(v1, v2)))
}

pub fn f259(v0: Adt13, v1: u64, v2: Adt0) -> Adt13 {
    Adt13::C0(v1, match v0.clone() {
        Adt13::C0(_, h1857, _, _, _, _) => {
            h1857
        }
    }, 3u64, match v0 {
        Adt13::C0(_, _, _, h1858, _, _) => {
            h1858
        }
    }, std::rc::Rc::new(v2), None::<Adt1>)
}

pub fn f260(v0: Adt13, v1: u64, v2: Adt14, v3: u64) -> R<Adt13> {
    Ok(Adt13::C0(v1, {
        let a1860 = match v0.clone() {
            Adt13::C0(_, h1859, _, _, _, _) => {
                h1859
            }
        };
        let a1861 = 1u64;
        nat_add(a1860, a1861)?
    }, v3, List::cons(v2, match v0 {
        Adt13::C0(_, _, _, h1862, _, _) => {
            h1862
        }
    }), std::rc::Rc::new(Adt0::C5), None::<Adt1>))
}

pub fn f261(v0: List<Adt5>, _: u64, v2: Adt13, v3: u64) -> R<Adt13> {
    let v4: u64 = match v2.clone() {
        Adt13::C0(h1863, _, _, _, _, _) => {
            h1863
        }
    };
    let v5: Bytes = f151(v0.clone(), v4);
    let a1864 = v5.clone();
    let a1865 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a1864, a1865) {
        Ok(f259(v2, {
            let a1866 = v4;
            let a1867 = 1u64;
            nat_add(a1866, a1867)?
        }, Adt0::C0(true)))
    } else {
        let a1868 = v5.clone();
        let a1869 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a1868, a1869) {
            Ok(f259(v2, {
                let a1870 = v4;
                let a1871 = 1u64;
                nat_add(a1870, a1871)?
            }, Adt0::C0(false)))
        } else {
            if f269(v5.clone())? {
                let m1872 = f270(v5.clone())?;
                match m1872 {
                    None => {
                        Ok(f258(v2, v3, f200(f200(f163(Str::lit("`")), v5), f163(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f259(v2, {
                            let a1873 = v4;
                            let a1874 = 1u64;
                            nat_add(a1873, a1874)?
                        }, v6))
                    }
                }
            } else {
                if f103(v5.clone())? {
                    let a1877 = f149(v0, {
                        let a1875 = v4;
                        let a1876 = 1u64;
                        nat_add(a1875, a1876)?
                    });
                    let a1878 = 0u64;
                    if nat_eq(a1877, a1878) {
                        f260(v2, {
                            let a1879 = v4;
                            let a1880 = 2u64;
                            nat_add(a1879, a1880)?
                        }, Adt14::C1(Bytes::lit(&[]), 0u64, List::cons((f163(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f259(v2, {
                            let a1881 = v4;
                            let a1882 = 1u64;
                            nat_add(a1881, a1882)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f258(v2, v3, f200(f200(f163(Str::lit("`")), v5), f163(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f262(v0: Adt13) -> R<Adt13> {
    let m1884 = match v0.clone() {
        Adt13::C0(_, _, _, h1883, _, _) => {
            h1883
        }
    };
    match m1884.uncons() {
        None => {
            Ok(f258(v0, 0u64, f163(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m1885 = v1;
            match m1885 {
                Adt14::C0(v3) => {
                    Ok(Adt13::C0({
                        let a1887 = match v0.clone() {
                            Adt13::C0(h1886, _, _, _, _, _) => {
                                h1886
                            }
                        };
                        let a1888 = 1u64;
                        nat_add(a1887, a1888)?
                    }, {
                        let a1890 = match v0 {
                            Adt13::C0(_, h1889, _, _, _, _) => {
                                h1889
                            }
                        };
                        let a1891 = 1u64;
                        nat_sub(a1890, a1891)
                    }, 3u64, v2, std::rc::Rc::new(Adt0::C3(std::rc::Rc::new(f25(v3)))), None::<Adt1>))
                }
                Adt14::C1(_, _, v6) => {
                    Ok(Adt13::C0({
                        let a1893 = match v0.clone() {
                            Adt13::C0(h1892, _, _, _, _, _) => {
                                h1892
                            }
                        };
                        let a1894 = 1u64;
                        nat_add(a1893, a1894)?
                    }, {
                        let a1896 = match v0 {
                            Adt13::C0(_, h1895, _, _, _, _) => {
                                h1895
                            }
                        };
                        let a1897 = 1u64;
                        nat_sub(a1896, a1897)
                    }, 3u64, v2, std::rc::Rc::new(Adt0::C4(std::rc::Rc::new(f175(f176(v6)?)))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f263(v0: List<Adt5>, v1: u64, v2: Adt13, v3: List<(Bytes, Adt0)>, v4: List<Adt14>) -> R<Adt13> {
    let v5: u64 = match v2.clone() {
        Adt13::C0(h1898, _, _, _, _, _) => {
            h1898
        }
    };
    let v6: u64 = f149(v0.clone(), v5);
    let v7: Bytes = f151(v0.clone(), v5);
    let a1899 = v6;
    let a1900 = 11u64;
    if if nat_eq(a1899, a1900) {
        true
    } else {
        let a1901 = v6;
        let a1902 = 12u64;
        if nat_eq(a1901, a1902) {
            f103(v7.clone())?
        } else {
            false
        }
    } {
        let a1905 = f149(v0.clone(), {
            let a1903 = v5;
            let a1904 = 1u64;
            nat_add(a1903, a1904)?
        });
        let a1906 = 7u64;
        if nat_eq(a1905, a1906) {
            Ok(Adt13::C0({
                let a1907 = v5;
                let a1908 = 2u64;
                nat_add(a1907, a1908)?
            }, match v2 {
                Adt13::C0(_, h1909, _, _, _, _) => {
                    h1909
                }
            }, 0u64, List::cons(Adt14::C1(v7, f16(v0, v1, v5), v3), v4), std::rc::Rc::new(Adt0::C5), None::<Adt1>))
        } else {
            Ok(f258(v2, f16(v0, v1, {
                let a1910 = v5;
                let a1911 = 1u64;
                nat_add(a1910, a1911)?
            }), f163(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f258(v2, f16(v0, v1, v5), f163(Str::lit("expected a member name"))))
    }
}

pub fn f264(v0: List<Adt5>, v1: Adt13, v2: List<Adt14>, v3: u64, v4: u64) -> R<Adt13> {
    let a1913 = f149(v0, match v1.clone() {
        Adt13::C0(h1912, _, _, _, _, _) => {
            h1912
        }
    });
    let a1914 = 6u64;
    if nat_eq(a1913, a1914) {
        Ok(Adt13::C0({
            let a1916 = match v1.clone() {
                Adt13::C0(h1915, _, _, _, _, _) => {
                    h1915
                }
            };
            let a1917 = 1u64;
            nat_add(a1916, a1917)?
        }, match v1 {
            Adt13::C0(_, h1918, _, _, _, _) => {
                h1918
            }
        }, v3, v2, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    } else {
        Ok(Adt13::C0(match v1.clone() {
            Adt13::C0(h1919, _, _, _, _, _) => {
                h1919
            }
        }, match v1 {
            Adt13::C0(_, h1920, _, _, _, _) => {
                h1920
            }
        }, v4, v2, std::rc::Rc::new(Adt0::C5), None::<Adt1>))
    }
}

pub fn f265(v0: Fn27, p1925: u64, p1926: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m1924 = (p1925, p1926);
    {
        let mut m1924 = m1924;
        loop {
            let step = {
                let (v1, v2) = m1924;
                let m1921 = v1;
                if m1921 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1921 - 1;
                    let m1923 = {
                        let c1922: Fn27 = v0.clone();
                        c1922.apply(v2.clone())?
                    };
                    match m1923 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f271(v0)
}

pub fn f267(v0: u64) -> Bytes {
    f184(f240(v0, 16u64), f241(v0))
}

pub fn f268(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f269(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1927 = f153(v0.clone(), 0u64)?;
        let a1928 = 45u64;
        if nat_eq(a1927, a1928) {
            1u64
        } else {
            0u64
        }
    };
    let a1930 = v1;
    let a1931 = {
        let a1929 = v0.clone();
        length_bytes(a1929)
    };
    if nat_lt(a1930, a1931) {
        let a1933 = f156(v0.clone(), v1)?;
        let a1934 = {
            let a1932 = v0;
            length_bytes(a1932)
        };
        Ok(nat_eq(a1933, a1934))
    } else {
        Ok(false)
    }
}

pub fn f270(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a1935 = f153(v0.clone(), 0u64)?;
        let a1936 = 45u64;
        nat_eq(a1935, a1936)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f272(v0.clone(), v2)?;
    let v4: Bytes = f51(v0.clone(), v3, {
        let a1938 = {
            let a1937 = v0;
            length_bytes(a1937)
        };
        let a1939 = v3;
        nat_sub(a1938, a1939)
    });
    let a1941 = {
        let a1940 = v4.clone();
        length_bytes(a1940)
    };
    let a1942 = 0u64;
    if nat_eq(a1941, a1942) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a1944 = 19u64;
        let a1945 = {
            let a1943 = v4.clone();
            length_bytes(a1943)
        };
        if nat_lt(a1944, a1945) {
            Ok(None::<Adt0>)
        } else {
            let m1946 = f273(v4)?;
            match m1946 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f274(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f271(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h1947, _) = v0;
        h1947
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h1948) = v0;
        h1948
    };
    let m1949 = v1;
    match m1949.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m1950 = v4;
            match m1950.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f275(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h1951, _) = f65(Fn8::F276(v0.clone()), {
        let a1953 = {
            let a1952 = v0;
            length_bytes(a1952)
        };
        let a1954 = 1u64;
        nat_add(a1953, a1954)?
    }, v1)?;
    Ok(h1951)
}

pub fn f273(v0: Bytes) -> R<Option<u64>> {
    if f277(v0.clone())? {
        Ok(Some({
            let (_, h1955) = {
                let (h1956, _) = f278(Fn28::F279(v0.clone()), {
                    let a1957 = v0;
                    length_bytes(a1957)
                }, (0u64, 0u64))?;
                h1956
            };
            h1955
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f274(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1958 = 0u64;
        let a1959 = v1;
        if nat_lt(a1958, a1959) {
            let a1960 = v1;
            let a1961 = 9223372036854775808u64;
            nat_le(a1960, a1961)
        } else {
            false
        }
    } else {
        let a1962 = v1;
        let a1963 = 9223372036854775807u64;
        nat_le(a1962, a1963)
    }
}

pub fn f275(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a1968 = {
            let a1966 = {
                let a1964 = v0.clone();
                length_list(a1964)
            };
            let a1967 = {
                let a1965 = v1.clone();
                length_list(a1965)
            };
            nat_add(a1966, a1967)?
        };
        let a1969 = 1u64;
        nat_add(a1968, a1969)?
    };
    Ok(f210({
        let (_, h1970) = {
            let (_, h1971) = {
                let (h1972, _) = f280(Fn29::F281, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h1972
            };
            h1971
        };
        h1970
    }))
}

pub fn f276(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f282(v0, v1)
}

pub fn f277(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1973 = v0.clone();
        length_bytes(a1973)
    };
    let a1974 = 0u64;
    let a1975 = v1;
    if nat_lt(a1974, a1975) {
        let a1976 = f283(v0.clone())?;
        let a1977 = v1;
        if nat_eq(a1976, a1977) {
            let a1978 = 1u64;
            let a1979 = v1;
            if nat_lt(a1978, a1979) {
                let m1982 = {
                    let a1980 = v0;
                    let a1981 = 0u64;
                    index_bytes(a1980, a1981)
                };
                match m1982 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1983 = v2;
                        let a1984 = 48u8;
                        Ok(!equal(a1983, a1984))
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

pub fn f278(v0: Fn28, p1989: u64, p1990: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1988 = (p1989, p1990);
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let (v1, v2) = m1988;
                let m1985 = v1;
                if m1985 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1985 - 1;
                    let m1987 = {
                        let c1986: Fn28 = v0.clone();
                        c1986.apply(v2)?
                    };
                    match m1987 {
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
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f284(v0, v1)
}

pub fn f280(v0: Fn29, p1995: u64, p1996: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m1994 = (p1995, p1996);
    {
        let mut m1994 = m1994;
        loop {
            let step = {
                let (v1, v2) = m1994;
                let m1991 = v1;
                if m1991 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m1991 - 1;
                    let m1993 = {
                        let c1992: Fn29 = v0.clone();
                        c1992.apply(v2.clone())
                    };
                    match m1993 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f285(v0)
}

pub fn f282(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1997 = f153(v0, v1)?;
    let a1998 = 48u64;
    if nat_eq(a1997, a1998) {
        Ok(Some({
            let a1999 = v1;
            let a2000 = 1u64;
            nat_add(a1999, a2000)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f283(v0: Bytes) -> R<u64> {
    let (h2001, _) = f65(Fn8::F286(v0.clone()), {
        let a2002 = v0;
        length_bytes(a2002)
    }, 0u64)?;
    Ok(h2001)
}

pub fn f284(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2006 = {
        let a2004 = v0;
        let a2005 = {
            let (h2003, _) = v1;
            h2003
        };
        index_bytes(a2004, a2005)
    };
    match m2006 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2008 = {
                    let (h2007, _) = v1;
                    h2007
                };
                let a2009 = 1u64;
                nat_add(a2008, a2009)?
            }, {
                let a2013 = {
                    let a2011 = {
                        let (_, h2010) = v1;
                        h2010
                    };
                    let a2012 = 10u64;
                    nat_mul(a2011, a2012)?
                };
                let a2014 = f287(v2)?;
                nat_add(a2013, a2014)?
            })))
        }
    }
}

pub fn f285(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h2015, _) = v0.clone();
        h2015
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h2016, _) = {
            let (_, h2017) = v0.clone();
            h2017
        };
        h2016
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h2018) = {
            let (_, h2019) = v0;
            h2019
        };
        h2018
    };
    let m2020 = v1.clone();
    match m2020.uncons() {
        None => {
            let m2021 = v2;
            match m2021.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2022 = v2.clone();
            match m2022.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f288({
                        let (h2023, _) = v8.clone();
                        h2023
                    }, {
                        let (h2024, _) = v6.clone();
                        h2024
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

pub fn f286(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2027 = {
        let a2025 = v0;
        let a2026 = v1;
        index_bytes(a2025, a2026)
    };
    match m2027 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f289(v2)? {
                Ok(Some({
                    let a2028 = v1;
                    let a2029 = 1u64;
                    nat_add(a2028, a2029)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f287(v0: u8) -> R<u64> {
    let a2076 = {
        let v1: u8 = v0;
        let a2074 = {
            let a2050 = {
                let a2038 = {
                    let a2032 = {
                        let a2030 = v1;
                        let a2031 = 128u8;
                        fixed_u8::bit_and(a2030, a2031)
                    };
                    let a2033 = 128u8;
                    if equal(a2032, a2033) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2039 = {
                    let a2036 = {
                        let a2034 = v1;
                        let a2035 = 64u8;
                        fixed_u8::bit_and(a2034, a2035)
                    };
                    let a2037 = 64u8;
                    if equal(a2036, a2037) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2038, a2039)?
            };
            let a2051 = {
                let a2048 = {
                    let a2042 = {
                        let a2040 = v1;
                        let a2041 = 32u8;
                        fixed_u8::bit_and(a2040, a2041)
                    };
                    let a2043 = 32u8;
                    if equal(a2042, a2043) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2049 = {
                    let a2046 = {
                        let a2044 = v1;
                        let a2045 = 16u8;
                        fixed_u8::bit_and(a2044, a2045)
                    };
                    let a2047 = 16u8;
                    if equal(a2046, a2047) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2048, a2049)?
            };
            nat_add(a2050, a2051)?
        };
        let a2075 = {
            let a2072 = {
                let a2060 = {
                    let a2054 = {
                        let a2052 = v1;
                        let a2053 = 8u8;
                        fixed_u8::bit_and(a2052, a2053)
                    };
                    let a2055 = 8u8;
                    if equal(a2054, a2055) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2061 = {
                    let a2058 = {
                        let a2056 = v1;
                        let a2057 = 4u8;
                        fixed_u8::bit_and(a2056, a2057)
                    };
                    let a2059 = 4u8;
                    if equal(a2058, a2059) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2060, a2061)?
            };
            let a2073 = {
                let a2070 = {
                    let a2064 = {
                        let a2062 = v1;
                        let a2063 = 2u8;
                        fixed_u8::bit_and(a2062, a2063)
                    };
                    let a2065 = 2u8;
                    if equal(a2064, a2065) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2071 = {
                    let a2068 = {
                        let a2066 = v1;
                        let a2067 = 1u8;
                        fixed_u8::bit_and(a2066, a2067)
                    };
                    let a2069 = 1u8;
                    if equal(a2068, a2069) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2070, a2071)?
            };
            nat_add(a2072, a2073)?
        };
        nat_add(a2074, a2075)?
    };
    let a2077 = 48u64;
    Ok(nat_sub(a2076, a2077))
}

pub fn f288(v0: Bytes, v1: Bytes) -> bool {
    let a2082 = {
        let a2078 = v0;
        let a2079 = v1;
        compare_bytes(a2078, a2079)
    };
    let a2083 = {
        let a2080 = Bytes::lit(&[]);
        let a2081 = Bytes::lit(&[0x01]);
        compare_bytes(a2080, a2081)
    };
    equal(a2082, a2083)
}

pub fn f289(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2128 = {
            let a2104 = {
                let a2092 = {
                    let a2086 = {
                        let a2084 = v1;
                        let a2085 = 128u8;
                        fixed_u8::bit_and(a2084, a2085)
                    };
                    let a2087 = 128u8;
                    if equal(a2086, a2087) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2093 = {
                    let a2090 = {
                        let a2088 = v1;
                        let a2089 = 64u8;
                        fixed_u8::bit_and(a2088, a2089)
                    };
                    let a2091 = 64u8;
                    if equal(a2090, a2091) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2092, a2093)?
            };
            let a2105 = {
                let a2102 = {
                    let a2096 = {
                        let a2094 = v1;
                        let a2095 = 32u8;
                        fixed_u8::bit_and(a2094, a2095)
                    };
                    let a2097 = 32u8;
                    if equal(a2096, a2097) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2103 = {
                    let a2100 = {
                        let a2098 = v1;
                        let a2099 = 16u8;
                        fixed_u8::bit_and(a2098, a2099)
                    };
                    let a2101 = 16u8;
                    if equal(a2100, a2101) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2102, a2103)?
            };
            nat_add(a2104, a2105)?
        };
        let a2129 = {
            let a2126 = {
                let a2114 = {
                    let a2108 = {
                        let a2106 = v1;
                        let a2107 = 8u8;
                        fixed_u8::bit_and(a2106, a2107)
                    };
                    let a2109 = 8u8;
                    if equal(a2108, a2109) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2115 = {
                    let a2112 = {
                        let a2110 = v1;
                        let a2111 = 4u8;
                        fixed_u8::bit_and(a2110, a2111)
                    };
                    let a2113 = 4u8;
                    if equal(a2112, a2113) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2114, a2115)?
            };
            let a2127 = {
                let a2124 = {
                    let a2118 = {
                        let a2116 = v1;
                        let a2117 = 2u8;
                        fixed_u8::bit_and(a2116, a2117)
                    };
                    let a2119 = 2u8;
                    if equal(a2118, a2119) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2125 = {
                    let a2122 = {
                        let a2120 = v1;
                        let a2121 = 1u8;
                        fixed_u8::bit_and(a2120, a2121)
                    };
                    let a2123 = 1u8;
                    if equal(a2122, a2123) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2124, a2125)?
            };
            nat_add(a2126, a2127)?
        };
        nat_add(a2128, a2129)?
    };
    let a2130 = 48u64;
    let a2131 = v2;
    if nat_le(a2130, a2131) {
        let a2132 = v2;
        let a2133 = 57u64;
        Ok(nat_le(a2132, a2133))
    } else {
        Ok(false)
    }
}

pub fn run(p0: Adt0, p1: Bytes) -> R<Result<List<(u64, u64)>, Adt1>> {
    f0(p0, p1)
}
