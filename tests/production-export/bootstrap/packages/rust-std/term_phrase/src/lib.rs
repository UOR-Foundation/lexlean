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
    C0(u64),
    C1(u64, u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(std::rc::Rc<Adt2>),
    C1(List<u64>, u64, u64, std::rc::Rc<Adt5>, List<Adt6>),
    C2,
    C3(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Option<Adt8>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, List<Adt0>, List<Adt2>, List<List<Adt3>>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt11>, List<Adt4>, Option<Adt7>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64),
    C1(u64, u64, std::rc::Rc<Adt6>, List<Adt4>, List<Adt12>),
    C2(u64, u64, std::rc::Rc<Adt6>, List<Adt4>, List<Adt12>, std::rc::Rc<Adt12>, std::rc::Rc<Adt6>),
    C3(u64, u64, std::rc::Rc<Adt6>, List<Adt4>, List<Adt12>, std::rc::Rc<Adt12>, std::rc::Rc<Adt6>, List<Adt4>),
    C4(u64, u64, std::rc::Rc<Adt6>, List<Adt4>, List<Adt12>, std::rc::Rc<Adt12>, std::rc::Rc<Adt6>, List<Adt4>, std::rc::Rc<Adt4>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(std::rc::Rc<Adt13>, List<Adt12>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(std::rc::Rc<Adt13>, List<Adt3>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(std::rc::Rc<Adt9>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn0::F3(k0) => f3((**k0).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F20(std::rc::Rc<Adt6>, std::rc::Rc<Adt9>, std::rc::Rc<Adt12>, std::rc::Rc<Adt4>, std::rc::Rc<Adt6>, u64, std::rc::Rc<Adt6>),
}

impl Fn1 {
    pub fn apply(&self, p0: (Adt13, (List<Adt4>, Option<Adt7>)), p1: Adt4) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
        match self {
            Fn1::F20(k0, k1, k2, k3, k4, k5, k6) => f20((**k0).clone(), (**k1).clone(), (**k2).clone(), (**k3).clone(), (**k4).clone(), *k5, (**k6).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(std::rc::Rc<Adt9>, u64, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt3) -> R<List<Adt12>> {
        match self {
            Fn2::F33(k0, k1, k2, k3) => f33((**k0).clone(), *k1, *k2, *k3, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F36,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt5, p1: Adt5) -> Adt5 {
        match self {
            Fn3::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F38,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn4::F38 => f38(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F49(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn5::F49(k0) => f49(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F51(u64, std::rc::Rc<Adt9>),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F51(k0, k1) => f51(*k0, (**k1).clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F54(u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn7::F54(k0, k1) => f54(*k0, k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Result<List<Adt4>, Adt7>> {
    let v11: Adt9 = f1(v0, v1, v2, v3, v4, v5, v6);
    let v12: Adt10 = {
        let (h1, _) = f2(Fn0::F3(std::rc::Rc::new(v11)), {
            let a6 = {
                let a4 = {
                    let a2 = v5;
                    let a3 = 2u64;
                    nat_add(a2, a3)?
                };
                let a5 = 64u64;
                nat_mul(a4, a5)?
            };
            let a7 = 64u64;
            nat_add(a6, a7)?
        }, Adt10::C0(List::cons(Adt11::C0(v9, v10), List::<Adt11>::nil()), List::<Adt4>::nil(), None::<Adt7>, std::rc::Rc::new(f4(v7, v8))))?;
        h1
    };
    let m9 = match v12.clone() {
        Adt10::C0(_, _, h8, _) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt4>, Adt7>(match v12 {
                Adt10::C0(_, h10, _, _) => {
                    h10
                }
            }))
        }
        Some(v13) => {
            Ok(Err::<List<Adt4>, Adt7>(v13))
        }
    }
}

pub fn f1(v0: Bytes, v1: List<Adt0>, v2: List<Adt2>, v3: List<List<Adt3>>, v4: u64, v5: u64, v6: u64) -> Adt9 {
    Adt9::C0(v0, v1, v2, v3, v5, v6, v4)
}

pub fn f2(v0: Fn0, p15: u64, p16: Adt10) -> R<(Adt10, bool)> {
    let m14 = (p15, p16);
    {
        let mut m14 = m14;
        loop {
            let step = {
                let (v1, v2) = m14;
                let m11 = v1;
                if m11 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m11 - 1;
                    let m13 = {
                        let c12: Fn0 = v0.clone();
                        c12.apply(v2.clone())?
                    };
                    match m13 {
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
                Ok(next) => { m14 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt9, v1: Adt10) -> R<Option<Adt10>> {
    f5(v0, v1)
}

pub fn f4(v0: u64, v1: u64) -> Adt13 {
    Adt13::C0(v0, v1, List::<u64>::nil())
}

pub fn f5(v0: Adt9, v1: Adt10) -> R<Option<Adt10>> {
    let m18 = match v1.clone() {
        Adt10::C0(_, _, h17, _) => {
            h17
        }
    };
    match m18 {
        None => {
            let m20 = match v1.clone() {
                Adt10::C0(h19, _, _, _) => {
                    h19
                }
            };
            match m20.uncons() {
                None => {
                    Ok(None::<Adt10>)
                }
                Some((v3, v4)) => {
                    Ok(Some({
                        let m21 = v3;
                        match m21 {
                            Adt11::C0(v5, v6) => {
                                f6(v0, v1, v4, v5, v6)?
                            }
                            Adt11::C1(v7, v8, r23, v10, v11) => {
                                let v9 = (*r23).clone();
                                f7(v0, v1, v4, v7, v8, v9, v10, v11)?
                            }
                            Adt11::C2(v12, v13, r24, v15, v16, r25, r26) => {
                                let v14 = (*r24).clone();
                                let v17 = (*r25).clone();
                                let v18 = (*r26).clone();
                                f8(v1.clone(), List::cons(Adt11::C3(v12, v13, std::rc::Rc::new(v14), v15, v16, std::rc::Rc::new(v17), std::rc::Rc::new(v18), match v1 {
                                    Adt10::C0(_, h22, _, _) => {
                                        h22
                                    }
                                }), v4), List::<Adt4>::nil())
                            }
                            Adt11::C3(v19, v20, r27, v22, v23, r28, r29, v26) => {
                                let v21 = (*r27).clone();
                                let v24 = (*r28).clone();
                                let v25 = (*r29).clone();
                                f9(v0, v1, v4, v19, v20, v21, v22, v23, v24, v25, v26)?
                            }
                            Adt11::C4(v27, v28, r30, v30, v31, r31, r32, v34, r33, r34) => {
                                let v29 = (*r30).clone();
                                let v32 = (*r31).clone();
                                let v33 = (*r32).clone();
                                let v35 = (*r33).clone();
                                let v36 = (*r34).clone();
                                f10(v0, v1, v4, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)?
                            }
                        }
                    }))
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt10>)
        }
    }
}

pub fn f6(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64) -> R<Adt10> {
    let v5: (Adt13, Option<Adt7>) = f11(v0.clone(), match v1.clone() {
        Adt10::C0(_, _, _, h35) => {
            (*h35).clone()
        }
    })?;
    let m37 = {
        let (_, h36) = v5;
        h36
    };
    match m37 {
        None => {
            let m39 = f13(v0.clone(), v4, v3);
            match m39 {
                None => {
                    let v9: List<Adt4> = {
                        let m41 = f14(v0.clone(), v3);
                        match m41 {
                            None => {
                                List::<Adt4>::nil()
                            }
                            Some(v8) => {
                                List::cons(Adt4::C0({
                                    let a42 = v3;
                                    let a43 = 1u64;
                                    nat_add(a42, a43)?
                                }, std::rc::Rc::new(Adt5::C0(std::rc::Rc::new(v8)))), List::<Adt4>::nil())
                            }
                        }
                    };
                    let m44 = f15(v0.clone(), v3, f16(Str::lit("the")));
                    match m44 {
                        None => {
                            Ok(Adt10::C0(v2, v9, None::<Adt7>, std::rc::Rc::new({
                                let (h45, _) = v5;
                                h45
                            })))
                        }
                        Some(v10) => {
                            let v11: Adt14 = f17(v0, {
                                let (h46, _) = v5;
                                h46
                            }, {
                                let a47 = v3;
                                let a48 = 1u64;
                                nat_add(a47, a48)?
                            }, 1u64, 2u64)?;
                            let m50 = match v11.clone() {
                                Adt14::C0(_, _, h49) => {
                                    h49
                                }
                            };
                            match m50 {
                                None => {
                                    Ok(Adt10::C0(List::cons(Adt11::C1(v3, v4, std::rc::Rc::new(v10), v9, match v11.clone() {
                                        Adt14::C0(_, h52, _) => {
                                            h52
                                        }
                                    }), v2), List::<Adt4>::nil(), None::<Adt7>, std::rc::Rc::new(match v11 {
                                        Adt14::C0(h53, _, _) => {
                                            (*h53).clone()
                                        }
                                    })))
                                }
                                Some(v12) => {
                                    Ok(f12(v1, match v11 {
                                        Adt14::C0(h51, _, _) => {
                                            (*h51).clone()
                                        }
                                    }, v12))
                                }
                            }
                        }
                    }
                }
                Some(v7) => {
                    Ok(f12(v1, {
                        let (h40, _) = v5;
                        h40
                    }, v7))
                }
            }
        }
        Some(v6) => {
            Ok(f12(v1, {
                let (h38, _) = v5;
                h38
            }, v6))
        }
    }
}

pub fn f7(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>) -> R<Adt10> {
    let m54 = v7;
    match m54.uncons() {
        None => {
            Ok(f8(v1, v2, v6))
        }
        Some((v8, v9)) => {
            let m56 = f15(v0, match v8.clone() {
                Adt12::C0(_, _, _, h55) => {
                    h55
                }
            }, f16(Str::lit("of")));
            match m56 {
                None => {
                    Ok(f8(v1, List::cons(Adt11::C1(v3, v4, std::rc::Rc::new(v5), v6, v9), v2), List::<Adt4>::nil()))
                }
                Some(v10) => {
                    Ok(f8(v1, List::cons(Adt11::C0({
                        let a58 = match v8.clone() {
                            Adt12::C0(_, _, _, h57) => {
                                h57
                            }
                        };
                        let a59 = 1u64;
                        nat_add(a58, a59)?
                    }, {
                        let a60 = v4;
                        let a61 = 1u64;
                        nat_add(a60, a61)?
                    }), List::cons(Adt11::C2(v3, v4, std::rc::Rc::new(v5), v6, v9, std::rc::Rc::new(v8), std::rc::Rc::new(v10)), v2)), List::<Adt4>::nil()))
                }
            }
        }
    }
}

pub fn f8(v0: Adt10, v1: List<Adt11>, v2: List<Adt4>) -> Adt10 {
    Adt10::C0(v1, v2, match v0.clone() {
        Adt10::C0(_, _, h62, _) => {
            h62
        }
    }, std::rc::Rc::new(match v0 {
        Adt10::C0(_, _, _, h63) => {
            (*h63).clone()
        }
    }))
}

pub fn f9(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>, v8: Adt12, v9: Adt6, v10: List<Adt4>) -> R<Adt10> {
    let m64 = v10;
    match m64.uncons() {
        None => {
            Ok(f8(v1, List::cons(Adt11::C1(v3, v4, std::rc::Rc::new(v5), v6, v7), v2), List::<Adt4>::nil()))
        }
        Some((v11, v12)) => {
            let a66 = match v8.clone() {
                Adt12::C0(_, h65, _, _) => {
                    h65
                }
            };
            let a67 = 1u64;
            if nat_eq(a66, a67) {
                Ok(f8(v1, List::cons(Adt11::C3(v3, v4, std::rc::Rc::new(v5.clone()), {
                    let a70 = v6;
                    let a71 = List::cons(Adt4::C0(match v11.clone() {
                        Adt4::C0(h68, _) => {
                            h68
                        }
                    }, std::rc::Rc::new(f18(v0, v3, v8.clone(), List::cons(match v11 {
                        Adt4::C0(_, h69) => {
                            (*h69).clone()
                        }
                    }, List::<Adt5>::nil()), List::cons(v5, List::cons(v9.clone(), List::<Adt6>::nil())))?)), List::<Adt4>::nil());
                    append_list(a70, a71)
                }, v7, std::rc::Rc::new(v8), std::rc::Rc::new(v9), v12), v2), List::<Adt4>::nil()))
            } else {
                let m73 = f15(v0, match v11.clone() {
                    Adt4::C0(h72, _) => {
                        h72
                    }
                }, f16(Str::lit("and")));
                match m73 {
                    None => {
                        Ok(f8(v1, List::cons(Adt11::C3(v3, v4, std::rc::Rc::new(v5), v6, v7, std::rc::Rc::new(v8), std::rc::Rc::new(v9), v12), v2), List::<Adt4>::nil()))
                    }
                    Some(v13) => {
                        Ok(f8(v1, List::cons(Adt11::C0({
                            let a75 = match v11.clone() {
                                Adt4::C0(h74, _) => {
                                    h74
                                }
                            };
                            let a76 = 1u64;
                            nat_add(a75, a76)?
                        }, {
                            let a77 = v4;
                            let a78 = 1u64;
                            nat_add(a77, a78)?
                        }), List::cons(Adt11::C4(v3, v4, std::rc::Rc::new(v5), v6, v7, std::rc::Rc::new(v8), std::rc::Rc::new(v9), v12, std::rc::Rc::new(v11), std::rc::Rc::new(v13)), v2)), List::<Adt4>::nil()))
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Adt9, v1: Adt10, v2: List<Adt11>, v3: u64, v4: u64, v5: Adt6, v6: List<Adt4>, v7: List<Adt12>, v8: Adt12, v9: Adt6, v10: List<Adt4>, v11: Adt4, v12: Adt6) -> R<Adt10> {
    let v13: (Adt13, (List<Adt4>, Option<Adt7>)) = f19(Fn1::F20(std::rc::Rc::new(v12), std::rc::Rc::new(v0), std::rc::Rc::new(v8.clone()), std::rc::Rc::new(v11), std::rc::Rc::new(v9.clone()), v3, std::rc::Rc::new(v5.clone())), (match v1.clone() {
        Adt10::C0(_, _, _, h79) => {
            (*h79).clone()
        }
    }, (v6, None::<Adt7>)), match v1.clone() {
        Adt10::C0(_, h80, _, _) => {
            h80
        }
    })?;
    let m83 = {
        let (_, h81) = {
            let (_, h82) = v13.clone();
            h82
        };
        h81
    };
    match m83 {
        None => {
            Ok(Adt10::C0(List::cons(Adt11::C3(v3, v4, std::rc::Rc::new(v5), {
                let (h85, _) = {
                    let (_, h86) = v13;
                    h86
                };
                h85
            }, v7, std::rc::Rc::new(v8), std::rc::Rc::new(v9), v10), v2), List::<Adt4>::nil(), None::<Adt7>, std::rc::Rc::new({
                let (h87, _) = v13;
                h87
            })))
        }
        Some(v14) => {
            Ok(f12(v1, {
                let (h84, _) = v13;
                h84
            }, v14))
        }
    }
}

pub fn f11(v0: Adt9, v1: Adt13) -> R<(Adt13, Option<Adt7>)> {
    let v2: u64 = {
        let a89 = match v1.clone() {
            Adt13::C0(h88, _, _) => {
                h88
            }
        };
        let a90 = 1u64;
        nat_add(a89, a90)?
    };
    let v3: Adt13 = Adt13::C0(v2, match v1.clone() {
        Adt13::C0(_, h91, _) => {
            h91
        }
    }, match v1 {
        Adt13::C0(_, _, h92) => {
            h92
        }
    });
    let a94 = match v0.clone() {
        Adt9::C0(_, _, _, _, h93, _, _) => {
            h93
        }
    };
    let a95 = v2;
    if nat_lt(a94, a95) {
        Ok((v3, Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f22(f16(Str::lit("max_parse_states exceeded in phase parse: configured ")), f23(match v0 {
            Adt9::C0(_, _, _, _, h96, _, _) => {
                h96
            }
        })), f16(Str::lit(", observed "))), f23(v2)), f16(Str::lit(" states"))), None::<Adt8>))))
    } else {
        Ok((v3, None::<Adt7>))
    }
}

pub fn f12(v0: Adt10, v1: Adt13, v2: Adt7) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt10::C0(h97, _, _, _) => {
            h97
        }
    }, match v0 {
        Adt10::C0(_, h98, _, _) => {
            h98
        }
    }, Some(v2), std::rc::Rc::new(v1))
}

pub fn f13(v0: Adt9, v1: u64, v2: u64) -> Option<Adt7> {
    let a100 = match v0.clone() {
        Adt9::C0(_, _, _, _, _, h99, _) => {
            h99
        }
    };
    let a101 = v1;
    if nat_lt(a100, a101) {
        Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f16(Str::lit("max_scope_depth exceeded in phase parse (proposition nesting): configured ")), f23(match v0.clone() {
            Adt9::C0(_, _, _, _, _, h102, _) => {
                h102
            }
        })), f16(Str::lit(", nesting depth "))), f23(v1)), {
            let m103 = f24(v0.clone(), v2);
            match m103 {
                None => {
                    None::<Adt8>
                }
                Some(v3) => {
                    Some(f25(v0, f26(v3)))
                }
            }
        }))
    } else {
        None::<Adt7>
    }
}

pub fn f14(v0: Adt9, v1: u64) -> Option<Adt2> {
    let m104 = f24(v0, v1);
    match m104 {
        None => {
            None::<Adt2>
        }
        Some(v2) => {
            let m105 = v2.clone();
            match m105 {
                Adt2::C0(_) => {
                    None::<Adt2>
                }
                Adt2::C1(_, _, _, _, _) => {
                    Some(v2)
                }
            }
        }
    }
}

pub fn f15(v0: Adt9, v1: u64, v2: Bytes) -> Option<Adt6> {
    let m106 = f27(v0.clone(), v1);
    match m106 {
        None => {
            None::<Adt6>
        }
        Some(v3) => {
            if f28({
                let m108 = f29(v0, v3);
                match m108 {
                    Adt0::C0(_, _, _, _, _, _, _, h107) => {
                        h107
                    }
                }
            }, v2.clone()) {
                let m109 = f30(v2);
                match m109 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v4) => {
                        Some(Adt6::C0(v3, v4))
                    }
                }
            } else {
                None::<Adt6>
            }
        }
    }
}

pub fn f16(v0: Str) -> Bytes {
    let a110 = v0;
    utf8_encode(a110)
}

pub fn f17(v0: Adt9, v1: Adt13, v2: u64, v3: u64, v4: u64) -> R<Adt14> {
    let m111 = f24(v0.clone(), v2);
    match m111 {
        None => {
            Ok(Adt14::C0(std::rc::Rc::new(v1), List::<Adt12>::nil(), None::<Adt7>))
        }
        Some(v5) => {
            let m112 = v5;
            match m112 {
                Adt2::C0(v6) => {
                    let v7: Adt15 = f31(v0.clone(), v1, v6)?;
                    let m114 = match v7.clone() {
                        Adt15::C0(_, _, h113) => {
                            h113
                        }
                    };
                    match m114 {
                        None => {
                            Ok(Adt14::C0(std::rc::Rc::new(match v7.clone() {
                                Adt15::C0(h116, _, _) => {
                                    (*h116).clone()
                                }
                            }), f32(Fn2::F33(std::rc::Rc::new(v0), v4, v3, v2), List::<Adt12>::nil(), match v7 {
                                Adt15::C0(_, h117, _) => {
                                    h117
                                }
                            })?, None::<Adt7>))
                        }
                        Some(v8) => {
                            Ok(Adt14::C0(std::rc::Rc::new(match v7 {
                                Adt15::C0(h115, _, _) => {
                                    (*h115).clone()
                                }
                            }), List::<Adt12>::nil(), Some(v8)))
                        }
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    Ok(Adt14::C0(std::rc::Rc::new(v1), List::<Adt12>::nil(), None::<Adt7>))
                }
            }
        }
    }
}

pub fn f18(v0: Adt9, v1: u64, v2: Adt12, v3: List<Adt5>, v4: List<Adt6>) -> R<Adt5> {
    let v5: (u64, u64) = f34(v0, {
        let a118 = v1;
        let a119 = 1u64;
        nat_add(a118, a119)?
    }, match v2.clone() {
        Adt12::C0(_, _, _, h120) => {
            h120
        }
    })?;
    Ok(Adt5::C1(List::cons(match v2 {
        Adt12::C0(h121, _, _, _) => {
            h121
        }
    }, List::<u64>::nil()), {
        let (h122, _) = v5;
        h122
    }, {
        let (_, h123) = v5;
        h123
    }, std::rc::Rc::new(f35(Fn3::F36, Adt5::C2, f37(Fn4::F38, List::<Adt5>::nil(), v3))), v4))
}

pub fn f19(v0: Fn1, p127: (Adt13, (List<Adt4>, Option<Adt7>)), p128: List<Adt4>) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
    let m126 = (p127, p128);
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v1, v2) = m126;
                let m124 = v2;
                match m124.uncons() {
                    None => {
                        Err::<((Adt13, (List<Adt4>, Option<Adt7>)), List<Adt4>), R<(Adt13, (List<Adt4>, Option<Adt7>))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((Adt13, (List<Adt4>, Option<Adt7>)), List<Adt4>), R<(Adt13, (List<Adt4>, Option<Adt7>))>>(({
                            let c125: Fn1 = v0.clone();
                            c125.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt6, v1: Adt9, v2: Adt12, v3: Adt4, v4: Adt6, v5: u64, v6: Adt6, v7: (Adt13, (List<Adt4>, Option<Adt7>)), v8: Adt4) -> R<(Adt13, (List<Adt4>, Option<Adt7>))> {
    let m131 = {
        let (_, h129) = {
            let (_, h130) = v7.clone();
            h130
        };
        h129
    };
    match m131 {
        None => {
            let v10: (Adt13, Option<Adt7>) = f11(v1.clone(), {
                let (h132, _) = v7;
                h132
            })?;
            let m134 = {
                let (_, h133) = v10;
                h133
            };
            match m134 {
                None => {
                    Ok(({
                        let (h138, _) = v10;
                        h138
                    }, ({
                        let a144 = {
                            let (h139, _) = {
                                let (_, h140) = v7;
                                h140
                            };
                            h139
                        };
                        let a145 = List::cons(Adt4::C0(match v8.clone() {
                            Adt4::C0(h141, _) => {
                                h141
                            }
                        }, std::rc::Rc::new(f18(v1, v5, v2, List::cons(match v3 {
                            Adt4::C0(_, h142) => {
                                (*h142).clone()
                            }
                        }, List::cons(match v8 {
                            Adt4::C0(_, h143) => {
                                (*h143).clone()
                            }
                        }, List::<Adt5>::nil())), List::cons(v6, List::cons(v4, List::cons(v0, List::<Adt6>::nil()))))?)), List::<Adt4>::nil());
                        append_list(a144, a145)
                    }, None::<Adt7>)))
                }
                Some(v11) => {
                    Ok(({
                        let (h135, _) = v10;
                        h135
                    }, ({
                        let (h136, _) = {
                            let (_, h137) = v7;
                            h137
                        };
                        h136
                    }, Some(v11))))
                }
            }
        }
        Some(_) => {
            Ok(v7)
        }
    }
}

pub fn f21(v0: Bytes, v1: Bytes, v2: Option<Adt8>) -> Adt7 {
    Adt7::C0(v0, v1, v2, None::<Bytes>)
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    let a146 = v0;
    let a147 = v1;
    append_bytes(a146, a147)
}

pub fn f23(v0: u64) -> Bytes {
    f39(v0)
}

pub fn f24(v0: Adt9, v1: u64) -> Option<Adt2> {
    let a149 = match v0 {
        Adt9::C0(_, _, h148, _, _, _, _) => {
            h148
        }
    };
    let a150 = v1;
    index_list(a149, a150)
}

pub fn f25(v0: Adt9, v1: u64) -> Adt8 {
    f40(match v0.clone() {
        Adt9::C0(h151, _, _, _, _, _, _) => {
            h151
        }
    }, f29(v0, v1))
}

pub fn f26(v0: Adt2) -> u64 {
    let m152 = v0;
    match m152 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f27(v0: Adt9, v1: u64) -> Option<u64> {
    let m153 = f24(v0.clone(), v1);
    match m153 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let m154 = v2;
            match m154 {
                Adt2::C0(v3) => {
                    let a155 = f41(f29(v0, v3));
                    let a156 = 1u64;
                    if nat_eq(a155, a156) {
                        Some(v3)
                    } else {
                        None::<u64>
                    }
                }
                Adt2::C1(_, _, _, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: Bytes) -> bool {
    let a157 = v0;
    let a158 = v1;
    equal(a157, a158)
}

pub fn f29(v0: Adt9, v1: u64) -> Adt0 {
    let m162 = {
        let a160 = match v0 {
            Adt9::C0(_, h159, _, _, _, _, _) => {
                h159
            }
        };
        let a161 = v1;
        index_list(a160, a161)
    };
    match m162 {
        None => {
            Adt0::C0(Adt1::C6, 0u64, 0u64, 1u64, 1u64, 1u64, 1u64, f16(Str::lit("")))
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: Bytes) -> Option<Bytes> {
    if if f28(v0.clone(), f16(Str::lit("a"))) {
        true
    } else {
        f28(v0.clone(), f16(Str::lit("A")))
    } {
        Some(f16(Str::lit("a")))
    } else {
        if if f28(v0.clone(), f16(Str::lit("an"))) {
            true
        } else {
            f28(v0.clone(), f16(Str::lit("An")))
        } {
            Some(f16(Str::lit("an")))
        } else {
            if f28(v0.clone(), f16(Str::lit("and"))) {
                Some(f16(Str::lit("and")))
            } else {
                if f28(v0.clone(), f16(Str::lit("as"))) {
                    Some(f16(Str::lit("as")))
                } else {
                    if f28(v0.clone(), f16(Str::lit("defined"))) {
                        Some(f16(Str::lit("defined")))
                    } else {
                        if f28(v0.clone(), f16(Str::lit("every"))) {
                            Some(f16(Str::lit("every")))
                        } else {
                            if f28(v0.clone(), f16(Str::lit("exactly"))) {
                                Some(f16(Str::lit("exactly")))
                            } else {
                                if f28(v0.clone(), f16(Str::lit("exists"))) {
                                    Some(f16(Str::lit("exists")))
                                } else {
                                    if if f28(v0.clone(), f16(Str::lit("for"))) {
                                        true
                                    } else {
                                        f28(v0.clone(), f16(Str::lit("For")))
                                    } {
                                        Some(f16(Str::lit("for")))
                                    } else {
                                        if f28(v0.clone(), f16(Str::lit("holds"))) {
                                            Some(f16(Str::lit("holds")))
                                        } else {
                                            if if f28(v0.clone(), f16(Str::lit("if"))) {
                                                true
                                            } else {
                                                f28(v0.clone(), f16(Str::lit("If")))
                                            } {
                                                Some(f16(Str::lit("if")))
                                            } else {
                                                if f28(v0.clone(), f16(Str::lit("implies"))) {
                                                    Some(f16(Str::lit("implies")))
                                                } else {
                                                    if f28(v0.clone(), f16(Str::lit("is"))) {
                                                        Some(f16(Str::lit("is")))
                                                    } else {
                                                        if if f28(v0.clone(), f16(Str::lit("not"))) {
                                                            true
                                                        } else {
                                                            f28(v0.clone(), f16(Str::lit("Not")))
                                                        } {
                                                            Some(f16(Str::lit("not")))
                                                        } else {
                                                            if f28(v0.clone(), f16(Str::lit("one"))) {
                                                                Some(f16(Str::lit("one")))
                                                            } else {
                                                                if f28(v0.clone(), f16(Str::lit("only"))) {
                                                                    Some(f16(Str::lit("only")))
                                                                } else {
                                                                    if f28(v0.clone(), f16(Str::lit("or"))) {
                                                                        Some(f16(Str::lit("or")))
                                                                    } else {
                                                                        if f28(v0.clone(), f16(Str::lit("such"))) {
                                                                            Some(f16(Str::lit("such")))
                                                                        } else {
                                                                            if f28(v0.clone(), f16(Str::lit("that"))) {
                                                                                Some(f16(Str::lit("that")))
                                                                            } else {
                                                                                if if f28(v0.clone(), f16(Str::lit("the"))) {
                                                                                    true
                                                                                } else {
                                                                                    f28(v0.clone(), f16(Str::lit("The")))
                                                                                } {
                                                                                    Some(f16(Str::lit("the")))
                                                                                } else {
                                                                                    if f28(v0.clone(), f16(Str::lit("then"))) {
                                                                                        Some(f16(Str::lit("then")))
                                                                                    } else {
                                                                                        if if f28(v0.clone(), f16(Str::lit("there"))) {
                                                                                            true
                                                                                        } else {
                                                                                            f28(v0.clone(), f16(Str::lit("There")))
                                                                                        } {
                                                                                            Some(f16(Str::lit("there")))
                                                                                        } else {
                                                                                            if f28(v0.clone(), f16(Str::lit("when"))) {
                                                                                                Some(f16(Str::lit("when")))
                                                                                            } else {
                                                                                                if f28(v0, f16(Str::lit("of"))) {
                                                                                                    Some(f16(Str::lit("of")))
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
                }
            }
        }
    }
}

pub fn f31(v0: Adt9, v1: Adt13, v2: u64) -> R<Adt15> {
    let v4: List<Adt3> = {
        let m166 = {
            let a164 = match v0.clone() {
                Adt9::C0(_, _, _, h163, _, _, _) => {
                    h163
                }
            };
            let a165 = v2;
            index_list(a164, a165)
        };
        match m166 {
            None => {
                List::<Adt3>::nil()
            }
            Some(v3) => {
                v3
            }
        }
    };
    if f42(v2, match v1.clone() {
        Adt13::C0(_, _, h167) => {
            h167
        }
    }) {
        Ok(Adt15::C0(std::rc::Rc::new(v1), v4, None::<Adt7>))
    } else {
        let v5: u64 = {
            let a170 = match v1.clone() {
                Adt13::C0(_, h168, _) => {
                    h168
                }
            };
            let a171 = {
                let a169 = v4.clone();
                length_list(a169)
            };
            nat_add(a170, a171)?
        };
        let a173 = match v0.clone() {
            Adt9::C0(_, _, _, _, _, _, h172) => {
                h172
            }
        };
        let a174 = v5;
        if nat_lt(a173, a174) {
            Ok(Adt15::C0(std::rc::Rc::new(Adt13::C0(match v1.clone() {
                Adt13::C0(h175, _, _) => {
                    h175
                }
            }, v5, match v1 {
                Adt13::C0(_, _, h176) => {
                    h176
                }
            })), List::<Adt3>::nil(), Some(f21(f16(Str::lit("LLS8002")), f22(f22(f22(f22(f16(Str::lit("max_token_lattice_edges exceeded in phase lexical resolution: configured ")), f23(match v0 {
                Adt9::C0(_, _, _, _, _, _, h177) => {
                    h177
                }
            })), f16(Str::lit(", observed "))), f23(v5)), f16(Str::lit(" distinct lattice edges"))), None::<Adt8>))))
        } else {
            Ok(Adt15::C0(std::rc::Rc::new(Adt13::C0(match v1.clone() {
                Adt13::C0(h178, _, _) => {
                    h178
                }
            }, v5, List::cons(v2, match v1 {
                Adt13::C0(_, _, h179) => {
                    h179
                }
            }))), v4, None::<Adt7>))
        }
    }
}

pub fn f32(v0: Fn2, p183: List<Adt12>, p184: List<Adt3>) -> R<List<Adt12>> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v1, v2) = m182;
                let m180 = v2;
                match m180.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt3>), R<List<Adt12>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt3>), R<List<Adt12>>>(({
                            let c181: Fn2 = v0.clone();
                            c181.apply(v1, v3)?
                        }, v4))
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

pub fn f33(v0: Adt9, v1: u64, v2: u64, v3: u64, v4: List<Adt12>, v5: Adt3) -> R<List<Adt12>> {
    let a195 = v4;
    let a196 = if f43({
        let a186 = v2;
        let a187 = match v5.clone() {
            Adt3::C0(_, _, h185, _) => {
                h185
            }
        };
        nat_le(a186, a187)
    }, {
        let a189 = match v5.clone() {
            Adt3::C0(_, _, h188, _) => {
                h188
            }
        };
        let a190 = v1;
        nat_le(a189, a190)
    }) {
        List::cons(Adt12::C0(match v5.clone() {
            Adt3::C0(h191, _, _, _) => {
                h191
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h192, _) => {
                h192
            }
        }, match v5.clone() {
            Adt3::C0(_, _, _, h193) => {
                h193
            }
        }, f44(v0, v3, match v5 {
            Adt3::C0(_, h194, _, _) => {
                h194
            }
        })?), List::<Adt12>::nil())
    } else {
        List::<Adt12>::nil()
    };
    Ok(append_list(a195, a196))
}

pub fn f34(v0: Adt9, v1: u64, v2: u64) -> R<(u64, u64)> {
    let v4: Adt2 = {
        let m197 = f24(v0.clone(), v1);
        match m197 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v3) => {
                v3
            }
        }
    };
    let v5: u64 = {
        let a200 = v1;
        let a201 = {
            let a198 = v2;
            let a199 = 1u64;
            nat_sub(a198, a199)
        };
        if nat_lt(a200, a201) {
            let a202 = v2;
            let a203 = 1u64;
            nat_sub(a202, a203)
        } else {
            v1
        }
    };
    let v7: Adt2 = {
        let m204 = f24(v0, v5);
        match m204 {
            None => {
                Adt2::C0(0u64)
            }
            Some(v6) => {
                v6
            }
        }
    };
    Ok((f26(v4), {
        let a205 = f45(v7);
        let a206 = 1u64;
        nat_add(a205, a206)?
    }))
}

pub fn f35(v0: Fn3, p210: Adt5, p211: List<Adt5>) -> Adt5 {
    let m209 = (p210, p211);
    {
        let mut m209 = m209;
        loop {
            let step = {
                let (v1, v2) = m209;
                let m207 = v2;
                match m207.uncons() {
                    None => {
                        Err::<(Adt5, List<Adt5>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<Adt5>), Adt5>(({
                            let c208: Fn3 = v0.clone();
                            c208.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Adt5, v1: Adt5) -> Adt5 {
    Adt5::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f37(v0: Fn4, p215: List<Adt5>, p216: List<Adt5>) -> List<Adt5> {
    let m214 = (p215, p216);
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, v2) = m214;
                let m212 = v2;
                match m212.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c213: Fn4 = v0.clone();
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

pub fn f38(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f39(v0: u64) -> Bytes {
    f46(f47(), 10u64, v0)
}

pub fn f40(v0: Bytes, v1: Adt0) -> Adt8 {
    Adt8::C0(v0, match v1.clone() {
        Adt0::C0(_, h217, _, _, _, _, _, _) => {
            h217
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h218, _, _, _, _, _) => {
            h218
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h219, _, _, _, _) => {
            h219
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h220, _, _, _) => {
            h220
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h221, _, _) => {
            h221
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h222, _) => {
            h222
        }
    })
}

pub fn f41(v0: Adt0) -> u64 {
    let m224 = match v0 {
        Adt0::C0(h223, _, _, _, _, _, _, _) => {
            h223
        }
    };
    match m224 {
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

pub fn f42(v0: u64, v1: List<u64>) -> bool {
    f48(Fn5::F49(v0), false, v1)
}

pub fn f43(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f44(v0: Adt9, v1: u64, v2: u64) -> R<u64> {
    let (h225, _) = f50(Fn6::F51(v2, std::rc::Rc::new(v0.clone())), {
        let a228 = {
            let a227 = match v0 {
                Adt9::C0(_, _, h226, _, _, _, _) => {
                    h226
                }
            };
            length_list(a227)
        };
        let a229 = 1u64;
        nat_add(a228, a229)?
    }, v1)?;
    Ok(h225)
}

pub fn f45(v0: Adt2) -> u64 {
    let m230 = v0;
    match m230 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(_, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a231 = v2;
    let a232 = 0u64;
    if nat_eq(a231, a232) {
        f52(v0, 0u64)
    } else {
        let (_, h233) = {
            let (h234, _) = f53(Fn7::F54(v1, v0), v2, (v2, Bytes::lit(&[])));
            h234
        };
        h233
    }
}

pub fn f47() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f48(v0: Fn5, p238: bool, p239: List<u64>) -> bool {
    let m237 = (p238, p239);
    {
        let mut m237 = m237;
        loop {
            let step = {
                let (v1, v2) = m237;
                let m235 = v2;
                match m235.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c236: Fn5 = v0.clone();
                            c236.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a240 = v2;
        let a241 = v0;
        nat_eq(a240, a241)
    }
}

pub fn f50(v0: Fn6, p246: u64, p247: u64) -> R<(u64, bool)> {
    let m245 = (p246, p247);
    {
        let mut m245 = m245;
        loop {
            let step = {
                let (v1, v2) = m245;
                let m242 = v1;
                if m242 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m242 - 1;
                    let m244 = {
                        let c243: Fn6 = v0.clone();
                        c243.apply(v2)?
                    };
                    match m244 {
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
                Ok(next) => { m245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: u64, v1: Adt9, v2: u64) -> R<Option<u64>> {
    let m248 = f24(v1, v2);
    match m248 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a249 = f26(v3);
            let a250 = v0;
            if nat_lt(a249, a250) {
                Ok(Some({
                    let a251 = v2;
                    let a252 = 1u64;
                    nat_add(a251, a252)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f52(v0: Bytes, v1: u64) -> Bytes {
    let m256 = {
        let a253 = v0;
        let a254 = v1;
        let a255 = 1u64;
        slice_bytes(a253, a254, a255)
    };
    match m256 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f53(v0: Fn7, p261: u64, p262: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m260 = (p261, p262);
    {
        let mut m260 = m260;
        loop {
            let step = {
                let (v1, v2) = m260;
                let m257 = v1;
                if m257 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m257 - 1;
                    let m259 = {
                        let c258: Fn7 = v0.clone();
                        c258.apply(v2.clone())
                    };
                    match m259 {
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
                Ok(next) => { m260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f55(v1, v0, v2)
}

pub fn f55(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h263, _) = v2;
        h263
    };
    let a264 = v3;
    let a265 = 0u64;
    if nat_eq(a264, a265) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a266 = v3;
            let a267 = v1;
            let a268 = 0u64;
            nat_quot(a266, a267, a268)
        }, {
            let a273 = f52(v0, {
                let a269 = v3;
                let a270 = v1;
                let a271 = 0u64;
                nat_rem(a269, a270, a271)
            });
            let a274 = {
                let (_, h272) = v2;
                h272
            };
            append_bytes(a273, a274)
        }))
    }
}

pub fn run(p0: Bytes, p1: List<Adt0>, p2: List<Adt2>, p3: List<List<Adt3>>, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64) -> R<Result<List<Adt4>, Adt7>> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
}
