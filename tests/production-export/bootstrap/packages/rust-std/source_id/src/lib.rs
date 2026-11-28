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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(std::rc::Rc<Adt0>, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11,
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn1::F11 => f11(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn2::F18 => f18(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23(Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn3::F23(k0, k1) => f23(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F35,
}

impl Fn4 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn4::F35 => f35(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F37(u64, Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn5::F37(k0, k1, k2) => f37(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F41,
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn6::F41 => f41(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F44,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn7::F44 => f44(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> R<Bytes> {
    f1(f2(Str::lit("lexlean-source-v2")), f3(f3(f4(f2(Str::lit("project")), v0), f4(f2(Str::lit("lock")), v1)), f5(Fn0::F6, Bytes::lit(&[]), v2)))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f7(f3(f3(v0, Bytes::lit(&[0x00])), v1))
}

pub fn f2(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    let a2 = v0;
    let a3 = v1;
    append_bytes(a2, a3)
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    f3(f3(f3(f8({
        let a4 = v0.clone();
        length_bytes(a4)
    }, 4u64), v0), f8({
        let a5 = v1.clone();
        length_bytes(a5)
    }, 8u64)), v1)
}

pub fn f5(v0: Fn0, p9: Bytes, p10: List<(Bytes, Bytes)>) -> Bytes {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    f3(v0, f3(f4(f2(Str::lit("path")), {
        let (h11, _) = v1;
        h11
    }), f4(f2(Str::lit("source")), {
        let (_, h12) = v1;
        h12
    })))
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    let a14 = {
        let a13 = Str::lit("blake3:");
        utf8_encode(a13)
    };
    let a15 = f9(v0)?;
    Ok(append_bytes(a14, a15))
}

pub fn f8(v0: u64, v1: u64) -> Bytes {
    let (_, h16) = f10(Fn1::F11, v1, (v0, Bytes::lit(&[])));
    h16
}

pub fn f9(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f12(v0)?;
    let a37 = f13(match v1.clone() {
        Adt0::C0(h17, _, _, _, _, _, _, _) => {
            h17
        }
    });
    let a38 = {
        let a35 = f13(match v1.clone() {
            Adt0::C0(_, h18, _, _, _, _, _, _) => {
                h18
            }
        });
        let a36 = {
            let a33 = f13(match v1.clone() {
                Adt0::C0(_, _, h19, _, _, _, _, _) => {
                    h19
                }
            });
            let a34 = {
                let a31 = f13(match v1.clone() {
                    Adt0::C0(_, _, _, h20, _, _, _, _) => {
                        h20
                    }
                });
                let a32 = {
                    let a29 = f13(match v1.clone() {
                        Adt0::C0(_, _, _, _, h21, _, _, _) => {
                            h21
                        }
                    });
                    let a30 = {
                        let a27 = f13(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h22, _, _) => {
                                h22
                            }
                        });
                        let a28 = {
                            let a25 = f13(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h23, _) => {
                                    h23
                                }
                            });
                            let a26 = f13(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h24) => {
                                    h24
                                }
                            });
                            append_bytes(a25, a26)
                        };
                        append_bytes(a27, a28)
                    };
                    append_bytes(a29, a30)
                };
                append_bytes(a31, a32)
            };
            append_bytes(a33, a34)
        };
        append_bytes(a35, a36)
    };
    Ok(append_bytes(a37, a38))
}

pub fn f10(v0: Fn1, p42: u64, p43: (u64, Bytes)) -> (u64, Bytes) {
    let m41 = (p42, p43);
    {
        let mut m41 = m41;
        loop {
            let step = {
                let (v1, v2) = m41;
                let m39 = v1;
                if m39 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m39 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c40: Fn1 = v0.clone();
                        c40.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m41 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a45 = {
            let (h44, _) = v0.clone();
            h44
        };
        let a46 = 256u64;
        let a47 = 0u64;
        nat_quot(a45, a46, a47)
    }, f3(f14({
        let a49 = {
            let (h48, _) = v0;
            h48
        };
        let a50 = 256u64;
        let a51 = 0u64;
        nat_rem(a49, a50, a51)
    }), {
        let (_, h52) = v0;
        h52
    }))
}

pub fn f12(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a53 = v0.clone();
        length_bytes(a53)
    };
    let v2: u64 = {
        let a54 = v1;
        let a55 = 0u64;
        if nat_eq(a54, a55) {
            1u64
        } else {
            let a58 = {
                let a56 = v1;
                let a57 = 1023u64;
                nat_add(a56, a57)?
            };
            let a59 = 1024u64;
            let a60 = 0u64;
            nat_quot(a58, a59, a60)
        }
    };
    let v3: u64 = {
        let a61 = v2;
        let a62 = 1u64;
        nat_sub(a61, a62)
    };
    let m64 = {
        let (_, h63) = f15(v0.clone(), v1, v3)?;
        h63
    };
    match m64.uncons() {
        None => {
            f16(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h65) = f17(Fn2::F18, 64u64, (List::cons(v4, v5), f16(v0, v1, v3, false)?));
            Ok(h65)
        }
    }
}

pub fn f13(v0: u32) -> Bytes {
    let a78 = {
        let a66 = f19(v0, 4u32);
        let a67 = f19(v0, 0u32);
        append_bytes(a66, a67)
    };
    let a79 = {
        let a76 = {
            let a68 = f19(v0, 12u32);
            let a69 = f19(v0, 8u32);
            append_bytes(a68, a69)
        };
        let a77 = {
            let a74 = {
                let a70 = f19(v0, 20u32);
                let a71 = f19(v0, 16u32);
                append_bytes(a70, a71)
            };
            let a75 = {
                let a72 = f19(v0, 28u32);
                let a73 = f19(v0, 24u32);
                append_bytes(a72, a73)
            };
            append_bytes(a74, a75)
        };
        append_bytes(a76, a77)
    };
    append_bytes(a78, a79)
}

pub fn f14(v0: u64) -> Bytes {
    f20(f21(), v0)
}

pub fn f15(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f22(Fn3::F23(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f16(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a80 = 1024u64;
        let a81 = v2;
        nat_mul(a80, a81)?
    };
    let v5: u64 = f24(v1, v4);
    let v6: u64 = f25(v5)?;
    let v7: u64 = {
        let a82 = v6;
        let a83 = 1u64;
        nat_sub(a82, a83)
    };
    Ok(f26(f27(v0.clone(), v4, v2, v7)?, f28(v0, {
        let a86 = v4;
        let a87 = {
            let a84 = 64u64;
            let a85 = v7;
            nat_mul(a84, a85)?
        };
        nat_add(a86, a87)?
    })?, v2, f29({
        let a90 = v5;
        let a91 = {
            let a88 = 64u64;
            let a89 = v7;
            nat_mul(a88, a89)?
        };
        nat_sub(a90, a91)
    }), {
        let a94 = {
            let a92 = v7;
            let a93 = 0u64;
            if nat_eq(a92, a93) {
                3u32
            } else {
                2u32
            }
        };
        let a95 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a94, a95)
    }))
}

pub fn f17(v0: Fn2, p99: u64, p100: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m98 = (p99, p100);
    {
        let mut m98 = m98;
        loop {
            let step = {
                let (v1, v2) = m98;
                let m96 = v1;
                if m96 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m96 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c97: Fn2 = v0.clone();
                        c97.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m98 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f30(v0)
}

pub fn f19(v0: u32, v1: u32) -> Bytes {
    f31(f32(v0, v1))
}

pub fn f20(v0: Bytes, v1: u64) -> Bytes {
    let m104 = {
        let a101 = v0;
        let a102 = v1;
        let a103 = 1u64;
        slice_bytes(a101, a102, a103)
    };
    match m104 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f21() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f22(v0: Fn3, p108: u64, p109: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m107 = (p108, p109);
    {
        let mut m107 = m107;
        loop {
            let step = {
                let (v1, v2) = m107;
                let m105 = v1;
                if m105 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m105 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c106: Fn3 = v0.clone();
                        c106.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f33(v2.clone(), f16(v0, v1, {
        let (h110, _) = v2;
        h110
    }, false)?)
}

pub fn f24(v0: u64, v1: u64) -> u64 {
    let a113 = 1024u64;
    let a114 = {
        let a111 = v0;
        let a112 = v1;
        nat_sub(a111, a112)
    };
    if nat_lt(a113, a114) {
        1024u64
    } else {
        let a115 = v0;
        let a116 = v1;
        nat_sub(a115, a116)
    }
}

pub fn f25(v0: u64) -> R<u64> {
    let a117 = v0;
    let a118 = 0u64;
    if nat_eq(a117, a118) {
        Ok(1u64)
    } else {
        let a121 = {
            let a119 = v0;
            let a120 = 63u64;
            nat_add(a119, a120)?
        };
        let a122 = 64u64;
        let a123 = 0u64;
        Ok(nat_quot(a121, a122, a123))
    }
}

pub fn f26(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h124, _) = f34(Fn4::F35, 7u64, (Adt2::C0(match v0.clone() {
            Adt0::C0(h125, _, _, _, _, _, _, _) => {
                h125
            }
        }, match v0.clone() {
            Adt0::C0(_, h126, _, _, _, _, _, _) => {
                h126
            }
        }, match v0.clone() {
            Adt0::C0(_, _, h127, _, _, _, _, _) => {
                h127
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, h128, _, _, _, _) => {
                h128
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, h129, _, _, _) => {
                h129
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, h130, _, _) => {
                h130
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h131, _) => {
                h131
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h132) => {
                h132
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f29({
            let a133 = v2;
            let a134 = 4294967296u64;
            let a135 = 0u64;
            nat_rem(a133, a134, a135)
        }), f29({
            let a136 = v2;
            let a137 = 4294967296u64;
            let a138 = 0u64;
            nat_quot(a136, a137, a138)
        }), v3, v4), v1));
        h124
    };
    Adt0::C0({
        let a141 = match v5.clone() {
            Adt2::C0(h139, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h139
            }
        };
        let a142 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h140, _, _, _, _, _, _, _) => {
                h140
            }
        };
        fixed_u32::bit_xor(a141, a142)
    }, {
        let a145 = match v5.clone() {
            Adt2::C0(_, h143, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h143
            }
        };
        let a146 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h144, _, _, _, _, _, _) => {
                h144
            }
        };
        fixed_u32::bit_xor(a145, a146)
    }, {
        let a149 = match v5.clone() {
            Adt2::C0(_, _, h147, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        };
        let a150 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h148, _, _, _, _, _) => {
                h148
            }
        };
        fixed_u32::bit_xor(a149, a150)
    }, {
        let a153 = match v5.clone() {
            Adt2::C0(_, _, _, h151, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h151
            }
        };
        let a154 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h152, _, _, _, _) => {
                h152
            }
        };
        fixed_u32::bit_xor(a153, a154)
    }, {
        let a157 = match v5.clone() {
            Adt2::C0(_, _, _, _, h155, _, _, _, _, _, _, _, _, _, _, _) => {
                h155
            }
        };
        let a158 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h156, _, _, _) => {
                h156
            }
        };
        fixed_u32::bit_xor(a157, a158)
    }, {
        let a161 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, h159, _, _, _, _, _, _, _, _, _, _) => {
                h159
            }
        };
        let a162 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h160, _, _) => {
                h160
            }
        };
        fixed_u32::bit_xor(a161, a162)
    }, {
        let a165 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, h163, _, _, _, _, _, _, _, _, _) => {
                h163
            }
        };
        let a166 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h164, _) => {
                h164
            }
        };
        fixed_u32::bit_xor(a165, a166)
    }, {
        let a169 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h167, _, _, _, _, _, _, _, _) => {
                h167
            }
        };
        let a170 = match v5 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h168) => {
                h168
            }
        };
        fixed_u32::bit_xor(a169, a170)
    })
}

pub fn f27(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h171) = f36(Fn5::F37(v2, v0, v1), v3, (0u64, f38()))?;
    Ok(h171)
}

pub fn f28(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(f39(v0.clone(), {
        let a172 = v1;
        let a173 = 0u64;
        nat_add(a172, a173)?
    })?, f39(v0.clone(), {
        let a174 = v1;
        let a175 = 4u64;
        nat_add(a174, a175)?
    })?, f39(v0.clone(), {
        let a176 = v1;
        let a177 = 8u64;
        nat_add(a176, a177)?
    })?, f39(v0.clone(), {
        let a178 = v1;
        let a179 = 12u64;
        nat_add(a178, a179)?
    })?, f39(v0.clone(), {
        let a180 = v1;
        let a181 = 16u64;
        nat_add(a180, a181)?
    })?, f39(v0.clone(), {
        let a182 = v1;
        let a183 = 20u64;
        nat_add(a182, a183)?
    })?, f39(v0.clone(), {
        let a184 = v1;
        let a185 = 24u64;
        nat_add(a184, a185)?
    })?, f39(v0.clone(), {
        let a186 = v1;
        let a187 = 28u64;
        nat_add(a186, a187)?
    })?, f39(v0.clone(), {
        let a188 = v1;
        let a189 = 32u64;
        nat_add(a188, a189)?
    })?, f39(v0.clone(), {
        let a190 = v1;
        let a191 = 36u64;
        nat_add(a190, a191)?
    })?, f39(v0.clone(), {
        let a192 = v1;
        let a193 = 40u64;
        nat_add(a192, a193)?
    })?, f39(v0.clone(), {
        let a194 = v1;
        let a195 = 44u64;
        nat_add(a194, a195)?
    })?, f39(v0.clone(), {
        let a196 = v1;
        let a197 = 48u64;
        nat_add(a196, a197)?
    })?, f39(v0.clone(), {
        let a198 = v1;
        let a199 = 52u64;
        nat_add(a198, a199)?
    })?, f39(v0.clone(), {
        let a200 = v1;
        let a201 = 56u64;
        nat_add(a200, a201)?
    })?, f39(v0, {
        let a202 = v1;
        let a203 = 60u64;
        nat_add(a202, a203)?
    })?))
}

pub fn f29(v0: u64) -> u32 {
    let (_, h204) = {
        let (_, h205) = f40(Fn6::F41, 32u64, (v0, (1u32, 0u32)));
        h205
    };
    h204
}

pub fn f30(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m207 = {
        let (h206, _) = v0.clone();
        h206
    };
    match m207.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f42(v1, {
                let (_, h208) = v0;
                h208
            }, {
                let m209 = v2;
                match m209.uncons() {
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

pub fn f31(v0: u32) -> Bytes {
    let a212 = {
        let a210 = v0;
        let a211 = 8u32;
        fixed_u32::bit_and(a210, a211)
    };
    let a213 = 0u32;
    if equal(a212, a213) {
        let a216 = {
            let a214 = v0;
            let a215 = 4u32;
            fixed_u32::bit_and(a214, a215)
        };
        let a217 = 0u32;
        if equal(a216, a217) {
            let a220 = {
                let a218 = v0;
                let a219 = 2u32;
                fixed_u32::bit_and(a218, a219)
            };
            let a221 = 0u32;
            if equal(a220, a221) {
                let a224 = {
                    let a222 = v0;
                    let a223 = 1u32;
                    fixed_u32::bit_and(a222, a223)
                };
                let a225 = 0u32;
                if equal(a224, a225) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a228 = {
                    let a226 = v0;
                    let a227 = 1u32;
                    fixed_u32::bit_and(a226, a227)
                };
                let a229 = 0u32;
                if equal(a228, a229) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a232 = {
                let a230 = v0;
                let a231 = 2u32;
                fixed_u32::bit_and(a230, a231)
            };
            let a233 = 0u32;
            if equal(a232, a233) {
                let a236 = {
                    let a234 = v0;
                    let a235 = 1u32;
                    fixed_u32::bit_and(a234, a235)
                };
                let a237 = 0u32;
                if equal(a236, a237) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a240 = {
                    let a238 = v0;
                    let a239 = 1u32;
                    fixed_u32::bit_and(a238, a239)
                };
                let a241 = 0u32;
                if equal(a240, a241) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a244 = {
            let a242 = v0;
            let a243 = 4u32;
            fixed_u32::bit_and(a242, a243)
        };
        let a245 = 0u32;
        if equal(a244, a245) {
            let a248 = {
                let a246 = v0;
                let a247 = 2u32;
                fixed_u32::bit_and(a246, a247)
            };
            let a249 = 0u32;
            if equal(a248, a249) {
                let a252 = {
                    let a250 = v0;
                    let a251 = 1u32;
                    fixed_u32::bit_and(a250, a251)
                };
                let a253 = 0u32;
                if equal(a252, a253) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a256 = {
                    let a254 = v0;
                    let a255 = 1u32;
                    fixed_u32::bit_and(a254, a255)
                };
                let a257 = 0u32;
                if equal(a256, a257) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a260 = {
                let a258 = v0;
                let a259 = 2u32;
                fixed_u32::bit_and(a258, a259)
            };
            let a261 = 0u32;
            if equal(a260, a261) {
                let a264 = {
                    let a262 = v0;
                    let a263 = 1u32;
                    fixed_u32::bit_and(a262, a263)
                };
                let a265 = 0u32;
                if equal(a264, a265) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a268 = {
                    let a266 = v0;
                    let a267 = 1u32;
                    fixed_u32::bit_and(a266, a267)
                };
                let a269 = 0u32;
                if equal(a268, a269) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f32(v0: u32, v1: u32) -> u32 {
    let m272 = {
        let a270 = v0;
        let a271 = v1;
        fixed_u32::shift_right(a270, a271)
    };
    match m272 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f33(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f43(Fn7::F44, 40u64, Adt3::C0(std::rc::Rc::new(v1), {
        let a274 = {
            let (h273, _) = v0.clone();
            h273
        };
        let a275 = 1u64;
        nat_add(a274, a275)?
    }, {
        let (_, h276) = v0;
        h276
    }));
    Ok(({
        let a278 = {
            let (h277, _) = v0;
            h277
        };
        let a279 = 1u64;
        nat_add(a278, a279)?
    }, List::cons(match v2.clone() {
        Adt3::C0(h280, _, _) => {
            (*h280).clone()
        }
    }, match v2 {
        Adt3::C0(_, _, h281) => {
            h281
        }
    })))
}

pub fn f34(v0: Fn4, p285: u64, p286: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m282 = v1;
                if m282 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m282 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c283: Fn4 = v0.clone();
                        c283.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f45(v0)
}

pub fn f36(v0: Fn5, p290: u64, p291: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m289 = (p290, p291);
    {
        let mut m289 = m289;
        loop {
            let step = {
                let (v1, v2) = m289;
                let m287 = v1;
                if m287 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m287 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c288: Fn5 = v0.clone();
                        c288.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a293 = {
            let (h292, _) = v3.clone();
            h292
        };
        let a294 = 1u64;
        nat_add(a293, a294)?
    }, f26({
        let (_, h295) = v3.clone();
        h295
    }, f28(v1, {
        let a299 = v2;
        let a300 = {
            let a297 = 64u64;
            let a298 = {
                let (h296, _) = v3.clone();
                h296
            };
            nat_mul(a297, a298)?
        };
        nat_add(a299, a300)?
    })?, v0, 64u32, {
        let a302 = {
            let (h301, _) = v3;
            h301
        };
        let a303 = 0u64;
        if nat_eq(a302, a303) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f38() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f39(v0: Bytes, v1: u64) -> R<u32> {
    let a314 = f46(v0.clone(), v1);
    let a315 = {
        let a312 = f47(f46(v0.clone(), {
            let a304 = v1;
            let a305 = 1u64;
            nat_add(a304, a305)?
        }), 8u32);
        let a313 = {
            let a310 = f47(f46(v0.clone(), {
                let a306 = v1;
                let a307 = 2u64;
                nat_add(a306, a307)?
            }), 16u32);
            let a311 = f47(f46(v0, {
                let a308 = v1;
                let a309 = 3u64;
                nat_add(a308, a309)?
            }), 24u32);
            fixed_u32::bit_or(a310, a311)
        };
        fixed_u32::bit_or(a312, a313)
    };
    Ok(fixed_u32::bit_or(a314, a315))
}

pub fn f40(v0: Fn6, p319: u64, p320: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m318 = (p319, p320);
    {
        let mut m318 = m318;
        loop {
            let step = {
                let (v1, v2) = m318;
                let m316 = v1;
                if m316 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m316 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c317: Fn6 = v0.clone();
                        c317.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h321, _) = v0;
        h321
    };
    let v2: u32 = {
        let (h322, _) = {
            let (_, h323) = v0;
            h323
        };
        h322
    };
    let v3: u32 = {
        let (_, h324) = {
            let (_, h325) = v0;
            h325
        };
        h324
    };
    ({
        let a326 = v1;
        let a327 = 2u64;
        let a328 = 0u64;
        nat_quot(a326, a327, a328)
    }, (f47(v2, 1u32), {
        let a332 = {
            let a329 = v1;
            let a330 = 2u64;
            let a331 = 0u64;
            nat_rem(a329, a330, a331)
        };
        let a333 = 1u64;
        if nat_eq(a332, a333) {
            let a334 = v3;
            let a335 = v2;
            fixed_u32::bit_or(a334, a335)
        } else {
            v3
        }
    }))
}

pub fn f42(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f26(f38(), f48(v0, v1), 0u64, 64u32, {
        let a336 = 4u32;
        let a337 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a336, a337)
    })
}

pub fn f43(v0: Fn7, p341: u64, p342: Adt3) -> Adt3 {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v1, v2) = m340;
                let m338 = v1;
                if m338 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m338 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c339: Fn7 = v0.clone();
                        c339.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt3) -> Adt3 {
    f49(v0)
}

pub fn f45(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (f50(f51({
        let (h343, _) = v0.clone();
        h343
    }, {
        let (_, h344) = v0.clone();
        h344
    }), {
        let (_, h345) = v0.clone();
        h345
    }), f52({
        let (_, h346) = v0;
        h346
    }))
}

pub fn f46(v0: Bytes, v1: u64) -> u32 {
    let m349 = {
        let a347 = v0;
        let a348 = v1;
        index_bytes(a347, a348)
    };
    match m349 {
        None => {
            0u32
        }
        Some(v2) => {
            let m351 = {
                let a350 = v2;
                fixed_u32::convert(i128::from(a350))
            };
            match m351 {
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

pub fn f47(v0: u32, v1: u32) -> u32 {
    let m354 = {
        let a352 = v0;
        let a353 = v1;
        fixed_u32::shift_left(a352, a353)
    };
    match m354 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt0::C0(h355, _, _, _, _, _, _, _) => {
            h355
        }
    }, match v0.clone() {
        Adt0::C0(_, h356, _, _, _, _, _, _) => {
            h356
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h357, _, _, _, _, _) => {
            h357
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h358, _, _, _, _) => {
            h358
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h359, _, _, _) => {
            h359
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h360, _, _) => {
            h360
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h361, _) => {
            h361
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h362) => {
            h362
        }
    }, match v1.clone() {
        Adt0::C0(h363, _, _, _, _, _, _, _) => {
            h363
        }
    }, match v1.clone() {
        Adt0::C0(_, h364, _, _, _, _, _, _) => {
            h364
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h365, _, _, _, _, _) => {
            h365
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h366, _, _, _, _) => {
            h366
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h367, _, _, _) => {
            h367
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h368, _, _) => {
            h368
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h369, _) => {
            h369
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h370) => {
            h370
        }
    })
}

pub fn f49(v0: Adt3) -> Adt3 {
    let a375 = {
        let a372 = match v0.clone() {
            Adt3::C0(_, h371, _) => {
                h371
            }
        };
        let a373 = 2u64;
        let a374 = 0u64;
        nat_rem(a372, a373, a374)
    };
    let a376 = 0u64;
    if nat_eq(a375, a376) {
        let m378 = match v0.clone() {
            Adt3::C0(_, _, h377) => {
                h377
            }
        };
        match m378.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt3::C0(std::rc::Rc::new(f42(v1, match v0.clone() {
                    Adt3::C0(h379, _, _) => {
                        (*h379).clone()
                    }
                }, false)), {
                    let a381 = match v0 {
                        Adt3::C0(_, h380, _) => {
                            h380
                        }
                    };
                    let a382 = 2u64;
                    let a383 = 0u64;
                    nat_quot(a381, a382, a383)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f50(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f53(match v0.clone() {
        Adt2::C0(h384, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h385, _, _, _, _, _, _, _, _, _, _) => {
            h385
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h386, _, _, _, _, _) => {
            h386
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h387) => {
            h387
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h388, _, _, _, _, _, _, _) => {
            h388
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h389, _, _, _, _, _, _) => {
            h389
        }
    });
    let v3: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, h390, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h390
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h391, _, _, _, _, _, _, _, _, _) => {
            h391
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h392, _, _, _, _) => {
            h392
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h394, _, _, _, _, _) => {
            h394
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h395, _, _, _, _) => {
            h395
        }
    });
    let v4: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, _, h396, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h396
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h397, _, _, _, _, _, _, _, _) => {
            h397
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h398, _, _, _, _, _, _, _) => {
            h398
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h399, _, _) => {
            h399
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h400, _, _, _) => {
            h400
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h401, _, _) => {
            h401
        }
    });
    let v5: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, _, _, h402, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h402
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h403, _, _, _, _, _, _, _, _, _, _, _) => {
            h403
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h404, _, _, _, _, _, _) => {
            h404
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h405, _) => {
            h405
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h406, _) => {
            h406
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h407) => {
            h407
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h408, _, _, _) => {
            h408
        }
    }, match v3.clone() {
        Adt4::C0(h409, _, _, _) => {
            h409
        }
    }, match v4.clone() {
        Adt4::C0(h410, _, _, _) => {
            h410
        }
    }, match v5.clone() {
        Adt4::C0(h411, _, _, _) => {
            h411
        }
    }, match v5.clone() {
        Adt4::C0(_, h412, _, _) => {
            h412
        }
    }, match v2.clone() {
        Adt4::C0(_, h413, _, _) => {
            h413
        }
    }, match v3.clone() {
        Adt4::C0(_, h414, _, _) => {
            h414
        }
    }, match v4.clone() {
        Adt4::C0(_, h415, _, _) => {
            h415
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h416, _) => {
            h416
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h417, _) => {
            h417
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h418, _) => {
            h418
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h419, _) => {
            h419
        }
    }, match v3 {
        Adt4::C0(_, _, _, h420) => {
            h420
        }
    }, match v4 {
        Adt4::C0(_, _, _, h421) => {
            h421
        }
    }, match v5 {
        Adt4::C0(_, _, _, h422) => {
            h422
        }
    }, match v2 {
        Adt4::C0(_, _, _, h423) => {
            h423
        }
    })
}

pub fn f51(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f53(match v0.clone() {
        Adt2::C0(h424, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h425, _, _, _, _, _, _, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h426, _, _, _, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h427, _, _, _) => {
            h427
        }
    }, match v1.clone() {
        Adt1::C0(h428, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    }, match v1.clone() {
        Adt1::C0(_, h429, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h429
        }
    });
    let v3: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, h430, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h431, _, _, _, _, _, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h432, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h433, _, _) => {
            h433
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h434, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h434
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h435, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h435
        }
    });
    let v4: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, _, h436, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h436
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h437, _, _, _, _, _, _, _, _, _) => {
            h437
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h438, _, _, _, _, _) => {
            h438
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h439, _) => {
            h439
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h440, _, _, _, _, _, _, _, _, _, _, _) => {
            h440
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h441, _, _, _, _, _, _, _, _, _, _) => {
            h441
        }
    });
    let v5: Adt4 = f53(match v0.clone() {
        Adt2::C0(_, _, _, h442, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h442
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h443, _, _, _, _, _, _, _, _) => {
            h443
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h444, _, _, _, _) => {
            h444
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h445) => {
            h445
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h446, _, _, _, _, _, _, _, _, _) => {
            h446
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h447, _, _, _, _, _, _, _, _) => {
            h447
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h448, _, _, _) => {
            h448
        }
    }, match v3.clone() {
        Adt4::C0(h449, _, _, _) => {
            h449
        }
    }, match v4.clone() {
        Adt4::C0(h450, _, _, _) => {
            h450
        }
    }, match v5.clone() {
        Adt4::C0(h451, _, _, _) => {
            h451
        }
    }, match v2.clone() {
        Adt4::C0(_, h452, _, _) => {
            h452
        }
    }, match v3.clone() {
        Adt4::C0(_, h453, _, _) => {
            h453
        }
    }, match v4.clone() {
        Adt4::C0(_, h454, _, _) => {
            h454
        }
    }, match v5.clone() {
        Adt4::C0(_, h455, _, _) => {
            h455
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h456, _) => {
            h456
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h457, _) => {
            h457
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h458, _) => {
            h458
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h459, _) => {
            h459
        }
    }, match v2 {
        Adt4::C0(_, _, _, h460) => {
            h460
        }
    }, match v3 {
        Adt4::C0(_, _, _, h461) => {
            h461
        }
    }, match v4 {
        Adt4::C0(_, _, _, h462) => {
            h462
        }
    }, match v5 {
        Adt4::C0(_, _, _, h463) => {
            h463
        }
    })
}

pub fn f52(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h464, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h464
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h465, _, _, _, _, _, _, _, _, _) => {
            h465
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h466, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h466
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h467, _, _, _, _, _) => {
            h467
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h468, _, _, _, _, _, _, _, _) => {
            h468
        }
    }, match v0.clone() {
        Adt1::C0(h469, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h469
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h470, _, _, _, _, _, _, _, _, _, _, _) => {
            h470
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h471, _, _) => {
            h471
        }
    }, match v0.clone() {
        Adt1::C0(_, h472, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h472
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h473, _, _, _, _) => {
            h473
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h474, _, _, _) => {
            h474
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h475, _, _, _, _, _, _, _, _, _, _) => {
            h475
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h476, _, _, _, _, _, _) => {
            h476
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h477, _) => {
            h477
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h478) => {
            h478
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h479, _, _, _, _, _, _, _) => {
            h479
        }
    })
}

pub fn f53(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f54(f54(v0, v1), v4);
    let v7: u32 = f55({
        let a480 = v3;
        let a481 = v6;
        fixed_u32::bit_xor(a480, a481)
    }, 16u32, 16u32);
    let v8: u32 = f54(v2, v7);
    let v9: u32 = f55({
        let a482 = v1;
        let a483 = v8;
        fixed_u32::bit_xor(a482, a483)
    }, 12u32, 20u32);
    let v10: u32 = f54(f54(v6, v9), v5);
    let v11: u32 = f55({
        let a484 = v7;
        let a485 = v10;
        fixed_u32::bit_xor(a484, a485)
    }, 8u32, 24u32);
    let v12: u32 = f54(v8, v11);
    let v13: u32 = f55({
        let a486 = v9;
        let a487 = v12;
        fixed_u32::bit_xor(a486, a487)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f54(v0: u32, v1: u32) -> u32 {
    let m490 = {
        let a488 = f56(v0);
        let a489 = f56(v1);
        fixed_u64::checked_add(a488, a489)
    };
    match m490 {
        None => {
            0u32
        }
        Some(v2) => {
            f57(v2)
        }
    }
}

pub fn f55(v0: u32, v1: u32, v2: u32) -> u32 {
    let a491 = f32(v0, v1);
    let a492 = f47(v0, v2);
    fixed_u32::bit_or(a491, a492)
}

pub fn f56(v0: u32) -> u64 {
    let m494 = {
        let a493 = v0;
        fixed_u64::convert(i128::from(a493))
    };
    match m494 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f57(v0: u64) -> u32 {
    let m498 = {
        let a497 = {
            let a495 = v0;
            let a496 = 4294967295u64;
            fixed_u64::bit_and(a495, a496)
        };
        fixed_u32::convert(i128::from(a497))
    };
    match m498 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn run(p0: Bytes, p1: Bytes, p2: List<(Bytes, Bytes)>) -> R<Bytes> {
    f0(p0, p1, p2)
}
